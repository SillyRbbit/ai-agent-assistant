import { useEffect, useMemo } from "react";
import {
  useNodesInitialized,
  useNodesState,
  useReactFlow,
  Background,
  Controls,
  ReactFlow,
  ReactFlowProvider,
  type Node,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import type { KnowledgeItem } from "../../infrastructure/tauri/knowledge-client";
import { neighborhood } from "./knowledgeNeighborhood";
function KnowledgeViewport({ signature }: { readonly signature: string }) {
  const ready = useNodesInitialized({ includeHiddenNodes: false });
  const { fitView, setViewport } = useReactFlow();
  useEffect(() => {
    if (ready) void fitView({ padding: 0.2 });
  }, [ready, signature, fitView]);
  return (
    <div className="knowledge-graph-actions">
      <button
        type="button"
        disabled={!ready}
        onClick={() => {
          void fitView({ padding: 0.2 });
        }}
      >
        Fit linked notes
      </button>
      <button
        type="button"
        disabled={!ready}
        onClick={() => {
          void setViewport({ x: 0, y: 0, zoom: 1 });
        }}
      >
        Reset knowledge viewport
      </button>
    </div>
  );
}
export function KnowledgeGraph({
  items,
  selected,
  onOpen,
}: {
  readonly items: readonly KnowledgeItem[];
  readonly selected: number;
  readonly onOpen: (id: number) => void;
}) {
  const { nodes: layoutNodes, edges } = useMemo(() => {
    const result = neighborhood(items, selected);
    const nodes: Node[] = [...result.ids].map((id, index) => ({
      id: String(id),
      position: { x: (index % 3) * 235, y: Math.floor(index / 3) * 110 },
      data: { label: items.find((i) => i.id === id)?.versions.at(-1)?.title ?? "Removed note" },
      selected: id === selected,
      style: { width: 210, minHeight: 65, overflowWrap: "anywhere" },
    }));
    return { nodes, edges: result.edges };
  }, [items, selected]);
  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
  useEffect(() => {
    setNodes(layoutNodes);
  }, [layoutNodes, setNodes]);
  return (
    <section aria-label="Local knowledge graph">
      <h3>Linked neighborhood</h3>
      <p>
        Current saved library · at most 25 notes · explicit links only. Historical Markdown is
        separate.
      </p>
      <ReactFlowProvider>
        <div className="knowledge-graph">
          <ReactFlow
            nodes={nodes}
            onNodesChange={onNodesChange}
            edges={edges}
            fitView
            nodesDraggable={false}
            nodesConnectable={false}
            edgesFocusable={false}
            edgesReconnectable={false}
            deleteKeyCode={null}
            onNodeClick={(_, node) => {
              onOpen(Number(node.id));
            }}
            minZoom={0.15}
            maxZoom={2}
          >
            <KnowledgeViewport signature={nodes.map((n) => n.id).join(",")} />
            <Background />
            <Controls showInteractive={false} />
          </ReactFlow>
        </div>
      </ReactFlowProvider>
      <nav aria-label="Linked notes">
        {nodes.map((n) => (
          <button
            key={n.id}
            aria-current={Number(n.id) === selected ? "page" : undefined}
            onClick={() => {
              onOpen(Number(n.id));
            }}
          >
            {String(n.data["label"])}
          </button>
        ))}
      </nav>
    </section>
  );
}
