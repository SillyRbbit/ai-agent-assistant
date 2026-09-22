//! Application-owned agent runtime foundation.
//!
//! This module describes only the in-process, one-turn behavior that exists
//! today. Provider transport, sessions, tool execution, policy, approval,
//! audit, persistence, and framework-specific lifecycle remain separate.

use std::fmt;

use thiserror::Error;

use super::gateway_protocol::{
    is_valid_opaque_id, MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN, MAX_FUNCTION_ARGUMENT_BYTES,
    MAX_GATEWAY_REQUEST_BYTES, MAX_OPAQUE_ID_BYTES, MAX_RETRY_AFTER_MS,
};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub const MAX_RUNTIME_OUTPUT_TEXT_BYTES: usize = MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN;

pub(crate) const PERSONAL_ASSISTANT_V0_SYNTHETIC_FIXTURE: &str = "Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed.";

/// Constructs one bounded runtime run from application-owned input.
pub trait AgentRuntime {
    type Run: RuntimeRun;

    fn describe(&self) -> RuntimeDescriptor;

    fn start(&self, request: RuntimeTurnRequest) -> RuntimeResult<Self::Run>;
}

/// Common event sink and lifecycle exposed by a single bounded runtime run.
///
/// The sink accepts only application-owned untrusted event types. Current
/// native policy- and approval-owned results remain on the concrete adapter.
pub trait RuntimeRun {
    fn run_id(&self) -> &RuntimeRunId;

    fn identity(&self) -> &RuntimeRunIdentity;

    fn status(&self) -> RuntimeRunStatus;

    /// Accepts one closed application-owned event from an untrusted runtime.
    fn accept_event(
        &mut self,
        event: RuntimeEventEnvelope,
    ) -> RuntimeResult<RuntimeEventAcceptance>;

    /// Terminates the local run exactly and idempotently.
    ///
    /// Implementations may close run-owned pending governance state through
    /// existing typed cleanup, but no approval or audit data enters this API.
    fn cancel(&mut self) -> RuntimeResult<RuntimeCancellationOutcome>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeId {
    Native,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeAvailability {
    Available,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeHealth {
    Healthy,
    Unhealthy,
}

/// Closed capabilities currently verified at the local native boundary.
///
/// These values report accepted event shapes; they do not grant tool,
/// execution, approval, provider, model, network, or platform authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeCapability {
    StreamingText,
    UntrustedToolProposals,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeCapabilities {
    streaming_text: bool,
    untrusted_tool_proposals: bool,
}

impl RuntimeCapabilities {
    #[must_use]
    pub const fn new(streaming_text: bool, untrusted_tool_proposals: bool) -> Self {
        Self {
            streaming_text,
            untrusted_tool_proposals,
        }
    }

    #[must_use]
    pub const fn supports(self, capability: RuntimeCapability) -> bool {
        match capability {
            RuntimeCapability::StreamingText => self.streaming_text,
            RuntimeCapability::UntrustedToolProposals => self.untrusted_tool_proposals,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeDescriptor {
    id: RuntimeId,
    availability: RuntimeAvailability,
    health: RuntimeHealth,
    capabilities: RuntimeCapabilities,
}

impl RuntimeDescriptor {
    #[must_use]
    pub const fn new(
        id: RuntimeId,
        availability: RuntimeAvailability,
        health: RuntimeHealth,
        capabilities: RuntimeCapabilities,
    ) -> Self {
        Self {
            id,
            availability,
            health,
            capabilities,
        }
    }

    #[must_use]
    pub const fn id(self) -> RuntimeId {
        self.id
    }

    #[must_use]
    pub const fn availability(self) -> RuntimeAvailability {
        self.availability
    }

    #[must_use]
    pub const fn health(self) -> RuntimeHealth {
        self.health
    }

    #[must_use]
    pub const fn capabilities(self) -> RuntimeCapabilities {
        self.capabilities
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RuntimeRunId(String);

impl RuntimeRunId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new(value: String) -> RuntimeResult<Self> {
        if is_valid_opaque_id(&value) {
            Ok(Self(value))
        } else {
            Err(RuntimeError::InvalidRequest(RuntimeInvalidRequest::RunId))
        }
    }
}

impl fmt::Debug for RuntimeRunId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("RuntimeRunId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RuntimeRequestId(String);

impl RuntimeRequestId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new(value: String) -> RuntimeResult<Self> {
        if is_valid_opaque_id(&value) {
            Ok(Self(value))
        } else {
            Err(RuntimeError::InvalidRequest(
                RuntimeInvalidRequest::RequestId,
            ))
        }
    }
}

impl fmt::Debug for RuntimeRequestId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("RuntimeRequestId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Eq, PartialEq)]
pub struct RuntimeSelectedText(String);

impl RuntimeSelectedText {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new(value: String) -> RuntimeResult<Self> {
        if value.trim().is_empty() {
            return Err(RuntimeError::InvalidRequest(
                RuntimeInvalidRequest::EmptySelectedContent,
            ));
        }
        if value.len() > MAX_GATEWAY_REQUEST_BYTES {
            return Err(RuntimeError::InvalidRequest(
                RuntimeInvalidRequest::SelectedContentTooLarge {
                    maximum_bytes: MAX_GATEWAY_REQUEST_BYTES,
                    actual_bytes: value.len(),
                },
            ));
        }
        Ok(Self(value))
    }

    pub(super) fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Debug for RuntimeSelectedText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeSelectedText")
            .field("text", &"[REDACTED]")
            .field("bytes", &self.0.len())
            .finish()
    }
}

#[derive(Eq, PartialEq)]
pub struct RuntimeTurnRequest {
    identity: RuntimeRunIdentity,
    selected_text: RuntimeSelectedText,
    profile: RuntimeTurnProfile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RuntimeTurnProfile {
    Initial,
    PersonalAssistantV0Synthetic,
    PersonalAssistantDirectSynthetic,
}

impl RuntimeTurnRequest {
    pub(crate) fn personal_assistant_direct(
        run_id: String,
        request_id: String,
    ) -> RuntimeResult<Self> {
        let mut request = Self::personal_assistant_v0_synthetic(run_id, request_id)?;
        request.profile = RuntimeTurnProfile::PersonalAssistantDirectSynthetic;
        Ok(request)
    }
    pub fn new(
        run_id: impl Into<String>,
        request_id: impl Into<String>,
        selected_text: impl Into<String>,
    ) -> RuntimeResult<Self> {
        Ok(Self {
            identity: RuntimeRunIdentity::new(run_id.into(), request_id.into())?,
            selected_text: RuntimeSelectedText::new(selected_text.into())?,
            profile: RuntimeTurnProfile::Initial,
        })
    }

    pub(crate) fn personal_assistant_v0_synthetic(
        run_id: String,
        request_id: String,
    ) -> RuntimeResult<Self> {
        Ok(Self {
            identity: RuntimeRunIdentity::new(run_id, request_id)?,
            selected_text: RuntimeSelectedText::new(
                PERSONAL_ASSISTANT_V0_SYNTHETIC_FIXTURE.to_owned(),
            )?,
            profile: RuntimeTurnProfile::PersonalAssistantV0Synthetic,
        })
    }

    #[must_use]
    pub fn run_id(&self) -> &RuntimeRunId {
        self.identity.run_id()
    }

    #[must_use]
    pub fn request_id(&self) -> &RuntimeRequestId {
        self.identity.request_id()
    }

    #[must_use]
    pub fn selected_text(&self) -> &RuntimeSelectedText {
        &self.selected_text
    }

    #[must_use]
    pub fn identity(&self) -> RuntimeRunIdentity {
        self.identity.clone()
    }

    pub(super) fn into_parts(
        self,
    ) -> (RuntimeRunIdentity, RuntimeSelectedText, RuntimeTurnProfile) {
        (self.identity, self.selected_text, self.profile)
    }
}

impl fmt::Debug for RuntimeTurnRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeTurnRequest")
            .field("identity", &self.identity)
            .field("selected_text", &"[REDACTED]")
            .field("selected_text_bytes", &self.selected_text.0.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RuntimeRunIdentity {
    run_id: RuntimeRunId,
    request_id: RuntimeRequestId,
}

impl RuntimeRunIdentity {
    fn new(run_id: String, request_id: String) -> RuntimeResult<Self> {
        Ok(Self {
            run_id: RuntimeRunId::new(run_id)?,
            request_id: RuntimeRequestId::new(request_id)?,
        })
    }

    #[must_use]
    pub fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    #[must_use]
    pub fn request_id(&self) -> &RuntimeRequestId {
        &self.request_id
    }
}

impl fmt::Debug for RuntimeRunIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeRunIdentity")
            .field("run_id", &"[REDACTED]")
            .field("request_id", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RuntimeResponseId(String);

impl RuntimeResponseId {
    pub fn new(value: impl Into<String>) -> RuntimeResult<Self> {
        let value = value.into();
        if is_valid_opaque_id(&value) {
            Ok(Self(value))
        } else {
            Err(RuntimeError::InvalidEvent(RuntimeInvalidEvent::ResponseId))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(super) fn from_validated(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Debug for RuntimeResponseId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("RuntimeResponseId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RuntimeOutputText(String);

impl RuntimeOutputText {
    pub fn new(value: impl Into<String>) -> RuntimeResult<Self> {
        let value = value.into();
        if value.is_empty() {
            return Err(RuntimeError::InvalidEvent(
                RuntimeInvalidEvent::EmptyOutputText,
            ));
        }
        let actual_characters = value.chars().count();
        if actual_characters > MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN {
            return Err(RuntimeError::InvalidEvent(
                RuntimeInvalidEvent::OutputTextTooLarge {
                    maximum_characters: MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN,
                    actual_characters,
                },
            ));
        }
        if value.len() > MAX_RUNTIME_OUTPUT_TEXT_BYTES {
            return Err(RuntimeError::InvalidEvent(
                RuntimeInvalidEvent::OutputTextTooManyBytes {
                    maximum_bytes: MAX_RUNTIME_OUTPUT_TEXT_BYTES,
                    actual_bytes: value.len(),
                },
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(super) fn from_validated(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Debug for RuntimeOutputText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeOutputText")
            .field("text", &"[REDACTED]")
            .field("characters", &self.0.chars().count())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct UntrustedRuntimeToolProposal {
    call_id: String,
    name: String,
    tool_contract_version: u16,
    arguments_json: String,
}

impl UntrustedRuntimeToolProposal {
    pub fn new(
        call_id: impl Into<String>,
        name: impl Into<String>,
        tool_contract_version: u16,
        arguments_json: impl Into<String>,
    ) -> RuntimeResult<Self> {
        let call_id = call_id.into();
        if !is_valid_opaque_id(&call_id) {
            return Err(RuntimeError::InvalidEvent(RuntimeInvalidEvent::ToolCallId));
        }

        let name = name.into();
        if name.is_empty()
            || name.len() > MAX_OPAQUE_ID_BYTES
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err(RuntimeError::InvalidEvent(RuntimeInvalidEvent::ToolName));
        }
        if tool_contract_version == 0 {
            return Err(RuntimeError::InvalidEvent(
                RuntimeInvalidEvent::ToolContractVersion,
            ));
        }

        let arguments_json = arguments_json.into();
        if arguments_json.len() > MAX_FUNCTION_ARGUMENT_BYTES {
            return Err(RuntimeError::InvalidEvent(
                RuntimeInvalidEvent::ToolArgumentsTooLarge {
                    maximum_bytes: MAX_FUNCTION_ARGUMENT_BYTES,
                    actual_bytes: arguments_json.len(),
                },
            ));
        }

        Ok(Self {
            call_id,
            name,
            tool_contract_version,
            arguments_json,
        })
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
    pub const fn tool_contract_version(&self) -> u16 {
        self.tool_contract_version
    }

    #[must_use]
    pub fn arguments_json(&self) -> &str {
        &self.arguments_json
    }
}

impl fmt::Debug for UntrustedRuntimeToolProposal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UntrustedRuntimeToolProposal")
            .field("call_id", &"[REDACTED]")
            .field("name", &self.name)
            .field("tool_contract_version", &self.tool_contract_version)
            .field("arguments_json", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeFailureCode {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeFailure {
    code: RuntimeFailureCode,
    retryable: bool,
    retry_after_ms: Option<u64>,
}

impl RuntimeFailure {
    pub fn new(
        code: RuntimeFailureCode,
        retryable: bool,
        retry_after_ms: Option<u64>,
    ) -> RuntimeResult<Self> {
        if retry_after_ms.is_some_and(|delay| delay > MAX_RETRY_AFTER_MS) {
            return Err(RuntimeError::InvalidEvent(
                RuntimeInvalidEvent::RetryDelayTooLarge {
                    maximum_ms: MAX_RETRY_AFTER_MS,
                },
            ));
        }
        Ok(Self {
            code,
            retryable,
            retry_after_ms,
        })
    }

    #[must_use]
    pub const fn code(self) -> RuntimeFailureCode {
        self.code
    }

    #[must_use]
    pub const fn retryable(self) -> bool {
        self.retryable
    }

    #[must_use]
    pub const fn retry_after_ms(self) -> Option<u64> {
        self.retry_after_ms
    }

    pub(super) const fn from_validated(
        code: RuntimeFailureCode,
        retryable: bool,
        retry_after_ms: Option<u64>,
    ) -> Self {
        Self {
            code,
            retryable,
            retry_after_ms,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UntrustedRuntimeEvent {
    ResponseStarted {
        response_id: RuntimeResponseId,
    },
    OutputTextDelta {
        delta: RuntimeOutputText,
    },
    ToolProposal {
        proposal: UntrustedRuntimeToolProposal,
    },
    ResponseCompleted,
    ResponseFailed {
        failure: RuntimeFailure,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeEventEnvelope {
    run_id: RuntimeRunId,
    request_id: RuntimeRequestId,
    sequence: u32,
    event: UntrustedRuntimeEvent,
}

impl RuntimeEventEnvelope {
    #[must_use]
    pub fn for_request(
        request: &RuntimeTurnRequest,
        sequence: u32,
        event: UntrustedRuntimeEvent,
    ) -> Self {
        Self::for_identity(&request.identity, sequence, event)
    }

    #[must_use]
    pub fn for_identity(
        identity: &RuntimeRunIdentity,
        sequence: u32,
        event: UntrustedRuntimeEvent,
    ) -> Self {
        Self {
            run_id: identity.run_id.clone(),
            request_id: identity.request_id.clone(),
            sequence,
            event,
        }
    }

    pub fn into_parts(self) -> (RuntimeRunId, RuntimeRequestId, u32, UntrustedRuntimeEvent) {
        (self.run_id, self.request_id, self.sequence, self.event)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeEventAcceptance {
    ResponseStarted {
        response_id: RuntimeResponseId,
    },
    OutputTextDelta {
        delta: RuntimeOutputText,
    },
    ToolProposal {
        proposal: UntrustedRuntimeToolProposal,
    },
    ResponseCompleted,
    ResponseFailed {
        failure: RuntimeFailure,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeRunStatus {
    AwaitingStart,
    Streaming,
    Completed,
    Failed,
    Cancelled,
}

impl RuntimeRunStatus {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeCancellationOutcome {
    Cancelled,
    AlreadyTerminal(RuntimeRunStatus),
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum RuntimeInvalidRequest {
    #[error("run id is invalid")]
    RunId,
    #[error("request id is invalid")]
    RequestId,
    #[error("selected content must contain a non-whitespace character")]
    EmptySelectedContent,
    #[error("selected content exceeds {maximum_bytes} bytes")]
    SelectedContentTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("serialized request exceeds {maximum_bytes} bytes")]
    SerializedRequestTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum RuntimeInvalidEvent {
    #[error("response id is invalid")]
    ResponseId,
    #[error("output text must not be empty")]
    EmptyOutputText,
    #[error("output text exceeds {maximum_characters} characters")]
    OutputTextTooLarge {
        maximum_characters: usize,
        actual_characters: usize,
    },
    #[error("output text exceeds {maximum_bytes} bytes")]
    OutputTextTooManyBytes {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("tool call id is invalid")]
    ToolCallId,
    #[error("tool name is invalid")]
    ToolName,
    #[error("tool contract version must be greater than zero")]
    ToolContractVersion,
    #[error("tool arguments exceed {maximum_bytes} bytes")]
    ToolArgumentsTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("runtime failure retry delay exceeds {maximum_ms} milliseconds")]
    RetryDelayTooLarge { maximum_ms: u64 },
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum RuntimeEventRejection {
    #[error("event identity does not match the active run")]
    IdentityMismatch,
    #[error("event sequence is invalid")]
    InvalidSequence,
    #[error("event is invalid for the current lifecycle state")]
    InvalidState,
    #[error("event content is invalid")]
    InvalidContent,
    #[error("event exceeds a closed runtime limit")]
    LimitExceeded,
    #[error("event uses an unsupported protocol version")]
    UnsupportedVersion,
    #[error("event serialization or framing is malformed")]
    Malformed,
    #[error("concrete native frames and shared runtime events cannot be mixed")]
    MixedInputSurface,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeBoundaryStage {
    Start,
    RequestSerialization,
    NativeConfiguration,
    EventTranslation,
    EventAcceptance,
    GovernanceIsolation,
    Cancellation,
}

impl fmt::Display for RuntimeBoundaryStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Start => "start",
            Self::RequestSerialization => "request serialization",
            Self::NativeConfiguration => "native configuration",
            Self::EventTranslation => "event translation",
            Self::EventAcceptance => "event acceptance",
            Self::GovernanceIsolation => "governance isolation",
            Self::Cancellation => "cancellation",
        };
        formatter.write_str(label)
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum RuntimeError {
    #[error("runtime is unavailable")]
    Unavailable,
    #[error("runtime request is invalid: {0}")]
    InvalidRequest(RuntimeInvalidRequest),
    #[error("runtime event is invalid: {0}")]
    InvalidEvent(RuntimeInvalidEvent),
    #[error("runtime event was rejected: {0}")]
    EventRejected(RuntimeEventRejection),
    #[error("runtime capability is unavailable: {0:?}")]
    CapabilityUnavailable(RuntimeCapability),
    #[error("runtime boundary failed during {0}")]
    BoundaryFailure(RuntimeBoundaryStage),
}

#[cfg(test)]
mod tests {
    use super::{
        RuntimeError, RuntimeInvalidRequest, RuntimeTurnProfile, RuntimeTurnRequest,
        PERSONAL_ASSISTANT_V0_SYNTHETIC_FIXTURE,
    };

    #[test]
    fn public_request_constructor_keeps_the_initial_profile() -> Result<(), RuntimeError> {
        let request = RuntimeTurnRequest::new("runtime-run-1", "runtime-request-1", "Plan")?;
        let (identity, selected_text, profile) = request.into_parts();

        assert_eq!(profile, RuntimeTurnProfile::Initial);
        assert_eq!(identity.run_id().as_str(), "runtime-run-1");
        assert_eq!(identity.request_id().as_str(), "runtime-request-1");
        assert_eq!(selected_text.as_str(), "Plan");
        Ok(())
    }

    #[test]
    fn personal_assistant_factory_accepts_only_identity_and_pins_fixture_and_profile(
    ) -> Result<(), RuntimeError> {
        let request = RuntimeTurnRequest::personal_assistant_v0_synthetic(
            "pa-v0-run-0000000000000001".to_owned(),
            "pa-v0-request-0000000000000001".to_owned(),
        )?;
        let debug = format!("{request:?}");
        let (identity, selected_text, profile) = request.into_parts();

        assert_eq!(profile, RuntimeTurnProfile::PersonalAssistantV0Synthetic);
        assert_eq!(identity.run_id().as_str(), "pa-v0-run-0000000000000001");
        assert_eq!(
            identity.request_id().as_str(),
            "pa-v0-request-0000000000000001"
        );
        assert_eq!(
            selected_text.as_str(),
            PERSONAL_ASSISTANT_V0_SYNTHETIC_FIXTURE
        );
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(PERSONAL_ASSISTANT_V0_SYNTHETIC_FIXTURE));
        assert!(!debug.contains("pa-v0-run-0000000000000001"));
        assert!(!debug.contains("pa-v0-request-0000000000000001"));
        Ok(())
    }

    #[test]
    fn personal_assistant_factory_rejects_invalid_and_oversize_identity() {
        assert_eq!(
            RuntimeTurnRequest::personal_assistant_v0_synthetic(
                "bad run".to_owned(),
                "valid-request".to_owned(),
            ),
            Err(RuntimeError::InvalidRequest(RuntimeInvalidRequest::RunId))
        );
        assert_eq!(
            RuntimeTurnRequest::personal_assistant_v0_synthetic(
                "valid-run".to_owned(),
                "r".repeat(129),
            ),
            Err(RuntimeError::InvalidRequest(
                RuntimeInvalidRequest::RequestId
            ))
        );

        let exact = "r".repeat(128);
        assert!(RuntimeTurnRequest::personal_assistant_v0_synthetic(exact.clone(), exact).is_ok());
    }
}
