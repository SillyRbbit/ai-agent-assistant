import type { CoreConnection } from "../../application/useCoreConnection";
import type { MenuRouteConnectionStatus } from "../../application/useMenuRouteSubscription";
import { PageState } from "../../components/PageState";
import { PageHeader } from "../shared/PageHeader";

interface SettingsPageProps {
  readonly coreConnection: CoreConnection;
  readonly menuRouteStatus: MenuRouteConnectionStatus;
}

export function SettingsPage({ coreConnection, menuRouteStatus }: SettingsPageProps) {
  return (
    <section aria-labelledby="settings-page-title" className="page-stack">
      <PageHeader
        description="Local preferences and diagnostics. Nothing on this page is persisted yet."
        headingId="settings-page-title"
        title="Settings"
      />

      <div className="settings-grid">
        <section aria-labelledby="general-settings-heading" className="settings-card page-panel">
          <div className="settings-card__header">
            <div>
              <p className="section-kicker">General</p>
              <h2 id="general-settings-heading">Application behavior</h2>
            </div>
            <span className="settings-tag">In memory</span>
          </div>
          <dl className="settings-list">
            <div>
              <dt>Launch destination</dt>
              <dd>Conversations</dd>
            </div>
            <div>
              <dt>Conversation persistence</dt>
              <dd>Off</dd>
            </div>
            <div>
              <dt>Model access</dt>
              <dd>Not configured</dd>
            </div>
            <div>
              <dt>Permission prompts</dt>
              <dd>None requested</dd>
            </div>
          </dl>
        </section>

        <section aria-labelledby="diagnostics-heading" className="settings-card page-panel">
          <div className="settings-card__header">
            <div>
              <p className="section-kicker">Diagnostics</p>
              <h2 id="diagnostics-heading">Native application connection</h2>
            </div>
            <span className="settings-tag">Read only</span>
          </div>

          <div className="diagnostic-grid" aria-live="polite">
            <div className="diagnostic-panel">
              {coreConnection.status === "checking" ? (
                <PageState
                  description="Waiting for the typed get_app_info response."
                  icon="…"
                  title="Connecting to Rust core"
                  tone="loading"
                />
              ) : null}

              {coreConnection.status === "error" ? (
                <PageState
                  description={coreConnection.message}
                  icon="!"
                  title="Rust core unavailable"
                  tone="error"
                />
              ) : null}

              {coreConnection.status === "ready" ? (
                <>
                  <p className="connection-state connection-state--ready">Rust core connected</p>
                  <dl className="app-metadata">
                    <div>
                      <dt>Command</dt>
                      <dd>get_app_info</dd>
                    </div>
                    <div>
                      <dt>Application</dt>
                      <dd>{coreConnection.info.name}</dd>
                    </div>
                    <div>
                      <dt>Version</dt>
                      <dd>{coreConnection.info.version}</dd>
                    </div>
                    <div>
                      <dt>Target</dt>
                      <dd>
                        {coreConnection.info.target} · {coreConnection.info.architecture}
                      </dd>
                    </div>
                    <div>
                      <dt>Build</dt>
                      <dd>{coreConnection.info.environment}</dd>
                    </div>
                  </dl>
                </>
              ) : null}
            </div>

            <div className="diagnostic-panel diagnostic-panel--compact">
              <p className="diagnostic-label">Native menu routing</p>
              {menuRouteStatus === "checking" ? (
                <p className="connection-state connection-state--checking">Connecting</p>
              ) : null}
              {menuRouteStatus === "ready" ? (
                <p className="connection-state connection-state--ready">Listener active</p>
              ) : null}
              {menuRouteStatus === "error" ? (
                <>
                  <p className="connection-state connection-state--error">Listener unavailable</p>
                  <p className="diagnostic-copy">
                    Sidebar navigation remains available. Restart the desktop app to retry.
                  </p>
                </>
              ) : null}
            </div>
          </div>
        </section>
      </div>
    </section>
  );
}
