import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  parseVoiceSnapshot,
  type VoiceOperation,
  type VoiceSnapshot,
} from "../../application/voice";
export interface VoiceClient {
  available(): boolean;
  start(conversationId: string, operation: VoiceOperation): Promise<VoiceSnapshot>;
  poll(previous: VoiceSnapshot): Promise<VoiceSnapshot>;
  control(previous: VoiceSnapshot, action: "finish" | "cancel"): Promise<VoiceSnapshot>;
}
export const voiceClient: VoiceClient = {
  available: () => isTauri(),
  start: async (conversationId, operation) =>
    parseVoiceSnapshot(
      await invoke<unknown>("start_voice", { request: { conversationId, operation } }),
      conversationId,
    ),
  poll: async (previous) =>
    parseVoiceSnapshot(
      await invoke<unknown>("poll_voice", {
        request: { conversationId: previous.conversationId, sessionId: previous.sessionId },
      }),
      previous.conversationId,
      previous,
    ),
  control: async (previous, action) => {
    const value = await invoke<unknown>("control_voice", {
      request: { conversationId: previous.conversationId, sessionId: previous.sessionId, action },
    });
    if (!previous.busy && previous.status === "review" && action === "cancel") {
      const next = parseVoiceSnapshot(value, previous.conversationId);
      if (
        next.sessionId !== previous.sessionId ||
        next.status !== "cancelled" ||
        next.busy ||
        next.sequence !== previous.sequence + 1
      )
        throw new Error("protocol");
      return next;
    }
    return parseVoiceSnapshot(value, previous.conversationId, previous);
  },
};
