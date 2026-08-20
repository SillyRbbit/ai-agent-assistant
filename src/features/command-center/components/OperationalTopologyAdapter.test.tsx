import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { buildCommandCenterProjection } from "../commandCenterFixtures";
import { OperationalTopologyAdapter } from "./OperationalTopologyAdapter";

interface MockFlowProps {
  readonly edges: readonly {
    readonly ariaLabel?: string;
    readonly labelBgStyle?: { readonly fill?: string };
    readonly labelShowBg?: boolean;
  }[];
  readonly maxZoom: number;
  readonly minZoom: number;
  readonly nodes: readonly {
    readonly id: string;
    readonly position: { x: number; y: number };
    readonly style?: { readonly height?: number; readonly width?: number };
  }[];
  readonly panOnDrag: boolean;
  readonly panOnScroll: boolean;
  readonly preventScrolling: boolean;
  readonly translateExtent: readonly [readonly [number, number], readonly [number, number]];
  readonly zoomOnScroll: boolean;
}

const flowHarness = vi.hoisted(() => {
  const fitView = vi.fn(() => Promise.resolve(true));
  const getNodesBounds = vi.fn(() => ({ height: 88, width: 180, x: 0, y: 0 }));
  const getViewport = vi.fn(() => ({ x: 0, y: 0, zoom: 1 }));
  const setViewport = vi.fn(() => Promise.resolve(true));
  const zoomIn = vi.fn(() => Promise.resolve(true));
  const zoomOut = vi.fn(() => Promise.resolve(true));
  return {
    fitView,
    getNodesBounds,
    getViewport,
    instance: {
      fitView,
      getNodesBounds,
      getViewport,
      setViewport,
      viewportInitialized: true,
      zoomIn,
      zoomOut,
    },
    lastProps: null as Record<string, unknown> | null,
    setViewport,
    zoomIn,
    zoomOut,
  };
});

vi.mock("@xyflow/react", async () => {
  const React = await import("react");
  return {
    Handle: () => null,
    MarkerType: { ArrowClosed: "arrow-closed" },
    Position: { Bottom: "bottom", Top: "top" },
    ReactFlow: (props: Record<string, unknown>) => {
      flowHarness.lastProps = props;
      const onInit = props["onInit"];
      React.useEffect(() => {
        if (typeof onInit === "function") {
          (onInit as (instance: typeof flowHarness.instance) => void)(flowHarness.instance);
        }
      }, [onInit]);
      return React.createElement("div", { "data-testid": "react-flow-mock" });
    },
  };
});

interface ResizeObserverHarness {
  readonly callback: ResizeObserverCallback;
  readonly targets: Set<Element>;
}

const resizeObservers: ResizeObserverHarness[] = [];

class ResizeObserverMock implements ResizeObserver {
  readonly harness: ResizeObserverHarness;

  constructor(callback: ResizeObserverCallback) {
    this.harness = { callback, targets: new Set<Element>() };
    resizeObservers.push(this.harness);
  }

  disconnect(): void {
    this.harness.targets.clear();
  }

  observe(target: Element): void {
    this.harness.targets.add(target);
  }

  unobserve(target: Element): void {
    this.harness.targets.delete(target);
  }
}

let nextFrameId = 1;
let frames = new Map<number, FrameRequestCallback>();

function flowProps(): MockFlowProps {
  if (flowHarness.lastProps === null) throw new Error("Expected React Flow to render");
  return flowHarness.lastProps as unknown as MockFlowProps;
}

function flushFrames(): void {
  for (let pass = 0; pass < 4 && frames.size > 0; pass += 1) {
    const pending = [...frames.values()];
    frames = new Map<number, FrameRequestCallback>();
    act(() => {
      for (const callback of pending) callback(performance.now());
    });
  }
}

function resizeCanvas(width: number, height: number): void {
  const canvas = screen.getByRole("listbox", { name: /Simulated operational topology/ });
  const observer = resizeObservers.find((candidate) => candidate.targets.has(canvas));
  if (observer === undefined) throw new Error("Expected the topology ResizeObserver");
  const contentRect = {
    bottom: height,
    height,
    left: 0,
    right: width,
    toJSON: () => ({}),
    top: 0,
    width,
    x: 0,
    y: 0,
  };
  act(() => {
    observer.callback(
      [{ contentRect, target: canvas } as unknown as ResizeObserverEntry],
      {} as ResizeObserver,
    );
  });
  flushFrames();
}

function renderTopology(selectedId: string | null = null): void {
  render(
    <OperationalTopologyAdapter
      onSelect={vi.fn()}
      projection={buildCommandCenterProjection("catalog-idle")}
      selectedId={selectedId}
    />,
  );
}

beforeEach(() => {
  resizeObservers.length = 0;
  frames = new Map<number, FrameRequestCallback>();
  nextFrameId = 1;
  flowHarness.fitView.mockClear();
  flowHarness.getNodesBounds.mockClear();
  flowHarness.getViewport.mockClear();
  flowHarness.setViewport.mockClear();
  flowHarness.zoomIn.mockClear();
  flowHarness.zoomOut.mockClear();
  flowHarness.lastProps = null;
  vi.stubGlobal("ResizeObserver", ResizeObserverMock);
  vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
    const id = nextFrameId;
    nextFrameId += 1;
    frames.set(id, callback);
    return id;
  });
  vi.stubGlobal("cancelAnimationFrame", (id: number) => {
    frames.delete(id);
  });
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({
    bottom: 0,
    height: 0,
    left: 0,
    right: 0,
    toJSON: () => ({}),
    top: 0,
    width: 0,
    x: 0,
    y: 0,
  });
});

afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("OperationalTopologyAdapter viewport contract", () => {
  it("waits for dimensions, then fits the compact layout with bounded controls", () => {
    renderTopology();
    flushFrames();
    expect(flowHarness.fitView).not.toHaveBeenCalled();

    resizeCanvas(502, 500);

    const canvas = screen.getByRole("listbox", { name: /Simulated operational topology/ });
    expect(canvas).toHaveAttribute("data-layout-mode", "compact");
    expect(flowHarness.fitView).toHaveBeenCalledWith(
      expect.objectContaining({ duration: 0, maxZoom: 1, minZoom: 0.5, padding: "24px" }),
    );
    expect(flowProps()).toMatchObject({
      maxZoom: 1.5,
      minZoom: 0.5,
      panOnDrag: true,
      panOnScroll: false,
      preventScrolling: false,
      zoomOnScroll: false,
    });
    expect(flowProps().translateExtent[0][0]).toBeLessThan(flowProps().translateExtent[1][0]);
    expect(flowProps().translateExtent[0][1]).toBeLessThan(flowProps().translateExtent[1][1]);
    expect(flowProps().edges).not.toHaveLength(0);
    for (const edge of flowProps().edges) {
      expect(edge.ariaLabel).toBeTruthy();
      expect(edge.labelShowBg).toBe(true);
      expect(edge.labelBgStyle).toEqual({ fill: "var(--command-surface)" });
    }
    const bounds = flowProps().nodes.reduce(
      (current, node) => ({
        maximumX: Math.max(current.maximumX, node.position.x + (node.style?.width ?? 180)),
        maximumY: Math.max(current.maximumY, node.position.y + (node.style?.height ?? 88)),
        minimumX: Math.min(current.minimumX, node.position.x),
        minimumY: Math.min(current.minimumY, node.position.y),
      }),
      {
        maximumX: Number.NEGATIVE_INFINITY,
        maximumY: Number.NEGATIVE_INFINITY,
        minimumX: Number.POSITIVE_INFINITY,
        minimumY: Number.POSITIVE_INFINITY,
      },
    );
    const compactFitZoom = Math.min(
      1,
      (502 - 48) / (bounds.maximumX - bounds.minimumX),
      (500 - 48) / (bounds.maximumY - bounds.minimumY),
    );
    expect(compactFitZoom).toBeGreaterThanOrEqual(0.5);

    const semanticNodes = flowProps().nodes.filter((node) => !node.id.startsWith("demo-group:"));
    expect(semanticNodes).not.toHaveLength(0);
    for (const node of semanticNodes) {
      expect(node.style).toEqual({ height: 88, width: 180 });
    }

    const groupMembers = {
      "demo-group:core": ["personal-assistant", "workflow-automation"],
      "demo-group:engineering": ["coding", "qa-validation"],
      "demo-group:governance": ["security-risk"],
      "demo-group:infrastructure": ["cloud-infrastructure", "systems-operations"],
      "demo-group:intelligence": ["research", "knowledge-document"],
    } as const;
    for (const [groupId, memberIds] of Object.entries(groupMembers)) {
      const lane = flowProps().nodes.find((node) => node.id === groupId);
      if (lane === undefined) throw new Error("Expected group lane " + groupId);
      const laneWidth = lane.style?.width ?? 0;
      const laneHeight = lane.style?.height ?? 0;
      for (const memberId of memberIds) {
        const member = flowProps().nodes.find((node) => node.id === "demo-node:agent:" + memberId);
        if (member === undefined) throw new Error("Expected agent node " + memberId);
        expect(member.position.x).toBeGreaterThanOrEqual(lane.position.x);
        expect(member.position.y).toBeGreaterThanOrEqual(lane.position.y + 24);
        expect(member.position.x + 180).toBeLessThanOrEqual(lane.position.x + laneWidth);
        expect(member.position.y + 88).toBeLessThanOrEqual(lane.position.y + laneHeight);
      }
    }
  });

  it("switches to the wide topology and automatically refits while in automatic mode", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(760, 520);
    flowHarness.fitView.mockClear();

    resizeCanvas(1400, 700);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "wide");
    expect(flowHarness.fitView).toHaveBeenCalledTimes(1);
    const semanticNodes = flowProps().nodes.filter((node) => !node.id.startsWith("demo-group:"));
    expect(Math.max(...semanticNodes.map((node) => node.position.x))).toBeGreaterThan(1500);
  });

  it("preserves manual viewport state across resize until Fit or Reset resumes auto framing", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(760, 520);
    flowHarness.fitView.mockClear();

    fireEvent.click(screen.getByRole("button", { name: "Zoom in" }));
    expect(flowHarness.zoomIn).toHaveBeenCalledWith({ duration: 0 });
    fireEvent.click(screen.getByRole("button", { name: "Zoom out" }));
    expect(flowHarness.zoomOut).toHaveBeenCalledWith({ duration: 0 });
    resizeCanvas(820, 540);
    expect(flowHarness.fitView).not.toHaveBeenCalled();

    fireEvent.click(screen.getByRole("button", { name: "Fit view" }));
    expect(flowHarness.fitView).toHaveBeenCalledTimes(1);
    resizeCanvas(840, 560);
    expect(flowHarness.fitView).toHaveBeenCalledTimes(2);

    fireEvent.click(screen.getByRole("button", { name: "Reset view" }));
    expect(flowHarness.fitView).toHaveBeenCalledTimes(3);
    expect(flowHarness.setViewport).not.toHaveBeenCalled();
  });

  it("reframes changed topology content even after a manual viewport interaction", () => {
    const idleProjection = buildCommandCenterProjection("catalog-idle");
    const { rerender } = render(
      <OperationalTopologyAdapter
        onSelect={vi.fn()}
        projection={idleProjection}
        selectedId={null}
      />,
    );
    flushFrames();
    resizeCanvas(760, 520);
    flowHarness.fitView.mockClear();
    fireEvent.click(screen.getByRole("button", { name: "Zoom in" }));

    rerender(
      <OperationalTopologyAdapter
        onSelect={vi.fn()}
        projection={buildCommandCenterProjection("research-queued")}
        selectedId={null}
      />,
    );
    flushFrames();

    expect(flowHarness.fitView).toHaveBeenCalledTimes(1);
  });

  it("centers selected context with bounded zoom without taking wheel ownership", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    renderTopology(projection.nodes[0]?.id ?? null);
    flushFrames();
    resizeCanvas(760, 520);
    flowHarness.fitView.mockClear();
    flowHarness.getViewport.mockReturnValue({ x: -1000, y: -1000, zoom: 1 });

    fireEvent.click(screen.getByRole("button", { name: "Center selected" }));

    expect(flowHarness.getNodesBounds).toHaveBeenCalled();
    expect(flowHarness.fitView).toHaveBeenCalledWith(
      expect.objectContaining({ maxZoom: 1.1, minZoom: 0.5, padding: "72px" }),
    );
    expect(flowProps()).toMatchObject({ panOnScroll: false, preventScrolling: false });
  });
});
