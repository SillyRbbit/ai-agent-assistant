import { useEffect, useState } from "react";
import {
  agentChatErrorMessage,
  type AgentChatClient,
  type AgentProfile,
  type AgentChatSnapshot,
} from "../../infrastructure/tauri/agent-chat-client";
import type {
  SavedSummary,
  SavedConversation,
  SavedTurn,
  NoteProvenance,
} from "../../infrastructure/tauri/conversation-history-client";

export function SavedConversations({
  client,
  profile,
  busy,
  onOpen,
  onDeleted,
}: {
  readonly client: AgentChatClient;
  readonly profile: AgentProfile;
  readonly busy: boolean;
  readonly onOpen: (s: AgentChatSnapshot) => void;
  readonly onDeleted?: () => void;
}) {
  const [items, setItems] = useState<readonly SavedSummary[]>([]);
  const [error, setError] = useState("");
  const [pending, setPending] = useState(false);
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState<SavedSummary | null>(null);
  const [title, setTitle] = useState("");
  const [confirmDelete, setConfirmDelete] = useState(false);
  useEffect(() => {
    let live = true;
    void client.persistence
      ?.list(profile.agentId)
      .then((v) => {
        if (live) {
          setItems(v);
          setLoading(false);
        }
      })
      .catch(() => {
        if (live) {
          setError("Saved conversations could not be loaded.");
          setLoading(false);
        }
      });
    return () => {
      live = false;
    };
  }, [client, profile.agentId]);
  const persistence = client.persistence;
  if (!persistence) return null;
  const run = async (action: () => Promise<void>) => {
    setPending(true);
    setError("");
    try {
      await action();
      setItems(await persistence.list(profile.agentId));
    } catch (e) {
      setError(agentChatErrorMessage(e));
    } finally {
      setPending(false);
    }
  };
  return (
    <section className="page-panel" aria-label="Saved conversations">
      <h2>Saved conversations</h2>
      <p>
        Local to this bot. Complete, cancelled and failed output stays here. Opening never sends or
        replays a request.
      </p>
      {loading ? (
        <p role="status">Loading saved conversations…</p>
      ) : error !== "" && items.length === 0 ? null : items.length === 0 ? (
        <p>No saved conversations yet.</p>
      ) : (
        <ul>
          {items.map((item) => (
            <li key={item.id}>
              <button
                type="button"
                disabled={busy || pending}
                onClick={() =>
                  void run(async () => {
                    const s = await persistence.open(item.id);
                    if (s.agentId !== profile.agentId) throw new Error("protocol");
                    onOpen(s);
                  })
                }
              >
                {item.title} · {item.turns} turns
              </button>{" "}
              <button
                type="button"
                disabled={busy || pending}
                onClick={() => {
                  setSelected(item);
                  setTitle(item.title);
                  setConfirmDelete(false);
                }}
              >
                Manage {item.title}
              </button>
            </li>
          ))}
        </ul>
      )}
      {selected ? (
        <div>
          <label>
            Conversation title
            <input
              maxLength={80}
              value={title}
              onChange={(e) => {
                setTitle(e.currentTarget.value);
              }}
            />
          </label>
          <button
            type="button"
            disabled={busy || pending || !title.trim()}
            onClick={() =>
              void run(async () => {
                await persistence.rename(selected.id, title);
                setSelected(null);
              })
            }
          >
            Save title
          </button>
          <label>
            <input
              type="checkbox"
              checked={confirmDelete}
              onChange={(e) => {
                setConfirmDelete(e.currentTarget.checked);
              }}
            />
            Delete local conversation history. Separately saved memories remain, with source marked
            deleted. Backups and provider records are not erased.
          </label>
          <button
            type="button"
            disabled={busy || pending || !confirmDelete}
            onClick={() =>
              void run(async () => {
                await persistence.delete(selected.id);
                setSelected(null);
                onDeleted?.();
              })
            }
          >
            Delete conversation
          </button>
        </div>
      ) : null}
      {pending ? <p role="status">Saving…</p> : null}
      {error ? <p role="alert">{error}</p> : null}
    </section>
  );
}
export function NoteHistory({
  client,
  profile,
  refresh = 0,
}: {
  readonly client: AgentChatClient;
  readonly profile: AgentProfile;
  readonly refresh?: number;
}) {
  const [value, setValue] = useState<NoteProvenance | null>(null);
  const [error, setError] = useState("");
  useEffect(() => {
    let live = true;
    void client.persistence
      ?.provenance(profile.agentId)
      .then((v) => {
        if (live) setValue(v);
      })
      .catch(() => {
        if (live) setError("Memory provenance unavailable.");
      });
    return () => {
      live = false;
    };
  }, [client, profile.agentId, profile.revision, refresh]);
  return (
    <aside>
      <p>
        Private notes are this bot’s durable memory. Add or edit them below and Save; Clear note
        deletes them. Memory Off excludes notes from future requests but keeps ordinary conversation
        history. Editing or deleting cannot retract information already sent to a provider or erase
        backups.
      </p>
      {value ? (
        <p>
          {value.createdAtMs === 0
            ? "Imported note; original creation time unknown."
            : `Created ${new Date(value.createdAtMs).toLocaleString()}.`}{" "}
          Updated{" "}
          {value.updatedAtMs === 0
            ? "before provenance was recorded"
            : new Date(value.updatedAtMs).toLocaleString()}
          .{" "}
          {value.sourceConversation
            ? `Last confirmed fact source: ${value.sourceConversation}${value.sourceDeleted ? " (conversation deleted)" : ""}.`
            : "Owner-edited note."}
        </p>
      ) : null}
      {error ? <p role="alert">{error}</p> : null}
    </aside>
  );
}
export function SavedTranscript({
  client,
  initial,
  busy,
  refresh,
}: {
  readonly client: AgentChatClient;
  readonly initial: AgentChatSnapshot;
  readonly busy: boolean;
  readonly refresh: number;
}) {
  const [saved, setSaved] = useState<SavedConversation | null>(null);
  const [error, setError] = useState("");
  const [turn, setTurn] = useState<SavedTurn | null>(null);
  const [fact, setFact] = useState("");
  const [confirmed, setConfirmed] = useState(false);
  const [pending, setPending] = useState(false);
  const [status, setStatus] = useState("");
  useEffect(() => {
    if (busy) return;
    let live = true;
    void client.persistence
      ?.read(initial.conversationId)
      .then((v) => {
        if (v.agentId !== initial.agentId) throw new Error("protocol");
        if (live) setSaved(v);
      })
      .catch(() => {
        if (live) setError("Saved transcript unavailable; no automatic retry or replay.");
      });
    return () => {
      live = false;
    };
  }, [client, initial.agentId, initial.conversationId, busy, refresh]);
  const persistence = client.persistence;
  if (!persistence) return null;
  const save = async () => {
    if (!turn || !confirmed || busy) return;
    setPending(true);
    setError("");
    try {
      const profile = (await client.list()).find((p) => p.agentId === initial.agentId);
      if (!profile) throw new Error("protocol");
      const updated = await persistence.fact(
        initial.conversationId,
        turn.id,
        fact,
        profile.revision,
      );
      if (updated.agentId !== initial.agentId) throw new Error("protocol");
      setTurn(null);
      setFact("");
      setConfirmed(false);
      setStatus("Fact appended to this bot’s private notes. Memory Off remains unchanged.");
    } catch (e) {
      setError(agentChatErrorMessage(e));
    } finally {
      setPending(false);
    }
  };
  return (
    <section aria-label="Stored transcript">
      <h3>Stored history</h3>
      <p>
        Only the newest fitting completed turns (at most three, 12,288 history characters) enter the
        next request. Older and incomplete turns remain stored but are omitted. Runtime/model
        changes require a new conversation; current notes and Memory Off apply to subsequent sends.
      </p>
      {saved ? (
        <p>
          Next request context: {saved.contextTurnIds.length} completed turns included
          {saved.contextTurnIds.length ? ` (turns ${saved.contextTurnIds.join(", ")})` : ""};{" "}
          {saved.turns.length - saved.contextTurnIds.length} stored turns omitted. Stored history is
          not summarized or deleted.
        </p>
      ) : null}
      {saved?.turns.map((t) => (
        <article className="agents-answer" key={t.id}>
          <h4>
            Turn {t.id} · {t.status}
          </h4>
          <p style={{ whiteSpace: "pre-wrap" }}>You: {t.message}</p>
          <p style={{ whiteSpace: "pre-wrap" }}>{t.answer}</p>
          {t.failure ? (
            <p role="status">
              {t.failure === "timeout"
                ? "The request reached its deadline. Send a shorter message when ready."
                : t.failure === "limit"
                  ? "The response reached a bounded output limit. Ask for a shorter response."
                  : t.failure === "storage"
                    ? "A native or local storage operation failed. Retained output may be incomplete; no request was replayed."
                    : "The provider could not complete this turn. Check the selected connection before another explicit send."}
            </p>
          ) : null}
          {t.status !== "running" ? (
            <button
              type="button"
              disabled={busy || pending}
              onClick={() => {
                setTurn(t);
                setFact("");
                setConfirmed(false);
              }}
            >
              Save a selected fact from turn {t.id}
            </button>
          ) : null}
        </article>
      ))}
      {turn ? (
        <div>
          <label>
            Fact preview · copy only the exact excerpt to save
            <textarea
              value={fact}
              maxLength={1024}
              onChange={(e) => {
                setFact(e.currentTarget.value);
                setConfirmed(false);
              }}
            />
          </label>
          <label>
            <input
              type="checkbox"
              checked={confirmed}
              onChange={(e) => {
                setConfirmed(e.currentTarget.checked);
              }}
            />
            Confirm appending this excerpt to this bot’s private notes. No automatic learning.
          </label>
          <button
            type="button"
            disabled={!confirmed || !fact.trim() || pending || busy}
            onClick={() => void save()}
          >
            Confirm save fact
          </button>
          <button
            type="button"
            disabled={pending}
            onClick={() => {
              setTurn(null);
            }}
          >
            Cancel memory save
          </button>
        </div>
      ) : null}
      {pending ? <p role="status">Saving memory…</p> : null}
      {status ? <p role="status">{status}</p> : null}
      {error ? <p role="alert">{error}</p> : null}
    </section>
  );
}
