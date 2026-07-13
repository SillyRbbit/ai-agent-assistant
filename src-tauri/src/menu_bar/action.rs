use serde::Serialize;

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const MENU_ROUTE_EVENT: &str = "assistant-menu-route";

const SHOW_MAIN_WINDOW_ID: &str = "menu-bar-show-main-window";
const NEW_REQUEST_ID: &str = "menu-bar-new-request";
const TASKS_PLACEHOLDER_ID: &str = "menu-bar-tasks-placeholder";
const QUIT_ID: &str = "menu-bar-quit";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuBarAction {
    ShowMainWindow,
    NewRequest,
    TasksPlaceholder,
    Quit,
}

impl MenuBarAction {
    pub const ALL: [Self; 4] = [
        Self::ShowMainWindow,
        Self::NewRequest,
        Self::TasksPlaceholder,
        Self::Quit,
    ];

    #[must_use]
    pub const fn menu_id(self) -> &'static str {
        match self {
            Self::ShowMainWindow => SHOW_MAIN_WINDOW_ID,
            Self::NewRequest => NEW_REQUEST_ID,
            Self::TasksPlaceholder => TASKS_PLACEHOLDER_ID,
            Self::Quit => QUIT_ID,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ShowMainWindow => "Open AI Agent Assistant",
            Self::NewRequest => "New Request",
            Self::TasksPlaceholder => "Tasks (Coming Soon)",
            Self::Quit => "Quit AI Agent Assistant",
        }
    }

    #[must_use]
    pub fn from_menu_id(menu_id: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|action| action.menu_id() == menu_id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MenuBarRoute {
    NewRequest,
    TasksPlaceholder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct MenuBarRouteEvent {
    pub route: MenuBarRoute,
}

impl MenuBarRouteEvent {
    #[must_use]
    pub const fn new(route: MenuBarRoute) -> Self {
        Self { route }
    }
}

#[cfg(test)]
mod tests {
    use super::MenuBarAction;

    #[test]
    fn exposes_stable_action_order_ids_and_labels() {
        let action_contract: Vec<(&str, &str)> = MenuBarAction::ALL
            .into_iter()
            .map(|action| (action.menu_id(), action.label()))
            .collect();

        assert_eq!(
            action_contract,
            vec![
                ("menu-bar-show-main-window", "Open AI Agent Assistant"),
                ("menu-bar-new-request", "New Request"),
                ("menu-bar-tasks-placeholder", "Tasks (Coming Soon)"),
                ("menu-bar-quit", "Quit AI Agent Assistant"),
            ]
        );
    }
}
