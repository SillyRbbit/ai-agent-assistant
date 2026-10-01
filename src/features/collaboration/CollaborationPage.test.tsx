import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { knowledgeClient } from "../../infrastructure/tauri/knowledge-client";
import { CollaborationPage } from "./CollaborationPage";
import { DEFAULT_BOT_IDENTITY } from "../../infrastructure/tauri/agent-chat-client";
import {
  WORKFLOWS,
  parseRooms,
  newerRooms,
  type Room,
  type Run,
  type Workflow,
  type CollaborationClient,
} from "../../infrastructure/tauri/collaboration-client";
export function fixture(workflow: Workflow = "research"): Run {
  return {
    id: "room-1/run-1",
    input: {
      workflow,
      objective: "Assess synthetic sources",
      sources: [{ label: "S1", text: "<script>untrusted</script>" }],
    },
    status: "completed",
    sequence: 10,
    error: null,
    stages: WORKFLOWS[workflow].map((id, i) => ({
      id: `room-1/run-1/stage/${String(i)}`,
      participant: {
        agentId: id,
        role: id,
        identity: { ...DEFAULT_BOT_IDENTITY, nickname: `Name ${String(i)}` },
        connection: "simulation",
        model: "simulation",
        effort: "default",
        endpoint: "",
        localAuth: false,
        ownerInstructions: "",
        revision: 0,
      },
      status: "completed",
      timestamp: 1,
      provisional: "",
      input: "synthetic transmitted input",
      handoff: {
        version: 1,
        stage: i,
        agentId: id,
        status: "complete",
        summary: "Proposal only",
        findings: ["Proposed test"],
        evidence: ["S1"],
        limitations: ["No actions executed"],
      },
    })),
  };
}
function client(initial: Room[]): CollaborationClient {
  let rooms = initial;
  return {
    list: vi.fn<CollaborationClient["list"]>(() => Promise.resolve(rooms)),
    create: vi.fn<CollaborationClient["create"]>((title) => {
      const r = { id: 1, title, runs: [] };
      rooms = [r];
      return Promise.resolve(r);
    }),
    prepare: vi.fn<CollaborationClient["prepare"]>((roomId, input) =>
      Promise.resolve({
        roomId,
        previewSerial: 1,
        run: {
          ...fixture(input.workflow),
          input,
          status: "queued",
          stages: fixture(input.workflow).stages.map((s) => ({
            ...s,
            status: "queued",
            handoff: null,
          })),
        },
        maximumCalls: WORKFLOWS[input.workflow].length,
        simulation: true,
      }),
    ),
    start: vi.fn<CollaborationClient["start"]>((p) => {
      const r = { id: p.roomId, title: "Room", runs: [fixture(p.run.input.workflow)] };
      rooms = [r];
      return Promise.resolve(r);
    }),
    cancel: vi.fn<CollaborationClient["cancel"]>((id) =>
      Promise.resolve({ id, title: "Room", runs: [{ ...fixture(), status: "cancelled" }] }),
    ),
    delete: vi.fn<CollaborationClient["delete"]>(() => {
      rooms = [];
      return Promise.resolve();
    }),
  };
}
describe("Collaboration room", () => {
  it("invalidates acknowledgement after native stale-source rejection without retry", async () => {
    const c = client([]);
    vi.mocked(c.start).mockRejectedValue("stale_context");
    render(<CollaborationPage client={c} />);
    fireEvent.change(screen.getByLabelText("Room title"), { target: { value: "Room" } });
    fireEvent.click(screen.getByRole("button", { name: "Create room" }));
    await screen.findByLabelText("Objective");
    fireEvent.change(screen.getByLabelText("Objective"), { target: { value: "Assess sources" } });
    fireEvent.click(
      screen.getByRole("button", { name: "Check readiness and review transmission" }),
    );
    await screen.findByRole("button", { name: "Start simulation workflow" });
    fireEvent.click(screen.getByRole("checkbox"));
    fireEvent.click(screen.getByRole("button", { name: "Start simulation workflow" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Detach outdated sources");
    expect(
      screen.queryByRole("button", { name: "Start simulation workflow" }),
    ).not.toBeInTheDocument();
    expect(c.start).toHaveBeenCalledTimes(1);
    fireEvent.click(
      screen.getByRole("button", { name: "Check readiness and review transmission" }),
    );
    expect(await screen.findByRole("button", { name: "Start simulation workflow" })).toBeDisabled();
    expect(screen.getByRole("checkbox")).not.toBeChecked();
  });

  it.each(Object.keys(WORKFLOWS) as Workflow[])(
    "%s explicitly includes library sources and invalidates detached preview",
    async (workflow) => {
      const source = {
        label: "K1_V1_P1",
        text: "Synthetic library passage",
        origin: {
          documentId: 1,
          version: 1,
          passageId: 1,
          title: "Runbook",
          hash: "a".repeat(64),
          startLine: 1,
          endLine: 1,
        },
      };
      const search = vi.spyOn(knowledgeClient, "search").mockResolvedValue([{ source }]);
      const c = client([]);
      render(<CollaborationPage client={c} />);
      fireEvent.change(screen.getByLabelText("Room title"), { target: { value: "Room" } });
      fireEvent.click(screen.getByRole("button", { name: "Create room" }));
      await screen.findByLabelText("Objective");
      fireEvent.change(screen.getByLabelText("Workflow"), { target: { value: workflow } });
      fireEvent.change(screen.getByLabelText("Objective"), { target: { value: "Assess sources" } });
      fireEvent.change(screen.getByLabelText("Find library passages"), {
        target: { value: "synthetic" },
      });
      fireEvent.click(screen.getByRole("button", { name: "Find passages" }));
      fireEvent.click(await screen.findByRole("button", { name: "Select K1_V1_P1" }));
      fireEvent.click(
        screen.getByRole("button", { name: "Check readiness and review transmission" }),
      );
      await screen.findByRole("button", { name: "Start simulation workflow" });
      expect(c.prepare).toHaveBeenCalledWith(1, {
        workflow,
        objective: "Assess sources",
        sources: [source],
      });
      fireEvent.click(screen.getByRole("checkbox"));
      fireEvent.click(screen.getByRole("button", { name: "Detach K1_V1_P1" }));
      expect(
        screen.queryByRole("button", { name: "Start simulation workflow" }),
      ).not.toBeInTheDocument();
      expect(c.start).not.toHaveBeenCalled();
      search.mockRestore();
    },
  );

  it.each(Object.keys(WORKFLOWS) as Workflow[])(
    "renders the complete %s route and inspectable handoffs",
    async (workflow) => {
      const c = client([{ id: 1, title: "Room", runs: [fixture(workflow)] }]);
      render(<CollaborationPage client={c} />);
      fireEvent.click(await screen.findByRole("button", { name: "Room" }));
      expect(screen.getAllByText("Handoff details and transmitted input")).toHaveLength(
        WORKFLOWS[workflow].length,
      );
      expect(screen.getByText("Final synthesis")).toBeInTheDocument();
      expect(c.start).not.toHaveBeenCalled();
    },
  );
  it("requires preview and acknowledgement; submits exactly one run and deletes history", async () => {
    const c = client([]);
    render(<CollaborationPage client={c} />);
    fireEvent.change(screen.getByLabelText("Room title"), { target: { value: "Room" } });
    fireEvent.click(screen.getByRole("button", { name: "Create room" }));
    await screen.findByLabelText("Objective");
    fireEvent.change(screen.getByLabelText("Objective"), {
      target: { value: "Assess synthetic sources" },
    });
    fireEvent.click(
      screen.getByRole("button", { name: "Check readiness and review transmission" }),
    );
    const start = await screen.findByRole("button", { name: "Start simulation workflow" });
    expect(start).toBeDisabled();
    expect(screen.getByText(/Headless Codex may retry internally/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("checkbox"));
    fireEvent.click(start);
    await waitFor(() => {
      expect(c.start).toHaveBeenCalledTimes(1);
    });
    await screen.findByText("Final synthesis");
    fireEvent.click(screen.getByRole("button", { name: "Delete room and local history" }));
    await waitFor(() => {
      expect(c.delete).toHaveBeenCalledWith(1);
    });
  });
  it("shows interrupted and provisional state honestly and supports Stop", async () => {
    const run = {
      ...fixture(),
      status: "running" as const,
      stages: fixture().stages.map((s, i) => ({
        ...s,
        status: i === 0 ? ("running" as const) : ("queued" as const),
        handoff: null,
        provisional: i === 0 ? "unvalidated streamed fragment" : "",
      })),
    };
    const c = client([{ id: 1, title: "Active", runs: [run] }]);
    render(<CollaborationPage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: "Active" }));
    expect(screen.getByText(/Provisional output/)).toBeInTheDocument();
    expect(screen.queryByText("Final synthesis")).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Stop collaboration" }));
    await waitFor(() => {
      expect(c.cancel).toHaveBeenCalledWith(1);
    });
  });
  it("retains labeled provisional output after Stop and releases UI ownership", async () => {
    const fragment =
      "Debug Simulation cancellation QA: fixed synthetic provisional fragment; waiting for Stop. No provider call or validated handoff.";
    const base = fixture("workflow");
    const run: Run = {
      ...base,
      status: "running",
      stages: base.stages.map((s, i) => ({
        ...s,
        status: i === 0 ? "running" : "queued",
        handoff: null,
        provisional: i === 0 ? fragment : "",
      })),
    };
    let rooms: Room[] = [{ id: 1, title: "Cancellation fixture", runs: [run] }];
    const list = vi.fn<CollaborationClient["list"]>(() => Promise.resolve(rooms));
    const cancel = vi.fn<CollaborationClient["cancel"]>((id) => {
      const cancelledRoom: Room = {
        id,
        title: "Cancellation fixture",
        runs: [
          {
            ...run,
            status: "cancelled",
            sequence: run.sequence + 1,
            stages: run.stages.map((s) => ({ ...s, status: "cancelled" })),
          },
        ],
      };
      rooms = [cancelledRoom];
      return Promise.resolve(cancelledRoom);
    });
    const c: CollaborationClient = { ...client(rooms), list, cancel };
    render(<CollaborationPage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: "Cancellation fixture" }));
    expect(screen.getByText(fragment)).toBeInTheDocument();
    expect(screen.getByText(/owns the native session/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Stop collaboration" }));
    await screen.findByText("workflow · cancelled");
    expect(c.cancel).toHaveBeenCalledTimes(1);
    expect(c.cancel).toHaveBeenCalledWith(1);
    expect(screen.getByText(fragment)).toBeInTheDocument();
    expect(screen.getByText(/Provisional output/)).toBeInTheDocument();
    expect(screen.queryByText("Final synthesis")).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Stop collaboration" })).not.toBeInTheDocument();
    expect(screen.queryByText(/owns the native session/)).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Add source" })).toBeEnabled();
  });
  it("labels partial output without claiming final synthesis", async () => {
    const base = fixture();
    const run: Run = {
      ...base,
      status: "partial",
      stages: base.stages.map((s) => ({
        ...s,
        status: "partial",
        handoff: s.handoff ? { ...s.handoff, status: "partial" } : null,
      })),
    };
    render(<CollaborationPage client={client([{ id: 1, title: "Partial", runs: [run] }])} />);
    fireEvent.click(await screen.findByRole("button", { name: "Partial" }));
    expect(screen.queryByText("Final synthesis")).not.toBeInTheDocument();
    expect(screen.getAllByText("Partial handoff")).toHaveLength(4);
  });
  it("does not let an older initial list erase a newly created room", async () => {
    const c = client([]);
    let finish: ((rooms: readonly Room[]) => void) | undefined;
    vi.mocked(c.list).mockImplementationOnce(
      () =>
        new Promise((resolve) => {
          finish = resolve;
        }),
    );
    render(<CollaborationPage client={c} />);
    fireEvent.change(screen.getByLabelText("Room title"), { target: { value: "New" } });
    fireEvent.click(screen.getByRole("button", { name: "Create room" }));
    await screen.findByRole("heading", { name: "New" });
    finish?.([]);
    await waitFor(() => {
      expect(screen.getByRole("heading", { name: "New" })).toBeInTheDocument();
    });
  });
  it("rejects bad attribution and references and ignores stale snapshots", () => {
    const room = { id: 1, title: "Room", runs: [fixture()] };
    expect(parseRooms([room])).toHaveLength(1);
    expect(newerRooms([room], [{ ...room, runs: [] }])).toEqual([room]);
    const bad = JSON.parse(
      JSON.stringify(room).replace("room-1/run-1/stage/0", "wrong"),
    ) as unknown;
    expect(() => parseRooms([bad])).toThrow("protocol");
    const references = JSON.parse(
      JSON.stringify(room).replaceAll('"S1"]', '"missing"]'),
    ) as unknown;
    expect(() => parseRooms([references])).toThrow("protocol");
    expect(newerRooms([room], [{ ...room, runs: [{ ...fixture(), sequence: 1 }] }])).toEqual([
      room,
    ]);
  });
});
