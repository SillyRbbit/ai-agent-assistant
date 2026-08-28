//! Native implementation of the application-owned runtime foundation.
//!
//! `NativeAgentRuntime` is the only implementation and therefore the default.
//! It creates one application-selected sealed turn per run and adds no provider,
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
    InitialGatewayTurnError, InitialGatewayTurnResult, PersonalAssistantFailureCode,
    PersonalAssistantTextEvent, PersonalAssistantTextTurn, PersonalAssistantTextTurnError,
};
use super::runtime::{
    AgentRuntime, RuntimeAvailability, RuntimeBoundaryStage, RuntimeCancellationOutcome,
    RuntimeCapabilities, RuntimeCapability, RuntimeDescriptor, RuntimeError,
    RuntimeEventAcceptance, RuntimeEventEnvelope, RuntimeEventRejection, RuntimeFailure,
    RuntimeFailureCode, RuntimeHealth, RuntimeId, RuntimeInvalidRequest, RuntimeOutputText,
    RuntimeResponseId, RuntimeResult, RuntimeRun, RuntimeRunId, RuntimeRunStatus,
    RuntimeTurnProfile, RuntimeTurnRequest, UntrustedRuntimeEvent,
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
        let (identity, selected_text, profile) = request.into_parts();
        let turn = match profile {
            RuntimeTurnProfile::Initial => NativeTurn::Initial(Box::new(
                InitialGatewayTurn::new(
                    identity.run_id().as_str(),
                    identity.request_id().as_str(),
                    selected_text.into_inner(),
                )
                .map_err(map_request_error)?,
            )),
            RuntimeTurnProfile::PersonalAssistantV0Synthetic => {
                NativeTurn::PersonalAssistant(Box::new(
                    PersonalAssistantTextTurn::new(
                        identity.run_id().as_str(),
                        identity.request_id().as_str(),
                        selected_text.into_inner(),
                    )
                    .map_err(map_personal_assistant_start_error)?,
                ))
            }
        };

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
    turn: NativeTurn,
    lane: NativeInputLane,
    terminal_override: Option<RuntimeRunStatus>,
}

enum NativeTurn {
    Initial(Box<InitialGatewayTurn>),
    PersonalAssistant(Box<PersonalAssistantTextTurn>),
}

impl NativeTurn {
    fn request_bytes(&self) -> &[u8] {
        match self {
            Self::Initial(turn) => turn.request_bytes(),
            Self::PersonalAssistant(turn) => turn.request_bytes(),
        }
    }

    fn status(&self) -> GatewayStreamStatus {
        match self {
            Self::Initial(turn) => turn.status(),
            Self::PersonalAssistant(turn) => turn.status(),
        }
    }

    fn cancel(&mut self) -> bool {
        match self {
            Self::Initial(turn) => turn.cancel(),
            Self::PersonalAssistant(turn) => turn.cancel(),
        }
    }

    fn accept_personal_assistant_frame(
        &mut self,
        frame: &[u8],
    ) -> Result<PersonalAssistantTextEvent, PersonalAssistantTextTurnError> {
        match self {
            Self::Initial(_) => Err(PersonalAssistantTextTurnError::WrongProfile),
            Self::PersonalAssistant(turn) => turn.accept_frame(frame),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NativeInputLane {
    Unselected,
    ConcreteFrames,
    RuntimeEvents,
}

impl NativeAgentRun {
    /// Returns the exact request owned and serialized by the selected sealed turn.
    #[must_use]
    pub fn request_bytes(&self) -> &[u8] {
        self.turn.request_bytes()
    }

    /// Delegates normalized untrusted frames to the unchanged native boundary.
    pub fn accept_frame(
        &mut self,
        frame: &[u8],
    ) -> NativeAgentRunResult<Option<InitialGatewayEvent>> {
        let NativeTurn::Initial(turn) = &mut self.turn else {
            return Err(NativeAgentRunError::WrongProfile);
        };
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
        turn.accept_frame(frame).map_err(NativeAgentRunError::Turn)
    }

    /// Uses the existing typed approval/audit termination path.
    pub fn cancel_pending_approval_for_run_termination(
        &mut self,
    ) -> InitialGatewayTurnResult<Option<AuditedApprovalResolution>> {
        let result = match &mut self.turn {
            NativeTurn::Initial(turn) => turn.cancel_pending_approval_for_run_termination()?,
            NativeTurn::PersonalAssistant(_) => return Ok(None),
        };
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
        match &mut self.turn {
            NativeTurn::Initial(turn) => turn.resolve_approval_source_outcome(outcome),
            NativeTurn::PersonalAssistant(_) => Err(InitialGatewayTurnError::WrongProfile),
        }
    }
}

pub type NativeAgentRunResult<T> = Result<T, NativeAgentRunError>;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum NativeAgentRunError {
    #[error("native concrete frame operation is unavailable for this runtime profile")]
    WrongProfile,
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
        if let NativeTurn::Initial(turn) = &mut self.turn {
            let accepted = turn.accept_frame(&frame).map_err(map_turn_event_error)?;
            map_event_acceptance(event, accepted)
        } else {
            let accepted = self
                .turn
                .accept_personal_assistant_frame(&frame)
                .map_err(map_personal_assistant_event_error)?;
            map_personal_assistant_event_acceptance(event, accepted)
        }
    }

    fn cancel(&mut self) -> RuntimeResult<RuntimeCancellationOutcome> {
        if let Some(status) = self.terminal_override {
            return Ok(RuntimeCancellationOutcome::AlreadyTerminal(status));
        }
        if self.turn.cancel() {
            return Ok(RuntimeCancellationOutcome::Cancelled);
        }

        let pending = match &mut self.turn {
            NativeTurn::Initial(turn) => turn
                .cancel_pending_approval_for_run_termination()
                .map_err(|_| RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation))?,
            NativeTurn::PersonalAssistant(_) => None,
        };
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

fn map_personal_assistant_start_error(error: PersonalAssistantTextTurnError) -> RuntimeError {
    match error {
        PersonalAssistantTextTurnError::InvalidTrustedIdentity
        | PersonalAssistantTextTurnError::WrongProfile
        | PersonalAssistantTextTurnError::ProtocolViolation => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::NativeConfiguration)
        }
        PersonalAssistantTextTurnError::SerializationFailed => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::RequestSerialization)
        }
        PersonalAssistantTextTurnError::LimitExceeded => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start)
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

fn map_personal_assistant_event_acceptance(
    submitted: UntrustedRuntimeEvent,
    accepted: PersonalAssistantTextEvent,
) -> RuntimeResult<RuntimeEventAcceptance> {
    match (submitted, accepted) {
        (
            UntrustedRuntimeEvent::ResponseStarted { response_id },
            PersonalAssistantTextEvent::Started,
        ) => Ok(RuntimeEventAcceptance::ResponseStarted { response_id }),
        (
            UntrustedRuntimeEvent::OutputTextDelta { .. },
            PersonalAssistantTextEvent::TextDelta { delta },
        ) => Ok(RuntimeEventAcceptance::OutputTextDelta {
            delta: RuntimeOutputText::from_validated(delta.into_inner()),
        }),
        (
            UntrustedRuntimeEvent::ResponseCompleted,
            PersonalAssistantTextEvent::Completed { final_answer },
        ) => {
            drop(final_answer.into_inner());
            Ok(RuntimeEventAcceptance::ResponseCompleted)
        }
        (
            UntrustedRuntimeEvent::ResponseFailed { .. },
            PersonalAssistantTextEvent::Failed { code },
        ) => Ok(RuntimeEventAcceptance::ResponseFailed {
            failure: RuntimeFailure::from_validated(
                map_personal_assistant_failure_code(code),
                false,
                None,
            ),
        }),
        _ => Err(RuntimeError::BoundaryFailure(
            RuntimeBoundaryStage::EventAcceptance,
        )),
    }
}

fn map_turn_event_error(error: InitialGatewayTurnError) -> RuntimeError {
    match error {
        InitialGatewayTurnError::WrongProfile => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::GovernanceIsolation)
        }
        InitialGatewayTurnError::Protocol(error) => map_protocol_error(error),
        InitialGatewayTurnError::FunctionCallValidation(_) => {
            RuntimeError::EventRejected(RuntimeEventRejection::InvalidContent)
        }
        InitialGatewayTurnError::Approval(_) | InitialGatewayTurnError::ApprovalAudit(_) => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::GovernanceIsolation)
        }
    }
}

fn map_personal_assistant_event_error(error: PersonalAssistantTextTurnError) -> RuntimeError {
    match error {
        PersonalAssistantTextTurnError::InvalidTrustedIdentity
        | PersonalAssistantTextTurnError::SerializationFailed
        | PersonalAssistantTextTurnError::WrongProfile => {
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::NativeConfiguration)
        }
        PersonalAssistantTextTurnError::ProtocolViolation => {
            RuntimeError::EventRejected(RuntimeEventRejection::InvalidContent)
        }
        PersonalAssistantTextTurnError::LimitExceeded => {
            RuntimeError::EventRejected(RuntimeEventRejection::LimitExceeded)
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

fn map_personal_assistant_failure_code(code: PersonalAssistantFailureCode) -> RuntimeFailureCode {
    match code {
        PersonalAssistantFailureCode::Unauthenticated => RuntimeFailureCode::Unauthenticated,
        PersonalAssistantFailureCode::Forbidden => RuntimeFailureCode::Forbidden,
        PersonalAssistantFailureCode::RateLimited => RuntimeFailureCode::RateLimited,
        PersonalAssistantFailureCode::RequestRejected => RuntimeFailureCode::RequestRejected,
        PersonalAssistantFailureCode::ProviderUnavailable => {
            RuntimeFailureCode::ProviderUnavailable
        }
        PersonalAssistantFailureCode::ProviderTimeout => RuntimeFailureCode::ProviderTimeout,
        PersonalAssistantFailureCode::ProtocolViolation => RuntimeFailureCode::ProtocolViolation,
        PersonalAssistantFailureCode::LimitExceeded => RuntimeFailureCode::LimitExceeded,
        PersonalAssistantFailureCode::Internal => RuntimeFailureCode::Internal,
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

#[cfg(test)]
mod tests {
    use super::{NativeAgentRunError, NativeAgentRuntime, NativeTurn};
    use crate::agent::gateway_protocol::GatewayStreamStatus;
    #[cfg(target_os = "macos")]
    use crate::agent::gateway_protocol::GATEWAY_PROTOCOL_VERSION;
    #[cfg(target_os = "macos")]
    use crate::agent::gateway_request::{InitialGatewayEvent, InitialGatewayTurnError};
    use crate::agent::gateway_request::{
        InitialGatewayTurn, PersonalAssistantTextTurnError, INITIAL_GATEWAY_TOOL_SET_VERSION,
    };
    use crate::agent::runtime::{
        AgentRuntime, RuntimeCancellationOutcome, RuntimeError, RuntimeEventAcceptance,
        RuntimeEventEnvelope, RuntimeEventRejection, RuntimeFailure, RuntimeFailureCode,
        RuntimeOutputText, RuntimeResponseId, RuntimeRun, RuntimeRunStatus, RuntimeTurnRequest,
        UntrustedRuntimeEvent, UntrustedRuntimeToolProposal,
    };
    use serde_json::json;

    const RUN_ID: &str = "pa-v0-native-run-1";
    const REQUEST_ID: &str = "pa-v0-native-request-1";

    fn personal_assistant_request() -> Result<RuntimeTurnRequest, RuntimeError> {
        RuntimeTurnRequest::personal_assistant_v0_synthetic(
            RUN_ID.to_owned(),
            REQUEST_ID.to_owned(),
        )
    }

    fn envelope(
        identity: &crate::agent::runtime::RuntimeRunIdentity,
        sequence: u32,
        event: UntrustedRuntimeEvent,
    ) -> RuntimeEventEnvelope {
        RuntimeEventEnvelope::for_identity(identity, sequence, event)
    }

    #[test]
    fn native_runtime_routes_both_boxed_profiles_through_one_start_method(
    ) -> Result<(), RuntimeError> {
        let initial_request = RuntimeTurnRequest::new("initial-run", "initial-request", "Plan")?;
        let initial = NativeAgentRuntime.start(initial_request)?;
        assert!(matches!(initial.turn, NativeTurn::Initial(_)));

        let personal = NativeAgentRuntime.start(personal_assistant_request()?)?;
        assert!(matches!(personal.turn, NativeTurn::PersonalAssistant(_)));
        assert_eq!(personal.status(), RuntimeRunStatus::AwaitingStart);
        let body: serde_json::Value = serde_json::from_slice(personal.request_bytes())
            .map_err(|_| RuntimeError::BoundaryFailure(super::RuntimeBoundaryStage::Start))?;
        assert_eq!(body["request_kind"], "personal_assistant_text_v0");
        assert_eq!(body["tool_set"]["tools"], json!([]));
        Ok(())
    }

    #[test]
    fn personal_assistant_shared_events_accept_text_lifecycle_and_closed_failure(
    ) -> Result<(), RuntimeError> {
        let request = personal_assistant_request()?;
        let identity = request.identity();
        let mut run = NativeAgentRuntime.start(request)?;

        let started = run.accept_event(envelope(
            &identity,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("pa-provider-response-1")?,
            },
        ))?;
        assert!(matches!(
            started,
            RuntimeEventAcceptance::ResponseStarted { response_id }
                if response_id.as_str() == "pa-provider-response-1"
        ));
        assert_eq!(run.status(), RuntimeRunStatus::Streaming);

        let delta = run.accept_event(envelope(
            &identity,
            1,
            UntrustedRuntimeEvent::OutputTextDelta {
                delta: RuntimeOutputText::new("Synthetic board update")?,
            },
        ))?;
        assert!(matches!(
            delta,
            RuntimeEventAcceptance::OutputTextDelta { delta }
                if delta.as_str() == "Synthetic board update"
        ));
        assert_eq!(
            run.accept_event(envelope(
                &identity,
                2,
                UntrustedRuntimeEvent::ResponseCompleted,
            ))?,
            RuntimeEventAcceptance::ResponseCompleted
        );
        assert_eq!(run.status(), RuntimeRunStatus::Completed);

        let failure_request = RuntimeTurnRequest::personal_assistant_v0_synthetic(
            "pa-v0-native-run-2".to_owned(),
            "pa-v0-native-request-2".to_owned(),
        )?;
        let failure_identity = failure_request.identity();
        let mut failed = NativeAgentRuntime.start(failure_request)?;
        failed.accept_event(envelope(
            &failure_identity,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("pa-provider-response-2")?,
            },
        ))?;
        let accepted = failed.accept_event(envelope(
            &failure_identity,
            1,
            UntrustedRuntimeEvent::ResponseFailed {
                failure: RuntimeFailure::new(RuntimeFailureCode::ProviderTimeout, false, None)?,
            },
        ))?;
        assert!(matches!(
            accepted,
            RuntimeEventAcceptance::ResponseFailed { failure }
                if failure.code() == RuntimeFailureCode::ProviderTimeout
                    && !failure.retryable()
                    && failure.retry_after_ms().is_none()
        ));
        assert_eq!(failed.status(), RuntimeRunStatus::Failed);
        Ok(())
    }

    #[test]
    fn personal_assistant_wrong_profile_and_governance_methods_fail_closed_without_mutation(
    ) -> Result<(), RuntimeError> {
        let request = personal_assistant_request()?;
        let identity = request.identity();
        let mut run = NativeAgentRuntime.start(request)?;

        assert!(matches!(
            run.accept_frame(b"{"),
            Err(NativeAgentRunError::WrongProfile)
        ));
        assert_eq!(run.status(), RuntimeRunStatus::AwaitingStart);
        assert!(run
            .cancel_pending_approval_for_run_termination()
            .map_err(|_| RuntimeError::BoundaryFailure(super::RuntimeBoundaryStage::Cancellation))?
            .is_none());
        assert_eq!(run.status(), RuntimeRunStatus::AwaitingStart);
        assert!(matches!(
            run.accept_event(envelope(
                &identity,
                0,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: RuntimeResponseId::new("pa-provider-response-3")?,
                },
            ))?,
            RuntimeEventAcceptance::ResponseStarted { .. }
        ));

        let initial = InitialGatewayTurn::new("initial-run", "initial-request", "Plan")
            .map_err(|_| RuntimeError::BoundaryFailure(super::RuntimeBoundaryStage::Start))?;
        let mut turn = NativeTurn::Initial(Box::new(initial));
        assert_eq!(
            turn.accept_personal_assistant_frame(b"{}"),
            Err(PersonalAssistantTextTurnError::WrongProfile)
        );
        assert_eq!(turn.status(), GatewayStreamStatus::AwaitingStart);
        Ok(())
    }

    #[test]
    fn personal_assistant_shared_tool_retry_and_limit_rejections_are_terminal(
    ) -> Result<(), RuntimeError> {
        let request = personal_assistant_request()?;
        let identity = request.identity();
        let mut tool_run = NativeAgentRuntime.start(request)?;
        let tool = UntrustedRuntimeToolProposal::new(
            "call-1",
            "create_local_task",
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            "{}",
        )?;
        assert!(matches!(
            tool_run.accept_event(envelope(
                &identity,
                0,
                UntrustedRuntimeEvent::ToolProposal { proposal: tool },
            )),
            Err(RuntimeError::CapabilityUnavailable(_))
        ));
        assert_eq!(tool_run.status(), RuntimeRunStatus::Failed);

        let retry_request = RuntimeTurnRequest::personal_assistant_v0_synthetic(
            "pa-v0-native-run-4".to_owned(),
            "pa-v0-native-request-4".to_owned(),
        )?;
        let retry_identity = retry_request.identity();
        let mut retry_run = NativeAgentRuntime.start(retry_request)?;
        retry_run.accept_event(envelope(
            &retry_identity,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("pa-provider-response-4")?,
            },
        ))?;
        assert_eq!(
            retry_run.accept_event(envelope(
                &retry_identity,
                1,
                UntrustedRuntimeEvent::ResponseFailed {
                    failure: RuntimeFailure::new(RuntimeFailureCode::ProviderTimeout, true, None,)?,
                },
            )),
            Err(RuntimeError::EventRejected(
                RuntimeEventRejection::InvalidContent
            ))
        );
        assert_eq!(retry_run.status(), RuntimeRunStatus::Failed);

        let limit_request = RuntimeTurnRequest::personal_assistant_v0_synthetic(
            "pa-v0-native-run-5".to_owned(),
            "pa-v0-native-request-5".to_owned(),
        )?;
        let limit_identity = limit_request.identity();
        let mut limit_run = NativeAgentRuntime.start(limit_request)?;
        limit_run.accept_event(envelope(
            &limit_identity,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("pa-provider-response-5")?,
            },
        ))?;
        assert_eq!(
            limit_run.accept_event(envelope(
                &limit_identity,
                1,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: RuntimeOutputText::new("x".repeat(1_025))?,
                },
            )),
            Err(RuntimeError::EventRejected(
                RuntimeEventRejection::LimitExceeded
            ))
        );
        assert_eq!(limit_run.status(), RuntimeRunStatus::Failed);
        Ok(())
    }

    #[test]
    fn personal_assistant_runtime_cancellation_is_exact_and_idempotent() -> Result<(), RuntimeError>
    {
        let mut run = NativeAgentRuntime.start(personal_assistant_request()?)?;
        assert_eq!(run.cancel()?, RuntimeCancellationOutcome::Cancelled);
        assert_eq!(run.status(), RuntimeRunStatus::Cancelled);
        assert_eq!(
            run.cancel()?,
            RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Cancelled)
        );
        assert!(run
            .cancel_pending_approval_for_run_termination()
            .map_err(|_| RuntimeError::BoundaryFailure(super::RuntimeBoundaryStage::Cancellation))?
            .is_none());
        assert_eq!(run.status(), RuntimeRunStatus::Cancelled);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn personal_assistant_rejects_native_approval_source_before_outcome_inspection(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::approvals::decision_source::test_outcome_from_dialog_result;
        use rfd::MessageDialogResult;

        let mut initial = NativeAgentRuntime.start(RuntimeTurnRequest::new(
            "approval-seed-run",
            "approval-seed-request",
            "Plan",
        )?)?;
        let frame = |sequence, event| {
            json!({
                "protocol_version": GATEWAY_PROTOCOL_VERSION,
                "run_id": "approval-seed-run",
                "gateway_request_id": "approval-seed-request",
                "sequence": sequence,
                "event": event,
            })
            .to_string()
            .into_bytes()
        };
        initial.accept_frame(&frame(
            0,
            json!({
                "type": "response_started",
                "provider_response_id": "approval-seed-response",
            }),
        ))?;
        initial.accept_frame(&frame(
            1,
            json!({
                "type": "function_call_completed",
                "call_id": "approval-seed-call",
                "name": "create_local_task",
                "tool_contract_version": INITIAL_GATEWAY_TOOL_SET_VERSION,
                "arguments_json": r#"{"title":"Synthetic task"}"#,
            }),
        ))?;
        let presentation = match initial
            .accept_frame(&frame(2, json!({ "type": "response_completed" })))?
        {
            Some(InitialGatewayEvent::ApprovalPresentationReady { presentation }) => presentation,
            _ => return Err("expected approval presentation".into()),
        };
        let outcome = test_outcome_from_dialog_result(presentation, MessageDialogResult::Cancel);

        let mut personal = NativeAgentRuntime.start(personal_assistant_request()?)?;
        assert!(matches!(
            personal.resolve_approval_source_outcome(outcome),
            Err(InitialGatewayTurnError::WrongProfile)
        ));
        assert_eq!(personal.status(), RuntimeRunStatus::AwaitingStart);
        Ok(())
    }
}
