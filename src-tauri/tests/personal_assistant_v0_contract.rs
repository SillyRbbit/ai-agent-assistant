use std::error::Error;
use std::sync::Mutex;

use ai_agent_assistant_lib::{
    PersonalAssistantV0Error, PersonalAssistantV0Host, PersonalAssistantV0Snapshot,
    PersonalAssistantV0Update,
};

static TEST_SERIAL: Mutex<()> = Mutex::new(());

#[test]
fn public_host_exposes_exact_handle_poll_cancel_and_restart_contract() -> Result<(), Box<dyn Error>>
{
    let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
    let mut host = PersonalAssistantV0Host::new();

    let first = host.start_synthetic()?;
    let first_handle = first.presentation_handle().clone();
    assert_eq!(first.snapshot(), &PersonalAssistantV0Snapshot::Starting);
    assert_eq!(first.snapshot().sequence(), 0);
    assert!(first.snapshot().accepted_text().is_empty());
    assert!(first_handle.as_str().starts_with("pa-v0-present-"));
    assert!(first_handle.as_str().len() <= 128);
    assert!(first_handle
        .as_str()
        .bytes()
        .all(|byte| (0x21..=0x7e).contains(&byte)));
    assert_eq!(host.start_synthetic(), Err(PersonalAssistantV0Error::Busy));

    let from_zero = host.updates(&first_handle, Some(0))?;
    assert!(from_zero.updates().is_empty());
    assert_eq!(from_zero.through_sequence(), 0);
    assert!(!from_zero.has_more());
    assert_eq!(from_zero.snapshot(), &PersonalAssistantV0Snapshot::Starting);

    let recovery = host.updates(&first_handle, None)?;
    assert!(recovery.updates().is_empty());
    assert_eq!(recovery.through_sequence(), 0);
    assert!(!recovery.has_more());
    assert_eq!(
        host.updates(&first_handle, Some(1)),
        Err(PersonalAssistantV0Error::InvalidRequest)
    );
    assert_eq!(
        host.updates(&first_handle, Some(129)),
        Err(PersonalAssistantV0Error::InvalidRequest)
    );

    let cancelled = host.cancel(&first_handle)?;
    assert!(matches!(
        cancelled,
        PersonalAssistantV0Snapshot::Cancelled(_)
    ));
    assert_eq!(cancelled.sequence(), 1);
    assert!(cancelled.accepted_text().is_empty());
    assert!(cancelled.is_terminal());
    assert_eq!(host.cancel(&first_handle)?, cancelled);

    let cancellation_page = host.updates(&first_handle, Some(0))?;
    assert_eq!(cancellation_page.updates().len(), 1);
    assert!(matches!(
        cancellation_page.updates(),
        [PersonalAssistantV0Update::Cancelled(update)] if update.sequence() == 1
    ));
    assert_eq!(cancellation_page.through_sequence(), 1);
    assert!(!cancellation_page.has_more());
    assert!(host.updates(&first_handle, Some(1))?.updates().is_empty());

    let second = host.start_synthetic()?;
    let second_handle = second.presentation_handle().clone();
    assert_ne!(second_handle, first_handle);
    assert_eq!(
        host.snapshot(&first_handle),
        Err(PersonalAssistantV0Error::InvalidHandle)
    );
    assert_eq!(
        host.updates(&first_handle, Some(0)),
        Err(PersonalAssistantV0Error::InvalidHandle)
    );
    assert_eq!(
        host.cancel(&first_handle),
        Err(PersonalAssistantV0Error::InvalidHandle)
    );
    assert_eq!(
        host.snapshot(&second_handle)?,
        PersonalAssistantV0Snapshot::Starting
    );
    host.cancel(&second_handle)?;
    Ok(())
}

#[test]
fn process_lease_denies_cross_host_start_until_proved_cleanup_or_drop() -> Result<(), Box<dyn Error>>
{
    let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
    let mut first = PersonalAssistantV0Host::default();
    let mut second = PersonalAssistantV0Host::default();

    let first_start = first.start_synthetic()?;
    assert_eq!(
        second.start_synthetic(),
        Err(PersonalAssistantV0Error::Busy)
    );
    first.cancel(first_start.presentation_handle())?;

    let second_start = second.start_synthetic()?;
    second.cancel(second_start.presentation_handle())?;

    let mut third = PersonalAssistantV0Host::default();
    let third_start = third.start_synthetic()?;
    let third_handle = third_start.presentation_handle().clone();
    drop(third);

    let mut replacement = PersonalAssistantV0Host::default();
    let replacement_start = replacement.start_synthetic()?;
    assert_ne!(replacement_start.presentation_handle(), &third_handle);
    replacement.cancel(replacement_start.presentation_handle())?;
    Ok(())
}

#[test]
fn public_debug_and_closed_errors_are_redacted_and_fixed() -> Result<(), Box<dyn Error>> {
    let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
    let mut host = PersonalAssistantV0Host::new();
    let start = host.start_synthetic()?;
    let handle = start.presentation_handle().clone();
    let handle_value = handle.as_str().to_owned();
    let cancelled = host.cancel(&handle)?;
    let batch = host.updates(&handle, Some(0))?;

    let rendered = format!("{host:?} {start:?} {handle:?} {cancelled:?} {batch:?}");
    assert!(rendered.contains("[REDACTED]"));
    for forbidden in [
        "Prepare a concise three-bullet board update",
        "personal-assistant-text-v0",
        "gpt-5.6-luna",
        "pa-v0-run-",
        "pa-v0-request-",
        handle_value.as_str(),
    ] {
        assert!(!rendered.contains(forbidden));
    }

    let errors = [
        (PersonalAssistantV0Error::Busy, "busy"),
        (PersonalAssistantV0Error::InvalidRequest, "invalid_request"),
        (PersonalAssistantV0Error::InvalidHandle, "invalid_handle"),
        (
            PersonalAssistantV0Error::ProtocolViolation,
            "protocol_violation",
        ),
        (PersonalAssistantV0Error::LimitExceeded, "limit_exceeded"),
        (
            PersonalAssistantV0Error::DeadlineExceeded,
            "deadline_exceeded",
        ),
        (PersonalAssistantV0Error::Internal, "internal"),
    ];
    for (error, expected) in errors {
        assert_eq!(error.code(), expected);
        assert_eq!(error.to_string(), expected);
        let error_debug = format!("{error:?} {error}");
        assert!(!error_debug.contains("pa-v0-run-"));
        assert!(!error_debug.contains("Prepare a concise"));
        assert!(!error_debug.contains("gpt-5.6-luna"));
    }
    Ok(())
}
