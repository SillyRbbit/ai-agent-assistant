import { Activity, GitBranch, Search } from "lucide-react";

import { COMMAND_CENTER_DISCLOSURE } from "../commandCenterProjection";

export interface ActivityViewEvent {
  readonly agentLabel: string | null;
  readonly demoOrigin: string;
  readonly detail: string;
  readonly id: string;
  readonly kind: string;
  readonly ordinal: number;
  readonly redaction: string;
  readonly relatedLabels: readonly string[];
  readonly severity: string;
  readonly summary: string;
  readonly taskLabel: string | null;
  readonly timestamp: string;
  readonly workflowLabel: string | null;
}

interface CommandCenterActivityStreamProps {
  readonly eventKind: string;
  readonly eventKinds: readonly string[];
  readonly events: readonly ActivityViewEvent[];
  readonly followPathDisabled: boolean;
  readonly followSelectedPath: boolean;
  readonly onEventKindChange: (value: string) => void;
  readonly onFollowSelectedPathChange: (value: boolean) => void;
  readonly onSearchChange: (value: string) => void;
  readonly onSeverityChange: (value: string) => void;
  readonly search: string;
  readonly severity: string;
}

function readable(value: string): string {
  return value.replaceAll("-", " ");
}

export function CommandCenterActivityStream({
  eventKind,
  eventKinds,
  events,
  followPathDisabled,
  followSelectedPath,
  onEventKindChange,
  onFollowSelectedPathChange,
  onSearchChange,
  onSeverityChange,
  search,
  severity,
}: CommandCenterActivityStreamProps) {
  return (
    <section
      aria-labelledby="command-center-activity-title"
      className="command-center-activity"
      data-scroll-region="command-center-activity"
    >
      <header className="command-center-activity__header">
        <div>
          <h2 id="command-center-activity-title">Structured fixture activity</h2>
          <p>
            Bounded, pre-redacted presentation records. Not authoritative audit. ·{" "}
            {COMMAND_CENTER_DISCLOSURE}
          </p>
        </div>
        <Activity aria-hidden="true" size={18} />
      </header>

      <div className="command-center-activity__controls">
        <label className="command-center-field command-center-field--search">
          <span>Filter activity</span>
          <span className="command-center-search">
            <Search aria-hidden="true" />
            <input
              className="command-center-field__control"
              onChange={(event) => {
                onSearchChange(event.target.value);
              }}
              placeholder="Summary, agent, or workflow"
              type="search"
              value={search}
            />
          </span>
        </label>
        <label className="command-center-field">
          <span>Severity</span>
          <select
            className="command-center-field__control"
            onChange={(event) => {
              onSeverityChange(event.target.value);
            }}
            value={severity}
          >
            <option value="all">All severities</option>
            <option value="info">Info</option>
            <option value="success">Success</option>
            <option value="warning">Warning</option>
            <option value="danger">Error</option>
          </select>
        </label>
        <label className="command-center-field">
          <span>Event kind</span>
          <select
            className="command-center-field__control"
            onChange={(event) => {
              onEventKindChange(event.target.value);
            }}
            value={eventKind}
          >
            <option value="all">All event kinds</option>
            {eventKinds.map((kind) => (
              <option key={kind} value={kind}>
                {readable(kind)}
              </option>
            ))}
          </select>
        </label>
        <button
          aria-pressed={followSelectedPath}
          className="command-center-button"
          disabled={followPathDisabled}
          onClick={() => {
            onFollowSelectedPathChange(!followSelectedPath);
          }}
          type="button"
        >
          <GitBranch aria-hidden="true" /> Follow selected path
        </button>
      </div>

      <div className="command-center-activity__body">
        {events.length === 0 ? (
          <div className="command-center-empty" role="status">
            <Activity aria-hidden="true" />
            <h3>No matching fixture activity</h3>
            <p>Adjust the local activity filters to show deterministic records.</p>
          </div>
        ) : (
          <ol
            aria-label="Simulated command center activity"
            className="command-center-activity__list"
          >
            {events.map((event) => (
              <li className="command-center-event" key={event.id}>
                <span
                  className="command-center-event__ordinal"
                  aria-label={`Step ${String(event.ordinal).padStart(2, "0")}`}
                >
                  {String(event.ordinal).padStart(2, "0")}
                </span>
                <div className="command-center-event__copy">
                  <strong>{event.summary}</strong>
                  <small>
                    {readable(event.kind)} · {event.agentLabel ?? "Application boundary"}
                    {event.workflowLabel === null ? "" : ` · ${event.workflowLabel}`}
                  </small>
                  <details>
                    <summary>View bounded detail</summary>
                    <p>{event.detail}</p>
                    <dl className="command-center-event__facts">
                      <div>
                        <dt>Fixture event ID</dt>
                        <dd>{event.id}</dd>
                      </div>
                      <div>
                        <dt>Task</dt>
                        <dd>{event.taskLabel ?? "Unavailable in fixture"}</dd>
                      </div>
                      <div>
                        <dt>Workflow</dt>
                        <dd>{event.workflowLabel ?? "Unavailable in fixture"}</dd>
                      </div>
                      <div>
                        <dt>Related path</dt>
                        <dd>
                          {event.relatedLabels.length === 0
                            ? "Unavailable in fixture"
                            : event.relatedLabels.join(" → ")}
                        </dd>
                      </div>
                      <div>
                        <dt>Origin and redaction</dt>
                        <dd>
                          {readable(event.demoOrigin)} · {readable(event.redaction)} ·{" "}
                          {readable(event.severity)}
                        </dd>
                      </div>
                    </dl>
                  </details>
                </div>
                <time dateTime={event.timestamp}>SIMULATED TIME · {event.timestamp}</time>
              </li>
            ))}
          </ol>
        )}
      </div>
    </section>
  );
}
