import { useId } from "react";
import type { UsageView } from "../../infrastructure/tauri/usage-efficiency-client";
import { displayMetric, trendGeometry } from "./usage-presentation";

export function TokenTrend({ view }: { readonly view: UsageView }) {
  const id = useId();
  const { ceiling, segments } = trendGeometry(view.daily);
  const hasAbsence = view.daily.some((day) => typeof day.total !== "number");
  return (
    <section
      className="usage-efficiency__surface usage-efficiency__trend"
      aria-labelledby={`${id}-heading`}
    >
      <div className="usage-efficiency__section-heading">
        <div>
          <h3 id={`${id}-heading`}>Token activity over time</h3>
          <p>Fictional daily totals · UTC</p>
        </div>
        <span className="usage-efficiency__badge">{view.days} days</span>
      </div>
      <p className="usage-efficiency__chart-key">
        <span aria-hidden="true" />
        Input + output
      </p>
      {view.daily.length > 0 && segments.length > 0 ? (
        <svg viewBox="0 0 700 250" role="img" aria-labelledby={`${id}-title ${id}-description`}>
          <title id={`${id}-title`}>Daily sample token activity</title>
          <desc id={`${id}-description`}>
            Each point is a supplied daily total. Unknown values are gaps. Exact dates and values
            are in Daily token details below.
          </desc>
          <defs>
            <linearGradient id={`${id}-fill`} x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="currentColor" stopOpacity="0.32" />
              <stop offset="100%" stopColor="currentColor" stopOpacity="0.02" />
            </linearGradient>
          </defs>
          {[40, 125, 210].map((y) => (
            <line key={y} x1="40" x2="660" y1={y} y2={y} className="usage-efficiency__gridline" />
          ))}
          <text x="40" y="23" className="usage-efficiency__axis">
            {displayMetric(ceiling)} tokens
          </text>
          <text x="24" y="214" className="usage-efficiency__axis">
            0
          </text>
          {segments.map((points, index) => {
            const first = points[0];
            const last = points.at(-1);
            if (!first || !last) return null;
            const line = points
              .map((point, i) => `${i === 0 ? "M" : "L"}${String(point.x)},${String(point.y)}`)
              .join(" ");
            return (
              <g key={index}>
                <path
                  d={`${line} L${String(last.x)},210 L${String(first.x)},210 Z`}
                  fill={`url(#${id}-fill)`}
                />
                <path
                  d={line}
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2.5"
                  strokeLinejoin="round"
                />
                {points.map((point) => (
                  <circle key={point.date} cx={point.x} cy={point.y} r="3" fill="currentColor">
                    <title>
                      {point.date}: {displayMetric(point.total)} tokens
                    </title>
                  </circle>
                ))}
              </g>
            );
          })}
          <text x="40" y="242" className="usage-efficiency__axis">
            {view.daily[0]?.date}
          </text>
          <text x="660" y="242" textAnchor="end" className="usage-efficiency__axis">
            {view.daily.at(-1)?.date}
          </text>
        </svg>
      ) : (
        <div className="usage-efficiency__chart-empty">
          {view.daily.length === 0
            ? "No daily records in this empty example."
            : "Daily totals are not available to plot."}
        </div>
      )}
      {hasAbsence && <p>Unknown daily totals appear as gaps, never as zero.</p>}
      <p className="usage-efficiency__footnote">
        Sample activity only · No live source connected to this view
      </p>
    </section>
  );
}

export function TokenComposition({ view }: { readonly view: UsageView }) {
  const id = useId();
  const [input, cached, output, reasoning] = view.tokens;
  const complete =
    typeof input === "number" && typeof output === "number" && typeof view.total === "number";
  const share =
    complete && typeof input === "number" && typeof view.total === "number" && view.total > 0
      ? (input / view.total) * 100
      : 0;
  return (
    <section
      className="usage-efficiency__surface usage-efficiency__composition"
      aria-labelledby={`${id}-heading`}
    >
      <div className="usage-efficiency__section-heading">
        <div>
          <h3 id={`${id}-heading`}>Token breakdown</h3>
          <p>One sample connection · no overlapping totals</p>
        </div>
      </div>
      <div className="usage-efficiency__composition-body">
        <div className="usage-efficiency__donut">
          <svg viewBox="0 0 120 120" aria-hidden="true">
            <circle
              cx="60"
              cy="60"
              r="48"
              fill="none"
              className="usage-efficiency__ring-track"
              strokeWidth="13"
            />
            {complete && typeof view.total === "number" && view.total > 0 && (
              <>
                <circle
                  cx="60"
                  cy="60"
                  r="48"
                  fill="none"
                  className="usage-efficiency__output-ring"
                  strokeWidth="13"
                />
                <circle
                  cx="60"
                  cy="60"
                  r="48"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="13"
                  pathLength="100"
                  strokeDasharray={`${String(share)} 100`}
                  transform="rotate(-90 60 60)"
                />
              </>
            )}
          </svg>
          <div>
            <strong>{displayMetric(view.total)}</strong>
            <span>total tokens</span>
          </div>
        </div>
        <dl className="usage-efficiency__breakdown">
          <div>
            <dt>
              <i className="usage-efficiency__input-dot" aria-hidden="true" />
              Input
            </dt>
            <dd>{displayMetric(input)}</dd>
          </div>
          <div>
            <dt>
              <i className="usage-efficiency__output-dot" aria-hidden="true" />
              Output
            </dt>
            <dd>{displayMetric(output)}</dd>
          </div>
          <div>
            <dt>
              Cached input tokens <small>Included in input</small>
            </dt>
            <dd>{displayMetric(cached)}</dd>
          </div>
          <div>
            <dt>
              Reasoning tokens <small>Included in output</small>
            </dt>
            <dd>{displayMetric(reasoning)}</dd>
          </div>
        </dl>
      </div>
      {!complete && <p>Breakdown incomplete — no proportional segments shown.</p>}
    </section>
  );
}
