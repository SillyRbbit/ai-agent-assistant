import { describe, it, expect } from "vitest";
import { parseRun, WORKFLOWS, type Run, type ActionEvidence } from "./collaboration-client";
import { DEFAULT_BOT_IDENTITY } from "./agent-chat-client";
export function evidence(): ActionEvidence {
  return {
    file: "solution.py",
    testFile: "test_solution.py",
    baseline: "a".repeat(64),
    original: "x=1",
    tests: "# synthetic",
    attempts: [],
    reviewHash: null,
    disposition: "prepared",
    recovery: "/synthetic/recovery",
    requests: 0,
  };
}
function actionRun(): Run {
  return {
    id: "room-1/run-1",
    sequence: 0,
    status: "queued",
    error: null,
    input: { workflow: "coding_action", objective: "Add two values", sources: [] },
    action: evidence(),
    stages: WORKFLOWS.coding_action.map((agentId, i) => ({
      id: `room-1/run-1/stage/${String(i)}`,
      participant: {
        agentId,
        role: agentId,
        identity: DEFAULT_BOT_IDENTITY,
        connection: "openai_api",
        model: "gpt-5.6-luna",
        effort: "low",
        endpoint: "",
        localAuth: false,
        ownerInstructions: "",
        revision: 1,
      },
      status: "queued",
      timestamp: 1,
      input: "",
      provisional: "",
      handoff: null,
    })),
  };
}
describe("isolated action IPC evidence", () => {
  it("accepts bound queued evidence without claiming validation", () => {
    expect(parseRun(actionRun()).action?.reviewHash).toBeNull();
  });
  it("rejects unsupported sources, fabricated approval and missing evidence", () => {
    const r = actionRun();
    expect(() => parseRun({ ...r, action: undefined })).toThrow();
    expect(() =>
      parseRun({ ...r, input: { ...r.input, sources: [{ label: "x", text: "unknown" }] } }),
    ).toThrow();
    expect(() => parseRun({ ...r, action: { ...evidence(), approved: true } })).toThrow();
  });
  it("rejects approval readiness without a passing executed check", () => {
    expect(() =>
      parseRun({ ...actionRun(), action: { ...evidence(), reviewHash: "b".repeat(64) } }),
    ).toThrow();
  });
  it("rejects excessive requests and attempts", () => {
    expect(() => parseRun({ ...actionRun(), action: { ...evidence(), requests: 5 } })).toThrow();
    expect(() =>
      parseRun({ ...actionRun(), action: { ...evidence(), attempts: [{}, {}, {}] } }),
    ).toThrow();
  });
  it("does not attach action authority to an analysis route", () => {
    const r = actionRun();
    expect(() => parseRun({ ...r, input: { ...r.input, workflow: "research" } })).toThrow();
  });
});
