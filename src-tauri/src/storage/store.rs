use std::sync::{Mutex, MutexGuard};

use crate::{
    agent::definition::AgentId,
    agent_preferences::{AgentPreferencesInput, AgentProfile, PreferencesError},
};

use super::{
    connection::{ConnectionSettings, DatabaseConnection},
    error::{StorageError, StorageResult},
    metadata::{self, AppMetadataKey, AppMetadataRecord, AppMetadataValue},
    migrations::{AppliedMigration, MigrationReport},
    timestamp::current_unix_time_ms,
    DatabaseConfig, DatabaseLocation,
};

#[derive(Debug)]
pub struct StorageInitialization {
    storage: Storage,
    was_already_initialized: bool,
}

impl StorageInitialization {
    #[must_use]
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    #[must_use]
    pub const fn was_already_initialized(&self) -> bool {
        self.was_already_initialized
    }

    #[must_use]
    pub fn into_storage(self) -> Storage {
        self.storage
    }
}

#[derive(Debug)]
pub struct Storage {
    connection: Mutex<DatabaseConnection>,
    location: DatabaseLocation,
    connection_settings: ConnectionSettings,
    startup_migration_report: MigrationReport,
}

impl Storage {
    pub(crate) fn agent_profiles(&self) -> Result<Vec<AgentProfile>, PreferencesError> {
        let connection = self
            .lock_connection()
            .map_err(|_| PreferencesError::Storage)?;
        AgentId::ALL
            .into_iter()
            .map(|id| super::agent_preferences::get(connection.raw(), id.as_str()))
            .collect()
    }

    pub(crate) fn agent_profile(&self, agent_id: &str) -> Result<AgentProfile, PreferencesError> {
        let connection = self
            .lock_connection()
            .map_err(|_| PreferencesError::Storage)?;
        super::agent_preferences::get(connection.raw(), agent_id)
    }

    pub(crate) fn save_agent_preferences(
        &self,
        input: AgentPreferencesInput,
    ) -> Result<AgentProfile, PreferencesError> {
        let connection = self
            .lock_connection()
            .map_err(|_| PreferencesError::Storage)?;
        super::agent_preferences::save(connection.raw(), input)
    }

    pub(crate) fn clear_agent_note(
        &self,
        agent_id: &str,
        revision: u64,
    ) -> Result<AgentProfile, PreferencesError> {
        let connection = self
            .lock_connection()
            .map_err(|_| PreferencesError::Storage)?;
        super::agent_preferences::clear_note(connection.raw(), agent_id, revision)
    }

    pub(crate) fn restore_agent_defaults(
        &self,
        agent_id: &str,
        revision: u64,
    ) -> Result<AgentProfile, PreferencesError> {
        let connection = self
            .lock_connection()
            .map_err(|_| PreferencesError::Storage)?;
        super::agent_preferences::restore_defaults(connection.raw(), agent_id, revision)
    }

    pub fn initialize(config: &DatabaseConfig) -> StorageResult<StorageInitialization> {
        let location = config.location().clone();
        let mut connection = DatabaseConnection::open(config)?;
        let startup_migration_report = connection.migrate()?;
        let connection_settings = connection.settings().clone();
        let storage = Self {
            connection: Mutex::new(connection),
            location,
            connection_settings,
            startup_migration_report,
        };

        let existing = storage.get_metadata(AppMetadataKey::AppInitialized)?;
        let was_already_initialized = existing
            .as_ref()
            .is_some_and(|record| record.value() == &AppMetadataValue::Boolean(true));

        if !was_already_initialized {
            storage.set_metadata(
                AppMetadataKey::AppInitialized,
                AppMetadataValue::Boolean(true),
            )?;
        }

        Ok(StorageInitialization {
            storage,
            was_already_initialized,
        })
    }

    #[must_use]
    pub const fn location(&self) -> &DatabaseLocation {
        &self.location
    }

    #[must_use]
    pub const fn connection_settings(&self) -> &ConnectionSettings {
        &self.connection_settings
    }

    #[must_use]
    pub const fn startup_migration_report(&self) -> &MigrationReport {
        &self.startup_migration_report
    }

    pub fn applied_migrations(&self) -> StorageResult<Vec<AppliedMigration>> {
        self.lock_connection()?.applied_migrations()
    }

    pub fn get_metadata(&self, key: AppMetadataKey) -> StorageResult<Option<AppMetadataRecord>> {
        let connection = self.lock_connection()?;
        metadata::get(connection.raw(), key)
    }

    pub fn set_metadata(
        &self,
        key: AppMetadataKey,
        value: AppMetadataValue,
    ) -> StorageResult<AppMetadataRecord> {
        self.set_metadata_at(key, value, current_unix_time_ms()?)
    }

    fn set_metadata_at(
        &self,
        key: AppMetadataKey,
        value: AppMetadataValue,
        updated_at_ms: i64,
    ) -> StorageResult<AppMetadataRecord> {
        let connection = self.lock_connection()?;
        metadata::set(connection.raw(), key, value, updated_at_ms)
    }

    fn lock_connection(&self) -> StorageResult<MutexGuard<'_, DatabaseConnection>> {
        self.connection
            .lock()
            .map_err(|_| StorageError::ConnectionLockPoisoned)
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::params;
    use tempfile::tempdir;

    use super::Storage;
    use crate::storage::{
        AppMetadataKey, AppMetadataValue, DatabaseConfig, StorageError, StorageResult,
    };

    #[test]
    fn storage_is_send_and_sync_for_tauri_managed_state() {
        fn assert_send_sync<T: Send + Sync + 'static>() {}

        assert_send_sync::<Storage>();
    }

    #[test]
    fn initializes_file_storage_and_applies_migrations_once() -> StorageResult<()> {
        let directory = tempdir()?;
        let path = directory.path().join("assistant.sqlite");
        let config = DatabaseConfig::file(&path)?;

        let first = Storage::initialize(&config)?;
        assert!(!first.was_already_initialized());
        assert_eq!(
            first.storage().startup_migration_report().applied_versions,
            vec![1, 2, 3, 4, 5]
        );

        let first_record = first
            .storage()
            .get_metadata(AppMetadataKey::AppInitialized)?
            .ok_or(StorageError::MetadataWriteVerificationFailed {
                key: AppMetadataKey::AppInitialized.as_str(),
            })?;
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
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(
            second
                .storage()
                .get_metadata(AppMetadataKey::AppInitialized)?,
            Some(first_record)
        );
        Ok(())
    }

    #[test]
    fn creates_updates_and_reads_typed_metadata() -> StorageResult<()> {
        let initialization = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialization.storage();

        let created = storage.set_metadata_at(
            AppMetadataKey::AppInitialized,
            AppMetadataValue::Boolean(false),
            100,
        )?;
        let updated = storage.set_metadata_at(
            AppMetadataKey::AppInitialized,
            AppMetadataValue::Boolean(true),
            200,
        )?;

        assert_eq!(created.updated_at_ms(), 100);
        assert_eq!(updated.updated_at_ms(), 200);
        assert_eq!(
            storage.get_metadata(AppMetadataKey::AppInitialized)?,
            Some(updated)
        );
        Ok(())
    }

    #[test]
    fn rejects_invalid_stored_metadata_without_exposing_the_value() -> StorageResult<()> {
        let initialization = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialization.storage();
        let connection = storage.lock_connection()?;
        connection
            .raw()
            .execute(
                "UPDATE app_metadata SET value = ?1 WHERE key = ?2",
                params!["not-a-boolean", AppMetadataKey::AppInitialized.as_str()],
            )
            .map_err(|source| StorageError::WriteMetadata {
                key: AppMetadataKey::AppInitialized.as_str(),
                source,
            })?;
        drop(connection);

        assert!(matches!(
            storage.get_metadata(AppMetadataKey::AppInitialized),
            Err(StorageError::InvalidMetadataValue {
                key: "app_initialized"
            })
        ));
        Ok(())
    }

    #[test]
    fn rejects_negative_metadata_timestamps() -> StorageResult<()> {
        let initialization = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialization.storage();

        assert!(matches!(
            storage.set_metadata_at(
                AppMetadataKey::AppInitialized,
                AppMetadataValue::Boolean(true),
                -1,
            ),
            Err(StorageError::InvalidMetadataTimestamp)
        ));

        let connection = storage.lock_connection()?;
        connection
            .raw()
            .execute(
                "UPDATE app_metadata SET updated_at_ms = ?1 WHERE key = ?2",
                params![-1, AppMetadataKey::AppInitialized.as_str()],
            )
            .map_err(|source| StorageError::WriteMetadata {
                key: AppMetadataKey::AppInitialized.as_str(),
                source,
            })?;
        drop(connection);

        assert!(matches!(
            storage.get_metadata(AppMetadataKey::AppInitialized),
            Err(StorageError::InvalidMetadataTimestamp)
        ));
        Ok(())
    }
}
