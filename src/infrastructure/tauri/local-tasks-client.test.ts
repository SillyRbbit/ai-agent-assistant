import { describe, expect, it, vi, beforeEach } from "vitest";
import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  createLocalTasksClient,
  parseLocalTask,
  parseLocalTasks,
  taskError,
  validTaskDate,
} from "./local-tasks-client";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(), isTauri: vi.fn(() => true) }));
const task = {
  id: 1,
  revision: 1,
  title: "Next step",
  notes: "Local",
  dueDate: "2028-02-29",
  priority: "normal",
  completed: false,
  createdMs: 1,
  updatedMs: 1,
};
beforeEach(() => {
  vi.mocked(invoke).mockReset();
  vi.mocked(isTauri).mockReturnValue(true);
});
describe("local task native boundary", () => {
  it("accepts exact bounded native records and rejects malformed/unknown responses", () => {
    expect(parseLocalTask(task)).toEqual(task);
    for (const patch of [
      { id: 0 },
      { revision: Number.MAX_SAFE_INTEGER + 1 },
      { dueDate: "2027-02-29" },
      { priority: "urgent" },
      { title: "界".repeat(81) },
      { notes: "x".repeat(8193) },
      { title: "bad\nline" },
      { notes: "bad\0" },
      { updatedMs: 0 },
      { completed: "true" },
      { tool: "execute" },
    ]) {
      expect(() => parseLocalTask({ ...task, ...patch })).toThrow("protocol");
    }
    expect(() => parseLocalTasks([task, task])).toThrow("protocol");
    expect(() => parseLocalTasks(Array(501).fill(task))).toThrow("protocol");
    expect(() => parseLocalTask(null)).toThrow("protocol");
  });
  it("does not convert date-only values through timezones", () => {
    expect(validTaskDate("2000-02-29")).toBe(true);
    for (const d of ["1900-02-29", "2026-04-31", "0000-01-01", "2026-01-01T00:00:00Z"])
      expect(validTaskDate(d)).toBe(false);
  });
  it("uses only fixed commands and exact optimistic revision requests", async () => {
    const c = createLocalTasksClient();
    vi.mocked(invoke).mockResolvedValueOnce([task]);
    await c.list();
    expect(invoke).toHaveBeenLastCalledWith("list_local_tasks");
    const t = parseLocalTask(task);
    vi.mocked(invoke).mockResolvedValueOnce({ ...task, revision: 2, completed: true });
    await c.complete(t, true);
    expect(invoke).toHaveBeenLastCalledWith("set_local_task_completion", {
      request: { id: 1, expectedRevision: 1, completed: true },
    });
    vi.mocked(invoke).mockResolvedValueOnce(null);
    await c.remove(t);
    expect(invoke).toHaveBeenLastCalledWith("remove_local_task", {
      request: { id: 1, expectedRevision: 1 },
    });
  });
  it("rejects mismatched mutation identities and revisions", async () => {
    const c = createLocalTasksClient();
    const t = parseLocalTask(task);
    vi.mocked(invoke).mockResolvedValueOnce({ ...task, id: 2, revision: 2, completed: true });
    await expect(c.complete(t, true)).rejects.toThrow("protocol");
    vi.mocked(invoke).mockResolvedValueOnce(task);
    await expect(
      c.save({
        id: 1,
        expectedRevision: 1,
        title: "Edit",
        notes: "",
        dueDate: null,
        priority: "normal",
      }),
    ).rejects.toThrow("protocol");
  });
  it("never falls back to pretend browser persistence", async () => {
    vi.mocked(isTauri).mockReturnValue(false);
    const c = createLocalTasksClient();
    expect(c.available).toBe(false);
    await expect(c.list()).rejects.toThrow("desktop_required");
    expect(invoke).not.toHaveBeenCalled();
  });
  it("does not display arbitrary native error strings", () => {
    expect(taskError("private/raw/value")).not.toContain("private");
    expect(taskError("conflict")).toContain("draft is kept");
  });
});
