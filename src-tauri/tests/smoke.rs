use ai_agent_assistant_lib::current_app_info;

#[test]
fn library_exposes_desktop_application_metadata() {
    let info = current_app_info();

    assert_eq!(info.name, "AI Agent Assistant");
    assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
    assert!(!info.target.is_empty());
    assert!(!info.architecture.is_empty());
}
