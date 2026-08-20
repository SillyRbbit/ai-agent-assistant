import { ListTree, Network, RotateCcw, Search } from "lucide-react";

interface SelectOption {
  readonly id: string;
  readonly label: string;
}

interface CommandCenterHeaderProps {
  readonly agentId: string;
  readonly agentOptions: readonly SelectOption[];
  readonly capabilityUnavailable: boolean;
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
  readonly onReset: () => void;
  readonly onScenarioChange: (value: string) => void;
  readonly onSearchChange: (value: string) => void;
  readonly onStatusChange: (value: string) => void;
  readonly onViewModeChange: (value: "graph" | "structured") => void;
  readonly scenarioId: string;
  readonly scenarioOptions: readonly SelectOption[];
  readonly search: string;
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
          <option key={option.id} value={option.id}>
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
  demoOrigin,
  demoOriginOptions,
  domain,
  domainOptions,
  entityKind,
  entityKindOptions,
  onAgentChange,
  onDemoOriginChange,
  onDomainChange,
  onEntityKindChange,
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
