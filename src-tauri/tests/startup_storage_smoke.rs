use ai_agent_assistant_lib::storage::{
    AppMetadataKey, AppMetadataValue, DatabaseConfig, Storage, StorageResult,
};
use tempfile::tempdir;

#[test]
fn file_backed_startup_is_idempotent_and_persists_only_the_bootstrap_marker() -> StorageResult<()> {
    let directory = tempdir()?;
    let path = directory.path().join("assistant-development.sqlite3");
    let config = DatabaseConfig::file(&path)?;

    let first = Storage::initialize(&config)?;
    assert!(!first.was_already_initialized());
    assert_eq!(
        first.storage().startup_migration_report().applied_versions,
        vec![1, 2]
    );
    let first_record = first
        .storage()
        .get_metadata(AppMetadataKey::AppInitialized)?;
    assert_eq!(
        first_record.as_ref().map(|record| record.value().clone()),
        Some(AppMetadataValue::Boolean(true))
    );
    drop(first);

    let second = Storage::initialize(&config)?;
    assert!(second.was_already_initialized());
    assert!(second
        .storage()
        .startup_migration_report()
        .applied_versions
        .is_empty());
    assert_eq!(
        second
            .storage()
            .startup_migration_report()
            .already_applied_versions,
        vec![1, 2]
    );
    assert_eq!(
        second
            .storage()
            .get_metadata(AppMetadataKey::AppInitialized)?,
        first_record
    );
    Ok(())
}
