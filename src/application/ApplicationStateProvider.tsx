import { useReducer, type PropsWithChildren } from "react";

import { ApplicationDispatchContext, ApplicationStateContext } from "./context";
import { applicationReducer, INITIAL_APPLICATION_STATE } from "./state";

export function ApplicationStateProvider({ children }: PropsWithChildren) {
  const [state, dispatch] = useReducer(applicationReducer, INITIAL_APPLICATION_STATE);

  return (
    <ApplicationStateContext.Provider value={state}>
      <ApplicationDispatchContext.Provider value={dispatch}>
        {children}
      </ApplicationDispatchContext.Provider>
    </ApplicationStateContext.Provider>
  );
}
