import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  directClient,
  directErrorMessage,
  IDLE_DIRECT,
  parseDirectSnapshot,
} from "./personal-assistant-direct-client";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(), isTauri: vi.fn() }));
describe("direct native boundary", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });
  it("uses only the three fixed control payloads without prompt or credentials", async () => {
    vi.mocked(invoke).mockResolvedValue(IDLE_DIRECT);
    vi.mocked(isTauri).mockReturnValue(false);
    expect(directClient.available()).toBe(false);
    await directClient.start();
    await directClient.poll(null, null);
    await directClient.cancel("pa-v0-present-0000000000000001");
    expect(vi.mocked(invoke).mock.calls).toEqual([
      [
        "start_personal_assistant_direct",
        { request: { version: 1, acknowledgment: "openai-synthetic-direct-v1" } },
      ],
      ["poll_personal_assistant_direct", { request: { handle: null, cursor: null } }],
      [
        "cancel_personal_assistant_direct",
        { request: { handle: "pa-v0-present-0000000000000001" } },
      ],
    ]);
  });
  it("rejects malformed, oversized and untrusted snapshots without echoing values", () => {
    for (const value of [
      null,
      {},
      { ...IDLE_DIRECT, secret: "private" },
      { ...IDLE_DIRECT, sequence: -1 },
      { ...IDLE_DIRECT, text: "x".repeat(32769) },
      { ...IDLE_DIRECT, error: "private provider body" },
      { ...IDLE_DIRECT, status: "completed" },
    ]) {
      expect(() => parseDirectSnapshot(value)).toThrow("protocol");
    }
    expect(parseDirectSnapshot(IDLE_DIRECT)).toEqual(IDLE_DIRECT);
    for (const value of [
      new Error("DUMMY-PRIVATE-CONTENT"),
      "DUMMY-PRIVATE-CONTENT",
      { key: "DUMMY" },
    ]) {
      expect(directErrorMessage(value)).toBe("The native session is unavailable.");
    }
    expect(directErrorMessage("missing_key")).toContain("No valid native session");
    expect(directErrorMessage("authentication")).toContain("rejected authentication");
    expect(directErrorMessage("rate_limited")).toContain("No automatic retry");
    expect(directErrorMessage("timeout")).toContain("timed out");
  });
  it.each([
    ["network", "connection failed while sending or receiving data"],
    ["http_status", "returned an unsuccessful HTTP response"],
    ["provider_stream", "reported a failure in the response stream"],
  ])("accepts only the closed %s code and displays its static message", (code, message) => {
    const snapshot = {
      ...IDLE_DIRECT,
      handle: "pa-v0-present-0000000000000001",
      status: "error",
      sequence: 1,
      error: code,
    };
    expect(parseDirectSnapshot(snapshot).error).toBe(code);
    expect(directErrorMessage(code)).toContain(message);
    expect(directErrorMessage(code)).toContain("No mock response was substituted.");
    for (const error of [`${code}: DUMMY-PRIVATE-CONTENT`, { code, message: "DUMMY" }]) {
      expect(() => parseDirectSnapshot({ ...snapshot, error })).toThrow("protocol");
      expect(directErrorMessage(error)).toBe("The native session is unavailable.");
    }
    expect(() => parseDirectSnapshot({ ...snapshot, diagnostic: "DUMMY-PRIVATE-CONTENT" })).toThrow(
      "protocol",
    );
  });
  it.each([
    ["http_bad_request", "OpenAI rejected the request (HTTP 400). No automatic retry was made."],
    ["http_forbidden", "OpenAI denied access (HTTP 403). No automatic retry was made."],
    [
      "http_not_found",
      "OpenAI could not find the requested resource (HTTP 404). No automatic retry was made.",
    ],
    [
      "provider_stream_server_error",
      "The provider reported a server error in the response stream. No automatic retry was made.",
    ],
    [
      "provider_stream_rate_limit",
      "The provider reported a rate limit in the response stream. No automatic retry was made.",
    ],
    [
      "provider_stream_invalid_prompt",
      "The provider reported an invalid prompt in the response stream. No automatic retry was made.",
    ],
    [
      "provider_stream_error_event",
      "The provider emitted a top-level error event in the response stream. No automatic retry was made.",
    ],
    [
      "provider_stream_failed_unknown_code",
      "The provider reported response.failed with an unrecognized error code. No automatic retry was made.",
    ],
    [
      "provider_stream_failed_invalid_code",
      "The provider reported response.failed without a usable error code. No automatic retry was made.",
    ],
  ])("narrows %s to its exact static message", (code, message) => {
    const snapshot = {
      ...IDLE_DIRECT,
      handle: "pa-v0-present-0000000000000001",
      status: "error",
      sequence: 1,
      error: code,
    };
    expect(parseDirectSnapshot(snapshot).error).toBe(code);
    expect(directErrorMessage(code)).toBe(message);
    for (const error of [
      `${code}: DUMMY-PRIVATE-CONTENT`,
      code.toUpperCase(),
      { code, message: "DUMMY-PRIVATE-CONTENT" },
    ]) {
      expect(() => parseDirectSnapshot({ ...snapshot, error })).toThrow("protocol");
      expect(directErrorMessage(error)).toBe("The native session is unavailable.");
      expect(directErrorMessage(error)).not.toContain("DUMMY-PRIVATE-CONTENT");
    }
    expect(() => parseDirectSnapshot({ ...snapshot, diagnostic: "DUMMY-PRIVATE-CONTENT" })).toThrow(
      "protocol",
    );
  });
});
