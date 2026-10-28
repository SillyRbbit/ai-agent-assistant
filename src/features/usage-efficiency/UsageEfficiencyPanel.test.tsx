import { fireEvent, render, screen, waitFor, within, act } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { usageFixture } from "../../test/usage-efficiency-fixture";
import type { UsageProjection } from "../../infrastructure/tauri/usage-efficiency-client";
import { UsageEfficiencyPanel } from "./UsageEfficiencyPanel";
import { ActivityPage } from "../activity/ActivityPage";

describe("mock-only Usage & Efficiency", () => {
  it("shares all six source choices between responsive controls without implicit loading", async () => {
    const loader = vi.fn().mockResolvedValue(usageFixture());
    render(<UsageEfficiencyPanel loader={loader} />);
    const sources = within(screen.getByRole("group", { name: "Sample source" }));
    const select = screen.getByLabelText("Example connection");
    expect(sources.getAllByRole("radio")).toHaveLength(6);
    for (const [name, value] of [
      ["Simulation", "simulation"],
      ["OpenAI API", "openai_api"],
      ["Anthropic API", "anthropic_api"],
      ["LM Studio", "lm_studio"],
      ["Ollama", "ollama"],
      ["Codex", "codex"],
    ]) {
      if (!name || !value) throw new Error("Expected a source pair");
      fireEvent.click(sources.getByRole("radio", { name }));
      expect(select).toHaveValue(value);
      expect(sources.getByRole("radio", { name })).toBeChecked();
    }
    expect(loader).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Load examples" }));
    await screen.findByRole("img", { name: /Daily sample token activity/ });
    fireEvent.click(sources.getByRole("radio", { name: "Codex" }));
    fireEvent.change(select, { target: { value: "codex" } });
    expect(screen.getByRole("img", { name: /Daily sample token activity/ })).toBeVisible();
    fireEvent.change(select, { target: { value: "simulation" } });
    expect(sources.getByRole("radio", { name: "Simulation" })).toBeChecked();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
    expect(loader).toHaveBeenCalledTimes(1);
  });

  it("isolates source radio groups and invalidates a pending load on source-radio change", async () => {
    let resolve: ((p: UsageProjection) => void) | undefined;
    const loader = vi.fn(
      () =>
        new Promise<UsageProjection>((next) => {
          resolve = next;
        }),
    );
    render(
      <>
        <UsageEfficiencyPanel loader={loader} />
        <UsageEfficiencyPanel />
      </>,
    );
    const [left, right] = screen.getAllByRole("group", { name: "Sample source" });
    if (!left || !right) throw new Error("Expected two source groups");
    const a = within(left),
      b = within(right);
    expect(a.getByRole("radio", { name: "Simulation" }).getAttribute("name")).not.toEqual(
      b.getByRole("radio", { name: "Simulation" }).getAttribute("name"),
    );
    const [load] = screen.getAllByRole("button", { name: "Load examples" });
    if (!load) throw new Error("Expected load control");
    fireEvent.click(load);
    fireEvent.click(a.getByRole("radio", { name: "Ollama" }));
    await act(() => {
      resolve?.(usageFixture());
      return Promise.resolve();
    });
    expect(a.getByRole("radio", { name: "Ollama" })).toBeChecked();
    expect(b.getByRole("radio", { name: "Simulation" })).toBeChecked();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
    expect(loader).toHaveBeenCalledTimes(1);
  });

  it("is opt-in and discloses mock provenance permanently", () => {
    const loader = vi.fn();
    render(<UsageEfficiencyPanel loader={loader} />);
    expect(loader).not.toHaveBeenCalled();
    expect(
      screen.getByText("Mock only — Cortexa-shaped examples, not your activity."),
    ).toBeVisible();
    expect(screen.getAllByRole("option").map((o) => o.textContent)).toEqual(
      expect.arrayContaining([
        "Simulation",
        "OpenAI API",
        "Anthropic API",
        "LM Studio",
        "Ollama",
        "Codex",
      ]),
    );
  });
  it("shows Rust-selected totals, range rows and distinct absence labels", async () => {
    const loader = vi.fn().mockResolvedValue(usageFixture());
    render(<UsageEfficiencyPanel loader={loader} />);
    fireEvent.click(screen.getByRole("button", { name: /^(Load|Refresh) examples$/ }));
    await screen.findByRole("img", { name: /Daily sample token activity/ });
    fireEvent.click(screen.getByText(/Daily token details/, { selector: "summary" }));
    expect(screen.getByRole("table")).toBeVisible();
    expect(screen.getAllByText("1,750")[0]).toBeVisible();
    expect(within(screen.getByRole("table")).getAllByRole("row")).toHaveLength(8);
    fireEvent.change(screen.getByLabelText("Example connection"), { target: { value: "codex" } });
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    expect(
      screen.queryByRole("img", { name: /Daily sample token activity/ }),
    ).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("radio", { name: "30 days" }));
    fireEvent.click(screen.getByRole("button", { name: /^(Load|Refresh) examples$/ }));
    await screen.findByRole("img", { name: /Daily sample token activity/ });
    fireEvent.click(screen.getByText(/Daily token details/, { selector: "summary" }));
    expect(screen.getAllByText("3,080")[0]).toBeVisible();
    expect(screen.getAllByText("Missing").length).toBeGreaterThan(0);
    expect(within(screen.getByRole("table")).getAllByRole("row")).toHaveLength(31);
  });
  it("keeps the main toolbar compact and moves diagnostics and raw rows into disclosures", async () => {
    render(<UsageEfficiencyPanel loader={() => Promise.resolve(usageFixture())} />);
    expect(screen.queryByRole("button", { name: /Rust/ })).not.toBeInTheDocument();
    expect(screen.getByText("Preview controls").closest("details")).not.toHaveAttribute("open");
    fireEvent.click(screen.getByRole("button", { name: "Load examples" }));
    await screen.findByRole("img", { name: /Daily sample token activity/ });
    expect(screen.getByRole("button", { name: "Refresh examples" })).toBeVisible();
    expect(
      screen.getByText(/Daily token details/, { selector: "summary" }).closest("details"),
    ).not.toHaveAttribute("open");
    expect(screen.getByRole("heading", { name: "Token breakdown" })).toBeVisible();
    expect(screen.getByText("Sample data")).toBeVisible();
    expect(screen.getByText("How to read these examples").closest("details")).not.toHaveAttribute(
      "open",
    );
  });
  it("groups named range segments and preserves a loaded view when reselecting the current range", async () => {
    const loader = vi.fn().mockResolvedValue(usageFixture());
    render(<UsageEfficiencyPanel loader={loader} />);
    const range = within(screen.getByRole("group", { name: "Fictional UTC range" }));
    expect(range.getAllByRole("radio")).toHaveLength(2);
    expect(range.getByRole("radio", { name: "7 days" })).toBeChecked();
    expect(range.getByRole("radio", { name: "30 days" })).not.toBeChecked();
    fireEvent.click(screen.getByRole("button", { name: "Load examples" }));
    await screen.findByRole("img", { name: /Daily sample token activity/ });
    fireEvent.click(range.getByRole("radio", { name: "7 days" }));
    expect(screen.getByRole("img", { name: /Daily sample token activity/ })).toBeVisible();
    fireEvent.click(range.getByRole("radio", { name: "30 days" }));
    expect(range.getByRole("radio", { name: "7 days" })).not.toBeChecked();
    expect(range.getByRole("radio", { name: "30 days" })).toBeChecked();
    expect(
      screen.queryByRole("img", { name: /Daily sample token activity/ }),
    ).not.toBeInTheDocument();
    expect(loader).toHaveBeenCalledTimes(1);
  });
  it("isolates native radio groups between panel instances", () => {
    render(
      <>
        <UsageEfficiencyPanel />
        <UsageEfficiencyPanel />
      </>,
    );
    const ranges = screen.getAllByRole("group", { name: "Fictional UTC range" });
    const [firstRange, secondRange] = ranges;
    if (!firstRange || !secondRange) throw new Error("Expected two independent range groups");
    const first = within(firstRange);
    const second = within(secondRange);
    expect(first.getByRole("radio", { name: "7 days" }).getAttribute("name")).not.toEqual(
      second.getByRole("radio", { name: "7 days" }).getAttribute("name"),
    );
    fireEvent.click(first.getByRole("radio", { name: "30 days" }));
    expect(first.getByRole("radio", { name: "30 days" })).toBeChecked();
    expect(second.getByRole("radio", { name: "7 days" })).toBeChecked();
  });
  it("rejects a late load after a range segment changes without issuing a new request", async () => {
    let resolve: ((p: UsageProjection) => void) | undefined;
    const loader = vi.fn(
      () =>
        new Promise<UsageProjection>((next) => {
          resolve = next;
        }),
    );
    render(<UsageEfficiencyPanel loader={loader} />);
    fireEvent.click(screen.getByRole("button", { name: "Load examples" }));
    fireEvent.click(screen.getByRole("radio", { name: "30 days" }));
    await act(() => {
      resolve?.(usageFixture());
      return Promise.resolve();
    });
    expect(screen.getByRole("radio", { name: "30 days" })).toBeChecked();
    expect(
      screen.queryByRole("img", { name: /Daily sample token activity/ }),
    ).not.toBeInTheDocument();
    expect(loader).toHaveBeenCalledTimes(1);
  });
  it("renders empty/loading/error illustrations without live fallback", async () => {
    const loader = vi.fn().mockResolvedValue(usageFixture());
    render(<UsageEfficiencyPanel loader={loader} />);
    fireEvent.change(screen.getByLabelText("Example state"), { target: { value: "empty" } });
    fireEvent.click(screen.getByRole("button", { name: /^(Load|Refresh) examples$/ }));
    expect(
      await screen.findByText("No fictional activity in this complete empty example."),
    ).toBeVisible();
    expect(screen.getAllByText("0").length).toBeGreaterThan(1);
    expect(screen.getAllByText("Unavailable").length).toBeGreaterThanOrEqual(2);
    fireEvent.change(screen.getByLabelText("Example state"), { target: { value: "loading" } });
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    expect(
      screen.queryByRole("img", { name: /Daily sample token activity/ }),
    ).not.toBeInTheDocument();
    expect(screen.getByRole("status")).toHaveTextContent("no request is pending");
    fireEvent.change(screen.getByLabelText("Example state"), { target: { value: "error" } });
    expect(screen.getByRole("status")).toHaveTextContent("Error illustration");
    expect(loader).toHaveBeenCalledTimes(1);
  });
  it("discards late success after a selection change and unmount", async () => {
    let resolve: ((p: UsageProjection) => void) | undefined;
    const loader = () =>
      new Promise<UsageProjection>((next) => {
        resolve = next;
      });
    const ui = render(<UsageEfficiencyPanel loader={loader} />);
    fireEvent.click(screen.getByRole("button", { name: /^(Load|Refresh) examples$/ }));
    expect(screen.getByRole("region", { name: "Usage & Efficiency" })).toHaveAttribute(
      "aria-busy",
      "true",
    );
    fireEvent.change(screen.getByLabelText("Example connection"), { target: { value: "ollama" } });
    await act(() => {
      resolve?.(usageFixture());
      return Promise.resolve();
    });
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    expect(
      screen.queryByRole("img", { name: /Daily sample token activity/ }),
    ).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /^(Load|Refresh) examples$/ }));
    ui.unmount();
    await act(() => {
      resolve?.(usageFixture());
      return Promise.resolve();
    });
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    expect(
      screen.queryByRole("img", { name: /Daily sample token activity/ }),
    ).not.toBeInTheDocument();
  });
  it("clears stale data before refresh failure and exposes no raw error", async () => {
    const loader = vi
      .fn()
      .mockResolvedValueOnce(usageFixture())
      .mockRejectedValueOnce(new Error("private response"));
    render(<UsageEfficiencyPanel loader={loader} />);
    fireEvent.click(screen.getByRole("button", { name: /^(Load|Refresh) examples$/ }));
    await screen.findByRole("table");
    fireEvent.click(screen.getByRole("button", { name: /^(Load|Refresh) examples$/ }));
    await waitFor(() => {
      expect(screen.getByRole("status")).toHaveTextContent("No live fallback");
    });
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    expect(
      screen.queryByRole("img", { name: /Daily sample token activity/ }),
    ).not.toBeInTheDocument();
    expect(screen.queryByText("private response")).not.toBeInTheDocument();
  });
  it("preserves the existing empty Activity presentation", () => {
    render(<ActivityPage events={[]} />);
    expect(screen.getByRole("heading", { name: "Activity" })).toBeVisible();
    expect(screen.getByText("No session activity")).toBeVisible();
    expect(screen.getByRole("region", { name: "Usage & Efficiency" })).toBeVisible();
  });
  it("preserves reverse-chronological redacted Activity events", () => {
    const events = [1, 2].map((n) => ({
      id: `fixture-${String(n)}`,
      runId: "mock-run-1",
      kind: "run-started" as const,
      summary: `Fixture event ${String(n)}`,
      detail: "Redacted fixture",
      tone: "neutral" as const,
    }));
    render(<ActivityPage events={events} />);
    const items = within(screen.getByRole("list", { name: "Run activity" })).getAllByRole(
      "listitem",
    );
    expect(items[0]).toHaveTextContent("Fixture event 2");
    expect(items[1]).toHaveTextContent("Fixture event 1");
  });
});
