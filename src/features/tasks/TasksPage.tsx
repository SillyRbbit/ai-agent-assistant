import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { PageHeader } from "../shared/PageHeader";
import {
  createLocalTasksClient,
  taskError,
  type LocalTask,
  type LocalTasksClient,
  type TaskInput,
  type TaskPriority,
} from "../../infrastructure/tauri/local-tasks-client";
import "./tasks.css";

const blank: TaskInput = {
  id: null,
  expectedRevision: null,
  title: "",
  notes: "",
  dueDate: null,
  priority: "normal",
};
type StatusFilter = "all" | "open" | "completed";
export function TasksPage({ client: supplied }: { readonly client?: LocalTasksClient }) {
  const client = useMemo(() => supplied ?? createLocalTasksClient(), [supplied]);
  const [tasks, setTasks] = useState<readonly LocalTask[]>([]);
  const [loadedClient, setLoadedClient] = useState<LocalTasksClient | null>(null);
  const [draft, setDraft] = useState<TaskInput>(blank);
  const [status, setStatus] = useState<StatusFilter>("open");
  const [priority, setPriority] = useState("all");
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(client.available);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [deleting, setDeleting] = useState<LocalTask | null>(null);
  const generation = useRef(0);
  const locked = useRef(false);
  const titleInput = useRef<HTMLInputElement>(null);
  const reload = useCallback(async () => {
    if (!client.available || locked.current) return;
    const token = ++generation.current;
    setLoading(true);
    setError("");
    try {
      const rows = await client.list();
      if (generation.current === token) {
        setTasks(rows);
        setLoadedClient(client);
      }
    } catch (e) {
      if (generation.current === token) setError(taskError(e));
    } finally {
      if (generation.current === token) setLoading(false);
    }
  }, [client]);
  useEffect(() => {
    const counter = generation;
    const lock = locked;
    const token = ++counter.current;
    // Hydration updates state only from the asynchronous result. Cleanup
    // invalidates both reads and writes from the previous store/session.
    const hydration = client.available ? client.list() : Promise.resolve([]);
    void hydration
      .then((rows) => {
        if (counter.current === token) {
          setTasks(rows);
          setLoadedClient(client);
          setDraft(blank);
          setDeleting(null);
          setError("");
        }
      })
      .catch((e: unknown) => {
        if (counter.current === token) setError(taskError(e));
      })
      .finally(() => {
        if (counter.current === token) {
          setLoading(false);
          setBusy(false);
        }
      });
    return () => {
      ++counter.current;
      lock.current = false;
    };
  }, [client]);
  const disabled = !client.available || busy || loading || loadedClient !== client;
  const visibleTasks = loadedClient === client ? tasks : [];
  const filtered = visibleTasks.filter(
    (t) =>
      (status === "all" || (status === "completed") === t.completed) &&
      (priority === "all" || t.priority === priority) &&
      `${t.title}\n${t.notes}`.toLocaleLowerCase().includes(search.trim().toLocaleLowerCase()),
  );
  function edit(task: LocalTask) {
    setDraft({
      id: task.id,
      expectedRevision: task.revision,
      title: task.title,
      notes: task.notes,
      dueDate: task.dueDate,
      priority: task.priority,
    });
    setError("");
    setNotice("");
    titleInput.current?.focus();
  }
  async function mutate(
    action: () => Promise<LocalTask | null>,
    kind: "save" | "complete" | "remove",
  ) {
    if (locked.current || disabled) return;
    locked.current = true;
    const token = ++generation.current;
    setBusy(true);
    setError("");
    setNotice("");
    try {
      const updated = await action();
      if (generation.current !== token) return;
      if (kind === "remove") {
        const id = deleting?.id;
        setTasks((rows) => rows.filter((t) => t.id !== id));
        if (draft.id === id) setDraft(blank);
        setDeleting(null);
        setNotice("Task deleted.");
      } else if (updated) {
        setTasks((rows) => [updated, ...rows.filter((t) => t.id !== updated.id)]);
        if (kind === "save") {
          setDraft(blank);
          setNotice("Task saved.");
          titleInput.current?.focus();
        } else {
          // Keep unsaved edits intact. Their old revision must fail rather than
          // silently rebinding the draft after another mutation.
          setNotice(updated.completed ? "Task completed." : "Task reopened.");
        }
      }
    } catch (e) {
      if (generation.current === token) setError(taskError(e));
    } finally {
      if (generation.current === token) {
        locked.current = false;
        setBusy(false);
      }
    }
  }
  return (
    <section className="tasks-page" aria-labelledby="tasks-page-title">
      <PageHeader
        headingId="tasks-page-title"
        title="Tasks"
        description="Keep your next steps in one place. Tasks stay on this device."
        badge="Local only"
      />
      {!client.available && (
        <p role="status" className="tasks-message">
          Open the Cortexa desktop app to save tasks. Browser preview does not persist tasks.
        </p>
      )}
      <div className="tasks-toolbar">
        <label>
          Search tasks
          <input
            type="search"
            value={search}
            maxLength={240}
            onChange={(e) => {
              setSearch(e.target.value);
            }}
          />
        </label>
        <label>
          Status
          <select
            value={status}
            onChange={(e) => {
              setStatus(e.target.value as StatusFilter);
            }}
          >
            <option value="open">Open</option>
            <option value="completed">Completed</option>
            <option value="all">All tasks</option>
          </select>
        </label>
        <label>
          Filter priority
          <select
            value={priority}
            onChange={(e) => {
              setPriority(e.target.value);
            }}
          >
            <option value="all">All priorities</option>
            <option value="high">High</option>
            <option value="normal">Normal</option>
            <option value="low">Low</option>
          </select>
        </label>
        <button
          type="button"
          disabled={!client.available || busy || loading}
          onClick={() => void reload()}
        >
          Reload tasks
        </button>
      </div>
      {error && (
        <p role="alert" className="tasks-message tasks-message--error">
          {error}
        </p>
      )}
      {notice && (
        <p role="status" className="tasks-message">
          {notice}
        </p>
      )}
      <div className="tasks-layout">
        <section
          className="page-panel tasks-list"
          aria-label="Task list"
          aria-busy={loading || busy}
        >
          <div className="tasks-list-heading">
            <h2>{status === "completed" ? "Completed tasks" : "Your tasks"}</h2>
            <span>
              {filtered.length} shown · {visibleTasks.length}/500
            </span>
          </div>
          {loading ? (
            <p role="status">Loading tasks…</p>
          ) : filtered.length === 0 ? (
            <p>
              {tasks.length === 0
                ? "No tasks yet. Add your first next step."
                : "No tasks match these filters."}
            </p>
          ) : (
            <ul>
              {filtered.map((task) => (
                <li
                  key={task.id}
                  className={task.completed ? "task-card task-card--completed" : "task-card"}
                >
                  <div>
                    <h3>{task.title}</h3>
                    <div className="task-meta">
                      <span>{task.priority} priority</span>
                      {task.dueDate && <time dateTime={task.dueDate}>Due {task.dueDate}</time>}
                      <span>{task.completed ? "Completed" : "Open"}</span>
                    </div>
                    {task.notes && <p className="task-notes">{task.notes}</p>}
                  </div>
                  <div className="task-actions">
                    <button
                      type="button"
                      aria-label={`${task.completed ? "Reopen" : "Complete"} ${task.title}`}
                      disabled={disabled}
                      onClick={() =>
                        void mutate(() => client.complete(task, !task.completed), "complete")
                      }
                    >
                      {task.completed ? "Reopen" : "Complete"}
                      <span className="sr-only"> {task.title}</span>
                    </button>
                    <button
                      type="button"
                      aria-label={`Edit ${task.title}`}
                      disabled={disabled}
                      onClick={() => {
                        edit(task);
                      }}
                    >
                      Edit<span className="sr-only"> {task.title}</span>
                    </button>
                    <button
                      type="button"
                      aria-label={`Delete ${task.title}`}
                      disabled={disabled}
                      onClick={() => {
                        setDeleting(task);
                      }}
                    >
                      Delete<span className="sr-only"> {task.title}</span>
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </section>
        <section className="page-panel tasks-editor" aria-labelledby="task-editor-title">
          <h2 id="task-editor-title">{draft.id === null ? "New task" : "Edit task"}</h2>
          <form
            onSubmit={(e) => {
              e.preventDefault();
              void mutate(() => client.save(draft), "save");
            }}
          >
            <label>
              Title
              <input
                ref={titleInput}
                required
                value={draft.title}
                maxLength={240}
                disabled={disabled}
                onChange={(e) => {
                  setDraft({ ...draft, title: e.target.value });
                }}
              />
            </label>
            <label>
              Notes
              <textarea
                rows={7}
                value={draft.notes}
                maxLength={8192}
                disabled={disabled}
                onChange={(e) => {
                  setDraft({ ...draft, notes: e.target.value });
                }}
              />
            </label>
            <label>
              Due date
              <input
                type="date"
                min="0001-01-01"
                max="9999-12-31"
                value={draft.dueDate ?? ""}
                disabled={disabled}
                onChange={(e) => {
                  setDraft({ ...draft, dueDate: e.target.value || null });
                }}
              />
            </label>
            <label>
              Priority
              <select
                value={draft.priority}
                disabled={disabled}
                onChange={(e) => {
                  setDraft({ ...draft, priority: e.target.value as TaskPriority });
                }}
              >
                <option value="low">Low</option>
                <option value="normal">Normal</option>
                <option value="high">High</option>
              </select>
            </label>
            <p className="task-hint">
              Due dates help you plan; they do not schedule notifications.
            </p>
            <div className="task-actions">
              <button type="submit" disabled={disabled || !draft.title.trim()}>
                {busy ? "Saving…" : "Save task"}
              </button>
              <button
                type="button"
                disabled={disabled}
                onClick={() => {
                  setDraft(blank);
                  setError("");
                  titleInput.current?.focus();
                }}
              >
                Clear draft
              </button>
            </div>
          </form>
        </section>
      </div>
      {deleting && (
        <section
          role="alertdialog"
          aria-modal="false"
          aria-labelledby="task-delete-title"
          aria-describedby="task-delete-description"
          className="page-panel tasks-delete"
        >
          <h2 id="task-delete-title">Delete “{deleting.title}”?</h2>
          <p id="task-delete-description">
            This removes this task and its notes from this device. This cannot be undone.
          </p>
          <div className="task-actions">
            <button
              type="button"
              disabled={disabled}
              onClick={() =>
                void mutate(async () => {
                  await client.remove(deleting);
                  return null;
                }, "remove")
              }
            >
              Delete task
            </button>
            <button
              type="button"
              disabled={busy}
              onClick={() => {
                setDeleting(null);
              }}
            >
              Keep task
            </button>
          </div>
        </section>
      )}
    </section>
  );
}
