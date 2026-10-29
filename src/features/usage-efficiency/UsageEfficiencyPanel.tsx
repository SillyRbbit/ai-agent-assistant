import { useEffect, useId, useRef, useState } from "react";
import { RefreshCw, Sparkles, Layers3 } from "lucide-react";
import {
  fetchUsageProjection,
  USAGE_CATEGORIES,
  USAGE_DISCLOSURE,
  type UsageCategory,
  type UsageLoader,
  type UsageProjection,
} from "../../infrastructure/tauri/usage-efficiency-client";
import { TokenComposition, TokenTrend } from "./UsageCharts";
import { UsageSummary } from "./UsageSummary";
import { displayMetric as display } from "./usage-presentation";
import "./usage-efficiency.css";
import "./usage-disclosure.css";

const LABELS: Record<UsageCategory, string> = {
  simulation: "Simulation",
  openai_api: "OpenAI API",
  anthropic_api: "Anthropic API",
  lm_studio: "LM Studio",
  ollama: "Ollama",
  codex: "Codex",
};
type Example = "activity" | "empty" | "loading" | "error";

export function UsageEfficiencyPanel({
  loader = fetchUsageProjection,
}: {
  readonly loader?: UsageLoader;
}) {
  const rangeId = useId();
  const [category, setCategory] = useState<UsageCategory>("simulation");
  const [days, setDays] = useState<7 | 30>(7);
  const [example, setExample] = useState<Example>("activity");
  const [data, setData] = useState<UsageProjection>();
  const [status, setStatus] = useState<"idle" | "loading" | "ready" | "error">("idle");
  const generation = useRef(0);
  useEffect(
    () => () => {
      generation.current += 1;
    },
    [],
  );
  const reset = () => {
    generation.current += 1;
    setData(undefined);
    setStatus("idle");
  };
  const selectCategory = (next: UsageCategory) => {
    if (next === category) return;
    reset();
    setCategory(next);
  };
  const load = async () => {
    const serial = ++generation.current;
    setData(undefined);
    setStatus("loading");
    try {
      const result = await loader();
      if (generation.current === serial) {
        setData(result);
        setStatus("ready");
      }
    } catch {
      if (generation.current === serial) {
        setData(undefined);
        setStatus("error");
      }
    }
  };
  const view =
    status === "ready" && (example === "activity" || example === "empty")
      ? data?.views.find((v) => v.category === category && v.days === days && v.example === example)
      : undefined;
  const loading = status === "loading" || example === "loading";

  return (
    <section
      aria-labelledby="usage-efficiency-title"
      aria-busy={loading}
      className="usage-efficiency"
    >
      <header className="usage-efficiency__header">
        <div>
          <p className="usage-efficiency__eyebrow">Usage intelligence</p>
          <h2 id="usage-efficiency-title">Usage &amp; Efficiency</h2>
          <p>Explore token activity and context efficiency.</p>
        </div>
      </header>
      <div className="usage-efficiency__notice">
        <span className="usage-efficiency__notice-icon" aria-hidden="true">
          <Sparkles size={18} />
        </span>
        <div>
          <strong>{USAGE_DISCLOSURE}</strong>
          <p>One sample connection at a time. No account usage, subscription limits or billing.</p>
        </div>
        <span className="usage-efficiency__badge">Sample data</span>
      </div>
      <div className="usage-efficiency__toolbar">
        <fieldset className="usage-efficiency__sources">
          <legend className="usage-efficiency__sr-only">Sample source</legend>
          {USAGE_CATEGORIES.map((source) => (
            <label key={source}>
              <input
                type="radio"
                className="usage-efficiency__sr-only"
                name={`${rangeId}-source`}
                value={source}
                checked={category === source}
                onChange={() => {
                  selectCategory(source);
                }}
              />
              <span>{LABELS[source]}</span>
            </label>
          ))}
        </fieldset>
        <label className="usage-efficiency__source-select">
          <span className="usage-efficiency__sr-only">Example connection</span>
          <select
            value={category}
            onChange={(event) => {
              const next = USAGE_CATEGORIES.find((c) => c === event.target.value);
              if (next) selectCategory(next);
            }}
          >
            {USAGE_CATEGORIES.map((c) => (
              <option key={c} value={c}>
                {LABELS[c]}
              </option>
            ))}
          </select>
        </label>
        <fieldset className="usage-efficiency__range">
          <legend className="usage-efficiency__sr-only">Fictional UTC range</legend>
          {([7, 30] as const).map((range) => (
            <label key={range}>
              <input
                className="usage-efficiency__sr-only"
                type="radio"
                name={`${rangeId}-range`}
                value={range}
                checked={days === range}
                onChange={() => {
                  reset();
                  setDays(range);
                }}
              />
              <span>{range} days</span>
            </label>
          ))}
        </fieldset>
        <button
          className="usage-efficiency__refresh"
          type="button"
          disabled={loading || example === "error"}
          onClick={() => {
            void load();
          }}
        >
          <RefreshCw size={14} aria-hidden="true" />
          {status === "loading" ? "Loading…" : view ? "Refresh examples" : "Load examples"}
        </button>
      </div>
      <div className="usage-efficiency__status-row">
        <p role="status" aria-live="polite" aria-atomic="true">
          {example === "loading"
            ? "Loading illustration only — no request is pending."
            : example === "error"
              ? "Error illustration — no activity data is available."
              : status === "loading"
                ? "Loading mock-only examples."
                : status === "error"
                  ? "Native preview unavailable or invalid. No live fallback. No data retained."
                  : view
                    ? `${LABELS[category]} · ${view.start} to ${view.end} UTC (end exclusive).`
                    : "Load sample metrics to explore the dashboard. Native preview only."}
        </p>
        <details className="usage-efficiency__demo-controls usage-disclosure">
          <summary>Preview controls</summary>
          <label>
            Example state
            <select
              value={example}
              onChange={(event) => {
                reset();
                const next = event.target.value;
                if (
                  next === "activity" ||
                  next === "empty" ||
                  next === "loading" ||
                  next === "error"
                )
                  setExample(next);
              }}
            >
              <option value="activity">Activity</option>
              <option value="empty">Empty</option>
              <option value="loading">Loading illustration</option>
              <option value="error">Error illustration</option>
            </select>
          </label>
        </details>
      </div>
      {view ? (
        <>
          {view.example === "empty" && <p>No fictional activity in this complete empty example.</p>}
          <UsageSummary view={view} />
          <div className="usage-efficiency__charts">
            <TokenTrend view={view} />
            <TokenComposition view={view} />
          </div>
          <details className="usage-efficiency__surface usage-efficiency__daily usage-disclosure">
            <summary>
              Daily token details <span>{view.daily.length} rows · UTC</span>
            </summary>
            <div
              className="usage-efficiency__table"
              role="region"
              aria-label="Daily token examples"
              tabIndex={0}
            >
              <table>
                <caption>
                  {LABELS[category]} — {days}-day fictional daily tokens, UTC
                </caption>
                <thead>
                  <tr>
                    {["Date", "Input", "Cached input", "Output", "Reasoning", "Total"].map((h) => (
                      <th key={h} scope="col">
                        {h}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {view.daily.map((row) => (
                    <tr key={row.date}>
                      <th scope="row">{row.date}</th>
                      {row.tokens.map((n, i) => (
                        <td key={i}>{display(n)}</td>
                      ))}
                      <td>{display(row.total)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
              {view.daily.length === 0 && <p>No daily records in the empty illustration.</p>}
            </div>
          </details>
        </>
      ) : (
        <div className="usage-efficiency__surface usage-efficiency__placeholder">
          <Layers3 size={28} aria-hidden="true" />
          <h3>
            {loading
              ? "Sample preview loading"
              : example === "error" || status === "error"
                ? "Sample preview unavailable"
                : "Your usage, at a glance"}
          </h3>
          <p>
            {loading
              ? "Metrics stay hidden until a complete result is available."
              : "Load examples to preview token cards, daily trends and efficiency metrics."}
          </p>
        </div>
      )}
      <details className="usage-efficiency__notes usage-disclosure">
        <summary>How to read these examples</summary>
        <p>
          Cached input is included in input; reasoning is included in output. Total = input +
          output. Failed/cancelled activity counts, but is excluded from completed-task averages.
          Incomplete task coverage makes that average Missing.
        </p>
        <p>
          Zero = explicitly supplied zero. Missing = incomplete coverage. Unavailable = not supplied
          for this example. Unverified = unsupported evidence. Every number and date is fabricated;
          connection labels do not establish live support.
        </p>
      </details>
    </section>
  );
}
