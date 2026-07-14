import { describe, expect, it } from "vitest";

import { approvalOutcomeMessage, createMockRunScript } from "./mockAssistantRun";

describe("createMockRunScript", () => {
  it("creates a deterministic closed script from a normalized request", () => {
    const first = createMockRunScript(1, "  Prepare the board update  ");
    const second = createMockRunScript(1, "Prepare the board update");

    expect(first).toEqual(second);
    expect(first).toMatchObject({
      assistantMessageId: "mock-run-1-assistant",
      runId: "mock-run-1",
      toolActivity: {
        status: "waiting",
        toolName: "create_local_task",
      },
      userMessage: {
        content: "Prepare the board update",
        id: "mock-run-1-user",
      },
    });
    expect(first?.chunks).toHaveLength(3);
  });

  it("rejects blank requests and invalid run ordinals", () => {
    expect(createMockRunScript(1, "  ")).toBeNull();
    expect(createMockRunScript(0, "Valid request")).toBeNull();
    expect(createMockRunScript(Number.NaN, "Valid request")).toBeNull();
  });
});

describe("approvalOutcomeMessage", () => {
  it.each([
    ["approve", "Mock approval recorded. No local task was created or executed."],
    ["reject", "Mock action rejected. No local data was changed."],
    ["edit", "Mock action returned to the composer for editing. Nothing was executed."],
  ] as const)("returns the fixed %s outcome", (decision, message) => {
    expect(approvalOutcomeMessage(decision)).toBe(message);
  });
});
