#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RiskClass {
    InformationOnly,
    ReadOnlyDeviceAccess,
    ReversibleLocalAction,
    PersonalDataModification,
    ExternalOrHighImpactAction,
    ProhibitedAutonomy,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PermissionKind {
    None,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolSchema {
    pub version: u16,
    pub document: String,
}

impl ToolSchema {
    #[must_use]
    pub fn placeholder(version: u16) -> Self {
        Self {
            version,
            document: "{}".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub risk_class: RiskClass,
    pub required_permission: PermissionKind,
    pub schema: ToolSchema,
}

impl ToolDefinition {
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        risk_class: RiskClass,
        required_permission: PermissionKind,
        schema: ToolSchema,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            risk_class,
            required_permission,
            schema,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolCallProposal {
    pub tool_name: String,
    pub arguments_json: String,
    pub risk_class: RiskClass,
    pub required_permission: PermissionKind,
}

impl ToolCallProposal {
    #[must_use]
    pub fn new(
        tool_name: impl Into<String>,
        arguments_json: impl Into<String>,
        risk_class: RiskClass,
        required_permission: PermissionKind,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            arguments_json: arguments_json.into(),
            risk_class,
            required_permission,
        }
    }
}
