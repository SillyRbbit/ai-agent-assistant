//! Owner-submitted room content only. No credentials, notes, envelopes or reasoning.
use super::Storage;
use crate::{
    agent_chat::ChatError,
    collaboration::{Room, Status, MAX_ROOMS},
};
use rusqlite::{params, Connection};

pub(super) fn interrupt(connection: &Connection) -> super::StorageResult<()> {
    // Storage initialization is the process boundary: unfinished work never replays.
    let mut q = connection
        .prepare("SELECT id,content FROM collaboration_rooms")
        .map_err(|_| super::StorageError::InvalidRoomHistory)?;
    let rows = q
        .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))
        .map_err(|_| super::StorageError::InvalidRoomHistory)?;
    for row in rows {
        let (id, raw) = row.map_err(|_| super::StorageError::InvalidRoomHistory)?;
        let mut room: Room =
            serde_json::from_str(&raw).map_err(|_| super::StorageError::InvalidRoomHistory)?;
        let mut changed = false;
        for run in &mut room.runs {
            if matches!(run.status, Status::Queued | Status::Running) {
                run.status = Status::Interrupted;
                run.sequence += 1;
                for s in &mut run.stages {
                    if matches!(s.status, Status::Running | Status::Queued) {
                        s.status = Status::Interrupted;
                    }
                }
                changed = true;
            }
        }
        if changed {
            let raw = serde_json::to_string(&room)
                .map_err(|_| super::StorageError::InvalidRoomHistory)?;
            connection
                .execute(
                    "UPDATE collaboration_rooms SET content=?1 WHERE id=?2",
                    params![raw, id],
                )
                .map_err(|_| super::StorageError::InvalidRoomHistory)?;
        }
    }
    Ok(())
}
impl Storage {
    pub(crate) fn collaboration_rooms(&self) -> Result<Vec<Room>, ChatError> {
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let mut q = c
            .raw()
            .prepare("SELECT content FROM collaboration_rooms ORDER BY id DESC LIMIT 10")
            .map_err(|_| ChatError::Internal)?;
        let rows = q
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(|_| ChatError::Internal)?;
        rows.map(|row| {
            let raw = row.map_err(|_| ChatError::Internal)?;
            if raw.len() > 2_000_000 {
                return Err(ChatError::Limit);
            }
            serde_json::from_str(&raw).map_err(|_| ChatError::Internal)
        })
        .collect()
    }
    pub(crate) fn create_collaboration_room(&self, title: &str) -> Result<Room, ChatError> {
        if title.trim().is_empty()
            || title.chars().count() > 80
            || title.chars().any(char::is_control)
        {
            return Err(ChatError::InvalidRequest);
        }
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let c = c.raw();
        let count: usize = c
            .query_row("SELECT count(*) FROM collaboration_rooms", [], |r| r.get(0))
            .map_err(|_| ChatError::Internal)?;
        if count >= MAX_ROOMS {
            return Err(ChatError::Limit);
        }
        let tx = c.unchecked_transaction().map_err(|_| ChatError::Internal)?;
        tx.execute("INSERT INTO collaboration_rooms(content) VALUES ('{}')", [])
            .map_err(|_| ChatError::Internal)?;
        let room = Room {
            id: tx.last_insert_rowid(),
            title: title.trim().into(),
            runs: Vec::new(),
        };
        let raw = serde_json::to_string(&room).map_err(|_| ChatError::Internal)?;
        tx.execute(
            "UPDATE collaboration_rooms SET content=?1 WHERE id=?2",
            params![raw, room.id],
        )
        .map_err(|_| ChatError::Internal)?;
        tx.commit().map_err(|_| ChatError::Internal)?;
        Ok(room)
    }
    pub(crate) fn save_collaboration_room(&self, room: &Room) -> Result<(), ChatError> {
        let raw = serde_json::to_string(room).map_err(|_| ChatError::Internal)?;
        if raw.len() > 2_000_000 || room.runs.len() > crate::collaboration::MAX_RUNS {
            return Err(ChatError::Limit);
        }
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let changed = c
            .raw()
            .execute(
                "UPDATE collaboration_rooms SET content=?1 WHERE id=?2",
                params![raw, room.id],
            )
            .map_err(|_| ChatError::Internal)?;
        if changed != 1 {
            return Err(ChatError::InvalidRequest);
        }
        Ok(())
    }
    pub(crate) fn delete_collaboration_room(&self, id: i64) -> Result<(), ChatError> {
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        c.raw()
            .execute("DELETE FROM collaboration_rooms WHERE id=?1", [id])
            .map_err(|_| ChatError::Internal)?;
        Ok(())
    }
}
