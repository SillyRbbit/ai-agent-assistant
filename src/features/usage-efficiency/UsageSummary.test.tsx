import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { usageFixture } from "../../test/usage-efficiency-fixture";
import { UsageSummary } from "./UsageSummary";

function sample() {
  const view = usageFixture().views.find(
    (v) => v.category === "simulation" && v.days === 7 && v.example === "activity",
  );
  if (!view) throw new Error("Expected synthetic sample");
  return { ...view, tokens: [1500, 400, 250, 40] as const, cachePercent: 26.67 };
}

describe("mock summary hierarchy", () => {
  it("presents the supplied total once with input/output subsets and task coverage", () => {
    const { container } = render(<UsageSummary view={sample()} />);
    expect(screen.getByText("1,750")).toBeVisible();
    expect(screen.getByText("1,500")).toBeVisible();
    expect(screen.getByText("250")).toBeVisible();
    expect(screen.getByText("26.67%")).toBeVisible();
    expect(screen.getByText(/Cached input: 400 · included in input/)).toBeVisible();
    expect(screen.getByText(/Reasoning: 40 · included in output/)).toBeVisible();
    expect(screen.getByText("Missing")).toBeVisible();
    expect(screen.getByText("Incomplete task coverage")).toBeVisible();
    expect(container.querySelector(".usage-efficiency__completed dd")).toHaveTextContent("1");
  });
  it("never substitutes zero or percent signs for absent metrics", () => {
    const { container } = render(
      <UsageSummary
        view={{
          ...sample(),
          total: "unavailable",
          tokens: ["missing", "unverified", "unavailable", "missing"],
          cachePercent: "unverified",
          completedTaskMean: "missing",
        }}
      />,
    );
    expect(container.querySelector(".usage-efficiency__total dd")).toHaveTextContent("Unavailable");
    expect(container.querySelector(".usage-efficiency__cache dd")).toHaveTextContent(
      /^Unverified$/,
    );
    expect(container.querySelector("svg")).toBeNull();
    expect(container.textContent).not.toContain("%");
    expect(screen.getByText("Missing", { selector: ".usage-efficiency__input dd" })).toBeVisible();
  });
  it("retains supplied zero and omits a missing-coverage explanation for available averages", () => {
    const { container } = render(
      <UsageSummary
        view={{
          ...sample(),
          total: 0,
          tokens: [0, 0, 0, 0],
          cachePercent: 0,
          completedTaskMean: 0,
          completedTasks: 0,
        }}
      />,
    );
    expect(container.querySelector(".usage-efficiency__total dd")).toHaveTextContent(/^0$/);
    expect(screen.getByText("0%")).toBeVisible();
    expect(screen.queryByText("Incomplete task coverage")).not.toBeInTheDocument();
    expect(container.querySelector(".usage-efficiency__completed dd")).toHaveTextContent(/^0$/);
  });
});
