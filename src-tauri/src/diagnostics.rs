//! Content-free local diagnostics. Not a security audit or a transcript store.
//! Producers cannot supply a message, path, prompt, response or raw error string.
use crate::{
    agent_preferences::AgentConnection,
    personal_assistant_direct::{
        DirectError, ErrorEnvelope, TopLevelErrorCode, TopLevelErrorParam,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::VecDeque,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, SyncSender},
        Arc, Mutex, OnceLock,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const CAPACITY: usize = 256;
const FILE_LIMIT: u64 = 5 * 1024 * 1024;
const MAX_RECORD: usize = 2048;
static SERVICE: OnceLock<Logger> = OnceLock::new();
static NEXT: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Severity {
    Info,
    Warning,
    Error,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Event {
    ApplicationStarted,
    ApplicationShutdown,
    RequestStarted,
    ConfigurationValidated,
    CredentialsAvailable,
    ProviderDispatch,
    TransportReleased,
    UiTerminalReturned,
    OwnershipReleased,
    RequestInterrupted,
    FirstResponse,
    FirstText,
    RequestFinished,
    RuntimeStarted,
    RuntimeExited,
    RuntimeCleanup,
    StageStarted,
    StageFinished,
    StorageFailure,
    ConfigurationFailure,
    OpenaiTopLevelError,
    OpenaiTopLevelErrorParam,
    OpenaiErrorEnvelope,
    OpenaiNestedErrorCode,
    OpenaiNestedErrorParam,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Outcome {
    Observed,
    Completed,
    Failed,
    TimedOut,
    Cancelled,
    Partial,
    Interrupted,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Category {
    Authentication,
    MissingCredentials,
    ModelUnavailable,
    RuntimeLaunch,
    Cleanup,
    Interrupted,
    DeniedAccess,
    RateLimit,
    Timeout,
    Network,
    MalformedStream,
    StreamError,
    OpenaiTopLevelServerError,
    OpenaiTopLevelRateLimitExceeded,
    OpenaiTopLevelInvalidPrompt,
    OpenaiInvalidApiKey,
    OpenaiInsufficientQuota,
    OpenaiCreditBalanceExhausted,
    OpenaiModelNotFound,
    OpenaiUnsupportedValue,
    OpenaiInvalidValue,
    OpenaiEnvelopeAbsent,
    OpenaiEnvelopeNull,
    OpenaiEnvelopeInvalid,
    OpenaiEnvelopeObject,
    OpenaiTopLevelUnknownCode,
    // Historical combined absent/null category remains readable.
    OpenaiTopLevelMissingCode,
    OpenaiTopLevelAbsentCode,
    OpenaiTopLevelNullCode,
    OpenaiTopLevelParamModel,
    OpenaiTopLevelParamReasoning,
    OpenaiTopLevelParamReasoningEffort,
    OpenaiTopLevelParamMaxOutputTokens,
    OpenaiTopLevelParamServiceTier,
    OpenaiTopLevelParamAbsent,
    OpenaiTopLevelParamNull,
    OpenaiTopLevelParamInvalid,
    OpenaiTopLevelParamUnknown,

    OpenaiTopLevelInvalidCode,
    Incomplete,
    ResourceLimit,
    Runtime,
    Configuration,
    Storage,
    Internal,
}
impl From<DirectError> for Category {
    fn from(e: DirectError) -> Self {
        match e {
            DirectError::Authentication | DirectError::CodexAuthentication => Self::Authentication,
            DirectError::MissingKey => Self::MissingCredentials,
            DirectError::ModelUnavailable => Self::ModelUnavailable,
            DirectError::CodexSetup => Self::RuntimeLaunch,
            DirectError::HttpForbidden => Self::DeniedAccess,
            DirectError::RateLimited | DirectError::ProviderStreamRateLimit => Self::RateLimit,
            DirectError::Timeout => Self::Timeout,
            DirectError::Network | DirectError::ProviderUnavailable => Self::Network,
            DirectError::Protocol => Self::MalformedStream,
            DirectError::Limit => Self::ResourceLimit,
            DirectError::ProviderStreamCreditBalanceExhausted => Self::OpenaiCreditBalanceExhausted,
            DirectError::ProviderStreamErrorEvent
            | DirectError::ProviderStreamFailedUnknownCode
            | DirectError::ProviderStreamFailedInvalidCode
            | DirectError::ProviderStreamServerError
            | DirectError::ProviderStreamInvalidPrompt => Self::StreamError,
            DirectError::Incomplete | DirectError::Truncated | DirectError::Refused => {
                Self::Incomplete
            }
            DirectError::CodexRuntime | DirectError::CodexIsolation => Self::Runtime,
            DirectError::Internal => Self::Internal,
            _ => Self::Configuration,
        }
    }
}

// Opaque hashes prevent discovered model IDs or caller-derived identities from
// becoming a content channel. Public fixed OpenAI model names remain recognizable.
fn fingerprint(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}
fn identifier() -> String {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    fingerprint(&format!(
        "{time}:{}:{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}
fn valid_id(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..].bytes().all(|b| b.is_ascii_hexdigit())
}
fn model_id(value: &str) -> String {
    match value {
        "simulation" | "gpt-5.6-luna" | "gpt-5.6-terra" | "gpt-5.6-sol" => value.to_owned(),
        _ => fingerprint(value),
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Record {
    pub(crate) utc_ms: u64,
    pub(crate) severity: Severity,
    pub(crate) app_version: String,
    pub(crate) event: Event,
    pub(crate) outcome: Outcome,
    pub(crate) error: Option<Category>,
    pub(crate) session: String,
    pub(crate) conversation: Option<String>,
    pub(crate) attempt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) request: Option<String>,
    pub(crate) workflow: Option<String>,
    pub(crate) stage: Option<String>,
    pub(crate) provider: Option<AgentConnection>,
    pub(crate) model: Option<String>,
    pub(crate) duration_ms: Option<u64>,
}
impl Record {
    fn valid(&self) -> bool {
        self.app_version == env!("CARGO_PKG_VERSION")
            && valid_id(&self.session)
            && [
                &self.request,
                &self.conversation,
                &self.attempt,
                &self.workflow,
                &self.stage,
            ]
            .into_iter()
            .all(|v| v.as_ref().is_none_or(|s| valid_id(s)))
            && self.model.as_ref().is_none_or(|m| {
                valid_id(m)
                    || matches!(
                        m.as_str(),
                        "simulation" | "gpt-5.6-luna" | "gpt-5.6-terra" | "gpt-5.6-sol"
                    )
            })
    }
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Snapshot {
    pub(crate) available: bool,
    pub(crate) events: Vec<Record>,
}
#[derive(Clone)]
pub(crate) struct Logger {
    shared: Arc<Shared>,
    tx: SyncSender<Message>,
    session: String,
}
struct Shared {
    available: AtomicBool,
    recent: Mutex<VecDeque<Record>>,
}
enum Message {
    Record(Box<Record>),
    Flush(mpsc::Sender<()>),
    Shutdown(mpsc::Sender<()>),
}

fn open_file(path: &Path, append: bool) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    if append {
        options.create(true).append(true);
    } else {
        options.create_new(true);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .mode(0o600)
            .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
    }
    let file = options.open(path)?;
    if !file.metadata()?.is_file() {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        if file.metadata()?.nlink() != 1 {
            return Err(io::ErrorKind::InvalidInput.into());
        }
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
    }
    Ok(file)
}
fn read_records(path: &Path) -> io::Result<Vec<Record>> {
    if !path.try_exists()? {
        return Ok(Vec::new());
    }
    let f = open_file(path, true)?;
    if f.metadata()?.len() > FILE_LIMIT {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let mut reader = BufReader::new(f);
    let mut recent = VecDeque::new();
    loop {
        let mut line = Vec::new();
        let n = std::io::Read::take(&mut reader, (MAX_RECORD + 1) as u64)
            .read_until(b'\n', &mut line)?;
        if n == 0 {
            break;
        }
        if n > MAX_RECORD || line.last() != Some(&b'\n') {
            return Err(io::ErrorKind::InvalidData.into());
        }
        let record: Record =
            serde_json::from_slice(&line).map_err(|_| io::ErrorKind::InvalidData)?;
        if !record.valid() {
            return Err(io::ErrorKind::InvalidData.into());
        }
        if recent.len() == CAPACITY {
            recent.pop_front();
        }
        recent.push_back(record);
    }
    Ok(recent.into())
}
impl Logger {
    pub(crate) fn open(directory: PathBuf) -> Self {
        Self::with_limit(directory, FILE_LIMIT)
    }
    fn with_limit(directory: PathBuf, limit: u64) -> Self {
        let shared = Arc::new(Shared {
            available: AtomicBool::new(true),
            recent: Mutex::new(VecDeque::new()),
        });
        let (tx, rx) = mpsc::sync_channel(CAPACITY);
        let logger = Self {
            shared: Arc::clone(&shared),
            tx,
            session: identifier(),
        };
        let initialized = (|| -> io::Result<()> {
            if directory.exists() && fs::symlink_metadata(&directory)?.file_type().is_symlink() {
                return Err(io::ErrorKind::InvalidInput.into());
            }
            fs::create_dir_all(&directory)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))?;
            }
            for i in (0..3).rev() {
                let records = read_records(&directory.join(format!("diagnostics-{i}.jsonl")))?;
                let mut recent = shared.recent.lock().map_err(|_| io::ErrorKind::Other)?;
                for r in records {
                    if recent.len() == CAPACITY {
                        recent.pop_front();
                    }
                    recent.push_back(r);
                }
            }
            Ok(())
        })();
        if initialized.is_err() {
            shared.available.store(false, Ordering::Relaxed);
            return logger;
        }
        if std::thread::Builder::new()
            .name("cortexa-diagnostics".into())
            .spawn(move || {
                while let Ok(message) = rx.recv() {
                    match message {
                        Message::Record(record) => {
                            if write_record(&directory, &record, limit).is_err() {
                                shared.available.store(false, Ordering::Relaxed);
                            }
                        }
                        Message::Flush(done) => {
                            let _ = done.send(());
                        }
                        Message::Shutdown(done) => {
                            let _ = done.send(());
                            break;
                        }
                    }
                }
            })
            .is_err()
        {
            logger.shared.available.store(false, Ordering::Relaxed);
        }
        // A retained, nonterminal request from a prior session has no observed outcome.
        // Persist one explicit interruption marker; never invent cleanup or duration.
        let prior = logger.snapshot().events;
        let mut pending = std::collections::BTreeMap::new();
        for r in prior {
            if let Some(id) = r.request.clone().or_else(|| r.attempt.clone()) {
                if matches!(r.event, Event::RequestFinished | Event::RequestInterrupted) {
                    pending.remove(&id);
                } else if !matches!(
                    r.event,
                    Event::StageFinished | Event::UiTerminalReturned | Event::OwnershipReleased
                ) {
                    pending.insert(id, r);
                }
            }
        }
        for mut r in pending.into_values() {
            let marker = logger.blank(
                Event::RequestInterrupted,
                Outcome::Interrupted,
                Some(Category::Interrupted),
            );
            r.utc_ms = marker.utc_ms;
            r.event = marker.event;
            r.outcome = marker.outcome;
            r.error = marker.error;
            r.severity = Severity::Warning;
            r.duration_ms = None;
            logger.record_event(r);
        }
        logger
    }
    fn blank(&self, event: Event, outcome: Outcome, error: Option<Category>) -> Record {
        Record {
            utc_ms: u64::try_from(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis(),
            )
            .unwrap_or(u64::MAX),
            severity: if error.is_some() {
                Severity::Error
            } else if outcome == Outcome::Cancelled {
                Severity::Warning
            } else {
                Severity::Info
            },
            app_version: env!("CARGO_PKG_VERSION").into(),
            event,
            outcome,
            error,
            session: self.session.clone(),
            conversation: None,
            attempt: None,
            request: None,
            workflow: None,
            stage: None,
            provider: None,
            model: None,
            duration_ms: None,
        }
    }
    fn record_event(&self, record: Record) {
        if record.utc_ms == 0 {
            self.shared.available.store(false, Ordering::Relaxed);
            return;
        }
        if let Ok(mut recent) = self.shared.recent.lock() {
            if recent.len() == CAPACITY {
                recent.pop_front();
            }
            recent.push_back(record.clone());
        } else {
            self.shared.available.store(false, Ordering::Relaxed);
        }
        if self.tx.try_send(Message::Record(Box::new(record))).is_err() {
            self.shared.available.store(false, Ordering::Relaxed);
        }
    }
    pub(crate) fn event(&self, event: Event, outcome: Outcome, error: Option<Category>) {
        self.record_event(self.blank(event, outcome, error));
    }
    pub(crate) fn snapshot(&self) -> Snapshot {
        match self.shared.recent.lock() {
            Ok(events) => Snapshot {
                available: self.shared.available.load(Ordering::Relaxed),
                events: events.iter().cloned().collect(),
            },
            Err(_) => Snapshot {
                available: false,
                events: Vec::new(),
            },
        }
    }
    pub(crate) fn flush(&self, shutdown: bool) -> bool {
        let (tx, rx) = mpsc::channel();
        let message = if shutdown {
            Message::Shutdown(tx)
        } else {
            Message::Flush(tx)
        };
        let ok =
            self.tx.try_send(message).is_ok() && rx.recv_timeout(Duration::from_secs(2)).is_ok();
        if !ok {
            self.shared.available.store(false, Ordering::Relaxed);
        }
        ok
    }
}
fn write_record(directory: &Path, record: &Record, limit: u64) -> io::Result<()> {
    let mut bytes = serde_json::to_vec(record).map_err(|_| io::ErrorKind::InvalidData)?;
    bytes.push(b'\n');
    if bytes.len() > MAX_RECORD || bytes.len() as u64 > limit {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let current = directory.join("diagnostics-0.jsonl");
    let mut file = open_file(&current, true)?;
    if file.metadata()?.len() + bytes.len() as u64 > limit {
        drop(file);
        for i in (1..=2).rev() {
            let old = directory.join(format!("diagnostics-{}.jsonl", i - 1));
            let next = directory.join(format!("diagnostics-{i}.jsonl"));
            if old.exists() {
                let _validated = open_file(&old, true)?;
                fs::rename(old, next)?;
            }
        }
        file = open_file(&current, false)?;
    }
    file.write_all(&bytes)?;
    file.flush()
}

pub(crate) fn install(directory: PathBuf) {
    let _ = SERVICE.set(Logger::open(directory));
}
pub(crate) fn logger() -> Option<Logger> {
    SERVICE.get().cloned()
}
pub(crate) fn event(event: Event, outcome: Outcome, error: Option<Category>) {
    if let Some(log) = logger() {
        log.event(event, outcome, error);
    }
}

// The guard exists outside the awaited transport: dropping an aborted future
// seals its attempt once. Observer clones cannot publish a second terminal event.
#[derive(Clone)]
pub(crate) struct Observer(Arc<Mutex<AttemptState>>);
struct AttemptState {
    logger: Option<Logger>,
    record: Record,
    start: Instant,
    response: bool,
    text: bool,
    top_level_error: bool,
    error_envelope: bool,
    terminal: bool,
    abort_error: Option<DirectError>,
    phases: Vec<Event>,
    ui_returned: bool,
}
pub(crate) struct Attempt {
    observer: Observer,
}
impl Attempt {
    pub(crate) fn new(
        provider: AgentConnection,
        model: &str,
        conversation: Option<&str>,
        workflow: Option<&str>,
        stage: Option<&str>,
    ) -> Self {
        Self::with_logger(logger(), provider, model, conversation, workflow, stage)
    }
    pub(crate) fn with_logger(
        log: Option<Logger>,
        provider: AgentConnection,
        model: &str,
        conversation: Option<&str>,
        workflow: Option<&str>,
        stage: Option<&str>,
    ) -> Self {
        let session = log
            .as_ref()
            .map(|l| l.session.clone())
            .unwrap_or_else(identifier);
        let record = Record {
            utc_ms: 0,
            severity: Severity::Info,
            app_version: env!("CARGO_PKG_VERSION").into(),
            event: Event::RequestStarted,
            outcome: Outcome::Observed,
            error: None,
            session: session.clone(),
            conversation: conversation.map(|s| fingerprint(&format!("{session}:{s}"))),
            attempt: None,
            request: Some(identifier()),
            workflow: workflow.map(fingerprint),
            stage: stage.map(fingerprint),
            provider: Some(provider),
            model: Some(model_id(model)),
            duration_ms: None,
        };
        let observer = Observer(Arc::new(Mutex::new(AttemptState {
            logger: log,
            record,
            start: Instant::now(),
            response: false,
            text: false,
            top_level_error: false,
            error_envelope: false,
            terminal: false,
            abort_error: None,
            phases: Vec::new(),
            ui_returned: false,
        })));
        observer.record(Event::RequestStarted, Outcome::Observed, None);
        if stage.is_some() {
            observer.record(Event::StageStarted, Outcome::Observed, None);
        }
        Self { observer }
    }
    pub(crate) fn observer(&self) -> Observer {
        self.observer.clone()
    }
    pub(crate) fn fail_preparation(&mut self, error: crate::agent_chat::ChatError) {
        let category = match error {
            crate::agent_chat::ChatError::Provider(e) => e.into(),
            crate::agent_chat::ChatError::Internal => Category::Internal,
            _ => Category::Configuration,
        };
        self.observer.finish(Outcome::Failed, Some(category));
    }
    pub(crate) fn partial(&mut self) {
        self.observer.finish(Outcome::Partial, None);
    }
    pub(crate) fn finish(&mut self, result: Result<(), DirectError>) {
        let (outcome, error) = match result {
            Ok(()) => (Outcome::Completed, None),
            Err(e) => (
                if e == DirectError::Timeout {
                    Outcome::TimedOut
                } else {
                    Outcome::Failed
                },
                Some(e.into()),
            ),
        };
        self.observer.finish(outcome, error);
    }
}
impl Drop for Attempt {
    fn drop(&mut self) {
        let error = self.observer.0.lock().ok().and_then(|s| s.abort_error);
        match error {
            Some(error) => self.finish(Err(error)),
            None => self.observer.finish(Outcome::Cancelled, None),
        }
    }
}
impl AttemptState {
    fn record_event(&self, event: Event, outcome: Outcome, error: Option<Category>) {
        if let Some(log) = &self.logger {
            let mut record = log.blank(event, outcome, error);
            record.conversation = self.record.conversation.clone();
            record.attempt = self.record.attempt.clone();
            record.request = self.record.request.clone();
            record.workflow = self.record.workflow.clone();
            record.stage = self.record.stage.clone();
            record.provider = self.record.provider;
            record.model = self.record.model.clone();
            record.duration_ms =
                Some(u64::try_from(self.start.elapsed().as_millis()).unwrap_or(u64::MAX));
            log.record_event(record);
        }
    }
}
impl Observer {
    pub(crate) fn request_id(&self) -> Option<String> {
        self.0.lock().ok().and_then(|s| s.record.request.clone())
    }
    pub(crate) fn dispatch(&self) {
        if let Ok(mut s) = self.0.lock() {
            if s.terminal || s.record.attempt.is_some() {
                return;
            }
            s.record.attempt = Some(identifier());
            s.record_event(Event::ProviderDispatch, Outcome::Observed, None);
        }
    }
    pub(crate) fn ownership_released(&self) {
        if let Ok(mut s) = self.0.lock() {
            if s.terminal && !s.phases.contains(&Event::OwnershipReleased) {
                s.phases.push(Event::OwnershipReleased);
                s.record_event(Event::OwnershipReleased, Outcome::Observed, None);
            }
        }
    }
    // The existing poll boundary returns a matching terminal snapshot. This is
    // not proof of frontend receipt/rendering or authority to execute.
    pub(crate) fn ui_returned(&self, request: &str) {
        if let Ok(mut s) = self.0.lock() {
            if s.terminal && !s.ui_returned && s.record.request.as_deref() == Some(request) {
                s.ui_returned = true;
                s.record_event(Event::UiTerminalReturned, Outcome::Observed, None);
            }
        }
    }

    // A diagnostic observation does not change the attempt's terminal outcome.
    // The decoder chooses a fixed terminal error; transport cleanup ownership is unchanged.
    pub(crate) fn top_level_error(&self, code: TopLevelErrorCode, param: TopLevelErrorParam) {
        if let Ok(mut s) = self.0.lock() {
            if s.terminal
                || s.top_level_error
                || s.record.provider != Some(AgentConnection::OpenaiApi)
            {
                return;
            }
            s.top_level_error = true;
            let category = error_code_category(code);
            s.record_event(
                Event::OpenaiTopLevelError,
                Outcome::Observed,
                Some(category),
            );
            let parameter = error_param_category(param);
            s.record_event(
                Event::OpenaiTopLevelErrorParam,
                Outcome::Observed,
                Some(parameter),
            );
        }
    }

    pub(crate) fn error_envelope(&self, envelope: ErrorEnvelope) {
        if let Ok(mut s) = self.0.lock() {
            if s.terminal
                || s.error_envelope
                || !s.top_level_error
                || s.record.provider != Some(AgentConnection::OpenaiApi)
            {
                return;
            }
            s.error_envelope = true;
            let shape = match envelope {
                ErrorEnvelope::Absent => Category::OpenaiEnvelopeAbsent,
                ErrorEnvelope::Null => Category::OpenaiEnvelopeNull,
                ErrorEnvelope::Invalid => Category::OpenaiEnvelopeInvalid,
                ErrorEnvelope::Object(_, _) => Category::OpenaiEnvelopeObject,
            };
            s.record_event(Event::OpenaiErrorEnvelope, Outcome::Observed, Some(shape));
            if let ErrorEnvelope::Object(code, param) = envelope {
                s.record_event(
                    Event::OpenaiNestedErrorCode,
                    Outcome::Observed,
                    Some(error_code_category(code)),
                );
                s.record_event(
                    Event::OpenaiNestedErrorParam,
                    Outcome::Observed,
                    Some(error_param_category(param)),
                );
            }
        }
    }

    // The host may observe its deadline before the transport timer runs. Retain
    // that actual reason while abort/join performs cleanup, then seal on Drop.
    pub(crate) fn aborting_with(&self, error: DirectError) {
        if let Ok(mut state) = self.0.lock() {
            if !state.terminal {
                state.abort_error = Some(error);
            }
        }
    }
    pub(crate) fn record(&self, event: Event, outcome: Outcome, error: Option<Category>) {
        if let Ok(mut s) = self.0.lock() {
            if !s.terminal && !s.phases.contains(&event) {
                s.phases.push(event);
                s.record_event(event, outcome, error);
            }
        }
    }
    pub(crate) fn first(&self, text: bool) {
        if let Ok(mut s) = self.0.lock() {
            if s.terminal {
                return;
            }
            if !s.response {
                s.response = true;
                s.record_event(Event::FirstResponse, Outcome::Observed, None);
            }
            if text && !s.text {
                s.text = true;
                s.record_event(Event::FirstText, Outcome::Observed, None);
            }
        }
    }
    fn finish(&self, outcome: Outcome, error: Option<Category>) {
        if let Ok(mut s) = self.0.lock() {
            if s.terminal {
                return;
            }
            s.terminal = true;
            s.record_event(Event::RequestFinished, outcome, error);
            if s.record.stage.is_some() {
                s.record_event(Event::StageFinished, outcome, error);
            }
        }
    }
}
// Dropping the transport future releases local stream ownership; it does not
// claim remote cancellation or a successfully reaped child (logged separately).
pub(crate) struct TransportScope(pub(crate) Observer);
impl Drop for TransportScope {
    fn drop(&mut self) {
        self.0
            .record(Event::TransportReleased, Outcome::Observed, None);
    }
}
tokio::task_local! { pub(crate) static CURRENT: Observer; }
pub(crate) fn response_received() {
    let _ = CURRENT.try_with(|o| o.first(false));
}
pub(crate) fn top_level_error(code: TopLevelErrorCode, param: TopLevelErrorParam) {
    let _ = CURRENT.try_with(|o| o.top_level_error(code, param));
}
pub(crate) fn error_envelope(envelope: ErrorEnvelope) {
    let _ = CURRENT.try_with(|o| o.error_envelope(envelope));
}
fn error_code_category(code: TopLevelErrorCode) -> Category {
    match code {
        TopLevelErrorCode::ServerError => Category::OpenaiTopLevelServerError,
        TopLevelErrorCode::RateLimitExceeded => Category::OpenaiTopLevelRateLimitExceeded,
        TopLevelErrorCode::InvalidPrompt => Category::OpenaiTopLevelInvalidPrompt,
        TopLevelErrorCode::InvalidApiKey => Category::OpenaiInvalidApiKey,
        TopLevelErrorCode::InsufficientQuota => Category::OpenaiInsufficientQuota,
        TopLevelErrorCode::CreditBalanceExhausted => Category::OpenaiCreditBalanceExhausted,
        TopLevelErrorCode::ModelNotFound => Category::OpenaiModelNotFound,
        TopLevelErrorCode::UnsupportedValue => Category::OpenaiUnsupportedValue,
        TopLevelErrorCode::InvalidValue => Category::OpenaiInvalidValue,
        TopLevelErrorCode::Unknown => Category::OpenaiTopLevelUnknownCode,
        TopLevelErrorCode::Absent => Category::OpenaiTopLevelAbsentCode,
        TopLevelErrorCode::Null => Category::OpenaiTopLevelNullCode,
        TopLevelErrorCode::Invalid => Category::OpenaiTopLevelInvalidCode,
    }
}
fn error_param_category(param: TopLevelErrorParam) -> Category {
    match param {
        TopLevelErrorParam::Model => Category::OpenaiTopLevelParamModel,
        TopLevelErrorParam::Reasoning => Category::OpenaiTopLevelParamReasoning,
        TopLevelErrorParam::ReasoningEffort => Category::OpenaiTopLevelParamReasoningEffort,
        TopLevelErrorParam::MaxOutputTokens => Category::OpenaiTopLevelParamMaxOutputTokens,
        TopLevelErrorParam::ServiceTier => Category::OpenaiTopLevelParamServiceTier,
        TopLevelErrorParam::Absent => Category::OpenaiTopLevelParamAbsent,
        TopLevelErrorParam::Null => Category::OpenaiTopLevelParamNull,
        TopLevelErrorParam::Invalid => Category::OpenaiTopLevelParamInvalid,
        TopLevelErrorParam::Unknown => Category::OpenaiTopLevelParamUnknown,
    }
}

pub(crate) fn current() -> Option<Observer> {
    CURRENT.try_with(Clone::clone).ok()
}

#[cfg(any(target_os = "macos", test))]
pub(crate) fn export_new(path: &Path, snapshot: &Snapshot) -> io::Result<()> {
    if path.extension().and_then(|s| s.to_str()) != Some("json")
        || snapshot.events.len() > CAPACITY
        || snapshot.events.iter().any(|r| !r.valid())
    {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    let bytes = serde_json::to_vec_pretty(snapshot).map_err(|_| io::ErrorKind::InvalidData)?;
    let mut file = open_file(path, false)?;
    file.write_all(&bytes)?;
    file.sync_all()
}
#[cfg(test)]
mod tests;
