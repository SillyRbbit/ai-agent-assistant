import {
  Activity,
  Brain,
  LayoutDashboard,
  ListTodo,
  MessagesSquare,
  Plug,
  Plus,
  Settings,
  ShieldCheck,
  type LucideIcon,
} from "lucide-react";

import type { ConversationSession } from "../application/conversations";
import { NAVIGATION_ITEMS, type AppRoute } from "../application/navigation";
import brandFavicon from "../../assets/branding/favicon.png";
import brandLogoDark from "../../assets/branding/logo-dark.png";
import brandLogoLight from "../../assets/branding/logo-light.png";

interface ApplicationSidebarProps {
  readonly activeConversationId: string;
  readonly activeRoute: AppRoute;
  readonly conversationNavigationDisabled: boolean;
  readonly conversations: readonly ConversationSession[];
  readonly expanded: boolean;
  readonly onNavigate: (route: AppRoute) => void;
  readonly onNewConversation: () => void;
  readonly onSelectConversation: (conversationId: string) => void;
}

const NAVIGATION_ICONS: Readonly<Record<AppRoute, LucideIcon>> = {
  activity: Activity,
  "command-center": LayoutDashboard,
  conversations: MessagesSquare,
  integrations: Plug,
  memory: Brain,
  permissions: ShieldCheck,
  settings: Settings,
  tasks: ListTodo,
};

export function ApplicationSidebar({
  activeConversationId,
  activeRoute,
  conversationNavigationDisabled,
  conversations,
  expanded,
  onNavigate,
  onNewConversation,
  onSelectConversation,
}: ApplicationSidebarProps) {
  const newestFirstConversations = [...conversations].reverse();

  return (
    <aside
      aria-label="Cortexa workspace navigation"
      className={`application-sidebar${expanded ? "" : " application-sidebar--collapsed"}`}
      data-expanded={expanded}
      data-scroll-region="application-sidebar"
      id="application-sidebar"
    >
      <div className="application-brand">
        {expanded ? (
          <picture className="application-brand__mark" aria-hidden="true">
            <source media="(prefers-color-scheme: dark)" srcSet={brandLogoDark} />
            <img
              alt=""
              className="application-brand__logo"
              height="434"
              src={brandLogoLight}
              width="360"
            />
          </picture>
        ) : (
          <img
            alt="Cortexa"
            className="application-brand__favicon"
            height="64"
            src={brandFavicon}
            width="64"
          />
        )}
        <div className="application-brand__copy">
          <strong>Cortexa</strong>
          <span>Private workspace</span>
        </div>
      </div>

      <button
        aria-label="Start new conversation"
        className="application-sidebar__new-conversation"
        disabled={conversationNavigationDisabled}
        onClick={onNewConversation}
        title="Start new conversation"
        type="button"
      >
        <Plus aria-hidden="true" />
        <span className="application-sidebar__label">New conversation</span>
      </button>

      {expanded ? (
        <section aria-labelledby="conversation-history-label" className="conversation-navigation">
          <p className="navigation-label" id="conversation-history-label">
            Conversations
          </p>
          <ul
            aria-label="Conversation history"
            className="conversation-navigation__list"
            data-scroll-region="conversation-list-scroll"
          >
            {newestFirstConversations.map((conversation) => (
              <li key={conversation.id}>
                <button
                  aria-label={`Open conversation: ${conversation.title}`}
                  aria-current={
                    activeRoute === "conversations" && conversation.id === activeConversationId
                      ? "page"
                      : undefined
                  }
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
      ) : null}

      <nav className="primary-navigation" aria-label="Primary navigation">
        <p className="navigation-label" hidden={!expanded}>
          Workspace
        </p>
        <ul data-scroll-region="primary-navigation-scroll">
          {NAVIGATION_ITEMS.map((item) => {
            const Icon = NAVIGATION_ICONS[item.route];
            const tooltipId = `navigation-tooltip-${item.route}`;

            return (
              <li key={item.route}>
                <button
                  aria-current={activeRoute === item.route ? "page" : undefined}
                  aria-describedby={expanded ? undefined : tooltipId}
                  aria-label={item.label}
                  className="primary-navigation__item"
                  onClick={() => {
                    onNavigate(item.route);
                  }}
                  title={`${item.label} — ${item.description}`}
                  type="button"
                >
                  <span className="primary-navigation__icon" aria-hidden="true">
                    <Icon />
                  </span>
                  <span className="primary-navigation__label">{item.label}</span>
                  {!expanded ? (
                    <span className="primary-navigation__tooltip" id={tooltipId} role="tooltip">
                      {item.label}
                    </span>
                  ) : null}
                </button>
              </li>
            );
          })}
        </ul>
      </nav>

      <div
        className="application-sidebar__footer"
        title="Local-first mode · No cloud account connected"
      >
        <span className="local-status-dot" aria-hidden="true" />
        <div className="application-sidebar__footer-copy">
          <strong>Local-first mode</strong>
          <span>No cloud account connected</span>
        </div>
      </div>
    </aside>
  );
}
