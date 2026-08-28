use ai_agent_assistant_lib::{
    ResearchKnowledgeDemoHost, ResearchKnowledgeDemoLifecycleError,
    ResearchKnowledgeDemoLifecycleState,
};

const ADAPTER_SOURCE: &str = include_str!("../src/research_knowledge_demo_lifecycle_tauri.rs");
const ENTRYPOINT_SOURCE: &str = include_str!("../src/lib.rs");

#[test]
fn lifecycle_tauri_boundary_is_exact_no_input_and_content_free(
) -> Result<(), Box<dyn std::error::Error>> {
    fn assert_send_static<T: Send + 'static>() {}
    assert_send_static::<ResearchKnowledgeDemoHost>();

    assert_eq!(ADAPTER_SOURCE.matches("#[tauri::command]").count(), 4);
    for command in [
        "get_research_knowledge_demo_lifecycle_snapshot",
        "start_research_knowledge_demo_lifecycle",
        "advance_research_knowledge_demo_lifecycle",
        "cancel_research_knowledge_demo_lifecycle",
    ] {
        assert_eq!(ENTRYPOINT_SOURCE.matches(command).count(), 1);
    }
    assert_eq!(
        ADAPTER_SOURCE
            .matches("research-knowledge-demo-lifecycle-v1")
            .count(),
        1
    );
    assert!(ADAPTER_SOURCE.contains("Mutex<ResearchKnowledgeDemoHost>"));
    assert!(!ADAPTER_SOURCE.contains("run_id"));
    assert!(!ADAPTER_SOURCE.contains("task_id"));
    assert!(!ADAPTER_SOURCE.contains("workflow_id"));
    assert!(!ADAPTER_SOURCE.contains("profile"));
    assert!(!ADAPTER_SOURCE.contains("runtime"));
    assert!(!ADAPTER_SOURCE.contains("objective"));
    assert!(!ADAPTER_SOURCE.contains("fixture"));
    assert!(!ADAPTER_SOURCE.contains("outcome"));
    assert!(!ADAPTER_SOURCE.contains("script"));
    assert!(!ADAPTER_SOURCE.contains("stage"));
    assert!(!ADAPTER_SOURCE.contains("std::fs"));
    assert!(!ADAPTER_SOURCE.contains("std::process"));
    assert!(!ADAPTER_SOURCE.contains("std::thread"));
    assert!(!ADAPTER_SOURCE.contains("unsafe"));

    let mut host = ResearchKnowledgeDemoHost::new();
    assert_eq!(
        host.start().map(|snapshot| snapshot.state()),
        Ok(ResearchKnowledgeDemoLifecycleState::Research)
    );
    host.advance()?;
    host.advance()?;
    assert_eq!(
        host.advance()
            .map(|transition| transition.snapshot().state()),
        Ok(ResearchKnowledgeDemoLifecycleState::Succeeded)
    );
    host.start()?;
    host.advance()?;
    host.advance()?;
    assert_eq!(
        host.advance()
            .map(|transition| transition.snapshot().state()),
        Ok(ResearchKnowledgeDemoLifecycleState::Failed)
    );
    assert_eq!(
        serde_json::to_value(ResearchKnowledgeDemoLifecycleError::Unavailable)?,
        serde_json::json!("unavailable")
    );

    Ok(())
}
