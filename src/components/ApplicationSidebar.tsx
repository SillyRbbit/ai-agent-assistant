import { NAVIGATION_ITEMS, type AppRoute } from "../application/navigation";

interface ApplicationSidebarProps {
  readonly activeRoute: AppRoute;
  readonly onNavigate: (route: AppRoute) => void;
}

export function ApplicationSidebar({ activeRoute, onNavigate }: ApplicationSidebarProps) {
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
