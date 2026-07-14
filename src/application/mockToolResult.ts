export interface MockToolResult {
  readonly conversationId: string;
  readonly executed: false;
  readonly id: string;
  readonly runId: string;
  readonly status: "simulated";
  readonly summary: "No local task was created and no data changed.";
  readonly toolActivityId: string;
  readonly toolName: "create_local_task";
}

const MOCK_RUN_ID_PATTERN = /^mock-run-[1-9]\d*$/u;
const CONVERSATION_ID_PATTERN = /^conversation-[1-9]\d*$/u;

export function createMockToolResult(runId: string, conversationId: string): MockToolResult | null {
  if (!MOCK_RUN_ID_PATTERN.test(runId) || !CONVERSATION_ID_PATTERN.test(conversationId)) {
    return null;
  }

  return {
    conversationId,
    executed: false,
    id: `${runId}-result`,
    runId,
    status: "simulated",
    summary: "No local task was created and no data changed.",
    toolActivityId: `${runId}-tool`,
    toolName: "create_local_task",
  };
}
