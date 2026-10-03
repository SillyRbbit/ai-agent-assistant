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
