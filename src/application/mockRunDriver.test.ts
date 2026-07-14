import { afterEach, describe, expect, it, vi } from "vitest";

import { createMockRunScript, MOCK_STREAM_INTERVAL_MS } from "./mockAssistantRun";
import { browserMockRunDriver, mockRunFailureMessage } from "./mockRunDriver";

afterEach(() => {
  vi.useRealTimers();
});

describe("browserMockRunDriver", () => {
  it("emits the fixed chunks and completion in order", () => {
    vi.useFakeTimers();
    const script = createMockRunScript(1, "Prepare an update");
    if (script === null) {
      throw new Error("Expected a valid mock script.");
    }
    const listener = vi.fn();

    browserMockRunDriver.start(script, listener);
    vi.advanceTimersByTime(MOCK_STREAM_INTERVAL_MS * 4);

    expect(listener.mock.calls).toEqual([
      [{ chunk: script.chunks[0], type: "chunk" }],
      [{ chunk: script.chunks[1], type: "chunk" }],
      [{ chunk: script.chunks[2], type: "chunk" }],
      [{ type: "completed" }],
    ]);
  });

  it("cancels idempotently and emits no later events", () => {
    vi.useFakeTimers();
    const script = createMockRunScript(1, "Prepare an update");
    if (script === null) {
      throw new Error("Expected a valid mock script.");
    }
    const listener = vi.fn();
    const handle = browserMockRunDriver.start(script, listener);

    handle.cancel();
    handle.cancel();
    vi.runAllTimers();

    expect(listener).not.toHaveBeenCalled();
  });
});

describe("mockRunFailureMessage", () => {
  it("returns bounded copy without an underlying error detail", () => {
    expect(mockRunFailureMessage("mock-provider-unavailable")).toBe(
      "The local mock run could not finish. No action was executed.",
    );
  });
});
