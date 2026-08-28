import { invoke } from "@tauri-apps/api/core";
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  fetchResearchKnowledgeDemoProjection,
  parseResearchKnowledgeDemoProjection,
} from "./research-knowledge-demo-projection-client";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const invokeMock = vi.mocked(invoke);

const valid = {
  schemaVersion: "research-knowledge-demo-projection-v1",
  scenarioId: "research-knowledge-demo-v1",
  disclosure: "DEMO MODE · SIMULATED AGENT DATA",
  proofBoundary:
    "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
  fixtureProvenance: "application-owned-synthetic-fixture",
  roles: [
    { id: "personal-assistant", label: "Personal Assistant", state: "ready" },
    { id: "research", label: "Research Agent", state: "ready" },
    { id: "knowledge-document", label: "Knowledge & Document Agent", state: "ready" },
  ],
  simulatedOutcomes: ["succeeded", "failed", "cancelled"],
};

describe("parseResearchKnowledgeDemoProjection", () => {
  it("accepts the exact v1 projection and creates fresh nested values", () => {
    const parsed = parseResearchKnowledgeDemoProjection(valid);

    expect(parsed).toEqual(valid);
    expect(parsed).not.toBe(valid);
    expect(parsed?.roles).not.toBe(valid.roles);
    expect(parsed?.roles[0]).not.toBe(valid.roles[0]);
    expect(parsed?.simulatedOutcomes).not.toBe(valid.simulatedOutcomes);
    expect(Object.isFrozen(parsed)).toBe(true);
    expect(Object.isFrozen(parsed?.roles)).toBe(true);
    expect(Object.isFrozen(parsed?.roles[0])).toBe(true);
    expect(Object.isFrozen(parsed?.simulatedOutcomes)).toBe(true);
  });

  it.each([
    null,
    [],
    Object.fromEntries(Object.entries(valid).filter(([key]) => key !== "scenarioId")),
    { ...valid, extra: "rejected" },
    { ...valid, schemaVersion: "v2" },
    { ...valid, scenarioId: "caller-selected" },
    { ...valid, disclosure: "live agent data" },
    { ...valid, proofBoundary: "x".repeat(161) },
    { ...valid, fixtureProvenance: "provider" },
    { ...valid, roles: "not-an-array" },
    { ...valid, roles: valid.roles.slice(0, 2) },
    {
      ...valid,
      roles: [{ ...valid.roles[0], state: "running" }, ...valid.roles.slice(1)],
    },
    {
      ...valid,
      roles: [{ ...valid.roles[0], taskId: "forged" }, ...valid.roles.slice(1)],
    },
    {
      ...valid,
      roles: [{ ...valid.roles[0], label: "🧠".repeat(129) }, ...valid.roles.slice(1)],
    },
    { ...valid, simulatedOutcomes: ["succeeded", "failed", "running"] },
    { error: "unavailable" },
  ])("rejects malformed, widened, or forged IPC data", (value) => {
    expect(parseResearchKnowledgeDemoProjection(value)).toBeUndefined();
  });
});

describe("fetchResearchKnowledgeDemoProjection", () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it("invokes only the exact no-argument projection command", async () => {
    invokeMock.mockResolvedValueOnce(valid);

    await expect(fetchResearchKnowledgeDemoProjection()).resolves.toEqual(valid);

    expect(invokeMock).toHaveBeenCalledOnce();
    expect(invokeMock).toHaveBeenCalledWith("get_research_knowledge_demo_projection");
  });

  it("closes a native rejection to one fixed error", async () => {
    invokeMock.mockRejectedValueOnce(new Error("sensitive native detail"));

    await expect(fetchResearchKnowledgeDemoProjection()).rejects.toEqual(
      new Error("Research/Knowledge demo projection unavailable."),
    );
  });

  it("closes a malformed native reply to the same fixed error", async () => {
    invokeMock.mockResolvedValueOnce({ ...valid, workflowId: "forged" });

    await expect(fetchResearchKnowledgeDemoProjection()).rejects.toEqual(
      new Error("Research/Knowledge demo projection unavailable."),
    );
  });
});
