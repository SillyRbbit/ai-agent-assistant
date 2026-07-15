use std::fmt;

use serde::Serialize;
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
use crate::approvals::manager::{
    ApprovalError, ApprovalManager, ApprovalPresentation, InMemoryApprovalManager,
};
use crate::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
use crate::policy::types::{PolicyDecision, PolicyInput, PolicyOutcome};
use crate::tools::registry::{InMemoryToolRegistry, ToolRegistry};
use crate::tools::schema::ToolSchema;
use crate::tools::types::ToolDefinition;

pub const INITIAL_GATEWAY_TOOL_SET_ID: &str = "cortexa_desktop_mvp";
pub const INITIAL_GATEWAY_TOOL_SET_VERSION: u16 = 1;

pub type GatewayRequestResult<T> = Result<T, GatewayRequestError>;

pub struct InitialGatewayTurn {
    request: InitialGatewayRequest,
    validator: GatewayStreamValidator,
    registry: InMemoryToolRegistry,
    approval_manager: InMemoryApprovalManager,
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

#[derive(Debug, Error, Eq, PartialEq)]
pub enum InitialGatewayTurnError {
    #[error("initial gateway protocol validation failed: {0}")]
    Protocol(GatewayProtocolError),
    #[error("initial gateway function call failed local schema validation: {0}")]
    FunctionCallValidation(FunctionCallValidationError),
    #[error("initial gateway approval binding failed: {0}")]
    Approval(ApprovalError),
}

pub enum InitialGatewayEvent {
    ResponseStarted { provider_response_id: String },
    OutputTextDelta { delta: String },
    PolicyEvaluated { decision: PolicyDecision },
    ApprovalPresentationReady { presentation: ApprovalPresentation },
    ResponseCompleted,
    ResponseFailed { failure: GatewayFailure },
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
    use serde_json::{json, Value};

    use super::{
        GatewayRequestError, InitialGatewayRequest, INITIAL_GATEWAY_TOOL_SET_ID,
        INITIAL_GATEWAY_TOOL_SET_VERSION,
    };
    use crate::agent::gateway_protocol::{
        AGENT_RUN_DEADLINE, GATEWAY_CONNECT_TIMEOUT, GATEWAY_PROTOCOL_VERSION,
        GATEWAY_STREAM_IDLE_TIMEOUT, MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
        MAX_FUNCTION_ARGUMENT_BYTES, MAX_FUNCTION_CALLS_PER_RUN, MAX_GATEWAY_EVENTS_PER_TURN,
        MAX_GATEWAY_EVENT_BYTES, MAX_GATEWAY_REQUESTS_PER_RUN, MAX_GATEWAY_REQUEST_BYTES,
        MAX_MODEL_TURNS_PER_RUN, MAX_RETRY_AFTER_MS, MAX_RETRY_ATTEMPTS_PER_RUN,
        PROVIDER_TURN_DEADLINE,
    };

    const RUN_ID: &str = "run-initial-request-1";
    const GATEWAY_REQUEST_ID: &str = "gateway-request-initial-1";

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
}
