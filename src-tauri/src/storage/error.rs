use std::{io, time::SystemTimeError};

use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("database file path must not be empty")]
    EmptyDatabasePath,
    #[error("database busy timeout must be greater than zero")]
    ZeroBusyTimeout,
    #[error("database busy timeout exceeds the maximum of {maximum_ms} milliseconds")]
    BusyTimeoutTooLarge { maximum_ms: u64 },
    #[error("database busy timeout cannot be represented in milliseconds")]
    BusyTimeoutOutOfRange,
    #[error(
        "SQLite busy timeout verification failed: expected {expected_ms} milliseconds, got {actual_ms}"
    )]
    BusyTimeoutMismatch { expected_ms: u64, actual_ms: u64 },
    #[error("failed to open the SQLite database: {source}")]
    OpenDatabase {
        #[source]
        source: rusqlite::Error,
    },
    #[error("failed to configure SQLite setting `{setting}`: {source}")]
    ConfigureConnection {
        setting: &'static str,
        #[source]
        source: rusqlite::Error,
    },
    #[error("SQLite foreign-key enforcement could not be enabled")]
    ForeignKeysDisabled,
    #[error("SQLite WAL journal mode was requested but SQLite reported `{actual}`")]
    WalModeUnavailable { actual: String },
    #[error("migration versions must be positive and strictly increasing")]
    InvalidMigrationOrder,
    #[error("database contains an unknown applied migration version: {version}")]
    UnknownAppliedMigration { version: i64 },
    #[error(
        "applied migration {version} metadata does not match the current definition for `{name}`"
    )]
    MigrationMetadataMismatch { version: i64, name: String },
    #[error("failed to inspect applied migrations: {source}")]
    InspectMigrations {
        #[source]
        source: rusqlite::Error,
    },
    #[error("failed to begin migration {version} (`{name}`): {source}")]
    BeginMigration {
        version: i64,
        name: String,
        #[source]
        source: rusqlite::Error,
    },
    #[error("migration {version} (`{name}`) failed: {source}")]
    MigrationExecution {
        version: i64,
        name: String,
        #[source]
        source: rusqlite::Error,
    },
    #[error(
        "migration {version} (`{name}`) failed and its transaction could not be rolled back: execution={execution}; rollback={rollback}"
    )]
    MigrationRollback {
        version: i64,
        name: String,
        execution: Box<rusqlite::Error>,
        rollback: Box<rusqlite::Error>,
    },
    #[error("failed to commit migration {version} (`{name}`): {source}")]
    CommitMigration {
        version: i64,
        name: String,
        #[source]
        source: rusqlite::Error,
    },
    #[error("the system clock is before the Unix epoch: {0}")]
    SystemClock(#[from] SystemTimeError),
    #[error("the current timestamp cannot be represented as a signed 64-bit millisecond value")]
    TimestampOutOfRange,
    #[error("temporary database setup failed: {0}")]
    Io(#[from] io::Error),
}
