//! Closed IPC for saved profiles and one native-owned, bounded text generation.
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::agent_adapter::{run_adapter_traced, AdapterRequest};
use crate::agent_chat::{BoundConversation, ChatError};
use crate::agent_preferences::{
    AgentConnection, AgentPreferencesInput, AgentProfile, MemoryMode, ReasoningEffort,
};
use crate::codex_connection;
use crate::personal_assistant_direct::{ApiKey, DirectError, GenerationLease, ProviderEvent};
use crate::personal_assistant_v0::{
    PersonalAssistantV0FailureCode, PersonalAssistantV0Host, PersonalAssistantV0PresentationHandle,
    PersonalAssistantV0Snapshot,
};
use crate::storage::Storage;
use crate::{
    agent_models::{validate_selection, ModelInfo},
    anthropic, local_models,
};

#[derive(Clone, Default)]
pub(crate) struct AgentChatState(Arc<Mutex<Session>>);

#[derive(Default)]
struct Session {
    voice_lease: Option<String>,
    conversation: Option<BoundConversation>,
    journal: Option<Storage>,
    turn: Option<u64>,
    omitted_turns: usize,
    catalogs: Vec<CatalogEntry>,
    #[cfg(test)]
    next_id: u64,
    host: PersonalAssistantV0Host,
    handle: Option<PersonalAssistantV0PresentationHandle>,
    task: Option<JoinHandle<()>>,
    stopping: bool,
    codex_cancel: Option<(
        PersonalAssistantV0PresentationHandle,
        codex_connection::CancellationHandle,
    )>,
    error: Option<ChatError>,
    diagnostic: Option<crate::diagnostics::Observer>,
}

impl Drop for Session {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            if let Some(cancellation) = self.owned_codex_cancellation() {
                cancellation.request_drop();
            }
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
    request_id: Option<String>,
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

// Preserve the host's bounded-resource failure through callback, poll and
// abort paths. A local limit is not evidence of malformed provider data.
fn presentation_error(code: PersonalAssistantV0FailureCode) -> DirectError {
    match code {
        PersonalAssistantV0FailureCode::DeadlineExceeded
        | PersonalAssistantV0FailureCode::ProviderTimeout => DirectError::Timeout,
        PersonalAssistantV0FailureCode::LimitExceeded => DirectError::Limit,
        _ => DirectError::Protocol,
    }
}

impl Session {
    fn owned_codex_cancellation(&self) -> Option<&codex_connection::CancellationHandle> {
        self.codex_cancel
            .as_ref()
            .and_then(|(owner, cancellation)| {
                (self.handle.as_ref() == Some(owner)).then_some(cancellation)
            })
    }
    fn checkpoint(
        &mut self,
        status: crate::storage::conversations::TurnStatus,
    ) -> Result<(), ChatError> {
        if let (Some(storage), Some(turn), Some(bound), Some(handle)) =
            (&self.journal, self.turn, &self.conversation, &self.handle)
        {
            let snapshot = self
                .host
                .snapshot(handle)
                .map_err(|_| ChatError::Internal)?;
            storage.checkpoint_chat_turn(&bound.id, turn, snapshot.accepted_text(), status)?;
            if status == crate::storage::conversations::TurnStatus::Failed {
                use crate::storage::conversations::FailureReason;
                let reason = match self.error {
                    Some(ChatError::Provider(DirectError::Timeout)) => FailureReason::Timeout,
                    Some(ChatError::Provider(DirectError::Limit)) | Some(ChatError::Limit) => {
                        FailureReason::Limit
                    }
                    Some(ChatError::Internal) => FailureReason::Storage,
                    _ => FailureReason::Provider,
                };
                storage.annotate_chat_failure(&bound.id, turn, reason)?;
            }
        }
        Ok(())
    }

    fn check_id(&self, id: &str) -> Result<(), ChatError> {
        if self.conversation.as_ref().is_some_and(|c| c.id == id) {
            Ok(())
        } else {
            Err(ChatError::InvalidRequest)
        }
    }

    fn owned(&self) -> bool {
        if self.voice_lease.is_some() {
            return true;
        }
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
                        if let PersonalAssistantV0Snapshot::Failed(failed) = &snapshot {
                            if let Some(observer) = &self.diagnostic {
                                observer.aborting_with(presentation_error(failed.failure().code()));
                            }
                        }
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
                        self.error = Some(ChatError::Provider(presentation_error(
                            failed.failure().code(),
                        )));
                    }
                    "error"
                }
            };
            text = snapshot.accepted_text().to_owned();
            sequence = snapshot.sequence();
        }
        if matches!(status, "completed" | "stopped" | "error") {
            use crate::storage::conversations::TurnStatus;
            let terminal = match status {
                "completed" => TurnStatus::Completed,
                "stopped" => TurnStatus::Cancelled,
                _ => TurnStatus::Failed,
            };
            // Poll can observe a host deadline before the transport task settles.
            // Persist that terminal outcome even when the task is aborted by polling.
            if self.checkpoint(terminal).is_err() {
                self.error = Some(ChatError::Internal);
                status = "error";
            }
        }
        if self.error == Some(ChatError::Internal) && status == "completed" {
            status = "error";
        }
        let bound = self
            .conversation
            .as_ref()
            .ok_or(ChatError::InvalidRequest)?;
        if !self.owned() && matches!(status, "completed" | "stopped" | "error") {
            if let Some(observer) = &self.diagnostic {
                observer.ownership_released();
            }
        }
        Ok(ChatSnapshot {
            version: 1,
            request_id: self.diagnostic.as_ref().and_then(|o| o.request_id()),
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

#[cfg(test)]
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
    session.journal = None;
    session.turn = None;
    session.conversation = Some(bound);
    session.handle = None;
    session.error = None;
    session.diagnostic = None;
    session.task = None;
    session.snapshot()
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
    if let Some(journal) = session.journal.clone() {
        let saved = journal.conversation(&request.conversation_id)?;
        let current = journal.agent_profile(&saved.agent_id)?;
        if !saved.compatible(&current) {
            return Err(ChatError::StaleContext);
        }
        let bound = session
            .conversation
            .as_mut()
            .ok_or(ChatError::InvalidRequest)?;
        // Latest explicit notes/Memory Off apply to the next request. Runtime stays pinned.
        bound.profile = current;
        session.omitted_turns = bound.restore_history(&saved);
    } else if session.handle.is_some()
        && !matches!(session.snapshot()?.status, "idle" | "completed")
    {
        return Err(ChatError::InvalidRequest);
    }
    let bound = session
        .conversation
        .as_ref()
        .ok_or(ChatError::InvalidRequest)?;
    let mut diagnostic = crate::diagnostics::Attempt::new(
        bound.profile.connection,
        &bound.profile.model,
        Some(&bound.id),
        None,
        None,
    );
    let observer = diagnostic.observer();
    let prepared = (|| -> Result<_, ChatError> {
        // The caller supplies only a conversation ID and message, never context or agent authority.
        let current = storage.agent_profile(&bound.profile.agent_id)?;
        bound.validate_send(&current, &request.message)?;
        observer.record(
            crate::diagnostics::Event::ConfigurationValidated,
            crate::diagnostics::Outcome::Observed,
            None,
        );
        let adapter = match bound.profile.connection {
            AgentConnection::Codex => {
                if request.acknowledgment != "codex-agent-text-v1" {
                    return Err(ChatError::InvalidRequest);
                }
                validate_catalog(&session.catalogs, &bound.profile)?;
                AdapterRequest::Codex {
                    setup: codex_connection::Setup::from_environment()?,
                    model: bound.profile.model.clone(),
                    effort: bound.profile.effort,
                    input: bound.codex_input(&request.message)?,
                }
            }
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
        if matches!(
            bound.profile.connection,
            AgentConnection::OpenaiApi | AgentConnection::AnthropicApi
        ) {
            observer.record(
                crate::diagnostics::Event::CredentialsAvailable,
                crate::diagnostics::Outcome::Observed,
                None,
            );
        }
        Ok((adapter, GenerationLease::acquire()?))
    })();
    let (adapter, lease) = match prepared {
        Ok(value) => value,
        Err(error) => {
            diagnostic.fail_preparation(error);
            session.diagnostic = Some(observer);
            return Err(error);
        }
    };
    if let Some(journal) = &session.journal {
        session.turn = Some(journal.begin_chat_turn(&request.conversation_id, &request.message)?);
    }
    session.diagnostic = Some(observer);
    // Presentation ownership and transport cleanup must both finish before reuse.
    let start = session.host.start_direct().map_err(|_| {
        diagnostic.fail_preparation(ChatError::Busy);
        ChatError::Busy
    })?;
    let handle = start.presentation_handle().clone();
    session.codex_cancel = match &adapter {
        AdapterRequest::Codex { setup, .. } => Some((handle.clone(), setup.cancellation_handle())),
        _ => None,
    };
    session.handle = Some(handle.clone());
    session.error = None;
    session.task = Some(tokio::spawn(execute(
        Arc::downgrade(state),
        handle,
        request.message,
        adapter,
        lease,
        Duration::from_secs(60),
        diagnostic,
    )));
    session.snapshot()
}

async fn execute(
    state: Weak<Mutex<Session>>,
    handle: PersonalAssistantV0PresentationHandle,
    prompt: String,
    adapter: AdapterRequest,
    _lease: GenerationLease,
    deadline: Duration,
    mut diagnostic: crate::diagnostics::Attempt,
) {
    let observer = diagnostic.observer();
    let mut completed = false;
    let result = tokio::time::timeout(
        deadline,
        run_adapter_traced(adapter, observer, |event| {
            let state = state.upgrade().ok_or(DirectError::InvalidRequest)?;
            let mut session = state.lock().map_err(|_| DirectError::Internal)?;
            if session.handle.as_ref() != Some(&handle) {
                return Err(DirectError::InvalidRequest);
            }
            if session.stopping {
                if let Some(cancellation) = session.owned_codex_cancellation() {
                    cancellation.stopping_callback();
                }
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
            if let PersonalAssistantV0Snapshot::Failed(failed) = &snapshot {
                session.error = Some(ChatError::Provider(presentation_error(
                    failed.failure().code(),
                )));
                session
                    .checkpoint(crate::storage::conversations::TurnStatus::Failed)
                    .map_err(|_| DirectError::Internal)?;
                return Err(presentation_error(failed.failure().code()));
            }
            session
                .checkpoint(crate::storage::conversations::TurnStatus::Running)
                .map_err(|_| DirectError::Internal)?;
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
    let mut final_result = result;
    // The transport future is dropped before the terminal transition releases the lease.
    let Some(state) = state.upgrade() else {
        return;
    };
    if let Ok(mut session) = state.lock() {
        if session.handle.as_ref() != Some(&handle) || session.stopping {
            return;
        }
        match session.host.snapshot(&handle) {
            Ok(PersonalAssistantV0Snapshot::Failed(failed)) => {
                let _ = session.checkpoint(crate::storage::conversations::TurnStatus::Failed);
                diagnostic.finish(Err(presentation_error(failed.failure().code())));
                return;
            }
            Ok(snapshot) if snapshot.is_terminal() => return,
            Err(_) => {
                diagnostic.finish(Err(DirectError::Internal));
                return;
            }
            _ => {}
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
                        final_result = Err(DirectError::Internal);
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
        let status = if final_result.is_ok() {
            crate::storage::conversations::TurnStatus::Completed
        } else {
            crate::storage::conversations::TurnStatus::Failed
        };
        if session.checkpoint(status).is_err() {
            session.error = Some(ChatError::Internal);
            final_result = Err(DirectError::Internal);
        }
    };
    diagnostic.finish(final_result);
}

async fn stop_owned(state: &Arc<Mutex<Session>>, id: &str) -> Result<ChatSnapshot, ChatError> {
    let task = {
        let mut session = state.lock().map_err(|_| ChatError::Internal)?;
        session.check_id(id)?;
        if session.voice_lease.is_some() {
            return Err(ChatError::Busy);
        }
        if session.stopping {
            return session.snapshot();
        }
        // Record intent for this exact native request before aborting its task.
        if let Some(cancellation) = session.owned_codex_cancellation() {
            cancellation.request_stop();
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
    if let Some(handle) = session.handle.clone() {
        let status = match session
            .host
            .snapshot(&handle)
            .map_err(|_| ChatError::Internal)?
        {
            PersonalAssistantV0Snapshot::Completed(_) => {
                crate::storage::conversations::TurnStatus::Completed
            }
            PersonalAssistantV0Snapshot::Failed(_) => {
                crate::storage::conversations::TurnStatus::Failed
            }
            _ => crate::storage::conversations::TurnStatus::Cancelled,
        };
        session.checkpoint(status)?;
    }
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
        AgentConnection::AnthropicApi | AgentConnection::Codex
            if request.endpoint.is_empty() && !request.local_auth =>
        {
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
        if request.connection == AgentConnection::Codex {
            codex_connection::discover().await
        } else if request.connection == AgentConnection::AnthropicApi {
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
    Ok(storage
        .save_agent_preferences(request)
        .inspect_err(|error| {
            crate::diagnostics::event(
                crate::diagnostics::Event::ConfigurationFailure,
                crate::diagnostics::Outcome::Failed,
                Some(match error {
                    crate::agent_preferences::PreferencesError::Storage => {
                        crate::diagnostics::Category::Storage
                    }
                    _ => crate::diagnostics::Category::Configuration,
                }),
            );
        })?)
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
            status: if codex_connection::Setup::from_environment().is_ok() { "owner_setup_required" } else { "blocked" },
            message: "Requires Codex 0.159.0, CORTEXA_CODEX_DEMO=1 and a dedicated owner-authenticated CORTEXA_CODEX_HOME without user configuration. Refresh models explicitly. No OpenAI API key inheritance or fallback.",
        },
    ]
}
#[tauri::command]
pub(crate) fn start_agent_conversation(
    request: AgentRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
    if session.owned() {
        return Err(ChatError::Busy);
    }
    let profile = storage.agent_profile(&request.agent_id)?;
    let saved = storage.create_conversation(&profile)?;
    open_saved(&mut session, &storage, saved, profile)
}
#[tauri::command]
pub(crate) async fn send_agent_message(
    request: SendRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    start_owned(&state.0, &storage, request, ApiKey::from_environment).inspect_err(|error| {
        let category = match error {
            ChatError::Provider(e) => crate::diagnostics::Category::from(*e),
            _ => crate::diagnostics::Category::Configuration,
        };
        crate::diagnostics::event(
            crate::diagnostics::Event::ConfigurationFailure,
            crate::diagnostics::Outcome::Failed,
            Some(category),
        );
    })
}
#[tauri::command]
pub(crate) fn poll_agent_conversation(
    request: ConversationRequest,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
    session.check_id(&request.conversation_id)?;
    let snapshot = session.snapshot()?;
    if !snapshot.busy && matches!(snapshot.status, "completed" | "stopped" | "error") {
        if let (Some(observer), Some(id)) = (&session.diagnostic, &snapshot.request_id) {
            observer.ui_returned(id);
        }
    }
    Ok(snapshot)
}
#[tauri::command]
pub(crate) async fn cancel_agent_conversation(
    request: ConversationRequest,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    stop_owned(&state.0, &request.conversation_id).await
}

/// Native collaboration reuses the saved selection and already-discovered catalog.
/// This never discovers a model, retries, or changes the single-conversation context.
pub(crate) fn collaboration_adapter(
    state: &AgentChatState,
    profile: crate::agent_preferences::AgentProfile,
    prompt: &str,
) -> Result<AdapterRequest, ChatError> {
    let session = state.0.lock().map_err(|_| ChatError::Internal)?;
    profile.validate()?;
    let bound = BoundConversation::collaboration(profile)?;
    let p = &bound.profile;
    if matches!(
        p.connection,
        AgentConnection::Codex
            | AgentConnection::AnthropicApi
            | AgentConnection::LmStudio
            | AgentConnection::Ollama
    ) {
        validate_catalog(&session.catalogs, p)?;
    }
    Ok(match p.connection {
        AgentConnection::Simulation => AdapterRequest::Simulation,
        AgentConnection::OpenaiApi => AdapterRequest::Openai {
            key: ApiKey::from_environment()?,
            body: bound.request_body(prompt)?,
        },
        AgentConnection::AnthropicApi => AdapterRequest::Anthropic {
            key: anthropic::AnthropicKey::from_environment()?,
            body: bound.provider_request_body(prompt)?,
        },
        AgentConnection::LmStudio | AgentConnection::Ollama => AdapterRequest::Local {
            endpoint: p.endpoint.clone(),
            key: local_models::LocalKey::from_environment(p.connection, &p.endpoint, p.local_auth)?,
            body: bound.provider_request_body(prompt)?,
        },
        AgentConnection::Codex => AdapterRequest::CollaborationCodex {
            setup: codex_connection::Setup::from_environment()?,
            model: p.model.clone(),
            effort: p.effort,
            input: bound.codex_input(prompt)?,
        },
    })
}

fn open_saved(
    session: &mut Session,
    storage: &Storage,
    saved: crate::storage::conversations::Conversation,
    mut profile: AgentProfile,
) -> Result<ChatSnapshot, ChatError> {
    profile.connection = saved.connection;
    profile.model = saved.model.clone();
    profile.endpoint = saved.endpoint.clone();
    profile.effort = saved.effort;
    profile.local_auth = false;
    profile.allow_unknown_locality_notes = false;
    let mut bound = BoundConversation::new(saved.id.clone(), profile)?;
    session.omitted_turns = bound.restore_history(&saved);
    session.conversation = Some(bound);
    session.handle = None;
    session.task = None;
    session.turn = None;
    session.error = None;
    session.diagnostic = None;
    session.journal = Some(storage.clone());
    session.snapshot()
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct RenameRequest {
    conversation_id: String,
    title: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct FactRequest {
    conversation_id: String,
    turn_id: u64,
    text: String,
    revision: u64,
}
#[tauri::command]
pub(crate) fn list_saved_agent_conversations(
    request: AgentRequest,
    storage: tauri::State<'_, Storage>,
) -> Result<Vec<crate::storage::conversations::ConversationSummary>, ChatError> {
    storage.conversations(&request.agent_id)
}
#[tauri::command]
pub(crate) fn read_saved_agent_conversation(
    request: ConversationRequest,
    storage: tauri::State<'_, Storage>,
) -> Result<SavedConversationView, ChatError> {
    let conversation = storage.conversation(&request.conversation_id)?;
    let context_turn_ids = crate::agent_chat::context_turn_ids(&conversation);
    Ok(SavedConversationView {
        conversation,
        context_turn_ids,
    })
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SavedConversationView {
    #[serde(flatten)]
    conversation: crate::storage::conversations::Conversation,
    context_turn_ids: Vec<u64>,
}

#[tauri::command]
pub(crate) fn open_saved_agent_conversation(
    request: ConversationRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<ChatSnapshot, ChatError> {
    let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
    if session.owned() {
        return Err(ChatError::Busy);
    }
    let saved = storage.conversation(&request.conversation_id)?;
    let profile = storage.agent_profile(&saved.agent_id)?;
    open_saved(&mut session, &storage, saved, profile)
}
#[tauri::command]
pub(crate) fn rename_saved_agent_conversation(
    request: RenameRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<(), ChatError> {
    let session = state.0.lock().map_err(|_| ChatError::Internal)?;
    if session.owned() {
        return Err(ChatError::Busy);
    }
    storage.rename_conversation(&request.conversation_id, &request.title)
}
#[tauri::command]
pub(crate) fn delete_saved_agent_conversation(
    request: ConversationRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<(), ChatError> {
    let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
    if session.owned() {
        return Err(ChatError::Busy);
    }
    storage.delete_conversation(&request.conversation_id)?;
    if session.check_id(&request.conversation_id).is_ok() {
        session.conversation = None;
        session.handle = None;
        session.turn = None;
        session.journal = None;
    }
    Ok(())
}
#[tauri::command]
pub(crate) fn save_conversation_fact(
    request: FactRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, AgentChatState>,
) -> Result<AgentProfile, ChatError> {
    let session = state.0.lock().map_err(|_| ChatError::Internal)?;
    if session.owned() {
        return Err(ChatError::Busy);
    }
    storage.save_conversation_fact(
        &request.conversation_id,
        request.turn_id,
        &request.text,
        request.revision,
    )
}
#[tauri::command]
pub(crate) fn read_agent_note_provenance(
    request: AgentRequest,
    storage: tauri::State<'_, Storage>,
) -> Result<Option<crate::storage::conversations::NoteProvenance>, ChatError> {
    storage.note_provenance(&request.agent_id)
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

    type ControlledEvents =
        tokio::sync::mpsc::Sender<(ProviderEvent, tokio::sync::oneshot::Sender<()>)>;

    fn controlled_request(
        state: &Arc<Mutex<Session>>,
        storage: &Storage,
    ) -> Result<(String, ControlledEvents), Box<dyn std::error::Error>> {
        let profile = storage.agent_profile("research")?;
        let saved = storage.create_conversation(&profile)?;
        let mut session = state.lock().map_err(|_| "lock")?;
        let initial = open_saved(&mut session, storage, saved, profile)?;
        session.turn = Some(storage.begin_chat_turn(&initial.conversation_id, "synthetic")?);
        let handle = session.host.start_direct()?.presentation_handle().clone();
        session.handle = Some(handle.clone());
        let (send, receive) = tokio::sync::mpsc::channel(1);
        session.task = Some(tokio::spawn(execute(
            Arc::downgrade(state),
            handle,
            "synthetic".into(),
            AdapterRequest::Stepped(receive),
            GenerationLease::acquire()?,
            Duration::from_secs(60),
            crate::diagnostics::Attempt::new(
                AgentConnection::Simulation,
                "simulation",
                None,
                None,
                None,
            ),
        )));
        Ok((initial.conversation_id, send))
    }

    async fn accepted_step(
        send: &ControlledEvents,
        event: ProviderEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (acknowledge, accepted) = tokio::sync::oneshot::channel();
        send.send((event, acknowledge))
            .await
            .map_err(|_| "closed fixture")?;
        tokio::time::timeout(Duration::from_secs(2), accepted).await??;
        Ok(())
    }

    async fn join_controlled(
        state: &Arc<Mutex<Session>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task = state
            .lock()
            .map_err(|_| "lock")?
            .task
            .take()
            .ok_or("no task")?;
        tokio::time::timeout(Duration::from_secs(2), task).await??;
        Ok(())
    }

    #[test]
    fn codex_cancel_handle_is_bound_to_exact_presentation_and_absent_for_other_adapters(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let mut session = Session::default();
        assert!(session.owned_codex_cancellation().is_none());
        let first = session.host.start_direct()?.presentation_handle().clone();
        session.handle = Some(first.clone());
        session.codex_cancel = Some((
            first.clone(),
            codex_connection::CancellationHandle::default(),
        ));
        assert!(session.owned_codex_cancellation().is_some());
        session.host.cancel(&first)?;
        session.handle = Some(session.host.start_direct()?.presentation_handle().clone());
        assert!(session.owned_codex_cancellation().is_none());
        session.codex_cancel = None;
        assert!(session.owned_codex_cancellation().is_none());
        Ok(())
    }
    #[test]
    fn controlled_stream_stop_completion_races_and_subsequent_send(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                // Active text; completion event before transport release; fully completed.
                // Channels control each boundary, without sleep-based race scheduling.
                for phase in 0..3 {
                    let state = Arc::new(Mutex::new(Session::default()));
                    let (id, events) = controlled_request(&state, storage)?;
                    accepted_step(&events, ProviderEvent::Started("resp_fixture".into())).await?;
                    accepted_step(&events, ProviderEvent::Delta("1\n".into())).await?;
                    let first = state.lock().map_err(|_| "lock")?.snapshot()?;
                    assert_eq!(first.status, "streaming");
                    assert_eq!(first.text, "1\n");
                    assert!(first.busy);
                    accepted_step(&events, ProviderEvent::Delta("2\n".into())).await?;
                    let second = state.lock().map_err(|_| "lock")?.snapshot()?;
                    assert_eq!(second.text, "1\n2\n");
                    assert!(second.sequence > first.sequence);
                    if phase > 0 {
                        accepted_step(&events, ProviderEvent::Completed).await?;
                    }
                    if phase == 2 {
                        drop(events);
                        join_controlled(&state).await?;
                        assert_eq!(
                            state.lock().map_err(|_| "lock")?.snapshot()?.status,
                            "completed"
                        );
                    } else {
                        // Keep the transport open until Stop explicitly aborts and joins it.
                        let stopped = stop_owned(&state, &id).await?;
                        assert_eq!(stopped.status, "stopped");
                        assert!(events.is_closed());
                        drop(events);
                    }
                    for _ in 0..2 {
                        let terminal = stop_owned(&state, &id).await?;
                        assert_eq!(
                            terminal.status,
                            if phase == 2 { "completed" } else { "stopped" }
                        );
                        assert_eq!(terminal.text, "1\n2\n");
                        assert_eq!(terminal.error, None);
                        assert!(!terminal.busy);
                    }
                    let persisted = storage.conversation(&id)?;
                    assert_eq!(persisted.turns.len(), 1);
                    assert_eq!(persisted.turns[0].answer, "1\n2\n");
                    assert!(
                        persisted.turns[0].status
                            == if phase == 2 {
                                crate::storage::conversations::TurnStatus::Completed
                            } else {
                                crate::storage::conversations::TurnStatus::Cancelled
                            }
                    );
                    drop(GenerationLease::acquire()?);
                    // A separate fresh conversation succeeds after cancellation/completion.
                    let (next_id, events) = controlled_request(&state, storage)?;
                    assert_ne!(id, next_id);
                    accepted_step(&events, ProviderEvent::Started("resp_next".into())).await?;
                    accepted_step(&events, ProviderEvent::Delta("OK".into())).await?;
                    accepted_step(&events, ProviderEvent::Completed).await?;
                    drop(events);
                    join_controlled(&state).await?;
                    let next = state.lock().map_err(|_| "lock")?.snapshot()?;
                    assert_eq!(next.status, "completed");
                    assert_eq!(next.text, "OK");
                    assert_eq!(next.error, None);
                    assert!(!next.busy);
                    drop(GenerationLease::acquire()?);
                }
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }

    #[test]
    fn fragmented_stream_limit_preserves_partial_text_and_terminal_reason(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let state = Arc::new(Mutex::new(Session::default()));
                let initial = begin_conversation(&state, storage.agent_profile("research")?)?;
                let handle = {
                    let mut session = state.lock().map_err(|_| "lock")?;
                    let handle = session.host.start_direct()?.presentation_handle().clone();
                    session.handle = Some(handle.clone());
                    handle
                };
                let mut events = vec![ProviderEvent::Started("resp_fixture".into())];
                for number in 1..=64 {
                    events.push(ProviderEvent::Delta(number.to_string()));
                    events.push(ProviderEvent::Delta("\n".into()));
                }
                events.push(ProviderEvent::Completed);
                execute(
                    Arc::downgrade(&state),
                    handle,
                    "synthetic".into(),
                    AdapterRequest::Events(events),
                    GenerationLease::acquire()?,
                    Duration::from_secs(60),
                    crate::diagnostics::Attempt::new(
                        AgentConnection::Simulation,
                        "simulation",
                        None,
                        None,
                        None,
                    ),
                )
                .await;
                let snapshot = state.lock().map_err(|_| "lock")?.snapshot()?;
                assert_eq!(snapshot.status, "error");
                assert_eq!(
                    snapshot.error,
                    Some(ChatError::Provider(DirectError::Limit))
                );
                let expected: String = (1..=63).map(|n| format!("{n}\n")).collect();
                assert_eq!(snapshot.text, expected);
                assert_eq!(snapshot.sequence, 128);
                assert!(!snapshot.busy);
                // Late and repeated Stop must preserve the real failure and partial text.
                for _ in 0..2 {
                    let stopped = stop_owned(&state, &initial.conversation_id).await?;
                    assert_eq!(stopped.status, snapshot.status);
                    assert_eq!(stopped.error, snapshot.error);
                    assert_eq!(stopped.text, expected);
                    assert_eq!(stopped.sequence, snapshot.sequence);
                    assert!(!stopped.busy);
                }
                drop(GenerationLease::acquire()?);
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
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
                        crate::diagnostics::Attempt::new(
                            AgentConnection::Simulation,
                            "simulation",
                            None,
                            None,
                            None,
                        ),
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
            identity: crate::agent_preferences::BotIdentity::default(),
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
        assert!(
            codex.is_some_and(|c| matches!(c.status, "blocked" | "owner_setup_required")
                && c.message.contains("dedicated"))
        );
    }
    #[test]
    fn poll_detected_failure_is_durable_before_aborted_transport_cleanup(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let storage = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let state = Arc::new(Mutex::new(Session::default()));
                let (id, events) = controlled_request(&state, &storage)?;
                accepted_step(&events, ProviderEvent::Started("synthetic".into())).await?;
                accepted_step(&events, ProviderEvent::Delta("retained partial".into())).await?;
                {
                    let mut session = state.lock().map_err(|_| "lock")?;
                    let handle = session.handle.clone().ok_or("missing handle")?;
                    session
                        .host
                        .fail_direct(&handle, PersonalAssistantV0FailureCode::DeadlineExceeded)?;
                    assert_eq!(session.snapshot()?.status, "error");
                }
                let saved = storage.conversation(&id)?;
                assert!(saved.turns[0].status == crate::storage::conversations::TurnStatus::Failed);
                assert!(
                    saved.turns[0].failure
                        == Some(crate::storage::conversations::FailureReason::Timeout)
                );
                assert_eq!(saved.turns[0].answer, "retained partial");
                drop(events);
                stop_owned(&state, &id).await?;
                assert!(
                    storage.conversation(&id)?.turns[0].status
                        == crate::storage::conversations::TurnStatus::Failed
                );
                start_owned(&state, &storage, send(&id), || Err(DirectError::MissingKey))?;
                join_controlled(&state).await?;
                assert_eq!(storage.conversation(&id)?.turns.len(), 2);
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }

    #[test]
    fn durable_same_conversation_recovers_after_stop_without_replaying_partial(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let storage = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let state = Arc::new(Mutex::new(Session::default()));
                let (id, events) = controlled_request(&state, &storage)?;
                accepted_step(&events, ProviderEvent::Started("synthetic".into())).await?;
                accepted_step(&events, ProviderEvent::Delta("partial canary".into())).await?;
                stop_owned(&state, &id).await?;
                drop(events);
                start_owned(&state, &storage, send(&id), || Err(DirectError::MissingKey))?;
                join_controlled(&state).await?;
                let saved = storage.conversation(&id)?;
                assert_eq!(saved.turns.len(), 2);
                assert!(
                    saved.turns[0].status == crate::storage::conversations::TurnStatus::Cancelled
                );
                assert!(
                    saved.turns[1].status == crate::storage::conversations::TurnStatus::Completed
                );
                assert_eq!(saved.turns[0].answer, "partial canary");
                assert!(!state.lock().map_err(|_| "lock")?.snapshot()?.busy);
                let mut bound = BoundConversation::new(id, storage.agent_profile("research")?)?;
                assert_eq!(bound.restore_history(&saved), 1);
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod diagnostic_boundary_tests {
    use super::*;
    #[test]
    fn preparation_failure_and_success_keep_trace_without_provider_or_content_in_id(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&crate::storage::DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let state = Arc::new(Mutex::new(Session::default()));
                let profile = storage.save_agent_preferences(AgentPreferencesInput {
                    identity: crate::agent_preferences::BotIdentity::default(),
                    agent_id: "personal-assistant".into(),
                    connection: AgentConnection::OpenaiApi,
                    model: "gpt-5.6-luna".into(),
                    effort: ReasoningEffort::Low,
                    endpoint: String::new(),
                    local_auth: false,
                    allow_unknown_locality_notes: false,
                    owner_instructions: String::new(),
                    memory_mode: MemoryMode::Off,
                    note: String::new(),
                    revision: 0,
                })?;
                let initial = begin_conversation(&state, profile)?;
                let result = start_owned(
                    &state,
                    storage,
                    SendRequest {
                        conversation_id: initial.conversation_id.clone(),
                        message: "DUMMY-PRIVATE-PROMPT".into(),
                        acknowledgment: "openai-agent-text-v1".into(),
                    },
                    || Err(DirectError::MissingKey),
                );
                assert!(matches!(
                    result,
                    Err(ChatError::Provider(DirectError::MissingKey))
                ));
                let failed = state.lock().map_err(|_| "lock")?.snapshot()?;
                assert!(!failed.busy);
                let failed_id = failed.request_id.ok_or("missing trace")?;
                assert!(failed_id.starts_with("sha256:"));
                assert!(!failed_id.contains("DUMMY"));
                let next = begin_conversation(&state, storage.agent_profile("research")?)?;
                assert!(next.request_id.is_none());
                let started = start_owned(
                    &state,
                    storage,
                    SendRequest {
                        conversation_id: next.conversation_id.clone(),
                        message: "DUMMY-SIMULATION".into(),
                        acknowledgment: "simulation".into(),
                    },
                    || Err(DirectError::MissingKey),
                )?;
                assert_ne!(started.request_id.as_ref(), Some(&failed_id));
                let id = started.request_id.ok_or("missing trace")?;
                let stopped = stop_owned(&state, &next.conversation_id).await?;
                assert!(!stopped.busy);
                assert_eq!(stopped.request_id.as_deref(), Some(id.as_str()));
                assert!(GenerationLease::acquire().is_ok());
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }
}

// Voice may read current presentation only; it never constructs provider context.
pub(crate) fn claim_voice(
    state: &AgentChatState,
    conversation_id: &str,
    owner: &str,
    speak: bool,
) -> Result<Option<String>, crate::voice::VoiceError> {
    use crate::voice::VoiceError;
    let mut session = state.0.lock().map_err(|_| VoiceError::CleanupFailed)?;
    session
        .check_id(conversation_id)
        .map_err(|_| VoiceError::StaleSession)?;
    if session.owned() {
        return Err(VoiceError::Busy);
    }
    let bound = session
        .conversation
        .as_ref()
        .ok_or(VoiceError::StaleSession)?;
    if bound.profile.agent_id != "personal-assistant" {
        return Err(VoiceError::Unsupported);
    }
    let text = if speak {
        let snapshot = session.snapshot().map_err(|_| VoiceError::Unavailable)?;
        if snapshot.status != "completed"
            || snapshot.text.is_empty()
            || snapshot.text.chars().count() > 4096
        {
            return Err(VoiceError::Unavailable);
        }
        Some(snapshot.text)
    } else {
        None
    };
    session.voice_lease = Some(owner.to_owned());
    Ok(text)
}
pub(crate) fn release_voice(state: &AgentChatState, owner: &str) {
    if let Ok(mut session) = state.0.lock() {
        if session.voice_lease.as_deref() == Some(owner) {
            session.voice_lease = None;
        }
    }
}

#[cfg(test)]
mod voice_ownership_tests {
    use super::*;
    use crate::voice::VoiceError;
    #[test]
    fn voice_owner_blocks_send_switch_and_wrong_release_then_recovers(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let initialized = Storage::initialize(&crate::storage::DatabaseConfig::in_memory())?;
        let storage = initialized.storage();
        let state = AgentChatState::default();
        let initial = begin_conversation(&state.0, storage.agent_profile("personal-assistant")?)?;
        assert_eq!(
            claim_voice(&state, "wrong", "a", false),
            Err(VoiceError::StaleSession)
        );
        assert_eq!(
            claim_voice(&state, &initial.conversation_id, "a", true),
            Err(VoiceError::Unavailable)
        );
        assert!(claim_voice(&state, &initial.conversation_id, "a", false).is_ok());
        assert_eq!(
            claim_voice(&state, &initial.conversation_id, "b", false),
            Err(VoiceError::Busy)
        );
        let send = || SendRequest {
            conversation_id: initial.conversation_id.clone(),
            message: "Synthetic typed equivalent".into(),
            acknowledgment: "simulation".into(),
        };
        assert!(matches!(
            start_owned(&state.0, storage, send(), || Err(DirectError::MissingKey)),
            Err(ChatError::Busy)
        ));
        assert!(matches!(
            begin_conversation(&state.0, storage.agent_profile("research")?),
            Err(ChatError::Busy)
        ));
        release_voice(&state, "stale");
        assert!(state.0.lock().map_err(|_| "lock")?.owned());
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                assert!(matches!(
                    stop_owned(&state.0, &initial.conversation_id).await,
                    Err(ChatError::Busy)
                ));
                release_voice(&state, "a");
                let next = start_owned(&state.0, storage, send(), || Err(DirectError::MissingKey))?;
                assert!(next.busy);
                let task = state
                    .0
                    .lock()
                    .map_err(|_| "lock")?
                    .task
                    .take()
                    .ok_or("missing task")?;
                task.await?;
                assert_eq!(
                    state.0.lock().map_err(|_| "lock")?.snapshot()?.status,
                    "completed"
                );
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        let other = begin_conversation(&state.0, storage.agent_profile("research")?)?;
        assert_eq!(
            claim_voice(&state, &other.conversation_id, "c", false),
            Err(VoiceError::Unsupported)
        );
        Ok(())
    }
}

pub(crate) fn validate_voice_scope(
    state: &AgentChatState,
    id: &str,
) -> Result<(), crate::voice::VoiceError> {
    let session = state
        .0
        .lock()
        .map_err(|_| crate::voice::VoiceError::CleanupFailed)?;
    session
        .check_id(id)
        .map_err(|_| crate::voice::VoiceError::StaleSession)
}
