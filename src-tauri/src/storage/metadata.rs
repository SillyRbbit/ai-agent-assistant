use rusqlite::{params, Connection, OptionalExtension};

use super::error::{StorageError, StorageResult};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AppMetadataKey {
    AppInitialized,
}

impl AppMetadataKey {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppInitialized => "app_initialized",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppMetadataValue {
    Boolean(bool),
}

impl AppMetadataValue {
    const fn as_database_value(&self) -> &'static str {
        match self {
            Self::Boolean(true) => "true",
            Self::Boolean(false) => "false",
        }
    }

    fn from_database_value(key: AppMetadataKey, value: &str) -> StorageResult<Self> {
        match (key, value) {
            (AppMetadataKey::AppInitialized, "true") => Ok(Self::Boolean(true)),
            (AppMetadataKey::AppInitialized, "false") => Ok(Self::Boolean(false)),
            _ => Err(StorageError::InvalidMetadataValue { key: key.as_str() }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppMetadataRecord {
    key: AppMetadataKey,
    value: AppMetadataValue,
    updated_at_ms: i64,
}

impl AppMetadataRecord {
    #[must_use]
    pub const fn key(&self) -> AppMetadataKey {
        self.key
    }

    #[must_use]
    pub const fn value(&self) -> &AppMetadataValue {
        &self.value
    }

    #[must_use]
    pub const fn updated_at_ms(&self) -> i64 {
        self.updated_at_ms
    }
}

pub(super) fn get(
    connection: &Connection,
    key: AppMetadataKey,
) -> StorageResult<Option<AppMetadataRecord>> {
    let row = connection
        .query_row(
            "SELECT value, updated_at_ms FROM app_metadata WHERE key = ?1",
            params![key.as_str()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
        )
        .optional()
        .map_err(|source| StorageError::ReadMetadata {
            key: key.as_str(),
            source,
        })?;

    row.map(|(value, updated_at_ms)| {
        if updated_at_ms < 0 {
            return Err(StorageError::InvalidMetadataTimestamp);
        }

        Ok(AppMetadataRecord {
            key,
            value: AppMetadataValue::from_database_value(key, &value)?,
            updated_at_ms,
        })
    })
    .transpose()
}

pub(super) fn set(
    connection: &Connection,
    key: AppMetadataKey,
    value: AppMetadataValue,
    updated_at_ms: i64,
) -> StorageResult<AppMetadataRecord> {
    if updated_at_ms < 0 {
        return Err(StorageError::InvalidMetadataTimestamp);
    }

    let changed_rows = connection
        .execute(
            "INSERT INTO app_metadata (key, value, updated_at_ms)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET
               value = excluded.value,
               updated_at_ms = excluded.updated_at_ms",
            params![key.as_str(), value.as_database_value(), updated_at_ms],
        )
        .map_err(|source| StorageError::WriteMetadata {
            key: key.as_str(),
            source,
        })?;

    if changed_rows != 1 {
        return Err(StorageError::MetadataWriteCount {
            key: key.as_str(),
            actual: changed_rows,
        });
    }

    let record = get(connection, key)?
        .ok_or(StorageError::MetadataWriteVerificationFailed { key: key.as_str() })?;

    if record.value != value || record.updated_at_ms != updated_at_ms {
        return Err(StorageError::MetadataWriteVerificationFailed { key: key.as_str() });
    }

    Ok(record)
}
