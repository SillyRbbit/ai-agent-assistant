use std::fmt;

use crate::agent::function_call_validation::SchemaValidatedFunctionCall;

/// A locally schema-validated call entering deterministic policy evaluation.
///
/// This value carries no approval, dispatch, or execution authority.
#[derive(Eq, PartialEq)]
pub struct PolicyInput {
    validated_call: SchemaValidatedFunctionCall,
}

impl PolicyInput {
    #[must_use]
    pub fn from_validated_call(validated_call: SchemaValidatedFunctionCall) -> Self {
        Self { validated_call }
    }

    #[must_use]
    pub fn validated_call(&self) -> &SchemaValidatedFunctionCall {
        &self.validated_call
    }
}

impl fmt::Debug for PolicyInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyInput")
            .field("validated_call", &self.validated_call)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyOutcome {
    Allow,
    RequireApproval,
    Deny,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyReason {
    InformationOnly,
    RequiredPermissionEvidenceUnavailable,
    ReadOnlyScopeEvidenceUnavailable,
    ReversibleRequiresApproval,
    PersonalDataRequiresApproval,
    ExternalOrHighImpactNotRegistered,
    ProhibitedAutonomy,
}

impl PolicyReason {
    #[must_use]
    pub fn outcome(self) -> PolicyOutcome {
        match self {
            Self::InformationOnly => PolicyOutcome::Allow,
            Self::ReversibleRequiresApproval | Self::PersonalDataRequiresApproval => {
                PolicyOutcome::RequireApproval
            }
            Self::RequiredPermissionEvidenceUnavailable
            | Self::ReadOnlyScopeEvidenceUnavailable
            | Self::ExternalOrHighImpactNotRegistered
            | Self::ProhibitedAutonomy => PolicyOutcome::Deny,
        }
    }
}

/// A deterministic policy result retaining the exact input that was evaluated.
///
/// Even an `Allow` outcome is non-authorizing data. This type cannot create an
/// approval request or dispatch a tool.
#[derive(Eq, PartialEq)]
pub struct PolicyDecision {
    input: PolicyInput,
    reason: PolicyReason,
}

impl PolicyDecision {
    pub(super) fn from_reason(input: PolicyInput, reason: PolicyReason) -> Self {
        Self { input, reason }
    }

    #[must_use]
    pub fn outcome(&self) -> PolicyOutcome {
        self.reason.outcome()
    }

    #[must_use]
    pub fn reason(&self) -> PolicyReason {
        self.reason
    }

    #[must_use]
    pub fn input(&self) -> &PolicyInput {
        &self.input
    }

    #[must_use]
    pub fn validated_call(&self) -> &SchemaValidatedFunctionCall {
        self.input.validated_call()
    }
}

impl fmt::Debug for PolicyDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyDecision")
            .field("outcome", &self.outcome())
            .field("reason", &self.reason)
            .field("input", &self.input)
            .finish()
    }
}
