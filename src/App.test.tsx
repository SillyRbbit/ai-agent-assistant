import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { App, type AppServices } from "./App";
import { APP_ROUTES, NAVIGATION_ITEMS, type AssistantMenuRoute } from "./application/navigation";
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
): AppServices {
  return { appInfoLoader, menuRouteSource };
}

function openSidebarRoute(label: string): void {
  fireEvent.click(screen.getByRole("button", { name: label }));
}

describe("App", () => {
  it("renders the conversation workspace, empty state, and non-functional composer", () => {
    const harness = createMenuRouteHarness();
    render(<App services={createServices(harness.source)} />);

    expect(screen.getByRole("heading", { level: 1, name: "Conversations" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "No conversations yet" })).toBeInTheDocument();
    expect(screen.getByLabelText("Assistant request")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Send" })).toBeDisabled();
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
});
