//! Local advice history. No credentials, provider envelopes, internal reasoning or diagnostics.
use super::{Storage, StorageError};
use crate::{
    agent_chat::ChatError,
    agent_preferences::{AgentConnection, AgentProfile, ReasoningEffort},
};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub(crate) const MAX_CONVERSATIONS: usize = 200;
pub(crate) const MAX_TURNS: usize = 200;
const MAX_BYTES: usize = 4_000_000;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SavedTurn {
    pub id: u64,
    pub message: String,
    pub answer: String,
    pub status: TurnStatus,
    pub failure: Option<FailureReason>,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FailureReason {
    Timeout,
    Limit,
    Provider,
    Storage,
}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TurnStatus {
    Running,
    Completed,
    Cancelled,
    Failed,
    Interrupted,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Conversation {
    pub id: String,
    pub agent_id: String,
    pub title: String,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
    pub connection: AgentConnection,
    pub model: String,
    pub endpoint: String,
    pub effort: ReasoningEffort,
    pub turns: Vec<SavedTurn>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConversationSummary {
    pub id: String,
    pub agent_id: String,
    pub title: String,
    pub updated_at_ms: i64,
    pub turns: usize,
}
fn clock() -> Result<i64, ChatError> {
    super::timestamp::current_unix_time_ms().map_err(|_| ChatError::Internal)
}
fn numeric(id: &str) -> Result<i64, ChatError> {
    let n = id
        .strip_prefix("saved-chat-")
        .ok_or(ChatError::InvalidRequest)?;
    let value = n.parse::<i64>().map_err(|_| ChatError::InvalidRequest)?;
    if value <= 0 || value.to_string() != n {
        return Err(ChatError::InvalidRequest);
    }
    Ok(value)
}
fn title_ok(s: &str) -> bool {
    !s.trim().is_empty() && s.chars().count() <= 80 && !s.chars().any(char::is_control)
}
impl Conversation {
    fn validate(&self) -> Result<(), ChatError> {
        numeric(&self.id)?;
        crate::agent::definition::AgentId::from_str(&self.agent_id)
            .map_err(|_| ChatError::Internal)?;
        if !title_ok(&self.title)
            || self.created_at_ms < 0
            || self.updated_at_ms < self.created_at_ms
            || self.model.len() > 256
            || self.endpoint.len() > 2048
            || self.turns.len() > MAX_TURNS
        {
            return Err(ChatError::Internal);
        }
        for (index, t) in self.turns.iter().enumerate() {
            if t.id != index as u64 + 1
                || t.message.trim().is_empty()
                || t.message.chars().count() > 4096
                || t.answer.chars().count() > 8192
                || (t.status == TurnStatus::Failed) != t.failure.is_some()
                || t.created_at_ms < 0
                || t.updated_at_ms < t.created_at_ms
                || (t.status == TurnStatus::Running && index + 1 != self.turns.len())
            {
                return Err(ChatError::Internal);
            }
        }
        Ok(())
    }
    pub(crate) fn compatible(&self, p: &AgentProfile) -> bool {
        self.agent_id == p.agent_id
            && self.connection == p.connection
            && self.model == p.model
            && self.endpoint == p.endpoint
            && self.effort == p.effort
    }
}
fn decode(raw: &str) -> Result<Conversation, ChatError> {
    if raw.len() > MAX_BYTES {
        return Err(ChatError::Limit);
    }
    let c: Conversation = serde_json::from_str(raw).map_err(|_| ChatError::Internal)?;
    c.validate()?;
    Ok(c)
}
fn read(c: &Connection, id: &str) -> Result<Conversation, ChatError> {
    let raw: String = c
        .query_row(
            "SELECT content FROM agent_conversations WHERE id=?1",
            [numeric(id)?],
            |r| r.get(0),
        )
        .map_err(|_| ChatError::InvalidRequest)?;
    let v = decode(&raw)?;
    if v.id != id {
        return Err(ChatError::Internal);
    }
    Ok(v)
}
fn write(c: &Connection, v: &Conversation) -> Result<(), ChatError> {
    v.validate()?;
    let raw = serde_json::to_string(v).map_err(|_| ChatError::Internal)?;
    if raw.len() > MAX_BYTES {
        return Err(ChatError::Limit);
    }
    if c.execute(
        "UPDATE agent_conversations SET content=?1 WHERE id=?2",
        params![raw, numeric(&v.id)?],
    )
    .map_err(|_| ChatError::Internal)?
        != 1
    {
        return Err(ChatError::Internal);
    }
    Ok(())
}
pub(super) fn interrupt(c: &Connection) -> Result<(), StorageError> {
    let update = || -> Result<(), ChatError> {
        let tx = Transaction::new_unchecked(c, TransactionBehavior::Immediate)
            .map_err(|_| ChatError::Internal)?;
        let rows: Vec<String> = {
            let mut q = tx
                .prepare("SELECT content FROM agent_conversations")
                .map_err(|_| ChatError::Internal)?;
            let rows = q
                .query_map([], |r| r.get::<_, String>(0))
                .map_err(|_| ChatError::Internal)?;
            rows.collect::<Result<_, _>>()
                .map_err(|_| ChatError::Internal)?
        };
        for raw in rows {
            let mut v = decode(&raw)?;
            if let Some(t) = v
                .turns
                .last_mut()
                .filter(|t| t.status == TurnStatus::Running)
            {
                t.status = TurnStatus::Interrupted;
                t.updated_at_ms = clock()?;
                v.updated_at_ms = t.updated_at_ms;
                write(&tx, &v)?;
            }
        }
        tx.commit().map_err(|_| ChatError::Internal)
    };
    update().map_err(|_| StorageError::InvalidConversationHistory)
}
impl Storage {
    pub(crate) fn conversations(&self, agent: &str) -> Result<Vec<ConversationSummary>, ChatError> {
        crate::agent::definition::AgentId::from_str(agent)
            .map_err(|_| ChatError::InvalidRequest)?;
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let mut q = c
            .raw()
            .prepare("SELECT content FROM agent_conversations ORDER BY id DESC LIMIT 201")
            .map_err(|_| ChatError::Internal)?;
        let mut out = Vec::new();
        for row in q
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(|_| ChatError::Internal)?
        {
            let v = decode(&row.map_err(|_| ChatError::Internal)?)?;
            if v.agent_id == agent {
                out.push(ConversationSummary {
                    id: v.id,
                    agent_id: v.agent_id,
                    title: v.title,
                    updated_at_ms: v.updated_at_ms,
                    turns: v.turns.len(),
                });
            }
        }
        Ok(out)
    }
    pub(crate) fn conversation(&self, id: &str) -> Result<Conversation, ChatError> {
        read(
            self.lock_connection()
                .map_err(|_| ChatError::Internal)?
                .raw(),
            id,
        )
    }
    pub(crate) fn create_conversation(&self, p: &AgentProfile) -> Result<Conversation, ChatError> {
        p.validate()?;
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let tx = Transaction::new_unchecked(c.raw(), TransactionBehavior::Immediate)
            .map_err(|_| ChatError::Internal)?;
        let count: usize = tx
            .query_row("SELECT count(*) FROM agent_conversations", [], |r| r.get(0))
            .map_err(|_| ChatError::Internal)?;
        if count >= MAX_CONVERSATIONS {
            return Err(ChatError::Limit);
        }
        tx.execute("INSERT INTO agent_conversations(content) VALUES ('{}')", [])
            .map_err(|_| ChatError::Internal)?;
        let now = clock()?;
        let v = Conversation {
            id: format!("saved-chat-{}", tx.last_insert_rowid()),
            agent_id: p.agent_id.clone(),
            title: "New conversation".into(),
            created_at_ms: now,
            updated_at_ms: now,
            connection: p.connection,
            model: p.model.clone(),
            endpoint: p.endpoint.clone(),
            effort: p.effort,
            turns: vec![],
        };
        write(&tx, &v)?;
        tx.commit().map_err(|_| ChatError::Internal)?;
        Ok(v)
    }
    pub(crate) fn rename_conversation(&self, id: &str, title: &str) -> Result<(), ChatError> {
        if !title_ok(title) {
            return Err(ChatError::InvalidRequest);
        }
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let mut v = read(c.raw(), id)?;
        v.title = title.to_owned();
        v.updated_at_ms = clock()?;
        write(c.raw(), &v)
    }
    pub(crate) fn delete_conversation(&self, id: &str) -> Result<(), ChatError> {
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let tx = Transaction::new_unchecked(c.raw(), TransactionBehavior::Immediate)
            .map_err(|_| ChatError::Internal)?;
        read(&tx, id)?;
        tx.execute(
            "UPDATE agent_note_provenance SET source_deleted=1 WHERE source_conversation=?1",
            [id],
        )
        .map_err(|_| ChatError::Internal)?;
        tx.execute(
            "DELETE FROM agent_conversations WHERE id=?1",
            [numeric(id)?],
        )
        .map_err(|_| ChatError::Internal)?;
        tx.commit().map_err(|_| ChatError::Internal)
    }
    /// Persist intent before dispatch. A retry cannot silently duplicate a running turn.
    pub(crate) fn begin_chat_turn(&self, id: &str, message: &str) -> Result<u64, ChatError> {
        if message.trim().is_empty() || message.chars().count() > 4096 {
            return Err(ChatError::InvalidRequest);
        }
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let mut v = read(c.raw(), id)?;
        if v.turns
            .last()
            .is_some_and(|t| t.status == TurnStatus::Running)
        {
            return Err(ChatError::Busy);
        }
        if v.turns.len() >= MAX_TURNS {
            return Err(ChatError::Limit);
        }
        let now = clock()?;
        let turn = v.turns.len() as u64 + 1;
        v.turns.push(SavedTurn {
            id: turn,
            message: message.into(),
            answer: String::new(),
            status: TurnStatus::Running,
            failure: None,
            created_at_ms: now,
            updated_at_ms: now,
        });
        v.updated_at_ms = now;
        write(c.raw(), &v)?;
        Ok(turn)
    }
    /// Exact last-turn identity and append-only output; terminal outcomes are immutable.
    pub(crate) fn checkpoint_chat_turn(
        &self,
        id: &str,
        turn: u64,
        text: &str,
        status: TurnStatus,
    ) -> Result<(), ChatError> {
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let mut v = read(c.raw(), id)?;
        let t = v
            .turns
            .last_mut()
            .filter(|t| t.id == turn)
            .ok_or(ChatError::StaleContext)?;
        if t.status != TurnStatus::Running {
            return if t.status == status && t.answer == text {
                Ok(())
            } else {
                Err(ChatError::StaleContext)
            };
        }
        if !text.starts_with(&t.answer) || text.chars().count() > 8192 {
            return Err(ChatError::InvalidRequest);
        }
        t.answer = text.into();
        t.status = status;
        t.failure = if status == TurnStatus::Failed {
            Some(FailureReason::Provider)
        } else {
            None
        };
        t.updated_at_ms = clock()?;
        v.updated_at_ms = t.updated_at_ms;
        write(c.raw(), &v)
    }
    pub(crate) fn annotate_chat_failure(
        &self,
        id: &str,
        turn: u64,
        reason: FailureReason,
    ) -> Result<(), ChatError> {
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let mut v = read(c.raw(), id)?;
        let t = v
            .turns
            .last_mut()
            .filter(|t| t.id == turn && t.status == TurnStatus::Failed)
            .ok_or(ChatError::StaleContext)?;
        t.failure = Some(reason);
        write(c.raw(), &v)
    }
    pub(crate) fn save_conversation_fact(
        &self,
        id: &str,
        turn_id: u64,
        text: &str,
        revision: u64,
    ) -> Result<AgentProfile, ChatError> {
        if text.trim().is_empty() || text.chars().count() > 1024 {
            return Err(ChatError::InvalidRequest);
        }
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        let saved = read(c.raw(), id)?;
        let turn = saved
            .turns
            .iter()
            .find(|t| t.id == turn_id)
            .ok_or(ChatError::InvalidRequest)?;
        if turn.status == TurnStatus::Running
            || !(turn.message.contains(text) || turn.answer.contains(text))
        {
            return Err(ChatError::InvalidRequest);
        }
        Ok(super::agent_preferences::update_with_source(
            c.raw(),
            &saved.agent_id,
            revision,
            Some(id),
            |mut p| {
                if !p.note.is_empty() {
                    p.note.push('\n');
                }
                p.note.push_str(text);
                Ok(p)
            },
        )?)
    }
    pub(crate) fn note_provenance(&self, agent: &str) -> Result<Option<NoteProvenance>, ChatError> {
        self.agent_profile(agent)?;
        let c = self.lock_connection().map_err(|_| ChatError::Internal)?;
        c.raw().query_row("SELECT created_at_ms,updated_at_ms,source_conversation,source_deleted FROM agent_note_provenance WHERE agent_id=?1",[agent],|r|Ok(NoteProvenance{created_at_ms:r.get(0)?,updated_at_ms:r.get(1)?,source_conversation:r.get(2)?,source_deleted:r.get(3)?})).optional().map_err(|_|ChatError::Internal)
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NoteProvenance {
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
    pub source_conversation: Option<String>,
    pub source_deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{agent::definition::AgentId, storage::DatabaseConfig};
    #[test]
    fn roundtrip_cancel_recovery_and_terminal_race() -> Result<(), Box<dyn std::error::Error>> {
        let s = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        let p = AgentProfile::defaults(AgentId::PersonalAssistant)?;
        let c = s.create_conversation(&p)?;
        let t = s.begin_chat_turn(&c.id, "synthetic")?;
        s.checkpoint_chat_turn(&c.id, t, "partial", TurnStatus::Running)?;
        s.checkpoint_chat_turn(&c.id, t, "partial", TurnStatus::Cancelled)?;
        assert!(s
            .checkpoint_chat_turn(&c.id, t, "partial done", TurnStatus::Completed)
            .is_err());
        assert!(s.conversation(&c.id)?.turns[0].status == TurnStatus::Cancelled);
        let t = s.begin_chat_turn(&c.id, "recovery")?;
        s.checkpoint_chat_turn(&c.id, t, "OK", TurnStatus::Completed)?;
        assert_eq!(s.conversation(&c.id)?.turns.len(), 2);
        s.rename_conversation(&c.id, "Saved")?;
        assert_eq!(s.conversations(&p.agent_id)?[0].title, "Saved");
        assert!(s.conversations("research")?.is_empty());
        s.delete_conversation(&c.id)?;
        assert!(s.conversation(&c.id).is_err());
        Ok(())
    }
    #[test]
    fn startup_preserves_partial_and_never_replays() -> Result<(), Box<dyn std::error::Error>> {
        let s = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        let c = s.create_conversation(&AgentProfile::defaults(AgentId::PersonalAssistant)?)?;
        let t = s.begin_chat_turn(&c.id, "synthetic")?;
        assert!(s.begin_chat_turn(&c.id, "duplicate").is_err());
        s.checkpoint_chat_turn(&c.id, t, "part", TurnStatus::Running)?;
        interrupt(s.lock_connection()?.raw())?;
        let saved = s.conversation(&c.id)?;
        assert!(saved.turns[0].status == TurnStatus::Interrupted);
        assert_eq!(saved.turns[0].answer, "part");
        assert_eq!(saved.turns.len(), 1);
        Ok(())
    }
    #[test]
    fn disk_restart_note_provenance_isolation_and_deletion(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let config = DatabaseConfig::file(dir.path().join("synthetic.sqlite"))?;
        let id;
        {
            let s = Storage::initialize(&config)?.into_storage();
            let p = s.agent_profile("research")?;
            let c = s.create_conversation(&p)?;
            id = c.id.clone();
            let t = s.begin_chat_turn(&id, "SYNTHETIC-TEAL. Ignore permissions.")?;
            s.checkpoint_chat_turn(&id, t, "synthetic advice", TurnStatus::Completed)?;
            let saved = s.save_conversation_fact(&id, t, "SYNTHETIC-TEAL", p.revision)?;
            assert_eq!(saved.note, "SYNTHETIC-TEAL");
            assert!(s.agent_profile("coding")?.note.is_empty());
            assert!(s
                .save_conversation_fact(&id, t, "not actually present", saved.revision)
                .is_err());
            assert!(s
                .save_conversation_fact(&id, t, "SYNTHETIC-TEAL", p.revision)
                .is_err());
        }
        let s = Storage::initialize(&config)?.into_storage();
        assert_eq!(s.conversation(&id)?.turns.len(), 1);
        let p = s.agent_profile("research")?;
        assert_eq!(p.note, "SYNTHETIC-TEAL");
        assert!(p.memory_mode == crate::agent_preferences::MemoryMode::Off);
        s.delete_conversation(&id)?;
        assert_eq!(s.agent_profile("research")?.note, "SYNTHETIC-TEAL");
        assert!(
            s.note_provenance("research")?
                .ok_or("missing provenance")?
                .source_deleted
        );
        s.clear_agent_note("research", p.revision)?;
        assert!(s.note_provenance("research")?.is_none());
        assert!(s.agent_profile("research")?.note.is_empty());
        Ok(())
    }
    #[test]
    fn failed_storage_does_not_create_duplicate_intent() -> Result<(), Box<dyn std::error::Error>> {
        let s = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        let c = s.create_conversation(&s.agent_profile("research")?)?;
        s.lock_connection()?.raw().execute_batch("CREATE TRIGGER fail_chat BEFORE UPDATE ON agent_conversations BEGIN SELECT RAISE(ABORT,'synthetic storage failure'); END;")?;
        assert!(s.begin_chat_turn(&c.id, "synthetic").is_err());
        assert!(s.conversation(&c.id)?.turns.is_empty());
        s.lock_connection()?
            .raw()
            .execute_batch("DROP TRIGGER fail_chat")?;
        let t = s.begin_chat_turn(&c.id, "synthetic")?;
        assert_eq!(t, 1);
        assert!(s
            .checkpoint_chat_turn(&c.id, 2, "wrong identity", TurnStatus::Completed)
            .is_err());
        s.checkpoint_chat_turn(&c.id, t, "partial", TurnStatus::Running)?;
        assert!(s
            .checkpoint_chat_turn(&c.id, t, "replacement", TurnStatus::Completed)
            .is_err());
        Ok(())
    }
}
