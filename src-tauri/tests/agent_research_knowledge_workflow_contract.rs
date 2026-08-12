use std::error::Error;

use ai_agent_assistant_lib::agent::definition::AgentId;
use ai_agent_assistant_lib::agent::orchestrator::{
    AgentOrchestrator, AgentOrchestratorError, ResearchKnowledgeWorkflowAcceptance,
    MAX_RESEARCH_KNOWLEDGE_CHILDREN_PER_ROOT, MAX_RESEARCH_KNOWLEDGE_RUNTIME_RUNS_PER_ROOT,
    MAX_RESEARCH_KNOWLEDGE_TASKS_PER_ROOT, MAX_RUNTIME_EVENTS_PER_ROOT,
};
use ai_agent_assistant_lib::agent::research_knowledge::{
    FinalSynthesisResultVersion, FinalSynthesisStatus, KnowledgeResultQuality,
    KnowledgeStageOutcome, ResearchKnowledgeContinuationFailure,
    ResearchKnowledgePartialFailureCode, ResearchKnowledgeStage, ResearchKnowledgeWorkflowEvent,
    ResearchKnowledgeWorkflowRequest, ResearchResultQuality, ResearchStageOutcome, WorkflowSource,
    WorkflowSourceCatalog,
};
use ai_agent_assistant_lib::agent::runtime::{
    RuntimeEventAcceptance, RuntimeEventEnvelope, RuntimeFailure, RuntimeFailureCode, RuntimeId,
    RuntimeOutputText, RuntimeResponseId, UntrustedRuntimeEvent,
};
use ai_agent_assistant_lib::agent::task::{AgentTaskOutcome, AgentTaskStatus};
use ai_agent_assistant_lib::memory::{MemoryContent, MemoryWriteTarget};

mod support;

use support::mock_agent_runtime::{MockAgentRuntime, MockMode};

const OBJECTIVE: &str = "Compare two technical approaches and create a structured decision brief";
const RESEARCH_JSON: &str = r#"{"findings":[{"statement":"Approach A is simpler and less costly","source_ids":["approach-a"],"confidence":"high"},{"statement":"Approach B has higher scale potential","source_ids":["approach-b"],"confidence":"medium"}],"unresolved_questions":["What scale is required?"],"limitations":["Only deterministic fixture evidence was supplied"],"recommended_follow_up":"Review against approved operational requirements"}"#;
const KNOWLEDGE_JSON: &str = r#"{"sections":[{"heading":"Decision factors","body":"A favors simplicity while B favors scale.","source_ids":["approach-a","approach-b"]}],"extracted_facts":[{"statement":"A has lower operating cost.","source_ids":["approach-a"]}],"contradictions":[],"summary":"Choose according to required scale and operations capacity.","reusable_knowledge_proposal":"Retain this comparison only after explicit application review.","artifact_outline":"Context; evidence; trade-offs; decision","incomplete":false}"#;
const FINAL_ANSWER: &str =
    "Fixture-based decision brief: A is simpler [approach-a]; B may scale further [approach-b]. Verify before acting.";
const FINAL_SYNTHESIS: &str = r#"{"version":"v1","answer":"Fixture-based decision brief: A is simpler [approach-a]; B may scale further [approach-b]. Verify before acting.","source_ids":["approach-a","approach-b"],"fixture_based":true,"status":"complete"}"#;
const PARTIAL_SYNTHESIS_NO_SOURCES: &str = r#"{"version":"v1","answer":"The fixture workflow is partial because Research was unavailable.","source_ids":[],"fixture_based":true,"status":"partial"}"#;
const PARTIAL_SYNTHESIS_WITH_SOURCES: &str = r#"{"version":"v1","answer":"The fixture workflow is partial; validated Research evidence remains available.","source_ids":["approach-a","approach-b"],"fixture_based":true,"status":"partial"}"#;

fn workflow_request() -> Result<ResearchKnowledgeWorkflowRequest, Box<dyn Error>> {
    Ok(ResearchKnowledgeWorkflowRequest::new(
        OBJECTIVE,
        WorkflowSourceCatalog::new(vec![
            WorkflowSource::deterministic_fixture(
                "approach-a",
                "Approach A fixture",
                "Approach A is simpler and has lower operating cost.",
            )?,
            WorkflowSource::deterministic_fixture(
                "approach-b",
                "Approach B fixture",
                "Approach B scales further but needs more operations work.",
            )?,
        ])?,
    )?)
}

fn started(
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
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
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
    text: &str,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        1,
        UntrustedRuntimeEvent::OutputTextDelta {
            delta: RuntimeOutputText::new(text)?,
        },
    ))
}

fn completed(
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
) -> RuntimeEventEnvelope {
    RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        2,
        UntrustedRuntimeEvent::ResponseCompleted,
    )
}

fn failed(
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
) -> Result<RuntimeEventEnvelope, Box<dyn Error>> {
    Ok(RuntimeEventEnvelope::for_identity(
        context.runtime_run_identity(),
        1,
        UntrustedRuntimeEvent::ResponseFailed {
            failure: RuntimeFailure::new(RuntimeFailureCode::ProviderUnavailable, true, None)?,
        },
    ))
}

fn start_workflow(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<
    (
        ai_agent_assistant_lib::agent::task::AgentExecutionContext,
        ai_agent_assistant_lib::agent::task::AgentExecutionContext,
    ),
    Box<dyn Error>,
> {
    let root = orchestrator.start_root(OBJECTIVE)?;
    let accepted = orchestrator.request_research_knowledge_workflow(&root, workflow_request()?)?;
    let research = match accepted {
        ResearchKnowledgeWorkflowAcceptance::ResearchStarted { context } => context,
        ResearchKnowledgeWorkflowAcceptance::PersonalFallbackStarted { .. } => {
            return Err("Research unexpectedly failed to start".into());
        }
    };
    Ok((root, research))
}

fn complete_stage(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
    context: &ai_agent_assistant_lib::agent::task::AgentExecutionContext,
    response_id: &str,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    let task_id = context.task_id().clone();
    orchestrator.accept_runtime_event(&task_id, started(context, response_id)?)?;
    orchestrator.accept_runtime_event(&task_id, delta(context, output)?)?;
    orchestrator.accept_runtime_event(&task_id, completed(context))?;
    Ok(())
}

fn advance_to_synthesis(
    orchestrator: &mut AgentOrchestrator<MockAgentRuntime>,
) -> Result<
    (
        ai_agent_assistant_lib::agent::task::AgentExecutionContext,
        ai_agent_assistant_lib::agent::task::AgentExecutionContext,
    ),
    Box<dyn Error>,
> {
    let (root, research) = start_workflow(orchestrator)?;
    complete_stage(orchestrator, &research, "research-ok", RESEARCH_JSON)?;
    let knowledge = orchestrator
        .active_child_task()
        .and_then(|task| orchestrator.current_context(task.id()).ok())
        .ok_or("missing Knowledge context")?;
    complete_stage(orchestrator, &knowledge, "knowledge-ok", KNOWLEDGE_JSON)?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    Ok((root, synthesis))
}

#[test]
fn deterministic_fixture_workflow_preserves_provenance_and_exact_order(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    let root_id = root.task_id().clone();
    let research_id = research.task_id().clone();

    assert_eq!(research.agent_id(), AgentId::Research);
    assert_eq!(research.depth(), 1);
    assert_eq!(research.root_task_id().task_id(), &root_id);
    assert_eq!(
        research.parent_task_id().map(|id| id.task_id()),
        Some(&root_id)
    );
    assert_eq!(research.runtime_id(), RuntimeId::Native);
    let research_memory = orchestrator.write_memory(
        &research,
        MemoryWriteTarget::TaskTemporary,
        MemoryContent::new("temporary Research notes")?,
    )?;
    let research_memory_id = research_memory.id().clone();
    complete_stage(
        &mut orchestrator,
        &research,
        "research-response",
        RESEARCH_JSON,
    )?;

    let knowledge = orchestrator
        .active_child_task()
        .and_then(|task| orchestrator.current_context(task.id()).ok())
        .ok_or("missing Knowledge context")?;
    let knowledge_id = knowledge.task_id().clone();
    assert_eq!(knowledge.agent_id(), AgentId::KnowledgeDocument);
    assert_eq!(knowledge.depth(), 1);
    assert_eq!(knowledge.root_task_id().task_id(), &root_id);
    assert_eq!(
        knowledge.parent_task_id().map(|id| id.task_id()),
        Some(&root_id)
    );
    assert_ne!(knowledge_id, research_id);
    let knowledge_memory = orchestrator.write_memory(
        &knowledge,
        MemoryWriteTarget::TaskTemporary,
        MemoryContent::new("temporary Knowledge notes")?,
    )?;
    let knowledge_memory_id = knowledge_memory.id().clone();
    complete_stage(
        &mut orchestrator,
        &knowledge,
        "knowledge-response",
        KNOWLEDGE_JSON,
    )?;

    let synthesis = orchestrator.current_context(&root_id)?;
    assert!(orchestrator
        .read_memory(&synthesis, &research_memory_id)
        .is_err());
    assert!(orchestrator
        .read_memory(&synthesis, &knowledge_memory_id)
        .is_err());
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "synthesis-response",
        FINAL_SYNTHESIS,
    )?;

    assert_eq!(
        orchestrator.task_count(),
        MAX_RESEARCH_KNOWLEDGE_TASKS_PER_ROOT
    );
    assert_eq!(
        orchestrator.run_count(),
        MAX_RESEARCH_KNOWLEDGE_RUNTIME_RUNS_PER_ROOT
    );
    assert_eq!(MAX_RESEARCH_KNOWLEDGE_CHILDREN_PER_ROOT, 2);
    assert!(orchestrator.active_child_task().is_none());
    assert_eq!(orchestrator.shared_memory_proposal_count(), 0);
    assert!(matches!(
        orchestrator.task(&root_id).and_then(|task| task.outcome()),
        Some(AgentTaskOutcome::Completed(result)) if result.output().as_str() == FINAL_ANSWER
    ));

    let result = orchestrator
        .research_knowledge_result()
        .ok_or("missing workflow result")?;
    assert!(result.fixture_based());
    assert!(matches!(
        result.research(),
        ResearchStageOutcome::Completed(research)
            if research.task_id() == &research_id
                && research.quality() == ResearchResultQuality::Complete
                && research.findings().iter().all(|finding| !finding.source_ids().is_empty())
    ));
    assert!(matches!(
        result.knowledge(),
        KnowledgeStageOutcome::Completed(knowledge)
            if knowledge.task_id() == &knowledge_id
                && knowledge.predecessor().task_id() == &research_id
                && knowledge.quality() == KnowledgeResultQuality::Complete
                && knowledge.reusable_knowledge_proposal().is_some()
    ));
    assert_eq!(
        result.synthesis().version(),
        FinalSynthesisResultVersion::V1
    );
    assert_eq!(result.synthesis().answer(), FINAL_ANSWER);
    assert_eq!(result.synthesis().status(), FinalSynthesisStatus::Complete);
    assert!(result.synthesis().fixture_based());
    assert_eq!(
        result
            .synthesis()
            .source_ids()
            .iter()
            .map(|source_id| source_id.as_str())
            .collect::<Vec<_>>(),
        ["approach-a", "approach-b"]
    );
    assert!(matches!(
        orchestrator.research_knowledge_events(),
        [
            ResearchKnowledgeWorkflowEvent::ResearchStarted { .. },
            ResearchKnowledgeWorkflowEvent::ResearchCompleted { .. },
            ResearchKnowledgeWorkflowEvent::KnowledgeOrganizationStarted { .. },
            ResearchKnowledgeWorkflowEvent::KnowledgeOrganizationCompleted { .. },
            ResearchKnowledgeWorkflowEvent::SynthesisStarted { .. },
            ResearchKnowledgeWorkflowEvent::Completed { .. },
        ]
    ));
    let audit = orchestrator.research_knowledge_audit_records();
    assert_eq!(audit.len(), 6);
    assert_eq!(audit[0].agent_id(), AgentId::Research);
    assert_eq!(audit[2].agent_id(), AgentId::KnowledgeDocument);
    assert_eq!(audit[4].agent_id(), AgentId::PersonalAssistant);
    assert_eq!(audit[0].attribution().runtime_id(), RuntimeId::Native);
    assert_eq!(audit[0].attribution().root_task_id().task_id(), &root_id);
    assert_eq!(audit[2].predecessor_task_id(), Some(&research_id));
    assert_eq!(
        audit[2].attribution().memory_profile_id(),
        ai_agent_assistant_lib::memory::AgentMemoryProfileId::KnowledgeWorkingMemoryV1
    );
    assert!(audit
        .windows(2)
        .all(|pair| pair[0].sequence() < pair[1].sequence()));

    let debug = format!(
        "{:?} {:?} {:?} {:?}",
        orchestrator,
        orchestrator.research_knowledge_events(),
        audit,
        result,
    );
    for sentinel in [
        OBJECTIVE,
        "Approach A fixture",
        "lower operating cost",
        "Approach A is simpler",
        "Retain this comparison",
        FINAL_ANSWER,
    ] {
        assert!(!debug.contains(sentinel));
    }

    let starts = recorder.starts();
    assert_eq!(starts.len(), 4);
    assert!(starts[1]
        .selected_text
        .contains("deterministic-fixture-only"));
    assert!(starts[2].selected_text.contains("approach-a"));
    assert!(starts[3].selected_text.contains("fixture-based"));
    assert!(starts[3].selected_text.contains("approach-b"));
    Ok(())
}

#[test]
fn research_failure_skips_knowledge_and_allows_truthful_partial_synthesis(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    let research_id = research.task_id().clone();
    orchestrator.accept_runtime_event(&research_id, started(&research, "research-failure")?)?;
    orchestrator.accept_runtime_event(&research_id, failed(&research)?)?;

    assert_eq!(orchestrator.task_count(), 2);
    let synthesis = orchestrator.current_context(root.task_id())?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "partial-synthesis",
        PARTIAL_SYNTHESIS_NO_SOURCES,
    )?;
    let result = orchestrator
        .research_knowledge_result()
        .ok_or("missing partial result")?;
    assert!(matches!(result.research(), ResearchStageOutcome::Failed(_)));
    assert!(matches!(
        result.knowledge(),
        KnowledgeStageOutcome::SkippedResearchUnavailable
    ));
    assert!(orchestrator
        .research_knowledge_events()
        .iter()
        .any(|event| {
            matches!(
                event,
                ResearchKnowledgeWorkflowEvent::PartialFailure {
                    stage: ResearchKnowledgeStage::Research,
                    code: ResearchKnowledgePartialFailureCode::RuntimeFailed,
                }
            )
        }));
    assert!(!orchestrator
        .research_knowledge_events()
        .iter()
        .any(|event| {
            matches!(
                event,
                ResearchKnowledgeWorkflowEvent::KnowledgeOrganizationStarted { .. }
            )
        }));
    Ok(())
}

#[test]
fn knowledge_failure_retains_validated_research_for_partial_synthesis() -> Result<(), Box<dyn Error>>
{
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    complete_stage(&mut orchestrator, &research, "research-ok", RESEARCH_JSON)?;
    let knowledge = orchestrator
        .active_child_task()
        .and_then(|task| orchestrator.current_context(task.id()).ok())
        .ok_or("missing Knowledge context")?;
    let knowledge_id = knowledge.task_id().clone();
    orchestrator.accept_runtime_event(&knowledge_id, started(&knowledge, "knowledge-failure")?)?;
    orchestrator.accept_runtime_event(&knowledge_id, failed(&knowledge)?)?;

    let synthesis = orchestrator.current_context(root.task_id())?;
    let starts = recorder.starts();
    assert_eq!(starts.len(), 4);
    assert!(starts[3].selected_text.contains("Approach A is simpler"));
    assert!(starts[3].selected_text.contains("status\":\"failed"));
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "partial-knowledge-synthesis",
        PARTIAL_SYNTHESIS_WITH_SOURCES,
    )?;
    let result = orchestrator
        .research_knowledge_result()
        .ok_or("missing partial result")?;
    assert!(matches!(
        result.research(),
        ResearchStageOutcome::Completed(_)
    ));
    assert!(matches!(
        result.knowledge(),
        KnowledgeStageOutcome::Failed(_)
    ));
    Ok(())
}

#[test]
fn incomplete_knowledge_is_retained_as_partial_for_final_synthesis() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    complete_stage(&mut orchestrator, &research, "research-ok", RESEARCH_JSON)?;
    let knowledge = orchestrator
        .active_child_task()
        .and_then(|task| orchestrator.current_context(task.id()).ok())
        .ok_or("missing Knowledge context")?;
    let incomplete = r#"{"sections":[{"heading":"Partial organization","body":"A is simpler.","source_ids":["approach-a"]}],"extracted_facts":[],"contradictions":[],"summary":"Only part of the comparison was organized.","reusable_knowledge_proposal":null,"artifact_outline":null,"incomplete":true}"#;
    complete_stage(
        &mut orchestrator,
        &knowledge,
        "knowledge-incomplete",
        incomplete,
    )?;
    let synthesis = orchestrator.current_context(root.task_id())?;
    assert!(recorder.starts()[3]
        .selected_text
        .contains("\"quality\":\"partial\""));
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "incomplete-synthesis",
        PARTIAL_SYNTHESIS_WITH_SOURCES,
    )?;
    assert!(matches!(
        orchestrator
            .research_knowledge_result()
            .ok_or("missing workflow result")?
            .knowledge(),
        KnowledgeStageOutcome::Completed(knowledge)
            if knowledge.quality() == KnowledgeResultQuality::Partial
    ));
    assert!(orchestrator
        .research_knowledge_events()
        .iter()
        .any(|event| {
            matches!(
                event,
                ResearchKnowledgeWorkflowEvent::PartialFailure {
                    stage: ResearchKnowledgeStage::KnowledgeOrganization,
                    ..
                }
            )
        }));
    Ok(())
}

#[test]
fn noncanonical_outer_whitespace_terminalizes_each_stage_and_starts_fallback(
) -> Result<(), Box<dyn Error>> {
    let mut research_case = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, research) = start_workflow(&mut research_case)?;
    complete_stage(
        &mut research_case,
        &research,
        "research-whitespace",
        &format!("{RESEARCH_JSON}\n"),
    )?;
    assert_eq!(
        research_case
            .task(research.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(research_case.current_context(root.task_id()).is_ok());
    assert!(research_case.active_child_task().is_none());

    let mut knowledge_case = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, research) = start_workflow(&mut knowledge_case)?;
    complete_stage(&mut knowledge_case, &research, "research-ok", RESEARCH_JSON)?;
    let knowledge = knowledge_case
        .active_child_task()
        .and_then(|task| knowledge_case.current_context(task.id()).ok())
        .ok_or("missing Knowledge context")?;
    complete_stage(
        &mut knowledge_case,
        &knowledge,
        "knowledge-whitespace",
        &format!(" {KNOWLEDGE_JSON}"),
    )?;
    assert_eq!(
        knowledge_case
            .task(knowledge.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(knowledge_case.current_context(root.task_id()).is_ok());
    assert!(knowledge_case.active_child_task().is_none());
    Ok(())
}

#[test]
fn final_synthesis_rejects_untrusted_provenance_and_claim_surfaces() -> Result<(), Box<dyn Error>> {
    let invalid_outputs = [
        r#"{"version":"v1","answer":"Fixture brief with an invented source.","source_ids":["approach-a","invented-source"],"fixture_based":true,"status":"complete"}"#,
        r#"{"version":"v1","answer":"Fixture brief missing preserved provenance.","source_ids":["approach-a"],"fixture_based":true,"status":"complete"}"#,
        r#"{"version":"v1","answer":"Fixture brief with false disclosure.","source_ids":["approach-a","approach-b"],"fixture_based":false,"status":"complete"}"#,
        r#"{"version":"v1","answer":"Fixture brief with hidden rationale.","source_ids":["approach-a","approach-b"],"fixture_based":true,"status":"complete","reasoning":"private chain"}"#,
        r#"{"version":"v1","answer":"Fixture live research confirms this decision.","source_ids":["approach-a","approach-b"],"fixture_based":true,"status":"complete"}"#,
        r#"{"version":"v1","answer":"Fixture evidence is at https://example.invalid.","source_ids":["approach-a","approach-b"],"fixture_based":true,"status":"complete"}"#,
    ];

    for (index, invalid_output) in invalid_outputs.into_iter().enumerate() {
        let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
        let (root, synthesis) = advance_to_synthesis(&mut orchestrator)?;
        complete_stage(
            &mut orchestrator,
            &synthesis,
            &format!("invalid-synthesis-{index}"),
            invalid_output,
        )?;
        assert_eq!(
            orchestrator.task(root.task_id()).map(|task| task.status()),
            Some(AgentTaskStatus::Failed)
        );
        assert!(orchestrator.research_knowledge_result().is_none());
        assert!(matches!(
            orchestrator.research_knowledge_events().last(),
            Some(ResearchKnowledgeWorkflowEvent::PartialFailure {
                stage: ResearchKnowledgeStage::Synthesis,
                code: ResearchKnowledgePartialFailureCode::InvalidStructuredOutput,
            })
        ));
    }
    Ok(())
}

#[test]
fn cancellation_before_selection_and_during_research_remains_terminal() -> Result<(), Box<dyn Error>>
{
    let mut initial = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = initial.start_root(OBJECTIVE)?;
    assert_eq!(
        initial.cancel_task(root.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert!(matches!(
        initial.request_research_knowledge_workflow(&root, workflow_request()?),
        Err(AgentOrchestratorError::NoActiveRun)
    ));
    assert!(initial.research_knowledge_events().is_empty());

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut research_running = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut research_running)?;
    assert_eq!(
        research_running.cancel_task(root.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        research_running
            .task(research.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        research_running
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(matches!(
        research_running.research_knowledge_events(),
        [
            ResearchKnowledgeWorkflowEvent::ResearchStarted { .. },
            ResearchKnowledgeWorkflowEvent::Cancelled {
                stage: ResearchKnowledgeStage::Research,
            },
        ]
    ));
    assert_eq!(recorder.starts().len(), 2);
    assert!(research_running.research_knowledge_result().is_none());
    Ok(())
}

#[test]
fn independent_research_cancellation_allows_only_partial_personal_synthesis(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    assert_eq!(
        orchestrator.cancel_task(research.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert!(orchestrator.active_child_task().is_none());
    assert!(orchestrator
        .research_knowledge_events()
        .iter()
        .any(|event| {
            matches!(
                event,
                ResearchKnowledgeWorkflowEvent::PartialFailure {
                    stage: ResearchKnowledgeStage::Research,
                    code: ResearchKnowledgePartialFailureCode::Cancelled,
                }
            )
        }));
    let synthesis = orchestrator.current_context(root.task_id())?;
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "cancelled-research-synthesis",
        PARTIAL_SYNTHESIS_NO_SOURCES,
    )?;
    let result = orchestrator
        .research_knowledge_result()
        .ok_or("missing partial cancellation result")?;
    assert!(matches!(result.research(), ResearchStageOutcome::Cancelled));
    assert!(matches!(
        result.knowledge(),
        KnowledgeStageOutcome::SkippedResearchUnavailable
    ));
    assert_eq!(result.synthesis().status(), FinalSynthesisStatus::Partial);
    Ok(())
}

#[test]
fn cancellation_and_runtime_failure_during_synthesis_never_fabricate_completion(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut cancelled = AgentOrchestrator::new(runtime)?;
    let (root, synthesis) = advance_to_synthesis(&mut cancelled)?;
    assert_eq!(
        cancelled.cancel_task(root.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        cancelled.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(matches!(
        cancelled.research_knowledge_events().last(),
        Some(ResearchKnowledgeWorkflowEvent::Cancelled {
            stage: ResearchKnowledgeStage::Synthesis,
        })
    ));
    assert!(cancelled.research_knowledge_result().is_none());
    assert!(cancelled.current_context(synthesis.task_id()).is_err());
    assert_eq!(recorder.starts().len(), 4);

    let mut failed_synthesis = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let (root, synthesis) = advance_to_synthesis(&mut failed_synthesis)?;
    failed_synthesis.accept_runtime_event(
        root.task_id(),
        started(&synthesis, "failed-final-synthesis")?,
    )?;
    assert_eq!(
        failed_synthesis.accept_runtime_event(root.task_id(), failed(&synthesis)?)?,
        RuntimeEventAcceptance::ResponseFailed {
            failure: RuntimeFailure::new(RuntimeFailureCode::ProviderUnavailable, true, None,)?,
        }
    );
    assert_eq!(
        failed_synthesis
            .task(root.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(failed_synthesis.research_knowledge_result().is_none());
    assert!(matches!(
        failed_synthesis.research_knowledge_events().last(),
        Some(ResearchKnowledgeWorkflowEvent::PartialFailure {
            stage: ResearchKnowledgeStage::Synthesis,
            code: ResearchKnowledgePartialFailureCode::RuntimeFailed,
        })
    ));
    Ok(())
}

#[test]
fn terminal_preparation_failure_leaves_the_live_run_and_workflow_unchanged(
) -> Result<(), Box<dyn Error>> {
    let sources = (0..8)
        .map(|index| {
            WorkflowSource::deterministic_fixture(
                format!("bounded-{index}"),
                "😀".repeat(128),
                "bounded deterministic fixture evidence",
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let findings = (0..8)
        .map(|index| {
            format!(
                r#"{{"statement":"bounded finding {index}","source_ids":["bounded-{index}"],"confidence":"high"}}"#
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let research_output = format!(
        r#"{{"findings":[{findings}],"unresolved_questions":[],"limitations":[],"recommended_follow_up":null}}"#
    );
    let mut orchestrator = AgentOrchestrator::new(MockAgentRuntime::new(MockMode::Success))?;
    let root = orchestrator.start_root("Prepare a bounded decision brief")?;
    let research = match orchestrator.request_research_knowledge_workflow(
        &root,
        ResearchKnowledgeWorkflowRequest::new(
            "Prepare a bounded decision brief",
            WorkflowSourceCatalog::new(sources)?,
        )?,
    )? {
        ResearchKnowledgeWorkflowAcceptance::ResearchStarted { context } => context,
        ResearchKnowledgeWorkflowAcceptance::PersonalFallbackStarted { .. } => {
            return Err("Research unexpectedly failed to start".into());
        }
    };
    let research_id = research.task_id().clone();
    orchestrator.accept_runtime_event(&research_id, started(&research, "bounded-preflight")?)?;
    orchestrator.accept_runtime_event(&research_id, delta(&research, &research_output)?)?;
    let workflow_events = orchestrator.research_knowledge_events().to_vec();
    let audit = orchestrator.research_knowledge_audit_records().to_vec();
    let generic_events = orchestrator.events().to_vec();
    let event_count = orchestrator.runtime_event_count();

    assert!(matches!(
        orchestrator.accept_runtime_event(&research_id, completed(&research)),
        Err(AgentOrchestratorError::ResearchKnowledge(
            ai_agent_assistant_lib::agent::research_knowledge::ResearchKnowledgeError::SynthesisInputTooLarge
        ))
    ));
    assert_eq!(orchestrator.runtime_event_count(), event_count);
    assert_eq!(orchestrator.research_knowledge_events(), workflow_events);
    assert_eq!(orchestrator.research_knowledge_audit_records(), audit);
    assert_eq!(orchestrator.events(), generic_events);
    assert_eq!(
        orchestrator.task(&research_id).map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(orchestrator.current_context(&research_id).is_ok());
    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    Ok(())
}

#[test]
fn missing_references_are_labeled_partial_and_never_invented() -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    let partial = r#"{"findings":[{"statement":"An unattributed fixture observation","source_ids":[],"confidence":"unknown"}],"unresolved_questions":[],"limitations":["No source reference was supplied"],"recommended_follow_up":null}"#;
    complete_stage(&mut orchestrator, &research, "research-partial", partial)?;
    assert_eq!(orchestrator.task_count(), 2);
    let synthesis = orchestrator.current_context(root.task_id())?;
    let starts = recorder.starts();
    assert_eq!(starts.len(), 3);
    assert!(starts[2].selected_text.contains("partial-missing-sources"));
    assert!(!starts[2].selected_text.contains("invented-source"));
    complete_stage(
        &mut orchestrator,
        &synthesis,
        "missing-source-synthesis",
        PARTIAL_SYNTHESIS_NO_SOURCES,
    )?;
    let result = orchestrator
        .research_knowledge_result()
        .ok_or("missing partial result")?;
    assert!(matches!(
        result.research(),
        ResearchStageOutcome::Completed(research)
            if research.quality() == ResearchResultQuality::PartialMissingSources
    ));
    assert!(matches!(
        result.knowledge(),
        KnowledgeStageOutcome::SkippedResearchIncomplete
    ));
    Ok(())
}

#[test]
fn root_cancellation_during_knowledge_is_terminal_and_starts_no_synthesis(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    complete_stage(&mut orchestrator, &research, "research-ok", RESEARCH_JSON)?;
    let knowledge_id = orchestrator
        .active_child_task()
        .ok_or("missing Knowledge child")?
        .id()
        .clone();
    let knowledge_context = orchestrator.current_context(&knowledge_id)?;

    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    assert_eq!(
        orchestrator.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert_eq!(
        orchestrator.task(&knowledge_id).map(|task| task.status()),
        Some(AgentTaskStatus::Cancelled)
    );
    assert!(orchestrator.active_child_task().is_none());
    assert!(orchestrator.research_knowledge_result().is_none());
    assert!(matches!(
        orchestrator.research_knowledge_events().last(),
        Some(ResearchKnowledgeWorkflowEvent::Cancelled {
            stage: ResearchKnowledgeStage::KnowledgeOrganization,
        })
    ));
    assert_eq!(recorder.starts().len(), 3);
    assert_eq!(recorder.cancellations().len(), 2);
    assert_eq!(
        orchestrator.accept_runtime_event(
            &knowledge_id,
            RuntimeEventEnvelope::for_identity(
                knowledge_context.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: RuntimeResponseId::new("late-knowledge")?,
                },
            ),
        ),
        Err(AgentOrchestratorError::NoActiveRun)
    );
    Ok(())
}

#[test]
fn cancellation_failure_preserves_live_knowledge_state_without_advancing(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::CancelFailureAt(3));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    complete_stage(&mut orchestrator, &research, "research-ok", RESEARCH_JSON)?;
    let knowledge = orchestrator
        .active_child_task()
        .and_then(|task| orchestrator.current_context(task.id()).ok())
        .ok_or("missing Knowledge context")?;
    let events_before = orchestrator.research_knowledge_events().to_vec();
    let generic_before = orchestrator.events().to_vec();
    assert!(orchestrator.cancel_task(root.task_id()).is_err());
    assert_eq!(
        orchestrator.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::WaitingForChild)
    );
    assert_eq!(
        orchestrator
            .task(knowledge.task_id())
            .map(|task| task.status()),
        Some(AgentTaskStatus::Running)
    );
    assert!(orchestrator.current_context(knowledge.task_id()).is_ok());
    assert_eq!(orchestrator.research_knowledge_events(), events_before);
    assert_eq!(orchestrator.events(), generic_before);
    assert_eq!(recorder.starts().len(), 3);
    assert_eq!(recorder.cancellations().len(), 1);
    Ok(())
}

#[test]
fn specialist_start_failures_consume_one_attempt_and_fall_back_without_retry(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(2));
    let mut research_failure = AgentOrchestrator::new(runtime)?;
    let root = research_failure.start_root(OBJECTIVE)?;
    let acceptance =
        research_failure.request_research_knowledge_workflow(&root, workflow_request()?)?;
    assert!(matches!(
        acceptance,
        ResearchKnowledgeWorkflowAcceptance::PersonalFallbackStarted { .. }
    ));
    assert_eq!(recorder.starts().len(), 3);
    assert_eq!(research_failure.task_count(), 2);
    assert_eq!(research_failure.run_count(), 3);
    assert!(matches!(
        research_failure.research_knowledge_events(),
        [
            ResearchKnowledgeWorkflowEvent::ResearchStarted { .. },
            ResearchKnowledgeWorkflowEvent::PartialFailure {
                stage: ResearchKnowledgeStage::Research,
                code: ResearchKnowledgePartialFailureCode::RuntimeStartFailed,
            },
            ResearchKnowledgeWorkflowEvent::SynthesisStarted { .. },
        ]
    ));

    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(3));
    let mut knowledge_failure = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut knowledge_failure)?;
    let research_id = research.task_id().clone();
    knowledge_failure.accept_runtime_event(
        &research_id,
        started(&research, "research-before-start-failure")?,
    )?;
    knowledge_failure.accept_runtime_event(&research_id, delta(&research, RESEARCH_JSON)?)?;
    assert_eq!(
        knowledge_failure.accept_runtime_event(&research_id, completed(&research))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(
        knowledge_failure.research_knowledge_continuation_failure(),
        Some(ResearchKnowledgeContinuationFailure::KnowledgeRuntimeStartFailed)
    );
    assert_eq!(recorder.starts().len(), 4);
    assert_eq!(knowledge_failure.task_count(), 3);
    assert_eq!(knowledge_failure.run_count(), 4);
    assert!(knowledge_failure.current_context(root.task_id()).is_ok());
    assert!(knowledge_failure
        .research_knowledge_events()
        .iter()
        .any(|event| {
            matches!(
                event,
                ResearchKnowledgeWorkflowEvent::PartialFailure {
                    stage: ResearchKnowledgeStage::KnowledgeOrganization,
                    code: ResearchKnowledgePartialFailureCode::RuntimeStartFailed,
                }
            )
        }));
    Ok(())
}

#[test]
fn synthesis_start_failure_terminalizes_root_without_fabricating_result(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailureAt(4));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    complete_stage(&mut orchestrator, &research, "research-ok", RESEARCH_JSON)?;
    let knowledge = orchestrator
        .active_child_task()
        .and_then(|task| orchestrator.current_context(task.id()).ok())
        .ok_or("missing Knowledge context")?;
    let knowledge_id = knowledge.task_id().clone();
    orchestrator.accept_runtime_event(&knowledge_id, started(&knowledge, "knowledge-ok")?)?;
    orchestrator.accept_runtime_event(&knowledge_id, delta(&knowledge, KNOWLEDGE_JSON)?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(&knowledge_id, completed(&knowledge))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(
        orchestrator.research_knowledge_continuation_failure(),
        Some(ResearchKnowledgeContinuationFailure::SynthesisRuntimeStartFailed)
    );
    assert_eq!(recorder.starts().len(), 4);
    assert_eq!(
        orchestrator.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(orchestrator.research_knowledge_result().is_none());
    assert!(orchestrator.current_context(root.task_id()).is_err());
    assert!(orchestrator
        .research_knowledge_events()
        .iter()
        .any(|event| {
            matches!(
                event,
                ResearchKnowledgeWorkflowEvent::PartialFailure {
                    stage: ResearchKnowledgeStage::Synthesis,
                    code: ResearchKnowledgePartialFailureCode::RuntimeStartFailed,
                }
            )
        }));
    Ok(())
}

#[test]
fn knowledge_and_fallback_synthesis_start_failures_close_the_accepted_terminal_event(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::StartFailuresAt(3, 4));
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    let research_id = research.task_id().clone();
    orchestrator.accept_runtime_event(
        &research_id,
        started(&research, "research-before-combined-start-failure")?,
    )?;
    orchestrator.accept_runtime_event(&research_id, delta(&research, RESEARCH_JSON)?)?;
    assert_eq!(
        orchestrator.accept_runtime_event(&research_id, completed(&research))?,
        RuntimeEventAcceptance::ResponseCompleted
    );
    assert_eq!(
        orchestrator.research_knowledge_continuation_failure(),
        Some(ResearchKnowledgeContinuationFailure::KnowledgeAndSynthesisRuntimeStartFailed)
    );
    assert_eq!(recorder.starts().len(), 4);
    assert_eq!(
        orchestrator.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(orchestrator.active_child_task().is_none());
    assert!(orchestrator.current_context(root.task_id()).is_err());
    assert!(orchestrator.current_context(&research_id).is_err());
    assert!(orchestrator.research_knowledge_result().is_none());
    assert!(matches!(
        orchestrator.accept_runtime_event(&research_id, completed(&research)),
        Err(AgentOrchestratorError::NoActiveRun)
    ));
    Ok(())
}

#[test]
fn runtime_event_hard_cap_terminalizes_the_workflow_without_starting_fallback(
) -> Result<(), Box<dyn Error>> {
    let (runtime, recorder) = MockAgentRuntime::recording(MockMode::Success);
    let mut orchestrator = AgentOrchestrator::new(runtime)?;
    let (root, research) = start_workflow(&mut orchestrator)?;
    let research_id = research.task_id().clone();
    orchestrator.accept_runtime_event(&research_id, started(&research, "bounded-response")?)?;
    for sequence in 1..u32::try_from(MAX_RUNTIME_EVENTS_PER_ROOT)? {
        orchestrator.accept_runtime_event(
            &research_id,
            RuntimeEventEnvelope::for_identity(
                research.runtime_run_identity(),
                sequence,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: RuntimeOutputText::new("x")?,
                },
            ),
        )?;
    }
    assert_eq!(
        orchestrator.runtime_event_count(),
        MAX_RUNTIME_EVENTS_PER_ROOT
    );
    assert_eq!(
        orchestrator.accept_runtime_event(
            &research_id,
            RuntimeEventEnvelope::for_identity(
                research.runtime_run_identity(),
                u32::try_from(MAX_RUNTIME_EVENTS_PER_ROOT)?,
                UntrustedRuntimeEvent::ResponseCompleted,
            ),
        ),
        Err(AgentOrchestratorError::RuntimeEventLimitExceeded)
    );
    assert_eq!(
        orchestrator.task(&research_id).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert_eq!(
        orchestrator.task(root.task_id()).map(|task| task.status()),
        Some(AgentTaskStatus::Failed)
    );
    assert!(orchestrator.active_child_task().is_none());
    assert!(orchestrator.current_context(root.task_id()).is_err());
    assert_eq!(recorder.starts().len(), 2);
    assert!(orchestrator
        .research_knowledge_events()
        .iter()
        .any(|event| {
            matches!(
                event,
                ResearchKnowledgeWorkflowEvent::PartialFailure {
                    stage: ResearchKnowledgeStage::Research,
                    code: ResearchKnowledgePartialFailureCode::RuntimeFailed,
                }
            )
        }));
    Ok(())
}

#[test]
fn native_runtime_remains_the_only_default_without_provider_or_network(
) -> Result<(), Box<dyn Error>> {
    let mut orchestrator = AgentOrchestrator::native()?;
    let root = orchestrator.start_root(OBJECTIVE)?;
    let accepted = orchestrator.request_research_knowledge_workflow(&root, workflow_request()?)?;
    assert_eq!(accepted.context().runtime_id(), RuntimeId::Native);
    assert_eq!(accepted.context().agent_id(), AgentId::Research);
    assert_eq!(
        orchestrator.cancel_task(root.task_id())?,
        ai_agent_assistant_lib::agent::task::AgentTaskCancellationOutcome::Cancelled
    );
    Ok(())
}
