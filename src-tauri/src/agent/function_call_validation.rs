use std::fmt;

use thiserror::Error;

use super::gateway_protocol::UntrustedFunctionCall;
use crate::tools::registry::{ToolRegistry, ToolRegistryError};
use crate::tools::schema::{ToolArgumentValidationError, ValidatedToolArguments};
use crate::tools::types::{PermissionKind, RiskClass};

pub type FunctionCallValidationResult<T> = Result<T, FunctionCallValidationError>;

/// A function call whose identity and arguments match one local schema.
///
/// This value is not approved, authorized, audited, or executable. A later
/// orchestration boundary must still apply policy and obtain any required
/// approval before dispatch.
#[derive(Eq, PartialEq)]
pub struct SchemaValidatedFunctionCall {
    run_id: String,
    gateway_request_id: String,
    call_id: String,
    tool_name: String,
    tool_contract_version: u16,
    arguments: ValidatedToolArguments,
    risk_class: RiskClass,
    required_permission: PermissionKind,
}

impl SchemaValidatedFunctionCall {
    #[must_use]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    #[must_use]
    pub fn gateway_request_id(&self) -> &str {
        &self.gateway_request_id
    }

    #[must_use]
    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    #[must_use]
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    #[must_use]
    pub fn tool_contract_version(&self) -> u16 {
        self.tool_contract_version
    }

    #[must_use]
    pub fn arguments(&self) -> &ValidatedToolArguments {
        &self.arguments
    }

    #[must_use]
    pub fn risk_class(&self) -> RiskClass {
        self.risk_class
    }

    #[must_use]
    pub fn required_permission(&self) -> PermissionKind {
        self.required_permission
    }
}

impl fmt::Debug for SchemaValidatedFunctionCall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SchemaValidatedFunctionCall")
            .field("run_id", &self.run_id)
            .field("gateway_request_id", &self.gateway_request_id)
            .field("call_id", &self.call_id)
            .field("tool_name", &self.tool_name)
            .field("tool_contract_version", &self.tool_contract_version)
            .field("arguments", &"[REDACTED]")
            .field("risk_class", &self.risk_class)
            .field("required_permission", &self.required_permission)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum FunctionCallValidationError {
    #[error("function name is not registered in the local tool catalog")]
    UnknownLocalTool,
    #[error("local tool registry lookup failed")]
    LocalRegistryFailure,
    #[error("function tool contract version does not match the local schema")]
    ToolContractVersionMismatch { expected: u16, actual: u16 },
    #[error("function arguments failed local schema validation: {reason}")]
    InvalidArguments { reason: ToolArgumentValidationError },
}

/// Consumes a normalized gateway call and validates it against local contracts.
///
/// Success establishes schema validity and locally owned classification only;
/// it does not grant permission or approval to execute the requested tool.
pub fn validate_function_call(
    call: UntrustedFunctionCall,
    registry: &impl ToolRegistry,
) -> FunctionCallValidationResult<SchemaValidatedFunctionCall> {
    let definition = registry.get(call.name()).map_err(map_registry_error)?;
    let schema = definition.schema();
    let actual_version = call.tool_contract_version();
    let expected_version = schema.version();

    if actual_version != expected_version {
        return Err(FunctionCallValidationError::ToolContractVersionMismatch {
            expected: expected_version,
            actual: actual_version,
        });
    }

    let arguments = schema
        .validate_arguments(call.arguments_json())
        .map_err(|reason| FunctionCallValidationError::InvalidArguments { reason })?;
    let validated = SchemaValidatedFunctionCall {
        run_id: call.run_id().to_owned(),
        gateway_request_id: call.gateway_request_id().to_owned(),
        call_id: call.call_id().to_owned(),
        tool_name: definition.name().to_owned(),
        tool_contract_version: expected_version,
        arguments,
        risk_class: definition.risk_class(),
        required_permission: definition.required_permission(),
    };

    drop(call);
    Ok(validated)
}

fn map_registry_error(error: ToolRegistryError) -> FunctionCallValidationError {
    match error {
        ToolRegistryError::UnknownTool(_) => FunctionCallValidationError::UnknownLocalTool,
        ToolRegistryError::EmptyName
        | ToolRegistryError::EmptyDescription
        | ToolRegistryError::DuplicateTool(_) => FunctionCallValidationError::LocalRegistryFailure,
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;

    use super::{validate_function_call, FunctionCallValidationError, SchemaValidatedFunctionCall};
    use crate::agent::gateway_protocol::{
        GatewayProtocolError, GatewayStreamValidator, UntrustedFunctionCall, ValidatedGatewayEvent,
        GATEWAY_PROTOCOL_VERSION,
    };
    use crate::tools::registry::{InMemoryToolRegistry, ToolRegistry, ToolRegistryResult};
    use crate::tools::schema::{ToolArgumentValidationError, ValidatedToolArguments};
    use crate::tools::types::{PermissionKind, RiskClass, ToolDefinition, ToolSchema};

    const RUN_ID: &str = "run-schema-validation-1";
    const GATEWAY_REQUEST_ID: &str = "gateway-request-schema-validation-1";

    fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
        let mut registry = InMemoryToolRegistry::new();
        registry.register(ToolDefinition::from_schema(
            ToolSchema::GetCurrentDatetimeV1,
        ))?;
        registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
        Ok(registry)
    }

    fn normalized_call(
        name: &str,
        tool_contract_version: u16,
        arguments_json: &str,
    ) -> Result<UntrustedFunctionCall, Box<dyn Error>> {
        let mut validator = GatewayStreamValidator::new(
            RUN_ID,
            GATEWAY_REQUEST_ID,
            [name.to_owned()],
            tool_contract_version,
        )?;
        let start_frame = json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": RUN_ID,
            "gateway_request_id": GATEWAY_REQUEST_ID,
            "sequence": 0,
            "event": {
                "type": "response_started",
                "provider_response_id": "provider-response-schema-validation-1",
            },
        });
        let function_frame = json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": RUN_ID,
            "gateway_request_id": GATEWAY_REQUEST_ID,
            "sequence": 1,
            "event": {
                "type": "function_call_completed",
                "call_id": "call-schema-validation-1",
                "name": name,
                "tool_contract_version": tool_contract_version,
                "arguments_json": arguments_json,
            },
        });

        let _ = validator.accept_frame(&serde_json::to_vec(&start_frame)?)?;
        match validator.accept_frame(&serde_json::to_vec(&function_frame)?)? {
            ValidatedGatewayEvent::FunctionCallCompleted { call } => Ok(call),
            _ => Err(GatewayProtocolError::MalformedEvent.into()),
        }
    }

    fn validate(
        name: &str,
        version: u16,
        arguments_json: &str,
    ) -> Result<SchemaValidatedFunctionCall, Box<dyn Error>> {
        let call = normalized_call(name, version, arguments_json)?;
        Ok(validate_function_call(call, &registry()?)?)
    }

    #[test]
    fn validates_a_local_task_with_locally_derived_metadata() -> Result<(), Box<dyn Error>> {
        let validated = validate("create_local_task", 1, r#"{"title":"Review plan"}"#)?;

        assert_eq!(validated.run_id(), RUN_ID);
        assert_eq!(validated.gateway_request_id(), GATEWAY_REQUEST_ID);
        assert_eq!(validated.call_id(), "call-schema-validation-1");
        assert_eq!(validated.tool_name(), "create_local_task");
        assert_eq!(validated.tool_contract_version(), 1);
        assert_eq!(validated.risk_class(), RiskClass::ReversibleLocalAction);
        assert_eq!(validated.required_permission(), PermissionKind::None);
        assert!(matches!(
            validated.arguments(),
            ValidatedToolArguments::CreateLocalTask(arguments)
                if arguments.title() == "Review plan"
        ));
        Ok(())
    }

    #[test]
    fn validates_the_no_argument_datetime_contract() -> Result<(), Box<dyn Error>> {
        let validated = validate("get_current_datetime", 1, "{}")?;

        assert_eq!(validated.run_id(), RUN_ID);
        assert_eq!(validated.gateway_request_id(), GATEWAY_REQUEST_ID);
        assert_eq!(validated.risk_class(), RiskClass::InformationOnly);
        assert_eq!(validated.required_permission(), PermissionKind::None);
        assert!(matches!(
            validated.arguments(),
            ValidatedToolArguments::GetCurrentDatetime
        ));
        Ok(())
    }

    #[test]
    fn rejects_a_gateway_allowed_tool_absent_from_the_local_catalog() -> Result<(), Box<dyn Error>>
    {
        let call = normalized_call("unregistered_local_tool", 1, "{}")?;

        assert_eq!(
            validate_function_call(call, &registry()?).err(),
            Some(FunctionCallValidationError::UnknownLocalTool)
        );
        Ok(())
    }

    #[test]
    fn independently_rejects_a_local_contract_version_mismatch() -> Result<(), Box<dyn Error>> {
        let call = normalized_call("create_local_task", 2, r#"{"title":"Review plan"}"#)?;

        assert_eq!(
            validate_function_call(call, &registry()?).err(),
            Some(FunctionCallValidationError::ToolContractVersionMismatch {
                expected: 1,
                actual: 2,
            })
        );
        Ok(())
    }

    #[test]
    fn rejects_gateway_normalized_objects_that_violate_the_local_schema(
    ) -> Result<(), Box<dyn Error>> {
        let invalid_cases = [
            ("{}", ToolArgumentValidationError::InvalidShape),
            (
                r#"{"title":"Review plan","priority":"high"}"#,
                ToolArgumentValidationError::InvalidShape,
            ),
            (r#"{"title":42}"#, ToolArgumentValidationError::InvalidShape),
            (r#"{"title":""}"#, ToolArgumentValidationError::EmptyTitle),
            (
                r#"{"title":" Review plan"}"#,
                ToolArgumentValidationError::NonCanonicalTitle,
            ),
            (
                r#"{"title":"Review\nplan"}"#,
                ToolArgumentValidationError::TitleContainsControlCharacter,
            ),
        ];

        for (arguments_json, reason) in invalid_cases {
            let call = normalized_call("create_local_task", 1, arguments_json)?;
            assert_eq!(
                validate_function_call(call, &registry()?).err(),
                Some(FunctionCallValidationError::InvalidArguments { reason })
            );
        }
        Ok(())
    }

    #[test]
    fn redacts_argument_content_from_validated_debug_and_errors() -> Result<(), Box<dyn Error>> {
        let sentinel = "private-task-title";
        let raw_arguments = json!({ "title": sentinel }).to_string();
        let validated = validate("create_local_task", 1, &raw_arguments)?;
        let debug_output = format!("{validated:?}");

        assert!(!debug_output.contains(sentinel));
        assert!(!debug_output.contains(&raw_arguments));

        let invalid_raw_arguments = json!({ "title": format!(" {sentinel}") }).to_string();
        let call = normalized_call("create_local_task", 1, &invalid_raw_arguments)?;
        let error = validate_function_call(call, &registry()?)
            .err()
            .ok_or(FunctionCallValidationError::LocalRegistryFailure)?;
        let error_output = format!("{error:?}");
        let error_display = error.to_string();

        assert!(!error_output.contains(sentinel));
        assert!(!error_output.contains(&invalid_raw_arguments));
        assert!(!error_display.contains(sentinel));
        assert!(!error_display.contains(&invalid_raw_arguments));
        Ok(())
    }
}
