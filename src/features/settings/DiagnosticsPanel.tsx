import { useEffect, useState } from "react";
import {
  diagnosticsClient,
  parseDiagnostics,
  type DiagnosticsSnapshot,
} from "../../infrastructure/tauri/diagnostics-client";

import { diagnosticAdvice, troubleshootingSummary } from "./diagnosticSummary";

export function DiagnosticsPanel({
  client = diagnosticsClient,
}: {
  readonly client?: typeof diagnosticsClient;
}) {
  const [snapshot, setSnapshot] = useState<DiagnosticsSnapshot | null>(null);
  const [severity, setSeverity] = useState("all");
  const [provider, setProvider] = useState("all");
  const [correlation, setCorrelation] = useState("all");
  const [notice, setNotice] = useState("");
  const [busy, setBusy] = useState(false);
  useEffect(() => {
    let active = true;
    void client
      .read()
      .then(parseDiagnostics)
      .then(
        (value) => {
          if (active) setSnapshot(value);
        },
        () => {
          if (active) setNotice("Diagnostics unavailable. Native access is required.");
        },
      );
    return () => {
      active = false;
    };
  }, [client]);
  async function refresh() {
    setBusy(true);
    try {
      setSnapshot(parseDiagnostics(await client.read()));
      setNotice("Diagnostic snapshot refreshed.");
    } catch {
      setNotice("Diagnostics unavailable. Existing operation results are unchanged.");
    } finally {
      setBusy(false);
    }
  }
  async function copy() {
    if (!snapshot) return;
    try {
      await navigator.clipboard.writeText(JSON.stringify(snapshot, null, 2));
      setNotice("Sanitized diagnostic summary copied.");
    } catch {
      setNotice("Copy unavailable. Use the selectable summary below or export.");
    }
  }
  async function copyTroubleshooting() {
    try {
      await navigator.clipboard.writeText(troubleshootingSummary(timeline));
      setNotice("Troubleshooting summary copied.");
    } catch {
      setNotice("Copy unavailable. Select the troubleshooting summary below.");
    }
  }
  async function exportRecords() {
    setBusy(true);
    try {
      setNotice(
        (await client.export())
          ? "Sanitized diagnostics exported."
          : "Export cancelled; no file written.",
      );
    } catch (error) {
      setNotice(
        error === "destination_exists"
          ? "Destination exists. Nothing was overwritten; choose a new filename."
          : "Diagnostics export unavailable. No automatic retry was made.",
      );
    } finally {
      setBusy(false);
    }
  }
  const timeline =
    snapshot?.events.filter(
      (e) =>
        (provider === "all" || e.provider === provider) &&
        (correlation === "all" || [e.request, e.attempt, e.workflow].includes(correlation)),
    ) ?? [];
  const events = timeline.filter((e) => severity === "all" || e.severity === severity);
  const correlations = [
    ...new Set(
      snapshot?.events
        .flatMap((e) => [e.request, e.workflow])
        .filter((v): v is string => Boolean(v)) ?? [],
    ),
  ];
  return (
    <section
      className="settings-card page-panel local-diagnostics"
      aria-labelledby="local-diagnostics-title"
    >
      <h2 id="local-diagnostics-title">Local diagnostics</h2>
      <p role="status">
        {snapshot
          ? snapshot.available
            ? "Logging available"
            : "Diagnostics unavailable — records may be incomplete"
          : notice
            ? "Diagnostics unavailable"
            : "Checking native diagnostics"}
      </p>
      <p>
        Content-free execution events. Up to three 5 MiB local files; the most recent 256 records
        are shown, copied and exported. No prompts, answers, notes, documents, credentials or raw
        runtime output. These are not tamper-proof audit records.
      </p>
      <p>
        Times are UTC. Elapsed times use a monotonic clock. First response means HTTP headers for
        API connections or the first accepted normalized event for Codex and Simulation; first text
        is measured separately. Dynamic model IDs are fingerprinted.
      </p>
      <div className="local-diagnostics__controls">
        <label>
          Severity{" "}
          <select
            value={severity}
            onChange={(e) => {
              setSeverity(e.target.value);
            }}
          >
            <option value="all">All</option>
            <option value="info">Info</option>
            <option value="warning">Warning</option>
            <option value="error">Error</option>
          </select>
        </label>
        <label>
          Provider/runtime{" "}
          <select
            value={provider}
            onChange={(e) => {
              setProvider(e.target.value);
            }}
          >
            <option value="all">All providers</option>
            {["simulation", "openai_api", "anthropic_api", "codex", "lm_studio", "ollama"].map(
              (p) => (
                <option key={p}>{p}</option>
              ),
            )}
          </select>
        </label>
        <label style={{ minWidth: 0, maxWidth: "100%" }}>
          Request/run ID{" "}
          <select
            value={correlation}
            onChange={(e) => {
              setCorrelation(e.target.value);
            }}
            style={{ width: "24rem", maxWidth: "100%" }}
          >
            <option value="all">All requests/runs</option>
            {correlations.map((id) => (
              <option key={id}>{id}</option>
            ))}
          </select>
        </label>
        <button
          type="button"
          disabled={!snapshot || busy}
          onClick={() => {
            void copyTroubleshooting();
          }}
        >
          Copy troubleshooting summary
        </button>
        <button
          type="button"
          disabled={busy}
          onClick={() => {
            void refresh();
          }}
        >
          Refresh diagnostics
        </button>
        <button
          type="button"
          disabled={!snapshot || busy}
          onClick={() => {
            void copy();
          }}
        >
          Copy sanitized summary
        </button>
        <button
          type="button"
          disabled={!snapshot || busy}
          onClick={() => {
            void exportRecords();
          }}
        >
          Export diagnostics
        </button>
      </div>
      {notice ? <p role="status">{notice}</p> : null}
      <h3>Request timeline</h3>
      <p>
        Chronological observations only. Attempt IDs begin at dispatch; Codex internal retries are
        runtime-controlled and not individually observable. Transport released means the local
        future was dropped, not remote cancellation. Terminal returned means the native poll
        returned terminal data, not proof of frontend receipt or screen rendering.
      </p>
      <details>
        <summary>Troubleshooting summary</summary>
        <pre>{troubleshootingSummary(timeline)}</pre>
      </details>
      <div className="local-diagnostics__events" tabIndex={0} aria-label="Recent diagnostic events">
        {events.length === 0 ? (
          <p>No matching diagnostic events.</p>
        ) : (
          <ol>
            {events
              .slice()

              .map((e, i) => (
                <li key={`${e.session}-${String(e.utcMs)}-${String(i)}`}>
                  <strong>
                    {e.event} · {e.outcome}
                  </strong>
                  <br />
                  <time>{new Date(e.utcMs).toISOString()}</time> · {e.severity} ·{" "}
                  {e.provider ?? "application"} {e.model ?? ""}
                  {e.error ? <span> · {e.error}</span> : null}
                  {e.durationMs !== null ? <span> · {e.durationMs} ms elapsed</span> : null}
                  {e.error ? <p>{diagnosticAdvice(e.error)}</p> : null}
                  {e.request || e.attempt ? (
                    <details>
                      <summary>Correlation</summary>
                      {e.request ? <p>Request: {e.request}</p> : null}
                      <code>{e.attempt ?? "No dispatch observed"}</code>
                      {e.conversation ? <p>Conversation: {e.conversation}</p> : null}
                      {e.workflow ? <p>Workflow: {e.workflow}</p> : null}
                      {e.stage ? <p>Stage: {e.stage}</p> : null}
                    </details>
                  ) : null}
                </li>
              ))}
          </ol>
        )}
      </div>
      {snapshot ? (
        <details>
          <summary>Selectable sanitized summary</summary>
          <pre>{JSON.stringify(snapshot, null, 2)}</pre>
        </details>
      ) : null}
    </section>
  );
}
