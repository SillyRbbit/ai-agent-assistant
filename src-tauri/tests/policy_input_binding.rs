use std::error::Error;

use ai_agent_assistant_lib::agent::function_call_validation::{
    validate_function_call, SchemaValidatedFunctionCall,
};
use ai_agent_assistant_lib::agent::gateway_protocol::{
    GatewayProtocolError, GatewayStreamValidator, UntrustedFunctionCall, ValidatedGatewayEvent,
    GATEWAY_PROTOCOL_VERSION,
};
use ai_agent_assistant_lib::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
use ai_agent_assistant_lib::policy::types::{PolicyInput, PolicyOutcome, PolicyReason};
use ai_agent_assistant_lib::tools::registry::{
    InMemoryToolRegistry, ToolRegistry, ToolRegistryResult,
};
use ai_agent_assistant_lib::tools::schema::ValidatedToolArguments;
use ai_agent_assistant_lib::tools::types::{PermissionKind, RiskClass, ToolDefinition, ToolSchema};
use serde_json::json;

const RUN_ID: &str = "run-policy-binding-1";
const GATEWAY_REQUEST_ID: &str = "gateway-request-policy-binding-1";

fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
    let mut registry = InMemoryToolRegistry::new();
    registry.register(ToolDefinition::from_schema(
        ToolSchema::GetCurrentDatetimeV1,
    ))?;
    registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
    Ok(registry)
}

fn normalized_call(
    call_id: &str,
    name: &str,
    arguments_json: &str,
) -> Result<UntrustedFunctionCall, Box<dyn Error>> {
    let mut validator =
        GatewayStreamValidator::new(RUN_ID, GATEWAY_REQUEST_ID, [name.to_owned()], 1)?;
    let start_frame = json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": RUN_ID,
        "gateway_request_id": GATEWAY_REQUEST_ID,
        "sequence": 0,
        "event": {
            "type": "response_started",
            "provider_response_id": "provider-response-policy-binding-1",
        },
    });
    let function_frame = json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": RUN_ID,
        "gateway_request_id": GATEWAY_REQUEST_ID,
        "sequence": 1,
        "event": {
            "type": "function_call_completed",
            "call_id": call_id,
            "name": name,
            "tool_contract_version": 1,
            "arguments_json": arguments_json,
        },
    });

    let _ = validator.accept_frame(&serde_json::to_vec(&start_frame)?)?;
    match validator.accept_frame(&serde_json::to_vec(&function_frame)?)? {
        ValidatedGatewayEvent::FunctionCallCompleted { call } => Ok(call),
        _ => Err(GatewayProtocolError::MalformedEvent.into()),
    }
}

fn validated_call(
    call_id: &str,
    name: &str,
    arguments_json: &str,
) -> Result<SchemaValidatedFunctionCall, Box<dyn Error>> {
    let call = normalized_call(call_id, name, arguments_json)?;
    Ok(validate_function_call(call, &registry()?)?)
}

#[test]
fn information_only_call_reaches_policy_through_the_validated_boundary(
) -> Result<(), Box<dyn Error>> {
    let input = PolicyInput::from_validated_call(validated_call(
        "call-information-only-1",
        "get_current_datetime",
        "{}",
    )?);
    let decision = DeterministicPolicyEngine::new().evaluate(input);
    let retained = decision.validated_call();

    assert_eq!(decision.outcome(), PolicyOutcome::Allow);
    assert_eq!(decision.reason(), PolicyReason::InformationOnly);
    assert_eq!(retained.call_id(), "call-information-only-1");
    assert_eq!(retained.tool_name(), "get_current_datetime");
    assert_eq!(retained.tool_contract_version(), 1);
    assert_eq!(retained.risk_class(), RiskClass::InformationOnly);
    assert_eq!(retained.required_permission(), PermissionKind::None);
    assert!(matches!(
        retained.arguments(),
        ValidatedToolArguments::GetCurrentDatetime
    ));
    Ok(())
}

#[test]
fn reversible_call_requires_approval_and_redacts_retained_arguments() -> Result<(), Box<dyn Error>>
{
    let sentinel = "private-policy-task-title";
    let raw_arguments = json!({ "title": sentinel }).to_string();
    let input = PolicyInput::from_validated_call(validated_call(
        "call-reversible-1",
        "create_local_task",
        &raw_arguments,
    )?);
    let input_debug = format!("{input:?}");

    assert!(!input_debug.contains(sentinel));
    assert!(!input_debug.contains(&raw_arguments));

    let decision = DeterministicPolicyEngine::new().evaluate(input);
    let decision_debug = format!("{decision:?}");
    let retained = decision.validated_call();

    assert_eq!(decision.outcome(), PolicyOutcome::RequireApproval);
    assert_eq!(decision.reason(), PolicyReason::ReversibleRequiresApproval);
    assert_eq!(retained.call_id(), "call-reversible-1");
    assert_eq!(retained.tool_name(), "create_local_task");
    assert_eq!(retained.tool_contract_version(), 1);
    assert_eq!(retained.risk_class(), RiskClass::ReversibleLocalAction);
    assert_eq!(retained.required_permission(), PermissionKind::None);
    assert!(matches!(
        retained.arguments(),
        ValidatedToolArguments::CreateLocalTask(arguments) if arguments.title() == sentinel
    ));
    assert!(!decision_debug.contains(sentinel));
    assert!(!decision_debug.contains(&raw_arguments));
    Ok(())
}
