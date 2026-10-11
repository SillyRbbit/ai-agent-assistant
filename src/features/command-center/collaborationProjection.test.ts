import { describe, expect, it } from "vitest";
import { projectCollaboration, matchesCard } from "./collaborationProjection";
import {
  WORKFLOWS,
  type Run,
  type Workflow,
  type Status,
} from "../../infrastructure/tauri/collaboration-client";
import { DEFAULT_BOT_IDENTITY } from "../../infrastructure/tauri/agent-chat-client";
function fixture(workflow: Workflow): Run {
  return {
    id: "room-1/run-1",
    sequence: 9,
    status: "completed",
    error: null,
    input: {
      workflow,
      objective: "secret objective",
      sources: [{ label: "S1", text: "secret source" }],
    },
    stages: WORKFLOWS[workflow].map((agentId, i) => ({
      id: `room-1/run-1/stage/${String(i)}`,
      participant: {
        agentId,
        role: agentId,
        identity: { ...DEFAULT_BOT_IDENTITY, nickname: "界".repeat(48) },
        connection: "simulation",
        model: "default",
        effort: "default",
        endpoint: "secret endpoint",
        localAuth: false,
        ownerInstructions: "secret instruction",
        revision: 1,
      },
      status: "completed",
      timestamp: 123,
      provisional: "provisional",
      input: "secret transmitted input",
      handoff: {
        version: 1,
        stage: i,
        agentId,
        status: "complete",
        summary: "validated summary",
        findings: [],
        evidence: ["S1"],
        limitations: ["not factual proof"],
      },
    })),
  };
}
describe("safe operational projection", () => {
  it.each(Object.keys(WORKFLOWS) as Workflow[])(
    "projects %s with nine identities and distinct executions",
    (workflow) => {
      const run = fixture(workflow),
        p = projectCollaboration([], run);
      expect(p.cards.filter((c) => c.kind === "bot")).toHaveLength(9);
      expect(p.cards.filter((c) => c.kind === "stage")).toHaveLength(WORKFLOWS[workflow].length);
      expect(new Set(p.cards.map((c) => c.id)).size).toBe(p.cards.length);
      expect(
        p.cards.filter((c) => c.kind === "stage" && c.agentId === "personal-assistant"),
      ).toHaveLength(workflow === "coding_action" ? 0 : 2);
      expect(p.links.filter((e) => e.kind === "handoff")).toHaveLength(run.stages.length - 1);
      expect(p.provenance).toContain("Simulation");
      expect(JSON.stringify(p)).not.toContain("secret");
      expect(p.cards[0]?.role).toBe("Application coordinator");
    },
  );
  it.each([
    "queued",
    "running",
    "completed",
    "failed",
    "cancelled",
    "interrupted",
    "partial",
  ] as Status[])("preserves actual %s state", (status) => {
    const r = fixture("workflow");
    const p = projectCollaboration([], {
      ...r,
      status,
      stages: r.stages.map((s) => ({ ...s, status })),
    });
    expect(p.cards[0]?.status).toBe(status);
    expect(p.cards.filter((c) => c.kind === "stage").every((c) => c.status === status)).toBe(true);
  });
  it("does not describe live configuration as successful or replace saved identities", () => {
    const r = fixture("research");
    const p = projectCollaboration([], {
      ...r,
      stages: r.stages.map((s) => ({
        ...s,
        participant: { ...s.participant, connection: "openai_api" },
      })),
    });
    expect(p.provenance).toContain("does not establish provider success");
    expect(p.cards.find((c) => c.kind === "stage")?.identity?.nickname).toBe("界".repeat(48));
  });
  it("does not claim a partial result reached an unstarted next stage", () => {
    const r = fixture("research");
    const p = projectCollaboration([], {
      ...r,
      stages: r.stages.map((s) => ({
        ...s,
        input: "",
        handoff: s.handoff ? { ...s.handoff, status: "partial" } : null,
      })),
    });
    expect(p.links.some((e) => e.kind === "handoff")).toBe(false);
  });
});

it("filters participant, domain, status and Unicode without mutating the accepted projection", () => {
  const p = projectCollaboration([], fixture("engineering"));
  const filters = {
    kind: "stage",
    status: "completed",
    group: "core",
    participant: "personal-assistant",
    search: "界",
  };
  expect(p.cards.filter((c) => matchesCard(c, filters))).toHaveLength(2);
  expect(p.cards.filter((c) => matchesCard(c, { ...filters, search: "no match" }))).toHaveLength(0);
  expect(p.cards.filter((c) => c.kind === "stage")).toHaveLength(5);
});

it("uses saved identity and settings when the current profile was renamed", () => {
  const run = fixture("research"),
    participant = run.stages[0]?.participant;
  if (!participant) throw Error("missing fixture");
  const current = {
    ...participant,
    displayName: "Personal Assistant",
    identity: { ...participant.identity, nickname: "Renamed current" },
    model: "different-current-model",
    note: "private-note",
    memoryMode: "off" as const,
    allowUnknownLocalityNotes: false,
  };
  const saved = projectCollaboration([current], run);
  expect(saved.cards.find((c) => c.id === "bot:personal-assistant")?.label).toBe("界".repeat(48));
  expect(JSON.stringify(saved)).not.toContain("Renamed current");
  expect(JSON.stringify(saved)).not.toContain("different-current-model");
  expect(JSON.stringify(saved)).not.toContain("private-note");
  expect(
    projectCollaboration([current], null).cards.find((c) => c.id === "bot:personal-assistant")
      ?.label,
  ).toBe("Renamed current");
});

it("projects actual Coding and QA states without exposing action source or paths", () => {
  const run = fixture("coding_action");
  const p = projectCollaboration([], run);
  expect(p.cards.filter((c) => c.kind === "stage").map((c) => c.agentId)).toEqual([
    "coding",
    "qa-validation",
  ]);
  expect(JSON.stringify(p)).not.toContain("secret");
});
