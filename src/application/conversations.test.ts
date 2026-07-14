import { describe, expect, it } from "vitest";

import {
  createConversationSession,
  conversationTitleForRequest,
  EMPTY_CONVERSATION_TITLE,
  isConversationSessionEmpty,
  MAX_CONVERSATION_TITLE_LENGTH,
  type ConversationSession,
} from "./conversations";

describe("conversation sessions", () => {
  it.each([0, -1, 1.5, Number.MAX_SAFE_INTEGER + 1])(
    "rejects invalid conversation ordinal %s",
    (ordinal) => {
      expect(createConversationSession(ordinal)).toBeNull();
    },
  );

  it("creates a deterministic empty conversation", () => {
    expect(createConversationSession(3)).toEqual({
      id: "conversation-3",
      messages: [],
      title: EMPTY_CONVERSATION_TITLE,
      toolActivities: [],
    });
  });

  it("normalizes whitespace without changing the request source", () => {
    const request = "  Prepare\n\tthe   board update  ";

    expect(conversationTitleForRequest(request)).toBe("Prepare the board update");
    expect(request).toBe("  Prepare\n\tthe   board update  ");
  });

  it("returns null for a blank request", () => {
    expect(conversationTitleForRequest(" \n\t ")).toBeNull();
  });

  it("caps a title at 48 Unicode code points including the ellipsis", () => {
    const request = "🙂".repeat(MAX_CONVERSATION_TITLE_LENGTH + 10);
    const title = conversationTitleForRequest(request);

    expect(title).not.toBeNull();
    expect(Array.from(title ?? "")).toHaveLength(MAX_CONVERSATION_TITLE_LENGTH);
    expect(title?.endsWith("…")).toBe(true);
  });

  it("recognizes only sessions without messages or tool activity as empty", () => {
    const empty = createConversationSession(1);
    if (empty === null) {
      throw new Error("Expected a valid conversation session.");
    }

    const withMessage: ConversationSession = {
      ...empty,
      messages: [{ content: "Request", id: "message-1", role: "user", status: "complete" }],
    };
    const withActivity: ConversationSession = {
      ...empty,
      toolActivities: [
        {
          description: "Prepared a mock proposal.",
          id: "tool-1",
          status: "waiting",
          toolName: "create_local_task",
        },
      ],
    };

    expect(isConversationSessionEmpty(empty)).toBe(true);
    expect(isConversationSessionEmpty(withMessage)).toBe(false);
    expect(isConversationSessionEmpty(withActivity)).toBe(false);
  });
});
