import { describe, expect, it } from "vitest";

import { buildCommandCenterProjection } from "./commandCenterFixtures";
import { COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE } from "./commandCenterEventPresentation";
import { buildCommandCenterOverview } from "./commandCenterOverview";
import {
  COMMAND_CENTER_AGENT_IDS,
  COMMAND_CENTER_DISCLOSURE,
  COMMAND_CENTER_SCENARIO_IDS,
  COMMAND_CENTER_SIMULATED_TIME_LABEL,
} from "./commandCenterProjection";

describe("buildCommandCenterOverview", () => {
  it.each(COMMAND_CENTER_SCENARIO_IDS)(
    "derives one deterministic nine-agent overview for %s",
    (scenarioId) => {
      const projection = buildCommandCenterProjection(scenarioId);
      const first = buildCommandCenterOverview(projection);
      const second = buildCommandCenterOverview(projection);
      const agentIds = first.agents.map((agent) => agent.agentId);
      const workNodeIds = new Set(
        projection.nodes
          .filter((node) => node.kind !== "agent" && node.kind !== "orchestrator")
          .map((node) => node.id),
      );

      expect(first).toEqual(second);
      expect(agentIds).toEqual(COMMAND_CENTER_AGENT_IDS);
      expect(new Set(agentIds).size).toBe(9);
      expect(first.agents).toHaveLength(9);
      expect(first.agents[0]).toMatchObject({
        entityKind: "agent-definition",
        toolAccess: "unavailable-in-projection",
      });
      expect(first.agents.every((agent) => agent.demoOrigin === "deterministic-fixture")).toBe(
        true,
      );
      expect(first.agents.every((agent) => agent.health === "not-measured")).toBe(true);
      expect(first.activeWork.every((item) => workNodeIds.has(item.nodeId))).toBe(true);
      expect(first.system.activeWorkCount).toBe(projection.summary.activeCount);
      expect(first.provenance).toBe("deterministic-fixture");
      expect(first.disclosure).toBe(COMMAND_CENTER_DISCLOSURE);
      expect(first.recentActivity[0]).toMatchObject({
        demoOrigin: "deterministic-fixture",
        timeLabel: COMMAND_CENTER_SIMULATED_TIME_LABEL,
      });
    },
  );

  it("keeps runtime, tool, and provider evidence explicitly unavailable", () => {
    const model = buildCommandCenterOverview(buildCommandCenterProjection("catalog-idle"));

    expect(model.system.sources).toEqual({
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
    });
    expect(model.system.runningAgentCount).toBe(0);
    expect(model.system.activeWorkCount).toBe(0);
    expect(model.activeWork).toEqual([]);
    expect(model.attention).toEqual([]);
  });

  it("preserves distinct task, workflow, checkpoint, approval, failure, and event facts", () => {
    const research = buildCommandCenterOverview(
      buildCommandCenterProjection("research-knowledge-active"),
    );
    expect(research.activeWork.some((item) => item.presentationKind === "task")).toBe(true);
    expect(research.activeWork.some((item) => item.presentationKind === "workflow")).toBe(true);
    expect(research.system.runningAgentCount).toBe(2);

    const approval = buildCommandCenterOverview(
      buildCommandCenterProjection("engineering-waiting-approval"),
    );
    expect(
      approval.activeWork.some(
        (item) => item.presentationKind === "checkpoint" && item.approval === "simulated-pending",
      ),
    ).toBe(true);
    expect(approval.attention).toContainEqual(
      expect.objectContaining({
        category: "approval",
        label: "Application approval checkpoint",
      }),
    );

    const failure = buildCommandCenterOverview(
      buildCommandCenterProjection("infrastructure-blocked"),
    );
    expect(failure.attention).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ category: "failure", label: "Security prerequisite" }),
        expect.objectContaining({ category: "blocked", label: "Personal synthesis" }),
      ]),
    );
    expect(failure.recentActivity[0]).toMatchObject({
      presentationKind: "event",
      summary: "Dependent synthesis blocked",
      timeLabel: COMMAND_CENTER_SIMULATED_TIME_LABEL,
    });
  });

  it("represents a projection without events as an explicit empty activity list", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const withoutEvents = {
      ...projection,
      events: [],
      summary: { ...projection.summary, eventCount: 0 },
    };

    expect(buildCommandCenterOverview(withoutEvents).recentActivity).toEqual([]);
  });

  it("keeps the QA advisory event presentation consistent without inferring source, target, or status", () => {
    const projection = buildCommandCenterProjection("engineering-waiting-approval");
    const model = buildCommandCenterOverview(projection);
    const qaEvent = model.recentActivity.find(
      (event) => event.summary === "QA advisory review complete",
    );
    const validationNode = projection.nodes.find((node) => node.label === "QA advisory review");

    expect(qaEvent).toMatchObject({
      associatedAgentLabel: "QA & Validation Agent",
      relatedEntityLabels: ["QA advisory review"],
      sourceLabel: COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
      statusLabel: COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
      targetLabel: COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
      taskLabel: null,
      workflowLabel: "Engineering proposal review",
    });
    expect(qaEvent?.selectionTargetId).toBe(validationNode?.id);
  });
});
