import {
  Handle,
  MarkerType,
  Position,
  ReactFlow,
  type CoordinateExtent,
  type Edge,
  type Node,
  type NodeProps,
  type ReactFlowInstance,
  type Rect,
  type Viewport,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import {
  Bot,
  Boxes,
  GitBranch,
  LocateFixed,
  Maximize2,
  Network,
  RotateCcw,
  ShieldCheck,
  ZoomIn,
  ZoomOut,
} from "lucide-react";
import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
} from "react";

import type {
  CommandCenterProjection,
  OperationalStatus,
  TopologyEdge,
  TopologyNode,
  TopologyNodeKind,
} from "../commandCenterProjection";

const GRAPH_HELP_ID = "command-center-topology-help";
const NODE_WIDTH = 180;
const NODE_HEIGHT = 88;
const DOMAIN_LANE_PADDING = 14;
const DOMAIN_LANE_VERTICAL_PADDING = 24;
const FIT_PADDING = "24px" as const;
const CENTER_PADDING = "72px" as const;
const MIN_ZOOM = 0.5;
const MAX_ZOOM = 1.5;
const AUTO_FIT_MAX_ZOOM = 1;
const CENTER_MAX_ZOOM = 1.1;
const WIDE_LAYOUT_MIN_CANVAS_WIDTH = 1280;
const COMPACT_OUTER_PADDING = 20;
const COMPACT_GROUP_MEMBER_GAP = 16;
const COMPACT_GROUP_COLUMN_GAP = 24;
const COMPACT_GROUP_SLOT_WIDTH = NODE_WIDTH * 2 + COMPACT_GROUP_MEMBER_GAP;
const COMPACT_LAYOUT_WIDTH =
  COMPACT_OUTER_PADDING * 2 + COMPACT_GROUP_SLOT_WIDTH * 2 + COMPACT_GROUP_COLUMN_GAP;
const COMPACT_GROUP_X = [
  COMPACT_OUTER_PADDING,
  COMPACT_OUTER_PADDING + COMPACT_GROUP_SLOT_WIDTH + COMPACT_GROUP_COLUMN_GAP,
] as const;
const COMPACT_GROUP_Y = [176, 304, 432] as const;
const COMPACT_AGENT_GAP = NODE_WIDTH + COMPACT_GROUP_MEMBER_GAP;
const COMPACT_WORK_Y = 590;
const COMPACT_WORK_ROW_GAP = NODE_HEIGHT + 24;
const COMPACT_WORK_NODE_GAP = 18;
const COMFORTABLE_VIEWPORT_GUTTER = 72;
const ACTIVE_STATUSES = new Set<OperationalStatus>([
  "running",
  "delegating",
  "waiting-child",
  "waiting-approval",
  "validating",
  "risk-review",
]);

interface OperationalNodeData extends Record<string, unknown> {
  readonly groupLabel: string;
  readonly id: string;
  readonly isCompositeActive: boolean;
  readonly isGroup: boolean;
  readonly isSelected: boolean;
  readonly kind: TopologyNodeKind | "domain-group";
  readonly label: string;
  readonly status: OperationalStatus;
}

type OperationalFlowNode = Node<OperationalNodeData, "operational">;
type OperationalFlowEdge = Edge<Record<string, never>, "smoothstep">;

interface CanvasSize {
  readonly height: number;
  readonly width: number;
}

type ViewportMode = "auto" | "manual";

export interface OperationalTopologyAdapterProps {
  readonly activeNodeId?: string | null | undefined;
  readonly onEscape?: (() => void) | undefined;
  readonly onSelect: (id: string) => void;
  readonly projection: CommandCenterProjection;
  readonly selectedId: string | null;
  readonly visibleEdges?: readonly TopologyEdge[] | undefined;
  readonly visibleNodes?: readonly TopologyNode[] | undefined;
}

function readable(value: string): string {
  return value.replaceAll("-", " ");
}

function nodeOptionId(id: string): string {
  return `command-center-topology-${id.replaceAll(/[^a-zA-Z0-9_-]/g, "-")}`;
}

function NodeKindIcon({ kind }: { readonly kind: TopologyNodeKind }) {
  if (kind === "orchestrator") {
    return <Network aria-hidden="true" />;
  }
  if (kind === "agent") {
    return <Bot aria-hidden="true" />;
  }
  if (kind === "workflow" || kind === "workflow-step") {
    return <GitBranch aria-hidden="true" />;
  }
  if (kind.endsWith("checkpoint")) {
    return <ShieldCheck aria-hidden="true" />;
  }
  return <Boxes aria-hidden="true" />;
}

function OperationalNode({ data }: NodeProps<OperationalFlowNode>) {
  if (data.isGroup) {
    return (
      <div aria-hidden="true" className="command-center-domain-lane">
        <strong>{data.label}</strong>
        <span>{data.groupLabel}</span>
      </div>
    );
  }

  const emphasized = data.isCompositeActive || data.isSelected;

  return (
    <>
      <Handle
        isConnectable={false}
        position={Position.Top}
        style={{ opacity: 0, pointerEvents: "none" }}
        type="target"
      />
      <div
        aria-label={`${data.label}, ${readable(data.kind)}, ${readable(data.status)}${
          data.groupLabel.length > 0 ? `, ${data.groupLabel} domain` : ""
        }`}
        aria-selected={data.isSelected}
        className={`command-center-node command-center-node--${data.kind}${
          emphasized ? " command-center-node--selected" : ""
        }`}
        id={nodeOptionId(data.id)}
        role="option"
      >
        <span className="command-center-node__topline">
          <NodeKindIcon kind={data.kind as TopologyNodeKind} />
          <strong className="command-center-node__title" title={data.label}>
            {data.label}
          </strong>
        </span>
        <span className="command-center-node__meta">
          <span className={`command-center-status command-center-status--${data.status}`}>
            {readable(data.status)}
          </span>
          <span>{data.groupLabel.length > 0 ? data.groupLabel : readable(data.kind)}</span>
        </span>
      </div>
      <Handle
        isConnectable={false}
        position={Position.Bottom}
        style={{ opacity: 0, pointerEvents: "none" }}
        type="source"
      />
    </>
  );
}

const NODE_TYPES = { operational: OperationalNode };

function visualNodeOrder(left: TopologyNode, right: TopologyNode): number {
  return (
    left.position.y - right.position.y ||
    left.position.x - right.position.x ||
    left.id.localeCompare(right.id)
  );
}

function buildCompactPositions(
  projection: CommandCenterProjection,
): ReadonlyMap<string, Readonly<{ x: number; y: number }>> {
  const positions = new Map<string, Readonly<{ x: number; y: number }>>();
  const orchestrator = projection.nodes.find((node) => node.kind === "orchestrator");
  if (orchestrator !== undefined) {
    positions.set(orchestrator.id, { x: (COMPACT_LAYOUT_WIDTH - NODE_WIDTH) / 2, y: 20 });
  }

  projection.groups.forEach((group, groupIndex) => {
    const members = projection.nodes.filter(
      (node) => node.kind === "agent" && node.groupId === group.id,
    );
    const isCenteredLastGroup =
      projection.groups.length % 2 === 1 && groupIndex === projection.groups.length - 1;
    const column = groupIndex % COMPACT_GROUP_X.length;
    const row = Math.floor(groupIndex / COMPACT_GROUP_X.length);
    const baseX = isCenteredLastGroup
      ? (COMPACT_LAYOUT_WIDTH -
          (members.length * NODE_WIDTH + Math.max(0, members.length - 1) * 16)) /
        2
      : (COMPACT_GROUP_X[column] ?? COMPACT_GROUP_X[0]);
    const y = COMPACT_GROUP_Y[row] ?? 342;
    members.forEach((member, memberIndex) => {
      positions.set(member.id, { x: baseX + memberIndex * COMPACT_AGENT_GAP, y });
    });
  });

  const workNodes = projection.nodes.filter(
    (node) => node.kind !== "agent" && node.kind !== "orchestrator",
  );
  const workRows = [...new Set(workNodes.map((node) => node.position.y))].sort(
    (left, right) => left - right,
  );
  workRows.forEach((sourceY, rowIndex) => {
    const rowNodes = workNodes
      .filter((node) => node.position.y === sourceY)
      .sort((left, right) => left.position.x - right.position.x || left.id.localeCompare(right.id));
    const rowWidth =
      rowNodes.length * NODE_WIDTH + Math.max(0, rowNodes.length - 1) * COMPACT_WORK_NODE_GAP;
    const startX = (COMPACT_LAYOUT_WIDTH - rowWidth) / 2;
    rowNodes.forEach((node, columnIndex) => {
      positions.set(node.id, {
        x: startX + columnIndex * (NODE_WIDTH + COMPACT_WORK_NODE_GAP),
        y: COMPACT_WORK_Y + rowIndex * COMPACT_WORK_ROW_GAP,
      });
    });
  });

  return positions;
}

function nodeDimension(node: OperationalFlowNode, key: "height" | "width"): number {
  const value = node.style?.[key];
  if (typeof value === "number") return value;
  return key === "width" ? NODE_WIDTH : NODE_HEIGHT;
}

function flowBounds(nodes: readonly OperationalFlowNode[]): Rect {
  const minimumX = Math.min(...nodes.map((node) => node.position.x));
  const minimumY = Math.min(...nodes.map((node) => node.position.y));
  const maximumX = Math.max(...nodes.map((node) => node.position.x + nodeDimension(node, "width")));
  const maximumY = Math.max(
    ...nodes.map((node) => node.position.y + nodeDimension(node, "height")),
  );
  return {
    height: Math.max(0, maximumY - minimumY),
    width: Math.max(0, maximumX - minimumX),
    x: minimumX,
    y: minimumY,
  };
}

function boundsAreComfortablyVisible(
  bounds: Rect,
  viewport: Viewport,
  canvasSize: CanvasSize,
): boolean {
  if (canvasSize.width <= 0 || canvasSize.height <= 0) return false;
  const left = bounds.x * viewport.zoom + viewport.x;
  const top = bounds.y * viewport.zoom + viewport.y;
  const right = (bounds.x + bounds.width) * viewport.zoom + viewport.x;
  const bottom = (bounds.y + bounds.height) * viewport.zoom + viewport.y;
  return (
    left >= COMFORTABLE_VIEWPORT_GUTTER &&
    top >= COMFORTABLE_VIEWPORT_GUTTER &&
    right <= canvasSize.width - COMFORTABLE_VIEWPORT_GUTTER &&
    bottom <= canvasSize.height - COMFORTABLE_VIEWPORT_GUTTER
  );
}

export function OperationalTopologyAdapter({
  activeNodeId,
  onEscape,
  onSelect,
  projection,
  selectedId,
  visibleEdges,
  visibleNodes,
}: OperationalTopologyAdapterProps) {
  const nodes = visibleNodes ?? projection.nodes;
  const requestedEdges = visibleEdges ?? projection.edges;
  const toolbarFirstControlRef = useRef<HTMLButtonElement>(null);
  const canvasRef = useRef<HTMLDivElement>(null);
  const resizeFrameRef = useRef<number | null>(null);
  const fitFrameRef = useRef<number | null>(null);
  const pendingCanvasSizeRef = useRef<CanvasSize | null>(null);
  const viewportModeRef = useRef<ViewportMode>("auto");
  const [canvasSize, setCanvasSize] = useState<CanvasSize>({ height: 0, width: 0 });
  const [instance, setInstance] = useState<ReactFlowInstance<
    OperationalFlowNode,
    OperationalFlowEdge
  > | null>(null);
  const compactPositions = useMemo(() => buildCompactPositions(projection), [projection]);
  const useWideLayout = canvasSize.width >= WIDE_LAYOUT_MIN_CANVAS_WIDTH;
  const layoutNodes = useMemo<readonly TopologyNode[]>(
    () =>
      nodes.map((node) => ({
        ...node,
        position: useWideLayout ? node.position : (compactPositions.get(node.id) ?? node.position),
      })),
    [compactPositions, nodes, useWideLayout],
  );
  const keyboardNodes = useMemo(() => [...layoutNodes].sort(visualNodeOrder), [layoutNodes]);
  const [compositeCursor, setCompositeCursor] = useState<{
    readonly basedOnSelectionId: string | null;
    readonly id: string | null;
  }>({ basedOnSelectionId: null, id: null });
  const selectedVisibleNode =
    selectedId === null ? undefined : layoutNodes.find((node) => node.id === selectedId);
  const cursorIsCurrent =
    compositeCursor.basedOnSelectionId === selectedId &&
    compositeCursor.id !== null &&
    keyboardNodes.some((node) => node.id === compositeCursor.id);
  const compositeActiveId = cursorIsCurrent
    ? compositeCursor.id
    : (selectedVisibleNode?.id ?? keyboardNodes[0]?.id ?? null);

  const groupLabels = useMemo(
    () => new Map(projection.groups.map((group) => [group.id, group.label])),
    [projection.groups],
  );
  const groupFlowNodes = useMemo<OperationalFlowNode[]>(
    () =>
      projection.groups.flatMap((group) => {
        const members = layoutNodes.filter(
          (node) => node.kind === "agent" && node.groupId === group.id,
        );
        if (members.length === 0) return [];
        const minimumX = Math.min(...members.map((member) => member.position.x));
        const maximumX = Math.max(...members.map((member) => member.position.x));
        const minimumY = Math.min(...members.map((member) => member.position.y));
        const maximumY = Math.max(...members.map((member) => member.position.y));
        return [
          {
            connectable: false,
            data: {
              groupLabel: group.description,
              id: `demo-group:${group.id}`,
              isCompositeActive: false,
              isGroup: true,
              isSelected: false,
              kind: "domain-group",
              label: group.label,
              status: "idle",
            },
            deletable: false,
            draggable: false,
            focusable: false,
            id: `demo-group:${group.id}`,
            position: {
              x: minimumX - DOMAIN_LANE_PADDING,
              y: minimumY - DOMAIN_LANE_VERTICAL_PADDING,
            },
            selectable: false,
            style: {
              height: maximumY - minimumY + NODE_HEIGHT + DOMAIN_LANE_VERTICAL_PADDING * 2,
              pointerEvents: "none",
              width: maximumX - minimumX + NODE_WIDTH + DOMAIN_LANE_PADDING * 2,
              zIndex: -1,
            },
            type: "operational" as const,
          },
        ];
      }),
    [layoutNodes, projection.groups],
  );
  const nodeIds = useMemo(() => new Set<string>(layoutNodes.map((node) => node.id)), [layoutNodes]);
  const edges = useMemo(
    () => requestedEdges.filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target)),
    [nodeIds, requestedEdges],
  );
  const flowNodes = useMemo<OperationalFlowNode[]>(
    () => [
      ...groupFlowNodes,
      ...layoutNodes.map((node) => ({
        ariaLabel: `${node.label}, ${readable(node.kind)}, ${readable(node.status)}`,
        connectable: false,
        data: {
          groupLabel: node.groupId === null ? "" : (groupLabels.get(node.groupId) ?? ""),
          id: node.id,
          isCompositeActive: compositeActiveId === node.id,
          isGroup: false,
          isSelected: selectedId === node.id,
          kind: node.kind,
          label: node.label,
          status: node.status,
        },
        deletable: false,
        draggable: false,
        focusable: false,
        id: node.id,
        position: node.position,
        selectable: false,
        style: { height: NODE_HEIGHT, width: NODE_WIDTH },
        type: "operational" as const,
      })),
    ],
    [compositeActiveId, groupFlowNodes, groupLabels, layoutNodes, selectedId],
  );
  const flowEdges = useMemo<OperationalFlowEdge[]>(
    () =>
      edges.map((edge) => ({
        ariaLabel: `${edge.label}: ${edge.source} to ${edge.target}`,
        data: {},
        deletable: false,
        focusable: false,
        id: edge.id,
        label: edge.label,
        labelBgBorderRadius: 4,
        labelBgPadding: [4, 2] as [number, number],
        labelBgStyle: { fill: "var(--command-surface)" },
        labelShowBg: true,
        labelStyle: {
          fill: "var(--command-text-subtle)",
          fontSize: 9,
          fontWeight: 700,
        },
        markerEnd: {
          color: "var(--command-teal)",
          height: 14,
          type: MarkerType.ArrowClosed,
          width: 14,
        },
        reconnectable: false,
        selectable: false,
        selected: selectedId === edge.id,
        source: edge.source,
        target: edge.target,
        type: "smoothstep",
      })),
    [edges, selectedId],
  );
  const layoutKey = useMemo(
    () =>
      flowNodes
        .map((node) => [node.id, node.position.x.toString(), node.position.y.toString()].join(":"))
        .join("|"),
    [flowNodes],
  );
  const fitGenerationKey = `${projection.scenarioId}:${layoutKey}`;
  const fitGenerationRef = useRef(fitGenerationKey);
  const fitNodeIdsKey = flowNodes.map((node) => node.id).join("|");
  const fitNodeReferences = useMemo(
    () =>
      fitNodeIdsKey
        .split("|")
        .filter(Boolean)
        .map((id) => ({ id })),
    [fitNodeIdsKey],
  );
  const topologyBounds = flowBounds(flowNodes);
  const translateExtent = useMemo<CoordinateExtent>(() => {
    const horizontalGutter = Math.max(
      NODE_WIDTH * 2,
      canvasSize.width > 0 ? canvasSize.width / MIN_ZOOM / 2 : 0,
    );
    const verticalGutter = Math.max(
      NODE_HEIGHT * 2,
      canvasSize.height > 0 ? canvasSize.height / MIN_ZOOM / 2 : 0,
    );
    return [
      [topologyBounds.x - horizontalGutter, topologyBounds.y - verticalGutter],
      [
        topologyBounds.x + topologyBounds.width + horizontalGutter,
        topologyBounds.y + topologyBounds.height + verticalGutter,
      ],
    ];
  }, [canvasSize.height, canvasSize.width, topologyBounds]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) return;

    const scheduleSize = (width: number, height: number) => {
      pendingCanvasSizeRef.current = {
        height: Math.max(0, Math.round(height)),
        width: Math.max(0, Math.round(width)),
      };
      if (resizeFrameRef.current !== null) return;
      resizeFrameRef.current = window.requestAnimationFrame(() => {
        resizeFrameRef.current = null;
        const nextSize = pendingCanvasSizeRef.current;
        pendingCanvasSizeRef.current = null;
        if (nextSize === null) return;
        setCanvasSize((current) =>
          current.width === nextSize.width && current.height === nextSize.height
            ? current
            : nextSize,
        );
      });
    };

    const initialBounds = canvas.getBoundingClientRect();
    scheduleSize(initialBounds.width, initialBounds.height);
    const observer = new ResizeObserver((entries) => {
      const entry = entries.find((candidate) => candidate.target === canvas);
      if (entry !== undefined) scheduleSize(entry.contentRect.width, entry.contentRect.height);
    });
    observer.observe(canvas);

    return () => {
      observer.disconnect();
      if (resizeFrameRef.current !== null) {
        window.cancelAnimationFrame(resizeFrameRef.current);
        resizeFrameRef.current = null;
      }
      pendingCanvasSizeRef.current = null;
    };
  }, []);

  const performCanonicalFit = useCallback(() => {
    viewportModeRef.current = "auto";
    if (
      instance === null ||
      !instance.viewportInitialized ||
      fitNodeReferences.length === 0 ||
      canvasSize.width <= 0 ||
      canvasSize.height <= 0
    ) {
      return;
    }
    void instance.fitView({
      duration: 0,
      maxZoom: AUTO_FIT_MAX_ZOOM,
      minZoom: MIN_ZOOM,
      nodes: fitNodeReferences,
      padding: FIT_PADDING,
    });
  }, [canvasSize.height, canvasSize.width, fitNodeReferences, instance]);

  useEffect(() => {
    const generationChanged = fitGenerationRef.current !== fitGenerationKey;
    fitGenerationRef.current = fitGenerationKey;
    if (generationChanged) viewportModeRef.current = "auto";
    if (viewportModeRef.current !== "auto") return;
    if (fitFrameRef.current !== null) window.cancelAnimationFrame(fitFrameRef.current);
    fitFrameRef.current = window.requestAnimationFrame(() => {
      fitFrameRef.current = null;
      performCanonicalFit();
    });

    return () => {
      if (fitFrameRef.current !== null) {
        window.cancelAnimationFrame(fitFrameRef.current);
        fitFrameRef.current = null;
      }
    };
  }, [canvasSize.height, canvasSize.width, fitGenerationKey, performCanonicalFit]);

  const selectedNodeId = selectedId !== null && nodeIds.has(selectedId) ? selectedId : null;
  const selectedEdge =
    selectedId === null ? undefined : edges.find((edge) => edge.id === selectedId);
  const derivedActiveNodeId = nodes.find((node) => ACTIVE_STATUSES.has(node.status))?.id ?? null;
  const resolvedActiveNodeId =
    activeNodeId === undefined
      ? derivedActiveNodeId
      : activeNodeId !== null && nodeIds.has(activeNodeId)
        ? activeNodeId
        : null;

  const contextualNodeIds = useCallback(
    (id: string): readonly string[] => {
      const related = new Set<string>([id]);
      for (const edge of edges) {
        if (edge.source === id) related.add(edge.target);
        if (edge.target === id) related.add(edge.source);
      }
      return [...related].filter((candidate) => nodeIds.has(candidate));
    },
    [edges, nodeIds],
  );
  const selectedGroup =
    selectedId === null ? undefined : projection.groups.find((group) => group.id === selectedId);
  const selectedCenterIds =
    selectedNodeId !== null
      ? contextualNodeIds(selectedNodeId)
      : selectedEdge !== undefined
        ? [selectedEdge.source, selectedEdge.target]
        : selectedGroup === undefined
          ? []
          : [
              `demo-group:${selectedGroup.id}`,
              ...layoutNodes
                .filter((node) => node.groupId === selectedGroup.id)
                .map((node) => node.id),
            ];

  const centerIds = useCallback(
    (ids: readonly string[]) => {
      if (instance === null || ids.length === 0) {
        return;
      }
      const centeredIds = [...new Set(ids)].filter(
        (id) => nodeIds.has(id) || id.startsWith("demo-group:"),
      );
      if (centeredIds.length === 0) return;
      const bounds = instance.getNodesBounds(centeredIds);
      if (boundsAreComfortablyVisible(bounds, instance.getViewport(), canvasSize)) return;
      viewportModeRef.current = "manual";
      void instance.fitView({
        duration: 0,
        maxZoom: CENTER_MAX_ZOOM,
        minZoom: MIN_ZOOM,
        nodes: centeredIds.map((id) => ({ id })),
        padding: CENTER_PADDING,
      });
    },
    [canvasSize, instance, nodeIds],
  );

  const handleCompositeKeyDown = useCallback(
    (event: ReactKeyboardEvent<HTMLDivElement>) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onEscape?.();
        toolbarFirstControlRef.current?.focus();
        return;
      }
      if (keyboardNodes.length === 0) {
        return;
      }

      const currentIndex = Math.max(
        0,
        keyboardNodes.findIndex((node) => node.id === compositeActiveId),
      );
      let nextIndex: number | null = null;
      if (event.key === "ArrowRight" || event.key === "ArrowDown") {
        nextIndex = Math.min(currentIndex + 1, keyboardNodes.length - 1);
      } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
        nextIndex = Math.max(currentIndex - 1, 0);
      } else if (event.key === "Home") {
        nextIndex = 0;
      } else if (event.key === "End") {
        nextIndex = keyboardNodes.length - 1;
      } else if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        const activeNode = keyboardNodes[currentIndex];
        if (activeNode !== undefined) {
          onSelect(activeNode.id);
          centerIds([activeNode.id]);
        }
        return;
      }

      if (nextIndex !== null) {
        event.preventDefault();
        const nextNode = keyboardNodes[nextIndex];
        if (nextNode !== undefined) {
          setCompositeCursor({
            basedOnSelectionId: selectedId,
            id: nextNode.id,
          });
          centerIds([nextNode.id]);
        }
      }
    },
    [centerIds, compositeActiveId, keyboardNodes, onEscape, onSelect, selectedId],
  );

  const controlsDisabled = instance === null || nodes.length === 0;

  return (
    <div className="command-center-topology">
      <div className="command-center-toolbar" role="toolbar" aria-label="Topology viewport">
        <button
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            viewportModeRef.current = "manual";
            void instance?.zoomIn({ duration: 0 });
          }}
          ref={toolbarFirstControlRef}
          type="button"
        >
          <ZoomIn aria-hidden="true" /> Zoom in
        </button>
        <button
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            viewportModeRef.current = "manual";
            void instance?.zoomOut({ duration: 0 });
          }}
          type="button"
        >
          <ZoomOut aria-hidden="true" /> Zoom out
        </button>
        <button
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            performCanonicalFit();
          }}
          type="button"
        >
          <Maximize2 aria-hidden="true" /> Fit view
        </button>
        <button
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            performCanonicalFit();
          }}
          type="button"
        >
          <RotateCcw aria-hidden="true" /> Reset view
        </button>
        <button
          className="command-center-button"
          disabled={selectedCenterIds.length === 0 || instance === null}
          onClick={() => {
            centerIds(selectedCenterIds);
          }}
          type="button"
        >
          <LocateFixed aria-hidden="true" /> Center selected
        </button>
        <button
          className="command-center-button"
          disabled={resolvedActiveNodeId === null || instance === null}
          onClick={() => {
            if (resolvedActiveNodeId !== null) {
              centerIds(contextualNodeIds(resolvedActiveNodeId));
            }
          }}
          type="button"
        >
          <LocateFixed aria-hidden="true" /> Center active
        </button>
        <p className="command-center-toolbar__help" id={GRAPH_HELP_ID}>
          Tab enters the topology once. Use arrow keys, Home, or End to review nodes; Enter or Space
          selects. Escape returns to these controls. Drag pans; ordinary wheel gestures scroll the
          page.
        </p>
      </div>

      <div
        aria-activedescendant={
          compositeActiveId === null ? undefined : nodeOptionId(compositeActiveId)
        }
        aria-describedby={GRAPH_HELP_ID}
        aria-label={`Simulated operational topology for ${projection.scenarioLabel}`}
        className="command-center-canvas"
        data-layout-mode={useWideLayout ? "wide" : "compact"}
        data-scroll-region="command-center-topology"
        onFocus={() => {
          if (compositeActiveId === null) {
            setCompositeCursor({
              basedOnSelectionId: selectedId,
              id: keyboardNodes[0]?.id ?? null,
            });
          }
        }}
        onKeyDown={handleCompositeKeyDown}
        role="listbox"
        ref={canvasRef}
        tabIndex={0}
      >
        <ReactFlow<OperationalFlowNode, OperationalFlowEdge>
          autoPanOnNodeFocus={false}
          deleteKeyCode={null}
          edges={flowEdges}
          edgesFocusable={false}
          edgesReconnectable={false}
          elementsSelectable={false}
          fitView={false}
          maxZoom={MAX_ZOOM}
          minZoom={MIN_ZOOM}
          multiSelectionKeyCode={null}
          nodeTypes={NODE_TYPES}
          nodes={flowNodes}
          nodesConnectable={false}
          nodesDraggable={false}
          nodesFocusable={false}
          onEdgeClick={(_event, edge) => {
            onSelect(edge.id);
          }}
          onInit={setInstance}
          onNodeClick={(_event, node) => {
            setCompositeCursor({
              basedOnSelectionId: selectedId,
              id: node.id,
            });
            onSelect(node.id);
          }}
          onMoveStart={(event) => {
            if (event !== null) viewportModeRef.current = "manual";
          }}
          onlyRenderVisibleElements={false}
          panOnDrag
          panOnScroll={false}
          preventScrolling={false}
          proOptions={{ hideAttribution: false }}
          selectionKeyCode={null}
          tabIndex={-1}
          translateExtent={translateExtent}
          zoomOnDoubleClick={false}
          zoomOnScroll={false}
        />
      </div>
    </div>
  );
}
