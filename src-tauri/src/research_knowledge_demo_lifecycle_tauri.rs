//! Narrow Tauri boundary for the volatile Research/Knowledge demo lifecycle.
//!
//! Commands accept no WebView-provided values. The application owns the sole
//! synthetic host, and emitted snapshots are notification-only copies of
//! committed command results.

use std::sync::Mutex;

use tauri::Emitter;

use crate::{
    ResearchKnowledgeDemoHost, ResearchKnowledgeDemoLifecycleError,
    ResearchKnowledgeDemoLifecycleSnapshot,
};

pub(crate) const RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT: &str =
    "research-knowledge-demo-lifecycle-v1";

pub(crate) struct ResearchKnowledgeDemoLifecycleTauriState {
    host: Mutex<ResearchKnowledgeDemoHost>,
}

impl ResearchKnowledgeDemoLifecycleTauriState {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            host: Mutex::new(ResearchKnowledgeDemoHost::new()),
        }
    }
}

impl Default for ResearchKnowledgeDemoLifecycleTauriState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
enum LifecycleOperation {
    Start,
    Advance,
    Cancel,
}

fn current_snapshot(
    state: &ResearchKnowledgeDemoLifecycleTauriState,
) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
    let host = state
        .host
        .lock()
        .map_err(|_| ResearchKnowledgeDemoLifecycleError::Unavailable)?;
    Ok(host.snapshot())
}

fn mutate_and_notify(
    state: &ResearchKnowledgeDemoLifecycleTauriState,
    operation: LifecycleOperation,
    emit: impl FnOnce(&ResearchKnowledgeDemoLifecycleSnapshot) -> Result<(), ()>,
) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
    let snapshot = {
        let mut host = state
            .host
            .lock()
            .map_err(|_| ResearchKnowledgeDemoLifecycleError::Unavailable)?;
        match operation {
            LifecycleOperation::Start => host.start()?,
            LifecycleOperation::Advance => host.advance()?.snapshot().clone(),
            LifecycleOperation::Cancel => host.cancel()?.snapshot().clone(),
        }
    };

    emit(&snapshot).map_err(|()| ResearchKnowledgeDemoLifecycleError::Unavailable)?;
    Ok(snapshot)
}

#[tauri::command]
pub(crate) fn get_research_knowledge_demo_lifecycle_snapshot(
    state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,
) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
    current_snapshot(&state)
}

#[tauri::command]
pub(crate) fn start_research_knowledge_demo_lifecycle(
    app: tauri::AppHandle,
    state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,
) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
    mutate_and_notify(&state, LifecycleOperation::Start, |snapshot| {
        app.emit(RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT, snapshot)
            .map_err(|_| ())
    })
}

#[tauri::command]
pub(crate) fn advance_research_knowledge_demo_lifecycle(
    app: tauri::AppHandle,
    state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,
) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
    mutate_and_notify(&state, LifecycleOperation::Advance, |snapshot| {
        app.emit(RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT, snapshot)
            .map_err(|_| ())
    })
}

#[tauri::command]
pub(crate) fn cancel_research_knowledge_demo_lifecycle(
    app: tauri::AppHandle,
    state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,
) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
    mutate_and_notify(&state, LifecycleOperation::Cancel, |snapshot| {
        app.emit(RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT, snapshot)
            .map_err(|_| ())
    })
}

#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use super::{
        current_snapshot, mutate_and_notify, LifecycleOperation,
        ResearchKnowledgeDemoLifecycleTauriState,
    };
    use crate::{
        ResearchKnowledgeDemoHost, ResearchKnowledgeDemoLifecycleError,
        ResearchKnowledgeDemoLifecycleState,
    };

    #[test]
    fn managed_state_and_host_satisfy_tauri_threading_bounds() {
        fn assert_send_sync_static<T: Send + Sync + 'static>() {}
        fn assert_send_static<T: Send + 'static>() {}

        assert_send_sync_static::<ResearchKnowledgeDemoLifecycleTauriState>();
        assert_send_static::<ResearchKnowledgeDemoHost>();
    }

    #[test]
    fn no_input_operations_complete_and_emit_each_committed_snapshot(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let state = ResearchKnowledgeDemoLifecycleTauriState::new();
        let mut emitted = Vec::new();

        for operation in [
            LifecycleOperation::Start,
            LifecycleOperation::Advance,
            LifecycleOperation::Advance,
            LifecycleOperation::Advance,
        ] {
            let snapshot = mutate_and_notify(&state, operation, |snapshot| {
                emitted.push(snapshot.clone());
                Ok(())
            })?;
            assert_eq!(emitted.last(), Some(&snapshot));
        }

        assert_eq!(emitted.len(), 4);
        assert_eq!(
            emitted.last().map(|snapshot| snapshot.state()),
            Some(ResearchKnowledgeDemoLifecycleState::Succeeded)
        );
        assert_eq!(emitted.last().map(|snapshot| snapshot.revision()), Some(6));
        Ok(())
    }

    #[test]
    fn no_input_completed_epochs_alternate_success_and_synthetic_failure(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let state = ResearchKnowledgeDemoLifecycleTauriState::new();

        let complete_epoch = || {
            mutate_and_notify(&state, LifecycleOperation::Start, |_| Ok(()))?;
            mutate_and_notify(&state, LifecycleOperation::Advance, |_| Ok(()))?;
            mutate_and_notify(&state, LifecycleOperation::Advance, |_| Ok(()))?;
            mutate_and_notify(&state, LifecycleOperation::Advance, |_| Ok(()))
        };

        assert_eq!(
            complete_epoch()?.state(),
            ResearchKnowledgeDemoLifecycleState::Succeeded
        );
        assert_eq!(
            complete_epoch()?.state(),
            ResearchKnowledgeDemoLifecycleState::Failed
        );
        assert_eq!(
            complete_epoch()?.state(),
            ResearchKnowledgeDemoLifecycleState::Succeeded
        );
        Ok(())
    }

    #[test]
    fn cancellation_is_committed_and_notified_without_replacing_the_host(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let state = ResearchKnowledgeDemoLifecycleTauriState::new();
        mutate_and_notify(&state, LifecycleOperation::Start, |_| Ok(()))?;

        let cancelled = mutate_and_notify(&state, LifecycleOperation::Cancel, |_| Ok(()))?;
        assert_eq!(
            cancelled.state(),
            ResearchKnowledgeDemoLifecycleState::Cancelled
        );

        let repeated = mutate_and_notify(&state, LifecycleOperation::Cancel, |_| Ok(()))?;
        assert_eq!(repeated, cancelled);
        Ok(())
    }

    #[test]
    fn notification_failure_returns_closed_error_after_preserving_mutation(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let state = ResearchKnowledgeDemoLifecycleTauriState::new();

        assert_eq!(
            mutate_and_notify(&state, LifecycleOperation::Start, |_| Err(())),
            Err(ResearchKnowledgeDemoLifecycleError::Unavailable)
        );

        let recovered = current_snapshot(&state)?;
        assert_eq!(
            recovered.state(),
            ResearchKnowledgeDemoLifecycleState::Research
        );
        assert_eq!(recovered.revision(), 1);
        Ok(())
    }

    #[test]
    fn poisoned_state_returns_only_the_closed_unavailable_error() {
        let state = ResearchKnowledgeDemoLifecycleTauriState::new();
        let poison = catch_unwind(AssertUnwindSafe(|| {
            let _guard = match state.host.lock() {
                Ok(host) => host,
                Err(poisoned) => poisoned.into_inner(),
            };
            std::panic::resume_unwind(Box::new(()));
        }));
        assert!(poison.is_err());

        assert_eq!(
            current_snapshot(&state),
            Err(ResearchKnowledgeDemoLifecycleError::Unavailable)
        );
        assert_eq!(
            mutate_and_notify(&state, LifecycleOperation::Start, |_| Ok(())),
            Err(ResearchKnowledgeDemoLifecycleError::Unavailable)
        );
    }
}
