//! Volatile owner for the sealed Personal Assistant v0 synthetic turn.
//!
//! This host performs no I/O; a crate-private typed ingress serves the direct demo. It owns one
//! process-local Native run, validates the exact returned identity and initial
//! status, and retains ambiguous cleanup ownership fail closed.

use std::fmt;
use std::mem;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[cfg(test)]
use std::sync::Arc;

use thiserror::Error;

use crate::agent::gateway_protocol::{
    AGENT_RUN_DEADLINE, GATEWAY_CONNECT_TIMEOUT, GATEWAY_STREAM_IDLE_TIMEOUT,
    PROVIDER_TURN_DEADLINE,
};
use crate::agent::native_runtime::{NativeAgentRun, NativeAgentRuntime};
use crate::agent::runtime::{
    AgentRuntime, RuntimeCancellationOutcome, RuntimeError, RuntimeInvalidRequest, RuntimeRun,
    RuntimeRunIdentity, RuntimeRunStatus, RuntimeTurnRequest,
};
use crate::agent::runtime::{
    RuntimeEventAcceptance, RuntimeEventEnvelope, RuntimeOutputText, RuntimeResponseId,
    UntrustedRuntimeEvent,
};
#[cfg(test)]
use crate::agent::runtime::{RuntimeEventRejection, RuntimeFailure, RuntimeFailureCode};

const MAX_PRESENTATION_UPDATES: usize = 128;
const MAX_PRESENTATION_SEQUENCE: u64 = 128;
const MAX_UPDATE_BATCH: usize = 16;
const MAX_CORRELATION_BYTES: usize = 128;

const MAX_DELTA_CHARACTERS: usize = 1_024;
const MAX_DELTA_BYTES: usize = 4_096;
const MAX_OUTPUT_CHARACTERS: usize = 8_192;
const MAX_OUTPUT_BYTES: usize = 32_768;

static PROCESS_LEASE_HELD: AtomicBool = AtomicBool::new(false);
static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);
static PROCESS_QUARANTINE: Mutex<Option<NativeQuarantine>> = Mutex::new(None);

/// Opaque presentation correlation issued only by trusted Rust.
#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0PresentationHandle(String);

impl PersonalAssistantV0PresentationHandle {
    fn for_generation(generation: u64) -> Result<Self, PersonalAssistantV0Error> {
        let value = format!("pa-v0-present-{generation:016x}");
        if is_valid_correlation(&value) {
            Ok(Self(value))
        } else {
            Err(PersonalAssistantV0Error::Internal)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PersonalAssistantV0PresentationHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PersonalAssistantV0PresentationHandle")
            .field(&"[REDACTED]")
            .finish()
    }
}

/// Closed failure codes preserved for the future versioned presentation DTO.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersonalAssistantV0FailureCode {
    Unauthenticated,
    Forbidden,
    RateLimited,
    RequestRejected,
    ProviderUnavailable,
    ProviderTimeout,
    ProtocolViolation,
    LimitExceeded,
    DeadlineExceeded,
    CleanupFailed,
    Internal,
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0Failure {
    code: PersonalAssistantV0FailureCode,
    support_correlation: Option<String>,
}

impl PersonalAssistantV0Failure {
    fn closed(code: PersonalAssistantV0FailureCode, generation: u64) -> Self {
        let support_correlation = if matches!(
            code,
            PersonalAssistantV0FailureCode::CleanupFailed
                | PersonalAssistantV0FailureCode::Internal
        ) {
            let value = format!("pa-v0-support-{generation:016x}");
            debug_assert!(is_valid_correlation(&value));
            Some(value)
        } else {
            None
        };
        Self {
            code,
            support_correlation,
        }
    }

    #[must_use]
    pub const fn code(&self) -> PersonalAssistantV0FailureCode {
        self.code
    }

    #[must_use]
    pub fn support_correlation(&self) -> Option<&str> {
        self.support_correlation.as_deref()
    }
}

impl fmt::Debug for PersonalAssistantV0Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0Failure")
            .field("code", &self.code)
            .field(
                "support_correlation",
                &self.support_correlation.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0TextSnapshot {
    sequence: u64,
    accepted_text: String,
}

impl PersonalAssistantV0TextSnapshot {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn accepted_text(&self) -> &str {
        &self.accepted_text
    }
}

impl fmt::Debug for PersonalAssistantV0TextSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0TextSnapshot")
            .field("sequence", &self.sequence)
            .field("accepted_text", &"[REDACTED]")
            .field("accepted_text_bytes", &self.accepted_text.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0CompletedSnapshot {
    sequence: u64,
    accepted_text: String,
    final_answer: String,
}

impl PersonalAssistantV0CompletedSnapshot {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn accepted_text(&self) -> &str {
        &self.accepted_text
    }

    #[must_use]
    pub fn final_answer(&self) -> &str {
        &self.final_answer
    }
}

impl fmt::Debug for PersonalAssistantV0CompletedSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0CompletedSnapshot")
            .field("sequence", &self.sequence)
            .field("accepted_text", &"[REDACTED]")
            .field("accepted_text_bytes", &self.accepted_text.len())
            .field("final_answer", &"[REDACTED]")
            .field("final_answer_bytes", &self.final_answer.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0FailedSnapshot {
    sequence: u64,
    accepted_text: String,
    failure: PersonalAssistantV0Failure,
}

impl PersonalAssistantV0FailedSnapshot {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn accepted_text(&self) -> &str {
        &self.accepted_text
    }

    #[must_use]
    pub const fn failure(&self) -> &PersonalAssistantV0Failure {
        &self.failure
    }
}

impl fmt::Debug for PersonalAssistantV0FailedSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0FailedSnapshot")
            .field("sequence", &self.sequence)
            .field("accepted_text", &"[REDACTED]")
            .field("accepted_text_bytes", &self.accepted_text.len())
            .field("failure", &self.failure)
            .finish()
    }
}

/// Closed volatile snapshot. It intentionally has no serialization derive.
#[derive(Clone, Eq, PartialEq)]
pub enum PersonalAssistantV0Snapshot {
    Starting,
    Streaming(PersonalAssistantV0TextSnapshot),
    Cancelling(PersonalAssistantV0TextSnapshot),
    Completed(PersonalAssistantV0CompletedSnapshot),
    Failed(PersonalAssistantV0FailedSnapshot),
    Cancelled(PersonalAssistantV0TextSnapshot),
}

impl PersonalAssistantV0Snapshot {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        match self {
            Self::Starting => 0,
            Self::Streaming(snapshot) | Self::Cancelling(snapshot) | Self::Cancelled(snapshot) => {
                snapshot.sequence
            }
            Self::Completed(snapshot) => snapshot.sequence,
            Self::Failed(snapshot) => snapshot.sequence,
        }
    }

    #[must_use]
    pub fn accepted_text(&self) -> &str {
        match self {
            Self::Starting => "",
            Self::Streaming(snapshot) | Self::Cancelling(snapshot) | Self::Cancelled(snapshot) => {
                &snapshot.accepted_text
            }
            Self::Completed(snapshot) => &snapshot.accepted_text,
            Self::Failed(snapshot) => &snapshot.accepted_text,
        }
    }

    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed(_) | Self::Failed(_) | Self::Cancelled(_)
        )
    }
}

impl fmt::Debug for PersonalAssistantV0Snapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Starting => formatter.write_str("Starting"),
            Self::Streaming(snapshot) => {
                formatter.debug_tuple("Streaming").field(snapshot).finish()
            }
            Self::Cancelling(snapshot) => {
                formatter.debug_tuple("Cancelling").field(snapshot).finish()
            }
            Self::Completed(snapshot) => {
                formatter.debug_tuple("Completed").field(snapshot).finish()
            }
            Self::Failed(snapshot) => formatter.debug_tuple("Failed").field(snapshot).finish(),
            Self::Cancelled(snapshot) => {
                formatter.debug_tuple("Cancelled").field(snapshot).finish()
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0SequenceUpdate {
    sequence: u64,
}

impl PersonalAssistantV0SequenceUpdate {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }
}

impl fmt::Debug for PersonalAssistantV0SequenceUpdate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0SequenceUpdate")
            .field("sequence", &self.sequence)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0TextDeltaUpdate {
    sequence: u64,
    text: String,
}

impl PersonalAssistantV0TextDeltaUpdate {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Debug for PersonalAssistantV0TextDeltaUpdate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0TextDeltaUpdate")
            .field("sequence", &self.sequence)
            .field("text", &"[REDACTED]")
            .field("text_bytes", &self.text.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0CompletedUpdate {
    sequence: u64,
    final_answer: String,
}

impl PersonalAssistantV0CompletedUpdate {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn final_answer(&self) -> &str {
        &self.final_answer
    }
}

impl fmt::Debug for PersonalAssistantV0CompletedUpdate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0CompletedUpdate")
            .field("sequence", &self.sequence)
            .field("final_answer", &"[REDACTED]")
            .field("final_answer_bytes", &self.final_answer.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersonalAssistantV0FailedUpdate {
    sequence: u64,
    failure: PersonalAssistantV0Failure,
}

impl PersonalAssistantV0FailedUpdate {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn failure(&self) -> &PersonalAssistantV0Failure {
        &self.failure
    }
}

impl fmt::Debug for PersonalAssistantV0FailedUpdate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0FailedUpdate")
            .field("sequence", &self.sequence)
            .field("failure", &self.failure)
            .finish()
    }
}

/// Closed chronological presentation update.
#[derive(Clone, Eq, PartialEq)]
pub enum PersonalAssistantV0Update {
    Started(PersonalAssistantV0SequenceUpdate),
    TextDelta(PersonalAssistantV0TextDeltaUpdate),
    Completed(PersonalAssistantV0CompletedUpdate),
    Failed(PersonalAssistantV0FailedUpdate),
    Cancelled(PersonalAssistantV0SequenceUpdate),
}

impl PersonalAssistantV0Update {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        match self {
            Self::Started(update) | Self::Cancelled(update) => update.sequence,
            Self::TextDelta(update) => update.sequence,
            Self::Completed(update) => update.sequence,
            Self::Failed(update) => update.sequence,
        }
    }
}

impl fmt::Debug for PersonalAssistantV0Update {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Started(update) => formatter.debug_tuple("Started").field(update).finish(),
            Self::TextDelta(update) => formatter.debug_tuple("TextDelta").field(update).finish(),
            Self::Completed(update) => formatter.debug_tuple("Completed").field(update).finish(),
            Self::Failed(update) => formatter.debug_tuple("Failed").field(update).finish(),
            Self::Cancelled(update) => formatter.debug_tuple("Cancelled").field(update).finish(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersonalAssistantV0Start {
    presentation_handle: PersonalAssistantV0PresentationHandle,
    snapshot: PersonalAssistantV0Snapshot,
}

impl PersonalAssistantV0Start {
    #[must_use]
    pub const fn presentation_handle(&self) -> &PersonalAssistantV0PresentationHandle {
        &self.presentation_handle
    }

    #[must_use]
    pub const fn snapshot(&self) -> &PersonalAssistantV0Snapshot {
        &self.snapshot
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        PersonalAssistantV0PresentationHandle,
        PersonalAssistantV0Snapshot,
    ) {
        (self.presentation_handle, self.snapshot)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersonalAssistantV0UpdateBatch {
    updates: Vec<PersonalAssistantV0Update>,
    through_sequence: u64,
    has_more: bool,
    snapshot: PersonalAssistantV0Snapshot,
}

impl PersonalAssistantV0UpdateBatch {
    #[must_use]
    pub fn updates(&self) -> &[PersonalAssistantV0Update] {
        &self.updates
    }

    #[must_use]
    pub const fn through_sequence(&self) -> u64 {
        self.through_sequence
    }

    #[must_use]
    pub const fn has_more(&self) -> bool {
        self.has_more
    }

    #[must_use]
    pub const fn snapshot(&self) -> &PersonalAssistantV0Snapshot {
        &self.snapshot
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum PersonalAssistantV0Error {
    #[error("busy")]
    Busy,
    #[error("invalid_request")]
    InvalidRequest,
    #[error("invalid_handle")]
    InvalidHandle,
    #[error("protocol_violation")]
    ProtocolViolation,
    #[error("limit_exceeded")]
    LimitExceeded,
    #[error("deadline_exceeded")]
    DeadlineExceeded,
    #[error("internal")]
    Internal,
}

impl PersonalAssistantV0Error {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Busy => "busy",
            Self::InvalidRequest => "invalid_request",
            Self::InvalidHandle => "invalid_handle",
            Self::ProtocolViolation => "protocol_violation",
            Self::LimitExceeded => "limit_exceeded",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::Internal => "internal",
        }
    }
}

pub struct PersonalAssistantV0Host {
    core: HostCore<NativeAgentRuntime>,
}

impl Default for PersonalAssistantV0Host {
    fn default() -> Self {
        Self::new()
    }
}

impl PersonalAssistantV0Host {
    pub(crate) fn start_direct(
        &mut self,
    ) -> Result<PersonalAssistantV0Start, PersonalAssistantV0Error> {
        self.core.start_with_profile(true)
    }
    #[must_use]
    pub fn new() -> Self {
        Self {
            core: HostCore::new(NativeAgentRuntime, MonotonicClock::system()),
        }
    }

    pub fn start_synthetic(
        &mut self,
    ) -> Result<PersonalAssistantV0Start, PersonalAssistantV0Error> {
        self.core.start_synthetic()
    }

    pub fn snapshot(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        self.core.snapshot(handle)
    }

    pub fn updates(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
        after_sequence: Option<u64>,
    ) -> Result<PersonalAssistantV0UpdateBatch, PersonalAssistantV0Error> {
        self.core.updates(handle, after_sequence)
    }

    pub fn cancel(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        self.core.cancel(handle)
    }

    /// Native-only ingress. The adapter supplies validated provider events;
    /// presentation callers cannot construct or submit these events.
    pub(crate) fn accept_direct(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
        event: crate::personal_assistant_direct::ProviderEvent,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        self.core.validate_handle(handle)?;
        self.core.sample_deadline()?;
        let state = mem::replace(&mut self.core.state, HostState::Idle);
        let HostState::Active(active) = state else {
            self.core.state = state;
            return Err(PersonalAssistantV0Error::ProtocolViolation);
        };
        let prepared = prepare_direct_event(
            &active.record,
            &active.owner.expected_identity,
            self.core.clock.sample(),
            event,
        );
        match prepared {
            Ok(prepared) => self.core.commit_input(active, prepared),
            Err(_) => self
                .core
                .finish_fixture_failure(active, PersonalAssistantV0FailureCode::ProtocolViolation),
        }
    }

    pub(crate) fn fail_direct(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
        code: PersonalAssistantV0FailureCode,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        self.core.validate_handle(handle)?;
        let state = mem::replace(&mut self.core.state, HostState::Idle);
        match state {
            HostState::Active(active) => self.core.finish_fixture_failure(active, code),
            other => {
                self.core.state = other;
                self.core.fixture_snapshot()
            }
        }
    }
}

impl fmt::Debug for PersonalAssistantV0Host {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersonalAssistantV0Host")
            .field("state", &self.core.state_label())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

impl Drop for PersonalAssistantV0Host {
    fn drop(&mut self) {
        let state = mem::replace(&mut self.core.state, HostState::Idle);
        let Some(payload) = cleanup_state_for_drop(state) else {
            return;
        };
        let native_payload = match payload {
            QuarantinePayload::Rejected(owner) => NativeQuarantine::Rejected(owner),
            QuarantinePayload::Session { record, owner } => {
                NativeQuarantine::Session { record, owner }
            }
        };
        quarantine_or_leak(native_payload, &PROCESS_QUARANTINE);
    }
}

struct HostCore<R: AgentRuntime> {
    runtime: R,
    clock: MonotonicClock,
    state: HostState<R::Run>,
}

impl<R: AgentRuntime> HostCore<R> {
    fn new(runtime: R, clock: MonotonicClock) -> Self {
        Self {
            runtime,
            clock,
            state: HostState::Idle,
        }
    }

    fn start_synthetic(&mut self) -> Result<PersonalAssistantV0Start, PersonalAssistantV0Error> {
        self.start_with_profile(false)
    }

    fn start_with_profile(
        &mut self,
        direct: bool,
    ) -> Result<PersonalAssistantV0Start, PersonalAssistantV0Error> {
        let prior_terminal = self.prepare_for_start()?;
        let lease = match ProcessLease::acquire() {
            Ok(lease) => lease,
            Err(error) => {
                self.restore_terminal(prior_terminal);
                return Err(error);
            }
        };
        let generation = match reserve_generation(&NEXT_GENERATION) {
            Ok(generation) => generation,
            Err(error) => {
                self.restore_terminal(prior_terminal);
                return Err(error);
            }
        };
        let now = self.clock.sample();
        let record = match SessionRecord::new(generation, now) {
            Ok(record) => record,
            Err(error) => {
                self.restore_terminal(prior_terminal);
                return Err(error);
            }
        };
        let run_id = format!("pa-v0-run-{generation:016x}");
        let request_id = format!("pa-v0-request-{generation:016x}");
        let request = match if direct {
            RuntimeTurnRequest::personal_assistant_direct(run_id, request_id)
        } else {
            RuntimeTurnRequest::personal_assistant_v0_synthetic(run_id, request_id)
        } {
            Ok(request) => request,
            Err(error) => {
                self.restore_terminal(prior_terminal);
                return Err(map_start_error(error));
            }
        };
        let expected_identity = request.identity();

        match start_checked(&self.runtime, request, &expected_identity) {
            Ok(ReturnedRunCheck::Accepted(run)) => {
                let snapshot = record.snapshot();
                let presentation_handle = record.presentation_handle.clone();
                self.state = HostState::Active(ActiveSession {
                    record,
                    owner: OwnedRun {
                        run,
                        expected_identity,
                        _lease: lease,
                    },
                });
                Ok(PersonalAssistantV0Start {
                    presentation_handle,
                    snapshot,
                })
            }
            Ok(ReturnedRunCheck::RejectedClean(error)) => {
                self.restore_terminal(prior_terminal);
                Err(error)
            }
            Ok(ReturnedRunCheck::RejectedAmbiguous { run, error }) => {
                self.state = HostState::Quarantined(OwnedRun {
                    run,
                    expected_identity,
                    _lease: lease,
                });
                Err(error)
            }
            Err(error) => {
                self.restore_terminal(prior_terminal);
                Err(map_start_error(error))
            }
        }
    }

    fn snapshot(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        self.validate_handle(handle)?;
        self.sample_deadline()?;
        self.current_record()
            .map(SessionRecord::snapshot)
            .ok_or(PersonalAssistantV0Error::InvalidHandle)
    }

    fn updates(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
        after_sequence: Option<u64>,
    ) -> Result<PersonalAssistantV0UpdateBatch, PersonalAssistantV0Error> {
        self.validate_handle(handle)?;
        let before_sampling_sequence = self
            .current_record()
            .map(SessionRecord::sequence)
            .ok_or(PersonalAssistantV0Error::InvalidHandle)?;
        if after_sequence.is_some_and(|cursor| {
            cursor > MAX_PRESENTATION_SEQUENCE || cursor > before_sampling_sequence
        }) {
            return Err(PersonalAssistantV0Error::InvalidRequest);
        }

        self.sample_deadline()?;
        let record = self
            .current_record()
            .ok_or(PersonalAssistantV0Error::InvalidHandle)?;
        let snapshot = record.snapshot();
        let Some(cursor) = after_sequence else {
            return Ok(PersonalAssistantV0UpdateBatch {
                updates: Vec::new(),
                through_sequence: snapshot.sequence(),
                has_more: false,
                snapshot,
            });
        };

        let start =
            usize::try_from(cursor).map_err(|_| PersonalAssistantV0Error::InvalidRequest)?;
        let end = start
            .saturating_add(MAX_UPDATE_BATCH)
            .min(record.journal.len());
        let updates = record.journal[start..end].to_vec();
        let through_sequence = match updates.last() {
            Some(update) => update.sequence(),
            None => cursor,
        };
        let has_more = snapshot.sequence() > through_sequence;
        Ok(PersonalAssistantV0UpdateBatch {
            updates,
            through_sequence,
            has_more,
            snapshot,
        })
    }

    fn cancel(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        let generation = self.validate_handle(handle)?;
        self.sample_deadline()?;
        let state = mem::replace(&mut self.state, HostState::Idle);
        match state {
            HostState::Active(active) => {
                self.finish_active(
                    active,
                    TerminalIntent::Cancelled,
                    CleanupExpectation::CancelledByHost,
                    PersonalAssistantV0Error::Internal,
                )?;
            }
            HostState::Cancelling(cancelling) => {
                if cancelling.record.generation != generation {
                    self.state = HostState::Cancelling(cancelling);
                    return Err(PersonalAssistantV0Error::InvalidHandle);
                }
                self.retry_cancelling(cancelling)?;
            }
            HostState::Terminal(terminal) => {
                let snapshot = terminal.record.snapshot();
                self.state = HostState::Terminal(terminal);
                return Ok(snapshot);
            }
            HostState::Idle => {
                self.state = HostState::Idle;
                return Err(PersonalAssistantV0Error::InvalidHandle);
            }
            HostState::Quarantined(owner) => {
                self.state = HostState::Quarantined(owner);
                return Err(PersonalAssistantV0Error::Busy);
            }
        }
        self.current_record()
            .map(SessionRecord::snapshot)
            .ok_or(PersonalAssistantV0Error::Internal)
    }

    fn prepare_for_start(&mut self) -> Result<Option<TerminalSession>, PersonalAssistantV0Error> {
        let state = mem::replace(&mut self.state, HostState::Idle);
        match state {
            HostState::Idle => Ok(None),
            HostState::Terminal(terminal) => Ok(Some(terminal)),
            HostState::Active(active) => {
                self.state = HostState::Active(active);
                Err(PersonalAssistantV0Error::Busy)
            }
            HostState::Cancelling(cancelling) => {
                self.state = HostState::Cancelling(cancelling);
                Err(PersonalAssistantV0Error::Busy)
            }
            HostState::Quarantined(owner) => {
                if let Some(owner) = owner_cleanup(owner) {
                    self.state = HostState::Quarantined(owner);
                    Err(PersonalAssistantV0Error::Busy)
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn restore_terminal(&mut self, terminal: Option<TerminalSession>) {
        if let Some(terminal) = terminal {
            self.state = HostState::Terminal(terminal);
        }
    }

    fn validate_handle(
        &self,
        handle: &PersonalAssistantV0PresentationHandle,
    ) -> Result<u64, PersonalAssistantV0Error> {
        let record = self
            .current_record()
            .ok_or(PersonalAssistantV0Error::InvalidHandle)?;
        if &record.presentation_handle != handle {
            return Err(PersonalAssistantV0Error::InvalidHandle);
        }
        Ok(record.generation)
    }

    fn current_record(&self) -> Option<&SessionRecord> {
        match &self.state {
            HostState::Active(active) => Some(&active.record),
            HostState::Cancelling(cancelling) => Some(&cancelling.record),
            HostState::Terminal(terminal) => Some(&terminal.record),
            HostState::Idle | HostState::Quarantined(_) => None,
        }
    }

    fn sample_deadline(&mut self) -> Result<(), PersonalAssistantV0Error> {
        let now = self.clock.sample();
        self.sample_deadline_at(now)
    }

    fn sample_deadline_at(&mut self, now: Duration) -> Result<(), PersonalAssistantV0Error> {
        let deadline = match &self.state {
            HostState::Active(active) => active.record.expired_deadline(now),
            HostState::Idle
            | HostState::Cancelling(_)
            | HostState::Terminal(_)
            | HostState::Quarantined(_) => None,
        };
        let Some(deadline) = deadline else {
            return Ok(());
        };

        let state = mem::replace(&mut self.state, HostState::Idle);
        let HostState::Active(mut active) = state else {
            self.state = state;
            return Err(PersonalAssistantV0Error::Internal);
        };
        active.record.note_expired_deadline(deadline);
        let generation = active.record.generation;
        self.finish_active(
            active,
            TerminalIntent::Failed(PersonalAssistantV0Failure::closed(
                PersonalAssistantV0FailureCode::DeadlineExceeded,
                generation,
            )),
            CleanupExpectation::CancelledByHost,
            PersonalAssistantV0Error::DeadlineExceeded,
        )
    }

    fn finish_active(
        &mut self,
        mut active: ActiveSession<R::Run>,
        intent: TerminalIntent,
        cleanup_expectation: CleanupExpectation,
        pending_error: PersonalAssistantV0Error,
    ) -> Result<(), PersonalAssistantV0Error> {
        let identity_matches = active.owner.run.identity() == &active.owner.expected_identity;
        let intended = if identity_matches {
            intent
        } else {
            TerminalIntent::Failed(PersonalAssistantV0Failure::closed(
                PersonalAssistantV0FailureCode::Internal,
                active.record.generation,
            ))
        };
        let pending_terminal = match active.record.prepare_terminal(intended) {
            Ok(terminal) => terminal,
            Err(error) => {
                self.state = HostState::Active(active);
                return Err(error);
            }
        };
        let internal_terminal = match active.record.prepare_terminal(TerminalIntent::Failed(
            PersonalAssistantV0Failure::closed(
                PersonalAssistantV0FailureCode::Internal,
                active.record.generation,
            ),
        )) {
            Ok(terminal) => terminal,
            Err(error) => {
                self.state = HostState::Active(active);
                return Err(error);
            }
        };
        active.record.phase = PresentationPhase::Cancelling;
        let cleanup = active.owner.run.cancel();
        match classify_cleanup(
            &active.owner.run,
            &active.owner.expected_identity,
            &cleanup,
            cleanup_expectation,
        ) {
            CleanupResolution::Expected if identity_matches => {
                active.record.commit_terminal(pending_terminal);
                self.state = HostState::Terminal(TerminalSession {
                    record: active.record,
                    _cleanup: CleanupDisposition::NotRequired,
                });
                Ok(())
            }
            CleanupResolution::Expected | CleanupResolution::ProvedUnexpected => {
                active.record.commit_terminal(internal_terminal);
                self.state = HostState::Terminal(TerminalSession {
                    record: active.record,
                    _cleanup: CleanupDisposition::NotRequired,
                });
                Ok(())
            }
            CleanupResolution::Ambiguous => {
                self.state = HostState::Cancelling(CancellingSession {
                    record: active.record,
                    owner: active.owner,
                    pending_terminal,
                    internal_terminal,
                    cleanup_expectation,
                });
                Err(pending_error)
            }
        }
    }

    fn retry_cancelling(
        &mut self,
        mut cancelling: CancellingSession<R::Run>,
    ) -> Result<(), PersonalAssistantV0Error> {
        let identity_matches =
            cancelling.owner.run.identity() == &cancelling.owner.expected_identity;
        let cleanup = cancelling.owner.run.cancel();
        match classify_cleanup(
            &cancelling.owner.run,
            &cancelling.owner.expected_identity,
            &cleanup,
            cancelling.cleanup_expectation,
        ) {
            CleanupResolution::Expected if identity_matches => {
                cancelling
                    .record
                    .commit_terminal(cancelling.pending_terminal);
                self.state = HostState::Terminal(TerminalSession {
                    record: cancelling.record,
                    _cleanup: CleanupDisposition::NotRequired,
                });
                Ok(())
            }
            CleanupResolution::Expected | CleanupResolution::ProvedUnexpected => {
                cancelling
                    .record
                    .commit_terminal(cancelling.internal_terminal);
                self.state = HostState::Terminal(TerminalSession {
                    record: cancelling.record,
                    _cleanup: CleanupDisposition::NotRequired,
                });
                Ok(())
            }
            CleanupResolution::Ambiguous => {
                self.state = HostState::Cancelling(cancelling);
                Err(PersonalAssistantV0Error::Internal)
            }
        }
    }

    #[cfg(test)]
    fn accept_fixture_event(
        &mut self,
        handle: &PersonalAssistantV0PresentationHandle,
        event: FixtureEvent,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        self.validate_handle(handle)?;
        let was_terminal = self
            .current_record()
            .is_some_and(|record| record.snapshot().is_terminal());
        let now = self.clock.sample();
        self.sample_deadline_at(now)?;
        if !was_terminal {
            if let Some(snapshot) = self.current_record().map(SessionRecord::snapshot) {
                if snapshot.is_terminal() {
                    return Ok(snapshot);
                }
            }
        }

        let state = mem::replace(&mut self.state, HostState::Idle);
        let HostState::Active(active) = state else {
            self.state = state;
            return Err(PersonalAssistantV0Error::ProtocolViolation);
        };

        if active.owner.run.identity() != &active.owner.expected_identity
            || !active
                .record
                .runtime_state_matches(active.owner.run.status())
        {
            return self.finish_fixture_failure(active, PersonalAssistantV0FailureCode::Internal);
        }

        let prepared = match prepare_fixture_event(
            &active.record,
            &active.owner.expected_identity,
            now,
            event,
        ) {
            Ok(prepared) => prepared,
            Err(error) => {
                self.state = HostState::Active(active);
                return Err(error);
            }
        };
        let FixturePreparation::Accept(prepared) = prepared else {
            return self
                .finish_fixture_failure(active, PersonalAssistantV0FailureCode::LimitExceeded);
        };
        let PreparedFixture { envelope, commit } = *prepared;

        self.commit_input(active, PreparedFixture { envelope, commit })
    }

    fn commit_input(
        &mut self,
        mut active: ActiveSession<R::Run>,
        prepared: PreparedFixture,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        let PreparedFixture { envelope, commit } = prepared;

        if active.owner.run.identity() != &active.owner.expected_identity
            || !active
                .record
                .runtime_state_matches(active.owner.run.status())
        {
            return self.finish_fixture_failure(active, PersonalAssistantV0FailureCode::Internal);
        }

        let acceptance = active.owner.run.accept_event(envelope);
        let status = active.owner.run.status();
        let identity_matches = active.owner.run.identity() == &active.owner.expected_identity;
        if !identity_matches || !fixture_acceptance_matches(&acceptance, &commit, status) {
            return self.finish_fixture_failure(active, PersonalAssistantV0FailureCode::Internal);
        }

        match commit {
            FixtureCommit::Started {
                sequence,
                next_runtime_sequence,
                idle_deadline,
                ..
            } => {
                let prepared = active.record.prepare_update(
                    PersonalAssistantV0Update::Started(PersonalAssistantV0SequenceUpdate {
                        sequence,
                    }),
                    RecordCommitMetadata {
                        next_runtime_sequence,
                        idle_deadline,
                        output_characters: active.record.accepted_characters,
                        output_bytes: active.record.accepted_bytes,
                    },
                );
                let Ok(prepared) = prepared else {
                    return self
                        .finish_fixture_failure(active, PersonalAssistantV0FailureCode::Internal);
                };
                active.record.commit_update(prepared);
                let snapshot = active.record.snapshot();
                self.state = HostState::Active(active);
                Ok(snapshot)
            }
            FixtureCommit::Delta {
                sequence,
                next_runtime_sequence,
                text,
                output_characters,
                output_bytes,
                idle_deadline,
            } => {
                let prepared = active.record.prepare_update(
                    PersonalAssistantV0Update::TextDelta(PersonalAssistantV0TextDeltaUpdate {
                        sequence,
                        text,
                    }),
                    RecordCommitMetadata {
                        next_runtime_sequence,
                        idle_deadline,
                        output_characters,
                        output_bytes,
                    },
                );
                let Ok(prepared) = prepared else {
                    return self
                        .finish_fixture_failure(active, PersonalAssistantV0FailureCode::Internal);
                };
                active.record.commit_update(prepared);
                let snapshot = active.record.snapshot();
                self.state = HostState::Active(active);
                Ok(snapshot)
            }
            FixtureCommit::Completed {
                next_runtime_sequence,
            } => {
                active.record.next_runtime_sequence = next_runtime_sequence;
                self.finish_active(
                    active,
                    TerminalIntent::Completed,
                    CleanupExpectation::AlreadyTerminal(RuntimeRunStatus::Completed),
                    PersonalAssistantV0Error::Internal,
                )?;
                self.fixture_snapshot()
            }
            #[cfg(test)]
            FixtureCommit::Failed {
                next_runtime_sequence,
                failure,
                ..
            } => {
                active.record.next_runtime_sequence = next_runtime_sequence;
                self.finish_active(
                    active,
                    TerminalIntent::Failed(failure),
                    CleanupExpectation::AlreadyTerminal(RuntimeRunStatus::Failed),
                    PersonalAssistantV0Error::Internal,
                )?;
                self.fixture_snapshot()
            }
            #[cfg(test)]
            FixtureCommit::ExpectedProtocolFailure { failure } => {
                self.finish_active(
                    active,
                    TerminalIntent::Failed(failure),
                    CleanupExpectation::AlreadyTerminal(RuntimeRunStatus::Failed),
                    PersonalAssistantV0Error::ProtocolViolation,
                )?;
                self.fixture_snapshot()
            }
        }
    }

    fn finish_fixture_failure(
        &mut self,
        active: ActiveSession<R::Run>,
        code: PersonalAssistantV0FailureCode,
    ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        let generation = active.record.generation;
        let status = active.owner.run.status();
        let cleanup_expectation = if status.is_terminal() {
            CleanupExpectation::AlreadyTerminal(status)
        } else {
            CleanupExpectation::CancelledByHost
        };
        self.finish_active(
            active,
            TerminalIntent::Failed(PersonalAssistantV0Failure::closed(code, generation)),
            cleanup_expectation,
            PersonalAssistantV0Error::Internal,
        )?;
        self.fixture_snapshot()
    }

    fn fixture_snapshot(&self) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error> {
        self.current_record()
            .map(SessionRecord::snapshot)
            .ok_or(PersonalAssistantV0Error::Internal)
    }

    fn state_label(&self) -> &'static str {
        match &self.state {
            HostState::Idle => "idle",
            HostState::Active(active) => active.record.phase.label(),
            HostState::Cancelling(_) => "cancelling",
            HostState::Terminal(terminal) => terminal.record.phase.label(),
            HostState::Quarantined(_) => "quarantined",
        }
    }
}

enum HostState<R: RuntimeRun> {
    Idle,
    Active(ActiveSession<R>),
    Cancelling(CancellingSession<R>),
    Terminal(TerminalSession),
    Quarantined(OwnedRun<R>),
}

struct ActiveSession<R: RuntimeRun> {
    record: SessionRecord,
    owner: OwnedRun<R>,
}

struct CancellingSession<R: RuntimeRun> {
    record: SessionRecord,
    owner: OwnedRun<R>,
    pending_terminal: PreparedTerminal,
    internal_terminal: PreparedTerminal,
    cleanup_expectation: CleanupExpectation,
}

struct TerminalSession {
    record: SessionRecord,
    _cleanup: CleanupDisposition,
}

struct SessionRecord {
    generation: u64,
    presentation_handle: PersonalAssistantV0PresentationHandle,
    phase: PresentationPhase,
    accepted_text: String,
    accepted_characters: usize,
    accepted_bytes: usize,
    journal: Vec<PersonalAssistantV0Update>,
    deadlines: SessionDeadlines,
    next_runtime_sequence: u32,
    #[cfg(test)]
    last_expired_deadline: Option<DeadlineKind>,
}

impl SessionRecord {
    fn new(generation: u64, now: Duration) -> Result<Self, PersonalAssistantV0Error> {
        Ok(Self {
            generation,
            presentation_handle: PersonalAssistantV0PresentationHandle::for_generation(generation)?,
            phase: PresentationPhase::Starting,
            accepted_text: String::new(),
            accepted_characters: 0,
            accepted_bytes: 0,
            journal: Vec::with_capacity(MAX_PRESENTATION_UPDATES),
            deadlines: SessionDeadlines::new(now)?,
            next_runtime_sequence: 0,
            #[cfg(test)]
            last_expired_deadline: None,
        })
    }

    fn sequence(&self) -> u64 {
        match self.journal.last() {
            Some(update) => update.sequence(),
            None => 0,
        }
    }

    fn snapshot(&self) -> PersonalAssistantV0Snapshot {
        let sequence = self.sequence();
        match &self.phase {
            PresentationPhase::Starting => PersonalAssistantV0Snapshot::Starting,
            PresentationPhase::Streaming => {
                PersonalAssistantV0Snapshot::Streaming(PersonalAssistantV0TextSnapshot {
                    sequence,
                    accepted_text: self.accepted_text.clone(),
                })
            }
            PresentationPhase::Cancelling => {
                PersonalAssistantV0Snapshot::Cancelling(PersonalAssistantV0TextSnapshot {
                    sequence,
                    accepted_text: self.accepted_text.clone(),
                })
            }
            PresentationPhase::Completed => {
                PersonalAssistantV0Snapshot::Completed(PersonalAssistantV0CompletedSnapshot {
                    sequence,
                    accepted_text: self.accepted_text.clone(),
                    final_answer: self.accepted_text.clone(),
                })
            }
            PresentationPhase::Failed(failure) => {
                PersonalAssistantV0Snapshot::Failed(PersonalAssistantV0FailedSnapshot {
                    sequence,
                    accepted_text: self.accepted_text.clone(),
                    failure: failure.clone(),
                })
            }
            PresentationPhase::Cancelled => {
                PersonalAssistantV0Snapshot::Cancelled(PersonalAssistantV0TextSnapshot {
                    sequence,
                    accepted_text: self.accepted_text.clone(),
                })
            }
        }
    }

    fn expired_deadline(&self, now: Duration) -> Option<DeadlineKind> {
        self.deadlines.expired(now, &self.phase)
    }

    fn runtime_state_matches(&self, status: RuntimeRunStatus) -> bool {
        matches!(
            (&self.phase, status),
            (PresentationPhase::Starting, RuntimeRunStatus::AwaitingStart)
                | (PresentationPhase::Streaming, RuntimeRunStatus::Streaming)
        )
    }

    fn note_expired_deadline(&mut self, deadline: DeadlineKind) {
        #[cfg(test)]
        {
            self.last_expired_deadline = Some(deadline);
        }
        #[cfg(not(test))]
        {
            let _ = deadline;
        }
    }

    fn prepare_terminal(
        &self,
        intent: TerminalIntent,
    ) -> Result<PreparedTerminal, PersonalAssistantV0Error> {
        let sequence = self.next_presentation_sequence()?;
        match intent {
            TerminalIntent::Completed => Ok(PreparedTerminal {
                prepared: self.prepare_update(
                    PersonalAssistantV0Update::Completed(PersonalAssistantV0CompletedUpdate {
                        sequence,
                        final_answer: self.accepted_text.clone(),
                    }),
                    self.current_commit_metadata(),
                )?,
            }),
            TerminalIntent::Failed(failure) => Ok(PreparedTerminal {
                prepared: self.prepare_update(
                    PersonalAssistantV0Update::Failed(PersonalAssistantV0FailedUpdate {
                        sequence,
                        failure,
                    }),
                    self.current_commit_metadata(),
                )?,
            }),
            TerminalIntent::Cancelled => Ok(PreparedTerminal {
                prepared: self.prepare_update(
                    PersonalAssistantV0Update::Cancelled(PersonalAssistantV0SequenceUpdate {
                        sequence,
                    }),
                    self.current_commit_metadata(),
                )?,
            }),
        }
    }

    fn next_presentation_sequence(&self) -> Result<u64, PersonalAssistantV0Error> {
        if self.journal.len() >= MAX_PRESENTATION_UPDATES {
            return Err(PersonalAssistantV0Error::Internal);
        }
        self.sequence()
            .checked_add(1)
            .filter(|sequence| *sequence <= MAX_PRESENTATION_SEQUENCE)
            .ok_or(PersonalAssistantV0Error::Internal)
    }

    fn current_commit_metadata(&self) -> RecordCommitMetadata {
        RecordCommitMetadata {
            next_runtime_sequence: self.next_runtime_sequence,
            idle_deadline: self.deadlines.idle,
            output_characters: self.accepted_characters,
            output_bytes: self.accepted_bytes,
        }
    }

    fn prepare_update(
        &self,
        update: PersonalAssistantV0Update,
        metadata: RecordCommitMetadata,
    ) -> Result<PreparedRecordUpdate, PersonalAssistantV0Error> {
        if update.sequence() != self.next_presentation_sequence()? {
            return Err(PersonalAssistantV0Error::Internal);
        }
        match &update {
            PersonalAssistantV0Update::Started(_) => {
                if !matches!(self.phase, PresentationPhase::Starting)
                    || !self.accepted_text.is_empty()
                    || metadata.output_characters != 0
                    || metadata.output_bytes != 0
                {
                    return Err(PersonalAssistantV0Error::ProtocolViolation);
                }
            }
            PersonalAssistantV0Update::TextDelta(delta) => {
                if !matches!(self.phase, PresentationPhase::Streaming) || delta.text.is_empty() {
                    return Err(PersonalAssistantV0Error::ProtocolViolation);
                }
                let delta_characters = delta.text.chars().count();
                let expected_characters = self
                    .accepted_characters
                    .checked_add(delta_characters)
                    .ok_or(PersonalAssistantV0Error::LimitExceeded)?;
                let expected_bytes = self
                    .accepted_bytes
                    .checked_add(delta.text.len())
                    .ok_or(PersonalAssistantV0Error::LimitExceeded)?;
                if delta_characters > MAX_DELTA_CHARACTERS
                    || delta.text.len() > MAX_DELTA_BYTES
                    || metadata.output_characters != expected_characters
                    || metadata.output_bytes != expected_bytes
                    || expected_characters > MAX_OUTPUT_CHARACTERS
                    || expected_bytes > MAX_OUTPUT_BYTES
                {
                    return Err(PersonalAssistantV0Error::LimitExceeded);
                }
            }
            PersonalAssistantV0Update::Completed(completed) => {
                if !matches!(self.phase, PresentationPhase::Streaming)
                    || self.accepted_text.is_empty()
                    || completed.final_answer != self.accepted_text
                {
                    return Err(PersonalAssistantV0Error::ProtocolViolation);
                }
            }
            PersonalAssistantV0Update::Failed(_) | PersonalAssistantV0Update::Cancelled(_) => {}
        }
        Ok(PreparedRecordUpdate { update, metadata })
    }

    fn commit_terminal(&mut self, terminal: PreparedTerminal) {
        self.commit_update(terminal.prepared);
    }

    fn commit_update(&mut self, prepared: PreparedRecordUpdate) {
        let PreparedRecordUpdate { update, metadata } = prepared;
        debug_assert!(self.journal.len() < MAX_PRESENTATION_UPDATES);
        debug_assert!(update.sequence() <= MAX_PRESENTATION_SEQUENCE);
        self.next_runtime_sequence = metadata.next_runtime_sequence;
        self.deadlines.idle = metadata.idle_deadline;
        match &update {
            PersonalAssistantV0Update::Started(_) => {
                self.phase = PresentationPhase::Streaming;
            }
            PersonalAssistantV0Update::TextDelta(delta) => {
                self.phase = PresentationPhase::Streaming;
                self.accepted_text.push_str(&delta.text);
                self.accepted_characters = metadata.output_characters;
                self.accepted_bytes = metadata.output_bytes;
            }
            PersonalAssistantV0Update::Completed(completed) => {
                debug_assert_eq!(completed.final_answer, self.accepted_text);
                self.phase = PresentationPhase::Completed;
            }
            PersonalAssistantV0Update::Failed(failed) => {
                self.phase = PresentationPhase::Failed(failed.failure.clone());
            }
            PersonalAssistantV0Update::Cancelled(_) => {
                self.phase = PresentationPhase::Cancelled;
            }
        }
        self.journal.push(update);
    }
}

enum PresentationPhase {
    Starting,
    Streaming,
    Cancelling,
    Completed,
    Failed(PersonalAssistantV0Failure),
    Cancelled,
}

impl PresentationPhase {
    const fn label(&self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Streaming => "streaming",
            Self::Cancelling => "cancelling",
            Self::Completed => "completed",
            Self::Failed(_) => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    fn is_streaming(&self) -> bool {
        matches!(self, Self::Streaming)
    }
}

enum TerminalIntent {
    Completed,
    Failed(PersonalAssistantV0Failure),
    Cancelled,
}

struct PreparedTerminal {
    prepared: PreparedRecordUpdate,
}

struct PreparedRecordUpdate {
    update: PersonalAssistantV0Update,
    metadata: RecordCommitMetadata,
}

#[derive(Clone, Copy)]
struct RecordCommitMetadata {
    next_runtime_sequence: u32,
    idle_deadline: Duration,
    output_characters: usize,
    output_bytes: usize,
}

#[derive(Clone, Copy)]
enum CleanupExpectation {
    CancelledByHost,
    AlreadyTerminal(RuntimeRunStatus),
}

enum CleanupResolution {
    Expected,
    ProvedUnexpected,
    Ambiguous,
}

#[cfg(test)]
#[derive(Clone, Copy)]
enum FixtureEvent {
    Started,
    TextDelta(FixtureDelta),
    Completed,
    Failed(FixtureFailure),
    RetryableFailure,
    ForeignSession,
    StaleSequence,
    SequenceGap,
}

#[cfg(test)]
#[derive(Clone, Copy)]
enum FixtureDelta {
    BoardPrefix,
    BoardSuffix,
    Unit,
    MaximumUnicode,
    Oversized,
    Empty,
    Canary,
}

#[cfg(test)]
impl FixtureDelta {
    fn text(self) -> String {
        match self {
            Self::BoardPrefix => "Board ".to_owned(),
            Self::BoardSuffix => "update ready.".to_owned(),
            Self::Unit => "x".to_owned(),
            Self::MaximumUnicode => "🙂".repeat(MAX_DELTA_CHARACTERS),
            Self::Oversized => "x".repeat(MAX_DELTA_CHARACTERS + 1),
            Self::Empty => String::new(),
            Self::Canary => "V0-2-SYNTHETIC-OUTPUT-CANARY".to_owned(),
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy)]
enum FixtureFailure {
    Unauthenticated,
    Forbidden,
    RateLimited,
    RequestRejected,
    ProviderUnavailable,
    ProviderTimeout,
    ProtocolViolation,
    LimitExceeded,
    Internal,
}

#[cfg(test)]
impl FixtureFailure {
    const fn runtime_code(self) -> RuntimeFailureCode {
        match self {
            Self::Unauthenticated => RuntimeFailureCode::Unauthenticated,
            Self::Forbidden => RuntimeFailureCode::Forbidden,
            Self::RateLimited => RuntimeFailureCode::RateLimited,
            Self::RequestRejected => RuntimeFailureCode::RequestRejected,
            Self::ProviderUnavailable => RuntimeFailureCode::ProviderUnavailable,
            Self::ProviderTimeout => RuntimeFailureCode::ProviderTimeout,
            Self::ProtocolViolation => RuntimeFailureCode::ProtocolViolation,
            Self::LimitExceeded => RuntimeFailureCode::LimitExceeded,
            Self::Internal => RuntimeFailureCode::Internal,
        }
    }

    const fn presentation_code(self) -> PersonalAssistantV0FailureCode {
        match self {
            Self::Unauthenticated => PersonalAssistantV0FailureCode::Unauthenticated,
            Self::Forbidden => PersonalAssistantV0FailureCode::Forbidden,
            Self::RateLimited => PersonalAssistantV0FailureCode::RateLimited,
            Self::RequestRejected => PersonalAssistantV0FailureCode::RequestRejected,
            Self::ProviderUnavailable => PersonalAssistantV0FailureCode::ProviderUnavailable,
            Self::ProviderTimeout => PersonalAssistantV0FailureCode::ProviderTimeout,
            Self::ProtocolViolation => PersonalAssistantV0FailureCode::ProtocolViolation,
            Self::LimitExceeded => PersonalAssistantV0FailureCode::LimitExceeded,
            Self::Internal => PersonalAssistantV0FailureCode::Internal,
        }
    }
}

#[cfg(test)]
enum FixturePreparation {
    Accept(Box<PreparedFixture>),
    CloseForJournalLimit,
}

struct PreparedFixture {
    envelope: RuntimeEventEnvelope,
    commit: FixtureCommit,
}

fn prepare_direct_event(
    record: &SessionRecord,
    identity: &RuntimeRunIdentity,
    now: Duration,
    event: crate::personal_assistant_direct::ProviderEvent,
) -> Result<PreparedFixture, PersonalAssistantV0Error> {
    use crate::personal_assistant_direct::ProviderEvent;
    let sequence = record.next_presentation_sequence()?;
    let next_runtime_sequence = record
        .next_runtime_sequence
        .checked_add(1)
        .ok_or(PersonalAssistantV0Error::LimitExceeded)?;
    let idle_deadline = SessionDeadlines::refreshed_idle(now)?;
    let (event, commit) = match event {
        ProviderEvent::Started(response_id)
            if matches!(record.phase, PresentationPhase::Starting) =>
        {
            let runtime_id = RuntimeResponseId::new(response_id.clone())
                .map_err(|_| PersonalAssistantV0Error::ProtocolViolation)?;
            (
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: runtime_id,
                },
                FixtureCommit::Started {
                    sequence,
                    next_runtime_sequence,
                    response_id,
                    idle_deadline,
                },
            )
        }
        ProviderEvent::Delta(text) if record.phase.is_streaming() => {
            let output_characters = record
                .accepted_characters
                .saturating_add(text.chars().count());
            let output_bytes = record.accepted_bytes.saturating_add(text.len());
            if text.is_empty()
                || text.chars().count() > MAX_DELTA_CHARACTERS
                || text.len() > MAX_DELTA_BYTES
                || output_characters > MAX_OUTPUT_CHARACTERS
                || output_bytes > MAX_OUTPUT_BYTES
                || sequence >= MAX_PRESENTATION_SEQUENCE
            {
                return Err(PersonalAssistantV0Error::LimitExceeded);
            }
            let delta = RuntimeOutputText::new(text.clone())
                .map_err(|_| PersonalAssistantV0Error::ProtocolViolation)?;
            (
                UntrustedRuntimeEvent::OutputTextDelta { delta },
                FixtureCommit::Delta {
                    sequence,
                    next_runtime_sequence,
                    text,
                    output_characters,
                    output_bytes,
                    idle_deadline,
                },
            )
        }
        ProviderEvent::Completed
            if record.phase.is_streaming() && !record.accepted_text.is_empty() =>
        {
            (
                UntrustedRuntimeEvent::ResponseCompleted,
                FixtureCommit::Completed {
                    next_runtime_sequence,
                },
            )
        }
        _ => return Err(PersonalAssistantV0Error::ProtocolViolation),
    };
    Ok(PreparedFixture {
        envelope: RuntimeEventEnvelope::for_identity(identity, record.next_runtime_sequence, event),
        commit,
    })
}

enum FixtureCommit {
    Started {
        sequence: u64,
        next_runtime_sequence: u32,
        response_id: String,
        idle_deadline: Duration,
    },
    Delta {
        sequence: u64,
        next_runtime_sequence: u32,
        text: String,
        output_characters: usize,
        output_bytes: usize,
        idle_deadline: Duration,
    },
    Completed {
        next_runtime_sequence: u32,
    },
    #[cfg(test)]
    Failed {
        next_runtime_sequence: u32,
        runtime_code: RuntimeFailureCode,
        failure: PersonalAssistantV0Failure,
    },
    #[cfg(test)]
    ExpectedProtocolFailure {
        failure: PersonalAssistantV0Failure,
    },
}

#[cfg(test)]
fn prepare_fixture_event(
    record: &SessionRecord,
    identity: &RuntimeRunIdentity,
    now: Duration,
    event: FixtureEvent,
) -> Result<FixturePreparation, PersonalAssistantV0Error> {
    let (event_identity, event_sequence) = match event {
        FixtureEvent::ForeignSession => (
            RuntimeTurnRequest::new(
                "pa-v0-foreign-fixture-run",
                "pa-v0-foreign-fixture-request",
                "synthetic",
            )
            .map_err(|_| PersonalAssistantV0Error::Internal)?
            .identity(),
            record.next_runtime_sequence,
        ),
        FixtureEvent::StaleSequence => (
            identity.clone(),
            record.next_runtime_sequence.saturating_sub(1),
        ),
        FixtureEvent::SequenceGap => (
            identity.clone(),
            record
                .next_runtime_sequence
                .checked_add(1)
                .ok_or(PersonalAssistantV0Error::Internal)?,
        ),
        FixtureEvent::Started
        | FixtureEvent::TextDelta(_)
        | FixtureEvent::Completed
        | FixtureEvent::Failed(_)
        | FixtureEvent::RetryableFailure => (identity.clone(), record.next_runtime_sequence),
    };
    validate_fixture_event_binding(
        identity,
        record.next_runtime_sequence,
        &event_identity,
        event_sequence,
    )?;

    let current_sequence = record.sequence();
    let current_length =
        usize::try_from(current_sequence).map_err(|_| PersonalAssistantV0Error::Internal)?;
    if record.journal.len() != current_length
        || u64::from(record.next_runtime_sequence) != current_sequence
    {
        return Err(PersonalAssistantV0Error::Internal);
    }
    let sequence = current_sequence
        .checked_add(1)
        .filter(|sequence| *sequence <= MAX_PRESENTATION_SEQUENCE)
        .ok_or(PersonalAssistantV0Error::Internal)?;
    let next_runtime_sequence = record
        .next_runtime_sequence
        .checked_add(1)
        .ok_or(PersonalAssistantV0Error::Internal)?;
    let nonterminal = matches!(event, FixtureEvent::Started | FixtureEvent::TextDelta(_));
    if nonterminal && sequence == MAX_PRESENTATION_SEQUENCE {
        return Ok(FixturePreparation::CloseForJournalLimit);
    }

    let runtime_event = match event {
        FixtureEvent::Started => {
            if !matches!(record.phase, PresentationPhase::Starting)
                || record.next_runtime_sequence != 0
            {
                return Err(PersonalAssistantV0Error::ProtocolViolation);
            }
            let response_id = format!("pa-v0-fixture-response-{:016x}", record.generation);
            let runtime_response_id = RuntimeResponseId::new(response_id.clone())
                .map_err(|_| PersonalAssistantV0Error::Internal)?;
            let idle_deadline = SessionDeadlines::refreshed_idle(now)?;
            let envelope = RuntimeEventEnvelope::for_identity(
                identity,
                record.next_runtime_sequence,
                UntrustedRuntimeEvent::ResponseStarted {
                    response_id: runtime_response_id,
                },
            );
            return Ok(FixturePreparation::Accept(Box::new(PreparedFixture {
                envelope,
                commit: FixtureCommit::Started {
                    sequence,
                    next_runtime_sequence,
                    response_id,
                    idle_deadline,
                },
            })));
        }
        FixtureEvent::TextDelta(delta) => {
            if !matches!(record.phase, PresentationPhase::Streaming) {
                return Err(PersonalAssistantV0Error::ProtocolViolation);
            }
            let text = delta.text();
            if text.is_empty() {
                return Err(PersonalAssistantV0Error::ProtocolViolation);
            }
            let delta_characters = text.chars().count();
            if delta_characters > MAX_DELTA_CHARACTERS || text.len() > MAX_DELTA_BYTES {
                return Err(PersonalAssistantV0Error::LimitExceeded);
            }
            let output_characters = record
                .accepted_characters
                .checked_add(delta_characters)
                .ok_or(PersonalAssistantV0Error::LimitExceeded)?;
            let output_bytes = record
                .accepted_bytes
                .checked_add(text.len())
                .ok_or(PersonalAssistantV0Error::LimitExceeded)?;
            if output_characters > MAX_OUTPUT_CHARACTERS || output_bytes > MAX_OUTPUT_BYTES {
                return Err(PersonalAssistantV0Error::LimitExceeded);
            }
            let runtime_text = RuntimeOutputText::new(text.clone())
                .map_err(|_| PersonalAssistantV0Error::Internal)?;
            let idle_deadline = SessionDeadlines::refreshed_idle(now)?;
            let envelope = RuntimeEventEnvelope::for_identity(
                identity,
                record.next_runtime_sequence,
                UntrustedRuntimeEvent::OutputTextDelta {
                    delta: runtime_text,
                },
            );
            return Ok(FixturePreparation::Accept(Box::new(PreparedFixture {
                envelope,
                commit: FixtureCommit::Delta {
                    sequence,
                    next_runtime_sequence,
                    text,
                    output_characters,
                    output_bytes,
                    idle_deadline,
                },
            })));
        }
        FixtureEvent::Completed => {
            if !matches!(record.phase, PresentationPhase::Streaming)
                || record.accepted_text.is_empty()
            {
                return Err(PersonalAssistantV0Error::ProtocolViolation);
            }
            UntrustedRuntimeEvent::ResponseCompleted
        }
        FixtureEvent::Failed(failure) => {
            if !matches!(record.phase, PresentationPhase::Streaming) {
                return Err(PersonalAssistantV0Error::ProtocolViolation);
            }
            let runtime_code = failure.runtime_code();
            let runtime_failure = RuntimeFailure::new(runtime_code, false, None)
                .map_err(|_| PersonalAssistantV0Error::Internal)?;
            let envelope = RuntimeEventEnvelope::for_identity(
                identity,
                record.next_runtime_sequence,
                UntrustedRuntimeEvent::ResponseFailed {
                    failure: runtime_failure,
                },
            );
            return Ok(FixturePreparation::Accept(Box::new(PreparedFixture {
                envelope,
                commit: FixtureCommit::Failed {
                    next_runtime_sequence,
                    runtime_code,
                    failure: PersonalAssistantV0Failure::closed(
                        failure.presentation_code(),
                        record.generation,
                    ),
                },
            })));
        }
        FixtureEvent::RetryableFailure => {
            if !matches!(record.phase, PresentationPhase::Streaming) {
                return Err(PersonalAssistantV0Error::ProtocolViolation);
            }
            let runtime_failure =
                RuntimeFailure::new(RuntimeFailureCode::ProviderTimeout, true, None)
                    .map_err(|_| PersonalAssistantV0Error::Internal)?;
            let envelope = RuntimeEventEnvelope::for_identity(
                identity,
                record.next_runtime_sequence,
                UntrustedRuntimeEvent::ResponseFailed {
                    failure: runtime_failure,
                },
            );
            return Ok(FixturePreparation::Accept(Box::new(PreparedFixture {
                envelope,
                commit: FixtureCommit::ExpectedProtocolFailure {
                    failure: PersonalAssistantV0Failure::closed(
                        PersonalAssistantV0FailureCode::ProtocolViolation,
                        record.generation,
                    ),
                },
            })));
        }
        FixtureEvent::ForeignSession | FixtureEvent::StaleSequence | FixtureEvent::SequenceGap => {
            return Err(PersonalAssistantV0Error::Internal);
        }
    };

    let envelope =
        RuntimeEventEnvelope::for_identity(identity, record.next_runtime_sequence, runtime_event);
    let commit = match event {
        FixtureEvent::Completed => FixtureCommit::Completed {
            next_runtime_sequence,
        },
        _ => return Err(PersonalAssistantV0Error::Internal),
    };
    Ok(FixturePreparation::Accept(Box::new(PreparedFixture {
        envelope,
        commit,
    })))
}

#[cfg(test)]
fn validate_fixture_event_binding(
    expected_identity: &RuntimeRunIdentity,
    expected_sequence: u32,
    event_identity: &RuntimeRunIdentity,
    event_sequence: u32,
) -> Result<(), PersonalAssistantV0Error> {
    if event_identity != expected_identity || event_sequence != expected_sequence {
        Err(PersonalAssistantV0Error::ProtocolViolation)
    } else {
        Ok(())
    }
}

fn fixture_acceptance_matches(
    acceptance: &Result<RuntimeEventAcceptance, RuntimeError>,
    commit: &FixtureCommit,
    actual_status: RuntimeRunStatus,
) -> bool {
    let expected_status = match commit {
        FixtureCommit::Started { .. } | FixtureCommit::Delta { .. } => RuntimeRunStatus::Streaming,
        FixtureCommit::Completed { .. } => RuntimeRunStatus::Completed,
        #[cfg(test)]
        FixtureCommit::Failed { .. } | FixtureCommit::ExpectedProtocolFailure { .. } => {
            RuntimeRunStatus::Failed
        }
    };
    if actual_status != expected_status {
        return false;
    }
    match (acceptance, commit) {
        (
            Ok(RuntimeEventAcceptance::ResponseStarted {
                response_id: actual,
            }),
            FixtureCommit::Started { response_id, .. },
        ) => actual.as_str() == response_id,
        (
            Ok(RuntimeEventAcceptance::OutputTextDelta { delta: actual }),
            FixtureCommit::Delta { text, .. },
        ) => actual.as_str() == text,
        (Ok(RuntimeEventAcceptance::ResponseCompleted), FixtureCommit::Completed { .. }) => true,
        #[cfg(test)]
        (
            Ok(RuntimeEventAcceptance::ResponseFailed { failure: actual }),
            FixtureCommit::Failed { runtime_code, .. },
        ) => {
            actual.code() == *runtime_code
                && !actual.retryable()
                && actual.retry_after_ms().is_none()
        }
        #[cfg(test)]
        (
            Err(RuntimeError::EventRejected(RuntimeEventRejection::InvalidContent)),
            FixtureCommit::ExpectedProtocolFailure { .. },
        ) => true,
        _ => false,
    }
}

#[derive(Clone, Copy)]
enum CleanupDisposition {
    NotRequired,
}

struct SessionDeadlines {
    connect: Duration,
    idle: Duration,
    provider: Duration,
    total: Duration,
}

impl SessionDeadlines {
    fn new(start: Duration) -> Result<Self, PersonalAssistantV0Error> {
        Ok(Self {
            connect: start
                .checked_add(GATEWAY_CONNECT_TIMEOUT)
                .ok_or(PersonalAssistantV0Error::Internal)?,
            idle: start
                .checked_add(GATEWAY_STREAM_IDLE_TIMEOUT)
                .ok_or(PersonalAssistantV0Error::Internal)?,
            provider: start
                .checked_add(PROVIDER_TURN_DEADLINE)
                .ok_or(PersonalAssistantV0Error::Internal)?,
            total: start
                .checked_add(AGENT_RUN_DEADLINE)
                .ok_or(PersonalAssistantV0Error::Internal)?,
        })
    }

    fn expired(&self, now: Duration, phase: &PresentationPhase) -> Option<DeadlineKind> {
        if now >= self.total {
            return Some(DeadlineKind::Total);
        }
        if now >= self.provider {
            return Some(DeadlineKind::Provider);
        }
        if matches!(phase, PresentationPhase::Starting) && now >= self.connect {
            return Some(DeadlineKind::Connect);
        }
        if phase.is_streaming() && now >= self.idle {
            return Some(DeadlineKind::Idle);
        }
        None
    }

    fn refreshed_idle(now: Duration) -> Result<Duration, PersonalAssistantV0Error> {
        now.checked_add(GATEWAY_STREAM_IDLE_TIMEOUT)
            .ok_or(PersonalAssistantV0Error::Internal)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeadlineKind {
    Total,
    Provider,
    Connect,
    Idle,
}

enum MonotonicClock {
    System {
        origin: Instant,
    },
    #[cfg(test)]
    Manual(ManualClock),
}

impl MonotonicClock {
    fn system() -> Self {
        Self::System {
            origin: Instant::now(),
        }
    }

    fn sample(&self) -> Duration {
        match self {
            Self::System { origin } => origin.elapsed(),
            #[cfg(test)]
            Self::Manual(clock) => clock.sample(),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct ManualClock {
    milliseconds: Arc<AtomicU64>,
}

#[cfg(test)]
impl ManualClock {
    fn new() -> Self {
        Self {
            milliseconds: Arc::new(AtomicU64::new(0)),
        }
    }

    fn advance_to(&self, value: Duration) -> Result<(), PersonalAssistantV0Error> {
        let milliseconds =
            u64::try_from(value.as_millis()).map_err(|_| PersonalAssistantV0Error::Internal)?;
        self.milliseconds
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                (milliseconds >= current).then_some(milliseconds)
            })
            .map(|_| ())
            .map_err(|_| PersonalAssistantV0Error::InvalidRequest)
    }

    fn sample(&self) -> Duration {
        Duration::from_millis(self.milliseconds.load(Ordering::Acquire))
    }
}

struct OwnedRun<R: RuntimeRun> {
    run: R,
    expected_identity: RuntimeRunIdentity,
    _lease: ProcessLease,
}

type OwnedNativeRun = OwnedRun<NativeAgentRun>;

struct ProcessLease;

impl ProcessLease {
    fn acquire() -> Result<Self, PersonalAssistantV0Error> {
        if try_acquire_lease(&PROCESS_LEASE_HELD) {
            Ok(Self)
        } else {
            Err(PersonalAssistantV0Error::Busy)
        }
    }
}

impl Drop for ProcessLease {
    fn drop(&mut self) {
        PROCESS_LEASE_HELD.store(false, Ordering::Release);
    }
}

enum ReturnedRunCheck<R> {
    Accepted(R),
    RejectedClean(PersonalAssistantV0Error),
    RejectedAmbiguous {
        run: R,
        error: PersonalAssistantV0Error,
    },
}

fn start_checked<R: AgentRuntime>(
    runtime: &R,
    request: RuntimeTurnRequest,
    expected_identity: &RuntimeRunIdentity,
) -> Result<ReturnedRunCheck<R::Run>, RuntimeError> {
    let run = runtime.start(request)?;
    Ok(check_returned_run(run, expected_identity))
}

fn check_returned_run<R: RuntimeRun>(
    mut run: R,
    expected_identity: &RuntimeRunIdentity,
) -> ReturnedRunCheck<R> {
    let rejection =
        if run.identity() != expected_identity || run.status() != RuntimeRunStatus::AwaitingStart {
            Some(PersonalAssistantV0Error::Internal)
        } else {
            None
        };

    let Some(error) = rejection else {
        return ReturnedRunCheck::Accepted(run);
    };
    let returned_identity = run.identity().clone();
    let cleanup = run.cancel();
    if cleanup_is_proved_for_identity(&run, &returned_identity, &cleanup) {
        ReturnedRunCheck::RejectedClean(error)
    } else {
        ReturnedRunCheck::RejectedAmbiguous { run, error }
    }
}

fn cleanup_is_proved_for_identity<R: RuntimeRun>(
    run: &R,
    identity_before_cleanup: &RuntimeRunIdentity,
    outcome: &Result<RuntimeCancellationOutcome, RuntimeError>,
) -> bool {
    run.identity() == identity_before_cleanup
        && cleanup_outcome_matches_status(run.status(), outcome)
}

fn cleanup_outcome_matches_status(
    actual_status: RuntimeRunStatus,
    outcome: &Result<RuntimeCancellationOutcome, RuntimeError>,
) -> bool {
    match outcome {
        Ok(RuntimeCancellationOutcome::Cancelled) => actual_status == RuntimeRunStatus::Cancelled,
        Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) => {
            status.is_terminal() && actual_status == *status
        }
        Err(_) => false,
    }
}

fn classify_cleanup<R: RuntimeRun>(
    run: &R,
    expected_identity: &RuntimeRunIdentity,
    outcome: &Result<RuntimeCancellationOutcome, RuntimeError>,
    expectation: CleanupExpectation,
) -> CleanupResolution {
    if run.identity() != expected_identity || !cleanup_outcome_matches_status(run.status(), outcome)
    {
        return CleanupResolution::Ambiguous;
    }
    match outcome {
        Ok(RuntimeCancellationOutcome::Cancelled)
            if matches!(expectation, CleanupExpectation::CancelledByHost) =>
        {
            CleanupResolution::Expected
        }
        Ok(RuntimeCancellationOutcome::AlreadyTerminal(actual))
            if matches!(
                expectation,
                CleanupExpectation::AlreadyTerminal(expected) if actual == &expected
            ) =>
        {
            CleanupResolution::Expected
        }
        Ok(RuntimeCancellationOutcome::Cancelled)
        | Ok(RuntimeCancellationOutcome::AlreadyTerminal(
            RuntimeRunStatus::Completed | RuntimeRunStatus::Failed | RuntimeRunStatus::Cancelled,
        )) => CleanupResolution::ProvedUnexpected,
        Ok(RuntimeCancellationOutcome::AlreadyTerminal(
            RuntimeRunStatus::AwaitingStart | RuntimeRunStatus::Streaming,
        ))
        | Err(_) => CleanupResolution::Ambiguous,
    }
}

enum QuarantinePayload<R: RuntimeRun> {
    Rejected(OwnedRun<R>),
    Session {
        record: SessionRecord,
        owner: OwnedRun<R>,
    },
}

enum NativeQuarantine {
    Rejected(OwnedNativeRun),
    Session {
        record: SessionRecord,
        owner: OwnedNativeRun,
    },
}

impl Drop for NativeQuarantine {
    fn drop(&mut self) {
        match self {
            Self::Rejected(owner) => {
                let _ = owner.run.status();
            }
            Self::Session { record, owner } => {
                record.accepted_text.clear();
                let _ = owner.run.status();
            }
        }
    }
}

fn cleanup_state_for_drop<R: RuntimeRun>(state: HostState<R>) -> Option<QuarantinePayload<R>> {
    let (record, mut owner, expectation) = match state {
        HostState::Idle | HostState::Terminal(_) => return None,
        HostState::Quarantined(owner) => {
            let cleanup = owner_cleanup(owner);
            return cleanup.map(QuarantinePayload::Rejected);
        }
        HostState::Active(active) => (
            active.record,
            active.owner,
            CleanupExpectation::CancelledByHost,
        ),
        HostState::Cancelling(cancelling) => (
            cancelling.record,
            cancelling.owner,
            cancelling.cleanup_expectation,
        ),
    };
    let cleanup = owner.run.cancel();
    match classify_cleanup(&owner.run, &owner.expected_identity, &cleanup, expectation) {
        CleanupResolution::Expected | CleanupResolution::ProvedUnexpected => None,
        CleanupResolution::Ambiguous => Some(QuarantinePayload::Session { record, owner }),
    }
}

fn owner_cleanup<R: RuntimeRun>(mut owner: OwnedRun<R>) -> Option<OwnedRun<R>> {
    let identity_before_cleanup = owner.run.identity().clone();
    let cleanup = owner.run.cancel();
    if cleanup_is_proved_for_identity(&owner.run, &identity_before_cleanup, &cleanup) {
        None
    } else {
        Some(owner)
    }
}

fn quarantine_or_leak<T>(value: T, slot: &Mutex<Option<T>>) {
    match slot.lock() {
        Ok(mut guard) if guard.is_none() => {
            *guard = Some(value);
        }
        Ok(_) | Err(_) => mem::forget(value),
    }
}

fn try_acquire_lease(flag: &AtomicBool) -> bool {
    flag.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
}

fn reserve_generation(counter: &AtomicU64) -> Result<u64, PersonalAssistantV0Error> {
    counter
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            if current == 0 {
                None
            } else {
                current.checked_add(1)
            }
        })
        .map_err(|_| PersonalAssistantV0Error::Internal)
}

fn map_start_error(error: RuntimeError) -> PersonalAssistantV0Error {
    match error {
        RuntimeError::InvalidRequest(RuntimeInvalidRequest::SelectedContentTooLarge { .. })
        | RuntimeError::InvalidRequest(RuntimeInvalidRequest::SerializedRequestTooLarge {
            ..
        }) => PersonalAssistantV0Error::LimitExceeded,
        RuntimeError::InvalidRequest(_) => PersonalAssistantV0Error::Internal,
        RuntimeError::InvalidEvent(_)
        | RuntimeError::EventRejected(_)
        | RuntimeError::CapabilityUnavailable(_)
        | RuntimeError::Unavailable
        | RuntimeError::BoundaryFailure(_) => PersonalAssistantV0Error::Internal,
    }
}

fn is_valid_correlation(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_CORRELATION_BYTES
        && value.bytes().all(|byte| (0x21..=0x7e).contains(&byte))
}

#[cfg(test)]
pub(crate) mod tests {
    use std::error::Error;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::atomic::AtomicUsize;
    use std::sync::{Arc, Mutex, MutexGuard};

    use super::*;

    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    pub(crate) fn serial_guard() -> Result<MutexGuard<'static, ()>, Box<dyn Error>> {
        match TEST_SERIAL.lock() {
            Ok(guard) => Ok(guard),
            Err(poisoned) => Ok(poisoned.into_inner()),
        }
    }

    fn native_host() -> (HostCore<NativeAgentRuntime>, ManualClock) {
        let clock = ManualClock::new();
        (
            HostCore::new(NativeAgentRuntime, MonotonicClock::Manual(clock.clone())),
            clock,
        )
    }

    fn start_handle(
        host: &mut HostCore<NativeAgentRuntime>,
    ) -> Result<PersonalAssistantV0PresentationHandle, PersonalAssistantV0Error> {
        Ok(host.start_synthetic()?.presentation_handle().clone())
    }

    fn terminal_failure(
        snapshot: &PersonalAssistantV0Snapshot,
    ) -> Result<&PersonalAssistantV0Failure, Box<dyn Error>> {
        let PersonalAssistantV0Snapshot::Failed(failed) = snapshot else {
            return Err("expected failed snapshot".into());
        };
        Ok(failed.failure())
    }

    #[test]
    fn fixture_success_is_chronological_bounded_and_exact() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let (mut host, _clock) = native_host();
        let handle = start_handle(&mut host)?;

        assert!(matches!(
            host.accept_fixture_event(&handle, FixtureEvent::Started)?,
            PersonalAssistantV0Snapshot::Streaming(_)
        ));
        host.accept_fixture_event(&handle, FixtureEvent::TextDelta(FixtureDelta::BoardPrefix))?;
        host.accept_fixture_event(&handle, FixtureEvent::TextDelta(FixtureDelta::BoardSuffix))?;
        let completed = host.accept_fixture_event(&handle, FixtureEvent::Completed)?;
        let PersonalAssistantV0Snapshot::Completed(completed_detail) = &completed else {
            return Err("expected completed snapshot".into());
        };
        assert_eq!(completed.sequence(), 4);
        assert_eq!(completed_detail.accepted_text(), "Board update ready.");
        assert_eq!(
            completed_detail.final_answer(),
            completed_detail.accepted_text()
        );

        let page = host.updates(&handle, Some(0))?;
        assert_eq!(
            page.updates()
                .iter()
                .map(PersonalAssistantV0Update::sequence)
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
        assert!(matches!(
            page.updates()[0],
            PersonalAssistantV0Update::Started(_)
        ));
        assert!(matches!(
            page.updates()[1],
            PersonalAssistantV0Update::TextDelta(_)
        ));
        assert!(matches!(
            page.updates()[2],
            PersonalAssistantV0Update::TextDelta(_)
        ));
        assert!(matches!(
            page.updates()[3],
            PersonalAssistantV0Update::Completed(_)
        ));
        assert_eq!(page.through_sequence(), 4);
        assert!(!page.has_more());

        let recovery = host.updates(&handle, None)?;
        assert!(recovery.updates().is_empty());
        assert_eq!(recovery.through_sequence(), 4);
        assert!(!recovery.has_more());
        assert_eq!(recovery.snapshot(), &completed);

        assert_eq!(
            host.accept_fixture_event(&handle, FixtureEvent::Completed),
            Err(PersonalAssistantV0Error::ProtocolViolation)
        );
        assert_eq!(host.cancel(&handle)?, completed);

        let restarted = host.start_synthetic()?;
        assert_ne!(restarted.presentation_handle(), &handle);
        let restarted_handle = restarted.presentation_handle().clone();
        host.cancel(&restarted_handle)?;
        Ok(())
    }

    #[test]
    fn provider_failures_map_to_closed_redacted_codes() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let cases = [
            (
                FixtureFailure::Unauthenticated,
                PersonalAssistantV0FailureCode::Unauthenticated,
            ),
            (
                FixtureFailure::Forbidden,
                PersonalAssistantV0FailureCode::Forbidden,
            ),
            (
                FixtureFailure::RateLimited,
                PersonalAssistantV0FailureCode::RateLimited,
            ),
            (
                FixtureFailure::RequestRejected,
                PersonalAssistantV0FailureCode::RequestRejected,
            ),
            (
                FixtureFailure::ProviderUnavailable,
                PersonalAssistantV0FailureCode::ProviderUnavailable,
            ),
            (
                FixtureFailure::ProviderTimeout,
                PersonalAssistantV0FailureCode::ProviderTimeout,
            ),
            (
                FixtureFailure::ProtocolViolation,
                PersonalAssistantV0FailureCode::ProtocolViolation,
            ),
            (
                FixtureFailure::LimitExceeded,
                PersonalAssistantV0FailureCode::LimitExceeded,
            ),
            (
                FixtureFailure::Internal,
                PersonalAssistantV0FailureCode::Internal,
            ),
        ];

        for (fixture, expected) in cases {
            let (mut host, _clock) = native_host();
            let handle = start_handle(&mut host)?;
            host.accept_fixture_event(&handle, FixtureEvent::Started)?;
            let failed = host.accept_fixture_event(&handle, FixtureEvent::Failed(fixture))?;
            let failure = terminal_failure(&failed)?;
            assert_eq!(failure.code(), expected);
            assert_eq!(failed.sequence(), 2);
            assert!(failed.accepted_text().is_empty());
            if expected == PersonalAssistantV0FailureCode::Internal {
                let correlation = failure
                    .support_correlation()
                    .ok_or("internal failure requires support correlation")?;
                assert!(correlation.starts_with("pa-v0-support-"));
                assert!(is_valid_correlation(correlation));
            } else {
                assert!(failure.support_correlation().is_none());
            }
            let rendered = format!("{failed:?} {failure:?}");
            assert!(!rendered.contains("pa-v0-support-"));
            assert!(!rendered.contains("pa-v0-run-"));
        }

        let (mut host, _clock) = native_host();
        let handle = start_handle(&mut host)?;
        host.accept_fixture_event(&handle, FixtureEvent::Started)?;
        let failed = host.accept_fixture_event(&handle, FixtureEvent::RetryableFailure)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::ProtocolViolation
        );
        Ok(())
    }

    #[test]
    fn cancellation_is_exact_from_starting_and_streaming() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let (mut starting, _clock) = native_host();
        let starting_handle = start_handle(&mut starting)?;
        let cancelled = starting.cancel(&starting_handle)?;
        assert!(matches!(
            cancelled,
            PersonalAssistantV0Snapshot::Cancelled(_)
        ));
        assert_eq!(cancelled.sequence(), 1);
        assert_eq!(starting.cancel(&starting_handle)?, cancelled);
        assert_eq!(
            starting.updates(&starting_handle, Some(0))?.updates().len(),
            1
        );

        let (mut streaming, _clock) = native_host();
        let streaming_handle = start_handle(&mut streaming)?;
        streaming.accept_fixture_event(&streaming_handle, FixtureEvent::Started)?;
        streaming.accept_fixture_event(
            &streaming_handle,
            FixtureEvent::TextDelta(FixtureDelta::Canary),
        )?;
        let cancelled = streaming.cancel(&streaming_handle)?;
        assert_eq!(cancelled.sequence(), 3);
        assert_eq!(cancelled.accepted_text(), "V0-2-SYNTHETIC-OUTPUT-CANARY");
        assert_eq!(streaming.cancel(&streaming_handle)?, cancelled);
        assert_eq!(
            streaming
                .updates(&streaming_handle, Some(0))?
                .updates()
                .len(),
            3
        );
        Ok(())
    }

    #[test]
    fn polling_journal_and_text_limits_are_exact() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let (mut paged, _clock) = native_host();
        let paged_handle = start_handle(&mut paged)?;
        paged.accept_fixture_event(&paged_handle, FixtureEvent::Started)?;
        for _ in 0..19 {
            paged
                .accept_fixture_event(&paged_handle, FixtureEvent::TextDelta(FixtureDelta::Unit))?;
        }
        let first = paged.updates(&paged_handle, Some(0))?;
        assert_eq!(first.updates().len(), MAX_UPDATE_BATCH);
        assert_eq!(first.through_sequence(), 16);
        assert!(first.has_more());
        assert_eq!(first.updates()[0].sequence(), 1);
        assert_eq!(first.updates()[15].sequence(), 16);
        let second = paged.updates(&paged_handle, Some(16))?;
        assert_eq!(second.updates().len(), 4);
        assert_eq!(second.through_sequence(), 20);
        assert!(!second.has_more());
        paged.cancel(&paged_handle)?;

        let (mut bounded, _clock) = native_host();
        let bounded_handle = start_handle(&mut bounded)?;
        bounded.accept_fixture_event(&bounded_handle, FixtureEvent::Started)?;
        let before = bounded.snapshot(&bounded_handle)?;
        assert_eq!(
            bounded.accept_fixture_event(
                &bounded_handle,
                FixtureEvent::TextDelta(FixtureDelta::Oversized),
            ),
            Err(PersonalAssistantV0Error::LimitExceeded)
        );
        assert_eq!(bounded.snapshot(&bounded_handle)?, before);
        assert_eq!(
            bounded.accept_fixture_event(
                &bounded_handle,
                FixtureEvent::TextDelta(FixtureDelta::Empty),
            ),
            Err(PersonalAssistantV0Error::ProtocolViolation)
        );
        for _ in 0..8 {
            bounded.accept_fixture_event(
                &bounded_handle,
                FixtureEvent::TextDelta(FixtureDelta::MaximumUnicode),
            )?;
        }
        let maximum = bounded.snapshot(&bounded_handle)?;
        assert_eq!(
            maximum.accepted_text().chars().count(),
            MAX_OUTPUT_CHARACTERS
        );
        assert_eq!(maximum.accepted_text().len(), MAX_OUTPUT_BYTES);
        assert_eq!(
            bounded.accept_fixture_event(
                &bounded_handle,
                FixtureEvent::TextDelta(FixtureDelta::Unit),
            ),
            Err(PersonalAssistantV0Error::LimitExceeded)
        );
        assert_eq!(bounded.snapshot(&bounded_handle)?, maximum);
        let completed = bounded.accept_fixture_event(&bounded_handle, FixtureEvent::Completed)?;
        let PersonalAssistantV0Snapshot::Completed(completed) = completed else {
            return Err("expected completion at output boundary".into());
        };
        assert_eq!(
            completed.final_answer().chars().count(),
            MAX_OUTPUT_CHARACTERS
        );

        let (mut journal, _clock) = native_host();
        let journal_handle = start_handle(&mut journal)?;
        journal.accept_fixture_event(&journal_handle, FixtureEvent::Started)?;
        for _ in 0..126 {
            journal.accept_fixture_event(
                &journal_handle,
                FixtureEvent::TextDelta(FixtureDelta::Unit),
            )?;
        }
        assert_eq!(journal.snapshot(&journal_handle)?.sequence(), 127);
        let terminal = journal
            .accept_fixture_event(&journal_handle, FixtureEvent::TextDelta(FixtureDelta::Unit))?;
        assert_eq!(terminal.sequence(), 128);
        assert_eq!(
            terminal_failure(&terminal)?.code(),
            PersonalAssistantV0FailureCode::LimitExceeded
        );
        let mut cursor = 0;
        let mut observed = Vec::new();
        loop {
            let page = journal.updates(&journal_handle, Some(cursor))?;
            observed.extend(
                page.updates()
                    .iter()
                    .map(PersonalAssistantV0Update::sequence),
            );
            cursor = page.through_sequence();
            if !page.has_more() {
                break;
            }
        }
        assert_eq!(observed, (1..=128).collect::<Vec<_>>());
        assert_eq!(
            journal.accept_fixture_event(&journal_handle, FixtureEvent::Completed),
            Err(PersonalAssistantV0Error::ProtocolViolation)
        );
        assert_eq!(journal.snapshot(&journal_handle)?, terminal);
        Ok(())
    }

    #[test]
    fn closed_faults_and_late_events_do_not_mutate_state() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let (mut host, _clock) = native_host();
        let handle = start_handle(&mut host)?;
        host.accept_fixture_event(&handle, FixtureEvent::Started)?;
        let before = host.snapshot(&handle)?;
        for fault in [
            FixtureEvent::ForeignSession,
            FixtureEvent::StaleSequence,
            FixtureEvent::SequenceGap,
        ] {
            assert_eq!(
                host.accept_fixture_event(&handle, fault),
                Err(PersonalAssistantV0Error::ProtocolViolation)
            );
            assert_eq!(host.snapshot(&handle)?, before);
        }
        host.accept_fixture_event(&handle, FixtureEvent::TextDelta(FixtureDelta::BoardPrefix))?;
        let cancelled = host.cancel(&handle)?;
        for late in [
            FixtureEvent::Started,
            FixtureEvent::TextDelta(FixtureDelta::Unit),
            FixtureEvent::Completed,
            FixtureEvent::Failed(FixtureFailure::ProviderTimeout),
        ] {
            assert_eq!(
                host.accept_fixture_event(&handle, late),
                Err(PersonalAssistantV0Error::ProtocolViolation)
            );
            assert_eq!(host.snapshot(&handle)?, cancelled);
        }
        assert_eq!(host.snapshot(&handle)?, cancelled);
        Ok(())
    }

    #[test]
    fn monotonic_deadlines_are_exact_and_invalid_poll_does_not_sample() -> Result<(), Box<dyn Error>>
    {
        let _serial = serial_guard()?;

        let (mut connect, connect_clock) = native_host();
        let connect_handle = start_handle(&mut connect)?;
        connect_clock.advance_to(Duration::from_millis(9_999))?;
        assert_eq!(
            connect.snapshot(&connect_handle)?,
            PersonalAssistantV0Snapshot::Starting
        );
        connect_clock.advance_to(GATEWAY_CONNECT_TIMEOUT)?;
        assert_eq!(
            connect.updates(&connect_handle, Some(1)),
            Err(PersonalAssistantV0Error::InvalidRequest)
        );
        assert!(connect
            .current_record()
            .is_some_and(|record| record.last_expired_deadline.is_none()));
        let failed = connect.snapshot(&connect_handle)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );
        assert!(connect
            .current_record()
            .is_some_and(|record| { record.last_expired_deadline == Some(DeadlineKind::Connect) }));

        let (mut idle, idle_clock) = native_host();
        let idle_handle = start_handle(&mut idle)?;
        idle.accept_fixture_event(&idle_handle, FixtureEvent::Started)?;
        idle_clock.advance_to(Duration::from_millis(19_999))?;
        assert!(matches!(
            idle.snapshot(&idle_handle)?,
            PersonalAssistantV0Snapshot::Streaming(_)
        ));
        idle_clock.advance_to(GATEWAY_STREAM_IDLE_TIMEOUT)?;
        let failed = idle.snapshot(&idle_handle)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );
        assert!(idle
            .current_record()
            .is_some_and(|record| { record.last_expired_deadline == Some(DeadlineKind::Idle) }));

        let (mut provider, provider_clock) = native_host();
        let provider_handle = start_handle(&mut provider)?;
        provider.accept_fixture_event(&provider_handle, FixtureEvent::Started)?;
        for seconds in [19, 38, 57] {
            provider_clock.advance_to(Duration::from_secs(seconds))?;
            provider.accept_fixture_event(
                &provider_handle,
                FixtureEvent::TextDelta(FixtureDelta::Unit),
            )?;
        }
        provider_clock.advance_to(Duration::from_millis(59_999))?;
        assert!(matches!(
            provider.snapshot(&provider_handle)?,
            PersonalAssistantV0Snapshot::Streaming(_)
        ));
        provider_clock.advance_to(PROVIDER_TURN_DEADLINE)?;
        let failed = provider.snapshot(&provider_handle)?;
        assert!(provider.current_record().is_some_and(|record| {
            record.last_expired_deadline == Some(DeadlineKind::Provider)
        }));
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );

        let (mut total, total_clock) = native_host();
        let total_handle = start_handle(&mut total)?;
        total_clock.advance_to(AGENT_RUN_DEADLINE)?;
        let failed = total.snapshot(&total_handle)?;
        assert!(total
            .current_record()
            .is_some_and(|record| { record.last_expired_deadline == Some(DeadlineKind::Total) }));
        assert_eq!(failed.sequence(), 1);
        assert_eq!(total.snapshot(&total_handle)?, failed);
        assert_eq!(
            total_clock.advance_to(Duration::from_secs(119)),
            Err(PersonalAssistantV0Error::InvalidRequest)
        );
        Ok(())
    }

    #[test]
    fn exact_deadlines_win_event_and_cancellation_races() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;

        let (mut before_connect, before_connect_clock) = native_host();
        let before_connect_handle = start_handle(&mut before_connect)?;
        before_connect_clock.advance_to(Duration::from_millis(9_999))?;
        assert!(matches!(
            before_connect.accept_fixture_event(&before_connect_handle, FixtureEvent::Started,)?,
            PersonalAssistantV0Snapshot::Streaming(_)
        ));
        before_connect.cancel(&before_connect_handle)?;

        let (mut connect, connect_clock) = native_host();
        let connect_handle = start_handle(&mut connect)?;
        connect_clock.advance_to(GATEWAY_CONNECT_TIMEOUT)?;
        let failed = connect.accept_fixture_event(&connect_handle, FixtureEvent::Started)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );
        assert_eq!(failed.sequence(), 1);
        assert!(connect
            .current_record()
            .is_some_and(|record| { record.last_expired_deadline == Some(DeadlineKind::Connect) }));

        let (mut idle, idle_clock) = native_host();
        let idle_handle = start_handle(&mut idle)?;
        idle.accept_fixture_event(&idle_handle, FixtureEvent::Started)?;
        idle_clock.advance_to(GATEWAY_STREAM_IDLE_TIMEOUT)?;
        let failed =
            idle.accept_fixture_event(&idle_handle, FixtureEvent::TextDelta(FixtureDelta::Canary))?;
        assert_eq!(failed.sequence(), 2);
        assert!(failed.accepted_text().is_empty());
        assert!(idle
            .current_record()
            .is_some_and(|record| record.last_expired_deadline == Some(DeadlineKind::Idle)));

        let (mut provider, provider_clock) = native_host();
        let provider_handle = start_handle(&mut provider)?;
        provider.accept_fixture_event(&provider_handle, FixtureEvent::Started)?;
        for seconds in [19, 38, 57] {
            provider_clock.advance_to(Duration::from_secs(seconds))?;
            provider.accept_fixture_event(
                &provider_handle,
                FixtureEvent::TextDelta(FixtureDelta::Unit),
            )?;
        }
        provider_clock.advance_to(PROVIDER_TURN_DEADLINE)?;
        let failed = provider.accept_fixture_event(&provider_handle, FixtureEvent::Completed)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );
        assert_eq!(failed.sequence(), 5);
        assert_eq!(failed.accepted_text(), "xxx");
        assert!(provider.current_record().is_some_and(|record| {
            record.last_expired_deadline == Some(DeadlineKind::Provider)
        }));

        let (mut total, total_clock) = native_host();
        let total_handle = start_handle(&mut total)?;
        total_clock.advance_to(AGENT_RUN_DEADLINE)?;
        let failed = total.accept_fixture_event(
            &total_handle,
            FixtureEvent::Failed(FixtureFailure::ProviderUnavailable),
        )?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );
        assert!(total
            .current_record()
            .is_some_and(|record| record.last_expired_deadline == Some(DeadlineKind::Total)));

        let (mut cancellation, cancellation_clock) = native_host();
        let cancellation_handle = start_handle(&mut cancellation)?;
        cancellation_clock.advance_to(GATEWAY_CONNECT_TIMEOUT)?;
        let failed = cancellation.cancel(&cancellation_handle)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );
        assert_eq!(cancellation.cancel(&cancellation_handle)?, failed);

        let (mut before_provider, before_provider_clock) = native_host();
        let before_provider_handle = start_handle(&mut before_provider)?;
        before_provider.accept_fixture_event(&before_provider_handle, FixtureEvent::Started)?;
        for seconds in [19, 38, 57] {
            before_provider_clock.advance_to(Duration::from_secs(seconds))?;
            before_provider.accept_fixture_event(
                &before_provider_handle,
                FixtureEvent::TextDelta(FixtureDelta::Unit),
            )?;
        }
        before_provider_clock.advance_to(Duration::from_millis(59_999))?;
        assert!(matches!(
            before_provider
                .accept_fixture_event(&before_provider_handle, FixtureEvent::Completed)?,
            PersonalAssistantV0Snapshot::Completed(_)
        ));
        Ok(())
    }

    #[derive(Clone)]
    struct MockRuntime {
        control: Arc<Mutex<MockControl>>,
    }

    struct MockControl {
        returned_identity: Option<RuntimeRunIdentity>,
        returned_status: RuntimeRunStatus,
        cancellation_results: Vec<Result<RuntimeCancellationOutcome, RuntimeError>>,
        cancellation_status_overrides: Vec<RuntimeRunStatus>,
        cancellation_identity_overrides: Vec<RuntimeRunIdentity>,
        cancellation_calls: usize,
    }

    struct MockRun {
        identity: RuntimeRunIdentity,
        status: RuntimeRunStatus,
        control: Arc<Mutex<MockControl>>,
    }

    impl AgentRuntime for MockRuntime {
        type Run = MockRun;

        fn describe(&self) -> crate::agent::runtime::RuntimeDescriptor {
            NativeAgentRuntime.describe()
        }

        fn start(&self, request: RuntimeTurnRequest) -> Result<Self::Run, RuntimeError> {
            let requested_identity = request.identity();
            let control = self.control.lock().map_err(|_| RuntimeError::Unavailable)?;
            let identity = match &control.returned_identity {
                Some(identity) => identity.clone(),
                None => requested_identity,
            };
            let status = control.returned_status;
            drop(control);
            Ok(MockRun {
                identity,
                status,
                control: Arc::clone(&self.control),
            })
        }
    }

    impl RuntimeRun for MockRun {
        fn run_id(&self) -> &crate::agent::runtime::RuntimeRunId {
            self.identity.run_id()
        }

        fn identity(&self) -> &RuntimeRunIdentity {
            &self.identity
        }

        fn status(&self) -> RuntimeRunStatus {
            self.status
        }

        fn accept_event(
            &mut self,
            event: RuntimeEventEnvelope,
        ) -> Result<RuntimeEventAcceptance, RuntimeError> {
            let (_, _, _, event) = event.into_parts();
            match event {
                UntrustedRuntimeEvent::ResponseStarted { response_id } => {
                    self.status = RuntimeRunStatus::Streaming;
                    Ok(RuntimeEventAcceptance::ResponseStarted { response_id })
                }
                UntrustedRuntimeEvent::OutputTextDelta { delta } => {
                    self.status = RuntimeRunStatus::Streaming;
                    Ok(RuntimeEventAcceptance::OutputTextDelta { delta })
                }
                UntrustedRuntimeEvent::ResponseCompleted => {
                    self.status = RuntimeRunStatus::Completed;
                    Ok(RuntimeEventAcceptance::ResponseCompleted)
                }
                UntrustedRuntimeEvent::ResponseFailed { failure } => {
                    self.status = RuntimeRunStatus::Failed;
                    Ok(RuntimeEventAcceptance::ResponseFailed { failure })
                }
                UntrustedRuntimeEvent::ToolProposal { .. } => Err(RuntimeError::Unavailable),
            }
        }

        fn cancel(&mut self) -> Result<RuntimeCancellationOutcome, RuntimeError> {
            let mut control = self.control.lock().map_err(|_| RuntimeError::Unavailable)?;
            control.cancellation_calls += 1;
            let result = if control.cancellation_results.is_empty() {
                Ok(RuntimeCancellationOutcome::Cancelled)
            } else {
                control.cancellation_results.remove(0)
            };
            match result {
                Ok(RuntimeCancellationOutcome::Cancelled) => {
                    self.status = RuntimeRunStatus::Cancelled;
                }
                Ok(RuntimeCancellationOutcome::AlreadyTerminal(status)) => {
                    self.status = status;
                }
                Err(_) => {}
            }
            if !control.cancellation_status_overrides.is_empty() {
                self.status = control.cancellation_status_overrides.remove(0);
            }
            if !control.cancellation_identity_overrides.is_empty() {
                self.identity = control.cancellation_identity_overrides.remove(0);
            }
            result
        }
    }

    fn mock_runtime(
        returned_identity: Option<RuntimeRunIdentity>,
        returned_status: RuntimeRunStatus,
        cancellation_results: Vec<Result<RuntimeCancellationOutcome, RuntimeError>>,
    ) -> (MockRuntime, Arc<Mutex<MockControl>>) {
        let control = Arc::new(Mutex::new(MockControl {
            returned_identity,
            returned_status,
            cancellation_results,
            cancellation_status_overrides: Vec::new(),
            cancellation_identity_overrides: Vec::new(),
            cancellation_calls: 0,
        }));
        (
            MockRuntime {
                control: Arc::clone(&control),
            },
            control,
        )
    }

    #[test]
    fn cleanup_ambiguity_retains_owner_and_retry_publishes_once() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let (runtime, control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![
                Err(RuntimeError::Unavailable),
                Ok(RuntimeCancellationOutcome::Cancelled),
            ],
        );
        let clock = ManualClock::new();
        let mut host = HostCore::new(runtime, MonotonicClock::Manual(clock));
        let handle = host.start_synthetic()?.presentation_handle().clone();

        assert_eq!(
            host.cancel(&handle),
            Err(PersonalAssistantV0Error::Internal)
        );
        assert!(matches!(
            host.current_record().map(SessionRecord::snapshot),
            Some(PersonalAssistantV0Snapshot::Cancelling(_))
        ));
        assert_eq!(host.start_synthetic(), Err(PersonalAssistantV0Error::Busy));
        let cancelled = host.cancel(&handle)?;
        assert!(matches!(
            cancelled,
            PersonalAssistantV0Snapshot::Cancelled(_)
        ));
        assert_eq!(cancelled.sequence(), 1);
        assert_eq!(host.cancel(&handle)?, cancelled);
        assert_eq!(
            control
                .lock()
                .map_err(|_| "mock lock poisoned")?
                .cancellation_calls,
            2
        );

        let (runtime, _control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![Ok(RuntimeCancellationOutcome::AlreadyTerminal(
                RuntimeRunStatus::Completed,
            ))],
        );
        let mut mismatch = HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        let mismatch_handle = mismatch.start_synthetic()?.presentation_handle().clone();
        let failed = mismatch.cancel(&mismatch_handle)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::Internal
        );
        assert!(terminal_failure(&failed)?.support_correlation().is_some());

        let (runtime, control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![
                Ok(RuntimeCancellationOutcome::Cancelled),
                Ok(RuntimeCancellationOutcome::Cancelled),
            ],
        );
        control
            .lock()
            .map_err(|_| "mock lock poisoned")?
            .cancellation_status_overrides
            .push(RuntimeRunStatus::Streaming);
        let mut contradictory_status =
            HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        let status_handle = contradictory_status
            .start_synthetic()?
            .presentation_handle()
            .clone();
        assert_eq!(
            contradictory_status.cancel(&status_handle),
            Err(PersonalAssistantV0Error::Internal)
        );
        assert!(matches!(
            contradictory_status
                .current_record()
                .map(SessionRecord::snapshot),
            Some(PersonalAssistantV0Snapshot::Cancelling(_))
        ));
        assert!(matches!(
            contradictory_status.cancel(&status_handle)?,
            PersonalAssistantV0Snapshot::Cancelled(_)
        ));

        let foreign_identity =
            RuntimeTurnRequest::new("pa-v0-mutated-run", "pa-v0-mutated-request", "synthetic")?
                .identity();
        let (runtime, control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![
                Ok(RuntimeCancellationOutcome::Cancelled),
                Ok(RuntimeCancellationOutcome::Cancelled),
            ],
        );
        let mut contradictory_identity =
            HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        let identity_handle = contradictory_identity
            .start_synthetic()?
            .presentation_handle()
            .clone();
        let expected_identity = match &contradictory_identity.state {
            HostState::Active(active) => active.owner.expected_identity.clone(),
            _ => return Err("expected active identity fixture".into()),
        };
        {
            let mut control = control.lock().map_err(|_| "mock lock poisoned")?;
            control.cancellation_identity_overrides = vec![foreign_identity, expected_identity];
        }
        assert_eq!(
            contradictory_identity.cancel(&identity_handle),
            Err(PersonalAssistantV0Error::Internal)
        );
        assert!(matches!(
            contradictory_identity
                .current_record()
                .map(SessionRecord::snapshot),
            Some(PersonalAssistantV0Snapshot::Cancelling(_))
        ));
        let failed = contradictory_identity.cancel(&identity_handle)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::Internal
        );
        Ok(())
    }

    #[test]
    fn deadline_and_terminal_cleanup_retries_preserve_original_intent() -> Result<(), Box<dyn Error>>
    {
        let _serial = serial_guard()?;

        let (runtime, _control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![
                Err(RuntimeError::Unavailable),
                Ok(RuntimeCancellationOutcome::Cancelled),
            ],
        );
        let clock = ManualClock::new();
        let mut deadline = HostCore::new(runtime, MonotonicClock::Manual(clock.clone()));
        let deadline_handle = deadline.start_synthetic()?.presentation_handle().clone();
        clock.advance_to(GATEWAY_CONNECT_TIMEOUT)?;
        assert_eq!(
            deadline.snapshot(&deadline_handle),
            Err(PersonalAssistantV0Error::DeadlineExceeded)
        );
        assert!(matches!(
            deadline.current_record().map(SessionRecord::snapshot),
            Some(PersonalAssistantV0Snapshot::Cancelling(_))
        ));
        let failed = deadline.cancel(&deadline_handle)?;
        assert_eq!(
            terminal_failure(&failed)?.code(),
            PersonalAssistantV0FailureCode::DeadlineExceeded
        );

        let (runtime, _control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![
                Err(RuntimeError::Unavailable),
                Ok(RuntimeCancellationOutcome::AlreadyTerminal(
                    RuntimeRunStatus::Completed,
                )),
            ],
        );
        let mut completed = HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        let completed_handle = completed.start_synthetic()?.presentation_handle().clone();
        completed.accept_fixture_event(&completed_handle, FixtureEvent::Started)?;
        completed.accept_fixture_event(
            &completed_handle,
            FixtureEvent::TextDelta(FixtureDelta::BoardPrefix),
        )?;
        assert_eq!(
            completed.accept_fixture_event(&completed_handle, FixtureEvent::Completed),
            Err(PersonalAssistantV0Error::Internal)
        );
        let completed_snapshot = completed.cancel(&completed_handle)?;
        let PersonalAssistantV0Snapshot::Completed(completed_detail) = completed_snapshot else {
            return Err("expected preserved completion intent".into());
        };
        assert_eq!(completed_detail.final_answer(), "Board ");

        let (runtime, _control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![
                Err(RuntimeError::Unavailable),
                Ok(RuntimeCancellationOutcome::AlreadyTerminal(
                    RuntimeRunStatus::Failed,
                )),
            ],
        );
        let mut failed_host = HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        let failed_handle = failed_host.start_synthetic()?.presentation_handle().clone();
        failed_host.accept_fixture_event(&failed_handle, FixtureEvent::Started)?;
        assert_eq!(
            failed_host.accept_fixture_event(
                &failed_handle,
                FixtureEvent::Failed(FixtureFailure::ProviderTimeout),
            ),
            Err(PersonalAssistantV0Error::Internal)
        );
        let failed_snapshot = failed_host.cancel(&failed_handle)?;
        assert_eq!(
            terminal_failure(&failed_snapshot)?.code(),
            PersonalAssistantV0FailureCode::ProviderTimeout
        );
        Ok(())
    }

    #[test]
    fn rejected_start_cleanup_is_resumable_and_drop_retains_full_session(
    ) -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let foreign_identity =
            RuntimeTurnRequest::new("pa-v0-foreign-run", "pa-v0-foreign-request", "synthetic")?
                .identity();
        let (runtime, control) = mock_runtime(
            Some(foreign_identity),
            RuntimeRunStatus::AwaitingStart,
            vec![
                Err(RuntimeError::Unavailable),
                Ok(RuntimeCancellationOutcome::Cancelled),
            ],
        );
        let mut host = HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        assert_eq!(
            host.start_synthetic(),
            Err(PersonalAssistantV0Error::Internal)
        );
        {
            let mut control = control.lock().map_err(|_| "mock lock poisoned")?;
            control.returned_identity = None;
        }
        let restarted = host.start_synthetic()?;
        let restarted_handle = restarted.presentation_handle().clone();
        host.cancel(&restarted_handle)?;
        assert_eq!(
            control
                .lock()
                .map_err(|_| "mock lock poisoned")?
                .cancellation_calls,
            3
        );

        let (runtime, _control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![Err(RuntimeError::Unavailable)],
        );
        let mut dropped = HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        dropped.start_synthetic()?;
        if let HostState::Active(active) = &mut dropped.state {
            active.record.accepted_text = "DROP-CONTENT-CANARY".to_owned();
        } else {
            return Err("expected active mock session".into());
        }
        let state = mem::replace(&mut dropped.state, HostState::Idle);
        let payload = cleanup_state_for_drop(state).ok_or("expected ambiguous drop payload")?;
        let QuarantinePayload::Session { record, .. } = &payload else {
            return Err("expected full session quarantine".into());
        };
        assert_eq!(record.accepted_text, "DROP-CONTENT-CANARY");
        assert!(matches!(
            ProcessLease::acquire(),
            Err(PersonalAssistantV0Error::Busy)
        ));
        drop(payload);
        let released = ProcessLease::acquire()?;
        drop(released);
        Ok(())
    }

    struct DropProbe(Arc<AtomicUsize>);

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::AcqRel);
        }
    }

    #[test]
    fn proved_drop_cleanup_releases_owner_and_fallback_quarantine_never_drops_replacement(
    ) -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        let (runtime, _control) = mock_runtime(
            None,
            RuntimeRunStatus::AwaitingStart,
            vec![Ok(RuntimeCancellationOutcome::Cancelled)],
        );
        let mut clean = HostCore::new(runtime, MonotonicClock::Manual(ManualClock::new()));
        clean.start_synthetic()?;
        if let HostState::Active(active) = &mut clean.state {
            active.record.accepted_text = "CLEAN-DROP-CONTENT-CANARY".to_owned();
        } else {
            return Err("expected active clean-drop session".into());
        }
        let state = mem::replace(&mut clean.state, HostState::Idle);
        assert!(cleanup_state_for_drop(state).is_none());
        let released = ProcessLease::acquire()?;
        drop(released);

        let occupied_drops = Arc::new(AtomicUsize::new(0));
        let occupied = Mutex::new(Some(DropProbe(Arc::clone(&occupied_drops))));
        quarantine_or_leak(DropProbe(Arc::clone(&occupied_drops)), &occupied);
        assert_eq!(occupied_drops.load(Ordering::Acquire), 0);
        drop(occupied);
        assert_eq!(occupied_drops.load(Ordering::Acquire), 1);

        let poisoned_drops = Arc::new(AtomicUsize::new(0));
        let poisoned = Mutex::new(None);
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let _guard = match poisoned.lock() {
                Ok(guard) => guard,
                Err(error) => error.into_inner(),
            };
            std::panic::resume_unwind(Box::new(()));
        }));
        assert!(poisoned.is_poisoned());
        quarantine_or_leak(DropProbe(Arc::clone(&poisoned_drops)), &poisoned);
        assert_eq!(poisoned_drops.load(Ordering::Acquire), 0);
        Ok(())
    }

    #[test]
    fn correlations_generation_and_debug_paths_fail_closed() -> Result<(), Box<dyn Error>> {
        let _serial = serial_guard()?;
        assert!(is_valid_correlation("x"));
        assert!(is_valid_correlation(&"x".repeat(MAX_CORRELATION_BYTES)));
        assert!(!is_valid_correlation(""));
        assert!(!is_valid_correlation(
            &"x".repeat(MAX_CORRELATION_BYTES + 1)
        ));
        assert!(!is_valid_correlation("contains space"));
        assert!(!is_valid_correlation("é"));
        let counter = AtomicU64::new(1);
        assert_eq!(reserve_generation(&counter)?, 1);
        assert_eq!(reserve_generation(&counter)?, 2);
        assert_eq!(
            reserve_generation(&AtomicU64::new(0)),
            Err(PersonalAssistantV0Error::Internal)
        );
        assert_eq!(
            reserve_generation(&AtomicU64::new(u64::MAX)),
            Err(PersonalAssistantV0Error::Internal)
        );

        let (mut host, _clock) = native_host();
        let start = host.start_synthetic()?;
        let handle = start.presentation_handle().clone();
        let handle_value = handle.as_str().to_owned();
        host.accept_fixture_event(&handle, FixtureEvent::Started)?;
        host.accept_fixture_event(&handle, FixtureEvent::TextDelta(FixtureDelta::Canary))?;
        let snapshot = host.snapshot(&handle)?;
        let batch = host.updates(&handle, Some(0))?;
        let rendered = format!("{start:?} {handle:?} {snapshot:?} {batch:?}");
        for forbidden in [
            "V0-2-SYNTHETIC-OUTPUT-CANARY",
            "pa-v0-run-",
            "pa-v0-request-",
            "pa-v0-fixture-response-",
            handle_value.as_str(),
        ] {
            assert!(!rendered.contains(forbidden));
        }
        assert!(rendered.contains("[REDACTED]"));
        host.cancel(&handle)?;
        Ok(())
    }
}
