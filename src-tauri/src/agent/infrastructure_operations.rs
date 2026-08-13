//! Strict contracts for the sealed fixture-only infrastructure workflows.
//!
//! This module owns immutable synthetic fixtures, bounded structured results,
//! and content-free lifecycle evidence for the Cloud Infrastructure and
//! Systems Operations workflows. It performs no I/O and grants no tool,
//! credential, approval, runtime, provider, host, or execution authority.

use std::{collections::BTreeSet, fmt};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

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

impl InfrastructureOperationsFixtureCatalog {
    #[must_use]
    pub const fn built_in() -> Self {
        Self
    }

    pub fn cloud_request(
        self,
        scenario_id: CloudScenarioId,
    ) -> InfrastructureOperationsResult<CloudInfrastructureWorkflowRequest> {
        let data = match scenario_id {
            CloudScenarioId::TerraformDecisionBriefV1 => cloud_fixture_request_data()?,
        };
        Ok(CloudInfrastructureWorkflowRequest {
            scenario_id,
            data,
            proof: CatalogProof {
                version: BUILT_IN_CATALOG_VERSION,
            },
        })
    }

    pub fn systems_request(
        self,
        scenario_id: SystemsOperationsScenarioId,
    ) -> InfrastructureOperationsResult<SystemsOperationsWorkflowRequest> {
        let data = match scenario_id {
            SystemsOperationsScenarioId::SanitizedServiceRecoveryV1 => {
                systems_fixture_request_data()?
            }
        };
        Ok(SystemsOperationsWorkflowRequest {
            scenario_id,
            data,
            proof: CatalogProof {
                version: BUILT_IN_CATALOG_VERSION,
            },
        })
    }
}

impl CloudInfrastructureWorkflowRequest {
    pub(super) fn validate_catalog_binding(&self) -> InfrastructureOperationsResult<()> {
        let canonical =
            InfrastructureOperationsFixtureCatalog::built_in().cloud_request(self.scenario_id)?;
        if self == &canonical && self.proof.version == BUILT_IN_CATALOG_VERSION {
            Ok(())
        } else {
            Err(InfrastructureOperationsError::CatalogBindingMismatch)
        }
    }
}

impl SystemsOperationsWorkflowRequest {
    pub(super) fn validate_catalog_binding(&self) -> InfrastructureOperationsResult<()> {
        let canonical =
            InfrastructureOperationsFixtureCatalog::built_in().systems_request(self.scenario_id)?;
        if self == &canonical && self.proof.version == BUILT_IN_CATALOG_VERSION {
            Ok(())
        } else {
            Err(InfrastructureOperationsError::CatalogBindingMismatch)
        }
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

fn cloud_fixture_request_data() -> InfrastructureOperationsResult<WorkflowRequestData> {
    let fixtures = vec![
        InfrastructureOperationsFixture::new(
            "terraform-configuration",
            InfrastructureOperationsFixtureKind::Cloud(CloudFixtureKind::TerraformConfiguration),
            "Synthetic Terraform configuration",
            "resource \"fixture_compute\" \"example\" {\n  public_ingress = true\n}",
        )?,
        InfrastructureOperationsFixture::new(
            "azure-architecture",
            InfrastructureOperationsFixtureKind::Cloud(CloudFixtureKind::AzureArchitecture),
            "Synthetic Azure architecture note",
            "The synthetic design places an internet-facing endpoint before one workload tier.",
        )?,
    ];
    let criteria = vec![
        InfrastructureOperationsCriterion::new(
            "criterion-static-review",
            "Identify fixture-supported infrastructure risks and limitations.",
        )?,
        InfrastructureOperationsCriterion::new(
            "criterion-no-execution",
            "State that Terraform, provider inventory, and external checks were not run.",
        )?,
    ];
    let evidence = vec![
        InfrastructureOperationsValidationEvidence::new(
            "evidence-static-observation",
            InfrastructureOperationsEvidenceStatus::ObservedFixture,
            "The application supplied the synthetic configuration and architecture note.",
            vec![InfrastructureOperationsId::new("criterion-static-review")?],
            vec![
                InfrastructureOperationsId::new("terraform-configuration")?,
                InfrastructureOperationsId::new("azure-architecture")?,
            ],
        )?,
        InfrastructureOperationsValidationEvidence::new(
            "evidence-terraform-not-run",
            InfrastructureOperationsEvidenceStatus::NotRun,
            "Terraform and provider checks were not run.",
            vec![InfrastructureOperationsId::new("criterion-no-execution")?],
            Vec::new(),
        )?,
    ];
    WorkflowRequestData::new(
        "Review the supplied synthetic Terraform and architecture fixtures and produce an inert decision brief.",
        fixtures,
        criteria,
        evidence,
    )
}

fn systems_fixture_request_data() -> InfrastructureOperationsResult<WorkflowRequestData> {
    let fixtures = vec![
        InfrastructureOperationsFixture::new(
            "service-snapshot",
            InfrastructureOperationsFixtureKind::Systems(SystemsFixtureKind::ServiceSnapshot),
            "Synthetic service snapshot",
            "fixture-api.service is inactive after three fixture-only start attempts.",
        )?,
        InfrastructureOperationsFixture::new(
            "sanitized-log",
            InfrastructureOperationsFixtureKind::Systems(
                SystemsFixtureKind::SanitizedLogExcerpt,
            ),
            "Sanitized synthetic log excerpt",
            "The fixture log reports a dependency timeout and contains no host or account identity.",
        )?,
        InfrastructureOperationsFixture::new(
            "recovery-scenario",
            InfrastructureOperationsFixtureKind::Systems(SystemsFixtureKind::RecoveryScenario),
            "Synthetic recovery scenario",
            "Analyze recovery options without restarting a service or changing configuration.",
        )?,
    ];
    let criteria = vec![
        InfrastructureOperationsCriterion::new(
            "criterion-diagnostic",
            "Distinguish fixture-supported findings from hypotheses.",
        )?,
        InfrastructureOperationsCriterion::new(
            "criterion-safe-remediation",
            "Propose diagnostics and remediation without executing a command or effect.",
        )?,
    ];
    let evidence = vec![InfrastructureOperationsValidationEvidence::new(
        "evidence-service-log-observation",
        InfrastructureOperationsEvidenceStatus::ObservedFixture,
        "The application supplied the sanitized service and log fixtures.",
        vec![InfrastructureOperationsId::new("criterion-diagnostic")?],
        vec![
            InfrastructureOperationsId::new("service-snapshot")?,
            InfrastructureOperationsId::new("sanitized-log")?,
        ],
    )?];
    WorkflowRequestData::new(
        "Analyze the supplied sanitized service and log fixtures and propose inert diagnostics and recovery steps.",
        fixtures,
        criteria,
        evidence,
    )
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

fn validate_text(
    value: &str,
    max_characters: usize,
    max_bytes: usize,
    allow_newline: bool,
) -> InfrastructureOperationsResult<()> {
    if value.is_empty()
        || value.chars().count() > max_characters
        || value.len() > max_bytes
        || value.chars().any(|character| {
            character.is_control() && !(allow_newline && matches!(character, '\n' | '\r' | '\t'))
        })
    {
        return Err(InfrastructureOperationsError::InvalidText);
    }
    Ok(())
}

fn ensure_count(
    count: usize,
    minimum: usize,
    maximum: usize,
) -> InfrastructureOperationsResult<()> {
    if (minimum..=maximum).contains(&count) {
        Ok(())
    } else {
        Err(InfrastructureOperationsError::BoundExceeded)
    }
}

fn ensure_unique<T: Ord>(
    values: &[T],
    minimum: usize,
    maximum: usize,
) -> InfrastructureOperationsResult<()> {
    ensure_count(values.len(), minimum, maximum)?;
    let unique: BTreeSet<_> = values.iter().collect();
    if unique.len() == values.len() {
        Ok(())
    } else {
        Err(InfrastructureOperationsError::DuplicateReference)
    }
}

fn ensure_unique_by<'a, T: 'a + Ord>(
    values: impl Iterator<Item = &'a T>,
) -> InfrastructureOperationsResult<()> {
    let values: Vec<_> = values.collect();
    let unique: BTreeSet<_> = values.iter().copied().collect();
    if unique.len() == values.len() {
        Ok(())
    } else {
        Err(InfrastructureOperationsError::DuplicateReference)
    }
}

fn reject_credential_text(value: &str) -> InfrastructureOperationsResult<()> {
    for line in value.replace("\r\n", "\n").lines() {
        let trimmed = line.trim_ascii();
        let folded = trimmed.to_ascii_lowercase();
        if [
            "cortexa_fixture_secret_do_not_use=",
            "authorization: bearer ",
            "bearer ",
            "-----begin private key-----",
            "-----begin rsa private key-----",
            "aws_access_key_id=",
            "aws_secret_access_key=",
            "aws_session_token=",
            "azure_client_secret=",
            "arm_client_secret=",
            "vmware_password=",
            "vsphere_password=",
        ]
        .iter()
        .any(|prefix| folded.starts_with(prefix))
        {
            return Err(InfrastructureOperationsError::CredentialContentRejected);
        }
        let bytes = trimmed.as_bytes();
        if bytes.len() >= 20
            && (bytes.starts_with(b"AKIA") || bytes.starts_with(b"ASIA"))
            && bytes[4..20]
                .iter()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
            && bytes
                .get(20)
                .is_none_or(|byte| !byte.is_ascii_alphanumeric())
        {
            return Err(InfrastructureOperationsError::CredentialContentRejected);
        }
    }
    Ok(())
}

fn build_catalog_stage_input(
    data: &WorkflowRequestData,
    assigned_agent: &str,
    stage: &str,
    assessment: Option<&str>,
    predecessor: Option<&str>,
) -> InfrastructureOperationsResult<String> {
    let catalog = serde_json::json!({
        "catalog_version": data.catalog_version,
        "objective": data.objective,
        "fixtures": data.fixtures.iter().map(|fixture| serde_json::json!({
            "id": fixture.id.as_str(),
            "kind": fixture.kind,
            "label": fixture.label,
            "content": fixture.content,
            "source": "synthetic-fixture"
        })).collect::<Vec<_>>(),
        "criteria": data.criteria.iter().map(|criterion| serde_json::json!({
            "id": criterion.id.as_str(),
            "text": criterion.text
        })).collect::<Vec<_>>(),
        "validation_evidence": data.evidence.iter().map(|evidence| serde_json::json!({
            "id": evidence.id.as_str(),
            "status": evidence.status,
            "description": evidence.description,
            "criterion_ids": evidence.criterion_ids.iter().map(InfrastructureOperationsId::as_str).collect::<Vec<_>>(),
            "fixture_ids": evidence.fixture_ids.iter().map(InfrastructureOperationsId::as_str).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    });
    let serialized = serde_json::to_string(&catalog)
        .map_err(|_| InfrastructureOperationsError::SerializationFailed)?;
    let terraform_disclosure = if assigned_agent == "cloud-infrastructure" {
        "\nterraform_fmt_check=not-run\nterraform_validate=not-run\nprovider_initialization=not-run\nterraform_plan=not-run\nterraform_apply=not-run"
    } else {
        ""
    };
    let input = format!(
        "infrastructure-operations-v1\nassigned_agent={assigned_agent}\nstage={stage}\nfixture_based=true\nlive_access=false\ncommands_executed=false\ncredentials_loaded=false\neffects_performed=false{terraform_disclosure}\ncatalog(untrusted):\n{serialized}\nvalidated_assessment(untrusted):\n{}\nvalidated_predecessor(untrusted):\n{}\nReturn one strict stage-specific V1 JSON object.",
        assessment.unwrap_or("none"),
        predecessor.unwrap_or("none"),
    );
    if input.len() > MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES {
        Err(InfrastructureOperationsError::BoundExceeded)
    } else {
        Ok(input)
    }
}

fn build_synthesis_stage_input(
    data: &WorkflowRequestData,
    first: &str,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
    cloud: bool,
    partial_failure_codes: Vec<InfrastructureOperationsPartialFailureCode>,
) -> InfrastructureOperationsResult<String> {
    build_synthesis_stage_input_from_transfers(
        data,
        first,
        qa_transfer(qa),
        security_transfer(security),
        cloud,
        &partial_failure_codes,
    )
}

fn build_synthesis_stage_input_from_transfers(
    data: &WorkflowRequestData,
    first: &str,
    qa: &str,
    security: &str,
    cloud: bool,
    partial_failure_codes: &[InfrastructureOperationsPartialFailureCode],
) -> InfrastructureOperationsResult<String> {
    let terraform_disclosure = if cloud {
        "\nterraform_fmt_check=not-run\nterraform_validate=not-run\nprovider_initialization=not-run\nterraform_plan=not-run\nterraform_apply=not-run"
    } else {
        ""
    };
    let input = format!(
        "infrastructure-operations-v1\nassigned_agent=personal-assistant\nstage=synthesis\nfixture_based=true\nlive_inventory_performed=false\ncommands_executed=false\ncredentials_loaded=false\neffects_performed=false{terraform_disclosure}\nfixture_ids={}\npartial_failure_codes={}\nfirst_stage(untrusted):\n{first}\nqa_outcome(untrusted):\n{}\nsecurity_outcome(untrusted):\n{}\nReturn one strict workflow-specific SynthesisV1 JSON object.",
        data.fixture_ids().iter().map(InfrastructureOperationsId::as_str).collect::<Vec<_>>().join(","),
        if partial_failure_codes.is_empty() {
            "none".to_owned()
        } else {
            partial_failure_codes
                .iter()
                .map(|code| partial_failure_code_label(*code))
                .collect::<Vec<_>>()
                .join(",")
        },
        qa,
        security,
    );
    if input.len() > MAX_INFRASTRUCTURE_OPERATIONS_STAGE_INPUT_BYTES {
        Err(InfrastructureOperationsError::BoundExceeded)
    } else {
        Ok(input)
    }
}

fn cloud_transfer(outcome: &CloudInfrastructureStageOutcome) -> &str {
    match outcome {
        CloudInfrastructureStageOutcome::Completed(value) => value.transfer(),
        CloudInfrastructureStageOutcome::Failed(_) => "status=cloud-assessment-failed",
        CloudInfrastructureStageOutcome::Cancelled => "status=cloud-assessment-cancelled",
    }
}

fn systems_transfer(outcome: &SystemsOperationsStageOutcome) -> &str {
    match outcome {
        SystemsOperationsStageOutcome::Completed(value) => value.transfer(),
        SystemsOperationsStageOutcome::Failed(_) => "status=systems-assessment-failed",
        SystemsOperationsStageOutcome::Cancelled => "status=systems-assessment-cancelled",
    }
}

fn qa_transfer(outcome: &InfrastructureOperationsQaStageOutcome) -> &str {
    match outcome {
        InfrastructureOperationsQaStageOutcome::Completed(value) => value.transfer(),
        InfrastructureOperationsQaStageOutcome::Failed(_) => "status=qa-failed",
        InfrastructureOperationsQaStageOutcome::Cancelled => "status=qa-cancelled",
        InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable => "status=qa-skipped",
    }
}

fn security_transfer(outcome: &InfrastructureOperationsSecurityStageOutcome) -> &str {
    match outcome {
        InfrastructureOperationsSecurityStageOutcome::Completed(value) => value.transfer(),
        InfrastructureOperationsSecurityStageOutcome::Failed(_) => "status=security-failed",
        InfrastructureOperationsSecurityStageOutcome::Cancelled => "status=security-cancelled",
        InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable => {
            "status=security-skipped"
        }
    }
}

fn infrastructure_partial_failure_codes(
    first: &CloudInfrastructureStageOutcome,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
) -> Vec<InfrastructureOperationsPartialFailureCode> {
    let mut codes = Vec::new();
    match first {
        CloudInfrastructureStageOutcome::Completed(value)
            if value.quality
                == InfrastructureOperationsAssessmentQuality::PartialDeniedCapability =>
        {
            codes.push(InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability);
        }
        CloudInfrastructureStageOutcome::Completed(_) => {}
        CloudInfrastructureStageOutcome::Failed(code) => push_failure_code(
            &mut codes,
            *code,
            InfrastructureOperationsPartialFailureCode::RuntimeFailed,
        ),
        CloudInfrastructureStageOutcome::Cancelled => {
            codes.push(InfrastructureOperationsPartialFailureCode::Cancelled);
        }
    }
    push_later_stage_failure_codes(&mut codes, qa, security);
    codes
}

fn systems_partial_failure_codes(
    first: &SystemsOperationsStageOutcome,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
) -> Vec<InfrastructureOperationsPartialFailureCode> {
    let mut codes = Vec::new();
    match first {
        SystemsOperationsStageOutcome::Completed(value)
            if value.quality
                == InfrastructureOperationsAssessmentQuality::PartialDeniedCapability =>
        {
            codes.push(InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability);
        }
        SystemsOperationsStageOutcome::Completed(_) => {}
        SystemsOperationsStageOutcome::Failed(code) => push_failure_code(
            &mut codes,
            *code,
            InfrastructureOperationsPartialFailureCode::RuntimeFailed,
        ),
        SystemsOperationsStageOutcome::Cancelled => {
            codes.push(InfrastructureOperationsPartialFailureCode::Cancelled);
        }
    }
    push_later_stage_failure_codes(&mut codes, qa, security);
    codes
}

fn push_later_stage_failure_codes(
    codes: &mut Vec<InfrastructureOperationsPartialFailureCode>,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
) {
    match qa {
        InfrastructureOperationsQaStageOutcome::Completed(report)
            if report.conclusion != InfrastructureOperationsValidationConclusion::Adequate =>
        {
            codes.push(InfrastructureOperationsPartialFailureCode::QaIncomplete);
        }
        InfrastructureOperationsQaStageOutcome::Completed(_) => {}
        InfrastructureOperationsQaStageOutcome::Failed(code) => {
            push_failure_code(
                codes,
                *code,
                InfrastructureOperationsPartialFailureCode::QaUnavailable,
            );
        }
        InfrastructureOperationsQaStageOutcome::Cancelled => {
            push_unique_code(codes, InfrastructureOperationsPartialFailureCode::Cancelled);
        }
        InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable => {
            push_unique_code(
                codes,
                InfrastructureOperationsPartialFailureCode::FirstStageUnavailable,
            );
        }
    }
    match security {
        InfrastructureOperationsSecurityStageOutcome::Completed(_) => {}
        InfrastructureOperationsSecurityStageOutcome::Failed(code) => {
            push_failure_code(
                codes,
                *code,
                InfrastructureOperationsPartialFailureCode::SecurityUnavailable,
            );
        }
        InfrastructureOperationsSecurityStageOutcome::Cancelled => {
            push_unique_code(codes, InfrastructureOperationsPartialFailureCode::Cancelled);
        }
        InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable => {
            push_unique_code(
                codes,
                InfrastructureOperationsPartialFailureCode::FirstStageUnavailable,
            );
        }
    }
}

fn push_failure_code(
    codes: &mut Vec<InfrastructureOperationsPartialFailureCode>,
    code: AgentTaskFailureCode,
    unavailable: InfrastructureOperationsPartialFailureCode,
) {
    let value = match code {
        AgentTaskFailureCode::RuntimeStartFailed => {
            InfrastructureOperationsPartialFailureCode::RuntimeStartFailed
        }
        AgentTaskFailureCode::RuntimeOutputInvalid => {
            InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput
        }
        AgentTaskFailureCode::RuntimeUnavailable
        | AgentTaskFailureCode::RuntimeStateMismatch
        | AgentTaskFailureCode::RuntimeEventRejected
        | AgentTaskFailureCode::RuntimeEventLimitExceeded
        | AgentTaskFailureCode::RuntimeReported(_) => unavailable,
    };
    push_unique_code(codes, value);
}

fn push_unique_code(
    codes: &mut Vec<InfrastructureOperationsPartialFailureCode>,
    value: InfrastructureOperationsPartialFailureCode,
) {
    if !codes.contains(&value) {
        codes.push(value);
    }
}

const fn partial_failure_code_label(
    code: InfrastructureOperationsPartialFailureCode,
) -> &'static str {
    match code {
        InfrastructureOperationsPartialFailureCode::FirstStageUnavailable => {
            "first-stage-unavailable"
        }
        InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability => {
            "contains-denied-capability"
        }
        InfrastructureOperationsPartialFailureCode::InvalidStructuredOutput => {
            "invalid-structured-output"
        }
        InfrastructureOperationsPartialFailureCode::QaUnavailable => "qa-unavailable",
        InfrastructureOperationsPartialFailureCode::QaIncomplete => "qa-incomplete",
        InfrastructureOperationsPartialFailureCode::SecurityUnavailable => "security-unavailable",
        InfrastructureOperationsPartialFailureCode::RuntimeStartFailed => "runtime-start-failed",
        InfrastructureOperationsPartialFailureCode::RuntimeFailed => "runtime-failed",
        InfrastructureOperationsPartialFailureCode::Cancelled => "cancelled",
    }
}

fn cloud_stage_projection(
    first: &CloudInfrastructureStageOutcome,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
) -> InfrastructureOperationsStageProjection {
    let first_stage = match first {
        CloudInfrastructureStageOutcome::Completed(_) => {
            InfrastructureOperationsStageDisposition::Completed
        }
        CloudInfrastructureStageOutcome::Failed(code) => {
            InfrastructureOperationsStageDisposition::Failed(*code)
        }
        CloudInfrastructureStageOutcome::Cancelled => {
            InfrastructureOperationsStageDisposition::Cancelled
        }
    };
    later_stage_projection(first_stage, qa, security)
}

fn systems_stage_projection(
    first: &SystemsOperationsStageOutcome,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
) -> InfrastructureOperationsStageProjection {
    let first_stage = match first {
        SystemsOperationsStageOutcome::Completed(_) => {
            InfrastructureOperationsStageDisposition::Completed
        }
        SystemsOperationsStageOutcome::Failed(code) => {
            InfrastructureOperationsStageDisposition::Failed(*code)
        }
        SystemsOperationsStageOutcome::Cancelled => {
            InfrastructureOperationsStageDisposition::Cancelled
        }
    };
    later_stage_projection(first_stage, qa, security)
}

fn later_stage_projection(
    first_stage: InfrastructureOperationsStageDisposition,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
) -> InfrastructureOperationsStageProjection {
    let qa = match qa {
        InfrastructureOperationsQaStageOutcome::Completed(_) => {
            InfrastructureOperationsStageDisposition::Completed
        }
        InfrastructureOperationsQaStageOutcome::Failed(code) => {
            InfrastructureOperationsStageDisposition::Failed(*code)
        }
        InfrastructureOperationsQaStageOutcome::Cancelled => {
            InfrastructureOperationsStageDisposition::Cancelled
        }
        InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable => {
            InfrastructureOperationsStageDisposition::SkippedFirstStageUnavailable
        }
    };
    let security = match security {
        InfrastructureOperationsSecurityStageOutcome::Completed(_) => {
            InfrastructureOperationsStageDisposition::Completed
        }
        InfrastructureOperationsSecurityStageOutcome::Failed(code) => {
            InfrastructureOperationsStageDisposition::Failed(*code)
        }
        InfrastructureOperationsSecurityStageOutcome::Cancelled => {
            InfrastructureOperationsStageDisposition::Cancelled
        }
        InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable => {
            InfrastructureOperationsStageDisposition::SkippedFirstStageUnavailable
        }
    };
    InfrastructureOperationsStageProjection {
        first_stage,
        qa,
        security,
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceRefWire {
    namespace: InfrastructureOperationsEvidenceNamespace,
    id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InfrastructureFindingWire {
    id: String,
    category: InfrastructureFindingCategory,
    statement: String,
    confidence: InfrastructureOperationsConfidence,
    references: Vec<EvidenceRefWire>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DiagnosticFindingWire {
    id: String,
    category: DiagnosticFindingCategory,
    statement: String,
    confidence: InfrastructureOperationsConfidence,
    basis: DiagnosticFindingBasis,
    references: Vec<EvidenceRefWire>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProposedValidationWire {
    id: String,
    text: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChangePlanWire {
    objective: String,
    affected_fixture_ids: Vec<String>,
    proposed_changes: Vec<String>,
    risks: Vec<String>,
    proposed_validation: Vec<ProposedValidationWire>,
    rollback_considerations: String,
    capabilities: Vec<CloudInfrastructureCapability>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InfrastructureAssessmentWire {
    version: String,
    objective_disposition: InfrastructureObjectiveDisposition,
    fixture_ids: Vec<String>,
    findings: Vec<InfrastructureFindingWire>,
    limitations: Vec<String>,
    unresolved_questions: Vec<String>,
    change_plan: ChangePlanWire,
    fixture_based: bool,
    live_inventory_performed: bool,
    commands_executed: bool,
    credentials_loaded: bool,
    effects_performed: bool,
    terraform_fmt_check: InfrastructureOperationsEvidenceStatus,
    terraform_validate: InfrastructureOperationsEvidenceStatus,
    provider_initialization: InfrastructureOperationsEvidenceStatus,
    terraform_plan: InfrastructureOperationsEvidenceStatus,
    terraform_apply: InfrastructureOperationsEvidenceStatus,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OperationalAssessmentWire {
    version: String,
    fixture_ids: Vec<String>,
    findings: Vec<DiagnosticFindingWire>,
    limitations: Vec<String>,
    unresolved_questions: Vec<String>,
    diagnostic_plan: Vec<String>,
    remediation_plan: Vec<String>,
    rollback_considerations: String,
    capabilities: Vec<SystemsOperationsCapability>,
    fixture_based: bool,
    live_diagnostics_performed: bool,
    commands_executed: bool,
    credentials_loaded: bool,
    effects_performed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CriterionCoverageWire {
    criterion_id: String,
    disposition: InfrastructureOperationsCriterionDisposition,
    references: Vec<EvidenceRefWire>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct QaFindingWire {
    id: String,
    statement: String,
    references: Vec<EvidenceRefWire>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct QaCheckWire {
    id: String,
    text: String,
    references: Vec<EvidenceRefWire>,
    status: InfrastructureOperationsEvidenceStatus,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidationReportWire {
    version: String,
    coverage: Vec<CriterionCoverageWire>,
    findings: Vec<QaFindingWire>,
    proposed_checks: Vec<QaCheckWire>,
    gaps: Vec<String>,
    conclusion: InfrastructureOperationsValidationConclusion,
    advisory_only: bool,
    approval_authority: bool,
    evidence_executed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskFindingWire {
    id: String,
    category: InfrastructureOperationsRiskCategory,
    severity: InfrastructureOperationsRiskSeverity,
    confidence: InfrastructureOperationsConfidence,
    statement: String,
    basis: InfrastructureOperationsRiskBasis,
    references: Vec<EvidenceRefWire>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskAssessmentWire {
    version: String,
    findings: Vec<RiskFindingWire>,
    unresolved_risks: Vec<String>,
    follow_ups: Vec<String>,
    dependency_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    provider_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    credential_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    platform_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    audit_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    rollback_evidence: InfrastructureOperationsSupportingEvidenceStatus,
    advisory_only: bool,
    authorization_granted: bool,
    remediation_executed: bool,
    credentials_loaded: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SynthesisWire {
    version: String,
    summary: String,
    fixture_ids: Vec<String>,
    unresolved_issues: Vec<String>,
    status: InfrastructureOperationsReviewStatus,
    fixture_based: bool,
    live_inventory_performed: bool,
    commands_executed: bool,
    credentials_loaded: bool,
    effects_performed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CloudSynthesisWire {
    version: String,
    summary: String,
    fixture_ids: Vec<String>,
    unresolved_issues: Vec<String>,
    status: InfrastructureOperationsReviewStatus,
    fixture_based: bool,
    live_inventory_performed: bool,
    commands_executed: bool,
    credentials_loaded: bool,
    effects_performed: bool,
    terraform_fmt_check: InfrastructureOperationsEvidenceStatus,
    terraform_validate: InfrastructureOperationsEvidenceStatus,
    provider_initialization: InfrastructureOperationsEvidenceStatus,
    terraform_plan: InfrastructureOperationsEvidenceStatus,
    terraform_apply: InfrastructureOperationsEvidenceStatus,
}

fn parse_infrastructure_assessment(
    data: &WorkflowRequestData,
    task_id: AgentTaskId,
    raw: &str,
) -> InfrastructureOperationsResult<InfrastructureAssessment> {
    let wire: InfrastructureAssessmentWire = parse_exact(raw)?;
    require_version_and_disclosure(
        &wire.version,
        wire.fixture_based,
        wire.live_inventory_performed,
        wire.commands_executed,
        wire.credentials_loaded,
        wire.effects_performed,
    )?;
    let terraform_execution = TerraformExecutionDisclosure::from_not_run_fields(
        wire.terraform_fmt_check,
        wire.terraform_validate,
        wire.provider_initialization,
        wire.terraform_plan,
        wire.terraform_apply,
    )?;
    let fixture_ids = parse_exact_fixture_ids(data, wire.fixture_ids)?;
    let findings = wire
        .findings
        .into_iter()
        .map(|finding| {
            validate_model_text(&finding.statement)?;
            let references = parse_refs(data, finding.references, None, None)?;
            ensure_count(
                references.len(),
                1,
                MAX_INFRASTRUCTURE_OPERATIONS_REFERENCES,
            )?;
            if !references
                .iter()
                .any(|reference| direct_reference_is_observed(data, reference))
            {
                return Err(InfrastructureOperationsError::EvidenceInconsistent);
            }
            Ok(InfrastructureFinding {
                id: InfrastructureOperationsId::new(finding.id)?,
                category: finding.category,
                statement: finding.statement,
                confidence: finding.confidence,
                references,
            })
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_count(findings.len(), 1, MAX_INFRASTRUCTURE_OPERATIONS_ITEMS)?;
    ensure_unique_by(findings.iter().map(|finding| &finding.id))?;
    validate_text_list(&wire.limitations)?;
    validate_text_list(&wire.unresolved_questions)?;
    let plan = wire.change_plan;
    validate_model_text(&plan.objective)?;
    let affected_fixture_ids = parse_fixture_ids(data, plan.affected_fixture_ids, 1)?;
    validate_text_list(&plan.proposed_changes)?;
    validate_text_list(&plan.risks)?;
    ensure_count(
        plan.proposed_validation.len(),
        1,
        MAX_INFRASTRUCTURE_OPERATIONS_ITEMS,
    )?;
    let proposed_validation = plan
        .proposed_validation
        .into_iter()
        .map(|step| {
            validate_model_text(&step.text)?;
            Ok(ProposedValidationStep {
                id: InfrastructureOperationsId::new(step.id)?,
                text: step.text,
            })
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_unique_by(proposed_validation.iter().map(|step| &step.id))?;
    validate_model_text(&plan.rollback_considerations)?;
    ensure_unique(&plan.capabilities, 1, MAX_INFRASTRUCTURE_OPERATIONS_ITEMS)?;
    let mut capability_text = Vec::new();
    capability_text.extend(findings.iter().map(InfrastructureFinding::statement));
    capability_text.extend(wire.limitations.iter().map(String::as_str));
    capability_text.extend(wire.unresolved_questions.iter().map(String::as_str));
    capability_text.push(plan.objective.as_str());
    capability_text.extend(plan.proposed_changes.iter().map(String::as_str));
    capability_text.extend(plan.risks.iter().map(String::as_str));
    capability_text.extend(proposed_validation.iter().map(ProposedValidationStep::text));
    capability_text.push(plan.rollback_considerations.as_str());
    validate_cloud_capability_consistency(&capability_text, &plan.capabilities)?;
    let quality = if plan.capabilities.iter().any(|capability| {
        capability.disposition() == InfrastructureOperationsCapabilityDisposition::Denied
    }) {
        InfrastructureOperationsAssessmentQuality::PartialDeniedCapability
    } else {
        InfrastructureOperationsAssessmentQuality::Complete
    };
    Ok(InfrastructureAssessment {
        task_id,
        objective_disposition: wire.objective_disposition,
        fixture_ids,
        findings,
        limitations: wire.limitations,
        unresolved_questions: wire.unresolved_questions,
        change_plan: ChangePlan {
            objective: plan.objective,
            affected_fixture_ids,
            proposed_changes: plan.proposed_changes,
            risks: plan.risks,
            proposed_validation,
            rollback_considerations: plan.rollback_considerations,
            capabilities: plan.capabilities,
        },
        terraform_execution,
        quality,
        transfer: bounded_transfer(raw)?,
    })
}

fn parse_operational_assessment(
    data: &WorkflowRequestData,
    task_id: AgentTaskId,
    raw: &str,
) -> InfrastructureOperationsResult<OperationalAssessment> {
    let wire: OperationalAssessmentWire = parse_exact(raw)?;
    require_version_and_disclosure(
        &wire.version,
        wire.fixture_based,
        wire.live_diagnostics_performed,
        wire.commands_executed,
        wire.credentials_loaded,
        wire.effects_performed,
    )?;
    let fixture_ids = parse_exact_fixture_ids(data, wire.fixture_ids)?;
    let findings = wire
        .findings
        .into_iter()
        .map(|finding| {
            validate_model_text(&finding.statement)?;
            let references = parse_refs(data, finding.references, None, None)?;
            match finding.basis {
                DiagnosticFindingBasis::EvidenceBound if references.is_empty() => {
                    return Err(InfrastructureOperationsError::EvidenceInconsistent);
                }
                DiagnosticFindingBasis::EvidenceBound
                    if !references
                        .iter()
                        .any(|reference| direct_reference_is_observed(data, reference)) =>
                {
                    return Err(InfrastructureOperationsError::EvidenceInconsistent);
                }
                DiagnosticFindingBasis::Hypothesis if !references.is_empty() => {
                    return Err(InfrastructureOperationsError::EvidenceInconsistent);
                }
                DiagnosticFindingBasis::EvidenceBound | DiagnosticFindingBasis::Hypothesis => {}
            }
            Ok(DiagnosticFinding {
                id: InfrastructureOperationsId::new(finding.id)?,
                category: finding.category,
                statement: finding.statement,
                confidence: finding.confidence,
                basis: finding.basis,
                references,
            })
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_count(findings.len(), 1, MAX_INFRASTRUCTURE_OPERATIONS_ITEMS)?;
    ensure_unique_by(findings.iter().map(|finding| &finding.id))?;
    validate_text_list(&wire.limitations)?;
    validate_text_list(&wire.unresolved_questions)?;
    validate_text_list(&wire.diagnostic_plan)?;
    validate_text_list(&wire.remediation_plan)?;
    validate_model_text(&wire.rollback_considerations)?;
    ensure_unique(&wire.capabilities, 1, MAX_INFRASTRUCTURE_OPERATIONS_ITEMS)?;
    let mut capability_text = Vec::new();
    capability_text.extend(findings.iter().map(DiagnosticFinding::statement));
    capability_text.extend(wire.limitations.iter().map(String::as_str));
    capability_text.extend(wire.unresolved_questions.iter().map(String::as_str));
    capability_text.extend(wire.diagnostic_plan.iter().map(String::as_str));
    capability_text.extend(wire.remediation_plan.iter().map(String::as_str));
    capability_text.push(wire.rollback_considerations.as_str());
    validate_systems_capability_consistency(&capability_text, &wire.capabilities)?;
    let quality = if wire.capabilities.iter().any(|capability| {
        capability.disposition() == InfrastructureOperationsCapabilityDisposition::Denied
    }) {
        InfrastructureOperationsAssessmentQuality::PartialDeniedCapability
    } else {
        InfrastructureOperationsAssessmentQuality::Complete
    };
    Ok(OperationalAssessment {
        task_id,
        fixture_ids,
        findings,
        limitations: wire.limitations,
        unresolved_questions: wire.unresolved_questions,
        diagnostic_plan: wire.diagnostic_plan,
        remediation_plan: wire.remediation_plan,
        rollback_considerations: wire.rollback_considerations,
        capabilities: wire.capabilities,
        quality,
        transfer: bounded_transfer(raw)?,
    })
}

fn parse_validation_report(
    data: &WorkflowRequestData,
    workflow_kind: InfrastructureOperationsWorkflowKind,
    task_id: AgentTaskId,
    assessment_task_id: AgentTaskId,
    assessment_refs: &[InfrastructureOperationsEvidenceRef],
    raw: &str,
) -> InfrastructureOperationsResult<ValidationReport> {
    let wire: ValidationReportWire = parse_exact(raw)?;
    if wire.version != "v1"
        || !wire.advisory_only
        || wire.approval_authority
        || wire.evidence_executed
    {
        return Err(InfrastructureOperationsError::AuthorityClaim);
    }
    ensure_count(
        wire.coverage.len(),
        data.criteria.len(),
        data.criteria.len(),
    )?;
    let coverage = wire
        .coverage
        .into_iter()
        .map(|coverage| {
            let criterion_id = InfrastructureOperationsId::new(coverage.criterion_id)?;
            if !data.criterion(&criterion_id) {
                return Err(InfrastructureOperationsError::UnknownReference);
            }
            let references = parse_refs(data, coverage.references, Some(assessment_refs), None)?;
            if references.iter().any(|reference| {
                reference.namespace
                    == InfrastructureOperationsEvidenceNamespace::ApplicationEvidence
                    && data
                        .evidence(&reference.id)
                        .is_none_or(|evidence| !evidence.criterion_ids.contains(&criterion_id))
            }) {
                return Err(InfrastructureOperationsError::EvidenceInconsistent);
            }
            if coverage.disposition == InfrastructureOperationsCriterionDisposition::Demonstrated
                && !references.iter().any(|reference| {
                    reference.namespace
                        == InfrastructureOperationsEvidenceNamespace::ApplicationEvidence
                        && data.evidence(&reference.id).is_some_and(|evidence| {
                            evidence.status
                                == InfrastructureOperationsEvidenceStatus::ObservedFixture
                                && evidence.criterion_ids.contains(&criterion_id)
                        })
                })
            {
                return Err(InfrastructureOperationsError::EvidenceInconsistent);
            }
            Ok(InfrastructureOperationsCriterionCoverage {
                criterion_id,
                disposition: coverage.disposition,
                references,
            })
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_unique_by(coverage.iter().map(|item| &item.criterion_id))?;
    let findings = wire
        .findings
        .into_iter()
        .map(|finding| {
            validate_model_text(&finding.statement)?;
            Ok(InfrastructureOperationsQaFinding {
                id: InfrastructureOperationsId::new(finding.id)?,
                statement: finding.statement,
                references: parse_refs(data, finding.references, Some(assessment_refs), None)?,
            })
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_count(findings.len(), 0, MAX_INFRASTRUCTURE_OPERATIONS_ITEMS)?;
    ensure_unique_by(findings.iter().map(|item| &item.id))?;
    let proposed_checks = wire
        .proposed_checks
        .into_iter()
        .map(|check| {
            if check.status != InfrastructureOperationsEvidenceStatus::NotRun {
                return Err(InfrastructureOperationsError::EvidenceInconsistent);
            }
            validate_model_text(&check.text)?;
            Ok(InfrastructureOperationsQaCheck {
                id: InfrastructureOperationsId::new(check.id)?,
                text: check.text,
                references: parse_refs(data, check.references, Some(assessment_refs), None)?,
                status: check.status,
            })
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_count(
        proposed_checks.len(),
        0,
        MAX_INFRASTRUCTURE_OPERATIONS_ITEMS,
    )?;
    ensure_unique_by(proposed_checks.iter().map(|item| &item.id))?;
    validate_text_list(&wire.gaps)?;
    let mut effect_text = Vec::new();
    effect_text.extend(findings.iter().map(|finding| finding.statement.as_str()));
    effect_text.extend(proposed_checks.iter().map(|check| check.text.as_str()));
    effect_text.extend(wire.gaps.iter().map(String::as_str));
    validate_no_effect_requests(&effect_text)?;
    let missing = coverage.iter().any(|item| {
        item.disposition == InfrastructureOperationsCriterionDisposition::NotDemonstrated
    });
    if (wire.conclusion == InfrastructureOperationsValidationConclusion::Adequate) == missing {
        return Err(InfrastructureOperationsError::StageInconsistent);
    }
    Ok(ValidationReport {
        workflow_kind,
        task_id,
        assessment_task_id,
        coverage,
        findings,
        proposed_checks,
        gaps: wire.gaps,
        conclusion: wire.conclusion,
        transfer: bounded_transfer(raw)?,
    })
}

fn parse_risk_assessment(
    data: &WorkflowRequestData,
    workflow_kind: InfrastructureOperationsWorkflowKind,
    task_id: AgentTaskId,
    assessment_task_id: AgentTaskId,
    assessment_refs: &[InfrastructureOperationsEvidenceRef],
    qa: &InfrastructureOperationsQaStageOutcome,
    raw: &str,
) -> InfrastructureOperationsResult<RiskAssessment> {
    let wire: RiskAssessmentWire = parse_exact(raw)?;
    if wire.version != "v1"
        || !wire.advisory_only
        || wire.authorization_granted
        || wire.remediation_executed
        || wire.credentials_loaded
    {
        return Err(InfrastructureOperationsError::AuthorityClaim);
    }
    let qa_report = match qa {
        InfrastructureOperationsQaStageOutcome::Completed(report) => Some(report),
        _ => None,
    };
    let findings = wire
        .findings
        .into_iter()
        .map(|finding| {
            validate_model_text(&finding.statement)?;
            let references =
                parse_refs(data, finding.references, Some(assessment_refs), qa_report)?;
            match finding.basis {
                InfrastructureOperationsRiskBasis::EvidenceBound if references.is_empty() => {
                    return Err(InfrastructureOperationsError::EvidenceInconsistent);
                }
                InfrastructureOperationsRiskBasis::EvidenceBound
                    if !references.iter().any(|reference| {
                        security_reference_is_observed(data, reference, assessment_refs, qa_report)
                    }) =>
                {
                    return Err(InfrastructureOperationsError::EvidenceInconsistent);
                }
                InfrastructureOperationsRiskBasis::Hypothesis
                    if !references.is_empty()
                        || finding.confidence == InfrastructureOperationsConfidence::High =>
                {
                    return Err(InfrastructureOperationsError::EvidenceInconsistent);
                }
                InfrastructureOperationsRiskBasis::EvidenceBound
                | InfrastructureOperationsRiskBasis::Hypothesis => {}
            }
            Ok(InfrastructureOperationsRiskFinding {
                id: InfrastructureOperationsId::new(finding.id)?,
                category: finding.category,
                severity: finding.severity,
                confidence: finding.confidence,
                statement: finding.statement,
                basis: finding.basis,
                references,
            })
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_count(findings.len(), 0, MAX_INFRASTRUCTURE_OPERATIONS_ITEMS)?;
    ensure_unique_by(findings.iter().map(|item| &item.id))?;
    validate_text_list(&wire.unresolved_risks)?;
    validate_text_list(&wire.follow_ups)?;
    let mut effect_text = Vec::new();
    effect_text.extend(findings.iter().map(|finding| finding.statement.as_str()));
    effect_text.extend(wire.unresolved_risks.iter().map(String::as_str));
    effect_text.extend(wire.follow_ups.iter().map(String::as_str));
    validate_no_effect_requests(&effect_text)?;
    if [
        wire.dependency_evidence,
        wire.provider_evidence,
        wire.credential_evidence,
        wire.platform_evidence,
        wire.audit_evidence,
        wire.rollback_evidence,
    ]
    .iter()
    .any(|status| *status != InfrastructureOperationsSupportingEvidenceStatus::Unavailable)
    {
        return Err(InfrastructureOperationsError::AuthorityClaim);
    }
    Ok(RiskAssessment {
        workflow_kind,
        task_id,
        assessment_task_id,
        qa_task_id: qa_report.map(|report| report.task_id.clone()),
        findings,
        unresolved_risks: wire.unresolved_risks,
        follow_ups: wire.follow_ups,
        dependency_evidence: wire.dependency_evidence,
        provider_evidence: wire.provider_evidence,
        credential_evidence: wire.credential_evidence,
        platform_evidence: wire.platform_evidence,
        audit_evidence: wire.audit_evidence,
        rollback_evidence: wire.rollback_evidence,
        transfer: bounded_transfer(raw)?,
    })
}

fn parse_cloud_synthesis(
    data: &WorkflowRequestData,
    first: &CloudInfrastructureStageOutcome,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
    raw: &str,
) -> InfrastructureOperationsResult<CloudInfrastructureSynthesis> {
    let stage_projection = cloud_stage_projection(first, qa, security);
    let partial_failure_codes = infrastructure_partial_failure_codes(first, qa, security);
    let wire: CloudSynthesisWire = parse_exact(raw)?;
    let terraform_execution = TerraformExecutionDisclosure::from_not_run_fields(
        wire.terraform_fmt_check,
        wire.terraform_validate,
        wire.provider_initialization,
        wire.terraform_plan,
        wire.terraform_apply,
    )?;
    let synthesis = parse_synthesis_wire(
        data,
        qa,
        security,
        stage_projection,
        partial_failure_codes,
        SynthesisWire {
            version: wire.version,
            summary: wire.summary,
            fixture_ids: wire.fixture_ids,
            unresolved_issues: wire.unresolved_issues,
            status: wire.status,
            fixture_based: wire.fixture_based,
            live_inventory_performed: wire.live_inventory_performed,
            commands_executed: wire.commands_executed,
            credentials_loaded: wire.credentials_loaded,
            effects_performed: wire.effects_performed,
        },
    )?;
    Ok(CloudInfrastructureSynthesis(synthesis, terraform_execution))
}

fn parse_systems_synthesis(
    data: &WorkflowRequestData,
    first: &SystemsOperationsStageOutcome,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
    raw: &str,
) -> InfrastructureOperationsResult<SystemsOperationsSynthesis> {
    let stage_projection = systems_stage_projection(first, qa, security);
    let partial_failure_codes = systems_partial_failure_codes(first, qa, security);
    let wire: SynthesisWire = parse_exact(raw)?;
    Ok(SystemsOperationsSynthesis(parse_synthesis_wire(
        data,
        qa,
        security,
        stage_projection,
        partial_failure_codes,
        wire,
    )?))
}

fn parse_synthesis_wire(
    data: &WorkflowRequestData,
    qa: &InfrastructureOperationsQaStageOutcome,
    security: &InfrastructureOperationsSecurityStageOutcome,
    stage_projection: InfrastructureOperationsStageProjection,
    partial_failure_codes: Vec<InfrastructureOperationsPartialFailureCode>,
    wire: SynthesisWire,
) -> InfrastructureOperationsResult<InfrastructureOperationsSynthesis> {
    require_version_and_disclosure(
        &wire.version,
        wire.fixture_based,
        wire.live_inventory_performed,
        wire.commands_executed,
        wire.credentials_loaded,
        wire.effects_performed,
    )?;
    validate_model_text(&wire.summary)?;
    let fixture_ids = parse_exact_fixture_ids(data, wire.fixture_ids)?;
    validate_text_list(&wire.unresolved_issues)?;
    let mut effect_text = vec![wire.summary.as_str()];
    effect_text.extend(wire.unresolved_issues.iter().map(String::as_str));
    validate_no_effect_requests(&effect_text)?;
    let denied = partial_failure_codes
        .contains(&InfrastructureOperationsPartialFailureCode::ContainsDeniedCapability);
    validate_approval_summary_consistency(&wire.summary, denied)?;
    let complete = stage_projection.first_stage
        == InfrastructureOperationsStageDisposition::Completed
        && !denied
        && matches!(qa, InfrastructureOperationsQaStageOutcome::Completed(report) if report.conclusion == InfrastructureOperationsValidationConclusion::Adequate)
        && matches!(
            security,
            InfrastructureOperationsSecurityStageOutcome::Completed(_)
        );
    let expected_status = if complete {
        InfrastructureOperationsReviewStatus::Complete
    } else {
        InfrastructureOperationsReviewStatus::Partial
    };
    if wire.status != expected_status || (!complete && wire.unresolved_issues.is_empty()) {
        return Err(InfrastructureOperationsError::StageInconsistent);
    }
    validate_synthesis_summary_disclosure(&wire.summary, wire.status)?;
    Ok(InfrastructureOperationsSynthesis {
        summary: wire.summary,
        fixture_ids,
        unresolved_issues: wire.unresolved_issues,
        approval_requirement: if denied {
            InfrastructureOperationsApprovalRequirement::RequiredBeforeConsequentialAction
        } else {
            InfrastructureOperationsApprovalRequirement::NotApplicable
        },
        execution_disposition: InfrastructureOperationsExecutionDisposition::NotAttempted,
        status: wire.status,
        stage_projection,
        partial_failure_codes,
    })
}

fn validate_synthesis_summary_disclosure(
    summary: &str,
    status: InfrastructureOperationsReviewStatus,
) -> InfrastructureOperationsResult<()> {
    let tokens = normalized_claim_tokens(summary);
    let fixture_disclosed = contains_token_sequence(&tokens, &["fixture", "only"])
        || contains_token_sequence(&tokens, &["fixture", "based"]);
    let no_command = contains_token_sequence(&tokens, &["no", "command", "ran"])
        || contains_token_sequence(&tokens, &["no", "commands", "ran"])
        || contains_token_sequence(&tokens, &["no", "command", "was", "executed"])
        || contains_token_sequence(&tokens, &["no", "commands", "were", "executed"])
        || contains_token_sequence(&tokens, &["no", "check", "ran"])
        || contains_token_sequence(&tokens, &["no", "checks", "ran"])
        || contains_token_sequence(&tokens, &["no", "terraform", "check", "ran"])
        || contains_token_sequence(&tokens, &["no", "terraform", "checks", "ran"])
        || contains_token_sequence(&tokens, &["no", "live", "check", "ran"])
        || contains_token_sequence(&tokens, &["no", "live", "checks", "ran"]);
    let no_effect = contains_token_sequence(&tokens, &["no", "effect", "was", "performed"])
        || contains_token_sequence(&tokens, &["no", "effects", "were", "performed"])
        || contains_token_sequence(&tokens, &["no", "state", "changed"])
        || contains_token_sequence(&tokens, &["no", "system", "state", "changed"])
        || contains_token_sequence(&tokens, &["no", "infrastructure", "state", "changed"]);
    let partial_disclosed = status != InfrastructureOperationsReviewStatus::Partial
        || tokens
            .iter()
            .any(|token| token == "partial" || token == "incomplete");
    if fixture_disclosed && no_command && no_effect && partial_disclosed {
        Ok(())
    } else {
        Err(InfrastructureOperationsError::StageInconsistent)
    }
}

fn validate_approval_summary_consistency(
    summary: &str,
    approval_required: bool,
) -> InfrastructureOperationsResult<()> {
    let tokens = normalized_claim_tokens(summary);
    let says_not_required = [
        &["approval", "is", "not", "required"][..],
        &["approval", "not", "required"],
        &["no", "approval", "required"],
        &["approval", "is", "unnecessary"],
        &["approval", "unnecessary"],
    ]
    .iter()
    .any(|phrase| contains_token_sequence(&tokens, phrase));
    let says_required = [
        &["approval", "is", "required"][..],
        &["approval", "required"],
        &["requires", "approval"],
    ]
    .iter()
    .any(|phrase| contains_token_sequence(&tokens, phrase));
    if (approval_required && says_not_required) || (!approval_required && says_required) {
        Err(InfrastructureOperationsError::StageInconsistent)
    } else {
        Ok(())
    }
}

fn contains_token_sequence(tokens: &[String], sequence: &[&str]) -> bool {
    tokens.windows(sequence.len()).any(|window| {
        window
            .iter()
            .map(String::as_str)
            .eq(sequence.iter().copied())
    })
}

fn parse_exact<T: DeserializeOwned>(raw: &str) -> InfrastructureOperationsResult<T> {
    if raw.trim() != raw
        || !raw.starts_with('{')
        || !raw.ends_with('}')
        || raw.chars().count() > MAX_INFRASTRUCTURE_OPERATIONS_RESULT_CHARACTERS
        || raw.len() > MAX_INFRASTRUCTURE_OPERATIONS_RESULT_BYTES
    {
        return Err(InfrastructureOperationsError::InvalidStructuredOutput);
    }
    let value: Value = serde_json::from_str(raw)
        .map_err(|_| InfrastructureOperationsError::InvalidStructuredOutput)?;
    reject_explicit_secret_fields(&value)?;
    reject_value_credentials(&value)?;
    let mut deserializer = serde_json::Deserializer::from_str(raw);
    let parsed = T::deserialize(&mut deserializer)
        .map_err(|_| InfrastructureOperationsError::InvalidStructuredOutput)?;
    deserializer
        .end()
        .map_err(|_| InfrastructureOperationsError::InvalidStructuredOutput)?;
    Ok(parsed)
}

fn reject_explicit_secret_fields(value: &Value) -> InfrastructureOperationsResult<()> {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                let key = key.trim_ascii().to_ascii_lowercase();
                if matches!(
                    key.as_str(),
                    "secret"
                        | "password"
                        | "token"
                        | "credential"
                        | "private_key"
                        | "access_key"
                        | "client_secret"
                ) {
                    return Err(InfrastructureOperationsError::CredentialContentRejected);
                }
                let reasoning_key = key.replace(['-', ' '], "_");
                if matches!(
                    reasoning_key.as_str(),
                    "chain_of_thought"
                        | "chain_of_thoughts"
                        | "hidden_reasoning"
                        | "private_reasoning"
                        | "internal_reasoning"
                        | "scratchpad"
                ) {
                    return Err(InfrastructureOperationsError::AuthorityClaim);
                }
                reject_explicit_secret_fields(value)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                reject_explicit_secret_fields(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn reject_value_credentials(value: &Value) -> InfrastructureOperationsResult<()> {
    match value {
        Value::String(value) => reject_credential_text(value),
        Value::Array(values) => values.iter().try_for_each(reject_value_credentials),
        Value::Object(map) => map.values().try_for_each(reject_value_credentials),
        _ => Ok(()),
    }
}

fn require_version_and_disclosure(
    version: &str,
    fixture_based: bool,
    live_or_diagnostics: bool,
    commands_executed: bool,
    credentials_loaded: bool,
    effects_performed: bool,
) -> InfrastructureOperationsResult<()> {
    if version != "v1"
        || !fixture_based
        || live_or_diagnostics
        || commands_executed
        || credentials_loaded
        || effects_performed
    {
        Err(InfrastructureOperationsError::AuthorityClaim)
    } else {
        Ok(())
    }
}

fn parse_exact_fixture_ids(
    data: &WorkflowRequestData,
    values: Vec<String>,
) -> InfrastructureOperationsResult<Vec<InfrastructureOperationsId>> {
    let parsed = parse_fixture_ids(data, values, 1)?;
    if parsed != data.fixture_ids() {
        Err(InfrastructureOperationsError::StageInconsistent)
    } else {
        Ok(parsed)
    }
}

fn parse_fixture_ids(
    data: &WorkflowRequestData,
    values: Vec<String>,
    minimum: usize,
) -> InfrastructureOperationsResult<Vec<InfrastructureOperationsId>> {
    let parsed = values
        .into_iter()
        .map(InfrastructureOperationsId::new)
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_unique(&parsed, minimum, MAX_INFRASTRUCTURE_OPERATIONS_REFERENCES)?;
    if parsed.iter().all(|id| data.fixture(id)) {
        Ok(parsed)
    } else {
        Err(InfrastructureOperationsError::UnknownReference)
    }
}

fn parse_refs(
    data: &WorkflowRequestData,
    values: Vec<EvidenceRefWire>,
    assessment_refs: Option<&[InfrastructureOperationsEvidenceRef]>,
    qa: Option<&ValidationReport>,
) -> InfrastructureOperationsResult<Vec<InfrastructureOperationsEvidenceRef>> {
    ensure_count(values.len(), 0, MAX_INFRASTRUCTURE_OPERATIONS_REFERENCES)?;
    let parsed = values
        .into_iter()
        .map(|reference| {
            InfrastructureOperationsEvidenceRef::new(reference.namespace, reference.id)
        })
        .collect::<InfrastructureOperationsResult<Vec<_>>>()?;
    ensure_unique(&parsed, 0, MAX_INFRASTRUCTURE_OPERATIONS_REFERENCES)?;
    for reference in &parsed {
        let valid = match reference.namespace {
            InfrastructureOperationsEvidenceNamespace::Fixture => data.fixture(&reference.id),
            InfrastructureOperationsEvidenceNamespace::ApplicationEvidence => {
                data.evidence(&reference.id).is_some()
            }
            InfrastructureOperationsEvidenceNamespace::AssessmentFinding
            | InfrastructureOperationsEvidenceNamespace::ProposedValidation => {
                assessment_refs.is_some_and(|refs| refs.contains(reference))
            }
            InfrastructureOperationsEvidenceNamespace::QaFinding => {
                qa.is_some_and(|report| report.findings.iter().any(|item| item.id == reference.id))
            }
            InfrastructureOperationsEvidenceNamespace::QaCheck => qa.is_some_and(|report| {
                report
                    .proposed_checks
                    .iter()
                    .any(|item| item.id == reference.id)
            }),
        };
        if !valid {
            return Err(InfrastructureOperationsError::UnknownReference);
        }
    }
    Ok(parsed)
}

fn direct_reference_is_observed(
    data: &WorkflowRequestData,
    reference: &InfrastructureOperationsEvidenceRef,
) -> bool {
    match reference.namespace {
        InfrastructureOperationsEvidenceNamespace::Fixture => data.fixture(&reference.id),
        InfrastructureOperationsEvidenceNamespace::ApplicationEvidence => {
            data.evidence(&reference.id).is_some_and(|evidence| {
                evidence.status == InfrastructureOperationsEvidenceStatus::ObservedFixture
            })
        }
        InfrastructureOperationsEvidenceNamespace::AssessmentFinding
        | InfrastructureOperationsEvidenceNamespace::ProposedValidation
        | InfrastructureOperationsEvidenceNamespace::QaFinding
        | InfrastructureOperationsEvidenceNamespace::QaCheck => false,
    }
}

fn security_reference_is_observed(
    data: &WorkflowRequestData,
    reference: &InfrastructureOperationsEvidenceRef,
    assessment_evidence_refs: &[InfrastructureOperationsEvidenceRef],
    qa: Option<&ValidationReport>,
) -> bool {
    match reference.namespace {
        InfrastructureOperationsEvidenceNamespace::Fixture
        | InfrastructureOperationsEvidenceNamespace::ApplicationEvidence => {
            direct_reference_is_observed(data, reference)
        }
        InfrastructureOperationsEvidenceNamespace::AssessmentFinding => {
            assessment_evidence_refs.contains(reference)
        }
        InfrastructureOperationsEvidenceNamespace::QaFinding => qa.is_some_and(|report| {
            report
                .findings
                .iter()
                .find(|finding| finding.id == reference.id)
                .is_some_and(|finding| {
                    finding.references.iter().any(|source| {
                        security_reference_is_observed(data, source, assessment_evidence_refs, None)
                    })
                })
        }),
        InfrastructureOperationsEvidenceNamespace::ProposedValidation
        | InfrastructureOperationsEvidenceNamespace::QaCheck => false,
    }
}

fn validate_model_text(value: &str) -> InfrastructureOperationsResult<()> {
    validate_text(
        value,
        MAX_RESULT_TEXT_CHARACTERS,
        MAX_RESULT_TEXT_BYTES,
        true,
    )?;
    reject_credential_text(value)?;
    reject_path_authority_text(value)?;
    let lower = value.to_ascii_lowercase();
    if lower.contains("://")
        || lower.contains("commands executed=true")
        || lower.contains("credentials loaded=true")
        || lower.contains("effects performed=true")
        || contains_forbidden_claim(&normalized_claim_tokens(value))
    {
        Err(InfrastructureOperationsError::AuthorityClaim)
    } else {
        Ok(())
    }
}

fn normalized_claim_tokens(value: &str) -> Vec<String> {
    let normalized = value.replace("\r\n", "\n");
    let mut token = String::new();
    let mut tokens = Vec::new();
    for character in normalized.chars() {
        if character.is_ascii_alphanumeric() {
            token.push(character.to_ascii_lowercase());
        } else if !token.is_empty() {
            tokens.push(std::mem::take(&mut token));
        }
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

fn contains_forbidden_claim(tokens: &[String]) -> bool {
    const PHRASES: &[&[&str]] = &[
        &["chain", "of", "thought"],
        &["chain", "of", "thoughts"],
        &["hidden", "reasoning"],
        &["private", "reasoning"],
        &["internal", "reasoning"],
        &["secret", "reasoning"],
        &["scratchpad"],
        &["scratch", "pad"],
        &["i", "ran"],
        &["we", "ran"],
        &["i", "executed"],
        &["we", "executed"],
        &["terraform", "apply", "succeeded"],
        &["terraform", "apply", "completed"],
        &["terraform", "apply", "finished", "successfully"],
        &["terraform", "apply", "was", "run"],
        &["terraform", "plan", "succeeded"],
        &["terraform", "plan", "completed"],
        &["terraform", "plan", "was", "run"],
        &["terraform", "validate", "succeeded"],
        &["terraform", "validate", "passed"],
        &["terraform", "validate", "ran", "successfully"],
        &["terraform", "fmt", "succeeded"],
        &["terraform", "fmt", "passed"],
        &["terraform", "fmt", "check", "passed"],
        &["terraform", "fmt", "check", "succeeded"],
        &["provider", "initialization", "completed"],
        &["provider", "initialized"],
        &["live", "inventory", "performed"],
        &["live", "inventory", "completed"],
        &["live", "inventory", "retrieved"],
        &["live", "inventory", "was", "retrieved"],
        &["live", "diagnostics", "performed"],
        &["live", "diagnostics", "completed"],
        &["tests", "executed"],
        &["tests", "passed"],
        &["checks", "executed"],
        &["checks", "passed"],
        &["configuration", "inspected", "live"],
        &["configuration", "was", "inspected", "live"],
        &["config", "inspected", "live"],
        &["config", "was", "inspected", "live"],
        &["service", "inspected", "live"],
        &["service", "was", "inspected", "live"],
        &["process", "inspected", "live"],
        &["process", "was", "inspected", "live"],
        &["log", "inspected", "live"],
        &["log", "was", "inspected", "live"],
        &["logs", "inspected", "live"],
        &["logs", "were", "inspected", "live"],
        &["inspected", "live", "configuration"],
        &["inspected", "live", "config"],
        &["inspected", "live", "service"],
        &["inspected", "live", "process"],
        &["inspected", "live", "log"],
        &["inspected", "live", "logs"],
        &["read", "live", "log"],
        &["read", "live", "logs"],
        &["commands", "were", "executed"],
        &["command", "was", "executed"],
        &["credentials", "loaded"],
        &["credentials", "were", "loaded"],
        &["credential", "loaded"],
        &["loaded", "credentials"],
        &["used", "credentials"],
        &["accessed", "credentials"],
        &["effects", "performed"],
        &["resource", "created"],
        &["resource", "updated"],
        &["resource", "deleted"],
        &["service", "restarted"],
        &["service", "stopped"],
        &["process", "killed"],
        &["system", "rebooted"],
        &["system", "shutdown"],
        &["approval", "granted"],
        &["approval", "has", "been", "granted"],
        &["authorization", "granted"],
        &["permission", "granted"],
        &["policy", "approved"],
        &["authorized", "to", "execute"],
        &["approved", "to", "execute"],
        &["account", "id"],
        &["subscription", "id"],
        &["tenant", "id"],
        &["resource", "id"],
        &["instance", "id"],
        &["project", "id"],
        &["organization", "id"],
        &["host", "identity"],
        &["host", "name"],
        &["hostname"],
    ];
    PHRASES
        .iter()
        .any(|phrase| contains_positive_token_sequence(tokens, phrase))
}

fn reject_path_authority_text(value: &str) -> InfrastructureOperationsResult<()> {
    for line in value.replace("\r\n", "\n").lines() {
        let trimmed = line.trim_ascii();
        let folded = trimmed.to_ascii_lowercase();
        let bytes = trimmed.as_bytes();
        let windows_absolute = bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && matches!(bytes[2], b'/' | b'\\');
        if folded.starts_with("file:")
            || trimmed.starts_with('/')
            || trimmed.starts_with("~/")
            || trimmed.starts_with("../")
            || trimmed.starts_with("..\\")
            || trimmed.contains("/../")
            || trimmed.contains("\\..\\")
            || windows_absolute
        {
            return Err(InfrastructureOperationsError::AuthorityClaim);
        }
    }
    Ok(())
}

fn contains_positive_token_sequence(tokens: &[String], phrase: &[&str]) -> bool {
    tokens
        .windows(phrase.len())
        .enumerate()
        .any(|(index, window)| {
            window.iter().map(String::as_str).eq(phrase.iter().copied())
                && !index.checked_sub(1).is_some_and(|prior| {
                    matches!(tokens[prior].as_str(), "no" | "not" | "never" | "without")
                })
        })
}

fn validate_cloud_capability_consistency(
    values: &[&str],
    capabilities: &[CloudInfrastructureCapability],
) -> InfrastructureOperationsResult<()> {
    let mappings: &[(&[&str], CloudInfrastructureCapability)] = &[
        (
            &["run", "terraform", "plan"],
            CloudInfrastructureCapability::TerraformPlanExecution,
        ),
        (
            &["execute", "terraform", "plan"],
            CloudInfrastructureCapability::TerraformPlanExecution,
        ),
        (
            &["generate", "terraform", "plan"],
            CloudInfrastructureCapability::TerraformPlanExecution,
        ),
        (
            &["run", "terraform", "apply"],
            CloudInfrastructureCapability::TerraformApply,
        ),
        (
            &["apply", "terraform"],
            CloudInfrastructureCapability::TerraformApply,
        ),
        (
            &["execute", "terraform", "apply"],
            CloudInfrastructureCapability::TerraformApply,
        ),
        (
            &["mutate", "terraform", "state"],
            CloudInfrastructureCapability::TerraformStateMutation,
        ),
        (
            &["change", "terraform", "backend"],
            CloudInfrastructureCapability::TerraformBackendChange,
        ),
        (
            &["download", "provider"],
            CloudInfrastructureCapability::ProviderDownload,
        ),
        (
            &["access", "cloud", "inventory"],
            CloudInfrastructureCapability::CloudInventoryAccess,
        ),
        (
            &["access", "live", "inventory"],
            CloudInfrastructureCapability::CloudInventoryAccess,
        ),
        (
            &["open", "cloud", "shell"],
            CloudInfrastructureCapability::CloudShell,
        ),
        (
            &["create", "resource"],
            CloudInfrastructureCapability::ResourceCreateOrUpdate,
        ),
        (
            &["update", "resource"],
            CloudInfrastructureCapability::ResourceCreateOrUpdate,
        ),
        (
            &["delete", "resource"],
            CloudInfrastructureCapability::ResourceDelete,
        ),
        (&["change", "iam"], CloudInfrastructureCapability::IamChange),
        (
            &["change", "firewall"],
            CloudInfrastructureCapability::FirewallChange,
        ),
        (
            &["access", "production"],
            CloudInfrastructureCapability::ProductionAccess,
        ),
        (
            &["access", "credential"],
            CloudInfrastructureCapability::CredentialAccess,
        ),
        (
            &["rotate", "secret"],
            CloudInfrastructureCapability::SecretRotation,
        ),
    ];
    validate_capability_mappings(values, mappings, capabilities)
}

fn validate_systems_capability_consistency(
    values: &[&str],
    capabilities: &[SystemsOperationsCapability],
) -> InfrastructureOperationsResult<()> {
    let mappings: &[(&[&str], SystemsOperationsCapability)] = &[
        (
            &["run", "live", "diagnostic"],
            SystemsOperationsCapability::LiveDiagnostic,
        ),
        (
            &["read", "live", "log"],
            SystemsOperationsCapability::ReadLiveLog,
        ),
        (
            &["inspect", "live", "service"],
            SystemsOperationsCapability::InspectLiveService,
        ),
        (
            &["inspect", "live", "process"],
            SystemsOperationsCapability::InspectLiveProcess,
        ),
        (
            &["inspect", "live", "configuration"],
            SystemsOperationsCapability::InspectLiveConfiguration,
        ),
        (
            &["restart", "service"],
            SystemsOperationsCapability::RestartOrStopService,
        ),
        (
            &["stop", "service"],
            SystemsOperationsCapability::RestartOrStopService,
        ),
        (
            &["reboot", "system"],
            SystemsOperationsCapability::RebootOrShutdown,
        ),
        (
            &["shutdown", "system"],
            SystemsOperationsCapability::RebootOrShutdown,
        ),
        (
            &["kill", "process"],
            SystemsOperationsCapability::KillProcess,
        ),
        (
            &["mutate", "configuration"],
            SystemsOperationsCapability::ConfigurationMutation,
        ),
        (
            &["install", "package"],
            SystemsOperationsCapability::PackageInstall,
        ),
        (
            &["patch", "system"],
            SystemsOperationsCapability::PatchSystem,
        ),
        (
            &["change", "permission"],
            SystemsOperationsCapability::AccountOrPermissionChange,
        ),
        (
            &["change", "account"],
            SystemsOperationsCapability::AccountOrPermissionChange,
        ),
        (
            &["modify", "permission"],
            SystemsOperationsCapability::AccountOrPermissionChange,
        ),
        (
            &["modify", "account"],
            SystemsOperationsCapability::AccountOrPermissionChange,
        ),
        (&["delete", "file"], SystemsOperationsCapability::DeleteFile),
        (
            &["privileged", "shell"],
            SystemsOperationsCapability::PrivilegedShell,
        ),
        (
            &["mutate", "vmware"],
            SystemsOperationsCapability::VmwareMutation,
        ),
        (
            &["mutate", "backup"],
            SystemsOperationsCapability::BackupMutation,
        ),
        (
            &["access", "credential"],
            SystemsOperationsCapability::CredentialAccess,
        ),
    ];
    validate_capability_mappings(values, mappings, capabilities)
}

fn validate_no_effect_requests(values: &[&str]) -> InfrastructureOperationsResult<()> {
    validate_cloud_capability_consistency(values, &[])?;
    validate_systems_capability_consistency(values, &[])
}

fn validate_capability_mappings<T: Copy + Eq>(
    values: &[&str],
    mappings: &[(&[&str], T)],
    capabilities: &[T],
) -> InfrastructureOperationsResult<()> {
    for value in values {
        let tokens = normalized_claim_tokens(value)
            .into_iter()
            .filter(|token| {
                !matches!(
                    token.as_str(),
                    "a" | "an"
                        | "the"
                        | "this"
                        | "that"
                        | "these"
                        | "those"
                        | "my"
                        | "your"
                        | "our"
                        | "their"
                        | "its"
                )
            })
            .collect::<Vec<_>>();
        for (phrase, capability) in mappings {
            if contains_effect_token_sequence(&tokens, phrase) && !capabilities.contains(capability)
            {
                return Err(InfrastructureOperationsError::CapabilityInconsistent);
            }
        }
    }
    Ok(())
}

fn contains_effect_token_sequence(tokens: &[String], phrase: &[&str]) -> bool {
    tokens
        .windows(phrase.len())
        .enumerate()
        .any(|(index, window)| {
            if !window.iter().map(String::as_str).eq(phrase.iter().copied()) {
                return false;
            }
            let prior_negated = index.checked_sub(1).is_some_and(|prior| {
                matches!(tokens[prior].as_str(), "no" | "not" | "never" | "without")
            });
            !prior_negated
        })
}

fn validate_text_list(values: &[String]) -> InfrastructureOperationsResult<()> {
    ensure_count(values.len(), 0, MAX_INFRASTRUCTURE_OPERATIONS_ITEMS)?;
    values
        .iter()
        .try_for_each(|value| validate_model_text(value))
}

fn bounded_transfer(raw: &str) -> InfrastructureOperationsResult<String> {
    if raw.len() > MAX_INFRASTRUCTURE_OPERATIONS_TRANSFER_BYTES {
        Err(InfrastructureOperationsError::BoundExceeded)
    } else {
        Ok(raw.to_owned())
    }
}

fn assessment_references(
    fixture_ids: &[InfrastructureOperationsId],
    finding_ids: impl Iterator<Item = InfrastructureOperationsId>,
    validation_ids: impl Iterator<Item = InfrastructureOperationsId>,
) -> Vec<InfrastructureOperationsEvidenceRef> {
    fixture_ids
        .iter()
        .cloned()
        .map(|id| InfrastructureOperationsEvidenceRef {
            namespace: InfrastructureOperationsEvidenceNamespace::Fixture,
            id,
        })
        .chain(finding_ids.map(|id| InfrastructureOperationsEvidenceRef {
            namespace: InfrastructureOperationsEvidenceNamespace::AssessmentFinding,
            id,
        }))
        .chain(
            validation_ids.map(|id| InfrastructureOperationsEvidenceRef {
                namespace: InfrastructureOperationsEvidenceNamespace::ProposedValidation,
                id,
            }),
        )
        .collect()
}

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
