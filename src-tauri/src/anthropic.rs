//! Opt-in native Messages transport and explicit bounded Models discovery.
//! Text-only follow-ups omit prior thinking, as permitted outside tool use.
use std::{collections::HashSet, fmt, time::Duration};

use reqwest::{header, Client, Request, StatusCode};
use serde_json::Value;

use crate::{
    agent_models::ModelInfo,
    agent_preferences::ReasoningEffort,
    personal_assistant_direct::{DirectError, ProviderEvent},
};

const MESSAGES: &str = "https://api.anthropic.com/v1/messages";
const MODELS: &str = "https://api.anthropic.com/v1/models";
const MAX_FRAME: usize = 65_536;
const MAX_WIRE: usize = 1_048_576;
const MAX_EVENTS: usize = 2048;
const PAGE_SIZE: usize = 100;
const MAX_PAGES: usize = 8;
const MAX_MODELS: usize = 200;
const MAX_CAPABILITIES: usize = 16_384;

pub(crate) struct AnthropicKey(header::HeaderValue);
impl fmt::Debug for AnthropicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AnthropicKey([REDACTED])")
    }
}
impl AnthropicKey {
    fn from_values(
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
        if value.len() > 512 || !value.bytes().all(|byte| byte.is_ascii_graphic()) {
            return Err(DirectError::MissingKey);
        }
        let mut value =
            header::HeaderValue::from_str(&value).map_err(|_| DirectError::MissingKey)?;
        value.set_sensitive(true);
        Ok(Self(value))
    }
    /// Invoked only by an explicit owner discovery/send action, never startup.
    pub(crate) fn from_environment() -> Result<Self, DirectError> {
        if !cfg!(debug_assertions) || std::env::var("CORTEXA_ANTHROPIC_DEMO").as_deref() != Ok("1")
        {
            return Err(DirectError::Disabled);
        }
        Self::from_values(true, true, std::env::var("ANTHROPIC_API_KEY").ok())
    }
}

pub(crate) fn documented_models() -> Vec<ModelInfo> {
    use ReasoningEffort::{Default, High, Low, Max, Medium, Xhigh};
    // Official public lifecycle and effort documentation verified 2026-09-22.
    // Restricted Mythos models are not advertised as generally accessible.
    [
        (
            "claude-fable-5-1",
            "Claude Fable 5.1",
            "always_on",
            vec![Default, Low, Medium, High, Xhigh, Max],
        ),
        (
            "claude-opus-5-5",
            "Claude Opus 5.5",
            "always_on",
            vec![Default, Low, Medium, High, Xhigh, Max],
        ),
        (
            "claude-sonnet-5",
            "Claude Sonnet 5",
            "default",
            vec![Default, Low, Medium, High, Xhigh, Max],
        ),
        (
            "claude-haiku-4-5-20251001",
            "Claude Haiku 4.5",
            "default",
            vec![Default],
        ),
        (
            "claude-opus-5",
            "Claude Opus 5 (active legacy)",
            "default",
            vec![Default, Low, Medium, High, Xhigh, Max],
        ),
        (
            "claude-fable-5",
            "Claude Fable 5 (active legacy)",
            "always_on",
            vec![Default, Low, Medium, High, Xhigh, Max],
        ),
        (
            "claude-opus-4-8",
            "Claude Opus 4.8 (active legacy)",
            "default",
            vec![Default, Low, Medium, High, Xhigh, Max],
        ),
        (
            "claude-opus-4-7",
            "Claude Opus 4.7 (active legacy)",
            "default",
            vec![Default, Low, Medium, High, Xhigh, Max],
        ),
        (
            "claude-opus-4-6",
            "Claude Opus 4.6 (active legacy)",
            "default",
            vec![Default, Low, Medium, High, Max],
        ),
        (
            "claude-opus-4-5-20251101",
            "Claude Opus 4.5 (active legacy)",
            "default",
            vec![Default, Low, Medium, High],
        ),
        (
            "claude-sonnet-4-6",
            "Claude Sonnet 4.6 (active legacy)",
            "default",
            vec![Default, Low, Medium, High, Max],
        ),
        (
            "claude-sonnet-4-5-20250929",
            "Claude Sonnet 4.5 (active legacy)",
            "default",
            vec![Default],
        ),
    ]
    .into_iter()
    .map(|(id, label, thinking, efforts)| {
        let mut model = ModelInfo::unknown(id.into());
        model.label = label.into();
        model.evidence = "documented".into();
        model.efforts = efforts;
        model.thinking = thinking.into();
        model.locality = "hosted".into();
        // Exact numeric limits are populated only from returned Models metadata.
        model
    })
    .collect()
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
        401 | 403 => Err(DirectError::Authentication),
        404 => Err(DirectError::ModelUnavailable),
        429 => Err(DirectError::RateLimited),
        _ => Err(DirectError::HttpStatus),
    }
}
fn request(client: &Client, key: AnthropicKey, body: Vec<u8>) -> Result<Request, DirectError> {
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(DirectError::Limit);
    }
    client
        .post(MESSAGES)
        .header("x-api-key", key.0)
        .header("anthropic-version", "2023-06-01")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "text/event-stream")
        .body(body)
        .build()
        .map_err(|_| DirectError::Internal)
}
pub(crate) async fn run(
    key: AnthropicKey,
    body: Vec<u8>,
    mut on_event: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    let client = client()?;
    let mut response = client
        .execute(request(&client, key, body)?)
        .await
        .map_err(network_error)?;
    status_error(response.status())?; // Never read or forward unsuccessful bodies.
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
        decoder.push(&chunk, &mut on_event)?;
        if decoder.complete {
            return Ok(());
        }
    }
    decoder.finish()
}

/// Explicit metadata request only; generation is never used to discover models.
pub(crate) async fn discover() -> Result<Vec<ModelInfo>, DirectError> {
    let key = AnthropicKey::from_environment()?;
    tokio::time::timeout(Duration::from_secs(30), discover_with_key(key))
        .await
        .map_err(|_| DirectError::Timeout)?
}
async fn discover_with_key(key: AnthropicKey) -> Result<Vec<ModelInfo>, DirectError> {
    let client = client()?;
    let mut cursor: Option<String> = None;
    let mut catalog = Catalog::default();
    for _ in 0..MAX_PAGES {
        let mut url = reqwest::Url::parse(MODELS).map_err(|_| DirectError::Internal)?;
        url.query_pairs_mut().append_pair("limit", "100");
        if let Some(id) = &cursor {
            url.query_pairs_mut().append_pair("after_id", id);
        }
        let request = client
            .get(url)
            .header("x-api-key", key.0.clone())
            .header("anthropic-version", "2023-06-01")
            .header(header::ACCEPT, "application/json");
        let mut response = request.send().await.map_err(network_error)?;
        status_error(response.status())?;
        if response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(';').next())
            .map(str::trim)
            != Some("application/json")
        {
            return Err(DirectError::Catalog);
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(network_error)? {
            if bytes.len().saturating_add(chunk.len()) > MAX_WIRE {
                return Err(DirectError::Catalog);
            }
            bytes.extend_from_slice(&chunk);
        }
        cursor = catalog.append(&bytes)?;
        if cursor.is_none() {
            return Ok(catalog.models);
        }
    }
    Err(DirectError::Catalog)
}

#[derive(Default)]
struct Catalog {
    models: Vec<ModelInfo>,
    seen: HashSet<String>,
    wire_bytes: usize,
    pages: usize,
}
impl Catalog {
    fn append(&mut self, bytes: &[u8]) -> Result<Option<String>, DirectError> {
        self.wire_bytes = self.wire_bytes.saturating_add(bytes.len());
        self.pages += 1;
        if self.wire_bytes > MAX_WIRE || self.pages > MAX_PAGES {
            return Err(DirectError::Catalog);
        }
        let value: Value = serde_json::from_slice(bytes).map_err(|_| DirectError::Catalog)?;
        let data = value["data"].as_array().ok_or(DirectError::Catalog)?;
        let has_more = value["has_more"].as_bool().ok_or(DirectError::Catalog)?;
        if data.len() > PAGE_SIZE
            || self.models.len().saturating_add(data.len()) > MAX_MODELS
            || (has_more && data.is_empty())
        {
            return Err(DirectError::Catalog);
        }
        let mut last_id = None;
        for item in data {
            if item["type"] != "model" {
                return Err(DirectError::Catalog);
            }
            let id = catalog_string(&item["id"], 128)?;
            if !id.bytes().all(|byte| byte.is_ascii_graphic()) || !self.seen.insert(id.clone()) {
                return Err(DirectError::Catalog);
            }
            let mut model = ModelInfo::unknown(id.clone());
            model.label = catalog_string(&item["display_name"], 256)?;
            model.availability = "available".into();
            model.locality = "hosted".into();
            model.context_limit = optional_limit(&item["max_input_tokens"])?;
            model.output_limit = optional_limit(&item["max_tokens"])?;
            model.thinking = documented_models()
                .into_iter()
                .find(|m| m.id == id)
                .map(|m| m.thinking)
                .unwrap_or_else(|| "unknown".into());
            if !item["capabilities"].is_null() {
                let capabilities = &item["capabilities"];
                if !capabilities.is_object()
                    || !bounded_metadata(capabilities, 0)
                    || serde_json::to_vec(capabilities)
                        .map_err(|_| DirectError::Catalog)?
                        .len()
                        > MAX_CAPABILITIES
                {
                    return Err(DirectError::Catalog);
                }
                if capabilities["effort"]["supported"] == true {
                    for effort in [
                        ReasoningEffort::Low,
                        ReasoningEffort::Medium,
                        ReasoningEffort::High,
                        ReasoningEffort::Xhigh,
                        ReasoningEffort::Max,
                    ] {
                        if capabilities["effort"][effort.as_str()]["supported"] == true {
                            model.efforts.push(effort);
                        }
                    }
                }
                if capabilities["thinking"]["supported"] == false {
                    model.thinking = "unsupported".into();
                }
                model.capabilities = Some(capabilities.clone());
            }
            self.models.push(model);
            last_id = Some(id);
        }
        if has_more {
            let cursor = catalog_string(&value["last_id"], 128)?;
            if last_id.as_deref() != Some(&cursor) {
                return Err(DirectError::Catalog);
            }
            Ok(Some(cursor))
        } else {
            Ok(None)
        }
    }
}
fn catalog_string(value: &Value, maximum: usize) -> Result<String, DirectError> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= maximum && !s.chars().any(char::is_control))
        .map(str::to_owned)
        .ok_or(DirectError::Catalog)
}
fn optional_limit(value: &Value) -> Result<Option<u64>, DirectError> {
    if value.is_null() {
        return Ok(None);
    }
    value
        .as_u64()
        .map(|limit| (limit > 0).then_some(limit))
        .ok_or(DirectError::Catalog)
}
fn bounded_metadata(value: &Value, depth: usize) -> bool {
    if depth > 8 {
        return false;
    }
    match value {
        Value::String(text) => text.len() <= 256 && !text.chars().any(char::is_control),
        Value::Array(items) => {
            items.len() <= 64 && items.iter().all(|v| bounded_metadata(v, depth + 1))
        }
        Value::Object(items) => {
            items.len() <= 64
                && items.iter().all(|(key, v)| {
                    key.len() <= 128
                        && !key.chars().any(char::is_control)
                        && bounded_metadata(v, depth + 1)
                })
        }
        _ => true,
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Block {
    Text,
    Thinking,
    Redacted,
}
#[derive(Default)]
struct Decoder {
    pending: Vec<u8>,
    wire_bytes: usize,
    events: usize,
    started: bool,
    block: Option<Block>,
    next_index: u64,
    text_bytes: usize,
    text_characters: usize,
    has_text: bool,
    end_turn: bool,
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
    fn push(
        &mut self,
        bytes: &[u8],
        mut on_event: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
    ) -> Result<(), DirectError> {
        self.wire_bytes = self.wire_bytes.saturating_add(bytes.len());
        if self.wire_bytes > MAX_WIRE {
            return Err(DirectError::Limit);
        }
        for byte in bytes {
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
                    // Deliver accepted text before processing a later failure in
                    // this same network chunk. A failure never emits completion.
                    on_event(event)?;
                }
            }
        }
        Ok(())
    }
    fn frame(&mut self, bytes: &[u8]) -> Result<Option<ProviderEvent>, DirectError> {
        self.events += 1;
        if self.events > MAX_EVENTS {
            return Err(DirectError::Limit);
        }
        let frame = std::str::from_utf8(bytes).map_err(|_| DirectError::Protocol)?;
        let mut name = None;
        let mut data = String::new();
        for line in frame.lines() {
            if let Some(value) = line.strip_prefix("data:") {
                if !data.is_empty() {
                    data.push('\n');
                }
                data.push_str(value.strip_prefix(' ').unwrap_or(value));
            } else if let Some(value) = line.strip_prefix("event:") {
                if name.replace(value.trim()).is_some() {
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
        if name.is_some_and(|name| name != kind) {
            return Err(DirectError::Protocol);
        }
        if kind == "ping" {
            return Ok(None);
        }
        if kind == "error" {
            return Err(stream_error(&value));
        }
        if kind == "message_start" {
            let message = &value["message"];
            if self.started
                || message["type"] != "message"
                || message["role"] != "assistant"
                || !message["stop_reason"].is_null()
                || message["content"]
                    .as_array()
                    .is_none_or(|blocks| !blocks.is_empty())
            {
                return Err(DirectError::Protocol);
            }
            let id = message["id"]
                .as_str()
                .filter(|id| {
                    !id.is_empty() && id.len() <= 128 && id.bytes().all(|b| b.is_ascii_graphic())
                })
                .ok_or(DirectError::Protocol)?;
            self.started = true;
            return Ok(Some(ProviderEvent::Started(id.into())));
        }
        match kind {
            "content_block_start" => {
                if !self.started
                    || self.end_turn
                    || self.block.is_some()
                    || value["index"].as_u64() != Some(self.next_index)
                    || self.next_index >= 64
                {
                    return Err(DirectError::Protocol);
                }
                let block = &value["content_block"];
                self.block = Some(match block["type"].as_str() {
                    Some("text") => Block::Text,
                    Some("thinking") => Block::Thinking,
                    Some("redacted_thinking") => Block::Redacted,
                    _ => return Err(DirectError::Unsupported),
                });
                if self.block == Some(Block::Text) {
                    return self.text(block["text"].as_str().ok_or(DirectError::Protocol)?);
                }
            }
            "content_block_delta" => {
                self.check_block(&value)?;
                match (self.block, value["delta"]["type"].as_str()) {
                    (Some(Block::Text), Some("text_delta")) => {
                        return self.text(
                            value["delta"]["text"]
                                .as_str()
                                .ok_or(DirectError::Protocol)?,
                        )
                    }
                    (Some(Block::Thinking), Some("thinking_delta"))
                        if value["delta"]["thinking"].is_string() => {}
                    (Some(Block::Thinking), Some("signature_delta"))
                        if value["delta"]["signature"].is_string() => {}
                    _ => return Err(DirectError::Protocol),
                }
            }
            "content_block_stop" => {
                self.check_block(&value)?;
                self.block = None;
                self.next_index += 1;
            }
            "message_delta" => {
                if !self.started || self.block.is_some() || self.end_turn {
                    return Err(DirectError::Protocol);
                }
                let delta = &value["delta"];
                if delta["stop_details"]["type"] == "refusal" {
                    return Err(DirectError::Refused);
                }
                match delta["stop_reason"].as_str() {
                    Some("end_turn") => self.end_turn = true,
                    Some("refusal") => return Err(DirectError::Refused),
                    Some("max_tokens" | "model_context_window_exceeded") => {
                        return Err(DirectError::Truncated)
                    }
                    Some("tool_use" | "pause_turn") => return Err(DirectError::Unsupported),
                    Some(_) => return Err(DirectError::Incomplete),
                    None if delta["stop_reason"].is_null() => {}
                    None => return Err(DirectError::Protocol),
                }
            }
            "message_stop" => {
                if !self.started || !self.end_turn || self.block.is_some() || !self.has_text {
                    return Err(DirectError::Incomplete);
                }
                self.complete = true;
                return Ok(Some(ProviderEvent::Completed));
            }
            _ => {} // Future event kinds cannot grant completion or tool authority.
        }
        Ok(None)
    }
    fn check_block(&self, value: &Value) -> Result<(), DirectError> {
        if !self.started
            || self.end_turn
            || self.block.is_none()
            || value["index"].as_u64() != Some(self.next_index)
        {
            Err(DirectError::Protocol)
        } else {
            Ok(())
        }
    }
    fn text(&mut self, text: &str) -> Result<Option<ProviderEvent>, DirectError> {
        self.text_bytes = self.text_bytes.saturating_add(text.len());
        self.text_characters = self.text_characters.saturating_add(text.chars().count());
        if self.text_bytes > 32_768 || self.text_characters > 8192 {
            return Err(DirectError::Limit);
        }
        self.has_text |= !text.trim().is_empty();
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ProviderEvent::Delta(text.into())))
        }
    }
}
fn stream_error(value: &Value) -> DirectError {
    match value["error"]["type"].as_str() {
        Some("authentication_error" | "permission_error") => DirectError::Authentication,
        Some("not_found_error") => DirectError::ModelUnavailable,
        Some("rate_limit_error") => DirectError::RateLimited,
        Some("overloaded_error") => DirectError::ProviderUnavailable,
        _ => DirectError::ProviderStreamErrorEvent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn frames(reason: &str) -> Vec<Value> {
        vec![
            json!({"type":"message_start","message":{"id":"msg_fixture","type":"message","role":"assistant","content":[],"stop_reason":null}}),
            json!({"type":"content_block_start","index":0,"content_block":{"type":"thinking","thinking":""}}),
            json!({"type":"content_block_delta","index":0,"delta":{"type":"thinking_delta","thinking":"private-thinking-sentinel"}}),
            json!({"type":"content_block_delta","index":0,"delta":{"type":"signature_delta","signature":"opaque-signature-sentinel"}}),
            json!({"type":"content_block_stop","index":0}),
            json!({"type":"content_block_start","index":1,"content_block":{"type":"text","text":""}}),
            json!({"type":"content_block_delta","index":1,"delta":{"type":"text_delta","text":"Board 🙂 ready."}}),
            json!({"type":"content_block_stop","index":1}),
            json!({"type":"message_delta","delta":{"stop_reason":reason}}),
            json!({"type":"message_stop"}),
        ]
    }
    fn wire(values: &[Value]) -> Vec<u8> {
        values
            .iter()
            .map(|value| {
                format!(
                    "event: {}\r\ndata: {value}\r\n\r\n",
                    value["type"].as_str().unwrap_or("fixture")
                )
            })
            .collect::<String>()
            .into_bytes()
    }
    fn page(id: &str, more: bool) -> Value {
        json!({"data":[{"id":id,"type":"model","display_name":"Claude fixture","max_input_tokens":200000,"max_tokens":4096,"capabilities":{"effort":{"supported":true,"high":{"supported":true},"max":{"supported":false}},"thinking":{"supported":true,"types":{"adaptive":{"supported":true}}},"future":{"supported":false}}}],"last_id":id,"has_more":more})
    }
    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Vec<ProviderEvent>, DirectError> {
        let mut events = Vec::new();
        decoder.push(bytes, |event| {
            events.push(event);
            Ok(())
        })?;
        Ok(events)
    }
    #[test]
    fn key_validation_and_request_headers_are_native_only_and_redacted(
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (development, enabled) in [(false, true), (true, false)] {
            assert!(matches!(
                AnthropicKey::from_values(development, enabled, Some("fixture-key".into())),
                Err(DirectError::Disabled)
            ));
        }
        for value in [
            None,
            Some(String::new()),
            Some("fixture\nkey".into()),
            Some("x".repeat(513)),
        ] {
            assert!(matches!(
                AnthropicKey::from_values(true, true, value),
                Err(DirectError::MissingKey)
            ));
        }
        let key =
            AnthropicKey::from_values(true, true, Some("synthetic-credential-sentinel".into()))?;
        assert_eq!(format!("{key:?}"), "AnthropicKey([REDACTED])");
        let request = request(&client()?, key, b"{}".to_vec())?;
        assert_eq!(request.url().as_str(), MESSAGES);
        assert_eq!(request.headers()["anthropic-version"], "2023-06-01");
        assert!(request.headers()["x-api-key"].is_sensitive());
        assert!(!request.headers().contains_key(header::AUTHORIZATION));
        assert!(!format!("{request:?}").contains("synthetic-credential-sentinel"));
        Ok(())
    }
    #[test]
    fn documented_catalog_does_not_claim_discovery_and_keeps_legacy_active() {
        let models = documented_models();
        for id in [
            "claude-fable-5-1",
            "claude-opus-5-5",
            "claude-sonnet-5",
            "claude-haiku-4-5-20251001",
            "claude-opus-5",
        ] {
            assert!(models.iter().any(|m| m.id == id
                && m.evidence == "documented"
                && m.availability == "access_unknown"));
        }
        assert!(models
            .iter()
            .filter(|m| matches!(m.id.as_str(), "claude-fable-5-1" | "claude-opus-5-5"))
            .all(|m| m.thinking == "always_on"));
        assert!(models
            .iter()
            .filter(|m| m.id == "claude-haiku-4-5-20251001")
            .all(|m| m.efforts == [ReasoningEffort::Default]));
        assert!(!models
            .iter()
            .any(|m| m.id.contains("mythos") || m.id.contains("preview")));
    }
    #[test]
    fn discovery_paginates_exact_ids_and_capabilities_without_inventing_support(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut catalog = Catalog::default();
        let first = page("claude-exact-fixture-1", true);
        assert_eq!(
            catalog.append(&serde_json::to_vec(&first)?)?,
            Some("claude-exact-fixture-1".into())
        );
        assert_eq!(
            catalog.append(&serde_json::to_vec(&page("claude-exact-fixture-2", false))?)?,
            None
        );
        assert_eq!(catalog.models.len(), 2);
        let model = &catalog.models[0];
        assert_eq!(model.id, "claude-exact-fixture-1");
        assert_eq!(
            model.capabilities.as_ref(),
            Some(&first["data"][0]["capabilities"])
        );
        assert_eq!(
            model.efforts,
            [ReasoningEffort::Default, ReasoningEffort::High]
        );
        assert_eq!(model.availability, "available");
        assert_eq!(model.evidence, "discovered");
        assert_eq!(model.context_limit, Some(200000));
        Ok(())
    }
    #[test]
    fn discovery_rejects_cycles_bad_cursors_unbounded_metadata_and_partial_pages(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let first = serde_json::to_vec(&page("fixture", true))?;
        let mut catalog = Catalog::default();
        catalog.append(&first)?;
        assert!(matches!(catalog.append(&first), Err(DirectError::Catalog)));
        let mut wrong = page("fixture", true);
        wrong["last_id"] = json!("other");
        let mut metadata = page("fixture", false);
        metadata["data"][0]["capabilities"]["oversized"] = json!("x".repeat(257));
        for value in [
            wrong,
            metadata,
            json!({"data":[],"has_more":true,"last_id":"fixture"}),
            json!({"data":[]}),
        ] {
            assert!(matches!(
                Catalog::default().append(&serde_json::to_vec(&value)?),
                Err(DirectError::Catalog)
            ));
        }
        let mut bound = Catalog::default();
        for index in 0..MAX_PAGES {
            bound.append(&serde_json::to_vec(&page(
                &format!("fixture-{index}"),
                true,
            ))?)?;
        }
        assert!(matches!(
            bound.append(&serde_json::to_vec(&page("overflow", false))?),
            Err(DirectError::Catalog)
        ));
        Ok(())
    }
    #[test]
    fn thinking_and_signature_never_become_answer_and_utf8_chunking_is_safe(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = wire(&frames("end_turn"));
        for chunk_size in [1, 2, 13, bytes.len()] {
            let mut decoder = Decoder::default();
            let mut text = String::new();
            let mut completed = 0;
            for chunk in bytes.chunks(chunk_size) {
                for event in decode(&mut decoder, chunk)? {
                    match event {
                        ProviderEvent::Delta(delta) => text.push_str(&delta),
                        ProviderEvent::Completed => completed += 1,
                        ProviderEvent::Started(_) => {}
                    }
                }
            }
            decoder.finish()?;
            assert_eq!(text, "Board 🙂 ready.");
            assert_eq!(completed, 1);
        }
        Ok(())
    }
    #[test]
    fn refusals_truncation_tools_and_missing_terminal_are_not_success() {
        for (reason, expected) in [
            ("refusal", DirectError::Refused),
            ("max_tokens", DirectError::Truncated),
            ("model_context_window_exceeded", DirectError::Truncated),
            ("tool_use", DirectError::Unsupported),
            ("pause_turn", DirectError::Unsupported),
            ("unknown", DirectError::Incomplete),
        ] {
            assert!(
                matches!(decode(&mut Decoder::default(), &wire(&frames(reason))), Err(error) if error == expected)
            );
        }
        let mut missing = frames("end_turn");
        missing.pop();
        let mut decoder = Decoder::default();
        assert!(decode(&mut decoder, &wire(&missing)).is_ok());
        assert_eq!(decoder.finish(), Err(DirectError::Incomplete));
        let mut no_reason = frames("end_turn");
        no_reason.remove(8);
        assert!(matches!(
            decode(&mut Decoder::default(), &wire(&no_reason)),
            Err(DirectError::Incomplete)
        ));
        let mut empty = frames("end_turn");
        empty[6]["delta"]["text"] = json!("");
        assert!(matches!(
            decode(&mut Decoder::default(), &wire(&empty)),
            Err(DirectError::Incomplete)
        ));
    }
    #[test]
    fn decoder_bounds_and_block_order_fail_closed_without_raw_provider_messages() {
        let mut wrong = frames("end_turn");
        wrong[6]["index"] = json!(0);
        assert!(matches!(
            decode(&mut Decoder::default(), &wire(&wrong)),
            Err(DirectError::Protocol)
        ));
        let mut tool = frames("end_turn");
        tool[1]["content_block"]["type"] = json!("tool_use");
        assert!(matches!(
            decode(&mut Decoder::default(), &wire(&tool)),
            Err(DirectError::Unsupported)
        ));
        let mut huge = frames("end_turn");
        huge[6]["delta"]["text"] = json!("x".repeat(8193));
        assert!(matches!(
            decode(&mut Decoder::default(), &wire(&huge)),
            Err(DirectError::Limit)
        ));
        for (kind, expected) in [
            ("authentication_error", DirectError::Authentication),
            ("rate_limit_error", DirectError::RateLimited),
            ("overloaded_error", DirectError::ProviderUnavailable),
            ("unrecognized", DirectError::ProviderStreamErrorEvent),
        ] {
            let error = json!({"type":"error","error":{"type":kind,"message":"sensitive-provider-body-sentinel"}});
            assert!(
                matches!(decode(&mut Decoder::default(), &wire(&[error])), Err(actual) if actual == expected)
            );
            assert!(
                !format!("{expected} {expected:?}").contains("sensitive-provider-body-sentinel")
            );
        }
        for (status, expected) in [
            (401, DirectError::Authentication),
            (403, DirectError::Authentication),
            (404, DirectError::ModelUnavailable),
            (429, DirectError::RateLimited),
            (302, DirectError::HttpStatus),
            (500, DirectError::HttpStatus),
        ] {
            assert_eq!(
                StatusCode::from_u16(status)
                    .map_err(|_| DirectError::Internal)
                    .and_then(status_error),
                Err(expected)
            );
        }
    }

    #[test]
    fn whitespace_only_output_is_not_a_completed_answer() {
        let mut values = frames("end_turn");
        values[6]["delta"]["text"] = json!(" \n\t\u{2003}");
        assert!(matches!(
            decode(&mut Decoder::default(), &wire(&values)),
            Err(DirectError::Incomplete)
        ));
    }

    #[test]
    fn same_chunk_failures_preserve_accepted_text_without_completion() {
        for (reason, expected) in [
            ("refusal", DirectError::Refused),
            ("max_tokens", DirectError::Truncated),
        ] {
            let mut decoder = Decoder::default();
            let mut text = String::new();
            let mut completed = false;
            let result = decoder.push(&wire(&frames(reason)), |event| {
                match event {
                    ProviderEvent::Delta(delta) => text.push_str(&delta),
                    ProviderEvent::Completed => completed = true,
                    ProviderEvent::Started(_) => {}
                }
                Ok(())
            });
            assert_eq!(result, Err(expected));
            assert_eq!(text, "Board 🙂 ready.");
            assert!(!completed);
        }
    }
}
