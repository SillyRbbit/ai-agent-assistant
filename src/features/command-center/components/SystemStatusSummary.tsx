import { CircleOff, DatabaseZap, RadioTower } from "lucide-react";

import type { CommandCenterSystemStatusView } from "../commandCenterOverview";

interface SystemStatusSummaryProps {
  readonly status: CommandCenterSystemStatusView;
}

function readable(value: string): string {
  return value
    .split("-")
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join(" ");
}

export function SystemStatusSummary({ status }: SystemStatusSummaryProps) {
  const metrics = [
    ["Simulated active work", status.activeWorkCount],
    ["Simulated running agents", status.runningAgentCount],
    ["Simulated waiting work", status.waitingWorkCount],
    ["Simulated approvals", status.pendingApprovalCount],
    ["Simulated failed work", status.failedCount],
    ["Simulated blocked work", status.blockedCount],
  ] as const;

  return (
    <section aria-label="Simulated system status" className="command-center-summary">
      <div className="command-center-summary__state">
        <span>Overall deterministic scenario</span>
        <strong>{status.scenarioLabel}</strong>
        <span className={`command-center-status command-center-status--${status.simulatedStatus}`}>
          Simulated status · {readable(status.simulatedStatus)}
        </span>
      </div>

      <dl className="command-center-summary__metrics">
        {metrics.map(([label, value]) => (
          <div key={label}>
            <dt>{label}</dt>
            <dd>{value}</dd>
          </div>
        ))}
        <div>
          <dt>Deterministic events</dt>
          <dd>{status.deterministicEventCount}</dd>
        </div>
      </dl>

      <ul aria-label="Unavailable operational data" className="command-center-summary__sources">
        <li title={status.sources.runtime.detail}>
          <CircleOff aria-hidden="true" /> Runtime data unavailable
        </li>
        <li title={status.sources.toolExecution.detail}>
          <DatabaseZap aria-hidden="true" /> Tool data unavailable
        </li>
        <li title={status.sources.provider.detail}>
          <RadioTower aria-hidden="true" /> Provider data unavailable
        </li>
      </ul>
    </section>
  );
}
