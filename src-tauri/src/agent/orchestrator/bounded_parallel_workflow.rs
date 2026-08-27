//! Private lifecycle for the sealed D-091 same-thread multiplexed selector.

use super::*;

use std::collections::BTreeMap;

use crate::agent::bounded_parallelism::{
    deadline_after, BoundedParallelAttribution, BoundedParallelAuditOutcome,
    BoundedParallelAuditRecord, BoundedParallelChildControl, BoundedParallelFailurePolicy,
    BoundedParallelScenarioId, BoundedParallelTerminalStatus, BoundedParallelWorkflowAcceptance,
    BoundedParallelWorkflowEvent, BoundedParallelWorkflowRequest, BoundedParallelWorkflowResult,
    BoundedParallelWorkflowStatus, ParallelCancellationReason, ParallelChildCancellationHandle,
    ParallelChildDisposition, ParallelChildOutcome, ParallelChildResultStatus, ParallelSkipReason,
    ParallelTimeoutReason, ParallelWorkItem, HARD_MAX_ACTIVE_PARALLEL_CHILDREN,
    MAX_PARALLEL_AUDIT_RECORDS, MAX_PARALLEL_CHILDREN, MAX_PARALLEL_CHILD_DURATION_SECONDS,
    MAX_PARALLEL_EVENTS_PER_RUN, MAX_PARALLEL_ORCHESTRATION_EVENTS, MAX_PARALLEL_RAW_RESULT_BYTES,
    MAX_PARALLEL_RAW_RESULT_CHARACTERS, MAX_PARALLEL_ROOT_DURATION_SECONDS,
    MAX_PARALLEL_RUNTIME_EVENTS, MAX_PARALLEL_RUNTIME_RUNS_PER_ROOT,
    MAX_PARALLEL_SYNTHESIS_OUTPUT_BYTES, MAX_PARALLEL_SYNTHESIS_OUTPUT_CHARACTERS,
    MAX_PARALLEL_TASKS_PER_ROOT, MAX_PARALLEL_WORKFLOW_EVENTS,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParallelSlotState {
    Pending,
    Running {
        task_id: AgentTaskId,
        context: AgentExecutionContext,
        deadline: std::time::Instant,
        progress_recorded: bool,
    },
    Terminal(ParallelChildOutcome),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::bounded_parallelism::{
        BoundedParallelScenarioCatalog, BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE,
    };
    use crate::agent::definition::{AgentActivationGate, AgentDefinition};
    use crate::agent::native_runtime::NativeAgentRun;
    use crate::agent::runtime::{
        RuntimeBoundaryStage, RuntimeDescriptor, RuntimeFailure, RuntimeOutputText,
        RuntimeResponseId, RuntimeResult, RuntimeRunId, RuntimeRunIdentity,
    };
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    struct TestClock {
        now: Arc<Mutex<std::time::Instant>>,
    }

    impl TestClock {
        fn new() -> (Self, workflow_automation_dispatch::WorkflowClock) {
            let now = Arc::new(Mutex::new(std::time::Instant::now()));
            let shared = Arc::clone(&now);
            let clock = Arc::new(move || {
                *shared
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            });
            (Self { now }, clock)
        }

        fn advance(&self, duration: Duration) {
            let mut now = self
                .now
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *now += duration;
        }
    }

    fn research_result(work_item_id: &str, fixture_id: &str) -> String {
        format!(
            r#"{{"version":"v1","scenario_id":"research-knowledge-independent-v1","work_item_id":"{work_item_id}","summary":"Fixture evidence supports this bounded finding.","findings":[{{"id":"bounded-finding","statement":"The immutable fixture supports the bounded assessment.","confidence":"high","references":[{{"namespace":"fixture","id":"{fixture_id}"}}]}}],"unresolved_issues":[],"fixture_based":true,"live_access_performed":false,"tools_executed":false,"credentials_loaded":false,"effects_performed":false}}"#
        )
    }

    fn timed_out_research_synthesis() -> String {
        format!(
            r#"{{"version":"v1","scenario_id":"research-knowledge-independent-v1","status":"partial","source_results":[{{"ordinal":1,"work_item_id":"research-analysis","agent_id":"research","status":"timed-out"}},{{"ordinal":2,"work_item_id":"knowledge-analysis","agent_id":"knowledge-document","status":"timed-out"}}],"source_findings":[{{"ordinal":1,"work_item_id":"research-analysis","agent_id":"research","finding_ids":[]}},{{"ordinal":2,"work_item_id":"knowledge-analysis","agent_id":"knowledge-document","finding_ids":[]}}],"summary":"Partial. {BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE}","unresolved_issues":["research-analysis is timed-out","knowledge-analysis is timed-out"],"fixture_based":true,"live_access_performed":false,"tools_executed":false,"credentials_loaded":false,"effects_performed":false}}"#
        )
    }

    fn complete_child(
        orchestrator: &mut AgentOrchestrator<NativeAgentRuntime>,
        context: &AgentExecutionContext,
        response_id: &str,
        output: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task_id = context.task_id().clone();
        orchestrator.accept_runtime_event(
            &task_id,
            RuntimeEventEnvelope::for_identity(
                context.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: RuntimeResponseId::new(response_id)?,
                },
            ),
        )?;
        orchestrator.accept_runtime_event(
            &task_id,
            RuntimeEventEnvelope::for_identity(
                context.runtime_run_identity(),
                1,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: RuntimeOutputText::new(output)?,
                },
            ),
        )?;
        orchestrator.accept_runtime_event(
            &task_id,
            RuntimeEventEnvelope::for_identity(
                context.runtime_run_identity(),
                2,
                UntrustedRuntimeEvent::ResponseCompleted,
            ),
        )?;
        Ok(())
    }

    #[derive(Clone)]
    struct CancelFailOnceRuntime {
        next_start: Arc<AtomicU8>,
        failure_used: Arc<AtomicBool>,
        fail_start_ordinal: u8,
    }

    impl CancelFailOnceRuntime {
        fn new(fail_start_ordinal: u8) -> Self {
            Self {
                next_start: Arc::new(AtomicU8::new(0)),
                failure_used: Arc::new(AtomicBool::new(false)),
                fail_start_ordinal,
            }
        }
    }

    impl AgentRuntime for CancelFailOnceRuntime {
        type Run = CancelFailOnceRun;

        fn describe(&self) -> RuntimeDescriptor {
            NativeAgentRuntime.describe()
        }

        fn start(&self, request: RuntimeTurnRequest) -> RuntimeResult<Self::Run> {
            let start_ordinal = self.next_start.fetch_add(1, Ordering::SeqCst) + 1;
            Ok(CancelFailOnceRun {
                inner: NativeAgentRuntime.start(request)?,
                start_ordinal,
                fail_start_ordinal: self.fail_start_ordinal,
                failure_used: Arc::clone(&self.failure_used),
            })
        }
    }

    struct CancelFailOnceRun {
        inner: NativeAgentRun,
        start_ordinal: u8,
        fail_start_ordinal: u8,
        failure_used: Arc<AtomicBool>,
    }

    #[derive(Clone)]
    struct StartCrossesDeadlineRuntime {
        next_start: Arc<AtomicU8>,
        clock: Arc<Mutex<std::time::Instant>>,
        advance_at: u8,
        advance_by: Duration,
        contradictory_cancel_status: Option<RuntimeRunStatus>,
    }

    impl StartCrossesDeadlineRuntime {
        fn new(
            clock: Arc<Mutex<std::time::Instant>>,
            advance_at: u8,
            advance_by: Duration,
        ) -> Self {
            Self {
                next_start: Arc::new(AtomicU8::new(0)),
                clock,
                advance_at,
                advance_by,
                contradictory_cancel_status: Some(RuntimeRunStatus::Completed),
            }
        }

        fn with_normal_cancel(
            clock: Arc<Mutex<std::time::Instant>>,
            advance_at: u8,
            advance_by: Duration,
        ) -> Self {
            Self {
                next_start: Arc::new(AtomicU8::new(0)),
                clock,
                advance_at,
                advance_by,
                contradictory_cancel_status: None,
            }
        }

        fn with_nonterminal_cancel(
            clock: Arc<Mutex<std::time::Instant>>,
            advance_at: u8,
            advance_by: Duration,
        ) -> Self {
            Self {
                next_start: Arc::new(AtomicU8::new(0)),
                clock,
                advance_at,
                advance_by,
                contradictory_cancel_status: Some(RuntimeRunStatus::Streaming),
            }
        }
    }

    impl AgentRuntime for StartCrossesDeadlineRuntime {
        type Run = StartCrossesDeadlineRun;

        fn describe(&self) -> RuntimeDescriptor {
            NativeAgentRuntime.describe()
        }

        fn start(&self, request: RuntimeTurnRequest) -> RuntimeResult<Self::Run> {
            let start_ordinal = self.next_start.fetch_add(1, Ordering::SeqCst) + 1;
            let inner = NativeAgentRuntime.start(request)?;
            if start_ordinal == self.advance_at {
                let mut now = self
                    .clock
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                *now += self.advance_by;
            }
            Ok(StartCrossesDeadlineRun {
                inner,
                contradictory_cancel_status: (start_ordinal == self.advance_at)
                    .then_some(self.contradictory_cancel_status)
                    .flatten(),
            })
        }
    }

    struct StartCrossesDeadlineRun {
        inner: NativeAgentRun,
        contradictory_cancel_status: Option<RuntimeRunStatus>,
    }

    #[derive(Clone)]
    struct AcceptCrossesDeadlineRuntime {
        next_start: Arc<AtomicU8>,
        clock: Arc<Mutex<std::time::Instant>>,
        advance_at: u8,
        advance_by: Duration,
    }

    impl AgentRuntime for AcceptCrossesDeadlineRuntime {
        type Run = AcceptCrossesDeadlineRun;

        fn describe(&self) -> RuntimeDescriptor {
            NativeAgentRuntime.describe()
        }

        fn start(&self, request: RuntimeTurnRequest) -> RuntimeResult<Self::Run> {
            let start_ordinal = self.next_start.fetch_add(1, Ordering::SeqCst) + 1;
            Ok(AcceptCrossesDeadlineRun {
                inner: NativeAgentRuntime.start(request)?,
                clock: Arc::clone(&self.clock),
                advance_by: (start_ordinal == self.advance_at).then_some(self.advance_by),
            })
        }
    }

    struct AcceptCrossesDeadlineRun {
        inner: NativeAgentRun,
        clock: Arc<Mutex<std::time::Instant>>,
        advance_by: Option<Duration>,
    }

    impl RuntimeRun for StartCrossesDeadlineRun {
        fn run_id(&self) -> &RuntimeRunId {
            self.inner.run_id()
        }

        fn identity(&self) -> &RuntimeRunIdentity {
            self.inner.identity()
        }

        fn status(&self) -> RuntimeRunStatus {
            self.inner.status()
        }

        fn accept_event(
            &mut self,
            event: RuntimeEventEnvelope,
        ) -> RuntimeResult<RuntimeEventAcceptance> {
            self.inner.accept_event(event)
        }

        fn cancel(&mut self) -> RuntimeResult<RuntimeCancellationOutcome> {
            if let Some(status) = self.contradictory_cancel_status.take() {
                Ok(RuntimeCancellationOutcome::AlreadyTerminal(status))
            } else {
                self.inner.cancel()
            }
        }
    }

    impl RuntimeRun for AcceptCrossesDeadlineRun {
        fn run_id(&self) -> &RuntimeRunId {
            self.inner.run_id()
        }

        fn identity(&self) -> &RuntimeRunIdentity {
            self.inner.identity()
        }

        fn status(&self) -> RuntimeRunStatus {
            self.inner.status()
        }

        fn accept_event(
            &mut self,
            event: RuntimeEventEnvelope,
        ) -> RuntimeResult<RuntimeEventAcceptance> {
            let terminal = matches!(
                event.clone().into_parts().3,
                UntrustedRuntimeEvent::ResponseCompleted
                    | UntrustedRuntimeEvent::ResponseFailed { .. }
            );
            let accepted = self.inner.accept_event(event)?;
            if terminal {
                if let Some(advance_by) = self.advance_by.take() {
                    let mut now = self
                        .clock
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    *now += advance_by;
                }
            }
            Ok(accepted)
        }

        fn cancel(&mut self) -> RuntimeResult<RuntimeCancellationOutcome> {
            self.inner.cancel()
        }
    }

    impl RuntimeRun for CancelFailOnceRun {
        fn run_id(&self) -> &RuntimeRunId {
            self.inner.run_id()
        }

        fn identity(&self) -> &RuntimeRunIdentity {
            self.inner.identity()
        }

        fn status(&self) -> RuntimeRunStatus {
            self.inner.status()
        }

        fn accept_event(
            &mut self,
            event: RuntimeEventEnvelope,
        ) -> RuntimeResult<RuntimeEventAcceptance> {
            self.inner.accept_event(event)
        }

        fn cancel(&mut self) -> RuntimeResult<RuntimeCancellationOutcome> {
            if self.start_ordinal == self.fail_start_ordinal
                && !self.failure_used.swap(true, Ordering::SeqCst)
            {
                return Err(RuntimeError::BoundaryFailure(
                    RuntimeBoundaryStage::Cancellation,
                ));
            }
            self.inner.cancel()
        }
    }

    #[test]
    fn private_three_independent_fixture_reaches_but_never_exceeds_hard_cap(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .three_independent_for_test(HARD_MAX_ACTIVE_PARALLEL_CHILDREN)?
            .with_active_limit_for_test(HARD_MAX_ACTIVE_PARALLEL_CHILDREN)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        let root = orchestrator.start_root(request.objective())?;
        let accepted = orchestrator.start_bounded_parallel_workflow(&root, request)?;

        assert_eq!(accepted.active_contexts().len(), 3);
        assert_eq!(orchestrator.bounded_parallel_active_children()?.len(), 3);
        assert_eq!(orchestrator.runs.len(), 3);
        assert_eq!(orchestrator.tasks.len(), MAX_PARALLEL_TASKS_PER_ROOT);
        assert_eq!(orchestrator.run_count, 4);
        orchestrator.cancel_task(root.task_id())?;
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn active_limit_one_queues_then_releases_the_next_independent_slot(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?
            .with_active_limit_for_test(1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;

        assert_eq!(acceptance.active_contexts().len(), 1);
        assert_eq!(orchestrator.runs.len(), 1);
        let first = acceptance.active_contexts()[0].clone();
        complete_child(
            &mut orchestrator,
            &first,
            "active-limit-first",
            &research_result("research-analysis", "approach-a"),
        )?;

        let controls = orchestrator.bounded_parallel_active_children()?;
        assert_eq!(controls.len(), 1);
        assert_eq!(controls[0].ordinal(), 2);
        assert_eq!(orchestrator.runs.len(), 1);
        assert!(orchestrator.bounded_parallel_events().iter().any(|event| {
            matches!(
                event,
                BoundedParallelWorkflowEvent::Started { ordinal: 2, .. }
            )
        }));

        orchestrator.cancel_task(root.task_id())?;
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn malformed_admission_over_hard_cap_or_total_budget_has_zero_mutation(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for request in [
            BoundedParallelScenarioCatalog::built_in()
                .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?
                .with_unchecked_active_limit_for_test(HARD_MAX_ACTIVE_PARALLEL_CHILDREN + 1),
            BoundedParallelScenarioCatalog::built_in()
                .request(BoundedParallelScenarioId::CodeSecurityQaV1)?
                .with_duplicated_work_item_for_test(),
        ] {
            let mut orchestrator = AgentOrchestrator::native()?;
            let root = orchestrator.start_root(request.objective())?;
            let before = (
                orchestrator.tasks.len(),
                orchestrator.runs.len(),
                orchestrator.run_count,
                orchestrator.events.len(),
            );

            assert_eq!(
                orchestrator.start_bounded_parallel_workflow(&root, request),
                Err(AgentOrchestratorError::BoundedParallel(
                    BoundedParallelError::InvalidCatalog
                ))
            );
            assert_eq!(
                (
                    orchestrator.tasks.len(),
                    orchestrator.runs.len(),
                    orchestrator.run_count,
                    orchestrator.events.len(),
                ),
                before
            );
            assert!(orchestrator.bounded_parallel.is_none());
            assert!(orchestrator.selected_workflow.is_none());
            assert!(!orchestrator.child_created);
            orchestrator.cancel_task(root.task_id())?;
        }
        Ok(())
    }

    #[test]
    fn oversized_future_dependent_input_is_rejected_before_any_mutation(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::CodeSecurityQaV1)?
            .with_oversized_dependent_fixture_for_test();
        let mut orchestrator = AgentOrchestrator::native()?;
        let root = orchestrator.start_root(request.objective())?;
        let before = (
            orchestrator.tasks.len(),
            orchestrator.runs.len(),
            orchestrator.run_count,
            orchestrator.events.len(),
        );

        assert_eq!(
            orchestrator.start_bounded_parallel_workflow(&root, request),
            Err(AgentOrchestratorError::BoundedParallel(
                BoundedParallelError::BoundExceeded
            ))
        );
        assert_eq!(
            (
                orchestrator.tasks.len(),
                orchestrator.runs.len(),
                orchestrator.run_count,
                orchestrator.events.len(),
            ),
            before
        );
        assert!(orchestrator.bounded_parallel.is_none());
        assert!(orchestrator.selected_workflow.is_none());
        assert!(!orchestrator.child_created);
        orchestrator.cancel_task(root.task_id())?;
        Ok(())
    }

    #[test]
    fn cooperative_child_deadlines_project_timeout_and_start_no_extra_specialist(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let child_task_ids = acceptance
            .active_contexts()
            .iter()
            .map(|context| context.task_id().clone())
            .collect::<Vec<_>>();
        clock.advance(Duration::from_secs(MAX_PARALLEL_CHILD_DURATION_SECONDS - 1));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Running
        );
        assert_eq!(orchestrator.runs.len(), 2);

        clock.advance(Duration::from_secs(1));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Running
        );
        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Synthesizing
        );
        let state = orchestrator
            .bounded_parallel
            .as_ref()
            .ok_or("missing bounded-parallel state")?;
        let outcomes = state.outcomes().ok_or("missing terminal outcomes")?;
        assert!(outcomes.iter().all(|outcome| {
            outcome.status() == ParallelChildResultStatus::TimedOut
                && matches!(
                    outcome.disposition(),
                    ParallelChildDisposition::TimedOut(
                        ParallelTimeoutReason::ChildDeadlineExceeded
                    )
                )
        }));
        assert_eq!(orchestrator.run_count, 4);
        let synthesis = orchestrator.current_context(root.task_id())?;
        orchestrator.accept_runtime_event(
            root.task_id(),
            RuntimeEventEnvelope::for_identity(
                synthesis.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: RuntimeResponseId::new("timed-out-partial-synthesis")?,
                },
            ),
        )?;
        orchestrator.accept_runtime_event(
            root.task_id(),
            RuntimeEventEnvelope::for_identity(
                synthesis.runtime_run_identity(),
                1,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: RuntimeOutputText::new(timed_out_research_synthesis())?,
                },
            ),
        )?;
        orchestrator.accept_runtime_event(
            root.task_id(),
            RuntimeEventEnvelope::for_identity(
                synthesis.runtime_run_identity(),
                2,
                UntrustedRuntimeEvent::ResponseCompleted,
            ),
        )?;
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Partial)
        );
        assert_eq!(
            orchestrator
                .bounded_parallel_result()
                .ok_or("missing timed-out partial result")?
                .children()
                .iter()
                .map(ParallelChildOutcome::status)
                .collect::<Vec<_>>(),
            [
                ParallelChildResultStatus::TimedOut,
                ParallelChildResultStatus::TimedOut,
            ]
        );
        assert!(orchestrator.runs.is_empty());

        let settled_result = orchestrator.bounded_parallel_result().cloned();
        let settled_workflow_events = orchestrator.bounded_parallel_events().to_vec();
        let settled_audit = orchestrator.bounded_parallel_audit_records().to_vec();
        let settled_generic_events = orchestrator.events.clone();
        let settled_tasks = orchestrator
            .tasks
            .iter()
            .map(|(task_id, task)| (task_id.clone(), task.status()))
            .collect::<BTreeMap<_, _>>();
        clock.advance(Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS));

        assert!(orchestrator.bounded_parallel_active_children()?.is_empty());
        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Partial
        );
        for task_id in child_task_ids {
            assert_eq!(
                orchestrator.cancel_task(&task_id)?,
                AgentTaskCancellationOutcome::AlreadyTerminal(AgentTaskStatus::Failed)
            );
        }
        assert_eq!(
            orchestrator.bounded_parallel_result(),
            settled_result.as_ref()
        );
        assert_eq!(
            orchestrator.bounded_parallel_events(),
            settled_workflow_events
        );
        assert_eq!(orchestrator.bounded_parallel_audit_records(), settled_audit);
        assert_eq!(orchestrator.events, settled_generic_events);
        assert_eq!(
            orchestrator
                .tasks
                .iter()
                .map(|(task_id, task)| (task_id.clone(), task.status()))
                .collect::<BTreeMap<_, _>>(),
            settled_tasks
        );
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn task_memory_ingress_is_live_at_n_minus_one_and_fails_closed_at_child_n(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        let research = orchestrator
            .start_bounded_parallel_workflow(&root, request)?
            .active_contexts()[0]
            .clone();
        let record = orchestrator.write_memory(
            &research,
            MemoryWriteTarget::TaskTemporary,
            MemoryContent::new("bounded child-lease memory fixture")?,
        )?;
        let selection = MemoryContextSelection::new([record.id().clone()])?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_CHILD_DURATION_SECONDS - 1));

        assert_eq!(
            orchestrator.read_memory(&research, record.id())?.content(),
            record.content()
        );
        assert_eq!(
            orchestrator
                .select_memory_context(&research, &selection)?
                .records()
                .len(),
            1
        );
        assert_eq!(orchestrator.memory.record_count(), 1);

        clock.advance(Duration::from_secs(1));
        assert_eq!(
            orchestrator.read_memory(&research, record.id()),
            Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded)
        );
        assert_eq!(
            orchestrator.select_memory_context(&research, &selection),
            Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded)
        );
        assert_eq!(orchestrator.memory.record_count(), 1);
        assert_eq!(
            orchestrator.write_memory(
                &research,
                MemoryWriteTarget::TaskTemporary,
                MemoryContent::new("must not be written after child deadline")?,
            ),
            Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded)
        );
        assert_eq!(orchestrator.memory.record_count(), 0);
        orchestrator.cancel_task(root.task_id())?;

        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        let research = orchestrator
            .start_bounded_parallel_workflow(&root, request)?
            .active_contexts()[0]
            .clone();
        let record = orchestrator.write_memory(
            &research,
            MemoryWriteTarget::TaskTemporary,
            MemoryContent::new("bounded delete deadline fixture")?,
        )?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_CHILD_DURATION_SECONDS));
        assert_eq!(
            orchestrator.delete_memory(&research, record.id(), record.version()),
            Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded)
        );
        assert_eq!(orchestrator.memory.record_count(), 0);
        orchestrator.cancel_task(root.task_id())?;
        Ok(())
    }

    #[test]
    fn root_deadline_is_open_at_n_minus_one_and_expires_exactly_at_n(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let contexts = acceptance.active_contexts().to_vec();
        complete_child(
            &mut orchestrator,
            &contexts[0],
            "root-deadline-first",
            &research_result("research-analysis", "approach-a"),
        )?;
        complete_child(
            &mut orchestrator,
            &contexts[1],
            "root-deadline-second",
            &research_result("knowledge-analysis", "decision-context"),
        )?;
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Synthesizing)
        );

        clock.advance(Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS - 1));
        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Synthesizing
        );
        assert_eq!(orchestrator.runs.len(), 1);

        clock.advance(Duration::from_secs(1));
        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Failed
        );
        assert!(orchestrator.runs.is_empty());
        assert!(orchestrator.events.iter().any(|event| matches!(
            event,
            AgentOrchestrationEvent::RootFailed {
                code: AgentTaskFailureCode::DeadlineExceeded,
                ..
            }
        )));
        Ok(())
    }

    #[test]
    fn waiting_root_expiry_uses_personal_root_waiting_attribution(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Failed
        );
        let terminal = orchestrator
            .bounded_parallel_audit_records()
            .last()
            .ok_or("missing waiting-root expiry audit")?;
        assert!(matches!(
            terminal.attribution(),
            BoundedParallelAttribution::PersonalRootWaiting(_)
        ));
        assert_eq!(terminal.attribution().runtime_id(), None);
        assert!(!terminal.attribution().has_live_run_binding());
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn synthesizing_root_expiry_already_terminal_mismatch_fails_once_and_retry_does_not_reinterpret(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let scenario_id = BoundedParallelScenarioId::ResearchKnowledgeIndependentV1;
        let request = BoundedParallelScenarioCatalog::built_in().request(scenario_id)?;
        let mut orchestrator = AgentOrchestrator::new(StartCrossesDeadlineRuntime::new(
            Arc::clone(&clock.now),
            4,
            Duration::ZERO,
        ))?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        let root_id = root.task_id().clone();
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        for (index, context) in acceptance.active_contexts().iter().enumerate() {
            let (work_item, fixture) = if index == 0 {
                ("research-analysis", "approach-a")
            } else {
                ("knowledge-analysis", "decision-context")
            };
            orchestrator.accept_runtime_event(
                context.task_id(),
                RuntimeEventEnvelope::for_identity(
                    context.runtime_run_identity(),
                    0,
                    UntrustedRuntimeEvent::ResponseStarted {
                        response_id: RuntimeResponseId::new(format!("expiry-child-{index}"))?,
                    },
                ),
            )?;
            orchestrator.accept_runtime_event(
                context.task_id(),
                RuntimeEventEnvelope::for_identity(
                    context.runtime_run_identity(),
                    1,
                    UntrustedRuntimeEvent::OutputTextDelta {
                        delta: RuntimeOutputText::new(research_result(work_item, fixture))?,
                    },
                ),
            )?;
            orchestrator.accept_runtime_event(
                context.task_id(),
                RuntimeEventEnvelope::for_identity(
                    context.runtime_run_identity(),
                    2,
                    UntrustedRuntimeEvent::ResponseCompleted,
                ),
            )?;
        }
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Synthesizing)
        );
        clock.advance(Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS));
        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines(),
            Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
                status: RuntimeRunStatus::Completed,
            })
        );
        assert_eq!(
            orchestrator.root_task().map(|task| task.status()),
            Some(AgentTaskStatus::Failed)
        );
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Failed)
        );
        let failed_count = orchestrator
            .events
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    AgentOrchestrationEvent::RootFailed {
                        code: AgentTaskFailureCode::RuntimeStateMismatch,
                        ..
                    }
                )
            })
            .count();
        assert_eq!(failed_count, 1);
        assert!(orchestrator.cancel_task(&root_id).is_ok());
        assert_eq!(
            orchestrator
                .events
                .iter()
                .filter(|event| matches!(event, AgentOrchestrationEvent::RootFailed { .. }))
                .count(),
            failed_count
        );
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn start_crossing_root_deadline_with_contradictory_terminal_cancel_never_projects_live_timeout(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let now = Arc::new(Mutex::new(std::time::Instant::now()));
        let runtime = StartCrossesDeadlineRuntime::new(
            Arc::clone(&now),
            2,
            Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS),
        );
        let clock_state = Arc::clone(&now);
        let injected: workflow_automation_dispatch::WorkflowClock = Arc::new(move || {
            *clock_state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        });
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;

        assert!(matches!(
            orchestrator.start_bounded_parallel_workflow(&root, request),
            Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
                status: RuntimeRunStatus::Completed
            })
        ));
        let attempted_task_id = AgentTaskId::new(format!(
            "agent-task-child-{}-1",
            orchestrator.workflow_sequence
        ))?;
        assert!(!orchestrator.runs.contains_key(&attempted_task_id));
        assert!(matches!(
            orchestrator
                .tasks
                .get(&attempted_task_id)
                .map(AgentTask::status),
            Some(AgentTaskStatus::Failed)
        ));
        let attempted_events = orchestrator
            .events
            .iter()
            .filter(|event| match event {
                AgentOrchestrationEvent::ChildCreated { task_id, .. }
                | AgentOrchestrationEvent::ChildStarted { task_id }
                | AgentOrchestrationEvent::ChildFailed { task_id, .. } => {
                    task_id == &attempted_task_id
                }
                AgentOrchestrationEvent::ResultReturned { child_task_id, .. } => {
                    child_task_id == &attempted_task_id
                }
                _ => false,
            })
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(
            attempted_events,
            vec![
                AgentOrchestrationEvent::ChildCreated {
                    task_id: attempted_task_id.clone(),
                    parent_task_id: root.task_id().clone(),
                },
                AgentOrchestrationEvent::ChildFailed {
                    task_id: attempted_task_id.clone(),
                    code: AgentTaskFailureCode::RuntimeStateMismatch,
                },
                AgentOrchestrationEvent::ResultReturned {
                    child_task_id: attempted_task_id.clone(),
                    parent_task_id: root.task_id().clone(),
                    outcome: AgentTaskOutcomeKind::Failed,
                },
            ]
        );
        assert!(!orchestrator.bounded_parallel_events().iter().any(|event| {
            matches!(
                event,
                BoundedParallelWorkflowEvent::Started { ordinal: 1, .. }
                    | BoundedParallelWorkflowEvent::TimedOut { ordinal: 1, .. }
            )
        }));
        assert!(orchestrator
            .bounded_parallel_audit_records()
            .iter()
            .all(|record| {
                record.attribution().ordinal() != Some(1)
                    || (!record.attribution().has_live_run_binding()
                        && record.outcome() != BoundedParallelAuditOutcome::TimedOut)
            }));
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Failed)
        );
        assert!(orchestrator.runs.is_empty());
        assert!(!orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(
                event,
                BoundedParallelWorkflowEvent::SynthesisStarted { .. }
                    | BoundedParallelWorkflowEvent::SynthesisCompleted { .. }
            )));
        Ok(())
    }

    #[test]
    fn start_crossing_root_deadline_with_successful_cancel_emits_exact_generic_failure_order(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let now = Arc::new(Mutex::new(std::time::Instant::now()));
        let runtime = StartCrossesDeadlineRuntime::with_normal_cancel(
            Arc::clone(&now),
            2,
            Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS),
        );
        let clock_state = Arc::clone(&now);
        let injected: workflow_automation_dispatch::WorkflowClock = Arc::new(move || {
            *clock_state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        });
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;

        assert_eq!(
            orchestrator.start_bounded_parallel_workflow(&root, request),
            Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded)
        );
        let attempted_task_id = AgentTaskId::new(format!(
            "agent-task-child-{}-1",
            orchestrator.workflow_sequence
        ))?;
        let attempted_events = orchestrator
            .events
            .iter()
            .filter(|event| match event {
                AgentOrchestrationEvent::ChildCreated { task_id, .. }
                | AgentOrchestrationEvent::ChildStarted { task_id }
                | AgentOrchestrationEvent::ChildFailed { task_id, .. } => {
                    task_id == &attempted_task_id
                }
                AgentOrchestrationEvent::ResultReturned { child_task_id, .. } => {
                    child_task_id == &attempted_task_id
                }
                _ => false,
            })
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(
            attempted_events,
            vec![
                AgentOrchestrationEvent::ChildCreated {
                    task_id: attempted_task_id.clone(),
                    parent_task_id: root.task_id().clone(),
                },
                AgentOrchestrationEvent::ChildFailed {
                    task_id: attempted_task_id.clone(),
                    code: AgentTaskFailureCode::DeadlineExceeded,
                },
                AgentOrchestrationEvent::ResultReturned {
                    child_task_id: attempted_task_id.clone(),
                    parent_task_id: root.task_id().clone(),
                    outcome: AgentTaskOutcomeKind::Failed,
                },
            ]
        );
        assert_eq!(
            orchestrator.task(&attempted_task_id).map(AgentTask::status),
            Some(AgentTaskStatus::Failed)
        );
        assert_eq!(
            orchestrator.root_task().map(AgentTask::status),
            Some(AgentTaskStatus::Failed)
        );
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Failed)
        );
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn start_crossing_root_deadline_retains_a_nonterminal_cancel_contradiction_until_retry(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let now = Arc::new(Mutex::new(std::time::Instant::now()));
        let runtime = StartCrossesDeadlineRuntime::with_nonterminal_cancel(
            Arc::clone(&now),
            2,
            Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS),
        );
        let clock_state = Arc::clone(&now);
        let injected: workflow_automation_dispatch::WorkflowClock = Arc::new(move || {
            *clock_state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        });
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;

        assert_eq!(
            orchestrator.start_bounded_parallel_workflow(&root, request),
            Err(AgentOrchestratorError::UnexpectedRuntimeStatus {
                status: RuntimeRunStatus::Streaming,
            })
        );
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelling)
        );
        assert_eq!(orchestrator.runs.len(), 1);
        assert!(orchestrator
            .tasks
            .values()
            .any(|task| task.status() == AgentTaskStatus::Running));
        assert!(!orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(event, BoundedParallelWorkflowEvent::TimedOut { .. })));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Failed
        );
        assert!(orchestrator.runs.is_empty());
        assert!(orchestrator
            .tasks
            .values()
            .all(|task| task.status().is_terminal()));
        assert!(orchestrator.bounded_parallel_outcomes().is_none());
        Ok(())
    }

    #[test]
    fn accepted_terminal_event_wins_when_runtime_acceptance_crosses_the_root_deadline(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let now = Arc::new(Mutex::new(std::time::Instant::now()));
        let runtime = AcceptCrossesDeadlineRuntime {
            next_start: Arc::new(AtomicU8::new(0)),
            clock: Arc::clone(&now),
            advance_at: 2,
            advance_by: Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS),
        };
        let clock_state = Arc::clone(&now);
        let injected: workflow_automation_dispatch::WorkflowClock = Arc::new(move || {
            *clock_state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        });
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let research = acceptance.active_contexts()[0].clone();

        orchestrator.accept_runtime_event(
            research.task_id(),
            RuntimeEventEnvelope::for_identity(
                research.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: RuntimeResponseId::new("accept-crossing-research")?,
                },
            ),
        )?;
        orchestrator.accept_runtime_event(
            research.task_id(),
            RuntimeEventEnvelope::for_identity(
                research.runtime_run_identity(),
                1,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: RuntimeOutputText::new(research_result(
                        "research-analysis",
                        "approach-a",
                    ))?,
                },
            ),
        )?;
        assert_eq!(
            orchestrator.accept_runtime_event(
                research.task_id(),
                RuntimeEventEnvelope::for_identity(
                    research.runtime_run_identity(),
                    2,
                    UntrustedRuntimeEvent::ResponseCompleted,
                ),
            )?,
            RuntimeEventAcceptance::ResponseCompleted
        );
        let outcomes = &orchestrator
            .bounded_parallel
            .as_ref()
            .ok_or("missing bounded-parallel state")?
            .slots;
        assert!(matches!(
            outcomes[0].state,
            ParallelSlotState::Terminal(ref outcome)
                if outcome.status() == ParallelChildResultStatus::Succeeded
        ));
        assert!(matches!(
            outcomes[1].state,
            ParallelSlotState::Running { .. }
        ));
        assert!(!orchestrator.bounded_parallel_events().iter().any(|event| {
            matches!(
                event,
                BoundedParallelWorkflowEvent::TimedOut { ordinal: 1, .. }
            )
        }));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Failed
        );
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn timeout_cancel_failure_retains_live_run_and_next_poll_resumes_cursor(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(CancelFailOnceRuntime::new(2))?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let first_task_id = orchestrator
            .bounded_parallel
            .as_ref()
            .and_then(|state| match &state.slots[0].state {
                ParallelSlotState::Running { task_id, .. } => Some(task_id.clone()),
                _ => None,
            })
            .ok_or("first child was not running")?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_CHILD_DURATION_SECONDS));

        assert!(matches!(
            orchestrator.check_bounded_parallel_deadlines(),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
            ))
        ));
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelling)
        );
        assert!(orchestrator.runs.contains_key(&first_task_id));
        assert_eq!(orchestrator.runs.len(), 2);

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Synthesizing
        );
        assert!(!orchestrator.runs.contains_key(&first_task_id));
        assert_eq!(orchestrator.runs.len(), 1);
        assert!(matches!(
            &orchestrator
                .bounded_parallel
                .as_ref()
                .ok_or("missing bounded-parallel state")?
                .slots[0]
                .state,
            ParallelSlotState::Terminal(outcome)
                if outcome.status() == ParallelChildResultStatus::TimedOut
        ));

        orchestrator.cancel_task(root.task_id())?;
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn resumed_child_timeout_at_root_deadline_is_immediately_superseded_by_root_expiry(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(CancelFailOnceRuntime::new(2))?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        assert_eq!(orchestrator.runs.len(), 2);
        clock.advance(Duration::from_secs(MAX_PARALLEL_CHILD_DURATION_SECONDS));

        assert!(matches!(
            orchestrator.check_bounded_parallel_deadlines(),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
            ))
        ));
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelling)
        );
        assert_eq!(orchestrator.runs.len(), 2);

        clock.advance(Duration::from_secs(
            MAX_PARALLEL_ROOT_DURATION_SECONDS - MAX_PARALLEL_CHILD_DURATION_SECONDS,
        ));
        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Failed
        );
        assert_eq!(
            orchestrator.root_task().map(AgentTask::status),
            Some(AgentTaskStatus::Failed)
        );
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Failed)
        );
        assert!(orchestrator.bounded_parallel_result().is_none());
        assert!(orchestrator.runs.is_empty());
        assert!(orchestrator
            .tasks
            .values()
            .all(|task| task.status().is_terminal()));
        assert!(orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(
                event,
                BoundedParallelWorkflowEvent::TimedOut {
                    ordinal: 1,
                    reason: ParallelTimeoutReason::ChildDeadlineExceeded,
                    ..
                }
            )));
        assert!(orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(
                event,
                BoundedParallelWorkflowEvent::TimedOut {
                    ordinal: 2,
                    reason: ParallelTimeoutReason::RootDeadlineExceeded,
                    ..
                }
            )));
        assert!(!orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(
                event,
                BoundedParallelWorkflowEvent::SynthesisStarted { .. }
                    | BoundedParallelWorkflowEvent::SynthesisCompleted { .. }
            )));
        Ok(())
    }

    #[test]
    fn root_cancel_crossing_a_child_timeout_cursor_still_cancels_the_root(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let runtime = CancelFailOnceRuntime::new(2);
        let starts = Arc::clone(&runtime.next_start);
        let mut orchestrator = AgentOrchestrator::new(runtime)?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_CHILD_DURATION_SECONDS));
        assert!(matches!(
            orchestrator.check_bounded_parallel_deadlines(),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
            ))
        ));
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelling)
        );
        let starts_before_root_cancel = starts.load(Ordering::SeqCst);
        let workflow_events_before_root_cancel = orchestrator.bounded_parallel_events().len();

        assert_eq!(
            orchestrator.cancel_task(root.task_id())?,
            AgentTaskCancellationOutcome::Cancelled
        );
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelled)
        );
        assert_eq!(starts.load(Ordering::SeqCst), starts_before_root_cancel);
        assert!(
            !orchestrator.bounded_parallel_events()[workflow_events_before_root_cancel..]
                .iter()
                .any(|event| matches!(
                    event,
                    BoundedParallelWorkflowEvent::DependencySatisfied { .. }
                        | BoundedParallelWorkflowEvent::ParentResumed { .. }
                        | BoundedParallelWorkflowEvent::SynthesisStarted { .. }
                ))
        );
        assert!(orchestrator.runs.is_empty());
        assert!(orchestrator.tasks.values().all(|task| {
            matches!(
                task.status(),
                AgentTaskStatus::Cancelled | AgentTaskStatus::Failed | AgentTaskStatus::Completed
            )
        }));
        Ok(())
    }

    #[test]
    fn root_cancel_at_root_n_uses_deadline_expiry_for_running_and_retained_timeout_states(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS));

        assert_eq!(
            orchestrator.cancel_task(root.task_id()),
            Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded)
        );
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Failed)
        );
        assert!(orchestrator
            .bounded_parallel_outcomes()
            .is_some_and(|outcomes| outcomes.iter().all(|outcome| {
                outcome.status() == ParallelChildResultStatus::TimedOut
                    && matches!(
                        outcome.disposition(),
                        ParallelChildDisposition::TimedOut(
                            ParallelTimeoutReason::RootDeadlineExceeded
                        )
                    )
            })));
        assert!(orchestrator.runs.is_empty());

        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(CancelFailOnceRuntime::new(2))?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_CHILD_DURATION_SECONDS));
        assert!(matches!(
            orchestrator.check_bounded_parallel_deadlines(),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
            ))
        ));
        clock.advance(Duration::from_secs(
            MAX_PARALLEL_ROOT_DURATION_SECONDS - MAX_PARALLEL_CHILD_DURATION_SECONDS,
        ));

        assert_eq!(
            orchestrator.cancel_task(root.task_id()),
            Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded)
        );
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Failed)
        );
        assert!(orchestrator.runs.is_empty());
        assert!(!orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(
                event,
                BoundedParallelWorkflowEvent::SynthesisStarted { .. }
                    | BoundedParallelWorkflowEvent::SynthesisCompleted { .. }
            )));
        Ok(())
    }

    #[test]
    fn root_cancellation_sweep_retries_every_child_ordinal_without_false_terminal(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for child_count in [2u8, 3] {
            for failed_ordinal in 1..=child_count {
                let request = if child_count == 2 {
                    BoundedParallelScenarioCatalog::built_in()
                        .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?
                } else {
                    BoundedParallelScenarioCatalog::built_in()
                        .three_independent_for_test(HARD_MAX_ACTIVE_PARALLEL_CHILDREN)?
                        .with_active_limit_for_test(HARD_MAX_ACTIVE_PARALLEL_CHILDREN)?
                };
                let mut orchestrator =
                    AgentOrchestrator::new(CancelFailOnceRuntime::new(failed_ordinal + 1))?;
                let root = orchestrator.start_root(request.objective())?;
                orchestrator.start_bounded_parallel_workflow(&root, request)?;
                let failed_task_id = orchestrator
                    .bounded_parallel
                    .as_ref()
                    .and_then(|state| {
                        state.slots.get(usize::from(failed_ordinal - 1)).and_then(
                            |slot| match &slot.state {
                                ParallelSlotState::Running { task_id, .. } => Some(task_id.clone()),
                                _ => None,
                            },
                        )
                    })
                    .ok_or("missing failed cancellation target")?;

                assert!(matches!(
                    orchestrator.cancel_task(root.task_id()),
                    Err(AgentOrchestratorError::Runtime(
                        RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
                    ))
                ));
                assert_eq!(
                    orchestrator.bounded_parallel_status(),
                    Some(BoundedParallelWorkflowStatus::Cancelling)
                );
                assert!(orchestrator.runs.contains_key(&failed_task_id));
                assert!(!orchestrator
                    .bounded_parallel_events()
                    .iter()
                    .any(|event| matches!(event, BoundedParallelWorkflowEvent::Terminal { .. })));

                assert_eq!(
                    orchestrator.cancel_task(root.task_id())?,
                    AgentTaskCancellationOutcome::Cancelled
                );
                assert_eq!(
                    orchestrator.bounded_parallel_status(),
                    Some(BoundedParallelWorkflowStatus::Cancelled)
                );
                assert!(orchestrator.runs.is_empty());
            }
        }
        Ok(())
    }

    #[test]
    fn fail_fast_sibling_cancel_failure_retains_run_then_resumes_to_partial_synthesis(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::CloudSystemsSecurityV1)?;
        let mut orchestrator = AgentOrchestrator::new(CancelFailOnceRuntime::new(2))?;
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let cloud = acceptance.active_contexts()[0].clone();
        let systems = acceptance.active_contexts()[1].clone();
        orchestrator.accept_runtime_event(
            systems.task_id(),
            RuntimeEventEnvelope::for_identity(
                systems.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: RuntimeResponseId::new("fail-fast-cancel-retry")?,
                },
            ),
        )?;
        assert!(matches!(
            orchestrator.accept_runtime_event(
                systems.task_id(),
                RuntimeEventEnvelope::for_identity(
                    systems.runtime_run_identity(),
                    1,
                    UntrustedRuntimeEvent::ResponseFailed {
                        failure: RuntimeFailure::new(
                            RuntimeFailureCode::ProviderUnavailable,
                            false,
                            None,
                        )?,
                    },
                ),
            ),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
            ))
        ));
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelling)
        );
        assert!(orchestrator.runs.contains_key(cloud.task_id()));
        assert!(!orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(event, BoundedParallelWorkflowEvent::Terminal { .. })));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Synthesizing
        );
        assert!(!orchestrator.runs.contains_key(cloud.task_id()));
        assert_eq!(orchestrator.runs.len(), 1);
        orchestrator.cancel_task(root.task_id())?;
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn fail_fast_cancel_failure_on_second_initial_sibling_retains_then_resumes(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::CloudSystemsSecurityV1)?;
        let mut orchestrator = AgentOrchestrator::new(CancelFailOnceRuntime::new(3))?;
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let cloud = acceptance.active_contexts()[0].clone();
        let systems = acceptance.active_contexts()[1].clone();
        orchestrator.accept_runtime_event(
            cloud.task_id(),
            RuntimeEventEnvelope::for_identity(
                cloud.runtime_run_identity(),
                0,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: RuntimeResponseId::new("fail-fast-second-cancel-retry")?,
                },
            ),
        )?;
        assert!(matches!(
            orchestrator.accept_runtime_event(
                cloud.task_id(),
                RuntimeEventEnvelope::for_identity(
                    cloud.runtime_run_identity(),
                    1,
                    UntrustedRuntimeEvent::ResponseFailed {
                        failure: RuntimeFailure::new(
                            RuntimeFailureCode::ProviderUnavailable,
                            false,
                            None,
                        )?,
                    },
                ),
            ),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
            ))
        ));
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelling)
        );
        assert!(orchestrator.runs.contains_key(systems.task_id()));
        assert!(!orchestrator
            .bounded_parallel_events()
            .iter()
            .any(|event| matches!(event, BoundedParallelWorkflowEvent::Terminal { .. })));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Synthesizing
        );
        assert!(!orchestrator.runs.contains_key(systems.task_id()));
        assert_eq!(orchestrator.runs.len(), 1);
        orchestrator.cancel_task(root.task_id())?;
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn root_expiry_cancel_failure_retains_live_run_and_poll_completes_expiry(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::new(CancelFailOnceRuntime::new(2))?;
        orchestrator.set_workflow_clock_for_test(injected);
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let first_task_id = orchestrator
            .bounded_parallel
            .as_ref()
            .and_then(|state| match &state.slots[0].state {
                ParallelSlotState::Running { task_id, .. } => Some(task_id.clone()),
                _ => None,
            })
            .ok_or("first child was not running")?;
        clock.advance(Duration::from_secs(MAX_PARALLEL_ROOT_DURATION_SECONDS));

        assert!(matches!(
            orchestrator.check_bounded_parallel_deadlines(),
            Err(AgentOrchestratorError::Runtime(
                RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Cancellation)
            ))
        ));
        assert_eq!(
            orchestrator.bounded_parallel_status(),
            Some(BoundedParallelWorkflowStatus::Cancelling)
        );
        assert!(orchestrator.runs.contains_key(&first_task_id));

        assert_eq!(
            orchestrator.check_bounded_parallel_deadlines()?,
            BoundedParallelWorkflowStatus::Failed
        );
        assert!(orchestrator.runs.is_empty());
        assert!(matches!(
            orchestrator
                .tasks
                .get(root.task_id())
                .map(AgentTask::status),
            Some(AgentTaskStatus::Failed)
        ));
        Ok(())
    }

    #[test]
    fn stale_and_foreign_child_cancellation_handles_are_rejected_without_mutation(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let catalog = BoundedParallelScenarioCatalog::built_in();
        let request = catalog.request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut first = AgentOrchestrator::native()?;
        let first_root = first.start_root(request.objective())?;
        first.start_bounded_parallel_workflow(&first_root, request.clone())?;
        let foreign_handle = first
            .bounded_parallel_active_children()?
            .remove(0)
            .into_cancellation_handle();

        let mut second = AgentOrchestrator::native()?;
        let second_root = second.start_root(request.objective())?;
        second.start_bounded_parallel_workflow(&second_root, request)?;
        let before = (second.runs.len(), second.events.len());
        assert_eq!(
            second.cancel_bounded_parallel_child(foreign_handle),
            Err(AgentOrchestratorError::BoundedParallelCancellationHandleMismatch)
        );
        assert_eq!((second.runs.len(), second.events.len()), before);

        let stale_control = second.bounded_parallel_active_children()?.remove(0);
        let stale_task_id = stale_control.context().task_id().clone();
        let stale_handle = stale_control.into_cancellation_handle();
        second.cancel_bounded_parallel_child_task(
            &stale_task_id,
            ParallelCancellationReason::UserRequested,
            false,
        )?;
        let before = (second.runs.len(), second.events.len());
        assert_eq!(
            second.cancel_bounded_parallel_child(stale_handle),
            Err(AgentOrchestratorError::BoundedParallelCancellationHandleMismatch)
        );
        assert_eq!((second.runs.len(), second.events.len()), before);

        first.cancel_task(first_root.task_id())?;
        second.cancel_task(second_root.task_id())?;
        Ok(())
    }

    #[test]
    fn deferred_registry_agent_rejects_admission_before_any_workflow_mutation(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let definitions = AgentId::ALL
            .into_iter()
            .map(|agent_id| {
                AgentDefinition::built_in(agent_id).map(|definition| {
                    if agent_id == AgentId::Research {
                        definition.with_activation_for_test(AgentActivation::Deferred(
                            AgentActivationGate::TypedWorkflowGovernance,
                        ))
                    } else {
                        definition
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let registry = AgentRegistry::from_definitions(definitions)?;
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::with_registry(NativeAgentRuntime, registry)?;
        let root = orchestrator.start_root(request.objective())?;
        let before = (
            orchestrator.tasks.len(),
            orchestrator.runs.len(),
            orchestrator.run_count,
            orchestrator.events.len(),
        );

        assert_eq!(
            orchestrator.start_bounded_parallel_workflow(&root, request),
            Err(AgentOrchestratorError::AgentDeferred {
                agent_id: AgentId::Research
            })
        );
        assert_eq!(
            (
                orchestrator.tasks.len(),
                orchestrator.runs.len(),
                orchestrator.run_count,
                orchestrator.events.len(),
            ),
            before
        );
        assert!(orchestrator.bounded_parallel.is_none());
        assert!(orchestrator.selected_workflow.is_none());
        orchestrator.cancel_task(root.task_id())?;
        Ok(())
    }

    #[test]
    fn paired_workflow_and_audit_transition_capacity_is_atomic_at_n_and_n_plus_one(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let mut orchestrator = AgentOrchestrator::native()?;
        let root = orchestrator.start_root(request.objective())?;
        orchestrator.start_bounded_parallel_workflow(&root, request)?;
        let root_id = root.task_id().clone();
        let (item, request) = {
            let state = orchestrator
                .bounded_parallel
                .as_ref()
                .ok_or("missing bounded-parallel state")?;
            (state.slots[0].item.clone(), state.request.clone())
        };
        let attribution = orchestrator.planned_parallel_attribution(&root_id, &request, &item)?;
        let mut state = orchestrator
            .bounded_parallel
            .take()
            .ok_or("missing bounded-parallel state")?;
        let initial_events = state.events.clone();
        let initial_audit = state.audit.clone();
        while state.events.len() < MAX_PARALLEL_WORKFLOW_EVENTS - 1 {
            orchestrator.push_bounded_parallel_transition(
                &mut state,
                BoundedParallelWorkflowEvent::Progress {
                    work_item_id: item.id().clone(),
                    ordinal: item.ordinal(),
                },
                attribution.clone(),
                BoundedParallelAuditOutcome::Progress,
            )?;
        }
        assert_eq!(state.events.len(), MAX_PARALLEL_WORKFLOW_EVENTS - 1);
        assert_eq!(state.events.len(), state.audit.len());
        orchestrator.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Progress {
                work_item_id: item.id().clone(),
                ordinal: item.ordinal(),
            },
            attribution.clone(),
            BoundedParallelAuditOutcome::Progress,
        )?;
        assert_eq!(state.events.len(), MAX_PARALLEL_WORKFLOW_EVENTS);
        assert_eq!(state.events.len(), state.audit.len());
        let before = (state.events.clone(), state.audit.clone());

        assert_eq!(
            orchestrator.push_bounded_parallel_transition(
                &mut state,
                BoundedParallelWorkflowEvent::Progress {
                    work_item_id: item.id().clone(),
                    ordinal: item.ordinal(),
                },
                attribution,
                BoundedParallelAuditOutcome::Progress,
            ),
            Err(AgentOrchestratorError::BoundedParallelJournalLimitExceeded)
        );
        assert_eq!((state.events.clone(), state.audit.clone()), before);
        state.events = initial_events;
        state.audit = initial_audit;
        orchestrator.bounded_parallel = Some(state);
        orchestrator.cancel_task(root.task_id())?;
        assert!(orchestrator.runs.is_empty());
        Ok(())
    }

    #[test]
    fn per_run_and_aggregate_runtime_event_limits_accept_n_then_reject_n_plus_one(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for phase in 0..3 {
            let request = BoundedParallelScenarioCatalog::built_in()
                .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
            let mut orchestrator = AgentOrchestrator::native()?;
            let root = orchestrator.start_root(request.objective())?;
            let acceptance = orchestrator.start_bounded_parallel_workflow(&root, request)?;
            let child = acceptance.active_contexts()[0].clone();
            let task_id = child.task_id().clone();
            if phase < 2 {
                let state = orchestrator
                    .bounded_parallel
                    .as_mut()
                    .ok_or("missing bounded-parallel state")?;
                state.accepted_events.insert(
                    task_id.clone(),
                    if phase == 0 {
                        MAX_PARALLEL_EVENTS_PER_RUN - 2
                    } else {
                        MAX_PARALLEL_EVENTS_PER_RUN
                    },
                );
            }
            if phase == 0 {
                orchestrator.accept_runtime_event(
                    &task_id,
                    RuntimeEventEnvelope::for_identity(
                        child.runtime_run_identity(),
                        0,
                        UntrustedRuntimeEvent::ResponseStarted {
                            response_id: RuntimeResponseId::new("per-run-event-n-minus-one")?,
                        },
                    ),
                )?;
                orchestrator.accept_runtime_event(
                    &task_id,
                    RuntimeEventEnvelope::for_identity(
                        child.runtime_run_identity(),
                        1,
                        UntrustedRuntimeEvent::ResponseFailed {
                            failure: RuntimeFailure::new(
                                RuntimeFailureCode::ProviderUnavailable,
                                false,
                                None,
                            )?,
                        },
                    ),
                )?;
                assert_eq!(
                    orchestrator
                        .bounded_parallel
                        .as_ref()
                        .and_then(|state| state.accepted_events.get(&task_id).copied()),
                    Some(MAX_PARALLEL_EVENTS_PER_RUN)
                );
            } else if phase == 1 {
                assert_eq!(
                    orchestrator.accept_runtime_event(
                        &task_id,
                        RuntimeEventEnvelope::for_identity(
                            child.runtime_run_identity(),
                            0,
                            UntrustedRuntimeEvent::ResponseStarted {
                                response_id: RuntimeResponseId::new("per-run-event-n-plus-one")?,
                            },
                        ),
                    ),
                    Err(AgentOrchestratorError::RuntimeEventLimitExceeded)
                );
                assert_eq!(
                    orchestrator.task(&task_id).map(|task| task.status()),
                    Some(AgentTaskStatus::Running)
                );
            } else {
                orchestrator.runtime_event_count = MAX_PARALLEL_RUNTIME_EVENTS - 1;
                orchestrator.accept_runtime_event(
                    &task_id,
                    RuntimeEventEnvelope::for_identity(
                        child.runtime_run_identity(),
                        0,
                        UntrustedRuntimeEvent::ResponseStarted {
                            response_id: RuntimeResponseId::new("aggregate-event-n")?,
                        },
                    ),
                )?;
                assert_eq!(
                    orchestrator.runtime_event_count,
                    MAX_PARALLEL_RUNTIME_EVENTS
                );
                let other = acceptance.active_contexts()[1].clone();
                assert_eq!(
                    orchestrator.accept_runtime_event(
                        other.task_id(),
                        RuntimeEventEnvelope::for_identity(
                            other.runtime_run_identity(),
                            0,
                            UntrustedRuntimeEvent::ResponseStarted {
                                response_id: RuntimeResponseId::new("aggregate-event-n-plus-one")?,
                            },
                        ),
                    ),
                    Err(AgentOrchestratorError::RuntimeEventLimitExceeded)
                );
                assert_eq!(
                    orchestrator.task(other.task_id()).map(|task| task.status()),
                    Some(AgentTaskStatus::Failed)
                );
            }
            orchestrator.cancel_task(root.task_id())?;
            assert!(orchestrator.runs.is_empty());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParallelSlot {
    item: ParallelWorkItem,
    state: ParallelSlotState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParallelCancellationMode {
    Root,
    RootExpired,
    FailFast,
    ChildTimeout,
    ForcedFailure,
    ForcedSynthesisFailure,
    RejectedChildStart,
    RejectedSynthesisStart,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParallelCancellationCursor {
    mode: ParallelCancellationMode,
    ordinals: Vec<u8>,
    next: usize,
    failure_code: Option<AgentTaskFailureCode>,
}

pub(super) struct BoundedParallelWorkflowState {
    request: BoundedParallelWorkflowRequest,
    slots: Vec<ParallelSlot>,
    task_to_ordinal: BTreeMap<AgentTaskId, u8>,
    root_deadline: std::time::Instant,
    status: BoundedParallelWorkflowStatus,
    accepted_events: BTreeMap<AgentTaskId, u32>,
    synthesis_progress_recorded: bool,
    events: Vec<BoundedParallelWorkflowEvent>,
    audit: Vec<BoundedParallelAuditRecord>,
    result: Option<BoundedParallelWorkflowResult>,
    cancellation: Option<ParallelCancellationCursor>,
}

impl BoundedParallelWorkflowState {
    fn new(request: BoundedParallelWorkflowRequest, root_deadline: std::time::Instant) -> Self {
        let slots = request
            .work_items()
            .iter()
            .cloned()
            .map(|item| ParallelSlot {
                item,
                state: ParallelSlotState::Pending,
            })
            .collect();
        Self {
            request,
            slots,
            task_to_ordinal: BTreeMap::new(),
            root_deadline,
            status: BoundedParallelWorkflowStatus::Running,
            accepted_events: BTreeMap::new(),
            synthesis_progress_recorded: false,
            events: Vec::with_capacity(MAX_PARALLEL_WORKFLOW_EVENTS),
            audit: Vec::with_capacity(MAX_PARALLEL_AUDIT_RECORDS),
            result: None,
            cancellation: None,
        }
    }

    pub(super) fn tracks(&self, root_id: &AgentTaskId, task_id: &AgentTaskId) -> bool {
        self.task_to_ordinal.contains_key(task_id)
            || (root_id == task_id
                && matches!(
                    self.status,
                    BoundedParallelWorkflowStatus::Synthesizing
                        | BoundedParallelWorkflowStatus::Cancelling
                ))
    }

    fn active_count(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| matches!(slot.state, ParallelSlotState::Running { .. }))
            .count()
    }

    fn all_terminal(&self) -> bool {
        self.slots
            .iter()
            .all(|slot| matches!(slot.state, ParallelSlotState::Terminal(_)))
    }

    fn outcomes(&self) -> Option<Vec<ParallelChildOutcome>> {
        self.slots
            .iter()
            .map(|slot| match &slot.state {
                ParallelSlotState::Terminal(outcome) => Some(outcome.clone()),
                ParallelSlotState::Pending | ParallelSlotState::Running { .. } => None,
            })
            .collect()
    }

    fn slot(&self, ordinal: u8) -> AgentOrchestratorResult<&ParallelSlot> {
        self.slots
            .get(usize::from(ordinal.saturating_sub(1)))
            .filter(|slot| slot.item.ordinal() == ordinal)
            .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)
    }

    fn slot_mut(&mut self, ordinal: u8) -> AgentOrchestratorResult<&mut ParallelSlot> {
        self.slots
            .get_mut(usize::from(ordinal.saturating_sub(1)))
            .filter(|slot| slot.item.ordinal() == ordinal)
            .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)
    }

    pub(super) fn ordinal_for_task(&self, task_id: &AgentTaskId) -> Option<u8> {
        self.task_to_ordinal.get(task_id).copied()
    }

    fn dependencies_succeeded(&self, item: &ParallelWorkItem) -> bool {
        item.dependencies().iter().all(|dependency| {
            self.slots.iter().any(|slot| {
                slot.item.id() == dependency
                    && matches!(
                        &slot.state,
                        ParallelSlotState::Terminal(outcome)
                            if outcome.status() == ParallelChildResultStatus::Succeeded
                    )
            })
        })
    }

    fn dependencies_terminal(&self, item: &ParallelWorkItem) -> bool {
        item.dependencies().iter().all(|dependency| {
            self.slots.iter().any(|slot| {
                slot.item.id() == dependency && matches!(slot.state, ParallelSlotState::Terminal(_))
            })
        })
    }
}

pub(super) enum PreparedBoundedParallelTerminal {
    Child {
        ordinal: u8,
        output: Option<AgentTaskOutput>,
        disposition: ParallelChildDisposition,
    },
    SynthesisCompleted {
        output: AgentTaskOutput,
        synthesis: crate::agent::bounded_parallelism::BoundedParallelSynthesis,
        outcomes: Vec<ParallelChildOutcome>,
    },
    SynthesisFailed {
        code: AgentTaskFailureCode,
    },
}

impl<R: AgentRuntime> AgentOrchestrator<R> {
    pub fn start_bounded_parallel_workflow(
        &mut self,
        source_context: &AgentExecutionContext,
        request: BoundedParallelWorkflowRequest,
    ) -> AgentOrchestratorResult<BoundedParallelWorkflowAcceptance> {
        self.ensure_workflow_selection_available(AgentWorkflowSelection::BoundedParallel, false)?;
        let attribution = self.live_personal_root_attribution(source_context)?;
        if self.governance.has_pending_for(&attribution) {
            return Err(AgentOrchestratorError::GovernanceApprovalPending);
        }
        if self.active_child_task_id.is_some() || self.child_created {
            return Err(AgentOrchestratorError::ActiveChildLimitExceeded);
        }
        let root_task_id = attribution.task_id().clone();
        let root_active = self
            .runs
            .get(&root_task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        if root_active.run.status() != RuntimeRunStatus::AwaitingStart
            || !root_active.output.is_empty()
        {
            return Err(AgentOrchestratorError::DelegationAfterRuntimeOutput);
        }
        request.validate_integrity()?;
        if request.work_items().len() > MAX_PARALLEL_CHILDREN
            || request.active_limit() == 0
            || request.active_limit() > HARD_MAX_ACTIVE_PARALLEL_CHILDREN
        {
            return Err(AgentOrchestratorError::ActiveChildLimitExceeded);
        }
        if self
            .tasks
            .len()
            .checked_add(request.work_items().len())
            .is_none_or(|count| count > MAX_PARALLEL_TASKS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::TotalChildLimitExceeded);
        }
        if self
            .run_count
            .checked_add(
                u8::try_from(request.work_items().len())
                    .map_err(|_| AgentOrchestratorError::RunLimitExceeded)?
                    .saturating_add(1),
            )
            .is_none_or(|count| count > MAX_PARALLEL_RUNTIME_RUNS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        if self.runtime_event_count >= MAX_PARALLEL_RUNTIME_EVENTS {
            return Err(AgentOrchestratorError::RuntimeEventLimitExceeded);
        }
        self.ensure_event_capacity(
            MAX_PARALLEL_ORCHESTRATION_EVENTS.saturating_sub(self.events.len()),
        )?;
        for item in request.work_items() {
            let definition = self.registry.get(item.agent_id())?;
            if definition.activation() != AgentActivation::Initial {
                return Err(AgentOrchestratorError::AgentDeferred {
                    agent_id: item.agent_id(),
                });
            }
            if item.dependencies().is_empty() {
                request.build_child_input(item.ordinal(), &[])?;
            }
        }
        let deadline = deadline_after(self.workflow_now(), MAX_PARALLEL_ROOT_DURATION_SECONDS)?;
        let mut state = BoundedParallelWorkflowState::new(request, deadline);
        let queued_items = state
            .slots
            .iter()
            .map(|slot| slot.item.clone())
            .collect::<Vec<_>>();
        for item in queued_items {
            let attribution =
                self.planned_parallel_attribution(&root_task_id, &state.request, &item)?;
            self.push_bounded_parallel_transition(
                &mut state,
                BoundedParallelWorkflowEvent::Queued {
                    work_item_id: item.id().clone(),
                    ordinal: item.ordinal(),
                },
                attribution,
                BoundedParallelAuditOutcome::Queued,
            )?;
        }

        let mut root_run = self
            .runs
            .remove(&root_task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        match root_run.run.cancel() {
            Ok(RuntimeCancellationOutcome::Cancelled)
            | Ok(RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Cancelled)) => {}
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) => {
                if !status.is_terminal() {
                    self.runs.insert(root_task_id, root_run);
                } else {
                    self.fail_active_task(
                        &root_task_id,
                        AgentTaskFailureCode::RuntimeStateMismatch,
                    )?;
                }
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
            Err(error) => {
                self.runs.insert(root_task_id, root_run);
                return Err(AgentOrchestratorError::Runtime(error));
            }
        }
        self.tasks
            .get_mut(&root_task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .wait_for_child()?;
        self.selected_workflow = Some(AgentWorkflowSelection::BoundedParallel);
        self.child_created = true;
        self.bounded_parallel = Some(state);
        let result = self.admit_ready_parallel_slots()?;
        Ok(BoundedParallelWorkflowAcceptance::new(result))
    }

    #[must_use]
    pub fn bounded_parallel_status(&self) -> Option<BoundedParallelWorkflowStatus> {
        self.bounded_parallel.as_ref().map(|state| state.status)
    }

    pub(super) fn bounded_parallel_cancellation_pending(&self) -> bool {
        self.bounded_parallel
            .as_ref()
            .is_some_and(|state| state.status == BoundedParallelWorkflowStatus::Cancelling)
    }

    pub(super) fn bounded_parallel_rejected_cleanup_pending(&self) -> bool {
        self.bounded_parallel.as_ref().is_some_and(|state| {
            state.cancellation.as_ref().is_some_and(|cursor| {
                matches!(
                    cursor.mode,
                    ParallelCancellationMode::RejectedChildStart
                        | ParallelCancellationMode::RejectedSynthesisStart
                )
            })
        })
    }

    pub(super) fn bounded_parallel_tracks_task(&self, task_id: &AgentTaskId) -> bool {
        self.bounded_parallel.as_ref().is_some_and(|state| {
            self.root_task_id
                .as_ref()
                .is_some_and(|root_id| state.tracks(root_id, task_id))
        })
    }

    #[must_use]
    pub fn bounded_parallel_result(&self) -> Option<&BoundedParallelWorkflowResult> {
        self.bounded_parallel
            .as_ref()
            .and_then(|state| state.result.as_ref())
    }

    /// Returns the canonical catalog-ordinal outcome projection once every
    /// planned specialist slot is terminal.
    ///
    /// This remains available when root cancellation, deadline expiry, or an
    /// invalid Personal synthesis prevents construction of a full workflow
    /// result, so successful findings and non-success statuses are not hidden.
    #[must_use]
    pub fn bounded_parallel_outcomes(&self) -> Option<Vec<ParallelChildOutcome>> {
        self.bounded_parallel
            .as_ref()
            .and_then(BoundedParallelWorkflowState::outcomes)
    }

    #[must_use]
    pub fn bounded_parallel_events(&self) -> &[BoundedParallelWorkflowEvent] {
        self.bounded_parallel
            .as_ref()
            .map_or(&[], |state| state.events.as_slice())
    }

    #[must_use]
    pub fn bounded_parallel_audit_records(&self) -> &[BoundedParallelAuditRecord] {
        self.bounded_parallel
            .as_ref()
            .map_or(&[], |state| state.audit.as_slice())
    }

    pub fn bounded_parallel_active_children(
        &mut self,
    ) -> AgentOrchestratorResult<Vec<BoundedParallelChildControl>> {
        self.enforce_bounded_parallel_control_deadline()?;
        let state = self
            .bounded_parallel
            .as_ref()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        if state.status == BoundedParallelWorkflowStatus::Cancelling {
            return Err(AgentOrchestratorError::BoundedParallelCancellationPending);
        }
        state
            .slots
            .iter()
            .filter_map(|slot| match &slot.state {
                ParallelSlotState::Running {
                    task_id, context, ..
                } => Some((slot, task_id, context)),
                ParallelSlotState::Pending | ParallelSlotState::Terminal(_) => None,
            })
            .map(|(slot, task_id, context)| {
                let active = self
                    .runs
                    .get(task_id)
                    .ok_or(AgentOrchestratorError::NoActiveRun)?;
                Ok(BoundedParallelChildControl::new(
                    slot.item.id().clone(),
                    slot.item.ordinal(),
                    context.clone(),
                    ParallelChildCancellationHandle::new(
                        self.workflow_sequence,
                        context.root_task_id().clone(),
                        task_id.clone(),
                        slot.item.id().clone(),
                        active.run.identity().clone(),
                    ),
                ))
            })
            .collect()
    }

    pub fn cancel_bounded_parallel_child(
        &mut self,
        handle: ParallelChildCancellationHandle,
    ) -> AgentOrchestratorResult<AgentTaskCancellationOutcome> {
        let (workflow_sequence, root_id, task_id, work_item_id, run_identity) = handle.parts();
        if workflow_sequence != self.workflow_sequence
            || self.root_task_id.as_ref() != Some(root_id.task_id())
        {
            return Err(AgentOrchestratorError::BoundedParallelCancellationHandleMismatch);
        }
        let state = self
            .bounded_parallel
            .as_ref()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        let ordinal = state
            .ordinal_for_task(task_id)
            .ok_or(AgentOrchestratorError::BoundedParallelCancellationHandleMismatch)?;
        if state.slot(ordinal)?.item.id() != work_item_id
            || self
                .runs
                .get(task_id)
                .is_none_or(|active| active.run.identity() != run_identity)
        {
            return Err(AgentOrchestratorError::BoundedParallelCancellationHandleMismatch);
        }
        self.enforce_bounded_parallel_deadline_before_ingress(task_id)?;
        self.cancel_bounded_parallel_child_task(
            task_id,
            ParallelCancellationReason::UserRequested,
            false,
        )
    }

    fn planned_parallel_attribution(
        &self,
        root_task_id: &AgentTaskId,
        request: &BoundedParallelWorkflowRequest,
        item: &ParallelWorkItem,
    ) -> AgentOrchestratorResult<BoundedParallelAttribution> {
        let definition = self.registry.get(item.agent_id())?;
        Ok(BoundedParallelAttribution::planned(
            RootTaskId::from_task_id(root_task_id.clone()),
            request.scenario_id(),
            item,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
        ))
    }

    fn push_bounded_parallel_transition(
        &self,
        state: &mut BoundedParallelWorkflowState,
        event: BoundedParallelWorkflowEvent,
        attribution: BoundedParallelAttribution,
        outcome: BoundedParallelAuditOutcome,
    ) -> AgentOrchestratorResult<()> {
        if state.events.len() >= MAX_PARALLEL_WORKFLOW_EVENTS
            || state.audit.len() >= MAX_PARALLEL_AUDIT_RECORDS
        {
            return Err(AgentOrchestratorError::BoundedParallelJournalLimitExceeded);
        }
        let sequence = u8::try_from(state.audit.len() + 1)
            .map_err(|_| AgentOrchestratorError::BoundedParallelJournalLimitExceeded)?;
        state.events.push(event);
        state.audit.push(BoundedParallelAuditRecord::new(
            sequence,
            attribution,
            outcome,
        ));
        Ok(())
    }

    fn admit_ready_parallel_slots(
        &mut self,
    ) -> AgentOrchestratorResult<Vec<AgentExecutionContext>> {
        loop {
            let next = {
                let state = self
                    .bounded_parallel
                    .as_ref()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                if state.status != BoundedParallelWorkflowStatus::Running
                    || state.active_count() >= usize::from(state.request.active_limit())
                {
                    None
                } else {
                    state
                        .slots
                        .iter()
                        .find(|slot| {
                            matches!(slot.state, ParallelSlotState::Pending)
                                && state.dependencies_succeeded(&slot.item)
                        })
                        .map(|slot| slot.item.ordinal())
                }
            };
            let Some(ordinal) = next else { break };
            let _ = self.attempt_parallel_slot_start(ordinal)?;
        }
        let state = self
            .bounded_parallel
            .as_ref()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        Ok(state
            .slots
            .iter()
            .filter_map(|slot| match &slot.state {
                ParallelSlotState::Running {
                    task_id, context, ..
                } if self.runs.contains_key(task_id) => Some(context.clone()),
                ParallelSlotState::Pending
                | ParallelSlotState::Running { .. }
                | ParallelSlotState::Terminal(_) => None,
            })
            .collect())
    }

    fn attempt_parallel_slot_start(
        &mut self,
        ordinal: u8,
    ) -> AgentOrchestratorResult<Option<AgentExecutionContext>> {
        let now = self.workflow_now();
        let (request, item, root_deadline, predecessors) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            let item = state.slot(ordinal)?.item.clone();
            let predecessors = item
                .dependencies()
                .iter()
                .map(|dependency| {
                    state
                        .slots
                        .iter()
                        .find(|slot| slot.item.id() == dependency)
                        .and_then(|slot| match &slot.state {
                            ParallelSlotState::Terminal(outcome) => Some(outcome.clone()),
                            ParallelSlotState::Pending | ParallelSlotState::Running { .. } => None,
                        })
                        .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)
                })
                .collect::<AgentOrchestratorResult<Vec<_>>>()?;
            (
                state.request.clone(),
                item,
                state.root_deadline,
                predecessors,
            )
        };
        if now >= root_deadline {
            self.expire_bounded_parallel_root()?;
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        let input = request.build_child_input(ordinal, &predecessors)?;
        let root_task_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        if !item.dependencies().is_empty() {
            let attribution = self.planned_parallel_attribution(&root_task_id, &request, &item)?;
            let mut state = self
                .bounded_parallel
                .take()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            for dependency in item.dependencies() {
                self.push_bounded_parallel_transition(
                    &mut state,
                    BoundedParallelWorkflowEvent::DependencySatisfied {
                        predecessor: dependency.clone(),
                        dependent: item.id().clone(),
                    },
                    attribution.clone(),
                    BoundedParallelAuditOutcome::DependencySatisfied,
                )?;
            }
            self.bounded_parallel = Some(state);
        }
        let task_id = AgentTaskId::new(format!(
            "agent-task-child-{}-{ordinal}",
            self.workflow_sequence
        ))?;
        let (definition_identity, policy_profile_id, memory_profile_id) = {
            let definition = self.registry.get(item.agent_id())?;
            (
                definition.identity(),
                definition.policy_profile_id(),
                definition.memory_profile_id(),
            )
        };
        let mut task = AgentTask::new_child(
            task_id.clone(),
            RootTaskId::from_task_id(root_task_id.clone()),
            ParentTaskId::from_task_id(root_task_id.clone()),
            definition_identity,
            AgentTaskObjective::new(item.objective().to_owned())?,
            Some(AgentTaskContext::new(
                "Use only the sealed D-091 fixture input and return no reasoning",
            )?),
            AgentTaskExpectedDeliverable::new(item.expected_output().to_owned())?,
        )?;
        let run_ordinal = self
            .run_count
            .checked_add(1)
            .ok_or(AgentOrchestratorError::RunLimitExceeded)?;
        let runtime_request = self.runtime_request(&task_id, run_ordinal, &input)?;
        self.ensure_event_capacity(3)?;
        self.tasks.insert(task_id.clone(), task.clone());
        self.events.push(AgentOrchestrationEvent::ChildCreated {
            task_id: task_id.clone(),
            parent_task_id: root_task_id.clone(),
        });
        self.run_count = run_ordinal;
        {
            let state = self
                .bounded_parallel
                .as_mut()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            state.task_to_ordinal.insert(task_id.clone(), ordinal);
        }
        let started = self.start_runtime_run(runtime_request);
        match started {
            Ok(mut run) => {
                let started_at = self.workflow_now();
                if started_at >= root_deadline {
                    match run.cancel() {
                        Ok(RuntimeCancellationOutcome::Cancelled)
                        | Ok(RuntimeCancellationOutcome::AlreadyTerminal(
                            RuntimeRunStatus::Cancelled,
                        )) => {
                            self.terminalize_parallel_start_timeout(
                                ordinal,
                                &task_id,
                                ParallelTimeoutReason::RootDeadlineExceeded,
                            )?;
                            self.expire_bounded_parallel_root()?;
                            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
                        }
                        Ok(RuntimeCancellationOutcome::AlreadyTerminal(status))
                            if status.is_terminal() =>
                        {
                            self.terminalize_parallel_start_failure(
                                ordinal,
                                &task_id,
                                &item,
                                request.scenario_id(),
                                AgentTaskFailureCode::RuntimeStateMismatch,
                            )?;
                            self.expire_bounded_parallel_root()?;
                            return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
                        }
                        Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) => {
                            task.start()?;
                            let context = AgentExecutionContext::for_task(
                                &task,
                                self.runtime_id,
                                run.identity().clone(),
                            );
                            self.tasks.insert(task_id.clone(), task);
                            self.runs.insert(
                                task_id.clone(),
                                ActiveRun {
                                    run,
                                    output: String::new(),
                                    next_sequence: 0,
                                },
                            );
                            self.events.push(AgentOrchestrationEvent::ChildStarted {
                                task_id: task_id.clone(),
                            });
                            let mut state = self
                                .bounded_parallel
                                .take()
                                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                            state.slot_mut(ordinal)?.state = ParallelSlotState::Running {
                                task_id: task_id.clone(),
                                context: context.clone(),
                                deadline: root_deadline,
                                progress_recorded: false,
                            };
                            self.push_bounded_parallel_transition(
                                &mut state,
                                BoundedParallelWorkflowEvent::Started {
                                    work_item_id: item.id().clone(),
                                    ordinal,
                                    task_id: task_id.clone(),
                                },
                                BoundedParallelAttribution::live(
                                    RootTaskId::from_task_id(root_task_id.clone()),
                                    request.scenario_id(),
                                    &item,
                                    policy_profile_id,
                                    memory_profile_id,
                                    &context,
                                ),
                                BoundedParallelAuditOutcome::Started,
                            )?;
                            self.bounded_parallel = Some(state);
                            let active = self.parallel_active_task_ids();
                            self.begin_bounded_parallel_cancellation(
                                ParallelCancellationMode::RootExpired,
                                active,
                            )?;
                            return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
                        }
                        Err(error) => {
                            task.start()?;
                            let context = AgentExecutionContext::for_task(
                                &task,
                                self.runtime_id,
                                run.identity().clone(),
                            );
                            self.tasks.insert(task_id.clone(), task);
                            self.runs.insert(
                                task_id.clone(),
                                ActiveRun {
                                    run,
                                    output: String::new(),
                                    next_sequence: 0,
                                },
                            );
                            self.events.push(AgentOrchestrationEvent::ChildStarted {
                                task_id: task_id.clone(),
                            });
                            let mut state = self
                                .bounded_parallel
                                .take()
                                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                            state.slot_mut(ordinal)?.state = ParallelSlotState::Running {
                                task_id: task_id.clone(),
                                context: context.clone(),
                                deadline: root_deadline,
                                progress_recorded: false,
                            };
                            self.push_bounded_parallel_transition(
                                &mut state,
                                BoundedParallelWorkflowEvent::Started {
                                    work_item_id: item.id().clone(),
                                    ordinal,
                                    task_id: task_id.clone(),
                                },
                                BoundedParallelAttribution::live(
                                    RootTaskId::from_task_id(root_task_id.clone()),
                                    request.scenario_id(),
                                    &item,
                                    policy_profile_id,
                                    memory_profile_id,
                                    &context,
                                ),
                                BoundedParallelAuditOutcome::Started,
                            )?;
                            self.bounded_parallel = Some(state);
                            let active = self.parallel_active_task_ids();
                            self.begin_bounded_parallel_cancellation(
                                ParallelCancellationMode::RootExpired,
                                active,
                            )?;
                            return Err(AgentOrchestratorError::Runtime(error));
                        }
                    }
                }
                task.start()?;
                let context =
                    AgentExecutionContext::for_task(&task, self.runtime_id, run.identity().clone());
                let child_deadline = started_at
                    .checked_add(std::time::Duration::from_secs(
                        MAX_PARALLEL_CHILD_DURATION_SECONDS,
                    ))
                    .map_or(root_deadline, |deadline| deadline.min(root_deadline));
                self.tasks.insert(task_id.clone(), task);
                self.runs.insert(
                    task_id.clone(),
                    ActiveRun {
                        run,
                        output: String::new(),
                        next_sequence: 0,
                    },
                );
                self.events.push(AgentOrchestrationEvent::ChildStarted {
                    task_id: task_id.clone(),
                });
                let live_attribution = BoundedParallelAttribution::live(
                    RootTaskId::from_task_id(root_task_id),
                    request.scenario_id(),
                    &item,
                    policy_profile_id,
                    memory_profile_id,
                    &context,
                );
                let mut state = self
                    .bounded_parallel
                    .take()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                state.slot_mut(ordinal)?.state = ParallelSlotState::Running {
                    task_id: task_id.clone(),
                    context: context.clone(),
                    deadline: child_deadline,
                    progress_recorded: false,
                };
                self.push_bounded_parallel_transition(
                    &mut state,
                    BoundedParallelWorkflowEvent::Started {
                        work_item_id: item.id().clone(),
                        ordinal,
                        task_id,
                    },
                    live_attribution,
                    BoundedParallelAuditOutcome::Started,
                )?;
                self.bounded_parallel = Some(state);
                Ok(Some(context))
            }
            Err(error) => {
                self.terminalize_parallel_start_failure(
                    ordinal,
                    &task_id,
                    &item,
                    request.scenario_id(),
                    AgentTaskFailureCode::RuntimeStartFailed,
                )?;
                if error == AgentOrchestratorError::RuntimeCleanupPending {
                    self.begin_bounded_parallel_rejected_cleanup(
                        ParallelCancellationMode::RejectedChildStart,
                        Some(ordinal),
                    )?;
                    return Err(error);
                }
                self.advance_bounded_parallel_after_terminal(ordinal)?;
                Ok(None)
            }
        }
    }

    fn terminalize_parallel_start_failure(
        &mut self,
        ordinal: u8,
        task_id: &AgentTaskId,
        item: &ParallelWorkItem,
        scenario_id: BoundedParallelScenarioId,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        self.tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                task_id.clone(),
                item.agent_id(),
                code,
            ))?;
        self.events.push(AgentOrchestrationEvent::ChildFailed {
            task_id: task_id.clone(),
            code,
        });
        let root_task_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: task_id.clone(),
            parent_task_id: root_task_id.clone(),
            outcome: AgentTaskOutcomeKind::Failed,
        });
        self.cleanup_terminal_task(task_id);
        let definition = self.registry.get(item.agent_id())?;
        let attempt = BoundedParallelAttribution::start_attempt(
            RootTaskId::from_task_id(root_task_id),
            scenario_id,
            item,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            task_id.clone(),
            self.runtime_id,
        );
        let outcome = ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::Failed(code),
        );
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(outcome);
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::StartAttemptFailed {
                work_item_id: item.id().clone(),
                ordinal,
                task_id: task_id.clone(),
            },
            attempt,
            BoundedParallelAuditOutcome::StartAttemptFailed,
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn terminalize_parallel_start_timeout(
        &mut self,
        ordinal: u8,
        task_id: &AgentTaskId,
        reason: ParallelTimeoutReason,
    ) -> AgentOrchestratorResult<()> {
        let (item, root_id) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            (
                state.slot(ordinal)?.item.clone(),
                self.root_task_id
                    .clone()
                    .ok_or(AgentOrchestratorError::RootMissing)?,
            )
        };
        self.tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                task_id.clone(),
                item.agent_id(),
                AgentTaskFailureCode::DeadlineExceeded,
            ))?;
        self.events.push(AgentOrchestrationEvent::ChildFailed {
            task_id: task_id.clone(),
            code: AgentTaskFailureCode::DeadlineExceeded,
        });
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: task_id.clone(),
            parent_task_id: root_id.clone(),
            outcome: AgentTaskOutcomeKind::Failed,
        });
        self.cleanup_terminal_task(task_id);
        let outcome = ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::TimedOut(reason),
        );
        let definition = self.registry.get(item.agent_id())?;
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(outcome);
        let scenario_id = state.request.scenario_id();
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::TimedOut {
                work_item_id: item.id().clone(),
                ordinal,
                reason,
            },
            BoundedParallelAttribution::start_attempt(
                RootTaskId::from_task_id(root_id),
                scenario_id,
                &item,
                definition.policy_profile_id(),
                definition.memory_profile_id(),
                task_id.clone(),
                self.runtime_id,
            ),
            BoundedParallelAuditOutcome::TimedOut,
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    pub(super) fn preflight_bounded_parallel_event_capacity(
        &self,
        task_id: &AgentTaskId,
        event: &UntrustedRuntimeEvent,
    ) -> AgentOrchestratorResult<()> {
        let Some(state) = self.bounded_parallel.as_ref() else {
            return Ok(());
        };
        if !state.tracks(
            self.root_task_id
                .as_ref()
                .ok_or(AgentOrchestratorError::RootMissing)?,
            task_id,
        ) {
            return Ok(());
        }
        if state.status == BoundedParallelWorkflowStatus::Cancelling {
            return Err(AgentOrchestratorError::BoundedParallelCancellationPending);
        }
        let accepted = state.accepted_events.get(task_id).copied().unwrap_or(0);
        let terminal = matches!(
            event,
            UntrustedRuntimeEvent::ResponseCompleted | UntrustedRuntimeEvent::ResponseFailed { .. }
        );
        let limit = if terminal {
            MAX_PARALLEL_EVENTS_PER_RUN
        } else {
            MAX_PARALLEL_EVENTS_PER_RUN.saturating_sub(1)
        };
        if accepted >= limit {
            return Err(AgentOrchestratorError::RuntimeEventLimitExceeded);
        }
        if let UntrustedRuntimeEvent::OutputTextDelta { delta, .. } = event {
            let active = self
                .runs
                .get(task_id)
                .ok_or(AgentOrchestratorError::NoActiveRun)?;
            let (max_characters, max_bytes) = if state.ordinal_for_task(task_id).is_some() {
                (
                    MAX_PARALLEL_RAW_RESULT_CHARACTERS,
                    MAX_PARALLEL_RAW_RESULT_BYTES,
                )
            } else {
                (
                    MAX_PARALLEL_SYNTHESIS_OUTPUT_CHARACTERS,
                    MAX_PARALLEL_SYNTHESIS_OUTPUT_BYTES,
                )
            };
            let next_bytes = active
                .output
                .len()
                .checked_add(delta.as_str().len())
                .ok_or(AgentOrchestratorError::OutputLimitExceeded)?;
            let next_characters = active
                .output
                .chars()
                .count()
                .checked_add(delta.as_str().chars().count())
                .ok_or(AgentOrchestratorError::OutputLimitExceeded)?;
            if next_bytes > max_bytes || next_characters > max_characters {
                return Err(AgentOrchestratorError::OutputLimitExceeded);
            }
        }
        let remaining = if state.ordinal_for_task(task_id).is_some() {
            state
                .slots
                .iter()
                .map(|slot| match &slot.state {
                    ParallelSlotState::Terminal(_) => 0,
                    ParallelSlotState::Running {
                        progress_recorded, ..
                    } => usize::from(!progress_recorded) + 1,
                    ParallelSlotState::Pending => slot.item.dependencies().len() + 3,
                })
                .sum::<usize>()
                .saturating_add(5)
        } else {
            usize::from(!state.synthesis_progress_recorded) + 2
        };
        if state.events.len().saturating_add(remaining) > MAX_PARALLEL_WORKFLOW_EVENTS
            || state.audit.len().saturating_add(remaining) > MAX_PARALLEL_AUDIT_RECORDS
        {
            return Err(AgentOrchestratorError::BoundedParallelJournalLimitExceeded);
        }
        Ok(())
    }

    pub(super) fn prepare_bounded_parallel_terminal(
        &self,
        task_id: &AgentTaskId,
        event: &UntrustedRuntimeEvent,
    ) -> AgentOrchestratorResult<Option<PreparedBoundedParallelTerminal>> {
        let Some(state) = self.bounded_parallel.as_ref() else {
            return Ok(None);
        };
        let root_id = self
            .root_task_id
            .as_ref()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        if !state.tracks(root_id, task_id) {
            return Ok(None);
        }
        let active = self
            .runs
            .get(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        if let Some(ordinal) = state.ordinal_for_task(task_id) {
            let item = state.slot(ordinal)?.item.clone();
            return match event {
                UntrustedRuntimeEvent::ResponseCompleted => {
                    let (output, disposition) = match AgentTaskOutput::new(active.output.clone()) {
                        Ok(output) => {
                            match state.request.parse_child_result(ordinal, output.as_str()) {
                                Ok(result) => {
                                    (Some(output), ParallelChildDisposition::Succeeded(result))
                                }
                                Err(_) => (
                                    None,
                                    ParallelChildDisposition::Failed(
                                        AgentTaskFailureCode::RuntimeOutputInvalid,
                                    ),
                                ),
                            }
                        }
                        Err(_) => (
                            None,
                            ParallelChildDisposition::Failed(
                                AgentTaskFailureCode::RuntimeOutputInvalid,
                            ),
                        ),
                    };
                    self.preflight_bounded_parallel_next_after_child(ordinal, &disposition)?;
                    Ok(Some(PreparedBoundedParallelTerminal::Child {
                        ordinal,
                        output,
                        disposition,
                    }))
                }
                UntrustedRuntimeEvent::ResponseFailed { failure } => {
                    let disposition = if failure.code() == RuntimeFailureCode::Cancelled {
                        ParallelChildDisposition::Cancelled(
                            ParallelCancellationReason::RuntimeReported,
                        )
                    } else {
                        ParallelChildDisposition::Failed(AgentTaskFailureCode::RuntimeReported(
                            failure.code(),
                        ))
                    };
                    self.preflight_bounded_parallel_next_after_child(ordinal, &disposition)?;
                    let _ = item;
                    Ok(Some(PreparedBoundedParallelTerminal::Child {
                        ordinal,
                        output: None,
                        disposition,
                    }))
                }
                _ => Ok(None),
            };
        }
        if root_id == task_id && state.status == BoundedParallelWorkflowStatus::Synthesizing {
            return match event {
                UntrustedRuntimeEvent::ResponseCompleted => {
                    let Ok(output) = AgentTaskOutput::new(active.output.clone()) else {
                        return Ok(Some(PreparedBoundedParallelTerminal::SynthesisFailed {
                            code: AgentTaskFailureCode::RuntimeOutputInvalid,
                        }));
                    };
                    let outcomes = state
                        .outcomes()
                        .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                    match state.request.parse_synthesis(&outcomes, output.as_str()) {
                        Ok(synthesis) => {
                            Ok(Some(PreparedBoundedParallelTerminal::SynthesisCompleted {
                                output,
                                synthesis,
                                outcomes,
                            }))
                        }
                        Err(_) => Ok(Some(PreparedBoundedParallelTerminal::SynthesisFailed {
                            code: AgentTaskFailureCode::RuntimeOutputInvalid,
                        })),
                    }
                }
                UntrustedRuntimeEvent::ResponseFailed { failure } => {
                    Ok(Some(PreparedBoundedParallelTerminal::SynthesisFailed {
                        code: AgentTaskFailureCode::RuntimeReported(failure.code()),
                    }))
                }
                _ => Ok(None),
            };
        }
        Err(AgentOrchestratorError::BoundedParallelStageMismatch)
    }

    fn preflight_bounded_parallel_next_after_child(
        &self,
        ordinal: u8,
        disposition: &ParallelChildDisposition,
    ) -> AgentOrchestratorResult<()> {
        let state = self
            .bounded_parallel
            .as_ref()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        let mut projected = state
            .slots
            .iter()
            .map(|slot| match &slot.state {
                ParallelSlotState::Terminal(outcome) => Some(outcome.clone()),
                ParallelSlotState::Pending | ParallelSlotState::Running { .. } => None,
            })
            .collect::<Vec<_>>();
        let item = state.slot(ordinal)?.item.clone();
        projected[usize::from(ordinal - 1)] = Some(ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            disposition.clone(),
        ));

        if state.request.failure_policy() == BoundedParallelFailurePolicy::FailFast
            && disposition.status() != ParallelChildResultStatus::Succeeded
        {
            for slot in &state.slots {
                let index = usize::from(slot.item.ordinal() - 1);
                if matches!(slot.state, ParallelSlotState::Pending) {
                    projected[index] = Some(ParallelChildOutcome::new(
                        slot.item.ordinal(),
                        slot.item.id().clone(),
                        slot.item.agent_id(),
                        ParallelChildDisposition::Skipped(ParallelSkipReason::FailFast),
                    ));
                }
            }
        } else {
            loop {
                let mut changed = false;
                for slot in &state.slots {
                    let index = usize::from(slot.item.ordinal() - 1);
                    if projected[index].is_some()
                        || !matches!(slot.state, ParallelSlotState::Pending)
                        || slot.item.dependencies().is_empty()
                    {
                        continue;
                    }
                    let dependencies = slot
                        .item
                        .dependencies()
                        .iter()
                        .map(|dependency| {
                            state
                                .slots
                                .iter()
                                .position(|candidate| candidate.item.id() == dependency)
                                .and_then(|dependency_index| projected[dependency_index].as_ref())
                        })
                        .collect::<Vec<_>>();
                    if dependencies.iter().all(|outcome| outcome.is_some())
                        && dependencies.iter().any(|outcome| {
                            outcome.is_some_and(|value| {
                                value.status() != ParallelChildResultStatus::Succeeded
                            })
                        })
                    {
                        projected[index] = Some(ParallelChildOutcome::new(
                            slot.item.ordinal(),
                            slot.item.id().clone(),
                            slot.item.agent_id(),
                            ParallelChildDisposition::Skipped(
                                ParallelSkipReason::DependencyUnavailable,
                            ),
                        ));
                        changed = true;
                    }
                }
                if !changed {
                    break;
                }
            }
        }

        for slot in &state.slots {
            let index = usize::from(slot.item.ordinal() - 1);
            if projected[index].is_some()
                || !matches!(slot.state, ParallelSlotState::Pending)
                || slot.item.dependencies().is_empty()
            {
                continue;
            }
            let dependency_indexes = slot
                .item
                .dependencies()
                .iter()
                .map(|dependency| {
                    state
                        .slots
                        .iter()
                        .position(|candidate| candidate.item.id() == dependency)
                        .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)
                })
                .collect::<AgentOrchestratorResult<Vec<_>>>()?;
            if dependency_indexes
                .iter()
                .any(|dependency_index| projected[*dependency_index].is_none())
            {
                continue;
            }
            let predecessors = slot
                .item
                .dependencies()
                .iter()
                .zip(dependency_indexes)
                .map(|(_dependency, dependency_index)| {
                    projected[dependency_index]
                        .clone()
                        .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)
                })
                .collect::<AgentOrchestratorResult<Vec<_>>>()?;
            if predecessors
                .iter()
                .all(|outcome| outcome.status() == ParallelChildResultStatus::Succeeded)
            {
                state
                    .request
                    .build_child_input(slot.item.ordinal(), &predecessors)?;
                if self.run_count >= MAX_PARALLEL_RUNTIME_RUNS_PER_ROOT {
                    return Err(AgentOrchestratorError::RunLimitExceeded);
                }
            }
        }

        if projected.iter().all(Option::is_some) {
            let outcomes = projected
                .into_iter()
                .collect::<Option<Vec<_>>>()
                .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
            state.request.build_synthesis_input(&outcomes)?;
            if self.run_count >= MAX_PARALLEL_RUNTIME_RUNS_PER_ROOT {
                return Err(AgentOrchestratorError::RunLimitExceeded);
            }
        }
        Ok(())
    }

    pub(super) fn record_bounded_parallel_event_acceptance(
        &mut self,
        task_id: &AgentTaskId,
        accepted: &RuntimeEventAcceptance,
    ) -> AgentOrchestratorResult<()> {
        let Some(mut state) = self.bounded_parallel.take() else {
            return Ok(());
        };
        if !state.tracks(
            self.root_task_id
                .as_ref()
                .ok_or(AgentOrchestratorError::RootMissing)?,
            task_id,
        ) {
            self.bounded_parallel = Some(state);
            return Ok(());
        }
        let accepted_events = state.accepted_events.entry(task_id.clone()).or_default();
        *accepted_events = accepted_events
            .checked_add(1)
            .ok_or(AgentOrchestratorError::RuntimeEventLimitExceeded)?;
        if matches!(accepted, RuntimeEventAcceptance::OutputTextDelta { .. }) {
            if let Some(ordinal) = state.ordinal_for_task(task_id) {
                let item = state.slot(ordinal)?.item.clone();
                let (item, context, should_record) = match &mut state.slot_mut(ordinal)?.state {
                    ParallelSlotState::Running {
                        context,
                        progress_recorded,
                        ..
                    } => {
                        let should_record = !*progress_recorded;
                        *progress_recorded = true;
                        (item, context.clone(), should_record)
                    }
                    _ => return Err(AgentOrchestratorError::BoundedParallelStageMismatch),
                };
                if should_record {
                    let definition = self.registry.get(item.agent_id())?;
                    let attribution = BoundedParallelAttribution::live(
                        context.root_task_id().clone(),
                        state.request.scenario_id(),
                        &item,
                        definition.policy_profile_id(),
                        definition.memory_profile_id(),
                        &context,
                    );
                    self.push_bounded_parallel_transition(
                        &mut state,
                        BoundedParallelWorkflowEvent::Progress {
                            work_item_id: item.id().clone(),
                            ordinal,
                        },
                        attribution,
                        BoundedParallelAuditOutcome::Progress,
                    )?;
                }
            } else if !state.synthesis_progress_recorded {
                state.synthesis_progress_recorded = true;
                let context = self.current_context(task_id)?;
                let attribution = BoundedParallelAttribution::personal_root_live(
                    state.request.scenario_id(),
                    &context,
                )?;
                self.push_bounded_parallel_transition(
                    &mut state,
                    BoundedParallelWorkflowEvent::SynthesisProgress,
                    attribution,
                    BoundedParallelAuditOutcome::SynthesisProgress,
                )?;
            }
        }
        self.bounded_parallel = Some(state);
        Ok(())
    }

    pub(super) fn apply_prepared_bounded_parallel_terminal(
        &mut self,
        task_id: &AgentTaskId,
        prepared: PreparedBoundedParallelTerminal,
    ) -> AgentOrchestratorResult<()> {
        match prepared {
            PreparedBoundedParallelTerminal::Child {
                ordinal,
                output,
                disposition,
            } => self.apply_bounded_parallel_child_terminal(task_id, ordinal, output, disposition),
            PreparedBoundedParallelTerminal::SynthesisCompleted {
                output,
                synthesis,
                outcomes,
            } => self
                .apply_bounded_parallel_synthesis_completed(task_id, output, synthesis, outcomes),
            PreparedBoundedParallelTerminal::SynthesisFailed { code } => {
                self.fail_bounded_parallel_synthesis(task_id, code)
            }
        }
    }

    fn apply_bounded_parallel_child_terminal(
        &mut self,
        task_id: &AgentTaskId,
        ordinal: u8,
        output: Option<AgentTaskOutput>,
        disposition: ParallelChildDisposition,
    ) -> AgentOrchestratorResult<()> {
        let (item, context, root_id) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            let slot = state.slot(ordinal)?;
            let ParallelSlotState::Running { context, .. } = &slot.state else {
                return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
            };
            (
                slot.item.clone(),
                context.clone(),
                self.root_task_id
                    .clone()
                    .ok_or(AgentOrchestratorError::RootMissing)?,
            )
        };
        self.runs
            .remove(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        let task = self
            .tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        let outcome_kind = match &disposition {
            ParallelChildDisposition::Succeeded(_) => {
                task.complete(AgentTaskResult::new(
                    task_id.clone(),
                    item.agent_id(),
                    output.ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?,
                ))?;
                self.events.push(AgentOrchestrationEvent::ChildCompleted {
                    task_id: task_id.clone(),
                });
                AgentTaskOutcomeKind::Completed
            }
            ParallelChildDisposition::Failed(code) => {
                task.fail(AgentTaskFailure::new(
                    task_id.clone(),
                    item.agent_id(),
                    *code,
                ))?;
                self.events.push(AgentOrchestrationEvent::ChildFailed {
                    task_id: task_id.clone(),
                    code: *code,
                });
                AgentTaskOutcomeKind::Failed
            }
            ParallelChildDisposition::Cancelled(_) => {
                let outcome = task.cancel();
                if outcome != AgentTaskCancellationOutcome::Cancelled {
                    return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
                }
                self.events.push(AgentOrchestrationEvent::TaskCancelled {
                    task_id: task_id.clone(),
                    agent_id: item.agent_id(),
                });
                AgentTaskOutcomeKind::Cancelled
            }
            ParallelChildDisposition::TimedOut(_) | ParallelChildDisposition::Skipped(_) => {
                return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
            }
        };
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: task_id.clone(),
            parent_task_id: root_id,
            outcome: outcome_kind,
        });
        self.cleanup_terminal_task(task_id);
        let definition = self.registry.get(item.agent_id())?;
        let attribution = BoundedParallelAttribution::live(
            context.root_task_id().clone(),
            self.bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?
                .request
                .scenario_id(),
            &item,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            &context,
        );
        let outcome = ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            disposition.clone(),
        );
        let (event, audit_outcome) = match disposition {
            ParallelChildDisposition::Succeeded(_) => (
                BoundedParallelWorkflowEvent::Completed {
                    work_item_id: item.id().clone(),
                    ordinal,
                    task_id: task_id.clone(),
                },
                BoundedParallelAuditOutcome::Completed,
            ),
            ParallelChildDisposition::Failed(code) => (
                BoundedParallelWorkflowEvent::Failed {
                    work_item_id: item.id().clone(),
                    ordinal,
                    code,
                },
                BoundedParallelAuditOutcome::Failed,
            ),
            ParallelChildDisposition::Cancelled(reason) => (
                BoundedParallelWorkflowEvent::Cancelled {
                    work_item_id: item.id().clone(),
                    ordinal,
                    reason,
                },
                BoundedParallelAuditOutcome::Cancelled,
            ),
            _ => return Err(AgentOrchestratorError::BoundedParallelStageMismatch),
        };
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(outcome);
        self.push_bounded_parallel_transition(&mut state, event, attribution, audit_outcome)?;
        self.bounded_parallel = Some(state);
        self.advance_bounded_parallel_after_terminal(ordinal)
    }

    fn advance_bounded_parallel_after_terminal(
        &mut self,
        ordinal: u8,
    ) -> AgentOrchestratorResult<()> {
        let (policy, status, terminal_status) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            (
                state.request.failure_policy(),
                state.status,
                state.slot(ordinal)?.state.clone(),
            )
        };
        if status == BoundedParallelWorkflowStatus::Cancelling {
            return Ok(());
        }
        let terminal_succeeded = matches!(
            terminal_status,
            ParallelSlotState::Terminal(ref outcome)
                if outcome.status() == ParallelChildResultStatus::Succeeded
        );
        if policy == BoundedParallelFailurePolicy::FailFast && !terminal_succeeded {
            self.mark_pending_parallel_slots_skipped(ParallelSkipReason::FailFast)?;
            let active = self.parallel_active_task_ids();
            if !active.is_empty() {
                self.begin_bounded_parallel_cancellation(
                    ParallelCancellationMode::FailFast,
                    active,
                )?;
                return self.resume_bounded_parallel_cancellation();
            }
        }

        self.skip_unavailable_parallel_dependents()?;
        self.admit_ready_parallel_slots()?;
        let should_synthesize = self.bounded_parallel.as_ref().is_some_and(|state| {
            state.status == BoundedParallelWorkflowStatus::Running
                && state.all_terminal()
                && state.active_count() == 0
        });
        if should_synthesize {
            self.start_bounded_parallel_synthesis()?;
        }
        Ok(())
    }

    fn skip_unavailable_parallel_dependents(&mut self) -> AgentOrchestratorResult<()> {
        let ordinals = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            state
                .slots
                .iter()
                .filter(|slot| {
                    matches!(slot.state, ParallelSlotState::Pending)
                        && !slot.item.dependencies().is_empty()
                        && state.dependencies_terminal(&slot.item)
                        && !state.dependencies_succeeded(&slot.item)
                })
                .map(|slot| slot.item.ordinal())
                .collect::<Vec<_>>()
        };
        for ordinal in ordinals {
            self.mark_parallel_slot_skipped(ordinal, ParallelSkipReason::DependencyUnavailable)?;
        }
        Ok(())
    }

    fn mark_pending_parallel_slots_skipped(
        &mut self,
        reason: ParallelSkipReason,
    ) -> AgentOrchestratorResult<()> {
        let ordinals = self
            .bounded_parallel
            .as_ref()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?
            .slots
            .iter()
            .filter(|slot| matches!(slot.state, ParallelSlotState::Pending))
            .map(|slot| slot.item.ordinal())
            .collect::<Vec<_>>();
        for ordinal in ordinals {
            self.mark_parallel_slot_skipped(ordinal, reason)?;
        }
        Ok(())
    }

    fn mark_parallel_slot_skipped(
        &mut self,
        ordinal: u8,
        reason: ParallelSkipReason,
    ) -> AgentOrchestratorResult<()> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        let item = state.slot(ordinal)?.item.clone();
        if !matches!(state.slot(ordinal)?.state, ParallelSlotState::Pending) {
            self.bounded_parallel = Some(state);
            return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
        }
        let attribution = self.planned_parallel_attribution(&root_id, &state.request, &item)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::Skipped(reason),
        ));
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Skipped {
                work_item_id: item.id().clone(),
                ordinal,
                reason,
            },
            attribution,
            BoundedParallelAuditOutcome::Skipped,
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn start_bounded_parallel_synthesis(&mut self) -> AgentOrchestratorResult<()> {
        let now = self.workflow_now();
        let (request, outcomes, root_deadline) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            (
                state.request.clone(),
                state
                    .outcomes()
                    .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?,
                state.root_deadline,
            )
        };
        if now >= root_deadline {
            self.expire_bounded_parallel_root()?;
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        if self.run_count >= MAX_PARALLEL_RUNTIME_RUNS_PER_ROOT {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        let input = request.build_synthesis_input(&outcomes)?;
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let run_ordinal = self
            .run_count
            .checked_add(1)
            .ok_or(AgentOrchestratorError::RunLimitExceeded)?;
        let runtime_request = self.runtime_request(&root_id, run_ordinal, &input)?;
        self.ensure_event_capacity(2)?;
        let root = self
            .tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        if root.status() != AgentTaskStatus::WaitingForChild {
            return Err(AgentOrchestratorError::InvalidTaskState {
                status: root.status(),
            });
        }
        root.resume_from_child()?;
        self.events.push(AgentOrchestrationEvent::ParentResumed {
            task_id: root_id.clone(),
        });
        self.run_count = run_ordinal;
        match self.start_runtime_run(runtime_request) {
            Ok(run) => {
                let context = {
                    let root = self
                        .tasks
                        .get(&root_id)
                        .ok_or(AgentOrchestratorError::TaskNotFound)?;
                    AgentExecutionContext::for_task(root, self.runtime_id, run.identity().clone())
                };
                self.runs.insert(
                    root_id.clone(),
                    ActiveRun {
                        run,
                        output: String::new(),
                        next_sequence: 0,
                    },
                );
                let attribution = BoundedParallelAttribution::personal_root_live(
                    request.scenario_id(),
                    &context,
                )?;
                let mut state = self
                    .bounded_parallel
                    .take()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                state.status = BoundedParallelWorkflowStatus::Synthesizing;
                self.push_bounded_parallel_transition(
                    &mut state,
                    BoundedParallelWorkflowEvent::ParentResumed {
                        task_id: root_id.clone(),
                    },
                    attribution.clone(),
                    BoundedParallelAuditOutcome::ParentResumed,
                )?;
                self.push_bounded_parallel_transition(
                    &mut state,
                    BoundedParallelWorkflowEvent::SynthesisStarted {
                        task_id: root_id.clone(),
                    },
                    attribution,
                    BoundedParallelAuditOutcome::SynthesisStarted,
                )?;
                self.bounded_parallel = Some(state);
                if self.workflow_now() >= root_deadline {
                    self.expire_bounded_parallel_root()?;
                    return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
                }
                Ok(())
            }
            Err(error) => {
                if error == AgentOrchestratorError::RuntimeCleanupPending {
                    self.begin_bounded_parallel_rejected_cleanup(
                        ParallelCancellationMode::RejectedSynthesisStart,
                        None,
                    )?;
                    return Err(error);
                }
                self.terminalize_bounded_parallel_synthesis_start_failure()?;
                Err(error)
            }
        }
    }

    fn terminalize_bounded_parallel_synthesis_start_failure(
        &mut self,
    ) -> AgentOrchestratorResult<()> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let scenario_id = self
            .bounded_parallel
            .as_ref()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?
            .request
            .scenario_id();
        let root = self
            .tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        root.fail(AgentTaskFailure::new(
            root_id.clone(),
            AgentId::PersonalAssistant,
            AgentTaskFailureCode::RuntimeStartFailed,
        ))?;
        self.events.push(AgentOrchestrationEvent::RootFailed {
            task_id: root_id.clone(),
            code: AgentTaskFailureCode::RuntimeStartFailed,
        });
        self.cleanup_terminal_task(&root_id);
        let definition = self.registry.get(AgentId::PersonalAssistant)?;
        let attribution = BoundedParallelAttribution::personal_root_start_attempt(
            RootTaskId::from_task_id(root_id.clone()),
            scenario_id,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            root_id,
            self.runtime_id,
        )?;
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.status = BoundedParallelWorkflowStatus::Failed;
        state.cancellation = None;
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Terminal {
                status: BoundedParallelTerminalStatus::Failed,
            },
            attribution,
            BoundedParallelAuditOutcome::Terminal(BoundedParallelTerminalStatus::Failed),
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn apply_bounded_parallel_synthesis_completed(
        &mut self,
        task_id: &AgentTaskId,
        output: AgentTaskOutput,
        synthesis: crate::agent::bounded_parallelism::BoundedParallelSynthesis,
        outcomes: Vec<ParallelChildOutcome>,
    ) -> AgentOrchestratorResult<()> {
        let active = self
            .runs
            .remove(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        let context = {
            let root = self
                .tasks
                .get_mut(task_id)
                .ok_or(AgentOrchestratorError::TaskNotFound)?;
            let context = active.context(root, self.runtime_id);
            root.complete(AgentTaskResult::new(
                task_id.clone(),
                AgentId::PersonalAssistant,
                output,
            ))?;
            context
        };
        self.events.push(AgentOrchestrationEvent::RootCompleted {
            task_id: task_id.clone(),
        });
        self.cleanup_terminal_task(task_id);
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        let terminal = match synthesis.status() {
            crate::agent::bounded_parallelism::BoundedParallelSynthesisStatus::Complete => {
                state.status = BoundedParallelWorkflowStatus::Completed;
                BoundedParallelTerminalStatus::Completed
            }
            crate::agent::bounded_parallelism::BoundedParallelSynthesisStatus::Partial => {
                state.status = BoundedParallelWorkflowStatus::Partial;
                BoundedParallelTerminalStatus::Partial
            }
        };
        state.result = Some(BoundedParallelWorkflowResult::new(
            RootTaskId::from_task_id(task_id.clone()),
            state.request.scenario_id(),
            state.request.failure_policy(),
            outcomes,
            synthesis,
        ));
        let attribution =
            BoundedParallelAttribution::personal_root_live(state.request.scenario_id(), &context)?;
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::SynthesisCompleted {
                task_id: task_id.clone(),
            },
            attribution.clone(),
            BoundedParallelAuditOutcome::SynthesisCompleted,
        )?;
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Terminal { status: terminal },
            attribution,
            BoundedParallelAuditOutcome::Terminal(terminal),
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn fail_bounded_parallel_synthesis(
        &mut self,
        task_id: &AgentTaskId,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let active = self
            .runs
            .remove(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        let root = self
            .tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        let context = active.context(root, self.runtime_id);
        root.fail(AgentTaskFailure::new(
            task_id.clone(),
            AgentId::PersonalAssistant,
            code,
        ))?;
        self.events.push(AgentOrchestrationEvent::RootFailed {
            task_id: task_id.clone(),
            code,
        });
        self.cleanup_terminal_task(task_id);
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.status = BoundedParallelWorkflowStatus::Failed;
        let attribution =
            BoundedParallelAttribution::personal_root_live(state.request.scenario_id(), &context)?;
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Terminal {
                status: BoundedParallelTerminalStatus::Failed,
            },
            attribution,
            BoundedParallelAuditOutcome::Terminal(BoundedParallelTerminalStatus::Failed),
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    pub(super) fn fail_bounded_parallel_task(
        &mut self,
        task_id: &AgentTaskId,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let (is_root, ordinal) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            (
                self.root_task_id.as_ref() == Some(task_id)
                    && state.status == BoundedParallelWorkflowStatus::Synthesizing,
                state.ordinal_for_task(task_id),
            )
        };
        if is_root {
            let status = self
                .runs
                .get(task_id)
                .map(|active| active.run.status())
                .ok_or(AgentOrchestratorError::NoActiveRun)?;
            if status.is_terminal() {
                let active = self
                    .runs
                    .remove(task_id)
                    .ok_or(AgentOrchestratorError::NoActiveRun)?;
                let context = active.context(
                    self.tasks
                        .get(task_id)
                        .ok_or(AgentOrchestratorError::TaskNotFound)?,
                    self.runtime_id,
                );
                return self.fail_bounded_parallel_synthesis_without_run(task_id, code, context);
            }
            self.begin_bounded_parallel_forced_synthesis_failure(code)?;
            return self.resume_bounded_parallel_cancellation();
        }
        let ordinal = ordinal.ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
        let status = self
            .runs
            .get(task_id)
            .map(|active| active.run.status())
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        if status.is_terminal() {
            self.runs.remove(task_id);
            self.terminalize_bounded_parallel_child_failure(task_id, code)?;
            return self.advance_bounded_parallel_after_terminal(ordinal);
        }
        self.begin_bounded_parallel_forced_failure(ordinal, code)?;
        self.resume_bounded_parallel_cancellation()
    }

    fn fail_bounded_parallel_synthesis_without_run(
        &mut self,
        task_id: &AgentTaskId,
        code: AgentTaskFailureCode,
        context: AgentExecutionContext,
    ) -> AgentOrchestratorResult<()> {
        self.tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                task_id.clone(),
                AgentId::PersonalAssistant,
                code,
            ))?;
        self.events.push(AgentOrchestrationEvent::RootFailed {
            task_id: task_id.clone(),
            code,
        });
        self.cleanup_terminal_task(task_id);
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.status = BoundedParallelWorkflowStatus::Failed;
        state.cancellation = None;
        let attribution =
            BoundedParallelAttribution::personal_root_live(state.request.scenario_id(), &context)?;
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Terminal {
                status: BoundedParallelTerminalStatus::Failed,
            },
            attribution,
            BoundedParallelAuditOutcome::Terminal(BoundedParallelTerminalStatus::Failed),
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn parallel_active_task_ids(&self) -> Vec<u8> {
        self.bounded_parallel
            .as_ref()
            .map(|state| {
                state
                    .slots
                    .iter()
                    .filter(|slot| matches!(slot.state, ParallelSlotState::Running { .. }))
                    .map(|slot| slot.item.ordinal())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn begin_bounded_parallel_cancellation(
        &mut self,
        mode: ParallelCancellationMode,
        ordinals: Vec<u8>,
    ) -> AgentOrchestratorResult<()> {
        let state = self
            .bounded_parallel
            .as_mut()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.status = BoundedParallelWorkflowStatus::Cancelling;
        state.cancellation = Some(ParallelCancellationCursor {
            mode,
            ordinals,
            next: 0,
            failure_code: None,
        });
        Ok(())
    }

    fn begin_bounded_parallel_forced_failure(
        &mut self,
        ordinal: u8,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let state = self
            .bounded_parallel
            .as_mut()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.status = BoundedParallelWorkflowStatus::Cancelling;
        state.cancellation = Some(ParallelCancellationCursor {
            mode: ParallelCancellationMode::ForcedFailure,
            ordinals: vec![ordinal],
            next: 0,
            failure_code: Some(code),
        });
        Ok(())
    }

    fn begin_bounded_parallel_forced_synthesis_failure(
        &mut self,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let state = self
            .bounded_parallel
            .as_mut()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.status = BoundedParallelWorkflowStatus::Cancelling;
        state.cancellation = Some(ParallelCancellationCursor {
            mode: ParallelCancellationMode::ForcedSynthesisFailure,
            ordinals: Vec::new(),
            next: 0,
            failure_code: Some(code),
        });
        Ok(())
    }

    fn begin_bounded_parallel_rejected_cleanup(
        &mut self,
        mode: ParallelCancellationMode,
        ordinal: Option<u8>,
    ) -> AgentOrchestratorResult<()> {
        if !matches!(
            mode,
            ParallelCancellationMode::RejectedChildStart
                | ParallelCancellationMode::RejectedSynthesisStart
        ) {
            return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
        }
        let state = self
            .bounded_parallel
            .as_mut()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.status = BoundedParallelWorkflowStatus::Cancelling;
        state.cancellation = Some(ParallelCancellationCursor {
            mode,
            ordinals: ordinal.into_iter().collect(),
            next: 0,
            failure_code: Some(AgentTaskFailureCode::RuntimeStartFailed),
        });
        Ok(())
    }

    pub(super) fn resume_bounded_parallel_cancellation(&mut self) -> AgentOrchestratorResult<()> {
        if self.bounded_parallel_rejected_cleanup_pending()
            && !self.rejected_runtime_runs.is_empty()
        {
            return Err(AgentOrchestratorError::RuntimeCleanupPending);
        }
        loop {
            let next = {
                let state = self
                    .bounded_parallel
                    .as_ref()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                let cursor = state
                    .cancellation
                    .as_ref()
                    .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                cursor
                    .ordinals
                    .get(cursor.next)
                    .copied()
                    .map(|ordinal| (cursor.mode, ordinal))
            };
            let Some((mode, ordinal)) = next else { break };
            let task_id = {
                let state = self
                    .bounded_parallel
                    .as_ref()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                match &state.slot(ordinal)?.state {
                    ParallelSlotState::Running { task_id, .. } => task_id.clone(),
                    ParallelSlotState::Terminal(_) => {
                        let state = self
                            .bounded_parallel
                            .as_mut()
                            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                        let cursor = state
                            .cancellation
                            .as_mut()
                            .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                        cursor.next = cursor
                            .next
                            .checked_add(1)
                            .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                        continue;
                    }
                    ParallelSlotState::Pending => {
                        return Err(AgentOrchestratorError::BoundedParallelStageMismatch)
                    }
                }
            };
            let result = match mode {
                ParallelCancellationMode::Root => self
                    .cancel_bounded_parallel_child_task(
                        &task_id,
                        ParallelCancellationReason::RootCancelled,
                        true,
                    )
                    .map(|_| ()),
                ParallelCancellationMode::RootExpired | ParallelCancellationMode::ChildTimeout => {
                    let reason = if mode == ParallelCancellationMode::RootExpired {
                        ParallelTimeoutReason::RootDeadlineExceeded
                    } else {
                        ParallelTimeoutReason::ChildDeadlineExceeded
                    };
                    self.timeout_bounded_parallel_child_task(&task_id, reason, true)
                        .map(|_| ())
                }
                ParallelCancellationMode::FailFast => self
                    .cancel_bounded_parallel_child_task(
                        &task_id,
                        ParallelCancellationReason::FailFast,
                        true,
                    )
                    .map(|_| ()),
                ParallelCancellationMode::ForcedFailure => {
                    let code = self
                        .bounded_parallel
                        .as_ref()
                        .and_then(|state| state.cancellation.as_ref())
                        .and_then(|cursor| cursor.failure_code)
                        .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                    self.cancel_then_fail_bounded_parallel_child(&task_id, code)
                }
                ParallelCancellationMode::ForcedSynthesisFailure => {
                    return Err(AgentOrchestratorError::BoundedParallelStageMismatch)
                }
                ParallelCancellationMode::RejectedChildStart
                | ParallelCancellationMode::RejectedSynthesisStart => {
                    return Err(AgentOrchestratorError::BoundedParallelStageMismatch)
                }
            };
            result?;
            let state = self
                .bounded_parallel
                .as_mut()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            let cursor = state
                .cancellation
                .as_mut()
                .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
            cursor.next = cursor
                .next
                .checked_add(1)
                .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
        }

        let (mode, last_ordinal) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            let cursor = state
                .cancellation
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
            (cursor.mode, cursor.ordinals.last().copied())
        };
        match mode {
            ParallelCancellationMode::Root => self.finalize_bounded_parallel_root_cancel(),
            ParallelCancellationMode::RootExpired => self.finalize_bounded_parallel_root_expiry(),
            ParallelCancellationMode::FailFast => {
                let state = self
                    .bounded_parallel
                    .as_mut()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                state.cancellation = None;
                state.status = BoundedParallelWorkflowStatus::Running;
                if state.all_terminal() {
                    self.start_bounded_parallel_synthesis()
                } else {
                    Err(AgentOrchestratorError::BoundedParallelStageMismatch)
                }
            }
            ParallelCancellationMode::ChildTimeout => {
                let state = self
                    .bounded_parallel
                    .as_mut()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                state.cancellation = None;
                state.status = BoundedParallelWorkflowStatus::Running;
                self.advance_bounded_parallel_after_terminal(
                    last_ordinal.ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?,
                )
            }
            ParallelCancellationMode::ForcedFailure => {
                let state = self
                    .bounded_parallel
                    .as_mut()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                state.cancellation = None;
                state.status = BoundedParallelWorkflowStatus::Running;
                self.advance_bounded_parallel_after_terminal(
                    last_ordinal.ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?,
                )
            }
            ParallelCancellationMode::ForcedSynthesisFailure => {
                let code = self
                    .bounded_parallel
                    .as_ref()
                    .and_then(|state| state.cancellation.as_ref())
                    .and_then(|cursor| cursor.failure_code)
                    .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                self.cancel_then_fail_bounded_parallel_synthesis(code)
            }
            ParallelCancellationMode::RejectedChildStart => {
                let state = self
                    .bounded_parallel
                    .as_mut()
                    .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                state.cancellation = None;
                state.status = BoundedParallelWorkflowStatus::Running;
                self.advance_bounded_parallel_after_terminal(
                    last_ordinal.ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?,
                )
            }
            ParallelCancellationMode::RejectedSynthesisStart => {
                self.terminalize_bounded_parallel_synthesis_start_failure()
            }
        }
    }

    fn cancel_then_fail_bounded_parallel_child(
        &mut self,
        task_id: &AgentTaskId,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        self.cancel_pending_governance(task_id)?;
        match self.cancel_run(task_id)? {
            RunCancellationDisposition::Cancelled => {}
            RunCancellationDisposition::UnexpectedTerminal(status) => {
                let (ordinal, item, context) = {
                    let state = self
                        .bounded_parallel
                        .as_ref()
                        .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
                    let ordinal = state
                        .ordinal_for_task(task_id)
                        .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                    let slot = state.slot(ordinal)?;
                    let ParallelSlotState::Running { context, .. } = &slot.state else {
                        return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
                    };
                    (ordinal, slot.item.clone(), context.clone())
                };
                self.terminalize_parallel_cancel_mismatch(task_id, ordinal, &item, &context)?;
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
        }
        self.terminalize_bounded_parallel_child_failure(task_id, code)
    }

    fn terminalize_bounded_parallel_child_failure(
        &mut self,
        task_id: &AgentTaskId,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let (ordinal, item, context, root_id) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            let ordinal = state
                .ordinal_for_task(task_id)
                .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
            let slot = state.slot(ordinal)?;
            let ParallelSlotState::Running { context, .. } = &slot.state else {
                return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
            };
            (
                ordinal,
                slot.item.clone(),
                context.clone(),
                self.root_task_id
                    .clone()
                    .ok_or(AgentOrchestratorError::RootMissing)?,
            )
        };
        self.tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                task_id.clone(),
                item.agent_id(),
                code,
            ))?;
        self.events.push(AgentOrchestrationEvent::ChildFailed {
            task_id: task_id.clone(),
            code,
        });
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: task_id.clone(),
            parent_task_id: root_id,
            outcome: AgentTaskOutcomeKind::Failed,
        });
        self.cleanup_terminal_task(task_id);
        let definition = self.registry.get(item.agent_id())?;
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::Failed(code),
        ));
        let attribution = BoundedParallelAttribution::live(
            context.root_task_id().clone(),
            state.request.scenario_id(),
            &item,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            &context,
        );
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Failed {
                work_item_id: item.id().clone(),
                ordinal,
                code,
            },
            attribution,
            BoundedParallelAuditOutcome::Failed,
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn cancel_then_fail_bounded_parallel_synthesis(
        &mut self,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let task_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let context = {
            let active = self
                .runs
                .get(&task_id)
                .ok_or(AgentOrchestratorError::NoActiveRun)?;
            active.context(
                self.tasks
                    .get(&task_id)
                    .ok_or(AgentOrchestratorError::TaskNotFound)?,
                self.runtime_id,
            )
        };
        self.cancel_pending_governance(&task_id)?;
        match self.cancel_run(&task_id)? {
            RunCancellationDisposition::Cancelled => {}
            RunCancellationDisposition::UnexpectedTerminal(status) => {
                self.fail_bounded_parallel_synthesis_without_run(
                    &task_id,
                    AgentTaskFailureCode::RuntimeStateMismatch,
                    context,
                )?;
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
        }
        self.fail_bounded_parallel_synthesis_without_run(&task_id, code, context)
    }

    pub(super) fn cancel_bounded_parallel_child_task(
        &mut self,
        task_id: &AgentTaskId,
        reason: ParallelCancellationReason,
        root_is_cancelling: bool,
    ) -> AgentOrchestratorResult<AgentTaskCancellationOutcome> {
        if !root_is_cancelling
            && self
                .bounded_parallel
                .as_ref()
                .is_some_and(|state| state.status == BoundedParallelWorkflowStatus::Cancelling)
        {
            return Err(AgentOrchestratorError::BoundedParallelCancellationPending);
        }
        let (ordinal, item, context, root_id) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            let ordinal = state
                .ordinal_for_task(task_id)
                .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
            let slot = state.slot(ordinal)?;
            let ParallelSlotState::Running { context, .. } = &slot.state else {
                return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
            };
            (
                ordinal,
                slot.item.clone(),
                context.clone(),
                self.root_task_id
                    .clone()
                    .ok_or(AgentOrchestratorError::RootMissing)?,
            )
        };
        self.cancel_pending_governance(task_id)?;
        match self.cancel_run(task_id)? {
            RunCancellationDisposition::Cancelled => {}
            RunCancellationDisposition::UnexpectedTerminal(status) => {
                self.terminalize_parallel_cancel_mismatch(task_id, ordinal, &item, &context)?;
                if !root_is_cancelling {
                    self.advance_bounded_parallel_after_terminal(ordinal)?;
                }
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
        }
        let task = self
            .tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        let outcome = task.cancel();
        if outcome != AgentTaskCancellationOutcome::Cancelled {
            return Ok(outcome);
        }
        self.events.push(AgentOrchestrationEvent::TaskCancelled {
            task_id: task_id.clone(),
            agent_id: item.agent_id(),
        });
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: task_id.clone(),
            parent_task_id: root_id,
            outcome: AgentTaskOutcomeKind::Cancelled,
        });
        self.cleanup_terminal_task(task_id);
        let definition = self.registry.get(item.agent_id())?;
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::Cancelled(reason),
        ));
        let attribution = BoundedParallelAttribution::live(
            context.root_task_id().clone(),
            state.request.scenario_id(),
            &item,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            &context,
        );
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Cancelled {
                work_item_id: item.id().clone(),
                ordinal,
                reason,
            },
            attribution,
            BoundedParallelAuditOutcome::Cancelled,
        )?;
        self.bounded_parallel = Some(state);
        if !root_is_cancelling {
            self.advance_bounded_parallel_after_terminal(ordinal)?;
        }
        Ok(outcome)
    }

    fn timeout_bounded_parallel_child_task(
        &mut self,
        task_id: &AgentTaskId,
        reason: ParallelTimeoutReason,
        root_is_cancelling: bool,
    ) -> AgentOrchestratorResult<AgentTaskCancellationOutcome> {
        let (ordinal, item, context, root_id) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            let ordinal = state
                .ordinal_for_task(task_id)
                .ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
            let slot = state.slot(ordinal)?;
            let ParallelSlotState::Running { context, .. } = &slot.state else {
                return Err(AgentOrchestratorError::BoundedParallelStageMismatch);
            };
            (
                ordinal,
                slot.item.clone(),
                context.clone(),
                self.root_task_id
                    .clone()
                    .ok_or(AgentOrchestratorError::RootMissing)?,
            )
        };
        self.cancel_pending_governance(task_id)?;
        match self.cancel_run(task_id)? {
            RunCancellationDisposition::Cancelled => {}
            RunCancellationDisposition::UnexpectedTerminal(status) => {
                self.terminalize_parallel_cancel_mismatch(task_id, ordinal, &item, &context)?;
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
        }
        self.tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                task_id.clone(),
                item.agent_id(),
                AgentTaskFailureCode::DeadlineExceeded,
            ))?;
        self.events.push(AgentOrchestrationEvent::ChildFailed {
            task_id: task_id.clone(),
            code: AgentTaskFailureCode::DeadlineExceeded,
        });
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: task_id.clone(),
            parent_task_id: root_id,
            outcome: AgentTaskOutcomeKind::Failed,
        });
        self.cleanup_terminal_task(task_id);
        let definition = self.registry.get(item.agent_id())?;
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::TimedOut(reason),
        ));
        let attribution = BoundedParallelAttribution::live(
            context.root_task_id().clone(),
            state.request.scenario_id(),
            &item,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            &context,
        );
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::TimedOut {
                work_item_id: item.id().clone(),
                ordinal,
                reason,
            },
            attribution,
            BoundedParallelAuditOutcome::TimedOut,
        )?;
        self.bounded_parallel = Some(state);
        if !root_is_cancelling {
            self.advance_bounded_parallel_after_terminal(ordinal)?;
        }
        Ok(AgentTaskCancellationOutcome::Cancelled)
    }

    fn terminalize_parallel_cancel_mismatch(
        &mut self,
        task_id: &AgentTaskId,
        ordinal: u8,
        item: &ParallelWorkItem,
        context: &AgentExecutionContext,
    ) -> AgentOrchestratorResult<()> {
        let code = AgentTaskFailureCode::RuntimeStateMismatch;
        self.tasks
            .get_mut(task_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                task_id.clone(),
                item.agent_id(),
                code,
            ))?;
        self.events.push(AgentOrchestrationEvent::ChildFailed {
            task_id: task_id.clone(),
            code,
        });
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: task_id.clone(),
            parent_task_id: root_id,
            outcome: AgentTaskOutcomeKind::Failed,
        });
        self.cleanup_terminal_task(task_id);
        let definition = self.registry.get(item.agent_id())?;
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        state.slot_mut(ordinal)?.state = ParallelSlotState::Terminal(ParallelChildOutcome::new(
            ordinal,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::Failed(code),
        ));
        let attribution = BoundedParallelAttribution::live(
            context.root_task_id().clone(),
            state.request.scenario_id(),
            item,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            context,
        );
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Failed {
                work_item_id: item.id().clone(),
                ordinal,
                code,
            },
            attribution,
            BoundedParallelAuditOutcome::Failed,
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    pub(super) fn cancel_bounded_parallel_root(
        &mut self,
        root_task_id: &AgentTaskId,
    ) -> AgentOrchestratorResult<AgentTaskCancellationOutcome> {
        if self.root_task_id.as_ref() != Some(root_task_id) {
            return Err(AgentOrchestratorError::ContextMismatch);
        }
        if self.bounded_parallel_rejected_cleanup_pending() {
            self.cleanup_rejected_runtime_runs()?;
        }
        let pending_mode = self
            .bounded_parallel
            .as_ref()
            .and_then(|state| state.cancellation.as_ref().map(|cursor| cursor.mode));
        if pending_mode.is_some_and(|mode| {
            !matches!(
                mode,
                ParallelCancellationMode::Root | ParallelCancellationMode::RootExpired
            )
        }) {
            let now = self.workflow_now();
            let root_deadline = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?
                .root_deadline;
            if now >= root_deadline {
                self.mark_pending_parallel_slots_skipped(ParallelSkipReason::RootDeadlineExpired)?;
                let active = self.parallel_active_task_ids();
                self.begin_bounded_parallel_cancellation(
                    ParallelCancellationMode::RootExpired,
                    active,
                )?;
                self.resume_bounded_parallel_cancellation()?;
                return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
            }
        }
        if let Some(mode) = pending_mode {
            match mode {
                ParallelCancellationMode::Root => {
                    self.resume_bounded_parallel_cancellation()?;
                    return Ok(AgentTaskCancellationOutcome::Cancelled);
                }
                ParallelCancellationMode::RootExpired => {
                    self.resume_bounded_parallel_cancellation()?;
                    return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
                }
                ParallelCancellationMode::FailFast
                | ParallelCancellationMode::ChildTimeout
                | ParallelCancellationMode::ForcedFailure
                | ParallelCancellationMode::ForcedSynthesisFailure
                | ParallelCancellationMode::RejectedChildStart
                | ParallelCancellationMode::RejectedSynthesisStart => {
                    self.mark_pending_parallel_slots_skipped(ParallelSkipReason::RootCancelled)?;
                    let active = self.parallel_active_task_ids();
                    self.begin_bounded_parallel_cancellation(
                        ParallelCancellationMode::Root,
                        active,
                    )?;
                    self.resume_bounded_parallel_cancellation()?;
                    return Ok(AgentTaskCancellationOutcome::Cancelled);
                }
            }
        }
        if self
            .tasks
            .get(root_task_id)
            .is_some_and(|task| task.status().is_terminal())
        {
            return Ok(AgentTaskCancellationOutcome::AlreadyTerminal(
                self.tasks
                    .get(root_task_id)
                    .ok_or(AgentOrchestratorError::TaskNotFound)?
                    .status(),
            ));
        }
        self.mark_pending_parallel_slots_skipped(ParallelSkipReason::RootCancelled)?;
        let active = self.parallel_active_task_ids();
        self.begin_bounded_parallel_cancellation(ParallelCancellationMode::Root, active)?;
        self.resume_bounded_parallel_cancellation()?;
        Ok(AgentTaskCancellationOutcome::Cancelled)
    }

    pub(super) fn enforce_bounded_parallel_root_cancel_deadline_precedence(
        &mut self,
    ) -> AgentOrchestratorResult<()> {
        let pending_mode = self
            .bounded_parallel
            .as_ref()
            .and_then(|state| state.cancellation.as_ref().map(|cursor| cursor.mode));
        if pending_mode.is_some_and(|mode| {
            !matches!(
                mode,
                ParallelCancellationMode::Root | ParallelCancellationMode::RootExpired
            )
        }) {
            let now = self.workflow_now();
            let root_deadline = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?
                .root_deadline;
            if now >= root_deadline {
                self.mark_pending_parallel_slots_skipped(ParallelSkipReason::RootDeadlineExpired)?;
                let active = self.parallel_active_task_ids();
                self.begin_bounded_parallel_cancellation(
                    ParallelCancellationMode::RootExpired,
                    active,
                )?;
                self.resume_bounded_parallel_cancellation()?;
                return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
            }
        }
        Ok(())
    }

    fn finalize_bounded_parallel_root_cancel(&mut self) -> AgentOrchestratorResult<()> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        self.cancel_pending_governance(&root_id)?;
        let live_context = self.runs.get(&root_id).and_then(|active| {
            self.tasks
                .get(&root_id)
                .map(|task| active.context(task, self.runtime_id))
        });
        match self.cancel_run(&root_id)? {
            RunCancellationDisposition::Cancelled => {}
            RunCancellationDisposition::UnexpectedTerminal(status) => {
                let context =
                    live_context.ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                self.fail_bounded_parallel_synthesis_without_run(
                    &root_id,
                    AgentTaskFailureCode::RuntimeStateMismatch,
                    context,
                )?;
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
        }
        let root = self
            .tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        let outcome = root.cancel();
        if outcome == AgentTaskCancellationOutcome::Cancelled {
            self.events.push(AgentOrchestrationEvent::TaskCancelled {
                task_id: root_id.clone(),
                agent_id: AgentId::PersonalAssistant,
            });
            self.cleanup_terminal_task(&root_id);
        }
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        let attribution = self.parallel_root_attribution(
            state.request.scenario_id(),
            &root_id,
            live_context.as_ref(),
        )?;
        state.status = BoundedParallelWorkflowStatus::Cancelled;
        state.cancellation = None;
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Terminal {
                status: BoundedParallelTerminalStatus::Cancelled,
            },
            attribution,
            BoundedParallelAuditOutcome::Terminal(BoundedParallelTerminalStatus::Cancelled),
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn finalize_bounded_parallel_root_expiry(&mut self) -> AgentOrchestratorResult<()> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        self.cancel_pending_governance(&root_id)?;
        let live_context = self.runs.get(&root_id).and_then(|active| {
            self.tasks
                .get(&root_id)
                .map(|task| active.context(task, self.runtime_id))
        });
        match self.cancel_run(&root_id)? {
            RunCancellationDisposition::Cancelled => {}
            RunCancellationDisposition::UnexpectedTerminal(status) => {
                let context =
                    live_context.ok_or(AgentOrchestratorError::BoundedParallelStageMismatch)?;
                self.fail_bounded_parallel_synthesis_without_run(
                    &root_id,
                    AgentTaskFailureCode::RuntimeStateMismatch,
                    context,
                )?;
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
        }
        self.tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                root_id.clone(),
                AgentId::PersonalAssistant,
                AgentTaskFailureCode::DeadlineExceeded,
            ))?;
        self.events.push(AgentOrchestrationEvent::RootFailed {
            task_id: root_id.clone(),
            code: AgentTaskFailureCode::DeadlineExceeded,
        });
        self.cleanup_terminal_task(&root_id);
        let mut state = self
            .bounded_parallel
            .take()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        let attribution = self.parallel_root_attribution(
            state.request.scenario_id(),
            &root_id,
            live_context.as_ref(),
        )?;
        state.status = BoundedParallelWorkflowStatus::Failed;
        state.cancellation = None;
        self.push_bounded_parallel_transition(
            &mut state,
            BoundedParallelWorkflowEvent::Terminal {
                status: BoundedParallelTerminalStatus::Failed,
            },
            attribution,
            BoundedParallelAuditOutcome::Terminal(BoundedParallelTerminalStatus::Failed),
        )?;
        self.bounded_parallel = Some(state);
        Ok(())
    }

    fn parallel_root_attribution(
        &self,
        scenario_id: BoundedParallelScenarioId,
        root_id: &AgentTaskId,
        live_context: Option<&AgentExecutionContext>,
    ) -> AgentOrchestratorResult<BoundedParallelAttribution> {
        if let Some(context) = live_context {
            return Ok(BoundedParallelAttribution::personal_root_live(
                scenario_id,
                context,
            )?);
        }
        let definition = self.registry.get(AgentId::PersonalAssistant)?;
        Ok(BoundedParallelAttribution::personal_root_waiting(
            RootTaskId::from_task_id(root_id.clone()),
            scenario_id,
            definition.policy_profile_id(),
            definition.memory_profile_id(),
            root_id.clone(),
        )?)
    }

    pub(super) fn expire_bounded_parallel_root(&mut self) -> AgentOrchestratorResult<()> {
        if self.bounded_parallel_rejected_cleanup_pending() {
            self.cleanup_rejected_runtime_runs()?;
            self.mark_pending_parallel_slots_skipped(ParallelSkipReason::RootDeadlineExpired)?;
            let active = self.parallel_active_task_ids();
            self.begin_bounded_parallel_cancellation(
                ParallelCancellationMode::RootExpired,
                active,
            )?;
            return self.resume_bounded_parallel_cancellation();
        }
        let status = self
            .bounded_parallel
            .as_ref()
            .map(|state| state.status)
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
        match status {
            BoundedParallelWorkflowStatus::Cancelling => {
                return self.resume_bounded_parallel_cancellation()
            }
            BoundedParallelWorkflowStatus::Completed
            | BoundedParallelWorkflowStatus::Partial
            | BoundedParallelWorkflowStatus::Failed
            | BoundedParallelWorkflowStatus::Cancelled => return Ok(()),
            BoundedParallelWorkflowStatus::Running
            | BoundedParallelWorkflowStatus::Synthesizing => {}
        }
        self.mark_pending_parallel_slots_skipped(ParallelSkipReason::RootDeadlineExpired)?;
        let active = self.parallel_active_task_ids();
        self.begin_bounded_parallel_cancellation(ParallelCancellationMode::RootExpired, active)?;
        self.resume_bounded_parallel_cancellation()
    }

    pub fn check_bounded_parallel_deadlines(
        &mut self,
    ) -> AgentOrchestratorResult<BoundedParallelWorkflowStatus> {
        let now = self.workflow_now();
        let (status, root_deadline, expired_child) = {
            let state = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            (
                state.status,
                state.root_deadline,
                state.slots.iter().find_map(|slot| match &slot.state {
                    ParallelSlotState::Running {
                        task_id, deadline, ..
                    } if now >= *deadline => Some((slot.item.ordinal(), task_id.clone())),
                    _ => None,
                }),
            )
        };
        if status == BoundedParallelWorkflowStatus::Cancelling {
            self.resume_bounded_parallel_cancellation()?;
            let resumed_now = self.workflow_now();
            let resumed = self
                .bounded_parallel
                .as_ref()
                .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)?;
            if matches!(
                resumed.status,
                BoundedParallelWorkflowStatus::Running
                    | BoundedParallelWorkflowStatus::Synthesizing
            ) && resumed_now >= resumed.root_deadline
            {
                self.expire_bounded_parallel_root()?;
            } else if resumed.status == BoundedParallelWorkflowStatus::Running {
                if let Some(ordinal) = resumed.slots.iter().find_map(|slot| match &slot.state {
                    ParallelSlotState::Running { deadline, .. } if resumed_now >= *deadline => {
                        Some(slot.item.ordinal())
                    }
                    ParallelSlotState::Pending
                    | ParallelSlotState::Running { .. }
                    | ParallelSlotState::Terminal(_) => None,
                }) {
                    self.begin_bounded_parallel_cancellation(
                        ParallelCancellationMode::ChildTimeout,
                        vec![ordinal],
                    )?;
                    self.resume_bounded_parallel_cancellation()?;
                }
            }
        } else if matches!(
            status,
            BoundedParallelWorkflowStatus::Running | BoundedParallelWorkflowStatus::Synthesizing
        ) && now >= root_deadline
        {
            self.expire_bounded_parallel_root()?;
        } else if let Some((ordinal, _task_id)) = expired_child {
            self.begin_bounded_parallel_cancellation(
                ParallelCancellationMode::ChildTimeout,
                vec![ordinal],
            )?;
            self.resume_bounded_parallel_cancellation()?;
        }
        self.bounded_parallel_status()
            .ok_or(AgentOrchestratorError::BoundedParallelWorkflowMissing)
    }

    fn enforce_bounded_parallel_control_deadline(&mut self) -> AgentOrchestratorResult<()> {
        let Some(state) = self.bounded_parallel.as_ref() else {
            return Ok(());
        };
        match state.status {
            BoundedParallelWorkflowStatus::Cancelling => {
                return Err(AgentOrchestratorError::BoundedParallelCancellationPending)
            }
            BoundedParallelWorkflowStatus::Completed
            | BoundedParallelWorkflowStatus::Partial
            | BoundedParallelWorkflowStatus::Failed
            | BoundedParallelWorkflowStatus::Cancelled => return Ok(()),
            BoundedParallelWorkflowStatus::Running
            | BoundedParallelWorkflowStatus::Synthesizing => {}
        }
        let now = self.workflow_now();
        if now >= state.root_deadline {
            self.expire_bounded_parallel_root()?;
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        if let Some(ordinal) = state.slots.iter().find_map(|slot| match &slot.state {
            ParallelSlotState::Running { deadline, .. } if now >= *deadline => {
                Some(slot.item.ordinal())
            }
            ParallelSlotState::Pending
            | ParallelSlotState::Running { .. }
            | ParallelSlotState::Terminal(_) => None,
        }) {
            self.begin_bounded_parallel_cancellation(
                ParallelCancellationMode::ChildTimeout,
                vec![ordinal],
            )?;
            self.resume_bounded_parallel_cancellation()?;
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        Ok(())
    }

    /// Fails closed for read-only task-memory ingress once a D-091 lease is no
    /// longer live. Read APIs intentionally remain immutable; cancellation and
    /// cleanup are driven by the next mutable event, control, or deadline poll.
    pub(super) fn ensure_bounded_parallel_lease_live_for_read(
        &self,
        task_id: &AgentTaskId,
    ) -> AgentOrchestratorResult<()> {
        let Some(state) = self.bounded_parallel.as_ref() else {
            return Ok(());
        };
        let root_id = self
            .root_task_id
            .as_ref()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let is_root = root_id == task_id;
        if !is_root && !state.tracks(root_id, task_id) {
            return Ok(());
        }
        match state.status {
            BoundedParallelWorkflowStatus::Cancelling => {
                return Err(AgentOrchestratorError::BoundedParallelCancellationPending)
            }
            BoundedParallelWorkflowStatus::Completed
            | BoundedParallelWorkflowStatus::Partial
            | BoundedParallelWorkflowStatus::Failed
            | BoundedParallelWorkflowStatus::Cancelled => return Ok(()),
            BoundedParallelWorkflowStatus::Running
            | BoundedParallelWorkflowStatus::Synthesizing => {}
        }
        let now = self.workflow_now();
        if now >= state.root_deadline {
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        if state.ordinal_for_task(task_id).is_some_and(|ordinal| {
            state.slot(ordinal).is_ok_and(|slot| {
                matches!(slot.state, ParallelSlotState::Running { deadline, .. } if now >= deadline)
            })
        }) {
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        Ok(())
    }

    pub(super) fn enforce_bounded_parallel_deadline_before_ingress(
        &mut self,
        task_id: &AgentTaskId,
    ) -> AgentOrchestratorResult<()> {
        let Some(state) = self.bounded_parallel.as_ref() else {
            return Ok(());
        };
        if matches!(
            state.status,
            BoundedParallelWorkflowStatus::Completed
                | BoundedParallelWorkflowStatus::Partial
                | BoundedParallelWorkflowStatus::Failed
                | BoundedParallelWorkflowStatus::Cancelled
        ) {
            return Ok(());
        }
        let root_id = self
            .root_task_id
            .as_ref()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let is_root = root_id == task_id;
        if !is_root && !state.tracks(root_id, task_id) {
            return Ok(());
        }
        if state.status == BoundedParallelWorkflowStatus::Cancelling {
            return Err(AgentOrchestratorError::BoundedParallelCancellationPending);
        }
        let now = self.workflow_now();
        if now >= state.root_deadline {
            self.expire_bounded_parallel_root()?;
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        let expired_ordinal = state.ordinal_for_task(task_id).and_then(|ordinal| {
            state.slot(ordinal).ok().and_then(|slot| match &slot.state {
                ParallelSlotState::Running { deadline, .. } if now >= *deadline => Some(ordinal),
                _ => None,
            })
        });
        if let Some(ordinal) = expired_ordinal {
            self.begin_bounded_parallel_cancellation(
                ParallelCancellationMode::ChildTimeout,
                vec![ordinal],
            )?;
            self.resume_bounded_parallel_cancellation()?;
            return Err(AgentOrchestratorError::BoundedParallelDeadlineExceeded);
        }
        Ok(())
    }
}
