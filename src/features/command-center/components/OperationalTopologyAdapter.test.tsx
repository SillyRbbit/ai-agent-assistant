import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { buildCommandCenterProjection } from "../commandCenterFixtures";
import { OperationalTopologyAdapter } from "./OperationalTopologyAdapter";

interface MockFlowProps {
  readonly edges: readonly {
    readonly ariaLabel?: string;
    readonly className?: string;
    readonly label?: string;
    readonly labelBgStyle?: { readonly fill?: string };
    readonly labelShowBg?: boolean;
    readonly markerEnd?: { readonly type?: string };
  }[];
  readonly maxZoom: number;
  readonly minZoom: number;
  readonly nodes: readonly {
    readonly data?: {
      readonly isConnected?: boolean;
      readonly isDimmed?: boolean;
      readonly isSelected?: boolean;
      readonly label?: string;
    };
    readonly id: string;
    readonly position: { x: number; y: number };
    readonly style?: {
      readonly height?: number;
      readonly width?: number;
      readonly zIndex?: number;
    };
  }[];
  readonly panOnDrag: boolean;
  readonly panOnScroll: boolean;
  readonly preventScrolling: boolean;
  readonly translateExtent: readonly [readonly [number, number], readonly [number, number]];
  readonly zoomOnScroll: boolean;
  readonly onEdgeClick?: (event: unknown, edge: { readonly id: string }) => void;
  readonly onMove?: (event: unknown, viewport: { x: number; y: number; zoom: number }) => void;
  readonly onMoveStart?: (event: unknown) => void;
  readonly onNodeClick?: (event: unknown, node: { readonly id: string }) => void;
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

function expectDomainLaneClearance(): void {
  const graphNodes = flowProps().nodes;
  const orchestrator = graphNodes.find((node) => node.id === "demo-node:orchestrator");
  const lanes = graphNodes.filter((node) => node.id.startsWith("demo-group:"));
  const semanticNodes = graphNodes.filter((node) => !node.id.startsWith("demo-group:"));
  if (orchestrator === undefined || lanes.length === 0) {
    throw new Error("Expected orchestrator and domain lanes");
  }

  const orchestratorBottom = orchestrator.position.y + (orchestrator.style?.height ?? 104);
  const firstLaneTop = Math.min(...lanes.map((lane) => lane.position.y));
  expect(firstLaneTop - orchestratorBottom).toBeGreaterThanOrEqual(32);

  for (let leftIndex = 0; leftIndex < lanes.length; leftIndex += 1) {
    const left = lanes[leftIndex];
    if (left === undefined) continue;
    for (let rightIndex = leftIndex + 1; rightIndex < lanes.length; rightIndex += 1) {
      const right = lanes[rightIndex];
      if (right === undefined) continue;
      const horizontalClearance = Math.max(
        right.position.x - (left.position.x + (left.style?.width ?? 0)),
        left.position.x - (right.position.x + (right.style?.width ?? 0)),
      );
      const verticalClearance = Math.max(
        right.position.y - (left.position.y + (left.style?.height ?? 0)),
        left.position.y - (right.position.y + (right.style?.height ?? 0)),
      );
      expect(Math.max(horizontalClearance, verticalClearance)).toBeGreaterThanOrEqual(8);
    }
  }

  expect(lanes.every((lane) => lane.style?.zIndex === 1)).toBe(true);
  expect(semanticNodes.every((node) => node.style?.zIndex === 2)).toBe(true);
}

function renderTopology(
  selectedId: string | null = null,
  scenario: Parameters<typeof buildCommandCenterProjection>[0] = "catalog-idle",
): void {
  render(
    <OperationalTopologyAdapter
      onClearSelection={vi.fn()}
      onSelect={vi.fn()}
      projection={buildCommandCenterProjection(scenario)}
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
      preventScrolling: true,
      zoomOnScroll: true,
    });
    expect(
      screen.getByText(/scroll up to zoom in and scroll down to zoom out/i),
    ).toBeInTheDocument();
    expect(flowProps().translateExtent[0][0]).toBeLessThan(flowProps().translateExtent[1][0]);
    expect(flowProps().translateExtent[0][1]).toBeLessThan(flowProps().translateExtent[1][1]);
    expect(flowProps().edges).not.toHaveLength(0);
    for (const edge of flowProps().edges) {
      expect(edge.ariaLabel).toBeTruthy();
      expect(edge.label).toBeUndefined();
      expect(edge.labelShowBg).toBe(false);
      expect(edge.labelBgStyle).toEqual({ fill: "var(--command-surface)" });
      expect(edge.markerEnd?.type).toBe("arrow-closed");
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
      expect(node.style?.width).toBe(196);
      expect(node.style?.height).toBeGreaterThanOrEqual(104);
      expect(node.style?.height).toBeLessThanOrEqual(120);
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
        const memberWidth = member.style?.width ?? 196;
        const memberHeight = member.style?.height ?? 104;
        expect(member.position.y).toBeGreaterThanOrEqual(lane.position.y + 24);
        expect(member.position.x + memberWidth).toBeLessThanOrEqual(lane.position.x + laneWidth);
        expect(member.position.y + memberHeight).toBeLessThanOrEqual(lane.position.y + laneHeight);
      }
    }
    expectDomainLaneClearance();
  });

  it("uses wide topology only when its content fits at a readable automatic zoom", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(760, 520);
    flowHarness.fitView.mockClear();

    resizeCanvas(1400, 700);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    expect(flowHarness.fitView).toHaveBeenCalledTimes(1);
    flowHarness.fitView.mockClear();

    resizeCanvas(5120, 1100);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "wide");
    expect(flowHarness.fitView).toHaveBeenCalledTimes(1);
    const semanticNodes = flowProps().nodes.filter((node) => !node.id.startsWith("demo-group:"));
    const agentRows = new Set(
      semanticNodes
        .filter((node) => node.id.startsWith("demo-node:agent:"))
        .map((node) => node.position.y),
    );
    expect(agentRows.size).toBeGreaterThan(1);
    const bounds = flowProps().nodes.reduce(
      (current, node) => ({
        maximumX: Math.max(current.maximumX, node.position.x + (node.style?.width ?? 196)),
        maximumY: Math.max(current.maximumY, node.position.y + (node.style?.height ?? 120)),
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
    expect(
      (bounds.maximumX - bounds.minimumX) / (bounds.maximumY - bounds.minimumY),
    ).toBeLessThanOrEqual(3);
    expect(flowHarness.fitView).toHaveBeenCalledWith(expect.objectContaining({ maxZoom: 1.5 }));
    expectDomainLaneClearance();
  });

  it("keeps workspace domain labels clear of routing and adjacent lanes", () => {
    renderTopology();
    flushFrames();

    resizeCanvas(1400, 700);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    expectDomainLaneClearance();
  });

  it("uses one agent height and common workspace row edges", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(1400, 700);

    const nodes = flowProps().nodes;
    const requireNode = (id: string) => {
      const node = nodes.find((candidate) => candidate.id === id);
      if (node === undefined) throw new Error(`Expected graph node ${id}`);
      return node;
    };
    const primaryIds = [
      "demo-node:agent:personal-assistant",
      "demo-node:agent:research",
      "demo-node:agent:coding",
      "demo-node:agent:cloud-infrastructure",
      "demo-node:agent:security-risk",
    ];
    const secondaryIds = [
      "demo-node:agent:workflow-automation",
      "demo-node:agent:knowledge-document",
      "demo-node:agent:qa-validation",
      "demo-node:agent:systems-operations",
    ];
    const twoMemberLaneIds = [
      "demo-group:core",
      "demo-group:intelligence",
      "demo-group:engineering",
      "demo-group:infrastructure",
    ];
    const agents = nodes.filter((node) => node.id.startsWith("demo-node:agent:"));
    const primaryNodes = primaryIds.map(requireNode);
    const secondaryNodes = secondaryIds.map(requireNode);
    const nodeBottom = (node: (typeof nodes)[number]) =>
      node.position.y + (node.style?.height ?? 0);

    expect(agents).toHaveLength(9);
    expect(new Set(agents.map((node) => node.style?.width))).toEqual(new Set([196]));
    expect(new Set(agents.map((node) => node.style?.height))).toEqual(new Set([120]));
    expect(requireNode("demo-node:orchestrator").style?.height).toBe(104);
    expect(new Set(primaryNodes.map((node) => node.position.y))).toEqual(new Set([180]));
    expect(new Set(primaryNodes.map(nodeBottom))).toEqual(new Set([300]));
    expect(new Set(secondaryNodes.map((node) => node.position.y))).toEqual(new Set([312]));
    expect(new Set(secondaryNodes.map(nodeBottom))).toEqual(new Set([432]));
    expect(
      Math.min(...secondaryNodes.map((node) => node.position.y)) -
        Math.max(...primaryNodes.map(nodeBottom)),
    ).toBe(12);
    expect(
      new Set(
        twoMemberLaneIds.map((id) => {
          const lane = requireNode(id);
          return lane.position.y + (lane.style?.height ?? 0);
        }),
      ).size,
    ).toBe(1);
  });

  it("does not change layout at the former 1279 to 1280 pixel breakpoint", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(1279, 700);
    const positionsAt1279 = flowProps().nodes.map((node) => ({
      id: node.id,
      position: node.position,
    }));

    resizeCanvas(1280, 700);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    expect(flowProps().nodes.map((node) => ({ id: node.id, position: node.position }))).toEqual(
      positionsAt1279,
    );
  });

  it("keeps the active workspace fit-capable when an inspector narrows the canvas", () => {
    renderTopology(null, "research-knowledge-active");
    flushFrames();
    resizeCanvas(633, 351);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    const bounds = flowProps().nodes.reduce(
      (current, node) => ({
        maximumX: Math.max(current.maximumX, node.position.x + (node.style?.width ?? 196)),
        maximumY: Math.max(current.maximumY, node.position.y + (node.style?.height ?? 120)),
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
    const fitZoom = Math.min(
      1,
      (633 - 48) / (bounds.maximumX - bounds.minimumX),
      (351 - 48) / (bounds.maximumY - bounds.minimumY),
    );
    expect(fitZoom).toBeGreaterThanOrEqual(0.5);
    expect(flowHarness.fitView).toHaveBeenCalledWith(
      expect.objectContaining({ maxZoom: 1, minZoom: 0.5 }),
    );
  });

  it("uses a readable landscape pack when the activity dock shortens the workspace", () => {
    renderTopology(null, "research-knowledge-active");
    flushFrames();
    resizeCanvas(973, 260);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    const bounds = flowProps().nodes.reduce(
      (current, node) => ({
        maximumX: Math.max(current.maximumX, node.position.x + (node.style?.width ?? 196)),
        maximumY: Math.max(current.maximumY, node.position.y + (node.style?.height ?? 120)),
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
    const fitZoom = Math.min(
      1,
      (973 - 16) / (bounds.maximumX - bounds.minimumX),
      (260 - 16) / (bounds.maximumY - bounds.minimumY),
    );
    expect(fitZoom).toBeGreaterThanOrEqual(0.54);
    expect(flowHarness.fitView).toHaveBeenCalledWith(
      expect.objectContaining({ maxZoom: 1, minZoom: 0.5, padding: "8px" }),
    );
    expectDomainLaneClearance();
  });

  it("stays fit-capable immediately above the former fixed dense-layout cutoff", () => {
    renderTopology(null, "research-knowledge-active");
    flushFrames();
    resizeCanvas(973, 281);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    const bounds = flowProps().nodes.reduce(
      (current, node) => ({
        maximumX: Math.max(current.maximumX, node.position.x + (node.style?.width ?? 196)),
        maximumY: Math.max(current.maximumY, node.position.y + (node.style?.height ?? 120)),
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
    const fitZoom = Math.min(
      1,
      (973 - 48) / (bounds.maximumX - bounds.minimumX),
      (281 - 48) / (bounds.maximumY - bounds.minimumY),
    );
    expect(fitZoom).toBeGreaterThanOrEqual(0.5);
    expectDomainLaneClearance();
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
    flushFrames();
    expect(flowHarness.fitView).toHaveBeenCalledTimes(1);
    resizeCanvas(840, 560);
    expect(flowHarness.fitView).toHaveBeenCalledTimes(2);

    fireEvent.click(screen.getByRole("button", { name: "Reset view" }));
    expect(flowHarness.fitView).toHaveBeenCalledTimes(3);
    expect(flowHarness.setViewport).not.toHaveBeenCalled();
  });

  it("keeps automatic framing when a node click opens an inspector and narrows the canvas", () => {
    renderTopology(null, "research-knowledge-active");
    flushFrames();
    resizeCanvas(973, 435);
    flowHarness.fitView.mockClear();

    flowProps().onMoveStart?.({ type: "pointerdown" });
    act(() => {
      flowProps().onNodeClick?.({ type: "click" }, { id: "demo-node:agent:coding" });
    });
    resizeCanvas(633, 435);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    expect(flowHarness.fitView).toHaveBeenCalledOnce();
  });

  it("pins world positions when manual interaction crosses a responsive layout threshold", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(1180, 620);
    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    const initialOrchestratorPosition = flowProps().nodes.find(
      (node) => node.id === "demo-node:orchestrator",
    )?.position;
    flowHarness.fitView.mockClear();

    fireEvent.click(screen.getByRole("button", { name: "Zoom in" }));
    resizeCanvas(5120, 1100);

    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "workspace");
    expect(
      flowProps().nodes.find((node) => node.id === "demo-node:orchestrator")?.position,
    ).toEqual(initialOrchestratorPosition);
    expect(flowHarness.fitView).not.toHaveBeenCalled();
    expect(flowHarness.setViewport).not.toHaveBeenCalled();

    fireEvent.click(screen.getByRole("button", { name: "Fit view" }));
    flushFrames();
    expect(screen.getByRole("listbox")).toHaveAttribute("data-layout-mode", "wide");
    expect(flowHarness.fitView).toHaveBeenCalledOnce();
  });

  it("reports viewport zoom without treating normal viewport updates as dataset changes", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(900, 560);
    flowHarness.fitView.mockClear();

    act(() => {
      flowProps().onMove?.(null, { x: 12, y: 18, zoom: 0.82 });
    });

    expect(screen.getByRole("status", { name: "Topology zoom level" })).toHaveTextContent("82%");
    expect(flowHarness.fitView).not.toHaveBeenCalled();
    resizeCanvas(920, 580);
    expect(flowHarness.fitView).toHaveBeenCalledOnce();
  });

  it("pins and reports the manual viewport after a wheel zoom transform", () => {
    renderTopology();
    flushFrames();
    resizeCanvas(900, 560);
    flowHarness.fitView.mockClear();

    act(() => {
      flowProps().onMove?.({ type: "wheel" }, { x: 24, y: 12, zoom: 1.12 });
    });
    expect(screen.getByRole("status", { name: "Topology zoom level" })).toHaveTextContent("112%");
    resizeCanvas(940, 580);

    expect(flowHarness.fitView).not.toHaveBeenCalled();
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

  it("centers selected context with bounded zoom while retaining canvas wheel ownership", () => {
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
    expect(flowProps()).toMatchObject({
      panOnScroll: false,
      preventScrolling: true,
      zoomOnScroll: true,
    });
  });

  it("uses directed non-color edge treatments and keeps unrelated context visible", () => {
    const projection = buildCommandCenterProjection("research-knowledge-active");
    const researchNode = projection.nodes.find((node) => node.agentId === "research");
    if (researchNode === undefined) throw new Error("Expected Research Agent node");
    renderTopology(researchNode.id, "research-knowledge-active");
    flushFrames();
    resizeCanvas(1040, 620);

    const semanticNodes = flowProps().nodes.filter((node) => !node.id.startsWith("demo-group:"));
    const selected = semanticNodes.find((node) => node.id === researchNode.id);
    const connected = semanticNodes.filter((node) => node.data?.isConnected);
    const dimmed = semanticNodes.filter((node) => node.data?.isDimmed);
    expect(selected?.data?.isSelected).toBe(true);
    expect(connected.length).toBeGreaterThan(0);
    expect(dimmed.length).toBeGreaterThan(0);

    const treatments = new Set(
      flowProps().edges.map(
        (edge) =>
          edge.className?.match(/command-center-edge--(hierarchy|dependency|review|outcome)/)?.[1],
      ),
    );
    expect(treatments).toEqual(new Set(["hierarchy", "dependency", "outcome"]));
    for (const edge of flowProps().edges) {
      expect(edge.markerEnd?.type).toBe("arrow-closed");
      expect(edge.ariaLabel).toContain(" to ");
    }

    fireEvent.click(screen.getByLabelText("Show relationship legend"));
    const legend = screen.getByText("Visible relationships").parentElement;
    expect(legend).toHaveTextContent("Delegates");
    expect(legend).toHaveTextContent("Depends On");
    expect(legend).toHaveTextContent("Result Flow");
  });

  it("offers every visible relationship through a keyboard-operable selection list", () => {
    const projection = buildCommandCenterProjection("research-knowledge-active");
    const onSelect = vi.fn();
    render(
      <OperationalTopologyAdapter onSelect={onSelect} projection={projection} selectedId={null} />,
    );
    flushFrames();
    resizeCanvas(1040, 620);

    fireEvent.click(screen.getByLabelText("Show relationship legend"));
    expect(screen.getByRole("list", { name: "Visible relationship selection" })).toBeVisible();
    for (const edge of projection.edges) {
      const source = projection.nodes.find((node) => node.id === edge.source);
      const target = projection.nodes.find((node) => node.id === edge.target);
      if (source === undefined || target === undefined) {
        throw new Error("Expected relationship endpoints");
      }
      const kindLabel = edge.kind
        .split("-")
        .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
        .join(" ");
      const button = screen.getByRole("button", {
        name: `Inspect relationship ${source.label}, ${kindLabel}, ${target.label}`,
      });
      button.focus();
      fireEvent.click(button);
      expect(onSelect).toHaveBeenLastCalledWith(edge.id);
      expect(button).toHaveFocus();
    }
    expect(onSelect).toHaveBeenCalledTimes(projection.edges.length);
  });

  it("returns pointer node and edge selection focus to the outer composite listbox", () => {
    const projection = buildCommandCenterProjection("research-knowledge-active");
    const onSelect = vi.fn();
    render(
      <OperationalTopologyAdapter onSelect={onSelect} projection={projection} selectedId={null} />,
    );
    flushFrames();
    resizeCanvas(1040, 620);
    const canvas = screen.getByRole("listbox", { name: /Simulated operational topology/ });
    const node = projection.nodes.find((candidate) => candidate.kind === "agent");
    const edge = projection.edges[0];
    if (node === undefined || edge === undefined) throw new Error("Expected graph selection data");

    act(() => {
      flowProps().onNodeClick?.({ type: "click" }, { id: node.id });
    });

    expect(canvas).toHaveFocus();
    expect(canvas).toHaveAttribute(
      "aria-activedescendant",
      `command-center-topology-${node.id.replaceAll(/[^a-zA-Z0-9_-]/g, "-")}`,
    );
    expect(onSelect).toHaveBeenLastCalledWith(node.id);

    act(() => {
      flowProps().onEdgeClick?.({ type: "click" }, { id: edge.id });
    });
    expect(canvas).toHaveFocus();
    expect(onSelect).toHaveBeenLastCalledWith(edge.id);
  });

  it("shows only the exact selected relationship label on the canvas", () => {
    const projection = buildCommandCenterProjection("research-knowledge-active");
    const selectedEdge = projection.edges.find((edge) => edge.kind === "depends-on");
    if (selectedEdge === undefined) throw new Error("Expected dependency edge");

    renderTopology(selectedEdge.id, "research-knowledge-active");
    flushFrames();
    resizeCanvas(1040, 620);

    const labeled = flowProps().edges.filter((edge) => edge.label !== undefined);
    expect(labeled).toHaveLength(1);
    expect(labeled[0]).toMatchObject({ label: "Depends On", labelShowBg: true });
  });

  it("uses expanded node height for long labels while preserving the full label", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const firstNode = projection.nodes[0];
    if (firstNode === undefined) throw new Error("Expected graph node");
    const longLabel = "A deliberately long canonical-style topology label";
    const longProjection = {
      ...projection,
      nodes: [{ ...firstNode, label: longLabel }, ...projection.nodes.slice(1)],
    };

    render(
      <OperationalTopologyAdapter
        onSelect={vi.fn()}
        projection={longProjection}
        selectedId={firstNode.id}
      />,
    );
    flushFrames();
    resizeCanvas(1040, 620);

    const node = flowProps().nodes.find((candidate) => candidate.id === firstNode.id);
    expect(node?.style).toEqual({ height: 120, width: 196, zIndex: 2 });
    expect(node?.data?.label).toBe(longLabel);
  });

  it("distinguishes filtered-empty, invalid, and stale-selection states", () => {
    const projection = buildCommandCenterProjection("catalog-idle");
    const reset = vi.fn();
    const clear = vi.fn();
    const { rerender } = render(
      <OperationalTopologyAdapter
        filtersActive
        onClearSelection={clear}
        onResetFilters={reset}
        onSelect={vi.fn()}
        projection={projection}
        selectedId={projection.nodes[0]?.id ?? null}
        visibleEdges={[]}
        visibleNodes={[]}
      />,
    );

    expect(screen.getByRole("heading", { name: "No graph results" })).toBeInTheDocument();
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
    expect(
      screen.getByRole("region", { name: /Simulated operational topology/ }),
    ).not.toHaveAttribute("aria-activedescendant");
    expect(screen.getByRole("toolbar", { name: "Topology viewport" })).toBeInTheDocument();
    expect(screen.getByRole("status", { name: "Topology zoom level" })).toHaveTextContent("N/A");
    expect(screen.getByRole("button", { name: "Zoom in" })).toBeDisabled();
    expect(screen.getByLabelText("Show relationship legend")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Reset filters" }));
    expect(reset).toHaveBeenCalledOnce();

    const visibleNode = projection.nodes[1];
    if (visibleNode === undefined) throw new Error("Expected visible graph node");
    rerender(
      <OperationalTopologyAdapter
        onClearSelection={clear}
        onSelect={vi.fn()}
        projection={projection}
        selectedId={projection.nodes[0]?.id ?? null}
        visibleEdges={[]}
        visibleNodes={[visibleNode]}
      />,
    );
    expect(screen.getByText(/Selected entity is no longer available/)).toBeInTheDocument();
    const staleClearButton = screen.getAllByRole("button", { name: "Clear selection" }).at(-1);
    if (staleClearButton === undefined) throw new Error("Expected stale-selection clear control");
    fireEvent.click(staleClearButton);
    expect(clear).toHaveBeenCalledOnce();

    rerender(
      <OperationalTopologyAdapter
        onSelect={vi.fn()}
        projection={{ ...projection, nodes: [visibleNode, visibleNode] }}
        selectedId={null}
      />,
    );
    expect(screen.getByRole("heading", { name: "Invalid graph data" })).toBeInTheDocument();
    expect(screen.getByRole("alert")).toHaveTextContent("Duplicate graph node identifier");
  });
});
