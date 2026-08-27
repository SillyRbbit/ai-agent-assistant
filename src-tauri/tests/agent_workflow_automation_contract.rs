use std::error::Error;

use ai_agent_assistant_lib::agent::definition::AgentId;
use ai_agent_assistant_lib::agent::gateway_protocol::MAX_GATEWAY_REQUEST_BYTES;
use ai_agent_assistant_lib::agent::native_runtime::NativeAgentRuntime;
use ai_agent_assistant_lib::agent::orchestrator::{
    AgentOrchestrator, AgentOrchestratorError, AgentWorkflowSelection, DelegationProposal,
    ResearchKnowledgeWorkflowAcceptance,
};
use ai_agent_assistant_lib::agent::registry::AgentRegistry;
use ai_agent_assistant_lib::agent::research_knowledge::FinalSynthesisStatus;
use ai_agent_assistant_lib::agent::runtime::{
    AgentRuntime, RuntimeCapability, RuntimeEventAcceptance, RuntimeEventEnvelope, RuntimeFailure,
    RuntimeFailureCode, RuntimeId, RuntimeOutputText, RuntimeResponseId, RuntimeTurnRequest,
    UntrustedRuntimeEvent, UntrustedRuntimeToolProposal,
};
use ai_agent_assistant_lib::agent::task::{
    AgentExecutionContext, AgentTaskCancellationOutcome, AgentTaskFailureCode, AgentTaskOutcome,
    AgentTaskStatus,
};
use ai_agent_assistant_lib::agent::workflow_automation::{
    WorkflowAutomationAuditOutcome, WorkflowAutomationContinuationFailure, WorkflowAutomationError,
    WorkflowAutomationPartialFailureCode, WorkflowAutomationProposalRequest,
    WorkflowAutomationStage, WorkflowAutomationSynthesisStatus,
    WorkflowAutomationWorkflowAcceptance, WorkflowAutomationWorkflowEvent, WorkflowFailureBehavior,
    WorkflowLimit, WorkflowManualDispatchAcceptance, WorkflowManualDispatchAvailability,
    WorkflowManualDispatchEvent, WorkflowManualDispatchStatus, WorkflowProposalId,
    WorkflowProposalStageOutcome, WorkflowStep, WorkflowSynthesisDisposition,
    WorkflowTemplateCatalog, WorkflowTemplateId, WorkflowValidationDisposition,
    WorkflowValidationFailureCode, MAX_NESTED_WORKFLOW_DEPTH, MAX_WORKFLOW_AGENT_TASK_STEPS,
    MAX_WORKFLOW_AUDIT_RECORDS, MAX_WORKFLOW_DURATION_SECONDS, MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS,
    MAX_WORKFLOW_OBJECTIVE_BYTES, MAX_WORKFLOW_PROPOSAL_BYTES, MAX_WORKFLOW_RETRIES,
    MAX_WORKFLOW_STEPS, WORKFLOW_AUTOMATION_CANCELLED_ISSUE,
    WORKFLOW_AUTOMATION_DOCUMENT_PROPOSAL_ONLY_ISSUE, WORKFLOW_AUTOMATION_FAILED_ISSUE,
    WORKFLOW_AUTOMATION_REJECTED_ISSUE, WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE,
};
use serde_json::{json, Value};

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

const RESEARCH_JSON: &str = r#"{"findings":[{"statement":"Approach A is simpler and less costly","source_ids":["approach-a"],"confidence":"high"},{"statement":"Approach B has higher scale potential","source_ids":["approach-b"],"confidence":"medium"}],"unresolved_questions":["What scale is required?"],"limitations":["Only deterministic fixture evidence was supplied"],"recommended_follow_up":"Review against approved operational requirements"}"#;
const KNOWLEDGE_JSON: &str = r#"{"sections":[{"heading":"Decision factors","body":"A favors simplicity while B favors scale.","source_ids":["approach-a","approach-b"]}],"extracted_facts":[{"statement":"A has lower operating cost.","source_ids":["approach-a"]}],"contradictions":[],"summary":"Choose according to required scale and operations capacity.","reusable_knowledge_proposal":"Retain this comparison only after explicit application review.","artifact_outline":"Context; evidence; trade-offs; decision","incomplete":false}"#;
const RESEARCH_FINAL_SYNTHESIS: &str = r#"{"version":"v1","answer":"Fixture-based decision brief: A is simpler [approach-a]; B may scale further [approach-b]. Verify before acting.","source_ids":["approach-a","approach-b"],"fixture_based":true,"status":"complete"}"#;
const RESEARCH_PARTIAL_SYNTHESIS: &str = r#"{"version":"v1","answer":"The fixture workflow is partial because Research was unavailable.","source_ids":[],"fixture_based":true,"status":"partial"}"#;

fn canonical_proposal_json(template_id: WorkflowTemplateId) -> Result<String, Box<dyn Error>> {
    let input = WorkflowTemplateCatalog::built_in()
        .proposal_request(template_id)?
        .build_planner_input()?;
    let (_, selected) = input
        .split_once("selected_template(application-owned):\n")
        .ok_or("missing application-owned template marker")?;
    let (proposal, _) = selected
        .split_once("\nReturn exactly one strict WorkflowProposalV1 JSON object.")
        .ok_or("missing strict planner instruction marker")?;
    Ok(proposal.to_owned())
}

fn proposal_id(value: &str) -> Result<WorkflowProposalId, Box<dyn Error>> {
    Ok(WorkflowProposalId::new(value)?)
}

fn canonical_proposal_value(template_id: WorkflowTemplateId) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_str(&canonical_proposal_json(
        template_id,
    )?)?)
}

fn assert_proposal_error(
    request: &WorkflowAutomationProposalRequest,
    registry: &AgentRegistry,
    value: &Value,
    expected: WorkflowAutomationError,
) -> Result<(), Box<dyn Error>> {
    let raw = serde_json::to_string(value)?;
    let error = request
        .parse_proposal(proposal_id("proposal-adversarial")?, &raw, registry)
        .err()
        .ok_or("adversarial proposal unexpectedly validated")?;
    assert_eq!(error, expected);
    Ok(())
}

fn synthesis_json(
    template_id: WorkflowTemplateId,
    proposal_id: &WorkflowProposalId,
    disposition: &str,
    status: &str,
    unresolved_issues: &[&str],
) -> Result<String, Box<dyn Error>> {
    Ok(serde_json::to_string(&json!({
        "version": "v1",
        "summary": if status == "complete" {
            WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE.to_owned()
        } else {
            format!("Partial. {WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE}")
        },
        "template_id": template_id,
        "proposal_id": proposal_id.as_str(),
        "validation_disposition": disposition,
        "status": status,
        "unresolved_issues": unresolved_issues,
        "fixture_based": true,
        "unwired": true,
        "workflow_executed": false,
        "tools_executed": false,
        "approvals_requested": false,
        "effects_performed": false,
    }))?)
}

fn started(
    context: &AgentExecutionContext,
    response_id: &str,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new(response_id)?,
        },
    ))
}

fn delta(
    context: &AgentExecutionContext,
    output: &str,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        1,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new(output)?,
        },
    ))
}

fn completed(context: &AgentExecutionContext) -> RuntimeEventEnvelope {
    RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        2,
        UntrustedRuntimeEvent::ResponseCompleted,
    )
}

fn failed(context: &AgentExecutionContext) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        1,
        UntrustedRuntimeEvent::ResponseFailed {
            failure: RuntimeFailure::new(RuntimeFailureCode::ProviderUnavailable, true, None)?,
        },
    ))
}

fn runtime_cancelled(
    context: &AgentExecutionContext,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        1,
        UntrustedRuntimeEvent::ResponseFailed {
            failure: RuntimeFailure::new(RuntimeFailureCode::Cancelled, false, None)?,
        },
    ))
}

fn active_child_context(
    orchestrator: &AgentOrchestrator<MockAgentRuntime>,
) -> Result<AgentExecutionContext, Box<dyn Error>> {
    let task = orchestrator
        .active_child_task()
        .ok_or("missing active child task")?;
    Ok(orchestrator.current_context(task.id())?)
}

fn complete_stage(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    response_id: &str,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    let task_id = context.task_id().clone();
    orchestrator.accept_runtime_event(&task_id, started(context, response_id)?)?;
    orchestrator.accept_runtime_event(&task_id, delta(context, output)?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(&task_id, completed(context))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    Ok(())
}

fn start_proposal(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    template_id: WorkflowTemplateId,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let request = WorkflowTemplateCatalog::built_in().proposal_request(template_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let acceptance = orchestrator.start_workflow_automation_proposal(&root, request)?;
    let planner = match acceptance {
        WorkflowAutomationWorkflowAcceptance::ProposalStarted { context } => context,
        WorkflowAutomationWorkflowAcceptance::PersonalFallbackStarted { .. } => {
            return Err("Workflow Automation unexpectedly failed to start".into())
        }
    };
    Ok((root, planner))
}

fn finish_successful_proposal(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    template_id: WorkflowTemplateId,
) -> Result<(AgentExecutionContext, WorkflowProposalId), Box<dyn Error>> {
    let (root, planner) = start_proposal(orchestrator, template_id)?;
    let proposal_id = orchestrator
        .workflow_automation_audit_records()
        .first()
        .ok_or("missing proposal-start audit")?
        .proposal_id()
        .clone();
    complete_stage(
        orchestrator,
        &planner,
        "workflow-proposal",
        &canonical_proposal_json(template_id)?,
    )?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    let (disposition, unresolved): (&str, &[&str]) = if template_id.is_manually_dispatchable() {
        ("ready-for-manual-dispatch", &[])
    } else {
        (
            "proposal-only",
            &[WORKFLOW_AUTOMATION_DOCUMENT_PROPOSAL_ONLY_ISSUE],
        )
    };
    complete_stage(
        orchestrator,
        &synthesis,
        "workflow-synthesis",
        &synthesis_json(
            template_id,
            &proposal_id,
            disposition,
            "complete",
            unresolved,
        )?,
    )?;
    Ok((root, proposal_id))
}

fn step_projection(step: &WorkflowStep) -> Option<(&str, AgentId, Vec<&str>, &str)> {
    match step {
        WorkflowStep::AgentTask(value) => Some((
            value.id().as_str(),
            value.agent_id(),
            value
                .dependencies()
                .iter()
                .map(|dependency| dependency.as_str())
                .collect(),
            value.expected_output(),
        )),
        WorkflowStep::Synthesis(value) => Some((
            value.id().as_str(),
            value.agent_id(),
            value
                .dependencies()
                .iter()
                .map(|dependency| dependency.as_str())
                .collect(),
            value.expected_output(),
        )),
        WorkflowStep::GovernedTool(_) | WorkflowStep::ApprovalCheckpoint(_) => None,
    }
}

#[test]
fn five_application_owned_templates_are_closed_bounded_and_exact() -> Result<(), Box<dyn Error>> {
    let expected = [
        (
            WorkflowTemplateId::ResearchBriefV1,
            vec![
                ("research", AgentId::Research, vec![], "research-result-v1"),
                (
                    "knowledge",
                    AgentId::KnowledgeDocument,
                    vec!["research"],
                    "knowledge-result-v1",
                ),
                (
                    "synthesis",
                    AgentId::PersonalAssistant,
                    vec!["knowledge"],
                    "research-knowledge-synthesis-v1",
                ),
            ],
        ),
        (
            WorkflowTemplateId::CodeQualityReviewV1,
            vec![
                ("coding", AgentId::Coding, vec![], "change-proposal-v1"),
                (
                    "qa",
                    AgentId::QaValidation,
                    vec!["coding"],
                    "validation-report-v1",
                ),
                (
                    "security",
                    AgentId::SecurityRisk,
                    vec!["qa"],
                    "risk-assessment-v1",
                ),
                (
                    "synthesis",
                    AgentId::PersonalAssistant,
                    vec!["security"],
                    "engineering-review-synthesis-v1",
                ),
            ],
        ),
        (
            WorkflowTemplateId::InfrastructureAssessmentV1,
            vec![
                (
                    "cloud",
                    AgentId::CloudInfrastructure,
                    vec![],
                    "infrastructure-assessment-v1",
                ),
                (
                    "qa",
                    AgentId::QaValidation,
                    vec!["cloud"],
                    "validation-report-v1",
                ),
                (
                    "security",
                    AgentId::SecurityRisk,
                    vec!["qa"],
                    "risk-assessment-v1",
                ),
                (
                    "synthesis",
                    AgentId::PersonalAssistant,
                    vec!["security"],
                    "cloud-infrastructure-synthesis-v1",
                ),
            ],
        ),
        (
            WorkflowTemplateId::SystemsIncidentAnalysisV1,
            vec![
                (
                    "systems",
                    AgentId::SystemsOperations,
                    vec![],
                    "operational-assessment-v1",
                ),
                (
                    "qa",
                    AgentId::QaValidation,
                    vec!["systems"],
                    "validation-report-v1",
                ),
                (
                    "security",
                    AgentId::SecurityRisk,
                    vec!["qa"],
                    "risk-assessment-v1",
                ),
                (
                    "synthesis",
                    AgentId::PersonalAssistant,
                    vec!["security"],
                    "systems-operations-synthesis-v1",
                ),
            ],
        ),
        (
            WorkflowTemplateId::DocumentToActionPlanV1,
            vec![
                (
                    "knowledge",
                    AgentId::KnowledgeDocument,
                    vec![],
                    "document-action-input-v1",
                ),
                (
                    "workflow-proposal",
                    AgentId::WorkflowAutomation,
                    vec!["knowledge"],
                    "workflow-proposal-v1",
                ),
                (
                    "synthesis",
                    AgentId::PersonalAssistant,
                    vec!["workflow-proposal"],
                    "document-action-plan-v1",
                ),
            ],
        ),
    ];

    assert_eq!(WorkflowTemplateId::ALL.len(), 5);
    for (template_id, expected_steps) in expected {
        let request = WorkflowTemplateCatalog::built_in().proposal_request(template_id)?;
        let definition = request.template();
        assert_eq!(request.template_id(), template_id);
        assert_eq!(definition.template_id(), template_id);
        assert_eq!(
            definition.failure_behavior(),
            WorkflowFailureBehavior::PartialSynthesis
        );
        assert_eq!(definition.limits(), WorkflowLimit::canonical());
        assert_eq!(definition.limits().max_steps(), MAX_WORKFLOW_STEPS as u8);
        assert_eq!(
            definition.limits().max_agent_tasks(),
            MAX_WORKFLOW_AGENT_TASK_STEPS as u8
        );
        assert_eq!(
            definition.limits().max_tool_steps(),
            MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS as u8
        );
        assert_eq!(
            definition.limits().max_duration_seconds(),
            MAX_WORKFLOW_DURATION_SECONDS
        );
        assert_eq!(definition.limits().max_retries(), MAX_WORKFLOW_RETRIES);
        assert_eq!(
            definition.limits().max_nested_workflows(),
            MAX_NESTED_WORKFLOW_DEPTH
        );
        let projected = definition
            .steps()
            .iter()
            .map(step_projection)
            .collect::<Option<Vec<_>>>()
            .ok_or("built-in workflows must not contain tools or approval checkpoints")?;
        assert_eq!(projected, expected_steps);
        let raw = canonical_proposal_json(template_id)?;
        assert!(raw.contains("\"fixture_based\":true"));
        assert!(raw.contains("\"execution_authorized\":false"));
        assert!(!raw.contains("governed-tool"));
        assert!(!raw.contains("approval-checkpoint"));
    }

    assert!(WorkflowTemplateId::ResearchBriefV1.is_manually_dispatchable());
    assert!(WorkflowTemplateId::CodeQualityReviewV1.is_manually_dispatchable());
    assert!(WorkflowTemplateId::InfrastructureAssessmentV1.is_manually_dispatchable());
    assert!(WorkflowTemplateId::SystemsIncidentAnalysisV1.is_manually_dispatchable());
    assert!(!WorkflowTemplateId::DocumentToActionPlanV1.is_manually_dispatchable());
    Ok(())
}

#[test]
fn canonical_proposals_validate_for_a_through_e_with_application_derived_disposition(
) -> Result<(), Box<dyn Error>> {
    let registry = AgentRegistry::built_in()?;
    for template_id in WorkflowTemplateId::ALL {
        let request = WorkflowTemplateCatalog::built_in().proposal_request(template_id)?;
        let id = proposal_id(&format!("proposal-canonical-{}", template_id as u8))?;
        let proposal = request.parse_proposal(
            id.clone(),
            &canonical_proposal_json(template_id)?,
            &registry,
        )?;
        assert_eq!(proposal.proposal().id(), &id);
        assert_eq!(proposal.proposal().definition(), request.template());
        assert_eq!(
            proposal.disposition(),
            if template_id.is_manually_dispatchable() {
                WorkflowValidationDisposition::ReadyForManualDispatch
            } else {
                WorkflowValidationDisposition::ProposalOnly
            }
        );
        let debug = format!("{request:?} {proposal:?}");
        assert!(!debug.contains(request.objective()));
        assert!(!debug.contains(id.as_str()));
    }
    Ok(())
}

#[test]
fn strict_parser_rejects_graph_agent_limit_shell_nesting_and_definition_mutation(
) -> Result<(), Box<dyn Error>> {
    let registry = AgentRegistry::built_in()?;
    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;

    let malformed = request
        .parse_proposal(proposal_id("proposal-malformed")?, "{", &registry)
        .err()
        .ok_or("malformed proposal unexpectedly validated")?;
    assert_eq!(
        malformed,
        WorkflowAutomationError::InvalidStructuredProposal
    );

    let duplicate_key = request
        .parse_proposal(
            proposal_id("proposal-duplicate-json-key")?,
            r#"{"version":"v1","version":"v1"}"#,
            &registry,
        )
        .err()
        .ok_or("duplicate-key proposal unexpectedly validated")?;
    assert_eq!(
        duplicate_key,
        WorkflowAutomationError::InvalidStructuredProposal
    );

    let oversized_raw = "x".repeat(MAX_WORKFLOW_PROPOSAL_BYTES + 1);
    let oversized_error = request
        .parse_proposal(
            proposal_id("proposal-oversized")?,
            &oversized_raw,
            &registry,
        )
        .err()
        .ok_or("oversized proposal unexpectedly validated")?;
    assert_eq!(oversized_error, WorkflowAutomationError::BoundExceeded);

    let canonical = canonical_proposal_value(WorkflowTemplateId::ResearchBriefV1)?;

    let mut excessive_objective = canonical.clone();
    excessive_objective["objective"] = json!("x".repeat(MAX_WORKFLOW_OBJECTIVE_BYTES + 1));
    assert_proposal_error(
        &request,
        &registry,
        &excessive_objective,
        WorkflowAutomationError::BoundExceeded,
    )?;

    let mut unknown_field = canonical.clone();
    unknown_field["shell_command"] = json!("fixture-only-no-op");
    assert_proposal_error(
        &request,
        &registry,
        &unknown_field,
        WorkflowAutomationError::InvalidStructuredProposal,
    )?;

    let mut unsupported_step = canonical.clone();
    unsupported_step["steps"][0]["type"] = json!("arbitrary-shell");
    assert_proposal_error(
        &request,
        &registry,
        &unsupported_step,
        WorkflowAutomationError::UnsupportedStep,
    )?;

    let mut unknown_agent = canonical.clone();
    unknown_agent["steps"][0]["agent_id"] = json!("unregistered-agent");
    assert_proposal_error(
        &request,
        &registry,
        &unknown_agent,
        WorkflowAutomationError::UnknownAgent,
    )?;

    let mut duplicate_step = canonical.clone();
    duplicate_step["steps"][1]["id"] = json!("research");
    assert_proposal_error(
        &request,
        &registry,
        &duplicate_step,
        WorkflowAutomationError::DuplicateStep,
    )?;

    let mut duplicate_dependency = canonical.clone();
    duplicate_dependency["steps"][1]["depends_on"] = json!(["research", "research"]);
    assert_proposal_error(
        &request,
        &registry,
        &duplicate_dependency,
        WorkflowAutomationError::DuplicateDependency,
    )?;

    let mut unknown_dependency = canonical.clone();
    unknown_dependency["steps"][1]["depends_on"] = json!(["missing-step"]);
    assert_proposal_error(
        &request,
        &registry,
        &unknown_dependency,
        WorkflowAutomationError::UnknownDependency,
    )?;

    let mut self_dependency = canonical.clone();
    self_dependency["steps"][0]["depends_on"] = json!(["research"]);
    assert_proposal_error(
        &request,
        &registry,
        &self_dependency,
        WorkflowAutomationError::SelfDependency,
    )?;

    let mut cycle = canonical.clone();
    cycle["steps"][0]["depends_on"] = json!(["knowledge"]);
    assert_proposal_error(&request, &registry, &cycle, WorkflowAutomationError::Cycle)?;

    let mut excessive_steps = canonical.clone();
    excessive_steps["steps"] = json!([
        {"id":"one","type":"synthesis","agent_id":"personal-assistant","depends_on":[],"expected_output":"one-v1"},
        {"id":"two","type":"synthesis","agent_id":"personal-assistant","depends_on":[],"expected_output":"two-v1"},
        {"id":"three","type":"synthesis","agent_id":"personal-assistant","depends_on":[],"expected_output":"three-v1"},
        {"id":"four","type":"synthesis","agent_id":"personal-assistant","depends_on":[],"expected_output":"four-v1"},
        {"id":"five","type":"synthesis","agent_id":"personal-assistant","depends_on":[],"expected_output":"five-v1"}
    ]);
    assert_proposal_error(
        &request,
        &registry,
        &excessive_steps,
        WorkflowAutomationError::BoundExceeded,
    )?;

    let mut excessive_retries = canonical.clone();
    excessive_retries["limits"]["max_retries"] = json!(1);
    assert_proposal_error(
        &request,
        &registry,
        &excessive_retries,
        WorkflowAutomationError::BoundExceeded,
    )?;

    let mut excessive_duration = canonical.clone();
    excessive_duration["limits"]["max_duration_seconds"] = json!(MAX_WORKFLOW_DURATION_SECONDS + 1);
    assert_proposal_error(
        &request,
        &registry,
        &excessive_duration,
        WorkflowAutomationError::BoundExceeded,
    )?;

    let mut excessive_dependencies = canonical.clone();
    excessive_dependencies["steps"][1]["depends_on"] =
        json!(["research", "synthesis", "research", "synthesis"]);
    assert_proposal_error(
        &request,
        &registry,
        &excessive_dependencies,
        WorkflowAutomationError::BoundExceeded,
    )?;

    for authority_field in [
        "nested_workflow",
        "self_modifying",
        "execution_authorized",
        "tools_executed",
        "approvals_requested",
        "effects_performed",
    ] {
        let mut authority_claim = canonical.clone();
        authority_claim[authority_field] = json!(true);
        assert_proposal_error(
            &request,
            &registry,
            &authority_claim,
            WorkflowAutomationError::AuthorityClaim,
        )?;
    }

    let mut self_modifying_definition = canonical.clone();
    self_modifying_definition["definition_patch"] = json!({"replace": "steps"});
    assert_proposal_error(
        &request,
        &registry,
        &self_modifying_definition,
        WorkflowAutomationError::InvalidStructuredProposal,
    )?;

    let mut template_deviation = canonical;
    template_deviation["steps"][0]["expected_output"] = json!("altered-output-v1");
    assert_proposal_error(
        &request,
        &registry,
        &template_deviation,
        WorkflowAutomationError::TemplateMismatch,
    )?;
    Ok(())
}

#[test]
fn tool_and_checkpoint_steps_are_recognized_validated_and_never_executable(
) -> Result<(), Box<dyn Error>> {
    let registry = AgentRegistry::built_in()?;
    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
    let mut proposal = canonical_proposal_value(WorkflowTemplateId::ResearchBriefV1)?;

    proposal["steps"] = json!([{
        "id": "shell-attempt",
        "type": "governed-tool",
        "tool_name": "unregistered_shell_tool",
        "tool_contract_version": 1,
        "arguments": {},
        "depends_on": []
    }]);
    assert_proposal_error(
        &request,
        &registry,
        &proposal,
        WorkflowAutomationError::UnknownTool,
    )?;

    proposal["steps"] = json!([{
        "id": "time",
        "type": "governed-tool",
        "tool_name": "get_current_datetime",
        "tool_contract_version": 2,
        "arguments": {},
        "depends_on": []
    }]);
    assert_proposal_error(
        &request,
        &registry,
        &proposal,
        WorkflowAutomationError::ToolVersionMismatch,
    )?;

    proposal["steps"][0]["tool_contract_version"] = json!(1);
    proposal["steps"][0]["arguments"] = json!({"unexpected": true});
    assert_proposal_error(
        &request,
        &registry,
        &proposal,
        WorkflowAutomationError::InvalidToolArguments,
    )?;

    proposal["steps"] = json!([{
        "id": "local-task",
        "type": "governed-tool",
        "tool_name": "create_local_task",
        "tool_contract_version": 1,
        "arguments": {"title": "Inert fixture proposal"},
        "depends_on": []
    }]);
    assert_proposal_error(
        &request,
        &registry,
        &proposal,
        WorkflowAutomationError::MissingApprovalCheckpoint,
    )?;

    proposal["steps"] = json!([
        {
            "id": "local-task",
            "type": "governed-tool",
            "tool_name": "create_local_task",
            "tool_contract_version": 1,
            "arguments": {"title": "Inert fixture proposal"},
            "depends_on": []
        },
        {
            "id": "checkpoint",
            "type": "approval-checkpoint",
            "subject_step_id": "local-task",
            "depends_on": ["local-task"]
        }
    ]);
    assert_proposal_error(
        &request,
        &registry,
        &proposal,
        WorkflowAutomationError::ToolStepsUnavailable,
    )?;

    proposal["steps"] = json!([{
        "id": "checkpoint",
        "type": "approval-checkpoint",
        "subject_step_id": "missing-tool",
        "depends_on": []
    }]);
    assert_proposal_error(
        &request,
        &registry,
        &proposal,
        WorkflowAutomationError::ApprovalDispatchUnavailable,
    )?;
    Ok(())
}

#[test]
fn strict_personal_synthesis_preserves_application_disposition_and_denies_authority_claims(
) -> Result<(), Box<dyn Error>> {
    let registry = AgentRegistry::built_in()?;
    for template_id in [
        WorkflowTemplateId::ResearchBriefV1,
        WorkflowTemplateId::DocumentToActionPlanV1,
    ] {
        let request = WorkflowTemplateCatalog::built_in().proposal_request(template_id)?;
        let id = proposal_id(&format!("proposal-synthesis-{}", template_id as u8))?;
        let validated = request.parse_proposal(
            id.clone(),
            &canonical_proposal_json(template_id)?,
            &registry,
        )?;
        let stage = WorkflowProposalStageOutcome::Validated(validated);
        let (disposition, unresolved): (&str, &[&str]) = if template_id.is_manually_dispatchable() {
            ("ready-for-manual-dispatch", &[])
        } else {
            (
                "proposal-only",
                &[WORKFLOW_AUTOMATION_DOCUMENT_PROPOSAL_ONLY_ISSUE],
            )
        };
        let raw = synthesis_json(template_id, &id, disposition, "complete", unresolved)?;
        let synthesis = request.parse_synthesis(&stage, &raw)?;
        assert_eq!(synthesis.template_id(), template_id);
        assert_eq!(synthesis.proposal_id(), &id);
        assert_eq!(
            synthesis.status(),
            WorkflowAutomationSynthesisStatus::Complete
        );
        assert_eq!(
            synthesis.disposition(),
            if template_id.is_manually_dispatchable() {
                WorkflowSynthesisDisposition::ReadyForManualDispatch
            } else {
                WorkflowSynthesisDisposition::ProposalOnly
            }
        );
        assert!(synthesis.fixture_based());
        assert!(synthesis.unwired());
        assert!(!synthesis.workflow_executed());
        assert!(!synthesis.tools_executed());
        assert!(!synthesis.approvals_requested());
        assert!(!synthesis.effects_performed());
        assert!(
            request.build_synthesis_input(&stage)?.len()
                <= ai_agent_assistant_lib::agent::workflow_automation::MAX_WORKFLOW_SYNTHESIS_INPUT_BYTES
        );

        let mut authority: Value = serde_json::from_str(&raw)?;
        authority["workflow_executed"] = json!(true);
        let authority_error = request
            .parse_synthesis(&stage, &serde_json::to_string(&authority)?)
            .err()
            .ok_or("authority-claim synthesis unexpectedly validated")?;
        assert_eq!(authority_error, WorkflowAutomationError::AuthorityClaim);

        let mut unknown_field: Value = serde_json::from_str(&raw)?;
        unknown_field["dispatch_authorized"] = json!(true);
        let unknown_error = request
            .parse_synthesis(&stage, &serde_json::to_string(&unknown_field)?)
            .err()
            .ok_or("unknown-field synthesis unexpectedly validated")?;
        assert_eq!(
            unknown_error,
            WorkflowAutomationError::InvalidStructuredProposal
        );

        let mut reasoning: Value = serde_json::from_str(&raw)?;
        reasoning["summary"] = json!(format!(
            "{WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE} Reasoning: hidden planner trace."
        ));
        let reasoning_error = request
            .parse_synthesis(&stage, &serde_json::to_string(&reasoning)?)
            .err()
            .ok_or("reasoning-label synthesis unexpectedly validated")?;
        assert_eq!(reasoning_error, WorkflowAutomationError::AuthorityClaim);

        let mut external_link: Value = serde_json::from_str(&raw)?;
        external_link["summary"] = json!(format!(
            "{WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE} See https://invalid.example."
        ));
        let link_error = request
            .parse_synthesis(&stage, &serde_json::to_string(&external_link)?)
            .err()
            .ok_or("URL-bearing synthesis unexpectedly validated")?;
        assert_eq!(link_error, WorkflowAutomationError::AuthorityClaim);

        let mut effect_claim: Value = serde_json::from_str(&raw)?;
        effect_claim["summary"] = json!(format!(
            "{WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE} Azure resources were provisioned."
        ));
        let effect_error = request
            .parse_synthesis(&stage, &serde_json::to_string(&effect_claim)?)
            .err()
            .ok_or("effect-claim synthesis unexpectedly validated")?;
        assert_eq!(effect_error, WorkflowAutomationError::TemplateMismatch);

        if template_id.is_manually_dispatchable() {
            let mut unresolved: Value = serde_json::from_str(&raw)?;
            unresolved["unresolved_issues"] = json!(["One issue remains."]);
            let unresolved_error = request
                .parse_synthesis(&stage, &serde_json::to_string(&unresolved)?)
                .err()
                .ok_or("dispatch-ready synthesis retained an unresolved issue")?;
            assert_eq!(unresolved_error, WorkflowAutomationError::TemplateMismatch);
        } else {
            let mut false_effect_issue: Value = serde_json::from_str(&raw)?;
            false_effect_issue["unresolved_issues"] = json!(["Azure resources were provisioned."]);
            let false_effect_error = request
                .parse_synthesis(&stage, &serde_json::to_string(&false_effect_issue)?)
                .err()
                .ok_or("proposal-only synthesis accepted model-authored issue text")?;
            assert_eq!(
                false_effect_error,
                WorkflowAutomationError::TemplateMismatch
            );
        }
    }

    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
    let id = proposal_id("proposal-rejected-synthesis")?;
    let rejected = WorkflowProposalStageOutcome::Rejected {
        proposal_id: id.clone(),
        code: WorkflowValidationFailureCode::Cycle,
    };
    let raw = synthesis_json(
        WorkflowTemplateId::ResearchBriefV1,
        &id,
        "unavailable",
        "partial",
        &[WORKFLOW_AUTOMATION_REJECTED_ISSUE],
    )?;
    let synthesis = request.parse_synthesis(&rejected, &raw)?;
    assert_eq!(
        synthesis.status(),
        WorkflowAutomationSynthesisStatus::Partial
    );
    assert_eq!(
        synthesis.disposition(),
        WorkflowSynthesisDisposition::Unavailable
    );
    Ok(())
}

#[test]
fn proposal_lifecycle_is_personal_to_workflow_automation_to_personal_and_content_redacted(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, proposal_id) =
        finish_successful_proposal(&mut orchestrator, WorkflowTemplateId::ResearchBriefV1)?;

    assert_eq!(
        orchestrator.selected_workflow(),
        Some(AgentWorkflowSelection::WorkflowAutomation)
    );
    assert_eq!(orchestrator.task_count(), 2);
    assert_eq!(orchestrator.run_count(), 3);
    assert_eq!(orchestrator.active_child_task(), None);
    assert_eq!(
        orchestrator.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    let result = orchestrator
        .workflow_automation_result()
        .ok_or("missing Workflow Automation result")?;
    assert_eq!(result.root_task_id().task_id(), root.task_id());
    assert_eq!(result.template_id(), WorkflowTemplateId::ResearchBriefV1);
    assert!(matches!(
        result.proposal(),
        WorkflowProposalStageOutcome::Validated(value)
            if value.disposition() == WorkflowValidationDisposition::ReadyForManualDispatch
                && value.proposal().id() == &proposal_id
    ));
    assert_eq!(
        result.synthesis().disposition(),
        WorkflowSynthesisDisposition::ReadyForManualDispatch
    );
    assert_eq!(
        result.synthesis().status(),
        WorkflowAutomationSynthesisStatus::Complete
    );
    assert!(!result.synthesis().workflow_executed());
    assert!(!result.synthesis().tools_executed());
    assert!(!result.synthesis().approvals_requested());
    assert!(!result.synthesis().effects_performed());

    assert!(matches!(
        orchestrator.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::ProposalCompleted {
                disposition: WorkflowValidationDisposition::ReadyForManualDispatch,
                ..
            },
            WorkflowAutomationWorkflowEvent::SynthesisStarted { .. },
            WorkflowAutomationWorkflowEvent::Completed { .. },
        ]
    ));
    let audit = orchestrator.workflow_automation_audit_records();
    assert_eq!(audit.len(), 4);
    assert!(audit.len() <= MAX_WORKFLOW_AUDIT_RECORDS);
    assert_eq!(audit[0].sequence(), 1);
    assert_eq!(
        audit[0].attribution().agent_id(),
        AgentId::WorkflowAutomation
    );
    assert_eq!(audit[0].attribution().depth(), 1);
    assert_eq!(audit[0].attribution().runtime_id(), RuntimeId::Native);
    assert_eq!(audit[0].stage(), WorkflowAutomationStage::Proposal);
    assert_eq!(audit[0].outcome(), WorkflowAutomationAuditOutcome::Started);
    assert_eq!(
        audit[2].attribution().agent_id(),
        AgentId::PersonalAssistant
    );
    assert_eq!(audit[2].attribution().depth(), 0);
    assert_eq!(audit[2].stage(), WorkflowAutomationStage::Synthesis);
    assert!(audit
        .windows(2)
        .all(|records| records[0].sequence() < records[1].sequence()));

    let starts = recorder.starts();
    assert_eq!(starts.len(), 3);
    assert!(starts[1]
        .selected_text
        .contains("deterministic-fixture-only"));
    assert!(starts[1]
        .selected_text
        .contains("execution_authorized=false"));
    assert!(starts[2].selected_text.contains("unwired=true"));
    assert!(starts[2].selected_text.contains("workflow_executed=false"));
    let debug = format!(
        "{orchestrator:?} {:?} {:?} {result:?}",
        orchestrator.workflow_automation_events(),
        audit,
    );
    for sensitive in [
        proposal_id.as_str(),
        "Compare two technical approaches",
        WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE,
    ] {
        assert!(!debug.contains(sensitive));
    }
    Ok(())
}

#[test]
fn a_through_d_issue_one_take_once_token_and_map_only_to_existing_sealed_selectors(
) -> Result<(), Box<dyn Error>> {
    let cases = [
        (
            WorkflowTemplateId::ResearchBriefV1,
            AgentWorkflowSelection::ResearchKnowledge,
            AgentId::Research,
        ),
        (
            WorkflowTemplateId::CodeQualityReviewV1,
            AgentWorkflowSelection::EngineeringQuality,
            AgentId::Coding,
        ),
        (
            WorkflowTemplateId::InfrastructureAssessmentV1,
            AgentWorkflowSelection::CloudInfrastructure,
            AgentId::CloudInfrastructure,
        ),
        (
            WorkflowTemplateId::SystemsIncidentAnalysisV1,
            AgentWorkflowSelection::SystemsOperations,
            AgentId::SystemsOperations,
        ),
    ];

    for (template_id, expected_selection, expected_agent) in cases {
        let mut planner = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
        let (_, proposal_id) = finish_successful_proposal(&mut planner, template_id)?;
        assert_eq!(
            planner.workflow_manual_dispatch_availability(),
            WorkflowManualDispatchAvailability::Available
        );
        let dispatch = planner.take_workflow_manual_dispatch()?;
        assert_eq!(dispatch.template_id(), template_id);
        let dispatch_debug = format!("{dispatch:?}");
        assert!(!dispatch_debug.contains(proposal_id.as_str()));
        assert_eq!(
            planner.workflow_manual_dispatch_availability(),
            WorkflowManualDispatchAvailability::Taken
        );
        assert!(matches!(
            planner.take_workflow_manual_dispatch(),
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchAlreadyTaken
            ))
        ));

        let mut destination = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
        let root = destination.start_root("Explicit trusted manual fixture dispatch")?;
        let acceptance = destination.start_manual_workflow(&root, dispatch)?;
        assert!(matches!(
            (&acceptance, template_id),
            (
                WorkflowManualDispatchAcceptance::ResearchBrief(_),
                WorkflowTemplateId::ResearchBriefV1
            ) | (
                WorkflowManualDispatchAcceptance::CodeQualityReview(_),
                WorkflowTemplateId::CodeQualityReviewV1
            ) | (
                WorkflowManualDispatchAcceptance::InfrastructureAssessment(_),
                WorkflowTemplateId::InfrastructureAssessmentV1
            ) | (
                WorkflowManualDispatchAcceptance::SystemsIncidentAnalysis(_),
                WorkflowTemplateId::SystemsIncidentAnalysisV1
            )
        ));
        assert_eq!(destination.selected_workflow(), Some(expected_selection));
        assert_eq!(
            destination.active_child_task().map(|task| task.agent_id()),
            Some(expected_agent)
        );
        assert_eq!(
            destination.manual_workflow_dispatch_status(),
            Some(WorkflowManualDispatchStatus::Accepted)
        );
        assert_eq!(
            destination.check_manual_workflow_deadline()?,
            Some(WorkflowManualDispatchStatus::Accepted)
        );
        assert!(matches!(
            destination.manual_workflow_dispatch_events(),
            [
                WorkflowManualDispatchEvent::Requested { .. },
                WorkflowManualDispatchEvent::Accepted { .. },
            ]
        ));
        let dispatch_audit = destination.manual_workflow_dispatch_audit_records();
        assert_eq!(dispatch_audit.len(), 2);
        assert_eq!(dispatch_audit[0].sequence(), 1);
        assert_eq!(dispatch_audit[0].template_id(), template_id);
        assert_eq!(
            dispatch_audit[0].attribution().root_task_id().task_id(),
            root.task_id()
        );
        assert_eq!(
            destination.cancel_task(root.task_id())?,
            AgentTaskCancellationOutcome::Cancelled
        );
        assert_eq!(
            destination.manual_workflow_result()?,
            Some(WorkflowManualDispatchStatus::Cancelled)
        );
        assert!(matches!(
            destination.manual_workflow_dispatch_events().last(),
            Some(WorkflowManualDispatchEvent::Cancelled { .. })
        ));
    }
    Ok(())
}

#[test]
fn document_template_and_rejected_tool_proposal_complete_truthfully_without_a_token(
) -> Result<(), Box<dyn Error>> {
    let mut document = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_, _) =
        finish_successful_proposal(&mut document, WorkflowTemplateId::DocumentToActionPlanV1)?;
    assert_eq!(
        document.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::ProposalOnly
    );
    assert!(matches!(
        document.take_workflow_manual_dispatch(),
        Err(AgentOrchestratorError::WorkflowAutomation(
            WorkflowAutomationError::ManualDispatchUnavailable
        ))
    ));
    assert!(matches!(
        document.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::ProposalCompleted {
                disposition: WorkflowValidationDisposition::ProposalOnly,
                ..
            },
            WorkflowAutomationWorkflowEvent::SynthesisStarted { .. },
            WorkflowAutomationWorkflowEvent::Completed { .. },
        ]
    ));
    let document_result = document
        .workflow_automation_result()
        .ok_or("missing document-to-action proposal result")?;
    assert_eq!(
        document_result.synthesis().disposition(),
        WorkflowSynthesisDisposition::ProposalOnly
    );
    assert!(!document_result.synthesis().unresolved_issues().is_empty());
    assert!(!document_result.synthesis().workflow_executed());

    let mut rejected = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) = start_proposal(&mut rejected, WorkflowTemplateId::ResearchBriefV1)?;
    let proposal_id = rejected
        .workflow_automation_audit_records()
        .first()
        .ok_or("missing rejected-proposal audit")?
        .proposal_id()
        .clone();
    let mut tool_attempt = canonical_proposal_value(WorkflowTemplateId::ResearchBriefV1)?;
    tool_attempt["steps"] = json!([
        {
            "id": "local-task",
            "type": "governed-tool",
            "tool_name": "create_local_task",
            "tool_contract_version": 1,
            "arguments": {"title": "Must remain an inert proposal"},
            "depends_on": []
        },
        {
            "id": "checkpoint",
            "type": "approval-checkpoint",
            "subject_step_id": "local-task",
            "depends_on": ["local-task"]
        }
    ]);
    complete_stage(
        &mut rejected,
        &planner,
        "recognized-but-unavailable-tool",
        &serde_json::to_string(&tool_attempt)?,
    )?;
    let synthesis = rejected.current_context(root.task_id())?;
    complete_stage(
        &mut rejected,
        &synthesis,
        "rejected-proposal-synthesis",
        &synthesis_json(
            WorkflowTemplateId::ResearchBriefV1,
            &proposal_id,
            "unavailable",
            "partial",
            &[WORKFLOW_AUTOMATION_REJECTED_ISSUE],
        )?,
    )?;
    assert_eq!(
        rejected.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    assert!(matches!(
        rejected.take_workflow_manual_dispatch(),
        Err(AgentOrchestratorError::WorkflowAutomation(
            WorkflowAutomationError::ManualDispatchUnavailable
        ))
    ));
    let rejected_result = rejected
        .workflow_automation_result()
        .ok_or("missing rejected proposal result")?;
    assert!(matches!(
        rejected_result.proposal(),
        WorkflowProposalStageOutcome::Rejected {
            code: WorkflowValidationFailureCode::ToolStepsUnavailable,
            ..
        }
    ));
    assert_eq!(
        rejected_result.synthesis().status(),
        WorkflowAutomationSynthesisStatus::Partial
    );
    assert_eq!(
        rejected_result.synthesis().disposition(),
        WorkflowSynthesisDisposition::Unavailable
    );
    assert!(matches!(
        rejected.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::PartialFailure {
                stage: WorkflowAutomationStage::Proposal,
                code: WorkflowAutomationPartialFailureCode::ProposalRejected(
                    WorkflowValidationFailureCode::ToolStepsUnavailable
                ),
            },
            WorkflowAutomationWorkflowEvent::SynthesisStarted { .. },
            WorkflowAutomationWorkflowEvent::Completed { .. },
        ]
    ));
    assert!(rejected.governance_audit_records().is_empty());
    Ok(())
}

#[test]
fn planner_failure_and_child_cancellation_allow_only_truthful_partial_synthesis(
) -> Result<(), Box<dyn Error>> {
    let mut failed_planner = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) =
        start_proposal(&mut failed_planner, WorkflowTemplateId::CodeQualityReviewV1)?;
    let proposal_id = failed_planner
        .workflow_automation_audit_records()
        .first()
        .ok_or("missing failed-planner audit")?
        .proposal_id()
        .clone();
    failed_planner.accept_runtime_event(
        planner.task_id(),
        started(&planner, "planner-runtime-failure")?,
    )?;
    assert!(matches!(
        failed_planner.accept_runtime_event(planner.task_id(), failed(&planner)?)?,
        RuntimeEventAcceptance::ResponseFailed { .. }
    ));
    let synthesis = failed_planner.current_context(root.task_id())?;
    complete_stage(
        &mut failed_planner,
        &synthesis,
        "failed-planner-partial-synthesis",
        &synthesis_json(
            WorkflowTemplateId::CodeQualityReviewV1,
            &proposal_id,
            "unavailable",
            "partial",
            &[WORKFLOW_AUTOMATION_FAILED_ISSUE],
        )?,
    )?;
    assert!(matches!(
        failed_planner.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::PartialFailure {
                stage: WorkflowAutomationStage::Proposal,
                code: WorkflowAutomationPartialFailureCode::RuntimeFailed,
            },
            WorkflowAutomationWorkflowEvent::SynthesisStarted { .. },
            WorkflowAutomationWorkflowEvent::Completed { .. },
        ]
    ));
    assert_eq!(
        failed_planner.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );

    let mut cancelled = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) = start_proposal(
        &mut cancelled,
        WorkflowTemplateId::InfrastructureAssessmentV1,
    )?;
    let proposal_id = cancelled
        .workflow_automation_audit_records()
        .first()
        .ok_or("missing cancelled-planner audit")?
        .proposal_id()
        .clone();
    assert_eq!(
        cancelled.cancel_task(planner.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    let synthesis = cancelled.current_context(root.task_id())?;
    complete_stage(
        &mut cancelled,
        &synthesis,
        "cancelled-planner-partial-synthesis",
        &synthesis_json(
            WorkflowTemplateId::InfrastructureAssessmentV1,
            &proposal_id,
            "unavailable",
            "partial",
            &[WORKFLOW_AUTOMATION_CANCELLED_ISSUE],
        )?,
    )?;
    assert!(matches!(
        cancelled.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::Cancelled {
                stage: WorkflowAutomationStage::Proposal,
            },
            WorkflowAutomationWorkflowEvent::SynthesisStarted { .. },
            WorkflowAutomationWorkflowEvent::Completed { .. },
        ]
    ));
    assert_eq!(
        cancelled.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    Ok(())
}

#[test]
fn root_cancellation_is_child_first_terminal_and_starts_no_successor() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, planner) = start_proposal(
        &mut orchestrator,
        WorkflowTemplateId::SystemsIncidentAnalysisV1,
    )?;

    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator
            .task(planner.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(orchestrator.active_child_task().is_none());
    assert!(orchestrator.workflow_automation_result().is_none());
    assert!(matches!(
        orchestrator.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::Cancelled {
                stage: WorkflowAutomationStage::Proposal,
            },
        ]
    ));
    assert_eq!(orchestrator.workflow_automation_audit_records().len(), 2);
    assert_eq!(recorder.starts().len(), 2);
    assert_eq!(recorder.cancellations().len(), 2);
    assert!(matches!(
        orchestrator.accept_runtime_event(planner.task_id(), completed(&planner)),
        Err(AgentOrchestratorError::NoActiveRun)
    ));
    assert_eq!(
        orchestrator.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    Ok(())
}

#[test]
fn invalid_or_failed_final_synthesis_never_creates_dispatch_authority() -> Result<(), Box<dyn Error>>
{
    let mut invalid = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) = start_proposal(&mut invalid, WorkflowTemplateId::ResearchBriefV1)?;
    complete_stage(
        &mut invalid,
        &planner,
        "proposal-before-invalid-synthesis",
        &canonical_proposal_json(WorkflowTemplateId::ResearchBriefV1)?,
    )?;
    let synthesis = invalid.current_context(root.task_id())?;
    complete_stage(
        &mut invalid,
        &synthesis,
        "invalid-authority-synthesis",
        r#"{"version":"v1","summary":"Fixture proposal falsely claimed execution.","template_id":"research-brief-v1","proposal_id":"forged","validation_disposition":"ready-for-manual-dispatch","status":"complete","unresolved_issues":[],"fixture_based":true,"unwired":true,"workflow_executed":true,"tools_executed":false,"approvals_requested":false,"effects_performed":false}"#,
    )?;
    assert_eq!(
        invalid.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(invalid.workflow_automation_result().is_none());
    assert_eq!(
        invalid.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    assert!(matches!(
        invalid.workflow_automation_events().last(),
        Some(WorkflowAutomationWorkflowEvent::Failed {
            stage: WorkflowAutomationStage::Synthesis,
            code: WorkflowAutomationPartialFailureCode::InvalidStructuredOutput,
        })
    ));

    let mut runtime_failure = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) = start_proposal(
        &mut runtime_failure,
        WorkflowTemplateId::CodeQualityReviewV1,
    )?;
    complete_stage(
        &mut runtime_failure,
        &planner,
        "proposal-before-synthesis-failure",
        &canonical_proposal_json(WorkflowTemplateId::CodeQualityReviewV1)?,
    )?;
    let synthesis = runtime_failure.current_context(root.task_id())?;
    runtime_failure.accept_runtime_event(
        root.task_id(),
        started(&synthesis, "synthesis-runtime-failure")?,
    )?;
    assert!(matches!(
        runtime_failure.accept_runtime_event(root.task_id(), failed(&synthesis)?)?,
        RuntimeEventAcceptance::ResponseFailed { .. }
    ));
    assert_eq!(
        runtime_failure
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(runtime_failure.workflow_automation_result().is_none());
    assert_eq!(
        runtime_failure.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    Ok(())
}

#[test]
fn generic_delegation_and_selector_reentry_cannot_execute_workflow_automation(
) -> Result<(), Box<dyn Error>> {
    let mut generic = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = generic.start_root("Inspect an application-owned workflow fixture")?;
    let proposal = DelegationProposal::new(
        AgentId::WorkflowAutomation,
        "Propose a workflow",
        None,
        "Return a bounded proposal",
    )?;
    assert!(matches!(
        generic.request_delegation(&root, proposal),
        Err(AgentOrchestratorError::RouteDenied {
            source_agent_id: AgentId::PersonalAssistant,
            target: AgentId::WorkflowAutomation,
        })
    ));
    assert_eq!(generic.task_count(), 1);
    assert_eq!(generic.run_count(), 1);
    assert_eq!(generic.selected_workflow(), None);

    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
    let accepted = generic.start_workflow_automation_proposal(&root, request)?;
    assert_eq!(accepted.context().agent_id(), AgentId::WorkflowAutomation);
    assert_eq!(
        generic.selected_workflow(),
        Some(AgentWorkflowSelection::WorkflowAutomation)
    );
    let repeated = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::CodeQualityReviewV1)?;
    assert!(matches!(
        generic.start_workflow_automation_proposal(&root, repeated),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::WorkflowAutomation,
        })
    ));
    assert_eq!(generic.governance_audit_records().len(), 1);
    assert_eq!(generic.task_count(), 2);
    Ok(())
}

#[test]
fn native_runtime_remains_default_text_only_for_planner_synthesis_and_selector(
) -> Result<(), Box<dyn Error>> {
    let registry = AgentRegistry::built_in()?;
    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
    let planner_input = request.build_planner_input()?;
    let planner_request = RuntimeTurnRequest::new(
        "workflow-native-planner-run",
        "workflow-native-planner-request",
        planner_input,
    )?;
    let runtime = NativeAgentRuntime;
    assert_eq!(runtime.describe().id(), RuntimeId::Native);
    assert!(runtime
        .describe()
        .capabilities()
        .supports(RuntimeCapability::StreamingText));
    assert!(!runtime
        .describe()
        .capabilities()
        .supports(RuntimeCapability::UntrustedToolProposals));
    let planner_run = runtime.start(planner_request)?;
    assert!(planner_run.request_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);

    let id = proposal_id("workflow-native-synthesis")?;
    let validated = request.parse_proposal(
        id,
        &canonical_proposal_json(WorkflowTemplateId::ResearchBriefV1)?,
        &registry,
    )?;
    let synthesis_input =
        request.build_synthesis_input(&WorkflowProposalStageOutcome::Validated(validated))?;
    let synthesis_request = RuntimeTurnRequest::new(
        "workflow-native-synthesis-run",
        "workflow-native-synthesis-request",
        synthesis_input,
    )?;
    let synthesis_run = runtime.start(synthesis_request)?;
    assert!(synthesis_run.request_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);

    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root(request.objective())?;
    let acceptance = orchestrator.start_workflow_automation_proposal(&root, request)?;
    assert_eq!(acceptance.context().runtime_id(), RuntimeId::Native);
    assert_eq!(acceptance.context().agent_id(), AgentId::WorkflowAutomation);
    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    Ok(())
}

#[test]
fn planner_start_failure_falls_back_once_and_chained_start_failure_closes_the_root(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(2));
    let mut fallback = AgentOrchestrator::new(runtime)?;
    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
    let root = fallback.start_root(request.objective())?;
    let acceptance = fallback.start_workflow_automation_proposal(&root, request)?;
    let synthesis = match acceptance {
        WorkflowAutomationWorkflowAcceptance::PersonalFallbackStarted { context } => context,
        WorkflowAutomationWorkflowAcceptance::ProposalStarted { .. } => {
            return Err("planner start failure did not start Personal fallback".into())
        }
    };
    let proposal_id = fallback
        .workflow_automation_audit_records()
        .first()
        .ok_or("missing planner-start-failure audit")?
        .proposal_id()
        .clone();
    assert_eq!(
        fallback.workflow_automation_continuation_failure(),
        Some(WorkflowAutomationContinuationFailure::ProposalStartFailed)
    );
    complete_stage(
        &mut fallback,
        &synthesis,
        "planner-start-fallback",
        &synthesis_json(
            WorkflowTemplateId::ResearchBriefV1,
            &proposal_id,
            "unavailable",
            "partial",
            &[WORKFLOW_AUTOMATION_FAILED_ISSUE],
        )?,
    )?;
    let result = fallback
        .workflow_automation_result()
        .ok_or("missing planner-start-failure result")?;
    assert!(matches!(
        result.proposal(),
        WorkflowProposalStageOutcome::Failed {
            code: AgentTaskFailureCode::RuntimeStartFailed,
            ..
        }
    ));
    assert_eq!(
        result.synthesis().status(),
        WorkflowAutomationSynthesisStatus::Partial
    );
    assert_eq!(
        fallback.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    assert!(matches!(
        fallback.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::PartialFailure {
                stage: WorkflowAutomationStage::Proposal,
                code: WorkflowAutomationPartialFailureCode::RuntimeStartFailed,
            },
            WorkflowAutomationWorkflowEvent::SynthesisStarted { .. },
            WorkflowAutomationWorkflowEvent::Completed { .. },
        ]
    ));
    assert_eq!(recorder.starts().len(), 3);
    assert_eq!(fallback.run_count(), 3);
    assert_eq!(fallback.task_count(), 2);

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailuresAt(2, 3));
    let mut chained = AgentOrchestrator::new(runtime)?;
    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::CodeQualityReviewV1)?;
    let root = chained.start_root(request.objective())?;
    assert!(chained
        .start_workflow_automation_proposal(&root, request)
        .is_err());
    assert_eq!(
        chained.workflow_automation_continuation_failure(),
        Some(WorkflowAutomationContinuationFailure::ProposalAndSynthesisStartFailed)
    );
    assert_eq!(
        chained.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(matches!(
        chained.task(root.task_id()).and_then(|task| task.outcome()),
        Some(AgentTaskOutcome::Failed(failure))
            if failure.code() == AgentTaskFailureCode::RuntimeStartFailed
    ));
    assert!(chained.active_child_task().is_none());
    assert!(chained.workflow_automation_result().is_none());
    assert_eq!(
        chained.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    assert!(matches!(
        chained.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::PartialFailure {
                stage: WorkflowAutomationStage::Proposal,
                code: WorkflowAutomationPartialFailureCode::RuntimeStartFailed,
            },
            WorkflowAutomationWorkflowEvent::Failed {
                stage: WorkflowAutomationStage::Synthesis,
                code: WorkflowAutomationPartialFailureCode::RuntimeStartFailed,
            },
        ]
    ));
    assert_eq!(recorder.starts().len(), 3);
    assert_eq!(chained.run_count(), 3);
    Ok(())
}

#[test]
fn rejected_planner_run_blocks_personal_fallback_until_explicit_cleanup(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) =
        MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchWithCancelFailureOnceAt(2));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = WorkflowTemplateCatalog::built_in()
        .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
    let root = orchestrator.start_root(request.objective())?;

    assert_eq!(
        orchestrator.start_workflow_automation_proposal(&root, request),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(recorder.starts().len(), 2);
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());
    assert!(orchestrator.current_context(root.task_id()).is_err());
    assert!(orchestrator.workflow_automation_result().is_none());

    orchestrator.retry_rejected_runtime_cleanup()?;
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn runtime_reported_cancellation_is_typed_and_never_issues_a_token() -> Result<(), Box<dyn Error>> {
    let mut planner_cancel = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) = start_proposal(
        &mut planner_cancel,
        WorkflowTemplateId::InfrastructureAssessmentV1,
    )?;
    let proposal_id = planner_cancel
        .workflow_automation_audit_records()
        .first()
        .ok_or("missing planner cancellation audit")?
        .proposal_id()
        .clone();
    planner_cancel.accept_runtime_event(
        planner.task_id(),
        started(&planner, "runtime-cancelled-planner")?,
    )?;
    assert!(matches!(
        planner_cancel.accept_runtime_event(planner.task_id(), runtime_cancelled(&planner)?)?,
        RuntimeEventAcceptance::ResponseFailed {
            failure
        } if failure.code() == RuntimeFailureCode::Cancelled
    ));
    let synthesis = planner_cancel.current_context(root.task_id())?;
    complete_stage(
        &mut planner_cancel,
        &synthesis,
        "runtime-cancelled-planner-synthesis",
        &synthesis_json(
            WorkflowTemplateId::InfrastructureAssessmentV1,
            &proposal_id,
            "unavailable",
            "partial",
            &[WORKFLOW_AUTOMATION_CANCELLED_ISSUE],
        )?,
    )?;
    let result = planner_cancel
        .workflow_automation_result()
        .ok_or("missing planner-cancelled result")?;
    assert!(matches!(
        result.proposal(),
        WorkflowProposalStageOutcome::Cancelled { .. }
    ));
    assert!(matches!(
        planner_cancel.workflow_automation_events(),
        [
            WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
            WorkflowAutomationWorkflowEvent::PartialFailure {
                stage: WorkflowAutomationStage::Proposal,
                code: WorkflowAutomationPartialFailureCode::Cancelled,
            },
            WorkflowAutomationWorkflowEvent::SynthesisStarted { .. },
            WorkflowAutomationWorkflowEvent::Completed { .. },
        ]
    ));
    assert_eq!(
        planner_cancel.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );

    let mut synthesis_cancel = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) = start_proposal(
        &mut synthesis_cancel,
        WorkflowTemplateId::SystemsIncidentAnalysisV1,
    )?;
    complete_stage(
        &mut synthesis_cancel,
        &planner,
        "proposal-before-runtime-cancelled-synthesis",
        &canonical_proposal_json(WorkflowTemplateId::SystemsIncidentAnalysisV1)?,
    )?;
    let synthesis = synthesis_cancel.current_context(root.task_id())?;
    synthesis_cancel.accept_runtime_event(
        root.task_id(),
        started(&synthesis, "runtime-cancelled-synthesis")?,
    )?;
    assert!(matches!(
        synthesis_cancel.accept_runtime_event(root.task_id(), runtime_cancelled(&synthesis)?)?,
        RuntimeEventAcceptance::ResponseFailed {
            failure
        } if failure.code() == RuntimeFailureCode::Cancelled
    ));
    assert_eq!(
        synthesis_cancel
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(matches!(
        synthesis_cancel
            .task(root.task_id())
            .and_then(|task| task.outcome()),
        Some(AgentTaskOutcome::Cancelled(_))
    ));
    assert!(synthesis_cancel.workflow_automation_result().is_none());
    assert_eq!(
        synthesis_cancel.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    assert!(matches!(
        synthesis_cancel.workflow_automation_events().last(),
        Some(WorkflowAutomationWorkflowEvent::Cancelled {
            stage: WorkflowAutomationStage::Synthesis,
        })
    ));
    Ok(())
}

#[test]
fn prior_selected_destination_consumes_token_without_mutating_existing_workflow(
) -> Result<(), Box<dyn Error>> {
    let mut planner = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    finish_successful_proposal(&mut planner, WorkflowTemplateId::ResearchBriefV1)?;
    let dispatch = planner.take_workflow_manual_dispatch()?;

    let mut destination = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = destination.start_root("Existing generic fixture workflow")?;
    let delegated = destination.request_delegation(
        &root,
        DelegationProposal::new(
            AgentId::Research,
            "Inspect one bounded fixture",
            None,
            "Return one fixture observation",
        )?,
    )?;
    complete_stage(
        &mut destination,
        delegated.child_context(),
        "existing-generic-child",
        "One bounded fixture observation.",
    )?;
    let live_root = destination.current_context(root.task_id())?;
    let selection_before = destination.selected_workflow();
    let tasks_before = destination.task_count();
    let runs_before = destination.run_count();
    let runtime_events_before = destination.runtime_event_count();
    let events_before = destination.events().to_vec();
    let root_status_before = destination.task(root.task_id()).map(|task| task.status());
    assert_eq!(
        selection_before,
        Some(AgentWorkflowSelection::GenericDelegation)
    );
    assert_eq!(destination.manual_workflow_dispatch_status(), None);

    assert_eq!(
        destination.start_manual_workflow(&live_root, dispatch),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::GenericDelegation,
        })
    );
    assert_eq!(destination.selected_workflow(), selection_before);
    assert_eq!(destination.task_count(), tasks_before);
    assert_eq!(destination.run_count(), runs_before);
    assert_eq!(destination.runtime_event_count(), runtime_events_before);
    assert_eq!(destination.events(), events_before);
    assert_eq!(
        destination.task(root.task_id()).map(|task| task.status()),
        root_status_before
    );
    assert_eq!(destination.manual_workflow_dispatch_status(), None);
    assert!(destination.manual_workflow_dispatch_events().is_empty());
    assert!(destination
        .manual_workflow_dispatch_audit_records()
        .is_empty());
    assert_eq!(
        planner.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::Taken
    );
    Ok(())
}

#[test]
fn manually_dispatched_research_updates_bridge_for_complete_and_partial_results(
) -> Result<(), Box<dyn Error>> {
    let mut complete_planner = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    finish_successful_proposal(&mut complete_planner, WorkflowTemplateId::ResearchBriefV1)?;
    let dispatch = complete_planner.take_workflow_manual_dispatch()?;
    let mut complete = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = complete.start_root("Manually run the sealed Research fixture")?;
    let acceptance = complete.start_manual_workflow(&root, dispatch)?;
    let research = match acceptance {
        WorkflowManualDispatchAcceptance::ResearchBrief(
            ResearchKnowledgeWorkflowAcceptance::ResearchStarted { context },
        ) => context,
        _ => return Err("manual Research dispatch selected the wrong workflow".into()),
    };
    complete_stage(
        &mut complete,
        &research,
        "manual-research-complete",
        RESEARCH_JSON,
    )?;
    let knowledge = active_child_context(&complete)?;
    complete_stage(
        &mut complete,
        &knowledge,
        "manual-knowledge-complete",
        KNOWLEDGE_JSON,
    )?;
    let synthesis = complete.current_context(root.task_id())?;
    complete_stage(
        &mut complete,
        &synthesis,
        "manual-research-synthesis-complete",
        RESEARCH_FINAL_SYNTHESIS,
    )?;
    assert_eq!(
        complete.manual_workflow_result()?,
        Some(WorkflowManualDispatchStatus::Completed)
    );
    assert_eq!(
        complete
            .research_knowledge_result()
            .ok_or("missing manually dispatched Research result")?
            .synthesis()
            .status(),
        FinalSynthesisStatus::Complete
    );
    assert!(matches!(
        complete.manual_workflow_dispatch_events(),
        [
            WorkflowManualDispatchEvent::Requested { .. },
            WorkflowManualDispatchEvent::Accepted { .. },
            WorkflowManualDispatchEvent::Completed {
                status: WorkflowManualDispatchStatus::Completed,
                ..
            },
        ]
    ));
    assert_eq!(complete.manual_workflow_dispatch_audit_records().len(), 3);

    let mut partial_planner = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    finish_successful_proposal(&mut partial_planner, WorkflowTemplateId::ResearchBriefV1)?;
    let dispatch = partial_planner.take_workflow_manual_dispatch()?;
    let mut partial = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = partial.start_root("Manually run a partial sealed Research fixture")?;
    let acceptance = partial.start_manual_workflow(&root, dispatch)?;
    let research = match acceptance {
        WorkflowManualDispatchAcceptance::ResearchBrief(
            ResearchKnowledgeWorkflowAcceptance::ResearchStarted { context },
        ) => context,
        _ => return Err("partial manual Research dispatch selected the wrong workflow".into()),
    };
    partial.accept_runtime_event(
        research.task_id(),
        started(&research, "manual-research-runtime-failure")?,
    )?;
    assert!(matches!(
        partial.accept_runtime_event(research.task_id(), failed(&research)?)?,
        RuntimeEventAcceptance::ResponseFailed { .. }
    ));
    let synthesis = partial.current_context(root.task_id())?;
    complete_stage(
        &mut partial,
        &synthesis,
        "manual-research-synthesis-partial",
        RESEARCH_PARTIAL_SYNTHESIS,
    )?;
    assert_eq!(
        partial.manual_workflow_result()?,
        Some(WorkflowManualDispatchStatus::Partial)
    );
    assert_eq!(
        partial
            .research_knowledge_result()
            .ok_or("missing partial manually dispatched Research result")?
            .synthesis()
            .status(),
        FinalSynthesisStatus::Partial
    );
    assert!(matches!(
        partial.manual_workflow_dispatch_events().last(),
        Some(WorkflowManualDispatchEvent::Completed {
            status: WorkflowManualDispatchStatus::Partial,
            ..
        })
    ));
    assert_eq!(partial.manual_workflow_dispatch_audit_records().len(), 3);
    Ok(())
}

#[test]
fn runtime_tool_proposal_from_workflow_automation_fails_closed_into_partial_synthesis(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, planner) =
        start_proposal(&mut orchestrator, WorkflowTemplateId::CodeQualityReviewV1)?;
    let proposal_id = orchestrator
        .workflow_automation_audit_records()
        .first()
        .ok_or("missing runtime-tool proposal audit")?
        .proposal_id()
        .clone();
    orchestrator.accept_runtime_event(
        planner.task_id(),
        started(&planner, "workflow-runtime-tool-attempt")?,
    )?;
    let tool_event = RuntimeEventEnvelope::for_identity(
        planner.runtime_run_identity(),
        1,
        UntrustedRuntimeEvent::ToolProposal {
            proposal: UntrustedRuntimeToolProposal::new(
                "workflow-runtime-tool-call",
                "create_local_task",
                1,
                r#"{"title":"Must not execute"}"#,
            )?,
        },
    );
    assert_eq!(
        orchestrator.accept_runtime_event(planner.task_id(), tool_event),
        Err(AgentOrchestratorError::ToolProposalUnsupported)
    );
    assert!(orchestrator.governance_audit_records().is_empty());
    assert!(orchestrator
        .pending_governance_approval(planner.task_id())?
        .is_none());
    let synthesis = orchestrator.current_context(root.task_id())?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "workflow-runtime-tool-partial-synthesis",
        &synthesis_json(
            WorkflowTemplateId::CodeQualityReviewV1,
            &proposal_id,
            "unavailable",
            "partial",
            &[WORKFLOW_AUTOMATION_FAILED_ISSUE],
        )?,
    )?;
    let result = orchestrator
        .workflow_automation_result()
        .ok_or("missing runtime-tool partial result")?;
    assert!(matches!(
        result.proposal(),
        WorkflowProposalStageOutcome::Failed {
            code: AgentTaskFailureCode::RuntimeOutputInvalid,
            ..
        }
    ));
    assert_eq!(
        result.synthesis().status(),
        WorkflowAutomationSynthesisStatus::Partial
    );
    assert_eq!(
        orchestrator.workflow_manual_dispatch_availability(),
        WorkflowManualDispatchAvailability::NotReady
    );
    assert!(orchestrator.governance_audit_records().is_empty());
    Ok(())
}
