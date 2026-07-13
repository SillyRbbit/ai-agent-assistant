pub mod agent;
mod app_info;
pub mod approvals;
pub mod audit;
mod error;
pub mod memory;
pub mod menu_bar;
pub mod platform;
pub mod policy;
mod startup;
pub mod storage;
pub mod tools;

pub use app_info::{current_app_info, AppInfo};
pub use error::AppError;
pub use tools::types::{PermissionKind, RiskClass};

pub fn run() -> Result<(), AppError> {
    let app = tauri::Builder::default()
        .setup(|app| {
            startup::initialize_tauri_app(app)?;
            menu_bar::initialize_tauri_app(app)?;
            Ok(())
        })
        .on_window_event(menu_bar::handle_window_event)
        .invoke_handler(tauri::generate_handler![app_info::get_app_info])
        .build(tauri::generate_context!())
        .map_err(AppError::from)?;

    app.run(|app_handle, event| menu_bar::handle_run_event(app_handle, &event));
    Ok(())
}
