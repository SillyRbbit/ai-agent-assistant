import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useMemo, useState } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  ApplicationWorkspacePanelsContext,
  type ApplicationWorkspacePanels,
} from "../../components/applicationWorkspacePanels";
import {
  createResearchKnowledgeDemoLifecycleClient,
  type ResearchKnowledgeDemoLifecycleClient,
  type ResearchKnowledgeDemoLifecycleSnapshot,
} from "../../infrastructure/tauri/research-knowledge-demo-lifecycle-client";
import type {
  ResearchKnowledgeDemoProjection,
  ResearchKnowledgeDemoProjectionLoader,
} from "../../infrastructure/tauri/research-knowledge-demo-projection-client";
import CommandCenterPage from "./CommandCenterPage";
import {
  COMMAND_CENTER_AGENT_IDS,
  COMMAND_CENTER_DISCLOSURE,
  COMMAND_CENTER_SCENARIO_IDS,
} from "./commandCenterProjection";

vi.mock("../../infrastructure/tauri/research-knowledge-demo-lifecycle-client", () => ({
  createResearchKnowledgeDemoLifecycleClient: vi.fn(),
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_DISCLOSURE: "DEMO MODE · SIMULATED AGENT DATA",
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_PROOF_BOUNDARY:
    "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE: "Research/Knowledge demo lifecycle unavailable.",
}));

class ResizeObserverMock {
  disconnect(): void {
    return;
  }

  observe(): void {
    return;
  }

  unobserve(): void {
    return;
  }
}

vi.stubGlobal("ResizeObserver", ResizeObserverMock);

const rustProjection: ResearchKnowledgeDemoProjection = {
  schemaVersion: "research-knowledge-demo-projection-v1",
  scenarioId: "research-knowledge-demo-v1",
  disclosure: "DEMO MODE · SIMULATED AGENT DATA",
  proofBoundary:
    "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
  fixtureProvenance: "application-owned-synthetic-fixture",
  roles: [
    { id: "personal-assistant", label: "Personal Assistant", state: "ready" },
    { id: "research", label: "Research Agent", state: "ready" },
    { id: "knowledge-document", label: "Knowledge & Document Agent", state: "ready" },
  ],
  simulatedOutcomes: ["succeeded", "failed", "cancelled"],
};

const idleLifecycleSnapshot: ResearchKnowledgeDemoLifecycleSnapshot = {
  schemaVersion: "research-knowledge-demo-lifecycle-v1",
  scenarioId: "research-knowledge-demo-v1",
  disclosure: "DEMO MODE · SIMULATED AGENT DATA",
  proofBoundary:
    "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
  fixtureProvenance: "application-owned-synthetic-fixture",
  presentationEpoch: 0,
  revision: 0,
  state: "idle",
  journal: [],
};

function createLifecycleClientDouble() {
  const snapshot = vi.fn(() => Promise.resolve(idleLifecycleSnapshot));
  const start = vi.fn(() => Promise.resolve(idleLifecycleSnapshot));
  const advance = vi.fn(() => Promise.resolve(idleLifecycleSnapshot));
  const cancel = vi.fn(() => Promise.resolve(idleLifecycleSnapshot));
  const dispose = vi.fn();
  const client: ResearchKnowledgeDemoLifecycleClient = {
    current: idleLifecycleSnapshot,
    recoveryRequired: false,
    advance,
    cancel,
    dispose,
    snapshot,
    start,
  };
  return { advance, cancel, client, dispose, snapshot, start };
}

const createLifecycleClient = vi.mocked(createResearchKnowledgeDemoLifecycleClient);

beforeEach(() => {
  createLifecycleClient.mockReset();
  createLifecycleClient.mockResolvedValue(createLifecycleClientDouble().client);
});

function openStructuredView(): void {
  fireEvent.click(screen.getByRole("button", { name: "Structured" }));
}

function selectScenario(label: string): void {
  fireEvent.change(screen.getByLabelText("Deterministic scenario"), {
    target: { value: label },
  });
}

interface CommandCenterTestShellProps {
  readonly projectionLoader?: ResearchKnowledgeDemoProjectionLoader;
}

function CommandCenterTestShell({ projectionLoader }: CommandCenterTestShellProps) {
  const [activityBodyTarget, setActivityBodyTarget] = useState<HTMLDivElement | null>(null);
  const [activitySummaryTarget, setActivitySummaryTarget] = useState<HTMLSpanElement | null>(null);
  const [inspectorBodyTarget, setInspectorBodyTarget] = useState<HTMLDivElement | null>(null);
  const [inspectorHeaderTarget, setInspectorHeaderTarget] = useState<HTMLDivElement | null>(null);
  const workspacePanels = useMemo<ApplicationWorkspacePanels>(
    () => ({
      activityBodyTarget,
      activitySummaryTarget,
      closeInspector: () => undefined,
      inspectorBodyTarget,
      inspectorHeaderTarget,
      openInspector: () => undefined,
    }),
    [activityBodyTarget, activitySummaryTarget, inspectorBodyTarget, inspectorHeaderTarget],
  );

  return (
    <ApplicationWorkspacePanelsContext.Provider value={workspacePanels}>
      <CommandCenterPage {...(projectionLoader === undefined ? {} : { projectionLoader })} />
      <aside aria-labelledby="application-inspector-title" data-testid="test-shell-inspector">
        <div ref={setInspectorHeaderTarget} />
        <div ref={setInspectorBodyTarget} />
      </aside>
      <section aria-labelledby="test-shell-activity-title" data-testid="test-shell-activity">
        <h2 id="test-shell-activity-title">Activity</h2>
        <span ref={setActivitySummaryTarget} />
        <div ref={setActivityBodyTarget} />
      </section>
    </ApplicationWorkspacePanelsContext.Provider>
  );
}

function renderCommandCenter(projectionLoader?: ResearchKnowledgeDemoProjectionLoader) {
  return render(
    <CommandCenterTestShell {...(projectionLoader === undefined ? {} : { projectionLoader })} />,
  );
}

describe("CommandCenterPage", () => {
  it("owns the full route width without changing the shared page-stack contract", () => {
    renderCommandCenter();

    const page = screen
      .getByRole("heading", { level: 1, name: "Command Center" })
      .closest("section");
    expect(page).toHaveClass("page-stack", "command-center-page", "command-center-page--graph");
    expect(page?.querySelector(".command-center-graph-shell")).not.toBeNull();
    const context = page?.querySelector<HTMLDetailsElement>(".command-center-graph-context");
    expect(context).not.toHaveAttribute("open");
    expect(page?.querySelector(".command-center-graph-inline-inspector")).toBeNull();
    expect(page?.querySelector(".command-center-inspector")).toBeNull();
    expect(page?.querySelector(".command-center-activity")).toBeNull();
    expect(screen.getByTestId("test-shell-inspector")).toHaveTextContent("AgentOrchestrator");
    expect(screen.getByTestId("test-shell-activity")).toHaveTextContent("simulated fixture events");

    openStructuredView();
    expect(page).toHaveClass("command-center-page--structured");
    expect(page?.querySelector(".command-center-graph-shell")).toBeNull();
    expect(page?.querySelector(".command-center-graph-context")).toBeNull();
    expect(page?.querySelector(".command-center-inspector")).not.toBeNull();
    expect(page?.querySelector(".command-center-activity")).not.toBeNull();
  });

  it("provides tooltips for graph controls that become icon-only", () => {
    const { container } = renderCommandCenter();

    expect(screen.getByRole("button", { name: "Graph" })).toHaveAttribute("title", "Graph view");
    expect(screen.getByRole("button", { name: "Structured" })).toHaveAttribute(
      "title",
      "Structured view",
    );
    expect(container.querySelector(".command-center-graph-filters > summary")).toHaveAttribute(
      "title",
      "Filters",
    );
  });

  it("renders the bounded catalog with persistent demo disclosure and no live claim", () => {
    renderCommandCenter();

    expect(screen.getByRole("heading", { level: 1, name: "Command Center" })).toBeInTheDocument();
    expect(screen.getAllByText(COMMAND_CENTER_DISCLOSURE, { exact: false })).toHaveLength(5);
    expect(document.querySelector(".command-center-controls__provenance")).toHaveTextContent(
      COMMAND_CENTER_DISCLOSURE,
    );
    expect(screen.getByText("command-center-demo-v1", { exact: false })).toBeInTheDocument();
    expect(
      screen.getByRole("listbox", { name: /Simulated operational topology/ }),
    ).toBeInTheDocument();
    expect(screen.getByLabelText("Deterministic scenario").querySelectorAll("option")).toHaveLength(
      COMMAND_CENTER_SCENARIO_IDS.length,
    );
    expect(screen.getByLabelText("Capability")).toBeDisabled();
    expect(document.body).not.toHaveTextContent(/\bLIVE\b/);
  });

  it("renders exactly the nine canonical agent definitions without proof-role duplication", () => {
    renderCommandCenter();
    selectScenario("research-knowledge-active");

    const roster = screen.getByRole("list", { name: "Canonical agent roster" });
    const agentRows = within(roster).getAllByRole("button");
    const agentIds = agentRows.map((row) => row.getAttribute("data-agent-id"));

    expect(agentRows).toHaveLength(9);
    expect(agentIds).toEqual(COMMAND_CENTER_AGENT_IDS);
    expect(new Set(agentIds).size).toBe(9);
    expect(roster).not.toHaveTextContent("AgentOrchestrator");
    expect(roster.closest("section")).toHaveTextContent(
      "Definitions · deterministic fixture representation",
    );
  });

  it("shows truthful empty and unavailable operational states without optimistic claims", () => {
    renderCommandCenter();

    const status = screen.getByRole("region", { name: "Simulated system status" });
    const activeWork = screen.getByRole("heading", { name: "Active Work" }).closest("section");
    const attention = screen
      .getByRole("heading", { name: "Approvals & Attention" })
      .closest("section");

    expect(status).toHaveTextContent("Runtime data unavailable");
    expect(status).toHaveTextContent("Tool data unavailable");
    expect(status).toHaveTextContent("Provider data unavailable");
    expect(activeWork).toHaveTextContent("No simulated active work");
    expect(attention).toHaveTextContent("No simulated attention item");
    expect(document.body).toHaveTextContent(
      "Runtime instances, tool execution, provider health, costs, and elapsed time are unavailable",
    );
    expect(document.body).not.toHaveTextContent(
      /\b(?:live data|healthy|online|provider connected|tools available)\b/i,
    );
  });

  it("uses overview agent, task, approval, and failure controls to update the inspector", () => {
    renderCommandCenter();
    const inspector = screen.getByTestId("test-shell-inspector");

    fireEvent.click(
      screen.getByRole("button", {
        name: /Inspect Research Agent, canonical agent definition, simulated status Idle/,
      }),
    );
    expect(within(inspector).getByRole("heading", { name: "Research Agent" })).toBeInTheDocument();

    selectScenario("research-queued");
    fireEvent.click(
      screen.getByRole("button", {
        name: /Inspect Research analysis, Task, simulated status Queued/,
      }),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Research analysis" }),
    ).toBeInTheDocument();

    selectScenario("engineering-waiting-approval");
    const approvalQueue = screen.getByRole("list", {
      name: "Simulated approvals and attention",
    });
    fireEvent.click(
      within(approvalQueue).getByRole("button", {
        name: /Inspect Application approval checkpoint, Simulated approval required/,
      }),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Application approval checkpoint" }),
    ).toBeInTheDocument();
    expect(
      within(approvalQueue).queryByRole("button", { name: /approve|reject|execute|dispatch/i }),
    ).toBeNull();

    selectScenario("infrastructure-blocked");
    const failureQueue = screen.getByRole("list", {
      name: "Simulated approvals and attention",
    });
    fireEvent.click(
      within(failureQueue).getByRole("button", {
        name: /Inspect Security prerequisite, Simulated failure/,
      }),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Security prerequisite" }),
    ).toBeInTheDocument();
  });

  it("mounts both separate Rust proofs only in the active scenario and disposes the lifecycle on exit", async () => {
    const loader = vi.fn(() => Promise.resolve(rustProjection));
    const lifecycle = createLifecycleClientDouble();
    createLifecycleClient.mockResolvedValueOnce(lifecycle.client);
    renderCommandCenter(loader);

    expect(screen.queryByRole("region", { name: "Read-only Rust demo projection" })).toBeNull();
    expect(
      screen.queryByRole("region", { name: "Research/Knowledge simulated lifecycle" }),
    ).toBeNull();
    for (const scenarioId of COMMAND_CENTER_SCENARIO_IDS.filter(
      (id) => id !== "research-knowledge-active",
    )) {
      selectScenario(scenarioId);
      expect(screen.queryByRole("region", { name: "Read-only Rust demo projection" })).toBeNull();
      expect(
        screen.queryByRole("region", { name: "Research/Knowledge simulated lifecycle" }),
      ).toBeNull();
    }

    selectScenario("research-knowledge-active");
    const contextSummary = screen.getByText("Operational context").closest("summary");
    if (contextSummary === null) throw new Error("Expected operational context disclosure");
    fireEvent.click(contextSummary);
    const panel = screen.getByRole("region", { name: "Read-only Rust demo projection" });
    const lifecyclePanel = screen.getByRole("region", {
      name: "Research/Knowledge simulated lifecycle",
    });
    expect(loader).not.toHaveBeenCalled();
    expect(panel).toHaveTextContent("DEMO MODE · SIMULATED AGENT DATA");
    expect(panel).toHaveTextContent(
      "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
    );
    expect(
      within(panel).queryByRole("button", { name: /start|cancel|approve|execute/i }),
    ).toBeNull();
    await waitFor(() => {
      expect(lifecycle.snapshot).toHaveBeenCalledOnce();
    });
    expect(createLifecycleClient).toHaveBeenCalledOnce();
    expect(lifecycle.start).not.toHaveBeenCalled();
    expect(lifecycle.advance).not.toHaveBeenCalled();
    expect(lifecycle.cancel).not.toHaveBeenCalled();
    expect(lifecyclePanel).toHaveTextContent("DEMO MODE · SIMULATED AGENT DATA");
    expect(lifecyclePanel).toHaveTextContent(
      "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs.",
    );

    fireEvent.click(within(panel).getByRole("button", { name: "Refresh Rust projection" }));

    expect(
      await within(panel).findByText(
        "Read-only Rust projection received. No workflow was started.",
      ),
    ).toBeVisible();
    expect(loader).toHaveBeenCalledOnce();

    selectScenario("workflow-completed");
    expect(screen.queryByRole("region", { name: "Read-only Rust demo projection" })).toBeNull();
    expect(
      screen.queryByRole("region", { name: "Research/Knowledge simulated lifecycle" }),
    ).toBeNull();
    await waitFor(() => {
      expect(lifecycle.dispose).toHaveBeenCalledOnce();
    });
  });

  it("renders every exact agent name and complete accessible label in graph nodes", () => {
    const { container } = renderCommandCenter();
    const exactAgentNames = [
      "Personal Assistant",
      "Workflow Automation Agent",
      "Research Agent",
      "Knowledge & Document Agent",
      "Coding Agent",
      "QA & Validation Agent",
      "Cloud Infrastructure Agent",
      "Systems Operations Agent",
      "Security & Risk Agent",
    ] as const;

    const graphNodes = Array.from(container.querySelectorAll<HTMLElement>(".command-center-node"));
    expect(graphNodes).toHaveLength(10);
    for (const name of exactAgentNames) {
      const option = graphNodes.find((node) =>
        node.getAttribute("aria-label")?.startsWith(name + ", agent,"),
      );
      if (option === undefined) throw new Error("Expected graph node " + name);
      expect(option).toHaveAttribute("role", "option");
      expect(option).toHaveTextContent(name);
      const title = within(option).getByTitle(name);
      expect(title).toHaveTextContent(name);
      expect(title).toHaveClass("command-center-node__title");
      const metadata = option.querySelector(".command-center-node__meta");
      expect(metadata).not.toBeNull();
      expect(metadata).toHaveTextContent("idle");
    }
  });

  it("keeps all nine exact agents available in the synchronized structured view", () => {
    renderCommandCenter();
    openStructuredView();

    const agentSelect = screen.getByLabelText<HTMLSelectElement>("Agent");
    const agentOptionValues = Array.from(agentSelect.options).map((option) => option.value);
    for (const agentId of COMMAND_CENTER_AGENT_IDS) {
      expect(agentOptionValues).toContain(agentId);
      fireEvent.change(agentSelect, { target: { value: agentId } });
      expect(agentSelect).toHaveValue(agentId);
    }
    fireEvent.change(agentSelect, { target: { value: "all" } });

    expect(screen.getByRole("heading", { name: "Relationship table" })).toBeInTheDocument();
    expect(screen.getByRole("columnheader", { name: "Relationship" })).toBeInTheDocument();
    expect(
      screen.queryByRole("listbox", { name: /Simulated operational topology/ }),
    ).not.toBeInTheDocument();
  });

  it("synchronizes structured selection with the read-only inspector", () => {
    renderCommandCenter();
    openStructuredView();

    const researchAgent = screen
      .getAllByRole("button", { name: /Research Agent/ })
      .find((button) => button.classList.contains("command-center-entity-button"));
    if (researchAgent === undefined) throw new Error("Expected the structured Research Agent row");
    fireEvent.click(researchAgent);
    const inspector = screen.getByRole("complementary", { name: "Contextual inspector" });
    expect(within(inspector).getByRole("heading", { name: "Research Agent" })).toBeInTheDocument();
    expect(inspector).toHaveTextContent("Advisory analysis only");
    expect(inspector).toHaveTextContent("deterministic fixture");
    expect(inspector).toHaveTextContent("Bounded deterministic fixture data");
    expect(inspector).toHaveTextContent("Advisory presentation finding");
    expect(inspector).toHaveTextContent("Available only in this fixture");

    fireEvent.click(screen.getByRole("button", { name: /Intelligence/ }));
    expect(within(inspector).getByRole("heading", { name: "Intelligence" })).toBeInTheDocument();
    expect(inspector).toHaveTextContent("Presentation grouping grants no route");
  });

  it("clears Graph filters without resetting scenario, selection, or activity state", () => {
    renderCommandCenter();
    selectScenario("research-knowledge-active");
    fireEvent.click(
      screen.getByRole("button", {
        name: /Inspect Research Agent, canonical agent definition, simulated status Running/,
      }),
    );
    fireEvent.change(screen.getByLabelText("Severity"), { target: { value: "info" } });
    fireEvent.change(screen.getByLabelText("Relationship focus"), {
      target: { value: "dependencies" },
    });

    fireEvent.change(screen.getByLabelText("Search simulated topology"), {
      target: { value: "no-such-fixture-entity" },
    });
    const emptyHeading = screen.getByRole("heading", { name: "No graph results" });
    expect(emptyHeading).toBeInTheDocument();

    const emptyState = emptyHeading.closest<HTMLElement>(".command-center-graph-state");
    if (emptyState === null) throw new Error("Expected the Graph no-result state");
    expect(screen.getByRole("toolbar", { name: "Topology viewport" })).toBeInTheDocument();
    const resetButton = within(emptyState).getByRole("button", { name: "Reset filters" });
    fireEvent.click(resetButton);
    expect(
      screen.getByRole("listbox", { name: /Simulated operational topology/ }),
    ).toBeInTheDocument();
    expect(screen.getByLabelText("Search simulated topology")).toHaveValue("");
    expect(screen.getByLabelText("Deterministic scenario")).toHaveValue(
      "research-knowledge-active",
    );
    expect(screen.getByLabelText("Relationship focus")).toHaveValue("all");
    expect(screen.getByLabelText("Severity")).toHaveValue("info");
    expect(
      within(screen.getByTestId("test-shell-inspector")).getByRole("heading", {
        name: "Research Agent",
      }),
    ).toBeInTheDocument();
  });

  it("focuses only existing Graph relationship families without changing Structured rows", () => {
    renderCommandCenter();
    selectScenario("research-knowledge-active");

    const relationshipFocus = screen.getByLabelText<HTMLSelectElement>("Relationship focus");
    expect(
      Array.from(relationshipFocus.options).find((option) => option.value === "tools-unavailable"),
    ).toBeDisabled();
    expect(
      Array.from(relationshipFocus.options).find((option) => option.value === "memory-unavailable"),
    ).toBeDisabled();

    fireEvent.change(relationshipFocus, { target: { value: "dependencies" } });
    fireEvent.click(screen.getByLabelText("Show relationship legend"));
    const legend = screen.getByText("Visible relationships").parentElement;
    expect(legend).toHaveTextContent("Depends On");
    expect(legend).not.toHaveTextContent("Delegates");
    expect(legend).not.toHaveTextContent("Result Flow");

    fireEvent.change(relationshipFocus, { target: { value: "tools-unavailable" } });
    expect(relationshipFocus).toHaveValue("dependencies");
    fireEvent.change(relationshipFocus, { target: { value: "untrusted-webview-value" } });
    expect(relationshipFocus).toHaveValue("dependencies");

    openStructuredView();
    expect(screen.queryByLabelText("Relationship focus")).not.toBeInTheDocument();
    const relationshipSection = screen
      .getByRole("heading", { name: "Relationship table" })
      .closest("section");
    if (relationshipSection === null) throw new Error("Expected Structured relationship table");
    expect(
      within(relationshipSection).getByRole("button", {
        name: /Assigns independent Research work/,
      }),
    ).toBeInTheDocument();
    expect(
      within(relationshipSection).getAllByRole("button", {
        name: /Returns source-attributed findings/,
      }),
    ).toHaveLength(2);
  });

  it("shows blocked and cancellation fixtures truthfully with bounded activity", () => {
    renderCommandCenter();

    selectScenario("infrastructure-blocked");
    expect(
      screen.getByText("Infrastructure blocked", {
        selector: ".command-center-provenance strong",
      }),
    ).toBeInTheDocument();
    const activity = screen.getByRole("list", { name: "Simulated command center activity" });
    expect(activity).toHaveTextContent("Security prerequisite failed");
    expect(activity).toHaveTextContent("Dependent synthesis blocked");
    fireEvent.change(screen.getByLabelText("Severity"), { target: { value: "danger" } });
    expect(activity).not.toHaveTextContent("Dependent synthesis blocked");
    expect(activity.querySelectorAll("li").length).toBeLessThanOrEqual(48);

    selectScenario("workflow-cancelled");
    expect(
      screen.getByText("Workflow cancelled", {
        selector: ".command-center-provenance strong",
      }),
    ).toBeInTheDocument();
    expect(within(activity).getByText("Child cancellation propagated")).toBeInTheDocument();
    expect(within(activity).queryByText("Proposal-only workflow complete")).not.toBeInTheDocument();
  });

  it("presents danger severity consistently as Error across fixture surfaces", () => {
    renderCommandCenter();
    selectScenario("infrastructure-blocked");

    const severity = screen.getByLabelText<HTMLSelectElement>("Severity");
    expect(
      Array.from(severity.options).find((option) => option.value === "danger"),
    ).toHaveTextContent("Error");

    const context = screen.getByText("Operational context").closest("details");
    if (context === null) throw new Error("Expected Graph operational context");
    expect(within(context).getAllByText("Error")).not.toHaveLength(0);

    const activity = screen.getByTestId("test-shell-activity");
    const event = within(activity).getByRole("button", {
      name: "Inspect deterministic event Security prerequisite failed",
    });
    expect(event).toHaveTextContent("Error");
    fireEvent.click(event);

    const inspector = screen.getByTestId("test-shell-inspector");
    expect(inspector).toHaveTextContent("Severity · Error");
    expect(inspector).toHaveTextContent("SeverityError");
    expect(inspector).not.toHaveTextContent(/Severity(?: · )?danger/i);
  });

  it("uses one composite graph entry and keeps ordinary wheel zoom canvas-local", () => {
    renderCommandCenter();
    const toolbar = screen.getByRole("toolbar", { name: "Topology viewport" });
    fireEvent.click(within(toolbar).getByRole("button", { name: "Clear selection" }));
    const graph = screen.getByRole("listbox", { name: /Simulated operational topology/ });
    const initialActive = graph.getAttribute("aria-activedescendant");
    if (initialActive === null) throw new Error("Expected initial composite descendant");
    expect(document.getElementById(initialActive)).toHaveClass("command-center-node--active");

    graph.focus();
    fireEvent.keyDown(graph, { key: "ArrowRight" });
    const nextActive = graph.getAttribute("aria-activedescendant");
    if (nextActive === null) throw new Error("Expected next composite descendant");
    const nextActiveNode = document.getElementById(nextActive);
    if (nextActiveNode === null) throw new Error("Expected active graph node");
    expect(nextActive).not.toBe(initialActive);
    expect(document.getElementById(initialActive)).not.toHaveClass("command-center-node--active");
    expect(nextActiveNode).toHaveClass("command-center-node--active");
    fireEvent.keyDown(graph, { key: "Enter" });
    expect(screen.getByTestId("test-shell-inspector")).toHaveTextContent(
      /Canonical agent definition|Application orchestrator projection/,
    );

    const renderer = graph.querySelector(".react-flow__renderer");
    if (renderer === null) throw new Error("Expected React Flow renderer");
    const wheel = new WheelEvent("wheel", { bubbles: true, cancelable: true, deltaY: 80 });
    renderer.dispatchEvent(wheel);
    expect(wheel.defaultPrevented).toBe(true);
  });

  it("covers queued, active, approval-wait, and completed fixture states", () => {
    renderCommandCenter();
    openStructuredView();

    selectScenario("research-queued");
    expect(
      screen
        .getAllByRole("button", { name: /Research analysis.*queued/ })
        .some((button) => button.classList.contains("command-center-entity-button")),
    ).toBe(true);

    selectScenario("research-knowledge-active");
    expect(
      screen
        .getAllByRole("button", { name: /Research findings.*running/ })
        .some((button) => button.classList.contains("command-center-entity-button")),
    ).toBe(true);
    expect(
      screen
        .getAllByRole("button", { name: /Knowledge findings.*running/ })
        .some((button) => button.classList.contains("command-center-entity-button")),
    ).toBe(true);

    selectScenario("engineering-waiting-approval");
    expect(
      screen
        .getAllByRole("button", {
          name: /Application approval checkpoint.*waiting approval/,
        })
        .some((button) => button.classList.contains("command-center-entity-button")),
    ).toBe(true);

    selectScenario("workflow-completed");
    fireEvent.change(screen.getByLabelText("Severity"), { target: { value: "success" } });
    expect(
      within(screen.getByRole("list", { name: "Simulated command center activity" })).getByText(
        "Proposal-only workflow complete",
      ),
    ).toBeInTheDocument();
    expect(screen.getByRole("option", { name: "Success" })).toBeInTheDocument();
  });

  it("keeps activity options stable and synchronizes global, agent, and selected-path filters", () => {
    renderCommandCenter();
    selectScenario("research-knowledge-active");

    fireEvent.change(screen.getByLabelText("Search simulated topology"), {
      target: { value: "Knowledge fixture started" },
    });
    let activity = screen.getByRole("list", { name: "Simulated command center activity" });
    expect(within(activity).getByText("Knowledge fixture started")).toBeInTheDocument();
    expect(within(activity).queryByText("Research fixture started")).not.toBeInTheDocument();

    fireEvent.change(screen.getByLabelText("Search simulated topology"), { target: { value: "" } });
    fireEvent.change(screen.getByLabelText("Agent"), { target: { value: "research" } });
    expect(activity).toHaveTextContent("Research fixture started");
    expect(activity).not.toHaveTextContent("Knowledge fixture started");

    fireEvent.change(screen.getByLabelText("Agent"), { target: { value: "all" } });
    openStructuredView();
    activity = screen.getByRole("list", { name: "Simulated command center activity" });
    fireEvent.click(screen.getByRole("button", { name: /Research findings.*running/ }));
    const beforeFollowCount = activity.querySelectorAll("li").length;
    fireEvent.click(screen.getByRole("button", { name: "Follow selected path" }));
    expect(activity.querySelectorAll("li").length).toBeGreaterThan(0);
    expect(activity.querySelectorAll("li").length).toBeLessThan(beforeFollowCount);

    fireEvent.change(screen.getByLabelText("Event kind"), { target: { value: "task-started" } });
    expect(screen.getByRole("option", { name: "task progress" })).toBeInTheDocument();
  });

  it("selects a real fixture event without passing its ID into the topology selection", () => {
    const { container } = renderCommandCenter();
    selectScenario("research-knowledge-active");
    const activity = screen.getByTestId("test-shell-activity");
    const eventButton = within(activity).getByRole("button", {
      name: "Inspect deterministic event Research fixture started",
    });

    expect(eventButton.querySelector("div, p, dl")).toBeNull();
    fireEvent.click(eventButton);

    const inspector = screen.getByTestId("test-shell-inspector");
    expect(
      within(inspector).getByRole("heading", { name: "Research fixture started" }),
    ).toBeInTheDocument();
    expect(inspector).toHaveTextContent("Fixture activity event");
    expect(inspector).toHaveTextContent(
      "The Research presentation lane entered its running state.",
    );
    expect(inspector).toHaveTextContent("Simulated time");
    expect(inspector).toHaveTextContent("SourceUnavailable in fixture data");
    expect(inspector).toHaveTextContent("ActionTask Started");
    expect(inspector).toHaveTextContent("TargetUnavailable in fixture data");
    expect(inspector).toHaveTextContent("SeverityInfo");
    expect(inspector).toHaveTextContent("StatusUnavailable in fixture data");
    expect(inspector).toHaveTextContent("Associated agentResearch Agent");
    expect(inspector).toHaveTextContent("TaskResearch findings");
    expect(inspector).toHaveTextContent("WorkflowBounded parallel analysis");
    expect(inspector).toHaveTextContent("Related entitiesResearch findings, Research Agent");
    expect(inspector).toHaveTextContent("deterministic frontend fixture · simulated time");
    expect(eventButton).toHaveAttribute("aria-pressed", "true");
    expect(container.querySelector(".command-center-node--selected")).toBeNull();

    const toolbar = screen.getByRole("toolbar", { name: "Topology viewport" });
    expect(within(toolbar).getByRole("button", { name: "Clear selection" })).toBeEnabled();
    fireEvent.click(within(activity).getByRole("button", { name: "Follow selected path" }));
    expect(within(activity).getByText("Research fixture started")).toBeInTheDocument();
    expect(
      within(activity).getByRole("list", { name: "Simulated command center activity" }),
    ).not.toBeEmptyDOMElement();

    fireEvent.change(within(activity).getByLabelText("Filter activity"), {
      target: { value: "no-matching-event" },
    });
    expect(
      within(inspector).getByRole("heading", { name: "Selection not visible" }),
    ).toBeInTheDocument();
    expect(inspector).toHaveTextContent("current local filters hide this selection");
    expect(within(toolbar).getByRole("button", { name: "Clear selection" })).toBeEnabled();

    fireEvent.click(within(toolbar).getByRole("button", { name: "Clear selection" }));
    expect(
      within(inspector).getByRole("heading", { name: "Nothing selected" }),
    ).toBeInTheDocument();
    expect(within(toolbar).getByRole("button", { name: "Clear selection" })).toBeDisabled();
  });

  it("selects the same QA fixture event from Recent Activity and the workspace stream", () => {
    const { container } = renderCommandCenter();
    selectScenario("engineering-waiting-approval");
    const context = container.querySelector<HTMLDetailsElement>(".command-center-graph-context");
    if (context === null) throw new Error("Expected Graph operational context");
    fireEvent.click(within(context).getByText("Operational context"));

    const recentActivity = screen.getByRole("list", { name: "Recent deterministic activity" });
    const recentEvent = within(recentActivity).getByRole("button", {
      name: "Inspect deterministic event QA advisory review complete",
    });
    const workspaceActivity = screen.getByTestId("test-shell-activity");
    const workspaceEvent = within(workspaceActivity).getByRole("button", {
      name: "Inspect deterministic event QA advisory review complete",
    });

    fireEvent.click(recentEvent);

    expect(recentEvent).toHaveAttribute("aria-pressed", "true");
    expect(workspaceEvent).toHaveAttribute("aria-pressed", "true");
    const inspector = screen.getByTestId("test-shell-inspector");
    expect(inspector).toHaveTextContent("QA advisory review complete");
    expect(inspector).toHaveTextContent("SourceUnavailable in fixture data");
    expect(inspector).toHaveTextContent("TargetUnavailable in fixture data");
    expect(inspector).toHaveTextContent("StatusUnavailable in fixture data");
    expect(inspector).toHaveTextContent("Associated agentQA & Validation Agent");
    expect(inspector).toHaveTextContent("TaskUnavailable in fixture data");
    expect(inspector).toHaveTextContent("WorkflowEngineering proposal review");
    expect(inspector).toHaveTextContent("Related entitiesQA advisory review");
    expect(workspaceActivity).toHaveTextContent("SourceUnavailable in fixture data");
    expect(workspaceActivity).toHaveTextContent("TargetUnavailable in fixture data");
    expect(workspaceActivity).toHaveTextContent("Associated agentQA & Validation Agent");
  });

  it("retains Structured Recent Activity's topology-context selection", () => {
    renderCommandCenter();
    selectScenario("engineering-waiting-approval");
    openStructuredView();
    const recentActivity = screen.getByRole("list", { name: "Recent deterministic activity" });
    const qaEvent = within(recentActivity).getByRole("button", {
      name: "Inspect context for deterministic event QA advisory review complete",
    });

    fireEvent.click(qaEvent);

    expect(qaEvent).toHaveAttribute("aria-pressed", "true");
    expect(
      within(screen.getByRole("complementary", { name: "Contextual inspector" })).getByRole(
        "heading",
        { name: "QA advisory review" },
      ),
    ).toBeInTheDocument();
  });

  it("resets an event selection to the next scenario's bounded orchestrator selection", () => {
    renderCommandCenter();
    selectScenario("research-knowledge-active");
    fireEvent.click(
      within(screen.getByTestId("test-shell-activity")).getByRole("button", {
        name: "Inspect deterministic event Knowledge fixture started",
      }),
    );
    expect(screen.getByTestId("test-shell-inspector")).toHaveTextContent(
      "Knowledge fixture started",
    );

    selectScenario("engineering-waiting-approval");

    expect(screen.getByTestId("test-shell-inspector")).toHaveTextContent("AgentOrchestrator");
    expect(
      screen.queryByRole("button", {
        name: "Inspect deterministic event Knowledge fixture started",
      }),
    ).toBeNull();
  });

  it("only exposes severity filters represented by the current fixture", () => {
    renderCommandCenter();
    selectScenario("research-knowledge-active");

    let severity = screen.getByLabelText<HTMLSelectElement>("Severity");
    expect(Array.from(severity.options).map((option) => option.value)).toEqual(["all", "info"]);

    selectScenario("engineering-waiting-approval");
    severity = screen.getByLabelText<HTMLSelectElement>("Severity");
    expect(Array.from(severity.options).map((option) => option.value)).toEqual([
      "all",
      "success",
      "warning",
    ]);
    expect(Array.from(severity.options).map((option) => option.value)).not.toContain("danger");
  });

  it("follows agent, relationship, and group paths without dropping attributed activity", () => {
    renderCommandCenter();
    selectScenario("engineering-waiting-approval");
    openStructuredView();

    const activity = screen.getByRole("list", { name: "Simulated command center activity" });
    const follow = screen.getByRole("button", { name: "Follow selected path" });

    fireEvent.change(screen.getByLabelText("Search simulated topology"), {
      target: { value: "Coding proposal retained" },
    });
    const searchedCodingAgent = screen
      .getAllByRole("button", { name: /Coding Agent/ })
      .find((button) => button.classList.contains("command-center-entity-button"));
    expect(searchedCodingAgent).toBeDefined();
    fireEvent.change(screen.getByLabelText("Search simulated topology"), {
      target: { value: "" },
    });

    const codingAgent = screen
      .getAllByRole("button", { name: /Coding Agent/ })
      .find((button) => button.classList.contains("command-center-entity-button"));
    if (codingAgent === undefined) throw new Error("Expected the structured Coding Agent row");
    fireEvent.click(codingAgent);
    expect(screen.getByRole("complementary", { name: "Contextual inspector" })).toHaveTextContent(
      "Coding proposal retained",
    );
    fireEvent.click(follow);
    expect(activity).toHaveTextContent("Coding proposal retained");

    fireEvent.click(follow);
    fireEvent.click(screen.getByRole("button", { name: /Assigned to Coding Agent/ }));
    fireEvent.click(follow);
    expect(activity).toHaveTextContent("Coding proposal retained");

    fireEvent.click(follow);
    fireEvent.click(screen.getByRole("button", { name: /Engineering.*Proposal preparation/ }));
    fireEvent.click(follow);
    expect(activity).toHaveTextContent("Coding proposal retained");
    expect(activity).toHaveTextContent("QA advisory review complete");
  });

  it("synchronizes relationship selection and exposes bounded provenance details", () => {
    renderCommandCenter();
    openStructuredView();

    const relationship = screen.getByRole("button", { name: /Owns the bounded root lifecycle/ });
    fireEvent.click(relationship);
    expect(relationship).toHaveAttribute("aria-pressed", "true");

    const inspector = screen.getByRole("complementary", { name: "Contextual inspector" });
    expect(inspector).toHaveTextContent("Source");
    expect(inspector).toHaveTextContent("AgentOrchestrator");
    expect(inspector).toHaveTextContent("Personal Assistant");

    const details = screen.getByText("View bounded detail");
    fireEvent.click(details);
    expect(screen.getByText("Fixture event ID")).toBeInTheDocument();
    expect(screen.getByText("presentation safe", { exact: false })).toBeInTheDocument();
  });

  it("renders all five inert group lanes and approved graph viewport controls", () => {
    const { container } = renderCommandCenter();

    expect(container.querySelectorAll(".command-center-domain-lane")).toHaveLength(5);
    for (const name of [
      "Zoom in",
      "Zoom out",
      "Fit view",
      "Reset view",
      "Center selected",
      "Center active",
      "Clear selection",
    ]) {
      expect(
        within(screen.getByRole("toolbar", { name: "Topology viewport" })).getByRole("button", {
          name,
        }),
      ).toHaveAttribute("title", name);
    }
    expect(screen.getByRole("status", { name: "Topology zoom level" })).toHaveTextContent("100%");
    expect(screen.getByLabelText("Show relationship legend")).toBeInTheDocument();
    expect(screen.getByTestId("test-shell-inspector")).toHaveTextContent("AgentOrchestrator");
  });

  it("defaults to the structured alternative at browser-only narrow width", () => {
    const previousMatchMedia = window.matchMedia;
    Object.defineProperty(window, "matchMedia", {
      configurable: true,
      value: vi.fn().mockReturnValue({
        addEventListener: vi.fn(),
        matches: true,
        media: "(max-width: 640px)",
        onchange: null,
        removeEventListener: vi.fn(),
      }),
    });

    try {
      renderCommandCenter();
      expect(screen.getByRole("heading", { name: "Relationship table" })).toBeInTheDocument();
      expect(
        screen.queryByRole("listbox", { name: /Simulated operational topology/ }),
      ).not.toBeInTheDocument();
    } finally {
      Object.defineProperty(window, "matchMedia", {
        configurable: true,
        value: previousMatchMedia,
      });
    }
  });
});
