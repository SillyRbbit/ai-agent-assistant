#![cfg(unix)]

//! Isolated transport-mechanics spike for the pinned Hermes TUI-gateway wire.
//!
//! This target is not linked into the application and does not execute Hermes
//! during ordinary tests. Its cooperative fixture is not conformance or
//! containment evidence.

use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
#[cfg(target_os = "macos")]
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use tempfile::TempDir;

const PYTHON: &str = "/usr/bin/python3";
#[cfg(target_os = "macos")]
const XCODE_DEVELOPER_DIR: &str = "/Applications/Xcode.app/Contents/Developer";
const EXPECTED_VERSION_LINE: &str = "Hermes Agent v0.20.0 (2026.8.3)";
const SESSION_ID: &str = "session-spike-1";
const MAX_FRAME_BYTES: usize = 4_096;
const MAX_FRAME_COUNT: usize = 24;
const MAX_STDERR_BYTES: usize = 512;
const MAX_VERSION_BYTES: usize = 1_024;
const TEST_DEADLINE: Duration = Duration::from_secs(2);
const TIMEOUT_DEADLINE: Duration = Duration::from_millis(120);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpikeErrorCode {
    ExecutableRejected,
    VersionMismatch,
    VersionTimeout,
    SpawnFailed,
    IoFailure,
    ReadyTimeout,
    RunTimeout,
    FrameTooLarge,
    MalformedFrame,
    ProtocolViolation,
    ForbiddenEvent,
    IdentityMismatch,
    UnexpectedExit,
    LateOutput,
    ShutdownFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SpikeError {
    code: SpikeErrorCode,
}

impl SpikeError {
    const fn new(code: SpikeErrorCode) -> Self {
        Self { code }
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
            SpikeErrorCode::VersionTimeout => "version_timeout",
            SpikeErrorCode::SpawnFailed => "spawn_failed",
            SpikeErrorCode::IoFailure => "io_failure",
            SpikeErrorCode::ReadyTimeout => "ready_timeout",
            SpikeErrorCode::RunTimeout => "run_timeout",
            SpikeErrorCode::FrameTooLarge => "frame_too_large",
            SpikeErrorCode::MalformedFrame => "malformed_frame",
            SpikeErrorCode::ProtocolViolation => "protocol_violation",
            SpikeErrorCode::ForbiddenEvent => "forbidden_event",
            SpikeErrorCode::IdentityMismatch => "identity_mismatch",
            SpikeErrorCode::UnexpectedExit => "unexpected_exit",
            SpikeErrorCode::LateOutput => "late_output",
            SpikeErrorCode::ShutdownFailed => "shutdown_failed",
        };
        write!(formatter, "hermes_transport_spike:{code}")
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
    UnterminatedFrame,
    ReadFailure,
    Eof,
}

fn send_stdout_item(sender: &SyncSender<StdoutItem>, item: StdoutItem) -> bool {
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
        let mut buffer = Vec::with_capacity(256);
        let mut chunk = [0_u8; 256];
        loop {
            match reader.read(&mut chunk) {
                Ok(0) => {
                    if !buffer.is_empty()
                        && !send_stdout_item(&sender, StdoutItem::UnterminatedFrame)
                    {
                        return;
                    }
                    let _sent = send_stdout_item(&sender, StdoutItem::Eof);
                    return;
                }
                Ok(read) => {
                    for byte in &chunk[..read] {
                        if *byte == b'\n' {
                            let frame = std::mem::take(&mut buffer);
                            if !send_stdout_item(&sender, StdoutItem::Frame(frame)) {
                                return;
                            }
                        } else {
                            buffer.push(*byte);
                            if buffer.len() > MAX_FRAME_BYTES {
                                let _sent = send_stdout_item(&sender, StdoutItem::FrameTooLarge);
                                return;
                            }
                        }
                    }
                }
                Err(_) => {
                    let _sent = send_stdout_item(&sender, StdoutItem::ReadFailure);
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
        let summary = StderrSummary {
            captured_bytes: total.min(MAX_STDERR_BYTES),
            truncated: total > MAX_STDERR_BYTES,
        };
        let _sent = sender.try_send(summary);
    });
    (receiver, handle)
}

#[derive(Debug)]
struct Capture {
    bytes: Vec<u8>,
    truncated: bool,
    read_failed: bool,
}

fn spawn_capture_reader<R>(mut reader: R, limit: usize) -> JoinHandle<Capture>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut bytes = Vec::with_capacity(limit.min(256));
        let mut total = 0_usize;
        let mut read_failed = false;
        let mut chunk = [0_u8; 256];
        loop {
            match reader.read(&mut chunk) {
                Ok(0) => break,
                Ok(read) => {
                    total = total.saturating_add(read);
                    let remaining = limit.saturating_sub(bytes.len());
                    bytes.extend_from_slice(&chunk[..read.min(remaining)]);
                }
                Err(_) => {
                    read_failed = true;
                    break;
                }
            }
        }
        Capture {
            bytes,
            truncated: total > limit,
            read_failed,
        }
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DiagnosticStderrCategory {
    Empty,
    DeveloperTool,
    LoaderArchitecture,
    PythonStartup,
    OtherNonempty,
    ReadFailure,
}

impl DiagnosticStderrCategory {
    const fn label(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::DeveloperTool => "developer_tool",
            Self::LoaderArchitecture => "loader_architecture",
            Self::PythonStartup => "python_startup",
            Self::OtherNonempty => "other_nonempty",
            Self::ReadFailure => "read_failure",
        }
    }
}

#[derive(Default)]
struct DiagnosticStderrScanner {
    suffix: Vec<u8>,
    total: usize,
    developer_tool: bool,
    loader_architecture: bool,
    python_startup: bool,
    read_failed: bool,
}

impl DiagnosticStderrScanner {
    fn observe(&mut self, chunk: &[u8]) {
        self.total = self.total.saturating_add(chunk.len());
        let mut window = Vec::with_capacity(self.suffix.len() + chunk.len());
        window.extend_from_slice(&self.suffix);
        window.extend_from_slice(chunk);
        window.make_ascii_lowercase();
        let contains = |needle: &[u8]| window.windows(needle.len()).any(|part| part == needle);
        self.developer_tool |=
            contains(b"xcrun") || contains(b"xcode-select") || contains(b"active developer");
        self.loader_architecture |= contains(b"dyld")
            || contains(b"mach-o")
            || contains(b"architecture")
            || contains(b"bad cpu type");
        self.python_startup |= contains(b"traceback")
            || contains(b"modulenotfounderror")
            || contains(b"importerror")
            || contains(b"syntaxerror")
            || contains(b"python");
        self.suffix.clear();
        self.suffix
            .extend_from_slice(&window[window.len().saturating_sub(32)..]);
    }

    fn summary(&self) -> DiagnosticStderrSummary {
        let category = if self.read_failed {
            DiagnosticStderrCategory::ReadFailure
        } else if self.developer_tool {
            DiagnosticStderrCategory::DeveloperTool
        } else if self.loader_architecture {
            DiagnosticStderrCategory::LoaderArchitecture
        } else if self.python_startup {
            DiagnosticStderrCategory::PythonStartup
        } else if self.total == 0 {
            DiagnosticStderrCategory::Empty
        } else {
            DiagnosticStderrCategory::OtherNonempty
        };
        DiagnosticStderrSummary {
            category,
            truncated: self.total > MAX_STDERR_BYTES,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DiagnosticStderrSummary {
    category: DiagnosticStderrCategory,
    truncated: bool,
}

#[cfg(target_os = "macos")]
fn spawn_diagnostic_stderr_reader<R>(mut reader: R) -> JoinHandle<DiagnosticStderrSummary>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut scanner = DiagnosticStderrScanner::default();
        let mut chunk = [0_u8; 256];
        loop {
            match reader.read(&mut chunk) {
                Ok(0) => break,
                Ok(read) => scanner.observe(&chunk[..read]),
                Err(_) => {
                    scanner.read_failed = true;
                    break;
                }
            }
        }
        scanner.summary()
    })
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hermes_tui_gateway_stub.py")
}

fn validate_executable(path: &Path) -> Result<(), SpikeError> {
    if !path.is_absolute() {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    let symlink_metadata = fs::symlink_metadata(path)
        .map_err(|_| SpikeError::new(SpikeErrorCode::ExecutableRejected))?;
    if symlink_metadata.file_type().is_symlink() || !symlink_metadata.file_type().is_file() {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    if symlink_metadata.permissions().mode() & 0o111 == 0 {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    Ok(())
}

fn resolved_fixture_interpreter() -> Result<PathBuf, SpikeError> {
    let configured = Path::new(PYTHON);
    if !configured.is_absolute() {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    let resolved = fs::canonicalize(configured)
        .map_err(|_| SpikeError::new(SpikeErrorCode::ExecutableRejected))?;
    validate_executable(&resolved)?;
    Ok(resolved)
}

fn configure_isolated_command(command: &mut Command, temp: &TempDir) {
    command
        .env_clear()
        .env("HOME", temp.path())
        .env("TMPDIR", temp.path())
        .env("XDG_CONFIG_HOME", temp.path())
        .env("XDG_CACHE_HOME", temp.path())
        .env("XDG_DATA_HOME", temp.path())
        .env("CORTEXA_SPIKE_MARKER", "fixture-only")
        .current_dir(temp.path())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
}

#[cfg(target_os = "macos")]
#[derive(Clone, Copy)]
enum DiagnosticStage {
    Version,
    FirstReady,
}

#[cfg(target_os = "macos")]
impl DiagnosticStage {
    const fn label(self) -> &'static str {
        match self {
            Self::Version => "version",
            Self::FirstReady => "first_ready",
        }
    }
}

#[cfg(target_os = "macos")]
struct DiagnosticObservation {
    status: Option<ExitStatus>,
    output_matches: bool,
    stderr: DiagnosticStderrSummary,
}

#[cfg(target_os = "macos")]
impl DiagnosticObservation {
    fn write_closed(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "exit_code={:?} signal={:?} output_matches={} stderr_category={} stderr_truncated={}",
            self.status.as_ref().and_then(ExitStatus::code),
            self.status.as_ref().and_then(ExitStatusExt::signal),
            self.output_matches,
            self.stderr.category.label(),
            self.stderr.truncated
        )
    }
}

#[cfg(target_os = "macos")]
struct DiagnosticFailure {
    stage: DiagnosticStage,
    baseline: DiagnosticObservation,
    developer_dir: DiagnosticObservation,
}

#[cfg(target_os = "macos")]
impl fmt::Display for DiagnosticFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "stage={} baseline=(", self.stage.label())?;
        self.baseline.write_closed(formatter)?;
        write!(formatter, ") developer_dir=(")?;
        self.developer_dir.write_closed(formatter)?;
        write!(formatter, ")")
    }
}

#[cfg(target_os = "macos")]
impl fmt::Debug for DiagnosticFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

#[cfg(target_os = "macos")]
impl Error for DiagnosticFailure {}

#[cfg(target_os = "macos")]
fn diagnostic_command(
    stage: DiagnosticStage,
    developer_dir: bool,
) -> Result<(Command, TempDir), SpikeError> {
    let python = resolved_fixture_interpreter()?;
    let fixture = fixture_path();
    if !fixture.is_absolute() || !fixture.is_file() {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    let temp = TempDir::new().map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let mut command = Command::new(python);
    configure_isolated_command(&mut command, &temp);
    command.arg("-I").arg("-B").arg("-u").arg(fixture);
    command.arg(match stage {
        DiagnosticStage::Version => "--version",
        DiagnosticStage::FirstReady => "success",
    });
    if developer_dir {
        command.env("DEVELOPER_DIR", XCODE_DEVELOPER_DIR);
    }
    if matches!(stage, DiagnosticStage::FirstReady) {
        command.stdin(Stdio::piped());
    }
    Ok((command, temp))
}

#[cfg(target_os = "macos")]
fn stop_diagnostic_child(child: &mut Child) -> Result<(), SpikeError> {
    match child.try_wait() {
        Ok(Some(_)) => Ok(()),
        Ok(None) => {
            drop(child.kill());
            child
                .wait()
                .map(|_| ())
                .map_err(|_| SpikeError::new(SpikeErrorCode::ShutdownFailed))
        }
        Err(_) => Err(SpikeError::new(SpikeErrorCode::ShutdownFailed)),
    }
}

#[cfg(target_os = "macos")]
fn observe_version_startup(developer_dir: bool) -> Result<DiagnosticObservation, SpikeError> {
    let (mut command, _temp) = diagnostic_command(DiagnosticStage::Version, developer_dir)?;
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
    let stdout_reader = spawn_capture_reader(stdout, MAX_VERSION_BYTES);
    let stderr_reader = spawn_diagnostic_stderr_reader(stderr);
    let deadline = Instant::now() + TEST_DEADLINE;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(5)),
            Ok(None) => break None,
            Err(_) => break None,
        }
    };
    stop_diagnostic_child(&mut child)?;
    let stdout = stdout_reader
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let stderr = stderr_reader
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let output_matches = !stdout.truncated
        && !stdout.read_failed
        && std::str::from_utf8(&stdout.bytes).is_ok_and(|output| {
            let mut lines = output.lines();
            lines.next() == Some(EXPECTED_VERSION_LINE) && lines.count() <= 7
        });
    Ok(DiagnosticObservation {
        status,
        output_matches,
        stderr,
    })
}

#[cfg(target_os = "macos")]
fn observe_first_ready_startup(developer_dir: bool) -> Result<DiagnosticObservation, SpikeError> {
    let (mut command, _temp) = diagnostic_command(DiagnosticStage::FirstReady, developer_dir)?;
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
    let (stdout_receiver, stdout_reader) = spawn_stdout_reader(stdout);
    let stderr_reader = spawn_diagnostic_stderr_reader(stderr);
    let output_matches = match stdout_receiver.recv_timeout(TEST_DEADLINE) {
        Ok(StdoutItem::Frame(bytes)) => {
            serde_json::from_slice::<Value>(&bytes).is_ok_and(|value| {
                value.get("jsonrpc").and_then(Value::as_str) == Some("2.0")
                    && value.get("method").and_then(Value::as_str) == Some("event")
                    && value.pointer("/params/type").and_then(Value::as_str)
                        == Some("gateway.ready")
            })
        }
        _ => false,
    };
    let status = child
        .try_wait()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    stop_diagnostic_child(&mut child)?;
    stdout_reader
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let stderr = stderr_reader
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    Ok(DiagnosticObservation {
        status,
        output_matches,
        stderr,
    })
}

fn probe_version(path: &Path, args: &[OsString]) -> Result<(), SpikeError> {
    validate_executable(path)?;
    let temp = TempDir::new().map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let mut command = Command::new(path);
    configure_isolated_command(&mut command, &temp);
    command.args(args);
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
    let stdout_reader = spawn_capture_reader(stdout, MAX_VERSION_BYTES);
    let (stderr_receiver, stderr_reader) = spawn_stderr_reader(stderr);

    let deadline = Instant::now() + TEST_DEADLINE;
    let (status, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (status, false),
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(5)),
            Ok(None) => {
                drop(child.kill());
                let status = child
                    .wait()
                    .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
                break (status, true);
            }
            Err(_) => {
                drop(child.kill());
                drop(child.wait());
                return Err(SpikeError::new(SpikeErrorCode::IoFailure));
            }
        }
    };

    let stdout_capture = stdout_reader
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    stderr_reader
        .join()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let _stderr_summary = stderr_receiver.try_recv().unwrap_or_default();

    if timed_out {
        return Err(SpikeError::new(SpikeErrorCode::VersionTimeout));
    }
    if !status.success() || stdout_capture.truncated || stdout_capture.read_failed {
        return Err(SpikeError::new(SpikeErrorCode::VersionMismatch));
    }
    let output = std::str::from_utf8(&stdout_capture.bytes)
        .map_err(|_| SpikeError::new(SpikeErrorCode::VersionMismatch))?;
    let mut lines = output.lines();
    if lines.next() != Some(EXPECTED_VERSION_LINE) || lines.count() > 7 {
        return Err(SpikeError::new(SpikeErrorCode::VersionMismatch));
    }
    Ok(())
}

struct TestChild {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout_receiver: Receiver<StdoutItem>,
    stderr_receiver: Receiver<StderrSummary>,
    stdout_thread: Option<JoinHandle<()>>,
    stderr_thread: Option<JoinHandle<()>>,
    stderr_summary: Option<StderrSummary>,
    temp: TempDir,
    frame_count: usize,
    reaped: bool,
}

impl TestChild {
    fn spawn(scenario: &'static str) -> Result<Self, SpikeError> {
        let python = resolved_fixture_interpreter()?;
        let fixture = fixture_path();
        if !fixture.is_absolute() || !fixture.is_file() {
            return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
        }
        let temp = TempDir::new().map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
        let mut command = Command::new(python);
        command
            .arg("-I")
            .arg("-B")
            .arg("-u")
            .arg(&fixture)
            .arg(scenario);
        configure_isolated_command(&mut command, &temp);
        command.stdin(Stdio::piped());

        let mut child = command
            .spawn()
            .map_err(|_| SpikeError::new(SpikeErrorCode::SpawnFailed))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?;
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

        Ok(Self {
            child,
            stdin: Some(stdin),
            stdout_receiver,
            stderr_receiver,
            stdout_thread: Some(stdout_thread),
            stderr_thread: Some(stderr_thread),
            stderr_summary: None,
            temp,
            frame_count: 0,
            reaped: false,
        })
    }

    fn send_request(&mut self, id: u64, method: &str, params: Value) -> Result<(), SpikeError> {
        let mut bytes = serde_json::to_vec(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }))
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
        bytes.push(b'\n');
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| SpikeError::new(SpikeErrorCode::IoFailure))?;
        stdin
            .write_all(&bytes)
            .and_then(|()| stdin.flush())
            .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))
    }

    fn read_json(
        &mut self,
        timeout: Duration,
        timeout_code: SpikeErrorCode,
    ) -> Result<Value, SpikeError> {
        if self.frame_count >= MAX_FRAME_COUNT {
            return self.fail(SpikeErrorCode::ProtocolViolation);
        }
        let item = match self.stdout_receiver.recv_timeout(timeout) {
            Ok(item) => item,
            Err(mpsc::RecvTimeoutError::Timeout) => return self.fail(timeout_code),
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return self.fail(SpikeErrorCode::UnexpectedExit)
            }
        };
        match item {
            StdoutItem::Frame(bytes) => {
                self.frame_count += 1;
                let text = match std::str::from_utf8(&bytes) {
                    Ok(text) => text,
                    Err(_) => return self.fail(SpikeErrorCode::MalformedFrame),
                };
                match serde_json::from_str(text) {
                    Ok(value) => Ok(value),
                    Err(_) => self.fail(SpikeErrorCode::MalformedFrame),
                }
            }
            StdoutItem::FrameTooLarge => self.fail(SpikeErrorCode::FrameTooLarge),
            StdoutItem::UnterminatedFrame | StdoutItem::ReadFailure => {
                self.fail(SpikeErrorCode::MalformedFrame)
            }
            StdoutItem::Eof => self.fail(SpikeErrorCode::UnexpectedExit),
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
            Ok(Some(_status)) => {}
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

    fn finish_gracefully(&mut self) -> Result<ExitStatus, SpikeError> {
        self.stdin.take();
        let deadline = Instant::now() + TEST_DEADLINE;
        let status = loop {
            match self.child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(5)),
                Ok(None) | Err(_) => {
                    drop(self.child.kill());
                    let status = self
                        .child
                        .wait()
                        .map_err(|_| SpikeError::new(SpikeErrorCode::ShutdownFailed))?;
                    self.reaped = true;
                    self.finish_readers()?;
                    let _status = status;
                    return Err(SpikeError::new(SpikeErrorCode::ShutdownFailed));
                }
            }
        };
        self.reaped = true;
        self.finish_readers()?;
        if status.success() {
            Ok(status)
        } else {
            Err(SpikeError::new(SpikeErrorCode::UnexpectedExit))
        }
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
            self.stderr_summary = self.stderr_receiver.try_recv().ok();
        }
        Ok(())
    }

    fn stderr_summary(&self) -> StderrSummary {
        self.stderr_summary.unwrap_or_default()
    }

    fn temp_path(&self) -> &Path {
        self.temp.path()
    }

    const fn was_reaped(&self) -> bool {
        self.reaped
    }
}

impl Drop for TestChild {
    fn drop(&mut self) {
        let _cleanup_failed = self.force_reap().is_err();
    }
}

#[derive(Debug)]
struct ReadyEvidence {
    env_keys: Vec<String>,
    cwd: String,
    isolated_paths: bool,
}

fn expect_ready(value: &Value) -> Result<ReadyEvidence, SpikeError> {
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
        || value.get("method").and_then(Value::as_str) != Some("event")
        || value.pointer("/params/type").and_then(Value::as_str) != Some("gateway.ready")
    {
        return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation));
    }
    let env_keys = value
        .pointer("/params/payload/env_keys")
        .and_then(Value::as_array)
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))?
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let cwd = value
        .pointer("/params/payload/cwd")
        .and_then(Value::as_str)
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))?
        .to_owned();
    let isolated_paths = value
        .pointer("/params/payload/isolated_paths")
        .and_then(Value::as_bool)
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))?;
    Ok(ReadyEvidence {
        env_keys,
        cwd,
        isolated_paths,
    })
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

fn event_type(value: &Value) -> Result<&str, SpikeError> {
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
        || value.get("method").and_then(Value::as_str) != Some("event")
    {
        return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation));
    }
    value
        .pointer("/params/type")
        .and_then(Value::as_str)
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))
}

fn ensure_allowed_event(value: &Value) -> Result<&str, SpikeError> {
    let kind = event_type(value)?;
    match kind {
        "gateway.ready" | "message.start" | "message.delta" | "message.complete" => Ok(kind),
        _ => Err(SpikeError::new(SpikeErrorCode::ForbiddenEvent)),
    }
}

fn expect_session_event<'a>(
    value: &'a Value,
    expected_type: &str,
) -> Result<&'a Value, SpikeError> {
    if ensure_allowed_event(value)? != expected_type {
        return Err(SpikeError::new(SpikeErrorCode::ProtocolViolation));
    }
    if value.pointer("/params/session_id").and_then(Value::as_str) != Some(SESSION_ID) {
        return Err(SpikeError::new(SpikeErrorCode::IdentityMismatch));
    }
    value
        .pointer("/params/payload")
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))
}

#[derive(Debug)]
struct SuccessEvidence {
    text: String,
    ready: ReadyEvidence,
    temp_path: PathBuf,
    canary_exists: bool,
    stderr: StderrSummary,
    reaped: bool,
}

fn run_success(scenario: &'static str, prompt: &str) -> Result<SuccessEvidence, SpikeError> {
    let mut child = TestChild::spawn(scenario)?;
    let ready_frame = child.read_json(TEST_DEADLINE, SpikeErrorCode::ReadyTimeout)?;
    let ready = expect_ready(&ready_frame)?;

    child.send_request(1, "session.create", json!({}))?;
    let create = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    let create_result = expect_result(&create, 1)?;
    if create_result.get("session_id").and_then(Value::as_str) != Some(SESSION_ID)
        || create_result
            .pointer("/info/desktop_contract")
            .and_then(Value::as_u64)
            != Some(5)
    {
        return child.fail(SpikeErrorCode::ProtocolViolation);
    }

    child.send_request(
        2,
        "prompt.submit",
        json!({"session_id": SESSION_ID, "text": prompt}),
    )?;
    let prompt_result = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    if expect_result(&prompt_result, 2)?
        .get("status")
        .and_then(Value::as_str)
        != Some("streaming")
    {
        return child.fail(SpikeErrorCode::ProtocolViolation);
    }
    let start = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    expect_session_event(&start, "message.start")?;
    let delta = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    let text = expect_session_event(&delta, "message.delta")?
        .get("text")
        .and_then(Value::as_str)
        .ok_or_else(|| SpikeError::new(SpikeErrorCode::ProtocolViolation))?
        .to_owned();
    let complete = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    let complete_payload = expect_session_event(&complete, "message.complete")?;
    if complete_payload.get("status").and_then(Value::as_str) != Some("completed")
        || complete_payload.get("text").and_then(Value::as_str) != Some(text.as_str())
    {
        return child.fail(SpikeErrorCode::ProtocolViolation);
    }

    child.send_request(3, "session.close", json!({"session_id": SESSION_ID}))?;
    let close = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    if expect_result(&close, 3)?
        .get("status")
        .and_then(Value::as_str)
        != Some("closed")
    {
        return child.fail(SpikeErrorCode::ProtocolViolation);
    }
    let temp_path = fs::canonicalize(child.temp_path())
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let canary_exists = child.temp_path().join("spike-shell-canary").exists();
    child.finish_gracefully()?;
    Ok(SuccessEvidence {
        text,
        ready,
        temp_path,
        canary_exists,
        stderr: child.stderr_summary(),
        reaped: child.was_reaped(),
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CancellationState {
    Active,
    Requested,
    Terminal,
}

impl CancellationState {
    fn request(&mut self) -> bool {
        if *self == Self::Active {
            *self = Self::Requested;
            true
        } else {
            false
        }
    }

    fn finish(&mut self) {
        *self = Self::Terminal;
    }

    fn accept_event<'a>(
        self,
        value: &'a Value,
        expected_type: &str,
    ) -> Result<&'a Value, SpikeError> {
        if self == Self::Terminal {
            Err(SpikeError::new(SpikeErrorCode::LateOutput))
        } else {
            expect_session_event(value, expected_type)
        }
    }
}

fn begin_run(child: &mut TestChild) -> Result<(), SpikeError> {
    let ready = child.read_json(TEST_DEADLINE, SpikeErrorCode::ReadyTimeout)?;
    expect_ready(&ready)?;
    child.send_request(1, "session.create", json!({}))?;
    let create = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    expect_result(&create, 1)?;
    child.send_request(
        2,
        "prompt.submit",
        json!({"session_id": SESSION_ID, "text": "fixture request"}),
    )?;
    let prompt = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    expect_result(&prompt, 2)?;
    let start = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    expect_session_event(&start, "message.start")?;
    Ok(())
}

#[test]
fn validates_explicit_executable_and_pinned_version() -> Result<(), Box<dyn Error>> {
    let fixture = fixture_path();
    let python = resolved_fixture_interpreter()?;
    let valid_args = [
        OsString::from("-I"),
        OsString::from("-B"),
        OsString::from("-u"),
        fixture.as_os_str().to_owned(),
        OsString::from("--version"),
    ];
    probe_version(&python, &valid_args)?;

    let wrong_args = [
        OsString::from("-I"),
        OsString::from("-B"),
        OsString::from("-u"),
        fixture.as_os_str().to_owned(),
        OsString::from("--wrong-version"),
    ];
    let wrong = probe_version(&python, &wrong_args)
        .err()
        .ok_or("wrong version must fail")?;
    assert_eq!(wrong.code(), SpikeErrorCode::VersionMismatch);
    assert_eq!(
        validate_executable(Path::new("python3"))
            .err()
            .ok_or("relative path must fail")?
            .code(),
        SpikeErrorCode::ExecutableRejected
    );
    assert_eq!(
        validate_executable(Path::new("/definitely/missing/hermes"))
            .err()
            .ok_or("missing path must fail")?
            .code(),
        SpikeErrorCode::ExecutableRejected
    );
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn compares_isolated_version_startup_with_xcode_child_only() -> Result<(), Box<dyn Error>> {
    let baseline = observe_version_startup(false)?;
    let developer_dir = observe_version_startup(true)?;
    let baseline_passed =
        baseline.status.as_ref().is_some_and(ExitStatus::success) && baseline.output_matches;
    let comparison = DiagnosticFailure {
        stage: DiagnosticStage::Version,
        baseline,
        developer_dir,
    };
    eprintln!("{comparison}");
    if baseline_passed {
        Ok(())
    } else {
        Err(Box::new(comparison))
    }
}

#[cfg(target_os = "macos")]
#[test]
fn compares_isolated_first_ready_with_xcode_child_only() -> Result<(), Box<dyn Error>> {
    let baseline = observe_first_ready_startup(false)?;
    let developer_dir = observe_first_ready_startup(true)?;
    let baseline_passed = baseline.output_matches;
    let comparison = DiagnosticFailure {
        stage: DiagnosticStage::FirstReady,
        baseline,
        developer_dir,
    };
    eprintln!("{comparison}");
    if baseline_passed {
        Ok(())
    } else {
        Err(Box::new(comparison))
    }
}

#[test]
fn diagnostic_stderr_scanner_finds_closed_signature_after_cap_and_across_chunks() {
    let mut scanner = DiagnosticStderrScanner::default();
    scanner.observe(&vec![b'x'; MAX_STDERR_BYTES + 1]);
    scanner.observe(b"xc");
    scanner.observe(b"run private-sentinel");
    assert_eq!(
        scanner.summary(),
        DiagnosticStderrSummary {
            category: DiagnosticStderrCategory::DeveloperTool,
            truncated: true,
        }
    );
    assert!(scanner.suffix.len() <= 32);
}

#[cfg(target_os = "macos")]
#[test]
fn diagnostic_failure_format_is_closed_and_redacted() {
    let mut scanner = DiagnosticStderrScanner::default();
    scanner.observe(b"/private/sensitive-path secret-token private-sentinel");
    let summary = scanner.summary();
    let observation = DiagnosticObservation {
        status: None,
        output_matches: false,
        stderr: summary,
    };
    let failure = DiagnosticFailure {
        stage: DiagnosticStage::FirstReady,
        baseline: observation,
        developer_dir: DiagnosticObservation {
            status: None,
            output_matches: true,
            stderr: summary,
        },
    };
    let line = format!("{failure:?}");
    assert!(line.contains("stage=first_ready baseline=(exit_code=None signal=None"));
    assert!(line.contains("stderr_category=other_nonempty stderr_truncated=false"));
    for secret in ["sensitive-path", "secret-token", "private-sentinel"] {
        assert!(!line.contains(secret));
    }
    assert!(line.len() < 300);
}

#[test]
fn completes_ready_session_text_and_clean_shutdown() -> Result<(), Box<dyn Error>> {
    let evidence = run_success(
        "success",
        "fixture prompt; touch spike-shell-canary $(never-evaluated)",
    )?;
    assert_eq!(evidence.text, "fixture response");
    assert!(!evidence.canary_exists);
    assert!(evidence.reaped);
    assert_eq!(evidence.stderr, StderrSummary::default());
    Ok(())
}

#[test]
fn bounds_readiness_and_in_run_timeouts_and_reaps() -> Result<(), Box<dyn Error>> {
    let mut before_ready = TestChild::spawn("hang_before_ready")?;
    let ready_error = before_ready
        .read_json(TIMEOUT_DEADLINE, SpikeErrorCode::ReadyTimeout)
        .err()
        .ok_or("readiness timeout must fail")?;
    assert_eq!(ready_error.code(), SpikeErrorCode::ReadyTimeout);
    assert!(before_ready.was_reaped());

    let mut during_run = TestChild::spawn("hang_after_start")?;
    begin_run(&mut during_run)?;
    let run_error = during_run
        .read_json(TIMEOUT_DEADLINE, SpikeErrorCode::RunTimeout)
        .err()
        .ok_or("run timeout must fail")?;
    assert_eq!(run_error.code(), SpikeErrorCode::RunTimeout);
    assert!(during_run.was_reaped());
    Ok(())
}

#[test]
fn cancellation_is_terminal_idempotent_and_rejects_late_output() -> Result<(), Box<dyn Error>> {
    let mut child = TestChild::spawn("cancel")?;
    begin_run(&mut child)?;
    let mut cancellation = CancellationState::Active;
    assert!(cancellation.request());
    child.send_request(3, "session.interrupt", json!({"session_id": SESSION_ID}))?;
    assert!(!cancellation.request());

    let interrupted = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    if expect_result(&interrupted, 3)?
        .get("status")
        .and_then(Value::as_str)
        != Some("interrupted")
    {
        return Err(Box::new(SpikeError::new(SpikeErrorCode::ProtocolViolation)));
    }
    let complete = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    if expect_session_event(&complete, "message.complete")?
        .get("status")
        .and_then(Value::as_str)
        != Some("interrupted")
    {
        return Err(Box::new(SpikeError::new(SpikeErrorCode::ProtocolViolation)));
    }
    cancellation.finish();
    let late = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    let late_error = cancellation
        .accept_event(&late, "message.delta")
        .err()
        .ok_or("terminal state must reject late output")?;
    child.force_reap()?;
    assert_eq!(cancellation, CancellationState::Terminal);
    assert_eq!(late_error.code(), SpikeErrorCode::LateOutput);
    assert!(!format!("{late_error:?}").contains("late-output-must-be-rejected"));
    assert!(child.was_reaped());
    Ok(())
}

#[test]
fn rejects_malformed_oversized_forbidden_and_mismatched_frames() -> Result<(), Box<dyn Error>> {
    for (scenario, expected) in [
        ("malformed", SpikeErrorCode::MalformedFrame),
        ("invalid_utf8", SpikeErrorCode::MalformedFrame),
        ("oversized", SpikeErrorCode::FrameTooLarge),
    ] {
        let mut child = TestChild::spawn(scenario)?;
        let ready = child.read_json(TEST_DEADLINE, SpikeErrorCode::ReadyTimeout)?;
        expect_ready(&ready)?;
        let error = child
            .read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)
            .err()
            .ok_or("invalid frame must fail")?;
        assert_eq!(error.code(), expected);
        assert!(child.was_reaped());
    }

    for scenario in ["forbidden", "unknown"] {
        let mut child = TestChild::spawn(scenario)?;
        let ready = child.read_json(TEST_DEADLINE, SpikeErrorCode::ReadyTimeout)?;
        expect_ready(&ready)?;
        let frame = child.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
        let error = ensure_allowed_event(&frame)
            .err()
            .ok_or("unknown event must fail")?;
        child.force_reap()?;
        assert_eq!(error.code(), SpikeErrorCode::ForbiddenEvent);
        assert!(child.was_reaped());
    }

    let mut wrong_identity = TestChild::spawn("wrong_identity")?;
    let ready = wrong_identity.read_json(TEST_DEADLINE, SpikeErrorCode::ReadyTimeout)?;
    expect_ready(&ready)?;
    let frame = wrong_identity.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    let error = expect_session_event(&frame, "message.delta")
        .err()
        .ok_or("wrong identity must fail")?;
    wrong_identity.force_reap()?;
    assert_eq!(error.code(), SpikeErrorCode::IdentityMismatch);
    assert!(wrong_identity.was_reaped());
    Ok(())
}

#[test]
fn maps_early_and_midstream_exit_to_closed_redacted_failure() -> Result<(), Box<dyn Error>> {
    let mut early = TestChild::spawn("early_exit")?;
    let early_error = early
        .read_json(TEST_DEADLINE, SpikeErrorCode::ReadyTimeout)
        .err()
        .ok_or("early exit must fail")?;
    assert_eq!(early_error.code(), SpikeErrorCode::UnexpectedExit);
    assert!(!format!("{early_error:?}").contains("fixture-private-stderr-sentinel"));
    assert!(!early_error
        .to_string()
        .contains("fixture-private-stderr-sentinel"));
    assert!(early.stderr_summary().captured_bytes > 0);
    assert!(early.was_reaped());

    let mut midstream = TestChild::spawn("midstream_exit")?;
    begin_run(&mut midstream)?;
    let partial = midstream.read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)?;
    assert_eq!(ensure_allowed_event(&partial)?, "message.delta");
    let exit_error = midstream
        .read_json(TEST_DEADLINE, SpikeErrorCode::RunTimeout)
        .err()
        .ok_or("midstream exit must fail")?;
    assert_eq!(exit_error.code(), SpikeErrorCode::UnexpectedExit);
    assert!(midstream.was_reaped());
    Ok(())
}

#[test]
fn clears_environment_isolates_paths_and_bounds_stderr() -> Result<(), Box<dyn Error>> {
    let evidence = run_success("noisy_stderr", "private-prompt-sentinel")?;
    let required = [
        "CORTEXA_SPIKE_MARKER",
        "HOME",
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
    assert!(required.iter().all(|required| evidence
        .ready
        .env_keys
        .iter()
        .any(|key| key == required)));
    assert!(evidence
        .ready
        .env_keys
        .iter()
        .all(|key| required.contains(&key.as_str())
            || permitted_system_injected.contains(&key.as_str())));
    assert!(evidence.ready.isolated_paths);
    assert_eq!(Path::new(&evidence.ready.cwd), evidence.temp_path);
    assert_eq!(evidence.stderr.captured_bytes, MAX_STDERR_BYTES);
    assert!(evidence.stderr.truncated);
    assert!(evidence.reaped);
    let closed_error = SpikeError::new(SpikeErrorCode::UnexpectedExit);
    assert!(!format!("{closed_error:?}").contains("fixture-private-stderr-sentinel"));
    assert!(!closed_error.to_string().contains("private-prompt-sentinel"));
    Ok(())
}

#[test]
#[ignore = "requires explicit opt-in and an operator-supplied pinned Hermes executable"]
fn real_hermes_version_probe_is_opt_in_and_version_only() -> Result<(), Box<dyn Error>> {
    if std::env::var("CORTEXA_REAL_HERMES_TEST").as_deref() != Ok("1") {
        eprintln!("real Hermes probe skipped: CORTEXA_REAL_HERMES_TEST is not 1");
        return Ok(());
    }
    let path = std::env::var_os("CORTEXA_REAL_HERMES_BIN")
        .map(PathBuf::from)
        .ok_or("CORTEXA_REAL_HERMES_BIN must name an absolute pinned executable")?;
    probe_version(&path, &[OsString::from("--version")])?;
    Ok(())
}
