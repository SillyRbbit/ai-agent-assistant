import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { App, CommandCenterLoadingPage, type AppServices } from "./App";
import { APP_ROUTES, NAVIGATION_ITEMS, type AssistantMenuRoute } from "./application/navigation";
import { MOCK_STREAM_INTERVAL_MS } from "./application/mockAssistantRun";
import {
  browserMockRunDriver,
  type MockRunDriver,
  type MockRunEvent,
  type MockRunEventListener,
} from "./application/mockRunDriver";
import { ApplicationWorkspacePanelsContext } from "./components/applicationWorkspacePanels";
import type { AppInfo } from "./infrastructure/tauri/app-info-client";
import type {
  AssistantMenuRouteListener,
  MenuRouteSource,
} from "./infrastructure/tauri/menu-route-client";
import type {
  ResearchKnowledgeDemoProjection,
  ResearchKnowledgeDemoProjectionLoader,
} from "./infrastructure/tauri/research-knowledge-demo-projection-client";

const CONNECTED_APP_INFO: AppInfo = {
  architecture: "aarch64",
  environment: "development",
  name: "Cortexa",
  secureCore: true,
  target: "macos",
  version: "0.1.0",
};

const RUST_DEMO_PROJECTION: ResearchKnowledgeDemoProjection = {
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

interface MenuRouteHarness {
  readonly emit: (route: AssistantMenuRoute) => void;
  readonly hasListener: () => boolean;
  readonly source: MenuRouteSource;
  readonly unlisten: ReturnType<typeof vi.fn>;
}

function createMenuRouteHarness(): MenuRouteHarness {
  let listener: AssistantMenuRouteListener | undefined;
  const unlisten = vi.fn();

  return {
    emit(route) {
      if (listener === undefined) {
        throw new Error("The menu route listener was not installed.");
      }

      listener(route);
    },
    hasListener() {
      return listener !== undefined;
    },
    source: {
      subscribe(nextListener) {
        listener = nextListener;
        return Promise.resolve(() => {
          listener = undefined;
          unlisten();
        });
      },
    },
    unlisten,
  };
}

function createServices(
  menuRouteSource: MenuRouteSource,
  appInfoLoader: () => Promise<AppInfo> = () => Promise.resolve(CONNECTED_APP_INFO),
  mockRunDriver: MockRunDriver = browserMockRunDriver,
  researchKnowledgeDemoProjectionLoader: ResearchKnowledgeDemoProjectionLoader = () =>
    Promise.reject(new Error("Projection unavailable in this test.")),
): AppServices {
  return {
    appInfoLoader,
    menuRouteSource,
    mockRunDriver,
    researchKnowledgeDemoProjectionLoader,
  };
}

interface MockRunDriverHarness {
  readonly cancel: ReturnType<typeof vi.fn>;
  readonly driver: MockRunDriver;
  readonly emit: (event: MockRunEvent) => void;
}

function createMockRunDriverHarness(): MockRunDriverHarness {
  let listener: MockRunEventListener | undefined;
  const cancel = vi.fn(() => {
    listener = undefined;
  });

  return {
    cancel,
    driver: {
      start(_script, nextListener) {
        listener = nextListener;
        return { cancel };
      },
    },
    emit(event) {
      if (listener === undefined) {
        throw new Error("The mock run listener was not installed.");
      }

      listener(event);
    },
  };
}

function openSidebarRoute(label: string): void {
  fireEvent.click(screen.getByRole("button", { name: label }));
}

function openConversation(title: string): void {
  fireEvent.click(screen.getByRole("button", { name: `Open conversation: ${title}` }));
}

function startNewConversation(): void {
  fireEvent.click(screen.getByRole("button", { name: "Start new conversation" }));
}

function submitMockRequest(request = "Prepare the board update"): void {
  fireEvent.change(screen.getByLabelText("Assistant request"), {
    target: { value: request },
  });

  const submitButton =
    screen.queryByRole("button", { name: "Send" }) ?? screen.getByRole("button", { name: "Stop" });
  fireEvent.click(submitButton);
}

function finishMockStream(): void {
  act(() => {
    vi.advanceTimersByTime(MOCK_STREAM_INTERVAL_MS * 4);
  });
}

function getShellRegions(container: Element): {
  readonly contentRegion: HTMLElement;
  readonly mainRegion: HTMLElement;
  readonly sidebarRegion: HTMLElement;
} {
  const mainRegion = container.querySelector('[data-scroll-region="application-main"]');
  const contentRegion = container.querySelector('[data-scroll-region="application-content"]');
  const sidebarRegion = container.querySelector('[data-scroll-region="application-sidebar"]');

  if (
    mainRegion === null ||
    contentRegion === null ||
    sidebarRegion === null ||
    !(mainRegion instanceof HTMLElement) ||
    !(contentRegion instanceof HTMLElement) ||
    !(sidebarRegion instanceof HTMLElement)
  ) {
    throw new Error("Expected all root scrolling containers to be present.");
  }

  return { contentRegion, mainRegion, sidebarRegion };
}

afterEach(() => {
  vi.useRealTimers();
});

describe("App", () => {
  it("keeps Command Center loading outlets truthful, named, and closable", () => {
    const inspector = document.createElement("aside");
    const inspectorHeaderTarget = document.createElement("div");
    const inspectorBodyTarget = document.createElement("div");
    inspector.setAttribute("aria-labelledby", "application-inspector-title");
    inspector.append(inspectorHeaderTarget, inspectorBodyTarget);

    const activity = document.createElement("section");
    const activityTitle = document.createElement("h2");
    const activitySummaryTarget = document.createElement("span");
    const activityBodyTarget = document.createElement("div");
    activityTitle.id = "application-activity-title";
    activityTitle.textContent = "Activity";
    activity.setAttribute("aria-labelledby", activityTitle.id);
    activity.append(activityTitle, activitySummaryTarget, activityBodyTarget);
    document.body.append(inspector, activity);

    const closeInspector = vi.fn();
    try {
      const view = render(
        <ApplicationWorkspacePanelsContext.Provider
          value={{
            activityBodyTarget,
            activitySummaryTarget,
            closeInspector,
            inspectorBodyTarget,
            inspectorHeaderTarget,
            openInspector: () => undefined,
          }}
        >
          <CommandCenterLoadingPage />
        </ApplicationWorkspacePanelsContext.Provider>,
      );

      expect(
        within(inspector).getByRole("heading", { name: "Preparing Command Center" }),
      ).toHaveAttribute("id", "application-inspector-title");
      expect(inspector).toHaveTextContent("No Command Center selection available yet");
      expect(inspector).toHaveTextContent("No live runtime data is available here");
      expect(activity).toHaveTextContent("Deterministic fixture activity loading");
      expect(activity).toHaveTextContent("No Command Center fixture activity available yet");
      expect(activity).toHaveTextContent("not live telemetry");

      fireEvent.click(within(inspector).getByRole("button", { name: "Close workspace inspector" }));
      expect(closeInspector).toHaveBeenCalledOnce();
      view.unmount();
    } finally {
      inspector.remove();
      activity.remove();
    }
  });

  it("renders the conversation workspace and requires a non-empty request", () => {
    const harness = createMenuRouteHarness();
    const { container } = render(<App services={createServices(harness.source)} />);

    expect(screen.getByText("Cortexa")).toBeInTheDocument();
    const brandMark = container.querySelector(".application-brand__mark");
    expect(brandMark).not.toBeNull();
    expect(brandMark).toHaveTextContent("");
    expect(
      brandMark?.querySelector('source[media="(prefers-color-scheme: dark)"]'),
    ).toHaveAttribute("srcset", expect.stringContaining("logo-dark"));
    expect(brandMark?.querySelector("img")).toHaveAttribute(
      "src",
      expect.stringContaining("logo-light"),
    );
    expect(screen.getByRole("heading", { level: 1, name: "Conversations" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "No messages yet" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Start new conversation" })).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Open conversation: New conversation" }),
    ).toHaveAttribute("aria-current", "page");
    expect(screen.getByLabelText("Assistant request")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Send" })).toBeDisabled();
  });

  it("separates the native fixed sample from the editable mock conversation", () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    fireEvent.change(screen.getByLabelText("Assistant request"), {
      target: { value: "private draft" },
    });
    fireEvent.change(screen.getByLabelText("Conversation mode"), { target: { value: "native" } });
    expect(screen.queryByLabelText("Assistant request")).not.toBeInTheDocument();
    expect(screen.queryByText("private draft")).not.toBeInTheDocument();
    expect(
      screen.getByText(/Browser-only mode: native OpenAI requests are unavailable/),
    ).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Start native sample" })).not.toBeInTheDocument();
    fireEvent.change(screen.getByLabelText("Conversation mode"), { target: { value: "mock" } });
    expect(screen.getByLabelText("Assistant request")).toHaveValue("private draft");
  });

  it("sends with Return while preserving multiline and composition input", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    const runHarness = createMockRunDriverHarness();
    const start = vi.spyOn(runHarness.driver, "start");
    render(
      <App
        services={createServices(
          harness.source,
          () => Promise.resolve(CONNECTED_APP_INFO),
          runHarness.driver,
        )}
      />,
    );
    const composer = screen.getByLabelText("Assistant request");

    expect(fireEvent.keyDown(composer, { code: "Enter", key: "Enter" })).toBe(false);
    expect(start).not.toHaveBeenCalled();
    expect(screen.queryByRole("button", { name: "Stop" })).not.toBeInTheDocument();

    fireEvent.change(composer, { target: { value: "First line" } });
    expect(fireEvent.keyDown(composer, { code: "Enter", key: "Enter", shiftKey: true })).toBe(true);
    expect(screen.queryByRole("button", { name: "Stop" })).not.toBeInTheDocument();
    expect(composer).toHaveValue("First line");

    expect(fireEvent.keyDown(composer, { code: "Enter", isComposing: true, key: "Enter" })).toBe(
      true,
    );
    expect(fireEvent.keyDown(composer, { code: "Enter", key: "Enter", keyCode: 229 })).toBe(true);
    for (const modifier of [{ altKey: true }, { ctrlKey: true }, { metaKey: true }]) {
      expect(fireEvent.keyDown(composer, { code: "Enter", key: "Enter", ...modifier })).toBe(true);
    }
    expect(screen.queryByRole("button", { name: "Stop" })).not.toBeInTheDocument();
    expect(composer).toHaveValue("First line");

    expect(fireEvent.keyDown(composer, { code: "Enter", key: "Enter" })).toBe(false);
    fireEvent.keyDown(composer, { code: "Enter", key: "Enter" });

    const transcript = screen.getByRole("region", {
      name: "Conversation transcript: First line",
    });
    expect(start).toHaveBeenCalledOnce();
    expect(within(transcript).getAllByText("First line")).toHaveLength(1);
    expect(screen.getByRole("button", { name: "Stop" })).toBeInTheDocument();
  });

  it("renders an empty current-session Activity view", () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    openSidebarRoute("Activity");

    expect(screen.getByRole("heading", { name: "No session activity" })).toBeInTheDocument();
    expect(screen.queryByRole("list", { name: "Run activity" })).not.toBeInTheDocument();
  });

  it("streams deterministic assistant text and presents mock tool approval", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    submitMockRequest();

    const transcript = screen.getByRole("region", {
      name: "Conversation transcript: Prepare the board update",
    });
    expect(within(transcript).getByText("Prepare the board update")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Stop" })).toBeInTheDocument();
    finishMockStream();

    expect(screen.getByRole("dialog", { name: "Create a mock local task" })).toBeInTheDocument();
    expect(screen.getByRole("article", { name: "Mock tool activity" })).toHaveTextContent(
      "create_local_task",
    );
    expect(screen.getByText("No execution")).toBeInTheDocument();
  });

  it("discloses fixed mock context without copying request content", () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    submitMockRequest("Confidential board request");

    const provenance = screen.getByRole("article", { name: "Mock context used" });
    expect(within(provenance).getByText("mock-run-1")).toBeInTheDocument();
    expect(within(provenance).getByText("Current request")).toBeInTheDocument();
    expect(within(provenance).getByText("Used")).toBeInTheDocument();
    expect(within(provenance).getAllByText("Not used")).toHaveLength(4);
    expect(within(provenance).getByText("Earlier conversation messages")).toBeInTheDocument();
    expect(within(provenance).getByText("Saved memory")).toBeInTheDocument();
    expect(within(provenance).getByText("Device data")).toBeInTheDocument();
    expect(within(provenance).getByText("External services")).toBeInTheDocument();
    expect(provenance).not.toHaveTextContent("Confidential board request");
    expect(provenance).toHaveTextContent("Frontend mock only. Not trusted audit evidence.");
  });

  it("stops streaming and cancels the remaining mock run events", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    submitMockRequest();
    act(() => {
      vi.advanceTimersByTime(MOCK_STREAM_INTERVAL_MS);
    });
    fireEvent.click(screen.getByRole("button", { name: "Stop" }));
    act(() => {
      vi.runAllTimers();
    });

    expect(screen.getByText("Stopped")).toBeInTheDocument();
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(screen.queryByRole("article", { name: "Mock tool activity" })).not.toBeInTheDocument();
    expect(screen.queryByRole("article", { name: "Mock tool result" })).not.toBeInTheDocument();
    expect(screen.queryByRole("article", { name: "Mock final answer" })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Send" })).toBeDisabled();

    openSidebarRoute("Activity");
    const main = screen.getByRole("main");
    expect(within(main).getByText("Mock run stopped")).toBeInTheDocument();
  });

  it("shows a bounded failure and retries without duplicating the user message", () => {
    const menuHarness = createMenuRouteHarness();
    const runHarness = createMockRunDriverHarness();
    render(
      <App
        services={createServices(
          menuHarness.source,
          () => Promise.resolve(CONNECTED_APP_INFO),
          runHarness.driver,
        )}
      />,
    );
    submitMockRequest("Sensitive board request");

    act(() => {
      runHarness.emit({ reason: "mock-provider-unavailable", type: "failed" });
    });

    expect(
      screen.getByText("The local mock run could not finish. No action was executed."),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Retry" })).toBeInTheDocument();
    expect(runHarness.cancel).toHaveBeenCalledOnce();

    fireEvent.click(screen.getByRole("button", { name: "Retry" }));

    const transcript = screen.getByRole("region", {
      name: "Conversation transcript: Sensitive board request",
    });
    expect(within(transcript).getAllByText("Sensitive board request")).toHaveLength(1);
    const provenance = within(transcript).getAllByRole("article", {
      name: "Mock context used",
    });
    expect(provenance).toHaveLength(2);
    expect(provenance[0]).toHaveTextContent("mock-run-1");
    expect(provenance[1]).toHaveTextContent("mock-run-2");
    expect(screen.getByRole("button", { name: "Stop" })).toBeInTheDocument();

    act(() => {
      runHarness.emit({ reason: "mock-provider-unavailable", type: "failed" });
    });

    expect(
      screen.getAllByText("The local mock run could not finish. No action was executed."),
    ).toHaveLength(2);
    expect(screen.queryByRole("button", { name: "Retry" })).not.toBeInTheDocument();
    expect(screen.queryByRole("article", { name: "Mock final answer" })).not.toBeInTheDocument();
  });

  it("bounds a driver startup exception without exposing its detail", () => {
    const menuHarness = createMenuRouteHarness();
    const driver: MockRunDriver = {
      start() {
        throw new Error("sensitive provider detail");
      },
    };
    render(
      <App
        services={createServices(
          menuHarness.source,
          () => Promise.resolve(CONNECTED_APP_INFO),
          driver,
        )}
      />,
    );

    submitMockRequest();

    expect(
      screen.getByText("The local mock run could not finish. No action was executed."),
    ).toBeInTheDocument();
    expect(screen.queryByText("sensitive provider detail")).not.toBeInTheDocument();
  });

  it.each([
    ["Approve mock", "Mock approved", null],
    ["Reject", "Rejected", "Mock action rejected"],
  ] as const)("records the %s decision without executing a tool", (button, status, outcome) => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    submitMockRequest();
    finishMockStream();

    fireEvent.click(screen.getByRole("button", { name: button }));

    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(screen.getByText(status)).toBeInTheDocument();
    if (button === "Approve mock") {
      expect(screen.getByRole("article", { name: "Mock tool result" })).toBeInTheDocument();
      expect(screen.getByRole("article", { name: "Mock final answer" })).toBeInTheDocument();
      expect(screen.queryByText(/Mock approval recorded/)).not.toBeInTheDocument();
    } else {
      expect(screen.getAllByText(new RegExp(outcome))).not.toHaveLength(0);
      expect(screen.queryByRole("article", { name: "Mock tool result" })).not.toBeInTheDocument();
      expect(screen.queryByRole("article", { name: "Mock final answer" })).not.toBeInTheDocument();
    }
  });

  it("renders a fixed simulated result immediately before its fixed final answer", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    submitMockRequest("Confidential board request");
    finishMockStream();

    fireEvent.click(screen.getByRole("button", { name: "Approve mock" }));

    const result = screen.getByRole("article", { name: "Mock tool result" });
    expect(within(result).getByText("mock-run-1")).toBeInTheDocument();
    expect(within(result).getByText("create_local_task")).toBeInTheDocument();
    expect(within(result).getByText("Simulated")).toBeInTheDocument();
    expect(within(result).getByText("No execution")).toBeInTheDocument();
    expect(result).toHaveTextContent("No local task was created and no data changed.");
    expect(result).toHaveTextContent("Frontend mock only. Not verified executor output.");
    expect(result).not.toHaveTextContent("Confidential board request");

    const finalAnswer = screen.getByRole("article", { name: "Mock final answer" });
    expect(finalAnswer).toHaveTextContent("Final answer");
    expect(finalAnswer).toHaveTextContent(
      "Mock run complete. The approved task action was simulated only; no local task was created and no data changed.",
    );
    expect(finalAnswer).toHaveTextContent("Deterministic frontend mock, turn 2, mock-run-1");
    expect(finalAnswer).not.toHaveTextContent("Confidential board request");
    expect(result.nextElementSibling).toBe(finalAnswer);
  });

  it("returns an edited mock action to the composer without execution", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    submitMockRequest();
    finishMockStream();

    fireEvent.click(screen.getByRole("button", { name: "Edit" }));

    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(screen.getByText("Edit requested")).toBeInTheDocument();
    expect(screen.queryByRole("article", { name: "Mock tool result" })).not.toBeInTheDocument();
    expect(screen.queryByRole("article", { name: "Mock final answer" })).not.toBeInTheDocument();
    expect(screen.getByLabelText("Assistant request")).toHaveValue(
      "Revise the local task for: Prepare the board update",
    );
    expect(screen.getByText(/Nothing was executed/)).toBeInTheDocument();
  });

  it("renders redacted current-session activity without request content", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    submitMockRequest("Confidential board request");
    finishMockStream();
    fireEvent.click(screen.getByRole("button", { name: "Reject" }));

    openSidebarRoute("Activity");

    const activity = screen.getByRole("list", { name: "Run activity" });
    expect(within(activity).getByText("Mock run started")).toBeInTheDocument();
    expect(within(activity).getByText("Mock approval requested")).toBeInTheDocument();
    expect(within(activity).getByText("Mock action rejected")).toBeInTheDocument();
    expect(within(activity).queryByText("Confidential board request")).not.toBeInTheDocument();
  });

  it("lists conversations newest-first and restores their messages and tool activity", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    submitMockRequest("First board request");
    finishMockStream();
    fireEvent.click(screen.getByRole("button", { name: "Reject" }));
    startNewConversation();
    submitMockRequest("Second board request");
    finishMockStream();
    fireEvent.click(screen.getByRole("button", { name: "Approve mock" }));

    const history = screen.getByRole("list", { name: "Conversation history" });
    const conversationButtons = within(history).getAllByRole("button");
    expect(conversationButtons).toHaveLength(2);
    expect(conversationButtons[0]).toHaveAccessibleName("Open conversation: Second board request");
    expect(conversationButtons[1]).toHaveAccessibleName("Open conversation: First board request");

    openConversation("First board request");
    const firstTranscript = screen.getByRole("region", {
      name: "Conversation transcript: First board request",
    });
    expect(within(firstTranscript).getByText("First board request")).toBeInTheDocument();
    expect(within(firstTranscript).getByText("Rejected")).toBeInTheDocument();
    expect(
      within(firstTranscript).getByRole("article", { name: "Mock context used" }),
    ).toHaveTextContent("mock-run-1");
    expect(
      within(firstTranscript).queryByRole("article", { name: "Mock tool result" }),
    ).not.toBeInTheDocument();
    expect(
      within(firstTranscript).queryByRole("article", { name: "Mock final answer" }),
    ).not.toBeInTheDocument();
    expect(within(firstTranscript).queryByText("Second board request")).not.toBeInTheDocument();

    openConversation("Second board request");
    const secondTranscript = screen.getByRole("region", {
      name: "Conversation transcript: Second board request",
    });
    expect(within(secondTranscript).getByText("Second board request")).toBeInTheDocument();
    expect(within(secondTranscript).getByText("Mock approved")).toBeInTheDocument();
    expect(
      within(secondTranscript).getByRole("article", { name: "Mock context used" }),
    ).toHaveTextContent("mock-run-2");
    expect(
      within(secondTranscript).getByRole("article", { name: "Mock tool result" }),
    ).toHaveTextContent("mock-run-2");
    expect(
      within(secondTranscript).getByRole("article", { name: "Mock final answer" }),
    ).toHaveTextContent("Deterministic frontend mock, turn 2, mock-run-2");
    expect(within(secondTranscript).queryByText("First board request")).not.toBeInTheDocument();
  });

  it("reuses an existing empty conversation instead of adding another", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    submitMockRequest("Completed request");
    finishMockStream();
    fireEvent.click(screen.getByRole("button", { name: "Reject" }));
    startNewConversation();
    openConversation("Completed request");
    startNewConversation();

    const history = screen.getByRole("list", { name: "Conversation history" });
    expect(within(history).getAllByRole("button")).toHaveLength(2);
    expect(
      within(history).getByRole("button", { name: "Open conversation: New conversation" }),
    ).toHaveAttribute("aria-current", "page");
    expect(screen.getByRole("heading", { name: "No messages yet" })).toBeInTheDocument();
  });

  it("disables conversation creation and selection during streaming and approval", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    submitMockRequest();

    const newConversation = screen.getByRole("button", { name: "Start new conversation" });
    const currentConversation = screen.getByRole("button", {
      name: "Open conversation: Prepare the board update",
    });
    expect(newConversation).toBeDisabled();
    expect(currentConversation).toBeDisabled();

    finishMockStream();
    expect(newConversation).toBeDisabled();
    expect(currentConversation).toBeDisabled();

    fireEvent.click(screen.getByRole("button", { name: "Reject" }));
    expect(newConversation).toBeEnabled();
    expect(currentConversation).toBeEnabled();
  });

  it("opens all eight page shells in the shared scroll region and moves route focus", async () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const { contentRegion, mainRegion } = getShellRegions(view.container);
    const activeConversation = screen.getByRole("button", {
      name: "Open conversation: New conversation",
    });

    expect(APP_ROUTES).toHaveLength(8);

    for (const route of APP_ROUTES) {
      const item = NAVIGATION_ITEMS.find((candidate) => candidate.route === route);
      if (item === undefined) {
        throw new Error(`Navigation metadata is missing for ${route}.`);
      }

      openSidebarRoute(item.label);
      const expectedHeading = route === "permissions" ? "Permissions" : item.label;
      const heading = await screen.findByRole("heading", { level: 1, name: expectedHeading });
      expect(contentRegion).toContainElement(heading);
      expect(mainRegion).toHaveFocus();
      expect(screen.getByRole("button", { name: item.label })).toHaveAttribute(
        "aria-current",
        "page",
      );
      const routeAnnouncement = screen.getByText(item.label, {
        selector: ".application-toolbar__location",
      });
      expect(routeAnnouncement).toHaveAttribute("aria-live", "polite");
      expect(routeAnnouncement).toHaveAttribute("aria-atomic", "true");
      if (route === "conversations") {
        expect(activeConversation).toHaveAttribute("aria-current", "page");
      } else {
        expect(activeConversation).not.toHaveAttribute("aria-current");
      }
    }
  });

  it("routes all pages into one primary content scroll container", () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const { contentRegion, mainRegion, sidebarRegion } = getShellRegions(view.container);

    expect(mainRegion).toContainElement(contentRegion);
    expect(contentRegion).toHaveClass("application-content");
    expect(sidebarRegion).toHaveAttribute("data-scroll-region", "application-sidebar");
    expect(
      sidebarRegion.querySelector('[data-scroll-region="conversation-list-scroll"]'),
    ).not.toBeNull();
    expect(
      sidebarRegion.querySelector('[data-scroll-region="primary-navigation-scroll"]'),
    ).not.toBeNull();
    expect(contentRegion.querySelector("section.page-stack")).toBeInTheDocument();

    openSidebarRoute("Settings");
    expect(contentRegion.querySelector("section.page-stack")).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Settings" })).toBeInTheDocument();

    const settingsSection = contentRegion.querySelector("section.page-stack");
    expect(settingsSection).not.toBeNull();
    expect(contentRegion.contains(settingsSection as HTMLElement)).toBe(true);
  });

  it("resets only the route-content scroll owner when navigation changes routes", async () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const { contentRegion, mainRegion } = getShellRegions(view.container);

    contentRegion.scrollTop = 160;
    const transcript = screen.getByRole("region", {
      name: "Conversation transcript: New conversation",
    });
    transcript.scrollTop = 28;

    openSidebarRoute("Command Center");
    await screen.findByRole("heading", { level: 1, name: "Command Center" });

    expect(contentRegion.scrollTop).toBe(0);
    expect(transcript.scrollTop).toBe(28);
    expect(mainRegion).toHaveFocus();
  });

  it("exposes independently collapsible shell regions without changing route behavior", async () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const shell = view.container.querySelector(".application-shell");
    const sidebar = view.container.querySelector("#application-sidebar");
    const inspector = view.container.querySelector("#application-inspector");
    const activity = view.container.querySelector("#application-activity-dock");

    expect(shell).not.toBeNull();
    expect(sidebar).toHaveAttribute("data-expanded", "true");
    expect(inspector).toHaveAttribute("hidden");
    expect(activity).not.toHaveAttribute("hidden");

    const navigationToggle = screen.getByRole("button", { name: "Collapse navigation" });
    expect(navigationToggle).toHaveAttribute("aria-controls", "application-sidebar");
    expect(navigationToggle).toHaveAttribute("aria-expanded", "true");
    fireEvent.click(navigationToggle);

    const navigationExpand = screen.getByRole("button", { name: "Expand navigation" });
    expect(navigationExpand).toHaveAttribute("aria-expanded", "false");
    expect(sidebar).toHaveAttribute("data-expanded", "false");
    expect(shell).toHaveClass("application-shell--navigation-collapsed");
    expect(screen.queryByRole("list", { name: "Conversation history" })).toBeNull();
    expect(
      screen.queryByRole("button", { name: "Open conversation: New conversation" }),
    ).toBeNull();
    expect(view.container.querySelector(".application-brand__mark")).toBeNull();
    expect(screen.getByRole("img", { name: "Cortexa" })).toHaveAttribute(
      "src",
      expect.stringContaining("favicon"),
    );
    for (const item of NAVIGATION_ITEMS) {
      const routeButton = screen.getByRole("button", { name: item.label });
      const tooltipId = routeButton.getAttribute("aria-describedby");
      expect(tooltipId).toBe(`navigation-tooltip-${item.route}`);
      expect(view.container.querySelector(`#${tooltipId ?? "missing"}`)).toHaveAttribute(
        "role",
        "tooltip",
      );
    }

    const inspectorToggle = screen.getByRole("button", { name: "Show workspace inspector" });
    expect(inspectorToggle).toHaveAttribute("aria-controls", "application-inspector");
    expect(inspectorToggle).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(inspectorToggle);
    expect(inspector).not.toHaveAttribute("hidden");
    expect(shell).toHaveClass("application-shell--inspector-expanded");

    fireEvent.click(screen.getByRole("button", { name: "Close workspace inspector" }));
    expect(inspector).toHaveAttribute("hidden");
    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Show workspace inspector" })).toHaveFocus();
    });

    openSidebarRoute("Settings");
    expect(await screen.findByRole("heading", { level: 1, name: "Settings" })).toBeInTheDocument();
    expect(view.container.querySelector("#main-content")).toHaveFocus();
    expect(sidebar).toHaveAttribute("data-expanded", "false");
  });

  it("resizes the activity dock with keyboard and pointer input within its bounds", () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const shell = view.container.querySelector(".application-shell");

    const activityToggle = screen.getByRole("button", { name: "Show activity panel" });
    expect(activityToggle).toHaveAttribute("aria-controls", "application-activity-dock");
    expect(activityToggle).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(activityToggle);

    const separator = screen.getByRole("separator", { name: "Resize activity panel" });
    const maximumHeight = Number(separator.getAttribute("aria-valuemax"));
    expect(separator).toHaveAttribute("aria-orientation", "horizontal");
    expect(separator).toHaveAttribute("aria-valuemin", "144");
    expect(separator).toHaveAttribute("aria-valuenow", "216");
    expect(shell).toHaveClass("application-shell--activity-expanded");
    expect(shell).toHaveStyle("--application-activity-height: 216px");
    expect(screen.getByRole("button", { name: "Collapse activity panel" })).toHaveAttribute(
      "aria-controls",
      "application-activity-content",
    );
    expect(view.container.querySelector("#application-activity-content")).not.toHaveAttribute(
      "hidden",
    );

    fireEvent.keyDown(separator, { key: "ArrowUp" });
    expect(separator).toHaveAttribute("aria-valuenow", "232");
    expect(shell).toHaveStyle("--application-activity-height: 232px");

    fireEvent.keyDown(separator, { key: "Home" });
    expect(separator).toHaveAttribute("aria-valuenow", "144");
    fireEvent.keyDown(separator, { key: "End" });
    expect(separator).toHaveAttribute("aria-valuenow", String(maximumHeight));

    fireEvent.pointerDown(separator, { button: 0, clientY: 300, pointerId: 7 });
    fireEvent.pointerMove(window, { clientY: 400, pointerId: 7 });
    const pointerHeight = Math.max(144, maximumHeight - 100);
    expect(separator).toHaveAttribute("aria-valuenow", String(pointerHeight));
    fireEvent.pointerUp(window, { clientY: 400, pointerId: 7 });
    fireEvent.pointerMove(window, { clientY: 100, pointerId: 7 });
    expect(separator).toHaveAttribute("aria-valuenow", String(pointerHeight));

    fireEvent.click(screen.getByRole("button", { name: "Collapse activity panel" }));
    expect(screen.queryByRole("separator", { name: "Resize activity panel" })).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Show activity panel" }));
    expect(screen.getByRole("separator", { name: "Resize activity panel" })).toHaveAttribute(
      "aria-valuenow",
      String(pointerHeight),
    );
  });

  it("opens the Command Center shell inspector from a typed selection without stealing focus", async () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    openSidebarRoute("Command Center");
    await screen.findByLabelText("Deterministic scenario");
    const contextSummary = screen.getByText("Operational context").closest("summary");
    if (contextSummary === null) throw new Error("Expected Operational context summary");
    fireEvent.click(contextSummary);
    const roster = screen.getByRole("list", { name: "Canonical agent roster" });
    const researchAgent = within(roster).getByRole("button", {
      name: /Inspect Research Agent, canonical agent definition/,
    });
    researchAgent.focus();

    fireEvent.click(researchAgent);

    const inspector = view.container.querySelector("#application-inspector");
    expect(inspector).not.toHaveAttribute("hidden");
    expect(view.container.querySelector(".application-shell")).toHaveClass(
      "application-shell--inspector-expanded",
    );
    expect(
      within(inspector as HTMLElement).getByRole("heading", { name: "Research Agent" }),
    ).toBeInTheDocument();
    expect(inspector).toHaveTextContent("Canonical agent definition");
    expect(inspector).toHaveTextContent("DEMO MODE · SIMULATED AGENT DATA");
    expect(researchAgent).toHaveFocus();
    expect(view.container.querySelector("#main-content .command-center-inspector")).toBeNull();

    fireEvent.keyDown(window, { key: "Escape" });

    expect(inspector).toHaveAttribute("hidden");
    await waitFor(() => {
      expect(researchAgent).toHaveFocus();
    });
  });

  it("leaves inspector state and dialog focus untouched when Escape belongs to an aria-modal approval", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    fireEvent.click(screen.getByRole("button", { name: "Show workspace inspector" }));
    submitMockRequest();
    finishMockStream();
    const approval = screen.getByRole("dialog", { name: "Create a mock local task" });
    const approve = within(approval).getByRole("button", { name: "Approve mock" });
    approve.focus();

    fireEvent.keyDown(window, { key: "Escape" });

    expect(view.container.querySelector("#application-inspector")).not.toHaveAttribute("hidden");
    expect(approval).toBeInTheDocument();
    expect(approve).toHaveFocus();
  });

  it("selects Command Center fixture activity in the shared shell and clears it coherently", async () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    openSidebarRoute("Command Center");
    fireEvent.change(await screen.findByLabelText("Deterministic scenario"), {
      target: { value: "research-knowledge-active" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Show activity panel" }));
    const dock = view.container.querySelector("#application-activity-dock");
    if (!(dock instanceof HTMLElement)) throw new Error("Expected the shared activity dock");
    const eventButton = within(dock).getByRole("button", {
      name: "Inspect deterministic event Research fixture started",
    });
    eventButton.focus();

    fireEvent.click(eventButton);

    const inspector = view.container.querySelector("#application-inspector");
    if (!(inspector instanceof HTMLElement)) throw new Error("Expected the shared inspector");
    expect(inspector).not.toHaveAttribute("hidden");
    expect(
      within(inspector).getByRole("heading", { name: "Research fixture started" }),
    ).toBeInTheDocument();
    expect(inspector).toHaveTextContent("Fixture activity event");
    expect(inspector).toHaveTextContent("StatusUnavailable in fixture data");
    expect(inspector).toHaveTextContent(
      "The Research presentation lane entered its running state.",
    );
    expect(dock).toHaveTextContent("SourceUnavailable in fixture data");
    expect(dock).toHaveTextContent("Actiontask started");
    expect(dock).toHaveTextContent("TargetUnavailable in fixture data");
    expect(dock).toHaveTextContent("StatusUnavailable in fixture data");
    expect(dock).toHaveTextContent("Associated agentResearch Agent");
    expect(dock).toHaveTextContent("TaskResearch findings");
    expect(dock).toHaveTextContent("WorkflowBounded parallel analysis");
    expect(dock).toHaveTextContent("SIMULATED TIME");
    expect(eventButton.querySelector("div, p, dl")).toBeNull();
    expect(eventButton).toHaveFocus();
    expect(view.container.querySelector("#main-content .command-center-activity")).toBeNull();
    const inspectorContent = inspector.querySelector(
      '[data-scroll-owner="application-inspector-content"]',
    );
    expect(inspectorContent?.querySelector("[data-scroll-owner]")).toBeNull();
    expect(
      dock
        .querySelector('[data-scroll-owner="application-activity-content"]')
        ?.querySelector("[data-scroll-owner]"),
    ).toBeNull();

    const toolbar = screen.getByRole("toolbar", { name: "Topology viewport" });
    const clearSelection = within(toolbar).getByRole("button", { name: "Clear selection" });
    expect(clearSelection).toBeEnabled();
    fireEvent.click(clearSelection);
    expect(
      within(inspector).getByRole("heading", { name: "Nothing selected" }),
    ).toBeInTheDocument();
    expect(clearSelection).toBeDisabled();
  });

  it("unmounts Command Center panel content on route exit and restores truthful fallbacks", async () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    openSidebarRoute("Command Center");
    await screen.findByRole("heading", { level: 1, name: "Command Center" });
    fireEvent.click(screen.getByRole("button", { name: "Show workspace inspector" }));
    fireEvent.click(screen.getByRole("button", { name: "Show activity panel" }));
    expect(view.container.querySelector("#application-inspector")).toHaveTextContent(
      "AgentOrchestrator",
    );
    expect(view.container.querySelector("#application-activity-dock")).toHaveTextContent(
      "simulated fixture events",
    );

    openSidebarRoute("Settings");
    await screen.findByRole("heading", { level: 1, name: "Settings" });

    const inspector = view.container.querySelector("#application-inspector");
    const activity = view.container.querySelector("#application-activity-dock");
    expect(inspector).toHaveTextContent("Workspace inspector");
    expect(inspector).toHaveTextContent("No inspectable workspace item selected");
    expect(inspector).not.toHaveTextContent("AgentOrchestrator");
    expect(activity).toHaveTextContent("No current-session mock events");
    expect(activity).not.toHaveTextContent("simulated fixture events");
  });

  it("describes generic shell activity as incomplete current-session mock data", () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    submitMockRequest();
    fireEvent.click(screen.getByRole("button", { name: "Show activity panel" }));

    const activity = view.container.querySelector("#application-activity-dock");
    expect(activity).toHaveTextContent("current-session mock event");
    expect(activity).toHaveTextContent("TimeUnavailable");
    expect(activity).toHaveTextContent("SourceDeterministic frontend mock loop");
    expect(activity).toHaveTextContent("Actionrun started");
    expect(activity).toHaveTextContent("Runmock-run-1");
    expect(activity).toHaveTextContent("StatusUnavailable in current-session event data");
  });

  it("keeps regional scroll owners outside the route content owner", () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const { contentRegion, mainRegion, sidebarRegion } = getShellRegions(view.container);

    fireEvent.click(screen.getByRole("button", { name: "Show workspace inspector" }));
    fireEvent.click(screen.getByRole("button", { name: "Show activity panel" }));

    const inspectorOwner = view.container.querySelector(
      '[data-scroll-owner="application-inspector-content"]',
    );
    const activityOwner = view.container.querySelector(
      '[data-scroll-owner="application-activity-content"]',
    );

    expect(contentRegion).toHaveAttribute("data-scroll-owner", "route-content");
    expect(mainRegion).toContainElement(contentRegion);
    expect(mainRegion).not.toContainElement(inspectorOwner as HTMLElement);
    expect(mainRegion).not.toContainElement(activityOwner as HTMLElement);
    expect(sidebarRegion).not.toContainElement(contentRegion);
    expect(inspectorOwner).not.toContainElement(contentRegion);
    expect(activityOwner).not.toContainElement(contentRegion);
  });

  it("keeps the compact core status available to assistive technology", () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const statusLabel = view.container.querySelector(".application-toolbar__status-label");

    expect(statusLabel).not.toBeNull();
    expect(statusLabel).toHaveClass("visually-hidden-at-compact");
    expect(statusLabel).not.toHaveAttribute("hidden");
    expect(statusLabel).not.toHaveAttribute("aria-hidden");
    expect(statusLabel).toHaveTextContent(/Connecting|Local core ready/u);
  });

  it("keeps long conversation content in the primary scroll region", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    const runHarness = createMockRunDriverHarness();
    const view = render(
      <App
        services={createServices(
          harness.source,
          () => Promise.resolve(CONNECTED_APP_INFO),
          runHarness.driver,
        )}
      />,
    );
    const { contentRegion } = getShellRegions(view.container);

    submitMockRequest(
      "First long request with repeated detail to emulate tall content.".repeat(20),
    );
    finishMockStream();

    const transcripts = screen.getAllByRole("region", {
      name: /^Conversation transcript:/u,
    });
    expect(transcripts).toHaveLength(1);
    const conversationTranscript = transcripts[0];
    if (conversationTranscript === undefined) {
      throw new Error("Expected the active conversation transcript.");
    }
    expect(contentRegion.contains(conversationTranscript)).toBe(true);
    expect(
      within(conversationTranscript).getByText("Preparing mock response…"),
    ).toBeInTheDocument();
  });

  it("keeps long lists inside the sidebar overflow owner", () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    const { sidebarRegion } = getShellRegions(view.container);
    const conversationList = sidebarRegion.querySelector(
      '[data-scroll-region="conversation-list-scroll"]',
    );
    const primaryList = sidebarRegion.querySelector(
      '[data-scroll-region="primary-navigation-scroll"]',
    );

    expect(conversationList).not.toBeNull();
    expect(primaryList).not.toBeNull();

    // create many conversations to ensure the list is structurally long-capable
    for (let index = 0; index < 20; index += 1) {
      submitMockRequest(`Request ${String(index)}`);
      finishMockStream();
      fireEvent.click(screen.getByRole("button", { name: "Reject" }));
      if (index < 19) {
        startNewConversation();
      }
    }

    const conversationHistory = screen.getByRole("list", { name: "Conversation history" });
    expect(within(conversationHistory).getAllByRole("button")).toHaveLength(20);
    expect(conversationList?.closest("aside")).toBe(sidebarRegion);
    expect(primaryList?.closest("aside")).toBe(sidebarRegion);
  });

  it("routes a new-request menu event to Conversations and clears the draft", async () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    await waitFor(() => {
      expect(harness.hasListener()).toBe(true);
    });

    fireEvent.change(screen.getByLabelText("Assistant request"), {
      target: { value: "Draft request" },
    });
    openSidebarRoute("Settings");

    act(() => {
      harness.emit("new_request");
    });

    expect(screen.getByRole("heading", { level: 1, name: "Conversations" })).toBeInTheDocument();
    expect(screen.getByLabelText("Assistant request")).toHaveValue("");
  });

  it("creates a new idle conversation from the native new-request route", async () => {
    vi.useFakeTimers();
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    await act(async () => {
      await Promise.resolve();
    });
    expect(harness.hasListener()).toBe(true);

    submitMockRequest("Native route request");
    finishMockStream();
    fireEvent.click(screen.getByRole("button", { name: "Reject" }));
    openSidebarRoute("Settings");

    act(() => {
      harness.emit("new_request");
    });

    const history = screen.getByRole("list", { name: "Conversation history" });
    expect(within(history).getAllByRole("button")).toHaveLength(2);
    expect(
      within(history).getByRole("button", { name: "Open conversation: Native route request" }),
    ).toBeInTheDocument();
    expect(
      within(history).getByRole("button", { name: "Open conversation: New conversation" }),
    ).toHaveAttribute("aria-current", "page");
    expect(screen.getByRole("heading", { name: "No messages yet" })).toBeInTheDocument();
  });

  it("routes a tasks-placeholder menu event to Tasks", async () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    await waitFor(() => {
      expect(harness.hasListener()).toBe(true);
    });

    act(() => {
      harness.emit("tasks_placeholder");
    });

    expect(screen.getByRole("heading", { level: 1, name: "Tasks" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "No tasks yet" })).toBeInTheDocument();
  });

  it("wires the explicit Command Center projection refresh through application services", async () => {
    const harness = createMenuRouteHarness();
    const projectionLoader = vi.fn(() => Promise.resolve(RUST_DEMO_PROJECTION));
    render(
      <App
        services={createServices(
          harness.source,
          () => Promise.resolve(CONNECTED_APP_INFO),
          browserMockRunDriver,
          projectionLoader,
        )}
      />,
    );

    openSidebarRoute("Command Center");
    fireEvent.change(await screen.findByLabelText("Deterministic scenario"), {
      target: { value: "research-knowledge-active" },
    });
    expect(projectionLoader).not.toHaveBeenCalled();
    const contextSummary = screen.getByText("Operational context").closest("summary");
    if (contextSummary === null) throw new Error("Expected operational context disclosure");
    fireEvent.click(contextSummary);

    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));

    expect(await screen.findByText("research-knowledge-demo-projection-v1")).toBeVisible();
    expect(projectionLoader).toHaveBeenCalledOnce();
  });

  it("shows typed Rust diagnostics in Settings", async () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    openSidebarRoute("Settings");

    expect(await screen.findByText("Rust core connected")).toBeInTheDocument();
    expect(screen.getAllByText("Cortexa")).toHaveLength(2);
    expect(screen.getByText("get_app_info")).toBeInTheDocument();
    expect(screen.getByText("macos · aarch64")).toBeInTheDocument();
    expect(await screen.findByText("Listener active")).toBeInTheDocument();
  });

  it("shows loading and error presentations for typed IPC", async () => {
    const loadingHarness = createMenuRouteHarness();
    const loadingView = render(
      <App
        services={createServices(
          loadingHarness.source,
          () => new Promise<AppInfo>(() => undefined),
        )}
      />,
    );
    openSidebarRoute("Settings");
    expect(screen.getByRole("heading", { name: "Connecting to Rust core" })).toBeInTheDocument();
    loadingView.unmount();

    const errorHarness = createMenuRouteHarness();
    render(
      <App
        services={createServices(errorHarness.source, () =>
          Promise.reject(new Error("IPC unavailable")),
        )}
      />,
    );
    openSidebarRoute("Settings");
    expect(
      await screen.findByRole("heading", { name: "Rust core unavailable" }),
    ).toBeInTheDocument();
    expect(
      screen.getByText("The Rust core did not return application information."),
    ).toBeInTheDocument();
    expect(screen.queryByText("IPC unavailable")).not.toBeInTheDocument();
  });

  it("renders the Permission Center without a permission request control", () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    openSidebarRoute("Permissions");

    const permissionCenter = screen.getByRole("region", { name: "Permission status list" });
    expect(
      within(permissionCenter).getByRole("heading", {
        name: "No device permission has been requested",
      }),
    ).toBeInTheDocument();
    expect(within(permissionCenter).getByText("Screen Recording")).toBeInTheDocument();
    expect(within(permissionCenter).queryByRole("button")).not.toBeInTheDocument();
  });

  it("shows a bounded menu-listener error without exposing the thrown message", async () => {
    const source: MenuRouteSource = {
      subscribe() {
        return Promise.reject(new Error("sensitive native detail"));
      },
    };
    render(<App services={createServices(source)} />);
    openSidebarRoute("Settings");

    expect(await screen.findByText("Listener unavailable")).toBeInTheDocument();
    expect(screen.queryByText("sensitive native detail")).not.toBeInTheDocument();
  });

  it("removes the native menu listener when the shell unmounts", async () => {
    const harness = createMenuRouteHarness();
    const view = render(<App services={createServices(harness.source)} />);
    await waitFor(() => {
      expect(harness.hasListener()).toBe(true);
    });

    view.unmount();

    expect(harness.unlisten).toHaveBeenCalledOnce();
  });

  it("cancels the active mock driver when the shell unmounts", () => {
    const menuHarness = createMenuRouteHarness();
    const runHarness = createMockRunDriverHarness();
    const view = render(
      <App
        services={createServices(
          menuHarness.source,
          () => Promise.resolve(CONNECTED_APP_INFO),
          runHarness.driver,
        )}
      />,
    );
    submitMockRequest();

    view.unmount();

    expect(runHarness.cancel).toHaveBeenCalledOnce();
  });
});
