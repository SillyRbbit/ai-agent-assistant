//! Closed, non-executing per-agent governance contracts.
//!
//! This module binds immutable policy profiles and live task/run attribution to
//! deterministic policy, approval, delegation-matrix, and volatile audit
//! evidence. It has no executor and cannot grant device authority.

use std::{collections::BTreeMap, fmt};

use thiserror::Error;

use super::orchestrator::LiveAgentAttributionProof;
use super::{
    definition::AgentId,
    gateway_protocol::{is_valid_opaque_id, MAX_FUNCTION_ARGUMENT_BYTES, MAX_OPAQUE_ID_BYTES},
    runtime::{RuntimeId, RuntimeRunIdentity},
    task::{AgentExecutionContext, AgentTaskId, ParentTaskId, RootTaskId},
};
use crate::{
    approvals::{
        manager::{ApprovalError, ApprovalManager, ApprovalPresentation, InMemoryApprovalManager},
        types::{ApprovalDisposition, ApprovalId, ApprovalRequestView, ApprovalResolution},
    },
    audit::governance::{
        AgentDelegationAuditReservation, AgentGovernanceRecord, AgentPendingApprovalAuditToken,
        AgentPendingDelegationAuditToken, InMemoryAgentGovernanceAudit,
    },
    memory::AgentMemoryProfileId,
    policy::engine::DeterministicPolicyEngine,
    policy::types::{PolicyOutcome, PolicyReason},
    tools::{
        registry::{InMemoryToolRegistry, ToolRegistry},
        schema::ValidatedToolArguments,
        types::{PermissionKind, RiskClass, ToolSchema},
    },
};

#[cfg(target_os = "macos")]
use crate::approvals::decision_source::TrustedApprovalSourceOutcome;

pub type AgentGovernanceResult<T> = Result<T, AgentGovernanceError>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AgentPolicyProfileId {
    PersonalAssistantV1,
    ResearchReadOnlyV1,
    CodingGovernedV1,
    CloudInfrastructureGovernedV1,
    SystemsOperationsGovernedV1,
    KnowledgeDocumentsV1,
    QualityValidationAdvisoryV1,
    SecurityRiskAdvisoryV1,
    WorkflowProposalOnlyV1,
}

impl AgentPolicyProfileId {
    pub const ALL: [Self; 9] = [
        Self::PersonalAssistantV1,
        Self::ResearchReadOnlyV1,
        Self::CodingGovernedV1,
        Self::CloudInfrastructureGovernedV1,
        Self::SystemsOperationsGovernedV1,
        Self::KnowledgeDocumentsV1,
        Self::QualityValidationAdvisoryV1,
        Self::SecurityRiskAdvisoryV1,
        Self::WorkflowProposalOnlyV1,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentPolicyProfile {
    id: AgentPolicyProfileId,
}

impl AgentPolicyProfile {
    const fn new(id: AgentPolicyProfileId) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(self) -> AgentPolicyProfileId {
        self.id
    }

    #[must_use]
    pub const fn permits(self, schema: ToolSchema) -> bool {
        matches!(self.id, AgentPolicyProfileId::PersonalAssistantV1)
            && matches!(
                schema,
                ToolSchema::GetCurrentDatetimeV1 | ToolSchema::CreateLocalTaskV1
            )
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentPolicyProfileRegistry {
    profiles: BTreeMap<AgentPolicyProfileId, AgentPolicyProfile>,
}

impl AgentPolicyProfileRegistry {
    pub fn built_in() -> AgentGovernanceResult<Self> {
        Self::from_profiles(
            AgentPolicyProfileId::ALL
                .into_iter()
                .map(AgentPolicyProfile::new),
        )
    }

    pub(crate) fn from_profiles(
        profiles: impl IntoIterator<Item = AgentPolicyProfile>,
    ) -> AgentGovernanceResult<Self> {
        let mut registered = BTreeMap::new();
        for profile in profiles {
            let id = profile.id();
            if registered.insert(id, profile).is_some() {
                return Err(AgentGovernanceError::DuplicateProfile { profile_id: id });
            }
        }
        for expected in AgentPolicyProfileId::ALL {
            if !registered.contains_key(&expected) {
                return Err(AgentGovernanceError::MissingProfile {
                    profile_id: expected,
                });
            }
        }
        Ok(Self {
            profiles: registered,
        })
    }

    pub fn get(
        &self,
        profile_id: AgentPolicyProfileId,
    ) -> AgentGovernanceResult<AgentPolicyProfile> {
        self.profiles
            .get(&profile_id)
            .copied()
            .ok_or(AgentGovernanceError::MissingProfile { profile_id })
    }

    pub fn list(&self) -> impl ExactSizeIterator<Item = AgentPolicyProfile> + '_ {
        self.profiles.values().copied()
    }
}

impl fmt::Debug for AgentPolicyProfileRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentPolicyProfileRegistry")
            .field("profile_ids", &self.profiles.keys().collect::<Vec<_>>())
            .field("profile_count", &self.profiles.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AgentAttribution {
    agent_id: AgentId,
    task_id: AgentTaskId,
    root_task_id: RootTaskId,
    parent_task_id: Option<ParentTaskId>,
    runtime_id: RuntimeId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    depth: u8,
    runtime_run_identity: RuntimeRunIdentity,
}

impl AgentAttribution {
    pub(super) fn from_live_context(
        context: &AgentExecutionContext,
        _proof: &LiveAgentAttributionProof,
    ) -> Self {
        Self {
            agent_id: context.agent_id(),
            task_id: context.task_id().clone(),
            root_task_id: context.root_task_id().clone(),
            parent_task_id: context.parent_task_id().cloned(),
            runtime_id: context.runtime_id(),
            policy_profile_id: context.policy_profile_id(),
            memory_profile_id: context.memory_profile_id(),
            depth: context.depth(),
            runtime_run_identity: context.runtime_run_identity().clone(),
        }
    }

    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
    }

    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub fn root_task_id(&self) -> &RootTaskId {
        &self.root_task_id
    }

    #[must_use]
    pub fn parent_task_id(&self) -> Option<&ParentTaskId> {
        self.parent_task_id.as_ref()
    }

    #[must_use]
    pub const fn runtime_id(&self) -> RuntimeId {
        self.runtime_id
    }

    #[must_use]
    pub const fn policy_profile_id(&self) -> AgentPolicyProfileId {
        self.policy_profile_id
    }

    #[must_use]
    pub const fn memory_profile_id(&self) -> AgentMemoryProfileId {
        self.memory_profile_id
    }

    #[must_use]
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    #[must_use]
    pub fn runtime_run_identity(&self) -> &RuntimeRunIdentity {
        &self.runtime_run_identity
    }
}

impl fmt::Debug for AgentAttribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentAttribution")
            .field("agent_id", &self.agent_id)
            .field("task_id", &self.task_id)
            .field("root_task_id", &self.root_task_id)
            .field("parent_task_id", &self.parent_task_id)
            .field("runtime_id", &self.runtime_id)
            .field("policy_profile_id", &self.policy_profile_id)
            .field("memory_profile_id", &self.memory_profile_id)
            .field("depth", &self.depth)
            .field("runtime_run_identity", &self.runtime_run_identity)
            .finish()
    }
}

/// Bounded untrusted synthetic proposal. This is deliberately separate from
/// runtime events, which remain unsupported by the orchestrator.
#[derive(Eq, PartialEq)]
pub struct AgentToolProposal {
    call_id: String,
    name: String,
    tool_contract_version: u16,
    arguments_json: String,
}

impl AgentToolProposal {
    pub fn new(
        call_id: impl Into<String>,
        name: impl Into<String>,
        tool_contract_version: u16,
        arguments_json: impl Into<String>,
    ) -> AgentGovernanceResult<Self> {
        let call_id = call_id.into();
        if !is_valid_opaque_id(&call_id) {
            return Err(AgentGovernanceError::InvalidToolCallId);
        }
        let name = name.into();
        if name.is_empty()
            || name.len() > MAX_OPAQUE_ID_BYTES
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err(AgentGovernanceError::InvalidToolName);
        }
        if tool_contract_version == 0 {
            return Err(AgentGovernanceError::InvalidToolContractVersion);
        }
        let arguments_json = arguments_json.into();
        if arguments_json.len() > MAX_FUNCTION_ARGUMENT_BYTES {
            return Err(AgentGovernanceError::ToolArgumentsTooLarge {
                maximum: MAX_FUNCTION_ARGUMENT_BYTES,
                actual: arguments_json.len(),
            });
        }
        Ok(Self {
            call_id,
            name,
            tool_contract_version,
            arguments_json,
        })
    }

    pub(super) fn into_parts(self) -> (String, String, u16, String) {
        (
            self.call_id,
            self.name,
            self.tool_contract_version,
            self.arguments_json,
        )
    }
}

impl fmt::Debug for AgentToolProposal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentToolProposal")
            .field("call_id", &"[REDACTED]")
            .field("name", &"[REDACTED]")
            .field("tool_contract_version", &self.tool_contract_version)
            .field("arguments_json", &"[REDACTED]")
            .finish()
    }
}

/// A locally schema-validated, exactly attributed agent request.
///
/// It cannot be converted into the legacy gateway `PolicyInput`.
#[derive(Eq, PartialEq)]
pub struct AgentSchemaValidatedToolRequest {
    attribution: AgentAttribution,
    call_id: String,
    schema: ToolSchema,
    arguments: ValidatedToolArguments,
    risk_class: RiskClass,
    required_permission: PermissionKind,
}

impl AgentSchemaValidatedToolRequest {
    pub(super) fn new(
        attribution: AgentAttribution,
        call_id: String,
        schema: ToolSchema,
        arguments: ValidatedToolArguments,
        risk_class: RiskClass,
        required_permission: PermissionKind,
    ) -> Self {
        Self {
            attribution,
            call_id,
            schema,
            arguments,
            risk_class,
            required_permission,
        }
    }

    #[must_use]
    pub fn attribution(&self) -> &AgentAttribution {
        &self.attribution
    }

    #[must_use]
    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    #[must_use]
    pub const fn schema(&self) -> ToolSchema {
        self.schema
    }

    #[must_use]
    pub fn arguments(&self) -> &ValidatedToolArguments {
        &self.arguments
    }

    #[must_use]
    pub const fn risk_class(&self) -> RiskClass {
        self.risk_class
    }

    #[must_use]
    pub const fn required_permission(&self) -> PermissionKind {
        self.required_permission
    }
}

impl fmt::Debug for AgentSchemaValidatedToolRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentSchemaValidatedToolRequest")
            .field("attribution", &self.attribution)
            .field("call_id", &"[REDACTED]")
            .field("schema", &self.schema)
            .field("arguments", &"[REDACTED]")
            .field("risk_class", &self.risk_class)
            .field("required_permission", &self.required_permission)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentPolicyReason {
    ProfileNotEligible,
    Deterministic(PolicyReason),
}

impl AgentPolicyReason {
    #[must_use]
    pub fn outcome(self) -> PolicyOutcome {
        match self {
            Self::ProfileNotEligible => PolicyOutcome::Deny,
            Self::Deterministic(reason) => reason.outcome(),
        }
    }

    #[must_use]
    pub const fn deterministic(self) -> Option<PolicyReason> {
        match self {
            Self::ProfileNotEligible => None,
            Self::Deterministic(reason) => Some(reason),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct AgentPolicyDecision {
    request: AgentSchemaValidatedToolRequest,
    reason: AgentPolicyReason,
}

impl AgentPolicyDecision {
    pub(crate) fn from_reason(
        request: AgentSchemaValidatedToolRequest,
        reason: AgentPolicyReason,
    ) -> Self {
        Self { request, reason }
    }

    #[must_use]
    pub fn outcome(&self) -> PolicyOutcome {
        self.reason.outcome()
    }

    #[must_use]
    pub const fn reason(&self) -> AgentPolicyReason {
        self.reason
    }

    #[must_use]
    pub fn request(&self) -> &AgentSchemaValidatedToolRequest {
        &self.request
    }
}

impl fmt::Debug for AgentPolicyDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentPolicyDecision")
            .field("outcome", &self.outcome())
            .field("reason", &self.reason)
            .field("request", &self.request)
            .finish()
    }
}

/// Closed result of deterministic policy at the non-executing agent boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentToolGovernanceOutcome {
    Final {
        policy_outcome: PolicyOutcome,
        policy_reason: AgentPolicyReason,
        approval: AgentApprovalAuditDisposition,
        execution: AgentExecutionDisposition,
    },
    ApprovalPending {
        approval_id: ApprovalId,
        policy_reason: AgentPolicyReason,
    },
}

/// Closed terminal approval evidence. Approval never dispatches a tool here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentApprovalGovernanceOutcome {
    approval_id: ApprovalId,
    disposition: AgentApprovalAuditDisposition,
    execution: AgentExecutionDisposition,
}

struct PendingAgentApproval {
    approval_id: ApprovalId,
    attribution: AgentAttribution,
    audit_token: AgentPendingApprovalAuditToken,
}

/// Application-owned, non-executing governance composition.
pub struct AgentGovernanceService {
    profiles: AgentPolicyProfileRegistry,
    tools: InMemoryToolRegistry,
    policy: DeterministicPolicyEngine,
    approvals: InMemoryApprovalManager,
    audit: InMemoryAgentGovernanceAudit,
    delegation_matrix: DelegationMatrix,
    pending_approval: Option<PendingAgentApproval>,
}

impl AgentGovernanceService {
    pub fn built_in() -> AgentGovernanceResult<Self> {
        let tools = InMemoryToolRegistry::built_in()
            .map_err(|_| AgentGovernanceError::GovernanceConfiguration)?;
        Ok(Self {
            profiles: AgentPolicyProfileRegistry::built_in()?,
            tools,
            policy: DeterministicPolicyEngine::new(),
            approvals: InMemoryApprovalManager::new(),
            audit: InMemoryAgentGovernanceAudit::new(),
            delegation_matrix: DelegationMatrix,
            pending_approval: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn force_pending_due_for_test(&mut self) {
        self.approvals.force_pending_due_for_test();
    }

    #[cfg(test)]
    pub(crate) fn fail_next_approval_cancel_for_test(&mut self) {
        self.approvals.fail_next_cancel_for_test();
    }

    #[cfg(test)]
    pub(crate) fn seed_pending_approval_for_test(
        &mut self,
        attribution: AgentAttribution,
        call_id: &str,
    ) -> AgentGovernanceResult<()> {
        if self.pending_approval.is_some() {
            return Err(AgentGovernanceError::ApprovalPending);
        }
        let schema = ToolSchema::CreateLocalTaskV1;
        let arguments = schema
            .validate_arguments(r#"{"title":"Synthetic cancellation fixture"}"#)
            .map_err(|_| AgentGovernanceError::InvalidToolArguments)?;
        let request = AgentSchemaValidatedToolRequest::new(
            attribution.clone(),
            call_id.to_owned(),
            schema,
            arguments,
            schema.risk_class(),
            schema.required_permission(),
        );
        let decision = AgentPolicyDecision::from_reason(
            request,
            AgentPolicyReason::Deterministic(PolicyReason::ReversibleRequiresApproval),
        );
        let reservation = self.audit.reserve_tool(attribution.clone(), call_id)?;
        let approval_id = self
            .approvals
            .create_agent_request(decision)
            .map_err(|_| AgentGovernanceError::ApprovalLifecycle)?;
        let audit_token = self.audit.commit_tool_approval_pending(
            reservation,
            AgentGovernanceAction::CreateLocalTask,
            AgentPolicyAuditOutcome::Evaluated {
                outcome: PolicyOutcome::RequireApproval,
                reason: AgentPolicyReason::Deterministic(PolicyReason::ReversibleRequiresApproval),
            },
        );
        self.pending_approval = Some(PendingAgentApproval {
            approval_id,
            attribution,
            audit_token,
        });
        Ok(())
    }

    pub(crate) fn evaluate_tool(
        &mut self,
        attribution: AgentAttribution,
        proposal: AgentToolProposal,
    ) -> AgentGovernanceResult<AgentToolGovernanceOutcome> {
        if self.pending_approval.is_some() {
            return Err(AgentGovernanceError::ApprovalPending);
        }
        let (call_id, name, actual_version, arguments_json) = proposal.into_parts();
        let reservation = self.audit.reserve_tool(attribution.clone(), &call_id)?;
        let definition = match self.tools.get(&name) {
            Ok(definition) => definition,
            Err(_) => {
                self.audit.commit_tool_validation_rejected(
                    reservation,
                    AgentGovernanceAction::UnknownTool,
                    AgentGovernanceErrorCode::UnknownTool,
                );
                return Err(AgentGovernanceError::UnknownTool);
            }
        };
        let schema = definition.schema();
        let action = AgentGovernanceAction::from_schema(schema);
        let expected_version = schema.version();
        if actual_version != expected_version {
            self.audit.commit_tool_validation_rejected(
                reservation,
                action,
                AgentGovernanceErrorCode::ToolContractVersionMismatch,
            );
            return Err(AgentGovernanceError::ToolContractVersionMismatch {
                expected: expected_version,
                actual: actual_version,
            });
        }
        let arguments = match schema.validate_arguments(&arguments_json) {
            Ok(arguments) => arguments,
            Err(_) => {
                self.audit.commit_tool_validation_rejected(
                    reservation,
                    action,
                    AgentGovernanceErrorCode::InvalidArguments,
                );
                return Err(AgentGovernanceError::InvalidToolArguments);
            }
        };
        let request = AgentSchemaValidatedToolRequest::new(
            attribution,
            call_id,
            schema,
            arguments,
            definition.risk_class(),
            definition.required_permission(),
        );
        let decision = match self.policy.evaluate_agent(request, &self.profiles) {
            Ok(decision) => decision,
            Err(error) => {
                self.audit.abort_tool(reservation);
                return Err(error);
            }
        };
        let policy_outcome = decision.outcome();
        let policy_reason = decision.reason();
        let policy_audit = AgentPolicyAuditOutcome::Evaluated {
            outcome: policy_outcome,
            reason: policy_reason,
        };
        match policy_outcome {
            PolicyOutcome::Allow | PolicyOutcome::Deny => {
                self.audit.commit_tool_policy_evaluated(
                    reservation,
                    action,
                    policy_audit,
                    (policy_outcome == PolicyOutcome::Deny)
                        .then_some(AgentGovernanceErrorCode::ProfileNotEligible),
                );
                Ok(AgentToolGovernanceOutcome::Final {
                    policy_outcome,
                    policy_reason,
                    approval: AgentApprovalAuditDisposition::NotRequired,
                    execution: AgentExecutionDisposition::NotAttempted,
                })
            }
            PolicyOutcome::RequireApproval => {
                let attribution = decision.request().attribution().clone();
                let approval_id = match self.approvals.create_agent_request(decision) {
                    Ok(approval_id) => approval_id,
                    Err(_) => {
                        self.audit.commit_tool_policy_evaluated(
                            reservation,
                            action,
                            policy_audit,
                            Some(AgentGovernanceErrorCode::ApprovalRequestFailed),
                        );
                        return Err(AgentGovernanceError::ApprovalLifecycle);
                    }
                };
                let audit_token =
                    self.audit
                        .commit_tool_approval_pending(reservation, action, policy_audit);
                self.pending_approval = Some(PendingAgentApproval {
                    approval_id,
                    attribution,
                    audit_token,
                });
                Ok(AgentToolGovernanceOutcome::ApprovalPending {
                    approval_id,
                    policy_reason,
                })
            }
        }
    }

    pub(crate) fn has_pending_for(&self, attribution: &AgentAttribution) -> bool {
        self.pending_approval
            .as_ref()
            .is_some_and(|pending| pending.attribution == *attribution)
    }

    pub(crate) fn has_pending_for_task(&self, task_id: &AgentTaskId) -> bool {
        self.pending_approval
            .as_ref()
            .is_some_and(|pending| pending.attribution.task_id() == task_id)
    }

    pub(crate) fn pending(
        &mut self,
        attribution: &AgentAttribution,
    ) -> AgentGovernanceResult<Option<ApprovalRequestView<'_>>> {
        self.ensure_pending_attribution(attribution)?;
        if let Some(resolution) = self.approvals.expire_due() {
            self.finish_resolution(resolution)?;
            return Ok(None);
        }
        self.approvals
            .pending()
            .map_err(|_| AgentGovernanceError::ApprovalLifecycle)
    }

    pub(crate) fn issue_presentation(
        &mut self,
        attribution: &AgentAttribution,
    ) -> AgentGovernanceResult<ApprovalPresentation> {
        self.ensure_pending_attribution(attribution)?;
        let id = self
            .pending_approval
            .as_ref()
            .map(|pending| pending.approval_id)
            .ok_or(AgentGovernanceError::NoPendingApproval)?;
        match self.approvals.issue_presentation(id) {
            Ok(presentation) => Ok(presentation),
            Err(ApprovalError::PresentationUnavailableOrExpired(_)) => {
                self.finish_pending_disposition(AgentApprovalAuditDisposition::Expired, None);
                Err(AgentGovernanceError::ApprovalExpired)
            }
            Err(_) => Err(AgentGovernanceError::ApprovalLifecycle),
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn resolve_source_outcome(
        &mut self,
        attribution: &AgentAttribution,
        outcome: TrustedApprovalSourceOutcome,
    ) -> AgentGovernanceResult<AgentApprovalGovernanceOutcome> {
        self.ensure_pending_attribution(attribution)?;
        let resolution = self
            .approvals
            .resolve_source_outcome(outcome)
            .map_err(|_| AgentGovernanceError::ApprovalLifecycle)?;
        self.finish_resolution(resolution)
    }

    pub(crate) fn expire_due(
        &mut self,
        attribution: &AgentAttribution,
    ) -> AgentGovernanceResult<Option<AgentApprovalGovernanceOutcome>> {
        self.ensure_pending_attribution(attribution)?;
        let Some(resolution) = self.approvals.expire_due() else {
            return Ok(None);
        };
        self.finish_resolution(resolution).map(Some)
    }

    pub(crate) fn cancel_pending_for_task(
        &mut self,
        attribution: &AgentAttribution,
    ) -> AgentGovernanceResult<Option<AgentApprovalGovernanceOutcome>> {
        if self.pending_approval.is_none() {
            return Ok(None);
        }
        self.ensure_pending_attribution(attribution)?;
        let id = self
            .pending_approval
            .as_ref()
            .map(|pending| pending.approval_id)
            .ok_or(AgentGovernanceError::NoPendingApproval)?;
        let resolution = self
            .approvals
            .cancel_for_run_termination(id)
            .map_err(|_| AgentGovernanceError::ApprovalLifecycle)?;
        self.finish_resolution(resolution).map(Some)
    }

    pub(crate) fn begin_delegation(
        &mut self,
        attribution: AgentAttribution,
        target: AgentId,
    ) -> AgentGovernanceResult<AgentDelegationAuditReservation> {
        self.audit.reserve_delegation(attribution, target)
    }

    #[must_use]
    pub(crate) const fn evaluate_delegation(
        &self,
        source: AgentId,
        target: AgentId,
    ) -> DelegationMatrixOutcome {
        let _ = self.delegation_matrix;
        DelegationMatrix::evaluate(source, target)
    }

    pub(crate) fn deny_delegation(
        &mut self,
        reservation: AgentDelegationAuditReservation,
        matrix: DelegationMatrixOutcome,
        error: AgentGovernanceErrorCode,
    ) {
        self.audit
            .commit_delegation_denied(reservation, matrix, error);
    }

    pub(crate) fn allow_delegation(
        &mut self,
        reservation: AgentDelegationAuditReservation,
    ) -> AgentPendingDelegationAuditToken {
        self.audit.commit_delegation_allowed(reservation)
    }

    pub(crate) fn finish_delegation(
        &mut self,
        token: AgentPendingDelegationAuditToken,
        control: AgentControlResult,
        error: Option<AgentGovernanceErrorCode>,
    ) {
        self.audit.finish_delegation(token, control, error);
    }

    #[must_use]
    pub fn audit_records(&self) -> Vec<AgentGovernanceRecord> {
        self.audit.records()
    }

    fn ensure_pending_attribution(
        &self,
        attribution: &AgentAttribution,
    ) -> AgentGovernanceResult<()> {
        match self.pending_approval.as_ref() {
            Some(pending) if pending.attribution == *attribution => Ok(()),
            Some(_) => Err(AgentGovernanceError::ContextMismatch),
            None => Err(AgentGovernanceError::NoPendingApproval),
        }
    }

    fn finish_pending_disposition(
        &mut self,
        disposition: AgentApprovalAuditDisposition,
        error: Option<AgentGovernanceErrorCode>,
    ) {
        if let Some(pending) = self.pending_approval.take() {
            self.audit
                .resolve_tool_approval(pending.audit_token, disposition, error);
        }
    }

    fn finish_resolution(
        &mut self,
        resolution: ApprovalResolution,
    ) -> AgentGovernanceResult<AgentApprovalGovernanceOutcome> {
        let id = resolution.id();
        let disposition = resolution.disposition();
        let result = AgentApprovalGovernanceOutcome::from_disposition(id, disposition);
        self.finish_pending_disposition(result.disposition(), None);
        Ok(result)
    }
}

impl fmt::Debug for AgentGovernanceService {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentGovernanceService")
            .field("profile_count", &self.profiles.list().count())
            .field("tool_count", &self.tools.list().len())
            .field("has_pending_approval", &self.pending_approval.is_some())
            .field("audit_record_count", &self.audit.record_count())
            .finish()
    }
}

impl AgentApprovalGovernanceOutcome {
    fn from_disposition(approval_id: ApprovalId, disposition: ApprovalDisposition) -> Self {
        let disposition = match disposition {
            ApprovalDisposition::Approved => AgentApprovalAuditDisposition::Approved,
            ApprovalDisposition::Rejected => AgentApprovalAuditDisposition::Rejected,
            ApprovalDisposition::Cancelled(_) => AgentApprovalAuditDisposition::Cancelled,
            ApprovalDisposition::Expired => AgentApprovalAuditDisposition::Expired,
        };
        Self {
            approval_id,
            disposition,
            execution: AgentExecutionDisposition::NotAttempted,
        }
    }

    #[must_use]
    pub const fn approval_id(self) -> ApprovalId {
        self.approval_id
    }

    #[must_use]
    pub const fn disposition(self) -> AgentApprovalAuditDisposition {
        self.disposition
    }

    #[must_use]
    pub const fn execution(self) -> AgentExecutionDisposition {
        self.execution
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentGovernanceAction {
    GetCurrentDatetime,
    CreateLocalTask,
    UnknownTool,
}

impl AgentGovernanceAction {
    #[must_use]
    pub const fn from_schema(schema: ToolSchema) -> Self {
        match schema {
            ToolSchema::GetCurrentDatetimeV1 => Self::GetCurrentDatetime,
            ToolSchema::CreateLocalTaskV1 => Self::CreateLocalTask,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentPolicyAuditOutcome {
    NotEvaluated,
    Evaluated {
        outcome: PolicyOutcome,
        reason: AgentPolicyReason,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentApprovalAuditDisposition {
    NotRequired,
    Pending,
    Approved,
    Rejected,
    Cancelled,
    Expired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentExecutionDisposition {
    NotAttempted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentControlResult {
    NotCreated,
    ChildCreated,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DelegationMatrixOutcome {
    Allowed,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentGovernanceErrorCode {
    UnknownTool,
    ToolContractVersionMismatch,
    InvalidArguments,
    ProfileNotEligible,
    ApprovalRequestFailed,
    ApprovalResolutionFailed,
    ApprovalExpired,
    DelegationDenied,
    ApprovalPending,
    UnauthorizedSource,
    TargetUnknown,
    TargetDeferred,
    DepthExceeded,
    ActiveChildLimitExceeded,
    TotalChildLimitExceeded,
    DelegationAfterRuntimeOutput,
    EventLimitExceeded,
    RunLimitExceeded,
    RuntimeCancellationFailed,
    ChildStartFailed,
    TaskMutationFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DelegationMatrix;

impl DelegationMatrix {
    #[must_use]
    pub const fn evaluate(source: AgentId, target: AgentId) -> DelegationMatrixOutcome {
        if matches!(source, AgentId::PersonalAssistant) && matches!(target, AgentId::Research) {
            DelegationMatrixOutcome::Allowed
        } else {
            DelegationMatrixOutcome::Denied
        }
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum AgentGovernanceError {
    #[error("agent governance built-in configuration is invalid")]
    GovernanceConfiguration,
    #[error("agent policy profile is duplicated")]
    DuplicateProfile { profile_id: AgentPolicyProfileId },
    #[error("agent policy profile is missing")]
    MissingProfile { profile_id: AgentPolicyProfileId },
    #[error("agent policy profile does not match the resolved definition")]
    ProfileMismatch,
    #[error("agent governance context does not match the live task and run")]
    ContextMismatch,
    #[error("agent governance requires a live task run")]
    NoActiveRun,
    #[error("agent governance approval is already pending")]
    ApprovalPending,
    #[error("agent governance has no pending approval")]
    NoPendingApproval,
    #[error("agent governance approval expired")]
    ApprovalExpired,
    #[error("tool call id is invalid")]
    InvalidToolCallId,
    #[error("tool name is invalid")]
    InvalidToolName,
    #[error("tool contract version is invalid")]
    InvalidToolContractVersion,
    #[error("tool arguments exceed {maximum} bytes")]
    ToolArgumentsTooLarge { maximum: usize, actual: usize },
    #[error("tool is not registered")]
    UnknownTool,
    #[error("tool contract version does not match the local schema")]
    ToolContractVersionMismatch { expected: u16, actual: u16 },
    #[error("tool arguments failed local schema validation")]
    InvalidToolArguments,
    #[error("agent policy profile is not eligible for this tool")]
    ProfileNotEligible,
    #[error("governance subject has already been consumed")]
    DuplicateSubject,
    #[error("governance audit capacity is exhausted")]
    AuditCapacityExhausted { maximum: usize },
    #[error("governance audit sequence space is exhausted")]
    AuditSequenceExhausted,
    #[error("approval lifecycle failed")]
    ApprovalLifecycle,
    #[error("delegation is not permitted")]
    DelegationDenied,
}

#[cfg(test)]
mod tests {
    use super::{
        AgentGovernanceError, AgentGovernanceService, AgentPolicyProfile, AgentPolicyProfileId,
        AgentPolicyProfileRegistry, AgentToolGovernanceOutcome, AgentToolProposal,
        DelegationMatrix, DelegationMatrixOutcome,
    };
    use crate::agent::{definition::AgentId, orchestrator::AgentOrchestrator};
    use crate::approvals::manager::ApprovalManager;
    use crate::memory::AgentMemoryProfileId;
    use crate::tools::types::ToolSchema;

    #[test]
    fn profiles_are_closed_and_personal_alone_has_current_eligibility(
    ) -> Result<(), AgentGovernanceError> {
        let registry = AgentPolicyProfileRegistry::built_in()?;
        assert_eq!(registry.list().count(), AgentPolicyProfileId::ALL.len());

        for profile_id in AgentPolicyProfileId::ALL {
            let profile = registry.get(profile_id)?;
            let expected = profile_id == AgentPolicyProfileId::PersonalAssistantV1;
            assert_eq!(profile.permits(ToolSchema::GetCurrentDatetimeV1), expected);
            assert_eq!(profile.permits(ToolSchema::CreateLocalTaskV1), expected);
        }
        Ok(())
    }

    #[test]
    fn profile_registry_rejects_missing_and_duplicate_profiles() {
        let missing = AgentPolicyProfileRegistry::from_profiles(
            AgentPolicyProfileId::ALL
                .into_iter()
                .filter(|profile| *profile != AgentPolicyProfileId::ResearchReadOnlyV1)
                .map(AgentPolicyProfile::new),
        );
        assert_eq!(
            missing,
            Err(AgentGovernanceError::MissingProfile {
                profile_id: AgentPolicyProfileId::ResearchReadOnlyV1,
            })
        );

        let duplicate = AgentPolicyProfileRegistry::from_profiles(
            AgentPolicyProfileId::ALL
                .into_iter()
                .chain([AgentPolicyProfileId::PersonalAssistantV1])
                .map(AgentPolicyProfile::new),
        );
        assert_eq!(
            duplicate,
            Err(AgentGovernanceError::DuplicateProfile {
                profile_id: AgentPolicyProfileId::PersonalAssistantV1,
            })
        );
    }

    #[test]
    fn proposal_debug_redacts_name_call_and_arguments() -> Result<(), AgentGovernanceError> {
        let call = "secret-call-id";
        let name = "secret_tool_name";
        let arguments = r#"{"secret":"value"}"#;
        let proposal = AgentToolProposal::new(call, name, 1, arguments)?;
        let debug = format!("{proposal:?}");
        assert!(!debug.contains(call));
        assert!(!debug.contains(name));
        assert!(!debug.contains(arguments));
        Ok(())
    }

    #[test]
    fn proposal_bounds_and_identifiers_fail_closed() {
        assert_eq!(
            AgentToolProposal::new("", "get_current_datetime", 1, "{}"),
            Err(AgentGovernanceError::InvalidToolCallId)
        );
        assert_eq!(
            AgentToolProposal::new("bounded-call", "unsafe name", 1, "{}"),
            Err(AgentGovernanceError::InvalidToolName)
        );
        assert_eq!(
            AgentToolProposal::new("bounded-call", "get_current_datetime", 0, "{}"),
            Err(AgentGovernanceError::InvalidToolContractVersion)
        );
        let oversized = "x".repeat(super::MAX_FUNCTION_ARGUMENT_BYTES + 1);
        assert_eq!(
            AgentToolProposal::new("bounded-call", "get_current_datetime", 1, oversized),
            Err(AgentGovernanceError::ToolArgumentsTooLarge {
                maximum: super::MAX_FUNCTION_ARGUMENT_BYTES,
                actual: super::MAX_FUNCTION_ARGUMENT_BYTES + 1,
            })
        );
    }

    #[test]
    fn delegation_matrix_allows_only_personal_to_research() {
        for source in AgentId::ALL {
            for target in AgentId::ALL {
                let expected = source == AgentId::PersonalAssistant && target == AgentId::Research;
                assert_eq!(
                    DelegationMatrix::evaluate(source, target),
                    if expected {
                        DelegationMatrixOutcome::Allowed
                    } else {
                        DelegationMatrixOutcome::Denied
                    }
                );
            }
        }
    }

    #[test]
    fn agent_approval_debug_redacts_origin_across_the_lifecycle(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut orchestrator = AgentOrchestrator::native()?;
        let context = orchestrator.start_root("Create one bounded local task")?;
        let task_id = context.task_id().as_str().to_owned();
        let root_task_id = context.root_task_id().task_id().as_str().to_owned();
        let run_id = context.runtime_run_identity().run_id().as_str().to_owned();
        let request_id = context
            .runtime_run_identity()
            .request_id()
            .as_str()
            .to_owned();

        let mut service = AgentGovernanceService::built_in()?;
        let attribution = orchestrator.live_attribution_for_test(&context)?;
        assert_eq!(
            attribution.memory_profile_id(),
            AgentMemoryProfileId::PersonalAssistantMemoryV1
        );
        assert_eq!(attribution, attribution.clone());
        assert!(format!("{attribution:?}").contains("PersonalAssistantMemoryV1"));
        let outcome = service.evaluate_tool(
            attribution,
            AgentToolProposal::new(
                "redacted-agent-call",
                "create_local_task",
                1,
                r#"{"title":"Sensitive task title"}"#,
            )?,
        )?;
        let AgentToolGovernanceOutcome::ApprovalPending { approval_id, .. } = outcome else {
            return Err("expected an approval-pending outcome".into());
        };

        let request_debug = {
            let pending = service
                .approvals
                .pending()?
                .ok_or("expected a pending agent approval")?;
            format!("{pending:?}")
        };
        let presentation_debug =
            format!("{:?}", service.approvals.issue_presentation(approval_id)?);
        let resolution_debug = format!(
            "{:?}",
            service.approvals.cancel_for_run_termination(approval_id)?
        );

        for debug in [request_debug, presentation_debug, resolution_debug] {
            assert!(debug.contains("Agent(\"[REDACTED]\")"));
            assert!(!debug.contains(&task_id));
            assert!(!debug.contains(&root_task_id));
            assert!(!debug.contains(&run_id));
            assert!(!debug.contains(&request_id));
            assert!(!debug.contains("PersonalAssistantV1"));
            assert!(!debug.contains("Sensitive task title"));
        }
        Ok(())
    }
}
