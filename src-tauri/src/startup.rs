use std::{error::Error, fs, path::Path};

use tauri::{App, Manager, Runtime};

use crate::{
    storage::{DatabaseConfig, DatabaseLocation, Storage, StorageInitialization},
    AppError,
};

const DEVELOPMENT_DATABASE_FILE_NAME: &str = "assistant-development.sqlite3";

pub(crate) fn initialize_tauri_app<R: Runtime>(app: &mut App<R>) -> Result<(), Box<dyn Error>> {
    initialize(app).map_err(|error| Box::new(error) as Box<dyn Error>)
}

fn initialize<R: Runtime>(app: &mut App<R>) -> Result<(), AppError> {
    let config = storage_config(app)?;
    let initialization = Storage::initialize(&config)?;
    let summary = startup_summary(&initialization);
    let storage = initialization.into_storage();

    if !app.manage(storage) {
        return Err(AppError::StorageStateAlreadyManaged);
    }

    eprintln!("{summary}");
    Ok(())
}

fn storage_config<R: Runtime>(app: &App<R>) -> Result<DatabaseConfig, AppError> {
    // Editable profiles/notes must survive restart in either build mode. Keep the
    // existing filename for compatibility, always under the app-owned data path.
    let directory = app
        .path()
        .app_local_data_dir()
        .map_err(|source| AppError::ResolveStorageDirectory { source })?;
    development_file_config(&directory)
}

fn development_file_config(directory: &Path) -> Result<DatabaseConfig, AppError> {
    fs::create_dir_all(directory).map_err(|source| AppError::CreateStorageDirectory { source })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(directory, fs::Permissions::from_mode(0o700))
            .map_err(|source| AppError::CreateStorageDirectory { source })?;
    }
    DatabaseConfig::file(directory.join(DEVELOPMENT_DATABASE_FILE_NAME)).map_err(AppError::from)
}

fn startup_summary(initialization: &StorageInitialization) -> String {
    let storage = initialization.storage();
    let mode = match storage.location() {
        DatabaseLocation::InMemory => "ephemeral-memory",
        DatabaseLocation::File(_) => "development-file",
    };

    format!(
        "Cortexa storage initialized: mode={mode}, applied_migrations={}, already_applied_migrations={}, previously_initialized={}",
        storage.startup_migration_report().applied_versions.len(),
        storage
            .startup_migration_report()
            .already_applied_versions
            .len(),
        initialization.was_already_initialized()
    )
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::{development_file_config, startup_summary, DEVELOPMENT_DATABASE_FILE_NAME};
    use crate::storage::{DatabaseConfig, DatabaseLocation, Storage, StorageResult};

    #[test]
    fn creates_a_development_database_configuration_under_the_supplied_directory(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = tempdir()?;
        let nested = root.path().join("nested").join("application-data");
        let config = development_file_config(&nested)?;

        assert_eq!(
            config.location(),
            &DatabaseLocation::File(nested.join(DEVELOPMENT_DATABASE_FILE_NAME))
        );
        assert!(nested.is_dir());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&nested)?.permissions().mode() & 0o777,
                0o700
            );
        }
        Ok(())
    }

    #[test]
    fn startup_summary_contains_only_non_sensitive_storage_state() -> StorageResult<()> {
        let initialization = Storage::initialize(&DatabaseConfig::in_memory())?;
        let summary = startup_summary(&initialization);

        assert!(summary.contains("mode=ephemeral-memory"));
        assert!(summary.contains("applied_migrations=5"));
        assert!(summary.contains("previously_initialized=false"));
        assert!(!summary.contains("sqlite"));
        Ok(())
    }

    #[test]
    fn startup_summary_omits_the_database_path_and_metadata_key() -> StorageResult<()> {
        let directory = tempdir()?;
        let path = directory.path().join(DEVELOPMENT_DATABASE_FILE_NAME);
        let initialization = Storage::initialize(&DatabaseConfig::file(&path)?)?;
        let summary = startup_summary(&initialization);
        let path_text = path.to_string_lossy();

        assert!(summary.contains("mode=development-file"));
        assert!(!summary.contains(path_text.as_ref()));
        assert!(!summary.contains("app_initialized"));
        Ok(())
    }
}
