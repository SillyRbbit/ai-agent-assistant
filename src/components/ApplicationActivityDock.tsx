import { Activity, ChevronDown, ChevronUp, GripHorizontal } from "lucide-react";
import {
  useEffect,
  useRef,
  type KeyboardEvent,
  type PointerEvent as ReactPointerEvent,
  type Ref,
} from "react";

import type { ActivityEvent } from "../application/activity";

interface ApplicationActivityDockProps {
  readonly bodyOutletRef: Ref<HTMLDivElement>;
  readonly customContent: boolean;
  readonly events: readonly ActivityEvent[];
  readonly expanded: boolean;
  readonly height: number;
  readonly maximumHeight: number;
  readonly minimumHeight: number;
  readonly onResize: (height: number) => void;
  readonly onToggle: () => void;
  readonly summaryOutletRef: Ref<HTMLSpanElement>;
}

const ACTIVITY_RESIZE_STEP = 16;

function readable(value: string): string {
  return value.replaceAll("-", " ");
}

export function ApplicationActivityDock({
  bodyOutletRef,
  customContent,
  events,
  expanded,
  height,
  maximumHeight,
  minimumHeight,
  onResize,
  onToggle,
  summaryOutletRef,
}: ApplicationActivityDockProps) {
  const removePointerListenersRef = useRef<(() => void) | null>(null);

  useEffect(
    () => () => {
      removePointerListenersRef.current?.();
    },
    [],
  );

  const handleResizeKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    let nextHeight: number | undefined;

    switch (event.key) {
      case "ArrowDown":
        nextHeight = height - ACTIVITY_RESIZE_STEP;
        break;
      case "ArrowUp":
        nextHeight = height + ACTIVITY_RESIZE_STEP;
        break;
      case "End":
        nextHeight = maximumHeight;
        break;
      case "Home":
        nextHeight = minimumHeight;
        break;
      default:
        return;
    }

    event.preventDefault();
    onResize(nextHeight);
  };

  const handleResizePointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) {
      return;
    }

    event.preventDefault();
    removePointerListenersRef.current?.();

    const pointerId = event.pointerId;
    const startHeight = height;
    const startY = event.clientY;

    const handlePointerMove = (pointerEvent: PointerEvent) => {
      if (pointerEvent.pointerId !== pointerId) {
        return;
      }

      onResize(startHeight + startY - pointerEvent.clientY);
    };
    const removeListeners = () => {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", handlePointerUp);
      window.removeEventListener("pointercancel", handlePointerUp);
      removePointerListenersRef.current = null;
    };
    const handlePointerUp = (pointerEvent: PointerEvent) => {
      if (pointerEvent.pointerId === pointerId) {
        removeListeners();
      }
    };

    removePointerListenersRef.current = removeListeners;
    window.addEventListener("pointermove", handlePointerMove);
    window.addEventListener("pointerup", handlePointerUp);
    window.addEventListener("pointercancel", handlePointerUp);
  };

  return (
    <section
      aria-labelledby="application-activity-title"
      className="application-activity-dock"
      data-scroll-region="application-activity"
      id="application-activity-dock"
    >
      {expanded ? (
        <div
          aria-label="Resize activity panel"
          aria-orientation="horizontal"
          aria-valuemax={maximumHeight}
          aria-valuemin={minimumHeight}
          aria-valuenow={height}
          aria-valuetext={`${String(height)} pixels high`}
          className="application-activity-dock__resize-handle"
          onKeyDown={handleResizeKeyDown}
          onPointerDown={handleResizePointerDown}
          role="separator"
          tabIndex={0}
          title="Drag or use arrow keys to resize activity"
        >
          <GripHorizontal aria-hidden="true" />
        </div>
      ) : null}

      <div className="application-activity-dock__header">
        <div className="application-activity-dock__title">
          <Activity aria-hidden="true" />
          <div>
            <h2 id="application-activity-title">Activity</h2>
            {customContent ? (
              <span className="application-activity-dock__summary-outlet" ref={summaryOutletRef} />
            ) : (
              <span>
                {events.length === 0
                  ? "No current-session mock events"
                  : `${String(events.length)} current-session mock event${events.length === 1 ? "" : "s"}`}
              </span>
            )}
          </div>
        </div>
        <button
          aria-controls="application-activity-content"
          aria-expanded={expanded}
          aria-label={expanded ? "Collapse activity panel" : "Expand activity panel"}
          className="application-panel-header__button"
          onClick={onToggle}
          title={expanded ? "Collapse activity panel" : "Expand activity panel"}
          type="button"
        >
          {expanded ? <ChevronDown aria-hidden="true" /> : <ChevronUp aria-hidden="true" />}
        </button>
      </div>

      <div
        className="application-activity-dock__content"
        data-scroll-owner="application-activity-content"
        hidden={!expanded}
        id="application-activity-content"
        ref={bodyOutletRef}
      >
        {customContent ? null : events.length === 0 ? (
          <div className="application-panel-empty application-panel-empty--inline">
            <strong>No current-session activity</strong>
            <p>
              Volatile mock lifecycle events appear here after a deterministic conversation run.
            </p>
          </div>
        ) : (
          <ol aria-label="Current-session activity" className="application-activity-dock__events">
            {[...events].reverse().map((event) => (
              <li key={event.id}>
                <span
                  aria-hidden="true"
                  className={`activity-event__marker activity-event__marker--${event.tone}`}
                />
                <div>
                  <strong>{event.summary}</strong>
                  <p>{event.detail}</p>
                  <dl>
                    <div>
                      <dt>Time</dt>
                      <dd>Unavailable</dd>
                    </div>
                    <div>
                      <dt>Source</dt>
                      <dd>Deterministic frontend mock loop</dd>
                    </div>
                    <div>
                      <dt>Action</dt>
                      <dd>{readable(event.kind)}</dd>
                    </div>
                    <div>
                      <dt>Run</dt>
                      <dd>{event.runId}</dd>
                    </div>
                    <div>
                      <dt>Status</dt>
                      <dd>Unavailable in current-session event data</dd>
                    </div>
                  </dl>
                </div>
              </li>
            ))}
          </ol>
        )}
      </div>
    </section>
  );
}
