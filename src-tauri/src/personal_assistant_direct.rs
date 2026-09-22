//! Fixed, opt-in development transport. No endpoint or credential is supplied by IPC.
use std::{fmt, time::Duration};

use reqwest::{header, Client, Request, StatusCode};
use serde::Serialize;
use serde_json::{json, Value};

pub(crate) const ENDPOINT: &str = "https://api.openai.com/v1/responses";
pub(crate) const MODEL: &str = "gpt-5.6-luna";
pub(crate) const SAMPLE: &str = crate::agent::runtime::PERSONAL_ASSISTANT_V0_SYNTHETIC_FIXTURE;
const MAX_FRAME: usize = 65_536;
const MAX_WIRE: usize = 1_048_576;
const MAX_EVENTS: u64 = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DirectError {
    #[error("Native live mode is disabled.")]
    Disabled,
    #[error("Set the native session API key before starting.")]
    MissingKey,
    #[error("The provider rejected authentication.")]
    Authentication,
    #[error("The fixed model is unavailable or the request was rejected.")]
    ModelUnavailable,
    #[error("The provider rate or spending limit was reached.")]
    RateLimited,
    #[error("The request timed out.")]
    Timeout,
    #[error("The provider connection failed while sending or receiving data.")]
    Network,
    #[error("The provider returned an unsuccessful HTTP response.")]
    HttpStatus,
    #[error("The provider emitted a top-level error event in the response stream. No automatic retry was made.")]
    ProviderStreamErrorEvent,
    #[error("The provider reported response.failed with an unrecognized error code. No automatic retry was made.")]
    ProviderStreamFailedUnknownCode,
    #[error("The provider reported response.failed without a usable error code. No automatic retry was made.")]
    ProviderStreamFailedInvalidCode,
    #[error(
        "The provider reported a server error in the response stream. No automatic retry was made."
    )]
    ProviderStreamServerError,
    #[error(
        "The provider reported a rate limit in the response stream. No automatic retry was made."
    )]
    ProviderStreamRateLimit,
    #[error("The provider reported an invalid prompt in the response stream. No automatic retry was made.")]
    ProviderStreamInvalidPrompt,
    #[error("The provider refused this request.")]
    Refused,
    #[error("The response ended without a complete answer.")]
    Incomplete,
    #[error("The response exceeded the demo limit.")]
    Limit,
    #[error("The provider response was invalid.")]
    Protocol,
    #[error("Another request is still active or stopping.")]
    Busy,
    #[error("The native request or acknowledgment is invalid.")]
    InvalidRequest,
    #[error("The native session is unavailable.")]
    Internal,
}

pub(crate) struct ApiKey(header::HeaderValue);
impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ApiKey([REDACTED])")
    }
}
impl ApiKey {
    pub(crate) fn from_values(
        development: bool,
        enabled: bool,
        value: Option<String>,
    ) -> Result<Self, DirectError> {
        if !development || !enabled {
            return Err(DirectError::Disabled);
        }
        let value = value
            .filter(|value| !value.is_empty())
            .ok_or(DirectError::MissingKey)?;
        if value.len() > 512 || !value.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(DirectError::MissingKey);
        }
        let mut header = header::HeaderValue::from_str(&format!("Bearer {value}"))
            .map_err(|_| DirectError::MissingKey)?;
        header.set_sensitive(true);
        Ok(Self(header))
    }
    /// Called only by an explicitly acknowledged Start, never during startup/poll.
    pub(crate) fn from_environment() -> Result<Self, DirectError> {
        if !cfg!(debug_assertions) {
            return Err(DirectError::Disabled);
        }
        if std::env::var("CORTEXA_OPENAI_DEMO").as_deref() != Ok("1") {
            return Err(DirectError::Disabled);
        }
        Self::from_values(true, true, std::env::var("OPENAI_API_KEY").ok())
    }
}

// Deliberately no Debug: provider text and identifiers must not enter logs.
pub(crate) enum ProviderEvent {
    Started(String),
    Delta(String),
    Completed,
}

pub(crate) fn request_body() -> Result<Vec<u8>, DirectError> {
    serde_json::to_vec(&json!({
        "model": MODEL,
        "instructions": "Act as Cortexa's Personal Assistant. Answer only from the supplied synthetic status in three concise plain-text bullets. Do not use tools or claim external actions.",
        "input": SAMPLE, "stream": true, "store": false, "background": false,
        "tools": [], "tool_choice": "none", "max_output_tokens": 512,
        "reasoning": {"effort": "none"}, "text": {"format": {"type": "text"}},
        "truncation": "disabled"
    })).map_err(|_| DirectError::Internal)
}

fn client() -> Result<Client, DirectError> {
    Client::builder()
        .https_only(true)
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .connect_timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(60))
        .pool_max_idle_per_host(0)
        .build()
        .map_err(|_| DirectError::Internal)
}
fn request(client: &Client, key: ApiKey) -> Result<Request, DirectError> {
    client
        .post(ENDPOINT)
        .header(header::AUTHORIZATION, key.0)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "text/event-stream")
        .body(request_body()?)
        .build()
        .map_err(|_| DirectError::Internal)
}
fn network_error(error: reqwest::Error) -> DirectError {
    if error.is_timeout() {
        DirectError::Timeout
    } else {
        DirectError::Network
    }
}
fn status_error(status: StatusCode) -> Result<(), DirectError> {
    match status.as_u16() {
        200 => Ok(()),
        401 => Err(DirectError::Authentication),
        400 | 403 | 404 => Err(DirectError::ModelUnavailable),
        429 => Err(DirectError::RateLimited),
        _ => Err(DirectError::HttpStatus),
    }
}

pub(crate) async fn run(
    key: ApiKey,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    let client = client()?;
    let request = request(&client, key)?;
    let mut response = client.execute(request).await.map_err(network_error)?;
    status_error(response.status())?; // Never read/forward an error response body.
    if response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(';').next())
        .map(str::trim)
        != Some("text/event-stream")
    {
        return Err(DirectError::Protocol);
    }
    let mut decoder = Decoder::default();
    while let Some(chunk) = response.chunk().await.map_err(network_error)? {
        for event in decoder.push(&chunk)? {
            emit(event)?;
        }
        if decoder.complete {
            return Ok(());
        }
    }
    decoder.finish()
}

#[derive(Default)]
struct Decoder {
    pending: Vec<u8>,
    wire_bytes: usize,
    sequence: u64,
    response_id: Option<String>,
    item_id: Option<String>,
    text: String,
    complete: bool,
}
impl Decoder {
    fn finish(&self) -> Result<(), DirectError> {
        if self.complete {
            Ok(())
        } else {
            Err(DirectError::Incomplete)
        }
    }
    fn push(&mut self, chunk: &[u8]) -> Result<Vec<ProviderEvent>, DirectError> {
        self.wire_bytes = self
            .wire_bytes
            .checked_add(chunk.len())
            .ok_or(DirectError::Limit)?;
        if self.wire_bytes > MAX_WIRE || chunk.len() > MAX_FRAME {
            return Err(DirectError::Limit);
        }
        let mut events = Vec::new();
        for byte in chunk {
            if self.complete {
                if !byte.is_ascii_whitespace() {
                    return Err(DirectError::Protocol);
                }
                continue;
            }
            if self.pending.len() >= MAX_FRAME {
                return Err(DirectError::Limit);
            }
            self.pending.push(*byte);
            if self.pending.ends_with(b"\n\n") || self.pending.ends_with(b"\r\n\r\n") {
                let frame = std::mem::take(&mut self.pending);
                if let Some(event) = self.frame(&frame)? {
                    events.push(event);
                }
            }
        }
        Ok(events)
    }
    fn frame(&mut self, bytes: &[u8]) -> Result<Option<ProviderEvent>, DirectError> {
        let frame = std::str::from_utf8(bytes).map_err(|_| DirectError::Protocol)?;
        let mut data = String::new();
        let mut event_name = None;
        for line in frame.lines() {
            if let Some(value) = line.strip_prefix("data:") {
                if !data.is_empty() {
                    data.push('\n');
                }
                data.push_str(value.strip_prefix(' ').unwrap_or(value));
            } else if let Some(value) = line.strip_prefix("event:") {
                if event_name.replace(value.trim()).is_some() {
                    return Err(DirectError::Protocol);
                }
            } else if !line.is_empty() && !line.starts_with(':') {
                return Err(DirectError::Protocol);
            }
        }
        if data.is_empty() {
            return Ok(None);
        }
        let value: Value = serde_json::from_str(&data).map_err(|_| DirectError::Protocol)?;
        let kind = value["type"].as_str().ok_or(DirectError::Protocol)?;
        if event_name.is_some_and(|name| name != kind)
            || self.sequence >= MAX_EVENTS
            || value["sequence_number"].as_u64() != Some(self.sequence)
        {
            return Err(DirectError::Protocol);
        }
        self.sequence += 1;
        if kind == "response.created" {
            if self.response_id.is_some() || value["response"]["status"] != "in_progress" {
                return Err(DirectError::Protocol);
            }
            let id = bounded_id(&value["response"]["id"])?;
            self.response_id = Some(id.clone());
            return Ok(Some(ProviderEvent::Started(id)));
        }
        if self.response_id.is_none() {
            return Err(DirectError::Protocol);
        }
        match kind {
            "response.in_progress" => {
                self.check_response(&value["response"])?;
                if value["response"]["status"] != "in_progress" {
                    return Err(DirectError::Protocol);
                }
            }
            "response.output_item.added" => {
                let item = &value["item"];
                if self.item_id.is_some()
                    || value["output_index"] != 0
                    || item["type"] != "message"
                    || item["role"] != "assistant"
                {
                    return Err(DirectError::Protocol);
                }
                self.item_id = Some(bounded_id(&item["id"])?);
            }
            "response.content_part.added" | "response.content_part.done" => {
                self.check_item(&value)?;
                if value["part"]["type"] == "refusal" {
                    return Err(DirectError::Refused);
                }
                if value["part"]["type"] != "output_text" {
                    return Err(DirectError::Protocol);
                }
                if kind.ends_with(".done") && value["part"]["text"].as_str() != Some(&self.text) {
                    return Err(DirectError::Protocol);
                }
            }
            "response.output_text.delta" => {
                self.check_item(&value)?;
                let delta = value["delta"].as_str().ok_or(DirectError::Protocol)?;
                if delta.is_empty()
                    || delta.len() > 4096
                    || delta.chars().count() > 1024
                    || self.text.len() + delta.len() > 32768
                    || self.text.chars().count() + delta.chars().count() > 8192
                {
                    return Err(DirectError::Limit);
                }
                self.text.push_str(delta);
                return Ok(Some(ProviderEvent::Delta(delta.to_owned())));
            }
            "response.output_text.done" => {
                self.check_item(&value)?;
                if value["text"].as_str() != Some(&self.text) {
                    return Err(DirectError::Protocol);
                }
            }
            "response.output_item.done" => {
                if value["output_index"] != 0 {
                    return Err(DirectError::Protocol);
                }
                self.check_message(&value["item"])?;
            }
            "response.completed" => {
                let response = &value["response"];
                self.check_response(response)?;
                if response["status"] != "completed"
                    || !response["error"].is_null()
                    || !response["incomplete_details"].is_null()
                {
                    return Err(DirectError::Incomplete);
                }
                let output = response["output"].as_array().ok_or(DirectError::Protocol)?;
                if output.len() != 1 || self.text.is_empty() {
                    return Err(DirectError::Protocol);
                }
                self.check_message(&output[0])?;
                self.complete = true;
                return Ok(Some(ProviderEvent::Completed));
            }
            "response.refusal.delta" | "response.refusal.done" => return Err(DirectError::Refused),
            "response.incomplete" => return Err(DirectError::Incomplete),
            "response.failed" => {
                return Err(match value["response"]["error"]["code"].as_str() {
                    Some("server_error") => DirectError::ProviderStreamServerError,
                    Some("rate_limit_exceeded") => DirectError::ProviderStreamRateLimit,
                    Some("invalid_prompt") => DirectError::ProviderStreamInvalidPrompt,
                    Some(code) if !code.is_empty() => DirectError::ProviderStreamFailedUnknownCode,
                    _ => DirectError::ProviderStreamFailedInvalidCode,
                });
            }
            "error" => return Err(DirectError::ProviderStreamErrorEvent),
            _ => return Err(DirectError::Protocol), // Includes every tool/function event.
        }
        Ok(None)
    }
    fn check_response(&self, response: &Value) -> Result<(), DirectError> {
        if response["id"].as_str() == self.response_id.as_deref() {
            Ok(())
        } else {
            Err(DirectError::Protocol)
        }
    }
    fn check_item(&self, value: &Value) -> Result<(), DirectError> {
        if self.item_id.is_some()
            && value["item_id"].as_str() == self.item_id.as_deref()
            && value["output_index"] == 0
            && value["content_index"] == 0
        {
            Ok(())
        } else {
            Err(DirectError::Protocol)
        }
    }
    fn check_message(&self, item: &Value) -> Result<(), DirectError> {
        if self.item_id.is_none()
            || item["id"].as_str() != self.item_id.as_deref()
            || item["type"] != "message"
            || item["role"] != "assistant"
            || item["status"] != "completed"
        {
            return Err(DirectError::Protocol);
        }
        let content = item["content"].as_array().ok_or(DirectError::Protocol)?;
        if content.len() != 1 {
            return Err(DirectError::Protocol);
        }
        if content[0]["type"] == "refusal" {
            return Err(DirectError::Refused);
        }
        if content[0]["type"] != "output_text" || content[0]["text"].as_str() != Some(&self.text) {
            return Err(DirectError::Protocol);
        }
        Ok(())
    }
}
fn bounded_id(value: &Value) -> Result<String, DirectError> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 128 && s.bytes().all(|b| b.is_ascii_graphic()))
        .map(str::to_owned)
        .ok_or(DirectError::Protocol)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn frames() -> Vec<Value> {
        vec![
            json!({"type":"response.created","response":{"id":"resp_fixture","status":"in_progress"}}),
            json!({"type":"response.in_progress","response":{"id":"resp_fixture","status":"in_progress"}}),
            json!({"type":"response.output_item.added","output_index":0,"item":{"id":"msg_fixture","type":"message","role":"assistant"}}),
            json!({"type":"response.output_text.delta","item_id":"msg_fixture","output_index":0,"content_index":0,"delta":"Board 🙂 ready."}),
            json!({"type":"response.completed","response":{"id":"resp_fixture","status":"completed","output":[{"id":"msg_fixture","type":"message","role":"assistant","status":"completed","content":[{"type":"output_text","text":"Board 🙂 ready."}]}]}}),
        ]
    }
    fn wire(mut values: Vec<Value>) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (index, value) in values.iter_mut().enumerate() {
            value["sequence_number"] = json!(index);
            bytes.extend_from_slice(
                format!(
                    "event: {}\r\ndata: {value}\r\n\r\n",
                    value["type"].as_str().unwrap_or("invalid-fixture")
                )
                .as_bytes(),
            );
        }
        bytes
    }
    #[test]
    fn split_stream_and_explicit_completion() -> Result<(), Box<dyn std::error::Error>> {
        let bytes = wire(frames());
        for size in [1, 2, 7, 128, 65536] {
            let mut decoder = Decoder::default();
            let mut count = 0;
            for chunk in bytes.chunks(size) {
                count += decoder.push(chunk)?.len();
            }
            assert!(decoder.complete);
            assert_eq!(count, 3);
            assert_eq!(decoder.text, "Board 🙂 ready.");
        }
        Ok(())
    }
    #[test]
    fn rejects_tools_refusal_incomplete_malformed_and_oversize(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (kind, error) in [
            (
                "response.function_call_arguments.delta",
                DirectError::Protocol,
            ),
            ("response.refusal.delta", DirectError::Refused),
            ("response.incomplete", DirectError::Incomplete),
            (
                "response.failed",
                DirectError::ProviderStreamFailedInvalidCode,
            ),
        ] {
            let mut input = frames();
            input.truncate(3);
            input.push(json!({"type":kind}));
            assert_eq!(Decoder::default().push(&wire(input)).err(), Some(error));
        }
        assert_eq!(
            Decoder::default().push(b"data: invalid\n\n").err(),
            Some(DirectError::Protocol)
        );
        assert_eq!(
            Decoder::default().push(&vec![b'x'; MAX_FRAME + 1]).err(),
            Some(DirectError::Limit)
        );
        let mut input = frames();
        input.pop();
        let mut decoder = Decoder::default();
        decoder.push(&wire(input))?;
        assert!(!decoder.complete);
        assert_eq!(decoder.finish(), Err(DirectError::Incomplete));
        let mut input = frames();
        input[3]["delta"] = json!("x".repeat(4097));
        assert_eq!(
            Decoder::default().push(&wire(input)).err(),
            Some(DirectError::Limit)
        );
        let mut input = frames();
        input[4]["response"]["output"][0]["content"][0]["text"] = json!("changed");
        assert_eq!(
            Decoder::default().push(&wire(input)).err(),
            Some(DirectError::Protocol)
        );
        Ok(())
    }
    #[test]
    fn accepts_documented_text_metadata_and_rejects_identity_or_sequence_drift(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let original = frames();
        let mut input = original[..3].to_vec();
        input.push(json!({"type":"response.content_part.added","item_id":"msg_fixture","output_index":0,"content_index":0,"part":{"type":"output_text","text":"","annotations":[]}}));
        input.push(original[3].clone());
        input.push(json!({"type":"response.output_text.done","item_id":"msg_fixture","output_index":0,"content_index":0,"text":"Board 🙂 ready."}));
        input.push(json!({"type":"response.content_part.done","item_id":"msg_fixture","output_index":0,"content_index":0,"part":{"type":"output_text","text":"Board 🙂 ready.","annotations":[]}}));
        input.push(json!({"type":"response.output_item.done","output_index":0,"item":original[4]["response"]["output"][0]}));
        input.push(original[4].clone());
        let mut decoder = Decoder::default();
        assert_eq!(decoder.push(&wire(input))?.len(), 3);
        assert_eq!(decoder.finish(), Ok(()));
        for field in ["item_id", "output_index", "content_index"] {
            let mut input = frames();
            input[3][field] = json!("wrong");
            assert_eq!(
                Decoder::default().push(&wire(input)).err(),
                Some(DirectError::Protocol)
            );
        }
        let bytes = String::from_utf8(wire(frames()))?
            .replace("\"sequence_number\":3", "\"sequence_number\":2");
        assert_eq!(
            Decoder::default().push(bytes.as_bytes()).err(),
            Some(DirectError::Protocol)
        );
        let mut decoder = Decoder::default();
        for _ in 0..16 {
            decoder.push(&b":\n\n".repeat(MAX_FRAME / 3))?;
        }
        assert_eq!(
            decoder.push(&vec![b' '; MAX_FRAME]).err(),
            Some(DirectError::Limit)
        );
        Ok(())
    }
    #[test]
    fn distinguishes_http_statuses_without_retaining_response_data(
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(status_error(StatusCode::OK), Ok(()));
        for (status, expected) in [
            (400, DirectError::ModelUnavailable),
            (401, DirectError::Authentication),
            (403, DirectError::ModelUnavailable),
            (404, DirectError::ModelUnavailable),
            (429, DirectError::RateLimited),
            (201, DirectError::HttpStatus),
            (204, DirectError::HttpStatus),
            (301, DirectError::HttpStatus),
            (408, DirectError::HttpStatus),
            (500, DirectError::HttpStatus),
            (503, DirectError::HttpStatus),
        ] {
            assert_eq!(status_error(StatusCode::from_u16(status)?), Err(expected));
        }
        // Construct an error without executing a request or opening a socket.
        let error = client()?
            .get("http://[invalid")
            .build()
            .err()
            .ok_or("expected an offline request-construction error")?;
        assert!(!error.is_timeout());
        assert_eq!(network_error(error), DirectError::Network);
        for (error, code) in [
            (DirectError::Network, "network"),
            (DirectError::HttpStatus, "http_status"),
            (DirectError::Timeout, "timeout"),
        ] {
            assert_eq!(serde_json::to_value(error)?, json!(code));
        }
        Ok(())
    }
    #[test]
    fn stream_failure_codes_are_sanitized_after_existing_validation(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (kind, expected, serialized) in [
            (
                "response.failed",
                DirectError::ProviderStreamFailedInvalidCode,
                "provider_stream_failed_invalid_code",
            ),
            (
                "error",
                DirectError::ProviderStreamErrorEvent,
                "provider_stream_error_event",
            ),
        ] {
            let failure = json!({"type":kind,"error":{"message":"DUMMY-PRIVATE-CONTENT"}});
            let input = vec![frames()[0].clone(), failure.clone()];
            let error = Decoder::default()
                .push(&wire(input))
                .err()
                .ok_or("expected a stream failure")?;
            assert_eq!(error, expected);
            assert_eq!(serde_json::to_value(error)?, json!(serialized));
            assert!(!error.to_string().contains("DUMMY-PRIVATE-CONTENT"));
            assert!(!format!("{error:?}").contains("DUMMY-PRIVATE-CONTENT"));
            assert_eq!(
                Decoder::default().push(&wire(vec![failure.clone()])).err(),
                Some(DirectError::Protocol)
            );
            let invalid_sequence = String::from_utf8(wire(vec![frames()[0].clone(), failure]))?
                .replace("\"sequence_number\":1", "\"sequence_number\":9");
            assert_eq!(
                Decoder::default().push(invalid_sequence.as_bytes()).err(),
                Some(DirectError::Protocol)
            );
        }
        Ok(())
    }
    #[test]
    fn failed_response_codes_are_closed_and_payload_free() -> Result<(), Box<dyn std::error::Error>>
    {
        for (code, expected, serialized) in [
            (
                "server_error",
                DirectError::ProviderStreamServerError,
                "provider_stream_server_error",
            ),
            (
                "rate_limit_exceeded",
                DirectError::ProviderStreamRateLimit,
                "provider_stream_rate_limit",
            ),
            (
                "invalid_prompt",
                DirectError::ProviderStreamInvalidPrompt,
                "provider_stream_invalid_prompt",
            ),
        ] {
            let failure = json!({"type":"response.failed", "response":{
                "id":"DUMMY-PRIVATE-CONTENT", "status":"failed",
                "error":{"code":code,"message":"DUMMY-PRIVATE-CONTENT"}},
                "param":"DUMMY-PRIVATE-CONTENT"});
            let error = Decoder::default()
                .push(&wire(vec![frames()[0].clone(), failure.clone()]))
                .err()
                .ok_or("expected failure")?;
            assert_eq!(error, expected);
            assert_eq!(serde_json::to_value(error)?, json!(serialized));
            assert!(!error.to_string().contains("DUMMY-PRIVATE-CONTENT"));
            assert!(!format!("{error:?}").contains("DUMMY-PRIVATE-CONTENT"));
            assert_eq!(
                Decoder::default().push(&wire(vec![failure.clone()])).err(),
                Some(DirectError::Protocol)
            );
            let bytes = String::from_utf8(wire(vec![frames()[0].clone(), failure.clone()]))?;
            assert_eq!(
                Decoder::default()
                    .push(
                        bytes
                            .replace("\"sequence_number\":1", "\"sequence_number\":9")
                            .as_bytes()
                    )
                    .err(),
                Some(DirectError::Protocol)
            );
            assert_eq!(
                Decoder::default()
                    .push(
                        bytes
                            .replace("event: response.failed", "event: error")
                            .as_bytes()
                    )
                    .err(),
                Some(DirectError::Protocol)
            );
            // The open-ended top-level error schema never inherits ResponseError meanings.
            let mut top = failure;
            top["type"] = json!("error");
            top["code"] = json!(code);
            assert_eq!(
                Decoder::default()
                    .push(&wire(vec![frames()[0].clone(), top]))
                    .err(),
                Some(DirectError::ProviderStreamErrorEvent)
            );
        }
        Ok(())
    }
    #[test]
    fn unrecognized_or_misplaced_stream_codes_identify_only_the_failure_shape(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for code in [
            json!(null),
            json!(42),
            json!(true),
            json!([]),
            json!({"code":"server_error"}),
            json!(""),
            json!("DUMMY-PRIVATE-CONTENT"),
            json!("SERVER_ERROR"),
            json!(" server_error"),
            json!("server_error "),
            json!("server_error:DUMMY-PRIVATE-CONTENT"),
            json!("data_residency_mismatch"),
            json!("invalid_image"),
            json!("x".repeat(8192)),
        ] {
            for kind in ["response.failed", "error"] {
                let (expected, serialized) = if kind == "error" {
                    (
                        DirectError::ProviderStreamErrorEvent,
                        "provider_stream_error_event",
                    )
                } else if code.as_str().is_some_and(|code| !code.is_empty()) {
                    (
                        DirectError::ProviderStreamFailedUnknownCode,
                        "provider_stream_failed_unknown_code",
                    )
                } else {
                    (
                        DirectError::ProviderStreamFailedInvalidCode,
                        "provider_stream_failed_invalid_code",
                    )
                };
                let event = json!({"type":kind,"code":code,"response":{"error":{"code":code,
                    "message":"DUMMY-PRIVATE-CONTENT"}},"message":"DUMMY-PRIVATE-CONTENT"});
                let error = Decoder::default()
                    .push(&wire(vec![frames()[0].clone(), event]))
                    .err()
                    .ok_or("expected failure")?;
                assert_eq!(error, expected);
                assert_eq!(serde_json::to_value(error)?, json!(serialized));
                assert!(!error.to_string().contains("DUMMY-PRIVATE-CONTENT"));
                assert!(!format!("{error:?}").contains("DUMMY-PRIVATE-CONTENT"));
            }
        }
        for (event, expected) in [
            (
                json!({"type":"response.failed"}),
                DirectError::ProviderStreamFailedInvalidCode,
            ),
            (
                json!({"type":"response.failed","response":null}),
                DirectError::ProviderStreamFailedInvalidCode,
            ),
            (
                json!({"type":"response.failed","response":42}),
                DirectError::ProviderStreamFailedInvalidCode,
            ),
            (
                json!({"type":"response.failed","response":{"error":[]}}),
                DirectError::ProviderStreamFailedInvalidCode,
            ),
            (
                json!({"type":"response.failed","response":{"error":{"message":"server_error"}}}),
                DirectError::ProviderStreamFailedInvalidCode,
            ),
            (
                json!({"type":"response.failed","code":"server_error","error":{"code":"server_error"}}),
                DirectError::ProviderStreamFailedInvalidCode,
            ),
            (
                json!({"type":"error","response":{"error":{"code":"server_error"}}}),
                DirectError::ProviderStreamErrorEvent,
            ),
        ] {
            assert_eq!(
                Decoder::default()
                    .push(&wire(vec![frames()[0].clone(), event]))
                    .err(),
                Some(expected)
            );
        }
        Ok(())
    }
    #[test]
    fn failure_stages_are_payload_free_across_split_and_coalesced_frames(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (failure, expected, serialized, message) in [
            (
                json!({"type":"error","code":"server_error","message":"DUMMY-PRIVATE-CONTENT",
                    "param":"DUMMY-PRIVATE-CONTENT","response":{"error":{"code":"invalid_prompt"}}}),
                DirectError::ProviderStreamErrorEvent,
                "provider_stream_error_event",
                "The provider emitted a top-level error event in the response stream. No automatic retry was made.",
            ),
            (
                json!({"type":"response.failed","response":{"error":{
                    "code":"DUMMY-PRIVATE-CONTENT","message":"DUMMY-PRIVATE-CONTENT"}}}),
                DirectError::ProviderStreamFailedUnknownCode,
                "provider_stream_failed_unknown_code",
                "The provider reported response.failed with an unrecognized error code. No automatic retry was made.",
            ),
            (
                json!({"type":"response.failed","response":{"error":{
                    "code":{"message":"DUMMY-PRIVATE-CONTENT"},"message":"DUMMY-PRIVATE-CONTENT"}}}),
                DirectError::ProviderStreamFailedInvalidCode,
                "provider_stream_failed_invalid_code",
                "The provider reported response.failed without a usable error code. No automatic retry was made.",
            ),
        ] {
            let input = wire(vec![frames()[0].clone(), failure.clone()]);
            for size in [1, 2, 7, 128, 65536] {
                let mut decoder = Decoder::default();
                let mut actual = None;
                for chunk in input.chunks(size) {
                    if let Err(error) = decoder.push(chunk) {
                        actual = Some(error);
                        break;
                    }
                }
                assert_eq!(actual, Some(expected));
                assert!(!decoder.complete);
            }
            assert_eq!(expected.to_string(), message);
            assert_eq!(serde_json::to_value(expected)?, json!(serialized));
            for rendered in [
                expected.to_string(),
                format!("{expected:?}"),
                serde_json::to_string(&expected)?,
            ] {
                assert!(!rendered.contains("DUMMY-PRIVATE-CONTENT"));
            }
            assert_eq!(
                Decoder::default().push(&wire(vec![failure])).err(),
                Some(DirectError::Protocol)
            );
            let valid_wire = String::from_utf8(input)?;
            for invalid_wire in [
                valid_wire.replace("\"sequence_number\":1", "\"sequence_number\":9"),
                valid_wire
                    .replace("event: response.failed", "event: unexpected")
                    .replace("event: error", "event: unexpected"),
                valid_wire.replace("\"type\":\"response.created\"", "\"type\":false"),
            ] {
                assert_eq!(
                    Decoder::default().push(invalid_wire.as_bytes()).err(),
                    Some(DirectError::Protocol)
                );
            }
        }
        Ok(())
    }
    #[test]
    fn credentials_and_request_are_closed_and_redacted() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            ApiKey::from_values(true, true, None).err(),
            Some(DirectError::MissingKey)
        );
        assert_eq!(
            ApiKey::from_values(false, true, Some("dummy".into())).err(),
            Some(DirectError::Disabled)
        );
        let key = ApiKey::from_values(true, true, Some("DUMMY-TEST-CREDENTIAL".into()))?;
        assert!(!format!("{key:?}").contains("DUMMY"));
        let request = request(&client()?, key)?;
        assert_eq!(request.url().as_str(), ENDPOINT);
        assert_eq!(request.method(), reqwest::Method::POST);
        assert!(!format!("{request:?}").contains("DUMMY"));
        let body: Value = serde_json::from_slice(
            request
                .body()
                .ok_or("missing request body")?
                .as_bytes()
                .ok_or("missing request bytes")?,
        )?;
        assert_eq!(body["input"], SAMPLE);
        assert_eq!(body["model"], MODEL);
        assert_eq!(body["tools"], json!([]));
        assert_eq!(body["store"], false);
        assert_eq!(body["background"], false);
        assert_eq!(body["stream"], true);
        assert_eq!(body["max_output_tokens"], 512);
        assert!(body.get("conversation").is_none());
        for (status, error) in [
            (401, DirectError::Authentication),
            (404, DirectError::ModelUnavailable),
            (429, DirectError::RateLimited),
            (500, DirectError::HttpStatus),
        ] {
            assert_eq!(status_error(StatusCode::from_u16(status)?), Err(error));
            assert!(!error.to_string().contains("DUMMY"));
        }
        Ok(())
    }
}
