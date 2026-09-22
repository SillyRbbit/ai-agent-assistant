//! Editable, non-authorizing preferences for the immutable native agent catalog.
//!
//! Private notes are owner-managed content, never credentials or tool authority.
//! Their revision is captured by each conversation and invalidates stale context.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::agent::definition::{AgentDefinition, AgentId};

pub(crate) const MAX_OWNER_INSTRUCTION_CHARACTERS: usize = 4_096;
pub(crate) const MAX_PRIVATE_NOTE_CHARACTERS: usize = 8_192;
pub(crate) const OPENAI_AGENT_MODEL: &str = "gpt-5.6-luna";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AgentConnection {
    Simulation,
    OpenaiApi,
    AnthropicApi,
    LmStudio,
    Ollama,
    Codex,
}

impl AgentConnection {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Simulation => "simulation",
            Self::OpenaiApi => "openai_api",
            Self::AnthropicApi => "anthropic_api",
            Self::LmStudio => "lm_studio",
            Self::Ollama => "ollama",
            Self::Codex => "codex",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReasoningEffort {
    Default,
    None,
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl ReasoningEffort {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Xhigh => "xhigh",
            Self::Max => "max",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MemoryMode {
    Off,
    PrivateNotes,
}

impl MemoryMode {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::PrivateNotes => "private_notes",
        }
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AgentPreferencesInput {
    pub(crate) agent_id: String,
    pub(crate) connection: AgentConnection,
    pub(crate) model: String,
    pub(crate) endpoint: String,
    pub(crate) local_auth: bool,
    pub(crate) allow_unknown_locality_notes: bool,
    pub(crate) effort: ReasoningEffort,
    pub(crate) owner_instructions: String,
    pub(crate) memory_mode: MemoryMode,
    pub(crate) note: String,
    pub(crate) revision: u64,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentProfile {
    pub(crate) agent_id: String,
    pub(crate) display_name: String,
    pub(crate) connection: AgentConnection,
    pub(crate) model: String,
    pub(crate) endpoint: String,
    pub(crate) local_auth: bool,
    pub(crate) allow_unknown_locality_notes: bool,
    pub(crate) effort: ReasoningEffort,
    pub(crate) owner_instructions: String,
    pub(crate) memory_mode: MemoryMode,
    pub(crate) note: String,
    pub(crate) revision: u64,
}

impl AgentProfile {
    pub(crate) fn defaults(agent_id: AgentId) -> Result<Self, PreferencesError> {
        let definition =
            AgentDefinition::built_in(agent_id).map_err(|_| PreferencesError::InvalidRequest)?;
        Ok(Self {
            agent_id: agent_id.as_str().to_owned(),
            display_name: definition.display_name().to_owned(),
            connection: AgentConnection::Simulation,
            model: "simulation".to_owned(),
            endpoint: String::new(),
            local_auth: false,
            allow_unknown_locality_notes: false,
            effort: ReasoningEffort::Default,
            owner_instructions: String::new(),
            memory_mode: MemoryMode::Off,
            note: String::new(),
            revision: 0,
        })
    }

    pub(crate) fn validate(&self) -> Result<(), PreferencesError> {
        let id = validate_preferences(
            &self.agent_id,
            self.connection,
            &self.model,
            self.effort,
            &self.owner_instructions,
            &self.note,
            self.revision,
        )?;
        let expected = Self::defaults(id)?;
        if self.display_name != expected.display_name {
            return Err(PreferencesError::InvalidRequest);
        }
        if normalize_configuration(
            self.connection,
            &self.endpoint,
            self.local_auth,
            self.allow_unknown_locality_notes,
        )? != self.endpoint
        {
            return Err(PreferencesError::InvalidRequest);
        }
        Ok(())
    }
}

impl AgentPreferencesInput {
    pub(crate) fn into_profile(self) -> Result<AgentProfile, PreferencesError> {
        let id = validate_preferences(
            &self.agent_id,
            self.connection,
            &self.model,
            self.effort,
            &self.owner_instructions,
            &self.note,
            self.revision,
        )?;
        let mut profile = AgentProfile::defaults(id)?;
        profile.connection = self.connection;
        profile.model = self.model;
        profile.endpoint = normalize_configuration(
            self.connection,
            &self.endpoint,
            self.local_auth,
            self.allow_unknown_locality_notes,
        )?;
        profile.local_auth = self.local_auth;
        profile.allow_unknown_locality_notes = self.allow_unknown_locality_notes;
        profile.effort = self.effort;
        profile.owner_instructions = self.owner_instructions;
        profile.memory_mode = self.memory_mode;
        profile.note = self.note;
        profile.revision = self.revision;
        Ok(profile)
    }
}

impl fmt::Debug for AgentPreferencesInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Inputs have not been validated yet; even the identifier may be hostile.
        formatter.write_str("AgentPreferencesInput { content: [redacted] }")
    }
}

impl fmt::Debug for AgentProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AgentProfile { content: [redacted] }")
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PreferencesError {
    #[error("Agent preferences are invalid.")]
    InvalidRequest,
    #[error("This connection, model, and reasoning combination is unavailable.")]
    UnsupportedSettings,
    #[error("Agent settings or notes changed. Reload and start a new conversation.")]
    StaleContext,
    #[error("Private agent preferences could not be stored or loaded.")]
    Storage,
}

fn validate_preferences(
    agent_id: &str,
    connection: AgentConnection,
    model: &str,
    effort: ReasoningEffort,
    owner_instructions: &str,
    note: &str,
    revision: u64,
) -> Result<AgentId, PreferencesError> {
    let id = AgentId::from_str(agent_id).map_err(|_| PreferencesError::InvalidRequest)?;
    if revision > i64::MAX as u64
        || !valid_content(owner_instructions, MAX_OWNER_INSTRUCTION_CHARACTERS)
        || !valid_content(note, MAX_PRIVATE_NOTE_CHARACTERS)
    {
        return Err(PreferencesError::InvalidRequest);
    }
    let supported = match connection {
        AgentConnection::Simulation => model == "simulation" && effort == ReasoningEffort::Default,
        AgentConnection::OpenaiApi => {
            matches!(model, OPENAI_AGENT_MODEL | "gpt-5.6-terra" | "gpt-5.6-sol")
                && effort != ReasoningEffort::Max
        }
        AgentConnection::AnthropicApi => {
            if !valid_model(model, false) || effort == ReasoningEffort::None {
                false
            } else if let Some(documented) = crate::anthropic::documented_models()
                .iter()
                .find(|entry| entry.id == model)
            {
                crate::agent_models::validate_selection(documented, effort).is_ok()
            } else {
                // Discovery decides access/capability before Send. Saving an unknown
                // exact ID neither probes the provider nor claims it is supported.
                true
            }
        }
        AgentConnection::LmStudio | AgentConnection::Ollama => {
            valid_model(model, true) && effort == ReasoningEffort::Default
        }
        // The installed runtime cannot prove tool isolation. Persist this explicit
        // unavailable choice without inventing a model or authentication status.
        AgentConnection::Codex => model == "unavailable" && effort == ReasoningEffort::Default,
    };
    if !supported {
        return Err(PreferencesError::UnsupportedSettings);
    }
    Ok(id)
}

fn valid_model(model: &str, empty_allowed: bool) -> bool {
    (empty_allowed || !model.is_empty())
        && model.chars().count() <= 256
        && !model.chars().any(char::is_control)
        && (model.is_empty() || model.trim() == model)
}

fn normalize_configuration(
    connection: AgentConnection,
    endpoint: &str,
    local_auth: bool,
    allow_unknown_locality_notes: bool,
) -> Result<String, PreferencesError> {
    if matches!(
        connection,
        AgentConnection::LmStudio | AgentConnection::Ollama
    ) {
        crate::local_models::normalize_endpoint(endpoint)
            .map_err(|_| PreferencesError::InvalidRequest)
    } else if endpoint.is_empty() && !local_auth && !allow_unknown_locality_notes {
        Ok(String::new())
    } else {
        Err(PreferencesError::InvalidRequest)
    }
}

fn valid_content(value: &str, maximum: usize) -> bool {
    value.chars().count() <= maximum
        && !value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
}
