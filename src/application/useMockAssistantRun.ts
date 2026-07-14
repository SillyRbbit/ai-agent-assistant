import { useEffect, type Dispatch } from "react";

import { MOCK_STREAM_INTERVAL_MS, type MockApprovalDecision } from "./mockAssistantRun";
import type { ActiveMockRun, ApplicationAction } from "./state";

export interface MockAssistantRunActions {
  readonly decideApproval: (decision: MockApprovalDecision) => void;
  readonly stop: () => void;
  readonly submit: () => void;
}

export function useMockAssistantRun(
  activeRun: ActiveMockRun | null,
  dispatch: Dispatch<ApplicationAction>,
): MockAssistantRunActions {
  const runId = activeRun?.script.runId;
  const runStatus = activeRun?.status;

  useEffect(() => {
    if (activeRun === null || runStatus !== "streaming" || runId === undefined) {
      return undefined;
    }

    const timers = activeRun.script.chunks.map((chunk, index) =>
      window.setTimeout(
        () => {
          dispatch({ chunk, runId, type: "mock-stream-chunk" });
        },
        MOCK_STREAM_INTERVAL_MS * (index + 1),
      ),
    );
    timers.push(
      window.setTimeout(
        () => {
          dispatch({ runId, type: "mock-stream-completed" });
        },
        MOCK_STREAM_INTERVAL_MS * (activeRun.script.chunks.length + 1),
      ),
    );

    return () => {
      for (const timer of timers) {
        window.clearTimeout(timer);
      }
    };
  }, [activeRun, dispatch, runId, runStatus]);

  return {
    decideApproval(decision) {
      dispatch({ decision, type: "mock-approval-decided" });
    },
    stop() {
      dispatch({ type: "mock-run-stopped" });
    },
    submit() {
      dispatch({ type: "mock-run-submitted" });
    },
  };
}
