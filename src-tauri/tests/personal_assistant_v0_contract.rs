use std::error::Error;
use std::sync::Mutex;

use ai_agent_assistant_lib::{
    PersonalAssistantV0Cancellation, PersonalAssistantV0Error, PersonalAssistantV0Host,
    PersonalAssistantV0Status,
};

static TEST_SERIAL: Mutex<()> = Mutex::new(());

#[test]
fn public_host_exposes_only_closed_volatile_no_input_lifecycle() -> Result<(), Box<dyn Error>> {
    let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
    let mut host = PersonalAssistantV0Host::new();

    assert_eq!(host.status(), PersonalAssistantV0Status::Idle);
    assert_eq!(
        host.request_byte_len(),
        Err(PersonalAssistantV0Error::NoActiveRun)
    );
    assert_eq!(host.cancel(), Err(PersonalAssistantV0Error::NoActiveRun));

    host.start_synthetic()?;
    assert_eq!(host.status(), PersonalAssistantV0Status::Starting);
    assert!(host.request_byte_len()? > 0);
    assert_eq!(host.start_synthetic(), Err(PersonalAssistantV0Error::Busy));
    assert_eq!(host.cancel()?, PersonalAssistantV0Cancellation::Cancelled);
    assert_eq!(host.status(), PersonalAssistantV0Status::Cancelled);
    assert_eq!(
        host.cancel()?,
        PersonalAssistantV0Cancellation::AlreadyTerminal(PersonalAssistantV0Status::Cancelled)
    );
    assert_eq!(
        host.request_byte_len(),
        Err(PersonalAssistantV0Error::NoActiveRun)
    );

    host.start_synthetic()?;
    assert_eq!(host.status(), PersonalAssistantV0Status::Starting);
    assert_eq!(host.cancel()?, PersonalAssistantV0Cancellation::Cancelled);
    Ok(())
}

#[test]
fn process_lease_denies_cross_host_start_until_cleanup_or_drop() -> Result<(), Box<dyn Error>> {
    let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
    let mut first = PersonalAssistantV0Host::default();
    let mut second = PersonalAssistantV0Host::default();

    first.start_synthetic()?;
    assert_eq!(
        second.start_synthetic(),
        Err(PersonalAssistantV0Error::Busy)
    );
    drop(first);

    second.start_synthetic()?;
    assert_eq!(second.status(), PersonalAssistantV0Status::Starting);
    assert_eq!(second.cancel()?, PersonalAssistantV0Cancellation::Cancelled);
    Ok(())
}

#[test]
fn public_debug_and_errors_are_fixed_and_content_free() -> Result<(), Box<dyn Error>> {
    let _serial = TEST_SERIAL.lock().map_err(|_| "test lock poisoned")?;
    let mut host = PersonalAssistantV0Host::new();
    host.start_synthetic()?;
    let debug = format!("{host:?}");
    assert!(debug.contains("[REDACTED]"));
    for forbidden in [
        "Prepare a concise three-bullet board update",
        "personal-assistant-text-v0",
        "gpt-5.6-luna",
        "pa-v0-run-",
        "pa-v0-request-",
    ] {
        assert!(!debug.contains(forbidden));
    }

    for error in [
        PersonalAssistantV0Error::Busy,
        PersonalAssistantV0Error::NoActiveRun,
        PersonalAssistantV0Error::IdentityExhausted,
        PersonalAssistantV0Error::ReturnedIdentityMismatch,
        PersonalAssistantV0Error::RequestRejected,
        PersonalAssistantV0Error::ProtocolViolation,
        PersonalAssistantV0Error::LimitExceeded,
        PersonalAssistantV0Error::Internal,
    ] {
        let rendered = format!("{error:?} {error}");
        assert!(!rendered.contains("pa-v0-run-"));
        assert!(!rendered.contains("Prepare a concise"));
        assert!(!rendered.contains("gpt-5.6-luna"));
    }
    host.cancel()?;
    Ok(())
}
