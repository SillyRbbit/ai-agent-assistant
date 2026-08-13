use super::*;

pub(super) fn request(
    scenario_id: BoundedParallelScenarioId,
    active_limit: u8,
) -> BoundedParallelResult<BoundedParallelWorkflowRequest> {
    let (objective, policy, work_items) = match scenario_id {
        BoundedParallelScenarioId::ResearchKnowledgeIndependentV1 => (
            "Independently assess research and knowledge fixtures, then synthesize every ordered outcome.",
            BoundedParallelFailurePolicy::ContinuePartial,
            vec![
                item(
                    "research-analysis",
                    1,
                    AgentId::Research,
                    &[],
                    "Analyze the supplied approach comparison as an independent research brief.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[
                        ("approach-a", "Approach A", "Approach A has lower operational complexity and fixture cost."),
                        ("approach-b", "Approach B", "Approach B has greater scale potential and operational complexity."),
                    ],
                )?,
                item(
                    "knowledge-analysis",
                    2,
                    AgentId::KnowledgeDocument,
                    &[],
                    "Independently organize the supplied decision-record fixtures.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[
                        ("decision-context", "Decision context", "The decision prioritizes bounded operations and explicit evidence."),
                        ("decision-format", "Decision format", "The output must preserve alternatives, limitations, and unresolved issues."),
                    ],
                )?,
            ],
        ),
        BoundedParallelScenarioId::CodeSecurityQaV1 => (
            "Independently review a synthetic code proposal for quality and security, then validate both ordered outcomes.",
            BoundedParallelFailurePolicy::CancelDependentOnly,
            vec![
                item(
                    "coding-review",
                    1,
                    AgentId::Coding,
                    &[],
                    "Review the immutable off-by-one proposal fixture without applying it.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[("proposal-fixture", "Code proposal", "Change answer() from 41 to 42; no file mutation or test execution occurred.")],
                )?,
                item(
                    "security-review",
                    2,
                    AgentId::SecurityRisk,
                    &[],
                    "Review the immutable proposal threat fixture independently.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[("threat-fixture", "Threat boundary", "The proposal is inert fixture text with no credential, network, or execution authority.")],
                )?,
                item(
                    "qa-validation",
                    3,
                    AgentId::QaValidation,
                    &["coding-review", "security-review"],
                    "Validate the two ordered predecessor results without executing tests.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[("qa-criterion", "QA criterion", "Both predecessors must be present, attributed, internally consistent, and proposal-only.")],
                )?,
            ],
        ),
        BoundedParallelScenarioId::CloudSystemsSecurityV1 => (
            "Independently assess cloud and systems fixture facets, then perform one bounded security review.",
            BoundedParallelFailurePolicy::FailFast,
            vec![
                item(
                    "cloud-assessment",
                    1,
                    AgentId::CloudInfrastructure,
                    &[],
                    "Assess only the synthetic cloud configuration facet.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[("cloud-fixture", "Cloud facet", "A synthetic Terraform plan uses a private subnet and no live provider credentials.")],
                )?,
                item(
                    "systems-assessment",
                    2,
                    AgentId::SystemsOperations,
                    &[],
                    "Assess only the sanitized systems recovery facet.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[("systems-fixture", "Systems facet", "A sanitized service report shows one failed health check and a documented rollback step.")],
                )?,
                item(
                    "security-assessment",
                    3,
                    AgentId::SecurityRisk,
                    &["cloud-assessment", "systems-assessment"],
                    "Review the two ordered fixture assessments for residual risk.",
                    "One strict ParallelAnalysisResultV1 JSON object.",
                    &[("security-criterion", "Security criterion", "Review provenance, missing evidence, and residual risk without authorizing remediation.")],
                )?,
            ],
        ),
    };
    BoundedParallelWorkflowRequest::new(
        scenario_id,
        objective.to_owned(),
        policy,
        work_items,
        active_limit,
    )
}

fn item(
    id: &str,
    ordinal: u8,
    agent_id: AgentId,
    dependencies: &[&str],
    objective: &str,
    expected_output: &str,
    fixtures: &[(&str, &str, &str)],
) -> BoundedParallelResult<ParallelWorkItem> {
    Ok(ParallelWorkItem {
        id: ParallelWorkItemId::new(id)?,
        ordinal,
        agent_id,
        dependencies: dependencies
            .iter()
            .map(|value| ParallelWorkItemId::new(*value))
            .collect::<BoundedParallelResult<Vec<_>>>()?,
        objective: objective.to_owned(),
        expected_output: expected_output.to_owned(),
        fixtures: fixtures
            .iter()
            .map(|(id, label, content)| ParallelFixture::new(*id, *label, *content))
            .collect::<BoundedParallelResult<Vec<_>>>()?,
    })
}

#[cfg(test)]
pub(super) fn three_independent_for_test(
    active_limit: u8,
) -> BoundedParallelResult<BoundedParallelWorkflowRequest> {
    let mut value = request(BoundedParallelScenarioId::CodeSecurityQaV1, active_limit)?;
    value.work_items[2].dependencies.clear();
    value.work_items[2].agent_id = AgentId::QaValidation;
    Ok(value)
}
