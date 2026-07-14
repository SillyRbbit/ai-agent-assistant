import { createActivityEvent, type ActivityEvent, type ActivityEventKind } from "./activity";
import { createMockContextProvenance } from "./contextProvenance";
import {
  createConversationSession,
  conversationTitleForRequest,
  EMPTY_CONVERSATION_TITLE,
  isConversationSessionEmpty,
  type ConversationSession,
} from "./conversations";
import { destinationForMenuRoute, type AppRoute, type AssistantMenuRoute } from "./navigation";
import {
  approvalOutcomeMessage,
  createMockRunScript,
  type ConversationMessage,
  type MockApprovalDecision,
  type MockApprovalRequest,
  type MockRunScript,
} from "./mockAssistantRun";
import { mockRunFailureMessage, type MockRunFailureReason } from "./mockRunDriver";

export type AssistantRunStatus = "awaiting-approval" | "idle" | "streaming";

export interface ActiveMockRun {
  readonly conversationId: string;
  readonly script: MockRunScript;
  readonly status: Exclude<AssistantRunStatus, "idle">;
}

export interface RetryableMockRun {
  readonly assistantMessageId: string;
  readonly conversationId: string;
  readonly request: string;
}

export interface ApplicationState {
  readonly activeApproval: MockApprovalRequest | null;
  readonly activeConversationId: string;
  readonly activeRun: ActiveMockRun | null;
  readonly activeRoute: AppRoute;
  readonly activityEvents: readonly ActivityEvent[];
  readonly composerDraft: string;
  readonly conversations: readonly ConversationSession[];
  readonly nextActivityOrdinal: number;
  readonly nextConversationOrdinal: number;
  readonly nextRunOrdinal: number;
  readonly retryableRun: RetryableMockRun | null;
}

export type ApplicationAction =
  | { readonly route: AppRoute; readonly type: "navigate" }
  | { readonly route: AssistantMenuRoute; readonly type: "menu-route-received" }
  | { readonly conversationId: string; readonly type: "conversation-selected" }
  | { readonly type: "new-conversation-requested" }
  | { readonly type: "composer-draft-changed"; readonly value: string }
  | { readonly type: "mock-run-submitted" }
  | { readonly chunk: string; readonly runId: string; readonly type: "mock-stream-chunk" }
  | { readonly runId: string; readonly type: "mock-stream-completed" }
  | {
      readonly reason: MockRunFailureReason;
      readonly runId: string;
      readonly type: "mock-stream-failed";
    }
  | { readonly type: "mock-run-stopped" }
  | { readonly type: "mock-run-retried" }
  | { readonly decision: MockApprovalDecision; readonly type: "mock-approval-decided" };

const INITIAL_CONVERSATION: ConversationSession = {
  contextProvenance: [],
  id: "conversation-1",
  messages: [],
  title: EMPTY_CONVERSATION_TITLE,
  toolActivities: [],
};

export const INITIAL_APPLICATION_STATE: ApplicationState = {
  activeApproval: null,
  activeConversationId: INITIAL_CONVERSATION.id,
  activeRun: null,
  activeRoute: "conversations",
  activityEvents: [],
  composerDraft: "",
  conversations: [INITIAL_CONVERSATION],
  nextActivityOrdinal: 1,
  nextConversationOrdinal: 2,
  nextRunOrdinal: 1,
  retryableRun: null,
};

function updateAssistantMessage(
  messages: readonly ConversationMessage[],
  assistantMessageId: string,
  update: (message: ConversationMessage) => ConversationMessage,
): readonly ConversationMessage[] {
  return messages.map((message) => (message.id === assistantMessageId ? update(message) : message));
}

function updateConversationById(
  conversations: readonly ConversationSession[],
  conversationId: string,
  update: (conversation: ConversationSession) => ConversationSession,
): readonly ConversationSession[] | null {
  const conversationIndex = conversations.findIndex(
    (conversation) => conversation.id === conversationId,
  );
  if (conversationIndex === -1) {
    return null;
  }

  return conversations.map((conversation, index) =>
    index === conversationIndex ? update(conversation) : conversation,
  );
}

function appendActivity(
  state: ApplicationState,
  runId: string,
  kind: ActivityEventKind,
): Pick<ApplicationState, "activityEvents" | "nextActivityOrdinal"> {
  const event = createActivityEvent(state.nextActivityOrdinal, runId, kind);
  if (event === null) {
    return {
      activityEvents: state.activityEvents,
      nextActivityOrdinal: state.nextActivityOrdinal,
    };
  }

  return {
    activityEvents: [...state.activityEvents, event],
    nextActivityOrdinal: state.nextActivityOrdinal + 1,
  };
}

function isConversationChangeBlocked(state: ApplicationState): boolean {
  return state.activeRun !== null || state.activeApproval !== null;
}

function createOrSelectEmptyConversation(state: ApplicationState): ApplicationState {
  if (isConversationChangeBlocked(state)) {
    return state;
  }

  const existingEmptyConversation = state.conversations.find(isConversationSessionEmpty);
  if (existingEmptyConversation !== undefined) {
    if (
      state.activeConversationId === existingEmptyConversation.id &&
      state.activeRoute === "conversations" &&
      state.composerDraft.length === 0 &&
      state.retryableRun === null
    ) {
      return state;
    }

    return {
      ...state,
      activeConversationId: existingEmptyConversation.id,
      activeRoute: "conversations",
      composerDraft: "",
      retryableRun: null,
    };
  }

  const conversation = createConversationSession(state.nextConversationOrdinal);
  if (conversation === null) {
    return state;
  }

  return {
    ...state,
    activeConversationId: conversation.id,
    activeRoute: "conversations",
    composerDraft: "",
    conversations: [...state.conversations, conversation],
    nextConversationOrdinal: state.nextConversationOrdinal + 1,
    retryableRun: null,
  };
}

function selectConversation(state: ApplicationState, conversationId: string): ApplicationState {
  if (
    isConversationChangeBlocked(state) ||
    !state.conversations.some((conversation) => conversation.id === conversationId)
  ) {
    return state;
  }

  if (conversationId === state.activeConversationId) {
    return state.activeRoute === "conversations"
      ? state
      : { ...state, activeRoute: "conversations" };
  }

  return {
    ...state,
    activeConversationId: conversationId,
    activeRoute: "conversations",
    composerDraft: "",
    retryableRun: null,
  };
}

function startMockRun(
  state: ApplicationState,
  request: string,
  includeUserMessage: boolean,
): ApplicationState {
  if (isConversationChangeBlocked(state)) {
    return state;
  }

  const activeConversation = state.conversations.find(
    (conversation) => conversation.id === state.activeConversationId,
  );
  if (activeConversation === undefined) {
    return state;
  }

  const script = createMockRunScript(state.nextRunOrdinal, request);
  if (script === null) {
    return state;
  }

  const contextProvenance = createMockContextProvenance(script.runId, activeConversation.id);
  if (contextProvenance === null) {
    return state;
  }

  const messages = includeUserMessage
    ? [...activeConversation.messages, script.userMessage]
    : activeConversation.messages;
  const title =
    activeConversation.messages.length === 0
      ? (conversationTitleForRequest(script.userMessage.content) ?? activeConversation.title)
      : activeConversation.title;
  const conversations = updateConversationById(
    state.conversations,
    activeConversation.id,
    (conversation) => ({
      ...conversation,
      contextProvenance: [...conversation.contextProvenance, contextProvenance],
      messages: [
        ...messages,
        {
          content: "",
          id: script.assistantMessageId,
          role: "assistant",
          status: "streaming",
        },
      ],
      title,
    }),
  );
  if (conversations === null) {
    return state;
  }

  return {
    ...state,
    ...appendActivity(state, script.runId, "run-started"),
    activeApproval: null,
    activeRun: {
      conversationId: activeConversation.id,
      script,
      status: "streaming",
    },
    activeRoute: "conversations",
    composerDraft: "",
    conversations,
    nextRunOrdinal: state.nextRunOrdinal + 1,
    retryableRun: null,
  };
}

export function applicationReducer(
  state: ApplicationState,
  action: ApplicationAction,
): ApplicationState {
  switch (action.type) {
    case "navigate":
      return action.route === state.activeRoute ? state : { ...state, activeRoute: action.route };
    case "conversation-selected":
      return selectConversation(state, action.conversationId);
    case "new-conversation-requested":
      return createOrSelectEmptyConversation(state);
    case "composer-draft-changed":
      return action.value === state.composerDraft
        ? state
        : { ...state, composerDraft: action.value };
    case "mock-run-submitted":
      return startMockRun(state, state.composerDraft, true);
    case "mock-run-retried": {
      const retryableRun = state.retryableRun;
      return retryableRun?.conversationId !== state.activeConversationId
        ? state
        : startMockRun(state, retryableRun.request, false);
    }
    case "mock-stream-chunk": {
      const activeRun = state.activeRun;
      if (
        activeRun?.status !== "streaming" ||
        activeRun.script.runId !== action.runId ||
        activeRun.conversationId !== state.activeConversationId
      ) {
        return state;
      }

      const conversations = updateConversationById(
        state.conversations,
        activeRun.conversationId,
        (conversation) => ({
          ...conversation,
          messages: updateAssistantMessage(
            conversation.messages,
            activeRun.script.assistantMessageId,
            (message) => ({ ...message, content: `${message.content}${action.chunk}` }),
          ),
        }),
      );

      return conversations === null ? state : { ...state, conversations };
    }
    case "mock-stream-completed": {
      const activeRun = state.activeRun;
      if (
        activeRun?.status !== "streaming" ||
        activeRun.script.runId !== action.runId ||
        activeRun.conversationId !== state.activeConversationId
      ) {
        return state;
      }

      const conversations = updateConversationById(
        state.conversations,
        activeRun.conversationId,
        (conversation) => ({
          ...conversation,
          messages: updateAssistantMessage(
            conversation.messages,
            activeRun.script.assistantMessageId,
            (message) => ({ ...message, status: "complete" }),
          ),
          toolActivities: [...conversation.toolActivities, activeRun.script.toolActivity],
        }),
      );
      if (conversations === null) {
        return state;
      }

      return {
        ...state,
        ...appendActivity(state, activeRun.script.runId, "approval-requested"),
        activeApproval: activeRun.script.approval,
        activeRun: { ...activeRun, status: "awaiting-approval" },
        conversations,
      };
    }
    case "mock-stream-failed": {
      const activeRun = state.activeRun;
      if (
        activeRun?.status !== "streaming" ||
        activeRun.script.runId !== action.runId ||
        activeRun.conversationId !== state.activeConversationId
      ) {
        return state;
      }

      const conversations = updateConversationById(
        state.conversations,
        activeRun.conversationId,
        (conversation) => ({
          ...conversation,
          messages: updateAssistantMessage(
            conversation.messages,
            activeRun.script.assistantMessageId,
            (message) => ({
              ...message,
              content: mockRunFailureMessage(action.reason),
              status: "failed",
            }),
          ),
        }),
      );
      if (conversations === null) {
        return state;
      }

      return {
        ...state,
        ...appendActivity(state, activeRun.script.runId, "run-failed"),
        activeApproval: null,
        activeRun: null,
        conversations,
        retryableRun: {
          assistantMessageId: activeRun.script.assistantMessageId,
          conversationId: activeRun.conversationId,
          request: activeRun.script.userMessage.content,
        },
      };
    }
    case "mock-run-stopped": {
      const activeRun = state.activeRun;
      if (
        activeRun?.status !== "streaming" ||
        activeRun.conversationId !== state.activeConversationId
      ) {
        return state;
      }

      const conversations = updateConversationById(
        state.conversations,
        activeRun.conversationId,
        (conversation) => ({
          ...conversation,
          messages: updateAssistantMessage(
            conversation.messages,
            activeRun.script.assistantMessageId,
            (message) => ({
              ...message,
              content: message.content.length === 0 ? "Mock response stopped." : message.content,
              status: "stopped",
            }),
          ),
        }),
      );
      if (conversations === null) {
        return state;
      }

      return {
        ...state,
        ...appendActivity(state, activeRun.script.runId, "run-stopped"),
        activeRun: null,
        conversations,
        retryableRun: null,
      };
    }
    case "mock-approval-decided": {
      const activeRun = state.activeRun;
      if (
        activeRun?.status !== "awaiting-approval" ||
        state.activeApproval === null ||
        activeRun.conversationId !== state.activeConversationId
      ) {
        return state;
      }

      const activityStatus =
        action.decision === "approve"
          ? "approved"
          : action.decision === "reject"
            ? "rejected"
            : "edit-requested";
      const outcome: ConversationMessage = {
        content: approvalOutcomeMessage(action.decision),
        id: `${activeRun.script.runId}-outcome`,
        role: "assistant",
        status: "complete",
      };
      const activityKind: ActivityEventKind =
        action.decision === "approve"
          ? "approval-approved"
          : action.decision === "reject"
            ? "approval-rejected"
            : "approval-edit-requested";
      const conversations = updateConversationById(
        state.conversations,
        activeRun.conversationId,
        (conversation) => ({
          ...conversation,
          messages: [...conversation.messages, outcome],
          toolActivities: conversation.toolActivities.map((activity) =>
            activity.id === activeRun.script.toolActivity.id
              ? { ...activity, status: activityStatus }
              : activity,
          ),
        }),
      );
      if (conversations === null) {
        return state;
      }

      return {
        ...state,
        ...appendActivity(state, activeRun.script.runId, activityKind),
        activeApproval: null,
        activeRun: null,
        composerDraft:
          action.decision === "edit" ? state.activeApproval.editDraft : state.composerDraft,
        conversations,
        retryableRun: null,
      };
    }
    case "menu-route-received": {
      const activeRoute = destinationForMenuRoute(action.route);

      if (action.route === "new_request") {
        return isConversationChangeBlocked(state)
          ? state.activeRoute === activeRoute
            ? state
            : { ...state, activeRoute }
          : createOrSelectEmptyConversation(state);
      }

      return activeRoute === state.activeRoute ? state : { ...state, activeRoute };
    }
  }
}
