import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  createResearchKnowledgeDemoLifecycleClient,
  parseResearchKnowledgeDemoLifecycleSnapshot,
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT,
} from "./research-knowledge-demo-lifecycle-client";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn() }));

const invokeMock = vi.mocked(invoke);
const listenMock = vi.mocked(listen);
const unlistenMock = vi.fn();
let eventHandler: ((event: { event: string; id: number; payload: unknown }) => void) | undefined;

const fixed = {
  schemaVersion: "research-knowledge-demo-lifecycle-v1",
  scenarioId: "research-knowledge-demo-v1",
  disclosure: "DEMO MODE · SIMULATED AGENT DATA",
  proofBoundary:
    "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
  fixtureProvenance: "application-owned-synthetic-fixture",
};

const idle = {
  ...fixed,
  presentationEpoch: 0,
  revision: 0,
  state: "idle",
  journal: [],
};
const research = {
  ...fixed,
  presentationEpoch: 1,
  revision: 1,
  state: "research",
  journal: [{ revision: 1, kind: "research-started" }],
};
const knowledge = {
  ...fixed,
  presentationEpoch: 1,
  revision: 3,
  state: "knowledge",
  journal: [
    { revision: 1, kind: "research-started" },
    { revision: 2, kind: "research-completed" },
    { revision: 3, kind: "knowledge-started" },
  ],
};
const synthesis = {
  ...fixed,
  presentationEpoch: 1,
  revision: 5,
  state: "synthesis",
  journal: [
    ...knowledge.journal,
    { revision: 4, kind: "knowledge-completed" },
    { revision: 5, kind: "synthesis-started" },
  ],
};
const succeeded = {
  ...fixed,
  presentationEpoch: 1,
  revision: 6,
  state: "succeeded",
  journal: [...synthesis.journal, { revision: 6, kind: "completed" }],
};
const failed = {
  ...succeeded,
  state: "failed",
  journal: [...synthesis.journal, { revision: 6, kind: "failed" }],
};
const cancelledDuringResearch = {
  ...fixed,
  presentationEpoch: 1,
  revision: 2,
  state: "cancelled",
  journal: [...research.journal, { revision: 2, kind: "cancelled" }],
};
const cancelledDuringKnowledge = {
  ...fixed,
  presentationEpoch: 1,
  revision: 4,
  state: "cancelled",
  journal: [...knowledge.journal, { revision: 4, kind: "cancelled" }],
};
const cancelledDuringSynthesis = {
  ...fixed,
  presentationEpoch: 1,
  revision: 6,
  state: "cancelled",
  journal: [...synthesis.journal, { revision: 6, kind: "cancelled" }],
};
const cleanupBeforeResearch = {
  ...fixed,
  presentationEpoch: 1,
  revision: 1,
  state: "cleanup-pending",
  journal: [{ revision: 1, kind: "cleanup-pending" }],
};

function emit(payload: unknown): void {
  eventHandler?.({ event: RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT, id: 1, payload });
}

describe("parseResearchKnowledgeDemoLifecycleSnapshot", () => {
  it.each([
    idle,
    research,
    knowledge,
    synthesis,
    succeeded,
    failed,
    cancelledDuringResearch,
    cancelledDuringKnowledge,
    cancelledDuringSynthesis,
    cleanupBeforeResearch,
  ])("accepts and freezes an exact bounded snapshot", (value) => {
    const parsed = parseResearchKnowledgeDemoLifecycleSnapshot(value);

    expect(parsed).toEqual(value);
    expect(parsed).not.toBe(value);
    expect(Object.isFrozen(parsed)).toBe(true);
    expect(Object.isFrozen(parsed?.journal)).toBe(true);
    if (parsed?.journal[0] !== undefined) {
      expect(parsed.journal[0]).not.toBe(value.journal[0]);
      expect(Object.isFrozen(parsed.journal[0])).toBe(true);
    }
  });

  it.each([
    null,
    [],
    { ...research, extra: "rejected" },
    { ...research, schemaVersion: "v2" },
    { ...research, scenarioId: "caller-selected" },
    { ...research, disclosure: "live data" },
    { ...research, fixtureProvenance: "provider" },
    { ...research, presentationEpoch: -1 },
    { ...research, presentationEpoch: 0x1_0000_0000 },
    { ...research, revision: 9 },
    { ...research, state: "running" },
    { ...research, journal: "not-an-array" },
    { ...research, journal: [] },
    { ...research, journal: [{ revision: 2, kind: "research-started" }] },
    { ...research, journal: [{ revision: 1, kind: "provider-output" }] },
    { ...research, journal: [{ revision: 1, kind: "research-started", runId: "forged" }] },
    { ...research, state: "succeeded" },
    { ...idle, presentationEpoch: 1 },
    { ...research, presentationEpoch: 0 },
    {
      ...fixed,
      presentationEpoch: 1,
      revision: 2,
      state: "succeeded",
      journal: [
        { revision: 1, kind: "research-started" },
        { revision: 2, kind: "completed" },
      ],
    },
    {
      ...knowledge,
      journal: [
        { revision: 1, kind: "research-started" },
        { revision: 2, kind: "knowledge-completed" },
        { revision: 3, kind: "knowledge-started" },
      ],
    },
  ])("rejects malformed, widened, oversized, or forged native data", (value) => {
    expect(parseResearchKnowledgeDemoLifecycleSnapshot(value)).toBeUndefined();
  });
});

describe("createResearchKnowledgeDemoLifecycleClient", () => {
  beforeEach(() => {
    invokeMock.mockReset();
    listenMock.mockReset();
    unlistenMock.mockReset();
    eventHandler = undefined;
    listenMock.mockImplementation((_event, handler) => {
      eventHandler = handler;
      return Promise.resolve(unlistenMock);
    });
  });

  it("uses one exact listener and one no-argument request per explicit operation", async () => {
    invokeMock
      .mockResolvedValueOnce(idle)
      .mockResolvedValueOnce(research)
      .mockResolvedValueOnce(knowledge)
      .mockResolvedValueOnce(cancelledDuringKnowledge);
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());

    await client.snapshot();
    await client.start();
    await client.advance();
    await client.cancel();

    expect(listenMock).toHaveBeenCalledOnce();
    expect(listenMock.mock.calls[0]?.[0]).toBe(RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT);
    expect(invokeMock.mock.calls).toEqual([
      ["get_research_knowledge_demo_lifecycle_snapshot"],
      ["start_research_knowledge_demo_lifecycle"],
      ["advance_research_knowledge_demo_lifecycle"],
      ["cancel_research_knowledge_demo_lifecycle"],
    ]);
  });

  it("keeps direct notifications non-authoritative until an explicit snapshot response", async () => {
    invokeMock.mockResolvedValueOnce(research).mockResolvedValueOnce(knowledge);
    const onSnapshot = vi.fn();
    const client = await createResearchKnowledgeDemoLifecycleClient(onSnapshot);
    await client.snapshot();

    emit(research);
    emit(idle);
    emit(knowledge);

    expect(client.current).toEqual(research);
    expect(client.recoveryRequired).toBe(true);
    expect(onSnapshot).toHaveBeenCalledOnce();

    await client.snapshot();

    expect(client.current).toEqual(knowledge);
    expect(client.recoveryRequired).toBe(false);
    expect(onSnapshot).toHaveBeenCalledTimes(2);
  });

  it("requires an explicit snapshot after a gap or newer epoch event", async () => {
    invokeMock.mockResolvedValueOnce(research).mockResolvedValueOnce(synthesis);
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    await client.snapshot();

    emit(synthesis);
    expect(client.current).toEqual(research);
    expect(client.recoveryRequired).toBe(true);

    await client.snapshot();
    expect(client.current).toEqual(synthesis);
    expect(client.recoveryRequired).toBe(false);

    emit({ ...research, presentationEpoch: 2 });
    expect(client.current).toEqual(synthesis);
    expect(client.recoveryRequired).toBe(true);
  });

  it("ignores malformed notifications without changing client state", async () => {
    invokeMock.mockResolvedValueOnce(research);
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    await client.snapshot();

    emit({ ...knowledge, workflowId: "forged" });

    expect(client.current).toEqual(research);
    expect(client.recoveryRequired).toBe(false);
  });

  it("rejects a shape-valid but impossible notification journal", async () => {
    invokeMock.mockResolvedValueOnce(research);
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    await client.snapshot();

    emit({
      ...fixed,
      presentationEpoch: 1,
      revision: 2,
      state: "succeeded",
      journal: [
        { revision: 1, kind: "research-started" },
        { revision: 2, kind: "completed" },
      ],
    });

    expect(client.current).toEqual(research);
    expect(client.recoveryRequired).toBe(false);
  });

  it("marks parser-valid non-successor notifications for explicit recovery", async () => {
    invokeMock.mockResolvedValueOnce(research);
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    await client.snapshot();

    emit(succeeded);

    expect(client.current).toEqual(research);
    expect(client.recoveryRequired).toBe(true);
  });

  it.each([succeeded, failed, cancelledDuringSynthesis])(
    "never commits an unsolicited grammar-valid terminal notification",
    async (terminal) => {
      invokeMock.mockResolvedValueOnce(synthesis);
      const onSnapshot = vi.fn();
      const client = await createResearchKnowledgeDemoLifecycleClient(onSnapshot);
      await client.snapshot();

      emit(terminal);

      expect(client.current).toEqual(synthesis);
      expect(client.recoveryRequired).toBe(true);
      expect(onSnapshot).toHaveBeenCalledOnce();
    },
  );

  it("disposes the listener once and ignores later events", async () => {
    invokeMock.mockResolvedValueOnce(research);
    const onSnapshot = vi.fn();
    const client = await createResearchKnowledgeDemoLifecycleClient(onSnapshot);
    await client.snapshot();

    client.dispose();
    client.dispose();
    emit(knowledge);

    expect(unlistenMock).toHaveBeenCalledOnce();
    expect(client.current).toEqual(research);
    expect(onSnapshot).toHaveBeenCalledOnce();
  });

  it("closes listener, native, and malformed response failures to one error", async () => {
    listenMock.mockRejectedValueOnce(new Error("sensitive listener detail"));
    await expect(createResearchKnowledgeDemoLifecycleClient(vi.fn())).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );

    listenMock.mockImplementationOnce((_event, handler) => {
      eventHandler = handler;
      return Promise.resolve(unlistenMock);
    });
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    invokeMock.mockRejectedValueOnce(new Error("sensitive native detail"));
    await expect(client.start()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );

    invokeMock.mockResolvedValueOnce({ ...research, taskId: "forged" });
    await expect(client.snapshot()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );
  });

  it("rejects concurrent, stale, contradictory, and post-disposal requests", async () => {
    let resolveFirst: ((value: unknown) => void) | undefined;
    invokeMock.mockImplementationOnce(
      () =>
        new Promise((resolve) => {
          resolveFirst = resolve;
        }),
    );
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    const first = client.snapshot();

    await expect(client.start()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );
    resolveFirst?.(research);
    await expect(first).resolves.toEqual(research);

    invokeMock.mockResolvedValueOnce(idle);
    await expect(client.snapshot()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );

    invokeMock.mockResolvedValueOnce({
      ...research,
      state: "failed",
      journal: [{ revision: 1, kind: "failed" }],
    });
    await expect(client.snapshot()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );

    invokeMock.mockResolvedValueOnce(succeeded);
    await expect(client.advance()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );

    client.dispose();
    await expect(client.snapshot()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );
  });

  it("rejects parser-valid responses that contradict the requested operation", async () => {
    invokeMock.mockResolvedValueOnce(idle).mockResolvedValueOnce(knowledge);
    const client = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    await client.snapshot();

    await expect(client.start()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );

    invokeMock.mockResolvedValueOnce(idle);
    const secondClient = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    await secondClient.snapshot();
    invokeMock.mockResolvedValueOnce(research);
    await secondClient.start();
    invokeMock.mockResolvedValueOnce(synthesis);
    await expect(secondClient.cancel()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );

    invokeMock.mockResolvedValueOnce(research);
    const thirdClient = await createResearchKnowledgeDemoLifecycleClient(vi.fn());
    await thirdClient.snapshot();
    invokeMock.mockResolvedValueOnce(cancelledDuringSynthesis);
    await expect(thirdClient.cancel()).rejects.toEqual(
      new Error("Research/Knowledge demo lifecycle unavailable."),
    );
    expect(thirdClient.current).toEqual(research);
  });
});
