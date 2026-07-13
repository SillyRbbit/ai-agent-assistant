use std::collections::BTreeMap;

use rusqlite::{params, Connection, Transaction, TransactionBehavior};

use super::{
    error::{StorageError, StorageResult},
    timestamp::current_unix_time_ms,
};

const CREATE_SCHEMA_MIGRATIONS_SQL: &str = r#"
CREATE TABLE schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  checksum TEXT NOT NULL,
  applied_at_ms INTEGER NOT NULL
) STRICT;
"#;

const CREATE_APP_METADATA_SQL: &str = r#"
CREATE TABLE app_metadata (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL
) STRICT;
"#;

const MIGRATIONS: [MigrationDefinition; 2] = [
    MigrationDefinition {
        version: 1,
        name: "create_schema_migrations",
        checksum: "sha256:768eca88e829eb900182d8abba50183b42559c1f9975343e1070e57138692a6d",
        sql: CREATE_SCHEMA_MIGRATIONS_SQL,
    },
    MigrationDefinition {
        version: 2,
        name: "create_app_metadata",
        checksum: "sha256:d16b7f69f3e25949aed1fbbdcb750ccbd87931c931f78b67f5d8b7c619479ade",
        sql: CREATE_APP_METADATA_SQL,
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationSummary {
    pub version: i64,
    pub name: String,
    pub checksum: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppliedMigration {
    pub version: i64,
    pub name: String,
    pub checksum: String,
    pub applied_at_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationReport {
    pub applied_versions: Vec<i64>,
    pub already_applied_versions: Vec<i64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MigrationDefinition {
    version: i64,
    name: &'static str,
    checksum: &'static str,
    sql: &'static str,
}

#[must_use]
pub fn available_migrations() -> Vec<MigrationSummary> {
    MIGRATIONS
        .iter()
        .map(|migration| MigrationSummary {
            version: migration.version,
            name: migration.name.to_owned(),
            checksum: migration.checksum.to_owned(),
        })
        .collect()
}

pub(crate) fn apply_pending_migrations(
    connection: &mut Connection,
) -> StorageResult<MigrationReport> {
    apply_migrations_with_clock(connection, &MIGRATIONS, current_unix_time_ms)
}

pub(crate) fn list_applied_migrations(
    connection: &Connection,
) -> StorageResult<Vec<AppliedMigration>> {
    if !migration_table_exists(connection)? {
        return Ok(Vec::new());
    }

    let mut statement = connection
        .prepare(
            "SELECT version, name, checksum, applied_at_ms
             FROM schema_migrations
             ORDER BY version ASC",
        )
        .map_err(|source| StorageError::InspectMigrations { source })?;

    let rows = statement
        .query_map([], |row| {
            Ok(AppliedMigration {
                version: row.get(0)?,
                name: row.get(1)?,
                checksum: row.get(2)?,
                applied_at_ms: row.get(3)?,
            })
        })
        .map_err(|source| StorageError::InspectMigrations { source })?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|source| StorageError::InspectMigrations { source })
}

fn apply_migrations_with_clock<F>(
    connection: &mut Connection,
    migrations: &[MigrationDefinition],
    mut clock: F,
) -> StorageResult<MigrationReport>
where
    F: FnMut() -> StorageResult<i64>,
{
    validate_migration_order(migrations)?;

    let applied = list_applied_migrations(connection)?;
    validate_applied_migrations(&applied, migrations)?;
    let applied_by_version: BTreeMap<i64, AppliedMigration> = applied
        .into_iter()
        .map(|migration| (migration.version, migration))
        .collect();

    let mut report = MigrationReport {
        applied_versions: Vec::new(),
        already_applied_versions: Vec::new(),
    };

    for migration in migrations {
        if let Some(existing) = applied_by_version.get(&migration.version) {
            validate_applied_metadata(existing, migration)?;
            report.already_applied_versions.push(migration.version);
            continue;
        }

        apply_one_migration(connection, migration, clock()?)?;
        report.applied_versions.push(migration.version);
    }

    Ok(report)
}

fn validate_migration_order(migrations: &[MigrationDefinition]) -> StorageResult<()> {
    let mut previous = None;

    for migration in migrations {
        if migration.version <= 0 || previous.is_some_and(|version| migration.version <= version) {
            return Err(StorageError::InvalidMigrationOrder);
        }
        previous = Some(migration.version);
    }

    Ok(())
}

fn validate_applied_migrations(
    applied: &[AppliedMigration],
    migrations: &[MigrationDefinition],
) -> StorageResult<()> {
    for existing in applied {
        let known = migrations
            .iter()
            .any(|migration| migration.version == existing.version);
        if !known {
            return Err(StorageError::UnknownAppliedMigration {
                version: existing.version,
            });
        }
    }

    Ok(())
}

fn validate_applied_metadata(
    existing: &AppliedMigration,
    migration: &MigrationDefinition,
) -> StorageResult<()> {
    if existing.name != migration.name || existing.checksum != migration.checksum {
        return Err(StorageError::MigrationMetadataMismatch {
            version: migration.version,
            name: migration.name.to_owned(),
        });
    }

    Ok(())
}

fn apply_one_migration(
    connection: &mut Connection,
    migration: &MigrationDefinition,
    applied_at_ms: i64,
) -> StorageResult<()> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|source| StorageError::BeginMigration {
            version: migration.version,
            name: migration.name.to_owned(),
            source,
        })?;

    match execute_migration(&transaction, migration, applied_at_ms) {
        Ok(()) => transaction
            .commit()
            .map_err(|source| StorageError::CommitMigration {
                version: migration.version,
                name: migration.name.to_owned(),
                source,
            }),
        Err(execution) => match transaction.rollback() {
            Ok(()) => Err(StorageError::MigrationExecution {
                version: migration.version,
                name: migration.name.to_owned(),
                source: execution,
            }),
            Err(rollback) => Err(StorageError::MigrationRollback {
                version: migration.version,
                name: migration.name.to_owned(),
                execution: Box::new(execution),
                rollback: Box::new(rollback),
            }),
        },
    }
}

fn execute_migration(
    transaction: &Transaction<'_>,
    migration: &MigrationDefinition,
    applied_at_ms: i64,
) -> rusqlite::Result<()> {
    transaction.execute_batch(migration.sql)?;
    transaction.execute(
        "INSERT INTO schema_migrations (version, name, checksum, applied_at_ms)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            migration.version,
            migration.name,
            migration.checksum,
            applied_at_ms
        ],
    )?;
    Ok(())
}

fn migration_table_exists(connection: &Connection) -> StorageResult<bool> {
    connection
        .table_exists(None, "schema_migrations")
        .map_err(|source| StorageError::InspectMigrations { source })
}

#[cfg(test)]
mod tests {
    use super::{
        apply_migrations_with_clock, available_migrations, MigrationDefinition,
        CREATE_APP_METADATA_SQL, CREATE_SCHEMA_MIGRATIONS_SQL, MIGRATIONS,
    };
    use crate::storage::{
        config::DatabaseConfig,
        connection::DatabaseConnection,
        error::{StorageError, StorageResult},
    };

    #[test]
    fn lists_migrations_in_deterministic_version_order() {
        let versions: Vec<i64> = available_migrations()
            .into_iter()
            .map(|migration| migration.version)
            .collect();

        assert_eq!(versions, vec![1, 2]);
        assert_eq!(MIGRATIONS[0].sql, CREATE_SCHEMA_MIGRATIONS_SQL);
        assert_eq!(MIGRATIONS[1].sql, CREATE_APP_METADATA_SQL);
    }

    #[test]
    fn applies_initial_schema_and_is_idempotent() -> StorageResult<()> {
        let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;

        let first = apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS, || {
            Ok(1_700_000_000_000)
        })?;
        let second = apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS, || {
            Ok(1_700_000_000_001)
        })?;

        assert_eq!(first.applied_versions, vec![1, 2]);
        assert!(first.already_applied_versions.is_empty());
        assert!(second.applied_versions.is_empty());
        assert_eq!(second.already_applied_versions, vec![1, 2]);
        assert!(connection.table_exists("schema_migrations")?);
        assert!(connection.table_exists("app_metadata")?);

        let applied = connection.applied_migrations()?;
        let versions: Vec<i64> = applied.iter().map(|migration| migration.version).collect();
        assert_eq!(versions, vec![1, 2]);
        assert!(applied
            .iter()
            .all(|migration| migration.applied_at_ms == 1_700_000_000_000));
        Ok(())
    }

    #[test]
    fn rolls_back_a_failing_migration() -> StorageResult<()> {
        const FAILING_MIGRATION_SQL: &str = r#"
CREATE TABLE rollback_probe (
  id INTEGER PRIMARY KEY
) STRICT;
INSERT INTO missing_table (id) VALUES (1);
"#;
        const PLAN: [MigrationDefinition; 3] = [
            MIGRATIONS[0],
            MIGRATIONS[1],
            MigrationDefinition {
                version: 3,
                name: "deliberately_failing_migration",
                checksum: "sha256:test-only-failing-migration",
                sql: FAILING_MIGRATION_SQL,
            },
        ];

        let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;
        let result =
            apply_migrations_with_clock(connection.raw_mut(), &PLAN, || Ok(1_700_000_000_000));

        assert!(matches!(
            result,
            Err(StorageError::MigrationExecution { version: 3, .. })
        ));
        assert!(!connection.table_exists("rollback_probe")?);
        let versions: Vec<i64> = connection
            .applied_migrations()?
            .into_iter()
            .map(|migration| migration.version)
            .collect();
        assert_eq!(versions, vec![1, 2]);
        Ok(())
    }

    #[test]
    fn rejects_invalid_migration_order() -> StorageResult<()> {
        const INVALID_PLAN: [MigrationDefinition; 2] = [
            MigrationDefinition {
                version: 2,
                name: "second",
                checksum: "sha256:second",
                sql: "SELECT 1;",
            },
            MigrationDefinition {
                version: 1,
                name: "first",
                checksum: "sha256:first",
                sql: "SELECT 1;",
            },
        ];

        let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;
        let result = apply_migrations_with_clock(connection.raw_mut(), &INVALID_PLAN, || Ok(1));

        assert!(matches!(result, Err(StorageError::InvalidMigrationOrder)));
        Ok(())
    }
}
