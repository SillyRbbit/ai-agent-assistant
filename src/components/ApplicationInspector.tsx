import { PanelRightClose } from "lucide-react";
import type { KeyboardEvent, Ref } from "react";

interface ApplicationInspectorProps {
  readonly bodyOutletRef: Ref<HTMLDivElement>;
  readonly customContent: boolean;
  readonly expanded: boolean;
  readonly headerOutletRef: Ref<HTMLDivElement>;
  readonly onClose: () => void;
}

export function ApplicationInspector({
  bodyOutletRef,
  customContent,
  expanded,
  headerOutletRef,
  onClose,
}: ApplicationInspectorProps) {
  const handleKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    if (event.key !== "Escape") return;
    event.preventDefault();
    event.stopPropagation();
    onClose();
  };

  return (
    <aside
      aria-labelledby="application-inspector-title"
      className="application-inspector"
      data-scroll-region="application-inspector"
      hidden={!expanded}
      id="application-inspector"
      onKeyDown={handleKeyDown}
    >
      <div className="application-inspector__header-outlet" ref={headerOutletRef}>
        {customContent ? null : (
          <div className="application-panel-header">
            <div>
              <span className="application-panel-header__eyebrow">Context</span>
              <h2 id="application-inspector-title">Workspace inspector</h2>
            </div>
            <button
              aria-label="Close workspace inspector"
              className="application-panel-header__button"
              data-application-inspector-close="true"
              onClick={onClose}
              title="Close workspace inspector"
              type="button"
            >
              <PanelRightClose aria-hidden="true" />
            </button>
          </div>
        )}
      </div>
      <div
        className="application-inspector__content"
        data-scroll-owner="application-inspector-content"
        ref={bodyOutletRef}
      >
        {customContent ? null : (
          <div className="application-panel-empty">
            <strong>No inspectable workspace item selected</strong>
            <p>This screen does not currently expose a typed presentation selection.</p>
          </div>
        )}
      </div>
    </aside>
  );
}
