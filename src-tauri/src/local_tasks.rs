//! Owner-operated product tasks. No agent, scheduler, provider or tool authority.
use serde::{Deserialize, Serialize};

pub(crate) const MAX_TASKS: usize = 500;
pub(crate) const MAX_TITLE: usize = 240;
pub(crate) const MAX_NOTES: usize = 8192;
pub(crate) const MAX_SAFE: i64 = 9_007_199_254_740_991;
pub(crate) type Result<T> = std::result::Result<T, TaskError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TaskError {
    #[error("invalid_request")]
    InvalidRequest,
    #[error("limit")]
    Limit,
    #[error("conflict")]
    Conflict,
    #[error("unavailable")]
    Unavailable,
    #[error("storage")]
    Storage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Priority {
    Low,
    Normal,
    High,
}
impl Priority {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LocalTask {
    pub id: i64,
    pub revision: i64,
    pub title: String,
    pub notes: String,
    pub due_date: Option<String>,
    pub priority: Priority,
    pub completed: bool,
    pub created_ms: i64,
    pub updated_ms: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SaveTask {
    pub id: Option<i64>,
    pub expected_revision: Option<i64>,
    pub title: String,
    pub notes: String,
    pub due_date: Option<String>,
    pub priority: Priority,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct TaskIdentity {
    pub id: i64,
    pub expected_revision: i64,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetCompletion {
    pub id: i64,
    pub expected_revision: i64,
    pub completed: bool,
}

pub(crate) fn valid_date(value: &str) -> bool {
    let b = value.as_bytes();
    if b.len() != 10
        || b[4] != b'-'
        || b[7] != b'-'
        || !b
            .iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
    {
        return false;
    }
    let year = value[0..4].parse::<u16>().ok();
    let month = value[5..7].parse::<u8>().ok();
    let day = value[8..10].parse::<u8>().ok();
    let (Some(y), Some(m), Some(d)) = (year, month, day) else {
        return false;
    };
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let days = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };
    y > 0 && d > 0 && d <= days
}

pub(crate) fn validate_identity(id: i64, revision: i64) -> Result<()> {
    if !(1..=MAX_SAFE).contains(&id) || !(1..MAX_SAFE).contains(&revision) {
        return Err(TaskError::InvalidRequest);
    }
    Ok(())
}
impl SaveTask {
    pub(crate) fn validate(&self) -> Result<()> {
        match (self.id, self.expected_revision) {
            (None, None) => {}
            (Some(id), Some(r)) => validate_identity(id, r)?,
            _ => return Err(TaskError::InvalidRequest),
        }
        if self.title.trim().is_empty()
            || self.title.chars().any(char::is_control)
            || self
                .notes
                .chars()
                .any(|c| c.is_control() && c != '\n' && c != '\t')
            || self.due_date.as_deref().is_some_and(|d| !valid_date(d))
        {
            return Err(TaskError::InvalidRequest);
        }
        if self.title.len() > MAX_TITLE || self.notes.len() > MAX_NOTES {
            return Err(TaskError::Limit);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dates_are_gregorian_and_timezone_free() {
        for s in ["2024-02-29", "2000-02-29", "2026-12-31", "0001-01-01"] {
            assert!(valid_date(s), "{s}");
        }
        for s in [
            "2025-02-29",
            "1900-02-29",
            "0000-01-01",
            "2026-04-31",
            "2026-00-01",
            "2026-01-00",
            "2026-1-01",
            "2026-01-01Z",
            "界界界界",
        ] {
            assert!(!valid_date(s), "{s}");
        }
    }
    #[test]
    fn requests_reject_unknown_fields_and_unsafe_identifiers() {
        let raw = r#"{"id":null,"expectedRevision":null,"title":"A","notes":"","dueDate":null,"priority":"normal","execute":true}"#;
        assert!(serde_json::from_str::<SaveTask>(raw).is_err());
        assert!(serde_json::from_str::<TaskIdentity>(
            r#"{"id":1,"expectedRevision":1,"path":"x"}"#
        )
        .is_err());
        assert!(serde_json::from_str::<SetCompletion>(
            r#"{"id":1,"expectedRevision":1,"completed":true,"tool":"x"}"#
        )
        .is_err());
        for (id, r) in [(0, 1), (-1, 1), (1, 0), (MAX_SAFE + 1, 1), (1, MAX_SAFE)] {
            assert_eq!(validate_identity(id, r), Err(TaskError::InvalidRequest));
        }
    }
    #[test]
    fn content_limits_use_utf8_bytes() {
        let mut s = SaveTask {
            id: None,
            expected_revision: None,
            title: "界".repeat(80),
            notes: "\n\t".into(),
            due_date: None,
            priority: Priority::Normal,
        };
        assert_eq!(s.validate(), Ok(()));
        s.title.push('界');
        assert_eq!(s.validate(), Err(TaskError::Limit));
        s.title = "  ".into();
        assert_eq!(s.validate(), Err(TaskError::InvalidRequest));
        s.title = "A\nB".into();
        assert_eq!(s.validate(), Err(TaskError::InvalidRequest));
        s.title = "A".into();
        s.notes = "x".repeat(MAX_NOTES + 1);
        assert_eq!(s.validate(), Err(TaskError::Limit));
        s.notes = "secret\0".into();
        assert_eq!(s.validate(), Err(TaskError::InvalidRequest));
        s.notes.clear();
        s.id = Some(1);
        assert_eq!(s.validate(), Err(TaskError::InvalidRequest));
    }
    #[test]
    fn errors_are_fixed_labels() -> std::result::Result<(), serde_json::Error> {
        assert_eq!(serde_json::to_string(&TaskError::Storage)?, "\"storage\"");
        Ok(())
    }
}
