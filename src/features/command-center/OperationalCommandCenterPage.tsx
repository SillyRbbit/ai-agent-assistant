import { ConductorIdentity } from "../agents/ConductorIdentity";
import { BotAvatar } from "../agents/BotAppearance";
import { SourceEvidence } from "../knowledge/KnowledgePage";
import type { Viewport } from "@xyflow/react";
import { AGENT_IDS } from "../../infrastructure/tauri/agent-chat-client";
import { COMMAND_CENTER_GROUP_IDS } from "./commandCenterProjection";
import { useEffect, useMemo, useState } from "react";
import { createPortal } from "react-dom";
import type { CollaborationLocation } from "../../application/state";
import { useApplicationWorkspacePanels } from "../../components/applicationWorkspacePanels";
import {
  agentChatClient,
  agentChatErrorMessage,
  type AgentProfile,
} from "../../infrastructure/tauri/agent-chat-client";
import {
  collaborationClient,
  type CollaborationClient,
} from "../../infrastructure/tauri/collaboration-client";
import type { ResearchKnowledgeDemoProjectionLoader } from "../../infrastructure/tauri/research-knowledge-demo-projection-client";
import { useCollaborationSnapshots } from "../collaboration/collaborationSnapshots";
import { projectCollaboration, matchesCard } from "./collaborationProjection";
import { CollaborationTopology } from "./CollaborationTopology";
import DemoCommandCenterPage from "./CommandCenterPage";
import "./operational-command-center.css";
type Mode = "roster" | "run" | "history" | "demo";
const makeSession = () => ({
  mode: "roster" as Mode,
  roomId: "",
  runId: "",
  selected: null as string | null,
  search: "",
  status: "all",
  kind: "all",
  group: "all",
  participant: "all",
  location: null as CollaborationLocation | null,
  viewports: new Map<string, Viewport>(),
});
type Session = ReturnType<typeof makeSession>;
const sessions = new WeakMap<CollaborationClient, Session>();
function sessionFor(client: CollaborationClient): Session {
  let value = sessions.get(client);
  if (!value) {
    value = makeSession();
    sessions.set(client, value);
  }
  return value;
}
export default function OperationalCommandCenterPage({
  projectionLoader,
  location,
  onOpenRoom,
  client = collaborationClient,
  loadProfiles = agentChatClient.list,
}: {
  readonly projectionLoader?: ResearchKnowledgeDemoProjectionLoader;
  readonly location?: CollaborationLocation | null;
  readonly onOpenRoom?: (location: CollaborationLocation) => void;
  readonly client?: CollaborationClient;
  readonly loadProfiles?: () => Promise<readonly AgentProfile[]>;
}) {
  const session = sessionFor(client);
  // Consume a navigation intent once. Reentry must not replay an older room link.
  if (location && location !== session.location) {
    Object.assign(session, {
      location,
      mode: "run",
      roomId: String(location.roomId),
      runId: location.runId,
      selected: location.stageId,
    });
  }
  const [mode, setMode] = useState<Mode>(session.mode);
  return (
    <div className="operational-workspace">
      <nav className="operational-modes" aria-label="Command Center data mode">
        {(
          [
            ["roster", "Current bot roster"],
            ["run", "Selected collaboration run"],
            ["history", "Historical room runs"],
            ["demo", "Deterministic demonstrations"],
          ] as const
        ).map(([id, label]) => (
          <button
            key={id}
            aria-pressed={mode === id}
            onClick={() => {
              session.mode = id;
              setMode(id);
            }}
          >
            {label}
          </button>
        ))}
      </nav>
      {mode === "demo" ? (
        <DemoCommandCenterPage {...(projectionLoader ? { projectionLoader } : {})} />
      ) : (
        <OperationalWorkspace
          mode={mode}
          session={session}
          onOpenRoom={onOpenRoom}
          client={client}
          loadProfiles={loadProfiles}
        />
      )}
    </div>
  );
}
function OperationalWorkspace({
  mode,
  session,
  onOpenRoom,
  client,
  loadProfiles,
}: {
  readonly mode: Exclude<Mode, "demo">;
  readonly session: Session;
  readonly onOpenRoom: ((location: CollaborationLocation) => void) | undefined;
  readonly client: CollaborationClient;
  readonly loadProfiles: () => Promise<readonly AgentProfile[]>;
}) {
  const { rooms, status: snapshotStatus, store } = useCollaborationSnapshots(client);
  const [profiles, setProfiles] = useState<readonly AgentProfile[]>([]),
    [profileError, setProfileError] = useState(false);
  const [roomId, setRoomId] = useState(session.roomId),
    [runId, setRunId] = useState(session.runId),
    [selected, setSelected] = useState<string | null>(session.selected),
    [search, setSearch] = useState(session.search),
    [status, setStatus] = useState(session.status),
    [kind, setKind] = useState(session.kind),
    [group, setGroup] = useState(session.group),
    [participant, setParticipant] = useState(session.participant);
  const filters = { search, status, kind, group, participant };
  const panels = useApplicationWorkspacePanels();
  useEffect(() => {
    let alive = true;
    void loadProfiles()
      .then((p) => {
        if (alive) setProfiles(p);
      })
      .catch(() => {
        if (alive) setProfileError(true);
      });
    return () => {
      alive = false;
    };
  }, [loadProfiles]);
  useEffect(() => {
    Object.assign(session, { roomId, runId, selected, search, status, kind, group, participant });
  }, [roomId, runId, selected, search, status, kind, group, participant, session]);
  const room = rooms.find((r) => String(r.id) === roomId),
    run = mode === "roster" ? null : (room?.runs.find((r) => r.id === runId) ?? null);
  const projection = useMemo(() => projectCollaboration(profiles, run), [profiles, run]);
  const missing = mode !== "roster" && !run;
  const card = missing ? undefined : projection.cards.find((c) => c.id === selected);
  function select(id: string | null) {
    setSelected(id);
    if (id) panels?.openInspector();
  }
  const inspector = (
    <div className="operational-inspector">
      {card ? (
        <>
          {card.kind === "coordinator" && (
            <ConductorIdentity
              run={run}
              current={mode === "run" && snapshotStatus === "current"}
              expressive
            />
          )}
          {card.identity && <BotAvatar identity={card.identity} agentId={card.agentId} />}
          <h3>{card.label}</h3>
          <p>{card.role}</p>
          <p>{card.description}</p>
          <dl>
            <dt>Stable identity</dt>
            <dd>{card.id}</dd>
            <dt>Canonical bot</dt>
            <dd>{card.agentId ?? "Not a bot"}</dd>
            <dt>Attribution</dt>
            <dd>{card.attribution}</dd>
            <dt>Status</dt>
            <dd>{card.status}</dd>
            <dt>Saved settings</dt>
            <dd>{card.settings}</dd>
          </dl>
          {card.timestamp !== null && (
            <p>
              Recorded preparation/start timestamp (not completion):{" "}
              <time>{new Date(card.timestamp).toLocaleString()}</time>. No duration is available.
            </p>
          )}
          {card.provisional && (
            <>
              <h4>Provisional output — not a validated result</h4>
              <pre>{card.provisional}</pre>
            </>
          )}
          {card.summary && (
            <>
              <h4>Schema-validated output summary</h4>
              <p>{card.summary}</p>
              <p>Schema validation does not establish factual accuracy.</p>
            </>
          )}
          <p>Source references: {card.sources.join(", ") || "none recorded"}</p>
          {run && <SourceEvidence sources={run.input.sources} labels={card.sources} />}
          <ul>
            {card.limitations.map((l, i) => (
              <li key={i}>{l}</li>
            ))}
          </ul>
          {run && room && onOpenRoom && (
            <button
              onClick={() => {
                onOpenRoom({
                  roomId: room.id,
                  runId: run.id,
                  stageId: card.kind === "stage" ? card.id : null,
                });
              }}
            >
              Open in room
            </button>
          )}
        </>
      ) : (
        <p>
          {selected
            ? "Selection is unavailable for this view."
            : "Select a bot, stage or Conductor."}
        </p>
      )}
    </div>
  );
  const activity = (
    <div className="operational-activity">
      <p>
        Snapshot stage summary, not an event journal. Transitions between polls are not observed
        history.
      </p>
      {run ? (
        <ol>
          {run.stages.map((s, i) => (
            <li key={s.id}>
              <button
                onClick={() => {
                  select(s.id);
                }}
              >
                Stage {i + 1} · {s.participant.identity.nickname || s.participant.role} · {s.status}
              </button>
              {s.handoff
                ? " · schema-validated result"
                : s.provisional
                  ? " · provisional text only"
                  : " · no result"}
            </li>
          ))}
        </ol>
      ) : (
        <p>No run selected; no execution activity claimed.</p>
      )}
    </div>
  );
  return (
    <section className="operational-center" aria-label="Operational Command Center">
      <header>
        <p className="section-kicker">Read-only operational projection</p>
        <h1>Command Center</h1>
        <p>
          Nine bots · Conductor is the application coordinator. Start, Stop and approval remain in
          the room.
        </p>
      </header>
      <div className="operational-controls">
        <label>
          Room
          <select
            value={roomId}
            onChange={(e) => {
              setRoomId(e.currentTarget.value);
              setRunId("");
              setSelected(null);
            }}
          >
            <option value="">Select room</option>
            {rooms.map((r) => (
              <option key={r.id} value={r.id}>
                {r.title}
              </option>
            ))}
          </select>
        </label>
        <label>
          {mode === "history" ? "Historical run" : "Run"}
          <select
            value={runId}
            onChange={(e) => {
              setRunId(e.currentTarget.value);
              setSelected(null);
            }}
          >
            <option value="">Select run</option>
            {room?.runs.map((r) => (
              <option key={r.id} value={r.id}>
                {r.id} · {r.input.workflow} · {r.status}
              </option>
            ))}
          </select>
        </label>
        <button
          onClick={() => {
            void store.refresh();
          }}
        >
          Refresh snapshots
        </button>
        {run && room && onOpenRoom && (
          <button
            onClick={() => {
              onOpenRoom({ roomId: room.id, runId: run.id, stageId: null });
            }}
          >
            Open selected run in room
          </button>
        )}
      </div>
      <p role="status">
        Snapshots: {snapshotStatus}. {mode === "history" ? "Historical inspection. " : ""}
        {missing ? "No selected run data" : projection.provenance}.{" "}
        {snapshotStatus === "stale" ? "Last accepted data; current state unknown." : ""}
      </p>
      {profileError && (
        <p role="status">Current profiles unavailable. Saved run identities are not replaced.</p>
      )}
      {run && (
        <p>
          {run.id} · {run.input.workflow} · {run.status} · sequence {run.sequence}. Current stage:{" "}
          {run.stages.find((s) => s.status === "running")?.id ?? "none running"}
        </p>
      )}
      {projection.error && <p role="alert">{agentChatErrorMessage(projection.error)}</p>}
      <div className="operational-controls">
        <label>
          Search nickname or canonical role
          <input
            value={search}
            onChange={(e) => {
              setSearch(e.currentTarget.value);
            }}
          />
        </label>
        <label>
          Entities
          <select
            value={kind}
            onChange={(e) => {
              setKind(e.currentTarget.value);
            }}
          >
            {["all", "coordinator", "bot", "stage"].map((x) => (
              <option key={x}>{x}</option>
            ))}
          </select>
        </label>
        <label>
          Status / participation
          <select
            value={status}
            onChange={(e) => {
              setStatus(e.currentTarget.value);
            }}
          >
            {[
              "all",
              "roster",
              "participant",
              "inactive",
              "queued",
              "running",
              "completed",
              "partial",
              "failed",
              "cancelled",
              "interrupted",
            ].map((x) => (
              <option key={x}>{x}</option>
            ))}
          </select>
        </label>
        <label>
          Domain
          <select
            value={group}
            onChange={(e) => {
              setGroup(e.currentTarget.value);
            }}
          >
            <option value="all">all</option>
            {COMMAND_CENTER_GROUP_IDS.map((id) => (
              <option key={id}>{id}</option>
            ))}
          </select>
        </label>
        <label>
          Participant
          <select
            value={participant}
            onChange={(e) => {
              setParticipant(e.currentTarget.value);
            }}
          >
            <option value="all">all</option>
            {AGENT_IDS.map((id) => (
              <option key={id} value={id}>
                {projection.cards.find((c) => c.id === `bot:${id}`)?.label ?? id}
              </option>
            ))}
          </select>
        </label>
        <button
          onClick={() => {
            setGroup("all");
            setParticipant("all");
            setSearch("");
            setStatus("all");
            setKind("all");
          }}
        >
          Clear filters
        </button>
      </div>
      {missing ? (
        <p role="status">
          Select an available room and run. A deleted room or missing run is never replaced with
          demonstration data.
        </p>
      ) : (
        <CollaborationTopology
          projection={projection}
          dataset={run?.id ?? "roster"}
          selected={selected}
          onSelect={select}
          filters={filters}
          viewports={session.viewports}
        />
      )}
      {card && !matchesCard(card, filters) && (
        <p role="status">
          Selected entity is filtered out.{" "}
          <button
            onClick={() => {
              select(null);
            }}
          >
            Clear selection
          </button>
        </p>
      )}
      {panels?.inspectorHeaderTarget &&
        createPortal(
          <div className="application-panel-header">
            <h2 id="application-inspector-title">Operational inspector</h2>
            <button
              aria-label="Close inspector"
              title="Close inspector"
              className="application-panel-header__button"
              data-application-inspector-close="true"
              onClick={panels.closeInspector}
            >
              ×
            </button>
          </div>,
          panels.inspectorHeaderTarget,
        )}
      {panels?.inspectorBodyTarget
        ? createPortal(inspector, panels.inspectorBodyTarget)
        : inspector}
      {panels?.activitySummaryTarget &&
        createPortal(
          <>
            {run ? `${run.input.workflow} · ${run.status}` : "No selected run"} · snapshot summary
          </>,
          panels.activitySummaryTarget,
        )}
      {panels?.activityBodyTarget ? createPortal(activity, panels.activityBodyTarget) : activity}
    </section>
  );
}
