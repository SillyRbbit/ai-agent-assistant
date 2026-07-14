import { describe, expect, it } from "vitest";

import { createActivityEvent, type ActivityEventKind } from "./activity";

const EVENT_KINDS: readonly ActivityEventKind[] = [
  "run-started",
  "run-stopped",
  "run-failed",
  "approval-requested",
  "approval-approved",
  "approval-rejected",
  "approval-edit-requested",
];

describe("createActivityEvent", () => {
  it.each(EVENT_KINDS)("creates fixed redacted copy for %s", (kind) => {
    const event = createActivityEvent(1, "mock-run-7", kind);

    expect(event).toMatchObject({ id: "activity-1", kind, runId: "mock-run-7" });
    expect(JSON.stringify(event)).not.toContain("board update");
    expect(JSON.stringify(event)).not.toContain("sensitive provider detail");
  });

  it("rejects invalid ordinals and non-mock run identifiers", () => {
    expect(createActivityEvent(0, "mock-run-1", "run-started")).toBeNull();
    expect(createActivityEvent(Number.NaN, "mock-run-1", "run-started")).toBeNull();
    expect(createActivityEvent(1, "external-run", "run-started")).toBeNull();
  });
});
