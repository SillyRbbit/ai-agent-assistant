import { describe, expect, it } from "vitest";
import { parseDiagnostics } from "../../infrastructure/tauri/diagnostics-client";
const record = {
  utcMs: 1790962000000,
  severity: "error",
  appVersion: "0.1.0",
  event: "request_finished",
  outcome: "failed",
  error: "network",
  session: `sha256:${"a".repeat(64)}`,
  conversation: null,
  attempt: `sha256:${"b".repeat(64)}`,
  workflow: null,
  stage: null,
  provider: "openai_api",
  model: "gpt-5.6-luna",
  durationMs: 20,
};
import { diagnosticAdvice, troubleshootingSummary } from "./diagnosticSummary";
describe("bounded troubleshooting summary", () => {
  it("preserves uncertain cause, missing dispatch and logical correlation", () => {
    const request = `sha256:${"e".repeat(64)}`;
    const events = parseDiagnostics({
      available: true,
      events: [
        {
          ...record,
          request,
          attempt: null,
          event: "request_started",
          outcome: "observed",
          error: null,
        },
        { ...record, request, attempt: null, error: "missing_credentials" },
      ],
    }).events;
    const summary = troubleshootingSummary(events);
    expect(summary).toContain(request);
    expect(summary).toContain("no dispatch observed");
    expect(summary).toContain("Terminal outcome: failed");
    expect(summary).toContain("owner-only private launch");
    expect(diagnosticAdvice("network")).toContain("Authentication and billing are not established");
    expect(diagnosticAdvice("openai_top_level_absent_code")).toContain("does not establish");
  });
  it("keeps runtime cleanup failures visible after a successful provider outcome", () => {
    const events = parseDiagnostics({
      available: true,
      events: [
        { ...record, event: "runtime_cleanup", error: "cleanup" },
        { ...record, event: "request_finished", outcome: "completed", error: null },
      ],
    }).events;
    const summary = troubleshootingSummary(events);
    expect(summary).toContain("cleanup did not confirm success");
    expect(summary).toContain("Terminal outcome: completed");
  });
});

it("keeps nested error guidance specific without claiming a successful retry", () => {
  expect(diagnosticAdvice("openai_insufficient_quota")).toContain("Do not retry unchanged");
  expect(diagnosticAdvice("openai_invalid_api_key")).toContain("Authentication was rejected");
  expect(diagnosticAdvice("openai_unsupported_value")).toContain("do not substitute a model");
  expect(diagnosticAdvice("openai_envelope_object")).toContain("separate observations");
});

it("distinguishes prepaid API credit exhaustion from transient limits and subscription credits", () => {
  const advice = diagnosticAdvice("openai_credit_balance_exhausted");
  expect(advice).toContain("no prepaid API credits remaining");
  expect(advice).toContain("Work/Codex credits are separate");
  expect(advice).toContain("Do not retry unchanged or change billing automatically");
  expect(diagnosticAdvice("rate_limit")).not.toBe(advice);
  expect(diagnosticAdvice("openai_insufficient_quota")).not.toBe(advice);
  expect(diagnosticAdvice("openai_top_level_unknown_code")).toContain("does not establish");
});

it("distinguishes local bounds from malformed or incomplete provider data", () => {
  const advice = diagnosticAdvice("resource_limit");
  expect(advice).toContain("bounded local response or event limit");
  expect(advice).toContain("shorter response");
  expect(advice).not.toBe(diagnosticAdvice("malformed_stream"));
  expect(advice).not.toBe(diagnosticAdvice("incomplete"));
});
