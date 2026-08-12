//! Bounded workflow-local volatile agent memory.
//!
//! This module deliberately owns no persistence, I/O, clock, runtime, tool,
//! policy, approval, or execution authority. Every agent operation requires a
//! sealed grant minted from live orchestrator attribution. Cross-agent context
//! assembly is limited to explicitly selected approved-shared records.

use std::{collections::BTreeMap, fmt};

use thiserror::Error;

use crate::agent::{
    definition::AgentId,
    governance::AgentAttribution,
    orchestrator::{ApplicationMemoryControlProof, LiveMemoryAccessProof},
    task::{AgentTaskId, RootTaskId},
};

pub const MAX_MEMORY_RECORDS: usize = 64;
pub const MAX_SHARED_MEMORY_PROPOSALS: usize = 16;
pub const MAX_MEMORY_RECORD_BYTES: usize = 8_192;
pub const MAX_MEMORY_RETAINED_BYTES: usize = 131_072;
pub const MAX_MEMORY_CONTEXT_RECORDS: usize = 8;
pub const MAX_MEMORY_CONTEXT_BYTES: usize = 8_192;

pub type MemoryStoreResult<T> = Result<T, MemoryStoreError>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AgentMemoryProfileId {
    PersonalAssistantMemoryV1,
    ResearchWorkingMemoryV1,
    KnowledgeWorkingMemoryV1,
    MemoryDisabledV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryNamespace {
    ApprovedShared,
    AgentPrivate,
    TaskTemporary,
    ProposedShared,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryWriteTarget {
    AgentPrivate,
    TaskTemporary,
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoryRecordId(String);

impl MemoryRecordId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for MemoryRecordId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("MemoryRecordId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SharedMemoryProposalId(String);

impl SharedMemoryProposalId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SharedMemoryProposalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("SharedMemoryProposalId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemoryRecordVersion(u64);

impl MemoryRecordVersion {
    pub const INITIAL: Self = Self(1);

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    fn next(self) -> MemoryStoreResult<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(MemoryStoreError::IdentityExhausted)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct MemoryContent(String);

impl MemoryContent {
    pub fn new(value: impl Into<String>) -> MemoryStoreResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(MemoryStoreError::EmptyContent);
        }
        if value.trim() != value {
            return Err(MemoryStoreError::NonCanonicalContent);
        }
        if value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
        {
            return Err(MemoryStoreError::ContentContainsControlCharacter);
        }
        if value.len() > MAX_MEMORY_RECORD_BYTES {
            return Err(MemoryStoreError::ContentTooLarge {
                maximum_bytes: MAX_MEMORY_RECORD_BYTES,
                actual_bytes: value.len(),
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn byte_count(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Debug for MemoryContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryContent")
            .field("content", &"[REDACTED]")
            .field("bytes", &self.byte_count())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct MemoryRecordView {
    id: MemoryRecordId,
    namespace: MemoryNamespace,
    owner_agent_id: AgentId,
    owner_task_id: Option<AgentTaskId>,
    version: MemoryRecordVersion,
    content: MemoryContent,
}

impl MemoryRecordView {
    #[must_use]
    pub fn id(&self) -> &MemoryRecordId {
        &self.id
    }

    #[must_use]
    pub const fn namespace(&self) -> MemoryNamespace {
        self.namespace
    }

    #[must_use]
    pub const fn owner_agent_id(&self) -> AgentId {
        self.owner_agent_id
    }

    #[must_use]
    pub fn owner_task_id(&self) -> Option<&AgentTaskId> {
        self.owner_task_id.as_ref()
    }

    #[must_use]
    pub const fn version(&self) -> MemoryRecordVersion {
        self.version
    }

    #[must_use]
    pub fn content(&self) -> &MemoryContent {
        &self.content
    }
}

impl fmt::Debug for MemoryRecordView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryRecordView")
            .field("id", &self.id)
            .field("namespace", &self.namespace)
            .field("owner_agent_id", &self.owner_agent_id)
            .field("owner_task_id", &self.owner_task_id)
            .field("version", &self.version)
            .field("content", &"[REDACTED]")
            .field("content_bytes", &self.content.byte_count())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SharedMemoryProposalView {
    id: SharedMemoryProposalId,
    proposer_agent_id: AgentId,
    proposer_task_id: AgentTaskId,
    root_task_id: RootTaskId,
    version: MemoryRecordVersion,
    content: MemoryContent,
}

impl SharedMemoryProposalView {
    #[must_use]
    pub const fn namespace(&self) -> MemoryNamespace {
        MemoryNamespace::ProposedShared
    }

    #[must_use]
    pub fn id(&self) -> &SharedMemoryProposalId {
        &self.id
    }

    #[must_use]
    pub const fn proposer_agent_id(&self) -> AgentId {
        self.proposer_agent_id
    }

    #[must_use]
    pub fn proposer_task_id(&self) -> &AgentTaskId {
        &self.proposer_task_id
    }

    #[must_use]
    pub fn root_task_id(&self) -> &RootTaskId {
        &self.root_task_id
    }

    #[must_use]
    pub const fn version(&self) -> MemoryRecordVersion {
        self.version
    }

    #[must_use]
    pub fn content(&self) -> &MemoryContent {
        &self.content
    }
}

impl fmt::Debug for SharedMemoryProposalView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SharedMemoryProposalView")
            .field("id", &self.id)
            .field("proposer_agent_id", &self.proposer_agent_id)
            .field("proposer_task_id", &self.proposer_task_id)
            .field("root_task_id", &self.root_task_id)
            .field("version", &self.version)
            .field("content", &"[REDACTED]")
            .field("content_bytes", &self.content.byte_count())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct MemoryContextSelection(Vec<MemoryRecordId>);

impl MemoryContextSelection {
    pub fn new(record_ids: impl IntoIterator<Item = MemoryRecordId>) -> MemoryStoreResult<Self> {
        let record_ids: Vec<_> = record_ids
            .into_iter()
            .take(MAX_MEMORY_CONTEXT_RECORDS + 1)
            .collect();
        if record_ids.len() > MAX_MEMORY_CONTEXT_RECORDS {
            return Err(MemoryStoreError::ContextRecordLimitExceeded);
        }
        let mut ordered = record_ids.clone();
        ordered.sort();
        ordered.dedup();
        if ordered.len() != record_ids.len() {
            return Err(MemoryStoreError::DuplicateContextRecord);
        }
        Ok(Self(record_ids))
    }

    #[must_use]
    pub fn record_ids(&self) -> &[MemoryRecordId] {
        &self.0
    }
}

impl fmt::Debug for MemoryContextSelection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryContextSelection")
            .field("record_count", &self.0.len())
            .field("record_ids", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct MemoryContextBundle {
    records: Vec<MemoryRecordView>,
    text: String,
}

impl MemoryContextBundle {
    #[must_use]
    pub fn records(&self) -> &[MemoryRecordView] {
        &self.records
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn byte_count(&self) -> usize {
        self.text.len()
    }
}

impl fmt::Debug for MemoryContextBundle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryContextBundle")
            .field("record_count", &self.records.len())
            .field("text", &"[REDACTED]")
            .field("text_bytes", &self.text.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum SharedMemoryReviewDecision {
    Edit(MemoryContent),
    Approve,
    ApproveEdited(MemoryContent),
    Reject,
}

impl fmt::Debug for SharedMemoryReviewDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Edit(content) => formatter
                .debug_struct("Edit")
                .field("content", &"[REDACTED]")
                .field("content_bytes", &content.byte_count())
                .finish(),
            Self::Approve => formatter.write_str("Approve"),
            Self::ApproveEdited(content) => formatter
                .debug_struct("ApproveEdited")
                .field("content", &"[REDACTED]")
                .field("content_bytes", &content.byte_count())
                .finish(),
            Self::Reject => formatter.write_str("Reject"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedMemoryReviewOutcome {
    Edited,
    Approved,
    Rejected,
}

#[derive(Clone, Eq, PartialEq)]
pub struct SharedMemoryReviewReceipt {
    proposal_id: SharedMemoryProposalId,
    outcome: SharedMemoryReviewOutcome,
    proposal_version: MemoryRecordVersion,
    approved_record_id: Option<MemoryRecordId>,
}

impl SharedMemoryReviewReceipt {
    #[must_use]
    pub fn proposal_id(&self) -> &SharedMemoryProposalId {
        &self.proposal_id
    }

    #[must_use]
    pub const fn outcome(&self) -> SharedMemoryReviewOutcome {
        self.outcome
    }

    #[must_use]
    pub const fn proposal_version(&self) -> MemoryRecordVersion {
        self.proposal_version
    }

    #[must_use]
    pub fn approved_record_id(&self) -> Option<&MemoryRecordId> {
        self.approved_record_id.as_ref()
    }
}

impl fmt::Debug for SharedMemoryReviewReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SharedMemoryReviewReceipt")
            .field("proposal_id", &self.proposal_id)
            .field("outcome", &self.outcome)
            .field("proposal_version", &self.proposal_version)
            .field("approved_record_id", &self.approved_record_id)
            .finish()
    }
}

/// Linear live authority derived by `AgentOrchestrator` for one operation.
pub struct MemoryAccessGrant {
    attribution: AgentAttribution,
}

impl MemoryAccessGrant {
    pub(crate) fn from_live_attribution(
        attribution: AgentAttribution,
        _proof: LiveMemoryAccessProof,
    ) -> Self {
        Self { attribution }
    }
}

impl fmt::Debug for MemoryAccessGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryAccessGrant")
            .field("attribution", &self.attribution)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct MemoryRecord {
    view: MemoryRecordView,
}

#[derive(Clone, Eq, PartialEq)]
struct SharedMemoryProposal {
    view: SharedMemoryProposalView,
}

pub struct MemoryStore {
    workflow_sequence: u64,
    root_task_id: Option<RootTaskId>,
    enabled: bool,
    records: BTreeMap<MemoryRecordId, MemoryRecord>,
    proposals: BTreeMap<SharedMemoryProposalId, SharedMemoryProposal>,
    retained_bytes: usize,
    next_record_sequence: u64,
    next_proposal_sequence: u64,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    #[must_use]
    pub fn new() -> Self {
        Self::for_workflow(0)
    }

    pub(crate) fn for_workflow(workflow_sequence: u64) -> Self {
        Self {
            workflow_sequence,
            root_task_id: None,
            enabled: true,
            records: BTreeMap::new(),
            proposals: BTreeMap::new(),
            retained_bytes: 0,
            next_record_sequence: 1,
            next_proposal_sequence: 1,
        }
    }

    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    #[must_use]
    pub fn proposal_count(&self) -> usize {
        self.proposals.len()
    }

    #[must_use]
    pub const fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    pub(crate) fn bind_root(
        &mut self,
        root_task_id: RootTaskId,
        _proof: ApplicationMemoryControlProof,
    ) -> MemoryStoreResult<()> {
        match &self.root_task_id {
            None => {
                self.root_task_id = Some(root_task_id);
                Ok(())
            }
            Some(bound) if bound == &root_task_id => Ok(()),
            Some(_) => Err(MemoryStoreError::RootWorkflowMismatch),
        }
    }

    pub(crate) fn write(
        &mut self,
        grant: &MemoryAccessGrant,
        target: MemoryWriteTarget,
        content: MemoryContent,
    ) -> MemoryStoreResult<MemoryRecordView> {
        self.ensure_agent_enabled(grant)?;
        self.ensure_record_capacity(content.byte_count())?;
        let namespace = match target {
            MemoryWriteTarget::AgentPrivate => MemoryNamespace::AgentPrivate,
            MemoryWriteTarget::TaskTemporary => MemoryNamespace::TaskTemporary,
        };
        let id = self.next_record_id()?;
        let view = MemoryRecordView {
            id: id.clone(),
            namespace,
            owner_agent_id: grant.attribution.agent_id(),
            owner_task_id: matches!(namespace, MemoryNamespace::TaskTemporary)
                .then(|| grant.attribution.task_id().clone()),
            version: MemoryRecordVersion::INITIAL,
            content,
        };
        self.retained_bytes += view.content.byte_count();
        self.records.insert(id, MemoryRecord { view: view.clone() });
        Ok(view)
    }

    pub(crate) fn read(
        &self,
        grant: &MemoryAccessGrant,
        record_id: &MemoryRecordId,
    ) -> MemoryStoreResult<MemoryRecordView> {
        self.ensure_agent_enabled(grant)?;
        let record = self
            .records
            .get(record_id)
            .ok_or(MemoryStoreError::RecordNotFound)?;
        self.ensure_record_readable(grant, &record.view)?;
        Ok(record.view.clone())
    }

    pub(crate) fn select(
        &self,
        grant: &MemoryAccessGrant,
        selection: &MemoryContextSelection,
    ) -> MemoryStoreResult<MemoryContextBundle> {
        self.assemble_context(grant, selection, false)
    }

    pub(crate) fn select_approved_shared_for_child(
        &self,
        grant: &MemoryAccessGrant,
        selection: &MemoryContextSelection,
    ) -> MemoryStoreResult<MemoryContextBundle> {
        self.assemble_context(grant, selection, true)
    }

    pub(crate) fn propose_shared(
        &mut self,
        grant: &MemoryAccessGrant,
        content: MemoryContent,
    ) -> MemoryStoreResult<SharedMemoryProposalView> {
        self.ensure_agent_enabled(grant)?;
        if self.proposals.len() >= MAX_SHARED_MEMORY_PROPOSALS {
            return Err(MemoryStoreError::ProposalCapacityExceeded);
        }
        self.ensure_total_capacity(content.byte_count())?;
        let id = self.next_proposal_id()?;
        let view = SharedMemoryProposalView {
            id: id.clone(),
            proposer_agent_id: grant.attribution.agent_id(),
            proposer_task_id: grant.attribution.task_id().clone(),
            root_task_id: grant.attribution.root_task_id().clone(),
            version: MemoryRecordVersion::INITIAL,
            content,
        };
        self.retained_bytes += view.content.byte_count();
        self.proposals
            .insert(id, SharedMemoryProposal { view: view.clone() });
        Ok(view)
    }

    pub(crate) fn proposal(
        &self,
        proposal_id: &SharedMemoryProposalId,
        _proof: ApplicationMemoryControlProof,
    ) -> MemoryStoreResult<SharedMemoryProposalView> {
        self.ensure_enabled()?;
        self.proposals
            .get(proposal_id)
            .map(|proposal| proposal.view.clone())
            .ok_or(MemoryStoreError::ProposalNotFound)
    }

    pub(crate) fn withdraw_shared(
        &mut self,
        grant: &MemoryAccessGrant,
        proposal_id: &SharedMemoryProposalId,
        expected_version: MemoryRecordVersion,
    ) -> MemoryStoreResult<()> {
        self.ensure_agent_enabled(grant)?;
        let proposal = self
            .proposals
            .get(proposal_id)
            .ok_or(MemoryStoreError::ProposalNotFound)?;
        if proposal.view.proposer_agent_id != grant.attribution.agent_id()
            || proposal.view.proposer_task_id != *grant.attribution.task_id()
        {
            return Err(MemoryStoreError::AccessDenied);
        }
        ensure_version(proposal.view.version, expected_version)?;
        let removed = self
            .proposals
            .remove(proposal_id)
            .ok_or(MemoryStoreError::ProposalNotFound)?;
        self.retained_bytes -= removed.view.content.byte_count();
        Ok(())
    }

    pub(crate) fn delete(
        &mut self,
        grant: &MemoryAccessGrant,
        record_id: &MemoryRecordId,
        expected_version: MemoryRecordVersion,
    ) -> MemoryStoreResult<()> {
        self.ensure_agent_enabled(grant)?;
        let record = self
            .records
            .get(record_id)
            .ok_or(MemoryStoreError::RecordNotFound)?;
        match record.view.namespace {
            MemoryNamespace::AgentPrivate
                if record.view.owner_agent_id == grant.attribution.agent_id() => {}
            MemoryNamespace::TaskTemporary
                if record.view.owner_agent_id == grant.attribution.agent_id()
                    && record.view.owner_task_id.as_ref() == Some(grant.attribution.task_id()) => {}
            _ => return Err(MemoryStoreError::AccessDenied),
        }
        ensure_version(record.view.version, expected_version)?;
        let removed = self
            .records
            .remove(record_id)
            .ok_or(MemoryStoreError::RecordNotFound)?;
        self.retained_bytes -= removed.view.content.byte_count();
        Ok(())
    }

    pub(crate) fn review_shared(
        &mut self,
        proposal_id: &SharedMemoryProposalId,
        expected_version: MemoryRecordVersion,
        decision: SharedMemoryReviewDecision,
        _proof: ApplicationMemoryControlProof,
    ) -> MemoryStoreResult<SharedMemoryReviewReceipt> {
        self.ensure_enabled()?;
        let proposal = self
            .proposals
            .get(proposal_id)
            .ok_or(MemoryStoreError::ProposalNotFound)?;
        ensure_version(proposal.view.version, expected_version)?;

        match decision {
            SharedMemoryReviewDecision::Edit(content) => {
                let next_version = expected_version.next()?;
                self.ensure_replacement_capacity(
                    proposal.view.content.byte_count(),
                    content.byte_count(),
                )?;
                let proposal = self
                    .proposals
                    .get_mut(proposal_id)
                    .ok_or(MemoryStoreError::ProposalNotFound)?;
                self.retained_bytes =
                    self.retained_bytes - proposal.view.content.byte_count() + content.byte_count();
                proposal.view.content = content;
                proposal.view.version = next_version;
                Ok(SharedMemoryReviewReceipt {
                    proposal_id: proposal_id.clone(),
                    outcome: SharedMemoryReviewOutcome::Edited,
                    proposal_version: next_version,
                    approved_record_id: None,
                })
            }
            SharedMemoryReviewDecision::Approve => {
                self.promote_proposal(proposal_id, expected_version, None)
            }
            SharedMemoryReviewDecision::ApproveEdited(content) => {
                self.promote_proposal(proposal_id, expected_version, Some(content))
            }
            SharedMemoryReviewDecision::Reject => {
                let removed = self
                    .proposals
                    .remove(proposal_id)
                    .ok_or(MemoryStoreError::ProposalNotFound)?;
                self.retained_bytes -= removed.view.content.byte_count();
                Ok(SharedMemoryReviewReceipt {
                    proposal_id: proposal_id.clone(),
                    outcome: SharedMemoryReviewOutcome::Rejected,
                    proposal_version: expected_version,
                    approved_record_id: None,
                })
            }
        }
    }

    pub(crate) fn delete_approved_shared(
        &mut self,
        record_id: &MemoryRecordId,
        expected_version: MemoryRecordVersion,
        _proof: ApplicationMemoryControlProof,
    ) -> MemoryStoreResult<()> {
        self.ensure_enabled()?;
        let record = self
            .records
            .get(record_id)
            .ok_or(MemoryStoreError::RecordNotFound)?;
        if record.view.namespace != MemoryNamespace::ApprovedShared {
            return Err(MemoryStoreError::AccessDenied);
        }
        ensure_version(record.view.version, expected_version)?;
        let removed = self
            .records
            .remove(record_id)
            .ok_or(MemoryStoreError::RecordNotFound)?;
        self.retained_bytes -= removed.view.content.byte_count();
        Ok(())
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool, _proof: ApplicationMemoryControlProof) {
        if !enabled {
            self.records.clear();
            self.proposals.clear();
            self.retained_bytes = 0;
        }
        self.enabled = enabled;
    }

    pub(crate) fn cleanup_task(&mut self, task_id: &AgentTaskId) {
        let ids: Vec<_> = self
            .records
            .iter()
            .filter(|(_, record)| {
                record.view.namespace == MemoryNamespace::TaskTemporary
                    && record.view.owner_task_id.as_ref() == Some(task_id)
            })
            .map(|(id, _)| id.clone())
            .collect();
        for id in ids {
            if let Some(record) = self.records.remove(&id) {
                self.retained_bytes -= record.view.content.byte_count();
            }
        }
    }

    fn assemble_context(
        &self,
        grant: &MemoryAccessGrant,
        selection: &MemoryContextSelection,
        approved_shared_only: bool,
    ) -> MemoryStoreResult<MemoryContextBundle> {
        self.ensure_agent_enabled(grant)?;
        if approved_shared_only && grant.attribution.agent_id() != AgentId::PersonalAssistant {
            return Err(MemoryStoreError::AccessDenied);
        }
        let mut records = Vec::with_capacity(selection.0.len());
        let mut text = String::new();
        for record_id in &selection.0 {
            let record = self
                .records
                .get(record_id)
                .ok_or(MemoryStoreError::RecordNotFound)?;
            if approved_shared_only {
                if record.view.namespace != MemoryNamespace::ApprovedShared {
                    return Err(MemoryStoreError::CrossAgentSelectionDenied);
                }
            } else {
                self.ensure_record_readable(grant, &record.view)?;
            }
            let separator_bytes = usize::from(!text.is_empty());
            let next = text
                .len()
                .checked_add(separator_bytes)
                .and_then(|value| value.checked_add(record.view.content.byte_count()))
                .ok_or(MemoryStoreError::ContextTooLarge)?;
            if next > MAX_MEMORY_CONTEXT_BYTES {
                return Err(MemoryStoreError::ContextTooLarge);
            }
            if separator_bytes == 1 {
                text.push('\n');
            }
            text.push_str(record.view.content.as_str());
            records.push(record.view.clone());
        }
        Ok(MemoryContextBundle { records, text })
    }

    fn ensure_record_readable(
        &self,
        grant: &MemoryAccessGrant,
        view: &MemoryRecordView,
    ) -> MemoryStoreResult<()> {
        let allowed = match view.namespace {
            MemoryNamespace::ApprovedShared => {
                grant.attribution.memory_profile_id()
                    == AgentMemoryProfileId::PersonalAssistantMemoryV1
            }
            MemoryNamespace::AgentPrivate => view.owner_agent_id == grant.attribution.agent_id(),
            MemoryNamespace::TaskTemporary => {
                view.owner_agent_id == grant.attribution.agent_id()
                    && view.owner_task_id.as_ref() == Some(grant.attribution.task_id())
            }
            MemoryNamespace::ProposedShared => false,
        };
        if allowed {
            Ok(())
        } else {
            Err(MemoryStoreError::AccessDenied)
        }
    }

    fn promote_proposal(
        &mut self,
        proposal_id: &SharedMemoryProposalId,
        expected_version: MemoryRecordVersion,
        edited: Option<MemoryContent>,
    ) -> MemoryStoreResult<SharedMemoryReviewReceipt> {
        if self.records.len() >= MAX_MEMORY_RECORDS {
            return Err(MemoryStoreError::RecordCapacityExceeded);
        }
        let proposal = self
            .proposals
            .get(proposal_id)
            .ok_or(MemoryStoreError::ProposalNotFound)?;
        let content = edited.unwrap_or_else(|| proposal.view.content.clone());
        self.ensure_replacement_capacity(proposal.view.content.byte_count(), content.byte_count())?;
        let owner_agent_id = proposal.view.proposer_agent_id;
        let record_id = self.next_record_id()?;
        let removed = self
            .proposals
            .remove(proposal_id)
            .ok_or(MemoryStoreError::ProposalNotFound)?;
        self.retained_bytes =
            self.retained_bytes - removed.view.content.byte_count() + content.byte_count();
        let view = MemoryRecordView {
            id: record_id.clone(),
            namespace: MemoryNamespace::ApprovedShared,
            owner_agent_id,
            owner_task_id: None,
            version: MemoryRecordVersion::INITIAL,
            content,
        };
        self.records
            .insert(record_id.clone(), MemoryRecord { view });
        Ok(SharedMemoryReviewReceipt {
            proposal_id: proposal_id.clone(),
            outcome: SharedMemoryReviewOutcome::Approved,
            proposal_version: expected_version,
            approved_record_id: Some(record_id),
        })
    }

    fn ensure_agent_enabled(&self, grant: &MemoryAccessGrant) -> MemoryStoreResult<()> {
        self.ensure_enabled()?;
        match &self.root_task_id {
            Some(root_task_id) if root_task_id == grant.attribution.root_task_id() => {}
            Some(_) => return Err(MemoryStoreError::RootWorkflowMismatch),
            None => return Err(MemoryStoreError::RootWorkflowUnbound),
        }
        match grant.attribution.memory_profile_id() {
            AgentMemoryProfileId::PersonalAssistantMemoryV1
            | AgentMemoryProfileId::ResearchWorkingMemoryV1
            | AgentMemoryProfileId::KnowledgeWorkingMemoryV1 => Ok(()),
            AgentMemoryProfileId::MemoryDisabledV1 => Err(MemoryStoreError::ProfileDisabled),
        }
    }

    fn ensure_enabled(&self) -> MemoryStoreResult<()> {
        if self.enabled {
            Ok(())
        } else {
            Err(MemoryStoreError::MemoryDisabled)
        }
    }

    fn ensure_record_capacity(&self, additional_bytes: usize) -> MemoryStoreResult<()> {
        if self.records.len() >= MAX_MEMORY_RECORDS {
            return Err(MemoryStoreError::RecordCapacityExceeded);
        }
        self.ensure_total_capacity(additional_bytes)
    }

    fn ensure_total_capacity(&self, additional_bytes: usize) -> MemoryStoreResult<()> {
        let next = self
            .retained_bytes
            .checked_add(additional_bytes)
            .ok_or(MemoryStoreError::RetainedBytesExceeded)?;
        if next > MAX_MEMORY_RETAINED_BYTES {
            Err(MemoryStoreError::RetainedBytesExceeded)
        } else {
            Ok(())
        }
    }

    fn ensure_replacement_capacity(
        &self,
        previous_bytes: usize,
        replacement_bytes: usize,
    ) -> MemoryStoreResult<()> {
        let base = self
            .retained_bytes
            .checked_sub(previous_bytes)
            .ok_or(MemoryStoreError::RetainedBytesExceeded)?;
        let next = base
            .checked_add(replacement_bytes)
            .ok_or(MemoryStoreError::RetainedBytesExceeded)?;
        if next > MAX_MEMORY_RETAINED_BYTES {
            Err(MemoryStoreError::RetainedBytesExceeded)
        } else {
            Ok(())
        }
    }

    fn next_record_id(&mut self) -> MemoryStoreResult<MemoryRecordId> {
        let sequence = self.next_record_sequence;
        self.next_record_sequence = sequence
            .checked_add(1)
            .ok_or(MemoryStoreError::IdentityExhausted)?;
        Ok(MemoryRecordId(format!(
            "memory-record-{}-{sequence}",
            self.workflow_sequence
        )))
    }

    fn next_proposal_id(&mut self) -> MemoryStoreResult<SharedMemoryProposalId> {
        let sequence = self.next_proposal_sequence;
        self.next_proposal_sequence = sequence
            .checked_add(1)
            .ok_or(MemoryStoreError::IdentityExhausted)?;
        Ok(SharedMemoryProposalId(format!(
            "shared-memory-proposal-{}-{sequence}",
            self.workflow_sequence
        )))
    }
}

impl fmt::Debug for MemoryStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryStore")
            .field("enabled", &self.enabled)
            .field("record_count", &self.records.len())
            .field("proposal_count", &self.proposals.len())
            .field("retained_bytes", &self.retained_bytes)
            .finish()
    }
}

fn ensure_version(
    actual: MemoryRecordVersion,
    expected: MemoryRecordVersion,
) -> MemoryStoreResult<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(MemoryStoreError::VersionMismatch { expected, actual })
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum MemoryStoreError {
    #[error("memory is disabled")]
    MemoryDisabled,
    #[error("the agent memory profile is disabled")]
    ProfileDisabled,
    #[error("memory access is denied")]
    AccessDenied,
    #[error("the memory store is not bound to a root workflow")]
    RootWorkflowUnbound,
    #[error("memory attribution belongs to another root workflow")]
    RootWorkflowMismatch,
    #[error("cross-agent context may contain only approved shared records")]
    CrossAgentSelectionDenied,
    #[error("memory content must contain a non-whitespace character")]
    EmptyContent,
    #[error("memory content must not have surrounding whitespace")]
    NonCanonicalContent,
    #[error("memory content contains a prohibited control character")]
    ContentContainsControlCharacter,
    #[error("memory content exceeds {maximum_bytes} bytes")]
    ContentTooLarge {
        maximum_bytes: usize,
        actual_bytes: usize,
    },
    #[error("the memory record does not exist")]
    RecordNotFound,
    #[error("the shared-memory proposal does not exist")]
    ProposalNotFound,
    #[error("the memory record version does not match")]
    VersionMismatch {
        expected: MemoryRecordVersion,
        actual: MemoryRecordVersion,
    },
    #[error("the memory record capacity is exhausted")]
    RecordCapacityExceeded,
    #[error("the shared-memory proposal capacity is exhausted")]
    ProposalCapacityExceeded,
    #[error("the retained memory byte capacity is exhausted")]
    RetainedBytesExceeded,
    #[error("the memory context contains too many records")]
    ContextRecordLimitExceeded,
    #[error("the memory context contains duplicate records")]
    DuplicateContextRecord,
    #[error("the assembled memory context exceeds its byte limit")]
    ContextTooLarge,
    #[error("the monotonic memory identity source is exhausted")]
    IdentityExhausted,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{
        orchestrator::{AgentOrchestrator, DelegationProposal},
        task::AgentTaskId,
    };

    fn personal_and_research_grants(
    ) -> Result<(MemoryAccessGrant, MemoryAccessGrant), Box<dyn std::error::Error>> {
        let mut orchestrator = AgentOrchestrator::native()?;
        let root = orchestrator.start_root("Bounded memory fixture")?;
        let personal_attribution = orchestrator.live_attribution_for_test(&root)?;
        let accepted = orchestrator.request_delegation(
            &root,
            DelegationProposal::new(
                AgentId::Research,
                "Analyze supplied memory",
                None,
                "Return one summary",
            )?,
        )?;
        let research_attribution =
            orchestrator.live_attribution_for_test(accepted.child_context())?;
        Ok((
            MemoryAccessGrant::from_live_attribution(
                personal_attribution,
                LiveMemoryAccessProof::for_test(),
            ),
            MemoryAccessGrant::from_live_attribution(
                research_attribution,
                LiveMemoryAccessProof::for_test(),
            ),
        ))
    }

    fn bound_store(grant: &MemoryAccessGrant) -> MemoryStoreResult<MemoryStore> {
        let mut store = MemoryStore::new();
        store.bind_root(
            grant.attribution.root_task_id().clone(),
            ApplicationMemoryControlProof::for_test(),
        )?;
        Ok(store)
    }

    #[test]
    fn proposal_review_is_versioned_atomic_and_shared_read_is_personal_only(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (personal, research) = personal_and_research_grants()?;
        let mut store = bound_store(&personal)?;
        let proposal =
            store.propose_shared(&research, MemoryContent::new("research proposal sentinel")?)?;
        let edit = store.review_shared(
            proposal.id(),
            proposal.version(),
            SharedMemoryReviewDecision::Edit(MemoryContent::new("reviewed sentinel")?),
            ApplicationMemoryControlProof::for_test(),
        )?;
        assert_eq!(edit.outcome(), SharedMemoryReviewOutcome::Edited);
        assert_eq!(
            store.review_shared(
                proposal.id(),
                proposal.version(),
                SharedMemoryReviewDecision::Approve,
                ApplicationMemoryControlProof::for_test(),
            ),
            Err(MemoryStoreError::VersionMismatch {
                expected: proposal.version(),
                actual: edit.proposal_version(),
            })
        );
        let approved = store.review_shared(
            proposal.id(),
            edit.proposal_version(),
            SharedMemoryReviewDecision::Approve,
            ApplicationMemoryControlProof::for_test(),
        )?;
        let record_id = approved.approved_record_id().ok_or("missing record")?;
        assert_eq!(
            store.read(&research, record_id),
            Err(MemoryStoreError::AccessDenied)
        );
        assert_eq!(
            store.read(&personal, record_id)?.content().as_str(),
            "reviewed sentinel"
        );
        Ok(())
    }

    #[test]
    fn private_task_selection_cleanup_disable_and_debug_are_closed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (personal, research) = personal_and_research_grants()?;
        let mut store = bound_store(&personal)?;
        let private = store.write(
            &personal,
            MemoryWriteTarget::AgentPrivate,
            MemoryContent::new("private memory sentinel")?,
        )?;
        let temporary = store.write(
            &personal,
            MemoryWriteTarget::TaskTemporary,
            MemoryContent::new("task memory sentinel")?,
        )?;
        assert_eq!(
            store.read(&research, private.id()),
            Err(MemoryStoreError::AccessDenied)
        );
        assert_eq!(
            store.select_approved_shared_for_child(
                &personal,
                &MemoryContextSelection::new([private.id().clone()])?,
            ),
            Err(MemoryStoreError::CrossAgentSelectionDenied)
        );
        store.cleanup_task(temporary.owner_task_id().ok_or("missing owner")?);
        assert_eq!(
            store.read(&personal, temporary.id()),
            Err(MemoryStoreError::RecordNotFound)
        );
        let debug = format!("{private:?} {store:?}");
        assert!(!debug.contains("private memory sentinel"));
        store.set_enabled(false, ApplicationMemoryControlProof::for_test());
        assert_eq!(store.record_count(), 0);
        assert_eq!(
            store.write(
                &personal,
                MemoryWriteTarget::AgentPrivate,
                MemoryContent::new("blocked")?,
            ),
            Err(MemoryStoreError::MemoryDisabled)
        );
        store.set_enabled(true, ApplicationMemoryControlProof::for_test());
        assert!(store.is_enabled());
        Ok(())
    }

    #[test]
    fn content_selection_and_identifiers_enforce_bounds_and_redaction(
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(MemoryContent::new(" "), Err(MemoryStoreError::EmptyContent));
        assert!(matches!(
            MemoryContent::new("x".repeat(MAX_MEMORY_RECORD_BYTES + 1)),
            Err(MemoryStoreError::ContentTooLarge { .. })
        ));
        let ids: Vec<_> = (0..=MAX_MEMORY_CONTEXT_RECORDS)
            .map(|index| MemoryRecordId(format!("record-{index}")))
            .collect();
        assert_eq!(
            MemoryContextSelection::new(ids),
            Err(MemoryStoreError::ContextRecordLimitExceeded)
        );
        assert_eq!(
            MemoryContextSelection::new(std::iter::repeat(MemoryRecordId("hostile".into()))),
            Err(MemoryStoreError::ContextRecordLimitExceeded)
        );
        let id = MemoryRecordId("secret-id".into());
        assert!(!format!("{id:?}").contains("secret-id"));
        let task_id = AgentTaskId::new("task-fixture")?;
        assert!(!format!("{task_id:?}").contains("task-fixture"));
        Ok(())
    }

    #[test]
    fn review_withdraw_delete_capacity_and_context_failures_are_atomic(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (personal, research) = personal_and_research_grants()?;
        let mut store = bound_store(&personal)?;

        let edited_proposal =
            store.propose_shared(&research, MemoryContent::new("original proposal")?)?;
        assert_eq!(edited_proposal.namespace(), MemoryNamespace::ProposedShared);
        let approved = store.review_shared(
            edited_proposal.id(),
            edited_proposal.version(),
            SharedMemoryReviewDecision::ApproveEdited(MemoryContent::new("approved edit")?),
            ApplicationMemoryControlProof::for_test(),
        )?;
        let approved_id = approved.approved_record_id().ok_or("missing record")?;
        assert_eq!(
            store.read(&personal, approved_id)?.content().as_str(),
            "approved edit"
        );

        let withdrawal =
            store.propose_shared(&research, MemoryContent::new("withdrawal draft")?)?;
        let withdrawal_edit = store.review_shared(
            withdrawal.id(),
            withdrawal.version(),
            SharedMemoryReviewDecision::Edit(MemoryContent::new("withdrawal edit")?),
            ApplicationMemoryControlProof::for_test(),
        )?;
        let retained_before = store.retained_bytes();
        assert!(matches!(
            store.withdraw_shared(&research, withdrawal.id(), withdrawal.version()),
            Err(MemoryStoreError::VersionMismatch { .. })
        ));
        assert_eq!(store.retained_bytes(), retained_before);
        assert_eq!(
            store.proposal(withdrawal.id(), ApplicationMemoryControlProof::for_test())?,
            SharedMemoryProposalView {
                version: withdrawal_edit.proposal_version(),
                content: MemoryContent::new("withdrawal edit")?,
                ..withdrawal.clone()
            }
        );
        store.withdraw_shared(
            &research,
            withdrawal.id(),
            withdrawal_edit.proposal_version(),
        )?;
        assert_eq!(
            store.proposal(withdrawal.id(), ApplicationMemoryControlProof::for_test()),
            Err(MemoryStoreError::ProposalNotFound)
        );

        let private = store.write(
            &personal,
            MemoryWriteTarget::AgentPrivate,
            MemoryContent::new("delete fixture")?,
        )?;
        let private_bytes = store.retained_bytes();
        assert!(matches!(
            store.delete(&personal, private.id(), MemoryRecordVersion(2)),
            Err(MemoryStoreError::VersionMismatch { .. })
        ));
        assert_eq!(store.retained_bytes(), private_bytes);
        assert_eq!(store.read(&personal, private.id())?, private);
        store.delete(&personal, private.id(), private.version())?;

        let first = store.write(
            &personal,
            MemoryWriteTarget::AgentPrivate,
            MemoryContent::new("a".repeat(MAX_MEMORY_RECORD_BYTES))?,
        )?;
        let second = store.write(
            &personal,
            MemoryWriteTarget::AgentPrivate,
            MemoryContent::new("b".repeat(MAX_MEMORY_RECORD_BYTES))?,
        )?;
        assert_eq!(
            MemoryContextSelection::new([first.id().clone(), first.id().clone()]),
            Err(MemoryStoreError::DuplicateContextRecord)
        );
        assert_eq!(
            store.select(
                &personal,
                &MemoryContextSelection::new([first.id().clone(), second.id().clone()])?,
            ),
            Err(MemoryStoreError::ContextTooLarge)
        );

        let mut capacity = bound_store(&personal)?;
        for index in 0..MAX_SHARED_MEMORY_PROPOSALS {
            capacity.propose_shared(
                &research,
                MemoryContent::new(format!("capacity proposal {index}"))?,
            )?;
        }
        let retained_before = capacity.retained_bytes();
        assert_eq!(
            capacity.propose_shared(&research, MemoryContent::new("one too many")?),
            Err(MemoryStoreError::ProposalCapacityExceeded)
        );
        assert_eq!(capacity.proposal_count(), MAX_SHARED_MEMORY_PROPOSALS);
        assert_eq!(capacity.retained_bytes(), retained_before);
        Ok(())
    }

    #[test]
    fn a_store_rejects_a_grant_from_another_root_workflow() -> Result<(), Box<dyn std::error::Error>>
    {
        let personal = personal_and_research_grants()?.0;
        let foreign = personal_and_research_grants()?.0;
        let mut store = bound_store(&personal)?;
        assert_eq!(
            store.write(
                &foreign,
                MemoryWriteTarget::AgentPrivate,
                MemoryContent::new("foreign root")?,
            ),
            Err(MemoryStoreError::RootWorkflowMismatch)
        );
        assert_eq!(store.record_count(), 0);
        assert_eq!(store.retained_bytes(), 0);
        Ok(())
    }

    #[test]
    fn record_and_retained_capacity_failures_preserve_all_state(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (personal, research) = personal_and_research_grants()?;

        let mut record_capacity = bound_store(&personal)?;
        for index in 0..MAX_MEMORY_RECORDS {
            record_capacity.write(
                &personal,
                MemoryWriteTarget::AgentPrivate,
                MemoryContent::new(format!("record {index}"))?,
            )?;
        }
        let retained_before = record_capacity.retained_bytes();
        assert_eq!(
            record_capacity.write(
                &personal,
                MemoryWriteTarget::AgentPrivate,
                MemoryContent::new("one record too many")?,
            ),
            Err(MemoryStoreError::RecordCapacityExceeded)
        );
        assert_eq!(record_capacity.record_count(), MAX_MEMORY_RECORDS);
        assert_eq!(record_capacity.retained_bytes(), retained_before);

        let mut promotion_capacity = bound_store(&personal)?;
        let proposal = promotion_capacity
            .propose_shared(&research, MemoryContent::new("promotion proposal")?)?;
        for index in 0..MAX_MEMORY_RECORDS {
            promotion_capacity.write(
                &personal,
                MemoryWriteTarget::AgentPrivate,
                MemoryContent::new(format!("promotion record {index}"))?,
            )?;
        }
        let retained_before = promotion_capacity.retained_bytes();
        assert_eq!(
            promotion_capacity.review_shared(
                proposal.id(),
                proposal.version(),
                SharedMemoryReviewDecision::Approve,
                ApplicationMemoryControlProof::for_test(),
            ),
            Err(MemoryStoreError::RecordCapacityExceeded)
        );
        assert_eq!(promotion_capacity.record_count(), MAX_MEMORY_RECORDS);
        assert_eq!(promotion_capacity.proposal_count(), 1);
        assert_eq!(promotion_capacity.retained_bytes(), retained_before);

        let mut byte_capacity = bound_store(&personal)?;
        let proposal = byte_capacity.propose_shared(&research, MemoryContent::new("p")?)?;
        for index in 0..15 {
            byte_capacity.write(
                &personal,
                MemoryWriteTarget::AgentPrivate,
                MemoryContent::new(
                    std::iter::repeat_n(char::from(b'a' + index), MAX_MEMORY_RECORD_BYTES)
                        .collect::<String>(),
                )?,
            )?;
        }
        byte_capacity.write(
            &personal,
            MemoryWriteTarget::AgentPrivate,
            MemoryContent::new("z".repeat(MAX_MEMORY_RECORD_BYTES - 1))?,
        )?;
        assert_eq!(byte_capacity.retained_bytes(), MAX_MEMORY_RETAINED_BYTES);
        let records_before = byte_capacity.record_count();
        assert_eq!(
            byte_capacity.write(
                &personal,
                MemoryWriteTarget::AgentPrivate,
                MemoryContent::new("retained overflow")?,
            ),
            Err(MemoryStoreError::RetainedBytesExceeded)
        );
        assert_eq!(
            byte_capacity.review_shared(
                proposal.id(),
                proposal.version(),
                SharedMemoryReviewDecision::Edit(MemoryContent::new("pp")?),
                ApplicationMemoryControlProof::for_test(),
            ),
            Err(MemoryStoreError::RetainedBytesExceeded)
        );
        assert_eq!(
            byte_capacity.review_shared(
                proposal.id(),
                proposal.version(),
                SharedMemoryReviewDecision::ApproveEdited(MemoryContent::new("pp")?),
                ApplicationMemoryControlProof::for_test(),
            ),
            Err(MemoryStoreError::RetainedBytesExceeded)
        );
        assert_eq!(byte_capacity.record_count(), records_before);
        assert_eq!(byte_capacity.proposal_count(), 1);
        assert_eq!(byte_capacity.retained_bytes(), MAX_MEMORY_RETAINED_BYTES);
        assert_eq!(
            byte_capacity
                .proposal(proposal.id(), ApplicationMemoryControlProof::for_test())?
                .content()
                .as_str(),
            "p"
        );
        Ok(())
    }
}
