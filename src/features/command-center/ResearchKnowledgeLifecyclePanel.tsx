import { useEffect, useRef, useState } from "react";

import {
  createResearchKnowledgeDemoLifecycleClient,
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE,
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY,
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE,
  type ResearchKnowledgeDemoLifecycleClient,
  type ResearchKnowledgeDemoLifecycleSnapshot,
} from "../../infrastructure/tauri/research-knowledge-demo-lifecycle-client";

const MAX_PRESENTATION_EPOCH = 0xffff_ffff;

type LifecycleState = ResearchKnowledgeDemoLifecycleSnapshot["state"];
type LifecycleEventKind = ResearchKnowledgeDemoLifecycleSnapshot["journal"][number]["kind"];

const STATE_COPY: Readonly<Record<LifecycleState, string>> = {
  idle: "Ready for an explicit simulated start.",
  research: "Application-owned synthetic Research stage active.",
  knowledge: "Application-owned synthetic Knowledge stage active.",
  synthesis: "Application-owned synthetic synthesis stage active.",
  succeeded: "Application-owned synthetic rehearsal completed.",
  failed: "Application-owned synthetic rehearsal failed.",
  cancelled: "Application-owned synthetic rehearsal cancelled.",
  "cleanup-pending": RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE,
};

const JOURNAL_COPY: Readonly<Record<LifecycleEventKind, string>> = {
  "research-started": "Research stage started",
  "research-completed": "Research stage completed",
  "knowledge-started": "Knowledge stage started",
  "knowledge-completed": "Knowledge stage completed",
  "synthesis-started": "Synthesis stage started",
  completed: "Synthetic rehearsal completed",
  failed: "Synthetic rehearsal failed",
  cancelled: "Synthetic rehearsal cancelled",
  "cleanup-pending": "Lifecycle unavailable",
};

type LifecycleOperation = "start" | "advance" | "cancel";

type LifecycleViewState =
  | Readonly<{ status: "connecting" }>
  | Readonly<{ snapshot: ResearchKnowledgeDemoLifecycleSnapshot; status: "ready" }>
  | Readonly<{ status: "unavailable" }>;

function operationCopy(operation: LifecycleOperation): string {
  switch (operation) {
    case "start":
      return "Starting the simulated lifecycle.";
    case "advance":
      return "Advancing the simulated lifecycle.";
    case "cancel":
      return "Cancelling the simulated lifecycle.";
  }
}

function stateLabel(state: LifecycleState): string {
  return state === "cleanup-pending"
    ? "Unavailable"
    : state.slice(0, 1).toUpperCase() + state.slice(1);
}

function disposeLifecycleClient(client: ResearchKnowledgeDemoLifecycleClient): void {
  client.dispose();
}

export function ResearchKnowledgeLifecyclePanel() {
  const [viewState, setViewState] = useState<LifecycleViewState>({ status: "connecting" });
  const [operation, setOperation] = useState<LifecycleOperation | null>(null);
  const clientRef = useRef<ResearchKnowledgeDemoLifecycleClient | undefined>(undefined);
  const generationRef = useRef(0);

  useEffect(() => {
    const generation = generationRef.current + 1;
    generationRef.current = generation;

    void createResearchKnowledgeDemoLifecycleClient((snapshot) => {
      if (generationRef.current === generation && clientRef.current !== undefined) {
        setViewState({ snapshot, status: "ready" });
      }
    })
      .then(async (client) => {
        if (generationRef.current !== generation) {
          disposeLifecycleClient(client);
          return;
        }
        clientRef.current = client;
        try {
          const snapshot = await client.snapshot();
          if (generationRef.current === generation) {
            setViewState({ snapshot, status: "ready" });
          }
        } catch {
          if (generationRef.current === generation) {
            disposeLifecycleClient(client);
            clientRef.current = undefined;
            setViewState({ status: "unavailable" });
          }
        }
      })
      .catch(() => {
        if (generationRef.current === generation) {
          setViewState({ status: "unavailable" });
        }
      });

    return () => {
      generationRef.current += 1;
      const client = clientRef.current;
      if (client !== undefined) disposeLifecycleClient(client);
      clientRef.current = undefined;
    };
  }, []);

  const snapshot = viewState.status === "ready" ? viewState.snapshot : undefined;
  const isActive =
    snapshot?.state === "research" ||
    snapshot?.state === "knowledge" ||
    snapshot?.state === "synthesis";
  const canStart =
    snapshot !== undefined &&
    snapshot.presentationEpoch < MAX_PRESENTATION_EPOCH &&
    (snapshot.state === "idle" ||
      snapshot.state === "succeeded" ||
      snapshot.state === "failed" ||
      snapshot.state === "cancelled");
  const busy = operation !== null || viewState.status === "connecting";

  const invokeOperation = async (requestedOperation: LifecycleOperation) => {
    const client = clientRef.current;
    if (client === undefined || viewState.status !== "ready" || operation !== null) return;
    if (client.recoveryRequired) {
      disposeLifecycleClient(client);
      clientRef.current = undefined;
      setViewState({ status: "unavailable" });
      return;
    }

    const generation = generationRef.current;
    setOperation(requestedOperation);
    try {
      let nextSnapshot: ResearchKnowledgeDemoLifecycleSnapshot;
      switch (requestedOperation) {
        case "start":
          nextSnapshot = await client.start();
          break;
        case "advance":
          nextSnapshot = await client.advance();
          break;
        case "cancel":
          nextSnapshot = await client.cancel();
          break;
      }
      if (generationRef.current === generation) {
        setViewState({ snapshot: nextSnapshot, status: "ready" });
      }
    } catch {
      if (generationRef.current === generation) {
        disposeLifecycleClient(client);
        clientRef.current = undefined;
        setViewState({ status: "unavailable" });
      }
    } finally {
      if (generationRef.current === generation) {
        setOperation(null);
      }
    }
  };

  const statusCopy =
    operation !== null
      ? operationCopy(operation)
      : viewState.status === "connecting"
        ? "Loading the current volatile lifecycle snapshot."
        : viewState.status === "unavailable"
          ? RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE
          : STATE_COPY[viewState.snapshot.state];

  return (
    <section
      aria-busy={busy}
      aria-labelledby="research-knowledge-lifecycle-title"
      className="command-center-native-lifecycle"
    >
      <div className="command-center-native-lifecycle__header">
        <div>
          <p className="section-kicker">Separate deterministic proof</p>
          <h2 id="research-knowledge-lifecycle-title">Research/Knowledge simulated lifecycle</h2>
          <p>{RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY}</p>
        </div>
        <span className="command-center-demo-badge">
          {RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE}
        </span>
      </div>

      <p aria-atomic="true" aria-live="polite" className="command-center-native-lifecycle__status">
        {statusCopy}
      </p>

      {snapshot !== undefined ? (
        <div className="command-center-native-lifecycle__body">
          <dl className="command-center-native-lifecycle__summary">
            <div>
              <dt>Epoch</dt>
              <dd>{snapshot.presentationEpoch}</dd>
            </div>
            <div>
              <dt>Revision</dt>
              <dd>{snapshot.revision}</dd>
            </div>
            <div>
              <dt>State</dt>
              <dd>{stateLabel(snapshot.state)}</dd>
            </div>
          </dl>

          <div className="command-center-native-lifecycle__journal">
            <h3>Simulated Rust lifecycle activity</h3>
            {snapshot.journal.length === 0 ? (
              <p>No simulated lifecycle activity yet.</p>
            ) : (
              <ol aria-label="Simulated Rust lifecycle activity">
                {snapshot.journal.map((entry) => (
                  <li key={`${String(snapshot.presentationEpoch)}-${String(entry.revision)}`}>
                    <span>{JOURNAL_COPY[entry.kind]}</span>
                    <small>Revision {entry.revision}</small>
                  </li>
                ))}
              </ol>
            )}
          </div>
        </div>
      ) : null}

      {snapshot?.state === "failed" ? (
        <p className="command-center-native-lifecycle__failure">
          This is an application-owned synthetic rehearsal outcome, not a provider, model, tool,
          approval, policy, audit, or device failure.
        </p>
      ) : null}

      <div
        aria-label="Simulated lifecycle actions"
        className="command-center-native-lifecycle__actions"
        role="group"
      >
        <button
          className="command-center-button"
          disabled={busy || !canStart}
          onClick={() => void invokeOperation("start")}
          type="button"
        >
          Start simulated lifecycle
        </button>
        <button
          className="command-center-button"
          disabled={busy || !isActive}
          onClick={() => void invokeOperation("advance")}
          type="button"
        >
          Advance simulated lifecycle
        </button>
        <button
          className="command-center-button"
          disabled={busy || !isActive}
          onClick={() => void invokeOperation("cancel")}
          type="button"
        >
          Cancel simulated lifecycle
        </button>
      </div>

      <p className="command-center-native-lifecycle__note">
        Controls manually step one volatile application-owned synthetic fixture. Notifications are
        non-authoritative copies of command results.
      </p>
    </section>
  );
}
