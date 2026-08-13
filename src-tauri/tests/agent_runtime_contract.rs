use std::error::Error;

use ai_agent_assistant_lib::agent::gateway_protocol::{
    GatewayFailureCode, GatewayProtocolError, GatewayStreamStatus, GATEWAY_PROTOCOL_VERSION,
    MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN, MAX_GATEWAY_REQUEST_BYTES,
};
use ai_agent_assistant_lib::agent::gateway_request::{
    InitialGatewayEvent, InitialGatewayTurn, InitialGatewayTurnError,
    INITIAL_GATEWAY_TOOL_SET_VERSION,
};
use ai_agent_assistant_lib::agent::native_runtime::{NativeAgentRunError, NativeAgentRuntime};
use ai_agent_assistant_lib::agent::runtime::{
    AgentRuntime, RuntimeAvailability, RuntimeBoundaryStage, RuntimeCancellationOutcome,
    RuntimeCapability, RuntimeError, RuntimeEventAcceptance, RuntimeEventEnvelope,
    RuntimeEventRejection, RuntimeFailure, RuntimeFailureCode, RuntimeHealth, RuntimeId,
    RuntimeInvalidEvent, RuntimeInvalidRequest, RuntimeOutputText, RuntimeResponseId, RuntimeRun,
    RuntimeRunStatus, RuntimeTurnRequest, UntrustedRuntimeEvent, UntrustedRuntimeToolProposal,
    MAX_RUNTIME_OUTPUT_TEXT_BYTES,
};
use ai_agent_assistant_lib::policy::types::PolicyOutcome;
use serde_json::{json, Value};

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

const RUN_ID: &str = "runtime-contract-run-1";
const REQUEST_ID: &str = "runtime-contract-request-1";
const SELECTED_TEXT: &str = "Plan my day";

fn frame_with_identity(run_id: &str, request_id: &str, sequence: u32, event: Value) -> Vec<u8> {
    json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": run_id,
        "gateway_request_id": request_id,
        "sequence": sequence,
        "event": event,
    })
    .to_string()
    .into_bytes()
}

fn frame(sequence: u32, event: Value) -> Vec<u8> {
    frame_with_identity(RUN_ID, REQUEST_ID, sequence, event)
}

fn start_frame() -> Vec<u8> {
    frame(
        0,
        json!({
            "type": "response_started",
            "provider_response_id": "runtime-contract-response-1",
        }),
    )
}

fn text_frame(sequence: u32, delta: &str) -> Vec<u8> {
    frame(
        sequence,
        json!({
            "type": "output_text_delta",
            "delta": delta,
        }),
    )
}

fn function_frame(sequence: u32, name: &str, arguments_json: &str) -> Vec<u8> {
    frame(
        sequence,
        json!({
            "type": "function_call_completed",
            "call_id": "runtime-contract-call-1",
            "name": name,
            "tool_contract_version": INITIAL_GATEWAY_TOOL_SET_VERSION,
            "arguments_json": arguments_json,
        }),
    )
}

fn completed_frame(sequence: u32) -> Vec<u8> {
    frame(sequence, json!({ "type": "response_completed" }))
}

fn failed_frame(sequence: u32) -> Vec<u8> {
    frame(
        sequence,
        json!({
            "type": "response_failed",
            "code": "provider_timeout",
            "retryable": true,
            "retry_after_ms": 1000,
        }),
    )
}

fn request() -> Result<RuntimeTurnRequest, RuntimeError> {
    RuntimeTurnRequest::new(RUN_ID, REQUEST_ID, SELECTED_TEXT)
}

#[test]
fn native_descriptor_is_closed_available_and_locally_healthy() {
    let descriptor = NativeAgentRuntime.describe();

    assert_eq!(descriptor.id(), RuntimeId::Native);
    assert_eq!(descriptor.availability(), RuntimeAvailability::Available);
    assert_eq!(descriptor.health(), RuntimeHealth::Healthy);
    assert!(descriptor
        .capabilities()
        .supports(RuntimeCapability::StreamingText));
    assert!(!descriptor
        .capabilities()
        .supports(RuntimeCapability::UntrustedToolProposals));
}

#[test]
fn runtime_request_uses_closed_validation_and_redacted_debug() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        RuntimeTurnRequest::new("", REQUEST_ID, SELECTED_TEXT),
        Err(RuntimeError::InvalidRequest(RuntimeInvalidRequest::RunId))
    );
    assert_eq!(
        RuntimeTurnRequest::new(RUN_ID, "request id with spaces", SELECTED_TEXT),
        Err(RuntimeError::InvalidRequest(
            RuntimeInvalidRequest::RequestId
        ))
    );
    let oversized_id = "r".repeat(129);
    assert_eq!(
        RuntimeTurnRequest::new(&oversized_id, REQUEST_ID, SELECTED_TEXT),
        Err(RuntimeError::InvalidRequest(RuntimeInvalidRequest::RunId))
    );
    assert_eq!(
        RuntimeTurnRequest::new(RUN_ID, &oversized_id, SELECTED_TEXT),
        Err(RuntimeError::InvalidRequest(
            RuntimeInvalidRequest::RequestId
        ))
    );
    assert_eq!(
        RuntimeTurnRequest::new(RUN_ID, REQUEST_ID, "   "),
        Err(RuntimeError::InvalidRequest(
            RuntimeInvalidRequest::EmptySelectedContent
        ))
    );

    let oversized = "x".repeat(MAX_GATEWAY_REQUEST_BYTES + 1);
    assert!(matches!(
        RuntimeTurnRequest::new(RUN_ID, REQUEST_ID, oversized),
        Err(RuntimeError::InvalidRequest(
            RuntimeInvalidRequest::SelectedContentTooLarge { .. }
        ))
    ));
    assert!(matches!(
        RuntimeOutputText::new("x".repeat(MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN + 1)),
        Err(RuntimeError::InvalidEvent(
            RuntimeInvalidEvent::OutputTextTooLarge { .. }
        ))
    ));

    let sentinel = "runtime-private-selected-text-sentinel";
    let run_sentinel = "runtime-private-run-id-sentinel";
    let request_sentinel = "runtime-private-request-id-sentinel";
    let request = RuntimeTurnRequest::new(run_sentinel, request_sentinel, sentinel)?;
    let debug = format!("{request:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains(sentinel));
    assert!(!debug.contains(run_sentinel));
    assert!(!debug.contains(request_sentinel));

    let arguments_sentinel = "runtime-private-arguments-sentinel";
    let proposal = UntrustedRuntimeToolProposal::new(
        "runtime-private-call-id-sentinel",
        "create_local_task",
        INITIAL_GATEWAY_TOOL_SET_VERSION,
        format!(r#"{{"title":"{arguments_sentinel}"}}"#),
    )?;
    let proposal_debug = format!("{proposal:?}");
    assert!(proposal_debug.contains("[REDACTED]"));
    assert!(!proposal_debug.contains(arguments_sentinel));
    assert!(!proposal_debug.contains("runtime-private-call-id-sentinel"));
    Ok(())
}

#[test]
fn runtime_event_types_are_bounded_closed_and_redacted() -> Result<(), Box<dyn Error>> {
    assert!(RuntimeResponseId::new("").is_err());
    assert!(RuntimeResponseId::new("r".repeat(129)).is_err());
    assert!(UntrustedRuntimeToolProposal::new("", "create_local_task", 1, "{}").is_err());
    assert!(UntrustedRuntimeToolProposal::new("call-1", "bad name", 1, "{}").is_err());
    assert!(UntrustedRuntimeToolProposal::new("call-1", "create_local_task", 0, "{}").is_err());
    assert!(
        UntrustedRuntimeToolProposal::new("call-1", "create_local_task", 1, "x".repeat(8_193),)
            .is_err()
    );
    assert_eq!(
        RuntimeFailure::new(RuntimeFailureCode::RateLimited, true, Some(60_001),),
        Err(RuntimeError::InvalidEvent(
            RuntimeInvalidEvent::RetryDelayTooLarge { maximum_ms: 60_000 }
        ))
    );

    let exact_unicode = RuntimeOutputText::new("é".repeat(MAX_RUNTIME_OUTPUT_TEXT_BYTES / 2))?;
    assert_eq!(exact_unicode.as_str().len(), MAX_RUNTIME_OUTPUT_TEXT_BYTES);
    assert!(matches!(
        RuntimeOutputText::new("é".repeat((MAX_RUNTIME_OUTPUT_TEXT_BYTES / 2) + 1)),
        Err(RuntimeError::InvalidEvent(
            RuntimeInvalidEvent::OutputTextTooManyBytes { .. }
        ))
    ));
    assert!(matches!(
        RuntimeOutputText::new("x".repeat(MAX_ASSISTANT_OUTPUT_CHARACTERS_PER_TURN + 1)),
        Err(RuntimeError::InvalidEvent(
            RuntimeInvalidEvent::OutputTextTooLarge { .. }
        ))
    ));

    let request = RuntimeTurnRequest::new(
        "runtime-debug-run-sentinel",
        "runtime-debug-request-sentinel",
        SELECTED_TEXT,
    )?;
    let envelope = RuntimeEventEnvelope::for_request(
        &request,
        0,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new("runtime-debug-output-sentinel")?,
        },
    );
    let debug = format!("{envelope:?}");
    assert!(!debug.contains("runtime-debug-run-sentinel"));
    assert!(!debug.contains("runtime-debug-request-sentinel"));
    assert!(!debug.contains("runtime-debug-output-sentinel"));
    assert!(debug.contains("[REDACTED]"));
    Ok(())
}

#[test]
fn every_runtime_failure_code_round_trips_through_native() -> Result<(), Box<dyn Error>> {
    let codes = [
        RuntimeFailureCode::Unauthenticated,
        RuntimeFailureCode::Forbidden,
        RuntimeFailureCode::RateLimited,
        RuntimeFailureCode::RequestRejected,
        RuntimeFailureCode::ProviderUnavailable,
        RuntimeFailureCode::ProviderTimeout,
        RuntimeFailureCode::ProtocolViolation,
        RuntimeFailureCode::LimitExceeded,
        RuntimeFailureCode::Cancelled,
        RuntimeFailureCode::Internal,
    ];

    for code in codes {
        let request = request()?;
        let started = RuntimeEventEnvelope::for_request(
            &request,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("failure-code-response-1")?,
            },
        );
        let failed = RuntimeEventEnvelope::for_request(
            &request,
            1,
            UntrustedRuntimeEvent::ResponseFailed {
                failure: RuntimeFailure::new(code, false, None)?,
            },
        );
        let mut run = NativeAgentRuntime.start(request)?;
        run.accept_event(started)?;
        match run.accept_event(failed)? {
            RuntimeEventAcceptance::ResponseFailed { failure } => {
                assert_eq!(failure.code(), code);
            }
            _ => return Err("expected closed runtime failure".into()),
        }
    }
    Ok(())
}

#[test]
fn native_start_delegates_exact_request_construction() -> Result<(), Box<dyn Error>> {
    let runtime = NativeAgentRuntime;
    let run = runtime.start(request()?)?;
    let direct = InitialGatewayTurn::new(RUN_ID, REQUEST_ID, SELECTED_TEXT)?;

    assert_eq!(run.run_id().as_str(), RUN_ID);
    assert_eq!(run.status(), RuntimeRunStatus::AwaitingStart);
    assert_eq!(run.request_bytes(), direct.request_bytes());

    let body: Value = serde_json::from_slice(run.request_bytes())?;
    assert_eq!(body["run_id"], RUN_ID);
    assert_eq!(body["gateway_request_id"], REQUEST_ID);
    assert_eq!(body["input"]["text"], SELECTED_TEXT);
    assert!(body.get("provider").is_none());
    assert!(body.get("model").is_none());
    assert!(body.get("execute").is_none());
    let debug = format!("{run:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains(RUN_ID));
    assert!(!debug.contains(SELECTED_TEXT));
    Ok(())
}

#[test]
fn native_shared_event_contract_preserves_text_and_terminal_lifecycle() -> Result<(), Box<dyn Error>>
{
    let request = request()?;
    let mut run = NativeAgentRuntime.start(request)?;
    let identity = run.identity().clone();
    let started = RuntimeEventEnvelope::for_identity(
        &identity,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("runtime-contract-response-1")?,
        },
    );
    let text = RuntimeEventEnvelope::for_identity(
        &identity,
        1,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new("A bounded response")?,
        },
    );
    let completed =
        RuntimeEventEnvelope::for_identity(&identity, 2, UntrustedRuntimeEvent::ResponseCompleted);

    assert!(matches!(
        run.accept_event(started)?,
        RuntimeEventAcceptance::ResponseStarted { .. }
    ));
    assert!(matches!(
        run.accept_event(text)?,
        RuntimeEventAcceptance::OutputTextDelta { delta }
            if delta.as_str() == "A bounded response"
    ));
    assert_eq!(
        run.accept_event(completed)?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(run.status(), RuntimeRunStatus::Completed);
    assert_eq!(
        run.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Completed)
    );
    Ok(())
}

#[test]
fn native_shared_failure_is_closed_typed_and_terminal() -> Result<(), Box<dyn Error>> {
    let request = request()?;
    let started = RuntimeEventEnvelope::for_request(
        &request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("runtime-contract-response-1")?,
        },
    );
    let failed = RuntimeEventEnvelope::for_request(
        &request,
        1,
        UntrustedRuntimeEvent::ResponseFailed {
            failure: RuntimeFailure::new(RuntimeFailureCode::ProviderTimeout, true, Some(1000))?,
        },
    );
    let mut run = NativeAgentRuntime.start(request)?;

    run.accept_event(started)?;
    match run.accept_event(failed)? {
        RuntimeEventAcceptance::ResponseFailed { failure } => {
            assert_eq!(failure.code(), RuntimeFailureCode::ProviderTimeout);
            assert!(failure.retryable());
            assert_eq!(failure.retry_after_ms(), Some(1000));
        }
        _ => return Err("expected closed runtime failure".into()),
    }
    assert_eq!(run.status(), RuntimeRunStatus::Failed);
    assert_eq!(
        run.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Failed)
    );
    Ok(())
}

#[test]
fn native_shared_events_reject_identity_and_sequence_without_state_change(
) -> Result<(), Box<dyn Error>> {
    let request = request()?;
    let foreign_request = RuntimeTurnRequest::new("foreign-run", REQUEST_ID, SELECTED_TEXT)?;
    let foreign = RuntimeEventEnvelope::for_request(
        &foreign_request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("runtime-contract-response-1")?,
        },
    );
    let out_of_order = RuntimeEventEnvelope::for_request(
        &request,
        1,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("runtime-contract-response-1")?,
        },
    );
    let started = RuntimeEventEnvelope::for_request(
        &request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("runtime-contract-response-1")?,
        },
    );
    let duplicate_sequence = RuntimeEventEnvelope::for_request(
        &request,
        0,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new("duplicate")?,
        },
    );
    let mut run = NativeAgentRuntime.start(request)?;

    assert_eq!(
        run.accept_event(foreign),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::IdentityMismatch
        ))
    );
    assert_eq!(run.status(), RuntimeRunStatus::AwaitingStart);
    assert_eq!(
        run.accept_event(out_of_order),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidSequence
        ))
    );
    assert_eq!(run.status(), RuntimeRunStatus::AwaitingStart);
    run.accept_event(started)?;
    assert_eq!(
        run.accept_event(duplicate_sequence),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidSequence
        ))
    );
    assert_eq!(run.status(), RuntimeRunStatus::Streaming);
    Ok(())
}

#[test]
fn native_run_rejects_mixing_shared_events_and_concrete_frames() -> Result<(), Box<dyn Error>> {
    let shared_request = request()?;
    let shared_start = RuntimeEventEnvelope::for_request(
        &shared_request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("runtime-contract-response-1")?,
        },
    );
    let mut shared = NativeAgentRuntime.start(shared_request)?;
    shared.accept_event(shared_start)?;
    assert_eq!(shared.status(), RuntimeRunStatus::Streaming);
    assert!(matches!(
        shared.accept_frame(&text_frame(1, "mixed lane")),
        Err(NativeAgentRunError::MixedInputSurface)
    ));
    assert_eq!(shared.status(), RuntimeRunStatus::Streaming);

    let concrete_request = request()?;
    let shared_after_concrete = RuntimeEventEnvelope::for_request(
        &concrete_request,
        1,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new("mixed lane")?,
        },
    );
    let mut concrete = NativeAgentRuntime.start(concrete_request)?;
    concrete.accept_frame(&start_frame())?;
    assert_eq!(concrete.status(), RuntimeRunStatus::Streaming);
    assert_eq!(
        concrete.accept_event(shared_after_concrete),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::MixedInputSurface
        ))
    );
    assert_eq!(concrete.status(), RuntimeRunStatus::Streaming);
    Ok(())
}

#[test]
fn native_run_preserves_text_lifecycle_and_typed_errors() -> Result<(), Box<dyn Error>> {
    let mut run = NativeAgentRuntime.start(request()?)?;

    assert!(matches!(
        run.accept_frame(&start_frame())?,
        Some(InitialGatewayEvent::ResponseStarted { .. })
    ));
    assert_eq!(run.status(), RuntimeRunStatus::Streaming);
    assert!(matches!(
        run.accept_frame(&text_frame(1, "A bounded response"))?,
        Some(InitialGatewayEvent::OutputTextDelta { .. })
    ));
    assert!(matches!(
        run.accept_frame(&completed_frame(2))?,
        Some(InitialGatewayEvent::ResponseCompleted)
    ));
    assert_eq!(run.status(), RuntimeRunStatus::Completed);

    assert!(matches!(
        run.accept_frame(&completed_frame(3)),
        Err(NativeAgentRunError::Turn(
            InitialGatewayTurnError::Protocol(GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Completed
            })
        ))
    ));
    assert_eq!(
        run.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Completed)
    );
    Ok(())
}

#[test]
fn native_run_preserves_policy_and_approval_ownership() -> Result<(), Box<dyn Error>> {
    let mut allowed = NativeAgentRuntime.start(request()?)?;
    allowed.accept_frame(&start_frame())?;
    assert!(allowed
        .accept_frame(&function_frame(1, "get_current_datetime", r#"{}"#))?
        .is_none());
    match allowed.accept_frame(&completed_frame(2))? {
        Some(InitialGatewayEvent::PolicyEvaluated { decision }) => {
            assert_eq!(decision.outcome(), PolicyOutcome::Allow);
        }
        _ => return Err("expected unchanged native policy result".into()),
    }

    let mut approval = NativeAgentRuntime.start(request()?)?;
    approval.accept_frame(&start_frame())?;
    assert!(approval
        .accept_frame(&function_frame(
            1,
            "create_local_task",
            r#"{"title":"Plan tomorrow"}"#,
        ))?
        .is_none());
    assert!(matches!(
        approval.accept_frame(&completed_frame(2))?,
        Some(InitialGatewayEvent::ApprovalPresentationReady { .. })
    ));
    let resolution = approval
        .cancel_pending_approval_for_run_termination()?
        .ok_or("expected audited pending-approval cancellation")?;
    assert_eq!(resolution.receipt().sequence().value(), 1);
    assert!(approval
        .cancel_pending_approval_for_run_termination()?
        .is_none());
    Ok(())
}

#[test]
fn native_shared_proposal_capability_fails_closed_without_governance_leak(
) -> Result<(), Box<dyn Error>> {
    let request = request()?;
    let started = RuntimeEventEnvelope::for_request(
        &request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("runtime-contract-response-1")?,
        },
    );
    let proposal = RuntimeEventEnvelope::for_request(
        &request,
        1,
        UntrustedRuntimeEvent::ToolProposal {
            proposal: UntrustedRuntimeToolProposal::new(
                "runtime-contract-call-1",
                "create_local_task",
                INITIAL_GATEWAY_TOOL_SET_VERSION,
                r#"{"title":"Plan tomorrow"}"#,
            )?,
        },
    );
    let late =
        RuntimeEventEnvelope::for_request(&request, 1, UntrustedRuntimeEvent::ResponseCompleted);
    let mut run = NativeAgentRuntime.start(request)?;

    run.accept_event(started)?;
    assert_eq!(
        run.accept_event(proposal),
        Err(RuntimeError::CapabilityUnavailable(
            RuntimeCapability::UntrustedToolProposals
        ))
    );
    assert_eq!(run.status(), RuntimeRunStatus::Failed);
    assert_eq!(
        run.accept_event(late),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidState
        ))
    );
    assert_eq!(
        run.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Failed)
    );
    Ok(())
}

#[test]
fn generic_cancellation_terminates_a_run_owned_pending_approval() -> Result<(), Box<dyn Error>> {
    let mut run = NativeAgentRuntime.start(request()?)?;
    run.accept_frame(&start_frame())?;
    run.accept_frame(&function_frame(
        1,
        "create_local_task",
        r#"{"title":"Plan tomorrow"}"#,
    ))?;
    assert!(matches!(
        run.accept_frame(&completed_frame(2))?,
        Some(InitialGatewayEvent::ApprovalPresentationReady { .. })
    ));
    assert_eq!(run.status(), RuntimeRunStatus::Completed);

    assert_eq!(run.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert_eq!(run.status(), RuntimeRunStatus::Cancelled);
    assert!(run.cancel_pending_approval_for_run_termination()?.is_none());
    assert_eq!(
        run.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Cancelled)
    );
    Ok(())
}

#[test]
fn native_cancellation_is_local_idempotent_and_terminal() -> Result<(), Box<dyn Error>> {
    let mut run = NativeAgentRuntime.start(request()?)?;

    assert_eq!(run.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert_eq!(run.status(), RuntimeRunStatus::Cancelled);
    assert_eq!(
        run.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Cancelled)
    );
    assert!(matches!(
        run.accept_frame(&start_frame()),
        Err(NativeAgentRunError::Turn(
            InitialGatewayTurnError::Protocol(GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Cancelled
            })
        ))
    ));
    Ok(())
}

#[test]
fn native_wrapper_preserves_failed_status_and_closed_failure() -> Result<(), Box<dyn Error>> {
    let mut run = NativeAgentRuntime.start(request()?)?;
    run.accept_frame(&start_frame())?;

    match run.accept_frame(&failed_frame(1))? {
        Some(InitialGatewayEvent::ResponseFailed { failure }) => {
            assert_eq!(failure.code(), GatewayFailureCode::ProviderTimeout);
            assert!(failure.retryable());
            assert_eq!(failure.retry_after_ms(), Some(1000));
        }
        _ => return Err("expected unchanged native failure".into()),
    }
    assert_eq!(run.status(), RuntimeRunStatus::Failed);
    assert!(matches!(
        run.accept_frame(&completed_frame(2)),
        Err(NativeAgentRunError::Turn(
            InitialGatewayTurnError::Protocol(GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Failed
            })
        ))
    ));
    assert_eq!(
        run.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Failed)
    );
    Ok(())
}

#[test]
fn native_wrapper_preserves_identity_order_and_tool_rejections() -> Result<(), Box<dyn Error>> {
    let mut identity = NativeAgentRuntime.start(request()?)?;
    let wrong_identity = frame_with_identity(
        "other-run",
        REQUEST_ID,
        0,
        json!({
            "type": "response_started",
            "provider_response_id": "runtime-contract-response-1",
        }),
    );
    assert!(matches!(
        identity.accept_frame(&wrong_identity),
        Err(NativeAgentRunError::Turn(
            InitialGatewayTurnError::Protocol(GatewayProtocolError::RunIdMismatch)
        ))
    ));
    assert_eq!(identity.status(), RuntimeRunStatus::AwaitingStart);
    assert!(matches!(
        identity.accept_frame(&text_frame(0, "too early")),
        Err(NativeAgentRunError::Turn(
            InitialGatewayTurnError::Protocol(GatewayProtocolError::EventBeforeResponseStarted)
        ))
    ));
    assert_eq!(identity.status(), RuntimeRunStatus::AwaitingStart);

    let mut unknown_tool = NativeAgentRuntime.start(request()?)?;
    unknown_tool.accept_frame(&start_frame())?;
    assert!(matches!(
        unknown_tool.accept_frame(&function_frame(1, "unknown_tool", r#"{}"#)),
        Err(NativeAgentRunError::Turn(
            InitialGatewayTurnError::Protocol(GatewayProtocolError::UnknownFunctionName)
        ))
    ));
    assert_eq!(unknown_tool.status(), RuntimeRunStatus::Streaming);

    let wrong_version = frame(
        1,
        json!({
            "type": "function_call_completed",
            "call_id": "runtime-contract-call-1",
            "name": "get_current_datetime",
            "tool_contract_version": INITIAL_GATEWAY_TOOL_SET_VERSION + 1,
            "arguments_json": "{}",
        }),
    );
    assert!(matches!(
        unknown_tool.accept_frame(&wrong_version),
        Err(NativeAgentRunError::Turn(
            InitialGatewayTurnError::Protocol(
                GatewayProtocolError::ToolContractVersionMismatch { .. }
            )
        ))
    ));
    assert_eq!(unknown_tool.status(), RuntimeRunStatus::Streaming);
    Ok(())
}

#[test]
fn native_start_maps_serialized_request_limit_without_content_leak() -> Result<(), Box<dyn Error>> {
    let selected_text = "s".repeat(MAX_GATEWAY_REQUEST_BYTES);
    let request = RuntimeTurnRequest::new(RUN_ID, REQUEST_ID, selected_text)?;
    let error = NativeAgentRuntime
        .start(request)
        .err()
        .ok_or("expected error")?;
    let debug = format!("{error:?}");
    let display = error.to_string();

    assert!(matches!(
        error,
        RuntimeError::InvalidRequest(RuntimeInvalidRequest::SerializedRequestTooLarge { .. })
    ));
    assert!(!debug.contains(&"s".repeat(64)));
    assert!(!display.contains(&"s".repeat(64)));
    Ok(())
}

fn collect_mock_script(
    runtime: &MockAgentRuntime,
) -> Result<Vec<RuntimeEventAcceptance>, Box<dyn Error>> {
    let request = request()?;
    let events = [
        RuntimeEventEnvelope::for_request(
            &request,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("mock-response-1")?,
            },
        ),
        RuntimeEventEnvelope::for_request(
            &request,
            1,
            UntrustedRuntimeEvent::OutputTextDelta {
                delta: RuntimeOutputText::new("deterministic response")?,
            },
        ),
        RuntimeEventEnvelope::for_request(&request, 2, UntrustedRuntimeEvent::ResponseCompleted),
    ];
    let mut run = runtime.start(request)?;
    let mut accepted = Vec::new();
    for event in events {
        accepted.push(run.accept_event(event)?);
    }
    Ok(accepted)
}

#[test]
fn mock_runtime_returns_the_same_closed_script_deterministically() -> Result<(), Box<dyn Error>> {
    let runtime = MockAgentRuntime::new(MockMode::Success);
    let first = collect_mock_script(&runtime)?;
    let second = collect_mock_script(&runtime)?;

    assert_eq!(first, second);
    assert_eq!(
        first,
        vec![
            RuntimeEventAcceptance::ResponseStarted {
                response_id: RuntimeResponseId::new("mock-response-1")?,
            },
            RuntimeEventAcceptance::OutputTextDelta {
                delta: RuntimeOutputText::new("deterministic response")?,
            },
            RuntimeEventAcceptance::ResponseCompleted,
        ]
    );
    Ok(())
}

#[test]
fn mock_runtime_supports_controlled_availability_start_and_event_failures(
) -> Result<(), Box<dyn Error>> {
    let unavailable = MockAgentRuntime::new(MockMode::Unavailable);
    assert_eq!(
        unavailable.describe().availability(),
        RuntimeAvailability::Unavailable
    );
    assert!(matches!(
        unavailable.start(request()?),
        Err(RuntimeError::Unavailable)
    ));

    let start_failure = MockAgentRuntime::new(MockMode::StartFailure);
    assert!(matches!(
        start_failure.start(request()?),
        Err(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))
    ));

    let event_request = request()?;
    let started = RuntimeEventEnvelope::for_request(
        &event_request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("mock-response-1")?,
        },
    );
    let text = RuntimeEventEnvelope::for_request(
        &event_request,
        1,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new("controlled failure")?,
        },
    );
    let late = RuntimeEventEnvelope::for_request(
        &event_request,
        1,
        UntrustedRuntimeEvent::ResponseCompleted,
    );
    let mut event_failure = MockAgentRuntime::new(MockMode::EventFailure).start(event_request)?;
    assert!(matches!(
        event_failure.accept_event(started)?,
        RuntimeEventAcceptance::ResponseStarted { .. }
    ));
    assert_eq!(
        event_failure.accept_event(text),
        Err(RuntimeError::BoundaryFailure(
            RuntimeBoundaryStage::EventAcceptance
        ))
    );
    assert_eq!(event_failure.status(), RuntimeRunStatus::Failed);
    assert_eq!(
        event_failure.accept_event(late),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidState
        ))
    );
    assert_eq!(
        event_failure.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Failed)
    );
    Ok(())
}

#[test]
fn mock_runtime_rejects_invalid_capability_and_terminal_transitions() -> Result<(), Box<dyn Error>>
{
    let invalid_request = request()?;
    let invalid_text = RuntimeEventEnvelope::for_request(
        &invalid_request,
        0,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new("out of order")?,
        },
    );
    let valid_start = RuntimeEventEnvelope::for_request(
        &invalid_request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("mock-response-1")?,
        },
    );
    let mut invalid = MockAgentRuntime::new(MockMode::Success).start(invalid_request)?;
    assert_eq!(
        invalid.accept_event(invalid_text),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidState
        ))
    );
    assert_eq!(invalid.status(), RuntimeRunStatus::AwaitingStart);
    assert!(matches!(
        invalid.accept_event(valid_start)?,
        RuntimeEventAcceptance::ResponseStarted { .. }
    ));

    let contradiction_request = request()?;
    let contradiction_start = RuntimeEventEnvelope::for_request(
        &contradiction_request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("mock-response-1")?,
        },
    );
    let contradiction_text = RuntimeEventEnvelope::for_request(
        &contradiction_request,
        1,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new("not declared")?,
        },
    );
    let contradiction_late = RuntimeEventEnvelope::for_request(
        &contradiction_request,
        1,
        UntrustedRuntimeEvent::ResponseCompleted,
    );
    let mut contradiction =
        MockAgentRuntime::new(MockMode::CapabilityContradiction).start(contradiction_request)?;
    contradiction.accept_event(contradiction_start)?;
    assert_eq!(
        contradiction.accept_event(contradiction_text),
        Err(RuntimeError::CapabilityUnavailable(
            RuntimeCapability::StreamingText
        ))
    );
    assert_eq!(contradiction.status(), RuntimeRunStatus::Failed);
    assert_eq!(
        contradiction.accept_event(contradiction_late),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidState
        ))
    );
    assert_eq!(
        contradiction.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Failed)
    );

    let cancelled_request = request()?;
    let cancelled_late = RuntimeEventEnvelope::for_request(
        &cancelled_request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("mock-response-1")?,
        },
    );
    let mut cancelled = MockAgentRuntime::new(MockMode::Success).start(cancelled_request)?;
    assert_eq!(cancelled.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert_eq!(
        cancelled.cancel()?,
        RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Cancelled)
    );
    assert_eq!(
        cancelled.accept_event(cancelled_late),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidState
        ))
    );

    let terminal_runtime = MockAgentRuntime::new(MockMode::Success);
    let terminal_request = request()?;
    let duplicate = RuntimeEventEnvelope::for_request(
        &terminal_request,
        3,
        UntrustedRuntimeEvent::ResponseCompleted,
    );
    let mut terminal_run = terminal_runtime.start(terminal_request)?;
    let scripted_request = request()?;
    let scripted = [
        RuntimeEventEnvelope::for_request(
            &scripted_request,
            0,
            UntrustedRuntimeEvent::ResponseStarted {
                response_id: RuntimeResponseId::new("mock-response-1")?,
            },
        ),
        RuntimeEventEnvelope::for_request(
            &scripted_request,
            1,
            UntrustedRuntimeEvent::OutputTextDelta {
                delta: RuntimeOutputText::new("done")?,
            },
        ),
        RuntimeEventEnvelope::for_request(
            &scripted_request,
            2,
            UntrustedRuntimeEvent::ResponseCompleted,
        ),
    ];
    for event in scripted {
        terminal_run.accept_event(event)?;
    }
    assert_eq!(
        terminal_run.accept_event(duplicate),
        Err(RuntimeError::EventRejected(
            RuntimeEventRejection::InvalidState
        ))
    );
    Ok(())
}

#[test]
fn mock_runtime_tracks_two_live_runs_and_local_terminal_dispositions() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let first_request = RuntimeTurnRequest::new(
        "mock-live-run-private-1",
        "mock-live-request-private-1",
        "first fixture",
    )?;
    let second_request = RuntimeTurnRequest::new(
        "mock-live-run-private-2",
        "mock-live-request-private-2",
        "second fixture",
    )?;
    let second_started = RuntimeEventEnvelope::for_request(
        &second_request,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("mock-live-response-2")?,
        },
    );
    let second_completed = RuntimeEventEnvelope::for_request(
        &second_request,
        1,
        UntrustedRuntimeEvent::ResponseCompleted,
    );

    let mut first = runtime.start(first_request)?;
    let mut second = runtime.start(second_request)?;

    assert_eq!(recorder.live_runs().len(), 2);
    assert_eq!(recorder.maximum_live_run_count(), 2);
    assert_eq!(
        recorder
            .live_runs()
            .iter()
            .map(|record| (record.start_ordinal, record.status))
            .collect::<Vec<_>>(),
        vec![
            (1, RuntimeRunStatus::AwaitingStart),
            (2, RuntimeRunStatus::AwaitingStart),
        ]
    );

    assert_eq!(first.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert_eq!(
        recorder
            .live_runs()
            .iter()
            .map(|record| record.start_ordinal)
            .collect::<Vec<_>>(),
        vec![2]
    );
    assert_eq!(second.status(), RuntimeRunStatus::AwaitingStart);

    second.accept_event(second_started)?;
    assert_eq!(recorder.live_runs()[0].status, RuntimeRunStatus::Streaming);
    second.accept_event(second_completed)?;

    assert!(recorder.live_runs().is_empty());
    assert_eq!(
        recorder
            .terminal_dispositions()
            .iter()
            .map(|record| (record.start_ordinal, record.status))
            .collect::<Vec<_>>(),
        vec![
            (1, RuntimeRunStatus::Cancelled),
            (2, RuntimeRunStatus::Completed),
        ]
    );
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn mock_runtime_one_shot_cancel_failure_retains_then_releases_the_live_run(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(1));
    let mut run = runtime.start(RuntimeTurnRequest::new(
        "mock-cancel-once-private-run",
        "mock-cancel-once-private-request",
        "cancel fixture",
    )?)?;

    assert_eq!(
        run.cancel(),
        Err(RuntimeError::BoundaryFailure(
            RuntimeBoundaryStage::Cancellation
        ))
    );
    assert_eq!(run.status(), RuntimeRunStatus::AwaitingStart);
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.terminal_dispositions().is_empty());
    assert!(recorder.cancellations().is_empty());

    assert_eq!(run.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert!(recorder.live_runs().is_empty());
    assert_eq!(recorder.cancellations().len(), 1);
    assert_eq!(
        recorder
            .terminal_dispositions()
            .iter()
            .map(|record| (record.start_ordinal, record.status))
            .collect::<Vec<_>>(),
        vec![(1, RuntimeRunStatus::Cancelled)]
    );
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn mock_runtime_records_nonterminal_drop_once_and_redacts_run_identity(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let run_id = "mock-dropped-private-run-sentinel";
    let dropped = runtime.start(RuntimeTurnRequest::new(
        run_id,
        "mock-dropped-private-request-sentinel",
        "drop fixture",
    )?)?;

    assert_eq!(recorder.live_runs().len(), 1);
    drop(dropped);

    assert!(recorder.live_runs().is_empty());
    assert!(recorder.terminal_dispositions().is_empty());
    let drops = recorder.nonterminal_drops();
    assert_eq!(drops.len(), 1);
    assert_eq!(drops[0].start_ordinal, 1);
    assert_eq!(drops[0].status, RuntimeRunStatus::AwaitingStart);
    let debug = format!("{drops:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains(run_id));
    drop(drops);
    assert_eq!(recorder.nonterminal_drops().len(), 1);

    let (failed_runtime, failed_recorder) = MockAgentRuntime::recording(MockMode::StartFailure);
    assert!(matches!(
        failed_runtime.start(RuntimeTurnRequest::new(
            "mock-failed-private-run",
            "mock-failed-private-request",
            "failed start fixture",
        )?),
        Err(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))
    ));
    assert!(failed_recorder.live_runs().is_empty());
    assert_eq!(failed_recorder.maximum_live_run_count(), 0);
    assert!(failed_recorder.terminal_dispositions().is_empty());
    assert!(failed_recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn mock_runtime_can_return_a_foreign_identity_with_redacted_lifecycle_accounting(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchAt(1));
    let request = RuntimeTurnRequest::new(
        "mock-requested-private-run",
        "mock-requested-private-request",
        "identity mismatch fixture",
    )?;
    let requested_identity = request.identity();
    let mut run = runtime.start(request)?;

    assert_ne!(run.identity(), &requested_identity);
    assert_ne!(run.run_id(), requested_identity.run_id());
    assert_ne!(run.identity().request_id(), requested_identity.request_id());
    assert_eq!(recorder.live_runs().len(), 1);
    assert_eq!(recorder.maximum_live_run_count(), 1);
    let live_debug = format!("{:?}", recorder.live_runs());
    for private_identity in [
        "mock-requested-private-run",
        "mock-requested-private-request",
        "mock-foreign-run-1",
        "mock-foreign-request-1",
    ] {
        assert!(!live_debug.contains(private_identity));
    }
    assert!(live_debug.contains("[REDACTED]"));

    assert_eq!(run.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert!(recorder.live_runs().is_empty());
    assert_eq!(
        recorder.terminal_dispositions()[0].status,
        RuntimeRunStatus::Cancelled
    );
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn mock_runtime_retains_a_foreign_run_until_one_shot_cancel_failure_is_retried(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) =
        MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchWithCancelFailureOnceAt(1));
    let request = RuntimeTurnRequest::new(
        "mock-retained-requested-private-run",
        "mock-retained-requested-private-request",
        "retained identity mismatch fixture",
    )?;
    let requested_identity = request.identity();
    let mut rejected = runtime.start(request)?;

    assert_ne!(rejected.identity(), &requested_identity);
    assert_eq!(
        rejected.cancel(),
        Err(RuntimeError::BoundaryFailure(
            RuntimeBoundaryStage::Cancellation
        ))
    );
    assert_eq!(rejected.status(), RuntimeRunStatus::AwaitingStart);
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.terminal_dispositions().is_empty());
    assert!(recorder.cancellations().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());

    assert_eq!(rejected.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert!(recorder.live_runs().is_empty());
    assert_eq!(recorder.cancellations().len(), 1);
    assert_eq!(recorder.terminal_dispositions().len(), 1);
    assert_eq!(
        recorder.terminal_dispositions()[0].status,
        RuntimeRunStatus::Cancelled
    );
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn mock_runtime_can_return_one_live_identity_for_two_distinct_starts() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::DuplicateLiveIdentityAt(2));
    let first_request = RuntimeTurnRequest::new(
        "mock-duplicate-private-run-1",
        "mock-duplicate-private-request-1",
        "first duplicate fixture",
    )?;
    let first_requested_identity = first_request.identity();
    let second_request = RuntimeTurnRequest::new(
        "mock-duplicate-private-run-2",
        "mock-duplicate-private-request-2",
        "second duplicate fixture",
    )?;
    let second_requested_identity = second_request.identity();

    let mut first = runtime.start(first_request)?;
    let mut second = runtime.start(second_request)?;

    assert_eq!(first.identity(), &first_requested_identity);
    assert_eq!(second.identity(), first.identity());
    assert_ne!(second.identity(), &second_requested_identity);
    assert_eq!(recorder.live_runs().len(), 2);
    assert_eq!(recorder.maximum_live_run_count(), 2);
    assert_eq!(
        recorder.live_runs()[0].run_id,
        recorder.live_runs()[1].run_id
    );
    assert_ne!(recorder.starts()[0].run_id, recorder.starts()[1].run_id);

    let live_debug = format!("{:?}", recorder.live_runs());
    for private_identity in [
        "mock-duplicate-private-run-1",
        "mock-duplicate-private-request-1",
        "mock-duplicate-private-run-2",
        "mock-duplicate-private-request-2",
    ] {
        assert!(!live_debug.contains(private_identity));
    }
    assert!(live_debug.contains("[REDACTED]"));

    assert_eq!(first.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert_eq!(recorder.live_runs().len(), 1);
    assert_eq!(second.status(), RuntimeRunStatus::AwaitingStart);
    assert_eq!(second.cancel()?, RuntimeCancellationOutcome::Cancelled);
    assert!(recorder.live_runs().is_empty());
    assert_eq!(recorder.terminal_dispositions().len(), 2);
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}
