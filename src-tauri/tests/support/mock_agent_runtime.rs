use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    fmt,
    rc::Rc,
};

use ai_agent_assistant_lib::agent::runtime::{
    AgentRuntime, RuntimeAvailability, RuntimeBoundaryStage, RuntimeCancellationOutcome,
    RuntimeCapabilities, RuntimeCapability, RuntimeDescriptor, RuntimeError,
    RuntimeEventAcceptance, RuntimeEventEnvelope, RuntimeEventRejection, RuntimeHealth, RuntimeId,
    RuntimeRun, RuntimeRunId, RuntimeRunIdentity, RuntimeRunStatus, RuntimeTurnRequest,
    UntrustedRuntimeEvent,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)] // Each integration-test crate exercises a different closed mode subset.
pub enum MockMode {
    Success,
    Unavailable,
    Unhealthy,
    StartFailure,
    StartFailureAt(u8),
    StartFailuresAt(u8, u8),
    StartFailuresAtThree(u8, u8, u8),
    UnexpectedStartStatusAt(u8, RuntimeRunStatus),
    ReturnedIdentityMismatchAt(u8),
    ReturnedIdentityMismatchWithCancelFailureOnceAt(u8),
    DuplicateLiveIdentityAt(u8),
    CancelFailureAt(u8),
    CancelFailureOnceAt(u8),
    CancelAlreadyTerminalAt(u8, RuntimeRunStatus),
    EventFailure,
    CapabilityContradiction,
}

pub struct MockAgentRuntime {
    mode: MockMode,
    starts: Rc<RefCell<Vec<MockRuntimeStart>>>,
    cancellations: Rc<RefCell<Vec<String>>>,
    lifecycle: Rc<RefCell<MockRuntimeLifecycle>>,
}

impl MockAgentRuntime {
    #[allow(dead_code)] // Not every integration-test crate uses the direct constructor.
    pub fn new(mode: MockMode) -> Self {
        Self {
            mode,
            starts: Rc::new(RefCell::new(Vec::new())),
            cancellations: Rc::new(RefCell::new(Vec::new())),
            lifecycle: Rc::new(RefCell::new(MockRuntimeLifecycle::default())),
        }
    }

    #[allow(dead_code)] // Each integration-test crate compiles this shared fixture independently.
    pub fn recording(mode: MockMode) -> (Self, MockRuntimeRecorder) {
        let starts = Rc::new(RefCell::new(Vec::new()));
        let cancellations = Rc::new(RefCell::new(Vec::new()));
        let lifecycle = Rc::new(RefCell::new(MockRuntimeLifecycle::default()));
        (
            Self {
                mode,
                starts: Rc::clone(&starts),
                cancellations: Rc::clone(&cancellations),
                lifecycle: Rc::clone(&lifecycle),
            },
            MockRuntimeRecorder {
                starts,
                cancellations,
                lifecycle,
            },
        )
    }
}

#[allow(dead_code)] // Used by orchestration contracts, not every importing test crate.
#[derive(Clone)]
pub struct MockRuntimeRecorder {
    starts: Rc<RefCell<Vec<MockRuntimeStart>>>,
    cancellations: Rc<RefCell<Vec<String>>>,
    lifecycle: Rc<RefCell<MockRuntimeLifecycle>>,
}

#[allow(dead_code)] // Used by orchestration contracts, not every importing test crate.
impl MockRuntimeRecorder {
    pub fn starts(&self) -> Vec<MockRuntimeStart> {
        self.starts.borrow().clone()
    }

    pub fn cancellations(&self) -> Vec<String> {
        self.cancellations.borrow().clone()
    }

    pub fn live_runs(&self) -> Vec<MockRuntimeRunRecord> {
        self.lifecycle
            .borrow()
            .live_runs
            .values()
            .cloned()
            .collect()
    }

    pub fn maximum_live_run_count(&self) -> usize {
        self.lifecycle.borrow().maximum_live_run_count
    }

    pub fn terminal_dispositions(&self) -> Vec<MockRuntimeRunRecord> {
        self.lifecycle.borrow().terminal_dispositions.clone()
    }

    pub fn nonterminal_drops(&self) -> Vec<MockRuntimeRunRecord> {
        self.lifecycle.borrow().nonterminal_drops.clone()
    }
}

#[derive(Default)]
struct MockRuntimeLifecycle {
    live_runs: BTreeMap<u8, MockRuntimeRunRecord>,
    maximum_live_run_count: usize,
    terminal_dispositions: Vec<MockRuntimeRunRecord>,
    nonterminal_drops: Vec<MockRuntimeRunRecord>,
    one_shot_cancel_failures: BTreeSet<u8>,
    live_identities: BTreeMap<u8, RuntimeRunIdentity>,
}

#[allow(dead_code)] // Used by bounded-parallelism contracts, not every importing test crate.
#[derive(Clone, Eq, PartialEq)]
pub struct MockRuntimeRunRecord {
    pub start_ordinal: u8,
    pub run_id: String,
    pub status: RuntimeRunStatus,
}

impl fmt::Debug for MockRuntimeRunRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MockRuntimeRunRecord")
            .field("start_ordinal", &self.start_ordinal)
            .field("run_id", &"[REDACTED]")
            .field("status", &self.status)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct MockRuntimeStart {
    pub run_id: String,
    pub request_id: String,
    pub selected_text: String,
}

impl fmt::Debug for MockRuntimeStart {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MockRuntimeStart")
            .field("run_id", &"[REDACTED]")
            .field("request_id", &"[REDACTED]")
            .field("selected_text", &"[REDACTED]")
            .field("selected_text_bytes", &self.selected_text.len())
            .finish()
    }
}

impl AgentRuntime for MockAgentRuntime {
    type Run = MockAgentRun;

    fn describe(&self) -> RuntimeDescriptor {
        let (availability, health, capabilities) = match self.mode {
            MockMode::Unavailable => (
                RuntimeAvailability::Unavailable,
                RuntimeHealth::Unhealthy,
                RuntimeCapabilities::new(false, false),
            ),
            MockMode::Unhealthy => (
                RuntimeAvailability::Available,
                RuntimeHealth::Unhealthy,
                RuntimeCapabilities::new(true, false),
            ),
            MockMode::CapabilityContradiction => (
                RuntimeAvailability::Available,
                RuntimeHealth::Healthy,
                RuntimeCapabilities::new(false, false),
            ),
            _ => (
                RuntimeAvailability::Available,
                RuntimeHealth::Healthy,
                RuntimeCapabilities::new(true, false),
            ),
        };
        RuntimeDescriptor::new(RuntimeId::Native, availability, health, capabilities)
    }

    fn start(&self, request: RuntimeTurnRequest) -> Result<Self::Run, RuntimeError> {
        let start_ordinal = self.starts.borrow().len() + 1;
        let start_ordinal = u8::try_from(start_ordinal)
            .map_err(|_| RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))?;
        self.starts.borrow_mut().push(MockRuntimeStart {
            run_id: request.run_id().as_str().to_owned(),
            request_id: request.request_id().as_str().to_owned(),
            selected_text: request.selected_text().as_str().to_owned(),
        });
        if matches!(self.mode, MockMode::StartFailureAt(ordinal) if ordinal == start_ordinal)
            || matches!(
                self.mode,
                MockMode::StartFailuresAt(first, second)
                    if first == start_ordinal || second == start_ordinal
            )
            || matches!(
                self.mode,
                MockMode::StartFailuresAtThree(first, second, third)
                    if first == start_ordinal
                        || second == start_ordinal
                        || third == start_ordinal
            )
        {
            return Err(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start));
        }
        match self.mode {
            MockMode::Unavailable => Err(RuntimeError::Unavailable),
            MockMode::StartFailure => {
                Err(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))
            }
            _ => {
                let requested_identity = request.identity();
                let identity = match self.mode {
                    MockMode::ReturnedIdentityMismatchAt(ordinal)
                    | MockMode::ReturnedIdentityMismatchWithCancelFailureOnceAt(ordinal)
                        if ordinal == start_ordinal =>
                    {
                        RuntimeTurnRequest::new(
                            format!("mock-foreign-run-{start_ordinal}"),
                            format!("mock-foreign-request-{start_ordinal}"),
                            "fixture-only returned identity mismatch",
                        )?
                        .identity()
                    }
                    MockMode::DuplicateLiveIdentityAt(ordinal) if ordinal == start_ordinal => self
                        .lifecycle
                        .borrow()
                        .live_identities
                        .values()
                        .next()
                        .cloned()
                        .ok_or(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))?,
                    _ => requested_identity,
                };
                let status = match self.mode {
                    MockMode::UnexpectedStartStatusAt(ordinal, status)
                        if ordinal == start_ordinal =>
                    {
                        status
                    }
                    _ => RuntimeRunStatus::AwaitingStart,
                };
                let tracked_live = !status.is_terminal();
                if tracked_live {
                    let mut lifecycle = self.lifecycle.borrow_mut();
                    lifecycle
                        .live_identities
                        .insert(start_ordinal, identity.clone());
                    lifecycle.live_runs.insert(
                        start_ordinal,
                        MockRuntimeRunRecord {
                            start_ordinal,
                            run_id: identity.run_id().as_str().to_owned(),
                            status,
                        },
                    );
                    lifecycle.maximum_live_run_count = lifecycle
                        .maximum_live_run_count
                        .max(lifecycle.live_runs.len());
                }
                Ok(MockAgentRun {
                    identity,
                    capabilities: self.describe().capabilities(),
                    mode: self.mode,
                    status,
                    next_sequence: 0,
                    start_ordinal,
                    cancellations: Rc::clone(&self.cancellations),
                    lifecycle: Rc::clone(&self.lifecycle),
                    tracked_live,
                })
            }
        }
    }
}

pub struct MockAgentRun {
    identity: RuntimeRunIdentity,
    capabilities: RuntimeCapabilities,
    mode: MockMode,
    status: RuntimeRunStatus,
    next_sequence: u32,
    start_ordinal: u8,
    cancellations: Rc<RefCell<Vec<String>>>,
    lifecycle: Rc<RefCell<MockRuntimeLifecycle>>,
    tracked_live: bool,
}

impl MockAgentRun {
    fn update_status(&mut self, status: RuntimeRunStatus) {
        self.status = status;
        if !self.tracked_live {
            return;
        }

        let mut lifecycle = self.lifecycle.borrow_mut();
        if status.is_terminal() {
            lifecycle.live_identities.remove(&self.start_ordinal);
            if let Some(mut record) = lifecycle.live_runs.remove(&self.start_ordinal) {
                record.status = status;
                lifecycle.terminal_dispositions.push(record);
            }
            self.tracked_live = false;
        } else if let Some(record) = lifecycle.live_runs.get_mut(&self.start_ordinal) {
            record.status = status;
        }
    }
}

impl Drop for MockAgentRun {
    fn drop(&mut self) {
        if !self.tracked_live {
            return;
        }

        let mut lifecycle = self.lifecycle.borrow_mut();
        lifecycle.live_identities.remove(&self.start_ordinal);
        if let Some(mut record) = lifecycle.live_runs.remove(&self.start_ordinal) {
            record.status = self.status;
            if self.status.is_terminal() {
                lifecycle.terminal_dispositions.push(record);
            } else {
                lifecycle.nonterminal_drops.push(record);
            }
        }
        self.tracked_live = false;
    }
}

impl RuntimeRun for MockAgentRun {
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
        envelope: RuntimeEventEnvelope,
    ) -> Result<RuntimeEventAcceptance, RuntimeError> {
        if self.status.is_terminal() {
            return Err(RuntimeError::EventRejected(
                RuntimeEventRejection::InvalidState,
            ));
        }

        let (run_id, request_id, sequence, event) = envelope.into_parts();
        if run_id != *self.identity.run_id() || request_id != *self.identity.request_id() {
            return Err(RuntimeError::EventRejected(
                RuntimeEventRejection::IdentityMismatch,
            ));
        }
        if sequence != self.next_sequence {
            return Err(RuntimeError::EventRejected(
                RuntimeEventRejection::InvalidSequence,
            ));
        }
        if self.mode == MockMode::EventFailure && sequence == 1 {
            self.update_status(RuntimeRunStatus::Failed);
            return Err(RuntimeError::BoundaryFailure(
                RuntimeBoundaryStage::EventAcceptance,
            ));
        }

        let accepted = match event {
            UntrustedRuntimeEvent::ResponseStarted { response_id }
                if self.status == RuntimeRunStatus::AwaitingStart =>
            {
                self.update_status(RuntimeRunStatus::Streaming);
                RuntimeEventAcceptance::ResponseStarted { response_id }
            }
            UntrustedRuntimeEvent::OutputTextDelta { .. }
                if !self.capabilities.supports(RuntimeCapability::StreamingText) =>
            {
                self.update_status(RuntimeRunStatus::Failed);
                return Err(RuntimeError::CapabilityUnavailable(
                    RuntimeCapability::StreamingText,
                ));
            }
            UntrustedRuntimeEvent::OutputTextDelta { delta }
                if self.status == RuntimeRunStatus::Streaming =>
            {
                RuntimeEventAcceptance::OutputTextDelta { delta }
            }
            UntrustedRuntimeEvent::ToolProposal { .. }
                if !self
                    .capabilities
                    .supports(RuntimeCapability::UntrustedToolProposals) =>
            {
                self.update_status(RuntimeRunStatus::Failed);
                return Err(RuntimeError::CapabilityUnavailable(
                    RuntimeCapability::UntrustedToolProposals,
                ));
            }
            UntrustedRuntimeEvent::ToolProposal { proposal }
                if self.status == RuntimeRunStatus::Streaming =>
            {
                RuntimeEventAcceptance::ToolProposal { proposal }
            }
            UntrustedRuntimeEvent::ResponseCompleted
                if self.status == RuntimeRunStatus::Streaming =>
            {
                self.update_status(RuntimeRunStatus::Completed);
                RuntimeEventAcceptance::ResponseCompleted
            }
            UntrustedRuntimeEvent::ResponseFailed { failure }
                if self.status == RuntimeRunStatus::Streaming =>
            {
                self.update_status(RuntimeRunStatus::Failed);
                RuntimeEventAcceptance::ResponseFailed { failure }
            }
            _ => {
                return Err(RuntimeError::EventRejected(
                    RuntimeEventRejection::InvalidState,
                ));
            }
        };
        self.next_sequence += 1;
        Ok(accepted)
    }

    fn cancel(&mut self) -> Result<RuntimeCancellationOutcome, RuntimeError> {
        if matches!(self.mode, MockMode::CancelFailureAt(ordinal) if ordinal == self.start_ordinal)
            && !self.status.is_terminal()
        {
            return Err(RuntimeError::BoundaryFailure(
                RuntimeBoundaryStage::Cancellation,
            ));
        }
        if matches!(
            self.mode,
            MockMode::CancelFailureOnceAt(ordinal)
                | MockMode::ReturnedIdentityMismatchWithCancelFailureOnceAt(ordinal)
                if ordinal == self.start_ordinal
        ) && !self.status.is_terminal()
            && self
                .lifecycle
                .borrow_mut()
                .one_shot_cancel_failures
                .insert(self.start_ordinal)
        {
            return Err(RuntimeError::BoundaryFailure(
                RuntimeBoundaryStage::Cancellation,
            ));
        }
        if let MockMode::CancelAlreadyTerminalAt(ordinal, status) = self.mode {
            if ordinal == self.start_ordinal && !self.status.is_terminal() {
                self.update_status(status);
                return Ok(RuntimeCancellationOutcome::AlreadyTerminal(status));
            }
        }
        if self.status.is_terminal() {
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(self.status))
        } else {
            self.cancellations
                .borrow_mut()
                .push(self.identity.run_id().as_str().to_owned());
            self.update_status(RuntimeRunStatus::Cancelled);
            Ok(RuntimeCancellationOutcome::Cancelled)
        }
    }
}
