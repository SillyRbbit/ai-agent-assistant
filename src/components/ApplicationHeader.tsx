import {
  PanelBottomClose,
  PanelBottomOpen,
  PanelLeftClose,
  PanelLeftOpen,
  PanelRightClose,
  PanelRightOpen,
} from "lucide-react";
import type { ReactNode, RefObject } from "react";

interface ApplicationHeaderProps {
  readonly activeLabel: string;
  readonly activityExpanded: boolean;
  readonly coreStatus: ReactNode;
  readonly inspectorExpanded: boolean;
  readonly inspectorToggleRef: RefObject<HTMLButtonElement | null>;
  readonly navigationExpanded: boolean;
  readonly onToggleActivity: () => void;
  readonly onToggleInspector: () => void;
  readonly onToggleNavigation: () => void;
}

export function ApplicationHeader({
  activeLabel,
  activityExpanded,
  coreStatus,
  inspectorExpanded,
  inspectorToggleRef,
  navigationExpanded,
  onToggleActivity,
  onToggleInspector,
  onToggleNavigation,
}: ApplicationHeaderProps) {
  return (
    <header className="application-header">
      <div className="application-header__primary">
        <button
          aria-controls="application-sidebar"
          aria-expanded={navigationExpanded}
          aria-label={navigationExpanded ? "Collapse navigation" : "Expand navigation"}
          className="application-header__icon-button"
          onClick={onToggleNavigation}
          title={navigationExpanded ? "Collapse navigation" : "Expand navigation"}
          type="button"
        >
          {navigationExpanded ? (
            <PanelLeftClose aria-hidden="true" />
          ) : (
            <PanelLeftOpen aria-hidden="true" />
          )}
        </button>

        <div className="application-header__location">
          <span>Workspace</span>
          <span aria-hidden="true">/</span>
          <strong aria-atomic="true" aria-live="polite" className="application-toolbar__location">
            {activeLabel}
          </strong>
        </div>
      </div>

      <div className="application-header__actions">
        {coreStatus}
        <span aria-hidden="true" className="application-header__divider" />
        <button
          aria-controls="application-inspector"
          aria-expanded={inspectorExpanded}
          aria-label={inspectorExpanded ? "Hide workspace inspector" : "Show workspace inspector"}
          className="application-header__icon-button"
          id="application-inspector-toggle"
          onClick={onToggleInspector}
          ref={inspectorToggleRef}
          title={inspectorExpanded ? "Hide workspace inspector" : "Show workspace inspector"}
          type="button"
        >
          {inspectorExpanded ? (
            <PanelRightClose aria-hidden="true" />
          ) : (
            <PanelRightOpen aria-hidden="true" />
          )}
        </button>
        <button
          aria-controls="application-activity-dock"
          aria-expanded={activityExpanded}
          aria-label={activityExpanded ? "Hide activity panel" : "Show activity panel"}
          className="application-header__icon-button"
          onClick={onToggleActivity}
          title={activityExpanded ? "Hide activity panel" : "Show activity panel"}
          type="button"
        >
          {activityExpanded ? (
            <PanelBottomClose aria-hidden="true" />
          ) : (
            <PanelBottomOpen aria-hidden="true" />
          )}
        </button>
      </div>
    </header>
  );
}
