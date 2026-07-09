mod app_info;
mod error;

pub use app_info::{current_app_info, AppInfo};
pub use error::AppError;

pub fn run() -> Result<(), AppError> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_info::get_app_info])
        .run(tauri::generate_context!())
        .map_err(AppError::from)
}
