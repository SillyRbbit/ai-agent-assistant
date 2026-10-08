//! Narrow room commands. Only Rust selects participants and advances stages.
use crate::{
    agent_adapter::{run_adapter_traced, AdapterRequest},
    agent_chat::ChatError,
    agent_chat_tauri::{action_adapter, collaboration_adapter, AgentChatState},
    agent_preferences::AgentConnection,
    collaboration::*,
    personal_assistant_direct::{DirectError, GenerationLease},
    storage::Storage,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::Manager;
#[derive(Default)]
pub(crate) struct CollaborationState(Arc<Mutex<Session>>);
#[derive(Default)]
struct Session {
    prepared: Option<(i64, Run)>,
    preview_serial: u64,
    active: Option<(i64, String)>,
    task: Option<tokio::task::JoinHandle<()>>,
    stopping: bool,
    selected: Option<(u64, std::path::PathBuf)>,
    action: Option<(String, Arc<crate::isolated_action::workspace::Workspace>)>,
}
impl Drop for Session {
    fn drop(&mut self) {
        if let Some(t) = self.task.take() {
            t.abort();
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct RoomRequest {
    room_id: i64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CreateRequest {
    title: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PrepareRequest {
    room_id: i64,
    input: Objective,
    action: Option<ActionSelection>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ActionSelection {
    selection: u64,
    file: String,
    test_file: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SelectedRepository {
    selection: u64,
    path: String,
}
#[cfg(target_os = "macos")]
fn choose_action_folder() -> Result<std::path::PathBuf, ChatError> {
    rfd::FileDialog::new()
        .set_title("Select a clean, small Git repository for one Python change")
        .pick_folder()
        .ok_or(crate::isolated_action::ActionError::Target.into())
}
#[cfg(not(target_os = "macos"))]
fn choose_action_folder() -> Result<std::path::PathBuf, ChatError> {
    Err(crate::isolated_action::ActionError::Scope.into())
}
#[tauri::command]
pub(crate) fn select_action_repository(
    state: tauri::State<'_, CollaborationState>,
) -> Result<SelectedRepository, ChatError> {
    let mut s = state.0.lock().map_err(|_| ChatError::Internal)?;
    if s.active.is_some() || s.stopping {
        return Err(ChatError::Busy);
    }
    let path = choose_action_folder()?;
    s.preview_serial = s.preview_serial.checked_add(1).ok_or(ChatError::Limit)?;
    let selection = s.preview_serial;
    s.selected = Some((selection, path.clone()));
    s.prepared = None;
    Ok(SelectedRepository {
        selection,
        path: path.to_string_lossy().into_owned(),
    })
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ReviewRequest {
    room_id: i64,
    run_id: String,
    review_hash: String,
}
#[tauri::command]
pub(crate) fn review_action_change(
    window: tauri::WebviewWindow,
    request: ReviewRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, CollaborationState>,
) -> Result<Room, ChatError> {
    use crate::isolated_action::ActionError;
    let s = state.0.lock().map_err(|_| ChatError::Internal)?;
    if s.active.is_some() || s.stopping {
        return Err(ChatError::Busy);
    }
    let (bound, workspace) = s.action.as_ref().ok_or(ActionError::Recovery)?;
    if bound != &request.run_id {
        return Err(ActionError::Drift.into());
    }
    let mut r = room(&storage, request.room_id)?;
    let run = r.runs.last_mut().ok_or(ActionError::Drift)?;
    let evidence = run.action.as_mut().ok_or(ActionError::Drift)?;
    if run.id != request.run_id
        || run.status != Status::Completed
        || evidence.disposition != "review_ready"
        || evidence.review_hash.as_deref() != Some(&request.review_hash)
        || evidence.review_hash()? != request.review_hash
    {
        return Err(ActionError::Drift.into());
    }
    workspace.recheck()?;
    crate::isolated_action::sandbox::verify_cleanup(&workspace.area)?;
    // This native decision is not supplied by the WebView or by a model.
    #[cfg(target_os = "macos")]
    let approved = {
        let source =
            crate::approvals::decision_source::MacOsNativeApprovalDecisionSource::new(&window)
                .map_err(|_| ActionError::Approval)?;
        workspace.native_approval(&request.run_id, evidence, &request.review_hash, &source)?
    };
    #[cfg(not(target_os = "macos"))]
    let approved = {
        let _ = window;
        workspace.native_approval(&request.run_id, evidence, &request.review_hash)?
    };
    if let Some(grant) = approved {
        // Persist intent first. A restart requires recovery review, never replay.
        evidence.disposition = "applying".into();
        run.sequence += 1;
        storage.save_collaboration_room(&r)?;
        let run = r.runs.last_mut().ok_or(ActionError::Drift)?;
        let evidence = run.action.as_mut().ok_or(ActionError::Drift)?;
        let applied = workspace.apply(evidence, grant);
        evidence.disposition = if applied.is_ok() {
            "applied"
        } else {
            "recovery_required"
        }
        .into();
        run.sequence += 1;
        storage.save_collaboration_room(&r)?;
        applied?;
    } else {
        evidence.disposition = "rejected_or_expired".into();
        run.sequence += 1;
        storage.save_collaboration_room(&r)?;
    }
    Ok(r)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct StartRequest {
    room_id: i64,
    run_id: String,
    preview_serial: u64,
    acknowledgment: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Preview {
    preview_serial: u64,
    room_id: i64,
    run: Run,
    maximum_calls: usize,
    simulation: bool,
}
fn room(storage: &Storage, id: i64) -> Result<Room, ChatError> {
    storage
        .collaboration_rooms()?
        .into_iter()
        .find(|r| r.id == id)
        .ok_or(ChatError::InvalidRequest)
}
#[tauri::command]
pub(crate) fn list_collaboration_rooms(
    storage: tauri::State<'_, Storage>,
) -> Result<Vec<Room>, ChatError> {
    storage.collaboration_rooms()
}
#[tauri::command]
pub(crate) fn create_collaboration_room(
    request: CreateRequest,
    storage: tauri::State<'_, Storage>,
) -> Result<Room, ChatError> {
    storage.create_collaboration_room(&request.title)
}
#[tauri::command]
pub(crate) fn prepare_collaboration(
    request: PrepareRequest,
    app: tauri::AppHandle,
    storage: tauri::State<'_, Storage>,
    chat: tauri::State<'_, AgentChatState>,
    state: tauri::State<'_, CollaborationState>,
) -> Result<Preview, ChatError> {
    let mut s = state.0.lock().map_err(|_| ChatError::Internal)?;
    if s.active.is_some() || s.stopping {
        return Err(ChatError::Busy);
    }
    let room = room(&storage, request.room_id)?;
    if room.runs.len() >= MAX_RUNS {
        return Err(ChatError::Limit);
    }
    storage
        .knowledge_check_sources(&request.input.sources)
        .map_err(|_| ChatError::StaleContext)?;
    let mut run = Run::new(
        format!("room-{}/run-{}", room.id, room.runs.len() + 1),
        request.input,
        &storage.agent_profiles()?,
    )?;
    let is_action = run.input.workflow == Workflow::CodingAction;
    if is_action {
        if !run.input.sources.is_empty() {
            return Err(ChatError::InvalidRequest);
        }
        let selection = request.action.ok_or(ChatError::InvalidRequest)?;
        let (serial, path) = s.selected.as_ref().ok_or(ChatError::InvalidRequest)?;
        if *serial != selection.selection {
            return Err(ChatError::StaleContext);
        }
        crate::isolated_action::sandbox::readiness()?;
        let base = app
            .path()
            .app_local_data_dir()
            .map_err(|_| ChatError::Internal)?
            .join("isolated-actions");
        if !base.exists() {
            use std::os::unix::fs::DirBuilderExt;
            std::fs::DirBuilder::new()
                .mode(0o700)
                .create(&base)
                .map_err(|_| ChatError::Internal)?;
        }
        use std::os::unix::fs::MetadataExt;
        let meta = std::fs::symlink_metadata(&base).map_err(|_| ChatError::Internal)?;
        if !meta.is_dir() || meta.mode() & 0o077 != 0 {
            return Err(crate::isolated_action::ActionError::Scope.into());
        }
        let root = base.join(format!(
            "cortexa-action-{}-{}-{}",
            std::process::id(),
            now(),
            s.preview_serial
        ));
        let workspace = Arc::new(crate::isolated_action::workspace::Workspace::prepare(
            path.clone(),
            selection.file,
            selection.test_file,
            root,
        )?);
        run.action = Some(workspace.evidence()?);
        s.action = Some((run.id.clone(), workspace));
    } else if request.action.is_some() {
        return Err(ChatError::InvalidRequest);
    }
    let simulation = preflight(&run, &storage.agent_profiles()?, |profile, prompt| {
        if is_action {
            action_adapter(&chat, profile, prompt)
        } else {
            collaboration_adapter(&chat, profile, prompt)
        }
    })?;
    s.preview_serial = s.preview_serial.checked_add(1).ok_or(ChatError::Limit)?;
    s.prepared = Some((room.id, run.clone()));
    Ok(Preview {
        preview_serial: s.preview_serial,
        room_id: room.id,
        maximum_calls: if is_action { 4 } else { run.stages.len() },
        run,
        simulation,
    })
}
fn preflight(
    run: &Run,
    current: &[crate::agent_preferences::AgentProfile],
    mut adapter: impl FnMut(
        crate::agent_preferences::AgentProfile,
        &str,
    ) -> Result<AdapterRequest, ChatError>,
) -> Result<bool, ChatError> {
    let simulation = run
        .stages
        .iter()
        .all(|s| s.participant.connection == AgentConnection::Simulation);
    if !simulation
        && run
            .stages
            .iter()
            .any(|s| s.participant.connection == AgentConnection::Simulation)
    {
        return Err(ChatError::InvalidRequest);
    }
    for stage in &run.stages {
        if current
            .iter()
            .find(|p| p.agent_id == stage.participant.agent_id)
            .is_none_or(|p| p.revision != stage.participant.revision)
        {
            return Err(ChatError::StaleContext);
        }
        // Construct and validate each adapter, but never execute a readiness call.
        drop(adapter(
            stage.participant.profile()?,
            "Readiness check; no call made.",
        )?);
    }
    Ok(simulation)
}
fn validate_approval(
    s: &Session,
    request: &StartRequest,
    id: i64,
    run: &Run,
) -> Result<(), ChatError> {
    if request.preview_serial != s.preview_serial
        || id != request.room_id
        || run.id != request.run_id
        || request.acknowledgment != "collaboration-shared-content-v1"
    {
        return Err(ChatError::InvalidRequest);
    }
    Ok(())
}
#[tauri::command]
pub(crate) fn start_collaboration(
    request: StartRequest,
    app: tauri::AppHandle,
    storage: tauri::State<'_, Storage>,
    chat: tauri::State<'_, AgentChatState>,
    state: tauri::State<'_, CollaborationState>,
) -> Result<Room, ChatError> {
    let mut s = state.0.lock().map_err(|_| ChatError::Internal)?;
    if s.active.is_some() || s.stopping {
        return Err(ChatError::Busy);
    }
    let (id, mut run) = s.prepared.clone().ok_or(ChatError::InvalidRequest)?;
    validate_approval(&s, &request, id, &run)?;
    storage
        .knowledge_check_sources(&run.input.sources)
        .map_err(|_| ChatError::StaleContext)?;
    let workspace = if run.input.workflow == Workflow::CodingAction {
        let (bound, w) = s.action.as_ref().ok_or(ChatError::StaleContext)?;
        if bound != &run.id
            || w.evidence()?.baseline
                != run.action.as_ref().ok_or(ChatError::StaleContext)?.baseline
        {
            return Err(ChatError::StaleContext);
        }
        w.recheck()?;
        Some(Arc::clone(w))
    } else {
        None
    };
    preflight(&run, &storage.agent_profiles()?, |profile, prompt| {
        if workspace.is_some() {
            action_adapter(&chat, profile, prompt)
        } else {
            collaboration_adapter(&chat, profile, prompt)
        }
    })?;
    let mut r = room(&storage, id)?;
    if r.runs.len() >= MAX_RUNS || run.id != format!("room-{}/run-{}", id, r.runs.len() + 1) {
        return Err(ChatError::InvalidRequest);
    }
    let lease = GenerationLease::acquire()?;
    run.status = Status::Running;
    r.runs.push(run.clone());
    storage.save_collaboration_room(&r)?;
    s.prepared = None;
    s.active = Some((id, run.id.clone()));
    let shared = Arc::clone(&state.0);
    s.task = Some(spawn_collaboration_task(async move {
        execute(app, shared, id, run, lease, workspace).await;
    }));
    Ok(r)
}
// Synchronous Tauri commands run without a thread-local Tokio context. Keep
// dispatch on the application runtime and retain the existing abort/join handle.
fn spawn_collaboration_task(
    task: impl std::future::Future<Output = ()> + Send + 'static,
) -> tokio::task::JoinHandle<()> {
    tauri::async_runtime::handle().inner().spawn(task)
}

fn update(
    app: &tauri::AppHandle,
    state: &Arc<Mutex<Session>>,
    id: i64,
    run: &Run,
) -> Result<(), ChatError> {
    let s = state.lock().map_err(|_| ChatError::Internal)?;
    if s.stopping || s.active.as_ref() != Some(&(id, run.id.clone())) {
        return Err(ChatError::InvalidRequest);
    }
    let storage = app.state::<Storage>();
    let mut r = room(&storage, id)?;
    let last = r.runs.last_mut().ok_or(ChatError::InvalidRequest)?;
    if last.id != run.id || last.sequence > run.sequence {
        return Err(ChatError::InvalidRequest);
    }
    *last = run.clone();
    storage.save_collaboration_room(&r).inspect_err(|_| {
        crate::diagnostics::event(
            crate::diagnostics::Event::StorageFailure,
            crate::diagnostics::Outcome::Failed,
            Some(crate::diagnostics::Category::Storage),
        );
    })
}
async fn execute(
    app: tauri::AppHandle,
    state: Arc<Mutex<Session>>,
    id: i64,
    mut run: Run,
    _lease: GenerationLease,
    workspace: Option<Arc<crate::isolated_action::workspace::Workspace>>,
) {
    let result = if let Some(w) = workspace.as_ref() {
        tokio::time::timeout(
            std::time::Duration::from_secs(310),
            crate::collaboration_action::execute(
                &mut run,
                w,
                |profile, prompt| action_adapter(&app.state::<AgentChatState>(), profile, prompt),
                |run| update(&app, &state, id, run),
            ),
        )
        .await
        .unwrap_or(Err(ChatError::Provider(DirectError::Timeout)))
    } else {
        execute_stages(
            &mut run,
            |profile, prompt| {
                collaboration_adapter(&app.state::<AgentChatState>(), profile, prompt)
            },
            |run| update(&app, &state, id, run),
        )
        .await
    };
    let cleanup = workspace.as_ref().map_or(Ok(()), |w| {
        crate::isolated_action::sandbox::verify_cleanup(&w.area)
    });
    let result = result.and(cleanup.map_err(ChatError::from));
    if let Err(error) = result {
        mark_failed(&mut run, error);
        let _ = update(&app, &state, id, &run);
    }
    if let Ok(mut s) = state.lock() {
        if s.active.as_ref() == Some(&(id, run.id.clone())) {
            if cleanup.is_ok() {
                s.active = None;
            } else {
                s.stopping = true;
            }
            s.task = None;
        }
    }
}
fn mark_failed(run: &mut Run, error: ChatError) {
    run.status = Status::Failed;
    run.error = serde_json::to_value(error)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned));
    for stage in &mut run.stages {
        if stage.status == Status::Running {
            stage.status = Status::Failed;
        } else if stage.status == Status::Queued {
            stage.status = Status::Cancelled;
        }
    }
    run.sequence += 1;
}
// An explicit non-secret debug-build opt-in; never accepted through IPC or profiles.
fn cancellation_fixture_opt_in() -> bool {
    #[cfg(debug_assertions)]
    {
        option_env!("CORTEXA_COLLABORATION_CANCEL_QA") == Some("1")
    }
    #[cfg(not(debug_assertions))]
    {
        false
    }
}
fn cancellation_fixture_selected(run: &Run, opted_in: bool) -> bool {
    cfg!(debug_assertions)
        && opted_in
        && run.input.workflow == Workflow::Proposal
        && run
            .stages
            .iter()
            .all(|s| s.participant.connection == AgentConnection::Simulation)
}
const CANCELLATION_FIXTURE_FRAGMENT: &str =
    "Debug Simulation cancellation QA: fixed synthetic provisional fragment; waiting for Stop. No provider call or validated handoff.";

// This exact executor is exercised offline with deterministic adapter events.
// The production caller supplies only the sealed existing provider adapters.
async fn execute_stages(
    run: &mut Run,
    adapter: impl FnMut(
        crate::agent_preferences::AgentProfile,
        &str,
    ) -> Result<AdapterRequest, ChatError>,
    persist: impl FnMut(&Run) -> Result<(), ChatError>,
) -> Result<(), ChatError> {
    let hold = cancellation_fixture_selected(run, cancellation_fixture_opt_in());
    execute_stages_with_fixture(run, adapter, persist, hold).await
}
async fn execute_stages_with_fixture(
    run: &mut Run,
    mut adapter: impl FnMut(
        crate::agent_preferences::AgentProfile,
        &str,
    ) -> Result<AdapterRequest, ChatError>,
    mut persist: impl FnMut(&Run) -> Result<(), ChatError>,
    hold: bool,
) -> Result<(), ChatError> {
    // Recheck the fixed route here so even internal callers cannot hold live stages.
    let hold = cancellation_fixture_selected(run, hold);
    tokio::time::timeout(std::time::Duration::from_secs(310), async {
        for i in 0..run.stages.len() {
            let prompt = run.prompt(i)?;
            run.stages[i].input = prompt.clone();
            run.stages[i].status = Status::Running;
            run.stages[i].timestamp = now();
            run.sequence += 1;
            persist(run)?;
            let profile = run.stages[i].participant.profile()?;
            let mut diagnostic = crate::diagnostics::Attempt::new(profile.connection, &profile.model, None, Some(&run.id), Some(&run.stages[i].id));
            let observer = diagnostic.observer();
            observer.record(crate::diagnostics::Event::ConfigurationValidated, crate::diagnostics::Outcome::Observed, None);
            let stage_result = async {
            let mut stream = StreamGuard::default();
            if hold && i == 0 {
                run.stages[i].provisional = CANCELLATION_FIXTURE_FRAGMENT.into();
                run.sequence += 1;
                persist(run)?;
                return tokio::time::timeout(std::time::Duration::from_secs(60),
                    std::future::pending::<Result<(), ChatError>>())
                    .await.map_err(|_| ChatError::Provider(DirectError::Timeout))?;
            }
            if profile.connection == AgentConnection::Simulation {
                stream.text = serde_json::json!({"version":1,"stage":i,"agentId":profile.agent_id,"status":"complete","summary":"Simulation only: fixture analysis, no provider call.","findings":["Proposed review of supplied material; no actions performed."],"evidence":run.input.sources.iter().map(|s|s.label.clone()).collect::<Vec<_>>(),"limitations":["Deterministic fixture; not live-provider evidence."]}).to_string();
                stream.completed = true;
                tokio::task::yield_now().await;
            } else {
                let credentialed = matches!(profile.connection, AgentConnection::OpenaiApi | AgentConnection::AnthropicApi);
                let request = adapter(profile, &prompt)?;
                if credentialed { observer.record(crate::diagnostics::Event::CredentialsAvailable, crate::diagnostics::Outcome::Observed, None); }
                tokio::time::timeout(std::time::Duration::from_secs(60), run_adapter_traced(request, observer.clone(), |event| {
                    if stream.accept(event)? {
                        run.stages[i].provisional = stream.text.clone();
                        run.sequence += 1;
                        persist(run).map_err(|_| DirectError::InvalidRequest)?;
                    }
                    Ok(())
                })).await.map_err(|_| ChatError::Provider(DirectError::Timeout))??;
            }
            if !stream.completed { return Err(ChatError::Provider(DirectError::Incomplete)); }
            run.accept(i, &stream.text)?;
            persist(run)?;
            Ok::<(), ChatError>(())
            }.await;
            if stage_result.is_ok() && run.status == Status::Partial { diagnostic.partial(); }
            else { diagnostic.finish(stage_result.map_err(|e| match e { ChatError::Provider(e) => e, _ => DirectError::Internal })); }
            stage_result?;
            if run.status == Status::Partial { break; }
        }
        Ok(())
    }).await.unwrap_or(Err(ChatError::Provider(DirectError::Timeout)))
}

async fn cancel(state: &CollaborationState, storage: &Storage, id: i64) -> Result<Room, ChatError> {
    let task = {
        let mut s = state.0.lock().map_err(|_| ChatError::Internal)?;
        if s.stopping {
            return Err(ChatError::Busy);
        }
        if s.active.as_ref().is_none_or(|(room, _)| *room != id) {
            return room(storage, id);
        }
        s.stopping = true;
        s.task.take()
    };
    if let Some(t) = task {
        t.abort();
        let _ = t.await;
    }
    let mut s = state.0.lock().map_err(|_| ChatError::Internal)?;
    if let Some((_, w)) = s
        .action
        .as_ref()
        .filter(|(run, _)| s.active.as_ref().is_some_and(|(_, active)| active == run))
    {
        crate::isolated_action::sandbox::verify_cleanup(&w.area)?;
    }
    // The child is joined and the lease dropped before releasing ingress, even
    // when history is unavailable. Never leave cancellation latched on an I/O error.
    s.active = None;
    s.stopping = false;
    let mut r = room(storage, id)?;
    if let Some(run) = r.runs.last_mut() {
        if run.status == Status::Running {
            run.status = Status::Cancelled;
            if let Some(e) = &mut run.action {
                e.disposition = "cancelled".into();
            }
            run.sequence += 1;
            for stage in &mut run.stages {
                if matches!(stage.status, Status::Running | Status::Queued) {
                    stage.status = Status::Cancelled;
                }
            }
        }
    }
    let result = storage.save_collaboration_room(&r);
    s.active = None;
    s.stopping = false;
    result?;
    Ok(r)
}
#[tauri::command]
pub(crate) async fn cancel_collaboration(
    request: RoomRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, CollaborationState>,
) -> Result<Room, ChatError> {
    cancel(&state, &storage, request.room_id).await
}
#[tauri::command]
pub(crate) async fn delete_collaboration_room(
    request: RoomRequest,
    storage: tauri::State<'_, Storage>,
    state: tauri::State<'_, CollaborationState>,
) -> Result<(), ChatError> {
    cancel(&state, &storage, request.room_id).await?;
    let mut s = state.0.lock().map_err(|_| ChatError::Internal)?;
    if s.active
        .as_ref()
        .is_some_and(|(id, _)| *id == request.room_id)
    {
        return Err(ChatError::Busy);
    }
    s.prepared = None;
    storage.delete_collaboration_room(request.room_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_adapter::run_adapter;
    #[test]
    fn command_dispatch_runs_without_an_ambient_tokio_runtime(
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert!(tokio::runtime::Handle::try_current().is_err());
        let (sender, receiver) = std::sync::mpsc::channel();
        // Exercise the production synchronous command dispatch boundary, not an
        // executor already wrapped in test-only Runtime::block_on.
        let task = spawn_collaboration_task(async move {
            assert!(tokio::runtime::Handle::try_current().is_ok());
            let _ = sender.send("dispatched");
        });
        assert!(tokio::runtime::Handle::try_current().is_err());
        assert_eq!(
            receiver.recv_timeout(std::time::Duration::from_secs(5))?,
            "dispatched"
        );
        tauri::async_runtime::block_on(task)?;
        Ok(())
    }

    #[test]
    fn command_dispatch_abort_joins_and_releases_ownership_without_ambient_runtime(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        assert!(tokio::runtime::Handle::try_current().is_err());
        let lease = GenerationLease::acquire()?;
        let dropped = Arc::new(std::sync::atomic::AtomicBool::new(false));
        struct ChildGuard(Arc<std::sync::atomic::AtomicBool>);
        impl Drop for ChildGuard {
            fn drop(&mut self) {
                self.0.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
        let child_drop = Arc::clone(&dropped);
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut session = Session {
            task: Some(spawn_collaboration_task(async move {
                let _lease = lease;
                let _guard = ChildGuard(child_drop);
                let _ = sender.send(());
                std::future::pending::<()>().await;
            })),
            prepared: None,
            preview_serial: 0,
            active: None,
            stopping: false,
            selected: None,
            action: None,
        };
        receiver.recv_timeout(std::time::Duration::from_secs(5))?;
        assert!(GenerationLease::acquire().is_err());
        let task = session.task.take().ok_or(ChatError::Internal)?;
        task.abort();
        let result = tauri::async_runtime::block_on(task);
        assert!(result.is_err_and(|error| error.is_cancelled()));
        assert!(dropped.load(std::sync::atomic::Ordering::SeqCst));
        drop(GenerationLease::acquire()?);
        assert!(tokio::runtime::Handle::try_current().is_err());
        Ok(())
    }

    fn simulation_run(workflow: Workflow) -> Result<Run, ChatError> {
        let mut run = live_run(workflow)?;
        for stage in &mut run.stages {
            stage.participant.connection = AgentConnection::Simulation;
            stage.participant.model = "simulation".into();
        }
        Ok(run)
    }
    #[test]
    fn cancellation_fixture_activation_is_debug_fixed_route_and_all_simulation_only(
    ) -> Result<(), ChatError> {
        let run = simulation_run(Workflow::Proposal)?;
        if !cfg!(debug_assertions) {
            assert!(!cancellation_fixture_opt_in());
        }
        assert!(!cancellation_fixture_selected(&run, false));
        assert_eq!(
            cancellation_fixture_selected(&run, true),
            cfg!(debug_assertions)
        );
        for workflow in [
            Workflow::Research,
            Workflow::Engineering,
            Workflow::Operations,
        ] {
            assert!(!cancellation_fixture_selected(
                &simulation_run(workflow)?,
                true
            ));
        }
        assert!(!cancellation_fixture_selected(
            &live_run(Workflow::Proposal)?,
            true
        ));
        let mut mixed = run;
        mixed.stages[1].participant.connection = AgentConnection::OpenaiApi;
        assert!(!cancellation_fixture_selected(&mixed, true));
        Ok(())
    }
    #[test]
    fn cancellation_fixture_disabled_preserves_ordinary_simulation_and_live_execution(
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                for workflow in [
                    Workflow::Research,
                    Workflow::Engineering,
                    Workflow::Operations,
                    Workflow::Proposal,
                ] {
                    let mut run = simulation_run(workflow)?;
                    let mut calls = 0;
                    execute_stages_with_fixture(
                        &mut run,
                        |_, _| {
                            calls += 1;
                            Err(ChatError::Internal)
                        },
                        |_| Ok(()),
                        false,
                    )
                    .await?;
                    assert_eq!(calls, 0);
                    assert_eq!(run.status, Status::Completed);
                    assert!(run
                        .stages
                        .iter()
                        .all(|s| s.handoff.is_some() && s.provisional.is_empty()));
                }
                let mut run = live_run(Workflow::Proposal)?;
                execute_stages_with_fixture(&mut run, events, |_| Ok(()), true).await?;
                assert_eq!(run.status, Status::Completed);
                Ok::<(), ChatError>(())
            })?;
        Ok(())
    }
    #[test]
    #[cfg(debug_assertions)]
    fn cancellation_fixture_abort_retains_fragment_and_joins_before_lease_release(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let storage = Arc::new(
            Storage::initialize(&crate::storage::DatabaseConfig::in_memory())?.into_storage(),
        );
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let mut initial_room =
                    storage.create_collaboration_room("Synthetic cancellation")?;
                let mut run = simulation_run(Workflow::Proposal)?;
                run.id = format!("room-{}/run-1", initial_room.id);
                initial_room.runs.push(run.clone());
                storage.save_collaboration_room(&initial_room)?;
                let state = CollaborationState::default();
                let lease = GenerationLease::acquire()?;
                let db = Arc::clone(&storage);
                let room_id = initial_room.id;
                let (tx, rx) = tokio::sync::oneshot::channel();
                let mut tx = Some(tx);
                {
                    let mut session = state.0.lock().map_err(|_| ChatError::Internal)?;
                    session.active = Some((room_id, run.id.clone()));
                    session.task = Some(spawn_collaboration_task(async move {
                        let _lease = lease;
                        let _ = execute_stages_with_fixture(
                            &mut run,
                            |_, _| Err(ChatError::Internal),
                            |snapshot| {
                                let mut room = room(&db, room_id)?;
                                room.runs[0] = snapshot.clone();
                                db.save_collaboration_room(&room)?;
                                if !snapshot.stages[0].provisional.is_empty() {
                                    if let Some(sender) = tx.take() {
                                        let _ = sender.send(());
                                    }
                                }
                                Ok(())
                            },
                            true,
                        )
                        .await;
                    }));
                }
                tokio::time::timeout(std::time::Duration::from_secs(5), rx).await??;
                assert!(GenerationLease::acquire().is_err());
                let before = room(&storage, room_id)?;
                assert_eq!(
                    before.runs[0].stages[0].provisional,
                    CANCELLATION_FIXTURE_FRAGMENT
                );
                assert_eq!(before.runs[0].stages[0].status, Status::Running);
                assert!(before.runs[0].stages[1..]
                    .iter()
                    .all(|s| s.status == Status::Queued));
                let cancelled = cancel(&state, &storage, room_id).await?;
                assert_eq!(cancelled.runs[0].status, Status::Cancelled);
                assert!(cancelled.runs[0]
                    .stages
                    .iter()
                    .all(|s| s.status == Status::Cancelled && s.handoff.is_none()));
                assert_eq!(
                    cancelled.runs[0].stages[0].provisional,
                    CANCELLATION_FIXTURE_FRAGMENT
                );
                {
                    let session = state.0.lock().map_err(|_| ChatError::Internal)?;
                    assert!(
                        session.active.is_none() && session.task.is_none() && !session.stopping
                    );
                }
                drop(GenerationLease::acquire()?);
                assert_eq!(
                    cancel(&state, &storage, room_id).await?.runs[0].status,
                    Status::Cancelled
                );
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }
    #[test]
    #[cfg(debug_assertions)]
    fn cancellation_fixture_safety_timeout_fails_without_advancing_and_releases_lease(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let mut run = simulation_run(Workflow::Proposal)?;
                let lease = GenerationLease::acquire()?;
                let task = spawn_collaboration_task(async move {
                    let _lease = lease;
                    let result = execute_stages_with_fixture(
                        &mut run,
                        |_, _| Err(ChatError::Internal),
                        |_| Ok(()),
                        true,
                    )
                    .await;
                    assert_eq!(result, Err(ChatError::Provider(DirectError::Timeout)));
                    if let Err(error) = result {
                        mark_failed(&mut run, error);
                    }
                    assert_eq!(run.status, Status::Failed);
                    assert_eq!(run.stages[0].status, Status::Failed);
                    assert_eq!(run.stages[0].provisional, CANCELLATION_FIXTURE_FRAGMENT);
                    assert!(run.stages[1..]
                        .iter()
                        .all(|s| s.status == Status::Cancelled && s.input.is_empty()));
                    assert!(run.stages.iter().all(|s| s.handoff.is_none()));
                });
                tokio::time::timeout(std::time::Duration::from_secs(65), task).await??;
                drop(GenerationLease::acquire()?);
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }

    fn live_run(workflow: Workflow) -> Result<Run, ChatError> {
        let profiles = crate::agent::definition::AgentId::ALL
            .into_iter()
            .map(|id| {
                let mut p = crate::agent_preferences::AgentProfile::defaults(id)?;
                p.connection = AgentConnection::OpenaiApi;
                p.model = "gpt-5.6-luna".into();
                p.note = "excluded-private-sentinel".into();
                p.memory_mode = crate::agent_preferences::MemoryMode::PrivateNotes;
                Ok(p)
            })
            .collect::<Result<Vec<_>, ChatError>>()?;
        let mut run = Run::new(
            "room-1/run-1".into(),
            Objective {
                workflow,
                objective: "Review synthetic material".into(),
                sources: vec![],
            },
            &profiles,
        )?;
        run.status = Status::Running;
        Ok(run)
    }
    fn events(
        profile: crate::agent_preferences::AgentProfile,
        prompt: &str,
    ) -> Result<AdapterRequest, ChatError> {
        use crate::personal_assistant_direct::ProviderEvent;
        let value: serde_json::Value =
            serde_json::from_str(prompt).map_err(|_| ChatError::Internal)?;
        assert!(profile.note.is_empty());
        let raw = serde_json::json!({"version":1,"stage":value["stage"],"agentId":profile.agent_id,
            "status":"complete","summary":"Deterministic proposal","findings":[],"evidence":[],"limitations":["No action performed"]}).to_string();
        let mid = raw.len() / 2;
        Ok(AdapterRequest::Events(vec![
            ProviderEvent::Started("unused".into()),
            ProviderEvent::Delta(raw[..mid].into()),
            ProviderEvent::Delta(raw[mid..].into()),
            ProviderEvent::Completed,
        ]))
    }
    #[test]
    fn actual_executor_covers_all_routes_streaming_persistence_and_failure_short_circuit(
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                for workflow in [
                    Workflow::Research,
                    Workflow::Engineering,
                    Workflow::Operations,
                    Workflow::Proposal,
                ] {
                    let mut run = live_run(workflow)?;
                    let mut calls = 0;
                    let mut snapshots = Vec::new();
                    execute_stages(
                        &mut run,
                        |p, prompt| {
                            calls += 1;
                            events(p, prompt)
                        },
                        |r| {
                            snapshots.push(r.clone());
                            Ok(())
                        },
                    )
                    .await?;
                    assert_eq!(calls, workflow.route().len());
                    assert_eq!(run.status, Status::Completed);
                    assert!(snapshots
                        .iter()
                        .any(|r| r.stages.iter().any(|s| !s.provisional.is_empty())));
                    assert!(snapshots
                        .windows(2)
                        .all(|pair| pair[0].sequence < pair[1].sequence));
                    for i in 1..run.stages.len() {
                        assert!(run.stages[i].input.contains("Deterministic proposal"));
                    }
                }
                for fail_stage in 0..5 {
                    let mut run = live_run(Workflow::Engineering)?;
                    let mut calls = 0;
                    let result = execute_stages(
                        &mut run,
                        |p, prompt| {
                            let index = calls;
                            calls += 1;
                            if index == fail_stage {
                                Ok(AdapterRequest::Failure(DirectError::ProviderUnavailable))
                            } else {
                                events(p, prompt)
                            }
                        },
                        |_| Ok(()),
                    )
                    .await;
                    assert!(result.is_err());
                    assert_eq!(calls, fail_stage + 1);
                    assert_ne!(run.status, Status::Completed);
                }
                let mut run = live_run(Workflow::Research)?;
                let mut calls = 0;
                assert!(execute_stages(
                    &mut run,
                    |p, prompt| {
                        calls += 1;
                        events(p, prompt)
                    },
                    |_| Err(ChatError::InvalidRequest)
                )
                .await
                .is_err());
                assert_eq!(
                    calls, 0,
                    "deleted or unavailable history stops before provider start"
                );
                Ok::<(), ChatError>(())
            })?;
        Ok(())
    }
    #[test]
    fn actual_stage_timeout_drops_child_and_never_advances(
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let mut run = live_run(Workflow::Research)?;
                let dropped = Arc::new(std::sync::atomic::AtomicBool::new(false));
                let mut calls = 0;
                let result = execute_stages(
                    &mut run,
                    |_, _| {
                        calls += 1;
                        Ok(AdapterRequest::Pending(Arc::clone(&dropped)))
                    },
                    |_| Ok(()),
                )
                .await;
                assert_eq!(result, Err(ChatError::Provider(DirectError::Timeout)));
                assert_eq!(calls, 1);
                assert!(dropped.load(std::sync::atomic::Ordering::SeqCst));
                assert!(run.stages[1..].iter().all(|s| s.status == Status::Queued));
                Ok::<(), ChatError>(())
            })?;
        Ok(())
    }
    #[test]
    fn preflight_rejects_unavailable_mixed_and_stale_profiles_without_execution(
    ) -> Result<(), ChatError> {
        let mut run = live_run(Workflow::Research)?;
        let mut profiles = run
            .stages
            .iter()
            .map(|s| s.participant.profile())
            .collect::<Result<Vec<_>, _>>()?;
        let mut checks = 0;
        assert!(preflight(&run, &profiles, |_, _| {
            checks += 1;
            Err(ChatError::Provider(DirectError::ProviderUnavailable))
        })
        .is_err());
        assert_eq!(checks, 1);
        profiles[0].revision += 1;
        assert_eq!(
            preflight(&run, &profiles, |_, _| {
                checks += 1;
                Ok(AdapterRequest::Simulation)
            }),
            Err(ChatError::StaleContext)
        );
        assert_eq!(checks, 1);
        run.stages[1].participant.connection = AgentConnection::Simulation;
        assert!(preflight(&run, &profiles, |_, _| {
            checks += 1;
            Ok(AdapterRequest::Simulation)
        })
        .is_err());
        assert_eq!(checks, 1);
        Ok(())
    }
    #[test]
    fn acknowledgement_is_bound_to_exact_native_preview() -> Result<(), ChatError> {
        let run = live_run(Workflow::Research)?;
        let s = Session {
            preview_serial: 2,
            prepared: None,
            active: None,
            task: None,
            stopping: false,
            selected: None,
            action: None,
        };
        let mut request = StartRequest {
            room_id: 1,
            run_id: run.id.clone(),
            preview_serial: 1,
            acknowledgment: "collaboration-shared-content-v1".into(),
        };
        assert!(validate_approval(&s, &request, 1, &run).is_err());
        request.preview_serial = 2;
        validate_approval(&s, &request, 1, &run)?;
        request.acknowledgment.clear();
        assert!(validate_approval(&s, &request, 1, &run).is_err());
        Ok(())
    }
    #[test]
    fn cancellation_waits_for_child_future_and_releases_shared_generation_lease(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let storage =
            Storage::initialize(&crate::storage::DatabaseConfig::in_memory())?.into_storage();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let mut room = storage.create_collaboration_room("Synthetic")?;
                let input = Objective {
                    workflow: Workflow::Research,
                    objective: "Synthetic".into(),
                    sources: vec![],
                };
                let mut run = Run::new(
                    format!("room-{}/run-1", room.id),
                    input,
                    &storage.agent_profiles()?,
                )?;
                run.status = Status::Running;
                run.stages[0].status = Status::Running;
                room.runs.push(run.clone());
                storage.save_collaboration_room(&room)?;
                let lease = GenerationLease::acquire()?;
                let state = CollaborationState::default();
                let dropped = Arc::new(std::sync::atomic::AtomicBool::new(false));
                let d = Arc::clone(&dropped);
                {
                    let mut s = state.0.lock().map_err(|_| ChatError::Internal)?;
                    s.active = Some((room.id, run.id));
                    s.task = Some(tokio::spawn(async move {
                        let _lease = lease;
                        let _ = run_adapter(AdapterRequest::Pending(d), |_| Ok(())).await;
                    }));
                }
                tokio::task::yield_now().await;
                assert!(GenerationLease::acquire().is_err());
                let cancelled = cancel(&state, &storage, room.id).await?;
                assert_eq!(cancelled.runs[0].status, Status::Cancelled);
                assert!(dropped.load(std::sync::atomic::Ordering::SeqCst));
                drop(GenerationLease::acquire()?);
                assert_eq!(
                    cancel(&state, &storage, room.id).await?.runs[0].status,
                    Status::Cancelled
                );
                Ok::<(), ChatError>(())
            })?;
        Ok(())
    }
    #[test]
    fn native_request_does_not_accept_arbitrary_recipients_secrets_or_context() {
        for raw in [
            r#"{"roomId":1,"input":{"workflow":"arbitrary","objective":"x","sources":[]}}"#,
            r#"{"roomId":1,"input":{"workflow":"research","objective":"x","sources":[]},"privateNote":"x"}"#,
        ] {
            assert!(serde_json::from_str::<PrepareRequest>(raw).is_err());
        }
        assert!(serde_json::from_str::<StartRequest>(
            r#"{"roomId":1,"runId":"r","acknowledgment":"x","apiKey":"x"}"#
        )
        .is_err());
    }

    #[test]
    fn action_codex_preflight_rejects_stale_profiles_and_missing_catalog_without_dispatch(
    ) -> Result<(), ChatError> {
        let mut run = live_run(Workflow::CodingAction)?;
        for stage in &mut run.stages {
            stage.participant.connection = AgentConnection::Codex;
        }
        let mut profiles = run
            .stages
            .iter()
            .map(|s| s.participant.profile())
            .collect::<Result<Vec<_>, _>>()?;
        let state = AgentChatState::default();
        let mut calls = 0;
        assert_eq!(
            preflight(&run, &profiles, |p, prompt| {
                calls += 1;
                action_adapter(&state, p, prompt)
            }),
            Err(ChatError::Provider(DirectError::Catalog))
        );
        assert_eq!(calls, 1);
        profiles[0].revision += 1;
        assert_eq!(
            preflight(&run, &profiles, |p, prompt| {
                calls += 1;
                action_adapter(&state, p, prompt)
            }),
            Err(ChatError::StaleContext)
        );
        assert_eq!(calls, 1);
        assert!(run.stages.iter().all(|s| s.handoff.is_none()));
        Ok(())
    }
}

#[cfg(test)]
mod approval_parent_request_tests {
    use super::ReviewRequest;
    #[test]
    fn review_request_cannot_supply_parent_or_native_decision() {
        let request =
            serde_json::json!({"roomId": 1, "runId": "room-1/run-1", "reviewHash": "a".repeat(64)});
        assert!(serde_json::from_value::<ReviewRequest>(request.clone()).is_ok());
        for key in ["window", "parent", "windowId", "approved", "decision"] {
            let mut changed = request.clone();
            changed[key] = serde_json::json!("synthetic");
            assert!(serde_json::from_value::<ReviewRequest>(changed).is_err());
        }
    }
}
