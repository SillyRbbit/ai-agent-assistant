import { describe, expect, it, vi } from "vitest";
import { diagnosticsClient, parseDiagnostics } from "./diagnostics-client";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(), isTauri: vi.fn(() => true) }));
import { invoke } from "@tauri-apps/api/core";
export const record = {
  utcMs: 1790962000000,
  severity: "error",
  appVersion: "0.1.0",
  event: "request_finished",
  outcome: "failed",
  error: "denied_access",
  session: `sha256:${"a".repeat(64)}`,
  conversation: null,
  attempt: `sha256:${"b".repeat(64)}`,
  workflow: null,
  stage: null,
  provider: "openai_api",
  model: "gpt-5.6-luna",
  durationMs: 25,
};
describe("closed diagnostics", () => {
  it("accepts closed events and rejects content channels, malformed fields and excess records", () => {
    const snapshot = { available: true, events: [record] };
    expect(parseDiagnostics(snapshot)).toEqual(snapshot);
    for (const patch of [
      { message: "SECRET_CANARY" },
      { error: "RAW_ERROR_CANARY" },
      { model: "/private/sensitive-model" },
      { conversation: "PROMPT_CANARY" },
      { utcMs: Infinity },
      { durationMs: -1 },
      { provider: "arbitrary" },
    ]) {
      expect(() =>
        parseDiagnostics({ available: true, events: [{ ...record, ...patch }] }),
      ).toThrow("Invalid diagnostic data.");
    }
    expect(() => parseDiagnostics({ available: true, events: Array(257).fill(record) })).toThrow();
    expect(() => parseDiagnostics({ ...snapshot, secret: "CANARY" })).toThrow();
  });
  it("accepts closed code/parameter classifications and preserves legacy stream errors", () => {
    const errors = [
      "openai_top_level_server_error",
      "openai_top_level_rate_limit_exceeded",
      "openai_top_level_invalid_prompt",
      "openai_top_level_unknown_code",
      "openai_top_level_missing_code",
      "openai_top_level_absent_code",
      "openai_top_level_null_code",
      "openai_top_level_param_model",
      "openai_top_level_param_reasoning",
      "openai_top_level_param_reasoning_effort",
      "openai_top_level_param_max_output_tokens",
      "openai_top_level_param_service_tier",
      "openai_top_level_param_absent",
      "openai_top_level_param_null",
      "openai_top_level_param_invalid",
      "openai_top_level_param_unknown",
      "openai_top_level_invalid_code",
    ];
    const events = errors.map((error) => ({
      ...record,
      event: error.startsWith("openai_top_level_param_")
        ? "openai_top_level_error_param"
        : "openai_top_level_error",
      outcome: "observed",
      error,
    }));
    events.push({ ...record, error: "stream_error" });
    const snapshot = { available: true, events };
    expect(parseDiagnostics(snapshot)).toEqual(snapshot);
    for (const error of [
      "server_error",
      "rate_limit_exceeded",
      "invalid_prompt",
      "OPENAI_TOP_LEVEL_SERVER_ERROR",
      "openai_top_level_RAW_CANARY",
      "RAW_ERROR_CANARY",
      { code: "server_error", message: "PRIVATE_MESSAGE_CANARY" },
      ["openai_top_level_server_error"],
    ]) {
      expect(() =>
        parseDiagnostics({
          available: true,
          events: [{ ...record, event: "openai_top_level_error", error }],
        }),
      ).toThrow("Invalid diagnostic data.");
    }
    for (const patch of [
      { event: "error" },
      { event: "response.failed" },
      { code: "server_error" },
      { message: "PRIVATE_MESSAGE_CANARY" },
      { param: "PRIVATE_PARAMETER_CANARY" },
      { frame: "RAW_FRAME_CANARY" },
    ]) {
      expect(() =>
        parseDiagnostics({
          available: true,
          events: [
            {
              ...record,
              event: "openai_top_level_error",
              error: "openai_top_level_unknown_code",
              ...patch,
            },
          ],
        }),
      ).toThrow("Invalid diagnostic data.");
    }
  });
  it("invokes only argument-free read and export", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce({ available: true, events: [] })
      .mockResolvedValueOnce(false);
    expect(await diagnosticsClient.read()).toEqual({ available: true, events: [] });
    expect(await diagnosticsClient.export()).toBe(false);
    expect(invoke).toHaveBeenNthCalledWith(1, "read_diagnostics");
    expect(invoke).toHaveBeenNthCalledWith(2, "export_diagnostics");
  });
});

it("accepts generated request identity and explicit interruption but rejects arbitrary correlation", () => {
  const request = `sha256:${"d".repeat(64)}`;
  const item = {
    ...record,
    request,
    event: "request_interrupted",
    outcome: "interrupted",
    error: "interrupted",
    durationMs: null,
  };
  expect(parseDiagnostics({ available: true, events: [item] }).events[0]?.request).toBe(request);
  for (const request of ["PRIVATE-REQUEST-CANARY", null, 42, { secret: "PRIVATE-CANARY" }]) {
    expect(() => parseDiagnostics({ available: true, events: [{ ...item, request }] })).toThrow();
  }
});

it("accepts bounded envelope observations but refuses raw nested content", () => {
  for (const error of [
    "openai_envelope_absent",
    "openai_envelope_null",
    "openai_envelope_invalid",
    "openai_envelope_object",
    "openai_invalid_api_key",
    "openai_insufficient_quota",
    "openai_credit_balance_exhausted",
    "openai_model_not_found",
    "openai_unsupported_value",
    "openai_invalid_value",
  ]) {
    const row = { ...record, event: "openai_error_envelope", error };
    expect(parseDiagnostics({ available: true, events: [row] }).events).toEqual([row]);
    expect(() =>
      parseDiagnostics({ available: true, events: [{ ...row, message: "PRIVATE_ERROR_CANARY" }] }),
    ).toThrow();
  }
  for (const event of ["openai_nested_error_code", "openai_nested_error_param"]) {
    expect(
      parseDiagnostics({ available: true, events: [{ ...record, event }] }).events,
    ).toHaveLength(1);
  }
});

it("rejects nonliteral billing categories and private fields", () => {
  for (const error of [
    "credit_balance_exhausted",
    "openai_credit_balance_exhausted PRIVATE_ERROR_CANARY",
    { code: "openai_credit_balance_exhausted" },
  ]) {
    expect(() => parseDiagnostics({ available: true, events: [{ ...record, error }] })).toThrow();
  }
});

it("accepts only the closed resource-limit category", () => {
  expect(
    parseDiagnostics({ available: true, events: [{ ...record, error: "resource_limit" }] }).events,
  ).toEqual([{ ...record, error: "resource_limit" }]);
  expect(() =>
    parseDiagnostics({
      available: true,
      events: [{ ...record, error: "resource_limit: PRIVATE_CANARY" }],
    }),
  ).toThrow();
});
