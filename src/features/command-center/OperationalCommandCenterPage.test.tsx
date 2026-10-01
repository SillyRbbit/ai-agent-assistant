import { render, screen, fireEvent, waitFor, cleanup } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import Page from "./OperationalCommandCenterPage";
import {
  WORKFLOWS,
  type CollaborationClient,
  type Room,
} from "../../infrastructure/tauri/collaboration-client";
import { DEFAULT_BOT_IDENTITY } from "../../infrastructure/tauri/agent-chat-client";
vi.mock("./CollaborationTopology", () => ({
  CollaborationTopology: () => <div>Operational canvas</div>,
}));
afterEach(cleanup);
function makeClient() {
  const room: Room = {
    id: 1,
    title: "Saved room",
    runs: [
      {
        id: "room-1/run-1",
        sequence: 1,
        status: "completed",
        error: null,
        input: { workflow: "research", objective: "synthetic", sources: [] },
        stages: WORKFLOWS.research.map((agentId, i) => ({
          id: `room-1/run-1/stage/${String(i)}`,
          participant: {
            agentId,
            role: agentId,
            identity: { ...DEFAULT_BOT_IDENTITY, nickname: "Saved name" },
            connection: "simulation",
            model: "simulation",
            effort: "default",
            endpoint: "",
            localAuth: false,
            ownerInstructions: "DO NOT PROJECT",
            revision: 1,
          },
          status: "completed",
          timestamp: 1,
          provisional: "",
          input: "DO NOT PROJECT",
          handoff: null,
        })),
      },
    ],
  };
  const client: CollaborationClient = {
    list: vi.fn().mockResolvedValue([room]),
    create: vi.fn(),
    prepare: vi.fn(),
    start: vi.fn(),
    cancel: vi.fn(),
    delete: vi.fn(),
  };
  return { client, room };
}
it("inspects a saved deep link without generation and explicitly handles deletion", async () => {
  const { client } = makeClient();
  render(
    <Page
      client={client}
      loadProfiles={() => Promise.resolve([])}
      location={{ roomId: 1, runId: "room-1/run-1", stageId: "room-1/run-1/stage/0" }}
    />,
  );
  await screen.findByRole("heading", { name: "Stage 1 · Saved name" });
  expect(screen.queryByText("DO NOT PROJECT")).not.toBeInTheDocument();
  expect(client.start).not.toHaveBeenCalled();
  expect(client.prepare).not.toHaveBeenCalled();
  expect(client.cancel).not.toHaveBeenCalled();
  vi.mocked(client.list).mockResolvedValue([]);
  fireEvent.click(screen.getByRole("button", { name: "Refresh snapshots" }));
  await screen.findByText(/A deleted room or missing run is never replaced/);
  expect(screen.queryByRole("heading", { name: "Stage 1 · Saved name" })).not.toBeInTheDocument();
  expect(screen.getByText(/No selected run data/)).toBeInTheDocument();
});
it("keeps a chosen run on route reentry without replaying a consumed deep link", async () => {
  const { client, room } = makeClient();
  const first = room.runs[0];
  if (!first) throw Error("fixture");
  vi.mocked(client.list).mockResolvedValue([
    { ...room, runs: [first, { ...first, id: "room-1/run-2" }] },
  ]);
  const location = { roomId: 1, runId: first.id, stageId: null };
  const loadProfiles = () => Promise.resolve([]);
  const view = render(<Page client={client} loadProfiles={loadProfiles} location={location} />);
  await screen.findByText(/sequence 1/);
  fireEvent.change(screen.getByRole("combobox", { name: "Run" }), {
    target: { value: "room-1/run-2" },
  });
  await waitFor(() => {
    expect(screen.getByRole("combobox", { name: "Run" })).toHaveValue("room-1/run-2");
  });
  view.unmount();
  render(<Page client={client} loadProfiles={loadProfiles} location={location} />);
  expect(screen.getByRole("combobox", { name: "Run" })).toHaveValue("room-1/run-2");
});
