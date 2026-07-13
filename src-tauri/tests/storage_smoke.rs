use ai_agent_assistant_lib::storage::{DatabaseConfig, DatabaseConnection, StorageResult};

#[test]
fn public_storage_api_applies_and_lists_initial_migrations() -> StorageResult<()> {
    let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;

    let report = connection.migrate()?;
    let applied = connection.applied_migrations()?;
    let versions: Vec<i64> = applied
        .into_iter()
        .map(|migration| migration.version)
        .collect();

    assert_eq!(report.applied_versions, vec![1, 2]);
    assert!(report.already_applied_versions.is_empty());
    assert_eq!(versions, vec![1, 2]);
    Ok(())
}
