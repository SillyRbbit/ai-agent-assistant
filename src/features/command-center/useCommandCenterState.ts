import { useReducer } from "react";

import type { CommandCenterScenarioId } from "./commandCenterProjection";
import { DEFAULT_COMMAND_CENTER_SCENARIO_ID } from "./commandCenterFixtures";

export interface CommandCenterViewState {
  readonly activityEventKind: string;
  readonly activitySearch: string;
  readonly activitySeverity: string;
  readonly agentId: string;
  readonly demoOrigin: string;
  readonly domain: string;
  readonly entityKind: string;
  readonly followSelectedPath: boolean;
  readonly scenarioId: CommandCenterScenarioId;
  readonly search: string;
  readonly selectedId: string | null;
  readonly status: string;
  readonly viewMode: "graph" | "structured";
}

type CommandCenterViewAction =
  | Readonly<{ type: "activity-event-kind-changed"; value: string }>
  | Readonly<{ type: "activity-search-changed"; value: string }>
  | Readonly<{ type: "activity-severity-changed"; value: string }>
  | Readonly<{ type: "agent-changed"; value: string }>
  | Readonly<{ type: "demo-origin-changed"; value: string }>
  | Readonly<{ type: "domain-changed"; value: string }>
  | Readonly<{ type: "entity-kind-changed"; value: string }>
  | Readonly<{ type: "follow-selected-path-changed"; value: boolean }>
  | Readonly<{ type: "reset" }>
  | Readonly<{ type: "scenario-changed"; value: CommandCenterScenarioId }>
  | Readonly<{ type: "search-changed"; value: string }>
  | Readonly<{ type: "selection-changed"; value: string | null }>
  | Readonly<{ type: "status-changed"; value: string }>
  | Readonly<{ type: "view-mode-changed"; value: "graph" | "structured" }>;

export const INITIAL_COMMAND_CENTER_VIEW_STATE: CommandCenterViewState = Object.freeze({
  activityEventKind: "all",
  activitySearch: "",
  activitySeverity: "all",
  agentId: "all",
  demoOrigin: "all",
  domain: "all",
  entityKind: "all",
  followSelectedPath: false,
  scenarioId: DEFAULT_COMMAND_CENTER_SCENARIO_ID,
  search: "",
  selectedId: "demo-node:orchestrator",
  status: "all",
  viewMode: "graph",
});

function commandCenterViewReducer(
  state: CommandCenterViewState,
  action: CommandCenterViewAction,
): CommandCenterViewState {
  switch (action.type) {
    case "activity-event-kind-changed":
      return { ...state, activityEventKind: action.value };
    case "activity-search-changed":
      return { ...state, activitySearch: action.value };
    case "activity-severity-changed":
      return { ...state, activitySeverity: action.value };
    case "agent-changed":
      return { ...state, agentId: action.value };
    case "demo-origin-changed":
      return { ...state, demoOrigin: action.value };
    case "domain-changed":
      return { ...state, domain: action.value };
    case "entity-kind-changed":
      return { ...state, entityKind: action.value };
    case "follow-selected-path-changed":
      return { ...state, followSelectedPath: action.value };
    case "reset":
      return { ...INITIAL_COMMAND_CENTER_VIEW_STATE, viewMode: state.viewMode };
    case "scenario-changed":
      return {
        ...INITIAL_COMMAND_CENTER_VIEW_STATE,
        scenarioId: action.value,
        viewMode: state.viewMode,
      };
    case "search-changed":
      return { ...state, search: action.value };
    case "selection-changed":
      return { ...state, selectedId: action.value };
    case "status-changed":
      return { ...state, status: action.value };
    case "view-mode-changed":
      return { ...state, viewMode: action.value };
  }
}

function initializeCommandCenterViewState(
  initialState: CommandCenterViewState,
): CommandCenterViewState {
  if (typeof window.matchMedia === "function" && window.matchMedia("(max-width: 640px)").matches) {
    return { ...initialState, viewMode: "structured" };
  }
  return initialState;
}

export function useCommandCenterState() {
  const [state, dispatch] = useReducer(
    commandCenterViewReducer,
    INITIAL_COMMAND_CENTER_VIEW_STATE,
    initializeCommandCenterViewState,
  );

  return {
    actions: {
      reset: () => {
        dispatch({ type: "reset" });
      },
      select: (value: string | null) => {
        dispatch({ type: "selection-changed", value });
      },
      setActivityEventKind: (value: string) => {
        dispatch({ type: "activity-event-kind-changed", value });
      },
      setActivitySearch: (value: string) => {
        dispatch({ type: "activity-search-changed", value });
      },
      setActivitySeverity: (value: string) => {
        dispatch({ type: "activity-severity-changed", value });
      },
      setAgentId: (value: string) => {
        dispatch({ type: "agent-changed", value });
      },
      setDemoOrigin: (value: string) => {
        dispatch({ type: "demo-origin-changed", value });
      },
      setDomain: (value: string) => {
        dispatch({ type: "domain-changed", value });
      },
      setEntityKind: (value: string) => {
        dispatch({ type: "entity-kind-changed", value });
      },
      setFollowSelectedPath: (value: boolean) => {
        dispatch({ type: "follow-selected-path-changed", value });
      },
      setScenarioId: (value: CommandCenterScenarioId) => {
        dispatch({ type: "scenario-changed", value });
      },
      setSearch: (value: string) => {
        dispatch({ type: "search-changed", value });
      },
      setStatus: (value: string) => {
        dispatch({ type: "status-changed", value });
      },
      setViewMode: (value: "graph" | "structured") => {
        dispatch({ type: "view-mode-changed", value });
      },
    },
    state,
  } as const;
}
