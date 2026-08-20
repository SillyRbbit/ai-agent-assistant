import {
  COMMAND_CENTER_AGENT_IDS,
  COMMAND_CENTER_DISCLOSURE,
  COMMAND_CENTER_PROJECTION_VERSION,
  COMMAND_CENTER_SIMULATED_TIME_LABEL,
  finalizeCommandCenterProjection,
  type AgentTopologyNode,
  type ApprovalStatus,
  type CommandCenterAgentId,
  type CommandCenterEvent,
  type CommandCenterEventId,
  type CommandCenterEventKind,
  type CommandCenterEventSeverity,
  type CommandCenterProjection,
  type CommandCenterScenarioId,
  type CommandCenterSummary,
  type DemoOrigin,
  type InspectorProjection,
  type OperationalStatus,
  type OrchestratorTopologyNode,
  type TopologyEdge,
  type TopologyEdgeId,
  type TopologyEdgeKind,
  type TopologyGroup,
  type TopologyGroupId,
  type TopologyNode,
  type TopologyNodeId,
  type TopologyNodeKind,
  type WorkTopologyNode,
} from "./commandCenterProjection";

const DEMO_ORIGIN: DemoOrigin = "deterministic-fixture";

export const DEFAULT_COMMAND_CENTER_SCENARIO_ID: CommandCenterScenarioId = "catalog-idle";

export interface CommandCenterFixtureMetadata {
  readonly id: CommandCenterScenarioId;
  readonly label: string;
  readonly description: string;
}

export const COMMAND_CENTER_FIXTURE_CATALOG = Object.freeze([
  {
    id: "catalog-idle",
    label: "Agent catalog",
    description: "All nine advisory roles with no selected simulated work.",
  },
  {
    id: "research-queued",
    label: "Research queued",
    description: "A bounded root request is queued for the Research Agent.",
  },
  {
    id: "research-knowledge-active",
    label: "Research and knowledge",
    description: "Independent fixture analyses are retained and interleaved on one thread.",
  },
  {
    id: "engineering-waiting-approval",
    label: "Engineering review",
    description: "Advisory QA and Security results precede a simulated application checkpoint.",
  },
  {
    id: "infrastructure-blocked",
    label: "Infrastructure blocked",
    description: "A simulated security failure blocks a dependent synthesis step.",
  },
  {
    id: "workflow-cancelled",
    label: "Workflow cancelled",
    description: "Root cancellation propagates to active simulated child work.",
  },
  {
    id: "workflow-completed",
    label: "Workflow proposal complete",
    description: "A proposal-only workflow returns ordered simulated results.",
  },
] as const satisfies readonly CommandCenterFixtureMetadata[]);

interface AgentCatalogEntry {
  readonly id: CommandCenterAgentId;
  readonly label: string;
  readonly groupId: TopologyGroupId;
  readonly responsibility: string;
  readonly authorityBoundary: string;
  readonly x: number;
  readonly y: number;
}

const AGENT_CATALOG: readonly AgentCatalogEntry[] = [
  {
    id: "personal-assistant",
    label: "Personal Assistant",
    groupId: "core",
    responsibility: "Receives the bounded root request and synthesizes specialist results.",
    authorityBoundary:
      "Personal Assistant is not AgentOrchestrator and cannot authorize consequential actions.",
    x: 60,
    y: 190,
  },
  {
    id: "workflow-automation",
    label: "Workflow Automation Agent",
    groupId: "core",
    responsibility: "Prepares proposals from the closed fixture workflow catalog.",
    authorityBoundary:
      "Workflow Automation is not AgentOrchestrator; it cannot create tasks, schedule, or execute work.",
    x: 250,
    y: 190,
  },
  {
    id: "research",
    label: "Research Agent",
    groupId: "intelligence",
    responsibility: "Produces bounded evidence-oriented analysis from supplied fixture material.",
    authorityBoundary: "Advisory analysis only; it cannot delegate, approve, or execute actions.",
    x: 460,
    y: 190,
  },
  {
    id: "knowledge-document",
    label: "Knowledge & Document Agent",
    groupId: "intelligence",
    responsibility:
      "Organizes bounded knowledge and document findings in approved Rust-only paths.",
    authorityBoundary:
      "This projection grants no document access, memory authority, delegation, or execution.",
    x: 650,
    y: 190,
  },
  {
    id: "coding",
    label: "Coding Agent",
    groupId: "engineering",
    responsibility: "Prepares fixture-only implementation and validation proposals.",
    authorityBoundary: "Proposal-only role; it cannot mutate a repository or execute a tool.",
    x: 860,
    y: 190,
  },
  {
    id: "qa-validation",
    label: "QA & Validation Agent",
    groupId: "engineering",
    responsibility: "Advises on criteria coverage, gaps, and not-run checks.",
    authorityBoundary:
      "QA & Validation is not ApprovalManager and cannot approve or execute actions.",
    x: 1050,
    y: 190,
  },
  {
    id: "cloud-infrastructure",
    label: "Cloud Infrastructure Agent",
    groupId: "infrastructure",
    responsibility: "Assesses bounded synthetic infrastructure fixtures without external access.",
    authorityBoundary: "Advisory proposal only; it cannot access cloud systems or apply changes.",
    x: 1260,
    y: 190,
  },
  {
    id: "systems-operations",
    label: "Systems Operations Agent",
    groupId: "infrastructure",
    responsibility: "Assesses sanitized service fixtures and proposed recovery steps.",
    authorityBoundary: "Advisory proposal only; it cannot access services or execute recovery.",
    x: 1450,
    y: 190,
  },
  {
    id: "security-risk",
    label: "Security & Risk Agent",
    groupId: "governance",
    responsibility: "Provides bounded advisory risk findings for fixture proposals.",
    authorityBoundary:
      "Security & Risk is not PolicyEngine and cannot authorize or execute remediation.",
    x: 1660,
    y: 190,
  },
] as const;

const GROUPS: readonly TopologyGroup[] = [
  {
    id: "core",
    label: "Core",
    description: "User-facing assistance and proposal-only workflow preparation.",
    demoOrigin: DEMO_ORIGIN,
  },
  {
    id: "intelligence",
    label: "Intelligence",
    description: "Research and approved knowledge-oriented advisory roles.",
    demoOrigin: DEMO_ORIGIN,
  },
  {
    id: "engineering",
    label: "Engineering",
    description: "Proposal preparation and advisory validation.",
    demoOrigin: DEMO_ORIGIN,
  },
  {
    id: "infrastructure",
    label: "Infrastructure",
    description: "Synthetic cloud and systems assessment roles.",
    demoOrigin: DEMO_ORIGIN,
  },
  {
    id: "governance",
    label: "Governance",
    description: "Advisory risk review kept separate from application policy authority.",
    demoOrigin: DEMO_ORIGIN,
  },
] as const;

type AgentStatusOverrides = Partial<Readonly<Record<CommandCenterAgentId, OperationalStatus>>>;
type AgentAssignmentOverrides = Partial<Readonly<Record<CommandCenterAgentId, string>>>;

interface ScenarioContent {
  readonly status: OperationalStatus;
  readonly orchestratorStatus: OperationalStatus;
  readonly agentStatuses?: AgentStatusOverrides;
  readonly agentAssignments?: AgentAssignmentOverrides;
  readonly workNodes: readonly WorkTopologyNode[];
  readonly edges: readonly TopologyEdge[];
  readonly events: readonly CommandCenterEvent[];
}

interface WorkNodeOptions {
  readonly scenarioId: CommandCenterScenarioId;
  readonly key: string;
  readonly kind: WorkTopologyNode["kind"];
  readonly label: string;
  readonly status: OperationalStatus;
  readonly agentId: CommandCenterAgentId | null;
  readonly approval?: ApprovalStatus;
  readonly responsibility: string;
  readonly authorityBoundary?: string;
  readonly assignment?: string | null;
  readonly inputs?: readonly string[];
  readonly outputs?: readonly string[];
  readonly dependencies?: readonly TopologyNodeId[];
  readonly facts?: readonly { readonly label: string; readonly value: string }[];
  readonly findings?: readonly string[];
  readonly unresolvedIssues?: readonly string[];
  readonly x: number;
  readonly y: number;
}

interface EventOptions {
  readonly scenarioId: CommandCenterScenarioId;
  readonly ordinal: number;
  readonly kind: CommandCenterEventKind;
  readonly severity: CommandCenterEventSeverity;
  readonly summary: string;
  readonly detail: string;
  readonly agentId?: CommandCenterAgentId | null;
  readonly taskNodeId?: TopologyNodeId | null;
  readonly workflowNodeId?: TopologyNodeId | null;
  readonly relatedNodeIds: readonly TopologyNodeId[];
}

const orchestratorNodeId = (): TopologyNodeId => "demo-node:orchestrator";
const agentNodeId = (agentId: CommandCenterAgentId): TopologyNodeId => `demo-node:agent:${agentId}`;
const workNodeId = (scenarioId: CommandCenterScenarioId, key: string): TopologyNodeId =>
  `demo-node:${scenarioId}:${key}`;
const edgeId = (scenarioId: CommandCenterScenarioId, key: string): TopologyEdgeId =>
  `demo-edge:${scenarioId}:${key}`;
const eventId = (scenarioId: CommandCenterScenarioId, ordinal: number): CommandCenterEventId =>
  `demo-event:${scenarioId}:${String(ordinal).padStart(2, "0")}`;

function inspector(
  entityId: TopologyNodeId,
  entityKind: TopologyNodeKind,
  options: Omit<InspectorProjection, "demoOrigin" | "entityId" | "entityKind">,
): InspectorProjection {
  return {
    entityId,
    entityKind,
    ...options,
    demoOrigin: DEMO_ORIGIN,
  };
}

function createOrchestratorNode(status: OperationalStatus): OrchestratorTopologyNode {
  const id = orchestratorNodeId();
  return {
    id,
    kind: "orchestrator",
    agentId: null,
    groupId: null,
    label: "AgentOrchestrator",
    status,
    availability: "available-in-fixture",
    health: "not-measured",
    trust: "application-authority",
    approval: "not-required",
    demoOrigin: DEMO_ORIGIN,
    position: { x: 790, y: 20 },
    inspector: inspector(id, "orchestrator", {
      eyebrow: "Application authority",
      title: "AgentOrchestrator",
      description: "A simulated projection of the bounded application-owned orchestration service.",
      responsibility:
        "Owns task lineage, runtime-run assignment, result routing, and cancellation.",
      authorityBoundary:
        "It is not a provider, policy engine, approval manager, tool executor, audit log, memory store, scheduler, or UI boundary.",
      assignment: status === "idle" ? null : "Coordinate the selected deterministic fixture path.",
      inputs: ["Application-owned fixture request"],
      outputs: ["Bounded presentation-only task and result state"],
      dependencyIds: [],
      facts: [
        { label: "Runtime", value: "Native boundary not connected to this projection" },
        { label: "Execution", value: "No action execution" },
      ],
      findings: [],
      unresolvedIssues: [],
    }),
  };
}

function createAgentNode(
  entry: AgentCatalogEntry,
  status: OperationalStatus,
  assignment: string | undefined,
): AgentTopologyNode {
  const id = agentNodeId(entry.id);
  return {
    id,
    kind: "agent",
    agentId: entry.id,
    groupId: entry.groupId,
    label: entry.label,
    status,
    availability: "available-in-fixture",
    health: "not-measured",
    trust: "advisory-agent",
    approval: "not-required",
    demoOrigin: DEMO_ORIGIN,
    position: { x: entry.x, y: entry.y },
    inspector: inspector(id, "agent", {
      eyebrow: `${entry.groupId} role`,
      title: entry.label,
      description: entry.responsibility,
      responsibility: entry.responsibility,
      authorityBoundary: entry.authorityBoundary,
      assignment: assignment ?? null,
      inputs: ["Bounded deterministic fixture data"],
      outputs: ["Advisory presentation finding"],
      dependencyIds: [],
      facts: [
        { label: "Availability", value: "Available only in this fixture" },
        { label: "Health", value: "Not measured" },
      ],
      findings: [],
      unresolvedIssues: [],
    }),
  };
}

function createWorkNode(options: WorkNodeOptions): WorkTopologyNode {
  const id = workNodeId(options.scenarioId, options.key);
  return {
    id,
    kind: options.kind,
    agentId: options.agentId,
    groupId: null,
    label: options.label,
    status: options.status,
    availability: "available-in-fixture",
    health: "not-measured",
    trust: "presentation-fixture",
    approval: options.approval ?? "not-required",
    demoOrigin: DEMO_ORIGIN,
    position: { x: options.x, y: options.y },
    inspector: inspector(id, options.kind, {
      eyebrow: "Simulated work",
      title: options.label,
      description: options.responsibility,
      responsibility: options.responsibility,
      authorityBoundary:
        options.authorityBoundary ??
        "Presentation-only fixture; application code retains every consequential authority.",
      assignment: options.assignment ?? null,
      inputs: options.inputs ?? ["Bounded fixture predecessor state"],
      outputs: options.outputs ?? ["Bounded presentation state"],
      dependencyIds: options.dependencies ?? [],
      facts: options.facts ?? [{ label: "Origin", value: "Deterministic frontend fixture" }],
      findings: options.findings ?? [],
      unresolvedIssues: options.unresolvedIssues ?? [],
    }),
  };
}

function createEdge(
  scenarioId: CommandCenterScenarioId,
  key: string,
  source: TopologyNodeId,
  target: TopologyNodeId,
  kind: TopologyEdgeKind,
  label: string,
  status: OperationalStatus,
): TopologyEdge {
  return {
    id: edgeId(scenarioId, key),
    source,
    target,
    kind,
    label,
    status,
    demoOrigin: DEMO_ORIGIN,
  };
}

function createEvent(options: EventOptions): CommandCenterEvent {
  const minute = String(options.ordinal).padStart(2, "0");
  return {
    id: eventId(options.scenarioId, options.ordinal),
    ordinal: options.ordinal,
    simulatedAt: `2026-01-15T09:${minute}:00.000Z`,
    timeLabel: COMMAND_CENTER_SIMULATED_TIME_LABEL,
    kind: options.kind,
    severity: options.severity,
    summary: options.summary,
    detail: options.detail,
    agentId: options.agentId ?? null,
    taskNodeId: options.taskNodeId ?? null,
    workflowNodeId: options.workflowNodeId ?? null,
    relatedNodeIds: options.relatedNodeIds,
    redaction: "presentation-safe",
    demoOrigin: DEMO_ORIGIN,
  };
}

function baseOrchestratorEdge(
  scenarioId: CommandCenterScenarioId,
  status: OperationalStatus,
): TopologyEdge {
  return createEdge(
    scenarioId,
    "orchestrator-personal-assistant",
    orchestratorNodeId(),
    agentNodeId("personal-assistant"),
    "orchestrates",
    "Owns the bounded root lifecycle",
    status,
  );
}

function catalogIdle(): ScenarioContent {
  const scenarioId = "catalog-idle";
  return {
    status: "idle",
    orchestratorStatus: "idle",
    workNodes: [],
    edges: [baseOrchestratorEdge(scenarioId, "idle")],
    events: [
      createEvent({
        scenarioId,
        ordinal: 1,
        kind: "catalog-loaded",
        severity: "info",
        summary: "Simulated catalog ready",
        detail: "All nine advisory roles are present; no work is selected.",
        relatedNodeIds: [orchestratorNodeId(), ...COMMAND_CENTER_AGENT_IDS.map(agentNodeId)],
      }),
    ],
  };
}

function researchQueued(): ScenarioContent {
  const scenarioId = "research-queued";
  const workflowId = workNodeId(scenarioId, "workflow");
  const taskId = workNodeId(scenarioId, "research-task");
  const workNodes = [
    createWorkNode({
      scenarioId,
      key: "workflow",
      kind: "workflow",
      label: "Queued research request",
      status: "queued",
      agentId: null,
      responsibility: "Represent one bounded queued root request.",
      assignment: "Await deterministic admission to the Research fixture lane.",
      x: 610,
      y: 390,
    }),
    createWorkNode({
      scenarioId,
      key: "research-task",
      kind: "task",
      label: "Research analysis",
      status: "queued",
      agentId: "research",
      responsibility: "Prepare an evidence-oriented fixture analysis.",
      assignment: "Queued; no runtime or provider work has begun.",
      dependencies: [workflowId],
      x: 820,
      y: 390,
    }),
  ] as const;
  return {
    status: "queued",
    orchestratorStatus: "delegating",
    agentStatuses: { research: "queued", "personal-assistant": "waiting-child" },
    agentAssignments: { research: "Queued fixture analysis" },
    workNodes,
    edges: [
      baseOrchestratorEdge(scenarioId, "delegating"),
      createEdge(
        scenarioId,
        "orchestrates-workflow",
        orchestratorNodeId(),
        workflowId,
        "orchestrates",
        "Creates bounded root state",
        "queued",
      ),
      createEdge(
        scenarioId,
        "delegates-research",
        orchestratorNodeId(),
        agentNodeId("research"),
        "delegates",
        "Assigns the queued specialist fixture",
        "queued",
      ),
      createEdge(
        scenarioId,
        "research-owns-task",
        taskId,
        agentNodeId("research"),
        "owns-task",
        "Assigned to Research Agent",
        "queued",
      ),
      createEdge(
        scenarioId,
        "task-depends-on-workflow",
        taskId,
        workflowId,
        "depends-on",
        "Waits for root admission",
        "queued",
      ),
    ],
    events: [
      createEvent({
        scenarioId,
        ordinal: 1,
        kind: "task-queued",
        severity: "info",
        summary: "Research fixture queued",
        detail: "A bounded presentation task is queued without provider execution.",
        agentId: "research",
        taskNodeId: taskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [workflowId, taskId, agentNodeId("research")],
      }),
    ],
  };
}

function researchKnowledgeActive(): ScenarioContent {
  const scenarioId = "research-knowledge-active";
  const workflowId = workNodeId(scenarioId, "workflow");
  const researchTaskId = workNodeId(scenarioId, "research-task");
  const knowledgeTaskId = workNodeId(scenarioId, "knowledge-task");
  const synthesisId = workNodeId(scenarioId, "synthesis");
  return {
    status: "running",
    orchestratorStatus: "running",
    agentStatuses: {
      "personal-assistant": "waiting-child",
      research: "running",
      "knowledge-document": "running",
    },
    agentAssignments: {
      research: "Independent approved fixture analysis",
      "knowledge-document": "Independent approved knowledge comparison",
    },
    workNodes: [
      createWorkNode({
        scenarioId,
        key: "workflow",
        kind: "workflow",
        label: "Bounded parallel analysis",
        status: "running",
        agentId: null,
        responsibility: "Retain independent fixture runs and accept their events interleaved.",
        facts: [
          { label: "Concurrency", value: "Same-thread event multiplexing" },
          { label: "Provider", value: "Not connected" },
        ],
        x: 590,
        y: 370,
      }),
      createWorkNode({
        scenarioId,
        key: "research-task",
        kind: "task",
        label: "Research findings",
        status: "running",
        agentId: "research",
        responsibility: "Produce the first independent bounded finding set.",
        findings: ["Fixture evidence comparison is in progress."],
        x: 790,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "knowledge-task",
        kind: "task",
        label: "Knowledge findings",
        status: "running",
        agentId: "knowledge-document",
        responsibility: "Produce the second independent bounded finding set.",
        findings: ["Approved fixture comparison is in progress."],
        x: 1010,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "synthesis",
        kind: "workflow-step",
        label: "Personal synthesis",
        status: "waiting-child",
        agentId: "personal-assistant",
        responsibility: "Preserve source status and synthesize ordered child results.",
        dependencies: [researchTaskId, knowledgeTaskId],
        unresolvedIssues: ["Both independent fixture results are still required."],
        x: 900,
        y: 650,
      }),
    ],
    edges: [
      baseOrchestratorEdge(scenarioId, "running"),
      createEdge(
        scenarioId,
        "orchestrates-workflow",
        orchestratorNodeId(),
        workflowId,
        "orchestrates",
        "Owns bounded fixture lifecycle",
        "running",
      ),
      createEdge(
        scenarioId,
        "delegates-research",
        orchestratorNodeId(),
        agentNodeId("research"),
        "delegates",
        "Assigns independent Research work",
        "running",
      ),
      createEdge(
        scenarioId,
        "delegates-knowledge",
        orchestratorNodeId(),
        agentNodeId("knowledge-document"),
        "delegates",
        "Assigns independent Knowledge work",
        "running",
      ),
      createEdge(
        scenarioId,
        "research-owns-task",
        researchTaskId,
        agentNodeId("research"),
        "owns-task",
        "Assigned to Research Agent",
        "running",
      ),
      createEdge(
        scenarioId,
        "knowledge-owns-task",
        knowledgeTaskId,
        agentNodeId("knowledge-document"),
        "owns-task",
        "Assigned to Knowledge & Document Agent",
        "running",
      ),
      createEdge(
        scenarioId,
        "synthesis-needs-research",
        synthesisId,
        researchTaskId,
        "depends-on",
        "Requires Research result",
        "waiting-child",
      ),
      createEdge(
        scenarioId,
        "synthesis-needs-knowledge",
        synthesisId,
        knowledgeTaskId,
        "depends-on",
        "Requires Knowledge result",
        "waiting-child",
      ),
      createEdge(
        scenarioId,
        "research-result",
        researchTaskId,
        agentNodeId("personal-assistant"),
        "result-flow",
        "Returns source-attributed findings",
        "running",
      ),
      createEdge(
        scenarioId,
        "knowledge-result",
        knowledgeTaskId,
        agentNodeId("personal-assistant"),
        "result-flow",
        "Returns source-attributed findings",
        "running",
      ),
    ],
    events: [
      createEvent({
        scenarioId,
        ordinal: 1,
        kind: "task-started",
        severity: "info",
        summary: "Research fixture started",
        detail: "The Research presentation lane entered its running state.",
        agentId: "research",
        taskNodeId: researchTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [researchTaskId, agentNodeId("research")],
      }),
      createEvent({
        scenarioId,
        ordinal: 2,
        kind: "task-started",
        severity: "info",
        summary: "Knowledge fixture started",
        detail: "The independent Knowledge presentation lane entered its running state.",
        agentId: "knowledge-document",
        taskNodeId: knowledgeTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [knowledgeTaskId, agentNodeId("knowledge-document")],
      }),
      createEvent({
        scenarioId,
        ordinal: 3,
        kind: "task-progress",
        severity: "info",
        summary: "Independent findings retained",
        detail: "Fixture events are interleaved; result ordering remains ordinal-based.",
        workflowNodeId: workflowId,
        relatedNodeIds: [researchTaskId, knowledgeTaskId, synthesisId],
      }),
    ],
  };
}

function engineeringWaitingApproval(): ScenarioContent {
  const scenarioId = "engineering-waiting-approval";
  const workflowId = workNodeId(scenarioId, "workflow");
  const codingTaskId = workNodeId(scenarioId, "coding-task");
  const validationId = workNodeId(scenarioId, "validation");
  const riskId = workNodeId(scenarioId, "risk");
  const approvalId = workNodeId(scenarioId, "approval");
  return {
    status: "waiting-approval",
    orchestratorStatus: "waiting-approval",
    agentStatuses: {
      "personal-assistant": "waiting-approval",
      coding: "completed",
      "qa-validation": "completed",
      "security-risk": "completed",
    },
    agentAssignments: {
      coding: "Completed proposal fixture",
      "qa-validation": "Completed advisory validation",
      "security-risk": "Completed advisory risk review",
    },
    workNodes: [
      createWorkNode({
        scenarioId,
        key: "workflow",
        kind: "workflow",
        label: "Engineering proposal review",
        status: "waiting-approval",
        agentId: null,
        responsibility:
          "Collect advisory engineering results before an application-owned decision.",
        x: 600,
        y: 370,
      }),
      createWorkNode({
        scenarioId,
        key: "coding-task",
        kind: "task",
        label: "Coding proposal",
        status: "completed",
        agentId: "coding",
        responsibility: "Prepare a bounded proposal without repository mutation.",
        findings: ["A proposal fixture is available for advisory review."],
        x: 760,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "validation",
        kind: "validation-checkpoint",
        label: "QA advisory review",
        status: "completed",
        agentId: "qa-validation",
        responsibility: "Report criteria coverage without approving the proposal.",
        dependencies: [codingTaskId],
        findings: ["Fixture criteria are covered; proposed checks remain not run."],
        x: 950,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "risk",
        kind: "security-risk-checkpoint",
        label: "Security advisory review",
        status: "completed",
        agentId: "security-risk",
        responsibility: "Report fixture risks without making a policy decision.",
        dependencies: [codingTaskId],
        findings: ["One bounded risk note accompanies the proposal."],
        x: 1140,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "approval",
        kind: "approval-checkpoint",
        label: "Application approval checkpoint",
        status: "waiting-approval",
        agentId: null,
        approval: "simulated-pending",
        responsibility: "Depict a simulated wait for application-owned approval authority.",
        authorityBoundary:
          "Only application approval code could decide; no Command Center control can approve.",
        dependencies: [validationId, riskId],
        unresolvedIssues: ["The simulated application decision remains pending."],
        x: 960,
        y: 650,
      }),
    ],
    edges: [
      baseOrchestratorEdge(scenarioId, "waiting-approval"),
      createEdge(
        scenarioId,
        "orchestrates-workflow",
        orchestratorNodeId(),
        workflowId,
        "orchestrates",
        "Owns the bounded review lifecycle",
        "waiting-approval",
      ),
      createEdge(
        scenarioId,
        "coding-owns-task",
        codingTaskId,
        agentNodeId("coding"),
        "owns-task",
        "Assigned to Coding Agent",
        "completed",
      ),
      createEdge(
        scenarioId,
        "qa-review",
        agentNodeId("qa-validation"),
        validationId,
        "validation-review",
        "Provides advisory validation",
        "completed",
      ),
      createEdge(
        scenarioId,
        "security-review",
        agentNodeId("security-risk"),
        riskId,
        "risk-review",
        "Provides advisory risk findings",
        "completed",
      ),
      createEdge(
        scenarioId,
        "validation-depends-on-coding",
        validationId,
        codingTaskId,
        "depends-on",
        "Reviews the coding proposal",
        "completed",
      ),
      createEdge(
        scenarioId,
        "risk-depends-on-coding",
        riskId,
        codingTaskId,
        "depends-on",
        "Reviews the coding proposal",
        "completed",
      ),
      createEdge(
        scenarioId,
        "approval-dependency",
        workflowId,
        approvalId,
        "approval-dependency",
        "Waits for application-owned approval",
        "waiting-approval",
      ),
      createEdge(
        scenarioId,
        "approval-depends-on-validation",
        approvalId,
        validationId,
        "depends-on",
        "Requires advisory validation",
        "waiting-approval",
      ),
      createEdge(
        scenarioId,
        "approval-depends-on-risk",
        approvalId,
        riskId,
        "depends-on",
        "Requires advisory risk findings",
        "waiting-approval",
      ),
    ],
    events: [
      createEvent({
        scenarioId,
        ordinal: 1,
        kind: "task-completed",
        severity: "success",
        summary: "Coding proposal retained",
        detail: "The fixture proposal completed without repository mutation.",
        agentId: "coding",
        taskNodeId: codingTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [codingTaskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 2,
        kind: "validation-completed",
        severity: "success",
        summary: "QA advisory review complete",
        detail: "QA reported fixture coverage and did not approve anything.",
        agentId: "qa-validation",
        workflowNodeId: workflowId,
        relatedNodeIds: [validationId],
      }),
      createEvent({
        scenarioId,
        ordinal: 3,
        kind: "risk-review-completed",
        severity: "warning",
        summary: "Security advisory review complete",
        detail: "Security reported bounded fixture risk without making policy.",
        agentId: "security-risk",
        workflowNodeId: workflowId,
        relatedNodeIds: [riskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 4,
        kind: "approval-required",
        severity: "warning",
        summary: "Simulated application decision pending",
        detail: "The presentation path waits; the Command Center exposes no approval action.",
        workflowNodeId: workflowId,
        relatedNodeIds: [approvalId, validationId, riskId],
      }),
    ],
  };
}

function infrastructureBlocked(): ScenarioContent {
  const scenarioId = "infrastructure-blocked";
  const workflowId = workNodeId(scenarioId, "workflow");
  const cloudTaskId = workNodeId(scenarioId, "cloud-task");
  const systemsTaskId = workNodeId(scenarioId, "systems-task");
  const riskId = workNodeId(scenarioId, "risk");
  const synthesisId = workNodeId(scenarioId, "synthesis");
  return {
    status: "blocked",
    orchestratorStatus: "blocked",
    agentStatuses: {
      "cloud-infrastructure": "completed",
      "systems-operations": "completed",
      "security-risk": "failed",
      "personal-assistant": "blocked",
    },
    workNodes: [
      createWorkNode({
        scenarioId,
        key: "workflow",
        kind: "workflow",
        label: "Infrastructure assessment",
        status: "blocked",
        agentId: null,
        responsibility: "Represent an explicit dependency-aware failure policy.",
        unresolvedIssues: ["Security prerequisite failed in the deterministic fixture."],
        x: 600,
        y: 370,
      }),
      createWorkNode({
        scenarioId,
        key: "cloud-task",
        kind: "task",
        label: "Cloud assessment",
        status: "completed",
        agentId: "cloud-infrastructure",
        responsibility: "Assess the synthetic cloud portion independently.",
        findings: ["The cloud fixture assessment completed."],
        x: 760,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "systems-task",
        kind: "task",
        label: "Systems assessment",
        status: "completed",
        agentId: "systems-operations",
        responsibility: "Assess the sanitized systems portion independently.",
        findings: ["The systems fixture assessment completed."],
        x: 950,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "risk",
        kind: "security-risk-checkpoint",
        label: "Security prerequisite",
        status: "failed",
        agentId: "security-risk",
        responsibility: "Provide an advisory risk result over both assessments.",
        dependencies: [cloudTaskId, systemsTaskId],
        unresolvedIssues: ["Fixture evidence is insufficient for the bounded risk conclusion."],
        x: 1140,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "synthesis",
        kind: "workflow-step",
        label: "Personal synthesis",
        status: "blocked",
        agentId: "personal-assistant",
        responsibility: "Preserve the failed source status instead of concealing it.",
        dependencies: [riskId],
        unresolvedIssues: ["Synthesis is blocked by the failed prerequisite."],
        x: 960,
        y: 650,
      }),
    ],
    edges: [
      baseOrchestratorEdge(scenarioId, "blocked"),
      createEdge(
        scenarioId,
        "orchestrates-workflow",
        orchestratorNodeId(),
        workflowId,
        "orchestrates",
        "Owns bounded failure handling",
        "blocked",
      ),
      createEdge(
        scenarioId,
        "cloud-owns-task",
        cloudTaskId,
        agentNodeId("cloud-infrastructure"),
        "owns-task",
        "Assigned to Cloud Infrastructure Agent",
        "completed",
      ),
      createEdge(
        scenarioId,
        "systems-owns-task",
        systemsTaskId,
        agentNodeId("systems-operations"),
        "owns-task",
        "Assigned to Systems Operations Agent",
        "completed",
      ),
      createEdge(
        scenarioId,
        "security-review",
        agentNodeId("security-risk"),
        riskId,
        "risk-review",
        "Reviews both independent findings",
        "failed",
      ),
      createEdge(
        scenarioId,
        "risk-depends-on-cloud",
        riskId,
        cloudTaskId,
        "depends-on",
        "Requires cloud findings",
        "failed",
      ),
      createEdge(
        scenarioId,
        "risk-depends-on-systems",
        riskId,
        systemsTaskId,
        "depends-on",
        "Requires systems findings",
        "failed",
      ),
      createEdge(
        scenarioId,
        "failure-propagates",
        riskId,
        synthesisId,
        "failure-propagation",
        "Blocks dependent synthesis only",
        "blocked",
      ),
      createEdge(
        scenarioId,
        "synthesis-depends-on-risk",
        synthesisId,
        riskId,
        "depends-on",
        "Requires successful risk review",
        "blocked",
      ),
    ],
    events: [
      createEvent({
        scenarioId,
        ordinal: 1,
        kind: "task-completed",
        severity: "success",
        summary: "Cloud fixture completed",
        detail: "The independent cloud assessment returned a bounded finding.",
        agentId: "cloud-infrastructure",
        taskNodeId: cloudTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [cloudTaskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 2,
        kind: "task-completed",
        severity: "success",
        summary: "Systems fixture completed",
        detail: "The independent systems assessment returned a bounded finding.",
        agentId: "systems-operations",
        taskNodeId: systemsTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [systemsTaskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 3,
        kind: "task-failed",
        severity: "danger",
        summary: "Security prerequisite failed",
        detail: "The advisory fixture result records the failure without authorizing remediation.",
        agentId: "security-risk",
        workflowNodeId: workflowId,
        relatedNodeIds: [riskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 4,
        kind: "workflow-blocked",
        severity: "warning",
        summary: "Dependent synthesis blocked",
        detail: "Only the dependent fixture step is blocked; completed findings remain visible.",
        workflowNodeId: workflowId,
        relatedNodeIds: [riskId, synthesisId],
      }),
    ],
  };
}

function workflowCancelled(): ScenarioContent {
  const scenarioId = "workflow-cancelled";
  const workflowId = workNodeId(scenarioId, "workflow");
  const rootTaskId = workNodeId(scenarioId, "root-task");
  const childTaskId = workNodeId(scenarioId, "child-task");
  return {
    status: "cancelled",
    orchestratorStatus: "cancelled",
    agentStatuses: { "personal-assistant": "cancelled", research: "cancelled" },
    workNodes: [
      createWorkNode({
        scenarioId,
        key: "workflow",
        kind: "workflow",
        label: "Cancelled workflow",
        status: "cancelled",
        agentId: null,
        responsibility: "Represent terminal root cancellation without orphan work.",
        x: 620,
        y: 370,
      }),
      createWorkNode({
        scenarioId,
        key: "root-task",
        kind: "task",
        label: "Personal root task",
        status: "cancelled",
        agentId: "personal-assistant",
        responsibility: "Carry the application-owned root cancellation state.",
        x: 800,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "child-task",
        kind: "task",
        label: "Research child task",
        status: "cancelled",
        agentId: "research",
        responsibility: "Terminate the active child when root cancellation propagates.",
        dependencies: [rootTaskId],
        x: 1010,
        y: 500,
      }),
    ],
    edges: [
      baseOrchestratorEdge(scenarioId, "cancelled"),
      createEdge(
        scenarioId,
        "orchestrator-cancels-workflow",
        orchestratorNodeId(),
        workflowId,
        "cancellation",
        "Cancels the bounded root workflow",
        "cancelled",
      ),
      createEdge(
        scenarioId,
        "orchestrator-cancels-root",
        orchestratorNodeId(),
        rootTaskId,
        "cancellation",
        "Cancels root task",
        "cancelled",
      ),
      createEdge(
        scenarioId,
        "root-cancels-child",
        rootTaskId,
        childTaskId,
        "cancellation",
        "Propagates cancellation to active child",
        "cancelled",
      ),
      createEdge(
        scenarioId,
        "root-owns-task",
        rootTaskId,
        agentNodeId("personal-assistant"),
        "owns-task",
        "Assigned to Personal Assistant",
        "cancelled",
      ),
      createEdge(
        scenarioId,
        "child-owns-task",
        childTaskId,
        agentNodeId("research"),
        "owns-task",
        "Assigned to Research Agent",
        "cancelled",
      ),
      createEdge(
        scenarioId,
        "child-depends-on-root",
        childTaskId,
        rootTaskId,
        "depends-on",
        "Bound to root lifecycle",
        "cancelled",
      ),
    ],
    events: [
      createEvent({
        scenarioId,
        ordinal: 1,
        kind: "task-started",
        severity: "info",
        summary: "Research child fixture started",
        detail: "The bounded child entered running state before cancellation.",
        agentId: "research",
        taskNodeId: childTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [rootTaskId, childTaskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 2,
        kind: "task-cancelled",
        severity: "warning",
        summary: "Root cancellation accepted",
        detail: "The application-owned fixture root entered terminal cancellation.",
        agentId: "personal-assistant",
        taskNodeId: rootTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [rootTaskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 3,
        kind: "task-cancelled",
        severity: "warning",
        summary: "Child cancellation propagated",
        detail: "The active child terminated; no successor fixture task exists.",
        agentId: "research",
        taskNodeId: childTaskId,
        workflowNodeId: workflowId,
        relatedNodeIds: [childTaskId],
      }),
      createEvent({
        scenarioId,
        ordinal: 4,
        kind: "workflow-cancelled",
        severity: "warning",
        summary: "Workflow terminally cancelled",
        detail: "All simulated work is terminal and no orphan task remains.",
        workflowNodeId: workflowId,
        relatedNodeIds: [workflowId, rootTaskId, childTaskId],
      }),
    ],
  };
}

function workflowCompleted(): ScenarioContent {
  const scenarioId = "workflow-completed";
  const workflowId = workNodeId(scenarioId, "workflow");
  const proposalId = workNodeId(scenarioId, "proposal");
  const validationId = workNodeId(scenarioId, "validation");
  const synthesisId = workNodeId(scenarioId, "synthesis");
  return {
    status: "completed",
    orchestratorStatus: "completed",
    agentStatuses: {
      "personal-assistant": "completed",
      "workflow-automation": "completed",
    },
    agentAssignments: { "workflow-automation": "Completed proposal-only fixture" },
    workNodes: [
      createWorkNode({
        scenarioId,
        key: "workflow",
        kind: "workflow",
        label: "Proposal-only workflow",
        status: "completed",
        agentId: null,
        responsibility: "Represent a completed closed workflow proposal without dispatch.",
        x: 600,
        y: 370,
      }),
      createWorkNode({
        scenarioId,
        key: "proposal",
        kind: "workflow-step",
        label: "Workflow proposal",
        status: "completed",
        agentId: "workflow-automation",
        responsibility: "Prepare a bounded inert workflow proposal.",
        outputs: ["Ordered proposal fixture"],
        x: 760,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "validation",
        kind: "workflow-step",
        label: "Application validation",
        status: "completed",
        agentId: null,
        responsibility: "Validate the closed fixture shape without executing it.",
        dependencies: [proposalId],
        x: 950,
        y: 500,
      }),
      createWorkNode({
        scenarioId,
        key: "synthesis",
        kind: "workflow-step",
        label: "Ordered result synthesis",
        status: "completed",
        agentId: "personal-assistant",
        responsibility: "Summarize the proposal status, findings, and unresolved issues.",
        dependencies: [validationId],
        findings: ["Proposal fixture is complete and remains inert."],
        x: 1140,
        y: 500,
      }),
    ],
    edges: [
      baseOrchestratorEdge(scenarioId, "completed"),
      createEdge(
        scenarioId,
        "orchestrates-workflow",
        orchestratorNodeId(),
        workflowId,
        "orchestrates",
        "Owns bounded proposal lifecycle",
        "completed",
      ),
      createEdge(
        scenarioId,
        "delegates-workflow-agent",
        orchestratorNodeId(),
        agentNodeId("workflow-automation"),
        "delegates",
        "Assigns proposal-only fixture work",
        "completed",
      ),
      createEdge(
        scenarioId,
        "validation-depends-on-proposal",
        validationId,
        proposalId,
        "depends-on",
        "Validates proposal shape",
        "completed",
      ),
      createEdge(
        scenarioId,
        "synthesis-depends-on-validation",
        synthesisId,
        validationId,
        "depends-on",
        "Preserves ordered results",
        "completed",
      ),
      createEdge(
        scenarioId,
        "proposal-result",
        proposalId,
        agentNodeId("personal-assistant"),
        "result-flow",
        "Returns proposal-only findings",
        "completed",
      ),
    ],
    events: [
      createEvent({
        scenarioId,
        ordinal: 1,
        kind: "task-completed",
        severity: "success",
        summary: "Workflow proposal prepared",
        detail: "The closed proposal fixture completed without scheduling or execution.",
        agentId: "workflow-automation",
        workflowNodeId: workflowId,
        relatedNodeIds: [proposalId],
      }),
      createEvent({
        scenarioId,
        ordinal: 2,
        kind: "dependency-satisfied",
        severity: "success",
        summary: "Proposal shape validated",
        detail: "Application-owned fixture validation satisfied the synthesis dependency.",
        workflowNodeId: workflowId,
        relatedNodeIds: [proposalId, validationId],
      }),
      createEvent({
        scenarioId,
        ordinal: 3,
        kind: "synthesis-started",
        severity: "info",
        summary: "Ordered synthesis started",
        detail: "Personal Assistant fixture synthesis preserved source order and status.",
        agentId: "personal-assistant",
        workflowNodeId: workflowId,
        relatedNodeIds: [synthesisId],
      }),
      createEvent({
        scenarioId,
        ordinal: 4,
        kind: "synthesis-completed",
        severity: "success",
        summary: "Ordered synthesis complete",
        detail: "The proposal-only result remains simulated, bounded, and non-executing.",
        agentId: "personal-assistant",
        workflowNodeId: workflowId,
        relatedNodeIds: [synthesisId],
      }),
      createEvent({
        scenarioId,
        ordinal: 5,
        kind: "workflow-completed",
        severity: "success",
        summary: "Proposal-only workflow complete",
        detail: "No task was scheduled, no tool executed, and no state persisted.",
        workflowNodeId: workflowId,
        relatedNodeIds: [workflowId, proposalId, validationId, synthesisId],
      }),
    ],
  };
}

function scenarioContent(scenarioId: CommandCenterScenarioId): ScenarioContent {
  switch (scenarioId) {
    case "catalog-idle":
      return catalogIdle();
    case "research-queued":
      return researchQueued();
    case "research-knowledge-active":
      return researchKnowledgeActive();
    case "engineering-waiting-approval":
      return engineeringWaitingApproval();
    case "infrastructure-blocked":
      return infrastructureBlocked();
    case "workflow-cancelled":
      return workflowCancelled();
    case "workflow-completed":
      return workflowCompleted();
  }
}

function scenarioMetadata(scenarioId: CommandCenterScenarioId): CommandCenterFixtureMetadata {
  const metadata = COMMAND_CENTER_FIXTURE_CATALOG.find((item) => item.id === scenarioId);
  if (metadata === undefined) {
    throw new Error("Closed Command Center fixture metadata is missing");
  }
  return metadata;
}

function createSummary(
  scenarioLabel: string,
  status: OperationalStatus,
  groups: readonly TopologyGroup[],
  nodes: readonly TopologyNode[],
  events: readonly CommandCenterEvent[],
): CommandCenterSummary {
  const workNodes = nodes.filter((node) => node.kind !== "agent" && node.kind !== "orchestrator");
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
  return {
    scenarioLabel,
    disclosure: COMMAND_CENTER_DISCLOSURE,
    status,
    agentCount: nodes.filter((node) => node.kind === "agent").length,
    groupCount: groups.length,
    workItemCount: workNodes.length,
    activeCount: workNodes.filter((node) => activeStatuses.includes(node.status)).length,
    attentionCount: workNodes.filter((node) => attentionStatuses.includes(node.status)).length,
    completedCount: workNodes.filter((node) => node.status === "completed").length,
    eventCount: events.length,
    demoOrigin: DEMO_ORIGIN,
  };
}

export function buildCommandCenterProjection(
  scenarioId: CommandCenterScenarioId,
): CommandCenterProjection {
  const metadata = scenarioMetadata(scenarioId);
  const content = scenarioContent(scenarioId);
  const groups = GROUPS.map((group) => ({ ...group }));
  const nodes: TopologyNode[] = [
    createOrchestratorNode(content.orchestratorStatus),
    ...AGENT_CATALOG.map((entry) =>
      createAgentNode(
        entry,
        content.agentStatuses?.[entry.id] ?? "idle",
        content.agentAssignments?.[entry.id],
      ),
    ),
    ...content.workNodes,
  ];
  const events = [...content.events];
  const projection: CommandCenterProjection = {
    version: COMMAND_CENTER_PROJECTION_VERSION,
    provenance: "deterministic-fixture",
    scenarioId,
    scenarioLabel: metadata.label,
    disclosure: COMMAND_CENTER_DISCLOSURE,
    groups,
    nodes,
    edges: [...content.edges],
    events,
    summary: createSummary(metadata.label, content.status, groups, nodes, events),
  };
  return finalizeCommandCenterProjection(projection);
}
