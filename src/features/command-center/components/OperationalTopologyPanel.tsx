import { Network } from "lucide-react";
import type { ReactNode } from "react";

import {
  COMMAND_CENTER_DISCLOSURE,
  type CommandCenterProjection,
  type TopologyEdge,
  type TopologyNode,
} from "../commandCenterProjection";
import { OperationalTopologyAdapter } from "./OperationalTopologyAdapter";

export interface OperationalTopologyPanelProps {
  readonly activeNodeId?: string | null | undefined;
  readonly onEscape?: (() => void) | undefined;
  readonly onSelect: (id: string) => void;
  readonly projection: CommandCenterProjection;
  readonly selectedId: string | null;
  readonly structuredView: ReactNode;
  readonly viewMode: "graph" | "structured";
  readonly visibleEdges?: readonly TopologyEdge[] | undefined;
  readonly visibleNodes?: readonly TopologyNode[] | undefined;
}

export function OperationalTopologyPanel({
  activeNodeId,
  onEscape,
  onSelect,
  projection,
  selectedId,
  structuredView,
  viewMode,
  visibleEdges,
  visibleNodes,
}: OperationalTopologyPanelProps) {
  return (
    <section aria-labelledby="command-center-topology-title" className="command-center-panel">
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
        <OperationalTopologyAdapter
          activeNodeId={activeNodeId}
          onEscape={onEscape}
          onSelect={onSelect}
          projection={projection}
          selectedId={selectedId}
          visibleEdges={visibleEdges}
          visibleNodes={visibleNodes}
        />
      ) : (
        structuredView
      )}
    </section>
  );
}
