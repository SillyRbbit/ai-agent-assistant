//! Native implementation of the application-owned runtime foundation.
//!
//! `NativeAgentRuntime` is the only implementation and therefore the default.
//! It creates one unchanged [`InitialGatewayTurn`] per run and adds no provider,
//! process, network, session registry, execution path, or fallback behavior.

use std::fmt;

use serde::Serialize;

#[cfg(target_os = "macos")]
use crate::approvals::decision_source::TrustedApprovalSourceOutcome;

use super::gateway_protocol::{
    GatewayFailureCode, GatewayProtocolError, GatewayStreamStatus, GATEWAY_PROTOCOL_VERSION,
};
use super::gateway_request::{
    AuditedApprovalResolution, GatewayRequestError, InitialGatewayEvent, InitialGatewayTurn,
    InitialGatewayTurnError, InitialGatewayTurnResult,
};
use super::runtime::{
    AgentRuntime, RuntimeAvailability, RuntimeBoundaryStage, RuntimeCancellationOutcome,
    RuntimeCapabilities, RuntimeCapability, RuntimeDescriptor, RuntimeError,
    RuntimeEventAcceptance, RuntimeEventEnvelope, RuntimeEventRejection, RuntimeFailure,
    RuntimeFailureCode, RuntimeHealth, RuntimeId, RuntimeInvalidRequest, RuntimeOutputText,
    RuntimeResponseId, RuntimeResult, RuntimeRun, RuntimeRunId, RuntimeRunStatus,
    RuntimeTurnRequest, UntrustedRuntimeEvent,
};

const NATIVE_CAPABILITIES: RuntimeCapabilities = RuntimeCapabilities::new(true, false);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NativeAgentRuntime;

impl AgentRuntime for NativeAgentRuntime {
    type Run = NativeAgentRun;

    fn describe(&self) -> RuntimeDescriptor {
        RuntimeDescriptor::new(
            RuntimeId::Native,
            RuntimeAvailability::Available,
            RuntimeHealth::Healthy,
            NATIVE_CAPABILITIES,
        )
    }

    fn start(&self, request: RuntimeTurnRequest) -> RuntimeResult<Self::Run> {
        let (identity, selected_text) = request.into_parts();
        let turn = InitialGatewayTurn::new(
            identity.run_id().as_str(),
            identity.request_id().as_str(),
            selected_text.into_inner(),
        )
        .map_err(map_request_error)?;

        Ok(NativeAgentRun {
            identity,
            turn,
            lane: NativeInputLane::Unselected,
            terminal_override: None,
        })
    }
}

pub struct NativeAgentRun {
    identity: super::runtime::RuntimeRunIdentity,
    turn: InitialGatewayTurn,
    lane: NativeInputLane,
    terminal_override: Option<RuntimeRunStatus>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NativeInputLane {
    Unselected,
    ConcreteFrames,
    RuntimeEvents,
}

impl NativeAgentRun {
    /// Returns the exact request owned and serialized by `InitialGatewayTurn`.
    #[must_use]
    pub fn request_bytes(&self) -> &[u8] {
        self.turn.request_bytes()
    }

    /// Delegates normalized untrusted frames to the unchanged native boundary.
    pub fn accept_frame(
        &mut self,
        frame: &[u8],
    ) -> NativeAgentRunResult<Option<InitialGatewayEvent>> {
        if let Some(status) = self.terminal_override {
            return Err(NativeAgentRunError::Turn(
                InitialGatewayTurnError::Protocol(GatewayProtocolError::StreamAlreadyTerminal {
                    status: map_runtime_status(status),
                }),
            ));
        }
        if self.lane == NativeInputLane::RuntimeEvents {
            return Err(NativeAgentRunError::MixedInputSurface);
        }
        self.lane = NativeInputLane::ConcreteFrames;
        self.turn
            .accept_frame(frame)
            .map_err(NativeAgentRunError::Turn)
    }

    /// Uses the existing typed approval/audit termination path.
    pub fn cancel_pending_approval_for_run_termination(
        &mut self,
    ) -> InitialGatewayTurnResult<Option<AuditedApprovalResolution>> {
        let result = self.turn.cancel_pending_approval_for_run_termination()?;
        if result.is_some() {
            self.terminal_override = Some(RuntimeRunStatus::Cancelled);
        }
        Ok(result)
    }

    /// Resolves an approval only from the existing trusted macOS source.
    #[cfg(target_os = "macos")]
    pub fn resolve_approval_source_outcome(
        &mut self,
        outcome: TrustedApprovalSourceOutcome,
    ) -> InitialGatewayTurnResult<AuditedApprovalResolution> {
        self.turn.resolve_approval_source_outcome(outcome)
    }
}

pub type NativeAgentRunResult<T> = Result<T, NativeAgentRunError>;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum NativeAgentRunError {
    #[error("native run cannot mix concrete frames and shared runtime events")]
    MixedInputSurface,
    #[error(transparent)]
    Turn(#[from] InitialGatewayTurnError),
}

impl RuntimeRun for NativeAgentRun {
    fn run_id(&self) -> &RuntimeRunId {
        self.identity.run_id()
    }

    fn identity(&self) -> &super::runtime::RuntimeRunIdentity {
        &self.identity
    }

    fn status(&self) -> RuntimeRunStatus {
        self.terminal_override
            .unwrap_or_else(|| map_status(self.turn.status()))
    }

    fn accept_event(
        &mut self,
        envelope: RuntimeEventEnvelope,
    ) -> RuntimeResult<RuntimeEventAcceptance> {
        if self.status().is_terminal() {
            return Err(RuntimeError::EventRejected(
                RuntimeEventRejection::InvalidState,
            ));
        }
        if self.lane == NativeInputLane::ConcreteFrames {
            return Err(RuntimeError::EventRejected(
                RuntimeEventRejection::MixedInputSurface,
            ));
        }

        let (run_id, request_id, sequence, event) = envelope.into_parts();
        if run_id != *self.identity.run_id() || request_id != *self.identity.request_id() {
            return Err(RuntimeError::EventRejected(
                RuntimeEventRejection::IdentityMismatch,
            ));
        }
        if matches!(event, UntrustedRuntimeEvent::ToolProposal { .. })
            && !NATIVE_CAPABILITIES.supports(RuntimeCapability::UntrustedToolProposals)
        {
            self.terminal_override = Some(RuntimeRunStatus::Failed);
            return Err(RuntimeError::CapabilityUnavailable(
                RuntimeCapability::UntrustedToolProposals,
            ));
        }

        self.lane = NativeInputLane::RuntimeEvents;
        let frame = serialize_runtime_event(
            self.identity.run_id().as_str(),
            self.identity.request_id().as_str(),
            sequence,
            &event,
        )?;
        let accepted = self
            .turn
            .accept_frame(&frame)
            .map_err(map_turn_event_error)?;
        map_event_acceptance(event, accepted)
    }

    fn cancel(&mut self) -> RuntimeResult<RuntimeCancellationOutcome> {
        if let Some(status) = self.terminal_override {
            return Ok(RuntimeCancellationOutcome::AlreadyTerminal(status));
        }
        if self.turn.cancel() {
            return Ok(RuntimeCancellationOutcome::Cancelled);
        }

        let pending = self
            .turn
            .cancel_pending_approval_for_run_termination()
            .map_err(|_| RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation))?;
        if pending.is_some() {
            self.terminal_override = Some(RuntimeRunStatus::Cancelled);
            Ok(RuntimeCancellationOutcome::Cancelled)
        } else {
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(self.status()))
        }
    }
}

impl fmt::Debug for NativeAgentRun {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeAgentRun")
            .field("identity", &self.identity)
            .field("request_body", &"[REDACTED]")
            .field("request_body_bytes", &self.request_bytes().len())
            .field("status", &self.status())
            .finish()
    }
}

fn map_status(status: GatewayStreamStatus) -> RuntimeRunStatus {
    match status {
        GatewayStreamStatus::AwaitingStart => RuntimeRunStatus::AwaitingStart,
        GatewayStreamStatus::Streaming => RuntimeRunStatus::Streaming,
        GatewayStreamStatus::Completed => RuntimeRunStatus::Completed,
        GatewayStreamStatus::Failed => RuntimeRunStatus::Failed,
        GatewayStreamStatus::Cancelled => RuntimeRunStatus::Cancelled,
    }
}

fn map_runtime_status(status: RuntimeRunStatus) -> GatewayStreamStatus {
    match status {
        RuntimeRunStatus::AwaitingStart => GatewayStreamStatus::AwaitingStart,
        RuntimeRunStatus::Streaming => GatewayStreamStatus::Streaming,
        RuntimeRunStatus::Completed => GatewayStreamStatus::Completed,
        RuntimeRunStatus::Failed => GatewayStreamStatus::Failed,
        RuntimeRunStatus::Cancelled => GatewayStreamStatus::Cancelled,
    }
}

fn map_request_error(error: GatewayRequestError) -> RuntimeError {
    match error {
        GatewayRequestError::InvalidRunId => {
            RuntimeError::InvalidRequest(RuntimeInvalidRequest::RunId)
        }
        GatewayRequestError::InvalidGatewayRequestId => {
            RuntimeError::InvalidRequest(RuntimeInvalidRequest::RequestId)
        }
        GatewayRequestError::EmptySelectedContent => {
            RuntimeError::InvalidRequest(RuntimeInvalidRequest::EmptySelectedContent)
        }
        GatewayRequestError::SelectedContentTooLarge {
            maximum_bytes,
            actual_bytes,
        } => RuntimeError::InvalidRequest(RuntimeInvalidRequest::SelectedContentTooLarge {
            maximum_bytes,
            actual_bytes,
        }),
        GatewayRequestError::RequestTooLarge {
            maximum_bytes,
            actual_bytes,
        } => RuntimeError::InvalidRequest(RuntimeInvalidRequest::SerializedRequestTooLarge {
            maximum_bytes,
            actual_bytes,
        }),
        GatewayRequestError::SerializationFailed => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::RequestSerialization)
        }
        GatewayRequestError::ValidatorConfigurationFailed => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::NativeConfiguration)
        }
    }
}

fn serialize_runtime_event(
    run_id: &str,
    request_id: &str,
    sequence: u32,
    event: &UntrustedRuntimeEvent,
) -> RuntimeResult<Vec<u8>> {
    let wire_event = match event {
        UntrustedRuntimeEvent::ResponseStarted { response_id } => {
            WireRuntimeEvent::ResponseStarted {
                provider_response_id: response_id.as_str(),
            }
        }
        UntrustedRuntimeEvent::OutputTextDelta { delta } => WireRuntimeEvent::OutputTextDelta {
            delta: delta.as_str(),
        },
        UntrustedRuntimeEvent::ToolProposal { proposal } => {
            WireRuntimeEvent::FunctionCallCompleted {
                call_id: proposal.call_id(),
                name: proposal.name(),
                tool_contract_version: proposal.tool_contract_version(),
                arguments_json: proposal.arguments_json(),
            }
        }
        UntrustedRuntimeEvent::ResponseCompleted => WireRuntimeEvent::ResponseCompleted,
        UntrustedRuntimeEvent::ResponseFailed { failure } => WireRuntimeEvent::ResponseFailed {
            code: map_runtime_failure_code(failure.code()),
            retryable: failure.retryable(),
            retry_after_ms: failure.retry_after_ms(),
        },
    };

    serde_json::to_vec(&WireRuntimeEnvelope {
        protocol_version: GATEWAY_PROTOCOL_VERSION,
        run_id,
        gateway_request_id: request_id,
        sequence,
        event: wire_event,
    })
    .map_err(|_| RuntimeError::BoundaryFailure(RuntimeBoundaryStage::EventTranslation))
}

fn map_event_acceptance(
    submitted: UntrustedRuntimeEvent,
    accepted: Option<InitialGatewayEvent>,
) -> RuntimeResult<RuntimeEventAcceptance> {
    match (submitted, accepted) {
        (
            UntrustedRuntimeEvent::ResponseStarted { .. },
            Some(InitialGatewayEvent::ResponseStarted {
                provider_response_id,
            }),
        ) => Ok(RuntimeEventAcceptance::ResponseStarted {
            response_id: RuntimeResponseId::from_validated(provider_response_id),
        }),
        (
            UntrustedRuntimeEvent::OutputTextDelta { .. },
            Some(InitialGatewayEvent::OutputTextDelta { delta }),
        ) => Ok(RuntimeEventAcceptance::OutputTextDelta {
            delta: RuntimeOutputText::from_validated(delta),
        }),
        (UntrustedRuntimeEvent::ToolProposal { proposal }, None) => {
            Ok(RuntimeEventAcceptance::ToolProposal { proposal })
        }
        (
            UntrustedRuntimeEvent::ResponseCompleted,
            Some(InitialGatewayEvent::ResponseCompleted),
        ) => Ok(RuntimeEventAcceptance::ResponseCompleted),
        (
            UntrustedRuntimeEvent::ResponseFailed { .. },
            Some(InitialGatewayEvent::ResponseFailed { failure }),
        ) => Ok(RuntimeEventAcceptance::ResponseFailed {
            failure: RuntimeFailure::from_validated(
                map_gateway_failure_code(failure.code()),
                failure.retryable(),
                failure.retry_after_ms(),
            ),
        }),
        (
            _,
            Some(
                InitialGatewayEvent::PolicyEvaluated { .. }
                | InitialGatewayEvent::ApprovalPresentationReady { .. },
            ),
        ) => Err(RuntimeError::BoundaryFailure(
            RuntimeBoundaryStage::GovernanceIsolation,
        )),
        _ => Err(RuntimeError::BoundaryFailure(
            RuntimeBoundaryStage::EventAcceptance,
        )),
    }
}

fn map_turn_event_error(error: InitialGatewayTurnError) -> RuntimeError {
    match error {
        InitialGatewayTurnError::Protocol(error) => map_protocol_error(error),
        InitialGatewayTurnError::FunctionCallValidation(_) => {
            RuntimeError::EventRejected(RuntimeEventRejection::InvalidContent)
        }
        InitialGatewayTurnError::Approval(_) | InitialGatewayTurnError::ApprovalAudit(_) => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::GovernanceIsolation)
        }
    }
}

fn map_protocol_error(error: GatewayProtocolError) -> RuntimeError {
    let rejection = match error {
        GatewayProtocolError::InvalidExpectedRunId
        | GatewayProtocolError::InvalidExpectedGatewayRequestId
        | GatewayProtocolError::InvalidAllowedToolName
        | GatewayProtocolError::InvalidExpectedToolContractVersion => {
            return RuntimeError::BoundaryFailure(RuntimeBoundaryStage::NativeConfiguration);
        }
        GatewayProtocolError::EventTooLarge { .. }
        | GatewayProtocolError::EventLimitExceeded { .. }
        | GatewayProtocolError::AssistantOutputLimitExceeded { .. }
        | GatewayProtocolError::FunctionCallLimitExceeded { .. }
        | GatewayProtocolError::FunctionArgumentsTooLarge { .. }
        | GatewayProtocolError::RetryDelayTooLarge { .. } => RuntimeEventRejection::LimitExceeded,
        GatewayProtocolError::MalformedEvent => RuntimeEventRejection::Malformed,
        GatewayProtocolError::UnsupportedProtocolVersion { .. } => {
            RuntimeEventRejection::UnsupportedVersion
        }
        GatewayProtocolError::InvalidRunId
        | GatewayProtocolError::RunIdMismatch
        | GatewayProtocolError::InvalidGatewayRequestId
        | GatewayProtocolError::GatewayRequestIdMismatch => RuntimeEventRejection::IdentityMismatch,
        GatewayProtocolError::InvalidSequence { .. } => RuntimeEventRejection::InvalidSequence,
        GatewayProtocolError::StreamAlreadyTerminal { .. }
        | GatewayProtocolError::EventBeforeResponseStarted
        | GatewayProtocolError::UnexpectedResponseStarted
        | GatewayProtocolError::MixedResponseOutput
        | GatewayProtocolError::MissingResponseOutput => RuntimeEventRejection::InvalidState,
        GatewayProtocolError::InvalidProviderResponseId
        | GatewayProtocolError::EmptyOutputTextDelta
        | GatewayProtocolError::InvalidFunctionCallId
        | GatewayProtocolError::InvalidFunctionName
        | GatewayProtocolError::UnknownFunctionName
        | GatewayProtocolError::ToolContractVersionMismatch { .. }
        | GatewayProtocolError::MalformedFunctionArguments
        | GatewayProtocolError::FunctionArgumentsMustBeObject
        | GatewayProtocolError::DuplicateFunctionArgumentKey => {
            RuntimeEventRejection::InvalidContent
        }
    };
    RuntimeError::EventRejected(rejection)
}

fn map_runtime_failure_code(code: RuntimeFailureCode) -> WireGatewayFailureCode {
    match code {
        RuntimeFailureCode::Unauthenticated => WireGatewayFailureCode::Unauthenticated,
        RuntimeFailureCode::Forbidden => WireGatewayFailureCode::Forbidden,
        RuntimeFailureCode::RateLimited => WireGatewayFailureCode::RateLimited,
        RuntimeFailureCode::RequestRejected => WireGatewayFailureCode::RequestRejected,
        RuntimeFailureCode::ProviderUnavailable => WireGatewayFailureCode::ProviderUnavailable,
        RuntimeFailureCode::ProviderTimeout => WireGatewayFailureCode::ProviderTimeout,
        RuntimeFailureCode::ProtocolViolation => WireGatewayFailureCode::ProtocolViolation,
        RuntimeFailureCode::LimitExceeded => WireGatewayFailureCode::LimitExceeded,
        RuntimeFailureCode::Cancelled => WireGatewayFailureCode::Cancelled,
        RuntimeFailureCode::Internal => WireGatewayFailureCode::Internal,
    }
}

fn map_gateway_failure_code(code: GatewayFailureCode) -> RuntimeFailureCode {
    match code {
        GatewayFailureCode::Unauthenticated => RuntimeFailureCode::Unauthenticated,
        GatewayFailureCode::Forbidden => RuntimeFailureCode::Forbidden,
        GatewayFailureCode::RateLimited => RuntimeFailureCode::RateLimited,
        GatewayFailureCode::RequestRejected => RuntimeFailureCode::RequestRejected,
        GatewayFailureCode::ProviderUnavailable => RuntimeFailureCode::ProviderUnavailable,
        GatewayFailureCode::ProviderTimeout => RuntimeFailureCode::ProviderTimeout,
        GatewayFailureCode::ProtocolViolation => RuntimeFailureCode::ProtocolViolation,
        GatewayFailureCode::LimitExceeded => RuntimeFailureCode::LimitExceeded,
        GatewayFailureCode::Cancelled => RuntimeFailureCode::Cancelled,
        GatewayFailureCode::Internal => RuntimeFailureCode::Internal,
    }
}

#[derive(Serialize)]
struct WireRuntimeEnvelope<'a> {
    protocol_version: u16,
    run_id: &'a str,
    gateway_request_id: &'a str,
    sequence: u32,
    event: WireRuntimeEvent<'a>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WireRuntimeEvent<'a> {
    ResponseStarted {
        provider_response_id: &'a str,
    },
    OutputTextDelta {
        delta: &'a str,
    },
    FunctionCallCompleted {
        call_id: &'a str,
        name: &'a str,
        tool_contract_version: u16,
        arguments_json: &'a str,
    },
    ResponseCompleted,
    ResponseFailed {
        code: WireGatewayFailureCode,
        retryable: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_after_ms: Option<u64>,
    },
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum WireGatewayFailureCode {
    Unauthenticated,
    Forbidden,
    RateLimited,
    RequestRejected,
    ProviderUnavailable,
    ProviderTimeout,
    ProtocolViolation,
    LimitExceeded,
    Cancelled,
    Internal,
}
