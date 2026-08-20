import { describe, expect, it } from "vitest";

import {
  COMMAND_CENTER_AGENT_IDS,
  COMMAND_CENTER_DISCLOSURE,
  COMMAND_CENTER_GROUP_IDS,
  COMMAND_CENTER_LIMITS,
  COMMAND_CENTER_SCENARIO_IDS,
  COMMAND_CENTER_SIMULATED_TIME_LABEL,
  CommandCenterProjectionValidationError,
  finalizeCommandCenterProjection,
  isCommandCenterScenarioId,
  validateCommandCenterProjection,
  type CommandCenterAgentId,
  type CommandCenterEvent,
  type CommandCenterEventId,
  type CommandCenterProjection,
  type ProjectionValidationCode,
  type TopologyEdge,
  type TopologyGroupId,
  type TopologyNode,
} from "./commandCenterProjection";
import {
  buildCommandCenterProjection,
  COMMAND_CENTER_FIXTURE_CATALOG,
  DEFAULT_COMMAND_CENTER_SCENARIO_ID,
} from "./commandCenterFixtures";

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

function validationCodes(projection: CommandCenterProjection): readonly ProjectionValidationCode[] {
  return validateCommandCenterProjection(projection).map((issue) => issue.code);
}

function expectValidationCode(
  projection: CommandCenterProjection,
  code: ProjectionValidationCode,
): void {
  expect(validationCodes(projection)).toContain(code);
}

function agentNode(projection: CommandCenterProjection, agentId: CommandCenterAgentId) {
  const node = projection.nodes.find(
    (candidate) => candidate.kind === "agent" && candidate.agentId === agentId,
  );
  if (node?.kind !== "agent") {
    throw new Error(`Missing fixture agent: ${agentId}`);
  }
  return node;
}

function workNode(projection: CommandCenterProjection, suffix: string) {
  const node = projection.nodes.find((candidate) => candidate.id.endsWith(`:${suffix}`));
  if (node === undefined) {
    throw new Error(`Missing fixture work node: ${suffix}`);
  }
  return node;
}

function expectDeepFrozen(value: unknown, seen = new WeakSet()): void {
  if (value === null || typeof value !== "object" || seen.has(value)) {
    return;
  }
  seen.add(value);
  expect(Object.isFrozen(value)).toBe(true);
  for (const child of Object.values(value)) {
    expectDeepFrozen(child, seen);
  }
}

describe("Command Center fixture catalog", () => {
  it("exposes the exact seven deterministic scenarios in approved order", () => {
    expect(DEFAULT_COMMAND_CENTER_SCENARIO_ID).toBe("catalog-idle");
    expect(COMMAND_CENTER_FIXTURE_CATALOG.map((scenario) => scenario.id)).toEqual(
      COMMAND_CENTER_SCENARIO_IDS,
    );
    expect(new Set(COMMAND_CENTER_FIXTURE_CATALOG.map((scenario) => scenario.id)).size).toBe(7);
  });

  it("narrows only closed scenario identifiers", () => {
    for (const scenarioId of COMMAND_CENTER_SCENARIO_IDS) {
      expect(isCommandCenterScenarioId(scenarioId)).toBe(true);
    }
    expect(isCommandCenterScenarioId("unknown-scenario")).toBe(false);
    expect(isCommandCenterScenarioId(7)).toBe(false);
    expect(isCommandCenterScenarioId(null)).toBe(false);
  });
});

describe("buildCommandCenterProjection", () => {
  it.each(COMMAND_CENTER_SCENARIO_IDS)(
    "builds a valid, deterministic, deeply immutable %s projection",
    (scenarioId) => {
      const first = buildCommandCenterProjection(scenarioId);
      const second = buildCommandCenterProjection(scenarioId);

      expect(first).toEqual(second);
      expect(first).not.toBe(second);
      expect(validateCommandCenterProjection(first)).toEqual([]);
      expectDeepFrozen(first);
      expect(first.scenarioId).toBe(scenarioId);
      expect(first.provenance).toBe("deterministic-fixture");
      expect(first.disclosure).toBe(COMMAND_CENTER_DISCLOSURE);
      expect(first.events.length).toBeLessThanOrEqual(COMMAND_CENTER_LIMITS.events);
    },
  );

  it.each(COMMAND_CENTER_SCENARIO_IDS)(
    "retains one distinct orchestrator, all nine agents, and five groups for %s",
    (scenarioId) => {
      const projection = buildCommandCenterProjection(scenarioId);
      const orchestrators = projection.nodes.filter((node) => node.kind === "orchestrator");
      const agents = projection.nodes.filter((node) => node.kind === "agent");

      expect(orchestrators).toHaveLength(1);
      expect(orchestrators[0]).toMatchObject({
        agentId: null,
        id: "demo-node:orchestrator",
        trust: "application-authority",
      });
      expect(agents).toHaveLength(9);
      expect(new Set(agents.map((node) => node.agentId))).toEqual(
        new Set(COMMAND_CENTER_AGENT_IDS),
      );
      expect(projection.groups.map((group) => group.id)).toEqual(COMMAND_CENTER_GROUP_IDS);

      for (const agentId of COMMAND_CENTER_AGENT_IDS) {
        const node = agentNode(projection, agentId);
        expect(node.groupId).toBe(EXPECTED_AGENT_GROUPS[agentId]);
        expect(node.trust).toBe("advisory-agent");
        expect(node.health).toBe("not-measured");
      }
    },
  );

  it.each(COMMAND_CENTER_SCENARIO_IDS)(
    "uses unique presentation-only IDs, valid endpoints, and finite coordinates for %s",
    (scenarioId) => {
      const projection = buildCommandCenterProjection(scenarioId);
      const nodeIds = projection.nodes.map((node) => node.id);
      const edgeIds = projection.edges.map((edge) => edge.id);
      const eventIds = projection.events.map((event) => event.id);
      const knownNodeIds = new Set(nodeIds);

      expect(new Set(nodeIds).size).toBe(nodeIds.length);
      expect(new Set(edgeIds).size).toBe(edgeIds.length);
      expect(new Set(eventIds).size).toBe(eventIds.length);
      expect(nodeIds.every((id) => id.startsWith("demo-node:"))).toBe(true);
      expect(edgeIds.every((id) => id.startsWith("demo-edge:"))).toBe(true);
      expect(eventIds.every((id) => id.startsWith("demo-event:"))).toBe(true);
      expect(
        projection.edges.every(
          (edge) => knownNodeIds.has(edge.source) && knownNodeIds.has(edge.target),
        ),
      ).toBe(true);
      expect(
        projection.nodes.every(
          (node) => Number.isFinite(node.position.x) && Number.isFinite(node.position.y),
        ),
      ).toBe(true);
    },
  );

  it.each(COMMAND_CENTER_SCENARIO_IDS)(
    "carries explicit demo provenance and deterministic simulated time for %s",
    (scenarioId) => {
      const projection = buildCommandCenterProjection(scenarioId);
      expect(projection.groups.every((group) => group.demoOrigin === "deterministic-fixture")).toBe(
        true,
      );
      expect(projection.nodes.every((node) => node.demoOrigin === "deterministic-fixture")).toBe(
        true,
      );
      expect(
        projection.nodes.every((node) => node.inspector.demoOrigin === "deterministic-fixture"),
      ).toBe(true);
      expect(projection.edges.every((edge) => edge.demoOrigin === "deterministic-fixture")).toBe(
        true,
      );
      expect(projection.events.every((event) => event.demoOrigin === "deterministic-fixture")).toBe(
        true,
      );
      expect(projection.summary.demoOrigin).toBe("deterministic-fixture");
      expect(projection.events.map((event) => event.ordinal)).toEqual(
        projection.events.map((_, index) => index + 1),
      );
      expect(projection.events.map((event) => event.timeLabel)).toEqual(
        projection.events.map(() => COMMAND_CENTER_SIMULATED_TIME_LABEL),
      );
      expect(
        projection.events.every((event) => event.simulatedAt.startsWith("2026-01-15T09:")),
      ).toBe(true);
    },
  );

  it("represents the idle catalog without selected work or measured health", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    expect(projection.summary).toMatchObject({ status: "idle", workItemCount: 0 });
    expect(
      projection.nodes.every(
        (node) => node.kind === "orchestrator" || node.health === "not-measured",
      ),
    ).toBe(true);
  });

  it("represents a queued bounded root and Research assignment", () => {
    const projection = buildCommandCenterProjection("research-queued");
    expect(agentNode(projection, "research").status).toBe("queued");
    expect(workNode(projection, "research-task")).toMatchObject({
      agentId: "research",
      kind: "task",
      status: "queued",
    });
    expect(projection.edges.some((edge) => edge.kind === "delegates")).toBe(true);
  });

  it("labels Research and Knowledge work as independent same-thread fixture activity", () => {
    const projection = buildCommandCenterProjection("research-knowledge-active");
    expect(agentNode(projection, "research").status).toBe("running");
    expect(agentNode(projection, "knowledge-document").status).toBe("running");
    expect(workNode(projection, "workflow").inspector.facts).toContainEqual({
      label: "Concurrency",
      value: "Same-thread event multiplexing",
    });
    expect(projection.edges.filter((edge) => edge.kind === "result-flow")).toHaveLength(2);
    expect(workNode(projection, "synthesis").inspector.dependencyIds).toHaveLength(2);
  });

  it("keeps QA, Security, and the simulated approval checkpoint separate", () => {
    const projection = buildCommandCenterProjection("engineering-waiting-approval");
    expect(agentNode(projection, "qa-validation").inspector.authorityBoundary).toContain(
      "not ApprovalManager",
    );
    expect(agentNode(projection, "security-risk").inspector.authorityBoundary).toContain(
      "not PolicyEngine",
    );
    expect(workNode(projection, "approval")).toMatchObject({
      agentId: null,
      approval: "simulated-pending",
      kind: "approval-checkpoint",
      status: "waiting-approval",
    });
    expect(projection.edges.some((edge) => edge.kind === "approval-dependency")).toBe(true);
  });

  it("preserves an infrastructure prerequisite failure and blocked dependent work", () => {
    const projection = buildCommandCenterProjection("infrastructure-blocked");
    expect(workNode(projection, "risk").status).toBe("failed");
    expect(workNode(projection, "synthesis").status).toBe("blocked");
    expect(projection.edges.some((edge) => edge.kind === "failure-propagation")).toBe(true);
    expect(projection.summary.attentionCount).toBeGreaterThanOrEqual(2);
  });

  it("makes cancellation terminal and leaves no running or queued successor work", () => {
    const projection = buildCommandCenterProjection("workflow-cancelled");
    const workNodes = projection.nodes.filter(
      (node) => node.kind !== "agent" && node.kind !== "orchestrator",
    );
    expect(workNodes.every((node) => node.status === "cancelled")).toBe(true);
    expect(projection.edges.filter((edge) => edge.kind === "cancellation")).toHaveLength(3);
    expect(projection.events.at(-1)?.kind).toBe("workflow-cancelled");
  });

  it("completes a proposal-only workflow with stable ordered results", () => {
    const projection = buildCommandCenterProjection("workflow-completed");
    expect(agentNode(projection, "workflow-automation").inspector.authorityBoundary).toContain(
      "not AgentOrchestrator",
    );
    expect(projection.summary.status).toBe("completed");
    expect(projection.events.map((event) => event.kind)).toEqual([
      "task-completed",
      "dependency-satisfied",
      "synthesis-started",
      "synthesis-completed",
      "workflow-completed",
    ]);
    expect(workNode(projection, "synthesis").inspector.findings).toEqual([
      "Proposal fixture is complete and remains inert.",
    ]);
  });

  it.each(COMMAND_CENTER_SCENARIO_IDS)("derives summary counts for %s", (scenarioId) => {
    const projection = buildCommandCenterProjection(scenarioId);
    const workNodes = projection.nodes.filter(
      (node) => node.kind !== "agent" && node.kind !== "orchestrator",
    );
    expect(projection.summary).toMatchObject({
      agentCount: 9,
      groupCount: 5,
      workItemCount: workNodes.length,
      eventCount: projection.events.length,
      disclosure: COMMAND_CENTER_DISCLOSURE,
    });
  });

  it.each(COMMAND_CENTER_SCENARIO_IDS)(
    "contains no protected projection fields, raw paths, or ambiguous operational claims for %s",
    (scenarioId) => {
      const serialized = JSON.stringify(buildCommandCenterProjection(scenarioId));
      const protectedKeys = [
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
        "approvalSubject",
        "affectedData",
        "credential",
        "rawJson",
        "requestId",
        "responseId",
        "runId",
        "runtimeIdentity",
      ] as const;
      for (const key of protectedKeys) {
        expect(serialized).not.toContain(`"${key}":`);
      }
      expect(serialized).not.toMatch(/\bLIVE\b/u);
      expect(serialized).not.toContain("/Users/");
      expect(serialized).not.toContain("file://");
    },
  );
});

describe("validateCommandCenterProjection", () => {
  it("rejects duplicate presentation IDs", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const duplicate = projection.nodes[0];
    if (duplicate === undefined) {
      throw new Error("Expected orchestrator fixture");
    }
    expectValidationCode(
      { ...projection, nodes: [...projection.nodes, duplicate] },
      "duplicate-id",
    );
  });

  it("rejects missing and foreign edge endpoints", () => {
    const projection = buildCommandCenterProjection("research-queued");
    const edge = projection.edges[0];
    if (edge === undefined) {
      throw new Error("Expected fixture edge");
    }
    const foreignEdge: TopologyEdge = { ...edge, target: "demo-node:foreign" };
    expectValidationCode(
      { ...projection, edges: [foreignEdge, ...projection.edges.slice(1)] },
      "invalid-reference",
    );
  });

  it("rejects foreign group references", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const group = projection.groups[0];
    if (group === undefined) {
      throw new Error("Expected fixture group");
    }
    const foreignGroup = { ...group, id: "foreign" as TopologyGroupId };
    expectValidationCode(
      { ...projection, groups: [foreignGroup, ...projection.groups.slice(1)] },
      "invalid-reference",
    );
  });

  it("rejects edge kinds with invalid source or target semantics", () => {
    const projection = buildCommandCenterProjection("engineering-waiting-approval");
    const approval = workNode(projection, "approval");
    const invalidEdge: TopologyEdge = {
      id: "demo-edge:invalid-qa-authority",
      source: agentNode(projection, "coding").id,
      target: approval.id,
      kind: "validation-review",
      label: "Invalid fixture edge",
      status: "validating",
      demoOrigin: "deterministic-fixture",
    };
    expectValidationCode(
      { ...projection, edges: [...projection.edges, invalidEdge] },
      "authority-boundary",
    );
  });

  it("rejects dependency cycles", () => {
    const projection = buildCommandCenterProjection("workflow-completed");
    const proposal = workNode(projection, "proposal");
    const validation = workNode(projection, "validation");
    const cycleEdge: TopologyEdge = {
      id: "demo-edge:workflow-completed:cycle",
      source: proposal.id,
      target: validation.id,
      kind: "depends-on",
      label: "Invalid reverse dependency",
      status: "completed",
      demoOrigin: "deterministic-fixture",
    };
    expectValidationCode(
      { ...projection, edges: [...projection.edges, cycleEdge] },
      "dependency-cycle",
    );
  });

  it("rejects more than 48 fixture events", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const seed = projection.events[0];
    if (seed === undefined) {
      throw new Error("Expected fixture event");
    }
    const events: readonly CommandCenterEvent[] = Array.from(
      { length: COMMAND_CENTER_LIMITS.events + 1 },
      (_, index) => {
        const ordinal = index + 1;
        return {
          ...seed,
          id: `demo-event:limit:${String(ordinal)}` as CommandCenterEventId,
          ordinal,
          simulatedAt: `2026-01-15T09:${String(ordinal).padStart(2, "0")}:00.000Z`,
        };
      },
    );
    expectValidationCode(
      { ...projection, events, summary: { ...projection.summary, eventCount: events.length } },
      "limit-exceeded",
    );
  });

  it("rejects nondeterministic event order", () => {
    const projection = buildCommandCenterProjection("workflow-completed");
    const first = projection.events[0];
    if (first === undefined) {
      throw new Error("Expected fixture event");
    }
    const events = [{ ...first, ordinal: 2 }, ...projection.events.slice(1)];
    expectValidationCode({ ...projection, events }, "invalid-order");
  });

  it("rejects false provenance", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const group = projection.groups[0];
    if (group === undefined) {
      throw new Error("Expected fixture group");
    }
    expectValidationCode(
      {
        ...projection,
        groups: [{ ...group, demoOrigin: "frontend-mock" }, ...projection.groups.slice(1)],
      },
      "invalid-provenance",
    );
  });

  it("rejects authority-boundary impersonation", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const qa = agentNode(projection, "qa-validation");
    const compromisedQa: TopologyNode = {
      ...qa,
      inspector: { ...qa.inspector, authorityBoundary: "Approves fixture work" },
    };
    expectValidationCode(
      {
        ...projection,
        nodes: projection.nodes.map((node) => (node.id === qa.id ? compromisedQa : node)),
      },
      "authority-boundary",
    );
  });

  it("rejects protected content fields and raw paths", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const orchestrator = projection.nodes[0];
    if (orchestrator === undefined) {
      throw new Error("Expected orchestrator fixture");
    }
    const unsafeNode = {
      ...orchestrator,
      prompt: "file:///Users/example/private.txt",
    } as unknown as TopologyNode;
    expectValidationCode(
      {
        ...projection,
        nodes: [unsafeNode, ...projection.nodes.slice(1)],
      },
      "unsafe-content",
    );
  });

  it("rejects inspector references outside the projection", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const personal = agentNode(projection, "personal-assistant");
    const invalidPersonal: TopologyNode = {
      ...personal,
      inspector: {
        ...personal.inspector,
        dependencyIds: ["demo-node:foreign"],
      },
    };
    expectValidationCode(
      {
        ...projection,
        nodes: projection.nodes.map((node) => (node.id === personal.id ? invalidPersonal : node)),
      },
      "invalid-reference",
    );
  });

  it("rejects summary drift", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    expectValidationCode(
      { ...projection, summary: { ...projection.summary, agentCount: 8 } },
      "invalid-summary",
    );
  });

  it("throws a typed error when finalizing an invalid projection", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const invalid = {
      ...projection,
      events: Array.from({ length: COMMAND_CENTER_LIMITS.events + 1 }, (_, index) => {
        const ordinal = index + 1;
        return {
          ...projection.events[0],
          id: `demo-event:invalid:${String(ordinal)}` as CommandCenterEventId,
          ordinal,
        } as CommandCenterEvent;
      }),
    };
    expect(() => finalizeCommandCenterProjection(invalid)).toThrow(
      CommandCenterProjectionValidationError,
    );
  });
});
