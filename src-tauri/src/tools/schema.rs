use std::fmt;

use serde::Deserialize;
use serde_json::{json, Value};
use thiserror::Error;

use super::types::{PermissionKind, RiskClass};

pub const MAX_LOCAL_TASK_TITLE_CHARACTERS: usize = 200;

pub type ToolArgumentValidationResult<T> = Result<T, ToolArgumentValidationError>;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ToolSchema {
    GetCurrentDatetimeV1,
    CreateLocalTaskV1,
}

impl ToolSchema {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::GetCurrentDatetimeV1 => "get_current_datetime",
            Self::CreateLocalTaskV1 => "create_local_task",
        }
    }

    #[must_use]
    pub fn description(self) -> &'static str {
        match self {
            Self::GetCurrentDatetimeV1 => "Get the current local date and time.",
            Self::CreateLocalTaskV1 => "Create a task in the local task list.",
        }
    }

    #[must_use]
    pub fn version(self) -> u16 {
        1
    }

    #[must_use]
    pub fn risk_class(self) -> RiskClass {
        match self {
            Self::GetCurrentDatetimeV1 => RiskClass::InformationOnly,
            Self::CreateLocalTaskV1 => RiskClass::ReversibleLocalAction,
        }
    }

    #[must_use]
    pub fn required_permission(self) -> PermissionKind {
        PermissionKind::None
    }

    #[must_use]
    pub fn strict_input_schema(self) -> Value {
        match self {
            Self::GetCurrentDatetimeV1 => json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false,
            }),
            Self::CreateLocalTaskV1 => json!({
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "minLength": 1,
                        "maxLength": MAX_LOCAL_TASK_TITLE_CHARACTERS,
                    },
                },
                "required": ["title"],
                "additionalProperties": false,
            }),
        }
    }

    pub(crate) fn validate_arguments(
        self,
        arguments_json: &str,
    ) -> ToolArgumentValidationResult<ValidatedToolArguments> {
        match self {
            Self::GetCurrentDatetimeV1 => {
                let parsed: Value = serde_json::from_str(arguments_json)
                    .map_err(|_| ToolArgumentValidationError::InvalidShape)?;
                if matches!(parsed, Value::Object(properties) if properties.is_empty()) {
                    Ok(ValidatedToolArguments::GetCurrentDatetime)
                } else {
                    Err(ToolArgumentValidationError::InvalidShape)
                }
            }
            Self::CreateLocalTaskV1 => validate_create_local_task(arguments_json),
        }
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum ToolArgumentValidationError {
    #[error("tool arguments do not match the exact local schema")]
    InvalidShape,
    #[error("local task title must not be empty")]
    EmptyTitle,
    #[error("local task title must not have surrounding whitespace")]
    NonCanonicalTitle,
    #[error("local task title exceeds the character limit")]
    TitleTooLong,
    #[error("local task title contains a control character")]
    TitleContainsControlCharacter,
}

#[derive(Eq, PartialEq)]
pub enum ValidatedToolArguments {
    GetCurrentDatetime,
    CreateLocalTask(CreateLocalTaskArguments),
}

impl fmt::Debug for ValidatedToolArguments {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetCurrentDatetime => formatter.write_str("GetCurrentDatetime"),
            Self::CreateLocalTask(_) => formatter
                .debug_tuple("CreateLocalTask")
                .field(&"[REDACTED]")
                .finish(),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct CreateLocalTaskArguments {
    title: String,
}

impl CreateLocalTaskArguments {
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl fmt::Debug for CreateLocalTaskArguments {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CreateLocalTaskArguments")
            .field("title", &"[REDACTED]")
            .finish()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCreateLocalTaskArguments {
    title: String,
}

fn validate_create_local_task(
    arguments_json: &str,
) -> ToolArgumentValidationResult<ValidatedToolArguments> {
    let parsed: RawCreateLocalTaskArguments = serde_json::from_str(arguments_json)
        .map_err(|_| ToolArgumentValidationError::InvalidShape)?;

    if parsed.title.trim().is_empty() {
        return Err(ToolArgumentValidationError::EmptyTitle);
    }
    if parsed.title.trim() != parsed.title {
        return Err(ToolArgumentValidationError::NonCanonicalTitle);
    }
    if parsed.title.chars().count() > MAX_LOCAL_TASK_TITLE_CHARACTERS {
        return Err(ToolArgumentValidationError::TitleTooLong);
    }
    if parsed.title.chars().any(char::is_control) {
        return Err(ToolArgumentValidationError::TitleContainsControlCharacter);
    }

    Ok(ValidatedToolArguments::CreateLocalTask(
        CreateLocalTaskArguments {
            title: parsed.title,
        },
    ))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        ToolArgumentValidationError, ToolSchema, ValidatedToolArguments,
        MAX_LOCAL_TASK_TITLE_CHARACTERS,
    };
    use crate::tools::types::{PermissionKind, RiskClass};

    #[test]
    fn exposes_exact_closed_schema_metadata() {
        let datetime = ToolSchema::GetCurrentDatetimeV1;
        assert_eq!(datetime.name(), "get_current_datetime");
        assert_eq!(
            datetime.description(),
            "Get the current local date and time."
        );
        assert_eq!(datetime.version(), 1);
        assert_eq!(datetime.risk_class(), RiskClass::InformationOnly);
        assert_eq!(datetime.required_permission(), PermissionKind::None);
        assert_eq!(
            datetime.strict_input_schema(),
            json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false,
            })
        );

        let local_task = ToolSchema::CreateLocalTaskV1;
        assert_eq!(local_task.name(), "create_local_task");
        assert_eq!(
            local_task.description(),
            "Create a task in the local task list."
        );
        assert_eq!(local_task.version(), 1);
        assert_eq!(local_task.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(local_task.required_permission(), PermissionKind::None);
        assert_eq!(
            local_task.strict_input_schema(),
            json!({
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "minLength": 1,
                        "maxLength": 200,
                    },
                },
                "required": ["title"],
                "additionalProperties": false,
            })
        );
    }

    #[test]
    fn validates_get_current_datetime_arguments() {
        let schema = ToolSchema::GetCurrentDatetimeV1;
        assert_eq!(
            schema.validate_arguments("{}"),
            Ok(ValidatedToolArguments::GetCurrentDatetime)
        );

        for invalid in [
            r#"{"timezone":"UTC"}"#,
            "[]",
            "null",
            "true",
            "1",
            r#""text""#,
            "{} trailing",
            "{",
        ] {
            assert_eq!(
                schema.validate_arguments(invalid),
                Err(ToolArgumentValidationError::InvalidShape)
            );
        }
    }

    #[test]
    fn validates_create_local_task_title_boundaries() {
        let schema = ToolSchema::CreateLocalTaskV1;
        let one_character = schema.validate_arguments(r#"{"title":"x"}"#);
        assert!(matches!(
            one_character,
            Ok(ValidatedToolArguments::CreateLocalTask(arguments)) if arguments.title() == "x"
        ));

        let maximum_title = "x".repeat(MAX_LOCAL_TASK_TITLE_CHARACTERS);
        let maximum_json = json!({ "title": maximum_title }).to_string();
        let maximum = schema.validate_arguments(&maximum_json);
        assert!(matches!(
            maximum,
            Ok(ValidatedToolArguments::CreateLocalTask(arguments))
                if arguments.title().chars().count() == MAX_LOCAL_TASK_TITLE_CHARACTERS
        ));
    }

    #[test]
    fn rejects_invalid_create_local_task_arguments() {
        let schema = ToolSchema::CreateLocalTaskV1;
        let oversized_title = "x".repeat(MAX_LOCAL_TASK_TITLE_CHARACTERS + 1);
        let oversized_json = json!({ "title": oversized_title }).to_string();
        let invalid_cases = [
            ("{}", ToolArgumentValidationError::InvalidShape),
            (
                r#"{"title":"Review plan","priority":"high"}"#,
                ToolArgumentValidationError::InvalidShape,
            ),
            (r#"{"title":42}"#, ToolArgumentValidationError::InvalidShape),
            ("[]", ToolArgumentValidationError::InvalidShape),
            ("null", ToolArgumentValidationError::InvalidShape),
            ("{", ToolArgumentValidationError::InvalidShape),
            (
                r#"{"title":"Review plan"} trailing"#,
                ToolArgumentValidationError::InvalidShape,
            ),
            (r#"{"title":""}"#, ToolArgumentValidationError::EmptyTitle),
            (
                r#"{"title":"   "}"#,
                ToolArgumentValidationError::EmptyTitle,
            ),
            (
                r#"{"title":" Review plan"}"#,
                ToolArgumentValidationError::NonCanonicalTitle,
            ),
            (
                r#"{"title":"Review plan "}"#,
                ToolArgumentValidationError::NonCanonicalTitle,
            ),
            (
                r#"{"title":"Review\nplan"}"#,
                ToolArgumentValidationError::TitleContainsControlCharacter,
            ),
            (&oversized_json, ToolArgumentValidationError::TitleTooLong),
        ];

        for (arguments, expected) in invalid_cases {
            assert_eq!(schema.validate_arguments(arguments), Err(expected));
        }
    }

    #[test]
    fn redacts_validated_argument_debug_output() {
        let sentinel = "private-task-title";
        let arguments_json = json!({ "title": sentinel }).to_string();
        let result = ToolSchema::CreateLocalTaskV1.validate_arguments(&arguments_json);

        assert!(matches!(
            result,
            Ok(ref arguments) if !format!("{arguments:?}").contains(sentinel)
        ));
    }
}
