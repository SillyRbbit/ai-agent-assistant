import { useEffect, type Dispatch } from "react";

import type { MockApprovalDecision } from "./mockAssistantRun";
import {
  browserMockRunDriver,
  type MockRunDriver,
  type MockRunEvent,
  type MockRunHandle,
} from "./mockRunDriver";
import type { ActiveMockRun, ApplicationAction } from "./state";

export interface MockAssistantRunActions {
  readonly decideApproval: (decision: MockApprovalDecision) => void;
  readonly retry: () => void;
  readonly stop: () => void;
  readonly submit: () => void;
}

export function useMockAssistantRun(
  activeRun: ActiveMockRun | null,
  dispatch: Dispatch<ApplicationAction>,
  driver: MockRunDriver = browserMockRunDriver,
): MockAssistantRunActions {
  const runId = activeRun?.script.runId;
  const runStatus = activeRun?.status;

  useEffect(() => {
    if (activeRun === null || runStatus !== "streaming" || runId === undefined) {
      return undefined;
    }

    const onEvent = (event: MockRunEvent) => {
      switch (event.type) {
        case "chunk":
          dispatch({ chunk: event.chunk, runId, type: "mock-stream-chunk" });
          break;
        case "completed":
          dispatch({ runId, type: "mock-stream-completed" });
          break;
        case "failed":
          dispatch({ reason: event.reason, runId, type: "mock-stream-failed" });
          break;
      }
    };

    let handle: MockRunHandle;
    try {
      handle = driver.start(activeRun.script, onEvent);
    } catch {
      dispatch({ reason: "mock-provider-unavailable", runId, type: "mock-stream-failed" });
      return undefined;
    }

    return () => {
      handle.cancel();
    };
  }, [activeRun, dispatch, driver, runId, runStatus]);

  return {
    decideApproval(decision) {
      dispatch({ decision, type: "mock-approval-decided" });
    },
    retry() {
      dispatch({ type: "mock-run-retried" });
    },
    stop() {
      dispatch({ type: "mock-run-stopped" });
    },
    submit() {
      dispatch({ type: "mock-run-submitted" });
    },
  };
}
