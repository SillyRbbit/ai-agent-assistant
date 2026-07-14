import { useContext, type Dispatch } from "react";

import { ApplicationDispatchContext, ApplicationStateContext } from "./context";
import type { ApplicationAction, ApplicationState } from "./state";

export function useApplicationState(): ApplicationState {
  const state = useContext(ApplicationStateContext);

  if (state === null) {
    throw new Error("useApplicationState must be used within ApplicationStateProvider.");
  }

  return state;
}

export function useApplicationDispatch(): Dispatch<ApplicationAction> {
  const dispatch = useContext(ApplicationDispatchContext);

  if (dispatch === null) {
    throw new Error("useApplicationDispatch must be used within ApplicationStateProvider.");
  }

  return dispatch;
}
