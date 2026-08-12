use std::error::Error;

use ai_agent_assistant_lib::{
    agent::{
        definition::AgentId,
        governance::{
            AgentApprovalAuditDisposition, AgentExecutionDisposition, AgentGovernanceAction,
            AgentGovernanceError, AgentPolicyReason, AgentToolGovernanceOutcome, AgentToolProposal,
        },
        orchestrator::{AgentOrchestrator, AgentOrchestratorError, DelegationProposal},
        runtime::{RuntimeEventEnvelope, RuntimeResponseId, UntrustedRuntimeEvent},
        task::{AgentTaskCancellationOutcome, AgentTaskStatus},
    },
    approvals::types::ApprovalOrigin,
    audit::governance::{
        AgentDelegationGovernanceLifecycleState, AgentGovernanceRecord,
        AgentToolGovernanceLifecycleState, MAX_AGENT_GOVERNANCE_RECORDS,
    },
    policy::types::{PolicyOutcome, PolicyReason},
};

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

fn datetime(call_id: &str) -> Result<AgentToolProposal, AgentGovernanceError> {
    AgentToolProposal::new(call_id, "get_current_datetime", 1, "{}")
}

fn local_task(call_id: &str) -> Result<AgentToolProposal, AgentGovernanceError> {
    AgentToolProposal::new(
        call_id,
        "create_local_task",
        1,
        r#"{"title":"Review the bounded fixture"}"#,
    )
}

fn delegation() -> Result<DelegationProposal, Box<dyn Error>> {
    Ok(DelegationProposal::new(
        AgentId::Research,
        "Analyze the bounded fixture",
        Some("Use only the supplied evidence".to_owned()),
        "Return one attributed summary",
    )?)
}

#[test]
fn personal_policy_is_deterministic_and_never_executes() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Answer one bounded question")?;

    let outcome = orchestrator.govern_tool_proposal(&root, datetime("call-datetime")?)?;
    assert_eq!(
        outcome,
        AgentToolGovernanceOutcome::Final {
            policy_outcome: PolicyOutcome::Allow,
            policy_reason: AgentPolicyReason::Deterministic(PolicyReason::InformationOnly),
            approval: AgentApprovalAuditDisposition::NotRequired,
            execution: AgentExecutionDisposition::NotAttempted,
        }
    );

    let records = orchestrator.governance_audit_records();
    assert!(matches!(
        records.as_slice(),
        [AgentGovernanceRecord::Tool(record)]
            if record.action() == AgentGovernanceAction::GetCurrentDatetime
                && record.lifecycle() == AgentToolGovernanceLifecycleState::PolicyEvaluated
                && record.execution() == AgentExecutionDisposition::NotAttempted
    ));
    Ok(())
}

#[test]
fn approval_retains_agent_origin_blocks_events_and_cancels_before_task(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Create one bounded local task")?;
    let root_id = root.task_id().clone();

    let outcome = orchestrator.govern_tool_proposal(&root, local_task("call-local-task")?)?;
    assert!(matches!(
        outcome,
        AgentToolGovernanceOutcome::ApprovalPending { .. }
    ));
    let pending = orchestrator
        .pending_governance_approval(&root_id)?
        .ok_or("missing pending approval")?;
    assert!(matches!(
        pending.origin(),
        ApprovalOrigin::Agent(attribution) if attribution.task_id() == &root_id
    ));
    drop(pending);
    let presentation = orchestrator.issue_governance_presentation(&root)?;
    assert!(matches!(
        presentation.origin(),
        ApprovalOrigin::Agent(attribution) if attribution.task_id() == &root_id
    ));
    drop(presentation);
    assert_eq!(
        orchestrator.request_delegation(&root, delegation()?),
        Err(AgentOrchestratorError::GovernanceApprovalPending)
    );

    let envelope = RuntimeEventEnvelope::for_identity(
        root.runtime_run_identity(),
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("pending-response")?,
        },
    );
    assert_eq!(
        orchestrator.accept_runtime_event(&root_id, envelope),
        Err(AgentOrchestratorError::GovernanceApprovalPending)
    );
    assert_eq!(orchestrator.runtime_event_count(), 0);
    assert_eq!(
        orchestrator.task(&root_id).map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );

    assert_eq!(
        orchestrator.cancel_task(&root_id)?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert!(orchestrator
        .pending_governance_approval(&root_id)?
        .is_none());
    let records = orchestrator.governance_audit_records();
    assert!(records.iter().any(|record| matches!(
        record,
        AgentGovernanceRecord::Tool(record)
            if record.lifecycle() == AgentToolGovernanceLifecycleState::ApprovalResolved
                && record.approval() == AgentApprovalAuditDisposition::Cancelled
                && record.execution() == AgentExecutionDisposition::NotAttempted
    )));
    assert!(records.iter().any(|record| matches!(
        record,
        AgentGovernanceRecord::Delegation(record)
            if record.lifecycle() == AgentDelegationGovernanceLifecycleState::Denied
                && record.matrix()
                    == ai_agent_assistant_lib::agent::governance::DelegationMatrixOutcome::Allowed
    )));
    Ok(())
}

#[test]
fn validation_rejections_are_typed_audited_and_non_executing() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Answer one bounded question")?;

    assert!(matches!(
        orchestrator.govern_tool_proposal(
            &root,
            AgentToolProposal::new("unknown-call", "unknown_tool", 1, "{}")?,
        ),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::UnknownTool
        ))
    ));
    assert!(matches!(
        orchestrator.govern_tool_proposal(
            &root,
            AgentToolProposal::new("version-call", "get_current_datetime", 2, "{}")?,
        ),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::ToolContractVersionMismatch { .. }
        ))
    ));
    assert!(matches!(
        orchestrator.govern_tool_proposal(
            &root,
            AgentToolProposal::new("arguments-call", "get_current_datetime", 1, "{bad")?,
        ),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::InvalidToolArguments
        ))
    ));
    let records = orchestrator.governance_audit_records();
    assert_eq!(records.len(), 3);
    assert!(records.iter().all(|record| matches!(
        record,
        AgentGovernanceRecord::Tool(record)
            if record.lifecycle() == AgentToolGovernanceLifecycleState::ValidationRejected
                && record.execution() == AgentExecutionDisposition::NotAttempted
    )));
    Ok(())
}

#[test]
fn audit_capacity_fails_before_policy_or_approval_mutation() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Answer one bounded question")?;
    for index in 0..MAX_AGENT_GOVERNANCE_RECORDS {
        orchestrator.govern_tool_proposal(&root, datetime(&format!("capacity-{index}"))?)?;
    }
    assert_eq!(
        orchestrator.govern_tool_proposal(&root, datetime("capacity-overflow")?),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::AuditCapacityExhausted {
                maximum: MAX_AGENT_GOVERNANCE_RECORDS,
            }
        ))
    );
    assert_eq!(
        orchestrator.governance_audit_records().len(),
        MAX_AGENT_GOVERNANCE_RECORDS
    );
    assert!(orchestrator
        .pending_governance_approval(root.task_id())?
        .is_none());
    Ok(())
}

#[test]
fn runtime_cancel_failure_after_approval_cancellation_leaves_live_retryable_task(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::CancelFailureAt(1)))?;
    let root = orchestrator.start_root("Create one bounded local task")?;
    let root_id = root.task_id().clone();
    orchestrator.govern_tool_proposal(&root, local_task("cancel-failure")?)?;

    assert!(matches!(
        orchestrator.cancel_task(&root_id),
        Err(AgentOrchestratorError::Runtime(_))
    ));
    assert_eq!(
        orchestrator.task(&root_id).map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(orchestrator.current_context(&root_id).is_ok());
    assert!(orchestrator
        .pending_governance_approval(&root_id)?
        .is_none());
    assert!(orchestrator
        .governance_audit_records()
        .iter()
        .any(|record| matches!(
            record,
            AgentGovernanceRecord::Tool(record)
                if record.approval() == AgentApprovalAuditDisposition::Cancelled
        )));
    Ok(())
}

#[test]
fn specialist_profiles_deny_current_tools_without_execution() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Answer one bounded question")?;
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();

    assert_eq!(
        orchestrator.govern_tool_proposal(&child, datetime("research-call")?)?,
        AgentToolGovernanceOutcome::Final {
            policy_outcome: PolicyOutcome::Deny,
            policy_reason: AgentPolicyReason::ProfileNotEligible,
            approval: AgentApprovalAuditDisposition::NotRequired,
            execution: AgentExecutionDisposition::NotAttempted,
        }
    );
    Ok(())
}

#[test]
fn replay_and_foreign_context_fail_before_new_authority() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Answer one bounded question")?;
    assert_eq!(
        orchestrator.govern_tool_proposal(
            &root,
            AgentToolProposal::new("replay-call", "not_registered", 1, "{}")?,
        ),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::UnknownTool
        ))
    );
    assert_eq!(
        orchestrator.govern_tool_proposal(&root, datetime("replay-call")?),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::DuplicateSubject
        ))
    );
    assert_eq!(orchestrator.governance_audit_records().len(), 1);

    let mut foreign = AgentOrchestrator::native()?;
    let foreign_context = foreign.start_root("Foreign bounded question")?;
    assert_eq!(
        orchestrator.govern_tool_proposal(&foreign_context, datetime("foreign-call")?),
        Err(AgentOrchestratorError::TaskNotFound)
    );
    assert_eq!(orchestrator.governance_audit_records().len(), 1);
    Ok(())
}

#[test]
fn replay_is_rejected_after_allow_deny_pending_and_terminal_task() -> Result<(), Box<dyn Error>> {
    let mut allowed = AgentOrchestrator::native()?;
    let allowed_root = allowed.start_root("Answer one bounded question")?;
    allowed.govern_tool_proposal(&allowed_root, datetime("allow-replay")?)?;
    assert_eq!(
        allowed.govern_tool_proposal(&allowed_root, local_task("allow-replay")?),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::DuplicateSubject
        ))
    );
    assert_eq!(allowed.governance_audit_records().len(), 1);

    let mut pending = AgentOrchestrator::native()?;
    let pending_root = pending.start_root("Create one bounded local task")?;
    pending.govern_tool_proposal(&pending_root, local_task("approval-replay")?)?;
    assert_eq!(
        pending.govern_tool_proposal(&pending_root, datetime("approval-replay")?),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::ApprovalPending
        ))
    );
    assert_eq!(pending.governance_audit_records().len(), 1);
    pending.cancel_task(pending_root.task_id())?;
    assert_eq!(
        pending.govern_tool_proposal(&pending_root, datetime("approval-replay")?),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    assert_eq!(pending.governance_audit_records().len(), 1);

    let mut denied = AgentOrchestrator::native()?;
    let root = denied.start_root("Answer one bounded question")?;
    let child = denied
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    denied.govern_tool_proposal(&child, datetime("deny-replay")?)?;
    assert_eq!(
        denied.govern_tool_proposal(&child, local_task("deny-replay")?),
        Err(AgentOrchestratorError::Governance(
            AgentGovernanceError::DuplicateSubject
        ))
    );
    assert_eq!(denied.governance_audit_records().len(), 2);
    Ok(())
}

#[test]
fn delegation_uses_a_separate_audited_matrix_path() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Answer one bounded question")?;
    let acceptance = orchestrator.request_delegation(&root, delegation()?)?;
    assert_eq!(
        acceptance.request().source_attribution().task_id(),
        root.task_id()
    );
    assert_eq!(
        acceptance
            .request()
            .source_attribution()
            .policy_profile_id(),
        root.policy_profile_id()
    );

    let records = orchestrator.governance_audit_records();
    assert!(matches!(
        records.as_slice(),
        [AgentGovernanceRecord::Delegation(record)]
            if record.target_agent_id() == AgentId::Research
                && record.lifecycle() == AgentDelegationGovernanceLifecycleState::ChildCreated
                && record.approval() == AgentApprovalAuditDisposition::NotRequired
    ));
    Ok(())
}

#[test]
fn governance_debug_surfaces_redact_untrusted_proposal_content() -> Result<(), Box<dyn Error>> {
    let call_id = "private-call-id";
    let tool_name = "private_tool_name";
    let arguments = r#"{"private":"value"}"#;
    let proposal = AgentToolProposal::new(call_id, tool_name, 1, arguments)?;
    let debug = format!("{proposal:?}");
    assert!(!debug.contains(call_id));
    assert!(!debug.contains(tool_name));
    assert!(!debug.contains(arguments));
    Ok(())
}
