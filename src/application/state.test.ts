import { describe, expect, it } from "vitest";

import { APP_ROUTES, type AppRoute } from "./navigation";
import { applicationReducer, INITIAL_APPLICATION_STATE, type ApplicationState } from "./state";

function activeConversation(state: ApplicationState) {
  const conversation = state.conversations.find(
    (candidate) => candidate.id === state.activeConversationId,
  );
  if (conversation === undefined) {
    throw new Error("Expected an active conversation.");
  }

  return conversation;
}

function startMockRun(request = "Prepare the board update") {
  const withDraft = applicationReducer(INITIAL_APPLICATION_STATE, {
    type: "composer-draft-changed",
    value: request,
  });

  return applicationReducer(withDraft, { type: "mock-run-submitted" });
}

function completeMockRun() {
  const started = startMockRun();
  const runId = started.activeRun?.script.runId;
  if (runId === undefined) {
    throw new Error("Expected the mock run to start.");
  }

  return applicationReducer(started, { runId, type: "mock-stream-completed" });
}

describe("applicationReducer", () => {
  it("starts with one selected empty conversation", () => {
    expect(INITIAL_APPLICATION_STATE.activeConversationId).toBe("conversation-1");
    expect(INITIAL_APPLICATION_STATE.conversations).toEqual([
      {
        id: "conversation-1",
        messages: [],
        title: "New conversation",
        toolActivities: [],
      },
    ]);
    expect(INITIAL_APPLICATION_STATE.nextConversationOrdinal).toBe(2);
  });

  it("exposes every route in deterministic navigation order", () => {
    expect(APP_ROUTES).toEqual([
      "conversations",
      "tasks",
      "memory",
      "activity",
      "integrations",
      "permissions",
      "settings",
    ]);
  });

  it.each<AppRoute>([
    "conversations",
    "tasks",
    "memory",
    "activity",
    "integrations",
    "permissions",
    "settings",
  ])("navigates deterministically to %s", (route) => {
    expect(
      applicationReducer(INITIAL_APPLICATION_STATE, { route, type: "navigate" }).activeRoute,
    ).toBe(route);
  });

  it("returns the same state when navigation is unchanged", () => {
    expect(
      applicationReducer(INITIAL_APPLICATION_STATE, {
        route: "conversations",
        type: "navigate",
      }),
    ).toBe(INITIAL_APPLICATION_STATE);
  });

  it("routes a new request to the existing empty conversation and clears the draft", () => {
    const state = {
      ...INITIAL_APPLICATION_STATE,
      activeRoute: "settings" as const,
      composerDraft: "Unsent text",
    };

    expect(
      applicationReducer(state, {
        route: "new_request",
        type: "menu-route-received",
      }),
    ).toEqual({ ...state, activeRoute: "conversations", composerDraft: "" });
  });

  it("routes the tasks placeholder to the tasks page", () => {
    expect(
      applicationReducer(INITIAL_APPLICATION_STATE, {
        route: "tasks_placeholder",
        type: "menu-route-received",
      }).activeRoute,
    ).toBe("tasks");
  });

  it("starts a deterministic in-memory run and clears the draft", () => {
    const state = startMockRun("  Prepare the board update  ");
    const conversation = activeConversation(state);

    expect(state.composerDraft).toBe("");
    expect(state.activeRun).toMatchObject({
      conversationId: "conversation-1",
      script: { runId: "mock-run-1" },
      status: "streaming",
    });
    expect(conversation.title).toBe("Prepare the board update");
    expect(conversation.messages).toMatchObject([
      { content: "Prepare the board update", role: "user", status: "complete" },
      { content: "", role: "assistant", status: "streaming" },
    ]);
    expect(state.nextRunOrdinal).toBe(2);
    expect(state.activityEvents).toMatchObject([{ kind: "run-started", runId: "mock-run-1" }]);
  });

  it("ignores blank submissions, duplicate submissions, and stale stream events", () => {
    expect(applicationReducer(INITIAL_APPLICATION_STATE, { type: "mock-run-submitted" })).toBe(
      INITIAL_APPLICATION_STATE,
    );

    const started = startMockRun();
    expect(applicationReducer(started, { type: "mock-run-submitted" })).toBe(started);
    expect(
      applicationReducer(started, {
        chunk: "stale",
        runId: "another-run",
        type: "mock-stream-chunk",
      }),
    ).toBe(started);
    expect(
      applicationReducer(started, {
        reason: "mock-provider-unavailable",
        runId: "another-run",
        type: "mock-stream-failed",
      }),
    ).toBe(started);
    expect(
      applicationReducer(started, {
        runId: "another-run",
        type: "mock-stream-completed",
      }),
    ).toBe(started);

    const mismatchedConversation = {
      ...started,
      activeConversationId: "another-conversation",
    };
    expect(
      applicationReducer(mismatchedConversation, {
        chunk: "wrong conversation",
        runId: started.activeRun?.script.runId ?? "missing-run",
        type: "mock-stream-chunk",
      }),
    ).toBe(mismatchedConversation);
  });

  it("appends matching stream chunks and opens the mock approval after completion", () => {
    const started = startMockRun();
    const runId = started.activeRun?.script.runId;
    if (runId === undefined) {
      throw new Error("Expected the mock run to start.");
    }

    const streamed = applicationReducer(started, {
      chunk: "First chunk.",
      runId,
      type: "mock-stream-chunk",
    });
    const completed = applicationReducer(streamed, {
      runId,
      type: "mock-stream-completed",
    });

    expect(activeConversation(streamed).messages[1]).toMatchObject({
      content: "First chunk.",
      status: "streaming",
    });
    expect(completed.activeRun?.status).toBe("awaiting-approval");
    expect(completed.activeApproval?.title).toBe("Create a mock local task");
    expect(activeConversation(completed).messages[1]?.status).toBe("complete");
    expect(activeConversation(completed).toolActivities).toMatchObject([
      { status: "waiting", toolName: "create_local_task" },
    ]);
    expect(completed.activityEvents.at(-1)?.kind).toBe("approval-requested");
  });

  it("records a bounded failure and retries without duplicating the user message", () => {
    const started = startMockRun("Sensitive request");
    const runId = started.activeRun?.script.runId;
    if (runId === undefined) {
      throw new Error("Expected the mock run to start.");
    }

    const failed = applicationReducer(started, {
      reason: "mock-provider-unavailable",
      runId,
      type: "mock-stream-failed",
    });
    const retried = applicationReducer(failed, { type: "mock-run-retried" });

    expect(failed.activeRun).toBeNull();
    expect(failed.retryableRun).toMatchObject({ assistantMessageId: "mock-run-1-assistant" });
    expect(activeConversation(failed).messages[1]).toMatchObject({
      content: "The local mock run could not finish. No action was executed.",
      status: "failed",
    });
    expect(failed.activityEvents.at(-1)?.kind).toBe("run-failed");
    expect(retried.activeRun?.script.runId).toBe("mock-run-2");
    expect(
      activeConversation(retried).messages.filter((message) => message.role === "user"),
    ).toHaveLength(1);
    expect(retried.retryableRun).toBeNull();
  });

  it("stops only an active stream and rejects subsequent chunks", () => {
    const started = startMockRun();
    const runId = started.activeRun?.script.runId;
    if (runId === undefined) {
      throw new Error("Expected the mock run to start.");
    }

    const stopped = applicationReducer(started, { type: "mock-run-stopped" });

    expect(stopped.activeRun).toBeNull();
    expect(activeConversation(stopped).messages[1]).toMatchObject({
      content: "Mock response stopped.",
      status: "stopped",
    });
    expect(stopped.activityEvents.at(-1)?.kind).toBe("run-stopped");
    expect(
      applicationReducer(stopped, {
        chunk: "late chunk",
        runId,
        type: "mock-stream-chunk",
      }),
    ).toBe(stopped);
  });

  it.each([
    ["approve", "approved", ""],
    ["reject", "rejected", ""],
    ["edit", "edit-requested", "Revise the local task for: Prepare the board update"],
  ] as const)("records the deterministic %s decision", (decision, status, composerDraft) => {
    const completed = completeMockRun();
    const decided = applicationReducer(completed, {
      decision,
      type: "mock-approval-decided",
    });

    expect(decided.activeApproval).toBeNull();
    expect(decided.activeRun).toBeNull();
    expect(decided.composerDraft).toBe(composerDraft);
    expect(activeConversation(decided).toolActivities[0]?.status).toBe(status);
    expect(activeConversation(decided).messages.at(-1)?.content).toMatch(
      /Nothing was executed|No local/,
    );
    expect(decided.activityEvents.at(-1)?.kind).toBe(
      decision === "approve"
        ? "approval-approved"
        : decision === "reject"
          ? "approval-rejected"
          : "approval-edit-requested",
    );
  });

  it("creates a second conversation and reuses it while it remains empty", () => {
    const completed = completeMockRun();
    const decided = applicationReducer(completed, {
      decision: "reject",
      type: "mock-approval-decided",
    });
    const created = applicationReducer(decided, { type: "new-conversation-requested" });
    const repeated = applicationReducer(created, { type: "new-conversation-requested" });

    expect(created.activeConversationId).toBe("conversation-2");
    expect(created.conversations).toHaveLength(2);
    expect(activeConversation(created)).toMatchObject({
      messages: [],
      title: "New conversation",
      toolActivities: [],
    });
    expect(created.nextConversationOrdinal).toBe(3);
    expect(repeated).toBe(created);
  });

  it("selects the existing empty conversation instead of creating another", () => {
    const completed = completeMockRun();
    const decided = applicationReducer(completed, {
      decision: "approve",
      type: "mock-approval-decided",
    });
    const created = applicationReducer(decided, { type: "new-conversation-requested" });
    const firstSelected = applicationReducer(created, {
      conversationId: "conversation-1",
      type: "conversation-selected",
    });
    const emptySelected = applicationReducer(firstSelected, {
      type: "new-conversation-requested",
    });

    expect(firstSelected.activeConversationId).toBe("conversation-1");
    expect(activeConversation(firstSelected).messages.at(-1)?.content).toMatch(
      /Mock approval recorded/,
    );
    expect(activeConversation(firstSelected).toolActivities[0]?.status).toBe("approved");
    expect(emptySelected.activeConversationId).toBe("conversation-2");
    expect(emptySelected.conversations).toHaveLength(2);
  });

  it("clears retry eligibility when selecting another conversation", () => {
    const started = startMockRun("Retry this request");
    const runId = started.activeRun?.script.runId;
    if (runId === undefined) {
      throw new Error("Expected the mock run to start.");
    }
    const failed = applicationReducer(started, {
      reason: "mock-provider-unavailable",
      runId,
      type: "mock-stream-failed",
    });
    const created = applicationReducer(failed, { type: "new-conversation-requested" });
    const invalidRetry = applicationReducer(created, { type: "mock-run-retried" });

    expect(created.activeConversationId).toBe("conversation-2");
    expect(created.retryableRun).toBeNull();
    expect(invalidRetry).toBe(created);
  });

  it("rejects conversation changes while streaming or awaiting approval", () => {
    const started = startMockRun();
    const runId = started.activeRun?.script.runId;
    if (runId === undefined) {
      throw new Error("Expected the mock run to start.");
    }

    expect(applicationReducer(started, { type: "new-conversation-requested" })).toBe(started);
    expect(
      applicationReducer(started, {
        conversationId: "conversation-2",
        type: "conversation-selected",
      }),
    ).toBe(started);

    const awaitingApproval = applicationReducer(started, {
      runId,
      type: "mock-stream-completed",
    });
    expect(applicationReducer(awaitingApproval, { type: "new-conversation-requested" })).toBe(
      awaitingApproval,
    );
    expect(
      applicationReducer(awaitingApproval, {
        conversationId: "conversation-1",
        type: "conversation-selected",
      }),
    ).toBe(awaitingApproval);
  });

  it("focuses the active conversation without creating one for a busy native request", () => {
    const started = startMockRun();
    const settingsState = applicationReducer(started, { route: "settings", type: "navigate" });
    const focused = applicationReducer(settingsState, {
      route: "new_request",
      type: "menu-route-received",
    });

    expect(focused.activeRoute).toBe("conversations");
    expect(focused.activeConversationId).toBe("conversation-1");
    expect(focused.conversations).toHaveLength(1);
    expect(focused.activeRun).toBe(settingsState.activeRun);
  });
});
