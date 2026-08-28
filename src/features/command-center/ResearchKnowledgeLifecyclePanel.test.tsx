import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  createResearchKnowledgeDemoLifecycleClient,
  type ResearchKnowledgeDemoLifecycleClient,
  type ResearchKnowledgeDemoLifecycleSnapshot,
} from "../../infrastructure/tauri/research-knowledge-demo-lifecycle-client";
import { ResearchKnowledgeLifecyclePanel } from "./ResearchKnowledgeLifecyclePanel";

vi.mock("../../infrastructure/tauri/research-knowledge-demo-lifecycle-client", () => ({
  createResearchKnowledgeDemoLifecycleClient: vi.fn(),
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE: "DEMO MODE · SIMULATED AGENT DATA",
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY:
    "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE: "Research/Knowledge demo lifecycle unavailable.",
}));

type LifecycleState = ResearchKnowledgeDemoLifecycleSnapshot["state"];
type LifecycleEventKind = ResearchKnowledgeDemoLifecycleSnapshot["journal"][number]["kind"];
type SnapshotListener = (snapshot: ResearchKnowledgeDemoLifecycleSnapshot) => void;

const DISCLOSURE = "DEMO MODE · SIMULATED AGENT DATA";
const PROOF_BOUNDARY =
  "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.";
const CLOSED_UNAVAILABLE = "Research/Knowledge demo lifecycle unavailable.";

const JOURNAL_KINDS: Readonly<Record<LifecycleState, readonly LifecycleEventKind[]>> = {
  idle: [],
  research: ["research-started"],
  knowledge: ["research-started", "research-completed", "knowledge-started"],
  synthesis: [
    "research-started",
    "research-completed",
    "knowledge-started",
    "knowledge-completed",
    "synthesis-started",
  ],
  succeeded: [
    "research-started",
    "research-completed",
    "knowledge-started",
    "knowledge-completed",
    "synthesis-started",
    "completed",
  ],
  failed: [
    "research-started",
    "research-completed",
    "knowledge-started",
    "knowledge-completed",
    "synthesis-started",
    "failed",
  ],
  cancelled: ["research-started", "cancelled"],
  "cleanup-pending": ["research-started", "cleanup-pending"],
};

function lifecycleSnapshot(
  state: LifecycleState,
  presentationEpoch = state === "idle" ? 0 : 1,
): ResearchKnowledgeDemoLifecycleSnapshot {
  const journal = JOURNAL_KINDS[state].map((kind, index) => ({
    kind,
    revision: index + 1,
  }));
  return {
    schemaVersion: "research-knowledge-demo-lifecycle-v1",
    scenarioId: "research-knowledge-demo-v1",
    disclosure: DISCLOSURE,
    proofBoundary: PROOF_BOUNDARY,
    fixtureProvenance: "application-owned-synthetic-fixture",
    presentationEpoch,
    revision: journal.length,
    state,
    journal,
  };
}

function createClientHarness(initial: ResearchKnowledgeDemoLifecycleSnapshot) {
  let recoveryRequired = false;
  const snapshot = vi.fn(() => Promise.resolve(initial));
  const start = vi.fn(() => Promise.resolve(initial));
  const advance = vi.fn(() => Promise.resolve(initial));
  const cancel = vi.fn(() => Promise.resolve(initial));
  const dispose = vi.fn();
  const client: ResearchKnowledgeDemoLifecycleClient = {
    get current() {
      return initial;
    },
    get recoveryRequired() {
      return recoveryRequired;
    },
    advance,
    cancel,
    dispose,
    snapshot,
    start,
  };
  return {
    advance,
    cancel,
    client,
    dispose,
    setRecoveryRequired(value: boolean) {
      recoveryRequired = value;
    },
    snapshot,
    start,
  };
}

function deferred<T>() {
  let resolvePromise: (value: T | PromiseLike<T>) => void = () => {
    throw new Error("Deferred promise was not initialized.");
  };
  let rejectPromise: (reason?: unknown) => void = () => {
    throw new Error("Deferred promise was not initialized.");
  };
  const promise = new Promise<T>((resolve, reject) => {
    resolvePromise = resolve;
    rejectPromise = reject;
  });
  return { promise, reject: rejectPromise, resolve: resolvePromise };
}

const createClient = vi.mocked(createResearchKnowledgeDemoLifecycleClient);

beforeEach(() => {
  createClient.mockReset();
});

async function renderReady(initial: ResearchKnowledgeDemoLifecycleSnapshot) {
  const harness = createClientHarness(initial);
  let onSnapshot: SnapshotListener = () => {
    throw new Error("Lifecycle snapshot listener was not registered.");
  };
  createClient.mockImplementation((listener) => {
    onSnapshot = listener;
    return Promise.resolve(harness.client);
  });
  const view = render(<ResearchKnowledgeLifecyclePanel />);
  await waitFor(() => {
    expect(harness.snapshot).toHaveBeenCalledOnce();
  });
  return { ...harness, onSnapshot, view };
}

function lifecycleButtons() {
  return {
    advance: screen.getByRole("button", { name: "Advance simulated lifecycle" }),
    cancel: screen.getByRole("button", { name: "Cancel simulated lifecycle" }),
    start: screen.getByRole("button", { name: "Start simulated lifecycle" }),
  };
}

describe("ResearchKnowledgeLifecyclePanel", () => {
  it("hydrates one snapshot without automatically invoking a lifecycle operation", async () => {
    const initial = lifecycleSnapshot("research");
    const harness = await renderReady(initial);
    const panel = screen.getByRole("region", {
      name: "Research/Knowledge simulated lifecycle",
    });

    expect(createClient).toHaveBeenCalledOnce();
    expect(harness.snapshot).toHaveBeenCalledOnce();
    expect(harness.start).not.toHaveBeenCalled();
    expect(harness.advance).not.toHaveBeenCalled();
    expect(harness.cancel).not.toHaveBeenCalled();
    expect(within(panel).getByText(DISCLOSURE)).toBeVisible();
    expect(within(panel).getByText(PROOF_BOUNDARY)).toBeVisible();
    expect(within(panel).getByText("Epoch").nextElementSibling).toHaveTextContent("1");
    expect(within(panel).getByText("Revision").nextElementSibling).toHaveTextContent("1");
    expect(within(panel).getByText("State").nextElementSibling).toHaveTextContent("Research");

    const liveStatus = within(panel).getByText(
      "Application-owned synthetic Research stage active.",
    );
    expect(liveStatus).toHaveAttribute("aria-live", "polite");
    expect(liveStatus).toHaveAttribute("aria-atomic", "true");
    expect(within(panel).getByRole("group", { name: "Simulated lifecycle actions" })).toBeVisible();
    expect(
      within(panel).getByRole("list", { name: "Simulated Rust lifecycle activity" }),
    ).toHaveTextContent("Research stage startedRevision 1");
    expect(
      within(panel)
        .getAllByRole("button")
        .map((button) => button.textContent),
    ).toEqual([
      "Start simulated lifecycle",
      "Advance simulated lifecycle",
      "Cancel simulated lifecycle",
    ]);
  });

  it("keeps all controls closed while the initial client is connecting", () => {
    const pendingClient = deferred<ResearchKnowledgeDemoLifecycleClient>();
    createClient.mockReturnValue(pendingClient.promise);
    render(<ResearchKnowledgeLifecyclePanel />);

    const panel = screen.getByRole("region", {
      name: "Research/Knowledge simulated lifecycle",
    });
    expect(panel).toHaveAttribute("aria-busy", "true");
    expect(
      within(panel).getByText("Loading the current volatile lifecycle snapshot."),
    ).toHaveAttribute("aria-live", "polite");
    for (const button of within(panel).getAllByRole("button")) expect(button).toBeDisabled();
  });

  it.each([
    ["idle", 0, true, false, false],
    ["research", 1, false, true, true],
    ["knowledge", 1, false, true, true],
    ["synthesis", 1, false, true, true],
    ["succeeded", 1, true, false, false],
    ["failed", 1, true, false, false],
    ["cancelled", 1, true, false, false],
    ["cleanup-pending", 1, false, false, false],
    ["idle", 0xffff_ffff, false, false, false],
  ] as const)(
    "applies the closed control matrix for %s at epoch %i",
    async (state, epoch, startEnabled, advanceEnabled, cancelEnabled) => {
      await renderReady(lifecycleSnapshot(state, epoch));
      const buttons = lifecycleButtons();

      expect(buttons.start).toHaveProperty("disabled", !startEnabled);
      expect(buttons.advance).toHaveProperty("disabled", !advanceEnabled);
      expect(buttons.cancel).toHaveProperty("disabled", !cancelEnabled);
    },
  );

  it("reaches success only through explicit no-argument start and advance actions", async () => {
    const harness = await renderReady(lifecycleSnapshot("idle"));
    harness.start.mockResolvedValueOnce(lifecycleSnapshot("research"));
    harness.advance
      .mockResolvedValueOnce(lifecycleSnapshot("knowledge"))
      .mockResolvedValueOnce(lifecycleSnapshot("synthesis"))
      .mockResolvedValueOnce(lifecycleSnapshot("succeeded"));

    fireEvent.click(lifecycleButtons().start);
    await screen.findByText("Application-owned synthetic Research stage active.");
    for (const expectedStatus of [
      "Application-owned synthetic Knowledge stage active.",
      "Application-owned synthetic synthesis stage active.",
      "Application-owned synthetic rehearsal completed.",
    ]) {
      fireEvent.click(lifecycleButtons().advance);
      await screen.findByText(expectedStatus);
    }

    expect(harness.start).toHaveBeenCalledWith();
    expect(harness.advance).toHaveBeenCalledTimes(3);
    for (const call of harness.advance.mock.calls) expect(call).toHaveLength(0);
    expect(lifecycleButtons().start).toBeEnabled();
    expect(lifecycleButtons().advance).toBeDisabled();
    expect(lifecycleButtons().cancel).toBeDisabled();
  });

  it("labels an explicit synthetic failure without implying an external failure", async () => {
    const harness = await renderReady(lifecycleSnapshot("synthesis"));
    harness.advance.mockResolvedValueOnce(lifecycleSnapshot("failed"));

    fireEvent.click(lifecycleButtons().advance);

    expect(await screen.findByText("Application-owned synthetic rehearsal failed.")).toBeVisible();
    expect(
      screen.getByText(
        "This is an application-owned synthetic rehearsal outcome, not a provider, model, tool, approval, policy, audit, or device failure.",
      ),
    ).toBeVisible();
    expect(harness.advance).toHaveBeenCalledWith();
    expect(lifecycleButtons().start).toBeEnabled();
    expect(lifecycleButtons().advance).toBeDisabled();
    expect(lifecycleButtons().cancel).toBeDisabled();
  });

  it("cancels an active epoch only through the explicit no-argument cancel action", async () => {
    const harness = await renderReady(lifecycleSnapshot("knowledge"));
    harness.cancel.mockResolvedValueOnce(lifecycleSnapshot("cancelled"));

    fireEvent.click(lifecycleButtons().cancel);

    expect(
      await screen.findByText("Application-owned synthetic rehearsal cancelled."),
    ).toBeVisible();
    expect(harness.cancel).toHaveBeenCalledWith();
    expect(harness.start).not.toHaveBeenCalled();
    expect(harness.advance).not.toHaveBeenCalled();
    expect(lifecycleButtons().start).toBeEnabled();
  });

  it("keeps an in-flight action busy when an accepted response updates the visible snapshot", async () => {
    const operation = deferred<ResearchKnowledgeDemoLifecycleSnapshot>();
    const harness = await renderReady(lifecycleSnapshot("research"));
    harness.advance.mockReturnValueOnce(operation.promise);

    fireEvent.click(lifecycleButtons().advance);
    const panel = screen.getByRole("region", {
      name: "Research/Knowledge simulated lifecycle",
    });
    expect(panel).toHaveAttribute("aria-busy", "true");
    expect(screen.getByText("Advancing the simulated lifecycle.")).toBeVisible();
    for (const button of lifecycleButtonsAsArray()) expect(button).toBeDisabled();

    act(() => {
      harness.onSnapshot(lifecycleSnapshot("knowledge"));
    });

    expect(panel).toHaveAttribute("aria-busy", "true");
    expect(screen.getByText("Advancing the simulated lifecycle.")).toBeVisible();
    for (const button of lifecycleButtonsAsArray()) expect(button).toBeDisabled();
    fireEvent.click(lifecycleButtons().start);
    fireEvent.click(lifecycleButtons().cancel);
    expect(harness.start).not.toHaveBeenCalled();
    expect(harness.cancel).not.toHaveBeenCalled();

    await act(async () => {
      operation.resolve(lifecycleSnapshot("knowledge"));
      await operation.promise;
    });
    expect(panel).toHaveAttribute("aria-busy", "false");
    expect(screen.getByText("Application-owned synthetic Knowledge stage active.")).toBeVisible();
    expect(harness.advance).toHaveBeenCalledOnce();
  });

  it("fails closed and disposes before an operation when recovery is required", async () => {
    const harness = await renderReady(lifecycleSnapshot("idle"));
    harness.setRecoveryRequired(true);

    fireEvent.click(lifecycleButtons().start);

    expect(await screen.findByText(CLOSED_UNAVAILABLE)).toBeVisible();
    expect(harness.start).not.toHaveBeenCalled();
    expect(harness.dispose).toHaveBeenCalledOnce();
    expect(screen.queryByText("Epoch")).not.toBeInTheDocument();
    for (const button of lifecycleButtonsAsArray()) expect(button).toBeDisabled();
  });

  it("closes a client-creation failure without exposing raw detail or retrying", async () => {
    createClient.mockRejectedValueOnce(new Error("sensitive listener registration detail"));
    render(<ResearchKnowledgeLifecyclePanel />);

    expect(await screen.findByText(CLOSED_UNAVAILABLE)).toBeVisible();
    expect(screen.queryByText("sensitive listener registration detail")).not.toBeInTheDocument();
    expect(screen.queryByText("Epoch")).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /retry/i })).not.toBeInTheDocument();
    for (const button of lifecycleButtonsAsArray()) expect(button).toBeDisabled();
    expect(createClient).toHaveBeenCalledOnce();
  });

  it("closes a snapshot failure, clears presentation state, and disposes without retry", async () => {
    const harness = createClientHarness(lifecycleSnapshot("research"));
    harness.snapshot.mockRejectedValueOnce(new Error("sensitive snapshot detail"));
    createClient.mockResolvedValueOnce(harness.client);
    render(<ResearchKnowledgeLifecyclePanel />);

    expect(await screen.findByText(CLOSED_UNAVAILABLE)).toBeVisible();
    expect(screen.queryByText("sensitive snapshot detail")).not.toBeInTheDocument();
    expect(screen.queryByText("Epoch")).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /retry/i })).not.toBeInTheDocument();
    expect(harness.dispose).toHaveBeenCalledOnce();
    for (const button of lifecycleButtonsAsArray()) expect(button).toBeDisabled();
  });

  it("closes an operation failure, clears the stale snapshot, and disposes without retry", async () => {
    const harness = await renderReady(lifecycleSnapshot("research"));
    harness.advance.mockRejectedValueOnce(new Error("sensitive native operation detail"));

    fireEvent.click(lifecycleButtons().advance);

    expect(await screen.findByText(CLOSED_UNAVAILABLE)).toBeVisible();
    expect(screen.queryByText("sensitive native operation detail")).not.toBeInTheDocument();
    expect(screen.queryByText("Epoch")).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /retry/i })).not.toBeInTheDocument();
    expect(harness.dispose).toHaveBeenCalledOnce();
    for (const button of lifecycleButtonsAsArray()) expect(button).toBeDisabled();
    expect(createClient).toHaveBeenCalledOnce();
  });

  it("disposes the mounted client on unmount", async () => {
    const harness = await renderReady(lifecycleSnapshot("idle"));

    harness.view.unmount();

    expect(harness.dispose).toHaveBeenCalledOnce();
  });

  it("disposes a client that resolves after unmount without requesting its snapshot", async () => {
    const creation = deferred<ResearchKnowledgeDemoLifecycleClient>();
    const harness = createClientHarness(lifecycleSnapshot("idle"));
    createClient.mockReturnValueOnce(creation.promise);
    const view = render(<ResearchKnowledgeLifecyclePanel />);

    view.unmount();
    await act(async () => {
      creation.resolve(harness.client);
      await creation.promise;
    });

    await waitFor(() => {
      expect(harness.dispose).toHaveBeenCalledOnce();
    });
    expect(harness.snapshot).not.toHaveBeenCalled();
  });

  it("ignores late operation results and notifications after unmount", async () => {
    const operation = deferred<ResearchKnowledgeDemoLifecycleSnapshot>();
    const harness = await renderReady(lifecycleSnapshot("idle"));
    harness.start.mockReturnValueOnce(operation.promise);
    fireEvent.click(lifecycleButtons().start);

    harness.view.unmount();
    act(() => {
      harness.onSnapshot(lifecycleSnapshot("knowledge"));
    });
    await act(async () => {
      operation.resolve(lifecycleSnapshot("research"));
      await operation.promise;
    });

    expect(harness.dispose).toHaveBeenCalledOnce();
    expect(screen.queryByText("Application-owned synthetic Research stage active.")).toBeNull();
    expect(screen.queryByText("Application-owned synthetic Knowledge stage active.")).toBeNull();
  });
});

function lifecycleButtonsAsArray(): HTMLElement[] {
  const buttons = lifecycleButtons();
  return [buttons.start, buttons.advance, buttons.cancel];
}
