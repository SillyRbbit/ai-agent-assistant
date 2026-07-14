use std::{cell::RefCell, convert::Infallible};

use ai_agent_assistant_lib::menu_bar::{
    app_reopen_policy, dispatch_menu_id, window_close_policy, AppReopenPolicy, DispatchOutcome,
    MenuBarAction, MenuBarRoute, MenuBarRuntime, WindowClosePolicy, MAIN_WINDOW_LABEL,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Call {
    Show,
    Route(MenuBarRoute),
    Quit,
}

#[derive(Default)]
struct RecordingRuntime {
    calls: RefCell<Vec<Call>>,
}

impl RecordingRuntime {
    fn calls(&self) -> Vec<Call> {
        self.calls.borrow().clone()
    }
}

impl MenuBarRuntime for RecordingRuntime {
    type Error = Infallible;

    fn show_main_window(&self) -> Result<(), Self::Error> {
        self.calls.borrow_mut().push(Call::Show);
        Ok(())
    }

    fn emit_route(&self, route: MenuBarRoute) -> Result<(), Self::Error> {
        self.calls.borrow_mut().push(Call::Route(route));
        Ok(())
    }

    fn quit(&self) -> Result<(), Self::Error> {
        self.calls.borrow_mut().push(Call::Quit);
        Ok(())
    }
}

#[test]
fn public_menu_bar_contract_routes_all_registered_actions_deterministically(
) -> Result<(), Infallible> {
    let runtime = RecordingRuntime::default();

    let outcomes = MenuBarAction::ALL
        .into_iter()
        .map(|action| dispatch_menu_id(&runtime, action.menu_id()))
        .collect::<Result<Vec<_>, _>>()?;

    assert_eq!(
        outcomes,
        vec![
            DispatchOutcome::Handled(MenuBarAction::ShowMainWindow),
            DispatchOutcome::Handled(MenuBarAction::NewRequest),
            DispatchOutcome::Handled(MenuBarAction::TasksPlaceholder),
            DispatchOutcome::Handled(MenuBarAction::Quit),
        ]
    );
    assert_eq!(
        runtime.calls(),
        vec![
            Call::Show,
            Call::Show,
            Call::Route(MenuBarRoute::NewRequest),
            Call::Show,
            Call::Route(MenuBarRoute::TasksPlaceholder),
            Call::Quit,
        ]
    );
    Ok(())
}

#[test]
fn public_close_policy_hides_only_the_main_window() {
    assert_eq!(
        window_close_policy(MAIN_WINDOW_LABEL),
        WindowClosePolicy::HideWindow
    );
    assert_eq!(
        window_close_policy("future-secondary-window"),
        WindowClosePolicy::AllowClose
    );
}

#[test]
fn public_reopen_policy_restores_the_main_window_only_when_needed() {
    assert_eq!(app_reopen_policy(false), AppReopenPolicy::ShowMainWindow);
    assert_eq!(app_reopen_policy(true), AppReopenPolicy::KeepCurrentState);
}
