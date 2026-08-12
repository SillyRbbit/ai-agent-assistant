//! Closed, bounded, volatile audit evidence for native-agent governance.
//!
//! This module records only application-owned attribution and closed outcomes.
//! It deliberately excludes prompts, objectives, arguments, results, approval
//! text, credentials, arbitrary tool names, and hidden reasoning. Audit
//! receipts and records are evidence only and never authorize execution.

use std::fmt;

use crate::agent::{
    definition::AgentId,
    governance::{
        AgentApprovalAuditDisposition, AgentAttribution, AgentControlResult,
        AgentExecutionDisposition, AgentGovernanceAction, AgentGovernanceError,
        AgentGovernanceErrorCode, AgentPolicyAuditOutcome, DelegationMatrixOutcome,
    },
};

pub const MAX_AGENT_GOVERNANCE_RECORDS: usize = 32;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AgentGovernanceAuditSequence(u64);

impl AgentGovernanceAuditSequence {
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AgentGovernanceTick(u64);

impl AgentGovernanceTick {
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentGovernanceAuditReceipt {
    sequence: AgentGovernanceAuditSequence,
}

impl AgentGovernanceAuditReceipt {
    #[must_use]
    pub const fn sequence(self) -> AgentGovernanceAuditSequence {
        self.sequence
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentToolGovernanceLifecycleState {
    ValidationRejected,
    PolicyEvaluated,
    ApprovalPending,
    ApprovalResolved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentDelegationGovernanceLifecycleState {
    Denied,
    Allowed,
    ChildCreated,
    Failed,
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentToolGovernanceRecord {
    sequence: AgentGovernanceAuditSequence,
    created_tick: AgentGovernanceTick,
    updated_tick: AgentGovernanceTick,
    attribution: AgentAttribution,
    call_id: String,
    action: AgentGovernanceAction,
    lifecycle: AgentToolGovernanceLifecycleState,
    policy: AgentPolicyAuditOutcome,
    approval: AgentApprovalAuditDisposition,
    execution: AgentExecutionDisposition,
    error: Option<AgentGovernanceErrorCode>,
}

impl AgentToolGovernanceRecord {
    #[must_use]
    pub const fn sequence(&self) -> AgentGovernanceAuditSequence {
        self.sequence
    }

    #[must_use]
    pub const fn created_tick(&self) -> AgentGovernanceTick {
        self.created_tick
    }

    #[must_use]
    pub const fn updated_tick(&self) -> AgentGovernanceTick {
        self.updated_tick
    }

    #[must_use]
    pub const fn attribution(&self) -> &AgentAttribution {
        &self.attribution
    }

    #[must_use]
    pub const fn action(&self) -> AgentGovernanceAction {
        self.action
    }

    #[must_use]
    pub const fn lifecycle(&self) -> AgentToolGovernanceLifecycleState {
        self.lifecycle
    }

    #[must_use]
    pub const fn policy(&self) -> AgentPolicyAuditOutcome {
        self.policy
    }

    #[must_use]
    pub const fn approval(&self) -> AgentApprovalAuditDisposition {
        self.approval
    }

    #[must_use]
    pub const fn execution(&self) -> AgentExecutionDisposition {
        self.execution
    }

    #[must_use]
    pub const fn error(&self) -> Option<AgentGovernanceErrorCode> {
        self.error
    }
}

impl fmt::Debug for AgentToolGovernanceRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentToolGovernanceRecord")
            .field("sequence", &self.sequence)
            .field("created_tick", &self.created_tick)
            .field("updated_tick", &self.updated_tick)
            .field("attribution", &"[REDACTED]")
            .field("call_id", &"[REDACTED]")
            .field("action", &self.action)
            .field("lifecycle", &self.lifecycle)
            .field("policy", &self.policy)
            .field("approval", &self.approval)
            .field("execution", &self.execution)
            .field("error", &self.error)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentDelegationGovernanceRecord {
    sequence: AgentGovernanceAuditSequence,
    created_tick: AgentGovernanceTick,
    updated_tick: AgentGovernanceTick,
    attribution: AgentAttribution,
    target_agent_id: AgentId,
    lifecycle: AgentDelegationGovernanceLifecycleState,
    matrix: DelegationMatrixOutcome,
    approval: AgentApprovalAuditDisposition,
    control: AgentControlResult,
    error: Option<AgentGovernanceErrorCode>,
}

impl AgentDelegationGovernanceRecord {
    #[must_use]
    pub const fn sequence(&self) -> AgentGovernanceAuditSequence {
        self.sequence
    }

    #[must_use]
    pub const fn created_tick(&self) -> AgentGovernanceTick {
        self.created_tick
    }

    #[must_use]
    pub const fn updated_tick(&self) -> AgentGovernanceTick {
        self.updated_tick
    }

    #[must_use]
    pub const fn attribution(&self) -> &AgentAttribution {
        &self.attribution
    }

    #[must_use]
    pub const fn target_agent_id(&self) -> AgentId {
        self.target_agent_id
    }

    #[must_use]
    pub const fn lifecycle(&self) -> AgentDelegationGovernanceLifecycleState {
        self.lifecycle
    }

    #[must_use]
    pub const fn matrix(&self) -> DelegationMatrixOutcome {
        self.matrix
    }

    #[must_use]
    pub const fn approval(&self) -> AgentApprovalAuditDisposition {
        self.approval
    }

    #[must_use]
    pub const fn control(&self) -> AgentControlResult {
        self.control
    }

    #[must_use]
    pub const fn error(&self) -> Option<AgentGovernanceErrorCode> {
        self.error
    }
}

impl fmt::Debug for AgentDelegationGovernanceRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentDelegationGovernanceRecord")
            .field("sequence", &self.sequence)
            .field("created_tick", &self.created_tick)
            .field("updated_tick", &self.updated_tick)
            .field("attribution", &"[REDACTED]")
            .field("target_agent_id", &self.target_agent_id)
            .field("lifecycle", &self.lifecycle)
            .field("matrix", &self.matrix)
            .field("approval", &self.approval)
            .field("control", &self.control)
            .field("error", &self.error)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum AgentGovernanceRecord {
    Tool(AgentToolGovernanceRecord),
    Delegation(AgentDelegationGovernanceRecord),
}

impl AgentGovernanceRecord {
    #[must_use]
    pub const fn sequence(&self) -> AgentGovernanceAuditSequence {
        match self {
            Self::Tool(record) => record.sequence(),
            Self::Delegation(record) => record.sequence(),
        }
    }
}

impl fmt::Debug for AgentGovernanceRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool(record) => formatter.debug_tuple("Tool").field(record).finish(),
            Self::Delegation(record) => formatter.debug_tuple("Delegation").field(record).finish(),
        }
    }
}

#[derive(Eq, PartialEq)]
enum AgentGovernanceSubjectKey {
    Tool {
        attribution: AgentAttribution,
        call_id: String,
    },
    Delegation {
        attribution: AgentAttribution,
        target_agent_id: AgentId,
    },
}

impl AgentGovernanceSubjectKey {
    fn attribution(&self) -> &AgentAttribution {
        match self {
            Self::Tool { attribution, .. } | Self::Delegation { attribution, .. } => attribution,
        }
    }
}

impl fmt::Debug for AgentGovernanceSubjectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool { .. } => formatter
                .debug_struct("Tool")
                .field("attribution", &"[REDACTED]")
                .field("call_id", &"[REDACTED]")
                .finish(),
            Self::Delegation {
                target_agent_id, ..
            } => formatter
                .debug_struct("Delegation")
                .field("attribution", &"[REDACTED]")
                .field("target_agent_id", target_agent_id)
                .finish(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ToolRecordState {
    action: AgentGovernanceAction,
    lifecycle: AgentToolGovernanceLifecycleState,
    policy: AgentPolicyAuditOutcome,
    approval: AgentApprovalAuditDisposition,
    error: Option<AgentGovernanceErrorCode>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DelegationRecordState {
    lifecycle: AgentDelegationGovernanceLifecycleState,
    matrix: DelegationMatrixOutcome,
    control: AgentControlResult,
    error: Option<AgentGovernanceErrorCode>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AgentGovernanceSlotState {
    ToolReserved,
    Tool(ToolRecordState),
    DelegationReserved,
    Delegation(DelegationRecordState),
}

struct AgentGovernanceSlot {
    sequence: AgentGovernanceAuditSequence,
    created_tick: AgentGovernanceTick,
    updated_tick: AgentGovernanceTick,
    subject: AgentGovernanceSubjectKey,
    state: AgentGovernanceSlotState,
}

impl AgentGovernanceSlot {
    fn snapshot(&self) -> Option<AgentGovernanceRecord> {
        match self.state {
            AgentGovernanceSlotState::ToolReserved
            | AgentGovernanceSlotState::DelegationReserved => None,
            AgentGovernanceSlotState::Tool(state) => {
                let AgentGovernanceSubjectKey::Tool { call_id, .. } = &self.subject else {
                    return None;
                };
                Some(AgentGovernanceRecord::Tool(AgentToolGovernanceRecord {
                    sequence: self.sequence,
                    created_tick: self.created_tick,
                    updated_tick: self.updated_tick,
                    attribution: self.subject.attribution().clone(),
                    call_id: call_id.clone(),
                    action: state.action,
                    lifecycle: state.lifecycle,
                    policy: state.policy,
                    approval: state.approval,
                    execution: AgentExecutionDisposition::NotAttempted,
                    error: state.error,
                }))
            }
            AgentGovernanceSlotState::Delegation(state) => {
                let AgentGovernanceSubjectKey::Delegation {
                    target_agent_id, ..
                } = &self.subject
                else {
                    return None;
                };
                Some(AgentGovernanceRecord::Delegation(
                    AgentDelegationGovernanceRecord {
                        sequence: self.sequence,
                        created_tick: self.created_tick,
                        updated_tick: self.updated_tick,
                        attribution: self.subject.attribution().clone(),
                        target_agent_id: *target_agent_id,
                        lifecycle: state.lifecycle,
                        matrix: state.matrix,
                        approval: AgentApprovalAuditDisposition::NotRequired,
                        control: state.control,
                        error: state.error,
                    },
                ))
            }
        }
    }
}

impl fmt::Debug for AgentGovernanceSlot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentGovernanceSlot")
            .field("sequence", &self.sequence)
            .field("created_tick", &self.created_tick)
            .field("updated_tick", &self.updated_tick)
            .field("subject", &self.subject)
            .field("state", &self.state)
            .finish()
    }
}

pub(crate) trait AgentGovernanceClock: Send {
    fn next_tick(&mut self) -> AgentGovernanceTick;
}

#[derive(Debug, Default)]
struct SaturatingAgentGovernanceClock {
    current: u64,
}

impl AgentGovernanceClock for SaturatingAgentGovernanceClock {
    fn next_tick(&mut self) -> AgentGovernanceTick {
        self.current = self.current.saturating_add(1);
        AgentGovernanceTick(self.current)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AgentGovernanceSlotLocator {
    index: usize,
    sequence: AgentGovernanceAuditSequence,
}

pub(crate) struct AgentToolAuditReservation {
    locator: AgentGovernanceSlotLocator,
}

impl fmt::Debug for AgentToolAuditReservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentToolAuditReservation")
            .field("subject", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct AgentPendingApprovalAuditToken {
    locator: AgentGovernanceSlotLocator,
}

impl AgentPendingApprovalAuditToken {
    #[must_use]
    #[cfg(test)]
    pub(crate) const fn sequence(&self) -> AgentGovernanceAuditSequence {
        self.locator.sequence
    }
}

impl fmt::Debug for AgentPendingApprovalAuditToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentPendingApprovalAuditToken")
            .field("sequence", &self.locator.sequence)
            .field("subject", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct AgentDelegationAuditReservation {
    locator: AgentGovernanceSlotLocator,
}

impl fmt::Debug for AgentDelegationAuditReservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentDelegationAuditReservation")
            .field("subject", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct AgentPendingDelegationAuditToken {
    locator: AgentGovernanceSlotLocator,
}

impl AgentPendingDelegationAuditToken {
    #[must_use]
    #[cfg(test)]
    pub(crate) const fn sequence(&self) -> AgentGovernanceAuditSequence {
        self.locator.sequence
    }
}

impl fmt::Debug for AgentPendingDelegationAuditToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentPendingDelegationAuditToken")
            .field("sequence", &self.locator.sequence)
            .field("subject", &"[REDACTED]")
            .finish()
    }
}

pub struct InMemoryAgentGovernanceAudit {
    next_sequence: u64,
    clock: Box<dyn AgentGovernanceClock>,
    slots: Vec<Option<AgentGovernanceSlot>>,
}

impl InMemoryAgentGovernanceAudit {
    #[must_use]
    pub fn new() -> Self {
        Self::with_clock(SaturatingAgentGovernanceClock::default())
    }

    pub(crate) fn with_clock(clock: impl AgentGovernanceClock + 'static) -> Self {
        Self {
            next_sequence: 0,
            clock: Box::new(clock),
            slots: Vec::with_capacity(MAX_AGENT_GOVERNANCE_RECORDS),
        }
    }

    pub(crate) fn reserve_tool(
        &mut self,
        attribution: AgentAttribution,
        call_id: impl Into<String>,
    ) -> Result<AgentToolAuditReservation, AgentGovernanceError> {
        let locator = self.reserve(
            AgentGovernanceSubjectKey::Tool {
                attribution,
                call_id: call_id.into(),
            },
            AgentGovernanceSlotState::ToolReserved,
        )?;
        Ok(AgentToolAuditReservation { locator })
    }

    pub(crate) fn reserve_delegation(
        &mut self,
        attribution: AgentAttribution,
        target_agent_id: AgentId,
    ) -> Result<AgentDelegationAuditReservation, AgentGovernanceError> {
        let locator = self.reserve(
            AgentGovernanceSubjectKey::Delegation {
                attribution,
                target_agent_id,
            },
            AgentGovernanceSlotState::DelegationReserved,
        )?;
        Ok(AgentDelegationAuditReservation { locator })
    }

    pub(crate) fn abort_tool(&mut self, reservation: AgentToolAuditReservation) {
        self.abort(reservation.locator, AgentGovernanceSlotState::ToolReserved);
    }

    #[cfg(test)]
    pub(crate) fn abort_delegation(&mut self, reservation: AgentDelegationAuditReservation) {
        self.abort(
            reservation.locator,
            AgentGovernanceSlotState::DelegationReserved,
        );
    }

    pub(crate) fn commit_tool_validation_rejected(
        &mut self,
        reservation: AgentToolAuditReservation,
        action: AgentGovernanceAction,
        error: AgentGovernanceErrorCode,
    ) -> AgentGovernanceAuditReceipt {
        self.commit_tool(
            reservation.locator,
            ToolRecordState {
                action,
                lifecycle: AgentToolGovernanceLifecycleState::ValidationRejected,
                policy: AgentPolicyAuditOutcome::NotEvaluated,
                approval: AgentApprovalAuditDisposition::NotRequired,
                error: Some(error),
            },
        )
    }

    pub(crate) fn commit_tool_policy_evaluated(
        &mut self,
        reservation: AgentToolAuditReservation,
        action: AgentGovernanceAction,
        policy: AgentPolicyAuditOutcome,
        error: Option<AgentGovernanceErrorCode>,
    ) -> AgentGovernanceAuditReceipt {
        self.commit_tool(
            reservation.locator,
            ToolRecordState {
                action,
                lifecycle: AgentToolGovernanceLifecycleState::PolicyEvaluated,
                policy,
                approval: AgentApprovalAuditDisposition::NotRequired,
                error,
            },
        )
    }

    pub(crate) fn commit_tool_approval_pending(
        &mut self,
        reservation: AgentToolAuditReservation,
        action: AgentGovernanceAction,
        policy: AgentPolicyAuditOutcome,
    ) -> AgentPendingApprovalAuditToken {
        let locator = reservation.locator;
        let _receipt = self.commit_tool(
            locator,
            ToolRecordState {
                action,
                lifecycle: AgentToolGovernanceLifecycleState::ApprovalPending,
                policy,
                approval: AgentApprovalAuditDisposition::Pending,
                error: None,
            },
        );
        AgentPendingApprovalAuditToken { locator }
    }

    /// Updates the preallocated pending slot. Valid terminal updates cannot
    /// fail for capacity or clock reasons.
    pub(crate) fn resolve_tool_approval(
        &mut self,
        token: AgentPendingApprovalAuditToken,
        disposition: AgentApprovalAuditDisposition,
        error: Option<AgentGovernanceErrorCode>,
    ) {
        let (disposition, error) = match disposition {
            terminal @ (AgentApprovalAuditDisposition::Approved
            | AgentApprovalAuditDisposition::Rejected
            | AgentApprovalAuditDisposition::Cancelled
            | AgentApprovalAuditDisposition::Expired) => (terminal, error),
            AgentApprovalAuditDisposition::NotRequired | AgentApprovalAuditDisposition::Pending => {
                (
                    AgentApprovalAuditDisposition::Cancelled,
                    Some(AgentGovernanceErrorCode::ApprovalResolutionFailed),
                )
            }
        };
        let Some(AgentGovernanceSlotState::Tool(current)) = self.state_for(token.locator).copied()
        else {
            return;
        };
        if current.lifecycle != AgentToolGovernanceLifecycleState::ApprovalPending {
            return;
        }
        let tick = self.clock.next_tick();
        if let Some(slot) = self.slot_for_mut(token.locator) {
            slot.updated_tick = tick;
            slot.state = AgentGovernanceSlotState::Tool(ToolRecordState {
                lifecycle: AgentToolGovernanceLifecycleState::ApprovalResolved,
                approval: disposition,
                error,
                ..current
            });
        }
    }

    pub(crate) fn commit_delegation_denied(
        &mut self,
        reservation: AgentDelegationAuditReservation,
        matrix: DelegationMatrixOutcome,
        error: AgentGovernanceErrorCode,
    ) -> AgentGovernanceAuditReceipt {
        self.commit_delegation(
            reservation.locator,
            DelegationRecordState {
                lifecycle: AgentDelegationGovernanceLifecycleState::Denied,
                matrix,
                control: AgentControlResult::NotCreated,
                error: Some(error),
            },
        )
    }

    pub(crate) fn commit_delegation_allowed(
        &mut self,
        reservation: AgentDelegationAuditReservation,
    ) -> AgentPendingDelegationAuditToken {
        let locator = reservation.locator;
        let _receipt = self.commit_delegation(
            locator,
            DelegationRecordState {
                lifecycle: AgentDelegationGovernanceLifecycleState::Allowed,
                matrix: DelegationMatrixOutcome::Allowed,
                control: AgentControlResult::NotCreated,
                error: None,
            },
        );
        AgentPendingDelegationAuditToken { locator }
    }

    /// Updates the preallocated allowed slot. Valid finishing updates cannot
    /// fail for capacity or clock reasons.
    pub(crate) fn finish_delegation(
        &mut self,
        token: AgentPendingDelegationAuditToken,
        control: AgentControlResult,
        error: Option<AgentGovernanceErrorCode>,
    ) {
        let Some(AgentGovernanceSlotState::Delegation(current)) =
            self.state_for(token.locator).copied()
        else {
            return;
        };
        if current.lifecycle != AgentDelegationGovernanceLifecycleState::Allowed {
            return;
        }
        let (lifecycle, control) = match control {
            AgentControlResult::ChildCreated => (
                AgentDelegationGovernanceLifecycleState::ChildCreated,
                AgentControlResult::ChildCreated,
            ),
            AgentControlResult::Failed | AgentControlResult::NotCreated => (
                AgentDelegationGovernanceLifecycleState::Failed,
                AgentControlResult::Failed,
            ),
        };
        let tick = self.clock.next_tick();
        if let Some(slot) = self.slot_for_mut(token.locator) {
            slot.updated_tick = tick;
            slot.state = AgentGovernanceSlotState::Delegation(DelegationRecordState {
                lifecycle,
                control,
                error,
                ..current
            });
        }
    }

    #[must_use]
    pub fn records(&self) -> Vec<AgentGovernanceRecord> {
        let mut records: Vec<_> = self
            .slots
            .iter()
            .flatten()
            .filter_map(AgentGovernanceSlot::snapshot)
            .collect();
        records.sort_by_key(AgentGovernanceRecord::sequence);
        records
    }

    #[must_use]
    pub fn record_count(&self) -> usize {
        self.slots.iter().flatten().count()
    }

    fn reserve(
        &mut self,
        subject: AgentGovernanceSubjectKey,
        state: AgentGovernanceSlotState,
    ) -> Result<AgentGovernanceSlotLocator, AgentGovernanceError> {
        if self
            .slots
            .iter()
            .flatten()
            .any(|slot| slot.subject == subject)
        {
            return Err(AgentGovernanceError::DuplicateSubject);
        }
        if self.record_count() >= MAX_AGENT_GOVERNANCE_RECORDS {
            return Err(AgentGovernanceError::AuditCapacityExhausted {
                maximum: MAX_AGENT_GOVERNANCE_RECORDS,
            });
        }

        let next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(AgentGovernanceError::AuditSequenceExhausted)?;
        self.next_sequence = next_sequence;
        let sequence = AgentGovernanceAuditSequence(self.next_sequence);
        let tick = self.clock.next_tick();
        let slot = AgentGovernanceSlot {
            sequence,
            created_tick: tick,
            updated_tick: tick,
            subject,
            state,
        };
        let index = if let Some(index) = self.slots.iter().position(Option::is_none) {
            self.slots[index] = Some(slot);
            index
        } else {
            let index = self.slots.len();
            self.slots.push(Some(slot));
            index
        };
        Ok(AgentGovernanceSlotLocator { index, sequence })
    }

    fn abort(&mut self, locator: AgentGovernanceSlotLocator, expected: AgentGovernanceSlotState) {
        let may_abort = self
            .state_for(locator)
            .is_some_and(|state| *state == expected);
        if may_abort {
            self.slots[locator.index] = None;
        }
    }

    fn commit_tool(
        &mut self,
        locator: AgentGovernanceSlotLocator,
        state: ToolRecordState,
    ) -> AgentGovernanceAuditReceipt {
        let may_commit = self
            .state_for(locator)
            .is_some_and(|current| *current == AgentGovernanceSlotState::ToolReserved);
        if may_commit {
            let tick = self.clock.next_tick();
            if let Some(slot) = self.slot_for_mut(locator) {
                slot.updated_tick = tick;
                slot.state = AgentGovernanceSlotState::Tool(state);
            }
        }
        AgentGovernanceAuditReceipt {
            sequence: locator.sequence,
        }
    }

    fn commit_delegation(
        &mut self,
        locator: AgentGovernanceSlotLocator,
        state: DelegationRecordState,
    ) -> AgentGovernanceAuditReceipt {
        let may_commit = self
            .state_for(locator)
            .is_some_and(|current| *current == AgentGovernanceSlotState::DelegationReserved);
        if may_commit {
            let tick = self.clock.next_tick();
            if let Some(slot) = self.slot_for_mut(locator) {
                slot.updated_tick = tick;
                slot.state = AgentGovernanceSlotState::Delegation(state);
            }
        }
        AgentGovernanceAuditReceipt {
            sequence: locator.sequence,
        }
    }

    fn state_for(&self, locator: AgentGovernanceSlotLocator) -> Option<&AgentGovernanceSlotState> {
        let slot = self.slots.get(locator.index)?.as_ref()?;
        (slot.sequence == locator.sequence).then_some(&slot.state)
    }

    fn slot_for_mut(
        &mut self,
        locator: AgentGovernanceSlotLocator,
    ) -> Option<&mut AgentGovernanceSlot> {
        let slot = self.slots.get_mut(locator.index)?.as_mut()?;
        (slot.sequence == locator.sequence).then_some(slot)
    }
}

impl Default for InMemoryAgentGovernanceAudit {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for InMemoryAgentGovernanceAudit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryAgentGovernanceAudit")
            .field("next_sequence", &self.next_sequence)
            .field("record_count", &self.record_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{
        AgentDelegationGovernanceLifecycleState, AgentGovernanceClock, AgentGovernanceRecord,
        AgentGovernanceTick, AgentToolGovernanceLifecycleState, InMemoryAgentGovernanceAudit,
        MAX_AGENT_GOVERNANCE_RECORDS,
    };
    use crate::{
        agent::{
            definition::AgentId,
            governance::{
                AgentApprovalAuditDisposition, AgentAttribution, AgentControlResult,
                AgentGovernanceAction, AgentGovernanceError, AgentGovernanceErrorCode,
                AgentPolicyAuditOutcome, AgentPolicyReason, DelegationMatrixOutcome,
            },
            native_runtime::NativeAgentRuntime,
            orchestrator::AgentOrchestrator,
        },
        policy::types::{PolicyOutcome, PolicyReason},
    };

    fn attribution() -> Result<AgentAttribution, Box<dyn Error>> {
        let mut orchestrator = AgentOrchestrator::new(NativeAgentRuntime)?;
        let context = orchestrator.start_root("synthetic audit fixture")?;
        Ok(orchestrator.live_attribution_for_test(&context)?)
    }

    fn allow_policy() -> AgentPolicyAuditOutcome {
        AgentPolicyAuditOutcome::Evaluated {
            outcome: PolicyOutcome::Allow,
            reason: AgentPolicyReason::Deterministic(PolicyReason::InformationOnly),
        }
    }

    fn approval_policy() -> AgentPolicyAuditOutcome {
        AgentPolicyAuditOutcome::Evaluated {
            outcome: PolicyOutcome::RequireApproval,
            reason: AgentPolicyReason::Deterministic(PolicyReason::ReversibleRequiresApproval),
        }
    }

    #[test]
    fn tool_subject_is_reserved_before_commit_and_replay_is_consumed() -> Result<(), Box<dyn Error>>
    {
        let attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation = audit.reserve_tool(attribution.clone(), "call-one")?;
        assert_eq!(audit.record_count(), 1);
        assert!(audit.records().is_empty());

        let receipt = audit.commit_tool_policy_evaluated(
            reservation,
            AgentGovernanceAction::GetCurrentDatetime,
            allow_policy(),
            None,
        );
        assert_eq!(receipt.sequence().value(), 1);
        assert_eq!(audit.records().len(), 1);
        assert!(matches!(
            audit.reserve_tool(attribution, "call-one"),
            Err(AgentGovernanceError::DuplicateSubject)
        ));
        Ok(())
    }

    #[test]
    fn abort_frees_capacity_and_does_not_consume_replay_subject() -> Result<(), Box<dyn Error>> {
        let attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation = audit.reserve_tool(attribution.clone(), "retryable-call")?;
        audit.abort_tool(reservation);
        assert_eq!(audit.record_count(), 0);

        let retried = audit.reserve_tool(attribution, "retryable-call")?;
        audit.commit_tool_validation_rejected(
            retried,
            AgentGovernanceAction::UnknownTool,
            AgentGovernanceErrorCode::UnknownTool,
        );
        assert_eq!(audit.records().len(), 1);
        Ok(())
    }

    #[test]
    fn approval_resolution_updates_the_preallocated_slot_in_place() -> Result<(), Box<dyn Error>> {
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation = audit.reserve_tool(attribution()?, "approval-call")?;
        let token = audit.commit_tool_approval_pending(
            reservation,
            AgentGovernanceAction::CreateLocalTask,
            approval_policy(),
        );
        assert_eq!(token.sequence().value(), 1);
        let before = audit.records();
        let AgentGovernanceRecord::Tool(before) = &before[0] else {
            return Err("expected a tool record".into());
        };
        assert_eq!(
            before.lifecycle(),
            AgentToolGovernanceLifecycleState::ApprovalPending
        );
        let created_tick = before.created_tick();
        let pending_tick = before.updated_tick();

        audit.resolve_tool_approval(
            token,
            AgentApprovalAuditDisposition::Expired,
            Some(AgentGovernanceErrorCode::ApprovalExpired),
        );
        let after = audit.records();
        let AgentGovernanceRecord::Tool(after) = &after[0] else {
            return Err("expected a tool record".into());
        };
        assert_eq!(after.sequence().value(), 1);
        assert_eq!(after.created_tick(), created_tick);
        assert!(after.updated_tick() > pending_tick);
        assert_eq!(
            after.lifecycle(),
            AgentToolGovernanceLifecycleState::ApprovalResolved
        );
        assert_eq!(after.approval(), AgentApprovalAuditDisposition::Expired);
        assert_eq!(
            after.error(),
            Some(AgentGovernanceErrorCode::ApprovalExpired)
        );
        assert_eq!(audit.record_count(), 1);
        Ok(())
    }

    #[test]
    fn delegation_records_exact_target_and_finishes_in_place() -> Result<(), Box<dyn Error>> {
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation = audit.reserve_delegation(attribution()?, AgentId::Research)?;
        let token = audit.commit_delegation_allowed(reservation);
        assert_eq!(token.sequence().value(), 1);
        audit.finish_delegation(token, AgentControlResult::ChildCreated, None);

        let records = audit.records();
        let AgentGovernanceRecord::Delegation(record) = &records[0] else {
            return Err("expected a delegation record".into());
        };
        assert_eq!(record.target_agent_id(), AgentId::Research);
        assert_eq!(record.matrix(), DelegationMatrixOutcome::Allowed);
        assert_eq!(record.control(), AgentControlResult::ChildCreated);
        assert_eq!(
            record.lifecycle(),
            AgentDelegationGovernanceLifecycleState::ChildCreated
        );
        assert_eq!(audit.record_count(), 1);
        Ok(())
    }

    #[test]
    fn denied_delegation_is_replay_safe_for_the_exact_typed_target() -> Result<(), Box<dyn Error>> {
        let attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation =
            audit.reserve_delegation(attribution.clone(), AgentId::WorkflowAutomation)?;
        audit.commit_delegation_denied(
            reservation,
            DelegationMatrixOutcome::Denied,
            AgentGovernanceErrorCode::DelegationDenied,
        );
        assert!(matches!(
            audit.reserve_delegation(attribution, AgentId::WorkflowAutomation),
            Err(AgentGovernanceError::DuplicateSubject)
        ));

        let records = audit.records();
        let AgentGovernanceRecord::Delegation(record) = &records[0] else {
            return Err("expected a delegation record".into());
        };
        assert_eq!(record.target_agent_id(), AgentId::WorkflowAutomation);
        assert_eq!(record.matrix(), DelegationMatrixOutcome::Denied);
        assert_eq!(record.control(), AgentControlResult::NotCreated);
        assert_eq!(
            record.lifecycle(),
            AgentDelegationGovernanceLifecycleState::Denied
        );
        Ok(())
    }

    #[test]
    fn capacity_counts_reserved_and_committed_subjects() -> Result<(), Box<dyn Error>> {
        let attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();
        for index in 0..MAX_AGENT_GOVERNANCE_RECORDS {
            let reservation =
                audit.reserve_tool(attribution.clone(), format!("capacity-call-{index}"))?;
            if index % 2 == 0 {
                audit.commit_tool_policy_evaluated(
                    reservation,
                    AgentGovernanceAction::GetCurrentDatetime,
                    allow_policy(),
                    None,
                );
            }
        }
        assert_eq!(audit.record_count(), MAX_AGENT_GOVERNANCE_RECORDS);
        assert!(matches!(
            audit.reserve_tool(attribution, "over-capacity"),
            Err(AgentGovernanceError::AuditCapacityExhausted {
                maximum: MAX_AGENT_GOVERNANCE_RECORDS
            })
        ));
        Ok(())
    }

    #[test]
    fn terminal_updates_need_no_capacity_after_all_slots_are_occupied() -> Result<(), Box<dyn Error>>
    {
        let attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let pending = audit.reserve_tool(attribution.clone(), "pending-at-capacity")?;
        let pending = audit.commit_tool_approval_pending(
            pending,
            AgentGovernanceAction::CreateLocalTask,
            approval_policy(),
        );
        for index in 1..MAX_AGENT_GOVERNANCE_RECORDS {
            let reservation =
                audit.reserve_tool(attribution.clone(), format!("capacity-terminal-{index}"))?;
            audit.commit_tool_policy_evaluated(
                reservation,
                AgentGovernanceAction::GetCurrentDatetime,
                allow_policy(),
                None,
            );
        }
        assert_eq!(audit.record_count(), MAX_AGENT_GOVERNANCE_RECORDS);

        audit.resolve_tool_approval(pending, AgentApprovalAuditDisposition::Cancelled, None);
        let records = audit.records();
        let AgentGovernanceRecord::Tool(record) = &records[0] else {
            return Err("expected a tool record".into());
        };
        assert_eq!(
            record.lifecycle(),
            AgentToolGovernanceLifecycleState::ApprovalResolved
        );
        assert_eq!(record.approval(), AgentApprovalAuditDisposition::Cancelled);
        assert_eq!(audit.record_count(), MAX_AGENT_GOVERNANCE_RECORDS);
        Ok(())
    }

    #[test]
    fn replay_key_uses_full_attribution_and_private_call_id() -> Result<(), Box<dyn Error>> {
        let first_attribution = attribution()?;
        let second_attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();

        let first = audit.reserve_tool(first_attribution.clone(), "shared-call")?;
        audit.commit_tool_policy_evaluated(
            first,
            AgentGovernanceAction::GetCurrentDatetime,
            allow_policy(),
            None,
        );
        let different_call = audit.reserve_tool(first_attribution, "different-call")?;
        audit.commit_tool_policy_evaluated(
            different_call,
            AgentGovernanceAction::GetCurrentDatetime,
            allow_policy(),
            None,
        );
        let different_attribution = audit.reserve_tool(second_attribution, "shared-call")?;
        audit.commit_tool_policy_evaluated(
            different_attribution,
            AgentGovernanceAction::GetCurrentDatetime,
            allow_policy(),
            None,
        );

        assert_eq!(audit.record_count(), 3);
        Ok(())
    }

    #[test]
    fn snapshots_remain_in_sequence_order_after_an_aborted_slot_is_reused(
    ) -> Result<(), Box<dyn Error>> {
        let attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let aborted = audit.reserve_tool(attribution.clone(), "aborted-first")?;
        let second = audit.reserve_tool(attribution.clone(), "committed-second")?;
        audit.abort_tool(aborted);
        audit.commit_tool_policy_evaluated(
            second,
            AgentGovernanceAction::GetCurrentDatetime,
            allow_policy(),
            None,
        );
        let reused = audit.reserve_tool(attribution, "committed-third")?;
        audit.commit_tool_policy_evaluated(
            reused,
            AgentGovernanceAction::GetCurrentDatetime,
            allow_policy(),
            None,
        );

        let sequences: Vec<_> = audit
            .records()
            .iter()
            .map(|record| record.sequence().value())
            .collect();
        assert_eq!(sequences, vec![2, 3]);
        Ok(())
    }

    #[test]
    fn nonterminal_resolution_input_fails_closed_in_the_existing_slot() -> Result<(), Box<dyn Error>>
    {
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation = audit.reserve_tool(attribution()?, "invalid-resolution")?;
        let token = audit.commit_tool_approval_pending(
            reservation,
            AgentGovernanceAction::CreateLocalTask,
            approval_policy(),
        );
        audit.resolve_tool_approval(token, AgentApprovalAuditDisposition::Pending, None);

        let records = audit.records();
        let AgentGovernanceRecord::Tool(record) = &records[0] else {
            return Err("expected a tool record".into());
        };
        assert_eq!(record.approval(), AgentApprovalAuditDisposition::Cancelled);
        assert_eq!(
            record.error(),
            Some(AgentGovernanceErrorCode::ApprovalResolutionFailed)
        );
        Ok(())
    }

    #[derive(Debug)]
    struct SaturatedClock {
        tick: u64,
    }

    impl AgentGovernanceClock for SaturatedClock {
        fn next_tick(&mut self) -> AgentGovernanceTick {
            self.tick = self.tick.saturating_add(1);
            AgentGovernanceTick(self.tick)
        }
    }

    #[test]
    fn logical_tick_saturation_does_not_block_terminal_update() -> Result<(), Box<dyn Error>> {
        let mut audit =
            InMemoryAgentGovernanceAudit::with_clock(SaturatedClock { tick: u64::MAX - 1 });
        let reservation = audit.reserve_tool(attribution()?, "saturated-call")?;
        let token = audit.commit_tool_approval_pending(
            reservation,
            AgentGovernanceAction::CreateLocalTask,
            approval_policy(),
        );
        audit.resolve_tool_approval(token, AgentApprovalAuditDisposition::Cancelled, None);
        let records = audit.records();
        let AgentGovernanceRecord::Tool(record) = &records[0] else {
            return Err("expected a tool record".into());
        };
        assert_eq!(record.created_tick().value(), u64::MAX);
        assert_eq!(record.updated_tick().value(), u64::MAX);
        assert_eq!(
            record.lifecycle(),
            AgentToolGovernanceLifecycleState::ApprovalResolved
        );
        Ok(())
    }

    #[test]
    fn sequence_exhaustion_rejects_reservation_without_mutation() -> Result<(), Box<dyn Error>> {
        let mut audit = InMemoryAgentGovernanceAudit::new();
        audit.next_sequence = u64::MAX;
        assert_eq!(
            audit
                .reserve_tool(attribution()?, "sequence-exhausted")
                .err(),
            Some(AgentGovernanceError::AuditSequenceExhausted)
        );
        assert_eq!(audit.record_count(), 0);
        Ok(())
    }

    #[test]
    fn debug_surfaces_redact_call_and_attribution_identity() -> Result<(), Box<dyn Error>> {
        let call_id = "audit-secret-call-id";
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation = audit.reserve_tool(attribution()?, call_id)?;
        audit.commit_tool_validation_rejected(
            reservation,
            AgentGovernanceAction::UnknownTool,
            AgentGovernanceErrorCode::UnknownTool,
        );

        let debug = format!("{audit:?} {:?}", audit.records());
        assert!(!debug.contains(call_id));
        assert!(!debug.contains("agent-task-root"));
        assert!(!debug.contains("runtime-run"));
        assert!(!debug.contains("runtime-request"));
        Ok(())
    }

    #[test]
    fn aborting_delegation_frees_its_exact_replay_subject() -> Result<(), Box<dyn Error>> {
        let attribution = attribution()?;
        let mut audit = InMemoryAgentGovernanceAudit::new();
        let reservation = audit.reserve_delegation(attribution.clone(), AgentId::Research)?;
        audit.abort_delegation(reservation);
        assert_eq!(audit.record_count(), 0);

        let retried = audit.reserve_delegation(attribution, AgentId::Research)?;
        audit.commit_delegation_denied(
            retried,
            DelegationMatrixOutcome::Denied,
            AgentGovernanceErrorCode::DelegationDenied,
        );
        assert_eq!(audit.records().len(), 1);
        Ok(())
    }
}
