use std::fmt;
#[cfg(target_os = "macos")]
use std::sync::Arc;
use std::time::{Duration, Instant};

use thiserror::Error;

#[cfg(target_os = "macos")]
use super::decision_source::{TrustedApprovalSourceOutcome, TrustedSourceDecision};
use super::types::ApprovalDecision;
#[cfg(target_os = "macos")]
use super::types::ApprovalInteractionSource;
use super::types::{
    ApprovalAction, ApprovalCancellationReason, ApprovalDisposition, ApprovalId,
    ApprovalInteractionEvidence, ApprovalOrigin, ApprovalPreview, ApprovalRecipients,
    ApprovalRequestView, ApprovalResolution, ApprovalReversibility, ApprovalRisk, ApprovalSchedule,
    ApprovalTarget,
};
use crate::agent::governance::{AgentAttribution, AgentPolicyDecision, AgentPolicyReason};
use crate::policy::types::{PolicyDecision, PolicyOutcome, PolicyReason};
use crate::tools::types::{PermissionKind, RiskClass};

pub const APPROVAL_TTL: Duration = Duration::from_secs(120);
pub const MAX_PENDING_APPROVALS: usize = 1;
pub const MAX_APPROVAL_SUBJECTS_PER_MANAGER: usize = 1_024;

pub type ApprovalResult<T> = Result<T, ApprovalError>;

pub trait ApprovalManager {
    fn create_request(&mut self, decision: PolicyDecision) -> ApprovalResult<ApprovalId>;
    fn create_agent_request(&mut self, decision: AgentPolicyDecision)
        -> ApprovalResult<ApprovalId>;
    fn pending(&self) -> ApprovalResult<Option<ApprovalRequestView<'_>>>;
    fn issue_presentation(&mut self, id: ApprovalId) -> ApprovalResult<ApprovalPresentation>;
    #[cfg(target_os = "macos")]
    fn resolve_source_outcome(
        &mut self,
        outcome: TrustedApprovalSourceOutcome,
    ) -> ApprovalResult<ApprovalResolution>;
    fn cancel_for_run_termination(&mut self, id: ApprovalId) -> ApprovalResult<ApprovalResolution>;
    fn expire_due(&mut self) -> Option<ApprovalResolution>;
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum ApprovalError {
    #[cfg(test)]
    #[error("test-only approval manager failure")]
    TestOnlyFailure,
    #[error("policy outcome is not eligible for approval")]
    IneligiblePolicyOutcome { actual: PolicyOutcome },
    #[error("policy decision has no registered approval preview")]
    UnsupportedApprovalSubject,
    #[error("an approval request is already pending")]
    PendingApprovalExists,
    #[error("approval subject has already been consumed")]
    DuplicateSubject,
    #[error("approval subject capacity is exhausted")]
    SubjectCapacityExhausted { maximum: usize },
    #[error("approval not found: {0}")]
    NotFound(u64),
    #[error("approval has already been consumed: {0}")]
    AlreadyConsumed(u64),
    #[error("approval id space is exhausted")]
    IdSpaceExhausted,
    #[error("approval deadline could not be calculated")]
    DeadlineOverflow,
    #[error("approval presentation has already been issued: {0}")]
    PresentationAlreadyIssued(u64),
    #[error("approval presentation is unavailable or expired: {0}")]
    PresentationUnavailableOrExpired(u64),
    #[error("approval source outcome belongs to another manager")]
    ManagerInstanceMismatch,
    #[error("approval source kind does not match the registered source")]
    SourceKindMismatch,
    #[error("approval source outcome identity does not match the pending subject")]
    SourceOutcomeIdentityMismatch,
}

#[cfg(target_os = "macos")]
pub(super) struct ApprovalManagerInstanceMarker {
    _owned: u8,
}

#[cfg(target_os = "macos")]
impl ApprovalManagerInstanceMarker {
    fn new() -> Self {
        Self { _owned: 0 }
    }
}

pub struct ApprovalPresentation {
    id: ApprovalId,
    #[cfg(target_os = "macos")]
    manager_instance: Arc<ApprovalManagerInstanceMarker>,
    origin: ApprovalOrigin,
    run_id: String,
    gateway_request_id: String,
    call_id: String,
    tool_name: String,
    tool_contract_version: u16,
    policy_outcome: PolicyOutcome,
    policy_reason: PolicyReason,
    agent_policy_reason: Option<AgentPolicyReason>,
    risk_class: RiskClass,
    required_permission: PermissionKind,
    action: ApprovalAction,
    target: ApprovalTarget,
    schedule: ApprovalSchedule,
    recipients: ApprovalRecipients,
    reversibility: ApprovalReversibility,
    risk: ApprovalRisk,
    title: String,
    remaining: Duration,
}

impl ApprovalPresentation {
    #[must_use]
    pub fn id(&self) -> ApprovalId {
        self.id
    }

    #[must_use]
    pub fn origin(&self) -> &ApprovalOrigin {
        &self.origin
    }

    #[must_use]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    #[must_use]
    pub fn gateway_request_id(&self) -> &str {
        &self.gateway_request_id
    }

    #[must_use]
    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    #[must_use]
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    #[must_use]
    pub fn tool_contract_version(&self) -> u16 {
        self.tool_contract_version
    }

    #[must_use]
    pub fn policy_outcome(&self) -> PolicyOutcome {
        self.policy_outcome
    }

    #[must_use]
    pub fn policy_reason(&self) -> PolicyReason {
        self.policy_reason
    }

    #[must_use]
    pub fn agent_policy_reason(&self) -> Option<AgentPolicyReason> {
        self.agent_policy_reason
    }

    #[must_use]
    pub fn risk_class(&self) -> RiskClass {
        self.risk_class
    }

    #[must_use]
    pub fn required_permission(&self) -> PermissionKind {
        self.required_permission
    }

    #[must_use]
    pub fn preview(&self) -> ApprovalPreview<'_> {
        ApprovalPreview::CreateLocalTask { title: &self.title }
    }

    #[must_use]
    pub fn remaining(&self) -> Duration {
        self.remaining
    }

    #[cfg(target_os = "macos")]
    pub(super) fn into_source_parts(self) -> ApprovalPresentationParts {
        ApprovalPresentationParts {
            id: self.id,
            manager_instance: self.manager_instance,
            origin: self.origin,
            run_id: self.run_id,
            gateway_request_id: self.gateway_request_id,
            call_id: self.call_id,
            tool_name: self.tool_name,
            tool_contract_version: self.tool_contract_version,
            policy_outcome: self.policy_outcome,
            policy_reason: self.policy_reason,
            agent_policy_reason: self.agent_policy_reason,
            risk_class: self.risk_class,
            required_permission: self.required_permission,
            action: self.action,
            target: self.target,
            schedule: self.schedule,
            recipients: self.recipients,
            reversibility: self.reversibility,
            risk: self.risk,
            title: self.title,
            remaining: self.remaining,
        }
    }
}

impl fmt::Debug for ApprovalPresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovalPresentation")
            .field("id", &self.id)
            .field("origin", &self.origin)
            .field("identity", &"[REDACTED]")
            .field("tool_name", &self.tool_name)
            .field("tool_contract_version", &self.tool_contract_version)
            .field("policy_outcome", &self.policy_outcome)
            .field("policy_reason", &self.policy_reason)
            .field("risk_class", &self.risk_class)
            .field("required_permission", &self.required_permission)
            .field("action", &self.action)
            .field("target", &self.target)
            .field("schedule", &self.schedule)
            .field("recipients", &self.recipients)
            .field("reversibility", &self.reversibility)
            .field("risk", &self.risk)
            .field("title", &"[REDACTED]")
            .field("remaining", &self.remaining)
            .finish()
    }
}

#[cfg(target_os = "macos")]
pub(super) struct ApprovalPresentationParts {
    pub(super) id: ApprovalId,
    pub(super) manager_instance: Arc<ApprovalManagerInstanceMarker>,
    pub(super) origin: ApprovalOrigin,
    pub(super) run_id: String,
    pub(super) gateway_request_id: String,
    pub(super) call_id: String,
    pub(super) tool_name: String,
    pub(super) tool_contract_version: u16,
    pub(super) policy_outcome: PolicyOutcome,
    pub(super) policy_reason: PolicyReason,
    pub(super) agent_policy_reason: Option<AgentPolicyReason>,
    pub(super) risk_class: RiskClass,
    pub(super) required_permission: PermissionKind,
    pub(super) action: ApprovalAction,
    pub(super) target: ApprovalTarget,
    pub(super) schedule: ApprovalSchedule,
    pub(super) recipients: ApprovalRecipients,
    pub(super) reversibility: ApprovalReversibility,
    pub(super) risk: ApprovalRisk,
    pub(super) title: String,
    pub(super) remaining: Duration,
}

trait ApprovalClock: fmt::Debug {
    fn now(&self) -> Instant;

    fn checked_add(&self, instant: Instant, duration: Duration) -> Option<Instant> {
        instant.checked_add(duration)
    }
}

#[derive(Debug)]
struct SystemApprovalClock;

impl ApprovalClock for SystemApprovalClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

#[derive(Clone, Eq, PartialEq)]
enum ApprovalSubjectKey {
    LegacyGateway {
        run_id: String,
        gateway_request_id: String,
        call_id: String,
    },
    Agent {
        attribution: AgentAttribution,
        call_id: String,
    },
}

impl ApprovalSubjectKey {
    fn from_decision(decision: &ApprovalDecision) -> Self {
        match decision {
            ApprovalDecision::LegacyGateway(decision) => {
                let call = decision.validated_call();
                Self::LegacyGateway {
                    run_id: call.run_id().to_owned(),
                    gateway_request_id: call.gateway_request_id().to_owned(),
                    call_id: call.call_id().to_owned(),
                }
            }
            ApprovalDecision::Agent { decision, .. } => Self::Agent {
                attribution: decision.request().attribution().clone(),
                call_id: decision.request().call_id().to_owned(),
            },
        }
    }

    fn matches_source(
        &self,
        origin: &ApprovalOrigin,
        run_id: &str,
        gateway_request_id: &str,
        call_id: &str,
    ) -> bool {
        match (self, origin) {
            (
                Self::LegacyGateway {
                    run_id: expected_run_id,
                    gateway_request_id: expected_request_id,
                    call_id: expected_call_id,
                },
                ApprovalOrigin::LegacyGateway,
            ) => {
                expected_run_id == run_id
                    && expected_request_id == gateway_request_id
                    && expected_call_id == call_id
            }
            (
                Self::Agent {
                    attribution,
                    call_id: expected_call_id,
                },
                ApprovalOrigin::Agent(source_attribution),
            ) => {
                attribution == source_attribution
                    && attribution.runtime_run_identity().run_id().as_str() == run_id
                    && attribution.runtime_run_identity().request_id().as_str()
                        == gateway_request_id
                    && expected_call_id == call_id
            }
            (Self::LegacyGateway { .. }, ApprovalOrigin::Agent(_))
            | (Self::Agent { .. }, ApprovalOrigin::LegacyGateway) => false,
        }
    }
}

struct PendingApproval {
    id: ApprovalId,
    subject: ApprovalSubjectKey,
    decision: ApprovalDecision,
    deadline: Instant,
    presentation_issued: bool,
}

pub struct InMemoryApprovalManager {
    next_id: u64,
    pending: Option<PendingApproval>,
    consumed_subjects: Vec<ApprovalSubjectKey>,
    clock: Box<dyn ApprovalClock>,
    #[cfg(test)]
    fail_next_cancel: bool,
    #[cfg(target_os = "macos")]
    manager_instance: Arc<ApprovalManagerInstanceMarker>,
}

impl InMemoryApprovalManager {
    #[must_use]
    pub fn new() -> Self {
        Self::with_clock(SystemApprovalClock)
    }

    fn with_clock(clock: impl ApprovalClock + 'static) -> Self {
        Self {
            next_id: 0,
            pending: None,
            consumed_subjects: Vec::new(),
            clock: Box::new(clock),
            #[cfg(test)]
            fail_next_cancel: false,
            #[cfg(target_os = "macos")]
            manager_instance: Arc::new(ApprovalManagerInstanceMarker::new()),
        }
    }

    #[cfg(test)]
    pub(crate) fn force_pending_due_for_test(&mut self) {
        if let Some(pending) = self.pending.as_mut() {
            pending.deadline = self.clock.now();
        }
    }

    #[cfg(test)]
    pub(crate) fn fail_next_cancel_for_test(&mut self) {
        self.fail_next_cancel = true;
    }

    fn resolve_by_id(
        &mut self,
        id: ApprovalId,
        requested_disposition: ApprovalDisposition,
        interaction_evidence: Option<ApprovalInteractionEvidence>,
    ) -> ApprovalResult<ApprovalResolution> {
        let (disposition, interaction_evidence) = {
            let Some(pending) = self.pending.as_ref() else {
                return Err(self.error_for_missing_id(id));
            };
            if pending.id != id {
                return Err(self.error_for_missing_id(id));
            }

            if self.clock.now() >= pending.deadline {
                (ApprovalDisposition::Expired, None)
            } else {
                (requested_disposition, interaction_evidence)
            }
        };

        let Some(pending) = self.pending.take() else {
            return Err(self.error_for_missing_id(id));
        };
        self.consumed_subjects.push(pending.subject);
        Ok(ApprovalResolution::from_approval_decision(
            pending.id,
            disposition,
            pending.decision,
            interaction_evidence,
        ))
    }

    fn error_for_missing_id(&self, id: ApprovalId) -> ApprovalError {
        if id.value() > 0 && id.value() <= self.next_id {
            ApprovalError::AlreadyConsumed(id.value())
        } else {
            ApprovalError::NotFound(id.value())
        }
    }
}

impl Default for InMemoryApprovalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for InMemoryApprovalManager {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryApprovalManager")
            .field("next_id", &self.next_id)
            .field("has_pending", &self.pending.is_some())
            .field("consumed_subject_count", &self.consumed_subjects.len())
            .finish()
    }
}

impl ApprovalManager for InMemoryApprovalManager {
    fn create_request(&mut self, decision: PolicyDecision) -> ApprovalResult<ApprovalId> {
        self.create_closed_request(ApprovalDecision::legacy(decision))
    }

    fn create_agent_request(
        &mut self,
        decision: AgentPolicyDecision,
    ) -> ApprovalResult<ApprovalId> {
        let actual = decision.outcome();
        if actual != PolicyOutcome::RequireApproval {
            return Err(ApprovalError::IneligiblePolicyOutcome { actual });
        }
        let decision =
            ApprovalDecision::agent(decision).ok_or(ApprovalError::UnsupportedApprovalSubject)?;
        self.create_closed_request(decision)
    }

    fn pending(&self) -> ApprovalResult<Option<ApprovalRequestView<'_>>> {
        let Some(pending) = self.pending.as_ref() else {
            return Ok(None);
        };
        let now = self.clock.now();
        if now >= pending.deadline {
            return Ok(None);
        }

        ApprovalRequestView::from_approval_decision(
            pending.id,
            &pending.decision,
            pending.deadline.saturating_duration_since(now),
        )
        .map(Some)
        .ok_or(ApprovalError::UnsupportedApprovalSubject)
    }

    fn issue_presentation(&mut self, id: ApprovalId) -> ApprovalResult<ApprovalPresentation> {
        let now = self.clock.now();
        let Some(pending) = self.pending.as_ref() else {
            return Err(self.error_for_missing_id(id));
        };
        if pending.id != id {
            return Err(self.error_for_missing_id(id));
        }
        if now >= pending.deadline {
            let Some(expired) = self.pending.take() else {
                return Err(self.error_for_missing_id(id));
            };
            self.consumed_subjects.push(expired.subject);
            return Err(ApprovalError::PresentationUnavailableOrExpired(id.value()));
        }
        if pending.presentation_issued {
            return Err(ApprovalError::PresentationAlreadyIssued(id.value()));
        }

        let preview = ApprovalPreview::from_approval_decision(&pending.decision)
            .ok_or(ApprovalError::UnsupportedApprovalSubject)?;
        let presentation = ApprovalPresentation {
            id,
            #[cfg(target_os = "macos")]
            manager_instance: Arc::clone(&self.manager_instance),
            origin: pending.decision.origin(),
            run_id: pending.decision.run_id().to_owned(),
            gateway_request_id: pending.decision.gateway_request_id().to_owned(),
            call_id: pending.decision.call_id().to_owned(),
            tool_name: pending.decision.tool_name().to_owned(),
            tool_contract_version: pending.decision.tool_contract_version(),
            policy_outcome: pending.decision.outcome(),
            policy_reason: pending.decision.policy_reason(),
            agent_policy_reason: pending.decision.agent_policy_reason(),
            risk_class: pending.decision.risk_class(),
            required_permission: pending.decision.required_permission(),
            action: preview.action(),
            target: preview.target(),
            schedule: preview.schedule(),
            recipients: preview.recipients(),
            reversibility: preview.reversibility(),
            risk: preview.risk(),
            title: preview.affected_data().value().to_owned(),
            remaining: pending.deadline.saturating_duration_since(now),
        };

        let Some(pending) = self.pending.as_mut() else {
            return Err(self.error_for_missing_id(id));
        };
        pending.presentation_issued = true;
        Ok(presentation)
    }

    #[cfg(target_os = "macos")]
    fn resolve_source_outcome(
        &mut self,
        outcome: TrustedApprovalSourceOutcome,
    ) -> ApprovalResult<ApprovalResolution> {
        let outcome = outcome.into_parts();
        let Some(pending) = self.pending.as_ref() else {
            return Err(self.error_for_missing_id(outcome.id));
        };

        if !Arc::ptr_eq(&self.manager_instance, &outcome.manager_instance) {
            return Err(ApprovalError::ManagerInstanceMismatch);
        }
        if outcome.source != ApprovalInteractionSource::MacOsNativeDialog {
            return Err(ApprovalError::SourceKindMismatch);
        }
        if pending.id != outcome.id
            || !pending.subject.matches_source(
                &outcome.origin,
                &outcome.run_id,
                &outcome.gateway_request_id,
                &outcome.call_id,
            )
        {
            return Err(ApprovalError::SourceOutcomeIdentityMismatch);
        }
        if !pending.presentation_issued {
            return Err(ApprovalError::PresentationUnavailableOrExpired(
                outcome.id.value(),
            ));
        }

        if self.clock.now() >= pending.deadline {
            return self.resolve_by_id(outcome.id, ApprovalDisposition::Expired, None);
        }

        let (disposition, evidence) = match outcome.decision {
            TrustedSourceDecision::RecognizedButton(button) => {
                let disposition = match button {
                    super::types::ApprovalNativeButton::Approve => ApprovalDisposition::Approved,
                    super::types::ApprovalNativeButton::Reject => ApprovalDisposition::Rejected,
                    super::types::ApprovalNativeButton::Edit => {
                        ApprovalDisposition::Cancelled(ApprovalCancellationReason::EditRequested)
                    }
                };
                (
                    disposition,
                    ApprovalInteractionEvidence::recognized_button(
                        outcome.source,
                        button,
                        outcome.authentication,
                    ),
                )
            }
            TrustedSourceDecision::NativeNoDecision => (
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::NativeNoDecision),
                ApprovalInteractionEvidence::no_decision(outcome.source, outcome.authentication),
            ),
            TrustedSourceDecision::SourceFailed(failure) => (
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed),
                ApprovalInteractionEvidence::source_failed(
                    outcome.source,
                    outcome.authentication,
                    failure,
                ),
            ),
        };

        self.resolve_by_id(outcome.id, disposition, Some(evidence))
    }

    fn cancel_for_run_termination(&mut self, id: ApprovalId) -> ApprovalResult<ApprovalResolution> {
        #[cfg(test)]
        if self.fail_next_cancel {
            self.fail_next_cancel = false;
            return Err(ApprovalError::TestOnlyFailure);
        }
        self.resolve_by_id(
            id,
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated),
            None,
        )
    }

    fn expire_due(&mut self) -> Option<ApprovalResolution> {
        let now = self.clock.now();
        let should_expire = self
            .pending
            .as_ref()
            .is_some_and(|pending| now >= pending.deadline);
        if !should_expire {
            return None;
        }

        let pending = self.pending.take()?;
        self.consumed_subjects.push(pending.subject);
        Some(ApprovalResolution::from_approval_decision(
            pending.id,
            ApprovalDisposition::Expired,
            pending.decision,
            None,
        ))
    }
}

impl InMemoryApprovalManager {
    fn create_closed_request(&mut self, decision: ApprovalDecision) -> ApprovalResult<ApprovalId> {
        let actual = decision.outcome();
        if actual != PolicyOutcome::RequireApproval {
            return Err(ApprovalError::IneligiblePolicyOutcome { actual });
        }
        if ApprovalPreview::from_approval_decision(&decision).is_none() {
            return Err(ApprovalError::UnsupportedApprovalSubject);
        }

        let subject = ApprovalSubjectKey::from_decision(&decision);
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.subject == subject)
            || self.consumed_subjects.contains(&subject)
        {
            return Err(ApprovalError::DuplicateSubject);
        }
        if self.pending.is_some() {
            return Err(ApprovalError::PendingApprovalExists);
        }
        if self.consumed_subjects.len() >= MAX_APPROVAL_SUBJECTS_PER_MANAGER {
            return Err(ApprovalError::SubjectCapacityExhausted {
                maximum: MAX_APPROVAL_SUBJECTS_PER_MANAGER,
            });
        }

        let Some(next_id) = self.next_id.checked_add(1) else {
            return Err(ApprovalError::IdSpaceExhausted);
        };
        let now = self.clock.now();
        let Some(deadline) = self.clock.checked_add(now, APPROVAL_TTL) else {
            return Err(ApprovalError::DeadlineOverflow);
        };

        let id = ApprovalId::new(next_id);
        self.next_id = next_id;
        self.pending = Some(PendingApproval {
            id,
            subject,
            decision,
            deadline,
            presentation_issued: false,
        });
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::error::Error;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

    use serde_json::json;

    #[cfg(target_os = "macos")]
    use rfd::MessageDialogResult;

    use super::{
        ApprovalClock, ApprovalError, ApprovalManager, ApprovalSubjectKey, InMemoryApprovalManager,
        APPROVAL_TTL, MAX_APPROVAL_SUBJECTS_PER_MANAGER, MAX_PENDING_APPROVALS,
    };
    use crate::agent::function_call_validation::validate_function_call;
    use crate::agent::gateway_protocol::{
        GatewayProtocolError, GatewayStreamValidator, ValidatedGatewayEvent,
        GATEWAY_PROTOCOL_VERSION,
    };
    #[cfg(target_os = "macos")]
    use crate::approvals::decision_source::test_outcome_from_dialog_result;
    use crate::approvals::types::{
        ApprovalAction, ApprovalCancellationReason, ApprovalDisposition, ApprovalId,
        ApprovalPreview, ApprovalRecipients, ApprovalReversibility, ApprovalRisk, ApprovalSchedule,
        ApprovalTarget,
    };
    use crate::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
    use crate::policy::types::{PolicyDecision, PolicyInput, PolicyOutcome, PolicyReason};
    use crate::tools::registry::{InMemoryToolRegistry, ToolRegistry, ToolRegistryResult};
    use crate::tools::types::{PermissionKind, RiskClass, ToolDefinition, ToolSchema};

    const TOOL_CONTRACT_VERSION: u16 = 1;

    #[derive(Clone, Debug)]
    struct TestClock {
        now: Rc<Cell<Instant>>,
    }

    impl TestClock {
        fn new() -> Self {
            Self {
                now: Rc::new(Cell::new(Instant::now())),
            }
        }

        fn advance(&self, duration: Duration) -> bool {
            let Some(next) = self.now.get().checked_add(duration) else {
                return false;
            };
            self.now.set(next);
            true
        }
    }

    impl ApprovalClock for TestClock {
        fn now(&self) -> Instant {
            self.now.get()
        }
    }

    #[derive(Debug)]
    struct OverflowClock {
        now: Instant,
    }

    impl ApprovalClock for OverflowClock {
        fn now(&self) -> Instant {
            self.now
        }

        fn checked_add(&self, _instant: Instant, _duration: Duration) -> Option<Instant> {
            None
        }
    }

    fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
        let mut registry = InMemoryToolRegistry::new();
        registry.register(ToolDefinition::from_schema(
            ToolSchema::GetCurrentDatetimeV1,
        ))?;
        registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
        Ok(registry)
    }

    fn decision(
        run_id: &str,
        gateway_request_id: &str,
        call_id: &str,
        name: &str,
        arguments_json: &str,
    ) -> Result<PolicyDecision, Box<dyn Error>> {
        let mut validator = GatewayStreamValidator::new(
            run_id,
            gateway_request_id,
            [name.to_owned()],
            TOOL_CONTRACT_VERSION,
        )?;
        let start_frame = json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": run_id,
            "gateway_request_id": gateway_request_id,
            "sequence": 0,
            "event": {
                "type": "response_started",
                "provider_response_id": "provider-response-approval-unit-1",
            },
        });
        let function_frame = json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": run_id,
            "gateway_request_id": gateway_request_id,
            "sequence": 1,
            "event": {
                "type": "function_call_completed",
                "call_id": call_id,
                "name": name,
                "tool_contract_version": TOOL_CONTRACT_VERSION,
                "arguments_json": arguments_json,
            },
        });

        let _ = validator.accept_frame(&serde_json::to_vec(&start_frame)?)?;
        let call = match validator.accept_frame(&serde_json::to_vec(&function_frame)?)? {
            ValidatedGatewayEvent::FunctionCallCompleted { call } => call,
            _ => return Err(GatewayProtocolError::MalformedEvent.into()),
        };
        let validated = validate_function_call(call, &registry()?)?;
        Ok(DeterministicPolicyEngine::new().evaluate(PolicyInput::from_validated_call(validated)))
    }

    fn local_task_decision(
        run_id: &str,
        gateway_request_id: &str,
        call_id: &str,
        title: &str,
    ) -> Result<PolicyDecision, Box<dyn Error>> {
        decision(
            run_id,
            gateway_request_id,
            call_id,
            "create_local_task",
            &json!({ "title": title }).to_string(),
        )
    }

    fn information_only_decision() -> Result<PolicyDecision, Box<dyn Error>> {
        decision(
            "run-approval-information-1",
            "gateway-request-approval-information-1",
            "call-approval-information-1",
            "get_current_datetime",
            "{}",
        )
    }

    #[test]
    fn creates_one_exact_typed_preview() -> Result<(), Box<dyn Error>> {
        assert_eq!(MAX_PENDING_APPROVALS, 1);
        let title = "Review approval plan";
        let mut manager = InMemoryApprovalManager::new();
        let id = manager.create_request(local_task_decision(
            "run-approval-create-1",
            "gateway-request-approval-create-1",
            "call-approval-create-1",
            title,
        )?)?;

        assert_eq!(id.value(), 1);
        let Some(view) = manager.pending()? else {
            return Err(ApprovalError::NotFound(id.value()).into());
        };
        assert_eq!(view.id(), id);
        assert_eq!(view.run_id(), "run-approval-create-1");
        assert_eq!(
            view.gateway_request_id(),
            "gateway-request-approval-create-1"
        );
        assert_eq!(view.call_id(), "call-approval-create-1");
        assert_eq!(view.tool_name(), "create_local_task");
        assert_eq!(view.tool_contract_version(), TOOL_CONTRACT_VERSION);
        assert_eq!(view.policy_outcome(), PolicyOutcome::RequireApproval);
        assert_eq!(
            view.policy_reason(),
            PolicyReason::ReversibleRequiresApproval
        );
        assert_eq!(view.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(view.required_permission(), PermissionKind::None);
        assert!(view.remaining() <= APPROVAL_TTL);
        assert!(view.remaining() > Duration::ZERO);

        let preview = view.preview();
        assert_eq!(preview.action(), ApprovalAction::CreateLocalTask);
        assert_eq!(preview.target(), ApprovalTarget::LocalTaskList);
        assert_eq!(preview.affected_data().value(), title);
        assert_eq!(preview.schedule(), ApprovalSchedule::NotScheduled);
        assert_eq!(preview.recipients(), ApprovalRecipients::None);
        assert_eq!(preview.reversibility(), ApprovalReversibility::Reversible);
        assert_eq!(preview.required_permission(), PermissionKind::None);
        assert_eq!(preview.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(preview.risk(), ApprovalRisk::CreatesLocalTask);

        let presentation = manager.issue_presentation(id)?;
        assert_eq!(presentation.id(), id);
        assert_eq!(presentation.run_id(), "run-approval-create-1");
        assert_eq!(
            presentation.gateway_request_id(),
            "gateway-request-approval-create-1"
        );
        assert_eq!(presentation.call_id(), "call-approval-create-1");
        assert_eq!(presentation.tool_name(), "create_local_task");
        assert_eq!(presentation.tool_contract_version(), TOOL_CONTRACT_VERSION);
        assert_eq!(
            presentation.policy_outcome(),
            PolicyOutcome::RequireApproval
        );
        assert_eq!(
            presentation.policy_reason(),
            PolicyReason::ReversibleRequiresApproval
        );
        assert_eq!(presentation.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(presentation.required_permission(), PermissionKind::None);
        assert_eq!(
            presentation.preview().affected_data().value(),
            "Review approval plan"
        );
        assert!(presentation.remaining() <= APPROVAL_TTL);
        assert!(presentation.remaining() > Duration::ZERO);
        assert_eq!(
            manager.issue_presentation(id).err(),
            Some(ApprovalError::PresentationAlreadyIssued(id.value()))
        );

        assert_eq!(
            manager.create_request(local_task_decision(
                "run-approval-create-1",
                "gateway-request-approval-create-1",
                "call-approval-create-1",
                "Changed pending task",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );
        assert_eq!(
            manager.create_request(local_task_decision(
                "run-approval-create-2",
                "gateway-request-approval-create-2",
                "call-approval-create-2",
                "Second task",
            )?),
            Err(ApprovalError::PendingApprovalExists)
        );
        Ok(())
    }

    #[test]
    fn consumes_run_termination_once() -> Result<(), Box<dyn Error>> {
        let mut manager = InMemoryApprovalManager::new();
        let cancelled_id = manager.create_request(local_task_decision(
            "run-approval-resolve-1",
            "gateway-request-approval-resolve-1",
            "call-approval-resolve-1",
            "Cancelled task",
        )?)?;
        let _presentation = manager.issue_presentation(cancelled_id)?;
        let cancelled = manager.cancel_for_run_termination(cancelled_id)?;

        assert_eq!(
            cancelled.disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );
        assert_eq!(cancelled.interaction_evidence(), None);
        assert_eq!(cancelled.id(), cancelled_id);
        assert_eq!(cancelled.run_id(), "run-approval-resolve-1");
        assert_eq!(
            cancelled.gateway_request_id(),
            "gateway-request-approval-resolve-1"
        );
        assert_eq!(cancelled.call_id(), "call-approval-resolve-1");
        assert_eq!(cancelled.tool_name(), "create_local_task");
        assert_eq!(cancelled.tool_contract_version(), TOOL_CONTRACT_VERSION);
        assert_eq!(cancelled.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(cancelled.required_permission(), PermissionKind::None);
        assert_eq!(cancelled.policy_outcome(), PolicyOutcome::RequireApproval);
        assert_eq!(
            cancelled.policy_reason(),
            PolicyReason::ReversibleRequiresApproval
        );
        let Some(preview) = cancelled.preview() else {
            return Err(ApprovalError::UnsupportedApprovalSubject.into());
        };
        assert_eq!(preview.affected_data().value(), "Cancelled task");
        assert_eq!(
            manager.cancel_for_run_termination(cancelled_id),
            Err(ApprovalError::AlreadyConsumed(cancelled_id.value()))
        );
        assert_eq!(
            manager.create_request(local_task_decision(
                "run-approval-resolve-1",
                "gateway-request-approval-resolve-1",
                "call-approval-resolve-1",
                "Changed replay task",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );

        let unpresented_id = manager.create_request(local_task_decision(
            "run-approval-resolve-2",
            "gateway-request-approval-resolve-2",
            "call-approval-resolve-2",
            "Unpresented cancelled task",
        )?)?;
        assert_eq!(
            manager
                .cancel_for_run_termination(unpresented_id)?
                .disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );
        assert_eq!(
            manager.cancel_for_run_termination(ApprovalId::new(404)),
            Err(ApprovalError::NotFound(404))
        );
        Ok(())
    }

    #[test]
    fn expiry_wins_immediately_before_at_and_after_deadline() -> Result<(), Box<dyn Error>> {
        let before_clock = TestClock::new();
        let mut before_manager = InMemoryApprovalManager::with_clock(before_clock.clone());
        let before_id = before_manager.create_request(local_task_decision(
            "run-approval-before-1",
            "gateway-request-approval-before-1",
            "call-approval-before-1",
            "Before deadline",
        )?)?;
        assert!(before_clock.advance(APPROVAL_TTL.saturating_sub(Duration::from_nanos(1))));
        assert!(before_manager.pending()?.is_some());
        assert_eq!(
            before_manager
                .cancel_for_run_termination(before_id)?
                .disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );

        let exact_clock = TestClock::new();
        let mut exact_manager = InMemoryApprovalManager::with_clock(exact_clock.clone());
        let exact_id = exact_manager.create_request(local_task_decision(
            "run-approval-exact-1",
            "gateway-request-approval-exact-1",
            "call-approval-exact-1",
            "Exact deadline",
        )?)?;
        #[cfg(target_os = "macos")]
        let exact_outcome = test_outcome_from_dialog_result(
            exact_manager.issue_presentation(exact_id)?,
            MessageDialogResult::Custom("Approve".to_owned()),
        );
        assert!(exact_clock.advance(APPROVAL_TTL));
        assert!(exact_manager.pending()?.is_none());
        #[cfg(target_os = "macos")]
        assert_eq!(
            exact_manager
                .resolve_source_outcome(exact_outcome)?
                .disposition(),
            ApprovalDisposition::Expired
        );
        #[cfg(not(target_os = "macos"))]
        assert_eq!(
            exact_manager
                .cancel_for_run_termination(exact_id)?
                .disposition(),
            ApprovalDisposition::Expired
        );
        assert_eq!(
            exact_manager.cancel_for_run_termination(exact_id),
            Err(ApprovalError::AlreadyConsumed(exact_id.value()))
        );
        assert_eq!(
            exact_manager.create_request(local_task_decision(
                "run-approval-exact-1",
                "gateway-request-approval-exact-1",
                "call-approval-exact-1",
                "Changed exact-expiry task",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );

        let after_clock = TestClock::new();
        let mut after_manager = InMemoryApprovalManager::with_clock(after_clock.clone());
        let after_id = after_manager.create_request(local_task_decision(
            "run-approval-after-1",
            "gateway-request-approval-after-1",
            "call-approval-after-1",
            "After deadline",
        )?)?;
        let Some(after_duration) = APPROVAL_TTL.checked_add(Duration::from_nanos(1)) else {
            return Err(ApprovalError::DeadlineOverflow.into());
        };
        assert!(after_clock.advance(after_duration));
        let Some(expired) = after_manager.expire_due() else {
            return Err(ApprovalError::NotFound(after_id.value()).into());
        };
        assert_eq!(expired.id(), after_id);
        assert_eq!(expired.disposition(), ApprovalDisposition::Expired);
        assert!(after_manager.expire_due().is_none());
        assert_eq!(
            after_manager.create_request(local_task_decision(
                "run-approval-after-1",
                "gateway-request-approval-after-1",
                "call-approval-after-1",
                "Changed expired task",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );

        let issuance_clock = TestClock::new();
        let mut issuance_manager = InMemoryApprovalManager::with_clock(issuance_clock.clone());
        let issuance_id = issuance_manager.create_request(local_task_decision(
            "run-approval-issuance-expired-1",
            "gateway-request-approval-issuance-expired-1",
            "call-approval-issuance-expired-1",
            "Expired before presentation",
        )?)?;
        assert!(issuance_clock.advance(APPROVAL_TTL));
        assert_eq!(
            issuance_manager.issue_presentation(issuance_id).err(),
            Some(ApprovalError::PresentationUnavailableOrExpired(
                issuance_id.value()
            ))
        );
        assert!(issuance_manager.pending()?.is_none());
        assert_eq!(
            issuance_manager.create_request(local_task_decision(
                "run-approval-issuance-expired-1",
                "gateway-request-approval-issuance-expired-1",
                "call-approval-issuance-expired-1",
                "Changed expired presentation",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );
        Ok(())
    }

    #[test]
    fn rejects_non_approval_policy_and_unsupported_preview() -> Result<(), Box<dyn Error>> {
        let allow = information_only_decision()?;
        assert_eq!(allow.outcome(), PolicyOutcome::Allow);
        assert!(ApprovalPreview::from_policy_decision(&allow).is_none());

        let mut manager = InMemoryApprovalManager::new();
        assert_eq!(
            manager.create_request(allow),
            Err(ApprovalError::IneligiblePolicyOutcome {
                actual: PolicyOutcome::Allow,
            })
        );
        Ok(())
    }

    #[test]
    fn bounds_subjects_ids_and_deadlines() -> Result<(), Box<dyn Error>> {
        let mut capacity_manager = InMemoryApprovalManager::new();
        for index in 0..MAX_APPROVAL_SUBJECTS_PER_MANAGER {
            capacity_manager
                .consumed_subjects
                .push(ApprovalSubjectKey::LegacyGateway {
                    run_id: format!("run-capacity-{index}"),
                    gateway_request_id: format!("gateway-request-capacity-{index}"),
                    call_id: format!("call-capacity-{index}"),
                });
        }
        assert_eq!(
            capacity_manager.create_request(local_task_decision(
                "run-capacity-new",
                "gateway-request-capacity-new",
                "call-capacity-new",
                "Capacity task",
            )?),
            Err(ApprovalError::SubjectCapacityExhausted {
                maximum: MAX_APPROVAL_SUBJECTS_PER_MANAGER,
            })
        );

        let mut id_manager = InMemoryApprovalManager::new();
        id_manager.next_id = u64::MAX;
        assert_eq!(
            id_manager.create_request(local_task_decision(
                "run-id-overflow-1",
                "gateway-request-id-overflow-1",
                "call-id-overflow-1",
                "ID overflow task",
            )?),
            Err(ApprovalError::IdSpaceExhausted)
        );

        let mut deadline_manager = InMemoryApprovalManager::with_clock(OverflowClock {
            now: Instant::now(),
        });
        assert_eq!(
            deadline_manager.create_request(local_task_decision(
                "run-deadline-overflow-1",
                "gateway-request-deadline-overflow-1",
                "call-deadline-overflow-1",
                "Deadline overflow task",
            )?),
            Err(ApprovalError::DeadlineOverflow)
        );
        Ok(())
    }

    #[test]
    fn debug_and_errors_redact_approval_content() -> Result<(), Box<dyn Error>> {
        let sentinel = "private-approval-title";
        let raw_arguments = json!({ "title": sentinel }).to_string();
        let mut manager = InMemoryApprovalManager::new();
        let id = manager.create_request(local_task_decision(
            "run-approval-redaction-1",
            "gateway-request-approval-redaction-1",
            "call-approval-redaction-1",
            sentinel,
        )?)?;
        let manager_debug = format!("{manager:?}");
        let Some(view) = manager.pending()? else {
            return Err(ApprovalError::NotFound(id.value()).into());
        };
        let view_debug = format!("{view:?}");
        let preview_debug = format!("{:?}", view.preview());
        let affected_debug = format!("{:?}", view.preview().affected_data());

        for output in [&manager_debug, &view_debug, &preview_debug, &affected_debug] {
            assert!(!output.contains(sentinel));
            assert!(!output.contains(&raw_arguments));
            assert!(!output.contains("run-approval-redaction-1"));
            assert!(!output.contains("gateway-request-approval-redaction-1"));
            assert!(!output.contains("call-approval-redaction-1"));
        }

        let presentation = manager.issue_presentation(id)?;
        let presentation_debug = format!("{presentation:?}");
        for private_value in [
            sentinel,
            "run-approval-redaction-1",
            "gateway-request-approval-redaction-1",
            "call-approval-redaction-1",
        ] {
            assert!(!presentation_debug.contains(private_value));
        }

        let resolution = manager.cancel_for_run_termination(id)?;
        let resolution_debug = format!("{resolution:?}");
        let replay_error = manager.cancel_for_run_termination(id).err();
        let Some(replay_error) = replay_error else {
            return Err(ApprovalError::AlreadyConsumed(id.value()).into());
        };
        let error_display = replay_error.to_string();
        let error_debug = format!("{replay_error:?}");

        for output in [&resolution_debug, &error_display, &error_debug] {
            assert!(!output.contains(sentinel));
            assert!(!output.contains(&raw_arguments));
        }
        Ok(())
    }
}
