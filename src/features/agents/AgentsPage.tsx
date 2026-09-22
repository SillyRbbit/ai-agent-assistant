import { useCallback, useEffect, useRef, useState } from "react";
import {
  agentChatClient,
  agentChatErrorMessage,
  EFFORTS,
  OPENAI_AGENT_MODELS,
  type AgentChatClient,
  type AgentChatSnapshot,
  type AgentConnection,
  type AgentConnectionReadiness,
  type AgentProfile,
  type AgentProfileInput,
} from "../../infrastructure/tauri/agent-chat-client";
import "./AgentsPage.css";

const CONNECTION_LABELS = {
  simulation: "Simulation · no provider",
  openai_api: "OpenAI API",
  codex: "Codex · live unavailable",
} as const;
const HOSTED_DISCLOSURE =
  "OpenAI receives this message, earlier turns in this conversation, saved owner instructions, and this agent's private note when enabled. This is a paid API request. store=false is not Zero Data Retention; provider abuse-monitoring and cache retention may still apply. Stop aborts local output but cannot guarantee immediate remote termination or zero billing. No tools, automatic retry, or connection fallback.";

export function AgentsPage({ client = agentChatClient }: { readonly client?: AgentChatClient }) {
  const [profiles, setProfiles] = useState<readonly AgentProfile[]>([]);
  const [connections, setConnections] = useState<readonly AgentConnectionReadiness[]>([]);
  const [selectedId, setSelectedId] = useState("personal-assistant");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);
  const [conversation, setConversation] = useState<AgentChatSnapshot | null>(null);
  const [busy, setBusy] = useState(false);
  const available = client.available();
  useEffect(() => {
    let active = true;
    if (available) {
      void Promise.all([client.list(), client.connections()])
        .then(([next, readiness]) => {
          if (active) {
            setProfiles(next);
            setConnections(readiness);
            setLoading(false);
          }
        })
        .catch((failure: unknown) => {
          if (active) {
            setError(agentChatErrorMessage(failure));
            setLoading(false);
          }
        });
    }
    return () => {
      active = false;
    };
  }, [available, client]);
  const selected = profiles.find((profile) => profile.agentId === selectedId);
  return (
    <section className="page-stack agents-page" aria-labelledby="agents-page-title">
      <header className="page-header">
        <div>
          <p className="section-kicker">Private workspace</p>
          <h1 id="agents-page-title">Agents</h1>
          <p>
            Choose one agent for bounded text advice. Settings and manually managed notes stay local
            until you explicitly send them.
          </p>
        </div>
      </header>
      {!available ? (
        <p role="status" className="page-panel">
          Open the native Cortexa app to load agents and saved settings. Browser mode cannot save
          settings or send provider requests.
        </p>
      ) : loading ? (
        <p role="status">Loading native agent settings…</p>
      ) : null}
      {error !== "" ? <p role="alert">{error}</p> : null}
      {selected !== undefined ? (
        <div className="agents-layout">
          <nav className="agents-list" aria-label="Agent selection">
            {profiles.map((profile) => (
              <button
                key={profile.agentId}
                type="button"
                aria-current={profile.agentId === selected.agentId ? "true" : undefined}
                disabled={busy}
                onClick={() => {
                  setSelectedId(profile.agentId);
                  setConversation(null);
                }}
              >
                <strong>{profile.displayName}</strong>
                <span>{CONNECTION_LABELS[profile.connection]}</span>
              </button>
            ))}
          </nav>
          <div className="agents-detail">
            {conversation === null ? (
              <AgentSettings
                key={`${selected.agentId}-${String(selected.revision)}`}
                profile={selected}
                client={client}
                connections={connections}
                onBusy={setBusy}
                onSaved={(next) => {
                  setProfiles((previous) =>
                    previous.map((profile) => (profile.agentId === next.agentId ? next : profile)),
                  );
                }}
                onConversation={setConversation}
              />
            ) : (
              <AgentConversation
                key={conversation.conversationId}
                initial={conversation}
                displayName={selected.displayName}
                client={client}
                onBusy={setBusy}
                onExit={() => {
                  setConversation(null);
                }}
              />
            )}
          </div>
        </div>
      ) : null}
    </section>
  );
}

interface SettingsProps {
  readonly profile: AgentProfile;
  readonly client: AgentChatClient;
  readonly connections: readonly AgentConnectionReadiness[];
  readonly onSaved: (next: AgentProfile) => void;
  readonly onBusy: (busy: boolean) => void;
  readonly onConversation: (next: AgentChatSnapshot) => void;
}
function AgentSettings({
  profile,
  client,
  connections,
  onSaved,
  onBusy,
  onConversation,
}: SettingsProps) {
  const [draft, setDraft] = useState<AgentProfileInput>(() => ({
    agentId: profile.agentId,
    connection: profile.connection,
    model: profile.model,
    effort: profile.effort,
    ownerInstructions: profile.ownerInstructions,
    memoryMode: profile.memoryMode,
    note: profile.note,
    revision: profile.revision,
  }));
  const [pending, setPending] = useState(false);
  const [error, setError] = useState("");
  const [confirmClear, setConfirmClear] = useState(false);
  const mounted = useRef(true);
  const activeOperation = useRef(false);
  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
    };
  }, []);
  const dirty =
    draft.connection !== profile.connection ||
    draft.model !== profile.model ||
    draft.effort !== profile.effort ||
    draft.ownerInstructions !== profile.ownerInstructions ||
    draft.memoryMode !== profile.memoryMode ||
    draft.note !== profile.note;
  const readiness = connections.find((entry) => entry.connection === draft.connection);
  const perform = async (action: () => Promise<void>) => {
    if (activeOperation.current) return;
    activeOperation.current = true;
    setPending(true);
    onBusy(true);
    setError("");
    try {
      await action();
    } catch (failure) {
      if (mounted.current) setError(agentChatErrorMessage(failure));
    } finally {
      activeOperation.current = false;
      onBusy(false);
      if (mounted.current) setPending(false);
    }
  };
  const changeConnection = (connection: AgentConnection) => {
    setDraft((previous) => ({
      ...previous,
      connection,
      model:
        connection === "openai_api"
          ? OPENAI_AGENT_MODELS[0]
          : connection === "codex"
            ? "unavailable"
            : "simulation",
      effort: "default",
    }));
  };
  return (
    <section className="agents-settings page-panel" aria-labelledby="agent-settings-title">
      <h2 id="agent-settings-title">{profile.displayName}</h2>
      <p>
        Saved settings apply to a new conversation. Selecting another agent discards unsaved edits.
        Agent authority and fixture workflows remain unchanged.
      </p>
      <fieldset disabled={pending}>
        <legend>Connection and response</legend>
        <div className="agents-fields">
          <label>
            Connection
            <select
              value={draft.connection}
              onChange={(event) => {
                changeConnection(event.currentTarget.value as AgentConnection);
              }}
            >
              {Object.entries(CONNECTION_LABELS).map(([value, label]) => (
                <option value={value} key={value}>
                  {label}
                </option>
              ))}
            </select>
          </label>
          <label>
            Model
            <select
              value={draft.model}
              onChange={(event) => {
                setDraft({ ...draft, model: event.currentTarget.value });
              }}
            >
              {draft.connection === "openai_api" ? (
                OPENAI_AGENT_MODELS.map((model) => (
                  <option value={model} key={model}>
                    {model}
                  </option>
                ))
              ) : (
                <option value={draft.model}>
                  {draft.model === "unavailable"
                    ? "Unavailable until isolation is verified"
                    : "Deterministic simulation"}
                </option>
              )}
            </select>
          </label>
          <label>
            Reasoning effort
            <select
              value={draft.effort}
              onChange={(event) => {
                setDraft({
                  ...draft,
                  effort: event.currentTarget.value as AgentProfileInput["effort"],
                });
              }}
            >
              {(draft.connection === "openai_api" ? EFFORTS : ["default"]).map((effort) => (
                <option key={effort} value={effort}>
                  {effort === "default" ? "Default" : effort}
                </option>
              ))}
            </select>
          </label>
        </div>
        <p className="agents-readiness" role="status">
          {readiness?.message ?? "Connection readiness unavailable."}
        </p>
        {draft.connection === "codex" ? (
          <p>
            Authentication is unverified. Codex is disabled because complete tool and file isolation
            could not be established with the installed runtime. No ChatGPT subscription or API
            billing mode is assumed.
          </p>
        ) : null}
        <label>
          Owner instructions <span>(optional, 4,096 characters)</span>
          <textarea
            rows={3}
            maxLength={4096}
            value={draft.ownerInstructions}
            onChange={(event) => {
              setDraft({ ...draft, ownerInstructions: event.currentTarget.value });
            }}
          />
        </label>
        <p>
          Use instructions for tone and preferences. They cannot grant tools, shell, file, or device
          authority. Do not enter API keys or other credentials.
        </p>
      </fieldset>
      <fieldset disabled={pending}>
        <legend>Private notes · manually managed</legend>
        <label>
          Memory mode
          <select
            value={draft.memoryMode}
            onChange={(event) => {
              setDraft({
                ...draft,
                memoryMode: event.currentTarget.value as AgentProfileInput["memoryMode"],
              });
            }}
          >
            <option value="off">Off · exclude notes from context</option>
            <option value="private_notes">Private notes · include this agent's note</option>
          </select>
        </label>
        <label>
          Private note <span>(8,192 characters)</span>
          <textarea
            rows={5}
            maxLength={8192}
            value={draft.note}
            onChange={(event) => {
              setDraft({ ...draft, note: event.currentTarget.value });
            }}
          />
        </label>
        <p>
          This note belongs only to {profile.displayName}. Notes are untrusted context, not
          automatic learning. Memory Off keeps the saved note locally but excludes it from requests.
          Changes or deletion require a new conversation.
        </p>
        <p>
          Enabled notes are sent to the selected hosted service when you send a message. Local
          deletion does not erase data already sent to a provider.
        </p>
        {confirmClear ? (
          <div className="agents-confirm" role="group" aria-label="Confirm note deletion">
            <p>Clear the saved note for {profile.displayName}? This cannot be undone.</p>
            <button
              type="button"
              onClick={() =>
                void perform(async () => {
                  onSaved(await client.clearNote(profile.agentId, profile.revision));
                })
              }
            >
              Confirm clear note
            </button>
            <button
              type="button"
              onClick={() => {
                setConfirmClear(false);
              }}
            >
              Keep note
            </button>
          </div>
        ) : (
          <button
            type="button"
            disabled={profile.note === ""}
            onClick={() => {
              setConfirmClear(true);
            }}
          >
            Clear saved note…
          </button>
        )}
      </fieldset>
      {error !== "" ? <p role="alert">{error}</p> : null}
      <div className="agents-actions">
        <button
          className="agents-primary"
          type="button"
          disabled={pending || !dirty}
          onClick={() =>
            void perform(async () => {
              onSaved(await client.save(draft));
            })
          }
        >
          Save settings and note
        </button>
        <button
          type="button"
          disabled={pending}
          onClick={() =>
            void perform(async () => {
              onSaved(await client.defaults(profile.agentId, profile.revision));
            })
          }
        >
          Restore defaults
        </button>
        <button
          type="button"
          disabled={
            pending ||
            dirty ||
            readiness === undefined ||
            readiness.status === "blocked" ||
            draft.connection === "codex"
          }
          onClick={() =>
            void perform(async () => {
              const next = await client.start(profile.agentId);
              if (
                next.agentId !== profile.agentId ||
                next.connection !== profile.connection ||
                next.model !== profile.model ||
                next.effort !== profile.effort ||
                next.memoryMode !== profile.memoryMode ||
                next.settingsRevision !== profile.revision
              )
                throw new Error("stale_context");
              if (mounted.current) onConversation(next);
            })
          }
        >
          Start conversation
        </button>
      </div>
      <p role="status">
        {pending
          ? "Saving / checking native state…"
          : dirty
            ? "Unsaved edits. Save before starting a conversation."
            : `Saved locally · revision ${String(profile.revision)}`}
      </p>
      <p>
        Restore defaults retains the saved note and turns Memory Off. Starting a conversation
        captures saved settings; it sends no provider request.
      </p>
    </section>
  );
}

interface ConversationProps {
  readonly initial: AgentChatSnapshot;
  readonly displayName: string;
  readonly client: AgentChatClient;
  readonly onBusy: (busy: boolean) => void;
  readonly onExit: () => void;
}
function AgentConversation({ initial, displayName, client, onBusy, onExit }: ConversationProps) {
  const [snapshot, setSnapshot] = useState(initial);
  const [message, setMessage] = useState("");
  const [acknowledged, setAcknowledged] = useState(false);
  const [pending, setPending] = useState(false);
  const [error, setError] = useState("");
  const mounted = useRef(true);
  const current = useRef(initial);
  const [history, setHistory] = useState<
    readonly { readonly message: string; readonly answer: string }[]
  >([]);
  const [sentMessage, setSentMessage] = useState("");
  const [cleanupUnconfirmed, setCleanupUnconfirmed] = useState(false);
  const uncertainCleanup = useRef(false);
  const sendPending = useRef(false);
  const stopRequested = useRef(false);
  const generation = useRef(0);
  const accept = useCallback(
    (next: AgentChatSnapshot, fresh = false) => {
      const previous = current.current;
      if (
        next.conversationId !== initial.conversationId ||
        next.agentId !== initial.agentId ||
        next.connection !== initial.connection ||
        next.model !== initial.model ||
        next.effort !== initial.effort ||
        next.memoryMode !== initial.memoryMode ||
        next.settingsRevision !== initial.settingsRevision ||
        (!fresh &&
          (next.sequence < previous.sequence ||
            !next.text.startsWith(previous.text) ||
            (["completed", "stopped", "error"].includes(previous.status) &&
              (next.status !== previous.status || next.text !== previous.text))))
      )
        throw new Error("protocol");
      current.current = next;
      uncertainCleanup.current = false;
      if (mounted.current) {
        setSnapshot(next);
        setCleanupUnconfirmed(false);
      }
    },
    [initial],
  );
  const stop = useCallback(async () => {
    stopRequested.current = true;
    ++generation.current;
    if (sendPending.current || (!current.current.busy && !uncertainCleanup.current)) return;
    try {
      const next = await client.cancel(initial.conversationId);
      if (mounted.current) accept(next);
    } catch (failure) {
      if (mounted.current) setError(agentChatErrorMessage(failure));
    }
  }, [accept, client, initial.conversationId]);
  useEffect(() => {
    mounted.current = true;
    const onVisibility = () => {
      if (document.hidden) void stop();
    };
    document.addEventListener("visibilitychange", onVisibility);
    return () => {
      mounted.current = false;
      document.removeEventListener("visibilitychange", onVisibility);
      void stop();
      onBusy(false);
    };
  }, [onBusy, stop]);
  useEffect(() => {
    onBusy(pending || snapshot.busy || cleanupUnconfirmed);
  }, [onBusy, pending, snapshot.busy, cleanupUnconfirmed]);
  useEffect(() => {
    if (!snapshot.busy || sendPending.current) return;
    let disposed = false;
    const operation = generation.current;
    const timer = window.setTimeout(() => {
      void client
        .poll(initial.conversationId)
        .then((next) => {
          if (!disposed && mounted.current && operation === generation.current) accept(next);
        })
        .catch((failure: unknown) => {
          if (!disposed && mounted.current) {
            setError(agentChatErrorMessage(failure));
            void stop();
          }
        });
    }, 250);
    return () => {
      disposed = true;
      window.clearTimeout(timer);
    };
  }, [accept, client, initial.conversationId, snapshot, stop]);
  const shouldStop = () => !mounted.current || stopRequested.current;
  const send = async () => {
    if (
      sendPending.current ||
      !["idle", "completed"].includes(current.current.status) ||
      current.current.busy ||
      uncertainCleanup.current ||
      history.length >= 3 ||
      message.trim() === "" ||
      (initial.connection === "openai_api" && !acknowledged) ||
      initial.connection === "codex"
    )
      return;
    sendPending.current = true;
    stopRequested.current = false;
    ++generation.current;
    setPending(true);
    setError("");
    try {
      let next = await client.send(
        initial.conversationId,
        message,
        initial.connection === "simulation" ? "simulation" : "openai-agent-text-v1",
      );
      if (shouldStop()) next = await client.cancel(initial.conversationId);
      if (mounted.current) {
        const previous = current.current;
        accept(next, true);
        if (previous.status === "completed")
          setHistory((turns) => [...turns, { message: sentMessage, answer: previous.text }]);
        setSentMessage(message);
        setMessage("");
        setAcknowledged(false);
      }
    } catch (failure) {
      if (mounted.current) setError(agentChatErrorMessage(failure));
      // A failed/invalid send response may still have created native work.
      // Close only this bound conversation; never retry the send.
      uncertainCleanup.current = true;
      if (mounted.current) setCleanupUnconfirmed(true);
      try {
        const next = await client.cancel(initial.conversationId);
        if (mounted.current) accept(next, true);
      } catch {
        // Preserve the closed original failure; native cleanup is not claimed.
      }
    } finally {
      sendPending.current = false;
      if (mounted.current) {
        setPending(false);
        setAcknowledged(false);
      }
    }
  };
  const canSend =
    !pending &&
    !snapshot.busy &&
    !cleanupUnconfirmed &&
    ["idle", "completed"].includes(snapshot.status) &&
    history.length < 3;
  return (
    <section className="agents-conversation page-panel" aria-labelledby="agent-conversation-title">
      <h2 id="agent-conversation-title">Conversation · {displayName}</h2>
      <p>
        {CONNECTION_LABELS[initial.connection]} · {initial.model} · effort {initial.effort} ·{" "}
        {initial.memoryMode === "off" ? "Memory Off" : "Own private note included"} · saved revision{" "}
        {initial.settingsRevision}
      </p>
      <p>
        Up to four bounded turns using the saved settings above. Start a new conversation after
        changing settings or notes. History is retained locally only for this native session; later
        hosted turns include that conversation text. No tools or device actions are available.
      </p>
      <p>
        Each message is limited to 4,096 characters. OpenAI requests have a 60-second deadline and a
        2,048-token output budget including reasoning. No automatic retry is made.
      </p>
      {initial.connection === "simulation" ? (
        <p className="agents-readiness">
          Simulation only. This produces labeled deterministic text without contacting a provider.
        </p>
      ) : (
        <>
          <p>{HOSTED_DISCLOSURE}</p>
          <label className="agents-checkbox">
            <input
              type="checkbox"
              checked={acknowledged}
              disabled={!canSend}
              onChange={(event) => {
                setAcknowledged(event.currentTarget.checked);
              }}
            />
            I acknowledge sending this message and selected context to OpenAI with possible API
            charges.
          </label>
        </>
      )}
      <label>
        Message
        <textarea
          rows={4}
          value={message}
          maxLength={4096}
          disabled={!canSend}
          onChange={(event) => {
            setMessage(event.currentTarget.value);
          }}
        />
      </label>
      <div className="agents-actions">
        <button
          className="agents-primary"
          type="button"
          disabled={
            !canSend ||
            message.trim() === "" ||
            (initial.connection === "openai_api" && !acknowledged) ||
            initial.connection === "codex"
          }
          onClick={() => void send()}
        >
          {initial.connection === "simulation" ? "Send simulated message" : "Send to OpenAI"}
        </button>
        <button
          type="button"
          disabled={!pending && !snapshot.busy && !cleanupUnconfirmed}
          onClick={() => void stop()}
        >
          Stop generation
        </button>
        <button
          type="button"
          disabled={pending || snapshot.busy || cleanupUnconfirmed}
          onClick={onExit}
        >
          Back to agent settings
        </button>
      </div>
      <p role="status">
        {pending ? "Submitting…" : snapshot.status}
        {cleanupUnconfirmed
          ? " · native cleanup unconfirmed"
          : snapshot.busy
            ? " · native generation owns this session"
            : " · no active native generation"}
      </p>
      {history.map((turn, index) => (
        <div className="agents-answer" key={index}>
          <h3>
            Completed turn {index + 1}
            {initial.connection === "simulation" ? " · simulated" : ""}
          </h3>
          <p>You: {turn.message}</p>
          <p>{turn.answer}</p>
        </div>
      ))}
      {sentMessage !== "" ? <p>You: {sentMessage}</p> : null}
      {snapshot.text !== "" ? (
        <div className="agents-answer">
          <h3>
            {snapshot.status === "completed"
              ? snapshot.connection === "simulation"
                ? "Simulated answer · completed"
                : "Answer · completed"
              : "Partial output · incomplete"}
          </h3>
          <p>{snapshot.text}</p>
        </div>
      ) : null}
      {snapshot.error !== null || error !== "" ? (
        <p role="alert">
          {snapshot.error !== null ? agentChatErrorMessage(snapshot.error) : error}
        </p>
      ) : null}
    </section>
  );
}
