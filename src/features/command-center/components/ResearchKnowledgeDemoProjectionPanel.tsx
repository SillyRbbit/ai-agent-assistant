import { DatabaseZap, RefreshCw } from "lucide-react";
import { useEffect, useRef, useState } from "react";

import {
  RESEARCH_KNOWLEDGE_DEMO_DISCLOSURE,
  RESEARCH_KNOWLEDGE_DEMO_PROOF_BOUNDARY,
  type ResearchKnowledgeDemoProjection,
  type ResearchKnowledgeDemoProjectionLoader,
} from "../../../infrastructure/tauri/research-knowledge-demo-projection-client";

type ProjectionState =
  | Readonly<{ status: "idle" }>
  | Readonly<{ status: "loading" }>
  | Readonly<{ projection: ResearchKnowledgeDemoProjection; status: "ready" }>
  | Readonly<{ status: "unavailable" }>;

interface ResearchKnowledgeDemoProjectionPanelProps {
  readonly loader: ResearchKnowledgeDemoProjectionLoader;
}

export function ResearchKnowledgeDemoProjectionPanel({
  loader,
}: ResearchKnowledgeDemoProjectionPanelProps) {
  const [state, setState] = useState<ProjectionState>({ status: "idle" });
  const requestGeneration = useRef(0);

  useEffect(
    () => () => {
      requestGeneration.current += 1;
    },
    [],
  );

  const refresh = async () => {
    const generation = requestGeneration.current + 1;
    requestGeneration.current = generation;
    setState({ status: "loading" });

    try {
      const projection = await loader();
      if (requestGeneration.current === generation) {
        setState({ projection, status: "ready" });
      }
    } catch {
      if (requestGeneration.current === generation) {
        setState({ status: "unavailable" });
      }
    }
  };

  const statusMessage =
    state.status === "idle"
      ? "Rust projection not requested. The frontend fixture remains unchanged."
      : state.status === "loading"
        ? "Requesting the read-only Rust projection."
        : state.status === "ready"
          ? "Read-only Rust projection received. No workflow was started."
          : "Rust demo projection unavailable. The deterministic frontend fixture remains available.";

  return (
    <section
      aria-busy={state.status === "loading"}
      aria-label="Read-only Rust demo projection"
      className="command-center-native-projection"
    >
      <div className="command-center-native-projection__header">
        <div>
          <p className="section-kicker">Separate deterministic proof</p>
          <h2>Research/Knowledge Rust projection</h2>
          <p>{RESEARCH_KNOWLEDGE_DEMO_PROOF_BOUNDARY}</p>
        </div>
        <span className="command-center-demo-badge">
          <DatabaseZap aria-hidden="true" /> {RESEARCH_KNOWLEDGE_DEMO_DISCLOSURE}
        </span>
      </div>

      <p aria-atomic="true" aria-live="polite" className="command-center-native-projection__status">
        {statusMessage}
      </p>

      {state.status === "ready" ? (
        <div className="command-center-native-projection__result">
          <dl>
            <div>
              <dt>Schema</dt>
              <dd>{state.projection.schemaVersion}</dd>
            </div>
            <div>
              <dt>Scenario</dt>
              <dd>{state.projection.scenarioId}</dd>
            </div>
            <div>
              <dt>Fixture provenance</dt>
              <dd>{state.projection.fixtureProvenance}</dd>
            </div>
          </dl>
          <div>
            <h3>Projected roles</h3>
            <ul>
              {state.projection.roles.map((role) => (
                <li key={role.id}>
                  <span>{role.label}</span>
                  <small>{role.state}</small>
                </li>
              ))}
            </ul>
          </div>
          <div>
            <h3>Simulated outcome vocabulary</h3>
            <ul>
              {state.projection.simulatedOutcomes.map((outcome) => (
                <li key={outcome}>{outcome}</li>
              ))}
            </ul>
          </div>
        </div>
      ) : null}

      <button
        className="command-center-button"
        disabled={state.status === "loading"}
        onClick={() => void refresh()}
        type="button"
      >
        <RefreshCw aria-hidden="true" />
        {state.status === "loading" ? "Refreshing Rust projection" : "Refresh Rust projection"}
      </button>
    </section>
  );
}
