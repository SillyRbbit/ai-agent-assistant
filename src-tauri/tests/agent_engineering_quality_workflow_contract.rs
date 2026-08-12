use std::{error::Error, fs};

use ai_agent_assistant_lib::agent::definition::AgentId;
use ai_agent_assistant_lib::agent::engineering_quality::{
    AcceptanceCriterion, ApplicationEvidenceKind, ApplicationEvidenceStatus,
    ApplicationValidationEvidence, ApplicationValidationEvidenceCatalog, ChangeProposal,
    ChangeProposalQuality, CodingStageOutcome, DependencyEvidenceStatus, EngineeringCapability,
    EngineeringCapabilityAuditDisposition, EngineeringCapabilityDisposition,
    EngineeringContinuationFailure, EngineeringId, EngineeringPartialFailureCode,
    EngineeringQualityAuditOutcome, EngineeringQualityError, EngineeringQualityStage,
    EngineeringQualityWorkflowEvent, EngineeringQualityWorkflowRequest, EngineeringReviewStatus,
    FixtureRepositoryCatalog, FixtureRepositoryFile, QaStageOutcome, RiskAssessment,
    SecurityStageOutcome, ValidationConclusion, ValidationReport, MAX_CHANGE_PROPOSAL_BYTES,
    MAX_ENGINEERING_AUDIT_RECORDS, MAX_ENGINEERING_CRITERIA_CATALOG_BYTES,
    MAX_ENGINEERING_EVIDENCE_CATALOG_BYTES, MAX_ENGINEERING_FIXTURE_CATALOG_BYTES,
    MAX_ENGINEERING_OBJECTIVE_BYTES, MAX_ENGINEERING_STAGE_INPUT_BYTES,
    MAX_ENGINEERING_WORKFLOW_EVENTS, MAX_RISK_ASSESSMENT_BYTES, MAX_VALIDATION_REPORT_BYTES,
};
use ai_agent_assistant_lib::agent::gateway_protocol::{
    MAX_GATEWAY_REQUEST_BYTES, MAX_OPAQUE_ID_BYTES,
};
use ai_agent_assistant_lib::agent::governance::{
    AgentApprovalAuditDisposition, AgentExecutionDisposition, AgentGovernanceError,
    AgentPolicyReason, AgentToolGovernanceOutcome, AgentToolProposal,
};
use ai_agent_assistant_lib::agent::native_runtime::NativeAgentRuntime;
use ai_agent_assistant_lib::agent::orchestrator::{
    AgentOrchestrator, AgentOrchestratorError, AgentWorkflowSelection, DelegationProposal,
    EngineeringQualityWorkflowAcceptance, MAX_ENGINEERING_QUALITY_CHILDREN_PER_ROOT,
    MAX_ENGINEERING_QUALITY_EVENTS_PER_RUN, MAX_ENGINEERING_QUALITY_RUNTIME_RUNS_PER_ROOT,
    MAX_ENGINEERING_QUALITY_TASKS_PER_ROOT, MAX_RUNTIME_EVENTS_PER_ROOT,
};
use ai_agent_assistant_lib::agent::research_knowledge::{
    ResearchKnowledgeWorkflowRequest, WorkflowSource, WorkflowSourceCatalog,
};
use ai_agent_assistant_lib::agent::runtime::{
    AgentRuntime, RuntimeBoundaryStage, RuntimeCapability, RuntimeError, RuntimeEventAcceptance,
    RuntimeEventEnvelope, RuntimeFailure, RuntimeFailureCode, RuntimeId, RuntimeOutputText,
    RuntimeResponseId, RuntimeRunStatus, RuntimeTurnRequest, UntrustedRuntimeEvent,
};
use ai_agent_assistant_lib::agent::task::{
    AgentExecutionContext, AgentTaskCancellationOutcome, AgentTaskId, AgentTaskStatus,
};
use ai_agent_assistant_lib::policy::types::PolicyOutcome;
use ai_agent_assistant_lib::{
    audit::governance::{AgentGovernanceRecord, AgentToolGovernanceLifecycleState},
    documents::{ApprovedDocumentSource, DocumentOperation},
};
use tempfile::tempdir;

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

const OBJECTIVE: &str = "Correct the fixture-only answer and prepare a governed review";
const SECRET_SENTINEL: &str = "fixture-secret-sentinel";

const CHANGE_PROPOSAL: &str = r#"{"version":"v1","objective_summary":"Correct the fixture-only answer proposal","findings":[{"id":"off-by-one","statement":"The fixture returns 41 instead of the required 42.","references":[{"namespace":"fixture-file","id":"src-main"},{"namespace":"application-evidence","id":"obs-bug"}],"confidence":"high"}],"affected_file_ids":["src-main"],"patch_operations":[{"file_id":"src-main","proposal":"Replace the literal 41 with 42 in the proposed patch."}],"risks":["The proposal may affect callers that expect 41."],"validation_steps":[{"id":"review-fixture","text":"Compare the proposed literal with the fixture criterion."}],"rollback":"Discard the proposal and retain the immutable fixture.","capability_requests":["fixture-inspect","fixture-text-search","patch-proposal","validation-plan"],"fixture_based":true,"proposal_only":true}"#;

const VALIDATION_REPORT: &str = r#"{"version":"v1","coverage":[{"criterion_id":"returns-42","disposition":"demonstrated","references":[{"namespace":"application-evidence","id":"obs-bug"}]}],"findings":[{"id":"coverage-observation","statement":"Fixture evidence demonstrates the observed mismatch.","references":[{"namespace":"application-evidence","id":"obs-bug"}]}],"proposed_checks":[{"id":"run-unit-check","text":"Run the proposed unit check after separate execution authorization.","references":[{"namespace":"proposal-validation-step","id":"review-fixture"}],"status":"not-run"}],"gaps":[],"conclusion":"adequate","advisory_only":true,"approval_authority":false,"evidence_executed":false}"#;

const RISK_ASSESSMENT: &str = r#"{"version":"v1","findings":[{"id":"boundary-review","category":"authorization-boundary","severity":"medium","confidence":"high","statement":"The proposal must remain separate from repository mutation authority.","basis":"evidence-bound","references":[{"namespace":"proposal-finding","id":"off-by-one"}]},{"id":"caller-risk","category":"general-change-risk","severity":"low","confidence":"medium","statement":"Callers may rely on the fixture's prior value.","basis":"hypothesis","references":[]}],"unresolved_risks":["No executable validation evidence is available."],"follow_ups":["Seek separate authorization before any mutation or command execution."],"advisory_only":true,"authorization_granted":false,"remediation_executed":false}"#;

const SYNTHESIS: &str = r#"{"version":"v1","summary":"Fixture-only proposal reviewed by application-owned stages.","affected_file_ids":["src-main"],"unresolved_issues":[],"fixture_based":true,"proposal_only":true,"changes_applied":false,"tests_executed":false,"status":"complete"}"#;
const PARTIAL_SYNTHESIS_WITH_PROPOSAL: &str = r#"{"version":"v1","summary":"Fixture-only proposal review is partial because a specialist result is unavailable.","affected_file_ids":["src-main"],"unresolved_issues":["Review the unavailable specialist stage before acting."],"fixture_based":true,"proposal_only":true,"changes_applied":false,"tests_executed":false,"status":"partial"}"#;
const PARTIAL_SYNTHESIS_NO_PROPOSAL: &str = r#"{"version":"v1","summary":"Fixture-only proposal review is partial because Coding is unavailable.","affected_file_ids":[],"unresolved_issues":["No validated change proposal is available."],"fixture_based":true,"proposal_only":true,"changes_applied":false,"tests_executed":false,"status":"partial"}"#;

const UNREGISTERED_REPOSITORY_TOOLS: [&str; 12] = [
    "write_file",
    "delete_file",
    "install_dependency",
    "run_package_manager",
    "run_tests",
    "run_formatter",
    "git_commit",
    "git_push",
    "delete_branch",
    "run_shell",
    "read_credential",
    "network_request",
];

fn id(value: &str) -> Result<EngineeringId, Box<dyn Error>> {
    Ok(EngineeringId::new(value)?)
}

fn fixture_request() -> Result<EngineeringQualityWorkflowRequest, Box<dyn Error>> {
    let fixtures = FixtureRepositoryCatalog::new(vec![
        FixtureRepositoryFile::new(
            "src-main",
            "src/main.rs",
            format!("fn answer() -> i32 {{ 41 }} // {SECRET_SENTINEL}"),
        )?,
        FixtureRepositoryFile::new(
            "dependency-lock",
            "Cargo.lock",
            "package = fixture-crate\nversion = 1.0.0",
        )?,
    ])?;
    let criteria = vec![AcceptanceCriterion::new(
        "returns-42",
        "The proposed answer returns 42 without applying a repository mutation.",
    )?];
    let evidence = ApplicationValidationEvidenceCatalog::new(vec![
        ApplicationValidationEvidence::new(
            "obs-bug",
            ApplicationEvidenceKind::FixtureObservation,
            ApplicationEvidenceStatus::ObservedFixture,
            "The supplied fixture contains the literal 41.",
            vec![id("returns-42")?],
            vec![id("src-main")?],
        )?,
        ApplicationValidationEvidence::new(
            "obs-dependency",
            ApplicationEvidenceKind::DependencyObservation,
            ApplicationEvidenceStatus::ObservedFixture,
            "The supplied lock fixture identifies one synthetic dependency.",
            vec![id("returns-42")?],
            vec![id("dependency-lock")?],
        )?,
        ApplicationValidationEvidence::new(
            "proposed-check",
            ApplicationEvidenceKind::ProposedCheck,
            ApplicationEvidenceStatus::NotRun,
            "A unit check is proposed but was not run.",
            vec![id("returns-42")?],
            vec![],
        )?,
    ])?;
    Ok(EngineeringQualityWorkflowRequest::new(
        OBJECTIVE, fixtures, criteria, evidence,
    )?)
}

fn research_workflow_request() -> Result<ResearchKnowledgeWorkflowRequest, Box<dyn Error>> {
    Ok(ResearchKnowledgeWorkflowRequest::new(
        "Compare two bounded fixture approaches",
        WorkflowSourceCatalog::new(vec![
            WorkflowSource::deterministic_fixture(
                "approach-a",
                "Approach A fixture",
                "Approach A is simpler.",
            )?,
            WorkflowSource::deterministic_fixture(
                "approach-b",
                "Approach B fixture",
                "Approach B has a different bounded trade-off.",
            )?,
        ])?,
    )?)
}

fn research_delegation() -> Result<DelegationProposal, Box<dyn Error>> {
    Ok(DelegationProposal::new(
        AgentId::Research,
        "Review the bounded fixture",
        None,
        "Return one attributed summary",
    )?)
}

fn task_id(value: &str) -> Result<AgentTaskId, Box<dyn Error>> {
    Ok(AgentTaskId::new(value)?)
}

fn proposal(request: &EngineeringQualityWorkflowRequest) -> Result<ChangeProposal, Box<dyn Error>> {
    Ok(request.parse_change_proposal(task_id("coding-task")?, CHANGE_PROPOSAL)?)
}

fn validation(
    request: &EngineeringQualityWorkflowRequest,
    proposal: &ChangeProposal,
) -> Result<ValidationReport, Box<dyn Error>> {
    Ok(request.parse_validation_report(task_id("qa-task")?, proposal, VALIDATION_REPORT)?)
}

fn risk(
    request: &EngineeringQualityWorkflowRequest,
    proposal: &ChangeProposal,
    qa: &QaStageOutcome,
) -> Result<RiskAssessment, Box<dyn Error>> {
    Ok(request.parse_risk_assessment(task_id("security-task")?, proposal, qa, RISK_ASSESSMENT)?)
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
    sequence: u32,
    text: &str,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        sequence,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new(text)?,
        },
    ))
}

fn completed(context: &AgentExecutionContext, sequence: u32) -> RuntimeEventEnvelope {
    RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        sequence,
        UntrustedRuntimeEvent::ResponseCompleted,
    )
}

fn failed(
    context: &AgentExecutionContext,
    sequence: u32,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        sequence,
        UntrustedRuntimeEvent::ResponseFailed {
            failure: RuntimeFailure::new(RuntimeFailureCode::ProviderUnavailable, true, None)?,
        },
    ))
}

fn runtime_cancelled(
    context: &AgentExecutionContext,
    sequence: u32,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        sequence,
        UntrustedRuntimeEvent::ResponseFailed {
            failure: RuntimeFailure::new(RuntimeFailureCode::Cancelled, false, None)?,
        },
    ))
}

fn complete_stage(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    response_id: &str,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    let task_id = context.task_id().clone();
    orchestrator.accept_runtime_event(&task_id, started(context, response_id)?)?;
    orchestrator.accept_runtime_event(&task_id, delta(context, 1, output)?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(&task_id, completed(context, 2))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    Ok(())
}

fn complete_stage_at_event_cap(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    response_id: &str,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    assert!(output.is_ascii());
    let task_id = context.task_id().clone();
    orchestrator.accept_runtime_event(&task_id, started(context, response_id)?)?;
    for index in 0..6_usize {
        let start = output.len() * index / 6;
        let end = output.len() * (index + 1) / 6;
        orchestrator.accept_runtime_event(
            &task_id,
            delta(context, u32::try_from(index + 1)?, &output[start..end])?,
        )?;
    }
    assert_eq!(
        orchestrator.accept_runtime_event(&task_id, completed(context, 7))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    Ok(())
}

fn start_workflow(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let root = orchestrator.start_root(OBJECTIVE)?;
    let acceptance = orchestrator.start_engineering_quality_workflow(&root, fixture_request()?)?;
    let coding = match acceptance {
        EngineeringQualityWorkflowAcceptance::CodingStarted { context } => context,
        EngineeringQualityWorkflowAcceptance::PersonalFallbackStarted { .. } => {
            return Err("Coding unexpectedly failed to start".into());
        }
    };
    Ok((root, coding))
}

fn active_child_context(
    orchestrator: &AgentOrchestrator<MockAgentRuntime>,
) -> Result<AgentExecutionContext, Box<dyn Error>> {
    let child = orchestrator
        .active_child_task()
        .ok_or("missing active specialist")?;
    Ok(orchestrator.current_context(child.id())?)
}

fn engineering_task_id(
    orchestrator: &AgentOrchestrator<MockAgentRuntime>,
    agent_id: AgentId,
) -> Result<AgentTaskId, Box<dyn Error>> {
    Ok(orchestrator
        .engineering_quality_audit_records()
        .iter()
        .find(|record| record.attribution().agent_id() == agent_id)
        .ok_or("missing engineering task attribution")?
        .attribution()
        .task_id()
        .clone())
}

fn observable_workflow_state(
    orchestrator: &AgentOrchestrator<MockAgentRuntime>,
) -> (
    usize,
    u8,
    usize,
    usize,
    usize,
    usize,
    Option<AgentWorkflowSelection>,
) {
    (
        orchestrator.task_count(),
        orchestrator.run_count(),
        orchestrator.events().len(),
        orchestrator.runtime_event_count(),
        orchestrator.engineering_quality_events().len(),
        orchestrator.research_knowledge_events().len(),
        orchestrator.selected_workflow(),
    )
}

fn assert_specialist_governance_denied(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    let audit_start = orchestrator.governance_audit_records().len();
    let inert_state = (
        orchestrator.task_count(),
        orchestrator.run_count(),
        orchestrator.events().len(),
        orchestrator.runtime_event_count(),
        orchestrator.engineering_quality_events().len(),
        orchestrator.engineering_quality_audit_records().len(),
    );
    let expected = AgentToolGovernanceOutcome::Final {
        policy_outcome: PolicyOutcome::Deny,
        policy_reason: AgentPolicyReason::ProfileNotEligible,
        approval: AgentApprovalAuditDisposition::NotRequired,
        execution: AgentExecutionDisposition::NotAttempted,
    };
    assert_eq!(
        orchestrator.govern_tool_proposal(
            context,
            AgentToolProposal::new(format!("{label}-datetime"), "get_current_datetime", 1, "{}",)?,
        )?,
        expected
    );
    assert_eq!(
        orchestrator.govern_tool_proposal(
            context,
            AgentToolProposal::new(
                format!("{label}-local-task"),
                "create_local_task",
                1,
                r#"{"title":"Review the bounded fixture"}"#,
            )?,
        )?,
        expected
    );
    let audit = orchestrator.governance_audit_records();
    assert_eq!(audit.len(), audit_start + 2);
    assert!(audit[audit_start..].iter().all(|record| matches!(
        record,
        AgentGovernanceRecord::Tool(record)
            if record.attribution().agent_id() == context.agent_id()
                && record.attribution().task_id() == context.task_id()
                && record.lifecycle() == AgentToolGovernanceLifecycleState::PolicyEvaluated
                && record.execution() == AgentExecutionDisposition::NotAttempted
    )));
    assert_eq!(
        (
            orchestrator.task_count(),
            orchestrator.run_count(),
            orchestrator.events().len(),
            orchestrator.runtime_event_count(),
            orchestrator.engineering_quality_events().len(),
            orchestrator.engineering_quality_audit_records().len(),
        ),
        inert_state
    );
    assert!(orchestrator
        .pending_governance_approval(context.task_id())?
        .is_none());
    Ok(())
}

fn assert_unregistered_repository_tools_rejected(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    let inert_state = (
        orchestrator.task_count(),
        orchestrator.run_count(),
        orchestrator.events().len(),
        orchestrator.runtime_event_count(),
        orchestrator.engineering_quality_events().len(),
        orchestrator.engineering_quality_audit_records().len(),
    );
    for (index, absent_tool) in UNREGISTERED_REPOSITORY_TOOLS.into_iter().enumerate() {
        let audit_index = orchestrator.governance_audit_records().len();
        assert!(matches!(
            orchestrator.govern_tool_proposal(
                context,
                AgentToolProposal::new(
                    format!("{label}-absent-tool-{index}"),
                    absent_tool,
                    1,
                    "{}",
                )?,
            ),
            Err(AgentOrchestratorError::Governance(
                AgentGovernanceError::UnknownTool
            ))
        ));
        let audit = orchestrator.governance_audit_records();
        assert_eq!(audit.len(), audit_index + 1);
        assert!(matches!(
            &audit[audit_index],
            AgentGovernanceRecord::Tool(record)
                if record.attribution().agent_id() == context.agent_id()
                    && record.attribution().task_id() == context.task_id()
                    && record.lifecycle()
                        == AgentToolGovernanceLifecycleState::ValidationRejected
                    && record.execution() == AgentExecutionDisposition::NotAttempted
        ));
        assert!(orchestrator
            .pending_governance_approval(context.task_id())?
            .is_none());
    }
    assert_eq!(
        (
            orchestrator.task_count(),
            orchestrator.run_count(),
            orchestrator.events().len(),
            orchestrator.runtime_event_count(),
            orchestrator.engineering_quality_events().len(),
            orchestrator.engineering_quality_audit_records().len(),
        ),
        inert_state
    );
    assert_eq!(
        orchestrator
            .task(context.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    Ok(())
}

fn advance_to_synthesis(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let (root, coding) = start_workflow(orchestrator)?;
    complete_stage(orchestrator, &coding, "coding-response", CHANGE_PROPOSAL)?;
    let qa = active_child_context(orchestrator)?;
    complete_stage(orchestrator, &qa, "qa-response", VALIDATION_REPORT)?;
    let security = active_child_context(orchestrator)?;
    complete_stage(
        orchestrator,
        &security,
        "security-response",
        RISK_ASSESSMENT,
    )?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    Ok((root, synthesis))
}

fn advance_to_security(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let (root, coding) = start_workflow(orchestrator)?;
    complete_stage(
        orchestrator,
        &coding,
        "coding-before-security",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(orchestrator)?;
    complete_stage(orchestrator, &qa, "qa-before-security", VALIDATION_REPORT)?;
    let security = active_child_context(orchestrator)?;
    Ok((root, security))
}

#[test]
fn deterministic_contracts_preserve_fixture_provenance_without_execution_authority(
) -> Result<(), Box<dyn Error>> {
    let request = fixture_request()?;
    let coding_input = request.build_coding_input()?;
    assert!(coding_input.contains("engineering-quality-v1"));
    assert!(coding_input.contains("stage=coding"));
    assert!(coding_input.contains("fixture_based=true"));
    assert!(coding_input.contains(SECRET_SENTINEL));
    assert!(coding_input.len() <= MAX_ENGINEERING_STAGE_INPUT_BYTES);

    let proposal = proposal(&request)?;
    assert_eq!(proposal.task_id(), &task_id("coding-task")?);
    assert_eq!(proposal.quality(), ChangeProposalQuality::Complete);
    assert_eq!(proposal.affected_file_ids(), [id("src-main")?]);
    assert_eq!(proposal.patch_operations().len(), 1);
    assert_eq!(proposal.patch_operations()[0].file_id(), &id("src-main")?);
    assert_eq!(proposal.transfer(), CHANGE_PROPOSAL);
    assert!(proposal.transfer().len() <= MAX_CHANGE_PROPOSAL_BYTES);
    assert!(proposal.capabilities().iter().all(
        |capability| capability.disposition() == EngineeringCapabilityDisposition::ProposalOnly
    ));

    let qa_input = request.build_qa_input(&proposal)?;
    assert!(qa_input.contains("stage=qa-validation"));
    assert!(qa_input.contains(CHANGE_PROPOSAL));
    assert!(qa_input.contains("No check was executed"));
    assert!(!qa_input.contains(SECRET_SENTINEL));
    assert!(qa_input.len() <= MAX_ENGINEERING_STAGE_INPUT_BYTES);
    let validation = validation(&request, &proposal)?;
    assert_eq!(validation.task_id(), &task_id("qa-task")?);
    assert_eq!(validation.conclusion(), ValidationConclusion::Adequate);
    assert_eq!(validation.transfer(), VALIDATION_REPORT);
    assert!(validation.transfer().len() <= MAX_VALIDATION_REPORT_BYTES);

    let qa = QaStageOutcome::Completed(validation.clone());
    let security_input = request.build_security_input(&proposal, &qa)?;
    assert!(security_input.contains("stage=security-risk"));
    assert!(security_input.contains(VALIDATION_REPORT));
    assert!(!security_input.contains(SECRET_SENTINEL));
    assert!(security_input.len() <= MAX_ENGINEERING_STAGE_INPUT_BYTES);
    let risk = risk(&request, &proposal, &qa)?;
    assert_eq!(risk.task_id(), &task_id("security-task")?);
    assert_eq!(risk.findings().len(), 2);
    assert!(matches!(
        risk.dependency_evidence(),
        DependencyEvidenceStatus::Available(references)
            if references.len() == 1
                && references[0].id().as_str() == "obs-dependency"
    ));
    assert_eq!(risk.transfer(), RISK_ASSESSMENT);
    assert!(risk.transfer().len() <= MAX_RISK_ASSESSMENT_BYTES);

    let coding = CodingStageOutcome::Completed(Box::new(proposal.clone()));
    let security = SecurityStageOutcome::Completed(risk.clone());
    let synthesis_input = request.build_synthesis_input(&coding, &qa, &security)?;
    assert!(synthesis_input.contains("stage=personal-synthesis"));
    assert!(synthesis_input.contains("changes_applied=false"));
    assert!(synthesis_input.contains("tests_executed=false"));
    assert!(!synthesis_input.contains(SECRET_SENTINEL));
    assert!(synthesis_input.len() <= MAX_ENGINEERING_STAGE_INPUT_BYTES);
    let synthesis = request.parse_synthesis(&coding, &qa, &security, SYNTHESIS)?;
    assert_eq!(
        synthesis.approval_requirement(),
        ai_agent_assistant_lib::agent::engineering_quality::EngineeringApprovalRequirement::RequiredBeforeMutation
    );
    assert_eq!(synthesis.status(), EngineeringReviewStatus::Complete);

    let debug = format!("{request:?} {proposal:?} {validation:?} {risk:?} {synthesis:?}");
    for sentinel in [
        OBJECTIVE,
        SECRET_SENTINEL,
        "Replace the literal 41",
        "Fixture evidence demonstrates",
        "Callers may rely",
        "Fixture-only proposal reviewed",
    ] {
        assert!(!debug.contains(sentinel));
    }
    assert_eq!(MAX_ENGINEERING_WORKFLOW_EVENTS, 16);
    assert_eq!(MAX_ENGINEERING_AUDIT_RECORDS, 16);
    Ok(())
}

#[test]
fn capability_classifier_is_closed_and_denied_requests_force_partial_synthesis(
) -> Result<(), Box<dyn Error>> {
    let proposal_only = [
        EngineeringCapability::FixtureInspect,
        EngineeringCapability::FixtureTextSearch,
        EngineeringCapability::ArchitectureExplain,
        EngineeringCapability::DiffReview,
        EngineeringCapability::ImplementationPlan,
        EngineeringCapability::PatchProposal,
        EngineeringCapability::ValidationPlan,
        EngineeringCapability::FormattingPlan,
    ];
    let denied = [
        EngineeringCapability::FileWrite,
        EngineeringCapability::FileDelete,
        EngineeringCapability::OutsideFixturePath,
        EngineeringCapability::DependencyInstall,
        EngineeringCapability::PackageManagerExecute,
        EngineeringCapability::TestExecute,
        EngineeringCapability::FormatterExecute,
        EngineeringCapability::GitCommit,
        EngineeringCapability::GitPush,
        EngineeringCapability::BranchDelete,
        EngineeringCapability::DestructiveShell,
        EngineeringCapability::CredentialAccess,
        EngineeringCapability::NetworkAccess,
    ];
    assert!(proposal_only.iter().all(
        |capability| capability.disposition() == EngineeringCapabilityDisposition::ProposalOnly
    ));
    assert!(denied
        .iter()
        .all(|capability| capability.disposition() == EngineeringCapabilityDisposition::Denied));

    let request = fixture_request()?;
    let denied_raw = CHANGE_PROPOSAL.replace(
        "\"validation-plan\"]",
        "\"validation-plan\",\"file-write\",\"dependency-install\",\"git-commit\",\"git-push\"]",
    );
    let proposal = request.parse_change_proposal(task_id("coding-denied")?, &denied_raw)?;
    assert_eq!(
        proposal.quality(),
        ChangeProposalQuality::PartialDeniedCapability
    );
    assert!(!proposal.transfer().contains("file-write"));
    assert!(!proposal.transfer().contains("dependency-install"));
    assert!(!proposal.transfer().contains("git-commit"));
    assert!(!proposal.transfer().contains("git-push"));
    assert!(proposal
        .transfer()
        .contains("\"contains_denied_capability\":true"));

    let validation = validation(&request, &proposal)?;
    let qa = QaStageOutcome::Completed(validation);
    let risk = risk(&request, &proposal, &qa)?;
    let coding = CodingStageOutcome::Completed(Box::new(proposal));
    let security = SecurityStageOutcome::Completed(risk);
    let partial_synthesis = SYNTHESIS
        .replace(
            "\"unresolved_issues\":[]",
            "\"unresolved_issues\":[\"Denied capabilities remain unresolved.\"]",
        )
        .replace("\"status\":\"complete\"", "\"status\":\"partial\"")
        .replace(
            "proposal reviewed",
            "proposal review is partial and was reviewed",
        );
    let synthesis = request.parse_synthesis(&coding, &qa, &security, &partial_synthesis)?;
    assert_eq!(synthesis.status(), EngineeringReviewStatus::Partial);
    Ok(())
}

#[test]
fn fixture_catalog_and_application_evidence_reject_path_escape_and_unknown_bindings(
) -> Result<(), Box<dyn Error>> {
    for invalid_path in [
        "/tmp/fixture.rs",
        "../fixture.rs",
        "src/../fixture.rs",
        "file://fixture.rs",
        "C:/fixture.rs",
        "src\\fixture.rs",
        "src/fixture\u{0085}.rs",
    ] {
        assert_eq!(
            FixtureRepositoryFile::new("fixture", invalid_path, "bounded content"),
            Err(EngineeringQualityError::InvalidFixturePath)
        );
    }

    let file = FixtureRepositoryFile::new("fixture", "src/lib.rs", "bounded content")?;
    assert_eq!(
        FixtureRepositoryCatalog::new(vec![file.clone(), file]),
        Err(EngineeringQualityError::DuplicateReference)
    );

    let fixtures = FixtureRepositoryCatalog::new(vec![FixtureRepositoryFile::new(
        "fixture",
        "src/lib.rs",
        "bounded content",
    )?])?;
    let criteria = vec![AcceptanceCriterion::new("criterion", "Bounded criterion")?];
    let evidence =
        ApplicationValidationEvidenceCatalog::new(vec![ApplicationValidationEvidence::new(
            "evidence",
            ApplicationEvidenceKind::FixtureObservation,
            ApplicationEvidenceStatus::ObservedFixture,
            "Bounded observation",
            vec![id("unknown-criterion")?],
            vec![id("fixture")?],
        )?])?;
    assert_eq!(
        EngineeringQualityWorkflowRequest::new(OBJECTIVE, fixtures, criteria, evidence),
        Err(EngineeringQualityError::UnknownReference)
    );
    Ok(())
}

#[test]
fn strict_outputs_reject_authority_claims_fabricated_evidence_and_parser_ambiguity(
) -> Result<(), Box<dyn Error>> {
    let request = fixture_request()?;

    let invalid_proposals = [
        format!(" {CHANGE_PROPOSAL}"),
        CHANGE_PROPOSAL.replace(
            "\"proposal_only\":true",
            "\"proposal_only\":true,\"reasoning\":\"private chain\"",
        ),
        CHANGE_PROPOSAL.replace(
            "\"fixture_based\":true",
            "\"fixture_based\":true,\"fixture_based\":true",
        ),
        CHANGE_PROPOSAL.replace("\"proposal_only\":true", "\"proposal_only\":false"),
        CHANGE_PROPOSAL.replace("\"validation-plan\"", "\"unknown-capability\""),
    ];
    for (index, raw) in invalid_proposals.iter().enumerate() {
        assert!(request
            .parse_change_proposal(task_id(&format!("invalid-coding-{index}"))?, raw)
            .is_err());
    }

    let missing_patch_capability = CHANGE_PROPOSAL.replace("\"patch-proposal\",", "");
    assert_eq!(
        request.parse_change_proposal(task_id("missing-capability")?, &missing_patch_capability),
        Err(EngineeringQualityError::CapabilityInconsistent)
    );
    let execution_claim = CHANGE_PROPOSAL.replace(
        "Correct the fixture-only answer proposal",
        "The patch applied to the fixture",
    );
    assert_eq!(
        request.parse_change_proposal(task_id("execution-claim")?, &execution_claim),
        Err(EngineeringQualityError::AuthorityClaim)
    );

    let proposal = proposal(&request)?;
    let qa_authority = VALIDATION_REPORT.replace(
        "\"approval_authority\":false",
        "\"approval_authority\":true",
    );
    assert_eq!(
        request.parse_validation_report(task_id("qa-authority")?, &proposal, &qa_authority),
        Err(EngineeringQualityError::AuthorityClaim)
    );
    let fabricated_demonstration =
        VALIDATION_REPORT.replace("\"id\":\"obs-bug\"", "\"id\":\"proposed-check\"");
    assert_eq!(
        request.parse_validation_report(
            task_id("qa-fabricated")?,
            &proposal,
            &fabricated_demonstration,
        ),
        Err(EngineeringQualityError::EvidenceInconsistent)
    );

    let validation = validation(&request, &proposal)?;
    let qa = QaStageOutcome::Completed(validation);
    let security_authority = RISK_ASSESSMENT.replace(
        "\"authorization_granted\":false",
        "\"authorization_granted\":true",
    );
    assert_eq!(
        request.parse_risk_assessment(
            task_id("security-authority")?,
            &proposal,
            &qa,
            &security_authority,
        ),
        Err(EngineeringQualityError::AuthorityClaim)
    );
    let unsupported_certainty = RISK_ASSESSMENT
        .replace("\"confidence\":\"medium\"", "\"confidence\":\"high\"")
        .replace(
            "Callers may rely on the fixture's prior value.",
            "The fixture may have caller compatibility risk.",
        );
    assert_eq!(
        request.parse_risk_assessment(
            task_id("security-certainty")?,
            &proposal,
            &qa,
            &unsupported_certainty,
        ),
        Err(EngineeringQualityError::EvidenceInconsistent)
    );

    let risk = risk(&request, &proposal, &qa)?;
    let coding = CodingStageOutcome::Completed(Box::new(proposal));
    let security = SecurityStageOutcome::Completed(risk);
    let mutation_claim = SYNTHESIS.replace("\"changes_applied\":false", "\"changes_applied\":true");
    assert_eq!(
        request.parse_synthesis(&coding, &qa, &security, &mutation_claim),
        Err(EngineeringQualityError::AuthorityClaim)
    );
    let wrong_files = SYNTHESIS.replace("\"src-main\"", "\"dependency-lock\"");
    assert_eq!(
        request.parse_synthesis(&coding, &qa, &security, &wrong_files),
        Err(EngineeringQualityError::SynthesisInconsistent)
    );
    Ok(())
}

#[test]
fn orchestrated_success_is_sequential_bounded_attributed_and_proposal_only(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut orchestrator)?;
    let root_id = root.task_id().clone();
    let coding_id = coding.task_id().clone();
    assert_eq!(coding.agent_id(), AgentId::Coding);
    assert_eq!(coding.depth(), 1);
    assert_eq!(coding.root_task_id().task_id(), &root_id);
    assert_eq!(
        coding.parent_task_id().map(|parent| parent.task_id()),
        Some(&root_id)
    );
    assert_eq!(coding.runtime_id(), RuntimeId::Native);
    assert_eq!(
        orchestrator.selected_workflow(),
        Some(AgentWorkflowSelection::EngineeringQuality)
    );

    complete_stage(
        &mut orchestrator,
        &coding,
        "coding-success",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&orchestrator)?;
    let qa_id = qa.task_id().clone();
    assert_eq!(qa.agent_id(), AgentId::QaValidation);
    assert_eq!(qa.depth(), 1);
    assert_eq!(qa.parent_task_id().map(|id| id.task_id()), Some(&root_id));
    assert_ne!(qa_id, coding_id);

    complete_stage(&mut orchestrator, &qa, "qa-success", VALIDATION_REPORT)?;
    let security = active_child_context(&orchestrator)?;
    let security_id = security.task_id().clone();
    assert_eq!(security.agent_id(), AgentId::SecurityRisk);
    assert_eq!(security.depth(), 1);
    assert_eq!(
        security.parent_task_id().map(|id| id.task_id()),
        Some(&root_id)
    );
    assert_ne!(security_id, qa_id);

    complete_stage(
        &mut orchestrator,
        &security,
        "security-success",
        RISK_ASSESSMENT,
    )?;
    assert!(orchestrator.active_child_task().is_none());
    let synthesis = orchestrator.current_context(&root_id)?;
    assert_eq!(synthesis.agent_id(), AgentId::PersonalAssistant);
    assert_eq!(synthesis.depth(), 0);
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "synthesis-success",
        SYNTHESIS,
    )?;

    assert_eq!(
        orchestrator.task_count(),
        MAX_ENGINEERING_QUALITY_TASKS_PER_ROOT
    );
    assert_eq!(
        orchestrator.run_count(),
        MAX_ENGINEERING_QUALITY_RUNTIME_RUNS_PER_ROOT
    );
    assert_eq!(MAX_ENGINEERING_QUALITY_CHILDREN_PER_ROOT, 3);
    assert_eq!(orchestrator.runtime_event_count(), 12);
    assert_eq!(orchestrator.events().len(), 16);
    assert_eq!(orchestrator.shared_memory_proposal_count(), 0);
    assert_eq!(
        orchestrator.task(&root_id).map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );

    let result = orchestrator
        .engineering_quality_result()
        .ok_or("missing engineering workflow result")?;
    assert_eq!(result.root_task_id().task_id(), &root_id);
    assert!(matches!(
        result.coding(),
        CodingStageOutcome::Completed(proposal)
            if proposal.task_id() == &coding_id
                && proposal.quality() == ChangeProposalQuality::Complete
    ));
    assert!(matches!(
        result.qa(),
        QaStageOutcome::Completed(report)
            if report.task_id() == &qa_id
                && report.conclusion() == ValidationConclusion::Adequate
    ));
    assert!(matches!(
        result.security(),
        SecurityStageOutcome::Completed(assessment)
            if assessment.task_id() == &security_id
    ));
    assert_eq!(
        result.synthesis().status(),
        EngineeringReviewStatus::Complete
    );

    assert!(matches!(
        orchestrator.engineering_quality_events(),
        [
            EngineeringQualityWorkflowEvent::CodingStarted { .. },
            EngineeringQualityWorkflowEvent::CodingCompleted { .. },
            EngineeringQualityWorkflowEvent::QaValidationStarted { .. },
            EngineeringQualityWorkflowEvent::QaValidationCompleted { .. },
            EngineeringQualityWorkflowEvent::SecurityReviewStarted { .. },
            EngineeringQualityWorkflowEvent::SecurityReviewCompleted { .. },
            EngineeringQualityWorkflowEvent::SynthesisStarted { .. },
            EngineeringQualityWorkflowEvent::Completed { .. },
        ]
    ));
    let audit = orchestrator.engineering_quality_audit_records();
    assert_eq!(audit.len(), 8);
    assert!(audit
        .windows(2)
        .all(|pair| pair[0].sequence() < pair[1].sequence()));
    assert_eq!(audit[0].attribution().agent_id(), AgentId::Coding);
    assert_eq!(audit[0].attribution().root_task_id().task_id(), &root_id);
    assert_eq!(
        audit[0]
            .attribution()
            .parent_task_id()
            .map(|id| id.task_id()),
        Some(&root_id)
    );
    assert_eq!(audit[0].attribution().runtime_id(), RuntimeId::Native);
    assert_eq!(audit[0].attribution().depth(), 1);
    assert_eq!(audit[2].predecessor_task_id(), Some(&coding_id));
    assert_eq!(audit[4].predecessor_task_id(), Some(&qa_id));
    assert_eq!(
        audit[6].attribution().agent_id(),
        AgentId::PersonalAssistant
    );
    assert_eq!(audit[6].attribution().depth(), 0);
    assert_eq!(
        audit[1].capability_disposition(),
        EngineeringCapabilityAuditDisposition::ProposalOnly
    );

    let starts = recorder.starts();
    assert_eq!(starts.len(), 5);
    assert!(starts[1].selected_text.contains("stage=coding"));
    assert!(starts[1].selected_text.contains(SECRET_SENTINEL));
    assert!(starts[2].selected_text.contains("stage=qa-validation"));
    assert!(!starts[2].selected_text.contains(SECRET_SENTINEL));
    assert!(starts[3].selected_text.contains("stage=security-risk"));
    assert!(starts[4].selected_text.contains("stage=personal-synthesis"));

    let debug = format!(
        "{orchestrator:?} {:?} {:?} {:?}",
        orchestrator.engineering_quality_events(),
        audit,
        result,
    );
    for sentinel in [
        OBJECTIVE,
        SECRET_SENTINEL,
        "Replace the literal 41",
        "Fixture evidence demonstrates",
        "Callers may rely",
    ] {
        assert!(!debug.contains(sentinel));
    }
    Ok(())
}

#[test]
fn specialist_failures_preserve_only_validated_predecessors_for_partial_synthesis(
) -> Result<(), Box<dyn Error>> {
    let mut coding_failure = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut coding_failure)?;
    coding_failure.accept_runtime_event(
        coding.task_id(),
        started(&coding, "coding-runtime-failure")?,
    )?;
    assert!(matches!(
        coding_failure.accept_runtime_event(coding.task_id(), failed(&coding, 1)?)?,
        RuntimeEventAcceptance::ResponseFailed { .. }
    ));
    assert!(coding_failure.active_child_task().is_none());
    let synthesis = coding_failure.current_context(root.task_id())?;
    complete_stage(
        &mut coding_failure,
        &synthesis,
        "coding-fallback",
        PARTIAL_SYNTHESIS_NO_PROPOSAL,
    )?;
    let result = coding_failure
        .engineering_quality_result()
        .ok_or("missing Coding-failure result")?;
    assert!(matches!(result.coding(), CodingStageOutcome::Failed(_)));
    assert!(matches!(
        result.qa(),
        QaStageOutcome::SkippedCodingUnavailable
    ));
    assert!(matches!(
        result.security(),
        SecurityStageOutcome::SkippedCodingUnavailable
    ));
    assert_eq!(
        result.synthesis().status(),
        EngineeringReviewStatus::Partial
    );
    assert!(coding_failure
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(
            event,
            EngineeringQualityWorkflowEvent::PartialFailure {
                stage: EngineeringQualityStage::Coding,
                code: EngineeringPartialFailureCode::RuntimeFailed,
            }
        )));

    let mut qa_failure = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut qa_failure)?;
    complete_stage(
        &mut qa_failure,
        &coding,
        "coding-before-qa-failure",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&qa_failure)?;
    let qa_id = qa.task_id().clone();
    qa_failure.accept_runtime_event(&qa.task_id().clone(), started(&qa, "qa-runtime-failure")?)?;
    qa_failure.accept_runtime_event(&qa.task_id().clone(), failed(&qa, 1)?)?;
    let security = active_child_context(&qa_failure)?;
    assert_eq!(security.agent_id(), AgentId::SecurityRisk);
    complete_stage(
        &mut qa_failure,
        &security,
        "security-after-qa-failure",
        RISK_ASSESSMENT,
    )?;
    let synthesis = qa_failure.current_context(root.task_id())?;
    complete_stage(
        &mut qa_failure,
        &synthesis,
        "qa-partial-synthesis",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    let result = qa_failure
        .engineering_quality_result()
        .ok_or("missing QA-failure result")?;
    assert!(matches!(result.coding(), CodingStageOutcome::Completed(_)));
    assert!(matches!(result.qa(), QaStageOutcome::Failed(_)));
    assert!(matches!(
        result.security(),
        SecurityStageOutcome::Completed(_)
    ));
    assert_eq!(
        result.synthesis().status(),
        EngineeringReviewStatus::Partial
    );
    assert!(qa_failure
        .engineering_quality_audit_records()
        .iter()
        .filter(|record| record.stage() == EngineeringQualityStage::SecurityReview)
        .all(|record| record.predecessor_task_id() == Some(&qa_id)));

    let mut security_failure = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut security_failure)?;
    complete_stage(
        &mut security_failure,
        &coding,
        "coding-before-security-failure",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&security_failure)?;
    complete_stage(
        &mut security_failure,
        &qa,
        "qa-before-security-failure",
        VALIDATION_REPORT,
    )?;
    let security = active_child_context(&security_failure)?;
    security_failure.accept_runtime_event(
        security.task_id(),
        started(&security, "security-runtime-failure")?,
    )?;
    security_failure.accept_runtime_event(security.task_id(), failed(&security, 1)?)?;
    let synthesis = security_failure.current_context(root.task_id())?;
    complete_stage(
        &mut security_failure,
        &synthesis,
        "security-partial-synthesis",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    let result = security_failure
        .engineering_quality_result()
        .ok_or("missing Security-failure result")?;
    assert!(matches!(result.security(), SecurityStageOutcome::Failed(_)));
    assert_eq!(
        result.synthesis().status(),
        EngineeringReviewStatus::Partial
    );
    Ok(())
}

#[test]
fn runtime_reported_cancellation_preserves_cancelled_stage_semantics() -> Result<(), Box<dyn Error>>
{
    let mut coding_cancel = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut coding_cancel)?;
    coding_cancel.accept_runtime_event(
        &coding.task_id().clone(),
        started(&coding, "coding-cancel")?,
    )?;
    coding_cancel
        .accept_runtime_event(&coding.task_id().clone(), runtime_cancelled(&coding, 1)?)?;
    let synthesis = coding_cancel.current_context(root.task_id())?;
    complete_stage(
        &mut coding_cancel,
        &synthesis,
        "coding-cancel-synthesis",
        PARTIAL_SYNTHESIS_NO_PROPOSAL,
    )?;
    assert!(matches!(
        coding_cancel
            .engineering_quality_result()
            .ok_or("missing Coding-cancel result")?
            .coding(),
        CodingStageOutcome::Cancelled
    ));

    let mut qa_cancel = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut qa_cancel)?;
    complete_stage(
        &mut qa_cancel,
        &coding,
        "coding-before-qa-cancel",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&qa_cancel)?;
    qa_cancel.accept_runtime_event(&qa.task_id().clone(), started(&qa, "qa-cancel")?)?;
    qa_cancel.accept_runtime_event(&qa.task_id().clone(), runtime_cancelled(&qa, 1)?)?;
    let security = active_child_context(&qa_cancel)?;
    complete_stage(
        &mut qa_cancel,
        &security,
        "security-after-qa-cancel",
        RISK_ASSESSMENT,
    )?;
    let synthesis = qa_cancel.current_context(root.task_id())?;
    complete_stage(
        &mut qa_cancel,
        &synthesis,
        "qa-cancel-synthesis",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    assert!(matches!(
        qa_cancel
            .engineering_quality_result()
            .ok_or("missing QA-cancel result")?
            .qa(),
        QaStageOutcome::Cancelled
    ));

    let mut security_cancel = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut security_cancel)?;
    complete_stage(
        &mut security_cancel,
        &coding,
        "coding-before-security-cancel",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&security_cancel)?;
    complete_stage(
        &mut security_cancel,
        &qa,
        "qa-before-security-cancel",
        VALIDATION_REPORT,
    )?;
    let security = active_child_context(&security_cancel)?;
    security_cancel
        .accept_runtime_event(security.task_id(), started(&security, "security-cancel")?)?;
    security_cancel.accept_runtime_event(security.task_id(), runtime_cancelled(&security, 1)?)?;
    let synthesis = security_cancel.current_context(root.task_id())?;
    complete_stage(
        &mut security_cancel,
        &synthesis,
        "security-cancel-synthesis",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    assert!(matches!(
        security_cancel
            .engineering_quality_result()
            .ok_or("missing Security-cancel result")?
            .security(),
        SecurityStageOutcome::Cancelled
    ));

    let mut synthesis_cancel = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, synthesis) = advance_to_synthesis(&mut synthesis_cancel)?;
    synthesis_cancel.accept_runtime_event(
        root.task_id(),
        started(&synthesis, "synthesis-runtime-cancel")?,
    )?;
    synthesis_cancel.accept_runtime_event(root.task_id(), runtime_cancelled(&synthesis, 1)?)?;
    assert_eq!(
        synthesis_cancel
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(synthesis_cancel.engineering_quality_result().is_none());
    assert!(matches!(
        synthesis_cancel.engineering_quality_events().last(),
        Some(EngineeringQualityWorkflowEvent::Cancelled {
            stage: EngineeringQualityStage::Synthesis,
        })
    ));
    Ok(())
}

#[test]
fn denied_capability_and_incomplete_qa_remain_inert_typed_partial_results(
) -> Result<(), Box<dyn Error>> {
    let denied_raw = CHANGE_PROPOSAL.replace(
        "\"validation-plan\"]",
        "\"validation-plan\",\"file-write\",\"dependency-install\",\"git-commit\",\"git-push\"]",
    );
    let mut denied_workflow = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut denied_workflow)?;
    complete_stage(
        &mut denied_workflow,
        &coding,
        "coding-denied-capability",
        &denied_raw,
    )?;
    assert_eq!(
        active_child_context(&denied_workflow)?.agent_id(),
        AgentId::QaValidation
    );
    assert!(matches!(
        denied_workflow.engineering_quality_events().get(1),
        Some(EngineeringQualityWorkflowEvent::PartialFailure {
            stage: EngineeringQualityStage::Coding,
            code: EngineeringPartialFailureCode::ContainsDeniedCapability,
        })
    ));
    assert_eq!(
        denied_workflow.engineering_quality_audit_records()[1].capability_disposition(),
        EngineeringCapabilityAuditDisposition::DeniedRequested
    );
    let qa = active_child_context(&denied_workflow)?;
    complete_stage(
        &mut denied_workflow,
        &qa,
        "qa-after-denied-capability",
        VALIDATION_REPORT,
    )?;
    let security = active_child_context(&denied_workflow)?;
    complete_stage(
        &mut denied_workflow,
        &security,
        "security-after-denied-capability",
        RISK_ASSESSMENT,
    )?;
    let synthesis = denied_workflow.current_context(root.task_id())?;
    complete_stage(
        &mut denied_workflow,
        &synthesis,
        "denied-capability-synthesis",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    let result = denied_workflow
        .engineering_quality_result()
        .ok_or("missing denied-capability result")?;
    assert!(matches!(
        result.coding(),
        CodingStageOutcome::Completed(proposal)
            if proposal.quality() == ChangeProposalQuality::PartialDeniedCapability
    ));
    assert_eq!(
        result.synthesis().status(),
        EngineeringReviewStatus::Partial
    );

    let incomplete_report = VALIDATION_REPORT
        .replace(
            "\"disposition\":\"demonstrated\"",
            "\"disposition\":\"not-demonstrated\"",
        )
        .replace(
            "\"conclusion\":\"adequate\"",
            "\"conclusion\":\"incomplete\"",
        );
    let mut incomplete_workflow = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, coding) = start_workflow(&mut incomplete_workflow)?;
    complete_stage(
        &mut incomplete_workflow,
        &coding,
        "coding-before-incomplete-qa",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&incomplete_workflow)?;
    complete_stage(
        &mut incomplete_workflow,
        &qa,
        "qa-incomplete",
        &incomplete_report,
    )?;
    let security = active_child_context(&incomplete_workflow)?;
    complete_stage(
        &mut incomplete_workflow,
        &security,
        "security-after-incomplete-qa",
        RISK_ASSESSMENT,
    )?;
    let synthesis = incomplete_workflow.current_context(root.task_id())?;
    complete_stage(
        &mut incomplete_workflow,
        &synthesis,
        "incomplete-qa-synthesis",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    let result = incomplete_workflow
        .engineering_quality_result()
        .ok_or("missing incomplete-QA result")?;
    assert!(matches!(
        result.qa(),
        QaStageOutcome::Completed(report)
            if report.conclusion() == ValidationConclusion::Incomplete
    ));
    assert_eq!(
        result.synthesis().status(),
        EngineeringReviewStatus::Partial
    );
    Ok(())
}

#[test]
fn start_failures_consume_bounded_attempts_without_retry_or_fabricated_completion(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(2));
    let mut coding_start_failure = AgentOrchestrator::new(runtime)?;
    let root = coding_start_failure.start_root(OBJECTIVE)?;
    let acceptance =
        coding_start_failure.start_engineering_quality_workflow(&root, fixture_request()?)?;
    let fallback = match acceptance {
        EngineeringQualityWorkflowAcceptance::PersonalFallbackStarted { context } => context,
        EngineeringQualityWorkflowAcceptance::CodingStarted { .. } => {
            return Err("Coding unexpectedly started".into());
        }
    };
    assert_eq!(recorder.starts().len(), 3);
    assert_eq!(coding_start_failure.task_count(), 2);
    assert_eq!(coding_start_failure.run_count(), 3);
    assert_eq!(
        coding_start_failure.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::CodingStartFailed)
    );
    complete_stage(
        &mut coding_start_failure,
        &fallback,
        "coding-start-fallback",
        PARTIAL_SYNTHESIS_NO_PROPOSAL,
    )?;
    assert!(coding_start_failure.engineering_quality_result().is_some());

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(3));
    let mut qa_start_failure = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut qa_start_failure)?;
    complete_stage(
        &mut qa_start_failure,
        &coding,
        "coding-before-qa-start-failure",
        CHANGE_PROPOSAL,
    )?;
    assert_eq!(
        qa_start_failure.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::QaStartFailed)
    );
    assert_eq!(recorder.starts().len(), 4);
    let security = active_child_context(&qa_start_failure)?;
    assert_eq!(security.agent_id(), AgentId::SecurityRisk);
    complete_stage(
        &mut qa_start_failure,
        &security,
        "security-after-qa-start-failure",
        RISK_ASSESSMENT,
    )?;
    let synthesis = qa_start_failure.current_context(root.task_id())?;
    complete_stage(
        &mut qa_start_failure,
        &synthesis,
        "qa-start-partial-synthesis",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    assert!(matches!(
        qa_start_failure
            .engineering_quality_result()
            .ok_or("missing QA-start-failure result")?
            .qa(),
        QaStageOutcome::Failed(_)
    ));

    let mut qa_and_synthesis_failure =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailuresAt(3, 5)))?;
    let (root, coding) = start_workflow(&mut qa_and_synthesis_failure)?;
    complete_stage(
        &mut qa_and_synthesis_failure,
        &coding,
        "coding-before-qa-and-synthesis-start-failure",
        CHANGE_PROPOSAL,
    )?;
    let security = active_child_context(&qa_and_synthesis_failure)?;
    complete_stage(
        &mut qa_and_synthesis_failure,
        &security,
        "security-before-synthesis-start-failure",
        RISK_ASSESSMENT,
    )?;
    assert_eq!(
        qa_and_synthesis_failure.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::QaAndSynthesisStartFailed)
    );
    assert_eq!(
        qa_and_synthesis_failure
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(qa_and_synthesis_failure
        .engineering_quality_result()
        .is_none());

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailuresAtThree(3, 4, 5));
    let mut triple_failure = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut triple_failure)?;
    complete_stage(
        &mut triple_failure,
        &coding,
        "coding-before-triple-start-failure",
        CHANGE_PROPOSAL,
    )?;
    assert_eq!(
        triple_failure.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::QaSecurityAndSynthesisStartFailed)
    );
    assert_eq!(recorder.starts().len(), 5);
    assert_eq!(triple_failure.task_count(), 4);
    assert_eq!(triple_failure.run_count(), 5);
    assert_eq!(
        triple_failure
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(triple_failure.active_child_task().is_none());
    assert!(triple_failure.engineering_quality_result().is_none());
    assert!(triple_failure.current_context(root.task_id()).is_err());
    Ok(())
}

#[test]
fn remaining_start_failure_snapshots_preserve_exact_attempts_and_terminal_state(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailuresAt(2, 3));
    let mut coding_and_synthesis = AgentOrchestrator::new(runtime)?;
    let root = coding_and_synthesis.start_root(OBJECTIVE)?;
    assert_eq!(
        coding_and_synthesis.start_engineering_quality_workflow(&root, fixture_request()?),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start)
        ))
    );
    assert_eq!(recorder.starts().len(), 3);
    assert_eq!(coding_and_synthesis.task_count(), 2);
    assert_eq!(coding_and_synthesis.run_count(), 3);
    assert_eq!(
        coding_and_synthesis.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::CodingAndSynthesisStartFailed)
    );
    assert_eq!(
        coding_and_synthesis
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    let coding_id = engineering_task_id(&coding_and_synthesis, AgentId::Coding)?;
    assert_eq!(
        coding_and_synthesis
            .task(&coding_id)
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(coding_and_synthesis.active_child_task().is_none());
    assert!(coding_and_synthesis.engineering_quality_result().is_none());
    assert_eq!(coding_and_synthesis.engineering_quality_events().len(), 4);
    assert_eq!(
        coding_and_synthesis
            .engineering_quality_audit_records()
            .iter()
            .map(|record| (record.stage(), record.outcome()))
            .collect::<Vec<_>>(),
        vec![
            (
                EngineeringQualityStage::Coding,
                EngineeringQualityAuditOutcome::Started,
            ),
            (
                EngineeringQualityStage::Coding,
                EngineeringQualityAuditOutcome::PartialFailure(
                    EngineeringPartialFailureCode::RuntimeStartFailed,
                ),
            ),
            (
                EngineeringQualityStage::Synthesis,
                EngineeringQualityAuditOutcome::Started,
            ),
            (
                EngineeringQualityStage::Synthesis,
                EngineeringQualityAuditOutcome::PartialFailure(
                    EngineeringPartialFailureCode::RuntimeStartFailed,
                ),
            ),
        ]
    );

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailuresAt(3, 4));
    let mut qa_and_security = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut qa_and_security)?;
    complete_stage(
        &mut qa_and_security,
        &coding,
        "coding-before-qa-security-start-failures",
        CHANGE_PROPOSAL,
    )?;
    assert_eq!(recorder.starts().len(), 5);
    assert_eq!(qa_and_security.task_count(), 4);
    assert_eq!(qa_and_security.run_count(), 5);
    assert_eq!(
        qa_and_security.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::QaAndSecurityStartFailed)
    );
    let qa_id = engineering_task_id(&qa_and_security, AgentId::QaValidation)?;
    let security_id = engineering_task_id(&qa_and_security, AgentId::SecurityRisk)?;
    assert_eq!(
        qa_and_security.task(&qa_id).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        qa_and_security.task(&security_id).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        qa_and_security
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(qa_and_security.current_context(root.task_id()).is_ok());
    assert!(qa_and_security.active_child_task().is_none());
    assert_eq!(qa_and_security.engineering_quality_events().len(), 7);
    assert_eq!(qa_and_security.engineering_quality_audit_records().len(), 7);
    assert!(matches!(
        qa_and_security.engineering_quality_events(),
        [
            EngineeringQualityWorkflowEvent::CodingStarted { .. },
            EngineeringQualityWorkflowEvent::CodingCompleted { .. },
            EngineeringQualityWorkflowEvent::QaValidationStarted { .. },
            EngineeringQualityWorkflowEvent::PartialFailure {
                stage: EngineeringQualityStage::QaValidation,
                code: EngineeringPartialFailureCode::RuntimeStartFailed,
            },
            EngineeringQualityWorkflowEvent::SecurityReviewStarted { .. },
            EngineeringQualityWorkflowEvent::PartialFailure {
                stage: EngineeringQualityStage::SecurityReview,
                code: EngineeringPartialFailureCode::RuntimeStartFailed,
            },
            EngineeringQualityWorkflowEvent::SynthesisStarted { .. },
        ]
    ));

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(4));
    let mut security_start = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut security_start)?;
    complete_stage(
        &mut security_start,
        &coding,
        "coding-before-security-start-failure",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&security_start)?;
    complete_stage(
        &mut security_start,
        &qa,
        "qa-before-security-start-failure",
        VALIDATION_REPORT,
    )?;
    assert_eq!(recorder.starts().len(), 5);
    assert_eq!(
        security_start.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::SecurityStartFailed)
    );
    let security_id = engineering_task_id(&security_start, AgentId::SecurityRisk)?;
    assert_eq!(
        security_start.task(&security_id).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        security_start
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(security_start.current_context(root.task_id()).is_ok());
    assert_eq!(security_start.engineering_quality_events().len(), 7);
    assert_eq!(security_start.engineering_quality_audit_records().len(), 7);

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailuresAt(4, 5));
    let mut security_and_synthesis = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut security_and_synthesis)?;
    complete_stage(
        &mut security_and_synthesis,
        &coding,
        "coding-before-security-synthesis-start-failures",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&security_and_synthesis)?;
    complete_stage(
        &mut security_and_synthesis,
        &qa,
        "qa-before-security-synthesis-start-failures",
        VALIDATION_REPORT,
    )?;
    assert_eq!(recorder.starts().len(), 5);
    assert_eq!(
        security_and_synthesis.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::SecurityAndSynthesisStartFailed)
    );
    assert_eq!(
        security_and_synthesis
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(security_and_synthesis.active_child_task().is_none());
    assert!(security_and_synthesis
        .engineering_quality_result()
        .is_none());
    assert_eq!(security_and_synthesis.engineering_quality_events().len(), 8);
    assert_eq!(
        security_and_synthesis
            .engineering_quality_audit_records()
            .last()
            .map(|record| (record.stage(), record.outcome())),
        Some((
            EngineeringQualityStage::Synthesis,
            EngineeringQualityAuditOutcome::PartialFailure(
                EngineeringPartialFailureCode::RuntimeStartFailed,
            ),
        ))
    );

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(5));
    let mut synthesis_start = AgentOrchestrator::new(runtime)?;
    let (root, security) = advance_to_security(&mut synthesis_start)?;
    complete_stage(
        &mut synthesis_start,
        &security,
        "security-before-synthesis-start-failure",
        RISK_ASSESSMENT,
    )?;
    assert_eq!(recorder.starts().len(), 5);
    assert_eq!(synthesis_start.task_count(), 4);
    assert_eq!(synthesis_start.run_count(), 5);
    assert_eq!(
        synthesis_start.engineering_quality_continuation_failure(),
        Some(EngineeringContinuationFailure::SynthesisStartFailed)
    );
    assert_eq!(
        synthesis_start
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(synthesis_start.active_child_task().is_none());
    assert!(synthesis_start.engineering_quality_result().is_none());
    assert_eq!(synthesis_start.engineering_quality_events().len(), 8);
    assert_eq!(synthesis_start.engineering_quality_audit_records().len(), 8);
    Ok(())
}

#[test]
fn cancellation_is_stage_local_child_first_and_never_advances_after_runtime_rejection(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut direct_child = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut direct_child)?;
    assert_eq!(
        direct_child.cancel_task(coding.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(recorder.starts().len(), 3);
    assert!(direct_child.active_child_task().is_none());
    assert!(direct_child
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(
            event,
            EngineeringQualityWorkflowEvent::Cancelled {
                stage: EngineeringQualityStage::Coding,
            }
        )));
    let synthesis = direct_child.current_context(root.task_id())?;
    complete_stage(
        &mut direct_child,
        &synthesis,
        "coding-cancelled-synthesis",
        PARTIAL_SYNTHESIS_NO_PROPOSAL,
    )?;
    assert!(matches!(
        direct_child
            .engineering_quality_result()
            .ok_or("missing Coding-cancelled result")?
            .coding(),
        CodingStageOutcome::Cancelled
    ));

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut root_cancel = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut root_cancel)?;
    assert_eq!(
        root_cancel.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(recorder.starts().len(), 2);
    assert_eq!(recorder.cancellations().len(), 2);
    assert_eq!(
        root_cancel.task(coding.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        root_cancel.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    let cancel_audit = root_cancel.engineering_quality_audit_records();
    assert_eq!(cancel_audit.len(), 3);
    assert_eq!(cancel_audit[1].attribution().agent_id(), AgentId::Coding);
    assert_eq!(
        cancel_audit[2].attribution().agent_id(),
        AgentId::PersonalAssistant
    );
    assert!(root_cancel.engineering_quality_result().is_none());
    assert!(matches!(
        root_cancel.accept_runtime_event(coding.task_id(), completed(&coding, 0)),
        Err(AgentOrchestratorError::NoActiveRun)
    ));

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(2));
    let mut cancellation_failure = AgentOrchestrator::new(runtime)?;
    let (_root, coding) = start_workflow(&mut cancellation_failure)?;
    assert!(cancellation_failure.cancel_task(coding.task_id()).is_err());
    assert_eq!(recorder.starts().len(), 2);
    assert_eq!(recorder.cancellations().len(), 1);
    assert_eq!(
        cancellation_failure
            .task(coding.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert_eq!(cancellation_failure.engineering_quality_events().len(), 1);

    let mut synthesis_cancel = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, _synthesis) = advance_to_synthesis(&mut synthesis_cancel)?;
    assert_eq!(
        synthesis_cancel.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert!(matches!(
        synthesis_cancel.engineering_quality_events().last(),
        Some(EngineeringQualityWorkflowEvent::Cancelled {
            stage: EngineeringQualityStage::Synthesis,
        })
    ));
    assert!(synthesis_cancel.engineering_quality_result().is_none());
    Ok(())
}

#[test]
fn qa_and_security_child_cancellation_start_only_their_allowed_successors(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut qa_cancel = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut qa_cancel)?;
    complete_stage(
        &mut qa_cancel,
        &coding,
        "coding-before-qa-cancel",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&qa_cancel)?;
    assert_eq!(
        qa_cancel.cancel_task(qa.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(recorder.starts().len(), 4);
    assert_eq!(
        qa_cancel.task(qa.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(qa_cancel
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(
            event,
            EngineeringQualityWorkflowEvent::Cancelled {
                stage: EngineeringQualityStage::QaValidation,
            }
        )));
    let security = active_child_context(&qa_cancel)?;
    assert_eq!(security.agent_id(), AgentId::SecurityRisk);
    complete_stage(
        &mut qa_cancel,
        &security,
        "security-after-qa-cancel",
        RISK_ASSESSMENT,
    )?;
    let synthesis = qa_cancel.current_context(root.task_id())?;
    complete_stage(
        &mut qa_cancel,
        &synthesis,
        "synthesis-after-qa-cancel",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    assert!(matches!(
        qa_cancel
            .engineering_quality_result()
            .ok_or("missing QA-cancelled result")?
            .qa(),
        QaStageOutcome::Cancelled
    ));

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut security_cancel = AgentOrchestrator::new(runtime)?;
    let (root, security) = advance_to_security(&mut security_cancel)?;
    assert_eq!(
        security_cancel.cancel_task(security.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(recorder.starts().len(), 5);
    assert_eq!(
        security_cancel
            .task(security.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(security_cancel
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(
            event,
            EngineeringQualityWorkflowEvent::Cancelled {
                stage: EngineeringQualityStage::SecurityReview,
            }
        )));
    assert!(security_cancel.active_child_task().is_none());
    let synthesis = security_cancel.current_context(root.task_id())?;
    complete_stage(
        &mut security_cancel,
        &synthesis,
        "synthesis-after-security-cancel",
        PARTIAL_SYNTHESIS_WITH_PROPOSAL,
    )?;
    assert!(matches!(
        security_cancel
            .engineering_quality_result()
            .ok_or("missing Security-cancelled result")?
            .security(),
        SecurityStageOutcome::Cancelled
    ));
    Ok(())
}

#[test]
fn qa_security_and_synthesis_cancel_failures_are_retryable_and_emit_no_false_transition(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(3));
    let mut qa_failure = AgentOrchestrator::new(runtime)?;
    let (_root, coding) = start_workflow(&mut qa_failure)?;
    complete_stage(
        &mut qa_failure,
        &coding,
        "coding-before-qa-cancel-failure",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&qa_failure)?;
    let before = (
        qa_failure.task_count(),
        qa_failure.run_count(),
        qa_failure.events().len(),
        qa_failure.runtime_event_count(),
        qa_failure.engineering_quality_events().len(),
        qa_failure.engineering_quality_audit_records().len(),
    );
    assert_eq!(
        qa_failure.cancel_task(qa.task_id()),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
        ))
    );
    assert_eq!(recorder.cancellations().len(), 1);
    assert_eq!(
        (
            qa_failure.task_count(),
            qa_failure.run_count(),
            qa_failure.events().len(),
            qa_failure.runtime_event_count(),
            qa_failure.engineering_quality_events().len(),
            qa_failure.engineering_quality_audit_records().len(),
        ),
        before
    );
    assert_eq!(
        qa_failure.task(qa.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert_eq!(
        qa_failure.active_child_task().map(|task| task.id()),
        Some(qa.task_id())
    );

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(4));
    let mut security_failure = AgentOrchestrator::new(runtime)?;
    let (_root, security) = advance_to_security(&mut security_failure)?;
    let before = (
        security_failure.task_count(),
        security_failure.run_count(),
        security_failure.events().len(),
        security_failure.runtime_event_count(),
        security_failure.engineering_quality_events().len(),
        security_failure.engineering_quality_audit_records().len(),
    );
    assert_eq!(
        security_failure.cancel_task(security.task_id()),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
        ))
    );
    assert_eq!(recorder.cancellations().len(), 1);
    assert_eq!(
        (
            security_failure.task_count(),
            security_failure.run_count(),
            security_failure.events().len(),
            security_failure.runtime_event_count(),
            security_failure.engineering_quality_events().len(),
            security_failure.engineering_quality_audit_records().len(),
        ),
        before
    );
    assert_eq!(
        security_failure
            .task(security.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert_eq!(
        security_failure.active_child_task().map(|task| task.id()),
        Some(security.task_id())
    );

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(5));
    let mut synthesis_failure = AgentOrchestrator::new(runtime)?;
    let (root, _synthesis) = advance_to_synthesis(&mut synthesis_failure)?;
    let before = (
        synthesis_failure.task_count(),
        synthesis_failure.run_count(),
        synthesis_failure.events().len(),
        synthesis_failure.runtime_event_count(),
        synthesis_failure.engineering_quality_events().len(),
        synthesis_failure.engineering_quality_audit_records().len(),
    );
    assert_eq!(
        synthesis_failure.cancel_task(root.task_id()),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
        ))
    );
    assert_eq!(recorder.cancellations().len(), 1);
    assert_eq!(
        (
            synthesis_failure.task_count(),
            synthesis_failure.run_count(),
            synthesis_failure.events().len(),
            synthesis_failure.runtime_event_count(),
            synthesis_failure.engineering_quality_events().len(),
            synthesis_failure.engineering_quality_audit_records().len(),
        ),
        before
    );
    assert_eq!(
        synthesis_failure
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );

    let mut terminal = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, synthesis) = advance_to_synthesis(&mut terminal)?;
    complete_stage(
        &mut terminal,
        &synthesis,
        "completed-before-terminal-cancel",
        SYNTHESIS,
    )?;
    let before = (
        terminal.events().len(),
        terminal.engineering_quality_events().len(),
        terminal.engineering_quality_audit_records().len(),
    );
    assert_eq!(
        terminal.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::AlreadyTerminal(AgentTaskStatus::Completed)
    );
    assert_eq!(
        (
            terminal.events().len(),
            terminal.engineering_quality_events().len(),
            terminal.engineering_quality_audit_records().len(),
        ),
        before
    );
    Ok(())
}

#[test]
fn unexpected_terminal_specialist_cancellation_never_strands_the_parent(
) -> Result<(), Box<dyn Error>> {
    let mut coding_terminal = AgentOrchestrator::new(MockAgentRuntime::new(
        MockMode::CancelAlreadyTerminalAt(2, RuntimeRunStatus::Completed),
    ))?;
    let (root, coding) = start_workflow(&mut coding_terminal)?;
    assert_eq!(
        coding_terminal.cancel_task(coding.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        coding_terminal
            .task(coding.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(coding_terminal.active_child_task().is_none());
    assert!(coding_terminal.current_context(root.task_id()).is_ok());
    assert!(!coding_terminal
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(event, EngineeringQualityWorkflowEvent::Cancelled { .. })));

    let mut qa_terminal = AgentOrchestrator::new(MockAgentRuntime::new(
        MockMode::CancelAlreadyTerminalAt(3, RuntimeRunStatus::Completed),
    ))?;
    let (_root, coding) = start_workflow(&mut qa_terminal)?;
    complete_stage(
        &mut qa_terminal,
        &coding,
        "coding-before-qa-terminal-cancel",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&qa_terminal)?;
    assert_eq!(
        qa_terminal.cancel_task(qa.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        qa_terminal.task(qa.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    let security = active_child_context(&qa_terminal)?;
    assert_eq!(security.agent_id(), AgentId::SecurityRisk);
    assert!(!qa_terminal
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(event, EngineeringQualityWorkflowEvent::Cancelled { .. })));

    let mut security_terminal = AgentOrchestrator::new(MockAgentRuntime::new(
        MockMode::CancelAlreadyTerminalAt(4, RuntimeRunStatus::Completed),
    ))?;
    let (root, security) = advance_to_security(&mut security_terminal)?;
    assert_eq!(
        security_terminal.cancel_task(security.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        security_terminal
            .task(security.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(security_terminal.active_child_task().is_none());
    assert!(security_terminal.current_context(root.task_id()).is_ok());
    assert!(!security_terminal
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(event, EngineeringQualityWorkflowEvent::Cancelled { .. })));

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelAlreadyTerminalAt(
        2,
        RuntimeRunStatus::Completed,
    ));
    let mut root_cancel = AgentOrchestrator::new(runtime)?;
    let (root, coding) = start_workflow(&mut root_cancel)?;
    assert_eq!(
        root_cancel.cancel_task(root.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(recorder.starts().len(), 2);
    assert_eq!(
        root_cancel.task(coding.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        root_cancel.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(root_cancel.active_child_task().is_none());
    assert!(root_cancel.engineering_quality_result().is_none());
    assert_eq!(
        root_cancel
            .engineering_quality_events()
            .iter()
            .filter(|event| matches!(event, EngineeringQualityWorkflowEvent::Cancelled { .. }))
            .count(),
        1
    );
    Ok(())
}

#[test]
fn specialist_governance_has_no_repository_executor_and_never_attempts_execution(
) -> Result<(), Box<dyn Error>> {
    let mut coding_orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, coding) = start_workflow(&mut coding_orchestrator)?;
    assert_specialist_governance_denied(&mut coding_orchestrator, &coding, "coding")?;
    assert_unregistered_repository_tools_rejected(&mut coding_orchestrator, &coding, "coding")?;

    let mut qa_orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, coding) = start_workflow(&mut qa_orchestrator)?;
    complete_stage(
        &mut qa_orchestrator,
        &coding,
        "coding-before-qa-governance",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&qa_orchestrator)?;
    assert_specialist_governance_denied(&mut qa_orchestrator, &qa, "qa")?;
    assert_unregistered_repository_tools_rejected(&mut qa_orchestrator, &qa, "qa")?;

    let mut security_orchestrator =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, security) = advance_to_security(&mut security_orchestrator)?;
    assert_specialist_governance_denied(&mut security_orchestrator, &security, "security")?;
    assert_unregistered_repository_tools_rejected(
        &mut security_orchestrator,
        &security,
        "security",
    )?;
    assert!(security_orchestrator
        .pending_governance_approval(security.task_id())?
        .is_none());
    assert_eq!(
        security_orchestrator
            .task(security.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert_eq!(security_orchestrator.engineering_quality_events().len(), 5);
    Ok(())
}

#[test]
fn engineering_generic_document_and_research_selectors_exclude_each_other_in_both_directions(
) -> Result<(), Box<dyn Error>> {
    let mut engineering_to_generic =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, coding) = start_workflow(&mut engineering_to_generic)?;
    let before = observable_workflow_state(&engineering_to_generic);
    assert_eq!(
        engineering_to_generic.request_delegation(&coding, research_delegation()?),
        Err(AgentOrchestratorError::UnauthorizedSource {
            agent_id: AgentId::Coding,
        })
    );
    assert_eq!(observable_workflow_state(&engineering_to_generic), before);

    let mut generic_to_engineering =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = generic_to_engineering.start_root(OBJECTIVE)?;
    let research = generic_to_engineering
        .request_delegation(&root, research_delegation()?)?
        .child_context()
        .clone();
    assert_eq!(
        generic_to_engineering.selected_workflow(),
        Some(AgentWorkflowSelection::GenericDelegation)
    );
    let before = observable_workflow_state(&generic_to_engineering);
    assert_eq!(
        generic_to_engineering.start_engineering_quality_workflow(&research, fixture_request()?),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::GenericDelegation,
        })
    );
    assert_eq!(observable_workflow_state(&generic_to_engineering), before);

    let mut engineering_to_research =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, coding) = start_workflow(&mut engineering_to_research)?;
    let before = observable_workflow_state(&engineering_to_research);
    assert_eq!(
        engineering_to_research
            .request_research_knowledge_workflow(&coding, research_workflow_request()?),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::EngineeringQuality,
        })
    );
    assert_eq!(observable_workflow_state(&engineering_to_research), before);

    let mut research_to_engineering =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = research_to_engineering.start_root(OBJECTIVE)?;
    let research = research_to_engineering
        .request_research_knowledge_workflow(&root, research_workflow_request()?)?
        .context()
        .clone();
    assert_eq!(
        research_to_engineering.selected_workflow(),
        Some(AgentWorkflowSelection::ResearchKnowledge)
    );
    let before = observable_workflow_state(&research_to_engineering);
    assert_eq!(
        research_to_engineering.start_engineering_quality_workflow(&research, fixture_request()?),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::ResearchKnowledge,
        })
    );
    assert_eq!(observable_workflow_state(&research_to_engineering), before);

    let directory = tempdir()?;
    let engineering_document_path = directory.path().join("engineering-selector.txt");
    fs::write(&engineering_document_path, "bounded selector fixture")?;
    let mut engineering_to_document =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = engineering_to_document.start_root(OBJECTIVE)?;
    let document_id = engineering_to_document.register_approved_document(
        &root,
        ApprovedDocumentSource::UserSelectedFile,
        &engineering_document_path,
    )?;
    let acceptance =
        engineering_to_document.start_engineering_quality_workflow(&root, fixture_request()?)?;
    let coding = acceptance.context().clone();
    let before = observable_workflow_state(&engineering_to_document);
    assert_eq!(
        engineering_to_document.request_document_task(
            &coding,
            &document_id,
            DocumentOperation::Read,
            None,
        ),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::EngineeringQuality,
        })
    );
    assert_eq!(observable_workflow_state(&engineering_to_document), before);

    let document_engineering_path = directory.path().join("document-selector.txt");
    fs::write(&document_engineering_path, "bounded selector fixture")?;
    let mut document_to_engineering =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = document_to_engineering.start_root(OBJECTIVE)?;
    let document_id = document_to_engineering.register_approved_document(
        &root,
        ApprovedDocumentSource::UserSelectedFile,
        &document_engineering_path,
    )?;
    let knowledge = document_to_engineering.request_document_task(
        &root,
        &document_id,
        DocumentOperation::Read,
        None,
    )?;
    assert_eq!(
        document_to_engineering.selected_workflow(),
        Some(AgentWorkflowSelection::ApprovedDocument)
    );
    let before = observable_workflow_state(&document_to_engineering);
    assert_eq!(
        document_to_engineering.start_engineering_quality_workflow(&knowledge, fixture_request()?),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::ApprovedDocument,
        })
    );
    assert_eq!(observable_workflow_state(&document_to_engineering), before);
    Ok(())
}

#[test]
fn runtime_identity_sequence_and_terminal_reserve_fail_before_capacity_mutation(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, coding) = start_workflow(&mut orchestrator)?;
    let foreign = RuntimeTurnRequest::new("foreign-run", "foreign-request", "bounded")?;
    let foreign_event = RuntimeEventEnvelope::for_request(
        &foreign,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("foreign-response")?,
        },
    );
    assert!(orchestrator
        .accept_runtime_event(coding.task_id(), foreign_event)
        .is_err());
    assert_eq!(orchestrator.runtime_event_count(), 0);

    orchestrator.accept_runtime_event(
        coding.task_id(),
        started(&coding, "bounded-coding-response")?,
    )?;
    assert!(orchestrator
        .accept_runtime_event(coding.task_id(), delta(&coding, 2, "out-of-order")?)
        .is_err());
    assert_eq!(orchestrator.runtime_event_count(), 1);

    for sequence in 1..MAX_ENGINEERING_QUALITY_EVENTS_PER_RUN - 1 {
        orchestrator.accept_runtime_event(coding.task_id(), delta(&coding, sequence, "x")?)?;
    }
    assert_eq!(
        orchestrator.runtime_event_count(),
        usize::try_from(MAX_ENGINEERING_QUALITY_EVENTS_PER_RUN - 1)?
    );
    assert_eq!(
        orchestrator.accept_runtime_event(
            coding.task_id(),
            delta(
                &coding,
                MAX_ENGINEERING_QUALITY_EVENTS_PER_RUN - 1,
                "seventh-delta",
            )?,
        ),
        Err(AgentOrchestratorError::EngineeringEventLimitExceeded)
    );
    assert_eq!(
        orchestrator.runtime_event_count(),
        usize::try_from(MAX_ENGINEERING_QUALITY_EVENTS_PER_RUN - 1)?
    );
    assert_eq!(
        orchestrator.accept_runtime_event(
            coding.task_id(),
            completed(&coding, MAX_ENGINEERING_QUALITY_EVENTS_PER_RUN - 1),
        )?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(
        orchestrator.runtime_event_count(),
        usize::try_from(MAX_ENGINEERING_QUALITY_EVENTS_PER_RUN)?
    );
    assert!(orchestrator.runtime_event_count() <= MAX_RUNTIME_EVENTS_PER_ROOT);
    assert!(orchestrator
        .engineering_quality_events()
        .iter()
        .any(|event| matches!(
            event,
            EngineeringQualityWorkflowEvent::PartialFailure {
                stage: EngineeringQualityStage::Coding,
                code: EngineeringPartialFailureCode::InvalidStructuredOutput,
            }
        )));
    Ok(())
}

#[test]
fn published_predecessor_caps_make_a_24577_byte_stage_input_unconstructible(
) -> Result<(), Box<dyn Error>> {
    let coding_framing = format!(
        "engineering-quality-v1\nstage=coding\nfixture_based=true\nproposal_only=true\nobjective(untrusted):\n{}\nfixtures(untrusted):\n{}\ncriteria(untrusted):\n{}\napplication_evidence(untrusted):\n{}\nReturn one strict ChangeProposalV1 JSON object.",
        "", "", "", "",
    )
    .len();
    let qa_framing = format!(
        "engineering-quality-v1\nstage=qa-validation\nfixture_based=true\nadvisory_only=true\nobjective(untrusted):\n{}\ncriteria(untrusted):\n{}\napplication_evidence(untrusted):\n{}\nvalidated_change_proposal(untrusted):\n{}\nReturn one strict ValidationReportV1 JSON object. No check was executed.",
        "", "", "", "",
    )
    .len();
    let security_framing = format!(
        "engineering-quality-v1\nstage=security-risk\nfixture_based=true\nadvisory_only=true\napplication_evidence(untrusted):\n{}\nvalidated_change_proposal(untrusted):\n{}\nqa_outcome(untrusted):\n{}\nReturn one strict RiskAssessmentV1 JSON object.",
        "", "", "",
    )
    .len();
    let synthesis_framing = format!(
        "engineering-quality-v1\nstage=personal-synthesis\nfixture_based=true\nproposal_only=true\nchanges_applied=false\ntests_executed=false\ncoding_outcome(untrusted):\n{}\nqa_outcome(untrusted):\n{}\nsecurity_outcome(untrusted):\n{}\nReturn one strict EngineeringReviewSynthesisV1 JSON object.",
        "", "", "",
    )
    .len();

    let maximum_projections = [
        coding_framing
            + MAX_ENGINEERING_OBJECTIVE_BYTES
            + MAX_ENGINEERING_FIXTURE_CATALOG_BYTES
            + MAX_ENGINEERING_CRITERIA_CATALOG_BYTES
            + MAX_ENGINEERING_EVIDENCE_CATALOG_BYTES,
        qa_framing
            + MAX_ENGINEERING_OBJECTIVE_BYTES
            + MAX_ENGINEERING_CRITERIA_CATALOG_BYTES
            + MAX_ENGINEERING_EVIDENCE_CATALOG_BYTES
            + MAX_CHANGE_PROPOSAL_BYTES,
        security_framing
            + MAX_ENGINEERING_EVIDENCE_CATALOG_BYTES
            + MAX_CHANGE_PROPOSAL_BYTES
            + MAX_VALIDATION_REPORT_BYTES,
        synthesis_framing
            + MAX_CHANGE_PROPOSAL_BYTES
            + MAX_VALIDATION_REPORT_BYTES
            + MAX_RISK_ASSESSMENT_BYTES,
    ];
    let first_rejected_size = MAX_ENGINEERING_STAGE_INPUT_BYTES
        .checked_add(1)
        .ok_or("engineering input bound overflow")?;
    assert_eq!(first_rejected_size, 24_577);
    assert!(maximum_projections
        .iter()
        .all(|projection| *projection <= MAX_ENGINEERING_STAGE_INPUT_BYTES));
    assert!(maximum_projections
        .iter()
        .all(|projection| *projection < first_rejected_size));

    let request = fixture_request()?;
    let proposal = proposal(&request)?;
    let qa = QaStageOutcome::Completed(validation(&request, &proposal)?);
    let security = SecurityStageOutcome::Completed(risk(&request, &proposal, &qa)?);
    let coding = CodingStageOutcome::Completed(Box::new(proposal));
    for input in [
        request.build_coding_input()?,
        request.build_qa_input(match &coding {
            CodingStageOutcome::Completed(proposal) => proposal,
            CodingStageOutcome::Failed(_) | CodingStageOutcome::Cancelled => {
                return Err("validated Coding proposal unexpectedly unavailable".into());
            }
        })?,
        request.build_security_input(
            match &coding {
                CodingStageOutcome::Completed(proposal) => proposal,
                CodingStageOutcome::Failed(_) | CodingStageOutcome::Cancelled => {
                    return Err("validated Coding proposal unexpectedly unavailable".into());
                }
            },
            &qa,
        )?,
        request.build_synthesis_input(&coding, &qa, &security)?,
    ] {
        assert!(input.len() <= MAX_ENGINEERING_STAGE_INPUT_BYTES);
    }
    Ok(())
}

#[test]
fn native_runtime_serializes_maximum_engineering_input_without_tool_or_network_authority(
) -> Result<(), Box<dyn Error>> {
    let adversarial_unit = "\"\\\n\t🙂";
    assert_eq!(adversarial_unit.len(), 8);
    let selected_text =
        adversarial_unit.repeat(MAX_ENGINEERING_STAGE_INPUT_BYTES / adversarial_unit.len());
    assert_eq!(selected_text.len(), MAX_ENGINEERING_STAGE_INPUT_BYTES);
    let request = RuntimeTurnRequest::new(
        "r".repeat(MAX_OPAQUE_ID_BYTES),
        "q".repeat(MAX_OPAQUE_ID_BYTES),
        selected_text,
    )?;
    let runtime = NativeAgentRuntime;
    let descriptor = runtime.describe();
    assert_eq!(descriptor.id(), RuntimeId::Native);
    assert!(descriptor
        .capabilities()
        .supports(RuntimeCapability::StreamingText));
    assert!(!descriptor
        .capabilities()
        .supports(RuntimeCapability::UntrustedToolProposals));
    let run = runtime.start(request)?;
    assert!(run.request_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);

    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root(OBJECTIVE)?;
    let acceptance = orchestrator.start_engineering_quality_workflow(&root, fixture_request()?)?;
    assert_eq!(acceptance.context().runtime_id(), RuntimeId::Native);
    assert_eq!(acceptance.context().agent_id(), AgentId::Coding);
    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    Ok(())
}

#[test]
fn four_event_consuming_runs_reach_the_exact_global_runtime_event_bound(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, coding) = start_workflow(&mut orchestrator)?;
    complete_stage_at_event_cap(
        &mut orchestrator,
        &coding,
        "coding-at-event-cap",
        CHANGE_PROPOSAL,
    )?;
    let qa = active_child_context(&orchestrator)?;
    complete_stage_at_event_cap(&mut orchestrator, &qa, "qa-at-event-cap", VALIDATION_REPORT)?;
    let security = active_child_context(&orchestrator)?;
    complete_stage_at_event_cap(
        &mut orchestrator,
        &security,
        "security-at-event-cap",
        RISK_ASSESSMENT,
    )?;
    let root_id = coding.root_task_id().task_id().clone();
    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage_at_event_cap(
        &mut orchestrator,
        &synthesis,
        "synthesis-at-event-cap",
        SYNTHESIS,
    )?;
    assert_eq!(
        orchestrator.runtime_event_count(),
        MAX_RUNTIME_EVENTS_PER_ROOT
    );
    assert!(orchestrator.engineering_quality_result().is_some());
    Ok(())
}
