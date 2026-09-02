import { FlaskConical, ListFilter, ListTree, Network, RotateCcw, Search } from "lucide-react";

import { COMMAND_CENTER_DISCLOSURE } from "../commandCenterProjection";
import type { GraphRelationshipFocus } from "../useCommandCenterState";

interface SelectOption {
  readonly disabled?: boolean;
  readonly id: string;
  readonly label: string;
}

interface CommandCenterHeaderProps {
  readonly agentId: string;
  readonly agentOptions: readonly SelectOption[];
  readonly capabilityUnavailable: boolean;
  readonly compact?: boolean;
  readonly demoOrigin: string;
  readonly demoOriginOptions: readonly SelectOption[];
  readonly domain: string;
  readonly domainOptions: readonly SelectOption[];
  readonly entityKind: string;
  readonly entityKindOptions: readonly SelectOption[];
  readonly onAgentChange: (value: string) => void;
  readonly onDemoOriginChange: (value: string) => void;
  readonly onDomainChange: (value: string) => void;
  readonly onEntityKindChange: (value: string) => void;
  readonly onGraphRelationshipFocusChange: (value: string) => void;
  readonly onReset: () => void;
  readonly onScenarioChange: (value: string) => void;
  readonly onSearchChange: (value: string) => void;
  readonly onStatusChange: (value: string) => void;
  readonly onViewModeChange: (value: "graph" | "structured") => void;
  readonly scenarioId: string;
  readonly scenarioOptions: readonly SelectOption[];
  readonly search: string;
  readonly graphRelationshipFocus: GraphRelationshipFocus;
  readonly graphRelationshipFocusOptions: readonly SelectOption[];
  readonly status: string;
  readonly statusOptions: readonly SelectOption[];
  readonly viewMode: "graph" | "structured";
}

function LabeledSelect({
  label,
  onChange,
  options,
  value,
}: {
  readonly label: string;
  readonly onChange: (value: string) => void;
  readonly options: readonly SelectOption[];
  readonly value: string;
}) {
  return (
    <label className="command-center-field">
      <span>{label}</span>
      <select
        className="command-center-field__control"
        onChange={(event) => {
          onChange(event.target.value);
        }}
        value={value}
      >
        {options.map((option) => (
          <option disabled={option.disabled} key={option.id} value={option.id}>
            {option.label}
          </option>
        ))}
      </select>
    </label>
  );
}

export function CommandCenterHeader({
  agentId,
  agentOptions,
  capabilityUnavailable,
  compact = false,
  demoOrigin,
  demoOriginOptions,
  domain,
  domainOptions,
  entityKind,
  entityKindOptions,
  graphRelationshipFocus,
  graphRelationshipFocusOptions,
  onAgentChange,
  onDemoOriginChange,
  onDomainChange,
  onEntityKindChange,
  onGraphRelationshipFocusChange,
  onReset,
  onScenarioChange,
  onSearchChange,
  onStatusChange,
  onViewModeChange,
  scenarioId,
  scenarioOptions,
  search,
  status,
  statusOptions,
  viewMode,
}: CommandCenterHeaderProps) {
  const activeFilterCount = [
    domain !== "all",
    agentId !== "all",
    status !== "all",
    entityKind !== "all",
    demoOrigin !== "all",
    graphRelationshipFocus !== "all",
  ].filter(Boolean).length;

  if (compact) {
    return (
      <section
        aria-label="Command Center filters"
        className="command-center-controls command-center-controls--graph"
      >
        <label className="command-center-field command-center-field--search">
          <span>Search simulated topology</span>
          <span className="command-center-search">
            <Search aria-hidden="true" />
            <input
              className="command-center-field__control"
              onChange={(event) => {
                onSearchChange(event.target.value);
              }}
              placeholder="Search graph"
              type="search"
              value={search}
            />
          </span>
        </label>

        <span className="command-center-controls__provenance">
          <FlaskConical aria-hidden="true" /> {COMMAND_CENTER_DISCLOSURE}
        </span>

        <details className="command-center-graph-filters">
          <summary className="command-center-button" title="Filters">
            <ListFilter aria-hidden="true" />
            Filters
            <span aria-label={`${String(activeFilterCount)} active filters`}>
              {activeFilterCount}
            </span>
          </summary>
          <div className="command-center-graph-filters__popover">
            <div className="command-center-controls__primary">
              <LabeledSelect
                label="Deterministic scenario"
                onChange={onScenarioChange}
                options={scenarioOptions}
                value={scenarioId}
              />
              <LabeledSelect
                label="Relationship focus"
                onChange={onGraphRelationshipFocusChange}
                options={graphRelationshipFocusOptions}
                value={graphRelationshipFocus}
              />
              <button className="command-center-button" onClick={onReset} type="button">
                <RotateCcw aria-hidden="true" />
                Reset filters
              </button>
            </div>
            <div className="command-center-controls__filters">
              <LabeledSelect
                label="Domain"
                onChange={onDomainChange}
                options={domainOptions}
                value={domain}
              />
              <LabeledSelect
                label="Agent"
                onChange={onAgentChange}
                options={agentOptions}
                value={agentId}
              />
              <LabeledSelect
                label="Status"
                onChange={onStatusChange}
                options={statusOptions}
                value={status}
              />
              <LabeledSelect
                label="Entity"
                onChange={onEntityKindChange}
                options={entityKindOptions}
                value={entityKind}
              />
              <LabeledSelect
                label="Demo origin"
                onChange={onDemoOriginChange}
                options={demoOriginOptions}
                value={demoOrigin}
              />
              <label className="command-center-field">
                <span>Capability</span>
                <select
                  aria-describedby="capability-filter-note-graph"
                  className="command-center-field__control"
                  disabled={capabilityUnavailable}
                  value="unavailable"
                >
                  <option value="unavailable">Unavailable in prototype</option>
                </select>
              </label>
            </div>
            <p className="command-center-graph-filters__note" id="capability-filter-note-graph">
              Tool and memory relationship focus remain unavailable because this bounded fixture
              contains no tool-use or memory-access edges. Capabilities also remain unavailable
              because this prototype has no live agent or runtime data.
            </p>
          </div>
        </details>

        <div className="command-center-controls__view">
          <span className="command-center-filter-label">Topology representation</span>
          <div aria-label="Topology representation" className="command-center-segmented">
            <button
              aria-pressed={viewMode === "graph"}
              onClick={() => {
                onViewModeChange("graph");
              }}
              title="Graph view"
              type="button"
            >
              <Network aria-hidden="true" /> Graph
            </button>
            <button
              aria-pressed={viewMode === "structured"}
              onClick={() => {
                onViewModeChange("structured");
              }}
              title="Structured view"
              type="button"
            >
              <ListTree aria-hidden="true" /> Structured
            </button>
          </div>
        </div>
      </section>
    );
  }

  return (
    <section aria-label="Command Center filters" className="command-center-controls">
      <div className="command-center-controls__primary">
        <label className="command-center-field command-center-field--search">
          <span>Search simulated topology</span>
          <span className="command-center-search">
            <Search aria-hidden="true" />
            <input
              className="command-center-field__control"
              onChange={(event) => {
                onSearchChange(event.target.value);
              }}
              placeholder="Agent, domain, work item, or event"
              type="search"
              value={search}
            />
          </span>
        </label>
        <LabeledSelect
          label="Deterministic scenario"
          onChange={onScenarioChange}
          options={scenarioOptions}
          value={scenarioId}
        />
        <button className="command-center-button" onClick={onReset} type="button">
          <RotateCcw aria-hidden="true" />
          Reset filters
        </button>
      </div>

      <div className="command-center-controls__filters">
        <LabeledSelect
          label="Domain"
          onChange={onDomainChange}
          options={domainOptions}
          value={domain}
        />
        <LabeledSelect
          label="Agent"
          onChange={onAgentChange}
          options={agentOptions}
          value={agentId}
        />
        <LabeledSelect
          label="Status"
          onChange={onStatusChange}
          options={statusOptions}
          value={status}
        />
        <LabeledSelect
          label="Entity"
          onChange={onEntityKindChange}
          options={entityKindOptions}
          value={entityKind}
        />
        <LabeledSelect
          label="Demo origin"
          onChange={onDemoOriginChange}
          options={demoOriginOptions}
          value={demoOrigin}
        />
        <label className="command-center-field">
          <span>Capability</span>
          <select
            aria-describedby="capability-filter-note"
            className="command-center-field__control"
            disabled={capabilityUnavailable}
            value="unavailable"
          >
            <option value="unavailable">Unavailable in prototype</option>
          </select>
        </label>
      </div>

      <div className="command-center-controls__view">
        <span className="command-center-filter-label">Topology representation</span>
        <div aria-label="Topology representation" className="command-center-segmented">
          <button
            aria-pressed={viewMode === "graph"}
            onClick={() => {
              onViewModeChange("graph");
            }}
            type="button"
          >
            <Network aria-hidden="true" /> Graph
          </button>
          <button
            aria-pressed={viewMode === "structured"}
            onClick={() => {
              onViewModeChange("structured");
            }}
            type="button"
          >
            <ListTree aria-hidden="true" /> Structured
          </button>
        </div>
        <span className="command-center-toolbar__help" id="capability-filter-note">
          Capabilities remain unavailable because this prototype has no live agent or runtime data.
        </span>
      </div>
    </section>
  );
}
