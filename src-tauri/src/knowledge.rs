//! Bounded library values. No provider, private-note or filesystem authority.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
pub(crate) const MAX_BYTES: usize = 16_384;
pub(crate) const MAX_ITEMS: usize = 200;
pub(crate) const MAX_VERSIONS: usize = 8;
pub(crate) const MAX_TOTAL: usize = 4 * 1024 * 1024;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub(crate) enum KnowledgeError {
    #[error("invalid_content")]
    InvalidContent,
    #[error("limit")]
    Limit,
    #[error("duplicate")]
    Duplicate,
    #[error("stale_selection")]
    StaleSelection,
    #[error("unavailable")]
    Unavailable,
    #[cfg(any(target_os = "macos", test))]
    #[error("unsupported_file")]
    UnsupportedFile,
    #[cfg(any(target_os = "macos", test))]
    #[error("unsafe_path")]
    UnsafePath,
    #[cfg(target_os = "macos")]
    #[error("changed_file")]
    ChangedFile,
    #[cfg(target_os = "macos")]
    #[error("invalid_utf8")]
    InvalidUtf8,
    #[cfg(any(target_os = "macos", test))]
    #[error("destination_exists")]
    DestinationExists,
    #[error("storage")]
    Storage,
}
pub(crate) type Result<T> = std::result::Result<T, KnowledgeError>;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Kind {
    Imported,
    OwnerNote,
    GeneratedDraft,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Version {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub hash: String,
    pub format: String,
    pub created_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<crate::knowledge_links::Link>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Item {
    pub id: i64,
    pub kind: Kind,
    pub versions: Vec<Version>,
}
impl Item {
    pub fn current(&self) -> Result<&Version> {
        self.versions.last().ok_or(KnowledgeError::Storage)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Origin {
    pub document_id: i64,
    pub version: u64,
    pub passage_id: usize,
    pub title: String,
    pub hash: String,
    pub start_line: usize,
    pub end_line: usize,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Passage {
    pub id: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub text: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SaveNote {
    pub id: Option<i64>,
    pub expected_version: Option<u64>,
    pub title: String,
    pub content: String,
    pub draft: bool,
}
pub(crate) fn hash(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
pub(crate) fn validate(title: &str, text: &str) -> Result<()> {
    if title.trim().is_empty()
        || title.chars().count() > 120
        || title.chars().any(char::is_control)
        || text.trim().is_empty()
        || text
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    {
        return Err(KnowledgeError::InvalidContent);
    }
    if text.len() > MAX_BYTES {
        return Err(KnowledgeError::Limit);
    }
    Ok(())
}
// Contiguous UTF-8 chunks, max 1024 characters, prefer newlines. IDs are offsets
// in one immutable version, not semantic IDs across edits. No silent truncation.
pub(crate) fn passages(text: &str) -> Vec<Passage> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut line = 1;
    while start < text.len() {
        let tail = &text[start..];
        let cap = tail.char_indices().nth(1024).map_or(tail.len(), |(i, _)| i);
        let end = if cap < tail.len() {
            tail[..cap].rfind('\n').map_or(cap, |i| i + 1)
        } else {
            cap
        };
        let part = &tail[..end];
        let count = part.chars().filter(|c| *c == '\n').count();
        result.push(Passage {
            id: result.len() + 1,
            start_line: line,
            end_line: line + count.saturating_sub(usize::from(part.ends_with('\n'))),
            text: part.into(),
        });
        start += end;
        line += count;
    }
    result
}
pub(crate) fn source(
    item: &Item,
    version: u64,
    passage: usize,
) -> Result<crate::collaboration::Source> {
    let v = item.current()?;
    if v.id != version {
        return Err(KnowledgeError::StaleSelection);
    }
    let p = if passage == 0 {
        Passage {
            id: 0,
            start_line: 1,
            end_line: v.content.lines().count().max(1),
            text: v.content.clone(),
        }
    } else {
        passages(&v.content)
            .into_iter()
            .find(|p| p.id == passage)
            .ok_or(KnowledgeError::StaleSelection)?
    };
    if p.text.chars().count() > 4096 {
        return Err(KnowledgeError::Limit);
    }
    Ok(crate::collaboration::Source {
        label: format!("K{}_V{}_P{}", item.id, v.id, passage),
        text: p.text,
        origin: Some(Origin {
            document_id: item.id,
            version,
            passage_id: passage,
            title: v.title.clone(),
            hash: v.hash.clone(),
            start_line: p.start_line,
            end_line: p.end_line,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn passages_are_lossless_unicode_bounded_and_version_located() {
        let text=format!("---\r\ntags: [incident]\r\n---\r\n# Runbook\n[[Related]] <script>never execute</script>\n{}\nEnd", "界".repeat(2200));
        let parts = passages(&text);
        assert_eq!(
            parts.iter().map(|p| p.text.as_str()).collect::<String>(),
            text
        );
        assert!(parts.iter().all(|p| p.text.chars().count() <= 1024));
        assert!(parts
            .windows(2)
            .all(|p| p[0].id + 1 == p[1].id && p[1].start_line >= p[0].end_line));
        assert_eq!(parts.last().map(|p| p.end_line), Some(7));
        assert!(validate("Unicode 界", &text).is_ok());
        assert_eq!(
            validate("x", "binary\0data"),
            Err(KnowledgeError::InvalidContent)
        );
        assert_eq!(
            validate("x", &"x".repeat(MAX_BYTES + 1)),
            Err(KnowledgeError::Limit)
        );
    }
}
