import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT =
  "research-knowledge-demo-lifecycle-v1" as const;
export const RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE =
  "DEMO MODE · SIMULATED AGENT DATA" as const;
export const RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY =
  "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs." as const;
export const RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE =
  "Research/Knowledge demo lifecycle unavailable." as const;

export type ResearchKnowledgeDemoLifecycleState =
  | "idle"
  | "research"
  | "knowledge"
  | "synthesis"
  | "succeeded"
  | "failed"
  | "cancelled"
  | "cleanup-pending";

export type ResearchKnowledgeDemoLifecycleEventKind =
  | "research-started"
  | "research-completed"
  | "knowledge-started"
  | "knowledge-completed"
  | "synthesis-started"
  | "completed"
  | "failed"
  | "cancelled"
  | "cleanup-pending";

export interface ResearchKnowledgeDemoLifecycleEntry {
  readonly revision: number;
  readonly kind: ResearchKnowledgeDemoLifecycleEventKind;
}

export interface ResearchKnowledgeDemoLifecycleSnapshot {
  readonly schemaVersion: "research-knowledge-demo-lifecycle-v1";
  readonly scenarioId: "research-knowledge-demo-v1";
  readonly disclosure: typeof RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE;
  readonly proofBoundary: typeof RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY;
  readonly fixtureProvenance: "application-owned-synthetic-fixture";
  readonly presentationEpoch: number;
  readonly revision: number;
  readonly state: ResearchKnowledgeDemoLifecycleState;
  readonly journal: readonly ResearchKnowledgeDemoLifecycleEntry[];
}

export interface ResearchKnowledgeDemoLifecycleClient {
  readonly current: ResearchKnowledgeDemoLifecycleSnapshot | undefined;
  readonly recoveryRequired: boolean;
  snapshot(): Promise<ResearchKnowledgeDemoLifecycleSnapshot>;
  start(): Promise<ResearchKnowledgeDemoLifecycleSnapshot>;
  advance(): Promise<ResearchKnowledgeDemoLifecycleSnapshot>;
  cancel(): Promise<ResearchKnowledgeDemoLifecycleSnapshot>;
  dispose(): void;
}

const SNAPSHOT_FIELDS = [
  "schemaVersion",
  "scenarioId",
  "disclosure",
  "proofBoundary",
  "fixtureProvenance",
  "presentationEpoch",
  "revision",
  "state",
  "journal",
] as const;
const ENTRY_FIELDS = ["revision", "kind"] as const;
const STATES = new Set<ResearchKnowledgeDemoLifecycleState>([
  "idle",
  "research",
  "knowledge",
  "synthesis",
  "succeeded",
  "failed",
  "cancelled",
  "cleanup-pending",
]);
const EVENT_KINDS = new Set<ResearchKnowledgeDemoLifecycleEventKind>([
  "research-started",
  "research-completed",
  "knowledge-started",
  "knowledge-completed",
  "synthesis-started",
  "completed",
  "failed",
  "cancelled",
  "cleanup-pending",
]);
const TERMINAL_STATES = new Set<ResearchKnowledgeDemoLifecycleState>([
  "succeeded",
  "failed",
  "cancelled",
]);
const JOURNAL_GRAMMAR: Readonly<
  Record<
    ResearchKnowledgeDemoLifecycleState,
    readonly (readonly ResearchKnowledgeDemoLifecycleEventKind[])[]
  >
> = {
  idle: [[]],
  research: [["research-started"]],
  knowledge: [["research-started", "research-completed", "knowledge-started"]],
  synthesis: [
    [
      "research-started",
      "research-completed",
      "knowledge-started",
      "knowledge-completed",
      "synthesis-started",
    ],
  ],
  succeeded: [
    [
      "research-started",
      "research-completed",
      "knowledge-started",
      "knowledge-completed",
      "synthesis-started",
      "completed",
    ],
  ],
  failed: [
    [
      "research-started",
      "research-completed",
      "knowledge-started",
      "knowledge-completed",
      "synthesis-started",
      "failed",
    ],
  ],
  cancelled: [
    ["research-started", "cancelled"],
    ["research-started", "research-completed", "knowledge-started", "cancelled"],
    [
      "research-started",
      "research-completed",
      "knowledge-started",
      "knowledge-completed",
      "synthesis-started",
      "cancelled",
    ],
  ],
  "cleanup-pending": [
    ["cleanup-pending"],
    ["research-started", "cleanup-pending"],
    ["research-started", "research-completed", "knowledge-started", "cleanup-pending"],
    [
      "research-started",
      "research-completed",
      "knowledge-started",
      "knowledge-completed",
      "synthesis-started",
      "cleanup-pending",
    ],
  ],
};
const MAX_JOURNAL_ENTRIES = 8;
const MAX_PRESENTATION_EPOCH = 0xffff_ffff;
function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function hasExactKeys(record: Record<string, unknown>, fields: readonly string[]): boolean {
  const keys = Object.keys(record);
  return keys.length === fields.length && keys.every((key) => fields.includes(key));
}

function isBoundedInteger(value: unknown, maximum: number): value is number {
  return Number.isInteger(value) && typeof value === "number" && value >= 0 && value <= maximum;
}

function isState(value: unknown): value is ResearchKnowledgeDemoLifecycleState {
  return typeof value === "string" && STATES.has(value as ResearchKnowledgeDemoLifecycleState);
}

function isEventKind(value: unknown): value is ResearchKnowledgeDemoLifecycleEventKind {
  return (
    typeof value === "string" && EVENT_KINDS.has(value as ResearchKnowledgeDemoLifecycleEventKind)
  );
}

function journalMatchesState(
  state: ResearchKnowledgeDemoLifecycleState,
  journal: readonly ResearchKnowledgeDemoLifecycleEntry[],
): boolean {
  return JOURNAL_GRAMMAR[state].some(
    (expected) =>
      expected.length === journal.length &&
      expected.every((kind, index) => journal[index]?.kind === kind),
  );
}

export function parseResearchKnowledgeDemoLifecycleSnapshot(
  value: unknown,
): ResearchKnowledgeDemoLifecycleSnapshot | undefined {
  if (!isRecord(value) || !hasExactKeys(value, SNAPSHOT_FIELDS)) return undefined;
  if (
    value["schemaVersion"] !== "research-knowledge-demo-lifecycle-v1" ||
    value["scenarioId"] !== "research-knowledge-demo-v1" ||
    value["disclosure"] !== RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE ||
    value["proofBoundary"] !== RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY ||
    value["fixtureProvenance"] !== "application-owned-synthetic-fixture" ||
    !isBoundedInteger(value["presentationEpoch"], MAX_PRESENTATION_EPOCH) ||
    !isBoundedInteger(value["revision"], MAX_JOURNAL_ENTRIES) ||
    !isState(value["state"]) ||
    !Array.isArray(value["journal"]) ||
    value["journal"].length !== value["revision"]
  ) {
    return undefined;
  }

  const journal: ResearchKnowledgeDemoLifecycleEntry[] = [];
  for (const [index, entry] of value["journal"].entries()) {
    if (
      !isRecord(entry) ||
      !hasExactKeys(entry, ENTRY_FIELDS) ||
      entry["revision"] !== index + 1 ||
      !isEventKind(entry["kind"])
    ) {
      return undefined;
    }
    journal.push(Object.freeze({ revision: index + 1, kind: entry["kind"] }));
  }
  if (
    !journalMatchesState(value["state"], journal) ||
    (value["state"] === "idle"
      ? value["presentationEpoch"] !== 0
      : value["presentationEpoch"] === 0)
  ) {
    return undefined;
  }

  return Object.freeze({
    schemaVersion: "research-knowledge-demo-lifecycle-v1",
    scenarioId: "research-knowledge-demo-v1",
    disclosure: RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE,
    proofBoundary: RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY,
    fixtureProvenance: "application-owned-synthetic-fixture",
    presentationEpoch: value["presentationEpoch"],
    revision: value["revision"],
    state: value["state"],
    journal: Object.freeze(journal),
  });
}

function snapshotsEqual(
  first: ResearchKnowledgeDemoLifecycleSnapshot | undefined,
  second: ResearchKnowledgeDemoLifecycleSnapshot,
): boolean {
  return (
    first?.presentationEpoch === second.presentationEpoch &&
    first.revision === second.revision &&
    first.state === second.state &&
    first.journal.length === second.journal.length &&
    first.journal.every((entry, index) => {
      const secondEntry = second.journal[index];
      return secondEntry?.revision === entry.revision && secondEntry.kind === entry.kind;
    })
  );
}

function isDirectNotificationSuccessor(
  current: ResearchKnowledgeDemoLifecycleSnapshot,
  candidate: ResearchKnowledgeDemoLifecycleSnapshot,
): boolean {
  if (
    candidate.presentationEpoch !== current.presentationEpoch ||
    candidate.revision <= current.revision ||
    candidate.revision - current.revision > 2 ||
    TERMINAL_STATES.has(current.state)
  ) {
    return false;
  }
  const allowedState =
    (current.state === "research" &&
      (candidate.state === "knowledge" ||
        candidate.state === "cancelled" ||
        candidate.state === "cleanup-pending")) ||
    (current.state === "knowledge" &&
      (candidate.state === "synthesis" ||
        candidate.state === "cancelled" ||
        candidate.state === "cleanup-pending")) ||
    (current.state === "synthesis" &&
      (candidate.state === "succeeded" ||
        candidate.state === "failed" ||
        candidate.state === "cancelled" ||
        candidate.state === "cleanup-pending"));
  if (!allowedState) return false;
  return current.journal.every((entry, index) => {
    const candidateEntry = candidate.journal[index];
    return candidateEntry?.revision === entry.revision && candidateEntry.kind === entry.kind;
  });
}

type LifecycleRequest = "snapshot" | "start" | "advance" | "cancel";

function isExpectedOperationResponse(
  request: LifecycleRequest,
  basis: ResearchKnowledgeDemoLifecycleSnapshot | undefined,
  candidate: ResearchKnowledgeDemoLifecycleSnapshot,
): boolean {
  if (request === "snapshot") return true;
  if (basis === undefined) return false;
  if (request === "start") {
    return (
      (basis.state === "idle" || TERMINAL_STATES.has(basis.state)) &&
      candidate.presentationEpoch === basis.presentationEpoch + 1 &&
      candidate.state === "research"
    );
  }
  if (candidate.presentationEpoch !== basis.presentationEpoch) return false;
  if (request === "advance") {
    return (
      isDirectNotificationSuccessor(basis, candidate) &&
      ((basis.state === "research" && candidate.state === "knowledge") ||
        (basis.state === "knowledge" && candidate.state === "synthesis") ||
        (basis.state === "synthesis" &&
          (candidate.state === "succeeded" || candidate.state === "failed")))
    );
  }
  return (
    isDirectNotificationSuccessor(basis, candidate) &&
    (basis.state === "research" || basis.state === "knowledge" || basis.state === "synthesis") &&
    candidate.state === "cancelled"
  );
}

function closedError(): Error {
  return new Error(RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE);
}

export async function createResearchKnowledgeDemoLifecycleClient(
  onSnapshot: (snapshot: ResearchKnowledgeDemoLifecycleSnapshot) => void,
): Promise<ResearchKnowledgeDemoLifecycleClient> {
  let current: ResearchKnowledgeDemoLifecycleSnapshot | undefined;
  let recoveryRequired = false;
  let disposed = false;
  let inFlight = false;
  let unlisten: UnlistenFn;

  const acceptResponse = (
    request: LifecycleRequest,
    basis: ResearchKnowledgeDemoLifecycleSnapshot | undefined,
    value: unknown,
  ): ResearchKnowledgeDemoLifecycleSnapshot => {
    const snapshot = parseResearchKnowledgeDemoLifecycleSnapshot(value);
    if (snapshot === undefined || !isExpectedOperationResponse(request, basis, snapshot)) {
      throw closedError();
    }
    if (
      current !== undefined &&
      (snapshot.presentationEpoch < current.presentationEpoch ||
        (snapshot.presentationEpoch === current.presentationEpoch &&
          (snapshot.revision < current.revision ||
            (snapshot.revision === current.revision && !snapshotsEqual(current, snapshot)))))
    ) {
      throw closedError();
    }
    const changed = !snapshotsEqual(current, snapshot);
    current = snapshot;
    recoveryRequired = false;
    if (changed && !disposed) onSnapshot(snapshot);
    return snapshot;
  };

  try {
    unlisten = await listen<unknown>(RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_EVENT, (event) => {
      if (disposed) return;
      const candidate = parseResearchKnowledgeDemoLifecycleSnapshot(event.payload);
      if (candidate === undefined || current === undefined) return;
      if (
        candidate.presentationEpoch < current.presentationEpoch ||
        (candidate.presentationEpoch === current.presentationEpoch &&
          candidate.revision <= current.revision)
      ) {
        return;
      }
      recoveryRequired = true;
    });
  } catch {
    throw closedError();
  }

  const invokeSnapshot = async (
    requestKind: LifecycleRequest,
    request: () => Promise<unknown>,
  ): Promise<ResearchKnowledgeDemoLifecycleSnapshot> => {
    if (disposed || inFlight) throw closedError();
    inFlight = true;
    const basis = current;
    try {
      return acceptResponse(requestKind, basis, await request());
    } catch {
      throw closedError();
    } finally {
      inFlight = false;
    }
  };

  return {
    get current() {
      return current;
    },
    get recoveryRequired() {
      return recoveryRequired;
    },
    snapshot: () =>
      invokeSnapshot("snapshot", () =>
        invoke<unknown>("get_research_knowledge_demo_lifecycle_snapshot"),
      ),
    start: () =>
      invokeSnapshot("start", () => invoke<unknown>("start_research_knowledge_demo_lifecycle")),
    advance: () =>
      invokeSnapshot("advance", () => invoke<unknown>("advance_research_knowledge_demo_lifecycle")),
    cancel: () =>
      invokeSnapshot("cancel", () => invoke<unknown>("cancel_research_knowledge_demo_lifecycle")),
    dispose: () => {
      if (disposed) return;
      disposed = true;
      unlisten();
    },
  };
}
