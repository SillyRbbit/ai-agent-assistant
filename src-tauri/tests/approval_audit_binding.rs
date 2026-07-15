use std::error::Error;

use ai_agent_assistant_lib::agent::function_call_validation::validate_function_call;
use ai_agent_assistant_lib::agent::gateway_protocol::{
    GatewayProtocolError, GatewayStreamValidator, ValidatedGatewayEvent, GATEWAY_PROTOCOL_VERSION,
};
use ai_agent_assistant_lib::approvals::manager::{ApprovalManager, InMemoryApprovalManager};
use ai_agent_assistant_lib::approvals::types::{ApprovalCancellationReason, ApprovalDisposition};
use ai_agent_assistant_lib::audit::approval::{ApprovalAuditError, InMemoryApprovalAuditAdapter};
use ai_agent_assistant_lib::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
use ai_agent_assistant_lib::policy::types::{PolicyInput, PolicyOutcome, PolicyReason};
use ai_agent_assistant_lib::tools::registry::{
    InMemoryToolRegistry, ToolRegistry, ToolRegistryResult,
};
use ai_agent_assistant_lib::tools::types::{PermissionKind, RiskClass, ToolDefinition, ToolSchema};
use serde_json::json;

const RUN_ID: &str = "run-approval-audit-binding-1";
const GATEWAY_REQUEST_ID: &str = "gateway-request-approval-audit-binding-1";
const CALL_ID: &str = "call-approval-audit-binding-1";
const TOOL_CONTRACT_VERSION: u16 = 1;

fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
    let mut registry = InMemoryToolRegistry::new();
    registry.register(ToolDefinition::from_schema(
        ToolSchema::GetCurrentDatetimeV1,
    ))?;
    registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
    Ok(registry)
}

#[test]
fn exact_terminal_resolution_reaches_one_redacted_typed_audit_record() -> Result<(), Box<dyn Error>>
{
    let title = "Private executive planning title";
    let mut validator = GatewayStreamValidator::new(
        RUN_ID,
        GATEWAY_REQUEST_ID,
        ["create_local_task".to_owned()],
        TOOL_CONTRACT_VERSION,
    )?;
    let start_frame = json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": RUN_ID,
        "gateway_request_id": GATEWAY_REQUEST_ID,
        "sequence": 0,
        "event": {
            "type": "response_started",
            "provider_response_id": "provider-response-approval-audit-binding-1",
        },
    });
    let function_frame = json!({
        "protocol_version": GATEWAY_PROTOCOL_VERSION,
        "run_id": RUN_ID,
        "gateway_request_id": GATEWAY_REQUEST_ID,
        "sequence": 1,
        "event": {
            "type": "function_call_completed",
            "call_id": CALL_ID,
            "name": "create_local_task",
            "tool_contract_version": TOOL_CONTRACT_VERSION,
            "arguments_json": json!({ "title": title }).to_string(),
        },
    });

    let _ = validator.accept_frame(&serde_json::to_vec(&start_frame)?)?;
    let call = match validator.accept_frame(&serde_json::to_vec(&function_frame)?)? {
        ValidatedGatewayEvent::FunctionCallCompleted { call } => call,
        _ => return Err(GatewayProtocolError::MalformedEvent.into()),
    };
    let validated = validate_function_call(call, &registry()?)?;
    let decision =
        DeterministicPolicyEngine::new().evaluate(PolicyInput::from_validated_call(validated));
    let mut manager = InMemoryApprovalManager::new();
    let approval_id = manager.create_request(decision)?;
    let _presentation = manager.issue_presentation(approval_id)?;
    let resolution = manager.cancel_for_run_termination(approval_id)?;
    let mut adapter = InMemoryApprovalAuditAdapter::new();

    let receipt = adapter.record(&resolution)?;
    assert_eq!(receipt.sequence().value(), 1);
    let [record] = adapter.records() else {
        return Err("expected one approval audit record".into());
    };
    assert_eq!(record.sequence(), receipt.sequence());
    assert_eq!(record.approval_id(), approval_id);
    assert_eq!(record.run_id(), RUN_ID);
    assert_eq!(record.gateway_request_id(), GATEWAY_REQUEST_ID);
    assert_eq!(record.call_id(), CALL_ID);
    assert_eq!(record.tool_name(), "create_local_task");
    assert_eq!(record.tool_contract_version(), TOOL_CONTRACT_VERSION);
    assert_eq!(record.risk_class(), RiskClass::ReversibleLocalAction);
    assert_eq!(record.required_permission(), PermissionKind::None);
    assert_eq!(record.policy_outcome(), PolicyOutcome::RequireApproval);
    assert_eq!(
        record.policy_reason(),
        PolicyReason::ReversibleRequiresApproval
    );
    assert_eq!(
        record.disposition(),
        ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
    );
    assert_eq!(record.interaction_evidence(), None);

    let record_debug = format!("{record:?}");
    for excluded in [title, RUN_ID, GATEWAY_REQUEST_ID, CALL_ID] {
        assert!(!record_debug.contains(excluded));
    }
    assert_eq!(
        adapter.record(&resolution),
        Err(ApprovalAuditError::DuplicateResolution)
    );
    assert_eq!(adapter.records().len(), 1);
    Ok(())
}
