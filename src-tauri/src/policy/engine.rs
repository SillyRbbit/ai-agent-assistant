use thiserror::Error;

use super::types::{PolicyDecision, ProposedAction};
use crate::tools::types::{PermissionKind, RiskClass};

pub type PolicyResult<T> = Result<T, PolicyError>;

pub trait PolicyEngine {
    fn evaluate(&self, action: ProposedAction) -> PolicyResult<PolicyDecision>;
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PolicyError {
    #[error("proposed action tool name must not be empty")]
    EmptyToolName,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DeterministicPolicyEngine;

impl DeterministicPolicyEngine {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl PolicyEngine for DeterministicPolicyEngine {
    fn evaluate(&self, action: ProposedAction) -> PolicyResult<PolicyDecision> {
        if action.tool_name.trim().is_empty() {
            return Err(PolicyError::EmptyToolName);
        }

        if action.required_permission != PermissionKind::None && !action.context.permission_granted
        {
            return Ok(PolicyDecision::deny(
                "required permission is not currently granted",
            ));
        }

        let decision = match action.risk_class {
            RiskClass::InformationOnly => PolicyDecision::allow("information-only action"),
            RiskClass::ReadOnlyDeviceAccess => {
                PolicyDecision::allow("read-only action with required scope")
            }
            RiskClass::ReversibleLocalAction if action.context.explicit_user_intent => {
                PolicyDecision::allow("reversible action with explicit user intent")
            }
            RiskClass::ReversibleLocalAction => PolicyDecision::require_approval(
                "reversible action lacks narrow deterministic intent",
            ),
            RiskClass::PersonalDataModification => PolicyDecision::require_approval(
                "personal-data modification always requires approval",
            ),
            RiskClass::ExternalOrHighImpactAction => PolicyDecision::deny(
                "external or high-impact actions are not registered in the MVP",
            ),
            RiskClass::ProhibitedAutonomy => {
                PolicyDecision::deny("prohibited autonomy is always denied")
            }
        };

        Ok(decision)
    }
}

#[cfg(test)]
mod tests {
    use super::{DeterministicPolicyEngine, PolicyEngine, PolicyError};
    use crate::policy::types::{PolicyContext, PolicyOutcome, ProposedAction};
    use crate::tools::types::{PermissionKind, RiskClass};

    fn action(risk_class: RiskClass, context: PolicyContext) -> ProposedAction {
        ProposedAction::new("example_tool", risk_class, PermissionKind::None, context)
    }

    #[test]
    fn allows_information_only_actions() {
        let engine = DeterministicPolicyEngine::new();
        let decision = engine.evaluate(action(
            RiskClass::InformationOnly,
            PolicyContext::new(false, true),
        ));

        assert!(matches!(
            decision,
            Ok(decision) if decision.outcome == PolicyOutcome::Allow
        ));
    }

    #[test]
    fn requires_approval_for_personal_data_modification() {
        let engine = DeterministicPolicyEngine::new();
        let decision = engine.evaluate(action(
            RiskClass::PersonalDataModification,
            PolicyContext::new(true, true),
        ));

        assert!(matches!(
            decision,
            Ok(decision) if decision.outcome == PolicyOutcome::RequireApproval
        ));
    }

    #[test]
    fn denies_prohibited_autonomy() {
        let engine = DeterministicPolicyEngine::new();
        let decision = engine.evaluate(action(
            RiskClass::ProhibitedAutonomy,
            PolicyContext::new(true, true),
        ));

        assert!(matches!(
            decision,
            Ok(decision) if decision.outcome == PolicyOutcome::Deny
        ));
    }

    #[test]
    fn denies_missing_permission_before_risk_evaluation() {
        let engine = DeterministicPolicyEngine::new();
        let decision = engine.evaluate(ProposedAction::new(
            "search_contacts",
            RiskClass::ReadOnlyDeviceAccess,
            PermissionKind::Contacts,
            PolicyContext::new(true, false),
        ));

        assert!(matches!(
            decision,
            Ok(decision) if decision.outcome == PolicyOutcome::Deny
        ));
    }

    #[test]
    fn rejects_empty_tool_names() {
        let engine = DeterministicPolicyEngine::new();

        assert_eq!(
            engine.evaluate(ProposedAction::new(
                " ",
                RiskClass::InformationOnly,
                PermissionKind::None,
                PolicyContext::new(true, true),
            )),
            Err(PolicyError::EmptyToolName)
        );
    }
}
