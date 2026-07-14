use std::collections::BTreeSet;
use std::fmt;
use std::time::Duration;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use thiserror::Error;

pub const GATEWAY_PROTOCOL_VERSION: u16 = 1;
pub const MAX_MODEL_TURNS_PER_RUN: u8 = 2;
pub const MAX_FUNCTION_CALLS_PER_RUN: u8 = 1;
pub const MAX_RETRY_ATTEMPTS_PER_RUN: u8 = 1;
pub const MAX_GATEWAY_REQUESTS_PER_RUN: u8 = 3;
pub const MAX_GATEWAY_REQUEST_BYTES: usize = 65_536;
pub const MAX_GATEWAY_EVENT_BYTES: usize = 16_384;
pub const MAX_FUNCTION_ARGUMENT_BYTES: usize = 8_192;
pub const MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN: usize = 8_192;
pub const MAX_GATEWAY_EVENTS_PER_TURN: usize = 256;
pub const MAX_OPAQUE_ID_BYTES: usize = 128;
pub const MAX_RETRY_AFTER_MS: u64 = 60_000;
pub const GATEWAY_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
pub const GATEWAY_STREAM_IDLE_TIMEOUT: Duration = Duration::from_secs(20);
pub const PROVIDER_TURN_DEADLINE: Duration = Duration::from_secs(60);
pub const AGENT_RUN_DEADLINE: Duration = Duration::from_secs(120);

pub type GatewayProtocolResult<T> = Result<T, GatewayProtocolError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GatewayStreamStatus {
    AwaitingStart,
    Streaming,
    Completed,
    Failed,
    Cancelled,
}

impl GatewayStreamStatus {
    fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GatewayFailureCode {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayFailure {
    code: GatewayFailureCode,
    retryable: bool,
    retry_after_ms: Option<u64>,
}

impl GatewayFailure {
    #[must_use]
    pub fn code(&self) -> GatewayFailureCode {
        self.code
    }

    #[must_use]
    pub fn retryable(&self) -> bool {
        self.retryable
    }

    #[must_use]
    pub fn retry_after_ms(&self) -> Option<u64> {
        self.retry_after_ms
    }
}

#[derive(Eq, PartialEq)]
pub struct UntrustedFunctionCall {
    run_id: String,
    gateway_request_id: String,
    call_id: String,
    name: String,
    tool_contract_version: u16,
    arguments_json: String,
}

impl UntrustedFunctionCall {
    #[must_use]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    #[must_use]
    pub fn gateway_request_id(&self) -> &str {
        &self.gateway_request_id
    }

    #[must_use]
    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn tool_contract_version(&self) -> u16 {
        self.tool_contract_version
    }

    #[must_use]
    pub fn arguments_json(&self) -> &str {
        &self.arguments_json
    }
}

impl fmt::Debug for UntrustedFunctionCall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UntrustedFunctionCall")
            .field("run_id", &self.run_id)
            .field("gateway_request_id", &self.gateway_request_id)
            .field("call_id", &self.call_id)
            .field("name", &self.name)
            .field("tool_contract_version", &self.tool_contract_version)
            .field("arguments_json", &"[REDACTED]")
            .finish()
    }
}

#[derive(Eq, PartialEq)]
pub enum ValidatedGatewayEvent {
    ResponseStarted { provider_response_id: String },
    OutputTextDelta { delta: String },
    FunctionCallCompleted { call: UntrustedFunctionCall },
    ResponseCompleted,
    ResponseFailed { failure: GatewayFailure },
}

impl fmt::Debug for ValidatedGatewayEvent {
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
            Self::FunctionCallCompleted { call } => formatter
                .debug_struct("FunctionCallCompleted")
                .field("call", call)
                .finish(),
            Self::ResponseCompleted => formatter.write_str("ResponseCompleted"),
            Self::ResponseFailed { failure } => formatter
                .debug_struct("ResponseFailed")
                .field("failure", failure)
                .finish(),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum GatewayProtocolError {
    #[error("expected run id is invalid")]
    InvalidExpectedRunId,
    #[error("expected gateway request id is invalid")]
    InvalidExpectedGatewayRequestId,
    #[error("allowed tool name is invalid")]
    InvalidAllowedToolName,
    #[error("expected tool contract version must be greater than zero")]
    InvalidExpectedToolContractVersion,
    #[error("gateway event exceeds {maximum_bytes} bytes")]
    EventTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("gateway event is not valid normalized protocol JSON")]
    MalformedEvent,
    #[error("unsupported gateway protocol version")]
    UnsupportedProtocolVersion { supported: u16, actual: u16 },
    #[error("gateway event run id is invalid")]
    InvalidRunId,
    #[error("gateway event run id does not match the active run")]
    RunIdMismatch,
    #[error("gateway event request id is invalid")]
    InvalidGatewayRequestId,
    #[error("gateway event request id does not match the active request")]
    GatewayRequestIdMismatch,
    #[error("gateway event sequence is not contiguous")]
    InvalidSequence { expected: u32, actual: u32 },
    #[error("gateway event count exceeds the per-turn limit")]
    EventLimitExceeded { maximum: usize },
    #[error("gateway stream is already terminal")]
    StreamAlreadyTerminal { status: GatewayStreamStatus },
    #[error("response event arrived before response_started")]
    EventBeforeResponseStarted,
    #[error("response_started may appear only once and first")]
    UnexpectedResponseStarted,
    #[error("provider response id is invalid")]
    InvalidProviderResponseId,
    #[error("output text delta must not be empty")]
    EmptyOutputTextDelta,
    #[error("assistant output exceeds the per-turn character limit")]
    AssistantOutputLimitExceeded { maximum: usize },
    #[error("a provider turn cannot contain both text and a function call")]
    MixedResponseOutput,
    #[error("response_completed requires text or one completed function call")]
    MissingResponseOutput,
    #[error("function call count exceeds the per-run limit")]
    FunctionCallLimitExceeded { maximum: u8 },
    #[error("function call id is invalid")]
    InvalidFunctionCallId,
    #[error("function name is invalid")]
    InvalidFunctionName,
    #[error("function name is not in the allowed tool set")]
    UnknownFunctionName,
    #[error("function tool contract version does not match the active contract")]
    ToolContractVersionMismatch { expected: u16, actual: u16 },
    #[error("function arguments exceed the byte limit")]
    FunctionArgumentsTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("function arguments are not valid JSON")]
    MalformedFunctionArguments,
    #[error("function arguments must be one JSON object")]
    FunctionArgumentsMustBeObject,
    #[error("function arguments contain a duplicate object key")]
    DuplicateFunctionArgumentKey,
    #[error("gateway failure retry delay exceeds the limit")]
    RetryDelayTooLarge { maximum_ms: u64, actual_ms: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResponseOutputKind {
    None,
    Text,
    FunctionCall,
}

#[derive(Clone, Debug)]
pub struct GatewayStreamValidator {
    expected_run_id: String,
    expected_gateway_request_id: String,
    allowed_function_names: BTreeSet<String>,
    expected_tool_contract_version: u16,
    status: GatewayStreamStatus,
    output_kind: ResponseOutputKind,
    next_sequence: u32,
    event_count: usize,
    assistant_output_characters: usize,
    function_call_count: u8,
}

impl GatewayStreamValidator {
    pub fn new(
        expected_run_id: impl Into<String>,
        expected_gateway_request_id: impl Into<String>,
        allowed_function_names: impl IntoIterator<Item = String>,
        expected_tool_contract_version: u16,
    ) -> GatewayProtocolResult<Self> {
        let expected_run_id = expected_run_id.into();
        if !is_valid_opaque_id(&expected_run_id) {
            return Err(GatewayProtocolError::InvalidExpectedRunId);
        }

        let expected_gateway_request_id = expected_gateway_request_id.into();
        if !is_valid_opaque_id(&expected_gateway_request_id) {
            return Err(GatewayProtocolError::InvalidExpectedGatewayRequestId);
        }

        if expected_tool_contract_version == 0 {
            return Err(GatewayProtocolError::InvalidExpectedToolContractVersion);
        }

        let allowed_function_names: BTreeSet<String> = allowed_function_names.into_iter().collect();
        if allowed_function_names
            .iter()
            .any(|name| !is_valid_function_name(name))
        {
            return Err(GatewayProtocolError::InvalidAllowedToolName);
        }

        Ok(Self {
            expected_run_id,
            expected_gateway_request_id,
            allowed_function_names,
            expected_tool_contract_version,
            status: GatewayStreamStatus::AwaitingStart,
            output_kind: ResponseOutputKind::None,
            next_sequence: 0,
            event_count: 0,
            assistant_output_characters: 0,
            function_call_count: 0,
        })
    }

    #[must_use]
    pub fn status(&self) -> GatewayStreamStatus {
        self.status
    }

    #[must_use]
    pub fn event_count(&self) -> usize {
        self.event_count
    }

    #[must_use]
    pub fn assistant_output_characters(&self) -> usize {
        self.assistant_output_characters
    }

    #[must_use]
    pub fn function_call_count(&self) -> u8 {
        self.function_call_count
    }

    #[must_use]
    pub fn cancel(&mut self) -> bool {
        if self.status.is_terminal() {
            return false;
        }

        self.status = GatewayStreamStatus::Cancelled;
        true
    }

    pub fn accept_frame(&mut self, frame: &[u8]) -> GatewayProtocolResult<ValidatedGatewayEvent> {
        if frame.len() > MAX_GATEWAY_EVENT_BYTES {
            return Err(GatewayProtocolError::EventTooLarge {
                maximum_bytes: MAX_GATEWAY_EVENT_BYTES,
                actual_bytes: frame.len(),
            });
        }

        if self.status.is_terminal() {
            return Err(GatewayProtocolError::StreamAlreadyTerminal {
                status: self.status,
            });
        }

        if self.event_count >= MAX_GATEWAY_EVENTS_PER_TURN {
            return Err(GatewayProtocolError::EventLimitExceeded {
                maximum: MAX_GATEWAY_EVENTS_PER_TURN,
            });
        }

        let envelope: WireGatewayEnvelope =
            serde_json::from_slice(frame).map_err(|_| GatewayProtocolError::MalformedEvent)?;
        self.validate_envelope(&envelope)?;

        let mut next = self.clone();
        let validated_event = next.apply_event(envelope.event)?;
        next.next_sequence += 1;
        next.event_count += 1;
        *self = next;

        Ok(validated_event)
    }

    fn validate_envelope(&self, envelope: &WireGatewayEnvelope) -> GatewayProtocolResult<()> {
        if envelope.protocol_version != GATEWAY_PROTOCOL_VERSION {
            return Err(GatewayProtocolError::UnsupportedProtocolVersion {
                supported: GATEWAY_PROTOCOL_VERSION,
                actual: envelope.protocol_version,
            });
        }

        if !is_valid_opaque_id(&envelope.run_id) {
            return Err(GatewayProtocolError::InvalidRunId);
        }
        if envelope.run_id != self.expected_run_id {
            return Err(GatewayProtocolError::RunIdMismatch);
        }

        if !is_valid_opaque_id(&envelope.gateway_request_id) {
            return Err(GatewayProtocolError::InvalidGatewayRequestId);
        }
        if envelope.gateway_request_id != self.expected_gateway_request_id {
            return Err(GatewayProtocolError::GatewayRequestIdMismatch);
        }

        if envelope.sequence != self.next_sequence {
            return Err(GatewayProtocolError::InvalidSequence {
                expected: self.next_sequence,
                actual: envelope.sequence,
            });
        }

        Ok(())
    }

    fn apply_event(
        &mut self,
        event: WireGatewayEvent,
    ) -> GatewayProtocolResult<ValidatedGatewayEvent> {
        match event {
            WireGatewayEvent::ResponseStarted {
                provider_response_id,
            } => self.apply_response_started(provider_response_id),
            WireGatewayEvent::OutputTextDelta { delta } => self.apply_output_text_delta(delta),
            WireGatewayEvent::FunctionCallCompleted {
                call_id,
                name,
                tool_contract_version,
                arguments_json,
            } => self.apply_function_call(call_id, name, tool_contract_version, arguments_json),
            WireGatewayEvent::ResponseCompleted => self.apply_response_completed(),
            WireGatewayEvent::ResponseFailed {
                code,
                retryable,
                retry_after_ms,
            } => self.apply_response_failed(code, retryable, retry_after_ms),
        }
    }

    fn apply_response_started(
        &mut self,
        provider_response_id: String,
    ) -> GatewayProtocolResult<ValidatedGatewayEvent> {
        if self.status != GatewayStreamStatus::AwaitingStart {
            return Err(GatewayProtocolError::UnexpectedResponseStarted);
        }
        if !is_valid_opaque_id(&provider_response_id) {
            return Err(GatewayProtocolError::InvalidProviderResponseId);
        }

        self.status = GatewayStreamStatus::Streaming;
        Ok(ValidatedGatewayEvent::ResponseStarted {
            provider_response_id,
        })
    }

    fn apply_output_text_delta(
        &mut self,
        delta: String,
    ) -> GatewayProtocolResult<ValidatedGatewayEvent> {
        self.require_streaming()?;
        if delta.is_empty() {
            return Err(GatewayProtocolError::EmptyOutputTextDelta);
        }
        if self.output_kind == ResponseOutputKind::FunctionCall {
            return Err(GatewayProtocolError::MixedResponseOutput);
        }

        let delta_characters = delta.chars().count();
        let Some(next_character_count) = self
            .assistant_output_characters
            .checked_add(delta_characters)
        else {
            return Err(GatewayProtocolError::AssistantOutputLimitExceeded {
                maximum: MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
            });
        };
        if next_character_count > MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN {
            return Err(GatewayProtocolError::AssistantOutputLimitExceeded {
                maximum: MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
            });
        }

        self.output_kind = ResponseOutputKind::Text;
        self.assistant_output_characters = next_character_count;
        Ok(ValidatedGatewayEvent::OutputTextDelta { delta })
    }

    fn apply_function_call(
        &mut self,
        call_id: String,
        name: String,
        tool_contract_version: u16,
        arguments_json: String,
    ) -> GatewayProtocolResult<ValidatedGatewayEvent> {
        self.require_streaming()?;
        if self.output_kind == ResponseOutputKind::Text {
            return Err(GatewayProtocolError::MixedResponseOutput);
        }
        if self.function_call_count >= MAX_FUNCTION_CALLS_PER_RUN {
            return Err(GatewayProtocolError::FunctionCallLimitExceeded {
                maximum: MAX_FUNCTION_CALLS_PER_RUN,
            });
        }
        if !is_valid_opaque_id(&call_id) {
            return Err(GatewayProtocolError::InvalidFunctionCallId);
        }
        if !is_valid_function_name(&name) {
            return Err(GatewayProtocolError::InvalidFunctionName);
        }
        if !self.allowed_function_names.contains(&name) {
            return Err(GatewayProtocolError::UnknownFunctionName);
        }
        if tool_contract_version != self.expected_tool_contract_version {
            return Err(GatewayProtocolError::ToolContractVersionMismatch {
                expected: self.expected_tool_contract_version,
                actual: tool_contract_version,
            });
        }
        validate_function_arguments(&arguments_json)?;

        self.output_kind = ResponseOutputKind::FunctionCall;
        self.function_call_count += 1;
        Ok(ValidatedGatewayEvent::FunctionCallCompleted {
            call: UntrustedFunctionCall {
                run_id: self.expected_run_id.clone(),
                gateway_request_id: self.expected_gateway_request_id.clone(),
                call_id,
                name,
                tool_contract_version,
                arguments_json,
            },
        })
    }

    fn apply_response_completed(&mut self) -> GatewayProtocolResult<ValidatedGatewayEvent> {
        self.require_streaming()?;
        if self.output_kind == ResponseOutputKind::None {
            return Err(GatewayProtocolError::MissingResponseOutput);
        }

        self.status = GatewayStreamStatus::Completed;
        Ok(ValidatedGatewayEvent::ResponseCompleted)
    }

    fn apply_response_failed(
        &mut self,
        code: GatewayFailureCode,
        retryable: bool,
        retry_after_ms: Option<u64>,
    ) -> GatewayProtocolResult<ValidatedGatewayEvent> {
        self.require_streaming()?;
        if let Some(actual_ms) = retry_after_ms {
            if actual_ms > MAX_RETRY_AFTER_MS {
                return Err(GatewayProtocolError::RetryDelayTooLarge {
                    maximum_ms: MAX_RETRY_AFTER_MS,
                    actual_ms,
                });
            }
        }

        self.status = GatewayStreamStatus::Failed;
        Ok(ValidatedGatewayEvent::ResponseFailed {
            failure: GatewayFailure {
                code,
                retryable,
                retry_after_ms,
            },
        })
    }

    fn require_streaming(&self) -> GatewayProtocolResult<()> {
        if self.status == GatewayStreamStatus::Streaming {
            Ok(())
        } else {
            Err(GatewayProtocolError::EventBeforeResponseStarted)
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireGatewayEnvelope {
    protocol_version: u16,
    run_id: String,
    gateway_request_id: String,
    sequence: u32,
    event: WireGatewayEvent,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum WireGatewayEvent {
    ResponseStarted {
        provider_response_id: String,
    },
    OutputTextDelta {
        delta: String,
    },
    FunctionCallCompleted {
        call_id: String,
        name: String,
        tool_contract_version: u16,
        arguments_json: String,
    },
    ResponseCompleted,
    ResponseFailed {
        code: GatewayFailureCode,
        retryable: bool,
        #[serde(default)]
        retry_after_ms: Option<u64>,
    },
}

fn is_valid_opaque_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_OPAQUE_ID_BYTES
        && value.bytes().all(|byte| byte.is_ascii_graphic())
}

fn is_valid_function_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_OPAQUE_ID_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}

fn validate_function_arguments(arguments_json: &str) -> GatewayProtocolResult<()> {
    if arguments_json.len() > MAX_FUNCTION_ARGUMENT_BYTES {
        return Err(GatewayProtocolError::FunctionArgumentsTooLarge {
            maximum_bytes: MAX_FUNCTION_ARGUMENT_BYTES,
            actual_bytes: arguments_json.len(),
        });
    }

    let checked: CheckedJson = serde_json::from_str(arguments_json)
        .map_err(|_| GatewayProtocolError::MalformedFunctionArguments)?;
    if !checked.is_object {
        return Err(GatewayProtocolError::FunctionArgumentsMustBeObject);
    }
    if checked.has_duplicate_key {
        return Err(GatewayProtocolError::DuplicateFunctionArgumentKey);
    }

    Ok(())
}

#[derive(Debug)]
struct CheckedJson {
    is_object: bool,
    has_duplicate_key: bool,
}

impl CheckedJson {
    fn scalar() -> Self {
        Self {
            is_object: false,
            has_duplicate_key: false,
        }
    }
}

impl<'de> Deserialize<'de> for CheckedJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckedJsonVisitor)
    }
}

struct CheckedJsonVisitor;

impl<'de> Visitor<'de> for CheckedJsonVisitor {
    type Value = CheckedJson;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }

    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(CheckedJson::scalar())
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut has_duplicate_key = false;
        while let Some(value) = sequence.next_element::<CheckedJson>()? {
            has_duplicate_key |= value.has_duplicate_key;
        }

        Ok(CheckedJson {
            is_object: false,
            has_duplicate_key,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut keys = BTreeSet::new();
        let mut has_duplicate_key = false;

        while let Some(key) = map.next_key::<String>()? {
            if !keys.insert(key) {
                has_duplicate_key = true;
            }
            let value = map.next_value::<CheckedJson>()?;
            has_duplicate_key |= value.has_duplicate_key;
        }

        Ok(CheckedJson {
            is_object: true,
            has_duplicate_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{
        GatewayFailureCode, GatewayProtocolError, GatewayProtocolResult, GatewayStreamStatus,
        GatewayStreamValidator, ValidatedGatewayEvent, AGENT_RUN_DEADLINE, GATEWAY_CONNECT_TIMEOUT,
        GATEWAY_PROTOCOL_VERSION, GATEWAY_STREAM_IDLE_TIMEOUT,
        MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN, MAX_FUNCTION_ARGUMENT_BYTES,
        MAX_FUNCTION_CALLS_PER_RUN, MAX_GATEWAY_EVENTS_PER_TURN, MAX_GATEWAY_EVENT_BYTES,
        MAX_GATEWAY_REQUESTS_PER_RUN, MAX_GATEWAY_REQUEST_BYTES, MAX_MODEL_TURNS_PER_RUN,
        MAX_OPAQUE_ID_BYTES, MAX_RETRY_AFTER_MS, MAX_RETRY_ATTEMPTS_PER_RUN,
        PROVIDER_TURN_DEADLINE,
    };

    const RUN_ID: &str = "run-1";
    const GATEWAY_REQUEST_ID: &str = "gateway-request-1";
    const TOOL_NAME: &str = "create_local_task";
    const TOOL_CONTRACT_VERSION: u16 = 1;

    fn validator() -> GatewayProtocolResult<GatewayStreamValidator> {
        GatewayStreamValidator::new(
            RUN_ID,
            GATEWAY_REQUEST_ID,
            [TOOL_NAME.to_owned()],
            TOOL_CONTRACT_VERSION,
        )
    }

    fn frame(sequence: u32, event: &str) -> String {
        format!(
            r#"{{"protocol_version":1,"run_id":"{RUN_ID}","gateway_request_id":"{GATEWAY_REQUEST_ID}","sequence":{sequence},"event":{event}}}"#
        )
    }

    fn start_frame() -> String {
        frame(
            0,
            r#"{"type":"response_started","provider_response_id":"provider-response-1"}"#,
        )
    }

    fn function_event(
        call_id: &str,
        name: &str,
        version: u16,
        arguments_json: &str,
    ) -> Result<String, serde_json::Error> {
        let encoded_arguments = serde_json::to_string(arguments_json)?;
        Ok(format!(
            r#"{{"type":"function_call_completed","call_id":"{call_id}","name":"{name}","tool_contract_version":{version},"arguments_json":{encoded_arguments}}}"#
        ))
    }

    fn start(validator: &mut GatewayStreamValidator) -> GatewayProtocolResult<()> {
        let event = validator.accept_frame(start_frame().as_bytes())?;
        assert_eq!(
            event,
            ValidatedGatewayEvent::ResponseStarted {
                provider_response_id: "provider-response-1".to_owned(),
            }
        );
        Ok(())
    }

    #[test]
    fn exposes_the_frozen_protocol_limits() {
        assert_eq!(GATEWAY_PROTOCOL_VERSION, 1);
        assert_eq!(MAX_MODEL_TURNS_PER_RUN, 2);
        assert_eq!(MAX_FUNCTION_CALLS_PER_RUN, 1);
        assert_eq!(MAX_RETRY_ATTEMPTS_PER_RUN, 1);
        assert_eq!(MAX_GATEWAY_REQUESTS_PER_RUN, 3);
        assert_eq!(MAX_GATEWAY_REQUEST_BYTES, 65_536);
        assert_eq!(MAX_GATEWAY_EVENT_BYTES, 16_384);
        assert_eq!(MAX_FUNCTION_ARGUMENT_BYTES, 8_192);
        assert_eq!(MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN, 8_192);
        assert_eq!(MAX_GATEWAY_EVENTS_PER_TURN, 256);
        assert_eq!(MAX_RETRY_AFTER_MS, 60_000);
        assert_eq!(GATEWAY_CONNECT_TIMEOUT.as_secs(), 10);
        assert_eq!(GATEWAY_STREAM_IDLE_TIMEOUT.as_secs(), 20);
        assert_eq!(PROVIDER_TURN_DEADLINE.as_secs(), 60);
        assert_eq!(AGENT_RUN_DEADLINE.as_secs(), 120);
    }

    #[test]
    fn rejects_invalid_validator_configuration() {
        assert_eq!(
            GatewayStreamValidator::new(
                " ",
                GATEWAY_REQUEST_ID,
                Vec::<String>::new(),
                TOOL_CONTRACT_VERSION,
            )
            .err(),
            Some(GatewayProtocolError::InvalidExpectedRunId)
        );
        assert_eq!(
            GatewayStreamValidator::new(
                RUN_ID,
                "gateway request",
                Vec::<String>::new(),
                TOOL_CONTRACT_VERSION,
            )
            .err(),
            Some(GatewayProtocolError::InvalidExpectedGatewayRequestId)
        );
        assert_eq!(
            GatewayStreamValidator::new(
                RUN_ID,
                GATEWAY_REQUEST_ID,
                ["invalid tool".to_owned()],
                TOOL_CONTRACT_VERSION,
            )
            .err(),
            Some(GatewayProtocolError::InvalidAllowedToolName)
        );
        assert_eq!(
            GatewayStreamValidator::new(RUN_ID, GATEWAY_REQUEST_ID, Vec::<String>::new(), 0,).err(),
            Some(GatewayProtocolError::InvalidExpectedToolContractVersion)
        );
    }

    #[test]
    fn accepts_a_bounded_text_response() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        start(&mut validator)?;

        assert_eq!(
            validator.accept_frame(
                frame(1, r#"{"type":"output_text_delta","delta":"Hello "}"#).as_bytes()
            ),
            Ok(ValidatedGatewayEvent::OutputTextDelta {
                delta: "Hello ".to_owned(),
            })
        );
        assert_eq!(
            validator.accept_frame(
                frame(2, r#"{"type":"output_text_delta","delta":"world"}"#).as_bytes()
            ),
            Ok(ValidatedGatewayEvent::OutputTextDelta {
                delta: "world".to_owned(),
            })
        );
        assert_eq!(
            validator.accept_frame(frame(3, r#"{"type":"response_completed"}"#).as_bytes()),
            Ok(ValidatedGatewayEvent::ResponseCompleted)
        );
        assert_eq!(validator.status(), GatewayStreamStatus::Completed);
        assert_eq!(validator.event_count(), 4);
        assert_eq!(validator.assistant_output_characters(), 11);
        assert_eq!(validator.function_call_count(), 0);
        Ok(())
    }

    #[test]
    fn accepts_one_non_actionable_function_call() -> Result<(), Box<dyn Error>> {
        let mut validator = validator()?;
        start(&mut validator)?;
        let event = function_event(
            "call-1",
            TOOL_NAME,
            TOOL_CONTRACT_VERSION,
            r#"{"title":"Review plan"}"#,
        )?;

        let accepted = validator.accept_frame(frame(1, &event).as_bytes())?;
        let ValidatedGatewayEvent::FunctionCallCompleted { call } = accepted else {
            return Err(GatewayProtocolError::MalformedEvent.into());
        };
        assert_eq!(call.run_id(), RUN_ID);
        assert_eq!(call.gateway_request_id(), GATEWAY_REQUEST_ID);
        assert_eq!(call.call_id(), "call-1");
        assert_eq!(call.name(), TOOL_NAME);
        assert_eq!(call.tool_contract_version(), TOOL_CONTRACT_VERSION);
        assert_eq!(call.arguments_json(), r#"{"title":"Review plan"}"#);
        assert_eq!(validator.function_call_count(), 1);
        assert_eq!(
            validator.accept_frame(frame(2, r#"{"type":"response_completed"}"#).as_bytes()),
            Ok(ValidatedGatewayEvent::ResponseCompleted)
        );
        assert_eq!(validator.status(), GatewayStreamStatus::Completed);
        Ok(())
    }

    #[test]
    fn rejects_oversized_and_malformed_frames_without_state_changes() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        let oversized = vec![b'x'; MAX_GATEWAY_EVENT_BYTES + 1];

        assert_eq!(
            validator.accept_frame(&oversized),
            Err(GatewayProtocolError::EventTooLarge {
                maximum_bytes: MAX_GATEWAY_EVENT_BYTES,
                actual_bytes: MAX_GATEWAY_EVENT_BYTES + 1,
            })
        );
        assert_eq!(
            validator.accept_frame(b"not-json"),
            Err(GatewayProtocolError::MalformedEvent)
        );
        assert_eq!(validator.status(), GatewayStreamStatus::AwaitingStart);
        assert_eq!(validator.event_count(), 0);
        Ok(())
    }

    #[test]
    fn rejects_unknown_fields_variants_and_action_metadata() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        let unknown_envelope_field = r#"{"protocol_version":1,"run_id":"run-1","gateway_request_id":"gateway-request-1","sequence":0,"extra":true,"event":{"type":"response_started","provider_response_id":"provider-response-1"}}"#;
        let unknown_event = frame(0, r#"{"type":"provider_debug","detail":"secret"}"#);
        let action_metadata = frame(
            0,
            r#"{"type":"response_started","provider_response_id":"provider-response-1","approved":true}"#,
        );

        for invalid in [unknown_envelope_field, &unknown_event, &action_metadata] {
            assert_eq!(
                validator.accept_frame(invalid.as_bytes()),
                Err(GatewayProtocolError::MalformedEvent)
            );
        }
        assert_eq!(validator.event_count(), 0);
        Ok(())
    }

    #[test]
    fn rejects_protocol_identity_and_sequence_mismatches() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        let wrong_version =
            start_frame().replacen("\"protocol_version\":1", "\"protocol_version\":2", 1);
        let wrong_run = start_frame().replacen("\"run_id\":\"run-1\"", "\"run_id\":\"run-2\"", 1);
        let wrong_request = start_frame().replacen(
            "\"gateway_request_id\":\"gateway-request-1\"",
            "\"gateway_request_id\":\"gateway-request-2\"",
            1,
        );
        let wrong_sequence = start_frame().replacen("\"sequence\":0", "\"sequence\":1", 1);

        assert_eq!(
            validator.accept_frame(wrong_version.as_bytes()),
            Err(GatewayProtocolError::UnsupportedProtocolVersion {
                supported: 1,
                actual: 2,
            })
        );
        assert_eq!(
            validator.accept_frame(wrong_run.as_bytes()),
            Err(GatewayProtocolError::RunIdMismatch)
        );
        assert_eq!(
            validator.accept_frame(wrong_request.as_bytes()),
            Err(GatewayProtocolError::GatewayRequestIdMismatch)
        );
        assert_eq!(
            validator.accept_frame(wrong_sequence.as_bytes()),
            Err(GatewayProtocolError::InvalidSequence {
                expected: 0,
                actual: 1,
            })
        );
        assert_eq!(validator.event_count(), 0);
        Ok(())
    }

    #[test]
    fn rejects_invalid_frame_identifiers() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        let invalid_run =
            start_frame().replacen("\"run_id\":\"run-1\"", "\"run_id\":\"bad id\"", 1);
        let invalid_request = start_frame().replacen(
            "\"gateway_request_id\":\"gateway-request-1\"",
            "\"gateway_request_id\":\"bad id\"",
            1,
        );
        let invalid_provider = frame(
            0,
            r#"{"type":"response_started","provider_response_id":"bad id"}"#,
        );
        let non_ascii_run =
            start_frame().replacen("\"run_id\":\"run-1\"", "\"run_id\":\"run-\u{00e9}\"", 1);
        let overlength_run = start_frame().replacen(
            "\"run_id\":\"run-1\"",
            &format!("\"run_id\":\"{}\"", "x".repeat(MAX_OPAQUE_ID_BYTES + 1)),
            1,
        );

        assert_eq!(
            validator.accept_frame(invalid_run.as_bytes()),
            Err(GatewayProtocolError::InvalidRunId)
        );
        assert_eq!(
            validator.accept_frame(invalid_request.as_bytes()),
            Err(GatewayProtocolError::InvalidGatewayRequestId)
        );
        assert_eq!(
            validator.accept_frame(invalid_provider.as_bytes()),
            Err(GatewayProtocolError::InvalidProviderResponseId)
        );
        assert_eq!(
            validator.accept_frame(non_ascii_run.as_bytes()),
            Err(GatewayProtocolError::InvalidRunId)
        );
        assert_eq!(
            validator.accept_frame(overlength_run.as_bytes()),
            Err(GatewayProtocolError::InvalidRunId)
        );
        assert_eq!(validator.event_count(), 0);
        Ok(())
    }

    #[test]
    fn rejects_out_of_order_duplicate_and_late_events() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        let delta = frame(0, r#"{"type":"output_text_delta","delta":"early"}"#);
        assert_eq!(
            validator.accept_frame(delta.as_bytes()),
            Err(GatewayProtocolError::EventBeforeResponseStarted)
        );

        start(&mut validator)?;
        assert_eq!(
            validator.accept_frame(
                frame(
                    1,
                    r#"{"type":"response_started","provider_response_id":"provider-response-2"}"#
                )
                .as_bytes()
            ),
            Err(GatewayProtocolError::UnexpectedResponseStarted)
        );
        assert_eq!(
            validator
                .accept_frame(frame(1, r#"{"type":"output_text_delta","delta":"ok"}"#).as_bytes()),
            Ok(ValidatedGatewayEvent::OutputTextDelta {
                delta: "ok".to_owned(),
            })
        );
        assert_eq!(
            validator.accept_frame(frame(2, r#"{"type":"response_completed"}"#).as_bytes()),
            Ok(ValidatedGatewayEvent::ResponseCompleted)
        );
        assert_eq!(
            validator.accept_frame(
                frame(3, r#"{"type":"output_text_delta","delta":"late"}"#).as_bytes()
            ),
            Err(GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Completed,
            })
        );
        assert_eq!(
            validator.accept_frame(frame(3, r#"{"type":"response_completed"}"#).as_bytes()),
            Err(GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Completed,
            })
        );
        Ok(())
    }

    #[test]
    fn cancellation_is_local_idempotent_and_terminal() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        start(&mut validator)?;

        assert!(validator.cancel());
        assert!(!validator.cancel());
        assert_eq!(validator.status(), GatewayStreamStatus::Cancelled);
        assert_eq!(
            validator.accept_frame(
                frame(1, r#"{"type":"output_text_delta","delta":"late"}"#).as_bytes()
            ),
            Err(GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Cancelled,
            })
        );
        Ok(())
    }

    #[test]
    fn rejects_empty_mixed_missing_and_excess_output() -> Result<(), Box<dyn Error>> {
        let mut empty = validator()?;
        start(&mut empty)?;
        assert_eq!(
            empty.accept_frame(frame(1, r#"{"type":"output_text_delta","delta":""}"#).as_bytes()),
            Err(GatewayProtocolError::EmptyOutputTextDelta)
        );
        assert_eq!(
            empty.accept_frame(frame(1, r#"{"type":"response_completed"}"#).as_bytes()),
            Err(GatewayProtocolError::MissingResponseOutput)
        );

        let mut text_then_call = validator()?;
        start(&mut text_then_call)?;
        text_then_call
            .accept_frame(frame(1, r#"{"type":"output_text_delta","delta":"text"}"#).as_bytes())?;
        let call = function_event(
            "call-1",
            TOOL_NAME,
            TOOL_CONTRACT_VERSION,
            r#"{"title":"x"}"#,
        )?;
        assert_eq!(
            text_then_call.accept_frame(frame(2, &call).as_bytes()),
            Err(GatewayProtocolError::MixedResponseOutput)
        );

        let mut call_then_text = validator()?;
        start(&mut call_then_text)?;
        call_then_text.accept_frame(frame(1, &call).as_bytes())?;
        assert_eq!(
            call_then_text.accept_frame(
                frame(2, r#"{"type":"output_text_delta","delta":"text"}"#).as_bytes()
            ),
            Err(GatewayProtocolError::MixedResponseOutput)
        );
        assert_eq!(
            call_then_text.accept_frame(frame(2, &call).as_bytes()),
            Err(GatewayProtocolError::FunctionCallLimitExceeded { maximum: 1 })
        );
        Ok(())
    }

    #[test]
    fn enforces_output_and_event_count_limits() -> GatewayProtocolResult<()> {
        let mut output_limited = validator()?;
        start(&mut output_limited)?;
        let oversized_delta = "x".repeat(MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN + 1);
        let oversized_event =
            format!(r#"{{"type":"output_text_delta","delta":"{oversized_delta}"}}"#);
        assert_eq!(
            output_limited.accept_frame(frame(1, &oversized_event).as_bytes()),
            Err(GatewayProtocolError::AssistantOutputLimitExceeded {
                maximum: MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
            })
        );
        assert_eq!(output_limited.assistant_output_characters(), 0);

        let mut event_limited = validator()?;
        start(&mut event_limited)?;
        for sequence in 1..MAX_GATEWAY_EVENTS_PER_TURN as u32 {
            event_limited.accept_frame(
                frame(sequence, r#"{"type":"output_text_delta","delta":"x"}"#).as_bytes(),
            )?;
        }
        assert_eq!(event_limited.event_count(), MAX_GATEWAY_EVENTS_PER_TURN);
        assert_eq!(
            event_limited.accept_frame(
                frame(
                    MAX_GATEWAY_EVENTS_PER_TURN as u32,
                    r#"{"type":"response_completed"}"#,
                )
                .as_bytes()
            ),
            Err(GatewayProtocolError::EventLimitExceeded {
                maximum: MAX_GATEWAY_EVENTS_PER_TURN,
            })
        );
        Ok(())
    }

    #[test]
    fn rejects_invalid_function_identity_and_contract() -> Result<(), Box<dyn Error>> {
        let invalid_cases = [
            (
                function_event("bad id", TOOL_NAME, TOOL_CONTRACT_VERSION, "{}")?,
                GatewayProtocolError::InvalidFunctionCallId,
            ),
            (
                function_event("call-1", "bad name", TOOL_CONTRACT_VERSION, "{}")?,
                GatewayProtocolError::InvalidFunctionName,
            ),
            (
                function_event("call-1", "unknown_tool", TOOL_CONTRACT_VERSION, "{}")?,
                GatewayProtocolError::UnknownFunctionName,
            ),
            (
                function_event("call-1", TOOL_NAME, 2, "{}")?,
                GatewayProtocolError::ToolContractVersionMismatch {
                    expected: TOOL_CONTRACT_VERSION,
                    actual: 2,
                },
            ),
        ];

        for (event, expected_error) in invalid_cases {
            let mut validator = validator()?;
            start(&mut validator)?;
            assert_eq!(
                validator.accept_frame(frame(1, &event).as_bytes()),
                Err(expected_error)
            );
            assert_eq!(validator.function_call_count(), 0);
            assert_eq!(validator.event_count(), 1);
        }
        Ok(())
    }

    #[test]
    fn rejects_malformed_non_object_duplicate_and_oversized_arguments() -> Result<(), Box<dyn Error>>
    {
        let oversized = format!(
            r#"{{"value":"{}"}}"#,
            "x".repeat(MAX_FUNCTION_ARGUMENT_BYTES)
        );
        let invalid_cases = [
            (
                function_event("call-1", TOOL_NAME, TOOL_CONTRACT_VERSION, "{")?,
                GatewayProtocolError::MalformedFunctionArguments,
            ),
            (
                function_event("call-1", TOOL_NAME, TOOL_CONTRACT_VERSION, "[]")?,
                GatewayProtocolError::FunctionArgumentsMustBeObject,
            ),
            (
                function_event(
                    "call-1",
                    TOOL_NAME,
                    TOOL_CONTRACT_VERSION,
                    r#"{"title":"a","title":"b"}"#,
                )?,
                GatewayProtocolError::DuplicateFunctionArgumentKey,
            ),
            (
                function_event(
                    "call-1",
                    TOOL_NAME,
                    TOOL_CONTRACT_VERSION,
                    r#"{"outer":{"title":"a","title":"b"}}"#,
                )?,
                GatewayProtocolError::DuplicateFunctionArgumentKey,
            ),
            (
                function_event("call-1", TOOL_NAME, TOOL_CONTRACT_VERSION, &oversized)?,
                GatewayProtocolError::FunctionArgumentsTooLarge {
                    maximum_bytes: MAX_FUNCTION_ARGUMENT_BYTES,
                    actual_bytes: oversized.len(),
                },
            ),
        ];

        for (event, expected_error) in invalid_cases {
            let mut validator = validator()?;
            start(&mut validator)?;
            assert_eq!(
                validator.accept_frame(frame(1, &event).as_bytes()),
                Err(expected_error)
            );
            assert_eq!(validator.event_count(), 1);
        }
        Ok(())
    }

    #[test]
    fn accepts_closed_redacted_failure_metadata() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        start(&mut validator)?;
        let event = frame(
            1,
            r#"{"type":"response_failed","code":"rate_limited","retryable":true,"retry_after_ms":1000}"#,
        );

        let accepted = validator.accept_frame(event.as_bytes())?;
        let ValidatedGatewayEvent::ResponseFailed { failure } = accepted else {
            return Err(GatewayProtocolError::MalformedEvent);
        };
        assert_eq!(failure.code(), GatewayFailureCode::RateLimited);
        assert!(failure.retryable());
        assert_eq!(failure.retry_after_ms(), Some(1_000));
        assert_eq!(validator.status(), GatewayStreamStatus::Failed);
        Ok(())
    }

    #[test]
    fn rejects_unbounded_or_extended_failure_metadata() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        start(&mut validator)?;
        assert_eq!(
            validator.accept_frame(
                frame(
                    1,
                    r#"{"type":"response_failed","code":"rate_limited","retryable":true,"retry_after_ms":60001}"#,
                )
                .as_bytes()
            ),
            Err(GatewayProtocolError::RetryDelayTooLarge {
                maximum_ms: MAX_RETRY_AFTER_MS,
                actual_ms: 60_001,
            })
        );
        assert_eq!(
            validator.accept_frame(
                frame(
                    1,
                    r#"{"type":"response_failed","code":"internal","retryable":false,"message":"raw provider detail"}"#,
                )
                .as_bytes()
            ),
            Err(GatewayProtocolError::MalformedEvent)
        );
        assert_eq!(
            validator.accept_frame(
                frame(
                    1,
                    r#"{"type":"response_failed","code":"provider_secret","retryable":false}"#,
                )
                .as_bytes()
            ),
            Err(GatewayProtocolError::MalformedEvent)
        );
        assert_eq!(validator.status(), GatewayStreamStatus::Streaming);
        assert_eq!(validator.event_count(), 1);
        Ok(())
    }

    #[test]
    fn event_debug_redacts_output_and_function_arguments() -> Result<(), Box<dyn Error>> {
        let output_sentinel = "private-model-output";
        let mut text_validator = validator()?;
        start(&mut text_validator)?;
        let output_event = text_validator.accept_frame(
            frame(
                1,
                &format!(r#"{{"type":"output_text_delta","delta":"{output_sentinel}"}}"#),
            )
            .as_bytes(),
        )?;
        let output_debug = format!("{output_event:?}");

        assert!(!output_debug.contains(output_sentinel));
        assert!(output_debug.contains("[REDACTED]"));

        let argument_sentinel = "private-task-title";
        let raw_arguments = format!(r#"{{"title":"{argument_sentinel}"}}"#);
        let mut function_validator = validator()?;
        start(&mut function_validator)?;
        let function = function_event(
            "call-private-1",
            TOOL_NAME,
            TOOL_CONTRACT_VERSION,
            &raw_arguments,
        )?;
        let function_event = function_validator.accept_frame(frame(1, &function).as_bytes())?;
        let function_debug = format!("{function_event:?}");

        assert!(!function_debug.contains(argument_sentinel));
        assert!(!function_debug.contains(&raw_arguments));
        assert!(function_debug.contains("[REDACTED]"));

        let ValidatedGatewayEvent::FunctionCallCompleted { call } = function_event else {
            return Err(GatewayProtocolError::MalformedEvent.into());
        };
        let call_debug = format!("{call:?}");
        assert!(!call_debug.contains(argument_sentinel));
        assert!(!call_debug.contains(&raw_arguments));
        Ok(())
    }

    #[test]
    fn error_display_and_debug_never_retain_untrusted_content() -> GatewayProtocolResult<()> {
        let mut validator = validator()?;
        let untrusted = r#"{"protocol_version":1,"run_id":"run-1","gateway_request_id":"gateway-request-1","sequence":0,"event":{"type":"response_started","provider_response_id":"provider-response-1","api_key":"sk-do-not-retain","arguments_json":"private-content"}}"#;
        let error = validator.accept_frame(untrusted.as_bytes()).err();
        let Some(error) = error else {
            return Err(GatewayProtocolError::MalformedEvent);
        };
        let display = error.to_string();
        let debug = format!("{error:?}");

        for secret in ["sk-do-not-retain", "private-content", "api_key"] {
            assert!(!display.contains(secret));
            assert!(!debug.contains(secret));
        }
        assert_eq!(error, GatewayProtocolError::MalformedEvent);
        Ok(())
    }
}
