use std::collections::BTreeMap;

use thiserror::Error;

use super::types::{CapabilityReport, CapabilityStatus, PlatformCapability, PlatformMetadata};

pub type PlatformResult<T> = Result<T, PlatformError>;

pub trait PlatformAdapter {
    fn metadata(&self) -> PlatformResult<PlatformMetadata>;
    fn capability_status(&self, capability: PlatformCapability)
        -> PlatformResult<CapabilityStatus>;
    fn list_capabilities(&self) -> Vec<CapabilityReport>;
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PlatformError {
    #[error("platform metadata must include an operating system")]
    EmptyOperatingSystem,
    #[error("platform metadata must include an architecture")]
    EmptyArchitecture,
    #[error("platform capability is not modeled by this adapter")]
    UnknownCapability,
}

#[derive(Clone, Debug)]
pub struct MockPlatformAdapter {
    metadata: PlatformMetadata,
    capabilities: BTreeMap<PlatformCapability, CapabilityStatus>,
}

impl MockPlatformAdapter {
    #[must_use]
    pub fn new(metadata: PlatformMetadata) -> Self {
        Self {
            metadata,
            capabilities: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_capability(
        mut self,
        capability: PlatformCapability,
        status: CapabilityStatus,
    ) -> Self {
        self.capabilities.insert(capability, status);
        self
    }
}

impl Default for MockPlatformAdapter {
    fn default() -> Self {
        Self::new(PlatformMetadata::new("mock-os", "mock-arch", "mock-family"))
            .with_capability(PlatformCapability::Calendar, CapabilityStatus::Disabled)
            .with_capability(PlatformCapability::Files, CapabilityStatus::Disabled)
    }
}

impl PlatformAdapter for MockPlatformAdapter {
    fn metadata(&self) -> PlatformResult<PlatformMetadata> {
        if self.metadata.operating_system.trim().is_empty() {
            return Err(PlatformError::EmptyOperatingSystem);
        }

        if self.metadata.architecture.trim().is_empty() {
            return Err(PlatformError::EmptyArchitecture);
        }

        Ok(self.metadata.clone())
    }

    fn capability_status(
        &self,
        capability: PlatformCapability,
    ) -> PlatformResult<CapabilityStatus> {
        self.capabilities
            .get(&capability)
            .copied()
            .ok_or(PlatformError::UnknownCapability)
    }

    fn list_capabilities(&self) -> Vec<CapabilityReport> {
        self.capabilities
            .iter()
            .map(|(capability, status)| CapabilityReport::new(*capability, *status))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{MockPlatformAdapter, PlatformAdapter, PlatformError};
    use crate::platform::types::{CapabilityStatus, PlatformCapability, PlatformMetadata};

    #[test]
    fn returns_platform_neutral_metadata() {
        let adapter =
            MockPlatformAdapter::new(PlatformMetadata::new("test-os", "test-arch", "test-family"));

        assert_eq!(
            adapter.metadata(),
            Ok(PlatformMetadata::new("test-os", "test-arch", "test-family"))
        );
    }

    #[test]
    fn lists_capabilities_in_deterministic_order() {
        let adapter =
            MockPlatformAdapter::new(PlatformMetadata::new("test-os", "test-arch", "test-family"))
                .with_capability(PlatformCapability::Files, CapabilityStatus::Disabled)
                .with_capability(PlatformCapability::Calendar, CapabilityStatus::Unavailable);

        let capabilities: Vec<PlatformCapability> = adapter
            .list_capabilities()
            .into_iter()
            .map(|report| report.capability)
            .collect();

        assert_eq!(
            capabilities,
            vec![PlatformCapability::Calendar, PlatformCapability::Files]
        );
    }

    #[test]
    fn rejects_unmodeled_capabilities() {
        let adapter = MockPlatformAdapter::default();

        assert_eq!(
            adapter.capability_status(PlatformCapability::Automation),
            Err(PlatformError::UnknownCapability)
        );
    }
}
