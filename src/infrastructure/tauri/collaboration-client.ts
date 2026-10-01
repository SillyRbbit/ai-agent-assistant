import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  AGENT_IDS,
  CONNECTIONS,
  EFFORTS,
  parseBotIdentity,
  type AgentId,
  type AgentConnection,
  type AgentEffort,
  type BotIdentity,
} from "./agent-chat-client";
export const WORKFLOWS = {
  research: ["personal-assistant", "research", "knowledge-document", "personal-assistant"],
  engineering: [
    "personal-assistant",
    "coding",
    "qa-validation",
    "security-risk",
    "personal-assistant",
  ],
  operations: [
    "personal-assistant",
    "cloud-infrastructure",
    "systems-operations",
    "security-risk",
    "personal-assistant",
  ],
  workflow: ["personal-assistant", "workflow-automation", "qa-validation", "personal-assistant"],
} as const;
export type Workflow = keyof typeof WORKFLOWS;
export type Status =
  | "queued"
  | "running"
  | "completed"
  | "failed"
  | "cancelled"
  | "interrupted"
  | "partial";
export interface Source {
  readonly label: string;
  readonly text: string;
}
export interface Objective {
  readonly workflow: Workflow;
  readonly objective: string;
  readonly sources: readonly Source[];
}
export interface Participant {
  readonly agentId: AgentId;
  readonly role: string;
  readonly identity: BotIdentity;
  readonly connection: AgentConnection;
  readonly model: string;
  readonly effort: AgentEffort;
  readonly endpoint: string;
  readonly localAuth: boolean;
  readonly ownerInstructions: string;
  readonly revision: number;
}
export interface Handoff {
  readonly version: 1;
  readonly stage: number;
  readonly agentId: AgentId;
  readonly status: "complete" | "partial";
  readonly summary: string;
  readonly findings: readonly string[];
  readonly evidence: readonly string[];
  readonly limitations: readonly string[];
}
export interface Stage {
  readonly id: string;
  readonly participant: Participant;
  readonly status: Status;
  readonly timestamp: number;
  readonly provisional: string;
  readonly handoff: Handoff | null;
  readonly input: string;
}
export interface Run {
  readonly id: string;
  readonly input: Objective;
  readonly status: Status;
  readonly stages: readonly Stage[];
  readonly error: string | null;
  readonly sequence: number;
}
export interface Room {
  readonly id: number;
  readonly title: string;
  readonly runs: readonly Run[];
}
export interface Preview {
  readonly previewSerial: number;
  readonly roomId: number;
  readonly run: Run;
  readonly maximumCalls: number;
  readonly simulation: boolean;
}
const statuses: readonly string[] = [
  "queued",
  "running",
  "completed",
  "failed",
  "cancelled",
  "interrupted",
  "partial",
];
function obj(v: unknown): Record<string, unknown> {
  if (!v || typeof v !== "object" || Array.isArray(v)) throw Error("protocol");
  return v as Record<string, unknown>;
}
function txt(v: unknown, n: number): v is string {
  return typeof v === "string" && v.length <= n;
}
function integer(v: unknown): v is number {
  return typeof v === "number" && Number.isSafeInteger(v) && v >= 0;
}
function strings(v: unknown, n: number, max: number): boolean {
  return Array.isArray(v) && v.length <= n && v.every((s) => txt(s, max));
}
export function parseRun(value: unknown): Run {
  const v = obj(value),
    input = obj(v["input"]);
  if (
    !txt(v["id"], 80) ||
    !integer(v["sequence"]) ||
    !statuses.includes(String(v["status"])) ||
    !(v["error"] === null || txt(v["error"], 100)) ||
    !Object.hasOwn(WORKFLOWS, String(input["workflow"])) ||
    !txt(input["objective"], 8192) ||
    !Array.isArray(input["sources"]) ||
    input["sources"].length > 6
  )
    throw Error("protocol");
  const labels = new Set<string>();
  for (const raw of input["sources"]) {
    const s = obj(raw);
    if (!txt(s["label"], 32) || !txt(s["text"], 16384) || labels.has(s["label"]))
      throw Error("protocol");
    labels.add(s["label"]);
  }
  const route = WORKFLOWS[input["workflow"] as Workflow];
  if (!Array.isArray(v["stages"]) || v["stages"].length !== route.length) throw Error("protocol");
  for (const [i, raw] of v["stages"].entries()) {
    const s = obj(raw),
      p = obj(s["participant"]);
    parseBotIdentity(p["identity"]);
    if (
      s["id"] !== `${v["id"]}/stage/${String(i)}` ||
      p["agentId"] !== route[i] ||
      !AGENT_IDS.includes(p["agentId"] as AgentId) ||
      !txt(p["role"], 80) ||
      !CONNECTIONS.includes(p["connection"] as AgentConnection) ||
      !EFFORTS.includes(p["effort"] as AgentEffort) ||
      !txt(p["model"], 256) ||
      !txt(p["endpoint"], 2048) ||
      !txt(p["ownerInstructions"], 8192) ||
      typeof p["localAuth"] !== "boolean" ||
      !integer(p["revision"]) ||
      !statuses.includes(String(s["status"])) ||
      !integer(s["timestamp"]) ||
      !txt(s["provisional"], 16384) ||
      !txt(s["input"], 55000)
    )
      throw Error("protocol");
    if (s["handoff"] !== null) {
      const h = obj(s["handoff"]);
      if (
        h["version"] !== 1 ||
        h["stage"] !== i ||
        h["agentId"] !== p["agentId"] ||
        !["complete", "partial"].includes(String(h["status"])) ||
        !txt(h["summary"], 4000) ||
        !strings(h["findings"], 8, 1000) ||
        !strings(h["limitations"], 8, 1000) ||
        !strings(h["evidence"], 6, 32) ||
        (h["evidence"] as string[]).some((e) => !labels.has(e))
      )
        throw Error("protocol");
    }
  }
  return value as Run;
}
export function parseRooms(value: unknown): readonly Room[] {
  if (!Array.isArray(value) || value.length > 10) throw Error("protocol");
  const ids = new Set<number>();
  for (const raw of value) {
    const r = obj(raw);
    if (
      !integer(r["id"]) ||
      r["id"] === 0 ||
      ids.has(r["id"]) ||
      !txt(r["title"], 160) ||
      !Array.isArray(r["runs"]) ||
      r["runs"].length > 4
    )
      throw Error("protocol");
    ids.add(r["id"]);
    for (const [i, run] of r["runs"].entries()) {
      if (parseRun(run).id !== `room-${String(r["id"])}/run-${String(i + 1)}`)
        throw Error("protocol");
    }
  }
  return value as readonly Room[];
}
export interface CollaborationClient {
  readonly list: () => Promise<readonly Room[]>;
  readonly create: (title: string) => Promise<Room>;
  readonly prepare: (roomId: number, input: Objective) => Promise<Preview>;
  readonly start: (preview: Preview) => Promise<Room>;
  readonly cancel: (roomId: number) => Promise<Room>;
  readonly delete: (roomId: number) => Promise<void>;
}
function native(): void {
  if (!isTauri()) throw Error("native_required");
}
function one(v: unknown): Room {
  const r = parseRooms([v])[0];
  if (!r) throw Error("protocol");
  return r;
}
export const collaborationClient: CollaborationClient = {
  async list() {
    native();
    return parseRooms(await invoke<unknown>("list_collaboration_rooms"));
  },
  async create(title) {
    native();
    return one(await invoke<unknown>("create_collaboration_room", { request: { title } }));
  },
  async prepare(roomId, input) {
    native();
    const v = obj(await invoke<unknown>("prepare_collaboration", { request: { roomId, input } }));
    const run = parseRun(v["run"]);
    if (
      !integer(v["previewSerial"]) ||
      v["previewSerial"] === 0 ||
      v["roomId"] !== roomId ||
      v["maximumCalls"] !== run.stages.length ||
      typeof v["simulation"] !== "boolean"
    )
      throw Error("protocol");
    return v as unknown as Preview;
  },
  async start(preview) {
    native();
    const request = {
      roomId: preview.roomId,
      runId: preview.run.id,
      previewSerial: preview.previewSerial,
      acknowledgment: "collaboration-shared-content-v1",
    };
    return one(await invoke<unknown>("start_collaboration", { request }));
  },
  async cancel(roomId) {
    native();
    return one(await invoke<unknown>("cancel_collaboration", { request: { roomId } }));
  },
  async delete(roomId) {
    native();
    await invoke<unknown>("delete_collaboration_room", { request: { roomId } });
  },
};
// Poll snapshots are replacement state, never appended events. Late/duplicate snapshots
// cannot replace a newer generation of the same persisted run.
export function newerRooms(old: readonly Room[], next: readonly Room[]): readonly Room[] {
  for (const room of next) {
    const previous = old.find((r) => r.id === room.id);
    if (previous && previous.runs.length > room.runs.length) return old;
    for (const run of room.runs) {
      const prior = previous?.runs.find((r) => r.id === run.id);
      if (prior && run.sequence < prior.sequence) return old;
    }
  }
  return next;
}
