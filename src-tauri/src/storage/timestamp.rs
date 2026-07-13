use std::time::{SystemTime, UNIX_EPOCH};

use super::error::{StorageError, StorageResult};

pub(crate) fn current_unix_time_ms() -> StorageResult<i64> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    i64::try_from(duration.as_millis()).map_err(|_| StorageError::TimestampOutOfRange)
}
