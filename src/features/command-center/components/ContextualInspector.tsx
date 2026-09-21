import { CircleOff, PanelRightClose, ShieldCheck } from "lucide-react";
import type { Ref } from "react";

import { COMMAND_CENTER_DISCLOSURE } from "../commandCenterProjection";
import { commandCenterEventSeverityLabel } from "../commandCenterEventPresentation";

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

function readable(value: string): string {
  return value.replaceAll("-", " ");
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
