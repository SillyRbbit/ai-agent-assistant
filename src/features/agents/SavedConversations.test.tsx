import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SavedConversations, SavedTranscript } from "./SavedConversations";
import {
  DEFAULT_BOT_IDENTITY,
  type AgentChatClient,
  type AgentProfile,
  type AgentChatSnapshot,
} from "../../infrastructure/tauri/agent-chat-client";
import type { SavedConversation } from "../../infrastructure/tauri/conversation-history-client";
const profile: AgentProfile = {
  agentId: "research",
  displayName: "Research",
  identity: DEFAULT_BOT_IDENTITY,
  connection: "simulation",
  model: "simulation",
  effort: "default",
  endpoint: "",
  localAuth: false,
  allowUnknownLocalityNotes: false,
  ownerInstructions: "",
  memoryMode: "off",
  note: "",
  revision: 0,
};
const idle: AgentChatSnapshot = {
  version: 1,
  conversationId: "saved-chat-1",
  agentId: "research",
  connection: "simulation",
  model: "simulation",
  endpoint: "",
  effort: "default",
  memoryMode: "off",
  settingsRevision: 0,
  status: "idle",
  text: "",
  sequence: 0,
  busy: false,
  error: null,
};
const history: SavedConversation = {
  id: "saved-chat-1",
  agentId: "research",
  title: "Synthetic",
  createdAtMs: 1,
  updatedAtMs: 2,
  connection: "simulation",
  model: "simulation",
  endpoint: "",
  effort: "default",
  contextTurnIds: [],
  turns: [
    {
      id: 1,
      message: "SYNTHETIC-TEAL",
      answer: "partial answer",
      status: "cancelled",
      failure: null,
      createdAtMs: 1,
      updatedAtMs: 2,
    },
  ],
};
function fixture() {
  const persistence = {
    list: vi
      .fn()
      .mockResolvedValue([
        { id: history.id, agentId: "research", title: "Synthetic", updatedAtMs: 2, turns: 1 },
      ]),
    read: vi.fn().mockResolvedValue(history),
    open: vi.fn().mockResolvedValue(idle),
    rename: vi.fn().mockResolvedValue(undefined),
    delete: vi.fn().mockResolvedValue(undefined),
    fact: vi.fn().mockResolvedValue({ ...profile, note: "SYNTHETIC-TEAL", revision: 1 }),
    provenance: vi.fn().mockResolvedValue(null),
  };
  const client: AgentChatClient = {
    persistence,
    available: () => true,
    list: vi.fn().mockResolvedValue([profile]),
    connections: vi.fn().mockResolvedValue([]),
    discover: vi.fn(),
    save: vi.fn(),
    clearNote: vi.fn(),
    defaults: vi.fn(),
    start: vi.fn(),
    send: vi.fn(),
    poll: vi.fn(),
    cancel: vi.fn(),
  };
  return { client, persistence };
}
describe("saved native conversations", () => {
  it("opens a conversation without sending and requires explicit deletion confirmation", async () => {
    const { client, persistence } = fixture();
    const onOpen = vi.fn();
    render(<SavedConversations client={client} profile={profile} busy={false} onOpen={onOpen} />);
    fireEvent.click(await screen.findByRole("button", { name: "Synthetic · 1 turns" }));
    await waitFor(() => {
      expect(onOpen).toHaveBeenCalledWith(idle);
    });
    expect(client.send).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Manage Synthetic" }));
    expect(screen.getByRole("button", { name: "Delete conversation" })).toBeDisabled();
    fireEvent.click(screen.getByRole("checkbox"));
    fireEvent.click(screen.getByRole("button", { name: "Delete conversation" }));
    await waitFor(() => {
      expect(persistence.delete).toHaveBeenCalledWith("saved-chat-1");
    });
  });
  it("preserves partial output and saves no memory until preview confirmation", async () => {
    const { client, persistence } = fixture();
    render(<SavedTranscript client={client} initial={idle} busy={false} refresh={0} />);
    expect(await screen.findByText("partial answer")).toBeVisible();
    expect(screen.getByText("Turn 1 · cancelled")).toBeVisible();
    fireEvent.click(screen.getByRole("button", { name: "Save a selected fact from turn 1" }));
    fireEvent.change(screen.getByRole("textbox"), { target: { value: "SYNTHETIC-TEAL" } });
    expect(persistence.fact).not.toHaveBeenCalled();
    expect(screen.getByRole("button", { name: "Confirm save fact" })).toBeDisabled();
    fireEvent.click(screen.getByRole("checkbox"));
    fireEvent.click(screen.getByRole("button", { name: "Confirm save fact" }));
    await waitFor(() => {
      expect(persistence.fact).toHaveBeenCalledWith("saved-chat-1", 1, "SYNTHETIC-TEAL", 0);
    });
    expect(client.send).not.toHaveBeenCalled();
  });
  it("rejects another bot transcript rather than rendering private text", async () => {
    const { client, persistence } = fixture();
    persistence.read.mockResolvedValue({ ...history, agentId: "coding" });
    render(<SavedTranscript client={client} initial={idle} busy={false} refresh={0} />);
    expect(await screen.findByRole("alert")).toHaveTextContent("Saved transcript unavailable");
    expect(screen.queryByText("partial answer")).not.toBeInTheDocument();
  });
});
