import type { ActivityEvent } from "../../application/activity";
import { PageState } from "../../components/PageState";
import { PageHeader } from "../shared/PageHeader";

interface ActivityPageProps {
  readonly events: readonly ActivityEvent[];
}

export function ActivityPage({ events }: ActivityPageProps) {
  return (
    <section aria-labelledby="activity-page-title" className="page-stack">
      <PageHeader
        description="Review redacted lifecycle events from the current in-memory session."
        eyebrow="Transparency"
        headingId="activity-page-title"
        title="Activity"
      />

      <div className="page-panel activity-panel">
        {events.length === 0 ? (
          <PageState
            description="Run activity appears here without request text, tool arguments, or error details."
            icon="A"
            title="No session activity"
          />
        ) : (
          <ol aria-label="Run activity" className="activity-list">
            {[...events].reverse().map((event) => (
              <li className="activity-event" key={event.id}>
                <span
                  aria-hidden="true"
                  className={`activity-event__marker activity-event__marker--${event.tone}`}
                />
                <div className="activity-event__copy">
                  <div>
                    <strong>{event.summary}</strong>
                    <span>{event.runId}</span>
                  </div>
                  <p>{event.detail}</p>
                </div>
              </li>
            ))}
          </ol>
        )}
      </div>
    </section>
  );
}
