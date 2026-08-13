//! Strict contracts for the sealed fixture-only infrastructure workflows.
//!
//! This module owns immutable synthetic fixtures, bounded structured results,
//! and content-free lifecycle evidence for the Cloud Infrastructure and
//! Systems Operations workflows. It performs no I/O and grants no tool,
//! credential, approval, runtime, provider, host, or execution authority.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

mod catalog;
mod framing;
mod validation;

#[cfg(test)]
use catalog::cloud_fixture_request_data;
#[cfg(test)]
use framing::build_synthesis_stage_input_from_transfers;
use framing::{
    assessment_references, build_catalog_stage_input, build_synthesis_stage_input, cloud_transfer,
    infrastructure_partial_failure_codes, qa_transfer, systems_partial_failure_codes,
    systems_transfer,
};
#[cfg(test)]
use validation::{
    contains_effect_token_sequence, normalized_claim_tokens, validate_approval_summary_consistency,
    validate_cloud_capability_consistency, validate_model_text, validate_no_effect_requests,
    validate_systems_capability_consistency,
};
use validation::{
    ensure_count, ensure_unique, ensure_unique_by, parse_cloud_synthesis,
    parse_infrastructure_assessment, parse_operational_assessment, parse_risk_assessment,
    parse_systems_synthesis, parse_validation_report, reject_credential_text, validate_text,
};

use super::{
    definition::AgentId,
    governance::AgentPolicyProfileId,
    runtime::{RuntimeId, RuntimeRunIdentity},
    task::{AgentExecutionContext, AgentTaskFailureCode, AgentTaskId, ParentTaskId, RootTaskId},
};
use crate::memory::AgentMemoryProfileId;

pub const MAX_INFRASTRUCTURE_OPERATIONS_FIXTURES: usize = 8;
pub const MAX_INFRASTRUCTURE_OPERATIONS_CRITERIA: usize = 8;
pub const MAX_INFRASTRUCTURE_OPERATIONS_EVIDENCE: usize = 8;
pub const MAX_INFRASTRUCTURE_OPERATIONS_ITEMS: usize = 8;
pub const MAX_INFRASTRUCTURE_OPERATIONS_REFERENCES: usize = 8;
pub const MAX_INFRASTRUCTURE_OPERATIONS_ID_BYTES: usize = 64;
pub const MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_CHARACTERS: usize = 2_048;
pub const MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_BYTES: usize = 4_096;
pub const MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_TOTAL_BYTES: usize = 16_384;
pub const MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_CHARACTERS: usize = 2_048;
pub const MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_BYTES: usize = 4_096;
pub const MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES: usize = 24_576;
pub const MAX_INFRASTRUCTURE_OPERATIONS_RESULT_CHARACTERS: usize = 8_192;
pub const MAX_INFRASTRUCTURE_OPERATIONS_RESULT_BYTES: usize = 16_384;
pub const MAX_INFRASTRUCTURE_OPERATIONS_TRANSFER_BYTES: usize = 6_144;
pub const MAX_INFRASTRUCTURE_OPERATIONS_WORKFLOW_EVENTS: usize = 16;
pub const MAX_INFRASTRUCTURE_OPERATIONS_ATTRIBUTION_RECORDS: usize = 16;

const MAX_RESULT_TEXT_CHARACTERS: usize = 1_024;
const MAX_RESULT_TEXT_BYTES: usize = 2_048;
const BUILT_IN_CATALOG_VERSION: u16 = 1;

pub type InfrastructureOperationsResult<T> = Result<T, InfrastructureOperationsError>;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InfrastructureOperationsId(String);

impl InfrastructureOperationsId {
    pub fn new(value: impl Into<String>) -> InfrastructureOperationsResult<Self> {
        let value = value.into();
        if value.is_empty()
            || value.len() > MAX_INFRASTRUCTURE_OPERATIONS_ID_BYTES
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InfrastructureOperationsError::InvalidIdentifier);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for InfrastructureOperationsId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("InfrastructureOperationsId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CloudScenarioId {
    TerraformDecisionBriefV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemsOperationsScenarioId {
    SanitizedServiceRecoveryV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsWorkflowKind {
    CloudInfrastructureV1,
    SystemsOperationsV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CloudFixtureKind {
    TerraformConfiguration,
    AzureArchitecture,
    AwsArchitecture,
    ApprovedInventorySnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemsFixtureKind {
    WindowsSnapshot,
    LinuxSnapshot,
    MacosSnapshot,
    ServiceSnapshot,
    ProcessSnapshot,
    SanitizedLogExcerpt,
    ConfigurationSnapshot,
    ResourceSnapshot,
    VmwareInventorySnapshot,
    BackupSnapshot,
    RecoveryScenario,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsFixtureKind {
    Cloud(CloudFixtureKind),
    Systems(SystemsFixtureKind),
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsFixture {
    id: InfrastructureOperationsId,
    kind: InfrastructureOperationsFixtureKind,
    label: String,
    content: String,
}

impl InfrastructureOperationsFixture {
    fn new(
        id: impl Into<String>,
        kind: InfrastructureOperationsFixtureKind,
        label: impl Into<String>,
        content: impl Into<String>,
    ) -> InfrastructureOperationsResult<Self> {
        let label = label.into();
        let content = content.into();
        validate_text(
            &label,
            MAX_RESULT_TEXT_CHARACTERS,
            MAX_RESULT_TEXT_BYTES,
            false,
        )?;
        validate_text(
            &content,
            MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_CHARACTERS,
            MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_BYTES,
            true,
        )?;
        reject_credential_text(&label)?;
        reject_credential_text(&content)?;
        Ok(Self {
            id: InfrastructureOperationsId::new(id)?,
            kind,
            label,
            content,
        })
    }

    #[must_use]
    pub fn id(&self) -> &InfrastructureOperationsId {
        &self.id
    }

    #[must_use]
    pub const fn kind(&self) -> InfrastructureOperationsFixtureKind {
        self.kind
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl fmt::Debug for InfrastructureOperationsFixture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InfrastructureOperationsFixture")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("label", &"[REDACTED]")
            .field("content", &"[REDACTED]")
            .field("content_bytes", &self.content.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsCriterion {
    id: InfrastructureOperationsId,
    text: String,
}

impl InfrastructureOperationsCriterion {
    fn new(id: impl Into<String>, text: impl Into<String>) -> InfrastructureOperationsResult<Self> {
        let text = text.into();
        validate_text(
            &text,
            MAX_RESULT_TEXT_CHARACTERS,
            MAX_RESULT_TEXT_BYTES,
            false,
        )?;
        reject_credential_text(&text)?;
        Ok(Self {
            id: InfrastructureOperationsId::new(id)?,
            text,
        })
    }

    #[must_use]
    pub fn id(&self) -> &InfrastructureOperationsId {
        &self.id
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Debug for InfrastructureOperationsCriterion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InfrastructureOperationsCriterion")
            .field("id", &self.id)
            .field("text", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsEvidenceStatus {
    ObservedFixture,
    NotRun,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraformExecutionDisclosure {
    fmt_check: InfrastructureOperationsEvidenceStatus,
    validate: InfrastructureOperationsEvidenceStatus,
    provider_initialization: InfrastructureOperationsEvidenceStatus,
    plan: InfrastructureOperationsEvidenceStatus,
    apply: InfrastructureOperationsEvidenceStatus,
}

impl TerraformExecutionDisclosure {
    fn from_not_run_fields(
        fmt_check: InfrastructureOperationsEvidenceStatus,
        validate: InfrastructureOperationsEvidenceStatus,
        provider_initialization: InfrastructureOperationsEvidenceStatus,
        plan: InfrastructureOperationsEvidenceStatus,
        apply: InfrastructureOperationsEvidenceStatus,
    ) -> InfrastructureOperationsResult<Self> {
        if [fmt_check, validate, provider_initialization, plan, apply]
            .iter()
            .any(|status| *status != InfrastructureOperationsEvidenceStatus::NotRun)
        {
            return Err(InfrastructureOperationsError::AuthorityClaim);
        }
        Ok(Self {
            fmt_check,
            validate,
            provider_initialization,
            plan,
            apply,
        })
    }

    #[must_use]
    pub const fn fmt_check(&self) -> InfrastructureOperationsEvidenceStatus {
        self.fmt_check
    }

    #[must_use]
    pub const fn validate(&self) -> InfrastructureOperationsEvidenceStatus {
        self.validate
    }

    #[must_use]
    pub const fn provider_initialization(&self) -> InfrastructureOperationsEvidenceStatus {
        self.provider_initialization
    }

    #[must_use]
    pub const fn plan(&self) -> InfrastructureOperationsEvidenceStatus {
        self.plan
    }

    #[must_use]
    pub const fn apply(&self) -> InfrastructureOperationsEvidenceStatus {
        self.apply
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsValidationEvidence {
    id: InfrastructureOperationsId,
    status: InfrastructureOperationsEvidenceStatus,
    description: String,
    criterion_ids: Vec<InfrastructureOperationsId>,
    fixture_ids: Vec<InfrastructureOperationsId>,
}

impl InfrastructureOperationsValidationEvidence {
    fn new(
        id: impl Into<String>,
        status: InfrastructureOperationsEvidenceStatus,
        description: impl Into<String>,
        criterion_ids: Vec<InfrastructureOperationsId>,
        fixture_ids: Vec<InfrastructureOperationsId>,
    ) -> InfrastructureOperationsResult<Self> {
        let description = description.into();
        validate_text(
            &description,
            MAX_RESULT_TEXT_CHARACTERS,
            MAX_RESULT_TEXT_BYTES,
            false,
        )?;
        reject_credential_text(&description)?;
        ensure_unique(&criterion_ids, 1, MAX_INFRASTRUCTURE_OPERATIONS_REFERENCES)?;
        ensure_unique(&fixture_ids, 0, MAX_INFRASTRUCTURE_OPERATIONS_REFERENCES)?;
        if status == InfrastructureOperationsEvidenceStatus::ObservedFixture
            && fixture_ids.is_empty()
        {
            return Err(InfrastructureOperationsError::EvidenceInconsistent);
        }
        Ok(Self {
            id: InfrastructureOperationsId::new(id)?,
            status,
            description,
            criterion_ids,
            fixture_ids,
        })
    }

    #[must_use]
    pub fn id(&self) -> &InfrastructureOperationsId {
        &self.id
    }

    #[must_use]
    pub const fn status(&self) -> InfrastructureOperationsEvidenceStatus {
        self.status
    }

    #[must_use]
    pub fn criterion_ids(&self) -> &[InfrastructureOperationsId] {
        &self.criterion_ids
    }

    #[must_use]
    pub fn fixture_ids(&self) -> &[InfrastructureOperationsId] {
        &self.fixture_ids
    }
}

impl fmt::Debug for InfrastructureOperationsValidationEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InfrastructureOperationsValidationEvidence")
            .field("id", &self.id)
            .field("status", &self.status)
            .field("description", &"[REDACTED]")
            .field("criterion_count", &self.criterion_ids.len())
            .field("fixture_count", &self.fixture_ids.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct CatalogProof {
    version: u16,
}

#[derive(Clone, Eq, PartialEq)]
struct WorkflowRequestData {
    catalog_version: u16,
    objective: String,
    fixtures: Vec<InfrastructureOperationsFixture>,
    criteria: Vec<InfrastructureOperationsCriterion>,
    evidence: Vec<InfrastructureOperationsValidationEvidence>,
}

impl WorkflowRequestData {
    fn new(
        objective: impl Into<String>,
        fixtures: Vec<InfrastructureOperationsFixture>,
        criteria: Vec<InfrastructureOperationsCriterion>,
        evidence: Vec<InfrastructureOperationsValidationEvidence>,
    ) -> InfrastructureOperationsResult<Self> {
        let objective = objective.into();
        validate_text(
            &objective,
            MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_CHARACTERS,
            MAX_INFRASTRUCTURE_OPERATIONS_OBJECTIVE_BYTES,
            true,
        )?;
        reject_credential_text(&objective)?;
        ensure_count(fixtures.len(), 1, MAX_INFRASTRUCTURE_OPERATIONS_FIXTURES)?;
        ensure_count(criteria.len(), 1, MAX_INFRASTRUCTURE_OPERATIONS_CRITERIA)?;
        ensure_count(evidence.len(), 0, MAX_INFRASTRUCTURE_OPERATIONS_EVIDENCE)?;
        ensure_unique_by(fixtures.iter().map(|item| &item.id))?;
        ensure_unique_by(criteria.iter().map(|item| &item.id))?;
        ensure_unique_by(evidence.iter().map(|item| &item.id))?;
        let total = fixtures.iter().try_fold(0_usize, |total, item| {
            total
                .checked_add(item.content.len())
                .ok_or(InfrastructureOperationsError::BoundExceeded)
        })?;
        if total > MAX_INFRASTRUCTURE_OPERATIONS_FIXTURE_TOTAL_BYTES {
            return Err(InfrastructureOperationsError::BoundExceeded);
        }
        for entry in &evidence {
            if !entry
                .criterion_ids
                .iter()
                .all(|id| criteria.iter().any(|criterion| criterion.id == *id))
                || !entry
                    .fixture_ids
                    .iter()
                    .all(|id| fixtures.iter().any(|fixture| fixture.id == *id))
            {
                return Err(InfrastructureOperationsError::UnknownReference);
            }
        }
        Ok(Self {
            catalog_version: BUILT_IN_CATALOG_VERSION,
            objective,
            fixtures,
            criteria,
            evidence,
        })
    }

    fn fixture_ids(&self) -> Vec<InfrastructureOperationsId> {
        self.fixtures.iter().map(|item| item.id.clone()).collect()
    }

    fn fixture(&self, id: &InfrastructureOperationsId) -> bool {
        self.fixtures.iter().any(|item| item.id == *id)
    }

    fn criterion(&self, id: &InfrastructureOperationsId) -> bool {
        self.criteria.iter().any(|item| item.id == *id)
    }

    fn evidence(
        &self,
        id: &InfrastructureOperationsId,
    ) -> Option<&InfrastructureOperationsValidationEvidence> {
        self.evidence.iter().find(|item| item.id == *id)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct CloudInfrastructureWorkflowRequest {
    scenario_id: CloudScenarioId,
    data: WorkflowRequestData,
    proof: CatalogProof,
}

#[derive(Clone, Eq, PartialEq)]
pub struct SystemsOperationsWorkflowRequest {
    scenario_id: SystemsOperationsScenarioId,
    data: WorkflowRequestData,
    proof: CatalogProof,
}

macro_rules! request_accessors {
    ($type:ty, $scenario:ty) => {
        impl $type {
            #[must_use]
            pub const fn scenario_id(&self) -> $scenario {
                self.scenario_id
            }

            #[must_use]
            pub fn objective(&self) -> &str {
                &self.data.objective
            }

            #[must_use]
            pub fn fixtures(&self) -> &[InfrastructureOperationsFixture] {
                &self.data.fixtures
            }

            #[must_use]
            pub fn criteria(&self) -> &[InfrastructureOperationsCriterion] {
                &self.data.criteria
            }

            #[must_use]
            pub fn evidence(&self) -> &[InfrastructureOperationsValidationEvidence] {
                &self.data.evidence
            }
        }
    };
}

request_accessors!(CloudInfrastructureWorkflowRequest, CloudScenarioId);
request_accessors!(
    SystemsOperationsWorkflowRequest,
    SystemsOperationsScenarioId
);

impl fmt::Debug for CloudInfrastructureWorkflowRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_request(formatter, "CloudInfrastructureWorkflowRequest", &self.data)
    }
}

impl fmt::Debug for SystemsOperationsWorkflowRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_request(formatter, "SystemsOperationsWorkflowRequest", &self.data)
    }
}

fn debug_request(
    formatter: &mut fmt::Formatter<'_>,
    name: &str,
    data: &WorkflowRequestData,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("catalog_version", &data.catalog_version)
        .field("objective", &"[REDACTED]")
        .field("fixture_count", &data.fixtures.len())
        .field("criterion_count", &data.criteria.len())
        .field("evidence_count", &data.evidence.len())
        .finish()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InfrastructureOperationsFixtureCatalog;

impl InfrastructureAssessment {
    pub(super) fn available_references(&self) -> Vec<InfrastructureOperationsEvidenceRef> {
        assessment_references(
            &self.fixture_ids,
            self.findings.iter().map(|item| item.id.clone()),
            self.change_plan
                .proposed_validation
                .iter()
                .map(|item| item.id.clone()),
        )
    }

    pub(super) fn evidence_references(&self) -> Vec<InfrastructureOperationsEvidenceRef> {
        assessment_references(
            &self.fixture_ids,
            self.findings.iter().map(|item| item.id.clone()),
            std::iter::empty(),
        )
    }
}

impl OperationalAssessment {
    pub(super) fn available_references(&self) -> Vec<InfrastructureOperationsEvidenceRef> {
        assessment_references(
            &self.fixture_ids,
            self.findings.iter().map(|item| item.id.clone()),
            std::iter::empty(),
        )
    }

    pub(super) fn evidence_references(&self) -> Vec<InfrastructureOperationsEvidenceRef> {
        assessment_references(
            &self.fixture_ids,
            self.findings
                .iter()
                .filter(|item| item.basis == DiagnosticFindingBasis::EvidenceBound)
                .map(|item| item.id.clone()),
            std::iter::empty(),
        )
    }
}

macro_rules! request_stage_builders {
    ($type:ty, $role:literal, $kind:expr) => {
        impl $type {
            pub(super) fn build_first_stage_input(&self) -> InfrastructureOperationsResult<String> {
                self.validate_catalog_binding()?;
                build_catalog_stage_input(&self.data, $role, "first-assessment", None, None)
            }

            pub(super) fn build_qa_input(
                &self,
                assessment_transfer: &str,
            ) -> InfrastructureOperationsResult<String> {
                build_catalog_stage_input(
                    &self.data,
                    "qa-validation",
                    "qa-validation",
                    Some(assessment_transfer),
                    None,
                )
            }

            pub(super) fn build_security_input(
                &self,
                assessment_transfer: &str,
                qa: &InfrastructureOperationsQaStageOutcome,
            ) -> InfrastructureOperationsResult<String> {
                build_catalog_stage_input(
                    &self.data,
                    "security-risk",
                    "security-review",
                    Some(assessment_transfer),
                    Some(qa_transfer(qa)),
                )
            }

            pub(super) fn parse_validation_report(
                &self,
                task_id: AgentTaskId,
                assessment_task_id: AgentTaskId,
                assessment_refs: &[InfrastructureOperationsEvidenceRef],
                raw: &str,
            ) -> InfrastructureOperationsResult<ValidationReport> {
                parse_validation_report(
                    &self.data,
                    $kind,
                    task_id,
                    assessment_task_id,
                    assessment_refs,
                    raw,
                )
            }

            pub(super) fn parse_risk_assessment(
                &self,
                task_id: AgentTaskId,
                assessment_task_id: AgentTaskId,
                assessment_refs: &[InfrastructureOperationsEvidenceRef],
                qa: &InfrastructureOperationsQaStageOutcome,
                raw: &str,
            ) -> InfrastructureOperationsResult<RiskAssessment> {
                parse_risk_assessment(
                    &self.data,
                    $kind,
                    task_id,
                    assessment_task_id,
                    assessment_refs,
                    qa,
                    raw,
                )
            }
        }
    };
}

request_stage_builders!(
    CloudInfrastructureWorkflowRequest,
    "cloud-infrastructure",
    InfrastructureOperationsWorkflowKind::CloudInfrastructureV1
);
request_stage_builders!(
    SystemsOperationsWorkflowRequest,
    "systems-operations",
    InfrastructureOperationsWorkflowKind::SystemsOperationsV1
);

impl CloudInfrastructureWorkflowRequest {
    pub(super) fn parse_assessment(
        &self,
        task_id: AgentTaskId,
        raw: &str,
    ) -> InfrastructureOperationsResult<InfrastructureAssessment> {
        parse_infrastructure_assessment(&self.data, task_id, raw)
    }

    pub(super) fn build_synthesis_input(
        &self,
        first: &CloudInfrastructureStageOutcome,
        qa: &InfrastructureOperationsQaStageOutcome,
        security: &InfrastructureOperationsSecurityStageOutcome,
    ) -> InfrastructureOperationsResult<String> {
        build_synthesis_stage_input(
            &self.data,
            cloud_transfer(first),
            qa,
            security,
            true,
            infrastructure_partial_failure_codes(first, qa, security),
        )
    }

    pub(super) fn parse_synthesis(
        &self,
        first: &CloudInfrastructureStageOutcome,
        qa: &InfrastructureOperationsQaStageOutcome,
        security: &InfrastructureOperationsSecurityStageOutcome,
        raw: &str,
    ) -> InfrastructureOperationsResult<CloudInfrastructureSynthesis> {
        parse_cloud_synthesis(&self.data, first, qa, security, raw)
    }
}

impl SystemsOperationsWorkflowRequest {
    pub(super) fn parse_assessment(
        &self,
        task_id: AgentTaskId,
        raw: &str,
    ) -> InfrastructureOperationsResult<OperationalAssessment> {
        parse_operational_assessment(&self.data, task_id, raw)
    }

    pub(super) fn build_synthesis_input(
        &self,
        first: &SystemsOperationsStageOutcome,
        qa: &InfrastructureOperationsQaStageOutcome,
        security: &InfrastructureOperationsSecurityStageOutcome,
    ) -> InfrastructureOperationsResult<String> {
        build_synthesis_stage_input(
            &self.data,
            systems_transfer(first),
            qa,
            security,
            false,
            systems_partial_failure_codes(first, qa, security),
        )
    }

    pub(super) fn parse_synthesis(
        &self,
        first: &SystemsOperationsStageOutcome,
        qa: &InfrastructureOperationsQaStageOutcome,
        security: &InfrastructureOperationsSecurityStageOutcome,
        raw: &str,
    ) -> InfrastructureOperationsResult<SystemsOperationsSynthesis> {
        parse_systems_synthesis(&self.data, first, qa, security, raw)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CloudInfrastructureCapability {
    ReviewTerraformFixture,
    AnalyzeAzureFixture,
    AnalyzeAwsFixture,
    AnalyzeInventoryFixture,
    ProposeInfrastructureChangePlan,
    TerraformPlanExecution,
    TerraformApply,
    TerraformStateMutation,
    TerraformBackendChange,
    ProviderDownload,
    CloudInventoryAccess,
    CloudShell,
    ResourceCreateOrUpdate,
    ResourceDelete,
    IamChange,
    FirewallChange,
    ProductionAccess,
    CredentialAccess,
    SecretRotation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemsOperationsCapability {
    AnalyzeServiceFixture,
    AnalyzeProcessFixture,
    AnalyzeLogFixture,
    AnalyzeConfigurationFixture,
    AnalyzeResourceFixture,
    AnalyzeVmwareFixture,
    AnalyzeBackupFixture,
    ProposeDiagnostics,
    ProposeRemediationPlan,
    LiveDiagnostic,
    ReadLiveLog,
    InspectLiveService,
    InspectLiveProcess,
    InspectLiveConfiguration,
    RestartOrStopService,
    RebootOrShutdown,
    KillProcess,
    ConfigurationMutation,
    PackageInstall,
    PatchSystem,
    AccountOrPermissionChange,
    DeleteFile,
    PrivilegedShell,
    VmwareMutation,
    BackupMutation,
    CredentialAccess,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsCapabilityDisposition {
    ProposalOnly,
    Denied,
}

impl CloudInfrastructureCapability {
    #[must_use]
    pub const fn disposition(self) -> InfrastructureOperationsCapabilityDisposition {
        match self {
            Self::ReviewTerraformFixture
            | Self::AnalyzeAzureFixture
            | Self::AnalyzeAwsFixture
            | Self::AnalyzeInventoryFixture
            | Self::ProposeInfrastructureChangePlan => {
                InfrastructureOperationsCapabilityDisposition::ProposalOnly
            }
            Self::TerraformPlanExecution
            | Self::TerraformApply
            | Self::TerraformStateMutation
            | Self::TerraformBackendChange
            | Self::ProviderDownload
            | Self::CloudInventoryAccess
            | Self::CloudShell
            | Self::ResourceCreateOrUpdate
            | Self::ResourceDelete
            | Self::IamChange
            | Self::FirewallChange
            | Self::ProductionAccess
            | Self::CredentialAccess
            | Self::SecretRotation => InfrastructureOperationsCapabilityDisposition::Denied,
        }
    }
}

impl SystemsOperationsCapability {
    #[must_use]
    pub const fn disposition(self) -> InfrastructureOperationsCapabilityDisposition {
        match self {
            Self::AnalyzeServiceFixture
            | Self::AnalyzeProcessFixture
            | Self::AnalyzeLogFixture
            | Self::AnalyzeConfigurationFixture
            | Self::AnalyzeResourceFixture
            | Self::AnalyzeVmwareFixture
            | Self::AnalyzeBackupFixture
            | Self::ProposeDiagnostics
            | Self::ProposeRemediationPlan => {
                InfrastructureOperationsCapabilityDisposition::ProposalOnly
            }
            Self::LiveDiagnostic
            | Self::ReadLiveLog
            | Self::InspectLiveService
            | Self::InspectLiveProcess
            | Self::InspectLiveConfiguration
            | Self::RestartOrStopService
            | Self::RebootOrShutdown
            | Self::KillProcess
            | Self::ConfigurationMutation
            | Self::PackageInstall
            | Self::PatchSystem
            | Self::AccountOrPermissionChange
            | Self::DeleteFile
            | Self::PrivilegedShell
            | Self::VmwareMutation
            | Self::BackupMutation
            | Self::CredentialAccess => InfrastructureOperationsCapabilityDisposition::Denied,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsAssessmentQuality {
    Complete,
    PartialDeniedCapability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsApprovalRequirement {
    NotApplicable,
    RequiredBeforeConsequentialAction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsExecutionDisposition {
    NotAttempted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsConfidence {
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsEvidenceNamespace {
    Fixture,
    ApplicationEvidence,
    AssessmentFinding,
    ProposedValidation,
    QaFinding,
    QaCheck,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct InfrastructureOperationsEvidenceRef {
    namespace: InfrastructureOperationsEvidenceNamespace,
    id: InfrastructureOperationsId,
}

impl InfrastructureOperationsEvidenceRef {
    fn new(
        namespace: InfrastructureOperationsEvidenceNamespace,
        id: impl Into<String>,
    ) -> InfrastructureOperationsResult<Self> {
        Ok(Self {
            namespace,
            id: InfrastructureOperationsId::new(id)?,
        })
    }

    #[must_use]
    pub const fn namespace(&self) -> InfrastructureOperationsEvidenceNamespace {
        self.namespace
    }

    #[must_use]
    pub fn id(&self) -> &InfrastructureOperationsId {
        &self.id
    }
}

impl fmt::Debug for InfrastructureOperationsEvidenceRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InfrastructureOperationsEvidenceRef")
            .field("namespace", &self.namespace)
            .field("id", &self.id)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureFindingCategory {
    Architecture,
    TerraformStaticReview,
    IdentityAccess,
    NetworkExposure,
    DataProtection,
    Reliability,
    CostEvidence,
    Operability,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticFindingCategory {
    ServiceState,
    ProcessState,
    LogSignal,
    Configuration,
    ResourcePressure,
    Virtualization,
    BackupRecovery,
    Maintenance,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticFindingBasis {
    EvidenceBound,
    Hypothesis,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureObjectiveDisposition {
    SupportedByFixtures,
    PartiallySupportedByFixtures,
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureFinding {
    id: InfrastructureOperationsId,
    category: InfrastructureFindingCategory,
    statement: String,
    confidence: InfrastructureOperationsConfidence,
    references: Vec<InfrastructureOperationsEvidenceRef>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct DiagnosticFinding {
    id: InfrastructureOperationsId,
    category: DiagnosticFindingCategory,
    statement: String,
    confidence: InfrastructureOperationsConfidence,
    basis: DiagnosticFindingBasis,
    references: Vec<InfrastructureOperationsEvidenceRef>,
}

macro_rules! finding_accessors {
    ($type:ty, $category:ty) => {
        impl $type {
            #[must_use]
            pub fn id(&self) -> &InfrastructureOperationsId {
                &self.id
            }
            #[must_use]
            pub const fn category(&self) -> $category {
                self.category
            }
            #[must_use]
            pub fn statement(&self) -> &str {
                &self.statement
            }
            #[must_use]
            pub const fn confidence(&self) -> InfrastructureOperationsConfidence {
                self.confidence
            }
            #[must_use]
            pub fn references(&self) -> &[InfrastructureOperationsEvidenceRef] {
                &self.references
            }
        }
    };
}

finding_accessors!(InfrastructureFinding, InfrastructureFindingCategory);
finding_accessors!(DiagnosticFinding, DiagnosticFindingCategory);

impl DiagnosticFinding {
    #[must_use]
    pub const fn basis(&self) -> DiagnosticFindingBasis {
        self.basis
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ProposedValidationStep {
    id: InfrastructureOperationsId,
    text: String,
}

impl ProposedValidationStep {
    #[must_use]
    pub fn id(&self) -> &InfrastructureOperationsId {
        &self.id
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ChangePlan {
    objective: String,
    affected_fixture_ids: Vec<InfrastructureOperationsId>,
    proposed_changes: Vec<String>,
    risks: Vec<String>,
    proposed_validation: Vec<ProposedValidationStep>,
    rollback_considerations: String,
    capabilities: Vec<CloudInfrastructureCapability>,
}

impl ChangePlan {
    #[must_use]
    pub fn objective(&self) -> &str {
        &self.objective
    }
    #[must_use]
    pub fn affected_fixture_ids(&self) -> &[InfrastructureOperationsId] {
        &self.affected_fixture_ids
    }
    #[must_use]
    pub fn proposed_changes(&self) -> &[String] {
        &self.proposed_changes
    }
    #[must_use]
    pub fn risks(&self) -> &[String] {
        &self.risks
    }
    #[must_use]
    pub fn proposed_validation(&self) -> &[ProposedValidationStep] {
        &self.proposed_validation
    }
    #[must_use]
    pub fn rollback_considerations(&self) -> &str {
        &self.rollback_considerations
    }
    #[must_use]
    pub fn capabilities(&self) -> &[CloudInfrastructureCapability] {
        &self.capabilities
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureAssessment {
    task_id: AgentTaskId,
    objective_disposition: InfrastructureObjectiveDisposition,
    fixture_ids: Vec<InfrastructureOperationsId>,
    findings: Vec<InfrastructureFinding>,
    limitations: Vec<String>,
    unresolved_questions: Vec<String>,
    change_plan: ChangePlan,
    terraform_execution: TerraformExecutionDisclosure,
    quality: InfrastructureOperationsAssessmentQuality,
    transfer: String,
}

impl InfrastructureAssessment {
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }
    #[must_use]
    pub const fn objective_disposition(&self) -> InfrastructureObjectiveDisposition {
        self.objective_disposition
    }
    #[must_use]
    pub fn fixture_ids(&self) -> &[InfrastructureOperationsId] {
        &self.fixture_ids
    }
    #[must_use]
    pub fn findings(&self) -> &[InfrastructureFinding] {
        &self.findings
    }
    #[must_use]
    pub fn limitations(&self) -> &[String] {
        &self.limitations
    }
    #[must_use]
    pub fn unresolved_questions(&self) -> &[String] {
        &self.unresolved_questions
    }
    #[must_use]
    pub fn change_plan(&self) -> &ChangePlan {
        &self.change_plan
    }
    #[must_use]
    pub fn terraform_execution(&self) -> &TerraformExecutionDisclosure {
        &self.terraform_execution
    }
    #[must_use]
    pub const fn quality(&self) -> InfrastructureOperationsAssessmentQuality {
        self.quality
    }
    #[must_use]
    pub(super) fn transfer(&self) -> &str {
        &self.transfer
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct OperationalAssessment {
    task_id: AgentTaskId,
    fixture_ids: Vec<InfrastructureOperationsId>,
    findings: Vec<DiagnosticFinding>,
    limitations: Vec<String>,
    unresolved_questions: Vec<String>,
    diagnostic_plan: Vec<String>,
    remediation_plan: Vec<String>,
    rollback_considerations: String,
    capabilities: Vec<SystemsOperationsCapability>,
    quality: InfrastructureOperationsAssessmentQuality,
    transfer: String,
}

impl OperationalAssessment {
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }
    #[must_use]
    pub fn fixture_ids(&self) -> &[InfrastructureOperationsId] {
        &self.fixture_ids
    }
    #[must_use]
    pub fn findings(&self) -> &[DiagnosticFinding] {
        &self.findings
    }
    #[must_use]
    pub fn limitations(&self) -> &[String] {
        &self.limitations
    }
    #[must_use]
    pub fn unresolved_questions(&self) -> &[String] {
        &self.unresolved_questions
    }
    #[must_use]
    pub fn diagnostic_plan(&self) -> &[String] {
        &self.diagnostic_plan
    }
    #[must_use]
    pub fn remediation_plan(&self) -> &[String] {
        &self.remediation_plan
    }
    #[must_use]
    pub fn rollback_considerations(&self) -> &str {
        &self.rollback_considerations
    }
    #[must_use]
    pub fn capabilities(&self) -> &[SystemsOperationsCapability] {
        &self.capabilities
    }
    #[must_use]
    pub const fn quality(&self) -> InfrastructureOperationsAssessmentQuality {
        self.quality
    }
    #[must_use]
    pub(super) fn transfer(&self) -> &str {
        &self.transfer
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_catalog_stage_input, build_synthesis_stage_input_from_transfers,
        cloud_fixture_request_data, contains_effect_token_sequence, normalized_claim_tokens,
        reject_credential_text, validate_approval_summary_consistency,
        validate_cloud_capability_consistency, validate_model_text, validate_no_effect_requests,
        validate_systems_capability_consistency, CloudInfrastructureCapability, CloudScenarioId,
        InfrastructureOperationsError, InfrastructureOperationsFixtureCatalog,
        InfrastructureOperationsPartialFailureCode, SystemsOperationsCapability,
        SystemsOperationsScenarioId, MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES,
        MAX_INFRASTRUCTURE_OPERATIONS_TRANSFER_BYTES,
    };

    #[test]
    fn normalized_positive_authority_and_execution_claims_are_rejected() {
        for value in [
            "I ran terraform apply.",
            "Terraform validate passed.",
            "Terraform fmt-check passed.",
            "Provider initialized.",
            "Live inventory completed.",
            "Tests passed.",
            "The service was inspected live.",
            "Authorization granted.",
            "Here is my hidden-reasoning.",
        ] {
            assert_eq!(
                validate_model_text(value),
                Err(InfrastructureOperationsError::AuthorityClaim),
                "claim should be rejected: {value}"
            );
        }
    }

    #[test]
    fn sealed_catalog_binding_and_credential_sentinels_fail_closed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let catalog = InfrastructureOperationsFixtureCatalog::built_in();
        let mut cloud = catalog.cloud_request(CloudScenarioId::TerraformDecisionBriefV1)?;
        cloud.data.objective.push('!');
        assert_eq!(
            cloud.validate_catalog_binding(),
            Err(InfrastructureOperationsError::CatalogBindingMismatch)
        );
        let mut systems =
            catalog.systems_request(SystemsOperationsScenarioId::SanitizedServiceRecoveryV1)?;
        systems.proof.version += 1;
        assert_eq!(
            systems.validate_catalog_binding(),
            Err(InfrastructureOperationsError::CatalogBindingMismatch)
        );

        let credential_sentinels = vec![
            "cortexa_fixture_secret_do_not_use=value".to_owned(),
            "Authorization: Bearer fixture".to_owned(),
            "bearer fixture".to_owned(),
            format!("-----BEGIN {} KEY-----", "PRIVATE"),
            format!("-----BEGIN RSA {} KEY-----", "PRIVATE"),
            "aws_access_key_id=value".to_owned(),
            "AWS_SECRET_ACCESS_KEY=value".to_owned(),
            "aws_session_token=value".to_owned(),
            "azure_client_secret=value".to_owned(),
            "arm_client_secret=value".to_owned(),
            "vmware_password=value".to_owned(),
            "vsphere_password=value".to_owned(),
            "header\r\n  authorization: bearer fixture".to_owned(),
            format!("{}{} ", "AKIA", "1234567890ABCDEF"),
            format!("{}{}\t", "ASIA", "1234567890ABCDEF"),
        ];
        for value in credential_sentinels {
            assert_eq!(
                reject_credential_text(&value),
                Err(InfrastructureOperationsError::CredentialContentRejected),
                "credential sentinel should be rejected: {value}"
            );
        }
        for value in [
            "authorization bearer fixture",
            "aws access key id is unavailable",
            "akia1234567890abcdef",
            "AKIA1234567890ABCDE ",
            "AKIA1234567890ABCDEFG",
        ] {
            assert_eq!(
                reject_credential_text(value),
                Ok(()),
                "nearby non-sentinel should remain valid: {value}"
            );
        }
        Ok(())
    }

    #[test]
    fn immediate_negation_and_nearby_nonclaims_remain_valid() {
        for value in [
            "Terraform validate was not run.",
            "No tests passed; no checks were executed.",
            "Provider initialization was not performed.",
            "Live inventory remains unavailable.",
            "The fixture contains a terraform validator setting.",
            "A service restart is proposed, not executed.",
        ] {
            assert_eq!(
                validate_model_text(value),
                Ok(()),
                "text should pass: {value}"
            );
        }
    }

    #[test]
    fn benign_fixture_and_inert_plan_language_does_not_become_effect_authority() {
        for value in [
            "Terraform state is represented only in the supplied fixture; no command was run.",
            "Propose a Terraform plan for later review without running it.",
            "The fixture describes a firewall change, but no effect was performed.",
        ] {
            assert_eq!(
                validate_model_text(value),
                Ok(()),
                "text should pass: {value}"
            );
        }
        assert!(!contains_effect_token_sequence(
            &normalized_claim_tokens(
                "Propose a Terraform plan for later review without running it."
            ),
            &["run", "terraform", "plan"],
        ));
    }

    #[test]
    fn article_bearing_effect_requests_require_the_exact_denied_capability() {
        assert_eq!(
            validate_cloud_capability_consistency(
                &[
                    "Run the Terraform plan.",
                    "Access the production environment.",
                ],
                &[CloudInfrastructureCapability::ProposeInfrastructureChangePlan],
            ),
            Err(InfrastructureOperationsError::CapabilityInconsistent)
        );
        assert_eq!(
            validate_systems_capability_consistency(
                &[
                    "Restart this service.",
                    "Change the account after separate review."
                ],
                &[SystemsOperationsCapability::ProposeRemediationPlan],
            ),
            Err(InfrastructureOperationsError::CapabilityInconsistent)
        );
        assert_eq!(
            validate_systems_capability_consistency(
                &["Restart the service.", "Delete the file."],
                &[SystemsOperationsCapability::ProposeRemediationPlan],
            ),
            Err(InfrastructureOperationsError::CapabilityInconsistent)
        );
        assert_eq!(
            validate_systems_capability_consistency(
                &[
                    "Restart the service now, but not automatically.",
                    "Delete the file, not the log.",
                ],
                &[SystemsOperationsCapability::ProposeRemediationPlan],
            ),
            Err(InfrastructureOperationsError::CapabilityInconsistent)
        );

        assert_eq!(
            validate_cloud_capability_consistency(
                &[
                    "Do not run the Terraform plan.",
                    "Never access the production environment.",
                ],
                &[CloudInfrastructureCapability::ProposeInfrastructureChangePlan],
            ),
            Ok(())
        );
        assert_eq!(
            validate_systems_capability_consistency(
                &["Do not restart the service.", "Never delete the file."],
                &[SystemsOperationsCapability::ProposeRemediationPlan],
            ),
            Ok(())
        );
    }

    #[test]
    fn later_stage_text_cannot_introduce_an_unclassified_effect_request() {
        for value in [
            "Restart the service.",
            "Change this account after review.",
            "Apply the Terraform configuration.",
            "Delete that resource.",
        ] {
            assert_eq!(
                validate_no_effect_requests(&[value]),
                Err(InfrastructureOperationsError::CapabilityInconsistent),
                "effect request should be rejected outside the first-stage capability list: {value}"
            );
        }

        for value in [
            "Do not restart this service.",
            "Never change the account.",
            "Do not apply Terraform.",
            "No resource was deleted.",
        ] {
            assert_eq!(
                validate_no_effect_requests(&[value]),
                Ok(()),
                "negated effect statement should remain valid: {value}"
            );
        }
    }

    #[test]
    fn synthesis_approval_language_cannot_contradict_the_derived_requirement() {
        assert_eq!(
            validate_approval_summary_consistency("Owner approval is not required.", true),
            Err(InfrastructureOperationsError::StageInconsistent)
        );
        assert_eq!(
            validate_approval_summary_consistency("Owner approval is required.", false),
            Err(InfrastructureOperationsError::StageInconsistent)
        );
        assert_eq!(
            validate_approval_summary_consistency(
                "The fixture-only partial review performed no effect.",
                true,
            ),
            Ok(())
        );
    }

    #[test]
    fn maximum_transfers_fit_every_successor_stage_input(
    ) -> Result<(), InfrastructureOperationsError> {
        let data = cloud_fixture_request_data()?;
        let transfer = "x".repeat(MAX_INFRASTRUCTURE_OPERATIONS_TRANSFER_BYTES);
        let qa = build_catalog_stage_input(
            &data,
            "qa-validation",
            "qa-validation",
            Some(&transfer),
            None,
        )?;
        let security = build_catalog_stage_input(
            &data,
            "security-risk",
            "security-review",
            Some(&transfer),
            Some(&transfer),
        )?;
        let synthesis = build_synthesis_stage_input_from_transfers(
            &data,
            &transfer,
            &transfer,
            &transfer,
            true,
            &[
                InfrastructureOperationsPartialFailureCode::QaIncomplete,
                InfrastructureOperationsPartialFailureCode::SecurityUnavailable,
            ],
        )?;
        assert!(qa.len() <= MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES);
        assert!(security.len() <= MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES);
        assert!(synthesis.len() <= MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES);
        Ok(())
    }
}

impl fmt::Debug for InfrastructureAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InfrastructureAssessment")
            .field("task_id", &self.task_id)
            .field("quality", &self.quality)
            .field("finding_count", &self.findings.len())
            .field("fixture_count", &self.fixture_ids.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

impl fmt::Debug for OperationalAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationalAssessment")
            .field("task_id", &self.task_id)
            .field("quality", &self.quality)
            .field("finding_count", &self.findings.len())
            .field("fixture_count", &self.fixture_ids.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsCriterionDisposition {
    Demonstrated,
    NotDemonstrated,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsValidationConclusion {
    Adequate,
    Incomplete,
    Blocked,
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsCriterionCoverage {
    criterion_id: InfrastructureOperationsId,
    disposition: InfrastructureOperationsCriterionDisposition,
    references: Vec<InfrastructureOperationsEvidenceRef>,
}

impl InfrastructureOperationsCriterionCoverage {
    #[must_use]
    pub fn criterion_id(&self) -> &InfrastructureOperationsId {
        &self.criterion_id
    }
    #[must_use]
    pub const fn disposition(&self) -> InfrastructureOperationsCriterionDisposition {
        self.disposition
    }
    #[must_use]
    pub fn references(&self) -> &[InfrastructureOperationsEvidenceRef] {
        &self.references
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsQaFinding {
    id: InfrastructureOperationsId,
    statement: String,
    references: Vec<InfrastructureOperationsEvidenceRef>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsQaCheck {
    id: InfrastructureOperationsId,
    text: String,
    references: Vec<InfrastructureOperationsEvidenceRef>,
    status: InfrastructureOperationsEvidenceStatus,
}

macro_rules! qa_item_accessors {
    ($type:ty, $text:ident) => {
        impl $type {
            #[must_use]
            pub fn id(&self) -> &InfrastructureOperationsId {
                &self.id
            }
            #[must_use]
            pub fn $text(&self) -> &str {
                &self.$text
            }
            #[must_use]
            pub fn references(&self) -> &[InfrastructureOperationsEvidenceRef] {
                &self.references
            }
        }
    };
}

qa_item_accessors!(InfrastructureOperationsQaFinding, statement);
qa_item_accessors!(InfrastructureOperationsQaCheck, text);

impl InfrastructureOperationsQaCheck {
    #[must_use]
    pub const fn status(&self) -> InfrastructureOperationsEvidenceStatus {
        self.status
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ValidationReport {
    workflow_kind: InfrastructureOperationsWorkflowKind,
    task_id: AgentTaskId,
    assessment_task_id: AgentTaskId,
    coverage: Vec<InfrastructureOperationsCriterionCoverage>,
    findings: Vec<InfrastructureOperationsQaFinding>,
    proposed_checks: Vec<InfrastructureOperationsQaCheck>,
    gaps: Vec<String>,
    conclusion: InfrastructureOperationsValidationConclusion,
    transfer: String,
}

impl ValidationReport {
    #[must_use]
    pub const fn workflow_kind(&self) -> InfrastructureOperationsWorkflowKind {
        self.workflow_kind
    }
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }
    #[must_use]
    pub fn assessment_task_id(&self) -> &AgentTaskId {
        &self.assessment_task_id
    }
    #[must_use]
    pub fn coverage(&self) -> &[InfrastructureOperationsCriterionCoverage] {
        &self.coverage
    }
    #[must_use]
    pub fn findings(&self) -> &[InfrastructureOperationsQaFinding] {
        &self.findings
    }
    #[must_use]
    pub fn proposed_checks(&self) -> &[InfrastructureOperationsQaCheck] {
        &self.proposed_checks
    }
    #[must_use]
    pub fn gaps(&self) -> &[String] {
        &self.gaps
    }
    #[must_use]
    pub const fn conclusion(&self) -> InfrastructureOperationsValidationConclusion {
        self.conclusion
    }
    #[must_use]
    pub(super) fn transfer(&self) -> &str {
        &self.transfer
    }
}

impl fmt::Debug for ValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidationReport")
            .field("workflow_kind", &self.workflow_kind)
            .field("task_id", &self.task_id)
            .field("assessment_task_id", &self.assessment_task_id)
            .field("conclusion", &self.conclusion)
            .field("coverage_count", &self.coverage.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsRiskCategory {
    AuthorizationBoundary,
    InputValidation,
    CredentialsSecrets,
    CloudIamNetwork,
    HostAvailability,
    DataLoss,
    UnsupportedPlatform,
    DependencyEvidence,
    Audit,
    Rollback,
    GeneralChangeRisk,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsRiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsRiskBasis {
    EvidenceBound,
    Hypothesis,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsSupportingEvidenceStatus {
    Unavailable,
    SyntheticFixture,
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsRiskFinding {
    id: InfrastructureOperationsId,
    category: InfrastructureOperationsRiskCategory,
    severity: InfrastructureOperationsRiskSeverity,
    confidence: InfrastructureOperationsConfidence,
    statement: String,
    basis: InfrastructureOperationsRiskBasis,
    references: Vec<InfrastructureOperationsEvidenceRef>,
}

impl InfrastructureOperationsRiskFinding {
    #[must_use]
    pub fn id(&self) -> &InfrastructureOperationsId {
        &self.id
    }
    #[must_use]
    pub const fn category(&self) -> InfrastructureOperationsRiskCategory {
        self.category
    }
    #[must_use]
    pub const fn severity(&self) -> InfrastructureOperationsRiskSeverity {
        self.severity
    }
    #[must_use]
    pub const fn confidence(&self) -> InfrastructureOperationsConfidence {
        self.confidence
    }
    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }
    #[must_use]
    pub const fn basis(&self) -> InfrastructureOperationsRiskBasis {
        self.basis
    }
    #[must_use]
    pub fn references(&self) -> &[InfrastructureOperationsEvidenceRef] {
        &self.references
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RiskAssessment {
    workflow_kind: InfrastructureOperationsWorkflowKind,
    task_id: AgentTaskId,
    assessment_task_id: AgentTaskId,
    qa_task_id: Option<AgentTaskId>,
    findings: Vec<InfrastructureOperationsRiskFinding>,
    unresolved_risks: Vec<String>,
    follow_ups: Vec<String>,
    dependency_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    provider_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    credential_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    platform_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    audit_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    rollback_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    transfer: String,
}

impl RiskAssessment {
    #[must_use]
    pub const fn workflow_kind(&self) -> InfrastructureOperationsWorkflowKind {
        self.workflow_kind
    }
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }
    #[must_use]
    pub fn assessment_task_id(&self) -> &AgentTaskId {
        &self.assessment_task_id
    }
    #[must_use]
    pub fn qa_task_id(&self) -> Option<&AgentTaskId> {
        self.qa_task_id.as_ref()
    }
    #[must_use]
    pub fn findings(&self) -> &[InfrastructureOperationsRiskFinding] {
        &self.findings
    }
    #[must_use]
    pub fn unresolved_risks(&self) -> &[String] {
        &self.unresolved_risks
    }
    #[must_use]
    pub fn follow_ups(&self) -> &[String] {
        &self.follow_ups
    }
    #[must_use]
    pub const fn dependency_evidence(&self) -> InfrastructureOperationsSupportingEvidenceStatus {
        self.dependency_evidence
    }
    #[must_use]
    pub const fn provider_evidence(&self) -> InfrastructureOperationsSupportingEvidenceStatus {
        self.provider_evidence
    }
    #[must_use]
    pub const fn credential_evidence(&self) -> InfrastructureOperationsSupportingEvidenceStatus {
        self.credential_evidence
    }
    #[must_use]
    pub const fn platform_evidence(&self) -> InfrastructureOperationsSupportingEvidenceStatus {
        self.platform_evidence
    }
    #[must_use]
    pub const fn audit_evidence(&self) -> InfrastructureOperationsSupportingEvidenceStatus {
        self.audit_evidence
    }
    #[must_use]
    pub const fn rollback_evidence(&self) -> InfrastructureOperationsSupportingEvidenceStatus {
        self.rollback_evidence
    }
    #[must_use]
    pub(super) fn transfer(&self) -> &str {
        &self.transfer
    }
}

impl fmt::Debug for RiskAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RiskAssessment")
            .field("workflow_kind", &self.workflow_kind)
            .field("task_id", &self.task_id)
            .field("assessment_task_id", &self.assessment_task_id)
            .field("qa_task_id", &self.qa_task_id)
            .field("finding_count", &self.findings.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureOperationsReviewStatus {
    Complete,
    Partial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsStageDisposition {
    Completed,
    Failed(AgentTaskFailureCode),
    Cancelled,
    SkippedFirstStageUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InfrastructureOperationsStageProjection {
    first_stage: InfrastructureOperationsStageDisposition,
    qa: InfrastructureOperationsStageDisposition,
    security: InfrastructureOperationsStageDisposition,
}

impl InfrastructureOperationsStageProjection {
    #[must_use]
    pub const fn first_stage(&self) -> InfrastructureOperationsStageDisposition {
        self.first_stage
    }

    #[must_use]
    pub const fn qa(&self) -> InfrastructureOperationsStageDisposition {
        self.qa
    }

    #[must_use]
    pub const fn security(&self) -> InfrastructureOperationsStageDisposition {
        self.security
    }
}

#[derive(Clone, Eq, PartialEq)]
struct InfrastructureOperationsSynthesis {
    summary: String,
    fixture_ids: Vec<InfrastructureOperationsId>,
    unresolved_issues: Vec<String>,
    approval_requirement: InfrastructureOperationsApprovalRequirement,
    execution_disposition: InfrastructureOperationsExecutionDisposition,
    status: InfrastructureOperationsReviewStatus,
    stage_projection: InfrastructureOperationsStageProjection,
    partial_failure_codes: Vec<InfrastructureOperationsPartialFailureCode>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct CloudInfrastructureSynthesis(
    InfrastructureOperationsSynthesis,
    TerraformExecutionDisclosure,
);

#[derive(Clone, Eq, PartialEq)]
pub struct SystemsOperationsSynthesis(InfrastructureOperationsSynthesis);

macro_rules! synthesis_accessors {
    ($type:ty) => {
        impl $type {
            #[must_use]
            pub fn summary(&self) -> &str {
                &self.0.summary
            }
            #[must_use]
            pub fn fixture_ids(&self) -> &[InfrastructureOperationsId] {
                &self.0.fixture_ids
            }
            #[must_use]
            pub fn unresolved_issues(&self) -> &[String] {
                &self.0.unresolved_issues
            }
            #[must_use]
            pub const fn approval_requirement(
                &self,
            ) -> InfrastructureOperationsApprovalRequirement {
                self.0.approval_requirement
            }
            #[must_use]
            pub const fn execution_disposition(
                &self,
            ) -> InfrastructureOperationsExecutionDisposition {
                self.0.execution_disposition
            }
            #[must_use]
            pub const fn status(&self) -> InfrastructureOperationsReviewStatus {
                self.0.status
            }
            #[must_use]
            pub const fn stage_projection(&self) -> &InfrastructureOperationsStageProjection {
                &self.0.stage_projection
            }
            #[must_use]
            pub fn partial_failure_codes(&self) -> &[InfrastructureOperationsPartialFailureCode] {
                &self.0.partial_failure_codes
            }
        }
    };
}

synthesis_accessors!(CloudInfrastructureSynthesis);
synthesis_accessors!(SystemsOperationsSynthesis);

impl CloudInfrastructureSynthesis {
    #[must_use]
    pub fn terraform_execution(&self) -> &TerraformExecutionDisclosure {
        &self.1
    }
}

impl fmt::Debug for CloudInfrastructureSynthesis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_synthesis(formatter, "CloudInfrastructureSynthesis", &self.0)
    }
}

impl fmt::Debug for SystemsOperationsSynthesis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_synthesis(formatter, "SystemsOperationsSynthesis", &self.0)
    }
}

fn debug_synthesis(
    formatter: &mut fmt::Formatter<'_>,
    name: &str,
    value: &InfrastructureOperationsSynthesis,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("summary", &"[REDACTED]")
        .field("fixture_count", &value.fixture_ids.len())
        .field("unresolved_count", &value.unresolved_issues.len())
        .field("approval_requirement", &value.approval_requirement)
        .field("execution_disposition", &value.execution_disposition)
        .field("status", &value.status)
        .field("stage_projection", &value.stage_projection)
        .field("partial_failure_codes", &value.partial_failure_codes)
        .finish()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CloudInfrastructureStageOutcome {
    Completed(Box<InfrastructureAssessment>),
    Failed(AgentTaskFailureCode),
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemsOperationsStageOutcome {
    Completed(Box<OperationalAssessment>),
    Failed(AgentTaskFailureCode),
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsQaStageOutcome {
    Completed(ValidationReport),
    Failed(AgentTaskFailureCode),
    Cancelled,
    SkippedFirstStageUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsSecurityStageOutcome {
    Completed(RiskAssessment),
    Failed(AgentTaskFailureCode),
    Cancelled,
    SkippedFirstStageUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloudInfrastructureWorkflowResult {
    root_task_id: RootTaskId,
    assessment: CloudInfrastructureStageOutcome,
    qa: InfrastructureOperationsQaStageOutcome,
    security: InfrastructureOperationsSecurityStageOutcome,
    synthesis: CloudInfrastructureSynthesis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemsOperationsWorkflowResult {
    root_task_id: RootTaskId,
    assessment: SystemsOperationsStageOutcome,
    qa: InfrastructureOperationsQaStageOutcome,
    security: InfrastructureOperationsSecurityStageOutcome,
    synthesis: SystemsOperationsSynthesis,
}

macro_rules! workflow_result_impl {
    ($type:ty, $first:ty, $synthesis:ty) => {
        impl $type {
            pub(super) fn new(
                root_task_id: RootTaskId,
                assessment: $first,
                qa: InfrastructureOperationsQaStageOutcome,
                security: InfrastructureOperationsSecurityStageOutcome,
                synthesis: $synthesis,
            ) -> Self {
                Self {
                    root_task_id,
                    assessment,
                    qa,
                    security,
                    synthesis,
                }
            }
            #[must_use]
            pub fn root_task_id(&self) -> &RootTaskId {
                &self.root_task_id
            }
            #[must_use]
            pub fn assessment(&self) -> &$first {
                &self.assessment
            }
            #[must_use]
            pub fn qa(&self) -> &InfrastructureOperationsQaStageOutcome {
                &self.qa
            }
            #[must_use]
            pub fn security(&self) -> &InfrastructureOperationsSecurityStageOutcome {
                &self.security
            }
            #[must_use]
            pub fn synthesis(&self) -> &$synthesis {
                &self.synthesis
            }
            #[must_use]
            pub fn stage_projection(&self) -> &InfrastructureOperationsStageProjection {
                self.synthesis.stage_projection()
            }
            #[must_use]
            pub fn partial_failure_codes(&self) -> &[InfrastructureOperationsPartialFailureCode] {
                self.synthesis.partial_failure_codes()
            }
        }
    };
}

workflow_result_impl!(
    CloudInfrastructureWorkflowResult,
    CloudInfrastructureStageOutcome,
    CloudInfrastructureSynthesis
);
workflow_result_impl!(
    SystemsOperationsWorkflowResult,
    SystemsOperationsStageOutcome,
    SystemsOperationsSynthesis
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsStage {
    CloudAssessment,
    SystemsAssessment,
    QaValidation,
    SecurityReview,
    Synthesis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsPartialFailureCode {
    FirstStageUnavailable,
    ContainsDeniedCapability,
    InvalidStructuredOutput,
    QaUnavailable,
    QaIncomplete,
    SecurityUnavailable,
    RuntimeStartFailed,
    RuntimeFailed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsContinuationFailure {
    CloudStartFailed,
    CloudAndSynthesisStartFailed,
    SystemsStartFailed,
    SystemsAndSynthesisStartFailed,
    QaStartFailed,
    QaAndSecurityStartFailed,
    QaAndSynthesisStartFailed,
    QaSecurityAndSynthesisStartFailed,
    SecurityStartFailed,
    SecurityAndSynthesisStartFailed,
    SynthesisStartFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CloudInfrastructureWorkflowEvent {
    CloudAssessmentStarted {
        task_id: AgentTaskId,
    },
    CloudAssessmentCompleted {
        task_id: AgentTaskId,
        quality: InfrastructureOperationsAssessmentQuality,
    },
    QaValidationStarted {
        task_id: AgentTaskId,
        predecessor_task_id: AgentTaskId,
    },
    QaValidationCompleted {
        task_id: AgentTaskId,
        conclusion: InfrastructureOperationsValidationConclusion,
    },
    SecurityReviewStarted {
        task_id: AgentTaskId,
        predecessor_task_id: AgentTaskId,
    },
    SecurityReviewCompleted {
        task_id: AgentTaskId,
    },
    SynthesisStarted {
        task_id: AgentTaskId,
    },
    PartialFailure {
        stage: InfrastructureOperationsStage,
        code: InfrastructureOperationsPartialFailureCode,
    },
    Cancelled {
        stage: InfrastructureOperationsStage,
    },
    Completed {
        task_id: AgentTaskId,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemsOperationsWorkflowEvent {
    SystemsAssessmentStarted {
        task_id: AgentTaskId,
    },
    SystemsAssessmentCompleted {
        task_id: AgentTaskId,
        quality: InfrastructureOperationsAssessmentQuality,
    },
    QaValidationStarted {
        task_id: AgentTaskId,
        predecessor_task_id: AgentTaskId,
    },
    QaValidationCompleted {
        task_id: AgentTaskId,
        conclusion: InfrastructureOperationsValidationConclusion,
    },
    SecurityReviewStarted {
        task_id: AgentTaskId,
        predecessor_task_id: AgentTaskId,
    },
    SecurityReviewCompleted {
        task_id: AgentTaskId,
    },
    SynthesisStarted {
        task_id: AgentTaskId,
    },
    PartialFailure {
        stage: InfrastructureOperationsStage,
        code: InfrastructureOperationsPartialFailureCode,
    },
    Cancelled {
        stage: InfrastructureOperationsStage,
    },
    Completed {
        task_id: AgentTaskId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsAuditOutcome {
    Started,
    Completed,
    PartialFailure(InfrastructureOperationsPartialFailureCode),
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfrastructureOperationsCapabilityAuditDisposition {
    NotApplicable,
    ProposalOnly,
    DeniedRequested,
}

#[derive(Clone, Eq, PartialEq)]
pub struct InfrastructureOperationsAttribution {
    agent_id: AgentId,
    policy_profile_id: AgentPolicyProfileId,
    memory_profile_id: AgentMemoryProfileId,
    task_id: AgentTaskId,
    root_task_id: RootTaskId,
    parent_task_id: Option<ParentTaskId>,
    runtime_id: RuntimeId,
    depth: u8,
    _runtime_run_identity: RuntimeRunIdentity,
}

impl InfrastructureOperationsAttribution {
    pub(super) fn from_execution_context(context: &AgentExecutionContext) -> Self {
        Self {
            agent_id: context.agent_id(),
            policy_profile_id: context.policy_profile_id(),
            memory_profile_id: context.memory_profile_id(),
            task_id: context.task_id().clone(),
            root_task_id: context.root_task_id().clone(),
            parent_task_id: context.parent_task_id().cloned(),
            runtime_id: context.runtime_id(),
            depth: context.depth(),
            _runtime_run_identity: context.runtime_run_identity().clone(),
        }
    }
    #[must_use]
    pub const fn agent_id(&self) -> AgentId {
        self.agent_id
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
    pub const fn depth(&self) -> u8 {
        self.depth
    }
}

impl fmt::Debug for InfrastructureOperationsAttribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InfrastructureOperationsAttribution")
            .field("agent_id", &self.agent_id)
            .field("policy_profile_id", &self.policy_profile_id)
            .field("memory_profile_id", &self.memory_profile_id)
            .field("task_id", &self.task_id)
            .field("root_task_id", &self.root_task_id)
            .field("parent_task_id", &self.parent_task_id)
            .field("runtime_id", &self.runtime_id)
            .field("depth", &self.depth)
            .field("runtime_run_identity", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfrastructureOperationsAttributionRecord {
    sequence: u8,
    attribution: InfrastructureOperationsAttribution,
    predecessor_task_id: Option<AgentTaskId>,
    stage: InfrastructureOperationsStage,
    outcome: InfrastructureOperationsAuditOutcome,
    capability_disposition: InfrastructureOperationsCapabilityAuditDisposition,
}

impl InfrastructureOperationsAttributionRecord {
    pub(super) fn new(
        sequence: u8,
        attribution: InfrastructureOperationsAttribution,
        predecessor_task_id: Option<AgentTaskId>,
        stage: InfrastructureOperationsStage,
        outcome: InfrastructureOperationsAuditOutcome,
        capability_disposition: InfrastructureOperationsCapabilityAuditDisposition,
    ) -> Self {
        Self {
            sequence,
            attribution,
            predecessor_task_id,
            stage,
            outcome,
            capability_disposition,
        }
    }
    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
    #[must_use]
    pub fn attribution(&self) -> &InfrastructureOperationsAttribution {
        &self.attribution
    }
    #[must_use]
    pub fn predecessor_task_id(&self) -> Option<&AgentTaskId> {
        self.predecessor_task_id.as_ref()
    }
    #[must_use]
    pub const fn stage(&self) -> InfrastructureOperationsStage {
        self.stage
    }
    #[must_use]
    pub const fn outcome(&self) -> InfrastructureOperationsAuditOutcome {
        self.outcome
    }
    #[must_use]
    pub const fn capability_disposition(
        &self,
    ) -> InfrastructureOperationsCapabilityAuditDisposition {
        self.capability_disposition
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum InfrastructureOperationsError {
    #[error("the identifier is invalid")]
    InvalidIdentifier,
    #[error("a bounded infrastructure/operations value exceeded its limit")]
    BoundExceeded,
    #[error("a required infrastructure/operations value is invalid")]
    InvalidText,
    #[error("a duplicate reference was supplied")]
    DuplicateReference,
    #[error("a reference is not present in the sealed fixture catalog")]
    UnknownReference,
    #[error("validation evidence is inconsistent with the sealed catalog")]
    EvidenceInconsistent,
    #[error("the request does not match the sealed built-in catalog")]
    CatalogBindingMismatch,
    #[error("credential-like content matched the narrow deterministic guard")]
    CredentialContentRejected,
    #[error("structured output could not be serialized")]
    SerializationFailed,
    #[error("structured output is malformed")]
    InvalidStructuredOutput,
    #[error("structured output claims unavailable authority")]
    AuthorityClaim,
    #[error("structured output contains an inconsistent capability")]
    CapabilityInconsistent,
    #[error("structured output is inconsistent with prior validated stages")]
    StageInconsistent,
    #[error("the infrastructure/operations journal reached its bound")]
    JournalLimitExceeded,
}
