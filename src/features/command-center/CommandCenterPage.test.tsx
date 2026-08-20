import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import CommandCenterPage from "./CommandCenterPage";
import {
  COMMAND_CENTER_AGENT_IDS,
  COMMAND_CENTER_DISCLOSURE,
  COMMAND_CENTER_SCENARIO_IDS,
} from "./commandCenterProjection";

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

function openStructuredView(): void {
  fireEvent.click(screen.getByRole("button", { name: "Structured" }));
}

function selectScenario(label: string): void {
  fireEvent.change(screen.getByLabelText("Deterministic scenario"), {
    target: { value: label },
  });
}

describe("CommandCenterPage", () => {
  it("owns the full route width without changing the shared page-stack contract", () => {
    render(<CommandCenterPage />);

    expect(
      screen.getByRole("heading", { level: 1, name: "Command Center" }).closest("section"),
    ).toHaveClass("page-stack", "command-center-page");
  });

  it("renders the bounded catalog with persistent demo disclosure and no live claim", () => {
    render(<CommandCenterPage />);

    expect(screen.getByRole("heading", { level: 1, name: "Command Center" })).toBeInTheDocument();
    expect(screen.getAllByText(COMMAND_CENTER_DISCLOSURE, { exact: false })).toHaveLength(4);
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

  it("renders every exact agent name and complete accessible label in graph nodes", () => {
    const { container } = render(<CommandCenterPage />);
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
    render(<CommandCenterPage />);
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
    render(<CommandCenterPage />);
    openStructuredView();

    fireEvent.click(screen.getByRole("button", { name: /Research Agent/ }));
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

  it("filters locally, presents a no-result state, and resets without side effects", () => {
    render(<CommandCenterPage />);

    fireEvent.change(screen.getByLabelText("Search simulated topology"), {
      target: { value: "no-such-fixture-entity" },
    });
    expect(
      screen.getByRole("heading", { name: "No matching topology entities" }),
    ).toBeInTheDocument();

    const [resetButton] = screen.getAllByRole("button", { name: "Reset filters" });
    if (resetButton === undefined) {
      throw new Error("Expected a reset control for the no-result state");
    }
    fireEvent.click(resetButton);
    expect(
      screen.getByRole("listbox", { name: /Simulated operational topology/ }),
    ).toBeInTheDocument();
    expect(screen.getByLabelText("Search simulated topology")).toHaveValue("");
  });

  it("shows blocked and cancellation fixtures truthfully with bounded activity", () => {
    render(<CommandCenterPage />);

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
    expect(screen.getByText("Child cancellation propagated")).toBeInTheDocument();
    expect(screen.queryByText("Proposal-only workflow complete")).not.toBeInTheDocument();
  });

  it("uses one composite graph entry and lets ordinary wheel gestures pass through", () => {
    render(<CommandCenterPage />);
    const graph = screen.getByRole("listbox", { name: /Simulated operational topology/ });
    const initialActive = graph.getAttribute("aria-activedescendant");

    graph.focus();
    fireEvent.keyDown(graph, { key: "ArrowRight" });
    expect(graph.getAttribute("aria-activedescendant")).not.toBe(initialActive);
    fireEvent.keyDown(graph, { key: "Enter" });
    expect(
      screen.getByRole("complementary", { name: "Contextual inspector" }),
    ).not.toHaveTextContent("Select a simulated entity");

    const wheel = new WheelEvent("wheel", { bubbles: true, cancelable: true, deltaY: 80 });
    graph.dispatchEvent(wheel);
    expect(wheel.defaultPrevented).toBe(false);
  });

  it("covers queued, active, approval-wait, and completed fixture states", () => {
    render(<CommandCenterPage />);
    openStructuredView();

    selectScenario("research-queued");
    expect(screen.getByRole("button", { name: /Research analysis.*queued/ })).toBeInTheDocument();

    selectScenario("research-knowledge-active");
    expect(screen.getByRole("button", { name: /Research findings.*running/ })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Knowledge findings.*running/ })).toBeInTheDocument();

    selectScenario("engineering-waiting-approval");
    expect(
      screen.getByRole("button", { name: /Application approval checkpoint.*waiting approval/ }),
    ).toBeInTheDocument();

    selectScenario("workflow-completed");
    fireEvent.change(screen.getByLabelText("Severity"), { target: { value: "success" } });
    expect(screen.getByText("Proposal-only workflow complete")).toBeInTheDocument();
    expect(screen.getByRole("option", { name: "Success" })).toBeInTheDocument();
  });

  it("keeps activity options stable and synchronizes global, agent, and selected-path filters", () => {
    render(<CommandCenterPage />);
    selectScenario("research-knowledge-active");

    fireEvent.change(screen.getByLabelText("Search simulated topology"), {
      target: { value: "Knowledge fixture started" },
    });
    expect(screen.getByText("Knowledge fixture started")).toBeInTheDocument();
    expect(screen.queryByText("Research fixture started")).not.toBeInTheDocument();

    fireEvent.change(screen.getByLabelText("Search simulated topology"), { target: { value: "" } });
    fireEvent.change(screen.getByLabelText("Agent"), { target: { value: "research" } });
    const activity = screen.getByRole("list", { name: "Simulated command center activity" });
    expect(activity).toHaveTextContent("Research fixture started");
    expect(activity).not.toHaveTextContent("Knowledge fixture started");

    fireEvent.change(screen.getByLabelText("Agent"), { target: { value: "all" } });
    openStructuredView();
    fireEvent.click(screen.getByRole("button", { name: /Research findings.*running/ }));
    const beforeFollowCount = activity.querySelectorAll("li").length;
    fireEvent.click(screen.getByRole("button", { name: "Follow selected path" }));
    expect(activity.querySelectorAll("li").length).toBeGreaterThan(0);
    expect(activity.querySelectorAll("li").length).toBeLessThan(beforeFollowCount);

    fireEvent.change(screen.getByLabelText("Event kind"), { target: { value: "task-started" } });
    expect(screen.getByRole("option", { name: "task progress" })).toBeInTheDocument();
  });

  it("follows agent, relationship, and group paths without dropping attributed activity", () => {
    render(<CommandCenterPage />);
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
    render(<CommandCenterPage />);
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
    const { container } = render(<CommandCenterPage />);

    expect(container.querySelectorAll(".command-center-domain-lane")).toHaveLength(5);
    for (const name of [
      "Zoom in",
      "Zoom out",
      "Fit view",
      "Reset view",
      "Center selected",
      "Center active",
    ]) {
      expect(screen.getByRole("button", { name })).toBeInTheDocument();
    }
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
      render(<CommandCenterPage />);
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
