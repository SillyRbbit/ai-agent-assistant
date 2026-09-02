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
  AlertTriangle,
  Bot,
  Boxes,
  CircleOff,
  GitBranch,
  ListTree,
  LocateFixed,
  Maximize2,
  Network,
  RotateCcw,
  ShieldCheck,
  SearchX,
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
  TopologyEdgeKind,
  TopologyNode,
  TopologyNodeKind,
} from "../commandCenterProjection";

const GRAPH_HELP_ID = "command-center-topology-help";
const NODE_WIDTH = 196;
const NODE_MIN_HEIGHT = 104;
const NODE_MAX_HEIGHT = 120;
const DOMAIN_LANE_PADDING = 14;
const DOMAIN_LANE_VERTICAL_PADDING = 24;
const DOMAIN_LANE_BOTTOM_PADDING = 16;
const DENSE_DOMAIN_LANE_PADDING = 8;
const FIT_GUTTER = 24;
const DENSE_FIT_GUTTER = 8;
const FIT_PADDING = "24px" as const;
const CENTER_PADDING = "72px" as const;
const MIN_ZOOM = 0.5;
const MAX_ZOOM = 1.5;
const AUTO_FIT_MAX_ZOOM = 1;
const CENTER_MAX_ZOOM = 1.1;
const WORKSPACE_LAYOUT_MIN_CANVAS_WIDTH = 600;
const WIDE_LAYOUT_MIN_READABLE_ZOOM = MAX_ZOOM;
const COMPACT_OUTER_PADDING = 20;
const COMPACT_GROUP_MEMBER_GAP = 16;
const COMPACT_GROUP_COLUMN_GAP = 36;
const COMPACT_GROUP_SLOT_WIDTH = NODE_WIDTH * 2 + COMPACT_GROUP_MEMBER_GAP;
const COMPACT_AGENT_GAP = NODE_WIDTH + COMPACT_GROUP_MEMBER_GAP;
const COMPACT_GROUP_START_Y = 180;
const COMPACT_GROUP_ROW_GAP = NODE_MAX_HEIGHT + DOMAIN_LANE_VERTICAL_PADDING * 2 + 14;
const COMPACT_WORK_ROW_GAP = NODE_MAX_HEIGHT + 26;
const COMPACT_WORK_NODE_GAP = 18;
const LANDSCAPE_COLUMN_GAP = 12;
const LANDSCAPE_GROUP_GAP = DENSE_DOMAIN_LANE_PADDING * 2 + 16;
const LANDSCAPE_FIRST_ROW_Y = 160;
const LANDSCAPE_SECOND_ROW_Y = 320;
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
  readonly isConnected: boolean;
  readonly isDenseLayout: boolean;
  readonly isDimmed: boolean;
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
  readonly filtersActive?: boolean | undefined;
  readonly onClearSelection?: (() => void) | undefined;
  readonly onEscape?: (() => void) | undefined;
  readonly onResetFilters?: (() => void) | undefined;
  readonly onSelect: (id: string) => void;
  readonly projection: CommandCenterProjection;
  readonly selectedId: string | null;
  readonly selectionPresent?: boolean | undefined;
  readonly visibleEdges?: readonly TopologyEdge[] | undefined;
  readonly visibleNodes?: readonly TopologyNode[] | undefined;
}

type LayoutMode = "compact" | "workspace" | "wide";

type EdgeTreatment = "dependency" | "hierarchy" | "outcome" | "review";

const EDGE_TREATMENT: Readonly<Record<TopologyEdgeKind, EdgeTreatment>> = {
  "approval-dependency": "dependency",
  cancellation: "outcome",
  delegates: "hierarchy",
  "depends-on": "dependency",
  "failure-propagation": "outcome",
  "owns-task": "hierarchy",
  orchestrates: "hierarchy",
  "result-flow": "outcome",
  "risk-review": "review",
  "validation-review": "review",
};

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
      <div
        aria-hidden="true"
        className={`command-center-domain-lane${
          data.isSelected ? " command-center-domain-lane--selected" : ""
        }${data.isConnected ? " command-center-domain-lane--connected" : ""}${
          data.isDimmed ? " command-center-domain-lane--dimmed" : ""
        }${data.isDenseLayout ? " command-center-domain-lane--dense" : ""}`}
        title={`${data.label}: ${data.groupLabel}`}
      >
        <strong>{data.label}</strong>
        <span>{data.groupLabel}</span>
      </div>
    );
  }

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
          data.isSelected ? " command-center-node--selected" : ""
        }${data.isConnected ? " command-center-node--connected" : ""}${
          data.isDimmed ? " command-center-node--dimmed" : ""
        }${data.isCompositeActive ? " command-center-node--active" : ""}`}
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
          <span title={data.groupLabel.length > 0 ? data.groupLabel : readable(data.kind)}>
            {data.groupLabel.length > 0 ? data.groupLabel : readable(data.kind)}
          </span>
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

function nodeHeight(label: string): number {
  return label.length > 22 ? NODE_MAX_HEIGHT : NODE_MIN_HEIGHT;
}

function visualNodeOrder(left: TopologyNode, right: TopologyNode): number {
  return (
    left.position.y - right.position.y ||
    left.position.x - right.position.x ||
    left.id.localeCompare(right.id)
  );
}

function buildPackedPositions(
  projection: CommandCenterProjection,
  groupColumnCount: 2 | 3,
): ReadonlyMap<string, Readonly<{ x: number; y: number }>> {
  const positions = new Map<string, Readonly<{ x: number; y: number }>>();
  const layoutWidth =
    COMPACT_OUTER_PADDING * 2 +
    COMPACT_GROUP_SLOT_WIDTH * groupColumnCount +
    COMPACT_GROUP_COLUMN_GAP * (groupColumnCount - 1);
  const orchestrator = projection.nodes.find((node) => node.kind === "orchestrator");
  if (orchestrator !== undefined) {
    positions.set(orchestrator.id, { x: (layoutWidth - NODE_WIDTH) / 2, y: 20 });
  }

  projection.groups.forEach((group, groupIndex, groups) => {
    const members = projection.nodes.filter(
      (node) => node.kind === "agent" && node.groupId === group.id,
    );
    const row = Math.floor(groupIndex / groupColumnCount);
    const column = groupIndex % groupColumnCount;
    const rowStart = row * groupColumnCount;
    const groupsInRow = Math.min(groupColumnCount, groups.length - rowStart);
    const occupiedWidth =
      groupsInRow * COMPACT_GROUP_SLOT_WIDTH +
      Math.max(0, groupsInRow - 1) * COMPACT_GROUP_COLUMN_GAP;
    const rowOffset = (layoutWidth - occupiedWidth) / 2;
    const baseX = rowOffset + column * (COMPACT_GROUP_SLOT_WIDTH + COMPACT_GROUP_COLUMN_GAP);
    const y = COMPACT_GROUP_START_Y + row * COMPACT_GROUP_ROW_GAP;
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
  const workColumnCount = groupColumnCount === 3 ? 4 : 2;
  const groupRowCount = Math.ceil(projection.groups.length / groupColumnCount);
  const workStartY = COMPACT_GROUP_START_Y + groupRowCount * COMPACT_GROUP_ROW_GAP + 18;
  const orderedWorkNodes = workRows.flatMap((sourceY) =>
    workNodes
      .filter((node) => node.position.y === sourceY)
      .sort((left, right) => left.position.x - right.position.x || left.id.localeCompare(right.id)),
  );
  for (let startIndex = 0; startIndex < orderedWorkNodes.length; startIndex += workColumnCount) {
    const rowNodes = orderedWorkNodes.slice(startIndex, startIndex + workColumnCount);
    const rowIndex = Math.floor(startIndex / workColumnCount);
    const rowWidth =
      rowNodes.length * NODE_WIDTH + Math.max(0, rowNodes.length - 1) * COMPACT_WORK_NODE_GAP;
    const startX = (layoutWidth - rowWidth) / 2;
    rowNodes.forEach((node, columnIndex) => {
      positions.set(node.id, {
        x: startX + columnIndex * (NODE_WIDTH + COMPACT_WORK_NODE_GAP),
        y: workStartY + rowIndex * COMPACT_WORK_ROW_GAP,
      });
    });
  }

  return positions;
}

function landscapeGap(left: TopologyNode, right: TopologyNode): number {
  return left.kind === "agent" && right.kind === "agent" && left.groupId !== right.groupId
    ? LANDSCAPE_GROUP_GAP
    : LANDSCAPE_COLUMN_GAP;
}

function landscapeRowWidth(nodes: readonly TopologyNode[]): number {
  let width = 0;
  let previous: TopologyNode | undefined;
  for (const node of nodes) {
    width += NODE_WIDTH;
    if (previous !== undefined) width += landscapeGap(previous, node);
    previous = node;
  }
  return width;
}

function buildWorkspacePositions(
  projection: CommandCenterProjection,
  dense: boolean,
): ReadonlyMap<string, Readonly<{ x: number; y: number }>> {
  if (dense) {
    const positions = new Map<string, Readonly<{ x: number; y: number }>>();
    const groupedAgents = projection.groups.map((group) =>
      projection.nodes.filter((node) => node.kind === "agent" && node.groupId === group.id),
    );
    const firstRowAgents = groupedAgents.slice(0, 3).flat();
    const secondRowAgents = groupedAgents.slice(3).flat();
    const workNodes = projection.nodes
      .filter((node) => node.kind !== "agent" && node.kind !== "orchestrator")
      .sort(
        (left, right) =>
          left.position.y - right.position.y ||
          left.position.x - right.position.x ||
          left.id.localeCompare(right.id),
      );
    const rowCapacity = Math.max(
      firstRowAgents.length,
      Math.ceil((firstRowAgents.length + secondRowAgents.length + workNodes.length) / 2),
    );
    const firstRowWorkCount = Math.min(
      workNodes.length,
      Math.max(0, rowCapacity - firstRowAgents.length),
    );
    const rows = [
      [...firstRowAgents, ...workNodes.slice(0, firstRowWorkCount)],
      [...secondRowAgents, ...workNodes.slice(firstRowWorkCount)],
    ] as const;
    const layoutWidth =
      COMPACT_OUTER_PADDING * 2 +
      Math.max(...rows.map((row) => landscapeRowWidth(row)), NODE_WIDTH);
    const orchestrator = projection.nodes.find((node) => node.kind === "orchestrator");
    if (orchestrator !== undefined) {
      positions.set(orchestrator.id, { x: (layoutWidth - NODE_WIDTH) / 2, y: 0 });
    }
    rows.forEach((row, rowIndex) => {
      const rowWidth = landscapeRowWidth(row);
      const startX = (layoutWidth - rowWidth) / 2;
      let x = startX;
      row.forEach((node, columnIndex) => {
        positions.set(node.id, {
          x,
          y: rowIndex === 0 ? LANDSCAPE_FIRST_ROW_Y : LANDSCAPE_SECOND_ROW_Y,
        });
        const nextNode = row[columnIndex + 1];
        if (nextNode !== undefined) x += NODE_WIDTH + landscapeGap(node, nextNode);
      });
    });
    return positions;
  }

  const positions = new Map<string, Readonly<{ x: number; y: number }>>();
  const groupWidth = projection.groups.length * NODE_WIDTH;
  const groupGaps = Math.max(0, projection.groups.length - 1) * COMPACT_GROUP_COLUMN_GAP;
  const layoutWidth = COMPACT_OUTER_PADDING * 2 + groupWidth + groupGaps;
  const orchestrator = projection.nodes.find((node) => node.kind === "orchestrator");
  if (orchestrator !== undefined) {
    positions.set(orchestrator.id, { x: (layoutWidth - NODE_WIDTH) / 2, y: 10 });
  }

  let maximumGroupMembers = 1;
  projection.groups.forEach((group, groupIndex) => {
    const members = projection.nodes.filter(
      (node) => node.kind === "agent" && node.groupId === group.id,
    );
    maximumGroupMembers = Math.max(maximumGroupMembers, members.length);
    const x = COMPACT_OUTER_PADDING + groupIndex * (NODE_WIDTH + COMPACT_GROUP_COLUMN_GAP);
    members.forEach((member, memberIndex) => {
      positions.set(member.id, {
        x,
        y: COMPACT_GROUP_START_Y + memberIndex * (NODE_MAX_HEIGHT + 12),
      });
    });
  });

  const workNodes = projection.nodes
    .filter((node) => node.kind !== "agent" && node.kind !== "orchestrator")
    .sort(
      (left, right) =>
        left.position.y - right.position.y ||
        left.position.x - right.position.x ||
        left.id.localeCompare(right.id),
    );
  const workColumnCount = 5;
  const workStartY = COMPACT_GROUP_START_Y + maximumGroupMembers * (NODE_MAX_HEIGHT + 12) + 16;
  for (let startIndex = 0; startIndex < workNodes.length; startIndex += workColumnCount) {
    const rowNodes = workNodes.slice(startIndex, startIndex + workColumnCount);
    const rowIndex = Math.floor(startIndex / workColumnCount);
    const rowWidth =
      rowNodes.length * NODE_WIDTH + Math.max(0, rowNodes.length - 1) * COMPACT_WORK_NODE_GAP;
    const startX = (layoutWidth - rowWidth) / 2;
    rowNodes.forEach((node, columnIndex) => {
      positions.set(node.id, {
        x: startX + columnIndex * (NODE_WIDTH + COMPACT_WORK_NODE_GAP),
        y: workStartY + rowIndex * COMPACT_WORK_ROW_GAP,
      });
    });
  }

  return positions;
}

function nodeDimension(node: OperationalFlowNode, key: "height" | "width"): number {
  const value = node.style?.[key];
  if (typeof value === "number") return value;
  return key === "width" ? NODE_WIDTH : NODE_MIN_HEIGHT;
}

function flowBounds(nodes: readonly OperationalFlowNode[]): Rect {
  if (nodes.length === 0) return { height: 0, width: 0, x: 0, y: 0 };
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

function positionedTopologyBounds(
  nodes: readonly TopologyNode[],
  positions: ReadonlyMap<string, Readonly<{ x: number; y: number }>>,
): Rect {
  if (nodes.length === 0) return { height: 0, width: 0, x: 0, y: 0 };
  const positionedNodes = nodes.map((node) => ({
    height: nodeHeight(node.label),
    position: positions.get(node.id) ?? node.position,
  }));
  const minimumX = Math.min(...positionedNodes.map((node) => node.position.x));
  const minimumY = Math.min(...positionedNodes.map((node) => node.position.y));
  const maximumX = Math.max(...positionedNodes.map((node) => node.position.x + NODE_WIDTH));
  const maximumY = Math.max(...positionedNodes.map((node) => node.position.y + node.height));
  return {
    height: maximumY - minimumY + DOMAIN_LANE_VERTICAL_PADDING * 2,
    width: maximumX - minimumX + DOMAIN_LANE_PADDING * 2,
    x: minimumX - DOMAIN_LANE_PADDING,
    y: minimumY - DOMAIN_LANE_VERTICAL_PADDING,
  };
}

function topologyFitZoom(bounds: Rect, canvasSize: CanvasSize, gutter: number): number {
  if (
    bounds.width <= 0 ||
    bounds.height <= 0 ||
    canvasSize.width <= gutter * 2 ||
    canvasSize.height <= gutter * 2
  ) {
    return 0;
  }
  return Math.min(
    MAX_ZOOM,
    (canvasSize.width - gutter * 2) / bounds.width,
    (canvasSize.height - gutter * 2) / bounds.height,
  );
}

function layoutFitsAtReadableZoom(bounds: Rect, canvasSize: CanvasSize): boolean {
  return topologyFitZoom(bounds, canvasSize, FIT_GUTTER) >= WIDE_LAYOUT_MIN_READABLE_ZOOM;
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

function graphDataIssue(
  nodes: readonly TopologyNode[],
  edges: readonly TopologyEdge[],
): string | null {
  const nodeIds = new Set<string>();
  for (const node of nodes) {
    if (nodeIds.has(node.id)) return `Duplicate graph node identifier: ${node.id}`;
    nodeIds.add(node.id);
  }
  const edgeIds = new Set<string>();
  for (const edge of edges) {
    if (edgeIds.has(edge.id)) return `Duplicate graph relationship identifier: ${edge.id}`;
    edgeIds.add(edge.id);
    if (!nodeIds.has(edge.source) || !nodeIds.has(edge.target)) {
      return `Graph relationship ${edge.id} references an unavailable endpoint.`;
    }
  }
  return null;
}

function relationshipLabel(kind: TopologyEdgeKind): string {
  return kind
    .split("-")
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join(" ");
}

interface GraphStateContent {
  readonly action?: (() => void) | undefined;
  readonly detail: string;
  readonly invalid?: boolean;
  readonly title: string;
}

function GraphState({ action, detail, invalid = false, title }: GraphStateContent) {
  const Icon = invalid ? AlertTriangle : SearchX;
  return (
    <div className="command-center-graph-state" role={invalid ? "alert" : "status"}>
      <Icon aria-hidden="true" />
      <div>
        <h3>{title}</h3>
        <p>{detail}</p>
      </div>
      {action === undefined ? null : (
        <button className="command-center-button" onClick={action} type="button">
          Reset filters
        </button>
      )}
    </div>
  );
}

interface OperationalTopologyCanvasProps extends OperationalTopologyAdapterProps {
  readonly graphState: GraphStateContent | null;
  readonly nodes: readonly TopologyNode[];
  readonly requestedEdges: readonly TopologyEdge[];
}

export function OperationalTopologyAdapter(props: OperationalTopologyAdapterProps) {
  const nodes = props.visibleNodes ?? props.projection.nodes;
  const requestedEdges = props.visibleEdges ?? props.projection.edges;
  const invalidGraphMessage = graphDataIssue(nodes, requestedEdges);
  const graphState: GraphStateContent | null =
    invalidGraphMessage !== null
      ? {
          detail: `${invalidGraphMessage} The deterministic projection was not rendered.`,
          invalid: true,
          title: "Invalid graph data",
        }
      : nodes.length === 0
        ? props.filtersActive
          ? {
              action: props.onResetFilters,
              detail: "The current local search or filters hide every bounded fixture entity.",
              title: "No graph results",
            }
          : {
              detail: "This deterministic projection contains no topology entities to display.",
              title: "No graph data",
            }
        : null;

  return (
    <OperationalTopologyCanvas
      {...props}
      graphState={graphState}
      nodes={invalidGraphMessage === null ? nodes : []}
      requestedEdges={invalidGraphMessage === null ? requestedEdges : []}
    />
  );
}

function OperationalTopologyCanvas({
  activeNodeId,
  graphState,
  nodes,
  onClearSelection,
  onEscape,
  onSelect,
  projection,
  requestedEdges,
  selectedId,
  selectionPresent,
}: OperationalTopologyCanvasProps) {
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
  const datasetKey = useMemo(
    () =>
      `${projection.scenarioId}:${nodes.map((node) => node.id).join("|")}:${requestedEdges
        .map((edge) => edge.id)
        .join("|")}`,
    [nodes, projection.scenarioId, requestedEdges],
  );
  const [manualLayout, setManualLayout] = useState<Readonly<{
    datasetKey: string;
    dense: boolean;
    mode: LayoutMode;
  }> | null>(null);
  const activeManualLayout = manualLayout?.datasetKey === datasetKey ? manualLayout : null;
  const [zoomLevel, setZoomLevel] = useState(1);
  const balancedPositions = useMemo(() => buildWorkspacePositions(projection, false), [projection]);
  const balancedTopologyBounds = useMemo(
    () => positionedTopologyBounds(nodes, balancedPositions),
    [balancedPositions, nodes],
  );
  const responsiveLayoutMode: LayoutMode = layoutFitsAtReadableZoom(
    balancedTopologyBounds,
    canvasSize,
  )
    ? "wide"
    : canvasSize.width >= WORKSPACE_LAYOUT_MIN_CANVAS_WIDTH
      ? "workspace"
      : "compact";
  const layoutMode = activeManualLayout?.mode ?? responsiveLayoutMode;
  const denseWorkspacePositions = useMemo(
    () => buildWorkspacePositions(projection, true),
    [projection],
  );
  const denseWorkspaceBounds = useMemo(
    () => positionedTopologyBounds(nodes, denseWorkspacePositions),
    [denseWorkspacePositions, nodes],
  );
  const standardWorkspaceFitZoom = topologyFitZoom(balancedTopologyBounds, canvasSize, FIT_GUTTER);
  const denseWorkspaceFitZoom = topologyFitZoom(denseWorkspaceBounds, canvasSize, DENSE_FIT_GUTTER);
  const shouldUseDenseWorkspace =
    standardWorkspaceFitZoom < MIN_ZOOM && denseWorkspaceFitZoom > standardWorkspaceFitZoom;
  const denseWorkspace =
    layoutMode === "workspace" && (activeManualLayout?.dense ?? shouldUseDenseWorkspace);
  const compactPositions = useMemo(() => buildPackedPositions(projection, 2), [projection]);
  const workspacePositions = denseWorkspace ? denseWorkspacePositions : balancedPositions;
  const activePositions =
    layoutMode === "wide"
      ? balancedPositions
      : layoutMode === "workspace"
        ? workspacePositions
        : compactPositions;
  const layoutNodes = useMemo<readonly TopologyNode[]>(
    () =>
      nodes.map((node) => ({
        ...node,
        position: activePositions.get(node.id) ?? node.position,
      })),
    [activePositions, nodes],
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
  const nodeIds = useMemo(() => new Set<string>(layoutNodes.map((node) => node.id)), [layoutNodes]);
  const edges = useMemo(
    () => requestedEdges.filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target)),
    [nodeIds, requestedEdges],
  );
  const selectedNode =
    selectedId === null ? undefined : layoutNodes.find((node) => node.id === selectedId);
  const selectedVisibleEdge =
    selectedId === null ? undefined : edges.find((edge) => edge.id === selectedId);
  const selectedGroup =
    selectedId === null ? undefined : projection.groups.find((group) => group.id === selectedId);
  const selectedGroupMembers = useMemo(
    () =>
      selectedGroup === undefined
        ? []
        : layoutNodes.filter((node) => node.groupId === selectedGroup.id).map((node) => node.id),
    [layoutNodes, selectedGroup],
  );
  const selectionIsVisible =
    selectedNode !== undefined ||
    selectedVisibleEdge !== undefined ||
    selectedGroupMembers.length > 0;
  const contextualNodeIdSet = useMemo(() => {
    const related = new Set<string>();
    if (selectedNode !== undefined) related.add(selectedNode.id);
    if (selectedVisibleEdge !== undefined) {
      related.add(selectedVisibleEdge.source);
      related.add(selectedVisibleEdge.target);
    }
    for (const memberId of selectedGroupMembers) related.add(memberId);
    for (const edge of edges) {
      if (edge.source === selectedNode?.id) related.add(edge.target);
      if (edge.target === selectedNode?.id) related.add(edge.source);
    }
    return related;
  }, [edges, selectedGroupMembers, selectedNode, selectedVisibleEdge]);
  const contextualEdgeIdSet = useMemo(() => {
    const related = new Set<string>();
    for (const edge of edges) {
      if (selectedVisibleEdge?.id === edge.id) related.add(edge.id);
      if (
        selectedNode !== undefined &&
        (edge.source === selectedNode.id || edge.target === selectedNode.id)
      ) {
        related.add(edge.id);
      }
      if (
        selectedGroupMembers.length > 0 &&
        (contextualNodeIdSet.has(edge.source) || contextualNodeIdSet.has(edge.target))
      ) {
        related.add(edge.id);
      }
    }
    return related;
  }, [contextualNodeIdSet, edges, selectedGroupMembers.length, selectedNode, selectedVisibleEdge]);
  const domainLaneHorizontalPadding = denseWorkspace
    ? DENSE_DOMAIN_LANE_PADDING
    : DOMAIN_LANE_PADDING;
  const domainLaneBottomPadding = denseWorkspace ? 8 : DOMAIN_LANE_BOTTOM_PADDING;
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
        const maximumHeight = Math.max(...members.map((member) => nodeHeight(member.label)));
        const groupIsSelected = selectedGroup?.id === group.id;
        const groupIsConnected =
          !groupIsSelected && members.some((member) => contextualNodeIdSet.has(member.id));
        return [
          {
            connectable: false,
            data: {
              groupLabel: group.description,
              id: `demo-group:${group.id}`,
              isCompositeActive: false,
              isConnected: groupIsConnected,
              isDenseLayout: denseWorkspace,
              isDimmed: selectionIsVisible && !groupIsSelected && !groupIsConnected,
              isGroup: true,
              isSelected: groupIsSelected,
              kind: "domain-group",
              label: group.label,
              status: "idle",
            },
            deletable: false,
            draggable: false,
            focusable: false,
            id: `demo-group:${group.id}`,
            position: {
              x: minimumX - domainLaneHorizontalPadding,
              y: minimumY - DOMAIN_LANE_VERTICAL_PADDING,
            },
            selectable: false,
            style: {
              height:
                maximumY -
                minimumY +
                maximumHeight +
                DOMAIN_LANE_VERTICAL_PADDING +
                domainLaneBottomPadding,
              pointerEvents: "none",
              width: maximumX - minimumX + NODE_WIDTH + domainLaneHorizontalPadding * 2,
              zIndex: 1,
            },
            type: "operational" as const,
          },
        ];
      }),
    [
      contextualNodeIdSet,
      denseWorkspace,
      domainLaneBottomPadding,
      domainLaneHorizontalPadding,
      layoutNodes,
      projection.groups,
      selectedGroup,
      selectionIsVisible,
    ],
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
          isConnected: contextualNodeIdSet.has(node.id) && selectedId !== node.id,
          isDenseLayout: denseWorkspace,
          isDimmed: selectionIsVisible && !contextualNodeIdSet.has(node.id),
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
        style: { height: nodeHeight(node.label), width: NODE_WIDTH, zIndex: 2 },
        type: "operational" as const,
      })),
    ],
    [
      compositeActiveId,
      contextualNodeIdSet,
      denseWorkspace,
      groupFlowNodes,
      groupLabels,
      layoutNodes,
      selectedId,
      selectionIsVisible,
    ],
  );
  const presentRelationshipKinds = useMemo(
    () => [...new Set(edges.map((edge) => edge.kind))].sort(),
    [edges],
  );
  const flowEdges = useMemo<OperationalFlowEdge[]>(
    () =>
      edges.map((edge) => {
        const selected = selectedId === edge.id;
        const connected = contextualEdgeIdSet.has(edge.id);
        const treatment = EDGE_TREATMENT[edge.kind];
        return {
          ariaLabel: `${edge.label}: ${edge.source} to ${edge.target}`,
          className: `command-center-edge command-center-edge--${treatment}${
            connected ? " command-center-edge--connected" : ""
          }${selectionIsVisible && !connected ? " command-center-edge--dimmed" : ""}`,
          data: {},
          deletable: false,
          focusable: false,
          id: edge.id,
          label: selected ? relationshipLabel(edge.kind) : undefined,
          labelBgBorderRadius: 4,
          labelBgPadding: [5, 3] as [number, number],
          labelBgStyle: { fill: "var(--command-surface)" },
          labelShowBg: selected,
          labelStyle: {
            fill: "var(--command-text-subtle)",
            fontSize: 12,
            fontWeight: 700,
          },
          markerEnd: {
            color: "var(--command-teal)",
            height: 15,
            type: MarkerType.ArrowClosed,
            width: 15,
          },
          reconnectable: false,
          selectable: false,
          selected,
          source: edge.source,
          target: edge.target,
          type: "smoothstep" as const,
        };
      }),
    [contextualEdgeIdSet, edges, selectedId, selectionIsVisible],
  );
  const datasetGenerationRef = useRef(datasetKey);
  const fitNodeIdsKey = flowNodes.map((node) => node.id).join("|");
  const fitNodeReferences = useMemo(
    () =>
      fitNodeIdsKey
        .split("|")
        .filter(Boolean)
        .map((id) => ({ id })),
    [fitNodeIdsKey],
  );
  const topologyBounds = useMemo(() => flowBounds(flowNodes), [flowNodes]);
  const canonicalFitPadding = denseWorkspace ? ("8px" as const) : FIT_PADDING;
  const canonicalFitMaxZoom = layoutMode === "wide" ? MAX_ZOOM : AUTO_FIT_MAX_ZOOM;
  const translateExtent = useMemo<CoordinateExtent>(() => {
    const horizontalGutter = Math.max(
      NODE_WIDTH * 2,
      canvasSize.width > 0 ? canvasSize.width / MIN_ZOOM / 2 : 0,
    );
    const verticalGutter = Math.max(
      NODE_MAX_HEIGHT * 2,
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

  const enterManualMode = useCallback(() => {
    viewportModeRef.current = "manual";
    setManualLayout((current) =>
      current?.datasetKey === datasetKey
        ? current
        : { datasetKey, dense: denseWorkspace, mode: layoutMode },
    );
  }, [datasetKey, denseWorkspace, layoutMode]);

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
      maxZoom: canonicalFitMaxZoom,
      minZoom: MIN_ZOOM,
      nodes: fitNodeReferences,
      padding: canonicalFitPadding,
    });
  }, [
    canonicalFitMaxZoom,
    canonicalFitPadding,
    canvasSize.height,
    canvasSize.width,
    fitNodeReferences,
    instance,
  ]);

  useEffect(() => {
    const generationChanged = datasetGenerationRef.current !== datasetKey;
    datasetGenerationRef.current = datasetKey;
    if (generationChanged) {
      viewportModeRef.current = "auto";
    }
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
  }, [activeManualLayout, canvasSize.height, canvasSize.width, datasetKey, performCanonicalFit]);

  const resumeAutomaticFit = useCallback(() => {
    viewportModeRef.current = "auto";
    if (activeManualLayout === null) {
      performCanonicalFit();
      return;
    }
    setManualLayout(null);
  }, [activeManualLayout, performCanonicalFit]);

  const selectedNodeId = selectedId !== null && nodeIds.has(selectedId) ? selectedId : null;
  const derivedActiveNodeId = nodes.find((node) => ACTIVE_STATUSES.has(node.status))?.id ?? null;
  const resolvedActiveNodeId =
    activeNodeId === undefined
      ? derivedActiveNodeId
      : activeNodeId !== null && nodeIds.has(activeNodeId)
        ? activeNodeId
        : null;

  const contextualIdsFor = useCallback(
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
  const selectedCenterIds =
    selectedNodeId !== null
      ? contextualIdsFor(selectedNodeId)
      : selectedVisibleEdge !== undefined
        ? [selectedVisibleEdge.source, selectedVisibleEdge.target]
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
      enterManualMode();
      void instance.fitView({
        duration: 0,
        maxZoom: CENTER_MAX_ZOOM,
        minZoom: MIN_ZOOM,
        nodes: centeredIds.map((id) => ({ id })),
        padding: CENTER_PADDING,
      });
    },
    [canvasSize, enterManualMode, instance, nodeIds],
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
  const hasSelection = selectionPresent ?? selectedId !== null;
  const handleFlowInit = useCallback(
    (nextInstance: ReactFlowInstance<OperationalFlowNode, OperationalFlowEdge>) => {
      setInstance(nextInstance);
      setZoomLevel(nextInstance.getViewport().zoom);
    },
    [],
  );

  return (
    <div className="command-center-topology">
      <div
        aria-activedescendant={
          graphState !== null || compositeActiveId === null
            ? undefined
            : nodeOptionId(compositeActiveId)
        }
        aria-describedby={graphState === null ? GRAPH_HELP_ID : undefined}
        aria-label={`Simulated operational topology for ${projection.scenarioLabel}`}
        className="command-center-canvas"
        data-layout-mode={layoutMode}
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
        role={graphState === null ? "listbox" : "region"}
        ref={canvasRef}
        tabIndex={graphState === null ? 0 : undefined}
      >
        {graphState === null ? (
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
              canvasRef.current?.focus({ preventScroll: true });
              onSelect(edge.id);
            }}
            onInit={handleFlowInit}
            onMove={(event, viewport) => {
              if (event !== null) enterManualMode();
              setZoomLevel((current) =>
                Math.abs(current - viewport.zoom) < 0.005 ? current : viewport.zoom,
              );
            }}
            onNodeClick={(_event, node) => {
              setCompositeCursor({
                basedOnSelectionId: selectedId,
                id: node.id,
              });
              canvasRef.current?.focus({ preventScroll: true });
              onSelect(node.id);
            }}
            onNodeDoubleClick={(_event, node) => {
              centerIds(contextualIdsFor(node.id));
            }}
            onlyRenderVisibleElements={false}
            panOnDrag
            panOnScroll={false}
            preventScrolling
            proOptions={{ hideAttribution: false }}
            selectionKeyCode={null}
            tabIndex={-1}
            translateExtent={translateExtent}
            zoomOnDoubleClick={false}
            zoomOnScroll
          />
        ) : (
          <GraphState {...graphState} />
        )}
      </div>

      <div
        className="command-center-toolbar command-center-graph-toolbar"
        role="toolbar"
        aria-label="Topology viewport"
      >
        <output aria-label="Topology zoom level" className="command-center-graph-toolbar__zoom">
          {graphState === null ? `${String(Math.round(zoomLevel * 100))}%` : "N/A"}
        </output>
        <button
          aria-label="Zoom in"
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            enterManualMode();
            void instance?.zoomIn({ duration: 0 });
          }}
          ref={toolbarFirstControlRef}
          title="Zoom in"
          type="button"
        >
          <ZoomIn aria-hidden="true" />
          <span className="command-center-graph-toolbar__label">Zoom in</span>
        </button>
        <button
          aria-label="Zoom out"
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            enterManualMode();
            void instance?.zoomOut({ duration: 0 });
          }}
          title="Zoom out"
          type="button"
        >
          <ZoomOut aria-hidden="true" />
          <span className="command-center-graph-toolbar__label">Zoom out</span>
        </button>
        <button
          aria-label="Fit view"
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            resumeAutomaticFit();
          }}
          title="Fit view"
          type="button"
        >
          <Maximize2 aria-hidden="true" />
          <span className="command-center-graph-toolbar__label">Fit</span>
        </button>
        <button
          aria-label="Reset view"
          className="command-center-button"
          disabled={controlsDisabled}
          onClick={() => {
            resumeAutomaticFit();
          }}
          title="Reset view"
          type="button"
        >
          <RotateCcw aria-hidden="true" />
          <span className="command-center-graph-toolbar__label">Reset</span>
        </button>
        <button
          aria-label="Center selected"
          className="command-center-button"
          disabled={selectedCenterIds.length === 0 || instance === null}
          onClick={() => {
            centerIds(selectedCenterIds);
          }}
          title="Center selected"
          type="button"
        >
          <LocateFixed aria-hidden="true" />
          <span className="command-center-graph-toolbar__label">Center selected</span>
        </button>
        <button
          aria-label="Center active"
          className="command-center-button"
          disabled={resolvedActiveNodeId === null || instance === null}
          onClick={() => {
            if (resolvedActiveNodeId !== null) {
              centerIds(contextualIdsFor(resolvedActiveNodeId));
            }
          }}
          title="Center active"
          type="button"
        >
          <LocateFixed aria-hidden="true" />
          <span className="command-center-graph-toolbar__label">Center active</span>
        </button>
        <button
          aria-label="Clear selection"
          className="command-center-button"
          disabled={!hasSelection || onClearSelection === undefined}
          onClick={onClearSelection}
          title="Clear selection"
          type="button"
        >
          <CircleOff aria-hidden="true" />
          <span className="command-center-graph-toolbar__label">Clear</span>
        </button>
        <details className="command-center-graph-legend">
          <summary
            aria-label="Show relationship legend"
            className="command-center-button"
            title="Show relationship legend"
          >
            <ListTree aria-hidden="true" />
            <span>Legend</span>
          </summary>
          <div className="command-center-graph-legend__popover">
            <div className="command-center-graph-legend__key">
              <strong>Visible relationships</strong>
              {presentRelationshipKinds.length === 0 ? (
                <p>No relationships are visible for the current graph filters.</p>
              ) : (
                <>
                  <ul>
                    {presentRelationshipKinds.map((kind) => (
                      <li data-treatment={EDGE_TREATMENT[kind]} key={kind}>
                        <span aria-hidden="true" className="command-center-graph-legend__line" />
                        <span>{relationshipLabel(kind)}</span>
                      </li>
                    ))}
                  </ul>
                  <p>Arrowheads show direction; line patterns distinguish relationship families.</p>
                </>
              )}
            </div>
            <div className="command-center-graph-relationship-options">
              <strong>Select a relationship</strong>
              {edges.length === 0 ? (
                <p>No relationship is available for selection.</p>
              ) : (
                <ul aria-label="Visible relationship selection">
                  {edges.map((edge) => {
                    const sourceLabel =
                      layoutNodes.find((node) => node.id === edge.source)?.label ?? edge.source;
                    const targetLabel =
                      layoutNodes.find((node) => node.id === edge.target)?.label ?? edge.target;
                    const kindLabel = relationshipLabel(edge.kind);
                    return (
                      <li key={edge.id}>
                        <button
                          aria-label={`Inspect relationship ${sourceLabel}, ${kindLabel}, ${targetLabel}`}
                          aria-pressed={selectedId === edge.id}
                          onClick={() => {
                            onSelect(edge.id);
                          }}
                          type="button"
                        >
                          <strong>{kindLabel}</strong>
                          <span>
                            {sourceLabel} → {targetLabel}
                          </span>
                        </button>
                      </li>
                    );
                  })}
                </ul>
              )}
            </div>
          </div>
        </details>
        <p className="command-center-graph-toolbar__help" id={GRAPH_HELP_ID}>
          Tab enters the topology once. Use arrow keys, Home, or End to review nodes; Enter or Space
          selects. Escape returns to these controls. Drag pans. Over the topology, scroll up to zoom
          in and scroll down to zoom out; scroll outside it to move the page.
        </p>
      </div>

      {graphState !== null || (canvasSize.width > 0 && canvasSize.height > 0) ? null : (
        <p aria-live="polite" className="command-center-graph-measuring" role="status">
          Measuring graph workspace…
        </p>
      )}

      {graphState === null && selectedId !== null && !selectionIsVisible ? (
        <div className="command-center-graph-stale-selection" role="status">
          <span>Selected entity is no longer available in the visible graph.</span>
          <button
            className="command-center-button"
            disabled={onClearSelection === undefined}
            onClick={onClearSelection}
            type="button"
          >
            Clear selection
          </button>
        </div>
      ) : null}
    </div>
  );
}
