import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { TasksPage } from "./TasksPage";
import type { LocalTask, LocalTasksClient } from "../../infrastructure/tauri/local-tasks-client";
const alpha: LocalTask = {
  id: 1,
  revision: 1,
  title: "Alpha",
  notes: "Private task note",
  dueDate: "2028-02-29",
  priority: "high",
  completed: false,
  createdMs: 1,
  updatedMs: 1,
};
function client(rows: readonly LocalTask[] = [alpha]): LocalTasksClient {
  return {
    available: true,
    list: vi.fn<LocalTasksClient["list"]>(() => Promise.resolve(rows)),
    save: vi.fn<LocalTasksClient["save"]>((input) =>
      Promise.resolve({
        ...alpha,
        ...input,
        id: input.id ?? 2,
        revision: (input.expectedRevision ?? 0) + 1,
      }),
    ),
    complete: vi.fn<LocalTasksClient["complete"]>((task, completed) =>
      Promise.resolve({ ...task, completed, revision: task.revision + 1 }),
    ),
    remove: vi.fn<LocalTasksClient["remove"]>(() => Promise.resolve()),
  };
}
function deferred<T>() {
  let resolve!: (v: T) => void;
  const promise = new Promise<T>((r) => {
    resolve = r;
  });
  return { promise, resolve };
}
describe("Tasks workspace", () => {
  it("creates a task with notes, a timezone-free due date and priority", async () => {
    const c = client([]);
    render(<TasksPage client={c} />);
    expect(await screen.findByText("No tasks yet. Add your first next step.")).toBeVisible();
    fireEvent.change(screen.getByLabelText("Title"), { target: { value: "Call dentist" } });
    fireEvent.change(screen.getByLabelText("Notes"), {
      target: { value: "Ask about appointment" },
    });
    fireEvent.change(screen.getByLabelText("Due date"), { target: { value: "2028-02-29" } });
    fireEvent.change(screen.getByLabelText("Priority"), { target: { value: "high" } });
    fireEvent.click(screen.getByRole("button", { name: "Save task" }));
    await waitFor(() => {
      expect(c.save).toHaveBeenCalledWith({
        id: null,
        expectedRevision: null,
        title: "Call dentist",
        notes: "Ask about appointment",
        dueDate: "2028-02-29",
        priority: "high",
      });
    });
    expect(await screen.findByRole("heading", { name: "Call dentist" })).toBeVisible();
    expect(screen.getByLabelText("Title")).toHaveValue("");
  });
  it("edits with the frozen revision and keeps the draft on conflict", async () => {
    const c = client();
    vi.mocked(c.save).mockRejectedValue("conflict");
    render(<TasksPage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: "Edit Alpha" }));
    fireEvent.change(screen.getByLabelText("Title"), { target: { value: "Edited" } });
    fireEvent.click(screen.getByRole("button", { name: "Save task" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("This task changed");
    expect(c.save).toHaveBeenCalledWith(
      expect.objectContaining({ id: 1, expectedRevision: 1, title: "Edited" }),
    );
    expect(screen.getByLabelText("Title")).toHaveValue("Edited");
    expect(screen.getByRole("heading", { name: "Alpha" })).toBeVisible();
  });
  it("completes and reopens tasks without conflating status filters", async () => {
    const c = client();
    render(<TasksPage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: "Complete Alpha" }));
    await screen.findByText("Task completed.");
    expect(screen.queryByRole("heading", { name: "Alpha" })).toBeNull();
    fireEvent.change(screen.getByLabelText("Status"), { target: { value: "completed" } });
    fireEvent.click(screen.getByRole("button", { name: "Reopen Alpha" }));
    await screen.findByText("Task reopened.");
    expect(c.complete).toHaveBeenLastCalledWith(
      expect.objectContaining({ revision: 2, completed: true }),
      false,
    );
  });
  it("requires exact task confirmation and does not delete when kept", async () => {
    const c = client();
    render(<TasksPage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: "Delete Alpha" }));
    const dialog = screen.getByRole("alertdialog", { name: "Delete “Alpha”?" });
    expect(c.remove).not.toHaveBeenCalled();
    fireEvent.click(within(dialog).getByRole("button", { name: "Keep task" }));
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(c.remove).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Delete Alpha" }));
    fireEvent.click(screen.getByRole("button", { name: "Delete task" }));
    await screen.findByText("Task deleted.");
    expect(c.remove).toHaveBeenCalledWith(alpha);
    expect(screen.queryByRole("heading", { name: "Alpha" })).toBeNull();
  });
  it("filters title/notes, priority and completion locally", async () => {
    const beta: LocalTask = {
      ...alpha,
      id: 2,
      title: "Beta",
      notes: "Follow up",
      priority: "low",
      completed: true,
    };
    render(<TasksPage client={client([alpha, beta])} />);
    await screen.findByRole("heading", { name: "Alpha" });
    fireEvent.change(screen.getByLabelText("Status"), { target: { value: "all" } });
    fireEvent.change(screen.getByLabelText("Search tasks"), { target: { value: "follow UP" } });
    expect(screen.getByRole("heading", { name: "Beta" })).toBeVisible();
    expect(screen.queryByRole("heading", { name: "Alpha" })).toBeNull();
    fireEvent.change(screen.getByLabelText("Filter priority"), { target: { value: "high" } });
    expect(screen.getByText("No tasks match these filters.")).toBeVisible();
  });
  it("does not dispatch duplicate writes while an operation is pending", async () => {
    const c = client();
    const result = deferred<LocalTask>();
    vi.mocked(c.complete).mockReturnValue(result.promise);
    render(<TasksPage client={c} />);
    const button = await screen.findByRole("button", { name: "Complete Alpha" });
    fireEvent.click(button);
    fireEvent.click(button);
    expect(c.complete).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("button", { name: "Reload tasks" })).toBeDisabled();
    result.resolve({ ...alpha, completed: true, revision: 2 });
    await screen.findByText("Task completed.");
  });
  it("ignores late hydration from a previous client", async () => {
    const old = deferred<readonly LocalTask[]>();
    const a = client();
    vi.mocked(a.list).mockReturnValue(old.promise);
    const b = client([{ ...alpha, title: "New store" }]);
    const page = render(<TasksPage client={a} />);
    page.rerender(<TasksPage client={b} />);
    await screen.findByRole("heading", { name: "New store" });
    old.resolve([alpha]);
    await waitFor(() => {
      expect(screen.queryByRole("heading", { name: "Alpha" })).toBeNull();
    });
  });
  it("blocks old-store actions until the new client hydrates", async () => {
    const a = client();
    const b = client();
    const pending = deferred<readonly LocalTask[]>();
    vi.mocked(b.list).mockReturnValue(pending.promise);
    const page = render(<TasksPage client={a} />);
    await screen.findByRole("heading", { name: "Alpha" });
    page.rerender(<TasksPage client={b} />);
    expect(screen.queryByRole("button", { name: "Complete Alpha" })).toBeNull();
    expect(screen.getByRole("button", { name: "Save task" })).toBeDisabled();
    expect(b.complete).not.toHaveBeenCalled();
    pending.resolve([{ ...alpha, title: "New store" }]);
    await screen.findByRole("heading", { name: "New store" });
  });
  it("allows retry after initial hydration fails without enabling writes", async () => {
    const c = client();
    vi.mocked(c.list).mockRejectedValueOnce("storage").mockResolvedValueOnce([alpha]);
    render(<TasksPage client={c} />);
    await screen.findByRole("alert");
    expect(screen.getByRole("button", { name: "Save task" })).toBeDisabled();
    const reload = screen.getByRole("button", { name: "Reload tasks" });
    expect(reload).not.toBeDisabled();
    fireEvent.click(reload);
    await screen.findByRole("heading", { name: "Alpha" });
    expect(screen.getByRole("button", { name: "Edit Alpha" })).not.toBeDisabled();
  });
  it("announces load failure and preserves existing rows on failed reload", async () => {
    const c = client();
    render(<TasksPage client={c} />);
    await screen.findByRole("heading", { name: "Alpha" });
    vi.mocked(c.list).mockRejectedValue("raw/private/exception");
    fireEvent.click(screen.getByRole("button", { name: "Reload tasks" }));
    expect(await screen.findByRole("alert")).not.toHaveTextContent("private");
    expect(screen.getByRole("heading", { name: "Alpha" })).toBeVisible();
  });
  it("shows desktop-only storage honestly without a fake persistence fallback", () => {
    const c = client();
    render(<TasksPage client={{ ...c, available: false }} />);
    expect(screen.getByText(/Browser preview does not persist/)).toBeVisible();
    expect(screen.getByRole("button", { name: "Save task" })).toBeDisabled();
    expect(c.list).not.toHaveBeenCalled();
  });
});
