#[cfg(target_os = "macos")]
mod macos {
    use std::error::Error;
    use std::process::ExitCode;

    use ai_agent_assistant_lib::agent::function_call_validation::validate_function_call;
    use ai_agent_assistant_lib::agent::gateway_protocol::{
        GatewayProtocolError, GatewayStreamValidator, ValidatedGatewayEvent,
        GATEWAY_PROTOCOL_VERSION,
    };
    use ai_agent_assistant_lib::approvals::decision_source::MacOsNativeApprovalDecisionSource;
    use ai_agent_assistant_lib::approvals::manager::{ApprovalManager, InMemoryApprovalManager};
    use ai_agent_assistant_lib::approvals::types::{
        ApprovalCancellationReason, ApprovalDisposition, ApprovalResolution,
    };
    use ai_agent_assistant_lib::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
    use ai_agent_assistant_lib::policy::types::{PolicyDecision, PolicyInput};
    use ai_agent_assistant_lib::tools::registry::{
        InMemoryToolRegistry, ToolRegistry, ToolRegistryResult,
    };
    use ai_agent_assistant_lib::tools::types::{ToolDefinition, ToolSchema};
    use serde_json::json;

    const RUN_ID: &str = "run-native-approval-example-1";
    const GATEWAY_REQUEST_ID: &str = "gateway-request-native-approval-example-1";
    const CALL_ID: &str = "call-native-approval-example-1";
    const TOOL_CONTRACT_VERSION: u16 = 1;

    pub fn main() -> ExitCode {
        match run() {
            Ok(resolution) => {
                print_resolution(&resolution);
                ExitCode::SUCCESS
            }
            Err(_) => {
                eprintln!("native approval example failed");
                ExitCode::FAILURE
            }
        }
    }

    fn run() -> Result<ApprovalResolution, Box<dyn Error>> {
        let mut manager = InMemoryApprovalManager::new();
        let id = manager.create_request(local_task_decision(
            "Review the trusted native approval decision source",
        )?)?;
        let presentation = manager.issue_presentation(id)?;
        let outcome = MacOsNativeApprovalDecisionSource::new().request_decision(presentation);
        Ok(manager.resolve_source_outcome(outcome)?)
    }

    fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
        let mut registry = InMemoryToolRegistry::new();
        registry.register(ToolDefinition::from_schema(
            ToolSchema::GetCurrentDatetimeV1,
        ))?;
        registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
        Ok(registry)
    }

    fn local_task_decision(title: &str) -> Result<PolicyDecision, Box<dyn Error>> {
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
                "provider_response_id": "provider-response-native-approval-example-1",
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
        Ok(DeterministicPolicyEngine::new().evaluate(PolicyInput::from_validated_call(validated)))
    }

    fn print_resolution(resolution: &ApprovalResolution) {
        let label = match resolution.disposition() {
            ApprovalDisposition::Approved => "approved",
            ApprovalDisposition::Rejected => "rejected",
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated) => {
                "cancelled: run terminated"
            }
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::EditRequested) => {
                "cancelled: edit requested"
            }
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::NativeNoDecision) => {
                "cancelled: native no decision"
            }
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed) => {
                "cancelled: source failed"
            }
            ApprovalDisposition::Expired => "expired",
        };
        println!("approval {}: {label}", resolution.id().value());
    }
}

#[cfg(target_os = "macos")]
fn main() -> std::process::ExitCode {
    macos::main()
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("native approval dialog example is available only on macOS");
}
