import { ConductorIdentity } from "../../src/features/agents/ConductorIdentity";
import {
  WORKFLOWS,
  type Run,
  type Status,
} from "../../src/infrastructure/tauri/collaboration-client";
import { useState } from "react";
import { BotAvatar } from "../../src/features/agents/BotAppearance";
import { AGENT_IDS, DEFAULT_BOT_IDENTITY } from "../../src/infrastructure/tauri/agent-chat-client";
import { setBotAnimations, useBotAnimations } from "../../src/features/agents/botMotion";
// Offline presentation fixture: no IPC, profile access or provider client.
export function MascotFixture() {
  const [conductor, setConductor] = useState(false);
  const [selected, setSelected] = useState<(typeof AGENT_IDS)[number]>(AGENT_IDS[0]);
  const [opened, setOpened] = useState(0),
    [success, setSuccess] = useState(0);
  const [light, setLight] = useState(false);
  const enabled = useBotAnimations();
  return (
    <main
      style={{
        padding: 24,
        color: light ? "#101722" : "#f3f5fb",
        background: light ? "#f3f5fb" : "#101722",
        minHeight: "100vh",
      }}
    >
      <h1>Offline bot animation fixture</h1>
      <p>Synthetic presentation only. No connection, work or provider status.</p>
      <button
        onClick={() => {
          setLight(!light);
        }}
      >
        Toggle contrast surface
      </button>
      <label>
        <input
          type="checkbox"
          checked={enabled}
          onChange={(e) => {
            setBotAnimations(e.target.checked);
          }}
        />{" "}
        Enable animations
      </label>
      <button
        onClick={() => {
          setConductor(!conductor);
        }}
      >
        Conductor preview
      </button>
      <nav aria-label="Nine mascots">
        {AGENT_IDS.map((id) => (
          <button
            key={id}
            onClick={() => {
              setSelected(id);
              setOpened(0);
              setSuccess(0);
            }}
          >
            <BotAvatar agentId={id} identity={DEFAULT_BOT_IDENTITY} />
            {id}
          </button>
        ))}
      </nav>
      {conductor ? (
        <ConductorFixture />
      ) : (
        <>
          <h2>{selected}</h2>
          <BotAvatar
            key={`${selected}-${String(opened)}`}
            agentId={selected}
            identity={DEFAULT_BOT_IDENTITY}
            expressive
            greeting={opened > 0}
            success={success}
          />
          <button
            onClick={() => {
              setOpened((n) => n + 1);
            }}
          >
            Synthetic conversation greeting
          </button>
          <button
            onClick={() => {
              setSuccess((n) => n + 1);
            }}
          >
            Synthetic success event
          </button>
        </>
      )}
    </main>
  );
}

// Fixed local presentation snapshots only; no client, IPC, rooms or profile writes.
export function ConductorFixture() {
  const makeRun = (status: Status, sequence: number): Run => ({
    id: "synthetic-conductor-run",
    sequence,
    status,
    error: null,
    input: { workflow: "research", objective: "Synthetic animation state", sources: [] },
    stages: WORKFLOWS.research.map((agentId, i) => ({
      id: `synthetic-conductor-run/stage/${String(i)}`,
      status: status === "running" && i > 0 ? "queued" : status,
      timestamp: 0,
      provisional: "",
      input: "",
      participant: {
        agentId,
        role: agentId,
        identity: DEFAULT_BOT_IDENTITY,
        connection: "simulation",
        model: "simulation",
        effort: "default",
        endpoint: "",
        localAuth: false,
        ownerInstructions: "",
        revision: 0,
      },
      handoff:
        status === "completed"
          ? {
              version: 1,
              stage: i,
              agentId,
              status: "complete",
              summary: "Synthetic",
              findings: [],
              evidence: [],
              limitations: [],
            }
          : null,
    })),
  });
  const [run, setRun] = useState(() => makeRun("queued", 0));
  const [current, setCurrent] = useState(true);
  const [visit, setVisit] = useState(0);
  return (
    <section aria-label="Synthetic coordinator fixture">
      <h2>Conductor · synthetic state fixture</h2>
      <p>No workflow is executed. These are isolated presentation snapshots, not live activity.</p>
      <ConductorIdentity key={visit} run={run} current={current} expressive />
      {(
        ["queued", "running", "completed", "failed", "cancelled", "interrupted", "partial"] as const
      ).map((status) => (
        <button
          key={status}
          onClick={() => {
            setRun(makeRun(status, run.sequence + 1));
          }}
        >
          {status}
        </button>
      ))}
      <button
        onClick={() => {
          setRun({
            ...run,
            sequence: run.sequence + 1,
            stages: run.stages.map((s, i) => (i === 0 ? { ...s, status: "completed" } : s)),
          });
        }}
      >
        One stage completed
      </button>
      <button
        onClick={() => {
          setCurrent(!current);
        }}
      >
        Toggle historical snapshot
      </button>
      <button
        onClick={() => {
          setVisit(visit + 1);
        }}
      >
        Reopen preview
      </button>
    </section>
  );
}
