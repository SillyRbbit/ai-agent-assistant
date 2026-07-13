pub mod agent;
mod app_info;
pub mod approvals;
pub mod audit;
mod error;
pub mod memory;
pub mod platform;
pub mod policy;
pub mod storage;
pub mod tools;

pub use app_info::{current_app_info, AppInfo};
pub use error::AppError;
pub use tools::types::{PermissionKind, RiskClass};

pub fn run() -> Result<(), AppError> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_info::get_app_info])
        .run(tauri::generate_context!())
        .map_err(AppError::from)
}
