export const MOCK_LOOP_LIMITS = Object.freeze({
  maxAssistantOutputCharactersPerTurn: 512,
  maxConsecutiveModelTurns: 2,
  maxFileBytes: 0,
  maxNetworkRequests: 0,
  maxRetryAttempts: 1,
  maxSearchResults: 0,
  maxToolCallsPerRun: 1,
  toolTimeoutMs: 0,
} as const);

export const MOCK_FINAL_ANSWER_CONTENT =
  "Mock run complete. The approved task action was simulated only; no local task was created and no data changed.";

export type MockRunAttempt = 0 | 1;

export interface MockFinalAnswer {
  readonly content: typeof MOCK_FINAL_ANSWER_CONTENT;
  readonly conversationId: string;
  readonly id: string;
  readonly modelTurn: 2;
  readonly runId: string;
  readonly source: "deterministic-frontend-mock";
  readonly toolResultId: string;
}

const MOCK_RUN_ID_PATTERN = /^mock-run-[1-9]\d*$/u;
const CONVERSATION_ID_PATTERN = /^conversation-[1-9]\d*$/u;

function characterCount(value: string): number {
  return Array.from(value).length;
}

export function appendMockAssistantOutput(currentOutput: string, chunk: string): string | null {
  const nextOutput = `${currentOutput}${chunk}`;
  return characterCount(nextOutput) <= MOCK_LOOP_LIMITS.maxAssistantOutputCharactersPerTurn
    ? nextOutput
    : null;
}

export function canAppendMockToolCall(existingToolCallCount: number): boolean {
  return (
    Number.isSafeInteger(existingToolCallCount) &&
    existingToolCallCount >= 0 &&
    existingToolCallCount < MOCK_LOOP_LIMITS.maxToolCallsPerRun
  );
}

export function isMockRunAttempt(attempt: number): attempt is MockRunAttempt {
  return (
    Number.isSafeInteger(attempt) && attempt >= 0 && attempt <= MOCK_LOOP_LIMITS.maxRetryAttempts
  );
}

export function nextMockRetryAttempt(attempt: number): MockRunAttempt | null {
  if (!isMockRunAttempt(attempt) || attempt >= MOCK_LOOP_LIMITS.maxRetryAttempts) {
    return null;
  }

  const nextAttempt = attempt + 1;
  return isMockRunAttempt(nextAttempt) ? nextAttempt : null;
}

export function createMockFinalAnswer(
  runId: string,
  conversationId: string,
  toolResultId: string,
): MockFinalAnswer | null {
  if (
    !MOCK_RUN_ID_PATTERN.test(runId) ||
    !CONVERSATION_ID_PATTERN.test(conversationId) ||
    toolResultId !== `${runId}-result` ||
    characterCount(MOCK_FINAL_ANSWER_CONTENT) > MOCK_LOOP_LIMITS.maxAssistantOutputCharactersPerTurn
  ) {
    return null;
  }

  return {
    content: MOCK_FINAL_ANSWER_CONTENT,
    conversationId,
    id: `${runId}-final`,
    modelTurn: MOCK_LOOP_LIMITS.maxConsecutiveModelTurns,
    runId,
    source: "deterministic-frontend-mock",
    toolResultId,
  };
}
