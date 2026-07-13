use crate::tools::types::{PermissionKind, RiskClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyContext {
    pub explicit_user_intent: bool,
    pub permission_granted: bool,
}

impl PolicyContext {
    #[must_use]
    pub fn new(explicit_user_intent: bool, permission_granted: bool) -> Self {
        Self {
            explicit_user_intent,
            permission_granted,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedAction {
    pub tool_name: String,
    pub risk_class: RiskClass,
    pub required_permission: PermissionKind,
    pub context: PolicyContext,
}

impl ProposedAction {
    #[must_use]
    pub fn new(
        tool_name: impl Into<String>,
        risk_class: RiskClass,
        required_permission: PermissionKind,
        context: PolicyContext,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            risk_class,
            required_permission,
            context,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PolicyOutcome {
    Allow,
    RequireApproval,
    Deny,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDecision {
    pub outcome: PolicyOutcome,
    pub reason: String,
}

impl PolicyDecision {
    #[must_use]
    pub fn allow(reason: impl Into<String>) -> Self {
        Self {
            outcome: PolicyOutcome::Allow,
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn require_approval(reason: impl Into<String>) -> Self {
        Self {
            outcome: PolicyOutcome::RequireApproval,
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            outcome: PolicyOutcome::Deny,
            reason: reason.into(),
        }
    }
}
