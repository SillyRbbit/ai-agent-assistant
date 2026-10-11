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

    const EXAMPLE_IDENTIFIER: &str = "com.cortexa.example.nativeapproval";

    pub fn main() -> ExitCode {
        match run_host() {
            Ok(0) => ExitCode::SUCCESS,
            Ok(_) => ExitCode::FAILURE,
            Err(_) => {
                eprintln!("native approval example failed");
                ExitCode::FAILURE
            }
        }
    }

    fn configure_host(config: &mut tauri::utils::config::Config) {
        // Do not load the product UI or use its persistent WebView identity.
        config.identifier = EXAMPLE_IDENTIFIER.into();
        config.app.windows.clear();
    }

    fn run_host() -> Result<i32, Box<dyn Error>> {
        let mut context = tauri::generate_context!();
        configure_host(context.config_mut());
        // This standalone host installs no product commands, services or database.
        let app = tauri::Builder::default().build(context)?;
        Ok(app.run_return(|app, event| {
            if let tauri::RunEvent::Ready = event {
                let code = match show_example(app) {
                    Ok(resolution) => {
                        print_resolution(&resolution);
                        0
                    }
                    Err(_) => {
                        eprintln!("native approval example failed");
                        1
                    }
                };
                app.exit(code);
            }
        }))
    }

    fn show_example(app: &tauri::AppHandle) -> Result<ApprovalResolution, Box<dyn Error>> {
        let owner = tauri::WebviewWindowBuilder::new(
            app,
            "main",
            tauri::WebviewUrl::External("about:blank".parse()?),
        )
        .title("Cortexa synthetic approval example")
        .incognito(true)
        .build()?;
        // Keep the owner and its captured borrowed handles alive through the
        // synchronous decision. Parent acquisition failure never shows a dialog.
        let source = MacOsNativeApprovalDecisionSource::new(&owner)?;
        let mut manager = InMemoryApprovalManager::new();
        let id = manager.create_request(local_task_decision(
            "Review the trusted native approval decision source",
        )?)?;
        let presentation = manager.issue_presentation(id)?;
        let outcome = source.request_decision(presentation);
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use ai_agent_assistant_lib::policy::types::PolicyOutcome;

        #[test]
        fn host_configuration_preserves_security_without_product_windows(
        ) -> Result<(), Box<dyn Error>> {
            let mut config: tauri::utils::config::Config =
                serde_json::from_str(include_str!("../tauri.conf.json"))?;
            let security = serde_json::to_value(&config.app.security)?;
            configure_host(&mut config);
            assert_eq!(config.identifier, EXAMPLE_IDENTIFIER);
            assert!(config.app.windows.is_empty());
            assert_eq!(serde_json::to_value(&config.app.security)?, security);
            Ok(())
        }

        #[test]
        fn synthetic_request_still_requires_approval_and_rejects_replay(
        ) -> Result<(), Box<dyn Error>> {
            let decision =
                local_task_decision("Review the trusted native approval decision source")?;
            assert_eq!(decision.outcome(), PolicyOutcome::RequireApproval);
            let mut manager = InMemoryApprovalManager::new();
            let id = manager.create_request(decision)?;
            let _presentation = manager.issue_presentation(id)?;
            assert!(manager.issue_presentation(id).is_err());
            let resolution = manager.cancel_for_run_termination(id)?;
            assert_eq!(
                resolution.disposition(),
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
            );
            assert!(manager.cancel_for_run_termination(id).is_err());
            Ok(())
        }

        #[test]
        fn synthetic_input_validation_remains_strict() {
            for invalid in ["", " leading whitespace", "line\nbreak"] {
                assert!(local_task_decision(invalid).is_err());
            }
        }
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
