//! Private, bounded profile rows. No raw SQLite error or content leaves this module.

use std::str::FromStr;

use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};

use crate::{
    agent::definition::AgentId,
    agent_preferences::{AgentPreferencesInput, AgentProfile, PreferencesError},
};

pub(super) fn get(
    connection: &Connection,
    agent_id: &str,
) -> Result<AgentProfile, PreferencesError> {
    let id = AgentId::from_str(agent_id).map_err(|_| PreferencesError::InvalidRequest)?;
    let row = connection
        .query_row(
            "SELECT connection, model, effort, owner_instructions, memory_mode, note, revision
             FROM agent_preferences WHERE agent_id = ?1",
            [id.as_str()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, u64>(6)?,
                ))
            },
        )
        .optional()
        .map_err(|_| PreferencesError::Storage)?;
    let Some((connection, model, effort, owner_instructions, memory_mode, note, revision)) = row
    else {
        return AgentProfile::defaults(id);
    };
    let input = AgentPreferencesInput {
        agent_id: id.as_str().to_owned(),
        connection: parse_stored_enum(&connection)?,
        model,
        effort: parse_stored_enum(&effort)?,
        owner_instructions,
        memory_mode: parse_stored_enum(&memory_mode)?,
        note,
        revision,
    };
    input.into_profile().map_err(|_| PreferencesError::Storage)
}

fn parse_stored_enum<T: serde::de::DeserializeOwned>(value: &str) -> Result<T, PreferencesError> {
    serde_json::from_value(serde_json::Value::String(value.to_owned()))
        .map_err(|_| PreferencesError::Storage)
}

pub(super) fn save(
    connection: &Connection,
    input: AgentPreferencesInput,
) -> Result<AgentProfile, PreferencesError> {
    let profile = input.into_profile()?;
    let agent_id = profile.agent_id.clone();
    let revision = profile.revision;
    update(connection, &agent_id, revision, |_| Ok(profile))
}

pub(super) fn clear_note(
    connection: &Connection,
    agent_id: &str,
    revision: u64,
) -> Result<AgentProfile, PreferencesError> {
    update(connection, agent_id, revision, |mut profile| {
        profile.note.clear();
        Ok(profile)
    })
}

pub(super) fn restore_defaults(
    connection: &Connection,
    agent_id: &str,
    revision: u64,
) -> Result<AgentProfile, PreferencesError> {
    update(connection, agent_id, revision, |profile| {
        let id =
            AgentId::from_str(&profile.agent_id).map_err(|_| PreferencesError::InvalidRequest)?;
        let mut defaults = AgentProfile::defaults(id)?;
        // Restoring settings is not consent to delete the separately managed note.
        defaults.note = profile.note;
        Ok(defaults)
    })
}

fn update(
    connection: &Connection,
    agent_id: &str,
    expected_revision: u64,
    change: impl FnOnce(AgentProfile) -> Result<AgentProfile, PreferencesError>,
) -> Result<AgentProfile, PreferencesError> {
    if expected_revision >= i64::MAX as u64 {
        return Err(PreferencesError::InvalidRequest);
    }
    AgentId::from_str(agent_id).map_err(|_| PreferencesError::InvalidRequest)?;
    let transaction = Transaction::new_unchecked(connection, TransactionBehavior::Immediate)
        .map_err(|_| PreferencesError::Storage)?;
    let current = get(&transaction, agent_id)?;
    if current.revision != expected_revision {
        return Err(PreferencesError::StaleContext);
    }
    let mut profile = change(current)?;
    profile.revision = expected_revision + 1;
    profile.validate()?;
    let affected = transaction
        .execute(
            "INSERT INTO agent_preferences
             (agent_id, connection, model, effort, owner_instructions, memory_mode, note, revision)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(agent_id) DO UPDATE SET
             connection = excluded.connection, model = excluded.model, effort = excluded.effort,
             owner_instructions = excluded.owner_instructions, memory_mode = excluded.memory_mode,
             note = excluded.note, revision = excluded.revision
             WHERE agent_preferences.revision = ?9",
            params![
                profile.agent_id,
                profile.connection.as_str(),
                profile.model,
                profile.effort.as_str(),
                profile.owner_instructions,
                profile.memory_mode.as_str(),
                profile.note,
                profile.revision,
                expected_revision,
            ],
        )
        .map_err(|_| PreferencesError::Storage)?;
    if affected != 1 {
        return Err(PreferencesError::StaleContext);
    }
    transaction
        .commit()
        .map_err(|_| PreferencesError::Storage)?;
    Ok(profile)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use tempfile::tempdir;

    use super::*;
    use crate::{
        agent_preferences::{
            AgentConnection, MemoryMode, ReasoningEffort, MAX_OWNER_INSTRUCTION_CHARACTERS,
            MAX_PRIVATE_NOTE_CHARACTERS, OPENAI_AGENT_MODEL,
        },
        storage::{DatabaseConfig, Storage},
    };

    fn input(profile: AgentProfile) -> AgentPreferencesInput {
        AgentPreferencesInput {
            agent_id: profile.agent_id,
            connection: profile.connection,
            model: profile.model,
            effort: profile.effort,
            owner_instructions: profile.owner_instructions,
            memory_mode: profile.memory_mode,
            note: profile.note,
            revision: profile.revision,
        }
    }

    #[test]
    fn nine_native_profiles_have_independent_defaults_and_restart_persistence(
    ) -> Result<(), Box<dyn Error>> {
        let directory = tempdir()?;
        let config = DatabaseConfig::file(directory.path().join("profiles.sqlite"))?;
        let initialization = Storage::initialize(&config)?;
        let storage = initialization.storage();
        let defaults = storage.agent_profiles()?;
        assert_eq!(defaults.len(), AgentId::ALL.len());
        for (profile, id) in defaults.iter().zip(AgentId::ALL) {
            assert_eq!(profile, &AgentProfile::defaults(id)?);
        }
        let mut expected = Vec::new();
        for (index, profile) in defaults.into_iter().enumerate() {
            let mut preferences = input(profile);
            preferences.owner_instructions = format!("Synthetic role instruction {index}");
            preferences.note = format!("Synthetic private note {index}");
            preferences.memory_mode = MemoryMode::PrivateNotes;
            if index % 2 == 0 {
                preferences.connection = AgentConnection::OpenaiApi;
                preferences.model = OPENAI_AGENT_MODEL.to_owned();
                preferences.effort = ReasoningEffort::Low;
            }
            let saved = storage.save_agent_preferences(preferences)?;
            assert_eq!(saved.revision, 1);
            expected.push(saved);
        }
        assert_eq!(storage.agent_profiles()?, expected);
        drop(initialization);
        let restarted = Storage::initialize(&config)?;
        assert_eq!(restarted.storage().agent_profiles()?, expected);
        Ok(())
    }

    #[test]
    fn clear_and_restore_invalidate_revision_without_deleting_other_agent_notes(
    ) -> Result<(), Box<dyn Error>> {
        let initialization = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialization.storage();
        for id in ["research", "coding"] {
            let mut preferences = input(storage.agent_profile(id)?);
            preferences.note = format!("Synthetic note for {id}");
            preferences.memory_mode = MemoryMode::PrivateNotes;
            preferences.owner_instructions = "Synthetic instruction".to_owned();
            preferences.connection = AgentConnection::OpenaiApi;
            preferences.model = OPENAI_AGENT_MODEL.to_owned();
            storage.save_agent_preferences(preferences)?;
        }
        let coding = storage.agent_profile("coding")?;
        let defaults = storage.restore_agent_defaults("research", 1)?;
        assert_eq!(defaults.connection, AgentConnection::Simulation);
        assert_eq!(defaults.memory_mode, MemoryMode::Off);
        assert_eq!(defaults.note, "Synthetic note for research");
        assert!(defaults.owner_instructions.is_empty());
        assert_eq!(defaults.revision, 2);
        let cleared = storage.clear_agent_note("research", 2)?;
        assert!(cleared.note.is_empty());
        assert_eq!(cleared.revision, 3);
        assert_eq!(storage.agent_profile("coding")?, coding);
        assert_eq!(
            storage.clear_agent_note("research", 2),
            Err(PreferencesError::StaleContext)
        );
        assert_eq!(
            storage.restore_agent_defaults("research", 2),
            Err(PreferencesError::StaleContext)
        );
        let cleared_again = storage.clear_agent_note("research", 3)?;
        assert_eq!(cleared_again.revision, 4);
        Ok(())
    }

    #[test]
    fn stale_updates_are_rejected_across_independent_storage_connections(
    ) -> Result<(), Box<dyn Error>> {
        let directory = tempdir()?;
        let config = DatabaseConfig::file(directory.path().join("profiles.sqlite"))?;
        let first = Storage::initialize(&config)?;
        let second = Storage::initialize(&config)?;
        let initial = input(first.storage().agent_profile("research")?);
        let mut edited = initial.clone();
        edited.note = "Synthetic retained note".to_owned();
        let saved = first.storage().save_agent_preferences(edited)?;
        assert_eq!(
            second.storage().save_agent_preferences(initial),
            Err(PreferencesError::StaleContext)
        );
        assert_eq!(second.storage().agent_profile("research")?, saved);
        Ok(())
    }

    #[test]
    fn accepts_only_closed_connection_model_and_effort_combinations() -> Result<(), Box<dyn Error>>
    {
        let initialization = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialization.storage();
        for effort in [
            ReasoningEffort::Default,
            ReasoningEffort::None,
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
            ReasoningEffort::Xhigh,
        ] {
            let mut preferences = input(storage.agent_profile("research")?);
            preferences.connection = AgentConnection::OpenaiApi;
            preferences.model = OPENAI_AGENT_MODEL.to_owned();
            preferences.effort = effort;
            assert_eq!(storage.save_agent_preferences(preferences)?.effort, effort);
        }
        let original = storage.agent_profile("research")?;
        for (connection, model, effort) in [
            (
                AgentConnection::OpenaiApi,
                "unsupported",
                ReasoningEffort::Default,
            ),
            (
                AgentConnection::Simulation,
                "simulation",
                ReasoningEffort::High,
            ),
            (
                AgentConnection::Simulation,
                OPENAI_AGENT_MODEL,
                ReasoningEffort::Default,
            ),
            (AgentConnection::Codex, "unavailable", ReasoningEffort::High),
            (
                AgentConnection::Codex,
                OPENAI_AGENT_MODEL,
                ReasoningEffort::Default,
            ),
        ] {
            let mut preferences = input(original.clone());
            preferences.connection = connection;
            preferences.model = model.to_owned();
            preferences.effort = effort;
            assert_eq!(
                storage.save_agent_preferences(preferences),
                Err(PreferencesError::UnsupportedSettings)
            );
            assert_eq!(storage.agent_profile("research")?, original);
        }
        let mut codex = input(original);
        codex.connection = AgentConnection::Codex;
        codex.model = "unavailable".to_owned();
        codex.effort = ReasoningEffort::Default;
        assert_eq!(
            storage.save_agent_preferences(codex)?.connection,
            AgentConnection::Codex
        );
        Ok(())
    }

    #[test]
    fn native_validation_bounds_unicode_and_rejects_invalid_ids_controls_and_revisions(
    ) -> Result<(), Box<dyn Error>> {
        let initialization = Storage::initialize(&DatabaseConfig::in_memory())?;
        let storage = initialization.storage();
        let mut boundary = input(storage.agent_profile("research")?);
        boundary.note = "🙂".repeat(MAX_PRIVATE_NOTE_CHARACTERS);
        boundary.owner_instructions = "é".repeat(MAX_OWNER_INSTRUCTION_CHARACTERS);
        let saved = storage.save_agent_preferences(boundary)?;
        for invalid_id in [
            "Research",
            " research",
            "orchestrator",
            "../../research",
            "",
        ] {
            assert_eq!(
                storage.agent_profile(invalid_id),
                Err(PreferencesError::InvalidRequest)
            );
            let mut invalid = input(saved.clone());
            invalid.agent_id = invalid_id.to_owned();
            assert_eq!(
                storage.save_agent_preferences(invalid),
                Err(PreferencesError::InvalidRequest)
            );
        }
        let mut too_long_note = input(saved.clone());
        too_long_note.note.push('a');
        assert_eq!(
            storage.save_agent_preferences(too_long_note),
            Err(PreferencesError::InvalidRequest)
        );
        let mut too_long_instructions = input(saved.clone());
        too_long_instructions.owner_instructions.push('a');
        assert_eq!(
            storage.save_agent_preferences(too_long_instructions),
            Err(PreferencesError::InvalidRequest)
        );
        for control in ['\0', '\r', '\u{7f}', '\u{85}'] {
            let mut invalid = input(saved.clone());
            invalid.note = control.to_string();
            assert_eq!(
                storage.save_agent_preferences(invalid),
                Err(PreferencesError::InvalidRequest)
            );
            let mut invalid = input(saved.clone());
            invalid.owner_instructions = control.to_string();
            assert_eq!(
                storage.save_agent_preferences(invalid),
                Err(PreferencesError::InvalidRequest)
            );
        }
        let mut invalid_revision = input(saved.clone());
        invalid_revision.revision = u64::MAX;
        assert_eq!(
            storage.save_agent_preferences(invalid_revision),
            Err(PreferencesError::InvalidRequest)
        );
        assert_eq!(
            storage.clear_agent_note("research", i64::MAX as u64),
            Err(PreferencesError::InvalidRequest)
        );
        let mut whitespace = input(saved);
        whitespace.note = "line one\n\tline two".to_owned();
        whitespace.owner_instructions.clear();
        assert!(storage.save_agent_preferences(whitespace).is_ok());
        Ok(())
    }

    #[test]
    fn debug_and_storage_failures_never_include_profile_content() -> Result<(), Box<dyn Error>> {
        let directory = tempdir()?;
        let path = directory.path().join("profiles.sqlite");
        let initialization = Storage::initialize(&DatabaseConfig::file(&path)?)?;
        let storage = initialization.storage();
        let mut preferences = input(storage.agent_profile("research")?);
        preferences.note = "Synthetic private content".to_owned();
        preferences.owner_instructions = "Synthetic owner context".to_owned();
        assert_eq!(
            format!("{preferences:?}"),
            "AgentPreferencesInput { content: [redacted] }"
        );
        let saved = storage.save_agent_preferences(preferences)?;
        assert_eq!(format!("{saved:?}"), "AgentProfile { content: [redacted] }");
        let connection = Connection::open(&path)?;
        connection.execute(
            "UPDATE agent_preferences SET model = ?1 WHERE agent_id = 'research'",
            ["Synthetic invalid content"],
        )?;
        assert_eq!(
            storage.agent_profile("research"),
            Err(PreferencesError::Storage)
        );
        assert_eq!(
            PreferencesError::Storage.to_string(),
            "Private agent preferences could not be stored or loaded."
        );
        assert_eq!(
            serde_json::to_string(&PreferencesError::Storage)?,
            "\"storage\""
        );
        Ok(())
    }

    #[test]
    fn ipc_input_rejects_unknown_credential_or_authority_fields() {
        let valid = serde_json::json!({
            "agentId":"research", "connection":"simulation", "model":"simulation",
            "effort":"default", "ownerInstructions":"", "memoryMode":"off", "note":"", "revision":0,
        });
        assert!(serde_json::from_value::<AgentPreferencesInput>(valid.clone()).is_ok());
        for unknown in [
            "credential",
            "apiKey",
            "tools",
            "permissions",
            "displayName",
        ] {
            let mut invalid = valid.clone();
            invalid[unknown] = serde_json::Value::Bool(true);
            assert!(serde_json::from_value::<AgentPreferencesInput>(invalid).is_err());
        }
    }
}
