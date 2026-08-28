//! Volatile owner for the sealed Personal Assistant v0 synthetic turn.
//!
//! This host performs no I/O and exposes no response-frame ingress. It owns one
//! process-local Native run, validates the exact returned identity and initial
//! status, and retains ambiguous cleanup ownership fail closed.

use std::fmt;
use std::mem;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

use thiserror::Error;

use crate::agent::native_runtime::{NativeAgentRun, NativeAgentRuntime};
use crate::agent::runtime::{
    AgentRuntime, RuntimeCancellationOutcome, RuntimeError, RuntimeInvalidRequest, RuntimeRun,
    RuntimeRunIdentity, RuntimeRunStatus, RuntimeTurnRequest,
};

static PROCESS_LEASE_HELD: AtomicBool = AtomicBool::new(false);
static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);
static PROCESS_QUARANTINE: Mutex<Option<OwnedNativeRun>> = Mutex::new(None);

pub struct PersonalAssistantV0Host {
    state: HostState,
}

enum HostState {
    Idle,
    Active(OwnedNativeRun),
    TerminalSummary(PersonalAssistantV0Status),
    Quarantined(OwnedNativeRun),
}

struct OwnedRun<R> {
    run: R,
    expected_identity: RuntimeRunIdentity,
    _lease: ProcessLease,
}

type OwnedNativeRun = OwnedRun<NativeAgentRun>;

struct ProcessLease;

impl ProcessLease {
    fn acquire() -> Result<Self, PersonalAssistantV0Error> {
        if try_acquire_lease(&PROCESS_LEASE_HELD) {
            Ok(Self)
        } else {
            Err(PersonalAssistantV0Error::Busy)
        }
    }
}

impl Drop for ProcessLease {
    fn drop(&mut self) {
        PROCESS_LEASE_HELD.store(false, Ordering::Release);
    }
}

impl Default for PersonalAssistantV0Host {
    fn default() -> Self {
        Self::new()
    }
}

impl PersonalAssistantV0Host {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: HostState::Idle,
        }
    }

    pub fn start_synthetic(&mut self) -> Result<(), PersonalAssistantV0Error> {
        let prior_terminal = self.prepare_for_start()?;

        let lease = match ProcessLease::acquire() {
            Ok(lease) => lease,
            Err(error) => {
                self.restore_terminal_summary(prior_terminal);
                return Err(error);
            }
        };
        let generation = match reserve_generation(&NEXT_GENERATION) {
            Ok(generation) => generation,
            Err(error) => {
                self.restore_terminal_summary(prior_terminal);
                return Err(error);
            }
        };
        let run_id = format!("pa-v0-run-{generation:016x}");
        let request_id = format!("pa-v0-request-{generation:016x}");
        let request = match RuntimeTurnRequest::personal_assistant_v0_synthetic(run_id, request_id)
        {
            Ok(request) => request,
            Err(error) => {
                self.restore_terminal_summary(prior_terminal);
                return Err(map_start_error(error));
            }
        };
        let expected_identity = request.identity();

        match start_checked(&NativeAgentRuntime, request, &expected_identity) {
            Ok(ReturnedRunCheck::Accepted(run)) => {
                self.state = HostState::Active(OwnedRun {
                    run,
                    expected_identity,
                    _lease: lease,
                });
                Ok(())
            }
            Ok(ReturnedRunCheck::RejectedClean(error)) => {
                self.restore_terminal_summary(prior_terminal);
                Err(error)
            }
            Ok(ReturnedRunCheck::RejectedAmbiguous { run, error }) => {
                self.state = HostState::Quarantined(OwnedRun {
                    run,
                    expected_identity,
                    _lease: lease,
                });
                Err(error)
            }
            Err(_) => {
                self.restore_terminal_summary(prior_terminal);
                Err(PersonalAssistantV0Error::Internal)
            }
        }
    }

    #[must_use]
    pub fn status(&self) -> PersonalAssistantV0Status {
        match &self.state {
            HostState::Idle => PersonalAssistantV0Status::Idle,
            HostState::TerminalSummary(status) => *status,
            HostState::Quarantined(_) => PersonalAssistantV0Status::Failed,
            HostState::Active(owner) => {
                if owner.run.identity() != &owner.expected_identity {
                    PersonalAssistantV0Status::Failed
                } else {
                    map_status(owner.run.status())
                }
            }
        }
    }

    pub fn request_byte_len(&self) -> Result<usize, PersonalAssistantV0Error> {
        self.request_bytes().map(<[u8]>::len)
    }

    pub(crate) fn request_bytes(&self) -> Result<&[u8], PersonalAssistantV0Error> {
        match &self.state {
            HostState::Active(owner) => {
                if owner.run.identity() != &owner.expected_identity {
                    return Err(PersonalAssistantV0Error::ReturnedIdentityMismatch);
                }
                if owner.run.status().is_terminal() {
                    return Err(PersonalAssistantV0Error::NoActiveRun);
                }
                Ok(owner.run.request_bytes())
            }
            HostState::Quarantined(_) => Err(PersonalAssistantV0Error::Busy),
            HostState::Idle | HostState::TerminalSummary(_) => {
                Err(PersonalAssistantV0Error::NoActiveRun)
            }
        }
    }

    pub fn cancel(&mut self) -> Result<PersonalAssistantV0Cancellation, PersonalAssistantV0Error> {
        let state = mem::replace(&mut self.state, HostState::Idle);
        match state {
            HostState::Idle => Err(PersonalAssistantV0Error::NoActiveRun),
            HostState::TerminalSummary(status) => {
                self.state = HostState::TerminalSummary(status);
                Ok(PersonalAssistantV0Cancellation::AlreadyTerminal(status))
            }
            HostState::Active(owner) => self.cancel_active_owned(owner),
            HostState::Quarantined(owner) => self.cancel_quarantined_owned(owner),
        }
    }

    fn prepare_for_start(
        &mut self,
    ) -> Result<Option<PersonalAssistantV0Status>, PersonalAssistantV0Error> {
        let state = mem::replace(&mut self.state, HostState::Idle);
        match state {
            HostState::Idle => Ok(None),
            HostState::TerminalSummary(status) => Ok(Some(status)),
            HostState::Quarantined(owner) => {
                self.state = HostState::Quarantined(owner);
                Err(PersonalAssistantV0Error::Busy)
            }
            HostState::Active(mut owner) => {
                if owner.run.identity() != &owner.expected_identity {
                    let cleanup = owner.run.cancel();
                    if cleanup_is_proved(&cleanup) {
                        Err(PersonalAssistantV0Error::ReturnedIdentityMismatch)
                    } else {
                        self.state = HostState::Quarantined(owner);
                        Err(PersonalAssistantV0Error::ReturnedIdentityMismatch)
                    }
                } else if !owner.run.status().is_terminal() {
                    self.state = HostState::Active(owner);
                    Err(PersonalAssistantV0Error::Busy)
                } else {
                    let terminal_status = map_status(owner.run.status());
                    let cleanup = owner.run.cancel();
                    if cleanup_is_proved(&cleanup) {
                        Ok(Some(terminal_status))
                    } else {
                        self.state = HostState::Quarantined(owner);
                        Err(PersonalAssistantV0Error::Busy)
                    }
                }
            }
        }
    }

    fn restore_terminal_summary(&mut self, status: Option<PersonalAssistantV0Status>) {
        if let Some(status) = status {
            self.state = HostState::TerminalSummary(status);
        }
    }

    fn cancel_active_owned(
        &mut self,
        mut owner: OwnedNativeRun,
    ) -> Result<PersonalAssistantV0Cancellation, PersonalAssistantV0Error> {
        let identity_mismatch = owner.run.identity() != &owner.expected_identity;
        let outcome = owner.run.cancel();
        match outcome {
            Ok(RuntimeCancellationOutcome::Cancelled) => {
                if identity_mismatch {
                    self.state = HostState::TerminalSummary(PersonalAssistantV0Status::Failed);
                    Err(PersonalAssistantV0Error::ReturnedIdentityMismatch)
                } else {
                    self.state = HostState::TerminalSummary(PersonalAssistantV0Status::Cancelled);
                    Ok(PersonalAssistantV0Cancellation::Cancelled)
                }
            }
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) if status.is_terminal() => {
                if identity_mismatch {
                    self.state = HostState::TerminalSummary(PersonalAssistantV0Status::Failed);
                    Err(PersonalAssistantV0Error::ReturnedIdentityMismatch)
                } else {
                    let status = map_status(status);
                    self.state = HostState::TerminalSummary(status);
                    Ok(PersonalAssistantV0Cancellation::AlreadyTerminal(status))
                }
            }
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(_)) => {
                self.state = HostState::Quarantined(owner);
                Err(PersonalAssistantV0Error::ProtocolViolation)
            }
            Err(_) => {
                self.state = HostState::Quarantined(owner);
                Err(PersonalAssistantV0Error::Internal)
            }
        }
    }

    fn cancel_quarantined_owned(
        &mut self,
        owner: OwnedNativeRun,
    ) -> Result<PersonalAssistantV0Cancellation, PersonalAssistantV0Error> {
        match retry_quarantined_cleanup(owner) {
            QuarantinedCleanup::Proved => {
                self.state = HostState::TerminalSummary(PersonalAssistantV0Status::Failed);
                Ok(PersonalAssistantV0Cancellation::AlreadyTerminal(
                    PersonalAssistantV0Status::Failed,
                ))
            }
            QuarantinedCleanup::Ambiguous { owner, error } => {
                self.state = HostState::Quarantined(owner);
                Err(error)
            }
        }
    }
}

impl fmt::Debug for PersonalAssistantV0Host {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0Host")
            .field("status", &self.status())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

impl Drop for PersonalAssistantV0Host {
    fn drop(&mut self) {
        let state = mem::replace(&mut self.state, HostState::Idle);
        let owner = match state {
            HostState::Active(owner) | HostState::Quarantined(owner) => owner,
            HostState::Idle | HostState::TerminalSummary(_) => return,
        };
        cleanup_or_quarantine_on_drop(owner, &PROCESS_QUARANTINE);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersonalAssistantV0Status {
    Idle,
    Starting,
    Streaming,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersonalAssistantV0Cancellation {
    Cancelled,
    AlreadyTerminal(PersonalAssistantV0Status),
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum PersonalAssistantV0Error {
    #[error("Personal Assistant v0 is busy")]
    Busy,
    #[error("Personal Assistant v0 has no active run")]
    NoActiveRun,
    #[error("Personal Assistant v0 identity space is exhausted")]
    IdentityExhausted,
    #[error("Personal Assistant v0 returned an unexpected runtime identity")]
    ReturnedIdentityMismatch,
    #[error("Personal Assistant v0 runtime request was rejected")]
    RequestRejected,
    #[error("Personal Assistant v0 protocol validation failed")]
    ProtocolViolation,
    #[error("Personal Assistant v0 closed limit was exceeded")]
    LimitExceeded,
    #[error("Personal Assistant v0 internal boundary failed")]
    Internal,
}

enum ReturnedRunCheck<R> {
    Accepted(R),
    RejectedClean(PersonalAssistantV0Error),
    RejectedAmbiguous {
        run: R,
        error: PersonalAssistantV0Error,
    },
}

enum QuarantinedCleanup<R> {
    Proved,
    Ambiguous {
        owner: OwnedRun<R>,
        error: PersonalAssistantV0Error,
    },
}

fn start_checked<R: AgentRuntime>(
    runtime: &R,
    request: RuntimeTurnRequest,
    expected_identity: &RuntimeRunIdentity,
) -> Result<ReturnedRunCheck<R::Run>, RuntimeError> {
    let run = runtime.start(request)?;
    Ok(check_returned_run(run, expected_identity))
}

fn check_returned_run<R: RuntimeRun>(
    mut run: R,
    expected_identity: &RuntimeRunIdentity,
) -> ReturnedRunCheck<R> {
    let rejection = if run.identity() != expected_identity {
        Some(PersonalAssistantV0Error::ReturnedIdentityMismatch)
    } else if run.status() != RuntimeRunStatus::AwaitingStart {
        Some(PersonalAssistantV0Error::RequestRejected)
    } else {
        None
    };

    let Some(error) = rejection else {
        return ReturnedRunCheck::Accepted(run);
    };
    let cleanup = run.cancel();
    if cleanup_is_proved(&cleanup) {
        ReturnedRunCheck::RejectedClean(error)
    } else {
        ReturnedRunCheck::RejectedAmbiguous { run, error }
    }
}

fn cleanup_is_proved(outcome: &Result<RuntimeCancellationOutcome, RuntimeError>) -> bool {
    match outcome {
        Ok(RuntimeCancellationOutcome::Cancelled) => true,
        Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) => status.is_terminal(),
        Err(_) => false,
    }
}

fn retry_quarantined_cleanup<R: RuntimeRun>(mut owner: OwnedRun<R>) -> QuarantinedCleanup<R> {
    let cleanup = owner.run.cancel();
    if cleanup_is_proved(&cleanup) {
        QuarantinedCleanup::Proved
    } else {
        let error = match cleanup {
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(_)) => {
                PersonalAssistantV0Error::ProtocolViolation
            }
            Ok(RuntimeCancellationOutcome::Cancelled) => PersonalAssistantV0Error::Internal,
            Err(_) => PersonalAssistantV0Error::Internal,
        };
        QuarantinedCleanup::Ambiguous { owner, error }
    }
}

fn cleanup_or_quarantine_on_drop<R: RuntimeRun>(
    mut owner: OwnedRun<R>,
    slot: &Mutex<Option<OwnedRun<R>>>,
) {
    let cleanup = owner.run.cancel();
    if cleanup_is_proved(&cleanup) {
        return;
    }
    quarantine_or_leak(owner, slot);
}

fn quarantine_or_leak<T>(value: T, slot: &Mutex<Option<T>>) {
    match slot.lock() {
        Ok(mut guard) if guard.is_none() => {
            *guard = Some(value);
        }
        Ok(_) | Err(_) => mem::forget(value),
    }
}

fn try_acquire_lease(flag: &AtomicBool) -> bool {
    flag.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
}

fn reserve_generation(counter: &AtomicU64) -> Result<u64, PersonalAssistantV0Error> {
    counter
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            if current == 0 {
                None
            } else {
                current.checked_add(1)
            }
        })
        .map_err(|_| PersonalAssistantV0Error::IdentityExhausted)
}

fn map_start_error(error: RuntimeError) -> PersonalAssistantV0Error {
    match error {
        RuntimeError::InvalidRequest(RuntimeInvalidRequest::RunId)
        | RuntimeError::InvalidRequest(RuntimeInvalidRequest::RequestId) => {
            PersonalAssistantV0Error::IdentityExhausted
        }
        RuntimeError::InvalidRequest(RuntimeInvalidRequest::SelectedContentTooLarge { .. })
        | RuntimeError::InvalidRequest(RuntimeInvalidRequest::SerializedRequestTooLarge {
            ..
        }) => PersonalAssistantV0Error::LimitExceeded,
        RuntimeError::InvalidRequest(_) => PersonalAssistantV0Error::RequestRejected,
        RuntimeError::InvalidEvent(_)
        | RuntimeError::EventRejected(_)
        | RuntimeError::CapabilityUnavailable(_)
        | RuntimeError::Unavailable
        | RuntimeError::BoundaryFailure(_) => PersonalAssistantV0Error::Internal,
    }
}

fn map_status(status: RuntimeRunStatus) -> PersonalAssistantV0Status {
    match status {
        RuntimeRunStatus::AwaitingStart => PersonalAssistantV0Status::Starting,
        RuntimeRunStatus::Streaming => PersonalAssistantV0Status::Streaming,
        RuntimeRunStatus::Completed => PersonalAssistantV0Status::Completed,
        RuntimeRunStatus::Failed => PersonalAssistantV0Status::Failed,
        RuntimeRunStatus::Cancelled => PersonalAssistantV0Status::Cancelled,
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use serde_json::Value;

    use super::{
        check_returned_run, cleanup_or_quarantine_on_drop, map_status, quarantine_or_leak,
        reserve_generation, retry_quarantined_cleanup, start_checked, try_acquire_lease, HostState,
        OwnedRun, PersonalAssistantV0Cancellation, PersonalAssistantV0Error,
        PersonalAssistantV0Host, PersonalAssistantV0Status, ProcessLease, QuarantinedCleanup,
        ReturnedRunCheck, NEXT_GENERATION, PROCESS_LEASE_HELD,
    };
    use crate::agent::runtime::{
        AgentRuntime, RuntimeAvailability, RuntimeBoundaryStage, RuntimeCancellationOutcome,
        RuntimeCapabilities, RuntimeDescriptor, RuntimeError, RuntimeEventAcceptance,
        RuntimeEventEnvelope, RuntimeHealth, RuntimeId, RuntimeOutputText, RuntimeResponseId,
        RuntimeResult, RuntimeRun, RuntimeRunId, RuntimeRunIdentity, RuntimeRunStatus,
        RuntimeTurnRequest, UntrustedRuntimeEvent,
    };

    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    struct FakeRuntime {
        run: RefCell<Option<FakeRun>>,
        start_error: Option<RuntimeError>,
        starts: Cell<usize>,
    }

    impl FakeRuntime {
        fn with_run(run: FakeRun) -> Self {
            Self {
                run: RefCell::new(Some(run)),
                start_error: None,
                starts: Cell::new(0),
            }
        }

        fn with_error(error: RuntimeError) -> Self {
            Self {
                run: RefCell::new(None),
                start_error: Some(error),
                starts: Cell::new(0),
            }
        }
    }

    impl AgentRuntime for FakeRuntime {
        type Run = FakeRun;

        fn describe(&self) -> RuntimeDescriptor {
            RuntimeDescriptor::new(
                RuntimeId::Native,
                RuntimeAvailability::Available,
                RuntimeHealth::Healthy,
                RuntimeCapabilities::new(true, false),
            )
        }

        fn start(&self, _request: RuntimeTurnRequest) -> RuntimeResult<Self::Run> {
            self.starts.set(self.starts.get() + 1);
            if let Some(error) = self.start_error {
                return Err(error);
            }
            self.run
                .borrow_mut()
                .take()
                .ok_or(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))
        }
    }

    struct FakeRun {
        identity: RuntimeRunIdentity,
        status: RuntimeRunStatus,
        cancellations: VecDeque<RuntimeResult<RuntimeCancellationOutcome>>,
        cancel_calls: Rc<Cell<usize>>,
    }

    impl FakeRun {
        fn new(
            identity: RuntimeRunIdentity,
            status: RuntimeRunStatus,
            cancellations: impl IntoIterator<Item = RuntimeResult<RuntimeCancellationOutcome>>,
            cancel_calls: Rc<Cell<usize>>,
        ) -> Self {
            Self {
                identity,
                status,
                cancellations: cancellations.into_iter().collect(),
                cancel_calls,
            }
        }
    }

    impl RuntimeRun for FakeRun {
        fn run_id(&self) -> &RuntimeRunId {
            self.identity.run_id()
        }

        fn identity(&self) -> &RuntimeRunIdentity {
            &self.identity
        }

        fn status(&self) -> RuntimeRunStatus {
            self.status
        }

        fn accept_event(
            &mut self,
            _event: RuntimeEventEnvelope,
        ) -> RuntimeResult<RuntimeEventAcceptance> {
            Err(RuntimeError::BoundaryFailure(
                RuntimeBoundaryStage::EventAcceptance,
            ))
        }

        fn cancel(&mut self) -> RuntimeResult<RuntimeCancellationOutcome> {
            self.cancel_calls.set(self.cancel_calls.get() + 1);
            self.cancellations
                .pop_front()
                .unwrap_or(Ok(RuntimeCancellationOutcome::AlreadyTerminal(self.status)))
        }
    }

    fn request(generation: u64) -> RuntimeResult<RuntimeTurnRequest> {
        RuntimeTurnRequest::personal_assistant_v0_synthetic(
            format!("pa-v0-test-run-{generation}"),
            format!("pa-v0-test-request-{generation}"),
        )
    }

    #[test]
    fn public_host_is_volatile_no_input_busy_across_instances_and_idempotently_cancelled(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));

        let mut first = PersonalAssistantV0Host::new();
        let mut second = PersonalAssistantV0Host::default();
        assert_eq!(first.status(), PersonalAssistantV0Status::Idle);
        assert_eq!(
            first.request_byte_len(),
            Err(PersonalAssistantV0Error::NoActiveRun)
        );
        first.start_synthetic()?;
        assert_eq!(first.status(), PersonalAssistantV0Status::Starting);
        assert!(first.request_byte_len()? > 0);
        let body: Value = serde_json::from_slice(first.request_bytes()?)?;
        let run_id = body["run_id"].as_str().ok_or("missing run id")?;
        let request_id = body["gateway_request_id"]
            .as_str()
            .ok_or("missing request id")?;
        assert!(run_id.starts_with("pa-v0-run-"));
        assert!(request_id.starts_with("pa-v0-request-"));
        assert_eq!(body["request_kind"], "personal_assistant_text_v0");
        assert_eq!(body["tool_set"]["tools"], serde_json::json!([]));
        assert_eq!(
            body["input"]["text"],
            "Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed."
        );
        assert_eq!(
            second.start_synthetic(),
            Err(PersonalAssistantV0Error::Busy)
        );

        let debug = format!("{first:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(run_id));
        assert!(!debug.contains(request_id));
        assert!(!debug.contains("Prepare a concise three-bullet board update"));

        assert_eq!(first.cancel()?, PersonalAssistantV0Cancellation::Cancelled);
        assert_eq!(first.status(), PersonalAssistantV0Status::Cancelled);
        assert_eq!(
            first.cancel()?,
            PersonalAssistantV0Cancellation::AlreadyTerminal(PersonalAssistantV0Status::Cancelled)
        );
        second.start_synthetic()?;
        assert_eq!(second.status(), PersonalAssistantV0Status::Starting);
        assert_eq!(second.cancel()?, PersonalAssistantV0Cancellation::Cancelled);
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        Ok(())
    }

    #[test]
    fn public_host_drop_proves_native_cleanup_and_releases_the_process_lease(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        {
            let mut host = PersonalAssistantV0Host::new();
            host.start_synthetic()?;
            assert!(PROCESS_LEASE_HELD.load(Ordering::Acquire));
        }
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        let mut replacement = PersonalAssistantV0Host::new();
        replacement.start_synthetic()?;
        replacement.cancel()?;
        Ok(())
    }

    #[test]
    fn failed_restart_retains_the_prior_terminal_summary_until_acceptance(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        let mut host = PersonalAssistantV0Host::new();
        host.start_synthetic()?;
        host.cancel()?;
        assert_eq!(host.status(), PersonalAssistantV0Status::Cancelled);

        let competing_lease = ProcessLease::acquire()?;
        assert_eq!(host.start_synthetic(), Err(PersonalAssistantV0Error::Busy));
        assert_eq!(host.status(), PersonalAssistantV0Status::Cancelled);
        assert_eq!(
            host.cancel()?,
            PersonalAssistantV0Cancellation::AlreadyTerminal(PersonalAssistantV0Status::Cancelled)
        );
        drop(competing_lease);

        host.start_synthetic()?;
        assert_eq!(host.status(), PersonalAssistantV0Status::Starting);
        host.cancel()?;
        Ok(())
    }

    #[test]
    fn quarantined_host_stays_failed_busy_and_never_promotes_rejected_runtime_outcome(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        let request = request(40)?;
        let expected_identity = request.identity();
        let run = crate::agent::native_runtime::NativeAgentRuntime.start(request)?;
        let lease = ProcessLease::acquire()?;
        let mut host = PersonalAssistantV0Host {
            state: HostState::Quarantined(OwnedRun {
                run,
                expected_identity,
                _lease: lease,
            }),
        };

        assert_eq!(host.status(), PersonalAssistantV0Status::Failed);
        assert_eq!(host.request_byte_len(), Err(PersonalAssistantV0Error::Busy));
        assert_eq!(host.start_synthetic(), Err(PersonalAssistantV0Error::Busy));
        assert!(PROCESS_LEASE_HELD.load(Ordering::Acquire));
        assert_eq!(
            host.cancel()?,
            PersonalAssistantV0Cancellation::AlreadyTerminal(PersonalAssistantV0Status::Failed)
        );
        assert_eq!(host.status(), PersonalAssistantV0Status::Failed);
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        assert_eq!(
            host.cancel()?,
            PersonalAssistantV0Cancellation::AlreadyTerminal(PersonalAssistantV0Status::Failed)
        );

        host.start_synthetic()?;
        assert_eq!(host.status(), PersonalAssistantV0Status::Starting);
        host.cancel()?;
        Ok(())
    }

    #[test]
    fn quarantine_cleanup_discards_terminal_status_from_a_rejected_run(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        let identity = request(41)?.identity();
        let calls = Rc::new(Cell::new(0));
        let owner = OwnedRun {
            run: FakeRun::new(
                identity.clone(),
                RuntimeRunStatus::AwaitingStart,
                [Ok(RuntimeCancellationOutcome::AlreadyTerminal(
                    RuntimeRunStatus::Completed,
                ))],
                Rc::clone(&calls),
            ),
            expected_identity: identity,
            _lease: ProcessLease::acquire()?,
        };

        assert!(matches!(
            retry_quarantined_cleanup(owner),
            QuarantinedCleanup::Proved
        ));
        assert_eq!(calls.get(), 1);
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        Ok(())
    }

    #[test]
    fn active_identity_mismatch_never_promotes_cancelled_or_completed_runtime_status(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));

        let awaiting_request = request(50)?;
        let awaiting_run =
            crate::agent::native_runtime::NativeAgentRuntime.start(awaiting_request)?;
        let mut awaiting_host = PersonalAssistantV0Host {
            state: HostState::Active(OwnedRun {
                run: awaiting_run,
                expected_identity: request(51)?.identity(),
                _lease: ProcessLease::acquire()?,
            }),
        };
        assert_eq!(awaiting_host.status(), PersonalAssistantV0Status::Failed);
        assert_eq!(
            awaiting_host.cancel(),
            Err(PersonalAssistantV0Error::ReturnedIdentityMismatch)
        );
        assert_eq!(awaiting_host.status(), PersonalAssistantV0Status::Failed);
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));

        let completed_request = request(52)?;
        let actual_identity = completed_request.identity();
        let mut completed_run =
            crate::agent::native_runtime::NativeAgentRuntime.start(completed_request)?;
        completed_run.accept_event(RuntimeEventEnvelope::for_identity(
            &actual_identity,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("pa-v0-test-response-52")?,
            },
        ))?;
        completed_run.accept_event(RuntimeEventEnvelope::for_identity(
            &actual_identity,
            1,
            UntrustedRuntimeEvent::OutputTextDelta {
                delta: RuntimeOutputText::new("Synthetic completion")?,
            },
        ))?;
        completed_run.accept_event(RuntimeEventEnvelope::for_identity(
            &actual_identity,
            2,
            UntrustedRuntimeEvent::ResponseCompleted,
        ))?;
        assert_eq!(completed_run.status(), RuntimeRunStatus::Completed);

        let mut completed_host = PersonalAssistantV0Host {
            state: HostState::Active(OwnedRun {
                run: completed_run,
                expected_identity: request(53)?.identity(),
                _lease: ProcessLease::acquire()?,
            }),
        };
        assert_eq!(completed_host.status(), PersonalAssistantV0Status::Failed);
        assert_eq!(
            completed_host.cancel(),
            Err(PersonalAssistantV0Error::ReturnedIdentityMismatch)
        );
        assert_eq!(completed_host.status(), PersonalAssistantV0Status::Failed);
        assert_eq!(
            completed_host.cancel()?,
            PersonalAssistantV0Cancellation::AlreadyTerminal(PersonalAssistantV0Status::Failed)
        );
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        Ok(())
    }

    #[test]
    fn checked_start_accepts_only_exact_identity_and_awaiting_start_once(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let accepted_request = request(1)?;
        let expected = accepted_request.identity();
        let calls = Rc::new(Cell::new(0));
        let runtime = FakeRuntime::with_run(FakeRun::new(
            expected.clone(),
            RuntimeRunStatus::AwaitingStart,
            [],
            Rc::clone(&calls),
        ));
        assert!(matches!(
            start_checked(&runtime, accepted_request, &expected)?,
            ReturnedRunCheck::Accepted(_)
        ));
        assert_eq!(runtime.starts.get(), 1);
        assert_eq!(calls.get(), 0);

        let error_runtime = FakeRuntime::with_error(RuntimeError::Unavailable);
        assert_eq!(
            start_checked(&error_runtime, request(2)?, &request(2)?.identity()).err(),
            Some(RuntimeError::Unavailable)
        );
        assert_eq!(error_runtime.starts.get(), 1);
        Ok(())
    }

    #[test]
    fn checked_start_rejects_wrong_run_wrong_request_and_every_wrong_initial_status(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for case in 0..2 {
            let start_request = request(10 + case)?;
            let expected = start_request.identity();
            let identity = if case == 0 {
                request(20)?.identity()
            } else {
                RuntimeTurnRequest::personal_assistant_v0_synthetic(
                    expected.run_id().as_str().to_owned(),
                    "pa-v0-test-request-wrong".to_owned(),
                )?
                .identity()
            };
            let calls = Rc::new(Cell::new(0));
            let run = FakeRun::new(
                identity,
                RuntimeRunStatus::AwaitingStart,
                [Ok(RuntimeCancellationOutcome::Cancelled)],
                Rc::clone(&calls),
            );
            let runtime = FakeRuntime::with_run(run);
            assert!(matches!(
                start_checked(&runtime, start_request, &expected)?,
                ReturnedRunCheck::RejectedClean(PersonalAssistantV0Error::ReturnedIdentityMismatch)
            ));
            assert_eq!(runtime.starts.get(), 1);
            assert_eq!(calls.get(), 1);
        }

        for (case, status) in [
            RuntimeRunStatus::Streaming,
            RuntimeRunStatus::Completed,
            RuntimeRunStatus::Failed,
            RuntimeRunStatus::Cancelled,
        ]
        .into_iter()
        .enumerate()
        {
            let start_request = request(30 + u64::try_from(case)?)?;
            let expected = start_request.identity();
            let calls = Rc::new(Cell::new(0));
            let cleanup = if status.is_terminal() {
                RuntimeCancellationOutcome::AlreadyTerminal(status)
            } else {
                RuntimeCancellationOutcome::Cancelled
            };
            let run = FakeRun::new(expected.clone(), status, [Ok(cleanup)], Rc::clone(&calls));
            let runtime = FakeRuntime::with_run(run);
            assert!(matches!(
                start_checked(&runtime, start_request, &expected)?,
                ReturnedRunCheck::RejectedClean(PersonalAssistantV0Error::RequestRejected)
            ));
            assert_eq!(runtime.starts.get(), 1);
            assert_eq!(calls.get(), 1);
        }
        Ok(())
    }

    #[test]
    fn rejected_run_cleanup_proofs_release_while_ambiguous_results_retain_ownership(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expected = request(20)?.identity();
        for outcome in [
            RuntimeCancellationOutcome::Cancelled,
            RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Completed),
            RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Failed),
            RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Cancelled),
        ] {
            let calls = Rc::new(Cell::new(0));
            let run = FakeRun::new(
                request(21)?.identity(),
                RuntimeRunStatus::AwaitingStart,
                [Ok(outcome)],
                Rc::clone(&calls),
            );
            assert!(matches!(
                check_returned_run(run, &expected),
                ReturnedRunCheck::RejectedClean(PersonalAssistantV0Error::ReturnedIdentityMismatch)
            ));
            assert_eq!(calls.get(), 1);
        }

        let ambiguous_cases = [
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(
                RuntimeRunStatus::AwaitingStart,
            )),
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(
                RuntimeRunStatus::Streaming,
            )),
            Err(RuntimeError::BoundaryFailure(
                RuntimeBoundaryStage::Cancellation,
            )),
        ];
        for outcome in ambiguous_cases {
            let calls = Rc::new(Cell::new(0));
            let run = FakeRun::new(
                request(22)?.identity(),
                RuntimeRunStatus::AwaitingStart,
                [outcome, Ok(RuntimeCancellationOutcome::Cancelled)],
                Rc::clone(&calls),
            );
            let ReturnedRunCheck::RejectedAmbiguous { mut run, error } =
                check_returned_run(run, &expected)
            else {
                return Err("ambiguous cleanup must retain the run".into());
            };
            assert_eq!(error, PersonalAssistantV0Error::ReturnedIdentityMismatch);
            assert_eq!(calls.get(), 1);
            assert_eq!(run.cancel()?, RuntimeCancellationOutcome::Cancelled);
            assert_eq!(calls.get(), 2);
        }
        Ok(())
    }

    #[test]
    fn generation_and_lease_helpers_fail_closed_on_busy_zero_and_wrap() {
        let lease = AtomicBool::new(false);
        assert!(try_acquire_lease(&lease));
        assert!(!try_acquire_lease(&lease));

        let generation = AtomicU64::new(1);
        assert_eq!(reserve_generation(&generation), Ok(1));
        assert_eq!(reserve_generation(&generation), Ok(2));
        let exhausted = AtomicU64::new(u64::MAX);
        assert_eq!(
            reserve_generation(&exhausted),
            Err(PersonalAssistantV0Error::IdentityExhausted)
        );
        let zero = AtomicU64::new(0);
        assert_eq!(
            reserve_generation(&zero),
            Err(PersonalAssistantV0Error::IdentityExhausted)
        );
    }

    #[test]
    fn drop_cleanup_transfers_ambiguous_owner_and_holds_lease_until_owner_drops(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        let lease = ProcessLease::acquire()?;
        let calls = Rc::new(Cell::new(0));
        let owner = OwnedRun {
            run: FakeRun::new(
                request(30)?.identity(),
                RuntimeRunStatus::AwaitingStart,
                [Err(RuntimeError::BoundaryFailure(
                    RuntimeBoundaryStage::Cancellation,
                ))],
                Rc::clone(&calls),
            ),
            expected_identity: request(30)?.identity(),
            _lease: lease,
        };
        let slot = Mutex::new(None);
        cleanup_or_quarantine_on_drop(owner, &slot);
        assert_eq!(calls.get(), 1);
        assert!(PROCESS_LEASE_HELD.load(Ordering::Acquire));
        let retained = slot
            .lock()
            .map_err(|_| "quarantine lock poisoned")?
            .take()
            .ok_or("expected retained owner")?;
        drop(retained);
        assert!(!PROCESS_LEASE_HELD.load(Ordering::Acquire));
        Ok(())
    }

    struct DropProbe(Arc<AtomicUsize>);

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::AcqRel);
        }
    }

    #[test]
    fn occupied_or_poisoned_quarantine_never_releases_replacement_ownership() {
        let occupied_drops = Arc::new(AtomicUsize::new(0));
        let occupied = Mutex::new(Some(DropProbe(Arc::clone(&occupied_drops))));
        quarantine_or_leak(DropProbe(Arc::clone(&occupied_drops)), &occupied);
        assert_eq!(occupied_drops.load(Ordering::Acquire), 0);
        drop(occupied);
        assert_eq!(occupied_drops.load(Ordering::Acquire), 1);

        let poisoned_drops = Arc::new(AtomicUsize::new(0));
        let poisoned = Mutex::new(None);
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let _guard = match poisoned.lock() {
                Ok(guard) => guard,
                Err(error) => error.into_inner(),
            };
            std::panic::resume_unwind(Box::new(()));
        }));
        assert!(poisoned.is_poisoned());
        quarantine_or_leak(DropProbe(Arc::clone(&poisoned_drops)), &poisoned);
        assert_eq!(poisoned_drops.load(Ordering::Acquire), 0);
    }

    #[test]
    fn host_states_and_errors_project_only_closed_redacted_values() {
        assert_eq!(
            map_status(RuntimeRunStatus::AwaitingStart),
            PersonalAssistantV0Status::Starting
        );
        assert_eq!(
            map_status(RuntimeRunStatus::Streaming),
            PersonalAssistantV0Status::Streaming
        );
        assert_eq!(
            map_status(RuntimeRunStatus::Completed),
            PersonalAssistantV0Status::Completed
        );
        assert_eq!(
            map_status(RuntimeRunStatus::Failed),
            PersonalAssistantV0Status::Failed
        );
        assert_eq!(
            map_status(RuntimeRunStatus::Cancelled),
            PersonalAssistantV0Status::Cancelled
        );

        let host = PersonalAssistantV0Host {
            state: HostState::TerminalSummary(PersonalAssistantV0Status::Completed),
        };
        assert_eq!(host.status(), PersonalAssistantV0Status::Completed);
        for error in [
            PersonalAssistantV0Error::Busy,
            PersonalAssistantV0Error::NoActiveRun,
            PersonalAssistantV0Error::IdentityExhausted,
            PersonalAssistantV0Error::ReturnedIdentityMismatch,
            PersonalAssistantV0Error::RequestRejected,
            PersonalAssistantV0Error::ProtocolViolation,
            PersonalAssistantV0Error::LimitExceeded,
            PersonalAssistantV0Error::Internal,
        ] {
            let text = format!("{error:?} {error}");
            assert!(!text.contains("pa-v0-test-run"));
            assert!(!text.contains("Prepare a concise"));
        }
        assert!(NEXT_GENERATION.load(Ordering::Acquire) >= 1);
    }
}
