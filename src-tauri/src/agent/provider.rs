use std::collections::BTreeMap;

use thiserror::Error;

use super::types::{AgentProviderResponse, AgentRequest};

pub type AgentProviderResult<T> = Result<T, AgentProviderError>;

pub trait AgentProvider {
    fn complete(&self, request: AgentRequest) -> AgentProviderResult<AgentProviderResponse>;
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AgentProviderError {
    #[error("agent request run id must not be empty")]
    EmptyRunId,
    #[error("agent request user message must not be empty")]
    EmptyUserMessage,
    #[error("mock provider failure: {0}")]
    MockFailure(String),
}

#[derive(Clone, Debug)]
pub struct MockAgentProvider {
    default_response: AgentProviderResponse,
    keyed_responses: BTreeMap<String, AgentProviderResponse>,
    forced_error: Option<String>,
}

impl MockAgentProvider {
    #[must_use]
    pub fn new(default_response: AgentProviderResponse) -> Self {
        Self {
            default_response,
            keyed_responses: BTreeMap::new(),
            forced_error: None,
        }
    }

    #[must_use]
    pub fn with_response(
        mut self,
        user_message: impl Into<String>,
        response: AgentProviderResponse,
    ) -> Self {
        self.keyed_responses.insert(user_message.into(), response);
        self
    }

    #[must_use]
    pub fn with_forced_error(mut self, message: impl Into<String>) -> Self {
        self.forced_error = Some(message.into());
        self
    }
}

impl Default for MockAgentProvider {
    fn default() -> Self {
        Self::new(AgentProviderResponse::assistant_text(
            "Mock assistant response from the local deterministic provider.",
        ))
    }
}

impl AgentProvider for MockAgentProvider {
    fn complete(&self, request: AgentRequest) -> AgentProviderResult<AgentProviderResponse> {
        if request.run_id.trim().is_empty() {
            return Err(AgentProviderError::EmptyRunId);
        }

        if request.user_message.trim().is_empty() {
            return Err(AgentProviderError::EmptyUserMessage);
        }

        if let Some(message) = &self.forced_error {
            return Err(AgentProviderError::MockFailure(message.clone()));
        }

        let response = match self.keyed_responses.get(&request.user_message) {
            Some(response) => response.clone(),
            None => self.default_response.clone(),
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::{AgentProvider, AgentProviderError, MockAgentProvider};
    use crate::agent::types::{AgentProviderResponse, AgentRequest};

    #[test]
    fn returns_default_response_deterministically() {
        let provider = MockAgentProvider::default();
        let request = AgentRequest::new("run-1", "Hello", Vec::new());

        assert_eq!(
            provider.complete(request),
            Ok(AgentProviderResponse::assistant_text(
                "Mock assistant response from the local deterministic provider."
            ))
        );
    }

    #[test]
    fn returns_keyed_response_for_matching_message() {
        let provider = MockAgentProvider::default().with_response(
            "Open the plan",
            AgentProviderResponse::assistant_text("Here is the plan."),
        );
        let request = AgentRequest::new("run-2", "Open the plan", Vec::new());

        assert_eq!(
            provider.complete(request),
            Ok(AgentProviderResponse::assistant_text("Here is the plan."))
        );
    }

    #[test]
    fn rejects_invalid_agent_requests() {
        let provider = MockAgentProvider::default();

        assert_eq!(
            provider.complete(AgentRequest::new("", "Hello", Vec::new())),
            Err(AgentProviderError::EmptyRunId)
        );
        assert_eq!(
            provider.complete(AgentRequest::new("run-3", " ", Vec::new())),
            Err(AgentProviderError::EmptyUserMessage)
        );
    }
}
