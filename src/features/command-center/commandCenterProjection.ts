export const COMMAND_CENTER_PROJECTION_VERSION = "command-center-demo-v1" as const;
export const COMMAND_CENTER_DISCLOSURE = "DEMO MODE · SIMULATED AGENT DATA" as const;
export const COMMAND_CENTER_SIMULATED_TIME_LABEL = "SIMULATED TIME" as const;

export const COMMAND_CENTER_AGENT_IDS = [
  "personal-assistant",
  "research",
  "coding",
  "cloud-infrastructure",
  "systems-operations",
  "knowledge-document",
  "qa-validation",
  "security-risk",
  "workflow-automation",
] as const;

export const COMMAND_CENTER_GROUP_IDS = [
  "core",
  "intelligence",
  "engineering",
  "infrastructure",
  "governance",
] as const;

export const COMMAND_CENTER_SCENARIO_IDS = [
  "catalog-idle",
  "research-queued",
  "research-knowledge-active",
  "engineering-waiting-approval",
  "infrastructure-blocked",
  "workflow-cancelled",
  "workflow-completed",
] as const;

export const COMMAND_CENTER_LIMITS = Object.freeze({
  edges: 64,
  events: 48,
  factsPerInspector: 8,
  findingsPerInspector: 8,
  groups: 5,
  nodes: 32,
  stringCharacters: 512,
  unresolvedIssuesPerInspector: 8,
});

export type CommandCenterAgentId = (typeof COMMAND_CENTER_AGENT_IDS)[number];
export type TopologyGroupId = (typeof COMMAND_CENTER_GROUP_IDS)[number];
export type CommandCenterScenarioId = (typeof COMMAND_CENTER_SCENARIO_IDS)[number];

export type TopologyNodeId = `demo-node:${string}`;
export type TopologyEdgeId = `demo-edge:${string}`;
export type CommandCenterEventId = `demo-event:${string}`;

export type TopologyNodeKind =
  | "orchestrator"
  | "agent"
  | "task"
  | "workflow"
  | "workflow-step"
  | "approval-checkpoint"
  | "validation-checkpoint"
  | "security-risk-checkpoint";

export type TopologyEdgeKind =
  | "orchestrates"
  | "delegates"
  | "owns-task"
  | "depends-on"
  | "validation-review"
  | "risk-review"
  | "approval-dependency"
  | "result-flow"
  | "cancellation"
  | "failure-propagation";

export type OperationalStatus =
  | "idle"
  | "queued"
  | "running"
  | "delegating"
  | "waiting-child"
  | "waiting-approval"
  | "validating"
  | "risk-review"
  | "blocked"
  | "cancelled"
  | "failed"
  | "completed";

export type AvailabilityStatus =
  | "available-in-fixture"
  | "preview-only"
  | "disabled"
  | "unavailable";

export type HealthStatus =
  | "not-measured"
  | "simulated-healthy"
  | "simulated-degraded"
  | "simulated-failed";

export type TrustStatus =
  | "application-authority"
  | "advisory-agent"
  | "untrusted-input"
  | "presentation-fixture";

export type ApprovalStatus =
  | "not-required"
  | "simulated-pending"
  | "simulated-approved"
  | "simulated-rejected"
  | "unavailable";

export type DemoOrigin =
  | "deterministic-fixture"
  | "frontend-mock"
  | "rust-fixture-only"
  | "unavailable";

export type CommandCenterEventKind =
  | "catalog-loaded"
  | "task-queued"
  | "task-started"
  | "task-progress"
  | "task-completed"
  | "task-failed"
  | "task-cancelled"
  | "dependency-satisfied"
  | "approval-required"
  | "validation-completed"
  | "risk-review-completed"
  | "workflow-blocked"
  | "workflow-cancelled"
  | "workflow-completed"
  | "result-returned"
  | "synthesis-started"
  | "synthesis-completed";

export type CommandCenterEventSeverity = "info" | "success" | "warning" | "danger";

export interface TopologyPosition {
  readonly x: number;
  readonly y: number;
}

export interface InspectorFact {
  readonly label: string;
  readonly value: string;
}

export interface InspectorProjection {
  readonly entityId: TopologyNodeId;
  readonly entityKind: TopologyNodeKind;
  readonly eyebrow: string;
  readonly title: string;
  readonly description: string;
  readonly responsibility: string;
  readonly authorityBoundary: string;
  readonly assignment: string | null;
  readonly inputs: readonly string[];
  readonly outputs: readonly string[];
  readonly dependencyIds: readonly TopologyNodeId[];
  readonly facts: readonly InspectorFact[];
  readonly findings: readonly string[];
  readonly unresolvedIssues: readonly string[];
  readonly demoOrigin: DemoOrigin;
}

interface TopologyNodeBase {
  readonly id: TopologyNodeId;
  readonly label: string;
  readonly status: OperationalStatus;
  readonly availability: AvailabilityStatus;
  readonly health: HealthStatus;
  readonly trust: TrustStatus;
  readonly approval: ApprovalStatus;
  readonly demoOrigin: DemoOrigin;
  readonly position: TopologyPosition;
  readonly inspector: InspectorProjection;
}

export interface OrchestratorTopologyNode extends TopologyNodeBase {
  readonly kind: "orchestrator";
  readonly agentId: null;
  readonly groupId: null;
}

export interface AgentTopologyNode extends TopologyNodeBase {
  readonly kind: "agent";
  readonly agentId: CommandCenterAgentId;
  readonly groupId: TopologyGroupId;
}

export interface WorkTopologyNode extends TopologyNodeBase {
  readonly kind: Exclude<TopologyNodeKind, "agent" | "orchestrator">;
  readonly agentId: CommandCenterAgentId | null;
  readonly groupId: null;
}

export type TopologyNode = OrchestratorTopologyNode | AgentTopologyNode | WorkTopologyNode;

export interface TopologyGroup {
  readonly id: TopologyGroupId;
  readonly label: string;
  readonly description: string;
  readonly demoOrigin: DemoOrigin;
}

export interface TopologyEdge {
  readonly id: TopologyEdgeId;
  readonly source: TopologyNodeId;
  readonly target: TopologyNodeId;
  readonly kind: TopologyEdgeKind;
  readonly label: string;
  readonly status: OperationalStatus;
  readonly demoOrigin: DemoOrigin;
}

export interface CommandCenterEvent {
  readonly id: CommandCenterEventId;
  readonly ordinal: number;
  readonly simulatedAt: string;
  readonly timeLabel: typeof COMMAND_CENTER_SIMULATED_TIME_LABEL;
  readonly kind: CommandCenterEventKind;
  readonly severity: CommandCenterEventSeverity;
  readonly summary: string;
  readonly detail: string;
  readonly agentId: CommandCenterAgentId | null;
  readonly taskNodeId: TopologyNodeId | null;
  readonly workflowNodeId: TopologyNodeId | null;
  readonly relatedNodeIds: readonly TopologyNodeId[];
  readonly redaction: "presentation-safe";
  readonly demoOrigin: DemoOrigin;
}

export interface CommandCenterSummary {
  readonly scenarioLabel: string;
  readonly disclosure: typeof COMMAND_CENTER_DISCLOSURE;
  readonly status: OperationalStatus;
  readonly agentCount: number;
  readonly groupCount: number;
  readonly workItemCount: number;
  readonly activeCount: number;
  readonly attentionCount: number;
  readonly completedCount: number;
  readonly eventCount: number;
  readonly demoOrigin: DemoOrigin;
}

export interface CommandCenterProjection {
  readonly version: typeof COMMAND_CENTER_PROJECTION_VERSION;
  readonly provenance: "deterministic-fixture";
  readonly scenarioId: CommandCenterScenarioId;
  readonly scenarioLabel: string;
  readonly disclosure: typeof COMMAND_CENTER_DISCLOSURE;
  readonly groups: readonly TopologyGroup[];
  readonly nodes: readonly TopologyNode[];
  readonly edges: readonly TopologyEdge[];
  readonly events: readonly CommandCenterEvent[];
  readonly summary: CommandCenterSummary;
}

export type ProjectionValidationCode =
  | "invalid-version"
  | "invalid-provenance"
  | "invalid-scenario"
  | "limit-exceeded"
  | "invalid-count"
  | "duplicate-id"
  | "invalid-reference"
  | "invalid-edge"
  | "dependency-cycle"
  | "invalid-order"
  | "invalid-summary"
  | "authority-boundary"
  | "unsafe-content";

export interface ProjectionValidationIssue {
  readonly code: ProjectionValidationCode;
  readonly path: string;
  readonly message: string;
}

export class CommandCenterProjectionValidationError extends Error {
  readonly issues: readonly ProjectionValidationIssue[];

  constructor(issues: readonly ProjectionValidationIssue[]) {
    super("Command Center projection failed deterministic validation");
    this.name = "CommandCenterProjectionValidationError";
    this.issues = issues;
  }
}

const EXPECTED_AGENT_GROUPS: Readonly<Record<CommandCenterAgentId, TopologyGroupId>> = {
  "personal-assistant": "core",
  research: "intelligence",
  coding: "engineering",
  "cloud-infrastructure": "infrastructure",
  "systems-operations": "infrastructure",
  "knowledge-document": "intelligence",
  "qa-validation": "engineering",
  "security-risk": "governance",
  "workflow-automation": "core",
};

const PROTECTED_FIELD_KEYS = new Set([
  "prompt",
  "objective",
  "context",
  "output",
  "reasoning",
  "toolArguments",
  "toolResult",
  "memoryContent",
  "documentContent",
  "documentPath",
  "path",
  "approvalSubject",
  "affectedData",
  "credential",
  "credentials",
  "rawJson",
  "requestId",
  "responseId",
  "runId",
  "runtimeIdentity",
]);

const EDGE_KIND_RULES: Readonly<
  Record<
    TopologyEdgeKind,
    Readonly<{ sources: readonly TopologyNodeKind[]; targets: readonly TopologyNodeKind[] }>
  >
> = {
  orchestrates: {
    sources: ["orchestrator"],
    targets: ["agent", "task", "workflow"],
  },
  delegates: {
    sources: ["orchestrator"],
    targets: ["agent"],
  },
  "owns-task": {
    sources: ["task"],
    targets: ["agent"],
  },
  "depends-on": {
    sources: [
      "task",
      "workflow-step",
      "approval-checkpoint",
      "validation-checkpoint",
      "security-risk-checkpoint",
    ],
    targets: [
      "workflow",
      "task",
      "workflow-step",
      "approval-checkpoint",
      "validation-checkpoint",
      "security-risk-checkpoint",
    ],
  },
  "validation-review": {
    sources: ["agent"],
    targets: ["task", "workflow-step", "validation-checkpoint"],
  },
  "risk-review": {
    sources: ["agent"],
    targets: ["task", "workflow-step", "security-risk-checkpoint"],
  },
  "approval-dependency": {
    sources: ["task", "workflow", "workflow-step"],
    targets: ["approval-checkpoint"],
  },
  "result-flow": {
    sources: [
      "agent",
      "task",
      "workflow-step",
      "validation-checkpoint",
      "security-risk-checkpoint",
    ],
    targets: ["agent", "workflow", "workflow-step"],
  },
  cancellation: {
    sources: ["orchestrator", "task", "workflow"],
    targets: ["task", "workflow"],
  },
  "failure-propagation": {
    sources: ["task", "validation-checkpoint", "security-risk-checkpoint"],
    targets: ["task", "workflow", "workflow-step"],
  },
};

function includesValue<const T extends readonly string[]>(
  values: T,
  value: string,
): value is T[number] {
  return (values as readonly string[]).includes(value);
}

function runtimeText(value: string): string {
  return value;
}

export function isCommandCenterScenarioId(value: unknown): value is CommandCenterScenarioId {
  return typeof value === "string" && includesValue(COMMAND_CENTER_SCENARIO_IDS, value);
}

function issue(
  issues: ProjectionValidationIssue[],
  code: ProjectionValidationCode,
  path: string,
  message: string,
): void {
  issues.push({ code, path, message });
}

function duplicateValues(values: readonly string[]): readonly string[] {
  const seen = new Set<string>();
  const duplicates = new Set<string>();
  for (const value of values) {
    if (seen.has(value)) {
      duplicates.add(value);
    }
    seen.add(value);
  }
  return [...duplicates];
}

function validateSafeContent(
  value: unknown,
  issues: ProjectionValidationIssue[],
  path: string,
  seen: WeakSet<object>,
): void {
  if (typeof value === "string") {
    if (value.length === 0 || value.length > COMMAND_CENTER_LIMITS.stringCharacters) {
      issue(issues, "unsafe-content", path, "Presentation text must be non-empty and bounded");
    }
    if (/\bLIVE\b/u.test(value)) {
      issue(issues, "unsafe-content", path, "Ambiguous operational wording is prohibited");
    }
    if (/(?:\/Users\/|file:\/\/|[A-Za-z]:\\)/u.test(value)) {
      issue(issues, "unsafe-content", path, "Filesystem paths are prohibited");
    }
    if (/\b(?:api[_-]?key|bearer|password|secret|token)\s*[:=]/iu.test(value)) {
      issue(issues, "unsafe-content", path, "Credential-shaped presentation text is prohibited");
    }
    return;
  }

  if (value === null || typeof value !== "object") {
    return;
  }
  if (seen.has(value)) {
    issue(issues, "unsafe-content", path, "Cyclic projection values are prohibited");
    return;
  }
  seen.add(value);

  if (Array.isArray(value)) {
    for (const [index, item] of value.entries()) {
      validateSafeContent(item, issues, `${path}[${String(index)}]`, seen);
    }
    return;
  }

  for (const [key, item] of Object.entries(value)) {
    if (PROTECTED_FIELD_KEYS.has(key)) {
      issue(issues, "unsafe-content", `${path}.${key}`, "Protected content fields are prohibited");
    }
    validateSafeContent(item, issues, `${path}.${key}`, seen);
  }
}

function validateAuthorityBoundaries(
  nodes: readonly TopologyNode[],
  issues: ProjectionValidationIssue[],
): void {
  const orchestrators = nodes.filter((node) => node.kind === "orchestrator");
  if (orchestrators.length !== 1) {
    issue(issues, "invalid-count", "nodes", "Exactly one orchestrator node is required");
  } else if (orchestrators[0]?.trust !== "application-authority") {
    issue(
      issues,
      "authority-boundary",
      "nodes.orchestrator.trust",
      "The orchestrator must remain application authority",
    );
  }

  const agentNodes = nodes.filter((node): node is AgentTopologyNode => node.kind === "agent");
  if (agentNodes.length !== COMMAND_CENTER_AGENT_IDS.length) {
    issue(issues, "invalid-count", "nodes", "Exactly nine agent nodes are required");
  }

  for (const agentId of COMMAND_CENTER_AGENT_IDS) {
    const matches = agentNodes.filter((node) => node.agentId === agentId);
    if (matches.length !== 1) {
      issue(
        issues,
        "invalid-count",
        `nodes.agent.${agentId}`,
        "Every closed agent identity must appear exactly once",
      );
      continue;
    }
    const node = matches[0];
    if (node === undefined) {
      continue;
    }
    if (node.groupId !== EXPECTED_AGENT_GROUPS[agentId] || node.trust !== "advisory-agent") {
      issue(
        issues,
        "authority-boundary",
        `nodes.agent.${agentId}`,
        "Agent group and advisory trust must match the closed catalog",
      );
    }
  }

  const personal = agentNodes.find((node) => node.agentId === "personal-assistant");
  const qa = agentNodes.find((node) => node.agentId === "qa-validation");
  const security = agentNodes.find((node) => node.agentId === "security-risk");
  const workflow = agentNodes.find((node) => node.agentId === "workflow-automation");
  if (personal?.inspector.authorityBoundary.includes("not AgentOrchestrator") !== true) {
    issue(
      issues,
      "authority-boundary",
      "nodes.agent.personal-assistant.inspector.authorityBoundary",
      "Personal Assistant must remain distinct from AgentOrchestrator",
    );
  }
  if (qa?.inspector.authorityBoundary.includes("not ApprovalManager") !== true) {
    issue(
      issues,
      "authority-boundary",
      "nodes.agent.qa-validation.inspector.authorityBoundary",
      "QA must not be represented as approval authority",
    );
  }
  if (security?.inspector.authorityBoundary.includes("not PolicyEngine") !== true) {
    issue(
      issues,
      "authority-boundary",
      "nodes.agent.security-risk.inspector.authorityBoundary",
      "Security must not be represented as policy authority",
    );
  }
  if (workflow?.inspector.authorityBoundary.includes("not AgentOrchestrator") !== true) {
    issue(
      issues,
      "authority-boundary",
      "nodes.agent.workflow-automation.inspector.authorityBoundary",
      "Workflow Automation must not be represented as orchestration authority",
    );
  }
}

function validateGroups(
  groups: readonly TopologyGroup[],
  issues: ProjectionValidationIssue[],
): void {
  if (groups.length !== COMMAND_CENTER_LIMITS.groups) {
    issue(issues, "invalid-count", "groups", "Exactly five presentation groups are required");
  }
  const groupIds = groups.map((group) => group.id);
  for (const duplicate of duplicateValues(groupIds)) {
    issue(issues, "duplicate-id", `groups.${duplicate}`, "Group IDs must be unique");
  }
  for (const groupId of COMMAND_CENTER_GROUP_IDS) {
    if (!groupIds.includes(groupId)) {
      issue(issues, "invalid-count", `groups.${groupId}`, "A closed group is missing");
    }
  }
  for (const group of groups) {
    const groupId: string = group.id;
    if (!includesValue(COMMAND_CENTER_GROUP_IDS, groupId)) {
      issue(issues, "invalid-reference", "groups.foreign", "Foreign groups are prohibited");
    }
  }
}

function validateEdges(
  edges: readonly TopologyEdge[],
  nodes: readonly TopologyNode[],
  issues: ProjectionValidationIssue[],
): void {
  const nodesById = new Map(nodes.map((node) => [node.id, node]));
  for (const edge of edges) {
    const source = nodesById.get(edge.source);
    const target = nodesById.get(edge.target);
    if (source === undefined || target === undefined) {
      issue(
        issues,
        "invalid-reference",
        `edges.${edge.id}`,
        "Every edge endpoint must resolve to a projection node",
      );
      continue;
    }
    const rule = EDGE_KIND_RULES[edge.kind];
    if (!rule.sources.includes(source.kind) || !rule.targets.includes(target.kind)) {
      issue(
        issues,
        "invalid-edge",
        `edges.${edge.id}`,
        "The edge kind does not permit this source and target node kind",
      );
    }
    if (edge.kind === "delegates" && target.agentId === "personal-assistant") {
      issue(
        issues,
        "invalid-edge",
        `edges.${edge.id}`,
        "Personal Assistant is the root role, not a delegated specialist",
      );
    }
    if (edge.kind === "owns-task" && source.agentId !== target.agentId) {
      issue(
        issues,
        "invalid-edge",
        `edges.${edge.id}`,
        "Task ownership must match the task agent identity",
      );
    }
    if (edge.kind === "validation-review" && source.agentId !== "qa-validation") {
      issue(
        issues,
        "authority-boundary",
        `edges.${edge.id}`,
        "Only the advisory QA role may originate validation-review edges",
      );
    }
    if (edge.kind === "risk-review" && source.agentId !== "security-risk") {
      issue(
        issues,
        "authority-boundary",
        `edges.${edge.id}`,
        "Only the advisory Security role may originate risk-review edges",
      );
    }
    if (
      edge.kind === "result-flow" &&
      target.kind === "agent" &&
      target.agentId !== "personal-assistant"
    ) {
      issue(
        issues,
        "invalid-edge",
        `edges.${edge.id}`,
        "Agent-directed results may return only to Personal Assistant",
      );
    }
  }

  const dependencyEdges = edges.filter((edge) => edge.kind === "depends-on");
  const dependencies = new Map<TopologyNodeId, TopologyNodeId[]>();
  for (const edge of dependencyEdges) {
    const existing = dependencies.get(edge.source) ?? [];
    existing.push(edge.target);
    dependencies.set(edge.source, existing);
  }
  const visiting = new Set<TopologyNodeId>();
  const visited = new Set<TopologyNodeId>();
  const hasCycle = (nodeId: TopologyNodeId): boolean => {
    if (visiting.has(nodeId)) {
      return true;
    }
    if (visited.has(nodeId)) {
      return false;
    }
    visiting.add(nodeId);
    for (const dependency of dependencies.get(nodeId) ?? []) {
      if (hasCycle(dependency)) {
        return true;
      }
    }
    visiting.delete(nodeId);
    visited.add(nodeId);
    return false;
  };
  if ([...dependencies.keys()].some((nodeId) => hasCycle(nodeId))) {
    issue(issues, "dependency-cycle", "edges", "Dependency edges must remain acyclic");
  }
}

function validateEvents(
  events: readonly CommandCenterEvent[],
  nodes: readonly TopologyNode[],
  issues: ProjectionValidationIssue[],
): void {
  const nodeIds = new Set(nodes.map((node) => node.id));
  for (const [index, event] of events.entries()) {
    if (event.ordinal !== index + 1) {
      issue(
        issues,
        "invalid-order",
        `events.${event.id}.ordinal`,
        "Event ordinals must be consecutive and deterministic",
      );
    }
    if (
      runtimeText(event.timeLabel) !== COMMAND_CENTER_SIMULATED_TIME_LABEL ||
      !/^20\d{2}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.000Z$/u.test(event.simulatedAt)
    ) {
      issue(
        issues,
        "invalid-order",
        `events.${event.id}.simulatedAt`,
        "Events require fixed ISO simulated time",
      );
    }
    for (const relatedNodeId of event.relatedNodeIds) {
      if (!nodeIds.has(relatedNodeId)) {
        issue(
          issues,
          "invalid-reference",
          `events.${event.id}.relatedNodeIds`,
          "Event node references must resolve",
        );
      }
    }
    for (const nodeId of [event.taskNodeId, event.workflowNodeId]) {
      if (nodeId !== null && !nodeIds.has(nodeId)) {
        issue(
          issues,
          "invalid-reference",
          `events.${event.id}`,
          "Event task and workflow references must resolve",
        );
      }
    }
  }
}

function validateSummary(
  projection: CommandCenterProjection,
  issues: ProjectionValidationIssue[],
): void {
  const workNodes = projection.nodes.filter(
    (node) => node.kind !== "agent" && node.kind !== "orchestrator",
  );
  const activeStatuses: readonly OperationalStatus[] = [
    "queued",
    "running",
    "delegating",
    "waiting-child",
    "waiting-approval",
    "validating",
    "risk-review",
  ];
  const attentionStatuses: readonly OperationalStatus[] = ["blocked", "failed", "cancelled"];
  const expected = {
    agentCount: projection.nodes.filter((node) => node.kind === "agent").length,
    groupCount: projection.groups.length,
    workItemCount: workNodes.length,
    activeCount: workNodes.filter((node) => activeStatuses.includes(node.status)).length,
    attentionCount: workNodes.filter((node) => attentionStatuses.includes(node.status)).length,
    completedCount: workNodes.filter((node) => node.status === "completed").length,
    eventCount: projection.events.length,
  };
  for (const [key, value] of Object.entries(expected)) {
    const summaryValue = projection.summary[key as keyof typeof expected];
    if (summaryValue !== value) {
      issue(
        issues,
        "invalid-summary",
        `summary.${key}`,
        "Summary counts must be derived from the projection",
      );
    }
  }
  if (
    projection.summary.scenarioLabel !== projection.scenarioLabel ||
    runtimeText(projection.summary.disclosure) !== COMMAND_CENTER_DISCLOSURE
  ) {
    issue(
      issues,
      "invalid-summary",
      "summary",
      "Summary identity and disclosure must match the projection",
    );
  }
}

export function validateCommandCenterProjection(
  projection: CommandCenterProjection,
): readonly ProjectionValidationIssue[] {
  const issues: ProjectionValidationIssue[] = [];
  if (runtimeText(projection.version) !== COMMAND_CENTER_PROJECTION_VERSION) {
    issue(issues, "invalid-version", "version", "Projection version is not supported");
  }
  if (
    runtimeText(projection.provenance) !== "deterministic-fixture" ||
    runtimeText(projection.disclosure) !== COMMAND_CENTER_DISCLOSURE
  ) {
    issue(
      issues,
      "invalid-provenance",
      "provenance",
      "Projection provenance and disclosure must identify simulated data",
    );
  }
  if (!isCommandCenterScenarioId(projection.scenarioId)) {
    issue(issues, "invalid-scenario", "scenarioId", "Scenario is outside the closed catalog");
  }
  if (projection.nodes.length > COMMAND_CENTER_LIMITS.nodes) {
    issue(issues, "limit-exceeded", "nodes", "Projection node limit exceeded");
  }
  if (projection.edges.length > COMMAND_CENTER_LIMITS.edges) {
    issue(issues, "limit-exceeded", "edges", "Projection edge limit exceeded");
  }
  if (projection.events.length > COMMAND_CENTER_LIMITS.events) {
    issue(issues, "limit-exceeded", "events", "Projection event limit exceeded");
  }

  const nodeIds = projection.nodes.map((node) => node.id);
  const edgeIds = projection.edges.map((edge) => edge.id);
  const eventIds = projection.events.map((event) => event.id);
  for (const [path, values] of [
    ["nodes", nodeIds],
    ["edges", edgeIds],
    ["events", eventIds],
  ] as const) {
    for (const duplicate of duplicateValues(values)) {
      issue(issues, "duplicate-id", `${path}.${duplicate}`, "Presentation IDs must be unique");
    }
  }

  validateGroups(projection.groups, issues);
  validateAuthorityBoundaries(projection.nodes, issues);
  validateEdges(projection.edges, projection.nodes, issues);
  validateEvents(projection.events, projection.nodes, issues);
  validateSummary(projection, issues);

  const nodeIdSet = new Set(nodeIds);
  for (const node of projection.nodes) {
    if (node.inspector.entityId !== node.id || node.inspector.entityKind !== node.kind) {
      issue(
        issues,
        "invalid-reference",
        `nodes.${node.id}.inspector`,
        "Inspector identity must match its node",
      );
    }
    if (node.inspector.facts.length > COMMAND_CENTER_LIMITS.factsPerInspector) {
      issue(
        issues,
        "limit-exceeded",
        `nodes.${node.id}.inspector.facts`,
        "Inspector fact limit exceeded",
      );
    }
    if (node.inspector.findings.length > COMMAND_CENTER_LIMITS.findingsPerInspector) {
      issue(
        issues,
        "limit-exceeded",
        `nodes.${node.id}.inspector.findings`,
        "Inspector finding limit exceeded",
      );
    }
    if (
      node.inspector.unresolvedIssues.length > COMMAND_CENTER_LIMITS.unresolvedIssuesPerInspector
    ) {
      issue(
        issues,
        "limit-exceeded",
        `nodes.${node.id}.inspector.unresolvedIssues`,
        "Inspector unresolved-issue limit exceeded",
      );
    }
    for (const dependencyId of node.inspector.dependencyIds) {
      if (!nodeIdSet.has(dependencyId)) {
        issue(
          issues,
          "invalid-reference",
          `nodes.${node.id}.inspector.dependencyIds`,
          "Inspector dependencies must resolve",
        );
      }
    }
  }

  const primaryOrigins: readonly DemoOrigin[] = [
    ...projection.groups.map((group) => group.demoOrigin),
    ...projection.nodes.map((node) => node.demoOrigin),
    ...projection.nodes.map((node) => node.inspector.demoOrigin),
    ...projection.edges.map((edge) => edge.demoOrigin),
    ...projection.events.map((event) => event.demoOrigin),
    projection.summary.demoOrigin,
  ];
  if (primaryOrigins.some((origin) => origin !== "deterministic-fixture")) {
    issue(
      issues,
      "invalid-provenance",
      "demoOrigin",
      "Every first-prototype projection entity must carry deterministic fixture provenance",
    );
  }

  validateSafeContent(projection, issues, "projection", new WeakSet());
  return Object.freeze(issues.map((item) => Object.freeze(item)));
}

function deepFreeze(value: unknown, seen = new WeakSet()): void {
  if (value === null || typeof value !== "object" || seen.has(value)) {
    return;
  }
  seen.add(value);
  for (const child of Object.values(value)) {
    deepFreeze(child, seen);
  }
  Object.freeze(value);
}

export function finalizeCommandCenterProjection(
  projection: CommandCenterProjection,
): CommandCenterProjection {
  const issues = validateCommandCenterProjection(projection);
  if (issues.length > 0) {
    throw new CommandCenterProjectionValidationError(issues);
  }
  deepFreeze(projection);
  return projection;
}
