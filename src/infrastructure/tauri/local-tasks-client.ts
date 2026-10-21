import { invoke, isTauri } from "@tauri-apps/api/core";

export type TaskPriority = "low" | "normal" | "high";
export interface LocalTask {
  readonly id: number;
  readonly revision: number;
  readonly title: string;
  readonly notes: string;
  readonly dueDate: string | null;
  readonly priority: TaskPriority;
  readonly completed: boolean;
  readonly createdMs: number;
  readonly updatedMs: number;
}
export interface TaskInput {
  readonly id: number | null;
  readonly expectedRevision: number | null;
  readonly title: string;
  readonly notes: string;
  readonly dueDate: string | null;
  readonly priority: TaskPriority;
}
export interface LocalTasksClient {
  readonly available: boolean;
  readonly list: () => Promise<readonly LocalTask[]>;
  readonly save: (input: TaskInput) => Promise<LocalTask>;
  readonly complete: (task: LocalTask, completed: boolean) => Promise<LocalTask>;
  readonly remove: (task: LocalTask) => Promise<void>;
}
const priorities: readonly string[] = ["low", "normal", "high"];
const keys = [
  "id",
  "revision",
  "title",
  "notes",
  "dueDate",
  "priority",
  "completed",
  "createdMs",
  "updatedMs",
];
function integer(v: unknown, min = 0): v is number {
  return typeof v === "number" && Number.isSafeInteger(v) && v >= min;
}
function bounded(v: unknown, bytes: number): v is string {
  return typeof v === "string" && new TextEncoder().encode(v).length <= bytes;
}
export function validTaskDate(value: string): boolean {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) return false;
  const year = Number(value.slice(0, 4));
  const month = Number(value.slice(5, 7));
  const day = Number(value.slice(8));
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  const days = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  return year > 0 && month >= 1 && month <= 12 && day >= 1 && day <= (days[month - 1] ?? 0);
}
export function parseLocalTask(value: unknown): LocalTask {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("protocol");
  const o = value as Record<string, unknown>;
  if (
    Object.keys(o).length !== keys.length ||
    keys.some((k) => !Object.hasOwn(o, k)) ||
    !integer(o["id"], 1) ||
    !integer(o["revision"], 1) ||
    !bounded(o["title"], 240) ||
    !o["title"].trim() ||
    /[\p{Cc}]/u.test(o["title"]) ||
    !bounded(o["notes"], 8192) ||
    /[\p{Cc}]/u.test(o["notes"].replaceAll("\n", "").replaceAll("\t", "")) ||
    (o["dueDate"] !== null && (typeof o["dueDate"] !== "string" || !validTaskDate(o["dueDate"]))) ||
    typeof o["priority"] !== "string" ||
    !priorities.includes(o["priority"]) ||
    typeof o["completed"] !== "boolean" ||
    !integer(o["createdMs"]) ||
    !integer(o["updatedMs"]) ||
    o["updatedMs"] < o["createdMs"]
  )
    throw Error("protocol");
  return value as LocalTask;
}
export function parseLocalTasks(value: unknown): readonly LocalTask[] {
  if (!Array.isArray(value) || value.length > 500) throw Error("protocol");
  const tasks = value.map(parseLocalTask);
  if (new Set(tasks.map((t) => t.id)).size !== tasks.length) throw Error("protocol");
  return tasks;
}
export function taskError(value: unknown): string {
  switch (value) {
    case "invalid_request":
      return "Check the title, notes and due date, then try again.";
    case "limit":
      return "Task limit reached: use at most 500 tasks, 240 title bytes and 8,192 note bytes.";
    case "conflict":
      return "This task changed. Reload tasks before trying again. Your draft is kept.";
    case "unavailable":
      return "This task is no longer available. Reload tasks to continue.";
    default:
      return "Tasks could not be saved or loaded. Your draft is kept. Try again.";
  }
}
export function createLocalTasksClient(): LocalTasksClient {
  const available = isTauri();
  function requireDesktop() {
    if (!available) throw Error("desktop_required");
  }
  return {
    available,
    list: async () => {
      requireDesktop();
      return parseLocalTasks(await invoke<unknown>("list_local_tasks"));
    },
    save: async (request) => {
      requireDesktop();
      const task = parseLocalTask(await invoke<unknown>("save_local_task", { request }));
      if (
        (request.id !== null && task.id !== request.id) ||
        task.revision !== (request.expectedRevision ?? 0) + 1 ||
        task.title !== request.title.trim() ||
        task.notes !== request.notes ||
        task.dueDate !== request.dueDate ||
        task.priority !== request.priority ||
        (request.id === null && task.completed)
      )
        throw Error("protocol");
      return task;
    },
    complete: async (task, completed) => {
      requireDesktop();
      const request = { id: task.id, expectedRevision: task.revision, completed };
      const result = parseLocalTask(
        await invoke<unknown>("set_local_task_completion", { request }),
      );
      if (
        result.id !== task.id ||
        result.revision !== task.revision + 1 ||
        result.completed !== completed
      )
        throw Error("protocol");
      return result;
    },
    remove: async (task) => {
      requireDesktop();
      const request = { id: task.id, expectedRevision: task.revision };
      const result = await invoke<unknown>("remove_local_task", { request });
      if (result !== null) throw Error("protocol");
    },
  };
}
