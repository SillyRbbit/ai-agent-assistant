import { FlaskConical, SearchX } from "lucide-react";
import { useMemo } from "react";

import type { ResearchKnowledgeDemoProjectionLoader } from "../../infrastructure/tauri/research-knowledge-demo-projection-client";

import "./command-center.css";
import {
  COMMAND_CENTER_AGENT_IDS,
  COMMAND_CENTER_DISCLOSURE,
  COMMAND_CENTER_GROUP_IDS,
  COMMAND_CENTER_SCENARIO_IDS,
  type CommandCenterAgentId,
  type CommandCenterEvent,
  type CommandCenterScenarioId,
  type InspectorProjection,
  type TopologyEdge,
  type TopologyGroup,
  type TopologyNode,
} from "./commandCenterProjection";
import {
  buildCommandCenterProjection,
  COMMAND_CENTER_FIXTURE_CATALOG,
} from "./commandCenterFixtures";
import {
  CommandCenterActivityStream,
  type ActivityViewEvent,
} from "./components/CommandCenterActivityStream";
import { CommandCenterHeader } from "./components/CommandCenterHeader";
import { ContextualInspector, type InspectorViewModel } from "./components/ContextualInspector";
import { OperationalTopologyPanel } from "./components/OperationalTopologyPanel";
import { SystemStatusSummary } from "./components/SystemStatusSummary";
import { ResearchKnowledgeDemoProjectionPanel } from "./components/ResearchKnowledgeDemoProjectionPanel";
import {
  TopologyStructuredView,
  type StructuredEdgeView,
  type StructuredGroupView,
  type StructuredNodeView,
} from "./components/TopologyStructuredView";
import { useCommandCenterState } from "./useCommandCenterState";

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

interface CommandCenterPageProps {
  readonly projectionLoader?: ResearchKnowledgeDemoProjectionLoader;
}

const unavailableProjectionLoader: ResearchKnowledgeDemoProjectionLoader = () =>
  Promise.reject(new Error("Projection loader unavailable."));

export default function CommandCenterPage({
  projectionLoader = unavailableProjectionLoader,
}: CommandCenterPageProps) {
  const { actions, state } = useCommandCenterState();
  const projection = useMemo(
    () => buildCommandCenterProjection(state.scenarioId),
    [state.scenarioId],
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
  const selectedPath = useMemo(() => {
    const agentIds = new Set<CommandCenterAgentId>();
    const nodeIds = new Set<string>();
    if (state.selectedId === null) return { agentIds, nodeIds };

    const selectedNode = projection.nodes.find((node) => node.id === state.selectedId);
    if (selectedNode !== undefined) {
      nodeIds.add(selectedNode.id);
      if (selectedNode.agentId !== null) agentIds.add(selectedNode.agentId);
      return { agentIds, nodeIds };
    }

    const selectedEdge = projection.edges.find((edge) => edge.id === state.selectedId);
    if (selectedEdge !== undefined) {
      nodeIds.add(selectedEdge.source);
      nodeIds.add(selectedEdge.target);
    } else {
      for (const node of projection.nodes) {
        if (node.groupId !== state.selectedId) continue;
        nodeIds.add(node.id);
        agentIds.add(node.agentId);
      }
    }

    for (const node of projection.nodes) {
      if (!nodeIds.has(node.id) || node.agentId === null) continue;
      agentIds.add(node.agentId);
    }
    return { agentIds, nodeIds };
  }, [projection.edges, projection.nodes, state.selectedId]);
  const filteredEvents = useMemo(
    () =>
      projection.events
        .filter((event) => state.agentId === "all" || event.agentId === state.agentId)
        .filter((event) => state.demoOrigin === "all" || event.demoOrigin === state.demoOrigin)
        .filter(
          (event) =>
            !state.followSelectedPath ||
            state.selectedId === null ||
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
    [agentLabels, nodeLabels, projection.events, selectedPath, state],
  );

  const selectedInspector = useMemo(() => {
    if (state.selectedId === null) return null;
    const node = projection.nodes.find((candidate) => candidate.id === state.selectedId);
    if (node !== undefined && visibleNodeIds.has(node.id)) {
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
    const edge = projection.edges.find((candidate) => candidate.id === state.selectedId);
    if (edge !== undefined && visibleEdges.some((candidate) => candidate.id === edge.id)) {
      const source = projection.nodes.find((candidate) => candidate.id === edge.source);
      const target = projection.nodes.find((candidate) => candidate.id === edge.target);
      if (source !== undefined && target !== undefined)
        return inspectorFromEdge(edge, source, target);
    }
    const group = projection.groups.find((candidate) => candidate.id === state.selectedId);
    return group === undefined || !visibleNodes.some((candidate) => candidate.groupId === group.id)
      ? null
      : inspectorFromGroup(
          group,
          projection.nodes.filter((candidate) => candidate.groupId === group.id),
        );
  }, [
    groupLabels,
    nodeLabels,
    projection,
    state.selectedId,
    visibleEdges,
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
  const activityEvents: readonly ActivityViewEvent[] = filteredEvents.map((event) => ({
    agentLabel: event.agentId === null ? null : (agentLabels.get(event.agentId) ?? event.agentId),
    demoOrigin: event.demoOrigin,
    detail: event.detail,
    id: event.id,
    kind: event.kind,
    ordinal: event.ordinal,
    redaction: event.redaction,
    relatedLabels: event.relatedNodeIds.map((id) => nodeLabels.get(id) ?? id),
    severity: event.severity,
    summary: event.summary,
    taskLabel:
      event.taskNodeId === null ? null : (nodeLabels.get(event.taskNodeId) ?? event.taskNodeId),
    timestamp: event.simulatedAt,
    workflowLabel:
      event.workflowNodeId === null
        ? null
        : (projection.nodes.find((node) => node.id === event.workflowNodeId)?.label ?? null),
  }));
  const activeNodeId =
    projection.nodes.find((node) => ACTIVE_STATUSES.has(node.status))?.id ?? null;

  return (
    <section aria-labelledby="command-center-page-title" className="page-stack command-center-page">
      <header className="command-center-page__header">
        <div>
          <p className="section-kicker">Operations · deterministic prototype</p>
          <h1 id="command-center-page-title">Command Center</h1>
          <p>
            Inspect a bounded visual projection of the application-owned agent architecture. The
            topology and activity remain frontend fixtures; the separate Rust projection is a
            read-only query. Nothing starts, cancels, approves, executes, or connects to a provider
            or model.
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
        onAgentChange={actions.setAgentId}
        onDemoOriginChange={actions.setDemoOrigin}
        onDomainChange={actions.setDomain}
        onEntityKindChange={actions.setEntityKind}
        onReset={actions.reset}
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

      {state.scenarioId === "research-knowledge-active" ? (
        <ResearchKnowledgeDemoProjectionPanel loader={projectionLoader} />
      ) : null}

      <SystemStatusSummary
        activeCount={projection.summary.activeCount}
        eventCount={projection.summary.eventCount}
        scenarioLabel={projection.summary.scenarioLabel}
        totalAgents={projection.summary.agentCount}
      />

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
          <OperationalTopologyPanel
            activeNodeId={activeNodeId}
            visibleEdges={visibleEdges}
            onSelect={actions.select}
            projection={projection}
            selectedId={state.selectedId}
            structuredView={
              <TopologyStructuredView
                edges={structuredEdges}
                groups={structuredGroups}
                nodes={structuredNodes}
                onSelect={actions.select}
                selectedId={state.selectedId}
              />
            }
            viewMode={state.viewMode}
            visibleNodes={visibleNodes}
          />
        )}
        <ContextualInspector selection={selectedInspector} />
      </div>

      <CommandCenterActivityStream
        eventKind={state.activityEventKind}
        eventKinds={activityEventKinds}
        events={activityEvents}
        followPathDisabled={state.selectedId === null}
        followSelectedPath={state.followSelectedPath}
        onEventKindChange={actions.setActivityEventKind}
        onFollowSelectedPathChange={actions.setFollowSelectedPath}
        onSearchChange={actions.setActivitySearch}
        onSeverityChange={actions.setActivitySeverity}
        search={state.activitySearch}
        severity={state.activitySeverity}
      />
    </section>
  );
}
