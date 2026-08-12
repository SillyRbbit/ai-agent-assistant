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
    UnexpectedStartStatusAt(u8, RuntimeRunStatus),
    CancelFailureAt(u8),
    CancelAlreadyTerminalAt(u8, RuntimeRunStatus),
    EventFailure,
    CapabilityContradiction,
}

pub struct MockAgentRuntime {
    mode: MockMode,
    starts: Rc<RefCell<Vec<MockRuntimeStart>>>,
    cancellations: Rc<RefCell<Vec<String>>>,
}

impl MockAgentRuntime {
    pub fn new(mode: MockMode) -> Self {
        Self {
            mode,
            starts: Rc::new(RefCell::new(Vec::new())),
            cancellations: Rc::new(RefCell::new(Vec::new())),
        }
    }

    #[allow(dead_code)] // Each integration-test crate compiles this shared fixture independently.
    pub fn recording(mode: MockMode) -> (Self, MockRuntimeRecorder) {
        let starts = Rc::new(RefCell::new(Vec::new()));
        let cancellations = Rc::new(RefCell::new(Vec::new()));
        (
            Self {
                mode,
                starts: Rc::clone(&starts),
                cancellations: Rc::clone(&cancellations),
            },
            MockRuntimeRecorder {
                starts,
                cancellations,
            },
        )
    }
}

#[allow(dead_code)] // Used by orchestration contracts, not every importing test crate.
#[derive(Clone)]
pub struct MockRuntimeRecorder {
    starts: Rc<RefCell<Vec<MockRuntimeStart>>>,
    cancellations: Rc<RefCell<Vec<String>>>,
}

#[allow(dead_code)] // Used by orchestration contracts, not every importing test crate.
impl MockRuntimeRecorder {
    pub fn starts(&self) -> Vec<MockRuntimeStart> {
        self.starts.borrow().clone()
    }

    pub fn cancellations(&self) -> Vec<String> {
        self.cancellations.borrow().clone()
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
        if matches!(self.mode, MockMode::StartFailureAt(ordinal) if ordinal == start_ordinal) {
            return Err(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start));
        }
        match self.mode {
            MockMode::Unavailable => Err(RuntimeError::Unavailable),
            MockMode::StartFailure => {
                Err(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))
            }
            _ => Ok(MockAgentRun {
                identity: request.identity(),
                capabilities: self.describe().capabilities(),
                mode: self.mode,
                status: match self.mode {
                    MockMode::UnexpectedStartStatusAt(ordinal, status)
                        if ordinal == start_ordinal =>
                    {
                        status
                    }
                    _ => RuntimeRunStatus::AwaitingStart,
                },
                next_sequence: 0,
                start_ordinal,
                cancellations: Rc::clone(&self.cancellations),
            }),
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
            self.status = RuntimeRunStatus::Failed;
            return Err(RuntimeError::BoundaryFailure(
                RuntimeBoundaryStage::EventAcceptance,
            ));
        }

        let accepted = match event {
            UntrustedRuntimeEvent::ResponseStarted { response_id }
                if self.status == RuntimeRunStatus::AwaitingStart =>
            {
                self.status = RuntimeRunStatus::Streaming;
                RuntimeEventAcceptance::ResponseStarted { response_id }
            }
            UntrustedRuntimeEvent::OutputTextDelta { .. }
                if !self.capabilities.supports(RuntimeCapability::StreamingText) =>
            {
                self.status = RuntimeRunStatus::Failed;
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
                self.status = RuntimeRunStatus::Failed;
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
                self.status = RuntimeRunStatus::Completed;
                RuntimeEventAcceptance::ResponseCompleted
            }
            UntrustedRuntimeEvent::ResponseFailed { failure }
                if self.status == RuntimeRunStatus::Streaming =>
            {
                self.status = RuntimeRunStatus::Failed;
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
        if let MockMode::CancelAlreadyTerminalAt(ordinal, status) = self.mode {
            if ordinal == self.start_ordinal && !self.status.is_terminal() {
                self.status = status;
                return Ok(RuntimeCancellationOutcome::AlreadyTerminal(status));
            }
        }
        if self.status.is_terminal() {
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(self.status))
        } else {
            self.cancellations
                .borrow_mut()
                .push(self.identity.run_id().as_str().to_owned());
            self.status = RuntimeRunStatus::Cancelled;
            Ok(RuntimeCancellationOutcome::Cancelled)
        }
    }
}
use std::{cell::RefCell, fmt, rc::Rc};
