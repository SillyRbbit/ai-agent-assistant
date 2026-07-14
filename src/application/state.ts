import { destinationForMenuRoute, type AppRoute, type AssistantMenuRoute } from "./navigation";

export interface ApplicationState {
  readonly activeRoute: AppRoute;
  readonly composerDraft: string;
}

export type ApplicationAction =
  | { readonly route: AppRoute; readonly type: "navigate" }
  | { readonly route: AssistantMenuRoute; readonly type: "menu-route-received" }
  | { readonly type: "composer-draft-changed"; readonly value: string };

export const INITIAL_APPLICATION_STATE: ApplicationState = {
  activeRoute: "conversations",
  composerDraft: "",
};

export function applicationReducer(
  state: ApplicationState,
  action: ApplicationAction,
): ApplicationState {
  switch (action.type) {
    case "navigate":
      return action.route === state.activeRoute ? state : { ...state, activeRoute: action.route };
    case "composer-draft-changed":
      return action.value === state.composerDraft
        ? state
        : { ...state, composerDraft: action.value };
    case "menu-route-received": {
      const activeRoute = destinationForMenuRoute(action.route);

      if (action.route === "new_request") {
        return {
          activeRoute,
          composerDraft: "",
        };
      }

      return activeRoute === state.activeRoute ? state : { ...state, activeRoute };
    }
  }
}
