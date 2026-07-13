#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemoryId(pub u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MemoryType {
    Session,
    Working,
    Preference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRecordInput {
    pub memory_type: MemoryType,
    pub title: String,
    pub content: String,
    pub source: String,
}

impl MemoryRecordInput {
    #[must_use]
    pub fn new(
        memory_type: MemoryType,
        title: impl Into<String>,
        content: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            memory_type,
            title: title.into(),
            content: content.into(),
            source: source.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRecordUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl MemoryRecordUpdate {
    #[must_use]
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        Self { title, content }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRecord {
    pub id: MemoryId,
    pub memory_type: MemoryType,
    pub title: String,
    pub content: String,
    pub source: String,
    pub version: u64,
}

impl MemoryRecord {
    #[must_use]
    pub fn from_input(id: MemoryId, input: MemoryRecordInput) -> Self {
        Self {
            id,
            memory_type: input.memory_type,
            title: input.title,
            content: input.content,
            source: input.source,
            version: 1,
        }
    }
}
