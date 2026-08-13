use std::{error::Error, fs};

use ai_agent_assistant_lib::agent::{
    bounded_parallelism::{
        BoundedParallelAuditOutcome, BoundedParallelError, BoundedParallelFailurePolicy,
        BoundedParallelScenarioCatalog, BoundedParallelScenarioId, BoundedParallelTerminalStatus,
        BoundedParallelWorkflowEvent, BoundedParallelWorkflowRequest,
        BoundedParallelWorkflowStatus, ParallelChildDisposition, ParallelChildResultStatus,
        BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE, DEFAULT_ACTIVE_PARALLEL_CHILDREN,
        HARD_MAX_ACTIVE_PARALLEL_CHILDREN, MAX_PARALLEL_CHILDREN, MAX_PARALLEL_EVENTS_PER_RUN,
        MAX_PARALLEL_RAW_RESULT_BYTES, MAX_PARALLEL_RAW_RESULT_CHARACTERS, MAX_PARALLEL_RETRIES,
        MAX_PARALLEL_SYNTHESIS_OUTPUT_BYTES, MAX_PARALLEL_SYNTHESIS_OUTPUT_CHARACTERS,
    },
    definition::AgentId,
    governance::{AgentPolicyProfileId, AgentToolProposal},
    native_runtime::NativeAgentRuntime,
    orchestrator::{
        AgentOrchestrator, AgentOrchestratorError, AgentWorkflowSelection, DelegationProposal,
    },
    runtime::{
        RuntimeError, RuntimeEventAcceptance, RuntimeEventEnvelope, RuntimeEventRejection,
        RuntimeFailure, RuntimeFailureCode, RuntimeId, RuntimeOutputText, RuntimeResponseId,
        UntrustedRuntimeEvent, UntrustedRuntimeToolProposal,
    },
    task::{AgentExecutionContext, AgentTaskCancellationOutcome, AgentTaskStatus},
};
use ai_agent_assistant_lib::documents::{
    ApprovedDocumentId, ApprovedDocumentSource, ApprovedRelativePath, ApprovedRootId,
    DocumentOperation,
};
use ai_agent_assistant_lib::memory::{
    AgentMemoryProfileId, MemoryContent, MemoryContextSelection, MemoryRecordId,
    MemoryRecordVersion, MemoryStoreError, MemoryWriteTarget, SharedMemoryProposalId,
};
use serde_json::json;
use tempfile::tempdir;

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

fn child_result(
    scenario_id: &str,
    work_item_id: &str,
    fixture_id: &str,
    statement: &str,
) -> String {
    format!(
        r#"{{"version":"v1","scenario_id":"{scenario_id}","work_item_id":"{work_item_id}","summary":"Fixture evidence supports the bounded finding.","findings":[{{"id":"bounded-finding","statement":"{statement}","confidence":"high","references":[{{"namespace":"fixture","id":"{fixture_id}"}}]}}],"unresolved_issues":[],"fixture_based":true,"live_access_performed":false,"tools_executed":false,"credentials_loaded":false,"effects_performed":false}}"#
    )
}

fn child_result_with_issues(
    scenario_id: &str,
    work_item_id: &str,
    fixture_id: &str,
    statement: &str,
    issues: &[&str],
) -> Result<String, serde_json::Error> {
    let mut value: serde_json::Value = serde_json::from_str(&child_result(
        scenario_id,
        work_item_id,
        fixture_id,
        statement,
    ))?;
    value["unresolved_issues"] = json!(issues);
    serde_json::to_string(&value)
}

fn child_result_for(
    scenario_id: BoundedParallelScenarioId,
    work_item_id: &str,
) -> Result<String, Box<dyn Error>> {
    let (scenario, fixture, statement) = match (scenario_id, work_item_id) {
        (BoundedParallelScenarioId::ResearchKnowledgeIndependentV1, "research-analysis") => (
            "research-knowledge-independent-v1",
            "approach-a",
            "The fixture supports the bounded research comparison.",
        ),
        (BoundedParallelScenarioId::ResearchKnowledgeIndependentV1, "knowledge-analysis") => (
            "research-knowledge-independent-v1",
            "decision-context",
            "The fixture preserves bounded decision evidence.",
        ),
        (BoundedParallelScenarioId::CodeSecurityQaV1, "coding-review") => (
            "code-security-qa-v1",
            "proposal-fixture",
            "The fixture describes an inert off-by-one proposal.",
        ),
        (BoundedParallelScenarioId::CodeSecurityQaV1, "security-review") => (
            "code-security-qa-v1",
            "threat-fixture",
            "The fixture grants no execution authority.",
        ),
        (BoundedParallelScenarioId::CodeSecurityQaV1, "qa-validation") => (
            "code-security-qa-v1",
            "qa-criterion",
            "The fixture predecessors are attributed and proposal-only.",
        ),
        (BoundedParallelScenarioId::CloudSystemsSecurityV1, "cloud-assessment") => (
            "cloud-systems-security-v1",
            "cloud-fixture",
            "The fixture cloud facet contains no live provider access.",
        ),
        (BoundedParallelScenarioId::CloudSystemsSecurityV1, "systems-assessment") => (
            "cloud-systems-security-v1",
            "systems-fixture",
            "The sanitized fixture preserves one bounded recovery observation.",
        ),
        (BoundedParallelScenarioId::CloudSystemsSecurityV1, "security-assessment") => (
            "cloud-systems-security-v1",
            "security-criterion",
            "The fixture review preserves residual risk without remediation authority.",
        ),
        _ => return Err("unknown sealed scenario/work-item pair".into()),
    };
    Ok(child_result(scenario, work_item_id, fixture, statement))
}

fn scenario_wire_id(scenario_id: BoundedParallelScenarioId) -> &'static str {
    match scenario_id {
        BoundedParallelScenarioId::ResearchKnowledgeIndependentV1 => {
            "research-knowledge-independent-v1"
        }
        BoundedParallelScenarioId::CodeSecurityQaV1 => "code-security-qa-v1",
        BoundedParallelScenarioId::CloudSystemsSecurityV1 => "cloud-systems-security-v1",
    }
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

fn accept_deltas(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    response_id: &str,
    deltas: &[String],
) -> Result<(), Box<dyn Error>> {
    orchestrator.accept_runtime_event(context.task_id(), started(context, response_id)?)?;
    for (index, value) in deltas.iter().enumerate() {
        orchestrator.accept_runtime_event(
            context.task_id(),
            RuntimeEventEnvelope::for_identity(
                context.runtime_run_identity(),
                u32::try_from(index + 1)?,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: RuntimeOutputText::new(value.clone())?,
                },
            ),
        )?;
    }
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

fn synthesis_result(
    scenario_id: &str,
    rows: &[(u8, &str, AgentId, ParallelChildResultStatus)],
) -> String {
    let partial = rows
        .iter()
        .any(|(_, _, _, status)| *status != ParallelChildResultStatus::Succeeded);
    let issues = rows
        .iter()
        .filter(|(_, _, _, status)| *status != ParallelChildResultStatus::Succeeded)
        .map(|(_, id, _, status)| {
            let label = match status {
                ParallelChildResultStatus::Succeeded => "succeeded",
                ParallelChildResultStatus::Failed => "failed",
                ParallelChildResultStatus::Cancelled => "cancelled",
                ParallelChildResultStatus::TimedOut => "timed-out",
                ParallelChildResultStatus::Skipped => "skipped",
            };
            format!("{id} is {label}")
        })
        .collect::<Vec<_>>();
    json!({
        "version": "v1",
        "scenario_id": scenario_id,
        "status": if partial { "partial" } else { "complete" },
        "source_results": rows.iter().map(|(ordinal, id, agent, status)| json!({
            "ordinal": ordinal,
            "work_item_id": id,
            "agent_id": agent.as_str(),
            "status": match status {
                ParallelChildResultStatus::Succeeded => "succeeded",
                ParallelChildResultStatus::Failed => "failed",
                ParallelChildResultStatus::Cancelled => "cancelled",
                ParallelChildResultStatus::TimedOut => "timed-out",
                ParallelChildResultStatus::Skipped => "skipped",
            },
        })).collect::<Vec<_>>(),
        "source_findings": rows.iter().map(|(ordinal, id, agent, status)| json!({
            "ordinal": ordinal,
            "work_item_id": id,
            "agent_id": agent.as_str(),
            "finding_ids": if *status == ParallelChildResultStatus::Succeeded {
                vec!["bounded-finding"]
            } else {
                Vec::<&str>::new()
            },
        })).collect::<Vec<_>>(),
        "summary": if partial {
            format!("Partial. {BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE}")
        } else {
            BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE.to_owned()
        },
        "unresolved_issues": issues,
        "fixture_based": true,
        "live_access_performed": false,
        "tools_executed": false,
        "credentials_loaded": false,
        "effects_performed": false,
    })
    .to_string()
}

fn synthesis_result_with_issues(
    scenario_id: &str,
    rows: &[(u8, &str, AgentId, ParallelChildResultStatus)],
    issues: &[&str],
) -> Result<String, serde_json::Error> {
    let mut value: serde_json::Value = serde_json::from_str(&synthesis_result(scenario_id, rows))?;
    value["unresolved_issues"] = json!(issues);
    serde_json::to_string(&value)
}

fn synthesis_for_request(
    request: &BoundedParallelWorkflowRequest,
    statuses: &[ParallelChildResultStatus],
) -> String {
    assert_eq!(request.work_items().len(), statuses.len());
    let rows = request
        .work_items()
        .iter()
        .zip(statuses)
        .map(|(item, status)| (item.ordinal(), item.id().as_str(), item.agent_id(), *status))
        .collect::<Vec<_>>();
    synthesis_result(scenario_wire_id(request.scenario_id()), &rows)
}

fn complete_all_active_children(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    scenario_id: BoundedParallelScenarioId,
) -> Result<(), Box<dyn Error>> {
    loop {
        let controls = orchestrator.bounded_parallel_active_children()?;
        if controls.is_empty() {
            return Ok(());
        }
        for control in controls {
            let work_item_id = control.work_item_id().as_str().to_owned();
            let context = control.context().clone();
            complete_stage(
                orchestrator,
                &context,
                &format!("{work_item_id}-complete"),
                &child_result_for(scenario_id, &work_item_id)?,
            )?;
        }
    }
}

fn assert_bounded_parallel_no_io_authority_denied(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    approved_root_id: &ApprovedRootId,
    approved_document_id: &ApprovedDocumentId,
    proposal_id: &SharedMemoryProposalId,
    proposal_version: MemoryRecordVersion,
    path_suffix: &str,
) -> Result<(), Box<dyn Error>> {
    let proposal_before = orchestrator.shared_memory_proposal(proposal_id)?;
    let state_before = (
        format!("{orchestrator:?}"),
        orchestrator.runtime_event_count(),
        orchestrator.events().len(),
        orchestrator.bounded_parallel_events().len(),
        orchestrator.bounded_parallel_audit_records().len(),
        orchestrator.governance_audit_records().len(),
        orchestrator.shared_memory_proposal_count(),
        orchestrator.root_task().map(|task| task.status()),
        orchestrator.bounded_parallel_status(),
    );

    assert!(matches!(
        orchestrator.register_approved_document(
            context,
            ApprovedDocumentSource::UserSelectedFile,
            format!("/d091-denied/{path_suffix}-document.txt"),
        ),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    ));
    assert!(matches!(
        orchestrator
            .register_approved_root(context, format!("/d091-denied/{path_suffix}-approved-root"),),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    ));
    assert!(matches!(
        orchestrator.register_approved_root_member(
            context,
            approved_root_id,
            ApprovedRelativePath::new(format!("{path_suffix}-missing-member.txt"))?,
        ),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    ));
    assert!(matches!(
        orchestrator.request_delegation(
            context,
            DelegationProposal::new(
                AgentId::Research,
                "Inspect the bounded fixture without execution authority",
                None,
                "Return one attributed fixture finding",
            )?,
        ),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    ));
    assert!(matches!(
        orchestrator.request_document_task(
            context,
            approved_document_id,
            DocumentOperation::Read,
            None,
        ),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    ));
    assert_eq!(
        orchestrator.withdraw_shared_memory_proposal(context, proposal_id, proposal_version),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    );

    assert_eq!(
        (
            format!("{orchestrator:?}"),
            orchestrator.runtime_event_count(),
            orchestrator.events().len(),
            orchestrator.bounded_parallel_events().len(),
            orchestrator.bounded_parallel_audit_records().len(),
            orchestrator.governance_audit_records().len(),
            orchestrator.shared_memory_proposal_count(),
            orchestrator.root_task().map(|task| task.status()),
            orchestrator.bounded_parallel_status(),
        ),
        state_before
    );
    assert_eq!(
        orchestrator.shared_memory_proposal(proposal_id)?,
        proposal_before
    );
    Ok(())
}

fn assert_cancelling_parallel_memory_ingress_denied(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &AgentExecutionContext,
    record_id: &MemoryRecordId,
    record_version: MemoryRecordVersion,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    let selection = MemoryContextSelection::new([record_id.clone()])?;
    assert_eq!(
        orchestrator.write_memory(
            context,
            MemoryWriteTarget::TaskTemporary,
            MemoryContent::new(content)?,
        ),
        Err(AgentOrchestratorError::BoundedParallelCancellationPending)
    );
    assert_eq!(
        orchestrator.read_memory(context, record_id),
        Err(AgentOrchestratorError::BoundedParallelCancellationPending)
    );
    assert_eq!(
        orchestrator.select_memory_context(context, &selection),
        Err(AgentOrchestratorError::BoundedParallelCancellationPending)
    );
    assert_eq!(
        orchestrator.delete_memory(context, record_id, record_version),
        Err(AgentOrchestratorError::BoundedParallelCancellationPending)
    );
    Ok(())
}

#[test]
fn built_in_scenarios_are_closed_bounded_and_deterministic() -> Result<(), Box<dyn Error>> {
    let catalog = BoundedParallelScenarioCatalog::built_in();
    let cases = [
        (
            BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
            BoundedParallelFailurePolicy::ContinuePartial,
            vec![
                (
                    1,
                    "research-analysis",
                    AgentId::Research,
                    Vec::<&str>::new(),
                ),
                (
                    2,
                    "knowledge-analysis",
                    AgentId::KnowledgeDocument,
                    Vec::<&str>::new(),
                ),
            ],
        ),
        (
            BoundedParallelScenarioId::CodeSecurityQaV1,
            BoundedParallelFailurePolicy::CancelDependentOnly,
            vec![
                (1, "coding-review", AgentId::Coding, vec![]),
                (2, "security-review", AgentId::SecurityRisk, vec![]),
                (
                    3,
                    "qa-validation",
                    AgentId::QaValidation,
                    vec!["coding-review", "security-review"],
                ),
            ],
        ),
        (
            BoundedParallelScenarioId::CloudSystemsSecurityV1,
            BoundedParallelFailurePolicy::FailFast,
            vec![
                (1, "cloud-assessment", AgentId::CloudInfrastructure, vec![]),
                (2, "systems-assessment", AgentId::SystemsOperations, vec![]),
                (
                    3,
                    "security-assessment",
                    AgentId::SecurityRisk,
                    vec!["cloud-assessment", "systems-assessment"],
                ),
            ],
        ),
    ];

    for (scenario_id, expected_policy, expected_items) in cases {
        let first = catalog.request(scenario_id)?;
        let second = catalog.request(scenario_id)?;
        assert_eq!(first.scenario_id(), scenario_id);
        assert_eq!(first.failure_policy(), expected_policy);
        assert_eq!(first.active_limit(), DEFAULT_ACTIVE_PARALLEL_CHILDREN);
        assert!(first.work_items().len() <= MAX_PARALLEL_CHILDREN);
        assert_eq!(format!("{first:?}"), format!("{second:?}"));
        assert_eq!(first.work_items().len(), expected_items.len());

        for (item, (ordinal, id, agent_id, dependencies)) in
            first.work_items().iter().zip(expected_items)
        {
            assert_eq!(item.ordinal(), ordinal);
            assert_eq!(item.id().as_str(), id);
            assert_eq!(item.agent_id(), agent_id);
            assert_eq!(
                item.dependencies()
                    .iter()
                    .map(|dependency| dependency.as_str())
                    .collect::<Vec<_>>(),
                dependencies
            );
            assert!(!item.fixtures().is_empty());
        }
    }

    assert_eq!(DEFAULT_ACTIVE_PARALLEL_CHILDREN, 2);
    assert_eq!(HARD_MAX_ACTIVE_PARALLEL_CHILDREN, 3);
    assert_eq!(MAX_PARALLEL_CHILDREN, 3);
    assert_eq!(MAX_PARALLEL_EVENTS_PER_RUN, 8);
    assert_eq!(MAX_PARALLEL_RETRIES, 0);
    Ok(())
}

#[test]
fn application_selected_active_limits_accept_one_through_three_and_reject_zero_or_four(
) -> Result<(), Box<dyn Error>> {
    let catalog = BoundedParallelScenarioCatalog::built_in();
    for active_limit in 1..=HARD_MAX_ACTIVE_PARALLEL_CHILDREN {
        let request = catalog
            .request_with_active_limit(BoundedParallelScenarioId::CodeSecurityQaV1, active_limit)?;
        assert_eq!(request.active_limit(), active_limit);
    }
    for active_limit in [0, HARD_MAX_ACTIVE_PARALLEL_CHILDREN + 1] {
        assert_eq!(
            catalog.request_with_active_limit(
                BoundedParallelScenarioId::CodeSecurityQaV1,
                active_limit,
            ),
            Err(BoundedParallelError::ActiveLimitExceeded)
        );
    }
    Ok(())
}

#[test]
fn child_results_are_strict_fixture_only_and_identity_bound() -> Result<(), Box<dyn Error>> {
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let raw = child_result(
        "research-knowledge-independent-v1",
        "research-analysis",
        "approach-a",
        "The fixture records lower bounded operational complexity.",
    );
    let result = request.parse_child_result(1, &raw)?;
    assert_eq!(result.work_item_id().as_str(), "research-analysis");
    assert_eq!(result.findings().len(), 1);
    assert_eq!(result.findings()[0].references()[0].namespace(), "fixture");
    assert_eq!(result.findings()[0].references()[0].id(), "approach-a");

    let wrong_item = raw.replace("research-analysis", "knowledge-analysis");
    assert_eq!(
        request.parse_child_result(1, &wrong_item),
        Err(BoundedParallelError::ResultIdentityMismatch)
    );
    let unknown_fixture = raw.replace("approach-a", "unknown-fixture");
    assert_eq!(
        request.parse_child_result(1, &unknown_fixture),
        Err(BoundedParallelError::UnknownFixtureReference)
    );
    let authority_claim = raw.replace("\"tools_executed\":false", "\"tools_executed\":true");
    assert_eq!(
        request.parse_child_result(1, &authority_claim),
        Err(BoundedParallelError::AuthorityClaim)
    );
    let unknown_field = raw.replace(
        "\"effects_performed\":false",
        "\"effects_performed\":false,\"shell\":\"echo unsafe\"",
    );
    assert_eq!(
        request.parse_child_result(1, &unknown_field),
        Err(BoundedParallelError::InvalidStructuredResult)
    );
    let duplicate_key = raw.replace(
        "\"version\":\"v1\"",
        "\"version\":\"v1\",\"version\":\"v1\"",
    );
    assert_eq!(
        request.parse_child_result(1, &duplicate_key),
        Err(BoundedParallelError::InvalidStructuredResult)
    );
    Ok(())
}

#[test]
fn child_input_is_fixture_only_and_contains_no_execution_authority() -> Result<(), Box<dyn Error>> {
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::CodeSecurityQaV1)?;
    let input = request.build_child_input(1, &[])?;
    assert!(input.contains("same-thread-event-multiplexed"));
    assert!(input.contains("fixture-only"));
    assert!(input.contains("proposal-fixture"));
    assert!(input.contains("Do not include reasoning"));
    assert!(!input.contains("execute_action"));
    assert!(!input.contains("approval_granted=true"));
    assert_eq!(
        request.build_child_input(3, &[]),
        Err(BoundedParallelError::DependencyUnavailable)
    );
    assert_eq!(
        ParallelChildResultStatus::Succeeded,
        ParallelChildResultStatus::Succeeded
    );
    Ok(())
}

#[test]
fn child_streaming_caps_accept_exact_character_and_byte_limits_then_close_at_n_plus_one(
) -> Result<(), Box<dyn Error>> {
    let cases = [
        vec!["x".repeat(MAX_PARALLEL_RAW_RESULT_CHARACTERS)],
        vec!["€".repeat(2_048), "€".repeat(2_048)],
    ];
    assert_eq!(cases[1].concat().len(), MAX_PARALLEL_RAW_RESULT_BYTES);
    for (case_index, exact_deltas) in cases.into_iter().enumerate() {
        let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let child = acceptance.active_contexts()[0].clone();
        accept_deltas(
            &mut orchestrator,
            &child,
            &format!("child-output-boundary-{case_index}"),
            &exact_deltas,
        )?;
        assert_eq!(
            orchestrator.task(child.task_id()).map(|task| task.status()),
            Some(AgentTaskStatus::Running)
        );
        let before_runs = orchestrator.run_count();
        let overflow_sequence = u32::try_from(exact_deltas.len() + 1)?;
        assert_eq!(
            orchestrator.accept_runtime_event(
                child.task_id(),
                RuntimeEventEnvelope::for_identity(
                    child.runtime_run_identity(),
                    overflow_sequence,
                    UntrustedRuntimeEvent::OutputTextDelta {
                        delta: RuntimeOutputText::new("a")?,
                    },
                ),
            ),
            Err(AgentOrchestratorError::OutputLimitExceeded)
        );
        assert_eq!(
            orchestrator.task(child.task_id()).map(|task| task.status()),
            Some(AgentTaskStatus::Failed)
        );
        assert_eq!(orchestrator.run_count(), before_runs);
        assert_eq!(orchestrator.bounded_parallel_active_children()?.len(), 1);
        assert!(!orchestrator.bounded_parallel_events().iter().any(|event| matches!(
            event,
            ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::SynthesisStarted { .. }
        )));
        orchestrator.cancel_task(root.task_id())?;
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn synthesis_streaming_caps_accept_exact_character_and_byte_limits_then_close_at_n_plus_one(
) -> Result<(), Box<dyn Error>> {
    let cases = [
        vec!["x".repeat(MAX_PARALLEL_SYNTHESIS_OUTPUT_CHARACTERS)],
        vec!["é".repeat(MAX_PARALLEL_SYNTHESIS_OUTPUT_CHARACTERS)],
    ];
    assert_eq!(cases[1][0].len(), MAX_PARALLEL_SYNTHESIS_OUTPUT_BYTES);
    for (case_index, exact_deltas) in cases.into_iter().enumerate() {
        let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
        let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
        let root = orchestrator.start_root(request.objective())?;
        let root_id = root.task_id().clone();
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        complete_all_active_children(&mut orchestrator, scenario_id)?;
        let synthesis = orchestrator.current_context(&root_id)?;
        accept_deltas(
            &mut orchestrator,
            &synthesis,
            &format!("synthesis-output-boundary-{case_index}"),
            &exact_deltas,
        )?;
        assert_eq!(
            orchestrator.root_task().map(|task| task.status()),
            Some(AgentTaskStatus::Running)
        );
        assert_eq!(
            orchestrator.accept_runtime_event(
                &root_id,
                RuntimeEventEnvelope::for_identity(
                    synthesis.runtime_run_identity(),
                    u32::try_from(exact_deltas.len() + 1)?,
                    UntrustedRuntimeEvent::OutputTextDelta {
                        delta: RuntimeOutputText::new("a")?,
                    },
                ),
            ),
            Err(AgentOrchestratorError::OutputLimitExceeded)
        );
        assert_eq!(
            orchestrator.root_task().map(|task| task.status()),
            Some(AgentTaskStatus::Failed)
        );
        assert!(orchestrator.bounded_parallel_result().is_none());
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn public_debug_output_redacts_fixture_and_result_content() -> Result<(), Box<dyn Error>> {
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::CloudSystemsSecurityV1)?;
    let result = request.parse_child_result(
        1,
        &child_result(
            "cloud-systems-security-v1",
            "cloud-assessment",
            "cloud-fixture",
            "Sensitive fixture sentinel supports the bounded observation.",
        ),
    )?;
    let debug = format!("{request:?} {:?} {:?}", request.work_items(), result);
    assert!(!debug.contains("private subnet"));
    assert!(!debug.contains("Sensitive fixture sentinel"));
    assert!(!debug.contains("Fixture evidence supports"));
    Ok(())
}

#[test]
fn bounded_parallel_memory_is_task_isolated_cleaned_and_disabled_outside_scenario_a(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let research = acceptance.active_contexts()[0].clone();
    let knowledge = acceptance.active_contexts()[1].clone();
    let research_record = orchestrator.write_memory(
        &research,
        MemoryWriteTarget::TaskTemporary,
        MemoryContent::new("bounded research temporary memory")?,
    )?;
    let knowledge_record = orchestrator.write_memory(
        &knowledge,
        MemoryWriteTarget::TaskTemporary,
        MemoryContent::new("bounded knowledge temporary memory")?,
    )?;
    assert_eq!(
        orchestrator.read_memory(&research, knowledge_record.id()),
        Err(AgentOrchestratorError::Memory(
            MemoryStoreError::AccessDenied
        ))
    );
    assert_eq!(
        orchestrator.read_memory(&knowledge, research_record.id()),
        Err(AgentOrchestratorError::Memory(
            MemoryStoreError::AccessDenied
        ))
    );
    assert_eq!(
        orchestrator.propose_shared_memory(
            &research,
            MemoryContent::new("must not become a shared proposal")?,
        ),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    );
    assert_eq!(orchestrator.shared_memory_proposal_count(), 0);

    complete_stage(
        &mut orchestrator,
        &research,
        "research-memory-cleanup",
        &child_result_for(scenario_id, "research-analysis")?,
    )?;
    assert_eq!(
        orchestrator.read_memory(&knowledge, research_record.id()),
        Err(AgentOrchestratorError::Memory(
            MemoryStoreError::RecordNotFound
        ))
    );
    complete_stage(
        &mut orchestrator,
        &knowledge,
        "knowledge-memory-cleanup",
        &child_result_for(scenario_id, "knowledge-analysis")?,
    )?;
    let synthesis = orchestrator.current_context(&root_id)?;
    assert_eq!(
        orchestrator.read_memory(&synthesis, knowledge_record.id()),
        Err(AgentOrchestratorError::Memory(
            MemoryStoreError::RecordNotFound
        ))
    );
    orchestrator.cancel_task(&root_id)?;
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());

    for scenario_id in [
        BoundedParallelScenarioId::CodeSecurityQaV1,
        BoundedParallelScenarioId::CloudSystemsSecurityV1,
    ] {
        let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        for context in acceptance.active_contexts() {
            assert_eq!(
                orchestrator.write_memory(
                    context,
                    MemoryWriteTarget::TaskTemporary,
                    MemoryContent::new("disabled specialist memory attempt")?,
                ),
                Err(AgentOrchestratorError::Memory(
                    MemoryStoreError::ProfileDisabled
                ))
            );
        }
        assert_eq!(orchestrator.shared_memory_proposal_count(), 0);
        orchestrator.cancel_task(root.task_id())?;
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn independent_research_and_knowledge_complete_in_reverse_order_but_collect_by_ordinal(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let contexts = acceptance.active_contexts();
    assert_eq!(contexts.len(), 2);
    assert_eq!(contexts[0].agent_id(), AgentId::Research);
    assert_eq!(contexts[1].agent_id(), AgentId::KnowledgeDocument);
    assert_ne!(
        contexts[0].runtime_run_identity(),
        contexts[1].runtime_run_identity()
    );
    assert_eq!(
        orchestrator.selected_workflow(),
        Some(AgentWorkflowSelection::BoundedParallel)
    );

    complete_stage(
        &mut orchestrator,
        &contexts[1],
        "knowledge-complete-first",
        &child_result(
            "research-knowledge-independent-v1",
            "knowledge-analysis",
            "decision-context",
            "The fixture preserves bounded decision evidence.",
        ),
    )?;
    complete_stage(
        &mut orchestrator,
        &contexts[0],
        "research-complete-second",
        &child_result(
            "research-knowledge-independent-v1",
            "research-analysis",
            "approach-a",
            "The fixture supports the bounded research comparison.",
        ),
    )?;

    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "personal-synthesis",
        &synthesis_result(
            "research-knowledge-independent-v1",
            &[
                (
                    1,
                    "research-analysis",
                    AgentId::Research,
                    ParallelChildResultStatus::Succeeded,
                ),
                (
                    2,
                    "knowledge-analysis",
                    AgentId::KnowledgeDocument,
                    ParallelChildResultStatus::Succeeded,
                ),
            ],
        ),
    )?;

    let result = orchestrator
        .bounded_parallel_result()
        .ok_or("missing bounded-parallel result")?;
    assert_eq!(
        result
            .children()
            .iter()
            .map(|child| (child.ordinal(), child.agent_id(), child.status()))
            .collect::<Vec<_>>(),
        [
            (1, AgentId::Research, ParallelChildResultStatus::Succeeded),
            (
                2,
                AgentId::KnowledgeDocument,
                ParallelChildResultStatus::Succeeded
            ),
        ]
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    assert_eq!(recorder.maximum_live_run_count(), 2);
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    let chronological_events = orchestrator
        .bounded_parallel_events()
        .iter()
        .map(|event| match event {
            BoundedParallelWorkflowEvent::Queued { ordinal, .. } => {
                (BoundedParallelAuditOutcome::Queued, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::Started { ordinal, .. } => {
                (BoundedParallelAuditOutcome::Started, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::Progress { ordinal, .. } => {
                (BoundedParallelAuditOutcome::Progress, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::StartAttemptFailed { ordinal, .. } => (
                BoundedParallelAuditOutcome::StartAttemptFailed,
                Some(*ordinal),
            ),
            BoundedParallelWorkflowEvent::Completed { ordinal, .. } => {
                (BoundedParallelAuditOutcome::Completed, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::Failed { ordinal, .. } => {
                (BoundedParallelAuditOutcome::Failed, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::Cancelled { ordinal, .. } => {
                (BoundedParallelAuditOutcome::Cancelled, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::TimedOut { ordinal, .. } => {
                (BoundedParallelAuditOutcome::TimedOut, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::Skipped { ordinal, .. } => {
                (BoundedParallelAuditOutcome::Skipped, Some(*ordinal))
            }
            BoundedParallelWorkflowEvent::DependencySatisfied { .. } => {
                (BoundedParallelAuditOutcome::DependencySatisfied, None)
            }
            BoundedParallelWorkflowEvent::ParentResumed { .. } => {
                (BoundedParallelAuditOutcome::ParentResumed, None)
            }
            BoundedParallelWorkflowEvent::SynthesisStarted { .. } => {
                (BoundedParallelAuditOutcome::SynthesisStarted, None)
            }
            BoundedParallelWorkflowEvent::SynthesisProgress => {
                (BoundedParallelAuditOutcome::SynthesisProgress, None)
            }
            BoundedParallelWorkflowEvent::SynthesisCompleted { .. } => {
                (BoundedParallelAuditOutcome::SynthesisCompleted, None)
            }
            BoundedParallelWorkflowEvent::Terminal { status } => {
                (BoundedParallelAuditOutcome::Terminal(*status), None)
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(
        chronological_events,
        [
            (BoundedParallelAuditOutcome::Queued, Some(1)),
            (BoundedParallelAuditOutcome::Queued, Some(2)),
            (BoundedParallelAuditOutcome::Started, Some(1)),
            (BoundedParallelAuditOutcome::Started, Some(2)),
            (BoundedParallelAuditOutcome::Progress, Some(2)),
            (BoundedParallelAuditOutcome::Completed, Some(2)),
            (BoundedParallelAuditOutcome::Progress, Some(1)),
            (BoundedParallelAuditOutcome::Completed, Some(1)),
            (BoundedParallelAuditOutcome::ParentResumed, None),
            (BoundedParallelAuditOutcome::SynthesisStarted, None),
            (BoundedParallelAuditOutcome::SynthesisProgress, None),
            (BoundedParallelAuditOutcome::SynthesisCompleted, None),
            (
                BoundedParallelAuditOutcome::Terminal(BoundedParallelTerminalStatus::Completed),
                None
            ),
        ]
    );
    let audit = orchestrator.bounded_parallel_audit_records();
    assert_eq!(audit.len(), chronological_events.len());
    assert!(audit
        .iter()
        .enumerate()
        .all(|(index, record)| usize::from(record.sequence()) == index + 1));
    assert_eq!(
        audit
            .iter()
            .map(|record| (record.outcome(), record.attribution().ordinal()))
            .collect::<Vec<_>>(),
        chronological_events
    );
    assert!(audit.iter().all(|record| {
        record.attribution().root_task_id().task_id() == &root_id
            && record.attribution().scenario_id()
                == BoundedParallelScenarioId::ResearchKnowledgeIndependentV1
    }));
    assert!(audit.iter().take(2).all(|record| {
        record.outcome() == BoundedParallelAuditOutcome::Queued
            && record.attribution().task_id().is_none()
            && record.attribution().runtime_id().is_none()
            && !record.attribution().has_live_run_binding()
    }));
    assert_eq!(audit[0].attribution().ordinal(), Some(1));
    assert_eq!(
        audit[0].attribution().expected_agent_id(),
        AgentId::Research
    );
    assert_eq!(
        audit[0].attribution().policy_profile_id(),
        AgentPolicyProfileId::ResearchReadOnlyV1
    );
    assert_eq!(
        audit[0].attribution().memory_profile_id(),
        AgentMemoryProfileId::ResearchWorkingMemoryV1
    );
    assert_eq!(audit[1].attribution().ordinal(), Some(2));
    assert_eq!(
        audit[1].attribution().expected_agent_id(),
        AgentId::KnowledgeDocument
    );
    assert_eq!(
        audit[1].attribution().memory_profile_id(),
        AgentMemoryProfileId::KnowledgeWorkingMemoryV1
    );
    assert!(audit.iter().any(|record| {
        record.outcome() == BoundedParallelAuditOutcome::Started
            && record.attribution().task_id().is_some()
            && record.attribution().runtime_id() == Some(RuntimeId::Native)
            && record.attribution().has_live_run_binding()
    }));
    assert!(audit.iter().any(|record| {
        record.outcome() == BoundedParallelAuditOutcome::SynthesisStarted
            && record.attribution().expected_agent_id() == AgentId::PersonalAssistant
            && record.attribution().work_item_id().is_none()
            && record.attribution().has_live_run_binding()
    }));
    Ok(())
}

#[test]
fn code_and_security_release_qa_only_after_both_succeed() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::CodeSecurityQaV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let initial = acceptance.active_contexts();
    assert_eq!(initial.len(), 2);
    assert_eq!(
        orchestrator
            .bounded_parallel_events()
            .iter()
            .filter(|event| matches!(
                event,
                ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::Queued { .. }
            ))
            .count(),
        3
    );
    assert_eq!(initial[0].agent_id(), AgentId::Coding);
    assert_eq!(initial[1].agent_id(), AgentId::SecurityRisk);

    complete_stage(
        &mut orchestrator,
        &initial[1],
        "security-first",
        &child_result(
            "code-security-qa-v1",
            "security-review",
            "threat-fixture",
            "The fixture grants no execution authority.",
        ),
    )?;
    assert_eq!(orchestrator.bounded_parallel_active_children()?.len(), 1);
    complete_stage(
        &mut orchestrator,
        &initial[0],
        "coding-second",
        &child_result(
            "code-security-qa-v1",
            "coding-review",
            "proposal-fixture",
            "The fixture describes an inert off-by-one proposal.",
        ),
    )?;
    let dependent = orchestrator.bounded_parallel_active_children()?;
    assert_eq!(dependent.len(), 1);
    assert_eq!(dependent[0].ordinal(), 3);
    assert_eq!(dependent[0].context().agent_id(), AgentId::QaValidation);
    let qa = dependent[0].context().clone();
    complete_stage(
        &mut orchestrator,
        &qa,
        "qa-dependent",
        &child_result(
            "code-security-qa-v1",
            "qa-validation",
            "qa-criterion",
            "The fixture predecessors are attributed and proposal-only.",
        ),
    )?;
    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "code-security-qa-synthesis",
        &synthesis_result(
            "code-security-qa-v1",
            &[
                (
                    1,
                    "coding-review",
                    AgentId::Coding,
                    ParallelChildResultStatus::Succeeded,
                ),
                (
                    2,
                    "security-review",
                    AgentId::SecurityRisk,
                    ParallelChildResultStatus::Succeeded,
                ),
                (
                    3,
                    "qa-validation",
                    AgentId::QaValidation,
                    ParallelChildResultStatus::Succeeded,
                ),
            ],
        ),
    )?;

    assert_eq!(
        orchestrator
            .bounded_parallel_result()
            .ok_or("missing result")?
            .children()
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Succeeded,
            ParallelChildResultStatus::Succeeded,
            ParallelChildResultStatus::Succeeded,
        ]
    );
    assert_eq!(recorder.maximum_live_run_count(), 2);
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn dependent_selected_text_contains_only_full_declared_predecessors_in_ordinal_order(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::CodeSecurityQaV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let coding = acceptance.active_contexts()[0].clone();
    let security = acceptance.active_contexts()[1].clone();

    complete_stage(
        &mut orchestrator,
        &security,
        "security-completed-first-for-transfer",
        &child_result_with_issues(
            "code-security-qa-v1",
            "security-review",
            "threat-fixture",
            "Distinct security finding from the immutable fixture.",
            &["security caveat remains"],
        )?,
    )?;
    complete_stage(
        &mut orchestrator,
        &coding,
        "coding-completed-second-for-transfer",
        &child_result_with_issues(
            "code-security-qa-v1",
            "coding-review",
            "proposal-fixture",
            "Distinct coding finding from the immutable fixture.",
            &["coding caveat remains"],
        )?,
    )?;

    let starts = recorder.starts();
    let dependent_start = starts.last().ok_or("missing dependent runtime start")?;
    let (_, tail) = dependent_start
        .selected_text
        .split_once("Validated predecessor transfer:\n")
        .ok_or("missing predecessor transfer marker")?;
    let (raw_transfer, _) = tail
        .split_once("\nReturn exactly one strict ParallelAnalysisResultV1")
        .ok_or("missing predecessor transfer terminator")?;
    let transfer: serde_json::Value = serde_json::from_str(raw_transfer)?;

    assert_eq!(
        transfer,
        json!([
            {
                "ordinal": 1,
                "work_item_id": "coding-review",
                "agent_id": "coding",
                "status": "succeeded",
                "summary": "Fixture evidence supports the bounded finding.",
                "findings": [{
                    "id": "bounded-finding",
                    "statement": "Distinct coding finding from the immutable fixture.",
                    "confidence": "high",
                    "references": [{"namespace": "fixture", "id": "proposal-fixture"}]
                }],
                "unresolved_issues": ["coding caveat remains"]
            },
            {
                "ordinal": 2,
                "work_item_id": "security-review",
                "agent_id": "security-risk",
                "status": "succeeded",
                "summary": "Fixture evidence supports the bounded finding.",
                "findings": [{
                    "id": "bounded-finding",
                    "statement": "Distinct security finding from the immutable fixture.",
                    "confidence": "high",
                    "references": [{"namespace": "fixture", "id": "threat-fixture"}]
                }],
                "unresolved_issues": ["security caveat remains"]
            }
        ])
    );
    assert_eq!(transfer.as_array().map(Vec::len), Some(2));
    assert!(!raw_transfer.contains("qa-validation"));
    orchestrator.cancel_task(root.task_id())?;
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn successful_child_issues_are_required_in_canonical_final_synthesis_order(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;
    let research = acceptance.active_contexts()[0].clone();
    let knowledge = acceptance.active_contexts()[1].clone();

    complete_stage(
        &mut orchestrator,
        &knowledge,
        "knowledge-issue-completes-first",
        &child_result_with_issues(
            "research-knowledge-independent-v1",
            "knowledge-analysis",
            "decision-context",
            "The fixture preserves bounded decision evidence.",
            &["document gap remains"],
        )?,
    )?;
    complete_stage(
        &mut orchestrator,
        &research,
        "research-issue-completes-second",
        &child_result_with_issues(
            "research-knowledge-independent-v1",
            "research-analysis",
            "approach-a",
            "The fixture preserves the bounded comparison.",
            &["research gap remains"],
        )?,
    )?;

    let rows = [
        (
            1,
            "research-analysis",
            AgentId::Research,
            ParallelChildResultStatus::Succeeded,
        ),
        (
            2,
            "knowledge-analysis",
            AgentId::KnowledgeDocument,
            ParallelChildResultStatus::Succeeded,
        ),
    ];
    let canonical_issues = [
        "research-analysis: research gap remains",
        "knowledge-analysis: document gap remains",
    ];
    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "canonical-success-issues",
        &synthesis_result_with_issues(
            "research-knowledge-independent-v1",
            &rows,
            &canonical_issues,
        )?,
    )?;

    let result = orchestrator
        .bounded_parallel_result()
        .ok_or("missing bounded-parallel result")?;
    assert_eq!(result.synthesis().unresolved_issues(), canonical_issues);
    assert_eq!(
        request.parse_synthesis(
            result.children(),
            &synthesis_result("research-knowledge-independent-v1", &rows),
        ),
        Err(BoundedParallelError::SynthesisStatusMismatch)
    );
    assert_eq!(
        request.parse_synthesis(
            result.children(),
            &synthesis_result_with_issues(
                "research-knowledge-independent-v1",
                &rows,
                &[canonical_issues[1], canonical_issues[0]],
            )?,
        ),
        Err(BoundedParallelError::SynthesisStatusMismatch)
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn continue_partial_preserves_a_sibling_failure_in_personal_synthesis() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let initial = acceptance.active_contexts();

    fail_stage(&mut orchestrator, &initial[1], "knowledge-failed")?;
    complete_stage(
        &mut orchestrator,
        &initial[0],
        "research-succeeded",
        &child_result(
            "research-knowledge-independent-v1",
            "research-analysis",
            "approach-b",
            "The fixture supports one bounded comparison.",
        ),
    )?;
    let synthesis = orchestrator.current_context(&root_id)?;
    let partial = synthesis_result(
        "research-knowledge-independent-v1",
        &[
            (
                1,
                "research-analysis",
                AgentId::Research,
                ParallelChildResultStatus::Succeeded,
            ),
            (
                2,
                "knowledge-analysis",
                AgentId::KnowledgeDocument,
                ParallelChildResultStatus::Failed,
            ),
        ],
    );
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "truthful-partial-synthesis",
        &partial,
    )?;

    let result = orchestrator
        .bounded_parallel_result()
        .ok_or("missing partial result")?;
    assert_eq!(
        result
            .children()
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Succeeded,
            ParallelChildResultStatus::Failed,
        ]
    );
    assert_eq!(
        result.synthesis().unresolved_issues(),
        ["knowledge-analysis is failed"]
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn fail_fast_cancels_the_other_specialist_skips_security_and_synthesizes_truthfully(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::CloudSystemsSecurityV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let initial = acceptance.active_contexts();
    assert_eq!(initial.len(), 2);
    assert_eq!(initial[0].agent_id(), AgentId::CloudInfrastructure);
    assert_eq!(initial[1].agent_id(), AgentId::SystemsOperations);

    fail_stage(&mut orchestrator, &initial[1], "systems-failed")?;
    assert!(orchestrator.bounded_parallel_active_children()?.is_empty());
    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "fail-fast-partial-synthesis",
        &synthesis_result(
            "cloud-systems-security-v1",
            &[
                (
                    1,
                    "cloud-assessment",
                    AgentId::CloudInfrastructure,
                    ParallelChildResultStatus::Cancelled,
                ),
                (
                    2,
                    "systems-assessment",
                    AgentId::SystemsOperations,
                    ParallelChildResultStatus::Failed,
                ),
                (
                    3,
                    "security-assessment",
                    AgentId::SecurityRisk,
                    ParallelChildResultStatus::Skipped,
                ),
            ],
        ),
    )?;

    assert_eq!(
        orchestrator
            .bounded_parallel_result()
            .ok_or("missing fail-fast result")?
            .children()
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Cancelled,
            ParallelChildResultStatus::Failed,
            ParallelChildResultStatus::Skipped,
        ]
    );
    assert_eq!(recorder.maximum_live_run_count(), 2);
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn individual_cancellation_keeps_the_independent_sibling_live() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let mut controls = orchestrator.bounded_parallel_active_children()?;
    assert_eq!(controls.len(), 2);
    let cancelled = controls.remove(0);
    let sibling = controls.remove(0).context().clone();
    let cancelled_context = cancelled.context().clone();
    let cancelled_task_id = cancelled.context().task_id().clone();
    let before_generic_cancel = (
        orchestrator.bounded_parallel_status(),
        orchestrator.events().len(),
        recorder.live_runs().len(),
    );
    assert_eq!(
        orchestrator.cancel_task(&cancelled_task_id),
        Err(AgentOrchestratorError::BoundedParallelCancellationHandleMismatch)
    );
    assert_eq!(
        (
            orchestrator.bounded_parallel_status(),
            orchestrator.events().len(),
            recorder.live_runs().len(),
        ),
        before_generic_cancel
    );
    orchestrator.cancel_bounded_parallel_child(cancelled.into_cancellation_handle())?;
    assert_eq!(
        orchestrator
            .task(&cancelled_task_id)
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.accept_runtime_event(
            &cancelled_task_id,
            started(&cancelled_context, "late-cancelled-event")?
        ),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    assert_eq!(orchestrator.bounded_parallel_active_children()?.len(), 1);

    complete_stage(
        &mut orchestrator,
        &sibling,
        "knowledge-after-research-cancel",
        &child_result(
            "research-knowledge-independent-v1",
            "knowledge-analysis",
            "decision-format",
            "The fixture preserves limitations and unresolved issues.",
        ),
    )?;
    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "individual-cancel-synthesis",
        &synthesis_result(
            "research-knowledge-independent-v1",
            &[
                (
                    1,
                    "research-analysis",
                    AgentId::Research,
                    ParallelChildResultStatus::Cancelled,
                ),
                (
                    2,
                    "knowledge-analysis",
                    AgentId::KnowledgeDocument,
                    ParallelChildResultStatus::Succeeded,
                ),
            ],
        ),
    )?;
    assert_eq!(
        orchestrator
            .bounded_parallel_result()
            .ok_or("missing cancellation result")?
            .children()
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Cancelled,
            ParallelChildResultStatus::Succeeded,
        ]
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn cancel_dependent_only_keeps_sibling_live_and_skips_qa() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::CodeSecurityQaV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;
    let mut controls = orchestrator.bounded_parallel_active_children()?;
    assert_eq!(controls.len(), 2);
    let coding = controls.remove(0);
    let security = controls.remove(0).context().clone();
    orchestrator.cancel_bounded_parallel_child(coding.into_cancellation_handle())?;
    assert_eq!(orchestrator.bounded_parallel_active_children()?.len(), 1);
    complete_stage(
        &mut orchestrator,
        &security,
        "security-after-coding-cancel",
        &child_result_for(
            BoundedParallelScenarioId::CodeSecurityQaV1,
            "security-review",
        )?,
    )?;
    assert!(orchestrator.bounded_parallel_active_children()?.is_empty());
    let synthesis = orchestrator.current_context(&root_id)?;
    let statuses = [
        ParallelChildResultStatus::Cancelled,
        ParallelChildResultStatus::Succeeded,
        ParallelChildResultStatus::Skipped,
    ];
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "cancel-dependent-only-synthesis",
        &synthesis_for_request(&request, &statuses),
    )?;
    assert_eq!(
        orchestrator
            .bounded_parallel_result()
            .ok_or("missing cancel-dependent result")?
            .children()
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        statuses
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn root_cancellation_is_child_first_and_leaves_no_live_or_dropped_run() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::CodeSecurityQaV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;

    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(
            ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowStatus::Cancelled
        )
    );
    assert!(orchestrator.bounded_parallel_result().is_none());
    let outcomes = orchestrator
        .bounded_parallel_outcomes()
        .ok_or("missing ordered outcomes after root cancellation")?;
    assert_eq!(
        outcomes
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Cancelled,
            ParallelChildResultStatus::Cancelled,
            ParallelChildResultStatus::Skipped,
        ]
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    let starts = recorder.starts();
    assert_eq!(
        recorder.cancellations(),
        vec![
            starts[0].run_id.clone(),
            starts[1].run_id.clone(),
            starts[2].run_id.clone(),
        ]
    );
    assert_eq!(
        orchestrator.bounded_parallel_events().len(),
        orchestrator.bounded_parallel_audit_records().len()
    );
    Ok(())
}

#[test]
fn one_shot_root_cancel_failure_retains_runs_then_resumes_without_orphans(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(2));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;

    assert!(orchestrator.cancel_task(&root_id).is_err());
    assert_eq!(recorder.live_runs().len(), 2);
    assert!(recorder.nonterminal_drops().is_empty());
    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn retained_child_cancellation_denies_all_preminted_specialist_memory_ingress_without_mutation(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(2));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let research = acceptance.active_contexts()[0].clone();
    let knowledge = acceptance.active_contexts()[1].clone();
    assert_eq!(research.agent_id(), AgentId::Research);
    assert_eq!(knowledge.agent_id(), AgentId::KnowledgeDocument);
    let research_record = orchestrator.write_memory(
        &research,
        MemoryWriteTarget::TaskTemporary,
        MemoryContent::new("Pre-cancellation bounded research memory fixture.")?,
    )?;
    let knowledge_record = orchestrator.write_memory(
        &knowledge,
        MemoryWriteTarget::TaskTemporary,
        MemoryContent::new("Pre-cancellation bounded knowledge memory fixture.")?,
    )?;

    assert!(matches!(
        orchestrator.cancel_task(&root_id),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(
                ai_agent_assistant_lib::agent::runtime::RuntimeBoundaryStage::Cancellation
            )
        ))
    ));
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(
            ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowStatus::Cancelling
        )
    );
    assert_eq!(recorder.live_runs().len(), 2);
    assert!(recorder.nonterminal_drops().is_empty());

    let orchestrator_before = (
        format!("{orchestrator:?}"),
        orchestrator.task_count(),
        orchestrator.run_count(),
        orchestrator.runtime_event_count(),
        orchestrator.events().len(),
        orchestrator.bounded_parallel_events().len(),
        orchestrator.bounded_parallel_audit_records().len(),
    );
    let runtime_before = (
        recorder.starts(),
        recorder.cancellations(),
        recorder.live_runs(),
        recorder.maximum_live_run_count(),
        recorder.terminal_dispositions(),
        recorder.nonterminal_drops(),
    );
    assert!(orchestrator_before.0.contains("record_count: 2"));

    assert_cancelling_parallel_memory_ingress_denied(
        &mut orchestrator,
        &research,
        research_record.id(),
        research_record.version(),
        "Denied research memory write during retained cancellation.",
    )?;
    assert_cancelling_parallel_memory_ingress_denied(
        &mut orchestrator,
        &knowledge,
        knowledge_record.id(),
        knowledge_record.version(),
        "Denied knowledge memory write during retained cancellation.",
    )?;

    assert_eq!(
        (
            format!("{orchestrator:?}"),
            orchestrator.task_count(),
            orchestrator.run_count(),
            orchestrator.runtime_event_count(),
            orchestrator.events().len(),
            orchestrator.bounded_parallel_events().len(),
            orchestrator.bounded_parallel_audit_records().len(),
        ),
        orchestrator_before
    );
    assert_eq!(
        (
            recorder.starts(),
            recorder.cancellations(),
            recorder.live_runs(),
            recorder.maximum_live_run_count(),
            recorder.terminal_dispositions(),
            recorder.nonterminal_drops(),
        ),
        runtime_before
    );

    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn waiting_root_cancel_uses_waiting_attribution_and_synthesis_start_failure_uses_start_attempt(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    orchestrator.cancel_task(root.task_id())?;
    let terminal = orchestrator
        .bounded_parallel_audit_records()
        .last()
        .ok_or("missing waiting-root cancellation audit")?;
    assert!(matches!(
        terminal.attribution(),
        ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelAttribution::PersonalRootWaiting(_)
    ));
    assert_eq!(terminal.attribution().runtime_id(), None);
    assert!(!terminal.attribution().has_live_run_binding());
    assert!(recorder.live_runs().is_empty());

    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(4));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    assert!(complete_all_active_children(&mut orchestrator, scenario_id).is_err());
    let terminal = orchestrator
        .bounded_parallel_audit_records()
        .last()
        .ok_or("missing synthesis-start-failure audit")?;
    assert!(matches!(
        terminal.attribution(),
        ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelAttribution::PersonalRootStartAttempt(_)
    ));
    assert_eq!(terminal.attribution().runtime_id(), Some(RuntimeId::Native));
    assert!(!terminal.attribution().has_live_run_binding());
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn a_cross_run_envelope_cannot_mutate_another_parallel_child() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let first = &acceptance.active_contexts()[0];
    let second = &acceptance.active_contexts()[1];
    let before = (
        orchestrator.runtime_event_count(),
        orchestrator.events().len(),
        orchestrator.bounded_parallel_events().len(),
        orchestrator.bounded_parallel_audit_records().len(),
    );

    assert_eq!(
        orchestrator
            .accept_runtime_event(first.task_id(), started(second, "foreign-child-envelope")?),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::IdentityMismatch)
        ))
    );
    assert_eq!(
        (
            orchestrator.runtime_event_count(),
            orchestrator.events().len(),
            orchestrator.bounded_parallel_events().len(),
            orchestrator.bounded_parallel_audit_records().len(),
        ),
        before
    );
    assert_eq!(orchestrator.bounded_parallel_active_children()?.len(), 2);
    orchestrator.cancel_task(&root_id)?;
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn each_initial_start_failure_is_attributed_without_retry_or_orphan() -> Result<(), Box<dyn Error>>
{
    let cases = [
        (
            BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
            2,
            1,
            4,
            vec![
                ParallelChildResultStatus::Failed,
                ParallelChildResultStatus::Succeeded,
            ],
        ),
        (
            BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
            3,
            2,
            4,
            vec![
                ParallelChildResultStatus::Succeeded,
                ParallelChildResultStatus::Failed,
            ],
        ),
        (
            BoundedParallelScenarioId::CodeSecurityQaV1,
            2,
            1,
            4,
            vec![
                ParallelChildResultStatus::Failed,
                ParallelChildResultStatus::Succeeded,
                ParallelChildResultStatus::Skipped,
            ],
        ),
        (
            BoundedParallelScenarioId::CodeSecurityQaV1,
            3,
            2,
            4,
            vec![
                ParallelChildResultStatus::Succeeded,
                ParallelChildResultStatus::Failed,
                ParallelChildResultStatus::Skipped,
            ],
        ),
        (
            BoundedParallelScenarioId::CloudSystemsSecurityV1,
            2,
            1,
            3,
            vec![
                ParallelChildResultStatus::Failed,
                ParallelChildResultStatus::Skipped,
                ParallelChildResultStatus::Skipped,
            ],
        ),
        (
            BoundedParallelScenarioId::CloudSystemsSecurityV1,
            3,
            2,
            4,
            vec![
                ParallelChildResultStatus::Cancelled,
                ParallelChildResultStatus::Failed,
                ParallelChildResultStatus::Skipped,
            ],
        ),
    ];

    for (scenario_id, start_ordinal, failed_slot, expected_starts, statuses) in cases {
        let (runtime, recorder) =
            MockAgentRuntime::recording(MockMode::StartFailureAt(start_ordinal));
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
        let root = orchestrator.start_root(request.objective())?;
        let root_id = root.task_id().clone();
        orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;
        complete_all_active_children(&mut orchestrator, scenario_id)?;
        let synthesis = orchestrator.current_context(&root_id)?;
        complete_stage(
            &mut orchestrator,
            &synthesis,
            "start-failure-synthesis",
            &synthesis_for_request(&request, &statuses),
        )?;

        assert_eq!(
            orchestrator
                .bounded_parallel_result()
                .ok_or("missing start-failure result")?
                .children()
                .iter()
                .map(|outcome| outcome.status())
                .collect::<Vec<_>>(),
            statuses
        );
        let failed = orchestrator
            .bounded_parallel_audit_records()
            .iter()
            .find(|record| record.outcome() == BoundedParallelAuditOutcome::StartAttemptFailed)
            .ok_or("missing start-attempt audit record")?;
        assert_eq!(failed.attribution().ordinal(), Some(failed_slot));
        assert!(failed.attribution().task_id().is_some());
        assert_eq!(failed.attribution().runtime_id(), Some(RuntimeId::Native));
        assert!(!failed.attribution().has_live_run_binding());
        assert_eq!(recorder.starts().len(), expected_starts);
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn rejected_parallel_child_cleanup_blocks_terminal_root_until_root_cancel_cleans_it(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) =
        MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchWithCancelFailureOnceAt(2));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();

    assert_eq!(
        orchestrator.start_bounded_parallel_workflow(&root, request),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(BoundedParallelWorkflowStatus::Cancelling)
    );
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(orchestrator.bounded_parallel_result().is_none());

    assert_eq!(
        orchestrator.cancel_task(&root_id)?,
        AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(BoundedParallelWorkflowStatus::Cancelled)
    );
    assert!(orchestrator.bounded_parallel_result().is_none());
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    assert_eq!(recorder.starts().len(), 2);
    Ok(())
}

#[test]
fn rejected_synthesis_cleanup_blocks_terminal_root_until_explicit_retry(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) =
        MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchWithCancelFailureOnceAt(4));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;

    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    assert!(matches!(
        complete_all_active_children(&mut orchestrator, scenario_id),
        Err(ref error)
            if error.downcast_ref::<AgentOrchestratorError>()
                == Some(&AgentOrchestratorError::RuntimeCleanupPending)
    ));
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(BoundedParallelWorkflowStatus::Cancelling)
    );
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(orchestrator.bounded_parallel_result().is_none());

    orchestrator.retry_rejected_runtime_cleanup()?;
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(BoundedParallelWorkflowStatus::Failed)
    );
    assert!(orchestrator.bounded_parallel_result().is_none());
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    assert_eq!(recorder.starts().len(), 4);
    Ok(())
}

#[test]
fn rejected_parallel_child_identities_project_as_start_failures_without_live_binding_or_orphans(
) -> Result<(), Box<dyn Error>> {
    let cases = [
        (
            MockMode::ReturnedIdentityMismatchAt(2),
            2,
            1,
            1,
            vec![
                ParallelChildResultStatus::Failed,
                ParallelChildResultStatus::Succeeded,
            ],
            false,
        ),
        (
            MockMode::DuplicateLiveIdentityAt(3),
            3,
            2,
            2,
            vec![
                ParallelChildResultStatus::Succeeded,
                ParallelChildResultStatus::Failed,
            ],
            true,
        ),
    ];

    for (
        mode,
        rejected_start_ordinal,
        rejected_slot_ordinal,
        expected_maximum_live,
        statuses,
        duplicate_live_identity,
    ) in cases
    {
        let (runtime, recorder) = MockAgentRuntime::recording(mode);
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let root = orchestrator.start_root(request.objective())?;
        let root_id = root.task_id().clone();
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;

        assert_eq!(acceptance.active_contexts().len(), 1);
        assert_eq!(recorder.live_runs().len(), 1);
        assert_eq!(recorder.maximum_live_run_count(), expected_maximum_live);
        let rejected = recorder
            .terminal_dispositions()
            .into_iter()
            .find(|record| record.start_ordinal == rejected_start_ordinal)
            .ok_or("missing rejected runtime disposition")?;
        assert_eq!(
            rejected.status,
            ai_agent_assistant_lib::agent::runtime::RuntimeRunStatus::Cancelled
        );
        if duplicate_live_identity {
            assert_eq!(rejected.run_id, recorder.live_runs()[0].run_id);
        } else {
            assert_ne!(rejected.run_id, recorder.live_runs()[0].run_id);
        }

        let rejected_audit = orchestrator
            .bounded_parallel_audit_records()
            .iter()
            .find(|record| {
                record.outcome() == BoundedParallelAuditOutcome::StartAttemptFailed
                    && record.attribution().ordinal() == Some(rejected_slot_ordinal)
            })
            .ok_or("missing rejected-identity start-attempt audit")?;
        assert!(rejected_audit.attribution().task_id().is_some());
        assert!(!rejected_audit.attribution().has_live_run_binding());
        assert!(!orchestrator.bounded_parallel_events().iter().any(|event| {
            matches!(
                event,
                ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::Started {
                    ordinal,
                    ..
                } if *ordinal == rejected_slot_ordinal
            )
        }));

        complete_all_active_children(
            &mut orchestrator,
            BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
        )?;
        let synthesis = orchestrator.current_context(&root_id)?;
        complete_stage(
            &mut orchestrator,
            &synthesis,
            "identity-rejection-partial-synthesis",
            &synthesis_for_request(&request, &statuses),
        )?;

        assert_eq!(
            orchestrator
                .bounded_parallel_result()
                .ok_or("missing identity-rejection result")?
                .children()
                .iter()
                .map(|outcome| outcome.status())
                .collect::<Vec<_>>(),
            statuses
        );
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn each_dependent_start_failure_preserves_ordered_partial_synthesis() -> Result<(), Box<dyn Error>>
{
    for scenario_id in [
        BoundedParallelScenarioId::CodeSecurityQaV1,
        BoundedParallelScenarioId::CloudSystemsSecurityV1,
    ] {
        let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(4));
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
        let root = orchestrator.start_root(request.objective())?;
        let root_id = root.task_id().clone();
        orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;
        complete_all_active_children(&mut orchestrator, scenario_id)
            .map_err(|error| format!("{scenario_id:?} dependent completion: {error}"))?;
        let statuses = [
            ParallelChildResultStatus::Succeeded,
            ParallelChildResultStatus::Succeeded,
            ParallelChildResultStatus::Failed,
        ];
        let synthesis = orchestrator.current_context(&root_id)?;
        complete_stage(
            &mut orchestrator,
            &synthesis,
            "dependent-start-failure-synthesis",
            &synthesis_for_request(&request, &statuses),
        )?;
        assert_eq!(
            orchestrator
                .bounded_parallel_result()
                .ok_or("missing dependent-failure result")?
                .children()
                .iter()
                .map(|outcome| outcome.status())
                .collect::<Vec<_>>(),
            statuses
        );
        let failed = orchestrator
            .bounded_parallel_audit_records()
            .iter()
            .find(|record| record.outcome() == BoundedParallelAuditOutcome::StartAttemptFailed)
            .ok_or("missing dependent start-attempt audit")?;
        assert_eq!(failed.attribution().ordinal(), Some(3));
        assert!(!failed.attribution().has_live_run_binding());
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn synthesis_start_failure_fails_root_without_fabricating_live_attribution(
) -> Result<(), Box<dyn Error>> {
    let cases = [
        (BoundedParallelScenarioId::ResearchKnowledgeIndependentV1, 4),
        (BoundedParallelScenarioId::CodeSecurityQaV1, 5),
        (BoundedParallelScenarioId::CloudSystemsSecurityV1, 5),
    ];
    for (scenario_id, synthesis_start_ordinal) in cases {
        let (runtime, recorder) =
            MockAgentRuntime::recording(MockMode::StartFailureAt(synthesis_start_ordinal));
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        assert!(complete_all_active_children(&mut orchestrator, scenario_id).is_err());
        assert_eq!(
            orchestrator.root_task().map(|task| task.status()),
            Some(AgentTaskStatus::Failed)
        );
        assert!(orchestrator.bounded_parallel_result().is_none());
        let terminal = orchestrator
            .bounded_parallel_audit_records()
            .last()
            .ok_or("missing synthesis-start terminal audit")?;
        assert_eq!(
            terminal.attribution().expected_agent_id(),
            AgentId::PersonalAssistant
        );
        assert!(terminal.attribution().work_item_id().is_none());
        assert!(terminal.attribution().task_id().is_some());
        assert_eq!(terminal.attribution().runtime_id(), Some(RuntimeId::Native));
        assert!(!terminal.attribution().has_live_run_binding());
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn duplicate_or_nested_selection_is_rejected_without_new_runtime_attempt(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;
    let before = recorder.starts().len();
    assert_eq!(
        orchestrator.start_bounded_parallel_workflow(&root, request),
        Err(AgentOrchestratorError::WorkflowAlreadySelected {
            selected: AgentWorkflowSelection::BoundedParallel,
        })
    );
    assert_eq!(recorder.starts().len(), before);
    orchestrator.cancel_task(root.task_id())?;
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn initial_root_cancellation_failure_is_atomic_and_the_same_request_can_retry_once(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(1));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let before = (
        recorder.starts().len(),
        orchestrator.events().len(),
        orchestrator.run_count(),
    );

    assert!(matches!(
        orchestrator.start_bounded_parallel_workflow(&root, request.clone()),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(
                ai_agent_assistant_lib::agent::runtime::RuntimeBoundaryStage::Cancellation
            )
        ))
    ));
    assert_eq!(orchestrator.selected_workflow(), None);
    assert_eq!(orchestrator.bounded_parallel_status(), None);
    assert_eq!(
        (
            recorder.starts().len(),
            orchestrator.events().len(),
            orchestrator.run_count(),
        ),
        before
    );
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());

    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    assert_eq!(acceptance.active_contexts().len(), 2);
    assert_eq!(recorder.starts().len(), before.0 + 2);
    orchestrator.cancel_task(root.task_id())?;
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn native_runtime_retains_two_distinct_depth_one_runs_without_trait_changes(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(NativeAgentRuntime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    assert_eq!(acceptance.active_contexts().len(), 2);
    assert!(acceptance.active_contexts().iter().all(|context| {
        context.depth() == 1
            && context.runtime_id() == RuntimeId::Native
            && context.root_task_id().task_id() == &root_id
    }));
    assert_ne!(
        acceptance.active_contexts()[0].runtime_run_identity(),
        acceptance.active_contexts()[1].runtime_run_identity()
    );
    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    Ok(())
}

#[test]
fn tool_proposal_fails_only_the_exact_child_and_never_enters_governance(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;
    let research = acceptance.active_contexts()[0].clone();
    let knowledge = acceptance.active_contexts()[1].clone();
    orchestrator.accept_runtime_event(
        research.task_id(),
        started(&research, "research-tool-proposal")?,
    )?;
    let proposal = RuntimeEventEnvelope::for_identity(
        research.runtime_run_identity(),
        1,
        UntrustedRuntimeEvent::ToolProposal {
            proposal: UntrustedRuntimeToolProposal::new(
                "parallel-tool-attempt",
                "create_local_task",
                1,
                r#"{"title":"must-not-execute"}"#,
            )?,
        },
    );
    assert_eq!(
        orchestrator.accept_runtime_event(research.task_id(), proposal),
        Err(AgentOrchestratorError::ToolProposalUnsupported)
    );
    assert_eq!(
        orchestrator
            .task(research.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        orchestrator
            .task(knowledge.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(orchestrator.governance_audit_records().is_empty());
    assert!(matches!(
        orchestrator.pending_governance_approval(research.task_id()),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    ));
    complete_stage(
        &mut orchestrator,
        &knowledge,
        "knowledge-after-tool-rejection",
        &child_result_for(
            BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
            "knowledge-analysis",
        )?,
    )?;
    let synthesis = orchestrator.current_context(&root_id)?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "tool-rejection-partial-synthesis",
        &synthesis_for_request(
            &request,
            &[
                ParallelChildResultStatus::Failed,
                ParallelChildResultStatus::Succeeded,
            ],
        ),
    )?;
    assert_eq!(
        orchestrator
            .bounded_parallel_result()
            .ok_or("missing tool-rejection result")?
            .children()
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Failed,
            ParallelChildResultStatus::Succeeded,
        ]
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn forced_child_failure_with_already_terminal_cancel_projects_runtime_mismatch_once(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelAlreadyTerminalAt(
        2,
        ai_agent_assistant_lib::agent::runtime::RuntimeRunStatus::Completed,
    ));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let research = acceptance.active_contexts()[0].clone();
    orchestrator
        .accept_runtime_event(research.task_id(), started(&research, "forced-mismatch")?)?;
    orchestrator.accept_runtime_event(
        research.task_id(),
        RuntimeEventEnvelope::for_identity(
            research.runtime_run_identity(),
            1,
            UntrustedRuntimeEvent::OutputTextDelta {
                delta: RuntimeOutputText::new("x".repeat(MAX_PARALLEL_RAW_RESULT_CHARACTERS))?,
            },
        ),
    )?;
    assert_eq!(
        orchestrator.accept_runtime_event(
            research.task_id(),
            RuntimeEventEnvelope::for_identity(
                research.runtime_run_identity(),
                2,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: RuntimeOutputText::new("a")?,
                },
            ),
        ),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: ai_agent_assistant_lib::agent::runtime::RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        orchestrator
            .task(research.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(orchestrator
        .bounded_parallel_events()
        .iter()
        .any(|event| matches!(
        event,
        ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::Failed {
            ordinal: 1,
            code: ai_agent_assistant_lib::agent::task::AgentTaskFailureCode::RuntimeStateMismatch,
            ..
        }
    )));
    assert!(!orchestrator.bounded_parallel_events().iter().any(|event| matches!(
        event,
        ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::Terminal { .. }
    )));
    let failed_count = orchestrator
        .bounded_parallel_events()
        .iter()
        .filter(|event| matches!(
            event,
            ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::Failed {
                ordinal: 1,
                code: ai_agent_assistant_lib::agent::task::AgentTaskFailureCode::RuntimeStateMismatch,
                ..
            }
        ))
        .count();
    assert_eq!(failed_count, 1);
    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator
            .bounded_parallel_events()
            .iter()
            .filter(|event| matches!(
                event,
                ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::Failed {
                    ordinal: 1,
                    code: ai_agent_assistant_lib::agent::task::AgentTaskFailureCode::RuntimeStateMismatch,
                    ..
                }
            ))
            .count(),
        failed_count
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn synthesis_cancel_already_terminal_mismatch_fails_root_without_retry_reinterpretation(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelAlreadyTerminalAt(
        4,
        ai_agent_assistant_lib::agent::runtime::RuntimeRunStatus::Completed,
    ));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    complete_all_active_children(&mut orchestrator, scenario_id)?;
    assert_eq!(
        orchestrator.cancel_task(&root_id),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: ai_agent_assistant_lib::agent::runtime::RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(
            ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowStatus::Failed
        )
    );
    let failed_count = orchestrator
        .events()
        .iter()
        .filter(|event| matches!(
            event,
            ai_agent_assistant_lib::agent::orchestrator::AgentOrchestrationEvent::RootFailed {
                code: ai_agent_assistant_lib::agent::task::AgentTaskFailureCode::RuntimeStateMismatch,
                ..
            }
        ))
        .count();
    assert_eq!(failed_count, 1);
    assert!(orchestrator.cancel_task(&root_id).is_ok());
    assert_eq!(
        orchestrator
            .events()
            .iter()
            .filter(|event| matches!(
                event,
                ai_agent_assistant_lib::agent::orchestrator::AgentOrchestrationEvent::RootFailed { .. }
            ))
            .count(),
        failed_count
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn malformed_synthesis_fails_the_root_without_concealing_child_results(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    complete_all_active_children(&mut orchestrator, scenario_id)?;
    let synthesis = orchestrator.current_context(&root_id)?;
    orchestrator.accept_runtime_event(&root_id, started(&synthesis, "malformed-synthesis")?)?;
    orchestrator.accept_runtime_event(&root_id, delta(&synthesis, r#"{"version":"v1"}"#)?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(&root_id, completed(&synthesis))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(orchestrator.bounded_parallel_result().is_none());
    let outcomes = orchestrator
        .bounded_parallel_outcomes()
        .ok_or("missing ordered outcomes after malformed synthesis")?;
    assert_eq!(
        outcomes
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Succeeded,
            ParallelChildResultStatus::Succeeded,
        ]
    );
    assert!(outcomes.iter().all(|outcome| matches!(
        outcome.disposition(),
        ParallelChildDisposition::Succeeded(result) if result.findings().len() == 1
    )));
    assert!(orchestrator.bounded_parallel_events().iter().any(|event| matches!(
        event,
        ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowEvent::Completed { ordinal: 1 | 2, .. }
    )));
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn empty_terminal_outputs_are_accepted_once_and_project_typed_failures(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
    let research = acceptance.active_contexts()[0].clone();
    let knowledge = acceptance.active_contexts()[1].clone();

    orchestrator.accept_runtime_event(
        research.task_id(),
        started(&research, "empty-child-output")?,
    )?;
    orchestrator.accept_runtime_event(research.task_id(), delta(&research, " ")?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(research.task_id(), completed(&research))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(
        orchestrator
            .task(research.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(recorder.live_runs().len(), 1);

    complete_stage(
        &mut orchestrator,
        &knowledge,
        "knowledge-after-empty-research",
        &child_result_for(scenario_id, "knowledge-analysis")?,
    )?;
    let synthesis = orchestrator.current_context(&root_id)?;
    orchestrator.accept_runtime_event(&root_id, started(&synthesis, "empty-synthesis-output")?)?;
    orchestrator.accept_runtime_event(&root_id, delta(&synthesis, " ")?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(&root_id, completed(&synthesis))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        orchestrator
            .bounded_parallel_outcomes()
            .ok_or("missing child outcomes after empty synthesis")?
            .iter()
            .map(|outcome| outcome.status())
            .collect::<Vec<_>>(),
        [
            ParallelChildResultStatus::Failed,
            ParallelChildResultStatus::Succeeded,
        ]
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn root_cancellation_while_synthesizing_cancels_the_personal_run() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    complete_all_active_children(&mut orchestrator, scenario_id)?;
    assert_eq!(recorder.live_runs().len(), 1);
    assert_eq!(
        orchestrator.current_context(&root_id)?.agent_id(),
        AgentId::PersonalAssistant
    );
    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn synthesis_cancel_failure_retains_personal_run_then_resumes_once() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(4));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    complete_all_active_children(&mut orchestrator, scenario_id)?;
    assert!(orchestrator.cancel_task(&root_id).is_err());
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());
    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn three_slot_synthesis_cancel_failure_retains_then_resumes_at_run_ordinal_five(
) -> Result<(), Box<dyn Error>> {
    for scenario_id in [
        BoundedParallelScenarioId::CodeSecurityQaV1,
        BoundedParallelScenarioId::CloudSystemsSecurityV1,
    ] {
        let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(5));
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
        let root = orchestrator.start_root(request.objective())?;
        let root_id = root.task_id().clone();
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        complete_all_active_children(&mut orchestrator, scenario_id)?;

        assert!(matches!(
            orchestrator.cancel_task(&root_id),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(
                    ai_agent_assistant_lib::agent::runtime::RuntimeBoundaryStage::Cancellation
                )
            ))
        ));
        assert_eq!(recorder.live_runs().len(), 1);
        assert!(recorder.nonterminal_drops().is_empty());
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(
                ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowStatus::Cancelling
            )
        );

        assert_eq!(
            orchestrator.cancel_task(&root_id)?,
            ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
        );
        assert_eq!(
            orchestrator.root_task().map(|task| task.status()),
            Some(AgentTaskStatus::Cancelled)
        );
        assert!(recorder.live_runs().is_empty());
        assert!(recorder.nonterminal_drops().is_empty());
    }
    Ok(())
}

#[test]
fn pending_synthesis_cancellation_rejects_all_ingress_without_mutation_then_resumes(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(4));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    complete_all_active_children(&mut orchestrator, scenario_id)?;
    let synthesis = orchestrator.current_context(&root_id)?;

    assert!(matches!(
        orchestrator.cancel_task(&root_id),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(
                ai_agent_assistant_lib::agent::runtime::RuntimeBoundaryStage::Cancellation
            )
        ))
    ));
    assert_eq!(recorder.live_runs().len(), 1);
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(
            ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowStatus::Cancelling
        )
    );
    assert!(recorder.nonterminal_drops().is_empty());

    let before = (
        orchestrator.runtime_event_count(),
        orchestrator.events().len(),
        orchestrator.bounded_parallel_events().len(),
        orchestrator.bounded_parallel_audit_records().len(),
        orchestrator.governance_audit_records().len(),
        orchestrator.shared_memory_proposal_count(),
        orchestrator.root_task().map(|task| task.status()),
        orchestrator.bounded_parallel_status(),
    );
    assert_eq!(
        orchestrator.accept_runtime_event(
            synthesis.task_id(),
            started(&synthesis, "cancel-pending-synthesis-event")?,
        ),
        Err(AgentOrchestratorError::BoundedParallelCancellationPending)
    );
    assert_eq!(
        orchestrator.govern_tool_proposal(
            &synthesis,
            AgentToolProposal::new("cancel-pending-tool-call", "get_current_datetime", 1, "{}",)?,
        ),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    );
    assert_eq!(
        orchestrator.propose_shared_memory(
            &synthesis,
            MemoryContent::new("cancel-pending shared-memory proposal sentinel")?,
        ),
        Err(AgentOrchestratorError::BoundedParallelAuthorityDenied)
    );
    assert_eq!(
        (
            orchestrator.runtime_event_count(),
            orchestrator.events().len(),
            orchestrator.bounded_parallel_events().len(),
            orchestrator.bounded_parallel_audit_records().len(),
            orchestrator.governance_audit_records().len(),
            orchestrator.shared_memory_proposal_count(),
            orchestrator.root_task().map(|task| task.status()),
            orchestrator.bounded_parallel_status(),
        ),
        before
    );
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());

    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.bounded_parallel_status(),
        Some(
            ai_agent_assistant_lib::agent::bounded_parallelism::BoundedParallelWorkflowStatus::Cancelled
        )
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn live_and_cancelling_synthesis_deny_document_and_memory_authority_before_io(
) -> Result<(), Box<dyn Error>> {
    let approved_root_directory = tempdir()?;
    let approved_document_path = approved_root_directory.path().join("approved-fixture.txt");
    fs::write(
        &approved_document_path,
        "Fixture-only document used to pre-mint an inert approved document identity.",
    )?;
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureOnceAt(4));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
    let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let approved_root_id =
        orchestrator.register_approved_root(&root, approved_root_directory.path())?;
    let approved_document_id = orchestrator.register_approved_document(
        &root,
        ApprovedDocumentSource::UserSelectedFile,
        &approved_document_path,
    )?;
    let proposal = orchestrator.propose_shared_memory(
        &root,
        MemoryContent::new("Preexisting proposal remains unchanged by denied workflow calls.")?,
    )?;
    let proposal_id = proposal.id().clone();
    let proposal_version = proposal.version();
    assert_eq!(orchestrator.shared_memory_proposal_count(), 1);
    let initialized_debug = format!("{orchestrator:?}");
    assert!(initialized_debug.contains("root_count: 1"));
    assert!(initialized_debug.contains("document_count: 1"));

    orchestrator.start_bounded_parallel_workflow(&root, request)?;
    complete_all_active_children(&mut orchestrator, scenario_id)?;
    let synthesis = orchestrator.current_context(&root_id)?;
    assert_eq!(synthesis.agent_id(), AgentId::PersonalAssistant);

    assert_bounded_parallel_no_io_authority_denied(
        &mut orchestrator,
        &synthesis,
        &approved_root_id,
        &approved_document_id,
        &proposal_id,
        proposal_version,
        "live-synthesis",
    )?;

    assert!(matches!(
        orchestrator.cancel_task(&root_id),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::BoundaryFailure(
                ai_agent_assistant_lib::agent::runtime::RuntimeBoundaryStage::Cancellation
            )
        ))
    ));
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());

    assert_bounded_parallel_no_io_authority_denied(
        &mut orchestrator,
        &synthesis,
        &approved_root_id,
        &approved_document_id,
        &proposal_id,
        proposal_version,
        "cancelling-synthesis",
    )?;

    orchestrator.cancel_task(&root_id)?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(recorder.live_runs().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn result_event_and_audit_debug_never_expose_model_content() -> Result<(), Box<dyn Error>> {
    const SENTINEL: &str = "parallel-private-content-sentinel";
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let request = BoundedParallelScenarioCatalog::built_in()
        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
    let root = orchestrator.start_root(request.objective())?;
    let root_id = root.task_id().clone();
    let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request.clone())?;
    for context in acceptance.active_contexts() {
        let (work_item, fixture) = match context.agent_id() {
            AgentId::Research => ("research-analysis", "approach-a"),
            AgentId::KnowledgeDocument => ("knowledge-analysis", "decision-context"),
            _ => return Err("unexpected sealed child agent".into()),
        };
        complete_stage(
            &mut orchestrator,
            context,
            &format!("{work_item}-redaction"),
            &child_result(
                "research-knowledge-independent-v1",
                work_item,
                fixture,
                &format!("Fixture evidence includes {SENTINEL}."),
            ),
        )?;
    }
    let synthesis = orchestrator.current_context(&root_id)?;
    let synthesis_raw = synthesis_for_request(
        &request,
        &[
            ParallelChildResultStatus::Succeeded,
            ParallelChildResultStatus::Succeeded,
        ],
    )
    .replace(
        BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE,
        &format!("{BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE} Records {SENTINEL}."),
    );
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "redacted-synthesis",
        &synthesis_raw,
    )?;
    let debug = format!(
        "{:?} {:?} {:?} {:?}",
        orchestrator.bounded_parallel_result(),
        orchestrator.bounded_parallel_events(),
        orchestrator.bounded_parallel_audit_records(),
        recorder.starts(),
    );
    assert!(!debug.contains(SENTINEL));
    assert!(!debug.contains("runtime-request"));
    Ok(())
}
