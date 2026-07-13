use crate::tools::types::ToolCallProposal;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentRequest {
    pub run_id: String,
    pub user_message: String,
    pub available_tools: Vec<String>,
}

impl AgentRequest {
    #[must_use]
    pub fn new(
        run_id: impl Into<String>,
        user_message: impl Into<String>,
        available_tools: Vec<String>,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            user_message: user_message.into(),
            available_tools,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentProviderResponse {
    AssistantText { content: String },
    ToolCalls { calls: Vec<ToolCallProposal> },
}

impl AgentProviderResponse {
    #[must_use]
    pub fn assistant_text(content: impl Into<String>) -> Self {
        Self::AssistantText {
            content: content.into(),
        }
    }

    #[must_use]
    pub fn tool_calls(calls: Vec<ToolCallProposal>) -> Self {
        Self::ToolCalls { calls }
    }
}
