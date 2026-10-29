use ai_agent_assistant_lib::current_usage_efficiency_demo_projection;
use std::error::Error;

#[test]
fn only_bounded_numeric_categorical_mock_contract_crosses_ipc() -> Result<(), Box<dyn Error>> {
    let projection = current_usage_efficiency_demo_projection()?;
    let source = include_str!("../src/usage_efficiency.rs");
    assert!(source.contains("fn get_usage_efficiency_demo_projection()"));
    assert_eq!(source.matches("#[tauri::command]").count(), 1);
    for forbidden in [
        "std::fs",
        "std::env",
        "std::process",
        "reqwest",
        "Storage",
        "SystemTime",
    ] {
        assert!(!source.contains(forbidden));
    }
    let bytes = serde_json::to_vec(&projection)?;
    assert!(bytes.len() <= 64 * 1024);
    let value = serde_json::to_value(projection)?;
    let object = value.as_object().ok_or("object required")?;
    assert_eq!(object.len(), 4);
    assert_eq!(value["schemaVersion"], 1);
    assert_eq!(value["provenance"], "synthetic");
    let views = value["views"].as_array().ok_or("views required")?;
    assert_eq!(views.len(), 24);
    for view in views {
        assert_eq!(view.as_object().ok_or("view required")?.len(), 12);
        assert!(view["entries"].as_u64().ok_or("entries")? <= 128);
        assert!(view["daily"].as_array().ok_or("daily")?.len() <= 30);
    }
    let serialized = String::from_utf8(bytes)?;
    // Opt-in test stdout supplies the same numeric fixture to browser boundary QA.
    println!("C1_PROJECTION={serialized}");
    for forbidden in [
        "taskId",
        "threadId",
        "account",
        "workspace",
        "prompt",
        "response",
        "cost",
        "quota",
        "credential",
        "lifetime",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "excluded field: {forbidden}"
        );
    }
    Ok(())
}
