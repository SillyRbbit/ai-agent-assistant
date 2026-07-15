use std::error::Error;

use ai_agent_assistant_lib::agent::gateway_protocol::{
    GatewayProtocolError, GatewayStreamStatus, ValidatedGatewayEvent, GATEWAY_PROTOCOL_VERSION,
    MAX_GATEWAY_REQUEST_BYTES,
};
use ai_agent_assistant_lib::agent::gateway_request::{
    InitialGatewayTurn, INITIAL_GATEWAY_TOOL_SET_ID, INITIAL_GATEWAY_TOOL_SET_VERSION,
};
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

fn started_turn() -> Result<InitialGatewayTurn, Box<dyn Error>> {
    let mut turn = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, "Plan my day")?;
    turn.accept_frame(&start_frame(RUN_ID, GATEWAY_REQUEST_ID))?;
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
        Err(GatewayProtocolError::RunIdMismatch)
    ));
    assert!(matches!(
        turn.accept_frame(&start_frame(RUN_ID, "gateway-request-other")),
        Err(GatewayProtocolError::GatewayRequestIdMismatch)
    ));
    assert_eq!(turn.status(), GatewayStreamStatus::AwaitingStart);
    assert!(matches!(
        turn.accept_frame(&start_frame(RUN_ID, GATEWAY_REQUEST_ID))?,
        ValidatedGatewayEvent::ResponseStarted { .. }
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
        let event = turn.accept_frame(&function_call_frame(
            name,
            INITIAL_GATEWAY_TOOL_SET_VERSION,
            arguments_json,
        ))?;
        match event {
            ValidatedGatewayEvent::FunctionCallCompleted { call } => {
                assert_eq!(call.name(), name);
                assert_eq!(
                    call.tool_contract_version(),
                    INITIAL_GATEWAY_TOOL_SET_VERSION
                );
            }
            _ => return Err("expected a validated function call".into()),
        }
    }
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
        Err(GatewayProtocolError::UnknownFunctionName)
    ));

    let mut wrong_version_turn = started_turn()?;
    assert!(matches!(
        wrong_version_turn.accept_frame(&function_call_frame(
            "get_current_datetime",
            INITIAL_GATEWAY_TOOL_SET_VERSION + 1,
            "{}",
        )),
        Err(GatewayProtocolError::ToolContractVersionMismatch {
            expected: INITIAL_GATEWAY_TOOL_SET_VERSION,
            actual,
        }) if actual == INITIAL_GATEWAY_TOOL_SET_VERSION + 1
    ));
    Ok(())
}

#[test]
fn cancellation_is_local_idempotent_and_terminal() -> Result<(), Box<dyn Error>> {
    let mut turn = InitialGatewayTurn::new(RUN_ID, GATEWAY_REQUEST_ID, "Plan my day")?;

    assert!(turn.cancel());
    assert!(!turn.cancel());
    assert_eq!(turn.status(), GatewayStreamStatus::Cancelled);
    assert!(matches!(
        turn.accept_frame(&start_frame(RUN_ID, GATEWAY_REQUEST_ID)),
        Err(GatewayProtocolError::StreamAlreadyTerminal {
            status: GatewayStreamStatus::Cancelled,
        })
    ));
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
    Ok(())
}
