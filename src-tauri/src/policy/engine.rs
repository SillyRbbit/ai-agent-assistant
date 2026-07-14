use super::types::{PolicyDecision, PolicyInput, PolicyReason};
use crate::tools::types::{PermissionKind, RiskClass};

pub trait PolicyEngine {
    fn evaluate(&self, input: PolicyInput) -> PolicyDecision;
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
    fn evaluate(&self, input: PolicyInput) -> PolicyDecision {
        let call = input.validated_call();
        let reason = classify(call.risk_class(), call.required_permission());

        PolicyDecision::from_reason(input, reason)
    }
}

fn classify(risk_class: RiskClass, required_permission: PermissionKind) -> PolicyReason {
    match risk_class {
        RiskClass::ProhibitedAutonomy => PolicyReason::ProhibitedAutonomy,
        RiskClass::ExternalOrHighImpactAction => PolicyReason::ExternalOrHighImpactNotRegistered,
        _ if required_permission != PermissionKind::None => {
            PolicyReason::RequiredPermissionEvidenceUnavailable
        }
        RiskClass::InformationOnly => PolicyReason::InformationOnly,
        RiskClass::ReadOnlyDeviceAccess => PolicyReason::ReadOnlyScopeEvidenceUnavailable,
        RiskClass::ReversibleLocalAction => PolicyReason::ReversibleRequiresApproval,
        RiskClass::PersonalDataModification => PolicyReason::PersonalDataRequiresApproval,
    }
}

#[cfg(test)]
mod tests {
    use super::classify;
    use crate::policy::types::{PolicyOutcome, PolicyReason};
    use crate::tools::types::{PermissionKind, RiskClass};

    #[test]
    fn derives_one_outcome_from_each_closed_reason() {
        let cases = [
            (PolicyReason::InformationOnly, PolicyOutcome::Allow),
            (
                PolicyReason::RequiredPermissionEvidenceUnavailable,
                PolicyOutcome::Deny,
            ),
            (
                PolicyReason::ReadOnlyScopeEvidenceUnavailable,
                PolicyOutcome::Deny,
            ),
            (
                PolicyReason::ReversibleRequiresApproval,
                PolicyOutcome::RequireApproval,
            ),
            (
                PolicyReason::PersonalDataRequiresApproval,
                PolicyOutcome::RequireApproval,
            ),
            (
                PolicyReason::ExternalOrHighImpactNotRegistered,
                PolicyOutcome::Deny,
            ),
            (PolicyReason::ProhibitedAutonomy, PolicyOutcome::Deny),
        ];

        for (reason, expected) in cases {
            assert_eq!(reason.outcome(), expected);
        }
    }

    #[test]
    fn classifies_every_risk_conservatively_without_permission_evidence() {
        let cases = [
            (RiskClass::InformationOnly, PolicyReason::InformationOnly),
            (
                RiskClass::ReadOnlyDeviceAccess,
                PolicyReason::ReadOnlyScopeEvidenceUnavailable,
            ),
            (
                RiskClass::ReversibleLocalAction,
                PolicyReason::ReversibleRequiresApproval,
            ),
            (
                RiskClass::PersonalDataModification,
                PolicyReason::PersonalDataRequiresApproval,
            ),
            (
                RiskClass::ExternalOrHighImpactAction,
                PolicyReason::ExternalOrHighImpactNotRegistered,
            ),
            (
                RiskClass::ProhibitedAutonomy,
                PolicyReason::ProhibitedAutonomy,
            ),
        ];

        for (risk_class, expected) in cases {
            assert_eq!(classify(risk_class, PermissionKind::None), expected);
        }
    }

    #[test]
    fn denies_every_required_permission_without_call_bound_evidence() {
        let permissions = [
            PermissionKind::Calendar,
            PermissionKind::Reminders,
            PermissionKind::Contacts,
            PermissionKind::Notifications,
            PermissionKind::Files,
            PermissionKind::Accessibility,
            PermissionKind::ScreenRecording,
            PermissionKind::Automation,
            PermissionKind::Microphone,
        ];

        for permission in permissions {
            assert_eq!(
                classify(RiskClass::InformationOnly, permission),
                PolicyReason::RequiredPermissionEvidenceUnavailable
            );
        }
    }

    #[test]
    fn preserves_hard_denials_before_permission_classification() {
        assert_eq!(
            classify(
                RiskClass::ExternalOrHighImpactAction,
                PermissionKind::Contacts
            ),
            PolicyReason::ExternalOrHighImpactNotRegistered
        );
        assert_eq!(
            classify(RiskClass::ProhibitedAutonomy, PermissionKind::Contacts),
            PolicyReason::ProhibitedAutonomy
        );
    }
}
