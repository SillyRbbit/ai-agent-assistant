import {
  Activity,
  AlertTriangle,
  Bot,
  BriefcaseBusiness,
  CheckCircle2,
  Clock3,
  ShieldAlert,
} from "lucide-react";

import type {
  CommandCenterAttentionItem,
  CommandCenterOverviewViewModel,
  CommandCenterWorkItem,
} from "../commandCenterOverview";
import {
  commandCenterEventSeverityLabel,
  COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
} from "../commandCenterEventPresentation";
import type { CommandCenterEventId, TopologyNodeId } from "../commandCenterProjection";

import { SystemStatusSummary } from "./SystemStatusSummary";

interface CommandCenterOverviewProps {
  readonly model: CommandCenterOverviewViewModel;
  readonly onSelect: (id: TopologyNodeId) => void;
  readonly onSelectEvent: (id: CommandCenterEventId) => void;
  readonly selectedEventId: CommandCenterEventId | null;
  readonly selectedId: string | null;
}

function readable(value: string): string {
  return value
    .split("-")
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join(" ");
}

function workKindLabel(item: CommandCenterWorkItem): string {
  switch (item.presentationKind) {
    case "task":
      return "Task";
    case "workflow":
      return item.entityKind === "workflow" ? "Workflow" : "Workflow step";
    case "checkpoint":
      return readable(item.entityKind);
  }
}

function attentionLabel(item: CommandCenterAttentionItem): string {
  switch (item.category) {
    case "approval":
      return "Simulated approval required";
    case "blocked":
      return "Simulated blocked work";
    case "cancelled":
      return "Simulated cancellation";
    case "failure":
      return "Simulated failure";
    case "unresolved":
      return "Simulated unresolved issue";
  }
}

export function CommandCenterOverview({
  model,
  onSelect,
  onSelectEvent,
  selectedEventId,
  selectedId,
}: CommandCenterOverviewProps) {
  return (
    <div className="command-center-overview">
      <SystemStatusSummary status={model.system} />

      <div className="command-center-overview__grid">
        <section
          aria-labelledby="command-center-active-work-title"
          className="command-center-overview-section command-center-overview-section--active"
        >
          <header className="command-center-overview-section__header">
            <div>
              <p className="section-kicker">Primary queue · deterministic fixture</p>
              <h2 id="command-center-active-work-title">Active Work</h2>
            </div>
            <BriefcaseBusiness aria-hidden="true" />
          </header>

          {model.activeWork.length === 0 ? (
            <div className="command-center-overview-empty" role="status">
              <CheckCircle2 aria-hidden="true" />
              <div>
                <strong>No simulated active work</strong>
                <span>The selected deterministic scenario contains no active work item.</span>
              </div>
            </div>
          ) : (
            <ul aria-label="Active deterministic work" className="command-center-work-list">
              {model.activeWork.map((item) => (
                <li key={item.nodeId}>
                  <button
                    aria-label={`Inspect ${item.label}, ${workKindLabel(item)}, simulated status ${readable(item.simulatedStatus)}`}
                    aria-pressed={selectedId === item.nodeId}
                    className="command-center-overview-row command-center-work-row"
                    onClick={() => {
                      onSelect(item.nodeId);
                    }}
                    type="button"
                  >
                    <span className="command-center-overview-row__topline">
                      <span>{workKindLabel(item)}</span>
                      <span
                        className={`command-center-status command-center-status--${item.simulatedStatus}`}
                      >
                        Simulated · {readable(item.simulatedStatus)}
                      </span>
                    </span>
                    <strong>{item.label}</strong>
                    <span>Owner · {item.ownerLabel}</span>
                    <span>Stage · Unavailable in this projection</span>
                    <span>
                      Last deterministic activity · {item.lastActivity?.summary ?? "Unavailable"}
                    </span>
                    {item.dependencyLabels.length === 0 ? null : (
                      <span>Dependencies · {item.dependencyLabels.join(", ")}</span>
                    )}
                    {item.blockedReason === null ? null : (
                      <span className="command-center-overview-row__attention">
                        Attention · {item.blockedReason}
                      </span>
                    )}
                  </button>
                </li>
              ))}
            </ul>
          )}
        </section>

        <section
          aria-labelledby="command-center-attention-title"
          className="command-center-overview-section command-center-overview-section--attention"
        >
          <header className="command-center-overview-section__header">
            <div>
              <p className="section-kicker">Decision queue · simulated</p>
              <h2 id="command-center-attention-title">Approvals &amp; Attention</h2>
            </div>
            <ShieldAlert aria-hidden="true" />
          </header>

          {model.attention.length === 0 ? (
            <div className="command-center-overview-empty" role="status">
              <CheckCircle2 aria-hidden="true" />
              <div>
                <strong>No simulated attention item</strong>
                <span>No approval, block, failure, or cancellation exists in this fixture.</span>
              </div>
            </div>
          ) : (
            <ul
              aria-label="Simulated approvals and attention"
              className="command-center-attention-list"
            >
              {model.attention.map((item) => (
                <li key={item.nodeId}>
                  <button
                    aria-label={`Inspect ${item.label}, ${attentionLabel(item)}`}
                    aria-pressed={selectedId === item.nodeId}
                    className={`command-center-overview-row command-center-attention-row command-center-attention-row--${item.category}`}
                    onClick={() => {
                      onSelect(item.nodeId);
                    }}
                    type="button"
                  >
                    <span className="command-center-overview-row__topline">
                      <span>{attentionLabel(item)}</span>
                      <span>{readable(item.simulatedStatus)}</span>
                    </span>
                    <strong>{item.label}</strong>
                    <span>{item.detail}</span>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </section>

        <section
          aria-labelledby="command-center-agent-roster-title"
          className="command-center-overview-section command-center-overview-section--roster"
        >
          <header className="command-center-overview-section__header">
            <div>
              <p className="section-kicker">Definitions · deterministic fixture representation</p>
              <h2 id="command-center-agent-roster-title">Canonical Agent Roster</h2>
            </div>
            <span className="command-center-overview-section__count">
              {model.agents.length} roles
            </span>
          </header>

          <ul aria-label="Canonical agent roster" className="command-center-agent-roster">
            {model.agents.map((agent) => (
              <li key={agent.agentId}>
                <button
                  aria-label={`Inspect ${agent.label}, canonical agent definition, simulated status ${readable(agent.simulatedStatus)}`}
                  aria-pressed={selectedId === agent.nodeId}
                  className="command-center-agent-row"
                  data-agent-id={agent.agentId}
                  onClick={() => {
                    onSelect(agent.nodeId);
                  }}
                  title={agent.responsibility}
                  type="button"
                >
                  <span className="command-center-agent-row__heading">
                    <Bot aria-hidden="true" />
                    <strong>{agent.label}</strong>
                  </span>
                  <span>{agent.domainLabel} · canonical definition</span>
                  <span>{agent.responsibility}</span>
                  <span>Simulated status · {readable(agent.simulatedStatus)}</span>
                  <span>Current fixture work · {agent.currentWorkLabel ?? "None"}</span>
                  <span>
                    Availability · {readable(agent.availability)} · Health ·{" "}
                    {readable(agent.health)}
                  </span>
                  <span>Tool access · {readable(agent.toolAccess)}</span>
                  <span>Last deterministic activity · {agent.lastActivity?.summary ?? "None"}</span>
                </button>
              </li>
            ))}
          </ul>
        </section>

        <section
          aria-labelledby="command-center-recent-activity-title"
          className="command-center-overview-section command-center-overview-section--recent"
        >
          <header className="command-center-overview-section__header">
            <div>
              <p className="section-kicker">Meaningful events · deterministic</p>
              <h2 id="command-center-recent-activity-title">Recent Activity</h2>
            </div>
            <Activity aria-hidden="true" />
          </header>

          {model.recentActivity.length === 0 ? (
            <div className="command-center-overview-empty" role="status">
              <Clock3 aria-hidden="true" />
              <div>
                <strong>No deterministic activity</strong>
                <span>The selected fixture contains no presentation event.</span>
              </div>
            </div>
          ) : (
            <ol aria-label="Recent deterministic activity" className="command-center-recent-list">
              {model.recentActivity.map((event) => {
                const taskLabel = event.taskLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
                const workflowLabel = event.workflowLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
                const relatedLabels =
                  event.relatedEntityLabels.length === 0
                    ? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE
                    : event.relatedEntityLabels.join(", ");
                const content = (
                  <>
                    <span className="command-center-overview-row__topline">
                      <span>Deterministic event · {readable(event.eventKind)}</span>
                      <span
                        className={`command-center-event-severity command-center-event-severity--${event.severity}`}
                      >
                        {commandCenterEventSeverityLabel(event.severity)}
                      </span>
                    </span>
                    <strong>{event.summary}</strong>
                    <span>
                      Associated agent ·{" "}
                      {event.associatedAgentLabel ?? COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE}
                    </span>
                    <span>Task · {taskLabel}</span>
                    <span>Workflow · {workflowLabel}</span>
                    <span>Related entities · {relatedLabels}</span>
                    <time dateTime={event.simulatedAt}>
                      {event.timeLabel} · {event.simulatedAt}
                    </time>
                  </>
                );

                return (
                  <li key={event.eventId}>
                    <button
                      aria-label={`Inspect deterministic event ${event.summary}`}
                      aria-pressed={selectedEventId === event.eventId}
                      className="command-center-overview-row command-center-recent-row"
                      onClick={() => {
                        onSelectEvent(event.eventId);
                      }}
                      type="button"
                    >
                      {content}
                    </button>
                  </li>
                );
              })}
            </ol>
          )}
        </section>
      </div>

      <p className="command-center-overview__boundary">
        <AlertTriangle aria-hidden="true" /> Runtime instances, tool execution, provider health,
        costs, and elapsed time are unavailable in this deterministic projection.
      </p>
    </div>
  );
}
