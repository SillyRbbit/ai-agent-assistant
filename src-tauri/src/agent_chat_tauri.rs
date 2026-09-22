//! Closed IPC for saved profiles and one native-owned, bounded text generation.
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::agent_chat::{BoundConversation, ChatError, CODEX_LIMITATION};
use crate::agent_preferences::{
    AgentConnection, AgentPreferencesInput, AgentProfile, MemoryMode, ReasoningEffort,
};
use crate::personal_assistant_direct::{
    self as provider, ApiKey, DirectError, GenerationLease, ProviderEvent,
};
use crate::personal_assistant_v0::{
    PersonalAssistantV0FailureCode, PersonalAssistantV0Host, PersonalAssistantV0PresentationHandle,
    PersonalAssistantV0Snapshot,
};
use crate::storage::Storage;
use crate::{
    agent_models::{validate_selection, ModelInfo},
    anthropic, local_models,
};

#[derive(Default)]
pub(crate) struct AgentChatState(Arc<Mutex<Session>>);

#[derive(Default)]
struct Session {
    conversation: Option<BoundConversation>,
    catalogs: Vec<CatalogEntry>,
    next_id: u64,
    host: PersonalAssistantV0Host,
    handle: Option<PersonalAssistantV0PresentationHandle>,
    task: Option<JoinHandle<()>>,
    stopping: bool,
    error: Option<ChatError>,
}

impl Drop for Session {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

struct CatalogEntry {
    connection: AgentConnection,
    endpoint: String,
    local_auth: bool,
    models: Vec<ModelInfo>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CatalogRequest {
    connection: AgentConnection,
    endpoint: String,
    local_auth: bool,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CatalogSnapshot {
    connection: AgentConnection,
    endpoint: String,
    models: Vec<ModelInfo>,
}
static DISCOVERY_OWNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
struct DiscoveryLease;
impl Drop for DiscoveryLease {
    fn drop(&mut self) {
        DISCOVERY_OWNED.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AgentRequest {
    agent_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct RevisionRequest {
    agent_id: String,
    revision: u64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ConversationRequest {
    conversation_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SendRequest {
    conversation_id: String,
    message: String,
    acknowledgment: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChatSnapshot {
    version: u8,
    conversation_id: String,
    agent_id: String,
    connection: AgentConnection,
    model: String,
    endpoint: String,
    effort: ReasoningEffort,
    memory_mode: MemoryMode,
    settings_revision: u64,
    status: &'static str,
    text: String,
    sequence: u64,
    busy: bool,
    error: Option<ChatError>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConnectionReadiness {
    connection: AgentConnection,
    status: &'static str,
    message: &'static str,
}

impl Session {
    fn check_id(&self, id: &str) -> Result<(), ChatError> {
        if self.conversation.as_ref().is_some_and(|c| c.id == id) {
            Ok(())
        } else {
            Err(ChatError::InvalidRequest)
        }
    }

    fn owned(&self) -> bool {
        self.stopping || self.task.as_ref().is_some_and(|task| !task.is_finished())
    }

    fn snapshot(&mut self) -> Result<ChatSnapshot, ChatError> {
        let mut status = "idle";
        let mut text = String::new();
        let mut sequence = 0;
        if let Some(handle) = &self.handle {
            let snapshot = self
                .host
                .snapshot(handle)
                .map_err(|_| ChatError::Internal)?;
            if !snapshot.is_terminal() && self.task.as_ref().is_some_and(JoinHandle::is_finished) {
                self.error = Some(ChatError::Internal);
                self.host
                    .fail_direct(handle, PersonalAssistantV0FailureCode::Internal)
                    .map_err(|_| ChatError::Internal)?;
                return self.snapshot();
            }
            if snapshot.is_terminal() {
                if let Some(task) = &self.task {
                    if !task.is_finished() {
                        task.abort();
                    }
                }
            }
            status = match &snapshot {
                PersonalAssistantV0Snapshot::Starting => "starting",
                PersonalAssistantV0Snapshot::Streaming(_) => "streaming",
                PersonalAssistantV0Snapshot::Completed(_) => "completed",
                PersonalAssistantV0Snapshot::Cancelled(_)
                | PersonalAssistantV0Snapshot::Cancelling(_) => "stopped",
                PersonalAssistantV0Snapshot::Failed(failed) => {
                    if self.error.is_none() {
                        self.error = Some(ChatError::Provider(match failed.failure().code() {
                            PersonalAssistantV0FailureCode::DeadlineExceeded
                            | PersonalAssistantV0FailureCode::ProviderTimeout => {
                                DirectError::Timeout
                            }
                            _ => DirectError::Protocol,
                        }));
                    }
                    "error"
                }
            };
            text = snapshot.accepted_text().to_owned();
            sequence = snapshot.sequence();
        }
        let bound = self
            .conversation
            .as_ref()
            .ok_or(ChatError::InvalidRequest)?;
        Ok(ChatSnapshot {
            version: 1,
            conversation_id: bound.id.clone(),
            agent_id: bound.profile.agent_id.clone(),
            connection: bound.profile.connection,
            model: bound.profile.model.clone(),
            endpoint: bound.profile.endpoint.clone(),
            effort: bound.profile.effort,
            memory_mode: bound.profile.memory_mode,
            settings_revision: bound.profile.revision,
            status,
            text,
            sequence,
            busy: self.owned(),
            error: self.error,
        })
    }
}

fn begin_conversation(
    state: &Arc<Mutex<Session>>,
    profile: AgentProfile,
) -> Result<ChatSnapshot, ChatError> {
    let mut session = state.lock().map_err(|_| ChatError::Internal)?;
    if session.owned() {
        return Err(ChatError::Busy);
    }
    let next = session.next_id.checked_add(1).ok_or(ChatError::Limit)?;
    let bound = BoundConversation::new(format!("agent-chat-{next}"), profile)?;
    session.next_id = next;
    session.conversation = Some(bound);
    session.handle = None;
    session.error = None;
    session.task = None;
    session.snapshot()
}

enum AdapterRequest {
    Simulation,
    Anthropic {
        key: anthropic::AnthropicKey,
        body: Vec<u8>,
    },
    Local {
        endpoint: String,
        key: Option<local_models::LocalKey>,
        body: Vec<u8>,
    },
    Openai {
        key: ApiKey,
        body: Vec<u8>,
    },
    #[cfg(test)]
    Failure(DirectError),
    #[cfg(test)]
    Pending(Arc<std::sync::atomic::AtomicBool>),
}

fn start_owned(
    state: &Arc<Mutex<Session>>,
    storage: &Storage,
    request: SendRequest,
    key: impl FnOnce() -> Result<ApiKey, DirectError>,
) -> Result<ChatSnapshot, ChatError> {
    let mut session = state.lock().map_err(|_| ChatError::Internal)?;
    session.check_id(&request.conversation_id)?;
    if session.owned() {
        return Err(ChatError::Busy);
    }
    if session.handle.is_some() && !matches!(session.snapshot()?.status, "idle" | "completed") {
        return Err(ChatError::InvalidRequest);
    }
    let bound = session
        .conversation
        .as_ref()
        .ok_or(ChatError::InvalidRequest)?;
    // The caller supplies only a conversation ID and message, never context or agent authority.
    let current = storage.agent_profile(&bound.profile.agent_id)?;
    bound.validate_send(&current, &request.message)?;
    let adapter = match bound.profile.connection {
        AgentConnection::Codex => return Err(ChatError::CodexIsolation),
        AgentConnection::Simulation => {
            if request.acknowledgment != "simulation" {
                return Err(ChatError::InvalidRequest);
            }
            AdapterRequest::Simulation
        }
        AgentConnection::AnthropicApi | AgentConnection::LmStudio | AgentConnection::Ollama => {
            let profile = &bound.profile;
            let expected = if profile.connection == AgentConnection::AnthropicApi {
                "anthropic-agent-text-v1"
            } else {
                "local-agent-text-v1"
            };
            if request.acknowledgment != expected {
                return Err(ChatError::InvalidRequest);
            }
            validate_catalog(&session.catalogs, profile)?;
            let body = bound.provider_request_body(&request.message)?;
            if profile.connection == AgentConnection::AnthropicApi {
                AdapterRequest::Anthropic {
                    key: anthropic::AnthropicKey::from_environment()?,
                    body,
                }
            } else {
                AdapterRequest::Local {
                    endpoint: profile.endpoint.clone(),
                    key: local_models::LocalKey::from_environment(
                        profile.connection,
                        &profile.endpoint,
                        profile.local_auth,
                    )?,
                    body,
                }
            }
        }
        AgentConnection::OpenaiApi => {
            if request.acknowledgment != "openai-agent-text-v1" {
                return Err(ChatError::InvalidRequest);
            }
            let body = bound.request_body(&request.message)?;
            AdapterRequest::Openai { key: key()?, body }
        }
    };
    let lease = GenerationLease::acquire()?;
    // Presentation ownership and transport cleanup must both finish before reuse.
    let start = session.host.start_direct().map_err(|_| ChatError::Busy)?;
    let handle = start.presentation_handle().clone();
    session.handle = Some(handle.clone());
    session.error = None;
    session.task = Some(tokio::spawn(execute(
        Arc::downgrade(state),
        handle,
        request.message,
        adapter,
        lease,
        Duration::from_secs(60),
    )));
    session.snapshot()
}

async fn run_adapter(
    adapter: AdapterRequest,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    match adapter {
        #[cfg(test)]
        AdapterRequest::Failure(error) => Err(error),
        #[cfg(test)]
        AdapterRequest::Pending(dropped) => {
            struct Dropped(Arc<std::sync::atomic::AtomicBool>);
            impl Drop for Dropped {
                fn drop(&mut self) {
                    self.0.store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
            let _guard = Dropped(dropped);
            std::future::pending().await
        }
        AdapterRequest::Anthropic { key, body } => anthropic::run(key, body, emit).await,
        AdapterRequest::Local {
            endpoint,
            key,
            body,
        } => local_models::run(&endpoint, key, body, emit).await,
        AdapterRequest::Openai { key, body } => provider::run_configured(key, body, emit).await,
        AdapterRequest::Simulation => {
            emit(ProviderEvent::Started("resp_cortexa_simulation".into()))?;
            for part in [
                "Simulation only — no hosted request was made. ",
                "This native conversation is bound to your selected agent and saved settings. ",
                "Switch to an available connection for generated text advice; no tools or actions run.",
            ] {
                tokio::time::sleep(Duration::from_millis(180)).await;
                emit(ProviderEvent::Delta(part.into()))?;
            }
            emit(ProviderEvent::Completed)
        }
    }
}

async fn execute(
    state: Weak<Mutex<Session>>,
    handle: PersonalAssistantV0PresentationHandle,
    prompt: String,
    adapter: AdapterRequest,
    _lease: GenerationLease,
    deadline: Duration,
) {
    let mut completed = false;
    let result = tokio::time::timeout(
        deadline,
        run_adapter(adapter, |event| {
            let state = state.upgrade().ok_or(DirectError::InvalidRequest)?;
            let mut session = state.lock().map_err(|_| DirectError::Internal)?;
            if session.handle.as_ref() != Some(&handle) || session.stopping {
                return Err(DirectError::InvalidRequest);
            }
            if matches!(event, ProviderEvent::Completed) {
                completed = true;
                return Ok(());
            }
            let snapshot = session
                .host
                .accept_direct(&handle, event)
                .map_err(|_| DirectError::Protocol)?;
            if snapshot.is_terminal() {
                return Err(DirectError::Protocol);
            }
            Ok(())
        }),
    )
    .await
    .unwrap_or(Err(DirectError::Timeout));
    let result = result.and({
        if completed {
            Ok(())
        } else {
            Err(DirectError::Incomplete)
        }
    });
    // The transport future is dropped before the terminal transition releases the lease.
    let Some(state) = state.upgrade() else {
        return;
    };
    if let Ok(mut session) = state.lock() {
        if session.handle.as_ref() != Some(&handle) || session.stopping {
            return;
        }
        if !session
            .host
            .snapshot(&handle)
            .is_ok_and(|s| !s.is_terminal())
        {
            return;
        }
        match result {
            Ok(()) => {
                match session
                    .host
                    .accept_direct(&handle, ProviderEvent::Completed)
                {
                    Ok(snapshot) => {
                        if let Some(bound) = session.conversation.as_mut() {
                            bound.complete_turn(prompt, snapshot.accepted_text().to_owned());
                        }
                    }
                    Err(_) => {
                        session.error = Some(ChatError::Internal);
                        let _ = session
                            .host
                            .fail_direct(&handle, PersonalAssistantV0FailureCode::Internal);
                    }
                }
            }
            Err(error) => {
                session.error = Some(ChatError::Provider(error));
                let _ = session
                    .host
                    .fail_direct(&handle, PersonalAssistantV0FailureCode::ProviderUnavailable);
            }
        }
    };
}

async fn stop_owned(state: &Arc<Mutex<Session>>, id: &str) -> Result<ChatSnapshot, ChatError> {
    let task = {
        let mut session = state.lock().map_err(|_| ChatError::Internal)?;
        session.check_id(id)?;
        if session.stopping {
            return session.snapshot();
        }
        // Close ingress first, but retain host ownership until the transport drops.
        session.stopping = true;
        session.task.take()
    };
    if let Some(task) = task {
        task.abort();
        let _ = task.await;
    }
    let mut session = state.lock().map_err(|_| ChatError::Internal)?;
    if let Some(handle) = session.handle.clone() {
        session
            .host
            .cancel(&handle)
            .map_err(|_| ChatError::Internal)?;
    }
    session.stopping = false;
    session.snapshot()
}

fn validate_catalog(catalogs: &[CatalogEntry], profile: &AgentProfile) -> Result<(), DirectError> {
    let discovered = catalogs.iter().find(|entry| {
        entry.connection == profile.connection
            && entry.endpoint == profile.endpoint
            && entry.local_auth == profile.local_auth
    });
    let seeds;
    let models = if let Some(entry) = discovered {
        &entry.models
    } else if profile.connection == AgentConnection::AnthropicApi {
        seeds = anthropic::documented_models();
        &seeds
    } else {
        return Err(DirectError::Catalog);
    };
    let model = models
        .iter()
        .find(|model| model.id == profile.model)
        .ok_or(DirectError::ModelUnavailable)?;
    if model.locality == "cloud"
        || (matches!(
            profile.connection,
            AgentConnection::LmStudio | AgentConnection::Ollama
        ) && model.availability != "available")
    {
        return Err(DirectError::Unsupported);
    }
    validate_selection(model, profile.effort)
}

/// Explicit bounded refresh. No prompt, notes, secret, path or arbitrary method is accepted.
#[tauri::command]
pub(crate) async fn discover_agent_models(
    request: CatalogRequest,
    state: tauri::State<'_, AgentChatState>,
) -> Result<CatalogSnapshot, ChatError> {
    let endpoint = match request.connection {
        AgentConnection::AnthropicApi if request.endpoint.is_empty() && !request.local_auth => {
            String::new()
        }
        AgentConnection::LmStudio | AgentConnection::Ollama => {
            local_models::normalize_endpoint(&request.endpoint)?
        }
        _ => return Err(ChatError::InvalidRequest),
    };
    if DISCOVERY_OWNED
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        )
        .is_err()
    {
        return Err(ChatError::Busy);
    }
    let _lease = DiscoveryLease;
    // A failed refresh preserves profiles, not stale model authorization.
    {
        let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
        session.catalogs.retain(|entry| {
            !(entry.connection == request.connection
                && entry.endpoint == endpoint
                && entry.local_auth == request.local_auth)
        });
    }
    let models = tokio::time::timeout(Duration::from_secs(30), async {
        if request.connection == AgentConnection::AnthropicApi {
            anthropic::discover().await
        } else {
            local_models::discover(request.connection, &endpoint, request.local_auth).await
        }
    })
    .await
    .map_err(|_| DirectError::Timeout)??;
    let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
    session.catalogs.retain(|entry| {
        !(entry.connection == request.connection
            && entry.endpoint == endpoint
            && entry.local_auth == request.local_auth)
    });
    if session.catalogs.len() >= 8 {
        session.catalogs.remove(0);
    }
    session.catalogs.push(CatalogEntry {
        connection: request.connection,
        endpoint: endpoint.clone(),
        local_auth: request.local_auth,
        models: models.clone(),
    });
    Ok(CatalogSnapshot {
        connection: request.connection,
        endpoint,
        models,
    })
}

#[tauri::command]
pub(crate) fn list_agent_preferences(
    storage: tauri::State<'_, Storage>,
) -> Result<Vec<AgentProfile>, ChatError> {
    Ok(storage.agent_profiles()?)
}
#[tauri::command]
pub(crate) fn save_agent_preferences(
    request: AgentPreferencesInput,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<AgentProfile, ChatError> {
    let _session = state.0.lock().map_err(|_| ChatError::Internal)?;
    Ok(storage.save_agent_preferences(request)?)
}
#[tauri::command]
pub(crate) fn clear_agent_note(
    request: RevisionRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<AgentProfile, ChatError> {
    let _session = state.0.lock().map_err(|_| ChatError::Internal)?;
    Ok(storage.clear_agent_note(&request.agent_id, request.revision)?)
}
#[tauri::command]
pub(crate) fn restore_agent_defaults(
    request: RevisionRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<AgentProfile, ChatError> {
    let _session = state.0.lock().map_err(|_| ChatError::Internal)?;
    Ok(storage.restore_agent_defaults(&request.agent_id, request.revision)?)
}
#[tauri::command]
pub(crate) fn list_agent_connections() -> Vec<ConnectionReadiness> {
    let enabled =
        cfg!(debug_assertions) && std::env::var("CORTEXA_OPENAI_DEMO").as_deref() == Ok("1");
    vec![
        ConnectionReadiness {
            connection: AgentConnection::Simulation,
            status: "ready",
            message: "Native simulation; no hosted request, model or automatic learning.",
        },
        ConnectionReadiness {
            connection: AgentConnection::OpenaiApi,
            status: if enabled {
                "owner_setup_required"
            } else {
                "blocked"
            },
            message: if enabled {
                "Owner session API key is validated only on acknowledged Send. API billing is separate from ChatGPT."
            } else {
                "Requires a debug build privately launched with CORTEXA_OPENAI_DEMO=1 and an owner session API key."
            },
        },
        ConnectionReadiness { connection: AgentConnection::AnthropicApi, status: if cfg!(debug_assertions) { "owner_setup_required" } else { "blocked" }, message: "Owner setup: CORTEXA_ANTHROPIC_DEMO=1 and a native ANTHROPIC_API_KEY. Discovery and Send are explicit; account access is unverified." },
        ConnectionReadiness { connection: AgentConnection::LmStudio, status: "owner_setup_required", message: "Connect an already-running loopback LM Studio server. Refresh models explicitly; no installation, loading or locality is assumed." },
        ConnectionReadiness { connection: AgentConnection::Ollama, status: "owner_setup_required", message: "Connect an already-running loopback Ollama server. Cloud models are excluded. Disable cloud in the runtime; locality remains unverified." },
        ConnectionReadiness {
            connection: AgentConnection::Codex,
            status: "blocked",
            message: CODEX_LIMITATION,
        },
    ]
}
#[tauri::command]
pub(crate) fn start_agent_conversation(
    request: AgentRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    begin_conversation(&state.0, storage.agent_profile(&request.agent_id)?)
}
#[tauri::command]
pub(crate) async fn send_agent_message(
    request: SendRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    start_owned(&state.0, &storage, request, ApiKey::from_environment)
}
#[tauri::command]
pub(crate) fn poll_agent_conversation(
    request: ConversationRequest,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
    session.check_id(&request.conversation_id)?;
    session.snapshot()
}
#[tauri::command]
pub(crate) async fn cancel_agent_conversation(
    request: ConversationRequest,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    stop_owned(&state.0, &request.conversation_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::DatabaseConfig;

    fn send(id: &str) -> SendRequest {
        SendRequest {
            conversation_id: id.into(),
            message: "Give one planning suggestion.".into(),
            acknowledgment: "simulation".into(),
        }
    }

    #[test]
    fn owner_drop_aborts_the_task_and_holds_generation_until_future_destruction(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let state = Arc::new(Mutex::new(Session::default()));
                let weak = Arc::downgrade(&state);
                let bound = begin_conversation(&state, storage.agent_profile("research")?)?;
                start_owned(&state, storage, send(&bound.conversation_id), || {
                    Err(DirectError::MissingKey)
                })?;
                tokio::task::yield_now().await;
                drop(state);
                assert!(weak.upgrade().is_none());
                assert!(GenerationLease::acquire().is_err());
                tokio::task::yield_now().await;
                drop(GenerationLease::acquire()?);
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }

    #[test]
    fn timeout_and_provider_error_release_only_after_cleanup_without_fallback(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::sync::atomic::{AtomicBool, Ordering};
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                for expected in [DirectError::Network, DirectError::Timeout] {
                    let dropped = Arc::new(AtomicBool::new(false));
                    let state = Arc::new(Mutex::new(Session::default()));
                    begin_conversation(&state, storage.agent_profile("research")?)?;
                    let handle = {
                        let mut session = state.lock().map_err(|_| "lock")?;
                        let handle = session.host.start_direct()?.presentation_handle().clone();
                        session.handle = Some(handle.clone());
                        handle
                    };
                    let adapter = if expected == DirectError::Timeout {
                        AdapterRequest::Pending(Arc::clone(&dropped))
                    } else {
                        AdapterRequest::Failure(expected)
                    };
                    let lease = GenerationLease::acquire()?;
                    assert!(GenerationLease::acquire().is_err());
                    execute(
                        Arc::downgrade(&state),
                        handle,
                        "synthetic".into(),
                        adapter,
                        lease,
                        Duration::ZERO,
                    )
                    .await;
                    let snapshot = state.lock().map_err(|_| "lock")?.snapshot()?;
                    assert_eq!(snapshot.status, "error");
                    assert_eq!(snapshot.error, Some(ChatError::Provider(expected)));
                    assert!(!snapshot.busy);
                    assert!(snapshot.text.is_empty());
                    if expected == DirectError::Timeout {
                        assert!(dropped.load(Ordering::SeqCst));
                    }
                    drop(GenerationLease::acquire()?);
                }
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }
    #[test]
    fn native_simulation_completion_duplicate_cancel_and_new_agent_isolation(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let state = Arc::new(Mutex::new(Session::default()));
                let first = begin_conversation(&state, storage.agent_profile("research")?)?;
                let started = start_owned(&state, storage, send(&first.conversation_id), || {
                    Err(DirectError::MissingKey)
                })?;
                assert!(started.busy);
                assert!(matches!(
                    start_owned(&state, storage, send(&first.conversation_id), || Err(
                        DirectError::MissingKey
                    )),
                    Err(ChatError::Busy)
                ));
                assert!(matches!(
                    begin_conversation(&state, storage.agent_profile("coding")?),
                    Err(ChatError::Busy)
                ));
                // The same native lease prevents overlap with the unchanged diagnostic path.
                let mut fixed = PersonalAssistantV0Host::default();
                assert!(fixed.start_direct().is_err());
                tokio::time::sleep(Duration::from_millis(210)).await;
                let streaming = state.lock().map_err(|_| "lock")?.snapshot()?;
                assert_eq!(streaming.status, "streaming");
                assert!(!streaming.text.is_empty());
                let stopped = stop_owned(&state, &first.conversation_id).await?;
                assert_eq!(stopped.status, "stopped");
                assert!(!stopped.busy);
                let next = begin_conversation(&state, storage.agent_profile("coding")?)?;
                assert_ne!(first.conversation_id, next.conversation_id);
                assert!(next.text.is_empty());
                assert!(matches!(
                    start_owned(&state, storage, send(&first.conversation_id), || Err(
                        DirectError::MissingKey
                    )),
                    Err(ChatError::InvalidRequest)
                ));
                start_owned(&state, storage, send(&next.conversation_id), || {
                    Err(DirectError::MissingKey)
                })?;
                tokio::time::sleep(Duration::from_millis(650)).await;
                let done = state.lock().map_err(|_| "lock")?.snapshot()?;
                assert_eq!(done.status, "completed");
                assert!(!done.busy);
                assert!(done.text.contains("Simulation only"));
                let fixed_start = fixed.start_direct()?;
                fixed.cancel(fixed_start.presentation_handle())?;
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }

    #[test]
    fn missing_key_acknowledgement_and_stale_notes_fail_before_generation_without_fallback(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        let profile = storage.save_agent_preferences(AgentPreferencesInput {
            agent_id: "research".into(),
            connection: AgentConnection::OpenaiApi,
            model: "gpt-5.6-luna".into(),
            effort: ReasoningEffort::Default,
            endpoint: String::new(),
            local_auth: false,
            allow_unknown_locality_notes: false,
            owner_instructions: String::new(),
            memory_mode: MemoryMode::PrivateNotes,
            note: "private-fixture".into(),
            revision: 0,
        })?;
        let state = Arc::new(Mutex::new(Session::default()));
        let bound = begin_conversation(&state, profile.clone())?;
        assert!(matches!(
            start_owned(&state, storage, send(&bound.conversation_id), || Err(
                DirectError::MissingKey
            )),
            Err(ChatError::InvalidRequest)
        ));
        let mut request = send(&bound.conversation_id);
        request.acknowledgment = "openai-agent-text-v1".into();
        assert!(matches!(
            start_owned(&state, storage, request, || Err(DirectError::MissingKey)),
            Err(ChatError::Provider(DirectError::MissingKey))
        ));
        assert!(state.lock().map_err(|_| "lock")?.task.is_none());
        assert_eq!(state.lock().map_err(|_| "lock")?.snapshot()?.status, "idle");
        storage.clear_agent_note("research", profile.revision)?;
        assert!(matches!(
            start_owned(&state, storage, send(&bound.conversation_id), || Err(
                DirectError::MissingKey
            )),
            Err(ChatError::StaleContext)
        ));
        assert!(state.lock().map_err(|_| "lock")?.task.is_none());
        Ok(())
    }

    #[test]
    fn native_catalog_binds_destination_auth_model_and_rejects_cloud_or_unsupported_effort(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::agent::definition::AgentId;
        let mut profile = AgentProfile::defaults(AgentId::Research)?;
        profile.connection = AgentConnection::LmStudio;
        profile.endpoint = "http://127.0.0.1:1234/v1".into();
        profile.model = "fixture/model:q4".into();
        let mut model = ModelInfo::unknown(profile.model.clone());
        model.availability = "available".into();
        let mut catalogs = vec![CatalogEntry {
            connection: profile.connection,
            endpoint: profile.endpoint.clone(),
            local_auth: false,
            models: vec![model],
        }];
        validate_catalog(&catalogs, &profile)?;
        profile.endpoint = "http://127.0.0.1:1235/v1".into();
        assert_eq!(
            validate_catalog(&catalogs, &profile),
            Err(DirectError::Catalog)
        );
        profile.endpoint = catalogs[0].endpoint.clone();
        profile.local_auth = true;
        assert_eq!(
            validate_catalog(&catalogs, &profile),
            Err(DirectError::Catalog)
        );
        profile.local_auth = false;
        profile.effort = ReasoningEffort::High;
        assert!(validate_catalog(&catalogs, &profile).is_err());
        profile.effort = ReasoningEffort::Default;
        catalogs[0].models[0].locality = "cloud".into();
        assert_eq!(
            validate_catalog(&catalogs, &profile),
            Err(DirectError::Unsupported)
        );
        profile.connection = AgentConnection::AnthropicApi;
        profile.endpoint.clear();
        profile.model = "claude-haiku-4-5-20251001".into();
        validate_catalog(&[], &profile)?;
        profile.effort = ReasoningEffort::Max;
        assert!(validate_catalog(&[], &profile).is_err());
        Ok(())
    }

    #[test]
    fn ipc_has_no_context_provider_rpc_or_credential_input() {
        for raw in [
            r#"{"conversationId":"agent-chat-1","message":"Hello","acknowledgment":"simulation","agentId":"coding"}"#,
            r#"{"conversationId":"agent-chat-1","message":"Hello","acknowledgment":"simulation","apiKey":"fixture"}"#,
            r#"{"conversationId":"agent-chat-1","message":"Hello","acknowledgment":"simulation","context":"fixture"}"#,
            r#"{"conversationId":"agent-chat-1","message":"Hello","acknowledgment":"simulation","method":"thread/start"}"#,
        ] {
            assert!(serde_json::from_str::<SendRequest>(raw).is_err());
        }
        let codex = list_agent_connections()
            .into_iter()
            .find(|c| c.connection == AgentConnection::Codex);
        assert!(codex.is_some_and(|c| c.status == "blocked" && c.message == CODEX_LIMITATION));
    }
}
