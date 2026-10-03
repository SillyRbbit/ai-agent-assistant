import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import {
  parseDiagnostics,
  type DiagnosticsSnapshot,
} from "../../infrastructure/tauri/diagnostics-client";
const snapshot: DiagnosticsSnapshot = {
  available: true,
  events: [
    {
      utcMs: 1790962000000,
      severity: "info",
      appVersion: "0.1.0",
      event: "application_started",
      outcome: "observed",
      error: null,
      session: `sha256:${"a".repeat(64)}`,
      conversation: null,
      attempt: null,
      workflow: null,
      stage: null,
      provider: null,
      model: null,
      durationMs: null,
    },
  ],
};
describe("Diagnostics UI", () => {
  it("shows bounded events, filters severity and copies only parsed diagnostics", async () => {
    const copy = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: copy },
    });
    const client = {
      read: vi.fn().mockResolvedValue(snapshot),
      export: vi.fn().mockResolvedValue(false),
    };
    render(<DiagnosticsPanel client={client} />);
    await screen.findByText("Logging available");
    expect(screen.getByText("application_started · observed")).toBeVisible();
    fireEvent.change(screen.getByLabelText("Severity"), { target: { value: "error" } });
    expect(screen.getByText("No matching diagnostic events.")).toBeVisible();
    fireEvent.click(screen.getByRole("button", { name: "Copy sanitized summary" }));
    await screen.findByText("Sanitized diagnostic summary copied.");
    expect(JSON.parse(copy.mock.calls[0]?.[0] as string)).toEqual(snapshot);
    fireEvent.click(screen.getByRole("button", { name: "Export diagnostics" }));
    await screen.findByText("Export cancelled; no file written.");
    expect(client.export).toHaveBeenCalledTimes(1);
    expect(client.read).toHaveBeenCalledTimes(1);
  });
  it("presents closed top-level classifications and copies only their sanitized correlation data", async () => {
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
    const attempt = `sha256:${"b".repeat(64)}`;
    const conversation = `sha256:${"c".repeat(64)}`;
    const closedSnapshot = parseDiagnostics({
      available: true,
      events: [
        ...errors.map((error, index) => ({
          ...snapshot.events[0],
          utcMs: 1790962000000 + index,
          severity: "error",
          event: error.startsWith("openai_top_level_param_")
            ? "openai_top_level_error_param"
            : "openai_top_level_error",
          error,
          provider: "openai_api",
          model: "gpt-5.6-luna",
          attempt,
          conversation,
          durationMs: 25,
        })),
        {
          ...snapshot.events[0],
          severity: "error",
          event: "request_finished",
          outcome: "failed",
          error: "stream_error",
          attempt,
          conversation,
        },
      ],
    });
    const copy = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: copy },
    });
    render(
      <DiagnosticsPanel
        client={{ read: vi.fn().mockResolvedValue(closedSnapshot), export: vi.fn() }}
      />,
    );
    await screen.findByText("Logging available");
    expect(screen.getAllByText("openai_top_level_error · observed")).toHaveLength(8);
    expect(screen.getAllByText("openai_top_level_error_param · observed")).toHaveLength(9);
    for (const error of errors) {
      expect(screen.getByText(`· ${error}`)).toBeVisible();
    }
    expect(screen.getByText("request_finished · failed")).toBeVisible();
    expect(screen.getByText("· stream_error")).toBeVisible();
    for (const summary of screen.getAllByText("Correlation")) fireEvent.click(summary);
    for (const correlation of screen.getAllByText(attempt)) expect(correlation).toBeVisible();
    for (const correlation of screen.getAllByText(`Conversation: ${conversation}`))
      expect(correlation).toBeVisible();
    const summary = screen.getByText("Selectable sanitized summary");
    fireEvent.click(summary);
    const summaryContent = summary.closest("details")?.querySelector("pre");
    expect(summaryContent).toBeVisible();
    expect(summaryContent?.textContent).toBe(JSON.stringify(closedSnapshot, null, 2));
    fireEvent.click(screen.getByRole("button", { name: "Copy sanitized summary" }));
    await screen.findByText("Sanitized diagnostic summary copied.");
    expect(JSON.parse(copy.mock.calls[0]?.[0] as string)).toEqual(closedSnapshot);
    expect(copy.mock.calls[0]?.[0]).not.toMatch(/"(?:code|message|param|frame)"/);
  });
  it("discloses unavailable storage and refuses export overwrite without raw errors", async () => {
    const client = {
      read: vi.fn().mockResolvedValue({ ...snapshot, available: false }),
      export: vi.fn().mockRejectedValue("destination_exists"),
    };
    render(<DiagnosticsPanel client={client} />);
    await screen.findByText("Diagnostics unavailable — records may be incomplete");
    fireEvent.click(screen.getByRole("button", { name: "Export diagnostics" }));
    await screen.findByText(/Nothing was overwritten/);
    client.export.mockRejectedValue("PRIVATE_ERROR_CANARY");
    fireEvent.click(screen.getByRole("button", { name: "Export diagnostics" }));
    await screen.findByText("Diagnostics export unavailable. No automatic retry was made.");
    expect(screen.queryByText("PRIVATE_ERROR_CANARY")).not.toBeInTheDocument();
    client.read.mockRejectedValue("PRIVATE_ERROR_CANARY");
    fireEvent.click(screen.getByRole("button", { name: "Refresh diagnostics" }));
    await waitFor(() => {
      expect(
        screen.getByText("Diagnostics unavailable. Existing operation results are unchanged."),
      ).toBeVisible();
    });
  });
  it("ends the loading state when native access is unavailable", async () => {
    render(
      <DiagnosticsPanel
        client={{ read: vi.fn().mockRejectedValue("PRIVATE_CANARY"), export: vi.fn() }}
      />,
    );
    await screen.findByText("Diagnostics unavailable");
    expect(screen.queryByText("Checking native diagnostics")).not.toBeInTheDocument();
    expect(screen.queryByText("PRIVATE_CANARY")).not.toBeInTheDocument();
  });
});

it("filters provider and request without erasing the full causal timeline from the summary", async () => {
  const request = `sha256:${"d".repeat(64)}`;
  const source = parseDiagnostics({
    available: true,
    events: [
      {
        ...snapshot.events[0],
        request,
        provider: "openai_api",
        model: "gpt-5.6-luna",
        event: "configuration_validated",
      },
      {
        ...snapshot.events[0],
        request,
        provider: "openai_api",
        model: "gpt-5.6-luna",
        event: "request_finished",
        outcome: "failed",
        severity: "error",
        error: "missing_credentials",
      },
      { ...snapshot.events[0], provider: "simulation" },
    ],
  });
  const copy = vi.fn().mockResolvedValue(undefined);
  Object.defineProperty(navigator, "clipboard", { configurable: true, value: { writeText: copy } });
  render(
    <DiagnosticsPanel client={{ read: vi.fn().mockResolvedValue(source), export: vi.fn() }} />,
  );
  await screen.findByText("Logging available");
  fireEvent.change(screen.getByLabelText("Provider/runtime"), { target: { value: "openai_api" } });
  fireEvent.change(screen.getByLabelText("Request/run ID"), { target: { value: request } });
  fireEvent.change(screen.getByLabelText("Severity"), { target: { value: "error" } });
  expect(screen.queryByText("application_started · observed")).not.toBeInTheDocument();
  expect(screen.getByText(/No usable native credential/, { selector: "p" })).toBeVisible();
  fireEvent.click(screen.getByRole("button", { name: "Copy troubleshooting summary" }));
  await screen.findByText("Troubleshooting summary copied.");
  expect(copy.mock.calls[0]?.[0]).toContain("Last observed phase: configuration_validated");
  expect(copy.mock.calls[0]?.[0]).not.toContain("runtime: simulation");
});
