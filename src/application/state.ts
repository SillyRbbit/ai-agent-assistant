import { createActivityEvent, type ActivityEvent, type ActivityEventKind } from "./activity";
import { destinationForMenuRoute, type AppRoute, type AssistantMenuRoute } from "./navigation";
import {
  approvalOutcomeMessage,
  createMockRunScript,
  type ConversationMessage,
  type MockApprovalDecision,
  type MockApprovalRequest,
  type MockRunScript,
  type ToolActivity,
} from "./mockAssistantRun";
import { mockRunFailureMessage, type MockRunFailureReason } from "./mockRunDriver";

export type AssistantRunStatus = "awaiting-approval" | "idle" | "streaming";

export interface ActiveMockRun {
  readonly script: MockRunScript;
  readonly status: Exclude<AssistantRunStatus, "idle">;
}

export interface RetryableMockRun {
  readonly assistantMessageId: string;
  readonly request: string;
}

export interface ApplicationState {
  readonly activeApproval: MockApprovalRequest | null;
  readonly activeRun: ActiveMockRun | null;
  readonly activeRoute: AppRoute;
  readonly activityEvents: readonly ActivityEvent[];
  readonly composerDraft: string;
  readonly messages: readonly ConversationMessage[];
  readonly nextActivityOrdinal: number;
  readonly nextRunOrdinal: number;
  readonly retryableRun: RetryableMockRun | null;
  readonly toolActivities: readonly ToolActivity[];
}

export type ApplicationAction =
  | { readonly route: AppRoute; readonly type: "navigate" }
  | { readonly route: AssistantMenuRoute; readonly type: "menu-route-received" }
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

export const INITIAL_APPLICATION_STATE: ApplicationState = {
  activeApproval: null,
  activeRun: null,
  activeRoute: "conversations",
  activityEvents: [],
  composerDraft: "",
  messages: [],
  nextActivityOrdinal: 1,
  nextRunOrdinal: 1,
  retryableRun: null,
  toolActivities: [],
};

function updateAssistantMessage(
  messages: readonly ConversationMessage[],
  assistantMessageId: string,
  update: (message: ConversationMessage) => ConversationMessage,
): readonly ConversationMessage[] {
  return messages.map((message) => (message.id === assistantMessageId ? update(message) : message));
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

function startMockRun(
  state: ApplicationState,
  request: string,
  includeUserMessage: boolean,
): ApplicationState {
  if (state.activeRun !== null) {
    return state;
  }

  const script = createMockRunScript(state.nextRunOrdinal, request);
  if (script === null) {
    return state;
  }

  const messages = includeUserMessage ? [...state.messages, script.userMessage] : state.messages;

  return {
    ...state,
    ...appendActivity(state, script.runId, "run-started"),
    activeApproval: null,
    activeRun: { script, status: "streaming" },
    activeRoute: "conversations",
    composerDraft: "",
    messages: [
      ...messages,
      {
        content: "",
        id: script.assistantMessageId,
        role: "assistant",
        status: "streaming",
      },
    ],
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
    case "composer-draft-changed":
      return action.value === state.composerDraft
        ? state
        : { ...state, composerDraft: action.value };
    case "mock-run-submitted": {
      return startMockRun(state, state.composerDraft, true);
    }
    case "mock-run-retried":
      return state.retryableRun === null
        ? state
        : startMockRun(state, state.retryableRun.request, false);
    case "mock-stream-chunk": {
      if (
        state.activeRun?.status !== "streaming" ||
        state.activeRun.script.runId !== action.runId
      ) {
        return state;
      }

      return {
        ...state,
        messages: updateAssistantMessage(
          state.messages,
          state.activeRun.script.assistantMessageId,
          (message) => ({ ...message, content: `${message.content}${action.chunk}` }),
        ),
      };
    }
    case "mock-stream-completed": {
      if (
        state.activeRun?.status !== "streaming" ||
        state.activeRun.script.runId !== action.runId
      ) {
        return state;
      }

      const { script } = state.activeRun;
      return {
        ...state,
        ...appendActivity(state, script.runId, "approval-requested"),
        activeApproval: script.approval,
        activeRun: { script, status: "awaiting-approval" },
        messages: updateAssistantMessage(state.messages, script.assistantMessageId, (message) => ({
          ...message,
          status: "complete",
        })),
        toolActivities: [...state.toolActivities, script.toolActivity],
      };
    }
    case "mock-stream-failed": {
      if (
        state.activeRun?.status !== "streaming" ||
        state.activeRun.script.runId !== action.runId
      ) {
        return state;
      }

      const { script } = state.activeRun;
      return {
        ...state,
        ...appendActivity(state, script.runId, "run-failed"),
        activeApproval: null,
        activeRun: null,
        messages: updateAssistantMessage(state.messages, script.assistantMessageId, (message) => ({
          ...message,
          content: mockRunFailureMessage(action.reason),
          status: "failed",
        })),
        retryableRun: {
          assistantMessageId: script.assistantMessageId,
          request: script.userMessage.content,
        },
      };
    }
    case "mock-run-stopped": {
      if (state.activeRun?.status !== "streaming") {
        return state;
      }

      return {
        ...state,
        ...appendActivity(state, state.activeRun.script.runId, "run-stopped"),
        activeRun: null,
        messages: updateAssistantMessage(
          state.messages,
          state.activeRun.script.assistantMessageId,
          (message) => ({
            ...message,
            content: message.content.length === 0 ? "Mock response stopped." : message.content,
            status: "stopped",
          }),
        ),
        retryableRun: null,
      };
    }
    case "mock-approval-decided": {
      if (state.activeRun?.status !== "awaiting-approval" || state.activeApproval === null) {
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
        id: `${state.activeRun.script.runId}-outcome`,
        role: "assistant",
        status: "complete",
      };
      const activityKind: ActivityEventKind =
        action.decision === "approve"
          ? "approval-approved"
          : action.decision === "reject"
            ? "approval-rejected"
            : "approval-edit-requested";
      const runId = state.activeRun.script.runId;

      return {
        ...state,
        ...appendActivity(state, runId, activityKind),
        activeApproval: null,
        activeRun: null,
        composerDraft:
          action.decision === "edit" ? state.activeApproval.editDraft : state.composerDraft,
        messages: [...state.messages, outcome],
        retryableRun: null,
        toolActivities: state.toolActivities.map((activity) =>
          activity.id === state.activeRun?.script.toolActivity.id
            ? { ...activity, status: activityStatus }
            : activity,
        ),
      };
    }
    case "menu-route-received": {
      const activeRoute = destinationForMenuRoute(action.route);

      if (action.route === "new_request") {
        return {
          ...state,
          activeRoute,
          composerDraft: "",
        };
      }

      return activeRoute === state.activeRoute ? state : { ...state, activeRoute };
    }
  }
}
