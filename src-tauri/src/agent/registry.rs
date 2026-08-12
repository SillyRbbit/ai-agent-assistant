//! Deterministic immutable registry for application-owned agent definitions.
//!
//! Registry membership and catalog activation are descriptive only. This
//! module has no start, selection, routing, gate-evaluation, or authorization
//! operation.

use std::{collections::BTreeMap, fmt};

use thiserror::Error;

use super::definition::{AgentDefinition, AgentDefinitionError, AgentId};

pub type AgentRegistryResult<T> = Result<T, AgentRegistryError>;

#[derive(Eq, PartialEq)]
pub struct AgentRegistry {
    definitions: BTreeMap<AgentId, AgentDefinition>,
}

impl AgentRegistry {
    pub fn built_in() -> AgentRegistryResult<Self> {
        let mut definitions = Vec::with_capacity(AgentId::ALL.len());
        for agent_id in AgentId::ALL {
            definitions.push(AgentDefinition::built_in(agent_id)?);
        }
        Self::from_definitions(definitions)
    }

    pub(crate) fn from_definitions(
        definitions: impl IntoIterator<Item = AgentDefinition>,
    ) -> AgentRegistryResult<Self> {
        let mut registered = BTreeMap::new();
        for definition in definitions {
            let agent_id = definition.id();
            if registered.contains_key(&agent_id) {
                return Err(AgentRegistryError::DuplicateAgent { agent_id });
            }
            registered.insert(agent_id, definition);
        }
        Ok(Self {
            definitions: registered,
        })
    }

    pub fn get(&self, agent_id: AgentId) -> AgentRegistryResult<&AgentDefinition> {
        self.definitions
            .get(&agent_id)
            .ok_or(AgentRegistryError::UnknownAgent { agent_id })
    }

    pub fn list(&self) -> impl ExactSizeIterator<Item = &AgentDefinition> {
        self.definitions.values()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

impl fmt::Debug for AgentRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentRegistry")
            .field("agent_ids", &self.definitions.keys().collect::<Vec<_>>())
            .field("definition_count", &self.definitions.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum AgentRegistryError {
    #[error(transparent)]
    InvalidDefinition(#[from] AgentDefinitionError),
    #[error("agent is already registered: {agent_id}")]
    DuplicateAgent { agent_id: AgentId },
    #[error("agent is not registered: {agent_id}")]
    UnknownAgent { agent_id: AgentId },
}

#[cfg(test)]
mod tests {
    use super::{AgentRegistry, AgentRegistryError};
    use crate::agent::definition::{AgentDefinition, AgentId};

    #[test]
    fn rejects_duplicates_before_replacing_the_first_definition() -> Result<(), AgentRegistryError>
    {
        let first = AgentDefinition::built_in(AgentId::Research)?;
        let duplicate = first.clone();

        assert_eq!(
            AgentRegistry::from_definitions([first, duplicate]),
            Err(AgentRegistryError::DuplicateAgent {
                agent_id: AgentId::Research,
            })
        );

        Ok(())
    }

    #[test]
    fn partial_registry_returns_typed_missing_agent_error() -> Result<(), AgentRegistryError> {
        let research = AgentDefinition::built_in(AgentId::Research)?;
        let registry = AgentRegistry::from_definitions([research])?;

        assert_eq!(
            registry.get(AgentId::PersonalAssistant),
            Err(AgentRegistryError::UnknownAgent {
                agent_id: AgentId::PersonalAssistant,
            })
        );
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());

        Ok(())
    }

    #[test]
    fn empty_test_registry_is_valid_and_deterministic() -> Result<(), AgentRegistryError> {
        let registry = AgentRegistry::from_definitions([])?;

        assert!(registry.is_empty());
        assert_eq!(registry.list().count(), 0);
        assert_eq!(
            format!("{registry:?}"),
            "AgentRegistry { agent_ids: [], definition_count: 0 }"
        );

        Ok(())
    }
}
