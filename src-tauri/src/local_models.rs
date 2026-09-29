//! Explicit discovery and text streaming through owner-configured loopback servers.
//! Loopback transport does not prove that inference remains on this device.
use crate::{
    agent_models::ModelInfo,
    agent_preferences::{AgentConnection, ReasoningEffort},
    personal_assistant_direct::{DirectError, ProviderEvent},
};
use reqwest::{header, Client, Request, Response, StatusCode, Url};
use serde_json::{json, Value};
use std::{collections::HashSet, fmt, net::IpAddr, time::Duration};

const MAX_BODY: usize = 65_536;
const MAX_WIRE: usize = 1_048_576;
const MAX_MODELS: usize = 200;
const MAX_SHOW: usize = MAX_MODELS;
const MAX_EVENTS: usize = 2_048;
const MAX_ANSWER: usize = 32_768;

/// Canonical endpoint identity binds optional local credentials. No DNS lookup.
pub(crate) fn normalize_endpoint(value: &str) -> Result<String, DirectError> {
    if value.len() > 512
        || value.bytes().any(|b| !b.is_ascii_graphic())
        || value.contains(['%', '\\', '@', '?', '#'])
    {
        return Err(DirectError::Endpoint);
    }
    let (scheme, remainder) = value.split_once("://").ok_or(DirectError::Endpoint)?;
    if !matches!(scheme, "http" | "https") {
        return Err(DirectError::Endpoint);
    }
    let (authority, path) = remainder.split_once('/').ok_or(DirectError::Endpoint)?;
    if !matches!(path, "v1" | "v1/") {
        return Err(DirectError::Endpoint);
    }
    let raw_host = if let Some(rest) = authority.strip_prefix('[') {
        rest.split_once(']').ok_or(DirectError::Endpoint)?.0
    } else {
        authority.split(':').next().ok_or(DirectError::Endpoint)?
    };
    let host = if raw_host.eq_ignore_ascii_case("localhost") {
        "127.0.0.1".to_owned()
    } else {
        let ip = raw_host
            .parse::<IpAddr>()
            .map_err(|_| DirectError::Endpoint)?;
        if !ip.is_loopback() {
            return Err(DirectError::Endpoint);
        }
        match ip {
            IpAddr::V4(ip) => ip.to_string(),
            IpAddr::V6(ip) => format!("[{ip}]"),
        }
    };
    let parsed = Url::parse(value).map_err(|_| DirectError::Endpoint)?;
    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || parsed.port() == Some(0)
    {
        return Err(DirectError::Endpoint);
    }
    let port = parsed.port().map(|p| format!(":{p}")).unwrap_or_default();
    Ok(format!("{scheme}://{host}{port}/v1"))
}

pub(crate) struct LocalKey {
    endpoint: String,
    header: header::HeaderValue,
}
impl fmt::Debug for LocalKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LocalKey([REDACTED])")
    }
}
impl LocalKey {
    pub(crate) fn from_environment(
        connection: AgentConnection,
        endpoint: &str,
        enabled: bool,
    ) -> Result<Option<Self>, DirectError> {
        let (name, binding) = match connection {
            AgentConnection::LmStudio => (
                "CORTEXA_LM_STUDIO_TOKEN",
                "CORTEXA_LM_STUDIO_TOKEN_ENDPOINT",
            ),
            AgentConnection::Ollama => ("CORTEXA_OLLAMA_TOKEN", "CORTEXA_OLLAMA_TOKEN_ENDPOINT"),
            _ => return Err(DirectError::Unsupported),
        };
        let endpoint = normalize_endpoint(endpoint)?;
        if !enabled {
            return Ok(None);
        }
        Self::from_values(
            &endpoint,
            std::env::var(name).ok(),
            std::env::var(binding).ok(),
        )
        .map(Some)
    }
    fn from_values(
        endpoint: &str,
        value: Option<String>,
        binding: Option<String>,
    ) -> Result<Self, DirectError> {
        let endpoint = normalize_endpoint(endpoint)?;
        if normalize_endpoint(&binding.ok_or(DirectError::MissingKey)?)? != endpoint {
            return Err(DirectError::Endpoint);
        }
        let value = value
            .filter(|v| !v.is_empty())
            .ok_or(DirectError::MissingKey)?;
        if value.len() > 512 || !value.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(DirectError::MissingKey);
        }
        let mut header = header::HeaderValue::from_str(&format!("Bearer {value}"))
            .map_err(|_| DirectError::MissingKey)?;
        header.set_sensitive(true);
        Ok(Self { endpoint, header })
    }
}

fn client() -> Result<Client, DirectError> {
    Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .connect_timeout(Duration::from_secs(5))
        .read_timeout(Duration::from_secs(60))
        .timeout(Duration::from_secs(120))
        .pool_max_idle_per_host(0)
        .build()
        .map_err(|_| DirectError::Internal)
}
fn network_error(error: reqwest::Error) -> DirectError {
    if error.is_timeout() {
        DirectError::Timeout
    } else {
        DirectError::ProviderUnavailable
    }
}
fn request(
    client: &Client,
    endpoint: &str,
    path: &str,
    key: Option<&LocalKey>,
    body: Option<Vec<u8>>,
) -> Result<Request, DirectError> {
    let endpoint = normalize_endpoint(endpoint)?;
    let origin = endpoint.strip_suffix("/v1").ok_or(DirectError::Endpoint)?;
    if !matches!(
        path,
        "/v1/models" | "/api/v1/models" | "/api/tags" | "/api/show" | "/v1/chat/completions"
    ) {
        return Err(DirectError::Endpoint);
    }
    let mut request = if let Some(body) = body {
        if body.is_empty() || body.len() > MAX_BODY {
            return Err(DirectError::Limit);
        }
        client.post(format!("{origin}{path}")).body(body)
    } else {
        client.get(format!("{origin}{path}"))
    };
    if let Some(key) = key {
        if key.endpoint != endpoint {
            return Err(DirectError::Endpoint);
        }
        request = request.header(header::AUTHORIZATION, key.header.clone());
    }
    request
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            header::ACCEPT,
            if path == "/v1/chat/completions" {
                "text/event-stream"
            } else {
                "application/json"
            },
        )
        .build()
        .map_err(|_| DirectError::InvalidRequest)
}
fn content_type(response: &Response, expected: &str) -> bool {
    response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.split(';').next())
        .is_some_and(|h| h.trim() == expected)
}
async fn bounded_body(mut response: Response, limit: usize) -> Result<Vec<u8>, DirectError> {
    if response.content_length().is_some_and(|n| n > limit as u64) {
        return Err(DirectError::Limit);
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(network_error)? {
        if bytes.len().saturating_add(chunk.len()) > limit {
            return Err(DirectError::Limit);
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
fn safe_error(value: &Value) -> DirectError {
    match value.pointer("/error/code").and_then(Value::as_str) {
        Some("out_of_memory" | "insufficient_memory") => DirectError::OutOfMemory,
        Some("model_not_found" | "model_not_loaded") => DirectError::ModelUnavailable,
        Some("rate_limit_exceeded") => DirectError::RateLimited,
        _ => DirectError::ProviderUnavailable,
    }
}
async fn status_error(response: Response) -> Result<Response, DirectError> {
    match response.status() {
        StatusCode::OK => return Ok(response),
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            return Err(DirectError::Authentication)
        }
        StatusCode::NOT_FOUND => return Err(DirectError::ModelUnavailable),
        StatusCode::TOO_MANY_REQUESTS => return Err(DirectError::RateLimited),
        status if status.is_redirection() => return Err(DirectError::Endpoint),
        _ => {}
    }
    // Error bodies never escape; only closed codes select static application errors.
    let bytes = bounded_body(response, 8_192).await.map_err(|error| {
        if error == DirectError::Limit {
            DirectError::ProviderUnavailable
        } else {
            error
        }
    })?;
    Err(serde_json::from_slice::<Value>(&bytes)
        .map(|v| safe_error(&v))
        .unwrap_or(DirectError::ProviderUnavailable))
}
async fn metadata(client: &Client, request: Request) -> Result<Value, DirectError> {
    let response = status_error(client.execute(request).await.map_err(network_error)?).await?;
    if !content_type(&response, "application/json") {
        return Err(DirectError::Catalog);
    }
    serde_json::from_slice(&bounded_body(response, MAX_WIRE).await?)
        .map_err(|_| DirectError::Catalog)
}
fn identifier(value: &Value) -> Option<&str> {
    value
        .as_str()
        .filter(|v| !v.is_empty() && v.len() <= 256 && !v.chars().any(char::is_control))
}
fn cloud_tag(id: &str) -> bool {
    let id = id.to_ascii_lowercase();
    id.ends_with(":cloud") || id.ends_with("-cloud")
}
fn primary_models(value: &Value) -> Result<Vec<ModelInfo>, DirectError> {
    let data = value
        .get("data")
        .and_then(Value::as_array)
        .ok_or(DirectError::Catalog)?;
    if data.len() > MAX_MODELS {
        return Err(DirectError::Limit);
    }
    let mut seen = HashSet::new();
    let mut result = Vec::with_capacity(data.len());
    for row in data {
        let id = identifier(&row["id"]).ok_or(DirectError::Catalog)?;
        if !seen.insert(id) {
            return Err(DirectError::Catalog);
        }
        let mut model = ModelInfo::unknown(id.to_owned());
        model.evidence = "discovered".into();
        model.availability = "unsupported".into();
        model.locality = if cloud_tag(id) { "cloud" } else { "unknown" }.into();
        model.efforts = vec![ReasoningEffort::Default];
        model.thinking = "unknown".into();
        result.push(model);
    }
    Ok(result)
}
fn short_text(value: &Value) -> Option<String> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
        .map(str::to_owned)
}
fn remote_metadata(row: &Value) -> bool {
    ["remote_host", "remote_model"].into_iter().any(|key| {
        row.get(key)
            .is_some_and(|v| !v.is_null() && v.as_str() != Some(""))
    })
}
fn lm_metadata(models: &mut [ModelInfo], value: &Value) -> Result<(), DirectError> {
    let rows = value
        .get("models")
        .and_then(Value::as_array)
        .ok_or(DirectError::Catalog)?;
    if rows.len() > MAX_MODELS {
        return Err(DirectError::Limit);
    }
    for model in models {
        let matches: Vec<_> = rows
            .iter()
            .filter(|row| {
                row["key"].as_str() == Some(&model.id)
                    || row["loaded_instances"].as_array().is_some_and(|instances| {
                        instances
                            .iter()
                            .any(|instance| instance["id"].as_str() == Some(&model.id))
                    })
            })
            .collect();
        if matches.len() > 1 {
            return Err(DirectError::Catalog);
        }
        let Some(row) = matches.first() else { continue };
        if row["type"].as_str() != Some("llm") {
            model.capabilities =
                Some(json!({"type": row["type"].as_str().filter(|t| *t == "embedding")}));
            continue;
        }
        model.availability = "available".into();
        model.label = short_text(&row["display_name"]).unwrap_or_else(|| model.id.clone());
        model.quantization = short_text(&row["quantization"]["name"]);
        model.size_bytes = row["size_bytes"].as_u64();
        model.context_limit = row["max_context_length"].as_u64();
        model.provenance = short_text(&row["publisher"]);
        model.loaded = row["loaded_instances"].as_array().map(|v| !v.is_empty());
        model.capabilities = Some(
            json!({"type":"llm", "vision":row["capabilities"]["vision"].as_bool(), "trained_for_tool_use":row["capabilities"]["trained_for_tool_use"].as_bool()}),
        );
        // LM Link may route a loaded instance elsewhere: locality remains unknown.
        if model.locality == "cloud" || remote_metadata(row) {
            model.locality = "cloud".into();
            model.availability = "unsupported".into();
        }
    }
    Ok(())
}
fn ollama_row(model: &mut ModelInfo, row: &Value) {
    if remote_metadata(row) || cloud_tag(&model.id) {
        model.locality = "cloud".into();
        model.availability = "unsupported".into();
        return;
    }
    let capabilities = row["capabilities"].as_array();
    let completion =
        capabilities.is_some_and(|v| v.iter().any(|c| c.as_str() == Some("completion")));
    model.availability = if completion {
        "available"
    } else {
        "unsupported"
    }
    .into();
    model.capabilities = capabilities.map(|values| {
        Value::Array(
            values
                .iter()
                .filter(|v| {
                    matches!(
                        v.as_str(),
                        Some(
                            "completion" | "embedding" | "vision" | "tools" | "thinking" | "image"
                        )
                    )
                })
                .cloned()
                .collect(),
        )
    });
    model.size_bytes = row["size"].as_u64().or(model.size_bytes);
    model.quantization =
        short_text(&row["details"]["quantization_level"]).or_else(|| model.quantization.clone());
    model.context_limit = row["details"]["context_length"]
        .as_u64()
        .or(model.context_limit);
    model.provenance = short_text(&row["license"]).or_else(|| model.provenance.clone());
}

/// Explicit bounded refresh. No generation, loading, download or service mutation.
pub(crate) async fn discover(
    connection: AgentConnection,
    endpoint: &str,
    local_auth: bool,
) -> Result<Vec<ModelInfo>, DirectError> {
    let endpoint = normalize_endpoint(endpoint)?;
    let key = LocalKey::from_environment(connection, &endpoint, local_auth)?;
    let client = client()?;
    tokio::time::timeout(Duration::from_secs(20), async {
        let primary = metadata(
            &client,
            request(&client, &endpoint, "/v1/models", key.as_ref(), None)?,
        )
        .await?;
        let mut models = primary_models(&primary)?;
        let path = match connection {
            AgentConnection::LmStudio => "/api/v1/models",
            AgentConnection::Ollama => "/api/tags",
            _ => return Err(DirectError::Unsupported),
        };
        let extra = match metadata(
            &client,
            request(&client, &endpoint, path, key.as_ref(), None)?,
        )
        .await
        {
            Ok(value) => value,
            Err(DirectError::ModelUnavailable) => return Ok(models), // optional endpoint absent on old server
            Err(error) => return Err(error),
        };
        if connection == AgentConnection::LmStudio {
            lm_metadata(&mut models, &extra)?;
        } else {
            let rows = extra["models"].as_array().ok_or(DirectError::Catalog)?;
            if rows.len() > MAX_MODELS {
                return Err(DirectError::Limit);
            }
            let mut shows = 0;
            for model in &mut models {
                let matches: Vec<_> = rows
                    .iter()
                    .filter(|row| {
                        row["model"].as_str().or_else(|| row["name"].as_str()) == Some(&model.id)
                    })
                    .collect();
                if matches.len() > 1 {
                    return Err(DirectError::Catalog);
                }
                let Some(row) = matches.first() else { continue };
                ollama_row(model, row);
                if model.locality != "cloud" && row["capabilities"].is_null() && shows < MAX_SHOW {
                    shows += 1;
                    let body = serde_json::to_vec(&json!({"model":model.id, "verbose":false}))
                        .map_err(|_| DirectError::Internal)?;
                    match metadata(
                        &client,
                        request(&client, &endpoint, "/api/show", key.as_ref(), Some(body))?,
                    )
                    .await
                    {
                        Ok(show) => ollama_row(model, &show),
                        Err(DirectError::ModelUnavailable) => {}
                        Err(error) => return Err(error),
                    }
                }
            }
        }
        Ok(models)
    })
    .await
    .map_err(|_| DirectError::Timeout)?
}

fn validate_body(body: &[u8]) -> Result<(), DirectError> {
    if body.is_empty() || body.len() > MAX_BODY {
        return Err(DirectError::Limit);
    }
    let value: Value = serde_json::from_slice(body).map_err(|_| DirectError::InvalidRequest)?;
    let object = value.as_object().ok_or(DirectError::InvalidRequest)?;
    if object
        .keys()
        .any(|key| !matches!(key.as_str(), "model" | "messages" | "stream" | "max_tokens"))
        || value["stream"] != true
        || !value["max_tokens"]
            .as_u64()
            .is_some_and(|n| (1..=2048).contains(&n))
        || identifier(&value["model"]).is_none()
        || identifier(&value["model"]).is_some_and(cloud_tag)
    {
        return Err(DirectError::InvalidRequest);
    }
    let messages = value["messages"]
        .as_array()
        .ok_or(DirectError::InvalidRequest)?;
    if messages.is_empty() || messages.len() > 12 {
        return Err(DirectError::Limit);
    }
    for message in messages {
        let object = message.as_object().ok_or(DirectError::InvalidRequest)?;
        if object.len() != 2
            || !matches!(
                message["role"].as_str(),
                Some("system" | "user" | "assistant")
            )
            || !message["content"].as_str().is_some_and(|s| {
                !s.is_empty() && !s.chars().any(|c| c.is_control() && c != '\n' && c != '\t')
            })
        {
            return Err(DirectError::InvalidRequest);
        }
    }
    Ok(())
}
pub(crate) async fn run(
    endpoint: &str,
    key: Option<LocalKey>,
    body: Vec<u8>,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    validate_body(&body)?;
    let client = client()?;
    let request = request(
        &client,
        endpoint,
        "/v1/chat/completions",
        key.as_ref(),
        Some(body),
    )?;
    let mut response = status_error(client.execute(request).await.map_err(network_error)?).await?;
    if !content_type(&response, "text/event-stream") {
        return Err(DirectError::Protocol);
    }
    emit(ProviderEvent::Started("resp_local".into()))?;
    let mut decoder = Decoder::default();
    while let Some(chunk) = response.chunk().await.map_err(network_error)? {
        decoder.push(&chunk, &mut emit)?;
        if decoder.complete {
            return Ok(());
        }
    }
    Err(DirectError::Incomplete)
}

#[derive(Default)]
struct Decoder {
    pending: Vec<u8>,
    wire: usize,
    events: usize,
    answer: usize,
    has_text: bool,
    stopped: bool,
    complete: bool,
}
impl Decoder {
    fn push(
        &mut self,
        chunk: &[u8],
        emit: &mut impl FnMut(ProviderEvent) -> Result<(), DirectError>,
    ) -> Result<(), DirectError> {
        self.wire = self.wire.saturating_add(chunk.len());
        if self.wire > MAX_WIRE {
            return Err(DirectError::Limit);
        }
        for byte in chunk {
            if self.complete {
                if byte.is_ascii_whitespace() {
                    continue;
                }
                return Err(DirectError::Protocol);
            }
            self.pending.push(*byte);
            if self.pending.len() > MAX_BODY {
                return Err(DirectError::Limit);
            }
            let delimiter = if self.pending.ends_with(b"\r\n\r\n") {
                4
            } else if self.pending.ends_with(b"\n\n") {
                2
            } else {
                continue;
            };
            let frame = std::mem::take(&mut self.pending);
            self.frame(&frame[..frame.len() - delimiter], emit)?;
        }
        Ok(())
    }
    fn frame(
        &mut self,
        frame: &[u8],
        emit: &mut impl FnMut(ProviderEvent) -> Result<(), DirectError>,
    ) -> Result<(), DirectError> {
        self.events += 1;
        if self.events > MAX_EVENTS {
            return Err(DirectError::Limit);
        }
        let text = std::str::from_utf8(frame).map_err(|_| DirectError::Protocol)?;
        let mut data = String::new();
        for line in text.lines() {
            if let Some(value) = line.strip_prefix("data:") {
                if !data.is_empty() {
                    data.push('\n');
                }
                data.push_str(value.strip_prefix(' ').unwrap_or(value));
            }
        }
        if data.is_empty() {
            return Ok(());
        }
        if data == "[DONE]" {
            if !self.stopped || !self.has_text {
                return Err(DirectError::Incomplete);
            }
            self.complete = true;
            return emit(ProviderEvent::Completed);
        }
        let value: Value = serde_json::from_str(&data).map_err(|_| DirectError::Protocol)?;
        if value.get("error").is_some_and(|error| !error.is_null()) {
            return Err(safe_error(&value));
        }
        let choices = value["choices"].as_array().ok_or(DirectError::Protocol)?;
        if choices.is_empty() && value["usage"].is_object() {
            return Ok(());
        }
        if self.stopped || choices.len() != 1 || choices[0]["index"].as_u64() != Some(0) {
            return Err(DirectError::Protocol);
        }
        let choice = &choices[0];
        let delta = choice["delta"].as_object().ok_or(DirectError::Protocol)?;
        if delta
            .get("tool_calls")
            .is_some_and(|v| !v.is_null() && v.as_array().is_none_or(|v| !v.is_empty()))
            || delta.get("function_call").is_some_and(|v| !v.is_null())
        {
            return Err(DirectError::Unsupported);
        }
        if delta
            .get("refusal")
            .is_some_and(|v| !v.is_null() && v.as_str() != Some(""))
        {
            return Err(DirectError::Refused);
        }
        if let Some(content) = delta.get("content").filter(|v| !v.is_null()) {
            let content = content.as_str().ok_or(DirectError::Protocol)?;
            self.answer = self.answer.saturating_add(content.len());
            self.has_text |= !content.trim().is_empty();
            if self.answer > MAX_ANSWER {
                return Err(DirectError::Limit);
            }
            if !content.is_empty() {
                emit(ProviderEvent::Delta(content.into()))?;
            }
        }
        match choice["finish_reason"].as_str() {
            None if choice["finish_reason"].is_null() => {}
            Some("") => {} // old Ollama nonterminal chunks
            Some("stop") => self.stopped = true,
            Some("length") => return Err(DirectError::Truncated),
            Some("content_filter") => return Err(DirectError::Refused),
            Some("tool_calls" | "function_call") => return Err(DirectError::Unsupported),
            _ => return Err(DirectError::Protocol),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn event(value: Value) -> String {
        format!("data: {value}\n\n")
    }
    fn chunk(text: Value, finish: Value) -> String {
        event(json!({"choices":[{"index":0,"delta":{"content":text},"finish_reason":finish}]}))
    }
    fn body() -> Vec<u8> {
        br#"{"model":"owner/model:Q4","messages":[{"role":"user","content":"Synthetic hello"}],"stream":true,"max_tokens":512}"#.to_vec()
    }
    #[test]
    fn endpoint_normalization_rejects_nonloopback_and_url_authority_tricks(
    ) -> Result<(), DirectError> {
        for (value, expected) in [
            ("http://localhost:1234/v1/", "http://127.0.0.1:1234/v1"),
            ("http://127.0.0.2:11434/v1", "http://127.0.0.2:11434/v1"),
            ("https://[::1]:1234/v1", "https://[::1]:1234/v1"),
        ] {
            assert_eq!(normalize_endpoint(value)?, expected);
        }
        for value in [
            "http://127.1:1234/v1",
            "http://2130706433/v1",
            "http://0x7f000001/v1",
            "http://0.0.0.0/v1",
            "http://192.168.1.2/v1",
            "https://example.com/v1",
            "http://localhost.example/v1",
            "http://localhost:0/v1",
            "http://user@localhost/v1",
            "http://localhost/v1?key=x",
            "http://localhost/v1#x",
            "http://localhost/%76%31",
            "http://localhost/../v1",
            "http://localhost\\@example.com/v1",
            "file:///v1",
            "http://[::ffff:127.0.0.1]/v1",
            " http://localhost/v1",
            "http://localhost/v1\n",
        ] {
            assert_eq!(
                normalize_endpoint(value),
                Err(DirectError::Endpoint),
                "{value}"
            );
        }
        Ok(())
    }
    #[test]
    fn optional_tokens_are_redacted_sensitive_and_bound_to_exact_endpoint(
    ) -> Result<(), DirectError> {
        let endpoint = "http://127.0.0.1:1234/v1";
        let key = LocalKey::from_values(
            endpoint,
            Some("synthetic-local-token".into()),
            Some(endpoint.into()),
        )?;
        assert_eq!(format!("{key:?}"), "LocalKey([REDACTED])");
        let client = client()?;
        let built = request(&client, endpoint, "/v1/models", Some(&key), None)?;
        assert!(built.headers()[header::AUTHORIZATION].is_sensitive());
        assert!(LocalKey::from_values(
            endpoint,
            Some("synthetic-local-token".into()),
            Some("http://127.0.0.1:11434/v1".into())
        )
        .is_err());
        assert!(request(
            &client,
            "http://127.0.0.1:11434/v1",
            "/v1/models",
            Some(&key),
            None
        )
        .is_err());
        assert!(request(&client, endpoint, "/api/pull", None, Some(body())).is_err());
        assert!(
            LocalKey::from_values(endpoint, Some("bad\nvalue".into()), Some(endpoint.into()))
                .is_err()
        );
        assert!(request(&client, endpoint, "/v1/models", None, None)?
            .headers()
            .get(header::AUTHORIZATION)
            .is_none());
        assert!(LocalKey::from_environment(AgentConnection::LmStudio, endpoint, false)?.is_none());
        assert!(LocalKey::from_environment(AgentConnection::Ollama, endpoint, false)?.is_none());
        assert!(matches!(
            LocalKey::from_environment(AgentConnection::OpenaiApi, endpoint, false),
            Err(DirectError::Unsupported)
        ));
        Ok(())
    }
    #[test]
    fn lm_discovery_keeps_ids_and_excludes_embedding_unknown_and_remote_models(
    ) -> Result<(), DirectError> {
        let mut models = primary_models(
            &json!({"data":[{"id":"owner/llm"},{"id":"instance-one"},{"id":"embed"},{"id":"unknown"},{"id":"remote"}]}),
        )?;
        lm_metadata(
            &mut models,
            &json!({"models":[{"key":"owner/llm","type":"llm","publisher":"owner","size_bytes":123,"quantization":{"name":"Q4"},"loaded_instances":[],"max_context_length":4096},{"key":"other-key","type":"llm","loaded_instances":[{"id":"instance-one"}]},{"key":"embed","type":"embedding"},{"key":"remote","type":"llm","remote_host":"remote.invalid"}]}),
        )?;
        assert_eq!(models[0].id, "owner/llm");
        assert_eq!(models[0].availability, "available");
        assert_eq!(models[0].locality, "unknown");
        assert_eq!(models[0].loaded, Some(false));
        assert_eq!(models[0].size_bytes, Some(123));
        assert_eq!(models[0].quantization.as_deref(), Some("Q4"));
        assert_eq!(models[1].loaded, Some(true));
        assert_eq!(models[2].availability, "unsupported");
        assert_eq!(models[3].availability, "unsupported");
        assert_eq!(models[4].locality, "cloud");
        assert_eq!(models[4].availability, "unsupported");
        assert_eq!(models[0].efforts, vec![ReasoningEffort::Default]);
        Ok(())
    }
    #[test]
    fn ollama_capabilities_are_positive_and_cloud_aliases_cannot_hide_remote_metadata(
    ) -> Result<(), DirectError> {
        for (id, row, availability, locality) in [
            (
                "local:Q4",
                json!({"capabilities":["completion","tools"],"size":321,"details":{"quantization_level":"Q4_K_M"}}),
                "available",
                "unknown",
            ),
            (
                "embed",
                json!({"capabilities":["embedding"]}),
                "unsupported",
                "unknown",
            ),
            ("legacy", json!({}), "unsupported", "unknown"),
            (
                "renamed-local",
                json!({"capabilities":["completion"],"remote_model":"hidden-upstream"}),
                "unsupported",
                "cloud",
            ),
            (
                "renamed-local",
                json!({"capabilities":["completion"],"remote_host":"https://ollama.com"}),
                "unsupported",
                "cloud",
            ),
            (
                "name:cloud",
                json!({"capabilities":["completion"]}),
                "unsupported",
                "cloud",
            ),
            (
                "name:120b-cloud",
                json!({"capabilities":["completion"]}),
                "unsupported",
                "cloud",
            ),
        ] {
            let mut models = primary_models(&json!({"data":[{"id":id}]}))?;
            ollama_row(&mut models[0], &row);
            assert_eq!(models[0].availability, availability);
            assert_eq!(models[0].locality, locality);
        }
        Ok(())
    }
    #[test]
    fn discovery_is_bounded_and_duplicate_ids_are_rejected() {
        assert!(matches!(
            primary_models(&json!({"data":[{"id":"duplicate"},{"id":"duplicate"}]})),
            Err(DirectError::Catalog)
        ));
        assert!(matches!(
            primary_models(&json!({"data":[{"id":"bad\nname"}]})),
            Err(DirectError::Catalog)
        ));
        assert!(matches!(
            primary_models(
                &json!({"data":(0..201).map(|i| json!({"id":format!("model-{i}")})).collect::<Vec<_>>()})
            ),
            Err(DirectError::Limit)
        ));
    }
    #[test]
    fn local_request_has_no_state_tools_reasoning_or_credential_channels() -> Result<(), DirectError>
    {
        validate_body(&body())?;
        for (key, value) in [
            ("tools", json!([])),
            ("previous_response_id", json!("x")),
            ("reasoning", json!({"effort":"high"})),
            ("authorization", json!("synthetic")),
        ] {
            let mut parsed: Value =
                serde_json::from_slice(&body()).map_err(|_| DirectError::Internal)?;
            parsed[key] = value;
            assert_eq!(
                validate_body(&serde_json::to_vec(&parsed).map_err(|_| DirectError::Internal)?),
                Err(DirectError::InvalidRequest)
            );
        }
        let mut parsed: Value =
            serde_json::from_slice(&body()).map_err(|_| DirectError::Internal)?;
        parsed["model"] = json!("gpt-oss:cloud");
        assert_eq!(
            validate_body(&serde_json::to_vec(&parsed).map_err(|_| DirectError::Internal)?),
            Err(DirectError::InvalidRequest)
        );
        Ok(())
    }
    #[test]
    fn local_stream_handles_fragmentation_unicode_usage_and_hides_reasoning(
    ) -> Result<(), DirectError> {
        let stream = format!(
            "{}{}{}{}data: [DONE]\n\n",
            event(
                json!({"choices":[{"index":0,"delta":{"reasoning":"do not display"},"finish_reason":null}]})
            ),
            chunk(json!("Hello 🌱"), Value::Null),
            chunk(json!(""), json!("stop")),
            event(json!({"choices":[],"usage":{"completion_tokens":3}}))
        );
        for split in 0..=stream.len() {
            let mut decoder = Decoder::default();
            let mut output = String::new();
            let mut completed = 0;
            let mut emit = |event| {
                match event {
                    ProviderEvent::Delta(text) => output.push_str(&text),
                    ProviderEvent::Completed => completed += 1,
                    ProviderEvent::Started(_) => {}
                };
                Ok(())
            };
            decoder.push(&stream.as_bytes()[..split], &mut emit)?;
            decoder.push(&stream.as_bytes()[split..], &mut emit)?;
            assert_eq!(output, "Hello 🌱");
            assert_eq!(completed, 1);
        }
        Ok(())
    }
    #[test]
    fn local_stream_requires_explicit_complete_nonempty_answer() -> Result<(), DirectError> {
        for stream in [
            "data: [DONE]\n\n".into(),
            format!("{}data: [DONE]\n\n", chunk(json!(""), json!("stop"))),
            format!("{}data: [DONE]\n\n", chunk(json!(" \n\t"), json!("stop"))),
        ] {
            assert_eq!(
                Decoder::default().push(stream.as_bytes(), &mut |_| Ok(())),
                Err(DirectError::Incomplete)
            );
        }
        let mut decoder = Decoder::default();
        decoder.push(chunk(json!("partial"), Value::Null).as_bytes(), &mut |_| {
            Ok(())
        })?;
        assert!(!decoder.complete);
        decoder.push(chunk(json!(""), json!("stop")).as_bytes(), &mut |_| Ok(()))?;
        assert!(!decoder.complete);
        Ok(())
    }

    #[test]
    fn terminal_marker_allows_only_trailing_ascii_whitespace() -> Result<(), DirectError> {
        let stream = format!(
            "{}data: [DONE]\n\n \r\n\t",
            chunk(json!("answer"), json!("stop"))
        );
        let mut decoder = Decoder::default();
        decoder.push(stream.as_bytes(), &mut |_| Ok(()))?;
        assert!(decoder.complete);
        assert_eq!(
            decoder.push(b"data: other\n\n", &mut |_| Ok(())),
            Err(DirectError::Protocol)
        );
        Ok(())
    }
    #[test]
    fn local_stream_failures_are_distinct_static_and_keep_prior_answer_text() {
        for (finish, error) in [
            ("length", DirectError::Truncated),
            ("content_filter", DirectError::Refused),
            ("tool_calls", DirectError::Unsupported),
            ("unexpected", DirectError::Protocol),
        ] {
            let stream = format!(
                "{}{}",
                chunk(json!("partial"), Value::Null),
                chunk(json!(""), json!(finish))
            );
            let mut output = String::new();
            assert_eq!(
                Decoder::default().push(stream.as_bytes(), &mut |event| {
                    if let ProviderEvent::Delta(text) = event {
                        output.push_str(&text);
                    }
                    Ok(())
                }),
                Err(error)
            );
            assert_eq!(output, "partial");
        }
        for (code, expected) in [
            ("out_of_memory", DirectError::OutOfMemory),
            ("model_not_loaded", DirectError::ModelUnavailable),
            ("model_not_found", DirectError::ModelUnavailable),
            (
                "unknown-synthetic-sentinel",
                DirectError::ProviderUnavailable,
            ),
        ] {
            let value = json!({"error":{"code":code,"message":"never expose this synthetic body"}});
            assert_eq!(
                Decoder::default()
                    .push(event(value).as_bytes(), &mut |_| Ok(()))
                    .err(),
                Some(expected)
            );
            assert!(!expected.to_string().contains("synthetic"));
        }
    }
    #[test]
    fn local_stream_bounds_protocol_and_cancel_propagation() {
        assert_eq!(
            Decoder::default().push(&vec![b'x'; MAX_BODY + 1], &mut |_| Ok(())),
            Err(DirectError::Limit)
        );
        assert_eq!(
            Decoder::default().push(chunk(json!("x"), Value::Null).as_bytes(), &mut |_| Err(
                DirectError::Busy
            )),
            Err(DirectError::Busy)
        );
        for value in [
            json!({"choices":[{"index":1,"delta":{},"finish_reason":null}]}),
            json!({"choices":[{"index":0,"delta":{"content":[]},"finish_reason":null}]}),
            json!({"choices":[]}),
        ] {
            assert_eq!(
                Decoder::default().push(event(value).as_bytes(), &mut |_| Ok(())),
                Err(DirectError::Protocol)
            );
        }
        let mut decoder = Decoder {
            wire: MAX_WIRE,
            ..Decoder::default()
        };
        assert_eq!(decoder.push(b"x", &mut |_| Ok(())), Err(DirectError::Limit));
    }
}
