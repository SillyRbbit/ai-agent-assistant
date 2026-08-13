//! Private take-once dispatch and cooperative-deadline bridge for D-090.

use super::*;

use std::{sync::Arc, time::Instant};

use crate::agent::{
    engineering_quality::EngineeringReviewStatus,
    infrastructure_operations::InfrastructureOperationsReviewStatus,
    research_knowledge::FinalSynthesisStatus,
    workflow_automation::{
        WorkflowAutomationAttribution, WorkflowAutomationError, WorkflowManualDispatch,
        WorkflowManualDispatchAcceptance, WorkflowManualDispatchAuditRecord,
        WorkflowManualDispatchEvent, WorkflowManualDispatchStatus, WorkflowProposalId,
        WorkflowTemplateCatalog, WorkflowTemplateId, MAX_WORKFLOW_MANUAL_DISPATCH_RECORDS,
        WORKFLOW_AUTOMATION_CATALOG_VERSION,
    },
};

pub(super) type WorkflowClock = Arc<dyn Fn() -> Instant + Send + Sync>;

pub(super) fn system_workflow_clock() -> WorkflowClock {
    Arc::new(Instant::now)
}

pub(super) struct ManualWorkflowDispatchState {
    proposal_id: WorkflowProposalId,
    template_id: WorkflowTemplateId,
    deadline: Instant,
    attribution: WorkflowAutomationAttribution,
    _planner_root_task_id: RootTaskId,
    _planner_task_id: AgentTaskId,
    status: WorkflowManualDispatchStatus,
    events: Vec<WorkflowManualDispatchEvent>,
    audit: Vec<WorkflowManualDispatchAuditRecord>,
    pub(super) expiring: bool,
}

impl ManualWorkflowDispatchState {
    fn new(
        proposal_id: WorkflowProposalId,
        template_id: WorkflowTemplateId,
        deadline: Instant,
        attribution: WorkflowAutomationAttribution,
        planner_root_task_id: RootTaskId,
        planner_task_id: AgentTaskId,
    ) -> Self {
        Self {
            proposal_id,
            template_id,
            deadline,
            attribution,
            _planner_root_task_id: planner_root_task_id,
            _planner_task_id: planner_task_id,
            status: WorkflowManualDispatchStatus::Requested,
            events: Vec::with_capacity(MAX_WORKFLOW_MANUAL_DISPATCH_RECORDS),
            audit: Vec::with_capacity(MAX_WORKFLOW_MANUAL_DISPATCH_RECORDS),
            expiring: false,
        }
    }

    fn is_settled(&self) -> bool {
        matches!(
            self.status,
            WorkflowManualDispatchStatus::Completed
                | WorkflowManualDispatchStatus::Partial
                | WorkflowManualDispatchStatus::Cancelled
                | WorkflowManualDispatchStatus::Expired
                | WorkflowManualDispatchStatus::Failed
        )
    }
}

impl<R: AgentRuntime> AgentOrchestrator<R> {
    pub(super) fn workflow_now(&self) -> Instant {
        (self.workflow_clock)()
    }

    pub fn start_manual_workflow(
        &mut self,
        source_context: &AgentExecutionContext,
        dispatch: WorkflowManualDispatch,
    ) -> AgentOrchestratorResult<WorkflowManualDispatchAcceptance> {
        // Ownership is consumed before any fallible operation. No error path
        // returns a token that could be replayed or corrected in place.
        let parts = dispatch.into_parts();
        if parts.catalog_version != WORKFLOW_AUTOMATION_CATALOG_VERSION {
            return Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::CatalogConfiguration,
            ));
        }
        if !parts.template_id.is_manually_dispatchable() {
            return Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchUnavailable,
            ));
        }
        if self.workflow_now() >= parts.deadline {
            return Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchExpired,
            ));
        }
        self.live_personal_root_attribution(source_context)?;
        let selected = dispatch_selection(parts.template_id)?;
        self.ensure_workflow_selection_available(selected, false)?;
        if self.manual_workflow_dispatch.is_some() {
            return Err(AgentOrchestratorError::ManualWorkflowDispatchAlreadySelected);
        }
        let mut state = ManualWorkflowDispatchState::new(
            parts.proposal_id,
            parts.template_id,
            parts.deadline,
            WorkflowAutomationAttribution::from_execution_context(source_context),
            parts.planner_root_task_id,
            parts.planner_task_id,
        );
        push_manual_dispatch_transition(
            &mut state,
            WorkflowManualDispatchEvent::Requested {
                template_id: parts.template_id,
            },
            WorkflowManualDispatchStatus::Requested,
        )?;
        self.manual_workflow_dispatch = Some(state);

        let result = self.start_selected_manual_workflow(
            source_context,
            parts.template_id,
            WorkflowTemplateCatalog::built_in(),
        );
        let mut state = self
            .manual_workflow_dispatch
            .take()
            .ok_or(AgentOrchestratorError::ManualWorkflowDispatchMissing)?;
        match result {
            Ok(acceptance) => {
                push_manual_dispatch_transition(
                    &mut state,
                    WorkflowManualDispatchEvent::Accepted {
                        template_id: parts.template_id,
                    },
                    WorkflowManualDispatchStatus::Accepted,
                )?;
                self.manual_workflow_dispatch = Some(state);
                Ok(acceptance)
            }
            Err(error) => {
                let transition = push_manual_dispatch_transition(
                    &mut state,
                    WorkflowManualDispatchEvent::Failed {
                        template_id: parts.template_id,
                    },
                    WorkflowManualDispatchStatus::Failed,
                );
                self.manual_workflow_dispatch = Some(state);
                transition?;
                Err(error)
            }
        }
    }

    fn start_selected_manual_workflow(
        &mut self,
        source_context: &AgentExecutionContext,
        template_id: WorkflowTemplateId,
        catalog: WorkflowTemplateCatalog,
    ) -> AgentOrchestratorResult<WorkflowManualDispatchAcceptance> {
        let request = catalog.proposal_request(template_id)?;
        match template_id {
            WorkflowTemplateId::ResearchBriefV1 => self
                .request_research_knowledge_workflow(source_context, request.research_request()?)
                .map(WorkflowManualDispatchAcceptance::ResearchBrief),
            WorkflowTemplateId::CodeQualityReviewV1 => self
                .start_engineering_quality_workflow(source_context, request.engineering_request()?)
                .map(WorkflowManualDispatchAcceptance::CodeQualityReview),
            WorkflowTemplateId::InfrastructureAssessmentV1 => self
                .start_cloud_infrastructure_workflow(source_context, request.cloud_request()?)
                .map(WorkflowManualDispatchAcceptance::InfrastructureAssessment),
            WorkflowTemplateId::SystemsIncidentAnalysisV1 => self
                .start_systems_operations_workflow(source_context, request.systems_request()?)
                .map(WorkflowManualDispatchAcceptance::SystemsIncidentAnalysis),
            WorkflowTemplateId::DocumentToActionPlanV1 => {
                Err(AgentOrchestratorError::WorkflowAutomation(
                    WorkflowAutomationError::ManualDispatchUnavailable,
                ))
            }
        }
    }

    pub fn check_manual_workflow_deadline(
        &mut self,
    ) -> AgentOrchestratorResult<Option<WorkflowManualDispatchStatus>> {
        self.refresh_manual_workflow_dispatch()?;
        let Some(state) = self.manual_workflow_dispatch.as_ref() else {
            return Ok(None);
        };
        if state.is_settled() || self.workflow_now() < state.deadline {
            return Ok(Some(state.status));
        }

        let root_task_id = self
            .root_task_id
            .clone()
            .ok_or(AgentOrchestratorError::RootMissing)?;
        let needs_failure_transition =
            state.status != WorkflowManualDispatchStatus::ExpiryCancellationFailed;
        self.manual_workflow_dispatch
            .as_mut()
            .ok_or(AgentOrchestratorError::ManualWorkflowDispatchMissing)?
            .expiring = true;
        match self.cancel_task(&root_task_id) {
            Ok(_) => {
                self.refresh_manual_workflow_dispatch()?;
                Err(AgentOrchestratorError::WorkflowAutomation(
                    WorkflowAutomationError::ManualDispatchExpired,
                ))
            }
            Err(error) => {
                self.manual_workflow_dispatch
                    .as_mut()
                    .ok_or(AgentOrchestratorError::ManualWorkflowDispatchMissing)?
                    .expiring = false;
                if needs_failure_transition {
                    let mut state = self
                        .manual_workflow_dispatch
                        .take()
                        .ok_or(AgentOrchestratorError::ManualWorkflowDispatchMissing)?;
                    let template_id = state.template_id;
                    let transition = push_manual_dispatch_transition(
                        &mut state,
                        WorkflowManualDispatchEvent::ExpiryCancellationFailed { template_id },
                        WorkflowManualDispatchStatus::ExpiryCancellationFailed,
                    );
                    self.manual_workflow_dispatch = Some(state);
                    transition?;
                }
                let _ = error;
                Err(AgentOrchestratorError::ManualWorkflowExpiryCancellationFailed)
            }
        }
    }

    pub fn manual_workflow_result(
        &mut self,
    ) -> AgentOrchestratorResult<Option<WorkflowManualDispatchStatus>> {
        match self.check_manual_workflow_deadline() {
            Ok(_) => {}
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchExpired,
            )) => {}
            Err(error) => return Err(error),
        }
        self.refresh_manual_workflow_dispatch()?;
        Ok(self
            .manual_workflow_dispatch
            .as_ref()
            .map(|state| state.status))
    }

    #[must_use]
    pub fn manual_workflow_dispatch_status(&self) -> Option<WorkflowManualDispatchStatus> {
        self.manual_workflow_dispatch
            .as_ref()
            .map(|state| state.status)
    }

    #[must_use]
    pub fn manual_workflow_dispatch_events(&self) -> &[WorkflowManualDispatchEvent] {
        self.manual_workflow_dispatch
            .as_ref()
            .map_or(&[], |state| state.events.as_slice())
    }

    #[must_use]
    pub fn manual_workflow_dispatch_audit_records(&self) -> &[WorkflowManualDispatchAuditRecord] {
        self.manual_workflow_dispatch
            .as_ref()
            .map_or(&[], |state| state.audit.as_slice())
    }

    pub(super) fn enforce_manual_workflow_deadline_before_ingress(
        &mut self,
    ) -> AgentOrchestratorResult<()> {
        if self
            .manual_workflow_dispatch
            .as_ref()
            .is_some_and(|state| state.expiring)
        {
            return Ok(());
        }
        match self.check_manual_workflow_deadline() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub(super) fn refresh_manual_workflow_dispatch(&mut self) -> AgentOrchestratorResult<()> {
        let Some(state) = self.manual_workflow_dispatch.as_ref() else {
            return Ok(());
        };
        if state.is_settled() {
            return Ok(());
        }
        let Some(root) = self.root_task() else {
            return Ok(());
        };
        let next_status = match root.status() {
            AgentTaskStatus::Completed => Some(self.manual_completion_status(state.template_id)?),
            AgentTaskStatus::Failed => Some(WorkflowManualDispatchStatus::Failed),
            AgentTaskStatus::Cancelled if state.expiring => {
                Some(WorkflowManualDispatchStatus::Expired)
            }
            AgentTaskStatus::Cancelled => Some(WorkflowManualDispatchStatus::Cancelled),
            AgentTaskStatus::Pending
            | AgentTaskStatus::Running
            | AgentTaskStatus::WaitingForChild => None,
        };
        let Some(next_status) = next_status else {
            return Ok(());
        };
        let mut state = self
            .manual_workflow_dispatch
            .take()
            .ok_or(AgentOrchestratorError::ManualWorkflowDispatchMissing)?;
        let event = match next_status {
            WorkflowManualDispatchStatus::Completed | WorkflowManualDispatchStatus::Partial => {
                WorkflowManualDispatchEvent::Completed {
                    template_id: state.template_id,
                    status: next_status,
                }
            }
            WorkflowManualDispatchStatus::Cancelled => WorkflowManualDispatchEvent::Cancelled {
                template_id: state.template_id,
            },
            WorkflowManualDispatchStatus::Expired => WorkflowManualDispatchEvent::Expired {
                template_id: state.template_id,
            },
            WorkflowManualDispatchStatus::Failed => WorkflowManualDispatchEvent::Failed {
                template_id: state.template_id,
            },
            WorkflowManualDispatchStatus::Requested
            | WorkflowManualDispatchStatus::Accepted
            | WorkflowManualDispatchStatus::ExpiryCancellationFailed => {
                self.manual_workflow_dispatch = Some(state);
                return Err(AgentOrchestratorError::ManualWorkflowDispatchStageMismatch);
            }
        };
        let transition = push_manual_dispatch_transition(&mut state, event, next_status);
        self.manual_workflow_dispatch = Some(state);
        transition
    }

    fn manual_completion_status(
        &self,
        template_id: WorkflowTemplateId,
    ) -> AgentOrchestratorResult<WorkflowManualDispatchStatus> {
        let partial = match template_id {
            WorkflowTemplateId::ResearchBriefV1 => {
                self.research_knowledge_result()
                    .ok_or(AgentOrchestratorError::ManualWorkflowDispatchStageMismatch)?
                    .synthesis()
                    .status()
                    == FinalSynthesisStatus::Partial
            }
            WorkflowTemplateId::CodeQualityReviewV1 => {
                self.engineering_quality_result()
                    .ok_or(AgentOrchestratorError::ManualWorkflowDispatchStageMismatch)?
                    .synthesis()
                    .status()
                    == EngineeringReviewStatus::Partial
            }
            WorkflowTemplateId::InfrastructureAssessmentV1 => {
                self.cloud_infrastructure_result()
                    .ok_or(AgentOrchestratorError::ManualWorkflowDispatchStageMismatch)?
                    .synthesis()
                    .status()
                    == InfrastructureOperationsReviewStatus::Partial
            }
            WorkflowTemplateId::SystemsIncidentAnalysisV1 => {
                self.systems_operations_result()
                    .ok_or(AgentOrchestratorError::ManualWorkflowDispatchStageMismatch)?
                    .synthesis()
                    .status()
                    == InfrastructureOperationsReviewStatus::Partial
            }
            WorkflowTemplateId::DocumentToActionPlanV1 => {
                return Err(AgentOrchestratorError::ManualWorkflowDispatchStageMismatch)
            }
        };
        Ok(if partial {
            WorkflowManualDispatchStatus::Partial
        } else {
            WorkflowManualDispatchStatus::Completed
        })
    }
}

fn dispatch_selection(
    template_id: WorkflowTemplateId,
) -> AgentOrchestratorResult<AgentWorkflowSelection> {
    match template_id {
        WorkflowTemplateId::ResearchBriefV1 => Ok(AgentWorkflowSelection::ResearchKnowledge),
        WorkflowTemplateId::CodeQualityReviewV1 => Ok(AgentWorkflowSelection::EngineeringQuality),
        WorkflowTemplateId::InfrastructureAssessmentV1 => {
            Ok(AgentWorkflowSelection::CloudInfrastructure)
        }
        WorkflowTemplateId::SystemsIncidentAnalysisV1 => {
            Ok(AgentWorkflowSelection::SystemsOperations)
        }
        WorkflowTemplateId::DocumentToActionPlanV1 => {
            Err(AgentOrchestratorError::WorkflowAutomation(
                WorkflowAutomationError::ManualDispatchUnavailable,
            ))
        }
    }
}

fn push_manual_dispatch_transition(
    state: &mut ManualWorkflowDispatchState,
    event: WorkflowManualDispatchEvent,
    status: WorkflowManualDispatchStatus,
) -> AgentOrchestratorResult<()> {
    if state.events.len() >= MAX_WORKFLOW_MANUAL_DISPATCH_RECORDS
        || state.audit.len() >= MAX_WORKFLOW_MANUAL_DISPATCH_RECORDS
    {
        return Err(AgentOrchestratorError::ManualWorkflowDispatchJournalLimitExceeded);
    }
    let sequence = u8::try_from(state.audit.len() + 1)
        .map_err(|_| AgentOrchestratorError::ManualWorkflowDispatchJournalLimitExceeded)?;
    state.events.push(event);
    state.audit.push(WorkflowManualDispatchAuditRecord::new(
        sequence,
        state.attribution.clone(),
        state.proposal_id.clone(),
        state.template_id,
        status,
    ));
    state.status = status;
    Ok(())
}
