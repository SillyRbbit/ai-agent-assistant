//! Rust-owned fixed native speech process. The WebView never receives audio or an executor.
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::agent_chat_tauri::AgentChatState;
use crate::voice::{Operation, Protocol, Snapshot, Status, VoiceError, MAX_CAPTURE, MAX_FRAME};
use serde::Deserialize;

#[derive(Default)]
pub(crate) struct VoiceState(Arc<Mutex<Owner>>);
#[derive(Default)]
struct Owner {
    next: u64,
    blocked: bool,
    active: Option<Running>,
    last: Option<Snapshot>,
}
impl Owner {
    fn can_start(&self) -> Result<(), VoiceError> {
        if self.blocked {
            return Err(VoiceError::CleanupFailed);
        }
        if self.active.is_some()
            || self
                .last
                .as_ref()
                .is_some_and(|s| s.status == Status::Review)
        {
            return Err(VoiceError::Busy);
        }
        Ok(())
    }
    fn finish_owned(&mut self, mut snapshot: Snapshot, operation: Operation) -> bool {
        // Process reaping does not establish an unconfirmed synthesizer stop.
        // Preserve the primary error and retain this exact native chat lease.
        self.blocked |= snapshot.error == Some(VoiceError::CleanupFailed)
            || (operation == Operation::Speak
                && snapshot.error.is_some()
                && snapshot.error != Some(VoiceError::Unavailable));
        if self.blocked {
            snapshot.busy = true;
        }
        self.last = Some(snapshot);
        self.active = None;
        !self.blocked
    }
    fn retire_previous_review(&mut self, current: &str) {
        if !self.blocked
            && self.active.is_none()
            && self
                .last
                .as_ref()
                .is_some_and(|s| s.conversation_id != current)
        {
            self.last = None;
        }
    }
    fn discard_review(&mut self, request: &SessionRequest) -> Result<Snapshot, VoiceError> {
        let last = self.last.as_mut().ok_or(VoiceError::StaleSession)?;
        if last.session_id != request.session_id || last.conversation_id != request.conversation_id
        {
            return Err(VoiceError::StaleSession);
        }
        if last.status == Status::Review {
            last.text = None;
            last.status = Status::Cancelled;
            last.sequence = last.sequence.checked_add(1).ok_or(VoiceError::Limit)?;
        }
        Ok(last.clone())
    }
}
struct Running {
    child: Child,
    input: Option<mpsc::SyncSender<Vec<u8>>>,
    readers: Vec<JoinHandle<()>>,
    frames: mpsc::Receiver<Vec<u8>>,
    failed_capture: Arc<AtomicBool>,
    helper_diagnostics: Arc<AtomicBool>,
    closed: Arc<AtomicUsize>,
    protocol: Protocol,
    deadline: Instant,
    stopping: Option<Instant>,
    exited: Option<bool>,
}

fn encode_frame(session: &str, operation: &str, text: Option<&str>) -> Result<Vec<u8>, VoiceError> {
    let value =
        serde_json::json!({"version":1,"session":session,"operation":operation,"text":text});
    let mut data = serde_json::to_vec(&value).map_err(|_| VoiceError::Protocol)?;
    if data.len() > MAX_FRAME {
        return Err(VoiceError::Limit);
    }
    data.push(b'\n');
    Ok(data)
}
fn send_frame(
    input: &Option<mpsc::SyncSender<Vec<u8>>>,
    session: &str,
    operation: &str,
    text: Option<&str>,
) -> Result<(), VoiceError> {
    input
        .as_ref()
        .ok_or(VoiceError::Protocol)?
        .try_send(encode_frame(session, operation, text)?)
        .map_err(|_| VoiceError::Protocol)
}

fn capture_frames(reader: impl Read, tx: &mpsc::SyncSender<Vec<u8>>, failed: &AtomicBool) {
    let mut reader = BufReader::new(reader);
    let mut total = 0;
    loop {
        let mut line = Vec::new();
        match reader
            .by_ref()
            .take((MAX_FRAME + 2) as u64)
            .read_until(b'\n', &mut line)
        {
            Ok(0) => break,
            Ok(_) if line.len() <= MAX_FRAME + 1 && line.last() == Some(&b'\n') => {
                total += line.len();
                line.pop();
                if total > MAX_CAPTURE || tx.try_send(line).is_err() {
                    failed.store(true, Ordering::SeqCst);
                    break;
                }
            }
            _ => {
                failed.store(true, Ordering::SeqCst);
                break;
            }
        }
    }
}

// Retain a fixed negative witness only. Never retain or print diagnostic bytes.
fn capture_diagnostics(mut reader: impl Read, failed: &AtomicBool, seen: &AtomicBool) {
    let mut bytes = [0u8; 4096];
    let mut total = 0;
    loop {
        match reader.read(&mut bytes) {
            Ok(0) => break,
            Ok(count) => {
                seen.store(true, Ordering::SeqCst);
                failed.store(true, Ordering::SeqCst);
                total += count;
                if total >= MAX_CAPTURE {
                    break;
                }
            }
            Err(_) => {
                failed.store(true, Ordering::SeqCst);
                break;
            }
        }
    }
}

#[cfg(target_os = "macos")]
#[derive(PartialEq, Eq)]
struct HelperIdentity {
    device: u64,
    inode: u64,
    owner: u32,
    group: u32,
    mode: u32,
    links: u64,
    size: u64,
    modified: (i64, i64),
    changed: (i64, i64),
}
#[cfg(not(target_os = "macos"))]
type HelperIdentity = ();
#[cfg(target_os = "macos")]
impl HelperIdentity {
    fn read(m: &std::fs::Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        Self {
            device: m.dev(),
            inode: m.ino(),
            owner: m.uid(),
            group: m.gid(),
            mode: m.mode(),
            links: m.nlink(),
            size: m.len(),
            modified: (m.mtime(), m.mtime_nsec()),
            changed: (m.ctime(), m.ctime_nsec()),
        }
    }
}
#[cfg(target_os = "macos")]
fn packaged_helper_candidate(executable_dir: &Path) -> Option<PathBuf> {
    let contents = executable_dir.parent()?;
    if !executable_dir.is_absolute()
        || executable_dir.file_name()? != "MacOS"
        || contents.file_name()? != "Contents"
    {
        return None;
    }
    Some(contents.join("Helpers/CortexaSpeechHelper"))
}
#[cfg(target_os = "macos")]
fn helper_path(_: &tauri::AppHandle) -> Result<(PathBuf, HelperIdentity), VoiceError> {
    let executable = std::env::current_exe().map_err(|_| VoiceError::Unavailable)?;
    let executable_dir = executable.parent().ok_or(VoiceError::Unavailable)?;
    let path = if let Some(packaged) = packaged_helper_candidate(executable_dir) {
        // A packaged app must never silently use a development helper when its
        // fixed bundled executable is missing, replaced or malformed.
        packaged
    } else if cfg!(debug_assertions) {
        PathBuf::from(env!("CORTEXA_SPEECH_HELPER_DEV"))
    } else {
        return Err(VoiceError::Unavailable);
    };
    let identity = verify_helper(&path)?;
    Ok((path, identity))
}
#[cfg(not(target_os = "macos"))]
fn helper_path(_: &tauri::AppHandle) -> Result<(PathBuf, HelperIdentity), VoiceError> {
    Err(VoiceError::Unsupported)
}

#[cfg(target_os = "macos")]
fn verify_helper(path: &Path) -> Result<HelperIdentity, VoiceError> {
    use sha2::{Digest, Sha256};
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    let parent = path.parent().ok_or(VoiceError::Unavailable)?;
    let mut current = Some(parent);
    while let Some(p) = current {
        if std::fs::symlink_metadata(p)
            .map_err(|_| VoiceError::Unavailable)?
            .file_type()
            .is_symlink()
        {
            return Err(VoiceError::Unavailable);
        }
        current = p.parent();
    }
    let before = std::fs::symlink_metadata(path).map_err(|_| VoiceError::Unavailable)?;
    let owner = std::fs::metadata(parent).map_err(|_| VoiceError::Unavailable)?;
    if !before.is_file()
        || before.nlink() != 1
        || before.uid() != owner.uid()
        || before.dev() != owner.dev()
        || before.mode() & 0o022 != 0
        || before.mode() & 0o111 == 0
        || before.len() > 16 * 1024 * 1024
    {
        return Err(VoiceError::Unavailable);
    }
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32)
        .open(path)
        .map_err(|_| VoiceError::Unavailable)?;
    let same = |m: &std::fs::Metadata| {
        before.dev() == m.dev()
            && before.ino() == m.ino()
            && before.uid() == m.uid()
            && before.gid() == m.gid()
            && before.mode() == m.mode()
            && before.nlink() == m.nlink()
            && before.len() == m.len()
            && before.mtime() == m.mtime()
            && before.mtime_nsec() == m.mtime_nsec()
            && before.ctime() == m.ctime()
            && before.ctime_nsec() == m.ctime_nsec()
    };
    if !same(&file.metadata().map_err(|_| VoiceError::Unavailable)?) {
        return Err(VoiceError::Unavailable);
    }
    let mut bytes = Vec::new();
    std::io::Read::by_ref(&mut file)
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| VoiceError::Unavailable)?;
    let expected = include_bytes!(concat!(env!("OUT_DIR"), "/CortexaSpeechHelper"));
    if Sha256::digest(&bytes) != Sha256::digest(expected)
        || !same(&file.metadata().map_err(|_| VoiceError::Unavailable)?)
        || !same(&std::fs::symlink_metadata(path).map_err(|_| VoiceError::Unavailable)?)
    {
        return Err(VoiceError::Unavailable);
    }
    Ok(HelperIdentity::read(&before))
}
#[cfg(not(target_os = "macos"))]
fn verify_helper(_: &Path) -> Result<HelperIdentity, VoiceError> {
    Err(VoiceError::Unsupported)
}

impl Running {
    fn snapshot(&self) -> Snapshot {
        let mut s = self.protocol.snapshot.clone();
        if s.busy {
            s.text = None;
        }
        s
    }
    fn stop(&mut self) {
        if self.stopping.is_some() {
            return;
        }
        self.protocol.discard();
        let id = self.protocol.snapshot.session_id.clone();
        // A child that has already exited may have a validated final frame queued.
        let _ = send_frame(&self.input, &id, "cancel", None);
        self.stopping = Some(Instant::now());
    }
    fn tick(&mut self) -> bool {
        for data in self.frames.try_iter() {
            if let Err(error) = self.protocol.accept(&data) {
                self.protocol.failed(error);
                self.failed_capture.store(true, Ordering::SeqCst);
            }
        }
        if self.failed_capture.load(Ordering::SeqCst) {
            self.protocol.failed(VoiceError::Protocol);
            self.stop();
        }
        let now = Instant::now();
        if now >= self.deadline && self.stopping.is_none() {
            self.protocol.failed(VoiceError::Timeout);
            self.stop();
        }
        if self
            .stopping
            .is_some_and(|s| now.duration_since(s) >= Duration::from_secs(5))
            && self.exited.is_none()
        {
            let _ = self.child.kill();
        }
        if self.exited.is_none() {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.exited = Some(status.success());
                    self.input = None;
                }
                Ok(None) => {}
                Err(_) => {
                    self.protocol.failed(VoiceError::CleanupFailed);
                    self.stop();
                }
            }
        }
        if let Some(ok) = self.exited {
            if self.closed.load(Ordering::SeqCst) == 3
                && self.readers.iter().all(JoinHandle::is_finished)
            {
                for data in self.frames.try_iter() {
                    if let Err(error) = self.protocol.accept(&data) {
                        self.protocol.failed(error);
                        self.failed_capture.store(true, Ordering::SeqCst);
                    }
                }
                for reader in self.readers.drain(..) {
                    if reader.join().is_err() {
                        self.failed_capture.store(true, Ordering::SeqCst);
                    }
                }
                self.protocol.settle_captured(
                    ok,
                    !self.failed_capture.load(Ordering::SeqCst),
                    self.helper_diagnostics.load(Ordering::SeqCst),
                );
                return true;
            }
        }
        false
    }
}
impl VoiceState {
    pub(crate) fn abort_for_focus(&self) {
        if let Ok(mut owner) = self.0.lock() {
            if !owner.blocked {
                owner.last = None;
            }
            if let Some(active) = owner.active.as_mut() {
                if active.protocol.operation() != Operation::Authorize {
                    active.stop();
                }
            }
        }
    }
    pub(crate) fn abort(&self) {
        if let Ok(mut owner) = self.0.lock() {
            if !owner.blocked {
                owner.last = None;
            }
            if let Some(active) = owner.active.as_mut() {
                active.stop();
            }
        }
    }
    pub(crate) fn shutdown(&self) -> bool {
        self.abort();
        let until = Instant::now() + Duration::from_secs(6);
        loop {
            let free = self.0.lock().is_ok_and(|owner| owner.active.is_none());
            if free {
                return true;
            }
            if Instant::now() >= until {
                return false;
            }
            thread::sleep(Duration::from_millis(25));
        }
    }
    fn start(
        &self,
        app: &tauri::AppHandle,
        chat: AgentChatState,
        request: StartRequest,
    ) -> Result<Snapshot, VoiceError> {
        crate::agent_chat_tauri::validate_voice_scope(&chat, &request.conversation_id)?;
        let (path, identity) = helper_path(app)?;
        let mut owner = self.0.lock().map_err(|_| VoiceError::CleanupFailed)?;
        owner.retire_previous_review(&request.conversation_id);
        owner.can_start()?;
        owner.next = owner.next.checked_add(1).ok_or(VoiceError::Limit)?;
        let id = format!("{:016x}-{:016x}", std::process::id(), owner.next);
        let text = crate::agent_chat_tauri::claim_voice(
            &chat,
            &request.conversation_id,
            &id,
            request.operation == Operation::Speak,
        )?;
        let result = (|| {
            let mut child = Command::new(&path)
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .env("LANG", "en_US.UTF-8")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|_| VoiceError::Unavailable)?;
            let admission_ok = verify_helper(&path).is_ok_and(|after| after == identity);
            let input = child.stdin.take();
            let output = child.stdout.take();
            let error = child.stderr.take();
            let (tx, rx) = mpsc::sync_channel(8);
            let (commands, command_rx) = mpsc::sync_channel::<Vec<u8>>(2);
            let failed = Arc::new(AtomicBool::new(false));
            let diagnostics = Arc::new(AtomicBool::new(false));
            let closed = Arc::new(AtomicUsize::new(0));
            let f = failed.clone();
            let c = closed.clone();
            let stdout = thread::spawn(move || {
                if let Some(output) = output {
                    capture_frames(output, &tx, &f);
                } else {
                    f.store(true, Ordering::SeqCst);
                }
                c.fetch_add(1, Ordering::SeqCst);
            });
            let f = failed.clone();
            let c = closed.clone();
            let seen = diagnostics.clone();
            let stderr = thread::spawn(move || {
                if let Some(error) = error {
                    capture_diagnostics(error, &f, &seen);
                } else {
                    f.store(true, Ordering::SeqCst);
                }
                c.fetch_add(1, Ordering::SeqCst);
            });
            let f = failed.clone();
            let c = closed.clone();
            let stdin = thread::spawn(move || {
                if let Some(mut input) = input {
                    while let Ok(data) = command_rx.recv() {
                        if input.write_all(&data).is_err() {
                            f.store(true, Ordering::SeqCst);
                            break;
                        }
                    }
                } else {
                    f.store(true, Ordering::SeqCst);
                }
                c.fetch_add(1, Ordering::SeqCst);
            });
            let operation = match request.operation {
                Operation::Authorize => "authorize",
                Operation::Record => "record",
                Operation::Speak => "speak",
            };
            let mut running = Running {
                child,
                input: Some(commands),
                readers: vec![stdout, stderr, stdin],
                frames: rx,
                failed_capture: failed,
                helper_diagnostics: diagnostics,
                closed,
                protocol: Protocol::new(request.conversation_id, id.clone(), request.operation),
                deadline: Instant::now()
                    + Duration::from_secs(match request.operation {
                        Operation::Authorize => 120,
                        Operation::Record => 45,
                        Operation::Speak => 65,
                    }),
                stopping: None,
                exited: None,
            };
            if !admission_ok || send_frame(&running.input, &id, operation, text.as_deref()).is_err()
            {
                running.protocol.failed(VoiceError::Protocol);
                running.stop();
            }
            Ok(running)
        })();
        let running = match result {
            Ok(r) => r,
            Err(e) => {
                crate::agent_chat_tauri::release_voice(&chat, &id);
                return Err(e);
            }
        };
        let snapshot = running.snapshot();
        owner.active = Some(running);
        drop(owner);
        let shared = self.0.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(25));
            let Ok(mut owner) = shared.lock() else {
                return;
            };
            let Some(active) = owner.active.as_mut() else {
                return;
            };
            if active.tick() {
                let snapshot = active.snapshot();
                let operation = active.protocol.operation();
                let id = snapshot.session_id.clone();
                if owner.finish_owned(snapshot, operation) {
                    crate::agent_chat_tauri::release_voice(&chat, &id);
                }
                return;
            }
        });
        Ok(snapshot)
    }
    fn poll(&self, request: &SessionRequest) -> Result<Snapshot, VoiceError> {
        let owner = self.0.lock().map_err(|_| VoiceError::CleanupFailed)?;
        let snapshot = owner
            .active
            .as_ref()
            .map(Running::snapshot)
            .or_else(|| owner.last.clone())
            .ok_or(VoiceError::StaleSession)?;
        if snapshot.session_id != request.session_id
            || snapshot.conversation_id != request.conversation_id
        {
            return Err(VoiceError::StaleSession);
        }
        Ok(snapshot)
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct StartRequest {
    conversation_id: String,
    operation: Operation,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SessionRequest {
    conversation_id: String,
    session_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ActionRequest {
    conversation_id: String,
    session_id: String,
    action: Action,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Action {
    Finish,
    Cancel,
}
fn window_guard(window: &tauri::Window, focused: bool) -> Result<(), VoiceError> {
    if window.label() != "main"
        || (focused && !window.is_focused().map_err(|_| VoiceError::Unavailable)?)
    {
        return Err(VoiceError::Unavailable);
    }
    Ok(())
}
#[tauri::command]
pub(crate) fn start_voice(
    window: tauri::Window,
    app: tauri::AppHandle,
    state: tauri::State<'_, VoiceState>,
    chat: tauri::State<'_, AgentChatState>,
    request: StartRequest,
) -> Result<Snapshot, VoiceError> {
    window_guard(&window, true)?;
    state.start(&app, chat.inner().clone(), request)
}
#[tauri::command]
pub(crate) fn poll_voice(
    window: tauri::Window,
    state: tauri::State<'_, VoiceState>,
    chat: tauri::State<'_, AgentChatState>,
    request: SessionRequest,
) -> Result<Snapshot, VoiceError> {
    window_guard(&window, false)?;
    crate::agent_chat_tauri::validate_voice_scope(&chat, &request.conversation_id)?;
    state.poll(&request)
}
#[tauri::command]
pub(crate) fn control_voice(
    window: tauri::Window,
    state: tauri::State<'_, VoiceState>,
    chat: tauri::State<'_, AgentChatState>,
    request: ActionRequest,
) -> Result<Snapshot, VoiceError> {
    window_guard(&window, false)?;
    let current_scope =
        crate::agent_chat_tauri::validate_voice_scope(&chat, &request.conversation_id);
    let session = SessionRequest {
        conversation_id: request.conversation_id,
        session_id: request.session_id,
    };
    if current_scope.is_err() {
        let mut owner = state.0.lock().map_err(|_| VoiceError::CleanupFailed)?;
        // An old view may erase only its exact inactive review after navigation.
        // It cannot stop, read or release a new conversation's active operation.
        if owner.active.is_none() && matches!(request.action, Action::Cancel) {
            return owner.discard_review(&session);
        }
        return Err(VoiceError::StaleSession);
    }
    state.poll(&session)?;
    let mut owner = state.0.lock().map_err(|_| VoiceError::CleanupFailed)?;
    if let Some(active) = owner.active.as_mut() {
        if active.protocol.snapshot.session_id != session.session_id {
            return Err(VoiceError::StaleSession);
        }
        match request.action {
            Action::Cancel => active.stop(),
            Action::Finish => {
                active.protocol.finish()?;
                send_frame(&active.input, &session.session_id, "finish", None)?;
                active.deadline = Instant::now() + Duration::from_secs(10);
            }
        }
    } else {
        return match request.action {
            Action::Cancel => owner.discard_review(&session),
            Action::Finish => Err(VoiceError::StaleSession),
        };
    }
    drop(owner);
    state.poll(&session)
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "macos")]
    #[test]
    fn fixed_packaged_helper_location_and_config_agree() -> Result<(), String> {
        let directory = Path::new("/QA/Cortexa.app/Contents/MacOS");
        let expected = PathBuf::from("/QA/Cortexa.app/Contents/Helpers/CortexaSpeechHelper");
        assert_eq!(packaged_helper_candidate(directory), Some(expected));
        // Candidate selection does not depend on presence and cannot fall back
        // merely because a bundled helper is absent.
        for bad in [
            "/QA/target/debug",
            "/QA/Cortexa.app/Contents/Resources",
            "Contents/MacOS",
            "/MacOS",
        ] {
            assert!(packaged_helper_candidate(Path::new(bad)).is_none());
        }
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../tauri.macos.conf.json"))
                .map_err(|_| "macOS config")?;
        assert_eq!(
            config["bundle"]["macOS"]["files"]["Helpers/CortexaSpeechHelper"],
            "voice-bin/CortexaSpeechHelper"
        );
        assert!(config["bundle"]["resources"]
            .get("voice-bin/CortexaSpeechHelper")
            .is_none());
        Ok(())
    }

    #[test]
    fn unconfirmed_cleanup_retains_exact_owner_and_primary_error() -> Result<(), String> {
        for (operation, error, retain) in [
            (Operation::Speak, "cleanup_failed", true),
            (Operation::Speak, "protocol", true),
            (Operation::Speak, "timeout", true),
            (Operation::Speak, "unavailable", false),
            (Operation::Record, "cleanup_failed", true),
            (Operation::Authorize, "permission_denied", false),
        ] {
            let mut p = Protocol::new("conversation".into(), "owner".into(), operation);
            let frame = serde_json::json!({"version":1,"session":"owner","sequence":1,
                "event":"error","text":null,"error":error})
            .to_string();
            p.accept(frame.as_bytes()).map_err(|e| format!("{e:?}"))?;
            p.settle(false, true);
            let expected_error = p.snapshot.error;
            let mut owner = Owner::default();
            assert_eq!(owner.finish_owned(p.snapshot.clone(), operation), !retain);
            assert_eq!(owner.blocked, retain);
            assert_eq!(
                owner.last.as_ref().ok_or("missing snapshot")?.error,
                expected_error
            );
            if retain {
                assert_eq!(owner.can_start(), Err(VoiceError::CleanupFailed));
                owner.retire_previous_review("different");
                assert!(owner.last.is_some());
                assert!(owner
                    .discard_review(&SessionRequest {
                        conversation_id: "wrong".into(),
                        session_id: "owner".into()
                    })
                    .is_err());
                let same = owner
                    .discard_review(&SessionRequest {
                        conversation_id: "conversation".into(),
                        session_id: "owner".into(),
                    })
                    .map_err(|e| format!("{e:?}"))?;
                assert!(same.busy);
                assert_eq!(same.error, expected_error);
                assert_eq!(owner.can_start(), Err(VoiceError::CleanupFailed));
            } else {
                assert!(owner.can_start().is_ok());
            }
        }
        Ok(())
    }

    use super::*;
    #[test]
    fn bounded_capture_requires_newline_and_never_accepts_truncated_output() -> Result<(), String> {
        for bytes in [vec![b'x'; MAX_FRAME + 2], b"no-newline".to_vec()] {
            let (tx, rx) = mpsc::sync_channel(8);
            let failed = AtomicBool::new(false);
            capture_frames(std::io::Cursor::new(bytes), &tx, &failed);
            assert!(failed.load(Ordering::SeqCst));
            assert!(rx.try_recv().is_err());
        }
        Ok(())
    }
    #[test]
    fn diagnostics_reader_retains_only_negative_witness_and_keeps_errors_strict() {
        let failed = AtomicBool::new(false);
        let seen = AtomicBool::new(false);
        capture_diagnostics(std::io::Cursor::new(Vec::<u8>::new()), &failed, &seen);
        assert!(!failed.load(Ordering::SeqCst) && !seen.load(Ordering::SeqCst));
        capture_diagnostics(
            std::io::Cursor::new(b"unclassified private diagnostic"),
            &failed,
            &seen,
        );
        assert!(failed.load(Ordering::SeqCst) && seen.load(Ordering::SeqCst));
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("not retained"))
            }
        }
        let failed = AtomicBool::new(false);
        let seen = AtomicBool::new(false);
        capture_diagnostics(Broken, &failed, &seen);
        assert!(failed.load(Ordering::SeqCst) && !seen.load(Ordering::SeqCst));
    }

    #[test]
    fn capture_queue_overflow_and_reader_failure_reject() -> Result<(), String> {
        let (tx, rx) = mpsc::sync_channel(1);
        let failed = AtomicBool::new(false);
        capture_frames(std::io::Cursor::new(b"one\ntwo\n"), &tx, &failed);
        assert!(failed.load(Ordering::SeqCst));
        assert_eq!(rx.try_recv().map_err(|e| format!("{e:?}"))?, b"one");
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::ErrorKind::PermissionDenied.into())
            }
        }
        let (tx, _rx) = mpsc::sync_channel(1);
        let failed = AtomicBool::new(false);
        capture_frames(Broken, &tx, &failed);
        assert!(failed.load(Ordering::SeqCst));
        Ok(())
    }
    #[cfg(target_os = "macos")]
    #[test]
    fn helper_content_modes_links_and_forward_identity_are_checked() -> Result<(), String> {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::tempdir().map_err(|e| format!("{e:?}"))?;
        let path = temp.path().join("CortexaSpeechHelper");
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/CortexaSpeechHelper"));
        std::fs::write(&path, bytes).map_err(|e| format!("{e:?}"))?;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("{e:?}"))?;
        let before = verify_helper(&path).map_err(|e| format!("{e:?}"))?;
        let replacement = temp.path().join("replacement");
        std::fs::write(&replacement, bytes).map_err(|e| format!("{e:?}"))?;
        std::fs::set_permissions(&replacement, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("{e:?}"))?;
        std::fs::rename(&replacement, &path).map_err(|e| format!("{e:?}"))?;
        assert!(verify_helper(&path).is_ok_and(|current| current != before));
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o775))
            .map_err(|e| format!("{e:?}"))?;
        assert!(verify_helper(&path).is_err());
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("{e:?}"))?;
        let link = temp.path().join("alias");
        std::fs::hard_link(&path, &link).map_err(|e| format!("{e:?}"))?;
        assert!(verify_helper(&path).is_err());
        std::fs::remove_file(&link).map_err(|e| format!("{e:?}"))?;
        std::fs::write(&path, b"substituted").map_err(|e| format!("{e:?}"))?;
        assert!(verify_helper(&path).is_err());
        std::fs::remove_file(&path).map_err(|e| format!("{e:?}"))?;
        std::os::unix::fs::symlink(&replacement, &path).map_err(|e| format!("{e:?}"))?;
        assert!(verify_helper(&path).is_err());
        Ok(())
    }
    #[test]
    fn explicit_review_discard_erases_only_matching_ephemeral_text() -> Result<(), String> {
        let mut owner = Owner::default();
        let mut p = Protocol::new("c".into(), "a".into(), Operation::Record);
        p.accept(br#"{"version":1,"session":"a","sequence":1,"event":"recording","text":null,"error":null}"#).map_err(|e| format!("{e:?}"))?;
        p.accept(br#"{"version":1,"session":"a","sequence":2,"event":"transcript","text":"Synthetic","error":null}"#).map_err(|e| format!("{e:?}"))?;
        p.settle(true, true);
        owner.last = Some(p.snapshot);
        assert!(owner
            .discard_review(&SessionRequest {
                conversation_id: "c".into(),
                session_id: "old".into()
            })
            .is_err());
        let discarded = owner
            .discard_review(&SessionRequest {
                conversation_id: "c".into(),
                session_id: "a".into(),
            })
            .map_err(|e| format!("{e:?}"))?;
        assert_eq!(discarded.status, Status::Cancelled);
        assert!(discarded.text.is_none());
        assert_eq!(discarded.sequence, 3);
        owner.retire_previous_review("new-conversation");
        assert!(owner.last.is_none());
        assert!(owner
            .discard_review(&SessionRequest {
                conversation_id: "c".into(),
                session_id: "a".into()
            })
            .is_err());
        Ok(())
    }
    #[test]
    fn ipc_unknown_fields_and_unknown_operations_reject() -> Result<(), String> {
        assert!(serde_json::from_str::<StartRequest>(
            r#"{"conversationId":"x","operation":"record","path":"x"}"#
        )
        .is_err());
        assert!(serde_json::from_str::<StartRequest>(
            r#"{"conversationId":"x","operation":"download"}"#
        )
        .is_err());
        assert!(serde_json::from_str::<ActionRequest>(
            r#"{"conversationId":"x","sessionId":"x","action":"restart"}"#
        )
        .is_err());
        Ok(())
    }
}
