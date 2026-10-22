//! Content-bearing voice data is never Debug/logged or persisted here.
use serde::{Deserialize, Serialize};

pub(crate) const MAX_FRAME: usize = 32_768;
pub(crate) const MAX_CAPTURE: usize = 262_144;

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum VoiceError {
    Unsupported,
    Unavailable,
    PermissionDenied,
    Busy,
    StaleSession,
    Protocol,
    Limit,
    Timeout,
    CaptureFailed,
    RecognitionFailed,
    RecognitionNoSpeech,
    RecognitionAssetsMissing,
    RecognitionDisabled,
    RecognitionService,
    RecognitionBusy,
    HelperDiagnostics,
    CleanupFailed,
}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Operation {
    Authorize,
    Record,
    Speak,
}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
    Idle,
    Starting,
    CheckingMicrophone,
    CheckingSpeech,
    Authorized,
    Recording,
    Finishing,
    Speaking,
    Review,
    Cancelled,
    Completed,
    Error,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Snapshot {
    pub(crate) version: u8,
    pub(crate) conversation_id: String,
    pub(crate) session_id: String,
    pub(crate) sequence: u64,
    pub(crate) status: Status,
    pub(crate) busy: bool,
    pub(crate) text: Option<String>,
    pub(crate) error: Option<VoiceError>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Frame {
    version: u8,
    session: String,
    sequence: u64,
    event: Event,
    text: Option<String>,
    error: Option<VoiceError>,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Event {
    CheckingMicrophone,
    CheckingSpeech,
    Authorized,
    Recording,
    Transcript,
    Speaking,
    Spoken,
    Cancelled,
    Error,
}

pub(crate) struct Protocol {
    pub(crate) snapshot: Snapshot,
    operation: Operation,
    terminal: bool,
    discard: bool,
}
impl Protocol {
    pub(crate) fn new(conversation: String, session: String, operation: Operation) -> Self {
        Self {
            snapshot: Snapshot {
                version: 1,
                conversation_id: conversation,
                session_id: session,
                sequence: 0,
                status: Status::Starting,
                busy: true,
                text: None,
                error: None,
            },
            operation,
            terminal: false,
            discard: false,
        }
    }
    pub(crate) fn accept(&mut self, data: &[u8]) -> Result<(), VoiceError> {
        if data.len() > MAX_FRAME || self.terminal {
            return Err(VoiceError::Protocol);
        }
        let frame: Frame = serde_json::from_slice(data).map_err(|_| VoiceError::Protocol)?;
        if frame.version != 1
            || frame.session != self.snapshot.session_id
            || frame.sequence != self.snapshot.sequence + 1
        {
            return Err(VoiceError::Protocol);
        }
        if matches!(frame.event, Event::Error) {
            if frame.text.is_some() || frame.error.is_none() {
                return Err(VoiceError::Protocol);
            }
            self.snapshot.error = frame.error;
            self.snapshot.status = Status::Error;
            self.terminal = true;
        } else {
            if frame.error.is_some() {
                return Err(VoiceError::Protocol);
            }
            if !matches!(frame.event, Event::Transcript) && frame.text.is_some() {
                return Err(VoiceError::Protocol);
            }
            match frame.event {
                Event::CheckingMicrophone
                    if self.operation == Operation::Authorize
                        && self.snapshot.status == Status::Starting =>
                {
                    self.snapshot.status = Status::CheckingMicrophone;
                }
                Event::CheckingSpeech
                    if self.operation == Operation::Authorize
                        && self.snapshot.status == Status::CheckingMicrophone =>
                {
                    self.snapshot.status = Status::CheckingSpeech;
                }
                Event::Authorized
                    if self.operation == Operation::Authorize
                        && self.snapshot.status == Status::CheckingSpeech =>
                {
                    self.snapshot.status = Status::Authorized;
                    self.terminal = true;
                }
                Event::Recording
                    if self.operation == Operation::Record
                        && self.snapshot.status == Status::Starting =>
                {
                    self.snapshot.status = Status::Recording
                }
                Event::Speaking
                    if self.operation == Operation::Speak
                        && self.snapshot.status == Status::Starting =>
                {
                    self.snapshot.status = Status::Speaking
                }
                Event::Transcript
                    if self.operation == Operation::Record
                        && matches!(
                            self.snapshot.status,
                            Status::Recording | Status::Finishing
                        ) =>
                {
                    let text = frame.text.ok_or(VoiceError::Protocol)?;
                    if text.trim().is_empty()
                        || text.chars().count() > 4096
                        || text
                            .chars()
                            .any(|c| c.is_control() && c != '\n' && c != '\t')
                    {
                        return Err(VoiceError::Limit);
                    }
                    self.snapshot.text = if self.discard { None } else { Some(text) };
                    self.snapshot.status = if self.discard {
                        Status::Cancelled
                    } else {
                        Status::Review
                    };
                    self.terminal = true;
                }
                Event::Spoken
                    if self.operation == Operation::Speak
                        && self.snapshot.status == Status::Speaking =>
                {
                    self.snapshot.status = if self.discard {
                        Status::Cancelled
                    } else {
                        Status::Completed
                    };
                    self.terminal = true;
                }
                Event::Cancelled if self.discard => {
                    self.snapshot.status = Status::Cancelled;
                    self.terminal = true;
                }
                _ => return Err(VoiceError::Protocol),
            }
        }
        self.snapshot.sequence = frame.sequence;
        Ok(())
    }
    pub(crate) fn operation(&self) -> Operation {
        self.operation
    }
    pub(crate) fn finish(&mut self) -> Result<(), VoiceError> {
        if self.operation != Operation::Record
            || self.snapshot.status != Status::Recording
            || self.terminal
        {
            return Err(VoiceError::StaleSession);
        }
        self.snapshot.status = Status::Finishing;
        Ok(())
    }
    pub(crate) fn discard(&mut self) {
        self.discard = true;
        self.snapshot.text = None;
    }
    pub(crate) fn failed(&mut self, error: VoiceError) {
        self.snapshot.status = Status::Error;
        self.snapshot.text = None;
        self.snapshot.error = Some(error);
        self.terminal = true;
    }
    pub(crate) fn settle_captured(
        &mut self,
        exit_ok: bool,
        clean_capture: bool,
        helper_diagnostics: bool,
    ) {
        self.settle(exit_ok, clean_capture);
        // This is a negative capture witness, never a benign-warning exception.
        if helper_diagnostics {
            self.failed(VoiceError::HelperDiagnostics);
        }
    }
    pub(crate) fn settle(&mut self, exit_ok: bool, clean_capture: bool) {
        if !clean_capture || !self.terminal || (self.snapshot.error.is_none() && !exit_ok) {
            self.failed(VoiceError::Protocol);
        }
        if self.discard && self.snapshot.error.is_none() {
            self.snapshot.text = None;
            self.snapshot.status = Status::Cancelled;
        }
        self.snapshot.busy = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn protocol(operation: Operation) -> Protocol {
        Protocol::new("conversation".into(), "abc-123".into(), operation)
    }
    fn frame(seq: u64, event: &str, text: Option<&str>, error: Option<&str>) -> Vec<u8> {
        (serde_json::json!({"version":1,"session":"abc-123","sequence":seq,"event":event,"text":text,"error":error})).to_string().into_bytes()
    }
    #[test]
    fn diagnostic_bytes_reject_even_valid_terminal_and_discard_transcript() -> Result<(), String> {
        let mut authorized = protocol(Operation::Authorize);
        for (sequence, event) in [
            (1, "checking_microphone"),
            (2, "checking_speech"),
            (3, "authorized"),
        ] {
            authorized
                .accept(&frame(sequence, event, None, None))
                .map_err(|e| format!("{e:?}"))?;
        }
        authorized.settle_captured(true, false, true);
        assert!(authorized.snapshot.status == Status::Error);
        assert!(authorized.snapshot.error == Some(VoiceError::HelperDiagnostics));
        assert!(!authorized.snapshot.busy);
        let mut captured = protocol(Operation::Record);
        captured
            .accept(&frame(1, "recording", None, None))
            .map_err(|e| format!("{e:?}"))?;
        captured
            .accept(&frame(2, "transcript", Some("synthetic"), None))
            .map_err(|e| format!("{e:?}"))?;
        captured.settle_captured(true, false, true);
        assert!(captured.snapshot.text.is_none());
        assert!(captured.snapshot.error == Some(VoiceError::HelperDiagnostics));
        assert!(captured
            .accept(&frame(3, "transcript", Some("late"), None))
            .is_err());
        Ok(())
    }

    #[test]
    fn transcript_is_not_released_until_complete_capture_and_exit() -> Result<(), String> {
        let mut p = protocol(Operation::Record);
        p.accept(&frame(1, "recording", None, None))
            .map_err(|e| format!("{e:?}"))?;
        p.finish().map_err(|e| format!("{e:?}"))?;
        p.accept(&frame(2, "transcript", Some("Synthetic draft."), None))
            .map_err(|e| format!("{e:?}"))?;
        assert!(p.snapshot.busy);
        p.settle(true, true);
        assert_eq!(p.snapshot.status, Status::Review);
        assert!(!p.snapshot.busy);
        Ok(())
    }
    #[test]
    fn cancellation_does_not_hide_errors_or_failed_capture() -> Result<(), String> {
        for error in [
            "protocol",
            "capture_failed",
            "recognition_failed",
            "recognition_no_speech",
            "recognition_assets_missing",
            "recognition_disabled",
            "recognition_service",
            "recognition_busy",
            "timeout",
        ] {
            let mut p = protocol(Operation::Record);
            p.discard();
            p.accept(&frame(1, "error", None, Some(error)))
                .map_err(|e| format!("{e:?}"))?;
            p.settle(false, true);
            assert_eq!(p.snapshot.status, Status::Error);
            assert!(p.snapshot.text.is_none());
        }
        let mut p = protocol(Operation::Record);
        p.discard();
        p.accept(&frame(1, "cancelled", None, None))
            .map_err(|e| format!("{e:?}"))?;
        p.settle(true, false);
        assert_eq!(p.snapshot.error, Some(VoiceError::Protocol));
        Ok(())
    }
    #[test]
    fn cancel_discards_a_valid_queued_final_result_without_inventing_abort_proof(
    ) -> Result<(), String> {
        let mut p = protocol(Operation::Record);
        p.accept(&frame(1, "recording", None, None))
            .map_err(|e| format!("{e:?}"))?;
        p.discard();
        p.accept(&frame(2, "transcript", Some("Late valid result"), None))
            .map_err(|e| format!("{e:?}"))?;
        p.settle(true, true);
        assert_eq!(p.snapshot.status, Status::Cancelled);
        assert!(p.snapshot.text.is_none());
        Ok(())
    }
    #[test]
    fn identities_unknown_fields_duplicate_fields_replay_and_limits_reject() -> Result<(), String> {
        let good =
            String::from_utf8(frame(1, "recording", None, None)).map_err(|e| format!("{e:?}"))?;
        for bad in [
            good.replace("abc-123", "stale"),
            good.replace("\"version\":1", "\"version\":2"),
            good.replace("\"sequence\":1", "\"sequence\":2"),
            good.replace("recording", "unknown"),
            good.replacen('{', "{\"extra\":true,", 1),
            good.replacen('{', "{\"version\":1,", 1),
            "{}".into(),
        ] {
            assert!(protocol(Operation::Record).accept(bad.as_bytes()).is_err());
        }
        let mut p = protocol(Operation::Record);
        p.accept(good.as_bytes()).map_err(|e| format!("{e:?}"))?;
        assert!(p.accept(good.as_bytes()).is_err());
        assert!(protocol(Operation::Record)
            .accept(&vec![b' '; MAX_FRAME + 1])
            .is_err());
        Ok(())
    }
    #[test]
    fn operation_order_text_and_terminal_states_are_strict() -> Result<(), String> {
        for op in [Operation::Record, Operation::Speak] {
            assert!(protocol(op)
                .accept(&frame(1, "authorized", None, None))
                .is_err());
        }
        for text in ["", "\0", &"x".repeat(4097)] {
            let mut p = protocol(Operation::Record);
            p.accept(&frame(1, "recording", None, None))
                .map_err(|e| format!("{e:?}"))?;
            assert!(p.accept(&frame(2, "transcript", Some(text), None)).is_err());
        }
        let mut p = protocol(Operation::Authorize);
        p.accept(&frame(1, "checking_microphone", None, None))
            .map_err(|e| format!("{e:?}"))?;
        p.accept(&frame(2, "checking_speech", None, None))
            .map_err(|e| format!("{e:?}"))?;
        p.accept(&frame(3, "authorized", None, None))
            .map_err(|e| format!("{e:?}"))?;
        assert!(p.accept(&frame(4, "authorized", None, None)).is_err());
        assert!(protocol(Operation::Record)
            .accept(&frame(1, "cancelled", None, None))
            .is_err());
        Ok(())
    }
    #[test]
    fn permission_phases_reject_skip_replay_wrong_operation_and_payload() -> Result<(), String> {
        for event in ["checking_speech", "authorized"] {
            assert!(protocol(Operation::Authorize)
                .accept(&frame(1, event, None, None))
                .is_err());
        }
        for op in [Operation::Record, Operation::Speak] {
            assert!(protocol(op)
                .accept(&frame(1, "checking_microphone", None, None))
                .is_err());
        }
        assert!(protocol(Operation::Authorize)
            .accept(&frame(1, "checking_microphone", Some("private"), None))
            .is_err());
        let mut p = protocol(Operation::Authorize);
        p.accept(&frame(1, "checking_microphone", None, None))
            .map_err(|e| format!("{e:?}"))?;
        assert!(p
            .accept(&frame(2, "checking_microphone", None, None))
            .is_err());
        p.accept(&frame(2, "error", None, Some("permission_denied")))
            .map_err(|e| format!("{e:?}"))?;
        p.settle(false, true);
        assert_eq!(p.snapshot.error, Some(VoiceError::PermissionDenied));
        assert!(!p.snapshot.busy);
        Ok(())
    }
    #[test]
    fn successful_playback_and_abnormal_exit_remain_separate() -> Result<(), String> {
        for exit in [true, false] {
            let mut p = protocol(Operation::Speak);
            p.accept(&frame(1, "speaking", None, None))
                .map_err(|e| format!("{e:?}"))?;
            p.accept(&frame(2, "spoken", None, None))
                .map_err(|e| format!("{e:?}"))?;
            p.settle(exit, true);
            assert_eq!(
                p.snapshot.status,
                if exit {
                    Status::Completed
                } else {
                    Status::Error
                }
            );
        }
        Ok(())
    }
}
