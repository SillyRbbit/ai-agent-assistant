import { createContext, type Dispatch } from "react";

import type { ApplicationAction, ApplicationState } from "./state";

export const ApplicationStateContext = createContext<ApplicationState | null>(null);
export const ApplicationDispatchContext = createContext<Dispatch<ApplicationAction> | null>(null);
