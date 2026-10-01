import { afterEach, describe, expect, it, vi } from "vitest";
import { CollaborationSnapshots } from "./collaborationSnapshots";
import type { CollaborationClient, Room } from "../../infrastructure/tauri/collaboration-client";
function deferred() {
  let resolve!: (rooms: readonly Room[]) => void;
  const promise = new Promise<readonly Room[]>((r) => {
    resolve = r;
  });
  return { promise, resolve };
}
function client(list: CollaborationClient["list"]): CollaborationClient {
  return {
    list,
    create: vi.fn(),
    prepare: vi.fn(),
    start: vi.fn(),
    cancel: vi.fn(),
    delete: vi.fn(),
  };
}
afterEach(() => vi.useRealTimers());
describe("shared collaboration snapshots", () => {
  it("deduplicates concurrent consumers and rejects a stale deletion response", async () => {
    const first = deferred(),
      second = deferred();
    const list = vi.fn().mockReturnValueOnce(first.promise).mockReturnValueOnce(second.promise);
    const api = client(list),
      store = new CollaborationSnapshots(api);
    const release = store.retain();
    const release2 = store.retain();
    expect(list).toHaveBeenCalledTimes(1);
    store.remove(1);
    void store.refresh();
    first.resolve([{ id: 1, title: "deleted", runs: [] }]);
    await first.promise;
    await Promise.resolve();
    await Promise.resolve();
    second.resolve([]);
    await second.promise;
    await Promise.resolve();
    expect(store.getSnapshot().rooms).toEqual([]);
    expect(api.start).not.toHaveBeenCalled();
    expect(api.cancel).not.toHaveBeenCalled();
    release();
    release2();
  });
  it("discards a response after last consumer leaves and refreshes on reentry", async () => {
    const first = deferred();
    const list = vi.fn().mockReturnValueOnce(first.promise).mockResolvedValue([]);
    const store = new CollaborationSnapshots(client(list));
    const release = store.retain();
    release();
    first.resolve([{ id: 8, title: "stale", runs: [] }]);
    await first.promise;
    await Promise.resolve();
    await Promise.resolve();
    expect(store.getSnapshot().rooms).toEqual([]);
    const stop = store.retain();
    await store.refresh();
    expect(list.mock.calls.length).toBeGreaterThan(1);
    stop();
  });
  it("labels loading failure unavailable without fixture fallback", async () => {
    const store = new CollaborationSnapshots(
      client(() => Promise.reject(Error("private diagnostic"))),
    );
    const release = store.retain();
    await store.refresh();
    expect(store.getSnapshot()).toEqual({ rooms: [], status: "unavailable" });
    release();
  });
});

function snapshot(
  sequence: number,
  status: "running" | "completed" = "running",
  extra = false,
): readonly Room[] {
  const run = {
    id: "room-1/run-1",
    sequence,
    status,
    stages: [],
    error: null,
    input: { workflow: "research" as const, objective: "synthetic", sources: [] },
  };
  return [
    {
      id: 1,
      title: "synthetic",
      runs: extra ? [run, { ...run, id: "room-1/run-2", sequence: 0 }] : [run],
    },
  ];
}
it("keeps one bounded poll loop, discovers new runs, rejects sequence rollback and stops at zero consumers", async () => {
  vi.useFakeTimers();
  const list = vi.fn().mockResolvedValue(snapshot(2));
  const api = client(list),
    store = new CollaborationSnapshots(api);
  const release = store.retain(),
    release2 = store.retain();
  await vi.advanceTimersByTimeAsync(0);
  expect(list).toHaveBeenCalledTimes(1);
  await vi.advanceTimersByTimeAsync(399);
  expect(list).toHaveBeenCalledTimes(1);
  list.mockResolvedValue(snapshot(3, "completed", true));
  await vi.advanceTimersByTimeAsync(1);
  expect(store.getSnapshot().rooms[0]?.runs).toHaveLength(2);
  list.mockResolvedValue(snapshot(1));
  await store.refresh();
  expect(store.getSnapshot().rooms[0]?.runs[0]?.sequence).toBe(3);
  expect(store.getSnapshot().status).toBe("stale");
  release();
  release2();
  const count = list.mock.calls.length;
  await vi.advanceTimersByTimeAsync(5000);
  expect(list).toHaveBeenCalledTimes(count);
  expect(api.start).not.toHaveBeenCalled();
  expect(api.cancel).not.toHaveBeenCalled();
});
it("accepts authoritative mutation immediately and rejects the old cross-run read", async () => {
  const old = deferred();
  const list = vi
    .fn()
    .mockReturnValueOnce(old.promise)
    .mockResolvedValue(snapshot(5, "completed", true));
  const store = new CollaborationSnapshots(client(list)),
    release = store.retain();
  const room = snapshot(5, "completed", true)[0];
  if (!room) throw Error("missing test room");
  store.acceptRoom(room);
  expect(store.getSnapshot().rooms[0]?.runs).toHaveLength(2);
  void store.refresh();
  old.resolve(snapshot(0));
  await old.promise;
  await Promise.resolve();
  await Promise.resolve();
  expect(store.getSnapshot().rooms[0]?.runs[0]?.sequence).toBe(5);
  release();
});
