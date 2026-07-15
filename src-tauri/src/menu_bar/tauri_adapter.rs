#[cfg(target_os = "macos")]
mod macos {
    use tauri::{
        menu::{Menu, MenuItem, PredefinedMenuItem},
        tray::TrayIconBuilder,
        App, AppHandle, Emitter, Manager, RunEvent, Runtime, Window, WindowEvent,
    };

    use super::super::{
        app_reopen_policy, dispatch_menu_id, window_close_policy, AppReopenPolicy, DispatchOutcome,
        MenuBarAction, MenuBarError, MenuBarRoute, MenuBarRouteEvent, MenuBarRuntime,
        WindowClosePolicy, MAIN_WINDOW_LABEL, MENU_ROUTE_EVENT,
    };

    const TRAY_ICON_ID: &str = "ai-agent-assistant-menu-bar";
    const TOOLTIP: &str = "Cortexa";

    pub(crate) fn install<R: Runtime>(app: &mut App<R>) -> Result<(), MenuBarError> {
        let show_main_window = menu_item(app, MenuBarAction::ShowMainWindow)?;
        let separator_after_open = PredefinedMenuItem::separator(app)
            .map_err(|source| MenuBarError::tauri("create first menu separator", source))?;
        let new_request = menu_item(app, MenuBarAction::NewRequest)?;
        let tasks_placeholder = menu_item(app, MenuBarAction::TasksPlaceholder)?;
        let separator_before_quit = PredefinedMenuItem::separator(app)
            .map_err(|source| MenuBarError::tauri("create second menu separator", source))?;
        let quit = menu_item(app, MenuBarAction::Quit)?;

        let menu = Menu::with_items(
            app,
            &[
                &show_main_window,
                &separator_after_open,
                &new_request,
                &tasks_placeholder,
                &separator_before_quit,
                &quit,
            ],
        )
        .map_err(|source| MenuBarError::tauri("build menu-bar menu", source))?;

        let icon = app
            .default_window_icon()
            .cloned()
            .ok_or(MenuBarError::MissingDefaultWindowIcon)?;

        TrayIconBuilder::with_id(TRAY_ICON_ID)
            .icon(icon)
            .icon_as_template(true)
            .tooltip(TOOLTIP)
            .menu(&menu)
            .show_menu_on_left_click(true)
            .on_menu_event(handle_menu_event)
            .build(app)
            .map_err(|source| MenuBarError::tauri("build menu-bar entry", source))?;

        Ok(())
    }

    pub(crate) fn handle_run_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
        let RunEvent::Reopen {
            has_visible_windows,
            ..
        } = event
        else {
            return;
        };

        if app_reopen_policy(*has_visible_windows) != AppReopenPolicy::ShowMainWindow {
            return;
        }

        let runtime = TauriMenuBarRuntime::new(app);
        if let Err(error) = runtime.show_main_window() {
            eprintln!("Cortexa could not reopen the main window: {error}");
        }
    }

    pub(crate) fn handle_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
        if !matches!(event, WindowEvent::CloseRequested { .. }) {
            return;
        }

        if window_close_policy(window.label()) != WindowClosePolicy::HideWindow {
            return;
        }

        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            if let Err(error) = hide_main_window(window) {
                eprintln!("Cortexa could not hide the main window: {error}");
            }
        }
    }

    fn hide_main_window<R: Runtime>(window: &Window<R>) -> Result<(), MenuBarError> {
        window
            .hide()
            .map_err(|source| MenuBarError::tauri("hide main window", source))
    }

    fn menu_item<R: Runtime>(
        app: &App<R>,
        action: MenuBarAction,
    ) -> Result<MenuItem<R>, MenuBarError> {
        MenuItem::with_id(app, action.menu_id(), action.label(), true, None::<&str>)
            .map_err(|source| MenuBarError::tauri("create menu item", source))
    }

    fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: tauri::menu::MenuEvent) {
        let runtime = TauriMenuBarRuntime::new(app);
        match dispatch_menu_id(&runtime, event.id().as_ref()) {
            Ok(DispatchOutcome::Handled(_)) | Ok(DispatchOutcome::IgnoredUnknownMenuItem) => {}
            Err(error) => {
                eprintln!("Cortexa menu-bar action failed: {error}");
            }
        }
    }

    struct TauriMenuBarRuntime<'a, R: Runtime> {
        app: &'a AppHandle<R>,
    }

    impl<'a, R: Runtime> TauriMenuBarRuntime<'a, R> {
        fn new(app: &'a AppHandle<R>) -> Self {
            Self { app }
        }

        fn main_window(&self) -> Result<tauri::WebviewWindow<R>, MenuBarError> {
            self.app
                .get_webview_window(MAIN_WINDOW_LABEL)
                .ok_or(MenuBarError::MainWindowNotFound {
                    label: MAIN_WINDOW_LABEL,
                })
        }
    }

    impl<R: Runtime> MenuBarRuntime for TauriMenuBarRuntime<'_, R> {
        type Error = MenuBarError;

        fn show_main_window(&self) -> Result<(), Self::Error> {
            let window = self.main_window()?;
            self.app
                .show()
                .map_err(|source| MenuBarError::tauri("show application", source))?;
            window
                .unminimize()
                .map_err(|source| MenuBarError::tauri("unminimize main window", source))?;
            window
                .show()
                .map_err(|source| MenuBarError::tauri("show main window", source))?;
            window
                .set_focus()
                .map_err(|source| MenuBarError::tauri("focus main window", source))
        }

        fn emit_route(&self, route: MenuBarRoute) -> Result<(), Self::Error> {
            self.main_window()?
                .emit(MENU_ROUTE_EVENT, MenuBarRouteEvent::new(route))
                .map_err(|source| MenuBarError::tauri("emit menu route", source))
        }

        fn quit(&self) -> Result<(), Self::Error> {
            self.app.exit(0);
            Ok(())
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod other_platforms {
    use tauri::{App, AppHandle, RunEvent, Runtime, Window, WindowEvent};

    use super::super::MenuBarError;

    pub(crate) fn install<R: Runtime>(_: &mut App<R>) -> Result<(), MenuBarError> {
        Ok(())
    }

    pub(crate) fn handle_window_event<R: Runtime>(_: &Window<R>, _: &WindowEvent) {}

    pub(crate) fn handle_run_event<R: Runtime>(_: &AppHandle<R>, _: &RunEvent) {}
}

#[cfg(target_os = "macos")]
pub(crate) use macos::{handle_run_event, handle_window_event, install};
#[cfg(not(target_os = "macos"))]
pub(crate) use other_platforms::{handle_run_event, handle_window_event, install};
