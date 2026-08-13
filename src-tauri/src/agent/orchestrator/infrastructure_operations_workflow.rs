//! Private Cloud Infrastructure and Systems Operations workflow lifecycle helpers.

use super::*;
use crate::agent::task::AgentTaskCancellation;

impl<R: AgentRuntime> AgentOrchestrator<R> {
    pub(super) fn infrastructure_root_cancel_stage(&self) -> Option<InfrastructureOperationsStage> {
        self.infrastructure_operations
            .as_ref()
            .filter(|workflow| {
                !matches!(
                    workflow.phase(),
                    InfrastructureOperationsPhase::Completed
                        | InfrastructureOperationsPhase::Failed
                        | InfrastructureOperationsPhase::Cancelled
                )
            })
            .map(|workflow| match workflow.phase() {
                InfrastructureOperationsPhase::FirstRunning(_) => match workflow {
                    InfrastructureOperationsWorkflowState::Cloud(_) => {
                        InfrastructureOperationsStage::CloudAssessment
                    }
                    InfrastructureOperationsWorkflowState::Systems(_) => {
                        InfrastructureOperationsStage::SystemsAssessment
                    }
                },
                InfrastructureOperationsPhase::QaRunning(_) => {
                    InfrastructureOperationsStage::QaValidation
                }
                InfrastructureOperationsPhase::SecurityRunning(_) => {
                    InfrastructureOperationsStage::SecurityReview
                }
                InfrastructureOperationsPhase::SynthesisRunning
                | InfrastructureOperationsPhase::Completed
                | InfrastructureOperationsPhase::Failed
                | InfrastructureOperationsPhase::Cancelled => {
                    InfrastructureOperationsStage::Synthesis
                }
            })
    }

    pub(super) fn preflight_infrastructure_root_cancellation(
        &self,
        stage: Option<InfrastructureOperationsStage>,
    ) -> AgentOrchestratorResult<()> {
        if stage.is_none() {
            return Ok(());
        }
        let workflow = self
            .infrastructure_operations
            .as_ref()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
        let at_capacity = match workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                value.events.len() >= MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS
                    || value.audit.len() >= MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                value.events.len() >= MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS
                    || value.audit.len() >= MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS
            }
        };
        if at_capacity {
            return Err(AgentOrchestratorError::InfrastructureOperationsJournalLimitExceeded);
        }
        Ok(())
    }

    pub(super) fn record_infrastructure_root_cancelled(
        &mut self,
        root_task_id: &AgentTaskId,
        stage: InfrastructureOperationsStage,
    ) -> AgentOrchestratorResult<()> {
        let mut workflow = self
            .infrastructure_operations
            .take()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
        let predecessor = (stage == InfrastructureOperationsStage::Synthesis)
            .then(|| infrastructure_synthesis_predecessor_from_state(&workflow))
            .flatten();
        match &mut workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                push_cloud_infrastructure_transition(
                    value,
                    root_task_id.clone(),
                    predecessor.clone(),
                    stage,
                    CloudInfrastructureWorkflowEvent::Cancelled { stage },
                    InfrastructureOperationsAuditOutcome::Cancelled,
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
                value.phase = InfrastructureOperationsPhase::Cancelled;
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                push_systems_operations_transition(
                    value,
                    root_task_id.clone(),
                    predecessor,
                    stage,
                    SystemsOperationsWorkflowEvent::Cancelled { stage },
                    InfrastructureOperationsAuditOutcome::Cancelled,
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
                value.phase = InfrastructureOperationsPhase::Cancelled;
            }
        }
        self.infrastructure_operations = Some(workflow);
        Ok(())
    }

    pub(super) fn perform_cloud_infrastructure_workflow_start(
        &mut self,
        source_context: &AgentExecutionContext,
        request: CloudInfrastructureWorkflowRequest,
    ) -> AgentOrchestratorResult<CloudInfrastructureWorkflowAcceptance> {
        let attribution = self.live_attribution(source_context)?;
        self.validate_infrastructure_operations_start(
            source_context,
            &attribution,
            AgentWorkflowSelection::CloudInfrastructure,
            AgentId::CloudInfrastructure,
        )?;
        let input = request.build_first_stage_input()?;
        let task = self.prepare_infrastructure_operations_child(
            AgentId::CloudInfrastructure,
            1,
            request.objective(),
            "Return one strict InfrastructureAssessmentV1 JSON object",
        )?;
        let task_id = task.id().clone();
        let runtime_request = self.runtime_request(&task_id, 2, &input)?;
        let context =
            AgentExecutionContext::for_task(&task, self.runtime_id, runtime_request.identity());
        self.consume_root_for_infrastructure_start(&task_id, task)?;
        let mut audit_contexts = BTreeMap::new();
        audit_contexts.insert(
            source_context.task_id().clone(),
            InfrastructureOperationsAttribution::from_execution_context(source_context),
        );
        audit_contexts.insert(
            task_id.clone(),
            InfrastructureOperationsAttribution::from_execution_context(&context),
        );
        let mut state = CloudWorkflowState {
            request,
            phase: InfrastructureOperationsPhase::FirstRunning(task_id.clone()),
            assessment: None,
            qa: None,
            security: None,
            result: None,
            child_count: 1,
            run_attempts: 2,
            accepted_events_by_task: BTreeMap::new(),
            events: Vec::with_capacity(MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS),
            audit: Vec::with_capacity(MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS),
            audit_contexts,
            continuation_failure: None,
        };
        push_cloud_infrastructure_transition(
            &mut state,
            task_id.clone(),
            None,
            InfrastructureOperationsStage::CloudAssessment,
            CloudInfrastructureWorkflowEvent::CloudAssessmentStarted {
                task_id: task_id.clone(),
            },
            InfrastructureOperationsAuditOutcome::Started,
            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
        )?;
        self.infrastructure_operations = Some(InfrastructureOperationsWorkflowState::Cloud(state));
        self.selected_workflow = Some(AgentWorkflowSelection::CloudInfrastructure);
        match self.start_runtime_run(runtime_request) {
            Ok(run) => {
                self.runs.insert(
                    task_id,
                    ActiveRun {
                        run,
                        output: String::new(),
                        next_sequence: 0,
                    },
                );
                Ok(CloudInfrastructureWorkflowAcceptance::CloudAssessmentStarted { context })
            }
            Err(_) => {
                self.record_infrastructure_continuation_failure(
                    InfrastructureOperationsContinuationFailure::CloudStartFailed,
                )?;
                self.terminalize_infrastructure_child_failure(
                    &task_id,
                    AgentTaskFailureCode::RuntimeStartFailed,
                    InfrastructureOperationsPartialFailureCode::RuntimeStartFailed,
                )?;
                let context = self.start_infrastructure_synthesis()?;
                Ok(CloudInfrastructureWorkflowAcceptance::PersonalFallbackStarted { context })
            }
        }
    }

    pub(super) fn perform_systems_operations_workflow_start(
        &mut self,
        source_context: &AgentExecutionContext,
        request: SystemsOperationsWorkflowRequest,
    ) -> AgentOrchestratorResult<SystemsOperationsWorkflowAcceptance> {
        let attribution = self.live_attribution(source_context)?;
        self.validate_infrastructure_operations_start(
            source_context,
            &attribution,
            AgentWorkflowSelection::SystemsOperations,
            AgentId::SystemsOperations,
        )?;
        let input = request.build_first_stage_input()?;
        let task = self.prepare_infrastructure_operations_child(
            AgentId::SystemsOperations,
            1,
            request.objective(),
            "Return one strict OperationalAssessmentV1 JSON object",
        )?;
        let task_id = task.id().clone();
        let runtime_request = self.runtime_request(&task_id, 2, &input)?;
        let context =
            AgentExecutionContext::for_task(&task, self.runtime_id, runtime_request.identity());
        self.consume_root_for_infrastructure_start(&task_id, task)?;
        let mut audit_contexts = BTreeMap::new();
        audit_contexts.insert(
            source_context.task_id().clone(),
            InfrastructureOperationsAttribution::from_execution_context(source_context),
        );
        audit_contexts.insert(
            task_id.clone(),
            InfrastructureOperationsAttribution::from_execution_context(&context),
        );
        let mut state = SystemsOperationsWorkflowState {
            request,
            phase: InfrastructureOperationsPhase::FirstRunning(task_id.clone()),
            assessment: None,
            qa: None,
            security: None,
            result: None,
            child_count: 1,
            run_attempts: 2,
            accepted_events_by_task: BTreeMap::new(),
            events: Vec::with_capacity(MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS),
            audit: Vec::with_capacity(MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS),
            audit_contexts,
            continuation_failure: None,
        };
        push_systems_operations_transition(
            &mut state,
            task_id.clone(),
            None,
            InfrastructureOperationsStage::SystemsAssessment,
            SystemsOperationsWorkflowEvent::SystemsAssessmentStarted {
                task_id: task_id.clone(),
            },
            InfrastructureOperationsAuditOutcome::Started,
            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
        )?;
        self.infrastructure_operations =
            Some(InfrastructureOperationsWorkflowState::Systems(state));
        self.selected_workflow = Some(AgentWorkflowSelection::SystemsOperations);
        match self.start_runtime_run(runtime_request) {
            Ok(run) => {
                self.runs.insert(
                    task_id,
                    ActiveRun {
                        run,
                        output: String::new(),
                        next_sequence: 0,
                    },
                );
                Ok(SystemsOperationsWorkflowAcceptance::SystemsAssessmentStarted { context })
            }
            Err(_) => {
                self.record_infrastructure_continuation_failure(
                    InfrastructureOperationsContinuationFailure::SystemsStartFailed,
                )?;
                self.terminalize_infrastructure_child_failure(
                    &task_id,
                    AgentTaskFailureCode::RuntimeStartFailed,
                    InfrastructureOperationsPartialFailureCode::RuntimeStartFailed,
                )?;
                let context = self.start_infrastructure_synthesis()?;
                Ok(SystemsOperationsWorkflowAcceptance::PersonalFallbackStarted { context })
            }
        }
    }

    pub(super) fn validate_infrastructure_operations_start(
        &self,
        source_context: &AgentExecutionContext,
        attribution: &AgentAttribution,
        selection: AgentWorkflowSelection,
        first_agent: AgentId,
    ) -> AgentOrchestratorResult<()> {
        self.ensure_workflow_selection_available(selection, false)?;
        let task = self
            .tasks
            .get(source_context.task_id())
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        let active = self
            .runs
            .get(source_context.task_id())
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        if self.governance.has_pending_for(attribution) {
            return Err(AgentOrchestratorError::GovernanceApprovalPending);
        }
        if attribution.agent_id() != AgentId::PersonalAssistant
            || attribution.task_id() != attribution.root_task_id().task_id()
            || attribution.parent_task_id().is_some()
            || attribution.depth() != 0
            || self.root_task_id.as_ref() != Some(task.id())
        {
            return Err(AgentOrchestratorError::UnauthorizedSource {
                agent_id: attribution.agent_id(),
            });
        }
        if active.run.status() != RuntimeRunStatus::AwaitingStart || !active.output.is_empty() {
            return Err(AgentOrchestratorError::DelegationAfterRuntimeOutput);
        }
        if self.active_child_task_id.is_some() {
            return Err(AgentOrchestratorError::ActiveChildLimitExceeded);
        }
        for agent_id in [first_agent, AgentId::QaValidation, AgentId::SecurityRisk] {
            let definition = self.registry.get(agent_id)?;
            if definition.activation() != AgentActivation::Initial {
                return Err(AgentOrchestratorError::AgentDeferred { agent_id });
            }
            if definition.memory_profile_id() != AgentMemoryProfileId::MemoryDisabledV1 {
                return Err(
                    AgentOrchestratorError::InfrastructureOperationsMemoryProfileMismatch {
                        agent_id,
                    },
                );
            }
        }
        if self
            .tasks
            .len()
            .checked_add(3)
            .is_none_or(|count| count > MAX_INFRASTRUCTURE_OPERATIONS_TASKS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::TotalChildLimitExceeded);
        }
        if self
            .run_count
            .checked_add(4)
            .is_none_or(|count| count > MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        if self.runtime_event_count >= MAX_RUNTIME_EVENTS_PER_ROOT {
            return Err(AgentOrchestratorError::RuntimeEventLimitExceeded);
        }
        self.ensure_event_capacity(14)
    }

    pub(super) fn prepare_infrastructure_operations_child(
        &self,
        agent_id: AgentId,
        ordinal: u8,
        objective: &str,
        deliverable: &str,
    ) -> AgentOrchestratorResult<AgentTask> {
        let root = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let mut task = AgentTask::new_child(
            AgentTaskId::new(format!("agent-task-child-{}-{ordinal}", self.workflow_sequence))?,
            RootTaskId::from_task_id(root.clone()), ParentTaskId::from_task_id(root),
            self.registry.get(agent_id)?.identity(), AgentTaskObjective::new(objective)?,
            Some(AgentTaskContext::new("Use only the sealed application-owned synthetic fixture and validated predecessor projections")?),
            AgentTaskExpectedDeliverable::new(deliverable)?,
        )?;
        task.start()?;
        Ok(task)
    }

    pub(super) fn consume_root_for_infrastructure_start(
        &mut self,
        child_id: &AgentTaskId,
        child: AgentTask,
    ) -> AgentOrchestratorResult<()> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let mut root_run = self
            .runs
            .remove(&root_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        match root_run.run.cancel() {
            Ok(RuntimeCancellationOutcome::Cancelled)
            | Ok(RuntimeCancellationOutcome::AlreadyTerminal(RuntimeRunStatus::Cancelled)) => {}
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) if status.is_terminal() => {
                self.fail_active_task(&root_id, AgentTaskFailureCode::RuntimeStateMismatch)?;
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
            Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) => {
                self.runs.insert(root_id, root_run);
                return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
            }
            Err(error) => {
                self.runs.insert(root_id, root_run);
                return Err(AgentOrchestratorError::Runtime(error));
            }
        }
        self.tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .wait_for_child()?;
        self.tasks.insert(child_id.clone(), child);
        self.active_child_task_id = Some(child_id.clone());
        self.child_created = true;
        self.run_count = 2;
        self.events.push(AgentOrchestrationEvent::ChildCreated {
            task_id: child_id.clone(),
            parent_task_id: root_id,
        });
        self.events.push(AgentOrchestrationEvent::ChildStarted {
            task_id: child_id.clone(),
        });
        Ok(())
    }

    pub(super) fn record_infrastructure_continuation_failure(
        &mut self,
        failure: InfrastructureOperationsContinuationFailure,
    ) -> AgentOrchestratorResult<()> {
        let workflow = self
            .infrastructure_operations
            .as_mut()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
        let prior = workflow.continuation_failure();
        let combined = match (prior, failure) {
            (
                Some(InfrastructureOperationsContinuationFailure::CloudStartFailed),
                InfrastructureOperationsContinuationFailure::SynthesisStartFailed,
            ) => InfrastructureOperationsContinuationFailure::CloudAndSynthesisStartFailed,
            (
                Some(InfrastructureOperationsContinuationFailure::SystemsStartFailed),
                InfrastructureOperationsContinuationFailure::SynthesisStartFailed,
            ) => InfrastructureOperationsContinuationFailure::SystemsAndSynthesisStartFailed,
            (
                Some(InfrastructureOperationsContinuationFailure::QaStartFailed),
                InfrastructureOperationsContinuationFailure::SecurityStartFailed,
            ) => InfrastructureOperationsContinuationFailure::QaAndSecurityStartFailed,
            (
                Some(InfrastructureOperationsContinuationFailure::QaStartFailed),
                InfrastructureOperationsContinuationFailure::SynthesisStartFailed,
            ) => InfrastructureOperationsContinuationFailure::QaAndSynthesisStartFailed,
            (
                Some(InfrastructureOperationsContinuationFailure::QaAndSecurityStartFailed),
                InfrastructureOperationsContinuationFailure::SynthesisStartFailed,
            ) => InfrastructureOperationsContinuationFailure::QaSecurityAndSynthesisStartFailed,
            (
                Some(InfrastructureOperationsContinuationFailure::SecurityStartFailed),
                InfrastructureOperationsContinuationFailure::SynthesisStartFailed,
            ) => InfrastructureOperationsContinuationFailure::SecurityAndSynthesisStartFailed,
            (_, value) => value,
        };
        match workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                value.continuation_failure = Some(combined)
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                value.continuation_failure = Some(combined)
            }
        }
        Ok(())
    }

    pub(super) fn terminalize_infrastructure_child_failure(
        &mut self,
        task_id: &AgentTaskId,
        task_code: AgentTaskFailureCode,
        workflow_code: InfrastructureOperationsPartialFailureCode,
    ) -> AgentOrchestratorResult<()> {
        let agent_id = self
            .tasks
            .get(task_id)
            .map(AgentTask::agent_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        self.finish_workflow_child_task(
            task_id,
            AgentTaskOutcome::Failed(AgentTaskFailure::new(task_id.clone(), agent_id, task_code)),
        )?;
        let mut workflow = self
            .infrastructure_operations
            .take()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
        match &mut workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                let stage = match value.phase {
                    InfrastructureOperationsPhase::FirstRunning(_) => {
                        value.assessment = Some(CloudInfrastructureStageOutcome::Failed(task_code));
                        value.qa = Some(
                            InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                        );
                        value.security = Some(InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable);
                        InfrastructureOperationsStage::CloudAssessment
                    }
                    InfrastructureOperationsPhase::QaRunning(_) => {
                        value.qa = Some(InfrastructureOperationsQaStageOutcome::Failed(task_code));
                        InfrastructureOperationsStage::QaValidation
                    }
                    InfrastructureOperationsPhase::SecurityRunning(_) => {
                        value.security = Some(
                            InfrastructureOperationsSecurityStageOutcome::Failed(task_code),
                        );
                        InfrastructureOperationsStage::SecurityReview
                    }
                    _ => return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                };
                push_cloud_infrastructure_transition(
                    value,
                    task_id.clone(),
                    None,
                    stage,
                    CloudInfrastructureWorkflowEvent::PartialFailure {
                        stage,
                        code: workflow_code,
                    },
                    InfrastructureOperationsAuditOutcome::PartialFailure(workflow_code),
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                let stage = match value.phase {
                    InfrastructureOperationsPhase::FirstRunning(_) => {
                        value.assessment = Some(SystemsOperationsStageOutcome::Failed(task_code));
                        value.qa = Some(
                            InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                        );
                        value.security = Some(InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable);
                        InfrastructureOperationsStage::SystemsAssessment
                    }
                    InfrastructureOperationsPhase::QaRunning(_) => {
                        value.qa = Some(InfrastructureOperationsQaStageOutcome::Failed(task_code));
                        InfrastructureOperationsStage::QaValidation
                    }
                    InfrastructureOperationsPhase::SecurityRunning(_) => {
                        value.security = Some(
                            InfrastructureOperationsSecurityStageOutcome::Failed(task_code),
                        );
                        InfrastructureOperationsStage::SecurityReview
                    }
                    _ => return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                };
                push_systems_operations_transition(
                    value,
                    task_id.clone(),
                    None,
                    stage,
                    SystemsOperationsWorkflowEvent::PartialFailure {
                        stage,
                        code: workflow_code,
                    },
                    InfrastructureOperationsAuditOutcome::PartialFailure(workflow_code),
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
            }
        }
        self.infrastructure_operations = Some(workflow);
        Ok(())
    }

    pub(super) fn start_infrastructure_synthesis(
        &mut self,
    ) -> AgentOrchestratorResult<AgentExecutionContext> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let input = match self
            .infrastructure_operations
            .as_ref()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?
        {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                value.request.build_synthesis_input(
                    value
                        .assessment
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .security
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                )?
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                value.request.build_synthesis_input(
                    value
                        .assessment
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .security
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                )?
            }
        };
        let next_run = self
            .run_count
            .checked_add(1)
            .ok_or(AgentOrchestratorError::RunLimitExceeded)?;
        if next_run > MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        let request = self.runtime_request(&root_id, next_run, &input)?;
        self.start_infrastructure_synthesis_prepared(request)
    }

    fn start_infrastructure_synthesis_prepared(
        &mut self,
        request: RuntimeTurnRequest,
    ) -> AgentOrchestratorResult<AgentExecutionContext> {
        let root_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        self.tasks
            .get_mut(&root_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?
            .resume_from_child()?;
        let next_run = self
            .run_count
            .checked_add(1)
            .ok_or(AgentOrchestratorError::RunLimitExceeded)?;
        self.run_count = next_run;
        let context = AgentExecutionContext::for_task(
            self.tasks
                .get(&root_id)
                .ok_or(AgentOrchestratorError::TaskNotFound)?,
            self.runtime_id,
            request.identity(),
        );
        let predecessor = self.infrastructure_synthesis_predecessor_task_id();
        {
            let workflow = self
                .infrastructure_operations
                .as_mut()
                .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
            match workflow {
                InfrastructureOperationsWorkflowState::Cloud(value) => {
                    value.run_attempts = next_run;
                    value.audit_contexts.insert(
                        root_id.clone(),
                        InfrastructureOperationsAttribution::from_execution_context(&context),
                    );
                    push_cloud_infrastructure_transition(
                        value,
                        root_id.clone(),
                        predecessor.clone(),
                        InfrastructureOperationsStage::Synthesis,
                        CloudInfrastructureWorkflowEvent::SynthesisStarted {
                            task_id: root_id.clone(),
                        },
                        InfrastructureOperationsAuditOutcome::Started,
                        InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                    )?;
                    value.phase = InfrastructureOperationsPhase::SynthesisRunning;
                }
                InfrastructureOperationsWorkflowState::Systems(value) => {
                    value.run_attempts = next_run;
                    value.audit_contexts.insert(
                        root_id.clone(),
                        InfrastructureOperationsAttribution::from_execution_context(&context),
                    );
                    push_systems_operations_transition(
                        value,
                        root_id.clone(),
                        predecessor,
                        InfrastructureOperationsStage::Synthesis,
                        SystemsOperationsWorkflowEvent::SynthesisStarted {
                            task_id: root_id.clone(),
                        },
                        InfrastructureOperationsAuditOutcome::Started,
                        InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                    )?;
                    value.phase = InfrastructureOperationsPhase::SynthesisRunning;
                }
            }
        }
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
                self.events
                    .push(AgentOrchestrationEvent::ParentResumed { task_id: root_id });
                Ok(context)
            }
            Err(error) => {
                self.record_infrastructure_continuation_failure(
                    InfrastructureOperationsContinuationFailure::SynthesisStartFailed,
                )?;
                self.fail_infrastructure_root_without_run(
                    AgentTaskFailureCode::RuntimeStartFailed,
                    InfrastructureOperationsPartialFailureCode::RuntimeStartFailed,
                )?;
                Err(error)
            }
        }
    }

    fn fail_infrastructure_root_without_run(
        &mut self,
        task_code: AgentTaskFailureCode,
        workflow_code: InfrastructureOperationsPartialFailureCode,
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
            .infrastructure_operations
            .take()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
        let predecessor = infrastructure_synthesis_predecessor_from_state(&workflow);
        match &mut workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                push_cloud_infrastructure_transition(
                    value,
                    root_id.clone(),
                    predecessor.clone(),
                    InfrastructureOperationsStage::Synthesis,
                    CloudInfrastructureWorkflowEvent::PartialFailure {
                        stage: InfrastructureOperationsStage::Synthesis,
                        code: workflow_code,
                    },
                    InfrastructureOperationsAuditOutcome::PartialFailure(workflow_code),
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
                value.phase = InfrastructureOperationsPhase::Failed;
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                push_systems_operations_transition(
                    value,
                    root_id.clone(),
                    predecessor,
                    InfrastructureOperationsStage::Synthesis,
                    SystemsOperationsWorkflowEvent::PartialFailure {
                        stage: InfrastructureOperationsStage::Synthesis,
                        code: workflow_code,
                    },
                    InfrastructureOperationsAuditOutcome::PartialFailure(workflow_code),
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
                value.phase = InfrastructureOperationsPhase::Failed;
            }
        }
        self.infrastructure_operations = Some(workflow);
        self.cleanup_terminal_task(&root_id);
        Ok(())
    }
}

impl<R: AgentRuntime> AgentOrchestrator<R> {
    pub(super) fn cancel_infrastructure_child(
        &mut self,
        child_task_id: &AgentTaskId,
        root_is_cancelling: bool,
    ) -> AgentOrchestratorResult<AgentTaskCancellationOutcome> {
        self.cancel_pending_governance(child_task_id)?;
        let disposition = self.cancel_run(child_task_id)?;
        let (stage, agent_id, predecessor) = match self
            .infrastructure_operations
            .as_ref()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?
            .phase()
        {
            InfrastructureOperationsPhase::FirstRunning(active) if active == child_task_id => {
                let (stage, agent_id) = match self
                    .infrastructure_operations
                    .as_ref()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?
                {
                    InfrastructureOperationsWorkflowState::Cloud(_) => (
                        InfrastructureOperationsStage::CloudAssessment,
                        AgentId::CloudInfrastructure,
                    ),
                    InfrastructureOperationsWorkflowState::Systems(_) => (
                        InfrastructureOperationsStage::SystemsAssessment,
                        AgentId::SystemsOperations,
                    ),
                };
                (stage, agent_id, None)
            }
            InfrastructureOperationsPhase::QaRunning(active) if active == child_task_id => (
                InfrastructureOperationsStage::QaValidation,
                AgentId::QaValidation,
                self.infrastructure_first_task_id(),
            ),
            InfrastructureOperationsPhase::SecurityRunning(active) if active == child_task_id => (
                InfrastructureOperationsStage::SecurityReview,
                AgentId::SecurityRisk,
                self.infrastructure_qa_task_id()
                    .or_else(|| self.infrastructure_first_task_id()),
            ),
            _ => return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
        };
        if let RunCancellationDisposition::UnexpectedTerminal(status) = disposition {
            self.terminalize_infrastructure_child_failure(
                child_task_id,
                AgentTaskFailureCode::RuntimeStateMismatch,
                InfrastructureOperationsPartialFailureCode::RuntimeFailed,
            )?;
            if !root_is_cancelling {
                let _ = match agent_id {
                    AgentId::CloudInfrastructure
                    | AgentId::SystemsOperations
                    | AgentId::SecurityRisk => self.start_infrastructure_synthesis().map(|_| ()),
                    AgentId::QaValidation => self.start_infrastructure_security().map(|_| ()),
                    _ => Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                };
            }
            return Err(AgentOrchestratorError::UnexpectedRuntimeStatus { status });
        }

        if root_is_cancelling {
            let child = self
                .tasks
                .get_mut(child_task_id)
                .ok_or(AgentOrchestratorError::TaskNotFound)?;
            let outcome = child.cancel();
            if outcome == AgentTaskCancellationOutcome::Cancelled {
                self.events.push(AgentOrchestrationEvent::TaskCancelled {
                    task_id: child_task_id.clone(),
                    agent_id,
                });
                self.active_child_task_id = None;
                self.cleanup_terminal_task(child_task_id);
                let workflow = self
                    .infrastructure_operations
                    .as_mut()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                match workflow {
                    InfrastructureOperationsWorkflowState::Cloud(value) => match stage {
                        InfrastructureOperationsStage::CloudAssessment => {
                            value.assessment = Some(CloudInfrastructureStageOutcome::Cancelled);
                            value.qa = Some(
                                InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                            );
                            value.security = Some(
                                InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable,
                            );
                        }
                        InfrastructureOperationsStage::QaValidation => {
                            value.qa = Some(InfrastructureOperationsQaStageOutcome::Cancelled);
                        }
                        InfrastructureOperationsStage::SecurityReview => {
                            value.security =
                                Some(InfrastructureOperationsSecurityStageOutcome::Cancelled);
                        }
                        InfrastructureOperationsStage::SystemsAssessment
                        | InfrastructureOperationsStage::Synthesis => {
                            return Err(
                                AgentOrchestratorError::InfrastructureOperationsStageMismatch,
                            )
                        }
                    },
                    InfrastructureOperationsWorkflowState::Systems(value) => match stage {
                        InfrastructureOperationsStage::SystemsAssessment => {
                            value.assessment = Some(SystemsOperationsStageOutcome::Cancelled);
                            value.qa = Some(
                                InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                            );
                            value.security = Some(
                                InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable,
                            );
                        }
                        InfrastructureOperationsStage::QaValidation => {
                            value.qa = Some(InfrastructureOperationsQaStageOutcome::Cancelled);
                        }
                        InfrastructureOperationsStage::SecurityReview => {
                            value.security =
                                Some(InfrastructureOperationsSecurityStageOutcome::Cancelled);
                        }
                        InfrastructureOperationsStage::CloudAssessment
                        | InfrastructureOperationsStage::Synthesis => {
                            return Err(
                                AgentOrchestratorError::InfrastructureOperationsStageMismatch,
                            )
                        }
                    },
                }
            }
            return Ok(outcome);
        }

        let cancellation = AgentTaskCancellation::new(child_task_id.clone(), agent_id);
        self.finish_workflow_child_task(child_task_id, AgentTaskOutcome::Cancelled(cancellation))?;
        let mut workflow = self
            .infrastructure_operations
            .take()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
        match &mut workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                match stage {
                    InfrastructureOperationsStage::CloudAssessment => {
                        value.assessment = Some(CloudInfrastructureStageOutcome::Cancelled);
                        value.qa = Some(
                            InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                        );
                        value.security = Some(
                            InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable,
                        );
                    }
                    InfrastructureOperationsStage::QaValidation => {
                        value.qa = Some(InfrastructureOperationsQaStageOutcome::Cancelled);
                    }
                    InfrastructureOperationsStage::SecurityReview => {
                        value.security =
                            Some(InfrastructureOperationsSecurityStageOutcome::Cancelled);
                    }
                    InfrastructureOperationsStage::SystemsAssessment
                    | InfrastructureOperationsStage::Synthesis => {
                        return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch)
                    }
                }
                push_cloud_infrastructure_transition(
                    value,
                    child_task_id.clone(),
                    predecessor.clone(),
                    stage,
                    CloudInfrastructureWorkflowEvent::PartialFailure {
                        stage,
                        code: InfrastructureOperationsPartialFailureCode::Cancelled,
                    },
                    InfrastructureOperationsAuditOutcome::Cancelled,
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                match stage {
                    InfrastructureOperationsStage::SystemsAssessment => {
                        value.assessment = Some(SystemsOperationsStageOutcome::Cancelled);
                        value.qa = Some(
                            InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                        );
                        value.security = Some(
                            InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable,
                        );
                    }
                    InfrastructureOperationsStage::QaValidation => {
                        value.qa = Some(InfrastructureOperationsQaStageOutcome::Cancelled);
                    }
                    InfrastructureOperationsStage::SecurityReview => {
                        value.security =
                            Some(InfrastructureOperationsSecurityStageOutcome::Cancelled);
                    }
                    InfrastructureOperationsStage::CloudAssessment
                    | InfrastructureOperationsStage::Synthesis => {
                        return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch)
                    }
                }
                push_systems_operations_transition(
                    value,
                    child_task_id.clone(),
                    predecessor,
                    stage,
                    SystemsOperationsWorkflowEvent::PartialFailure {
                        stage,
                        code: InfrastructureOperationsPartialFailureCode::Cancelled,
                    },
                    InfrastructureOperationsAuditOutcome::Cancelled,
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
            }
        }
        self.infrastructure_operations = Some(workflow);
        match agent_id {
            AgentId::CloudInfrastructure | AgentId::SystemsOperations | AgentId::SecurityRisk => {
                self.start_infrastructure_synthesis()?;
            }
            AgentId::QaValidation => {
                self.start_infrastructure_security()?;
            }
            _ => return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
        }
        Ok(AgentTaskCancellationOutcome::Cancelled)
    }
}

impl<R: AgentRuntime> AgentOrchestrator<R> {
    pub(super) fn fail_infrastructure_task(
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
            return self.fail_infrastructure_root_without_run(
                code,
                InfrastructureOperationsPartialFailureCode::RuntimeFailed,
            );
        }
        let agent_id = self
            .tasks
            .get(task_id)
            .map(AgentTask::agent_id)
            .ok_or(AgentOrchestratorError::TaskNotFound)?;
        self.terminalize_infrastructure_child_failure(
            task_id,
            code,
            InfrastructureOperationsPartialFailureCode::RuntimeFailed,
        )?;
        match agent_id {
            AgentId::CloudInfrastructure | AgentId::SystemsOperations | AgentId::SecurityRisk => {
                self.start_infrastructure_synthesis().map(|_| ())
            }
            AgentId::QaValidation => self.start_infrastructure_security().map(|_| ()),
            _ => Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
        }
    }
}

pub(super) fn push_cloud_infrastructure_transition(
    workflow: &mut CloudWorkflowState,
    task_id: AgentTaskId,
    predecessor_task_id: Option<AgentTaskId>,
    stage: InfrastructureOperationsStage,
    event: CloudInfrastructureWorkflowEvent,
    outcome: InfrastructureOperationsAuditOutcome,
    capability: InfrastructureOperationsCapabilityAuditDisposition,
) -> AgentOrchestratorResult<()> {
    if workflow.events.len() >= MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS
        || workflow.audit.len() >= MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS
    {
        return Err(AgentOrchestratorError::InfrastructureOperationsJournalLimitExceeded);
    }
    let attribution = workflow
        .audit_contexts
        .get(&task_id)
        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?
        .clone();
    let sequence = u8::try_from(workflow.audit.len())
        .map_err(|_| AgentOrchestratorError::InfrastructureOperationsJournalLimitExceeded)?;
    workflow.events.push(event);
    workflow
        .audit
        .push(InfrastructureOperationsAttributionRecord::new(
            sequence,
            attribution,
            predecessor_task_id,
            stage,
            outcome,
            capability,
        ));
    Ok(())
}

pub(super) fn push_systems_operations_transition(
    workflow: &mut SystemsOperationsWorkflowState,
    task_id: AgentTaskId,
    predecessor_task_id: Option<AgentTaskId>,
    stage: InfrastructureOperationsStage,
    event: SystemsOperationsWorkflowEvent,
    outcome: InfrastructureOperationsAuditOutcome,
    capability: InfrastructureOperationsCapabilityAuditDisposition,
) -> AgentOrchestratorResult<()> {
    if workflow.events.len() >= MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS
        || workflow.audit.len() >= MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS
    {
        return Err(AgentOrchestratorError::InfrastructureOperationsJournalLimitExceeded);
    }
    let attribution = workflow
        .audit_contexts
        .get(&task_id)
        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?
        .clone();
    let sequence = u8::try_from(workflow.audit.len())
        .map_err(|_| AgentOrchestratorError::InfrastructureOperationsJournalLimitExceeded)?;
    workflow.events.push(event);
    workflow
        .audit
        .push(InfrastructureOperationsAttributionRecord::new(
            sequence,
            attribution,
            predecessor_task_id,
            stage,
            outcome,
            capability,
        ));
    Ok(())
}

pub(super) fn infrastructure_synthesis_predecessor_from_state(
    workflow: &InfrastructureOperationsWorkflowState,
) -> Option<AgentTaskId> {
    let contexts = match workflow {
        InfrastructureOperationsWorkflowState::Cloud(value) => &value.audit_contexts,
        InfrastructureOperationsWorkflowState::Systems(value) => &value.audit_contexts,
    };
    [
        AgentId::SecurityRisk,
        AgentId::QaValidation,
        AgentId::CloudInfrastructure,
        AgentId::SystemsOperations,
    ]
    .into_iter()
    .find_map(|agent_id| {
        contexts
            .iter()
            .find(|(_, attribution)| attribution.agent_id() == agent_id)
            .map(|(task_id, _)| task_id.clone())
    })
}

impl<R: AgentRuntime> AgentOrchestrator<R> {
    pub(super) fn preflight_infrastructure_operations_event_capacity(
        &self,
        task_id: &AgentTaskId,
        event: &UntrustedRuntimeEvent,
    ) -> AgentOrchestratorResult<()> {
        let Some(workflow) = self.infrastructure_operations.as_ref() else {
            return Ok(());
        };
        let root = self
            .root_task_id
            .as_ref()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        if workflow.active_event_task(root) != Some(task_id) {
            return Ok(());
        }
        let accepted = match workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                value.accepted_events_by_task.get(task_id)
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                value.accepted_events_by_task.get(task_id)
            }
        }
        .copied()
        .unwrap_or(0);
        let terminal = matches!(
            event,
            UntrustedRuntimeEvent::ResponseCompleted | UntrustedRuntimeEvent::ResponseFailed { .. }
        );
        let maximum = if terminal {
            MAX_INFRASTRUCTURE_OPERATIONS_EVENTS_PER_RUN
        } else {
            MAX_INFRASTRUCTURE_OPERATIONS_EVENTS_PER_RUN - 1
        };
        if accepted >= maximum {
            return Err(AgentOrchestratorError::InfrastructureOperationsEventLimitExceeded);
        }
        if !terminal {
            return Ok(());
        }
        let (remaining_runs, remaining_children, workflow_records, orchestration_events) =
            match workflow.phase() {
                InfrastructureOperationsPhase::FirstRunning(_) => {
                    (3_u8, 2_usize, 9_usize, 12_usize)
                }
                InfrastructureOperationsPhase::QaRunning(_) => (2, 1, 6, 8),
                InfrastructureOperationsPhase::SecurityRunning(_) => (1, 0, 3, 4),
                InfrastructureOperationsPhase::SynthesisRunning => (0, 0, 1, 1),
                InfrastructureOperationsPhase::Completed
                | InfrastructureOperationsPhase::Failed
                | InfrastructureOperationsPhase::Cancelled => {
                    return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch)
                }
            };
        let reserved_runtime_events = usize::from(remaining_runs)
            .checked_mul(MAX_INFRASTRUCTURE_OPERATIONS_EVENTS_PER_RUN as usize)
            .and_then(|value| value.checked_add(1))
            .ok_or(AgentOrchestratorError::RuntimeEventLimitExceeded)?;
        if self
            .runtime_event_count
            .checked_add(reserved_runtime_events)
            .is_none_or(|value| value > MAX_RUNTIME_EVENTS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::RuntimeEventLimitExceeded);
        }
        if self
            .tasks
            .len()
            .checked_add(remaining_children)
            .is_none_or(|value| value > MAX_INFRASTRUCTURE_OPERATIONS_TASKS_PER_ROOT)
            || self
                .run_count
                .checked_add(remaining_runs)
                .is_none_or(|value| value > MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT)
        {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        self.ensure_event_capacity(orchestration_events)?;
        let journal_has_capacity =
            match workflow {
                InfrastructureOperationsWorkflowState::Cloud(value) => value
                    .events
                    .len()
                    .checked_add(workflow_records)
                    .is_some_and(|count| {
                        count <= MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS
                            && value.audit.len().checked_add(workflow_records).is_some_and(
                                |audit| audit <= MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS,
                            )
                    }),
                InfrastructureOperationsWorkflowState::Systems(value) => value
                    .events
                    .len()
                    .checked_add(workflow_records)
                    .is_some_and(|count| {
                        count <= MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS
                            && value.audit.len().checked_add(workflow_records).is_some_and(
                                |audit| audit <= MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS,
                            )
                    }),
            };
        if !journal_has_capacity {
            return Err(AgentOrchestratorError::InfrastructureOperationsJournalLimitExceeded);
        }
        Ok(())
    }

    pub(super) fn record_infrastructure_operations_event_acceptance(
        &mut self,
        task_id: &AgentTaskId,
    ) {
        let Some(workflow) = self.infrastructure_operations.as_mut() else {
            return;
        };
        let Some(root) = self.root_task_id.as_ref() else {
            return;
        };
        if workflow.active_event_task(root) != Some(task_id) {
            return;
        }
        let map = match workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                &mut value.accepted_events_by_task
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                &mut value.accepted_events_by_task
            }
        };
        *map.entry(task_id.clone()).or_insert(0) += 1;
    }

    pub(super) fn prepare_infrastructure_operations_terminal(
        &self,
        task_id: &AgentTaskId,
        event: &UntrustedRuntimeEvent,
    ) -> AgentOrchestratorResult<Option<PreparedInfrastructureOperationsTerminal>> {
        let Some(workflow) = self.infrastructure_operations.as_ref() else {
            return Ok(None);
        };
        if !matches!(
            event,
            UntrustedRuntimeEvent::ResponseCompleted | UntrustedRuntimeEvent::ResponseFailed { .. }
        ) {
            return Ok(None);
        }
        let root = self
            .root_task_id
            .as_ref()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        if workflow.active_event_task(root) != Some(task_id) {
            return Ok(None);
        }
        let active = self
            .runs
            .get(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        let next_run = self
            .run_count
            .checked_add(1)
            .ok_or(AgentOrchestratorError::RunLimitExceeded)?;
        let terminal = match (workflow, workflow.phase(), event) {
            (
                InfrastructureOperationsWorkflowState::Cloud(value),
                InfrastructureOperationsPhase::FirstRunning(active_id),
                _,
            ) if active_id == task_id => {
                let (task_outcome, first, failure) = match event {
                    UntrustedRuntimeEvent::ResponseCompleted => match value
                        .request
                        .parse_assessment(task_id.clone(), &active.output)
                    {
                        Ok(assessment) => {
                            let quality = assessment.quality();
                            (AgentTaskOutcome::Completed(AgentTaskResult::new(task_id.clone(), AgentId::CloudInfrastructure, AgentTaskOutput::new(active.output.clone())?)), InfrastructureOperationsFirstOutcome::Cloud(CloudInfrastructureStageOutcome::Completed(Box::new(assessment))), (quality == InfrastructureOperationsAssessmentQuality::PartialDeniedCapability).then_some(InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability))
                        }
                        Err(_) => (
                            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                                task_id.clone(),
                                AgentId::CloudInfrastructure,
                                AgentTaskFailureCode::RuntimeOutputInvalid,
                            )),
                            InfrastructureOperationsFirstOutcome::Cloud(
                                CloudInfrastructureStageOutcome::Failed(
                                    AgentTaskFailureCode::RuntimeOutputInvalid,
                                ),
                            ),
                            Some(
                                InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                            ),
                        ),
                    },
                    UntrustedRuntimeEvent::ResponseFailed { failure }
                        if failure.code() == RuntimeFailureCode::Cancelled =>
                    {
                        (
                            AgentTaskOutcome::Cancelled(AgentTaskCancellation::new(
                                task_id.clone(),
                                AgentId::CloudInfrastructure,
                            )),
                            InfrastructureOperationsFirstOutcome::Cloud(
                                CloudInfrastructureStageOutcome::Cancelled,
                            ),
                            Some(InfrastructureOperationsPartialFailureCode::Cancelled),
                        )
                    }
                    UntrustedRuntimeEvent::ResponseFailed { failure } => {
                        let code = AgentTaskFailureCode::RuntimeReported(failure.code());
                        (
                            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                                task_id.clone(),
                                AgentId::CloudInfrastructure,
                                code,
                            )),
                            InfrastructureOperationsFirstOutcome::Cloud(
                                CloudInfrastructureStageOutcome::Failed(code),
                            ),
                            Some(InfrastructureOperationsPartialFailureCode::RuntimeFailed),
                        )
                    }
                    _ => return Ok(None),
                };
                let next = match &first {
                    InfrastructureOperationsFirstOutcome::Cloud(
                        CloudInfrastructureStageOutcome::Completed(assessment),
                    ) => self.prepare_infrastructure_qa_next(
                        &value.request.build_qa_input(assessment.transfer())?,
                        next_run,
                    )?,
                    _ => PreparedInfrastructureOperationsNext::Synthesis(
                        self.prepare_infrastructure_synthesis_request_for_first(&first, next_run)?,
                    ),
                };
                PreparedInfrastructureOperationsTerminal::First {
                    task_outcome,
                    first,
                    failure,
                    next,
                }
            }
            (
                InfrastructureOperationsWorkflowState::Systems(value),
                InfrastructureOperationsPhase::FirstRunning(active_id),
                _,
            ) if active_id == task_id => {
                let (task_outcome, first, failure) = match event {
                    UntrustedRuntimeEvent::ResponseCompleted => match value
                        .request
                        .parse_assessment(task_id.clone(), &active.output)
                    {
                        Ok(assessment) => {
                            let quality = assessment.quality();
                            (AgentTaskOutcome::Completed(AgentTaskResult::new(task_id.clone(), AgentId::SystemsOperations, AgentTaskOutput::new(active.output.clone())?)), InfrastructureOperationsFirstOutcome::Systems(SystemsOperationsStageOutcome::Completed(Box::new(assessment))), (quality == InfrastructureOperationsAssessmentQuality::PartialDeniedCapability).then_some(InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability))
                        }
                        Err(_) => (
                            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                                task_id.clone(),
                                AgentId::SystemsOperations,
                                AgentTaskFailureCode::RuntimeOutputInvalid,
                            )),
                            InfrastructureOperationsFirstOutcome::Systems(
                                SystemsOperationsStageOutcome::Failed(
                                    AgentTaskFailureCode::RuntimeOutputInvalid,
                                ),
                            ),
                            Some(
                                InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                            ),
                        ),
                    },
                    UntrustedRuntimeEvent::ResponseFailed { failure }
                        if failure.code() == RuntimeFailureCode::Cancelled =>
                    {
                        (
                            AgentTaskOutcome::Cancelled(AgentTaskCancellation::new(
                                task_id.clone(),
                                AgentId::SystemsOperations,
                            )),
                            InfrastructureOperationsFirstOutcome::Systems(
                                SystemsOperationsStageOutcome::Cancelled,
                            ),
                            Some(InfrastructureOperationsPartialFailureCode::Cancelled),
                        )
                    }
                    UntrustedRuntimeEvent::ResponseFailed { failure } => {
                        let code = AgentTaskFailureCode::RuntimeReported(failure.code());
                        (
                            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                                task_id.clone(),
                                AgentId::SystemsOperations,
                                code,
                            )),
                            InfrastructureOperationsFirstOutcome::Systems(
                                SystemsOperationsStageOutcome::Failed(code),
                            ),
                            Some(InfrastructureOperationsPartialFailureCode::RuntimeFailed),
                        )
                    }
                    _ => return Ok(None),
                };
                let next = match &first {
                    InfrastructureOperationsFirstOutcome::Systems(
                        SystemsOperationsStageOutcome::Completed(assessment),
                    ) => self.prepare_infrastructure_qa_next(
                        &value.request.build_qa_input(assessment.transfer())?,
                        next_run,
                    )?,
                    _ => PreparedInfrastructureOperationsNext::Synthesis(
                        self.prepare_infrastructure_synthesis_request_for_first(&first, next_run)?,
                    ),
                };
                PreparedInfrastructureOperationsTerminal::First {
                    task_outcome,
                    first,
                    failure,
                    next,
                }
            }
            (_, InfrastructureOperationsPhase::QaRunning(active_id), _) if active_id == task_id => {
                let (assessment_task_id, refs, transfer) =
                    self.infrastructure_assessment_projection()?;
                let (task_outcome, qa, failure) = match event {
                    UntrustedRuntimeEvent::ResponseCompleted => {
                        let parsed = match workflow {
                            InfrastructureOperationsWorkflowState::Cloud(value) => {
                                value.request.parse_validation_report(
                                    task_id.clone(),
                                    assessment_task_id.clone(),
                                    &refs,
                                    &active.output,
                                )
                            }
                            InfrastructureOperationsWorkflowState::Systems(value) => {
                                value.request.parse_validation_report(
                                    task_id.clone(),
                                    assessment_task_id.clone(),
                                    &refs,
                                    &active.output,
                                )
                            }
                        };
                        match parsed {
                            Ok(report) => { let incomplete = report.conclusion() != InfrastructureOperationsValidationConclusion::Adequate; (AgentTaskOutcome::Completed(AgentTaskResult::new(task_id.clone(), AgentId::QaValidation, AgentTaskOutput::new(active.output.clone())?)), InfrastructureOperationsQaStageOutcome::Completed(report), incomplete.then_some(InfrastructureOperationsPartialFailureCode::QaIncomplete)) }
                            Err(_) => (AgentTaskOutcome::Failed(AgentTaskFailure::new(task_id.clone(), AgentId::QaValidation, AgentTaskFailureCode::RuntimeOutputInvalid)), InfrastructureOperationsQaStageOutcome::Failed(AgentTaskFailureCode::RuntimeOutputInvalid), Some(InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput)),
                        }
                    }
                    UntrustedRuntimeEvent::ResponseFailed { failure }
                        if failure.code() == RuntimeFailureCode::Cancelled =>
                    {
                        (
                            AgentTaskOutcome::Cancelled(AgentTaskCancellation::new(
                                task_id.clone(),
                                AgentId::QaValidation,
                            )),
                            InfrastructureOperationsQaStageOutcome::Cancelled,
                            Some(InfrastructureOperationsPartialFailureCode::Cancelled),
                        )
                    }
                    UntrustedRuntimeEvent::ResponseFailed { failure } => {
                        let code = AgentTaskFailureCode::RuntimeReported(failure.code());
                        (
                            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                                task_id.clone(),
                                AgentId::QaValidation,
                                code,
                            )),
                            InfrastructureOperationsQaStageOutcome::Failed(code),
                            Some(InfrastructureOperationsPartialFailureCode::QaUnavailable),
                        )
                    }
                    _ => return Ok(None),
                };
                let input = match workflow {
                    InfrastructureOperationsWorkflowState::Cloud(value) => {
                        value.request.build_security_input(&transfer, &qa)?
                    }
                    InfrastructureOperationsWorkflowState::Systems(value) => {
                        value.request.build_security_input(&transfer, &qa)?
                    }
                };
                PreparedInfrastructureOperationsTerminal::Qa {
                    task_outcome,
                    qa,
                    failure,
                    next: self.prepare_infrastructure_security_next(&input, next_run)?,
                }
            }
            (_, InfrastructureOperationsPhase::SecurityRunning(active_id), _)
                if active_id == task_id =>
            {
                let (assessment_task_id, refs) =
                    self.infrastructure_assessment_evidence_projection()?;
                let qa = match workflow {
                    InfrastructureOperationsWorkflowState::Cloud(value) => value.qa.as_ref(),
                    InfrastructureOperationsWorkflowState::Systems(value) => value.qa.as_ref(),
                }
                .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?;
                let (task_outcome, security, failure) = match event {
                    UntrustedRuntimeEvent::ResponseCompleted => {
                        let parsed = match workflow {
                            InfrastructureOperationsWorkflowState::Cloud(value) => {
                                value.request.parse_risk_assessment(
                                    task_id.clone(),
                                    assessment_task_id,
                                    &refs,
                                    qa,
                                    &active.output,
                                )
                            }
                            InfrastructureOperationsWorkflowState::Systems(value) => {
                                value.request.parse_risk_assessment(
                                    task_id.clone(),
                                    assessment_task_id,
                                    &refs,
                                    qa,
                                    &active.output,
                                )
                            }
                        };
                        match parsed {
                            Ok(risk) => (AgentTaskOutcome::Completed(AgentTaskResult::new(task_id.clone(), AgentId::SecurityRisk, AgentTaskOutput::new(active.output.clone())?)), InfrastructureOperationsSecurityStageOutcome::Completed(risk), None),
                            Err(_) => (AgentTaskOutcome::Failed(AgentTaskFailure::new(task_id.clone(), AgentId::SecurityRisk, AgentTaskFailureCode::RuntimeOutputInvalid)), InfrastructureOperationsSecurityStageOutcome::Failed(AgentTaskFailureCode::RuntimeOutputInvalid), Some(InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput)),
                        }
                    }
                    UntrustedRuntimeEvent::ResponseFailed { failure }
                        if failure.code() == RuntimeFailureCode::Cancelled =>
                    {
                        (
                            AgentTaskOutcome::Cancelled(AgentTaskCancellation::new(
                                task_id.clone(),
                                AgentId::SecurityRisk,
                            )),
                            InfrastructureOperationsSecurityStageOutcome::Cancelled,
                            Some(InfrastructureOperationsPartialFailureCode::Cancelled),
                        )
                    }
                    UntrustedRuntimeEvent::ResponseFailed { failure } => {
                        let code = AgentTaskFailureCode::RuntimeReported(failure.code());
                        (
                            AgentTaskOutcome::Failed(AgentTaskFailure::new(
                                task_id.clone(),
                                AgentId::SecurityRisk,
                                code,
                            )),
                            InfrastructureOperationsSecurityStageOutcome::Failed(code),
                            Some(InfrastructureOperationsPartialFailureCode::SecurityUnavailable),
                        )
                    }
                    _ => return Ok(None),
                };
                PreparedInfrastructureOperationsTerminal::Security {
                    task_outcome,
                    synthesis_request: self
                        .prepare_infrastructure_synthesis_request_with_security(
                            &security, next_run,
                        )?,
                    security,
                    failure,
                }
            }
            (
                InfrastructureOperationsWorkflowState::Cloud(value),
                InfrastructureOperationsPhase::SynthesisRunning,
                _,
            ) if root == task_id => match event {
                UntrustedRuntimeEvent::ResponseCompleted => match value.request.parse_synthesis(
                    value
                        .assessment
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .security
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    &active.output,
                ) {
                    Ok(synthesis) => {
                        PreparedInfrastructureOperationsTerminal::CloudSynthesisCompleted {
                            output: AgentTaskOutput::new(synthesis.summary().to_owned())?,
                            synthesis,
                        }
                    }
                    Err(_) => PreparedInfrastructureOperationsTerminal::SynthesisFailed {
                        task_code: AgentTaskFailureCode::RuntimeOutputInvalid,
                        workflow_code:
                            InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                    },
                },
                UntrustedRuntimeEvent::ResponseFailed { failure }
                    if failure.code() == RuntimeFailureCode::Cancelled =>
                {
                    PreparedInfrastructureOperationsTerminal::SynthesisCancelled
                }
                UntrustedRuntimeEvent::ResponseFailed { failure } => {
                    PreparedInfrastructureOperationsTerminal::SynthesisFailed {
                        task_code: AgentTaskFailureCode::RuntimeReported(failure.code()),
                        workflow_code: InfrastructureOperationsPartialFailureCode::RuntimeFailed,
                    }
                }
                _ => return Ok(None),
            },
            (
                InfrastructureOperationsWorkflowState::Systems(value),
                InfrastructureOperationsPhase::SynthesisRunning,
                _,
            ) if root == task_id => match event {
                UntrustedRuntimeEvent::ResponseCompleted => match value.request.parse_synthesis(
                    value
                        .assessment
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .security
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    &active.output,
                ) {
                    Ok(synthesis) => {
                        PreparedInfrastructureOperationsTerminal::SystemsSynthesisCompleted {
                            output: AgentTaskOutput::new(synthesis.summary().to_owned())?,
                            synthesis,
                        }
                    }
                    Err(_) => PreparedInfrastructureOperationsTerminal::SynthesisFailed {
                        task_code: AgentTaskFailureCode::RuntimeOutputInvalid,
                        workflow_code:
                            InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput,
                    },
                },
                UntrustedRuntimeEvent::ResponseFailed { failure }
                    if failure.code() == RuntimeFailureCode::Cancelled =>
                {
                    PreparedInfrastructureOperationsTerminal::SynthesisCancelled
                }
                UntrustedRuntimeEvent::ResponseFailed { failure } => {
                    PreparedInfrastructureOperationsTerminal::SynthesisFailed {
                        task_code: AgentTaskFailureCode::RuntimeReported(failure.code()),
                        workflow_code: InfrastructureOperationsPartialFailureCode::RuntimeFailed,
                    }
                }
                _ => return Ok(None),
            },
            _ => return Ok(None),
        };
        Ok(Some(terminal))
    }

    fn prepare_infrastructure_qa_next(
        &self,
        input: &str,
        run_ordinal: u8,
    ) -> AgentOrchestratorResult<PreparedInfrastructureOperationsNext> {
        if run_ordinal > MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        let task = self.prepare_infrastructure_operations_child(
            AgentId::QaValidation,
            2,
            "Validate the exact sealed fixture assessment against application criteria",
            "Return one strict ValidationReportV1 JSON object",
        )?;
        let request = self.runtime_request(task.id(), run_ordinal, input)?;
        let context = AgentExecutionContext::for_task(&task, self.runtime_id, request.identity());
        Ok(PreparedInfrastructureOperationsNext::Child {
            task: Box::new(task),
            request,
            attribution: InfrastructureOperationsAttribution::from_execution_context(&context),
        })
    }

    fn prepare_infrastructure_security_next(
        &self,
        input: &str,
        run_ordinal: u8,
    ) -> AgentOrchestratorResult<PreparedInfrastructureOperationsNext> {
        if run_ordinal > MAX_INFRASTRUCTURE_OPERATIONS_RUNTIME_RUNS_PER_ROOT {
            return Err(AgentOrchestratorError::RunLimitExceeded);
        }
        let task = self.prepare_infrastructure_operations_child(
            AgentId::SecurityRisk,
            3,
            "Assess bounded security and operational risk for the validated fixture proposal",
            "Return one strict RiskAssessmentV1 JSON object",
        )?;
        let request = self.runtime_request(task.id(), run_ordinal, input)?;
        let context = AgentExecutionContext::for_task(&task, self.runtime_id, request.identity());
        Ok(PreparedInfrastructureOperationsNext::Child {
            task: Box::new(task),
            request,
            attribution: InfrastructureOperationsAttribution::from_execution_context(&context),
        })
    }

    fn infrastructure_assessment_projection(
        &self,
    ) -> AgentOrchestratorResult<(
        AgentTaskId,
        Vec<InfrastructureOperationsEvidenceRef>,
        String,
    )> {
        match self
            .infrastructure_operations
            .as_ref()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?
        {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                match value.assessment.as_ref() {
                    Some(CloudInfrastructureStageOutcome::Completed(assessment)) => Ok((
                        assessment.task_id().clone(),
                        assessment.available_references(),
                        assessment.transfer().to_owned(),
                    )),
                    _ => Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                }
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                match value.assessment.as_ref() {
                    Some(SystemsOperationsStageOutcome::Completed(assessment)) => Ok((
                        assessment.task_id().clone(),
                        assessment.available_references(),
                        assessment.transfer().to_owned(),
                    )),
                    _ => Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                }
            }
        }
    }

    fn infrastructure_assessment_evidence_projection(
        &self,
    ) -> AgentOrchestratorResult<(AgentTaskId, Vec<InfrastructureOperationsEvidenceRef>)> {
        match self
            .infrastructure_operations
            .as_ref()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?
        {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                match value.assessment.as_ref() {
                    Some(CloudInfrastructureStageOutcome::Completed(assessment)) => Ok((
                        assessment.task_id().clone(),
                        assessment.evidence_references(),
                    )),
                    _ => Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                }
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                match value.assessment.as_ref() {
                    Some(SystemsOperationsStageOutcome::Completed(assessment)) => Ok((
                        assessment.task_id().clone(),
                        assessment.evidence_references(),
                    )),
                    _ => Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                }
            }
        }
    }

    fn prepare_infrastructure_synthesis_request_for_first(
        &self,
        first: &InfrastructureOperationsFirstOutcome,
        run_ordinal: u8,
    ) -> AgentOrchestratorResult<RuntimeTurnRequest> {
        let qa = InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable;
        let security = InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable;
        let input = match (
            self.infrastructure_operations
                .as_ref()
                .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?,
            first,
        ) {
            (
                InfrastructureOperationsWorkflowState::Cloud(value),
                InfrastructureOperationsFirstOutcome::Cloud(first),
            ) => value.request.build_synthesis_input(first, &qa, &security)?,
            (
                InfrastructureOperationsWorkflowState::Systems(value),
                InfrastructureOperationsFirstOutcome::Systems(first),
            ) => value.request.build_synthesis_input(first, &qa, &security)?,
            _ => return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
        };
        self.runtime_request(
            self.root_task_id
                .as_ref()
                .ok_or(AgentOrchestratorError::RootMissing)?,
            run_ordinal,
            &input,
        )
    }

    fn prepare_infrastructure_synthesis_request_with_security(
        &self,
        security: &InfrastructureOperationsSecurityStageOutcome,
        run_ordinal: u8,
    ) -> AgentOrchestratorResult<RuntimeTurnRequest> {
        let input = match self
            .infrastructure_operations
            .as_ref()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?
        {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                value.request.build_synthesis_input(
                    value
                        .assessment
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    security,
                )?
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                value.request.build_synthesis_input(
                    value
                        .assessment
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .as_ref()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    security,
                )?
            }
        };
        self.runtime_request(
            self.root_task_id
                .as_ref()
                .ok_or(AgentOrchestratorError::RootMissing)?,
            run_ordinal,
            &input,
        )
    }

    pub(super) fn apply_prepared_infrastructure_operations_terminal(
        &mut self,
        task_id: &AgentTaskId,
        prepared: PreparedInfrastructureOperationsTerminal,
    ) -> AgentOrchestratorResult<()> {
        self.runs
            .remove(task_id)
            .ok_or(AgentOrchestratorError::NoActiveRun)?;
        match prepared {
            PreparedInfrastructureOperationsTerminal::First {
                task_outcome,
                first,
                failure,
                next,
            } => {
                self.finish_workflow_child_task(task_id, task_outcome)?;
                let mut workflow = self
                    .infrastructure_operations
                    .take()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                match (&mut workflow, first) {
                    (
                        InfrastructureOperationsWorkflowState::Cloud(value),
                        InfrastructureOperationsFirstOutcome::Cloud(outcome),
                    ) => {
                        let capability = match &outcome {
                            CloudInfrastructureStageOutcome::Completed(assessment) => {
                                match assessment.quality() {
                                    InfrastructureOperationsAssessmentQuality::Complete => {
                                        InfrastructureOperationsCapabilityAuditDisposition::ProposalOnly
                                    }
                                    InfrastructureOperationsAssessmentQuality::PartialDeniedCapability => {
                                        InfrastructureOperationsCapabilityAuditDisposition::DeniedRequested
                                    }
                                }
                            }
                            CloudInfrastructureStageOutcome::Failed(_)
                            | CloudInfrastructureStageOutcome::Cancelled => {
                                InfrastructureOperationsCapabilityAuditDisposition::NotApplicable
                            }
                        };
                        let parsed =
                            matches!(outcome, CloudInfrastructureStageOutcome::Completed(_));
                        let unavailable =
                            !matches!(outcome, CloudInfrastructureStageOutcome::Completed(_));
                        value.assessment = Some(outcome);
                        if unavailable {
                            value.qa = Some(
                                InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                            );
                            value.security = Some(
                                InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable,
                            );
                        }
                        if let Some(CloudInfrastructureStageOutcome::Completed(assessment)) =
                            value.assessment.as_ref()
                        {
                            push_cloud_infrastructure_transition(
                                value,
                                task_id.clone(),
                                None,
                                InfrastructureOperationsStage::CloudAssessment,
                                CloudInfrastructureWorkflowEvent::CloudAssessmentCompleted {
                                    task_id: task_id.clone(),
                                    quality: assessment.quality(),
                                },
                                InfrastructureOperationsAuditOutcome::Completed,
                                capability,
                            )?;
                        }
                        if let Some(code) = failure {
                            push_cloud_infrastructure_transition(
                                value,
                                task_id.clone(),
                                None,
                                InfrastructureOperationsStage::CloudAssessment,
                                CloudInfrastructureWorkflowEvent::PartialFailure {
                                    stage: InfrastructureOperationsStage::CloudAssessment,
                                    code,
                                },
                                InfrastructureOperationsAuditOutcome::PartialFailure(code),
                                capability,
                            )?;
                        } else if !parsed {
                            return Err(
                                AgentOrchestratorError::InfrastructureOperationsStageMismatch,
                            );
                        }
                    }
                    (
                        InfrastructureOperationsWorkflowState::Systems(value),
                        InfrastructureOperationsFirstOutcome::Systems(outcome),
                    ) => {
                        let capability = match &outcome {
                            SystemsOperationsStageOutcome::Completed(assessment) => {
                                match assessment.quality() {
                                    InfrastructureOperationsAssessmentQuality::Complete => {
                                        InfrastructureOperationsCapabilityAuditDisposition::ProposalOnly
                                    }
                                    InfrastructureOperationsAssessmentQuality::PartialDeniedCapability => {
                                        InfrastructureOperationsCapabilityAuditDisposition::DeniedRequested
                                    }
                                }
                            }
                            SystemsOperationsStageOutcome::Failed(_)
                            | SystemsOperationsStageOutcome::Cancelled => {
                                InfrastructureOperationsCapabilityAuditDisposition::NotApplicable
                            }
                        };
                        let parsed = matches!(outcome, SystemsOperationsStageOutcome::Completed(_));
                        let unavailable =
                            !matches!(outcome, SystemsOperationsStageOutcome::Completed(_));
                        value.assessment = Some(outcome);
                        if unavailable {
                            value.qa = Some(
                                InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable,
                            );
                            value.security = Some(
                                InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable,
                            );
                        }
                        if let Some(SystemsOperationsStageOutcome::Completed(assessment)) =
                            value.assessment.as_ref()
                        {
                            push_systems_operations_transition(
                                value,
                                task_id.clone(),
                                None,
                                InfrastructureOperationsStage::SystemsAssessment,
                                SystemsOperationsWorkflowEvent::SystemsAssessmentCompleted {
                                    task_id: task_id.clone(),
                                    quality: assessment.quality(),
                                },
                                InfrastructureOperationsAuditOutcome::Completed,
                                capability,
                            )?;
                        }
                        if let Some(code) = failure {
                            push_systems_operations_transition(
                                value,
                                task_id.clone(),
                                None,
                                InfrastructureOperationsStage::SystemsAssessment,
                                SystemsOperationsWorkflowEvent::PartialFailure {
                                    stage: InfrastructureOperationsStage::SystemsAssessment,
                                    code,
                                },
                                InfrastructureOperationsAuditOutcome::PartialFailure(code),
                                capability,
                            )?;
                        } else if !parsed {
                            return Err(
                                AgentOrchestratorError::InfrastructureOperationsStageMismatch,
                            );
                        }
                    }
                    _ => return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                }
                self.infrastructure_operations = Some(workflow);
                self.start_prepared_infrastructure_operations_next(task_id, next)
            }
            PreparedInfrastructureOperationsTerminal::Qa {
                task_outcome,
                qa,
                failure,
                next,
            } => {
                self.finish_workflow_child_task(task_id, task_outcome)?;
                let predecessor = self
                    .infrastructure_first_task_id()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?;
                let mut workflow = self
                    .infrastructure_operations
                    .take()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                match &mut workflow {
                    InfrastructureOperationsWorkflowState::Cloud(value) => {
                        let parsed =
                            matches!(qa, InfrastructureOperationsQaStageOutcome::Completed(_));
                        value.qa = Some(qa);
                        if let Some(InfrastructureOperationsQaStageOutcome::Completed(report)) =
                            value.qa.as_ref()
                        {
                            push_cloud_infrastructure_transition(
                                value,
                                task_id.clone(),
                                Some(predecessor.clone()),
                                InfrastructureOperationsStage::QaValidation,
                                CloudInfrastructureWorkflowEvent::QaValidationCompleted {
                                    task_id: task_id.clone(),
                                    conclusion: report.conclusion(),
                                },
                                InfrastructureOperationsAuditOutcome::Completed,
                                InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                            )?;
                        }
                        if let Some(code) = failure {
                            push_cloud_infrastructure_transition(
                                value,
                                task_id.clone(),
                                Some(predecessor.clone()),
                                InfrastructureOperationsStage::QaValidation,
                                CloudInfrastructureWorkflowEvent::PartialFailure {
                                    stage: InfrastructureOperationsStage::QaValidation,
                                    code,
                                },
                                InfrastructureOperationsAuditOutcome::PartialFailure(code),
                                InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                            )?;
                        } else if !parsed {
                            return Err(
                                AgentOrchestratorError::InfrastructureOperationsStageMismatch,
                            );
                        }
                    }
                    InfrastructureOperationsWorkflowState::Systems(value) => {
                        let parsed =
                            matches!(qa, InfrastructureOperationsQaStageOutcome::Completed(_));
                        value.qa = Some(qa);
                        if let Some(InfrastructureOperationsQaStageOutcome::Completed(report)) =
                            value.qa.as_ref()
                        {
                            push_systems_operations_transition(
                                value,
                                task_id.clone(),
                                Some(predecessor.clone()),
                                InfrastructureOperationsStage::QaValidation,
                                SystemsOperationsWorkflowEvent::QaValidationCompleted {
                                    task_id: task_id.clone(),
                                    conclusion: report.conclusion(),
                                },
                                InfrastructureOperationsAuditOutcome::Completed,
                                InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                            )?;
                        }
                        if let Some(code) = failure {
                            push_systems_operations_transition(
                                value,
                                task_id.clone(),
                                Some(predecessor.clone()),
                                InfrastructureOperationsStage::QaValidation,
                                SystemsOperationsWorkflowEvent::PartialFailure {
                                    stage: InfrastructureOperationsStage::QaValidation,
                                    code,
                                },
                                InfrastructureOperationsAuditOutcome::PartialFailure(code),
                                InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                            )?;
                        } else if !parsed {
                            return Err(
                                AgentOrchestratorError::InfrastructureOperationsStageMismatch,
                            );
                        }
                    }
                }
                self.infrastructure_operations = Some(workflow);
                self.start_prepared_infrastructure_operations_next(task_id, next)
            }
            PreparedInfrastructureOperationsTerminal::Security {
                task_outcome,
                security,
                failure,
                synthesis_request,
            } => {
                self.finish_workflow_child_task(task_id, task_outcome)?;
                let predecessor = self
                    .infrastructure_qa_task_id()
                    .or_else(|| self.infrastructure_first_task_id())
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?;
                let mut workflow = self
                    .infrastructure_operations
                    .take()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                match &mut workflow {
                    InfrastructureOperationsWorkflowState::Cloud(value) => {
                        let (event, audit) = match failure {
                            None => (
                                CloudInfrastructureWorkflowEvent::SecurityReviewCompleted {
                                    task_id: task_id.clone(),
                                },
                                InfrastructureOperationsAuditOutcome::Completed,
                            ),
                            Some(code) => (
                                CloudInfrastructureWorkflowEvent::PartialFailure {
                                    stage: InfrastructureOperationsStage::SecurityReview,
                                    code,
                                },
                                InfrastructureOperationsAuditOutcome::PartialFailure(code),
                            ),
                        };
                        value.security = Some(security);
                        push_cloud_infrastructure_transition(
                            value,
                            task_id.clone(),
                            Some(predecessor.clone()),
                            InfrastructureOperationsStage::SecurityReview,
                            event,
                            audit,
                            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                        )?;
                    }
                    InfrastructureOperationsWorkflowState::Systems(value) => {
                        let (event, audit) = match failure {
                            None => (
                                SystemsOperationsWorkflowEvent::SecurityReviewCompleted {
                                    task_id: task_id.clone(),
                                },
                                InfrastructureOperationsAuditOutcome::Completed,
                            ),
                            Some(code) => (
                                SystemsOperationsWorkflowEvent::PartialFailure {
                                    stage: InfrastructureOperationsStage::SecurityReview,
                                    code,
                                },
                                InfrastructureOperationsAuditOutcome::PartialFailure(code),
                            ),
                        };
                        value.security = Some(security);
                        push_systems_operations_transition(
                            value,
                            task_id.clone(),
                            Some(predecessor.clone()),
                            InfrastructureOperationsStage::SecurityReview,
                            event,
                            audit,
                            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                        )?;
                    }
                }
                self.infrastructure_operations = Some(workflow);
                self.start_infrastructure_synthesis_prepared(synthesis_request)
                    .map(|_| ())
            }
            PreparedInfrastructureOperationsTerminal::CloudSynthesisCompleted {
                output,
                synthesis,
            } => {
                self.tasks
                    .get_mut(task_id)
                    .ok_or(AgentOrchestratorError::TaskNotFound)?
                    .complete(AgentTaskResult::new(
                        task_id.clone(),
                        AgentId::PersonalAssistant,
                        output,
                    ))?;
                self.events.push(AgentOrchestrationEvent::RootCompleted {
                    task_id: task_id.clone(),
                });
                let mut workflow = self
                    .infrastructure_operations
                    .take()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                let InfrastructureOperationsWorkflowState::Cloud(value) = &mut workflow else {
                    return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch);
                };
                let predecessor = value
                    .audit_contexts
                    .iter()
                    .find(|(_, attribution)| attribution.agent_id() == AgentId::SecurityRisk)
                    .or_else(|| {
                        value.audit_contexts.iter().find(|(_, attribution)| {
                            attribution.agent_id() == AgentId::QaValidation
                        })
                    })
                    .or_else(|| {
                        value.audit_contexts.iter().find(|(_, attribution)| {
                            attribution.agent_id() == AgentId::CloudInfrastructure
                        })
                    })
                    .map(|(task_id, _)| task_id.clone());
                push_cloud_infrastructure_transition(
                    value,
                    task_id.clone(),
                    predecessor,
                    InfrastructureOperationsStage::Synthesis,
                    CloudInfrastructureWorkflowEvent::Completed {
                        task_id: task_id.clone(),
                    },
                    InfrastructureOperationsAuditOutcome::Completed,
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
                value.result = Some(CloudInfrastructureWorkflowResult::new(
                    RootTaskId::from_task_id(task_id.clone()),
                    value
                        .assessment
                        .clone()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .clone()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .security
                        .clone()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    synthesis,
                ));
                value.phase = InfrastructureOperationsPhase::Completed;
                self.infrastructure_operations = Some(workflow);
                self.cleanup_terminal_task(task_id);
                Ok(())
            }
            PreparedInfrastructureOperationsTerminal::SystemsSynthesisCompleted {
                output,
                synthesis,
            } => {
                self.tasks
                    .get_mut(task_id)
                    .ok_or(AgentOrchestratorError::TaskNotFound)?
                    .complete(AgentTaskResult::new(
                        task_id.clone(),
                        AgentId::PersonalAssistant,
                        output,
                    ))?;
                self.events.push(AgentOrchestrationEvent::RootCompleted {
                    task_id: task_id.clone(),
                });
                let mut workflow = self
                    .infrastructure_operations
                    .take()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                let InfrastructureOperationsWorkflowState::Systems(value) = &mut workflow else {
                    return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch);
                };
                let predecessor = value
                    .audit_contexts
                    .iter()
                    .find(|(_, attribution)| attribution.agent_id() == AgentId::SecurityRisk)
                    .or_else(|| {
                        value.audit_contexts.iter().find(|(_, attribution)| {
                            attribution.agent_id() == AgentId::QaValidation
                        })
                    })
                    .or_else(|| {
                        value.audit_contexts.iter().find(|(_, attribution)| {
                            attribution.agent_id() == AgentId::SystemsOperations
                        })
                    })
                    .map(|(task_id, _)| task_id.clone());
                push_systems_operations_transition(
                    value,
                    task_id.clone(),
                    predecessor,
                    InfrastructureOperationsStage::Synthesis,
                    SystemsOperationsWorkflowEvent::Completed {
                        task_id: task_id.clone(),
                    },
                    InfrastructureOperationsAuditOutcome::Completed,
                    InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                )?;
                value.result = Some(SystemsOperationsWorkflowResult::new(
                    RootTaskId::from_task_id(task_id.clone()),
                    value
                        .assessment
                        .clone()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .qa
                        .clone()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    value
                        .security
                        .clone()
                        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?,
                    synthesis,
                ));
                value.phase = InfrastructureOperationsPhase::Completed;
                self.infrastructure_operations = Some(workflow);
                self.cleanup_terminal_task(task_id);
                Ok(())
            }
            PreparedInfrastructureOperationsTerminal::SynthesisFailed {
                task_code,
                workflow_code,
            } => self.fail_infrastructure_root_without_run(task_code, workflow_code),
            PreparedInfrastructureOperationsTerminal::SynthesisCancelled => {
                let outcome = self
                    .tasks
                    .get_mut(task_id)
                    .ok_or(AgentOrchestratorError::TaskNotFound)?
                    .cancel();
                if outcome != AgentTaskCancellationOutcome::Cancelled {
                    return Err(AgentOrchestratorError::OutcomeMismatch);
                }
                self.events.push(AgentOrchestrationEvent::TaskCancelled {
                    task_id: task_id.clone(),
                    agent_id: AgentId::PersonalAssistant,
                });
                let mut workflow = self
                    .infrastructure_operations
                    .take()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                let predecessor = infrastructure_synthesis_predecessor_from_state(&workflow);
                match &mut workflow {
                    InfrastructureOperationsWorkflowState::Cloud(value) => {
                        push_cloud_infrastructure_transition(
                            value,
                            task_id.clone(),
                            predecessor.clone(),
                            InfrastructureOperationsStage::Synthesis,
                            CloudInfrastructureWorkflowEvent::Cancelled {
                                stage: InfrastructureOperationsStage::Synthesis,
                            },
                            InfrastructureOperationsAuditOutcome::Cancelled,
                            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                        )?;
                        value.phase = InfrastructureOperationsPhase::Cancelled;
                    }
                    InfrastructureOperationsWorkflowState::Systems(value) => {
                        push_systems_operations_transition(
                            value,
                            task_id.clone(),
                            predecessor,
                            InfrastructureOperationsStage::Synthesis,
                            SystemsOperationsWorkflowEvent::Cancelled {
                                stage: InfrastructureOperationsStage::Synthesis,
                            },
                            InfrastructureOperationsAuditOutcome::Cancelled,
                            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                        )?;
                        value.phase = InfrastructureOperationsPhase::Cancelled;
                    }
                }
                self.infrastructure_operations = Some(workflow);
                self.cleanup_terminal_task(task_id);
                Ok(())
            }
        }
    }

    fn start_prepared_infrastructure_operations_next(
        &mut self,
        predecessor_task_id: &AgentTaskId,
        next: PreparedInfrastructureOperationsNext,
    ) -> AgentOrchestratorResult<()> {
        match next {
            PreparedInfrastructureOperationsNext::Synthesis(request) => self
                .start_infrastructure_synthesis_prepared(request)
                .map(|_| ()),
            PreparedInfrastructureOperationsNext::Child {
                task,
                request,
                attribution,
            } => {
                let task_id = task.id().clone();
                let agent_id = task.agent_id();
                let (stage, continuation_failure, is_qa) = match agent_id {
                    AgentId::QaValidation => (
                        InfrastructureOperationsStage::QaValidation,
                        InfrastructureOperationsContinuationFailure::QaStartFailed,
                        true,
                    ),
                    AgentId::SecurityRisk => (
                        InfrastructureOperationsStage::SecurityReview,
                        InfrastructureOperationsContinuationFailure::SecurityStartFailed,
                        false,
                    ),
                    _ => return Err(AgentOrchestratorError::InfrastructureOperationsStageMismatch),
                };
                let root_task_id = self
                    .root_task_id
                    .clone()
                    .ok_or(AgentOrchestratorError::RootMissing)?;
                self.tasks.insert(task_id.clone(), *task);
                self.active_child_task_id = Some(task_id.clone());
                self.run_count = self
                    .run_count
                    .checked_add(1)
                    .ok_or(AgentOrchestratorError::RunLimitExceeded)?;
                self.events.push(AgentOrchestrationEvent::ChildCreated {
                    task_id: task_id.clone(),
                    parent_task_id: root_task_id,
                });
                self.events.push(AgentOrchestrationEvent::ChildStarted {
                    task_id: task_id.clone(),
                });
                let mut workflow = self
                    .infrastructure_operations
                    .take()
                    .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
                match &mut workflow {
                    InfrastructureOperationsWorkflowState::Cloud(value) => {
                        value.child_count = value
                            .child_count
                            .checked_add(1)
                            .ok_or(AgentOrchestratorError::TotalChildLimitExceeded)?;
                        value.run_attempts = self.run_count;
                        value.audit_contexts.insert(task_id.clone(), attribution);
                        let event = if is_qa {
                            CloudInfrastructureWorkflowEvent::QaValidationStarted {
                                task_id: task_id.clone(),
                                predecessor_task_id: predecessor_task_id.clone(),
                            }
                        } else {
                            CloudInfrastructureWorkflowEvent::SecurityReviewStarted {
                                task_id: task_id.clone(),
                                predecessor_task_id: predecessor_task_id.clone(),
                            }
                        };
                        push_cloud_infrastructure_transition(
                            value,
                            task_id.clone(),
                            Some(predecessor_task_id.clone()),
                            stage,
                            event,
                            InfrastructureOperationsAuditOutcome::Started,
                            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                        )?;
                        value.phase = if is_qa {
                            InfrastructureOperationsPhase::QaRunning(task_id.clone())
                        } else {
                            InfrastructureOperationsPhase::SecurityRunning(task_id.clone())
                        };
                    }
                    InfrastructureOperationsWorkflowState::Systems(value) => {
                        value.child_count = value
                            .child_count
                            .checked_add(1)
                            .ok_or(AgentOrchestratorError::TotalChildLimitExceeded)?;
                        value.run_attempts = self.run_count;
                        value.audit_contexts.insert(task_id.clone(), attribution);
                        let event = if is_qa {
                            SystemsOperationsWorkflowEvent::QaValidationStarted {
                                task_id: task_id.clone(),
                                predecessor_task_id: predecessor_task_id.clone(),
                            }
                        } else {
                            SystemsOperationsWorkflowEvent::SecurityReviewStarted {
                                task_id: task_id.clone(),
                                predecessor_task_id: predecessor_task_id.clone(),
                            }
                        };
                        push_systems_operations_transition(
                            value,
                            task_id.clone(),
                            Some(predecessor_task_id.clone()),
                            stage,
                            event,
                            InfrastructureOperationsAuditOutcome::Started,
                            InfrastructureOperationsCapabilityAuditDisposition::NotApplicable,
                        )?;
                        value.phase = if is_qa {
                            InfrastructureOperationsPhase::QaRunning(task_id.clone())
                        } else {
                            InfrastructureOperationsPhase::SecurityRunning(task_id.clone())
                        };
                    }
                }
                self.infrastructure_operations = Some(workflow);
                match self.start_runtime_run(request) {
                    Ok(run) => {
                        self.runs.insert(
                            task_id,
                            ActiveRun {
                                run,
                                output: String::new(),
                                next_sequence: 0,
                            },
                        );
                        Ok(())
                    }
                    Err(error) => {
                        self.record_infrastructure_continuation_failure(continuation_failure)?;
                        self.terminalize_infrastructure_child_failure(
                            &task_id,
                            AgentTaskFailureCode::RuntimeStartFailed,
                            InfrastructureOperationsPartialFailureCode::RuntimeStartFailed,
                        )?;
                        if is_qa {
                            let _ = self.start_infrastructure_security();
                        } else {
                            let _ = self.start_infrastructure_synthesis();
                        }
                        Err(error)
                    }
                }
            }
        }
    }

    pub(super) fn start_infrastructure_security(
        &mut self,
    ) -> AgentOrchestratorResult<AgentExecutionContext> {
        let (_, _, transfer) = self.infrastructure_assessment_projection()?;
        let workflow = self
            .infrastructure_operations
            .as_ref()
            .ok_or(AgentOrchestratorError::InfrastructureOperationsWorkflowMissing)?;
        let qa = match workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => value.qa.as_ref(),
            InfrastructureOperationsWorkflowState::Systems(value) => value.qa.as_ref(),
        }
        .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?;
        let input = match workflow {
            InfrastructureOperationsWorkflowState::Cloud(value) => {
                value.request.build_security_input(&transfer, qa)?
            }
            InfrastructureOperationsWorkflowState::Systems(value) => {
                value.request.build_security_input(&transfer, qa)?
            }
        };
        let task = self.prepare_infrastructure_operations_child(
            AgentId::SecurityRisk,
            3,
            "Assess bounded security and operational risk for the validated fixture proposal",
            "Return one strict RiskAssessmentV1 JSON object",
        )?;
        let task_id = task.id().clone();
        let request = self.runtime_request(&task_id, self.run_count + 1, &input)?;
        let context = AgentExecutionContext::for_task(&task, self.runtime_id, request.identity());
        let predecessor = self
            .infrastructure_qa_task_id()
            .or_else(|| self.infrastructure_first_task_id())
            .ok_or(AgentOrchestratorError::InfrastructureOperationsStageMismatch)?;
        self.start_prepared_infrastructure_operations_next(
            &predecessor,
            PreparedInfrastructureOperationsNext::Child {
                task: Box::new(task),
                request,
                attribution: InfrastructureOperationsAttribution::from_execution_context(&context),
            },
        )?;
        Ok(context)
    }

    pub(super) fn infrastructure_first_task_id(&self) -> Option<AgentTaskId> {
        self.infrastructure_operations
            .as_ref()
            .map(|workflow| match workflow {
                InfrastructureOperationsWorkflowState::Cloud(value) => &value.audit_contexts,
                InfrastructureOperationsWorkflowState::Systems(value) => &value.audit_contexts,
            })
            .and_then(|contexts| {
                contexts.iter().find(|(_, attribution)| {
                    matches!(
                        attribution.agent_id(),
                        AgentId::CloudInfrastructure | AgentId::SystemsOperations
                    )
                })
            })
            .map(|(task_id, _)| task_id.clone())
    }

    pub(super) fn infrastructure_qa_task_id(&self) -> Option<AgentTaskId> {
        self.infrastructure_operations
            .as_ref()
            .map(|workflow| match workflow {
                InfrastructureOperationsWorkflowState::Cloud(value) => &value.audit_contexts,
                InfrastructureOperationsWorkflowState::Systems(value) => &value.audit_contexts,
            })
            .and_then(|contexts| {
                contexts
                    .iter()
                    .find(|(_, attribution)| attribution.agent_id() == AgentId::QaValidation)
            })
            .map(|(task_id, _)| task_id.clone())
    }

    pub(super) fn infrastructure_security_task_id(&self) -> Option<AgentTaskId> {
        self.infrastructure_operations
            .as_ref()
            .map(|workflow| match workflow {
                InfrastructureOperationsWorkflowState::Cloud(value) => &value.audit_contexts,
                InfrastructureOperationsWorkflowState::Systems(value) => &value.audit_contexts,
            })
            .and_then(|contexts| {
                contexts
                    .iter()
                    .find(|(_, attribution)| attribution.agent_id() == AgentId::SecurityRisk)
            })
            .map(|(task_id, _)| task_id.clone())
    }

    fn infrastructure_synthesis_predecessor_task_id(&self) -> Option<AgentTaskId> {
        self.infrastructure_security_task_id()
            .or_else(|| self.infrastructure_qa_task_id())
            .or_else(|| self.infrastructure_first_task_id())
    }
}
