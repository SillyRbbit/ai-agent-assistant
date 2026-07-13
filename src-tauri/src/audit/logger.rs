use thiserror::Error;

use super::types::{AuditEvent, AuditEventInput, AuditSequence};

pub type AuditResult<T> = Result<T, AuditError>;

pub trait AuditLogger {
    fn record(&mut self, input: AuditEventInput) -> AuditResult<AuditEvent>;
    fn list(&self) -> Vec<AuditEvent>;
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AuditError {
    #[error("audit event type must not be empty")]
    EmptyEventType,
    #[error("audit summary must not be empty")]
    EmptySummary,
    #[error("audit sequence space is exhausted")]
    SequenceSpaceExhausted,
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryAuditLogger {
    next_sequence: u64,
    events: Vec<AuditEvent>,
}

impl InMemoryAuditLogger {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl AuditLogger for InMemoryAuditLogger {
    fn record(&mut self, input: AuditEventInput) -> AuditResult<AuditEvent> {
        if input.event_type.trim().is_empty() {
            return Err(AuditError::EmptyEventType);
        }

        if input.summary.trim().is_empty() {
            return Err(AuditError::EmptySummary);
        }

        let Some(next_sequence) = self.next_sequence.checked_add(1) else {
            return Err(AuditError::SequenceSpaceExhausted);
        };

        self.next_sequence = next_sequence;
        let details_redacted = redact_secret_like_content(&input.details);
        let event =
            AuditEvent::from_input(AuditSequence(self.next_sequence), input, details_redacted);
        self.events.push(event.clone());
        Ok(event)
    }

    fn list(&self) -> Vec<AuditEvent> {
        self.events.clone()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NoopAuditLogger;

impl NoopAuditLogger {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl AuditLogger for NoopAuditLogger {
    fn record(&mut self, input: AuditEventInput) -> AuditResult<AuditEvent> {
        if input.event_type.trim().is_empty() {
            return Err(AuditError::EmptyEventType);
        }

        if input.summary.trim().is_empty() {
            return Err(AuditError::EmptySummary);
        }

        Ok(AuditEvent::from_input(
            AuditSequence(0),
            input,
            "[not stored]".to_owned(),
        ))
    }

    fn list(&self) -> Vec<AuditEvent> {
        Vec::new()
    }
}

#[must_use]
pub fn redact_secret_like_content(input: &str) -> String {
    input
        .split_whitespace()
        .map(redact_token)
        .collect::<Vec<String>>()
        .join(" ")
}

fn redact_token(token: &str) -> String {
    let lower = token.to_ascii_lowercase();
    let secret_markers = ["password=", "token=", "api_key=", "apikey=", "secret="];

    if secret_markers.iter().any(|marker| lower.contains(*marker)) {
        "[redacted]".to_owned()
    } else {
        token.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        redact_secret_like_content, AuditError, AuditLogger, InMemoryAuditLogger, NoopAuditLogger,
    };
    use crate::audit::types::{AuditActor, AuditEventInput, AuditSequence};

    #[test]
    fn records_events_in_deterministic_order() {
        let mut logger = InMemoryAuditLogger::new();

        assert!(logger
            .record(AuditEventInput::new(
                AuditActor::System,
                "run_started",
                "Run started",
                "run_id=1",
            ))
            .is_ok());
        assert!(logger
            .record(AuditEventInput::new(
                AuditActor::Policy,
                "policy_allow",
                "Policy allowed action",
                "tool=get_current_datetime",
            ))
            .is_ok());

        let sequences: Vec<AuditSequence> = logger
            .list()
            .into_iter()
            .map(|event| event.sequence)
            .collect();

        assert_eq!(sequences, vec![AuditSequence(1), AuditSequence(2)]);
    }

    #[test]
    fn redacts_secret_like_tokens() {
        assert_eq!(
            redact_secret_like_content("tool=demo token=abc123 ok=true"),
            "tool=demo [redacted] ok=true".to_owned()
        );
    }

    #[test]
    fn rejects_invalid_events() {
        let mut logger = InMemoryAuditLogger::new();

        assert_eq!(
            logger.record(AuditEventInput::new(
                AuditActor::System,
                " ",
                "Run started",
                "run_id=1",
            )),
            Err(AuditError::EmptyEventType)
        );
    }

    #[test]
    fn noop_logger_validates_but_does_not_store() {
        let mut logger = NoopAuditLogger::new();

        assert!(logger
            .record(AuditEventInput::new(
                AuditActor::System,
                "run_started",
                "Run started",
                "run_id=1",
            ))
            .is_ok());
        assert_eq!(logger.list(), Vec::new());
    }
}
