use std::collections::BTreeMap;

use thiserror::Error;

use super::types::ToolDefinition;

pub type ToolRegistryResult<T> = Result<T, ToolRegistryError>;

pub trait ToolRegistry {
    fn register(&mut self, definition: ToolDefinition) -> ToolRegistryResult<()>;
    fn get(&self, name: &str) -> ToolRegistryResult<ToolDefinition>;
    fn list(&self) -> Vec<ToolDefinition>;
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ToolRegistryError {
    #[error("tool name must not be empty")]
    EmptyName,
    #[error("tool description must not be empty")]
    EmptyDescription,
    #[error("tool already registered: {0}")]
    DuplicateTool(String),
    #[error("unknown tool: {0}")]
    UnknownTool(String),
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryToolRegistry {
    definitions: BTreeMap<String, ToolDefinition>,
}

impl InMemoryToolRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToolRegistry for InMemoryToolRegistry {
    fn register(&mut self, definition: ToolDefinition) -> ToolRegistryResult<()> {
        let name = definition.name.trim().to_owned();
        if name.is_empty() {
            return Err(ToolRegistryError::EmptyName);
        }

        if definition.description.trim().is_empty() {
            return Err(ToolRegistryError::EmptyDescription);
        }

        if self.definitions.contains_key(&name) {
            return Err(ToolRegistryError::DuplicateTool(name));
        }

        self.definitions.insert(name, definition);
        Ok(())
    }

    fn get(&self, name: &str) -> ToolRegistryResult<ToolDefinition> {
        self.definitions
            .get(name)
            .cloned()
            .ok_or_else(|| ToolRegistryError::UnknownTool(name.to_owned()))
    }

    fn list(&self) -> Vec<ToolDefinition> {
        self.definitions.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{InMemoryToolRegistry, ToolRegistry, ToolRegistryError};
    use crate::tools::types::{PermissionKind, RiskClass, ToolDefinition, ToolSchema};

    fn example_tool(name: &str) -> ToolDefinition {
        ToolDefinition::new(
            name,
            "example deterministic tool",
            RiskClass::InformationOnly,
            PermissionKind::None,
            ToolSchema::placeholder(1),
        )
    }

    #[test]
    fn registers_and_gets_a_tool() {
        let mut registry = InMemoryToolRegistry::new();
        let result = registry.register(example_tool("get_current_datetime"));

        assert_eq!(result, Ok(()));
        assert_eq!(
            registry.get("get_current_datetime"),
            Ok(example_tool("get_current_datetime"))
        );
    }

    #[test]
    fn rejects_unknown_tools() {
        let registry = InMemoryToolRegistry::new();

        assert_eq!(
            registry.get("run_shell"),
            Err(ToolRegistryError::UnknownTool("run_shell".to_owned()))
        );
    }

    #[test]
    fn lists_tools_in_deterministic_name_order() {
        let mut registry = InMemoryToolRegistry::new();

        assert_eq!(registry.register(example_tool("zeta")), Ok(()));
        assert_eq!(registry.register(example_tool("alpha")), Ok(()));

        let names: Vec<String> = registry.list().into_iter().map(|tool| tool.name).collect();

        assert_eq!(names, vec!["alpha".to_owned(), "zeta".to_owned()]);
    }
}
