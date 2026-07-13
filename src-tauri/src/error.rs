use std::io;

use thiserror::Error;

use crate::{menu_bar::MenuBarError, storage::StorageError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("the Tauri runtime failed: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("storage initialization failed: {0}")]
    Storage(#[from] StorageError),
    #[error("menu-bar initialization or routing failed: {0}")]
    MenuBar(#[from] MenuBarError),
    #[error("failed to resolve the application-local storage directory: {source}")]
    ResolveStorageDirectory {
        #[source]
        source: tauri::Error,
    },
    #[error("failed to create the application-local storage directory: {source}")]
    CreateStorageDirectory {
        #[source]
        source: io::Error,
    },
    #[error("application storage state was already registered")]
    StorageStateAlreadyManaged,
}
