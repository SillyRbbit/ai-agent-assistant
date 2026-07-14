import type { ConversationSession } from "../application/conversations";
import { NAVIGATION_ITEMS, type AppRoute } from "../application/navigation";

interface ApplicationSidebarProps {
  readonly activeConversationId: string;
  readonly activeRoute: AppRoute;
  readonly conversationNavigationDisabled: boolean;
  readonly conversations: readonly ConversationSession[];
  readonly onNavigate: (route: AppRoute) => void;
  readonly onNewConversation: () => void;
  readonly onSelectConversation: (conversationId: string) => void;
}

export function ApplicationSidebar({
  activeConversationId,
  activeRoute,
  conversationNavigationDisabled,
  conversations,
  onNavigate,
  onNewConversation,
  onSelectConversation,
}: ApplicationSidebarProps) {
  const newestFirstConversations = [...conversations].reverse();

  return (
    <aside className="application-sidebar">
      <div className="application-brand">
        <div className="application-brand__mark" aria-hidden="true">
          A
        </div>
        <div className="application-brand__copy">
          <strong>AI Agent Assistant</strong>
          <span>Private workspace</span>
        </div>
      </div>

      <button
        aria-label="Start new conversation"
        className="application-sidebar__new-conversation"
        disabled={conversationNavigationDisabled}
        onClick={onNewConversation}
        type="button"
      >
        <span aria-hidden="true">+</span>
        New conversation
      </button>

      <section aria-labelledby="conversation-history-label" className="conversation-navigation">
        <p className="navigation-label" id="conversation-history-label">
          Conversations
        </p>
        <ul aria-label="Conversation history" className="conversation-navigation__list">
          {newestFirstConversations.map((conversation) => (
            <li key={conversation.id}>
              <button
                aria-label={`Open conversation: ${conversation.title}`}
                aria-current={conversation.id === activeConversationId ? "page" : undefined}
                className="conversation-navigation__item"
                disabled={conversationNavigationDisabled}
                onClick={() => {
                  onSelectConversation(conversation.id);
                }}
                type="button"
              >
                {conversation.title}
              </button>
            </li>
          ))}
        </ul>
      </section>

      <nav className="primary-navigation" aria-label="Primary navigation">
        <p className="navigation-label">Workspace</p>
        <ul>
          {NAVIGATION_ITEMS.map((item) => (
            <li key={item.route}>
              <button
                aria-current={activeRoute === item.route ? "page" : undefined}
                aria-label={item.label}
                className="primary-navigation__item"
                onClick={() => {
                  onNavigate(item.route);
                }}
                title={item.description}
                type="button"
              >
                <span className="primary-navigation__icon" aria-hidden="true">
                  {item.glyph}
                </span>
                <span>{item.label}</span>
              </button>
            </li>
          ))}
        </ul>
      </nav>

      <div className="application-sidebar__footer">
        <span className="local-status-dot" aria-hidden="true" />
        <div>
          <strong>Local-first mode</strong>
          <span>No cloud account connected</span>
        </div>
      </div>
    </aside>
  );
}
