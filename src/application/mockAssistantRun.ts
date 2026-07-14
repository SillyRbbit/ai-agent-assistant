export type ConversationMessageRole = "assistant" | "user";
export type ConversationMessageStatus = "complete" | "stopped" | "streaming";

export interface ConversationMessage {
  readonly content: string;
  readonly id: string;
  readonly role: ConversationMessageRole;
  readonly status: ConversationMessageStatus;
}

export type ToolActivityStatus = "approved" | "edit-requested" | "rejected" | "waiting";

export interface ToolActivity {
  readonly description: string;
  readonly id: string;
  readonly status: ToolActivityStatus;
  readonly toolName: "create_local_task";
}

export interface MockApprovalRequest {
  readonly affectedData: string;
  readonly editDraft: string;
  readonly id: string;
  readonly isReversible: true;
  readonly permission: "None";
  readonly risk: "Mock personal-data modification";
  readonly target: "Local task list";
  readonly title: string;
}

export type MockApprovalDecision = "approve" | "edit" | "reject";

export interface MockRunScript {
  readonly approval: MockApprovalRequest;
  readonly assistantMessageId: string;
  readonly chunks: readonly string[];
  readonly runId: string;
  readonly toolActivity: ToolActivity;
  readonly userMessage: ConversationMessage;
}

const STREAM_CHUNKS = [
  "I can prepare a local task preview for that request. ",
  "This demonstration stays in memory and does not call a model or execute a tool. ",
  "Review the mock action before choosing what happens next.",
] as const;

export const MOCK_STREAM_INTERVAL_MS = 160;

export function createMockRunScript(ordinal: number, request: string): MockRunScript | null {
  const normalizedRequest = request.trim();
  if (normalizedRequest.length === 0 || !Number.isSafeInteger(ordinal) || ordinal < 1) {
    return null;
  }

  const runId = `mock-run-${String(ordinal)}`;

  return {
    approval: {
      affectedData: `Task title: Follow up on “${normalizedRequest}”`,
      editDraft: `Revise the local task for: ${normalizedRequest}`,
      id: `${runId}-approval`,
      isReversible: true,
      permission: "None",
      risk: "Mock personal-data modification",
      target: "Local task list",
      title: "Create a mock local task",
    },
    assistantMessageId: `${runId}-assistant`,
    chunks: STREAM_CHUNKS,
    runId,
    toolActivity: {
      description: "Prepared an in-memory task proposal. No tool was executed.",
      id: `${runId}-tool`,
      status: "waiting",
      toolName: "create_local_task",
    },
    userMessage: {
      content: normalizedRequest,
      id: `${runId}-user`,
      role: "user",
      status: "complete",
    },
  };
}

export function approvalOutcomeMessage(decision: MockApprovalDecision): string {
  switch (decision) {
    case "approve":
      return "Mock approval recorded. No local task was created or executed.";
    case "reject":
      return "Mock action rejected. No local data was changed.";
    case "edit":
      return "Mock action returned to the composer for editing. Nothing was executed.";
  }
}
