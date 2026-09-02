import { fireEvent, render, screen } from "@testing-library/react";
import { Component, type ReactNode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { GraphRenderBoundary } from "./GraphRenderBoundary";

class ControlledFailure extends Component<
  Readonly<{ children?: ReactNode; fail: boolean }>,
  Record<string, never>
> {
  override render() {
    if (this.props.fail) throw new Error("bounded renderer failure");
    return <p>Graph recovered</p>;
  }
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("GraphRenderBoundary", () => {
  it("contains a Graph-only render failure and retries without claiming execution", () => {
    vi.spyOn(console, "error").mockImplementation(() => undefined);
    const { rerender } = render(
      <GraphRenderBoundary resetKey="first">
        <ControlledFailure fail />
      </GraphRenderBoundary>,
    );

    const alert = screen.getByRole("alert");
    expect(alert).toHaveTextContent("Graph rendering unavailable");
    expect(alert).toHaveTextContent("No task, provider, tool, or device action was attempted");
    expect(screen.getByRole("toolbar", { name: "Topology viewport" })).toBeInTheDocument();
    expect(screen.getByRole("status", { name: "Topology zoom level" })).toHaveTextContent("N/A");
    expect(screen.getByRole("button", { name: "Fit view" })).toBeDisabled();
    expect(screen.getByLabelText("Show relationship legend")).toBeInTheDocument();

    rerender(
      <GraphRenderBoundary resetKey="first">
        <ControlledFailure fail={false} />
      </GraphRenderBoundary>,
    );
    fireEvent.click(screen.getByRole("button", { name: "Retry graph renderer" }));
    expect(screen.getByText("Graph recovered")).toBeInTheDocument();
  });
});
