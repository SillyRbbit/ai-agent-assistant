#![cfg(unix)]

//! Isolated host-mechanics spike for the Hermes ACP stdio shape.
//!
//! The target is not linked into the application and never executes Hermes in
//! ordinary tests. Its cooperative fixture is not ACP conformance, candidate
//! provenance, tool containment, or process-tree containment evidence.

use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use tempfile::TempDir;

const PYTHON: &str = "/usr/bin/python3";
#[cfg(target_os = "macos")]
const XCODE_DEVELOPER_DIR: &str = "/Applications/Xcode.app/Contents/Developer";
const SESSION_ID: &str = "acp-session-fixture-1";
const EXPECTED_VERSION: &str = "0.20.0";
const MAX_FRAME_BYTES: usize = 4_096;
const MAX_FRAME_COUNT: usize = 24;
const MAX_STDERR_BYTES: usize = 512;
const TEST_DEADLINE: Duration = Duration::from_secs(2);
#[cfg(target_os = "macos")]
const STARTUP_DEADLINE: Duration = Duration::from_secs(10);
#[cfg(not(target_os = "macos"))]
const STARTUP_DEADLINE: Duration = TEST_DEADLINE;
const TIMEOUT_DEADLINE: Duration = Duration::from_millis(120);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpikeErrorCode {
    ExecutableRejected,
    VersionMismatch,
    CheckFailed,
    SpawnFailed,
    IoFailure,
    Timeout,
    FrameTooLarge,
    MalformedFrame,
    ProtocolViolation,
    ForbiddenUpdate,
    IdentityMismatch,
    UnexpectedExit,
    LateOutput,
    ShutdownFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SpikeError {
    code: SpikeErrorCode,
    timeout_site: Option<TimeoutSite>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TimeoutSite {
    Probe(&'static str),
    Read {
        scenario: &'static str,
        received_frames: usize,
    },
}

fn probe_label(arg: &OsStr) -> &'static str {
    match arg.to_str() {
        Some("--version") => "version",
        Some("--check") => "check",
        Some("--hang-probe") => "expected_hang",
        Some("--flood-probe") => "flood",
        _ => "unknown_probe",
    }
}

fn scenario_label(scenario: &str) -> &'static str {
    match scenario {
        "noisy_stderr" => "noisy_stderr",
        "hang_before_initialize" => "hang_before_initialize",
        "hang_after_update" => "hang_after_update",
        "malformed" => "malformed",
        "oversized" => "oversized",
        "early_exit" => "early_exit",
        "midstream_exit" => "midstream_exit",
        "unknown_method" => "unknown_method",
        "tool_update" => "tool_update",
        "permission_request" => "permission_request",
        "wrong_session" => "wrong_session",
        "wrong_id" => "wrong_id",
        "cancel" => "cancel",
        _ => "unknown_scenario",
    }
}

impl SpikeError {
    const fn new(code: SpikeErrorCode) -> Self {
        Self {
            code,
            timeout_site: None,
        }
    }

    const fn timeout(site: TimeoutSite) -> Self {
        Self {
            code: SpikeErrorCode::Timeout,
            timeout_site: Some(site),
        }
    }

    const fn code(self) -> SpikeErrorCode {
        self.code
    }
}

impl fmt::Display for SpikeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self.code {
            SpikeErrorCode::ExecutableRejected => "executable_rejected",
            SpikeErrorCode::VersionMismatch => "version_mismatch",
            SpikeErrorCode::CheckFailed => "check_failed",
            SpikeErrorCode::SpawnFailed => "spawn_failed",
            SpikeErrorCode::IoFailure => "io_failure",
            SpikeErrorCode::Timeout => "timeout",
            SpikeErrorCode::FrameTooLarge => "frame_too_large",
            SpikeErrorCode::MalformedFrame => "malformed_frame",
            SpikeErrorCode::ProtocolViolation => "protocol_violation",
            SpikeErrorCode::ForbiddenUpdate => "forbidden_update",
            SpikeErrorCode::IdentityMismatch => "identity_mismatch",
            SpikeErrorCode::UnexpectedExit => "unexpected_exit",
            SpikeErrorCode::LateOutput => "late_output",
            SpikeErrorCode::ShutdownFailed => "shutdown_failed",
        };
        write!(formatter, "hermes_acp_transport_spike:{code}")?;
        match self.timeout_site {
            Some(TimeoutSite::Probe(label)) => write!(formatter, ":probe:{label}"),
            Some(TimeoutSite::Read {
                scenario,
                received_frames,
            }) => write!(formatter, ":read:{scenario}:{received_frames}"),
            None => Ok(()),
        }
    }
}

impl Error for SpikeError {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct StderrSummary {
    captured_bytes: usize,
    truncated: bool,
}

#[derive(Debug)]
enum StdoutItem {
    Frame(Vec<u8>),
    FrameTooLarge,
    Unterminated,
    ReadFailure,
    Eof,
}

fn send_item(sender: &SyncSender<StdoutItem>, item: StdoutItem) -> bool {
    match sender.try_send(item) {
        Ok(()) => true,
        Err(TrySendError::Full(_)) | Err(TrySendError::Disconnected(_)) => false,
    }
}

fn spawn_stdout_reader<R>(mut reader: R) -> (Receiver<StdoutItem>, JoinHandle<()>)
where
    R: Read + Send + 'static,
{
    let (sender, receiver) = mpsc::sync_channel(16);
    let handle = thread::spawn(move || {
        let mut frame = Vec::with_capacity(256);
        let mut chunk = [0_u8; 256];
        loop {
            match reader.read(&mut chunk) {
                Ok(0) => {
                    if !frame.is_empty() && !send_item(&sender, StdoutItem::Unterminated) {
                        return;
                    }
                    let _sent = send_item(&sender, StdoutItem::Eof);
                    return;
                }
                Ok(read) => {
                    for byte in &chunk[..read] {
                        if *byte == b'\n' {
                            if !send_item(&sender, StdoutItem::Frame(std::mem::take(&mut frame))) {
                                return;
                            }
                        } else {
                            frame.push(*byte);
                            if frame.len() > MAX_FRAME_BYTES {
                                let _sent = send_item(&sender, StdoutItem::FrameTooLarge);
                                return;
                            }
                        }
                    }
                }
                Err(_) => {
                    let _sent = send_item(&sender, StdoutItem::ReadFailure);
                    return;
                }
            }
        }
    });
    (receiver, handle)
}

fn spawn_stderr_reader<R>(mut reader: R) -> (Receiver<StderrSummary>, JoinHandle<()>)
where
    R: Read + Send + 'static,
{
    let (sender, receiver) = mpsc::sync_channel(1);
    let handle = thread::spawn(move || {
        let mut total = 0_usize;
        let mut chunk = [0_u8; 256];
        loop {
            match reader.read(&mut chunk) {
                Ok(0) | Err(_) => break,
                Ok(read) => total = total.saturating_add(read),
            }
        }
        let _sent = sender.try_send(StderrSummary {
            captured_bytes: total.min(MAX_STDERR_BYTES),
            truncated: total > MAX_STDERR_BYTES,
        });
    });
    (receiver, handle)
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hermes_acp_server_stub.py")
}

fn validate_executable(path: &Path) -> Result<(), SpikeError> {
    if !path.is_absolute() {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| SpikeError::new(SpikeErrorCode::ExecutableRejected))?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_file()
        || metadata.permissions().mode() & 0o111 == 0
    {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    Ok(())
}

fn resolved_python() -> Result<PathBuf, SpikeError> {
    let resolved = fs::canonicalize(PYTHON)
        .map_err(|_| SpikeError::new(SpikeErrorCode::ExecutableRejected))?;
    validate_executable(&resolved)?;
    Ok(resolved)
}

fn configure_isolated(command: &mut Command, temp: &TempDir) {
    command
        .env_clear()
        .env("HOME", temp.path())
        .env("HERMES_HOME", temp.path())
        .env("TMPDIR", temp.path())
        .env("XDG_CONFIG_HOME", temp.path())
        .env("XDG_CACHE_HOME", temp.path())
        .env("XDG_DATA_HOME", temp.path())
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .env("PYTHONNOUSERSITE", "1")
        .env("PYTHONSAFEPATH", "1")
        .env("CORTEXA_ACP_FIXTURE", "1")
        .current_dir(temp.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(target_os = "macos")]
    command.env("DEVELOPER_DIR", XCODE_DEVELOPER_DIR);
}

fn probe_deadline(arg: &OsStr) -> Duration {
    if matches!(
        arg.to_str(),
        Some("--version" | "--check" | "--flood-probe")
    ) {
        STARTUP_DEADLINE
    } else {
        TEST_DEADLINE
    }
}

fn run_probe(arg: &OsStr, expected: &str) -> Result<(), SpikeError> {
    let python = resolved_python()?;
    let fixture = fixture_path();
    if !fixture.is_absolute() || !fixture.is_file() {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    let temp = TempDir::new().map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let mut command = Command::new(python);
    configure_isolated(&mut command, &temp);
    command.arg("-I").arg("-B").arg("-u").arg(fixture).arg(arg);
    command.stdin(Stdio::null());
    let mut child = command
        .spawn()
        .map_err(|_| SpikeError::new(SpikeErrorCode::SpawnFailed))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let (stdout_receiver, stdout_thread) = spawn_stdout_reader(stdout);
    let (stderr_receiver, stderr_thread) = spawn_stderr_reader(stderr);

    let deadline = Instant::now() + probe_deadline(arg);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(5)),
            Ok(None) | Err(_) => {
                let terminated = child.kill().is_ok() || matches!(child.try_wait(), Ok(Some(_)));
                if !terminated || child.wait().is_err() {
                    return Err(SpikeError::new(SpikeErrorCode::ShutdownFailed));
                }
                stdout_thread
                    .join()
                    .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
                stderr_thread
                    .join()
                    .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
                return Err(SpikeError::timeout(TimeoutSite::Probe(probe_label(arg))));
            }
        }
    };
    stdout_thread
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    stderr_thread
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;

    let mut frame = None;
    for item in stdout_receiver.try_iter() {
        match item {
            StdoutItem::Frame(bytes) if frame.is_none() => frame = Some(bytes),
            StdoutItem::Frame(_) => {
                return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation));
            }
            StdoutItem::FrameTooLarge => {
                return Err(SpikeError::new(SpikeErrorCode::FrameTooLarge));
            }
            StdoutItem::Unterminated | StdoutItem::ReadFailure => {
                return Err(SpikeError::new(SpikeErrorCode::MalformedFrame));
            }
            StdoutItem::Eof => {}
        }
    }
    let stderr = stderr_receiver.try_recv().unwrap_or_default();
    let output_matches = frame
        .as_deref()
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .map(str::trim)
        == Some(expected);
    if !status.success() || stderr.truncated || !output_matches {
        return Err(SpikeError::new(if arg == OsStr::new("--version") {
            SpikeErrorCode::VersionMismatch
        } else {
            SpikeErrorCode::CheckFailed
        }));
    }
    Ok(())
}

struct TestChild {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: Receiver<StdoutItem>,
    stderr: Receiver<StderrSummary>,
    stdout_thread: Option<JoinHandle<()>>,
    stderr_thread: Option<JoinHandle<()>>,
    stderr_summary: Option<StderrSummary>,
    temp: TempDir,
    frames: usize,
    scenario_label: &'static str,
    reaped: bool,
}

impl TestChild {
    fn spawn(scenario: &'static str) -> Result<Self, SpikeError> {
        let python = resolved_python()?;
        let fixture = fixture_path();
        if !fixture.is_absolute() || !fixture.is_file() {
            return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
        }
        let temp = TempDir::new().map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
        let mut command = Command::new(python);
        configure_isolated(&mut command, &temp);
        command
            .arg("-I")
            .arg("-B")
            .arg("-u")
            .arg(fixture)
            .arg(scenario);
        let mut child = command
            .spawn()
            .map_err(|_| SpikeError::new(SpikeErrorCode::SpawnFailed))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?;
        let child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?;
        let child_stderr = child
            .stderr
            .take()
            .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?;
        let (stdout, stdout_thread) = spawn_stdout_reader(child_stdout);
        let (stderr, stderr_thread) = spawn_stderr_reader(child_stderr);
        Ok(Self {
            child,
            stdin: Some(stdin),
            stdout,
            stderr,
            stdout_thread: Some(stdout_thread),
            stderr_thread: Some(stderr_thread),
            stderr_summary: None,
            temp,
            frames: 0,
            scenario_label: scenario_label(scenario),
            reaped: false,
        })
    }

    fn send(&mut self, value: &Value) -> Result<(), SpikeError> {
        let mut bytes =
            serde_json::to_vec(value).map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
        bytes.push(b'\n');
        self.stdin
            .as_mut()
            .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?
            .write_all(&bytes)
            .and_then(|()| {
                self.stdin
                    .as_mut()
                    .ok_or(std::io::Error::from(std::io::ErrorKind::BrokenPipe))?
                    .flush()
            })
            .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))
    }

    fn request(&mut self, id: u64, method: &str, params: Value) -> Result<(), SpikeError> {
        self.send(&json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
    }

    fn notify(&mut self, method: &str, params: Value) -> Result<(), SpikeError> {
        self.send(&json!({"jsonrpc":"2.0","method":method,"params":params}))
    }

    fn read_json(&mut self, timeout: Duration) -> Result<Value, SpikeError> {
        if self.frames >= MAX_FRAME_COUNT {
            return self.fail(SpikeErrorCode::ProtocolViolation);
        }
        match self.stdout.recv_timeout(timeout) {
            Ok(StdoutItem::Frame(bytes)) => {
                self.frames += 1;
                let text = match std::str::from_utf8(&bytes) {
                    Ok(text) => text,
                    Err(_) => return self.fail(SpikeErrorCode::MalformedFrame),
                };
                match serde_json::from_str(text) {
                    Ok(value) => Ok(value),
                    Err(_) => self.fail(SpikeErrorCode::MalformedFrame),
                }
            }
            Ok(StdoutItem::FrameTooLarge) => self.fail(SpikeErrorCode::FrameTooLarge),
            Ok(StdoutItem::Unterminated | StdoutItem::ReadFailure) => {
                self.fail(SpikeErrorCode::MalformedFrame)
            }
            Ok(StdoutItem::Eof) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                self.fail(SpikeErrorCode::UnexpectedExit)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let site = TimeoutSite::Read {
                    scenario: self.scenario_label,
                    received_frames: self.frames,
                };
                let _cleanup_failed = self.force_reap().is_err();
                Err(SpikeError::timeout(site))
            }
        }
    }

    fn fail<T>(&mut self, code: SpikeErrorCode) -> Result<T, SpikeError> {
        let _cleanup_failed = self.force_reap().is_err();
        Err(SpikeError::new(code))
    }

    fn force_reap(&mut self) -> Result<(), SpikeError> {
        if self.reaped {
            return Ok(());
        }
        self.stdin.take();
        match self.child.try_wait() {
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => {
                drop(self.child.kill());
                self.child
                    .wait()
                    .map_err(|_| SpikeError::new(SpikeErrorCode::ShutdownFailed))?;
            }
        }
        self.reaped = true;
        self.finish_readers()
    }

    fn finish_gracefully(&mut self) -> Result<(), SpikeError> {
        self.stdin.take();
        let deadline = Instant::now() + TEST_DEADLINE;
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) if status.success() => break,
                Ok(Some(_)) => return self.fail(SpikeErrorCode::UnexpectedExit),
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(5)),
                Ok(None) | Err(_) => return self.fail(SpikeErrorCode::ShutdownFailed),
            }
        }
        self.reaped = true;
        self.finish_readers()
    }

    fn finish_readers(&mut self) -> Result<(), SpikeError> {
        if let Some(handle) = self.stdout_thread.take() {
            handle
                .join()
                .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
        }
        if let Some(handle) = self.stderr_thread.take() {
            handle
                .join()
                .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
        }
        if self.stderr_summary.is_none() {
            self.stderr_summary = self.stderr.try_recv().ok();
        }
        Ok(())
    }

    fn stderr_summary(&self) -> StderrSummary {
        self.stderr_summary.unwrap_or_default()
    }
}

impl Drop for TestChild {
    fn drop(&mut self) {
        let _cleanup_failed = self.force_reap().is_err();
    }
}

fn expect_result(value: &Value, id: u64) -> Result<&Value, SpikeError> {
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
        || value.get("id").and_then(Value::as_u64) != Some(id)
    {
        return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation));
    }
    value
        .get("result")
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))
}

fn expect_text_update(value: &Value) -> Result<&str, SpikeError> {
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation));
    }
    match value.get("method").and_then(Value::as_str) {
        Some("session/request_permission") => {
            return Err(SpikeError::new(SpikeErrorCode::ForbiddenUpdate));
        }
        Some("session/update") => {}
        _ => return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation)),
    }
    if value.pointer("/params/sessionId").and_then(Value::as_str) != Some(SESSION_ID) {
        return Err(SpikeError::new(SpikeErrorCode::IdentityMismatch));
    }
    let kind = value
        .pointer("/params/update/sessionUpdate")
        .and_then(Value::as_str)
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))?;
    if kind != "agent_message_chunk" {
        return Err(SpikeError::new(SpikeErrorCode::ForbiddenUpdate));
    }
    value
        .pointer("/params/update/content/text")
        .and_then(Value::as_str)
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PromptState {
    Active,
    CancelRequested,
    Terminal,
}

impl PromptState {
    fn cancel(&mut self, child: &mut TestChild) -> Result<bool, SpikeError> {
        match self {
            Self::Active => {
                child.notify("session/cancel", json!({"sessionId":SESSION_ID}))?;
                *self = Self::CancelRequested;
                Ok(true)
            }
            Self::CancelRequested | Self::Terminal => Ok(false),
        }
    }

    fn accept_cancelled(&mut self, value: &Value) -> Result<(), SpikeError> {
        match self {
            Self::Active => return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation)),
            Self::CancelRequested => {}
            Self::Terminal => return Err(SpikeError::new(SpikeErrorCode::LateOutput)),
        }
        if expect_result(value, 3)?
            .get("stopReason")
            .and_then(Value::as_str)
            != Some("cancelled")
        {
            return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation));
        }
        *self = Self::Terminal;
        Ok(())
    }

    fn accept_text<'a>(&self, value: &'a Value) -> Result<&'a str, SpikeError> {
        if *self == Self::Terminal {
            return Err(SpikeError::new(SpikeErrorCode::LateOutput));
        }
        expect_text_update(value)
    }
}

fn initialize(child: &mut TestChild) -> Result<Value, SpikeError> {
    child.request(
        1,
        "initialize",
        json!({
            "protocolVersion": 1,
            "clientCapabilities": {},
            "clientInfo": {"name":"cortexa-fixture-host","version":"0.0.0"}
        }),
    )?;
    let response = child.read_json(STARTUP_DEADLINE)?;
    let result = expect_result(&response, 1)?;
    if result.get("protocolVersion").and_then(Value::as_u64) != Some(1)
        || result.pointer("/agentInfo/version").and_then(Value::as_str) != Some(EXPECTED_VERSION)
    {
        return Err(SpikeError::new(SpikeErrorCode::VersionMismatch));
    }
    Ok(response)
}

fn begin_prompt(child: &mut TestChild, prompt: &str) -> Result<(), SpikeError> {
    child.request(
        2,
        "session/new",
        json!({"cwd":child.temp.path(),"mcpServers":[]}),
    )?;
    let session = child.read_json(TEST_DEADLINE)?;
    if expect_result(&session, 2)?
        .get("sessionId")
        .and_then(Value::as_str)
        != Some(SESSION_ID)
    {
        return Err(SpikeError::new(SpikeErrorCode::IdentityMismatch));
    }
    child.request(
        3,
        "session/prompt",
        json!({
            "sessionId":SESSION_ID,
            "prompt":[{"type":"text","text":prompt}]
        }),
    )
}

#[test]
fn timeout_site_labels_are_closed_and_payload_free() {
    let probe = SpikeError::timeout(TimeoutSite::Probe(probe_label(OsStr::new(
        "private-probe-sentinel",
    ))));
    assert_eq!(
        probe.to_string(),
        "hermes_acp_transport_spike:timeout:probe:unknown_probe"
    );
    let read = SpikeError::timeout(TimeoutSite::Read {
        scenario: scenario_label("private-scenario-sentinel"),
        received_frames: 2,
    });
    assert_eq!(
        read.to_string(),
        "hermes_acp_transport_spike:timeout:read:unknown_scenario:2"
    );
    assert!(!format!("{probe:?} {read:?}").contains("private-"));
    assert_eq!(probe_label(OsStr::new("--version")), "version");
    assert_eq!(scenario_label("cancel"), "cancel");
}

#[test]
fn startup_allowance_preserves_protocol_and_negative_case_deadlines() {
    let expected_startup = if cfg!(target_os = "macos") {
        Duration::from_secs(10)
    } else {
        Duration::from_secs(2)
    };
    assert_eq!(STARTUP_DEADLINE, expected_startup);
    for arg in ["--version", "--check", "--flood-probe"] {
        assert_eq!(probe_deadline(OsStr::new(arg)), expected_startup);
    }
    for arg in ["--hang-probe", "unknown-probe"] {
        assert_eq!(probe_deadline(OsStr::new(arg)), Duration::from_secs(2));
    }
    assert_eq!(TEST_DEADLINE, Duration::from_secs(2));
    assert_eq!(TIMEOUT_DEADLINE, Duration::from_millis(120));
}

#[test]
fn validates_fixture_version_check_and_explicit_executable() -> Result<(), Box<dyn Error>> {
    run_probe(OsStr::new("--version"), EXPECTED_VERSION)?;
    run_probe(OsStr::new("--check"), "Hermes ACP fixture check OK")?;
    assert_eq!(
        validate_executable(Path::new("python3"))
            .err()
            .ok_or("relative executable must fail")?
            .code(),
        SpikeErrorCode::ExecutableRejected
    );
    assert_eq!(
        run_probe(OsStr::new("--hang-probe"), "never")
            .err()
            .ok_or("hanging probe must time out")?
            .code(),
        SpikeErrorCode::Timeout
    );
    assert_eq!(
        run_probe(OsStr::new("--flood-probe"), "never")
            .err()
            .ok_or("flooding probe must fail closed")?
            .code(),
        SpikeErrorCode::FrameTooLarge
    );
    Ok(())
}

#[test]
fn completes_initialize_session_text_and_eof_shutdown_in_isolation() -> Result<(), Box<dyn Error>> {
    let mut child = TestChild::spawn("noisy_stderr")?;
    let initialized = initialize(&mut child)?;
    let evidence = initialized
        .pointer("/result/_meta/fixture")
        .ok_or("fixture evidence missing")?;
    assert_eq!(
        evidence.get("isolatedPaths").and_then(Value::as_bool),
        Some(true)
    );
    let observed_cwd = evidence
        .get("cwd")
        .and_then(Value::as_str)
        .ok_or("fixture cwd missing")?;
    assert_eq!(
        fs::canonicalize(observed_cwd)?,
        fs::canonicalize(child.temp.path())?
    );
    let env_keys = evidence
        .get("envKeys")
        .and_then(Value::as_array)
        .ok_or("fixture env names missing")?;
    let required = [
        "CORTEXA_ACP_FIXTURE",
        "HOME",
        "HERMES_HOME",
        "PYTHONDONTWRITEBYTECODE",
        "PYTHONNOUSERSITE",
        "PYTHONSAFEPATH",
        "TMPDIR",
        "XDG_CACHE_HOME",
        "XDG_CONFIG_HOME",
        "XDG_DATA_HOME",
    ];
    let permitted_system_injected = [
        "CPATH",
        "LANG",
        "LANGUAGE",
        "LC_ALL",
        "LC_CTYPE",
        "LIBRARY_PATH",
        "MANPATH",
        "SDKROOT",
        "__CF_USER_TEXT_ENCODING",
    ];
    for required in required {
        assert!(env_keys.iter().any(|key| key.as_str() == Some(required)));
    }
    #[cfg(target_os = "macos")]
    assert!(env_keys
        .iter()
        .any(|key| key.as_str() == Some("DEVELOPER_DIR")));
    assert!(env_keys
        .iter()
        .all(|key| key.as_str().is_some_and(|key| required.contains(&key)
            || permitted_system_injected.contains(&key)
            || (cfg!(target_os = "macos") && key == "DEVELOPER_DIR"))));

    begin_prompt(
        &mut child,
        "fixture prompt; touch acp-shell-canary $(never-evaluated)",
    )?;
    let update = child.read_json(TEST_DEADLINE)?;
    assert_eq!(expect_text_update(&update)?, "fixture response");
    let terminal = child.read_json(TEST_DEADLINE)?;
    assert_eq!(
        expect_result(&terminal, 3)?
            .get("stopReason")
            .and_then(Value::as_str),
        Some("end_turn")
    );
    assert!(!child.temp.path().join("acp-shell-canary").exists());
    child.finish_gracefully()?;
    assert_eq!(child.stderr_summary().captured_bytes, MAX_STDERR_BYTES);
    assert!(child.stderr_summary().truncated);
    Ok(())
}

#[test]
fn bounds_timeouts_malformed_output_and_process_exit() -> Result<(), Box<dyn Error>> {
    let mut startup = TestChild::spawn("hang_before_initialize")?;
    startup.request(1, "initialize", json!({"protocolVersion":1}))?;
    let timeout = startup
        .read_json(TIMEOUT_DEADLINE)
        .err()
        .ok_or("startup timeout must fail")?;
    assert_eq!(timeout.code(), SpikeErrorCode::Timeout);
    assert!(startup.reaped);

    let mut in_run = TestChild::spawn("hang_after_update")?;
    initialize(&mut in_run)?;
    begin_prompt(&mut in_run, "fixture")?;
    let update = in_run.read_json(TEST_DEADLINE)?;
    assert_eq!(expect_text_update(&update)?, "fixture response");
    let timeout = in_run
        .read_json(TIMEOUT_DEADLINE)
        .err()
        .ok_or("in-run timeout must fail")?;
    assert_eq!(timeout.code(), SpikeErrorCode::Timeout);
    assert!(in_run.reaped);

    for (scenario, code) in [
        ("malformed", SpikeErrorCode::MalformedFrame),
        ("oversized", SpikeErrorCode::FrameTooLarge),
    ] {
        let mut child = TestChild::spawn(scenario)?;
        initialize(&mut child)?;
        let error = child
            .read_json(TEST_DEADLINE)
            .err()
            .ok_or("invalid stdout must fail")?;
        assert_eq!(error.code(), code);
        assert!(child.reaped);
    }

    for scenario in ["early_exit", "midstream_exit"] {
        let mut child = TestChild::spawn(scenario)?;
        if scenario == "midstream_exit" {
            initialize(&mut child)?;
            begin_prompt(&mut child, "fixture")?;
            let update = child.read_json(TEST_DEADLINE)?;
            assert_eq!(expect_text_update(&update)?, "fixture response");
        } else {
            child.request(1, "initialize", json!({"protocolVersion":1}))?;
        }
        let exit_deadline = if scenario == "early_exit" {
            STARTUP_DEADLINE
        } else {
            TEST_DEADLINE
        };
        let error = child
            .read_json(exit_deadline)
            .err()
            .ok_or("exit must fail closed")?;
        assert_eq!(error.code(), SpikeErrorCode::UnexpectedExit);
        assert!(!format!("{error:?}").contains("fixture-private-stderr-sentinel"));
        assert!(child.reaped);
    }
    Ok(())
}

#[test]
fn rejects_unknown_tool_wrong_session_and_wrong_id() -> Result<(), Box<dyn Error>> {
    let mut unknown = TestChild::spawn("unknown_method")?;
    initialize(&mut unknown)?;
    unknown.request(9, "future/unknown", json!({}))?;
    let response = unknown.read_json(TEST_DEADLINE)?;
    assert_eq!(response.get("id").and_then(Value::as_u64), Some(9));
    assert_eq!(
        response.pointer("/error/code").and_then(Value::as_i64),
        Some(-32601)
    );
    unknown.finish_gracefully()?;

    let mut tool = TestChild::spawn("tool_update")?;
    initialize(&mut tool)?;
    begin_prompt(&mut tool, "fixture")?;
    let first = tool.read_json(TEST_DEADLINE)?;
    assert_eq!(expect_text_update(&first)?, "fixture response");
    let forbidden = tool.read_json(TEST_DEADLINE)?;
    let error = expect_text_update(&forbidden)
        .err()
        .ok_or("tool update must fail")?;
    tool.force_reap()?;
    assert_eq!(error.code(), SpikeErrorCode::ForbiddenUpdate);

    let mut permission = TestChild::spawn("permission_request")?;
    initialize(&mut permission)?;
    begin_prompt(&mut permission, "fixture")?;
    let first = permission.read_json(TEST_DEADLINE)?;
    assert_eq!(expect_text_update(&first)?, "fixture response");
    let forbidden = permission.read_json(TEST_DEADLINE)?;
    let error = expect_text_update(&forbidden)
        .err()
        .ok_or("permission request must fail")?;
    permission.force_reap()?;
    assert_eq!(error.code(), SpikeErrorCode::ForbiddenUpdate);

    let mut wrong_session = TestChild::spawn("wrong_session")?;
    initialize(&mut wrong_session)?;
    begin_prompt(&mut wrong_session, "fixture")?;
    let first = wrong_session.read_json(TEST_DEADLINE)?;
    assert_eq!(expect_text_update(&first)?, "fixture response");
    let wrong = wrong_session.read_json(TEST_DEADLINE)?;
    let error = expect_text_update(&wrong)
        .err()
        .ok_or("wrong session must fail")?;
    wrong_session.force_reap()?;
    assert_eq!(error.code(), SpikeErrorCode::IdentityMismatch);

    let mut wrong_id = TestChild::spawn("wrong_id")?;
    initialize(&mut wrong_id)?;
    begin_prompt(&mut wrong_id, "fixture")?;
    let first = wrong_id.read_json(TEST_DEADLINE)?;
    assert_eq!(expect_text_update(&first)?, "fixture response");
    let terminal = wrong_id.read_json(TEST_DEADLINE)?;
    let error = expect_result(&terminal, 3)
        .err()
        .ok_or("wrong id must fail")?;
    wrong_id.force_reap()?;
    assert_eq!(error.code(), SpikeErrorCode::ProtocolViolation);
    Ok(())
}

#[test]
fn cancellation_is_terminal_idempotent_and_rejects_late_output() -> Result<(), Box<dyn Error>> {
    let mut child = TestChild::spawn("cancel")?;
    initialize(&mut child)?;
    begin_prompt(&mut child, "fixture")?;
    let first = child.read_json(TEST_DEADLINE)?;
    assert_eq!(expect_text_update(&first)?, "fixture response");

    let mut state = PromptState::Active;
    assert!(state.cancel(&mut child)?);
    assert!(!state.cancel(&mut child)?);
    let terminal = child.read_json(TEST_DEADLINE)?;
    state.accept_cancelled(&terminal)?;
    assert!(!state.cancel(&mut child)?);
    let late = child.read_json(TEST_DEADLINE)?;
    let late_error = state
        .accept_text(&late)
        .err()
        .ok_or("late output must fail")?;
    child.force_reap()?;
    assert_eq!(late_error.code(), SpikeErrorCode::LateOutput);
    assert!(!format!("{late_error:?}").contains("late-output-must-be-rejected"));
    Ok(())
}
