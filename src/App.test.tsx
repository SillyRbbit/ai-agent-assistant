import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { App, type AppServices } from "./App";
import { APP_ROUTES, NAVIGATION_ITEMS, type AssistantMenuRoute } from "./application/navigation";
import { MOCK_STREAM_INTERVAL_MS } from "./application/mockAssistantRun";
import {
  browserMockRunDriver,
  type MockRunDriver,
  type MockRunEvent,
  type MockRunEventListener,
} from "./application/mockRunDriver";
import type { AppInfo } from "./infrastructure/tauri/app-info-client";
import type {
  AssistantMenuRouteListener,
  MenuRouteSource,
} from "./infrastructure/tauri/menu-route-client";

const CONNECTED_APP_INFO: AppInfo = {
  architecture: "aarch64",
  environment: "development",
  name: "AI Agent Assistant",
  secureCore: true,
  target: "macos",
  version: "0.1.0",
};

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
): AppServices {
  return { appInfoLoader, menuRouteSource, mockRunDriver };
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
  fireEvent.click(screen.getByRole("button", { name: "Send" }));
}

function finishMockStream(): void {
  act(() => {
    vi.advanceTimersByTime(MOCK_STREAM_INTERVAL_MS * 4);
  });
}

afterEach(() => {
  vi.useRealTimers();
});

describe("App", () => {
  it("renders the conversation workspace and requires a non-empty request", () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    expect(screen.getByRole("heading", { level: 1, name: "Conversations" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "No messages yet" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Start new conversation" })).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Open conversation: New conversation" }),
    ).toHaveAttribute("aria-current", "page");
    expect(screen.getByLabelText("Assistant request")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Send" })).toBeDisabled();
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
    expect(screen.getByText("Mock run stopped")).toBeInTheDocument();
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
      expect(screen.getByText(new RegExp(outcome))).toBeInTheDocument();
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

  it("opens every page shell from the sidebar", () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    for (const route of APP_ROUTES) {
      const item = NAVIGATION_ITEMS.find((candidate) => candidate.route === route);
      if (item === undefined) {
        throw new Error(`Navigation metadata is missing for ${route}.`);
      }

      openSidebarRoute(item.label);
      const expectedHeading = route === "permissions" ? "Permissions" : item.label;
      expect(screen.getByRole("heading", { level: 1, name: expectedHeading })).toBeInTheDocument();
      expect(screen.getByRole("button", { name: item.label })).toHaveAttribute(
        "aria-current",
        "page",
      );
    }
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

  it("shows typed Rust diagnostics in Settings", async () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);
    openSidebarRoute("Settings");

    expect(await screen.findByText("Rust core connected")).toBeInTheDocument();
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
    expect(screen.getByText("IPC unavailable")).toBeInTheDocument();
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
