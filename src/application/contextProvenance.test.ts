import { describe, expect, it } from "vitest";

import { createMockContextProvenance } from "./contextProvenance";

describe("mock context provenance", () => {
  it.each([
    ["", "conversation-1"],
    ["mock-run-0", "conversation-1"],
    ["mock-run-1", ""],
    ["mock-run-1", "conversation-0"],
    ["another-run", "conversation-1"],
    ["mock-run-1", "another-conversation"],
  ])("rejects malformed identifiers %s and %s", (runId, conversationId) => {
    expect(createMockContextProvenance(runId, conversationId)).toBeNull();
  });

  it("creates deterministic fixed-copy provenance without a request-content input", () => {
    expect(createMockContextProvenance("mock-run-2", "conversation-3")).toEqual({
      conversationId: "conversation-3",
      id: "mock-run-2-context",
      runId: "mock-run-2",
      sources: [
        { id: "current-request", label: "Current request", status: "used" },
        {
          id: "earlier-conversation-messages",
          label: "Earlier conversation messages",
          status: "not-used",
        },
        { id: "saved-memory", label: "Saved memory", status: "not-used" },
        { id: "device-data", label: "Device data", status: "not-used" },
        { id: "external-services", label: "External services", status: "not-used" },
      ],
    });
  });

  it("returns fresh source records without changing the deterministic values", () => {
    const first = createMockContextProvenance("mock-run-1", "conversation-1");
    const second = createMockContextProvenance("mock-run-1", "conversation-1");

    expect(first).toEqual(second);
    expect(first?.sources).not.toBe(second?.sources);
    expect(first?.sources[0]).not.toBe(second?.sources[0]);
  });
});
