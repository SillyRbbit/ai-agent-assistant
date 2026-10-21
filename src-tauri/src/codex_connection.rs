//! Closed text-only Codex app-server adapter. No caller-supplied executable,
//! filesystem roots, RPC methods, tools, credentials or config reach the child.
use crate::{
    agent_models::ModelInfo,
    agent_preferences::ReasoningEffort,
    personal_assistant_direct::{DirectError, ProviderEvent},
};
use serde_json::{json, Value};
use std::{
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc, Mutex, Weak,
    },
    thread::JoinHandle,
    time::Duration,
};

const MAX_FRAME: usize = 262_144;
const MAX_FRAMES: usize = 4096;
const FEATURES: &[&str] = &[
    "shell_tool",
    "unified_exec",
    "apps",
    "plugins",
    "hooks",
    "multi_agent",
    "browser_use",
    "computer_use",
    "image_generation",
    "in_app_browser",
    "workspace_dependencies",
    "tool_suggest",
    "memories",
    "code_mode_host",
    "view_image",
    "shell_snapshot",
    "daemon_auto_start",
    "in_app_local_automation",
    "unbounded_connection_retries",
    "remote_plugin",
    "skill_search",
];
static NEXT: AtomicU64 = AtomicU64::new(0);
type OwnedChild = Arc<Mutex<Child>>;
static CHILDREN: Mutex<Vec<Weak<Mutex<Child>>>> = Mutex::new(Vec::new());
static SHUTTING_DOWN: AtomicBool = AtomicBool::new(false);

fn reap(child: &OwnedChild) -> bool {
    let Ok(mut child) = child.lock() else {
        return false;
    };
    let _ = child.kill();
    child.wait().is_ok()
}
fn reap_children(children: &[Weak<Mutex<Child>>]) -> bool {
    let mut complete = true;
    for child in children.iter().filter_map(Weak::upgrade) {
        complete &= reap(&child);
    }
    complete
}
/// Called before Tauri exits, independent of async runtime scheduling.
/// Only children registered by this adapter are affected.
pub(crate) fn shutdown() -> bool {
    SHUTTING_DOWN.store(true, Ordering::SeqCst);
    let Ok(children) = CHILDREN.lock() else {
        return false;
    };
    reap_children(&children)
}

// Created by native Setup and shared only with its matching presentation owner.
// Owner intent and the transport terminal decision share one linearization lock.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CancellationPhase {
    Preparing,
    Active,
    BeforeActive,
    OwnerStop,
    OwnerDrop,
    Completed,
    Rejected,
}
struct CancellationState {
    phase: CancellationPhase,
    callback_stopped: bool,
    returned_locally: bool,
}
#[derive(Clone)]
pub(crate) struct CancellationHandle(Arc<Mutex<CancellationState>>);
impl Default for CancellationHandle {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(CancellationState {
            phase: CancellationPhase::Preparing,
            callback_stopped: false,
            returned_locally: false,
        })))
    }
}
impl CancellationHandle {
    pub(crate) fn request_stop(&self) {
        self.request(false);
    }
    pub(crate) fn request_drop(&self) {
        self.request(true);
    }
    fn request(&self, dropping: bool) {
        let Ok(mut state) = self.0.lock() else {
            observe_fixed("owner_cancel_rejected", Err(DirectError::Internal));
            return;
        };
        let label = match state.phase {
            CancellationPhase::Preparing => {
                state.phase = CancellationPhase::BeforeActive;
                "owner_cancel_before_active"
            }
            CancellationPhase::Active => {
                state.phase = if dropping {
                    CancellationPhase::OwnerDrop
                } else {
                    CancellationPhase::OwnerStop
                };
                if dropping {
                    "owner_drop_active"
                } else {
                    "owner_stop_active"
                }
            }
            CancellationPhase::Completed => "owner_cancel_after_completion",
            _ => return,
        };
        observe_fixed(label, Ok(()));
    }
    fn arm(&self) -> Result<bool, DirectError> {
        let mut state = self.0.lock().map_err(|_| DirectError::Internal)?;
        match state.phase {
            CancellationPhase::Preparing => {
                state.phase = CancellationPhase::Active;
                observe_fixed("owner_cancel_armed", Ok(()));
                Ok(true)
            }
            CancellationPhase::BeforeActive => Ok(false),
            _ => Err(DirectError::Internal),
        }
    }
    fn active_requested(&self) -> bool {
        self.0.lock().is_ok_and(|s| {
            matches!(
                s.phase,
                CancellationPhase::OwnerStop | CancellationPhase::OwnerDrop
            )
        })
    }
    pub(crate) fn stopping_callback(&self) {
        if let Ok(mut state) = self.0.lock() {
            if matches!(
                state.phase,
                CancellationPhase::BeforeActive
                    | CancellationPhase::OwnerStop
                    | CancellationPhase::OwnerDrop
            ) {
                state.callback_stopped = true;
                observe_fixed("owner_cancel_callback", Ok(()));
            }
        }
    }
    fn take_stopping_callback(&self) -> bool {
        self.0.lock().is_ok_and(|mut s| {
            std::mem::take(&mut s.callback_stopped)
                && matches!(
                    s.phase,
                    CancellationPhase::BeforeActive
                        | CancellationPhase::OwnerStop
                        | CancellationPhase::OwnerDrop
                )
        })
    }
    fn complete(&self) -> Result<bool, DirectError> {
        let mut state = self.0.lock().map_err(|_| DirectError::Internal)?;
        match state.phase {
            CancellationPhase::Active => {
                state.phase = CancellationPhase::Completed;
                observe_fixed("owner_completion_won", Ok(()));
                Ok(true)
            }
            CancellationPhase::BeforeActive
            | CancellationPhase::OwnerStop
            | CancellationPhase::OwnerDrop => Ok(false),
            _ => Err(DirectError::Internal),
        }
    }
    fn reject(&self) {
        if let Ok(mut state) = self.0.lock() {
            state.phase = CancellationPhase::Rejected;
            state.callback_stopped = false;
            state.returned_locally = false;
        }
    }
    fn local_return(&self) {
        if let Ok(mut state) = self.0.lock() {
            if matches!(
                state.phase,
                CancellationPhase::BeforeActive
                    | CancellationPhase::OwnerStop
                    | CancellationPhase::OwnerDrop
            ) {
                state.returned_locally = true;
                observe_fixed("owner_cancel_returned", Ok(()));
            }
        }
    }
    fn returned_locally(&self) -> bool {
        self.0.lock().is_ok_and(|s| {
            s.returned_locally
                && matches!(
                    s.phase,
                    CancellationPhase::BeforeActive
                        | CancellationPhase::OwnerStop
                        | CancellationPhase::OwnerDrop
                )
        })
    }
}

// Intentionally no Debug: even owner configuration stays out of error output.
pub(crate) struct Setup {
    executable: PathBuf,
    home: PathBuf,
    derived: bool,
    verified_derived: Mutex<Option<VerifiedArtifact>>,
    cancellation: CancellationHandle,
}
impl Setup {
    pub(crate) fn from_environment() -> Result<Self, DirectError> {
        if !cfg!(debug_assertions) || std::env::var("CORTEXA_CODEX_DEMO").as_deref() != Ok("1") {
            return Err(rejection_error("predicate_000", DirectError::Disabled));
        }
        let executable = std::env::var_os("CORTEXA_CODEX_EXECUTABLE")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from("/Applications/ChatGPT.app/Contents/Resources/codex-cli/bin/codex")
            });
        let home = std::env::var_os("CORTEXA_CODEX_HOME")
            .map(PathBuf::from)
            .ok_or_else(|| rejection_error("predicate_085", DirectError::CodexSetup))?;
        if !executable.is_absolute()
            || !executable.is_file()
            || !home.is_absolute()
            || !home.is_dir()
            || home.join("config.toml").exists()
            || home.join("AGENTS.md").exists()
        {
            return Err(rejection_error("predicate_001", DirectError::CodexSetup));
        }
        let verified = prepare_derived_artifact(&executable, DERIVED_BYTES, DERIVED_SHA256)?;
        let derived = verified.is_some();
        Ok(Self {
            executable,
            home,
            derived,
            verified_derived: Mutex::new(verified),
            cancellation: CancellationHandle::default(),
        })
    }

    pub(crate) fn cancellation_handle(&self) -> CancellationHandle {
        self.cancellation.clone()
    }

    fn consume_verified(&self) -> Result<Option<VerifiedArtifact>, DirectError> {
        let verified = self
            .verified_derived
            .lock()
            .map_err(|_| rejection_error("predicate_002", DirectError::CodexSetup))?
            .take();
        match (self.derived, verified) {
            (true, Some(verified)) => {
                verified.verify(&self.executable)?;
                Ok(Some(verified))
            }
            (false, None) => Ok(None),
            _ => Err(rejection_error("predicate_002", DirectError::CodexSetup)),
        }
    }
}

// The derived lane rejects duplicate keys before Value loses their identity.
// No rejected text or serde diagnostic escapes this decoder.
struct UniqueValue(Value);
impl<'de> serde::Deserialize<'de> for UniqueValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UniqueValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("bounded unique JSON")
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(UniqueValue(v.into()))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(UniqueValue(v.into()))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(UniqueValue(v.into()))
            }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
                serde_json::Number::from_f64(v)
                    .map(|n| UniqueValue(Value::Number(n)))
                    .ok_or_else(|| E::custom("number"))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(UniqueValue(v.into()))
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(UniqueValue(v.into()))
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(UniqueValue(Value::Null))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut a: A,
            ) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(UniqueValue(v)) = a.next_element()? {
                    values.push(v);
                }
                Ok(UniqueValue(Value::Array(values)))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut a: A,
            ) -> Result<Self::Value, A::Error> {
                let mut values = serde_json::Map::new();
                while let Some(k) = a.next_key::<String>()? {
                    if values.contains_key(&k) {
                        return Err(serde::de::Error::custom("duplicate key"));
                    }
                    let UniqueValue(v) = a.next_value()?;
                    values.insert(k, v);
                }
                Ok(UniqueValue(Value::Object(values)))
            }
        }
        d.deserialize_any(Visitor)
    }
}
fn decode_frame(bytes: &[u8], derived: bool) -> Result<Value, DirectError> {
    if derived {
        let UniqueValue(v) = serde_json::from_slice(bytes).map_err(|_| DirectError::Protocol)?;
        bounded_frame(&v, 0, &mut 0)?;
        Ok(v)
    } else {
        serde_json::from_slice(bytes).map_err(|_| DirectError::Protocol)
    }
}
#[derive(Default)]
struct Capture {
    bytes: usize,
    frames: usize,
    eof: bool,
    failure: Option<DirectError>,
}
impl Capture {
    fn charge(&mut self, bytes: usize) -> Result<(), DirectError> {
        self.bytes = self.bytes.saturating_add(bytes);
        self.frames = self.frames.saturating_add(1);
        if bytes > MAX_FRAME || self.bytes > 1_048_576 || self.frames > MAX_FRAMES {
            self.failure = Some(DirectError::Limit);
            return Err(DirectError::Limit);
        }
        Ok(())
    }
    fn complete(&self) -> Result<(), DirectError> {
        if let Some(error) = self.failure {
            return Err(error);
        }
        if !self.eof {
            return Err(DirectError::Incomplete);
        }
        Ok(())
    }
}
fn read_frames(
    mut reader: impl BufRead,
    output: &mpsc::SyncSender<Result<Value, DirectError>>,
    capture: &Mutex<Capture>,
    derived: bool,
) {
    loop {
        let mut bytes = Vec::new();
        let read =
            std::io::Read::take(&mut reader, (MAX_FRAME + 1) as u64).read_until(b'\n', &mut bytes);
        let Ok(mut state) = capture.lock() else {
            return;
        };
        if matches!(read, Ok(0)) {
            state.eof = true;
            return;
        }
        let result = if derived && state.charge(bytes.len()).is_err() {
            Err(DirectError::Limit)
        } else {
            match read {
                Ok(n) if n <= MAX_FRAME && bytes.last() == Some(&b'\n') => {
                    decode_frame(&bytes, derived)
                }
                _ => Err(DirectError::Limit),
            }
        };
        if let Err(error) = result {
            state.failure = Some(error);
        }
        let failed = result.is_err();
        if output.try_send(result).is_err() {
            state.failure = Some(DirectError::Limit);
            return;
        }
        if failed {
            return;
        }
    }
}
const WARNING_MESSAGES: &[(&str, &str)] = &[
    ("Code Mode is unavailable because code-mode host is disabled. Falling back to direct tools; enable `features.code_mode_host` and install `codex-code-mode-host`.", "warning_disabled_code_mode_direct_fallback"),
    ("Code Mode is unavailable because code-mode host is disabled. Code mode will fail closed; enable `features.code_mode_host` and install `codex-code-mode-host`.", "warning_disabled_code_mode_fail_closed"),
    ("Model metadata for `gpt-5.6-luna` not found. Defaulting to fallback metadata; this can degrade performance and cause issues.", "warning_model_metadata_fallback"),
    ("Code Mode is enabled in configuration, but model `gpt-5.6-luna` does not advertise Code Mode support. This may degrade model performance. Disable `features.code_mode` and `features.code_mode_only`, or select a model whose metadata enables Code Mode.", "warning_code_mode_unsupported_model"),
    ("`--dangerously-bypass-hook-trust` is enabled. Enabled hooks may run without review for this invocation.", "warning_hook_trust_bypass_enabled"),
    ("Codex couldn't save diagnostic logs to its local database. Run `codex doctor` for diagnostics.", "warning_log_database_write_failure"),
];
const NOTIFICATION_METHOD_LABELS: &[(&str, &str)] = &[
    ("error", "known_notification_000"),
    ("thread/started", "known_notification_001"),
    ("thread/status/changed", "known_notification_002"),
    ("thread/archived", "known_notification_003"),
    ("thread/deleted", "known_notification_004"),
    ("thread/unarchived", "known_notification_005"),
    ("thread/closed", "known_notification_006"),
    ("thread/reverted", "known_notification_007"),
    ("skills/changed", "known_notification_008"),
    ("thread/name/updated", "known_notification_009"),
    ("thread/attachment/updated", "known_notification_010"),
    ("thread/goal/updated", "known_notification_011"),
    ("thread/prediction/updated", "known_notification_012"),
    ("thread/goal/cleared", "known_notification_013"),
    ("thread/queue/changed", "known_notification_014"),
    ("project/changed", "known_notification_015"),
    ("thread/project/updated", "known_notification_016"),
    ("thread/environment/connected", "known_notification_017"),
    ("thread/environment/disconnected", "known_notification_018"),
    ("thread/settings/updated", "known_notification_019"),
    ("thread/tokenUsage/updated", "known_notification_020"),
    ("turn/started", "known_notification_021"),
    ("hook/started", "known_notification_022"),
    ("turn/completed", "known_notification_023"),
    ("hook/completed", "known_notification_024"),
    ("turn/diff/updated", "known_notification_025"),
    ("turn/plan/updated", "known_notification_026"),
    ("item/started", "known_notification_027"),
    ("item/autoApprovalReview/started", "known_notification_028"),
    (
        "item/autoApprovalReview/completed",
        "known_notification_029",
    ),
    (
        "autoApprovalReview/strictReviewRequired",
        "known_notification_030",
    ),
    ("item/completed", "known_notification_031"),
    ("item/agentMessage/delta", "known_notification_032"),
    ("item/plan/delta", "known_notification_033"),
    ("command/exec/outputDelta", "known_notification_034"),
    ("process/outputDelta", "known_notification_035"),
    ("process/exited", "known_notification_036"),
    (
        "item/commandExecution/outputDelta",
        "known_notification_037",
    ),
    (
        "item/commandExecution/terminalInteraction",
        "known_notification_038",
    ),
    ("item/fileChange/outputDelta", "known_notification_039"),
    ("item/fileChange/patchUpdated", "known_notification_040"),
    ("serverRequest/resolved", "known_notification_041"),
    ("item/mcpToolCall/progress", "known_notification_042"),
    ("mcpServer/oauthLogin/completed", "known_notification_043"),
    ("mcpServer/startupStatus/updated", "known_notification_044"),
    (
        "mcpServer/event/stream/notification",
        "known_notification_045",
    ),
    ("account/updated", "known_notification_046"),
    ("account/gatewayOAuth/changed", "known_notification_047"),
    ("account/rateLimits/updated", "known_notification_048"),
    ("app/list/updated", "known_notification_049"),
    ("remoteControl/status/changed", "known_notification_050"),
    (
        "externalAgentConfig/import/progress",
        "known_notification_051",
    ),
    (
        "externalAgentConfig/import/completed",
        "known_notification_052",
    ),
    ("fs/changed", "known_notification_053"),
    ("item/reasoning/summaryTextDelta", "known_notification_054"),
    ("item/reasoning/summaryPartAdded", "known_notification_055"),
    ("item/reasoning/textDelta", "known_notification_056"),
    ("thread/compacted", "known_notification_057"),
    ("model/rerouted", "known_notification_058"),
    ("model/verification", "known_notification_059"),
    (
        "modelProvider/authRecoveryStarted",
        "known_notification_060",
    ),
    (
        "modelProvider/authRecoveryCompleted",
        "known_notification_061",
    ),
    ("turn/moderationMetadata", "known_notification_062"),
    ("model/safetyBuffering/updated", "known_notification_063"),
    ("warning", "known_notification_064"),
    ("guardianWarning", "known_notification_065"),
    ("deprecationNotice", "known_notification_066"),
    ("configWarning", "known_notification_067"),
    ("fuzzyFileSearch/sessionUpdated", "known_notification_068"),
    ("fuzzyFileSearch/sessionCompleted", "known_notification_069"),
    ("thread/realtime/started", "known_notification_070"),
    ("thread/realtime/itemAdded", "known_notification_071"),
    ("thread/realtime/item/started", "known_notification_072"),
    (
        "thread/realtime/item/transcript/delta",
        "known_notification_073",
    ),
    ("thread/realtime/item/completed", "known_notification_074"),
    ("thread/realtime/transcript/delta", "known_notification_075"),
    ("thread/realtime/transcript/done", "known_notification_076"),
    (
        "thread/realtime/outputAudio/delta",
        "known_notification_077",
    ),
    ("thread/realtime/sdp", "known_notification_078"),
    ("thread/realtime/error", "known_notification_079"),
    ("thread/realtime/closed", "known_notification_080"),
    ("windows/worldWritableWarning", "known_notification_081"),
    ("windowsSandbox/setupCompleted", "known_notification_082"),
    ("account/login/completed", "known_notification_083"),
];
fn notification_method_site(value: &Value) -> &'static str {
    match value.get("method") {
        None => "notification_method_missing",
        Some(Value::String(method)) => NOTIFICATION_METHOD_LABELS
            .iter()
            .find(|(name, _)| *name == method)
            .map_or("unknown_notification_method", |(_, label)| *label),
        Some(_) => "notification_method_non_string",
    }
}
fn reject_notification(value: &Value, error: DirectError, phase: &'static str) -> DirectError {
    observe_fixed(phase, Err(error));
    rejection_error(notification_method_site(value), error)
}
const OBSERVATION_LABELS: &[&str] = &[
    "moderation_metadata_handled",
    "moderation_metadata_context_rejected",
    "moderation_metadata_schema_rejected",
    "known_notification_000",
    "known_notification_001",
    "known_notification_002",
    "known_notification_003",
    "known_notification_004",
    "known_notification_005",
    "known_notification_006",
    "known_notification_007",
    "known_notification_008",
    "known_notification_009",
    "known_notification_010",
    "known_notification_011",
    "known_notification_012",
    "known_notification_013",
    "known_notification_014",
    "known_notification_015",
    "known_notification_016",
    "known_notification_017",
    "known_notification_018",
    "known_notification_019",
    "known_notification_020",
    "known_notification_021",
    "known_notification_022",
    "known_notification_023",
    "known_notification_024",
    "known_notification_025",
    "known_notification_026",
    "known_notification_027",
    "known_notification_028",
    "known_notification_029",
    "known_notification_030",
    "known_notification_031",
    "known_notification_032",
    "known_notification_033",
    "known_notification_034",
    "known_notification_035",
    "known_notification_036",
    "known_notification_037",
    "known_notification_038",
    "known_notification_039",
    "known_notification_040",
    "known_notification_041",
    "known_notification_042",
    "known_notification_043",
    "known_notification_044",
    "known_notification_045",
    "known_notification_046",
    "known_notification_047",
    "known_notification_048",
    "known_notification_049",
    "known_notification_050",
    "known_notification_051",
    "known_notification_052",
    "known_notification_053",
    "known_notification_054",
    "known_notification_055",
    "known_notification_056",
    "known_notification_057",
    "known_notification_058",
    "known_notification_059",
    "known_notification_060",
    "known_notification_061",
    "known_notification_062",
    "known_notification_063",
    "known_notification_064",
    "known_notification_065",
    "known_notification_066",
    "known_notification_067",
    "known_notification_068",
    "known_notification_069",
    "known_notification_070",
    "known_notification_071",
    "known_notification_072",
    "known_notification_073",
    "known_notification_074",
    "known_notification_075",
    "known_notification_076",
    "known_notification_077",
    "known_notification_078",
    "known_notification_079",
    "known_notification_080",
    "known_notification_081",
    "known_notification_082",
    "known_notification_083",
    "notification_method_missing",
    "notification_method_non_string",
    "unknown_notification_method",
    "notification_phase_before_response",
    "notification_phase_stream",
    "notification_phase_shutdown",
    "warning_disabled_code_mode_fail_closed_handled",
    "warning_disabled_code_mode_direct_fallback",
    "warning_disabled_code_mode_fail_closed",
    "warning_model_metadata_fallback",
    "warning_code_mode_unsupported_model",
    "warning_hook_trust_bypass_enabled",
    "warning_log_database_write_failure",
    "warning_unknown",
    "warning_malformed",
    "warning_unbound",
    "warning_over_bound",
    "warning_before_response",
    "warning_streaming",
    "warning_shutdown",
    "protocol_tail_complete",
    "protocol_tail_rejected",
    "protocol_abort_tail_complete",
    "protocol_abort_tail_rejected",
    "protocol_capture_incomplete",
    "owner_cancel_rejected",
    "owner_cancel_before_active",
    "owner_drop_active",
    "owner_stop_active",
    "owner_cancel_after_completion",
    "owner_cancel_armed",
    "owner_cancel_callback",
    "owner_completion_won",
    "owner_cancel_returned",
    "abort_eligibility_verified",
    "abort_eligibility_rejected",
    "abort_guard_authenticated_verified",
    "abort_guard_authenticated_rejected",
    "abort_guard_deny_all_verified",
    "abort_guard_deny_all_rejected",
    "abort_guard_low_effort_verified",
    "abort_guard_low_effort_rejected",
    "abort_guard_model_verified",
    "abort_guard_model_rejected",
    "abort_guard_thread_verified",
    "abort_guard_thread_rejected",
    "abort_guard_turn_verified",
    "abort_guard_turn_rejected",
    "abort_guard_unpoisoned_verified",
    "abort_guard_unpoisoned_rejected",
    "abort_turn_fields_verified",
    "abort_turn_context_rejected",
    "abort_turn_shape_rejected",
    "abort_turn_identifier_rejected",
    "abort_turn_time_rejected",
    "abort_turn_error_rejected",
    "abort_turn_items_view_rejected",
    "abort_turn_root_turn_rejected",
    "abort_turn_items_array_rejected",
    "abort_turn_status_rejected",
    "abort_turn_nonempty_items_rejected",
];
fn observe_fixed(label: &'static str, result: Result<(), DirectError>) {
    if !OBSERVATION_LABELS.contains(&label) {
        return;
    }
    let mut attempt = crate::diagnostics::Attempt::new(
        crate::agent_preferences::AgentConnection::Codex,
        "",
        None,
        None,
        Some(label),
    );
    attempt.finish(result);
}
fn warning_label(value: &Value, thread: Option<&str>) -> &'static str {
    if derived_envelope(value).is_err()
        || value["method"] != "warning"
        || closed(&value["params"], &["message"], &["threadId"]).is_err()
    {
        return "warning_malformed";
    }
    let Some(message) = value["params"]["message"].as_str() else {
        return "warning_malformed";
    };
    if message.len() > 4096 {
        return "warning_over_bound";
    }
    if thread.is_none()
        || value["params"]["threadId"].as_str() != thread
        || identifier(&value["params"]["threadId"]).is_err()
    {
        return "warning_unbound";
    }
    WARNING_MESSAGES
        .iter()
        .find(|(text, _)| *text == message)
        .map_or("warning_unknown", |(_, label)| *label)
}
fn reject_warning(
    value: &Value,
    thread: Option<&str>,
    phase: &'static str,
) -> Result<(), DirectError> {
    if value["method"] == "warning" {
        observe_fixed(phase, Err(DirectError::Protocol));
        let label = warning_label(value, thread);
        // Classification never changes the original fatal warning result.
        return Err(rejection_error(label, DirectError::Protocol));
    }
    Ok(())
}
fn validate_tail(policy: &mut DerivedState, value: &Value) -> Result<(), DirectError> {
    derived_envelope(value)?;
    if policy.handle_warning(value, "warning_shutdown")? {
        return Ok(());
    }
    if value.get("id").is_some()
        || value["method"] == "thread/started"
        || (value["method"] == "thread/status/changed"
            && !matches!(
                value["params"]["status"]["type"].as_str(),
                Some("idle" | "notLoaded")
            ))
    {
        return Err(DirectError::Protocol);
    }
    reject_tool(value)?;
    if !policy.benign(value)? {
        return Err(reject_notification(
            value,
            DirectError::Protocol,
            "notification_phase_shutdown",
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TailMode {
    Strict,
    LocalAbort,
}

fn returned_tail_mode(local_limit: bool, result: &Result<(), DirectError>) -> TailMode {
    if local_limit && matches!(result, Err(DirectError::Limit)) {
        TailMode::LocalAbort
    } else {
        TailMode::Strict
    }
}

#[derive(Default)]
struct AbortStream {
    started: bool,
    terminal: bool,
    poisoned: bool,
    characters: usize,
    bytes: usize,
    items: std::collections::BTreeMap<String, (&'static str, bool)>,
}
impl AbortStream {
    // Only already schema-validated frames enter this bounded lifecycle tracker.
    // Tracking never alters ordinary successful stream acceptance.
    fn observe(&mut self, value: &Value) -> Result<(), DirectError> {
        if self.poisoned {
            return Err(DirectError::Protocol);
        }
        let p = &value["params"];
        match value["method"].as_str() {
            Some("turn/started") => {
                abort_turn_fields(&p["turn"], false)?;
                if self.started || self.terminal || !self.items.is_empty() {
                    return Err(DirectError::Protocol);
                }
                self.started = true;
            }
            Some("item/started" | "item/completed") => {
                if self.terminal {
                    return Err(DirectError::Protocol);
                }
                abort_item(&p["item"])?;
                let time_key = if value["method"] == "item/started" {
                    "startedAtMs"
                } else {
                    "completedAtMs"
                };
                if p[time_key].as_i64().is_none() {
                    return Err(DirectError::Protocol);
                }
                let id = identifier(&p["item"]["id"])?;
                let kind = match p["item"]["type"].as_str() {
                    Some("agentMessage") => "agentMessage",
                    Some("reasoning") => "reasoning",
                    Some("userMessage") => "userMessage",
                    _ => return Err(DirectError::CodexIsolation),
                };
                if value["method"] == "item/started" {
                    if self.items.len() >= MAX_FRAMES || self.items.contains_key(&id) {
                        return Err(DirectError::Protocol);
                    }
                    self.items.insert(id, (kind, false));
                } else {
                    let item = self.items.get_mut(&id).ok_or(DirectError::Protocol)?;
                    if item.0 != kind || item.1 {
                        return Err(DirectError::Protocol);
                    }
                    item.1 = true;
                }
            }
            Some(
                "item/agentMessage/delta"
                | "item/reasoning/summaryTextDelta"
                | "item/reasoning/textDelta"
                | "item/reasoning/summaryPartAdded",
            ) => {
                let id = identifier(&p["itemId"])?;
                let item = self.items.get(&id).ok_or(DirectError::Protocol)?;
                let message = value["method"] == "item/agentMessage/delta";
                if self.terminal
                    || item.1
                    || item.0 != if message { "agentMessage" } else { "reasoning" }
                {
                    return Err(DirectError::Protocol);
                }
                if message {
                    let text = p["delta"].as_str().ok_or(DirectError::Protocol)?;
                    self.characters = self.characters.saturating_add(text.chars().count());
                    self.bytes = self.bytes.saturating_add(text.len());
                    if self.characters > 8192 || self.bytes > 32768 {
                        return Err(DirectError::Limit);
                    }
                }
            }
            Some("turn/completed") => {
                abort_turn_fields(&p["turn"], true)?;
                if self.terminal
                    || !matches!(
                        p["turn"]["status"].as_str(),
                        Some("completed" | "interrupted" | "failed")
                    )
                {
                    return Err(DirectError::Protocol);
                }
                for key in ["completedAt", "durationMs", "startedAt"] {
                    if p["turn"]
                        .get(key)
                        .is_some_and(|v| !v.is_null() && v.as_i64().is_none())
                    {
                        return Err(DirectError::Protocol);
                    }
                }
                if p["turn"].get("error").is_some_and(|v| !v.is_null())
                    || p["turn"]
                        .get("rootTurnId")
                        .is_some_and(|v| !v.is_null() && *v != p["turn"]["id"])
                {
                    return Err(DirectError::Protocol);
                }
                let items = p["turn"]["items"].as_array().ok_or(DirectError::Protocol)?;
                let mut seen = std::collections::BTreeSet::new();
                for item in items {
                    abort_item(item)?;
                    let id = identifier(&item["id"])?;
                    let prior = self.items.get(&id).ok_or(DirectError::Protocol)?;
                    if !seen.insert(id)
                        || item["type"].as_str() != Some(prior.0)
                        || (p["turn"]["itemsView"] == "summary" && !prior.1)
                    {
                        return Err(DirectError::Protocol);
                    }
                }
                self.terminal = true;
            }
            _ => {}
        }
        Ok(())
    }
}
fn abort_turn_fields_checked(
    turn: &Value,
    terminal: bool,
) -> Result<(), (DirectError, &'static str)> {
    closed(
        turn,
        &["id", "items", "status"],
        &[
            "completedAt",
            "durationMs",
            "error",
            "itemsView",
            "rootTurnId",
            "startedAt",
        ],
    )
    .map_err(|e| (e, "abort_turn_shape_rejected"))?;
    identifier(&turn["id"]).map_err(|e| (e, "abort_turn_identifier_rejected"))?;
    for key in ["completedAt", "durationMs", "startedAt"] {
        if turn
            .get(key)
            .is_some_and(|v| !v.is_null() && v.as_i64().is_none())
        {
            return Err((DirectError::Protocol, "abort_turn_time_rejected"));
        }
    }
    if turn.get("error").is_some_and(|v| !v.is_null()) {
        return Err((DirectError::Protocol, "abort_turn_error_rejected"));
    }
    if turn
        .get("rootTurnId")
        .is_some_and(|v| !v.is_null() && *v != turn["id"])
    {
        return Err((DirectError::Protocol, "abort_turn_root_turn_rejected"));
    }
    let items = turn["items"]
        .as_array()
        .ok_or((DirectError::Protocol, "abort_turn_items_array_rejected"))?;
    // The pinned producer emits unloaded initial/interrupted snapshots and a
    // final-answer summary on success. These are not complete item histories;
    // AbortStream retains independent per-item lifecycle validation.
    let view_valid = match turn.get("itemsView").and_then(Value::as_str) {
        None => turn.get("itemsView").is_none(),
        Some("full") => true,
        Some("notLoaded") => items.is_empty(),
        Some("summary") => {
            terminal
                && turn["status"] == "completed"
                && items.len() == 1
                && items[0]["type"] == "agentMessage"
                && items[0]["text"]
                    .as_str()
                    .is_some_and(|text| !text.trim().is_empty())
                && items[0]
                    .get("phase")
                    .is_none_or(|phase| phase.is_null() || *phase == "final_answer")
                && abort_item(&items[0]).is_ok()
        }
        Some(_) => false,
    };
    if !view_valid {
        return Err((DirectError::Protocol, "abort_turn_items_view_rejected"));
    }
    if terminal {
        if !matches!(
            turn["status"].as_str(),
            Some("completed" | "interrupted" | "failed")
        ) {
            return Err((DirectError::Protocol, "abort_turn_status_rejected"));
        }
    } else {
        if turn["status"] != "inProgress" {
            return Err((DirectError::Protocol, "abort_turn_status_rejected"));
        }
        if !items.is_empty() {
            return Err((DirectError::Protocol, "abort_turn_nonempty_items_rejected"));
        }
    }
    Ok(())
}
fn abort_turn_fields(turn: &Value, terminal: bool) -> Result<(), DirectError> {
    abort_turn_fields_checked(turn, terminal).map_err(|(error, _)| error)
}
fn abort_item(item: &Value) -> Result<(), DirectError> {
    validate_text_item(item)?;
    if item["type"] == "agentMessage"
        && (item
            .get("delivery")
            .is_some_and(|v| !v.is_null() && *v != "async")
            || item.get("phase").is_some_and(|v| {
                !v.is_null()
                    && !matches!(
                        v.as_str(),
                        Some("commentary" | "partial_answer" | "final_answer")
                    )
            })
            || item.get("memoryCitation").is_some_and(|v| !v.is_null()))
    {
        return Err(DirectError::Protocol);
    }
    if item["type"] == "userMessage" {
        if item
            .get("clientId")
            .is_some_and(|v| !v.is_null() && identifier(v).is_err())
        {
            return Err(DirectError::Protocol);
        }
        for part in item["content"].as_array().ok_or(DirectError::Protocol)? {
            if part.get("text_elements").is_some_and(|v| *v != json!([])) {
                return Err(DirectError::Protocol);
            }
        }
    }
    Ok(())
}
fn abort_context_checks(policy: &DerivedState, state: &AbortStream) -> [bool; 7] {
    let valid = |id: &Option<String>| {
        id.as_deref()
            .is_some_and(|s| !s.is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
    };
    [
        policy.authenticated,
        policy.deny_all_verified,
        policy.low_effort_verified,
        policy.model.as_deref() == Some("gpt-5.6-luna"),
        valid(&policy.thread),
        valid(&policy.turn),
        !state.poisoned,
    ]
}
fn abort_context(policy: &DerivedState, state: &AbortStream) -> bool {
    abort_context_checks(policy, state)
        .into_iter()
        .all(|value| value)
}
fn observe_abort_context(checks: [bool; 7]) -> bool {
    let labels = [
        (
            "abort_guard_authenticated_verified",
            "abort_guard_authenticated_rejected",
        ),
        (
            "abort_guard_deny_all_verified",
            "abort_guard_deny_all_rejected",
        ),
        (
            "abort_guard_low_effort_verified",
            "abort_guard_low_effort_rejected",
        ),
        ("abort_guard_model_verified", "abort_guard_model_rejected"),
        ("abort_guard_thread_verified", "abort_guard_thread_rejected"),
        ("abort_guard_turn_verified", "abort_guard_turn_rejected"),
        (
            "abort_guard_unpoisoned_verified",
            "abort_guard_unpoisoned_rejected",
        ),
    ];
    for (value, (positive, negative)) in checks.into_iter().zip(labels) {
        observe_fixed(
            if value { positive } else { negative },
            if value {
                Ok(())
            } else {
                Err(DirectError::CodexIsolation)
            },
        );
    }
    checks.into_iter().all(|value| value)
}
fn validate_abort_tail(
    policy: &mut DerivedState,
    state: &mut AbortStream,
    value: &Value,
) -> Result<(), DirectError> {
    if !abort_context(policy, state) {
        return Err(DirectError::CodexIsolation);
    }
    derived_envelope(value)?;
    if value.get("id").is_some() || value["method"] == "thread/started" {
        return Err(DirectError::Protocol);
    }
    reject_tool(value)?;
    match value["method"].as_str() {
        Some(
            "turn/started"
            | "turn/completed"
            | "item/started"
            | "item/completed"
            | "item/agentMessage/delta"
            | "item/reasoning/summaryTextDelta"
            | "item/reasoning/textDelta"
            | "item/reasoning/summaryPartAdded"
            | "thread/tokenUsage/updated"
            | "turn/moderationMetadata",
        ) => {
            let p = &value["params"];
            let turn = if matches!(
                value["method"].as_str(),
                Some("turn/started" | "turn/completed")
            ) {
                &p["turn"]["id"]
            } else {
                &p["turnId"]
            };
            if identifier(&p["threadId"])?
                != policy.thread.as_deref().ok_or(DirectError::Protocol)?
                || identifier(turn)? != policy.turn.as_deref().ok_or(DirectError::Protocol)?
            {
                return Err(DirectError::Protocol);
            }
            // Existing exact schemas, tool rejection and opaque metadata checks
            // remain controlling; the prebound turn cannot be learned here.
            policy.stream_at(value, "notification_phase_stream")?;
            state.observe(value)
        }
        Some("thread/status/changed") => {
            if state.terminal && value["params"]["status"]["type"] == "active" {
                return Err(DirectError::Protocol);
            }
            if policy.benign(value)? {
                Ok(())
            } else {
                Err(DirectError::Protocol)
            }
        }
        _ => validate_tail(policy, value),
    }
}

struct Session {
    child: OwnedChild,
    tx: Option<mpsc::SyncSender<Vec<u8>>>,
    rx: mpsc::Receiver<Result<Value, DirectError>>,
    readers: Vec<JoinHandle<()>>,
    directory: PathBuf,
    frames: usize,
    derived: bool,
    home: PathBuf,
    policy: DerivedState,
    deferred: std::collections::VecDeque<(Value, usize)>,
    deferred_bytes: usize,
    diagnostic: Option<crate::diagnostics::Observer>,
    capture: Arc<Mutex<Capture>>,
    cleaned: bool,
    tail_mode: TailMode,
    cancellation: CancellationHandle,
    abort_stream: AbortStream,
}
impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}
impl Session {
    // A successful turn must explicitly inspect this result before completion.
    // Drop still cleans failures/cancellation, without replacing their primary error.
    fn finish(&mut self) -> Result<(), DirectError> {
        if self.cleaned {
            return Ok(());
        }
        self.cleaned = true;
        self.tx.take();
        let reaped = reap(&self.child);
        let mut joined = true;
        for reader in self.readers.drain(..) {
            joined &= reader.join().is_ok();
        }
        let abort = self.tail_mode == TailMode::LocalAbort || self.cancellation.active_requested();
        let mut tail = if abort && !abort_context(&self.policy, &self.abort_stream) {
            Err(DirectError::CodexIsolation)
        } else {
            Ok(())
        };
        if self.derived {
            for (value, _) in self.deferred.drain(..) {
                let result = if abort {
                    validate_abort_tail(&mut self.policy, &mut self.abort_stream, &value)
                } else {
                    validate_tail(&mut self.policy, &value)
                };
                if tail.is_ok() {
                    tail = result;
                }
            }
            while let Ok(value) = self.rx.try_recv() {
                let result = value.and_then(|v| {
                    if abort {
                        validate_abort_tail(&mut self.policy, &mut self.abort_stream, &v)
                    } else {
                        validate_tail(&mut self.policy, &v)
                    }
                });
                if tail.is_ok() {
                    tail = result;
                }
            }
            let complete = self
                .capture
                .lock()
                .map_err(|_| DirectError::Internal)
                .and_then(|state| state.complete());
            if complete.is_err() {
                observe_fixed("protocol_capture_incomplete", complete);
            }
            if tail.is_ok() {
                tail = complete;
            }
        }
        let removed = std::fs::remove_dir_all(&self.directory).is_ok();
        if let Some(o) = &self.diagnostic {
            use crate::diagnostics::{Category, Event, Outcome};
            o.record(
                Event::RuntimeExited,
                if reaped {
                    Outcome::Completed
                } else {
                    Outcome::Failed
                },
                if reaped {
                    None
                } else {
                    Some(Category::Runtime)
                },
            );
            o.record(
                Event::RuntimeCleanup,
                if removed && reaped && joined {
                    Outcome::Completed
                } else {
                    Outcome::Failed
                },
                if removed && reaped && joined {
                    None
                } else {
                    Some(Category::Cleanup)
                },
            );
        }
        if !reaped || !joined || !removed {
            tail = Err(DirectError::Internal);
        }
        if tail.is_err() {
            self.cancellation.reject();
        }
        if self.derived {
            observe_fixed(
                match (abort, tail.is_ok()) {
                    (false, true) => "protocol_tail_complete",
                    (false, false) => "protocol_tail_rejected",
                    (true, true) => "protocol_abort_tail_complete",
                    (true, false) => "protocol_abort_tail_rejected",
                },
                tail,
            );
        }
        tail
    }
}

fn command(setup: &Setup, directory: &std::path::Path) -> Command {
    let mut command = Command::new(&setup.executable);
    command
        .env_clear()
        .env("HOME", directory)
        .env("CODEX_HOME", &setup.home)
        .env("PATH", "/usr/bin:/bin")
        .current_dir(directory)
        .args([
            "app-server",
            "--stdio",
            "--strict-config",
            "-c",
            "web_search=\"disabled\"",
            "-c",
            "cli_auth_credentials_store=\"file\"",
        ]);
    for feature in FEATURES {
        command.args(["-c", &format!("features.{feature}=false")]);
    }
    if setup.derived {
        for feature in DERIVED_FEATURES {
            command.args(["-c", &format!("features.{feature}=false")]);
        }
        command.args([
            "-c",
            "tools.experimental_request_user_input.enabled=false",
            "-c",
            "tools.update_plan.enabled=false",
        ]);
    }
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    command
}
impl Session {
    fn spawn(setup: &Setup) -> Result<Self, DirectError> {
        let verified = setup.consume_verified()?;
        let directory = std::env::temp_dir().join(format!(
            "cortexa-codex-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        std::fs::create_dir(&directory)
            .map_err(|_| rejection_error("predicate_071", DirectError::CodexSetup))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))
                .map_err(|_| rejection_error("predicate_072", DirectError::CodexSetup))?;
        }
        let mut children = CHILDREN
            .lock()
            .map_err(|_| rejection_error("predicate_073", DirectError::Internal))?;
        children.retain(|child| child.strong_count() > 0);
        if SHUTTING_DOWN.load(Ordering::SeqCst) || children.len() >= 8 {
            let _ = std::fs::remove_dir(&directory);
            return Err(rejection_error("predicate_003", DirectError::Busy));
        }
        // Retain the verified descriptor through spawn; do not spend the host
        // connection deadline rehashing the complete executable here.
        if let Some(verified) = &verified {
            if let Err(error) = verified.verify(&setup.executable) {
                let _ = std::fs::remove_dir(&directory);
                return Err(error);
            }
        }
        let child = command(setup, &directory).spawn().map_err(|_| {
            let _ = std::fs::remove_dir(&directory);
            DirectError::CodexSetup
        })?;
        let (tx, input) = mpsc::sync_channel::<Vec<u8>>(4);
        let (output, rx) = mpsc::sync_channel(64);
        let child = Arc::new(Mutex::new(child));
        children.push(Arc::downgrade(&child));
        drop(children);
        let mut session = Self {
            child,
            tx: Some(tx),
            rx,
            readers: Vec::new(),
            directory,
            frames: 0,
            derived: setup.derived,
            home: setup.home.clone(),
            policy: DerivedState::default(),
            deferred: std::collections::VecDeque::new(),
            deferred_bytes: 0,
            diagnostic: crate::diagnostics::current(),
            capture: Arc::new(Mutex::new(Capture::default())),
            cleaned: false,
            tail_mode: TailMode::Strict,
            cancellation: setup.cancellation_handle(),
            abort_stream: AbortStream::default(),
        };
        if let Some(o) = &session.diagnostic {
            o.record(
                crate::diagnostics::Event::RuntimeStarted,
                crate::diagnostics::Outcome::Observed,
                None,
            );
        }
        let (mut stdin, stdout) = {
            let mut child = session
                .child
                .lock()
                .map_err(|_| rejection_error("predicate_074", DirectError::Internal))?;
            (
                child
                    .stdin
                    .take()
                    .ok_or_else(|| rejection_error("predicate_086", DirectError::Internal))?,
                child
                    .stdout
                    .take()
                    .ok_or_else(|| rejection_error("predicate_087", DirectError::Internal))?,
            )
        };
        session.readers.push(std::thread::spawn(move || {
            while let Ok(bytes) = input.recv() {
                if stdin
                    .write_all(&bytes)
                    .and_then(|()| stdin.flush())
                    .is_err()
                {
                    break;
                }
            }
        }));
        let capture = Arc::clone(&session.capture);
        let derived = session.derived;
        session.readers.push(std::thread::spawn(move || {
            read_frames(BufReader::new(stdout), &output, &capture, derived);
        }));
        Ok(session)
    }
    fn send(&self, value: Value) -> Result<(), DirectError> {
        let mut bytes = serde_json::to_vec(&value)
            .map_err(|_| rejection_error("predicate_076", DirectError::Protocol))?;
        if bytes.len() > 65_536 {
            return Err(rejection_error("predicate_005", DirectError::Limit));
        }
        bytes.push(b'\n');
        self.tx
            .as_ref()
            .ok_or_else(|| rejection_error("predicate_088", DirectError::Internal))?
            .try_send(bytes)
            .map_err(|_| rejection_error("predicate_077", DirectError::Protocol))
    }
    async fn next(&mut self) -> Result<Value, DirectError> {
        if let Some((value, size)) = self.deferred.pop_front() {
            self.deferred_bytes -= size;
            return Ok(value);
        }
        self.receive().await
    }
    async fn receive(&mut self) -> Result<Value, DirectError> {
        loop {
            if self.derived {
                if let Some(error) = self
                    .capture
                    .lock()
                    .map_err(|_| DirectError::Internal)?
                    .failure
                {
                    return Err(rejection_error("reader_result", error));
                }
            }
            match self.rx.try_recv() {
                Ok(value) => {
                    self.frames += 1;
                    if self.frames > MAX_FRAMES {
                        return Err(rejection_error("predicate_006", DirectError::Limit));
                    }
                    let value = value.map_err(|e| rejection_error("reader_result", e))?;
                    if !value.is_object()
                        || (value.get("method").is_some() && value.get("id").is_some())
                    {
                        // Never approve a runtime tool, permission, auth or dynamic-tool request.
                        return Err(rejection_error(
                            "predicate_007",
                            DirectError::CodexIsolation,
                        ));
                    }
                    if self.derived {
                        derived_envelope(&value)?;
                    }
                    return Ok(value);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err(rejection_error(
                        "predicate_008",
                        DirectError::ProviderUnavailable,
                    ))
                }
                Err(mpsc::TryRecvError::Empty) => {
                    tokio::time::sleep(Duration::from_millis(5)).await
                }
            }
        }
    }
    async fn request(
        &mut self,
        id: u64,
        method: &str,
        params: Value,
    ) -> Result<Value, DirectError> {
        if self.derived {
            if method == "thread/start" {
                self.policy.model = Some(identifier_at("identifier_000", &params["model"])?);
            }
            if method == "turn/start" {
                self.policy.awaiting_turn = true;
            }
        }
        self.send(json!({"id":id,"method":method,"params":params}))?;
        loop {
            let value = self
                .receive()
                .await
                .map_err(|e| rejection_error(request_receive_site(method), e))?;
            if self.derived
                && self
                    .policy
                    .handle_warning(&value, "warning_before_response")?
            {
                continue;
            }
            if value.get("id").is_some() {
                if value["id"] != id {
                    return Err(rejection_error("predicate_009", DirectError::Protocol));
                }
                if value.get("error").is_some() {
                    if self.derived {
                        closed_at(
                            &REJECTION_SCHEMAS[0],
                            &value["error"],
                            &["code", "message"],
                            &["data"],
                        )?;
                        if value["error"]["code"].as_i64().is_none()
                            || !value["error"]["message"].is_string()
                        {
                            return Err(rejection_error("predicate_010", DirectError::Protocol));
                        }
                    }
                    return Err(if self.derived && method == "account/read" {
                        DirectError::CodexAuthentication
                    } else {
                        DirectError::CodexRuntime
                    });
                }
                let result = value
                    .get("result")
                    .cloned()
                    .ok_or_else(|| rejection_error("predicate_089", DirectError::Protocol))?;
                if self.derived && method == "thread/start" {
                    let thread = derived_thread_response(
                        &result,
                        self.policy.model.as_deref().ok_or_else(|| {
                            rejection_error("predicate_090", DirectError::Protocol)
                        })?,
                    )?;
                    if self.policy.thread.as_ref().is_some_and(|v| *v != thread) {
                        return Err(rejection_error("predicate_011", DirectError::Protocol));
                    }
                    self.policy.thread = Some(thread);
                    self.policy.deny_all_verified = true;
                }
                if self.derived && method == "turn/start" {
                    closed_at(&REJECTION_SCHEMAS[1], &result, &["turn"], &[])?;
                    let turn = identifier_at("identifier_001", &result["turn"]["id"])?;
                    if self.policy.turn.as_ref().is_some_and(|v| *v != turn) {
                        return Err(rejection_error("predicate_012", DirectError::Protocol));
                    }
                    self.policy.turn = Some(turn);
                    self.policy.awaiting_turn = false;
                }
                return Ok(result);
            }
            reject_tool(&value)?;
            if self.derived && !self.policy.benign(&value)? {
                if method != "turn/start" {
                    return Err(reject_notification(
                        &value,
                        DirectError::CodexIsolation,
                        "notification_phase_before_response",
                    ));
                }
                self.policy
                    .stream_at(&value, "notification_phase_before_response")?;
                let size = serde_json::to_vec(&value)
                    .map_err(|_| rejection_error("predicate_078", DirectError::Protocol))?
                    .len();
                self.deferred_bytes += size;
                if self.deferred.len() >= 64 || self.deferred_bytes > MAX_FRAME {
                    return Err(rejection_error("predicate_014", DirectError::Limit));
                }
                self.deferred.push_back((value, size));
            }
        }
    }
    async fn initialize(&mut self) -> Result<(), DirectError> {
        let value = self.request(1,"initialize",json!({"clientInfo":{"name":"cortexa","version":"0.1.0"},"capabilities":{"experimentalApi":true}})).await?;
        if self.derived {
            derived_identity(&value, &self.home)?;
        } else {
            validate_runtime_identity(&value)?;
        }
        self.send(json!({"method":"initialized"}))
    }
    async fn authenticate(&mut self) -> Result<(), DirectError> {
        let value = self
            .request(2, "account/read", json!({"refreshToken":false}))
            .await?;
        if self.derived {
            derived_account(&value)?;
            self.policy.authenticated = true;
        } else if !matches!(
            value["account"]["type"].as_str(),
            Some("chatgpt" | "apiKey")
        ) {
            return Err(rejection_error(
                "predicate_015",
                DirectError::CodexAuthentication,
            ));
        }
        Ok(())
    }
}
// Only explicitly reviewed protocol versions; never accept an arbitrary newer runtime.
// The installed 0.160.1 handshake/schema were checked independently of authentication.
fn validate_runtime_identity(value: &Value) -> Result<(), DirectError> {
    let identity = value["userAgent"]
        .as_str()
        .filter(|v| v.len() <= 512 && !v.chars().any(char::is_control))
        .and_then(|v| v.split_once(' '))
        .map(|(product, _)| product);
    if matches!(identity, Some("cortexa/0.159.0" | "cortexa/0.160.1")) {
        Ok(())
    } else {
        Err(rejection_error("predicate_016", DirectError::CodexSetup))
    }
}

// This exact derived artifact, not the upstream version string alone, admits the
// experimental immutable deny-all interface. Other alpha builds remain rejected.
const DERIVED_SHA256: &str = "11169da653f2ef007982f58a26c66e67b21bb49dc7bb59fd454e5dac32842905";
const DERIVED_BYTES: u64 = 395_308_288;
const DERIVED_FEATURES: &[&str] = &[
    "code_mode",
    "code_mode_only",
    "code_mode_tool_search",
    "stable_environment_tools",
    "deferred_executor",
    "sleep_tool",
    "current_time_reminder",
    "token_budget",
    "request_permissions_tool",
    "send_message_to_user_async",
    "multi_agent_v2",
    "multi_agent_v2_dynamic_tools",
];

// A request-local descriptor witness, never a persistent digest cache.
#[derive(Clone, PartialEq, Eq)]
struct ArtifactIdentity {
    bytes: u64,
    #[cfg(unix)]
    unix: [u64; 10],
    #[cfg(not(unix))]
    modified: Option<std::time::SystemTime>,
}
impl ArtifactIdentity {
    fn read(metadata: &std::fs::Metadata) -> Result<Self, DirectError> {
        if !metadata.is_file() {
            return Err(DirectError::CodexSetup);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if metadata.nlink() != 1 {
                return Err(DirectError::CodexSetup);
            }
            Ok(Self {
                bytes: metadata.len(),
                unix: [
                    metadata.dev(),
                    metadata.ino(),
                    u64::from(metadata.mode()),
                    u64::from(metadata.uid()),
                    u64::from(metadata.gid()),
                    metadata.nlink(),
                    metadata.mtime() as u64,
                    metadata.mtime_nsec() as u64,
                    metadata.ctime() as u64,
                    metadata.ctime_nsec() as u64,
                ],
            })
        }
        #[cfg(not(unix))]
        {
            Ok(Self {
                bytes: metadata.len(),
                modified: metadata.modified().ok(),
            })
        }
    }
}
struct VerifiedArtifact {
    file: std::fs::File,
    identity: ArtifactIdentity,
}
impl VerifiedArtifact {
    fn verify(&self, path: &std::path::Path) -> Result<(), DirectError> {
        let descriptor = self.file.metadata().map_err(|_| DirectError::CodexSetup)?;
        let named = std::fs::symlink_metadata(path).map_err(|_| DirectError::CodexSetup)?;
        if ArtifactIdentity::read(&descriptor)? != self.identity
            || ArtifactIdentity::read(&named)? != self.identity
        {
            return Err(rejection_error("predicate_019", DirectError::CodexSetup));
        }
        Ok(())
    }
}
#[cfg(test)]
type ArtifactTestHook = Box<dyn FnOnce() -> Result<(), DirectError>>;
#[cfg(test)]
thread_local! {
    static ARTIFACT_HASH_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ARTIFACT_AFTER_FIRST: std::cell::RefCell<Option<ArtifactTestHook>> = const { std::cell::RefCell::new(None) };
}
fn prepare_derived_artifact(
    path: &std::path::Path,
    expected_bytes: u64,
    expected_hash: &str,
) -> Result<Option<VerifiedArtifact>, DirectError> {
    let Some(first) = verify_artifact(path, expected_bytes, expected_hash)? else {
        return Ok(None);
    };
    let first_identity = first.identity.clone();
    drop(first);
    #[cfg(test)]
    if let Some(hook) = ARTIFACT_AFTER_FIRST.with(|slot| slot.borrow_mut().take()) {
        hook()?;
    }
    // Preserve the second complete verification, before the presentation host
    // clock starts. Session::spawn consumes and revalidates this exact object.
    let second = verify_artifact(path, expected_bytes, expected_hash)?
        .ok_or_else(|| rejection_error("predicate_002", DirectError::CodexSetup))?;
    if second.identity != first_identity {
        return Err(rejection_error("predicate_019", DirectError::CodexSetup));
    }
    Ok(Some(second))
}
#[cfg(test)]
fn derived_artifact(path: &std::path::Path) -> Result<bool, DirectError> {
    Ok(verify_artifact(path, DERIVED_BYTES, DERIVED_SHA256)?.is_some())
}
fn verify_artifact(
    path: &std::path::Path,
    expected_bytes: u64,
    expected_hash: &str,
) -> Result<Option<VerifiedArtifact>, DirectError> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let before = std::fs::symlink_metadata(path)
        .map_err(|_| rejection_error("predicate_079", DirectError::CodexSetup))?;
    if !before.is_file() || before.len() != expected_bytes {
        return Ok(None);
    }
    let before = ArtifactIdentity::read(&before)?;
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
    }
    let mut file = options
        .open(path)
        .map_err(|_| rejection_error("predicate_080", DirectError::CodexSetup))?;
    let opened = ArtifactIdentity::read(
        &file
            .metadata()
            .map_err(|_| rejection_error("predicate_081", DirectError::CodexSetup))?,
    )?;
    if before != opened {
        return Err(rejection_error("predicate_017", DirectError::CodexSetup));
    }
    #[cfg(test)]
    ARTIFACT_HASH_PASSES.with(|count| count.set(count.get() + 1));
    let mut hash = Sha256::new();
    let mut bytes = 0_u64;
    let mut buffer = [0_u8; 65_536];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|_| rejection_error("predicate_082", DirectError::CodexSetup))?;
        if count == 0 {
            break;
        }
        bytes += count as u64;
        if bytes > expected_bytes {
            return Err(rejection_error("predicate_018", DirectError::CodexSetup));
        }
        hash.update(&buffer[..count]);
    }
    let verified = VerifiedArtifact {
        file,
        identity: opened,
    };
    verified.verify(path)?;
    if bytes != expected_bytes {
        return Err(rejection_error("predicate_019", DirectError::CodexSetup));
    }
    if format!("{:x}", hash.finalize()) != expected_hash {
        return Ok(None);
    }
    Ok(Some(verified))
}

fn closed(value: &Value, required: &[&str], optional: &[&str]) -> Result<(), DirectError> {
    let object = value.as_object().ok_or(DirectError::Protocol)?;
    if required.iter().any(|key| !object.contains_key(*key))
        || object
            .keys()
            .any(|key| !required.contains(&key.as_str()) && !optional.contains(&key.as_str()))
    {
        return Err(DirectError::Protocol);
    }
    Ok(())
}
fn bounded_frame(value: &Value, depth: usize, count: &mut usize) -> Result<(), DirectError> {
    *count += 1;
    if depth > 32 || *count > 32_768 || value.as_str().is_some_and(|s| s.len() > 65_536) {
        return Err(rejection_error("predicate_020", DirectError::Limit));
    }
    match value {
        Value::Array(values) => {
            for child in values {
                bounded_frame(child, depth + 1, count)?;
            }
        }
        Value::Object(values) => {
            for child in values.values() {
                bounded_frame(child, depth + 1, count)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn derived_envelope(value: &Value) -> Result<(), DirectError> {
    bounded_frame(value, 0, &mut 0)?;
    if value.get("method").is_some() {
        closed_at(
            &REJECTION_SCHEMAS[2],
            value,
            &["method", "params"],
            &["emittedAtMs"],
        )?;
        if !value["method"].is_string()
            || value
                .get("emittedAtMs")
                .is_some_and(|v| !v.is_null() && v.as_u64().is_none())
        {
            return Err(rejection_error("predicate_021", DirectError::Protocol));
        }
    } else {
        closed_at(&REJECTION_SCHEMAS[3], value, &["id"], &["result", "error"])?;
        if value["id"].as_u64().is_none()
            || value.get("result").is_some() == value.get("error").is_some()
        {
            return Err(rejection_error("predicate_022", DirectError::Protocol));
        }
    }
    Ok(())
}
// Exact derived GetAccountResponse schema. Routing metadata is validated only;
// it never chooses a destination, changes policy, or leaves this native boundary.
fn derived_account(value: &Value) -> Result<(), DirectError> {
    bounded_frame(value, 0, &mut 0)?;
    closed_at(
        &REJECTION_SCHEMAS[4],
        value,
        &["requiresOpenaiAuth"],
        &["account", "workspaceRouting"],
    )?;
    if !value["requiresOpenaiAuth"].is_boolean() {
        return Err(rejection_error("predicate_023", DirectError::Protocol));
    }
    if let Some(routing) = value.get("workspaceRouting").filter(|v| !v.is_null()) {
        closed_at(
            &REJECTION_SCHEMAS[5],
            routing,
            &[
                "chatgptAccountId",
                "backendOrigin",
                "accountRoutingOverride",
            ],
            &[],
        )?;
        if !routing["chatgptAccountId"].is_string()
            || !routing["backendOrigin"].is_string()
            || !matches!(
                routing["accountRoutingOverride"].as_str(),
                Some("NO_CONSTRAINT" | "us" | "us_cr")
            )
        {
            return Err(rejection_error("predicate_024", DirectError::Protocol));
        }
    }
    let account = &value["account"];
    if account.is_null() {
        return Err(rejection_error(
            "predicate_025",
            DirectError::CodexAuthentication,
        ));
    }
    match account["type"].as_str() {
        Some("apiKey") => {
            closed_at(&REJECTION_SCHEMAS[6], account, &["type"], &[])?;
            return Err(rejection_error(
                "predicate_026",
                DirectError::CodexAuthentication,
            ));
        }
        Some("chatgpt") => {}
        _ => return Err(rejection_error("predicate_027", DirectError::Protocol)),
    }
    closed_at(
        &REJECTION_SCHEMAS[7],
        account,
        &["type", "email", "planType"],
        &[],
    )?;
    if !(account["email"].is_null() || account["email"].is_string())
        || !matches!(
            account["planType"].as_str(),
            Some(
                "free"
                    | "go"
                    | "plus"
                    | "pro"
                    | "prolite"
                    | "promax"
                    | "team"
                    | "self_serve_business_prolite"
                    | "self_serve_business_usage_based"
                    | "business"
                    | "ent26"
                    | "enterprise_cbp_automation"
                    | "enterprise_cbp_usage_based"
                    | "enterprise"
                    | "edu"
                    | "edu_plus"
                    | "edu_pro"
                    | "unknown"
            )
        )
    {
        return Err(rejection_error("predicate_028", DirectError::Protocol));
    }
    if value["requiresOpenaiAuth"] != true {
        return Err(rejection_error(
            "predicate_029",
            DirectError::CodexAuthentication,
        ));
    }
    Ok(())
}
fn derived_identity(value: &Value, home: &std::path::Path) -> Result<(), DirectError> {
    closed_at(
        &REJECTION_SCHEMAS[8],
        value,
        &["userAgent", "codexHome", "platformOs", "platformFamily"],
        &[],
    )?;
    let agent = value["userAgent"]
        .as_str()
        .ok_or_else(|| rejection_error("predicate_091", DirectError::CodexSetup))?;
    if agent.len() > 512
        || agent.chars().any(char::is_control)
        || !agent.starts_with("cortexa/0.162.0-alpha.17.2 ")
        || value["codexHome"].as_str() != home.to_str()
        || value["platformOs"] != "macos"
        || value["platformFamily"] != "unix"
    {
        return Err(rejection_error("predicate_030", DirectError::CodexSetup));
    }
    Ok(())
}
fn derived_thread(value: &Value, model: &str) -> Result<String, DirectError> {
    closed_at(
        &REJECTION_SCHEMAS[9],
        value,
        &[
            "id",
            "cliVersion",
            "createdAt",
            "cwd",
            "ephemeral",
            "modelProvider",
            "preview",
            "projectId",
            "sessionId",
            "source",
            "status",
            "turns",
            "updatedAt",
        ],
        &[
            "agentNickname",
            "agentRole",
            "canAcceptDirectInput",
            "daybreakEnabled",
            "environments",
            "extra",
            "forkedFromId",
            "gitInfo",
            "historyMode",
            "model",
            "name",
            "originator",
            "parentThreadId",
            "path",
            "reasoningEffort",
            "recencyAt",
            "section",
            "sectionEnteredAt",
            "threadSource",
        ],
    )?;
    if value["ephemeral"] != true
        || value["modelProvider"] != "openai"
        || !value["turns"].as_array().is_some_and(Vec::is_empty)
        || value
            .get("environments")
            .is_some_and(|v| !v.is_null() && *v != json!([]))
        || value
            .get("model")
            .is_some_and(|v| !v.is_null() && *v != model)
    {
        return Err(rejection_error(
            "predicate_031",
            DirectError::CodexIsolation,
        ));
    }
    identifier_at("identifier_002", &value["id"])
}
fn derived_thread_response(value: &Value, model: &str) -> Result<String, DirectError> {
    closed_at(
        &REJECTION_SCHEMAS[10],
        value,
        &[
            "thread",
            "model",
            "modelProvider",
            "approvalPolicy",
            "approvalsReviewer",
            "cwd",
            "sandbox",
            "denyAllTools",
        ],
        &[
            "activePermissionProfile",
            "disabledPluginIds",
            "instructionSources",
            "multiAgentMode",
            "reasoningEffort",
            "runtimeWorkspaceRoots",
            "serviceTier",
        ],
    )?;
    if value["denyAllTools"] != true {
        return Err(rejection_error(
            "predicate_032",
            DirectError::CodexIsolation,
        ));
    }
    let id = thread_id(value, model)?;
    if derived_thread(&value["thread"], model)? != id {
        return Err(rejection_error("predicate_033", DirectError::Protocol));
    }
    Ok(id)
}
fn derived_input(rules: &str, input: &str) -> Result<String, DirectError> {
    let text = format!("{rules}\n\n{input}");
    if text.chars().count() > 16_384 {
        return Err(rejection_error("predicate_034", DirectError::Limit));
    }
    Ok(text)
}

#[derive(Default)]
struct DerivedState {
    deny_all_verified: bool,
    disabled_code_mode_warning_seen: bool,
    model: Option<String>,
    thread: Option<String>,
    turn: Option<String>,
    authenticated: bool,
    awaiting_turn: bool,
    low_effort_verified: bool,
}
impl DerivedState {
    // Only the pinned derived deny-all lane may handle this exact notification.
    // The source emits it before turn work; it neither starts an interpreter nor
    // changes immutable ToolPolicy. Never use it as evidence of tool exclusion.
    fn handle_warning(&mut self, value: &Value, phase: &'static str) -> Result<bool, DirectError> {
        if value["method"] != "warning" {
            return Ok(false);
        }
        if self.authenticated
            && self.deny_all_verified
            && self.model.as_deref() == Some("gpt-5.6-luna")
            && (self.awaiting_turn || self.turn.is_some())
            && !self.disabled_code_mode_warning_seen
            && warning_label(value, self.thread.as_deref())
                == "warning_disabled_code_mode_fail_closed"
        {
            self.disabled_code_mode_warning_seen = true;
            observe_fixed("warning_disabled_code_mode_fail_closed_handled", Ok(()));
            return Ok(true);
        }
        reject_warning(value, self.thread.as_deref(), phase)?;
        Ok(false)
    }
    // Only these reviewed status events may interleave with an awaited response.
    // They never authenticate the client or consume the pending response ID.
    fn benign(&mut self, value: &Value) -> Result<bool, DirectError> {
        let p = &value["params"];
        match value["method"].as_str() {
            Some("remoteControl/status/changed") => {
                closed_at(
                    &REJECTION_SCHEMAS[11],
                    p,
                    &["installationId", "serverName", "status"],
                    &["environmentId"],
                )?;
                if p["status"] != "disabled"
                    || !p["installationId"].is_string()
                    || !p["serverName"].is_string()
                {
                    return Err(rejection_error(
                        "predicate_035",
                        DirectError::CodexIsolation,
                    ));
                }
            }
            Some("account/updated") => {
                closed_at(&REJECTION_SCHEMAS[12], p, &[], &["authMode", "planType"])?;
                if p.get("authMode")
                    .is_some_and(|v| *v != "chatgpt" && (self.authenticated || !v.is_null()))
                {
                    return Err(rejection_error(
                        "predicate_036",
                        DirectError::CodexAuthentication,
                    ));
                }
                if p.get("planType")
                    .is_some_and(|v| !v.is_null() && !v.is_string())
                {
                    return Err(rejection_error("predicate_037", DirectError::Protocol));
                }
            }
            Some("app/list/updated") => {
                closed_at(&REJECTION_SCHEMAS[13], p, &["data"], &[])?;
                if p["data"] != json!([]) {
                    return Err(rejection_error(
                        "predicate_038",
                        DirectError::CodexIsolation,
                    ));
                }
            }
            Some("thread/started") => {
                closed_at(&REJECTION_SCHEMAS[14], p, &["thread"], &[])?;
                let id = derived_thread(
                    &p["thread"],
                    self.model
                        .as_deref()
                        .ok_or_else(|| rejection_error("predicate_092", DirectError::Protocol))?,
                )?;
                if self.thread.as_ref().is_some_and(|v| *v != id) {
                    return Err(rejection_error("predicate_039", DirectError::Protocol));
                }
                self.thread = Some(id);
            }
            Some("thread/status/changed") => {
                closed_at(&REJECTION_SCHEMAS[15], p, &["threadId", "status"], &[])?;
                if Some(identifier_at("identifier_003", &p["threadId"])?) != self.thread {
                    return Err(rejection_error("predicate_040", DirectError::Protocol));
                }
                match p["status"]["type"].as_str() {
                    Some("idle" | "notLoaded") => {
                        closed_at(&REJECTION_SCHEMAS[16], &p["status"], &["type"], &[])?
                    }
                    Some("active") if self.turn.is_some() || self.awaiting_turn => {
                        closed_at(
                            &REJECTION_SCHEMAS[17],
                            &p["status"],
                            &["type", "activeFlags"],
                            &[],
                        )?;
                        if p["status"]["activeFlags"] != json!([]) {
                            return Err(rejection_error(
                                "predicate_041",
                                DirectError::CodexIsolation,
                            ));
                        }
                    }
                    _ => {
                        return Err(rejection_error(
                            "predicate_042",
                            DirectError::CodexIsolation,
                        ))
                    }
                }
            }
            Some("account/rateLimits/updated") => {
                closed_at(&REJECTION_SCHEMAS[18], p, &["rateLimits"], &[])?;
                validate_rate_limits(&p["rateLimits"])?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }
    fn stream(&mut self, value: &Value) -> Result<(), DirectError> {
        self.stream_at(value, "notification_phase_stream")
    }
    fn stream_at(&mut self, value: &Value, phase: &'static str) -> Result<(), DirectError> {
        if self.handle_warning(value, "warning_streaming")? {
            return Ok(());
        }
        if self.benign(value)? {
            return Ok(());
        }
        let p = &value["params"];
        if Some(identifier_at("identifier_004", &p["threadId"])?) != self.thread {
            return Err(rejection_error("predicate_043", DirectError::Protocol));
        }
        let method = value["method"]
            .as_str()
            .ok_or_else(|| rejection_error("predicate_093", DirectError::Protocol))?;
        let turn = if matches!(method, "turn/started" | "turn/completed") {
            identifier_at("identifier_005", &p["turn"]["id"])?
        } else {
            stream_turn_identifier(value)?
        };
        if self.turn.as_ref().is_some_and(|v| *v != turn) {
            return Err(rejection_error("predicate_044", DirectError::Protocol));
        }
        if method == "turn/moderationMetadata"
            && (!self.authenticated
                || !self.deny_all_verified
                || self.model.as_deref() != Some("gpt-5.6-luna")
                || self.turn.as_ref() != Some(&turn)
                || !matches!(
                    phase,
                    "notification_phase_stream" | "notification_phase_before_response"
                ))
        {
            return Err(rejection_error(
                "moderation_metadata_context_rejected",
                DirectError::CodexIsolation,
            ));
        }
        self.turn = Some(turn);
        match method {
            "turn/moderationMetadata" => {
                // Pinned schema declares metadata as arbitrary JSON. Treat it as
                // opaque, bounded transport data; never inspect, log, persist or
                // apply it to model selection, ToolPolicy, context or authority.
                derived_envelope(value)
                    .and_then(|()| closed(p, &["threadId", "turnId", "metadata"], &[]))
                    .map_err(|e| rejection_error("moderation_metadata_schema_rejected", e))?;
                observe_fixed("moderation_metadata_handled", Ok(()));
            }
            "turn/started" | "turn/completed" => {
                closed_at(&REJECTION_SCHEMAS[19], p, &["threadId", "turn"], &[])?;
                closed_at(
                    &REJECTION_SCHEMAS[20],
                    &p["turn"],
                    &["id", "items", "status"],
                    &[
                        "completedAt",
                        "durationMs",
                        "error",
                        "itemsView",
                        "rootTurnId",
                        "startedAt",
                    ],
                )?;
                let items = p["turn"]["items"]
                    .as_array()
                    .ok_or_else(|| rejection_error("predicate_094", DirectError::Protocol))?;
                for item in items {
                    validate_text_item(item)?;
                }
                if method == "turn/started"
                    && (p["turn"]["status"] != "inProgress" || !items.is_empty())
                {
                    return Err(rejection_error(
                        "predicate_045",
                        DirectError::CodexIsolation,
                    ));
                }
            }
            "item/started" | "item/completed" => {
                closed_at(
                    &REJECTION_SCHEMAS[21],
                    p,
                    &[
                        "threadId",
                        "turnId",
                        "item",
                        if method == "item/started" {
                            "startedAtMs"
                        } else {
                            "completedAtMs"
                        },
                    ],
                    &[],
                )?;
                validate_text_item(&p["item"])?;
            }
            "item/agentMessage/delta" => {
                closed_at(
                    &REJECTION_SCHEMAS[22],
                    p,
                    &["threadId", "turnId", "itemId", "delta"],
                    &[],
                )?;
                identifier_at("identifier_007", &p["itemId"])?;
                if !p["delta"].is_string() {
                    return Err(rejection_error("predicate_046", DirectError::Protocol));
                }
            }
            "item/reasoning/summaryTextDelta"
            | "item/reasoning/textDelta"
            | "item/reasoning/summaryPartAdded" => {
                let index = if method == "item/reasoning/textDelta" {
                    "contentIndex"
                } else {
                    "summaryIndex"
                };
                let mut required = vec!["threadId", "turnId", "itemId", index];
                if method != "item/reasoning/summaryPartAdded" {
                    required.push("delta");
                }
                closed_at(&REJECTION_SCHEMAS[23], p, &required, &[])?;
                identifier_at("identifier_008", &p["itemId"])?;
                if p[index].as_u64().is_none()
                    || (method != "item/reasoning/summaryPartAdded" && !p["delta"].is_string())
                {
                    return Err(rejection_error("predicate_047", DirectError::Protocol));
                }
            }
            "thread/tokenUsage/updated" => {
                closed_at(
                    &REJECTION_SCHEMAS[24],
                    p,
                    &["threadId", "turnId", "tokenUsage"],
                    &[],
                )?;
                closed_at(
                    &REJECTION_SCHEMAS[25],
                    &p["tokenUsage"],
                    &["total", "last"],
                    &["modelContextWindow"],
                )?;
                for key in ["total", "last"] {
                    let b = &p["tokenUsage"][key];
                    closed_at(
                        &REJECTION_SCHEMAS[26],
                        b,
                        &[
                            "inputTokens",
                            "cachedInputTokens",
                            "outputTokens",
                            "reasoningOutputTokens",
                            "totalTokens",
                        ],
                        &["cacheWriteInputTokens"],
                    )?;
                    if b.as_object()
                        .ok_or_else(|| rejection_error("predicate_095", DirectError::Protocol))?
                        .values()
                        .any(|v| v.as_u64().is_none())
                    {
                        return Err(rejection_error("predicate_048", DirectError::Protocol));
                    }
                }
            }
            "error" => return Err(rejection_error("predicate_049", DirectError::CodexRuntime)),
            _ => {
                return Err(reject_notification(
                    value,
                    DirectError::CodexIsolation,
                    phase,
                ))
            }
        }
        Ok(())
    }
}
fn validate_rate_limits(value: &Value) -> Result<(), DirectError> {
    closed_at(
        &REJECTION_SCHEMAS[27],
        value,
        &[],
        &[
            "credits",
            "individualLimit",
            "limitId",
            "limitName",
            "normalModelSlug",
            "planType",
            "primary",
            "secondary",
            "rateLimitReachedType",
            "spendControlReached",
        ],
    )?;
    for key in ["primary", "secondary"] {
        if let Some(v) = value.get(key).filter(|v| !v.is_null()) {
            closed_at(
                &REJECTION_SCHEMAS[28],
                v,
                &["usedPercent"],
                &["resetsAt", "windowDurationMins"],
            )?;
            if v["usedPercent"].as_i64().is_none()
                || v.as_object()
                    .ok_or_else(|| rejection_error("predicate_096", DirectError::Protocol))?
                    .values()
                    .any(|v| !v.is_null() && v.as_i64().is_none())
            {
                return Err(rejection_error("predicate_051", DirectError::Protocol));
            }
        }
    }
    Ok(())
}
fn validate_text_item(item: &Value) -> Result<(), DirectError> {
    identifier_at("identifier_009", &item["id"])?;
    match item["type"].as_str() {
        Some("agentMessage") => {
            closed_at(
                &REJECTION_SCHEMAS[29],
                item,
                &["id", "type", "text"],
                &["delivery", "memoryCitation", "phase", "questions"],
            )?;
            if !item["text"].is_string()
                || item
                    .get("questions")
                    .is_some_and(|v| !v.is_null() && *v != json!([]))
            {
                return Err(rejection_error(
                    "predicate_052",
                    DirectError::CodexIsolation,
                ));
            }
        }
        Some("userMessage") => {
            closed_at(
                &REJECTION_SCHEMAS[30],
                item,
                &["id", "type", "content"],
                &["clientId"],
            )?;
            for v in item["content"]
                .as_array()
                .ok_or_else(|| rejection_error("predicate_097", DirectError::Protocol))?
            {
                closed_at(
                    &REJECTION_SCHEMAS[31],
                    v,
                    &["type", "text"],
                    &["text_elements"],
                )?;
                if v["type"] != "text" || !v["text"].is_string() {
                    return Err(rejection_error(
                        "predicate_053",
                        DirectError::CodexIsolation,
                    ));
                }
            }
        }
        Some("reasoning") => {
            closed_at(
                &REJECTION_SCHEMAS[32],
                item,
                &["id", "type"],
                &["content", "summary"],
            )?;
            for key in ["content", "summary"] {
                if let Some(v) = item.get(key) {
                    if !v.as_array().is_some_and(|a| a.iter().all(Value::is_string)) {
                        return Err(rejection_error("predicate_054", DirectError::Protocol));
                    }
                }
            }
        }
        _ => {
            return Err(rejection_error(
                "predicate_055",
                DirectError::CodexIsolation,
            ))
        }
    }
    Ok(())
}

const MAX_CATALOG_PAGES: u64 = 4;
const MAX_CATALOG_ROWS: usize = 128;

// Opaque cursors stay native and ephemeral; never log them or accept unbounded pages.
fn append_catalog_page(
    value: &Value,
    rows: &mut Vec<Value>,
    cursors: &mut Vec<String>,
) -> Result<Option<String>, DirectError> {
    let page = value["data"]
        .as_array()
        .ok_or_else(|| rejection_error("predicate_098", DirectError::Catalog))?;
    if page.len() > MAX_CATALOG_ROWS || rows.len() + page.len() > MAX_CATALOG_ROWS {
        return Err(rejection_error("predicate_056", DirectError::Catalog));
    }
    let next = match value.get("nextCursor") {
        Some(Value::Null) => None,
        Some(Value::String(cursor))
            if !cursor.is_empty()
                && cursor.len() <= 256
                && !cursor.chars().any(char::is_control)
                && !cursors.contains(cursor) =>
        {
            Some(cursor.clone())
        }
        _ => return Err(rejection_error("predicate_057", DirectError::Catalog)),
    };
    if let Some(cursor) = &next {
        cursors.push(cursor.clone());
    }
    rows.extend(page.iter().cloned());
    Ok(next)
}

impl Session {
    async fn catalog(&mut self) -> Result<Vec<ModelInfo>, DirectError> {
        let mut rows = Vec::new();
        let mut cursors = Vec::new();
        let mut cursor = None;
        for page in 0..MAX_CATALOG_PAGES {
            let mut params = json!({"limit":128,"includeHidden":false});
            if let Some(cursor) = cursor {
                params["cursor"] = Value::String(cursor);
            }
            let value = self.request(3 + page, "model/list", params).await?;
            cursor = append_catalog_page(&value, &mut rows, &mut cursors)?;
            if cursor.is_none() {
                return models(&json!({"data":rows,"nextCursor":null}));
            }
        }
        Err(rejection_error("predicate_058", DirectError::Catalog))
    }
}

fn reject_tool(value: &Value) -> Result<(), DirectError> {
    if matches!(
        value["method"].as_str(),
        Some("item/started" | "item/completed")
    ) && !matches!(
        value["params"]["item"]["type"].as_str(),
        Some("userMessage" | "agentMessage" | "reasoning")
    ) {
        return Err(rejection_error(
            "predicate_059",
            DirectError::CodexIsolation,
        ));
    }
    Ok(())
}
fn thread_parameters(model: &str) -> Value {
    json!({"model":model,"modelProvider":"openai","approvalPolicy":"never","sandbox":"read-only",
        "ephemeral":true,"environments":[],"dynamicTools":[],"runtimeWorkspaceRoots":[],"selectedCapabilityRoots":[],
        "baseInstructions":crate::agent_chat::COMMUNICATION_RULES,
        "allowProviderModelFallback":false})
}
fn thread_id(value: &Value, model: &str) -> Result<String, DirectError> {
    if value["model"] != model
        || value["modelProvider"] != "openai"
        || value["approvalPolicy"] != "never"
        || value["sandbox"]["type"] != "readOnly"
        || value["sandbox"]["networkAccess"] != false
        || value["runtimeWorkspaceRoots"] != json!([])
        || value["instructionSources"] != json!([])
    {
        return Err(rejection_error(
            "predicate_060",
            DirectError::CodexIsolation,
        ));
    }
    identifier_at("identifier_010", &value["thread"]["id"])
}
fn identifier(value: &Value) -> Result<String, DirectError> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
        .map(str::to_owned)
        .ok_or(DirectError::Protocol)
}
fn models(value: &Value) -> Result<Vec<ModelInfo>, DirectError> {
    if !value["nextCursor"].is_null() {
        return Err(rejection_error("predicate_061", DirectError::Catalog));
    }
    let entries = value["data"]
        .as_array()
        .filter(|a| a.len() <= 128)
        .ok_or_else(|| rejection_error("predicate_099", DirectError::Catalog))?;
    let mut models = Vec::new();
    for entry in entries {
        if entry["hidden"] == true {
            continue;
        }
        let id = identifier_at("identifier_011", &entry["model"])?;
        if models.iter().any(|m: &ModelInfo| m.id == id) {
            return Err(rejection_error("predicate_062", DirectError::Catalog));
        }
        let mut model = ModelInfo::unknown(id);
        let efforts = entry["supportedReasoningEfforts"]
            .as_array()
            .ok_or_else(|| rejection_error("predicate_100", DirectError::Catalog))?;
        for effort in efforts {
            if let Ok(effort) =
                serde_json::from_value::<ReasoningEffort>(effort["reasoningEffort"].clone())
            {
                if !model.efforts.contains(&effort) {
                    model.efforts.push(effort);
                }
            }
        }
        model.provenance =
            Some("Installed Codex model/list; access requires owner authentication".into());
        models.push(model);
    }
    if models.is_empty() {
        return Err(rejection_error("predicate_063", DirectError::Catalog));
    }
    Ok(models)
}
pub(crate) async fn discover() -> Result<Vec<ModelInfo>, DirectError> {
    let attempt = crate::diagnostics::Attempt::new(
        crate::agent_preferences::AgentConnection::Codex,
        "",
        None,
        None,
        None,
    );
    discover_observed(Setup::from_environment, attempt).await
}

// The attempt starts before either executable validation and finishes after the
// owned session is dropped. Cancellation remains cancellation, not inferred auth.
async fn discover_observed(
    setup: impl FnOnce() -> Result<Setup, DirectError>,
    mut attempt: crate::diagnostics::Attempt,
) -> Result<Vec<ModelInfo>, DirectError> {
    use crate::diagnostics::{Event, Outcome, CURRENT};
    let observer = attempt.observer();
    let result = CURRENT
        .scope(observer.clone(), async {
            let setup = setup()?;
            observer.record(Event::ConfigurationValidated, Outcome::Observed, None);
            let mut session = Session::spawn(&setup)?;
            observer.record(Event::StageStarted, Outcome::Observed, None);
            let initialized = session.initialize().await;
            observer.record(
                Event::StageFinished,
                if initialized.is_ok() {
                    Outcome::Completed
                } else {
                    Outcome::Failed
                },
                initialized.as_ref().err().copied().map(Into::into),
            );
            initialized?;
            session.authenticate().await?;
            observer.record(Event::CredentialsAvailable, Outcome::Observed, None);
            let models = session.catalog().await?;
            if session.derived {
                session.finish()?;
            }
            observer.first(false);
            Ok(models)
        })
        .await;
    attempt.finish(result.as_ref().map(|_| ()).map_err(|e| *e));
    result
}
pub(crate) async fn run(
    setup: Setup,
    model: String,
    effort: ReasoningEffort,
    input: String,
    emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    run_with_rules(
        setup,
        model,
        effort,
        input,
        crate::agent_chat::COMMUNICATION_RULES,
        emit,
    )
    .await
}
pub(crate) async fn run_collaboration(
    setup: Setup,
    model: String,
    effort: ReasoningEffort,
    input: String,
    emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    run_with_rules(
        setup,
        model,
        effort,
        input,
        crate::collaboration::RULES,
        emit,
    )
    .await
}
async fn run_with_rules_inner(
    setup: Setup,
    model: String,
    effort: ReasoningEffort,
    input: String,
    rules: &str,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    let mut session = Session::spawn(&setup)?;
    session.initialize().await?;
    session.authenticate().await?;
    if let Some(observer) = crate::diagnostics::current() {
        observer.record(
            crate::diagnostics::Event::CredentialsAvailable,
            crate::diagnostics::Outcome::Observed,
            None,
        );
    }
    let catalog = session.catalog().await?;
    crate::agent_models::validate_selection(
        catalog
            .iter()
            .find(|m| m.id == model)
            .ok_or_else(|| rejection_error("predicate_101", DirectError::ModelUnavailable))?,
        effort,
    )?;
    let mut parameters = thread_parameters(&model);
    let input = if session.derived {
        if model != "gpt-5.6-luna" || effort != ReasoningEffort::Low {
            return Err(rejection_error("predicate_064", DirectError::Unsupported));
        }
        parameters["baseInstructions"] = json!("");
        parameters["developerInstructions"] = json!("");
        parameters["denyAllTools"] = json!(true);
        parameters["config"] = json!({"model_reasoning_effort":"low"});
        derived_input(rules, &input)?
    } else {
        parameters["baseInstructions"] = json!(rules);
        input
    };
    let thread = thread_id(
        &session.request(10, "thread/start", parameters).await?,
        &model,
    )?;
    let mut params =
        json!({"threadId":thread,"environments":[],"input":[{"type":"text","text":input}]});
    if effort != ReasoningEffort::Default {
        params["effort"] = json!(effort);
    }
    let response = session.request(11, "turn/start", params).await?;
    let turn = identifier_at("identifier_012", &response["turn"]["id"])?;
    session.policy.low_effort_verified =
        if session.derived && model == "gpt-5.6-luna" && effort == ReasoningEffort::Low {
            let checked = abort_turn_fields_checked(&response["turn"], false);
            match checked {
                Ok(()) => observe_fixed("abort_turn_fields_verified", Ok(())),
                Err((error, label)) => observe_fixed(label, Err(error)),
            }
            checked.is_ok()
        } else {
            if session.derived {
                observe_fixed(
                    "abort_turn_context_rejected",
                    Err(DirectError::CodexIsolation),
                );
            }
            false
        };
    let mut owner_stopped = false;
    if session.derived {
        let eligible =
            observe_abort_context(abort_context_checks(&session.policy, &session.abort_stream));
        observe_fixed(
            if eligible {
                "abort_eligibility_verified"
            } else {
                "abort_eligibility_rejected"
            },
            if eligible {
                Ok(())
            } else {
                Err(DirectError::CodexIsolation)
            },
        );
        if !eligible {
            session.cancellation.reject();
            return Err(DirectError::CodexIsolation);
        }
        owner_stopped = !session.cancellation.arm()?;
    }
    let mut local_limit = false;
    let result = async {
        if owner_stopped {
            return Err(DirectError::InvalidRequest);
        }
        if let Err(error) = emit(ProviderEvent::Started("resp_cortexa_codex".into())) {
            owner_stopped = error == DirectError::InvalidRequest
                && session.cancellation.take_stopping_callback();
            local_limit = error == DirectError::Limit;
            return Err(error);
        }
        let mut output = String::new();
        loop {
            if session.cancellation.active_requested() {
                owner_stopped = true;
                return Err(DirectError::InvalidRequest);
            }
            let value = session
                .next()
                .await
                .map_err(|e| rejection_error("stream_receive", e))?;
            reject_tool(&value)?;
            if session.derived {
                session.policy.stream(&value)?;
                if session.abort_stream.observe(&value).is_err() {
                    session.abort_stream.poisoned = true;
                }
            }
            let method = value["method"]
                .as_str()
                .ok_or_else(|| rejection_error("predicate_102", DirectError::Protocol))?;
            if matches!(
                method,
                "item/agentMessage/delta" | "turn/completed" | "error"
            ) {
                if value["params"]["threadId"] != thread {
                    return Err(rejection_error("predicate_065", DirectError::Protocol));
                }
                if method == "error" {
                    return Err(rejection_error("predicate_066", DirectError::CodexRuntime));
                }
                if method == "turn/completed" {
                    if value["params"]["turn"]["id"] != turn
                        || value["params"]["turn"]["status"] != "completed"
                    {
                        return Err(rejection_error("predicate_067", DirectError::CodexRuntime));
                    }
                    if output.is_empty() {
                        return Err(rejection_error("predicate_068", DirectError::Incomplete));
                    }
                    // The owner signal and validated completion arbitrate before cleanup.
                    if session.derived && !session.cancellation.complete()? {
                        owner_stopped = true;
                        return Err(DirectError::InvalidRequest);
                    }
                    // Kill/reap and join before exposing completion or releasing ownership.
                    if session.derived {
                        session.tail_mode = TailMode::Strict;
                        session.finish()?;
                    } else {
                        let _ = session.finish();
                    }
                    return emit(ProviderEvent::Completed);
                }
                if value["params"]["turnId"] != turn {
                    return Err(rejection_error("predicate_069", DirectError::Protocol));
                }
                let delta = value["params"]["delta"]
                    .as_str()
                    .ok_or_else(|| rejection_error("predicate_103", DirectError::Protocol))?;
                output.push_str(delta);
                if output.chars().count() > 8192 {
                    local_limit = true;
                    return Err(rejection_error("predicate_070", DirectError::Limit));
                }
                if !delta.is_empty() {
                    if let Err(error) = emit(ProviderEvent::Delta(delta.into())) {
                        owner_stopped = error == DirectError::InvalidRequest
                            && session.cancellation.take_stopping_callback();
                        local_limit = error == DirectError::Limit;
                        return Err(error);
                    }
                }
            }
        }
    }
    .await;
    if owner_stopped {
        // Only explicit owner origin reaches this branch; generic errors stay strict.
        session.tail_mode = TailMode::Strict;
        session.finish()?;
        session.cancellation.local_return();
    } else {
        session.tail_mode = returned_tail_mode(local_limit, &result);
        if result.is_err() && session.tail_mode == TailMode::Strict {
            session.cancellation.reject();
        }
    }
    result
}

// Only reviewed static sites are retained; rejected values never enter diagnostics.
tokio::task_local! { static REJECTION_RECORDED: std::cell::Cell<bool>; }
struct RejectionSchema {
    shape: &'static str,
    unknown: &'static str,
    missing: &'static [&'static str],
}
const REJECTION_FIXED: &[&str] = &[
    "predicate_000",
    "predicate_001",
    "predicate_002",
    "predicate_003",
    "predicate_004",
    "predicate_005",
    "predicate_006",
    "predicate_007",
    "predicate_008",
    "predicate_009",
    "predicate_010",
    "predicate_011",
    "predicate_012",
    "predicate_013",
    "predicate_014",
    "predicate_015",
    "predicate_016",
    "predicate_017",
    "predicate_018",
    "predicate_019",
    "predicate_020",
    "predicate_021",
    "predicate_022",
    "predicate_023",
    "predicate_024",
    "predicate_025",
    "predicate_026",
    "predicate_027",
    "predicate_028",
    "predicate_029",
    "predicate_030",
    "predicate_031",
    "predicate_032",
    "predicate_033",
    "predicate_034",
    "predicate_035",
    "predicate_036",
    "predicate_037",
    "predicate_038",
    "predicate_039",
    "predicate_040",
    "predicate_041",
    "predicate_042",
    "predicate_043",
    "predicate_044",
    "predicate_045",
    "predicate_046",
    "predicate_047",
    "predicate_048",
    "predicate_049",
    "predicate_050",
    "predicate_051",
    "predicate_052",
    "predicate_053",
    "predicate_054",
    "predicate_055",
    "predicate_056",
    "predicate_057",
    "predicate_058",
    "predicate_059",
    "predicate_060",
    "predicate_061",
    "predicate_062",
    "predicate_063",
    "predicate_064",
    "predicate_065",
    "predicate_066",
    "predicate_067",
    "predicate_068",
    "predicate_069",
    "predicate_070",
    "predicate_071",
    "predicate_072",
    "predicate_073",
    "predicate_074",
    "predicate_075",
    "predicate_076",
    "predicate_077",
    "predicate_078",
    "predicate_079",
    "predicate_080",
    "predicate_081",
    "predicate_082",
    "predicate_083",
    "predicate_084",
    "predicate_085",
    "predicate_086",
    "predicate_087",
    "predicate_088",
    "predicate_089",
    "predicate_090",
    "predicate_091",
    "predicate_092",
    "predicate_093",
    "predicate_094",
    "predicate_095",
    "predicate_096",
    "predicate_097",
    "predicate_098",
    "predicate_099",
    "predicate_100",
    "predicate_101",
    "predicate_102",
    "predicate_103",
    "identifier_000",
    "identifier_001",
    "identifier_002",
    "identifier_003",
    "identifier_004",
    "identifier_005",
    "identifier_006",
    "identifier_007",
    "identifier_008",
    "identifier_009",
    "identifier_010",
    "identifier_011",
    "identifier_012",
    "reader_result",
    "stream_receive",
    "request_initialize",
    "request_account",
    "request_catalog",
    "request_thread",
    "request_turn",
    "unreviewed_site",
    "run_unclassified",
];
const REJECTION_SCHEMAS: &[RejectionSchema] = &[
    RejectionSchema {
        shape: "object_000_shape",
        unknown: "object_000_unknown",
        missing: &[
            "object_000_missing_00",
            "object_000_missing_01",
            "object_000_missing_02",
            "object_000_missing_03",
            "object_000_missing_04",
            "object_000_missing_05",
            "object_000_missing_06",
            "object_000_missing_07",
            "object_000_missing_08",
            "object_000_missing_09",
            "object_000_missing_10",
            "object_000_missing_11",
            "object_000_missing_12",
            "object_000_missing_13",
            "object_000_missing_14",
            "object_000_missing_15",
            "object_000_missing_16",
            "object_000_missing_17",
            "object_000_missing_18",
            "object_000_missing_19",
            "object_000_missing_20",
            "object_000_missing_21",
            "object_000_missing_22",
            "object_000_missing_23",
            "object_000_missing_24",
            "object_000_missing_25",
            "object_000_missing_26",
            "object_000_missing_27",
            "object_000_missing_28",
            "object_000_missing_29",
            "object_000_missing_30",
            "object_000_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_001_shape",
        unknown: "object_001_unknown",
        missing: &[
            "object_001_missing_00",
            "object_001_missing_01",
            "object_001_missing_02",
            "object_001_missing_03",
            "object_001_missing_04",
            "object_001_missing_05",
            "object_001_missing_06",
            "object_001_missing_07",
            "object_001_missing_08",
            "object_001_missing_09",
            "object_001_missing_10",
            "object_001_missing_11",
            "object_001_missing_12",
            "object_001_missing_13",
            "object_001_missing_14",
            "object_001_missing_15",
            "object_001_missing_16",
            "object_001_missing_17",
            "object_001_missing_18",
            "object_001_missing_19",
            "object_001_missing_20",
            "object_001_missing_21",
            "object_001_missing_22",
            "object_001_missing_23",
            "object_001_missing_24",
            "object_001_missing_25",
            "object_001_missing_26",
            "object_001_missing_27",
            "object_001_missing_28",
            "object_001_missing_29",
            "object_001_missing_30",
            "object_001_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_002_shape",
        unknown: "object_002_unknown",
        missing: &[
            "object_002_missing_00",
            "object_002_missing_01",
            "object_002_missing_02",
            "object_002_missing_03",
            "object_002_missing_04",
            "object_002_missing_05",
            "object_002_missing_06",
            "object_002_missing_07",
            "object_002_missing_08",
            "object_002_missing_09",
            "object_002_missing_10",
            "object_002_missing_11",
            "object_002_missing_12",
            "object_002_missing_13",
            "object_002_missing_14",
            "object_002_missing_15",
            "object_002_missing_16",
            "object_002_missing_17",
            "object_002_missing_18",
            "object_002_missing_19",
            "object_002_missing_20",
            "object_002_missing_21",
            "object_002_missing_22",
            "object_002_missing_23",
            "object_002_missing_24",
            "object_002_missing_25",
            "object_002_missing_26",
            "object_002_missing_27",
            "object_002_missing_28",
            "object_002_missing_29",
            "object_002_missing_30",
            "object_002_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_003_shape",
        unknown: "object_003_unknown",
        missing: &[
            "object_003_missing_00",
            "object_003_missing_01",
            "object_003_missing_02",
            "object_003_missing_03",
            "object_003_missing_04",
            "object_003_missing_05",
            "object_003_missing_06",
            "object_003_missing_07",
            "object_003_missing_08",
            "object_003_missing_09",
            "object_003_missing_10",
            "object_003_missing_11",
            "object_003_missing_12",
            "object_003_missing_13",
            "object_003_missing_14",
            "object_003_missing_15",
            "object_003_missing_16",
            "object_003_missing_17",
            "object_003_missing_18",
            "object_003_missing_19",
            "object_003_missing_20",
            "object_003_missing_21",
            "object_003_missing_22",
            "object_003_missing_23",
            "object_003_missing_24",
            "object_003_missing_25",
            "object_003_missing_26",
            "object_003_missing_27",
            "object_003_missing_28",
            "object_003_missing_29",
            "object_003_missing_30",
            "object_003_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_004_shape",
        unknown: "object_004_unknown",
        missing: &[
            "object_004_missing_00",
            "object_004_missing_01",
            "object_004_missing_02",
            "object_004_missing_03",
            "object_004_missing_04",
            "object_004_missing_05",
            "object_004_missing_06",
            "object_004_missing_07",
            "object_004_missing_08",
            "object_004_missing_09",
            "object_004_missing_10",
            "object_004_missing_11",
            "object_004_missing_12",
            "object_004_missing_13",
            "object_004_missing_14",
            "object_004_missing_15",
            "object_004_missing_16",
            "object_004_missing_17",
            "object_004_missing_18",
            "object_004_missing_19",
            "object_004_missing_20",
            "object_004_missing_21",
            "object_004_missing_22",
            "object_004_missing_23",
            "object_004_missing_24",
            "object_004_missing_25",
            "object_004_missing_26",
            "object_004_missing_27",
            "object_004_missing_28",
            "object_004_missing_29",
            "object_004_missing_30",
            "object_004_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_005_shape",
        unknown: "object_005_unknown",
        missing: &[
            "object_005_missing_00",
            "object_005_missing_01",
            "object_005_missing_02",
            "object_005_missing_03",
            "object_005_missing_04",
            "object_005_missing_05",
            "object_005_missing_06",
            "object_005_missing_07",
            "object_005_missing_08",
            "object_005_missing_09",
            "object_005_missing_10",
            "object_005_missing_11",
            "object_005_missing_12",
            "object_005_missing_13",
            "object_005_missing_14",
            "object_005_missing_15",
            "object_005_missing_16",
            "object_005_missing_17",
            "object_005_missing_18",
            "object_005_missing_19",
            "object_005_missing_20",
            "object_005_missing_21",
            "object_005_missing_22",
            "object_005_missing_23",
            "object_005_missing_24",
            "object_005_missing_25",
            "object_005_missing_26",
            "object_005_missing_27",
            "object_005_missing_28",
            "object_005_missing_29",
            "object_005_missing_30",
            "object_005_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_006_shape",
        unknown: "object_006_unknown",
        missing: &[
            "object_006_missing_00",
            "object_006_missing_01",
            "object_006_missing_02",
            "object_006_missing_03",
            "object_006_missing_04",
            "object_006_missing_05",
            "object_006_missing_06",
            "object_006_missing_07",
            "object_006_missing_08",
            "object_006_missing_09",
            "object_006_missing_10",
            "object_006_missing_11",
            "object_006_missing_12",
            "object_006_missing_13",
            "object_006_missing_14",
            "object_006_missing_15",
            "object_006_missing_16",
            "object_006_missing_17",
            "object_006_missing_18",
            "object_006_missing_19",
            "object_006_missing_20",
            "object_006_missing_21",
            "object_006_missing_22",
            "object_006_missing_23",
            "object_006_missing_24",
            "object_006_missing_25",
            "object_006_missing_26",
            "object_006_missing_27",
            "object_006_missing_28",
            "object_006_missing_29",
            "object_006_missing_30",
            "object_006_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_007_shape",
        unknown: "object_007_unknown",
        missing: &[
            "object_007_missing_00",
            "object_007_missing_01",
            "object_007_missing_02",
            "object_007_missing_03",
            "object_007_missing_04",
            "object_007_missing_05",
            "object_007_missing_06",
            "object_007_missing_07",
            "object_007_missing_08",
            "object_007_missing_09",
            "object_007_missing_10",
            "object_007_missing_11",
            "object_007_missing_12",
            "object_007_missing_13",
            "object_007_missing_14",
            "object_007_missing_15",
            "object_007_missing_16",
            "object_007_missing_17",
            "object_007_missing_18",
            "object_007_missing_19",
            "object_007_missing_20",
            "object_007_missing_21",
            "object_007_missing_22",
            "object_007_missing_23",
            "object_007_missing_24",
            "object_007_missing_25",
            "object_007_missing_26",
            "object_007_missing_27",
            "object_007_missing_28",
            "object_007_missing_29",
            "object_007_missing_30",
            "object_007_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_008_shape",
        unknown: "object_008_unknown",
        missing: &[
            "object_008_missing_00",
            "object_008_missing_01",
            "object_008_missing_02",
            "object_008_missing_03",
            "object_008_missing_04",
            "object_008_missing_05",
            "object_008_missing_06",
            "object_008_missing_07",
            "object_008_missing_08",
            "object_008_missing_09",
            "object_008_missing_10",
            "object_008_missing_11",
            "object_008_missing_12",
            "object_008_missing_13",
            "object_008_missing_14",
            "object_008_missing_15",
            "object_008_missing_16",
            "object_008_missing_17",
            "object_008_missing_18",
            "object_008_missing_19",
            "object_008_missing_20",
            "object_008_missing_21",
            "object_008_missing_22",
            "object_008_missing_23",
            "object_008_missing_24",
            "object_008_missing_25",
            "object_008_missing_26",
            "object_008_missing_27",
            "object_008_missing_28",
            "object_008_missing_29",
            "object_008_missing_30",
            "object_008_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_009_shape",
        unknown: "object_009_unknown",
        missing: &[
            "object_009_missing_00",
            "object_009_missing_01",
            "object_009_missing_02",
            "object_009_missing_03",
            "object_009_missing_04",
            "object_009_missing_05",
            "object_009_missing_06",
            "object_009_missing_07",
            "object_009_missing_08",
            "object_009_missing_09",
            "object_009_missing_10",
            "object_009_missing_11",
            "object_009_missing_12",
            "object_009_missing_13",
            "object_009_missing_14",
            "object_009_missing_15",
            "object_009_missing_16",
            "object_009_missing_17",
            "object_009_missing_18",
            "object_009_missing_19",
            "object_009_missing_20",
            "object_009_missing_21",
            "object_009_missing_22",
            "object_009_missing_23",
            "object_009_missing_24",
            "object_009_missing_25",
            "object_009_missing_26",
            "object_009_missing_27",
            "object_009_missing_28",
            "object_009_missing_29",
            "object_009_missing_30",
            "object_009_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_010_shape",
        unknown: "object_010_unknown",
        missing: &[
            "object_010_missing_00",
            "object_010_missing_01",
            "object_010_missing_02",
            "object_010_missing_03",
            "object_010_missing_04",
            "object_010_missing_05",
            "object_010_missing_06",
            "object_010_missing_07",
            "object_010_missing_08",
            "object_010_missing_09",
            "object_010_missing_10",
            "object_010_missing_11",
            "object_010_missing_12",
            "object_010_missing_13",
            "object_010_missing_14",
            "object_010_missing_15",
            "object_010_missing_16",
            "object_010_missing_17",
            "object_010_missing_18",
            "object_010_missing_19",
            "object_010_missing_20",
            "object_010_missing_21",
            "object_010_missing_22",
            "object_010_missing_23",
            "object_010_missing_24",
            "object_010_missing_25",
            "object_010_missing_26",
            "object_010_missing_27",
            "object_010_missing_28",
            "object_010_missing_29",
            "object_010_missing_30",
            "object_010_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_011_shape",
        unknown: "object_011_unknown",
        missing: &[
            "object_011_missing_00",
            "object_011_missing_01",
            "object_011_missing_02",
            "object_011_missing_03",
            "object_011_missing_04",
            "object_011_missing_05",
            "object_011_missing_06",
            "object_011_missing_07",
            "object_011_missing_08",
            "object_011_missing_09",
            "object_011_missing_10",
            "object_011_missing_11",
            "object_011_missing_12",
            "object_011_missing_13",
            "object_011_missing_14",
            "object_011_missing_15",
            "object_011_missing_16",
            "object_011_missing_17",
            "object_011_missing_18",
            "object_011_missing_19",
            "object_011_missing_20",
            "object_011_missing_21",
            "object_011_missing_22",
            "object_011_missing_23",
            "object_011_missing_24",
            "object_011_missing_25",
            "object_011_missing_26",
            "object_011_missing_27",
            "object_011_missing_28",
            "object_011_missing_29",
            "object_011_missing_30",
            "object_011_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_012_shape",
        unknown: "object_012_unknown",
        missing: &[
            "object_012_missing_00",
            "object_012_missing_01",
            "object_012_missing_02",
            "object_012_missing_03",
            "object_012_missing_04",
            "object_012_missing_05",
            "object_012_missing_06",
            "object_012_missing_07",
            "object_012_missing_08",
            "object_012_missing_09",
            "object_012_missing_10",
            "object_012_missing_11",
            "object_012_missing_12",
            "object_012_missing_13",
            "object_012_missing_14",
            "object_012_missing_15",
            "object_012_missing_16",
            "object_012_missing_17",
            "object_012_missing_18",
            "object_012_missing_19",
            "object_012_missing_20",
            "object_012_missing_21",
            "object_012_missing_22",
            "object_012_missing_23",
            "object_012_missing_24",
            "object_012_missing_25",
            "object_012_missing_26",
            "object_012_missing_27",
            "object_012_missing_28",
            "object_012_missing_29",
            "object_012_missing_30",
            "object_012_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_013_shape",
        unknown: "object_013_unknown",
        missing: &[
            "object_013_missing_00",
            "object_013_missing_01",
            "object_013_missing_02",
            "object_013_missing_03",
            "object_013_missing_04",
            "object_013_missing_05",
            "object_013_missing_06",
            "object_013_missing_07",
            "object_013_missing_08",
            "object_013_missing_09",
            "object_013_missing_10",
            "object_013_missing_11",
            "object_013_missing_12",
            "object_013_missing_13",
            "object_013_missing_14",
            "object_013_missing_15",
            "object_013_missing_16",
            "object_013_missing_17",
            "object_013_missing_18",
            "object_013_missing_19",
            "object_013_missing_20",
            "object_013_missing_21",
            "object_013_missing_22",
            "object_013_missing_23",
            "object_013_missing_24",
            "object_013_missing_25",
            "object_013_missing_26",
            "object_013_missing_27",
            "object_013_missing_28",
            "object_013_missing_29",
            "object_013_missing_30",
            "object_013_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_014_shape",
        unknown: "object_014_unknown",
        missing: &[
            "object_014_missing_00",
            "object_014_missing_01",
            "object_014_missing_02",
            "object_014_missing_03",
            "object_014_missing_04",
            "object_014_missing_05",
            "object_014_missing_06",
            "object_014_missing_07",
            "object_014_missing_08",
            "object_014_missing_09",
            "object_014_missing_10",
            "object_014_missing_11",
            "object_014_missing_12",
            "object_014_missing_13",
            "object_014_missing_14",
            "object_014_missing_15",
            "object_014_missing_16",
            "object_014_missing_17",
            "object_014_missing_18",
            "object_014_missing_19",
            "object_014_missing_20",
            "object_014_missing_21",
            "object_014_missing_22",
            "object_014_missing_23",
            "object_014_missing_24",
            "object_014_missing_25",
            "object_014_missing_26",
            "object_014_missing_27",
            "object_014_missing_28",
            "object_014_missing_29",
            "object_014_missing_30",
            "object_014_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_015_shape",
        unknown: "object_015_unknown",
        missing: &[
            "object_015_missing_00",
            "object_015_missing_01",
            "object_015_missing_02",
            "object_015_missing_03",
            "object_015_missing_04",
            "object_015_missing_05",
            "object_015_missing_06",
            "object_015_missing_07",
            "object_015_missing_08",
            "object_015_missing_09",
            "object_015_missing_10",
            "object_015_missing_11",
            "object_015_missing_12",
            "object_015_missing_13",
            "object_015_missing_14",
            "object_015_missing_15",
            "object_015_missing_16",
            "object_015_missing_17",
            "object_015_missing_18",
            "object_015_missing_19",
            "object_015_missing_20",
            "object_015_missing_21",
            "object_015_missing_22",
            "object_015_missing_23",
            "object_015_missing_24",
            "object_015_missing_25",
            "object_015_missing_26",
            "object_015_missing_27",
            "object_015_missing_28",
            "object_015_missing_29",
            "object_015_missing_30",
            "object_015_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_016_shape",
        unknown: "object_016_unknown",
        missing: &[
            "object_016_missing_00",
            "object_016_missing_01",
            "object_016_missing_02",
            "object_016_missing_03",
            "object_016_missing_04",
            "object_016_missing_05",
            "object_016_missing_06",
            "object_016_missing_07",
            "object_016_missing_08",
            "object_016_missing_09",
            "object_016_missing_10",
            "object_016_missing_11",
            "object_016_missing_12",
            "object_016_missing_13",
            "object_016_missing_14",
            "object_016_missing_15",
            "object_016_missing_16",
            "object_016_missing_17",
            "object_016_missing_18",
            "object_016_missing_19",
            "object_016_missing_20",
            "object_016_missing_21",
            "object_016_missing_22",
            "object_016_missing_23",
            "object_016_missing_24",
            "object_016_missing_25",
            "object_016_missing_26",
            "object_016_missing_27",
            "object_016_missing_28",
            "object_016_missing_29",
            "object_016_missing_30",
            "object_016_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_017_shape",
        unknown: "object_017_unknown",
        missing: &[
            "object_017_missing_00",
            "object_017_missing_01",
            "object_017_missing_02",
            "object_017_missing_03",
            "object_017_missing_04",
            "object_017_missing_05",
            "object_017_missing_06",
            "object_017_missing_07",
            "object_017_missing_08",
            "object_017_missing_09",
            "object_017_missing_10",
            "object_017_missing_11",
            "object_017_missing_12",
            "object_017_missing_13",
            "object_017_missing_14",
            "object_017_missing_15",
            "object_017_missing_16",
            "object_017_missing_17",
            "object_017_missing_18",
            "object_017_missing_19",
            "object_017_missing_20",
            "object_017_missing_21",
            "object_017_missing_22",
            "object_017_missing_23",
            "object_017_missing_24",
            "object_017_missing_25",
            "object_017_missing_26",
            "object_017_missing_27",
            "object_017_missing_28",
            "object_017_missing_29",
            "object_017_missing_30",
            "object_017_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_018_shape",
        unknown: "object_018_unknown",
        missing: &[
            "object_018_missing_00",
            "object_018_missing_01",
            "object_018_missing_02",
            "object_018_missing_03",
            "object_018_missing_04",
            "object_018_missing_05",
            "object_018_missing_06",
            "object_018_missing_07",
            "object_018_missing_08",
            "object_018_missing_09",
            "object_018_missing_10",
            "object_018_missing_11",
            "object_018_missing_12",
            "object_018_missing_13",
            "object_018_missing_14",
            "object_018_missing_15",
            "object_018_missing_16",
            "object_018_missing_17",
            "object_018_missing_18",
            "object_018_missing_19",
            "object_018_missing_20",
            "object_018_missing_21",
            "object_018_missing_22",
            "object_018_missing_23",
            "object_018_missing_24",
            "object_018_missing_25",
            "object_018_missing_26",
            "object_018_missing_27",
            "object_018_missing_28",
            "object_018_missing_29",
            "object_018_missing_30",
            "object_018_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_019_shape",
        unknown: "object_019_unknown",
        missing: &[
            "object_019_missing_00",
            "object_019_missing_01",
            "object_019_missing_02",
            "object_019_missing_03",
            "object_019_missing_04",
            "object_019_missing_05",
            "object_019_missing_06",
            "object_019_missing_07",
            "object_019_missing_08",
            "object_019_missing_09",
            "object_019_missing_10",
            "object_019_missing_11",
            "object_019_missing_12",
            "object_019_missing_13",
            "object_019_missing_14",
            "object_019_missing_15",
            "object_019_missing_16",
            "object_019_missing_17",
            "object_019_missing_18",
            "object_019_missing_19",
            "object_019_missing_20",
            "object_019_missing_21",
            "object_019_missing_22",
            "object_019_missing_23",
            "object_019_missing_24",
            "object_019_missing_25",
            "object_019_missing_26",
            "object_019_missing_27",
            "object_019_missing_28",
            "object_019_missing_29",
            "object_019_missing_30",
            "object_019_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_020_shape",
        unknown: "object_020_unknown",
        missing: &[
            "object_020_missing_00",
            "object_020_missing_01",
            "object_020_missing_02",
            "object_020_missing_03",
            "object_020_missing_04",
            "object_020_missing_05",
            "object_020_missing_06",
            "object_020_missing_07",
            "object_020_missing_08",
            "object_020_missing_09",
            "object_020_missing_10",
            "object_020_missing_11",
            "object_020_missing_12",
            "object_020_missing_13",
            "object_020_missing_14",
            "object_020_missing_15",
            "object_020_missing_16",
            "object_020_missing_17",
            "object_020_missing_18",
            "object_020_missing_19",
            "object_020_missing_20",
            "object_020_missing_21",
            "object_020_missing_22",
            "object_020_missing_23",
            "object_020_missing_24",
            "object_020_missing_25",
            "object_020_missing_26",
            "object_020_missing_27",
            "object_020_missing_28",
            "object_020_missing_29",
            "object_020_missing_30",
            "object_020_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_021_shape",
        unknown: "object_021_unknown",
        missing: &[
            "object_021_missing_00",
            "object_021_missing_01",
            "object_021_missing_02",
            "object_021_missing_03",
            "object_021_missing_04",
            "object_021_missing_05",
            "object_021_missing_06",
            "object_021_missing_07",
            "object_021_missing_08",
            "object_021_missing_09",
            "object_021_missing_10",
            "object_021_missing_11",
            "object_021_missing_12",
            "object_021_missing_13",
            "object_021_missing_14",
            "object_021_missing_15",
            "object_021_missing_16",
            "object_021_missing_17",
            "object_021_missing_18",
            "object_021_missing_19",
            "object_021_missing_20",
            "object_021_missing_21",
            "object_021_missing_22",
            "object_021_missing_23",
            "object_021_missing_24",
            "object_021_missing_25",
            "object_021_missing_26",
            "object_021_missing_27",
            "object_021_missing_28",
            "object_021_missing_29",
            "object_021_missing_30",
            "object_021_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_022_shape",
        unknown: "object_022_unknown",
        missing: &[
            "object_022_missing_00",
            "object_022_missing_01",
            "object_022_missing_02",
            "object_022_missing_03",
            "object_022_missing_04",
            "object_022_missing_05",
            "object_022_missing_06",
            "object_022_missing_07",
            "object_022_missing_08",
            "object_022_missing_09",
            "object_022_missing_10",
            "object_022_missing_11",
            "object_022_missing_12",
            "object_022_missing_13",
            "object_022_missing_14",
            "object_022_missing_15",
            "object_022_missing_16",
            "object_022_missing_17",
            "object_022_missing_18",
            "object_022_missing_19",
            "object_022_missing_20",
            "object_022_missing_21",
            "object_022_missing_22",
            "object_022_missing_23",
            "object_022_missing_24",
            "object_022_missing_25",
            "object_022_missing_26",
            "object_022_missing_27",
            "object_022_missing_28",
            "object_022_missing_29",
            "object_022_missing_30",
            "object_022_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_023_shape",
        unknown: "object_023_unknown",
        missing: &[
            "object_023_missing_00",
            "object_023_missing_01",
            "object_023_missing_02",
            "object_023_missing_03",
            "object_023_missing_04",
            "object_023_missing_05",
            "object_023_missing_06",
            "object_023_missing_07",
            "object_023_missing_08",
            "object_023_missing_09",
            "object_023_missing_10",
            "object_023_missing_11",
            "object_023_missing_12",
            "object_023_missing_13",
            "object_023_missing_14",
            "object_023_missing_15",
            "object_023_missing_16",
            "object_023_missing_17",
            "object_023_missing_18",
            "object_023_missing_19",
            "object_023_missing_20",
            "object_023_missing_21",
            "object_023_missing_22",
            "object_023_missing_23",
            "object_023_missing_24",
            "object_023_missing_25",
            "object_023_missing_26",
            "object_023_missing_27",
            "object_023_missing_28",
            "object_023_missing_29",
            "object_023_missing_30",
            "object_023_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_024_shape",
        unknown: "object_024_unknown",
        missing: &[
            "object_024_missing_00",
            "object_024_missing_01",
            "object_024_missing_02",
            "object_024_missing_03",
            "object_024_missing_04",
            "object_024_missing_05",
            "object_024_missing_06",
            "object_024_missing_07",
            "object_024_missing_08",
            "object_024_missing_09",
            "object_024_missing_10",
            "object_024_missing_11",
            "object_024_missing_12",
            "object_024_missing_13",
            "object_024_missing_14",
            "object_024_missing_15",
            "object_024_missing_16",
            "object_024_missing_17",
            "object_024_missing_18",
            "object_024_missing_19",
            "object_024_missing_20",
            "object_024_missing_21",
            "object_024_missing_22",
            "object_024_missing_23",
            "object_024_missing_24",
            "object_024_missing_25",
            "object_024_missing_26",
            "object_024_missing_27",
            "object_024_missing_28",
            "object_024_missing_29",
            "object_024_missing_30",
            "object_024_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_025_shape",
        unknown: "object_025_unknown",
        missing: &[
            "object_025_missing_00",
            "object_025_missing_01",
            "object_025_missing_02",
            "object_025_missing_03",
            "object_025_missing_04",
            "object_025_missing_05",
            "object_025_missing_06",
            "object_025_missing_07",
            "object_025_missing_08",
            "object_025_missing_09",
            "object_025_missing_10",
            "object_025_missing_11",
            "object_025_missing_12",
            "object_025_missing_13",
            "object_025_missing_14",
            "object_025_missing_15",
            "object_025_missing_16",
            "object_025_missing_17",
            "object_025_missing_18",
            "object_025_missing_19",
            "object_025_missing_20",
            "object_025_missing_21",
            "object_025_missing_22",
            "object_025_missing_23",
            "object_025_missing_24",
            "object_025_missing_25",
            "object_025_missing_26",
            "object_025_missing_27",
            "object_025_missing_28",
            "object_025_missing_29",
            "object_025_missing_30",
            "object_025_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_026_shape",
        unknown: "object_026_unknown",
        missing: &[
            "object_026_missing_00",
            "object_026_missing_01",
            "object_026_missing_02",
            "object_026_missing_03",
            "object_026_missing_04",
            "object_026_missing_05",
            "object_026_missing_06",
            "object_026_missing_07",
            "object_026_missing_08",
            "object_026_missing_09",
            "object_026_missing_10",
            "object_026_missing_11",
            "object_026_missing_12",
            "object_026_missing_13",
            "object_026_missing_14",
            "object_026_missing_15",
            "object_026_missing_16",
            "object_026_missing_17",
            "object_026_missing_18",
            "object_026_missing_19",
            "object_026_missing_20",
            "object_026_missing_21",
            "object_026_missing_22",
            "object_026_missing_23",
            "object_026_missing_24",
            "object_026_missing_25",
            "object_026_missing_26",
            "object_026_missing_27",
            "object_026_missing_28",
            "object_026_missing_29",
            "object_026_missing_30",
            "object_026_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_027_shape",
        unknown: "object_027_unknown",
        missing: &[
            "object_027_missing_00",
            "object_027_missing_01",
            "object_027_missing_02",
            "object_027_missing_03",
            "object_027_missing_04",
            "object_027_missing_05",
            "object_027_missing_06",
            "object_027_missing_07",
            "object_027_missing_08",
            "object_027_missing_09",
            "object_027_missing_10",
            "object_027_missing_11",
            "object_027_missing_12",
            "object_027_missing_13",
            "object_027_missing_14",
            "object_027_missing_15",
            "object_027_missing_16",
            "object_027_missing_17",
            "object_027_missing_18",
            "object_027_missing_19",
            "object_027_missing_20",
            "object_027_missing_21",
            "object_027_missing_22",
            "object_027_missing_23",
            "object_027_missing_24",
            "object_027_missing_25",
            "object_027_missing_26",
            "object_027_missing_27",
            "object_027_missing_28",
            "object_027_missing_29",
            "object_027_missing_30",
            "object_027_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_028_shape",
        unknown: "object_028_unknown",
        missing: &[
            "object_028_missing_00",
            "object_028_missing_01",
            "object_028_missing_02",
            "object_028_missing_03",
            "object_028_missing_04",
            "object_028_missing_05",
            "object_028_missing_06",
            "object_028_missing_07",
            "object_028_missing_08",
            "object_028_missing_09",
            "object_028_missing_10",
            "object_028_missing_11",
            "object_028_missing_12",
            "object_028_missing_13",
            "object_028_missing_14",
            "object_028_missing_15",
            "object_028_missing_16",
            "object_028_missing_17",
            "object_028_missing_18",
            "object_028_missing_19",
            "object_028_missing_20",
            "object_028_missing_21",
            "object_028_missing_22",
            "object_028_missing_23",
            "object_028_missing_24",
            "object_028_missing_25",
            "object_028_missing_26",
            "object_028_missing_27",
            "object_028_missing_28",
            "object_028_missing_29",
            "object_028_missing_30",
            "object_028_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_029_shape",
        unknown: "object_029_unknown",
        missing: &[
            "object_029_missing_00",
            "object_029_missing_01",
            "object_029_missing_02",
            "object_029_missing_03",
            "object_029_missing_04",
            "object_029_missing_05",
            "object_029_missing_06",
            "object_029_missing_07",
            "object_029_missing_08",
            "object_029_missing_09",
            "object_029_missing_10",
            "object_029_missing_11",
            "object_029_missing_12",
            "object_029_missing_13",
            "object_029_missing_14",
            "object_029_missing_15",
            "object_029_missing_16",
            "object_029_missing_17",
            "object_029_missing_18",
            "object_029_missing_19",
            "object_029_missing_20",
            "object_029_missing_21",
            "object_029_missing_22",
            "object_029_missing_23",
            "object_029_missing_24",
            "object_029_missing_25",
            "object_029_missing_26",
            "object_029_missing_27",
            "object_029_missing_28",
            "object_029_missing_29",
            "object_029_missing_30",
            "object_029_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_030_shape",
        unknown: "object_030_unknown",
        missing: &[
            "object_030_missing_00",
            "object_030_missing_01",
            "object_030_missing_02",
            "object_030_missing_03",
            "object_030_missing_04",
            "object_030_missing_05",
            "object_030_missing_06",
            "object_030_missing_07",
            "object_030_missing_08",
            "object_030_missing_09",
            "object_030_missing_10",
            "object_030_missing_11",
            "object_030_missing_12",
            "object_030_missing_13",
            "object_030_missing_14",
            "object_030_missing_15",
            "object_030_missing_16",
            "object_030_missing_17",
            "object_030_missing_18",
            "object_030_missing_19",
            "object_030_missing_20",
            "object_030_missing_21",
            "object_030_missing_22",
            "object_030_missing_23",
            "object_030_missing_24",
            "object_030_missing_25",
            "object_030_missing_26",
            "object_030_missing_27",
            "object_030_missing_28",
            "object_030_missing_29",
            "object_030_missing_30",
            "object_030_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_031_shape",
        unknown: "object_031_unknown",
        missing: &[
            "object_031_missing_00",
            "object_031_missing_01",
            "object_031_missing_02",
            "object_031_missing_03",
            "object_031_missing_04",
            "object_031_missing_05",
            "object_031_missing_06",
            "object_031_missing_07",
            "object_031_missing_08",
            "object_031_missing_09",
            "object_031_missing_10",
            "object_031_missing_11",
            "object_031_missing_12",
            "object_031_missing_13",
            "object_031_missing_14",
            "object_031_missing_15",
            "object_031_missing_16",
            "object_031_missing_17",
            "object_031_missing_18",
            "object_031_missing_19",
            "object_031_missing_20",
            "object_031_missing_21",
            "object_031_missing_22",
            "object_031_missing_23",
            "object_031_missing_24",
            "object_031_missing_25",
            "object_031_missing_26",
            "object_031_missing_27",
            "object_031_missing_28",
            "object_031_missing_29",
            "object_031_missing_30",
            "object_031_missing_31",
        ],
    },
    RejectionSchema {
        shape: "object_032_shape",
        unknown: "object_032_unknown",
        missing: &[
            "object_032_missing_00",
            "object_032_missing_01",
            "object_032_missing_02",
            "object_032_missing_03",
            "object_032_missing_04",
            "object_032_missing_05",
            "object_032_missing_06",
            "object_032_missing_07",
            "object_032_missing_08",
            "object_032_missing_09",
            "object_032_missing_10",
            "object_032_missing_11",
            "object_032_missing_12",
            "object_032_missing_13",
            "object_032_missing_14",
            "object_032_missing_15",
            "object_032_missing_16",
            "object_032_missing_17",
            "object_032_missing_18",
            "object_032_missing_19",
            "object_032_missing_20",
            "object_032_missing_21",
            "object_032_missing_22",
            "object_032_missing_23",
            "object_032_missing_24",
            "object_032_missing_25",
            "object_032_missing_26",
            "object_032_missing_27",
            "object_032_missing_28",
            "object_032_missing_29",
            "object_032_missing_30",
            "object_032_missing_31",
        ],
    },
];

// Exact generated ServerNotification vocabulary; diagnostics only, never event admission.
const STREAM_TURN_ID_LABELS: &[(&str, [&str; 6])] = &[
    (
        "error",
        [
            "stream_turn_id_000_missing",
            "stream_turn_id_000_null",
            "stream_turn_id_000_non_string",
            "stream_turn_id_000_empty_string",
            "stream_turn_id_000_over_256_bytes",
            "stream_turn_id_000_control_character",
        ],
    ),
    (
        "thread/started",
        [
            "stream_turn_id_001_missing",
            "stream_turn_id_001_null",
            "stream_turn_id_001_non_string",
            "stream_turn_id_001_empty_string",
            "stream_turn_id_001_over_256_bytes",
            "stream_turn_id_001_control_character",
        ],
    ),
    (
        "thread/status/changed",
        [
            "stream_turn_id_002_missing",
            "stream_turn_id_002_null",
            "stream_turn_id_002_non_string",
            "stream_turn_id_002_empty_string",
            "stream_turn_id_002_over_256_bytes",
            "stream_turn_id_002_control_character",
        ],
    ),
    (
        "thread/archived",
        [
            "stream_turn_id_003_missing",
            "stream_turn_id_003_null",
            "stream_turn_id_003_non_string",
            "stream_turn_id_003_empty_string",
            "stream_turn_id_003_over_256_bytes",
            "stream_turn_id_003_control_character",
        ],
    ),
    (
        "thread/deleted",
        [
            "stream_turn_id_004_missing",
            "stream_turn_id_004_null",
            "stream_turn_id_004_non_string",
            "stream_turn_id_004_empty_string",
            "stream_turn_id_004_over_256_bytes",
            "stream_turn_id_004_control_character",
        ],
    ),
    (
        "thread/unarchived",
        [
            "stream_turn_id_005_missing",
            "stream_turn_id_005_null",
            "stream_turn_id_005_non_string",
            "stream_turn_id_005_empty_string",
            "stream_turn_id_005_over_256_bytes",
            "stream_turn_id_005_control_character",
        ],
    ),
    (
        "thread/closed",
        [
            "stream_turn_id_006_missing",
            "stream_turn_id_006_null",
            "stream_turn_id_006_non_string",
            "stream_turn_id_006_empty_string",
            "stream_turn_id_006_over_256_bytes",
            "stream_turn_id_006_control_character",
        ],
    ),
    (
        "thread/reverted",
        [
            "stream_turn_id_007_missing",
            "stream_turn_id_007_null",
            "stream_turn_id_007_non_string",
            "stream_turn_id_007_empty_string",
            "stream_turn_id_007_over_256_bytes",
            "stream_turn_id_007_control_character",
        ],
    ),
    (
        "skills/changed",
        [
            "stream_turn_id_008_missing",
            "stream_turn_id_008_null",
            "stream_turn_id_008_non_string",
            "stream_turn_id_008_empty_string",
            "stream_turn_id_008_over_256_bytes",
            "stream_turn_id_008_control_character",
        ],
    ),
    (
        "thread/name/updated",
        [
            "stream_turn_id_009_missing",
            "stream_turn_id_009_null",
            "stream_turn_id_009_non_string",
            "stream_turn_id_009_empty_string",
            "stream_turn_id_009_over_256_bytes",
            "stream_turn_id_009_control_character",
        ],
    ),
    (
        "thread/attachment/updated",
        [
            "stream_turn_id_010_missing",
            "stream_turn_id_010_null",
            "stream_turn_id_010_non_string",
            "stream_turn_id_010_empty_string",
            "stream_turn_id_010_over_256_bytes",
            "stream_turn_id_010_control_character",
        ],
    ),
    (
        "thread/goal/updated",
        [
            "stream_turn_id_011_missing",
            "stream_turn_id_011_null",
            "stream_turn_id_011_non_string",
            "stream_turn_id_011_empty_string",
            "stream_turn_id_011_over_256_bytes",
            "stream_turn_id_011_control_character",
        ],
    ),
    (
        "thread/prediction/updated",
        [
            "stream_turn_id_012_missing",
            "stream_turn_id_012_null",
            "stream_turn_id_012_non_string",
            "stream_turn_id_012_empty_string",
            "stream_turn_id_012_over_256_bytes",
            "stream_turn_id_012_control_character",
        ],
    ),
    (
        "thread/goal/cleared",
        [
            "stream_turn_id_013_missing",
            "stream_turn_id_013_null",
            "stream_turn_id_013_non_string",
            "stream_turn_id_013_empty_string",
            "stream_turn_id_013_over_256_bytes",
            "stream_turn_id_013_control_character",
        ],
    ),
    (
        "thread/queue/changed",
        [
            "stream_turn_id_014_missing",
            "stream_turn_id_014_null",
            "stream_turn_id_014_non_string",
            "stream_turn_id_014_empty_string",
            "stream_turn_id_014_over_256_bytes",
            "stream_turn_id_014_control_character",
        ],
    ),
    (
        "project/changed",
        [
            "stream_turn_id_015_missing",
            "stream_turn_id_015_null",
            "stream_turn_id_015_non_string",
            "stream_turn_id_015_empty_string",
            "stream_turn_id_015_over_256_bytes",
            "stream_turn_id_015_control_character",
        ],
    ),
    (
        "thread/project/updated",
        [
            "stream_turn_id_016_missing",
            "stream_turn_id_016_null",
            "stream_turn_id_016_non_string",
            "stream_turn_id_016_empty_string",
            "stream_turn_id_016_over_256_bytes",
            "stream_turn_id_016_control_character",
        ],
    ),
    (
        "thread/environment/connected",
        [
            "stream_turn_id_017_missing",
            "stream_turn_id_017_null",
            "stream_turn_id_017_non_string",
            "stream_turn_id_017_empty_string",
            "stream_turn_id_017_over_256_bytes",
            "stream_turn_id_017_control_character",
        ],
    ),
    (
        "thread/environment/disconnected",
        [
            "stream_turn_id_018_missing",
            "stream_turn_id_018_null",
            "stream_turn_id_018_non_string",
            "stream_turn_id_018_empty_string",
            "stream_turn_id_018_over_256_bytes",
            "stream_turn_id_018_control_character",
        ],
    ),
    (
        "thread/settings/updated",
        [
            "stream_turn_id_019_missing",
            "stream_turn_id_019_null",
            "stream_turn_id_019_non_string",
            "stream_turn_id_019_empty_string",
            "stream_turn_id_019_over_256_bytes",
            "stream_turn_id_019_control_character",
        ],
    ),
    (
        "thread/tokenUsage/updated",
        [
            "stream_turn_id_020_missing",
            "stream_turn_id_020_null",
            "stream_turn_id_020_non_string",
            "stream_turn_id_020_empty_string",
            "stream_turn_id_020_over_256_bytes",
            "stream_turn_id_020_control_character",
        ],
    ),
    (
        "turn/started",
        [
            "stream_turn_id_021_missing",
            "stream_turn_id_021_null",
            "stream_turn_id_021_non_string",
            "stream_turn_id_021_empty_string",
            "stream_turn_id_021_over_256_bytes",
            "stream_turn_id_021_control_character",
        ],
    ),
    (
        "hook/started",
        [
            "stream_turn_id_022_missing",
            "stream_turn_id_022_null",
            "stream_turn_id_022_non_string",
            "stream_turn_id_022_empty_string",
            "stream_turn_id_022_over_256_bytes",
            "stream_turn_id_022_control_character",
        ],
    ),
    (
        "turn/completed",
        [
            "stream_turn_id_023_missing",
            "stream_turn_id_023_null",
            "stream_turn_id_023_non_string",
            "stream_turn_id_023_empty_string",
            "stream_turn_id_023_over_256_bytes",
            "stream_turn_id_023_control_character",
        ],
    ),
    (
        "hook/completed",
        [
            "stream_turn_id_024_missing",
            "stream_turn_id_024_null",
            "stream_turn_id_024_non_string",
            "stream_turn_id_024_empty_string",
            "stream_turn_id_024_over_256_bytes",
            "stream_turn_id_024_control_character",
        ],
    ),
    (
        "turn/diff/updated",
        [
            "stream_turn_id_025_missing",
            "stream_turn_id_025_null",
            "stream_turn_id_025_non_string",
            "stream_turn_id_025_empty_string",
            "stream_turn_id_025_over_256_bytes",
            "stream_turn_id_025_control_character",
        ],
    ),
    (
        "turn/plan/updated",
        [
            "stream_turn_id_026_missing",
            "stream_turn_id_026_null",
            "stream_turn_id_026_non_string",
            "stream_turn_id_026_empty_string",
            "stream_turn_id_026_over_256_bytes",
            "stream_turn_id_026_control_character",
        ],
    ),
    (
        "item/started",
        [
            "stream_turn_id_027_missing",
            "stream_turn_id_027_null",
            "stream_turn_id_027_non_string",
            "stream_turn_id_027_empty_string",
            "stream_turn_id_027_over_256_bytes",
            "stream_turn_id_027_control_character",
        ],
    ),
    (
        "item/autoApprovalReview/started",
        [
            "stream_turn_id_028_missing",
            "stream_turn_id_028_null",
            "stream_turn_id_028_non_string",
            "stream_turn_id_028_empty_string",
            "stream_turn_id_028_over_256_bytes",
            "stream_turn_id_028_control_character",
        ],
    ),
    (
        "item/autoApprovalReview/completed",
        [
            "stream_turn_id_029_missing",
            "stream_turn_id_029_null",
            "stream_turn_id_029_non_string",
            "stream_turn_id_029_empty_string",
            "stream_turn_id_029_over_256_bytes",
            "stream_turn_id_029_control_character",
        ],
    ),
    (
        "autoApprovalReview/strictReviewRequired",
        [
            "stream_turn_id_030_missing",
            "stream_turn_id_030_null",
            "stream_turn_id_030_non_string",
            "stream_turn_id_030_empty_string",
            "stream_turn_id_030_over_256_bytes",
            "stream_turn_id_030_control_character",
        ],
    ),
    (
        "item/completed",
        [
            "stream_turn_id_031_missing",
            "stream_turn_id_031_null",
            "stream_turn_id_031_non_string",
            "stream_turn_id_031_empty_string",
            "stream_turn_id_031_over_256_bytes",
            "stream_turn_id_031_control_character",
        ],
    ),
    (
        "item/agentMessage/delta",
        [
            "stream_turn_id_032_missing",
            "stream_turn_id_032_null",
            "stream_turn_id_032_non_string",
            "stream_turn_id_032_empty_string",
            "stream_turn_id_032_over_256_bytes",
            "stream_turn_id_032_control_character",
        ],
    ),
    (
        "item/plan/delta",
        [
            "stream_turn_id_033_missing",
            "stream_turn_id_033_null",
            "stream_turn_id_033_non_string",
            "stream_turn_id_033_empty_string",
            "stream_turn_id_033_over_256_bytes",
            "stream_turn_id_033_control_character",
        ],
    ),
    (
        "command/exec/outputDelta",
        [
            "stream_turn_id_034_missing",
            "stream_turn_id_034_null",
            "stream_turn_id_034_non_string",
            "stream_turn_id_034_empty_string",
            "stream_turn_id_034_over_256_bytes",
            "stream_turn_id_034_control_character",
        ],
    ),
    (
        "process/outputDelta",
        [
            "stream_turn_id_035_missing",
            "stream_turn_id_035_null",
            "stream_turn_id_035_non_string",
            "stream_turn_id_035_empty_string",
            "stream_turn_id_035_over_256_bytes",
            "stream_turn_id_035_control_character",
        ],
    ),
    (
        "process/exited",
        [
            "stream_turn_id_036_missing",
            "stream_turn_id_036_null",
            "stream_turn_id_036_non_string",
            "stream_turn_id_036_empty_string",
            "stream_turn_id_036_over_256_bytes",
            "stream_turn_id_036_control_character",
        ],
    ),
    (
        "item/commandExecution/outputDelta",
        [
            "stream_turn_id_037_missing",
            "stream_turn_id_037_null",
            "stream_turn_id_037_non_string",
            "stream_turn_id_037_empty_string",
            "stream_turn_id_037_over_256_bytes",
            "stream_turn_id_037_control_character",
        ],
    ),
    (
        "item/commandExecution/terminalInteraction",
        [
            "stream_turn_id_038_missing",
            "stream_turn_id_038_null",
            "stream_turn_id_038_non_string",
            "stream_turn_id_038_empty_string",
            "stream_turn_id_038_over_256_bytes",
            "stream_turn_id_038_control_character",
        ],
    ),
    (
        "item/fileChange/outputDelta",
        [
            "stream_turn_id_039_missing",
            "stream_turn_id_039_null",
            "stream_turn_id_039_non_string",
            "stream_turn_id_039_empty_string",
            "stream_turn_id_039_over_256_bytes",
            "stream_turn_id_039_control_character",
        ],
    ),
    (
        "item/fileChange/patchUpdated",
        [
            "stream_turn_id_040_missing",
            "stream_turn_id_040_null",
            "stream_turn_id_040_non_string",
            "stream_turn_id_040_empty_string",
            "stream_turn_id_040_over_256_bytes",
            "stream_turn_id_040_control_character",
        ],
    ),
    (
        "serverRequest/resolved",
        [
            "stream_turn_id_041_missing",
            "stream_turn_id_041_null",
            "stream_turn_id_041_non_string",
            "stream_turn_id_041_empty_string",
            "stream_turn_id_041_over_256_bytes",
            "stream_turn_id_041_control_character",
        ],
    ),
    (
        "item/mcpToolCall/progress",
        [
            "stream_turn_id_042_missing",
            "stream_turn_id_042_null",
            "stream_turn_id_042_non_string",
            "stream_turn_id_042_empty_string",
            "stream_turn_id_042_over_256_bytes",
            "stream_turn_id_042_control_character",
        ],
    ),
    (
        "mcpServer/oauthLogin/completed",
        [
            "stream_turn_id_043_missing",
            "stream_turn_id_043_null",
            "stream_turn_id_043_non_string",
            "stream_turn_id_043_empty_string",
            "stream_turn_id_043_over_256_bytes",
            "stream_turn_id_043_control_character",
        ],
    ),
    (
        "mcpServer/startupStatus/updated",
        [
            "stream_turn_id_044_missing",
            "stream_turn_id_044_null",
            "stream_turn_id_044_non_string",
            "stream_turn_id_044_empty_string",
            "stream_turn_id_044_over_256_bytes",
            "stream_turn_id_044_control_character",
        ],
    ),
    (
        "mcpServer/event/stream/notification",
        [
            "stream_turn_id_045_missing",
            "stream_turn_id_045_null",
            "stream_turn_id_045_non_string",
            "stream_turn_id_045_empty_string",
            "stream_turn_id_045_over_256_bytes",
            "stream_turn_id_045_control_character",
        ],
    ),
    (
        "account/updated",
        [
            "stream_turn_id_046_missing",
            "stream_turn_id_046_null",
            "stream_turn_id_046_non_string",
            "stream_turn_id_046_empty_string",
            "stream_turn_id_046_over_256_bytes",
            "stream_turn_id_046_control_character",
        ],
    ),
    (
        "account/gatewayOAuth/changed",
        [
            "stream_turn_id_047_missing",
            "stream_turn_id_047_null",
            "stream_turn_id_047_non_string",
            "stream_turn_id_047_empty_string",
            "stream_turn_id_047_over_256_bytes",
            "stream_turn_id_047_control_character",
        ],
    ),
    (
        "account/rateLimits/updated",
        [
            "stream_turn_id_048_missing",
            "stream_turn_id_048_null",
            "stream_turn_id_048_non_string",
            "stream_turn_id_048_empty_string",
            "stream_turn_id_048_over_256_bytes",
            "stream_turn_id_048_control_character",
        ],
    ),
    (
        "app/list/updated",
        [
            "stream_turn_id_049_missing",
            "stream_turn_id_049_null",
            "stream_turn_id_049_non_string",
            "stream_turn_id_049_empty_string",
            "stream_turn_id_049_over_256_bytes",
            "stream_turn_id_049_control_character",
        ],
    ),
    (
        "remoteControl/status/changed",
        [
            "stream_turn_id_050_missing",
            "stream_turn_id_050_null",
            "stream_turn_id_050_non_string",
            "stream_turn_id_050_empty_string",
            "stream_turn_id_050_over_256_bytes",
            "stream_turn_id_050_control_character",
        ],
    ),
    (
        "externalAgentConfig/import/progress",
        [
            "stream_turn_id_051_missing",
            "stream_turn_id_051_null",
            "stream_turn_id_051_non_string",
            "stream_turn_id_051_empty_string",
            "stream_turn_id_051_over_256_bytes",
            "stream_turn_id_051_control_character",
        ],
    ),
    (
        "externalAgentConfig/import/completed",
        [
            "stream_turn_id_052_missing",
            "stream_turn_id_052_null",
            "stream_turn_id_052_non_string",
            "stream_turn_id_052_empty_string",
            "stream_turn_id_052_over_256_bytes",
            "stream_turn_id_052_control_character",
        ],
    ),
    (
        "fs/changed",
        [
            "stream_turn_id_053_missing",
            "stream_turn_id_053_null",
            "stream_turn_id_053_non_string",
            "stream_turn_id_053_empty_string",
            "stream_turn_id_053_over_256_bytes",
            "stream_turn_id_053_control_character",
        ],
    ),
    (
        "item/reasoning/summaryTextDelta",
        [
            "stream_turn_id_054_missing",
            "stream_turn_id_054_null",
            "stream_turn_id_054_non_string",
            "stream_turn_id_054_empty_string",
            "stream_turn_id_054_over_256_bytes",
            "stream_turn_id_054_control_character",
        ],
    ),
    (
        "item/reasoning/summaryPartAdded",
        [
            "stream_turn_id_055_missing",
            "stream_turn_id_055_null",
            "stream_turn_id_055_non_string",
            "stream_turn_id_055_empty_string",
            "stream_turn_id_055_over_256_bytes",
            "stream_turn_id_055_control_character",
        ],
    ),
    (
        "item/reasoning/textDelta",
        [
            "stream_turn_id_056_missing",
            "stream_turn_id_056_null",
            "stream_turn_id_056_non_string",
            "stream_turn_id_056_empty_string",
            "stream_turn_id_056_over_256_bytes",
            "stream_turn_id_056_control_character",
        ],
    ),
    (
        "thread/compacted",
        [
            "stream_turn_id_057_missing",
            "stream_turn_id_057_null",
            "stream_turn_id_057_non_string",
            "stream_turn_id_057_empty_string",
            "stream_turn_id_057_over_256_bytes",
            "stream_turn_id_057_control_character",
        ],
    ),
    (
        "model/rerouted",
        [
            "stream_turn_id_058_missing",
            "stream_turn_id_058_null",
            "stream_turn_id_058_non_string",
            "stream_turn_id_058_empty_string",
            "stream_turn_id_058_over_256_bytes",
            "stream_turn_id_058_control_character",
        ],
    ),
    (
        "model/verification",
        [
            "stream_turn_id_059_missing",
            "stream_turn_id_059_null",
            "stream_turn_id_059_non_string",
            "stream_turn_id_059_empty_string",
            "stream_turn_id_059_over_256_bytes",
            "stream_turn_id_059_control_character",
        ],
    ),
    (
        "modelProvider/authRecoveryStarted",
        [
            "stream_turn_id_060_missing",
            "stream_turn_id_060_null",
            "stream_turn_id_060_non_string",
            "stream_turn_id_060_empty_string",
            "stream_turn_id_060_over_256_bytes",
            "stream_turn_id_060_control_character",
        ],
    ),
    (
        "modelProvider/authRecoveryCompleted",
        [
            "stream_turn_id_061_missing",
            "stream_turn_id_061_null",
            "stream_turn_id_061_non_string",
            "stream_turn_id_061_empty_string",
            "stream_turn_id_061_over_256_bytes",
            "stream_turn_id_061_control_character",
        ],
    ),
    (
        "turn/moderationMetadata",
        [
            "stream_turn_id_062_missing",
            "stream_turn_id_062_null",
            "stream_turn_id_062_non_string",
            "stream_turn_id_062_empty_string",
            "stream_turn_id_062_over_256_bytes",
            "stream_turn_id_062_control_character",
        ],
    ),
    (
        "model/safetyBuffering/updated",
        [
            "stream_turn_id_063_missing",
            "stream_turn_id_063_null",
            "stream_turn_id_063_non_string",
            "stream_turn_id_063_empty_string",
            "stream_turn_id_063_over_256_bytes",
            "stream_turn_id_063_control_character",
        ],
    ),
    (
        "warning",
        [
            "stream_turn_id_064_missing",
            "stream_turn_id_064_null",
            "stream_turn_id_064_non_string",
            "stream_turn_id_064_empty_string",
            "stream_turn_id_064_over_256_bytes",
            "stream_turn_id_064_control_character",
        ],
    ),
    (
        "guardianWarning",
        [
            "stream_turn_id_065_missing",
            "stream_turn_id_065_null",
            "stream_turn_id_065_non_string",
            "stream_turn_id_065_empty_string",
            "stream_turn_id_065_over_256_bytes",
            "stream_turn_id_065_control_character",
        ],
    ),
    (
        "deprecationNotice",
        [
            "stream_turn_id_066_missing",
            "stream_turn_id_066_null",
            "stream_turn_id_066_non_string",
            "stream_turn_id_066_empty_string",
            "stream_turn_id_066_over_256_bytes",
            "stream_turn_id_066_control_character",
        ],
    ),
    (
        "configWarning",
        [
            "stream_turn_id_067_missing",
            "stream_turn_id_067_null",
            "stream_turn_id_067_non_string",
            "stream_turn_id_067_empty_string",
            "stream_turn_id_067_over_256_bytes",
            "stream_turn_id_067_control_character",
        ],
    ),
    (
        "fuzzyFileSearch/sessionUpdated",
        [
            "stream_turn_id_068_missing",
            "stream_turn_id_068_null",
            "stream_turn_id_068_non_string",
            "stream_turn_id_068_empty_string",
            "stream_turn_id_068_over_256_bytes",
            "stream_turn_id_068_control_character",
        ],
    ),
    (
        "fuzzyFileSearch/sessionCompleted",
        [
            "stream_turn_id_069_missing",
            "stream_turn_id_069_null",
            "stream_turn_id_069_non_string",
            "stream_turn_id_069_empty_string",
            "stream_turn_id_069_over_256_bytes",
            "stream_turn_id_069_control_character",
        ],
    ),
    (
        "thread/realtime/started",
        [
            "stream_turn_id_070_missing",
            "stream_turn_id_070_null",
            "stream_turn_id_070_non_string",
            "stream_turn_id_070_empty_string",
            "stream_turn_id_070_over_256_bytes",
            "stream_turn_id_070_control_character",
        ],
    ),
    (
        "thread/realtime/itemAdded",
        [
            "stream_turn_id_071_missing",
            "stream_turn_id_071_null",
            "stream_turn_id_071_non_string",
            "stream_turn_id_071_empty_string",
            "stream_turn_id_071_over_256_bytes",
            "stream_turn_id_071_control_character",
        ],
    ),
    (
        "thread/realtime/item/started",
        [
            "stream_turn_id_072_missing",
            "stream_turn_id_072_null",
            "stream_turn_id_072_non_string",
            "stream_turn_id_072_empty_string",
            "stream_turn_id_072_over_256_bytes",
            "stream_turn_id_072_control_character",
        ],
    ),
    (
        "thread/realtime/item/transcript/delta",
        [
            "stream_turn_id_073_missing",
            "stream_turn_id_073_null",
            "stream_turn_id_073_non_string",
            "stream_turn_id_073_empty_string",
            "stream_turn_id_073_over_256_bytes",
            "stream_turn_id_073_control_character",
        ],
    ),
    (
        "thread/realtime/item/completed",
        [
            "stream_turn_id_074_missing",
            "stream_turn_id_074_null",
            "stream_turn_id_074_non_string",
            "stream_turn_id_074_empty_string",
            "stream_turn_id_074_over_256_bytes",
            "stream_turn_id_074_control_character",
        ],
    ),
    (
        "thread/realtime/transcript/delta",
        [
            "stream_turn_id_075_missing",
            "stream_turn_id_075_null",
            "stream_turn_id_075_non_string",
            "stream_turn_id_075_empty_string",
            "stream_turn_id_075_over_256_bytes",
            "stream_turn_id_075_control_character",
        ],
    ),
    (
        "thread/realtime/transcript/done",
        [
            "stream_turn_id_076_missing",
            "stream_turn_id_076_null",
            "stream_turn_id_076_non_string",
            "stream_turn_id_076_empty_string",
            "stream_turn_id_076_over_256_bytes",
            "stream_turn_id_076_control_character",
        ],
    ),
    (
        "thread/realtime/outputAudio/delta",
        [
            "stream_turn_id_077_missing",
            "stream_turn_id_077_null",
            "stream_turn_id_077_non_string",
            "stream_turn_id_077_empty_string",
            "stream_turn_id_077_over_256_bytes",
            "stream_turn_id_077_control_character",
        ],
    ),
    (
        "thread/realtime/sdp",
        [
            "stream_turn_id_078_missing",
            "stream_turn_id_078_null",
            "stream_turn_id_078_non_string",
            "stream_turn_id_078_empty_string",
            "stream_turn_id_078_over_256_bytes",
            "stream_turn_id_078_control_character",
        ],
    ),
    (
        "thread/realtime/error",
        [
            "stream_turn_id_079_missing",
            "stream_turn_id_079_null",
            "stream_turn_id_079_non_string",
            "stream_turn_id_079_empty_string",
            "stream_turn_id_079_over_256_bytes",
            "stream_turn_id_079_control_character",
        ],
    ),
    (
        "thread/realtime/closed",
        [
            "stream_turn_id_080_missing",
            "stream_turn_id_080_null",
            "stream_turn_id_080_non_string",
            "stream_turn_id_080_empty_string",
            "stream_turn_id_080_over_256_bytes",
            "stream_turn_id_080_control_character",
        ],
    ),
    (
        "windows/worldWritableWarning",
        [
            "stream_turn_id_081_missing",
            "stream_turn_id_081_null",
            "stream_turn_id_081_non_string",
            "stream_turn_id_081_empty_string",
            "stream_turn_id_081_over_256_bytes",
            "stream_turn_id_081_control_character",
        ],
    ),
    (
        "windowsSandbox/setupCompleted",
        [
            "stream_turn_id_082_missing",
            "stream_turn_id_082_null",
            "stream_turn_id_082_non_string",
            "stream_turn_id_082_empty_string",
            "stream_turn_id_082_over_256_bytes",
            "stream_turn_id_082_control_character",
        ],
    ),
    (
        "account/login/completed",
        [
            "stream_turn_id_083_missing",
            "stream_turn_id_083_null",
            "stream_turn_id_083_non_string",
            "stream_turn_id_083_empty_string",
            "stream_turn_id_083_over_256_bytes",
            "stream_turn_id_083_control_character",
        ],
    ),
];
fn stream_turn_id_site(value: &Value) -> &'static str {
    let Some((_, labels)) = STREAM_TURN_ID_LABELS
        .iter()
        .find(|(method, _)| value["method"].as_str() == Some(*method))
    else {
        return "stream_turn_id_unknown_method";
    };
    let shape = match value["params"].get("turnId") {
        None => 0,
        Some(Value::Null) => 1,
        Some(Value::String(text)) if text.is_empty() => 3,
        Some(Value::String(text)) if text.len() > 256 => 4,
        Some(Value::String(text)) if text.chars().any(char::is_control) => 5,
        Some(Value::String(_)) => return "stream_turn_id_unclassified",
        Some(_) => 2,
    };
    labels[shape]
}
fn stream_turn_identifier(value: &Value) -> Result<String, DirectError> {
    identifier(&value["params"]["turnId"])
        .map_err(|error| rejection_error(stream_turn_id_site(value), error))
}
fn reviewed_rejection_site(site: &'static str) -> &'static str {
    if STREAM_TURN_ID_LABELS
        .iter()
        .any(|(_, labels)| labels.contains(&site))
        || [
            "stream_turn_id_unknown_method",
            "stream_turn_id_unclassified",
        ]
        .contains(&site)
        || OBSERVATION_LABELS.contains(&site)
        || REJECTION_FIXED.contains(&site)
        || REJECTION_SCHEMAS
            .iter()
            .any(|s| s.shape == site || s.unknown == site || s.missing.contains(&site))
    {
        site
    } else {
        "unreviewed_site"
    }
}
fn rejection_error_using(
    site: &'static str,
    error: DirectError,
    make: impl FnOnce(&'static str) -> crate::diagnostics::Attempt,
) -> DirectError {
    let first = REJECTION_RECORDED
        .try_with(|v| !v.replace(true))
        .unwrap_or(false);
    if first {
        let mut attempt = make(reviewed_rejection_site(site));
        attempt.finish(Err(error));
    }
    error
}
fn rejection_error(site: &'static str, error: DirectError) -> DirectError {
    rejection_error_using(site, error, |label| {
        crate::diagnostics::Attempt::new(
            crate::agent_preferences::AgentConnection::Codex,
            "",
            None,
            None,
            Some(label),
        )
    })
}
fn identifier_at(site: &'static str, value: &Value) -> Result<String, DirectError> {
    identifier(value).map_err(|e| rejection_error(site, e))
}
fn closed_reason(schema: &RejectionSchema, value: &Value, required: &[&str]) -> &'static str {
    match value.as_object() {
        None => schema.shape,
        Some(object) => required
            .iter()
            .position(|key| !object.contains_key(*key))
            .and_then(|i| schema.missing.get(i).copied())
            .unwrap_or(schema.unknown),
    }
}
fn closed_at(
    schema: &RejectionSchema,
    value: &Value,
    required: &[&str],
    optional: &[&str],
) -> Result<(), DirectError> {
    closed(value, required, optional)
        .map_err(|e| rejection_error(closed_reason(schema, value, required), e))
}
fn request_receive_site(method: &str) -> &'static str {
    match method {
        "initialize" => "request_initialize",
        "account/read" => "request_account",
        "model/list" => "request_catalog",
        "thread/start" => "request_thread",
        "turn/start" => "request_turn",
        _ => "unreviewed_site",
    }
}
async fn run_with_rules(
    setup: Setup,
    model: String,
    effort: ReasoningEffort,
    input: String,
    rules: &str,
    emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    let cancellation = setup.cancellation_handle();
    REJECTION_RECORDED
        .scope(std::cell::Cell::new(false), async {
            run_with_rules_inner(setup, model, effort, input, rules, emit)
                .await
                .map_err(|e| {
                    if e == DirectError::InvalidRequest && cancellation.returned_locally() {
                        e
                    } else {
                        rejection_error("run_unclassified", e)
                    }
                })
        })
        .await
}

#[cfg(test)]
mod tests {
    fn original_abort_turn_fields(turn: &Value, terminal: bool) -> Result<(), DirectError> {
        closed(
            turn,
            &["id", "items", "status"],
            &[
                "completedAt",
                "durationMs",
                "error",
                "itemsView",
                "rootTurnId",
                "startedAt",
            ],
        )?;
        identifier(&turn["id"])?;
        for key in ["completedAt", "durationMs", "startedAt"] {
            if turn
                .get(key)
                .is_some_and(|v| !v.is_null() && v.as_i64().is_none())
            {
                return Err(DirectError::Protocol);
            }
        }
        if turn.get("error").is_some_and(|v| !v.is_null())
            || turn.get("itemsView").is_some_and(|v| *v != "full")
            || turn
                .get("rootTurnId")
                .is_some_and(|v| !v.is_null() && *v != turn["id"])
        {
            return Err(DirectError::Protocol);
        }
        let items = turn["items"].as_array().ok_or(DirectError::Protocol)?;
        if terminal {
            if !matches!(
                turn["status"].as_str(),
                Some("completed" | "interrupted" | "failed")
            ) {
                return Err(DirectError::Protocol);
            }
        } else if turn["status"] != "inProgress" || !items.is_empty() {
            return Err(DirectError::Protocol);
        }
        Ok(())
    }

    #[test]
    fn eligibility_diagnostic_guard_conjunction_equivalence() {
        for mask in 0..128u8 {
            let policy = DerivedState {
                authenticated: mask & 1 != 0,
                deny_all_verified: mask & 2 != 0,
                low_effort_verified: mask & 4 != 0,
                model: Some(
                    if mask & 8 != 0 {
                        "gpt-5.6-luna"
                    } else {
                        "other"
                    }
                    .into(),
                ),
                thread: (mask & 16 != 0).then(|| "thread".into()),
                turn: (mask & 32 != 0).then(|| "turn".into()),
                ..DerivedState::default()
            };
            let state = AbortStream {
                poisoned: mask & 64 == 0,
                ..AbortStream::default()
            };
            let checks = abort_context_checks(&policy, &state);
            assert_eq!(checks, std::array::from_fn(|i| mask & (1 << i) != 0));
            assert_eq!(abort_context(&policy, &state), mask == 127);
        }
        for invalid in ["".to_string(), "a".repeat(257), "a\nb".to_string()] {
            let policy = DerivedState {
                thread: Some(invalid.clone()),
                turn: Some(invalid),
                ..DerivedState::default()
            };
            assert!(!abort_context_checks(&policy, &AbortStream::default())[4]);
            assert!(!abort_context_checks(&policy, &AbortStream::default())[5]);
        }
    }
    #[test]
    fn pinned_turn_view_snapshots_preserve_strict_lifecycle() -> Result<(), DirectError> {
        let initial = json!({"id":"u","items":[],"status":"inProgress","itemsView":"notLoaded"});
        assert_eq!(abort_turn_fields(&initial, false), Ok(()));
        let mut stream = AbortStream::default();
        stream.observe(&json!({"method":"turn/started","params":{"turn":initial}}))?;
        stream.observe(&abort_message("item/started"))?;
        stream.observe(&abort_message("item/completed"))?;
        let item = json!({"id":"i","type":"agentMessage","text":"fixture","phase":"final_answer"});
        let summary = json!({"id":"u","items":[item],"status":"completed","itemsView":"summary"});
        assert_eq!(abort_turn_fields(&summary, true), Ok(()));
        stream.observe(&json!({"method":"turn/completed","params":{"turn":summary}}))?;
        assert!(stream.terminal);
        let unloaded = json!({"id":"u","items":[],"status":"interrupted","itemsView":"notLoaded"});
        let (_, mut interrupted) = abort_open()?;
        interrupted.observe(&json!({"method":"turn/completed","params":{"turn":unloaded}}))?;
        assert!(interrupted.terminal);
        for view in [Value::Null, json!(true), json!("unknown"), json!("summary")] {
            let mut bad = initial.clone();
            bad["itemsView"] = view;
            assert!(abort_turn_fields(&bad, false).is_err());
        }
        let mut nonempty = initial.clone();
        nonempty["items"] = json!([item]);
        assert!(abort_turn_fields(&nonempty, false).is_err());
        for items in [
            json!([]),
            json!([item.clone(), item.clone()]),
            json!([{"id":"i","type":"toolCall"}]),
        ] {
            let mut bad = summary.clone();
            bad["items"] = items;
            assert!(abort_turn_fields(&bad, true).is_err());
        }
        for phase in [json!("commentary"), json!("partial_answer"), json!(false)] {
            let mut bad = summary.clone();
            bad["items"][0]["phase"] = phase;
            assert!(abort_turn_fields(&bad, true).is_err());
        }
        for status in ["interrupted", "failed", "inProgress"] {
            let mut bad = summary.clone();
            bad["status"] = json!(status);
            assert!(abort_turn_fields(&bad, true).is_err());
        }
        for text in [json!(" \n"), Value::Null, json!(1)] {
            let mut bad = summary.clone();
            bad["items"][0]["text"] = text;
            assert!(abort_turn_fields(&bad, true).is_err());
        }
        for completed in [false, true] {
            let mut state = AbortStream::default();
            if !completed {
                state.observe(&abort_message("item/started"))?;
            }
            assert!(state
                .observe(&json!({"method":"turn/completed","params":{"turn":summary}}))
                .is_err());
        }
        Ok(())
    }

    #[test]
    fn eligibility_diagnostic_turn_reasons_and_projection() -> Result<(), DirectError> {
        let good = json!({"id":"turn", "items":[], "status":"inProgress"});
        assert_eq!(abort_turn_fields_checked(&good, false), Ok(()));
        let cases = [
            ("extra", json!(true), "shape"),
            ("id", json!(""), "identifier"),
            ("durationMs", json!(1.5), "time"),
            ("error", json!({}), "error"),
            ("itemsView", json!("partial"), "items_view"),
            ("rootTurnId", json!("other"), "root_turn"),
            ("items", json!({}), "items_array"),
            ("status", json!("completed"), "status"),
            ("items", json!([{}]), "nonempty_items"),
        ];
        for (key, value, reason) in cases {
            let mut turn = good.clone();
            turn[key] = value;
            let checked = abort_turn_fields_checked(&turn, false);
            let (error, label) = checked.err().ok_or(DirectError::Internal)?;
            assert_eq!(label, format!("abort_turn_{reason}_rejected"));
            assert!(OBSERVATION_LABELS.contains(&label));
            assert_eq!(abort_turn_fields(&turn, false), Err(error));
            assert_eq!(
                abort_turn_fields(&turn, false),
                original_abort_turn_fields(&turn, false)
            );
        }
        for key in ["completedAt", "durationMs", "startedAt"] {
            for value in [
                Value::Null,
                json!(0),
                json!(-1),
                json!("invalid"),
                json!(true),
                json!(1.2),
            ] {
                let mut turn = good.clone();
                turn[key] = value;
                for terminal in [false, true] {
                    assert_eq!(
                        abort_turn_fields(&turn, terminal),
                        original_abort_turn_fields(&turn, terminal)
                    );
                }
            }
        }
        for status in [
            "inProgress",
            "completed",
            "interrupted",
            "failed",
            "unknown",
        ] {
            for items in [json!([]), json!([{}]), Value::Null] {
                let mut turn = good.clone();
                turn["status"] = json!(status);
                turn["items"] = items;
                for terminal in [false, true] {
                    assert_eq!(
                        abort_turn_fields(&turn, terminal),
                        original_abort_turn_fields(&turn, terminal)
                    );
                }
            }
        }
        for turn in [
            Value::Null,
            json!([]),
            json!({}),
            json!({"id": false, "items":[], "status":"inProgress"}),
        ] {
            for terminal in [false, true] {
                assert_eq!(
                    abort_turn_fields(&turn, terminal),
                    original_abort_turn_fields(&turn, terminal)
                );
            }
        }
        Ok(())
    }

    use super::*;

    fn warning_ready() -> DerivedState {
        DerivedState {
            authenticated: true,
            deny_all_verified: true,
            model: Some("gpt-5.6-luna".into()),
            thread: Some("t".into()),
            awaiting_turn: true,
            ..Default::default()
        }
    }
    fn closed_warning() -> Value {
        json!({"method":"warning","params":{"message":WARNING_MESSAGES[1].0,"threadId":"t"}})
    }
    #[test]
    fn pinned_fail_closed_warning_requires_effective_policy_and_exact_binding() {
        for phase in [
            "warning_before_response",
            "warning_streaming",
            "warning_shutdown",
        ] {
            let mut state = warning_ready();
            assert_eq!(state.handle_warning(&closed_warning(), phase), Ok(true));
            assert!(state.turn.is_none()); // Notification never consumes a turn response.
            assert_eq!(
                state.handle_warning(&closed_warning(), phase),
                Err(DirectError::Protocol)
            );
            for change in 0..6 {
                let mut state = warning_ready();
                match change {
                    0 => state.authenticated = false,
                    1 => state.deny_all_verified = false,
                    2 => state.model = Some("other".into()),
                    3 => state.thread = Some("other".into()),
                    4 => state.thread = None,
                    _ => state.awaiting_turn = false,
                }
                assert_eq!(
                    state.handle_warning(&closed_warning(), phase),
                    Err(DirectError::Protocol)
                );
            }
            for value in [
                json!({"method":"warning","params":{"message":WARNING_MESSAGES[0].0,"threadId":"t"}}),
                json!({"method":"warning","params":{"message":format!("{} PRIVATE_CANARY", WARNING_MESSAGES[1].0),"threadId":"t"}}),
                json!({"method":"warning","params":{"message":WARNING_MESSAGES[1].0,"threadId":"t","extra":false}}),
                json!({"id":1,"method":"warning","params":{"message":WARNING_MESSAGES[1].0,"threadId":"t"}}),
            ] {
                assert_eq!(
                    warning_ready().handle_warning(&value, phase),
                    Err(DirectError::Protocol)
                );
            }
        }
    }
    #[test]
    fn pinned_warning_stream_and_tail_keep_unknown_rejection() {
        let mut early = warning_ready();
        assert_eq!(early.stream(&closed_warning()), Ok(()));
        assert_eq!(early.stream(&closed_warning()), Err(DirectError::Protocol));
        let mut late = warning_ready();
        late.awaiting_turn = false;
        late.turn = Some("u".into());
        assert_eq!(validate_tail(&mut late, &closed_warning()), Ok(()));
        assert_eq!(
            validate_tail(&mut late, &closed_warning()),
            Err(DirectError::Protocol)
        );
        assert!(validate_tail(
            &mut warning_ready(),
            &json!({"method":"unknown","params":{}})
        )
        .is_err());
    }
    #[test]
    fn native_warning_exact_signatures_and_private_unknowns() {
        for (message, label) in WARNING_MESSAGES {
            let value = json!({"method":"warning","params":{"message":message,"threadId":"t"}});
            assert_eq!(warning_label(&value, Some("t")), *label);
            for phase in [
                "warning_before_response",
                "warning_streaming",
                "warning_shutdown",
            ] {
                assert_eq!(
                    reject_warning(&value, Some("t"), phase),
                    Err(DirectError::Protocol)
                );
            }
            assert_eq!(warning_label(&value, Some("other")), "warning_unbound");
            let partial = json!({"method":"warning","params":{"message":format!("{message}PRIVATE_CANARY"),"threadId":"t"}});
            assert_eq!(warning_label(&partial, Some("t")), "warning_unknown");
        }
        let value = json!({"method":"warning","params":{"message":"PRIVATE_CANARY /private/path","threadId":"t"}});
        assert_eq!(warning_label(&value, Some("t")), "warning_unknown");
        for thread in [json!(null), json!(33), json!(""), json!("private\nvalue")] {
            let value =
                json!({"method":"warning","params":{"message":"synthetic","threadId":thread}});
            assert_eq!(warning_label(&value, Some("t")), "warning_unbound");
        }
        assert_eq!(
            warning_label(
                &json!({"method":"warning","params":{"message":"synthetic"}}),
                None
            ),
            "warning_unbound"
        );
        for params in [
            json!({"message":null,"threadId":"t"}),
            json!({"threadId":"t"}),
            json!({"message":"synthetic","threadId":"t","turnId":"forbidden"}),
        ] {
            assert_eq!(
                warning_label(&json!({"method":"warning","params":params}), Some("t")),
                "warning_malformed"
            );
        }
        assert_eq!(
            warning_label(
                &json!({"method":"warning","params":{"message":"x".repeat(4097),"threadId":"t"}}),
                Some("t")
            ),
            "warning_over_bound"
        );
        assert_eq!(
            warning_label(
                &json!({"method":"warning","params":{"message":"x".repeat(4096),"threadId":"t"}}),
                Some("t")
            ),
            "warning_unknown"
        );
    }
    #[test]
    fn native_unique_decoder_rejects_duplicate_nested_and_escaped_keys() {
        for bytes in [
            br#"{"id":1,"id":1,"result":{}}"#.as_slice(),
            br#"{"id":1,"result":{"a":1,"\u0061":1}}"#,
            br#"{"id":1,"result":[{"a":1,"a":2}]}"#,
            br#"{"id":1,"result":[]"#,
            b"{} {}",
            b"1e400",
        ] {
            assert!(decode_frame(bytes, true).is_err());
        }
        let bytes = br#"{"id":1,"result":{"a":null,"b":[true,false,-1,1.5,"x"]}}"#;
        assert_eq!(decode_frame(bytes, true), decode_frame(bytes, false));
        assert!(decode_frame(br#"{"id":1,"id":2}"#, false).is_ok());
        let deep = format!("{}0{}", "[".repeat(34), "]".repeat(34));
        assert!(decode_frame(deep.as_bytes(), true).is_err());
    }
    #[test]
    fn native_capture_eof_bounds_truncation_and_queue_loss_are_explicit() {
        use std::io::Cursor;
        let (tx, rx) = mpsc::sync_channel(2);
        let capture = Mutex::new(Capture::default());
        read_frames(
            Cursor::new(b"{\"id\":1,\"result\":{}}\n"),
            &tx,
            &capture,
            true,
        );
        assert!(rx.try_recv().is_ok());
        assert!(capture
            .lock()
            .is_ok_and(|c| c.complete().is_ok() && c.frames == 1));
        let (tx, _rx) = mpsc::sync_channel(1);
        let capture = Mutex::new(Capture::default());
        read_frames(Cursor::new(b"{}\n{}\n"), &tx, &capture, true);
        assert!(capture
            .lock()
            .is_ok_and(|c| c.complete() == Err(DirectError::Limit) && !c.eof));
        let (tx, _rx) = mpsc::sync_channel(1);
        let capture = Mutex::new(Capture::default());
        read_frames(Cursor::new(b"{}"), &tx, &capture, true);
        assert!(capture
            .lock()
            .is_ok_and(|c| c.complete().is_err() && !c.eof));
        let mut capture = Capture::default();
        for _ in 0..4 {
            assert!(capture.charge(MAX_FRAME).is_ok());
        }
        assert_eq!(capture.charge(1), Err(DirectError::Limit));
        let mut capture = Capture::default();
        for _ in 0..MAX_FRAMES {
            assert!(capture.charge(1).is_ok());
        }
        assert_eq!(capture.charge(1), Err(DirectError::Limit));
        assert_eq!(Capture::default().complete(), Err(DirectError::Incomplete));
    }
    #[test]
    fn native_tail_rejects_unknown_responses_requests_and_post_terminal_content() {
        let mut state = DerivedState {
            thread: Some("t".into()),
            turn: Some("u".into()),
            ..Default::default()
        };
        for value in [
            json!({"id":1,"result":{}}),
            json!({"id":1,"method":"tool","params":{}}),
            json!({"method":"private_unknown","params":{}}),
            json!({"method":"item/agentMessage/delta","params":{"threadId":"t","turnId":"u","itemId":"i","delta":"late"}}),
            json!({"method":"thread/status/changed","params":{"threadId":"t","status":{"type":"active","activeFlags":[]}}}),
            json!({"method":"warning","params":{"threadId":"t","message":"synthetic"}}),
        ] {
            assert!(validate_tail(&mut state, &value).is_err());
        }
        assert!(validate_tail(&mut state, &json!({"method":"thread/status/changed","params":{"threadId":"t","status":{"type":"idle"}}})).is_ok());
        assert!(validate_tail(
            &mut state,
            &json!({"method":"app/list/updated","params":{"data":[]}})
        )
        .is_ok());
    }
    #[test]
    fn native_warning_all_phases_remain_fatal_without_logging_scope() {
        for phase in [
            "warning_before_response",
            "warning_streaming",
            "warning_shutdown",
        ] {
            let value = json!({"method":"warning","params":{"message":"synthetic","threadId":"t"}});
            assert_eq!(
                reject_warning(&value, Some("t"), phase),
                Err(DirectError::Protocol)
            );
        }
        let mut state = DerivedState {
            thread: Some("t".into()),
            ..Default::default()
        };
        assert_eq!(state.stream(&json!({"method":"warning","params":{"message":WARNING_MESSAGES[0].0,"threadId":"t"}})), Err(DirectError::Protocol));
        assert!(state.turn.is_none());
        for label in OBSERVATION_LABELS {
            assert_eq!(reviewed_rejection_site(label), *label);
        }
    }
    fn moderation_state() -> DerivedState {
        DerivedState {
            authenticated: true,
            deny_all_verified: true,
            model: Some("gpt-5.6-luna".into()),
            thread: Some("t".into()),
            turn: Some("u".into()),
            ..Default::default()
        }
    }
    fn moderation_frame(metadata: Value) -> Value {
        json!({"method":"turn/moderationMetadata","params":{"threadId":"t","turnId":"u","metadata":metadata}})
    }
    #[test]
    fn moderation_metadata_exact_schema_is_opaque_and_thread_turn_bound() {
        for metadata in [
            json!(null),
            json!(true),
            json!(3),
            json!("PRIVATE_CANARY"),
            json!([1, "opaque"]),
            json!({"instructions":"PRIVATE_CANARY","tool":"untrusted"}),
        ] {
            for phase in [
                "notification_phase_stream",
                "notification_phase_before_response",
            ] {
                let mut state = moderation_state();
                let value = moderation_frame(metadata.clone());
                assert_eq!(state.stream_at(&value, phase), Ok(()));
                // The source can forward repeated metadata events. They remain
                // opaque updates bounded by existing frame/capture limits.
                assert_eq!(state.stream_at(&value, phase), Ok(()));
                assert_eq!(state.thread.as_deref(), Some("t"));
                assert_eq!(state.turn.as_deref(), Some("u"));
                assert_eq!(state.model.as_deref(), Some("gpt-5.6-luna"));
                assert!(state.deny_all_verified);
                assert!(validate_tail(&mut state, &value).is_err());
            }
        }
    }
    #[test]
    fn moderation_metadata_rejects_wrong_context_envelope_and_fields() {
        for field in ["threadId", "turnId", "metadata"] {
            let mut value = moderation_frame(json!({}));
            assert!(value["params"]
                .as_object_mut()
                .is_some_and(|p| p.remove(field).is_some()));
            assert!(moderation_state().stream(&value).is_err());
        }
        for field in ["threadId", "turnId"] {
            for wrong in [json!(null), json!(1), json!("other"), json!("bad\n")] {
                let mut value = moderation_frame(json!({}));
                value["params"][field] = wrong;
                assert!(moderation_state().stream(&value).is_err());
            }
        }
        for change in 0..6 {
            let mut state = moderation_state();
            match change {
                0 => state.authenticated = false,
                1 => state.deny_all_verified = false,
                2 => state.model = Some("other".into()),
                3 => state.turn = None,
                4 => state.thread = None,
                _ => state.turn = Some("other".into()),
            };
            assert!(state.stream(&moderation_frame(json!({}))).is_err());
        }
        let mut extra = moderation_frame(json!({}));
        extra["params"]["extra"] = json!(true);
        assert!(moderation_state().stream(&extra).is_err());
        let mut request = moderation_frame(json!({}));
        request["id"] = json!(1);
        assert!(moderation_state().stream(&request).is_err());
        for method in [
            "PRIVATE_CANARY",
            "item/mcpToolCall/progress",
            "model/rerouted",
            "model/verification",
        ] {
            let mut value = moderation_frame(json!({}));
            value["method"] = json!(method);
            assert!(moderation_state().stream(&value).is_err());
        }
    }
    #[test]
    fn notification_method_diagnostic_is_closed_and_payload_private() {
        assert_eq!(NOTIFICATION_METHOD_LABELS.len(), 84);
        for (method, label) in NOTIFICATION_METHOD_LABELS {
            let value = json!({"method":method,"params":{"PRIVATE_CANARY":"private/path"}});
            assert_eq!(notification_method_site(&value), *label);
            assert_eq!(reviewed_rejection_site(label), *label);
            assert!(!label.contains("PRIVATE_CANARY"));
        }
        for method in [
            json!("PRIVATE_CANARY"),
            json!("thread/tokenUsage/updated\n"),
            json!("x".repeat(4097)),
        ] {
            assert_eq!(
                notification_method_site(&json!({"method":method})),
                "unknown_notification_method"
            );
        }
        assert_eq!(
            notification_method_site(&json!({})),
            "notification_method_missing"
        );
        for method in [json!(null), json!(1), json!({"PRIVATE_CANARY":true})] {
            assert_eq!(
                notification_method_site(&json!({"method":method})),
                "notification_method_non_string"
            );
        }
    }
    #[test]
    fn notification_diagnostic_preserves_rejection_in_all_phases() {
        for method in [
            "turn/moderationMetadata",
            "model/verification",
            "item/mcpToolCall/progress",
            "PRIVATE_CANARY",
        ] {
            let value = json!({"method":method,"params":{"threadId":"t","turnId":"u","PRIVATE_CANARY":"private/path"}});
            for phase in [
                "notification_phase_before_response",
                "notification_phase_stream",
                "notification_phase_shutdown",
            ] {
                assert_eq!(
                    reject_notification(&value, DirectError::CodexIsolation, phase),
                    DirectError::CodexIsolation
                );
                let mut state = DerivedState {
                    thread: Some("t".into()),
                    turn: Some("u".into()),
                    ..Default::default()
                };
                assert!(state.stream_at(&value, phase).is_err());
                assert!(validate_tail(&mut state, &value).is_err());
            }
        }
    }
    #[test]
    fn stream_turn_diagnostic_exact_matrix_preserves_errors() {
        let cases = [
            None,
            Some(json!(null)),
            Some(json!({"PRIVATE_CANARY":"private/path"})),
            Some(json!("")),
            Some(json!("x".repeat(257))),
            Some(json!("private\nvalue")),
        ];
        for (method, labels) in STREAM_TURN_ID_LABELS {
            for (shape, input) in cases.iter().enumerate() {
                let mut value = json!({"method":method,"params": {"threadId":"synthetic"}});
                if let Some(input) = input {
                    value["params"]["turnId"] = input.clone();
                }
                assert_eq!(stream_turn_id_site(&value), labels[shape]);
                assert_eq!(reviewed_rejection_site(labels[shape]), labels[shape]);
                assert_eq!(
                    stream_turn_identifier(&value),
                    identifier(&value["params"]["turnId"])
                );
            }
            for input in [json!("synthetic"), json!("x".repeat(256))] {
                let value = json!({"method":method,"params":{"turnId":input}});
                assert_eq!(
                    stream_turn_identifier(&value),
                    identifier(&value["params"]["turnId"])
                );
                assert_eq!(stream_turn_id_site(&value), "stream_turn_id_unclassified");
            }
        }
    }
    #[test]
    fn stream_turn_diagnostic_unknowns_are_fixed_not_values() {
        for value in [
            json!({}),
            json!({"method":"PRIVATE_CANARY","params":{"turnId":"private/path"}}),
            json!({"method":null}),
            json!({"method":31}),
        ] {
            assert_eq!(stream_turn_id_site(&value), "stream_turn_id_unknown_method");
        }
        assert_eq!(reviewed_rejection_site("PRIVATE_CANARY"), "unreviewed_site");
        assert_eq!(STREAM_TURN_ID_LABELS.len(), 84);
    }
    #[test]
    fn rejection_sites_preserve_strict_closed_results_and_fixed_reasons() {
        let schema = &REJECTION_SCHEMAS[0];
        for value in [
            json!(null),
            json!([]),
            json!({}),
            json!({"known":null}),
            json!({"known":null,"PRIVATE_CANARY":"private/path"}),
        ] {
            assert_eq!(
                closed_at(schema, &value, &["known"], &[]),
                closed(&value, &["known"], &[])
            );
        }
        assert_eq!(
            closed_reason(schema, &json!(null), &["known"]),
            schema.shape
        );
        assert_eq!(
            closed_reason(schema, &json!({}), &["known"]),
            schema.missing[0]
        );
        assert_eq!(
            closed_reason(
                schema,
                &json!({"known":null,"PRIVATE_CANARY":1}),
                &["known"]
            ),
            schema.unknown
        );
        assert_eq!(reviewed_rejection_site("PRIVATE_CANARY"), "unreviewed_site");
        assert_eq!(request_receive_site("PRIVATE_CANARY"), "unreviewed_site");
    }
    #[test]
    fn rejection_sites_first_only_preserves_primary_and_privacy(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::diagnostics::{Attempt, Event, Logger};
        let directory = tempfile::tempdir()?;
        let log = Logger::open(directory.path().join("diagnostics"));
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(
                REJECTION_RECORDED.scope(std::cell::Cell::new(false), async {
                    for error in [DirectError::Protocol, DirectError::CodexAuthentication] {
                        assert_eq!(
                            rejection_error_using(STREAM_TURN_ID_LABELS[0].1[0], error, |label| {
                                Attempt::with_logger(
                                    Some(log.clone()),
                                    crate::agent_preferences::AgentConnection::Codex,
                                    "",
                                    None,
                                    None,
                                    Some(label),
                                )
                            }),
                            error
                        );
                    }
                }),
            );
        let rows = log.snapshot();
        assert_eq!(
            rows.events
                .iter()
                .filter(|r| r.event == Event::RequestFinished)
                .count(),
            1
        );
        let text = serde_json::to_string(&rows)?;
        assert!(!text.contains("PRIVATE_CANARY"));
        assert!(!text.contains("private/path"));
        assert!(log.flush(true));
        Ok(())
    }
    #[test]
    fn rejection_sites_writer_unavailable_and_no_scope_preserve_errors(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let called = std::cell::Cell::new(false);
        assert_eq!(
            rejection_error_using("unreviewed_site", DirectError::Protocol, |label| {
                called.set(true);
                crate::diagnostics::Attempt::with_logger(
                    None,
                    crate::agent_preferences::AgentConnection::Codex,
                    "",
                    None,
                    None,
                    Some(label),
                )
            }),
            DirectError::Protocol
        );
        assert!(!called.get());
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("not_directory");
        std::fs::write(&path, b"synthetic")?;
        let log = crate::diagnostics::Logger::open(path);
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(
                REJECTION_RECORDED.scope(std::cell::Cell::new(false), async {
                    assert_eq!(
                        rejection_error_using("unreviewed_site", DirectError::Protocol, |label| {
                            crate::diagnostics::Attempt::with_logger(
                                Some(log.clone()),
                                crate::agent_preferences::AgentConnection::Codex,
                                "",
                                None,
                                None,
                                Some(label),
                            )
                        }),
                        DirectError::Protocol
                    );
                }),
            );
        Ok(())
    }
    fn derived_snapshot() -> Value {
        json!({"id":"thread-derived","cliVersion":"0.162.0-alpha.17.2","createdAt":1,"cwd":"/synthetic","ephemeral":true,"modelProvider":"openai","preview":"","projectId":null,"sessionId":"session","source":"appServer","status":{"type":"idle"},"turns":[],"updatedAt":1,"environments":[],"model":"gpt-5.6-luna"})
    }
    fn derived_response() -> Value {
        json!({"thread":derived_snapshot(),"model":"gpt-5.6-luna","modelProvider":"openai","approvalPolicy":"never","approvalsReviewer":"user","cwd":"/synthetic","sandbox":{"type":"readOnly","networkAccess":false},"denyAllTools":true,"runtimeWorkspaceRoots":[],"instructionSources":[]})
    }
    #[test]
    fn derived_alpha_does_not_expand_legacy_admission() -> Result<(), Box<dyn std::error::Error>> {
        let v = json!({"userAgent":"cortexa/0.162.0-alpha.17.2 fixture","codexHome":"/synthetic","platformOs":"macos","platformFamily":"unix"});
        assert_eq!(validate_runtime_identity(&v), Err(DirectError::CodexSetup));
        assert_eq!(
            derived_identity(&v, std::path::Path::new("/synthetic")),
            Ok(())
        );
        let mut wrong = v.clone();
        wrong["userAgent"] = json!("cortexa/0.162.0-alpha.17.3 fixture");
        assert_eq!(
            derived_identity(&wrong, std::path::Path::new("/synthetic")),
            Err(DirectError::CodexSetup)
        );
        assert_eq!(
            derived_identity(&v, std::path::Path::new("/other")),
            Err(DirectError::CodexSetup)
        );
        let dir = tempfile::tempdir()?;
        let file = dir.path().join("not-runtime");
        std::fs::write(&file, b"synthetic")?;
        assert!(!derived_artifact(&file)?);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&file, dir.path().join("link"))?;
            assert!(!derived_artifact(&dir.path().join("link"))?);
        }
        Ok(())
    }
    #[test]
    fn derived_flags_preserve_clear_environment_and_disable_model_forced_tools() {
        let setup = Setup {
            executable: "/synthetic".into(),
            home: "/synthetic/auth".into(),
            derived: true,
            verified_derived: Mutex::new(None),
            cancellation: CancellationHandle::default(),
        };
        let c = command(&setup, std::path::Path::new("/synthetic/home"));
        let args: Vec<_> = c
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        for flag in FEATURES.iter().chain(DERIVED_FEATURES.iter()) {
            assert!(args.contains(&format!("features.{flag}=false")));
        }
        for flag in [
            "tools.experimental_request_user_input.enabled=false",
            "tools.update_plan.enabled=false",
        ] {
            assert!(args.iter().any(|a| a == flag));
        }
        assert_eq!(c.get_envs().filter(|(_, v)| v.is_some()).count(), 3);
    }
    #[test]
    fn derived_thread_ack_and_isolation_are_mandatory() -> Result<(), DirectError> {
        let good = derived_response();
        assert_eq!(
            derived_thread_response(&good, "gpt-5.6-luna"),
            Ok("thread-derived".into())
        );
        for (pointer, value) in [
            ("/denyAllTools", json!(false)),
            ("/denyAllTools", Value::Null),
            ("/sandbox/networkAccess", json!(true)),
            ("/runtimeWorkspaceRoots", json!(["/private"])),
            ("/instructionSources", json!(["PRIVATE"])),
            ("/thread/ephemeral", json!(false)),
            ("/thread/turns", json!([null])),
            ("/thread/environments", json!([null])),
            ("/model", json!("gpt-6")),
        ] {
            let mut bad = good.clone();
            *bad.pointer_mut(pointer).ok_or(DirectError::Internal)? = value;
            assert!(derived_thread_response(&bad, "gpt-5.6-luna").is_err());
        }
        let mut unknown = good.clone();
        unknown["unreviewed"] = json!(true);
        assert_eq!(
            derived_thread_response(&unknown, "gpt-5.6-luna"),
            Err(DirectError::Protocol)
        );
        Ok(())
    }
    #[test]
    fn derived_notifications_require_bound_identity_and_do_not_authenticate(
    ) -> Result<(), DirectError> {
        let mut state = DerivedState::default();
        assert!(state.benign(&json!({"method":"remoteControl/status/changed","params":{"installationId":"synthetic","serverName":"synthetic","status":"disabled"}}))?);
        assert!(state.benign(
            &json!({"method":"account/updated","params":{"authMode":"chatgpt","planType":"plus"}})
        )?);
        assert!(!state.authenticated);
        let status = json!({"method":"thread/status/changed","params":{"threadId":"thread-derived","status":{"type":"idle"}}});
        assert!(state.benign(&status).is_err());
        state.model = Some("gpt-5.6-luna".into());
        assert!(state
            .benign(&json!({"method":"thread/started","params":{"thread":derived_snapshot()}}))?);
        assert!(state.benign(&status)?);
        let active = json!({"method":"thread/status/changed","params":{"threadId":"thread-derived","status":{"type":"active","activeFlags":[]}}});
        assert!(state.benign(&active).is_err());
        state.awaiting_turn = true;
        assert!(state.benign(&active)?);
        state.authenticated = true;
        assert_eq!(
            state.benign(&json!({"method":"account/updated","params":{"authMode":null}})),
            Err(DirectError::CodexAuthentication)
        );
        Ok(())
    }
    #[test]
    fn derived_account_routing_schema_is_validated_without_authority() -> Result<(), DirectError> {
        let base = json!({"requiresOpenaiAuth":true,"account":{"type":"chatgpt","email":null,"planType":"plus"}});
        derived_account(&base)?;
        let mut value = base.clone();
        value["workspaceRouting"] = Value::Null;
        // Reproduce the old closed-shape rejection with synthetic data only.
        assert_eq!(
            closed(&value, &["account", "requiresOpenaiAuth"], &[]),
            Err(DirectError::Protocol)
        );
        derived_account(&value)?;
        for policy in ["NO_CONSTRAINT", "us", "us_cr"] {
            value["workspaceRouting"] = json!({"chatgptAccountId":"synthetic-account","backendOrigin":"https://example.invalid","accountRoutingOverride":policy});
            derived_account(&value)?;
        }
        for routing in [
            json!([]),
            json!({}),
            json!({"chatgptAccountId":7,"backendOrigin":"synthetic","accountRoutingOverride":"us"}),
            json!({"chatgptAccountId":"synthetic","backendOrigin":false,"accountRoutingOverride":"us"}),
            json!({"chatgptAccountId":"synthetic","backendOrigin":"synthetic","accountRoutingOverride":"unknown"}),
            json!({"chatgptAccountId":"synthetic","backendOrigin":"synthetic","accountRoutingOverride":"us","unreviewed":true}),
        ] {
            value["workspaceRouting"] = routing;
            assert_eq!(derived_account(&value), Err(DirectError::Protocol));
        }
        value = base.clone();
        value["unreviewed"] = json!(true);
        assert_eq!(derived_account(&value), Err(DirectError::Protocol));
        for account in [Value::Null, json!({"type":"apiKey"})] {
            value = base.clone();
            value["account"] = account;
            assert_eq!(
                derived_account(&value),
                Err(DirectError::CodexAuthentication)
            );
        }
        value = base.clone();
        value["requiresOpenaiAuth"] = json!(false);
        assert_eq!(
            derived_account(&value),
            Err(DirectError::CodexAuthentication)
        );
        value = base.clone();
        value["requiresOpenaiAuth"] = json!("true");
        assert_eq!(derived_account(&value), Err(DirectError::Protocol));
        for (key, bad) in [
            ("email", json!(7)),
            ("planType", json!("unreviewed")),
            ("unexpected", json!(true)),
        ] {
            value = base.clone();
            value["account"][key] = bad;
            assert_eq!(derived_account(&value), Err(DirectError::Protocol));
        }
        value = base;
        value["workspaceRouting"] = json!({"chatgptAccountId":"x".repeat(65_537),"backendOrigin":"synthetic","accountRoutingOverride":"us"});
        assert_eq!(derived_account(&value), Err(DirectError::Limit));
        Ok(())
    }
    #[test]
    fn derived_unknown_requests_warnings_and_tools_are_fatal() {
        let mut state = DerivedState {
            thread: Some("t".into()),
            turn: Some("u".into()),
            ..DerivedState::default()
        };
        for method in [
            "unreviewed/event",
            "configWarning",
            "deprecationNotice",
            "model/rerouted",
            "hook/started",
            "mcpServer/status/updated",
        ] {
            assert!(state
                .stream(&json!({"method":method,"params":{"threadId":"t","turnId":"u"}}))
                .is_err());
        }
        assert!(derived_envelope(&json!({"id":5,"method":"tool/call","params":{}})).is_err());
        assert!(derived_envelope(&json!({"id":5,"result":{},"error":{}})).is_err());
        for kind in [
            "commandExecution",
            "mcpToolCall",
            "dynamicToolCall",
            "webSearch",
            "fileChange",
            "unknown",
        ] {
            assert_eq!(
                validate_text_item(&json!({"id":"i","type":kind})),
                Err(DirectError::CodexIsolation)
            );
        }
    }
    #[test]
    fn derived_stream_identity_numeric_and_input_bounds_fail_closed() -> Result<(), DirectError> {
        let mut state = DerivedState {
            thread: Some("t".into()),
            turn: Some("u".into()),
            ..DerivedState::default()
        };
        let good = json!({"method":"item/agentMessage/delta","params":{"threadId":"t","turnId":"u","itemId":"i","delta":"synthetic"}});
        state.stream(&good)?;
        for (key, value) in [
            ("threadId", json!("other")),
            ("turnId", json!("other")),
            ("itemId", json!("")),
            ("delta", json!(0)),
        ] {
            let mut bad = good.clone();
            bad["params"][key] = value;
            assert!(state.stream(&bad).is_err());
        }
        assert_eq!(derived_input("rules", "input")?, "rules\n\ninput");
        assert_eq!(
            derived_input("rules", &"x".repeat(16_384)),
            Err(DirectError::Limit)
        );
        assert!(
            derived_envelope(&json!({"method":"status","params":{},"emittedAtMs":-1})).is_err()
        );
        Ok(())
    }

    #[test]
    fn runtime_identity_accepts_only_reviewed_versions_and_rejects_untrusted_values() {
        for version in ["0.159.0", "0.160.1"] {
            assert_eq!(
                validate_runtime_identity(
                    &json!({"userAgent":format!("cortexa/{version} fixture")})
                ),
                Ok(())
            );
        }
        for value in [
            Value::Null,
            json!(1),
            json!("cortexa/0.160.10 fixture"),
            json!("cortexa/0.160.1-preview fixture"),
            json!("other/0.160.1 fixture"),
            json!("cortexa/0.160.1"),
            json!("cortexa/0.160.1 fixture\nPRIVATE"),
            json!(format!("cortexa/0.160.1 {}", "x".repeat(512))),
        ] {
            assert_eq!(
                validate_runtime_identity(&json!({"userAgent":value})),
                Err(DirectError::CodexSetup)
            );
        }
    }

    #[test]
    fn catalog_pagination_rejects_cycles_malformed_cursors_and_total_overflow(
    ) -> Result<(), DirectError> {
        let mut rows = Vec::new();
        let mut cursors = Vec::new();
        assert_eq!(
            append_catalog_page(
                &json!({"data":[],"nextCursor":"opaque"}),
                &mut rows,
                &mut cursors
            )?,
            Some("opaque".into())
        );
        for value in [
            json!({"data":[],"nextCursor":"opaque"}),
            json!({"data":[],"nextCursor":""}),
            json!({"data":[],"nextCursor":4}),
            json!({"data":[]}),
            json!({"data":null,"nextCursor":null}),
            json!({"data":[],"nextCursor":"unsafe\n"}),
            json!({"data":[],"nextCursor":"x".repeat(257)}),
        ] {
            assert_eq!(
                append_catalog_page(&value, &mut rows, &mut cursors),
                Err(DirectError::Catalog)
            );
        }
        rows.resize(128, Value::Null);
        assert_eq!(
            append_catalog_page(
                &json!({"data":[null],"nextCursor":null}),
                &mut rows,
                &mut cursors
            ),
            Err(DirectError::Catalog)
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn offline_catalog_pages_use_unique_rpc_ids_and_fail_closed_at_page_cap(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        for (mode, success) in [
            ("pages", true),
            ("page-loop", false),
            ("page-limit", false),
            ("page-duplicate", false),
        ] {
            let (_directory, setup) = fixture(mode)?;
            let mut session = Session::spawn(&setup)?;
            let result = runtime.block_on(async {
                tokio::time::timeout(Duration::from_secs(5), async {
                    session.initialize().await?;
                    session.catalog().await
                })
                .await
            })?;
            assert_eq!(result.is_ok(), success);
            if let Ok(models) = result {
                assert_eq!(models.len(), 2);
                assert_eq!(models[0].id, "fixture-model");
                assert_eq!(models[1].id, "fixture-second");
            } else {
                assert_eq!(result.err(), Some(DirectError::Catalog));
            }
        }
        Ok(())
    }

    #[test]
    fn isolation_parameters_and_clean_environment_are_closed() {
        let setup = Setup {
            executable: "/synthetic/codex".into(),
            home: "/synthetic/auth".into(),
            derived: false,
            verified_derived: Mutex::new(None),
            cancellation: CancellationHandle::default(),
        };
        let command = command(&setup, std::path::Path::new("/synthetic/empty"));
        let keys: Vec<_> = command
            .get_envs()
            .filter_map(|(k, v)| v.map(|_| k.to_string_lossy().into_owned()))
            .collect();
        assert_eq!(keys, ["CODEX_HOME", "HOME", "PATH"]);
        assert_eq!(command.get_program(), "/synthetic/codex");
        assert_eq!(
            command.get_current_dir(),
            Some(std::path::Path::new("/synthetic/empty"))
        );
        let environment: Vec<_> = command
            .get_envs()
            .map(|(key, value)| (key.to_str(), value.and_then(|v| v.to_str())))
            .collect();
        assert_eq!(
            environment,
            [
                (Some("CODEX_HOME"), Some("/synthetic/auth")),
                (Some("HOME"), Some("/synthetic/empty")),
                (Some("PATH"), Some("/usr/bin:/bin")),
            ]
        );
        let arguments: Vec<_> = command
            .get_args()
            .map(|v| v.to_string_lossy().into_owned())
            .collect();
        let mut expected = vec![
            "app-server".to_owned(),
            "--stdio".to_owned(),
            "--strict-config".to_owned(),
            "-c".to_owned(),
            "web_search=\"disabled\"".to_owned(),
            "-c".to_owned(),
            "cli_auth_credentials_store=\"file\"".to_owned(),
        ];
        for feature in FEATURES {
            expected.extend(["-c".to_owned(), format!("features.{feature}=false")]);
        }
        assert_eq!(arguments, expected);

        let p = thread_parameters("model");
        for key in [
            "environments",
            "dynamicTools",
            "runtimeWorkspaceRoots",
            "selectedCapabilityRoots",
        ] {
            assert_eq!(p[key], json!([]));
        }
        assert_eq!(p["approvalPolicy"], "never");
        assert_eq!(p["allowProviderModelFallback"], false);
    }
    #[test]
    fn runtime_drift_and_tools_fail_closed_without_payloads() {
        for kind in [
            "commandExecution",
            "fileChange",
            "mcpToolCall",
            "webSearch",
            "dynamicToolCall",
        ] {
            assert_eq!(
                reject_tool(
                    &json!({"method":"item/started","params":{"item":{"type":kind,"secret":"sentinel"}}})
                ),
                Err(DirectError::CodexIsolation)
            );
        }
        assert_eq!(
            thread_id(
                &json!({"model":"model","sandbox":{"type":"dangerFullAccess"},"secret":"sentinel"}),
                "model"
            ),
            Err(DirectError::CodexIsolation)
        );
        for error in [
            DirectError::CodexSetup,
            DirectError::CodexRuntime,
            DirectError::CodexAuthentication,
            DirectError::CodexIsolation,
        ] {
            assert!(!format!("{error:?} {error}").contains("sentinel"));
        }
    }
    #[test]
    fn discovered_models_only_expose_recognized_supported_efforts() -> Result<(), DirectError> {
        let result = models(
            &json!({"nextCursor":null,"data":[{"model":"gpt-runtime","hidden":false,"supportedReasoningEfforts":[{"reasoningEffort":"low"},{"reasoningEffort":"high"},{"reasoningEffort":"future"}]}]}),
        )?;
        assert_eq!(
            result[0].efforts,
            vec![
                ReasoningEffort::Default,
                ReasoningEffort::Low,
                ReasoningEffort::High
            ]
        );
        assert_eq!(
            crate::agent_models::validate_selection(&result[0], ReasoningEffort::Max),
            Err(DirectError::Unsupported)
        );
        assert!(models(&json!({"data":[],"nextCursor":"more"})).is_err());
        Ok(())
    }
    #[cfg(unix)]
    static FIXTURE_LOCK: Mutex<()> = Mutex::new(());
    #[cfg(unix)]
    struct Fixture {
        directory: tempfile::TempDir,
        _guard: std::sync::MutexGuard<'static, ()>,
    }
    #[cfg(unix)]
    impl Fixture {
        fn path(&self) -> &std::path::Path {
            self.directory.path()
        }
    }
    #[cfg(unix)]
    fn fixture(mode: &str) -> Result<(Fixture, Setup), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;
        let guard = FIXTURE_LOCK.lock().map_err(|_| "fixture lock")?;
        let directory = tempfile::tempdir()?;
        let executable = directory.path().join("fixture");
        let python = if cfg!(target_os = "macos") {
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3"
        } else {
            "/usr/bin/python3"
        };
        let script = r#"
import sys,json,time,os
mode=MODE
assert set(os.environ).issubset({'HOME','CODEX_HOME','PATH','LC_CTYPE','__CF_USER_TEXT_ENCODING'})
def out(v): print(json.dumps(v),flush=True)
for line in sys.stdin:
 v=json.loads(line);m=v['method'];i=v.get('id')
 if m=='initialized': continue
 if m=='initialize':
  if mode=='pending-initialize': time.sleep(30)
  if mode=='invalid-frame': print('{',flush=True);time.sleep(30)
  result={'userAgent':'cortexa/0.160.1 fixture'}
 elif m=='account/read': result={'account':None if mode=='unauthenticated' else {'type':'chatgpt'}}
 elif m=='model/list':
  page=i-3;assert 0<=page<4
  assert v['params'].get('cursor')==(None if page==0 else 'cursor-'+str(page))
  name='fixture-second' if mode=='pages' and page>0 else 'fixture-model'
  result={'nextCursor':None,'data':[{'model':name,'hidden':False,'supportedReasoningEfforts':[{'reasoningEffort':'high'}]}]}
  if mode in ['pages','page-duplicate'] and page==0: result['nextCursor']='cursor-1'
  if mode=='page-loop': result['nextCursor']='cursor-1'
  if mode=='page-limit': result['nextCursor']='cursor-'+str(page+1);result['data']=[]
 elif m=='thread/start':
  p=v['params'];assert all(p[k]==[] for k in ['environments','runtimeWorkspaceRoots','selectedCapabilityRoots','dynamicTools']);assert p['approvalPolicy']=='never'
  result={'model':'fixture-model','modelProvider':'openai','approvalPolicy':'never','sandbox':{'type':'readOnly','networkAccess':False},'runtimeWorkspaceRoots':[],'instructionSources':[],'thread':{'id':'thread-fixture'}}
 elif m=='turn/start':
  assert v['params']['effort']=='high';assert v['params']['environments']==[]
  result={'turn':{'id':'turn-fixture'}}
 else: raise RuntimeError('closed fixture protocol mismatch')
 out({'id':i,'result':result})
 if m=='turn/start':
  if mode=='pending': time.sleep(30)
  elif mode=='tool': out({'method':'item/started','params':{'item':{'type':'commandExecution','raw':'secret-sentinel'}}})
  elif mode=='error': out({'method':'error','params':{'threadId':'thread-fixture','error':{'message':'secret-sentinel'}}})
  else:
   out({'method':'item/agentMessage/delta','params':{'threadId':'thread-fixture','turnId':'turn-fixture','delta':'Synthetic answer.'}})
   out({'method':'turn/completed','params':{'threadId':'thread-fixture','turn':{'id':'turn-fixture','status':'completed'}}})
"#;
        std::fs::write(
            &executable,
            format!(
                "#!{python}\n{}",
                script.replace("MODE", &serde_json::to_string(mode)?)
            ),
        )?;
        std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700))?;
        let setup = Setup {
            executable,
            home: directory.path().into(),
            derived: false,
            verified_derived: Mutex::new(None),
            cancellation: CancellationHandle::default(),
        };
        Ok((
            Fixture {
                directory,
                _guard: guard,
            },
            setup,
        ))
    }
    #[cfg(unix)]
    #[test]
    fn fixture_custody_releases_directory_before_guard() -> Result<(), Box<dyn std::error::Error>> {
        static LOCAL: Mutex<()> = Mutex::new(());
        let directory = tempfile::tempdir()?;
        let path = directory.path().to_path_buf();
        let fixture = Fixture {
            directory,
            _guard: LOCAL.lock().map_err(|_| "fixture lock")?,
        };
        assert_eq!(fixture.path(), path.as_path());
        assert!(matches!(
            LOCAL.try_lock(),
            Err(std::sync::TryLockError::WouldBlock)
        ));
        drop(fixture);
        assert!(!path.exists());
        assert!(LOCAL.try_lock().is_ok());
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn offline_app_server_streams_completes_and_rejects_tools_and_raw_errors(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        for (mode, expected) in [
            ("success", None),
            ("tool", Some(DirectError::CodexIsolation)),
            ("error", Some(DirectError::CodexRuntime)),
        ] {
            let (_directory, setup) = fixture(mode)?;
            let mut text = String::new();
            let mut complete = false;
            let result = runtime.block_on(async {
                tokio::time::timeout(Duration::from_secs(5), async {
                    run(
                        setup,
                        "fixture-model".into(),
                        ReasoningEffort::High,
                        "synthetic".into(),
                        |event| {
                            match event {
                                ProviderEvent::Delta(delta) => text.push_str(&delta),
                                ProviderEvent::Completed => complete = true,
                                ProviderEvent::Started(_) => {}
                            }
                            Ok(())
                        },
                    )
                    .await
                })
                .await
            });
            assert_eq!(result?, expected.map_or(Ok(()), Err));
            assert_eq!(complete, expected.is_none());
            assert!(!text.contains("secret-sentinel"));
            if complete {
                assert_eq!(text, "Synthetic answer.");
            }
        }
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn diagnostics_capture_child_lifecycle_without_subprocess_content(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::diagnostics::{Attempt, Event, Logger, Outcome, CURRENT};
        let (directory, setup) = fixture("error")?;
        let log = Logger::open(directory.path().join("diagnostics"));
        let mut attempt = Attempt::with_logger(
            Some(log.clone()),
            crate::agent_preferences::AgentConnection::Codex,
            "fixture-model",
            Some("chat-1"),
            None,
            None,
        );
        let observer = attempt.observer();
        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(CURRENT.scope(
                observer,
                run(
                    setup,
                    "fixture-model".into(),
                    ReasoningEffort::High,
                    "PRIVATE_PROMPT_CANARY".into(),
                    |_| Ok(()),
                ),
            ));
        assert_eq!(result, Err(DirectError::CodexRuntime));
        attempt.finish(result);
        let snapshot = log.snapshot();
        assert!(snapshot
            .events
            .iter()
            .any(|r| r.event == Event::RuntimeStarted));
        assert!(snapshot
            .events
            .iter()
            .any(|r| r.event == Event::RuntimeExited && r.outcome == Outcome::Completed));
        assert!(snapshot
            .events
            .iter()
            .any(|r| r.event == Event::RuntimeCleanup && r.outcome == Outcome::Completed));
        let encoded = serde_json::to_string(&snapshot)?;
        assert!(!encoded.contains("PRIVATE_PROMPT_CANARY"));
        assert!(!encoded.contains("secret-sentinel"));
        assert!(!encoded.contains("fixture-model"));
        assert!(log.flush(true));
        Ok(())
    }
    #[test]
    fn derived_account_malformed_is_not_authentication_failure() {
        for account in [
            json!({}),
            json!([]),
            json!(7),
            json!({"type":"unknown"}),
            json!({"type":"apiKey","extra":"PRIVATE_CANARY"}),
            json!({"type":"chatgpt","email":null}),
        ] {
            assert_eq!(
                derived_account(&json!({"requiresOpenaiAuth":true,"account":account})),
                Err(DirectError::Protocol)
            );
        }
        for value in [
            json!({"requiresOpenaiAuth":true}),
            json!({"requiresOpenaiAuth":true,"account":null}),
            json!({"requiresOpenaiAuth":true,"account":{"type":"apiKey"}}),
        ] {
            assert_eq!(
                derived_account(&value),
                Err(DirectError::CodexAuthentication)
            );
        }
        assert_eq!(
            derived_account(
                &json!({"requiresOpenaiAuth":false,"account":{"type":"chatgpt","email":7,"planType":"plus"}})
            ),
            Err(DirectError::Protocol)
        );
    }
    #[cfg(unix)]
    #[test]
    fn catalog_diagnostics_orders_fixed_phases_without_dispatch_or_content(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::diagnostics::{Attempt, Event, Logger, Outcome};
        let (directory, setup) = fixture("pages")?;
        let log = Logger::open(directory.path().join("diagnostics"));
        let attempt = Attempt::with_logger(
            Some(log.clone()),
            crate::agent_preferences::AgentConnection::Codex,
            "",
            None,
            None,
            None,
        );
        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(discover_observed(|| Ok(setup), attempt))?;
        assert_eq!(result.len(), 2);
        let snapshot = log.snapshot();
        assert_eq!(
            snapshot.events.iter().map(|r| r.event).collect::<Vec<_>>(),
            vec![
                Event::RequestStarted,
                Event::ConfigurationValidated,
                Event::RuntimeStarted,
                Event::StageStarted,
                Event::StageFinished,
                Event::CredentialsAvailable,
                Event::FirstResponse,
                Event::RuntimeExited,
                Event::RuntimeCleanup,
                Event::RequestFinished
            ]
        );
        assert_eq!(
            snapshot.events.last().map(|r| r.outcome),
            Some(Outcome::Completed)
        );
        assert!(snapshot.events.iter().all(|r| r.attempt.is_none()
            && r.conversation.is_none()
            && r.workflow.is_none()
            && r.stage.is_none()
            && r.duration_ms.is_some()));
        let encoded = serde_json::to_string(&snapshot)?;
        for secret in [
            "fixture-model",
            "fixture-second",
            "secret-sentinel",
            "PRIVATE_CANARY",
        ] {
            assert!(!encoded.contains(secret));
        }
        assert!(log.flush(true));
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn catalog_diagnostics_preserves_reader_and_authentication_failures_after_cleanup(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::diagnostics::{Attempt, Category, Event, Logger, Outcome};
        for (mode, expected, category) in [
            (
                "invalid-frame",
                DirectError::Protocol,
                Category::MalformedStream,
            ),
            (
                "unauthenticated",
                DirectError::CodexAuthentication,
                Category::Authentication,
            ),
        ] {
            let (directory, setup) = fixture(mode)?;
            let log = Logger::open(directory.path().join("diagnostics"));
            let attempt = Attempt::with_logger(
                Some(log.clone()),
                crate::agent_preferences::AgentConnection::Codex,
                "",
                None,
                None,
                None,
            );
            let result = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?
                .block_on(discover_observed(|| Ok(setup), attempt));
            assert!(matches!(result, Err(e) if e==expected));
            let snapshot = log.snapshot();
            assert!(snapshot
                .events
                .iter()
                .any(|r| r.event == Event::RuntimeCleanup && r.outcome == Outcome::Completed));
            assert!(!snapshot.events.iter().any(|r| matches!(
                r.event,
                Event::ProviderDispatch | Event::FirstResponse | Event::CredentialsAvailable
            )));
            assert!(snapshot
                .events
                .iter()
                .any(|r| r.event == Event::RequestFinished
                    && r.outcome == Outcome::Failed
                    && r.error == Some(category)));
            assert!(log.flush(true));
        }
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn catalog_diagnostics_cancel_is_not_authentication_or_inferred_timeout(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::diagnostics::{Attempt, Event, Logger, Outcome};
        let (directory, setup) = fixture("pending-initialize")?;
        let log = Logger::open(directory.path().join("diagnostics"));
        let attempt = Attempt::with_logger(
            Some(log.clone()),
            crate::agent_preferences::AgentConnection::Codex,
            "",
            None,
            None,
            None,
        );
        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                tokio::time::timeout(
                    Duration::from_millis(200),
                    discover_observed(|| Ok(setup), attempt),
                )
                .await
            });
        assert!(result.is_err());
        let snapshot = log.snapshot();
        assert!(snapshot
            .events
            .iter()
            .any(|r| r.event == Event::StageStarted));
        assert!(!snapshot.events.iter().any(|r| matches!(
            r.event,
            Event::StageFinished
                | Event::CredentialsAvailable
                | Event::FirstResponse
                | Event::ProviderDispatch
        )));
        assert!(snapshot
            .events
            .iter()
            .any(|r| r.event == Event::RuntimeCleanup && r.outcome == Outcome::Completed));
        assert!(snapshot
            .events
            .iter()
            .any(|r| r.event == Event::RequestFinished
                && r.outcome == Outcome::Cancelled
                && r.error.is_none()));
        assert!(log.flush(true));
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn catalog_diagnostics_unavailable_logger_preserves_primary_error(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::diagnostics::{Attempt, Logger};
        let (directory, setup) = fixture("invalid-frame")?;
        let path = directory.path().join("unavailable");
        std::fs::write(&path, b"synthetic")?;
        let log = Logger::open(path);
        assert!(!log.snapshot().available);
        let attempt = Attempt::with_logger(
            Some(log),
            crate::agent_preferences::AgentConnection::Codex,
            "",
            None,
            None,
            None,
        );
        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(discover_observed(|| Ok(setup), attempt));
        assert!(matches!(result, Err(DirectError::Protocol)));
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn cancelled_session_reaps_child_and_removes_only_owned_directory(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, setup) = fixture("pending")?;
        let mut session = Session::spawn(&setup)?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        runtime.block_on(session.initialize())?;
        let path = session.directory.clone();
        let pid = session.child.lock().map_err(|_| "child lock")?.id();
        drop(session);
        assert!(!path.exists());
        let alive = Command::new("/bin/kill")
            .args(["-0", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        assert!(!alive.success());
        assert!(setup.home.exists());
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn exit_cleanup_reaps_registered_child_without_waiting_for_future_drop(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, setup) = fixture("pending")?;
        let session = Session::spawn(&setup)?;
        assert!(reap_children(&[Arc::downgrade(&session.child)]));
        assert!(session
            .child
            .lock()
            .map_err(|_| "child lock")?
            .try_wait()?
            .is_some());
        // The session remains alive here; cleanup does not depend on polling it.
        assert!(session.directory.exists());
        drop(session);
        Ok(())
    }
    fn abort_policy() -> DerivedState {
        DerivedState {
            low_effort_verified: true,
            ..moderation_state()
        }
    }
    fn abort_message(method: &str) -> Value {
        let key = if method == "item/started" {
            "startedAtMs"
        } else {
            "completedAtMs"
        };
        json!({"method":method,"params":{"threadId":"t","turnId":"u","item":{"id":"i","type":"agentMessage","text":""},(key):1}})
    }
    fn abort_delta() -> Value {
        json!({"method":"item/agentMessage/delta","params":{"threadId":"t","turnId":"u","itemId":"i","delta":"fixture"}})
    }
    fn abort_open() -> Result<(DerivedState, AbortStream), DirectError> {
        let mut policy = abort_policy();
        let mut state = AbortStream::default();
        validate_abort_tail(&mut policy, &mut state, &abort_message("item/started"))?;
        Ok((policy, state))
    }
    #[test]
    fn abort_tail_validates_bound_buffered_text_without_emitting_output() -> Result<(), DirectError>
    {
        let (mut policy, mut state) = abort_open()?;
        assert_eq!(
            validate_abort_tail(&mut policy, &mut state, &abort_delta()),
            Ok(())
        );
        assert_eq!(state.characters, 7);
        assert_eq!(policy.thread.as_deref(), Some("t"));
        assert_eq!(policy.turn.as_deref(), Some("u"));
        // Exactly the same frame remains forbidden by successful-turn tail rules.
        assert!(validate_tail(&mut policy, &abort_delta()).is_err());
        Ok(())
    }
    #[test]
    fn abort_tail_requires_full_authenticated_immutable_context() {
        for change in 0..8 {
            let mut p = abort_policy();
            match change {
                0 => p.authenticated = false,
                1 => p.deny_all_verified = false,
                2 => p.low_effort_verified = false,
                3 => p.model = Some("other".into()),
                4 => p.thread = None,
                5 => p.turn = None,
                6 => p.thread = Some("".into()),
                _ => p.turn = Some("bad\n".into()),
            }
            assert!(!abort_context(&p, &AbortStream::default()));
            assert!(validate_abort_tail(
                &mut p,
                &mut AbortStream::default(),
                &abort_message("item/started")
            )
            .is_err());
        }
        let mut poisoned = AbortStream {
            poisoned: true,
            ..Default::default()
        };
        assert!(validate_abort_tail(
            &mut abort_policy(),
            &mut poisoned,
            &abort_message("item/started")
        )
        .is_err());
    }
    #[test]
    fn abort_tail_rejects_wrong_binding_unknown_items_and_shapes() -> Result<(), DirectError> {
        for field in ["threadId", "turnId", "itemId", "delta"] {
            for wrong in [json!(null), json!(4), json!({"x":true})] {
                let (mut p, mut state) = abort_open()?;
                let mut value = abort_delta();
                value["params"][field] = wrong;
                assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
            }
            let (mut p, mut state) = abort_open()?;
            let mut value = abort_delta();
            assert!(value["params"]
                .as_object_mut()
                .is_some_and(|v| v.remove(field).is_some()));
            assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
        }
        for field in ["threadId", "turnId", "itemId"] {
            let (mut p, mut state) = abort_open()?;
            let mut value = abort_delta();
            value["params"][field] = json!("other");
            assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
        }
        let (mut p, mut state) = abort_open()?;
        let mut value = abort_delta();
        value["params"]["extra"] = json!(true);
        assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
        Ok(())
    }
    #[test]
    fn abort_tail_rejects_tools_requests_unknowns_and_warnings() -> Result<(), DirectError> {
        for method in [
            "private_unknown",
            "item/mcpToolCall/progress",
            "item/commandExecution/outputDelta",
            "item/dynamicToolCall/delta",
            "model/rerouted",
            "tool/call",
        ] {
            let (mut p, mut state) = abort_open()?;
            let mut value = abort_delta();
            value["method"] = json!(method);
            assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
        }
        for value in [
            json!({"id":1,"result":{}}),
            json!({"id":1,"method":"item/agentMessage/delta","params":{}}),
            json!({"method":"warning","params":{"threadId":"t","message":"unknown"}}),
        ] {
            let (mut p, mut state) = abort_open()?;
            assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
        }
        let mut value = abort_message("item/started");
        value["params"]["item"]["type"] = json!("commandExecution");
        assert!(
            validate_abort_tail(&mut abort_policy(), &mut AbortStream::default(), &value).is_err()
        );
        Ok(())
    }
    #[test]
    fn abort_tail_lifecycle_rejects_duplicates_reordering_and_post_terminal(
    ) -> Result<(), DirectError> {
        let (mut p, mut s) = abort_open()?;
        assert!(validate_abort_tail(&mut p, &mut s, &abort_message("item/started")).is_err());
        let (mut p, mut s) = abort_open()?;
        validate_abort_tail(&mut p, &mut s, &abort_message("item/completed"))?;
        assert!(validate_abort_tail(&mut p, &mut s, &abort_message("item/completed")).is_err());
        assert!(validate_abort_tail(&mut p, &mut s, &abort_delta()).is_err());
        let completed = json!({"method":"turn/completed","params":{"threadId":"t","turn":{"id":"u","status":"interrupted","items":[{"id":"i","type":"agentMessage","text":"fixture"}]}}});
        validate_abort_tail(&mut p, &mut s, &completed)?;
        assert!(validate_abort_tail(&mut p, &mut s, &completed).is_err());
        assert!(validate_abort_tail(&mut p, &mut s, &abort_delta()).is_err());
        assert!(validate_abort_tail(&mut p, &mut s, &abort_message("item/started")).is_err());
        let start = json!({"method":"turn/started","params":{"threadId":"t","turn":{"id":"u","status":"inProgress","items":[]}}});
        let mut p = abort_policy();
        let mut s = AbortStream::default();
        validate_abort_tail(&mut p, &mut s, &start)?;
        assert!(validate_abort_tail(&mut p, &mut s, &start).is_err());
        assert!(validate_abort_tail(
            &mut abort_policy(),
            &mut AbortStream::default(),
            &abort_message("item/completed")
        )
        .is_err());
        Ok(())
    }
    #[test]
    fn abort_tail_terminal_status_and_metadata_types_are_closed() -> Result<(), DirectError> {
        for status in ["inProgress", "unknown"] {
            let (mut p, mut state) = abort_open()?;
            let value = json!({"method":"turn/completed","params":{"threadId":"t","turn":{"id":"u","items":[],"status":status}}});
            assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
        }
        for key in [
            "completedAt",
            "durationMs",
            "startedAt",
            "itemsView",
            "rootTurnId",
            "error",
        ] {
            let (mut p, mut state) = abort_open()?;
            let mut value = json!({"method":"turn/completed","params":{"threadId":"t","turn":{"id":"u","items":[],"status":"completed"}}});
            value["params"]["turn"][key] = json!({"unreviewed":true});
            assert!(validate_abort_tail(&mut p, &mut state, &value).is_err());
        }
        for key in ["phase", "delivery", "memoryCitation"] {
            let mut value = abort_message("item/started");
            value["params"]["item"][key] = json!(true);
            assert!(
                validate_abort_tail(&mut abort_policy(), &mut AbortStream::default(), &value)
                    .is_err()
            );
        }
        let mut value = abort_message("item/started");
        value["params"]["startedAtMs"] = json!("not_integer");
        assert!(
            validate_abort_tail(&mut abort_policy(), &mut AbortStream::default(), &value).is_err()
        );
        Ok(())
    }
    #[test]
    fn abort_tail_preserves_active_flag_and_payload_bounds() -> Result<(), DirectError> {
        let value = json!({"method":"thread/status/changed","params":{"threadId":"t","status":{"type":"active","activeFlags":[]}}});
        assert_eq!(
            validate_abort_tail(&mut abort_policy(), &mut AbortStream::default(), &value),
            Ok(())
        );
        let mut unsafe_value = value;
        unsafe_value["params"]["status"]["activeFlags"] = json!(["waitingOnApproval"]);
        assert!(validate_abort_tail(
            &mut abort_policy(),
            &mut AbortStream::default(),
            &unsafe_value
        )
        .is_err());
        let (mut p, mut s) = abort_open()?;
        let mut value = abort_delta();
        value["params"]["delta"] = json!("x".repeat(8193));
        assert_eq!(
            validate_abort_tail(&mut p, &mut s, &value),
            Err(DirectError::Limit)
        );
        let (mut p, mut s) = abort_open()?;
        let mut value = abort_delta();
        value["params"]["delta"] = json!("😀".repeat(8192));
        assert_eq!(validate_abort_tail(&mut p, &mut s, &value), Ok(()));
        assert_eq!(s.bytes, 32768);
        assert_eq!(
            validate_abort_tail(&mut p, &mut s, &abort_delta()),
            Err(DirectError::Limit)
        );
        Ok(())
    }
    #[test]
    fn abort_mode_is_not_inferred_from_provider_or_capture_errors() {
        for error in [
            DirectError::Protocol,
            DirectError::CodexRuntime,
            DirectError::ProviderUnavailable,
            DirectError::Incomplete,
            DirectError::Internal,
            DirectError::Limit,
        ] {
            assert!(matches!(
                returned_tail_mode(false, &Err(error)),
                TailMode::Strict
            ));
            if error != DirectError::Limit {
                assert!(matches!(
                    returned_tail_mode(true, &Err(error)),
                    TailMode::Strict
                ));
            }
        }
        assert!(matches!(
            returned_tail_mode(true, &Err(DirectError::Limit)),
            TailMode::LocalAbort
        ));
        assert!(matches!(
            returned_tail_mode(true, &Ok(())),
            TailMode::Strict
        ));
        assert!(matches!(
            returned_tail_mode(false, &Ok(())),
            TailMode::Strict
        ));
    }
    #[test]
    fn stopping_callback_without_owner_origin_rejects_a_valid_abort_tail() -> Result<(), DirectError>
    {
        let (policy, stream) = abort_open()?;
        assert!(abort_context(&policy, &stream));
        let callback_result = Err(DirectError::InvalidRequest);
        let mode = returned_tail_mode(false, &callback_result);
        assert!(mode == TailMode::Strict);
        let mut strict_policy = policy;
        assert!(validate_tail(&mut strict_policy, &abort_delta()).is_err());
        let (mut abort_policy, mut abort_stream) = abort_open()?;
        assert_eq!(
            validate_abort_tail(&mut abort_policy, &mut abort_stream, &abort_delta()),
            Ok(())
        );
        Ok(())
    }
    #[test]
    fn owner_cancel_arbitrates_both_terminal_orders_without_sleep(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for cancellation_first in [false, true] {
            let owner = CancellationHandle::default();
            assert!(owner.arm()?);
            let peer = owner.clone();
            let (ready_tx, ready_rx) = std::sync::mpsc::channel();
            let (go_tx, go_rx) = std::sync::mpsc::channel();
            let worker = std::thread::spawn(move || {
                if cancellation_first {
                    peer.request_stop();
                }
                ready_tx.send(()).map_err(|_| ())?;
                go_rx.recv().map_err(|_| ())?;
                if !cancellation_first {
                    peer.request_stop();
                }
                Ok::<(), ()>(())
            });
            ready_rx.recv()?;
            assert_eq!(owner.complete()?, !cancellation_first);
            go_tx.send(())?;
            assert!(worker.join().is_ok_and(|r| r.is_ok()));
            assert_eq!(owner.active_requested(), cancellation_first);
            owner.request_stop();
            assert_eq!(owner.active_requested(), cancellation_first);
        }
        Ok(())
    }
    #[test]
    fn owner_callback_origin_is_request_local_and_generic_errors_stay_strict(
    ) -> Result<(), DirectError> {
        let owner = CancellationHandle::default();
        let other = CancellationHandle::default();
        assert!(owner.arm()?);
        owner.request_stop();
        assert!(!owner.take_stopping_callback());
        other.stopping_callback();
        assert!(!other.take_stopping_callback());
        owner.stopping_callback();
        assert!(owner.take_stopping_callback());
        assert!(!owner.take_stopping_callback());
        assert!(returned_tail_mode(false, &Err(DirectError::InvalidRequest)) == TailMode::Strict);
        owner.reject();
        owner.stopping_callback();
        owner.local_return();
        assert!(!owner.active_requested());
        assert!(!owner.take_stopping_callback());
        assert!(!owner.returned_locally());
        Ok(())
    }
    #[test]
    fn owner_stop_before_active_is_never_active_abort() -> Result<(), DirectError> {
        let owner = CancellationHandle::default();
        owner.request_stop();
        assert!(!owner.arm()?);
        assert!(!owner.active_requested());
        owner.stopping_callback();
        assert!(owner.take_stopping_callback());
        owner.local_return();
        assert!(owner.returned_locally());
        assert!(!owner.complete()?);
        Ok(())
    }
    #[cfg(unix)]
    fn owned_abort_fixture() -> Result<(Fixture, Session), Box<dyn std::error::Error>> {
        let (directory, setup) = fixture("pending-initialize")?;
        let mut session = Session::spawn(&setup)?;
        session.derived = true;
        session.policy = abort_policy();
        session.abort_stream = AbortStream::default();
        assert!(session.cancellation.arm()?);
        Ok((directory, session))
    }
    #[cfg(unix)]
    #[test]
    fn owned_abort_drop_reaps_before_releasing_session_resources(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_fixture, session) = owned_abort_fixture()?;
        let directory = session.directory.clone();
        let child = session.child.clone();
        let capture = session.capture.clone();
        session.cancellation.request_stop();
        drop(session);
        assert!(!directory.exists());
        assert!(child
            .lock()
            .map_err(|_| "fixture lock")?
            .try_wait()?
            .is_some());
        assert_eq!(
            capture.lock().map_err(|_| "capture lock")?.complete(),
            Ok(())
        );
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn owned_callback_abort_requires_successful_finish_before_local_return(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_fixture, mut session) = owned_abort_fixture()?;
        session.cancellation.request_stop();
        session.cancellation.stopping_callback();
        assert!(session.cancellation.take_stopping_callback());
        assert_eq!(session.finish(), Ok(()));
        session.cancellation.local_return();
        assert!(session.cancellation.returned_locally());
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn owned_stop_never_clears_capture_or_lifecycle_rejection(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for poison_capture in [false, true] {
            let (_fixture, mut session) = owned_abort_fixture()?;
            session.cancellation.request_stop();
            if poison_capture {
                session.capture.lock().map_err(|_| "capture lock")?.failure =
                    Some(DirectError::Protocol);
            } else {
                session.abort_stream.poisoned = true;
            }
            assert!(session.finish().is_err());
            session.cancellation.local_return();
            assert!(!session.cancellation.returned_locally());
            assert!(!session.cancellation.active_requested());
        }
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn owned_stop_never_clears_cleanup_failure() -> Result<(), Box<dyn std::error::Error>> {
        let (_fixture, mut session) = owned_abort_fixture()?;
        let original = session.directory.clone();
        session.directory = original.join("absent-fixture");
        session.cancellation.request_stop();
        assert_eq!(session.finish(), Err(DirectError::Internal));
        session.cancellation.local_return();
        assert!(!session.cancellation.returned_locally());
        std::fs::remove_dir(original)?;
        Ok(())
    }
    #[test]
    fn owner_stop_preserves_all_abort_tail_rejection_rules() -> Result<(), DirectError> {
        for frame in [
            json!({"method":"unknown","params":{}}),
            json!({"id":17,"method":"tool/call","params":{}}),
            json!({"method":"item/started","params":{"threadId":"t","turnId":"u","item":{"id":"i","type":"commandExecution"},"startedAtMs":1}}),
            json!({"method":"warning","params":{"message":"unknown synthetic warning","threadId":"t"}}),
            json!({"method":"item/agentMessage/delta","params":{"threadId":"wrong","turnId":"u","itemId":"i","delta":"fixture"}}),
        ] {
            let owner = CancellationHandle::default();
            assert!(owner.arm()?);
            owner.request_stop();
            let (mut policy, mut stream) = abort_open()?;
            assert!(validate_abort_tail(&mut policy, &mut stream, &frame).is_err());
            owner.reject();
            owner.local_return();
            assert!(!owner.returned_locally());
        }
        Ok(())
    }

    #[test]
    fn abort_tail_fixed_labels_are_distinct_from_successful_completion() {
        for label in [
            "protocol_abort_tail_complete",
            "protocol_abort_tail_rejected",
        ] {
            assert!(OBSERVATION_LABELS.contains(&label));
            assert_eq!(reviewed_rejection_site(label), label);
            assert_ne!(label, "protocol_tail_complete");
        }
        assert!(decode_frame(
            br#"{"method":"item/agentMessage/delta","params":{"delta":"a","delta":"b"}}"#,
            true
        )
        .is_err());
        assert_eq!(Capture::default().complete(), Err(DirectError::Incomplete));
    }
    #[test]
    fn abort_acknowledgement_requires_complete_in_progress_turn_shape() {
        let good = json!({"id":"u","status":"inProgress","items":[]});
        assert_eq!(abort_turn_fields(&good, false), Ok(()));
        for key in ["id", "status", "items"] {
            let mut v = good.clone();
            assert!(v.as_object_mut().is_some_and(|m| m.remove(key).is_some()));
            assert!(abort_turn_fields(&v, false).is_err());
        }
        for status in ["completed", "interrupted", "failed", "unknown"] {
            let mut v = good.clone();
            v["status"] = json!(status);
            assert!(abort_turn_fields(&v, false).is_err());
        }
        let mut v = good;
        v["items"] = json!([{"type":"commandExecution"}]);
        assert!(abort_turn_fields(&v, false).is_err());
    }

    fn artifact_fixture() -> Result<(tempfile::TempDir, PathBuf, String), Box<dyn std::error::Error>>
    {
        use sha2::{Digest, Sha256};
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("synthetic-artifact");
        std::fs::write(&path, b"fixture")?;
        let hash = format!("{:x}", Sha256::digest(b"fixture"));
        Ok((directory, path, hash))
    }
    fn fixture_setup(path: PathBuf, hash: &str) -> Result<Setup, DirectError> {
        let lease = prepare_derived_artifact(&path, 7, hash)?;
        Ok(Setup {
            executable: path,
            home: "/synthetic/auth".into(),
            derived: true,
            verified_derived: Mutex::new(lease),
            cancellation: CancellationHandle::default(),
        })
    }
    #[test]
    fn prepared_artifact_hashes_twice_before_consumption_and_cannot_replay(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, path, hash) = artifact_fixture()?;
        ARTIFACT_HASH_PASSES.with(|n| n.set(0));
        let setup = fixture_setup(path, &hash)?;
        assert_eq!(ARTIFACT_HASH_PASSES.with(std::cell::Cell::get), 2);
        let lease = setup.consume_verified()?.ok_or("missing lease")?;
        assert!(setup.consume_verified().is_err());
        lease.verify(&setup.executable)?;
        assert_eq!(ARTIFACT_HASH_PASSES.with(std::cell::Cell::get), 2);
        Ok(())
    }
    #[test]
    fn prepared_artifact_first_and_second_hash_failures_reject(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, path, hash) = artifact_fixture()?;
        ARTIFACT_HASH_PASSES.with(|n| n.set(0));
        assert!(prepare_derived_artifact(&path, 7, "incorrect")?.is_none());
        assert_eq!(ARTIFACT_HASH_PASSES.with(std::cell::Cell::get), 1);
        let changed = path.clone();
        ARTIFACT_AFTER_FIRST.with(|slot| {
            *slot.borrow_mut() = Some(Box::new(move || {
                std::fs::write(changed, b"changed").map_err(|_| DirectError::Internal)
            }))
        });
        assert!(prepare_derived_artifact(&path, 7, &hash).is_err());
        Ok(())
    }
    #[test]
    fn prepared_artifact_missing_closed_and_non_derived_states_are_closed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, path, hash) = artifact_fixture()?;
        let setup = fixture_setup(path.clone(), &hash)?;
        let lease = setup.consume_verified()?;
        drop(lease);
        assert!(setup.consume_verified().is_err());
        let legacy = Setup {
            executable: path.clone(),
            home: "/synthetic".into(),
            derived: false,
            verified_derived: Mutex::new(None),
            cancellation: CancellationHandle::default(),
        };
        assert!(legacy.consume_verified()?.is_none());
        let wrong = Setup {
            executable: path,
            home: "/synthetic".into(),
            derived: false,
            verified_derived: Mutex::new(prepare_derived_artifact(&legacy.executable, 7, &hash)?),
            cancellation: CancellationHandle::default(),
        };
        assert!(wrong.consume_verified().is_err());
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn prepared_artifact_replacement_mutation_symlink_and_links_reject(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::{symlink, PermissionsExt};
        for action in 0..4 {
            let (directory, path, hash) = artifact_fixture()?;
            let setup = fixture_setup(path.clone(), &hash)?;
            match action {
                0 => {
                    std::fs::rename(&path, directory.path().join("old"))?;
                    std::fs::write(&path, b"fixture")?;
                }
                1 => std::fs::write(&path, b"changed")?,
                2 => {
                    std::fs::rename(&path, directory.path().join("old"))?;
                    symlink(directory.path().join("old"), &path)?;
                }
                _ => std::fs::hard_link(&path, directory.path().join("link"))?,
            }
            assert!(setup.consume_verified().is_err());
        }
        let (_directory, path, hash) = artifact_fixture()?;
        let setup = fixture_setup(path.clone(), &hash)?;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700))?;
        assert!(setup.consume_verified().is_err());
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn prepared_artifact_each_identity_field_and_timestamp_is_strict(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, path, hash) = artifact_fixture()?;
        let lease = prepare_derived_artifact(&path, 7, &hash)?.ok_or("missing lease")?;
        for field in 0..10 {
            let mut altered = lease.identity.clone();
            altered.unix[field] = altered.unix[field].wrapping_add(1);
            assert!(altered != ArtifactIdentity::read(&lease.file.metadata()?)?);
        }
        let mut altered = lease.identity.clone();
        altered.bytes += 1;
        assert!(altered != ArtifactIdentity::read(&lease.file.metadata()?)?);
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn prepared_artifact_drop_and_failed_spawn_release_descriptor(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::fd::AsRawFd;
        let (_directory, path, hash) = artifact_fixture()?;
        let setup = fixture_setup(path.clone(), &hash)?;
        let lease = setup.consume_verified()?.ok_or("missing lease")?;
        let fd_path = PathBuf::from(format!("/dev/fd/{}", lease.file.as_raw_fd()));
        let identity = lease.identity.clone();
        drop(lease);
        assert!(std::fs::metadata(&fd_path)
            .ok()
            .and_then(|m| ArtifactIdentity::read(&m).ok())
            .is_none_or(|m| m != identity));
        let setup = fixture_setup(path.clone(), &hash)?;
        let fd = setup
            .verified_derived
            .lock()
            .map_err(|_| "lease lock")?
            .as_ref()
            .ok_or("missing lease")?
            .file
            .as_raw_fd();
        let fd_path = PathBuf::from(format!("/dev/fd/{fd}"));
        assert!(Session::spawn(&setup).is_err());
        assert!(setup.consume_verified().is_err());
        assert!(std::fs::metadata(&fd_path)
            .ok()
            .and_then(|m| ArtifactIdentity::read(&m).ok())
            .is_none_or(|m| m != identity));
        Ok(())
    }

    #[test]
    fn prepared_artifact_second_read_failure_and_identical_replacement_reject(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for replacement in [false, true] {
            let (directory, path, hash) = artifact_fixture()?;
            let target = path.clone();
            let retired = directory.path().join("original");
            ARTIFACT_AFTER_FIRST.with(|slot| {
                *slot.borrow_mut() = Some(Box::new(move || {
                    std::fs::rename(&target, &retired).map_err(|_| DirectError::Internal)?;
                    if replacement {
                        std::fs::write(&target, b"fixture").map_err(|_| DirectError::Internal)?;
                    }
                    Ok(())
                }))
            });
            assert!(prepare_derived_artifact(&path, 7, &hash).is_err());
        }
        let directory = tempfile::tempdir()?;
        assert!(prepare_derived_artifact(&directory.path().join("missing"), 7, "unused").is_err());
        Ok(())
    }
    #[test]
    fn prepared_artifact_immediate_pre_spawn_revalidation_rejects_change(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, path, hash) = artifact_fixture()?;
        let setup = fixture_setup(path.clone(), &hash)?;
        let lease = setup.consume_verified()?.ok_or("missing lease")?;
        std::fs::write(&path, b"changed")?;
        assert!(lease.verify(&path).is_err());
        Ok(())
    }
}
