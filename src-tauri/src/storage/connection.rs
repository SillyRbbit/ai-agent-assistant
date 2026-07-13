use rusqlite::Connection;

use super::{
    config::{duration_as_millis, DatabaseConfig, DatabaseLocation},
    error::{StorageError, StorageResult},
    migrations::{
        apply_pending_migrations, list_applied_migrations, AppliedMigration, MigrationReport,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JournalMode {
    Memory,
    Wal,
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectionSettings {
    foreign_keys_enabled: bool,
    journal_mode: JournalMode,
    busy_timeout_ms: u64,
}

impl ConnectionSettings {
    #[must_use]
    pub fn foreign_keys_enabled(&self) -> bool {
        self.foreign_keys_enabled
    }

    #[must_use]
    pub fn journal_mode(&self) -> &JournalMode {
        &self.journal_mode
    }

    #[must_use]
    pub fn busy_timeout_ms(&self) -> u64 {
        self.busy_timeout_ms
    }
}

#[derive(Debug)]
pub struct DatabaseConnection {
    connection: Connection,
    settings: ConnectionSettings,
}

impl DatabaseConnection {
    pub fn open(config: &DatabaseConfig) -> StorageResult<Self> {
        config.validate()?;

        let connection = match config.location() {
            DatabaseLocation::InMemory => Connection::open_in_memory(),
            DatabaseLocation::File(path) => Connection::open(path),
        }
        .map_err(|source| StorageError::OpenDatabase { source })?;

        connection
            .busy_timeout(config.busy_timeout())
            .map_err(|source| StorageError::ConfigureConnection {
                setting: "busy_timeout",
                source,
            })?;

        connection
            .pragma_update(None, "foreign_keys", 1_i64)
            .map_err(|source| StorageError::ConfigureConnection {
                setting: "foreign_keys",
                source,
            })?;

        let foreign_keys_enabled = query_foreign_keys(&connection)?;
        if !foreign_keys_enabled {
            return Err(StorageError::ForeignKeysDisabled);
        }

        let journal_mode = match config.location() {
            DatabaseLocation::InMemory => query_journal_mode(&connection)?,
            DatabaseLocation::File(_) => enable_wal(&connection)?,
        };

        let busy_timeout_ms = query_busy_timeout(&connection)?;
        let expected_busy_timeout_ms = duration_as_millis(config.busy_timeout())?;
        if busy_timeout_ms != expected_busy_timeout_ms {
            return Err(StorageError::BusyTimeoutMismatch {
                expected_ms: expected_busy_timeout_ms,
                actual_ms: busy_timeout_ms,
            });
        }

        Ok(Self {
            connection,
            settings: ConnectionSettings {
                foreign_keys_enabled,
                journal_mode,
                busy_timeout_ms,
            },
        })
    }

    #[must_use]
    pub fn settings(&self) -> &ConnectionSettings {
        &self.settings
    }

    pub fn migrate(&mut self) -> StorageResult<MigrationReport> {
        apply_pending_migrations(&mut self.connection)
    }

    pub fn applied_migrations(&self) -> StorageResult<Vec<AppliedMigration>> {
        list_applied_migrations(&self.connection)
    }

    #[cfg(test)]
    pub(crate) fn raw_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    #[cfg(test)]
    pub(crate) fn table_exists(&self, table_name: &str) -> StorageResult<bool> {
        self.connection
            .table_exists(None, table_name)
            .map_err(|source| StorageError::InspectMigrations { source })
    }
}

fn query_foreign_keys(connection: &Connection) -> StorageResult<bool> {
    let enabled = connection
        .pragma_query_value(None, "foreign_keys", |row| row.get::<_, i64>(0))
        .map_err(|source| StorageError::ConfigureConnection {
            setting: "foreign_keys_verification",
            source,
        })?;
    Ok(enabled == 1)
}

fn enable_wal(connection: &Connection) -> StorageResult<JournalMode> {
    let mode = connection
        .pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get::<_, String>(0))
        .map_err(|source| StorageError::ConfigureConnection {
            setting: "journal_mode",
            source,
        })?;
    let journal_mode = journal_mode_from_sqlite(mode);

    if journal_mode != JournalMode::Wal {
        return Err(StorageError::WalModeUnavailable {
            actual: journal_mode_name(&journal_mode),
        });
    }

    Ok(journal_mode)
}

fn query_journal_mode(connection: &Connection) -> StorageResult<JournalMode> {
    let mode = connection
        .pragma_query_value(None, "journal_mode", |row| row.get::<_, String>(0))
        .map_err(|source| StorageError::ConfigureConnection {
            setting: "journal_mode_verification",
            source,
        })?;
    Ok(journal_mode_from_sqlite(mode))
}

fn query_busy_timeout(connection: &Connection) -> StorageResult<u64> {
    let milliseconds = connection
        .pragma_query_value(None, "busy_timeout", |row| row.get::<_, i64>(0))
        .map_err(|source| StorageError::ConfigureConnection {
            setting: "busy_timeout_verification",
            source,
        })?;

    u64::try_from(milliseconds).map_err(|_| StorageError::ConfigureConnection {
        setting: "busy_timeout_verification",
        source: rusqlite::Error::IntegralValueOutOfRange(0, milliseconds),
    })
}

fn journal_mode_from_sqlite(value: String) -> JournalMode {
    match value.to_ascii_lowercase().as_str() {
        "memory" => JournalMode::Memory,
        "wal" => JournalMode::Wal,
        _ => JournalMode::Other(value),
    }
}

fn journal_mode_name(mode: &JournalMode) -> String {
    match mode {
        JournalMode::Memory => "memory".to_owned(),
        JournalMode::Wal => "wal".to_owned(),
        JournalMode::Other(value) => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::{DatabaseConnection, JournalMode};
    use crate::storage::{config::DatabaseConfig, error::StorageResult};

    #[test]
    fn opens_in_memory_database_with_verified_settings() -> StorageResult<()> {
        let connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;

        assert!(connection.settings().foreign_keys_enabled());
        assert_eq!(connection.settings().journal_mode(), &JournalMode::Memory);
        assert_eq!(connection.settings().busy_timeout_ms(), 5_000);
        Ok(())
    }

    #[test]
    fn opens_file_database_with_wal_enabled() -> StorageResult<()> {
        let directory = tempdir()?;
        let path = directory.path().join("assistant.sqlite");
        let connection = DatabaseConnection::open(&DatabaseConfig::file(&path)?)?;

        assert!(connection.settings().foreign_keys_enabled());
        assert_eq!(connection.settings().journal_mode(), &JournalMode::Wal);
        assert!(path.exists());
        Ok(())
    }
}
