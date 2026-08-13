use std::{collections::BTreeSet, error::Error, fs};

use ai_agent_assistant_lib::agent::definition::AgentId;
use ai_agent_assistant_lib::agent::engineering_quality::{
    AcceptanceCriterion, ApplicationValidationEvidenceCatalog, EngineeringQualityWorkflowRequest,
    FixtureRepositoryCatalog, FixtureRepositoryFile,
};
use ai_agent_assistant_lib::agent::gateway_protocol::{
    MAX_GATEWAY_REQUEST_BYTES, MAX_OPAQUE_ID_BYTES,
};
use ai_agent_assistant_lib::agent::infrastructure_operations::{
    CloudFixtureKind, CloudInfrastructureCapability, CloudInfrastructureStageOutcome,
    CloudInfrastructureWorkflowEvent, CloudScenarioId, InfrastructureOperationsApprovalRequirement,
    InfrastructureOperationsAuditOutcome, InfrastructureOperationsCapabilityAuditDisposition,
    InfrastructureOperationsCapabilityDisposition, InfrastructureOperationsContinuationFailure,
    InfrastructureOperationsError, InfrastructureOperationsEvidenceStatus,
    InfrastructureOperationsExecutionDisposition, InfrastructureOperationsFixtureCatalog,
    InfrastructureOperationsFixtureKind, InfrastructureOperationsId,
    InfrastructureOperationsPartialFailureCode, InfrastructureOperationsQaStageOutcome,
    InfrastructureOperationsReviewStatus, InfrastructureOperationsSecurityStageOutcome,
    InfrastructureOperationsStage, InfrastructureOperationsStageDisposition,
    InfrastructureOperationsSupportingEvidenceStatus, InfrastructureOperationsValidationConclusion,
    SystemsFixtureKind, SystemsOperationsCapability, SystemsOperationsScenarioId,
    SystemsOperationsStageOutcome, SystemsOperationsWorkflowEvent,
    MAX_INFRASTRUCTURE_OPERATIONS_CRITERIA, MAX_INFRASTRUCTURE_OPERATIONS_EVIDENCE,
    MAX_INFRASTRUCTURE_OPERATIONS_FIXTURES, MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_BYTES,
    MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_CHARACTERS,
    MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_TOTAL_BYTES, MAX_INFRASTRUCTURE_OPERATIONS_ID_BYTES,
    MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_BYTES,
    MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_CHARACTERS, MAX_INFRASTRUCTURE_OPERATIONS_RESULT_BYTES,
    MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES, MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS,
};
use ai_agent_assistant_lib::agent::native_runtime::NativeAgentRuntime;
use ai_agent_assistant_lib::agent::orchestrator::{
    AgentOrchestrator, AgentOrchestratorError, AgentWorkflowSelection,
    CloudInfrastructureWorkflowAcceptance, DelegationProposal, SystemsOperationsWorkflowAcceptance,
    MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT,
    MAX_INFRASTRUCTURE_OPERATIONS_TASKS_PER_ROOT, MAX_RUNTIME_EVENTS_PER_ROOT,
};
use ai_agent_assistant_lib::agent::research_knowledge::{
    ResearchKnowledgeWorkflowRequest, WorkflowSource, WorkflowSourceCatalog,
};
use ai_agent_assistant_lib::agent::runtime::{
    AgentRuntime, RuntimeBoundaryStage, RuntimeCapability, RuntimeError, RuntimeEventAcceptance,
    RuntimeEventEnvelope, RuntimeEventRejection, RuntimeFailure, RuntimeFailureCode, RuntimeId,
    RuntimeOutputText, RuntimeResponseId, RuntimeRunStatus, RuntimeTurnRequest,
    UntrustedRuntimeEvent,
};
use ai_agent_assistant_lib::agent::task::{
    AgentExecutionContext, AgentTaskCancellationOutcome, AgentTaskStatus,
};
use ai_agent_assistant_lib::documents::{ApprovedDocumentSource, DocumentOperation};
use tempfile::tempdir;

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

const CLOUD_ASSESSMENT: &str = r#"{"version":"v1","objective_disposition":"supported-by-fixtures","fixture_ids":["terraform-configuration","azure-architecture"],"findings":[{"id":"public-ingress","category":"terraform-static-review","statement":"The synthetic configuration exposes public ingress for fixture review.","confidence":"high","references":[{"namespace":"fixture","id":"terraform-configuration"}]}],"limitations":["Only synthetic fixture text was reviewed."],"unresolved_questions":["Confirm the intended exposure in a separately authorized environment."],"change_plan":{"objective":"Prepare an inert fixture-only infrastructure hardening proposal.","affected_fixture_ids":["terraform-configuration","azure-architecture"],"proposed_changes":["Propose restricting public ingress after separate review."],"risks":["A future restriction could affect intended connectivity."],"proposed_validation":[{"id":"validate-network-later","text":"Propose a separately authorized validation of intended network access."}],"rollback_considerations":"Retain the immutable fixture and discard the proposal.","capabilities":["review-terraform-fixture","analyze-azure-fixture","propose-infrastructure-change-plan"]},"fixture_based":true,"live_inventory_performed":false,"commands_executed":false,"credentials_loaded":false,"effects_performed":false,"terraform_fmt_check":"not-run","terraform_validate":"not-run","provider_initialization":"not-run","terraform_plan":"not-run","terraform_apply":"not-run"}"#;

const SYSTEMS_ASSESSMENT: &str = r#"{"version":"v1","fixture_ids":["service-snapshot","sanitized-log","recovery-scenario"],"findings":[{"id":"dependency-timeout","category":"log-signal","statement":"The sanitized fixture log records a dependency timeout.","confidence":"high","basis":"evidence-bound","references":[{"namespace":"fixture","id":"sanitized-log"}]},{"id":"dependency-recovery-hypothesis","category":"backup-recovery","statement":"A dependency recovery issue is a bounded hypothesis requiring separate validation.","confidence":"medium","basis":"hypothesis","references":[]}],"limitations":["No live service, process, host, or log was inspected."],"unresolved_questions":["The live dependency state is unavailable."],"diagnostic_plan":["Propose reviewing dependency health in a separately authorized environment."],"remediation_plan":["Propose restoring dependency availability before considering service recovery."],"rollback_considerations":"Discard the inert proposal; no system state changed.","capabilities":["analyze-service-fixture","analyze-log-fixture","propose-diagnostics","propose-remediation-plan"],"fixture_based":true,"live_diagnostics_performed":false,"commands_executed":false,"credentials_loaded":false,"effects_performed":false}"#;

const CLOUD_QA: &str = r#"{"version":"v1","coverage":[{"criterion_id":"criterion-static-review","disposition":"demonstrated","references":[{"namespace":"application-evidence","id":"evidence-static-observation"}]},{"criterion_id":"criterion-no-execution","disposition":"not-demonstrated","references":[{"namespace":"application-evidence","id":"evidence-terraform-not-run"}]}],"findings":[{"id":"qa-static-only","statement":"The finding is supported only by the supplied synthetic fixtures.","references":[{"namespace":"assessment-finding","id":"public-ingress"}]}],"proposed_checks":[{"id":"qa-terraform-check","text":"Terraform validation remains proposed and not run.","references":[{"namespace":"proposed-validation","id":"validate-network-later"}],"status":"not-run"}],"gaps":["No Terraform or provider check was executed."],"conclusion":"incomplete","advisory_only":true,"approval_authority":false,"evidence_executed":false}"#;

const SYSTEMS_QA: &str = r#"{"version":"v1","coverage":[{"criterion_id":"criterion-diagnostic","disposition":"demonstrated","references":[{"namespace":"application-evidence","id":"evidence-service-log-observation"}]},{"criterion_id":"criterion-safe-remediation","disposition":"not-demonstrated","references":[]}],"findings":[{"id":"qa-sanitized-only","statement":"The diagnostic finding is bounded to sanitized fixtures.","references":[{"namespace":"assessment-finding","id":"dependency-timeout"}]}],"proposed_checks":[{"id":"qa-service-check","text":"A live service check remains proposed and not run.","references":[],"status":"not-run"}],"gaps":["No live diagnostic or remediation was executed."],"conclusion":"incomplete","advisory_only":true,"approval_authority":false,"evidence_executed":false}"#;

const RISK_ASSESSMENT: &str = r#"{"version":"v1","findings":[{"id":"authorization-boundary","category":"authorization-boundary","severity":"medium","confidence":"high","statement":"The proposal must not become execution authority.","basis":"evidence-bound","references":[{"namespace":"qa-finding","id":"qa-static-only"}]}],"unresolved_risks":["Live dependency and platform evidence remains unavailable."],"follow_ups":["Seek separate authorization before any consequential action."],"dependency_evidence":"unavailable","provider_evidence":"unavailable","credential_evidence":"unavailable","platform_evidence":"unavailable","audit_evidence":"unavailable","rollback_evidence":"unavailable","advisory_only":true,"authorization_granted":false,"remediation_executed":false,"credentials_loaded":false}"#;

const RISK_WITHOUT_QA: &str = r#"{"version":"v1","findings":[{"id":"authorization-boundary","category":"authorization-boundary","severity":"medium","confidence":"high","statement":"The proposal must not become execution authority when QA is unavailable.","basis":"evidence-bound","references":[{"namespace":"assessment-finding","id":"public-ingress"}]}],"unresolved_risks":["QA, live dependency, and platform evidence remain unavailable."],"follow_ups":["Repeat QA before any separately authorized consequential action."],"dependency_evidence":"unavailable","provider_evidence":"unavailable","credential_evidence":"unavailable","platform_evidence":"unavailable","audit_evidence":"unavailable","rollback_evidence":"unavailable","advisory_only":true,"authorization_granted":false,"remediation_executed":false,"credentials_loaded":false}"#;

const SYSTEMS_RISK_ASSESSMENT: &str = r#"{"version":"v1","findings":[{"id":"authorization-boundary","category":"authorization-boundary","severity":"medium","confidence":"high","statement":"The systems proposal must not become execution authority.","basis":"evidence-bound","references":[{"namespace":"qa-finding","id":"qa-sanitized-only"}]}],"unresolved_risks":["Live dependency and platform evidence remains unavailable."],"follow_ups":["Seek separate authorization before any consequential action."],"dependency_evidence":"unavailable","provider_evidence":"unavailable","credential_evidence":"unavailable","platform_evidence":"unavailable","audit_evidence":"unavailable","rollback_evidence":"unavailable","advisory_only":true,"authorization_granted":false,"remediation_executed":false,"credentials_loaded":false}"#;

const CLOUD_SYNTHESIS: &str = r#"{"version":"v1","summary":"The Cloud review remains fixture-only and incomplete; no Terraform check ran and no infrastructure state changed.","fixture_ids":["terraform-configuration","azure-architecture"],"unresolved_issues":["Terraform and provider validation remain not run."],"status":"partial","fixture_based":true,"live_inventory_performed":false,"commands_executed":false,"credentials_loaded":false,"effects_performed":false,"terraform_fmt_check":"not-run","terraform_validate":"not-run","provider_initialization":"not-run","terraform_plan":"not-run","terraform_apply":"not-run"}"#;

const SYSTEMS_SYNTHESIS: &str = r#"{"version":"v1","summary":"The Systems review remains fixture-only and incomplete; no live check ran and no system state changed.","fixture_ids":["service-snapshot","sanitized-log","recovery-scenario"],"unresolved_issues":["Live service and dependency validation remain unavailable."],"status":"partial","fixture_based":true,"live_inventory_performed":false,"commands_executed":false,"credentials_loaded":false,"effects_performed":false}"#;

const MODEL_TEXT_BYTE_LIMIT: usize = 2_048;

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
    text: &str,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    delta_at(context, 1, text)
}

fn delta_at(
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

fn completed(context: &AgentExecutionContext) -> RuntimeEventEnvelope {
    completed_at(context, 2)
}

fn completed_at(context: &AgentExecutionContext, sequence: u32) -> RuntimeEventEnvelope {
    RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        sequence,
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

fn six_ascii_chunks(value: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    if !value.is_ascii() || value.len() < 6 {
        return Err("eight-event fixture must be nonempty ASCII".into());
    }
    let mut chunks = Vec::with_capacity(6);
    let mut start = 0;
    for remaining_chunks in (1..6).rev() {
        let remaining_bytes = value.len() - start;
        let chunk_len = remaining_bytes / (remaining_chunks + 1);
        let end = start + chunk_len;
        chunks.push(&value[start..end]);
        start = end;
    }
    chunks.push(&value[start..]);
    Ok(chunks)
}

fn complete_stage_in_eight_events(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    response_id: &str,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    let task_id = context.task_id().clone();
    orchestrator.accept_runtime_event(&task_id, started(context, response_id)?)?;
    for (index, chunk) in six_ascii_chunks(output)?.into_iter().enumerate() {
        orchestrator.accept_runtime_event(
            &task_id,
            delta_at(context, u32::try_from(index)? + 1, chunk)?,
        )?;
    }
    assert_eq!(
        orchestrator.accept_runtime_event(&task_id, completed_at(context, 7))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    Ok(())
}

fn fail_stage(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    response_id: &str,
) -> Result<(), Box<dyn Error>> {
    let task_id = context.task_id().clone();
    orchestrator.accept_runtime_event(&task_id, started(context, response_id)?)?;
    assert!(matches!(
        orchestrator.accept_runtime_event(&task_id, failed(context)?)?,
        RuntimeEventAcceptance::ResponseFailed { .. }
    ));
    Ok(())
}

fn active_child_context(
    orchestrator: &AgentOrchestrator<MockAgentRuntime>,
) -> Result<AgentExecutionContext, Box<dyn Error>> {
    let task = orchestrator
        .active_child_task()
        .ok_or("missing active infrastructure/operations child")?;
    Ok(orchestrator.current_context(task.id())?)
}

fn start_cloud(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let request = InfrastructureOperationsFixtureCatalog::built_in()
        .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let acceptance = orchestrator.start_cloud_infrastructure_workflow(&root, request)?;
    let assessment = match acceptance {
        CloudInfrastructureWorkflowAcceptance::CloudAssessmentStarted { context } => context,
        CloudInfrastructureWorkflowAcceptance::PersonalFallbackStarted { .. } => {
            return Err("Cloud assessment unexpectedly failed to start".into());
        }
    };
    Ok((root, assessment))
}

fn start_systems(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let request = InfrastructureOperationsFixtureCatalog::built_in()
        .systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let acceptance = orchestrator.start_systems_operations_workflow(&root, request)?;
    let assessment = match acceptance {
        SystemsOperationsWorkflowAcceptance::SystemsAssessmentStarted { context } => context,
        SystemsOperationsWorkflowAcceptance::PersonalFallbackStarted { .. } => {
            return Err("Systems assessment unexpectedly failed to start".into());
        }
    };
    Ok((root, assessment))
}

fn advance_cloud_to_qa(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let (root, cloud) = start_cloud(orchestrator)?;
    complete_stage(orchestrator, &cloud, "cloud-before-qa", CLOUD_ASSESSMENT)?;
    Ok((root, active_child_context(orchestrator)?))
}

fn advance_cloud_to_security(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let (root, qa) = advance_cloud_to_qa(orchestrator)?;
    complete_stage(orchestrator, &qa, "qa-before-security", CLOUD_QA)?;
    Ok((root, active_child_context(orchestrator)?))
}

fn advance_cloud_to_synthesis(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn Error>> {
    let (root, security) = advance_cloud_to_security(orchestrator)?;
    complete_stage(
        orchestrator,
        &security,
        "security-before-synthesis",
        RISK_ASSESSMENT,
    )?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    Ok((root, synthesis))
}

fn selector_research_request() -> Result<ResearchKnowledgeWorkflowRequest, Box<dyn Error>> {
    Ok(ResearchKnowledgeWorkflowRequest::new(
        "Review one deterministic selector fixture",
        WorkflowSourceCatalog::new(vec![WorkflowSource::deterministic_fixture(
            "selector-source",
            "Selector fixture",
            "Bounded fixture evidence for selector exclusivity.",
        )?])?,
    )?)
}

fn selector_engineering_request() -> Result<EngineeringQualityWorkflowRequest, Box<dyn Error>> {
    Ok(EngineeringQualityWorkflowRequest::new(
        "Review one deterministic selector fixture",
        FixtureRepositoryCatalog::new(vec![FixtureRepositoryFile::new(
            "selector-file",
            "selector.txt",
            "bounded selector fixture",
        )?])?,
        vec![AcceptanceCriterion::new(
            "selector-criterion",
            "Keep selector state mutually exclusive.",
        )?],
        ApplicationValidationEvidenceCatalog::new(vec![])?,
    )?)
}

fn selector_research_delegation() -> Result<DelegationProposal, Box<dyn Error>> {
    Ok(DelegationProposal::new(
        AgentId::Research,
        "Review one deterministic selector fixture",
        None,
        "Return one bounded fixture result",
    )?)
}

#[test]
fn built_in_scenarios_are_distinct_deterministic_and_debug_redacted() -> Result<(), Box<dyn Error>>
{
    let catalog = InfrastructureOperationsFixtureCatalog::built_in();
    let cloud = catalog.cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let cloud_again = catalog.cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let systems =
        catalog.systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;

    assert_eq!(cloud, cloud_again);
    assert_eq!(
        cloud.scenario_id(),
        CloudScenarioId::TerraformDecisionBriefV1
    );
    assert_eq!(
        systems.scenario_id(),
        SystemsOperationsScenarioId::SanitizedServiceRecoveryV1
    );
    assert_ne!(cloud.objective(), systems.objective());
    assert!(!cloud.fixtures().is_empty());
    assert!(!systems.fixtures().is_empty());
    assert_eq!(
        cloud
            .fixtures()
            .iter()
            .map(|fixture| fixture.kind())
            .collect::<Vec<_>>(),
        [
            InfrastructureOperationsFixtureKind::Cloud(CloudFixtureKind::TerraformConfiguration),
            InfrastructureOperationsFixtureKind::Cloud(CloudFixtureKind::AzureArchitecture),
        ]
    );
    assert_eq!(
        systems
            .fixtures()
            .iter()
            .map(|fixture| fixture.kind())
            .collect::<Vec<_>>(),
        [
            InfrastructureOperationsFixtureKind::Systems(SystemsFixtureKind::ServiceSnapshot),
            InfrastructureOperationsFixtureKind::Systems(SystemsFixtureKind::SanitizedLogExcerpt),
            InfrastructureOperationsFixtureKind::Systems(SystemsFixtureKind::RecoveryScenario),
        ]
    );
    assert!(cloud.fixtures().iter().all(|fixture| matches!(
        fixture.kind(),
        InfrastructureOperationsFixtureKind::Cloud(
            CloudFixtureKind::TerraformConfiguration
                | CloudFixtureKind::AzureArchitecture
                | CloudFixtureKind::AwsArchitecture
                | CloudFixtureKind::ApprovedInventorySnapshot
        )
    )));
    assert!(systems.fixtures().iter().all(|fixture| matches!(
        fixture.kind(),
        InfrastructureOperationsFixtureKind::Systems(
            SystemsFixtureKind::WindowsSnapshot
                | SystemsFixtureKind::LinuxSnapshot
                | SystemsFixtureKind::MacosSnapshot
                | SystemsFixtureKind::ServiceSnapshot
                | SystemsFixtureKind::ProcessSnapshot
                | SystemsFixtureKind::SanitizedLogExcerpt
                | SystemsFixtureKind::ConfigurationSnapshot
                | SystemsFixtureKind::ResourceSnapshot
                | SystemsFixtureKind::VmwareInventorySnapshot
                | SystemsFixtureKind::BackupSnapshot
                | SystemsFixtureKind::RecoveryScenario
        )
    )));
    assert!(cloud
        .evidence()
        .iter()
        .any(|entry| entry.status() == InfrastructureOperationsEvidenceStatus::NotRun));
    assert!(systems.evidence().iter().all(|entry| {
        entry.status() == InfrastructureOperationsEvidenceStatus::ObservedFixture
            && !entry.fixture_ids().is_empty()
    }));

    for request_debug in [format!("{cloud:?}"), format!("{systems:?}")] {
        assert!(request_debug.contains("[REDACTED]"));
        assert!(!request_debug.contains(cloud.objective()));
        assert!(!request_debug.contains(systems.objective()));
        for fixture in cloud.fixtures().iter().chain(systems.fixtures()) {
            assert!(!request_debug.contains(fixture.label()));
            assert!(!request_debug.contains(fixture.content()));
        }
    }
    Ok(())
}

#[test]
fn scenario_and_capability_selectors_are_closed_and_cross_kind_values_fail(
) -> Result<(), Box<dyn Error>> {
    let cloud_scenario = serde_json::to_string(&CloudScenarioId::TerraformDecisionBriefV1)?;
    let systems_scenario =
        serde_json::to_string(&SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;
    assert_eq!(cloud_scenario, r#""terraform-decision-brief-v1""#);
    assert_eq!(systems_scenario, r#""sanitized-service-recovery-v1""#);
    assert_eq!(
        serde_json::from_str::<CloudScenarioId>(&cloud_scenario)?,
        CloudScenarioId::TerraformDecisionBriefV1
    );
    assert_eq!(
        serde_json::from_str::<SystemsOperationsScenarioId>(&systems_scenario)?,
        SystemsOperationsScenarioId::SanitizedServiceRecoveryV1
    );

    assert!(serde_json::from_str::<CloudScenarioId>(&systems_scenario).is_err());
    assert!(serde_json::from_str::<SystemsOperationsScenarioId>(&cloud_scenario).is_err());
    assert!(serde_json::from_str::<CloudScenarioId>(r#""caller-supplied-scenario""#).is_err());
    assert!(
        serde_json::from_str::<CloudInfrastructureCapability>(r#""restart-or-stop-service""#)
            .is_err()
    );
    assert!(serde_json::from_str::<SystemsOperationsCapability>(r#""terraform-apply""#).is_err());
    assert!(serde_json::from_str::<SystemsOperationsCapability>(r#""run-shell""#).is_err());
    Ok(())
}

#[test]
fn capability_catalog_is_closed_and_every_effect_request_is_denied_data() {
    let cloud_proposal_only = [
        CloudInfrastructureCapability::ReviewTerraformFixture,
        CloudInfrastructureCapability::AnalyzeAzureFixture,
        CloudInfrastructureCapability::AnalyzeAwsFixture,
        CloudInfrastructureCapability::AnalyzeInventoryFixture,
        CloudInfrastructureCapability::ProposeInfrastructureChangePlan,
    ];
    let cloud_denied = [
        CloudInfrastructureCapability::TerraformPlanExecution,
        CloudInfrastructureCapability::TerraformApply,
        CloudInfrastructureCapability::TerraformStateMutation,
        CloudInfrastructureCapability::TerraformBackendChange,
        CloudInfrastructureCapability::ProviderDownload,
        CloudInfrastructureCapability::CloudInventoryAccess,
        CloudInfrastructureCapability::CloudShell,
        CloudInfrastructureCapability::ResourceCreateOrUpdate,
        CloudInfrastructureCapability::ResourceDelete,
        CloudInfrastructureCapability::IamChange,
        CloudInfrastructureCapability::FirewallChange,
        CloudInfrastructureCapability::ProductionAccess,
        CloudInfrastructureCapability::CredentialAccess,
        CloudInfrastructureCapability::SecretRotation,
    ];
    let systems_proposal_only = [
        SystemsOperationsCapability::AnalyzeServiceFixture,
        SystemsOperationsCapability::AnalyzeProcessFixture,
        SystemsOperationsCapability::AnalyzeLogFixture,
        SystemsOperationsCapability::AnalyzeConfigurationFixture,
        SystemsOperationsCapability::AnalyzeResourceFixture,
        SystemsOperationsCapability::AnalyzeVmwareFixture,
        SystemsOperationsCapability::AnalyzeBackupFixture,
        SystemsOperationsCapability::ProposeDiagnostics,
        SystemsOperationsCapability::ProposeRemediationPlan,
    ];
    let systems_denied = [
        SystemsOperationsCapability::LiveDiagnostic,
        SystemsOperationsCapability::ReadLiveLog,
        SystemsOperationsCapability::InspectLiveService,
        SystemsOperationsCapability::InspectLiveProcess,
        SystemsOperationsCapability::InspectLiveConfiguration,
        SystemsOperationsCapability::RestartOrStopService,
        SystemsOperationsCapability::RebootOrShutdown,
        SystemsOperationsCapability::KillProcess,
        SystemsOperationsCapability::ConfigurationMutation,
        SystemsOperationsCapability::PackageInstall,
        SystemsOperationsCapability::PatchSystem,
        SystemsOperationsCapability::AccountOrPermissionChange,
        SystemsOperationsCapability::DeleteFile,
        SystemsOperationsCapability::PrivilegedShell,
        SystemsOperationsCapability::VmwareMutation,
        SystemsOperationsCapability::BackupMutation,
        SystemsOperationsCapability::CredentialAccess,
    ];

    assert_eq!(cloud_proposal_only.len(), 5);
    assert_eq!(cloud_denied.len(), 14);
    assert_eq!(systems_proposal_only.len(), 9);
    assert_eq!(systems_denied.len(), 17);
    assert!(cloud_proposal_only.into_iter().all(|capability| {
        capability.disposition() == InfrastructureOperationsCapabilityDisposition::ProposalOnly
    }));
    assert!(cloud_denied.into_iter().all(|capability| {
        capability.disposition() == InfrastructureOperationsCapabilityDisposition::Denied
    }));
    assert!(systems_proposal_only.into_iter().all(|capability| {
        capability.disposition() == InfrastructureOperationsCapabilityDisposition::ProposalOnly
    }));
    assert!(systems_denied.into_iter().all(|capability| {
        capability.disposition() == InfrastructureOperationsCapabilityDisposition::Denied
    }));
}

#[test]
fn built_in_catalog_is_bounded_reference_closed_and_cross_kind_disjoint(
) -> Result<(), Box<dyn Error>> {
    let catalog = InfrastructureOperationsFixtureCatalog::built_in();
    let cloud = catalog.cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let systems =
        catalog.systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;

    for (objective, fixtures, criteria, evidence) in [
        (
            cloud.objective(),
            cloud.fixtures(),
            cloud.criteria(),
            cloud.evidence(),
        ),
        (
            systems.objective(),
            systems.fixtures(),
            systems.criteria(),
            systems.evidence(),
        ),
    ] {
        assert!(objective.chars().count() <= MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_CHARACTERS);
        assert!(objective.len() <= MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_BYTES);
        assert!((1..=MAX_INFRASTRUCTURE_OPERATIONS_FIXTURES).contains(&fixtures.len()));
        assert!((1..=MAX_INFRASTRUCTURE_OPERATIONS_CRITERIA).contains(&criteria.len()));
        assert!(evidence.len() <= MAX_INFRASTRUCTURE_OPERATIONS_EVIDENCE);

        let fixture_ids: BTreeSet<_> = fixtures
            .iter()
            .map(|fixture| fixture.id().as_str())
            .collect();
        let criterion_ids: BTreeSet<_> = criteria
            .iter()
            .map(|criterion| criterion.id().as_str())
            .collect();
        let evidence_ids: BTreeSet<_> = evidence.iter().map(|entry| entry.id().as_str()).collect();
        assert_eq!(fixture_ids.len(), fixtures.len());
        assert_eq!(criterion_ids.len(), criteria.len());
        assert_eq!(evidence_ids.len(), evidence.len());

        let total_fixture_bytes = fixtures
            .iter()
            .map(|fixture| fixture.content().len())
            .sum::<usize>();
        assert!(total_fixture_bytes <= MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_TOTAL_BYTES);
        for fixture in fixtures {
            assert!(
                fixture.content().chars().count()
                    <= MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_CHARACTERS
            );
            assert!(fixture.content().len() <= MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_BYTES);
            let debug = format!("{fixture:?}");
            assert!(debug.contains("[REDACTED]"));
            assert!(!debug.contains(fixture.label()));
            assert!(!debug.contains(fixture.content()));
        }
        for criterion in criteria {
            let debug = format!("{criterion:?}");
            assert!(debug.contains("[REDACTED]"));
            assert!(!debug.contains(criterion.text()));
        }
        for entry in evidence {
            assert!(entry
                .criterion_ids()
                .iter()
                .all(|id| criterion_ids.contains(id.as_str())));
            assert!(entry
                .fixture_ids()
                .iter()
                .all(|id| fixture_ids.contains(id.as_str())));
            match entry.status() {
                InfrastructureOperationsEvidenceStatus::ObservedFixture => {
                    assert!(!entry.fixture_ids().is_empty());
                }
                InfrastructureOperationsEvidenceStatus::NotRun => {
                    assert!(entry.fixture_ids().is_empty());
                }
            }
            assert!(format!("{entry:?}").contains("[REDACTED]"));
        }
    }

    let cloud_fixture_ids: BTreeSet<_> = cloud
        .fixtures()
        .iter()
        .map(|fixture| fixture.id().as_str())
        .collect();
    let systems_fixture_ids: BTreeSet<_> = systems
        .fixtures()
        .iter()
        .map(|fixture| fixture.id().as_str())
        .collect();
    assert!(cloud_fixture_ids.is_disjoint(&systems_fixture_ids));
    assert!(cloud.fixtures().iter().all(|fixture| matches!(
        fixture.kind(),
        InfrastructureOperationsFixtureKind::Cloud(_)
    )));
    assert!(systems.fixtures().iter().all(|fixture| matches!(
        fixture.kind(),
        InfrastructureOperationsFixtureKind::Systems(_)
    )));
    Ok(())
}

#[test]
fn public_identifiers_are_closed_bounded_and_debug_redacted() -> Result<(), Box<dyn Error>> {
    let boundary_value = "a".repeat(MAX_INFRASTRUCTURE_OPERATIONS_ID_BYTES);
    let boundary_id = InfrastructureOperationsId::new(boundary_value.clone())?;
    assert_eq!(boundary_id.as_str(), boundary_value);
    assert!(!format!("{boundary_id:?}").contains(boundary_id.as_str()));
    assert!(format!("{boundary_id:?}").contains("[REDACTED]"));

    for invalid in [
        String::new(),
        "contains space".to_owned(),
        "contains/slash".to_owned(),
        "unicode-é".to_owned(),
        "control\ncharacter".to_owned(),
        "a".repeat(MAX_INFRASTRUCTURE_OPERATIONS_ID_BYTES + 1),
    ] {
        let Err(error) = InfrastructureOperationsId::new(invalid.clone()) else {
            return Err("invalid infrastructure/operations ID was accepted".into());
        };
        assert_eq!(error, InfrastructureOperationsError::InvalidIdentifier);
        if !invalid.is_empty() {
            assert!(!error.to_string().contains(&invalid));
        }
    }

    let permitted = InfrastructureOperationsId::new("AZaz09-_")?;
    assert_eq!(permitted.as_str(), "AZaz09-_");
    Ok(())
}

#[test]
fn deterministic_cloud_workflow_is_sequential_attributed_fixture_only_and_inert(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let governance_before = orchestrator.governance_audit_records().len();
    let (root, cloud) = start_cloud(&mut orchestrator)?;
    let root_id = root.task_id().clone();
    let cloud_id = cloud.task_id().clone();

    assert_eq!(cloud.agent_id(), AgentId::CloudInfrastructure);
    assert_eq!(cloud.runtime_id(), RuntimeId::Native);
    assert_eq!(cloud.depth(), 1);
    assert_eq!(cloud.root_task_id().task_id(), &root_id);
    assert_eq!(
        cloud.parent_task_id().map(|parent| parent.task_id()),
        Some(&root_id)
    );
    assert!(orchestrator
        .pending_governance_approval(cloud.task_id())?
        .is_none());
    complete_stage(
        &mut orchestrator,
        &cloud,
        "cloud-assessment-ok",
        CLOUD_ASSESSMENT,
    )?;

    let qa = active_child_context(&orchestrator)?;
    let qa_id = qa.task_id().clone();
    assert_eq!(qa.agent_id(), AgentId::QaValidation);
    assert!(orchestrator
        .pending_governance_approval(qa.task_id())?
        .is_none());
    complete_stage(&mut orchestrator, &qa, "cloud-qa-ok", CLOUD_QA)?;

    let security = active_child_context(&orchestrator)?;
    let security_id = security.task_id().clone();
    assert_eq!(security.agent_id(), AgentId::SecurityRisk);
    assert!(orchestrator
        .pending_governance_approval(security.task_id())?
        .is_none());
    complete_stage(
        &mut orchestrator,
        &security,
        "cloud-security-ok",
        RISK_ASSESSMENT,
    )?;

    let synthesis = orchestrator.current_context(&root_id)?;
    assert_eq!(synthesis.agent_id(), AgentId::PersonalAssistant);
    assert_eq!(synthesis.depth(), 0);
    assert!(orchestrator
        .pending_governance_approval(synthesis.task_id())?
        .is_none());
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "cloud-synthesis-ok",
        CLOUD_SYNTHESIS,
    )?;

    let result = orchestrator
        .cloud_infrastructure_result()
        .ok_or("missing Cloud workflow result")?;
    assert_eq!(result.root_task_id().task_id(), &root_id);
    let CloudInfrastructureStageOutcome::Completed(assessment) = result.assessment() else {
        return Err("Cloud assessment was not completed".into());
    };
    assert_eq!(assessment.task_id(), &cloud_id);
    assert_eq!(assessment.fixture_ids().len(), 2);
    assert_eq!(assessment.findings().len(), 1);
    assert!(assessment
        .change_plan()
        .capabilities()
        .iter()
        .all(|capability| capability.disposition()
            == InfrastructureOperationsCapabilityDisposition::ProposalOnly));
    for status in [
        assessment.terraform_execution().fmt_check(),
        assessment.terraform_execution().validate(),
        assessment.terraform_execution().provider_initialization(),
        assessment.terraform_execution().plan(),
        assessment.terraform_execution().apply(),
    ] {
        assert_eq!(status, InfrastructureOperationsEvidenceStatus::NotRun);
    }

    let InfrastructureOperationsQaStageOutcome::Completed(validation) = result.qa() else {
        return Err("Cloud QA was not completed".into());
    };
    assert_eq!(validation.task_id(), &qa_id);
    assert_eq!(validation.assessment_task_id(), &cloud_id);
    assert_eq!(
        validation.conclusion(),
        InfrastructureOperationsValidationConclusion::Incomplete
    );
    assert!(validation
        .proposed_checks()
        .iter()
        .all(|check| check.status() == InfrastructureOperationsEvidenceStatus::NotRun));

    let InfrastructureOperationsSecurityStageOutcome::Completed(risk) = result.security() else {
        return Err("Cloud Security was not completed".into());
    };
    assert_eq!(risk.task_id(), &security_id);
    assert_eq!(risk.assessment_task_id(), &cloud_id);
    assert_eq!(risk.qa_task_id(), Some(&qa_id));
    assert_eq!(
        risk.credential_evidence(),
        InfrastructureOperationsSupportingEvidenceStatus::Unavailable
    );

    assert_eq!(
        result.synthesis().approval_requirement(),
        InfrastructureOperationsApprovalRequirement::NotApplicable
    );
    assert_eq!(
        result.synthesis().execution_disposition(),
        InfrastructureOperationsExecutionDisposition::NotAttempted
    );
    assert_eq!(
        result.synthesis().status(),
        InfrastructureOperationsReviewStatus::Partial
    );
    assert_eq!(
        result.synthesis().partial_failure_codes(),
        &[InfrastructureOperationsPartialFailureCode::QaIncomplete]
    );
    assert_eq!(
        result.synthesis().stage_projection().first_stage(),
        InfrastructureOperationsStageDisposition::Completed
    );
    assert_eq!(
        result.synthesis().stage_projection().qa(),
        InfrastructureOperationsStageDisposition::Completed
    );
    assert_eq!(
        result.synthesis().stage_projection().security(),
        InfrastructureOperationsStageDisposition::Completed
    );
    for status in [
        result.synthesis().terraform_execution().fmt_check(),
        result.synthesis().terraform_execution().validate(),
        result
            .synthesis()
            .terraform_execution()
            .provider_initialization(),
        result.synthesis().terraform_execution().plan(),
        result.synthesis().terraform_execution().apply(),
    ] {
        assert_eq!(status, InfrastructureOperationsEvidenceStatus::NotRun);
    }

    let cloud_events = orchestrator.cloud_infrastructure_events();
    assert!(
        matches!(
            cloud_events,
            [
                CloudInfrastructureWorkflowEvent::CloudAssessmentStarted { .. },
                CloudInfrastructureWorkflowEvent::CloudAssessmentCompleted { .. },
                CloudInfrastructureWorkflowEvent::QaValidationStarted { .. },
                CloudInfrastructureWorkflowEvent::QaValidationCompleted {
                    conclusion: InfrastructureOperationsValidationConclusion::Incomplete,
                    ..
                },
                CloudInfrastructureWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::QaValidation,
                    code: InfrastructureOperationsPartialFailureCode::QaIncomplete,
                },
                CloudInfrastructureWorkflowEvent::SecurityReviewStarted { .. },
                CloudInfrastructureWorkflowEvent::SecurityReviewCompleted { .. },
                CloudInfrastructureWorkflowEvent::SynthesisStarted { .. },
                CloudInfrastructureWorkflowEvent::Completed { .. },
            ]
        ),
        "unexpected Cloud events: {cloud_events:?}"
    );
    let audit = orchestrator.cloud_infrastructure_attribution_records();
    assert_eq!(
        audit.len(),
        orchestrator.cloud_infrastructure_events().len()
    );
    assert!(audit
        .iter()
        .enumerate()
        .all(|(index, record)| usize::from(record.sequence()) == index));
    assert_eq!(
        audit[0].attribution().agent_id(),
        AgentId::CloudInfrastructure
    );
    assert_eq!(
        audit[1].capability_disposition(),
        InfrastructureOperationsCapabilityAuditDisposition::ProposalOnly
    );
    assert_eq!(audit[2].attribution().agent_id(), AgentId::QaValidation);
    assert!(audit.iter().any(|record| {
        record.stage() == InfrastructureOperationsStage::SecurityReview
            && record.attribution().agent_id() == AgentId::SecurityRisk
    }));
    assert!(audit.iter().any(|record| {
        record.stage() == InfrastructureOperationsStage::Synthesis
            && record.attribution().agent_id() == AgentId::PersonalAssistant
    }));
    assert!(audit
        .iter()
        .all(|record| record.attribution().runtime_id() == RuntimeId::Native));
    assert_eq!(
        orchestrator.governance_audit_records().len(),
        governance_before
    );
    assert_eq!(
        orchestrator.task_count(),
        MAX_INFRASTRUCTURE_OPERATIONS_TASKS_PER_ROOT
    );
    assert_eq!(
        orchestrator.run_count(),
        MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT
    );
    assert_eq!(orchestrator.active_child_task(), None);
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );

    let starts = recorder.starts();
    assert_eq!(
        starts.len(),
        usize::from(MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT)
    );
    assert!(starts.iter().all(|start| {
        start.selected_text.len() <= MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES
    }));
    assert!(starts[1]
        .selected_text
        .contains("source\":\"synthetic-fixture"));
    assert!(!starts.iter().any(|start| {
        start.selected_text.contains("terraform apply")
            || start.selected_text.contains("credentials_loaded=true")
            || start.selected_text.contains("effects_performed=true")
    }));
    Ok(())
}

#[test]
fn exact_thirty_two_runtime_events_complete_without_replenishing_any_closed_budget(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, cloud) = start_cloud(&mut orchestrator)?;
    complete_stage_in_eight_events(
        &mut orchestrator,
        &cloud,
        "cloud-eight-events",
        CLOUD_ASSESSMENT,
    )?;
    let qa = active_child_context(&orchestrator)?;
    complete_stage_in_eight_events(&mut orchestrator, &qa, "qa-eight-events", CLOUD_QA)?;
    let security = active_child_context(&orchestrator)?;
    complete_stage_in_eight_events(
        &mut orchestrator,
        &security,
        "security-eight-events",
        RISK_ASSESSMENT,
    )?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    complete_stage_in_eight_events(
        &mut orchestrator,
        &synthesis,
        "synthesis-eight-events",
        CLOUD_SYNTHESIS,
    )?;

    assert_eq!(
        orchestrator.runtime_event_count(),
        MAX_RUNTIME_EVENTS_PER_ROOT
    );
    assert!(orchestrator.cloud_infrastructure_result().is_some());
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    assert!(
        orchestrator.cloud_infrastructure_events().len()
            <= MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS
    );
    assert_eq!(
        orchestrator
            .cloud_infrastructure_attribution_records()
            .len(),
        orchestrator.cloud_infrastructure_events().len()
    );
    Ok(())
}

#[test]
fn eighth_nonterminal_event_is_rejected_atomically_while_the_reserved_terminal_still_advances(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, cloud) = start_cloud(&mut orchestrator)?;
    orchestrator.accept_runtime_event(cloud.task_id(), started(&cloud, "reserved-terminal")?)?;
    for (index, chunk) in six_ascii_chunks(CLOUD_ASSESSMENT)?.into_iter().enumerate() {
        orchestrator.accept_runtime_event(
            cloud.task_id(),
            delta_at(&cloud, u32::try_from(index)? + 1, chunk)?,
        )?;
    }
    let before = (
        orchestrator.runtime_event_count(),
        orchestrator.events().len(),
        orchestrator.cloud_infrastructure_events().len(),
        orchestrator
            .cloud_infrastructure_attribution_records()
            .len(),
        orchestrator.task_count(),
        orchestrator.run_count(),
    );
    assert_eq!(
        orchestrator.accept_runtime_event(cloud.task_id(), delta_at(&cloud, 7, "extra")?),
        Err(AgentOrchestratorError::InfrastructureOperationsEventLimitExceeded)
    );
    assert_eq!(
        (
            orchestrator.runtime_event_count(),
            orchestrator.events().len(),
            orchestrator.cloud_infrastructure_events().len(),
            orchestrator
                .cloud_infrastructure_attribution_records()
                .len(),
            orchestrator.task_count(),
            orchestrator.run_count(),
        ),
        before
    );
    assert_eq!(
        orchestrator.task(cloud.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert_eq!(
        orchestrator.accept_runtime_event(cloud.task_id(), completed_at(&cloud, 7))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(orchestrator.runtime_event_count(), 8);
    assert_eq!(
        active_child_context(&orchestrator)?.agent_id(),
        AgentId::QaValidation
    );
    Ok(())
}

#[test]
fn deterministic_systems_workflow_preserves_evidence_hypotheses_and_no_effects(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let governance_before = orchestrator.governance_audit_records().len();
    let (root, systems) = start_systems(&mut orchestrator)?;
    let root_id = root.task_id().clone();
    let systems_id = systems.task_id().clone();

    complete_stage(
        &mut orchestrator,
        &systems,
        "systems-assessment-ok",
        SYSTEMS_ASSESSMENT,
    )?;
    let qa = active_child_context(&orchestrator)?;
    let qa_id = qa.task_id().clone();
    complete_stage(&mut orchestrator, &qa, "systems-qa-ok", SYSTEMS_QA)?;
    let security = active_child_context(&orchestrator)?;
    let security_id = security.task_id().clone();
    complete_stage(
        &mut orchestrator,
        &security,
        "systems-security-ok",
        SYSTEMS_RISK_ASSESSMENT,
    )?;
    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "systems-synthesis-ok",
        SYSTEMS_SYNTHESIS,
    )?;

    let result = orchestrator
        .systems_operations_result()
        .ok_or("missing Systems workflow result")?;
    let SystemsOperationsStageOutcome::Completed(assessment) = result.assessment() else {
        return Err("Systems assessment was not completed".into());
    };
    assert_eq!(assessment.task_id(), &systems_id);
    assert_eq!(assessment.fixture_ids().len(), 3);
    assert_eq!(assessment.findings().len(), 2);
    assert_eq!(assessment.findings()[0].references().len(), 1);
    assert!(assessment.findings()[1].references().is_empty());
    assert!(assessment
        .capabilities()
        .iter()
        .all(|capability| capability.disposition()
            == InfrastructureOperationsCapabilityDisposition::ProposalOnly));
    let InfrastructureOperationsQaStageOutcome::Completed(validation) = result.qa() else {
        return Err("Systems QA was not completed".into());
    };
    assert_eq!(validation.task_id(), &qa_id);
    assert_eq!(validation.assessment_task_id(), &systems_id);
    assert_eq!(
        validation.conclusion(),
        InfrastructureOperationsValidationConclusion::Incomplete
    );
    let InfrastructureOperationsSecurityStageOutcome::Completed(risk) = result.security() else {
        return Err("Systems Security was not completed".into());
    };
    assert_eq!(risk.task_id(), &security_id);
    assert_eq!(risk.assessment_task_id(), &systems_id);
    assert_eq!(risk.qa_task_id(), Some(&qa_id));
    assert_eq!(
        result.synthesis().approval_requirement(),
        InfrastructureOperationsApprovalRequirement::NotApplicable
    );
    assert_eq!(
        result.synthesis().execution_disposition(),
        InfrastructureOperationsExecutionDisposition::NotAttempted
    );
    assert_eq!(
        result.synthesis().status(),
        InfrastructureOperationsReviewStatus::Partial
    );
    assert_eq!(
        result.synthesis().partial_failure_codes(),
        &[InfrastructureOperationsPartialFailureCode::QaIncomplete]
    );
    assert_eq!(result.root_task_id().task_id(), &root_id);

    let events = orchestrator.systems_operations_events();
    assert!(
        matches!(
            events,
            [
                SystemsOperationsWorkflowEvent::SystemsAssessmentStarted { .. },
                SystemsOperationsWorkflowEvent::SystemsAssessmentCompleted { .. },
                SystemsOperationsWorkflowEvent::QaValidationStarted { .. },
                SystemsOperationsWorkflowEvent::QaValidationCompleted {
                    conclusion: InfrastructureOperationsValidationConclusion::Incomplete,
                    ..
                },
                SystemsOperationsWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::QaValidation,
                    code: InfrastructureOperationsPartialFailureCode::QaIncomplete,
                },
                SystemsOperationsWorkflowEvent::SecurityReviewStarted { .. },
                SystemsOperationsWorkflowEvent::SecurityReviewCompleted { .. },
                SystemsOperationsWorkflowEvent::SynthesisStarted { .. },
                SystemsOperationsWorkflowEvent::Completed { .. },
            ]
        ),
        "unexpected Systems events: {events:?}"
    );
    let audit = orchestrator.systems_operations_attribution_records();
    assert_eq!(audit.len(), events.len());
    assert!(audit
        .iter()
        .enumerate()
        .all(|(index, record)| usize::from(record.sequence()) == index));
    assert!(audit.iter().any(|record| {
        record.stage() == InfrastructureOperationsStage::SystemsAssessment
            && record.capability_disposition()
                == InfrastructureOperationsCapabilityAuditDisposition::ProposalOnly
    }));
    assert!(audit
        .iter()
        .all(|record| record.attribution().runtime_id() == RuntimeId::Native));
    assert_eq!(
        orchestrator.governance_audit_records().len(),
        governance_before
    );
    assert_eq!(
        orchestrator.task_count(),
        MAX_INFRASTRUCTURE_OPERATIONS_TASKS_PER_ROOT
    );
    assert_eq!(
        orchestrator.run_count(),
        MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    let starts = recorder.starts();
    assert_eq!(
        starts.len(),
        usize::from(MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT)
    );
    assert!(starts[1]
        .selected_text
        .contains("source\":\"synthetic-fixture"));
    assert!(starts.iter().all(|start| {
        start.selected_text.len() <= MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES
            && !start.selected_text.contains("credentials_loaded=true")
            && !start.selected_text.contains("effects_performed=true")
    }));
    Ok(())
}

#[test]
fn denied_cloud_capability_is_validated_partial_and_never_creates_execution_authority(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let governance_before = orchestrator.governance_audit_records().len();
    let (root, cloud) = start_cloud(&mut orchestrator)?;
    let denied_assessment = CLOUD_ASSESSMENT.replace(
        "\"propose-infrastructure-change-plan\"]",
        "\"propose-infrastructure-change-plan\",\"terraform-apply\"]",
    );
    complete_stage(
        &mut orchestrator,
        &cloud,
        "cloud-denied-capability",
        &denied_assessment,
    )?;
    let qa = active_child_context(&orchestrator)?;
    complete_stage(&mut orchestrator, &qa, "cloud-denied-qa", CLOUD_QA)?;
    let security = active_child_context(&orchestrator)?;
    complete_stage(
        &mut orchestrator,
        &security,
        "cloud-denied-security",
        RISK_ASSESSMENT,
    )?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "cloud-denied-synthesis",
        CLOUD_SYNTHESIS,
    )?;

    let result = orchestrator
        .cloud_infrastructure_result()
        .ok_or("missing denied-capability Cloud result")?;
    let CloudInfrastructureStageOutcome::Completed(assessment) = result.assessment() else {
        return Err("denied-capability assessment was not preserved".into());
    };
    assert_eq!(
        assessment.quality(),
        ai_agent_assistant_lib::agent::infrastructure_operations::InfrastructureOperationsAssessmentQuality::PartialDeniedCapability
    );
    assert!(assessment
        .change_plan()
        .capabilities()
        .contains(&CloudInfrastructureCapability::TerraformApply));
    assert_eq!(
        result.synthesis().partial_failure_codes(),
        &[
            InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability,
            InfrastructureOperationsPartialFailureCode::QaIncomplete,
        ]
    );
    assert_eq!(
        result.synthesis().approval_requirement(),
        InfrastructureOperationsApprovalRequirement::RequiredBeforeConsequentialAction
    );
    assert_eq!(
        result.synthesis().execution_disposition(),
        InfrastructureOperationsExecutionDisposition::NotAttempted
    );
    assert_eq!(
        result.synthesis().status(),
        InfrastructureOperationsReviewStatus::Partial
    );
    assert!(orchestrator
        .cloud_infrastructure_events()
        .windows(2)
        .any(|events| matches!(
            events,
            [
                CloudInfrastructureWorkflowEvent::CloudAssessmentCompleted { .. },
                CloudInfrastructureWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::CloudAssessment,
                    code: InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability,
                }
            ]
        )));
    assert!(orchestrator
        .cloud_infrastructure_attribution_records()
        .windows(2)
        .any(|records| matches!(
            records,
            [completed, partial]
                if completed.outcome() == InfrastructureOperationsAuditOutcome::Completed
                    && completed.capability_disposition()
                        == InfrastructureOperationsCapabilityAuditDisposition::DeniedRequested
                    && partial.outcome()
                        == InfrastructureOperationsAuditOutcome::PartialFailure(
                            InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability
                        )
        )));
    assert_eq!(
        orchestrator.governance_audit_records().len(),
        governance_before
    );
    assert!(orchestrator
        .pending_governance_approval(root.task_id())?
        .is_none());
    Ok(())
}

#[test]
fn denied_systems_capability_is_partial_inert_and_does_not_create_approval(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let governance_before = orchestrator.governance_audit_records().len();
    let (root, systems) = start_systems(&mut orchestrator)?;
    let denied_assessment = SYSTEMS_ASSESSMENT.replace(
        "\"propose-remediation-plan\"]",
        "\"propose-remediation-plan\",\"restart-or-stop-service\"]",
    );
    complete_stage(
        &mut orchestrator,
        &systems,
        "systems-denied-capability",
        &denied_assessment,
    )?;
    let qa = active_child_context(&orchestrator)?;
    complete_stage(&mut orchestrator, &qa, "systems-denied-qa", SYSTEMS_QA)?;
    let security = active_child_context(&orchestrator)?;
    complete_stage(
        &mut orchestrator,
        &security,
        "systems-denied-security",
        SYSTEMS_RISK_ASSESSMENT,
    )?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "systems-denied-synthesis",
        SYSTEMS_SYNTHESIS,
    )?;

    let result = orchestrator
        .systems_operations_result()
        .ok_or("missing denied-capability Systems result")?;
    let SystemsOperationsStageOutcome::Completed(assessment) = result.assessment() else {
        return Err("denied-capability Systems assessment was not preserved".into());
    };
    assert_eq!(
        assessment.quality(),
        ai_agent_assistant_lib::agent::infrastructure_operations::InfrastructureOperationsAssessmentQuality::PartialDeniedCapability
    );
    assert!(assessment
        .capabilities()
        .contains(&SystemsOperationsCapability::RestartOrStopService));
    assert_eq!(
        result.synthesis().partial_failure_codes(),
        &[
            InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability,
            InfrastructureOperationsPartialFailureCode::QaIncomplete,
        ]
    );
    assert_eq!(
        result.synthesis().approval_requirement(),
        InfrastructureOperationsApprovalRequirement::RequiredBeforeConsequentialAction
    );
    assert_eq!(
        result.synthesis().execution_disposition(),
        InfrastructureOperationsExecutionDisposition::NotAttempted
    );
    assert!(orchestrator
        .systems_operations_events()
        .windows(2)
        .any(|events| matches!(
            events,
            [
                SystemsOperationsWorkflowEvent::SystemsAssessmentCompleted { .. },
                SystemsOperationsWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::SystemsAssessment,
                    code: InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability,
                }
            ]
        )));
    assert!(orchestrator
        .systems_operations_attribution_records()
        .iter()
        .any(|record| {
            record.stage() == InfrastructureOperationsStage::SystemsAssessment
                && record.capability_disposition()
                    == InfrastructureOperationsCapabilityAuditDisposition::DeniedRequested
        }));
    assert_eq!(
        orchestrator.governance_audit_records().len(),
        governance_before
    );
    assert!(orchestrator
        .pending_governance_approval(root.task_id())?
        .is_none());
    Ok(())
}

#[test]
fn first_stage_runtime_failure_skips_specialists_and_returns_truthful_partial_synthesis(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, cloud) = start_cloud(&mut orchestrator)?;
    fail_stage(&mut orchestrator, &cloud, "cloud-runtime-failed")?;

    assert_eq!(orchestrator.active_child_task(), None);
    let synthesis = orchestrator.current_context(root.task_id())?;
    assert_eq!(synthesis.agent_id(), AgentId::PersonalAssistant);
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "cloud-partial-synthesis",
        CLOUD_SYNTHESIS,
    )?;
    let result = orchestrator
        .cloud_infrastructure_result()
        .ok_or("missing partial Cloud result")?;
    assert!(matches!(
        result.assessment(),
        CloudInfrastructureStageOutcome::Failed(_)
    ));
    assert!(matches!(
        result.qa(),
        InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable
    ));
    assert!(matches!(
        result.security(),
        InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable
    ));
    assert_eq!(
        result.synthesis().partial_failure_codes(),
        &[
            InfrastructureOperationsPartialFailureCode::RuntimeFailed,
            InfrastructureOperationsPartialFailureCode::FirstStageUnavailable,
        ]
    );
    assert_eq!(
        result.synthesis().status(),
        InfrastructureOperationsReviewStatus::Partial
    );
    assert_eq!(
        result.synthesis().execution_disposition(),
        InfrastructureOperationsExecutionDisposition::NotAttempted
    );
    assert!(matches!(
        orchestrator.cloud_infrastructure_events(),
        [
            CloudInfrastructureWorkflowEvent::CloudAssessmentStarted { .. },
            CloudInfrastructureWorkflowEvent::PartialFailure {
                stage: InfrastructureOperationsStage::CloudAssessment,
                code: InfrastructureOperationsPartialFailureCode::RuntimeFailed,
            },
            CloudInfrastructureWorkflowEvent::SynthesisStarted { .. },
            CloudInfrastructureWorkflowEvent::Completed { .. },
        ]
    ));
    assert_eq!(recorder.starts().len(), 3);
    assert!(!recorder.starts().iter().any(|start| {
        start.selected_text.contains("assigned_agent=qa-validation")
            || start.selected_text.contains("assigned_agent=security-risk")
    }));
    Ok(())
}

#[test]
fn later_stage_failures_preserve_only_validated_predecessors_and_final_failure_fails_root(
) -> Result<(), Box<dyn Error>> {
    let mut qa_failure = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, cloud) = start_cloud(&mut qa_failure)?;
    complete_stage(
        &mut qa_failure,
        &cloud,
        "cloud-before-qa-failure",
        CLOUD_ASSESSMENT,
    )?;
    let qa = active_child_context(&qa_failure)?;
    fail_stage(&mut qa_failure, &qa, "qa-runtime-failure")?;
    let security = active_child_context(&qa_failure)?;
    assert_eq!(security.agent_id(), AgentId::SecurityRisk);
    complete_stage(
        &mut qa_failure,
        &security,
        "security-without-qa",
        RISK_WITHOUT_QA,
    )?;
    let synthesis = qa_failure.current_context(root.task_id())?;
    complete_stage(
        &mut qa_failure,
        &synthesis,
        "qa-failure-synthesis",
        CLOUD_SYNTHESIS,
    )?;
    let result = qa_failure
        .cloud_infrastructure_result()
        .ok_or("missing QA-failure result")?;
    assert!(matches!(
        result.qa(),
        InfrastructureOperationsQaStageOutcome::Failed(_)
    ));
    assert!(matches!(
        result.security(),
        InfrastructureOperationsSecurityStageOutcome::Completed(_)
    ));
    assert_eq!(
        result.synthesis().status(),
        InfrastructureOperationsReviewStatus::Partial
    );
    assert!(qa_failure
        .cloud_infrastructure_events()
        .iter()
        .any(|event| matches!(
            event,
            CloudInfrastructureWorkflowEvent::PartialFailure {
                stage: InfrastructureOperationsStage::QaValidation,
                code: InfrastructureOperationsPartialFailureCode::QaUnavailable,
            }
        )));

    let mut security_failure = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, cloud) = start_cloud(&mut security_failure)?;
    complete_stage(
        &mut security_failure,
        &cloud,
        "cloud-before-security-failure",
        CLOUD_ASSESSMENT,
    )?;
    let qa = active_child_context(&security_failure)?;
    complete_stage(
        &mut security_failure,
        &qa,
        "qa-before-security-failure",
        CLOUD_QA,
    )?;
    let security = active_child_context(&security_failure)?;
    fail_stage(&mut security_failure, &security, "security-runtime-failure")?;
    let synthesis = security_failure.current_context(root.task_id())?;
    complete_stage(
        &mut security_failure,
        &synthesis,
        "security-failure-synthesis",
        CLOUD_SYNTHESIS,
    )?;
    let result = security_failure
        .cloud_infrastructure_result()
        .ok_or("missing Security-failure result")?;
    assert!(matches!(
        result.security(),
        InfrastructureOperationsSecurityStageOutcome::Failed(_)
    ));
    assert!(security_failure
        .cloud_infrastructure_events()
        .iter()
        .any(|event| matches!(
            event,
            CloudInfrastructureWorkflowEvent::PartialFailure {
                stage: InfrastructureOperationsStage::SecurityReview,
                code: InfrastructureOperationsPartialFailureCode::SecurityUnavailable,
            }
        )));

    let mut synthesis_failure = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, cloud) = start_cloud(&mut synthesis_failure)?;
    complete_stage(
        &mut synthesis_failure,
        &cloud,
        "cloud-before-final-failure",
        CLOUD_ASSESSMENT,
    )?;
    let qa = active_child_context(&synthesis_failure)?;
    complete_stage(
        &mut synthesis_failure,
        &qa,
        "qa-before-final-failure",
        CLOUD_QA,
    )?;
    let security = active_child_context(&synthesis_failure)?;
    complete_stage(
        &mut synthesis_failure,
        &security,
        "security-before-final-failure",
        RISK_ASSESSMENT,
    )?;
    let synthesis = synthesis_failure.current_context(root.task_id())?;
    fail_stage(
        &mut synthesis_failure,
        &synthesis,
        "synthesis-runtime-failure",
    )?;
    assert_eq!(
        synthesis_failure.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(synthesis_failure.cloud_infrastructure_result().is_none());
    assert!(matches!(
        synthesis_failure.cloud_infrastructure_events().last(),
        Some(CloudInfrastructureWorkflowEvent::PartialFailure {
            stage: InfrastructureOperationsStage::Synthesis,
            code: InfrastructureOperationsPartialFailureCode::RuntimeFailed,
        })
    ));
    Ok(())
}

#[test]
fn cloud_and_systems_selection_is_mutually_exclusive_and_root_cancel_is_child_first(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, cloud) = start_cloud(&mut orchestrator)?;
    assert_eq!(
        orchestrator.selected_workflow(),
        Some(AgentWorkflowSelection::CloudInfrastructure)
    );
    let before = (
        orchestrator.task_count(),
        orchestrator.run_count(),
        orchestrator.events().len(),
        orchestrator.cloud_infrastructure_events().len(),
    );
    let systems_request = InfrastructureOperationsFixtureCatalog::built_in()
        .systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;
    assert_eq!(
        orchestrator.start_systems_operations_workflow(&cloud, systems_request),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::CloudInfrastructure,
        })
    );
    assert_eq!(
        (
            orchestrator.task_count(),
            orchestrator.run_count(),
            orchestrator.events().len(),
            orchestrator.cloud_infrastructure_events().len(),
        ),
        before
    );

    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator.task(cloud.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(orchestrator.active_child_task(), None);
    assert!(orchestrator.cloud_infrastructure_result().is_none());
    assert!(matches!(
        orchestrator.cloud_infrastructure_events(),
        [
            CloudInfrastructureWorkflowEvent::CloudAssessmentStarted { .. },
            CloudInfrastructureWorkflowEvent::Cancelled {
                stage: InfrastructureOperationsStage::CloudAssessment,
            },
        ]
    ));
    assert_eq!(recorder.cancellations().len(), 2);

    let mut reverse = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (reverse_root, systems) = start_systems(&mut reverse)?;
    assert_eq!(
        reverse.selected_workflow(),
        Some(AgentWorkflowSelection::SystemsOperations)
    );
    let before = (
        reverse.task_count(),
        reverse.run_count(),
        reverse.events().len(),
        reverse.systems_operations_events().len(),
    );
    let cloud_request = InfrastructureOperationsFixtureCatalog::built_in()
        .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    assert_eq!(
        reverse.start_cloud_infrastructure_workflow(&systems, cloud_request),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::SystemsOperations,
        })
    );
    assert_eq!(
        (
            reverse.task_count(),
            reverse.run_count(),
            reverse.events().len(),
            reverse.systems_operations_events().len(),
        ),
        before
    );
    assert_eq!(
        reverse.cancel_task(reverse_root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    Ok(())
}

#[test]
fn infrastructure_selectors_exclude_every_existing_selector_and_repetition_before_mutation(
) -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let document_path = directory.path().join("infrastructure-selector.txt");
    fs::write(&document_path, "bounded selector fixture")?;

    let mut cloud_selected = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let cloud_request = InfrastructureOperationsFixtureCatalog::built_in()
        .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let root = cloud_selected.start_root(cloud_request.objective())?;
    let document_id = cloud_selected.register_approved_document(
        &root,
        ApprovedDocumentSource::UserSelectedFile,
        &document_path,
    )?;
    let acceptance = cloud_selected.start_cloud_infrastructure_workflow(&root, cloud_request)?;
    let cloud = acceptance.context().clone();
    let before = (
        cloud_selected.task_count(),
        cloud_selected.run_count(),
        cloud_selected.events().len(),
        cloud_selected.cloud_infrastructure_events().len(),
        cloud_selected
            .cloud_infrastructure_attribution_records()
            .len(),
    );
    assert_eq!(
        cloud_selected.start_cloud_infrastructure_workflow(
            &cloud,
            InfrastructureOperationsFixtureCatalog::built_in()
                .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?,
        ),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::CloudInfrastructure,
        })
    );
    assert_eq!(
        cloud_selected.request_research_knowledge_workflow(&cloud, selector_research_request()?),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::CloudInfrastructure,
        })
    );
    assert_eq!(
        cloud_selected.start_engineering_quality_workflow(&cloud, selector_engineering_request()?,),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::CloudInfrastructure,
        })
    );
    assert_eq!(
        cloud_selected.request_document_task(&cloud, &document_id, DocumentOperation::Read, None),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::CloudInfrastructure,
        })
    );
    assert_eq!(
        cloud_selected.request_delegation(&cloud, selector_research_delegation()?),
        Err(AgentOrchestratorError::UnauthorizedSource {
            agent_id: AgentId::CloudInfrastructure,
        })
    );
    assert_eq!(
        (
            cloud_selected.task_count(),
            cloud_selected.run_count(),
            cloud_selected.events().len(),
            cloud_selected.cloud_infrastructure_events().len(),
            cloud_selected
                .cloud_infrastructure_attribution_records()
                .len(),
        ),
        before
    );

    let mut systems_selected = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (systems_root, systems) = start_systems(&mut systems_selected)?;
    let before = (
        systems_selected.task_count(),
        systems_selected.run_count(),
        systems_selected.events().len(),
        systems_selected.systems_operations_events().len(),
    );
    assert_eq!(
        systems_selected.start_systems_operations_workflow(
            &systems,
            InfrastructureOperationsFixtureCatalog::built_in()
                .systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?,
        ),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::SystemsOperations,
        })
    );
    assert_eq!(
        (
            systems_selected.task_count(),
            systems_selected.run_count(),
            systems_selected.events().len(),
            systems_selected.systems_operations_events().len(),
        ),
        before
    );
    assert_eq!(
        systems_selected.cancel_task(systems_root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );

    let mut generic = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = generic.start_root("Generic selector owns this root")?;
    let research = generic
        .request_delegation(&root, selector_research_delegation()?)?
        .child_context()
        .clone();
    let before = (
        generic.task_count(),
        generic.run_count(),
        generic.events().len(),
    );
    assert_eq!(
        generic.start_cloud_infrastructure_workflow(
            &research,
            InfrastructureOperationsFixtureCatalog::built_in()
                .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?,
        ),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::GenericDelegation,
        })
    );
    assert_eq!(
        (
            generic.task_count(),
            generic.run_count(),
            generic.events().len()
        ),
        before
    );

    let mut research = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = research.start_root("Research selector owns this root")?;
    let research_context = research
        .request_research_knowledge_workflow(&root, selector_research_request()?)?
        .context()
        .clone();
    let before = (
        research.task_count(),
        research.run_count(),
        research.events().len(),
    );
    assert_eq!(
        research.start_cloud_infrastructure_workflow(
            &research_context,
            InfrastructureOperationsFixtureCatalog::built_in()
                .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?,
        ),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::ResearchKnowledge,
        })
    );
    assert_eq!(
        (
            research.task_count(),
            research.run_count(),
            research.events().len()
        ),
        before
    );

    let mut engineering = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = engineering.start_root("Engineering selector owns this root")?;
    let coding = engineering
        .start_engineering_quality_workflow(&root, selector_engineering_request()?)?
        .context()
        .clone();
    let before = (
        engineering.task_count(),
        engineering.run_count(),
        engineering.events().len(),
    );
    assert_eq!(
        engineering.start_cloud_infrastructure_workflow(
            &coding,
            InfrastructureOperationsFixtureCatalog::built_in()
                .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?,
        ),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::EngineeringQuality,
        })
    );
    assert_eq!(
        (
            engineering.task_count(),
            engineering.run_count(),
            engineering.events().len(),
        ),
        before
    );

    let document_path = directory.path().join("document-selector.txt");
    fs::write(&document_path, "bounded document selector fixture")?;
    let mut document = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = document.start_root("Document selector owns this root")?;
    let document_id = document.register_approved_document(
        &root,
        ApprovedDocumentSource::UserSelectedFile,
        &document_path,
    )?;
    let knowledge =
        document.request_document_task(&root, &document_id, DocumentOperation::Read, None)?;
    let before = (
        document.task_count(),
        document.run_count(),
        document.events().len(),
    );
    assert_eq!(
        document.start_cloud_infrastructure_workflow(
            &knowledge,
            InfrastructureOperationsFixtureCatalog::built_in()
                .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?,
        ),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::ApprovedDocument,
        })
    );
    assert_eq!(
        (
            document.task_count(),
            document.run_count(),
            document.events().len()
        ),
        before
    );
    Ok(())
}

#[test]
fn native_runtime_remains_default_text_only_and_accepts_the_infrastructure_input_bound(
) -> Result<(), Box<dyn Error>> {
    let adversarial_unit = "\"\\\n\t🙂";
    let selected_text = adversarial_unit
        .repeat(MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES / adversarial_unit.len());
    assert_eq!(
        selected_text.len(),
        MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES
    );
    let runtime_request = RuntimeTurnRequest::new(
        "r".repeat(MAX_OPAQUE_ID_BYTES),
        "q".repeat(MAX_OPAQUE_ID_BYTES),
        selected_text,
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
    let run = runtime.start(runtime_request)?;
    assert!(run.request_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);

    let mut orchestrator = AgentOrchestrator::native()?;
    let request = InfrastructureOperationsFixtureCatalog::built_in()
        .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let acceptance = orchestrator.start_cloud_infrastructure_workflow(&root, request)?;
    assert_eq!(acceptance.context().runtime_id(), RuntimeId::Native);
    assert_eq!(
        acceptance.context().agent_id(),
        AgentId::CloudInfrastructure
    );
    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    Ok(())
}

#[test]
fn strict_stage_outputs_reject_unknown_cross_kind_secret_and_false_authority_data(
) -> Result<(), Box<dyn Error>> {
    let oversized_utf8_scalar = "🙂".repeat((MODEL_TEXT_BYTE_LIMIT / "🙂".len()) + 1);
    let invalid_assessments = vec![
        CLOUD_ASSESSMENT.replacen(
            "\"version\":\"v1\"",
            "\"version\":\"v1\",\"version\":\"v1\"",
            1,
        ),
        CLOUD_ASSESSMENT.replacen(
            "\"id\":\"public-ingress\"",
            "\"id\":\"public-ingress\",\"id\":\"public-ingress\"",
            1,
        ),
        CLOUD_ASSESSMENT.replace(
            "\"fixture_based\":true",
            "\"fixture_based\":true,\"unknown_field\":null",
        ),
        CLOUD_ASSESSMENT.replace(
            "\"azure-architecture\"]",
            "\"azure-architecture\",\"service-snapshot\"]",
        ),
        CLOUD_ASSESSMENT.replace(
            "\"azure-architecture\"]",
            "\"azure-architecture\",\"azure-architecture\"]",
        ),
        CLOUD_ASSESSMENT.replace(
            "\"statement\":\"The synthetic",
            "\"secret\":\"forbidden\",\"statement\":\"The synthetic",
        ),
        CLOUD_ASSESSMENT.replace(
            "The synthetic configuration exposes public ingress for fixture review.",
            "  Authorization: Bearer fixture-credential",
        ),
        CLOUD_ASSESSMENT.replace(
            "The synthetic configuration exposes public ingress for fixture review.",
            "cortexa_fixture_secret_do_not_use=fixture-value",
        ),
        CLOUD_ASSESSMENT.replace(
            "The synthetic configuration exposes public ingress for fixture review.",
            &oversized_utf8_scalar,
        ),
        CLOUD_ASSESSMENT.replace(
            "\"terraform_apply\":\"not-run\"",
            "\"terraform_apply\":\"observed-fixture\"",
        ),
        CLOUD_ASSESSMENT.replace(
            "Propose restricting public ingress after separate review.",
            "Run the Terraform plan after separate review.",
        ),
        format!(" {CLOUD_ASSESSMENT}"),
        format!("{CLOUD_ASSESSMENT}{CLOUD_ASSESSMENT}"),
        CLOUD_ASSESSMENT[..CLOUD_ASSESSMENT.len() - 1].to_owned(),
    ];

    for (index, raw) in invalid_assessments.iter().enumerate() {
        let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
        let (root, cloud) = start_cloud(&mut orchestrator)?;
        let raw_sentinel = raw.clone();
        complete_stage(
            &mut orchestrator,
            &cloud,
            &format!("invalid-cloud-{index}"),
            raw,
        )?;
        let synthesis = orchestrator.current_context(root.task_id())?;
        assert!(orchestrator.active_child_task().is_none());
        assert!(matches!(
            orchestrator.cloud_infrastructure_events(),
            [
                CloudInfrastructureWorkflowEvent::CloudAssessmentStarted { .. },
                CloudInfrastructureWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::CloudAssessment,
                    code: InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                },
                CloudInfrastructureWorkflowEvent::SynthesisStarted { .. },
            ]
        ));
        let diagnostics = format!(
            "{:?} {:?}",
            orchestrator.cloud_infrastructure_events(),
            orchestrator.cloud_infrastructure_attribution_records()
        );
        assert!(!diagnostics.contains(&raw_sentinel));
        assert_eq!(synthesis.agent_id(), AgentId::PersonalAssistant);
    }

    let cross_kind_null = SYSTEMS_ASSESSMENT.replace(
        "\"effects_performed\":false",
        "\"effects_performed\":false,\"terraform_apply\":null",
    );
    let article_omitted_capability = SYSTEMS_ASSESSMENT.replace(
        "Propose restoring dependency availability before considering service recovery.",
        "Restart the service after separate review.",
    );
    for (index, raw) in [cross_kind_null, article_omitted_capability]
        .iter()
        .enumerate()
    {
        let mut systems = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
        let (root, first) = start_systems(&mut systems)?;
        complete_stage(
            &mut systems,
            &first,
            &format!("systems-invalid-{index}"),
            raw,
        )?;
        assert!(systems.active_child_task().is_none());
        assert_eq!(
            systems.current_context(root.task_id())?.agent_id(),
            AgentId::PersonalAssistant
        );
        assert!(systems
            .systems_operations_events()
            .iter()
            .any(|event| matches!(
                event,
                SystemsOperationsWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::SystemsAssessment,
                    code: InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                }
            )));
    }

    let nearby_nonmatch = CLOUD_ASSESSMENT.replace(
        "The synthetic configuration exposes public ingress for fixture review.",
        "A bearer concept remains fixture-only and contains no credential material.",
    );
    let mut nearby = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, cloud) = start_cloud(&mut nearby)?;
    complete_stage(
        &mut nearby,
        &cloud,
        "nearby-secret-nonmatch",
        &nearby_nonmatch,
    )?;
    assert_eq!(
        active_child_context(&nearby)?.agent_id(),
        AgentId::QaValidation
    );
    Ok(())
}

#[test]
fn aggregate_result_byte_n_plus_one_fails_closed_without_retaining_the_extra_content(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, cloud) = start_cloud(&mut orchestrator)?;
    orchestrator.accept_runtime_event(cloud.task_id(), started(&cloud, "result-byte-bound")?)?;
    let half = "🙂".repeat(MAX_INFRASTRUCTURE_OPERATIONS_RESULT_BYTES / 2 / "🙂".len());
    orchestrator.accept_runtime_event(cloud.task_id(), delta_at(&cloud, 1, &half)?)?;
    orchestrator.accept_runtime_event(cloud.task_id(), delta_at(&cloud, 2, &half)?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(cloud.task_id(), delta_at(&cloud, 3, "x")?),
        Err(AgentOrchestratorError::OutputLimitExceeded)
    );
    assert_eq!(orchestrator.runtime_event_count(), 4);
    assert_eq!(
        orchestrator.task(cloud.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(orchestrator.active_child_task().is_none());
    assert_eq!(
        orchestrator.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );
    assert!(orchestrator.cloud_infrastructure_result().is_none());
    assert!(orchestrator
        .cloud_infrastructure_events()
        .iter()
        .any(|event| matches!(
            event,
            CloudInfrastructureWorkflowEvent::PartialFailure {
                stage: InfrastructureOperationsStage::CloudAssessment,
                code: InfrastructureOperationsPartialFailureCode::RuntimeFailed,
            }
        )));
    Ok(())
}

#[test]
fn qa_and_security_authority_or_provenance_lies_fail_closed_without_false_completion(
) -> Result<(), Box<dyn Error>> {
    let invalid_qa = [
        CLOUD_QA.replace("\"approval_authority\":false", "\"approval_authority\":true"),
        CLOUD_QA.replacen(
            "\"conclusion\":\"incomplete\"",
            "\"conclusion\":\"incomplete\",\"conclusion\":\"incomplete\"",
            1,
        ),
        CLOUD_QA.replacen(
            "\"criterion_id\":\"criterion-static-review\"",
            "\"criterion_id\":\"criterion-static-review\",\"criterion_id\":\"criterion-static-review\"",
            1,
        ),
        CLOUD_QA.replace(
            "\"criterion_id\":\"criterion-no-execution\",\"disposition\":\"not-demonstrated\",\"references\":[{\"namespace\":\"application-evidence\",\"id\":\"evidence-terraform-not-run\"}]",
            "\"criterion_id\":\"criterion-no-execution\",\"disposition\":\"not-demonstrated\",\"references\":[{\"namespace\":\"application-evidence\",\"id\":\"evidence-static-observation\"}]",
        ),
    ];
    for (index, raw) in invalid_qa.iter().enumerate() {
        let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
        let (_root, cloud) = start_cloud(&mut orchestrator)?;
        complete_stage(
            &mut orchestrator,
            &cloud,
            &format!("cloud-before-invalid-qa-{index}"),
            CLOUD_ASSESSMENT,
        )?;
        let qa = active_child_context(&orchestrator)?;
        complete_stage(&mut orchestrator, &qa, &format!("invalid-qa-{index}"), raw)?;
        assert!(matches!(
            orchestrator.cloud_infrastructure_events().last(),
            Some(CloudInfrastructureWorkflowEvent::SecurityReviewStarted { .. })
        ));
        assert!(orchestrator
            .cloud_infrastructure_events()
            .iter()
            .any(|event| matches!(
                event,
                CloudInfrastructureWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::QaValidation,
                    code: InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                }
            )));
    }

    let invalid_security = [
        RISK_ASSESSMENT.replace(
            "\"authorization_granted\":false",
            "\"authorization_granted\":true",
        ),
        RISK_ASSESSMENT
            .replace("\"basis\":\"evidence-bound\"", "\"basis\":\"hypothesis\"")
            .replace("\"confidence\":\"high\"", "\"confidence\":\"medium\""),
        RISK_ASSESSMENT.replace(
            "\"credential_evidence\":\"unavailable\"",
            "\"credential_evidence\":\"synthetic-fixture\"",
        ),
        RISK_ASSESSMENT.replace(
            "\"provider_evidence\":\"unavailable\"",
            "\"provider_evidence\":\"synthetic-fixture\"",
        ),
    ];
    for (index, raw) in invalid_security.iter().enumerate() {
        let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
        let (root, cloud) = start_cloud(&mut orchestrator)?;
        complete_stage(
            &mut orchestrator,
            &cloud,
            &format!("cloud-before-security-{index}"),
            CLOUD_ASSESSMENT,
        )?;
        let qa = active_child_context(&orchestrator)?;
        complete_stage(
            &mut orchestrator,
            &qa,
            &format!("qa-before-security-{index}"),
            CLOUD_QA,
        )?;
        let security = active_child_context(&orchestrator)?;
        complete_stage(
            &mut orchestrator,
            &security,
            &format!("invalid-security-{index}"),
            raw,
        )?;
        assert_eq!(
            orchestrator.current_context(root.task_id())?.agent_id(),
            AgentId::PersonalAssistant
        );
        assert!(orchestrator
            .cloud_infrastructure_events()
            .iter()
            .any(|event| matches!(
                event,
                CloudInfrastructureWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::SecurityReview,
                    code: InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                }
            )));
    }
    Ok(())
}

#[test]
fn direct_child_cancel_advances_only_to_partial_synthesis_and_cancel_failure_is_atomic(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut cancelled = AgentOrchestrator::new(runtime)?;
    let (root, cloud) = start_cloud(&mut cancelled)?;
    assert_eq!(
        cancelled.cancel_task(cloud.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        cancelled.task(cloud.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(cancelled.active_child_task().is_none());
    assert_eq!(
        cancelled.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );
    let cancelled_events = cancelled.cloud_infrastructure_events();
    assert!(
        matches!(
            cancelled_events,
            [
                CloudInfrastructureWorkflowEvent::CloudAssessmentStarted { .. },
                CloudInfrastructureWorkflowEvent::PartialFailure {
                    stage: InfrastructureOperationsStage::CloudAssessment,
                    code: InfrastructureOperationsPartialFailureCode::Cancelled,
                },
                CloudInfrastructureWorkflowEvent::SynthesisStarted { .. },
            ]
        ),
        "unexpected direct-cancel events: {cancelled_events:?}"
    );
    assert_eq!(recorder.cancellations().len(), 2);

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(2));
    let mut failure = AgentOrchestrator::new(runtime)?;
    let (_root, cloud) = start_cloud(&mut failure)?;
    let before = (
        failure.task_count(),
        failure.run_count(),
        failure.events().len(),
        failure.cloud_infrastructure_events().len(),
        failure.cloud_infrastructure_attribution_records().len(),
    );
    assert_eq!(
        failure.cancel_task(cloud.task_id()),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
        ))
    );
    assert_eq!(
        (
            failure.task_count(),
            failure.run_count(),
            failure.events().len(),
            failure.cloud_infrastructure_events().len(),
            failure.cloud_infrastructure_attribution_records().len(),
        ),
        before
    );
    assert_eq!(
        failure.task(cloud.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert_eq!(recorder.cancellations().len(), 1);
    Ok(())
}

#[test]
fn qa_security_and_synthesis_cancellation_is_stage_local_and_failure_atomic(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut qa_cancel = AgentOrchestrator::new(runtime)?;
    let (_root, qa) = advance_cloud_to_qa(&mut qa_cancel)?;
    assert_eq!(
        qa_cancel.cancel_task(qa.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        qa_cancel.task(qa.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        active_child_context(&qa_cancel)?.agent_id(),
        AgentId::SecurityRisk
    );
    assert_eq!(recorder.cancellations().len(), 2);
    assert!(qa_cancel.cloud_infrastructure_events().iter().any(|event| {
        matches!(
            event,
            CloudInfrastructureWorkflowEvent::PartialFailure {
                stage: InfrastructureOperationsStage::QaValidation,
                code: InfrastructureOperationsPartialFailureCode::Cancelled,
            }
        )
    }));

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut security_cancel = AgentOrchestrator::new(runtime)?;
    let (root, security) = advance_cloud_to_security(&mut security_cancel)?;
    assert_eq!(
        security_cancel.cancel_task(security.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        security_cancel
            .task(security.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(security_cancel.active_child_task().is_none());
    assert_eq!(
        security_cancel.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );
    assert_eq!(recorder.cancellations().len(), 2);

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut synthesis_cancel = AgentOrchestrator::new(runtime)?;
    let (root, _synthesis) = advance_cloud_to_synthesis(&mut synthesis_cancel)?;
    assert_eq!(
        synthesis_cancel.cancel_task(root.task_id())?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        synthesis_cancel
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(synthesis_cancel.cloud_infrastructure_result().is_none());
    assert!(matches!(
        synthesis_cancel.cloud_infrastructure_events().last(),
        Some(CloudInfrastructureWorkflowEvent::Cancelled {
            stage: InfrastructureOperationsStage::Synthesis,
        })
    ));
    assert_eq!(recorder.cancellations().len(), 2);

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(3));
    let mut qa_failure = AgentOrchestrator::new(runtime)?;
    let (_root, qa) = advance_cloud_to_qa(&mut qa_failure)?;
    let before = (
        qa_failure.task_count(),
        qa_failure.run_count(),
        qa_failure.events().len(),
        qa_failure.runtime_event_count(),
        qa_failure.cloud_infrastructure_events().len(),
        qa_failure.cloud_infrastructure_attribution_records().len(),
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
            qa_failure.cloud_infrastructure_events().len(),
            qa_failure.cloud_infrastructure_attribution_records().len(),
        ),
        before
    );
    assert_eq!(
        qa_failure.active_child_task().map(|task| task.id()),
        Some(qa.task_id())
    );

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(4));
    let mut security_failure = AgentOrchestrator::new(runtime)?;
    let (_root, security) = advance_cloud_to_security(&mut security_failure)?;
    let before = (
        security_failure.task_count(),
        security_failure.run_count(),
        security_failure.events().len(),
        security_failure.runtime_event_count(),
        security_failure.cloud_infrastructure_events().len(),
        security_failure
            .cloud_infrastructure_attribution_records()
            .len(),
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
            security_failure.cloud_infrastructure_events().len(),
            security_failure
                .cloud_infrastructure_attribution_records()
                .len(),
        ),
        before
    );
    assert_eq!(
        security_failure.active_child_task().map(|task| task.id()),
        Some(security.task_id())
    );

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(5));
    let mut synthesis_failure = AgentOrchestrator::new(runtime)?;
    let (root, _synthesis) = advance_cloud_to_synthesis(&mut synthesis_failure)?;
    let before = (
        synthesis_failure.task_count(),
        synthesis_failure.run_count(),
        synthesis_failure.events().len(),
        synthesis_failure.runtime_event_count(),
        synthesis_failure.cloud_infrastructure_events().len(),
        synthesis_failure
            .cloud_infrastructure_attribution_records()
            .len(),
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
            synthesis_failure.cloud_infrastructure_events().len(),
            synthesis_failure
                .cloud_infrastructure_attribution_records()
                .len(),
        ),
        before
    );
    assert_eq!(
        synthesis_failure
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    Ok(())
}

#[test]
fn already_terminal_specialist_cancellation_never_strands_the_workflow_or_forges_cancellation(
) -> Result<(), Box<dyn Error>> {
    let mut cloud = AgentOrchestrator::new(MockAgentRuntime::new(
        MockMode::CancelAlreadyTerminalAt(2, RuntimeRunStatus::Completed),
    ))?;
    let (root, first) = start_cloud(&mut cloud)?;
    assert_eq!(
        cloud.cancel_task(first.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        cloud.task(first.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(cloud.active_child_task().is_none());
    assert_eq!(
        cloud.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );
    assert!(cloud.cloud_infrastructure_result().is_none());
    assert!(!cloud
        .cloud_infrastructure_events()
        .iter()
        .any(|event| matches!(
            event,
            CloudInfrastructureWorkflowEvent::Cancelled { .. }
                | CloudInfrastructureWorkflowEvent::PartialFailure {
                    code: InfrastructureOperationsPartialFailureCode::Cancelled,
                    ..
                }
        )));

    let mut systems = AgentOrchestrator::new(MockAgentRuntime::new(
        MockMode::CancelAlreadyTerminalAt(2, RuntimeRunStatus::Failed),
    ))?;
    let (root, first) = start_systems(&mut systems)?;
    assert_eq!(
        systems.cancel_task(first.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Failed,
        })
    );
    assert_eq!(
        systems.task(first.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(systems.active_child_task().is_none());
    assert_eq!(
        systems.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );
    assert!(!systems
        .systems_operations_events()
        .iter()
        .any(|event| matches!(
            event,
            SystemsOperationsWorkflowEvent::Cancelled { .. }
                | SystemsOperationsWorkflowEvent::PartialFailure {
                    code: InfrastructureOperationsPartialFailureCode::Cancelled,
                    ..
                }
        )));

    let mut qa = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::CancelAlreadyTerminalAt(
        3,
        RuntimeRunStatus::Completed,
    )))?;
    let (_root, first) = start_cloud(&mut qa)?;
    complete_stage(
        &mut qa,
        &first,
        "before-qa-terminal-cancel",
        CLOUD_ASSESSMENT,
    )?;
    let qa_context = active_child_context(&qa)?;
    assert_eq!(
        qa.cancel_task(qa_context.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        qa.task(qa_context.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(active_child_context(&qa)?.agent_id(), AgentId::SecurityRisk);
    assert!(!qa
        .cloud_infrastructure_events()
        .iter()
        .any(|event| matches!(
            event,
            CloudInfrastructureWorkflowEvent::Cancelled { .. }
                | CloudInfrastructureWorkflowEvent::PartialFailure {
                    code: InfrastructureOperationsPartialFailureCode::Cancelled,
                    ..
                }
        )));

    let mut security = AgentOrchestrator::new(MockAgentRuntime::new(
        MockMode::CancelAlreadyTerminalAt(4, RuntimeRunStatus::Failed),
    ))?;
    let (root, first) = start_cloud(&mut security)?;
    complete_stage(
        &mut security,
        &first,
        "before-security-terminal-cancel",
        CLOUD_ASSESSMENT,
    )?;
    let qa_context = active_child_context(&security)?;
    complete_stage(
        &mut security,
        &qa_context,
        "qa-before-terminal-cancel",
        CLOUD_QA,
    )?;
    let security_context = active_child_context(&security)?;
    assert_eq!(
        security.cancel_task(security_context.task_id()),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Failed,
        })
    );
    assert_eq!(
        security
            .task(security_context.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(security.active_child_task().is_none());
    assert_eq!(
        security.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );
    assert!(!security
        .cloud_infrastructure_events()
        .iter()
        .any(|event| matches!(
            event,
            CloudInfrastructureWorkflowEvent::Cancelled { .. }
                | CloudInfrastructureWorkflowEvent::PartialFailure {
                    code: InfrastructureOperationsPartialFailureCode::Cancelled,
                    ..
                }
        )));
    Ok(())
}

#[test]
fn start_failures_preserve_exact_single_and_chained_continuation_snapshots(
) -> Result<(), Box<dyn Error>> {
    let mut first_failure =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailureAt(2)))?;
    let request = InfrastructureOperationsFixtureCatalog::built_in()
        .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let root = first_failure.start_root(request.objective())?;
    let acceptance = first_failure.start_cloud_infrastructure_workflow(&root, request)?;
    assert!(matches!(
        acceptance,
        CloudInfrastructureWorkflowAcceptance::PersonalFallbackStarted { .. }
    ));
    assert_eq!(
        first_failure.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::CloudStartFailed)
    );
    assert_eq!(first_failure.task_count(), 2);
    assert_eq!(first_failure.run_count(), 3);
    assert_eq!(
        first_failure.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(first_failure
        .cloud_infrastructure_events()
        .iter()
        .any(|event| matches!(
            event,
            CloudInfrastructureWorkflowEvent::PartialFailure {
                stage: InfrastructureOperationsStage::CloudAssessment,
                code: InfrastructureOperationsPartialFailureCode::RuntimeStartFailed,
            }
        )));

    let mut chained =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailuresAt(2, 3)))?;
    let request = InfrastructureOperationsFixtureCatalog::built_in()
        .cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
    let root = chained.start_root(request.objective())?;
    assert!(matches!(
        chained.start_cloud_infrastructure_workflow(&root, request),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start)
        ))
    ));
    assert_eq!(
        chained.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::CloudAndSynthesisStartFailed)
    );
    assert_eq!(
        chained.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(chained.cloud_infrastructure_result().is_none());
    assert!(matches!(
        chained.cloud_infrastructure_events().last(),
        Some(CloudInfrastructureWorkflowEvent::PartialFailure {
            stage: InfrastructureOperationsStage::Synthesis,
            code: InfrastructureOperationsPartialFailureCode::RuntimeStartFailed,
        })
    ));
    Ok(())
}

#[test]
fn every_remaining_start_failure_snapshot_is_typed_and_non_retrying() -> Result<(), Box<dyn Error>>
{
    let mut systems = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailureAt(2)))?;
    let request = InfrastructureOperationsFixtureCatalog::built_in()
        .systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;
    let root = systems.start_root(request.objective())?;
    assert!(matches!(
        systems.start_systems_operations_workflow(&root, request)?,
        SystemsOperationsWorkflowAcceptance::PersonalFallbackStarted { .. }
    ));
    assert_eq!(
        systems.systems_operations_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::SystemsStartFailed)
    );

    let mut systems_chained =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailuresAt(2, 3)))?;
    let request = InfrastructureOperationsFixtureCatalog::built_in()
        .systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;
    let root = systems_chained.start_root(request.objective())?;
    assert!(matches!(
        systems_chained.start_systems_operations_workflow(&root, request),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start)
        ))
    ));
    assert_eq!(
        systems_chained.systems_operations_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::SystemsAndSynthesisStartFailed)
    );
    assert_eq!(
        systems_chained
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );

    let mut qa = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailureAt(3)))?;
    let (_root, cloud) = start_cloud(&mut qa)?;
    complete_stage(&mut qa, &cloud, "qa-start-failure", CLOUD_ASSESSMENT)?;
    assert_eq!(
        qa.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::QaStartFailed)
    );
    assert_eq!(active_child_context(&qa)?.agent_id(), AgentId::SecurityRisk);

    let mut qa_security =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailuresAt(3, 4)))?;
    let (root, cloud) = start_cloud(&mut qa_security)?;
    complete_stage(
        &mut qa_security,
        &cloud,
        "qa-security-start-failures",
        CLOUD_ASSESSMENT,
    )?;
    assert_eq!(
        qa_security.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::QaAndSecurityStartFailed)
    );
    assert_eq!(
        qa_security.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );

    let mut qa_synthesis =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailuresAt(3, 5)))?;
    let (root, cloud) = start_cloud(&mut qa_synthesis)?;
    complete_stage(
        &mut qa_synthesis,
        &cloud,
        "qa-before-synthesis-start-failure",
        CLOUD_ASSESSMENT,
    )?;
    let security = active_child_context(&qa_synthesis)?;
    complete_stage(
        &mut qa_synthesis,
        &security,
        "security-before-qa-synthesis-start-failure",
        RISK_WITHOUT_QA,
    )?;
    assert_eq!(
        qa_synthesis.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::QaAndSynthesisStartFailed)
    );
    assert_eq!(
        qa_synthesis.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );

    let mut triple = AgentOrchestrator::new(MockAgentRuntime::new(
        MockMode::StartFailuresAtThree(3, 4, 5),
    ))?;
    let (root, cloud) = start_cloud(&mut triple)?;
    complete_stage(
        &mut triple,
        &cloud,
        "qa-security-synthesis-start-failures",
        CLOUD_ASSESSMENT,
    )?;
    assert_eq!(
        triple.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::QaSecurityAndSynthesisStartFailed)
    );
    assert_eq!(
        triple.task_count(),
        MAX_INFRASTRUCTURE_OPERATIONS_TASKS_PER_ROOT
    );
    assert_eq!(
        triple.run_count(),
        MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT
    );
    assert_eq!(
        triple.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );

    let mut security = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailureAt(4)))?;
    let (root, qa) = advance_cloud_to_qa(&mut security)?;
    complete_stage(&mut security, &qa, "security-start-failure", CLOUD_QA)?;
    assert_eq!(
        security.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::SecurityStartFailed)
    );
    assert_eq!(
        security.current_context(root.task_id())?.agent_id(),
        AgentId::PersonalAssistant
    );

    let mut security_synthesis =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailuresAt(4, 5)))?;
    let (root, qa) = advance_cloud_to_qa(&mut security_synthesis)?;
    complete_stage(
        &mut security_synthesis,
        &qa,
        "security-synthesis-start-failures",
        CLOUD_QA,
    )?;
    assert_eq!(
        security_synthesis.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::SecurityAndSynthesisStartFailed)
    );
    assert_eq!(
        security_synthesis
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );

    let mut synthesis = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailureAt(5)))?;
    let (root, security) = advance_cloud_to_security(&mut synthesis)?;
    complete_stage(
        &mut synthesis,
        &security,
        "synthesis-start-failure",
        RISK_ASSESSMENT,
    )?;
    assert_eq!(
        synthesis.cloud_infrastructure_continuation_failure(),
        Some(InfrastructureOperationsContinuationFailure::SynthesisStartFailed)
    );
    assert_eq!(
        synthesis.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    Ok(())
}

#[test]
fn foreign_and_replayed_runtime_events_fail_before_workflow_mutation() -> Result<(), Box<dyn Error>>
{
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (_root, cloud) = start_cloud(&mut orchestrator)?;
    let baseline = (
        orchestrator.runtime_event_count(),
        orchestrator.cloud_infrastructure_events().len(),
        orchestrator
            .cloud_infrastructure_attribution_records()
            .len(),
        orchestrator.task_count(),
        orchestrator.run_count(),
    );
    let foreign = RuntimeTurnRequest::new("foreign-run", "foreign-request", "bounded")?;
    let foreign_event = RuntimeEventEnvelope::for_request(
        &foreign,
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("foreign-response")?,
        },
    );
    assert_eq!(
        orchestrator.accept_runtime_event(cloud.task_id(), foreign_event),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::IdentityMismatch)
        ))
    );
    assert_eq!(
        (
            orchestrator.runtime_event_count(),
            orchestrator.cloud_infrastructure_events().len(),
            orchestrator
                .cloud_infrastructure_attribution_records()
                .len(),
            orchestrator.task_count(),
            orchestrator.run_count(),
        ),
        baseline
    );

    orchestrator.accept_runtime_event(cloud.task_id(), started(&cloud, "cloud-live")?)?;
    let after_started = (
        orchestrator.runtime_event_count(),
        orchestrator.cloud_infrastructure_events().len(),
        orchestrator
            .cloud_infrastructure_attribution_records()
            .len(),
    );
    let replay = RuntimeEventEnvelope::for_identity(
        cloud.runtime_run_identity(),
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("replayed-response")?,
        },
    );
    assert_eq!(
        orchestrator.accept_runtime_event(cloud.task_id(), replay),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::InvalidSequence)
        ))
    );
    assert_eq!(
        (
            orchestrator.runtime_event_count(),
            orchestrator.cloud_infrastructure_events().len(),
            orchestrator
                .cloud_infrastructure_attribution_records()
                .len(),
        ),
        after_started
    );
    Ok(())
}
