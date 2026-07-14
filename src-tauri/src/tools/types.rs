pub use super::schema::ToolSchema;

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
pub struct ToolDefinition {
    name: String,
    description: String,
    risk_class: RiskClass,
    required_permission: PermissionKind,
    schema: ToolSchema,
}

impl ToolDefinition {
    #[must_use]
    pub fn from_schema(schema: ToolSchema) -> Self {
        Self {
            name: schema.name().to_owned(),
            description: schema.description().to_owned(),
            risk_class: schema.risk_class(),
            required_permission: schema.required_permission(),
            schema,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn risk_class(&self) -> RiskClass {
        self.risk_class
    }

    #[must_use]
    pub fn required_permission(&self) -> PermissionKind {
        self.required_permission
    }

    #[must_use]
    pub fn schema(&self) -> ToolSchema {
        self.schema
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
