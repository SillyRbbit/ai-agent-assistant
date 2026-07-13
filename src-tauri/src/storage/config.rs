use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use super::error::{StorageError, StorageResult};

pub const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_secs(5);
pub const MAXIMUM_BUSY_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DatabaseLocation {
    InMemory,
    File(PathBuf),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatabaseConfig {
    location: DatabaseLocation,
    busy_timeout: Duration,
}

impl DatabaseConfig {
    #[must_use]
    pub fn in_memory() -> Self {
        Self {
            location: DatabaseLocation::InMemory,
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        }
    }

    pub fn file(path: impl Into<PathBuf>) -> StorageResult<Self> {
        let config = Self {
            location: DatabaseLocation::File(path.into()),
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn with_busy_timeout(mut self, busy_timeout: Duration) -> StorageResult<Self> {
        self.busy_timeout = busy_timeout;
        self.validate()?;
        Ok(self)
    }

    #[must_use]
    pub fn location(&self) -> &DatabaseLocation {
        &self.location
    }

    #[must_use]
    pub fn busy_timeout(&self) -> Duration {
        self.busy_timeout
    }

    pub(crate) fn validate(&self) -> StorageResult<()> {
        if let DatabaseLocation::File(path) = &self.location {
            validate_file_path(path)?;
        }

        if self.busy_timeout.is_zero() {
            return Err(StorageError::ZeroBusyTimeout);
        }

        if self.busy_timeout > MAXIMUM_BUSY_TIMEOUT {
            let maximum_ms = duration_as_millis(MAXIMUM_BUSY_TIMEOUT)?;
            return Err(StorageError::BusyTimeoutTooLarge { maximum_ms });
        }

        Ok(())
    }
}

fn validate_file_path(path: &Path) -> StorageResult<()> {
    if path.as_os_str().is_empty() {
        return Err(StorageError::EmptyDatabasePath);
    }

    Ok(())
}

pub(crate) fn duration_as_millis(duration: Duration) -> StorageResult<u64> {
    u64::try_from(duration.as_millis()).map_err(|_| StorageError::BusyTimeoutOutOfRange)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, time::Duration};

    use super::{DatabaseConfig, DatabaseLocation, DEFAULT_BUSY_TIMEOUT};
    use crate::storage::error::StorageError;

    #[test]
    fn creates_deterministic_default_configurations() {
        let memory = DatabaseConfig::in_memory();
        let file = DatabaseConfig::file("assistant.sqlite");

        assert_eq!(memory.location(), &DatabaseLocation::InMemory);
        assert_eq!(memory.busy_timeout(), DEFAULT_BUSY_TIMEOUT);
        assert!(matches!(
            file,
            Ok(config)
                if config.location()
                    == &DatabaseLocation::File(PathBuf::from("assistant.sqlite"))
        ));
    }

    #[test]
    fn rejects_invalid_configuration() {
        assert!(matches!(
            DatabaseConfig::file(PathBuf::new()),
            Err(StorageError::EmptyDatabasePath)
        ));
        assert!(matches!(
            DatabaseConfig::in_memory().with_busy_timeout(Duration::ZERO),
            Err(StorageError::ZeroBusyTimeout)
        ));
        assert!(matches!(
            DatabaseConfig::in_memory().with_busy_timeout(Duration::from_secs(31)),
            Err(StorageError::BusyTimeoutTooLarge { .. })
        ));
    }
}
