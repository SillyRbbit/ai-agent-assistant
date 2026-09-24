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
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use tempfile::TempDir;

const PYTHON: &str = "/usr/bin/python3";
const SESSION_ID: &str = "acp-session-fixture-1";
const EXPECTED_VERSION: &str = "0.20.0";
const MAX_FRAME_BYTES: usize = 4_096;
const MAX_FRAME_COUNT: usize = 24;
const MAX_STDERR_BYTES: usize = 512;
const STDERR_SIGNATURE_OVERLAP: usize = b"modulenotfounderror".len() - 1;
const TEST_DEADLINE: Duration = Duration::from_secs(2);
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
        write!(formatter, "hermes_acp_transport_spike:{code}")
    }
}

impl Error for SpikeError {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct StderrSummary {
    captured_bytes: usize,
    truncated: bool,
    category: StderrCategory,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum StderrCategory {
    #[default]
    Empty,
    DeveloperTool,
    LoaderArchitecture,
    PythonStartup,
    OtherNonempty,
    Truncated,
    ReadFailure,
}

impl StderrCategory {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::DeveloperTool => "developer_tool",
            Self::LoaderArchitecture => "loader_architecture",
            Self::PythonStartup => "python_startup",
            Self::OtherNonempty => "other_nonempty",
            Self::Truncated => "truncated",
            Self::ReadFailure => "read_failure",
        }
    }
}

fn classify_stderr(sample: &[u8], truncated: bool, read_failed: bool) -> StderrCategory {
    if read_failed {
        return StderrCategory::ReadFailure;
    }
    if truncated {
        return StderrCategory::Truncated;
    }
    if sample.is_empty() {
        return StderrCategory::Empty;
    }
    let lower = sample.to_ascii_lowercase();
    let contains = |needle: &[u8]| lower.windows(needle.len()).any(|window| window == needle);
    if contains(b"xcrun") || contains(b"xcode-select") || contains(b"active developer") {
        StderrCategory::DeveloperTool
    } else if contains(b"dyld")
        || contains(b"mach-o")
        || contains(b"architecture")
        || contains(b"bad cpu type")
    {
        StderrCategory::LoaderArchitecture
    } else if contains(b"traceback")
        || contains(b"modulenotfounderror")
        || contains(b"importerror")
        || contains(b"syntaxerror")
        || contains(b"python")
    {
        StderrCategory::PythonStartup
    } else {
        StderrCategory::OtherNonempty
    }
}

#[derive(Default)]
struct StderrSignatureScanner {
    suffix: Vec<u8>,
    developer_tool: bool,
    loader_architecture: bool,
    python_startup: bool,
}

impl StderrSignatureScanner {
    fn observe(&mut self, chunk: &[u8]) {
        let mut window = Vec::with_capacity(self.suffix.len() + chunk.len());
        window.extend_from_slice(&self.suffix);
        window.extend_from_slice(chunk);
        match classify_stderr(&window, false, false) {
            StderrCategory::DeveloperTool => self.developer_tool = true,
            StderrCategory::LoaderArchitecture => self.loader_architecture = true,
            StderrCategory::PythonStartup => self.python_startup = true,
            _ => {}
        }
        let suffix_start = window.len().saturating_sub(STDERR_SIGNATURE_OVERLAP);
        self.suffix.clear();
        self.suffix.extend_from_slice(&window[suffix_start..]);
    }

    const fn category(&self) -> Option<StderrCategory> {
        if self.developer_tool {
            Some(StderrCategory::DeveloperTool)
        } else if self.loader_architecture {
            Some(StderrCategory::LoaderArchitecture)
        } else if self.python_startup {
            Some(StderrCategory::PythonStartup)
        } else {
            None
        }
    }
}

fn version_probe_diagnostic(
    status: &ExitStatus,
    output_matches: bool,
    category: StderrCategory,
) -> String {
    format!(
        "hermes_acp_fixture_version_probe: exit_code={:?} signal={:?} output_matches={output_matches} stderr_category={}",
        status.code(),
        status.signal(),
        category.as_str()
    )
}

fn version_probe_line(
    status: &ExitStatus,
    output_matches: bool,
    category: StderrCategory,
    truncated: bool,
) -> String {
    format!(
        "{} stderr_truncated={truncated}",
        version_probe_diagnostic(status, output_matches, category)
    )
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
        let mut sample = Vec::with_capacity(MAX_STDERR_BYTES);
        let mut signatures = StderrSignatureScanner::default();
        let mut read_failed = false;
        let mut chunk = [0_u8; 256];
        loop {
            match reader.read(&mut chunk) {
                Ok(0) => break,
                Ok(read) => {
                    total = total.saturating_add(read);
                    signatures.observe(&chunk[..read]);
                    let remaining = MAX_STDERR_BYTES.saturating_sub(sample.len());
                    sample.extend_from_slice(&chunk[..read.min(remaining)]);
                }
                Err(_) => {
                    read_failed = true;
                    break;
                }
            }
        }
        let _sent = sender.try_send(StderrSummary {
            captured_bytes: total.min(MAX_STDERR_BYTES),
            truncated: total > MAX_STDERR_BYTES,
            category: if read_failed {
                StderrCategory::ReadFailure
            } else {
                signatures
                    .category()
                    .unwrap_or_else(|| classify_stderr(&sample, total > MAX_STDERR_BYTES, false))
            },
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

    let deadline = Instant::now() + TEST_DEADLINE;
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
                return Err(SpikeError::new(SpikeErrorCode::Timeout));
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
        if arg == OsStr::new("--version") {
            eprintln!(
                "{}",
                version_probe_line(&status, output_matches, stderr.category, stderr.truncated)
            );
            return Err(SpikeError::new(SpikeErrorCode::VersionMismatch));
        }
        return Err(SpikeError::new(SpikeErrorCode::CheckFailed));
    }
    Ok(())
}

// This diagnostic leaves run_probe and the fixture assertions unchanged. The
// second child differs only by the already verified Xcode developer directory.
fn compare_version_child(with_developer_dir: bool) -> Result<String, SpikeError> {
    let python = resolved_python()?;
    let fixture = fixture_path();
    if !fixture.is_absolute() || !fixture.is_file() {
        return Err(SpikeError::new(SpikeErrorCode::ExecutableRejected));
    }
    let temp = TempDir::new().map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let mut command = Command::new(python);
    configure_isolated(&mut command, &temp);
    if with_developer_dir {
        command.env(
            "DEVELOPER_DIR",
            "/Applications/Xcode.app/Contents/Developer",
        );
    }
    command
        .arg("-I")
        .arg("-B")
        .arg("-u")
        .arg(fixture)
        .arg("--version")
        .stdin(Stdio::null());
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

    let deadline = Instant::now() + Duration::from_secs(10);
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
                return Err(SpikeError::new(SpikeErrorCode::Timeout));
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
    let mut stdout_valid = true;
    for item in stdout_receiver.try_iter() {
        match item {
            StdoutItem::Frame(bytes) if frame.is_none() => frame = Some(bytes),
            StdoutItem::Eof => {}
            StdoutItem::Frame(_)
            | StdoutItem::FrameTooLarge
            | StdoutItem::Unterminated
            | StdoutItem::ReadFailure => stdout_valid = false,
        }
    }
    let stderr = stderr_receiver
        .try_recv()
        .map_err(|_| SpikeError::new(SpikeErrorCode::IoFailure))?;
    let output_matches = stdout_valid
        && frame
            .as_deref()
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .map(str::trim)
            == Some(EXPECTED_VERSION);
    let label = if with_developer_dir {
        "developer_dir"
    } else {
        "baseline"
    };
    Ok(format!(
        "hermes_acp_fixture_startup_comparison {label}: exit_code={:?} signal={:?} output_matches={output_matches} stderr_category={} stderr_truncated={}",
        status.code(),
        status.signal(),
        stderr.category.as_str(),
        stderr.truncated
    ))
}

#[test]
fn compares_isolated_version_startup_with_xcode_child_only() -> Result<(), SpikeError> {
    let baseline = compare_version_child(false)?;
    eprintln!("{baseline}");
    let developer_dir = compare_version_child(true)?;
    eprintln!("{developer_dir}");
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
            Err(mpsc::RecvTimeoutError::Timeout) => self.fail(SpikeErrorCode::Timeout),
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
    let response = child.read_json(TEST_DEADLINE)?;
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
fn version_probe_diagnostic_is_closed_and_redacted() {
    let private_stderr = b"Traceback: fixture-private-stderr-sentinel";
    let category = classify_stderr(private_stderr, false, false);
    let status = ExitStatus::from_raw(1 << 8);
    let diagnostic = version_probe_diagnostic(&status, false, category);
    assert_eq!(category, StderrCategory::PythonStartup);
    assert_eq!(
        diagnostic,
        "hermes_acp_fixture_version_probe: exit_code=Some(1) signal=None output_matches=false stderr_category=python_startup"
    );
    assert!(!diagnostic.contains("fixture-private-stderr-sentinel"));
    for (sample, expected) in [
        (
            b"xcrun: tool unavailable".as_slice(),
            StderrCategory::DeveloperTool,
        ),
        (
            b"dyld: missing architecture".as_slice(),
            StderrCategory::LoaderArchitecture,
        ),
        (
            b"unrecognized private detail".as_slice(),
            StderrCategory::OtherNonempty,
        ),
    ] {
        assert_eq!(classify_stderr(sample, false, false), expected);
    }
    assert_eq!(classify_stderr(b"", false, false), StderrCategory::Empty);
    assert_eq!(
        classify_stderr(private_stderr, true, false),
        StderrCategory::Truncated
    );
    assert_eq!(
        classify_stderr(private_stderr, false, true),
        StderrCategory::ReadFailure
    );
}

#[test]
fn streamed_stderr_classifies_late_signatures_without_revealing_content(
) -> Result<(), Box<dyn Error>> {
    let status = ExitStatus::from_raw(1 << 8);
    for (signature, expected) in [
        (b"xcrun".as_slice(), StderrCategory::DeveloperTool),
        (b"dyld".as_slice(), StderrCategory::LoaderArchitecture),
        (b"Traceback".as_slice(), StderrCategory::PythonStartup),
    ] {
        let mut stderr = vec![b'z'; MAX_STDERR_BYTES - 3];
        stderr.extend_from_slice(signature);
        stderr.extend_from_slice(b": fixture-private-stderr-sentinel");
        let (receiver, handle) = spawn_stderr_reader(std::io::Cursor::new(stderr));
        handle
            .join()
            .map_err(|_| "test stderr reader must finish")?;
        let summary = receiver
            .try_recv()
            .map_err(|_| "test stderr summary must exist")?;
        assert_eq!(summary.category, expected);
        assert!(summary.truncated);
        assert_eq!(summary.captured_bytes, MAX_STDERR_BYTES);
        let diagnostic = version_probe_line(&status, false, summary.category, summary.truncated);
        assert!(diagnostic.contains("stderr_truncated=true"));
        assert!(diagnostic.contains(expected.as_str()));
        assert!(!diagnostic.contains("fixture-private-stderr-sentinel"));
    }

    let (receiver, handle) =
        spawn_stderr_reader(std::io::Cursor::new(vec![b'z'; MAX_STDERR_BYTES + 1]));
    handle
        .join()
        .map_err(|_| "test stderr reader must finish")?;
    let summary = receiver
        .try_recv()
        .map_err(|_| "test stderr summary must exist")?;
    assert_eq!(summary.category, StderrCategory::Truncated);
    assert!(summary.truncated);
    assert_eq!(summary.captured_bytes, MAX_STDERR_BYTES);
    assert_eq!(
        version_probe_line(&status, false, summary.category, summary.truncated),
        "hermes_acp_fixture_version_probe: exit_code=Some(1) signal=None output_matches=false stderr_category=truncated stderr_truncated=true"
    );
    Ok(())
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
    assert!(env_keys.iter().all(|key| key
        .as_str()
        .is_some_and(|key| required.contains(&key) || permitted_system_injected.contains(&key))));

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
        let error = child
            .read_json(TEST_DEADLINE)
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
