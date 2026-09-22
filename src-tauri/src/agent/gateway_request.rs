use std::fmt;

use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

use super::function_call_validation::{
    validate_function_call, FunctionCallValidationError, SchemaValidatedFunctionCall,
};
use super::gateway_protocol::{
    is_valid_opaque_id, GatewayFailure, GatewayProtocolError, GatewayStreamStatus,
    GatewayStreamValidator, ValidatedGatewayEvent, AGENT_RUN_DEADLINE, GATEWAY_CONNECT_TIMEOUT,
    GATEWAY_PROTOCOL_VERSION, GATEWAY_STREAM_IDLE_TIMEOUT,
    MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN, MAX_FUNCTION_ARGUMENT_BYTES,
    MAX_FUNCTION_CALLS_PER_RUN, MAX_GATEWAY_EVENTS_PER_TURN, MAX_GATEWAY_EVENT_BYTES,
    MAX_GATEWAY_REQUESTS_PER_RUN, MAX_GATEWAY_REQUEST_BYTES, MAX_MODEL_TURNS_PER_RUN,
    MAX_RETRY_AFTER_MS, MAX_RETRY_ATTEMPTS_PER_RUN, PROVIDER_TURN_DEADLINE,
};
#[cfg(target_os = "macos")]
use crate::approvals::decision_source::TrustedApprovalSourceOutcome;
use crate::approvals::manager::{
    ApprovalError, ApprovalManager, ApprovalPresentation, InMemoryApprovalManager,
};
use crate::approvals::types::{ApprovalId, ApprovalResolution};
use crate::audit::approval::{
    ApprovalAuditError, ApprovalAuditReceipt, InMemoryApprovalAuditAdapter,
};
use crate::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
use crate::policy::types::{PolicyDecision, PolicyInput, PolicyOutcome};
use crate::tools::registry::{InMemoryToolRegistry, ToolRegistry};
use crate::tools::schema::ToolSchema;
use crate::tools::types::ToolDefinition;

pub const INITIAL_GATEWAY_TOOL_SET_ID: &str = "cortexa_desktop_mvp";
pub const INITIAL_GATEWAY_TOOL_SET_VERSION: u16 = 1;

const PERSONAL_ASSISTANT_V0_CONTRACT_VERSION: u16 = 1;
const PERSONAL_ASSISTANT_V0_INSTRUCTION_PROFILE_ID: &str = "personal-assistant-text-v0";
const PERSONAL_ASSISTANT_V0_INSTRUCTION_PROFILE_VERSION: u16 = 1;
const PERSONAL_ASSISTANT_V0_INSTRUCTIONS: &str = "Act as Cortexa's Personal Assistant for one bounded synthetic text request. Answer only from the supplied application-owned synthetic text and return concise plain text. Do not call or propose tools, access files, memory, retrieval, networks, or devices, delegate, schedule, persist, approve, execute, retry, or claim any action or context not supplied.";
const PERSONAL_ASSISTANT_V0_PROVIDER_PROFILE_ID: &str = "openai-cloudflare-personal-assistant-v0";
const PERSONAL_ASSISTANT_V0_PROVIDER_PROFILE_VERSION: u16 = 1;
const PERSONAL_ASSISTANT_V0_DATA_CLASS_ID: &str = "application-owned-synthetic";
const PERSONAL_ASSISTANT_V0_DATA_CLASS_VERSION: u16 = 1;
const PERSONAL_ASSISTANT_V0_TOOL_SET_ID: &str = "empty";
const PERSONAL_ASSISTANT_V0_TOOL_SET_VERSION: u16 = 1;
const PERSONAL_ASSISTANT_V0_MAX_INPUT_CHARACTERS: usize = 4_096;
const PERSONAL_ASSISTANT_V0_MAX_INPUT_BYTES: usize = 16_384;
const PERSONAL_ASSISTANT_V0_MAX_EVENTS: usize = 128;
const PERSONAL_ASSISTANT_V0_MAX_DELTA_CHARACTERS: usize = 1_024;
const PERSONAL_ASSISTANT_V0_MAX_DELTA_BYTES: usize = 4_096;
const PERSONAL_ASSISTANT_V0_MAX_OUTPUT_CHARACTERS: usize = 8_192;
const PERSONAL_ASSISTANT_V0_MAX_OUTPUT_BYTES: usize = 32_768;

pub type GatewayRequestResult<T> = Result<T, GatewayRequestError>;

pub struct InitialGatewayTurn {
    request: InitialGatewayRequest,
    validator: GatewayStreamValidator,
    registry: InMemoryToolRegistry,
    approval_manager: InMemoryApprovalManager,
    approval_audit: InMemoryApprovalAuditAdapter,
    pending_approval_id: Option<ApprovalId>,
    pending_function_call: Option<SchemaValidatedFunctionCall>,
    local_schema_failed: bool,
}

impl InitialGatewayTurn {
    pub fn new(
        run_id: impl Into<String>,
        gateway_request_id: impl Into<String>,
        selected_content: impl Into<String>,
    ) -> GatewayRequestResult<Self> {
        let run_id = run_id.into();
        let gateway_request_id = gateway_request_id.into();
        let request = InitialGatewayRequest::new(
            run_id.clone(),
            gateway_request_id.clone(),
            selected_content,
        )?;

        let schemas = [
            ToolSchema::GetCurrentDatetimeV1,
            ToolSchema::CreateLocalTaskV1,
        ];
        let expected_tool_contract_version = schemas[0].version();
        if expected_tool_contract_version != INITIAL_GATEWAY_TOOL_SET_VERSION
            || schemas
                .iter()
                .any(|schema| schema.version() != expected_tool_contract_version)
        {
            return Err(GatewayRequestError::ValidatorConfigurationFailed);
        }

        let mut registry = InMemoryToolRegistry::new();
        for schema in schemas.iter().copied() {
            registry
                .register(ToolDefinition::from_schema(schema))
                .map_err(|_| GatewayRequestError::ValidatorConfigurationFailed)?;
        }

        let validator = GatewayStreamValidator::new(
            run_id,
            gateway_request_id,
            schemas.iter().map(|schema| schema.name().to_owned()),
            expected_tool_contract_version,
        )
        .map_err(|_| GatewayRequestError::ValidatorConfigurationFailed)?;

        Ok(Self {
            request,
            validator,
            registry,
            approval_manager: InMemoryApprovalManager::new(),
            approval_audit: InMemoryApprovalAuditAdapter::new(),
            pending_approval_id: None,
            pending_function_call: None,
            local_schema_failed: false,
        })
    }

    #[must_use]
    pub fn request_bytes(&self) -> &[u8] {
        self.request.as_bytes()
    }

    #[must_use]
    pub fn status(&self) -> GatewayStreamStatus {
        if self.local_schema_failed {
            GatewayStreamStatus::Failed
        } else {
            self.validator.status()
        }
    }

    pub fn accept_frame(
        &mut self,
        frame: &[u8],
    ) -> InitialGatewayTurnResult<Option<InitialGatewayEvent>> {
        if self.local_schema_failed {
            return Err(InitialGatewayTurnError::Protocol(
                GatewayProtocolError::StreamAlreadyTerminal {
                    status: GatewayStreamStatus::Failed,
                },
            ));
        }

        let event = self
            .validator
            .accept_frame(frame)
            .map_err(InitialGatewayTurnError::Protocol)?;

        match event {
            ValidatedGatewayEvent::ResponseStarted {
                provider_response_id,
            } => Ok(Some(InitialGatewayEvent::ResponseStarted {
                provider_response_id,
            })),
            ValidatedGatewayEvent::OutputTextDelta { delta } => {
                Ok(Some(InitialGatewayEvent::OutputTextDelta { delta }))
            }
            ValidatedGatewayEvent::FunctionCallCompleted { call } => {
                match validate_function_call(call, &self.registry) {
                    Ok(call) => {
                        self.pending_function_call = Some(call);
                        Ok(None)
                    }
                    Err(error) => {
                        self.local_schema_failed = true;
                        Err(InitialGatewayTurnError::FunctionCallValidation(error))
                    }
                }
            }
            ValidatedGatewayEvent::ResponseCompleted => {
                Ok(Some(match self.pending_function_call.take() {
                    Some(call) => {
                        let input = PolicyInput::from_validated_call(call);
                        let decision = DeterministicPolicyEngine::new().evaluate(input);
                        match decision.outcome() {
                            PolicyOutcome::RequireApproval => {
                                let id = self
                                    .approval_manager
                                    .create_request(decision)
                                    .map_err(InitialGatewayTurnError::Approval)?;
                                let presentation = self
                                    .approval_manager
                                    .issue_presentation(id)
                                    .map_err(InitialGatewayTurnError::Approval)?;
                                self.pending_approval_id = Some(id);
                                InitialGatewayEvent::ApprovalPresentationReady { presentation }
                            }
                            PolicyOutcome::Allow | PolicyOutcome::Deny => {
                                InitialGatewayEvent::PolicyEvaluated { decision }
                            }
                        }
                    }
                    None => InitialGatewayEvent::ResponseCompleted,
                }))
            }
            ValidatedGatewayEvent::ResponseFailed { failure } => {
                self.pending_function_call.take();
                Ok(Some(InitialGatewayEvent::ResponseFailed { failure }))
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn resolve_approval_source_outcome(
        &mut self,
        outcome: TrustedApprovalSourceOutcome,
    ) -> InitialGatewayTurnResult<AuditedApprovalResolution> {
        let resolution = self
            .approval_manager
            .resolve_source_outcome(outcome)
            .map_err(InitialGatewayTurnError::Approval)?;
        self.pending_approval_id.take();
        self.record_terminal_approval(resolution)
    }

    pub fn cancel_pending_approval_for_run_termination(
        &mut self,
    ) -> InitialGatewayTurnResult<Option<AuditedApprovalResolution>> {
        let Some(id) = self.pending_approval_id else {
            return Ok(None);
        };

        let resolution = self
            .approval_manager
            .cancel_for_run_termination(id)
            .map_err(InitialGatewayTurnError::Approval)?;
        self.pending_approval_id.take();
        self.record_terminal_approval(resolution).map(Some)
    }

    fn record_terminal_approval(
        &mut self,
        resolution: ApprovalResolution,
    ) -> InitialGatewayTurnResult<AuditedApprovalResolution> {
        let receipt = self
            .approval_audit
            .record(&resolution)
            .map_err(InitialGatewayTurnError::ApprovalAudit)?;
        Ok(AuditedApprovalResolution {
            resolution,
            receipt,
        })
    }

    #[must_use]
    pub fn cancel(&mut self) -> bool {
        let cancelled = !self.local_schema_failed && self.validator.cancel();
        if cancelled {
            self.pending_function_call.take();
        }
        cancelled
    }
}

impl fmt::Debug for InitialGatewayTurn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InitialGatewayTurn")
            .field("request_body", &"[REDACTED]")
            .field("request_body_bytes", &self.request.as_bytes().len())
            .field("status", &self.status())
            .finish()
    }
}

pub type InitialGatewayTurnResult<T> = Result<T, InitialGatewayTurnError>;

pub struct AuditedApprovalResolution {
    resolution: ApprovalResolution,
    receipt: ApprovalAuditReceipt,
}

impl AuditedApprovalResolution {
    #[must_use]
    pub fn resolution(&self) -> &ApprovalResolution {
        &self.resolution
    }

    #[must_use]
    pub fn receipt(&self) -> ApprovalAuditReceipt {
        self.receipt
    }
}

impl fmt::Debug for AuditedApprovalResolution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuditedApprovalResolution")
            .field("resolution", &"[REDACTED]")
            .field("receipt", &self.receipt)
            .finish()
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum InitialGatewayTurnError {
    #[error("initial gateway operation is unavailable for this runtime profile")]
    WrongProfile,
    #[error("initial gateway protocol validation failed: {0}")]
    Protocol(GatewayProtocolError),
    #[error("initial gateway function call failed local schema validation: {0}")]
    FunctionCallValidation(FunctionCallValidationError),
    #[error("initial gateway approval binding failed: {0}")]
    Approval(ApprovalError),
    #[error("initial gateway approval audit failed: {0}")]
    ApprovalAudit(ApprovalAuditError),
}

pub enum InitialGatewayEvent {
    ResponseStarted { provider_response_id: String },
    OutputTextDelta { delta: String },
    PolicyEvaluated { decision: PolicyDecision },
    ApprovalPresentationReady { presentation: ApprovalPresentation },
    ResponseCompleted,
    ResponseFailed { failure: GatewayFailure },
}

pub(super) struct PersonalAssistantTextTurn {
    request: PersonalAssistantTextRequest,
    stream: PersonalAssistantStreamState,
}

#[derive(Clone)]
struct PersonalAssistantStreamState {
    validator: GatewayStreamValidator,
    provider_response_id: Option<String>,
    output: String,
    output_characters: usize,
    output_bytes: usize,
    locally_failed: bool,
}

impl PersonalAssistantTextTurn {
    pub(super) fn new_direct(
        run_id: String,
        request_id: String,
        text: String,
    ) -> PersonalAssistantTextTurnResult<Self> {
        let mut turn = Self::new(run_id, request_id, text)?;
        turn.request.body = crate::personal_assistant_direct::request_body()
            .map_err(|_| PersonalAssistantTextTurnError::SerializationFailed)?;
        Ok(turn)
    }
    pub(super) fn new(
        run_id: impl Into<String>,
        gateway_request_id: impl Into<String>,
        selected_content: impl Into<String>,
    ) -> PersonalAssistantTextTurnResult<Self> {
        let run_id = run_id.into();
        let gateway_request_id = gateway_request_id.into();
        let selected_content = selected_content.into();
        let request =
            PersonalAssistantTextRequest::new(&run_id, &gateway_request_id, &selected_content)?;
        let validator = GatewayStreamValidator::new(
            run_id,
            gateway_request_id,
            std::iter::empty::<String>(),
            PERSONAL_ASSISTANT_V0_TOOL_SET_VERSION,
        )
        .map_err(|_| PersonalAssistantTextTurnError::InvalidTrustedIdentity)?;

        Ok(Self {
            request,
            stream: PersonalAssistantStreamState {
                validator,
                provider_response_id: None,
                output: String::new(),
                output_characters: 0,
                output_bytes: 0,
                locally_failed: false,
            },
        })
    }

    #[must_use]
    pub(super) fn request_bytes(&self) -> &[u8] {
        self.request.as_bytes()
    }

    #[must_use]
    pub(super) fn status(&self) -> GatewayStreamStatus {
        if self.stream.locally_failed {
            GatewayStreamStatus::Failed
        } else {
            self.stream.validator.status()
        }
    }

    pub(super) fn accept_frame(
        &mut self,
        frame: &[u8],
    ) -> PersonalAssistantTextTurnResult<PersonalAssistantTextEvent> {
        if matches!(
            self.status(),
            GatewayStreamStatus::Completed
                | GatewayStreamStatus::Failed
                | GatewayStreamStatus::Cancelled
        ) {
            return Err(PersonalAssistantTextTurnError::ProtocolViolation);
        }
        if self.stream.validator.event_count() >= PERSONAL_ASSISTANT_V0_MAX_EVENTS {
            self.fail_locally();
            return Err(PersonalAssistantTextTurnError::LimitExceeded);
        }
        if frame.len() > MAX_GATEWAY_EVENT_BYTES {
            self.fail_locally();
            return Err(PersonalAssistantTextTurnError::LimitExceeded);
        }
        if personal_assistant_retry_after_field_is_present(frame) {
            self.fail_locally();
            return Err(PersonalAssistantTextTurnError::ProtocolViolation);
        }

        let mut next = self.stream.clone();
        let validated = match next.validator.accept_frame(frame) {
            Ok(event) => event,
            Err(error) => {
                let mapped = map_personal_assistant_protocol_error(&error);
                self.fail_locally();
                return Err(mapped);
            }
        };

        let event = match apply_personal_assistant_event(&mut next, validated) {
            Ok(event) => event,
            Err(error) => {
                self.fail_locally();
                return Err(error);
            }
        };
        self.stream = next;
        Ok(event)
    }

    #[must_use]
    pub(super) fn cancel(&mut self) -> bool {
        if self.stream.locally_failed {
            return false;
        }
        let cancelled = self.stream.validator.cancel();
        if cancelled {
            self.stream.provider_response_id.take();
            self.stream.output.clear();
            self.stream.output_characters = 0;
            self.stream.output_bytes = 0;
        }
        cancelled
    }

    fn fail_locally(&mut self) {
        self.stream.locally_failed = true;
        self.stream.provider_response_id.take();
        self.stream.output.clear();
        self.stream.output_characters = 0;
        self.stream.output_bytes = 0;
    }
}

fn personal_assistant_retry_after_field_is_present(frame: &[u8]) -> bool {
    let Ok(Value::Object(envelope)) = serde_json::from_slice(frame) else {
        return false;
    };
    let Some(Value::Object(event)) = envelope.get("event") else {
        return false;
    };
    matches!(
        event.get("type"),
        Some(Value::String(event_type)) if event_type == "response_failed"
    ) && event.contains_key("retry_after_ms")
}

impl fmt::Debug for PersonalAssistantTextTurn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantTextTurn")
            .field("request_body", &"[REDACTED]")
            .field("request_body_bytes", &self.request.as_bytes().len())
            .field("status", &self.status())
            .finish()
    }
}

pub(super) type PersonalAssistantTextTurnResult<T> = Result<T, PersonalAssistantTextTurnError>;

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub(super) enum PersonalAssistantTextTurnError {
    #[error("trusted Personal Assistant identity is invalid")]
    InvalidTrustedIdentity,
    #[error("Personal Assistant request serialization failed")]
    SerializationFailed,
    #[error("Personal Assistant operation is unavailable for this runtime profile")]
    WrongProfile,
    #[error("Personal Assistant protocol validation failed")]
    ProtocolViolation,
    #[error("Personal Assistant closed limit was exceeded")]
    LimitExceeded,
}

#[derive(Eq, PartialEq)]
pub(super) enum PersonalAssistantTextEvent {
    Started,
    TextDelta {
        delta: PersonalAssistantTextDelta,
    },
    Completed {
        final_answer: PersonalAssistantFinalAnswer,
    },
    Failed {
        code: PersonalAssistantFailureCode,
    },
}

impl fmt::Debug for PersonalAssistantTextEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Started => formatter.write_str("Started"),
            Self::TextDelta { .. } => formatter
                .debug_struct("TextDelta")
                .field("delta", &"[REDACTED]")
                .finish(),
            Self::Completed { .. } => formatter
                .debug_struct("Completed")
                .field("final_answer", &"[REDACTED]")
                .finish(),
            Self::Failed { code } => formatter
                .debug_struct("Failed")
                .field("code", code)
                .finish(),
        }
    }
}

#[derive(Eq, PartialEq)]
pub(super) struct PersonalAssistantTextDelta(String);

impl PersonalAssistantTextDelta {
    pub(super) fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Eq, PartialEq)]
pub(super) struct PersonalAssistantFinalAnswer(String);

impl PersonalAssistantFinalAnswer {
    pub(super) fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PersonalAssistantFailureCode {
    Unauthenticated,
    Forbidden,
    RateLimited,
    RequestRejected,
    ProviderUnavailable,
    ProviderTimeout,
    ProtocolViolation,
    LimitExceeded,
    Internal,
}

fn apply_personal_assistant_event(
    state: &mut PersonalAssistantStreamState,
    event: ValidatedGatewayEvent,
) -> PersonalAssistantTextTurnResult<PersonalAssistantTextEvent> {
    match event {
        ValidatedGatewayEvent::ResponseStarted {
            provider_response_id,
        } => {
            state.provider_response_id = Some(provider_response_id);
            Ok(PersonalAssistantTextEvent::Started)
        }
        ValidatedGatewayEvent::OutputTextDelta { delta } => {
            let delta_characters = delta.chars().count();
            if delta_characters > PERSONAL_ASSISTANT_V0_MAX_DELTA_CHARACTERS
                || delta.len() > PERSONAL_ASSISTANT_V0_MAX_DELTA_BYTES
            {
                return Err(PersonalAssistantTextTurnError::LimitExceeded);
            }
            let output_characters = state
                .output_characters
                .checked_add(delta_characters)
                .ok_or(PersonalAssistantTextTurnError::LimitExceeded)?;
            let output_bytes = state
                .output_bytes
                .checked_add(delta.len())
                .ok_or(PersonalAssistantTextTurnError::LimitExceeded)?;
            if output_characters > PERSONAL_ASSISTANT_V0_MAX_OUTPUT_CHARACTERS
                || output_bytes > PERSONAL_ASSISTANT_V0_MAX_OUTPUT_BYTES
            {
                return Err(PersonalAssistantTextTurnError::LimitExceeded);
            }
            state.output.push_str(&delta);
            state.output_characters = output_characters;
            state.output_bytes = output_bytes;
            Ok(PersonalAssistantTextEvent::TextDelta {
                delta: PersonalAssistantTextDelta(delta),
            })
        }
        ValidatedGatewayEvent::FunctionCallCompleted { .. } => {
            Err(PersonalAssistantTextTurnError::ProtocolViolation)
        }
        ValidatedGatewayEvent::ResponseCompleted => {
            state.provider_response_id.take();
            state.output_characters = 0;
            state.output_bytes = 0;
            Ok(PersonalAssistantTextEvent::Completed {
                final_answer: PersonalAssistantFinalAnswer(std::mem::take(&mut state.output)),
            })
        }
        ValidatedGatewayEvent::ResponseFailed { failure } => {
            if failure.retryable() || failure.retry_after_ms().is_some() {
                return Err(PersonalAssistantTextTurnError::ProtocolViolation);
            }
            let code = match failure.code() {
                super::gateway_protocol::GatewayFailureCode::Unauthenticated => {
                    PersonalAssistantFailureCode::Unauthenticated
                }
                super::gateway_protocol::GatewayFailureCode::Forbidden => {
                    PersonalAssistantFailureCode::Forbidden
                }
                super::gateway_protocol::GatewayFailureCode::RateLimited => {
                    PersonalAssistantFailureCode::RateLimited
                }
                super::gateway_protocol::GatewayFailureCode::RequestRejected => {
                    PersonalAssistantFailureCode::RequestRejected
                }
                super::gateway_protocol::GatewayFailureCode::ProviderUnavailable => {
                    PersonalAssistantFailureCode::ProviderUnavailable
                }
                super::gateway_protocol::GatewayFailureCode::ProviderTimeout => {
                    PersonalAssistantFailureCode::ProviderTimeout
                }
                super::gateway_protocol::GatewayFailureCode::ProtocolViolation => {
                    PersonalAssistantFailureCode::ProtocolViolation
                }
                super::gateway_protocol::GatewayFailureCode::LimitExceeded => {
                    PersonalAssistantFailureCode::LimitExceeded
                }
                super::gateway_protocol::GatewayFailureCode::Internal => {
                    PersonalAssistantFailureCode::Internal
                }
                super::gateway_protocol::GatewayFailureCode::Cancelled => {
                    return Err(PersonalAssistantTextTurnError::ProtocolViolation);
                }
            };
            state.provider_response_id.take();
            state.output.clear();
            state.output_characters = 0;
            state.output_bytes = 0;
            Ok(PersonalAssistantTextEvent::Failed { code })
        }
    }
}

fn map_personal_assistant_protocol_error(
    error: &GatewayProtocolError,
) -> PersonalAssistantTextTurnError {
    match error {
        GatewayProtocolError::EventTooLarge { .. }
        | GatewayProtocolError::EventLimitExceeded { .. }
        | GatewayProtocolError::AssistantOutputLimitExceeded { .. }
        | GatewayProtocolError::FunctionCallLimitExceeded { .. }
        | GatewayProtocolError::FunctionArgumentsTooLarge { .. } => {
            PersonalAssistantTextTurnError::LimitExceeded
        }
        GatewayProtocolError::RetryDelayTooLarge { .. }
        | GatewayProtocolError::InvalidExpectedRunId
        | GatewayProtocolError::InvalidExpectedGatewayRequestId
        | GatewayProtocolError::InvalidAllowedToolName
        | GatewayProtocolError::InvalidExpectedToolContractVersion
        | GatewayProtocolError::MalformedEvent
        | GatewayProtocolError::UnsupportedProtocolVersion { .. }
        | GatewayProtocolError::InvalidRunId
        | GatewayProtocolError::RunIdMismatch
        | GatewayProtocolError::InvalidGatewayRequestId
        | GatewayProtocolError::GatewayRequestIdMismatch
        | GatewayProtocolError::InvalidSequence { .. }
        | GatewayProtocolError::StreamAlreadyTerminal { .. }
        | GatewayProtocolError::EventBeforeResponseStarted
        | GatewayProtocolError::UnexpectedResponseStarted
        | GatewayProtocolError::InvalidProviderResponseId
        | GatewayProtocolError::EmptyOutputTextDelta
        | GatewayProtocolError::MixedResponseOutput
        | GatewayProtocolError::MissingResponseOutput
        | GatewayProtocolError::InvalidFunctionCallId
        | GatewayProtocolError::InvalidFunctionName
        | GatewayProtocolError::UnknownFunctionName
        | GatewayProtocolError::ToolContractVersionMismatch { .. }
        | GatewayProtocolError::MalformedFunctionArguments
        | GatewayProtocolError::FunctionArgumentsMustBeObject
        | GatewayProtocolError::DuplicateFunctionArgumentKey => {
            PersonalAssistantTextTurnError::ProtocolViolation
        }
    }
}

struct PersonalAssistantTextRequest {
    body: Vec<u8>,
}

impl PersonalAssistantTextRequest {
    fn new(
        run_id: &str,
        gateway_request_id: &str,
        selected_content: &str,
    ) -> PersonalAssistantTextTurnResult<Self> {
        if !is_valid_opaque_id(run_id) || !is_valid_opaque_id(gateway_request_id) {
            return Err(PersonalAssistantTextTurnError::InvalidTrustedIdentity);
        }
        if selected_content.chars().count() > PERSONAL_ASSISTANT_V0_MAX_INPUT_CHARACTERS
            || selected_content.len() > PERSONAL_ASSISTANT_V0_MAX_INPUT_BYTES
        {
            return Err(PersonalAssistantTextTurnError::LimitExceeded);
        }

        let wire = WirePersonalAssistantTextRequest {
            contract_version: PERSONAL_ASSISTANT_V0_CONTRACT_VERSION,
            protocol_version: GATEWAY_PROTOCOL_VERSION,
            run_id,
            gateway_request_id,
            request_kind: WirePersonalAssistantRequestKind::PersonalAssistantTextV0,
            model_turn: 1,
            retry_attempt: 0,
            agent: WirePersonalAssistantAgent {
                id: "personal-assistant",
                instruction_profile_id: PERSONAL_ASSISTANT_V0_INSTRUCTION_PROFILE_ID,
                instruction_profile_version: PERSONAL_ASSISTANT_V0_INSTRUCTION_PROFILE_VERSION,
                instructions: PERSONAL_ASSISTANT_V0_INSTRUCTIONS,
            },
            provider: WirePersonalAssistantProvider {
                profile_id: PERSONAL_ASSISTANT_V0_PROVIDER_PROFILE_ID,
                profile_version: PERSONAL_ASSISTANT_V0_PROVIDER_PROFILE_VERSION,
                provider: "openai",
                model: "gpt-5.6-luna",
                stream: true,
                store: false,
                background: false,
                reasoning_effort: "low",
                text_format: "text",
                text_verbosity: "low",
                truncation: "disabled",
                parallel_tool_calls: false,
                fallback: "none",
            },
            data_class: WirePersonalAssistantVersionedId {
                id: PERSONAL_ASSISTANT_V0_DATA_CLASS_ID,
                version: PERSONAL_ASSISTANT_V0_DATA_CLASS_VERSION,
            },
            input: WirePersonalAssistantInput {
                input_type: WirePersonalAssistantInputType::ApplicationOwnedSyntheticText,
                text: selected_content,
            },
            tool_set: WirePersonalAssistantToolSet {
                id: PERSONAL_ASSISTANT_V0_TOOL_SET_ID,
                version: PERSONAL_ASSISTANT_V0_TOOL_SET_VERSION,
                tools: [],
            },
            limits: WirePersonalAssistantLimits::current(),
        };
        let body = serde_json::to_vec(&wire)
            .map_err(|_| PersonalAssistantTextTurnError::SerializationFailed)?;
        if body.len() > MAX_GATEWAY_REQUEST_BYTES {
            return Err(PersonalAssistantTextTurnError::LimitExceeded);
        }
        Ok(Self { body })
    }

    fn as_bytes(&self) -> &[u8] {
        &self.body
    }
}

impl fmt::Debug for PersonalAssistantTextRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantTextRequest")
            .field("body", &"[REDACTED]")
            .field("body_bytes", &self.body.len())
            .finish()
    }
}

#[derive(Serialize)]
struct WirePersonalAssistantTextRequest<'a> {
    contract_version: u16,
    protocol_version: u16,
    run_id: &'a str,
    gateway_request_id: &'a str,
    request_kind: WirePersonalAssistantRequestKind,
    model_turn: u8,
    retry_attempt: u8,
    agent: WirePersonalAssistantAgent<'a>,
    provider: WirePersonalAssistantProvider<'a>,
    data_class: WirePersonalAssistantVersionedId<'a>,
    input: WirePersonalAssistantInput<'a>,
    tool_set: WirePersonalAssistantToolSet<'a>,
    limits: WirePersonalAssistantLimits,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum WirePersonalAssistantRequestKind {
    PersonalAssistantTextV0,
}

#[derive(Serialize)]
struct WirePersonalAssistantAgent<'a> {
    id: &'a str,
    instruction_profile_id: &'a str,
    instruction_profile_version: u16,
    instructions: &'a str,
}

#[derive(Serialize)]
struct WirePersonalAssistantProvider<'a> {
    profile_id: &'a str,
    profile_version: u16,
    provider: &'a str,
    model: &'a str,
    stream: bool,
    store: bool,
    background: bool,
    reasoning_effort: &'a str,
    text_format: &'a str,
    text_verbosity: &'a str,
    truncation: &'a str,
    parallel_tool_calls: bool,
    fallback: &'a str,
}

#[derive(Serialize)]
struct WirePersonalAssistantVersionedId<'a> {
    id: &'a str,
    version: u16,
}

#[derive(Serialize)]
struct WirePersonalAssistantInput<'a> {
    #[serde(rename = "type")]
    input_type: WirePersonalAssistantInputType,
    text: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum WirePersonalAssistantInputType {
    ApplicationOwnedSyntheticText,
}

#[derive(Serialize)]
struct WirePersonalAssistantToolSet<'a> {
    id: &'a str,
    version: u16,
    tools: [(); 0],
}

#[derive(Serialize)]
struct WirePersonalAssistantLimits {
    max_user_input_characters: usize,
    max_user_input_bytes: usize,
    max_model_turns: u8,
    max_gateway_requests: u8,
    max_retry_attempts: u8,
    max_function_calls: u8,
    max_gateway_request_bytes: usize,
    max_gateway_event_bytes: usize,
    max_gateway_events: usize,
    max_text_delta_characters: usize,
    max_text_delta_bytes: usize,
    max_assistant_output_characters: usize,
    max_assistant_output_bytes: usize,
    connect_timeout_ms: u128,
    stream_idle_timeout_ms: u128,
    provider_deadline_ms: u128,
    run_deadline_ms: u128,
}

impl WirePersonalAssistantLimits {
    fn current() -> Self {
        Self {
            max_user_input_characters: PERSONAL_ASSISTANT_V0_MAX_INPUT_CHARACTERS,
            max_user_input_bytes: PERSONAL_ASSISTANT_V0_MAX_INPUT_BYTES,
            max_model_turns: 1,
            max_gateway_requests: 1,
            max_retry_attempts: 0,
            max_function_calls: 0,
            max_gateway_request_bytes: MAX_GATEWAY_REQUEST_BYTES,
            max_gateway_event_bytes: MAX_GATEWAY_EVENT_BYTES,
            max_gateway_events: PERSONAL_ASSISTANT_V0_MAX_EVENTS,
            max_text_delta_characters: PERSONAL_ASSISTANT_V0_MAX_DELTA_CHARACTERS,
            max_text_delta_bytes: PERSONAL_ASSISTANT_V0_MAX_DELTA_BYTES,
            max_assistant_output_characters: PERSONAL_ASSISTANT_V0_MAX_OUTPUT_CHARACTERS,
            max_assistant_output_bytes: PERSONAL_ASSISTANT_V0_MAX_OUTPUT_BYTES,
            connect_timeout_ms: GATEWAY_CONNECT_TIMEOUT.as_millis(),
            stream_idle_timeout_ms: GATEWAY_STREAM_IDLE_TIMEOUT.as_millis(),
            provider_deadline_ms: PROVIDER_TURN_DEADLINE.as_millis(),
            run_deadline_ms: AGENT_RUN_DEADLINE.as_millis(),
        }
    }
}

impl fmt::Debug for InitialGatewayEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponseStarted {
                provider_response_id,
            } => formatter
                .debug_struct("ResponseStarted")
                .field("provider_response_id", provider_response_id)
                .finish(),
            Self::OutputTextDelta { .. } => formatter
                .debug_struct("OutputTextDelta")
                .field("delta", &"[REDACTED]")
                .finish(),
            Self::PolicyEvaluated { decision } => formatter
                .debug_struct("PolicyEvaluated")
                .field("decision", decision)
                .finish(),
            Self::ApprovalPresentationReady { presentation } => formatter
                .debug_struct("ApprovalPresentationReady")
                .field("presentation", presentation)
                .finish(),
            Self::ResponseCompleted => formatter.write_str("ResponseCompleted"),
            Self::ResponseFailed { failure } => formatter
                .debug_struct("ResponseFailed")
                .field("failure", failure)
                .finish(),
        }
    }
}

struct InitialGatewayRequest {
    body: Vec<u8>,
}

impl InitialGatewayRequest {
    fn new(
        run_id: impl Into<String>,
        gateway_request_id: impl Into<String>,
        selected_content: impl Into<String>,
    ) -> GatewayRequestResult<Self> {
        let run_id = run_id.into();
        if !is_valid_opaque_id(&run_id) {
            return Err(GatewayRequestError::InvalidRunId);
        }

        let gateway_request_id = gateway_request_id.into();
        if !is_valid_opaque_id(&gateway_request_id) {
            return Err(GatewayRequestError::InvalidGatewayRequestId);
        }

        let selected_content = selected_content.into();
        if selected_content.trim().is_empty() {
            return Err(GatewayRequestError::EmptySelectedContent);
        }
        if selected_content.len() > MAX_GATEWAY_REQUEST_BYTES {
            return Err(GatewayRequestError::SelectedContentTooLarge {
                maximum_bytes: MAX_GATEWAY_REQUEST_BYTES,
                actual_bytes: selected_content.len(),
            });
        }

        let wire = WireInitialGatewayRequest {
            protocol_version: GATEWAY_PROTOCOL_VERSION,
            run_id: &run_id,
            gateway_request_id: &gateway_request_id,
            request_kind: WireRequestKind::InitialUserTurn,
            model_turn: 1,
            retry_attempt: 0,
            input: WireInput {
                input_type: WireInputType::UserSelectedText,
                text: &selected_content,
            },
            tool_set: WireToolSet {
                id: INITIAL_GATEWAY_TOOL_SET_ID,
                version: INITIAL_GATEWAY_TOOL_SET_VERSION,
            },
            limits: WireLimits::current(),
        };
        let body =
            serde_json::to_vec(&wire).map_err(|_| GatewayRequestError::SerializationFailed)?;
        if body.len() > MAX_GATEWAY_REQUEST_BYTES {
            return Err(GatewayRequestError::RequestTooLarge {
                maximum_bytes: MAX_GATEWAY_REQUEST_BYTES,
                actual_bytes: body.len(),
            });
        }

        Ok(Self { body })
    }

    #[must_use]
    fn as_bytes(&self) -> &[u8] {
        &self.body
    }
}

impl fmt::Debug for InitialGatewayRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InitialGatewayRequest")
            .field("body", &"[REDACTED]")
            .field("body_bytes", &self.body.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum GatewayRequestError {
    #[error("gateway request run id is invalid")]
    InvalidRunId,
    #[error("gateway request id is invalid")]
    InvalidGatewayRequestId,
    #[error("selected content must contain a non-whitespace character")]
    EmptySelectedContent,
    #[error("selected content exceeds {maximum_bytes} bytes")]
    SelectedContentTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("serialized gateway request exceeds {maximum_bytes} bytes")]
    RequestTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("gateway request serialization failed")]
    SerializationFailed,
    #[error("gateway response validator configuration failed")]
    ValidatorConfigurationFailed,
}

#[derive(Serialize)]
struct WireInitialGatewayRequest<'a> {
    protocol_version: u16,
    run_id: &'a str,
    gateway_request_id: &'a str,
    request_kind: WireRequestKind,
    model_turn: u8,
    retry_attempt: u8,
    input: WireInput<'a>,
    tool_set: WireToolSet<'a>,
    limits: WireLimits,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum WireRequestKind {
    InitialUserTurn,
}

#[derive(Serialize)]
struct WireInput<'a> {
    #[serde(rename = "type")]
    input_type: WireInputType,
    text: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum WireInputType {
    UserSelectedText,
}

#[derive(Serialize)]
struct WireToolSet<'a> {
    id: &'a str,
    version: u16,
}

#[derive(Serialize)]
struct WireLimits {
    max_model_turns_per_run: u8,
    max_function_calls_per_run: u8,
    max_retry_attempts_per_run: u8,
    max_gateway_requests_per_run: u8,
    max_gateway_request_bytes: usize,
    max_gateway_event_bytes: usize,
    max_function_argument_bytes: usize,
    max_assistant_output_characters_per_turn: usize,
    max_gateway_events_per_turn: usize,
    max_retry_after_ms: u64,
    gateway_connect_timeout_ms: u128,
    gateway_stream_idle_timeout_ms: u128,
    provider_turn_deadline_ms: u128,
    agent_run_deadline_ms: u128,
}

impl WireLimits {
    fn current() -> Self {
        Self {
            max_model_turns_per_run: MAX_MODEL_TURNS_PER_RUN,
            max_function_calls_per_run: MAX_FUNCTION_CALLS_PER_RUN,
            max_retry_attempts_per_run: MAX_RETRY_ATTEMPTS_PER_RUN,
            max_gateway_requests_per_run: MAX_GATEWAY_REQUESTS_PER_RUN,
            max_gateway_request_bytes: MAX_GATEWAY_REQUEST_BYTES,
            max_gateway_event_bytes: MAX_GATEWAY_EVENT_BYTES,
            max_function_argument_bytes: MAX_FUNCTION_ARGUMENT_BYTES,
            max_assistant_output_characters_per_turn: MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
            max_gateway_events_per_turn: MAX_GATEWAY_EVENTS_PER_TURN,
            max_retry_after_ms: MAX_RETRY_AFTER_MS,
            gateway_connect_timeout_ms: GATEWAY_CONNECT_TIMEOUT.as_millis(),
            gateway_stream_idle_timeout_ms: GATEWAY_STREAM_IDLE_TIMEOUT.as_millis(),
            provider_turn_deadline_ms: PROVIDER_TURN_DEADLINE.as_millis(),
            agent_run_deadline_ms: AGENT_RUN_DEADLINE.as_millis(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "macos")]
    use std::error::Error;

    #[cfg(target_os = "macos")]
    use rfd::MessageDialogResult;
    use serde_json::{json, Value};

    #[cfg(target_os = "macos")]
    use super::AuditedApprovalResolution;
    use super::{
        GatewayRequestError, InitialGatewayRequest, PersonalAssistantFailureCode,
        PersonalAssistantTextEvent, PersonalAssistantTextRequest, PersonalAssistantTextTurn,
        PersonalAssistantTextTurnError, INITIAL_GATEWAY_TOOL_SET_ID,
        INITIAL_GATEWAY_TOOL_SET_VERSION, PERSONAL_ASSISTANT_V0_INSTRUCTIONS,
        PERSONAL_ASSISTANT_V0_MAX_EVENTS,
    };
    #[cfg(target_os = "macos")]
    use super::{InitialGatewayEvent, InitialGatewayTurn, InitialGatewayTurnError};
    use crate::agent::gateway_protocol::{
        GatewayStreamStatus, AGENT_RUN_DEADLINE, GATEWAY_CONNECT_TIMEOUT, GATEWAY_PROTOCOL_VERSION,
        GATEWAY_STREAM_IDLE_TIMEOUT, MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
        MAX_FUNCTION_ARGUMENT_BYTES, MAX_FUNCTION_CALLS_PER_RUN, MAX_GATEWAY_EVENTS_PER_TURN,
        MAX_GATEWAY_EVENT_BYTES, MAX_GATEWAY_REQUESTS_PER_RUN, MAX_GATEWAY_REQUEST_BYTES,
        MAX_MODEL_TURNS_PER_RUN, MAX_RETRY_AFTER_MS, MAX_RETRY_ATTEMPTS_PER_RUN,
        PROVIDER_TURN_DEADLINE,
    };
    #[cfg(target_os = "macos")]
    use crate::approvals::decision_source::test_outcome_from_dialog_result;
    #[cfg(target_os = "macos")]
    use crate::approvals::manager::{ApprovalError, ApprovalPresentation};
    #[cfg(target_os = "macos")]
    use crate::approvals::types::{
        ApprovalAction, ApprovalAuthenticationEvidence, ApprovalCancellationReason,
        ApprovalDisposition, ApprovalInteractionSource, ApprovalNativeButton, ApprovalRecipients,
        ApprovalResolution, ApprovalReversibility, ApprovalRisk, ApprovalSchedule,
        ApprovalSourceFailure, ApprovalTarget,
    };
    #[cfg(target_os = "macos")]
    use crate::audit::approval::ApprovalAuditError;
    #[cfg(target_os = "macos")]
    use crate::policy::types::{PolicyOutcome, PolicyReason};
    #[cfg(target_os = "macos")]
    use crate::tools::types::{PermissionKind, RiskClass};

    const RUN_ID: &str = "run-initial-request-1";
    const GATEWAY_REQUEST_ID: &str = "gateway-request-initial-1";
    const PA_RUN_ID: &str = "pa-v0-run-test-1";
    const PA_REQUEST_ID: &str = "pa-v0-request-test-1";
    const PA_FIXTURE: &str = "Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed.";
    #[cfg(target_os = "macos")]
    const CALL_ID: &str = "call-initial-request-1";
    #[cfg(target_os = "macos")]
    const TASK_TITLE: &str = "Plan tomorrow";

    fn personal_assistant_frame(sequence: u32, event: Value) -> Vec<u8> {
        json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": PA_RUN_ID,
            "gateway_request_id": PA_REQUEST_ID,
            "sequence": sequence,
            "event": event,
        })
        .to_string()
        .into_bytes()
    }

    fn started_personal_assistant_turn(
    ) -> Result<PersonalAssistantTextTurn, PersonalAssistantTextTurnError> {
        let mut turn = PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, PA_FIXTURE)?;
        let started = turn.accept_frame(&personal_assistant_frame(
            0,
            json!({
                "type": "response_started",
                "provider_response_id": "pa-provider-response-1",
            }),
        ))?;
        assert!(matches!(started, PersonalAssistantTextEvent::Started));
        Ok(turn)
    }

    #[cfg(target_os = "macos")]
    fn frame(sequence: u32, event: Value) -> Vec<u8> {
        json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": RUN_ID,
            "gateway_request_id": GATEWAY_REQUEST_ID,
            "sequence": sequence,
            "event": event,
        })
        .to_string()
        .into_bytes()
    }

    #[cfg(target_os = "macos")]
    fn turn_with_approval_presentation(
    ) -> Result<(InitialGatewayTurn, ApprovalPresentation), Box<dyn Error>> {
        let mut turn = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, "Plan my day")?;
        let started = frame(
            0,
            json!({
                "type": "response_started",
                "provider_response_id": "provider-response-initial-1",
            }),
        );
        if !matches!(
            turn.accept_frame(&started)?,
            Some(InitialGatewayEvent::ResponseStarted { .. })
        ) {
            return Err("expected response start".into());
        }

        let function_call = frame(
            1,
            json!({
                "type": "function_call_completed",
                "call_id": CALL_ID,
                "name": "create_local_task",
                "tool_contract_version": INITIAL_GATEWAY_TOOL_SET_VERSION,
                "arguments_json": format!(r#"{{"title":"{TASK_TITLE}"}}"#),
            }),
        );
        if turn.accept_frame(&function_call)?.is_some() {
            return Err("validated function call should remain pending".into());
        }

        let completed = frame(2, json!({ "type": "response_completed" }));
        match turn.accept_frame(&completed)? {
            Some(InitialGatewayEvent::ApprovalPresentationReady { presentation }) => {
                Ok((turn, presentation))
            }
            _ => Err("expected terminal approval presentation".into()),
        }
    }

    #[cfg(target_os = "macos")]
    fn assert_exact_resolution(
        resolution: &ApprovalResolution,
        disposition: ApprovalDisposition,
        native_button: Option<ApprovalNativeButton>,
        source_failure: Option<ApprovalSourceFailure>,
    ) -> Result<(), &'static str> {
        assert_eq!(resolution.id().value(), 1);
        assert_eq!(resolution.disposition(), disposition);
        assert_eq!(resolution.run_id(), RUN_ID);
        assert_eq!(resolution.gateway_request_id(), GATEWAY_REQUEST_ID);
        assert_eq!(resolution.call_id(), CALL_ID);
        assert_eq!(resolution.tool_name(), "create_local_task");
        assert_eq!(
            resolution.tool_contract_version(),
            INITIAL_GATEWAY_TOOL_SET_VERSION
        );
        assert_eq!(resolution.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(resolution.required_permission(), PermissionKind::None);
        assert_eq!(resolution.policy_outcome(), PolicyOutcome::RequireApproval);
        assert_eq!(
            resolution.policy_reason(),
            PolicyReason::ReversibleRequiresApproval
        );

        let preview = resolution
            .preview()
            .ok_or("approval resolution must retain its registered preview")?;
        assert_eq!(preview.action(), ApprovalAction::CreateLocalTask);
        assert_eq!(preview.target(), ApprovalTarget::LocalTaskList);
        assert_eq!(preview.affected_data().value(), TASK_TITLE);
        assert_eq!(preview.schedule(), ApprovalSchedule::NotScheduled);
        assert_eq!(preview.recipients(), ApprovalRecipients::None);
        assert_eq!(preview.reversibility(), ApprovalReversibility::Reversible);
        assert_eq!(preview.required_permission(), PermissionKind::None);
        assert_eq!(preview.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(preview.risk(), ApprovalRisk::CreatesLocalTask);

        let evidence = resolution
            .interaction_evidence()
            .ok_or("native outcome must retain interaction evidence")?;
        assert_eq!(
            evidence.source(),
            ApprovalInteractionSource::MacOsNativeDialog
        );
        assert_eq!(evidence.native_button(), native_button);
        assert_eq!(
            evidence.authentication(),
            ApprovalAuthenticationEvidence::NotEvaluated
        );
        assert_eq!(evidence.source_failure(), source_failure);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn assert_exact_audit_binding(
        turn: &InitialGatewayTurn,
        audited: &AuditedApprovalResolution,
    ) -> Result<(), &'static str> {
        let resolution = audited.resolution();
        let receipt = audited.receipt();
        assert_eq!(receipt.sequence().value(), 1);

        let [record] = turn.approval_audit.records() else {
            return Err("expected one turn-owned approval audit record");
        };
        assert_eq!(record.sequence(), receipt.sequence());
        assert_eq!(record.approval_id(), resolution.id());
        assert_eq!(record.run_id(), resolution.run_id());
        assert_eq!(record.gateway_request_id(), resolution.gateway_request_id());
        assert_eq!(record.call_id(), resolution.call_id());
        assert_eq!(record.tool_name(), resolution.tool_name());
        assert_eq!(
            record.tool_contract_version(),
            resolution.tool_contract_version()
        );
        assert_eq!(record.risk_class(), resolution.risk_class());
        assert_eq!(
            record.required_permission(),
            resolution.required_permission()
        );
        assert_eq!(record.policy_outcome(), resolution.policy_outcome());
        assert_eq!(record.policy_reason(), resolution.policy_reason());
        assert_eq!(record.disposition(), resolution.disposition());
        assert_eq!(
            record.interaction_evidence(),
            resolution.interaction_evidence()
        );
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn resolves_each_closed_native_outcome_through_the_turn_owned_manager(
    ) -> Result<(), Box<dyn Error>> {
        let cases = [
            (
                MessageDialogResult::Custom("Approve".to_owned()),
                ApprovalDisposition::Approved,
                Some(ApprovalNativeButton::Approve),
                None,
            ),
            (
                MessageDialogResult::Custom("Reject".to_owned()),
                ApprovalDisposition::Rejected,
                Some(ApprovalNativeButton::Reject),
                None,
            ),
            (
                MessageDialogResult::Custom("Edit".to_owned()),
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::EditRequested),
                Some(ApprovalNativeButton::Edit),
                None,
            ),
            (
                MessageDialogResult::Cancel,
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::NativeNoDecision),
                None,
                None,
            ),
            (
                MessageDialogResult::Ok,
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed),
                None,
                Some(ApprovalSourceFailure::UnexpectedDialogResult),
            ),
        ];

        for (dialog_result, disposition, native_button, source_failure) in cases {
            let (mut turn, presentation) = turn_with_approval_presentation()?;
            let outcome = test_outcome_from_dialog_result(presentation, dialog_result);
            let outcome_debug = format!("{outcome:?}");
            for sensitive in [RUN_ID, GATEWAY_REQUEST_ID, CALL_ID, TASK_TITLE] {
                assert!(!outcome_debug.contains(sensitive));
            }
            assert!(outcome_debug.contains("[REDACTED]"));

            let audited = turn.resolve_approval_source_outcome(outcome)?;
            assert_exact_resolution(
                audited.resolution(),
                disposition,
                native_button,
                source_failure,
            )?;
            assert_exact_audit_binding(&turn, &audited)?;

            let resolution_debug = format!("{audited:?}");
            for sensitive in [RUN_ID, GATEWAY_REQUEST_ID, CALL_ID, TASK_TITLE] {
                assert!(!resolution_debug.contains(sensitive));
            }
            assert!(resolution_debug.contains("[REDACTED]"));
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn rejects_a_foreign_outcome_without_consuming_the_recipient_pending_subject(
    ) -> Result<(), Box<dyn Error>> {
        let (_foreign_turn, foreign_presentation) = turn_with_approval_presentation()?;
        let foreign_outcome = test_outcome_from_dialog_result(
            foreign_presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );

        let (mut recipient_turn, recipient_presentation) = turn_with_approval_presentation()?;
        let recipient_outcome = test_outcome_from_dialog_result(
            recipient_presentation,
            MessageDialogResult::Custom("Reject".to_owned()),
        );

        assert!(matches!(
            recipient_turn.resolve_approval_source_outcome(foreign_outcome),
            Err(InitialGatewayTurnError::Approval(
                ApprovalError::ManagerInstanceMismatch
            ))
        ));

        let audited = recipient_turn.resolve_approval_source_outcome(recipient_outcome)?;
        assert_exact_resolution(
            audited.resolution(),
            ApprovalDisposition::Rejected,
            Some(ApprovalNativeButton::Reject),
            None,
        )?;
        assert_exact_audit_binding(&recipient_turn, &audited)?;
        assert!(recipient_turn
            .cancel_pending_approval_for_run_termination()?
            .is_none());
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn run_termination_consumes_the_subject_and_rejects_a_late_native_outcome(
    ) -> Result<(), Box<dyn Error>> {
        let (mut turn, presentation) = turn_with_approval_presentation()?;
        let approval_id = presentation.id();
        let late_outcome = test_outcome_from_dialog_result(
            presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );

        let audited = turn
            .cancel_pending_approval_for_run_termination()?
            .ok_or("run termination must consume the pending approval")?;
        let resolution = audited.resolution();
        assert_eq!(resolution.id(), approval_id);
        assert_eq!(
            resolution.disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );
        assert!(resolution.interaction_evidence().is_none());
        assert_exact_audit_binding(&turn, &audited)?;
        assert_eq!(turn.status(), GatewayStreamStatus::Completed);

        assert!(matches!(
            turn.resolve_approval_source_outcome(late_outcome),
            Err(InitialGatewayTurnError::Approval(
                ApprovalError::AlreadyConsumed(actual)
            )) if actual == approval_id.value()
        ));
        assert_eq!(turn.status(), GatewayStreamStatus::Completed);
        assert!(turn
            .cancel_pending_approval_for_run_termination()?
            .is_none());
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn audit_failure_after_manager_success_returns_no_resolution_or_stale_pending_subject(
    ) -> Result<(), Box<dyn Error>> {
        let (mut seed_turn, _) = turn_with_approval_presentation()?;
        let seed = seed_turn
            .cancel_pending_approval_for_run_termination()?
            .ok_or("seed turn must produce an audited resolution")?;

        let (mut turn, presentation) = turn_with_approval_presentation()?;
        let approval_id = presentation.id();
        let late_outcome = test_outcome_from_dialog_result(
            presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );
        assert_eq!(
            turn.approval_audit
                .record(seed.resolution())?
                .sequence()
                .value(),
            1
        );

        let error = turn
            .cancel_pending_approval_for_run_termination()
            .err()
            .ok_or("duplicate audit evidence must fail closed")?;
        assert_eq!(
            error,
            InitialGatewayTurnError::ApprovalAudit(ApprovalAuditError::DuplicateResolution)
        );
        assert_eq!(turn.approval_audit.records().len(), 1);
        assert!(turn
            .cancel_pending_approval_for_run_termination()?
            .is_none());
        assert!(matches!(
            turn.resolve_approval_source_outcome(late_outcome),
            Err(InitialGatewayTurnError::Approval(
                ApprovalError::AlreadyConsumed(actual)
            )) if actual == approval_id.value()
        ));

        let error_debug = format!("{error:?} {error}");
        for sensitive in [RUN_ID, GATEWAY_REQUEST_ID, CALL_ID, TASK_TITLE] {
            assert!(!error_debug.contains(sensitive));
        }
        Ok(())
    }

    #[test]
    fn serializes_the_exact_closed_initial_request() -> Result<(), GatewayRequestError> {
        let request = InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, "Plan my day")?;
        let actual: Value = serde_json::from_slice(request.as_bytes())
            .map_err(|_| GatewayRequestError::SerializationFailed)?;

        assert_eq!(
            actual,
            json!({
                "protocol_version": GATEWAY_PROTOCOL_VERSION,
                "run_id": RUN_ID,
                "gateway_request_id": GATEWAY_REQUEST_ID,
                "request_kind": "initial_user_turn",
                "model_turn": 1,
                "retry_attempt": 0,
                "input": {
                    "type": "user_selected_text",
                    "text": "Plan my day",
                },
                "tool_set": {
                    "id": INITIAL_GATEWAY_TOOL_SET_ID,
                    "version": INITIAL_GATEWAY_TOOL_SET_VERSION,
                },
                "limits": {
                    "max_model_turns_per_run": MAX_MODEL_TURNS_PER_RUN,
                    "max_function_calls_per_run": MAX_FUNCTION_CALLS_PER_RUN,
                    "max_retry_attempts_per_run": MAX_RETRY_ATTEMPTS_PER_RUN,
                    "max_gateway_requests_per_run": MAX_GATEWAY_REQUESTS_PER_RUN,
                    "max_gateway_request_bytes": MAX_GATEWAY_REQUEST_BYTES,
                    "max_gateway_event_bytes": MAX_GATEWAY_EVENT_BYTES,
                    "max_function_argument_bytes": MAX_FUNCTION_ARGUMENT_BYTES,
                    "max_assistant_output_characters_per_turn": MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
                    "max_gateway_events_per_turn": MAX_GATEWAY_EVENTS_PER_TURN,
                    "max_retry_after_ms": MAX_RETRY_AFTER_MS,
                    "gateway_connect_timeout_ms": GATEWAY_CONNECT_TIMEOUT.as_millis(),
                    "gateway_stream_idle_timeout_ms": GATEWAY_STREAM_IDLE_TIMEOUT.as_millis(),
                    "provider_turn_deadline_ms": PROVIDER_TURN_DEADLINE.as_millis(),
                    "agent_run_deadline_ms": AGENT_RUN_DEADLINE.as_millis(),
                },
            })
        );
        assert_eq!(
            request.as_bytes(),
            br#"{"protocol_version":1,"run_id":"run-initial-request-1","gateway_request_id":"gateway-request-initial-1","request_kind":"initial_user_turn","model_turn":1,"retry_attempt":0,"input":{"type":"user_selected_text","text":"Plan my day"},"tool_set":{"id":"cortexa_desktop_mvp","version":1},"limits":{"max_model_turns_per_run":2,"max_function_calls_per_run":1,"max_retry_attempts_per_run":1,"max_gateway_requests_per_run":3,"max_gateway_request_bytes":65536,"max_gateway_event_bytes":16384,"max_function_argument_bytes":8192,"max_assistant_output_characters_per_turn":8192,"max_gateway_events_per_turn":256,"max_retry_after_ms":60000,"gateway_connect_timeout_ms":10000,"gateway_stream_idle_timeout_ms":20000,"provider_turn_deadline_ms":60000,"agent_run_deadline_ms":120000}}"#,
        );
        assert!(request.as_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);
        Ok(())
    }

    #[test]
    fn serializes_deterministically_and_preserves_selected_text() -> Result<(), GatewayRequestError>
    {
        let selected_content = "Review cafe notes\nThen plan tomorrow";
        let first = InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, selected_content)?;
        let second = InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, selected_content)?;
        let parsed: Value = serde_json::from_slice(first.as_bytes())
            .map_err(|_| GatewayRequestError::SerializationFailed)?;

        assert_eq!(first.as_bytes(), second.as_bytes());
        assert_eq!(parsed["input"]["text"], selected_content);
        Ok(())
    }

    #[test]
    fn rejects_invalid_identity_and_empty_content() {
        assert!(matches!(
            InitialGatewayRequest::new("", GATEWAY_REQUEST_ID, "content"),
            Err(GatewayRequestError::InvalidRunId)
        ));
        assert!(matches!(
            InitialGatewayRequest::new(RUN_ID, "gateway request", "content"),
            Err(GatewayRequestError::InvalidGatewayRequestId)
        ));
        assert!(matches!(
            InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, " \n\t"),
            Err(GatewayRequestError::EmptySelectedContent)
        ));
    }

    #[test]
    fn enforces_source_and_post_serialization_byte_limits() {
        let oversized_source = "x".repeat(MAX_GATEWAY_REQUEST_BYTES + 1);
        assert!(matches!(
            InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, oversized_source),
            Err(GatewayRequestError::SelectedContentTooLarge {
                maximum_bytes,
                actual_bytes,
            }) if maximum_bytes == MAX_GATEWAY_REQUEST_BYTES
                && actual_bytes == MAX_GATEWAY_REQUEST_BYTES + 1
        ));

        let escaping_expansion = "\"".repeat(MAX_GATEWAY_REQUEST_BYTES / 2);
        assert!(matches!(
            InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, escaping_expansion),
            Err(GatewayRequestError::RequestTooLarge {
                maximum_bytes: MAX_GATEWAY_REQUEST_BYTES,
                actual_bytes,
            }) if actual_bytes > MAX_GATEWAY_REQUEST_BYTES
        ));
    }

    #[test]
    fn redacts_content_from_debug_and_errors() -> Result<(), String> {
        let sentinel = "private-initial-request-sentinel";
        let request = InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, sentinel)
            .map_err(|error| format!("unexpected request error: {error}"))?;
        let debug = format!("{request:?}");
        assert!(!debug.contains(sentinel));
        assert!(debug.contains("[REDACTED]"));

        let oversized = format!("{sentinel}{}", "\"".repeat(MAX_GATEWAY_REQUEST_BYTES / 2));
        let error = InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, oversized)
            .err()
            .ok_or_else(|| "escaped request should exceed the byte limit".to_owned())?;
        let error_text = format!("{error:?} {error}");
        assert!(!error_text.contains(sentinel));
        Ok(())
    }

    #[test]
    fn omits_provider_credentials_schemas_and_authority_fields() -> Result<(), GatewayRequestError>
    {
        let request = InitialGatewayRequest::new(RUN_ID, GATEWAY_REQUEST_ID, "content")?;
        let parsed: Value = serde_json::from_slice(request.as_bytes())
            .map_err(|_| GatewayRequestError::SerializationFailed)?;
        let object = parsed
            .as_object()
            .ok_or(GatewayRequestError::SerializationFailed)?;

        for forbidden in [
            "gateway_url",
            "provider",
            "model",
            "authorization",
            "credential",
            "principal",
            "tools",
            "tool_schema",
            "policy",
            "approval",
            "audit",
            "execute",
        ] {
            assert!(!object.contains_key(forbidden));
        }
        assert_eq!(object.len(), 9);
        Ok(())
    }

    #[test]
    fn personal_assistant_request_is_exact_deterministic_and_empty_tool() -> Result<(), String> {
        assert_eq!(
            PERSONAL_ASSISTANT_V0_INSTRUCTIONS,
            "Act as Cortexa's Personal Assistant for one bounded synthetic text request. Answer only from the supplied application-owned synthetic text and return concise plain text. Do not call or propose tools, access files, memory, retrieval, networks, or devices, delegate, schedule, persist, approve, execute, retry, or claim any action or context not supplied."
        );
        let first = PersonalAssistantTextRequest::new(PA_RUN_ID, PA_REQUEST_ID, PA_FIXTURE)
            .map_err(|error| error.to_string())?;
        let second = PersonalAssistantTextRequest::new(PA_RUN_ID, PA_REQUEST_ID, PA_FIXTURE)
            .map_err(|error| error.to_string())?;
        let actual: Value =
            serde_json::from_slice(first.as_bytes()).map_err(|error| error.to_string())?;

        assert_eq!(first.as_bytes(), second.as_bytes());
        assert_eq!(
            actual,
            json!({
                "contract_version": 1,
                "protocol_version": 1,
                "run_id": PA_RUN_ID,
                "gateway_request_id": PA_REQUEST_ID,
                "request_kind": "personal_assistant_text_v0",
                "model_turn": 1,
                "retry_attempt": 0,
                "agent": {
                    "id": "personal-assistant",
                    "instruction_profile_id": "personal-assistant-text-v0",
                    "instruction_profile_version": 1,
                    "instructions": PERSONAL_ASSISTANT_V0_INSTRUCTIONS,
                },
                "provider": {
                    "profile_id": "openai-cloudflare-personal-assistant-v0",
                    "profile_version": 1,
                    "provider": "openai",
                    "model": "gpt-5.6-luna",
                    "stream": true,
                    "store": false,
                    "background": false,
                    "reasoning_effort": "low",
                    "text_format": "text",
                    "text_verbosity": "low",
                    "truncation": "disabled",
                    "parallel_tool_calls": false,
                    "fallback": "none",
                },
                "data_class": {
                    "id": "application-owned-synthetic",
                    "version": 1,
                },
                "input": {
                    "type": "application_owned_synthetic_text",
                    "text": PA_FIXTURE,
                },
                "tool_set": {
                    "id": "empty",
                    "version": 1,
                    "tools": [],
                },
                "limits": {
                    "max_user_input_characters": 4096,
                    "max_user_input_bytes": 16384,
                    "max_model_turns": 1,
                    "max_gateway_requests": 1,
                    "max_retry_attempts": 0,
                    "max_function_calls": 0,
                    "max_gateway_request_bytes": 65536,
                    "max_gateway_event_bytes": 16384,
                    "max_gateway_events": 128,
                    "max_text_delta_characters": 1024,
                    "max_text_delta_bytes": 4096,
                    "max_assistant_output_characters": 8192,
                    "max_assistant_output_bytes": 32768,
                    "connect_timeout_ms": 10000,
                    "stream_idle_timeout_ms": 20000,
                    "provider_deadline_ms": 60000,
                    "run_deadline_ms": 120000,
                },
            })
        );
        assert_eq!(actual.as_object().map(|object| object.len()), Some(13));
        assert!(first.as_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);
        Ok(())
    }

    #[test]
    fn personal_assistant_request_enforces_identity_scalar_and_byte_bounds() {
        assert_eq!(
            PersonalAssistantTextTurn::new("bad run", PA_REQUEST_ID, PA_FIXTURE).err(),
            Some(PersonalAssistantTextTurnError::InvalidTrustedIdentity)
        );
        assert_eq!(
            PersonalAssistantTextTurn::new(PA_RUN_ID, "bad request", PA_FIXTURE).err(),
            Some(PersonalAssistantTextTurnError::InvalidTrustedIdentity)
        );

        let exact_multibyte = "🦀".repeat(4_096);
        assert_eq!(exact_multibyte.len(), 16_384);
        assert!(PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, exact_multibyte).is_ok());
        assert_eq!(
            PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, "x".repeat(4_097),).err(),
            Some(PersonalAssistantTextTurnError::LimitExceeded)
        );
        assert_eq!(
            PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, "🦀".repeat(4_097),).err(),
            Some(PersonalAssistantTextTurnError::LimitExceeded)
        );
    }

    #[test]
    fn personal_assistant_success_concatenates_ordered_text_into_exact_final_answer(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let mut turn = started_personal_assistant_turn()?;
        let first = turn.accept_frame(&personal_assistant_frame(
            1,
            json!({ "type": "output_text_delta", "delta": "Board " }),
        ))?;
        let second = turn.accept_frame(&personal_assistant_frame(
            2,
            json!({ "type": "output_text_delta", "delta": "ready" }),
        ))?;
        let completed = turn.accept_frame(&personal_assistant_frame(
            3,
            json!({ "type": "response_completed" }),
        ))?;

        let PersonalAssistantTextEvent::TextDelta { delta } = first else {
            return Err(PersonalAssistantTextTurnError::ProtocolViolation);
        };
        assert_eq!(delta.into_inner(), "Board ");
        let PersonalAssistantTextEvent::TextDelta { delta } = second else {
            return Err(PersonalAssistantTextTurnError::ProtocolViolation);
        };
        assert_eq!(delta.into_inner(), "ready");
        let PersonalAssistantTextEvent::Completed { final_answer } = completed else {
            return Err(PersonalAssistantTextTurnError::ProtocolViolation);
        };
        assert_eq!(final_answer.into_inner(), "Board ready");
        assert_eq!(turn.status(), GatewayStreamStatus::Completed);
        assert_eq!(
            turn.accept_frame(&personal_assistant_frame(
                4,
                json!({ "type": "output_text_delta", "delta": "late" }),
            )),
            Err(PersonalAssistantTextTurnError::ProtocolViolation)
        );
        Ok(())
    }

    #[test]
    fn personal_assistant_accepts_only_closed_nonretryable_failure_codes(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let cases = [
            (
                "unauthenticated",
                PersonalAssistantFailureCode::Unauthenticated,
            ),
            ("forbidden", PersonalAssistantFailureCode::Forbidden),
            ("rate_limited", PersonalAssistantFailureCode::RateLimited),
            (
                "request_rejected",
                PersonalAssistantFailureCode::RequestRejected,
            ),
            (
                "provider_unavailable",
                PersonalAssistantFailureCode::ProviderUnavailable,
            ),
            (
                "provider_timeout",
                PersonalAssistantFailureCode::ProviderTimeout,
            ),
            (
                "protocol_violation",
                PersonalAssistantFailureCode::ProtocolViolation,
            ),
            (
                "limit_exceeded",
                PersonalAssistantFailureCode::LimitExceeded,
            ),
            ("internal", PersonalAssistantFailureCode::Internal),
        ];

        for (wire_code, expected) in cases {
            let mut turn = started_personal_assistant_turn()?;
            let event = turn.accept_frame(&personal_assistant_frame(
                1,
                json!({
                    "type": "response_failed",
                    "code": wire_code,
                    "retryable": false,
                }),
            ))?;
            assert!(matches!(
                event,
                PersonalAssistantTextEvent::Failed { code } if code == expected
            ));
            assert_eq!(turn.status(), GatewayStreamStatus::Failed);
        }
        Ok(())
    }

    #[test]
    fn personal_assistant_rejects_retry_cancel_and_retry_metadata_as_terminal_protocol_failure(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let cases = [
            json!({
                "type": "response_failed",
                "code": "provider_timeout",
                "retryable": true,
            }),
            json!({
                "type": "response_failed",
                "code": "rate_limited",
                "retryable": false,
                "retry_after_ms": 1,
            }),
            json!({
                "type": "response_failed",
                "code": "rate_limited",
                "retryable": false,
                "retry_after_ms": null,
            }),
            json!({
                "type": "response_failed",
                "code": "cancelled",
                "retryable": false,
            }),
        ];
        for event in cases {
            let mut turn = started_personal_assistant_turn()?;
            assert_eq!(
                turn.accept_frame(&personal_assistant_frame(1, event)),
                Err(PersonalAssistantTextTurnError::ProtocolViolation)
            );
            assert_eq!(turn.status(), GatewayStreamStatus::Failed);
            assert_eq!(
                turn.accept_frame(&personal_assistant_frame(
                    2,
                    json!({ "type": "response_completed" }),
                )),
                Err(PersonalAssistantTextTurnError::ProtocolViolation)
            );
        }
        Ok(())
    }

    #[test]
    fn personal_assistant_cancellation_is_local_idempotent_and_terminal(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let mut awaiting = PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, PA_FIXTURE)?;
        assert!(awaiting.cancel());
        assert!(!awaiting.cancel());
        assert_eq!(awaiting.status(), GatewayStreamStatus::Cancelled);
        assert_eq!(
            awaiting.accept_frame(&personal_assistant_frame(
                0,
                json!({
                    "type": "response_started",
                    "provider_response_id": "late-response",
                }),
            )),
            Err(PersonalAssistantTextTurnError::ProtocolViolation)
        );

        let mut streaming = started_personal_assistant_turn()?;
        assert!(streaming.cancel());
        assert!(!streaming.cancel());
        assert_eq!(streaming.status(), GatewayStreamStatus::Cancelled);
        Ok(())
    }

    #[test]
    fn personal_assistant_rejects_malformed_identity_sequence_state_and_unknown_fields(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let cases = [
            b"{".to_vec(),
            json!({
                "protocol_version": 2,
                "run_id": PA_RUN_ID,
                "gateway_request_id": PA_REQUEST_ID,
                "sequence": 0,
                "event": {
                    "type": "response_started",
                    "provider_response_id": "response-1",
                },
            })
            .to_string()
            .into_bytes(),
            json!({
                "protocol_version": 1,
                "run_id": "wrong-run",
                "gateway_request_id": PA_REQUEST_ID,
                "sequence": 0,
                "event": {
                    "type": "response_started",
                    "provider_response_id": "response-1",
                },
            })
            .to_string()
            .into_bytes(),
            json!({
                "protocol_version": 1,
                "run_id": PA_RUN_ID,
                "gateway_request_id": "wrong-request",
                "sequence": 0,
                "event": {
                    "type": "response_started",
                    "provider_response_id": "response-1",
                },
            })
            .to_string()
            .into_bytes(),
            personal_assistant_frame(
                1,
                json!({
                    "type": "response_started",
                    "provider_response_id": "response-1",
                }),
            ),
            json!({
                "protocol_version": 1,
                "run_id": PA_RUN_ID,
                "gateway_request_id": PA_REQUEST_ID,
                "sequence": 0,
                "unknown": true,
                "event": {
                    "type": "response_started",
                    "provider_response_id": "response-1",
                },
            })
            .to_string()
            .into_bytes(),
            personal_assistant_frame(
                0,
                json!({
                    "type": "response_started",
                    "provider_response_id": "response-1",
                    "unknown": true,
                }),
            ),
            personal_assistant_frame(0, json!({ "type": "output_text_delta", "delta": "early" })),
        ];

        for frame in cases {
            let mut turn = PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, PA_FIXTURE)?;
            assert!(matches!(
                turn.accept_frame(&frame),
                Err(PersonalAssistantTextTurnError::ProtocolViolation)
                    | Err(PersonalAssistantTextTurnError::LimitExceeded)
            ));
            assert_eq!(turn.status(), GatewayStreamStatus::Failed);
        }
        Ok(())
    }

    #[test]
    fn personal_assistant_rejects_duplicate_start_empty_or_missing_output_and_oversize_frames(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let invalid_streaming_events = [
            json!({
                "type": "response_started",
                "provider_response_id": "duplicate-response",
            }),
            json!({ "type": "output_text_delta", "delta": "" }),
            json!({ "type": "response_completed" }),
        ];
        for event in invalid_streaming_events {
            let mut turn = started_personal_assistant_turn()?;
            assert_eq!(
                turn.accept_frame(&personal_assistant_frame(1, event)),
                Err(PersonalAssistantTextTurnError::ProtocolViolation)
            );
            assert_eq!(turn.status(), GatewayStreamStatus::Failed);
        }

        let mut invalid_provider =
            PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, PA_FIXTURE)?;
        assert_eq!(
            invalid_provider.accept_frame(&personal_assistant_frame(
                0,
                json!({
                    "type": "response_started",
                    "provider_response_id": "",
                }),
            )),
            Err(PersonalAssistantTextTurnError::ProtocolViolation)
        );
        assert_eq!(invalid_provider.status(), GatewayStreamStatus::Failed);

        let oversized = personal_assistant_frame(
            0,
            json!({
                "type": "response_started",
                "provider_response_id": "x".repeat(MAX_GATEWAY_EVENT_BYTES),
            }),
        );
        assert!(oversized.len() > MAX_GATEWAY_EVENT_BYTES);
        let mut oversized_turn =
            PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, PA_FIXTURE)?;
        assert_eq!(
            oversized_turn.accept_frame(&oversized),
            Err(PersonalAssistantTextTurnError::LimitExceeded)
        );
        assert_eq!(oversized_turn.status(), GatewayStreamStatus::Failed);
        Ok(())
    }

    #[test]
    fn personal_assistant_rejects_all_function_call_shapes_without_partial_recovery(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let cases = [
            json!({
                "type": "function_call_completed",
                "call_id": "call-1",
                "name": "create_local_task",
                "tool_contract_version": 1,
                "arguments_json": "{}",
            }),
            json!({
                "type": "function_call_completed",
                "call_id": "",
                "name": "get_current_datetime",
                "tool_contract_version": 1,
                "arguments_json": "{}",
            }),
            json!({
                "type": "function_call_completed",
                "call_id": "call-1",
                "name": "bad name",
                "tool_contract_version": 0,
                "arguments_json": "not-json",
            }),
        ];
        for event in cases {
            let mut turn = started_personal_assistant_turn()?;
            assert_eq!(
                turn.accept_frame(&personal_assistant_frame(1, event)),
                Err(PersonalAssistantTextTurnError::ProtocolViolation)
            );
            assert_eq!(turn.status(), GatewayStreamStatus::Failed);
            assert_eq!(
                turn.accept_frame(&personal_assistant_frame(
                    1,
                    json!({ "type": "output_text_delta", "delta": "resume" }),
                )),
                Err(PersonalAssistantTextTurnError::ProtocolViolation)
            );
        }

        let mut after_text = started_personal_assistant_turn()?;
        after_text.accept_frame(&personal_assistant_frame(
            1,
            json!({ "type": "output_text_delta", "delta": "kept-before-rejection" }),
        ))?;
        assert_eq!(
            after_text.accept_frame(&personal_assistant_frame(
                2,
                json!({
                    "type": "function_call_completed",
                    "call_id": "call-2",
                    "name": "create_local_task",
                    "tool_contract_version": 1,
                    "arguments_json": "{}",
                }),
            )),
            Err(PersonalAssistantTextTurnError::ProtocolViolation)
        );
        assert_eq!(after_text.status(), GatewayStreamStatus::Failed);
        assert!(!format!("{after_text:?}").contains("kept-before-rejection"));
        Ok(())
    }

    #[test]
    fn personal_assistant_enforces_delta_and_aggregate_text_limits(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let exact_delta = "🦀".repeat(1_024);
        assert_eq!(exact_delta.len(), 4_096);
        let mut exact = started_personal_assistant_turn()?;
        assert!(matches!(
            exact.accept_frame(&personal_assistant_frame(
                1,
                json!({ "type": "output_text_delta", "delta": exact_delta }),
            ))?,
            PersonalAssistantTextEvent::TextDelta { .. }
        ));

        let mut scalar_overflow = started_personal_assistant_turn()?;
        assert_eq!(
            scalar_overflow.accept_frame(&personal_assistant_frame(
                1,
                json!({ "type": "output_text_delta", "delta": "x".repeat(1_025) }),
            )),
            Err(PersonalAssistantTextTurnError::LimitExceeded)
        );
        assert_eq!(scalar_overflow.status(), GatewayStreamStatus::Failed);

        let mut aggregate_exact = started_personal_assistant_turn()?;
        for sequence in 1..=8 {
            aggregate_exact.accept_frame(&personal_assistant_frame(
                sequence,
                json!({ "type": "output_text_delta", "delta": "🦀".repeat(1_024) }),
            ))?;
        }
        let completed = aggregate_exact.accept_frame(&personal_assistant_frame(
            9,
            json!({ "type": "response_completed" }),
        ))?;
        let PersonalAssistantTextEvent::Completed { final_answer } = completed else {
            return Err(PersonalAssistantTextTurnError::ProtocolViolation);
        };
        assert_eq!(final_answer.into_inner().chars().count(), 8_192);

        let mut aggregate_overflow = started_personal_assistant_turn()?;
        for sequence in 1..=8 {
            aggregate_overflow.accept_frame(&personal_assistant_frame(
                sequence,
                json!({ "type": "output_text_delta", "delta": "x".repeat(1_024) }),
            ))?;
        }
        assert_eq!(
            aggregate_overflow.accept_frame(&personal_assistant_frame(
                9,
                json!({ "type": "output_text_delta", "delta": "x" }),
            )),
            Err(PersonalAssistantTextTurnError::LimitExceeded)
        );
        assert_eq!(aggregate_overflow.status(), GatewayStreamStatus::Failed);
        Ok(())
    }

    #[test]
    fn personal_assistant_enforces_the_128_event_ceiling_transactionally(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let mut exact = started_personal_assistant_turn()?;
        for sequence in 1..=126 {
            exact.accept_frame(&personal_assistant_frame(
                sequence,
                json!({ "type": "output_text_delta", "delta": "x" }),
            ))?;
        }
        exact.accept_frame(&personal_assistant_frame(
            127,
            json!({ "type": "response_completed" }),
        ))?;
        assert_eq!(exact.status(), GatewayStreamStatus::Completed);

        let mut overflow = started_personal_assistant_turn()?;
        for sequence in 1..=127 {
            overflow.accept_frame(&personal_assistant_frame(
                sequence,
                json!({ "type": "output_text_delta", "delta": "x" }),
            ))?;
        }
        assert_eq!(
            overflow.stream.validator.event_count(),
            PERSONAL_ASSISTANT_V0_MAX_EVENTS
        );
        assert_eq!(
            overflow.accept_frame(&personal_assistant_frame(
                128,
                json!({ "type": "response_completed" }),
            )),
            Err(PersonalAssistantTextTurnError::LimitExceeded)
        );
        assert_eq!(overflow.status(), GatewayStreamStatus::Failed);
        Ok(())
    }

    #[test]
    fn personal_assistant_debug_and_closed_errors_redact_request_and_output_content(
    ) -> Result<(), PersonalAssistantTextTurnError> {
        let canary = "pa-private-content-canary";
        let mut turn = PersonalAssistantTextTurn::new(PA_RUN_ID, PA_REQUEST_ID, canary)?;
        let request_debug = format!("{turn:?}");
        assert!(request_debug.contains("[REDACTED]"));
        assert!(!request_debug.contains(canary));
        assert!(!request_debug.contains(PA_RUN_ID));
        assert!(!request_debug.contains(PA_REQUEST_ID));

        turn.accept_frame(&personal_assistant_frame(
            0,
            json!({
                "type": "response_started",
                "provider_response_id": "private-provider-response-canary",
            }),
        ))?;
        let event = turn.accept_frame(&personal_assistant_frame(
            1,
            json!({ "type": "output_text_delta", "delta": canary }),
        ))?;
        let event_debug = format!("{event:?}");
        assert!(event_debug.contains("[REDACTED]"));
        assert!(!event_debug.contains(canary));

        let error = match turn.accept_frame(&personal_assistant_frame(
            9,
            json!({ "type": "response_completed" }),
        )) {
            Ok(_) => return Err(PersonalAssistantTextTurnError::ProtocolViolation),
            Err(error) => error,
        };
        let error_text = format!("{error:?} {error}");
        assert!(!error_text.contains(canary));
        assert!(!format!("{turn:?}").contains(canary));
        Ok(())
    }
}
