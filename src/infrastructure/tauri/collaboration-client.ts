import { parseOrigin, type Origin } from "./knowledge-client";
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
  coding_action: ["coding", "qa-validation"],
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
  readonly origin?: Origin;
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
export interface ActionSelection {
  readonly selection: number;
  readonly file: string;
  readonly testFile: string;
}
export interface SelectedRepository {
  readonly selection: number;
  readonly path: string;
}
export interface ActionCheck {
  readonly command: string;
  readonly exit: number | null;
  readonly output: string;
  readonly passed: boolean;
  readonly image: string;
  readonly candidateHash: string;
  readonly testHash: string;
  readonly containerId: string;
  readonly processAbsent: boolean;
}
export interface ActionAttempt {
  readonly candidate: string;
  readonly candidateHash: string;
  readonly diff: string;
  readonly check: ActionCheck;
  readonly qaSummary: string;
  readonly qaHandoff: Handoff | null;
}
export interface ActionEvidence {
  readonly file: string;
  readonly testFile: string;
  readonly baseline: string;
  readonly original: string;
  readonly tests: string;
  readonly attempts: readonly ActionAttempt[];
  readonly reviewHash: string | null;
  readonly disposition: string;
  readonly recovery: string;
  readonly requests: number;
}
const IMAGE =
  "docker.io/library/python@sha256:739ba32ae445e8d58f3d90feb85f83bebc8346f8dd280fa1eb5848f4ff1ed163";
const hash = (v: unknown): v is string => typeof v === "string" && /^[a-f0-9]{64}$/.test(v);
function keys(v: Record<string, unknown>, allowed: readonly string[]): boolean {
  return (
    Object.keys(v).length === allowed.length && Object.keys(v).every((k) => allowed.includes(k))
  );
}
function parseAction(raw: unknown): ActionEvidence {
  const e = obj(raw);
  if (
    !keys(e, [
      "file",
      "testFile",
      "baseline",
      "original",
      "tests",
      "attempts",
      "reviewHash",
      "disposition",
      "recovery",
      "requests",
    ]) ||
    !txt(e["file"], 64) ||
    !txt(e["testFile"], 64) ||
    !hash(e["baseline"]) ||
    !txt(e["original"], 8192) ||
    !txt(e["tests"], 8192) ||
    !Array.isArray(e["attempts"]) ||
    e["attempts"].length > 2 ||
    !(e["reviewHash"] === null || hash(e["reviewHash"])) ||
    ![
      "prepared",
      "review_ready",
      "validation_failed",
      "applying",
      "applied",
      "recovery_required",
      "rejected_or_expired",
      "cancelled",
    ].includes(String(e["disposition"])) ||
    !txt(e["recovery"], 4096) ||
    !integer(e["requests"]) ||
    e["requests"] > 4
  )
    throw Error("protocol");
  for (const rawAttempt of e["attempts"]) {
    const a = obj(rawAttempt),
      c = obj(a["check"]);
    if (
      !keys(a, ["candidate", "candidateHash", "diff", "check", "qaSummary", "qaHandoff"]) ||
      !txt(a["candidate"], 8192) ||
      !hash(a["candidateHash"]) ||
      !txt(a["diff"], 33000) ||
      !txt(a["qaSummary"], 4000) ||
      !keys(c, [
        "command",
        "exit",
        "output",
        "passed",
        "image",
        "candidateHash",
        "testHash",
        "containerId",
        "processAbsent",
      ]) ||
      !txt(c["command"], 512) ||
      !(
        c["exit"] === null ||
        (Number.isSafeInteger(c["exit"]) && Number(c["exit"]) >= 0 && Number(c["exit"]) <= 255)
      ) ||
      !txt(c["output"], 49152) ||
      typeof c["passed"] !== "boolean" ||
      c["image"] !== IMAGE ||
      !hash(c["candidateHash"]) ||
      !hash(c["testHash"]) ||
      !hash(c["containerId"]) ||
      typeof c["processAbsent"] !== "boolean" ||
      c["candidateHash"] !== a["candidateHash"] ||
      (c["passed"] && (c["exit"] !== 0 || !c["processAbsent"]))
    )
      throw Error("protocol");
    if (a["qaHandoff"] !== null) {
      const q = obj(a["qaHandoff"]);
      if (
        q["version"] !== 1 ||
        q["stage"] !== 1 ||
        q["agentId"] !== "qa-validation" ||
        !["complete", "partial"].includes(String(q["status"])) ||
        !txt(q["summary"], 4000) ||
        !strings(q["findings"], 8, 1000) ||
        !strings(q["limitations"], 8, 1000) ||
        !strings(q["evidence"], 0, 32)
      )
        throw Error("protocol");
    }
  }
  if (e["reviewHash"] !== null) {
    const last: unknown = e["attempts"].at(-1);
    if (
      !last ||
      obj(obj(last)["check"])["passed"] !== true ||
      obj(obj(last)["qaHandoff"])["status"] !== "complete"
    )
      throw Error("protocol");
  }
  return raw as ActionEvidence;
}
export interface Run {
  readonly action?: ActionEvidence;
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
    if (s["origin"] !== undefined) parseOrigin(s["origin"]);
    labels.add(s["label"]);
  }
  if (input["workflow"] === "coding_action") {
    parseAction(v["action"]);
    if (input["sources"].length) throw Error("protocol");
  } else if (v["action"] !== undefined) throw Error("protocol");
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
  readonly prepare: (
    roomId: number,
    input: Objective,
    action?: ActionSelection,
  ) => Promise<Preview>;
  readonly selectActionRepository?: () => Promise<SelectedRepository>;
  readonly reviewAction?: (roomId: number, runId: string, reviewHash: string) => Promise<Room>;
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
  async selectActionRepository() {
    native();
    const v = obj(await invoke<unknown>("select_action_repository"));
    if (
      !keys(v, ["selection", "path"]) ||
      !integer(v["selection"]) ||
      v["selection"] === 0 ||
      !txt(v["path"], 4096)
    )
      throw Error("protocol");
    return v as unknown as SelectedRepository;
  },
  async reviewAction(roomId, runId, reviewHash) {
    native();
    return one(
      await invoke<unknown>("review_action_change", { request: { roomId, runId, reviewHash } }),
    );
  },
  async prepare(roomId, input, action) {
    native();
    const v = obj(
      await invoke<unknown>("prepare_collaboration", {
        request: { roomId, input, ...(action ? { action } : {}) },
      }),
    );
    const run = parseRun(v["run"]);
    if (
      !integer(v["previewSerial"]) ||
      v["previewSerial"] === 0 ||
      v["roomId"] !== roomId ||
      v["maximumCalls"] !== (run.input.workflow === "coding_action" ? 4 : run.stages.length) ||
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

export function actionError(code: string): string | undefined {
  const messages: Record<string, string> = {
    timeout: "The bounded validation deadline expired; no application was performed.",
    limit: "The action exceeded its file, output or correction limit; no automatic retry.",
    cancelled: "Isolated execution cancelled; target unchanged.",
    scope:
      "This target or edit is outside the supported new-file Python scope; the target filename must not already exist.",
    target: "Choose a clean, small, ordinary Git repository with no extra files.",
    drift: "The target or reviewed evidence changed. No automatic retry or application.",
    isolation:
      "Isolated validation or cleanup could not be verified. Stop and inspect retained evidence.",
    validation:
      "Validation did not pass. Failed attempts remain available; application is blocked.",
    approval: "Native approval was unavailable, rejected or expired.",
    recovery:
      "Application needs recovery review. Preserve the backup and journals; no automatic replay.",
    storage: "Action evidence could not be saved. Stop and preserve the target.",
  };
  return messages[code];
}
