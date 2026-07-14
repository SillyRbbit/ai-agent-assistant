use std::collections::BTreeSet;
use std::fmt;
use std::time::{Duration, Instant};

use thiserror::Error;

use super::types::{
    ApprovalChoice, ApprovalDisposition, ApprovalId, ApprovalPreview, ApprovalRequestView,
    ApprovalResolution,
};
use crate::policy::types::{PolicyDecision, PolicyOutcome};

pub const APPROVAL_TTL: Duration = Duration::from_secs(120);
pub const MAX_PENDING_APPROVALS: usize = 1;
pub const MAX_APPROVAL_SUBJECTS_PER_MANAGER: usize = 1_024;

pub type ApprovalResult<T> = Result<T, ApprovalError>;

pub trait ApprovalManager {
    fn create_request(&mut self, decision: PolicyDecision) -> ApprovalResult<ApprovalId>;
    fn pending(&self) -> ApprovalResult<Option<ApprovalRequestView<'_>>>;
    fn decide(
        &mut self,
        id: ApprovalId,
        choice: ApprovalChoice,
    ) -> ApprovalResult<ApprovalResolution>;
    fn cancel(&mut self, id: ApprovalId) -> ApprovalResult<ApprovalResolution>;
    fn expire_due(&mut self) -> Option<ApprovalResolution>;
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum ApprovalError {
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

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct ApprovalSubjectKey {
    run_id: String,
    gateway_request_id: String,
    call_id: String,
}

impl ApprovalSubjectKey {
    fn from_decision(decision: &PolicyDecision) -> Self {
        let call = decision.validated_call();
        Self {
            run_id: call.run_id().to_owned(),
            gateway_request_id: call.gateway_request_id().to_owned(),
            call_id: call.call_id().to_owned(),
        }
    }
}

struct PendingApproval {
    id: ApprovalId,
    subject: ApprovalSubjectKey,
    decision: PolicyDecision,
    deadline: Instant,
}

pub struct InMemoryApprovalManager {
    next_id: u64,
    pending: Option<PendingApproval>,
    consumed_subjects: BTreeSet<ApprovalSubjectKey>,
    clock: Box<dyn ApprovalClock>,
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
            consumed_subjects: BTreeSet::new(),
            clock: Box::new(clock),
        }
    }

    fn resolve(
        &mut self,
        id: ApprovalId,
        requested_disposition: ApprovalDisposition,
    ) -> ApprovalResult<ApprovalResolution> {
        let disposition = {
            let Some(pending) = self.pending.as_ref() else {
                return Err(self.error_for_missing_id(id));
            };
            if pending.id != id {
                return Err(self.error_for_missing_id(id));
            }

            if self.clock.now() >= pending.deadline {
                ApprovalDisposition::Expired
            } else {
                requested_disposition
            }
        };

        let Some(pending) = self.pending.take() else {
            return Err(self.error_for_missing_id(id));
        };
        self.consumed_subjects.insert(pending.subject);
        Ok(ApprovalResolution::new(
            pending.id,
            disposition,
            pending.decision,
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
        let actual = decision.outcome();
        if actual != PolicyOutcome::RequireApproval {
            return Err(ApprovalError::IneligiblePolicyOutcome { actual });
        }
        if ApprovalPreview::from_policy_decision(&decision).is_none() {
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
        });
        Ok(id)
    }

    fn pending(&self) -> ApprovalResult<Option<ApprovalRequestView<'_>>> {
        let Some(pending) = self.pending.as_ref() else {
            return Ok(None);
        };
        let now = self.clock.now();
        if now >= pending.deadline {
            return Ok(None);
        }

        ApprovalRequestView::from_policy_decision(
            pending.id,
            &pending.decision,
            pending.deadline.saturating_duration_since(now),
        )
        .map(Some)
        .ok_or(ApprovalError::UnsupportedApprovalSubject)
    }

    fn decide(
        &mut self,
        id: ApprovalId,
        choice: ApprovalChoice,
    ) -> ApprovalResult<ApprovalResolution> {
        let disposition = match choice {
            ApprovalChoice::Approve => ApprovalDisposition::Approved,
            ApprovalChoice::Reject => ApprovalDisposition::Rejected,
        };
        self.resolve(id, disposition)
    }

    fn cancel(&mut self, id: ApprovalId) -> ApprovalResult<ApprovalResolution> {
        self.resolve(id, ApprovalDisposition::Cancelled)
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
        self.consumed_subjects.insert(pending.subject);
        Some(ApprovalResolution::new(
            pending.id,
            ApprovalDisposition::Expired,
            pending.decision,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::error::Error;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

    use serde_json::json;

    use super::{
        ApprovalClock, ApprovalError, ApprovalManager, ApprovalSubjectKey, InMemoryApprovalManager,
        APPROVAL_TTL, MAX_APPROVAL_SUBJECTS_PER_MANAGER, MAX_PENDING_APPROVALS,
    };
    use crate::agent::function_call_validation::validate_function_call;
    use crate::agent::gateway_protocol::{
        GatewayProtocolError, GatewayStreamValidator, ValidatedGatewayEvent,
        GATEWAY_PROTOCOL_VERSION,
    };
    use crate::approvals::types::{
        ApprovalAction, ApprovalChoice, ApprovalDisposition, ApprovalId, ApprovalPreview,
        ApprovalRecipients, ApprovalReversibility, ApprovalRisk, ApprovalSchedule, ApprovalTarget,
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
    fn consumes_approve_reject_and_cancel_once() -> Result<(), Box<dyn Error>> {
        let mut manager = InMemoryApprovalManager::new();
        let approved_id = manager.create_request(local_task_decision(
            "run-approval-resolve-1",
            "gateway-request-approval-resolve-1",
            "call-approval-resolve-1",
            "Approved task",
        )?)?;
        let approved = manager.decide(approved_id, ApprovalChoice::Approve)?;

        assert_eq!(approved.disposition(), ApprovalDisposition::Approved);
        assert_eq!(approved.id(), approved_id);
        assert_eq!(approved.run_id(), "run-approval-resolve-1");
        assert_eq!(
            approved.gateway_request_id(),
            "gateway-request-approval-resolve-1"
        );
        assert_eq!(approved.call_id(), "call-approval-resolve-1");
        assert_eq!(approved.tool_name(), "create_local_task");
        assert_eq!(approved.tool_contract_version(), TOOL_CONTRACT_VERSION);
        assert_eq!(approved.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(approved.required_permission(), PermissionKind::None);
        assert_eq!(approved.policy_outcome(), PolicyOutcome::RequireApproval);
        assert_eq!(
            approved.policy_reason(),
            PolicyReason::ReversibleRequiresApproval
        );
        let Some(preview) = approved.preview() else {
            return Err(ApprovalError::UnsupportedApprovalSubject.into());
        };
        assert_eq!(preview.affected_data().value(), "Approved task");
        assert_eq!(
            manager.decide(approved_id, ApprovalChoice::Reject),
            Err(ApprovalError::AlreadyConsumed(approved_id.value()))
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

        let rejected_id = manager.create_request(local_task_decision(
            "run-approval-resolve-2",
            "gateway-request-approval-resolve-2",
            "call-approval-resolve-2",
            "Rejected task",
        )?)?;
        assert_eq!(
            manager
                .decide(rejected_id, ApprovalChoice::Reject)?
                .disposition(),
            ApprovalDisposition::Rejected
        );
        assert_eq!(
            manager.create_request(local_task_decision(
                "run-approval-resolve-2",
                "gateway-request-approval-resolve-2",
                "call-approval-resolve-2",
                "Changed rejected task",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );

        let cancelled_id = manager.create_request(local_task_decision(
            "run-approval-resolve-3",
            "gateway-request-approval-resolve-3",
            "call-approval-resolve-3",
            "Cancelled task",
        )?)?;
        assert_eq!(
            manager.cancel(cancelled_id)?.disposition(),
            ApprovalDisposition::Cancelled
        );
        assert_eq!(
            manager.create_request(local_task_decision(
                "run-approval-resolve-3",
                "gateway-request-approval-resolve-3",
                "call-approval-resolve-3",
                "Changed cancelled task",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );
        assert_eq!(
            manager.cancel(ApprovalId::new(404)),
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
            before_manager.cancel(before_id)?.disposition(),
            ApprovalDisposition::Cancelled
        );

        let exact_clock = TestClock::new();
        let mut exact_manager = InMemoryApprovalManager::with_clock(exact_clock.clone());
        let exact_id = exact_manager.create_request(local_task_decision(
            "run-approval-exact-1",
            "gateway-request-approval-exact-1",
            "call-approval-exact-1",
            "Exact deadline",
        )?)?;
        assert!(exact_clock.advance(APPROVAL_TTL));
        assert!(exact_manager.pending()?.is_none());
        assert_eq!(
            exact_manager
                .decide(exact_id, ApprovalChoice::Approve)?
                .disposition(),
            ApprovalDisposition::Expired
        );
        assert_eq!(
            exact_manager.decide(exact_id, ApprovalChoice::Approve),
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
                .insert(ApprovalSubjectKey {
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
        }

        let resolution = manager.decide(id, ApprovalChoice::Approve)?;
        let resolution_debug = format!("{resolution:?}");
        let replay_error = manager.decide(id, ApprovalChoice::Approve).err();
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
