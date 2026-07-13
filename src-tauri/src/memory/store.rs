use std::collections::BTreeMap;

use thiserror::Error;

use super::types::{MemoryId, MemoryRecord, MemoryRecordInput, MemoryRecordUpdate, MemoryType};

pub type MemoryResult<T> = Result<T, MemoryError>;

pub trait MemoryStore {
    fn create(&mut self, input: MemoryRecordInput) -> MemoryResult<MemoryRecord>;
    fn list(&self, memory_type: Option<MemoryType>) -> Vec<MemoryRecord>;
    fn update(&mut self, id: MemoryId, update: MemoryRecordUpdate) -> MemoryResult<MemoryRecord>;
    fn delete(&mut self, id: MemoryId) -> MemoryResult<MemoryRecord>;
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum MemoryError {
    #[error("memory title must not be empty")]
    EmptyTitle,
    #[error("memory content must not be empty")]
    EmptyContent,
    #[error("memory source must not be empty")]
    EmptySource,
    #[error("memory contains secret-like content and was rejected")]
    SecretLikeContentRejected,
    #[error("memory not found: {0}")]
    NotFound(u64),
    #[error("memory id space is exhausted")]
    IdSpaceExhausted,
    #[error("memory version space is exhausted")]
    VersionSpaceExhausted,
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryMemoryStore {
    next_id: u64,
    records: BTreeMap<MemoryId, MemoryRecord>,
}

impl InMemoryMemoryStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl MemoryStore for InMemoryMemoryStore {
    fn create(&mut self, input: MemoryRecordInput) -> MemoryResult<MemoryRecord> {
        validate_memory_text(&input.title, &input.content, &input.source)?;

        let Some(next_id) = self.next_id.checked_add(1) else {
            return Err(MemoryError::IdSpaceExhausted);
        };

        self.next_id = next_id;
        let record = MemoryRecord::from_input(MemoryId(self.next_id), input);
        self.records.insert(record.id, record.clone());
        Ok(record)
    }

    fn list(&self, memory_type: Option<MemoryType>) -> Vec<MemoryRecord> {
        self.records
            .values()
            .filter(|record| match memory_type {
                Some(filter) => record.memory_type == filter,
                None => true,
            })
            .cloned()
            .collect()
    }

    fn update(&mut self, id: MemoryId, update: MemoryRecordUpdate) -> MemoryResult<MemoryRecord> {
        let Some(record) = self.records.get_mut(&id) else {
            return Err(MemoryError::NotFound(id.0));
        };

        let next_title = match &update.title {
            Some(title) => title.as_str(),
            None => record.title.as_str(),
        };
        let next_content = match &update.content {
            Some(content) => content.as_str(),
            None => record.content.as_str(),
        };

        validate_memory_text(next_title, next_content, &record.source)?;

        if let Some(title) = update.title {
            record.title = title;
        }

        if let Some(content) = update.content {
            record.content = content;
        }

        let Some(next_version) = record.version.checked_add(1) else {
            return Err(MemoryError::VersionSpaceExhausted);
        };
        record.version = next_version;

        Ok(record.clone())
    }

    fn delete(&mut self, id: MemoryId) -> MemoryResult<MemoryRecord> {
        self.records.remove(&id).ok_or(MemoryError::NotFound(id.0))
    }
}

fn validate_memory_text(title: &str, content: &str, source: &str) -> MemoryResult<()> {
    if title.trim().is_empty() {
        return Err(MemoryError::EmptyTitle);
    }

    if content.trim().is_empty() {
        return Err(MemoryError::EmptyContent);
    }

    if source.trim().is_empty() {
        return Err(MemoryError::EmptySource);
    }

    if contains_secret_like_content(title)
        || contains_secret_like_content(content)
        || contains_secret_like_content(source)
    {
        return Err(MemoryError::SecretLikeContentRejected);
    }

    Ok(())
}

#[must_use]
pub fn contains_secret_like_content(input: &str) -> bool {
    let lower = input.to_ascii_lowercase();
    [
        "password",
        "api_key",
        "apikey",
        "access_token",
        "refresh_token",
        "private key",
    ]
    .iter()
    .any(|marker| lower.contains(*marker))
}

#[cfg(test)]
mod tests {
    use super::{contains_secret_like_content, InMemoryMemoryStore, MemoryError, MemoryStore};
    use crate::memory::types::{MemoryId, MemoryRecordInput, MemoryRecordUpdate, MemoryType};

    fn memory(title: &str, content: &str, memory_type: MemoryType) -> MemoryRecordInput {
        MemoryRecordInput::new(memory_type, title, content, "user-confirmed preference")
    }

    #[test]
    fn creates_lists_updates_and_deletes_memory_records() {
        let mut store = InMemoryMemoryStore::new();

        assert!(store
            .create(memory(
                "Call preference",
                "Prefers morning calls",
                MemoryType::Preference
            ))
            .is_ok());
        assert!(store
            .create(memory(
                "Project",
                "Working on launch plan",
                MemoryType::Working
            ))
            .is_ok());

        let preference_ids: Vec<MemoryId> = store
            .list(Some(MemoryType::Preference))
            .into_iter()
            .map(|record| record.id)
            .collect();
        assert_eq!(preference_ids, vec![MemoryId(1)]);

        assert!(matches!(
            store.update(
                MemoryId(1),
                MemoryRecordUpdate::new(None, Some("Prefers afternoon calls".to_owned())),
            ),
            Ok(record) if record.version == 2
        ));

        assert!(matches!(store.delete(MemoryId(2)), Ok(record) if record.id == MemoryId(2)));
        assert_eq!(store.list(None).len(), 1);
    }

    #[test]
    fn rejects_secret_like_memory_content() {
        let mut store = InMemoryMemoryStore::new();

        assert_eq!(
            store.create(memory(
                "Credential",
                "password is swordfish",
                MemoryType::Session
            )),
            Err(MemoryError::SecretLikeContentRejected)
        );
        assert!(contains_secret_like_content("refresh_token=abc"));
    }

    #[test]
    fn returns_not_found_for_missing_records() {
        let mut store = InMemoryMemoryStore::new();

        assert_eq!(store.delete(MemoryId(42)), Err(MemoryError::NotFound(42)));
    }
}
