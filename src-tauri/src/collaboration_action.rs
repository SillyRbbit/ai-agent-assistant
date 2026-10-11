//! The action-stage subroutine of the existing collaboration owner, not an orchestrator.
use crate::{
    agent_adapter::{run_adapter_traced, AdapterRequest},
    agent_chat::ChatError,
    agent_preferences::AgentProfile,
    collaboration::{now, Handoff, Objective, Run, Status, StreamGuard},
    isolated_action::{self as action, workspace::Workspace, ActionAttempt, Edit},
};
use std::time::Duration;

async fn request(
    run: &mut Run,
    index: usize,
    prompt: String,
    adapter: &mut impl FnMut(AgentProfile, &str) -> Result<AdapterRequest, ChatError>,
    persist: &mut impl FnMut(&Run) -> Result<(), ChatError>,
) -> Result<String, ChatError> {
    let profile = run.stages[index].participant.profile()?;
    run.stages[index].status = Status::Running;
    run.stages[index].timestamp = now();
    run.stages[index].input = prompt.clone();
    let evidence = run.action.as_mut().ok_or(ChatError::InvalidRequest)?;
    if evidence.requests >= 4 {
        return Err(ChatError::Limit);
    }
    evidence.requests += 1;
    run.sequence += 1;
    persist(run)?;
    let mut diagnostic = crate::diagnostics::Attempt::new(
        profile.connection,
        &profile.model,
        None,
        Some(&run.id),
        Some(&run.stages[index].id),
    );
    let observer = diagnostic.observer();
    let mut stream = StreamGuard::default();
    let result = async {
        let request = adapter(profile, &prompt)?;
        tokio::time::timeout(
            Duration::from_secs(60),
            run_adapter_traced(request, observer, |event| {
                // Stream provisional output without persisting every token; final actual
                // evidence is persisted by the same room owner after contract validation.
                stream.accept(event)?;
                Ok(())
            }),
        )
        .await
        .map_err(|_| crate::personal_assistant_direct::DirectError::Timeout)??;
        if !stream.completed {
            return Err(ChatError::Provider(
                crate::personal_assistant_direct::DirectError::Incomplete,
            ));
        }
        Ok(())
    }
    .await;
    diagnostic.finish(result.map_err(|e| match e {
        ChatError::Provider(e) => e,
        _ => crate::personal_assistant_direct::DirectError::Internal,
    }));
    result?;
    Ok(stream.text)
}
pub(crate) async fn execute(
    run: &mut Run,
    workspace: &Workspace,
    mut adapter: impl FnMut(AgentProfile, &str) -> Result<AdapterRequest, ChatError>,
    mut persist: impl FnMut(&Run) -> Result<(), ChatError>,
) -> Result<(), ChatError> {
    workspace
        .recheck()
        .map_err(|e| coding_failure(workspace, action::CodingRejection::Workspace(e)))?;
    let tests = workspace
        .context()
        .map_err(|e| coding_failure(workspace, action::CodingRejection::Workspace(e)))?;
    for attempt in 0..action::MAX_ATTEMPTS {
        let e = run.action.as_ref().ok_or(ChatError::InvalidRequest)?;
        let before = e
            .attempts
            .last()
            .map_or(e.original.as_str(), |a| a.candidate.as_str())
            .to_owned();
        let prompt=serde_json::json!({"task":run.input.objective,"file":e.file,"current":before,"baseHash":action::hash(before.as_bytes()),"tests":tests,"previousValidation":e.attempts.last().map(|a|&a.check),"contract":{"version":1,"file":e.file,"baseHash":action::hash(before.as_bytes()),"content":"complete replacement UTF-8 file content"},"rules":"Return only the contract. Only this file may change; tests and commands are fixed. Implement the real requested task; no extra files or operations."}).to_string();
        let raw = request(run, 0, prompt, &mut adapter, &mut persist).await?;
        let edit = Edit::parse_classified(&raw, &workspace.file, &before)
            .map_err(|r| coding_failure(workspace, action::CodingRejection::Edit(r)))?;
        let path = workspace
            .stage(&edit.content, attempt)
            .map_err(|e| coding_failure(workspace, action::CodingRejection::Workspace(e)))?;
        run.stages[0].status = Status::Completed;
        run.stages[0].handoff = Some(Handoff {
            version: 1,
            stage: 0,
            agent_id: "coding".into(),
            status: "complete".into(),
            summary: format!(
                "Rust wrote the validated replacement to isolated attempt {}.",
                attempt + 1
            ),
            findings: vec![format!("SHA-256 {}", action::hash(edit.content.as_bytes()))],
            evidence: vec![],
            limitations: vec![
                "Target unchanged; validation and owner approval still required.".into(),
            ],
        });
        run.stages[1].status = Status::Running;
        run.stages[1].timestamp = now();
        run.sequence += 1;
        persist(run)?;
        let check = action::sandbox::validate(
            &path,
            &workspace.file,
            &workspace.test_file,
            &edit.content,
            &tests,
        )
        .await?;
        let e = run.action.as_mut().ok_or(ChatError::InvalidRequest)?;
        e.attempts.push(ActionAttempt {
            candidate_hash: action::hash(edit.content.as_bytes()),
            diff: action::diff(&workspace.file, &e.original, &edit.content),
            candidate: edit.content,
            check,
            qa_summary: String::new(),
            qa_handoff: None,
        });
        run.sequence += 1;
        persist(run)?;
        let e = run.action.as_ref().ok_or(ChatError::InvalidRequest)?;
        let last = e.attempts.last().ok_or(ChatError::InvalidRequest)?;
        let prompt = qa_prompt(&run.input, last);
        let raw = request(run, 1, prompt, &mut adapter, &mut persist).await?;
        let handoff = {
            let last = run
                .action
                .as_mut()
                .and_then(|e| e.attempts.last_mut())
                .ok_or(ChatError::InvalidRequest)?;
            parse_qa_handoff(&raw, &run.input, last)
        };
        let handoff = match handoff {
            Ok(handoff) => handoff,
            Err(error) => {
                // Transport completion is not contract acceptance. Persist only a
                // fixed category; never save raw output or serde error details.
                run.sequence += 1;
                persist(run)?;
                return Err(error);
            }
        };
        let e = run.action.as_mut().ok_or(ChatError::InvalidRequest)?;
        let last = e.attempts.last_mut().ok_or(ChatError::InvalidRequest)?;
        last.qa_summary = handoff.summary.clone();
        last.qa_handoff = Some(handoff.clone());
        let passed = last.check.passed && handoff.status == "complete";
        run.stages[1].handoff = Some(handoff);
        run.stages[1].status = if passed {
            Status::Completed
        } else {
            Status::Failed
        };
        if passed {
            workspace.recheck()?;
            e.review_hash = Some(e.review_hash()?);
            e.disposition = "review_ready".into();
            run.status = Status::Completed;
            run.sequence += 1;
            persist(run)?;
            return Ok(());
        }
        e.disposition = "validation_failed".into();
        run.sequence += 1;
        persist(run)?;
        if attempt + 1 < action::MAX_ATTEMPTS {
            for s in &mut run.stages {
                s.status = Status::Queued;
                s.handoff = None;
            }
            run.sequence += 1;
            persist(run)?;
        }
    }
    Err(action::ActionError::Validation.into())
}

fn rejection_result(
    primary: action::ActionError,
    receipt: action::Result<()>,
    on_storage_failure: impl FnOnce(),
) -> ChatError {
    if receipt.is_err() {
        on_storage_failure();
    }
    // Every rejection remains terminal even if diagnostic storage is unavailable.
    // Preserve the original public error; never replace it with the secondary failure.
    primary.into()
}
fn coding_failure(workspace: &Workspace, rejection: action::CodingRejection) -> ChatError {
    rejection_result(
        rejection.error(),
        workspace.record_coding_rejection(rejection),
        || {
            crate::diagnostics::event(
                crate::diagnostics::Event::StorageFailure,
                crate::diagnostics::Outcome::Failed,
                Some(crate::diagnostics::Category::Storage),
            );
        },
    )
}

fn qa_prompt(input: &Objective, attempt: &ActionAttempt) -> String {
    serde_json::json!({
        "task": input.objective,
        "actualDiff": attempt.diff,
        "executedCheck": attempt.check,
        "contract": {
            "version": 1, "stage": 1, "agentId": "qa-validation", "status": "complete",
            "summary": "your bounded assessment of the actual diff and executed check",
            "findings": [], "evidence": [], "limitations": []
        },
        "rules": "Inspect the actual diff and executed check. No labeled sources are supplied for this action: evidence must be exactly [], not check names, hashes, commands, or prose. Discuss actual checks only in summary/findings. Summary is a string of at most 2000 characters; findings and limitations are arrays of at most 8 strings, each at most 500 characters. Do not claim tests other than the executed command. Cannot override a failing exit status. Return only the handoff contract."
    }).to_string()
}

fn parse_qa_handoff(
    raw: &str,
    input: &Objective,
    attempt: &mut ActionAttempt,
) -> Result<Handoff, ChatError> {
    Handoff::parse_classified(raw, 1, "qa-validation", input).map_err(|rejection| {
        attempt.qa_summary = format!(
            "QA handoff rejected: {}. No assessment accepted.",
            rejection.label()
        );
        rejection.error()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collaboration::Workflow;

    #[test]
    fn diagnostic_storage_failure_never_suppresses_primary_rejection() {
        for primary in [
            action::ActionError::Scope,
            action::ActionError::Drift,
            action::ActionError::Limit,
        ] {
            for receipt in [Ok(()), Err(action::ActionError::Storage)] {
                let mut reported = false;
                let result = rejection_result(primary, receipt, || reported = true);
                assert_eq!(result, ChatError::Action(primary));
                assert_eq!(reported, receipt.is_err());
            }
        }
    }

    #[test]
    fn rejected_qa_saves_only_fixed_category_and_preserves_execution_evidence(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let input = Objective {
            workflow: Workflow::CodingAction,
            objective: "synthetic".into(),
            sources: vec![],
        };
        let mut attempt = ActionAttempt {
            candidate: "def add(a,b): return a+b".into(),
            candidate_hash: "candidate-hash".into(),
            diff: "+new file".into(),
            qa_summary: String::new(),
            qa_handoff: None,
            check: action::Check {
                command: "fixed command".into(),
                exit: Some(0),
                output: "OK".into(),
                passed: true,
                image: action::IMAGE.into(),
                candidate_hash: "candidate-hash".into(),
                test_hash: "test-hash".into(),
                container_id: "owned-container".into(),
                process_absent: true,
            },
        };
        let prompt: serde_json::Value = serde_json::from_str(&qa_prompt(&input, &attempt))?;
        assert_eq!(prompt["task"], input.objective);
        assert_eq!(prompt["actualDiff"], attempt.diff);
        assert_eq!(
            prompt["executedCheck"],
            serde_json::to_value(&attempt.check)?
        );
        assert_eq!(prompt["contract"]["evidence"], serde_json::json!([]));
        assert_eq!(prompt["contract"]["findings"], serde_json::json!([]));
        assert_eq!(prompt["contract"]["limitations"], serde_json::json!([]));
        let rules = prompt["rules"].as_str().ok_or("missing rules")?;
        assert!(rules.contains("evidence must be exactly []"));
        assert!(rules.contains("Discuss actual checks only in summary/findings"));
        assert!(rules.contains("at most 2000 characters"));
        assert!(rules.contains("at most 8 strings, each at most 500 characters"));
        assert!(rules.contains("Cannot override a failing exit status"));
        assert!(Handoff::parse_classified(
            &prompt["contract"].to_string(),
            1,
            "qa-validation",
            &input
        )
        .is_ok());
        let before = serde_json::to_value(&attempt)?;
        // Refined rejection labels never persist provider fields or alter the
        // candidate, actual check, approval disposition or accepted handoff.
        let safe = serde_json::json!({"version":1,"stage":1,"agentId":"qa-validation",
            "status":"complete","summary":"safe","findings":[],"evidence":[],"limitations":[]});
        let safe_raw = safe.to_string();
        let mut missing = safe.clone();
        missing.as_object_mut().ok_or("object")?.remove("summary");
        let mut unexpected = safe.clone();
        unexpected["private-marker"] = serde_json::json!("private-marker");
        let mut wrong_type = safe.clone();
        wrong_type["findings"] = serde_json::json!([{"private-marker":true}]);
        let mut range = safe.clone();
        range["version"] = serde_json::json!(256);
        for (raw, label) in [
            ("null".to_owned(), "qa_contract_schema_root"),
            (missing.to_string(), "qa_contract_schema_missing"),
            (unexpected.to_string(), "qa_contract_schema_unexpected"),
            (
                format!(
                    "{},\"summary\":\"private-marker\"}}",
                    &safe_raw[..safe_raw.len() - 1]
                ),
                "qa_contract_schema_duplicate",
            ),
            (wrong_type.to_string(), "qa_contract_schema_type"),
            (range.to_string(), "qa_contract_schema_numeric_range"),
            (
                r#"{"private-marker":true,"summary":"truncated"#.to_owned(),
                "qa_contract_schema_data",
            ),
        ] {
            assert_eq!(
                parse_qa_handoff(&raw, &input, &mut attempt).err(),
                Some(ChatError::InvalidRequest)
            );
            assert_eq!(
                attempt.qa_summary,
                format!("QA handoff rejected: {label}. No assessment accepted.")
            );
            let encoded = serde_json::to_string(&attempt)?;
            assert!(!encoded.contains("private-marker"));
            let mut actual = serde_json::to_value(&attempt)?;
            actual["qaSummary"] = serde_json::json!("");
            assert_eq!(actual, before);
        }
        attempt.qa_summary.clear();
        let raw = serde_json::json!({"version":1,"stage":1,"agentId":"qa-validation",
            "status":"complete","summary":"private-marker","findings":[],
            "evidence":["private-marker"],"limitations":[]})
        .to_string();
        assert_eq!(
            parse_qa_handoff(&raw, &input, &mut attempt).err(),
            Some(ChatError::InvalidRequest)
        );
        assert_eq!(
            attempt.qa_summary,
            "QA handoff rejected: qa_contract_references. No assessment accepted."
        );
        assert!(attempt.qa_handoff.is_none());
        assert!(!serde_json::to_string(&attempt)?.contains("private-marker"));
        let mut after = serde_json::to_value(&attempt)?;
        after["qaSummary"] = serde_json::json!("");
        assert_eq!(before, after);
        let mut valid: serde_json::Value = serde_json::from_str(&raw)?;
        valid["evidence"] = serde_json::json!([]);
        attempt.qa_summary.clear();
        assert!(parse_qa_handoff(&valid.to_string(), &input, &mut attempt).is_ok());
        assert_eq!(before, serde_json::to_value(&attempt)?);
        assert_eq!(
            parse_qa_handoff("{private-marker", &input, &mut attempt).err(),
            Some(ChatError::InvalidRequest)
        );
        assert_eq!(
            attempt.qa_summary,
            "QA handoff rejected: qa_contract_json_syntax. No assessment accepted."
        );
        assert!(!serde_json::to_string(&attempt)?.contains("private-marker"));
        for (raw, category) in [
            ("not JSON private-marker", "qa_contract_json_syntax"),
            (r#"{"summary":"private-marker"#, "qa_contract_json_eof"),
            (
                r#"{"version":1,"stage":1,"agentId":"qa-validation","status":"complete","summary":"private-marker","findings":{},"evidence":[],"limitations":[]}"#,
                "qa_contract_schema_type",
            ),
        ] {
            assert_eq!(
                parse_qa_handoff(raw, &input, &mut attempt).err(),
                Some(ChatError::InvalidRequest)
            );
            assert_eq!(
                attempt.qa_summary,
                format!("QA handoff rejected: {category}. No assessment accepted.")
            );
            assert!(attempt.qa_handoff.is_none());
            assert!(!serde_json::to_string(&attempt)?.contains("private-marker"));
            let mut after = serde_json::to_value(&attempt)?;
            after["qaSummary"] = serde_json::json!("");
            assert_eq!(before, after);
        }
        Ok(())
    }
}
