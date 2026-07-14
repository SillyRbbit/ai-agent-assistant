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
        contextProvenance: [],
        finalAnswers: [],
        id: "conversation-1",
        messages: [],
        title: "New conversation",
        toolActivities: [],
        toolResults: [],
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
      retryAttempt: 0,
      script: { runId: "mock-run-1" },
      status: "streaming",
    });
    expect(conversation.title).toBe("Prepare the board update");
    expect(conversation.messages).toMatchObject([
      { content: "Prepare the board update", role: "user", status: "complete" },
      { content: "", role: "assistant", status: "streaming" },
    ]);
    expect(conversation.contextProvenance).toEqual([
      expect.objectContaining({
        conversationId: "conversation-1",
        id: "mock-run-1-context",
        runId: "mock-run-1",
      }),
    ]);
    expect(conversation.contextProvenance[0]?.sources).toMatchObject([
      { id: "current-request", status: "used" },
      { id: "earlier-conversation-messages", status: "not-used" },
      { id: "saved-memory", status: "not-used" },
      { id: "device-data", status: "not-used" },
      { id: "external-services", status: "not-used" },
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

  it("rejects a stream chunk that would exceed the mock output limit", () => {
    const started = startMockRun();
    const runId = started.activeRun?.script.runId;
    if (runId === undefined) {
      throw new Error("Expected the mock run to start.");
    }

    const oversized = applicationReducer(started, {
      chunk: "x".repeat(513),
      runId,
      type: "mock-stream-chunk",
    });

    expect(oversized).toBe(started);
    expect(activeConversation(oversized).messages[1]?.content).toBe("");
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
    expect(failed.retryableRun).toMatchObject({
      assistantMessageId: "mock-run-1-assistant",
      retryAttempt: 1,
    });
    expect(activeConversation(failed).messages[1]).toMatchObject({
      content: "The local mock run could not finish. No action was executed.",
      status: "failed",
    });
    expect(failed.activityEvents.at(-1)?.kind).toBe("run-failed");
    expect(retried.activeRun?.script.runId).toBe("mock-run-2");
    expect(retried.activeRun?.retryAttempt).toBe(1);
    expect(
      activeConversation(retried).messages.filter((message) => message.role === "user"),
    ).toHaveLength(1);
    expect(activeConversation(retried).contextProvenance).toMatchObject([
      { conversationId: "conversation-1", runId: "mock-run-1" },
      { conversationId: "conversation-1", runId: "mock-run-2" },
    ]);
    expect(retried.retryableRun).toBeNull();
  });

  it("does not offer another retry after the retry attempt fails", () => {
    const started = startMockRun("Retry once");
    const firstRunId = started.activeRun?.script.runId;
    if (firstRunId === undefined) {
      throw new Error("Expected the initial mock run to start.");
    }

    const failed = applicationReducer(started, {
      reason: "mock-provider-unavailable",
      runId: firstRunId,
      type: "mock-stream-failed",
    });
    const retried = applicationReducer(failed, { type: "mock-run-retried" });
    const retryRunId = retried.activeRun?.script.runId;
    if (retryRunId === undefined) {
      throw new Error("Expected the retry mock run to start.");
    }

    const retryFailed = applicationReducer(retried, {
      reason: "mock-provider-unavailable",
      runId: retryRunId,
      type: "mock-stream-failed",
    });

    expect(retryFailed.activeRun).toBeNull();
    expect(retryFailed.retryableRun).toBeNull();
    expect(applicationReducer(retryFailed, { type: "mock-run-retried" })).toBe(retryFailed);
    expect(
      activeConversation(retryFailed).messages.filter((message) => message.role === "user"),
    ).toHaveLength(1);
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
    ["approve", "approved", "", 1],
    ["reject", "rejected", "", 0],
    ["edit", "edit-requested", "Revise the local task for: Prepare the board update", 0],
  ] as const)(
    "records the deterministic %s decision",
    (decision, status, composerDraft, resultCount) => {
      const completed = completeMockRun();
      const decided = applicationReducer(completed, {
        decision,
        type: "mock-approval-decided",
      });

      expect(decided.activeApproval).toBeNull();
      expect(decided.activeRun).toBeNull();
      expect(decided.composerDraft).toBe(composerDraft);
      expect(activeConversation(decided).toolActivities[0]?.status).toBe(status);
      expect(activeConversation(decided).toolResults).toHaveLength(resultCount);
      expect(activeConversation(decided).finalAnswers).toHaveLength(resultCount);
      if (decision === "approve") {
        expect(activeConversation(decided).toolResults[0]).toEqual({
          conversationId: "conversation-1",
          executed: false,
          id: "mock-run-1-result",
          runId: "mock-run-1",
          status: "simulated",
          summary: "No local task was created and no data changed.",
          toolActivityId: "mock-run-1-tool",
          toolName: "create_local_task",
        });
        expect(activeConversation(decided).finalAnswers[0]).toEqual({
          content:
            "Mock run complete. The approved task action was simulated only; no local task was created and no data changed.",
          conversationId: "conversation-1",
          id: "mock-run-1-final",
          modelTurn: 2,
          runId: "mock-run-1",
          source: "deterministic-frontend-mock",
          toolResultId: "mock-run-1-result",
        });
        expect(activeConversation(decided).messages.at(-1)?.id).toBe("mock-run-1-assistant");
        expect(
          applicationReducer(decided, { decision: "approve", type: "mock-approval-decided" }),
        ).toBe(decided);
      } else {
        expect(activeConversation(decided).messages.at(-1)?.content).toMatch(
          /Nothing was executed|No local/,
        );
      }
      expect(decided.activityEvents.at(-1)?.kind).toBe(
        decision === "approve"
          ? "approval-approved"
          : decision === "reject"
            ? "approval-rejected"
            : "approval-edit-requested",
      );
    },
  );

  it("attributes an approved retried run result to the fresh run", () => {
    const started = startMockRun("Retry this request");
    const firstRunId = started.activeRun?.script.runId;
    if (firstRunId === undefined) {
      throw new Error("Expected the first mock run to start.");
    }

    const failed = applicationReducer(started, {
      reason: "mock-provider-unavailable",
      runId: firstRunId,
      type: "mock-stream-failed",
    });
    const retried = applicationReducer(failed, { type: "mock-run-retried" });
    const retryRunId = retried.activeRun?.script.runId;
    if (retryRunId === undefined) {
      throw new Error("Expected the retry mock run to start.");
    }

    const completed = applicationReducer(retried, {
      runId: retryRunId,
      type: "mock-stream-completed",
    });
    const approved = applicationReducer(completed, {
      decision: "approve",
      type: "mock-approval-decided",
    });

    expect(activeConversation(approved).toolResults).toMatchObject([
      {
        conversationId: "conversation-1",
        id: "mock-run-2-result",
        runId: "mock-run-2",
        toolActivityId: "mock-run-2-tool",
      },
    ]);
    expect(activeConversation(approved).finalAnswers).toMatchObject([
      {
        conversationId: "conversation-1",
        id: "mock-run-2-final",
        runId: "mock-run-2",
        toolResultId: "mock-run-2-result",
      },
    ]);
    expect(
      activeConversation(approved).messages.filter((message) => message.role === "user"),
    ).toHaveLength(1);
  });

  it("rejects approval when the active proposal is not the conversation proposal", () => {
    const completed = completeMockRun();
    const activeRun = completed.activeRun;
    if (activeRun === null) {
      throw new Error("Expected an active approval run.");
    }

    const mismatched: ApplicationState = {
      ...completed,
      activeRun: {
        ...activeRun,
        script: {
          ...activeRun.script,
          toolActivity: {
            ...activeRun.script.toolActivity,
            id: "mock-run-99-tool",
          },
        },
      },
    };

    expect(
      applicationReducer(mismatched, {
        decision: "approve",
        type: "mock-approval-decided",
      }),
    ).toBe(mismatched);
    expect(activeConversation(mismatched).toolResults).toEqual([]);
    expect(activeConversation(mismatched).finalAnswers).toEqual([]);
  });

  it("rejects approval when the run already has its maximum tool call", () => {
    const completed = completeMockRun();
    const conversation = activeConversation(completed);
    const withExistingResult: ApplicationState = {
      ...completed,
      conversations: completed.conversations.map((candidate) =>
        candidate.id === conversation.id
          ? {
              ...candidate,
              toolResults: [
                {
                  conversationId: "conversation-1",
                  executed: false,
                  id: "mock-run-1-result",
                  runId: "mock-run-1",
                  status: "simulated",
                  summary: "No local task was created and no data changed.",
                  toolActivityId: "mock-run-1-tool",
                  toolName: "create_local_task",
                },
              ],
            }
          : candidate,
      ),
    };

    expect(
      applicationReducer(withExistingResult, {
        decision: "approve",
        type: "mock-approval-decided",
      }),
    ).toBe(withExistingResult);
    expect(activeConversation(withExistingResult).finalAnswers).toEqual([]);
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
      contextProvenance: [],
      finalAnswers: [],
      messages: [],
      title: "New conversation",
      toolActivities: [],
      toolResults: [],
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
    expect(activeConversation(firstSelected).finalAnswers).toMatchObject([
      { runId: "mock-run-1", toolResultId: "mock-run-1-result" },
    ]);
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
