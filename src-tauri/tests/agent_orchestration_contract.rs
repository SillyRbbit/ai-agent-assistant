use std::error::Error;

use ai_agent_assistant_lib::agent::definition::{AgentId, AgentIdParseError};
use ai_agent_assistant_lib::agent::orchestrator::{
    AgentOrchestrationEvent, AgentOrchestrator, AgentOrchestratorError, DelegationProposal,
    MAX_RUNTIME_EVENTS_PER_ROOT,
};
use ai_agent_assistant_lib::agent::runtime::{
    RuntimeError, RuntimeEventEnvelope, RuntimeEventRejection, RuntimeFailure, RuntimeFailureCode,
    RuntimeId, RuntimeOutputText, RuntimeResponseId, RuntimeRunStatus, UntrustedRuntimeEvent,
    UntrustedRuntimeToolProposal,
};
use ai_agent_assistant_lib::agent::task::{
    AgentTaskFailureCode, AgentTaskOutcome, AgentTaskOutcomeKind, AgentTaskStatus,
    MAX_AGENT_TASK_OUTPUT_CHARACTERS,
};

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

const ROOT_OBJECTIVE: &str = "Answer the user's bounded question";
const RESEARCH_OBJECTIVE: &str = "Compare the supplied synthetic evidence";
const RESEARCH_RESULT: &str = "Source A and Source B agree on the bounded finding.";
const FINAL_SYNTHESIS: &str = "The supplied evidence agrees on the bounded finding.";

type DelegatedScenarioEvidence = (Vec<&'static str>, Vec<String>);

fn started(
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
    sequence: u32,
    response_id: &str,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        sequence,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new(response_id)?,
        },
    ))
}

fn delta(
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
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

fn completed(
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
    sequence: u32,
) -> RuntimeEventEnvelope {
    RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        sequence,
        UntrustedRuntimeEvent::ResponseCompleted,
    )
}

fn delegation() -> Result<DelegationProposal, Box<dyn Error>> {
    Ok(DelegationProposal::new(
        AgentId::Research,
        RESEARCH_OBJECTIVE,
        Some("Use only the supplied synthetic evidence".to_owned()),
        "Return one attributed bounded comparison",
    )?)
}

fn delegated_scenario_evidence() -> Result<DelegatedScenarioEvidence, Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    orchestrator.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    orchestrator.accept_runtime_event(&child_id, delta(&child, 1, RESEARCH_RESULT)?)?;
    orchestrator.accept_runtime_event(&child_id, completed(&child, 2))?;
    let synthesis = orchestrator.current_context(&root_id)?;
    orchestrator.accept_runtime_event(&root_id, started(&synthesis, 0, "synthesis-response")?)?;
    orchestrator.accept_runtime_event(&root_id, delta(&synthesis, 1, FINAL_SYNTHESIS)?)?;
    orchestrator.accept_runtime_event(&root_id, completed(&synthesis, 2))?;
    let topology = orchestrator
        .events()
        .iter()
        .map(|event| match event {
            AgentOrchestrationEvent::RootTaskCreated { .. } => "root-created",
            AgentOrchestrationEvent::TaskStarted { .. } => "task-started",
            AgentOrchestrationEvent::DelegationRequested { .. } => "delegation-requested",
            AgentOrchestrationEvent::DelegationAccepted { .. } => "delegation-accepted",
            AgentOrchestrationEvent::ChildCreated { .. } => "child-created",
            AgentOrchestrationEvent::ChildStarted { .. } => "child-started",
            AgentOrchestrationEvent::ChildCompleted { .. } => "child-completed",
            AgentOrchestrationEvent::ChildFailed { .. } => "child-failed",
            AgentOrchestrationEvent::ResultReturned { .. } => "result-returned",
            AgentOrchestrationEvent::ParentResumed { .. } => "parent-resumed",
            AgentOrchestrationEvent::RootCompleted { .. } => "root-completed",
            AgentOrchestrationEvent::RootFailed { .. } => "root-failed",
            AgentOrchestrationEvent::TaskCancelled { .. } => "task-cancelled",
        })
        .collect();
    let inputs = recorder
        .starts()
        .into_iter()
        .map(|start| start.selected_text)
        .collect();
    Ok((topology, inputs))
}

#[test]
fn personal_assistant_can_complete_directly_without_a_child() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();

    orchestrator.accept_runtime_event(&root_id, started(&root, 0, "direct-response")?)?;
    orchestrator.accept_runtime_event(&root_id, delta(&root, 1, FINAL_SYNTHESIS)?)?;
    orchestrator.accept_runtime_event(&root_id, completed(&root, 2))?;

    let task = orchestrator.task(&root_id).ok_or("missing root task")?;
    assert_eq!(task.status(), AgentTaskStatus::Completed);
    assert!(matches!(
        task.outcome(),
        Some(AgentTaskOutcome::Completed(result))
            if result.task_id() == &root_id
                && result.agent_id() == AgentId::PersonalAssistant
                && result.output().as_str() == FINAL_SYNTHESIS
    ));
    assert_eq!(orchestrator.task_count(), 1);
    assert_eq!(orchestrator.run_count(), 1);
    assert!(orchestrator.active_child_task().is_none());
    assert_eq!(orchestrator.events().len(), 3);
    assert!(matches!(
        orchestrator.events(),
        [
            AgentOrchestrationEvent::RootTaskCreated { .. },
            AgentOrchestrationEvent::TaskStarted {
                agent_id: AgentId::PersonalAssistant,
                ..
            },
            AgentOrchestrationEvent::RootCompleted { .. }
        ]
    ));
    Ok(())
}

#[test]
fn deterministic_personal_research_personal_flow_returns_attributed_result(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let accepted = orchestrator.request_delegation(&root, delegation()?)?;
    let child = accepted.child_context().clone();
    let child_id = child.task_id().clone();

    assert_eq!(
        accepted.request().source_agent_id(),
        AgentId::PersonalAssistant
    );
    assert_eq!(accepted.request().target_agent_id(), AgentId::Research);
    assert_eq!(child.root_task_id().task_id(), &root_id);
    assert_eq!(
        child.parent_task_id().map(|id| id.task_id()),
        Some(&root_id)
    );
    assert_eq!(child.depth(), 1);
    assert_eq!(child.runtime_id(), RuntimeId::Native);

    orchestrator.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    orchestrator.accept_runtime_event(&child_id, delta(&child, 1, RESEARCH_RESULT)?)?;
    orchestrator.accept_runtime_event(&child_id, completed(&child, 2))?;

    let synthesis = orchestrator.current_context(&root_id)?;
    assert_ne!(
        synthesis.runtime_run_identity(),
        root.runtime_run_identity()
    );
    orchestrator.accept_runtime_event(&root_id, started(&synthesis, 0, "synthesis-response")?)?;
    orchestrator.accept_runtime_event(&root_id, delta(&synthesis, 1, FINAL_SYNTHESIS)?)?;
    orchestrator.accept_runtime_event(&root_id, completed(&synthesis, 2))?;

    assert_eq!(orchestrator.task_count(), 2);
    assert_eq!(orchestrator.run_count(), 3);
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    assert_eq!(
        orchestrator.task(&child_id).map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    assert_eq!(
        orchestrator.child_outcome().map(|outcome| outcome.kind()),
        Some(AgentTaskOutcomeKind::Completed)
    );
    assert!(orchestrator.active_child_task().is_none());

    let starts = recorder.starts();
    assert_eq!(starts.len(), 3);
    assert_eq!(recorder.cancellations(), vec![starts[0].run_id.clone()]);
    assert!(starts[1].selected_text.contains(RESEARCH_OBJECTIVE));
    assert!(starts[2]
        .selected_text
        .contains("Research child outcome (untrusted"));
    assert!(starts[2].selected_text.contains(RESEARCH_RESULT));
    assert!(starts[2].selected_text.contains(ROOT_OBJECTIVE));

    let child_task = orchestrator.task(&child_id).ok_or("missing child task")?;
    assert!(matches!(
        child_task.outcome(),
        Some(AgentTaskOutcome::Completed(result))
            if result.task_id() == &child_id
                && result.agent_id() == AgentId::Research
                && result.output().as_str() == RESEARCH_RESULT
    ));
    let root_task = orchestrator.root_task().ok_or("missing root task")?;
    assert!(matches!(
        root_task.outcome(),
        Some(AgentTaskOutcome::Completed(result))
            if result.task_id() == &root_id
                && result.agent_id() == AgentId::PersonalAssistant
                && result.output().as_str() == FINAL_SYNTHESIS
    ));

    assert!(matches!(
        orchestrator.events(),
        [
            AgentOrchestrationEvent::RootTaskCreated { .. },
            AgentOrchestrationEvent::TaskStarted {
                agent_id: AgentId::PersonalAssistant,
                ..
            },
            AgentOrchestrationEvent::DelegationRequested { .. },
            AgentOrchestrationEvent::DelegationAccepted { .. },
            AgentOrchestrationEvent::ChildCreated { .. },
            AgentOrchestrationEvent::ChildStarted { .. },
            AgentOrchestrationEvent::ChildCompleted { .. },
            AgentOrchestrationEvent::ResultReturned {
                outcome: AgentTaskOutcomeKind::Completed,
                ..
            },
            AgentOrchestrationEvent::ParentResumed { .. },
            AgentOrchestrationEvent::RootCompleted { .. }
        ]
    ));
    Ok(())
}

#[test]
fn complete_delegated_scenario_is_deterministic_across_fresh_instances(
) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        delegated_scenario_evidence()?,
        delegated_scenario_evidence()?
    );
    Ok(())
}

#[test]
fn generic_delegation_denies_all_six_active_governed_specialists_and_unknown_targets(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let baseline = orchestrator.events().to_vec();

    for target in [
        AgentId::Coding,
        AgentId::CloudInfrastructure,
        AgentId::SystemsOperations,
        AgentId::QaValidation,
        AgentId::SecurityRisk,
        AgentId::WorkflowAutomation,
    ] {
        let proposal = DelegationProposal::new(
            target,
            "Inspect supplied fixtures",
            None,
            "Return a bounded proposal",
        )?;
        assert_eq!(
            orchestrator.request_delegation(&root, proposal),
            Err(AgentOrchestratorError::RouteDenied {
                source_agent_id: AgentId::PersonalAssistant,
                target,
            })
        );
        assert_eq!(orchestrator.events(), baseline);
        assert_eq!(orchestrator.task_count(), 1);
    }

    assert_eq!(orchestrator.task_count(), 1);
    assert_eq!(
        "unknown-specialist".parse::<AgentId>(),
        Err(AgentIdParseError::Unknown)
    );
    Ok(())
}

#[test]
fn unavailable_and_capability_incompatible_runtimes_are_rejected() {
    assert!(matches!(
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Unavailable)),
        Err(AgentOrchestratorError::RuntimeUnavailable)
    ));
    assert!(matches!(
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Unhealthy)),
        Err(AgentOrchestratorError::RuntimeUnhealthy)
    ));
    assert!(matches!(
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::CapabilityContradiction)),
        Err(AgentOrchestratorError::RuntimeCapabilityMissing { .. })
    ));
}

#[test]
fn research_cannot_delegate_and_total_child_budget_never_replenishes() -> Result<(), Box<dyn Error>>
{
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    assert_eq!(
        orchestrator.request_delegation(&child, delegation()?),
        Err(AgentOrchestratorError::UnauthorizedSource {
            agent_id: AgentId::Research,
        })
    );

    let child_id = child.task_id().clone();
    orchestrator.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    orchestrator.accept_runtime_event(&child_id, delta(&child, 1, RESEARCH_RESULT)?)?;
    orchestrator.accept_runtime_event(&child_id, completed(&child, 2))?;
    let synthesis = orchestrator.current_context(&root_id)?;
    assert_eq!(
        orchestrator.request_delegation(&synthesis, delegation()?),
        Err(AgentOrchestratorError::TotalChildLimitExceeded)
    );
    assert_eq!(orchestrator.task_count(), 2);
    Ok(())
}

#[test]
fn child_failure_returns_typed_outcome_and_parent_can_synthesize_fallback(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    orchestrator.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    orchestrator.accept_runtime_event(
        &child_id,
        RuntimeEventEnvelope::for_identity(
            child.runtime_run_identity(),
            1,
            UntrustedRuntimeEvent::ResponseFailed {
                failure: RuntimeFailure::new(RuntimeFailureCode::ProviderTimeout, true, None)?,
            },
        ),
    )?;

    assert_eq!(
        orchestrator.task(&child_id).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(matches!(
        orchestrator.child_outcome(),
        Some(ai_agent_assistant_lib::agent::task::AgentTaskOutcome::Failed(failure))
            if failure.code()
                == AgentTaskFailureCode::RuntimeReported(RuntimeFailureCode::ProviderTimeout)
    ));
    let synthesis = orchestrator.current_context(&root_id)?;
    orchestrator.accept_runtime_event(&root_id, started(&synthesis, 0, "fallback-response")?)?;
    orchestrator
        .accept_runtime_event(&root_id, delta(&synthesis, 1, "Research was unavailable.")?)?;
    orchestrator.accept_runtime_event(&root_id, completed(&synthesis, 2))?;
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    Ok(())
}

#[test]
fn root_cancellation_propagates_child_first_and_rejects_late_events() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();

    assert_eq!(
        orchestrator.cancel_task(&root_id)?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.task(&child_id).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(orchestrator.active_child_task().is_none());
    let starts = recorder.starts();
    assert_eq!(starts.len(), 2);
    assert_eq!(
        recorder.cancellations(),
        vec![starts[0].run_id.clone(), starts[1].run_id.clone()]
    );
    let events = orchestrator.events();
    assert!(matches!(
        &events[events.len() - 2..],
        [
            AgentOrchestrationEvent::TaskCancelled {
                agent_id: AgentId::Research,
                ..
            },
            AgentOrchestrationEvent::TaskCancelled {
                agent_id: AgentId::PersonalAssistant,
                ..
            }
        ]
    ));
    assert_eq!(
        orchestrator.accept_runtime_event(&child_id, started(&child, 0, "late-response")?),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    Ok(())
}

#[test]
fn cancellation_before_delegation_is_terminal_and_idempotent() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    assert_eq!(
        orchestrator.cancel_task(&root_id)?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator.cancel_task(&root_id)?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::AlreadyTerminal(
            AgentTaskStatus::Cancelled
        )
    );
    assert_eq!(orchestrator.task_count(), 1);
    let starts = recorder.starts();
    assert_eq!(recorder.cancellations(), vec![starts[0].run_id.clone()]);
    assert!(orchestrator.active_child_task().is_none());
    assert_eq!(
        orchestrator.request_delegation(&root, delegation()?),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    Ok(())
}

#[test]
fn independent_child_cancellation_returns_result_and_resumes_parent() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    assert_eq!(
        orchestrator.cancel_task(&child_id)?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator.task(&child_id).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.child_outcome().map(|outcome| outcome.kind()),
        Some(AgentTaskOutcomeKind::Cancelled)
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(orchestrator.active_child_task().is_none());
    assert!(orchestrator.current_context(&root_id).is_ok());
    let starts = recorder.starts();
    assert_eq!(starts.len(), 3);
    assert_eq!(
        recorder.cancellations(),
        vec![starts[0].run_id.clone(), starts[1].run_id.clone()]
    );
    Ok(())
}

#[test]
fn delegation_after_runtime_output_and_tool_proposals_fail_closed() -> Result<(), Box<dyn Error>> {
    let mut after_output = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = after_output.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    after_output.accept_runtime_event(&root_id, started(&root, 0, "partial-response")?)?;
    assert_eq!(
        after_output.request_delegation(&root, delegation()?),
        Err(AgentOrchestratorError::DelegationAfterRuntimeOutput)
    );
    assert_eq!(after_output.task_count(), 1);

    let mut tool = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let tool_root = tool.start_root(ROOT_OBJECTIVE)?;
    let tool_root_id = tool_root.task_id().clone();
    let proposal = UntrustedRuntimeToolProposal::new(
        "tool-call-1",
        "create_local_task",
        1,
        r#"{"title":"synthetic"}"#,
    )?;
    assert_eq!(
        tool.accept_runtime_event(
            &tool_root_id,
            RuntimeEventEnvelope::for_identity(
                tool_root.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ToolProposal { proposal },
            ),
        ),
        Err(AgentOrchestratorError::ToolProposalUnsupported)
    );
    assert_eq!(
        tool.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    Ok(())
}

#[test]
fn foreign_and_out_of_sequence_tool_proposals_cannot_terminalize_a_live_task(
) -> Result<(), Box<dyn Error>> {
    let mut foreign_source = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let foreign = foreign_source.start_root("Foreign workflow")?;
    let mut target = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = target.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let baseline = target.events().to_vec();

    let foreign_proposal = UntrustedRuntimeToolProposal::new(
        "foreign-tool-call",
        "create_local_task",
        1,
        r#"{"title":"synthetic"}"#,
    )?;
    assert_eq!(
        target.accept_runtime_event(
            &root_id,
            RuntimeEventEnvelope::for_identity(
                foreign.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ToolProposal {
                    proposal: foreign_proposal,
                },
            ),
        ),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::IdentityMismatch)
        ))
    );
    assert_eq!(target.events(), baseline);
    assert_eq!(
        target.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );

    let out_of_sequence = UntrustedRuntimeToolProposal::new(
        "out-of-sequence-tool-call",
        "create_local_task",
        1,
        r#"{"title":"synthetic"}"#,
    )?;
    assert_eq!(
        target.accept_runtime_event(
            &root_id,
            RuntimeEventEnvelope::for_identity(
                root.runtime_run_identity(),
                1,
                UntrustedRuntimeEvent::ToolProposal {
                    proposal: out_of_sequence,
                },
            ),
        ),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::InvalidSequence)
        ))
    );
    assert_eq!(target.events(), baseline);
    assert_eq!(
        target.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    Ok(())
}

#[test]
fn workflow_namespaces_reject_cross_instance_contexts_and_events() -> Result<(), Box<dyn Error>> {
    let mut first = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let first_root = first.start_root("First workflow")?;
    let mut second = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let second_root = second.start_root("Second workflow")?;
    let second_id = second_root.task_id().clone();

    assert_ne!(first_root.task_id(), second_root.task_id());
    assert_ne!(
        first_root.runtime_run_identity(),
        second_root.runtime_run_identity()
    );
    let baseline = second.events().to_vec();
    assert_eq!(
        second.request_delegation(&first_root, delegation()?),
        Err(AgentOrchestratorError::TaskNotFound)
    );
    assert_eq!(
        second.accept_runtime_event(
            &second_id,
            started(&first_root, 0, "cross-workflow-response")?
        ),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::IdentityMismatch)
        ))
    );
    assert_eq!(second.events(), baseline);
    assert_eq!(
        second.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    Ok(())
}

#[test]
fn self_reverse_and_specialist_routes_are_denied_before_mutation() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let baseline = orchestrator.events().to_vec();
    let self_route =
        DelegationProposal::new(AgentId::PersonalAssistant, "Self route", None, "Reject")?;
    assert_eq!(
        orchestrator.request_delegation(&root, self_route),
        Err(AgentOrchestratorError::RouteDenied {
            source_agent_id: AgentId::PersonalAssistant,
            target: AgentId::PersonalAssistant,
        })
    );
    assert_eq!(orchestrator.events(), baseline);

    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let after_child = orchestrator.events().to_vec();
    let reverse =
        DelegationProposal::new(AgentId::PersonalAssistant, "Reverse route", None, "Reject")?;
    assert_eq!(
        orchestrator.request_delegation(&child, reverse),
        Err(AgentOrchestratorError::UnauthorizedSource {
            agent_id: AgentId::Research,
        })
    );
    assert_eq!(orchestrator.events(), after_child);
    assert_eq!(orchestrator.task_count(), 2);
    Ok(())
}

#[test]
fn cumulative_output_bound_terminalizes_the_task_without_retaining_extra_content(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    orchestrator.accept_runtime_event(&root_id, started(&root, 0, "bounded-response")?)?;
    orchestrator.accept_runtime_event(
        &root_id,
        delta(&root, 1, &"x".repeat(MAX_AGENT_TASK_OUTPUT_CHARACTERS))?,
    )?;
    assert_eq!(
        orchestrator.accept_runtime_event(&root_id, delta(&root, 2, "x")?),
        Err(AgentOrchestratorError::OutputLimitExceeded)
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(orchestrator.runtime_event_count(), 3);
    Ok(())
}

#[test]
fn runtime_event_cap_validates_attribution_then_closes_the_entire_delegated_workflow(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    orchestrator.accept_runtime_event(&child_id, started(&child, 0, "bounded-response")?)?;
    for sequence in 1..u32::try_from(MAX_RUNTIME_EVENTS_PER_ROOT)? {
        orchestrator.accept_runtime_event(&child_id, delta(&child, sequence, "x")?)?;
    }
    assert_eq!(
        orchestrator.runtime_event_count(),
        MAX_RUNTIME_EVENTS_PER_ROOT
    );
    let baseline = orchestrator.events().to_vec();

    assert_eq!(
        orchestrator.accept_runtime_event(
            &root_id,
            delta(
                &child,
                u32::try_from(MAX_RUNTIME_EVENTS_PER_ROOT)?,
                "stale-root"
            )?,
        ),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    assert_eq!(
        orchestrator.accept_runtime_event(
            &child_id,
            delta(
                &root,
                u32::try_from(MAX_RUNTIME_EVENTS_PER_ROOT)?,
                "foreign-run"
            )?,
        ),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::IdentityMismatch)
        ))
    );
    assert_eq!(
        orchestrator.accept_runtime_event(
            &child_id,
            delta(
                &child,
                u32::try_from(MAX_RUNTIME_EVENTS_PER_ROOT + 1)?,
                "out-of-sequence"
            )?,
        ),
        Err(AgentOrchestratorError::Runtime(
            RuntimeError::EventRejected(RuntimeEventRejection::InvalidSequence)
        ))
    );
    assert_eq!(orchestrator.events(), baseline);
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::WaitingForChild)
    );
    assert_eq!(
        orchestrator.task(&child_id).map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );

    assert_eq!(
        orchestrator.accept_runtime_event(
            &child_id,
            delta(
                &child,
                u32::try_from(MAX_RUNTIME_EVENTS_PER_ROOT)?,
                "bounded-limit"
            )?,
        ),
        Err(AgentOrchestratorError::RuntimeEventLimitExceeded)
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        orchestrator.task(&child_id).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(orchestrator.active_child_task().is_none());
    assert_eq!(
        orchestrator.current_context(&root_id),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    assert_eq!(
        orchestrator.current_context(&child_id),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    Ok(())
}

#[test]
fn runtime_start_failures_leave_no_orphaned_task_or_run() -> Result<(), Box<dyn Error>> {
    let mut child_start =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailureAt(2)))?;
    let root = child_start.start_root(ROOT_OBJECTIVE)?;
    assert!(matches!(
        child_start.request_delegation(&root, delegation()?),
        Err(AgentOrchestratorError::Runtime(_))
    ));
    assert_eq!(child_start.task_count(), 1);
    assert!(child_start.active_child_task().is_none());
    assert_eq!(
        child_start.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );

    let mut synthesis_start =
        AgentOrchestrator::new(MockAgentRuntime::new(MockMode::StartFailureAt(3)))?;
    let root = synthesis_start.start_root(ROOT_OBJECTIVE)?;
    let child = synthesis_start
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    synthesis_start.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    synthesis_start.accept_runtime_event(&child_id, delta(&child, 1, RESEARCH_RESULT)?)?;
    assert!(matches!(
        synthesis_start.accept_runtime_event(&child_id, completed(&child, 2)),
        Err(AgentOrchestratorError::Runtime(_))
    ));
    assert_eq!(
        synthesis_start.task(&child_id).map(|task| task.status()),
        Some(AgentTaskStatus::Completed)
    );
    assert_eq!(
        synthesis_start.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(synthesis_start.active_child_task().is_none());
    Ok(())
}

#[test]
fn unexpected_initial_run_status_is_cancelled_and_rejected_at_every_stage(
) -> Result<(), Box<dyn Error>> {
    let (root_runtime, root_recorder) = MockAgentRuntime::recording(
        MockMode::UnexpectedStartStatusAt(1, RuntimeRunStatus::Streaming),
    );
    let mut root_start = AgentOrchestrator::new(root_runtime)?;
    assert_eq!(
        root_start.start_root(ROOT_OBJECTIVE),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Streaming,
        })
    );
    assert_eq!(root_start.task_count(), 0);
    assert_eq!(root_recorder.cancellations().len(), 1);

    let (child_runtime, child_recorder) = MockAgentRuntime::recording(
        MockMode::UnexpectedStartStatusAt(2, RuntimeRunStatus::Streaming),
    );
    let mut child_start = AgentOrchestrator::new(child_runtime)?;
    let root = child_start.start_root(ROOT_OBJECTIVE)?;
    assert_eq!(
        child_start.request_delegation(&root, delegation()?),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Streaming,
        })
    );
    assert_eq!(child_start.task_count(), 1);
    assert_eq!(
        child_start.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(child_recorder.cancellations().len(), 2);

    let (synthesis_runtime, synthesis_recorder) = MockAgentRuntime::recording(
        MockMode::UnexpectedStartStatusAt(3, RuntimeRunStatus::Streaming),
    );
    let mut synthesis = AgentOrchestrator::new(synthesis_runtime)?;
    let root = synthesis.start_root(ROOT_OBJECTIVE)?;
    let child = synthesis
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    synthesis.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    synthesis.accept_runtime_event(&child_id, delta(&child, 1, RESEARCH_RESULT)?)?;
    assert_eq!(
        synthesis.accept_runtime_event(&child_id, completed(&child, 2)),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Streaming,
        })
    );
    assert_eq!(
        synthesis.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(synthesis_recorder.cancellations().len(), 2);
    Ok(())
}

#[test]
fn cancellation_failure_or_wrong_terminal_status_never_falsely_cancels_task(
) -> Result<(), Box<dyn Error>> {
    let (failure_runtime, failure_recorder) =
        MockAgentRuntime::recording(MockMode::CancelFailureAt(1));
    let mut failure = AgentOrchestrator::new(failure_runtime)?;
    let failure_root = failure.start_root(ROOT_OBJECTIVE)?;
    let failure_id = failure_root.task_id().clone();
    let failure_events = failure.events().to_vec();
    assert!(matches!(
        failure.cancel_task(&failure_id),
        Err(AgentOrchestratorError::Runtime(_))
    ));
    assert_eq!(failure.events(), failure_events);
    assert_eq!(
        failure.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(failure.current_context(&failure_id).is_ok());
    assert!(failure_recorder.cancellations().is_empty());

    let (child_failure_runtime, child_failure_recorder) =
        MockAgentRuntime::recording(MockMode::CancelFailureAt(2));
    let mut child_failure = AgentOrchestrator::new(child_failure_runtime)?;
    let child_failure_root = child_failure.start_root(ROOT_OBJECTIVE)?;
    let child_failure_root_id = child_failure_root.task_id().clone();
    let child_failure_context = child_failure
        .request_delegation(&child_failure_root, delegation()?)?
        .child_context()
        .clone();
    let child_failure_id = child_failure_context.task_id().clone();
    let child_failure_events = child_failure.events().to_vec();
    assert!(matches!(
        child_failure.cancel_task(&child_failure_root_id),
        Err(AgentOrchestratorError::Runtime(_))
    ));
    assert_eq!(child_failure.events(), child_failure_events);
    assert_eq!(
        child_failure.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::WaitingForChild)
    );
    assert_eq!(
        child_failure
            .task(&child_failure_id)
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(child_failure.current_context(&child_failure_id).is_ok());
    assert_eq!(child_failure_recorder.cancellations().len(), 1);

    let contradictory_runtime = MockAgentRuntime::new(MockMode::CancelAlreadyTerminalAt(
        2,
        RuntimeRunStatus::Streaming,
    ));
    let mut contradictory = AgentOrchestrator::new(contradictory_runtime)?;
    let contradictory_root = contradictory.start_root(ROOT_OBJECTIVE)?;
    let contradictory_root_id = contradictory_root.task_id().clone();
    let contradictory_child = contradictory
        .request_delegation(&contradictory_root, delegation()?)?
        .child_context()
        .clone();
    let contradictory_child_id = contradictory_child.task_id().clone();
    let contradictory_events = contradictory.events().to_vec();
    assert_eq!(
        contradictory.cancel_task(&contradictory_root_id),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Streaming,
        })
    );
    assert_eq!(contradictory.events(), contradictory_events);
    assert_eq!(
        contradictory.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::WaitingForChild)
    );
    assert_eq!(
        contradictory
            .task(&contradictory_child_id)
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(contradictory
        .current_context(&contradictory_child_id)
        .is_ok());

    let terminal_runtime = MockAgentRuntime::new(MockMode::CancelAlreadyTerminalAt(
        1,
        RuntimeRunStatus::Completed,
    ));
    let mut terminal = AgentOrchestrator::new(terminal_runtime)?;
    let terminal_root = terminal.start_root(ROOT_OBJECTIVE)?;
    let terminal_id = terminal_root.task_id().clone();
    assert_eq!(
        terminal.cancel_task(&terminal_id),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Completed,
        })
    );
    assert_eq!(
        terminal.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(matches!(
        terminal.root_task().and_then(|task| task.outcome()),
        Some(AgentTaskOutcome::Failed(failure))
            if failure.code() == AgentTaskFailureCode::RuntimeStateMismatch
    ));
    assert!(matches!(
        terminal.events().last(),
        Some(AgentOrchestrationEvent::RootFailed {
            code: AgentTaskFailureCode::RuntimeStateMismatch,
            ..
        })
    ));
    assert_eq!(
        terminal.current_context(&terminal_id),
        Err(AgentOrchestratorError::NoActiveRun)
    );

    let delegation_runtime = MockAgentRuntime::new(MockMode::CancelAlreadyTerminalAt(
        1,
        RuntimeRunStatus::Failed,
    ));
    let mut delegation_orchestrator = AgentOrchestrator::new(delegation_runtime)?;
    let delegation_root = delegation_orchestrator.start_root(ROOT_OBJECTIVE)?;
    let delegation_id = delegation_root.task_id().clone();
    assert_eq!(
        delegation_orchestrator.request_delegation(&delegation_root, delegation()?),
        Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
            status: RuntimeRunStatus::Failed,
        })
    );
    assert_eq!(delegation_orchestrator.task_count(), 1);
    assert!(delegation_orchestrator.active_child_task().is_none());
    assert_eq!(
        delegation_orchestrator
            .root_task()
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        delegation_orchestrator.current_context(&delegation_id),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    Ok(())
}

#[test]
fn stale_context_and_foreign_runtime_events_are_rejected_without_rebinding(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    let root_id = root.task_id().clone();
    let child = orchestrator
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    orchestrator.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    orchestrator.accept_runtime_event(&child_id, delta(&child, 1, RESEARCH_RESULT)?)?;
    orchestrator.accept_runtime_event(&child_id, completed(&child, 2))?;
    assert_eq!(
        orchestrator.request_delegation(&root, delegation()?),
        Err(AgentOrchestratorError::ContextMismatch)
    );

    let synthesis = orchestrator.current_context(&root_id)?;
    let foreign = RuntimeEventEnvelope::for_identity(
        child.runtime_run_identity(),
        0,
        UntrustedRuntimeEvent::ResponseStarted {
            response_id: RuntimeResponseId::new("foreign-response")?,
        },
    );
    assert!(matches!(
        orchestrator.accept_runtime_event(&root_id, foreign),
        Err(AgentOrchestratorError::Runtime(_))
    ));
    assert_ne!(
        synthesis.runtime_run_identity(),
        child.runtime_run_identity()
    );
    assert_eq!(
        orchestrator.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    Ok(())
}

#[test]
fn legacy_root_rejects_foreign_returned_identity_before_trusted_state() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchAt(1));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;

    assert_eq!(
        orchestrator.start_root(ROOT_OBJECTIVE),
        Err(AgentOrchestratorError::RuntimeIdentityMismatch)
    );
    assert_eq!(orchestrator.task_count(), 0);
    assert_eq!(orchestrator.run_count(), 0);
    assert!(orchestrator.events().is_empty());
    assert!(recorder.live_runs().is_empty());
    assert_eq!(recorder.cancellations().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn legacy_rejected_root_is_quarantined_and_blocks_restart_until_cleanup(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) =
        MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchWithCancelFailureOnceAt(1));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;

    assert_eq!(
        orchestrator.start_root(ROOT_OBJECTIVE),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(recorder.starts().len(), 1);
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());
    assert_eq!(
        orchestrator.start_root(ROOT_OBJECTIVE),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(recorder.starts().len(), 1);
    assert_eq!(recorder.live_runs().len(), 1);

    orchestrator.retry_rejected_runtime_cleanup()?;
    assert!(recorder.live_runs().is_empty());
    assert_eq!(recorder.cancellations().len(), 1);
    let root = orchestrator.start_root(ROOT_OBJECTIVE)?;
    assert_eq!(root.agent_id(), AgentId::PersonalAssistant);
    assert_eq!(recorder.starts().len(), 2);
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn contradictory_nonterminal_rejection_disposition_remains_quarantined(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(
        MockMode::UnexpectedStartStatusWithCancelAlreadyTerminalOnceAt(
            1,
            RuntimeRunStatus::Streaming,
            RuntimeRunStatus::AwaitingStart,
        ),
    );
    let mut orchestrator = AgentOrchestrator::new(runtime)?;

    assert_eq!(
        orchestrator.start_root(ROOT_OBJECTIVE),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(recorder.starts().len(), 1);
    assert_eq!(recorder.live_runs().len(), 1);
    assert_eq!(
        recorder.live_runs()[0].status,
        RuntimeRunStatus::AwaitingStart
    );
    assert!(recorder.cancellations().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());

    orchestrator.retry_rejected_runtime_cleanup()?;
    assert!(recorder.live_runs().is_empty());
    assert_eq!(recorder.cancellations().len(), 1);
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn permanent_rejection_cancellation_failure_stays_closed_and_owned() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(
        MockMode::UnexpectedStartStatusWithCancelFailureAt(1, RuntimeRunStatus::Streaming),
    );
    let mut orchestrator = AgentOrchestrator::new(runtime)?;

    assert_eq!(
        orchestrator.start_root(ROOT_OBJECTIVE),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(
        orchestrator.retry_rejected_runtime_cleanup(),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(
        orchestrator.retry_rejected_runtime_cleanup(),
        Err(AgentOrchestratorError::RuntimeCleanupPending)
    );
    assert_eq!(recorder.starts().len(), 1);
    assert_eq!(recorder.live_runs().len(), 1);
    assert!(recorder.cancellations().is_empty());
    assert!(recorder.terminal_dispositions().is_empty());
    assert!(recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn legacy_child_and_synthesis_reject_foreign_returned_identity() -> Result<(), Box<dyn Error>> {
    let (child_runtime, child_recorder) =
        MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchAt(2));
    let mut child_start = AgentOrchestrator::new(child_runtime)?;
    let child_root = child_start.start_root(ROOT_OBJECTIVE)?;
    assert_eq!(
        child_start.request_delegation(&child_root, delegation()?),
        Err(AgentOrchestratorError::RuntimeIdentityMismatch)
    );
    assert_eq!(
        child_start.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(child_recorder.live_runs().is_empty());
    assert!(child_recorder.nonterminal_drops().is_empty());

    let (synthesis_runtime, synthesis_recorder) =
        MockAgentRuntime::recording(MockMode::ReturnedIdentityMismatchAt(3));
    let mut synthesis_start = AgentOrchestrator::new(synthesis_runtime)?;
    let root = synthesis_start.start_root(ROOT_OBJECTIVE)?;
    let child = synthesis_start
        .request_delegation(&root, delegation()?)?
        .child_context()
        .clone();
    let child_id = child.task_id().clone();
    synthesis_start.accept_runtime_event(&child_id, started(&child, 0, "research-response")?)?;
    synthesis_start.accept_runtime_event(&child_id, delta(&child, 1, RESEARCH_RESULT)?)?;
    assert_eq!(
        synthesis_start.accept_runtime_event(&child_id, completed(&child, 2)),
        Err(AgentOrchestratorError::RuntimeIdentityMismatch)
    );
    assert_eq!(
        synthesis_start.root_task().map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(synthesis_recorder.live_runs().is_empty());
    assert!(synthesis_recorder.nonterminal_drops().is_empty());
    Ok(())
}

#[test]
fn native_runtime_remains_the_default_orchestration_runtime() -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root("Native regression")?;
    assert_eq!(root.runtime_id(), RuntimeId::Native);
    let root_id = root.task_id().clone();
    assert_eq!(
        orchestrator.cancel_task(&root_id)?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    Ok(())
}

#[test]
fn debug_and_events_do_not_expose_task_content() -> Result<(), Box<dyn Error>> {
    let sentinel = "private-orchestration-sentinel";
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root(sentinel)?;
    let proposal = DelegationProposal::new(
        AgentId::Research,
        sentinel,
        Some(sentinel.to_owned()),
        sentinel,
    )?;
    let acceptance = orchestrator.request_delegation(&root, proposal)?;
    assert!(!format!("{orchestrator:?}").contains(sentinel));
    assert!(!format!("{:?}", orchestrator.events()).contains(sentinel));
    assert!(!format!("{:?}", acceptance.request()).contains(sentinel));
    Ok(())
}
