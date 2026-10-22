import { beforeEach, expect, it, vi } from "vitest";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { voiceClient } from "./voice-client";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(), isTauri: vi.fn() }));
const snapshot = {
  version: 1,
  conversationId: "conversation-1",
  sessionId: "abc-123",
  sequence: 0,
  status: "starting",
  busy: true,
  text: null,
  error: null,
} as const;
beforeEach(() => vi.resetAllMocks());
it("uses only fixed commands and IDs, never audio, caller text, locale, context or executable paths", async () => {
  vi.mocked(isTauri).mockReturnValue(true);
  vi.mocked(invoke).mockResolvedValue(snapshot);
  expect(voiceClient.available()).toBe(true);
  const initial = await voiceClient.start("conversation-1", "record");
  await voiceClient.poll(initial);
  await voiceClient.control(initial, "cancel");
  expect(vi.mocked(invoke).mock.calls).toEqual([
    ["start_voice", { request: { conversationId: "conversation-1", operation: "record" } }],
    ["poll_voice", { request: { conversationId: "conversation-1", sessionId: "abc-123" } }],
    [
      "control_voice",
      { request: { conversationId: "conversation-1", sessionId: "abc-123", action: "cancel" } },
    ],
  ]);
});
it("rejects a mismatched native reply and never sends a provider request", async () => {
  vi.mocked(invoke).mockResolvedValue({ ...snapshot, conversationId: "other" });
  await expect(voiceClient.start("conversation-1", "record")).rejects.toThrow("protocol");
  expect(invoke).toHaveBeenCalledTimes(1);
});

it("permits only an explicit matching review discard transition", async () => {
  const previous = {
    ...snapshot,
    sequence: 2,
    status: "review",
    busy: false,
    text: "Synthetic",
  } as const;
  vi.mocked(invoke).mockResolvedValue({
    ...previous,
    sequence: 3,
    status: "cancelled",
    text: null,
  });
  expect((await voiceClient.control(previous, "cancel")).text).toBeNull();
  vi.mocked(invoke).mockResolvedValue({
    ...previous,
    sequence: 3,
    status: "cancelled",
    text: null,
    sessionId: "other",
  });
  await expect(voiceClient.control(previous, "cancel")).rejects.toThrow("protocol");
});
