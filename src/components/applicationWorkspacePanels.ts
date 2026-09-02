import { createContext, useContext } from "react";

export interface ApplicationWorkspacePanels {
  readonly activityBodyTarget: HTMLDivElement | null;
  readonly activitySummaryTarget: HTMLSpanElement | null;
  readonly closeInspector: () => void;
  readonly inspectorBodyTarget: HTMLDivElement | null;
  readonly inspectorHeaderTarget: HTMLDivElement | null;
  readonly openInspector: (returnFocus?: HTMLElement | null) => void;
}

export const ApplicationWorkspacePanelsContext = createContext<ApplicationWorkspacePanels | null>(
  null,
);

export function useApplicationWorkspacePanels(): ApplicationWorkspacePanels | null {
  return useContext(ApplicationWorkspacePanelsContext);
}
