import { describe, expect, it } from "vitest";

import { approvalOutcomeMessage, createMockRunScript } from "./mockAssistantRun";
import {
  appendMockAssistantOutput,
  canAppendMockToolCall,
  createMockFinalAnswer,
  isMockRunAttempt,
  MOCK_FINAL_ANSWER_CONTENT,
  MOCK_LOOP_LIMITS,
  nextMockRetryAttempt,
} from "./mockLoop";
import { mockRunFailureMessage } from "./mockRunDriver";

describe("mock loop limits", () => {
  it("exposes one frozen conservative limit contract", () => {
    expect(MOCK_LOOP_LIMITS).toEqual({
      maxAssistantOutputCharactersPerTurn: 512,
      maxConsecutiveModelTurns: 2,
      maxFileBytes: 0,
      maxNetworkRequests: 0,
      maxRetryAttempts: 1,
      maxSearchResults: 0,
      maxToolCallsPerRun: 1,
      toolTimeoutMs: 0,
    });
    expect(Object.isFrozen(MOCK_LOOP_LIMITS)).toBe(true);
  });

  it("allows only the initial attempt and one retry", () => {
    expect(isMockRunAttempt(0)).toBe(true);
    expect(isMockRunAttempt(1)).toBe(true);
    expect(isMockRunAttempt(-1)).toBe(false);
    expect(isMockRunAttempt(2)).toBe(false);
    expect(isMockRunAttempt(0.5)).toBe(false);
    expect(nextMockRetryAttempt(0)).toBe(1);
    expect(nextMockRetryAttempt(1)).toBeNull();
    expect(nextMockRetryAttempt(-1)).toBeNull();
  });

  it("allows no more than one mock tool call per run", () => {
    expect(canAppendMockToolCall(0)).toBe(true);
    expect(canAppendMockToolCall(1)).toBe(false);
    expect(canAppendMockToolCall(-1)).toBe(false);
    expect(canAppendMockToolCall(0.5)).toBe(false);
  });

  it("bounds assistant output by Unicode code points", () => {
    const atLimit = "🙂".repeat(MOCK_LOOP_LIMITS.maxAssistantOutputCharactersPerTurn);

    expect(appendMockAssistantOutput("", atLimit)).toBe(atLimit);
    expect(appendMockAssistantOutput(atLimit, "x")).toBeNull();
  });

  it("keeps both deterministic mock turns within the output ceiling", () => {
    const script = createMockRunScript(1, "Request");
    if (script === null) {
      throw new Error("Expected a valid mock run script.");
    }

    expect(Array.from(script.chunks.join("")).length).toBeLessThanOrEqual(
      MOCK_LOOP_LIMITS.maxAssistantOutputCharactersPerTurn,
    );
    expect(Array.from(MOCK_FINAL_ANSWER_CONTENT).length).toBeLessThanOrEqual(
      MOCK_LOOP_LIMITS.maxAssistantOutputCharactersPerTurn,
    );
  });

  it("keeps fixed stop, failure, and decision copy within the output ceiling", () => {
    const fixedOutputs = [
      "Mock response stopped.",
      mockRunFailureMessage("mock-provider-unavailable"),
      approvalOutcomeMessage("approve"),
      approvalOutcomeMessage("edit"),
      approvalOutcomeMessage("reject"),
    ];

    for (const output of fixedOutputs) {
      expect(Array.from(output).length).toBeLessThanOrEqual(
        MOCK_LOOP_LIMITS.maxAssistantOutputCharactersPerTurn,
      );
    }
  });
});

describe("mock final answer", () => {
  it.each(["", "mock-run-0", "mock-run-1-result", "run-1", " mock-run-1"])(
    "rejects invalid run ID %j",
    (runId) => {
      expect(createMockFinalAnswer(runId, "conversation-1", `${runId}-result`)).toBeNull();
    },
  );

  it.each(["", "conversation-0", "conversation-one", " conversation-1"])(
    "rejects invalid conversation ID %j",
    (conversationId) => {
      expect(createMockFinalAnswer("mock-run-1", conversationId, "mock-run-1-result")).toBeNull();
    },
  );

  it("rejects a result from another run", () => {
    expect(createMockFinalAnswer("mock-run-1", "conversation-1", "mock-run-2-result")).toBeNull();
  });

  it("creates one fixed answer bound to exact identifiers", () => {
    const answer = createMockFinalAnswer("mock-run-7", "conversation-3", "mock-run-7-result");

    expect(answer).toEqual({
      content: MOCK_FINAL_ANSWER_CONTENT,
      conversationId: "conversation-3",
      id: "mock-run-7-final",
      modelTurn: 2,
      runId: "mock-run-7",
      source: "deterministic-frontend-mock",
      toolResultId: "mock-run-7-result",
    });
    expect(answer).not.toHaveProperty("request");
    expect(answer).not.toHaveProperty("result");
  });
});
