export const ACTIVITY_PANEL_MIN_HEIGHT = 144;
export const ACTIVITY_PANEL_MAX_HEIGHT = 360;
export const ACTIVITY_PANEL_DEFAULT_HEIGHT = 216;
export const ACTIVITY_PANEL_VIEWPORT_RATIO = 0.42;

export interface ApplicationShellState {
  readonly activityExpanded: boolean;
  readonly activityHeight: number;
  readonly activityMaxHeight: number;
  readonly inspectorExpanded: boolean;
  readonly navigationExpanded: boolean;
}

export type ApplicationShellAction =
  | { readonly type: "activity-resized"; readonly height: number }
  | { readonly type: "activity-closed" }
  | { readonly type: "activity-opened" }
  | { readonly type: "toggle-activity" }
  | { readonly type: "inspector-closed" }
  | { readonly type: "inspector-opened" }
  | { readonly type: "toggle-inspector" }
  | { readonly type: "toggle-navigation" }
  | { readonly type: "viewport-resized"; readonly height: number };

function clampActivityHeight(height: number, maximum: number): number {
  if (!Number.isFinite(height)) {
    return ACTIVITY_PANEL_DEFAULT_HEIGHT;
  }

  return Math.min(Math.max(Math.round(height), ACTIVITY_PANEL_MIN_HEIGHT), maximum);
}

export function getActivityPanelMaxHeight(viewportHeight: number): number {
  if (!Number.isFinite(viewportHeight) || viewportHeight <= 0) {
    return ACTIVITY_PANEL_MAX_HEIGHT;
  }

  return Math.max(
    ACTIVITY_PANEL_MIN_HEIGHT,
    Math.min(ACTIVITY_PANEL_MAX_HEIGHT, Math.floor(viewportHeight * ACTIVITY_PANEL_VIEWPORT_RATIO)),
  );
}

export function createApplicationShellState(viewportHeight: number): ApplicationShellState {
  const activityMaxHeight = getActivityPanelMaxHeight(viewportHeight);

  return {
    activityExpanded: false,
    activityHeight: clampActivityHeight(ACTIVITY_PANEL_DEFAULT_HEIGHT, activityMaxHeight),
    activityMaxHeight,
    inspectorExpanded: false,
    navigationExpanded: true,
  };
}

export function applicationShellReducer(
  state: ApplicationShellState,
  action: ApplicationShellAction,
): ApplicationShellState {
  switch (action.type) {
    case "activity-resized":
      return {
        ...state,
        activityHeight: clampActivityHeight(action.height, state.activityMaxHeight),
      };
    case "activity-closed":
      return state.activityExpanded ? { ...state, activityExpanded: false } : state;
    case "activity-opened":
      return state.activityExpanded ? state : { ...state, activityExpanded: true };
    case "toggle-activity":
      return { ...state, activityExpanded: !state.activityExpanded };
    case "inspector-closed":
      return state.inspectorExpanded ? { ...state, inspectorExpanded: false } : state;
    case "inspector-opened":
      return state.inspectorExpanded ? state : { ...state, inspectorExpanded: true };
    case "toggle-inspector":
      return { ...state, inspectorExpanded: !state.inspectorExpanded };
    case "toggle-navigation":
      return { ...state, navigationExpanded: !state.navigationExpanded };
    case "viewport-resized": {
      const activityMaxHeight = getActivityPanelMaxHeight(action.height);

      return {
        ...state,
        activityHeight: clampActivityHeight(state.activityHeight, activityMaxHeight),
        activityMaxHeight,
      };
    }
  }
}
