//! Application-owned direct demo session. Locks protect transitions, never network awaits.
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

use crate::personal_assistant_direct::{self as provider, ApiKey, DirectError, GenerationLease};
use crate::personal_assistant_v0::{
    PersonalAssistantV0FailureCode, PersonalAssistantV0Host, PersonalAssistantV0PresentationHandle,
    PersonalAssistantV0Snapshot,
};

#[derive(Default)]
pub(crate) struct DirectState(Arc<Mutex<Session>>);
#[derive(Default)]
struct Session {
    host: PersonalAssistantV0Host,
    handle: Option<PersonalAssistantV0PresentationHandle>,
    task: Option<JoinHandle<()>>,
    stopping: bool,
    error: Option<DirectError>,
}
impl Drop for Session {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct StartRequest {
    version: u8,
    acknowledgment: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PollRequest {
    handle: Option<String>,
    cursor: Option<u64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CancelRequest {
    handle: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Snapshot {
    version: u8,
    handle: Option<String>,
    status: &'static str,
    text: String,
    sequence: u64,
    error: Option<DirectError>,
    busy: bool,
}

impl Session {
    fn matching(&self, handle: &str) -> Result<PersonalAssistantV0PresentationHandle, DirectError> {
        self.handle
            .as_ref()
            .filter(|known| known.as_str() == handle)
            .cloned()
            .ok_or(DirectError::InvalidRequest)
    }
    fn snapshot(&mut self, cursor: Option<u64>) -> Result<Snapshot, DirectError> {
        let Some(handle) = self.handle.clone() else {
            return Ok(Snapshot {
                version: 1,
                handle: None,
                status: "idle",
                text: String::new(),
                sequence: 0,
                error: None,
                busy: false,
            });
        };
        // Reuse bounded host update/cursor validation; this UI needs only its latest snapshot.
        let batch = self
            .host
            .updates(&handle, cursor)
            .map_err(|_| DirectError::InvalidRequest)?;
        let snapshot = batch.snapshot().clone();
        if snapshot.is_terminal() {
            if let Some(task) = &self.task {
                if !task.is_finished() {
                    task.abort();
                }
            }
        } else if self.task.as_ref().is_some_and(JoinHandle::is_finished) {
            self.error = Some(DirectError::Internal);
            self.host
                .fail_direct(&handle, PersonalAssistantV0FailureCode::Internal)
                .map_err(|_| DirectError::Internal)?;
            return self.snapshot(cursor);
        }
        let status = match &snapshot {
            PersonalAssistantV0Snapshot::Starting => "starting",
            PersonalAssistantV0Snapshot::Streaming(_) => "streaming",
            PersonalAssistantV0Snapshot::Completed(_) => "completed",
            PersonalAssistantV0Snapshot::Cancelled(_)
            | PersonalAssistantV0Snapshot::Cancelling(_) => "stopped",
            PersonalAssistantV0Snapshot::Failed(failed) => {
                if self.error.is_none() {
                    self.error = Some(match failed.failure().code() {
                        PersonalAssistantV0FailureCode::DeadlineExceeded
                        | PersonalAssistantV0FailureCode::ProviderTimeout => DirectError::Timeout,
                        _ => DirectError::Protocol,
                    });
                }
                "error"
            }
        };
        Ok(Snapshot {
            version: 1,
            handle: Some(handle.as_str().to_owned()),
            status,
            text: snapshot.accepted_text().to_owned(),
            sequence: snapshot.sequence(),
            error: self.error,
            busy: self.stopping
                || !snapshot.is_terminal()
                || self.task.as_ref().is_some_and(|task| !task.is_finished()),
        })
    }
}

fn start_owned(
    state: &Arc<Mutex<Session>>,
    spawn: impl FnOnce(
        Arc<Mutex<Session>>,
        PersonalAssistantV0PresentationHandle,
        GenerationLease,
    ) -> JoinHandle<()>,
) -> Result<Snapshot, DirectError> {
    let mut session = state.lock().map_err(|_| DirectError::Internal)?;
    if session.stopping
        || session
            .task
            .as_ref()
            .is_some_and(|task| !task.is_finished())
    {
        return Err(DirectError::Busy);
    }
    let lease = GenerationLease::acquire()?;
    let start = session.host.start_direct().map_err(|_| DirectError::Busy)?;
    let handle = start.presentation_handle().clone();
    session.error = None;
    session.handle = Some(handle.clone());
    session.task = Some(spawn(Arc::clone(state), handle, lease));
    session.snapshot(None)
}

async fn execute(
    state: Arc<Mutex<Session>>,
    handle: PersonalAssistantV0PresentationHandle,
    key: ApiKey,
    _lease: GenerationLease,
) {
    let result = bounded_request(
        std::time::Duration::from_secs(60),
        provider::run(key, |event| {
            let mut session = state.lock().map_err(|_| DirectError::Internal)?;
            session.matching(handle.as_str())?;
            let snapshot = session
                .host
                .accept_direct(&handle, event)
                .map_err(|_| DirectError::Protocol)?;
            if matches!(
                snapshot,
                PersonalAssistantV0Snapshot::Failed(_) | PersonalAssistantV0Snapshot::Cancelled(_)
            ) {
                return Err(DirectError::Protocol);
            }
            Ok(())
        }),
    )
    .await;
    if let Err(error) = result {
        if let Ok(mut session) = state.lock() {
            if session.matching(handle.as_str()).is_ok() {
                // A cancelled/terminal run must never be overwritten by a late error.
                if session
                    .host
                    .snapshot(&handle)
                    .is_ok_and(|snapshot| !snapshot.is_terminal())
                {
                    session.error = Some(error);
                    let _ = session
                        .host
                        .fail_direct(&handle, PersonalAssistantV0FailureCode::ProviderUnavailable);
                }
            }
        }
    }
}

async fn bounded_request(
    duration: std::time::Duration,
    request: impl std::future::Future<Output = Result<(), DirectError>>,
) -> Result<(), DirectError> {
    tokio::time::timeout(duration, request)
        .await
        .unwrap_or(Err(DirectError::Timeout))
}

async fn stop_owned(state: &Arc<Mutex<Session>>, handle: &str) -> Result<Snapshot, DirectError> {
    let task = {
        let mut session = state.lock().map_err(|_| DirectError::Internal)?;
        let known = session.matching(handle)?;
        session
            .host
            .cancel(&known)
            .map_err(|_| DirectError::Internal)?; // Close ingress first.
        if session.stopping {
            return session.snapshot(None);
        }
        session.stopping = true;
        session.task.take()
    };
    if let Some(task) = task {
        task.abort();
        let _ = task.await;
    }
    let mut session = state.lock().map_err(|_| DirectError::Internal)?;
    session.stopping = false;
    session.snapshot(None)
}

#[tauri::command]
pub(crate) async fn start_personal_assistant_direct(
    request: StartRequest,
    state: tauri::State<'_, DirectState>,
) -> Result<Snapshot, DirectError> {
    if request.version != 1 || request.acknowledgment != "openai-synthetic-direct-v1" {
        return Err(DirectError::InvalidRequest);
    }
    let key = ApiKey::from_environment()?;
    start_owned(&state.0, |state, handle, lease| {
        tokio::spawn(execute(state, handle, key, lease))
    })
}
#[tauri::command]
pub(crate) fn poll_personal_assistant_direct(
    request: PollRequest,
    state: tauri::State<'_, DirectState>,
) -> Result<Snapshot, DirectError> {
    let mut session = state.0.lock().map_err(|_| DirectError::Internal)?;
    if let Some(handle) = request.handle {
        session.matching(&handle)?;
    } else if request.cursor.is_some() {
        return Err(DirectError::InvalidRequest);
    }
    session.snapshot(request.cursor)
}
#[tauri::command]
pub(crate) async fn cancel_personal_assistant_direct(
    request: CancelRequest,
    state: tauri::State<'_, DirectState>,
) -> Result<Snapshot, DirectError> {
    stop_owned(&state.0, &request.handle).await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timeout_and_stop_drop_the_owned_future_without_network(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        use std::sync::atomic::{AtomicBool, Ordering};
        struct Dropped(Arc<AtomicBool>);
        impl Drop for Dropped {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let dropped = Arc::new(AtomicBool::new(false));
                let guard = Dropped(Arc::clone(&dropped));
                assert_eq!(
                    bounded_request(std::time::Duration::ZERO, async move {
                        let _guard = guard;
                        std::future::pending().await
                    })
                    .await,
                    Err(DirectError::Timeout)
                );
                assert!(dropped.load(Ordering::SeqCst));
                dropped.store(false, Ordering::SeqCst);
                let guard = Dropped(Arc::clone(&dropped));
                let state = Arc::new(Mutex::new(Session::default()));
                let first = start_owned(&state, |_, _, lease| {
                    tokio::spawn(async move {
                        let _lease = lease;
                        let _guard = guard;
                        std::future::pending::<()>().await;
                    })
                })?;
                tokio::task::yield_now().await;
                stop_owned(
                    &state,
                    first.handle.as_deref().ok_or("missing fixture handle")?,
                )
                .await?;
                assert!(dropped.load(Ordering::SeqCst));
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }
    #[test]
    fn actual_provider_identity_and_text_flow_through_existing_host(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        let mut host = PersonalAssistantV0Host::default();
        let start = host.start_direct()?;
        let handle = start.presentation_handle();
        host.accept_direct(
            handle,
            provider::ProviderEvent::Started("resp_fixture_from_stream".into()),
        )?;
        host.accept_direct(
            handle,
            provider::ProviderEvent::Delta("Three synthetic bullets.".into()),
        )?;
        let completed = host.accept_direct(handle, provider::ProviderEvent::Completed)?;
        assert!(matches!(
            completed,
            PersonalAssistantV0Snapshot::Completed(_)
        ));
        assert_eq!(completed.accepted_text(), "Three synthetic bullets.");
        assert_eq!(host.updates(handle, None)?.snapshot(), &completed);
        assert!(host
            .accept_direct(handle, provider::ProviderEvent::Delta("late".into()))
            .is_err());
        let next = host.start_direct()?;
        assert_ne!(next.presentation_handle(), handle);
        assert_eq!(
            host.snapshot(next.presentation_handle())?.accepted_text(),
            ""
        );
        host.cancel(next.presentation_handle())?;
        Ok(())
    }
    #[test]
    fn cancellation_duplicates_stale_output_and_safe_restart(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Shares the host's process-wide lease with host unit tests.
        let _serial = crate::personal_assistant_v0::tests::serial_guard()?;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let state = Arc::new(Mutex::new(Session::default()));
                let spawn = |_: Arc<Mutex<Session>>,
                             _: PersonalAssistantV0PresentationHandle,
                             lease: GenerationLease| {
                    tokio::spawn(async move {
                        let _lease = lease;
                        std::future::pending().await
                    })
                };
                let first = start_owned(&state, spawn)?;
                assert!(matches!(start_owned(&state, spawn), Err(DirectError::Busy)));
                let old = state
                    .lock()
                    .map_err(|_| "poisoned test session")?
                    .handle
                    .clone()
                    .ok_or("missing fixture handle")?;
                let stopped = stop_owned(
                    &state,
                    first.handle.as_deref().ok_or("missing fixture handle")?,
                )
                .await?;
                assert_eq!(stopped.status, "stopped");
                assert!(!stopped.busy);
                assert_eq!(stop_owned(&state, old.as_str()).await?.status, "stopped");
                let next = start_owned(&state, spawn)?;
                assert_ne!(first.handle, next.handle);
                assert_eq!(next.text, "");
                assert!(state
                    .lock()
                    .map_err(|_| "poisoned test session")?
                    .host
                    .accept_direct(&old, provider::ProviderEvent::Delta("late".into()))
                    .is_err());
                stop_owned(
                    &state,
                    next.handle.as_deref().ok_or("missing fixture handle")?,
                )
                .await?;
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        Ok(())
    }
    #[test]
    fn request_payloads_reject_extra_fields() {
        assert!(serde_json::from_str::<StartRequest>(
            r#"{"version":1,"acknowledgment":"openai-synthetic-direct-v1","prompt":"secret"}"#
        )
        .is_err());
        assert!(serde_json::from_str::<CancelRequest>(
            r#"{"handle":"x","url":"https://example.com"}"#
        )
        .is_err());
    }
}
