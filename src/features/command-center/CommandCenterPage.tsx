import { FlaskConical, PanelRightClose, SearchX } from "lucide-react";
import { useMemo } from "react";
import { createPortal } from "react-dom";

import { useApplicationWorkspacePanels } from "../../components/applicationWorkspacePanels";
import type { ResearchKnowledgeDemoProjectionLoader } from "../../infrastructure/tauri/research-knowledge-demo-projection-client";

import "./command-center.css";
import {
  COMMAND_CENTER_AGENT_IDS,
  COMMAND_CENTER_DISCLOSURE,
  COMMAND_CENTER_GROUP_IDS,
  COMMAND_CENTER_SCENARIO_IDS,
  type CommandCenterAgentId,
  type CommandCenterEvent,
  type CommandCenterProjection,
  type CommandCenterScenarioId,
  type InspectorProjection,
  type TopologyEdge,
  type TopologyEdgeKind,
  type TopologyGroup,
  type TopologyNode,
} from "./commandCenterProjection";
import {
  buildCommandCenterEventPresentations,
  commandCenterEventSeverityLabel,
  COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
  type CommandCenterEventPresentation,
} from "./commandCenterEventPresentation";
import {
  buildCommandCenterProjection,
  COMMAND_CENTER_FIXTURE_CATALOG,
} from "./commandCenterFixtures";
import { buildCommandCenterOverview } from "./commandCenterOverview";
import {
  CommandCenterActivityStream,
  type ActivityViewEvent,
} from "./components/CommandCenterActivityStream";
import { CommandCenterHeader } from "./components/CommandCenterHeader";
import { CommandCenterOverview } from "./components/CommandCenterOverview";
import {
  ContextualInspector,
  WorkspaceContextualInspectorBody,
  WorkspaceContextualInspectorHeader,
  type InspectorViewModel,
  type WorkspaceInspectorSection,
  type WorkspaceInspectorViewModel,
} from "./components/ContextualInspector";
import { OperationalTopologyPanel } from "./components/OperationalTopologyPanel";
import { ResearchKnowledgeDemoProjectionPanel } from "./components/ResearchKnowledgeDemoProjectionPanel";
import { ResearchKnowledgeLifecyclePanel } from "./ResearchKnowledgeLifecyclePanel";
import {
  TopologyStructuredView,
  type StructuredEdgeView,
  type StructuredGroupView,
  type StructuredNodeView,
} from "./components/TopologyStructuredView";
import {
  type CommandCenterPresentationSelection,
  type GraphRelationshipFocus,
  useCommandCenterState,
} from "./useCommandCenterState";

const ACTIVE_STATUSES = new Set([
  "running",
  "delegating",
  "waiting-child",
  "waiting-approval",
  "validating",
  "risk-review",
]);

const STATUS_OPTIONS = [
  "idle",
  "queued",
  "running",
  "delegating",
  "waiting-child",
  "waiting-approval",
  "validating",
  "risk-review",
  "blocked",
  "cancelled",
  "failed",
  "completed",
] as const;

const ENTITY_KIND_OPTIONS = [
  "orchestrator",
  "agent",
  "task",
  "workflow",
  "workflow-step",
  "approval-checkpoint",
  "validation-checkpoint",
  "security-risk-checkpoint",
] as const;

const GRAPH_RELATIONSHIP_KINDS: Readonly<
  Record<Exclude<GraphRelationshipFocus, "all">, ReadonlySet<TopologyEdgeKind>>
> = {
  approvals: new Set(["approval-dependency"]),
  delegation: new Set(["orchestrates", "delegates", "owns-task"]),
  dependencies: new Set(["depends-on"]),
  outcomes: new Set(["result-flow", "cancellation", "failure-propagation"]),
  validation: new Set(["validation-review", "risk-review"]),
};

const GRAPH_RELATIONSHIP_FOCUS_OPTIONS = [
  { id: "all", label: "All existing relationships" },
  { id: "delegation", label: "Delegation and ownership" },
  { id: "dependencies", label: "Dependencies" },
  { id: "approvals", label: "Approvals" },
  { id: "validation", label: "Validation and risk review" },
  { id: "outcomes", label: "Results and lifecycle" },
  { disabled: true, id: "tools-unavailable", label: "Tool usage — unavailable" },
  { disabled: true, id: "memory-unavailable", label: "Memory access — unavailable" },
] as const;

const GRAPH_RELATIONSHIP_FOCUS_IDS: readonly GraphRelationshipFocus[] = [
  "all",
  "delegation",
  "dependencies",
  "approvals",
  "validation",
  "outcomes",
];

function isGraphRelationshipFocus(value: string): value is GraphRelationshipFocus {
  return (GRAPH_RELATIONSHIP_FOCUS_IDS as readonly string[]).includes(value);
}

function readable(value: string): string {
  return value
    .split("-")
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join(" ");
}

function nodeMatches(
  node: TopologyNode,
  groupLabels: ReadonlyMap<string, string>,
  filters: Readonly<{
    agentId: string;
    demoOrigin: string;
    domain: string;
    entityKind: string;
    relatedSearchNodeIds: ReadonlySet<string>;
    search: string;
    status: string;
  }>,
): boolean {
  if (filters.agentId !== "all" && node.agentId !== filters.agentId) return false;
  if (filters.demoOrigin !== "all" && node.demoOrigin !== filters.demoOrigin) return false;
  if (filters.domain !== "all" && node.groupId !== filters.domain) return false;
  if (filters.entityKind !== "all" && node.kind !== filters.entityKind) return false;
  if (filters.status !== "all" && node.status !== filters.status) return false;

  const query = filters.search.trim().toLocaleLowerCase();
  if (query.length === 0) return true;
  const searchable = [
    node.id,
    node.label,
    node.kind,
    node.status,
    node.agentId ?? "",
    node.groupId === null ? "" : (groupLabels.get(node.groupId) ?? ""),
    node.inspector.assignment ?? "",
    ...node.inspector.findings,
    ...node.inspector.unresolvedIssues,
  ]
    .join(" ")
    .toLocaleLowerCase();
  return searchable.includes(query) || filters.relatedSearchNodeIds.has(node.id);
}

function eventMatches(
  event: CommandCenterEvent,
  search: string,
  severity: string,
  kind: string,
  agentLabels: ReadonlyMap<CommandCenterAgentId, string>,
  nodeLabels: ReadonlyMap<string, string>,
): boolean {
  if (severity !== "all" && event.severity !== severity) return false;
  if (kind !== "all" && event.kind !== kind) return false;
  const query = search.trim().toLocaleLowerCase();
  if (query.length === 0) return true;
  return [
    event.id,
    event.summary,
    event.detail,
    event.kind,
    event.demoOrigin,
    event.redaction,
    event.agentId === null ? "application boundary" : (agentLabels.get(event.agentId) ?? ""),
    event.taskNodeId === null ? "" : (nodeLabels.get(event.taskNodeId) ?? event.taskNodeId),
    event.workflowNodeId === null
      ? ""
      : (nodeLabels.get(event.workflowNodeId) ?? event.workflowNodeId),
    ...event.relatedNodeIds.map((id) => nodeLabels.get(id) ?? id),
  ]
    .join(" ")
    .toLocaleLowerCase()
    .includes(query);
}

function recentActivityFor(
  node: TopologyNode,
  events: readonly CommandCenterEvent[],
): readonly string[] {
  return events
    .filter(
      (event) =>
        event.relatedNodeIds.includes(node.id) ||
        event.taskNodeId === node.id ||
        event.workflowNodeId === node.id ||
        (node.agentId !== null && event.agentId === node.agentId),
    )
    .map((event) => `Step ${String(event.ordinal).padStart(2, "0")} · ${event.summary}`)
    .slice(-6);
}

function inspectorFromProjection(
  inspector: InspectorProjection,
  node: TopologyNode,
  domain: string,
  events: readonly CommandCenterEvent[],
  nodeLabels: ReadonlyMap<string, string>,
): InspectorViewModel {
  return {
    approval: node.approval,
    assignment: inspector.assignment ?? "No current assignment in this fixture state.",
    authorityBoundary: inspector.authorityBoundary,
    availability: node.availability,
    demoOrigin: node.demoOrigin,
    dependencies: inspector.dependencyIds.map((id) => nodeLabels.get(id) ?? id),
    domain,
    facts: inspector.facts,
    findings: inspector.findings,
    health: node.health,
    id: node.id,
    inputs: inspector.inputs,
    kind: node.kind,
    label: inspector.title,
    outputs: inspector.outputs,
    recentActivity: recentActivityFor(node, events),
    responsibility: inspector.responsibility,
    status: node.status,
    trust: node.trust,
    unresolved: inspector.unresolvedIssues,
  };
}

function inspectorFromEdge(
  edge: TopologyEdge,
  source: TopologyNode,
  target: TopologyNode,
): InspectorViewModel {
  return {
    approval: "not-required",
    assignment: `${source.label} → ${target.label}`,
    authorityBoundary:
      "This is a read-only fixture relationship. It cannot route or authorize work.",
    availability: "preview-only",
    demoOrigin: edge.demoOrigin,
    dependencies: [source.label],
    domain: "Relationship",
    facts: [
      { label: "Source", value: source.label },
      { label: "Target", value: target.label },
      { label: "Direction", value: `${source.label} → ${target.label}` },
    ],
    findings: [`${readable(edge.kind)} relationship is present in the selected fixture.`],
    health: "not-measured",
    id: edge.id,
    inputs: [`${source.label} · ${readable(source.status)}`],
    kind: "edge",
    label: edge.label,
    outputs: [`${target.label} · ${readable(target.status)}`],
    recentActivity: [],
    responsibility: `Explains the deterministic ${readable(edge.kind)} direction between two presentation entities.`,
    status: edge.status,
    trust: "presentation-fixture",
    unresolved: [],
  };
}

function inspectorFromGroup(
  group: TopologyGroup,
  members: readonly TopologyNode[],
): InspectorViewModel {
  return {
    approval: "not-required",
    assignment: `${String(members.length)} fixed catalog role${members.length === 1 ? "" : "s"}`,
    authorityBoundary:
      "Presentation grouping grants no route, policy, approval, tool, or runtime authority.",
    availability: "preview-only",
    demoOrigin: group.demoOrigin,
    dependencies: [],
    domain: group.label,
    facts: [{ label: "Fixed membership", value: members.map((member) => member.label).join(", ") }],
    findings: members.map((member) => member.label),
    health: "not-measured",
    id: group.id,
    inputs: [],
    kind: "domain-group",
    label: group.label,
    outputs: [],
    recentActivity: [],
    responsibility: group.description,
    status: members.some((member) => ACTIVE_STATUSES.has(member.status)) ? "running" : "idle",
    trust: "presentation-fixture",
    unresolved: [],
  };
}

function nodeEntityType(node: TopologyNode): string {
  switch (node.kind) {
    case "agent":
      return "Canonical agent definition";
    case "orchestrator":
      return "Application orchestrator projection";
    case "task":
      return "Simulated task";
    case "workflow":
      return "Simulated workflow";
    case "workflow-step":
      return "Simulated workflow step";
    case "approval-checkpoint":
      return "Simulated approval checkpoint";
    case "validation-checkpoint":
      return "Simulated validation checkpoint";
    case "security-risk-checkpoint":
      return "Simulated security risk checkpoint";
  }
}

function workspaceInspectorFromNode(
  node: TopologyNode,
  domain: string,
  events: readonly CommandCenterEvent[],
  nodeLabels: ReadonlyMap<string, string>,
): WorkspaceInspectorViewModel {
  const inspector = node.inspector;
  const dependencies = inspector.dependencyIds.map((id) => nodeLabels.get(id) ?? id);
  const relatedActivity = recentActivityFor(node, events);
  const sections: WorkspaceInspectorSection[] = [
    {
      notes: [inspector.responsibility, inspector.authorityBoundary],
      title: "Responsibility and authority",
    },
  ];
  if (inspector.assignment !== null) {
    sections.push({ notes: [inspector.assignment], title: "Current simulated assignment" });
  }
  if (inspector.facts.length > 0) {
    sections.push({ facts: inspector.facts, title: "Fixture facts" });
  }
  if (inspector.inputs.length > 0) {
    sections.push({ items: inspector.inputs, title: "Inputs" });
  }
  if (inspector.outputs.length > 0) {
    sections.push({ items: inspector.outputs, title: "Outputs" });
  }
  if (dependencies.length > 0) {
    sections.push({ items: dependencies, title: "Dependencies" });
  }
  if (inspector.findings.length > 0) {
    sections.push({ items: inspector.findings, title: "Fixture findings" });
  }
  if (inspector.unresolvedIssues.length > 0) {
    sections.push({ items: inspector.unresolvedIssues, title: "Failures and unresolved issues" });
  }
  if (relatedActivity.length > 0) {
    sections.push({ items: relatedActivity, title: "Recent deterministic activity" });
  }

  return {
    description: inspector.description,
    entityType: nodeEntityType(node),
    facts: [
      { label: "Domain", value: domain },
      { label: "Availability", value: readable(node.availability) },
      { label: "Health", value: readable(node.health) },
      { label: "Trust", value: readable(node.trust) },
      { label: "Approval", value: readable(node.approval) },
      { label: "Origin", value: readable(node.demoOrigin) },
    ],
    id: node.id,
    label: inspector.title,
    provenance: "deterministic frontend fixture",
    sections,
    state: { label: node.status, term: "Simulated status" },
  };
}

function workspaceInspectorFromEdge(
  edge: TopologyEdge,
  source: TopologyNode,
  target: TopologyNode,
): WorkspaceInspectorViewModel {
  return {
    description: edge.label,
    entityType: "Fixture relationship",
    facts: [
      { label: "Relationship", value: readable(edge.kind) },
      { label: "Source", value: source.label },
      { label: "Target", value: target.label },
      { label: "Direction", value: `${source.label} → ${target.label}` },
      { label: "Origin", value: readable(edge.demoOrigin) },
    ],
    id: edge.id,
    label: edge.label,
    provenance: "deterministic frontend fixture",
    sections: [
      {
        notes: [
          "This read-only relationship explains fixture topology only. It cannot route or authorize work.",
        ],
        title: "Authority boundary",
      },
    ],
    state: { label: edge.status, term: "Simulated status" },
  };
}

function workspaceInspectorFromGroup(
  group: TopologyGroup,
  members: readonly TopologyNode[],
): WorkspaceInspectorViewModel {
  return {
    description: group.description,
    entityType: "Fixture domain group",
    facts: [
      { label: "Status", value: "Unavailable for presentation groups" },
      { label: "Origin", value: readable(group.demoOrigin) },
      { label: "Fixed membership", value: members.map((member) => member.label).join(", ") },
    ],
    id: group.id,
    label: group.label,
    provenance: "deterministic frontend fixture",
    sections: [
      {
        notes: [
          "Presentation grouping grants no route, policy, approval, tool, or runtime authority.",
        ],
        title: "Authority boundary",
      },
    ],
    state: null,
  };
}

function workspaceInspectorFromEvent(
  event: CommandCenterEventPresentation,
): WorkspaceInspectorViewModel {
  const associatedAgent = event.associatedAgentLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
  const task = event.taskLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
  const workflow = event.workflowLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
  const relatedEntities =
    event.relatedEntityLabels.length === 0
      ? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE
      : event.relatedEntityLabels.join(", ");

  return {
    description: event.description,
    entityType: "Fixture activity event",
    facts: [
      { label: "Simulated time", value: event.simulatedAt },
      { label: "Source", value: event.sourceLabel },
      { label: "Action", value: readable(event.eventKind) },
      { label: "Target", value: event.targetLabel },
      { label: "Severity", value: commandCenterEventSeverityLabel(event.severity) },
      { label: "Status", value: event.statusLabel },
      { label: "Associated agent", value: associatedAgent },
      { label: "Task", value: task },
      { label: "Workflow", value: workflow },
      { label: "Related entities", value: relatedEntities },
      { label: "Origin", value: readable(event.demoOrigin) },
      { label: "Redaction", value: readable(event.redaction) },
    ],
    id: event.eventId,
    label: event.summary,
    provenance: "deterministic frontend fixture · simulated time",
    sections: [
      {
        notes: [
          "This presentation-safe fixture event is not authoritative audit or proof of execution.",
        ],
        title: "Authority boundary",
      },
    ],
    state: { label: event.severity, term: "Severity" },
  };
}

function topologySelectionFor(
  projection: CommandCenterProjection,
  entityId: string,
): CommandCenterPresentationSelection | null {
  const node = projection.nodes.find((candidate) => candidate.id === entityId);
  if (node !== undefined) {
    return {
      entityId: node.id,
      provenance: "deterministic-fixture",
      scenarioId: projection.scenarioId,
      type: "topology-node",
    };
  }
  const edge = projection.edges.find((candidate) => candidate.id === entityId);
  if (edge !== undefined) {
    return {
      entityId: edge.id,
      provenance: "deterministic-fixture",
      scenarioId: projection.scenarioId,
      type: "topology-edge",
    };
  }
  const group = projection.groups.find((candidate) => candidate.id === entityId);
  return group === undefined
    ? null
    : {
        entityId: group.id,
        provenance: "deterministic-fixture",
        scenarioId: projection.scenarioId,
        type: "topology-group",
      };
}

interface CommandCenterPageProps {
  readonly projectionLoader?: ResearchKnowledgeDemoProjectionLoader;
}

const unavailableProjectionLoader: ResearchKnowledgeDemoProjectionLoader = () =>
  Promise.reject(new Error("Projection loader unavailable."));

export default function CommandCenterPage({
  projectionLoader = unavailableProjectionLoader,
}: CommandCenterPageProps) {
  const { actions, state } = useCommandCenterState();
  const workspacePanels = useApplicationWorkspacePanels();
  const projection = useMemo(
    () => buildCommandCenterProjection(state.scenarioId),
    [state.scenarioId],
  );
  const currentSelection =
    state.selection?.scenarioId === projection.scenarioId ? state.selection : null;
  const selectedTopologyId =
    currentSelection === null || currentSelection.type === "fixture-event"
      ? null
      : currentSelection.entityId;
  const selectedEventId =
    currentSelection?.type === "fixture-event" ? currentSelection.entityId : null;
  const captureSelectionFocus = (): HTMLElement | null =>
    document.activeElement instanceof HTMLElement ? document.activeElement : null;
  const selectTopology = (entityId: string | null) => {
    if (entityId === null) {
      actions.select(null);
      return;
    }
    const selection = topologySelectionFor(projection, entityId);
    if (selection === null) return;
    const returnFocus = captureSelectionFocus();
    actions.select(selection);
    if (state.viewMode === "graph") workspacePanels?.openInspector(returnFocus);
  };
  const selectFixtureEvent = (entityId: string) => {
    const event = projection.events.find((candidate) => candidate.id === entityId);
    if (event === undefined) return;
    const returnFocus = captureSelectionFocus();
    actions.select({
      entityId: event.id,
      provenance: "deterministic-fixture",
      scenarioId: projection.scenarioId,
      type: "fixture-event",
    });
    workspacePanels?.openInspector(returnFocus);
  };
  const overview = useMemo(() => buildCommandCenterOverview(projection), [projection]);
  const eventPresentations = useMemo(
    () => buildCommandCenterEventPresentations(projection),
    [projection],
  );
  const eventPresentationById = useMemo(
    () => new Map(eventPresentations.map((event) => [event.eventId, event])),
    [eventPresentations],
  );
  const groupLabels = useMemo(
    () => new Map(projection.groups.map((group) => [group.id, group.label])),
    [projection.groups],
  );
  const nodeLabels = useMemo(
    () => new Map(projection.nodes.map((node) => [node.id, node.label])),
    [projection.nodes],
  );
  const agentLabels = useMemo(
    () =>
      new Map(
        projection.nodes
          .filter((node): node is Extract<TopologyNode, { kind: "agent" }> => node.kind === "agent")
          .map((node) => [node.agentId, node.label]),
      ),
    [projection.nodes],
  );
  const agentNodeIds = useMemo(
    () =>
      new Map(
        projection.nodes
          .filter((node): node is Extract<TopologyNode, { kind: "agent" }> => node.kind === "agent")
          .map((node) => [node.agentId, node.id]),
      ),
    [projection.nodes],
  );

  const primarySearchEventNodeIds = useMemo(() => {
    const ids = new Set<string>();
    if (state.search.trim().length === 0) return ids;
    for (const event of projection.events) {
      if (!eventMatches(event, state.search, "all", "all", agentLabels, nodeLabels)) continue;
      if (event.taskNodeId !== null) ids.add(event.taskNodeId);
      if (event.workflowNodeId !== null) ids.add(event.workflowNodeId);
      if (event.agentId !== null) {
        const agentNodeId = agentNodeIds.get(event.agentId);
        if (agentNodeId !== undefined) ids.add(agentNodeId);
      }
      for (const id of event.relatedNodeIds) ids.add(id);
    }
    return ids;
  }, [agentLabels, agentNodeIds, nodeLabels, projection.events, state.search]);

  const visibleNodes = useMemo(
    () =>
      projection.nodes.filter((node) =>
        nodeMatches(node, groupLabels, {
          agentId: state.agentId,
          demoOrigin: state.demoOrigin,
          domain: state.domain,
          entityKind: state.entityKind,
          relatedSearchNodeIds: primarySearchEventNodeIds,
          search: state.search,
          status: state.status,
        }),
      ),
    [groupLabels, primarySearchEventNodeIds, projection.nodes, state],
  );
  const visibleNodeIds = useMemo(
    () => new Set(visibleNodes.map((node) => node.id)),
    [visibleNodes],
  );
  const visibleEdges = useMemo(
    () =>
      projection.edges.filter(
        (edge) => visibleNodeIds.has(edge.source) && visibleNodeIds.has(edge.target),
      ),
    [projection.edges, visibleNodeIds],
  );
  const graphVisibleEdges = useMemo(() => {
    if (state.graphRelationshipFocus === "all") return visibleEdges;
    const allowedKinds = GRAPH_RELATIONSHIP_KINDS[state.graphRelationshipFocus];
    return visibleEdges.filter((edge) => allowedKinds.has(edge.kind));
  }, [state.graphRelationshipFocus, visibleEdges]);
  const selectedPath = useMemo(() => {
    const agentIds = new Set<CommandCenterAgentId>();
    const nodeIds = new Set<string>();
    if (currentSelection === null) return { agentIds, nodeIds };

    if (currentSelection.type === "fixture-event") {
      const event = projection.events.find(
        (candidate) => candidate.id === currentSelection.entityId,
      );
      if (event === undefined) return { agentIds, nodeIds };
      if (event.agentId !== null) {
        agentIds.add(event.agentId);
        const agentNodeId = agentNodeIds.get(event.agentId);
        if (agentNodeId !== undefined) nodeIds.add(agentNodeId);
      }
      if (event.taskNodeId !== null) nodeIds.add(event.taskNodeId);
      if (event.workflowNodeId !== null) nodeIds.add(event.workflowNodeId);
      for (const id of event.relatedNodeIds) nodeIds.add(id);
    } else if (currentSelection.type === "topology-node") {
      const selectedNode = projection.nodes.find((node) => node.id === currentSelection.entityId);
      if (selectedNode === undefined) return { agentIds, nodeIds };
      nodeIds.add(selectedNode.id);
      if (selectedNode.agentId !== null) agentIds.add(selectedNode.agentId);
      return { agentIds, nodeIds };
    } else if (currentSelection.type === "topology-edge") {
      const selectedEdge = projection.edges.find((edge) => edge.id === currentSelection.entityId);
      if (selectedEdge === undefined) return { agentIds, nodeIds };
      nodeIds.add(selectedEdge.source);
      nodeIds.add(selectedEdge.target);
    } else {
      for (const node of projection.nodes) {
        if (node.groupId !== currentSelection.entityId) continue;
        nodeIds.add(node.id);
        agentIds.add(node.agentId);
      }
    }

    for (const node of projection.nodes) {
      if (!nodeIds.has(node.id) || node.agentId === null) continue;
      agentIds.add(node.agentId);
    }
    return { agentIds, nodeIds };
  }, [agentNodeIds, currentSelection, projection.edges, projection.events, projection.nodes]);
  const filteredEvents = useMemo(
    () =>
      projection.events
        .filter((event) => state.agentId === "all" || event.agentId === state.agentId)
        .filter((event) => state.demoOrigin === "all" || event.demoOrigin === state.demoOrigin)
        .filter(
          (event) =>
            !state.followSelectedPath ||
            currentSelection === null ||
            (event.agentId !== null && selectedPath.agentIds.has(event.agentId)) ||
            (event.taskNodeId !== null && selectedPath.nodeIds.has(event.taskNodeId)) ||
            (event.workflowNodeId !== null && selectedPath.nodeIds.has(event.workflowNodeId)) ||
            event.relatedNodeIds.some((id) => selectedPath.nodeIds.has(id)),
        )
        .filter((event) => eventMatches(event, state.search, "all", "all", agentLabels, nodeLabels))
        .filter((event) =>
          eventMatches(
            event,
            state.activitySearch,
            state.activitySeverity,
            state.activityEventKind,
            agentLabels,
            nodeLabels,
          ),
        ),
    [agentLabels, currentSelection, nodeLabels, projection.events, selectedPath, state],
  );

  const selectedInspector = useMemo(() => {
    if (currentSelection === null || currentSelection.type === "fixture-event") return null;
    if (currentSelection.type === "topology-node") {
      const node = projection.nodes.find((candidate) => candidate.id === currentSelection.entityId);
      if (node === undefined || !visibleNodeIds.has(node.id)) return null;
      return inspectorFromProjection(
        node.inspector,
        node,
        node.groupId === null
          ? "Application boundary"
          : (groupLabels.get(node.groupId) ?? "Unknown"),
        projection.events,
        nodeLabels,
      );
    }
    if (currentSelection.type === "topology-edge") {
      const edge = projection.edges.find((candidate) => candidate.id === currentSelection.entityId);
      const inspectorVisibleEdges = state.viewMode === "graph" ? graphVisibleEdges : visibleEdges;
      if (
        edge === undefined ||
        !inspectorVisibleEdges.some((candidate) => candidate.id === edge.id)
      ) {
        return null;
      }
      const source = projection.nodes.find((candidate) => candidate.id === edge.source);
      const target = projection.nodes.find((candidate) => candidate.id === edge.target);
      return source === undefined || target === undefined
        ? null
        : inspectorFromEdge(edge, source, target);
    }
    const group = projection.groups.find((candidate) => candidate.id === currentSelection.entityId);
    return group === undefined || !visibleNodes.some((candidate) => candidate.groupId === group.id)
      ? null
      : inspectorFromGroup(
          group,
          projection.nodes.filter((candidate) => candidate.groupId === group.id),
        );
  }, [
    currentSelection,
    groupLabels,
    nodeLabels,
    projection,
    graphVisibleEdges,
    state.viewMode,
    visibleEdges,
    visibleNodeIds,
    visibleNodes,
  ]);

  const workspaceInspector = useMemo<WorkspaceInspectorViewModel | null>(() => {
    if (currentSelection === null) return null;
    if (currentSelection.type === "fixture-event") {
      const eventIsVisible = filteredEvents.some(
        (candidate) => candidate.id === currentSelection.entityId,
      );
      const event = eventPresentationById.get(currentSelection.entityId);
      return !eventIsVisible || event === undefined ? null : workspaceInspectorFromEvent(event);
    }
    if (currentSelection.type === "topology-node") {
      const node = projection.nodes.find((candidate) => candidate.id === currentSelection.entityId);
      return node === undefined || !visibleNodeIds.has(node.id)
        ? null
        : workspaceInspectorFromNode(
            node,
            node.groupId === null
              ? "Application boundary"
              : (groupLabels.get(node.groupId) ?? "Unknown"),
            projection.events,
            nodeLabels,
          );
    }
    if (currentSelection.type === "topology-edge") {
      const edge = projection.edges.find((candidate) => candidate.id === currentSelection.entityId);
      if (edge === undefined || !graphVisibleEdges.some((candidate) => candidate.id === edge.id)) {
        return null;
      }
      const source = projection.nodes.find((candidate) => candidate.id === edge.source);
      const target = projection.nodes.find((candidate) => candidate.id === edge.target);
      return source === undefined || target === undefined
        ? null
        : workspaceInspectorFromEdge(edge, source, target);
    }
    const group = projection.groups.find((candidate) => candidate.id === currentSelection.entityId);
    return group === undefined || !visibleNodes.some((node) => node.groupId === group.id)
      ? null
      : workspaceInspectorFromGroup(
          group,
          projection.nodes.filter((node) => node.groupId === group.id),
        );
  }, [
    currentSelection,
    eventPresentationById,
    filteredEvents,
    graphVisibleEdges,
    groupLabels,
    nodeLabels,
    projection,
    visibleNodeIds,
    visibleNodes,
  ]);

  const structuredGroups: readonly StructuredGroupView[] = projection.groups.map((group) => ({
    description: group.description,
    id: group.id,
    label: group.label,
  }));
  const structuredNodes: readonly StructuredNodeView[] = visibleNodes.map((node) => ({
    groupId: node.groupId,
    id: node.id,
    kind: node.kind,
    label: node.label,
    status: node.status,
  }));
  const structuredEdges: readonly StructuredEdgeView[] = visibleEdges.map((edge) => ({
    id: edge.id,
    kind: edge.kind,
    label: edge.label,
    sourceId: edge.source,
    sourceLabel:
      projection.nodes.find((candidate) => candidate.id === edge.source)?.label ?? edge.source,
    status: edge.status,
    targetId: edge.target,
    targetLabel:
      projection.nodes.find((candidate) => candidate.id === edge.target)?.label ?? edge.target,
  }));
  const activityEventKinds = [...new Set(projection.events.map((event) => event.kind))];
  const activitySeverities = [...new Set(projection.events.map((event) => event.severity))];
  const activityEvents: readonly ActivityViewEvent[] = filteredEvents.flatMap((event) => {
    const presentation = eventPresentationById.get(event.id);
    return presentation === undefined ? [] : [presentation];
  });
  const activeNodeId =
    projection.nodes.find((node) => ACTIVE_STATUSES.has(node.status))?.id ?? null;
  const filtersActive =
    state.search.trim().length > 0 ||
    state.domain !== "all" ||
    state.agentId !== "all" ||
    state.status !== "all" ||
    state.entityKind !== "all" ||
    state.demoOrigin !== "all" ||
    state.graphRelationshipFocus !== "all";
  const filterControls = (
    <CommandCenterHeader
      agentId={state.agentId}
      agentOptions={[
        { id: "all", label: "All agents" },
        ...COMMAND_CENTER_AGENT_IDS.map((id) => ({
          id,
          label: agentLabels.get(id) ?? readable(id),
        })),
      ]}
      capabilityUnavailable
      compact={state.viewMode === "graph"}
      demoOrigin={state.demoOrigin}
      demoOriginOptions={[
        { id: "all", label: "All fixture origins" },
        { id: "deterministic-fixture", label: "Deterministic fixture" },
      ]}
      domain={state.domain}
      domainOptions={[
        { id: "all", label: "All domains" },
        ...COMMAND_CENTER_GROUP_IDS.map((id) => ({
          id,
          label: groupLabels.get(id) ?? readable(id),
        })),
      ]}
      entityKind={state.entityKind}
      entityKindOptions={[
        { id: "all", label: "All entity kinds" },
        ...ENTITY_KIND_OPTIONS.map((id) => ({ id, label: readable(id) })),
      ]}
      graphRelationshipFocus={state.graphRelationshipFocus}
      graphRelationshipFocusOptions={GRAPH_RELATIONSHIP_FOCUS_OPTIONS}
      onAgentChange={actions.setAgentId}
      onDemoOriginChange={actions.setDemoOrigin}
      onDomainChange={actions.setDomain}
      onEntityKindChange={actions.setEntityKind}
      onGraphRelationshipFocusChange={(value) => {
        if (isGraphRelationshipFocus(value)) actions.setGraphRelationshipFocus(value);
      }}
      onReset={state.viewMode === "graph" ? actions.clearGraphFilters : actions.reset}
      onScenarioChange={(value) => {
        if (COMMAND_CENTER_SCENARIO_IDS.includes(value as CommandCenterScenarioId)) {
          actions.setScenarioId(value as CommandCenterScenarioId);
        }
      }}
      onSearchChange={actions.setSearch}
      onStatusChange={actions.setStatus}
      onViewModeChange={actions.setViewMode}
      scenarioId={state.scenarioId}
      scenarioOptions={COMMAND_CENTER_FIXTURE_CATALOG.map(({ id, label }) => ({ id, label }))}
      search={state.search}
      status={state.status}
      statusOptions={[
        { id: "all", label: "All statuses" },
        ...STATUS_OPTIONS.map((id) => ({ id, label: readable(id) })),
      ]}
      viewMode={state.viewMode}
    />
  );
  const nativeProofPanels = (
    <>
      {state.scenarioId === "research-knowledge-active" ? (
        <>
          <ResearchKnowledgeDemoProjectionPanel loader={projectionLoader} />
          <ResearchKnowledgeLifecyclePanel />
        </>
      ) : null}
    </>
  );
  const topologyPanel = (
    <OperationalTopologyPanel
      activeNodeId={activeNodeId}
      filtersActive={filtersActive}
      onClearSelection={() => {
        actions.select(null);
      }}
      onResetFilters={state.viewMode === "graph" ? actions.clearGraphFilters : actions.reset}
      onSelect={selectTopology}
      projection={projection}
      selectedId={selectedTopologyId}
      selectionPresent={currentSelection !== null}
      structuredView={
        <TopologyStructuredView
          edges={structuredEdges}
          groups={structuredGroups}
          nodes={structuredNodes}
          onSelect={selectTopology}
          selectedId={selectedTopologyId}
        />
      }
      viewMode={state.viewMode}
      visibleEdges={state.viewMode === "graph" ? graphVisibleEdges : visibleEdges}
      visibleNodes={visibleNodes}
    />
  );
  const activityStream = (
    <CommandCenterActivityStream
      eventKind={state.activityEventKind}
      eventKinds={activityEventKinds}
      events={activityEvents}
      followPathDisabled={currentSelection === null}
      followSelectedPath={state.followSelectedPath}
      onEventKindChange={actions.setActivityEventKind}
      onFollowSelectedPathChange={actions.setFollowSelectedPath}
      onSearchChange={actions.setActivitySearch}
      onSeverityChange={actions.setActivitySeverity}
      search={state.activitySearch}
      severity={state.activitySeverity}
      severities={activitySeverities}
    />
  );
  const workspaceActivityStream = (
    <CommandCenterActivityStream
      eventKind={state.activityEventKind}
      eventKinds={activityEventKinds}
      events={activityEvents}
      followPathDisabled={currentSelection === null}
      followSelectedPath={state.followSelectedPath}
      onEventKindChange={actions.setActivityEventKind}
      onFollowSelectedPathChange={actions.setFollowSelectedPath}
      onSearchChange={actions.setActivitySearch}
      onSelectEvent={selectFixtureEvent}
      onSeverityChange={actions.setActivitySeverity}
      search={state.activitySearch}
      selectedEventId={selectedEventId}
      severities={activitySeverities}
      severity={state.activitySeverity}
      workspace
    />
  );
  const inspectorHeaderPortal =
    workspacePanels?.inspectorHeaderTarget === null ||
    workspacePanels?.inspectorHeaderTarget === undefined
      ? null
      : createPortal(
          state.viewMode === "graph" ? (
            <WorkspaceContextualInspectorHeader
              onClose={workspacePanels.closeInspector}
              selection={workspaceInspector}
              selectionUnavailable={currentSelection !== null && workspaceInspector === null}
            />
          ) : (
            <div className="application-panel-header">
              <div>
                <span className="application-panel-header__eyebrow">Structured view</span>
                <h2 id="application-inspector-title">Inline inspector active</h2>
              </div>
              <button
                aria-label="Close workspace inspector"
                className="application-panel-header__button"
                data-application-inspector-close="true"
                onClick={workspacePanels.closeInspector}
                title="Close workspace inspector"
                type="button"
              >
                <PanelRightClose aria-hidden="true" />
              </button>
            </div>
          ),
          workspacePanels.inspectorHeaderTarget,
        );
  const inspectorBodyPortal =
    workspacePanels?.inspectorBodyTarget === null ||
    workspacePanels?.inspectorBodyTarget === undefined
      ? null
      : createPortal(
          state.viewMode === "graph" ? (
            <WorkspaceContextualInspectorBody
              onClearSelection={() => {
                actions.select(null);
              }}
              selection={workspaceInspector}
              selectionUnavailable={currentSelection !== null && workspaceInspector === null}
            />
          ) : (
            <div className="application-panel-empty">
              <strong>Structured view keeps its inline inspector</strong>
              <p>Inspect the selected fixture entity beside the structured topology.</p>
            </div>
          ),
          workspacePanels.inspectorBodyTarget,
        );
  const activitySummaryPortal =
    workspacePanels?.activitySummaryTarget === null ||
    workspacePanels?.activitySummaryTarget === undefined
      ? null
      : createPortal(
          state.viewMode === "graph" ? (
            <>{activityEvents.length} simulated fixture events</>
          ) : (
            <>Fixture activity remains inline</>
          ),
          workspacePanels.activitySummaryTarget,
        );
  const activityBodyPortal =
    workspacePanels?.activityBodyTarget === null ||
    workspacePanels?.activityBodyTarget === undefined
      ? null
      : createPortal(
          state.viewMode === "graph" ? (
            workspaceActivityStream
          ) : (
            <div className="application-panel-empty application-panel-empty--inline">
              <strong>Structured fixture activity remains in the page</strong>
              <p>The protected Structured composition retains its own bounded activity stream.</p>
            </div>
          ),
          workspacePanels.activityBodyTarget,
        );

  return (
    <>
      <section
        aria-labelledby="command-center-page-title"
        className={`page-stack command-center-page command-center-page--${state.viewMode}`}
      >
        <header className="command-center-page__header">
          <div>
            <p className="section-kicker">Operations · deterministic prototype</p>
            <h1 id="command-center-page-title">Command Center</h1>
            <p>
              Inspect a bounded visual projection of the application-owned agent architecture. The
              topology and activity remain frontend fixtures. Separate Rust panels provide one
              read-only projection and one explicitly stepped volatile synthetic lifecycle. Nothing
              connects to a provider, model, tool executor, durable audit, or device effect.
            </p>
          </div>
          <span className="command-center-demo-badge">
            <FlaskConical aria-hidden="true" /> {COMMAND_CENTER_DISCLOSURE}
          </span>
        </header>

        <p aria-atomic="true" aria-live="polite" className="command-center-provenance">
          <strong>{projection.scenarioLabel}</strong>
          <span>
            Projection {projection.version} · deterministic frontend fixture · separate from native
            proof
          </span>
        </p>

        {state.viewMode === "graph" ? (
          <div className="command-center-graph-shell">
            {filterControls}
            <div className="command-center-graph-workspace">
              <div className="command-center-workbench command-center-workbench--graph">
                {topologyPanel}
              </div>

              <details className="command-center-graph-context">
                <summary>
                  <span>Operational context</span>
                  <span>
                    {overview.activeWork.length} simulated active · {overview.attention.length}{" "}
                    attention · runtime unavailable
                  </span>
                </summary>
                <div className="command-center-graph-context__body">
                  <CommandCenterOverview
                    model={overview}
                    onSelect={selectTopology}
                    onSelectEvent={selectFixtureEvent}
                    selectedEventId={selectedEventId}
                    selectedId={selectedTopologyId}
                  />
                  {nativeProofPanels}
                </div>
              </details>
            </div>
          </div>
        ) : (
          <>
            <CommandCenterOverview
              model={overview}
              onSelect={selectTopology}
              selectedId={selectedTopologyId}
            />
            {filterControls}
            {nativeProofPanels}
            <div className="command-center-workbench">
              {visibleNodes.length === 0 ? (
                <section className="command-center-panel command-center-empty" role="status">
                  <SearchX aria-hidden="true" />
                  <h2>No matching topology entities</h2>
                  <p>The current local filters hide every bounded fixture entity.</p>
                  <button className="command-center-button" onClick={actions.reset} type="button">
                    Reset filters
                  </button>
                </section>
              ) : (
                topologyPanel
              )}
              <ContextualInspector selection={selectedInspector} />
            </div>
            {activityStream}
          </>
        )}
      </section>
      {inspectorHeaderPortal}
      {inspectorBodyPortal}
      {activitySummaryPortal}
      {activityBodyPortal}
    </>
  );
}
