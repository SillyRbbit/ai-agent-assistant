import type { UsageView } from "../../infrastructure/tauri/usage-efficiency-client";
import { displayMetric as display } from "./usage-presentation";

// Presentation only: every count, ratio and task average is supplied by Rust.
export function UsageSummary({ view }: { readonly view: UsageView }) {
  return (
    <div className="usage-efficiency__summary">
      <dl
        className="usage-efficiency__headline usage-efficiency__surface"
        aria-label="Sample token summary"
      >
        <div className="usage-efficiency__total">
          <dt>Total tokens</dt>
          <dd>{display(view.total)}</dd>
          <dd className="usage-efficiency__annotation">Input + output, counted once</dd>
        </div>
        <div className="usage-efficiency__input">
          <dt>Input tokens</dt>
          <dd>{display(view.tokens[0])}</dd>
          <dd className="usage-efficiency__annotation">
            Cached input: {display(view.tokens[1])} · included in input
          </dd>
        </div>
        <div className="usage-efficiency__output">
          <dt>Output tokens</dt>
          <dd>{display(view.tokens[2])}</dd>
          <dd className="usage-efficiency__annotation">
            Reasoning: {display(view.tokens[3])} · included in output
          </dd>
        </div>
      </dl>
      <dl
        className="usage-efficiency__efficiency usage-efficiency__surface"
        aria-label="Sample efficiency summary"
      >
        <div className="usage-efficiency__cache">
          <dt>Cache utilization</dt>
          <dd>{display(view.cachePercent, "%")}</dd>
          <dd className="usage-efficiency__annotation">
            Cached share of input · not quota
            {typeof view.cachePercent === "number" && (
              <svg className="usage-efficiency__cache-ring" viewBox="0 0 64 64" aria-hidden="true">
                <circle
                  cx="32"
                  cy="32"
                  r="25"
                  fill="none"
                  strokeWidth="5"
                  className="usage-efficiency__ring-track"
                />
                <circle
                  cx="32"
                  cy="32"
                  r="25"
                  fill="none"
                  strokeWidth="5"
                  stroke="currentColor"
                  pathLength="100"
                  strokeDasharray={`${String(view.cachePercent)} 100`}
                  transform="rotate(-90 32 32)"
                />
              </svg>
            )}
          </dd>
        </div>
        <div className="usage-efficiency__task-average">
          <dt>Tokens per completed task</dt>
          <dd>{display(view.completedTaskMean)}</dd>
          {view.completedTaskMean === "missing" && (
            <dd className="usage-efficiency__annotation">Incomplete task coverage</dd>
          )}
        </div>
        <div className="usage-efficiency__completed">
          <dt>Completed tasks</dt>
          <dd>{display(view.completedTasks)}</dd>
        </div>
      </dl>
    </div>
  );
}
