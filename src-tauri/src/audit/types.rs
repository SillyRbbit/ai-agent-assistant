#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AuditSequence(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuditActor {
    User,
    Model,
    Policy,
    Executor,
    System,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditEventInput {
    pub actor: AuditActor,
    pub event_type: String,
    pub summary: String,
    pub details: String,
}

impl AuditEventInput {
    #[must_use]
    pub fn new(
        actor: AuditActor,
        event_type: impl Into<String>,
        summary: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            actor,
            event_type: event_type.into(),
            summary: summary.into(),
            details: details.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditEvent {
    pub sequence: AuditSequence,
    pub actor: AuditActor,
    pub event_type: String,
    pub summary: String,
    pub details_redacted: String,
}

impl AuditEvent {
    #[must_use]
    pub fn from_input(
        sequence: AuditSequence,
        input: AuditEventInput,
        details_redacted: String,
    ) -> Self {
        Self {
            sequence,
            actor: input.actor,
            event_type: input.event_type,
            summary: input.summary,
            details_redacted,
        }
    }
}
