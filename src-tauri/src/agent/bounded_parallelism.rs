//! Closed fixture-only contracts for D-091 bounded parallel orchestration.
//!
//! "Parallel" in this module means that one application-owned orchestrator may
//! retain several independent runtime runs and accept their events in an
//! interleaved order. It does not imply threads, provider concurrency, remote
//! workers, tool execution, or device authority.

mod catalog;
mod validation;

use std::{fmt, time::Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    definition::AgentId,
    governance::AgentPolicyProfileId,
    runtime::{RuntimeId, RuntimeRunIdentity},
    task::{AgentExecutionContext, AgentTaskFailureCode, AgentTaskId, RootTaskId},
};
use crate::memory::AgentMemoryProfileId;

pub const BOUNDED_PARALLEL_CONTRACT_VERSION: u16 = 1;
pub const DEFAULT_ACTIVE_PARALLEL_CHILDREN: u8 = 2;
pub const HARD_MAX_ACTIVE_PARALLEL_CHILDREN: u8 = 3;
pub const MAX_PARALLEL_CHILDREN: usize = 3;
pub const MAX_PARALLEL_TASKS_PER_ROOT: usize = 4;
pub const MAX_PARALLEL_RUNTIME_RUNS_PER_ROOT: u8 = 5;
pub const MAX_PARALLEL_EVENTS_PER_RUN: u32 = 8;
pub const MAX_PARALLEL_RUNTIME_EVENTS: usize = 32;
pub const MAX_PARALLEL_ORCHESTRATION_EVENTS: usize = 32;
pub const MAX_PARALLEL_WORKFLOW_EVENTS: usize = 32;
pub const MAX_PARALLEL_AUDIT_RECORDS: usize = 32;
pub const MAX_PARALLEL_ROOT_DURATION_SECONDS: u64 = 120;
pub const MAX_PARALLEL_CHILD_DURATION_SECONDS: u64 = 60;
pub const MAX_PARALLEL_RETRIES: u8 = 0;
pub const MAX_PARALLEL_OBJECTIVE_CHARACTERS: usize = 1_024;
pub const MAX_PARALLEL_OBJECTIVE_BYTES: usize = 4_096;
pub const MAX_PARALLEL_FIXTURES: usize = 8;
pub const MAX_PARALLEL_FIXTURE_CHARACTERS: usize = 2_048;
pub const MAX_PARALLEL_FIXTURE_BYTES: usize = 4_096;
pub const MAX_PARALLEL_CATALOG_BYTES: usize = 16_384;
pub const MAX_PARALLEL_RAW_RESULT_CHARACTERS: usize = 6_144;
pub const MAX_PARALLEL_RAW_RESULT_BYTES: usize = 12_288;
pub const MAX_PARALLEL_SYNTHESIS_OUTPUT_CHARACTERS: usize = 4_096;
pub const MAX_PARALLEL_SYNTHESIS_OUTPUT_BYTES: usize = 8_192;
pub const MAX_PARALLEL_FINDINGS: usize = 3;
pub const MAX_PARALLEL_UNRESOLVED_ISSUES: usize = 1;
pub const MAX_PARALLEL_REFERENCES_PER_FINDING: usize = 4;
pub const MAX_PARALLEL_TEXT_CHARACTERS: usize = 256;
pub const MAX_PARALLEL_TEXT_BYTES: usize = 1_024;
pub const MAX_PARALLEL_SYNTHESIS_ISSUE_CHARACTERS: usize = 322;
pub const MAX_PARALLEL_SYNTHESIS_ISSUE_BYTES: usize = 1_090;
pub const MAX_PARALLEL_RESULT_TEXT_BYTES: usize = 4_096;
pub const MAX_PARALLEL_CHILD_TRANSFER_BYTES: usize = 6_144;
pub const MAX_PARALLEL_ORDERED_TRANSFER_BYTES: usize = 18_432;
pub const MAX_PARALLEL_SYNTHESIS_FRAMING_BYTES: usize = 2_048;
pub const MAX_PARALLEL_SYNTHESIS_INPUT_BYTES: usize = 24_576;
pub const MAX_PARALLEL_SYNTHESIS_CHARACTERS: usize = 4_096;
pub const MAX_PARALLEL_SYNTHESIS_BYTES: usize = 8_192;
pub const MAX_PARALLEL_SYNTHESIS_SUMMARY_CHARACTERS: usize = 1_024;
pub const MAX_PARALLEL_SYNTHESIS_SUMMARY_BYTES: usize = 2_048;
pub const MAX_PARALLEL_STATUS_TABLE_BYTES: usize = 2_048;
pub const BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE: &str =
    "Fixture-based only; no live access, tool, credential, or external effect occurred.";

pub type BoundedParallelResult<T> = Result<T, BoundedParallelError>;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BoundedParallelScenarioId {
    ResearchKnowledgeIndependentV1,
    CodeSecurityQaV1,
    CloudSystemsSecurityV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedParallelFailurePolicy {
    ContinuePartial,
    CancelDependentOnly,
    FailFast,
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParallelWorkItemId(String);

impl ParallelWorkItemId {
    pub fn new(value: impl Into<String>) -> BoundedParallelResult<Self> {
        let value = value.into();
        let Some(first) = value.bytes().next() else {
            return Err(BoundedParallelError::InvalidWorkItemId);
        };
        if value.len() > 64
            || !first.is_ascii_lowercase()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(BoundedParallelError::InvalidWorkItemId);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ParallelWorkItemId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ParallelWorkItemId")
            .field(&self.0)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParallelFixture {
    id: String,
    label: String,
    content: String,
}

impl ParallelFixture {
    pub(super) fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        content: impl Into<String>,
    ) -> BoundedParallelResult<Self> {
        let value = Self {
            id: id.into(),
            label: label.into(),
            content: content.into(),
        };
        validation::validate_identifier(&value.id)?;
        validation::validate_text(&value.label, 256, 1_024, false)?;
        validation::validate_text(
            &value.content,
            MAX_PARALLEL_FIXTURE_CHARACTERS,
            MAX_PARALLEL_FIXTURE_BYTES,
            true,
        )?;
        Ok(value)
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl fmt::Debug for ParallelFixture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParallelFixture")
            .field("id", &self.id)
            .field("label", &"[REDACTED]")
            .field("content", &"[REDACTED]")
            .field("content_bytes", &self.content.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParallelWorkItem {
    id: ParallelWorkItemId,
    ordinal: u8,
    agent_id: AgentId,
    dependencies: Vec<ParallelWorkItemId>,
    objective: String,
    expected_output: String,
    fixtures: Vec<ParallelFixture>,
}

impl ParallelWorkItem {
    #[must_use]
    pub fn id(&self) -> &ParallelWorkItemId {
        &self.id
    }
    #[must_use]
    pub const fn ordinal(&self) -> u8 {
        self.ordinal
    }
    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }
    #[must_use]
    pub fn dependencies(&self) -> &[ParallelWorkItemId] {
        &self.dependencies
    }
    #[must_use]
    pub fn objective(&self) -> &str {
        &self.objective
    }
    #[must_use]
    pub fn expected_output(&self) -> &str {
        &self.expected_output
    }
    #[must_use]
    pub fn fixtures(&self) -> &[ParallelFixture] {
        &self.fixtures
    }

    pub(super) fn fixture_known(&self, id: &str) -> bool {
        self.fixtures.iter().any(|fixture| fixture.id == id)
    }
}

impl fmt::Debug for ParallelWorkItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParallelWorkItem")
            .field("id", &self.id)
            .field("ordinal", &self.ordinal)
            .field("agent_id", &self.agent_id)
            .field("dependencies", &self.dependencies)
            .field("objective", &"[REDACTED]")
            .field("expected_output", &"[REDACTED]")
            .field("fixture_count", &self.fixtures.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BoundedParallelWorkflowRequest {
    scenario_id: BoundedParallelScenarioId,
    objective: String,
    failure_policy: BoundedParallelFailurePolicy,
    work_items: Vec<ParallelWorkItem>,
    active_limit: u8,
}

impl BoundedParallelWorkflowRequest {
    #[must_use]
    pub const fn scenario_id(&self) -> BoundedParallelScenarioId {
        self.scenario_id
    }
    #[must_use]
    pub fn objective(&self) -> &str {
        &self.objective
    }
    #[must_use]
    pub const fn failure_policy(&self) -> BoundedParallelFailurePolicy {
        self.failure_policy
    }
    #[must_use]
    pub fn work_items(&self) -> &[ParallelWorkItem] {
        &self.work_items
    }
    #[must_use]
    pub const fn active_limit(&self) -> u8 {
        self.active_limit
    }

    #[cfg(test)]
    pub(super) fn with_active_limit_for_test(mut self, value: u8) -> BoundedParallelResult<Self> {
        if value == 0 || value > HARD_MAX_ACTIVE_PARALLEL_CHILDREN {
            return Err(BoundedParallelError::ActiveLimitExceeded);
        }
        self.active_limit = value;
        Ok(self)
    }

    #[cfg(test)]
    pub(super) fn with_unchecked_active_limit_for_test(mut self, value: u8) -> Self {
        self.active_limit = value;
        self
    }

    #[cfg(test)]
    pub(super) fn with_duplicated_work_item_for_test(mut self) -> Self {
        if self.work_items.len() >= 2 {
            self.work_items[1] = self.work_items[0].clone();
        }
        self
    }

    #[cfg(test)]
    pub(super) fn with_self_dependency_for_test(mut self, ordinal: u8) -> Self {
        if let Some(item) = self
            .work_items
            .get_mut(usize::from(ordinal.saturating_sub(1)))
        {
            item.dependencies = vec![item.id.clone()];
        }
        self
    }

    #[cfg(test)]
    pub(super) fn with_objective_for_test(mut self, objective: impl Into<String>) -> Self {
        self.objective = objective.into();
        self
    }

    #[cfg(test)]
    pub(super) fn with_oversized_dependent_fixture_for_test(mut self) -> Self {
        if let Some(fixture) = self
            .work_items
            .iter_mut()
            .find(|item| !item.dependencies.is_empty())
            .and_then(|item| item.fixtures.first_mut())
        {
            fixture.content = "x".repeat(MAX_PARALLEL_SYNTHESIS_INPUT_BYTES + 1);
        }
        self
    }

    #[cfg(test)]
    pub(super) fn with_fixture_catalog_for_test(
        mut self,
        counts: &[usize],
        content: &str,
    ) -> BoundedParallelResult<Self> {
        for (item_index, count) in counts.iter().copied().enumerate() {
            let item = self
                .work_items
                .get_mut(item_index)
                .ok_or(BoundedParallelError::InvalidCatalog)?;
            item.fixtures = (0..count)
                .map(|fixture_index| {
                    ParallelFixture::new(
                        format!("fixture{item_index}-{fixture_index}"),
                        format!("Fixture {item_index} {fixture_index}"),
                        content,
                    )
                })
                .collect::<BoundedParallelResult<Vec<_>>>()?;
        }
        Ok(self)
    }

    pub fn build_child_input(
        &self,
        ordinal: u8,
        predecessors: &[ParallelChildOutcome],
    ) -> BoundedParallelResult<String> {
        validation::build_child_input(self, ordinal, predecessors)
    }

    pub(super) fn validate_integrity(&self) -> BoundedParallelResult<()> {
        validation::validate_request(
            self.scenario_id,
            &self.objective,
            self.failure_policy,
            &self.work_items,
            self.active_limit,
        )
    }

    pub fn parse_child_result(
        &self,
        ordinal: u8,
        raw: &str,
    ) -> BoundedParallelResult<ParallelChildResult> {
        validation::parse_child_result(self, ordinal, raw)
    }

    pub fn build_synthesis_input(
        &self,
        outcomes: &[ParallelChildOutcome],
    ) -> BoundedParallelResult<String> {
        validation::build_synthesis_input(self, outcomes)
    }

    pub fn parse_synthesis(
        &self,
        outcomes: &[ParallelChildOutcome],
        raw: &str,
    ) -> BoundedParallelResult<BoundedParallelSynthesis> {
        validation::parse_synthesis(self, outcomes, raw)
    }

    pub(super) fn new(
        scenario_id: BoundedParallelScenarioId,
        objective: String,
        failure_policy: BoundedParallelFailurePolicy,
        work_items: Vec<ParallelWorkItem>,
        active_limit: u8,
    ) -> BoundedParallelResult<Self> {
        validation::validate_request(
            scenario_id,
            &objective,
            failure_policy,
            &work_items,
            active_limit,
        )?;
        Ok(Self {
            scenario_id,
            objective,
            failure_policy,
            work_items,
            active_limit,
        })
    }

    pub(super) fn item(&self, ordinal: u8) -> BoundedParallelResult<&ParallelWorkItem> {
        self.work_items
            .get(usize::from(ordinal.saturating_sub(1)))
            .filter(|item| item.ordinal == ordinal)
            .ok_or(BoundedParallelError::UnknownWorkItem)
    }
}

impl fmt::Debug for BoundedParallelWorkflowRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedParallelWorkflowRequest")
            .field("scenario_id", &self.scenario_id)
            .field("objective", &"[REDACTED]")
            .field("work_item_count", &self.work_items.len())
            .field("active_limit", &self.active_limit)
            .finish()
    }
}

pub struct BoundedParallelScenarioCatalog;

impl BoundedParallelScenarioCatalog {
    #[must_use]
    pub const fn built_in() -> Self {
        Self
    }

    pub fn request(
        &self,
        scenario_id: BoundedParallelScenarioId,
    ) -> BoundedParallelResult<BoundedParallelWorkflowRequest> {
        catalog::request(scenario_id, DEFAULT_ACTIVE_PARALLEL_CHILDREN)
    }

    /// Selects one application-owned admission width for a sealed scenario.
    /// This does not accept agent lists, dependencies, retries, tools, or a
    /// model-authored workflow definition.
    pub fn request_with_active_limit(
        &self,
        scenario_id: BoundedParallelScenarioId,
        active_limit: u8,
    ) -> BoundedParallelResult<BoundedParallelWorkflowRequest> {
        if active_limit == 0 || active_limit > HARD_MAX_ACTIVE_PARALLEL_CHILDREN {
            return Err(BoundedParallelError::ActiveLimitExceeded);
        }
        catalog::request(scenario_id, active_limit)
    }

    #[cfg(test)]
    pub(super) fn three_independent_for_test(
        &self,
        active_limit: u8,
    ) -> BoundedParallelResult<BoundedParallelWorkflowRequest> {
        catalog::three_independent_for_test(active_limit)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParallelConfidence {
    Low,
    Medium,
    High,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParallelEvidenceRef {
    namespace: String,
    id: String,
}

impl ParallelEvidenceRef {
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParallelFinding {
    id: String,
    statement: String,
    confidence: ParallelConfidence,
    references: Vec<ParallelEvidenceRef>,
}

impl ParallelFinding {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }
    #[must_use]
    pub const fn confidence(&self) -> ParallelConfidence {
        self.confidence
    }
    #[must_use]
    pub fn references(&self) -> &[ParallelEvidenceRef] {
        &self.references
    }
}

impl fmt::Debug for ParallelFinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParallelFinding")
            .field("id", &self.id)
            .field("statement", &"[REDACTED]")
            .field("confidence", &self.confidence)
            .field("references", &self.references.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParallelChildResult {
    work_item_id: ParallelWorkItemId,
    summary: String,
    findings: Vec<ParallelFinding>,
    unresolved_issues: Vec<String>,
    transfer: String,
}

impl ParallelChildResult {
    #[must_use]
    pub fn work_item_id(&self) -> &ParallelWorkItemId {
        &self.work_item_id
    }
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }
    #[must_use]
    pub fn findings(&self) -> &[ParallelFinding] {
        &self.findings
    }
    #[must_use]
    pub fn unresolved_issues(&self) -> &[String] {
        &self.unresolved_issues
    }
    pub(super) fn transfer(&self) -> &str {
        &self.transfer
    }
}

impl fmt::Debug for ParallelChildResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParallelChildResult")
            .field("work_item_id", &self.work_item_id)
            .field("summary", &"[REDACTED]")
            .field("finding_count", &self.findings.len())
            .field("unresolved_count", &self.unresolved_issues.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParallelChildResultStatus {
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    Skipped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParallelCancellationReason {
    UserRequested,
    FailFast,
    RootCancelled,
    RuntimeReported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParallelTimeoutReason {
    ChildDeadlineExceeded,
    RootDeadlineExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParallelSkipReason {
    DependencyUnavailable,
    FailFast,
    RootCancelled,
    RootDeadlineExpired,
}

#[derive(Clone, Eq, PartialEq)]
pub enum ParallelChildDisposition {
    Succeeded(ParallelChildResult),
    Failed(AgentTaskFailureCode),
    Cancelled(ParallelCancellationReason),
    TimedOut(ParallelTimeoutReason),
    Skipped(ParallelSkipReason),
}

impl ParallelChildDisposition {
    #[must_use]
    pub const fn status(&self) -> ParallelChildResultStatus {
        match self {
            Self::Succeeded(_) => ParallelChildResultStatus::Succeeded,
            Self::Failed(_) => ParallelChildResultStatus::Failed,
            Self::Cancelled(_) => ParallelChildResultStatus::Cancelled,
            Self::TimedOut(_) => ParallelChildResultStatus::TimedOut,
            Self::Skipped(_) => ParallelChildResultStatus::Skipped,
        }
    }
}

impl fmt::Debug for ParallelChildDisposition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Succeeded(_) => formatter.write_str("Succeeded([REDACTED])"),
            Self::Failed(value) => formatter.debug_tuple("Failed").field(value).finish(),
            Self::Cancelled(value) => formatter.debug_tuple("Cancelled").field(value).finish(),
            Self::TimedOut(value) => formatter.debug_tuple("TimedOut").field(value).finish(),
            Self::Skipped(value) => formatter.debug_tuple("Skipped").field(value).finish(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParallelChildOutcome {
    ordinal: u8,
    work_item_id: ParallelWorkItemId,
    agent_id: AgentId,
    disposition: ParallelChildDisposition,
}

impl ParallelChildOutcome {
    pub(super) fn new(
        ordinal: u8,
        work_item_id: ParallelWorkItemId,
        agent_id: AgentId,
        disposition: ParallelChildDisposition,
    ) -> Self {
        Self {
            ordinal,
            work_item_id,
            agent_id,
            disposition,
        }
    }
    #[must_use]
    pub const fn ordinal(&self) -> u8 {
        self.ordinal
    }
    #[must_use]
    pub fn work_item_id(&self) -> &ParallelWorkItemId {
        &self.work_item_id
    }
    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }
    #[must_use]
    pub const fn status(&self) -> ParallelChildResultStatus {
        self.disposition.status()
    }
    #[must_use]
    pub fn disposition(&self) -> &ParallelChildDisposition {
        &self.disposition
    }
}

impl fmt::Debug for ParallelChildOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParallelChildOutcome")
            .field("ordinal", &self.ordinal)
            .field("work_item_id", &self.work_item_id)
            .field("agent_id", &self.agent_id)
            .field("disposition", &self.disposition)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BoundedParallelSynthesisStatus {
    Complete,
    Partial,
}

#[derive(Clone, Eq, PartialEq)]
pub struct BoundedParallelSynthesisFindingSource {
    ordinal: u8,
    work_item_id: ParallelWorkItemId,
    agent_id: AgentId,
    finding_ids: Vec<String>,
}

impl BoundedParallelSynthesisFindingSource {
    #[must_use]
    pub const fn ordinal(&self) -> u8 {
        self.ordinal
    }
    #[must_use]
    pub fn work_item_id(&self) -> &ParallelWorkItemId {
        &self.work_item_id
    }
    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }
    #[must_use]
    pub fn finding_ids(&self) -> &[String] {
        &self.finding_ids
    }
}

impl fmt::Debug for BoundedParallelSynthesisFindingSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedParallelSynthesisFindingSource")
            .field("ordinal", &self.ordinal)
            .field("work_item_id", &self.work_item_id)
            .field("agent_id", &self.agent_id)
            .field("finding_ids", &self.finding_ids)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BoundedParallelSynthesis {
    status: BoundedParallelSynthesisStatus,
    source_findings: Vec<BoundedParallelSynthesisFindingSource>,
    summary: String,
    unresolved_issues: Vec<String>,
}

impl BoundedParallelSynthesis {
    #[must_use]
    pub const fn status(&self) -> BoundedParallelSynthesisStatus {
        self.status
    }
    #[must_use]
    pub fn source_findings(&self) -> &[BoundedParallelSynthesisFindingSource] {
        &self.source_findings
    }
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }
    #[must_use]
    pub fn unresolved_issues(&self) -> &[String] {
        &self.unresolved_issues
    }
}

impl fmt::Debug for BoundedParallelSynthesis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedParallelSynthesis")
            .field("status", &self.status)
            .field("source_finding_count", &self.source_findings.len())
            .field("summary", &"[REDACTED]")
            .field("unresolved_count", &self.unresolved_issues.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedParallelWorkflowStatus {
    Running,
    Cancelling,
    Synthesizing,
    Completed,
    Partial,
    Failed,
    Cancelled,
}

#[derive(Clone, Eq, PartialEq)]
pub struct BoundedParallelWorkflowResult {
    root_task_id: RootTaskId,
    scenario_id: BoundedParallelScenarioId,
    failure_policy: BoundedParallelFailurePolicy,
    children: Vec<ParallelChildOutcome>,
    synthesis: BoundedParallelSynthesis,
}

impl BoundedParallelWorkflowResult {
    pub(super) fn new(
        root_task_id: RootTaskId,
        scenario_id: BoundedParallelScenarioId,
        failure_policy: BoundedParallelFailurePolicy,
        children: Vec<ParallelChildOutcome>,
        synthesis: BoundedParallelSynthesis,
    ) -> Self {
        Self {
            root_task_id,
            scenario_id,
            failure_policy,
            children,
            synthesis,
        }
    }
    #[must_use]
    pub fn root_task_id(&self) -> &RootTaskId {
        &self.root_task_id
    }
    #[must_use]
    pub const fn scenario_id(&self) -> BoundedParallelScenarioId {
        self.scenario_id
    }
    #[must_use]
    pub const fn failure_policy(&self) -> BoundedParallelFailurePolicy {
        self.failure_policy
    }
    #[must_use]
    pub fn children(&self) -> &[ParallelChildOutcome] {
        &self.children
    }
    #[must_use]
    pub fn synthesis(&self) -> &BoundedParallelSynthesis {
        &self.synthesis
    }
}

impl fmt::Debug for BoundedParallelWorkflowResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedParallelWorkflowResult")
            .field("root_task_id", &self.root_task_id)
            .field("scenario_id", &self.scenario_id)
            .field("failure_policy", &self.failure_policy)
            .field("children", &self.children)
            .field("synthesis", &self.synthesis)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedParallelTerminalStatus {
    Cancelled,
    Partial,
    Failed,
    Completed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedParallelWorkflowEvent {
    Queued {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
    },
    Started {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        task_id: AgentTaskId,
    },
    Progress {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
    },
    StartAttemptFailed {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        task_id: AgentTaskId,
    },
    Completed {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        task_id: AgentTaskId,
    },
    Failed {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        code: AgentTaskFailureCode,
    },
    Cancelled {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        reason: ParallelCancellationReason,
    },
    TimedOut {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        reason: ParallelTimeoutReason,
    },
    Skipped {
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        reason: ParallelSkipReason,
    },
    DependencySatisfied {
        predecessor: ParallelWorkItemId,
        dependent: ParallelWorkItemId,
    },
    ParentResumed {
        task_id: AgentTaskId,
    },
    SynthesisStarted {
        task_id: AgentTaskId,
    },
    SynthesisProgress,
    SynthesisCompleted {
        task_id: AgentTaskId,
    },
    Terminal {
        status: BoundedParallelTerminalStatus,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedParallelAuditOutcome {
    Queued,
    Started,
    Progress,
    StartAttemptFailed,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
    Skipped,
    DependencySatisfied,
    ParentResumed,
    SynthesisStarted,
    SynthesisProgress,
    SynthesisCompleted,
    Terminal(BoundedParallelTerminalStatus),
}

#[derive(Clone, Eq, PartialEq)]
pub struct PlannedParallelAttribution {
    root_task_id: RootTaskId,
    scenario_id: BoundedParallelScenarioId,
    work_item_id: ParallelWorkItemId,
    ordinal: u8,
    expected_agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
}

#[derive(Clone, Eq, PartialEq)]
pub struct StartAttemptParallelAttribution {
    planned: PlannedParallelAttribution,
    task_id: AgentTaskId,
    runtime_id: RuntimeId,
}

#[derive(Clone, Eq, PartialEq)]
pub struct LiveParallelAttribution {
    planned: PlannedParallelAttribution,
    task_id: AgentTaskId,
    runtime_id: RuntimeId,
    _runtime_run_identity: RuntimeRunIdentity,
}

/// Content-free attribution for a Personal Assistant synthesis run that the
/// application attempted to start but for which no runtime run was returned.
///
/// The absence of `RuntimeRunIdentity` is intentional: a failed start attempt
/// must never fabricate live runtime authority.
#[derive(Clone, Eq, PartialEq)]
pub struct PersonalRootStartAttemptParallelAttribution {
    root_task_id: RootTaskId,
    scenario_id: BoundedParallelScenarioId,
    agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    task_id: AgentTaskId,
    runtime_id: RuntimeId,
}

/// Content-free attribution for the Personal Assistant root while it is
/// waiting for bounded specialist children and has no live synthesis run.
/// This is not a runtime-start attempt and carries no runtime identity.
#[derive(Clone, Eq, PartialEq)]
pub struct PersonalRootWaitingParallelAttribution {
    root_task_id: RootTaskId,
    scenario_id: BoundedParallelScenarioId,
    agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    task_id: AgentTaskId,
}

/// Content-free attribution copied from the exact live Personal Assistant
/// execution context returned after synthesis runtime start succeeds.
///
/// This snapshot is descriptive only and cannot be converted back into a live
/// context or runtime-run capability.
#[derive(Clone, Eq, PartialEq)]
pub struct PersonalRootLiveParallelAttribution {
    root_task_id: RootTaskId,
    scenario_id: BoundedParallelScenarioId,
    agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    task_id: AgentTaskId,
    runtime_id: RuntimeId,
    depth: u8,
    _runtime_run_identity: RuntimeRunIdentity,
}

#[derive(Clone, Eq, PartialEq)]
pub enum BoundedParallelAttribution {
    Planned(PlannedParallelAttribution),
    StartAttempt(StartAttemptParallelAttribution),
    Live(LiveParallelAttribution),
    PersonalRootWaiting(PersonalRootWaitingParallelAttribution),
    PersonalRootStartAttempt(PersonalRootStartAttemptParallelAttribution),
    PersonalRootLive(PersonalRootLiveParallelAttribution),
}

impl BoundedParallelAttribution {
    pub(super) fn planned(
        root_task_id: RootTaskId,
        scenario_id: BoundedParallelScenarioId,
        item: &ParallelWorkItem,
        policy_profile_id: AgentPolicyProfileId,
        memory_profile_id: AgentMemoryProfileId,
    ) -> Self {
        Self::Planned(PlannedParallelAttribution {
            root_task_id,
            scenario_id,
            work_item_id: item.id.clone(),
            ordinal: item.ordinal,
            expected_agent_id: item.agent_id,
            policy_profile_id,
            memory_profile_id,
        })
    }

    pub(super) fn start_attempt(
        root_task_id: RootTaskId,
        scenario_id: BoundedParallelScenarioId,
        item: &ParallelWorkItem,
        policy_profile_id: AgentPolicyProfileId,
        memory_profile_id: AgentMemoryProfileId,
        task_id: AgentTaskId,
        runtime_id: RuntimeId,
    ) -> Self {
        Self::StartAttempt(StartAttemptParallelAttribution {
            planned: PlannedParallelAttribution {
                root_task_id,
                scenario_id,
                work_item_id: item.id.clone(),
                ordinal: item.ordinal,
                expected_agent_id: item.agent_id,
                policy_profile_id,
                memory_profile_id,
            },
            task_id,
            runtime_id,
        })
    }

    pub(super) fn live(
        root_task_id: RootTaskId,
        scenario_id: BoundedParallelScenarioId,
        item: &ParallelWorkItem,
        policy_profile_id: AgentPolicyProfileId,
        memory_profile_id: AgentMemoryProfileId,
        context: &AgentExecutionContext,
    ) -> Self {
        Self::Live(LiveParallelAttribution {
            planned: PlannedParallelAttribution {
                root_task_id,
                scenario_id,
                work_item_id: item.id.clone(),
                ordinal: item.ordinal,
                expected_agent_id: item.agent_id,
                policy_profile_id,
                memory_profile_id,
            },
            task_id: context.task_id().clone(),
            runtime_id: context.runtime_id(),
            _runtime_run_identity: context.runtime_run_identity().clone(),
        })
    }

    /// Records a Personal Assistant synthesis start attempt without claiming a
    /// runtime-run identity that does not exist.
    pub(super) fn personal_root_start_attempt(
        root_task_id: RootTaskId,
        scenario_id: BoundedParallelScenarioId,
        policy_profile_id: AgentPolicyProfileId,
        memory_profile_id: AgentMemoryProfileId,
        task_id: AgentTaskId,
        runtime_id: RuntimeId,
    ) -> BoundedParallelResult<Self> {
        if root_task_id.task_id() != &task_id
            || policy_profile_id != AgentPolicyProfileId::PersonalAssistantV1
            || memory_profile_id != AgentMemoryProfileId::PersonalAssistantMemoryV1
        {
            return Err(BoundedParallelError::AttributionMismatch);
        }
        Ok(Self::PersonalRootStartAttempt(
            PersonalRootStartAttemptParallelAttribution {
                root_task_id,
                scenario_id,
                agent_id: AgentId::PersonalAssistant,
                policy_profile_id,
                memory_profile_id,
                task_id,
                runtime_id,
            },
        ))
    }

    pub(super) fn personal_root_waiting(
        root_task_id: RootTaskId,
        scenario_id: BoundedParallelScenarioId,
        policy_profile_id: AgentPolicyProfileId,
        memory_profile_id: AgentMemoryProfileId,
        task_id: AgentTaskId,
    ) -> BoundedParallelResult<Self> {
        if root_task_id.task_id() != &task_id
            || policy_profile_id != AgentPolicyProfileId::PersonalAssistantV1
            || memory_profile_id != AgentMemoryProfileId::PersonalAssistantMemoryV1
        {
            return Err(BoundedParallelError::AttributionMismatch);
        }
        Ok(Self::PersonalRootWaiting(
            PersonalRootWaitingParallelAttribution {
                root_task_id,
                scenario_id,
                agent_id: AgentId::PersonalAssistant,
                policy_profile_id,
                memory_profile_id,
                task_id,
            },
        ))
    }

    /// Copies attribution from the exact successful Personal Assistant
    /// synthesis context. Supplying a child, foreign root, or non-root context
    /// fails closed.
    pub(super) fn personal_root_live(
        scenario_id: BoundedParallelScenarioId,
        context: &AgentExecutionContext,
    ) -> BoundedParallelResult<Self> {
        if context.agent_id() != AgentId::PersonalAssistant
            || context.policy_profile_id() != AgentPolicyProfileId::PersonalAssistantV1
            || context.memory_profile_id() != AgentMemoryProfileId::PersonalAssistantMemoryV1
            || context.parent_task_id().is_some()
            || context.depth() != 0
            || context.root_task_id().task_id() != context.task_id()
        {
            return Err(BoundedParallelError::AttributionMismatch);
        }
        Ok(Self::PersonalRootLive(
            PersonalRootLiveParallelAttribution {
                root_task_id: context.root_task_id().clone(),
                scenario_id,
                agent_id: context.agent_id(),
                policy_profile_id: context.policy_profile_id(),
                memory_profile_id: context.memory_profile_id(),
                task_id: context.task_id().clone(),
                runtime_id: context.runtime_id(),
                depth: context.depth(),
                _runtime_run_identity: context.runtime_run_identity().clone(),
            },
        ))
    }

    /// Returns the application-owned root identity shared by every
    /// attribution variant.
    #[must_use]
    pub fn root_task_id(&self) -> &RootTaskId {
        match self {
            Self::Planned(value) => &value.root_task_id,
            Self::StartAttempt(value) => &value.planned.root_task_id,
            Self::Live(value) => &value.planned.root_task_id,
            Self::PersonalRootWaiting(value) => &value.root_task_id,
            Self::PersonalRootStartAttempt(value) => &value.root_task_id,
            Self::PersonalRootLive(value) => &value.root_task_id,
        }
    }

    /// Returns the sealed application scenario associated with the record.
    #[must_use]
    pub const fn scenario_id(&self) -> BoundedParallelScenarioId {
        match self {
            Self::Planned(value) => value.scenario_id,
            Self::StartAttempt(value) => value.planned.scenario_id,
            Self::Live(value) => value.planned.scenario_id,
            Self::PersonalRootWaiting(value) => value.scenario_id,
            Self::PersonalRootStartAttempt(value) => value.scenario_id,
            Self::PersonalRootLive(value) => value.scenario_id,
        }
    }

    /// Returns a work-item identity only for specialist-slot attribution.
    /// Personal root synthesis is deliberately not represented as a child
    /// work item.
    #[must_use]
    pub fn work_item_id(&self) -> Option<&ParallelWorkItemId> {
        match self {
            Self::Planned(value) => Some(&value.work_item_id),
            Self::StartAttempt(value) => Some(&value.planned.work_item_id),
            Self::Live(value) => Some(&value.planned.work_item_id),
            Self::PersonalRootWaiting(_)
            | Self::PersonalRootStartAttempt(_)
            | Self::PersonalRootLive(_) => None,
        }
    }

    /// Returns a catalog ordinal only for specialist-slot attribution.
    #[must_use]
    pub const fn ordinal(&self) -> Option<u8> {
        match self {
            Self::Planned(value) => Some(value.ordinal),
            Self::StartAttempt(value) => Some(value.planned.ordinal),
            Self::Live(value) => Some(value.planned.ordinal),
            Self::PersonalRootWaiting(_)
            | Self::PersonalRootStartAttempt(_)
            | Self::PersonalRootLive(_) => None,
        }
    }

    /// Returns the application-derived agent identity. For a planned child it
    /// is the sealed expected agent; for Personal synthesis it is always the
    /// Personal Assistant.
    #[must_use]
    pub const fn expected_agent_id(&self) -> AgentId {
        match self {
            Self::Planned(value) => value.expected_agent_id,
            Self::StartAttempt(value) => value.planned.expected_agent_id,
            Self::Live(value) => value.planned.expected_agent_id,
            Self::PersonalRootWaiting(value) => value.agent_id,
            Self::PersonalRootStartAttempt(value) => value.agent_id,
            Self::PersonalRootLive(value) => value.agent_id,
        }
    }

    #[must_use]
    pub const fn policy_profile_id(&self) -> AgentPolicyProfileId {
        match self {
            Self::Planned(value) => value.policy_profile_id,
            Self::StartAttempt(value) => value.planned.policy_profile_id,
            Self::Live(value) => value.planned.policy_profile_id,
            Self::PersonalRootWaiting(value) => value.policy_profile_id,
            Self::PersonalRootStartAttempt(value) => value.policy_profile_id,
            Self::PersonalRootLive(value) => value.policy_profile_id,
        }
    }

    #[must_use]
    pub const fn memory_profile_id(&self) -> AgentMemoryProfileId {
        match self {
            Self::Planned(value) => value.memory_profile_id,
            Self::StartAttempt(value) => value.planned.memory_profile_id,
            Self::Live(value) => value.planned.memory_profile_id,
            Self::PersonalRootWaiting(value) => value.memory_profile_id,
            Self::PersonalRootStartAttempt(value) => value.memory_profile_id,
            Self::PersonalRootLive(value) => value.memory_profile_id,
        }
    }

    /// Returns a task identity only after admission allocated the task.
    #[must_use]
    pub fn task_id(&self) -> Option<&AgentTaskId> {
        match self {
            Self::Planned(_) => None,
            Self::StartAttempt(value) => Some(&value.task_id),
            Self::Live(value) => Some(&value.task_id),
            Self::PersonalRootWaiting(value) => Some(&value.task_id),
            Self::PersonalRootStartAttempt(value) => Some(&value.task_id),
            Self::PersonalRootLive(value) => Some(&value.task_id),
        }
    }

    /// Returns the attempted or live runtime adapter identity. This never
    /// exposes the private runtime-run binding.
    #[must_use]
    pub const fn runtime_id(&self) -> Option<RuntimeId> {
        match self {
            Self::Planned(_) => None,
            Self::StartAttempt(value) => Some(value.runtime_id),
            Self::Live(value) => Some(value.runtime_id),
            Self::PersonalRootWaiting(_) => None,
            Self::PersonalRootStartAttempt(value) => Some(value.runtime_id),
            Self::PersonalRootLive(value) => Some(value.runtime_id),
        }
    }

    /// Reports only whether an exact live run binding exists. The binding
    /// value itself is intentionally not exposed.
    #[must_use]
    pub const fn has_live_run_binding(&self) -> bool {
        matches!(self, Self::Live(_) | Self::PersonalRootLive(_))
    }

    #[must_use]
    pub const fn is_live(&self) -> bool {
        self.has_live_run_binding()
    }
}

impl fmt::Debug for BoundedParallelAttribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Planned(value) => formatter
                .debug_struct("PlannedParallelAttribution")
                .field("scenario_id", &value.scenario_id)
                .field("work_item_id", &value.work_item_id)
                .field("ordinal", &value.ordinal)
                .field("expected_agent_id", &value.expected_agent_id)
                .finish(),
            Self::StartAttempt(value) => formatter
                .debug_struct("StartAttemptParallelAttribution")
                .field("planned", &value.planned.work_item_id)
                .field("task_id", &value.task_id)
                .field("runtime_id", &value.runtime_id)
                .field("runtime_run_identity", &"none")
                .finish(),
            Self::Live(value) => formatter
                .debug_struct("LiveParallelAttribution")
                .field("planned", &value.planned.work_item_id)
                .field("task_id", &value.task_id)
                .field("runtime_id", &value.runtime_id)
                .field("runtime_run_identity", &"[REDACTED]")
                .finish(),
            Self::PersonalRootWaiting(value) => formatter
                .debug_struct("PersonalRootWaitingParallelAttribution")
                .field("scenario_id", &value.scenario_id)
                .field("agent_id", &value.agent_id)
                .field("policy_profile_id", &value.policy_profile_id)
                .field("memory_profile_id", &value.memory_profile_id)
                .field("task_id", &value.task_id)
                .field("root_task_id", &value.root_task_id)
                .field("runtime_run_identity", &"none")
                .finish(),
            Self::PersonalRootStartAttempt(value) => formatter
                .debug_struct("PersonalRootStartAttemptParallelAttribution")
                .field("scenario_id", &value.scenario_id)
                .field("agent_id", &value.agent_id)
                .field("policy_profile_id", &value.policy_profile_id)
                .field("memory_profile_id", &value.memory_profile_id)
                .field("task_id", &value.task_id)
                .field("root_task_id", &value.root_task_id)
                .field("runtime_id", &value.runtime_id)
                .field("runtime_run_identity", &"none")
                .finish(),
            Self::PersonalRootLive(value) => formatter
                .debug_struct("PersonalRootLiveParallelAttribution")
                .field("scenario_id", &value.scenario_id)
                .field("agent_id", &value.agent_id)
                .field("policy_profile_id", &value.policy_profile_id)
                .field("memory_profile_id", &value.memory_profile_id)
                .field("task_id", &value.task_id)
                .field("root_task_id", &value.root_task_id)
                .field("runtime_id", &value.runtime_id)
                .field("depth", &value.depth)
                .field("runtime_run_identity", &"[REDACTED]")
                .finish(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BoundedParallelAuditRecord {
    sequence: u8,
    attribution: BoundedParallelAttribution,
    outcome: BoundedParallelAuditOutcome,
}

impl BoundedParallelAuditRecord {
    pub(super) fn new(
        sequence: u8,
        attribution: BoundedParallelAttribution,
        outcome: BoundedParallelAuditOutcome,
    ) -> Self {
        Self {
            sequence,
            attribution,
            outcome,
        }
    }
    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
    #[must_use]
    pub fn attribution(&self) -> &BoundedParallelAttribution {
        &self.attribution
    }
    #[must_use]
    pub const fn outcome(&self) -> BoundedParallelAuditOutcome {
        self.outcome
    }
}

impl fmt::Debug for BoundedParallelAuditRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedParallelAuditRecord")
            .field("sequence", &self.sequence)
            .field("attribution", &self.attribution)
            .field("outcome", &self.outcome)
            .finish()
    }
}

pub struct ParallelChildCancellationHandle {
    workflow_sequence: u64,
    root_task_id: RootTaskId,
    task_id: AgentTaskId,
    work_item_id: ParallelWorkItemId,
    runtime_run_identity: RuntimeRunIdentity,
}

impl ParallelChildCancellationHandle {
    pub(super) fn new(
        workflow_sequence: u64,
        root_task_id: RootTaskId,
        task_id: AgentTaskId,
        work_item_id: ParallelWorkItemId,
        runtime_run_identity: RuntimeRunIdentity,
    ) -> Self {
        Self {
            workflow_sequence,
            root_task_id,
            task_id,
            work_item_id,
            runtime_run_identity,
        }
    }
    pub(super) fn parts(
        &self,
    ) -> (
        u64,
        &RootTaskId,
        &AgentTaskId,
        &ParallelWorkItemId,
        &RuntimeRunIdentity,
    ) {
        (
            self.workflow_sequence,
            &self.root_task_id,
            &self.task_id,
            &self.work_item_id,
            &self.runtime_run_identity,
        )
    }
}

impl fmt::Debug for ParallelChildCancellationHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParallelChildCancellationHandle")
            .field("workflow_sequence", &"[REDACTED]")
            .field("root_task_id", &self.root_task_id)
            .field("task_id", &self.task_id)
            .field("work_item_id", &self.work_item_id)
            .field("runtime_run_identity", &"[REDACTED]")
            .finish()
    }
}

pub struct BoundedParallelChildControl {
    work_item_id: ParallelWorkItemId,
    ordinal: u8,
    context: AgentExecutionContext,
    handle: ParallelChildCancellationHandle,
}

impl BoundedParallelChildControl {
    pub(super) fn new(
        work_item_id: ParallelWorkItemId,
        ordinal: u8,
        context: AgentExecutionContext,
        handle: ParallelChildCancellationHandle,
    ) -> Self {
        Self {
            work_item_id,
            ordinal,
            context,
            handle,
        }
    }
    #[must_use]
    pub fn work_item_id(&self) -> &ParallelWorkItemId {
        &self.work_item_id
    }
    #[must_use]
    pub const fn ordinal(&self) -> u8 {
        self.ordinal
    }
    #[must_use]
    pub fn context(&self) -> &AgentExecutionContext {
        &self.context
    }
    pub fn into_cancellation_handle(self) -> ParallelChildCancellationHandle {
        self.handle
    }
}

impl fmt::Debug for BoundedParallelChildControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedParallelChildControl")
            .field("work_item_id", &self.work_item_id)
            .field("ordinal", &self.ordinal)
            .field("context", &self.context)
            .field("handle", &self.handle)
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedParallelWorkflowAcceptance {
    active_contexts: Vec<AgentExecutionContext>,
}

impl BoundedParallelWorkflowAcceptance {
    pub(super) fn new(active_contexts: Vec<AgentExecutionContext>) -> Self {
        Self { active_contexts }
    }
    #[must_use]
    pub fn active_contexts(&self) -> &[AgentExecutionContext] {
        &self.active_contexts
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum BoundedParallelError {
    #[error("the bounded-parallel work-item identifier is invalid")]
    InvalidWorkItemId,
    #[error("the bounded-parallel scenario catalog is invalid")]
    InvalidCatalog,
    #[error("the bounded-parallel objective or fixture text is invalid")]
    InvalidText,
    #[error("the bounded-parallel content exceeds a closed bound")]
    BoundExceeded,
    #[error("the bounded-parallel active-child limit is invalid")]
    ActiveLimitExceeded,
    #[error("the bounded-parallel work item is unknown")]
    UnknownWorkItem,
    #[error("the bounded-parallel dependency is unavailable")]
    DependencyUnavailable,
    #[error("the bounded-parallel result is not one strict JSON object")]
    InvalidStructuredResult,
    #[error("the bounded-parallel result identifies the wrong scenario or work item")]
    ResultIdentityMismatch,
    #[error("the bounded-parallel result contains an unknown fixture reference")]
    UnknownFixtureReference,
    #[error("the bounded-parallel result contains a duplicate identifier or reference")]
    DuplicateReference,
    #[error("the bounded-parallel result makes a prohibited authority or execution claim")]
    AuthorityClaim,
    #[error("the bounded-parallel ordered transfer exceeds its bound")]
    TransferTooLarge,
    #[error("the bounded-parallel synthesis status table is inconsistent")]
    SynthesisStatusMismatch,
    #[error("the bounded-parallel synthesis disclosure is inconsistent")]
    SynthesisDisclosureMismatch,
    #[error("the bounded-parallel workflow state is missing")]
    WorkflowMissing,
    #[error("the bounded-parallel workflow stage is inconsistent")]
    StageMismatch,
    #[error("the bounded-parallel workflow journal reached its closed bound")]
    JournalLimitExceeded,
    #[error("the bounded-parallel child cancellation handle is stale or foreign")]
    CancellationHandleMismatch,
    #[error("the bounded-parallel attribution does not match the exact task authority")]
    AttributionMismatch,
    #[error("bounded-parallel cancellation propagation did not finish")]
    CancellationIncomplete,
    #[error("the bounded-parallel workflow is closed while cancellation is pending")]
    CancellationPending,
    #[error("the bounded-parallel workflow deadline is exhausted")]
    DeadlineExceeded,
    #[error("bounded-parallel serialization failed")]
    SerializationFailed,
}

pub(super) fn deadline_after(now: Instant, seconds: u64) -> BoundedParallelResult<Instant> {
    now.checked_add(std::time::Duration::from_secs(seconds))
        .ok_or(BoundedParallelError::DeadlineExceeded)
}

#[cfg(test)]
mod attribution_tests {
    use super::*;
    use crate::agent::{
        definition::AgentDefinition,
        runtime::RuntimeTurnRequest,
        task::{AgentTask, AgentTaskObjective},
    };

    fn personal_context() -> Result<AgentExecutionContext, Box<dyn std::error::Error>> {
        let definition = AgentDefinition::built_in(AgentId::PersonalAssistant)?;
        let mut task = AgentTask::new_root(
            AgentTaskId::new("parallel-personal-root")?,
            definition.identity(),
            AgentTaskObjective::new("Synthesize bounded fixture results")?,
        );
        task.start()?;
        let identity = RuntimeTurnRequest::new(
            "parallel-personal-run",
            "parallel-personal-request",
            "fixture-only synthesis",
        )?
        .identity();
        Ok(AgentExecutionContext::for_task(
            &task,
            RuntimeId::Native,
            identity,
        ))
    }

    #[test]
    fn personal_root_start_attempt_has_no_runtime_run_identity(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task_id = AgentTaskId::new("parallel-personal-root")?;
        let attribution = BoundedParallelAttribution::personal_root_start_attempt(
            RootTaskId::from_task_id(task_id.clone()),
            BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
            AgentPolicyProfileId::PersonalAssistantV1,
            AgentMemoryProfileId::PersonalAssistantMemoryV1,
            task_id,
            RuntimeId::Native,
        )?;

        assert!(!attribution.is_live());
        let BoundedParallelAttribution::PersonalRootStartAttempt(value) = &attribution else {
            return Err("expected Personal root start-attempt attribution".into());
        };
        assert_eq!(value.agent_id, AgentId::PersonalAssistant);
        assert_eq!(value.root_task_id.task_id(), &value.task_id);
        assert_eq!(value.runtime_id, RuntimeId::Native);
        assert_eq!(
            format!("{attribution:?}"),
            "PersonalRootStartAttemptParallelAttribution { scenario_id: ResearchKnowledgeIndependentV1, agent_id: PersonalAssistant, policy_profile_id: PersonalAssistantV1, memory_profile_id: PersonalAssistantMemoryV1, task_id: AgentTaskId(\"[REDACTED]\"), root_task_id: RootTaskId(\"[REDACTED]\"), runtime_id: Native, runtime_run_identity: \"none\" }"
        );
        Ok(())
    }

    #[test]
    fn personal_root_start_attempt_rejects_foreign_identity(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = AgentTaskId::new("parallel-personal-root")?;
        let foreign = AgentTaskId::new("foreign-task")?;
        assert_eq!(
            BoundedParallelAttribution::personal_root_start_attempt(
                RootTaskId::from_task_id(root),
                BoundedParallelScenarioId::CodeSecurityQaV1,
                AgentPolicyProfileId::PersonalAssistantV1,
                AgentMemoryProfileId::PersonalAssistantMemoryV1,
                foreign,
                RuntimeId::Native,
            ),
            Err(BoundedParallelError::AttributionMismatch)
        );
        Ok(())
    }

    #[test]
    fn personal_root_live_is_derived_from_exact_context_and_redacts_run_identity(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let context = personal_context()?;
        let attribution = BoundedParallelAttribution::personal_root_live(
            BoundedParallelScenarioId::CloudSystemsSecurityV1,
            &context,
        )?;

        assert!(attribution.is_live());
        let BoundedParallelAttribution::PersonalRootLive(value) = &attribution else {
            return Err("expected live Personal root attribution".into());
        };
        assert_eq!(value.agent_id, AgentId::PersonalAssistant);
        assert_eq!(value.root_task_id.task_id(), &value.task_id);
        assert_eq!(value.depth, 0);
        let debug = format!("{attribution:?}");
        assert!(debug.contains("runtime_run_identity: \"[REDACTED]\""));
        assert!(!debug.contains("parallel-personal-run"));
        assert!(!debug.contains("parallel-personal-request"));
        Ok(())
    }

    #[test]
    fn content_free_getters_cover_every_attribution_variant_without_exposing_run_identity(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root_task_id = RootTaskId::from_task_id(AgentTaskId::new("parallel-root")?);
        let child_task_id = AgentTaskId::new("parallel-child")?;
        let item_id = ParallelWorkItemId::new("research-analysis")?;
        let planned_value = PlannedParallelAttribution {
            root_task_id: root_task_id.clone(),
            scenario_id: BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
            work_item_id: item_id.clone(),
            ordinal: 1,
            expected_agent_id: AgentId::Research,
            policy_profile_id: AgentPolicyProfileId::ResearchReadOnlyV1,
            memory_profile_id: AgentMemoryProfileId::ResearchWorkingMemoryV1,
        };
        let run_identity = RuntimeTurnRequest::new(
            "parallel-child-secret-run",
            "parallel-child-secret-request",
            "fixture-only child",
        )?
        .identity();
        let child_attributions = [
            BoundedParallelAttribution::Planned(planned_value.clone()),
            BoundedParallelAttribution::StartAttempt(StartAttemptParallelAttribution {
                planned: planned_value.clone(),
                task_id: child_task_id.clone(),
                runtime_id: RuntimeId::Native,
            }),
            BoundedParallelAttribution::Live(LiveParallelAttribution {
                planned: planned_value,
                task_id: child_task_id.clone(),
                runtime_id: RuntimeId::Native,
                _runtime_run_identity: run_identity,
            }),
        ];

        for (index, attribution) in child_attributions.iter().enumerate() {
            assert_eq!(attribution.root_task_id(), &root_task_id);
            assert_eq!(
                attribution.scenario_id(),
                BoundedParallelScenarioId::ResearchKnowledgeIndependentV1
            );
            assert_eq!(attribution.work_item_id(), Some(&item_id));
            assert_eq!(attribution.ordinal(), Some(1));
            assert_eq!(attribution.expected_agent_id(), AgentId::Research);
            assert_eq!(
                attribution.policy_profile_id(),
                AgentPolicyProfileId::ResearchReadOnlyV1
            );
            assert_eq!(
                attribution.memory_profile_id(),
                AgentMemoryProfileId::ResearchWorkingMemoryV1
            );
            assert_eq!(attribution.task_id(), (index > 0).then_some(&child_task_id));
            assert_eq!(
                attribution.runtime_id(),
                (index > 0).then_some(RuntimeId::Native)
            );
            assert_eq!(attribution.has_live_run_binding(), index == 2);
            let debug = format!("{attribution:?}");
            assert!(!debug.contains("parallel-child-secret-run"));
            assert!(!debug.contains("parallel-child-secret-request"));
        }

        let personal_task_id = AgentTaskId::new("parallel-personal-root")?;
        let personal_attempt = BoundedParallelAttribution::personal_root_start_attempt(
            RootTaskId::from_task_id(personal_task_id.clone()),
            BoundedParallelScenarioId::CodeSecurityQaV1,
            AgentPolicyProfileId::PersonalAssistantV1,
            AgentMemoryProfileId::PersonalAssistantMemoryV1,
            personal_task_id.clone(),
            RuntimeId::Native,
        )?;
        let personal_live = BoundedParallelAttribution::personal_root_live(
            BoundedParallelScenarioId::CloudSystemsSecurityV1,
            &personal_context()?,
        )?;

        for attribution in [&personal_attempt, &personal_live] {
            assert!(attribution.work_item_id().is_none());
            assert_eq!(attribution.ordinal(), None);
            assert_eq!(attribution.expected_agent_id(), AgentId::PersonalAssistant);
            assert_eq!(
                attribution.policy_profile_id(),
                AgentPolicyProfileId::PersonalAssistantV1
            );
            assert_eq!(
                attribution.memory_profile_id(),
                AgentMemoryProfileId::PersonalAssistantMemoryV1
            );
            assert_eq!(attribution.runtime_id(), Some(RuntimeId::Native));
        }
        assert_eq!(personal_attempt.task_id(), Some(&personal_task_id));
        assert!(!personal_attempt.has_live_run_binding());
        assert!(personal_live.has_live_run_binding());
        Ok(())
    }
}
