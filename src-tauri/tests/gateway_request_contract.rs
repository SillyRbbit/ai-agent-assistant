use std::error::Error;

use ai_agent_assistant_lib::agent::gateway_protocol::{
    GATEWAY_PROTOCOL_VERSION, MAX_GATEWAY_REQUEST_BYTES,
};
use ai_agent_assistant_lib::agent::gateway_request::{
    InitialGatewayRequest, INITIAL_GATEWAY_TOOL_SET_ID, INITIAL_GATEWAY_TOOL_SET_VERSION,
};
use serde_json::Value;

#[test]
fn exposes_only_the_bounded_initial_gateway_request() -> Result<(), Box<dyn Error>> {
    let sentinel = "private-public-request-sentinel";
    let request =
        InitialGatewayRequest::new("run-public-request-1", "gateway-request-public-1", sentinel)?;
    let debug = format!("{request:?}");
    let body: Value = serde_json::from_slice(request.as_bytes())?;

    assert!(!debug.contains(sentinel));
    assert!(request.as_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);
    assert_eq!(body["protocol_version"], GATEWAY_PROTOCOL_VERSION);
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
