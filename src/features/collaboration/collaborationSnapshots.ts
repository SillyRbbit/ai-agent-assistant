import { useEffect, useMemo, useSyncExternalStore } from "react";
import {
  collaborationClient,
  newerRooms,
  type CollaborationClient,
  type Room,
} from "../../infrastructure/tauri/collaboration-client";

export interface CollaborationSnapshot {
  readonly rooms: readonly Room[];
  readonly status: "loading" | "current" | "stale" | "unavailable";
}
// One store per typed client. No generation methods are invoked by subscriptions.
export class CollaborationSnapshots {
  private value: CollaborationSnapshot = { rooms: [], status: "loading" };
  private listeners = new Set<() => void>();
  private consumers = 0;
  private epoch = 0;
  private pending: Promise<void> | null = null;
  private refreshAgain = false;
  private timer: ReturnType<typeof setTimeout> | null = null;
  constructor(private readonly client: CollaborationClient) {}
  readonly getSnapshot = () => this.value;
  readonly subscribe = (listener: () => void) => {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  };
  private publish(value: CollaborationSnapshot) {
    this.value = value;
    for (const listener of this.listeners) listener();
  }
  private clearTimer() {
    if (this.timer !== null) clearTimeout(this.timer);
    this.timer = null;
  }
  readonly retain = () => {
    this.consumers++;
    if (this.consumers === 1) void this.refresh();
    return () => {
      this.consumers--;
      if (this.consumers === 0) {
        this.epoch++;
        this.clearTimer();
        this.refreshAgain = false;
      }
    };
  };
  // Mutations invalidate in-flight reads immediately, even before IPC settles.
  readonly invalidate = () => {
    this.epoch++;
    this.clearTimer();
  };
  readonly acceptRoom = (room: Room) => {
    this.invalidate();
    const rooms = this.value.rooms.some((r) => r.id === room.id)
      ? this.value.rooms.map((r) => (r.id === room.id ? room : r))
      : [...this.value.rooms, room];
    this.publish({ rooms: newerRooms(this.value.rooms, rooms), status: "current" });
  };
  readonly remove = (id: number) => {
    this.invalidate();
    this.publish({ ...this.value, rooms: this.value.rooms.filter((r) => r.id !== id) });
  };
  readonly refresh = (): Promise<void> => {
    this.clearTimer();
    if (this.pending) {
      this.refreshAgain = true;
      return this.pending;
    }
    const epoch = this.epoch;
    this.pending = this.client
      .list()
      .then((rooms) => {
        if (epoch !== this.epoch || !this.consumers) return;
        const accepted = newerRooms(this.value.rooms, rooms);
        this.publish({ rooms: accepted, status: accepted === rooms ? "current" : "stale" });
      })
      .catch(() => {
        if (epoch === this.epoch && this.consumers)
          this.publish({
            ...this.value,
            status: this.value.rooms.length ? "stale" : "unavailable",
          });
      })
      .finally(() => {
        this.pending = null;
        if (!this.consumers) return;
        if (this.refreshAgain) {
          this.refreshAgain = false;
          void this.refresh();
          return;
        }
        if (
          this.value.rooms.some((r) =>
            r.runs.some((run) => run.status === "running" || run.status === "queued"),
          )
        )
          this.timer = setTimeout(() => {
            void this.refresh();
          }, 400);
      });
    return this.pending;
  };
}
const stores = new WeakMap<CollaborationClient, CollaborationSnapshots>();
export function snapshotsFor(client: CollaborationClient) {
  let store = stores.get(client);
  if (!store) {
    store = new CollaborationSnapshots(client);
    stores.set(client, store);
  }
  return store;
}
export function useCollaborationSnapshots(client = collaborationClient) {
  const store = useMemo(() => snapshotsFor(client), [client]);
  const snapshot = useSyncExternalStore(store.subscribe, store.getSnapshot);
  useEffect(() => store.retain(), [store]);
  return { ...snapshot, store };
}
