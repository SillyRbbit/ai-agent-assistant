//! Bounded, non-authorizing model metadata. Discovery does not prove generation.
use serde::Serialize;
use serde_json::Value;

use crate::{agent_preferences::ReasoningEffort, personal_assistant_direct::DirectError};

// Deliberately no Debug: remote metadata and identifiers never enter diagnostics.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelInfo {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) evidence: String,
    pub(crate) availability: String,
    pub(crate) efforts: Vec<ReasoningEffort>,
    pub(crate) thinking: String,
    pub(crate) locality: String,
    pub(crate) context_limit: Option<u64>,
    pub(crate) output_limit: Option<u64>,
    pub(crate) size_bytes: Option<u64>,
    pub(crate) quantization: Option<String>,
    pub(crate) provenance: Option<String>,
    pub(crate) loaded: Option<bool>,
    pub(crate) capabilities: Option<Value>,
}

impl ModelInfo {
    /// A discovered identifier whose optional metadata has not been established.
    pub(crate) fn unknown(id: String) -> Self {
        Self {
            label: id.clone(),
            id,
            evidence: "discovered".into(),
            availability: "access_unknown".into(),
            efforts: vec![ReasoningEffort::Default],
            thinking: "unknown".into(),
            locality: "unknown".into(),
            context_limit: None,
            output_limit: None,
            size_bytes: None,
            quantization: None,
            provenance: None,
            loaded: None,
            capabilities: None,
        }
    }
}

pub(crate) fn validate_selection(
    model: &ModelInfo,
    effort: ReasoningEffort,
) -> Result<(), DirectError> {
    if model.id.is_empty()
        || model.id.len() > 256
        || model.id.chars().any(char::is_control)
        || !matches!(model.availability.as_str(), "access_unknown" | "available")
    {
        return Err(DirectError::ModelUnavailable);
    }
    if !model.efforts.contains(&effort) {
        return Err(DirectError::Unsupported);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unknown_metadata_cannot_claim_supported_effort_or_live_access() {
        let model = ModelInfo::unknown("runtime/exact:tag".into());
        assert_eq!(model.id, "runtime/exact:tag");
        assert_eq!(model.locality, "unknown");
        assert_eq!(model.availability, "access_unknown");
        assert_eq!(validate_selection(&model, ReasoningEffort::Default), Ok(()));
        for effort in [
            ReasoningEffort::None,
            ReasoningEffort::High,
            ReasoningEffort::Max,
        ] {
            assert_eq!(
                validate_selection(&model, effort),
                Err(DirectError::Unsupported)
            );
        }
        assert!(model.loaded.is_none() && model.capabilities.is_none());
    }
    #[test]
    fn unavailable_models_and_invalid_ids_fail_before_transport() {
        for availability in ["retired", "unsupported", "unrecognized"] {
            let mut model = ModelInfo::unknown("model".into());
            model.availability = availability.into();
            assert_eq!(
                validate_selection(&model, ReasoningEffort::Default),
                Err(DirectError::ModelUnavailable)
            );
        }
        for id in [String::new(), "x".repeat(257), "invalid\nmodel".into()] {
            assert_eq!(
                validate_selection(&ModelInfo::unknown(id), ReasoningEffort::Default),
                Err(DirectError::ModelUnavailable)
            );
        }
    }
}
