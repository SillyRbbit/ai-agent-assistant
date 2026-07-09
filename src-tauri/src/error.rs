use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("the Tauri runtime failed: {0}")]
    Tauri(#[from] tauri::Error),
}
