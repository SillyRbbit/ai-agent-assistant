use serde_json::{json, Value};

use super::*;
use crate::agent::{
    engineering_quality::{
        AcceptanceCriterion, ApplicationEvidenceKind, ApplicationEvidenceStatus,
        ApplicationValidationEvidence, ApplicationValidationEvidenceCatalog, EngineeringId,
        FixtureRepositoryCatalog, FixtureRepositoryFile,
    },
    infrastructure_operations::{
        CloudScenarioId, InfrastructureOperationsFixtureCatalog, SystemsOperationsScenarioId,
    },
    research_knowledge::{WorkflowSource, WorkflowSourceCatalog},
};

pub(super) fn proposal_request(
    template_id: WorkflowTemplateId,
) -> WorkflowAutomationResult<WorkflowAutomationProposalRequest> {
    let template = canonical_definition(template_id)?;
    let catalog_serialized = definition_json(&template)?.to_string();
    if catalog_serialized.len() > MAX_WORKFLOW_CATALOG_BYTES {
        return Err(WorkflowAutomationError::CatalogConfiguration);
    }
    Ok(WorkflowAutomationProposalRequest {
        template,
        catalog_serialized,
    })
}

pub(super) fn build_planner_input(
    request: &WorkflowAutomationProposalRequest,
) -> WorkflowAutomationResult<String> {
    let input = format!(
        "workflow-automation-proposal-v1\nmode=deterministic-fixture-only\nmanual_trigger_only=true\nproposal_only=true\nexecution_authorized=false\ntools_executed=false\napprovals_requested=false\neffects_performed=false\nselected_template(application-owned):\n{}\nReturn exactly one strict WorkflowProposalV1 JSON object. Copy the selected template objective, ordered steps, dependencies, expected outputs, failure behavior, and limits exactly. Set fixture_based=true, proposal_only=true, manual_trigger_only=true, nested_workflow=false, self_modifying=false, execution_authorized=false, tools_executed=false, approvals_requested=false, and effects_performed=false. Do not include reasoning, scripts, commands, unknown fields, tools, approval checkpoints, or trusted identity.",
        request.catalog_serialized
    );
    if input.len() > MAX_WORKFLOW_PLANNER_INPUT_BYTES {
        return Err(WorkflowAutomationError::BoundExceeded);
    }
    Ok(input)
}

pub(super) fn canonical_definition(
    template_id: WorkflowTemplateId,
) -> WorkflowAutomationResult<WorkflowDefinition> {
    let (objective, steps) = match template_id {
        WorkflowTemplateId::ResearchBriefV1 => (
            "Compare two technical approaches and create a fixture-only structured decision brief",
            vec![
                agent_step("research", AgentId::Research, &[], "research-result-v1")?,
                agent_step(
                    "knowledge",
                    AgentId::KnowledgeDocument,
                    &["research"],
                    "knowledge-result-v1",
                )?,
                synthesis_step(
                    "synthesis",
                    &["knowledge"],
                    "research-knowledge-synthesis-v1",
                )?,
            ],
        ),
        WorkflowTemplateId::CodeQualityReviewV1 => (
            "Review a synthetic off-by-one fixture and produce a proposal-only code quality decision",
            vec![
                agent_step("coding", AgentId::Coding, &[], "change-proposal-v1")?,
                agent_step(
                    "qa",
                    AgentId::QaValidation,
                    &["coding"],
                    "validation-report-v1",
                )?,
                agent_step(
                    "security",
                    AgentId::SecurityRisk,
                    &["qa"],
                    "risk-assessment-v1",
                )?,
                synthesis_step(
                    "synthesis",
                    &["security"],
                    "engineering-review-synthesis-v1",
                )?,
            ],
        ),
        WorkflowTemplateId::InfrastructureAssessmentV1 => (
            "Review the supplied synthetic Terraform and architecture fixtures and produce an inert decision brief.",
            vec![
                agent_step(
                    "cloud",
                    AgentId::CloudInfrastructure,
                    &[],
                    "infrastructure-assessment-v1",
                )?,
                agent_step(
                    "qa",
                    AgentId::QaValidation,
                    &["cloud"],
                    "validation-report-v1",
                )?,
                agent_step(
                    "security",
                    AgentId::SecurityRisk,
                    &["qa"],
                    "risk-assessment-v1",
                )?,
                synthesis_step(
                    "synthesis",
                    &["security"],
                    "cloud-infrastructure-synthesis-v1",
                )?,
            ],
        ),
        WorkflowTemplateId::SystemsIncidentAnalysisV1 => (
            "Analyze the supplied sanitized service and log fixtures and propose inert diagnostics and recovery steps.",
            vec![
                agent_step(
                    "systems",
                    AgentId::SystemsOperations,
                    &[],
                    "operational-assessment-v1",
                )?,
                agent_step(
                    "qa",
                    AgentId::QaValidation,
                    &["systems"],
                    "validation-report-v1",
                )?,
                agent_step(
                    "security",
                    AgentId::SecurityRisk,
                    &["qa"],
                    "risk-assessment-v1",
                )?,
                synthesis_step(
                    "synthesis",
                    &["security"],
                    "systems-operations-synthesis-v1",
                )?,
            ],
        ),
        WorkflowTemplateId::DocumentToActionPlanV1 => (
            "Organize one synthetic document fixture into a proposal-only action plan without reading or changing a file",
            vec![
                agent_step(
                    "knowledge",
                    AgentId::KnowledgeDocument,
                    &[],
                    "document-action-input-v1",
                )?,
                agent_step(
                    "workflow-proposal",
                    AgentId::WorkflowAutomation,
                    &["knowledge"],
                    "workflow-proposal-v1",
                )?,
                synthesis_step(
                    "synthesis",
                    &["workflow-proposal"],
                    "document-action-plan-v1",
                )?,
            ],
        ),
    };
    validate_text(
        objective,
        MAX_WORKFLOW_OBJECTIVE_CHARACTERS,
        MAX_WORKFLOW_OBJECTIVE_BYTES,
        false,
    )?;
    Ok(WorkflowDefinition {
        version: WORKFLOW_AUTOMATION_CONTRACT_VERSION,
        template_id,
        objective: objective.to_owned(),
        steps,
        limits: WorkflowLimit::canonical(),
        failure_behavior: WorkflowFailureBehavior::PartialSynthesis,
    })
}

fn agent_step(
    id: &str,
    agent_id: AgentId,
    dependencies: &[&str],
    expected_output: &str,
) -> WorkflowAutomationResult<WorkflowStep> {
    validate_text(
        expected_output,
        MAX_WORKFLOW_EXPECTED_OUTPUT_CHARACTERS,
        MAX_WORKFLOW_EXPECTED_OUTPUT_BYTES,
        false,
    )?;
    Ok(WorkflowStep::AgentTask(WorkflowAgentTaskStep {
        id: WorkflowStepId::new(id)?,
        agent_id,
        dependencies: dependency_ids(dependencies)?,
        expected_output: expected_output.to_owned(),
    }))
}

fn synthesis_step(
    id: &str,
    dependencies: &[&str],
    expected_output: &str,
) -> WorkflowAutomationResult<WorkflowStep> {
    validate_text(
        expected_output,
        MAX_WORKFLOW_EXPECTED_OUTPUT_CHARACTERS,
        MAX_WORKFLOW_EXPECTED_OUTPUT_BYTES,
        false,
    )?;
    Ok(WorkflowStep::Synthesis(WorkflowSynthesisStep {
        id: WorkflowStepId::new(id)?,
        agent_id: AgentId::PersonalAssistant,
        dependencies: dependency_ids(dependencies)?,
        expected_output: expected_output.to_owned(),
    }))
}

fn dependency_ids(values: &[&str]) -> WorkflowAutomationResult<Vec<WorkflowStepId>> {
    values
        .iter()
        .map(|value| WorkflowStepId::new(*value))
        .collect()
}

pub(super) fn definition_json(definition: &WorkflowDefinition) -> WorkflowAutomationResult<Value> {
    let steps = definition
        .steps
        .iter()
        .map(step_json)
        .collect::<WorkflowAutomationResult<Vec<_>>>()?;
    Ok(json!({
        "version": "v1",
        "template_id": definition.template_id,
        "objective": definition.objective,
        "steps": steps,
        "limits": definition.limits,
        "failure_behavior": definition.failure_behavior,
        "fixture_based": true,
        "proposal_only": true,
        "manual_trigger_only": true,
        "nested_workflow": false,
        "self_modifying": false,
        "execution_authorized": false,
        "tools_executed": false,
        "approvals_requested": false,
        "effects_performed": false,
    }))
}

fn step_json(step: &WorkflowStep) -> WorkflowAutomationResult<Value> {
    let dependencies: Vec<&str> = step
        .dependencies()
        .iter()
        .map(WorkflowStepId::as_str)
        .collect();
    Ok(match step {
        WorkflowStep::AgentTask(value) => json!({
            "id": value.id.as_str(),
            "type": "agent-task",
            "agent_id": value.agent_id.as_str(),
            "depends_on": dependencies,
            "expected_output": value.expected_output,
        }),
        WorkflowStep::Synthesis(value) => json!({
            "id": value.id.as_str(),
            "type": "synthesis",
            "agent_id": value.agent_id.as_str(),
            "depends_on": dependencies,
            "expected_output": value.expected_output,
        }),
        WorkflowStep::GovernedTool(value) => {
            let arguments: Value = serde_json::from_str(&value.arguments_json)
                .map_err(|_| WorkflowAutomationError::SerializationFailed)?;
            json!({
                "id": value.id.as_str(),
                "type": "governed-tool",
                "tool_name": value.tool_name,
                "tool_contract_version": value.tool_contract_version,
                "arguments": arguments,
                "depends_on": dependencies,
            })
        }
        WorkflowStep::ApprovalCheckpoint(value) => json!({
            "id": value.id.as_str(),
            "type": "approval-checkpoint",
            "subject_step_id": value.subject_step_id.as_str(),
            "depends_on": dependencies,
        }),
    })
}

pub(super) fn research_request(
    template_id: WorkflowTemplateId,
) -> WorkflowAutomationResult<ResearchKnowledgeWorkflowRequest> {
    require_template(template_id, WorkflowTemplateId::ResearchBriefV1)?;
    Ok(ResearchKnowledgeWorkflowRequest::new(
        canonical_definition(template_id)?.objective,
        WorkflowSourceCatalog::new(vec![
            WorkflowSource::deterministic_fixture(
                "approach-a",
                "Approach A fixture",
                "Approach A has lower operational complexity and lower fixture cost.",
            )?,
            WorkflowSource::deterministic_fixture(
                "approach-b",
                "Approach B fixture",
                "Approach B has higher fixture scale potential and greater operational complexity.",
            )?,
        ])?,
    )?)
}

pub(super) fn engineering_request(
    template_id: WorkflowTemplateId,
) -> WorkflowAutomationResult<EngineeringQualityWorkflowRequest> {
    require_template(template_id, WorkflowTemplateId::CodeQualityReviewV1)?;
    let source_id = EngineeringId::new("source-fixture")?;
    let test_id = EngineeringId::new("test-fixture")?;
    let criterion_id = EngineeringId::new("criterion-answer")?;
    Ok(EngineeringQualityWorkflowRequest::new(
        canonical_definition(template_id)?.objective,
        FixtureRepositoryCatalog::new(vec![
            FixtureRepositoryFile::new(
                source_id.as_str(),
                "src/lib.rs",
                "fn answer() -> i32 { 41 }",
            )?,
            FixtureRepositoryFile::new(
                test_id.as_str(),
                "tests/answer.rs",
                "assert_eq!(answer(), 42);",
            )?,
        ])?,
        vec![AcceptanceCriterion::new(
            criterion_id.as_str(),
            "The proposal corrects the fixture answer to 42 without applying a mutation.",
        )?],
        ApplicationValidationEvidenceCatalog::new(vec![ApplicationValidationEvidence::new(
            "evidence-answer-mismatch",
            ApplicationEvidenceKind::FixtureObservation,
            ApplicationEvidenceStatus::ObservedFixture,
            "The synthetic source and test fixtures demonstrate an answer mismatch.",
            vec![criterion_id],
            vec![source_id, test_id],
        )?])?,
    )?)
}

pub(super) fn cloud_request(
    template_id: WorkflowTemplateId,
) -> WorkflowAutomationResult<CloudInfrastructureWorkflowRequest> {
    require_template(template_id, WorkflowTemplateId::InfrastructureAssessmentV1)?;
    Ok(InfrastructureOperationsFixtureCatalog::built_in()
        .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?)
}

pub(super) fn systems_request(
    template_id: WorkflowTemplateId,
) -> WorkflowAutomationResult<SystemsOperationsWorkflowRequest> {
    require_template(template_id, WorkflowTemplateId::SystemsIncidentAnalysisV1)?;
    Ok(InfrastructureOperationsFixtureCatalog::built_in()
        .systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?)
}

fn require_template(
    actual: WorkflowTemplateId,
    expected: WorkflowTemplateId,
) -> WorkflowAutomationResult<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(WorkflowAutomationError::ManualDispatchUnavailable)
    }
}

pub(super) fn validate_text(
    value: &str,
    maximum_characters: usize,
    maximum_bytes: usize,
    allow_newlines: bool,
) -> WorkflowAutomationResult<()> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().count() > maximum_characters
        || value.len() > maximum_bytes
        || value.chars().any(|character| {
            character.is_control() && !(allow_newlines && matches!(character, '\n' | '\t'))
        })
    {
        Err(WorkflowAutomationError::BoundExceeded)
    } else {
        Ok(())
    }
}
