//! Volatile, manually stepped Research/Knowledge demo lifecycle.
//!
//! This module owns one application-selected synthetic D-086 workflow at a
//! time. It is intentionally unwired to Tauri and React and performs no
//! provider, model, network, tool, approval, persistence, filesystem, timer,
//! thread, background, or device work.

use std::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
};

use serde::Serialize;
use thiserror::Error;

use crate::agent::{
    definition::AgentId,
    native_runtime::NativeAgentRuntime,
    orchestrator::{
        AgentOrchestrator, AgentOrchestratorError, ResearchKnowledgeWorkflowAcceptance,
    },
    research_knowledge::{
        FinalSynthesisStatus, ResearchKnowledgeWorkflowRequest, WorkflowSource,
        WorkflowSourceCatalog,
    },
    runtime::{
        AgentRuntime, RuntimeEventEnvelope, RuntimeOutputText, RuntimeResponseId,
        UntrustedRuntimeEvent,
    },
    task::{AgentExecutionContext, AgentTaskCancellationOutcome, AgentTaskId, AgentTaskStatus},
};

#[cfg(test)]
use crate::agent::runtime::{RuntimeFailure, RuntimeFailureCode};

const DISCLOSURE: &str = "DEMO MODE · SIMULATED AGENT DATA";
const FIXTURE_PROVENANCE: &str = "application-owned-synthetic-fixture";
const MAX_JOURNAL_ENTRIES: usize = 8;
const OBJECTIVE: &str = "Compare two technical approaches and create a structured decision brief";
const PROOF_BOUNDARY: &str = "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.";
const SCENARIO_ID: &str = "research-knowledge-demo-v1";
const SCHEMA_VERSION: &str = "research-knowledge-demo-lifecycle-v1";

static NATIVE_DROP_QUARANTINE_ACTIVE: AtomicBool = AtomicBool::new(false);

const RESEARCH_JSON: &str = r#"{"findings":[{"statement":"Approach A is simpler and less costly","source_ids":["approach-a"],"confidence":"high"},{"statement":"Approach B has higher scale potential","source_ids":["approach-b"],"confidence":"medium"}],"unresolved_questions":["What scale is required?"],"limitations":["Only deterministic fixture evidence was supplied"],"recommended_follow_up":"Review against approved operational requirements"}"#;
const KNOWLEDGE_JSON: &str = r#"{"sections":[{"heading":"Decision factors","body":"A favors simplicity while B favors scale.","source_ids":["approach-a","approach-b"]}],"extracted_facts":[{"statement":"A has lower operating cost.","source_ids":["approach-a"]}],"contradictions":[],"summary":"Choose according to required scale and operations capacity.","reusable_knowledge_proposal":"Retain this comparison only after explicit application review.","artifact_outline":"Context; evidence; trade-offs; decision","incomplete":false}"#;
const FINAL_SYNTHESIS: &str = r#"{"version":"v1","answer":"Fixture-based decision brief: A is simpler [approach-a]; B may scale further [approach-b]. Verify before acting.","source_ids":["approach-a","approach-b"],"fixture_based":true,"status":"complete"}"#;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResearchKnowledgeDemoLifecycleState {
    Idle,
    Research,
    Knowledge,
    Synthesis,
    Succeeded,
    Failed,
    Cancelled,
    CleanupPending,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResearchKnowledgeDemoLifecycleEventKind {
    ResearchStarted,
    ResearchCompleted,
    KnowledgeStarted,
    KnowledgeCompleted,
    SynthesisStarted,
    Completed,
    Failed,
    Cancelled,
    CleanupPending,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchKnowledgeDemoLifecycleEntry {
    revision: u8,
    kind: ResearchKnowledgeDemoLifecycleEventKind,
}

impl ResearchKnowledgeDemoLifecycleEntry {
    #[must_use]
    pub const fn revision(&self) -> u8 {
        self.revision
    }

    #[must_use]
    pub const fn kind(&self) -> ResearchKnowledgeDemoLifecycleEventKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchKnowledgeDemoLifecycleSnapshot {
    schema_version: &'static str,
    scenario_id: &'static str,
    disclosure: &'static str,
    proof_boundary: &'static str,
    fixture_provenance: &'static str,
    presentation_epoch: u32,
    revision: u8,
    state: ResearchKnowledgeDemoLifecycleState,
    journal: Vec<ResearchKnowledgeDemoLifecycleEntry>,
}

impl ResearchKnowledgeDemoLifecycleSnapshot {
    fn idle() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            scenario_id: SCENARIO_ID,
            disclosure: DISCLOSURE,
            proof_boundary: PROOF_BOUNDARY,
            fixture_provenance: FIXTURE_PROVENANCE,
            presentation_epoch: 0,
            revision: 0,
            state: ResearchKnowledgeDemoLifecycleState::Idle,
            journal: Vec::with_capacity(MAX_JOURNAL_ENTRIES),
        }
    }

    #[must_use]
    pub const fn schema_version(&self) -> &'static str {
        self.schema_version
    }

    #[must_use]
    pub const fn scenario_id(&self) -> &'static str {
        self.scenario_id
    }

    #[must_use]
    pub const fn disclosure(&self) -> &'static str {
        self.disclosure
    }

    #[must_use]
    pub const fn proof_boundary(&self) -> &'static str {
        self.proof_boundary
    }

    #[must_use]
    pub const fn fixture_provenance(&self) -> &'static str {
        self.fixture_provenance
    }

    #[must_use]
    pub const fn presentation_epoch(&self) -> u32 {
        self.presentation_epoch
    }

    #[must_use]
    pub const fn revision(&self) -> u8 {
        self.revision
    }

    #[must_use]
    pub const fn state(&self) -> ResearchKnowledgeDemoLifecycleState {
        self.state
    }

    #[must_use]
    pub fn journal(&self) -> &[ResearchKnowledgeDemoLifecycleEntry] {
        &self.journal
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchKnowledgeDemoLifecycleTransition {
    snapshot: ResearchKnowledgeDemoLifecycleSnapshot,
    entries: Vec<ResearchKnowledgeDemoLifecycleEntry>,
}

impl ResearchKnowledgeDemoLifecycleTransition {
    #[must_use]
    pub const fn snapshot(&self) -> &ResearchKnowledgeDemoLifecycleSnapshot {
        &self.snapshot
    }

    #[must_use]
    pub fn entries(&self) -> &[ResearchKnowledgeDemoLifecycleEntry] {
        &self.entries
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResearchKnowledgeDemoLifecycleError {
    #[error("Research/Knowledge demo is already active.")]
    AlreadyActive,
    #[error("Research/Knowledge demo is not active.")]
    NotActive,
    #[error("Research/Knowledge demo transition is unavailable.")]
    InvalidTransition,
    #[error("Research/Knowledge demo cleanup is pending.")]
    CleanupPending,
    #[error("Research/Knowledge demo is unavailable.")]
    Unavailable,
}

impl ResearchKnowledgeDemoLifecycleError {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::AlreadyActive => "already-active",
            Self::NotActive => "not-active",
            Self::InvalidTransition => "invalid-transition",
            Self::CleanupPending => "cleanup-pending",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Clone, Copy)]
enum DemoScript {
    Success,
    #[cfg(test)]
    SynthesisFailure,
}

struct OwnedWorkflow<R: AgentRuntime> {
    orchestrator: AgentOrchestrator<R>,
    root_task_id: Option<AgentTaskId>,
}

enum WorkflowOwnership<R: AgentRuntime> {
    Active(OwnedWorkflow<R>),
    CleanupPending(OwnedWorkflow<R>),
}

struct DemoHostCore<R: AgentRuntime + Clone> {
    runtime: R,
    script: DemoScript,
    ownership: Option<WorkflowOwnership<R>>,
    snapshot: ResearchKnowledgeDemoLifecycleSnapshot,
}

impl<R: AgentRuntime + Clone> DemoHostCore<R> {
    fn new(runtime: R, script: DemoScript) -> Self {
        Self {
            runtime,
            script,
            ownership: None,
            snapshot: ResearchKnowledgeDemoLifecycleSnapshot::idle(),
        }
    }

    fn start(
        &mut self,
    ) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
        match self.ownership {
            Some(WorkflowOwnership::Active(_)) => {
                return Err(ResearchKnowledgeDemoLifecycleError::AlreadyActive)
            }
            Some(WorkflowOwnership::CleanupPending(_)) => {
                return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
            }
            None => {}
        }

        let next_epoch = self
            .snapshot
            .presentation_epoch
            .checked_add(1)
            .ok_or(ResearchKnowledgeDemoLifecycleError::Unavailable)?;
        let mut orchestrator = AgentOrchestrator::new(self.runtime.clone())
            .map_err(|_| ResearchKnowledgeDemoLifecycleError::Unavailable)?;
        let root = match orchestrator.start_root(OBJECTIVE) {
            Ok(root) => root,
            Err(AgentOrchestratorError::RuntimeCleanupPending) => {
                self.reset_snapshot(next_epoch);
                let owned = OwnedWorkflow {
                    orchestrator,
                    root_task_id: None,
                };
                self.ownership = Some(WorkflowOwnership::CleanupPending(owned));
                self.commit_cleanup_pending()?;
                return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
            }
            Err(_) => {
                self.reset_snapshot(next_epoch);
                self.commit_transition(
                    ResearchKnowledgeDemoLifecycleState::Failed,
                    &[ResearchKnowledgeDemoLifecycleEventKind::Failed],
                )?;
                return Err(ResearchKnowledgeDemoLifecycleError::Unavailable);
            }
        };

        let root_task_id = root.task_id().clone();
        let mut owned = OwnedWorkflow {
            orchestrator,
            root_task_id: Some(root_task_id),
        };
        let acceptance = build_workflow_request().and_then(|request| {
            owned
                .orchestrator
                .request_research_knowledge_workflow(&root, request)
        });

        match acceptance {
            Ok(ResearchKnowledgeWorkflowAcceptance::ResearchStarted { context })
                if context.agent_id() == AgentId::Research =>
            {
                self.reset_snapshot(next_epoch);
                self.ownership = Some(WorkflowOwnership::Active(owned));
                self.commit_transition(
                    ResearchKnowledgeDemoLifecycleState::Research,
                    &[ResearchKnowledgeDemoLifecycleEventKind::ResearchStarted],
                )?;
                Ok(self.snapshot.clone())
            }
            Err(AgentOrchestratorError::RuntimeCleanupPending) => {
                self.reset_snapshot(next_epoch);
                self.ownership = Some(WorkflowOwnership::CleanupPending(owned));
                self.commit_cleanup_pending()?;
                Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
            }
            Ok(_) | Err(_) => {
                self.reset_snapshot(next_epoch);
                self.contain_unavailable(owned)
            }
        }
    }

    fn advance(
        &mut self,
    ) -> Result<ResearchKnowledgeDemoLifecycleTransition, ResearchKnowledgeDemoLifecycleError> {
        let ownership = self
            .ownership
            .take()
            .ok_or(ResearchKnowledgeDemoLifecycleError::InvalidTransition)?;
        let mut owned = match ownership {
            WorkflowOwnership::Active(owned) => owned,
            WorkflowOwnership::CleanupPending(owned) => {
                self.ownership = Some(WorkflowOwnership::CleanupPending(owned));
                return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
            }
        };
        let state = self.snapshot.state;
        let additional_entries = match state {
            ResearchKnowledgeDemoLifecycleState::Research
            | ResearchKnowledgeDemoLifecycleState::Knowledge => 2,
            ResearchKnowledgeDemoLifecycleState::Synthesis => 1,
            _ => {
                self.ownership = Some(WorkflowOwnership::Active(owned));
                return Err(ResearchKnowledgeDemoLifecycleError::InvalidTransition);
            }
        };
        if self.ensure_journal_capacity(additional_entries).is_err() {
            return self.contain_unavailable(owned);
        }

        let progress = match state {
            ResearchKnowledgeDemoLifecycleState::Research => {
                advance_research(&mut owned.orchestrator)
            }
            ResearchKnowledgeDemoLifecycleState::Knowledge => {
                advance_knowledge(&mut owned.orchestrator)
            }
            ResearchKnowledgeDemoLifecycleState::Synthesis => {
                let root_task_id = owned.root_task_id.clone();
                match root_task_id.as_ref() {
                    Some(root_task_id) => {
                        advance_synthesis(&mut owned.orchestrator, root_task_id, self.script)
                    }
                    None => Err(AgentOrchestratorError::RootMissing),
                }
            }
            _ => {
                self.ownership = Some(WorkflowOwnership::Active(owned));
                return Err(ResearchKnowledgeDemoLifecycleError::InvalidTransition);
            }
        };

        let DemoProgress {
            state: next_state,
            entries,
            terminal,
        } = match progress {
            Ok(progress) => progress,
            Err(_) => return self.contain_unavailable(owned),
        };
        let transition = match self.commit_transition(next_state, entries) {
            Ok(transition) => transition,
            Err(_) => return self.contain_unavailable(owned),
        };
        if terminal {
            self.ownership = None;
        } else {
            self.ownership = Some(WorkflowOwnership::Active(owned));
        }
        Ok(transition)
    }

    fn cancel(
        &mut self,
    ) -> Result<ResearchKnowledgeDemoLifecycleTransition, ResearchKnowledgeDemoLifecycleError> {
        let Some(ownership) = self.ownership.take() else {
            if self.snapshot.state == ResearchKnowledgeDemoLifecycleState::Cancelled {
                return Ok(ResearchKnowledgeDemoLifecycleTransition {
                    snapshot: self.snapshot.clone(),
                    entries: Vec::new(),
                });
            }
            return Err(ResearchKnowledgeDemoLifecycleError::NotActive);
        };
        let mut owned = match ownership {
            WorkflowOwnership::Active(owned) | WorkflowOwnership::CleanupPending(owned) => owned,
        };

        if self.ensure_journal_capacity(1).is_err() {
            self.ownership = Some(WorkflowOwnership::CleanupPending(owned));
            return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
        }
        if retry_owned_cleanup(&mut owned).is_err() {
            self.ownership = Some(WorkflowOwnership::CleanupPending(owned));
            self.commit_cleanup_pending()?;
            return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
        }

        self.commit_transition(
            ResearchKnowledgeDemoLifecycleState::Cancelled,
            &[ResearchKnowledgeDemoLifecycleEventKind::Cancelled],
        )
    }

    fn snapshot(&self) -> ResearchKnowledgeDemoLifecycleSnapshot {
        self.snapshot.clone()
    }

    fn reset_snapshot(&mut self, presentation_epoch: u32) {
        self.snapshot.presentation_epoch = presentation_epoch;
        self.snapshot.revision = 0;
        self.snapshot.state = ResearchKnowledgeDemoLifecycleState::Idle;
        self.snapshot.journal.clear();
    }

    fn ensure_journal_capacity(
        &self,
        additional: usize,
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let next_len = self
            .snapshot
            .journal
            .len()
            .checked_add(additional)
            .ok_or(ResearchKnowledgeDemoLifecycleError::Unavailable)?;
        let additional_revision = u8::try_from(additional)
            .map_err(|_| ResearchKnowledgeDemoLifecycleError::Unavailable)?;
        self.snapshot
            .revision
            .checked_add(additional_revision)
            .ok_or(ResearchKnowledgeDemoLifecycleError::Unavailable)?;
        if next_len > MAX_JOURNAL_ENTRIES {
            return Err(ResearchKnowledgeDemoLifecycleError::Unavailable);
        }
        Ok(())
    }

    fn commit_transition(
        &mut self,
        state: ResearchKnowledgeDemoLifecycleState,
        kinds: &[ResearchKnowledgeDemoLifecycleEventKind],
    ) -> Result<ResearchKnowledgeDemoLifecycleTransition, ResearchKnowledgeDemoLifecycleError> {
        self.ensure_journal_capacity(kinds.len())?;
        let mut entries = Vec::with_capacity(kinds.len());
        for kind in kinds {
            self.snapshot.revision = self
                .snapshot
                .revision
                .checked_add(1)
                .ok_or(ResearchKnowledgeDemoLifecycleError::Unavailable)?;
            let entry = ResearchKnowledgeDemoLifecycleEntry {
                revision: self.snapshot.revision,
                kind: *kind,
            };
            self.snapshot.journal.push(entry);
            entries.push(entry);
        }
        self.snapshot.state = state;
        Ok(ResearchKnowledgeDemoLifecycleTransition {
            snapshot: self.snapshot.clone(),
            entries,
        })
    }

    fn commit_cleanup_pending(
        &mut self,
    ) -> Result<ResearchKnowledgeDemoLifecycleTransition, ResearchKnowledgeDemoLifecycleError> {
        if self.snapshot.state == ResearchKnowledgeDemoLifecycleState::CleanupPending {
            return Ok(ResearchKnowledgeDemoLifecycleTransition {
                snapshot: self.snapshot.clone(),
                entries: Vec::new(),
            });
        }
        self.commit_transition(
            ResearchKnowledgeDemoLifecycleState::CleanupPending,
            &[ResearchKnowledgeDemoLifecycleEventKind::CleanupPending],
        )
    }

    fn contain_unavailable<T>(
        &mut self,
        mut owned: OwnedWorkflow<R>,
    ) -> Result<T, ResearchKnowledgeDemoLifecycleError> {
        if retry_owned_cleanup(&mut owned).is_err() {
            self.ownership = Some(WorkflowOwnership::CleanupPending(owned));
            match self.commit_cleanup_pending() {
                Ok(_) | Err(_) => {}
            }
            return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
        }
        self.ownership = None;
        match self.commit_transition(
            ResearchKnowledgeDemoLifecycleState::Failed,
            &[ResearchKnowledgeDemoLifecycleEventKind::Failed],
        ) {
            Ok(_) | Err(_) => {}
        }
        Err(ResearchKnowledgeDemoLifecycleError::Unavailable)
    }

    fn shutdown(&mut self) {
        let Some(ownership) = self.ownership.take() else {
            return;
        };
        let mut owned = match ownership {
            WorkflowOwnership::Active(owned) | WorkflowOwnership::CleanupPending(owned) => owned,
        };
        if retry_owned_cleanup(&mut owned).is_err() {
            // Destruction cannot report a retryable error. Keep the rejected or
            // nonterminal run owned until process exit instead of discarding it,
            // and prevent any replacement host from starting beside it.
            NATIVE_DROP_QUARANTINE_ACTIVE.store(true, Ordering::Release);
            std::mem::forget(owned);
        }
    }
}

impl<R: AgentRuntime + Clone> Drop for DemoHostCore<R> {
    fn drop(&mut self) {
        self.shutdown();
    }
}

struct DemoProgress {
    state: ResearchKnowledgeDemoLifecycleState,
    entries: &'static [ResearchKnowledgeDemoLifecycleEventKind],
    terminal: bool,
}

fn build_workflow_request() -> Result<ResearchKnowledgeWorkflowRequest, AgentOrchestratorError> {
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

fn advance_research<R: AgentRuntime>(
    orchestrator: &mut AgentOrchestrator<R>,
) -> Result<DemoProgress, AgentOrchestratorError> {
    let context = active_child_context(orchestrator, AgentId::Research)?;
    complete_stage(
        orchestrator,
        &context,
        "demo-research-response",
        RESEARCH_JSON,
    )?;
    let knowledge = active_child_context(orchestrator, AgentId::KnowledgeDocument)?;
    if knowledge.parent_task_id() != context.parent_task_id() {
        return Err(AgentOrchestratorError::ContextMismatch);
    }
    Ok(DemoProgress {
        state: ResearchKnowledgeDemoLifecycleState::Knowledge,
        entries: &[
            ResearchKnowledgeDemoLifecycleEventKind::ResearchCompleted,
            ResearchKnowledgeDemoLifecycleEventKind::KnowledgeStarted,
        ],
        terminal: false,
    })
}

fn advance_knowledge<R: AgentRuntime>(
    orchestrator: &mut AgentOrchestrator<R>,
) -> Result<DemoProgress, AgentOrchestratorError> {
    let context = active_child_context(orchestrator, AgentId::KnowledgeDocument)?;
    let root_task_id = context.root_task_id().task_id().clone();
    complete_stage(
        orchestrator,
        &context,
        "demo-knowledge-response",
        KNOWLEDGE_JSON,
    )?;
    let synthesis = orchestrator.current_context(&root_task_id)?;
    if synthesis.agent_id() != AgentId::PersonalAssistant {
        return Err(AgentOrchestratorError::ContextMismatch);
    }
    Ok(DemoProgress {
        state: ResearchKnowledgeDemoLifecycleState::Synthesis,
        entries: &[
            ResearchKnowledgeDemoLifecycleEventKind::KnowledgeCompleted,
            ResearchKnowledgeDemoLifecycleEventKind::SynthesisStarted,
        ],
        terminal: false,
    })
}

fn advance_synthesis<R: AgentRuntime>(
    orchestrator: &mut AgentOrchestrator<R>,
    root_task_id: &AgentTaskId,
    script: DemoScript,
) -> Result<DemoProgress, AgentOrchestratorError> {
    let context = orchestrator.current_context(root_task_id)?;
    if context.agent_id() != AgentId::PersonalAssistant {
        return Err(AgentOrchestratorError::ContextMismatch);
    }
    match script {
        DemoScript::Success => {
            complete_stage(
                orchestrator,
                &context,
                "demo-synthesis-response",
                FINAL_SYNTHESIS,
            )?;
            let result = orchestrator
                .research_knowledge_result()
                .ok_or(AgentOrchestratorError::ResearchKnowledgeWorkflowMissing)?;
            if !result.fixture_based()
                || result.synthesis().status() != FinalSynthesisStatus::Complete
            {
                return Err(AgentOrchestratorError::ResearchKnowledgeStageMismatch);
            }
            Ok(DemoProgress {
                state: ResearchKnowledgeDemoLifecycleState::Succeeded,
                entries: &[ResearchKnowledgeDemoLifecycleEventKind::Completed],
                terminal: true,
            })
        }
        #[cfg(test)]
        DemoScript::SynthesisFailure => {
            fail_stage(orchestrator, &context, "demo-synthesis-failure")?;
            if orchestrator.research_knowledge_result().is_some() {
                return Err(AgentOrchestratorError::ResearchKnowledgeStageMismatch);
            }
            Ok(DemoProgress {
                state: ResearchKnowledgeDemoLifecycleState::Failed,
                entries: &[ResearchKnowledgeDemoLifecycleEventKind::Failed],
                terminal: true,
            })
        }
    }
}

fn active_child_context<R: AgentRuntime>(
    orchestrator: &AgentOrchestrator<R>,
    expected_agent: AgentId,
) -> Result<AgentExecutionContext, AgentOrchestratorError> {
    let task = orchestrator
        .active_child_task()
        .ok_or(AgentOrchestratorError::TaskNotFound)?;
    if task.agent_id() != expected_agent {
        return Err(AgentOrchestratorError::ContextMismatch);
    }
    orchestrator.current_context(task.id())
}

fn complete_stage<R: AgentRuntime>(
    orchestrator: &mut AgentOrchestrator<R>,
    context: &AgentExecutionContext,
    response_id: &str,
    output: &str,
) -> Result<(), AgentOrchestratorError> {
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

#[cfg(test)]
fn fail_stage<R: AgentRuntime>(
    orchestrator: &mut AgentOrchestrator<R>,
    context: &AgentExecutionContext,
    response_id: &str,
) -> Result<(), AgentOrchestratorError> {
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
            UntrustedRuntimeEvent::ResponseFailed {
                failure: RuntimeFailure::new(RuntimeFailureCode::ProviderUnavailable, false, None)?,
            },
        ),
    )?;
    Ok(())
}

fn retry_owned_cleanup<R: AgentRuntime>(
    owned: &mut OwnedWorkflow<R>,
) -> Result<(), AgentOrchestratorError> {
    if let Some(root_task_id) = owned.root_task_id.as_ref() {
        let root_is_live = owned
            .orchestrator
            .task(root_task_id)
            .is_some_and(|task| !task.status().is_terminal());
        if root_is_live {
            match owned.orchestrator.cancel_task(root_task_id)? {
                AgentTaskCancellationOutcome::Cancelled
                | AgentTaskCancellationOutcome::AlreadyTerminal(
                    AgentTaskStatus::Completed
                    | AgentTaskStatus::Failed
                    | AgentTaskStatus::Cancelled,
                ) => {}
                AgentTaskCancellationOutcome::AlreadyTerminal(
                    AgentTaskStatus::Pending
                    | AgentTaskStatus::Running
                    | AgentTaskStatus::WaitingForChild,
                ) => return Err(AgentOrchestratorError::RuntimeCleanupPending),
            }
        }
    }
    owned.orchestrator.retry_rejected_runtime_cleanup()
}

pub struct ResearchKnowledgeDemoHost {
    core: DemoHostCore<NativeAgentRuntime>,
}

impl ResearchKnowledgeDemoHost {
    #[must_use]
    pub fn new() -> Self {
        Self {
            core: DemoHostCore::new(NativeAgentRuntime, DemoScript::Success),
        }
    }

    pub fn start(
        &mut self,
    ) -> Result<ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleError> {
        if NATIVE_DROP_QUARANTINE_ACTIVE.load(Ordering::Acquire) {
            return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
        }
        self.core.start()
    }

    pub fn advance(
        &mut self,
    ) -> Result<ResearchKnowledgeDemoLifecycleTransition, ResearchKnowledgeDemoLifecycleError> {
        if NATIVE_DROP_QUARANTINE_ACTIVE.load(Ordering::Acquire) {
            return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
        }
        self.core.advance()
    }

    pub fn cancel(
        &mut self,
    ) -> Result<ResearchKnowledgeDemoLifecycleTransition, ResearchKnowledgeDemoLifecycleError> {
        if NATIVE_DROP_QUARANTINE_ACTIVE.load(Ordering::Acquire) {
            return Err(ResearchKnowledgeDemoLifecycleError::CleanupPending);
        }
        self.core.cancel()
    }

    #[must_use]
    pub fn snapshot(&self) -> ResearchKnowledgeDemoLifecycleSnapshot {
        self.core.snapshot()
    }
}

impl Default for ResearchKnowledgeDemoHost {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ResearchKnowledgeDemoHost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResearchKnowledgeDemoHost")
            .field("presentation_epoch", &self.core.snapshot.presentation_epoch)
            .field("revision", &self.core.snapshot.revision)
            .field("state", &self.core.snapshot.state)
            .field("journal_count", &self.core.snapshot.journal.len())
            .field("owns_workflow", &self.core.ownership.is_some())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        rc::Rc,
        sync::{Mutex, MutexGuard},
    };

    use super::*;
    use crate::agent::{
        native_runtime::NativeAgentRun,
        runtime::{
            RuntimeBoundaryStage, RuntimeCancellationOutcome, RuntimeDescriptor, RuntimeError,
            RuntimeEventAcceptance, RuntimeRun, RuntimeRunId, RuntimeRunIdentity, RuntimeRunStatus,
            RuntimeTurnRequest,
        },
    };

    static TEST_LIFECYCLE_LOCK: Mutex<()> = Mutex::new(());

    fn lifecycle_test_guard() -> MutexGuard<'static, ()> {
        match TEST_LIFECYCLE_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    struct DropQuarantineReset;

    impl Drop for DropQuarantineReset {
        fn drop(&mut self) {
            NATIVE_DROP_QUARANTINE_ACTIVE.store(false, Ordering::Release);
        }
    }

    #[derive(Clone, Copy)]
    enum FaultMode {
        MismatchAndTransientCancel(u8),
        PersistentCancel(u8),
        OneShotCancellation(u8),
    }

    #[derive(Default)]
    struct FaultState {
        starts: u8,
        live_runs: usize,
        nonterminal_drops: usize,
        one_shot_cancel_failed: bool,
    }

    #[derive(Clone)]
    struct FaultRuntime {
        mode: FaultMode,
        state: Rc<RefCell<FaultState>>,
    }

    impl FaultRuntime {
        fn recording(mode: FaultMode) -> (Self, Rc<RefCell<FaultState>>) {
            let state = Rc::new(RefCell::new(FaultState::default()));
            (
                Self {
                    mode,
                    state: Rc::clone(&state),
                },
                state,
            )
        }
    }

    impl AgentRuntime for FaultRuntime {
        type Run = FaultRun;

        fn describe(&self) -> RuntimeDescriptor {
            NativeAgentRuntime.describe()
        }

        fn start(&self, request: RuntimeTurnRequest) -> Result<Self::Run, RuntimeError> {
            let start_ordinal = {
                let mut state = self.state.borrow_mut();
                state.starts = state
                    .starts
                    .checked_add(1)
                    .ok_or(RuntimeError::BoundaryFailure(RuntimeBoundaryStage::Start))?;
                state.starts
            };
            let inner = NativeAgentRuntime.start(request)?;
            let identity = match self.mode {
                FaultMode::MismatchAndTransientCancel(ordinal) if ordinal == start_ordinal => {
                    RuntimeTurnRequest::new(
                        format!("demo-foreign-run-{start_ordinal}"),
                        format!("demo-foreign-request-{start_ordinal}"),
                        "application-owned synthetic identity mismatch",
                    )?
                    .identity()
                }
                FaultMode::MismatchAndTransientCancel(_)
                | FaultMode::PersistentCancel(_)
                | FaultMode::OneShotCancellation(_) => inner.identity().clone(),
            };
            self.state.borrow_mut().live_runs += 1;
            Ok(FaultRun {
                inner,
                identity,
                mode: self.mode,
                start_ordinal,
                state: Rc::clone(&self.state),
                tracked_live: true,
            })
        }
    }

    struct FaultRun {
        inner: NativeAgentRun,
        identity: RuntimeRunIdentity,
        mode: FaultMode,
        start_ordinal: u8,
        state: Rc<RefCell<FaultState>>,
        tracked_live: bool,
    }

    impl FaultRun {
        fn record_terminal(&mut self) {
            if self.tracked_live && self.inner.status().is_terminal() {
                self.state.borrow_mut().live_runs -= 1;
                self.tracked_live = false;
            }
        }
    }

    impl Drop for FaultRun {
        fn drop(&mut self) {
            if self.tracked_live {
                let mut state = self.state.borrow_mut();
                state.live_runs -= 1;
                state.nonterminal_drops += 1;
                self.tracked_live = false;
            }
        }
    }

    impl RuntimeRun for FaultRun {
        fn run_id(&self) -> &RuntimeRunId {
            self.identity.run_id()
        }

        fn identity(&self) -> &RuntimeRunIdentity {
            &self.identity
        }

        fn status(&self) -> RuntimeRunStatus {
            self.inner.status()
        }

        fn accept_event(
            &mut self,
            event: RuntimeEventEnvelope,
        ) -> Result<RuntimeEventAcceptance, RuntimeError> {
            let result = self.inner.accept_event(event);
            self.record_terminal();
            result
        }

        fn cancel(&mut self) -> Result<RuntimeCancellationOutcome, RuntimeError> {
            if matches!(
                self.mode,
                FaultMode::PersistentCancel(ordinal) if ordinal == self.start_ordinal
            ) && !self.inner.status().is_terminal()
            {
                return Err(RuntimeError::BoundaryFailure(
                    RuntimeBoundaryStage::Cancellation,
                ));
            }
            let should_fail_once = matches!(
                self.mode,
                FaultMode::MismatchAndTransientCancel(ordinal)
                    | FaultMode::OneShotCancellation(ordinal)
                    if ordinal == self.start_ordinal
            ) && !self.inner.status().is_terminal()
                && !self.state.borrow().one_shot_cancel_failed;
            if should_fail_once {
                self.state.borrow_mut().one_shot_cancel_failed = true;
                return Err(RuntimeError::BoundaryFailure(
                    RuntimeBoundaryStage::Cancellation,
                ));
            }

            let result = self.inner.cancel();
            self.record_terminal();
            result
        }
    }

    #[test]
    fn native_host_completes_the_exact_manual_fixture_sequence(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let _guard = lifecycle_test_guard();
        let mut host = ResearchKnowledgeDemoHost::new();
        let started = host.start()?;
        assert_eq!(started.presentation_epoch(), 1);
        assert_eq!(
            started.state(),
            ResearchKnowledgeDemoLifecycleState::Research
        );

        let knowledge = host.advance()?;
        assert_eq!(
            knowledge.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::Knowledge
        );
        assert_eq!(knowledge.entries().len(), 2);

        let synthesis = host.advance()?;
        assert_eq!(
            synthesis.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::Synthesis
        );
        assert_eq!(synthesis.entries().len(), 2);

        let completed = host.advance()?;
        assert_eq!(
            completed.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::Succeeded
        );
        assert_eq!(completed.snapshot().journal().len(), 6);
        assert_eq!(
            completed
                .snapshot()
                .journal()
                .iter()
                .map(ResearchKnowledgeDemoLifecycleEntry::kind)
                .collect::<Vec<_>>(),
            [
                ResearchKnowledgeDemoLifecycleEventKind::ResearchStarted,
                ResearchKnowledgeDemoLifecycleEventKind::ResearchCompleted,
                ResearchKnowledgeDemoLifecycleEventKind::KnowledgeStarted,
                ResearchKnowledgeDemoLifecycleEventKind::KnowledgeCompleted,
                ResearchKnowledgeDemoLifecycleEventKind::SynthesisStarted,
                ResearchKnowledgeDemoLifecycleEventKind::Completed,
            ]
        );
        Ok(())
    }

    #[test]
    fn private_failure_script_terminalizes_without_a_result_or_retry(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let _guard = lifecycle_test_guard();
        let mut core = DemoHostCore::new(NativeAgentRuntime, DemoScript::SynthesisFailure);
        core.start()?;
        core.advance()?;
        core.advance()?;
        let failed = core.advance()?;

        assert_eq!(
            failed.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::Failed
        );
        assert_eq!(failed.entries().len(), 1);
        assert_eq!(
            failed.entries()[0].kind(),
            ResearchKnowledgeDemoLifecycleEventKind::Failed
        );
        assert!(core.ownership.is_none());
        Ok(())
    }

    #[test]
    fn cancellation_is_child_first_terminal_and_idempotent_at_each_stage(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let _guard = lifecycle_test_guard();
        for advances in 0..=2 {
            let mut host = ResearchKnowledgeDemoHost::new();
            host.start()?;
            for _ in 0..advances {
                host.advance()?;
            }
            let cancelled = host.cancel()?;
            assert_eq!(
                cancelled.snapshot().state(),
                ResearchKnowledgeDemoLifecycleState::Cancelled
            );
            let repeated = host.cancel()?;
            assert_eq!(repeated.snapshot(), cancelled.snapshot());
            assert!(repeated.entries().is_empty());
            assert_eq!(
                host.advance(),
                Err(ResearchKnowledgeDemoLifecycleError::InvalidTransition)
            );
        }
        Ok(())
    }

    #[test]
    fn start_is_single_owner_and_terminal_restart_uses_a_fresh_epoch(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let _guard = lifecycle_test_guard();
        let mut host = ResearchKnowledgeDemoHost::new();
        host.start()?;
        assert_eq!(
            host.start(),
            Err(ResearchKnowledgeDemoLifecycleError::AlreadyActive)
        );
        host.cancel()?;
        let restarted = host.start()?;
        assert_eq!(restarted.presentation_epoch(), 2);
        assert_eq!(restarted.revision(), 1);
        assert_eq!(restarted.journal().len(), 1);
        Ok(())
    }

    #[test]
    fn snapshot_and_errors_are_closed_bounded_and_content_free(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _guard = lifecycle_test_guard();
        let mut host = ResearchKnowledgeDemoHost::new();
        let snapshot = host.start()?;
        assert_eq!(snapshot.schema_version(), SCHEMA_VERSION);
        assert_eq!(snapshot.scenario_id(), SCENARIO_ID);
        assert_eq!(snapshot.disclosure(), DISCLOSURE);
        assert_eq!(snapshot.proof_boundary(), PROOF_BOUNDARY);
        assert_eq!(snapshot.fixture_provenance(), FIXTURE_PROVENANCE);
        assert!(snapshot.journal().len() <= MAX_JOURNAL_ENTRIES);

        let serialized = serde_json::to_string(&snapshot)?;
        let debug = format!("{host:?}");
        for sentinel in [
            OBJECTIVE,
            "Approach A fixture",
            "lower operating cost",
            "agent-task-",
            "-run-",
            "-request-",
            "Fixture-based decision brief",
        ] {
            assert!(!serialized.contains(sentinel));
            assert!(!debug.contains(sentinel));
        }
        for error in [
            ResearchKnowledgeDemoLifecycleError::AlreadyActive,
            ResearchKnowledgeDemoLifecycleError::NotActive,
            ResearchKnowledgeDemoLifecycleError::InvalidTransition,
            ResearchKnowledgeDemoLifecycleError::CleanupPending,
            ResearchKnowledgeDemoLifecycleError::Unavailable,
        ] {
            assert_eq!(
                serde_json::to_string(&error)?,
                format!("\"{}\"", error.code())
            );
        }
        Ok(())
    }

    #[test]
    fn epoch_and_journal_bounds_fail_closed_without_mutation() {
        let _guard = lifecycle_test_guard();
        let mut core = DemoHostCore::new(NativeAgentRuntime, DemoScript::Success);
        core.snapshot.presentation_epoch = u32::MAX;
        let before = core.snapshot();
        assert_eq!(
            core.start(),
            Err(ResearchKnowledgeDemoLifecycleError::Unavailable)
        );
        assert_eq!(core.snapshot(), before);

        const MAX_JOURNAL_REVISION: u8 = 8;
        assert_eq!(usize::from(MAX_JOURNAL_REVISION), MAX_JOURNAL_ENTRIES);
        core.snapshot.journal = (1..=MAX_JOURNAL_REVISION)
            .map(|revision| ResearchKnowledgeDemoLifecycleEntry {
                revision,
                kind: ResearchKnowledgeDemoLifecycleEventKind::ResearchStarted,
            })
            .collect();
        core.snapshot.revision = MAX_JOURNAL_REVISION;
        assert_eq!(
            core.ensure_journal_capacity(1),
            Err(ResearchKnowledgeDemoLifecycleError::Unavailable)
        );
    }

    #[test]
    fn returned_identity_mismatch_is_quarantined_until_explicit_cleanup(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let _guard = lifecycle_test_guard();
        let (runtime, state) = FaultRuntime::recording(FaultMode::MismatchAndTransientCancel(2));
        let mut core = DemoHostCore::new(runtime, DemoScript::Success);

        assert_eq!(
            core.start(),
            Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
        );
        assert_eq!(
            core.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::CleanupPending
        );
        assert_eq!(state.borrow().starts, 2);
        assert_eq!(state.borrow().live_runs, 1);
        assert_eq!(state.borrow().nonterminal_drops, 0);
        assert_eq!(
            core.start(),
            Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
        );

        let cancelled = core.cancel()?;
        assert_eq!(
            cancelled.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::Cancelled
        );
        assert_eq!(state.borrow().live_runs, 0);
        assert_eq!(state.borrow().nonterminal_drops, 0);
        assert!(core.ownership.is_none());
        Ok(())
    }

    #[test]
    fn cancellation_failure_retains_ownership_until_retry_succeeds(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let _guard = lifecycle_test_guard();
        let (runtime, state) = FaultRuntime::recording(FaultMode::OneShotCancellation(2));
        let mut core = DemoHostCore::new(runtime, DemoScript::Success);
        core.start()?;

        assert_eq!(
            core.cancel(),
            Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
        );
        assert_eq!(
            core.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::CleanupPending
        );
        assert_eq!(state.borrow().live_runs, 1);
        assert_eq!(state.borrow().nonterminal_drops, 0);

        let cancelled = core.cancel()?;
        assert_eq!(
            cancelled.snapshot().state(),
            ResearchKnowledgeDemoLifecycleState::Cancelled
        );
        assert_eq!(state.borrow().live_runs, 0);
        assert_eq!(state.borrow().nonterminal_drops, 0);
        assert!(core.ownership.is_none());
        Ok(())
    }

    #[test]
    fn drop_never_discards_a_run_when_cleanup_persistently_fails(
    ) -> Result<(), ResearchKnowledgeDemoLifecycleError> {
        let _guard = lifecycle_test_guard();
        let _reset = DropQuarantineReset;
        let (runtime, state) = FaultRuntime::recording(FaultMode::PersistentCancel(2));
        {
            let mut core = DemoHostCore::new(runtime, DemoScript::Success);
            core.start()?;
            assert_eq!(state.borrow().live_runs, 1);
        }

        assert_eq!(state.borrow().live_runs, 1);
        assert_eq!(state.borrow().nonterminal_drops, 0);
        let mut replacement = ResearchKnowledgeDemoHost::new();
        assert_eq!(
            replacement.start(),
            Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
        );
        assert_eq!(
            replacement.advance(),
            Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
        );
        assert_eq!(
            replacement.cancel(),
            Err(ResearchKnowledgeDemoLifecycleError::CleanupPending)
        );
        Ok(())
    }
}
