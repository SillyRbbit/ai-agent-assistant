//! Task-only SQL and persistence; no changes to conversations or Knowledge.
use super::{timestamp::current_unix_time_ms, Storage};
use crate::local_tasks::{
    validate_identity, LocalTask, Priority, Result, SaveTask, SetCompletion, TaskError as E,
    TaskIdentity, MAX_SAFE, MAX_TASKS,
};
use rusqlite::{params, Connection, OptionalExtension, Row, Transaction, TransactionBehavior};

pub(super) const CREATE_LOCAL_TASKS_SQL: &str = r#"
CREATE TABLE local_tasks (
 id INTEGER PRIMARY KEY AUTOINCREMENT CHECK(id BETWEEN 1 AND 9007199254740991),
 revision INTEGER NOT NULL CHECK(revision BETWEEN 1 AND 9007199254740991),
 title TEXT NOT NULL CHECK(length(CAST(title AS BLOB)) BETWEEN 1 AND 240),
 notes TEXT NOT NULL CHECK(length(CAST(notes AS BLOB)) <= 8192),
 due_date TEXT,
 priority TEXT NOT NULL CHECK(priority IN ('low','normal','high')),
 completed INTEGER NOT NULL CHECK(completed IN (0,1)),
 created_ms INTEGER NOT NULL CHECK(created_ms BETWEEN 0 AND 9007199254740991),
 updated_ms INTEGER NOT NULL CHECK(updated_ms BETWEEN created_ms AND 9007199254740991)
) STRICT;
"#;
const SELECT: &str = "SELECT id,revision,title,notes,due_date,priority,completed,created_ms,updated_ms FROM local_tasks";
fn row(r: &Row<'_>) -> rusqlite::Result<LocalTask> {
    let priority: String = r.get(5)?;
    let priority = match priority.as_str() {
        "low" => Priority::Low,
        "normal" => Priority::Normal,
        "high" => Priority::High,
        _ => return Err(rusqlite::Error::InvalidQuery),
    };
    Ok(LocalTask {
        id: r.get(0)?,
        revision: r.get(1)?,
        title: r.get(2)?,
        notes: r.get(3)?,
        due_date: r.get(4)?,
        priority,
        completed: r.get(6)?,
        created_ms: r.get(7)?,
        updated_ms: r.get(8)?,
    })
}
fn validated(t: LocalTask) -> Result<LocalTask> {
    let input = SaveTask {
        id: None,
        expected_revision: None,
        title: t.title.clone(),
        notes: t.notes.clone(),
        due_date: t.due_date.clone(),
        priority: t.priority,
    };
    if input.validate().is_err()
        || !(1..=MAX_SAFE).contains(&t.id)
        || !(1..=MAX_SAFE).contains(&t.revision)
        || t.created_ms < 0
        || t.updated_ms < t.created_ms
        || t.updated_ms > MAX_SAFE
    {
        return Err(E::Storage);
    }
    Ok(t)
}
fn get(c: &Connection, id: i64) -> Result<LocalTask> {
    c.query_row(&format!("{SELECT} WHERE id=?1"), [id], row)
        .optional()
        .map_err(|_| E::Storage)?
        .ok_or(E::Unavailable)
        .and_then(validated)
}
fn current(c: &Connection, id: i64, revision: i64) -> Result<LocalTask> {
    let t = get(c, id)?;
    if t.revision != revision {
        return Err(E::Conflict);
    }
    Ok(t)
}
fn now() -> Result<i64> {
    current_unix_time_ms()
        .map_err(|_| E::Storage)
        .and_then(|t| {
            if (0..=MAX_SAFE).contains(&t) {
                Ok(t)
            } else {
                Err(E::Storage)
            }
        })
}
impl Storage {
    pub(crate) fn list_local_tasks(&self) -> Result<Vec<LocalTask>> {
        let lock = self.lock_connection().map_err(|_| E::Storage)?;
        let mut q=lock.raw().prepare(&format!("{SELECT} ORDER BY completed ASC, due_date IS NULL ASC, due_date ASC, id DESC LIMIT 501")).map_err(|_|E::Storage)?;
        let items = q
            .query_map([], row)
            .map_err(|_| E::Storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|_| E::Storage)?;
        if items.len() > MAX_TASKS {
            return Err(E::Limit);
        }
        items.into_iter().map(validated).collect()
    }

    pub(crate) fn save_local_task(&self, input: SaveTask) -> Result<LocalTask> {
        input.validate()?;
        let lock = self.lock_connection().map_err(|_| E::Storage)?;
        let tx = Transaction::new_unchecked(lock.raw(), TransactionBehavior::Immediate)
            .map_err(|_| E::Storage)?;
        let stamp = now()?;
        let id = match (input.id, input.expected_revision) {
            (Some(id), Some(revision)) => {
                let old = current(&tx, id, revision)?;
                let changed=tx.execute("UPDATE local_tasks SET title=?1,notes=?2,due_date=?3,priority=?4,revision=revision+1,updated_ms=?5 WHERE id=?6 AND revision=?7",params![input.title.trim(),input.notes,input.due_date,input.priority.as_str(),stamp.max(old.updated_ms),id,revision]).map_err(|_|E::Storage)?;
                if changed != 1 {
                    return Err(E::Conflict);
                }
                id
            }
            (None, None) => {
                let count: i64 = tx
                    .query_row("SELECT count(*) FROM local_tasks", [], |r| r.get(0))
                    .map_err(|_| E::Storage)?;
                if count >= MAX_TASKS as i64 {
                    return Err(E::Limit);
                }
                tx.execute("INSERT INTO local_tasks(revision,title,notes,due_date,priority,completed,created_ms,updated_ms) VALUES(1,?1,?2,?3,?4,0,?5,?5)",params![input.title.trim(),input.notes,input.due_date,input.priority.as_str(),stamp]).map_err(|_|E::Storage)?;
                tx.last_insert_rowid()
            }
            _ => return Err(E::InvalidRequest),
        };
        let task = get(&tx, id)?;
        tx.commit().map_err(|_| E::Storage)?;
        Ok(task)
    }
    pub(crate) fn set_local_task_completion(&self, input: SetCompletion) -> Result<LocalTask> {
        validate_identity(input.id, input.expected_revision)?;
        let lock = self.lock_connection().map_err(|_| E::Storage)?;
        let tx = Transaction::new_unchecked(lock.raw(), TransactionBehavior::Immediate)
            .map_err(|_| E::Storage)?;
        let old = current(&tx, input.id, input.expected_revision)?;
        let changed=tx.execute("UPDATE local_tasks SET completed=?1,revision=revision+1,updated_ms=?2 WHERE id=?3 AND revision=?4",params![input.completed,now()?.max(old.updated_ms),input.id,input.expected_revision]).map_err(|_|E::Storage)?;
        if changed != 1 {
            return Err(E::Conflict);
        }
        let task = get(&tx, input.id)?;
        tx.commit().map_err(|_| E::Storage)?;
        Ok(task)
    }
    pub(crate) fn remove_local_task(&self, input: TaskIdentity) -> Result<()> {
        validate_identity(input.id, input.expected_revision)?;
        let lock = self.lock_connection().map_err(|_| E::Storage)?;
        let tx = Transaction::new_unchecked(lock.raw(), TransactionBehavior::Immediate)
            .map_err(|_| E::Storage)?;
        current(&tx, input.id, input.expected_revision)?;
        if tx
            .execute(
                "DELETE FROM local_tasks WHERE id=?1 AND revision=?2",
                params![input.id, input.expected_revision],
            )
            .map_err(|_| E::Storage)?
            != 1
        {
            return Err(E::Conflict);
        }
        tx.commit().map_err(|_| E::Storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{AppMetadataKey, AppMetadataValue, DatabaseConfig};
    fn input(title: &str) -> SaveTask {
        SaveTask {
            id: None,
            expected_revision: None,
            title: title.into(),
            notes: "Owner note\nsecond line".into(),
            due_date: Some("2028-02-29".into()),
            priority: Priority::High,
        }
    }
    #[test]
    fn crud_revisions_completion_and_stale_delete(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let s = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        let first = s.save_local_task(input(" First "))?;
        assert_eq!(first.title, "First");
        let edited = s.save_local_task(SaveTask {
            id: Some(first.id),
            expected_revision: Some(first.revision),
            title: "Edited".into(),
            ..input("x")
        })?;
        assert_eq!(edited.revision, 2);
        assert!(!edited.completed);
        assert_eq!(
            s.remove_local_task(TaskIdentity {
                id: first.id,
                expected_revision: 1
            }),
            Err(E::Conflict)
        );
        assert_eq!(
            s.save_local_task(SaveTask {
                id: Some(first.id),
                expected_revision: Some(1),
                ..input("lost update")
            }),
            Err(E::Conflict)
        );
        let done = s.set_local_task_completion(SetCompletion {
            id: first.id,
            expected_revision: 2,
            completed: true,
        })?;
        assert!(done.completed);
        let reopened = s.set_local_task_completion(SetCompletion {
            id: first.id,
            expected_revision: done.revision,
            completed: false,
        })?;
        assert!(!reopened.completed);
        assert_eq!(reopened.created_ms, first.created_ms);
        assert!(reopened.updated_ms >= first.created_ms);
        s.remove_local_task(TaskIdentity {
            id: first.id,
            expected_revision: reopened.revision,
        })?;
        assert_eq!(
            s.remove_local_task(TaskIdentity {
                id: first.id,
                expected_revision: reopened.revision
            }),
            Err(E::Unavailable)
        );
        assert!(s.list_local_tasks()?.is_empty());
        Ok(())
    }
    #[test]
    fn survives_reopen_without_touching_other_tables(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let config = DatabaseConfig::file(dir.path().join("tasks.sqlite3"))?;
        let saved;
        {
            let s = Storage::initialize(&config)?.into_storage();
            s.set_metadata(
                AppMetadataKey::AppInitialized,
                AppMetadataValue::Boolean(true),
            )?;
            saved = s.save_local_task(input("Restart"))?;
        }
        let s = Storage::initialize(&config)?.into_storage();
        assert_eq!(s.list_local_tasks()?, vec![saved]);
        assert!(s.get_metadata(AppMetadataKey::AppInitialized)?.is_some());
        assert_eq!(s.applied_migrations()?.last().map(|m| m.version), Some(11));
        Ok(())
    }
    #[test]
    fn two_connections_cannot_overwrite_a_stale_revision(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let config = DatabaseConfig::file(dir.path().join("concurrency.sqlite3"))?;
        let a = Storage::initialize(&config)?.into_storage();
        let b = Storage::initialize(&config)?.into_storage();
        let first = a.save_local_task(input("Shared"))?;
        b.set_local_task_completion(SetCompletion {
            id: first.id,
            expected_revision: 1,
            completed: true,
        })?;
        assert_eq!(
            a.save_local_task(SaveTask {
                id: Some(first.id),
                expected_revision: Some(1),
                ..input("stale")
            }),
            Err(E::Conflict)
        );
        assert!(a.list_local_tasks()?[0].completed);
        Ok(())
    }
    #[test]
    fn rejects_bad_requests_before_writing_and_bounds_total(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let s = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        let mut bad = input("A");
        bad.due_date = Some("2027-02-29".into());
        assert_eq!(s.save_local_task(bad), Err(E::InvalidRequest));
        assert!(s.list_local_tasks()?.is_empty());
        for _ in 0..MAX_TASKS {
            s.save_local_task(input("Bounded"))?;
        }
        assert_eq!(s.save_local_task(input("Overflow")), Err(E::Limit));
        assert_eq!(s.list_local_tasks()?.len(), MAX_TASKS);
        Ok(())
    }
    #[test]
    fn version_ten_upgrade_preserves_knowledge_and_history(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut c = super::super::DatabaseConnection::open(&DatabaseConfig::in_memory())?;
        super::super::migrations::migrate_before_tasks_for_test(c.raw_mut())?;
        c.raw().execute(
            "INSERT INTO knowledge_items(content) VALUES(?1)",
            ["SYNTHETIC-PRESERVE"],
        )?;
        let count: i64 = c
            .raw()
            .query_row("SELECT count(*) FROM schema_migrations", [], |r| r.get(0))?;
        assert_eq!(count, 10);
        let before = c.applied_migrations()?;
        c.migrate()?;
        let after = c.applied_migrations()?;
        assert_eq!(&after[..before.len()], &before);
        let content: String =
            c.raw()
                .query_row("SELECT content FROM knowledge_items", [], |r| r.get(0))?;
        assert_eq!(content, "SYNTHETIC-PRESERVE");
        assert!(c.table_exists("local_tasks")?);
        Ok(())
    }
    #[test]
    fn corrupt_persisted_content_is_rejected_without_exposure(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let s = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        let task = s.save_local_task(input("Corruption fixture"))?;
        {
            let lock = s.lock_connection()?;
            lock.raw().execute(
                "UPDATE local_tasks SET due_date='2026-02-30' WHERE id=?1",
                [task.id],
            )?;
        }
        assert_eq!(s.list_local_tasks(), Err(E::Storage));
        assert_eq!(
            s.set_local_task_completion(SetCompletion {
                id: task.id,
                expected_revision: task.revision,
                completed: true
            }),
            Err(E::Storage)
        );
        assert_eq!(
            s.remove_local_task(TaskIdentity {
                id: task.id,
                expected_revision: task.revision
            }),
            Err(E::Storage)
        );
        Ok(())
    }
}
