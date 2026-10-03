//! No request arguments, caller paths, arbitrary reads or provider access.
use crate::diagnostics::{self, Snapshot};
use serde::Serialize;
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DiagnosticError {
    Unavailable,
    #[cfg(target_os = "macos")]
    DestinationExists,
    #[cfg(target_os = "macos")]
    UnsupportedDestination,
    #[cfg(target_os = "macos")]
    ExportFailed,
}
#[tauri::command]
pub(crate) fn read_diagnostics() -> Snapshot {
    diagnostics::logger()
        .map(|l| l.snapshot())
        .unwrap_or(Snapshot {
            available: false,
            events: Vec::new(),
        })
}
#[tauri::command]
pub(crate) async fn export_diagnostics() -> Result<bool, DiagnosticError> {
    #[cfg(target_os = "macos")]
    {
        let log = diagnostics::logger().ok_or(DiagnosticError::Unavailable)?;
        let selected = tauri::async_runtime::spawn_blocking(|| {
            rfd::FileDialog::new()
                .add_filter("Diagnostic JSON", &["json"])
                .set_file_name("cortexa-diagnostics.json")
                .save_file()
        })
        .await
        .map_err(|_| DiagnosticError::Unavailable)?;
        let Some(path) = selected else {
            return Ok(false);
        };
        let snapshot = log.snapshot();
        tauri::async_runtime::spawn_blocking(move || diagnostics::export_new(&path, &snapshot))
            .await
            .map_err(|_| DiagnosticError::ExportFailed)?
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::AlreadyExists => DiagnosticError::DestinationExists,
                std::io::ErrorKind::InvalidInput => DiagnosticError::UnsupportedDestination,
                _ => DiagnosticError::ExportFailed,
            })?;
        Ok(true)
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(DiagnosticError::Unavailable)
    }
}
