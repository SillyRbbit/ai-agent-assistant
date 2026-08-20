interface SystemStatusSummaryProps {
  readonly activeCount: number;
  readonly eventCount: number;
  readonly scenarioLabel: string;
  readonly totalAgents: number;
}

export function SystemStatusSummary({
  activeCount,
  eventCount,
  scenarioLabel,
  totalAgents,
}: SystemStatusSummaryProps) {
  return (
    <section aria-label="Simulated system summary" className="command-center-summary">
      <div className="command-center-summary__item">
        <span>Catalog roles</span>
        <strong>{totalAgents}</strong>
        <small>Exact application-owned agent definitions</small>
      </div>
      <div className="command-center-summary__item">
        <span>Active fixture items</span>
        <strong>{activeCount}</strong>
        <small>Simulated status, not live health</small>
      </div>
      <div className="command-center-summary__item">
        <span>Fixture events</span>
        <strong>{eventCount}</strong>
        <small>Bounded deterministic records</small>
      </div>
      <div className="command-center-summary__item">
        <span>Scenario</span>
        <strong>{scenarioLabel}</strong>
        <small>Frontend fixture · no execution</small>
      </div>
    </section>
  );
}
