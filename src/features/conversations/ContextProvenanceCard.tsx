import type { MockContextProvenance } from "../../application/contextProvenance";

interface ContextProvenanceCardProps {
  readonly provenance: MockContextProvenance;
}

export function ContextProvenanceCard({ provenance }: ContextProvenanceCardProps) {
  const headingId = `${provenance.id}-heading`;

  return (
    <article aria-labelledby={headingId} className="context-provenance-card">
      <header className="context-provenance-card__header">
        <div>
          <span>Run context</span>
          <strong id={headingId}>Mock context used</strong>
        </div>
        <span className="context-provenance-card__run">{provenance.runId}</span>
      </header>

      <dl className="context-provenance-card__sources">
        {provenance.sources.map((source) => (
          <div key={source.id}>
            <dt>{source.label}</dt>
            <dd className={`context-provenance-card__status--${source.status}`}>
              {source.status === "used" ? "Used" : "Not used"}
            </dd>
          </div>
        ))}
      </dl>

      <p>Frontend mock only. Not trusted audit evidence.</p>
    </article>
  );
}
