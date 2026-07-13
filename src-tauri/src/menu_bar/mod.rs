mod action;
mod controller;
mod tauri_adapter;

use std::error::Error;

use tauri::{App, AppHandle, RunEvent, Runtime, Window, WindowEvent};
use thiserror::Error;

use crate::AppError;

pub use action::{
    MenuBarAction, MenuBarRoute, MenuBarRouteEvent, MAIN_WINDOW_LABEL, MENU_ROUTE_EVENT,
};
pub use controller::{
    app_reopen_policy, dispatch_action, dispatch_menu_id, window_close_policy, AppReopenPolicy,
    DispatchOutcome, MenuBarRuntime, WindowClosePolicy,
};

#[derive(Debug, Error)]
pub enum MenuBarError {
    #[error("the default application icon is unavailable for the menu-bar entry")]
    MissingDefaultWindowIcon,
    #[error("the configured main window `{label}` was not found")]
    MainWindowNotFound { label: &'static str },
    #[error("menu-bar operation `{operation}` failed: {source}")]
    Tauri {
        operation: &'static str,
        #[source]
        source: tauri::Error,
    },
}

#[cfg(target_os = "macos")]
impl MenuBarError {
    pub(crate) fn tauri(operation: &'static str, source: tauri::Error) -> Self {
        Self::Tauri { operation, source }
    }
}

pub(crate) fn initialize_tauri_app<R: Runtime>(
    app: &mut App<R>,
) -> Result<(), Box<dyn Error>> {
    tauri_adapter::install(app)
        .map_err(AppError::from)
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}

pub(crate) fn handle_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    tauri_adapter::handle_window_event(window, event);
}

pub(crate) fn handle_run_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
    tauri_adapter::handle_run_event(app, event);
}
