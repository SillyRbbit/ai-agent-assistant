import type { MockContextProvenance } from "./contextProvenance";
import type { ConversationMessage, ToolActivity } from "./mockAssistantRun";
import type { MockFinalAnswer } from "./mockLoop";
import type { MockToolResult } from "./mockToolResult";

export const EMPTY_CONVERSATION_TITLE = "New conversation";
export const MAX_CONVERSATION_TITLE_LENGTH = 48;

export interface ConversationSession {
  readonly contextProvenance: readonly MockContextProvenance[];
  readonly finalAnswers: readonly MockFinalAnswer[];
  readonly id: string;
  readonly messages: readonly ConversationMessage[];
  readonly title: string;
  readonly toolActivities: readonly ToolActivity[];
  readonly toolResults: readonly MockToolResult[];
}

export function createConversationSession(ordinal: number): ConversationSession | null {
  if (!Number.isSafeInteger(ordinal) || ordinal < 1) {
    return null;
  }

  return {
    contextProvenance: [],
    finalAnswers: [],
    id: `conversation-${String(ordinal)}`,
    messages: [],
    title: EMPTY_CONVERSATION_TITLE,
    toolActivities: [],
    toolResults: [],
  };
}

export function conversationTitleForRequest(request: string): string | null {
  const normalizedRequest = request.trim().replace(/\s+/gu, " ");
  if (normalizedRequest.length === 0) {
    return null;
  }

  const codePoints = Array.from(normalizedRequest);
  if (codePoints.length <= MAX_CONVERSATION_TITLE_LENGTH) {
    return normalizedRequest;
  }

  return `${codePoints
    .slice(0, MAX_CONVERSATION_TITLE_LENGTH - 1)
    .join("")
    .trimEnd()}…`;
}

export function isConversationSessionEmpty(conversation: ConversationSession): boolean {
  return (
    conversation.contextProvenance.length === 0 &&
    conversation.finalAnswers.length === 0 &&
    conversation.messages.length === 0 &&
    conversation.toolActivities.length === 0 &&
    conversation.toolResults.length === 0
  );
}
