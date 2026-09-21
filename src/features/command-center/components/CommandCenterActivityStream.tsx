import { Activity, GitBranch, Search } from "lucide-react";

import {
  commandCenterEventSeverityLabel,
  COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
  type CommandCenterEventPresentation,
} from "../commandCenterEventPresentation";
import { COMMAND_CENTER_DISCLOSURE } from "../commandCenterProjection";

export type ActivityViewEvent = CommandCenterEventPresentation;

interface CommandCenterActivityStreamProps {
  readonly eventKind: string;
  readonly eventKinds: readonly string[];
  readonly events: readonly ActivityViewEvent[];
  readonly followPathDisabled: boolean;
  readonly followSelectedPath: boolean;
  readonly onEventKindChange: (value: string) => void;
  readonly onFollowSelectedPathChange: (value: boolean) => void;
  readonly onSearchChange: (value: string) => void;
  readonly onSelectEvent: (id: string) => void;
  readonly onSeverityChange: (value: string) => void;
  readonly search: string;
  readonly selectedEventId: string | null;
  readonly severities: readonly string[];
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
  onSelectEvent,
  onSeverityChange,
  search,
  selectedEventId,
  severities,
  severity,
}: CommandCenterActivityStreamProps) {
  return (
    <section
      aria-label="Deterministic fixture activity"
      className="command-center-activity command-center-activity--workspace"
      data-scroll-region="command-center-activity"
    >
      <p className="command-center-activity__workspace-provenance">
        Bounded, pre-redacted deterministic fixture records. Not authoritative audit. ·{" "}
        {COMMAND_CENTER_DISCLOSURE}
      </p>

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
            {severities.map((value) => (
              <option key={value} value={value}>
                {commandCenterEventSeverityLabel(value)}
              </option>
            ))}
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
            {events.map((event) => {
              const associatedAgent =
                event.associatedAgentLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
              const task = event.taskLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
              const workflow = event.workflowLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
              const relatedEntities =
                event.relatedEntityLabels.length === 0
                  ? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE
                  : event.relatedEntityLabels.join(" → ");
              const eventContents = (
                <>
                  <span
                    className="command-center-event__ordinal"
                    aria-label={`Step ${String(event.ordinal).padStart(2, "0")}`}
                  >
                    {String(event.ordinal).padStart(2, "0")}
                  </span>
                  <span className="command-center-event__copy">
                    <strong>{event.summary}</strong>
                    <small>{readable(event.eventKind)} · deterministic fixture event</small>
                    <span className="command-center-event__workspace-summary">
                      <span>
                        <b>Source</b>
                        <span>{event.sourceLabel}</span>
                      </span>
                      <span>
                        <b>Action</b>
                        <span>{readable(event.eventKind)}</span>
                      </span>
                      <span>
                        <b>Target</b>
                        <span>{event.targetLabel}</span>
                      </span>
                      <span>
                        <b>Severity</b>
                        <span>{commandCenterEventSeverityLabel(event.severity)}</span>
                      </span>
                      <span>
                        <b>Status</b>
                        <span>{event.statusLabel}</span>
                      </span>
                      <span>
                        <b>Associated agent</b>
                        <span>{associatedAgent}</span>
                      </span>
                      <span>
                        <b>Task</b>
                        <span>{task}</span>
                      </span>
                      <span>
                        <b>Workflow</b>
                        <span>{workflow}</span>
                      </span>
                      <span>
                        <b>Related entities</b>
                        <span>{relatedEntities}</span>
                      </span>
                    </span>
                    <span className="command-center-event__description">{event.description}</span>
                  </span>
                  <time dateTime={event.simulatedAt}>
                    {event.timeLabel} · {event.simulatedAt}
                  </time>
                </>
              );
              return (
                <li
                  className={`command-center-event${selectedEventId === event.eventId ? " command-center-event--selected" : ""}`}
                  key={event.eventId}
                >
                  <button
                    aria-label={`Inspect deterministic event ${event.summary}`}
                    aria-pressed={selectedEventId === event.eventId}
                    className="command-center-event__select"
                    onClick={() => {
                      onSelectEvent(event.eventId);
                    }}
                    type="button"
                  >
                    {eventContents}
                  </button>
                  <div className="command-center-event__detail">
                    <details>
                      <summary>View bounded detail</summary>
                      <dl className="command-center-event__facts">
                        <div>
                          <dt>Fixture event ID</dt>
                          <dd>{event.eventId}</dd>
                        </div>
                        <div>
                          <dt>Source</dt>
                          <dd>{event.sourceLabel}</dd>
                        </div>
                        <div>
                          <dt>Target</dt>
                          <dd>{event.targetLabel}</dd>
                        </div>
                        <div>
                          <dt>Status</dt>
                          <dd>{event.statusLabel}</dd>
                        </div>
                        <div>
                          <dt>Associated agent</dt>
                          <dd>{associatedAgent}</dd>
                        </div>
                        <div>
                          <dt>Task</dt>
                          <dd>{task}</dd>
                        </div>
                        <div>
                          <dt>Workflow</dt>
                          <dd>{workflow}</dd>
                        </div>
                        <div>
                          <dt>Related entities</dt>
                          <dd>{relatedEntities}</dd>
                        </div>
                        <div>
                          <dt>Origin and redaction</dt>
                          <dd>
                            {readable(event.demoOrigin)} · {readable(event.redaction)} ·{" "}
                            {commandCenterEventSeverityLabel(event.severity)}
                          </dd>
                        </div>
                      </dl>
                    </details>
                  </div>
                </li>
              );
            })}
          </ol>
        )}
      </div>
    </section>
  );
}
