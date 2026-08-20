import { Info, ShieldCheck } from "lucide-react";

import { COMMAND_CENTER_DISCLOSURE } from "../commandCenterProjection";

export interface InspectorViewModel {
  readonly approval: string;
  readonly assignment: string;
  readonly authorityBoundary: string;
  readonly availability: string;
  readonly demoOrigin: string;
  readonly dependencies: readonly string[];
  readonly domain: string;
  readonly facts: readonly Readonly<{ label: string; value: string }>[];
  readonly findings: readonly string[];
  readonly health: string;
  readonly id: string;
  readonly inputs: readonly string[];
  readonly kind: string;
  readonly label: string;
  readonly outputs: readonly string[];
  readonly recentActivity: readonly string[];
  readonly responsibility: string;
  readonly status: string;
  readonly trust: string;
  readonly unresolved: readonly string[];
}

interface ContextualInspectorProps {
  readonly selection: InspectorViewModel | null;
}

function readable(value: string): string {
  return value.replaceAll("-", " ");
}

export function ContextualInspector({ selection }: ContextualInspectorProps) {
  return (
    <aside
      aria-labelledby="command-center-inspector-title"
      className="command-center-inspector command-center-panel"
      data-scroll-region="command-center-inspector"
    >
      <header className="command-center-inspector__header">
        <div>
          <h2 id="command-center-inspector-title">Contextual inspector</h2>
          <p>
            Read-only fixture detail. No action can be dispatched here. ·{" "}
            {COMMAND_CENTER_DISCLOSURE}
          </p>
        </div>
        <ShieldCheck aria-hidden="true" size={18} />
      </header>

      <div className="command-center-inspector__body">
        {selection === null ? (
          <div className="command-center-empty">
            <Info aria-hidden="true" />
            <h3>Select a simulated entity</h3>
            <p>Choose an item in the graph or structured view to inspect its bounded facts.</p>
          </div>
        ) : (
          <>
            <section className="command-center-inspector__section">
              <h3>{selection.label}</h3>
              <dl>
                <dt>Presentation ID</dt>
                <dd>{selection.id}</dd>
                <dt>Kind</dt>
                <dd>{readable(selection.kind)}</dd>
                <dt>Domain</dt>
                <dd>{selection.domain}</dd>
                <dt>Status</dt>
                <dd>{readable(selection.status)}</dd>
                <dt>Availability</dt>
                <dd>{readable(selection.availability)}</dd>
                <dt>Health</dt>
                <dd>{readable(selection.health)}</dd>
                <dt>Trust</dt>
                <dd>{readable(selection.trust)}</dd>
                <dt>Approval</dt>
                <dd>{readable(selection.approval)}</dd>
                <dt>Origin</dt>
                <dd>{readable(selection.demoOrigin)}</dd>
              </dl>
            </section>

            <section className="command-center-inspector__section">
              <h3>Responsibility and authority</h3>
              <p className="command-center-inspector__note">{selection.responsibility}</p>
              <p className="command-center-inspector__note">{selection.authorityBoundary}</p>
            </section>

            <section className="command-center-inspector__section">
              <h3>Current simulated assignment</h3>
              <p className="command-center-inspector__note">{selection.assignment}</p>
            </section>

            <section className="command-center-inspector__section">
              <h3>Fixture facts</h3>
              {selection.facts.length === 0 ? (
                <p className="command-center-inspector__note">
                  No additional fact is available in this fixture.
                </p>
              ) : (
                <dl>
                  {selection.facts.map((fact) => (
                    <div className="command-center-inspector__fact" key={fact.label}>
                      <dt>{fact.label}</dt>
                      <dd>{fact.value}</dd>
                    </div>
                  ))}
                </dl>
              )}
            </section>

            <section className="command-center-inspector__section">
              <h3>Inputs, outputs, and dependencies</h3>
              {[
                ["Inputs", selection.inputs],
                ["Outputs", selection.outputs],
                ["Dependencies", selection.dependencies],
              ].map(([label, values]) => (
                <div className="command-center-inspector__bounded-list" key={label as string}>
                  <h4>{label}</h4>
                  {(values as readonly string[]).length === 0 ? (
                    <p className="command-center-inspector__note">
                      Unavailable in this fixture state.
                    </p>
                  ) : (
                    <ul>
                      {(values as readonly string[]).map((value) => (
                        <li key={value}>{value}</li>
                      ))}
                    </ul>
                  )}
                </div>
              ))}
            </section>

            <section className="command-center-inspector__section">
              <h3>Fixture findings</h3>
              {selection.findings.length === 0 ? (
                <p className="command-center-inspector__note">No finding in this fixture state.</p>
              ) : (
                <ul>
                  {selection.findings.map((finding) => (
                    <li key={finding}>{finding}</li>
                  ))}
                </ul>
              )}
            </section>

            <section className="command-center-inspector__section">
              <h3>Failures and unresolved issues</h3>
              {selection.unresolved.length === 0 ? (
                <p className="command-center-inspector__note">No unresolved fixture issue.</p>
              ) : (
                <ul>
                  {selection.unresolved.map((issue) => (
                    <li key={issue}>{issue}</li>
                  ))}
                </ul>
              )}
            </section>

            <section className="command-center-inspector__section">
              <h3>Recent deterministic activity</h3>
              {selection.recentActivity.length === 0 ? (
                <p className="command-center-inspector__note">No related fixture event.</p>
              ) : (
                <ol>
                  {selection.recentActivity.map((activity) => (
                    <li key={activity}>{activity}</li>
                  ))}
                </ol>
              )}
            </section>
          </>
        )}
      </div>
    </aside>
  );
}
