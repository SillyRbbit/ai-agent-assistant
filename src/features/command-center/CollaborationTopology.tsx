import { centeredRowX } from "./layoutRows";
import { matchesCard, type OperationalFilters } from "./collaborationProjection";
import { useCallback, useEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import {
  ReactFlow,
  BaseEdge,
  type EdgeProps,
  Handle,
  Position,
  MarkerType,
  useNodesInitialized,
  type Node,
  type NodeProps,
  type ReactFlowInstance,
  type Viewport,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { BotAvatar, CoordinatorAvatar } from "../agents/BotAppearance";
import type { OperationalCard, OperationalProjection } from "./collaborationProjection";
type CardNode = Node<{ card: OperationalCard }, "card">;

function Card({ data }: NodeProps<CardNode>) {
  const c = data.card;
  return (
    <div className={`operational-card operational-card--${c.kind}`} data-status={c.status}>
      <Handle id="input" type="target" position={Position.Left} style={{ top: "40%" }} />
      <Handle id="result-in" type="target" position={Position.Left} style={{ top: "75%" }} />
      <Handle id="coord-in" type="target" position={Position.Top} />
      {c.kind === "coordinator" ? (
        <CoordinatorAvatar />
      ) : c.identity ? (
        <BotAvatar identity={c.identity} agentId={c.agentId} />
      ) : (
        <span className="operational-unknown-avatar" aria-hidden="true">
          ◇
        </span>
      )}
      <strong title={c.label}>{c.label}</strong>
      <span title={c.role}>{c.role}</span>
      <small>
        {c.status} ·{" "}
        {c.kind === "stage"
          ? "Stage execution"
          : c.kind === "bot"
            ? c.attribution
            : "AgentOrchestrator"}
      </small>
      <Handle id="output" type="source" position={Position.Right} style={{ top: "40%" }} />
      <Handle id="result-out" type="source" position={Position.Right} style={{ top: "75%" }} />
      <Handle id="coord-out" type="source" position={Position.Bottom} />
    </div>
  );
}
const nodeTypes = { card: Card };
function CoordinationEdge({ id, sourceX, sourceY, targetX, targetY, style, markerEnd }: EdgeProps) {
  // A shared outside lane keeps coordinator links out of roster cards.
  const laneX = 1620;
  return (
    <BaseEdge
      id={id}
      path={`M ${String(sourceX)} ${String(sourceY)} H ${String(laneX)} V ${String(targetY - 40)} H ${String(targetX)} V ${String(targetY)}`}
      style={style}
      {...(markerEnd ? { markerEnd } : {})}
    />
  );
}
const edgeTypes = { coordination: CoordinationEdge };

function Measurements({ onReady }: { readonly onReady: (ready: boolean) => void }) {
  const ready = useNodesInitialized({ includeHiddenNodes: true });
  useEffect(() => {
    onReady(ready);
  }, [ready, onReady]);
  return null;
}
export function CollaborationTopology({
  projection,
  dataset,
  selected,
  onSelect,
  filters,
  viewports,
}: {
  readonly projection: OperationalProjection;
  readonly dataset: string;
  readonly selected: string | null;
  readonly onSelect: (id: string | null) => void;
  readonly filters: OperationalFilters;
  readonly viewports: Map<string, Viewport>;
}) {
  const [instance, setInstance] = useState<ReactFlowInstance<CardNode> | null>(null),
    [ready, setReady] = useState(false),
    [size, setSize] = useState({ width: 0, height: 0 }),
    [zoom, setZoom] = useState(1);
  const ref = useRef<HTMLDivElement>(null),
    manual = useRef(viewports.has(dataset));
  const nodes = useMemo(
    () =>
      projection.cards.map(
        (card, i): CardNode => ({
          id: card.id,
          type: "card",
          width: 240,
          height: 130,
          position:
            card.kind === "coordinator"
              ? { x: 680, y: 0 }
              : card.kind === "bot"
                ? {
                    x: centeredRowX(i <= 5 ? 5 : 4, (i - 1) % 5, 240, 70, 1600),
                    y: 180 + Math.floor((i - 1) / 5) * 160,
                  }
                : { x: centeredRowX(projection.cards.length - 10, i - 10, 240, 100, 1600), y: 590 },
          data: { card },
          selected: card.id === selected,
        }),
      ),
    [projection.cards, selected],
  );
  const visible = nodes.filter(({ data: { card } }) => matchesCard(card, filters));
  const ids = new Set(visible.map((n) => n.id));
  const edges = projection.links
    .filter((e) => ids.has(e.source) && ids.has(e.target))
    .map((e) => ({
      ...e,
      type: e.kind === "coordination" ? "coordination" : "smoothstep",
      sourceHandle:
        e.kind === "coordination" ? "coord-out" : e.kind === "handoff" ? "result-out" : "output",
      targetHandle:
        e.kind === "coordination" ? "coord-in" : e.kind === "handoff" ? "result-in" : "input",
      label: e.kind === "coordination" ? "" : e.kind,
      markerEnd: { type: MarkerType.ArrowClosed },
      style: {
        stroke: e.kind === "handoff" ? "#40d8c2" : e.kind === "dependency" ? "#ffae55" : "#8798c9",
        strokeDasharray:
          e.kind === "dependency" ? "6 5" : e.kind === "coordination" ? "2 6" : undefined,
        strokeWidth: e.kind === "handoff" ? 3 : 1.5,
      },
      labelStyle: { fill: "#e3e8f4" },
      labelBgStyle: { fill: "#101722" },
    }));
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const observer = new ResizeObserver(([entry]) => {
      if (entry) setSize({ width: entry.contentRect.width, height: entry.contentRect.height });
    });
    observer.observe(el);
    return () => {
      observer.disconnect();
    };
  }, []);
  const fit = useCallback(() => {
    if (instance && ready && size.width && size.height)
      void instance.fitView({ padding: 0.18, minZoom: 0.25, maxZoom: 1, duration: 0 });
  }, [instance, ready, size]);
  useEffect(() => {
    manual.current = viewports.has(dataset);
    if (instance) {
      const saved = viewports.get(dataset);
      if (saved) void instance.setViewport(saved);
    }
  }, [dataset, instance, viewports]);
  useEffect(() => {
    if (!manual.current) fit();
  }, [fit, dataset]);
  function key(e: KeyboardEvent) {
    if (!["ArrowDown", "ArrowUp", "Home", "End", "Enter", "Escape"].includes(e.key)) return;
    e.preventDefault();
    const i = visible.findIndex((n) => n.id === selected);
    const n =
      e.key === "Home"
        ? visible[0]
        : e.key === "End"
          ? visible.at(-1)
          : e.key === "ArrowUp"
            ? visible[Math.max(0, i - 1)]
            : visible[Math.min(visible.length - 1, i + 1)];
    if (e.key === "Escape") onSelect(null);
    else if (e.key === "Enter" && selected && instance) {
      manual.current = true;
      void instance.fitView({ nodes: [{ id: selected }], maxZoom: 1, padding: 0.3 });
    } else if (n) onSelect(n.id);
  }
  const canvasReady = ready && size.width > 0 && size.height > 0 && instance !== null;
  return (
    <>
      {!canvasReady && visible.length > 0 && (
        <p role="status">Waiting for graph canvas and node measurements.</p>
      )}
      <div
        className="operational-canvas"
        ref={ref}
        tabIndex={0}
        onKeyDown={key}
        aria-label="Operational graph; arrows select, Enter focuses, Escape clears"
        role="region"
      >
        {visible.length === 0 ? (
          <p>No matching nodes. Clear filters to restore the graph.</p>
        ) : (
          <ReactFlow<CardNode>
            nodes={visible}
            edges={edges}
            nodeTypes={nodeTypes}
            edgeTypes={edgeTypes}
            onInit={setInstance}
            onNodeClick={(_, n) => {
              onSelect(n.id);
            }}
            nodesDraggable={false}
            nodesConnectable={false}
            deleteKeyCode={null}
            ariaLabelConfig={{
              "edge.a11yDescription.default":
                "Read-only information flow. Edges cannot be edited or deleted.",
              "node.a11yDescription.default":
                "Read-only card. Use graph arrows to select, Enter to focus and Escape to clear.",
              "node.a11yDescription.keyboardDisabled": "Read-only card. Select to inspect details.",
            }}
            minZoom={0.25}
            maxZoom={2}
            onMove={(e, v) => {
              setZoom(v.zoom);
              if (e) {
                manual.current = true;
                viewports.set(dataset, v);
              }
            }}
            onMoveEnd={(_, v) => {
              if (manual.current) viewports.set(dataset, v);
            }}
            fitView={false}
          >
            <Measurements onReady={setReady} />
          </ReactFlow>
        )}
      </div>
      <div className="operational-toolbar" role="toolbar" aria-label="Operational graph viewport">
        <output>{Math.round(zoom * 100)}%</output>
        <button
          disabled={!canvasReady}
          onClick={() => {
            manual.current = true;
            void instance?.zoomIn();
          }}
        >
          Zoom in
        </button>
        <button
          disabled={!canvasReady}
          onClick={() => {
            manual.current = true;
            void instance?.zoomOut();
          }}
        >
          Zoom out
        </button>
        <button
          disabled={!canvasReady}
          onClick={() => {
            manual.current = true;
            fit();
          }}
        >
          Fit visible
        </button>
        <button
          disabled={!canvasReady}
          onClick={() => {
            manual.current = false;
            viewports.delete(dataset);
            onSelect(null);
            fit();
          }}
        >
          Reset viewport
        </button>
        <span>
          Fit frames visible nodes. Reset clears selection and resumes automatic fitting. Drag to
          pan; wheel to zoom.
        </span>
      </div>
      <p>
        Legend: dotted coordination · dashed planned dependency · solid teal validated result
        handoff. Cortexa controls information flow; bots do not spawn one another.
      </p>
    </>
  );
}
