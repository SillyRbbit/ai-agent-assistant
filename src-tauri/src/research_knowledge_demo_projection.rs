use serde::Serialize;
use thiserror::Error;

const DISCLOSURE: &str = "DEMO MODE · SIMULATED AGENT DATA";
const FIXTURE_PROVENANCE: &str = "application-owned-synthetic-fixture";
const PROOF_BOUNDARY: &str = "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.";
const SCENARIO_ID: &str = "research-knowledge-demo-v1";
const SCHEMA_VERSION: &str = "research-knowledge-demo-projection-v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResearchKnowledgeDemoRole {
    id: &'static str,
    label: &'static str,
    state: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchKnowledgeDemoProjection {
    schema_version: &'static str,
    scenario_id: &'static str,
    disclosure: &'static str,
    proof_boundary: &'static str,
    fixture_provenance: &'static str,
    roles: [ResearchKnowledgeDemoRole; 3],
    simulated_outcomes: [&'static str; 3],
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResearchKnowledgeDemoProjectionError {
    #[error("Research/Knowledge demo projection unavailable.")]
    Unavailable,
}

fn validate_text(value: &str, maximum: usize) -> Result<(), ResearchKnowledgeDemoProjectionError> {
    let length = value.chars().count();
    if length == 0 || length > maximum {
        return Err(ResearchKnowledgeDemoProjectionError::Unavailable);
    }
    Ok(())
}

fn build_research_knowledge_demo_projection(
) -> Result<ResearchKnowledgeDemoProjection, ResearchKnowledgeDemoProjectionError> {
    let projection = ResearchKnowledgeDemoProjection {
        schema_version: SCHEMA_VERSION,
        scenario_id: SCENARIO_ID,
        disclosure: DISCLOSURE,
        proof_boundary: PROOF_BOUNDARY,
        fixture_provenance: FIXTURE_PROVENANCE,
        roles: [
            ResearchKnowledgeDemoRole {
                id: "personal-assistant",
                label: "Personal Assistant",
                state: "ready",
            },
            ResearchKnowledgeDemoRole {
                id: "research",
                label: "Research Agent",
                state: "ready",
            },
            ResearchKnowledgeDemoRole {
                id: "knowledge-document",
                label: "Knowledge & Document Agent",
                state: "ready",
            },
        ],
        simulated_outcomes: ["succeeded", "failed", "cancelled"],
    };

    for value in [
        projection.schema_version,
        projection.scenario_id,
        projection.disclosure,
        projection.fixture_provenance,
    ] {
        validate_text(value, 128)?;
    }
    validate_text(projection.proof_boundary, 160)?;
    for role in &projection.roles {
        validate_text(role.id, 128)?;
        validate_text(role.label, 128)?;
        validate_text(role.state, 128)?;
    }
    for outcome in projection.simulated_outcomes {
        validate_text(outcome, 128)?;
    }

    Ok(projection)
}

pub fn current_research_knowledge_demo_projection(
) -> Result<ResearchKnowledgeDemoProjection, ResearchKnowledgeDemoProjectionError> {
    build_research_knowledge_demo_projection()
}

#[tauri::command]
pub(crate) fn get_research_knowledge_demo_projection(
) -> Result<ResearchKnowledgeDemoProjection, ResearchKnowledgeDemoProjectionError> {
    current_research_knowledge_demo_projection()
}

#[cfg(test)]
mod tests {
    use super::{
        current_research_knowledge_demo_projection, get_research_knowledge_demo_projection,
        ResearchKnowledgeDemoProjectionError, DISCLOSURE, PROOF_BOUNDARY,
    };

    #[test]
    fn zero_argument_command_returns_the_exact_application_owned_projection(
    ) -> Result<(), ResearchKnowledgeDemoProjectionError> {
        let projection = get_research_knowledge_demo_projection()?;

        assert_eq!(projection.disclosure, DISCLOSURE);
        assert_eq!(projection.proof_boundary, PROOF_BOUNDARY);
        assert_eq!(projection.roles.len(), 3);
        assert_eq!(
            projection.simulated_outcomes,
            ["succeeded", "failed", "cancelled"]
        );

        Ok(())
    }

    #[test]
    fn keeps_every_exposed_string_within_the_closed_bounds(
    ) -> Result<(), ResearchKnowledgeDemoProjectionError> {
        let projection = current_research_knowledge_demo_projection()?;
        let bounded = [
            projection.schema_version,
            projection.scenario_id,
            projection.disclosure,
            projection.fixture_provenance,
        ];

        assert!(bounded
            .iter()
            .all(|value| !value.is_empty() && value.chars().count() <= 128));
        assert!(!projection.proof_boundary.is_empty());
        assert!(projection.proof_boundary.chars().count() <= 160);
        assert!(projection.roles.iter().all(|role| {
            [role.id, role.label, role.state]
                .iter()
                .all(|value| !value.is_empty() && value.chars().count() <= 128)
        }));
        assert!(projection
            .simulated_outcomes
            .iter()
            .all(|value| !value.is_empty() && value.chars().count() <= 128));

        Ok(())
    }

    #[test]
    fn maps_invalid_private_projection_text_to_one_closed_error() -> Result<(), serde_json::Error> {
        assert_eq!(super::validate_text(&"🧠".repeat(128), 128), Ok(()));
        assert_eq!(
            super::validate_text("", 128),
            Err(super::ResearchKnowledgeDemoProjectionError::Unavailable)
        );
        assert_eq!(
            super::validate_text(&"🧠".repeat(129), 128),
            Err(super::ResearchKnowledgeDemoProjectionError::Unavailable)
        );
        let serialized_error =
            serde_json::to_value(super::ResearchKnowledgeDemoProjectionError::Unavailable)?;
        assert_eq!(serialized_error, serde_json::json!("unavailable"));

        Ok(())
    }
}
