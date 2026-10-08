import type { ActionEvidence } from "../../infrastructure/tauri/collaboration-client";
export function ActionReview({
  evidence,
  disabled,
  onReview,
}: {
  readonly evidence: ActionEvidence;
  readonly disabled: boolean;
  readonly onReview?: (() => void) | undefined;
}) {
  return (
    <section className="action-review" aria-label="Isolated action evidence">
      <h4>Isolated Python change · {evidence.disposition.replaceAll("_", " ")}</h4>
      <p>
        New file: <code>{evidence.file}</code>. Unchanged checks: <code>{evidence.testFile}</code>.
      </p>
      <p>
        Baseline: <code>{evidence.baseline}</code>. Generation requests: {evidence.requests}/4.
      </p>
      <p>
        Recovery directory: <code>{evidence.recovery}</code>. Original backup and journals are
        retained. No commit or automatic replay.
      </p>
      {evidence.attempts.map((a, i) => (
        <article key={i} aria-label={`Validation attempt ${String(i + 1)}`}>
          <h5>
            Attempt {i + 1}: {a.check.passed ? "Checks passed" : "Checks failed"}
          </h5>
          <p>
            {a.check.command} · exit {a.check.exit ?? "unavailable"} · owned container{" "}
            {a.check.processAbsent ? "absent" : "unverified"}
          </p>
          <p>
            Candidate SHA-256: <code>{a.candidateHash}</code>
          </p>
          <details open>
            <summary>Actual file diff</summary>
            <pre>{a.diff}</pre>
          </details>
          <details>
            <summary>Executed check output (untrusted)</summary>
            <pre>{a.check.output}</pre>
          </details>
          <p>{a.qaSummary || "QA assessment pending"}</p>
          {a.qaHandoff?.limitations.map((v, n) => (
            <p key={n}>{v}</p>
          ))}
        </article>
      ))}
      {evidence.reviewHash && (
        <p>
          Exact review SHA-256: <code>{evidence.reviewHash}</code>
        </p>
      )}
      {evidence.disposition === "review_ready" && (
        <button disabled={disabled || !onReview} onClick={onReview}>
          Review exact change in native approval
        </button>
      )}
      {evidence.disposition === "review_ready" && (
        <p>
          Target matched when this review was prepared; it is checked again before native approval
          and application. Review the actual diff before opening native approval. Approve applies
          only this file after target revalidation; Reject, Edit or closing leaves it unchanged.
        </p>
      )}
      {evidence.disposition === "recovery_required" && (
        <p role="alert">
          No automatic replay. Inspect the retained backup and apply journals before any separately
          approved recovery. A new preview cannot reuse this approval.
        </p>
      )}
    </section>
  );
}
