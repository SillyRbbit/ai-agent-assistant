export type ActivityEventKind =
  | "approval-approved"
  | "approval-edit-requested"
  | "approval-rejected"
  | "approval-requested"
  | "run-failed"
  | "run-started"
  | "run-stopped";

export type ActivityEventTone = "danger" | "neutral" | "success" | "warning";

export interface ActivityEvent {
  readonly detail: string;
  readonly id: string;
  readonly kind: ActivityEventKind;
  readonly runId: string;
  readonly summary: string;
  readonly tone: ActivityEventTone;
}

interface ActivityEventCopy {
  readonly detail: string;
  readonly summary: string;
  readonly tone: ActivityEventTone;
}

const EVENT_COPY: Readonly<Record<ActivityEventKind, ActivityEventCopy>> = {
  "approval-approved": {
    detail: "A mock approval decision was recorded. No tool was executed.",
    summary: "Mock action approved",
    tone: "success",
  },
  "approval-edit-requested": {
    detail: "The mock action was returned to the composer. No tool was executed.",
    summary: "Mock action returned for editing",
    tone: "warning",
  },
  "approval-rejected": {
    detail: "The mock action was rejected. No local data was changed.",
    summary: "Mock action rejected",
    tone: "neutral",
  },
  "approval-requested": {
    detail: "A fixed in-memory action preview is waiting for review.",
    summary: "Mock approval requested",
    tone: "warning",
  },
  "run-failed": {
    detail: "The local mock run ended with a bounded error. No action was executed.",
    summary: "Mock run failed",
    tone: "danger",
  },
  "run-started": {
    detail: "A deterministic in-memory run started.",
    summary: "Mock run started",
    tone: "neutral",
  },
  "run-stopped": {
    detail: "The active mock run was cancelled before completion.",
    summary: "Mock run stopped",
    tone: "neutral",
  },
};

const MOCK_RUN_ID_PATTERN = /^mock-run-[1-9][0-9]*$/;

export function createActivityEvent(
  ordinal: number,
  runId: string,
  kind: ActivityEventKind,
): ActivityEvent | null {
  if (!Number.isSafeInteger(ordinal) || ordinal < 1 || !MOCK_RUN_ID_PATTERN.test(runId)) {
    return null;
  }

  return {
    ...EVENT_COPY[kind],
    id: `activity-${String(ordinal)}`,
    kind,
    runId,
  };
}
