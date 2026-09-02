import {
  AlertTriangle,
  CircleOff,
  ListTree,
  LocateFixed,
  Maximize2,
  RotateCcw,
  ZoomIn,
  ZoomOut,
} from "lucide-react";
import { Component, type ReactNode } from "react";

interface GraphRenderBoundaryProps {
  readonly children: ReactNode;
  readonly resetKey: string;
}

interface GraphRenderBoundaryState {
  readonly failed: boolean;
}

const UNAVAILABLE_GRAPH_CONTROLS = [
  { icon: ZoomIn, label: "Zoom in" },
  { icon: ZoomOut, label: "Zoom out" },
  { icon: Maximize2, label: "Fit view" },
  { icon: RotateCcw, label: "Reset view" },
  { icon: LocateFixed, label: "Center selected" },
  { icon: LocateFixed, label: "Center active" },
  { icon: CircleOff, label: "Clear selection" },
] as const;

export class GraphRenderBoundary extends Component<
  GraphRenderBoundaryProps,
  GraphRenderBoundaryState
> {
  override state: GraphRenderBoundaryState = { failed: false };

  static getDerivedStateFromError(): GraphRenderBoundaryState {
    return { failed: true };
  }

  override componentDidCatch(): void {
    // React owns error reporting. The boundary keeps the Graph-only failure local and truthful.
  }

  override componentDidUpdate(previousProps: GraphRenderBoundaryProps): void {
    if (this.state.failed && previousProps.resetKey !== this.props.resetKey) {
      this.setState({ failed: false });
    }
  }

  override render() {
    if (!this.state.failed) return this.props.children;

    return (
      <div className="command-center-topology">
        <div
          aria-label="Simulated operational topology rendering error"
          className="command-center-canvas"
          role="region"
        >
          <div className="command-center-graph-state" role="alert">
            <AlertTriangle aria-hidden="true" />
            <div>
              <h3>Graph rendering unavailable</h3>
              <p>
                The bounded topology renderer could not draw this view. No task, provider, tool, or
                device action was attempted.
              </p>
            </div>
            <button
              className="command-center-button"
              onClick={() => {
                this.setState({ failed: false });
              }}
              type="button"
            >
              Retry graph renderer
            </button>
          </div>
        </div>
        <div
          aria-label="Topology viewport"
          className="command-center-toolbar command-center-graph-toolbar"
          role="toolbar"
        >
          <output aria-label="Topology zoom level" className="command-center-graph-toolbar__zoom">
            N/A
          </output>
          {UNAVAILABLE_GRAPH_CONTROLS.map(({ icon: Icon, label }) => (
            <button
              aria-label={label}
              className="command-center-button"
              disabled
              key={label}
              title={label}
              type="button"
            >
              <Icon aria-hidden="true" />
              <span className="command-center-graph-toolbar__label">{label}</span>
            </button>
          ))}
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
              <strong>Visible relationships</strong>
              <p>Relationship data is unavailable while the graph renderer is unavailable.</p>
            </div>
          </details>
        </div>
      </div>
    );
  }
}
