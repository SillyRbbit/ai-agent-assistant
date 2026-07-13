use super::action::{MenuBarAction, MenuBarRoute, MAIN_WINDOW_LABEL};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchOutcome {
    Handled(MenuBarAction),
    IgnoredUnknownMenuItem,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowClosePolicy {
    HideWindow,
    AllowClose,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppReopenPolicy {
    ShowMainWindow,
    KeepCurrentState,
}

pub trait MenuBarRuntime {
    type Error;

    fn show_main_window(&self) -> Result<(), Self::Error>;
    fn emit_route(&self, route: MenuBarRoute) -> Result<(), Self::Error>;
    fn quit(&self) -> Result<(), Self::Error>;
}

pub fn dispatch_menu_id<R: MenuBarRuntime>(
    runtime: &R,
    menu_id: &str,
) -> Result<DispatchOutcome, R::Error> {
    let Some(action) = MenuBarAction::from_menu_id(menu_id) else {
        return Ok(DispatchOutcome::IgnoredUnknownMenuItem);
    };

    dispatch_action(runtime, action)?;
    Ok(DispatchOutcome::Handled(action))
}

pub fn dispatch_action<R: MenuBarRuntime>(
    runtime: &R,
    action: MenuBarAction,
) -> Result<(), R::Error> {
    match action {
        MenuBarAction::ShowMainWindow => runtime.show_main_window(),
        MenuBarAction::NewRequest => {
            runtime.show_main_window()?;
            runtime.emit_route(MenuBarRoute::NewRequest)
        }
        MenuBarAction::TasksPlaceholder => {
            runtime.show_main_window()?;
            runtime.emit_route(MenuBarRoute::TasksPlaceholder)
        }
        MenuBarAction::Quit => runtime.quit(),
    }
}

#[must_use]
pub fn window_close_policy(window_label: &str) -> WindowClosePolicy {
    if window_label == MAIN_WINDOW_LABEL {
        WindowClosePolicy::HideWindow
    } else {
        WindowClosePolicy::AllowClose
    }
}

#[must_use]
pub const fn app_reopen_policy(has_visible_windows: bool) -> AppReopenPolicy {
    if has_visible_windows {
        AppReopenPolicy::KeepCurrentState
    } else {
        AppReopenPolicy::ShowMainWindow
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error, fmt};

    use super::{
        app_reopen_policy, dispatch_menu_id, window_close_policy, AppReopenPolicy,
        DispatchOutcome, MenuBarRuntime, WindowClosePolicy,
    };
    use crate::menu_bar::{MenuBarAction, MenuBarRoute, MAIN_WINDOW_LABEL};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum RuntimeCall {
        ShowMainWindow,
        EmitRoute(MenuBarRoute),
        Quit,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct MockRuntimeError;

    impl fmt::Display for MockRuntimeError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("mock menu-bar runtime failure")
        }
    }

    impl Error for MockRuntimeError {}

    #[derive(Default)]
    struct RecordingRuntime {
        calls: RefCell<Vec<RuntimeCall>>,
        fail_on: Option<RuntimeCall>,
    }

    impl RecordingRuntime {
        fn failing_on(call: RuntimeCall) -> Self {
            Self {
                calls: RefCell::new(Vec::new()),
                fail_on: Some(call),
            }
        }

        fn record(&self, call: RuntimeCall) -> Result<(), MockRuntimeError> {
            self.calls.borrow_mut().push(call);
            if self.fail_on == Some(call) {
                Err(MockRuntimeError)
            } else {
                Ok(())
            }
        }

        fn calls(&self) -> Vec<RuntimeCall> {
            self.calls.borrow().clone()
        }
    }

    impl MenuBarRuntime for RecordingRuntime {
        type Error = MockRuntimeError;

        fn show_main_window(&self) -> Result<(), Self::Error> {
            self.record(RuntimeCall::ShowMainWindow)
        }

        fn emit_route(&self, route: MenuBarRoute) -> Result<(), Self::Error> {
            self.record(RuntimeCall::EmitRoute(route))
        }

        fn quit(&self) -> Result<(), Self::Error> {
            self.record(RuntimeCall::Quit)
        }
    }

    #[test]
    fn routes_new_request_after_showing_the_main_window() -> Result<(), MockRuntimeError> {
        let runtime = RecordingRuntime::default();

        let outcome = dispatch_menu_id(&runtime, MenuBarAction::NewRequest.menu_id())?;

        assert_eq!(outcome, DispatchOutcome::Handled(MenuBarAction::NewRequest));
        assert_eq!(
            runtime.calls(),
            vec![
                RuntimeCall::ShowMainWindow,
                RuntimeCall::EmitRoute(MenuBarRoute::NewRequest)
            ]
        );
        Ok(())
    }

    #[test]
    fn routes_tasks_placeholder_after_showing_the_main_window(
    ) -> Result<(), MockRuntimeError> {
        let runtime = RecordingRuntime::default();

        let outcome = dispatch_menu_id(&runtime, MenuBarAction::TasksPlaceholder.menu_id())?;

        assert_eq!(
            outcome,
            DispatchOutcome::Handled(MenuBarAction::TasksPlaceholder)
        );
        assert_eq!(
            runtime.calls(),
            vec![
                RuntimeCall::ShowMainWindow,
                RuntimeCall::EmitRoute(MenuBarRoute::TasksPlaceholder)
            ]
        );
        Ok(())
    }

    #[test]
    fn quits_without_showing_or_routing() -> Result<(), MockRuntimeError> {
        let runtime = RecordingRuntime::default();

        let outcome = dispatch_menu_id(&runtime, MenuBarAction::Quit.menu_id())?;

        assert_eq!(outcome, DispatchOutcome::Handled(MenuBarAction::Quit));
        assert_eq!(runtime.calls(), vec![RuntimeCall::Quit]);
        Ok(())
    }

    #[test]
    fn ignores_unknown_menu_items() -> Result<(), MockRuntimeError> {
        let runtime = RecordingRuntime::default();

        let outcome = dispatch_menu_id(&runtime, "not-registered")?;

        assert_eq!(outcome, DispatchOutcome::IgnoredUnknownMenuItem);
        assert!(runtime.calls().is_empty());
        Ok(())
    }

    #[test]
    fn stops_before_emitting_a_route_when_window_activation_fails() {
        let runtime = RecordingRuntime::failing_on(RuntimeCall::ShowMainWindow);

        assert_eq!(
            dispatch_menu_id(&runtime, MenuBarAction::NewRequest.menu_id()),
            Err(MockRuntimeError)
        );
        assert_eq!(runtime.calls(), vec![RuntimeCall::ShowMainWindow]);
    }

    #[test]
    fn returns_route_failure_after_showing_the_window() {
        let runtime =
            RecordingRuntime::failing_on(RuntimeCall::EmitRoute(MenuBarRoute::NewRequest));

        assert_eq!(
            dispatch_menu_id(&runtime, MenuBarAction::NewRequest.menu_id()),
            Err(MockRuntimeError)
        );
        assert_eq!(
            runtime.calls(),
            vec![
                RuntimeCall::ShowMainWindow,
                RuntimeCall::EmitRoute(MenuBarRoute::NewRequest)
            ]
        );
    }

    #[test]
    fn reopens_the_main_window_only_when_no_window_is_visible() {
        assert_eq!(
            app_reopen_policy(false),
            AppReopenPolicy::ShowMainWindow
        );
        assert_eq!(
            app_reopen_policy(true),
            AppReopenPolicy::KeepCurrentState
        );
    }

    #[test]
    fn hides_only_the_configured_main_window_on_close() {
        assert_eq!(
            window_close_policy(MAIN_WINDOW_LABEL),
            WindowClosePolicy::HideWindow
        );
        assert_eq!(
            window_close_policy("secondary"),
            WindowClosePolicy::AllowClose
        );
    }
}
