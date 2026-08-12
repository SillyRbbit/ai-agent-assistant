use std::fmt;
use std::time::Duration;

use thiserror::Error;

use crate::agent::governance::{AgentAttribution, AgentPolicyDecision, AgentPolicyReason};
use crate::policy::types::{PolicyDecision, PolicyOutcome, PolicyReason};
use crate::tools::schema::ValidatedToolArguments;
use crate::tools::types::{PermissionKind, RiskClass};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApprovalId(u64);

impl ApprovalId {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalDisposition {
    Approved,
    Rejected,
    Cancelled(ApprovalCancellationReason),
    Expired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalCancellationReason {
    RunTerminated,
    EditRequested,
    NativeNoDecision,
    SourceFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalInteractionSource {
    MacOsNativeDialog,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalNativeButton {
    Approve,
    Reject,
    Edit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalAuthenticationEvidence {
    NotEvaluated,
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum ApprovalSourceFailure {
    #[error("approval presentation contains unsafe formatting")]
    UnsafePresentationFormatting,
    #[error("approval presentation exceeds the message limit")]
    MessageLimitExceeded,
    #[error("native approval dialog returned an unexpected result")]
    UnexpectedDialogResult,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApprovalInteractionEvidence {
    source: ApprovalInteractionSource,
    native_button: Option<ApprovalNativeButton>,
    authentication: ApprovalAuthenticationEvidence,
    source_failure: Option<ApprovalSourceFailure>,
}

impl ApprovalInteractionEvidence {
    #[cfg(target_os = "macos")]
    pub(super) fn recognized_button(
        source: ApprovalInteractionSource,
        native_button: ApprovalNativeButton,
        authentication: ApprovalAuthenticationEvidence,
    ) -> Self {
        Self {
            source,
            native_button: Some(native_button),
            authentication,
            source_failure: None,
        }
    }

    #[cfg(target_os = "macos")]
    pub(super) fn no_decision(
        source: ApprovalInteractionSource,
        authentication: ApprovalAuthenticationEvidence,
    ) -> Self {
        Self {
            source,
            native_button: None,
            authentication,
            source_failure: None,
        }
    }

    #[cfg(target_os = "macos")]
    pub(super) fn source_failed(
        source: ApprovalInteractionSource,
        authentication: ApprovalAuthenticationEvidence,
        failure: ApprovalSourceFailure,
    ) -> Self {
        Self {
            source,
            native_button: None,
            authentication,
            source_failure: Some(failure),
        }
    }

    #[must_use]
    pub fn source(self) -> ApprovalInteractionSource {
        self.source
    }

    #[must_use]
    pub fn native_button(self) -> Option<ApprovalNativeButton> {
        self.native_button
    }

    #[must_use]
    pub fn authentication(self) -> ApprovalAuthenticationEvidence {
        self.authentication
    }

    #[must_use]
    pub fn source_failure(self) -> Option<ApprovalSourceFailure> {
        self.source_failure
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalAction {
    CreateLocalTask,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalTarget {
    LocalTaskList,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalSchedule {
    NotScheduled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalRecipients {
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalReversibility {
    Reversible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalRisk {
    CreatesLocalTask,
}

/// Closed provenance for an approval lifecycle.
///
/// Agent attribution is derived by the application and remains attached to
/// the approval through presentation and terminal resolution. This value is
/// evidence only; it carries no execution authority.
#[derive(Clone, Eq, PartialEq)]
pub enum ApprovalOrigin {
    LegacyGateway,
    Agent(AgentAttribution),
}

impl ApprovalOrigin {
    #[must_use]
    pub fn agent_attribution(&self) -> Option<&AgentAttribution> {
        match self {
            Self::LegacyGateway => None,
            Self::Agent(attribution) => Some(attribution),
        }
    }
}

impl fmt::Debug for ApprovalOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LegacyGateway => formatter.write_str("LegacyGateway"),
            Self::Agent(_) => formatter.debug_tuple("Agent").field(&"[REDACTED]").finish(),
        }
    }
}

/// The one closed internal decision family accepted by the approval manager.
///
/// The deterministic reason is sealed alongside an agent decision so legacy
/// accessors can remain infallible without treating an ineligible agent
/// profile as approval-eligible.
#[derive(Eq, PartialEq)]
pub(crate) enum ApprovalDecision {
    LegacyGateway(PolicyDecision),
    Agent {
        decision: AgentPolicyDecision,
        deterministic_reason: PolicyReason,
    },
}

impl ApprovalDecision {
    pub(crate) fn legacy(decision: PolicyDecision) -> Self {
        Self::LegacyGateway(decision)
    }

    pub(crate) fn agent(decision: AgentPolicyDecision) -> Option<Self> {
        let AgentPolicyReason::Deterministic(deterministic_reason) = decision.reason() else {
            return None;
        };
        Some(Self::Agent {
            decision,
            deterministic_reason,
        })
    }

    pub(crate) fn outcome(&self) -> PolicyOutcome {
        match self {
            Self::LegacyGateway(decision) => decision.outcome(),
            Self::Agent { decision, .. } => decision.outcome(),
        }
    }

    pub(crate) fn policy_reason(&self) -> PolicyReason {
        match self {
            Self::LegacyGateway(decision) => decision.reason(),
            Self::Agent {
                deterministic_reason,
                ..
            } => *deterministic_reason,
        }
    }

    pub(crate) fn agent_policy_reason(&self) -> Option<AgentPolicyReason> {
        match self {
            Self::LegacyGateway(_) => None,
            Self::Agent { decision, .. } => Some(decision.reason()),
        }
    }

    pub(crate) fn origin(&self) -> ApprovalOrigin {
        match self {
            Self::LegacyGateway(_) => ApprovalOrigin::LegacyGateway,
            Self::Agent { decision, .. } => {
                ApprovalOrigin::Agent(decision.request().attribution().clone())
            }
        }
    }

    pub(crate) fn run_id(&self) -> &str {
        match self {
            Self::LegacyGateway(decision) => decision.validated_call().run_id(),
            Self::Agent { decision, .. } => decision
                .request()
                .attribution()
                .runtime_run_identity()
                .run_id()
                .as_str(),
        }
    }

    pub(crate) fn gateway_request_id(&self) -> &str {
        match self {
            Self::LegacyGateway(decision) => decision.validated_call().gateway_request_id(),
            Self::Agent { decision, .. } => decision
                .request()
                .attribution()
                .runtime_run_identity()
                .request_id()
                .as_str(),
        }
    }

    pub(crate) fn call_id(&self) -> &str {
        match self {
            Self::LegacyGateway(decision) => decision.validated_call().call_id(),
            Self::Agent { decision, .. } => decision.request().call_id(),
        }
    }

    pub(crate) fn tool_name(&self) -> &str {
        match self {
            Self::LegacyGateway(decision) => decision.validated_call().tool_name(),
            Self::Agent { decision, .. } => decision.request().schema().name(),
        }
    }

    pub(crate) fn tool_contract_version(&self) -> u16 {
        match self {
            Self::LegacyGateway(decision) => decision.validated_call().tool_contract_version(),
            Self::Agent { decision, .. } => decision.request().schema().version(),
        }
    }

    pub(crate) fn risk_class(&self) -> RiskClass {
        match self {
            Self::LegacyGateway(decision) => decision.validated_call().risk_class(),
            Self::Agent { decision, .. } => decision.request().risk_class(),
        }
    }

    pub(crate) fn required_permission(&self) -> PermissionKind {
        match self {
            Self::LegacyGateway(decision) => decision.validated_call().required_permission(),
            Self::Agent { decision, .. } => decision.request().required_permission(),
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum ApprovalAffectedData<'a> {
    LocalTaskTitle(&'a str),
}

impl ApprovalAffectedData<'_> {
    #[must_use]
    pub fn value(&self) -> &str {
        match self {
            Self::LocalTaskTitle(title) => title,
        }
    }
}

impl fmt::Debug for ApprovalAffectedData<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LocalTaskTitle(_) => formatter
                .debug_tuple("LocalTaskTitle")
                .field(&"[REDACTED]")
                .finish(),
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum ApprovalPreview<'a> {
    CreateLocalTask { title: &'a str },
}

impl<'a> ApprovalPreview<'a> {
    pub(crate) fn from_policy_decision(decision: &'a PolicyDecision) -> Option<Self> {
        if decision.outcome() != PolicyOutcome::RequireApproval {
            return None;
        }

        match decision.validated_call().arguments() {
            ValidatedToolArguments::CreateLocalTask(arguments) => Some(Self::CreateLocalTask {
                title: arguments.title(),
            }),
            ValidatedToolArguments::GetCurrentDatetime => None,
        }
    }

    pub(crate) fn from_approval_decision(decision: &'a ApprovalDecision) -> Option<Self> {
        match decision {
            ApprovalDecision::LegacyGateway(decision) => Self::from_policy_decision(decision),
            ApprovalDecision::Agent { decision, .. } => {
                if decision.outcome() != PolicyOutcome::RequireApproval {
                    return None;
                }
                match decision.request().arguments() {
                    ValidatedToolArguments::CreateLocalTask(arguments) => {
                        Some(Self::CreateLocalTask {
                            title: arguments.title(),
                        })
                    }
                    ValidatedToolArguments::GetCurrentDatetime => None,
                }
            }
        }
    }

    #[must_use]
    pub fn action(&self) -> ApprovalAction {
        ApprovalAction::CreateLocalTask
    }

    #[must_use]
    pub fn target(&self) -> ApprovalTarget {
        ApprovalTarget::LocalTaskList
    }

    #[must_use]
    pub fn affected_data(&self) -> ApprovalAffectedData<'a> {
        match self {
            Self::CreateLocalTask { title } => ApprovalAffectedData::LocalTaskTitle(title),
        }
    }

    #[must_use]
    pub fn schedule(&self) -> ApprovalSchedule {
        ApprovalSchedule::NotScheduled
    }

    #[must_use]
    pub fn recipients(&self) -> ApprovalRecipients {
        ApprovalRecipients::None
    }

    #[must_use]
    pub fn reversibility(&self) -> ApprovalReversibility {
        ApprovalReversibility::Reversible
    }

    #[must_use]
    pub fn required_permission(&self) -> PermissionKind {
        PermissionKind::None
    }

    #[must_use]
    pub fn risk_class(&self) -> RiskClass {
        RiskClass::ReversibleLocalAction
    }

    #[must_use]
    pub fn risk(&self) -> ApprovalRisk {
        ApprovalRisk::CreatesLocalTask
    }
}

impl fmt::Debug for ApprovalPreview<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateLocalTask { .. } => formatter
                .debug_struct("CreateLocalTask")
                .field("title", &"[REDACTED]")
                .finish(),
        }
    }
}

pub struct ApprovalRequestView<'a> {
    id: ApprovalId,
    origin: ApprovalOrigin,
    decision: &'a ApprovalDecision,
    preview: ApprovalPreview<'a>,
    remaining: Duration,
}

impl<'a> ApprovalRequestView<'a> {
    pub(crate) fn from_approval_decision(
        id: ApprovalId,
        decision: &'a ApprovalDecision,
        remaining: Duration,
    ) -> Option<Self> {
        Some(Self {
            id,
            origin: decision.origin(),
            decision,
            preview: ApprovalPreview::from_approval_decision(decision)?,
            remaining,
        })
    }

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
        self.decision.run_id()
    }

    #[must_use]
    pub fn gateway_request_id(&self) -> &str {
        self.decision.gateway_request_id()
    }

    #[must_use]
    pub fn call_id(&self) -> &str {
        self.decision.call_id()
    }

    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.decision.tool_name()
    }

    #[must_use]
    pub fn tool_contract_version(&self) -> u16 {
        self.decision.tool_contract_version()
    }

    #[must_use]
    pub fn risk_class(&self) -> RiskClass {
        self.decision.risk_class()
    }

    #[must_use]
    pub fn required_permission(&self) -> PermissionKind {
        self.decision.required_permission()
    }

    #[must_use]
    pub fn policy_outcome(&self) -> PolicyOutcome {
        self.decision.outcome()
    }

    #[must_use]
    pub fn policy_reason(&self) -> PolicyReason {
        self.decision.policy_reason()
    }

    #[must_use]
    pub fn agent_policy_reason(&self) -> Option<AgentPolicyReason> {
        self.decision.agent_policy_reason()
    }

    #[must_use]
    pub fn preview(&self) -> &ApprovalPreview<'a> {
        &self.preview
    }

    #[must_use]
    pub fn remaining(&self) -> Duration {
        self.remaining
    }
}

impl fmt::Debug for ApprovalRequestView<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovalRequestView")
            .field("id", &self.id)
            .field("origin", &self.origin)
            .field("identity", &"[REDACTED]")
            .field("policy_outcome", &self.policy_outcome())
            .field("policy_reason", &self.policy_reason())
            .field("preview", &self.preview)
            .field("remaining", &self.remaining)
            .finish()
    }
}

#[derive(Eq, PartialEq)]
pub struct ApprovalResolution {
    id: ApprovalId,
    disposition: ApprovalDisposition,
    origin: ApprovalOrigin,
    decision: ApprovalDecision,
    interaction_evidence: Option<ApprovalInteractionEvidence>,
}

impl ApprovalResolution {
    pub(crate) fn new(
        id: ApprovalId,
        disposition: ApprovalDisposition,
        decision: PolicyDecision,
        interaction_evidence: Option<ApprovalInteractionEvidence>,
    ) -> Self {
        Self {
            id,
            disposition,
            origin: ApprovalOrigin::LegacyGateway,
            decision: ApprovalDecision::legacy(decision),
            interaction_evidence,
        }
    }

    pub(crate) fn from_approval_decision(
        id: ApprovalId,
        disposition: ApprovalDisposition,
        decision: ApprovalDecision,
        interaction_evidence: Option<ApprovalInteractionEvidence>,
    ) -> Self {
        let decision = match decision {
            ApprovalDecision::LegacyGateway(decision) => {
                return Self::new(id, disposition, decision, interaction_evidence);
            }
            agent @ ApprovalDecision::Agent { .. } => agent,
        };
        let origin = decision.origin();
        Self {
            id,
            disposition,
            origin,
            decision,
            interaction_evidence,
        }
    }

    #[must_use]
    pub fn id(&self) -> ApprovalId {
        self.id
    }

    #[must_use]
    pub fn disposition(&self) -> ApprovalDisposition {
        self.disposition
    }

    #[must_use]
    pub fn origin(&self) -> &ApprovalOrigin {
        &self.origin
    }

    #[must_use]
    pub fn interaction_evidence(&self) -> Option<ApprovalInteractionEvidence> {
        self.interaction_evidence
    }

    #[must_use]
    pub fn run_id(&self) -> &str {
        self.decision.run_id()
    }

    #[must_use]
    pub fn gateway_request_id(&self) -> &str {
        self.decision.gateway_request_id()
    }

    #[must_use]
    pub fn call_id(&self) -> &str {
        self.decision.call_id()
    }

    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.decision.tool_name()
    }

    #[must_use]
    pub fn tool_contract_version(&self) -> u16 {
        self.decision.tool_contract_version()
    }

    #[must_use]
    pub fn risk_class(&self) -> RiskClass {
        self.decision.risk_class()
    }

    #[must_use]
    pub fn required_permission(&self) -> PermissionKind {
        self.decision.required_permission()
    }

    #[must_use]
    pub fn policy_outcome(&self) -> PolicyOutcome {
        self.decision.outcome()
    }

    #[must_use]
    pub fn policy_reason(&self) -> PolicyReason {
        self.decision.policy_reason()
    }

    #[must_use]
    pub fn agent_policy_reason(&self) -> Option<AgentPolicyReason> {
        self.decision.agent_policy_reason()
    }

    #[must_use]
    pub fn preview(&self) -> Option<ApprovalPreview<'_>> {
        ApprovalPreview::from_approval_decision(&self.decision)
    }
}

impl fmt::Debug for ApprovalResolution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovalResolution")
            .field("id", &self.id)
            .field("disposition", &self.disposition)
            .field("origin", &self.origin)
            .field("identity", &"[REDACTED]")
            .field("policy_outcome", &self.policy_outcome())
            .field("policy_reason", &self.policy_reason())
            .field("content", &"[REDACTED]")
            .field("interaction_evidence", &self.interaction_evidence)
            .finish()
    }
}
