import { CircleOff, Info, PanelRightClose, ShieldCheck } from "lucide-react";
import type { Ref } from "react";

import { COMMAND_CENTER_DISCLOSURE } from "../commandCenterProjection";
import { commandCenterEventSeverityLabel } from "../commandCenterEventPresentation";

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

export interface WorkspaceInspectorSection {
  readonly facts?: readonly Readonly<{ label: string; value: string }>[] | undefined;
  readonly items?: readonly string[] | undefined;
  readonly notes?: readonly string[] | undefined;
  readonly title: string;
}

export interface WorkspaceInspectorViewModel {
  readonly description: string;
  readonly entityType: string;
  readonly facts: readonly Readonly<{ label: string; value: string }>[];
  readonly id: string;
  readonly label: string;
  readonly provenance: string;
  readonly sections: readonly WorkspaceInspectorSection[];
  readonly state: Readonly<{ label: string; term: "Severity" | "Simulated status" }> | null;
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

interface WorkspaceContextualInspectorHeaderProps {
  readonly closeButtonRef?: Ref<HTMLButtonElement> | undefined;
  readonly onClose: () => void;
  readonly selection: WorkspaceInspectorViewModel | null;
  readonly selectionUnavailable?: boolean | undefined;
}

export function WorkspaceContextualInspectorHeader({
  closeButtonRef,
  onClose,
  selection,
  selectionUnavailable = false,
}: WorkspaceContextualInspectorHeaderProps) {
  return (
    <div className="application-panel-header command-center-workspace-inspector__header">
      <div>
        <span className="application-panel-header__eyebrow">
          {selection?.entityType ??
            (selectionUnavailable ? "Filtered fixture selection" : "Presentation selection")}
        </span>
        <h2 id="application-inspector-title">
          {selection?.label ??
            (selectionUnavailable ? "Selection not visible" : "Nothing selected")}
        </h2>
        {selection?.state === null || selection === null ? null : (
          <span className="command-center-workspace-inspector__state">
            {selection.state.term} ·{" "}
            {selection.state.term === "Severity"
              ? commandCenterEventSeverityLabel(selection.state.label)
              : readable(selection.state.label)}
          </span>
        )}
      </div>
      <button
        aria-label="Close workspace inspector"
        className="application-panel-header__button"
        data-application-inspector-close="true"
        onClick={onClose}
        ref={closeButtonRef}
        title="Close workspace inspector"
        type="button"
      >
        <PanelRightClose aria-hidden="true" />
      </button>
    </div>
  );
}

interface WorkspaceContextualInspectorBodyProps {
  readonly onClearSelection: () => void;
  readonly selection: WorkspaceInspectorViewModel | null;
  readonly selectionUnavailable?: boolean | undefined;
}

export function WorkspaceContextualInspectorBody({
  onClearSelection,
  selection,
  selectionUnavailable = false,
}: WorkspaceContextualInspectorBodyProps) {
  if (selection === null) {
    return (
      <div className="application-panel-empty command-center-workspace-inspector__empty">
        <strong>
          {selectionUnavailable
            ? "Selected fixture entity is not visible"
            : "No simulated entity selected"}
        </strong>
        <p>
          {selectionUnavailable
            ? "The current local filters hide this selection. Clear the selection or adjust filters."
            : "Select a graph entity or deterministic fixture event to inspect its bounded context."}
        </p>
        {selectionUnavailable ? (
          <button className="command-center-button" onClick={onClearSelection} type="button">
            <CircleOff aria-hidden="true" /> Clear selection
          </button>
        ) : null}
      </div>
    );
  }

  return (
    <div className="command-center-workspace-inspector">
      <p className="command-center-workspace-inspector__provenance">
        <ShieldCheck aria-hidden="true" /> {COMMAND_CENTER_DISCLOSURE} · {selection.provenance}
      </p>

      <section>
        <h3>Overview</h3>
        <p>{selection.description}</p>
        <dl>
          <dt>Presentation ID</dt>
          <dd>{selection.id}</dd>
          {selection.facts.map((fact) => (
            <div className="command-center-workspace-inspector__fact" key={fact.label}>
              <dt>{fact.label}</dt>
              <dd>{fact.value}</dd>
            </div>
          ))}
        </dl>
      </section>

      {selection.sections.map((section) => (
        <section key={section.title}>
          <h3>{section.title}</h3>
          {section.notes?.map((note) => (
            <p key={note}>{note}</p>
          ))}
          {section.facts === undefined || section.facts.length === 0 ? null : (
            <dl>
              {section.facts.map((fact) => (
                <div className="command-center-workspace-inspector__fact" key={fact.label}>
                  <dt>{fact.label}</dt>
                  <dd>{fact.value}</dd>
                </div>
              ))}
            </dl>
          )}
          {section.items === undefined || section.items.length === 0 ? null : (
            <ul>
              {section.items.map((item) => (
                <li key={item}>{item}</li>
              ))}
            </ul>
          )}
        </section>
      ))}

      <button className="command-center-button" onClick={onClearSelection} type="button">
        <CircleOff aria-hidden="true" /> Clear selection
      </button>
    </div>
  );
}
