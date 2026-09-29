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

const CREATE_AGENT_PREFERENCES_SQL: &str = r#"
CREATE TABLE agent_preferences (
  agent_id TEXT PRIMARY KEY CHECK(agent_id IN (
    'personal-assistant', 'research', 'coding', 'cloud-infrastructure',
    'systems-operations', 'knowledge-document', 'qa-validation',
    'security-risk', 'workflow-automation'
  )),
  connection TEXT NOT NULL CHECK(connection IN ('simulation', 'openai_api', 'codex')),
  model TEXT NOT NULL,
  effort TEXT NOT NULL CHECK(effort IN ('default', 'none', 'low', 'medium', 'high', 'xhigh')),
  owner_instructions TEXT NOT NULL CHECK(length(owner_instructions) <= 4096),
  memory_mode TEXT NOT NULL CHECK(memory_mode IN ('off', 'private_notes')),
  note TEXT NOT NULL CHECK(length(note) <= 8192),
  revision INTEGER NOT NULL CHECK(revision > 0)
) STRICT;
"#;

const UPGRADE_AGENT_PREFERENCE_MODELS_SQL: &str = r#"
UPDATE agent_preferences
SET model = 'gpt-5.6-luna',
    revision = CASE
      WHEN revision < 9223372036854775807 THEN revision + 1
      ELSE revision
    END
WHERE connection = 'openai_api'
  AND model IN ('gpt-5.4-mini', 'gpt-5.4-nano');
"#;

// A sidecar extends connection configuration without rewriting the existing
// preferences table, notes, revisions, or migration history.
const CREATE_AGENT_CONNECTION_SETTINGS_SQL: &str = r#"
CREATE TABLE agent_connection_settings (
  agent_id TEXT PRIMARY KEY REFERENCES agent_preferences(agent_id),
  connection TEXT NOT NULL CHECK(connection IN (
    'simulation', 'openai_api', 'codex', 'anthropic_api', 'lm_studio', 'ollama'
  )),
  model TEXT NOT NULL CHECK(length(model) <= 256),
  effort TEXT NOT NULL CHECK(effort IN ('default', 'none', 'low', 'medium', 'high', 'xhigh', 'max')),
  endpoint TEXT NOT NULL CHECK(length(endpoint) <= 2048),
  local_auth INTEGER NOT NULL CHECK(local_auth IN (0, 1)),
  allow_unknown_locality_notes INTEGER NOT NULL CHECK(allow_unknown_locality_notes IN (0, 1)),
  CHECK(connection IN ('lm_studio', 'ollama') OR
        (endpoint = '' AND local_auth = 0 AND allow_unknown_locality_notes = 0))
) STRICT;
"#;

const MIGRATIONS: [MigrationDefinition; 5] = [
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
    MigrationDefinition {
        version: 3,
        name: "create_agent_preferences",
        checksum: "sha256:de2860c765836e89ced5164e9776708f9ecdfe248e54091b106975497e0f57b7",
        sql: CREATE_AGENT_PREFERENCES_SQL,
    },
    MigrationDefinition {
        version: 4,
        name: "upgrade_agent_preference_models",
        checksum: "sha256:25c320dee1ec4e2693c66b386639f6f52dbb20a2811cbc02911966cbe5b46681",
        sql: UPGRADE_AGENT_PREFERENCE_MODELS_SQL,
    },
    MigrationDefinition {
        version: 5,
        name: "create_agent_connection_settings",
        checksum: "sha256:676c971dd48eefc1ae7c53204537458118a7d05d490050e64bbcb1b788ad6be2",
        sql: CREATE_AGENT_CONNECTION_SETTINGS_SQL,
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
    use std::error::Error;

    use super::{
        apply_migrations_with_clock, available_migrations, MigrationDefinition,
        CREATE_AGENT_CONNECTION_SETTINGS_SQL, CREATE_AGENT_PREFERENCES_SQL,
        CREATE_APP_METADATA_SQL, CREATE_SCHEMA_MIGRATIONS_SQL, MIGRATIONS,
        UPGRADE_AGENT_PREFERENCE_MODELS_SQL,
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

        assert_eq!(versions, vec![1, 2, 3, 4, 5]);
        assert_eq!(MIGRATIONS[0].sql, CREATE_SCHEMA_MIGRATIONS_SQL);
        assert_eq!(MIGRATIONS[1].sql, CREATE_APP_METADATA_SQL);
        assert_eq!(MIGRATIONS[2].sql, CREATE_AGENT_PREFERENCES_SQL);
        assert_eq!(MIGRATIONS[3].sql, UPGRADE_AGENT_PREFERENCE_MODELS_SQL);
        assert_eq!(MIGRATIONS[4].sql, CREATE_AGENT_CONNECTION_SETTINGS_SQL);
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

        assert_eq!(first.applied_versions, vec![1, 2, 3, 4, 5]);
        assert!(first.already_applied_versions.is_empty());
        assert!(second.applied_versions.is_empty());
        assert_eq!(second.already_applied_versions, vec![1, 2, 3, 4, 5]);
        assert!(connection.table_exists("schema_migrations")?);
        assert!(connection.table_exists("app_metadata")?);
        assert!(connection.table_exists("agent_preferences")?);
        assert!(connection.table_exists("agent_connection_settings")?);

        let applied = connection.applied_migrations()?;
        let versions: Vec<i64> = applied.iter().map(|migration| migration.version).collect();
        assert_eq!(versions, vec![1, 2, 3, 4, 5]);
        assert!(applied
            .iter()
            .all(|migration| migration.applied_at_ms == 1_700_000_000_000));
        Ok(())
    }

    #[test]
    fn upgrades_existing_metadata_database_without_rewriting_prior_migrations() -> StorageResult<()>
    {
        let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;
        apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS[..2], || Ok(100))?;
        let prior = connection.applied_migrations()?;

        let upgrade = apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS, || Ok(200))?;
        let applied = connection.applied_migrations()?;

        assert_eq!(upgrade.already_applied_versions, vec![1, 2]);
        assert_eq!(upgrade.applied_versions, vec![3, 4, 5]);
        assert_eq!(&applied[..2], prior.as_slice());
        assert_eq!(applied[2].version, 3);
        assert_eq!(applied[2].applied_at_ms, 200);
        assert_eq!(applied[3].version, 4);
        assert_eq!(applied[3].applied_at_ms, 200);
        assert!(connection.table_exists("agent_preferences")?);
        Ok(())
    }

    #[test]
    fn upgrades_removed_openai_models_without_changing_other_profile_content(
    ) -> Result<(), Box<dyn Error>> {
        let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;
        apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS[..3], || Ok(100))?;
        for (agent_id, model, revision) in [
            ("research", "gpt-5.4-mini", 4_i64),
            ("coding", "gpt-5.4-nano", 9_i64),
        ] {
            connection.raw_mut().execute(
                "INSERT INTO agent_preferences
                 (agent_id, connection, model, effort, owner_instructions, memory_mode, note, revision)
                 VALUES (?1, 'openai_api', ?2, 'low', 'Keep concise.', 'private_notes', 'Owned note.', ?3)",
                rusqlite::params![agent_id, model, revision],
            )?;
        }
        connection.raw_mut().execute(
            "INSERT INTO agent_preferences
             (agent_id, connection, model, effort, owner_instructions, memory_mode, note, revision)
             VALUES ('qa-validation', 'simulation', 'simulation', 'default', '', 'off', '', 2)",
            [],
        )?;
        connection.raw_mut().execute(
            "INSERT INTO agent_preferences
             (agent_id, connection, model, effort, owner_instructions, memory_mode, note, revision)
             VALUES ('security-risk', 'openai_api', 'gpt-5.4-mini', 'default', '', 'off', '', 9223372036854775807)",
            [],
        )?;

        let report = apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS, || Ok(200))?;

        assert_eq!(report.applied_versions, vec![4, 5]);
        assert_eq!(report.already_applied_versions, vec![1, 2, 3]);
        for (agent_id, revision) in [("research", 5_i64), ("coding", 10_i64)] {
            let row = connection.raw_mut().query_row(
                "SELECT model, effort, owner_instructions, memory_mode, note, revision
                 FROM agent_preferences WHERE agent_id = ?1",
                [agent_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, i64>(5)?,
                    ))
                },
            )?;
            assert_eq!(
                row,
                (
                    "gpt-5.6-luna".to_owned(),
                    "low".to_owned(),
                    "Keep concise.".to_owned(),
                    "private_notes".to_owned(),
                    "Owned note.".to_owned(),
                    revision,
                )
            );
        }
        let simulation: (String, i64) = connection.raw_mut().query_row(
            "SELECT model, revision FROM agent_preferences WHERE agent_id = 'qa-validation'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        assert_eq!(simulation, ("simulation".to_owned(), 2));
        let saturated: (String, i64) = connection.raw_mut().query_row(
            "SELECT model, revision FROM agent_preferences WHERE agent_id = 'security-risk'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        assert_eq!(
            saturated,
            ("gpt-5.6-luna".to_owned(), 9_223_372_036_854_775_807)
        );
        Ok(())
    }

    #[test]
    fn connection_sidecar_preserves_every_v4_profile_and_prior_migration_record(
    ) -> Result<(), Box<dyn Error>> {
        let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;
        apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS[..4], || Ok(100))?;
        for (index, id) in crate::agent::definition::AgentId::ALL.iter().enumerate() {
            connection.raw_mut().execute(
                "INSERT INTO agent_preferences
                 (agent_id, connection, model, effort, owner_instructions, memory_mode, note, revision)
                 VALUES (?1, 'openai_api', 'gpt-5.6-luna', 'low', ?2, 'private_notes', ?3, ?4)",
                rusqlite::params![id.as_str(), format!("owner-{index}"),
                    format!("note-{index}"), index as i64 + 1],
            )?;
        }
        let prior = connection.applied_migrations()?;
        let schema: String = connection.raw_mut().query_row(
            "SELECT sql FROM sqlite_master WHERE name = 'agent_preferences'",
            [],
            |row| row.get(0),
        )?;
        let upgrade = apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS, || Ok(200))?;
        assert_eq!(upgrade.applied_versions, vec![5]);
        assert_eq!(upgrade.already_applied_versions, vec![1, 2, 3, 4]);
        assert_eq!(&connection.applied_migrations()?[..4], prior.as_slice());
        let after_schema: String = connection.raw_mut().query_row(
            "SELECT sql FROM sqlite_master WHERE name = 'agent_preferences'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(schema, after_schema);
        for (index, id) in crate::agent::definition::AgentId::ALL.iter().enumerate() {
            let row: (String, String, String, String, String, String, i64) = connection
                .raw_mut()
                .query_row(
                "SELECT connection, model, effort, owner_instructions, memory_mode, note, revision
                     FROM agent_preferences WHERE agent_id = ?1",
                [id.as_str()],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                    ))
                },
            )?;
            assert_eq!(
                row,
                (
                    "openai_api".into(),
                    "gpt-5.6-luna".into(),
                    "low".into(),
                    format!("owner-{index}"),
                    "private_notes".into(),
                    format!("note-{index}"),
                    index as i64 + 1
                )
            );
        }
        let count: i64 = connection.raw_mut().query_row(
            "SELECT count(*) FROM agent_connection_settings",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 0);
        Ok(())
    }

    #[test]
    fn failing_sidecar_creation_rolls_back_without_touching_v4_data() -> Result<(), Box<dyn Error>>
    {
        let mut connection = DatabaseConnection::open(&DatabaseConfig::in_memory())?;
        apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS[..4], || Ok(100))?;
        connection.raw_mut().execute(
            "INSERT INTO agent_preferences
             (agent_id, connection, model, effort, owner_instructions, memory_mode, note, revision)
             VALUES ('research', 'simulation', 'simulation', 'default', 'retained', 'private_notes', 'retained-note', 7)",
            [],
        )?;
        // A name collision rejects only the new CREATE TABLE; no owner row or
        // pre-existing schema is changed before that failure.
        connection
            .raw_mut()
            .execute("CREATE TABLE agent_connection_settings (sentinel TEXT)", [])?;
        let failed = apply_migrations_with_clock(connection.raw_mut(), &MIGRATIONS, || Ok(200));
        assert!(matches!(
            failed,
            Err(StorageError::MigrationExecution { version: 5, .. })
        ));
        let retained: (String, String, i64) = connection.raw_mut().query_row(
            "SELECT owner_instructions, note, revision FROM agent_preferences WHERE agent_id = 'research'",
            [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        assert_eq!(retained, ("retained".into(), "retained-note".into(), 7));
        assert_eq!(connection.applied_migrations()?.len(), 4);
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
