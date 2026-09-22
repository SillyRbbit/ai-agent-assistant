import { Fragment, useState } from "react";
import { PersonalAssistantDirectDemo } from "./PersonalAssistantDirectDemo";

import type { MockContextProvenance } from "../../application/contextProvenance";
import type {
  ConversationMessage,
  MockApprovalDecision,
  MockApprovalRequest,
  ToolActivity,
} from "../../application/mockAssistantRun";
import type { AssistantRunStatus } from "../../application/state";
import type { MockFinalAnswer } from "../../application/mockLoop";
import type { MockToolResult } from "../../application/mockToolResult";
import { PageState } from "../../components/PageState";
import { PageHeader } from "../shared/PageHeader";
import { ApprovalDialog } from "./ApprovalDialog";
import { ContextProvenanceCard } from "./ContextProvenanceCard";
import { FinalAnswerMessage } from "./FinalAnswerMessage";
import { ToolActivityCard } from "./ToolActivityCard";
import { ToolResultCard } from "./ToolResultCard";

interface ConversationWorkspaceProps {
  readonly activeApproval: MockApprovalRequest | null;
  readonly composerDraft: string;
  readonly contextProvenance: readonly MockContextProvenance[];
  readonly conversationTitle: string;
  readonly finalAnswers: readonly MockFinalAnswer[];
  readonly messages: readonly ConversationMessage[];
  readonly onApprovalDecision: (decision: MockApprovalDecision) => void;
  readonly onComposerDraftChange: (value: string) => void;
  readonly onRetry: () => void;
  readonly onStop: () => void;
  readonly onSubmit: () => void;
  readonly runStatus: AssistantRunStatus;
  readonly retryableMessageId: string | null;
  readonly toolActivities: readonly ToolActivity[];
  readonly toolResults: readonly MockToolResult[];
}

export function ConversationWorkspace({
  activeApproval,
  composerDraft,
  contextProvenance,
  conversationTitle,
  finalAnswers,
  messages,
  onApprovalDecision,
  onComposerDraftChange,
  onRetry,
  onStop,
  onSubmit,
  runStatus,
  retryableMessageId,
  toolActivities,
  toolResults,
}: ConversationWorkspaceProps) {
  const [mode, setMode] = useState<"mock" | "native">("mock");
  const isBusy = runStatus !== "idle";
  const canSubmit = !isBusy && composerDraft.trim().length > 0;
  const submitComposer = () => {
    if (canSubmit) {
      onSubmit();
    }
  };

  return (
    <section aria-labelledby="conversations-page-title" className="page-stack conversation-page">
      <PageHeader
        description="A private workspace for requests, context, and reviewed local actions."
        eyebrow="Assistant"
        headingId="conversations-page-title"
        title="Conversations"
      />

      <label>
        Conversation mode{" "}
        <select
          value={mode}
          disabled={isBusy}
          onChange={(event) => {
            setMode(event.currentTarget.value === "native" ? "native" : "mock");
          }}
        >
          <option value="mock">Mock demonstration</option>
          <option value="native">Native live model · synthetic sample</option>
        </select>
      </label>
      {mode === "native" ? (
        <PersonalAssistantDirectDemo />
      ) : (
        <div className="conversation-workspace">
          <div
            aria-label={`Conversation transcript: ${conversationTitle}`}
            aria-live="polite"
            className="conversation-transcript"
            role="region"
          >
            {messages.length === 0 ? (
              <PageState
                description="Use the composer below to start a deterministic mock run. This conversation remains in memory only."
                icon="C"
                title="No messages yet"
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
                    {message.status === "failed" ? (
                      <div className="conversation-message__failure">
                        <small>Failed</small>
                        {message.id === retryableMessageId ? (
                          <button onClick={onRetry} type="button">
                            Retry
                          </button>
                        ) : null}
                      </div>
                    ) : null}
                  </article>
                ))}
                {contextProvenance.map((provenance) => (
                  <ContextProvenanceCard key={provenance.id} provenance={provenance} />
                ))}
                {toolActivities.map((activity) => (
                  <ToolActivityCard activity={activity} key={activity.id} />
                ))}
                {toolResults.map((result) => {
                  const finalAnswer = finalAnswers.find(
                    (answer) =>
                      answer.toolResultId === result.id &&
                      answer.runId === result.runId &&
                      answer.conversationId === result.conversationId,
                  );

                  return (
                    <Fragment key={result.id}>
                      <ToolResultCard result={result} />
                      {finalAnswer === undefined ? null : (
                        <FinalAnswerMessage answer={finalAnswer} />
                      )}
                    </Fragment>
                  );
                })}
              </div>
            )}
          </div>

          <form
            className="composer-shell"
            onSubmit={(event) => {
              event.preventDefault();
              submitComposer();
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
                onKeyDown={(event) => {
                  if (
                    event.key !== "Enter" ||
                    event.altKey ||
                    event.ctrlKey ||
                    event.metaKey ||
                    event.shiftKey ||
                    event.nativeEvent.isComposing ||
                    (event.nativeEvent as { readonly keyCode?: number }).keyCode === 229
                  ) {
                    return;
                  }

                  event.preventDefault();
                  submitComposer();
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
                <button disabled={!canSubmit} type="submit">
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
      )}

      {mode !== "mock" || activeApproval === null ? null : (
        <ApprovalDialog approval={activeApproval} onDecision={onApprovalDecision} />
      )}
    </section>
  );
}
