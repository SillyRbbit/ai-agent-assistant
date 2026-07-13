#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformMetadata {
    pub operating_system: String,
    pub architecture: String,
    pub family: String,
}

impl PlatformMetadata {
    #[must_use]
    pub fn new(
        operating_system: impl Into<String>,
        architecture: impl Into<String>,
        family: impl Into<String>,
    ) -> Self {
        Self {
            operating_system: operating_system.into(),
            architecture: architecture.into(),
            family: family.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlatformCapability {
    Calendar,
    Reminders,
    Contacts,
    Notifications,
    Files,
    Accessibility,
    ScreenRecording,
    Automation,
    Microphone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityStatus {
    Unavailable,
    Disabled,
    Available,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityReport {
    pub capability: PlatformCapability,
    pub status: CapabilityStatus,
}

impl CapabilityReport {
    #[must_use]
    pub fn new(capability: PlatformCapability, status: CapabilityStatus) -> Self {
        Self { capability, status }
    }
}
