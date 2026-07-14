use std::error::Error;

use ai_agent_assistant_lib::agent::function_call_validation::validate_function_call;
use ai_agent_assistant_lib::agent::gateway_protocol::{
    GatewayProtocolError, GatewayStreamValidator, ValidatedGatewayEvent, GATEWAY_PROTOCOL_VERSION,
};
use ai_agent_assistant_lib::approvals::manager::{
    ApprovalError, ApprovalManager, InMemoryApprovalManager,
};
use ai_agent_assistant_lib::approvals::types::{
    ApprovalAction, ApprovalChoice, ApprovalDisposition, ApprovalRecipients, ApprovalReversibility,
    ApprovalRisk, ApprovalSchedule, ApprovalTarget,
};
use ai_agent_assistant_lib::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
use ai_agent_assistant_lib::policy::types::{PolicyDecision, PolicyInput, PolicyOutcome};
use ai_agent_assistant_lib::tools::registry::{
    InMemoryToolRegistry, ToolRegistry, ToolRegistryResult,
};
use ai_agent_assistant_lib::tools::types::{PermissionKind, RiskClass, ToolDefinition, ToolSchema};
use serde_json::json;

const RUN_ID: &str = "run-approval-binding-1";
const GATEWAY_REQUEST_ID: &str = "gateway-request-approval-binding-1";
const CALL_ID: &str = "call-approval-binding-1";
const TOOL_CONTRACT_VERSION: u16 = 1;

fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
    let mut registry = InMemoryToolRegistry::new();
    registry.register(ToolDefinition::from_schema(
        ToolSchema::GetCurrentDatetimeV1,
    ))?;
    registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
    Ok(registry)
}

fn policy_decision(
    run_id: &str,
    gateway_request_id: &str,
    call_id: &str,
    name: &str,
    arguments_json: &str,
) -> Result<PolicyDecision, Box<dyn Error>> {
    let mut validator = GatewayStreamValidator::new(
        run_id,
        gateway_request_id,
        [name.to_owned()],
        TOOL_CONTRACT_VERSION,
    )?;
    let start_frame = json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": run_id,
        "gateway_request_id": gateway_request_id,
        "sequence": 0,
        "event": {
            "type": "response_started",
            "provider_response_id": "provider-response-approval-binding-1",
        },
    });
    let function_frame = json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": run_id,
        "gateway_request_id": gateway_request_id,
        "sequence": 1,
        "event": {
            "type": "function_call_completed",
            "call_id": call_id,
            "name": name,
            "tool_contract_version": TOOL_CONTRACT_VERSION,
            "arguments_json": arguments_json,
        },
    });

    let _ = validator.accept_frame(&serde_json::to_vec(&start_frame)?)?;
    let call = match validator.accept_frame(&serde_json::to_vec(&function_frame)?)? {
        ValidatedGatewayEvent::FunctionCallCompleted { call } => call,
        _ => return Err(GatewayProtocolError::MalformedEvent.into()),
    };
    let validated = validate_function_call(call, &registry()?)?;
    Ok(DeterministicPolicyEngine::new().evaluate(PolicyInput::from_validated_call(validated)))
}

fn local_task_decision(title: &str) -> Result<PolicyDecision, Box<dyn Error>> {
    policy_decision(
        RUN_ID,
        GATEWAY_REQUEST_ID,
        CALL_ID,
        "create_local_task",
        &json!({ "title": title }).to_string(),
    )
}

#[test]
fn exact_policy_subject_reaches_one_terminal_approval() -> Result<(), Box<dyn Error>> {
    let title = "Review exact approval binding";
    let mut manager = InMemoryApprovalManager::new();
    let id = manager.create_request(local_task_decision(title)?)?;
    let Some(view) = manager.pending()? else {
        return Err(ApprovalError::NotFound(id.value()).into());
    };

    assert_eq!(view.id(), id);
    assert_eq!(view.run_id(), RUN_ID);
    assert_eq!(view.gateway_request_id(), GATEWAY_REQUEST_ID);
    assert_eq!(view.call_id(), CALL_ID);
    assert_eq!(view.tool_name(), "create_local_task");
    assert_eq!(view.tool_contract_version(), TOOL_CONTRACT_VERSION);
    assert_eq!(view.policy_outcome(), PolicyOutcome::RequireApproval);
    assert_eq!(view.risk_class(), RiskClass::ReversibleLocalAction);
    assert_eq!(view.required_permission(), PermissionKind::None);

    let preview = view.preview();
    assert_eq!(preview.action(), ApprovalAction::CreateLocalTask);
    assert_eq!(preview.target(), ApprovalTarget::LocalTaskList);
    assert_eq!(preview.affected_data().value(), title);
    assert_eq!(preview.schedule(), ApprovalSchedule::NotScheduled);
    assert_eq!(preview.recipients(), ApprovalRecipients::None);
    assert_eq!(preview.reversibility(), ApprovalReversibility::Reversible);
    assert_eq!(preview.risk(), ApprovalRisk::CreatesLocalTask);

    let resolution = manager.decide(id, ApprovalChoice::Approve)?;
    assert_eq!(resolution.id(), id);
    assert_eq!(resolution.disposition(), ApprovalDisposition::Approved);
    assert_eq!(resolution.run_id(), RUN_ID);
    assert_eq!(resolution.gateway_request_id(), GATEWAY_REQUEST_ID);
    assert_eq!(resolution.call_id(), CALL_ID);
    let Some(resolved_preview) = resolution.preview() else {
        return Err(ApprovalError::UnsupportedApprovalSubject.into());
    };
    assert_eq!(resolved_preview.affected_data().value(), title);

    assert_eq!(
        manager.decide(id, ApprovalChoice::Approve),
        Err(ApprovalError::AlreadyConsumed(id.value()))
    );
    assert_eq!(
        manager.create_request(local_task_decision("Changed replay content")?),
        Err(ApprovalError::DuplicateSubject)
    );
    Ok(())
}

#[test]
fn information_only_policy_result_cannot_enter_approval() -> Result<(), Box<dyn Error>> {
    let decision = policy_decision(
        "run-approval-binding-information-1",
        "gateway-request-approval-binding-information-1",
        "call-approval-binding-information-1",
        "get_current_datetime",
        "{}",
    )?;
    assert_eq!(decision.outcome(), PolicyOutcome::Allow);

    let mut manager = InMemoryApprovalManager::new();
    assert_eq!(
        manager.create_request(decision),
        Err(ApprovalError::IneligiblePolicyOutcome {
            actual: PolicyOutcome::Allow,
        })
    );
    Ok(())
}
