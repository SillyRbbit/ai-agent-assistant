import { Network } from "lucide-react";
import type { ReactNode } from "react";

import {
  COMMAND_CENTER_DISCLOSURE,
  type CommandCenterProjection,
  type TopologyEdge,
  type TopologyNode,
} from "../commandCenterProjection";
import { GraphRenderBoundary } from "./GraphRenderBoundary";
import { OperationalTopologyAdapter } from "./OperationalTopologyAdapter";

export interface OperationalTopologyPanelProps {
  readonly activeNodeId?: string | null | undefined;
  readonly filtersActive?: boolean | undefined;
  readonly onClearSelection?: (() => void) | undefined;
  readonly onEscape?: (() => void) | undefined;
  readonly onResetFilters?: (() => void) | undefined;
  readonly onSelect: (id: string) => void;
  readonly projection: CommandCenterProjection;
  readonly selectedId: string | null;
  readonly selectionPresent?: boolean | undefined;
  readonly structuredView: ReactNode;
  readonly viewMode: "graph" | "structured";
  readonly visibleEdges?: readonly TopologyEdge[] | undefined;
  readonly visibleNodes?: readonly TopologyNode[] | undefined;
}

export function OperationalTopologyPanel({
  activeNodeId,
  filtersActive,
  onClearSelection,
  onEscape,
  onResetFilters,
  onSelect,
  projection,
  selectedId,
  selectionPresent,
  structuredView,
  viewMode,
  visibleEdges,
  visibleNodes,
}: OperationalTopologyPanelProps) {
  const graphResetKey = `${projection.scenarioId}:${(visibleNodes ?? projection.nodes)
    .map((node) => node.id)
    .join("|")}:${(visibleEdges ?? projection.edges).map((edge) => edge.id).join("|")}`;

  return (
    <section
      aria-labelledby="command-center-topology-title"
      className={`command-center-panel${
        viewMode === "graph" ? " command-center-panel--graph" : ""
      }`}
    >
      <header className="command-center-panel__header">
        <div>
          <h2 id="command-center-topology-title">Operational topology</h2>
          <p>
            {projection.scenarioLabel} · deterministic frontend fixture ·{" "}
            {COMMAND_CENTER_DISCLOSURE}
          </p>
        </div>
        <Network aria-hidden="true" size={18} />
      </header>

      {viewMode === "graph" ? (
        <GraphRenderBoundary resetKey={graphResetKey}>
          <OperationalTopologyAdapter
            activeNodeId={activeNodeId}
            filtersActive={filtersActive}
            onClearSelection={onClearSelection}
            onEscape={onEscape}
            onResetFilters={onResetFilters}
            onSelect={onSelect}
            projection={projection}
            selectedId={selectedId}
            selectionPresent={selectionPresent}
            visibleEdges={visibleEdges}
            visibleNodes={visibleNodes}
          />
        </GraphRenderBoundary>
      ) : (
        structuredView
      )}
    </section>
  );
}
