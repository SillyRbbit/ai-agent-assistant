import type {
  ConversationMessage,
  MockApprovalDecision,
  MockApprovalRequest,
  ToolActivity,
} from "../../application/mockAssistantRun";
import type { AssistantRunStatus } from "../../application/state";
import { PageState } from "../../components/PageState";
import { PageHeader } from "../shared/PageHeader";
import { ApprovalDialog } from "./ApprovalDialog";
import { ToolActivityCard } from "./ToolActivityCard";

interface ConversationWorkspaceProps {
  readonly activeApproval: MockApprovalRequest | null;
  readonly composerDraft: string;
  readonly messages: readonly ConversationMessage[];
  readonly onApprovalDecision: (decision: MockApprovalDecision) => void;
  readonly onComposerDraftChange: (value: string) => void;
  readonly onStop: () => void;
  readonly onSubmit: () => void;
  readonly runStatus: AssistantRunStatus;
  readonly toolActivities: readonly ToolActivity[];
}

export function ConversationWorkspace({
  activeApproval,
  composerDraft,
  messages,
  onApprovalDecision,
  onComposerDraftChange,
  onStop,
  onSubmit,
  runStatus,
  toolActivities,
}: ConversationWorkspaceProps) {
  const isBusy = runStatus !== "idle";

  return (
    <section aria-labelledby="conversations-page-title" className="page-stack conversation-page">
      <PageHeader
        description="A private workspace for requests, context, and reviewed local actions."
        eyebrow="Assistant"
        headingId="conversations-page-title"
        title="Conversations"
      />

      <div className="conversation-workspace">
        <div aria-live="polite" className="conversation-transcript">
          {messages.length === 0 ? (
            <PageState
              description="Use the composer below to start a deterministic mock run. Messages remain in memory only."
              icon="C"
              title="No conversations yet"
            />
          ) : (
            <div className="conversation-timeline">
              {messages.map((message) => (
                <article
                  className={`conversation-message conversation-message--${message.role}`}
                  key={message.id}
                >
                  <span>{message.role === "user" ? "You" : "Assistant"}</span>
                  <p>{message.content || "Preparing mock response…"}</p>
                  {message.status === "stopped" ? <small>Stopped</small> : null}
                </article>
              ))}
              {toolActivities.map((activity) => (
                <ToolActivityCard activity={activity} key={activity.id} />
              ))}
            </div>
          )}
        </div>

        <form
          className="composer-shell"
          onSubmit={(event) => {
            event.preventDefault();
            onSubmit();
          }}
        >
          <label htmlFor="assistant-request">Assistant request</label>
          <div className="composer-shell__field">
            <textarea
              disabled={isBusy}
              id="assistant-request"
              onChange={(event) => {
                onComposerDraftChange(event.currentTarget.value);
              }}
              placeholder="Describe what you need help with…"
              rows={3}
              value={composerDraft}
            />
            {runStatus === "streaming" ? (
              <button className="composer-shell__stop" onClick={onStop} type="button">
                Stop
              </button>
            ) : (
              <button disabled={isBusy || composerDraft.trim().length === 0} type="submit">
                Send
              </button>
            )}
          </div>
          <div className="composer-shell__footer">
            <span>
              {runStatus === "streaming"
                ? "Streaming from the deterministic local mock"
                : runStatus === "awaiting-approval"
                  ? "Waiting for a mock approval decision"
                  : "Local mock only · no model or tool execution"}
            </span>
            <span>{composerDraft.length} characters</span>
          </div>
        </form>
      </div>

      {activeApproval === null ? null : (
        <ApprovalDialog approval={activeApproval} onDecision={onApprovalDecision} />
      )}
    </section>
  );
}
