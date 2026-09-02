import { useReducer } from "react";

import type {
  CommandCenterEventId,
  CommandCenterScenarioId,
  TopologyEdgeId,
  TopologyGroupId,
  TopologyNodeId,
} from "./commandCenterProjection";
import { DEFAULT_COMMAND_CENTER_SCENARIO_ID } from "./commandCenterFixtures";

export type GraphRelationshipFocus =
  | "all"
  | "approvals"
  | "delegation"
  | "dependencies"
  | "outcomes"
  | "validation";

const COMMAND_CENTER_SELECTION_PROVENANCE = "deterministic-fixture" as const;

export type CommandCenterPresentationSelection =
  | Readonly<{
      entityId: TopologyNodeId;
      provenance: typeof COMMAND_CENTER_SELECTION_PROVENANCE;
      scenarioId: CommandCenterScenarioId;
      type: "topology-node";
    }>
  | Readonly<{
      entityId: TopologyEdgeId;
      provenance: typeof COMMAND_CENTER_SELECTION_PROVENANCE;
      scenarioId: CommandCenterScenarioId;
      type: "topology-edge";
    }>
  | Readonly<{
      entityId: TopologyGroupId;
      provenance: typeof COMMAND_CENTER_SELECTION_PROVENANCE;
      scenarioId: CommandCenterScenarioId;
      type: "topology-group";
    }>
  | Readonly<{
      entityId: CommandCenterEventId;
      provenance: typeof COMMAND_CENTER_SELECTION_PROVENANCE;
      scenarioId: CommandCenterScenarioId;
      type: "fixture-event";
    }>;

export interface CommandCenterViewState {
  readonly activityEventKind: string;
  readonly activitySearch: string;
  readonly activitySeverity: string;
  readonly agentId: string;
  readonly demoOrigin: string;
  readonly domain: string;
  readonly entityKind: string;
  readonly followSelectedPath: boolean;
  readonly graphRelationshipFocus: GraphRelationshipFocus;
  readonly scenarioId: CommandCenterScenarioId;
  readonly search: string;
  readonly selection: CommandCenterPresentationSelection | null;
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
  | Readonly<{ type: "graph-filters-cleared" }>
  | Readonly<{ type: "graph-relationship-focus-changed"; value: GraphRelationshipFocus }>
  | Readonly<{ type: "reset" }>
  | Readonly<{ type: "scenario-changed"; value: CommandCenterScenarioId }>
  | Readonly<{ type: "search-changed"; value: string }>
  | Readonly<{ type: "selection-changed"; value: CommandCenterPresentationSelection | null }>
  | Readonly<{ type: "status-changed"; value: string }>
  | Readonly<{ type: "view-mode-changed"; value: "graph" | "structured" }>;

function initialSelectionForScenario(
  scenarioId: CommandCenterScenarioId,
): CommandCenterPresentationSelection {
  return {
    entityId: "demo-node:orchestrator",
    provenance: COMMAND_CENTER_SELECTION_PROVENANCE,
    scenarioId,
    type: "topology-node",
  };
}

export const INITIAL_COMMAND_CENTER_VIEW_STATE: CommandCenterViewState = Object.freeze({
  activityEventKind: "all",
  activitySearch: "",
  activitySeverity: "all",
  agentId: "all",
  demoOrigin: "all",
  domain: "all",
  entityKind: "all",
  followSelectedPath: false,
  graphRelationshipFocus: "all",
  scenarioId: DEFAULT_COMMAND_CENTER_SCENARIO_ID,
  search: "",
  selection: initialSelectionForScenario(DEFAULT_COMMAND_CENTER_SCENARIO_ID),
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
    case "graph-filters-cleared":
      return {
        ...state,
        agentId: "all",
        demoOrigin: "all",
        domain: "all",
        entityKind: "all",
        graphRelationshipFocus: "all",
        search: "",
        status: "all",
      };
    case "graph-relationship-focus-changed":
      return { ...state, graphRelationshipFocus: action.value };
    case "reset":
      return { ...INITIAL_COMMAND_CENTER_VIEW_STATE, viewMode: state.viewMode };
    case "scenario-changed":
      return {
        ...INITIAL_COMMAND_CENTER_VIEW_STATE,
        scenarioId: action.value,
        selection: initialSelectionForScenario(action.value),
        viewMode: state.viewMode,
      };
    case "search-changed":
      return { ...state, search: action.value };
    case "selection-changed":
      return action.value !== null && action.value.scenarioId !== state.scenarioId
        ? state
        : { ...state, selection: action.value };
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
      clearGraphFilters: () => {
        dispatch({ type: "graph-filters-cleared" });
      },
      reset: () => {
        dispatch({ type: "reset" });
      },
      select: (value: CommandCenterPresentationSelection | null) => {
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
      setGraphRelationshipFocus: (value: GraphRelationshipFocus) => {
        dispatch({ type: "graph-relationship-focus-changed", value });
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
