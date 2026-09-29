//! Fixed, opt-in development transport. No endpoint or credential is supplied by IPC.
use std::{fmt, time::Duration};

use reqwest::{header, Client, Request, StatusCode};
use serde::Serialize;
use serde_json::{json, Value};

static GENERATION_OWNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Retained by either native generation future through transport cleanup.
pub(crate) struct GenerationLease;
impl GenerationLease {
    pub(crate) fn acquire() -> Result<Self, DirectError> {
        GENERATION_OWNED
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            )
            .map(|_| Self)
            .map_err(|_| DirectError::Busy)
    }
}
impl Drop for GenerationLease {
    fn drop(&mut self) {
        GENERATION_OWNED.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

pub(crate) const ENDPOINT: &str = "https://api.openai.com/v1/responses";
pub(crate) const MODEL: &str = "gpt-5.6-luna";
pub(crate) const SAMPLE: &str = crate::agent::runtime::PERSONAL_ASSISTANT_V0_SYNTHETIC_FIXTURE;
const MAX_FRAME: usize = 65_536;
const MAX_WIRE: usize = 1_048_576;
const MAX_EVENTS: u64 = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DirectError {
    #[error("The selected provider is unavailable. No fallback was used.")]
    ProviderUnavailable,
    #[error("The selected model is unavailable or not loaded. Check the selected server.")]
    ModelUnavailable,
    #[error("The server reported insufficient memory. Choose a smaller already-installed model or context.")]
    OutOfMemory,
    #[error("The response reached its output or context limit and is incomplete.")]
    Truncated,
    #[error("The selected model or response mode is unsupported. No fallback was used.")]
    Unsupported,
    #[error("Use a credential-free loopback HTTP(S) endpoint ending in /v1.")]
    Endpoint,
    #[error(
        "Locality is unknown. Explicitly acknowledge transmitting notes to this destination first."
    )]
    Locality,
    #[error("Refresh the model catalog for this exact connection before sending.")]
    Catalog,
    #[error("Native live mode is disabled.")]
    Disabled,
    #[error("Set the native session API key before starting.")]
    MissingKey,
    #[error("The provider rejected authentication.")]
    Authentication,
    #[error("OpenAI rejected the request (HTTP 400). No automatic retry was made.")]
    HttpBadRequest,
    #[error("OpenAI denied access (HTTP 403). No automatic retry was made.")]
    HttpForbidden,
    #[error(
        "OpenAI could not find the requested resource (HTTP 404). No automatic retry was made."
    )]
    HttpNotFound,
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
        400 => Err(DirectError::HttpBadRequest),
        403 => Err(DirectError::HttpForbidden),
        404 => Err(DirectError::HttpNotFound),
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

/// Native-owned configurable requests use the same no-retry transport, with a
/// separately enabled bounded reasoning-item decoder. The fixed sample stays strict.
pub(crate) async fn run_configured(
    key: ApiKey,
    body: Vec<u8>,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    let client = client()?;
    let request = configured_request(&client, key, body)?;
    let mut response = client.execute(request).await.map_err(network_error)?;
    status_error(response.status())?;
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
    let mut decoder = Decoder::configured();
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

fn configured_request(client: &Client, key: ApiKey, body: Vec<u8>) -> Result<Request, DirectError> {
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(DirectError::Limit);
    }
    client
        .post(ENDPOINT)
        .header(header::AUTHORIZATION, key.0)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "text/event-stream")
        .body(body)
        .build()
        .map_err(|_| DirectError::Internal)
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
    configured: bool,
    reasoning_id: Option<String>,
    reasoning_done: bool,
    message_done: bool,
}
impl Decoder {
    fn configured() -> Self {
        Self {
            configured: true,
            ..Self::default()
        }
    }
    fn message_index(&self) -> u64 {
        u64::from(self.reasoning_id.is_some())
    }
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
                if self.configured && item["type"] == "reasoning" {
                    if self.reasoning_id.is_some()
                        || self.item_id.is_some()
                        || value["output_index"] != 0
                    {
                        return Err(DirectError::Protocol);
                    }
                    check_reasoning(item, "in_progress")?;
                    self.reasoning_id = Some(bounded_id(&item["id"])?);
                    return Ok(None);
                }
                if self.item_id.is_some()
                    || value["output_index"] != self.message_index()
                    || item["type"] != "message"
                    || item["role"] != "assistant"
                    || (self.configured
                        && ((self.reasoning_id.is_some() && !self.reasoning_done)
                            || item["id"].as_str() == self.reasoning_id.as_deref()
                            || item["status"] != "in_progress"
                            || item["content"]
                                .as_array()
                                .is_none_or(|parts| !parts.is_empty())))
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
                let item = &value["item"];
                if self.configured && item["type"] == "reasoning" {
                    if self.reasoning_id.is_none()
                        || self.reasoning_done
                        || self.item_id.is_some()
                        || value["output_index"] != 0
                        || item["id"].as_str() != self.reasoning_id.as_deref()
                    {
                        return Err(DirectError::Protocol);
                    }
                    check_reasoning(item, "completed")?;
                    self.reasoning_done = true;
                    return Ok(None);
                }
                if value["output_index"] != self.message_index()
                    || (self.configured && self.message_done)
                {
                    return Err(DirectError::Protocol);
                }
                self.check_message(item)?;
                self.message_done = true;
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
                let message_index = usize::from(self.reasoning_id.is_some());
                if output.len() != message_index + 1
                    || self.text.is_empty()
                    || (self.configured && !self.message_done)
                {
                    return Err(DirectError::Protocol);
                }
                if let Some(reasoning_id) = &self.reasoning_id {
                    if !self.reasoning_done || output[0]["id"].as_str() != Some(reasoning_id) {
                        return Err(DirectError::Protocol);
                    }
                    check_reasoning(&output[0], "completed")?;
                }
                self.check_message(&output[message_index])?;
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
            && value["output_index"] == self.message_index()
            && value["content_index"] == 0
            && !(self.configured && self.message_done)
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
// Reasoning is framing only: no summary, reasoning text, or encrypted payload is
// retained or emitted. Unknown event types still fail closed in Decoder::frame.
fn check_reasoning(item: &Value, expected_status: &str) -> Result<(), DirectError> {
    bounded_id(&item["id"])?;
    if item["type"] != "reasoning"
        || item["summary"]
            .as_array()
            .is_none_or(|summary| !summary.is_empty())
        || item
            .get("status")
            .is_some_and(|status| status != expected_status)
        || item
            .get("content")
            .is_some_and(|content| content.as_array().is_none_or(|parts| !parts.is_empty()))
        || item
            .get("encrypted_content")
            .is_some_and(|value| !value.is_null() && !value.is_string())
    {
        return Err(DirectError::Protocol);
    }
    Ok(())
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
    fn configured_frames(with_reasoning: bool) -> Vec<Value> {
        let original = frames();
        let mut values = original[..2].to_vec();
        let reasoning = json!({"id":"rs_fixture","type":"reasoning","summary":[],
            "encrypted_content":"DUMMY-OPAQUE-REASONING"});
        if with_reasoning {
            values.push(json!({"type":"response.output_item.added","output_index":0,
                "item":reasoning}));
            values.push(json!({"type":"response.output_item.done","output_index":0,
                "item":reasoning}));
        }
        let index = u64::from(with_reasoning);
        let mut added = original[2].clone();
        added["output_index"] = json!(index);
        added["item"]["status"] = json!("in_progress");
        added["item"]["content"] = json!([]);
        values.push(added);
        let mut delta = original[3].clone();
        delta["output_index"] = json!(index);
        values.push(delta);
        let message = original[4]["response"]["output"][0].clone();
        values
            .push(json!({"type":"response.output_item.done","output_index":index,"item":message}));
        let mut completed = original[4].clone();
        if with_reasoning {
            completed["response"]["output"] = json!([reasoning, message]);
        }
        values.push(completed);
        values
    }
    #[test]
    fn configured_reasoning_is_hidden_and_completion_is_explicit(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for with_reasoning in [false, true] {
            let input = wire(configured_frames(with_reasoning));
            for size in [1, 7, 128, 65536] {
                let mut decoder = Decoder::configured();
                let mut starts = 0;
                let mut completions = 0;
                let mut visible = String::new();
                for chunk in input.chunks(size) {
                    for event in decoder.push(chunk)? {
                        match event {
                            ProviderEvent::Started(id) => {
                                assert_eq!(id, "resp_fixture");
                                starts += 1;
                            }
                            ProviderEvent::Delta(text) => visible.push_str(&text),
                            ProviderEvent::Completed => completions += 1,
                        }
                    }
                }
                assert_eq!(starts, 1);
                assert_eq!(completions, 1);
                assert_eq!(visible, "Board 🙂 ready.");
                assert!(!visible.contains("DUMMY-OPAQUE-REASONING"));
                assert_eq!(decoder.finish(), Ok(()));
            }
            let mut incomplete = configured_frames(with_reasoning);
            incomplete.pop();
            let mut decoder = Decoder::configured();
            decoder.push(&wire(incomplete))?;
            assert_eq!(decoder.finish(), Err(DirectError::Incomplete));
        }
        // Reasoning support is never silently enabled for the fixed sample.
        assert_eq!(
            Decoder::default()
                .push(&wire(configured_frames(true)))
                .err(),
            Some(DirectError::Protocol)
        );
        Ok(())
    }
    #[test]
    fn configured_reasoning_rejects_malformed_items_and_identity_drift() {
        for (frame, field, replacement) in [
            (2, "output_index", json!(1)),
            (3, "output_index", json!(1)),
            (4, "output_index", json!(0)),
            (5, "output_index", json!(0)),
            (5, "content_index", json!(1)),
            (5, "item_id", json!("rs_fixture")),
            (6, "output_index", json!(0)),
        ] {
            let mut values = configured_frames(true);
            values[frame][field] = replacement;
            assert_eq!(
                Decoder::configured().push(&wire(values)).err(),
                Some(DirectError::Protocol)
            );
        }
        for (frame, field, replacement) in [
            (2, "id", json!("")),
            (2, "id", json!("x".repeat(129))),
            (2, "summary", json!(null)),
            (
                2,
                "summary",
                json!([{"type":"summary_text","text":"DUMMY-HIDDEN"}]),
            ),
            (
                2,
                "content",
                json!([{"type":"reasoning_text","text":"DUMMY-HIDDEN"}]),
            ),
            (2, "encrypted_content", json!(42)),
            (2, "status", json!("completed")),
            (3, "id", json!("rs_other")),
            (3, "status", json!("in_progress")),
            (3, "type", json!("function_call")),
            (4, "id", json!("rs_fixture")),
            (4, "role", json!("user")),
            (4, "status", json!("completed")),
            (
                4,
                "content",
                json!([{"type":"output_text","text":"unstreamed"}]),
            ),
            (6, "id", json!("msg_other")),
            (6, "status", json!("in_progress")),
        ] {
            let mut values = configured_frames(true);
            values[frame]["item"][field] = replacement;
            assert_eq!(
                Decoder::configured().push(&wire(values)).err(),
                Some(DirectError::Protocol)
            );
        }
        for malformed in [
            json!([]),
            json!({"id":"rs_other","type":"reasoning","summary":[]}),
            json!({"id":"rs_fixture","type":"function_call","summary":[]}),
        ] {
            let mut values = configured_frames(true);
            values[7]["response"]["output"][0] = malformed;
            assert_eq!(
                Decoder::configured().push(&wire(values)).err(),
                Some(DirectError::Protocol)
            );
        }
    }
    #[test]
    fn configured_reasoning_rejects_invalid_order_extra_items_and_unknown_events(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for mutation in 0..8 {
            let mut values = configured_frames(true);
            match mutation {
                0 => {
                    values.remove(3);
                } // Message before reasoning done.
                1 => {
                    values.remove(2);
                } // Done without reasoning added.
                2 => {
                    values.insert(3, values[2].clone());
                } // Duplicate reasoning.
                3 => {
                    values.insert(4, values[3].clone());
                } // Duplicate done.
                4 => {
                    values.remove(6);
                } // Completion without message done.
                5 => {
                    values.insert(7, values[5].clone());
                } // Delta after done.
                6 => {
                    values[7]["response"]["output"]
                        .as_array_mut()
                        .ok_or("output")?
                        .swap(0, 1);
                }
                _ => {
                    values[7]["response"]["output"]
                        .as_array_mut()
                        .ok_or("output")?
                        .push(json!({"type":"function_call"}));
                }
            }
            assert_eq!(
                Decoder::configured().push(&wire(values)).err(),
                Some(DirectError::Protocol)
            );
        }
        for kind in [
            "response.reasoning_summary_text.delta",
            "response.reasoning_text.delta",
            "response.function_call_arguments.delta",
            "response.unknown",
        ] {
            let values = vec![
                frames()[0].clone(),
                json!({"type":kind,"delta":"DUMMY-HIDDEN"}),
            ];
            assert_eq!(
                Decoder::configured().push(&wire(values)).err(),
                Some(DirectError::Protocol)
            );
        }
        let mut values = configured_frames(false);
        values[2]["item"]["type"] = json!("function_call");
        assert_eq!(
            Decoder::configured().push(&wire(values)).err(),
            Some(DirectError::Protocol)
        );
        let sequence_drift = String::from_utf8(wire(configured_frames(true)))?
            .replace("\"sequence_number\":3", "\"sequence_number\":2");
        assert_eq!(
            Decoder::configured().push(sequence_drift.as_bytes()).err(),
            Some(DirectError::Protocol)
        );
        Ok(())
    }
    #[test]
    fn configured_request_preserves_native_body_and_transport_identity(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let body = serde_json::to_vec(&json!({"model":"gpt-5.6-luna","stream":true,
            "input":"DUMMY-NATIVE-CONTEXT","tools":[],"tool_choice":"none"}))?;
        let key = ApiKey::from_values(true, true, Some("DUMMY-PRIVATE-KEY".to_owned()))?;
        let request = configured_request(&client()?, key, body.clone())?;
        assert_eq!(request.url().as_str(), ENDPOINT);
        assert_eq!(request.method(), reqwest::Method::POST);
        assert_eq!(
            request.body().and_then(reqwest::Body::as_bytes),
            Some(body.as_slice())
        );
        assert!(request.headers()[header::AUTHORIZATION].is_sensitive());
        for body in [vec![], vec![b'x'; MAX_FRAME + 1]] {
            let key = ApiKey::from_values(true, true, Some("DUMMY-PRIVATE-KEY".to_owned()))?;
            assert_eq!(
                configured_request(&client()?, key, body).err(),
                Some(DirectError::Limit)
            );
        }
        Ok(())
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
            (400, DirectError::HttpBadRequest),
            (401, DirectError::Authentication),
            (403, DirectError::HttpForbidden),
            (404, DirectError::HttpNotFound),
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
            (DirectError::HttpBadRequest, "http_bad_request"),
            (DirectError::HttpForbidden, "http_forbidden"),
            (DirectError::HttpNotFound, "http_not_found"),
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
            (404, DirectError::HttpNotFound),
            (429, DirectError::RateLimited),
            (500, DirectError::HttpStatus),
        ] {
            assert_eq!(status_error(StatusCode::from_u16(status)?), Err(error));
            assert!(!error.to_string().contains("DUMMY"));
        }
        Ok(())
    }
}
