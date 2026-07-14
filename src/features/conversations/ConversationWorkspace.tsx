import { PageState } from "../../components/PageState";
import { PageHeader } from "../shared/PageHeader";

interface ConversationWorkspaceProps {
  readonly composerDraft: string;
  readonly onComposerDraftChange: (value: string) => void;
}

export function ConversationWorkspace({
  composerDraft,
  onComposerDraftChange,
}: ConversationWorkspaceProps) {
  return (
    <section aria-labelledby="conversations-page-title" className="page-stack conversation-page">
      <PageHeader
        description="A private workspace for requests, context, and reviewed local actions."
        eyebrow="Assistant"
        headingId="conversations-page-title"
        title="Conversations"
      />

      <div className="conversation-workspace">
        <div className="conversation-transcript">
          <PageState
            description="Use the composer below to prepare a request. Messages remain in memory only in this increment."
            icon="C"
            title="No conversations yet"
          />
        </div>

        <form
          className="composer-shell"
          onSubmit={(event) => {
            event.preventDefault();
          }}
        >
          <label htmlFor="assistant-request">Assistant request</label>
          <div className="composer-shell__field">
            <textarea
              id="assistant-request"
              onChange={(event) => {
                onComposerDraftChange(event.currentTarget.value);
              }}
              placeholder="Describe what you need help with…"
              rows={3}
              value={composerDraft}
            />
            <button disabled type="submit">
              Send
            </button>
          </div>
          <div className="composer-shell__footer">
            <span>Mock assistant execution arrives in Increment 2F.</span>
            <span>{composerDraft.length} characters</span>
          </div>
        </form>
      </div>
    </section>
  );
}
