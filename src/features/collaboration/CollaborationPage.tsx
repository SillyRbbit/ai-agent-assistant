import { useCallback, useEffect, useRef, useState } from "react";
import {
  collaborationClient,
  newerRooms,
  WORKFLOWS,
  type CollaborationClient,
  type Room,
  type Workflow,
  type Source,
  type Preview,
} from "../../infrastructure/tauri/collaboration-client";
import { BotAvatar } from "../agents/BotAppearance";
import { ROLE_DESCRIPTIONS } from "../agents/botAppearanceValues";
import { agentChatErrorMessage } from "../../infrastructure/tauri/agent-chat-client";
import "./CollaborationPage.css";
export function CollaborationPage({
  client = collaborationClient,
}: {
  readonly client?: CollaborationClient;
}) {
  const [rooms, setRooms] = useState<readonly Room[]>([]),
    [selected, setSelected] = useState<number | null>(null),
    [title, setTitle] = useState(""),
    [workflow, setWorkflow] = useState<Workflow>("research"),
    [objective, setObjective] = useState(""),
    [sources, setSources] = useState<readonly Source[]>([{ label: "source1", text: "" }]),
    [preview, setPreview] = useState<Preview | null>(null),
    [ack, setAck] = useState(false),
    [busy, setBusy] = useState(false),
    [error, setError] = useState("");
  const epoch = useRef(0),
    alive = useRef(true);
  const room = rooms.find((r) => r.id === selected);
  const active = rooms.some((r) =>
    r.runs.some((run) => run.status === "running" || run.status === "queued"),
  );
  const load = useCallback(async () => {
    const n = epoch.current;
    try {
      const next = await client.list();
      if (alive.current && epoch.current === n) setRooms((old) => newerRooms(old, next));
    } catch {
      if (alive.current)
        setError("Room history is unavailable. Native Cortexa access is required.");
    }
  }, [client]);
  useEffect(() => {
    alive.current = true;
    let cancelled = false;
    const initialEpoch = epoch.current;
    void client
      .list()
      .then((next) => {
        if (!cancelled && initialEpoch === epoch.current) setRooms((old) => newerRooms(old, next));
      })
      .catch(() => {
        if (!cancelled) setError("Room history is unavailable. Native Cortexa access is required.");
      });
    return () => {
      cancelled = true;
      alive.current = false;
    };
  }, [client]);
  useEffect(() => {
    if (!active) return;
    const id = window.setInterval(() => void load(), 400);
    return () => {
      window.clearInterval(id);
    };
  }, [active, load]);
  async function action(fn: () => Promise<void>) {
    setBusy(true);
    setError("");
    epoch.current++;
    try {
      await fn();
      await load();
    } catch (e) {
      setError(
        typeof e === "string"
          ? agentChatErrorMessage(e)
          : "The bounded operation could not complete. Check saved connections and readiness; no automatic retry was made.",
      );
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
      <header>
        <h1 id="collaboration-title">Collaboration</h1>
        <p>
          Conductor · Application coordinator. Four fixed analysis routes; no browsing, tools,
          commands, edits, tests or infrastructure actions.
        </p>
      </header>
      <p className="room-retention">
        Submitted objectives, sources and visible results are stored locally (up to 10 rooms, 4 runs
        each). Encryption at rest is not claimed. Delete removes local history, not provider-held
        copies. Never submit credentials. Private bot notes and hidden reasoning are excluded.
      </p>
      {error && <p role="alert">{error}</p>}
      <div className="room-layout">
        <aside aria-label="Room list">
          <form
            onSubmit={(e) => {
              e.preventDefault();
              void action(async () => {
                const created = await client.create(title);
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
        <section aria-label="Selected collaboration room">
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
                    setRooms((old) => old.filter((r) => r.id !== room.id));
                    invalidate();
                  })
                }
              >
                Delete room and local history
              </button>
              {room.runs.map((run) => (
                <section key={run.id} aria-label={`Run ${run.id}`}>
                  <h3>
                    {run.input.workflow} · {run.status}
                  </h3>
                  <p>{run.input.objective}</p>
                  {run.error && <p role="alert">{agentChatErrorMessage(run.error)}</p>}
                  {run.stages.map((s, i) => (
                    <article key={s.id} className="room-message">
                      <header>
                        <BotAvatar identity={s.participant.identity} />
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
                          await client.cancel(room.id);
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
                  Route: {WORKFLOWS[workflow].join(" → ")}. Maximum {WORKFLOWS[workflow].length}{" "}
                  generation calls.
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
                <button
                  type="button"
                  disabled={!objective.trim()}
                  onClick={() =>
                    void action(async () => {
                      setPreview(
                        await client.prepare(room.id, {
                          workflow,
                          objective,
                          sources: sources.filter((s) => s.text.trim()),
                        }),
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
                  <ol>
                    {preview.run.stages.map((s) => (
                      <li key={s.id}>
                        <BotAvatar identity={s.participant.identity} />
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
                        await client.start(preview);
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
