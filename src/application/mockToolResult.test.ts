import { describe, expect, it } from "vitest";

import { createMockToolResult } from "./mockToolResult";

describe("mock tool results", () => {
  it.each([
    ["", "conversation-1"],
    ["mock-run-0", "conversation-1"],
    ["mock-run-01", "conversation-1"],
    ["mock-run-1", ""],
    ["mock-run-1", "conversation-0"],
    ["mock-run-1", "conversation-01"],
    ["another-run", "conversation-1"],
    ["mock-run-1", "another-conversation"],
  ])("rejects malformed identifiers %s and %s", (runId, conversationId) => {
    expect(createMockToolResult(runId, conversationId)).toBeNull();
  });

  it("creates a deterministic fixed result without request or output inputs", () => {
    expect(createMockToolResult("mock-run-2", "conversation-3")).toEqual({
      conversationId: "conversation-3",
      executed: false,
      id: "mock-run-2-result",
      runId: "mock-run-2",
      status: "simulated",
      summary: "No local task was created and no data changed.",
      toolActivityId: "mock-run-2-tool",
      toolName: "create_local_task",
    });
  });
});
