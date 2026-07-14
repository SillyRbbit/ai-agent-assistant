export type MockContextSourceId =
  | "current-request"
  | "earlier-conversation-messages"
  | "saved-memory"
  | "device-data"
  | "external-services";

export type MockContextSourceStatus = "not-used" | "used";

export interface MockContextSource {
  readonly id: MockContextSourceId;
  readonly label: string;
  readonly status: MockContextSourceStatus;
}

export interface MockContextProvenance {
  readonly conversationId: string;
  readonly id: string;
  readonly runId: string;
  readonly sources: readonly MockContextSource[];
}

const MOCK_RUN_ID_PATTERN = /^mock-run-[1-9]\d*$/u;
const CONVERSATION_ID_PATTERN = /^conversation-[1-9]\d*$/u;

const MOCK_CONTEXT_SOURCES: readonly MockContextSource[] = [
  { id: "current-request", label: "Current request", status: "used" },
  {
    id: "earlier-conversation-messages",
    label: "Earlier conversation messages",
    status: "not-used",
  },
  { id: "saved-memory", label: "Saved memory", status: "not-used" },
  { id: "device-data", label: "Device data", status: "not-used" },
  { id: "external-services", label: "External services", status: "not-used" },
];

export function createMockContextProvenance(
  runId: string,
  conversationId: string,
): MockContextProvenance | null {
  if (!MOCK_RUN_ID_PATTERN.test(runId) || !CONVERSATION_ID_PATTERN.test(conversationId)) {
    return null;
  }

  return {
    conversationId,
    id: `${runId}-context`,
    runId,
    sources: MOCK_CONTEXT_SOURCES.map((source) => ({ ...source })),
  };
}
