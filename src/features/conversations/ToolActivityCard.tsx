import type { ToolActivity } from "../../application/mockAssistantRun";

interface ToolActivityCardProps {
  readonly activity: ToolActivity;
}

const STATUS_LABELS: Readonly<Record<ToolActivity["status"], string>> = {
  approved: "Mock approved",
  "edit-requested": "Edit requested",
  rejected: "Rejected",
  waiting: "Awaiting review",
};

export function ToolActivityCard({ activity }: ToolActivityCardProps) {
  return (
    <article aria-label="Mock tool activity" className="tool-activity-card">
      <div className="tool-activity-card__icon" aria-hidden="true">
        T
      </div>
      <div className="tool-activity-card__copy">
        <span>Mock tool proposal</span>
        <strong>{activity.toolName}</strong>
        <p>{activity.description}</p>
      </div>
      <span className={`tool-activity-status tool-activity-status--${activity.status}`}>
        {STATUS_LABELS[activity.status]}
      </span>
    </article>
  );
}
