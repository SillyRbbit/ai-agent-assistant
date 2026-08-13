//! Closed fixture-only workflow proposal contracts.
//!
//! Workflow Automation is an untrusted planner. These types contain inert
//! proposal data only. Application validation may make one of four existing
//! sealed workflows eligible for a later explicit manual dispatch, but no
//! value in this module can create a task, execute a tool, request approval,
//! change policy, or perform an effect.

mod catalog;
mod validation;

use std::{fmt, time::Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    definition::{AgentId, AgentIdParseError},
    engineering_quality::{EngineeringQualityError, EngineeringQualityWorkflowRequest},
    gateway_protocol::is_valid_opaque_id,
    governance::AgentPolicyProfileId,
    infrastructure_operations::{
        CloudInfrastructureWorkflowRequest, InfrastructureOperationsError,
        SystemsOperationsWorkflowRequest,
    },
    orchestrator::{
        CloudInfrastructureWorkflowAcceptance, EngineeringQualityWorkflowAcceptance,
        ResearchKnowledgeWorkflowAcceptance, SystemsOperationsWorkflowAcceptance,
    },
    registry::{AgentRegistry, AgentRegistryError},
    research_knowledge::{ResearchKnowledgeError, ResearchKnowledgeWorkflowRequest},
    runtime::{RuntimeId, RuntimeRunIdentity},
    task::{AgentExecutionContext, AgentTaskFailureCode, AgentTaskId, ParentTaskId, RootTaskId},
};
use crate::memory::AgentMemoryProfileId;

pub const WORKFLOW_AUTOMATION_CONTRACT_VERSION: u16 = 1;
pub const WORKFLOW_AUTOMATION_CATALOG_VERSION: u16 = 1;
pub const MAX_WORKFLOW_STEPS: usize = 4;
pub const MAX_WORKFLOW_AGENT_TASK_STEPS: usize = 3;
pub const MAX_WORKFLOW_PROPOSED_TOOL_STEPS: usize = 2;
pub const MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS: usize = 0;
pub const MAX_WORKFLOW_DEPENDENCIES_PER_STEP: usize = 3;
pub const MAX_WORKFLOW_DURATION_SECONDS: u64 = 120;
pub const MAX_WORKFLOW_RETRIES: u8 = 0;
pub const MAX_NESTED_WORKFLOW_DEPTH: u8 = 0;
pub const MAX_WORKFLOW_OBJECTIVE_CHARACTERS: usize = 2_048;
pub const MAX_WORKFLOW_OBJECTIVE_BYTES: usize = 8_192;
pub const MAX_WORKFLOW_PROPOSAL_CHARACTERS: usize = 8_192;
pub const MAX_WORKFLOW_PROPOSAL_BYTES: usize = 16_384;
pub const MAX_WORKFLOW_EXPECTED_OUTPUT_CHARACTERS: usize = 512;
pub const MAX_WORKFLOW_EXPECTED_OUTPUT_BYTES: usize = 2_048;
pub const MAX_WORKFLOW_TOOL_NAME_BYTES: usize = 64;
pub const MAX_WORKFLOW_TOOL_ARGUMENT_BYTES: usize = 2_048;
pub const MAX_WORKFLOW_CATALOG_BYTES: usize = 8_192;
pub const MAX_WORKFLOW_PLANNER_INPUT_BYTES: usize = 24_576;
pub const MAX_WORKFLOW_SYNTHESIS_INPUT_BYTES: usize = 32_768;
pub const MAX_WORKFLOW_SYNTHESIS_CHARACTERS: usize = 8_192;
pub const MAX_WORKFLOW_SYNTHESIS_BYTES: usize = 16_384;
pub const MAX_WORKFLOW_UNRESOLVED_ISSUES: usize = 8;
pub const MAX_WORKFLOW_EVENTS: usize = 8;
pub const MAX_WORKFLOW_AUDIT_RECORDS: usize = 8;
pub const MAX_WORKFLOW_MANUAL_DISPATCH_RECORDS: usize = 4;
pub const WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE: &str = "Fixture-only, proposal-only, and unwired; manual dispatch was not run; no workflow or tool ran, no approval was requested, and no effect was performed.";
pub const WORKFLOW_AUTOMATION_DOCUMENT_PROPOSAL_ONLY_ISSUE: &str =
    "Document-to-action remains proposal-only and cannot be manually dispatched in D-090.";
pub const WORKFLOW_AUTOMATION_REJECTED_ISSUE: &str =
    "Application validation rejected the fixture-only workflow proposal; no manual dispatch is available.";
pub const WORKFLOW_AUTOMATION_FAILED_ISSUE: &str =
    "The Workflow Automation proposal task failed; no manual dispatch is available.";
pub const WORKFLOW_AUTOMATION_CANCELLED_ISSUE: &str =
    "The fixture-only proposal stage is unavailable after cancellation; no manual dispatch is available.";

pub type WorkflowAutomationResult<T> = Result<T, WorkflowAutomationError>;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkflowProposalId(String);

impl WorkflowProposalId {
    /// Parses one opaque proposal correlation identifier.
    ///
    /// The identifier is inert data. Constructing it does not validate a
    /// proposal or grant task, dispatch, policy, approval, or execution
    /// authority.
    pub fn new(value: impl Into<String>) -> WorkflowAutomationResult<Self> {
        let value = value.into();
        if is_valid_opaque_id(&value) {
            Ok(Self(value))
        } else {
            Err(WorkflowAutomationError::InvalidProposalId)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for WorkflowProposalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkflowProposalId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkflowStepId(String);

impl WorkflowStepId {
    fn new(value: impl Into<String>) -> WorkflowAutomationResult<Self> {
        let value = value.into();
        if is_valid_workflow_id(&value) {
            Ok(Self(value))
        } else {
            Err(WorkflowAutomationError::InvalidStepId)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for WorkflowStepId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkflowStepId")
            .field(&self.0)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkflowTemplateId {
    ResearchBriefV1,
    CodeQualityReviewV1,
    InfrastructureAssessmentV1,
    SystemsIncidentAnalysisV1,
    DocumentToActionPlanV1,
}

impl WorkflowTemplateId {
    pub const ALL: [Self; 5] = [
        Self::ResearchBriefV1,
        Self::CodeQualityReviewV1,
        Self::InfrastructureAssessmentV1,
        Self::SystemsIncidentAnalysisV1,
        Self::DocumentToActionPlanV1,
    ];

    #[must_use]
    pub const fn is_manually_dispatchable(self) -> bool {
        !matches!(self, Self::DocumentToActionPlanV1)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkflowFailureBehavior {
    PartialSynthesis,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowLimit {
    max_steps: u8,
    max_agent_tasks: u8,
    max_tool_steps: u8,
    max_duration_seconds: u64,
    max_retries: u8,
    max_nested_workflows: u8,
}

impl WorkflowLimit {
    #[must_use]
    pub const fn canonical() -> Self {
        Self {
            max_steps: MAX_WORKFLOW_STEPS as u8,
            max_agent_tasks: MAX_WORKFLOW_AGENT_TASK_STEPS as u8,
            max_tool_steps: MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS as u8,
            max_duration_seconds: MAX_WORKFLOW_DURATION_SECONDS,
            max_retries: MAX_WORKFLOW_RETRIES,
            max_nested_workflows: MAX_NESTED_WORKFLOW_DEPTH,
        }
    }

    #[must_use]
    pub const fn max_steps(self) -> u8 {
        self.max_steps
    }
    #[must_use]
    pub const fn max_agent_tasks(self) -> u8 {
        self.max_agent_tasks
    }
    #[must_use]
    pub const fn max_tool_steps(self) -> u8 {
        self.max_tool_steps
    }
    #[must_use]
    pub const fn max_duration_seconds(self) -> u64 {
        self.max_duration_seconds
    }
    #[must_use]
    pub const fn max_retries(self) -> u8 {
        self.max_retries
    }
    #[must_use]
    pub const fn max_nested_workflows(self) -> u8 {
        self.max_nested_workflows
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowAgentTaskStep {
    id: WorkflowStepId,
    agent_id: AgentId,
    dependencies: Vec<WorkflowStepId>,
    expected_output: String,
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowSynthesisStep {
    id: WorkflowStepId,
    agent_id: AgentId,
    dependencies: Vec<WorkflowStepId>,
    expected_output: String,
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowGovernedToolStep {
    id: WorkflowStepId,
    tool_name: String,
    tool_contract_version: u16,
    arguments_json: String,
    dependencies: Vec<WorkflowStepId>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowApprovalCheckpoint {
    id: WorkflowStepId,
    subject_step_id: WorkflowStepId,
    dependencies: Vec<WorkflowStepId>,
}

macro_rules! step_accessors {
    ($type:ty) => {
        impl $type {
            #[must_use]
            pub fn id(&self) -> &WorkflowStepId {
                &self.id
            }
            #[must_use]
            pub fn dependencies(&self) -> &[WorkflowStepId] {
                &self.dependencies
            }
        }
    };
}

step_accessors!(WorkflowAgentTaskStep);
step_accessors!(WorkflowSynthesisStep);
step_accessors!(WorkflowGovernedToolStep);
step_accessors!(WorkflowApprovalCheckpoint);

impl WorkflowAgentTaskStep {
    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }
    #[must_use]
    pub fn expected_output(&self) -> &str {
        &self.expected_output
    }
}

impl WorkflowSynthesisStep {
    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }
    #[must_use]
    pub fn expected_output(&self) -> &str {
        &self.expected_output
    }
}

impl WorkflowGovernedToolStep {
    #[must_use]
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }
    #[must_use]
    pub const fn tool_contract_version(&self) -> u16 {
        self.tool_contract_version
    }
}

impl WorkflowApprovalCheckpoint {
    #[must_use]
    pub fn subject_step_id(&self) -> &WorkflowStepId {
        &self.subject_step_id
    }
}

impl fmt::Debug for WorkflowGovernedToolStep {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowGovernedToolStep")
            .field("id", &self.id)
            .field("tool_name", &self.tool_name)
            .field("tool_contract_version", &self.tool_contract_version)
            .field("arguments_json", &"[REDACTED]")
            .field("dependencies", &self.dependencies)
            .finish()
    }
}

impl fmt::Debug for WorkflowAgentTaskStep {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowAgentTaskStep")
            .field("id", &self.id)
            .field("agent_id", &self.agent_id)
            .field("dependencies", &self.dependencies)
            .field("expected_output", &self.expected_output)
            .finish()
    }
}

impl fmt::Debug for WorkflowSynthesisStep {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowSynthesisStep")
            .field("id", &self.id)
            .field("agent_id", &self.agent_id)
            .field("dependencies", &self.dependencies)
            .field("expected_output", &self.expected_output)
            .finish()
    }
}

impl fmt::Debug for WorkflowApprovalCheckpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowApprovalCheckpoint")
            .field("id", &self.id)
            .field("subject_step_id", &self.subject_step_id)
            .field("dependencies", &self.dependencies)
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowStep {
    AgentTask(WorkflowAgentTaskStep),
    GovernedTool(WorkflowGovernedToolStep),
    ApprovalCheckpoint(WorkflowApprovalCheckpoint),
    Synthesis(WorkflowSynthesisStep),
}

impl WorkflowStep {
    #[must_use]
    pub fn id(&self) -> &WorkflowStepId {
        match self {
            Self::AgentTask(value) => value.id(),
            Self::GovernedTool(value) => value.id(),
            Self::ApprovalCheckpoint(value) => value.id(),
            Self::Synthesis(value) => value.id(),
        }
    }

    #[must_use]
    pub fn dependencies(&self) -> &[WorkflowStepId] {
        match self {
            Self::AgentTask(value) => value.dependencies(),
            Self::GovernedTool(value) => value.dependencies(),
            Self::ApprovalCheckpoint(value) => value.dependencies(),
            Self::Synthesis(value) => value.dependencies(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowDefinition {
    version: u16,
    template_id: WorkflowTemplateId,
    objective: String,
    steps: Vec<WorkflowStep>,
    limits: WorkflowLimit,
    failure_behavior: WorkflowFailureBehavior,
}

impl WorkflowDefinition {
    #[must_use]
    pub const fn version(&self) -> u16 {
        self.version
    }
    #[must_use]
    pub const fn template_id(&self) -> WorkflowTemplateId {
        self.template_id
    }
    #[must_use]
    pub fn objective(&self) -> &str {
        &self.objective
    }
    #[must_use]
    pub fn steps(&self) -> &[WorkflowStep] {
        &self.steps
    }
    #[must_use]
    pub const fn limits(&self) -> WorkflowLimit {
        self.limits
    }
    #[must_use]
    pub const fn failure_behavior(&self) -> WorkflowFailureBehavior {
        self.failure_behavior
    }
}

impl fmt::Debug for WorkflowDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowDefinition")
            .field("version", &self.version)
            .field("template_id", &self.template_id)
            .field("objective", &"[REDACTED]")
            .field("objective_bytes", &self.objective.len())
            .field("steps", &self.steps)
            .field("limits", &self.limits)
            .field("failure_behavior", &self.failure_behavior)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowProposal {
    id: WorkflowProposalId,
    definition: WorkflowDefinition,
    transfer_json: String,
}

impl WorkflowProposal {
    #[must_use]
    pub fn id(&self) -> &WorkflowProposalId {
        &self.id
    }
    #[must_use]
    pub fn definition(&self) -> &WorkflowDefinition {
        &self.definition
    }
    pub(super) fn transfer_json(&self) -> &str {
        &self.transfer_json
    }
}

impl fmt::Debug for WorkflowProposal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowProposal")
            .field("id", &self.id)
            .field("definition", &self.definition)
            .field("transfer_json", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkflowValidationDisposition {
    ReadyForManualDispatch,
    ProposalOnly,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ValidatedWorkflowProposal {
    proposal: WorkflowProposal,
    disposition: WorkflowValidationDisposition,
}

impl ValidatedWorkflowProposal {
    #[must_use]
    pub fn proposal(&self) -> &WorkflowProposal {
        &self.proposal
    }
    #[must_use]
    pub const fn disposition(&self) -> WorkflowValidationDisposition {
        self.disposition
    }
}

impl fmt::Debug for ValidatedWorkflowProposal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidatedWorkflowProposal")
            .field("proposal", &self.proposal)
            .field("disposition", &self.disposition)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowValidationFailureCode {
    InvalidStructuredProposal,
    InvalidVersion,
    InvalidIdentifier,
    BoundExceeded,
    AuthorityClaim,
    DuplicateStep,
    DuplicateDependency,
    UnknownDependency,
    SelfDependency,
    Cycle,
    UnsupportedStep,
    UnknownAgent,
    AgentUnavailable,
    UnknownTool,
    ToolVersionMismatch,
    InvalidToolArguments,
    MissingApprovalCheckpoint,
    ApprovalCheckpointMismatch,
    ApprovalDispatchUnavailable,
    ToolStepsUnavailable,
    TemplateMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowProposalStageOutcome {
    Validated(ValidatedWorkflowProposal),
    Rejected {
        proposal_id: WorkflowProposalId,
        code: WorkflowValidationFailureCode,
    },
    Failed {
        proposal_id: WorkflowProposalId,
        code: AgentTaskFailureCode,
    },
    Cancelled {
        proposal_id: WorkflowProposalId,
    },
}

impl WorkflowProposalStageOutcome {
    #[must_use]
    pub fn proposal_id(&self) -> &WorkflowProposalId {
        match self {
            Self::Validated(value) => value.proposal.id(),
            Self::Rejected { proposal_id, .. }
            | Self::Failed { proposal_id, .. }
            | Self::Cancelled { proposal_id } => proposal_id,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkflowAutomationSynthesisStatus {
    Complete,
    Partial,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkflowSynthesisDisposition {
    ReadyForManualDispatch,
    ProposalOnly,
    Unavailable,
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowAutomationSynthesis {
    summary: String,
    template_id: WorkflowTemplateId,
    proposal_id: WorkflowProposalId,
    disposition: WorkflowSynthesisDisposition,
    status: WorkflowAutomationSynthesisStatus,
    unresolved_issues: Vec<String>,
}

impl WorkflowAutomationSynthesis {
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }
    #[must_use]
    pub const fn template_id(&self) -> WorkflowTemplateId {
        self.template_id
    }
    #[must_use]
    pub fn proposal_id(&self) -> &WorkflowProposalId {
        &self.proposal_id
    }
    #[must_use]
    pub const fn disposition(&self) -> WorkflowSynthesisDisposition {
        self.disposition
    }
    #[must_use]
    pub const fn status(&self) -> WorkflowAutomationSynthesisStatus {
        self.status
    }
    #[must_use]
    pub fn unresolved_issues(&self) -> &[String] {
        &self.unresolved_issues
    }
    #[must_use]
    pub const fn fixture_based(&self) -> bool {
        true
    }
    #[must_use]
    pub const fn unwired(&self) -> bool {
        true
    }
    #[must_use]
    pub const fn workflow_executed(&self) -> bool {
        false
    }
    #[must_use]
    pub const fn tools_executed(&self) -> bool {
        false
    }
    #[must_use]
    pub const fn approvals_requested(&self) -> bool {
        false
    }
    #[must_use]
    pub const fn effects_performed(&self) -> bool {
        false
    }
}

impl fmt::Debug for WorkflowAutomationSynthesis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowAutomationSynthesis")
            .field("summary", &"[REDACTED]")
            .field("template_id", &self.template_id)
            .field("proposal_id", &self.proposal_id)
            .field("disposition", &self.disposition)
            .field("status", &self.status)
            .field("unresolved_issue_count", &self.unresolved_issues.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowAutomationProposalRequest {
    template: WorkflowDefinition,
    catalog_serialized: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowAutomationWorkflowAcceptance {
    ProposalStarted { context: AgentExecutionContext },
    PersonalFallbackStarted { context: AgentExecutionContext },
}

impl WorkflowAutomationWorkflowAcceptance {
    #[must_use]
    pub fn context(&self) -> &AgentExecutionContext {
        match self {
            Self::ProposalStarted { context } | Self::PersonalFallbackStarted { context } => {
                context
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowManualDispatchAcceptance {
    ResearchBrief(ResearchKnowledgeWorkflowAcceptance),
    CodeQualityReview(EngineeringQualityWorkflowAcceptance),
    InfrastructureAssessment(CloudInfrastructureWorkflowAcceptance),
    SystemsIncidentAnalysis(SystemsOperationsWorkflowAcceptance),
}

impl WorkflowAutomationProposalRequest {
    #[must_use]
    pub const fn template_id(&self) -> WorkflowTemplateId {
        self.template.template_id
    }
    #[must_use]
    pub fn objective(&self) -> &str {
        &self.template.objective
    }
    #[must_use]
    pub fn template(&self) -> &WorkflowDefinition {
        &self.template
    }

    pub fn build_planner_input(&self) -> WorkflowAutomationResult<String> {
        catalog::build_planner_input(self)
    }

    pub fn parse_proposal(
        &self,
        proposal_id: WorkflowProposalId,
        raw: &str,
        registry: &AgentRegistry,
    ) -> WorkflowAutomationResult<ValidatedWorkflowProposal> {
        validation::parse_and_validate(self, proposal_id, raw, registry)
    }

    pub fn build_synthesis_input(
        &self,
        proposal: &WorkflowProposalStageOutcome,
    ) -> WorkflowAutomationResult<String> {
        validation::build_synthesis_input(self, proposal)
    }

    pub fn parse_synthesis(
        &self,
        proposal: &WorkflowProposalStageOutcome,
        raw: &str,
    ) -> WorkflowAutomationResult<WorkflowAutomationSynthesis> {
        validation::parse_synthesis(self, proposal, raw)
    }

    pub(super) fn research_request(
        &self,
    ) -> WorkflowAutomationResult<ResearchKnowledgeWorkflowRequest> {
        catalog::research_request(self.template_id())
    }

    pub(super) fn engineering_request(
        &self,
    ) -> WorkflowAutomationResult<EngineeringQualityWorkflowRequest> {
        catalog::engineering_request(self.template_id())
    }

    pub(super) fn cloud_request(
        &self,
    ) -> WorkflowAutomationResult<CloudInfrastructureWorkflowRequest> {
        catalog::cloud_request(self.template_id())
    }

    pub(super) fn systems_request(
        &self,
    ) -> WorkflowAutomationResult<SystemsOperationsWorkflowRequest> {
        catalog::systems_request(self.template_id())
    }
}

impl fmt::Debug for WorkflowAutomationProposalRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowAutomationProposalRequest")
            .field("template_id", &self.template_id())
            .field("objective", &"[REDACTED]")
            .field("catalog_bytes", &self.catalog_serialized.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkflowTemplateCatalog;

impl WorkflowTemplateCatalog {
    #[must_use]
    pub const fn built_in() -> Self {
        Self
    }

    pub fn proposal_request(
        self,
        template_id: WorkflowTemplateId,
    ) -> WorkflowAutomationResult<WorkflowAutomationProposalRequest> {
        catalog::proposal_request(template_id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowAutomationStage {
    Proposal,
    Synthesis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowAutomationPartialFailureCode {
    RuntimeStartFailed,
    RuntimeFailed,
    InvalidStructuredOutput,
    ProposalRejected(WorkflowValidationFailureCode),
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowAutomationContinuationFailure {
    ProposalStartFailed,
    ProposalAndSynthesisStartFailed,
    SynthesisStartFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowAutomationWorkflowEvent {
    ProposalStarted {
        task_id: AgentTaskId,
    },
    ProposalCompleted {
        task_id: AgentTaskId,
        disposition: WorkflowValidationDisposition,
    },
    SynthesisStarted {
        task_id: AgentTaskId,
    },
    PartialFailure {
        stage: WorkflowAutomationStage,
        code: WorkflowAutomationPartialFailureCode,
    },
    Expired {
        stage: WorkflowAutomationStage,
    },
    Cancelled {
        stage: WorkflowAutomationStage,
    },
    Completed {
        task_id: AgentTaskId,
    },
    Failed {
        stage: WorkflowAutomationStage,
        code: WorkflowAutomationPartialFailureCode,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowAutomationAuditOutcome {
    Started,
    Completed,
    PartialFailure(WorkflowAutomationPartialFailureCode),
    Expired,
    Cancelled,
    Failed(WorkflowAutomationPartialFailureCode),
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowAutomationAttribution {
    agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    task_id: AgentTaskId,
    root_task_id: RootTaskId,
    parent_task_id: Option<ParentTaskId>,
    runtime_id: RuntimeId,
    depth: u8,
    _runtime_run_identity: RuntimeRunIdentity,
}

impl WorkflowAutomationAttribution {
    pub(super) fn from_execution_context(context: &AgentExecutionContext) -> Self {
        Self {
            agent_id: context.agent_id(),
            policy_profile_id: context.policy_profile_id(),
            memory_profile_id: context.memory_profile_id(),
            task_id: context.task_id().clone(),
            root_task_id: context.root_task_id().clone(),
            parent_task_id: context.parent_task_id().cloned(),
            runtime_id: context.runtime_id(),
            depth: context.depth(),
            _runtime_run_identity: context.runtime_run_identity().clone(),
        }
    }
    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }
    #[must_use]
    pub const fn policy_profile_id(&self) -> AgentPolicyProfileId {
        self.policy_profile_id
    }
    #[must_use]
    pub const fn memory_profile_id(&self) -> AgentMemoryProfileId {
        self.memory_profile_id
    }
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }
    #[must_use]
    pub fn root_task_id(&self) -> &RootTaskId {
        &self.root_task_id
    }
    #[must_use]
    pub fn parent_task_id(&self) -> Option<&ParentTaskId> {
        self.parent_task_id.as_ref()
    }
    #[must_use]
    pub const fn runtime_id(&self) -> RuntimeId {
        self.runtime_id
    }
    #[must_use]
    pub const fn depth(&self) -> u8 {
        self.depth
    }
}

impl fmt::Debug for WorkflowAutomationAttribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowAutomationAttribution")
            .field("agent_id", &self.agent_id)
            .field("policy_profile_id", &self.policy_profile_id)
            .field("memory_profile_id", &self.memory_profile_id)
            .field("task_id", &self.task_id)
            .field("root_task_id", &self.root_task_id)
            .field("parent_task_id", &self.parent_task_id)
            .field("runtime_id", &self.runtime_id)
            .field("depth", &self.depth)
            .field("runtime_run_identity", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowAutomationAuditRecord {
    sequence: u8,
    attribution: WorkflowAutomationAttribution,
    proposal_id: WorkflowProposalId,
    template_id: WorkflowTemplateId,
    stage: WorkflowAutomationStage,
    outcome: WorkflowAutomationAuditOutcome,
}

impl WorkflowAutomationAuditRecord {
    pub(super) fn new(
        sequence: u8,
        attribution: WorkflowAutomationAttribution,
        proposal_id: WorkflowProposalId,
        template_id: WorkflowTemplateId,
        stage: WorkflowAutomationStage,
        outcome: WorkflowAutomationAuditOutcome,
    ) -> Self {
        Self {
            sequence,
            attribution,
            proposal_id,
            template_id,
            stage,
            outcome,
        }
    }
    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
    #[must_use]
    pub fn attribution(&self) -> &WorkflowAutomationAttribution {
        &self.attribution
    }
    #[must_use]
    pub fn proposal_id(&self) -> &WorkflowProposalId {
        &self.proposal_id
    }
    #[must_use]
    pub const fn template_id(&self) -> WorkflowTemplateId {
        self.template_id
    }
    #[must_use]
    pub const fn stage(&self) -> WorkflowAutomationStage {
        self.stage
    }
    #[must_use]
    pub const fn outcome(&self) -> WorkflowAutomationAuditOutcome {
        self.outcome
    }
}

impl fmt::Debug for WorkflowAutomationAuditRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowAutomationAuditRecord")
            .field("sequence", &self.sequence)
            .field("attribution", &self.attribution)
            .field("proposal_id", &self.proposal_id)
            .field("template_id", &self.template_id)
            .field("stage", &self.stage)
            .field("outcome", &self.outcome)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowAutomationWorkflowResult {
    root_task_id: RootTaskId,
    template_id: WorkflowTemplateId,
    proposal: WorkflowProposalStageOutcome,
    synthesis: WorkflowAutomationSynthesis,
}

impl WorkflowAutomationWorkflowResult {
    pub(super) fn new(
        root_task_id: RootTaskId,
        template_id: WorkflowTemplateId,
        proposal: WorkflowProposalStageOutcome,
        synthesis: WorkflowAutomationSynthesis,
    ) -> Self {
        Self {
            root_task_id,
            template_id,
            proposal,
            synthesis,
        }
    }
    #[must_use]
    pub fn root_task_id(&self) -> &RootTaskId {
        &self.root_task_id
    }
    #[must_use]
    pub const fn template_id(&self) -> WorkflowTemplateId {
        self.template_id
    }
    #[must_use]
    pub fn proposal(&self) -> &WorkflowProposalStageOutcome {
        &self.proposal
    }
    #[must_use]
    pub fn synthesis(&self) -> &WorkflowAutomationSynthesis {
        &self.synthesis
    }
    #[must_use]
    pub const fn fixture_based(&self) -> bool {
        true
    }
    #[must_use]
    pub const fn unwired(&self) -> bool {
        true
    }
}

impl fmt::Debug for WorkflowAutomationWorkflowResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowAutomationWorkflowResult")
            .field("root_task_id", &self.root_task_id)
            .field("template_id", &self.template_id)
            .field("proposal", &self.proposal)
            .field("synthesis", &self.synthesis)
            .finish()
    }
}

/// Process-local, take-once evidence that one validated A-D proposal may be
/// manually mapped to its existing sealed selector. It is not authorization.
pub struct WorkflowManualDispatch {
    proposal_id: WorkflowProposalId,
    template_id: WorkflowTemplateId,
    planner_root_task_id: RootTaskId,
    planner_task_id: AgentTaskId,
    catalog_version: u16,
    deadline: Instant,
}

impl WorkflowManualDispatch {
    pub(super) fn issue(
        proposal: &ValidatedWorkflowProposal,
        planner_root_task_id: RootTaskId,
        planner_task_id: AgentTaskId,
        deadline: Instant,
    ) -> WorkflowAutomationResult<Self> {
        if proposal.disposition != WorkflowValidationDisposition::ReadyForManualDispatch {
            return Err(WorkflowAutomationError::ManualDispatchUnavailable);
        }
        Ok(Self {
            proposal_id: proposal.proposal.id.clone(),
            template_id: proposal.proposal.definition.template_id,
            planner_root_task_id,
            planner_task_id,
            catalog_version: WORKFLOW_AUTOMATION_CATALOG_VERSION,
            deadline,
        })
    }

    #[must_use]
    pub const fn template_id(&self) -> WorkflowTemplateId {
        self.template_id
    }

    pub(super) fn into_parts(self) -> WorkflowManualDispatchParts {
        WorkflowManualDispatchParts {
            proposal_id: self.proposal_id,
            template_id: self.template_id,
            planner_root_task_id: self.planner_root_task_id,
            planner_task_id: self.planner_task_id,
            catalog_version: self.catalog_version,
            deadline: self.deadline,
        }
    }
}

impl fmt::Debug for WorkflowManualDispatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowManualDispatch")
            .field("proposal_id", &"[REDACTED]")
            .field("template_id", &self.template_id)
            .field("planner_identity", &"[REDACTED]")
            .field("catalog_version", &self.catalog_version)
            .field("deadline", &"[REDACTED]")
            .finish()
    }
}

pub(super) struct WorkflowManualDispatchParts {
    pub(super) proposal_id: WorkflowProposalId,
    pub(super) template_id: WorkflowTemplateId,
    pub(super) planner_root_task_id: RootTaskId,
    pub(super) planner_task_id: AgentTaskId,
    pub(super) catalog_version: u16,
    pub(super) deadline: Instant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowManualDispatchAvailability {
    NotReady,
    Available,
    Taken,
    ProposalOnly,
    Expired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowManualDispatchStatus {
    Requested,
    Accepted,
    Completed,
    Partial,
    Cancelled,
    Expired,
    Failed,
    ExpiryCancellationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowManualDispatchEvent {
    Requested {
        template_id: WorkflowTemplateId,
    },
    Accepted {
        template_id: WorkflowTemplateId,
    },
    Completed {
        template_id: WorkflowTemplateId,
        status: WorkflowManualDispatchStatus,
    },
    Cancelled {
        template_id: WorkflowTemplateId,
    },
    Expired {
        template_id: WorkflowTemplateId,
    },
    Failed {
        template_id: WorkflowTemplateId,
    },
    ExpiryCancellationFailed {
        template_id: WorkflowTemplateId,
    },
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkflowManualDispatchAuditRecord {
    sequence: u8,
    attribution: WorkflowAutomationAttribution,
    proposal_id: WorkflowProposalId,
    template_id: WorkflowTemplateId,
    status: WorkflowManualDispatchStatus,
}

impl WorkflowManualDispatchAuditRecord {
    pub(super) fn new(
        sequence: u8,
        attribution: WorkflowAutomationAttribution,
        proposal_id: WorkflowProposalId,
        template_id: WorkflowTemplateId,
        status: WorkflowManualDispatchStatus,
    ) -> Self {
        Self {
            sequence,
            attribution,
            proposal_id,
            template_id,
            status,
        }
    }
    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
    #[must_use]
    pub fn attribution(&self) -> &WorkflowAutomationAttribution {
        &self.attribution
    }
    #[must_use]
    pub fn proposal_id(&self) -> &WorkflowProposalId {
        &self.proposal_id
    }
    #[must_use]
    pub const fn template_id(&self) -> WorkflowTemplateId {
        self.template_id
    }
    #[must_use]
    pub const fn status(&self) -> WorkflowManualDispatchStatus {
        self.status
    }
}

impl fmt::Debug for WorkflowManualDispatchAuditRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowManualDispatchAuditRecord")
            .field("sequence", &self.sequence)
            .field("attribution", &self.attribution)
            .field("proposal_id", &"[REDACTED]")
            .field("template_id", &self.template_id)
            .field("status", &self.status)
            .finish()
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum WorkflowAutomationError {
    #[error("workflow proposal identity is invalid")]
    InvalidProposalId,
    #[error("workflow step identity is invalid")]
    InvalidStepId,
    #[error("workflow text or aggregate exceeds its closed bound")]
    BoundExceeded,
    #[error("workflow proposal JSON is invalid")]
    InvalidStructuredProposal,
    #[error("workflow proposal version is unsupported")]
    InvalidVersion,
    #[error("workflow proposal makes an authority or execution claim")]
    AuthorityClaim,
    #[error("workflow proposal contains a duplicate step")]
    DuplicateStep,
    #[error("workflow proposal contains a duplicate dependency")]
    DuplicateDependency,
    #[error("workflow proposal references an unknown dependency")]
    UnknownDependency,
    #[error("workflow proposal contains a self dependency")]
    SelfDependency,
    #[error("workflow proposal contains a dependency cycle")]
    Cycle,
    #[error("workflow proposal contains an unsupported step type")]
    UnsupportedStep,
    #[error("workflow proposal references an unknown agent")]
    UnknownAgent,
    #[error("workflow proposal references an unavailable agent")]
    AgentUnavailable,
    #[error("workflow proposal references an unknown tool")]
    UnknownTool,
    #[error("workflow proposal tool contract version does not match")]
    ToolVersionMismatch,
    #[error("workflow proposal tool arguments are invalid")]
    InvalidToolArguments,
    #[error("workflow proposal omits a required approval checkpoint")]
    MissingApprovalCheckpoint,
    #[error("workflow proposal approval checkpoint binding is invalid")]
    ApprovalCheckpointMismatch,
    #[error("workflow approval dispatch is unavailable")]
    ApprovalDispatchUnavailable,
    #[error("workflow tool steps are recognized but unavailable for execution")]
    ToolStepsUnavailable,
    #[error("workflow proposal does not exactly match its application template")]
    TemplateMismatch,
    #[error("workflow template is not eligible for manual dispatch")]
    ManualDispatchUnavailable,
    #[error("workflow manual dispatch has expired")]
    ManualDispatchExpired,
    #[error("workflow manual dispatch token was already taken")]
    ManualDispatchAlreadyTaken,
    #[error("workflow proposal deadline could not be calculated")]
    DeadlineOverflow,
    #[error("workflow proposal deadline expired")]
    WorkflowExpired,
    #[error("workflow catalog configuration is invalid")]
    CatalogConfiguration,
    #[error("workflow serialization failed")]
    SerializationFailed,
    #[error("agent registry rejected workflow validation")]
    Registry(#[from] AgentRegistryError),
    #[error("agent identifier is invalid")]
    AgentId(#[from] AgentIdParseError),
    #[error("research workflow fixture construction failed")]
    Research(#[from] ResearchKnowledgeError),
    #[error("engineering workflow fixture construction failed")]
    Engineering(#[from] EngineeringQualityError),
    #[error("infrastructure workflow fixture construction failed")]
    Infrastructure(#[from] InfrastructureOperationsError),
}

impl WorkflowAutomationError {
    #[must_use]
    pub const fn validation_failure_code(&self) -> WorkflowValidationFailureCode {
        match self {
            Self::InvalidStructuredProposal | Self::SerializationFailed => {
                WorkflowValidationFailureCode::InvalidStructuredProposal
            }
            Self::InvalidVersion => WorkflowValidationFailureCode::InvalidVersion,
            Self::InvalidProposalId | Self::InvalidStepId | Self::AgentId(_) => {
                WorkflowValidationFailureCode::InvalidIdentifier
            }
            Self::BoundExceeded => WorkflowValidationFailureCode::BoundExceeded,
            Self::AuthorityClaim => WorkflowValidationFailureCode::AuthorityClaim,
            Self::DuplicateStep => WorkflowValidationFailureCode::DuplicateStep,
            Self::DuplicateDependency => WorkflowValidationFailureCode::DuplicateDependency,
            Self::UnknownDependency => WorkflowValidationFailureCode::UnknownDependency,
            Self::SelfDependency => WorkflowValidationFailureCode::SelfDependency,
            Self::Cycle => WorkflowValidationFailureCode::Cycle,
            Self::UnsupportedStep => WorkflowValidationFailureCode::UnsupportedStep,
            Self::UnknownAgent | Self::Registry(_) => WorkflowValidationFailureCode::UnknownAgent,
            Self::AgentUnavailable => WorkflowValidationFailureCode::AgentUnavailable,
            Self::UnknownTool => WorkflowValidationFailureCode::UnknownTool,
            Self::ToolVersionMismatch => WorkflowValidationFailureCode::ToolVersionMismatch,
            Self::InvalidToolArguments => WorkflowValidationFailureCode::InvalidToolArguments,
            Self::MissingApprovalCheckpoint => {
                WorkflowValidationFailureCode::MissingApprovalCheckpoint
            }
            Self::ApprovalCheckpointMismatch => {
                WorkflowValidationFailureCode::ApprovalCheckpointMismatch
            }
            Self::ApprovalDispatchUnavailable => {
                WorkflowValidationFailureCode::ApprovalDispatchUnavailable
            }
            Self::ToolStepsUnavailable => WorkflowValidationFailureCode::ToolStepsUnavailable,
            Self::TemplateMismatch
            | Self::ManualDispatchUnavailable
            | Self::ManualDispatchExpired
            | Self::ManualDispatchAlreadyTaken
            | Self::DeadlineOverflow
            | Self::WorkflowExpired
            | Self::CatalogConfiguration
            | Self::Research(_)
            | Self::Engineering(_)
            | Self::Infrastructure(_) => WorkflowValidationFailureCode::TemplateMismatch,
        }
    }
}

fn is_valid_workflow_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;
    use crate::agent::definition::{AgentActivation, AgentActivationGate, AgentDefinition};

    fn raw_template(template_id: WorkflowTemplateId) -> WorkflowAutomationResult<String> {
        let input = WorkflowTemplateCatalog::built_in()
            .proposal_request(template_id)?
            .build_planner_input()?;
        let (_, selected) = input
            .split_once("selected_template(application-owned):\n")
            .ok_or(WorkflowAutomationError::SerializationFailed)?;
        selected
            .split_once("\nReturn exactly one strict WorkflowProposalV1 JSON object.")
            .map(|(raw, _)| raw.to_owned())
            .ok_or(WorkflowAutomationError::SerializationFailed)
    }

    fn mutate_step(
        template_id: WorkflowTemplateId,
        index: usize,
        replacement: Value,
    ) -> WorkflowAutomationResult<String> {
        let mut value: Value = serde_json::from_str(&raw_template(template_id)?)
            .map_err(|_| WorkflowAutomationError::SerializationFailed)?;
        value["steps"][index] = replacement;
        serde_json::to_string(&value).map_err(|_| WorkflowAutomationError::SerializationFailed)
    }

    fn validated_stage(
        template_id: WorkflowTemplateId,
        proposal_id: &str,
    ) -> WorkflowAutomationResult<(
        WorkflowAutomationProposalRequest,
        WorkflowProposalStageOutcome,
    )> {
        let request = WorkflowTemplateCatalog::built_in().proposal_request(template_id)?;
        let validated = request.parse_proposal(
            WorkflowProposalId::new(proposal_id)?,
            &raw_template(template_id)?,
            &AgentRegistry::built_in()?,
        )?;
        Ok((request, WorkflowProposalStageOutcome::Validated(validated)))
    }

    fn synthesis_raw(
        template_id: WorkflowTemplateId,
        proposal_id: &str,
        summary: &str,
        status: &str,
        unresolved_issues: &[&str],
    ) -> WorkflowAutomationResult<String> {
        serde_json::to_string(&json!({
            "version": "v1",
            "summary": summary,
            "template_id": template_id,
            "proposal_id": proposal_id,
            "validation_disposition": "ready-for-manual-dispatch",
            "status": status,
            "unresolved_issues": unresolved_issues,
            "fixture_based": true,
            "unwired": true,
            "workflow_executed": false,
            "tools_executed": false,
            "approvals_requested": false,
            "effects_performed": false,
        }))
        .map_err(|_| WorkflowAutomationError::SerializationFailed)
    }

    #[test]
    fn validates_all_five_exact_templates_and_derives_dispatch_posture(
    ) -> WorkflowAutomationResult<()> {
        let registry = AgentRegistry::built_in()?;
        for template_id in WorkflowTemplateId::ALL {
            let request = WorkflowTemplateCatalog::built_in().proposal_request(template_id)?;
            let validated = request.parse_proposal(
                WorkflowProposalId::new(format!("proposal-{template_id:?}"))?,
                &raw_template(template_id)?,
                &registry,
            )?;
            assert_eq!(
                validated.disposition(),
                if template_id.is_manually_dispatchable() {
                    WorkflowValidationDisposition::ReadyForManualDispatch
                } else {
                    WorkflowValidationDisposition::ProposalOnly
                }
            );
        }
        Ok(())
    }

    #[test]
    fn unavailable_agent_fails_before_template_dispatch_eligibility(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut definitions = Vec::with_capacity(AgentId::ALL.len());
        for agent_id in AgentId::ALL {
            let definition = AgentDefinition::built_in(agent_id)?;
            definitions.push(if agent_id == AgentId::Research {
                definition.with_activation_for_test(AgentActivation::Deferred(
                    AgentActivationGate::TypedWorkflowGovernance,
                ))
            } else {
                definition
            });
        }
        let registry = AgentRegistry::from_definitions(definitions)?;
        let request = WorkflowTemplateCatalog::built_in()
            .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;

        assert_eq!(
            request.parse_proposal(
                WorkflowProposalId::new("proposal-unavailable-agent")?,
                &raw_template(WorkflowTemplateId::ResearchBriefV1)?,
                &registry,
            ),
            Err(WorkflowAutomationError::AgentUnavailable)
        );
        Ok(())
    }

    #[test]
    fn rejects_unknown_tool_before_the_zero_execution_boundary() -> WorkflowAutomationResult<()> {
        let request = WorkflowTemplateCatalog::built_in()
            .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
        let raw = mutate_step(
            WorkflowTemplateId::ResearchBriefV1,
            0,
            json!({
                "id": "research",
                "type": "governed-tool",
                "tool_name": "run_shell",
                "tool_contract_version": 1,
                "arguments": {},
                "depends_on": [],
            }),
        )?;
        assert_eq!(
            request.parse_proposal(
                WorkflowProposalId::new("unknown-tool-proposal")?,
                &raw,
                &AgentRegistry::built_in()?,
            ),
            Err(WorkflowAutomationError::UnknownTool)
        );
        Ok(())
    }

    #[test]
    fn known_tool_remains_non_executable() -> WorkflowAutomationResult<()> {
        let request = WorkflowTemplateCatalog::built_in()
            .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
        let raw = mutate_step(
            WorkflowTemplateId::ResearchBriefV1,
            0,
            json!({
                "id": "research",
                "type": "governed-tool",
                "tool_name": "get_current_datetime",
                "tool_contract_version": 1,
                "arguments": {},
                "depends_on": [],
            }),
        )?;
        assert_eq!(
            request.parse_proposal(
                WorkflowProposalId::new("known-tool-proposal")?,
                &raw,
                &AgentRegistry::built_in()?,
            ),
            Err(WorkflowAutomationError::ToolStepsUnavailable)
        );
        Ok(())
    }

    #[test]
    fn duplicate_json_keys_and_authority_claims_fail_closed() -> WorkflowAutomationResult<()> {
        let request = WorkflowTemplateCatalog::built_in()
            .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
        let raw = raw_template(WorkflowTemplateId::ResearchBriefV1)?;
        let duplicate = raw.replacen(
            "\"version\":\"v1\"",
            "\"version\":\"v1\",\"version\":\"v1\"",
            1,
        );
        assert_eq!(
            request.parse_proposal(
                WorkflowProposalId::new("duplicate-key-proposal")?,
                &duplicate,
                &AgentRegistry::built_in()?,
            ),
            Err(WorkflowAutomationError::InvalidStructuredProposal)
        );
        let claim = raw.replace(
            "\"execution_authorized\":false",
            "\"execution_authorized\":true",
        );
        assert_eq!(
            request.parse_proposal(
                WorkflowProposalId::new("authority-claim-proposal")?,
                &claim,
                &AgentRegistry::built_in()?,
            ),
            Err(WorkflowAutomationError::AuthorityClaim)
        );
        Ok(())
    }

    #[test]
    fn debug_output_redacts_objectives_arguments_and_token_identity() -> WorkflowAutomationResult<()>
    {
        let request = WorkflowTemplateCatalog::built_in()
            .proposal_request(WorkflowTemplateId::ResearchBriefV1)?;
        assert!(!format!("{request:?}").contains(request.objective()));

        let tool = WorkflowGovernedToolStep {
            id: WorkflowStepId::new("tool")?,
            tool_name: "create_local_task".to_owned(),
            tool_contract_version: 1,
            arguments_json: r#"{"title":"private-title"}"#.to_owned(),
            dependencies: Vec::new(),
        };
        assert!(!format!("{tool:?}").contains("private-title"));
        Ok(())
    }

    #[test]
    fn synthesis_requires_complete_truthful_disclosure_and_rejects_unsafe_text(
    ) -> WorkflowAutomationResult<()> {
        let proposal_id = "truthful-synthesis-proposal";
        let (request, stage) = validated_stage(WorkflowTemplateId::ResearchBriefV1, proposal_id)?;
        let valid = synthesis_raw(
            WorkflowTemplateId::ResearchBriefV1,
            proposal_id,
            WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE,
            "complete",
            &[],
        )?;
        assert!(request.parse_synthesis(&stage, &valid).is_ok());

        for unsafe_suffix in [
            " The workflow executed successfully.",
            " A tool was invoked.",
            " Approval was granted.",
            " Execution was authorized.",
            " A device effect occurred.",
            " See https://example.invalid/evidence.",
            " Private reasoning: omitted.",
        ] {
            let summary = format!("{WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE}{unsafe_suffix}");
            let raw = synthesis_raw(
                WorkflowTemplateId::ResearchBriefV1,
                proposal_id,
                &summary,
                "complete",
                &[],
            )?;
            assert_eq!(
                request.parse_synthesis(&stage, &raw),
                Err(WorkflowAutomationError::AuthorityClaim),
                "unsafe suffix unexpectedly accepted: {unsafe_suffix}"
            );
        }

        let unsafe_issue = synthesis_raw(
            WorkflowTemplateId::ResearchBriefV1,
            proposal_id,
            WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE,
            "complete",
            &["See www.example.invalid or private_reasoning."],
        )?;
        assert_eq!(
            request.parse_synthesis(&stage, &unsafe_issue),
            Err(WorkflowAutomationError::AuthorityClaim)
        );

        let incomplete_disclosure = synthesis_raw(
            WorkflowTemplateId::ResearchBriefV1,
            proposal_id,
            "Fixture-only proposal; no effect occurred.",
            "complete",
            &[],
        )?;
        assert_eq!(
            request.parse_synthesis(&stage, &incomplete_disclosure),
            Err(WorkflowAutomationError::TemplateMismatch)
        );

        let partial_id = WorkflowProposalId::new("partial-synthesis-proposal")?;
        let rejected = WorkflowProposalStageOutcome::Rejected {
            proposal_id: partial_id.clone(),
            code: WorkflowValidationFailureCode::Cycle,
        };
        let partial_without_disclosure = serde_json::to_string(&json!({
            "version": "v1",
            "summary": WORKFLOW_AUTOMATION_SYNTHESIS_DISCLOSURE,
            "template_id": WorkflowTemplateId::ResearchBriefV1,
            "proposal_id": partial_id.as_str(),
            "validation_disposition": "unavailable",
            "status": "partial",
            "unresolved_issues": [WORKFLOW_AUTOMATION_REJECTED_ISSUE],
            "fixture_based": true,
            "unwired": true,
            "workflow_executed": false,
            "tools_executed": false,
            "approvals_requested": false,
            "effects_performed": false,
        }))
        .map_err(|_| WorkflowAutomationError::SerializationFailed)?;
        assert_eq!(
            request.parse_synthesis(&rejected, &partial_without_disclosure),
            Err(WorkflowAutomationError::TemplateMismatch)
        );
        Ok(())
    }
}
