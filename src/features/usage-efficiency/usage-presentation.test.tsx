import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { Metric, UsageDay } from "../../infrastructure/tauri/usage-efficiency-client";
import { usageFixture } from "../../test/usage-efficiency-fixture";
import { TokenComposition, TokenTrend } from "./UsageCharts";
import { displayMetric, trendGeometry } from "./usage-presentation";

const days = (totals: readonly Metric[]): UsageDay[] =>
  totals.map((total, i) => ({ date: `2026-09-${String(24 + i)}`, tokens: [0, 0, 0, 0], total }));

describe("numeric dashboard presentation", () => {
  it("preserves all absence labels and does not attach percent units to absence", () => {
    expect(
      [0, "missing", "unavailable", "unverified"].map((value) =>
        displayMetric(value as Metric, "%"),
      ),
    ).toEqual(["0%", "Missing", "Unavailable", "Unverified"]);
  });
  it("breaks trend segments across each absence without joining or zero-filling", () => {
    const result = trendGeometry(days([600, "missing", 0, "unavailable", 450, "unverified", 700]));
    expect(result.ceiling).toBe(700);
    expect(result.segments.map((segment) => segment.map((point) => point.total))).toEqual([
      [600],
      [0],
      [450],
      [700],
    ]);
    expect(result.segments[1]?.[0]?.y).toBe(210);
    expect(result.segments[3]?.[0]?.y).toBe(40);
  });
  it("keeps zero-only and absent-only series distinct with finite geometry", () => {
    expect(trendGeometry(days([0, 0])).segments[0]?.map((point) => point.y)).toEqual([210, 210]);
    expect(trendGeometry(days(["missing", "unverified"])).segments).toEqual([]);
    expect(trendGeometry([]).segments).toEqual([]);
  });
  it("uses only input and output for composition and labels cached/reasoning as subsets", () => {
    const view = usageFixture().views[0];
    if (!view) throw new Error("Test fixture missing");
    render(<TokenComposition view={view} />);
    expect(screen.getByText("Included in input")).toBeVisible();
    expect(screen.getByText("Included in output")).toBeVisible();
    expect(screen.getByText("1,750")).toBeVisible();
    expect(screen.queryByText("2,190")).not.toBeInTheDocument();
  });
  it("does not draw proportions or a zero line for absent metrics", () => {
    const base = usageFixture().views[0];
    if (!base) throw new Error("Test fixture missing");
    const view = {
      ...base,
      tokens: ["missing", "missing", 250, 40] as const,
      total: "missing" as const,
      daily: days(["missing", "unavailable"]),
    };
    const { container } = render(
      <>
        <TokenComposition view={view} />
        <TokenTrend view={view} />
      </>,
    );
    expect(screen.getByText(/Breakdown incomplete/)).toBeVisible();
    expect(screen.getByText("Daily totals are not available to plot.")).toBeVisible();
    expect(container.querySelectorAll("circle")).toHaveLength(1);
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
  });
});
