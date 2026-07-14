import type { MockToolResult } from "../../application/mockToolResult";

interface ToolResultCardProps {
  readonly result: MockToolResult;
}

export function ToolResultCard({ result }: ToolResultCardProps) {
  const headingId = `${result.id}-heading`;

  return (
    <article aria-labelledby={headingId} className="tool-result-card">
      <header className="tool-result-card__header">
        <div>
          <span>Run result</span>
          <strong id={headingId}>Mock tool result</strong>
        </div>
        <span className="tool-result-card__run">{result.runId}</span>
      </header>

      <div aria-label="Mock result status" className="tool-result-card__status" role="group">
        <strong>{result.toolName}</strong>
        <span>Simulated</span>
        <span>No execution</span>
      </div>

      <p>{result.summary}</p>
      <small>Frontend mock only. Not verified executor output.</small>
    </article>
  );
}
