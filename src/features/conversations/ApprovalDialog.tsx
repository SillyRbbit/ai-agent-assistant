import type { MockApprovalDecision, MockApprovalRequest } from "../../application/mockAssistantRun";

interface ApprovalDialogProps {
  readonly approval: MockApprovalRequest;
  readonly onDecision: (decision: MockApprovalDecision) => void;
}

export function ApprovalDialog({ approval, onDecision }: ApprovalDialogProps) {
  return (
    <div className="approval-backdrop">
      <section
        aria-describedby="mock-approval-description"
        aria-labelledby="mock-approval-title"
        aria-modal="true"
        className="approval-dialog"
        role="dialog"
      >
        <div className="approval-dialog__header">
          <div>
            <p className="section-kicker">Mock approval preview</p>
            <h2 id="mock-approval-title">{approval.title}</h2>
          </div>
          <span className="approval-dialog__mock-label">No execution</span>
        </div>

        <p id="mock-approval-description" className="approval-dialog__description">
          Review this deterministic in-memory proposal. These controls record a mock decision only.
        </p>

        <dl className="approval-preview">
          <div>
            <dt>Target</dt>
            <dd>{approval.target}</dd>
          </div>
          <div>
            <dt>Affected data</dt>
            <dd>{approval.affectedData}</dd>
          </div>
          <div>
            <dt>Reversible</dt>
            <dd>Yes</dd>
          </div>
          <div>
            <dt>Permission</dt>
            <dd>{approval.permission}</dd>
          </div>
          <div>
            <dt>Main risk</dt>
            <dd>{approval.risk}</dd>
          </div>
        </dl>

        <div className="approval-dialog__actions">
          <button
            className="approval-button approval-button--secondary"
            onClick={() => {
              onDecision("reject");
            }}
            type="button"
          >
            Reject
          </button>
          <button
            className="approval-button approval-button--secondary"
            onClick={() => {
              onDecision("edit");
            }}
            type="button"
          >
            Edit
          </button>
          <button
            className="approval-button approval-button--primary"
            onClick={() => {
              onDecision("approve");
            }}
            type="button"
          >
            Approve mock
          </button>
        </div>
      </section>
    </div>
  );
}
