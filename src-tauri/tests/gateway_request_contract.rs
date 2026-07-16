use std::error::Error;

use ai_agent_assistant_lib::agent::function_call_validation::FunctionCallValidationError;
use ai_agent_assistant_lib::agent::gateway_protocol::{
    GatewayFailureCode, GatewayProtocolError, GatewayStreamStatus, GATEWAY_PROTOCOL_VERSION,
    MAX_GATEWAY_REQUEST_BYTES,
};
use ai_agent_assistant_lib::agent::gateway_request::{
    InitialGatewayEvent, InitialGatewayTurn, InitialGatewayTurnError, INITIAL_GATEWAY_TOOL_SET_ID,
    INITIAL_GATEWAY_TOOL_SET_VERSION,
};
use ai_agent_assistant_lib::approvals::types::{
    ApprovalAction, ApprovalCancellationReason, ApprovalDisposition, ApprovalRecipients,
    ApprovalReversibility, ApprovalRisk, ApprovalSchedule, ApprovalTarget,
};
use ai_agent_assistant_lib::policy::types::{PolicyOutcome, PolicyReason};
use ai_agent_assistant_lib::tools::schema::{ToolArgumentValidationError, ValidatedToolArguments};
use ai_agent_assistant_lib::tools::types::{PermissionKind, RiskClass};
use serde_json::{json, Value};

const RUN_ID: &str = "run-public-request-1";
const GATEWAY_REQUEST_ID: &str = "gateway-request-public-1";

fn frame(run_id: &str, gateway_request_id: &str, sequence: u32, event: Value) -> Vec<u8> {
    json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": run_id,
        "gateway_request_id": gateway_request_id,
        "sequence": sequence,
        "event": event,
    })
    .to_string()
    .into_bytes()
}

fn start_frame(run_id: &str, gateway_request_id: &str) -> Vec<u8> {
    frame(
        run_id,
        gateway_request_id,
        0,
        json!({
            "type": "response_started",
            "provider_response_id": "provider-response-public-1",
        }),
    )
}

fn function_call_frame(name: &str, version: u16, arguments_json: &str) -> Vec<u8> {
    frame(
        RUN_ID,
        GATEWAY_REQUEST_ID,
        1,
        json!({
            "type": "function_call_completed",
            "call_id": "call-public-1",
            "name": name,
            "tool_contract_version": version,
            "arguments_json": arguments_json,
        }),
    )
}

fn completion_frame(sequence: u32) -> Vec<u8> {
    frame(
        RUN_ID,
        GATEWAY_REQUEST_ID,
        sequence,
        json!({ "type": "response_completed" }),
    )
}

fn started_turn() -> Result<InitialGatewayTurn, Box<dyn Error>> {
    let mut turn = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, "Plan my day")?;
    if !matches!(
        turn.accept_frame(&start_frame(RUN_ID, GATEWAY_REQUEST_ID))?,
        Some(InitialGatewayEvent::ResponseStarted { .. })
    ) {
        return Err("expected response start".into());
    }
    Ok(turn)
}

fn turn_with_pending_approval() -> Result<InitialGatewayTurn, Box<dyn Error>> {
    let mut turn = started_turn()?;
    if turn
        .accept_frame(&function_call_frame(
            "create_local_task",
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            r#"{"title":"Plan tomorrow"}"#,
        ))?
        .is_some()
    {
        return Err("validated function call should remain pending".into());
    }

    if !matches!(
        turn.accept_frame(&completion_frame(2))?,
        Some(InitialGatewayEvent::ApprovalPresentationReady { .. })
    ) {
        return Err("expected terminal approval presentation".into());
    }
    Ok(turn)
}

#[test]
fn exposes_only_the_bound_initial_gateway_turn() -> Result<(), Box<dyn Error>> {
    let sentinel = "private-public-request-sentinel";
    let turn = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, sentinel)?;
    let debug = format!("{turn:?}");
    let body: Value = serde_json::from_slice(turn.request_bytes())?;

    assert!(!debug.contains(sentinel));
    assert!(debug.contains("[REDACTED]"));
    assert!(turn.request_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);
    assert_eq!(turn.status(), GatewayStreamStatus::AwaitingStart);
    assert_eq!(body["protocol_version"], GATEWAY_PROTOCOL_VERSION);
    assert_eq!(body["run_id"], RUN_ID);
    assert_eq!(body["gateway_request_id"], GATEWAY_REQUEST_ID);
    assert_eq!(body["request_kind"], "initial_user_turn");
    assert_eq!(body["model_turn"], 1);
    assert_eq!(body["retry_attempt"], 0);
    assert_eq!(body["input"]["type"], "user_selected_text");
    assert_eq!(body["input"]["text"], sentinel);
    assert_eq!(body["tool_set"]["id"], INITIAL_GATEWAY_TOOL_SET_ID);
    assert_eq!(
        body["tool_set"]["version"],
        INITIAL_GATEWAY_TOOL_SET_VERSION
    );
    assert!(body.get("provider").is_none());
    assert!(body.get("model").is_none());
    assert!(body.get("authorization").is_none());
    assert!(body.get("tools").is_none());
    assert!(body.get("execute").is_none());
    Ok(())
}

#[test]
fn binds_response_identity_to_the_request() -> Result<(), Box<dyn Error>> {
    let mut turn = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, "Plan my day")?;

    assert!(matches!(
        turn.accept_frame(&start_frame("run-other", GATEWAY_REQUEST_ID)),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::RunIdMismatch
        ))
    ));
    assert!(matches!(
        turn.accept_frame(&start_frame(RUN_ID, "gateway-request-other")),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::GatewayRequestIdMismatch
        ))
    ));
    assert_eq!(turn.status(), GatewayStreamStatus::AwaitingStart);
    assert!(matches!(
        turn.accept_frame(&start_frame(RUN_ID, GATEWAY_REQUEST_ID))?,
        Some(InitialGatewayEvent::ResponseStarted { .. })
    ));
    assert_eq!(turn.status(), GatewayStreamStatus::Streaming);
    Ok(())
}

#[test]
fn accepts_each_exact_local_tool_contract() -> Result<(), Box<dyn Error>> {
    for (name, arguments_json) in [
        ("get_current_datetime", "{}"),
        ("create_local_task", r#"{"title":"Plan tomorrow"}"#),
    ] {
        let mut turn = started_turn()?;
        assert!(turn
            .accept_frame(&function_call_frame(
                name,
                INITIAL_GATEWAY_TOOL_SET_VERSION,
                arguments_json,
            ))?
            .is_none());
        assert_eq!(turn.status(), GatewayStreamStatus::Streaming);
        let pending_debug = format!("{turn:?}");
        assert!(!pending_debug.contains(arguments_json));
        assert!(!pending_debug.contains("Plan tomorrow"));

        let event = turn
            .accept_frame(&completion_frame(2))?
            .ok_or("terminal completion must return a closed outcome")?;
        let debug = format!("{event:?}");
        match (name, event) {
            ("get_current_datetime", InitialGatewayEvent::PolicyEvaluated { decision }) => {
                assert_eq!(decision.outcome(), PolicyOutcome::Allow);
                assert_eq!(decision.reason(), PolicyReason::InformationOnly);
                let call = decision.validated_call();
                assert_eq!(call.run_id(), RUN_ID);
                assert_eq!(call.gateway_request_id(), GATEWAY_REQUEST_ID);
                assert_eq!(call.call_id(), "call-public-1");
                assert_eq!(call.tool_name(), name);
                assert_eq!(
                    call.tool_contract_version(),
                    INITIAL_GATEWAY_TOOL_SET_VERSION
                );
                assert_eq!(call.required_permission(), PermissionKind::None);
                assert_eq!(call.risk_class(), RiskClass::InformationOnly);
                assert!(matches!(
                    call.arguments(),
                    ValidatedToolArguments::GetCurrentDatetime
                ));
            }
            (
                "create_local_task",
                InitialGatewayEvent::ApprovalPresentationReady { presentation },
            ) => {
                assert_eq!(presentation.id().value(), 1);
                assert_eq!(presentation.run_id(), RUN_ID);
                assert_eq!(presentation.gateway_request_id(), GATEWAY_REQUEST_ID);
                assert_eq!(presentation.call_id(), "call-public-1");
                assert_eq!(presentation.tool_name(), name);
                assert_eq!(
                    presentation.tool_contract_version(),
                    INITIAL_GATEWAY_TOOL_SET_VERSION
                );
                assert_eq!(
                    presentation.policy_outcome(),
                    PolicyOutcome::RequireApproval
                );
                assert_eq!(
                    presentation.policy_reason(),
                    PolicyReason::ReversibleRequiresApproval
                );
                assert_eq!(presentation.risk_class(), RiskClass::ReversibleLocalAction);
                assert_eq!(presentation.required_permission(), PermissionKind::None);

                let preview = presentation.preview();
                assert_eq!(preview.action(), ApprovalAction::CreateLocalTask);
                assert_eq!(preview.target(), ApprovalTarget::LocalTaskList);
                assert_eq!(preview.affected_data().value(), "Plan tomorrow");
                assert_eq!(preview.schedule(), ApprovalSchedule::NotScheduled);
                assert_eq!(preview.recipients(), ApprovalRecipients::None);
                assert_eq!(preview.reversibility(), ApprovalReversibility::Reversible);
                assert_eq!(preview.required_permission(), PermissionKind::None);
                assert_eq!(preview.risk_class(), RiskClass::ReversibleLocalAction);
                assert_eq!(preview.risk(), ApprovalRisk::CreatesLocalTask);

                assert!(!debug.contains("Plan tomorrow"));
                assert!(!debug.contains(RUN_ID));
                assert!(!debug.contains(GATEWAY_REQUEST_ID));
                assert!(!debug.contains("call-public-1"));
                assert!(debug.contains("[REDACTED]"));
            }
            _ => return Err("terminal event did not match the local tool contract".into()),
        }
        assert_eq!(turn.status(), GatewayStreamStatus::Completed);
        assert!(matches!(
            turn.accept_frame(&completion_frame(3)),
            Err(InitialGatewayTurnError::Protocol(
                GatewayProtocolError::StreamAlreadyTerminal {
                    status: GatewayStreamStatus::Completed,
                }
            ))
        ));
    }
    Ok(())
}

#[test]
fn protocol_errors_retain_the_pending_call_until_correct_completion() -> Result<(), Box<dyn Error>>
{
    let argument_sentinel = "private-pending-task-title";
    let arguments_json = json!({ "title": argument_sentinel }).to_string();
    let mut turn = started_turn()?;

    assert!(turn
        .accept_frame(&function_call_frame(
            "create_local_task",
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            &arguments_json,
        ))?
        .is_none());
    assert!(matches!(
        turn.accept_frame(b"{"),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::MalformedEvent
        ))
    ));
    assert!(matches!(
        turn.accept_frame(&frame(
            RUN_ID,
            "gateway-request-other",
            2,
            json!({ "type": "response_completed" }),
        )),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::GatewayRequestIdMismatch
        ))
    ));
    assert!(matches!(
        turn.accept_frame(&completion_frame(3)),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::InvalidSequence {
                expected: 2,
                actual: 3,
            }
        ))
    ));
    assert_eq!(turn.status(), GatewayStreamStatus::Streaming);
    assert!(!format!("{turn:?}").contains(argument_sentinel));

    let event = turn
        .accept_frame(&completion_frame(2))?
        .ok_or("correct completion must present the retained call")?;
    match event {
        InitialGatewayEvent::ApprovalPresentationReady { presentation } => {
            assert_eq!(
                presentation.policy_outcome(),
                PolicyOutcome::RequireApproval
            );
            assert_eq!(
                presentation.policy_reason(),
                PolicyReason::ReversibleRequiresApproval
            );
            assert_eq!(
                presentation.preview().affected_data().value(),
                argument_sentinel
            );
            assert!(!format!("{presentation:?}").contains(argument_sentinel));
        }
        _ => return Err("correct completion did not produce an approval presentation".into()),
    }
    assert_eq!(turn.status(), GatewayStreamStatus::Completed);
    Ok(())
}

#[test]
fn rejects_unknown_tools_and_contract_versions() -> Result<(), Box<dyn Error>> {
    let mut unknown_tool_turn = started_turn()?;
    assert!(matches!(
        unknown_tool_turn.accept_frame(&function_call_frame(
            "unknown_tool",
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            "{}",
        )),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::UnknownFunctionName
        ))
    ));

    let mut wrong_version_turn = started_turn()?;
    assert!(matches!(
        wrong_version_turn.accept_frame(&function_call_frame(
            "get_current_datetime",
            INITIAL_GATEWAY_TOOL_SET_VERSION + 1,
            "{}",
        )),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::ToolContractVersionMismatch {
                expected: INITIAL_GATEWAY_TOOL_SET_VERSION,
                actual,
            }
        )) if actual == INITIAL_GATEWAY_TOOL_SET_VERSION + 1
    ));
    Ok(())
}

#[test]
fn local_schema_rejection_is_typed_redacted_and_terminal() -> Result<(), Box<dyn Error>> {
    let invalid_cases = [
        (
            "get_current_datetime",
            r#"{"timezone":"UTC"}"#,
            ToolArgumentValidationError::InvalidShape,
        ),
        (
            "create_local_task",
            r#"{"title":" Plan tomorrow"}"#,
            ToolArgumentValidationError::NonCanonicalTitle,
        ),
    ];

    for (name, arguments_json, expected_reason) in invalid_cases {
        let mut turn = started_turn()?;
        let error = turn
            .accept_frame(&function_call_frame(
                name,
                INITIAL_GATEWAY_TOOL_SET_VERSION,
                arguments_json,
            ))
            .err()
            .ok_or("local schema rejection was expected")?;

        assert!(matches!(
            error,
            InitialGatewayTurnError::FunctionCallValidation(
                FunctionCallValidationError::InvalidArguments { reason }
            ) if reason == expected_reason
        ));
        assert!(!format!("{error:?} {error}").contains(arguments_json));
        assert_eq!(turn.status(), GatewayStreamStatus::Failed);
        assert!(!turn.cancel());
        assert!(matches!(
            turn.accept_frame(&frame(
                RUN_ID,
                GATEWAY_REQUEST_ID,
                2,
                json!({ "type": "response_completed" }),
            )),
            Err(InitialGatewayTurnError::Protocol(
                GatewayProtocolError::StreamAlreadyTerminal {
                    status: GatewayStreamStatus::Failed,
                }
            ))
        ));
    }
    Ok(())
}

#[test]
fn preserves_text_completion_and_closed_gateway_failure() -> Result<(), Box<dyn Error>> {
    let output_sentinel = "private-output-delta-sentinel";
    let mut text_turn = started_turn()?;
    let output = text_turn
        .accept_frame(&frame(
            RUN_ID,
            GATEWAY_REQUEST_ID,
            1,
            json!({ "type": "output_text_delta", "delta": output_sentinel }),
        ))?
        .ok_or("text delta must remain caller-visible")?;
    assert!(matches!(
        &output,
        InitialGatewayEvent::OutputTextDelta { delta } if delta == output_sentinel
    ));
    assert!(!format!("{output:?}").contains(output_sentinel));
    assert!(matches!(
        text_turn.accept_frame(&frame(
            RUN_ID,
            GATEWAY_REQUEST_ID,
            2,
            json!({ "type": "response_completed" }),
        ))?,
        Some(InitialGatewayEvent::ResponseCompleted)
    ));
    assert_eq!(text_turn.status(), GatewayStreamStatus::Completed);

    let mut failed_turn = started_turn()?;
    assert!(failed_turn
        .accept_frame(&function_call_frame(
            "create_local_task",
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            r#"{"title":"Discard this pending task"}"#,
        ))?
        .is_none());
    match failed_turn.accept_frame(&frame(
        RUN_ID,
        GATEWAY_REQUEST_ID,
        2,
        json!({
            "type": "response_failed",
            "code": "provider_unavailable",
            "retryable": true,
            "retry_after_ms": 250,
        }),
    ))? {
        Some(InitialGatewayEvent::ResponseFailed { failure }) => {
            assert_eq!(failure.code(), GatewayFailureCode::ProviderUnavailable);
            assert!(failure.retryable());
            assert_eq!(failure.retry_after_ms(), Some(250));
        }
        _ => return Err("expected a closed gateway failure".into()),
    }
    assert_eq!(failed_turn.status(), GatewayStreamStatus::Failed);
    assert!(matches!(
        failed_turn.accept_frame(&completion_frame(3)),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Failed,
            }
        ))
    ));
    Ok(())
}

#[test]
fn cancellation_is_local_idempotent_and_terminal() -> Result<(), Box<dyn Error>> {
    let argument_sentinel = "private-cancelled-task-title";
    let arguments_json = json!({ "title": argument_sentinel }).to_string();
    let mut turn = started_turn()?;

    assert!(turn
        .accept_frame(&function_call_frame(
            "create_local_task",
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            &arguments_json,
        ))?
        .is_none());
    assert!(!format!("{turn:?}").contains(argument_sentinel));
    assert!(turn.cancel());
    assert!(!turn.cancel());
    assert_eq!(turn.status(), GatewayStreamStatus::Cancelled);
    assert!(matches!(
        turn.accept_frame(&completion_frame(2)),
        Err(InitialGatewayTurnError::Protocol(
            GatewayProtocolError::StreamAlreadyTerminal {
                status: GatewayStreamStatus::Cancelled,
            }
        ))
    ));
    Ok(())
}

#[test]
fn run_termination_cancels_only_the_turn_owned_pending_approval() -> Result<(), Box<dyn Error>> {
    let mut no_pending_turn = started_turn()?;
    assert!(no_pending_turn
        .cancel_pending_approval_for_run_termination()?
        .is_none());

    let mut turn = turn_with_pending_approval()?;
    let resolution = turn
        .cancel_pending_approval_for_run_termination()?
        .ok_or("run termination must consume the pending approval")?;

    assert_eq!(resolution.id().value(), 1);
    assert_eq!(
        resolution.disposition(),
        ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
    );
    assert!(resolution.interaction_evidence().is_none());
    assert_eq!(resolution.run_id(), RUN_ID);
    assert_eq!(resolution.gateway_request_id(), GATEWAY_REQUEST_ID);
    assert_eq!(resolution.call_id(), "call-public-1");
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
        .ok_or("run termination must retain the registered preview")?;
    assert_eq!(preview.action(), ApprovalAction::CreateLocalTask);
    assert_eq!(preview.target(), ApprovalTarget::LocalTaskList);
    assert_eq!(preview.affected_data().value(), "Plan tomorrow");
    assert_eq!(preview.schedule(), ApprovalSchedule::NotScheduled);
    assert_eq!(preview.recipients(), ApprovalRecipients::None);
    assert_eq!(preview.reversibility(), ApprovalReversibility::Reversible);
    assert_eq!(preview.required_permission(), PermissionKind::None);
    assert_eq!(preview.risk_class(), RiskClass::ReversibleLocalAction);
    assert_eq!(preview.risk(), ApprovalRisk::CreatesLocalTask);

    assert!(turn
        .cancel_pending_approval_for_run_termination()?
        .is_none());
    Ok(())
}

#[test]
fn public_debug_and_errors_redact_selected_content() -> Result<(), Box<dyn Error>> {
    let sentinel = "private-bound-turn-sentinel";
    let turn = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, sentinel)?;
    let debug = format!("{turn:?}");
    assert!(!debug.contains(sentinel));
    assert!(debug.contains("[REDACTED]"));

    let oversized = format!("{sentinel}{}", "\"".repeat(MAX_GATEWAY_REQUEST_BYTES / 2));
    let error = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, oversized).err();
    let error_text = format!("{error:?}");
    assert!(error.is_some());
    assert!(!error_text.contains(sentinel));

    let argument_sentinel = "private-invalid-task-title";
    let mut schema_turn = started_turn()?;
    let invalid_arguments = json!({ "title": format!(" {argument_sentinel}") }).to_string();
    let error = schema_turn
        .accept_frame(&function_call_frame(
            "create_local_task",
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            &invalid_arguments,
        ))
        .err()
        .ok_or("local schema rejection was expected")?;
    let error_text = format!("{error:?} {error}");
    let turn_debug = format!("{schema_turn:?}");
    assert!(!error_text.contains(argument_sentinel));
    assert!(!error_text.contains(&invalid_arguments));
    assert!(!turn_debug.contains(argument_sentinel));
    assert!(turn_debug.contains("Failed"));
    Ok(())
}
