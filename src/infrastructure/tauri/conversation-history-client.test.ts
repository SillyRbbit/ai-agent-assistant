import { describe, expect, it, vi } from "vitest";
import { conversationHistoryClient, parseSavedConversation } from "./conversation-history-client";
const invoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke, isTauri: () => true }));
const saved = {
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
      message: "synthetic",
      answer: "part",
      status: "cancelled",
      failure: null,
      createdAtMs: 1,
      updatedAtMs: 2,
    },
  ],
};
describe("strict saved history boundary", () => {
  it("accepts preserved cancelled text without treating it as completed", () => {
    expect(parseSavedConversation(saved).turns[0]?.status).toBe("cancelled");
  });
  it("rejects extra authority, malformed identifiers, corrupt states and oversized content", () => {
    const values = [
      { ...saved, tool: "execute" },
      { ...saved, id: "../database" },
      { ...saved, agentId: "other" },
      { ...saved, turns: [{ ...saved.turns[0], status: "resubmit" }] },
      { ...saved, turns: [{ ...saved.turns[0], id: 2 }] },
      { ...saved, turns: [{ ...saved.turns[0], answer: "x".repeat(8193) }] },
      { ...saved, turns: [{ ...saved.turns[0], failure: "raw-private-error" }] },
    ];
    for (const value of values) expect(() => parseSavedConversation(value)).toThrow("protocol");
  });
  it("rejects a mismatched conversation response", async () => {
    invoke.mockResolvedValue({ ...saved, id: "saved-chat-2" });
    await expect(conversationHistoryClient.read("saved-chat-1")).rejects.toThrow("protocol");
  });
  it("does not send an arbitrary path or authority with a fact", async () => {
    invoke.mockResolvedValue(null);
    await expect(conversationHistoryClient.fact("saved-chat-1", 1, "synthetic", 4)).rejects.toThrow(
      "protocol",
    );
    expect(invoke).toHaveBeenLastCalledWith("save_conversation_fact", {
      request: { conversationId: "saved-chat-1", turnId: 1, text: "synthetic", revision: 4 },
    });
  });
  it("rejects other bot summaries and duplicate identities", async () => {
    invoke.mockResolvedValue([
      { id: "saved-chat-1", agentId: "coding", title: "s", updatedAtMs: 2, turns: 1 },
    ]);
    await expect(conversationHistoryClient.list("research")).rejects.toThrow("protocol");
  });
});
