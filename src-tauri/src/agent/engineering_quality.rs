//! Strict contracts for the sealed fixture-only engineering review workflow.
//!
//! All fixture and runtime content is untrusted proposal data. This module
//! validates bounded structured results and never grants tool, approval, or
//! execution authority.

use std::{collections::BTreeSet, fmt};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

use super::{
    definition::AgentId,
    governance::AgentPolicyProfileId,
    runtime::{RuntimeId, RuntimeRunIdentity},
    task::{AgentExecutionContext, AgentTaskFailureCode, AgentTaskId, ParentTaskId, RootTaskId},
};
use crate::memory::AgentMemoryProfileId;

pub const MAX_ENGINEERING_FILES: usize = 8;
pub const MAX_ENGINEERING_CRITERIA: usize = 8;
pub const MAX_ENGINEERING_EVIDENCE: usize = 8;
pub const MAX_ENGINEERING_ITEMS: usize = 8;
pub const MAX_ENGINEERING_REFERENCES: usize = 8;
pub const MAX_ENGINEERING_ID_BYTES: usize = 64;
pub const MAX_ENGINEERING_PATH_CHARACTERS: usize = 256;
pub const MAX_ENGINEERING_PATH_BYTES: usize = 1_024;
pub const MAX_ENGINEERING_FILE_CHARACTERS: usize = 2_048;
pub const MAX_ENGINEERING_FILE_BYTES: usize = 4_096;
pub const MAX_ENGINEERING_FILE_TOTAL_BYTES: usize = 6_144;
pub const MAX_ENGINEERING_FIXTURE_CATALOG_BYTES: usize = 8_192;
pub const MAX_ENGINEERING_CRITERIA_CATALOG_BYTES: usize = 4_096;
pub const MAX_ENGINEERING_CRITERIA_TEXT_TOTAL_BYTES: usize = 2_048;
pub const MAX_ENGINEERING_EVIDENCE_CATALOG_BYTES: usize = 4_096;
pub const MAX_ENGINEERING_EVIDENCE_DESCRIPTION_TOTAL_BYTES: usize = 3_072;
pub const MAX_ENGINEERING_OBJECTIVE_CHARACTERS: usize = 1_024;
pub const MAX_ENGINEERING_OBJECTIVE_BYTES: usize = 2_048;
pub const MAX_ENGINEERING_STAGE_INPUT_BYTES: usize = 24_576;
pub const MAX_CHANGE_PROPOSAL_CHARACTERS: usize = 4_096;
pub const MAX_CHANGE_PROPOSAL_BYTES: usize = 6_144;
pub const MAX_VALIDATION_REPORT_CHARACTERS: usize = 3_072;
pub const MAX_VALIDATION_REPORT_BYTES: usize = 4_096;
pub const MAX_RISK_ASSESSMENT_CHARACTERS: usize = 3_072;
pub const MAX_RISK_ASSESSMENT_BYTES: usize = 4_096;
pub const MAX_ENGINEERING_SYNTHESIS_CHARACTERS: usize = 2_048;
pub const MAX_ENGINEERING_SYNTHESIS_BYTES: usize = 4_096;
pub const MAX_ENGINEERING_WORKFLOW_EVENTS: usize = 16;
pub const MAX_ENGINEERING_AUDIT_RECORDS: usize = 16;

const MAX_TEXT_CHARACTERS: usize = 512;
const MAX_TEXT_BYTES: usize = 2_048;
const MAX_SHORT_TEXT_BYTES: usize = 1_024;

pub type EngineeringQualityResult<T> = Result<T, EngineeringQualityError>;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EngineeringId(String);

impl EngineeringId {
    pub fn new(value: impl Into<String>) -> EngineeringQualityResult<Self> {
        let value = value.into();
        if value.is_empty()
            || value.len() > MAX_ENGINEERING_ID_BYTES
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(EngineeringQualityError::InvalidId);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for EngineeringId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("EngineeringId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct FixtureRepositoryFile {
    id: EngineeringId,
    display_path: String,
    content: String,
}

impl FixtureRepositoryFile {
    pub fn new(
        id: impl Into<String>,
        display_path: impl Into<String>,
        content: impl Into<String>,
    ) -> EngineeringQualityResult<Self> {
        let display_path = display_path.into();
        let content = content.into();
        validate_fixture_path(&display_path)?;
        validate_text(
            &content,
            MAX_ENGINEERING_FILE_CHARACTERS,
            MAX_ENGINEERING_FILE_BYTES,
            true,
        )?;
        Ok(Self {
            id: EngineeringId::new(id)?,
            display_path,
            content,
        })
    }

    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub fn display_path(&self) -> &str {
        &self.display_path
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl fmt::Debug for FixtureRepositoryFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureRepositoryFile")
            .field("id", &self.id)
            .field("display_path", &"[REDACTED]")
            .field("content", &"[REDACTED]")
            .field("content_bytes", &self.content.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct FixtureRepositoryCatalog {
    files: Vec<FixtureRepositoryFile>,
    serialized: String,
}

impl FixtureRepositoryCatalog {
    pub fn new(files: Vec<FixtureRepositoryFile>) -> EngineeringQualityResult<Self> {
        ensure_count(files.len(), 1, MAX_ENGINEERING_FILES)?;
        let mut ids = BTreeSet::new();
        let mut paths = BTreeSet::new();
        let mut content_bytes = 0usize;
        for file in &files {
            if !ids.insert(file.id.clone()) || !paths.insert(file.display_path.clone()) {
                return Err(EngineeringQualityError::DuplicateReference);
            }
            content_bytes = content_bytes
                .checked_add(file.content.len())
                .ok_or(EngineeringQualityError::BoundExceeded)?;
        }
        if content_bytes > MAX_ENGINEERING_FILE_TOTAL_BYTES {
            return Err(EngineeringQualityError::BoundExceeded);
        }
        let wire: Vec<FixtureFileWire<'_>> = files
            .iter()
            .map(|file| FixtureFileWire {
                id: file.id.as_str(),
                path: &file.display_path,
                content: &file.content,
            })
            .collect();
        let serialized = serde_json::to_string(&wire)
            .map_err(|_| EngineeringQualityError::SerializationFailed)?;
        if serialized.len() > MAX_ENGINEERING_FIXTURE_CATALOG_BYTES {
            return Err(EngineeringQualityError::BoundExceeded);
        }
        Ok(Self { files, serialized })
    }

    #[must_use]
    pub fn files(&self) -> &[FixtureRepositoryFile] {
        &self.files
    }

    #[must_use]
    pub fn contains(&self, id: &EngineeringId) -> bool {
        self.files.iter().any(|file| file.id == *id)
    }
}

impl fmt::Debug for FixtureRepositoryCatalog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureRepositoryCatalog")
            .field("file_count", &self.files.len())
            .field("serialized", &"[REDACTED]")
            .field("serialized_bytes", &self.serialized.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AcceptanceCriterion {
    id: EngineeringId,
    text: String,
}

impl AcceptanceCriterion {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> EngineeringQualityResult<Self> {
        let text = text.into();
        validate_text(&text, MAX_TEXT_CHARACTERS, MAX_SHORT_TEXT_BYTES, false)?;
        Ok(Self {
            id: EngineeringId::new(id)?,
            text,
        })
    }

    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Debug for AcceptanceCriterion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcceptanceCriterion")
            .field("id", &self.id)
            .field("text", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationEvidenceKind {
    FixtureObservation,
    DependencyObservation,
    ProposedCheck,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationEvidenceStatus {
    ObservedFixture,
    NotRun,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ApplicationValidationEvidence {
    id: EngineeringId,
    kind: ApplicationEvidenceKind,
    status: ApplicationEvidenceStatus,
    description: String,
    criterion_ids: Vec<EngineeringId>,
    file_ids: Vec<EngineeringId>,
}

impl ApplicationValidationEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        kind: ApplicationEvidenceKind,
        status: ApplicationEvidenceStatus,
        description: impl Into<String>,
        criterion_ids: Vec<EngineeringId>,
        file_ids: Vec<EngineeringId>,
    ) -> EngineeringQualityResult<Self> {
        let description = description.into();
        validate_text(&description, MAX_TEXT_CHARACTERS, MAX_TEXT_BYTES, false)?;
        ensure_unique(&criterion_ids, 1, MAX_ENGINEERING_REFERENCES)?;
        ensure_unique(&file_ids, 0, MAX_ENGINEERING_REFERENCES)?;
        match (kind, status) {
            (
                ApplicationEvidenceKind::FixtureObservation
                | ApplicationEvidenceKind::DependencyObservation,
                ApplicationEvidenceStatus::ObservedFixture,
            ) if !file_ids.is_empty() => {}
            (ApplicationEvidenceKind::ProposedCheck, ApplicationEvidenceStatus::NotRun) => {}
            _ => return Err(EngineeringQualityError::EvidenceInconsistent),
        }
        Ok(Self {
            id: EngineeringId::new(id)?,
            kind,
            status,
            description,
            criterion_ids,
            file_ids,
        })
    }

    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub const fn kind(&self) -> ApplicationEvidenceKind {
        self.kind
    }

    #[must_use]
    pub const fn status(&self) -> ApplicationEvidenceStatus {
        self.status
    }

    #[must_use]
    pub fn criterion_ids(&self) -> &[EngineeringId] {
        &self.criterion_ids
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn file_ids(&self) -> &[EngineeringId] {
        &self.file_ids
    }
}

impl fmt::Debug for ApplicationValidationEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApplicationValidationEvidence")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("status", &self.status)
            .field("description", &"[REDACTED]")
            .field("criterion_count", &self.criterion_ids.len())
            .field("file_count", &self.file_ids.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ApplicationValidationEvidenceCatalog {
    entries: Vec<ApplicationValidationEvidence>,
    serialized: String,
}

impl ApplicationValidationEvidenceCatalog {
    pub fn new(entries: Vec<ApplicationValidationEvidence>) -> EngineeringQualityResult<Self> {
        ensure_count(entries.len(), 0, MAX_ENGINEERING_EVIDENCE)?;
        let description_bytes = entries.iter().try_fold(0_usize, |total, entry| {
            total
                .checked_add(entry.description.len())
                .ok_or(EngineeringQualityError::BoundExceeded)
        })?;
        if description_bytes > MAX_ENGINEERING_EVIDENCE_DESCRIPTION_TOTAL_BYTES {
            return Err(EngineeringQualityError::BoundExceeded);
        }
        let mut ids = BTreeSet::new();
        for entry in &entries {
            if !ids.insert(entry.id.clone()) {
                return Err(EngineeringQualityError::DuplicateReference);
            }
        }
        let serialized = serde_json::to_string(
            &entries
                .iter()
                .map(ApplicationEvidenceWire::from)
                .collect::<Vec<_>>(),
        )
        .map_err(|_| EngineeringQualityError::SerializationFailed)?;
        if serialized.len() > MAX_ENGINEERING_EVIDENCE_CATALOG_BYTES {
            return Err(EngineeringQualityError::BoundExceeded);
        }
        Ok(Self {
            entries,
            serialized,
        })
    }

    #[must_use]
    pub fn entries(&self) -> &[ApplicationValidationEvidence] {
        &self.entries
    }

    fn get(&self, id: &EngineeringId) -> Option<&ApplicationValidationEvidence> {
        self.entries.iter().find(|entry| entry.id == *id)
    }
}

impl fmt::Debug for ApplicationValidationEvidenceCatalog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApplicationValidationEvidenceCatalog")
            .field("entry_count", &self.entries.len())
            .field("serialized", &"[REDACTED]")
            .field("serialized_bytes", &self.serialized.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringEvidenceNamespace {
    FixtureFile,
    ApplicationEvidence,
    ProposalFinding,
    ProposalValidationStep,
    QaFinding,
    QaCheck,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EngineeringEvidenceRef {
    namespace: EngineeringEvidenceNamespace,
    id: EngineeringId,
}

impl EngineeringEvidenceRef {
    pub fn new(
        namespace: EngineeringEvidenceNamespace,
        id: impl Into<String>,
    ) -> EngineeringQualityResult<Self> {
        Ok(Self {
            namespace,
            id: EngineeringId::new(id)?,
        })
    }

    #[must_use]
    pub const fn namespace(&self) -> EngineeringEvidenceNamespace {
        self.namespace
    }

    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }
}

impl fmt::Debug for EngineeringEvidenceRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EngineeringEvidenceRef")
            .field("namespace", &self.namespace)
            .field("id", &self.id)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringCapability {
    FixtureInspect,
    FixtureTextSearch,
    ArchitectureExplain,
    DiffReview,
    ImplementationPlan,
    PatchProposal,
    ValidationPlan,
    FormattingPlan,
    FileWrite,
    FileDelete,
    OutsideFixturePath,
    DependencyInstall,
    PackageManagerExecute,
    TestExecute,
    FormatterExecute,
    GitCommit,
    GitPush,
    BranchDelete,
    DestructiveShell,
    CredentialAccess,
    NetworkAccess,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringCapabilityDisposition {
    ProposalOnly,
    Denied,
}

impl EngineeringCapability {
    #[must_use]
    pub const fn disposition(self) -> EngineeringCapabilityDisposition {
        match self {
            Self::FixtureInspect
            | Self::FixtureTextSearch
            | Self::ArchitectureExplain
            | Self::DiffReview
            | Self::ImplementationPlan
            | Self::PatchProposal
            | Self::ValidationPlan
            | Self::FormattingPlan => EngineeringCapabilityDisposition::ProposalOnly,
            Self::FileWrite
            | Self::FileDelete
            | Self::OutsideFixturePath
            | Self::DependencyInstall
            | Self::PackageManagerExecute
            | Self::TestExecute
            | Self::FormatterExecute
            | Self::GitCommit
            | Self::GitPush
            | Self::BranchDelete
            | Self::DestructiveShell
            | Self::CredentialAccess
            | Self::NetworkAccess => EngineeringCapabilityDisposition::Denied,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Confidence {
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeProposalQuality {
    Complete,
    PartialDeniedCapability,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ChangeProposal {
    task_id: AgentTaskId,
    objective_summary: String,
    findings: Vec<ProposalFinding>,
    affected_file_ids: Vec<EngineeringId>,
    patch_operations: Vec<PatchOperation>,
    risks: Vec<String>,
    validation_steps: Vec<ProposalValidationStep>,
    rollback: String,
    capabilities: Vec<EngineeringCapability>,
    quality: ChangeProposalQuality,
    transfer: String,
}

impl ChangeProposal {
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub fn objective_summary(&self) -> &str {
        &self.objective_summary
    }

    #[must_use]
    pub fn findings(&self) -> &[ProposalFinding] {
        &self.findings
    }

    #[must_use]
    pub const fn quality(&self) -> ChangeProposalQuality {
        self.quality
    }

    #[must_use]
    pub fn affected_file_ids(&self) -> &[EngineeringId] {
        &self.affected_file_ids
    }

    #[must_use]
    pub fn patch_operations(&self) -> &[PatchOperation] {
        &self.patch_operations
    }

    #[must_use]
    pub fn risks(&self) -> &[String] {
        &self.risks
    }

    #[must_use]
    pub fn validation_steps(&self) -> &[ProposalValidationStep] {
        &self.validation_steps
    }

    #[must_use]
    pub fn rollback(&self) -> &str {
        &self.rollback
    }

    #[must_use]
    pub fn capabilities(&self) -> &[EngineeringCapability] {
        &self.capabilities
    }

    #[must_use]
    pub fn transfer(&self) -> &str {
        &self.transfer
    }
}

impl fmt::Debug for ChangeProposal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ChangeProposal")
            .field("task_id", &self.task_id)
            .field("quality", &self.quality)
            .field("finding_count", &self.findings.len())
            .field("affected_file_count", &self.affected_file_ids.len())
            .field("patch_count", &self.patch_operations.len())
            .field("risk_count", &self.risks.len())
            .field("validation_step_count", &self.validation_steps.len())
            .field("capability_count", &self.capabilities.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ProposalFinding {
    id: EngineeringId,
    statement: String,
    references: Vec<EngineeringEvidenceRef>,
    confidence: Confidence,
}

impl ProposalFinding {
    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }

    #[must_use]
    pub fn references(&self) -> &[EngineeringEvidenceRef] {
        &self.references
    }

    #[must_use]
    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PatchOperation {
    file_id: EngineeringId,
    proposal: String,
}

impl PatchOperation {
    #[must_use]
    pub fn file_id(&self) -> &EngineeringId {
        &self.file_id
    }

    #[must_use]
    pub fn proposal(&self) -> &str {
        &self.proposal
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ProposalValidationStep {
    id: EngineeringId,
    text: String,
}

impl ProposalValidationStep {
    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CriterionDisposition {
    Demonstrated,
    NotDemonstrated,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValidationConclusion {
    Adequate,
    Incomplete,
    Blocked,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ValidationReport {
    task_id: AgentTaskId,
    coverage: Vec<CriterionCoverage>,
    findings: Vec<QaFinding>,
    proposed_checks: Vec<QaCheck>,
    gaps: Vec<String>,
    conclusion: ValidationConclusion,
    transfer: String,
}

impl ValidationReport {
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub const fn conclusion(&self) -> ValidationConclusion {
        self.conclusion
    }

    #[must_use]
    pub fn coverage(&self) -> &[CriterionCoverage] {
        &self.coverage
    }

    #[must_use]
    pub fn findings(&self) -> &[QaFinding] {
        &self.findings
    }

    #[must_use]
    pub fn proposed_checks(&self) -> &[QaCheck] {
        &self.proposed_checks
    }

    #[must_use]
    pub fn gaps(&self) -> &[String] {
        &self.gaps
    }

    #[must_use]
    pub fn transfer(&self) -> &str {
        &self.transfer
    }
}

impl fmt::Debug for ValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidationReport")
            .field("task_id", &self.task_id)
            .field("conclusion", &self.conclusion)
            .field("coverage_count", &self.coverage.len())
            .field("finding_count", &self.findings.len())
            .field("check_count", &self.proposed_checks.len())
            .field("gap_count", &self.gaps.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct CriterionCoverage {
    criterion_id: EngineeringId,
    disposition: CriterionDisposition,
    references: Vec<EngineeringEvidenceRef>,
}

impl CriterionCoverage {
    #[must_use]
    pub fn criterion_id(&self) -> &EngineeringId {
        &self.criterion_id
    }

    #[must_use]
    pub const fn disposition(&self) -> CriterionDisposition {
        self.disposition
    }

    #[must_use]
    pub fn references(&self) -> &[EngineeringEvidenceRef] {
        &self.references
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct QaFinding {
    id: EngineeringId,
    statement: String,
    references: Vec<EngineeringEvidenceRef>,
}

impl QaFinding {
    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }

    #[must_use]
    pub fn references(&self) -> &[EngineeringEvidenceRef] {
        &self.references
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct QaCheck {
    id: EngineeringId,
    text: String,
    references: Vec<EngineeringEvidenceRef>,
}

impl QaCheck {
    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn references(&self) -> &[EngineeringEvidenceRef] {
        &self.references
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RiskCategory {
    AuthorizationBoundary,
    InputValidation,
    SecretsPrivacy,
    DependencyEvidence,
    Audit,
    Rollback,
    GeneralChangeRisk,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RiskSeverity {
    Informational,
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RiskFindingBasis {
    EvidenceBound(Vec<EngineeringEvidenceRef>),
    Hypothesis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DependencyEvidenceStatus {
    Available(Vec<EngineeringEvidenceRef>),
    Unavailable,
}

#[derive(Clone, Eq, PartialEq)]
pub struct RiskAssessment {
    task_id: AgentTaskId,
    findings: Vec<RiskFinding>,
    unresolved_risks: Vec<String>,
    follow_ups: Vec<String>,
    dependency_evidence: DependencyEvidenceStatus,
    transfer: String,
}

impl RiskAssessment {
    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub fn findings(&self) -> &[RiskFinding] {
        &self.findings
    }

    #[must_use]
    pub fn dependency_evidence(&self) -> &DependencyEvidenceStatus {
        &self.dependency_evidence
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
    pub fn transfer(&self) -> &str {
        &self.transfer
    }
}

impl fmt::Debug for RiskAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RiskAssessment")
            .field("task_id", &self.task_id)
            .field("finding_count", &self.findings.len())
            .field("unresolved_count", &self.unresolved_risks.len())
            .field("follow_up_count", &self.follow_ups.len())
            .field("dependency_evidence", &"[REDACTED]")
            .field("content", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RiskFinding {
    id: EngineeringId,
    category: RiskCategory,
    severity: RiskSeverity,
    confidence: Confidence,
    statement: String,
    basis: RiskFindingBasis,
}

impl RiskFinding {
    #[must_use]
    pub fn id(&self) -> &EngineeringId {
        &self.id
    }

    #[must_use]
    pub const fn category(&self) -> RiskCategory {
        self.category
    }

    #[must_use]
    pub const fn severity(&self) -> RiskSeverity {
        self.severity
    }

    #[must_use]
    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }

    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }

    #[must_use]
    pub fn basis(&self) -> &RiskFindingBasis {
        &self.basis
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringReviewStatus {
    Complete,
    Partial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringApprovalRequirement {
    NotApplicable,
    RequiredBeforeMutation,
}

#[derive(Clone, Eq, PartialEq)]
pub struct EngineeringReviewSynthesis {
    summary: String,
    affected_file_ids: Vec<EngineeringId>,
    unresolved_issues: Vec<String>,
    approval_requirement: EngineeringApprovalRequirement,
    status: EngineeringReviewStatus,
}

impl EngineeringReviewSynthesis {
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    #[must_use]
    pub const fn approval_requirement(&self) -> EngineeringApprovalRequirement {
        self.approval_requirement
    }

    #[must_use]
    pub const fn status(&self) -> EngineeringReviewStatus {
        self.status
    }

    #[must_use]
    pub fn affected_file_ids(&self) -> &[EngineeringId] {
        &self.affected_file_ids
    }

    #[must_use]
    pub fn unresolved_issues(&self) -> &[String] {
        &self.unresolved_issues
    }
}

impl fmt::Debug for EngineeringReviewSynthesis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EngineeringReviewSynthesis")
            .field("summary", &"[REDACTED]")
            .field("affected_file_count", &self.affected_file_ids.len())
            .field("unresolved_count", &self.unresolved_issues.len())
            .field("approval_requirement", &self.approval_requirement)
            .field("status", &self.status)
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CodingStageOutcome {
    Completed(Box<ChangeProposal>),
    Failed(AgentTaskFailureCode),
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QaStageOutcome {
    Completed(ValidationReport),
    Failed(AgentTaskFailureCode),
    Cancelled,
    SkippedCodingUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityStageOutcome {
    Completed(RiskAssessment),
    Failed(AgentTaskFailureCode),
    Cancelled,
    SkippedCodingUnavailable,
}

#[derive(Clone, Eq, PartialEq)]
pub struct EngineeringQualityWorkflowResult {
    root_task_id: RootTaskId,
    coding: CodingStageOutcome,
    qa: QaStageOutcome,
    security: SecurityStageOutcome,
    synthesis: EngineeringReviewSynthesis,
}

impl EngineeringQualityWorkflowResult {
    pub(super) fn new(
        root_task_id: RootTaskId,
        coding: CodingStageOutcome,
        qa: QaStageOutcome,
        security: SecurityStageOutcome,
        synthesis: EngineeringReviewSynthesis,
    ) -> Self {
        Self {
            root_task_id,
            coding,
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
    pub fn coding(&self) -> &CodingStageOutcome {
        &self.coding
    }

    #[must_use]
    pub fn qa(&self) -> &QaStageOutcome {
        &self.qa
    }

    #[must_use]
    pub fn security(&self) -> &SecurityStageOutcome {
        &self.security
    }

    #[must_use]
    pub fn synthesis(&self) -> &EngineeringReviewSynthesis {
        &self.synthesis
    }
}

impl fmt::Debug for EngineeringQualityWorkflowResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EngineeringQualityWorkflowResult")
            .field("root_task_id", &self.root_task_id)
            .field("coding", &stage_label(&self.coding))
            .field("qa", &stage_label(&self.qa))
            .field("security", &stage_label(&self.security))
            .field("synthesis", &self.synthesis)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct EngineeringQualityWorkflowRequest {
    objective: String,
    fixtures: FixtureRepositoryCatalog,
    criteria: Vec<AcceptanceCriterion>,
    evidence: ApplicationValidationEvidenceCatalog,
    criteria_serialized: String,
}

impl EngineeringQualityWorkflowRequest {
    pub fn new(
        objective: impl Into<String>,
        fixtures: FixtureRepositoryCatalog,
        criteria: Vec<AcceptanceCriterion>,
        evidence: ApplicationValidationEvidenceCatalog,
    ) -> EngineeringQualityResult<Self> {
        let objective = objective.into();
        validate_text(
            &objective,
            MAX_ENGINEERING_OBJECTIVE_CHARACTERS,
            MAX_ENGINEERING_OBJECTIVE_BYTES,
            false,
        )?;
        ensure_count(criteria.len(), 1, MAX_ENGINEERING_CRITERIA)?;
        let criteria_text_bytes = criteria.iter().try_fold(0_usize, |total, criterion| {
            total
                .checked_add(criterion.text.len())
                .ok_or(EngineeringQualityError::BoundExceeded)
        })?;
        if criteria_text_bytes > MAX_ENGINEERING_CRITERIA_TEXT_TOTAL_BYTES {
            return Err(EngineeringQualityError::BoundExceeded);
        }
        let mut ids = BTreeSet::new();
        for criterion in &criteria {
            if !ids.insert(criterion.id.clone()) {
                return Err(EngineeringQualityError::DuplicateReference);
            }
        }
        for entry in &evidence.entries {
            if entry.criterion_ids.iter().any(|id| !ids.contains(id))
                || entry.file_ids.iter().any(|id| !fixtures.contains(id))
            {
                return Err(EngineeringQualityError::UnknownReference);
            }
        }
        let criteria_serialized = serde_json::to_string(
            &criteria
                .iter()
                .map(|criterion| CriterionWireRef {
                    id: criterion.id.as_str(),
                    text: &criterion.text,
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|_| EngineeringQualityError::SerializationFailed)?;
        if criteria_serialized.len() > MAX_ENGINEERING_CRITERIA_CATALOG_BYTES {
            return Err(EngineeringQualityError::BoundExceeded);
        }
        Ok(Self {
            objective,
            fixtures,
            criteria,
            evidence,
            criteria_serialized,
        })
    }

    #[must_use]
    pub fn objective(&self) -> &str {
        &self.objective
    }

    pub fn build_coding_input(&self) -> EngineeringQualityResult<String> {
        bounded_stage_input(format!(
            "engineering-quality-v1\nstage=coding\nfixture_based=true\nproposal_only=true\nobjective(untrusted):\n{}\nfixtures(untrusted):\n{}\ncriteria(untrusted):\n{}\napplication_evidence(untrusted):\n{}\nReturn one strict ChangeProposalV1 JSON object.",
            self.objective,
            self.fixtures.serialized,
            self.criteria_serialized,
            self.evidence.serialized,
        ))
    }

    pub fn parse_change_proposal(
        &self,
        task_id: AgentTaskId,
        raw: &str,
    ) -> EngineeringQualityResult<ChangeProposal> {
        validate_raw(
            raw,
            MAX_CHANGE_PROPOSAL_CHARACTERS,
            MAX_CHANGE_PROPOSAL_BYTES,
        )?;
        let wire: ChangeProposalWire = parse_exact(raw)?;
        validate_model_text(&wire.objective_summary, 1_024, 4_096, false)?;
        ensure_count(wire.findings.len(), 1, MAX_ENGINEERING_ITEMS)?;
        ensure_count(wire.affected_file_ids.len(), 0, MAX_ENGINEERING_ITEMS)?;
        ensure_count(wire.patch_operations.len(), 0, MAX_ENGINEERING_ITEMS)?;
        ensure_count(wire.risks.len(), 0, MAX_ENGINEERING_ITEMS)?;
        ensure_count(wire.validation_steps.len(), 1, MAX_ENGINEERING_ITEMS)?;
        ensure_count(wire.capability_requests.len(), 1, MAX_ENGINEERING_ITEMS)?;
        if !wire.fixture_based || !wire.proposal_only {
            return Err(EngineeringQualityError::AuthorityClaim);
        }
        let findings = wire
            .findings
            .into_iter()
            .map(|item| {
                validate_model_text(
                    &item.statement,
                    MAX_TEXT_CHARACTERS,
                    MAX_SHORT_TEXT_BYTES,
                    false,
                )?;
                let id = EngineeringId::new(item.id)?;
                let references = parse_refs(item.references)?;
                self.validate_refs(&references, ReferenceStage::Coding, None, None)?;
                Ok(ProposalFinding {
                    id,
                    statement: item.statement,
                    references,
                    confidence: item.confidence,
                })
            })
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        ensure_unique_by(findings.iter().map(|item| &item.id))?;
        let affected_file_ids = parse_ids(wire.affected_file_ids, 0, MAX_ENGINEERING_ITEMS)?;
        self.validate_file_ids(&affected_file_ids)?;
        let patch_operations = wire
            .patch_operations
            .into_iter()
            .map(|item| {
                validate_model_text(&item.proposal, MAX_TEXT_CHARACTERS, MAX_TEXT_BYTES, true)?;
                let file_id = EngineeringId::new(item.file_id)?;
                if !self.fixtures.contains(&file_id) {
                    return Err(EngineeringQualityError::UnknownReference);
                }
                Ok(PatchOperation {
                    file_id,
                    proposal: item.proposal,
                })
            })
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        ensure_unique_by(patch_operations.iter().map(|item| &item.file_id))?;
        if patch_operations
            .iter()
            .any(|operation| !affected_file_ids.contains(&operation.file_id))
        {
            return Err(EngineeringQualityError::UnknownReference);
        }
        validate_texts(&wire.risks, MAX_SHORT_TEXT_BYTES)?;
        let validation_steps = wire
            .validation_steps
            .into_iter()
            .map(|item| {
                validate_model_text(&item.text, MAX_TEXT_CHARACTERS, MAX_TEXT_BYTES, false)?;
                Ok(ProposalValidationStep {
                    id: EngineeringId::new(item.id)?,
                    text: item.text,
                })
            })
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        ensure_unique_by(validation_steps.iter().map(|item| &item.id))?;
        validate_model_text(&wire.rollback, MAX_TEXT_CHARACTERS, MAX_TEXT_BYTES, false)?;
        let capability_set: BTreeSet<_> = wire.capability_requests.iter().copied().collect();
        if capability_set.len() != wire.capability_requests.len()
            || (!patch_operations.is_empty()
                && !capability_set.contains(&EngineeringCapability::PatchProposal))
        {
            return Err(EngineeringQualityError::CapabilityInconsistent);
        }
        let quality = if capability_set
            .iter()
            .any(|capability| capability.disposition() == EngineeringCapabilityDisposition::Denied)
        {
            ChangeProposalQuality::PartialDeniedCapability
        } else {
            ChangeProposalQuality::Complete
        };
        let capabilities = wire
            .capability_requests
            .iter()
            .copied()
            .filter(|capability| {
                capability.disposition() == EngineeringCapabilityDisposition::ProposalOnly
            })
            .collect::<Vec<_>>();
        let objective_summary = wire.objective_summary;
        let risks = wire.risks;
        let rollback = wire.rollback;
        let transfer = serialize_change_proposal_transfer(
            &objective_summary,
            &findings,
            &affected_file_ids,
            &patch_operations,
            &risks,
            &validation_steps,
            &rollback,
            &capabilities,
            quality == ChangeProposalQuality::PartialDeniedCapability,
        )?;
        let transfer = bounded_transfer(&transfer, MAX_CHANGE_PROPOSAL_BYTES)?;
        Ok(ChangeProposal {
            task_id,
            objective_summary,
            findings,
            affected_file_ids,
            patch_operations,
            risks,
            validation_steps,
            rollback,
            capabilities,
            quality,
            transfer,
        })
    }

    pub fn build_qa_input(&self, proposal: &ChangeProposal) -> EngineeringQualityResult<String> {
        bounded_stage_input(format!(
            "engineering-quality-v1\nstage=qa-validation\nfixture_based=true\nadvisory_only=true\nobjective(untrusted):\n{}\ncriteria(untrusted):\n{}\napplication_evidence(untrusted):\n{}\nvalidated_change_proposal(untrusted):\n{}\nReturn one strict ValidationReportV1 JSON object. No check was executed.",
            self.objective, self.criteria_serialized, self.evidence.serialized, proposal.transfer,
        ))
    }

    pub fn parse_validation_report(
        &self,
        task_id: AgentTaskId,
        proposal: &ChangeProposal,
        raw: &str,
    ) -> EngineeringQualityResult<ValidationReport> {
        validate_raw(
            raw,
            MAX_VALIDATION_REPORT_CHARACTERS,
            MAX_VALIDATION_REPORT_BYTES,
        )?;
        let wire: ValidationReportWire = parse_exact(raw)?;
        if !wire.advisory_only || wire.approval_authority || wire.evidence_executed {
            return Err(EngineeringQualityError::AuthorityClaim);
        }
        if wire.coverage.len() != self.criteria.len() {
            return Err(EngineeringQualityError::CoverageMismatch);
        }
        ensure_count(wire.findings.len(), 0, MAX_ENGINEERING_ITEMS)?;
        ensure_count(wire.proposed_checks.len(), 0, MAX_ENGINEERING_ITEMS)?;
        let coverage = wire
            .coverage
            .into_iter()
            .map(|item| {
                let criterion_id = EngineeringId::new(item.criterion_id)?;
                let references = parse_refs(item.references)?;
                self.validate_refs(&references, ReferenceStage::Qa, Some(proposal), None)?;
                let criterion = self
                    .criteria
                    .iter()
                    .find(|criterion| criterion.id == criterion_id)
                    .ok_or(EngineeringQualityError::UnknownReference)?;
                if references.iter().any(|reference| {
                    reference.namespace == EngineeringEvidenceNamespace::ApplicationEvidence
                        && self
                            .evidence
                            .get(&reference.id)
                            .is_none_or(|entry| !entry.criterion_ids.contains(&criterion.id))
                }) {
                    return Err(EngineeringQualityError::EvidenceInconsistent);
                }
                let observed = references.iter().any(|reference| {
                    reference.namespace == EngineeringEvidenceNamespace::ApplicationEvidence
                        && self.evidence.get(&reference.id).is_some_and(|entry| {
                            entry.status == ApplicationEvidenceStatus::ObservedFixture
                                && entry.criterion_ids.contains(&criterion.id)
                        })
                });
                if item.disposition == CriterionDisposition::Demonstrated && !observed {
                    return Err(EngineeringQualityError::EvidenceInconsistent);
                }
                Ok(CriterionCoverage {
                    criterion_id,
                    disposition: item.disposition,
                    references,
                })
            })
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        ensure_unique_by(coverage.iter().map(|item| &item.criterion_id))?;
        let findings = wire
            .findings
            .into_iter()
            .map(|item| {
                validate_model_text(
                    &item.statement,
                    MAX_TEXT_CHARACTERS,
                    MAX_SHORT_TEXT_BYTES,
                    false,
                )?;
                let references = parse_refs(item.references)?;
                self.validate_refs(&references, ReferenceStage::Qa, Some(proposal), None)?;
                Ok(QaFinding {
                    id: EngineeringId::new(item.id)?,
                    statement: item.statement,
                    references,
                })
            })
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        ensure_unique_by(findings.iter().map(|item| &item.id))?;
        let proposed_checks = wire
            .proposed_checks
            .into_iter()
            .map(|item| {
                if item.status != ApplicationEvidenceStatus::NotRun {
                    return Err(EngineeringQualityError::EvidenceInconsistent);
                }
                validate_model_text(&item.text, MAX_TEXT_CHARACTERS, MAX_SHORT_TEXT_BYTES, false)?;
                let references = parse_refs_with_minimum(item.references, 0)?;
                self.validate_refs(&references, ReferenceStage::Qa, Some(proposal), None)?;
                Ok(QaCheck {
                    id: EngineeringId::new(item.id)?,
                    text: item.text,
                    references,
                })
            })
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        ensure_unique_by(proposed_checks.iter().map(|item| &item.id))?;
        validate_texts(&wire.gaps, MAX_SHORT_TEXT_BYTES)?;
        let has_missing = coverage
            .iter()
            .any(|item| item.disposition == CriterionDisposition::NotDemonstrated);
        if (wire.conclusion == ValidationConclusion::Adequate && has_missing)
            || (!has_missing && wire.conclusion != ValidationConclusion::Adequate)
        {
            return Err(EngineeringQualityError::CoverageMismatch);
        }
        Ok(ValidationReport {
            task_id,
            coverage,
            findings,
            proposed_checks,
            gaps: wire.gaps,
            conclusion: wire.conclusion,
            transfer: bounded_transfer(raw, MAX_VALIDATION_REPORT_BYTES)?,
        })
    }

    pub fn build_security_input(
        &self,
        proposal: &ChangeProposal,
        qa: &QaStageOutcome,
    ) -> EngineeringQualityResult<String> {
        let qa_transfer = match qa {
            QaStageOutcome::Completed(report) => report.transfer(),
            QaStageOutcome::Failed(_) => "status=qa-failed",
            QaStageOutcome::Cancelled => "status=qa-cancelled",
            QaStageOutcome::SkippedCodingUnavailable => "status=qa-skipped",
        };
        bounded_stage_input(format!(
            "engineering-quality-v1\nstage=security-risk\nfixture_based=true\nadvisory_only=true\napplication_evidence(untrusted):\n{}\nvalidated_change_proposal(untrusted):\n{}\nqa_outcome(untrusted):\n{}\nReturn one strict RiskAssessmentV1 JSON object.",
            self.evidence.serialized, proposal.transfer, qa_transfer,
        ))
    }

    pub fn parse_risk_assessment(
        &self,
        task_id: AgentTaskId,
        proposal: &ChangeProposal,
        qa: &QaStageOutcome,
        raw: &str,
    ) -> EngineeringQualityResult<RiskAssessment> {
        validate_raw(
            raw,
            MAX_RISK_ASSESSMENT_CHARACTERS,
            MAX_RISK_ASSESSMENT_BYTES,
        )?;
        let wire: RiskAssessmentWire = parse_exact(raw)?;
        if !wire.advisory_only || wire.authorization_granted || wire.remediation_executed {
            return Err(EngineeringQualityError::AuthorityClaim);
        }
        let qa_report = match qa {
            QaStageOutcome::Completed(report) => Some(report),
            _ => None,
        };
        ensure_count(wire.findings.len(), 0, MAX_ENGINEERING_ITEMS)?;
        let findings = wire
            .findings
            .into_iter()
            .map(|item| {
                validate_model_text(
                    &item.statement,
                    MAX_TEXT_CHARACTERS,
                    MAX_SHORT_TEXT_BYTES,
                    false,
                )?;
                let basis = match item.basis {
                    RiskFindingBasisWire::EvidenceBound => {
                        let references = parse_refs(item.references)?;
                        self.validate_refs(
                            &references,
                            ReferenceStage::Security,
                            Some(proposal),
                            qa_report,
                        )?;
                        RiskFindingBasis::EvidenceBound(references)
                    }
                    RiskFindingBasisWire::Hypothesis => {
                        if item.confidence == Confidence::High || !item.references.is_empty() {
                            return Err(EngineeringQualityError::EvidenceInconsistent);
                        }
                        RiskFindingBasis::Hypothesis
                    }
                };
                Ok(RiskFinding {
                    id: EngineeringId::new(item.id)?,
                    category: item.category,
                    severity: item.severity,
                    confidence: item.confidence,
                    statement: item.statement,
                    basis,
                })
            })
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        ensure_unique_by(findings.iter().map(|item| &item.id))?;
        validate_texts(&wire.unresolved_risks, MAX_SHORT_TEXT_BYTES)?;
        validate_texts(&wire.follow_ups, MAX_SHORT_TEXT_BYTES)?;
        let dependency_refs: Vec<_> = self
            .evidence
            .entries
            .iter()
            .filter(|entry| {
                entry.kind == ApplicationEvidenceKind::DependencyObservation
                    && entry.status == ApplicationEvidenceStatus::ObservedFixture
            })
            .map(|entry| EngineeringEvidenceRef {
                namespace: EngineeringEvidenceNamespace::ApplicationEvidence,
                id: entry.id.clone(),
            })
            .collect();
        let dependency_evidence = if dependency_refs.is_empty() {
            DependencyEvidenceStatus::Unavailable
        } else {
            DependencyEvidenceStatus::Available(dependency_refs)
        };
        Ok(RiskAssessment {
            task_id,
            findings,
            unresolved_risks: wire.unresolved_risks,
            follow_ups: wire.follow_ups,
            dependency_evidence,
            transfer: bounded_transfer(raw, MAX_RISK_ASSESSMENT_BYTES)?,
        })
    }

    pub fn build_synthesis_input(
        &self,
        coding: &CodingStageOutcome,
        qa: &QaStageOutcome,
        security: &SecurityStageOutcome,
    ) -> EngineeringQualityResult<String> {
        bounded_stage_input(format!(
            "engineering-quality-v1\nstage=personal-synthesis\nfixture_based=true\nproposal_only=true\nchanges_applied=false\ntests_executed=false\ncoding_outcome(untrusted):\n{}\nqa_outcome(untrusted):\n{}\nsecurity_outcome(untrusted):\n{}\nReturn one strict EngineeringReviewSynthesisV1 JSON object.",
            coding_transfer(coding), qa_transfer(qa), security_transfer(security),
        ))
    }

    pub fn parse_synthesis(
        &self,
        coding: &CodingStageOutcome,
        qa: &QaStageOutcome,
        security: &SecurityStageOutcome,
        raw: &str,
    ) -> EngineeringQualityResult<EngineeringReviewSynthesis> {
        validate_raw(
            raw,
            MAX_ENGINEERING_SYNTHESIS_CHARACTERS,
            MAX_ENGINEERING_SYNTHESIS_BYTES,
        )?;
        let wire: SynthesisWire = parse_exact(raw)?;
        if !wire.fixture_based || !wire.proposal_only || wire.changes_applied || wire.tests_executed
        {
            return Err(EngineeringQualityError::AuthorityClaim);
        }
        validate_model_text(&wire.summary, 1_024, MAX_TEXT_BYTES, false)?;
        validate_texts(&wire.unresolved_issues, MAX_SHORT_TEXT_BYTES)?;
        let affected_file_ids = parse_ids(wire.affected_file_ids, 0, MAX_ENGINEERING_ITEMS)?;
        self.validate_file_ids(&affected_file_ids)?;
        let (expected_files, has_patch, expected_status) = match coding {
            CodingStageOutcome::Completed(proposal) => {
                let partial = proposal.quality != ChangeProposalQuality::Complete
                    || !matches!(qa, QaStageOutcome::Completed(report) if report.conclusion == ValidationConclusion::Adequate)
                    || !matches!(security, SecurityStageOutcome::Completed(_));
                (
                    proposal.affected_file_ids.clone(),
                    !proposal.patch_operations.is_empty(),
                    if partial {
                        EngineeringReviewStatus::Partial
                    } else {
                        EngineeringReviewStatus::Complete
                    },
                )
            }
            CodingStageOutcome::Failed(_) | CodingStageOutcome::Cancelled => {
                (Vec::new(), false, EngineeringReviewStatus::Partial)
            }
        };
        if affected_file_ids != expected_files || wire.status != expected_status {
            return Err(EngineeringQualityError::SynthesisInconsistent);
        }
        let summary_lower = wire.summary.to_ascii_lowercase();
        if !contains_ascii_word(&summary_lower, "fixture")
            || !contains_ascii_word(&summary_lower, "proposal")
            || (expected_status == EngineeringReviewStatus::Partial
                && (!contains_ascii_word(&summary_lower, "partial")
                    || wire.unresolved_issues.is_empty()))
        {
            return Err(EngineeringQualityError::SynthesisInconsistent);
        }
        let approval_requirement = if has_patch {
            EngineeringApprovalRequirement::RequiredBeforeMutation
        } else {
            EngineeringApprovalRequirement::NotApplicable
        };
        Ok(EngineeringReviewSynthesis {
            summary: wire.summary,
            affected_file_ids,
            unresolved_issues: wire.unresolved_issues,
            approval_requirement,
            status: wire.status,
        })
    }

    fn validate_file_ids(&self, ids: &[EngineeringId]) -> EngineeringQualityResult<()> {
        if ids.iter().all(|id| self.fixtures.contains(id)) {
            Ok(())
        } else {
            Err(EngineeringQualityError::UnknownReference)
        }
    }

    fn validate_refs(
        &self,
        references: &[EngineeringEvidenceRef],
        stage: ReferenceStage,
        proposal: Option<&ChangeProposal>,
        qa: Option<&ValidationReport>,
    ) -> EngineeringQualityResult<()> {
        for reference in references {
            let valid = match reference.namespace {
                EngineeringEvidenceNamespace::FixtureFile => self.fixtures.contains(&reference.id),
                EngineeringEvidenceNamespace::ApplicationEvidence => {
                    self.evidence.get(&reference.id).is_some()
                }
                EngineeringEvidenceNamespace::ProposalFinding => proposal
                    .is_some_and(|value| value.findings.iter().any(|item| item.id == reference.id)),
                EngineeringEvidenceNamespace::ProposalValidationStep => {
                    proposal.is_some_and(|value| {
                        value
                            .validation_steps
                            .iter()
                            .any(|item| item.id == reference.id)
                    })
                }
                EngineeringEvidenceNamespace::QaFinding => qa
                    .is_some_and(|value| value.findings.iter().any(|item| item.id == reference.id)),
                EngineeringEvidenceNamespace::QaCheck => qa.is_some_and(|value| {
                    value
                        .proposed_checks
                        .iter()
                        .any(|item| item.id == reference.id)
                }),
            };
            let allowed = match stage {
                ReferenceStage::Coding => matches!(
                    reference.namespace,
                    EngineeringEvidenceNamespace::FixtureFile
                        | EngineeringEvidenceNamespace::ApplicationEvidence
                ),
                ReferenceStage::Qa => !matches!(
                    reference.namespace,
                    EngineeringEvidenceNamespace::QaFinding | EngineeringEvidenceNamespace::QaCheck
                ),
                ReferenceStage::Security => true,
            };
            if !valid || !allowed {
                return Err(EngineeringQualityError::UnknownReference);
            }
        }
        Ok(())
    }
}

impl fmt::Debug for EngineeringQualityWorkflowRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EngineeringQualityWorkflowRequest")
            .field("objective", &"[REDACTED]")
            .field("fixture_count", &self.fixtures.files.len())
            .field("criterion_count", &self.criteria.len())
            .field("evidence_count", &self.evidence.entries.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringQualityStage {
    Coding,
    QaValidation,
    SecurityReview,
    Synthesis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringPartialFailureCode {
    RuntimeStartFailed,
    RuntimeFailed,
    InvalidStructuredOutput,
    Cancelled,
    ContainsDeniedCapability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringContinuationFailure {
    CodingStartFailed,
    CodingAndSynthesisStartFailed,
    QaStartFailed,
    QaAndSynthesisStartFailed,
    QaAndSecurityStartFailed,
    QaSecurityAndSynthesisStartFailed,
    SecurityStartFailed,
    SecurityAndSynthesisStartFailed,
    SynthesisStartFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EngineeringQualityWorkflowEvent {
    CodingStarted {
        task_id: AgentTaskId,
    },
    CodingCompleted {
        task_id: AgentTaskId,
        quality: ChangeProposalQuality,
    },
    QaValidationStarted {
        task_id: AgentTaskId,
        predecessor_task_id: AgentTaskId,
    },
    QaValidationCompleted {
        task_id: AgentTaskId,
        conclusion: ValidationConclusion,
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
        stage: EngineeringQualityStage,
        code: EngineeringPartialFailureCode,
    },
    Cancelled {
        stage: EngineeringQualityStage,
    },
    Completed {
        task_id: AgentTaskId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringQualityAuditOutcome {
    Started,
    Completed,
    PartialFailure(EngineeringPartialFailureCode),
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringCapabilityAuditDisposition {
    NotApplicable,
    ProposalOnly,
    DeniedRequested,
}

#[derive(Clone, Eq, PartialEq)]
pub struct EngineeringQualityAttribution {
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

impl EngineeringQualityAttribution {
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

impl fmt::Debug for EngineeringQualityAttribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EngineeringQualityAttribution")
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
pub struct EngineeringQualityAuditRecord {
    sequence: u8,
    attribution: EngineeringQualityAttribution,
    predecessor_task_id: Option<AgentTaskId>,
    stage: EngineeringQualityStage,
    outcome: EngineeringQualityAuditOutcome,
    capability_disposition: EngineeringCapabilityAuditDisposition,
}

impl EngineeringQualityAuditRecord {
    pub(super) fn new(
        sequence: u8,
        attribution: EngineeringQualityAttribution,
        predecessor_task_id: Option<AgentTaskId>,
        stage: EngineeringQualityStage,
        outcome: EngineeringQualityAuditOutcome,
        capability_disposition: EngineeringCapabilityAuditDisposition,
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
    pub fn attribution(&self) -> &EngineeringQualityAttribution {
        &self.attribution
    }
    #[must_use]
    pub const fn stage(&self) -> EngineeringQualityStage {
        self.stage
    }
    #[must_use]
    pub const fn outcome(&self) -> EngineeringQualityAuditOutcome {
        self.outcome
    }

    #[must_use]
    pub const fn capability_disposition(&self) -> EngineeringCapabilityAuditDisposition {
        self.capability_disposition
    }

    #[must_use]
    pub fn predecessor_task_id(&self) -> Option<&AgentTaskId> {
        self.predecessor_task_id.as_ref()
    }
}

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum EngineeringQualityError {
    #[error("engineering identifier is invalid")]
    InvalidId,
    #[error("engineering text or aggregate exceeds its closed bound")]
    BoundExceeded,
    #[error("fixture path label is invalid")]
    InvalidFixturePath,
    #[error("engineering reference is duplicated")]
    DuplicateReference,
    #[error("engineering reference is unknown or unavailable to this stage")]
    UnknownReference,
    #[error("application evidence is inconsistent")]
    EvidenceInconsistent,
    #[error("capability request is inconsistent")]
    CapabilityInconsistent,
    #[error("criterion coverage is inconsistent")]
    CoverageMismatch,
    #[error("the result makes an execution or authority claim")]
    AuthorityClaim,
    #[error("the synthesis contradicts application-derived stage state")]
    SynthesisInconsistent,
    #[error("structured output is invalid")]
    InvalidStructuredOutput,
    #[error("bounded serialization failed")]
    SerializationFailed,
}

#[derive(Serialize)]
struct FixtureFileWire<'a> {
    id: &'a str,
    path: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct CriterionWireRef<'a> {
    id: &'a str,
    text: &'a str,
}

#[derive(Serialize)]
struct ApplicationEvidenceWire<'a> {
    id: &'a str,
    kind: ApplicationEvidenceKind,
    status: ApplicationEvidenceStatus,
    description: &'a str,
    criterion_ids: Vec<&'a str>,
    file_ids: Vec<&'a str>,
}

impl<'a> From<&'a ApplicationValidationEvidence> for ApplicationEvidenceWire<'a> {
    fn from(value: &'a ApplicationValidationEvidence) -> Self {
        Self {
            id: value.id.as_str(),
            kind: value.kind,
            status: value.status,
            description: &value.description,
            criterion_ids: value
                .criterion_ids
                .iter()
                .map(EngineeringId::as_str)
                .collect(),
            file_ids: value.file_ids.iter().map(EngineeringId::as_str).collect(),
        }
    }
}

#[derive(Serialize)]
struct ChangeProposalTransferWire<'a> {
    version: &'static str,
    objective_summary: &'a str,
    findings: Vec<ProposalFindingTransferWire<'a>>,
    affected_file_ids: Vec<&'a str>,
    patch_operations: Vec<PatchOperationTransferWire<'a>>,
    risks: &'a [String],
    validation_steps: Vec<ValidationStepTransferWire<'a>>,
    rollback: &'a str,
    capability_requests: &'a [EngineeringCapability],
    fixture_based: bool,
    proposal_only: bool,
    #[serde(skip_serializing_if = "is_false")]
    contains_denied_capability: bool,
}

const fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Serialize)]
struct ProposalFindingTransferWire<'a> {
    id: &'a str,
    statement: &'a str,
    references: Vec<EvidenceRefTransferWire<'a>>,
    confidence: Confidence,
}

#[derive(Serialize)]
struct PatchOperationTransferWire<'a> {
    file_id: &'a str,
    proposal: &'a str,
}

#[derive(Serialize)]
struct ValidationStepTransferWire<'a> {
    id: &'a str,
    text: &'a str,
}

#[derive(Serialize)]
struct EvidenceRefTransferWire<'a> {
    namespace: EngineeringEvidenceNamespace,
    id: &'a str,
}

#[allow(clippy::too_many_arguments)]
fn serialize_change_proposal_transfer(
    objective_summary: &str,
    findings: &[ProposalFinding],
    affected_file_ids: &[EngineeringId],
    patch_operations: &[PatchOperation],
    risks: &[String],
    validation_steps: &[ProposalValidationStep],
    rollback: &str,
    capabilities: &[EngineeringCapability],
    contains_denied_capability: bool,
) -> EngineeringQualityResult<String> {
    serde_json::to_string(&ChangeProposalTransferWire {
        version: "v1",
        objective_summary,
        findings: findings
            .iter()
            .map(|finding| ProposalFindingTransferWire {
                id: finding.id.as_str(),
                statement: &finding.statement,
                references: finding
                    .references
                    .iter()
                    .map(|reference| EvidenceRefTransferWire {
                        namespace: reference.namespace,
                        id: reference.id.as_str(),
                    })
                    .collect(),
                confidence: finding.confidence,
            })
            .collect(),
        affected_file_ids: affected_file_ids
            .iter()
            .map(EngineeringId::as_str)
            .collect(),
        patch_operations: patch_operations
            .iter()
            .map(|operation| PatchOperationTransferWire {
                file_id: operation.file_id.as_str(),
                proposal: &operation.proposal,
            })
            .collect(),
        risks,
        validation_steps: validation_steps
            .iter()
            .map(|step| ValidationStepTransferWire {
                id: step.id.as_str(),
                text: &step.text,
            })
            .collect(),
        rollback,
        capability_requests: capabilities,
        fixture_based: true,
        proposal_only: true,
        contains_denied_capability,
    })
    .map_err(|_| EngineeringQualityError::SerializationFailed)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChangeProposalWire {
    #[serde(rename = "version")]
    _version: ChangeProposalVersionWire,
    objective_summary: String,
    findings: Vec<ProposalFindingWire>,
    affected_file_ids: Vec<String>,
    patch_operations: Vec<PatchOperationWire>,
    risks: Vec<String>,
    validation_steps: Vec<ValidationStepWire>,
    rollback: String,
    capability_requests: Vec<EngineeringCapability>,
    fixture_based: bool,
    proposal_only: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ChangeProposalVersionWire {
    V1,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProposalFindingWire {
    id: String,
    statement: String,
    references: Vec<EvidenceRefWire>,
    confidence: Confidence,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PatchOperationWire {
    file_id: String,
    proposal: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidationStepWire {
    id: String,
    text: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidationReportWire {
    #[serde(rename = "version")]
    _version: ValidationReportVersionWire,
    coverage: Vec<CriterionCoverageWire>,
    findings: Vec<QaFindingWire>,
    proposed_checks: Vec<QaCheckWire>,
    gaps: Vec<String>,
    conclusion: ValidationConclusion,
    advisory_only: bool,
    approval_authority: bool,
    evidence_executed: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ValidationReportVersionWire {
    V1,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CriterionCoverageWire {
    criterion_id: String,
    disposition: CriterionDisposition,
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
    status: ApplicationEvidenceStatus,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskAssessmentWire {
    #[serde(rename = "version")]
    _version: RiskAssessmentVersionWire,
    findings: Vec<RiskFindingWire>,
    unresolved_risks: Vec<String>,
    follow_ups: Vec<String>,
    advisory_only: bool,
    authorization_granted: bool,
    remediation_executed: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum RiskAssessmentVersionWire {
    V1,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskFindingWire {
    id: String,
    category: RiskCategory,
    severity: RiskSeverity,
    confidence: Confidence,
    statement: String,
    basis: RiskFindingBasisWire,
    references: Vec<EvidenceRefWire>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum RiskFindingBasisWire {
    EvidenceBound,
    Hypothesis,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SynthesisWire {
    #[serde(rename = "version")]
    _version: SynthesisVersionWire,
    summary: String,
    affected_file_ids: Vec<String>,
    unresolved_issues: Vec<String>,
    fixture_based: bool,
    proposal_only: bool,
    changes_applied: bool,
    tests_executed: bool,
    status: EngineeringReviewStatus,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum SynthesisVersionWire {
    V1,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceRefWire {
    namespace: EngineeringEvidenceNamespace,
    id: String,
}

#[derive(Clone, Copy)]
enum ReferenceStage {
    Coding,
    Qa,
    Security,
}

fn parse_exact<T: DeserializeOwned>(raw: &str) -> EngineeringQualityResult<T> {
    if raw.trim() != raw || !raw.starts_with('{') || !raw.ends_with('}') {
        return Err(EngineeringQualityError::InvalidStructuredOutput);
    }
    let mut deserializer = serde_json::Deserializer::from_str(raw);
    let value = T::deserialize(&mut deserializer)
        .map_err(|_| EngineeringQualityError::InvalidStructuredOutput)?;
    deserializer
        .end()
        .map_err(|_| EngineeringQualityError::InvalidStructuredOutput)?;
    Ok(value)
}

fn parse_refs(
    values: Vec<EvidenceRefWire>,
) -> EngineeringQualityResult<Vec<EngineeringEvidenceRef>> {
    parse_refs_with_minimum(values, 1)
}

fn parse_refs_with_minimum(
    values: Vec<EvidenceRefWire>,
    minimum: usize,
) -> EngineeringQualityResult<Vec<EngineeringEvidenceRef>> {
    ensure_count(values.len(), minimum, MAX_ENGINEERING_REFERENCES)?;
    let parsed = values
        .into_iter()
        .map(|value| EngineeringEvidenceRef::new(value.namespace, value.id))
        .collect::<EngineeringQualityResult<Vec<_>>>()?;
    ensure_unique(&parsed, minimum, MAX_ENGINEERING_REFERENCES)?;
    Ok(parsed)
}

fn parse_ids(
    values: Vec<String>,
    minimum: usize,
    maximum: usize,
) -> EngineeringQualityResult<Vec<EngineeringId>> {
    let parsed = values
        .into_iter()
        .map(EngineeringId::new)
        .collect::<EngineeringQualityResult<Vec<_>>>()?;
    ensure_unique(&parsed, minimum, maximum)?;
    Ok(parsed)
}

fn validate_fixture_path(path: &str) -> EngineeringQualityResult<()> {
    if path.is_empty()
        || path.chars().count() > MAX_ENGINEERING_PATH_CHARACTERS
        || path.len() > MAX_ENGINEERING_PATH_BYTES
        || path.starts_with('/')
        || path.contains('\\')
        || path.contains("://")
        || path.chars().any(char::is_control)
    {
        return Err(EngineeringQualityError::InvalidFixturePath);
    }
    let parts: Vec<_> = path.split('/').collect();
    if parts.len() > 8
        || parts
            .iter()
            .any(|part| part.is_empty() || *part == "." || *part == "..")
        || parts.first().is_some_and(|part| part.contains(':'))
    {
        return Err(EngineeringQualityError::InvalidFixturePath);
    }
    Ok(())
}

fn validate_raw(
    raw: &str,
    max_characters: usize,
    max_bytes: usize,
) -> EngineeringQualityResult<()> {
    if raw.chars().count() > max_characters || raw.len() > max_bytes {
        Err(EngineeringQualityError::BoundExceeded)
    } else {
        Ok(())
    }
}

fn validate_text(
    value: &str,
    max_characters: usize,
    max_bytes: usize,
    allow_newline: bool,
) -> EngineeringQualityResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().count() > max_characters
        || value.len() > max_bytes
        || value.chars().any(|character| {
            character.is_control() && !(allow_newline && matches!(character, '\n' | '\t'))
        })
    {
        return Err(EngineeringQualityError::BoundExceeded);
    }
    Ok(())
}

fn validate_model_text(
    value: &str,
    max_characters: usize,
    max_bytes: usize,
    allow_newline: bool,
) -> EngineeringQualityResult<()> {
    validate_text(value, max_characters, max_bytes, allow_newline)?;
    let lower = value.to_ascii_lowercase();
    if contains_overclaim(value)
        || lower.contains("://")
        || contains_ascii_token_prefix(&lower, "www.")
        || contains_ascii_token_prefix(&lower, "mailto:")
        || contains_ascii_token_prefix(&lower, "data:")
        || contains_ascii_token_prefix(&lower, "file:")
        || contains_normalized_phrase(value, "chain of thought")
        || contains_normalized_phrase(value, "private reasoning")
    {
        Err(EngineeringQualityError::AuthorityClaim)
    } else {
        Ok(())
    }
}

fn contains_ascii_token_prefix(value: &str, prefix: &str) -> bool {
    value.match_indices(prefix).any(|(index, _)| {
        index == 0
            || value[..index]
                .chars()
                .next_back()
                .is_none_or(|character| !character.is_ascii_alphanumeric())
    })
}

fn validate_texts(values: &[String], max_bytes: usize) -> EngineeringQualityResult<()> {
    ensure_count(values.len(), 0, MAX_ENGINEERING_ITEMS)?;
    values
        .iter()
        .try_for_each(|value| validate_model_text(value, MAX_TEXT_CHARACTERS, max_bytes, false))
}

fn contains_overclaim(value: &str) -> bool {
    let normalized = normalized_ascii_words(value);
    let padded = format!(" {normalized} ");
    const PHRASES: [&str; 17] = [
        "changes applied",
        "patch applied",
        "files updated",
        "test passed",
        "tests passed",
        "test failed",
        "tests failed",
        "test executed",
        "tests executed",
        "dependency installed",
        "git committed",
        "git pushed",
        "approval granted",
        "authorized to execute",
        "remediation completed",
        "confirmed vulnerability",
        "definitely vulnerable",
    ];
    PHRASES
        .iter()
        .any(|phrase| padded.contains(&format!(" {phrase} ")))
        || padded.contains(" proven exploitable ")
}

fn contains_normalized_phrase(value: &str, phrase: &str) -> bool {
    format!(" {} ", normalized_ascii_words(value)).contains(&format!(" {phrase} "))
}

fn normalized_ascii_words(value: &str) -> String {
    let mut normalized = String::new();
    let mut separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            if separator && !normalized.is_empty() {
                normalized.push(' ');
            }
            normalized.push(character.to_ascii_lowercase());
            separator = false;
        } else {
            separator = true;
        }
    }
    normalized
}

fn contains_ascii_word(value: &str, expected: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|word| word == expected)
}

fn bounded_stage_input(value: String) -> EngineeringQualityResult<String> {
    if value.len() > MAX_ENGINEERING_STAGE_INPUT_BYTES
        || value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
    {
        Err(EngineeringQualityError::BoundExceeded)
    } else {
        Ok(value)
    }
}

fn bounded_transfer(raw: &str, maximum: usize) -> EngineeringQualityResult<String> {
    if raw.len() > maximum {
        Err(EngineeringQualityError::BoundExceeded)
    } else {
        Ok(raw.to_owned())
    }
}

fn ensure_count(count: usize, minimum: usize, maximum: usize) -> EngineeringQualityResult<()> {
    if (minimum..=maximum).contains(&count) {
        Ok(())
    } else {
        Err(EngineeringQualityError::BoundExceeded)
    }
}

fn ensure_unique<T: Ord>(
    values: &[T],
    minimum: usize,
    maximum: usize,
) -> EngineeringQualityResult<()> {
    ensure_count(values.len(), minimum, maximum)?;
    if values.iter().collect::<BTreeSet<_>>().len() == values.len() {
        Ok(())
    } else {
        Err(EngineeringQualityError::DuplicateReference)
    }
}

fn ensure_unique_by<'a, T: 'a + Ord>(
    values: impl Iterator<Item = &'a T>,
) -> EngineeringQualityResult<()> {
    let values: Vec<_> = values.collect();
    if values.iter().copied().collect::<BTreeSet<_>>().len() == values.len() {
        Ok(())
    } else {
        Err(EngineeringQualityError::DuplicateReference)
    }
}

fn coding_transfer(value: &CodingStageOutcome) -> &str {
    match value {
        CodingStageOutcome::Completed(result) => result.transfer(),
        CodingStageOutcome::Failed(_) => "status=coding-failed",
        CodingStageOutcome::Cancelled => "status=coding-cancelled",
    }
}
fn qa_transfer(value: &QaStageOutcome) -> &str {
    match value {
        QaStageOutcome::Completed(result) => result.transfer(),
        QaStageOutcome::Failed(_) => "status=qa-failed",
        QaStageOutcome::Cancelled => "status=qa-cancelled",
        QaStageOutcome::SkippedCodingUnavailable => "status=qa-skipped",
    }
}
fn security_transfer(value: &SecurityStageOutcome) -> &str {
    match value {
        SecurityStageOutcome::Completed(result) => result.transfer(),
        SecurityStageOutcome::Failed(_) => "status=security-failed",
        SecurityStageOutcome::Cancelled => "status=security-cancelled",
        SecurityStageOutcome::SkippedCodingUnavailable => "status=security-skipped",
    }
}

fn stage_label<T>(_value: &T) -> &'static str {
    "[REDACTED]"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> EngineeringQualityResult<EngineeringQualityWorkflowRequest> {
        let fixtures = FixtureRepositoryCatalog::new(vec![FixtureRepositoryFile::new(
            "source-file",
            "src/example.rs",
            "fn compare() { /* deterministic fixture */ }",
        )?])?;
        let criteria = vec![AcceptanceCriterion::new(
            "criterion-1",
            "The proposal addresses the fixture comparison",
        )?];
        let evidence =
            ApplicationValidationEvidenceCatalog::new(vec![ApplicationValidationEvidence::new(
                "evidence-1",
                ApplicationEvidenceKind::FixtureObservation,
                ApplicationEvidenceStatus::ObservedFixture,
                "The supplied fixture contains the comparison function",
                vec![EngineeringId::new("criterion-1")?],
                vec![EngineeringId::new("source-file")?],
            )?])?;
        EngineeringQualityWorkflowRequest::new(
            "Compare two fixture approaches and propose a safe change",
            fixtures,
            criteria,
            evidence,
        )
    }

    fn proposal_json() -> &'static str {
        r#"{"version":"v1","objective_summary":"Compare the supplied fixture approaches","findings":[{"id":"finding-1","statement":"The comparison branch is duplicated","references":[{"namespace":"fixture-file","id":"source-file"}],"confidence":"high"}],"affected_file_ids":["source-file"],"patch_operations":[{"file_id":"source-file","proposal":"Extract one shared proposal-only helper"}],"risks":["The proposed helper could alter branch selection"],"validation_steps":[{"id":"validation-1","text":"Review both fixture branches with the supplied criterion"}],"rollback":"Discard the inert proposal","capability_requests":["fixture-inspect","patch-proposal","validation-plan"],"fixture_based":true,"proposal_only":true}"#
    }

    fn qa_json() -> &'static str {
        r#"{"version":"v1","coverage":[{"criterion_id":"criterion-1","disposition":"demonstrated","references":[{"namespace":"application-evidence","id":"evidence-1"}]}],"findings":[{"id":"qa-finding-1","statement":"The supplied evidence covers the requested comparison","references":[{"namespace":"proposal-finding","id":"finding-1"}]}],"proposed_checks":[{"id":"qa-check-1","text":"Review the proposed helper without running it","references":[{"namespace":"proposal-validation-step","id":"validation-1"}],"status":"not-run"}],"gaps":[],"conclusion":"adequate","advisory_only":true,"approval_authority":false,"evidence_executed":false}"#
    }

    fn risk_json() -> &'static str {
        r#"{"version":"v1","findings":[{"id":"risk-1","category":"input-validation","severity":"medium","confidence":"medium","statement":"The proposal should preserve branch input checks","basis":"evidence-bound","references":[{"namespace":"qa-finding","id":"qa-finding-1"}]}],"unresolved_risks":["No live test evidence is available"],"follow_ups":["Review the not-run check before mutation"],"advisory_only":true,"authorization_granted":false,"remediation_executed":false}"#
    }

    #[test]
    fn strict_results_preserve_tagged_provenance_and_derive_authority(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = request()?;
        let proposal = request.parse_change_proposal(
            AgentTaskId::new("engineering-coding-task")?,
            proposal_json(),
        )?;
        assert_eq!(proposal.quality(), ChangeProposalQuality::Complete);
        let qa = request.parse_validation_report(
            AgentTaskId::new("engineering-qa-task")?,
            &proposal,
            qa_json(),
        )?;
        assert_eq!(qa.conclusion(), ValidationConclusion::Adequate);
        let qa_outcome = QaStageOutcome::Completed(qa);
        let risk = request.parse_risk_assessment(
            AgentTaskId::new("engineering-security-task")?,
            &proposal,
            &qa_outcome,
            risk_json(),
        )?;
        assert_eq!(risk.findings().len(), 1);
        assert_eq!(
            risk.dependency_evidence(),
            &DependencyEvidenceStatus::Unavailable
        );
        let synthesis = request.parse_synthesis(
            &CodingStageOutcome::Completed(Box::new(proposal)),
            &qa_outcome,
            &SecurityStageOutcome::Completed(risk),
            r#"{"version":"v1","summary":"Fixture review supports the inert helper proposal","affected_file_ids":["source-file"],"unresolved_issues":["No live checks were run"],"fixture_based":true,"proposal_only":true,"changes_applied":false,"tests_executed":false,"status":"complete"}"#,
        )?;
        assert_eq!(
            synthesis.approval_requirement(),
            EngineeringApprovalRequirement::RequiredBeforeMutation
        );
        assert_eq!(synthesis.status(), EngineeringReviewStatus::Complete);
        Ok(())
    }

    #[test]
    fn fixture_labels_and_bounds_reject_path_authority() {
        for path in [
            "../outside.rs",
            "/absolute.rs",
            "a/../../b",
            "a\\b",
            "file://a",
        ] {
            assert!(matches!(
                FixtureRepositoryFile::new("file", path, "bounded fixture"),
                Err(EngineeringQualityError::InvalidFixturePath)
            ));
        }
        assert!(FixtureRepositoryFile::new(
            "file",
            "src/example.rs",
            "x".repeat(MAX_ENGINEERING_FILE_BYTES + 1),
        )
        .is_err());
    }

    #[test]
    fn duplicate_unknown_and_authority_claims_fail_closed() -> Result<(), Box<dyn std::error::Error>>
    {
        let request = request()?;
        assert_eq!(
            request.parse_change_proposal(
                AgentTaskId::new("engineering-coding-task")?,
                &proposal_json().replace(
                    "\"version\":\"v1\"",
                    "\"version\":\"v1\",\"version\":\"v1\""
                ),
            ),
            Err(EngineeringQualityError::InvalidStructuredOutput)
        );
        assert_eq!(
            request.parse_change_proposal(
                AgentTaskId::new("engineering-coding-task")?,
                &proposal_json().replace("source-file", "unknown-file"),
            ),
            Err(EngineeringQualityError::UnknownReference)
        );
        assert_eq!(
            request.parse_change_proposal(
                AgentTaskId::new("engineering-coding-task")?,
                &proposal_json().replace(
                    "Compare the supplied fixture approaches",
                    "Patch applied to the supplied fixture"
                ),
            ),
            Err(EngineeringQualityError::AuthorityClaim)
        );
        assert_eq!(
            request.parse_change_proposal(
                AgentTaskId::new("engineering-coding-task")?,
                &proposal_json().replace(
                    "The comparison branch is duplicated",
                    "See https://example.invalid/private reasoning",
                ),
            ),
            Err(EngineeringQualityError::AuthorityClaim)
        );
        Ok(())
    }

    #[test]
    fn claim_and_url_guards_use_token_boundaries_without_benign_substring_matches() {
        assert!(validate_model_text(
            "Contest passed; metadata: unavailable; profile: bounded",
            128,
            256,
            false,
        )
        .is_ok());
        for rejected in [
            "Tests passed",
            "See FTP://example.invalid",
            "Contact mailto:owner@example.invalid",
            "Use data:text/plain,fixture",
            "Open file:/private/example",
            "Do not expose chain-of-thought",
            "Do not expose private_reasoning",
        ] {
            assert_eq!(
                validate_model_text(rejected, 128, 256, false),
                Err(EngineeringQualityError::AuthorityClaim)
            );
        }
    }

    #[test]
    fn denied_capability_is_inert_partial_data() -> Result<(), Box<dyn std::error::Error>> {
        let request = request()?;
        let raw = proposal_json().replace(
            "\"validation-plan\"]",
            "\"validation-plan\",\"file-write\",\"git-push\"]",
        );
        let proposal =
            request.parse_change_proposal(AgentTaskId::new("engineering-coding-task")?, &raw)?;
        assert_eq!(
            proposal.quality(),
            ChangeProposalQuality::PartialDeniedCapability
        );
        assert!(!proposal.transfer().contains("file-write"));
        assert!(!proposal.transfer().contains("git-push"));
        assert!(proposal
            .transfer()
            .contains("\"contains_denied_capability\":true"));
        assert_eq!(
            EngineeringCapability::GitPush.disposition(),
            EngineeringCapabilityDisposition::Denied
        );
        Ok(())
    }

    #[test]
    fn debug_and_errors_redact_fixture_and_result_content() -> Result<(), Box<dyn std::error::Error>>
    {
        let request = request()?;
        let proposal = request.parse_change_proposal(
            AgentTaskId::new("engineering-coding-task")?,
            proposal_json(),
        )?;
        let rendered = format!("{request:?} {proposal:?}");
        assert!(!rendered.contains("deterministic fixture"));
        assert!(!rendered.contains("comparison branch is duplicated"));
        assert!(!format!("{:?}", EngineeringQualityError::UnknownReference).contains("source-file"));
        Ok(())
    }

    #[test]
    fn catalog_aggregate_bounds_and_reference_free_proposed_checks_are_exact(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixtures = FixtureRepositoryCatalog::new(vec![FixtureRepositoryFile::new(
            "source-file",
            "src/example.rs",
            "bounded fixture",
        )?])?;
        let criteria = (0..3)
            .map(|index| AcceptanceCriterion::new(format!("criterion-{index}"), "€".repeat(300)))
            .collect::<EngineeringQualityResult<Vec<_>>>()?;
        assert!(matches!(
            EngineeringQualityWorkflowRequest::new(
                "bounded objective",
                fixtures.clone(),
                criteria,
                ApplicationValidationEvidenceCatalog::new(Vec::new())?,
            ),
            Err(EngineeringQualityError::BoundExceeded)
        ));

        let criterion_id = EngineeringId::new("criterion-1")?;
        let excessive_evidence = vec![
            ApplicationValidationEvidence::new(
                "evidence-a",
                ApplicationEvidenceKind::ProposedCheck,
                ApplicationEvidenceStatus::NotRun,
                "€".repeat(400),
                vec![criterion_id.clone()],
                Vec::new(),
            )?,
            ApplicationValidationEvidence::new(
                "evidence-b",
                ApplicationEvidenceKind::ProposedCheck,
                ApplicationEvidenceStatus::NotRun,
                "€".repeat(400),
                vec![criterion_id.clone()],
                Vec::new(),
            )?,
            ApplicationValidationEvidence::new(
                "evidence-c",
                ApplicationEvidenceKind::ProposedCheck,
                ApplicationEvidenceStatus::NotRun,
                "€".repeat(400),
                vec![criterion_id],
                Vec::new(),
            )?,
        ];
        assert!(matches!(
            ApplicationValidationEvidenceCatalog::new(excessive_evidence),
            Err(EngineeringQualityError::BoundExceeded)
        ));

        let request = request()?;
        let proposal = request.parse_change_proposal(
            AgentTaskId::new("engineering-coding-task")?,
            proposal_json(),
        )?;
        let qa_without_check_references = qa_json().replace(
            r#""references":[{"namespace":"proposal-validation-step","id":"validation-1"}]"#,
            r#""references":[]"#,
        );
        let report = request.parse_validation_report(
            AgentTaskId::new("engineering-qa-task")?,
            &proposal,
            &qa_without_check_references,
        )?;
        assert!(report.proposed_checks()[0].references().is_empty());
        Ok(())
    }

    #[test]
    fn qa_coverage_rejects_application_evidence_bound_to_another_criterion(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixtures = FixtureRepositoryCatalog::new(vec![FixtureRepositoryFile::new(
            "source-file",
            "src/example.rs",
            "bounded fixture",
        )?])?;
        let criteria = vec![
            AcceptanceCriterion::new("criterion-1", "First criterion")?,
            AcceptanceCriterion::new("criterion-2", "Second criterion")?,
        ];
        let evidence = ApplicationValidationEvidenceCatalog::new(vec![
            ApplicationValidationEvidence::new(
                "evidence-1",
                ApplicationEvidenceKind::FixtureObservation,
                ApplicationEvidenceStatus::ObservedFixture,
                "Evidence for the first criterion",
                vec![EngineeringId::new("criterion-1")?],
                vec![EngineeringId::new("source-file")?],
            )?,
            ApplicationValidationEvidence::new(
                "evidence-2",
                ApplicationEvidenceKind::FixtureObservation,
                ApplicationEvidenceStatus::ObservedFixture,
                "Evidence for the second criterion",
                vec![EngineeringId::new("criterion-2")?],
                vec![EngineeringId::new("source-file")?],
            )?,
        ])?;
        let request = EngineeringQualityWorkflowRequest::new(
            "Review the bounded fixture",
            fixtures,
            criteria,
            evidence,
        )?;
        let proposal = request.parse_change_proposal(
            AgentTaskId::new("engineering-coding-task")?,
            proposal_json(),
        )?;
        let invalid = r#"{"version":"v1","coverage":[{"criterion_id":"criterion-1","disposition":"demonstrated","references":[{"namespace":"application-evidence","id":"evidence-1"},{"namespace":"application-evidence","id":"evidence-2"}]},{"criterion_id":"criterion-2","disposition":"demonstrated","references":[{"namespace":"application-evidence","id":"evidence-2"}]}],"findings":[],"proposed_checks":[],"gaps":[],"conclusion":"adequate","advisory_only":true,"approval_authority":false,"evidence_executed":false}"#;
        assert_eq!(
            request.parse_validation_report(
                AgentTaskId::new("engineering-qa-task")?,
                &proposal,
                invalid,
            ),
            Err(EngineeringQualityError::EvidenceInconsistent)
        );
        Ok(())
    }
}
