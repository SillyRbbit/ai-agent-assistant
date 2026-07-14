import { MOCK_STREAM_INTERVAL_MS, type MockRunScript } from "./mockAssistantRun";

export type MockRunFailureReason = "mock-provider-unavailable";

export type MockRunEvent =
  | { readonly chunk: string; readonly type: "chunk" }
  | { readonly type: "completed" }
  | { readonly reason: MockRunFailureReason; readonly type: "failed" };

export type MockRunEventListener = (event: MockRunEvent) => void;

export interface MockRunHandle {
  readonly cancel: () => void;
}

export interface MockRunDriver {
  readonly start: (script: MockRunScript, listener: MockRunEventListener) => MockRunHandle;
}

export const browserMockRunDriver: MockRunDriver = {
  start(script, listener) {
    let cancelled = false;
    const timers = script.chunks.map((chunk, index) =>
      window.setTimeout(
        () => {
          if (!cancelled) {
            listener({ chunk, type: "chunk" });
          }
        },
        MOCK_STREAM_INTERVAL_MS * (index + 1),
      ),
    );
    timers.push(
      window.setTimeout(
        () => {
          if (!cancelled) {
            listener({ type: "completed" });
          }
        },
        MOCK_STREAM_INTERVAL_MS * (script.chunks.length + 1),
      ),
    );

    return {
      cancel() {
        if (cancelled) {
          return;
        }

        cancelled = true;
        for (const timer of timers) {
          window.clearTimeout(timer);
        }
      },
    };
  },
};

const FAILURE_MESSAGES: Readonly<Record<MockRunFailureReason, string>> = {
  "mock-provider-unavailable": "The local mock run could not finish. No action was executed.",
};

export function mockRunFailureMessage(reason: MockRunFailureReason): string {
  return FAILURE_MESSAGES[reason];
}
