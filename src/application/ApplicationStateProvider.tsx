import { useReducer, useCallback, type PropsWithChildren } from "react";

import { ApplicationDispatchContext, ApplicationStateContext } from "./context";
import { applicationReducer, INITIAL_APPLICATION_STATE } from "./state";

import { allowKnowledgeNavigation } from "../features/knowledge/useUnsavedKnowledge";
import type { ApplicationAction } from "./state";

export function ApplicationStateProvider({ children }: PropsWithChildren) {
  const [state, rawDispatch] = useReducer(applicationReducer, INITIAL_APPLICATION_STATE);

  const dispatch = useCallback((action: ApplicationAction) => {
    if (
      [
        "navigate",
        "menu-route-received",
        "collaboration-navigate",
        "conversation-selected",
        "new-conversation-requested",
      ].includes(action.type) &&
      !allowKnowledgeNavigation(() => {
        rawDispatch(action);
      })
    )
      return;
    rawDispatch(action);
  }, []);
  return (
    <ApplicationStateContext.Provider value={state}>
      <ApplicationDispatchContext.Provider value={dispatch}>
        {children}
      </ApplicationDispatchContext.Provider>
    </ApplicationStateContext.Provider>
  );
}
