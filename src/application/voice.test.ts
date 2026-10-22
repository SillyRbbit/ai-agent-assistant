import { describe, expect, it } from "vitest";
import {
  appendTranscript,
  parseVoiceSnapshot,
  voiceErrorMessage,
  type VoiceSnapshot,
} from "./voice";
export const voiceFixture: VoiceSnapshot = {
  version: 1,
  conversationId: "conversation-1",
  sessionId: "abc-123",
  sequence: 0,
  status: "starting",
  busy: true,
  text: null,
  error: null,
};
describe("voice draft and strict snapshots", () => {
  it("appends without replacing typed text and never silently truncates UTF16", () => {
    expect(appendTranscript("Typed draft.", "Spoken text.")).toBe("Typed draft. Spoken text.");
    expect(appendTranscript("Typed\n", "Spoken")).toBe("Typed\nSpoken");
    expect(appendTranscript("", "hello")).toBe("hello");
    expect(appendTranscript("", " ")).toBeNull();
    expect(appendTranscript("x".repeat(4095), "a")).toBeNull();
    expect(appendTranscript("", "😀".repeat(2048))).toHaveLength(4096);
    expect(appendTranscript("", "😀".repeat(2049))).toBeNull();
  });
  it("rejects unknown fields, status, error, identity and unbounded fields", () => {
    for (const patch of [
      { extra: "private" },
      { version: 2 },
      { conversationId: "other" },
      { sessionId: "../private" },
      { sequence: -1 },
      { sequence: Infinity },
      { status: "unknown" },
      { error: "raw_error" },
      { text: "private audio" },
      { busy: "true" },
    ]) {
      expect(() => parseVoiceSnapshot({ ...voiceFixture, ...patch }, "conversation-1")).toThrow(
        "protocol",
      );
    }
  });
  it("requires complete native cleanup before exposing review text", () => {
    const review = {
      ...voiceFixture,
      status: "review",
      busy: false,
      sequence: 2,
      text: "Synthetic phrase",
    };
    expect(parseVoiceSnapshot(review, "conversation-1").text).toBe("Synthetic phrase");
    expect(() => parseVoiceSnapshot({ ...review, busy: true }, "conversation-1")).toThrow();
    expect(() => parseVoiceSnapshot({ ...review, text: null }, "conversation-1")).toThrow();
    expect(() => parseVoiceSnapshot({ ...review, text: "\u0000" }, "conversation-1")).toThrow();
  });
  it("rejects sequence rollback, cross-request replies and terminal mutation", () => {
    const next = parseVoiceSnapshot({ ...voiceFixture, sequence: 2 }, "conversation-1");
    expect(() =>
      parseVoiceSnapshot({ ...voiceFixture, sequence: 1 }, "conversation-1", next),
    ).toThrow();
    expect(() => parseVoiceSnapshot({ ...next, sessionId: "b" }, "conversation-1", next)).toThrow();
    const done = parseVoiceSnapshot(
      { ...next, status: "cancelled", busy: false },
      "conversation-1",
    );
    expect(() =>
      parseVoiceSnapshot({ ...done, status: "completed" }, "conversation-1", done),
    ).toThrow();
    expect(parseVoiceSnapshot(done, "conversation-1", done)).toEqual(done);
  });
  it("accepts only closed recognition failure labels and retains terminal rejection", () => {
    for (const error of [
      "recognition_no_speech",
      "recognition_assets_missing",
      "recognition_disabled",
      "recognition_service",
      "recognition_busy",
    ]) {
      const failed = parseVoiceSnapshot(
        { ...voiceFixture, status: "error", error, busy: false, sequence: 1 },
        "conversation-1",
      );
      expect(failed.text).toBeNull();
      expect(voiceErrorMessage(error)).not.toContain(error);
      expect(() =>
        parseVoiceSnapshot({ ...failed, error: "arbitrary-native-detail" }, "conversation-1"),
      ).toThrow("protocol");
      expect(() =>
        parseVoiceSnapshot(
          { ...failed, status: "review", error: null, text: "late" },
          "conversation-1",
          failed,
        ),
      ).toThrow("protocol");
    }
  });
  it("maps closed errors without exposing supplied arbitrary details", () => {
    expect(voiceErrorMessage(new Error("private-path-and-token"))).not.toContain(
      "private-path-and-token",
    );
    expect(voiceErrorMessage("permission_denied")).toContain("permission");
    expect(voiceErrorMessage("cleanup_failed")).toContain("blocked");
    expect(voiceErrorMessage("helper_diagnostics")).toContain("diagnostic output");
    expect(voiceErrorMessage("protocol")).toContain("safety validation");
    expect(voiceErrorMessage("capture_failed")).toContain("microphone");
    expect(voiceErrorMessage("recognition_failed")).toContain("On-device");
  });
});
