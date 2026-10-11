import { ActionReview } from "./ActionReview";
import { actionError } from "../../infrastructure/tauri/collaboration-client";
import { ConductorIdentity } from "../agents/ConductorIdentity";
import { PageHeader } from "../shared/PageHeader";
import { SourcePicker } from "../knowledge/SourcePicker";
import { SourceEvidence, NoteEditor } from "../knowledge/KnowledgePage";
import { knowledgeClient, type Draft } from "../../infrastructure/tauri/knowledge-client";
import { useEffect, useRef, useState } from "react";
import {
  collaborationClient,
  WORKFLOWS,
  type CollaborationClient,
  type Workflow,
  type Source,
  type Preview,
  type SelectedRepository,
} from "../../infrastructure/tauri/collaboration-client";
import { BotAvatar } from "../agents/BotAppearance";
import { ROLE_DESCRIPTIONS } from "../agents/botAppearanceValues";
import { agentChatErrorMessage } from "../../infrastructure/tauri/agent-chat-client";
import "./CollaborationPage.css";
import { useCollaborationSnapshots } from "./collaborationSnapshots";
import type { CollaborationLocation } from "../../application/state";
export function CollaborationPage({
  client = collaborationClient,
  location,
  onOpenGraph,
}: {
  readonly client?: CollaborationClient;
  readonly location?: CollaborationLocation | null;
  readonly onOpenGraph?: (location: CollaborationLocation) => void;
}) {
  const { rooms, status: snapshotStatus, store } = useCollaborationSnapshots(client);
  const [selected, setSelected] = useState<number | null>(location?.roomId ?? null),
    [title, setTitle] = useState(""),
    [workflow, setWorkflow] = useState<Workflow>("research"),
    [objective, setObjective] = useState(""),
    [sources, setSources] = useState<readonly Source[]>([{ label: "source1", text: "" }]),
    [preview, setPreview] = useState<Preview | null>(null),
    [ack, setAck] = useState(false),
    [busy, setBusy] = useState(false),
    [error, setError] = useState("");
  const [target, setTarget] = useState<SelectedRepository | null>(null);
  const [editFile, setEditFile] = useState("solution.py");
  const [testFile, setTestFile] = useState("test_solution.py");
  const [librarySources, setLibrarySources] = useState<readonly Source[]>([]);
  const [draft, setDraft] = useState<Draft | null>(null);
  const [noteNotice, setNoteNotice] = useState("");
  const alive = useRef(true);
  const room = rooms.find((r) => r.id === selected);
  const active = rooms.some((r) =>
    r.runs.some((run) => run.status === "running" || run.status === "queued"),
  );
  useEffect(() => {
    alive.current = true;
    return () => {
      alive.current = false;
    };
  }, []);
  const focusedLocation = useRef<CollaborationLocation | null>(null);
  useEffect(() => {
    if (location?.stageId && location !== focusedLocation.current) {
      const target = document.getElementById(location.stageId);
      if (target) {
        target.focus();
        focusedLocation.current = location;
      }
    }
  }, [location, rooms]);
  async function action(fn: () => Promise<void>) {
    setBusy(true);
    setError("");
    store.invalidate();
    try {
      await fn();
      await store.refresh();
    } catch (e) {
      if (e === "stale_context") {
        invalidate();
        setError(
          "Saved settings or library sources changed. Detach outdated sources, select current versions, then prepare and acknowledge a new preview.",
        );
      } else {
        setError(
          e === "limit"
            ? "A room, run or source limit was reached. Narrow the source selection or use an available room; nothing was truncated."
            : typeof e === "string"
              ? (actionError(e) ?? agentChatErrorMessage(e))
              : "The bounded operation could not complete. Check saved connections and readiness; no automatic retry was made.",
        );
      }
    } finally {
      if (alive.current) setBusy(false);
    }
  }
  function invalidate() {
    setPreview(null);
    setAck(false);
  }
  return (
    <section className="collaboration" aria-labelledby="collaboration-title">
      <PageHeader
        eyebrow="Shared work"
        headingId="collaboration-title"
        title="Collaboration"
        description="Conductor · Application coordinator. Four analysis routes and one bounded, isolated Python change workflow with native owner approval."
      />
      <p className="room-retention">
        Submitted objectives, sources and visible results are stored locally (up to 10 rooms, 4 runs
        each). Encryption at rest is not claimed. Delete removes local history, not provider-held
        copies. Never submit credentials. Private bot notes and hidden reasoning are excluded.
      </p>
      {snapshotStatus !== "current" && (
        <p role="status">Room snapshots: {snapshotStatus}. No fixture fallback.</p>
      )}
      {location &&
        !rooms.some(
          (r) =>
            r.id === location.roomId &&
            r.runs.some(
              (run) =>
                run.id === location.runId &&
                (!location.stageId || run.stages.some((s) => s.id === location.stageId)),
            ),
        ) &&
        snapshotStatus === "current" && (
          <p role="status">The linked room, run or stage is no longer available.</p>
        )}
      {error && <p role="alert">{error}</p>}
      {noteNotice && <p role="status">{noteNotice}</p>}
      {draft && (
        <NoteEditor
          initial={draft}
          onSaved={() => {
            setNoteNotice("Draft saved to Knowledge. It is not automatically shared.");
            setDraft(null);
          }}
          onCancel={() => {
            setDraft(null);
          }}
        />
      )}
      <div className="room-layout">
        <aside aria-label="Room list">
          <h2>Rooms</h2>
          <form
            onSubmit={(e) => {
              e.preventDefault();
              void action(async () => {
                const created = await client.create(title);
                store.acceptRoom(created);
                setSelected(created.id);
                setTitle("");
                invalidate();
              });
            }}
          >
            <label>
              Room title
              <input
                maxLength={80}
                value={title}
                onChange={(e) => {
                  setTitle(e.currentTarget.value);
                }}
              />
            </label>
            <button disabled={busy || !title.trim()}>Create room</button>
          </form>
          {rooms.length === 0 && <p>No rooms yet. Create one to prepare a bounded workflow.</p>}
          {rooms.map((r) => (
            <button
              key={r.id}
              className="room-list-item"
              aria-pressed={selected === r.id}
              onClick={() => {
                setSelected(r.id);
                invalidate();
              }}
            >
              {r.title}
            </button>
          ))}
        </aside>
        <section className="room-workspace" aria-label="Selected collaboration room">
          {!room ? (
            <p>Select a room to prepare its objective and sources.</p>
          ) : (
            <>
              <h2>{room.title}</h2>
              <button
                disabled={busy}
                onClick={() =>
                  void action(async () => {
                    await client.delete(room.id);
                    setSelected(null);
                    store.remove(room.id);
                    invalidate();
                  })
                }
              >
                Delete room and local history
              </button>
              {room.runs.map((run) => (
                <section className="collaboration-run" key={run.id} aria-label={`Run ${run.id}`}>
                  <ConductorIdentity
                    run={run}
                    current={snapshotStatus === "current"}
                    expressive={run === room.runs.at(-1)}
                  />
                  <h3 className="room-run-title" data-status={run.status}>
                    {run.input.workflow} · {run.status}
                  </h3>
                  {onOpenGraph && (
                    <button
                      onClick={() => {
                        onOpenGraph({ roomId: room.id, runId: run.id, stageId: null });
                      }}
                    >
                      Inspect run in Command Center
                    </button>
                  )}
                  <p>{run.input.objective}</p>
                  {run.action && (
                    <ActionReview
                      evidence={run.action}
                      disabled={busy || active}
                      onReview={
                        client.reviewAction && run.action.reviewHash
                          ? () => {
                              void action(async () => {
                                if (client.reviewAction && run.action?.reviewHash)
                                  store.acceptRoom(
                                    await client.reviewAction(
                                      room.id,
                                      run.id,
                                      run.action.reviewHash,
                                    ),
                                  );
                              });
                            }
                          : undefined
                      }
                    />
                  )}
                  {run.error && (
                    <p role="alert">
                      {(run.action ? actionError(run.error) : undefined) ??
                        agentChatErrorMessage(run.error)}
                    </p>
                  )}
                  <ol className="room-stage-overview" aria-label={`Stages for ${run.id}`}>
                    {run.stages.map((stage, index) => (
                      <li key={stage.id}>
                        <BotAvatar
                          identity={stage.participant.identity}
                          agentId={stage.participant.agentId}
                        />
                        <span>
                          <strong>
                            {stage.participant.identity.nickname || stage.participant.role}
                          </strong>
                          <small>
                            Stage {index + 1} · {stage.status}
                          </small>
                        </span>
                      </li>
                    ))}
                  </ol>
                  {run.stages.map((s, i) => (
                    <article key={s.id} id={s.id} tabIndex={-1} className="room-message">
                      <header>
                        <BotAvatar
                          identity={s.participant.identity}
                          agentId={s.participant.agentId}
                        />
                        <div>
                          <strong>{s.participant.identity.nickname || s.participant.role}</strong>
                          <br />
                          {s.participant.role} · {s.participant.connection} · {s.participant.model}{" "}
                          · {s.participant.effort}
                          <p>{ROLE_DESCRIPTIONS[s.participant.agentId]}</p>
                        </div>
                      </header>
                      <p>
                        Stage {i + 1} · {s.status} ·{" "}
                        <time dateTime={new Date(s.timestamp).toISOString()}>
                          {new Date(s.timestamp).toLocaleString()}
                        </time>
                      </p>
                      {s.provisional && (
                        <div aria-live="polite">
                          <strong>Provisional output — not a validated handoff</strong>
                          <pre>{s.provisional}</pre>
                        </div>
                      )}
                      {s.handoff && (
                        <>
                          <h4>
                            {s.handoff.status === "partial"
                              ? "Partial handoff"
                              : i === run.stages.length - 1
                                ? "Final synthesis"
                                : "Validated handoff"}
                          </h4>
                          <p>{s.handoff.summary}</p>
                          <ul>
                            {s.handoff.findings.map((f, n) => (
                              <li key={n}>{f}</li>
                            ))}
                          </ul>
                          <SourceEvidence sources={run.input.sources} labels={s.handoff.evidence} />
                          <button
                            onClick={() =>
                              void action(async () => {
                                setDraft(await knowledgeClient.draft(room.id, run.id, s.id));
                              })
                            }
                          >
                            Save as knowledge note
                          </button>
                          <details>
                            <summary>Handoff details and transmitted input</summary>
                            <p>
                              Schema validated; factual accuracy is not established. Source
                              references: {s.handoff.evidence.join(", ") || "none"}
                            </p>
                            <ul>
                              {s.handoff.limitations.map((l, n) => (
                                <li key={n}>{l}</li>
                              ))}
                            </ul>
                            <pre>{s.input}</pre>
                          </details>
                        </>
                      )}
                    </article>
                  ))}
                  {run.status === "running" && (
                    <button
                      disabled={busy}
                      onClick={() =>
                        void action(async () => {
                          store.acceptRoom(await client.cancel(room.id));
                        })
                      }
                    >
                      Stop collaboration
                    </button>
                  )}
                  {run.status === "interrupted" && (
                    <p>
                      Interrupted on restart. No calls will replay. Prepare an explicit new run.
                    </p>
                  )}
                </section>
              ))}
              <p>
                Use Bots to save each participant’s connection and refresh its supported models when
                required. A route must use either all Simulation profiles or all live connections;
                mixed simulation/live routes are rejected, never switched silently.
              </p>
              <fieldset disabled={active || busy || room.runs.length >= 4}>
                <legend>New bounded run</legend>
                <label>
                  Workflow
                  <select
                    value={workflow}
                    onChange={(e) => {
                      setWorkflow(e.currentTarget.value as Workflow);
                      invalidate();
                    }}
                  >
                    {Object.keys(WORKFLOWS).map((w) => (
                      <option key={w} value={w}>
                        {w}
                      </option>
                    ))}
                  </select>
                </label>
                <p>
                  Route: {WORKFLOWS[workflow].join(" → ")}. Maximum{" "}
                  {workflow === "coding_action" ? 4 : WORKFLOWS[workflow].length} generation calls.
                </p>
                <label>
                  Objective
                  <textarea
                    maxLength={4096}
                    value={objective}
                    onChange={(e) => {
                      setObjective(e.currentTarget.value);
                      invalidate();
                    }}
                  />
                </label>
                {workflow === "coding_action" ? (
                  <section aria-label="Isolated change scope">
                    <p>
                      OpenAI API Coding → QA &amp; Validation only. One new root-level Python file
                      (must not already exist), one unchanged unittest file; at most one correction.
                      No arbitrary commands or dependencies. Clean ordinary Git clone, up to 32
                      small regular files; no links or subdirectories. Docker runs without network
                      or credentials. Repository content and diff are shared with the configured
                      provider.
                    </p>
                    <button
                      type="button"
                      disabled={!client.selectActionRepository}
                      onClick={() => {
                        void action(async () => {
                          if (client.selectActionRepository) {
                            setTarget(await client.selectActionRepository());
                            invalidate();
                          }
                        });
                      }}
                    >
                      Choose target repository in native dialog
                    </button>
                    <p>{target?.path ?? "No target selected"}</p>
                    <label>
                      New Python file
                      <input
                        value={editFile}
                        maxLength={64}
                        onChange={(e) => {
                          setEditFile(e.currentTarget.value);
                          invalidate();
                        }}
                      />
                    </label>
                    <label>
                      Unchanged test file
                      <input
                        value={testFile}
                        maxLength={64}
                        onChange={(e) => {
                          setTestFile(e.currentTarget.value);
                          invalidate();
                        }}
                      />
                    </label>
                  </section>
                ) : (
                  <>
                    <SourcePicker
                      value={librarySources}
                      onChange={(s) => {
                        setLibrarySources(s);
                        invalidate();
                      }}
                    />
                    {sources.map((s, i) => (
                      <div key={i}>
                        <label>
                          Source {i + 1} label
                          <input
                            maxLength={32}
                            value={s.label}
                            onChange={(e) => {
                              setSources(
                                sources.map((x, n) =>
                                  n === i ? { ...x, label: e.currentTarget.value } : x,
                                ),
                              );
                              invalidate();
                            }}
                          />
                        </label>
                        <label>
                          Source {i + 1} text
                          <textarea
                            maxLength={4096}
                            value={s.text}
                            onChange={(e) => {
                              setSources(
                                sources.map((x, n) =>
                                  n === i ? { ...x, text: e.currentTarget.value } : x,
                                ),
                              );
                              invalidate();
                            }}
                          />
                        </label>
                      </div>
                    ))}
                    <button
                      type="button"
                      disabled={sources.length >= 6}
                      onClick={() => {
                        setSources([
                          ...sources,
                          { label: `source${String(sources.length + 1)}`, text: "" },
                        ]);
                        invalidate();
                      }}
                    >
                      Add source
                    </button>
                  </>
                )}
                <button
                  type="button"
                  disabled={!objective.trim() || (workflow === "coding_action" && !target)}
                  onClick={() =>
                    void action(async () => {
                      const input = {
                        workflow,
                        objective,
                        sources:
                          workflow === "coding_action"
                            ? []
                            : [...sources.filter((s) => s.text.trim()), ...librarySources],
                      };
                      setPreview(
                        workflow === "coding_action" && target
                          ? await client.prepare(room.id, input, {
                              selection: target.selection,
                              file: editFile,
                              testFile,
                            })
                          : await client.prepare(room.id, input),
                      );
                      setAck(false);
                    })
                  }
                >
                  Check readiness and review transmission
                </button>
              </fieldset>
              {preview?.roomId === room.id && (
                <section aria-label="Transmission review">
                  <h3>
                    {preview.simulation
                      ? "Simulation only · no provider calls"
                      : "Live workflow · explicit approval required"}
                  </h3>
                  {preview.run.action && (
                    <>
                      <p>
                        Only the new file <code>{preview.run.action.file}</code> may be added. Fixed
                        unittest command uses <code>{preview.run.action.testFile}</code>; 15-second
                        execution, 128 MiB memory, 32 PIDs and 16 KiB output. The target remains
                        unchanged until separate native approval.
                      </p>
                      <details>
                        <summary>Selected original file sent to Coding</summary>
                        <pre>{preview.run.action.original}</pre>
                      </details>
                      <details>
                        <summary>Unchanged tests sent to Coding and executed in isolation</summary>
                        <pre>{preview.run.action.tests}</pre>
                      </details>
                    </>
                  )}
                  <ol>
                    {preview.run.stages.map((s) => (
                      <li key={s.id}>
                        <BotAvatar
                          identity={s.participant.identity}
                          agentId={s.participant.agentId}
                        />
                        {s.participant.identity.nickname || s.participant.role} ·{" "}
                        {s.participant.role} · {s.participant.connection} · {s.participant.model} ·{" "}
                        {s.participant.effort}
                        <details>
                          <summary>Shared personality and custom instructions</summary>
                          <pre>
                            {JSON.stringify(
                              {
                                identity: s.participant.identity,
                                instructions: s.participant.ownerInstructions,
                                endpoint: s.participant.endpoint,
                              },
                              null,
                              2,
                            )}
                          </pre>
                        </details>
                      </li>
                    ))}
                  </ol>
                  <p>
                    Each listed provider receives the objective, all labeled sources, validated
                    prior-stage outputs and that bot's displayed identity/personality and custom
                    instructions. Private notes are excluded. At most {preview.maximumCalls}{" "}
                    sequential calls, 60 seconds per stage, 310 seconds per run, 16 KiB output and
                    1,024 events per stage. No application retries or fallback. Headless Codex may
                    retry internally within one stage. Provider retention and charges may apply;
                    store=false is not zero retention. Stop cannot guarantee immediate remote
                    termination or zero billing. No mid-run steering.
                  </p>
                  <details>
                    <summary>Objective and shared sources</summary>
                    <pre>{JSON.stringify(preview.run.input, null, 2)}</pre>
                  </details>
                  <label>
                    <input
                      type="checkbox"
                      checked={ack}
                      onChange={(e) => {
                        setAck(e.currentTarget.checked);
                      }}
                    />
                    I acknowledge this exact route, settings, shared content, local retention,
                    provider terms and possible charges.
                  </label>
                  <button
                    disabled={!ack || busy || active}
                    onClick={() =>
                      void action(async () => {
                        store.acceptRoom(await client.start(preview));
                        invalidate();
                      })
                    }
                  >
                    {preview.simulation
                      ? "Start simulation workflow"
                      : "Start approved live workflow"}
                  </button>
                </section>
              )}
              {active && (
                <p role="status">
                  A collaboration run owns the native session. Other generations are unavailable
                  until cleanup.
                </p>
              )}
            </>
          )}
        </section>
      </div>
    </section>
  );
}
