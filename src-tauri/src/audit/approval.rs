use std::fmt;

use thiserror::Error;

use crate::approvals::types::{
    ApprovalAuthenticationEvidence, ApprovalCancellationReason, ApprovalDisposition, ApprovalId,
    ApprovalInteractionEvidence, ApprovalInteractionSource, ApprovalNativeButton, ApprovalOrigin,
    ApprovalResolution,
};
use crate::policy::types::{PolicyOutcome, PolicyReason};
use crate::tools::types::{PermissionKind, RiskClass};

pub const MAX_APPROVAL_AUDIT_RECORDS: usize = 1_024;

const APPROVAL_TOOL_NAME: &str = "create_local_task";
const APPROVAL_TOOL_CONTRACT_VERSION: u16 = 1;

pub type ApprovalAuditResult<T> = Result<T, ApprovalAuditError>;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApprovalAuditSequence(u64);

impl ApprovalAuditSequence {
    #[must_use]
    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApprovalAuditReceipt {
    sequence: ApprovalAuditSequence,
}

impl ApprovalAuditReceipt {
    #[must_use]
    pub fn sequence(self) -> ApprovalAuditSequence {
        self.sequence
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ApprovalAuditTool {
    CreateLocalTaskV1,
}

impl ApprovalAuditTool {
    fn name(self) -> &'static str {
        match self {
            Self::CreateLocalTaskV1 => APPROVAL_TOOL_NAME,
        }
    }

    fn contract_version(self) -> u16 {
        match self {
            Self::CreateLocalTaskV1 => APPROVAL_TOOL_CONTRACT_VERSION,
        }
    }
}

pub struct ApprovalAuditRecord {
    sequence: ApprovalAuditSequence,
    approval_id: ApprovalId,
    run_id: String,
    gateway_request_id: String,
    call_id: String,
    tool: ApprovalAuditTool,
    risk_class: RiskClass,
    required_permission: PermissionKind,
    policy_outcome: PolicyOutcome,
    policy_reason: PolicyReason,
    disposition: ApprovalDisposition,
    interaction_evidence: Option<ApprovalInteractionEvidence>,
}

impl ApprovalAuditRecord {
    fn from_resolution(sequence: ApprovalAuditSequence, resolution: &ApprovalResolution) -> Self {
        Self {
            sequence,
            approval_id: resolution.id(),
            run_id: resolution.run_id().to_owned(),
            gateway_request_id: resolution.gateway_request_id().to_owned(),
            call_id: resolution.call_id().to_owned(),
            tool: ApprovalAuditTool::CreateLocalTaskV1,
            risk_class: resolution.risk_class(),
            required_permission: resolution.required_permission(),
            policy_outcome: resolution.policy_outcome(),
            policy_reason: resolution.policy_reason(),
            disposition: resolution.disposition(),
            interaction_evidence: resolution.interaction_evidence(),
        }
    }

    fn has_same_subject(&self, resolution: &ApprovalResolution) -> bool {
        self.approval_id == resolution.id()
            && self.run_id == resolution.run_id()
            && self.gateway_request_id == resolution.gateway_request_id()
            && self.call_id == resolution.call_id()
    }

    #[must_use]
    pub fn sequence(&self) -> ApprovalAuditSequence {
        self.sequence
    }

    #[must_use]
    pub fn approval_id(&self) -> ApprovalId {
        self.approval_id
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
    pub fn tool_name(&self) -> &'static str {
        self.tool.name()
    }

    #[must_use]
    pub fn tool_contract_version(&self) -> u16 {
        self.tool.contract_version()
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
    pub fn policy_outcome(&self) -> PolicyOutcome {
        self.policy_outcome
    }

    #[must_use]
    pub fn policy_reason(&self) -> PolicyReason {
        self.policy_reason
    }

    #[must_use]
    pub fn disposition(&self) -> ApprovalDisposition {
        self.disposition
    }

    #[must_use]
    pub fn interaction_evidence(&self) -> Option<ApprovalInteractionEvidence> {
        self.interaction_evidence
    }
}

impl fmt::Debug for ApprovalAuditRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovalAuditRecord")
            .field("sequence", &self.sequence)
            .field("approval_id", &self.approval_id)
            .field("identity", &"[REDACTED]")
            .field("tool_name", &self.tool.name())
            .field("tool_contract_version", &self.tool.contract_version())
            .field("risk_class", &self.risk_class)
            .field("required_permission", &self.required_permission)
            .field("policy_outcome", &self.policy_outcome)
            .field("policy_reason", &self.policy_reason)
            .field("disposition", &self.disposition)
            .field("interaction_evidence", &self.interaction_evidence)
            .finish()
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ApprovalAuditError {
    #[error("approval resolution has unsupported trusted facts")]
    UnsupportedResolutionFacts,
    #[error("approval resolution interaction evidence is inconsistent")]
    InconsistentInteractionEvidence,
    #[error("approval resolution has already been recorded")]
    DuplicateResolution,
    #[error("approval audit capacity is exhausted (maximum {maximum})")]
    CapacityExhausted { maximum: usize },
    #[error("approval audit sequence space is exhausted")]
    SequenceSpaceExhausted,
}

#[derive(Default)]
pub struct InMemoryApprovalAuditAdapter {
    next_sequence: u64,
    records: Vec<ApprovalAuditRecord>,
}

impl InMemoryApprovalAuditAdapter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(
        &mut self,
        resolution: &ApprovalResolution,
    ) -> ApprovalAuditResult<ApprovalAuditReceipt> {
        validate_resolution(resolution)?;

        if self
            .records
            .iter()
            .any(|record| record.has_same_subject(resolution))
        {
            return Err(ApprovalAuditError::DuplicateResolution);
        }

        if self.records.len() >= MAX_APPROVAL_AUDIT_RECORDS {
            return Err(ApprovalAuditError::CapacityExhausted {
                maximum: MAX_APPROVAL_AUDIT_RECORDS,
            });
        }

        let Some(next_sequence) = self.next_sequence.checked_add(1) else {
            return Err(ApprovalAuditError::SequenceSpaceExhausted);
        };
        let sequence = ApprovalAuditSequence(next_sequence);
        let record = ApprovalAuditRecord::from_resolution(sequence, resolution);

        self.records.push(record);
        self.next_sequence = next_sequence;
        Ok(ApprovalAuditReceipt { sequence })
    }

    #[must_use]
    pub fn records(&self) -> &[ApprovalAuditRecord] {
        &self.records
    }
}

impl fmt::Debug for InMemoryApprovalAuditAdapter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryApprovalAuditAdapter")
            .field("next_sequence", &self.next_sequence)
            .field("record_count", &self.records.len())
            .finish()
    }
}

fn validate_resolution(resolution: &ApprovalResolution) -> ApprovalAuditResult<()> {
    if resolution.origin() != &ApprovalOrigin::LegacyGateway
        || resolution.tool_name() != APPROVAL_TOOL_NAME
        || resolution.tool_contract_version() != APPROVAL_TOOL_CONTRACT_VERSION
        || resolution.risk_class() != RiskClass::ReversibleLocalAction
        || resolution.required_permission() != PermissionKind::None
        || resolution.policy_outcome() != PolicyOutcome::RequireApproval
        || resolution.policy_reason() != PolicyReason::ReversibleRequiresApproval
    {
        return Err(ApprovalAuditError::UnsupportedResolutionFacts);
    }

    if has_consistent_interaction_evidence(
        resolution.disposition(),
        resolution.interaction_evidence(),
    ) {
        Ok(())
    } else {
        Err(ApprovalAuditError::InconsistentInteractionEvidence)
    }
}

fn has_consistent_interaction_evidence(
    disposition: ApprovalDisposition,
    evidence: Option<ApprovalInteractionEvidence>,
) -> bool {
    match (disposition, evidence) {
        (ApprovalDisposition::Approved, Some(evidence)) => {
            is_recognized_button(evidence, ApprovalNativeButton::Approve)
        }
        (ApprovalDisposition::Rejected, Some(evidence)) => {
            is_recognized_button(evidence, ApprovalNativeButton::Reject)
        }
        (
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::EditRequested),
            Some(evidence),
        ) => is_recognized_button(evidence, ApprovalNativeButton::Edit),
        (
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::NativeNoDecision),
            Some(evidence),
        ) => {
            has_expected_source_and_authentication(evidence)
                && evidence.native_button().is_none()
                && evidence.source_failure().is_none()
        }
        (
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed),
            Some(evidence),
        ) => {
            has_expected_source_and_authentication(evidence)
                && evidence.native_button().is_none()
                && evidence.source_failure().is_some()
        }
        (
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
            | ApprovalDisposition::Expired,
            None,
        ) => true,
        _ => false,
    }
}

fn is_recognized_button(
    evidence: ApprovalInteractionEvidence,
    expected_button: ApprovalNativeButton,
) -> bool {
    has_expected_source_and_authentication(evidence)
        && evidence.native_button() == Some(expected_button)
        && evidence.source_failure().is_none()
}

fn has_expected_source_and_authentication(evidence: ApprovalInteractionEvidence) -> bool {
    evidence.source() == ApprovalInteractionSource::MacOsNativeDialog
        && evidence.authentication() == ApprovalAuthenticationEvidence::NotEvaluated
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;

    use super::{
        ApprovalAuditError, ApprovalAuditRecord, ApprovalAuditSequence, ApprovalAuditTool,
        InMemoryApprovalAuditAdapter, MAX_APPROVAL_AUDIT_RECORDS,
    };
    use crate::agent::function_call_validation::validate_function_call;
    use crate::agent::gateway_protocol::{
        GatewayProtocolError, GatewayStreamValidator, ValidatedGatewayEvent,
        GATEWAY_PROTOCOL_VERSION,
    };
    use crate::approvals::types::{
        ApprovalCancellationReason, ApprovalDisposition, ApprovalId, ApprovalResolution,
    };
    use crate::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
    use crate::policy::types::{PolicyDecision, PolicyInput, PolicyOutcome, PolicyReason};
    use crate::tools::registry::{InMemoryToolRegistry, ToolRegistry, ToolRegistryResult};
    use crate::tools::types::{PermissionKind, RiskClass, ToolDefinition, ToolSchema};

    const TOOL_CONTRACT_VERSION: u16 = 1;

    fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
        let mut registry = InMemoryToolRegistry::new();
        registry.register(ToolDefinition::from_schema(
            ToolSchema::GetCurrentDatetimeV1,
        ))?;
        registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
        Ok(registry)
    }

    fn policy_decision(
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
                "provider_response_id": "provider-response-approval-audit-test-1",
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

    fn local_task_resolution(
        approval_id: u64,
        run_id: &str,
        gateway_request_id: &str,
        call_id: &str,
        title: &str,
        disposition: ApprovalDisposition,
    ) -> Result<ApprovalResolution, Box<dyn Error>> {
        let decision = policy_decision(
            run_id,
            gateway_request_id,
            call_id,
            "create_local_task",
            &json!({ "title": title }).to_string(),
        )?;
        Ok(ApprovalResolution::new(
            ApprovalId::new(approval_id),
            disposition,
            decision,
            None,
        ))
    }

    fn stored_record(sequence: u64) -> ApprovalAuditRecord {
        ApprovalAuditRecord {
            sequence: ApprovalAuditSequence(sequence),
            approval_id: ApprovalId::new(sequence),
            run_id: format!("run-capacity-{sequence}"),
            gateway_request_id: format!("gateway-request-capacity-{sequence}"),
            call_id: format!("call-capacity-{sequence}"),
            tool: ApprovalAuditTool::CreateLocalTaskV1,
            risk_class: RiskClass::ReversibleLocalAction,
            required_permission: PermissionKind::None,
            policy_outcome: PolicyOutcome::RequireApproval,
            policy_reason: PolicyReason::ReversibleRequiresApproval,
            disposition: ApprovalDisposition::Expired,
            interaction_evidence: None,
        }
    }

    #[test]
    fn records_exact_redacted_run_termination_without_preview_content() -> Result<(), Box<dyn Error>>
    {
        let title = "Board password=private-planning-title";
        let run_id = "run-audit-redaction-1";
        let gateway_request_id = "gateway-request-audit-redaction-1";
        let call_id = "call-audit-redaction-1";
        let resolution = local_task_resolution(
            7,
            run_id,
            gateway_request_id,
            call_id,
            title,
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated),
        )?;
        let mut adapter = InMemoryApprovalAuditAdapter::new();

        let receipt = adapter.record(&resolution)?;
        assert_eq!(receipt.sequence().value(), 1);
        let [record] = adapter.records() else {
            return Err("expected one approval audit record".into());
        };
        assert_eq!(record.sequence(), receipt.sequence());
        assert_eq!(record.approval_id(), resolution.id());
        assert_eq!(record.run_id(), run_id);
        assert_eq!(record.gateway_request_id(), gateway_request_id);
        assert_eq!(record.call_id(), call_id);
        assert_eq!(record.tool_name(), "create_local_task");
        assert_eq!(record.tool_contract_version(), TOOL_CONTRACT_VERSION);
        assert_eq!(record.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(record.required_permission(), PermissionKind::None);
        assert_eq!(record.policy_outcome(), PolicyOutcome::RequireApproval);
        assert_eq!(
            record.policy_reason(),
            PolicyReason::ReversibleRequiresApproval
        );
        assert_eq!(record.disposition(), resolution.disposition());
        assert_eq!(record.interaction_evidence(), None);

        let record_debug = format!("{record:?}");
        let adapter_debug = format!("{adapter:?}");
        for excluded in [title, run_id, gateway_request_id, call_id] {
            assert!(!record_debug.contains(excluded));
            assert!(!adapter_debug.contains(excluded));
        }
        Ok(())
    }

    #[test]
    fn records_expiry_and_assigns_contiguous_sequences() -> Result<(), Box<dyn Error>> {
        let expired = local_task_resolution(
            1,
            "run-audit-sequence-1",
            "gateway-request-audit-sequence-1",
            "call-audit-sequence-1",
            "First title",
            ApprovalDisposition::Expired,
        )?;
        let cancelled = local_task_resolution(
            2,
            "run-audit-sequence-2",
            "gateway-request-audit-sequence-2",
            "call-audit-sequence-2",
            "Second title",
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated),
        )?;
        let mut adapter = InMemoryApprovalAuditAdapter::new();

        assert_eq!(adapter.record(&expired)?.sequence().value(), 1);
        assert_eq!(adapter.record(&cancelled)?.sequence().value(), 2);
        assert_eq!(adapter.records().len(), 2);
        assert_eq!(
            adapter.records()[0].disposition(),
            ApprovalDisposition::Expired
        );
        assert_eq!(adapter.records()[1].interaction_evidence(), None);
        Ok(())
    }

    #[test]
    fn rejects_duplicate_without_consuming_sequence_or_mutating_records(
    ) -> Result<(), Box<dyn Error>> {
        let first = local_task_resolution(
            1,
            "run-audit-duplicate-1",
            "gateway-request-audit-duplicate-1",
            "call-audit-duplicate-1",
            "First title",
            ApprovalDisposition::Expired,
        )?;
        let second = local_task_resolution(
            2,
            "run-audit-duplicate-2",
            "gateway-request-audit-duplicate-2",
            "call-audit-duplicate-2",
            "Second title",
            ApprovalDisposition::Expired,
        )?;
        let mut adapter = InMemoryApprovalAuditAdapter::new();

        assert_eq!(adapter.record(&first)?.sequence().value(), 1);
        assert_eq!(
            adapter.record(&first),
            Err(ApprovalAuditError::DuplicateResolution)
        );
        assert_eq!(adapter.records().len(), 1);
        assert_eq!(adapter.record(&second)?.sequence().value(), 2);
        Ok(())
    }

    #[test]
    fn rejects_unsupported_or_inconsistent_resolution_before_mutation() -> Result<(), Box<dyn Error>>
    {
        let approved_without_evidence = local_task_resolution(
            1,
            "run-audit-invalid-1",
            "gateway-request-audit-invalid-1",
            "call-audit-invalid-1",
            "Missing evidence title",
            ApprovalDisposition::Approved,
        )?;
        let no_decision_without_evidence = local_task_resolution(
            2,
            "run-audit-invalid-2",
            "gateway-request-audit-invalid-2",
            "call-audit-invalid-2",
            "Missing no-decision evidence title",
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::NativeNoDecision),
        )?;
        let information_decision = policy_decision(
            "run-audit-information-1",
            "gateway-request-audit-information-1",
            "call-audit-information-1",
            "get_current_datetime",
            "{}",
        )?;
        let unsupported = ApprovalResolution::new(
            ApprovalId::new(3),
            ApprovalDisposition::Expired,
            information_decision,
            None,
        );
        let mut adapter = InMemoryApprovalAuditAdapter::new();

        assert_eq!(
            adapter.record(&approved_without_evidence),
            Err(ApprovalAuditError::InconsistentInteractionEvidence)
        );
        assert_eq!(
            adapter.record(&no_decision_without_evidence),
            Err(ApprovalAuditError::InconsistentInteractionEvidence)
        );
        assert_eq!(
            adapter.record(&unsupported),
            Err(ApprovalAuditError::UnsupportedResolutionFacts)
        );
        assert!(adapter.records().is_empty());
        assert_eq!(
            format!("{adapter:?}"),
            "InMemoryApprovalAuditAdapter { next_sequence: 0, record_count: 0 }"
        );
        Ok(())
    }

    #[test]
    fn rejects_capacity_without_eviction_or_partial_mutation() -> Result<(), Box<dyn Error>> {
        let records = (1_u64..=1_024_u64).map(stored_record).collect();
        let mut adapter = InMemoryApprovalAuditAdapter {
            next_sequence: 1_024,
            records,
        };
        let overflow = local_task_resolution(
            1_025,
            "run-audit-capacity-overflow",
            "gateway-request-audit-capacity-overflow",
            "call-audit-capacity-overflow",
            "Overflow title",
            ApprovalDisposition::Expired,
        )?;

        assert_eq!(adapter.records().len(), MAX_APPROVAL_AUDIT_RECORDS);
        assert_eq!(
            adapter.record(&overflow),
            Err(ApprovalAuditError::CapacityExhausted {
                maximum: MAX_APPROVAL_AUDIT_RECORDS,
            })
        );
        assert_eq!(adapter.records().len(), MAX_APPROVAL_AUDIT_RECORDS);
        assert_eq!(adapter.records()[0].sequence().value(), 1);
        assert_eq!(
            adapter.records()[MAX_APPROVAL_AUDIT_RECORDS - 1]
                .sequence()
                .value(),
            1_024
        );
        Ok(())
    }

    #[test]
    fn rejects_sequence_exhaustion_without_mutation() -> Result<(), Box<dyn Error>> {
        let resolution = local_task_resolution(
            1,
            "run-audit-sequence-overflow",
            "gateway-request-audit-sequence-overflow",
            "call-audit-sequence-overflow",
            "Sequence overflow title",
            ApprovalDisposition::Expired,
        )?;
        let mut adapter = InMemoryApprovalAuditAdapter {
            next_sequence: u64::MAX,
            records: Vec::new(),
        };

        assert_eq!(
            adapter.record(&resolution),
            Err(ApprovalAuditError::SequenceSpaceExhausted)
        );
        assert!(adapter.records().is_empty());
        Ok(())
    }
}
