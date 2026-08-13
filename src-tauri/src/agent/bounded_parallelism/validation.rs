use std::collections::BTreeSet;

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use super::*;

#[derive(Serialize)]
struct FixtureWire<'a> {
    id: &'a str,
    label: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChildWire {
    version: String,
    scenario_id: BoundedParallelScenarioId,
    work_item_id: String,
    summary: String,
    findings: Vec<FindingWire>,
    unresolved_issues: Vec<String>,
    fixture_based: bool,
    live_access_performed: bool,
    tools_executed: bool,
    credentials_loaded: bool,
    effects_performed: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FindingWire {
    id: String,
    statement: String,
    confidence: ParallelConfidence,
    references: Vec<EvidenceWire>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceWire {
    namespace: String,
    id: String,
}

#[derive(Serialize)]
struct ChildTransferWire<'a> {
    ordinal: u8,
    work_item_id: &'a str,
    agent_id: &'a str,
    status: ParallelChildResultStatus,
    summary: &'a str,
    findings: Vec<TransferFindingWire<'a>>,
    unresolved_issues: &'a [String],
}

#[derive(Serialize)]
struct TransferFindingWire<'a> {
    id: &'a str,
    statement: &'a str,
    confidence: ParallelConfidence,
    references: Vec<TransferEvidenceWire<'a>>,
}

#[derive(Serialize)]
struct TransferEvidenceWire<'a> {
    namespace: &'a str,
    id: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SynthesisWire {
    version: String,
    scenario_id: BoundedParallelScenarioId,
    status: BoundedParallelSynthesisStatus,
    source_results: Vec<StatusWire>,
    source_findings: Vec<FindingSourceWire>,
    summary: String,
    unresolved_issues: Vec<String>,
    fixture_based: bool,
    live_access_performed: bool,
    tools_executed: bool,
    credentials_loaded: bool,
    effects_performed: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StatusWire {
    ordinal: u8,
    work_item_id: String,
    agent_id: String,
    status: ParallelChildResultStatus,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FindingSourceWire {
    ordinal: u8,
    work_item_id: String,
    agent_id: String,
    finding_ids: Vec<String>,
}

pub(super) fn validate_identifier(value: &str) -> BoundedParallelResult<()> {
    let Some(first) = value.bytes().next() else {
        return Err(BoundedParallelError::InvalidText);
    };
    if value.len() > 64
        || !first.is_ascii_lowercase()
        || !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        Err(BoundedParallelError::InvalidText)
    } else {
        Ok(())
    }
}

pub(super) fn validate_text(
    value: &str,
    max_chars: usize,
    max_bytes: usize,
    layout: bool,
) -> BoundedParallelResult<()> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().count() > max_chars
        || value.len() > max_bytes
        || value
            .chars()
            .any(|c| c.is_control() && !(layout && matches!(c, '\n' | '\t')))
    {
        Err(BoundedParallelError::InvalidText)
    } else {
        Ok(())
    }
}

pub(super) fn validate_request(
    _scenario_id: BoundedParallelScenarioId,
    objective: &str,
    _policy: BoundedParallelFailurePolicy,
    items: &[ParallelWorkItem],
    active_limit: u8,
) -> BoundedParallelResult<()> {
    validate_text(
        objective,
        MAX_PARALLEL_OBJECTIVE_CHARACTERS,
        MAX_PARALLEL_OBJECTIVE_BYTES,
        true,
    )?;
    if items.is_empty()
        || items.len() > MAX_PARALLEL_CHILDREN
        || active_limit == 0
        || active_limit > HARD_MAX_ACTIVE_PARALLEL_CHILDREN
    {
        return Err(BoundedParallelError::InvalidCatalog);
    }
    let mut ids = BTreeSet::new();
    let mut agents = BTreeSet::new();
    let mut catalog_bytes = 0usize;
    let mut fixture_count = 0usize;
    for (index, item) in items.iter().enumerate() {
        if item.ordinal
            != u8::try_from(index + 1).map_err(|_| BoundedParallelError::InvalidCatalog)?
            || !ids.insert(item.id.clone())
            || !agents.insert(item.agent_id)
        {
            return Err(BoundedParallelError::InvalidCatalog);
        }
        validate_text(
            &item.objective,
            MAX_PARALLEL_OBJECTIVE_CHARACTERS,
            MAX_PARALLEL_OBJECTIVE_BYTES,
            true,
        )?;
        validate_text(&item.expected_output, 512, 2_048, true)?;
        if item.fixtures.is_empty() || item.fixtures.len() > MAX_PARALLEL_FIXTURES {
            return Err(BoundedParallelError::InvalidCatalog);
        }
        fixture_count = fixture_count
            .checked_add(item.fixtures.len())
            .ok_or(BoundedParallelError::BoundExceeded)?;
        if fixture_count > MAX_PARALLEL_FIXTURES {
            return Err(BoundedParallelError::BoundExceeded);
        }
        for dependency in &item.dependencies {
            if dependency == &item.id || !ids.contains(dependency) {
                return Err(BoundedParallelError::InvalidCatalog);
            }
        }
        let mut fixture_ids = BTreeSet::new();
        for fixture in &item.fixtures {
            if !fixture_ids.insert(fixture.id.clone()) {
                return Err(BoundedParallelError::InvalidCatalog);
            }
            validate_identifier(&fixture.id)?;
            validate_text(&fixture.label, 256, 1_024, false)?;
            if fixture.content.chars().count() > MAX_PARALLEL_FIXTURE_CHARACTERS
                || fixture.content.len() > MAX_PARALLEL_FIXTURE_BYTES
            {
                return Err(BoundedParallelError::BoundExceeded);
            }
            validate_text(
                &fixture.content,
                MAX_PARALLEL_FIXTURE_CHARACTERS,
                MAX_PARALLEL_FIXTURE_BYTES,
                true,
            )?;
        }
        let serialized = serde_json::to_vec(
            &item
                .fixtures
                .iter()
                .map(|fixture| FixtureWire {
                    id: &fixture.id,
                    label: &fixture.label,
                    content: &fixture.content,
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|_| BoundedParallelError::SerializationFailed)?;
        catalog_bytes = catalog_bytes
            .checked_add(serialized.len())
            .ok_or(BoundedParallelError::BoundExceeded)?;
    }
    if catalog_bytes > MAX_PARALLEL_CATALOG_BYTES {
        return Err(BoundedParallelError::BoundExceeded);
    }
    Ok(())
}

fn validate_framing_bytes(actual: usize) -> BoundedParallelResult<()> {
    if actual > MAX_PARALLEL_SYNTHESIS_FRAMING_BYTES {
        Err(BoundedParallelError::BoundExceeded)
    } else {
        Ok(())
    }
}

pub(super) fn build_child_input(
    request: &BoundedParallelWorkflowRequest,
    ordinal: u8,
    predecessors: &[ParallelChildOutcome],
) -> BoundedParallelResult<String> {
    let item = request.item(ordinal)?;
    if predecessors.len() != item.dependencies.len() {
        return Err(BoundedParallelError::DependencyUnavailable);
    }
    for (dependency, outcome) in item.dependencies.iter().zip(predecessors) {
        let expected = request
            .work_items
            .iter()
            .find(|candidate| candidate.id == *dependency)
            .ok_or(BoundedParallelError::InvalidCatalog)?;
        if outcome.work_item_id != *dependency
            || outcome.ordinal != expected.ordinal
            || outcome.agent_id != expected.agent_id
            || outcome.status() != ParallelChildResultStatus::Succeeded
        {
            return Err(BoundedParallelError::DependencyUnavailable);
        }
    }
    let fixtures = serde_json::to_string(
        &item
            .fixtures
            .iter()
            .map(|f| FixtureWire {
                id: &f.id,
                label: &f.label,
                content: &f.content,
            })
            .collect::<Vec<_>>(),
    )
    .map_err(|_| BoundedParallelError::SerializationFailed)?;
    let predecessor_transfer = serialize_outcomes(predecessors)?;
    let input = format!("Workflow: bounded-parallel-v1\nConcurrency: same-thread-event-multiplexed; fixture-only; no live access or effect.\nScenario: {:?}\nWork item: {}\nObjective:\n{}\nApplication fixture catalog:\n{}\nValidated predecessor transfer:\n{}\nReturn exactly one strict ParallelAnalysisResultV1 JSON object. Do not include reasoning, URLs, commands, credentials, execution, or authority claims.", request.scenario_id, item.id.as_str(), item.objective, fixtures, predecessor_transfer);
    let selected_bytes = item
        .objective
        .len()
        .checked_add(fixtures.len())
        .and_then(|value| value.checked_add(predecessor_transfer.len()))
        .ok_or(BoundedParallelError::BoundExceeded)?;
    validate_framing_bytes(input.len().saturating_sub(selected_bytes))?;
    if input.len() > MAX_PARALLEL_SYNTHESIS_INPUT_BYTES {
        return Err(BoundedParallelError::BoundExceeded);
    }
    Ok(input)
}

pub(super) fn parse_child_result(
    request: &BoundedParallelWorkflowRequest,
    ordinal: u8,
    raw: &str,
) -> BoundedParallelResult<ParallelChildResult> {
    validate_raw(
        raw,
        MAX_PARALLEL_RAW_RESULT_CHARACTERS,
        MAX_PARALLEL_RAW_RESULT_BYTES,
    )?;
    reject_duplicate_json_keys(raw)?;
    let wire: ChildWire =
        serde_json::from_str(raw).map_err(|_| BoundedParallelError::InvalidStructuredResult)?;
    let item = request.item(ordinal)?;
    if wire.version != "v1"
        || wire.scenario_id != request.scenario_id
        || wire.work_item_id != item.id.as_str()
    {
        return Err(BoundedParallelError::ResultIdentityMismatch);
    }
    if !wire.fixture_based
        || wire.live_access_performed
        || wire.tools_executed
        || wire.credentials_loaded
        || wire.effects_performed
    {
        return Err(BoundedParallelError::AuthorityClaim);
    }
    validate_model_text(&wire.summary)?;
    if wire.findings.is_empty()
        || wire.findings.len() > MAX_PARALLEL_FINDINGS
        || wire.unresolved_issues.len() > MAX_PARALLEL_UNRESOLVED_ISSUES
    {
        return Err(BoundedParallelError::BoundExceeded);
    }
    let mut ids = BTreeSet::new();
    let mut total = wire.summary.len();
    let findings = wire
        .findings
        .into_iter()
        .map(|finding| {
            validate_identifier(&finding.id)?;
            if !ids.insert(finding.id.clone()) {
                return Err(BoundedParallelError::DuplicateReference);
            }
            validate_model_text(&finding.statement)?;
            if finding.references.is_empty()
                || finding.references.len() > MAX_PARALLEL_REFERENCES_PER_FINDING
            {
                return Err(BoundedParallelError::BoundExceeded);
            }
            let mut refs = BTreeSet::new();
            let references = finding
                .references
                .into_iter()
                .map(|value| {
                    if value.namespace != "fixture" || !item.fixture_known(&value.id) {
                        return Err(BoundedParallelError::UnknownFixtureReference);
                    }
                    if !refs.insert(value.id.clone()) {
                        return Err(BoundedParallelError::DuplicateReference);
                    }
                    Ok(ParallelEvidenceRef {
                        namespace: value.namespace,
                        id: value.id,
                    })
                })
                .collect::<BoundedParallelResult<Vec<_>>>()?;
            total = total
                .checked_add(finding.statement.len())
                .ok_or(BoundedParallelError::BoundExceeded)?;
            Ok(ParallelFinding {
                id: finding.id,
                statement: finding.statement,
                confidence: finding.confidence,
                references,
            })
        })
        .collect::<BoundedParallelResult<Vec<_>>>()?;
    for issue in &wire.unresolved_issues {
        validate_model_text(issue)?;
        total = total
            .checked_add(issue.len())
            .ok_or(BoundedParallelError::BoundExceeded)?;
    }
    if total > MAX_PARALLEL_RESULT_TEXT_BYTES {
        return Err(BoundedParallelError::BoundExceeded);
    }
    let mut result = ParallelChildResult {
        work_item_id: item.id.clone(),
        summary: wire.summary,
        findings,
        unresolved_issues: wire.unresolved_issues,
        transfer: String::new(),
    };
    result.transfer = serialize_child_result(item, &result)?;
    Ok(result)
}

fn serialize_child_result(
    item: &ParallelWorkItem,
    result: &ParallelChildResult,
) -> BoundedParallelResult<String> {
    let value = serde_json::to_string(&ChildTransferWire {
        ordinal: item.ordinal,
        work_item_id: item.id.as_str(),
        agent_id: item.agent_id.as_str(),
        status: ParallelChildResultStatus::Succeeded,
        summary: &result.summary,
        findings: result
            .findings
            .iter()
            .map(|f| TransferFindingWire {
                id: &f.id,
                statement: &f.statement,
                confidence: f.confidence,
                references: f
                    .references
                    .iter()
                    .map(|r| TransferEvidenceWire {
                        namespace: &r.namespace,
                        id: &r.id,
                    })
                    .collect(),
            })
            .collect(),
        unresolved_issues: &result.unresolved_issues,
    })
    .map_err(|_| BoundedParallelError::SerializationFailed)?;
    if value.len() > MAX_PARALLEL_CHILD_TRANSFER_BYTES {
        Err(BoundedParallelError::TransferTooLarge)
    } else {
        Ok(value)
    }
}

fn status_rows(outcomes: &[ParallelChildOutcome]) -> Vec<StatusWire> {
    outcomes
        .iter()
        .map(|o| StatusWire {
            ordinal: o.ordinal,
            work_item_id: o.work_item_id.as_str().to_owned(),
            agent_id: o.agent_id.as_str().to_owned(),
            status: o.status(),
        })
        .collect()
}

fn finding_source_rows(outcomes: &[ParallelChildOutcome]) -> Vec<FindingSourceWire> {
    outcomes
        .iter()
        .map(|outcome| FindingSourceWire {
            ordinal: outcome.ordinal,
            work_item_id: outcome.work_item_id.as_str().to_owned(),
            agent_id: outcome.agent_id.as_str().to_owned(),
            finding_ids: match &outcome.disposition {
                ParallelChildDisposition::Succeeded(result) => result
                    .findings
                    .iter()
                    .map(|finding| finding.id.clone())
                    .collect(),
                _ => Vec::new(),
            },
        })
        .collect()
}

fn serialize_outcomes(outcomes: &[ParallelChildOutcome]) -> BoundedParallelResult<String> {
    let mut entries = Vec::with_capacity(outcomes.len());
    for outcome in outcomes {
        match &outcome.disposition {
            ParallelChildDisposition::Succeeded(result) => entries.push(result.transfer.clone()),
            _ => entries.push(
                serde_json::to_string(&StatusWire {
                    ordinal: outcome.ordinal,
                    work_item_id: outcome.work_item_id.as_str().to_owned(),
                    agent_id: outcome.agent_id.as_str().to_owned(),
                    status: outcome.status(),
                })
                .map_err(|_| BoundedParallelError::SerializationFailed)?,
            ),
        }
    }
    let value = format!("[{}]", entries.join(","));
    if value.len() > MAX_PARALLEL_ORDERED_TRANSFER_BYTES {
        Err(BoundedParallelError::TransferTooLarge)
    } else {
        Ok(value)
    }
}

fn expected_status(outcomes: &[ParallelChildOutcome]) -> BoundedParallelSynthesisStatus {
    if outcomes
        .iter()
        .all(|o| o.status() == ParallelChildResultStatus::Succeeded)
    {
        BoundedParallelSynthesisStatus::Complete
    } else {
        BoundedParallelSynthesisStatus::Partial
    }
}

fn expected_issues(outcomes: &[ParallelChildOutcome]) -> Vec<String> {
    let mut issues = Vec::new();
    for outcome in outcomes {
        match &outcome.disposition {
            ParallelChildDisposition::Succeeded(result) => {
                issues.extend(
                    result
                        .unresolved_issues
                        .iter()
                        .map(|issue| format!("{}: {issue}", outcome.work_item_id.as_str())),
                );
            }
            _ => issues.push(format!(
                "{} is {}",
                outcome.work_item_id.as_str(),
                status_label(outcome.status())
            )),
        }
    }
    issues
}

fn validate_expected_issues(issues: &[String]) -> BoundedParallelResult<()> {
    if issues.len() > MAX_PARALLEL_CHILDREN * MAX_PARALLEL_UNRESOLVED_ISSUES {
        return Err(BoundedParallelError::BoundExceeded);
    }
    for issue in issues {
        validate_text(
            issue,
            MAX_PARALLEL_SYNTHESIS_ISSUE_CHARACTERS,
            MAX_PARALLEL_SYNTHESIS_ISSUE_BYTES,
            true,
        )?;
    }
    Ok(())
}

fn status_label(value: ParallelChildResultStatus) -> &'static str {
    match value {
        ParallelChildResultStatus::Succeeded => "succeeded",
        ParallelChildResultStatus::Failed => "failed",
        ParallelChildResultStatus::Cancelled => "cancelled",
        ParallelChildResultStatus::TimedOut => "timed-out",
        ParallelChildResultStatus::Skipped => "skipped",
    }
}

pub(super) fn build_synthesis_input(
    request: &BoundedParallelWorkflowRequest,
    outcomes: &[ParallelChildOutcome],
) -> BoundedParallelResult<String> {
    validate_outcome_order(request, outcomes)?;
    let rows = serde_json::to_string(&status_rows(outcomes))
        .map_err(|_| BoundedParallelError::SerializationFailed)?;
    if rows.len() > MAX_PARALLEL_STATUS_TABLE_BYTES {
        return Err(BoundedParallelError::TransferTooLarge);
    }
    let transfers = outcomes
        .iter()
        .filter_map(|o| match &o.disposition {
            ParallelChildDisposition::Succeeded(result) => Some(result.transfer()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    if transfers.len() > MAX_PARALLEL_ORDERED_TRANSFER_BYTES {
        return Err(BoundedParallelError::TransferTooLarge);
    }
    let finding_rows = serde_json::to_string(&finding_source_rows(outcomes))
        .map_err(|_| BoundedParallelError::SerializationFailed)?;
    let input = format!("Workflow: bounded-parallel-v1\nEvidence disclosure: fixture-only; no live access or effect.\nObjective:\n{}\nApplication-derived ordered status table:\n{}\nApplication-derived ordered finding table:\n{}\nValidated bounded child transfers:\n{}\nReturn exactly one strict synthesis JSON object whose ordered source_results, source_findings, and status match the application tables. Include this disclosure: {}", request.objective, rows, finding_rows, transfers, BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE);
    let selected_bytes = request
        .objective
        .len()
        .checked_add(rows.len())
        .and_then(|value| value.checked_add(finding_rows.len()))
        .and_then(|value| value.checked_add(transfers.len()))
        .ok_or(BoundedParallelError::BoundExceeded)?;
    validate_framing_bytes(input.len().saturating_sub(selected_bytes))?;
    if input.len() > MAX_PARALLEL_SYNTHESIS_INPUT_BYTES {
        Err(BoundedParallelError::TransferTooLarge)
    } else {
        Ok(input)
    }
}

pub(super) fn parse_synthesis(
    request: &BoundedParallelWorkflowRequest,
    outcomes: &[ParallelChildOutcome],
    raw: &str,
) -> BoundedParallelResult<BoundedParallelSynthesis> {
    validate_outcome_order(request, outcomes)?;
    validate_raw(
        raw,
        MAX_PARALLEL_SYNTHESIS_CHARACTERS,
        MAX_PARALLEL_SYNTHESIS_BYTES,
    )?;
    reject_duplicate_json_keys(raw)?;
    let wire: SynthesisWire =
        serde_json::from_str(raw).map_err(|_| BoundedParallelError::InvalidStructuredResult)?;
    if wire.version != "v1"
        || wire.scenario_id != request.scenario_id
        || wire.status != expected_status(outcomes)
    {
        return Err(BoundedParallelError::SynthesisStatusMismatch);
    }
    let expected_rows = status_rows(outcomes);
    if wire.source_results.len() != expected_rows.len()
        || wire
            .source_results
            .iter()
            .zip(expected_rows.iter())
            .any(|(a, b)| {
                a.ordinal != b.ordinal
                    || a.work_item_id != b.work_item_id
                    || a.agent_id != b.agent_id
                    || a.status != b.status
            })
    {
        return Err(BoundedParallelError::SynthesisStatusMismatch);
    }
    let expected_finding_rows = finding_source_rows(outcomes);
    if wire.source_findings.len() != expected_finding_rows.len()
        || wire
            .source_findings
            .iter()
            .zip(expected_finding_rows.iter())
            .any(|(actual, expected)| {
                actual.ordinal != expected.ordinal
                    || actual.work_item_id != expected.work_item_id
                    || actual.agent_id != expected.agent_id
                    || actual.finding_ids != expected.finding_ids
            })
    {
        return Err(BoundedParallelError::SynthesisStatusMismatch);
    }
    if !wire.fixture_based
        || wire.live_access_performed
        || wire.tools_executed
        || wire.credentials_loaded
        || wire.effects_performed
    {
        return Err(BoundedParallelError::AuthorityClaim);
    }
    validate_text(
        &wire.summary,
        MAX_PARALLEL_SYNTHESIS_SUMMARY_CHARACTERS,
        MAX_PARALLEL_SYNTHESIS_SUMMARY_BYTES,
        true,
    )?;
    validate_model_claims(&wire.summary)?;
    let lower = wire.summary.to_ascii_lowercase();
    if !lower.contains(&BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE.to_ascii_lowercase())
        || (wire.status == BoundedParallelSynthesisStatus::Partial
            && !lower.starts_with("partial. "))
    {
        return Err(BoundedParallelError::SynthesisDisclosureMismatch);
    }
    let expected = expected_issues(outcomes);
    validate_expected_issues(&expected)?;
    if wire.unresolved_issues != expected {
        return Err(BoundedParallelError::SynthesisStatusMismatch);
    }
    Ok(BoundedParallelSynthesis {
        status: wire.status,
        source_findings: expected_finding_rows
            .into_iter()
            .zip(outcomes)
            .map(|(row, outcome)| BoundedParallelSynthesisFindingSource {
                ordinal: outcome.ordinal,
                work_item_id: outcome.work_item_id.clone(),
                agent_id: outcome.agent_id,
                finding_ids: row.finding_ids,
            })
            .collect(),
        summary: wire.summary,
        unresolved_issues: wire.unresolved_issues,
    })
}

fn validate_outcome_order(
    request: &BoundedParallelWorkflowRequest,
    outcomes: &[ParallelChildOutcome],
) -> BoundedParallelResult<()> {
    if outcomes.len() != request.work_items.len()
        || outcomes
            .iter()
            .zip(request.work_items.iter())
            .any(|(o, i)| {
                o.ordinal != i.ordinal || o.work_item_id != i.id || o.agent_id != i.agent_id
            })
    {
        Err(BoundedParallelError::SynthesisStatusMismatch)
    } else {
        Ok(())
    }
}

fn validate_raw(raw: &str, chars: usize, bytes: usize) -> BoundedParallelResult<()> {
    if raw.trim().is_empty()
        || raw.trim() != raw
        || raw.chars().count() > chars
        || raw.len() > bytes
        || raw
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    {
        Err(BoundedParallelError::InvalidStructuredResult)
    } else {
        Ok(())
    }
}

fn validate_model_text(value: &str) -> BoundedParallelResult<()> {
    validate_text(
        value,
        MAX_PARALLEL_TEXT_CHARACTERS,
        MAX_PARALLEL_TEXT_BYTES,
        true,
    )?;
    validate_model_claims(value)
}

fn validate_model_claims(value: &str) -> BoundedParallelResult<()> {
    let lower = value.to_ascii_lowercase();
    let tokens = normalized_claim_tokens(value);
    if lower.contains("://") || lower.contains("www.") || contains_forbidden_claim(&tokens) {
        Err(BoundedParallelError::AuthorityClaim)
    } else {
        Ok(())
    }
}

fn normalized_claim_tokens(value: &str) -> Vec<String> {
    let mut token = String::new();
    let mut tokens = Vec::new();
    for character in value.chars() {
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
        &["hidden", "reasoning"],
        &["private", "reasoning"],
        &["internal", "reasoning"],
        &["scratchpad"],
        &["scratch", "pad"],
        &["i", "ran"],
        &["we", "ran"],
        &["i", "executed"],
        &["we", "executed"],
        &["ran", "tests"],
        &["tests", "ran"],
        &["tests", "passed"],
        &["executed", "terraform"],
        &["terraform", "executed"],
        &["command", "ran"],
        &["command", "was", "run"],
        &["commands", "were", "run"],
        &["tool", "executed"],
        &["tools", "executed"],
        &["live", "repository"],
        &["live", "environment"],
        &["production", "environment"],
        &["live", "access", "performed"],
        &["accessed", "live"],
        &["credentials", "loaded"],
        &["credentials", "were", "loaded"],
        &["loaded", "credentials"],
        &["used", "credentials"],
        &["effect", "performed"],
        &["effects", "performed"],
        &["resource", "created"],
        &["resource", "updated"],
        &["resource", "deleted"],
        &["service", "restarted"],
        &["file", "written"],
        &["file", "modified"],
        &["approval", "granted"],
        &["approval", "has", "been", "granted"],
        &["authorization", "granted"],
        &["authorized", "to", "execute"],
    ];
    PHRASES
        .iter()
        .any(|phrase| contains_positive_token_sequence(tokens, phrase))
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

fn reject_duplicate_json_keys(raw: &str) -> BoundedParallelResult<()> {
    let mut deserializer = serde_json::Deserializer::from_str(raw);
    StrictJson::deserialize(&mut deserializer)
        .map_err(|_| BoundedParallelError::InvalidStructuredResult)?;
    deserializer
        .end()
        .map_err(|_| BoundedParallelError::InvalidStructuredResult)
}

struct StrictJson;
impl<'de> Deserialize<'de> for StrictJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StrictJsonVisitor)
    }
}
struct StrictJsonVisitor;
impl<'de> Visitor<'de> for StrictJsonVisitor {
    type Value = StrictJson;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JSON without duplicate object keys")
    }
    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJson)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while seq.next_element::<StrictJson>()?.is_some() {}
        Ok(StrictJson)
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut keys = BTreeSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !keys.insert(key) {
                return Err(serde::de::Error::custom("duplicate key"));
            }
            map.next_value::<StrictJson>()?;
        }
        Ok(StrictJson)
    }
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        StrictJson::deserialize(d)
    }
    fn visit_newtype_struct<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        StrictJson::deserialize(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{
        gateway_protocol::MAX_GATEWAY_REQUEST_BYTES,
        native_runtime::NativeAgentRuntime,
        runtime::{AgentRuntime, RuntimeTurnRequest},
    };
    use serde_json::json;

    fn research_child_wire(work_item_id: &str, fixture_id: &str, summary: &str) -> String {
        json!({
            "version": "v1",
            "scenario_id": "research-knowledge-independent-v1",
            "work_item_id": work_item_id,
            "summary": summary,
            "findings": [{
                "id": "fixture-finding",
                "statement": "The fixture supports this bounded finding.",
                "confidence": "high",
                "references": [{"namespace": "fixture", "id": fixture_id}],
            }],
            "unresolved_issues": [],
            "fixture_based": true,
            "live_access_performed": false,
            "tools_executed": false,
            "credentials_loaded": false,
            "effects_performed": false,
        })
        .to_string()
    }

    fn research_synthesis_wire(summary: &str) -> String {
        let source_findings = [
            (1, "research-analysis", "research", "fixture-finding"),
            (
                2,
                "knowledge-analysis",
                "knowledge-document",
                "fixture-finding",
            ),
        ]
        .map(|(ordinal, work_item_id, agent_id, finding_id)| {
            json!({
                "ordinal": ordinal,
                "work_item_id": work_item_id,
                "agent_id": agent_id,
                "finding_ids": [finding_id],
            })
        });
        json!({
            "version": "v1",
            "scenario_id": "research-knowledge-independent-v1",
            "status": "complete",
            "source_results": [
                {
                    "ordinal": 1,
                    "work_item_id": "research-analysis",
                    "agent_id": "research",
                    "status": "succeeded",
                },
                {
                    "ordinal": 2,
                    "work_item_id": "knowledge-analysis",
                    "agent_id": "knowledge-document",
                    "status": "succeeded",
                },
            ],
            "source_findings": source_findings,
            "summary": summary,
            "unresolved_issues": [],
            "fixture_based": true,
            "live_access_performed": false,
            "tools_executed": false,
            "credentials_loaded": false,
            "effects_performed": false,
        })
        .to_string()
    }

    fn research_child_wire_with_text(
        summary: &str,
        statements: &[String],
        unresolved_issues: &[String],
    ) -> String {
        let findings = statements
            .iter()
            .enumerate()
            .map(|(index, statement)| {
                json!({
                    "id": format!("fixture-finding-{}", index + 1),
                    "statement": statement,
                    "confidence": "high",
                    "references": [{"namespace": "fixture", "id": "approach-a"}],
                })
            })
            .collect::<Vec<_>>();
        json!({
            "version": "v1",
            "scenario_id": "research-knowledge-independent-v1",
            "work_item_id": "research-analysis",
            "summary": summary,
            "findings": findings,
            "unresolved_issues": unresolved_issues,
            "fixture_based": true,
            "live_access_performed": false,
            "tools_executed": false,
            "credentials_loaded": false,
            "effects_performed": false,
        })
        .to_string()
    }

    fn successful_research_outcomes(
        request: &BoundedParallelWorkflowRequest,
    ) -> BoundedParallelResult<Vec<ParallelChildOutcome>> {
        [
            (1, "research-analysis", "approach-a"),
            (2, "knowledge-analysis", "decision-context"),
        ]
        .into_iter()
        .map(|(ordinal, work_item_id, fixture_id)| {
            let item = request.item(ordinal)?;
            let result = request.parse_child_result(
                ordinal,
                &research_child_wire(
                    work_item_id,
                    fixture_id,
                    "Fixture evidence supports this bounded result.",
                ),
            )?;
            Ok(ParallelChildOutcome::new(
                ordinal,
                item.id().clone(),
                item.agent_id(),
                ParallelChildDisposition::Succeeded(result),
            ))
        })
        .collect()
    }

    #[test]
    fn catalog_is_closed_bounded_and_ordered() -> BoundedParallelResult<()> {
        for scenario in [
            BoundedParallelScenarioId::ResearchKnowledgeIndependentV1,
            BoundedParallelScenarioId::CodeSecurityQaV1,
            BoundedParallelScenarioId::CloudSystemsSecurityV1,
        ] {
            let request = BoundedParallelScenarioCatalog::built_in().request(scenario)?;
            assert!((2..=3).contains(&request.work_items().len()));
            for (index, item) in request.work_items().iter().enumerate() {
                assert_eq!(usize::from(item.ordinal()), index + 1);
            }
            assert!(request.build_child_input(1, &[])?.len() <= MAX_PARALLEL_SYNTHESIS_INPUT_BYTES);
        }
        Ok(())
    }

    #[test]
    fn framing_bound_accepts_n_and_rejects_n_plus_one() -> BoundedParallelResult<()> {
        validate_framing_bytes(MAX_PARALLEL_SYNTHESIS_FRAMING_BYTES)?;
        assert_eq!(
            validate_framing_bytes(MAX_PARALLEL_SYNTHESIS_FRAMING_BYTES + 1),
            Err(BoundedParallelError::BoundExceeded)
        );
        Ok(())
    }

    #[test]
    fn request_objective_accepts_exact_scalar_and_byte_bounds_and_rejects_n_plus_one(
    ) -> BoundedParallelResult<()> {
        let base = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        for exact in [
            "x".repeat(MAX_PARALLEL_OBJECTIVE_CHARACTERS),
            "😀".repeat(MAX_PARALLEL_OBJECTIVE_CHARACTERS),
        ] {
            assert!(exact.chars().count() <= MAX_PARALLEL_OBJECTIVE_CHARACTERS);
            assert!(exact.len() <= MAX_PARALLEL_OBJECTIVE_BYTES);
            base.clone()
                .with_objective_for_test(exact)
                .validate_integrity()?;
        }
        assert_eq!(
            base.clone()
                .with_objective_for_test("x".repeat(MAX_PARALLEL_OBJECTIVE_CHARACTERS + 1))
                .validate_integrity(),
            Err(BoundedParallelError::InvalidText)
        );
        assert_eq!(
            base.with_objective_for_test("😀".repeat(MAX_PARALLEL_OBJECTIVE_CHARACTERS + 1))
                .validate_integrity(),
            Err(BoundedParallelError::InvalidText)
        );
        Ok(())
    }

    #[test]
    fn self_dependency_is_rejected_as_an_invalid_catalog() -> BoundedParallelResult<()> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?
            .with_self_dependency_for_test(1);
        assert_eq!(
            request.validate_integrity(),
            Err(BoundedParallelError::InvalidCatalog)
        );
        Ok(())
    }

    #[test]
    fn finding_and_unresolved_text_accept_exact_scalar_and_byte_bounds_then_reject_n_plus_one(
    ) -> BoundedParallelResult<()> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let exact_ascii = "x".repeat(MAX_PARALLEL_TEXT_CHARACTERS);
        let exact_utf8 = "😀".repeat(MAX_PARALLEL_TEXT_CHARACTERS);
        for value in [&exact_ascii, &exact_utf8] {
            assert_eq!(value.chars().count(), MAX_PARALLEL_TEXT_CHARACTERS);
            assert!(value.len() <= MAX_PARALLEL_TEXT_BYTES);
            request.parse_child_result(
                1,
                &research_child_wire_with_text("bounded summary", std::slice::from_ref(value), &[]),
            )?;
            request.parse_child_result(
                1,
                &research_child_wire_with_text(
                    "bounded summary",
                    &["bounded finding".to_owned()],
                    std::slice::from_ref(value),
                ),
            )?;
        }
        for over in [
            "x".repeat(MAX_PARALLEL_TEXT_CHARACTERS + 1),
            "😀".repeat(MAX_PARALLEL_TEXT_CHARACTERS + 1),
        ] {
            assert_eq!(
                request.parse_child_result(
                    1,
                    &research_child_wire_with_text(
                        "bounded summary",
                        std::slice::from_ref(&over),
                        &[],
                    ),
                ),
                Err(BoundedParallelError::InvalidText)
            );
            assert_eq!(
                request.parse_child_result(
                    1,
                    &research_child_wire_with_text(
                        "bounded summary",
                        &["bounded finding".to_owned()],
                        &[over],
                    ),
                ),
                Err(BoundedParallelError::InvalidText)
            );
        }
        Ok(())
    }

    #[test]
    fn finding_count_references_and_aggregate_text_enforce_exact_n_and_n_plus_one(
    ) -> BoundedParallelResult<()> {
        let base = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let exact_findings = vec!["bounded finding".to_owned(); MAX_PARALLEL_FINDINGS];
        base.parse_child_result(
            1,
            &research_child_wire_with_text("bounded summary", &exact_findings, &[]),
        )?;
        let too_many_findings = vec!["bounded finding".to_owned(); MAX_PARALLEL_FINDINGS + 1];
        assert_eq!(
            base.parse_child_result(
                1,
                &research_child_wire_with_text("bounded summary", &too_many_findings, &[]),
            ),
            Err(BoundedParallelError::BoundExceeded)
        );

        let references_request = base
            .clone()
            .with_fixture_catalog_for_test(&[MAX_PARALLEL_REFERENCES_PER_FINDING, 1], "fixture")?;
        references_request.validate_integrity()?;
        let mut reference_wire: serde_json::Value = serde_json::from_str(
            &research_child_wire_with_text("bounded summary", &["bounded finding".to_owned()], &[]),
        )
        .map_err(|_| BoundedParallelError::SerializationFailed)?;
        reference_wire["findings"][0]["references"] = json!((0
            ..MAX_PARALLEL_REFERENCES_PER_FINDING)
            .map(|index| json!({"namespace": "fixture", "id": format!("fixture0-{index}")}))
            .collect::<Vec<_>>());
        references_request.parse_child_result(1, &reference_wire.to_string())?;
        reference_wire["findings"][0]["references"]
            .as_array_mut()
            .ok_or(BoundedParallelError::SerializationFailed)?
            .push(json!({"namespace": "fixture", "id": "fixture0-4"}));
        assert_eq!(
            references_request.parse_child_result(1, &reference_wire.to_string()),
            Err(BoundedParallelError::BoundExceeded)
        );

        let exact_text = "😀".repeat(MAX_PARALLEL_TEXT_CHARACTERS);
        let exact_aggregate = research_child_wire_with_text(
            &exact_text,
            &vec![exact_text.clone(); MAX_PARALLEL_FINDINGS],
            &[],
        );
        base.parse_child_result(1, &exact_aggregate)?;
        let aggregate_over = research_child_wire_with_text(
            &exact_text,
            &vec![exact_text.clone(); MAX_PARALLEL_FINDINGS],
            &["a".to_owned()],
        );
        assert_eq!(
            base.parse_child_result(1, &aggregate_over),
            Err(BoundedParallelError::BoundExceeded)
        );
        Ok(())
    }

    #[test]
    fn child_and_ordered_transfer_serializers_accept_n_and_reject_n_plus_one(
    ) -> BoundedParallelResult<()> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let item = request.item(1)?;
        let mut result = ParallelChildResult {
            work_item_id: item.id().clone(),
            summary: String::new(),
            findings: Vec::new(),
            unresolved_issues: Vec::new(),
            transfer: String::new(),
        };
        let base_len = serialize_child_result(item, &result)?.len();
        result.summary = "x".repeat(
            MAX_PARALLEL_CHILD_TRANSFER_BYTES
                .checked_sub(base_len)
                .ok_or(BoundedParallelError::BoundExceeded)?,
        );
        assert_eq!(
            serialize_child_result(item, &result)?.len(),
            MAX_PARALLEL_CHILD_TRANSFER_BYTES
        );
        result.summary.push('x');
        assert_eq!(
            serialize_child_result(item, &result),
            Err(BoundedParallelError::TransferTooLarge)
        );

        result.transfer = format!(
            "\"{}\"",
            "x".repeat(MAX_PARALLEL_ORDERED_TRANSFER_BYTES - 4)
        );
        let mut outcomes = vec![ParallelChildOutcome::new(
            1,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::Succeeded(result.clone()),
        )];
        assert_eq!(
            serialize_outcomes(&outcomes)?.len(),
            MAX_PARALLEL_ORDERED_TRANSFER_BYTES
        );
        result.transfer = format!(
            "\"{}\"",
            "x".repeat(MAX_PARALLEL_ORDERED_TRANSFER_BYTES - 3)
        );
        outcomes[0] = ParallelChildOutcome::new(
            1,
            item.id().clone(),
            item.agent_id(),
            ParallelChildDisposition::Succeeded(result),
        );
        assert_eq!(
            serialize_outcomes(&outcomes),
            Err(BoundedParallelError::TransferTooLarge)
        );
        Ok(())
    }

    #[test]
    fn application_derived_synthesis_issues_accept_exact_scalar_and_byte_bounds_then_reject_n_plus_one(
    ) -> BoundedParallelResult<()> {
        let exact_characters = "x".repeat(MAX_PARALLEL_SYNTHESIS_ISSUE_CHARACTERS);
        validate_expected_issues(&[exact_characters])?;
        assert_eq!(
            validate_expected_issues(&["x".repeat(MAX_PARALLEL_SYNTHESIS_ISSUE_CHARACTERS + 1)]),
            Err(BoundedParallelError::InvalidText)
        );
        let exact_bytes = format!(
            "{}{}",
            "😀".repeat(MAX_PARALLEL_SYNTHESIS_ISSUE_BYTES / 4),
            "x".repeat(MAX_PARALLEL_SYNTHESIS_ISSUE_BYTES % 4)
        );
        assert_eq!(exact_bytes.len(), MAX_PARALLEL_SYNTHESIS_ISSUE_BYTES);
        validate_expected_issues(std::slice::from_ref(&exact_bytes))?;
        assert_eq!(
            validate_expected_issues(&[format!("{exact_bytes}x")]),
            Err(BoundedParallelError::InvalidText)
        );
        Ok(())
    }

    #[test]
    fn maximum_bounded_child_and_synthesis_inputs_start_through_native_gateway(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::CodeSecurityQaV1)?
            .with_objective_for_test("😀".repeat(MAX_PARALLEL_OBJECTIVE_CHARACTERS))
            .with_fixture_catalog_for_test(&[3, 3, 2], &"é".repeat(800))?;
        request.validate_integrity()?;

        for item in request.work_items() {
            if item.dependencies().is_empty() {
                let input = request.build_child_input(item.ordinal(), &[])?;
                NativeAgentRuntime.start(RuntimeTurnRequest::new(
                    format!("parallel-gateway-child-{}", item.ordinal()),
                    format!("parallel-gateway-request-{}", item.ordinal()),
                    input,
                )?)?;
            }
        }
        let mut outcomes = [
            (1, "coding-review", "fixture0-0"),
            (2, "security-review", "fixture1-0"),
        ]
        .into_iter()
        .map(|(ordinal, work_item_id, fixture_id)| {
            let item = request.item(ordinal)?;
            let raw = json!({
                "version": "v1",
                "scenario_id": "code-security-qa-v1",
                "work_item_id": work_item_id,
                "summary": "bounded summary",
                "findings": [{
                    "id": "bounded-finding",
                    "statement": "bounded finding",
                    "confidence": "high",
                    "references": [{"namespace": "fixture", "id": fixture_id}],
                }],
                "unresolved_issues": [],
                "fixture_based": true,
                "live_access_performed": false,
                "tools_executed": false,
                "credentials_loaded": false,
                "effects_performed": false,
            })
            .to_string();
            Ok(ParallelChildOutcome::new(
                ordinal,
                item.id().clone(),
                item.agent_id(),
                ParallelChildDisposition::Succeeded(request.parse_child_result(ordinal, &raw)?),
            ))
        })
        .collect::<BoundedParallelResult<Vec<_>>>()?;
        let dependent_input = request.build_child_input(3, &outcomes)?;
        NativeAgentRuntime.start(RuntimeTurnRequest::new(
            "parallel-gateway-dependent",
            "parallel-gateway-dependent-request",
            dependent_input,
        )?)?;
        let dependent = request.item(3)?;
        let dependent_raw = json!({
            "version": "v1",
            "scenario_id": "code-security-qa-v1",
            "work_item_id": "qa-validation",
            "summary": "bounded dependent summary",
            "findings": [{
                "id": "bounded-dependent-finding",
                "statement": "bounded dependent finding",
                "confidence": "high",
                "references": [{"namespace": "fixture", "id": "fixture2-0"}],
            }],
            "unresolved_issues": [],
            "fixture_based": true,
            "live_access_performed": false,
            "tools_executed": false,
            "credentials_loaded": false,
            "effects_performed": false,
        })
        .to_string();
        outcomes.push(ParallelChildOutcome::new(
            3,
            dependent.id().clone(),
            dependent.agent_id(),
            ParallelChildDisposition::Succeeded(request.parse_child_result(3, &dependent_raw)?),
        ));
        let synthesis_input = request.build_synthesis_input(&outcomes)?;
        let synthesis_run = NativeAgentRuntime.start(RuntimeTurnRequest::new(
            "parallel-gateway-synthesis",
            "parallel-gateway-synthesis-request",
            synthesis_input,
        )?)?;
        assert!(synthesis_run.request_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);

        for (index, selected_text) in [
            "\"".repeat(MAX_PARALLEL_SYNTHESIS_INPUT_BYTES),
            "\\".repeat(MAX_PARALLEL_SYNTHESIS_INPUT_BYTES),
            format!("x{}", "\n".repeat(MAX_PARALLEL_SYNTHESIS_INPUT_BYTES - 1)),
            "😀".repeat(MAX_PARALLEL_SYNTHESIS_INPUT_BYTES / 4),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(selected_text.len(), MAX_PARALLEL_SYNTHESIS_INPUT_BYTES);
            let run = NativeAgentRuntime.start(RuntimeTurnRequest::new(
                format!("parallel-gateway-max-{index}"),
                format!("parallel-gateway-max-request-{index}"),
                selected_text,
            )?)?;
            assert!(run.request_bytes().len() <= MAX_GATEWAY_REQUEST_BYTES);
        }
        Ok(())
    }

    #[test]
    fn strict_result_rejects_duplicate_and_authority_claims() -> BoundedParallelResult<()> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let duplicate = r#"{"version":"v1","version":"v1"}"#;
        assert_eq!(
            request.parse_child_result(1, duplicate),
            Err(BoundedParallelError::InvalidStructuredResult)
        );
        let claim = r#"{"version":"v1","scenario_id":"research-knowledge-independent-v1","work_item_id":"research-analysis","summary":"tool executed","findings":[{"id":"finding-1","statement":"fixture finding","confidence":"high","references":[{"namespace":"fixture","id":"approach-a"}]}],"unresolved_issues":[],"fixture_based":true,"live_access_performed":false,"tools_executed":false,"credentials_loaded":false,"effects_performed":false}"#;
        assert_eq!(
            request.parse_child_result(1, claim),
            Err(BoundedParallelError::AuthorityClaim)
        );
        Ok(())
    }

    #[test]
    fn child_and_synthesis_text_reject_positive_authority_claims_but_allow_clear_negations(
    ) -> BoundedParallelResult<()> {
        let request = BoundedParallelScenarioCatalog::built_in()
            .request(BoundedParallelScenarioId::ResearchKnowledgeIndependentV1)?;
        let outcomes = successful_research_outcomes(&request)?;
        let positive_claims = [
            "Fixture review accessed live material.",
            "Fixture review reports a tool executed.",
            "Fixture review used credentials.",
            "Fixture review reports a resource updated.",
            "Fixture review says authorization granted.",
        ];

        for claim in positive_claims {
            assert_eq!(
                request.parse_child_result(
                    1,
                    &research_child_wire("research-analysis", "approach-a", claim),
                ),
                Err(BoundedParallelError::AuthorityClaim)
            );
            assert_eq!(
                request.parse_synthesis(&outcomes, &research_synthesis_wire(claim)),
                Err(BoundedParallelError::AuthorityClaim)
            );
        }

        let negated = BOUNDED_PARALLEL_SYNTHESIS_DISCLOSURE;
        request.parse_child_result(
            1,
            &research_child_wire("research-analysis", "approach-a", negated),
        )?;
        request.parse_synthesis(&outcomes, &research_synthesis_wire(negated))?;
        assert_eq!(
            request.parse_synthesis(
                &outcomes,
                &research_synthesis_wire("Fixture review completed without effects."),
            ),
            Err(BoundedParallelError::SynthesisDisclosureMismatch)
        );
        Ok(())
    }

    #[test]
    fn aggregate_fixture_count_is_eight_and_serialized_catalog_accounts_for_escaping_and_utf8(
    ) -> BoundedParallelResult<()> {
        fn serialized_catalog_len(
            request: &BoundedParallelWorkflowRequest,
        ) -> BoundedParallelResult<usize> {
            request.work_items.iter().try_fold(0usize, |total, item| {
                let serialized = serde_json::to_vec(
                    &item
                        .fixtures
                        .iter()
                        .map(|fixture| FixtureWire {
                            id: &fixture.id,
                            label: &fixture.label,
                            content: &fixture.content,
                        })
                        .collect::<Vec<_>>(),
                )
                .map_err(|_| BoundedParallelError::SerializationFailed)?;
                total
                    .checked_add(serialized.len())
                    .ok_or(BoundedParallelError::BoundExceeded)
            })
        }

        let catalog = BoundedParallelScenarioCatalog::built_in();
        let base = catalog.request(BoundedParallelScenarioId::CodeSecurityQaV1)?;
        let exact_count = base
            .clone()
            .with_fixture_catalog_for_test(&[3, 3, 2], "a")?;
        exact_count.validate_integrity()?;
        let too_many = base
            .clone()
            .with_fixture_catalog_for_test(&[3, 3, 3], "fixture")?;
        assert_eq!(
            too_many.validate_integrity(),
            Err(BoundedParallelError::BoundExceeded)
        );

        let mut exact_serialized = base
            .clone()
            .with_fixture_catalog_for_test(&[3, 3, 2], "a")?;
        let mut remaining = MAX_PARALLEL_CATALOG_BYTES
            .checked_sub(serialized_catalog_len(&exact_serialized)?)
            .ok_or(BoundedParallelError::BoundExceeded)?;
        for fixture in exact_serialized
            .work_items
            .iter_mut()
            .flat_map(|item| item.fixtures.iter_mut())
        {
            if remaining == 0 {
                break;
            }
            let increment = remaining.min(4_095);
            let target_payload = increment + 1;
            fixture.content = format!(
                "{}{}",
                "\"".repeat(target_payload / 2),
                "a".repeat(target_payload % 2)
            );
            remaining -= increment;
        }
        assert_eq!(remaining, 0);
        assert_eq!(
            serialized_catalog_len(&exact_serialized)?,
            MAX_PARALLEL_CATALOG_BYTES
        );
        exact_serialized.validate_integrity()?;
        let mut serialized_over = exact_serialized.clone();
        serialized_over
            .work_items
            .last_mut()
            .and_then(|item| item.fixtures.last_mut())
            .ok_or(BoundedParallelError::InvalidCatalog)?
            .content
            .push('a');
        assert_eq!(
            serialized_catalog_len(&serialized_over)?,
            MAX_PARALLEL_CATALOG_BYTES + 1
        );
        assert_eq!(
            serialized_over.validate_integrity(),
            Err(BoundedParallelError::BoundExceeded)
        );
        let multibyte = "é";
        let exact_utf8 = base
            .clone()
            .with_fixture_catalog_for_test(&[1, 1, 1], &multibyte.repeat(2_048))?;
        exact_utf8.validate_integrity()?;
        let mut utf8_over = exact_utf8.clone();
        utf8_over.work_items[0].fixtures[0].content.push('é');
        assert_eq!(
            utf8_over.validate_integrity(),
            Err(BoundedParallelError::BoundExceeded)
        );
        Ok(())
    }
}
