//! Application-owned agent task domain.
//!
//! A task is one bounded, attributable unit of application work above a
//! runtime run. It is not an agent definition, provider session, model
//! request, tool request, policy decision, approval, audit record, or memory
//! namespace. Task content is untrusted data; identity, policy profile, lineage,
//! lifecycle, and runtime binding are derived and owned by the application.

use std::fmt;

use thiserror::Error;

use super::{
    definition::{AgentDefinitionIdentity, AgentId},
    gateway_protocol::is_valid_opaque_id,
    governance::AgentPolicyProfileId,
    runtime::{RuntimeFailureCode, RuntimeId, RuntimeRunIdentity},
};
use crate::memory::AgentMemoryProfileId;

pub const MAX_AGENT_TASK_OBJECTIVE_CHARACTERS: usize = 4_096;
pub const MAX_AGENT_TASK_OBJECTIVE_BYTES: usize = 16_384;
pub const MAX_AGENT_TASK_CONTEXT_CHARACTERS: usize = 2_048;
pub const MAX_AGENT_TASK_CONTEXT_BYTES: usize = 8_192;
pub const MAX_AGENT_TASK_EXPECTED_DELIVERABLE_CHARACTERS: usize = 512;
pub const MAX_AGENT_TASK_EXPECTED_DELIVERABLE_BYTES: usize = 2_048;
pub const MAX_AGENT_TASK_OUTPUT_CHARACTERS: usize = 8_192;
pub const MAX_AGENT_TASK_OUTPUT_BYTES: usize = 16_384;
pub const MAX_AGENT_TASK_DEPTH: u8 = 1;

pub type AgentTaskDomainResult<T> = Result<T, AgentTaskError>;

/// Opaque application-generated identity for one agent task.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AgentTaskId(String);

impl AgentTaskId {
    pub fn new(value: impl Into<String>) -> AgentTaskDomainResult<Self> {
        let value = value.into();
        if is_valid_opaque_id(&value) {
            Ok(Self(value))
        } else {
            Err(AgentTaskError::InvalidTaskId)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AgentTaskId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AgentTaskId")
            .field(&"[REDACTED]")
            .finish()
    }
}

/// Typed lineage identity for the root of a task tree.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RootTaskId(AgentTaskId);

impl RootTaskId {
    pub(super) fn from_task_id(task_id: AgentTaskId) -> Self {
        Self(task_id)
    }

    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.0
    }
}

impl fmt::Debug for RootTaskId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("RootTaskId")
            .field(&"[REDACTED]")
            .finish()
    }
}

/// Typed lineage identity for a child task's direct parent.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParentTaskId(AgentTaskId);

impl ParentTaskId {
    pub(super) fn from_task_id(task_id: AgentTaskId) -> Self {
        Self(task_id)
    }

    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.0
    }
}

impl fmt::Debug for ParentTaskId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ParentTaskId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct BoundedTaskText(String);

impl BoundedTaskText {
    fn new(
        value: impl Into<String>,
        field: AgentTaskContentField,
        maximum_characters: usize,
        maximum_bytes: usize,
    ) -> AgentTaskDomainResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AgentTaskError::InvalidContent {
                field,
                violation: AgentTaskContentViolation::Empty,
            });
        }
        if value.trim() != value {
            return Err(AgentTaskError::InvalidContent {
                field,
                violation: AgentTaskContentViolation::NonCanonical,
            });
        }
        if value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
        {
            return Err(AgentTaskError::InvalidContent {
                field,
                violation: AgentTaskContentViolation::ProhibitedControlCharacter,
            });
        }

        let actual_characters = value.chars().count();
        if actual_characters > maximum_characters {
            return Err(AgentTaskError::InvalidContent {
                field,
                violation: AgentTaskContentViolation::TooManyCharacters {
                    maximum: maximum_characters,
                    actual: actual_characters,
                },
            });
        }
        let actual_bytes = value.len();
        if actual_bytes > maximum_bytes {
            return Err(AgentTaskError::InvalidContent {
                field,
                violation: AgentTaskContentViolation::TooManyBytes {
                    maximum: maximum_bytes,
                    actual: actual_bytes,
                },
            });
        }

        Ok(Self(value))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

macro_rules! bounded_task_text {
    ($name:ident, $field:expr, $maximum_characters:expr, $maximum_bytes:expr) => {
        #[derive(Clone, Eq, PartialEq)]
        pub struct $name(BoundedTaskText);

        impl $name {
            pub fn new(value: impl Into<String>) -> AgentTaskDomainResult<Self> {
                BoundedTaskText::new(value, $field, $maximum_characters, $maximum_bytes).map(Self)
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("text", &"[REDACTED]")
                    .field("characters", &self.0.as_str().chars().count())
                    .field("bytes", &self.0.as_str().len())
                    .finish()
            }
        }
    };
}

bounded_task_text!(
    AgentTaskObjective,
    AgentTaskContentField::Objective,
    MAX_AGENT_TASK_OBJECTIVE_CHARACTERS,
    MAX_AGENT_TASK_OBJECTIVE_BYTES
);
bounded_task_text!(
    AgentTaskContext,
    AgentTaskContentField::DelegatedContext,
    MAX_AGENT_TASK_CONTEXT_CHARACTERS,
    MAX_AGENT_TASK_CONTEXT_BYTES
);
bounded_task_text!(
    AgentTaskExpectedDeliverable,
    AgentTaskContentField::ExpectedDeliverable,
    MAX_AGENT_TASK_EXPECTED_DELIVERABLE_CHARACTERS,
    MAX_AGENT_TASK_EXPECTED_DELIVERABLE_BYTES
);
bounded_task_text!(
    AgentTaskOutput,
    AgentTaskContentField::Output,
    MAX_AGENT_TASK_OUTPUT_CHARACTERS,
    MAX_AGENT_TASK_OUTPUT_BYTES
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentTaskStatus {
    Pending,
    Running,
    WaitingForChild,
    Completed,
    Failed,
    Cancelled,
}

impl AgentTaskStatus {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentTaskFailureCode {
    RuntimeUnavailable,
    RuntimeStartFailed,
    RuntimeStateMismatch,
    RuntimeEventRejected,
    RuntimeEventLimitExceeded,
    RuntimeOutputInvalid,
    DeadlineExceeded,
    RuntimeReported(RuntimeFailureCode),
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentTaskResult {
    task_id: AgentTaskId,
    agent_id: AgentId,
    output: AgentTaskOutput,
}

impl AgentTaskResult {
    pub(super) fn new(task_id: AgentTaskId, agent_id: AgentId, output: AgentTaskOutput) -> Self {
        Self {
            task_id,
            agent_id,
            output,
        }
    }

    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }

    #[must_use]
    pub fn output(&self) -> &AgentTaskOutput {
        &self.output
    }
}

impl fmt::Debug for AgentTaskResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentTaskResult")
            .field("task_id", &self.task_id)
            .field("agent_id", &self.agent_id)
            .field("output", &self.output)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentTaskFailure {
    task_id: AgentTaskId,
    agent_id: AgentId,
    code: AgentTaskFailureCode,
}

impl AgentTaskFailure {
    pub(super) fn new(task_id: AgentTaskId, agent_id: AgentId, code: AgentTaskFailureCode) -> Self {
        Self {
            task_id,
            agent_id,
            code,
        }
    }

    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }

    #[must_use]
    pub const fn code(&self) -> AgentTaskFailureCode {
        self.code
    }
}

impl fmt::Debug for AgentTaskFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentTaskFailure")
            .field("task_id", &self.task_id)
            .field("agent_id", &self.agent_id)
            .field("code", &self.code)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentTaskCancellation {
    task_id: AgentTaskId,
    agent_id: AgentId,
}

impl AgentTaskCancellation {
    pub(super) fn new(task_id: AgentTaskId, agent_id: AgentId) -> Self {
        Self { task_id, agent_id }
    }

    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }
}

impl fmt::Debug for AgentTaskCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentTaskCancellation")
            .field("task_id", &self.task_id)
            .field("agent_id", &self.agent_id)
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentTaskOutcome {
    Completed(AgentTaskResult),
    Failed(AgentTaskFailure),
    Cancelled(AgentTaskCancellation),
}

impl AgentTaskOutcome {
    #[must_use]
    pub const fn kind(&self) -> AgentTaskOutcomeKind {
        match self {
            Self::Completed(_) => AgentTaskOutcomeKind::Completed,
            Self::Failed(_) => AgentTaskOutcomeKind::Failed,
            Self::Cancelled(_) => AgentTaskOutcomeKind::Cancelled,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentTaskOutcomeKind {
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentTaskCancellationOutcome {
    Cancelled,
    AlreadyTerminal(AgentTaskStatus),
}

/// Trusted application context bound to one concrete runtime run.
#[derive(Clone, Eq, PartialEq)]
pub struct AgentExecutionContext {
    agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    task_id: AgentTaskId,
    root_task_id: RootTaskId,
    parent_task_id: Option<ParentTaskId>,
    runtime_id: RuntimeId,
    depth: u8,
    runtime_run_identity: RuntimeRunIdentity,
}

impl AgentExecutionContext {
    pub(super) fn for_task(
        task: &AgentTask,
        runtime_id: RuntimeId,
        runtime_run_identity: RuntimeRunIdentity,
    ) -> Self {
        Self {
            agent_id: task.agent_id,
            policy_profile_id: task.policy_profile_id,
            memory_profile_id: task.memory_profile_id,
            task_id: task.id.clone(),
            root_task_id: task.root_task_id.clone(),
            parent_task_id: task.parent_task_id.clone(),
            runtime_id,
            depth: task.depth,
            runtime_run_identity,
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

    #[must_use]
    pub fn runtime_run_identity(&self) -> &RuntimeRunIdentity {
        &self.runtime_run_identity
    }

    #[must_use]
    pub(super) fn matches_task(&self, task: &AgentTask) -> bool {
        self.agent_id == task.agent_id
            && self.policy_profile_id == task.policy_profile_id
            && self.memory_profile_id == task.memory_profile_id
            && self.task_id == task.id
            && self.root_task_id == task.root_task_id
            && self.parent_task_id == task.parent_task_id
            && self.depth == task.depth
    }
}

impl fmt::Debug for AgentExecutionContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentExecutionContext")
            .field("agent_id", &self.agent_id)
            .field("policy_profile_id", &self.policy_profile_id)
            .field("memory_profile_id", &self.memory_profile_id)
            .field("task_id", &self.task_id)
            .field("root_task_id", &self.root_task_id)
            .field("parent_task_id", &self.parent_task_id)
            .field("runtime_id", &self.runtime_id)
            .field("depth", &self.depth)
            .field("runtime_run_identity", &self.runtime_run_identity)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentTask {
    id: AgentTaskId,
    root_task_id: RootTaskId,
    parent_task_id: Option<ParentTaskId>,
    agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    depth: u8,
    objective: AgentTaskObjective,
    delegated_context: Option<AgentTaskContext>,
    expected_deliverable: Option<AgentTaskExpectedDeliverable>,
    status: AgentTaskStatus,
    outcome: Option<AgentTaskOutcome>,
}

impl AgentTask {
    pub(super) fn new_root(
        id: AgentTaskId,
        definition_identity: AgentDefinitionIdentity,
        objective: AgentTaskObjective,
    ) -> Self {
        Self {
            root_task_id: RootTaskId::from_task_id(id.clone()),
            id,
            parent_task_id: None,
            agent_id: definition_identity.agent_id(),
            policy_profile_id: definition_identity.policy_profile_id(),
            memory_profile_id: definition_identity.memory_profile_id(),
            depth: 0,
            objective,
            delegated_context: None,
            expected_deliverable: None,
            status: AgentTaskStatus::Pending,
            outcome: None,
        }
    }

    pub(super) fn new_child(
        id: AgentTaskId,
        root_task_id: RootTaskId,
        parent_task_id: ParentTaskId,
        definition_identity: AgentDefinitionIdentity,
        objective: AgentTaskObjective,
        delegated_context: Option<AgentTaskContext>,
        expected_deliverable: AgentTaskExpectedDeliverable,
    ) -> AgentTaskDomainResult<Self> {
        if root_task_id.task_id() != parent_task_id.task_id() {
            return Err(AgentTaskError::InvalidLineage(
                AgentTaskLineageError::ParentMustBeRoot,
            ));
        }
        if &id == root_task_id.task_id() || &id == parent_task_id.task_id() {
            return Err(AgentTaskError::InvalidLineage(
                AgentTaskLineageError::DuplicateTaskIdentity,
            ));
        }

        Ok(Self {
            id,
            root_task_id,
            parent_task_id: Some(parent_task_id),
            agent_id: definition_identity.agent_id(),
            policy_profile_id: definition_identity.policy_profile_id(),
            memory_profile_id: definition_identity.memory_profile_id(),
            depth: MAX_AGENT_TASK_DEPTH,
            objective,
            delegated_context,
            expected_deliverable: Some(expected_deliverable),
            status: AgentTaskStatus::Pending,
            outcome: None,
        })
    }

    #[must_use]
    pub fn id(&self) -> &AgentTaskId {
        &self.id
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
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    #[must_use]
    pub fn objective(&self) -> &AgentTaskObjective {
        &self.objective
    }

    #[must_use]
    pub fn delegated_context(&self) -> Option<&AgentTaskContext> {
        self.delegated_context.as_ref()
    }

    #[must_use]
    pub fn expected_deliverable(&self) -> Option<&AgentTaskExpectedDeliverable> {
        self.expected_deliverable.as_ref()
    }

    #[must_use]
    pub const fn status(&self) -> AgentTaskStatus {
        self.status
    }

    #[must_use]
    pub fn outcome(&self) -> Option<&AgentTaskOutcome> {
        self.outcome.as_ref()
    }

    pub(super) fn start(&mut self) -> AgentTaskDomainResult<()> {
        self.transition(AgentTaskStatus::Running)
    }

    pub(super) fn wait_for_child(&mut self) -> AgentTaskDomainResult<()> {
        self.transition(AgentTaskStatus::WaitingForChild)
    }

    pub(super) fn resume_from_child(&mut self) -> AgentTaskDomainResult<()> {
        self.transition(AgentTaskStatus::Running)
    }

    pub(super) fn complete(&mut self, result: AgentTaskResult) -> AgentTaskDomainResult<()> {
        self.validate_outcome_identity(result.task_id(), result.agent_id())?;
        self.transition(AgentTaskStatus::Completed)?;
        self.outcome = Some(AgentTaskOutcome::Completed(result));
        Ok(())
    }

    pub(super) fn fail(&mut self, failure: AgentTaskFailure) -> AgentTaskDomainResult<()> {
        self.validate_outcome_identity(failure.task_id(), failure.agent_id())?;
        self.transition(AgentTaskStatus::Failed)?;
        self.outcome = Some(AgentTaskOutcome::Failed(failure));
        Ok(())
    }

    pub(super) fn cancel(&mut self) -> AgentTaskCancellationOutcome {
        if self.status.is_terminal() {
            return AgentTaskCancellationOutcome::AlreadyTerminal(self.status);
        }

        self.status = AgentTaskStatus::Cancelled;
        self.outcome = Some(AgentTaskOutcome::Cancelled(AgentTaskCancellation::new(
            self.id.clone(),
            self.agent_id,
        )));
        AgentTaskCancellationOutcome::Cancelled
    }

    fn validate_outcome_identity(
        &self,
        task_id: &AgentTaskId,
        agent_id: AgentId,
    ) -> AgentTaskDomainResult<()> {
        if task_id == &self.id && agent_id == self.agent_id {
            Ok(())
        } else {
            Err(AgentTaskError::OutcomeIdentityMismatch)
        }
    }

    fn transition(&mut self, target: AgentTaskStatus) -> AgentTaskDomainResult<()> {
        let allowed = matches!(
            (self.status, target),
            (
                AgentTaskStatus::Pending,
                AgentTaskStatus::Running | AgentTaskStatus::Failed | AgentTaskStatus::Cancelled
            ) | (
                AgentTaskStatus::Running,
                AgentTaskStatus::WaitingForChild
                    | AgentTaskStatus::Completed
                    | AgentTaskStatus::Failed
                    | AgentTaskStatus::Cancelled
            ) | (
                AgentTaskStatus::WaitingForChild,
                AgentTaskStatus::Running | AgentTaskStatus::Failed | AgentTaskStatus::Cancelled
            )
        );
        if !allowed {
            return Err(AgentTaskError::InvalidTransition {
                from: self.status,
                to: target,
            });
        }

        self.status = target;
        Ok(())
    }
}

impl fmt::Debug for AgentTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentTask")
            .field("id", &self.id)
            .field("root_task_id", &self.root_task_id)
            .field("parent_task_id", &self.parent_task_id)
            .field("agent_id", &self.agent_id)
            .field("policy_profile_id", &self.policy_profile_id)
            .field("memory_profile_id", &self.memory_profile_id)
            .field("depth", &self.depth)
            .field("objective", &self.objective)
            .field("delegated_context", &self.delegated_context)
            .field("expected_deliverable", &self.expected_deliverable)
            .field("status", &self.status)
            .field("outcome", &self.outcome)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentTaskContentField {
    Objective,
    DelegatedContext,
    ExpectedDeliverable,
    Output,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentTaskContentViolation {
    Empty,
    NonCanonical,
    ProhibitedControlCharacter,
    TooManyCharacters { maximum: usize, actual: usize },
    TooManyBytes { maximum: usize, actual: usize },
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum AgentTaskLineageError {
    #[error("a child task identity must differ from its root and parent")]
    DuplicateTaskIdentity,
    #[error("the initial depth-one child must have the root task as its direct parent")]
    ParentMustBeRoot,
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum AgentTaskError {
    #[error("agent task id is invalid")]
    InvalidTaskId,
    #[error("agent task content is invalid for {field:?}: {violation:?}")]
    InvalidContent {
        field: AgentTaskContentField,
        violation: AgentTaskContentViolation,
    },
    #[error(transparent)]
    InvalidLineage(AgentTaskLineageError),
    #[error("agent task transition from {from:?} to {to:?} is invalid")]
    InvalidTransition {
        from: AgentTaskStatus,
        to: AgentTaskStatus,
    },
    #[error("agent task terminal outcome identity does not match the task")]
    OutcomeIdentityMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{definition::AgentDefinition, runtime::RuntimeTurnRequest};
    use crate::memory::AgentMemoryProfileId;

    fn task_id(value: &str) -> AgentTaskDomainResult<AgentTaskId> {
        AgentTaskId::new(value)
    }

    fn objective(value: &str) -> AgentTaskDomainResult<AgentTaskObjective> {
        AgentTaskObjective::new(value)
    }

    fn definition_identity(agent_id: AgentId) -> AgentTaskDomainResult<AgentDefinitionIdentity> {
        AgentDefinition::built_in(agent_id)
            .map(|definition| definition.identity())
            .map_err(|_| AgentTaskError::InvalidTaskId)
    }

    fn root_task() -> AgentTaskDomainResult<AgentTask> {
        Ok(AgentTask::new_root(
            task_id("root-task-1")?,
            definition_identity(AgentId::PersonalAssistant)?,
            objective("Answer one bounded question")?,
        ))
    }

    fn child_task() -> AgentTaskDomainResult<AgentTask> {
        let root_id = task_id("root-task-1")?;
        AgentTask::new_child(
            task_id("child-task-1")?,
            RootTaskId::from_task_id(root_id.clone()),
            ParentTaskId::from_task_id(root_id),
            definition_identity(AgentId::Research)?,
            objective("Compare the supplied evidence")?,
            Some(AgentTaskContext::new("Only use the supplied fixtures")?),
            AgentTaskExpectedDeliverable::new("Return one attributed summary")?,
        )
    }

    #[test]
    fn task_ids_are_closed_owned_orderable_and_redacted() -> AgentTaskDomainResult<()> {
        assert_eq!(AgentTaskId::new(""), Err(AgentTaskError::InvalidTaskId));
        assert_eq!(
            AgentTaskId::new("task id with spaces"),
            Err(AgentTaskError::InvalidTaskId)
        );

        let first = task_id("task-1")?;
        let second = task_id("task-2")?;
        assert!(first < second);
        assert_eq!(first.clone(), first);
        assert_eq!(first.as_str(), "task-1");
        assert_eq!(format!("{first:?}"), "AgentTaskId(\"[REDACTED]\")");
        Ok(())
    }

    #[test]
    fn each_content_type_enforces_its_exact_character_and_byte_limits() -> AgentTaskDomainResult<()>
    {
        let exact_objective = "x".repeat(MAX_AGENT_TASK_OBJECTIVE_CHARACTERS);
        assert_eq!(
            AgentTaskObjective::new(&exact_objective)?.as_str(),
            exact_objective
        );
        assert!(matches!(
            AgentTaskObjective::new("x".repeat(MAX_AGENT_TASK_OBJECTIVE_CHARACTERS + 1)),
            Err(AgentTaskError::InvalidContent {
                field: AgentTaskContentField::Objective,
                violation: AgentTaskContentViolation::TooManyCharacters { .. }
            })
        ));

        let exact_context = "🦀".repeat(MAX_AGENT_TASK_CONTEXT_CHARACTERS);
        assert_eq!(
            AgentTaskContext::new(&exact_context)?.as_str().len(),
            MAX_AGENT_TASK_CONTEXT_BYTES
        );

        let exact_deliverable = "x".repeat(MAX_AGENT_TASK_EXPECTED_DELIVERABLE_CHARACTERS);
        assert_eq!(
            AgentTaskExpectedDeliverable::new(&exact_deliverable)?
                .as_str()
                .chars()
                .count(),
            MAX_AGENT_TASK_EXPECTED_DELIVERABLE_CHARACTERS
        );
        let exact_output = "x".repeat(MAX_AGENT_TASK_OUTPUT_CHARACTERS);
        assert_eq!(
            AgentTaskOutput::new(&exact_output)?
                .as_str()
                .chars()
                .count(),
            MAX_AGENT_TASK_OUTPUT_CHARACTERS
        );
        assert!(matches!(
            AgentTaskOutput::new("€".repeat((MAX_AGENT_TASK_OUTPUT_BYTES / 3) + 1)),
            Err(AgentTaskError::InvalidContent {
                field: AgentTaskContentField::Output,
                violation: AgentTaskContentViolation::TooManyBytes { .. }
            })
        ));
        Ok(())
    }

    #[test]
    fn content_rejects_empty_noncanonical_and_prohibited_controls_without_echoing_content() {
        assert_eq!(
            AgentTaskObjective::new("  "),
            Err(AgentTaskError::InvalidContent {
                field: AgentTaskContentField::Objective,
                violation: AgentTaskContentViolation::Empty,
            })
        );
        assert_eq!(
            AgentTaskContext::new(" context"),
            Err(AgentTaskError::InvalidContent {
                field: AgentTaskContentField::DelegatedContext,
                violation: AgentTaskContentViolation::NonCanonical,
            })
        );
        assert_eq!(
            AgentTaskExpectedDeliverable::new("unsafe\u{0000}content"),
            Err(AgentTaskError::InvalidContent {
                field: AgentTaskContentField::ExpectedDeliverable,
                violation: AgentTaskContentViolation::ProhibitedControlCharacter,
            })
        );
        assert!(AgentTaskOutput::new("line one\nline two\tvalue").is_ok());
        assert!(!AgentTaskObjective::new("private-objective-sentinel")
            .map(|value| format!("{value:?}"))
            .is_ok_and(|debug| debug.contains("private-objective-sentinel")));
    }

    #[test]
    fn root_and_child_lineage_are_exact_and_bounded() -> AgentTaskDomainResult<()> {
        let root = root_task()?;
        assert_eq!(root.id(), root.root_task_id().task_id());
        assert_eq!(root.parent_task_id(), None);
        assert_eq!(root.agent_id(), AgentId::PersonalAssistant);
        assert_eq!(
            root.policy_profile_id(),
            AgentPolicyProfileId::PersonalAssistantV1
        );
        assert_eq!(
            root.memory_profile_id(),
            AgentMemoryProfileId::PersonalAssistantMemoryV1
        );
        assert_eq!(root.depth(), 0);
        assert_eq!(root.status(), AgentTaskStatus::Pending);

        let child = child_task()?;
        assert_eq!(child.root_task_id().task_id().as_str(), "root-task-1");
        assert_eq!(
            child.parent_task_id().map(ParentTaskId::task_id),
            Some(child.root_task_id().task_id())
        );
        assert_eq!(child.agent_id(), AgentId::Research);
        assert_eq!(
            child.policy_profile_id(),
            AgentPolicyProfileId::ResearchReadOnlyV1
        );
        assert_eq!(
            child.memory_profile_id(),
            AgentMemoryProfileId::ResearchWorkingMemoryV1
        );
        assert_eq!(child.depth(), MAX_AGENT_TASK_DEPTH);
        assert!(child.delegated_context().is_some());
        assert!(child.expected_deliverable().is_some());
        Ok(())
    }

    #[test]
    fn child_lineage_rejects_duplicate_and_non_root_parent_before_creation(
    ) -> AgentTaskDomainResult<()> {
        let root_id = task_id("root-task-1")?;
        let child = AgentTask::new_child(
            root_id.clone(),
            RootTaskId::from_task_id(root_id.clone()),
            ParentTaskId::from_task_id(root_id),
            definition_identity(AgentId::Research)?,
            objective("Research")?,
            None,
            AgentTaskExpectedDeliverable::new("Summary")?,
        );
        assert_eq!(
            child,
            Err(AgentTaskError::InvalidLineage(
                AgentTaskLineageError::DuplicateTaskIdentity
            ))
        );

        let wrong_parent = AgentTask::new_child(
            task_id("child-task-1")?,
            RootTaskId::from_task_id(task_id("root-task-1")?),
            ParentTaskId::from_task_id(task_id("other-parent")?),
            definition_identity(AgentId::Research)?,
            objective("Research")?,
            None,
            AgentTaskExpectedDeliverable::new("Summary")?,
        );
        assert_eq!(
            wrong_parent,
            Err(AgentTaskError::InvalidLineage(
                AgentTaskLineageError::ParentMustBeRoot
            ))
        );
        Ok(())
    }

    #[test]
    fn direct_and_waiting_lifecycles_accept_only_explicit_transitions() -> AgentTaskDomainResult<()>
    {
        let mut direct = root_task()?;
        direct.start()?;
        direct.complete(AgentTaskResult::new(
            direct.id().clone(),
            direct.agent_id(),
            AgentTaskOutput::new("Direct answer")?,
        ))?;
        assert_eq!(direct.status(), AgentTaskStatus::Completed);
        assert_eq!(
            direct.outcome().map(AgentTaskOutcome::kind),
            Some(AgentTaskOutcomeKind::Completed)
        );

        let mut delegated = root_task()?;
        delegated.start()?;
        delegated.wait_for_child()?;
        assert_eq!(delegated.status(), AgentTaskStatus::WaitingForChild);
        delegated.resume_from_child()?;
        assert_eq!(delegated.status(), AgentTaskStatus::Running);
        Ok(())
    }

    #[test]
    fn transition_table_accepts_only_the_closed_state_machine() -> AgentTaskDomainResult<()> {
        let statuses = [
            AgentTaskStatus::Pending,
            AgentTaskStatus::Running,
            AgentTaskStatus::WaitingForChild,
            AgentTaskStatus::Completed,
            AgentTaskStatus::Failed,
            AgentTaskStatus::Cancelled,
        ];
        for from in statuses {
            for to in statuses {
                let expected_allowed = matches!(
                    (from, to),
                    (
                        AgentTaskStatus::Pending,
                        AgentTaskStatus::Running
                            | AgentTaskStatus::Failed
                            | AgentTaskStatus::Cancelled
                    ) | (
                        AgentTaskStatus::Running,
                        AgentTaskStatus::WaitingForChild
                            | AgentTaskStatus::Completed
                            | AgentTaskStatus::Failed
                            | AgentTaskStatus::Cancelled
                    ) | (
                        AgentTaskStatus::WaitingForChild,
                        AgentTaskStatus::Running
                            | AgentTaskStatus::Failed
                            | AgentTaskStatus::Cancelled
                    )
                );
                let mut task = root_task()?;
                task.status = from;
                let result = task.transition(to);
                assert_eq!(result.is_ok(), expected_allowed, "{from:?} -> {to:?}");
                assert_eq!(
                    task.status(),
                    if expected_allowed { to } else { from },
                    "{from:?} -> {to:?} must be atomic"
                );
            }
        }
        Ok(())
    }

    #[test]
    fn invalid_and_mismatched_terminal_transitions_are_typed_and_atomic(
    ) -> AgentTaskDomainResult<()> {
        let mut task = root_task()?;
        assert_eq!(
            task.wait_for_child(),
            Err(AgentTaskError::InvalidTransition {
                from: AgentTaskStatus::Pending,
                to: AgentTaskStatus::WaitingForChild,
            })
        );
        assert_eq!(task.status(), AgentTaskStatus::Pending);

        task.start()?;
        let foreign = AgentTaskResult::new(
            task_id("foreign-task")?,
            task.agent_id(),
            AgentTaskOutput::new("Foreign output")?,
        );
        assert_eq!(
            task.complete(foreign),
            Err(AgentTaskError::OutcomeIdentityMismatch)
        );
        assert_eq!(task.status(), AgentTaskStatus::Running);
        assert_eq!(task.outcome(), None);

        task.complete(AgentTaskResult::new(
            task.id().clone(),
            task.agent_id(),
            AgentTaskOutput::new("Final output")?,
        ))?;
        assert_eq!(
            task.start(),
            Err(AgentTaskError::InvalidTransition {
                from: AgentTaskStatus::Completed,
                to: AgentTaskStatus::Running,
            })
        );
        assert!(matches!(
            task.outcome(),
            Some(AgentTaskOutcome::Completed(result))
                if result.output().as_str() == "Final output"
        ));
        Ok(())
    }

    #[test]
    fn failure_and_cancellation_are_attributed_terminal_and_idempotent() -> AgentTaskDomainResult<()>
    {
        let mut failed = child_task()?;
        failed.start()?;
        let failure = AgentTaskFailure::new(
            failed.id().clone(),
            failed.agent_id(),
            AgentTaskFailureCode::RuntimeReported(RuntimeFailureCode::ProviderTimeout),
        );
        failed.fail(failure)?;
        assert_eq!(failed.status(), AgentTaskStatus::Failed);
        assert!(matches!(
            failed.outcome(),
            Some(AgentTaskOutcome::Failed(failure))
                if failure.task_id() == failed.id()
                    && failure.agent_id() == AgentId::Research
                    && failure.code()
                        == AgentTaskFailureCode::RuntimeReported(
                            RuntimeFailureCode::ProviderTimeout
                        )
        ));

        let mut cancelled = root_task()?;
        assert_eq!(cancelled.cancel(), AgentTaskCancellationOutcome::Cancelled);
        assert_eq!(cancelled.status(), AgentTaskStatus::Cancelled);
        assert_eq!(
            cancelled.cancel(),
            AgentTaskCancellationOutcome::AlreadyTerminal(AgentTaskStatus::Cancelled)
        );
        assert!(matches!(
            cancelled.outcome(),
            Some(AgentTaskOutcome::Cancelled(outcome))
                if outcome.task_id() == cancelled.id()
                    && outcome.agent_id() == AgentId::PersonalAssistant
        ));
        Ok(())
    }

    #[test]
    fn execution_context_is_derived_from_task_and_runtime_identity(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task = child_task()?;
        let request = RuntimeTurnRequest::new("child-run-1", "child-request-1", "Research")?;
        let identity = request.identity();
        let context = AgentExecutionContext::for_task(&task, RuntimeId::Native, identity.clone());

        assert!(context.matches_task(&task));
        assert_eq!(context.agent_id(), AgentId::Research);
        assert_eq!(
            context.policy_profile_id(),
            AgentPolicyProfileId::ResearchReadOnlyV1
        );
        assert_eq!(
            context.memory_profile_id(),
            AgentMemoryProfileId::ResearchWorkingMemoryV1
        );
        assert_eq!(context.task_id(), task.id());
        assert_eq!(context.root_task_id(), task.root_task_id());
        assert_eq!(context.parent_task_id(), task.parent_task_id());
        assert_eq!(context.runtime_id(), RuntimeId::Native);
        assert_eq!(context.depth(), MAX_AGENT_TASK_DEPTH);
        assert_eq!(context.runtime_run_identity(), &identity);

        let mut forged_profile = context.clone();
        forged_profile.policy_profile_id = AgentPolicyProfileId::PersonalAssistantV1;
        assert!(!forged_profile.matches_task(&task));

        let mut forged_memory_profile = context.clone();
        forged_memory_profile.memory_profile_id = AgentMemoryProfileId::PersonalAssistantMemoryV1;
        assert!(!forged_memory_profile.matches_task(&task));
        Ok(())
    }

    #[test]
    fn debug_surfaces_redact_all_task_content_and_identities() -> AgentTaskDomainResult<()> {
        let sentinel = "private-task-content-sentinel";
        let mut task = AgentTask::new_root(
            task_id("private-task-id-sentinel")?,
            definition_identity(AgentId::PersonalAssistant)?,
            objective(sentinel)?,
        );
        task.start()?;
        task.complete(AgentTaskResult::new(
            task.id().clone(),
            task.agent_id(),
            AgentTaskOutput::new(sentinel)?,
        ))?;

        let debug = format!("{task:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(sentinel));
        assert!(!debug.contains("private-task-id-sentinel"));
        Ok(())
    }
}
