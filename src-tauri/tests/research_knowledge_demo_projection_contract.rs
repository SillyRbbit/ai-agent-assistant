use std::error::Error;

use ai_agent_assistant_lib::current_research_knowledge_demo_projection;
use serde_json::json;

#[test]
fn exposes_only_the_exact_closed_projection_contract() -> Result<(), Box<dyn Error>> {
    let projection = current_research_knowledge_demo_projection()?;
    let serialized = serde_json::to_value(projection)?;

    assert_eq!(
        serialized,
        json!({
            "schemaVersion": "research-knowledge-demo-projection-v1",
            "scenarioId": "research-knowledge-demo-v1",
            "disclosure": "DEMO MODE · SIMULATED AGENT DATA",
            "proofBoundary": "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
            "fixtureProvenance": "application-owned-synthetic-fixture",
            "roles": [
                {"id": "personal-assistant", "label": "Personal Assistant", "state": "ready"},
                {"id": "research", "label": "Research Agent", "state": "ready"},
                {"id": "knowledge-document", "label": "Knowledge & Document Agent", "state": "ready"}
            ],
            "simulatedOutcomes": ["succeeded", "failed", "cancelled"]
        })
    );
    Ok(())
}
