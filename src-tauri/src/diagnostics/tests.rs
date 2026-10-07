use super::*;
use crate::personal_assistant_direct::ProviderEvent;

#[test]
fn streaming_terminal_and_late_events_are_bounded_and_content_free(
) -> Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let log = Logger::open(d.path().join("logs"));
    let mut a = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::OpenaiApi,
        "gpt-5.6-luna",
        Some("agent-chat-1"),
        None,
        None,
    );
    let o = a.observer();
    o.first(false);
    for _ in 0..1000 {
        o.first(true);
    }
    a.finish(Ok(()));
    o.first(true);
    a.finish(Err(DirectError::Network));
    drop(a);
    assert!(log.flush(false));
    let snapshot = log.snapshot();
    assert!(snapshot.available);
    assert_eq!(snapshot.events.len(), 4);
    assert_eq!(
        snapshot.events.iter().map(|r| r.event).collect::<Vec<_>>(),
        vec![
            Event::RequestStarted,
            Event::FirstResponse,
            Event::FirstText,
            Event::RequestFinished
        ]
    );
    assert!(snapshot
        .events
        .windows(2)
        .all(|r| r[0].duration_ms <= r[1].duration_ms));
    assert!(snapshot
        .events
        .iter()
        .all(|r| r.attempt == snapshot.events[0].attempt
            && r.conversation == snapshot.events[0].conversation));
    assert_eq!(snapshot.events[3].outcome, Outcome::Completed);
    assert_eq!(
        read_records(&d.path().join("logs/diagnostics-0.jsonl"))?.len(),
        4
    );
    assert!(log.flush(true));
    Ok(())
}
#[test]
fn errors_cancel_timeout_restart_rotation_and_export() -> Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let root = d.path().join("logs");
    let log = Logger::with_limit(root.clone(), 2500);
    for error in [
        DirectError::Authentication,
        DirectError::HttpForbidden,
        DirectError::RateLimited,
        DirectError::Timeout,
        DirectError::Protocol,
        DirectError::CodexRuntime,
    ] {
        let mut a = Attempt::with_logger(
            Some(log.clone()),
            AgentConnection::Codex,
            "SECRET_MODEL_CANARY",
            None,
            Some("room-1/run-1"),
            Some("stage-1"),
        );
        a.finish(Err(error));
        assert!(log.flush(false));
    }
    let a = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::Simulation,
        "simulation",
        None,
        None,
        None,
    );
    drop(a);
    assert!(log.flush(false));
    let snapshot = log.snapshot();
    assert_eq!(
        snapshot.events.last().map(|r| r.outcome),
        Some(Outcome::Cancelled)
    );
    assert!(snapshot
        .events
        .iter()
        .any(|r| r.error == Some(Category::DeniedAccess)));
    assert!(snapshot
        .events
        .iter()
        .any(|r| r.outcome == Outcome::TimedOut));
    for p in fs::read_dir(&root)? {
        let p = p?;
        assert!(p.metadata()?.len() <= 2500);
        assert!(!fs::read_to_string(p.path())?.contains("SECRET_MODEL_CANARY"));
    }
    assert_eq!(fs::read_dir(&root)?.count(), 3);
    let destination = d.path().join("diagnostics.json");
    export_new(&destination, &snapshot)?;
    let before = fs::read(&destination)?;
    assert!(!String::from_utf8_lossy(&before).contains("SECRET_MODEL_CANARY"));
    assert!(export_new(&destination, &snapshot).is_err());
    assert_eq!(fs::read(&destination)?, before);
    assert!(log.flush(true));
    let restarted = Logger::open(root);
    assert!(restarted.snapshot().available);
    assert!(!restarted.snapshot().events.is_empty());
    assert!(restarted.flush(true));
    Ok(())
}
#[test]
fn unavailable_storage_and_tampered_records_are_not_exposed(
) -> Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let p = d.path().join("not-directory");
    fs::write(&p, "preserved")?;
    let unavailable = Logger::open(p.clone());
    unavailable.event(
        Event::StorageFailure,
        Outcome::Failed,
        Some(Category::Storage),
    );
    assert!(!unavailable.snapshot().available);
    assert_eq!(fs::read_to_string(p)?, "preserved");
    let root = d.path().join("logs");
    fs::create_dir(&root)?;
    fs::write(
        root.join("diagnostics-0.jsonl"),
        "{\"secret\":\"PRIVATE_CONTENT_CANARY\"}\n",
    )?;
    let log = Logger::open(root);
    assert!(!log.snapshot().available);
    assert!(log.snapshot().events.is_empty());
    Ok(())
}
#[test]
#[cfg(unix)]
fn symlinks_hardlinks_and_permissions() -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let d = tempfile::tempdir()?;
    let external = d.path().join("preserved");
    fs::write(&external, "canary")?;
    let root = d.path().join("logs");
    fs::create_dir(&root)?;
    symlink(&external, root.join("diagnostics-0.jsonl"))?;
    assert!(!Logger::open(root).snapshot().available);
    assert_eq!(fs::read_to_string(&external)?, "canary");
    let hard = d.path().join("hard");
    fs::hard_link(&external, &hard)?;
    assert!(open_file(&hard, true).is_err());
    let root = d.path().join("safe");
    let log = Logger::open(root.clone());
    log.event(Event::ApplicationStarted, Outcome::Observed, None);
    assert!(log.flush(false));
    assert_eq!(fs::metadata(&root)?.permissions().mode() & 0o777, 0o700);
    assert_eq!(
        fs::metadata(root.join("diagnostics-0.jsonl"))?
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    assert!(log.flush(true));
    Ok(())
}
#[test]
fn actual_adapter_events_keep_payloads_out_of_diagnostics() -> Result<(), Box<dyn std::error::Error>>
{
    let d = tempfile::tempdir()?;
    let log = Logger::open(d.path().join("logs"));
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let mut a = Attempt::with_logger(
                Some(log.clone()),
                AgentConnection::Simulation,
                "simulation",
                Some("conversation"),
                None,
                None,
            );
            let observer = a.observer();
            let adapter = crate::agent_adapter::AdapterRequest::Events(vec![
                ProviderEvent::Started("SECRET_ID_CANARY".into()),
                ProviderEvent::Delta("PRIVATE_TEXT_CANARY".into()),
                ProviderEvent::Completed,
            ]);
            let result =
                crate::agent_adapter::run_adapter_traced(adapter, observer.clone(), |event| {
                    observer.first(matches!(event, ProviderEvent::Delta(_)));
                    Ok(())
                })
                .await;
            a.finish(result);
            Ok::<(), DirectError>(())
        })?;
    assert!(log.flush(false));
    let data = fs::read_to_string(d.path().join("logs/diagnostics-0.jsonl"))?;
    assert!(!data.contains("SECRET_ID_CANARY"));
    assert!(!data.contains("PRIVATE_TEXT_CANARY"));
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn abort_join_releases_future_before_one_cancel_terminal_and_rejects_late_events(
) -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::atomic::AtomicBool;
    let d = tempfile::tempdir()?;
    let log = Logger::open(d.path().join("logs"));
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let a = Attempt::with_logger(
                Some(log.clone()),
                AgentConnection::Simulation,
                "simulation",
                Some("private-id-canary"),
                None,
                None,
            );
            let late = a.observer();
            let observer = late.clone();
            let dropped = Arc::new(AtomicBool::new(false));
            let inner = Arc::clone(&dropped);
            let task = tokio::spawn(async move {
                let _attempt = a;
                crate::agent_adapter::run_adapter_traced(
                    crate::agent_adapter::AdapterRequest::Pending(inner),
                    observer,
                    |_| Ok(()),
                )
                .await
            });
            tokio::task::yield_now().await;
            task.abort();
            assert!(task.await.is_err());
            assert!(dropped.load(Ordering::SeqCst));
            late.first(true);
            late.record(Event::RuntimeStarted, Outcome::Observed, None);
            let records = log.snapshot().events;
            assert_eq!(
                records.iter().map(|r| r.event).collect::<Vec<_>>(),
                vec![
                    Event::RequestStarted,
                    Event::ProviderDispatch,
                    Event::TransportReleased,
                    Event::RequestFinished
                ]
            );
            assert_eq!(records[3].outcome, Outcome::Cancelled);
            assert!(records[0].attempt.is_none());
            assert!(records[1].attempt.is_some());
            assert!(records.iter().all(|r| r.request == records[0].request));
            Ok::<(), Box<dyn std::error::Error>>(())
        })?;
    assert!(log.flush(true));
    Ok(())
}
#[test]
fn queue_saturation_is_nonfatal_and_memory_is_bounded() {
    let (tx, _rx) = mpsc::sync_channel(1);
    let log = Logger {
        shared: Arc::new(Shared {
            available: AtomicBool::new(true),
            recent: Mutex::new(VecDeque::new()),
        }),
        tx,
        session: identifier(),
    };
    for _ in 0..1000 {
        log.event(Event::ApplicationStarted, Outcome::Observed, None);
    }
    assert!(!log.snapshot().available);
    assert_eq!(log.snapshot().events.len(), 256);
}
#[test]
fn observed_response_timing_precedes_text_and_transport_error_is_preserved(
) -> Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let log = Logger::open(d.path().join("logs"));
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let mut a = Attempt::with_logger(
                Some(log.clone()),
                AgentConnection::OpenaiApi,
                "gpt-5.6-luna",
                Some("conversation"),
                None,
                None,
            );
            let observer = a.observer();
            CURRENT
                .scope(observer.clone(), async {
                    response_received(); // same hook called immediately after HTTP headers
                    tokio::time::sleep(Duration::from_millis(2)).await;
                    observer.first(true);
                })
                .await;
            a.finish(Err(DirectError::ProviderStreamErrorEvent));
        });
    let s = log.snapshot();
    assert_eq!(s.events[1].event, Event::FirstResponse);
    assert_eq!(s.events[2].event, Event::FirstText);
    assert!(s.events[2].duration_ms > s.events[1].duration_ms);
    assert_eq!(s.events[3].error, Some(Category::StreamError));
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn writer_failure_does_not_change_result_and_stage_partial_is_not_success(
) -> Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let root = d.path().join("logs");
    let log = Logger::open(root.clone());
    log.event(Event::ApplicationStarted, Outcome::Observed, None);
    assert!(log.flush(false));
    fs::rename(
        root.join("diagnostics-0.jsonl"),
        root.join("preserved.jsonl"),
    )?;
    fs::create_dir(root.join("diagnostics-0.jsonl"))?;
    let mut a = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::Simulation,
        "simulation",
        None,
        Some("run-1"),
        Some("stage-1"),
    );
    a.partial();
    drop(a);
    assert!(log.flush(false));
    assert!(!log.snapshot().available);
    let s = log.snapshot();
    assert_eq!(s.events.last().map(|r| r.outcome), Some(Outcome::Partial));
    assert_eq!(
        s.events
            .iter()
            .filter(|r| r.event == Event::StageFinished)
            .count(),
        1
    );
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn observed_host_timeout_survives_abort_cleanup() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let log = Logger::open(directory.path().join("logs"));
    let attempt = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::Simulation,
        "simulation",
        None,
        None,
        None,
    );
    let observer = attempt.observer();
    observer.aborting_with(DirectError::Timeout);
    observer.record(Event::RuntimeCleanup, Outcome::Completed, None);
    drop(attempt);
    observer.first(true);
    let events = log.snapshot().events;
    assert_eq!(events.len(), 3);
    assert_eq!(events[2].outcome, Outcome::TimedOut);
    assert_eq!(events[2].error, Some(Category::Timeout));
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn top_level_error_observation_is_correlated_once_and_keeps_original_terminal_error(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::personal_assistant_direct::{TopLevelErrorCode, TopLevelErrorParam};
    let d = tempfile::tempdir()?;
    let log = Logger::open(d.path().join("logs"));
    let mut attempt = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::OpenaiApi,
        "gpt-5.6-luna",
        Some("DUMMY-CONVERSATION-CANARY"),
        Some("DUMMY-WORKFLOW-CANARY"),
        Some("DUMMY-STAGE-CANARY"),
    );
    let observer = attempt.observer();
    tokio::runtime::Builder::new_current_thread()
        .build()?
        .block_on(CURRENT.scope(observer.clone(), async {
            response_received();
            top_level_error(TopLevelErrorCode::ServerError, TopLevelErrorParam::Null);
            top_level_error(TopLevelErrorCode::InvalidPrompt, TopLevelErrorParam::Null);
        }));
    attempt.finish(Err(DirectError::ProviderStreamErrorEvent));
    observer.top_level_error(
        TopLevelErrorCode::RateLimitExceeded,
        TopLevelErrorParam::Null,
    );
    observer.first(true);
    attempt.finish(Ok(()));
    drop(attempt);
    let records = log.snapshot().events;
    assert_eq!(
        records.iter().map(|r| r.event).collect::<Vec<_>>(),
        vec![
            Event::RequestStarted,
            Event::StageStarted,
            Event::FirstResponse,
            Event::OpenaiTopLevelError,
            Event::OpenaiTopLevelErrorParam,
            Event::RequestFinished,
            Event::StageFinished,
        ]
    );
    assert!(records.iter().all(|r| r.attempt == records[0].attempt
        && r.conversation == records[0].conversation
        && r.workflow == records[0].workflow
        && r.stage == records[0].stage
        && r.provider == Some(AgentConnection::OpenaiApi)));
    assert_eq!(records[3].outcome, Outcome::Observed);
    assert_eq!(records[3].error, Some(Category::OpenaiTopLevelServerError));
    assert_eq!(records[4].error, Some(Category::OpenaiTopLevelParamNull));
    assert_eq!(records[5].outcome, Outcome::Failed);
    assert_eq!(records[6].error, Some(Category::StreamError));
    assert_eq!(records[5].error, Some(Category::StreamError));
    assert!(records
        .windows(2)
        .all(|pair| pair[0].duration_ms <= pair[1].duration_ms));
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn top_level_error_ignores_other_providers_and_observation_after_terminal(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::personal_assistant_direct::{TopLevelErrorCode, TopLevelErrorParam};
    let d = tempfile::tempdir()?;
    let log = Logger::open(d.path().join("logs"));
    for provider in [
        AgentConnection::Simulation,
        AgentConnection::AnthropicApi,
        AgentConnection::LmStudio,
        AgentConnection::Ollama,
        AgentConnection::Codex,
    ] {
        let mut attempt =
            Attempt::with_logger(Some(log.clone()), provider, "simulation", None, None, None);
        attempt
            .observer()
            .top_level_error(TopLevelErrorCode::ServerError, TopLevelErrorParam::Null);
        attempt.finish(Ok(()));
    }
    let mut finished = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::OpenaiApi,
        "gpt-5.6-luna",
        None,
        None,
        None,
    );
    finished.finish(Err(DirectError::ProviderStreamErrorEvent));
    finished
        .observer()
        .top_level_error(TopLevelErrorCode::Unknown, TopLevelErrorParam::Null);
    let records = log.snapshot().events;
    assert_eq!(records.len(), 12);
    assert!(records.iter().all(
        |r| r.event != Event::OpenaiTopLevelError && r.event != Event::OpenaiTopLevelErrorParam
    ));
    assert_eq!(records[11].error, Some(Category::StreamError));
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn top_level_error_without_current_or_available_logger_is_nonfatal(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::personal_assistant_direct::{TopLevelErrorCode, TopLevelErrorParam};
    assert!(current().is_none());
    top_level_error(TopLevelErrorCode::Absent, TopLevelErrorParam::Null);
    let d = tempfile::tempdir()?;
    let blocked = d.path().join("not-a-log-directory");
    fs::write(&blocked, "DUMMY-PRESERVED-CANARY")?;
    let unavailable = Logger::open(blocked.clone());
    assert!(!unavailable.snapshot().available);
    for logger in [None, Some(unavailable.clone())] {
        let mut attempt = Attempt::with_logger(
            logger,
            AgentConnection::OpenaiApi,
            "gpt-5.6-luna",
            None,
            None,
            None,
        );
        let observer = attempt.observer();
        tokio::runtime::Builder::new_current_thread()
            .build()?
            .block_on(CURRENT.scope(observer, async {
                top_level_error(TopLevelErrorCode::Invalid, TopLevelErrorParam::Null);
            }));
        attempt.finish(Err(DirectError::ProviderStreamErrorEvent));
    }
    let snapshot = unavailable.snapshot();
    assert!(!snapshot.available);
    assert_eq!(snapshot.events.len(), 4);
    assert_eq!(snapshot.events[1].event, Event::OpenaiTopLevelError);
    assert_eq!(
        snapshot.events[1].error,
        Some(Category::OpenaiTopLevelInvalidCode)
    );
    assert_eq!(snapshot.events[3].outcome, Outcome::Failed);
    assert_eq!(snapshot.events[3].error, Some(Category::StreamError));
    assert_eq!(fs::read_to_string(blocked)?, "DUMMY-PRESERVED-CANARY");
    Ok(())
}

#[test]
fn top_level_error_buckets_survive_disk_restart_and_export_without_content(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::personal_assistant_direct::{TopLevelErrorCode, TopLevelErrorParam};
    let d = tempfile::tempdir()?;
    let root = d.path().join("logs");
    let log = Logger::open(root.clone());
    let cases = [
        (
            TopLevelErrorCode::ServerError,
            Category::OpenaiTopLevelServerError,
        ),
        (
            TopLevelErrorCode::RateLimitExceeded,
            Category::OpenaiTopLevelRateLimitExceeded,
        ),
        (
            TopLevelErrorCode::InvalidPrompt,
            Category::OpenaiTopLevelInvalidPrompt,
        ),
        (
            TopLevelErrorCode::Unknown,
            Category::OpenaiTopLevelUnknownCode,
        ),
        (
            TopLevelErrorCode::Absent,
            Category::OpenaiTopLevelAbsentCode,
        ),
        (
            TopLevelErrorCode::CreditBalanceExhausted,
            Category::OpenaiCreditBalanceExhausted,
        ),
        (TopLevelErrorCode::Null, Category::OpenaiTopLevelNullCode),
        (
            TopLevelErrorCode::Invalid,
            Category::OpenaiTopLevelInvalidCode,
        ),
    ];
    for (code, _) in cases {
        let mut attempt = Attempt::with_logger(
            Some(log.clone()),
            AgentConnection::OpenaiApi,
            "DUMMY-MODEL-CANARY",
            Some("DUMMY-CONVERSATION-CANARY"),
            None,
            None,
        );
        attempt
            .observer()
            .top_level_error(code, TopLevelErrorParam::Null);
        attempt.finish(Err(DirectError::ProviderStreamErrorEvent));
    }
    assert!(log.flush(true));
    let disk = fs::read_to_string(root.join("diagnostics-0.jsonl"))?;
    assert!(!disk.contains("DUMMY-"));
    let restarted = Logger::open(root);
    let snapshot = restarted.snapshot();
    assert!(snapshot.available);
    assert_eq!(snapshot.events.len(), cases.len() * 4);
    assert_eq!(
        snapshot
            .events
            .iter()
            .filter(|r| r.event == Event::OpenaiTopLevelError)
            .map(|r| r.error)
            .collect::<Vec<_>>(),
        cases
            .iter()
            .map(|(_, category)| Some(*category))
            .collect::<Vec<_>>()
    );
    let destination = d.path().join("safe-export.json");
    export_new(&destination, &snapshot)?;
    let exported = fs::read_to_string(destination)?;
    assert!(!exported.contains("DUMMY-"));
    for record in &snapshot.events {
        assert!(record.valid());
        let encoded = serde_json::to_value(record)?;
        assert_eq!(encoded.as_object().ok_or("expected record")?.len(), 15);
        assert!(encoded.get("message").is_none());
        assert!(encoded.get("param").is_none());
        assert!(encoded.get("code").is_none());
    }
    assert!(restarted.flush(true));
    Ok(())
}

#[test]
fn top_level_error_diagnostic_enums_reject_arbitrary_strings_and_nested_payloads(
) -> Result<(), Box<dyn std::error::Error>> {
    for value in [
        serde_json::json!("DUMMY-UNKNOWN-CODE-CANARY"),
        serde_json::json!({"code":"server_error","message":"DUMMY-MESSAGE-CANARY"}),
        serde_json::json!({"openai_top_level_server_error":"DUMMY-MESSAGE-CANARY"}),
        serde_json::json!(null),
    ] {
        assert!(serde_json::from_value::<Category>(value.clone()).is_err());
        assert!(serde_json::from_value::<Event>(value).is_err());
    }
    assert_eq!(
        serde_json::to_value(Event::OpenaiTopLevelError)?,
        serde_json::json!("openai_top_level_error")
    );
    for category in [
        Category::OpenaiTopLevelServerError,
        Category::OpenaiTopLevelRateLimitExceeded,
        Category::OpenaiTopLevelInvalidPrompt,
        Category::OpenaiTopLevelUnknownCode,
        Category::OpenaiTopLevelMissingCode,
        Category::OpenaiTopLevelInvalidCode,
    ] {
        let encoded = serde_json::to_value(category)?;
        assert_eq!(serde_json::from_value::<Category>(encoded)?, category);
    }
    Ok(())
}

#[test]
fn top_level_error_legacy_missing_category_remains_readable(
) -> Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let root = d.path().join("logs");
    let log = Logger::open(root.clone());
    let record = log.blank(
        Event::OpenaiTopLevelError,
        Outcome::Observed,
        Some(Category::OpenaiTopLevelMissingCode),
    );
    log.record_event(record);
    assert!(log.flush(true));
    let restarted = Logger::open(root);
    let snapshot = restarted.snapshot();
    assert_eq!(snapshot.events.len(), 1);
    assert_eq!(
        snapshot.events[0].error,
        Some(Category::OpenaiTopLevelMissingCode)
    );
    let exported = d.path().join("legacy.json");
    export_new(&exported, &snapshot)?;
    let text = fs::read_to_string(exported)?;
    assert!(text.contains("openai_top_level_missing_code"));
    assert!(!text.contains("openai_top_level_absent_code"));
    assert!(!text.contains("openai_top_level_null_code"));
    assert!(restarted.flush(true));
    Ok(())
}

#[test]
fn request_preparation_dispatch_receipt_and_interruption_are_truthful(
) -> Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let root = d.path().join("logs");
    let log = Logger::open(root.clone());
    let mut rejected = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::OpenaiApi,
        "gpt-5.6-luna",
        None,
        None,
        None,
    );
    rejected.fail_preparation(crate::agent_chat::ChatError::Provider(
        DirectError::MissingKey,
    ));
    assert!(log.snapshot().events.iter().all(|r| r.attempt.is_none()));
    assert_eq!(
        log.snapshot().events.last().and_then(|r| r.error),
        Some(Category::MissingCredentials)
    );
    let mut a = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::Simulation,
        "simulation",
        None,
        Some("DUMMY-RUN"),
        Some("DUMMY-STAGE"),
    );
    let observer = a.observer();
    let id = observer.request_id().ok_or("request")?;
    observer.ui_returned(&id); // before terminal: ignored
    observer.dispatch();
    observer.dispatch();
    observer.first(true);
    a.finish(Ok(()));
    observer.ui_returned("DUMMY-INJECTED-ID");
    observer.ui_returned(&id);
    observer.ui_returned(&id);
    let records = log.snapshot().events;
    assert_eq!(
        records
            .iter()
            .filter(|r| r.event == Event::ProviderDispatch)
            .count(),
        1
    );
    assert_eq!(
        records
            .iter()
            .filter(|r| r.event == Event::UiTerminalReturned)
            .count(),
        1
    );
    assert_eq!(
        records
            .iter()
            .filter(|r| r.request.as_deref() == Some(&id) && r.event == Event::RequestFinished)
            .count(),
        1
    );
    assert_ne!(records.last().and_then(|r| r.attempt.as_ref()), Some(&id));
    // Simulate a prior process record whose owner vanished without a terminal write.
    let pending = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::OpenaiApi,
        "gpt-5.6-luna",
        None,
        None,
        None,
    );
    let pending_id = pending.observer().request_id();
    assert!(log.flush(true));
    let reopened = Logger::open(root.clone());
    let snapshot = reopened.snapshot();
    let interrupted: Vec<_> = snapshot
        .events
        .iter()
        .filter(|r| r.event == Event::RequestInterrupted)
        .collect();
    assert_eq!(interrupted.len(), 1);
    assert_eq!(interrupted[0].request, pending_id);
    assert_eq!(interrupted[0].outcome, Outcome::Interrupted);
    assert_eq!(interrupted[0].duration_ms, None);
    assert!(reopened.flush(true));
    let again = Logger::open(root);
    assert_eq!(
        again
            .snapshot()
            .events
            .iter()
            .filter(|r| r.event == Event::RequestInterrupted)
            .count(),
        1
    );
    let export = d.path().join("export.json");
    export_new(&export, &again.snapshot())?;
    assert!(!fs::read_to_string(export)?.contains("DUMMY-"));
    assert!(again.flush(true));
    drop(pending);
    Ok(())
}

#[test]
fn precise_categories_do_not_guess_http_resource_or_provider_cause() {
    assert_eq!(
        Category::from(DirectError::MissingKey),
        Category::MissingCredentials
    );
    assert_eq!(
        Category::from(DirectError::ModelUnavailable),
        Category::ModelUnavailable
    );
    assert_eq!(
        Category::from(DirectError::HttpNotFound),
        Category::Configuration
    );
    assert_eq!(Category::from(DirectError::Network), Category::Network);
    assert_eq!(
        Category::from(DirectError::CodexSetup),
        Category::RuntimeLaunch
    );
}

#[test]
fn traced_adapter_marks_only_accepted_events_and_stops_at_callback_error(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::agent_adapter::{run_adapter_traced, AdapterRequest};
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            for reject_at in [None, Some(0), Some(2)] {
                let directory = tempfile::tempdir()?;
                let log = Logger::open(directory.path().join("logs"));
                let mut attempt = Attempt::with_logger(
                    Some(log.clone()),
                    AgentConnection::Simulation,
                    "simulation",
                    None,
                    None,
                    None,
                );
                let mut delivered = 0;
                let result = run_adapter_traced(
                    AdapterRequest::Events(vec![
                        ProviderEvent::Started("resp_fixture".into()),
                        ProviderEvent::Delta(String::new()),
                        ProviderEvent::Delta("Synthetic text".into()),
                        ProviderEvent::Completed,
                    ]),
                    attempt.observer(),
                    |_| {
                        // Each callback precedes diagnostic acceptance of that event.
                        let records = log.snapshot().events;
                        assert_eq!(
                            records
                                .iter()
                                .filter(|r| r.event == Event::FirstResponse)
                                .count(),
                            usize::from(delivered > 0)
                        );
                        assert_eq!(
                            records
                                .iter()
                                .filter(|r| r.event == Event::FirstText)
                                .count(),
                            usize::from(delivered > 2)
                        );
                        let reject = reject_at == Some(delivered);
                        delivered += 1;
                        if reject {
                            Err(DirectError::InvalidRequest)
                        } else {
                            Ok(())
                        }
                    },
                )
                .await;
                assert_eq!(
                    result,
                    reject_at.map_or(Ok(()), |_| Err(DirectError::InvalidRequest))
                );
                assert_eq!(delivered, reject_at.map_or(4, |index| index + 1));
                let records = log.snapshot().events;
                assert_eq!(
                    records
                        .iter()
                        .filter(|r| r.event == Event::FirstResponse)
                        .count(),
                    usize::from(reject_at != Some(0))
                );
                assert_eq!(
                    records
                        .iter()
                        .filter(|r| r.event == Event::FirstText)
                        .count(),
                    usize::from(reject_at.is_none())
                );
                assert_eq!(
                    records.last().map(|r| r.event),
                    Some(Event::TransportReleased)
                );
                assert!(!records.iter().any(|r| r.event == Event::RequestFinished));
                attempt.finish(result);
                assert!(log.flush(true));
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        })
}

#[test]
fn traced_adapter_keeps_interleaved_task_local_observers_separate(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::agent_adapter::{run_adapter_traced, AdapterRequest};
    let directory = tempfile::tempdir()?;
    let log = Logger::open(directory.path().join("logs"));
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let mut left = Attempt::with_logger(
                Some(log.clone()),
                AgentConnection::Simulation,
                "simulation",
                Some("left-conversation"),
                None,
                None,
            );
            let mut right = Attempt::with_logger(
                Some(log.clone()),
                AgentConnection::Simulation,
                "simulation",
                None,
                Some("right-workflow"),
                Some("right-stage"),
            );
            let outer = Attempt::with_logger(
                None,
                AgentConnection::Simulation,
                "simulation",
                None,
                None,
                None,
            );
            let left_id = left.observer().request_id();
            let right_id = right.observer().request_id();
            let outer_id = outer.observer().request_id();
            assert!(left_id.is_some() && right_id.is_some());
            assert_ne!(left_id, right_id);
            let deliveries = std::cell::RefCell::new(Vec::new());
            assert!(current().is_none());
            let (left_result, right_result) = CURRENT
                .scope(outer.observer(), async {
                    let mut left_run = std::pin::pin!(run_adapter_traced(
                        AdapterRequest::Simulation,
                        left.observer(),
                        |event| {
                            let active = current().ok_or(DirectError::Internal)?;
                            assert_eq!(active.request_id(), left_id);
                            active.record(Event::RuntimeStarted, Outcome::Observed, None);
                            deliveries
                                .borrow_mut()
                                .push(("left", matches!(event, ProviderEvent::Started(_))));
                            Ok(())
                        }
                    ));
                    let mut right_run = std::pin::pin!(run_adapter_traced(
                        AdapterRequest::Simulation,
                        right.observer(),
                        |event| {
                            let active = current().ok_or(DirectError::Internal)?;
                            assert_eq!(active.request_id(), right_id);
                            active.record(Event::RuntimeStarted, Outcome::Observed, None);
                            deliveries
                                .borrow_mut()
                                .push(("right", matches!(event, ProviderEvent::Started(_))));
                            Ok(())
                        }
                    ));
                    // Poll both existing adapter futures; no Tokio macros feature is needed.
                    let mut left_result = None;
                    let mut right_result = None;
                    let results = std::future::poll_fn(|context| {
                        if left_result.is_none() {
                            if let std::task::Poll::Ready(result) =
                                std::future::Future::poll(left_run.as_mut(), context)
                            {
                                left_result = Some(result);
                            }
                        }
                        assert_eq!(current().and_then(|o| o.request_id()), outer_id);
                        if right_result.is_none() {
                            if let std::task::Poll::Ready(result) =
                                std::future::Future::poll(right_run.as_mut(), context)
                            {
                                right_result = Some(result);
                            }
                        }
                        assert_eq!(current().and_then(|o| o.request_id()), outer_id);
                        match (left_result, right_result) {
                            (Some(left), Some(right)) => std::task::Poll::Ready((left, right)),
                            _ => std::task::Poll::Pending,
                        }
                    })
                    .await;
                    assert_eq!(current().and_then(|o| o.request_id()), outer_id);
                    results
                })
                .await;
            assert!(current().is_none());
            assert_eq!(left_result, Ok(()));
            assert_eq!(right_result, Ok(()));
            // Both transports began before either yielded its first delta.
            let deliveries = deliveries.into_inner();
            assert!(deliveries.len() >= 2);
            assert!(deliveries[0].1 && deliveries[1].1);
            assert_ne!(deliveries[0].0, deliveries[1].0);
            left.finish(left_result);
            right.finish(right_result);
            let records = log.snapshot().events;
            for id in [&left_id, &right_id] {
                let correlated = records
                    .iter()
                    .filter(|r| &r.request == id)
                    .collect::<Vec<_>>();
                let initial = correlated.first().ok_or("missing request records")?;
                assert!(correlated
                    .iter()
                    .all(|r| r.conversation == initial.conversation
                        && r.workflow == initial.workflow
                        && r.stage == initial.stage));
                assert_eq!(
                    correlated
                        .iter()
                        .filter(|r| r.event == Event::RuntimeStarted)
                        .count(),
                    1
                );
                assert_eq!(
                    correlated
                        .iter()
                        .filter(|r| r.event == Event::ProviderDispatch)
                        .count(),
                    1
                );
                assert_eq!(
                    correlated
                        .iter()
                        .filter(|r| r.event == Event::TransportReleased)
                        .count(),
                    1
                );
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        })?;
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn traced_adapter_failure_returns_before_fresh_attempt_without_fallback(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::agent_adapter::{run_adapter_traced, AdapterRequest};
    let directory = tempfile::tempdir()?;
    let log = Logger::open(directory.path().join("logs"));
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let mut failed = Attempt::with_logger(
                Some(log.clone()),
                AgentConnection::Simulation,
                "simulation",
                None,
                None,
                None,
            );
            let failed_id = failed.observer().request_id();
            let mut delivered = 0;
            let failure = run_adapter_traced(
                AdapterRequest::Failure(DirectError::Network),
                failed.observer(),
                |_| {
                    delivered += 1;
                    Ok(())
                },
            )
            .await;
            assert_eq!(failure, Err(DirectError::Network));
            assert_eq!(delivered, 0);
            assert_eq!(
                log.snapshot()
                    .events
                    .iter()
                    .map(|r| r.event)
                    .collect::<Vec<_>>(),
                vec![
                    Event::RequestStarted,
                    Event::ProviderDispatch,
                    Event::TransportReleased
                ]
            );
            // The caller, not the adapter wrapper, owns terminal disposition.
            failed.finish(failure);
            let mut fresh = Attempt::with_logger(
                Some(log.clone()),
                AgentConnection::Simulation,
                "simulation",
                None,
                None,
                None,
            );
            let fresh_id = fresh.observer().request_id();
            assert_ne!(failed_id, fresh_id);
            let success = run_adapter_traced(
                AdapterRequest::Events(vec![
                    ProviderEvent::Started("resp_fresh_fixture".into()),
                    ProviderEvent::Delta("Synthetic text".into()),
                    ProviderEvent::Completed,
                ]),
                fresh.observer(),
                |_| {
                    delivered += 1;
                    Ok(())
                },
            )
            .await;
            assert_eq!(success, Ok(()));
            assert_eq!(delivered, 3);
            let before_finish = log.snapshot().events;
            assert_eq!(
                before_finish.last().map(|r| r.event),
                Some(Event::TransportReleased)
            );
            assert!(!before_finish
                .iter()
                .any(|r| r.request == fresh_id && r.event == Event::RequestFinished));
            fresh.finish(success);
            let records = log.snapshot().events;
            let dispatches = records
                .iter()
                .filter(|r| r.event == Event::ProviderDispatch)
                .collect::<Vec<_>>();
            assert_eq!(dispatches.len(), 2);
            assert!(dispatches.iter().all(|r| r.attempt.is_some()));
            assert_ne!(dispatches[0].attempt, dispatches[1].attempt);
            let terminals = records
                .iter()
                .filter(|r| r.event == Event::RequestFinished)
                .collect::<Vec<_>>();
            assert_eq!(terminals.len(), 2);
            assert_eq!(terminals[0].request, failed_id);
            assert_eq!(terminals[0].outcome, Outcome::Failed);
            assert_eq!(terminals[0].error, Some(Category::Network));
            assert_eq!(terminals[1].request, fresh_id);
            assert_eq!(terminals[1].outcome, Outcome::Completed);
            assert_eq!(terminals[1].error, None);
        });
    assert!(log.flush(true));
    Ok(())
}

#[test]
fn nested_error_observation_requires_openai_root_error_and_stops_at_terminal(
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::personal_assistant_direct::{ErrorEnvelope, TopLevelErrorCode, TopLevelErrorParam};
    for provider in [AgentConnection::OpenaiApi, AgentConnection::Codex] {
        let mut a = Attempt::with_logger(None, provider, "gpt-5.6-luna", None, None, None);
        let o = a.observer();
        let nested =
            ErrorEnvelope::Object(TopLevelErrorCode::ModelNotFound, TopLevelErrorParam::Model);
        o.error_envelope(nested);
        assert!(
            !o.0.lock()
                .map_err(|_| "observer lock poisoned")?
                .error_envelope
        );
        o.top_level_error(TopLevelErrorCode::Absent, TopLevelErrorParam::Absent);
        o.error_envelope(nested);
        o.error_envelope(nested);
        assert_eq!(
            o.0.lock()
                .map_err(|_| "observer lock poisoned")?
                .error_envelope,
            provider == AgentConnection::OpenaiApi
        );
        a.finish(Err(DirectError::ProviderStreamErrorEvent));
        let before =
            o.0.lock()
                .map_err(|_| "observer lock poisoned")?
                .phases
                .clone();
        o.error_envelope(nested);
        assert_eq!(
            o.0.lock().map_err(|_| "observer lock poisoned")?.phases,
            before
        );
    }
    Ok(())
}

#[test]
fn credit_balance_terminal_category_is_closed_and_serializable(
) -> Result<(), Box<dyn std::error::Error>> {
    let error = DirectError::ProviderStreamCreditBalanceExhausted;
    assert_eq!(
        Category::from(error),
        Category::OpenaiCreditBalanceExhausted
    );
    assert_eq!(
        serde_json::to_value(error)?,
        serde_json::json!("provider_stream_credit_balance_exhausted")
    );
    let category = serde_json::to_value(Category::from(error))?;
    assert_eq!(
        category,
        serde_json::json!("openai_credit_balance_exhausted")
    );
    assert_eq!(
        serde_json::from_value::<Category>(category)?,
        Category::OpenaiCreditBalanceExhausted
    );
    for value in [
        serde_json::json!("openai_credit_balance_exhausted PRIVATE_ERROR_CANARY"),
        serde_json::json!({"code":"openai_credit_balance_exhausted"}),
    ] {
        assert!(serde_json::from_value::<Category>(value).is_err());
    }
    Ok(())
}

#[test]
fn local_limit_is_not_configuration_or_malformed_stream() -> Result<(), Box<dyn std::error::Error>>
{
    let d = tempfile::tempdir()?;
    let log = Logger::open(d.path().join("logs"));
    let mut attempt = Attempt::with_logger(
        Some(log.clone()),
        AgentConnection::Simulation,
        "simulation",
        None,
        None,
        None,
    );
    let observer = attempt.observer();
    observer.first(true);
    attempt.finish(Err(DirectError::Limit));
    // Late abort/completion observations cannot overwrite the terminal reason.
    observer.aborting_with(DirectError::Protocol);
    attempt.finish(Ok(()));
    drop(attempt);
    assert!(log.flush(false));
    let snapshot = log.snapshot();
    let terminal: Vec<_> = snapshot
        .events
        .iter()
        .filter(|r| r.event == Event::RequestFinished)
        .collect();
    assert_eq!(terminal.len(), 1);
    assert_eq!(terminal[0].outcome, Outcome::Failed);
    assert_eq!(terminal[0].error, Some(Category::ResourceLimit));
    assert_eq!(
        Category::from(DirectError::Protocol),
        Category::MalformedStream
    );
    assert_eq!(
        Category::from(DirectError::Incomplete),
        Category::Incomplete
    );
    Ok(())
}
