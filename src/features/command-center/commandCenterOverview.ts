import {
  COMMAND_CENTER_AGENT_IDS,
  type COMMAND_CENTER_DISCLOSURE,
  type COMMAND_CENTER_SIMULATED_TIME_LABEL,
  type AgentTopologyNode,
  type ApprovalStatus,
  type CommandCenterAgentId,
  type CommandCenterEvent,
  type CommandCenterProjection,
  type DemoOrigin,
  type HealthStatus,
  type OperationalStatus,
  type TopologyNodeId,
  type WorkTopologyNode,
} from "./commandCenterProjection";
import {
  buildCommandCenterEventPresentations,
  type CommandCenterEventPresentation,
} from "./commandCenterEventPresentation";

const ACTIVE_WORK_STATUSES: ReadonlySet<OperationalStatus> = new Set([
  "queued",
  "running",
  "delegating",
  "waiting-child",
  "waiting-approval",
  "validating",
  "risk-review",
]);

const WAITING_WORK_STATUSES: ReadonlySet<OperationalStatus> = new Set([
  "queued",
  "waiting-child",
  "waiting-approval",
]);

export interface CommandCenterUnavailableSource {
  readonly detail: string;
  readonly state: "unavailable-in-projection";
}

export interface CommandCenterSystemStatusView {
  readonly activeWorkCount: number;
  readonly blockedCount: number;
  readonly deterministicEventCount: number;
  readonly failedCount: number;
  readonly pendingApprovalCount: number;
  readonly provenance: CommandCenterProjection["provenance"];
  readonly runningAgentCount: number;
  readonly scenarioLabel: string;
  readonly simulatedStatus: OperationalStatus;
  readonly sources: Readonly<{
    provider: CommandCenterUnavailableSource;
    runtime: CommandCenterUnavailableSource;
    toolExecution: CommandCenterUnavailableSource;
  }>;
  readonly waitingWorkCount: number;
}

export interface CommandCenterRelatedActivity {
  readonly simulatedAt: string;
  readonly summary: string;
  readonly timeLabel: typeof COMMAND_CENTER_SIMULATED_TIME_LABEL;
}

export interface CommandCenterAgentDefinitionView {
  readonly agentId: CommandCenterAgentId;
  readonly availability: AgentTopologyNode["availability"];
  readonly currentWorkLabel: string | null;
  readonly demoOrigin: DemoOrigin;
  readonly domainLabel: string;
  readonly entityKind: "agent-definition";
  readonly health: HealthStatus;
  readonly label: string;
  readonly lastActivity: CommandCenterRelatedActivity | null;
  readonly nodeId: TopologyNodeId;
  readonly responsibility: string;
  readonly simulatedStatus: OperationalStatus;
  readonly toolAccess: "unavailable-in-projection";
}

interface CommandCenterWorkItemBase {
  readonly approval: ApprovalStatus;
  readonly blockedReason: string | null;
  readonly demoOrigin: DemoOrigin;
  readonly dependencyLabels: readonly string[];
  readonly label: string;
  readonly lastActivity: CommandCenterRelatedActivity | null;
  readonly nodeId: TopologyNodeId;
  readonly ownerLabel: string;
  readonly simulatedStatus: OperationalStatus;
}

export type CommandCenterWorkItem =
  | (CommandCenterWorkItemBase & {
      readonly entityKind: "task";
      readonly presentationKind: "task";
    })
  | (CommandCenterWorkItemBase & {
      readonly entityKind: "workflow" | "workflow-step";
      readonly presentationKind: "workflow";
    })
  | (CommandCenterWorkItemBase & {
      readonly entityKind:
        | "approval-checkpoint"
        | "security-risk-checkpoint"
        | "validation-checkpoint";
      readonly presentationKind: "checkpoint";
    });

export type CommandCenterAttentionItem = Readonly<{
  category: "approval" | "blocked" | "cancelled" | "failure" | "unresolved";
  demoOrigin: DemoOrigin;
  detail: string;
  label: string;
  nodeId: TopologyNodeId;
  simulatedStatus: OperationalStatus;
}>;

export interface CommandCenterRecentEventView extends CommandCenterEventPresentation {
  readonly presentationKind: "event";
  readonly selectionTargetId: TopologyNodeId | null;
}

export interface CommandCenterOverviewViewModel {
  readonly activeWork: readonly CommandCenterWorkItem[];
  readonly agents: readonly CommandCenterAgentDefinitionView[];
  readonly attention: readonly CommandCenterAttentionItem[];
  readonly disclosure: typeof COMMAND_CENTER_DISCLOSURE;
  readonly provenance: CommandCenterProjection["provenance"];
  readonly recentActivity: readonly CommandCenterRecentEventView[];
  readonly system: CommandCenterSystemStatusView;
}

function relatedEventForNode(
  node: AgentTopologyNode | WorkTopologyNode,
  events: readonly CommandCenterEvent[],
): CommandCenterRelatedActivity | null {
  const related = [...events]
    .filter(
      (event) =>
        event.relatedNodeIds.includes(node.id) ||
        event.taskNodeId === node.id ||
        event.workflowNodeId === node.id ||
        (node.kind === "agent" && event.agentId === node.agentId),
    )
    .sort((left, right) => right.ordinal - left.ordinal)[0];

  return related === undefined
    ? null
    : {
        simulatedAt: related.simulatedAt,
        summary: related.summary,
        timeLabel: related.timeLabel,
      };
}

function workPresentationKind(node: WorkTopologyNode): CommandCenterWorkItem {
  const common = {
    approval: node.approval,
    blockedReason: node.inspector.unresolvedIssues[0] ?? null,
    demoOrigin: node.demoOrigin,
    dependencyLabels: [] as readonly string[],
    label: node.label,
    lastActivity: null,
    nodeId: node.id,
    ownerLabel: "Application boundary",
    simulatedStatus: node.status,
  };

  switch (node.kind) {
    case "task":
      return { ...common, entityKind: node.kind, presentationKind: "task" };
    case "workflow":
    case "workflow-step":
      return { ...common, entityKind: node.kind, presentationKind: "workflow" };
    case "approval-checkpoint":
    case "security-risk-checkpoint":
    case "validation-checkpoint":
      return { ...common, entityKind: node.kind, presentationKind: "checkpoint" };
  }
}

function attentionFor(node: WorkTopologyNode): CommandCenterAttentionItem | null {
  if (node.approval === "simulated-pending") {
    return {
      category: "approval",
      demoOrigin: node.demoOrigin,
      detail: node.inspector.unresolvedIssues[0] ?? "A simulated application decision is pending.",
      label: node.label,
      nodeId: node.id,
      simulatedStatus: node.status,
    };
  }

  if (node.status === "failed") {
    return {
      category: "failure",
      demoOrigin: node.demoOrigin,
      detail:
        node.inspector.unresolvedIssues[0] ?? "Failure detail is unavailable in this fixture.",
      label: node.label,
      nodeId: node.id,
      simulatedStatus: node.status,
    };
  }

  if (node.status === "blocked") {
    return {
      category: "blocked",
      demoOrigin: node.demoOrigin,
      detail:
        node.inspector.unresolvedIssues[0] ?? "Blocked detail is unavailable in this fixture.",
      label: node.label,
      nodeId: node.id,
      simulatedStatus: node.status,
    };
  }

  if (node.status === "cancelled") {
    return {
      category: "cancelled",
      demoOrigin: node.demoOrigin,
      detail: node.inspector.unresolvedIssues[0] ?? "The simulated work is terminally cancelled.",
      label: node.label,
      nodeId: node.id,
      simulatedStatus: node.status,
    };
  }

  const unresolved = node.inspector.unresolvedIssues[0];
  return unresolved === undefined
    ? null
    : {
        category: "unresolved",
        demoOrigin: node.demoOrigin,
        detail: unresolved,
        label: node.label,
        nodeId: node.id,
        simulatedStatus: node.status,
      };
}

function eventSelectionTarget(
  event: CommandCenterEvent,
  nodeById: ReadonlyMap<TopologyNodeId, AgentTopologyNode | WorkTopologyNode>,
  agentNodeById: ReadonlyMap<CommandCenterAgentId, AgentTopologyNode>,
): TopologyNodeId | null {
  if (event.taskNodeId !== null && nodeById.has(event.taskNodeId)) return event.taskNodeId;

  const relatedTarget = event.relatedNodeIds.find((nodeId) => nodeById.has(nodeId));
  if (relatedTarget !== undefined) return relatedTarget;

  if (event.workflowNodeId !== null && nodeById.has(event.workflowNodeId)) {
    return event.workflowNodeId;
  }

  return event.agentId === null ? null : (agentNodeById.get(event.agentId)?.id ?? null);
}

export function buildCommandCenterOverview(
  projection: CommandCenterProjection,
): CommandCenterOverviewViewModel {
  const agentNodes = projection.nodes.filter(
    (node): node is AgentTopologyNode => node.kind === "agent",
  );
  const workNodes = projection.nodes.filter(
    (node): node is WorkTopologyNode => node.kind !== "agent" && node.kind !== "orchestrator",
  );
  const agentNodeById = new Map(agentNodes.map((node) => [node.agentId, node]));
  const nodeById = new Map([...agentNodes, ...workNodes].map((node) => [node.id, node] as const));
  const groupLabelById = new Map(projection.groups.map((group) => [group.id, group.label]));
  const nodeLabelById = new Map(projection.nodes.map((node) => [node.id, node.label]));
  const agentLabelById = new Map(agentNodes.map((node) => [node.agentId, node.label]));
  const eventPresentationById = new Map(
    buildCommandCenterEventPresentations(projection).map((event) => [event.eventId, event]),
  );

  const agents = COMMAND_CENTER_AGENT_IDS.map((agentId): CommandCenterAgentDefinitionView => {
    const matchingNodes = agentNodes.filter((node) => node.agentId === agentId);
    const node = matchingNodes[0];
    if (node === undefined || matchingNodes.length !== 1) {
      throw new Error(`Validated Command Center projection must contain one ${agentId} agent`);
    }

    const currentWork = workNodes.find(
      (workNode) => workNode.agentId === agentId && ACTIVE_WORK_STATUSES.has(workNode.status),
    );
    return {
      agentId,
      availability: node.availability,
      currentWorkLabel: currentWork?.label ?? null,
      demoOrigin: node.demoOrigin,
      domainLabel: groupLabelById.get(node.groupId) ?? node.groupId,
      entityKind: "agent-definition",
      health: node.health,
      label: node.label,
      lastActivity: relatedEventForNode(node, projection.events),
      nodeId: node.id,
      responsibility: node.inspector.responsibility,
      simulatedStatus: node.status,
      toolAccess: "unavailable-in-projection",
    };
  });

  const activeWork = workNodes
    .filter((node) => ACTIVE_WORK_STATUSES.has(node.status))
    .map((node): CommandCenterWorkItem => {
      const base = workPresentationKind(node);
      return {
        ...base,
        dependencyLabels: node.inspector.dependencyIds.map(
          (dependencyId) => nodeLabelById.get(dependencyId) ?? dependencyId,
        ),
        lastActivity: relatedEventForNode(node, projection.events),
        ownerLabel:
          node.agentId === null
            ? "Application boundary"
            : (agentLabelById.get(node.agentId) ?? node.agentId),
      };
    });
  const attention = workNodes
    .map(attentionFor)
    .filter((item): item is CommandCenterAttentionItem => item !== null);
  const recentActivity = [...projection.events]
    .sort((left, right) => right.ordinal - left.ordinal)
    .slice(0, 5)
    .map((event): CommandCenterRecentEventView => {
      const selectionTargetId = eventSelectionTarget(event, nodeById, agentNodeById);
      const presentation = eventPresentationById.get(event.id);
      if (presentation === undefined) {
        throw new Error(`Validated Command Center event presentation is missing ${event.id}`);
      }
      return {
        ...presentation,
        presentationKind: "event",
        selectionTargetId,
      };
    });

  return {
    activeWork,
    agents,
    attention,
    disclosure: projection.disclosure,
    provenance: projection.provenance,
    recentActivity,
    system: {
      activeWorkCount: activeWork.length,
      blockedCount: workNodes.filter((node) => node.status === "blocked").length,
      deterministicEventCount: projection.events.length,
      failedCount: workNodes.filter((node) => node.status === "failed").length,
      pendingApprovalCount: workNodes.filter((node) => node.approval === "simulated-pending")
        .length,
      provenance: projection.provenance,
      runningAgentCount: agentNodes.filter((node) => node.status === "running").length,
      scenarioLabel: projection.scenarioLabel,
      simulatedStatus: projection.summary.status,
      sources: {
        provider: {
          detail: "No model or provider health feed is connected to this projection.",
          state: "unavailable-in-projection",
        },
        runtime: {
          detail: "No runtime instance or runtime health feed is connected to this projection.",
          state: "unavailable-in-projection",
        },
        toolExecution: {
          detail: "No tool availability or execution feed is connected to this projection.",
          state: "unavailable-in-projection",
        },
      },
      waitingWorkCount: workNodes.filter((node) => WAITING_WORK_STATUSES.has(node.status)).length,
    },
  };
}
