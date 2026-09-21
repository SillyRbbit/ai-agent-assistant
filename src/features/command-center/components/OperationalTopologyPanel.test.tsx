import { render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { buildCommandCenterProjection } from "../commandCenterFixtures";
import type { TopologyNode } from "../commandCenterProjection";
import { OperationalTopologyPanel } from "./OperationalTopologyPanel";

vi.mock("./OperationalTopologyAdapter", async () => {
  const React = await import("react");
  return {
    OperationalTopologyAdapter: ({
      visibleNodes,
    }: {
      readonly visibleNodes?: readonly TopologyNode[];
    }) => {
      const id = visibleNodes?.[0]?.id ?? "empty";
      if (id === "demo-node:renderer-failure") throw new Error("bounded renderer failure");
      return React.createElement("p", null, `Rendered ${id}`);
    },
  };
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("OperationalTopologyPanel Graph boundary", () => {
  it("resets a latched render failure for a same-count dataset with different exact IDs", () => {
    vi.spyOn(console, "error").mockImplementation(() => undefined);
    const projection = buildCommandCenterProjection("catalog-idle");
    const sourceNode = projection.nodes[0];
    if (sourceNode === undefined) throw new Error("Expected fixture node");
    const failedNode = { ...sourceNode, id: "demo-node:renderer-failure" } as TopologyNode;
    const recoveredNode = { ...sourceNode, id: "demo-node:renderer-recovered" } as TopologyNode;
    const { rerender } = render(
      <OperationalTopologyPanel
        onSelect={vi.fn()}
        projection={projection}
        selectedId={null}
        visibleEdges={[]}
        visibleNodes={[failedNode]}
      />,
    );

    expect(screen.getByRole("alert")).toHaveTextContent("Graph rendering unavailable");

    rerender(
      <OperationalTopologyPanel
        onSelect={vi.fn()}
        projection={projection}
        selectedId={null}
        visibleEdges={[]}
        visibleNodes={[recoveredNode]}
      />,
    );

    expect(screen.getByText("Rendered demo-node:renderer-recovered")).toBeInTheDocument();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });
});
