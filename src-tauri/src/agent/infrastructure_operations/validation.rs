use std::collections::BTreeSet;

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::framing::{
    bounded_transfer, cloud_stage_projection, infrastructure_partial_failure_codes,
    systems_partial_failure_codes, systems_stage_projection,
};
use super::*;

pub(super) fn validate_text(
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

pub(super) fn ensure_count(
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

pub(super) fn ensure_unique<T: Ord>(
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

pub(super) fn ensure_unique_by<'a, T: 'a + Ord>(
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

pub(super) fn reject_credential_text(value: &str) -> InfrastructureOperationsResult<()> {
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

pub(super) fn parse_infrastructure_assessment(
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

pub(super) fn parse_operational_assessment(
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

pub(super) fn parse_validation_report(
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

pub(super) fn parse_risk_assessment(
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

pub(super) fn parse_cloud_synthesis(
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

pub(super) fn parse_systems_synthesis(
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

pub(super) fn validate_approval_summary_consistency(
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

pub(super) fn validate_model_text(value: &str) -> InfrastructureOperationsResult<()> {
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

pub(super) fn normalized_claim_tokens(value: &str) -> Vec<String> {
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

pub(super) fn validate_cloud_capability_consistency(
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

pub(super) fn validate_systems_capability_consistency(
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

pub(super) fn validate_no_effect_requests(values: &[&str]) -> InfrastructureOperationsResult<()> {
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

pub(super) fn contains_effect_token_sequence(tokens: &[String], phrase: &[&str]) -> bool {
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
