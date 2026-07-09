import { useEffect, useState } from "react";

import { fetchAppInfo, type AppInfo } from "./infrastructure/tauri/app-info-client";

type CoreConnection =
  | { readonly status: "checking" }
  | { readonly info: AppInfo; readonly status: "ready" }
  | { readonly message: string; readonly status: "error" };

function getErrorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export function App() {
  const [connection, setConnection] = useState<CoreConnection>({ status: "checking" });

  useEffect(() => {
    let isMounted = true;

    const connectToCore = async () => {
      try {
        const info = await fetchAppInfo();

        if (isMounted) {
          setConnection({ info, status: "ready" });
        }
      } catch (error: unknown) {
        if (isMounted) {
          setConnection({ message: getErrorMessage(error), status: "error" });
        }
      }
    };

    void connectToCore();

    return () => {
      isMounted = false;
    };
  }, []);

  return (
    <main className="app-shell">
      <header className="app-header">
        <div className="brand-mark" aria-hidden="true">
          A
        </div>
        <div>
          <p className="eyebrow">AI Agent Assistant</p>
          <h1>Secure desktop shell</h1>
        </div>
      </header>

      <section className="status-card" aria-labelledby="status-heading">
        <div>
          <p className="increment-label">Phase 2 · Increment 1</p>
          <h2 id="status-heading">Smallest runnable application</h2>
          <p className="status-copy">
            React is rendered inside a Tauri 2 window and communicates with a typed Rust command. No
            API key, shell plugin, privileged macOS permission, or persistent data store is present
            in this increment.
          </p>
        </div>

        <div className="connection-panel" aria-live="polite">
          {connection.status === "checking" ? (
            <p className="connection-state connection-state--checking">Connecting to Rust core…</p>
          ) : null}

          {connection.status === "ready" ? (
            <>
              <p className="connection-state connection-state--ready">Rust core connected</p>
              <dl className="app-metadata">
                <div>
                  <dt>Application</dt>
                  <dd>{connection.info.name}</dd>
                </div>
                <div>
                  <dt>Version</dt>
                  <dd>{connection.info.version}</dd>
                </div>
                <div>
                  <dt>Target</dt>
                  <dd>
                    {connection.info.target} · {connection.info.architecture}
                  </dd>
                </div>
                <div>
                  <dt>Build</dt>
                  <dd>{connection.info.environment}</dd>
                </div>
              </dl>
            </>
          ) : null}

          {connection.status === "error" ? (
            <>
              <p className="connection-state connection-state--error">Rust core unavailable</p>
              <p className="error-detail">{connection.message}</p>
            </>
          ) : null}
        </div>
      </section>

      <footer className="app-footer">
        <span>Local-first foundation</span>
        <span aria-hidden="true">•</span>
        <span>No credentials stored</span>
      </footer>
    </main>
  );
}
