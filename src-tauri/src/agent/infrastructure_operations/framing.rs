use super::*;

pub(super) fn build_catalog_stage_input(
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

pub(super) fn build_synthesis_stage_input(
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

pub(super) fn build_synthesis_stage_input_from_transfers(
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

pub(super) fn cloud_transfer(outcome: &CloudInfrastructureStageOutcome) -> &str {
    match outcome {
        CloudInfrastructureStageOutcome::Completed(value) => value.transfer(),
        CloudInfrastructureStageOutcome::Failed(_) => "status=cloud-assessment-failed",
        CloudInfrastructureStageOutcome::Cancelled => "status=cloud-assessment-cancelled",
    }
}

pub(super) fn systems_transfer(outcome: &SystemsOperationsStageOutcome) -> &str {
    match outcome {
        SystemsOperationsStageOutcome::Completed(value) => value.transfer(),
        SystemsOperationsStageOutcome::Failed(_) => "status=systems-assessment-failed",
        SystemsOperationsStageOutcome::Cancelled => "status=systems-assessment-cancelled",
    }
}

pub(super) fn qa_transfer(outcome: &InfrastructureOperationsQaStageOutcome) -> &str {
    match outcome {
        InfrastructureOperationsQaStageOutcome::Completed(value) => value.transfer(),
        InfrastructureOperationsQaStageOutcome::Failed(_) => "status=qa-failed",
        InfrastructureOperationsQaStageOutcome::Cancelled => "status=qa-cancelled",
        InfrastructureOperationsQaStageOutcome::SkippedFirstStageUnavailable => "status=qa-skipped",
    }
}

pub(super) fn security_transfer(outcome: &InfrastructureOperationsSecurityStageOutcome) -> &str {
    match outcome {
        InfrastructureOperationsSecurityStageOutcome::Completed(value) => value.transfer(),
        InfrastructureOperationsSecurityStageOutcome::Failed(_) => "status=security-failed",
        InfrastructureOperationsSecurityStageOutcome::Cancelled => "status=security-cancelled",
        InfrastructureOperationsSecurityStageOutcome::SkippedFirstStageUnavailable => {
            "status=security-skipped"
        }
    }
}

pub(super) fn infrastructure_partial_failure_codes(
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

pub(super) fn systems_partial_failure_codes(
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
        | AgentTaskFailureCode::DeadlineExceeded
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

pub(super) fn cloud_stage_projection(
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

pub(super) fn systems_stage_projection(
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

pub(super) fn bounded_transfer(raw: &str) -> InfrastructureOperationsResult<String> {
    if raw.len() > MAX_INFRASTRUCTURE_OPERATIONS_TRANSFER_BYTES {
        Err(InfrastructureOperationsError::BoundExceeded)
    } else {
        Ok(raw.to_owned())
    }
}

pub(super) fn assessment_references(
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
