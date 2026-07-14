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

export type AssistantRunStatus = "awaiting-approval" | "idle" | "streaming";

export interface ActiveMockRun {
  readonly script: MockRunScript;
  readonly status: Exclude<AssistantRunStatus, "idle">;
}

export interface ApplicationState {
  readonly activeApproval: MockApprovalRequest | null;
  readonly activeRun: ActiveMockRun | null;
  readonly activeRoute: AppRoute;
  readonly composerDraft: string;
  readonly messages: readonly ConversationMessage[];
  readonly nextRunOrdinal: number;
  readonly toolActivities: readonly ToolActivity[];
}

export type ApplicationAction =
  | { readonly route: AppRoute; readonly type: "navigate" }
  | { readonly route: AssistantMenuRoute; readonly type: "menu-route-received" }
  | { readonly type: "composer-draft-changed"; readonly value: string }
  | { readonly type: "mock-run-submitted" }
  | { readonly chunk: string; readonly runId: string; readonly type: "mock-stream-chunk" }
  | { readonly runId: string; readonly type: "mock-stream-completed" }
  | { readonly type: "mock-run-stopped" }
  | { readonly decision: MockApprovalDecision; readonly type: "mock-approval-decided" };

export const INITIAL_APPLICATION_STATE: ApplicationState = {
  activeApproval: null,
  activeRun: null,
  activeRoute: "conversations",
  composerDraft: "",
  messages: [],
  nextRunOrdinal: 1,
  toolActivities: [],
};

function updateAssistantMessage(
  messages: readonly ConversationMessage[],
  assistantMessageId: string,
  update: (message: ConversationMessage) => ConversationMessage,
): readonly ConversationMessage[] {
  return messages.map((message) => (message.id === assistantMessageId ? update(message) : message));
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
      if (state.activeRun !== null) {
        return state;
      }

      const script = createMockRunScript(state.nextRunOrdinal, state.composerDraft);
      if (script === null) {
        return state;
      }

      return {
        ...state,
        activeApproval: null,
        activeRun: { script, status: "streaming" },
        activeRoute: "conversations",
        composerDraft: "",
        messages: [
          ...state.messages,
          script.userMessage,
          {
            content: "",
            id: script.assistantMessageId,
            role: "assistant",
            status: "streaming",
          },
        ],
        nextRunOrdinal: state.nextRunOrdinal + 1,
      };
    }
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
        activeApproval: script.approval,
        activeRun: { script, status: "awaiting-approval" },
        messages: updateAssistantMessage(state.messages, script.assistantMessageId, (message) => ({
          ...message,
          status: "complete",
        })),
        toolActivities: [...state.toolActivities, script.toolActivity],
      };
    }
    case "mock-run-stopped": {
      if (state.activeRun?.status !== "streaming") {
        return state;
      }

      return {
        ...state,
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

      return {
        ...state,
        activeApproval: null,
        activeRun: null,
        composerDraft:
          action.decision === "edit" ? state.activeApproval.editDraft : state.composerDraft,
        messages: [...state.messages, outcome],
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
