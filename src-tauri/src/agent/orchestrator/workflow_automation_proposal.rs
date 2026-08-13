//! Private D-090 Workflow Automation proposal lifecycle.

use super::*;

use std::time::Duration;

pub(super) struct WorkflowAutomationWorkflowState {
    request: WorkflowAutomationProposalRequest,
    proposal_id: WorkflowProposalId,
    planner_task_id: AgentTaskId,
    phase: WorkflowAutomationPhase,
    proposal: Option<WorkflowProposalStageOutcome>,
    result: Option<WorkflowAutomationWorkflowResult>,
    events: Vec<WorkflowAutomationWorkflowEvent>,
    audit: Vec<WorkflowAutomationAuditRecord>,
    audit_contexts: BTreeMap<AgentTaskId, WorkflowAutomationAttribution>,
    accepted_events_by_task: BTreeMap<AgentTaskId, u32>,
    continuation_failure: Option<WorkflowAutomationContinuationFailure>,
    deadline: std::time::Instant,
    dispatch: Option<WorkflowManualDispatch>,
    dispatch_taken: bool,
    expiring: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorkflowAutomationPhase {
    ProposalRunning(AgentTaskId),
    SynthesisRunning,
    Completed,
    Failed,
    Cancelled,
    Expired,
}

pub(super) enum PreparedWorkflowAutomationTerminal {
    Proposal {
        task_outcome: AgentTaskOutcome,
        proposal: WorkflowProposalStageOutcome,
        failure: Option<WorkflowAutomationPartialFailureCode>,
        synthesis_request: RuntimeTurnRequest,
    },
    SynthesisCompleted {
        output: AgentTaskOutput,
        synthesis: super::super::workflow_automation::WorkflowAutomationSynthesis,
    },
    SynthesisFailed {
        task_code: AgentTaskFailureCode,
        workflow_code: WorkflowAutomationPartialFailureCode,
    },
    SynthesisCancelled,
}

impl WorkflowAutomationWorkflowState {
    pub(super) fn active_event_task<'a>(
        &'a self,
        root: &'a AgentTaskId,
    ) -> Option<&'a AgentTaskId> {
        match &self.phase {
            WorkflowAutomationPhase::ProposalRunning(task_id) => Some(task_id),
            WorkflowAutomationPhase::SynthesisRunning => Some(root),
            WorkflowAutomationPhase::Completed
            | WorkflowAutomationPhase::Failed
            | WorkflowAutomationPhase::Cancelled
            | WorkflowAutomationPhase::Expired => None,
        }
    }

    pub(super) fn active_child(&self, task_id: &AgentTaskId) -> bool {
        matches!(&self.phase, WorkflowAutomationPhase::ProposalRunning(active) if active == task_id)
    }

    fn stage(&self) -> WorkflowAutomationStage {
        match self.phase {
            WorkflowAutomationPhase::ProposalRunning(_) => WorkflowAutomationStage::Proposal,
            WorkflowAutomationPhase::SynthesisRunning
            | WorkflowAutomationPhase::Completed
            | WorkflowAutomationPhase::Failed
            | WorkflowAutomationPhase::Cancelled
            | WorkflowAutomationPhase::Expired => WorkflowAutomationStage::Synthesis,
        }
    }
}

impl<R: AgentRuntime> AgentOrchestrator<R> {
    pub fn start_workflow_automation_proposal(
        &mut self,
        source_context: &AgentExecutionContext,
        request: WorkflowAutomationProposalRequest,
    ) -> AgentOrchestratorResult<WorkflowAutomationWorkflowAcceptance> {
        self.ensure_workflow_selection_available(
            AgentWorkflowSelection::WorkflowAutomation,
            false,
        )?;
        let attribution = self.live_personal_root_attribution(source_context)?;
        if self.governance.has_pending_for(&attribution) {
            return Err(AgentOrchestratorError::GovernanceApprovalPending);
        }
        let root_task_id = attribution.task_id().clone();
        let active = self
            .runs
            .get(&root_task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        if active.run.status() != RuntimeRunStatus::AwaitingStart || !active.output.is_empty() {
            return Err(AgentOrchestratorError::DelegationAfterRuntimeOutput);
        }
        if self.active_child_task_id.is_some() || self.child_created {
            return Err(AgentOrchestratorError::ActiveChildLimitExceeded);
        }
        let definition = self.registry.get(AgentId::WorkflowAutomation)?;
        if definition.activation() != AgentActivation::Initial {
            return Err(AgentOrchestratorError::AgentDeferred {
                agent_id: AgentId::WorkflowAutomation,
            });
        }
        if definition.memory_profile_id() != AgentMemoryProfileId::MemoryDisabledV1 {
            return Err(AgentOrchestratorError::WorkflowAutomationMemoryProfileMismatch);
        }
        self.ensure_event_capacity(6)?;
        if self
            .events
            .len()
            .checked_add(6)
            .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_ORCHESTRATION_EVENTS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::EventLimitExceeded);
        }
        if self
            .tasks
            .len()
            .checked_add(1)
            .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_TASKS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::TotalChildLimitExceeded);
        }
        if self
            .run_count
            .checked_add(2)
            .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_RUNTIME_RUNS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        if self.runtime_event_count >= MAX_WORKFLOW_AUTOMATION_RUNTIME_EVENTS_PER_ROOT {
            return Err(AgentOrchestratorError::RuntimeEventLimitExceeded);
        }
        let planner_input = request.build_planner_input()?;
        let deadline = self
            .workflow_now()
            .checked_add(Duration::from_secs(MAX_WORKFLOW_DURATION_SECONDS))
            .ok_or(WorkflowAutomationError::DeadlineOverflow)?;
        let proposal_id =
            WorkflowProposalId::new(format!("workflow-proposal-{}", self.workflow_sequence))?;
        let child_task_id =
            AgentTaskId::new(format!("agent-task-child-{}-1", self.workflow_sequence))?;
        let mut child = AgentTask::new_child(
            child_task_id.clone(),
            attribution.root_task_id().clone(),
            ParentTaskId::from_task_id(root_task_id.clone()),
            definition.identity(),
            AgentTaskObjective::new(request.objective().to_owned())?,
            Some(AgentTaskContext::new(
                "Use only the immutable application-owned workflow template catalog",
            )?),
            AgentTaskExpectedDeliverable::new(
                "Return one strict WorkflowProposalV1 JSON object; do not execute it",
            )?,
        )?;
        child.start()?;
        let runtime_request = self.runtime_request(&child_task_id, 2, &planner_input)?;
        let child_context =
            AgentExecutionContext::for_task(&child, self.runtime_id, runtime_request.identity());

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
        self.tasks.insert(child_task_id.clone(), child);
        self.active_child_task_id = Some(child_task_id.clone());
        self.child_created = true;
        self.run_count = 2;
        self.events.push(AgentOrchestrationEvent::ChildCreated {
            task_id: child_task_id.clone(),
            parent_task_id: root_task_id.clone(),
        });
        self.events.push(AgentOrchestrationEvent::ChildStarted {
            task_id: child_task_id.clone(),
        });
        let mut audit_contexts = BTreeMap::new();
        audit_contexts.insert(
            root_task_id.clone(),
            WorkflowAutomationAttribution::from_execution_context(source_context),
        );
        audit_contexts.insert(
            child_task_id.clone(),
            WorkflowAutomationAttribution::from_execution_context(&child_context),
        );
        let mut workflow = WorkflowAutomationWorkflowState {
            request,
            proposal_id,
            planner_task_id: child_task_id.clone(),
            phase: WorkflowAutomationPhase::ProposalRunning(child_task_id.clone()),
            proposal: None,
            result: None,
            events: Vec::with_capacity(MAX_WORKFLOW_AUTOMATION_EVENTS),
            audit: Vec::with_capacity(MAX_WORKFLOW_AUTOMATION_AUDIT_RECORDS),
            audit_contexts,
            accepted_events_by_task: BTreeMap::new(),
            continuation_failure: None,
            deadline,
            dispatch: None,
            dispatch_taken: false,
            expiring: false,
        };
        push_workflow_automation_transition(
            &mut workflow,
            child_task_id.clone(),
            WorkflowAutomationStage::Proposal,
            WorkflowAutomationWorkflowEvent::ProposalStarted {
                task_id: child_task_id.clone(),
            },
            WorkflowAutomationAuditOutcome::Started,
        )?;
        self.workflow_automation = Some(workflow);
        self.selected_workflow = Some(AgentWorkflowSelection::WorkflowAutomation);
        match self.start_runtime_run(runtime_request) {
            Ok(run) => {
                self.runs.insert(
                    child_task_id,
                    ActiveRun {
                        run,
                        output: String::new(),
                        next_sequence: 0,
                    },
                );
                Ok(WorkflowAutomationWorkflowAcceptance::ProposalStarted {
                    context: child_context,
                })
            }
            Err(_) => {
                self.workflow_automation
                    .as_mut()
                    .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?
                    .continuation_failure =
                    Some(WorkflowAutomationContinuationFailure::ProposalStartFailed);
                self.terminalize_workflow_automation_child_failure(
                    &child_task_id,
                    AgentTaskFailureCode::RuntimeStartFailed,
                    WorkflowAutomationPartialFailureCode::RuntimeStartFailed,
                )?;
                let context = self.start_workflow_automation_synthesis()?;
                Ok(WorkflowAutomationWorkflowAcceptance::PersonalFallbackStarted { context })
            }
        }
    }

    #[must_use]
    pub fn workflow_automation_result(&self) -> Option<&WorkflowAutomationWorkflowResult> {
        self.workflow_automation
            .as_ref()
            .and_then(|workflow| workflow.result.as_ref())
    }

    #[must_use]
    pub fn workflow_automation_events(&self) -> &[WorkflowAutomationWorkflowEvent] {
        self.workflow_automation
            .as_ref()
            .map_or(&[], |workflow| workflow.events.as_slice())
    }

    #[must_use]
    pub fn workflow_automation_audit_records(&self) -> &[WorkflowAutomationAuditRecord] {
        self.workflow_automation
            .as_ref()
            .map_or(&[], |workflow| workflow.audit.as_slice())
    }

    #[must_use]
    pub fn workflow_automation_continuation_failure(
        &self,
    ) -> Option<WorkflowAutomationContinuationFailure> {
        self.workflow_automation
            .as_ref()
            .and_then(|workflow| workflow.continuation_failure)
    }

    #[must_use]
    pub fn workflow_manual_dispatch_availability(&self) -> WorkflowManualDispatchAvailability {
        let Some(workflow) = self.workflow_automation.as_ref() else {
            return WorkflowManualDispatchAvailability::NotReady;
        };
        if workflow.dispatch_taken {
            return WorkflowManualDispatchAvailability::Taken;
        }
        if self.workflow_now() >= workflow.deadline {
            return WorkflowManualDispatchAvailability::Expired;
        }
        match workflow
            .result
            .as_ref()
            .map(|result| result.synthesis().disposition())
        {
            Some(WorkflowSynthesisDisposition::ReadyForManualDispatch)
                if workflow.dispatch.is_some() =>
            {
                WorkflowManualDispatchAvailability::Available
            }
            Some(WorkflowSynthesisDisposition::ProposalOnly) => {
                WorkflowManualDispatchAvailability::ProposalOnly
            }
            _ => WorkflowManualDispatchAvailability::NotReady,
        }
    }

    pub fn take_workflow_manual_dispatch(
        &mut self,
    ) -> AgentOrchestratorResult<WorkflowManualDispatch> {
        self.enforce_workflow_automation_deadline_before_ingress()?;
        let now = self.workflow_now();
        let workflow = self
            .workflow_automation
            .as_mut()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        if workflow.dispatch_taken {
            return Err(WorkflowAutomationError::ManualDispatchAlreadyTaken.into());
        }
        if now >= workflow.deadline {
            workflow.dispatch = None;
            return Err(WorkflowAutomationError::ManualDispatchExpired.into());
        }
        let dispatch = workflow
            .dispatch
            .take()
            .ok_or(WorkflowAutomationError::ManualDispatchUnavailable)?;
        workflow.dispatch_taken = true;
        Ok(dispatch)
    }

    pub(super) fn prepare_workflow_automation_terminal(
        &mut self,
        task_id: &AgentTaskId,
        event: &UntrustedRuntimeEvent,
    ) -> AgentOrchestratorResult<Option<PreparedWorkflowAutomationTerminal>> {
        self.enforce_workflow_automation_deadline_before_ingress()?;
        let Some(workflow) = self.workflow_automation.as_ref() else {
            return Ok(None);
        };
        if workflow.active_event_task(
            self.root_task_id
                .as_ref()
                .ok_or(AgentOrchestratorError::RootMissing)?,
        ) != Some(task_id)
            || !matches!(
                event,
                UntrustedRuntimeEvent::ResponseCompleted
                    | UntrustedRuntimeEvent::ResponseFailed { .. }
            )
        {
            return Ok(None);
        }
        let active = self
            .runs
            .get(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        match (&workflow.phase, event) {
            (
                WorkflowAutomationPhase::ProposalRunning(_),
                UntrustedRuntimeEvent::ResponseCompleted,
            ) => {
                let parsed = workflow.request.parse_proposal(
                    workflow.proposal_id.clone(),
                    &active.output,
                    &self.registry,
                );
                let (proposal, task_outcome, failure) = match parsed {
                    Ok(value) => {
                        let output = AgentTaskOutput::new(active.output.clone())?;
                        (
                            WorkflowProposalStageOutcome::Validated(value),
                            AgentTaskOutcome::Completed(AgentTaskResult::new(
                                task_id.clone(),
                                AgentId::WorkflowAutomation,
                                output,
                            )),
                            None,
                        )
                    }
                    Err(error) => {
                        let code = error.validation_failure_code();
                        (
                            WorkflowProposalStageOutcome::Rejected {
                                proposal_id: workflow.proposal_id.clone(),
                                code,
                            },
                            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                                task_id.clone(),
                                AgentId::WorkflowAutomation,
                                AgentTaskFailureCode::RuntimeOutputInvalid,
                            )),
                            Some(WorkflowAutomationPartialFailureCode::ProposalRejected(code)),
                        )
                    }
                };
                let synthesis_input = workflow.request.build_synthesis_input(&proposal)?;
                let synthesis_request = self.runtime_request(
                    self.root_task_id
                        .as_ref()
                        .ok_or(AgentOrchestratorError::RootMissing)?,
                    3,
                    &synthesis_input,
                )?;
                Ok(Some(PreparedWorkflowAutomationTerminal::Proposal {
                    task_outcome,
                    proposal,
                    failure,
                    synthesis_request,
                }))
            }
            (
                WorkflowAutomationPhase::ProposalRunning(_),
                UntrustedRuntimeEvent::ResponseFailed { failure },
            ) => {
                let cancelled = failure.code() == RuntimeFailureCode::Cancelled;
                let proposal = if cancelled {
                    WorkflowProposalStageOutcome::Cancelled {
                        proposal_id: workflow.proposal_id.clone(),
                    }
                } else {
                    WorkflowProposalStageOutcome::Failed {
                        proposal_id: workflow.proposal_id.clone(),
                        code: AgentTaskFailureCode::RuntimeReported(failure.code()),
                    }
                };
                let synthesis_input = workflow.request.build_synthesis_input(&proposal)?;
                let synthesis_request = self.runtime_request(
                    self.root_task_id
                        .as_ref()
                        .ok_or(AgentOrchestratorError::RootMissing)?,
                    3,
                    &synthesis_input,
                )?;
                Ok(Some(PreparedWorkflowAutomationTerminal::Proposal {
                    task_outcome: if cancelled {
                        AgentTaskOutcome::Cancelled(super::super::task::AgentTaskCancellation::new(
                            task_id.clone(),
                            AgentId::WorkflowAutomation,
                        ))
                    } else {
                        AgentTaskOutcome::Failed(AgentTaskFailure::new(
                            task_id.clone(),
                            AgentId::WorkflowAutomation,
                            AgentTaskFailureCode::RuntimeReported(failure.code()),
                        ))
                    },
                    proposal,
                    failure: Some(if cancelled {
                        WorkflowAutomationPartialFailureCode::Cancelled
                    } else {
                        WorkflowAutomationPartialFailureCode::RuntimeFailed
                    }),
                    synthesis_request,
                }))
            }
            (
                WorkflowAutomationPhase::SynthesisRunning,
                UntrustedRuntimeEvent::ResponseCompleted,
            ) => {
                let proposal = workflow
                    .proposal
                    .as_ref()
                    .ok_or(AgentOrchestratorError::WorkflowAutomationStageMismatch)?;
                match workflow.request.parse_synthesis(proposal, &active.output) {
                    Ok(synthesis) => Ok(Some(
                        PreparedWorkflowAutomationTerminal::SynthesisCompleted {
                            output: AgentTaskOutput::new(synthesis.summary().to_owned())?,
                            synthesis,
                        },
                    )),
                    Err(_) => Ok(Some(PreparedWorkflowAutomationTerminal::SynthesisFailed {
                        task_code: AgentTaskFailureCode::RuntimeOutputInvalid,
                        workflow_code:
                            WorkflowAutomationPartialFailureCode::InvalidStructuredOutput,
                    })),
                }
            }
            (
                WorkflowAutomationPhase::SynthesisRunning,
                UntrustedRuntimeEvent::ResponseFailed { failure },
            ) => {
                if failure.code() == RuntimeFailureCode::Cancelled {
                    Ok(Some(PreparedWorkflowAutomationTerminal::SynthesisCancelled))
                } else {
                    Ok(Some(PreparedWorkflowAutomationTerminal::SynthesisFailed {
                        task_code: AgentTaskFailureCode::RuntimeReported(failure.code()),
                        workflow_code: WorkflowAutomationPartialFailureCode::RuntimeFailed,
                    }))
                }
            }
            _ => Err(AgentOrchestratorError::WorkflowAutomationStageMismatch),
        }
    }

    pub(super) fn preflight_workflow_automation_event_capacity(
        &self,
        task_id: &AgentTaskId,
        event: &UntrustedRuntimeEvent,
    ) -> AgentOrchestratorResult<()> {
        let Some(workflow) = self.workflow_automation.as_ref() else {
            return Ok(());
        };
        let root_id = self
            .root_task_id
            .as_ref()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        if workflow.active_event_task(root_id) != Some(task_id) {
            return Ok(());
        }
        let accepted = workflow
            .accepted_events_by_task
            .get(task_id)
            .copied()
            .unwrap_or(0);
        let terminal = matches!(
            event,
            UntrustedRuntimeEvent::ResponseCompleted | UntrustedRuntimeEvent::ResponseFailed { .. }
        );
        let maximum_before_accept = if terminal { 8 } else { 7 };
        if accepted >= maximum_before_accept {
            return Err(AgentOrchestratorError::WorkflowAutomationEventLimitExceeded);
        }
        if terminal {
            let (workflow_transitions, generic_events, additional_runs, runtime_events) =
                match workflow.phase {
                    WorkflowAutomationPhase::ProposalRunning(_) => (3usize, 4usize, 1u8, 9usize),
                    WorkflowAutomationPhase::SynthesisRunning => (1, 1, 0, 1),
                    WorkflowAutomationPhase::Completed
                    | WorkflowAutomationPhase::Failed
                    | WorkflowAutomationPhase::Cancelled
                    | WorkflowAutomationPhase::Expired => {
                        return Err(AgentOrchestratorError::WorkflowAutomationStageMismatch)
                    }
                };
            if workflow
                .events
                .len()
                .checked_add(workflow_transitions)
                .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_EVENTS)
                || workflow
                    .audit
                    .len()
                    .checked_add(workflow_transitions)
                    .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_AUDIT_RECORDS)
            {
                return Err(AgentOrchestratorError::WorkflowAutomationJournalLimitExceeded);
            }
            if self
                .run_count
                .checked_add(additional_runs)
                .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_RUNTIME_RUNS_PER_ROOT)
            {
                return Err(AgentOrchestratorError::RunLimitExceeded);
            }
            if self
                .runtime_event_count
                .checked_add(runtime_events)
                .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_RUNTIME_EVENTS_PER_ROOT)
            {
                return Err(AgentOrchestratorError::RuntimeEventLimitExceeded);
            }
            if self
                .events
                .len()
                .checked_add(generic_events)
                .is_none_or(|count| count > MAX_WORKFLOW_AUTOMATION_ORCHESTRATION_EVENTS_PER_ROOT)
            {
                return Err(AgentOrchestratorError::EventLimitExceeded);
            }
            self.ensure_event_capacity(generic_events)?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn set_workflow_clock_for_test(
        &mut self,
        clock: workflow_automation_dispatch::WorkflowClock,
    ) {
        self.workflow_clock = clock;
    }

    pub(super) fn record_workflow_automation_event_acceptance(&mut self, task_id: &AgentTaskId) {
        let Some(workflow) = self.workflow_automation.as_mut() else {
            return;
        };
        let Some(root_id) = self.root_task_id.as_ref() else {
            return;
        };
        if workflow.active_event_task(root_id) == Some(task_id) {
            *workflow
                .accepted_events_by_task
                .entry(task_id.clone())
                .or_default() += 1;
        }
    }

    pub(super) fn apply_prepared_workflow_automation_terminal(
        &mut self,
        task_id: &AgentTaskId,
        prepared: PreparedWorkflowAutomationTerminal,
    ) -> AgentOrchestratorResult<()> {
        self.runs
            .remove(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        match prepared {
            PreparedWorkflowAutomationTerminal::Proposal {
                task_outcome,
                proposal,
                failure,
                synthesis_request,
            } => {
                self.finish_workflow_automation_child(task_id, task_outcome)?;
                let mut workflow = self
                    .workflow_automation
                    .take()
                    .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
                workflow.proposal = Some(proposal.clone());
                let (event, outcome) = if let Some(code) = failure {
                    (
                        WorkflowAutomationWorkflowEvent::PartialFailure {
                            stage: WorkflowAutomationStage::Proposal,
                            code,
                        },
                        WorkflowAutomationAuditOutcome::PartialFailure(code),
                    )
                } else {
                    let disposition = match &proposal {
                        WorkflowProposalStageOutcome::Validated(value) => value.disposition(),
                        _ => return Err(AgentOrchestratorError::WorkflowAutomationStageMismatch),
                    };
                    (
                        WorkflowAutomationWorkflowEvent::ProposalCompleted {
                            task_id: task_id.clone(),
                            disposition,
                        },
                        WorkflowAutomationAuditOutcome::Completed,
                    )
                };
                push_workflow_automation_transition(
                    &mut workflow,
                    task_id.clone(),
                    WorkflowAutomationStage::Proposal,
                    event,
                    outcome,
                )?;
                self.workflow_automation = Some(workflow);
                self.start_workflow_automation_synthesis_prepared(synthesis_request)
                    .map(|_| ())
            }
            PreparedWorkflowAutomationTerminal::SynthesisCompleted { output, synthesis } => {
                let root_id = self
                    .root_task_id
                    .clone()
                    .ok_or(AgentOrchestratorError::RootMissing)?;
                let mut workflow = self
                    .workflow_automation
                    .take()
                    .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
                let proposal = workflow
                    .proposal
                    .clone()
                    .ok_or(AgentOrchestratorError::WorkflowAutomationStageMismatch)?;
                let result = WorkflowAutomationWorkflowResult::new(
                    RootTaskId::from_task_id(root_id.clone()),
                    workflow.request.template_id(),
                    proposal.clone(),
                    synthesis.clone(),
                );
                let root = self
                    .tasks
                    .get_mut(&root_id)
                    .ok_or(AgentOrchestratorError::TaskNotFound)?;
                root.complete(AgentTaskResult::new(
                    root_id.clone(),
                    AgentId::PersonalAssistant,
                    output,
                ))?;
                self.events.push(AgentOrchestrationEvent::RootCompleted {
                    task_id: root_id.clone(),
                });
                push_workflow_automation_transition(
                    &mut workflow,
                    root_id.clone(),
                    WorkflowAutomationStage::Synthesis,
                    WorkflowAutomationWorkflowEvent::Completed {
                        task_id: root_id.clone(),
                    },
                    WorkflowAutomationAuditOutcome::Completed,
                )?;
                if synthesis.disposition() == WorkflowSynthesisDisposition::ReadyForManualDispatch
                    && self.workflow_now() < workflow.deadline
                {
                    let validated = match &proposal {
                        WorkflowProposalStageOutcome::Validated(value) => value,
                        _ => return Err(AgentOrchestratorError::WorkflowAutomationStageMismatch),
                    };
                    workflow.dispatch = Some(WorkflowManualDispatch::issue(
                        validated,
                        RootTaskId::from_task_id(root_id.clone()),
                        workflow.planner_task_id.clone(),
                        workflow.deadline,
                    )?);
                }
                workflow.result = Some(result);
                workflow.phase = WorkflowAutomationPhase::Completed;
                self.workflow_automation = Some(workflow);
                self.cleanup_terminal_task(&root_id);
                Ok(())
            }
            PreparedWorkflowAutomationTerminal::SynthesisFailed {
                task_code,
                workflow_code,
            } => self.fail_workflow_automation_root_without_run(task_code, workflow_code),
            PreparedWorkflowAutomationTerminal::SynthesisCancelled => {
                self.cancel_workflow_automation_synthesis_without_run()
            }
        }
    }

    fn finish_workflow_automation_child(
        &mut self,
        child_id: &AgentTaskId,
        outcome: AgentTaskOutcome,
    ) -> AgentOrchestratorResult<()> {
        self.ensure_event_capacity(2)?;
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let child = self
            .tasks
            .get_mut(child_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        match &outcome {
            AgentTaskOutcome::Completed(result) => child.complete(result.clone())?,
            AgentTaskOutcome::Failed(failure) => child.fail(failure.clone())?,
            AgentTaskOutcome::Cancelled(_) => {
                if child.cancel() != AgentTaskCancellationOutcome::Cancelled {
                    return Err(AgentOrchestratorError::WorkflowAutomationStageMismatch);
                }
            }
        }
        match &outcome {
            AgentTaskOutcome::Completed(_) => {
                self.events.push(AgentOrchestrationEvent::ChildCompleted {
                    task_id: child_id.clone(),
                })
            }
            AgentTaskOutcome::Failed(failure) => {
                self.events.push(AgentOrchestrationEvent::ChildFailed {
                    task_id: child_id.clone(),
                    code: failure.code(),
                })
            }
            AgentTaskOutcome::Cancelled(_) => {
                self.events.push(AgentOrchestrationEvent::TaskCancelled {
                    task_id: child_id.clone(),
                    agent_id: AgentId::WorkflowAutomation,
                })
            }
        }
        self.events.push(AgentOrchestrationEvent::ResultReturned {
            child_task_id: child_id.clone(),
            parent_task_id: root_id,
            outcome: outcome.kind(),
        });
        self.child_outcome = Some(outcome);
        self.active_child_task_id = None;
        self.cleanup_terminal_task(child_id);
        Ok(())
    }

    fn terminalize_workflow_automation_child_failure(
        &mut self,
        child_id: &AgentTaskId,
        task_code: AgentTaskFailureCode,
        workflow_code: WorkflowAutomationPartialFailureCode,
    ) -> AgentOrchestratorResult<()> {
        self.finish_workflow_automation_child(
            child_id,
            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                child_id.clone(),
                AgentId::WorkflowAutomation,
                task_code,
            )),
        )?;
        let proposal_id = self
            .workflow_automation
            .as_ref()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?
            .proposal_id
            .clone();
        let proposal = WorkflowProposalStageOutcome::Failed {
            proposal_id,
            code: task_code,
        };
        let mut workflow = self
            .workflow_automation
            .take()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        workflow.proposal = Some(proposal);
        push_workflow_automation_transition(
            &mut workflow,
            child_id.clone(),
            WorkflowAutomationStage::Proposal,
            WorkflowAutomationWorkflowEvent::PartialFailure {
                stage: WorkflowAutomationStage::Proposal,
                code: workflow_code,
            },
            WorkflowAutomationAuditOutcome::PartialFailure(workflow_code),
        )?;
        self.workflow_automation = Some(workflow);
        Ok(())
    }

    fn start_workflow_automation_synthesis(
        &mut self,
    ) -> AgentOrchestratorResult<AgentExecutionContext> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let workflow = self
            .workflow_automation
            .as_ref()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        let proposal = workflow
            .proposal
            .as_ref()
            .ok_or(AgentOrchestratorError::WorkflowAutomationStageMismatch)?;
        let input = workflow.request.build_synthesis_input(proposal)?;
        let request = self.runtime_request(&root_id, 3, &input)?;
        self.start_workflow_automation_synthesis_prepared(request)
    }

    fn start_workflow_automation_synthesis_prepared(
        &mut self,
        request: RuntimeTurnRequest,
    ) -> AgentOrchestratorResult<AgentExecutionContext> {
        self.ensure_event_capacity(1)?;
        self.enforce_workflow_automation_deadline_before_ingress()?;
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let mut workflow = self
            .workflow_automation
            .take()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        self.tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .resume_from_child()?;
        let context = AgentExecutionContext::for_task(
            self.tasks
                .get(&root_id)
                .ok_or(AgentOrchestratorError::TaskNotFound)?,
            self.runtime_id,
            request.identity(),
        );
        workflow.audit_contexts.insert(
            root_id.clone(),
            WorkflowAutomationAttribution::from_execution_context(&context),
        );
        self.run_count = 3;
        match self.start_runtime_run(request) {
            Ok(run) => {
                self.runs.insert(
                    root_id.clone(),
                    ActiveRun {
                        run,
                        output: String::new(),
                        next_sequence: 0,
                    },
                );
                self.events.push(AgentOrchestrationEvent::ParentResumed {
                    task_id: root_id.clone(),
                });
                push_workflow_automation_transition(
                    &mut workflow,
                    root_id.clone(),
                    WorkflowAutomationStage::Synthesis,
                    WorkflowAutomationWorkflowEvent::SynthesisStarted {
                        task_id: root_id.clone(),
                    },
                    WorkflowAutomationAuditOutcome::Started,
                )?;
                workflow.phase = WorkflowAutomationPhase::SynthesisRunning;
                self.workflow_automation = Some(workflow);
                Ok(context)
            }
            Err(error) => {
                workflow.continuation_failure = Some(match workflow.continuation_failure {
                    Some(WorkflowAutomationContinuationFailure::ProposalStartFailed) => {
                        WorkflowAutomationContinuationFailure::ProposalAndSynthesisStartFailed
                    }
                    _ => WorkflowAutomationContinuationFailure::SynthesisStartFailed,
                });
                self.workflow_automation = Some(workflow);
                self.fail_workflow_automation_root_without_run(
                    AgentTaskFailureCode::RuntimeStartFailed,
                    WorkflowAutomationPartialFailureCode::RuntimeStartFailed,
                )?;
                Err(error)
            }
        }
    }

    fn fail_workflow_automation_root_without_run(
        &mut self,
        task_code: AgentTaskFailureCode,
        workflow_code: WorkflowAutomationPartialFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        self.tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .fail(AgentTaskFailure::new(
                root_id.clone(),
                AgentId::PersonalAssistant,
                task_code,
            ))?;
        self.events.push(AgentOrchestrationEvent::RootFailed {
            task_id: root_id.clone(),
            code: task_code,
        });
        let mut workflow = self
            .workflow_automation
            .take()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        push_workflow_automation_transition(
            &mut workflow,
            root_id.clone(),
            WorkflowAutomationStage::Synthesis,
            WorkflowAutomationWorkflowEvent::Failed {
                stage: WorkflowAutomationStage::Synthesis,
                code: workflow_code,
            },
            WorkflowAutomationAuditOutcome::Failed(workflow_code),
        )?;
        workflow.phase = WorkflowAutomationPhase::Failed;
        self.workflow_automation = Some(workflow);
        self.cleanup_terminal_task(&root_id);
        Ok(())
    }

    fn cancel_workflow_automation_synthesis_without_run(&mut self) -> AgentOrchestratorResult<()> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let root = self
            .tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        if root.cancel() != AgentTaskCancellationOutcome::Cancelled {
            return Err(AgentOrchestratorError::WorkflowAutomationStageMismatch);
        }
        self.events.push(AgentOrchestrationEvent::TaskCancelled {
            task_id: root_id.clone(),
            agent_id: AgentId::PersonalAssistant,
        });
        let mut workflow = self
            .workflow_automation
            .take()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        push_workflow_automation_transition(
            &mut workflow,
            root_id.clone(),
            WorkflowAutomationStage::Synthesis,
            WorkflowAutomationWorkflowEvent::Cancelled {
                stage: WorkflowAutomationStage::Synthesis,
            },
            WorkflowAutomationAuditOutcome::Cancelled,
        )?;
        workflow.phase = WorkflowAutomationPhase::Cancelled;
        self.workflow_automation = Some(workflow);
        self.cleanup_terminal_task(&root_id);
        Ok(())
    }

    pub(super) fn fail_workflow_automation_task(
        &mut self,
        task_id: &AgentTaskId,
        code: AgentTaskFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let run_status = self.runs.get(task_id).map(|active| active.run.status());
        if run_status.is_some_and(RuntimeRunStatus::is_terminal) {
            self.runs.remove(task_id);
        } else {
            let _ = self.cancel_run(task_id)?;
        }
        if self.root_task_id.as_ref() == Some(task_id) {
            self.fail_workflow_automation_root_without_run(
                code,
                WorkflowAutomationPartialFailureCode::RuntimeFailed,
            )
        } else {
            self.terminalize_workflow_automation_child_failure(
                task_id,
                code,
                WorkflowAutomationPartialFailureCode::RuntimeFailed,
            )?;
            self.start_workflow_automation_synthesis().map(|_| ())
        }
    }

    pub(super) fn cancel_workflow_automation_child(
        &mut self,
        child_id: &AgentTaskId,
        root_is_cancelling: bool,
    ) -> AgentOrchestratorResult<AgentTaskCancellationOutcome> {
        self.cancel_pending_governance(child_id)?;
        let disposition = self.cancel_run(child_id)?;
        if let RunCancellationDisposition::UnexpectedTerminal(status) = disposition {
            self.terminalize_workflow_automation_child_failure(
                child_id,
                AgentTaskFailureCode::RuntimeStateMismatch,
                WorkflowAutomationPartialFailureCode::RuntimeFailed,
            )?;
            if !root_is_cancelling {
                let _ = self.start_workflow_automation_synthesis();
            }
            return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
        }
        if root_is_cancelling {
            let child = self
                .tasks
                .get_mut(child_id)
                .ok_or(AgentOrchestratorError::TaskNotFound)?;
            let outcome = child.cancel();
            if outcome == AgentTaskCancellationOutcome::Cancelled {
                self.events.push(AgentOrchestrationEvent::TaskCancelled {
                    task_id: child_id.clone(),
                    agent_id: AgentId::WorkflowAutomation,
                });
                self.active_child_task_id = None;
                self.cleanup_terminal_task(child_id);
            }
            return Ok(outcome);
        }
        let proposal_id = self
            .workflow_automation
            .as_ref()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?
            .proposal_id
            .clone();
        self.finish_workflow_automation_child(
            child_id,
            AgentTaskOutcome::Cancelled(super::super::task::AgentTaskCancellation::new(
                child_id.clone(),
                AgentId::WorkflowAutomation,
            )),
        )?;
        let proposal = WorkflowProposalStageOutcome::Cancelled { proposal_id };
        let mut workflow = self
            .workflow_automation
            .take()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        workflow.proposal = Some(proposal);
        push_workflow_automation_transition(
            &mut workflow,
            child_id.clone(),
            WorkflowAutomationStage::Proposal,
            WorkflowAutomationWorkflowEvent::Cancelled {
                stage: WorkflowAutomationStage::Proposal,
            },
            WorkflowAutomationAuditOutcome::Cancelled,
        )?;
        self.workflow_automation = Some(workflow);
        self.start_workflow_automation_synthesis()?;
        Ok(AgentTaskCancellationOutcome::Cancelled)
    }

    pub(super) fn enforce_workflow_automation_deadline_before_ingress(
        &mut self,
    ) -> AgentOrchestratorResult<()> {
        let Some(workflow) = self.workflow_automation.as_ref() else {
            return Ok(());
        };
        if workflow.expiring
            || matches!(
                workflow.phase,
                WorkflowAutomationPhase::Completed
                    | WorkflowAutomationPhase::Failed
                    | WorkflowAutomationPhase::Cancelled
                    | WorkflowAutomationPhase::Expired
            )
            || self.workflow_now() < workflow.deadline
        {
            return Ok(());
        }
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        self.workflow_automation
            .as_mut()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?
            .expiring = true;
        let outcome = self.cancel_root(&root_id);
        match outcome {
            Ok(_) => Err(WorkflowAutomationError::WorkflowExpired.into()),
            Err(error) => {
                self.workflow_automation
                    .as_mut()
                    .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?
                    .expiring = false;
                Err(error)
            }
        }
    }

    pub(super) fn workflow_automation_root_cancellation_stage(
        &self,
    ) -> Option<WorkflowAutomationStage> {
        self.workflow_automation.as_ref().and_then(|workflow| {
            (!matches!(
                workflow.phase,
                WorkflowAutomationPhase::Completed
                    | WorkflowAutomationPhase::Failed
                    | WorkflowAutomationPhase::Cancelled
                    | WorkflowAutomationPhase::Expired
            ))
            .then(|| workflow.stage())
        })
    }

    pub(super) fn preflight_workflow_automation_root_cancellation(
        &self,
        stage: Option<WorkflowAutomationStage>,
    ) -> AgentOrchestratorResult<()> {
        if stage.is_none() {
            return Ok(());
        }
        let workflow = self
            .workflow_automation
            .as_ref()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        if workflow.events.len() >= MAX_WORKFLOW_AUTOMATION_EVENTS
            || workflow.audit.len() >= MAX_WORKFLOW_AUTOMATION_AUDIT_RECORDS
        {
            return Err(AgentOrchestratorError::WorkflowAutomationJournalLimitExceeded);
        }
        Ok(())
    }

    pub(super) fn record_workflow_automation_root_cancelled(
        &mut self,
        root_id: &AgentTaskId,
        stage: WorkflowAutomationStage,
    ) -> AgentOrchestratorResult<()> {
        let mut workflow = self
            .workflow_automation
            .take()
            .ok_or(AgentOrchestratorError::WorkflowAutomationWorkflowMissing)?;
        let (event, outcome, phase) = if workflow.expiring {
            (
                WorkflowAutomationWorkflowEvent::Expired { stage },
                WorkflowAutomationAuditOutcome::Expired,
                WorkflowAutomationPhase::Expired,
            )
        } else {
            (
                WorkflowAutomationWorkflowEvent::Cancelled { stage },
                WorkflowAutomationAuditOutcome::Cancelled,
                WorkflowAutomationPhase::Cancelled,
            )
        };
        push_workflow_automation_transition(&mut workflow, root_id.clone(), stage, event, outcome)?;
        workflow.phase = phase;
        self.workflow_automation = Some(workflow);
        Ok(())
    }
}

pub(super) fn push_workflow_automation_transition(
    workflow: &mut WorkflowAutomationWorkflowState,
    task_id: AgentTaskId,
    stage: WorkflowAutomationStage,
    event: WorkflowAutomationWorkflowEvent,
    outcome: WorkflowAutomationAuditOutcome,
) -> AgentOrchestratorResult<()> {
    if workflow.events.len() >= MAX_WORKFLOW_AUTOMATION_EVENTS
        || workflow.audit.len() >= MAX_WORKFLOW_AUTOMATION_AUDIT_RECORDS
    {
        return Err(AgentOrchestratorError::WorkflowAutomationJournalLimitExceeded);
    }
    let attribution = workflow
        .audit_contexts
        .get(&task_id)
        .cloned()
        .ok_or(AgentOrchestratorError::WorkflowAutomationStageMismatch)?;
    let sequence = u8::try_from(workflow.audit.len() + 1)
        .map_err(|_| AgentOrchestratorError::WorkflowAutomationJournalLimitExceeded)?;
    workflow.events.push(event);
    workflow.audit.push(WorkflowAutomationAuditRecord::new(
        sequence,
        attribution,
        workflow.proposal_id.clone(),
        workflow.request.template_id(),
        stage,
        outcome,
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    };

    use crate::agent::{
        research_knowledge::ResearchKnowledgeWorkflowEvent,
        runtime::{
            RuntimeEventEnvelope, RuntimeFailure, RuntimeFailureCode, RuntimeOutputText,
            RuntimeResponseId,
        },
        workflow_automation::{
            WorkflowManualDispatchAuditRecord, WorkflowManualDispatchAvailability,
            WorkflowManualDispatchEvent, WorkflowManualDispatchStatus, WorkflowTemplateCatalog,
            WorkflowTemplateId, WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE,
        },
    };

    #[derive(Clone)]
    struct TestClock {
        now: Arc<Mutex<std::time::Instant>>,
    }

    impl TestClock {
        fn new() -> (Self, workflow_automation_dispatch::WorkflowClock) {
            let now = Arc::new(Mutex::new(std::time::Instant::now()));
            let clock_now = Arc::clone(&now);
            let clock = Arc::new(move || {
                *clock_now
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

    fn started(
        context: &AgentExecutionContext,
        response_id: &str,
    ) -> Result<RuntimeEventEnvelope, Box<dyn std::error::Error>> {
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
    ) -> Result<RuntimeEventEnvelope, Box<dyn std::error::Error>> {
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

    fn complete_stage(
        orchestrator: &mut AgentOrchestrator<NativeAgentRuntime>,
        context: &AgentExecutionContext,
        response_id: &str,
        output: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task_id = context.task_id().clone();
        orchestrator.accept_runtime_event(&task_id, started(context, response_id)?)?;
        orchestrator.accept_runtime_event(&task_id, delta(context, output)?)?;
        orchestrator.accept_runtime_event(&task_id, completed(context))?;
        Ok(())
    }

    fn canonical_proposal_json(
        template_id: WorkflowTemplateId,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let input = WorkflowTemplateCatalog::built_in()
            .proposal_request(template_id)?
            .build_planner_input()?;
        let (_, selected) = input
            .split_once("selected_template(application-owned):\n")
            .ok_or("missing selected-template marker")?;
        let (proposal, _) = selected
            .split_once("\nReturn exactly one strict WorkflowProposalV1 JSON object.")
            .ok_or("missing proposal instruction marker")?;
        Ok(proposal.to_owned())
    }

    fn synthesis_json(
        template_id: WorkflowTemplateId,
        proposal_id: &WorkflowProposalId,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(&serde_json::json!({
            "version": "v1",
            "summary": WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE,
            "template_id": template_id,
            "proposal_id": proposal_id.as_str(),
            "validation_disposition": "ready-for-manual-dispatch",
            "status": "complete",
            "unresolved_issues": [],
            "fixture_based": true,
            "unwired": true,
            "workflow_executed": false,
            "tools_executed": false,
            "approvals_requested": false,
            "effects_performed": false,
        }))?)
    }

    fn start_ready_proposal(
        orchestrator: &mut AgentOrchestrator<NativeAgentRuntime>,
    ) -> Result<(AgentExecutionContext, AgentExecutionContext), Box<dyn std::error::Error>> {
        let request = WorkflowTemplateCatalog::built_in()
            .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
        let root = orchestrator.start_root(request.objective())?;
        let acceptance = orchestrator.start_workflow_automation_proposal(&root, request)?;
        Ok((root, acceptance.context().clone()))
    }

    fn finish_ready_proposal(
        orchestrator: &mut AgentOrchestrator<NativeAgentRuntime>,
    ) -> Result<(AgentExecutionContext, WorkflowProposalId), Box<dyn std::error::Error>> {
        let (root, planner) = start_ready_proposal(orchestrator)?;
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
            &canonical_proposal_json(WorkflowTemplateId::ResearchBriefV1)?,
        )?;
        let synthesis = orchestrator.current_context(root.task_id())?;
        complete_stage(
            orchestrator,
            &synthesis,
            "workflow-synthesis",
            &synthesis_json(WorkflowTemplateId::ResearchBriefV1, &proposal_id)?,
        )?;
        Ok((root, proposal_id))
    }

    fn assert_child_then_root_cancelled(
        orchestrator: &AgentOrchestrator<NativeAgentRuntime>,
        child_id: &AgentTaskId,
        root_id: &AgentTaskId,
        child_agent_id: AgentId,
    ) {
        assert!(matches!(
            &orchestrator.events()[orchestrator.events().len() - 2..],
            [
                AgentOrchestrationEvent::TaskCancelled {
                    task_id,
                    agent_id,
                },
                AgentOrchestrationEvent::TaskCancelled {
                    task_id: cancelled_root_id,
                    agent_id: AgentId::PersonalAssistant,
                },
            ] if task_id == child_id
                && *agent_id == child_agent_id
                && cancelled_root_id == root_id
        ));
    }

    #[test]
    fn proposal_deadline_expiry_is_child_first_single_terminal_and_has_no_synthesis(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let (root, planner) = start_ready_proposal(&mut orchestrator)?;
        let root_id = root.task_id().clone();
        let planner_id = planner.task_id().clone();

        clock.advance(Duration::from_secs(MAX_WORKFLOW_DURATION_SECONDS));
        assert_eq!(
            orchestrator
                .accept_runtime_event(&planner_id, started(&planner, "expired-planner-response")?,),
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::WorkflowExpired,
            ))
        );
        assert_eq!(
            orchestrator.task(&planner_id).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert_eq!(
            orchestrator.task(&root_id).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert_child_then_root_cancelled(
            &orchestrator,
            &planner_id,
            &root_id,
            AgentId::WorkflowAutomation,
        );
        assert_eq!(
            orchestrator
                .workflow_automation_events()
                .iter()
                .filter(|event| matches!(event, WorkflowAutomationWorkflowEvent::Expired { .. }))
                .count(),
            1
        );
        assert!(matches!(
            orchestrator.workflow_automation_events(),
            [
                WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
                WorkflowAutomationWorkflowEvent::Expired {
                    stage: WorkflowAutomationStage::Proposal,
                },
            ]
        ));
        assert!(orchestrator.workflow_automation_result().is_none());
        assert!(orchestrator.active_child_task().is_none());
        assert_eq!(orchestrator.run_count(), 2);
        assert!(orchestrator
            .workflow_automation
            .as_ref()
            .is_some_and(|workflow| workflow.dispatch.is_none()));

        assert!(matches!(
            orchestrator.take_workflow_manual_dispatch(),
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchExpired,
            ))
        ));
        assert_eq!(
            orchestrator
                .workflow_automation_events()
                .iter()
                .filter(|event| matches!(event, WorkflowAutomationWorkflowEvent::Expired { .. }))
                .count(),
            1
        );
        Ok(())
    }

    #[test]
    fn proposal_lease_is_resampled_after_terminal_preparation_before_event_acceptance(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (base_clock, injected) = TestClock::new();
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        let (root, planner) = start_ready_proposal(&mut orchestrator)?;
        let root_id = root.task_id().clone();
        let planner_id = planner.task_id().clone();
        let base = *base_clock
            .now
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let reads = Arc::new(AtomicUsize::new(0));
        let reads_for_clock = Arc::clone(&reads);
        orchestrator.set_workflow_clock_for_test(Arc::new(move || {
            if reads_for_clock.fetch_add(1, Ordering::SeqCst) < 2 {
                base
            } else {
                base + Duration::from_secs(MAX_WORKFLOW_DURATION_SECONDS)
            }
        }));

        let terminal = RuntimeEventEnvelope::for_identity(
            planner.runtime_run_identity(),
            0,
            UntrustedRuntimeEvent::ResponseFailed {
                failure: RuntimeFailure::new(RuntimeFailureCode::ProviderUnavailable, true, None)?,
            },
        );
        assert_eq!(
            orchestrator.accept_runtime_event(&planner_id, terminal),
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::WorkflowExpired,
            ))
        );
        assert!(reads.load(Ordering::SeqCst) >= 3);
        assert_eq!(orchestrator.runtime_event_count(), 0);
        assert_eq!(
            orchestrator.task(&planner_id).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert_eq!(
            orchestrator.task(&root_id).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert_child_then_root_cancelled(
            &orchestrator,
            &planner_id,
            &root_id,
            AgentId::WorkflowAutomation,
        );
        assert!(matches!(
            orchestrator.workflow_automation_events(),
            [
                WorkflowAutomationWorkflowEvent::ProposalStarted { .. },
                WorkflowAutomationWorkflowEvent::Expired {
                    stage: WorkflowAutomationStage::Proposal,
                },
            ]
        ));
        assert!(orchestrator.workflow_automation_result().is_none());
        assert!(orchestrator.active_child_task().is_none());
        assert_eq!(orchestrator.task_count(), 2);
        assert_eq!(orchestrator.run_count(), 2);
        assert!(orchestrator
            .workflow_automation
            .as_ref()
            .is_some_and(|workflow| workflow.dispatch.is_none()));
        Ok(())
    }

    #[test]
    fn completed_dispatch_expires_before_take_and_cannot_yield_a_token(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, injected) = TestClock::new();
        let mut orchestrator = AgentOrchestrator::native()?;
        orchestrator.set_workflow_clock_for_test(injected);
        finish_ready_proposal(&mut orchestrator)?;
        assert_eq!(
            orchestrator.workflow_manual_dispatch_availability(),
            WorkflowManualDispatchAvailability::Available
        );

        clock.advance(Duration::from_secs(MAX_WORKFLOW_DURATION_SECONDS));
        assert_eq!(
            orchestrator.workflow_manual_dispatch_availability(),
            WorkflowManualDispatchAvailability::Expired
        );
        assert!(matches!(
            orchestrator.take_workflow_manual_dispatch(),
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchExpired,
            ))
        ));
        let workflow = orchestrator
            .workflow_automation
            .as_ref()
            .ok_or("missing completed workflow")?;
        assert!(workflow.dispatch.is_none());
        assert!(!workflow.dispatch_taken);
        assert!(matches!(
            workflow.events.last(),
            Some(WorkflowAutomationWorkflowEvent::Completed { .. })
        ));
        assert!(orchestrator.manual_workflow_dispatch.is_none());
        Ok(())
    }

    #[test]
    fn manual_lease_expiry_is_child_first_and_cancellation_failure_is_retryable(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (clock, planner_clock) = TestClock::new();
        let mut planner = AgentOrchestrator::native()?;
        planner.set_workflow_clock_for_test(planner_clock);
        finish_ready_proposal(&mut planner)?;
        let dispatch = planner.take_workflow_manual_dispatch()?;

        let destination_clock = {
            let now = Arc::clone(&clock.now);
            Arc::new(move || {
                *now.lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            })
        };
        let mut destination = AgentOrchestrator::native()?;
        destination.set_workflow_clock_for_test(destination_clock);
        let root = destination.start_root("Explicit trusted manual fixture dispatch")?;
        destination.start_manual_workflow(&root, dispatch)?;
        let child = destination
            .active_child_task()
            .ok_or("missing manually dispatched child")?;
        let child_id = child.id().clone();
        let child_context = destination.current_context(&child_id)?;
        let attribution = destination.live_attribution(&child_context)?;
        destination
            .governance
            .seed_pending_approval_for_test(attribution, "expiry-cancel-retry")?;
        destination.governance.fail_next_approval_cancel_for_test();

        clock.advance(Duration::from_secs(MAX_WORKFLOW_DURATION_SECONDS));
        assert_eq!(
            destination.check_manual_workflow_deadline(),
            Err(AgentOrchestratorError::ManualWorkflowExpiryCancellationFailed)
        );
        assert_eq!(
            destination.manual_workflow_dispatch_status(),
            Some(WorkflowManualDispatchStatus::ExpiryCancellationFailed)
        );
        assert_eq!(
            destination.task(&child_id).map(AgentTask::status),
            Some(AgentTaskStatus::Running)
        );
        assert_eq!(
            destination.task(root.task_id()).map(AgentTask::status),
            Some(AgentTaskStatus::WaitingForChild)
        );
        assert_eq!(
            destination
                .manual_workflow_dispatch_events()
                .iter()
                .filter(|event| matches!(
                    event,
                    WorkflowManualDispatchEvent::ExpiryCancellationFailed { .. }
                ))
                .count(),
            1
        );

        assert_eq!(
            destination.check_manual_workflow_deadline(),
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchExpired,
            ))
        );
        assert_eq!(
            destination.manual_workflow_result()?,
            Some(WorkflowManualDispatchStatus::Expired)
        );
        assert_eq!(
            destination.task(&child_id).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert_eq!(
            destination.task(root.task_id()).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert_child_then_root_cancelled(
            &destination,
            &child_id,
            root.task_id(),
            AgentId::Research,
        );
        assert!(destination.active_child_task().is_none());
        assert_eq!(destination.task_count(), 2);
        assert_eq!(destination.run_count(), 2);
        assert!(matches!(
            destination.research_knowledge_events(),
            [
                ResearchKnowledgeWorkflowEvent::ResearchStarted { .. },
                ResearchKnowledgeWorkflowEvent::Cancelled { .. },
            ]
        ));
        assert!(matches!(
            destination.manual_workflow_dispatch_events(),
            [
                WorkflowManualDispatchEvent::Requested { .. },
                WorkflowManualDispatchEvent::Accepted { .. },
                WorkflowManualDispatchEvent::ExpiryCancellationFailed { .. },
                WorkflowManualDispatchEvent::Expired { .. },
            ]
        ));
        assert_eq!(
            destination
                .manual_workflow_dispatch_audit_records()
                .iter()
                .map(WorkflowManualDispatchAuditRecord::status)
                .collect::<Vec<_>>(),
            vec![
                WorkflowManualDispatchStatus::Requested,
                WorkflowManualDispatchStatus::Accepted,
                WorkflowManualDispatchStatus::ExpiryCancellationFailed,
                WorkflowManualDispatchStatus::Expired,
            ]
        );
        Ok(())
    }

    #[test]
    fn manual_lease_is_resampled_after_terminal_preparation_before_event_acceptance(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (planner_clock, injected) = TestClock::new();
        let mut planner = AgentOrchestrator::native()?;
        planner.set_workflow_clock_for_test(injected);
        finish_ready_proposal(&mut planner)?;
        let dispatch = planner.take_workflow_manual_dispatch()?;

        let destination_clock = {
            let now = Arc::clone(&planner_clock.now);
            Arc::new(move || {
                *now.lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            })
        };
        let mut destination = AgentOrchestrator::native()?;
        destination.set_workflow_clock_for_test(destination_clock);
        let root = destination.start_root("Explicit trusted manual fixture dispatch")?;
        destination.start_manual_workflow(&root, dispatch)?;
        let child = destination
            .active_child_task()
            .ok_or("missing manually dispatched child")?;
        let child_id = child.id().clone();
        let child_context = destination.current_context(&child_id)?;
        let base = *planner_clock
            .now
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let reads = Arc::new(AtomicUsize::new(0));
        let reads_for_clock = Arc::clone(&reads);
        destination.set_workflow_clock_for_test(Arc::new(move || {
            if reads_for_clock.fetch_add(1, Ordering::SeqCst) == 0 {
                base
            } else {
                base + Duration::from_secs(MAX_WORKFLOW_DURATION_SECONDS)
            }
        }));

        let terminal = RuntimeEventEnvelope::for_identity(
            child_context.runtime_run_identity(),
            0,
            UntrustedRuntimeEvent::ResponseFailed {
                failure: RuntimeFailure::new(RuntimeFailureCode::ProviderUnavailable, true, None)?,
            },
        );
        assert_eq!(
            destination.accept_runtime_event(&child_id, terminal),
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchExpired,
            ))
        );
        assert!(reads.load(Ordering::SeqCst) >= 2);
        assert_eq!(destination.runtime_event_count(), 0);
        assert_eq!(
            destination.task(&child_id).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert_eq!(
            destination.task(root.task_id()).map(AgentTask::status),
            Some(AgentTaskStatus::Cancelled)
        );
        assert!(destination.active_child_task().is_none());
        assert_eq!(destination.task_count(), 2);
        assert_eq!(destination.run_count(), 2);
        assert!(matches!(
            destination.research_knowledge_events(),
            [
                ResearchKnowledgeWorkflowEvent::ResearchStarted { .. },
                ResearchKnowledgeWorkflowEvent::Cancelled { .. },
            ]
        ));
        assert!(matches!(
            destination.manual_workflow_dispatch_events(),
            [
                WorkflowManualDispatchEvent::Requested { .. },
                WorkflowManualDispatchEvent::Accepted { .. },
                WorkflowManualDispatchEvent::Expired { .. },
            ]
        ));
        Ok(())
    }
}
