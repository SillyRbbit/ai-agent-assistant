import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { App } from "./App";
import { fetchAppInfo } from "./infrastructure/tauri/app-info-client";

vi.mock("./infrastructure/tauri/app-info-client", () => ({
  fetchAppInfo: vi.fn(),
}));

const fetchAppInfoMock = vi.mocked(fetchAppInfo);

describe("App", () => {
  beforeEach(() => {
    fetchAppInfoMock.mockReset();
  });

  it("renders metadata returned by the Rust core", async () => {
    fetchAppInfoMock.mockResolvedValue({
      architecture: "aarch64",
      environment: "development",
      name: "AI Agent Assistant",
      secureCore: true,
      target: "macos",
      version: "0.1.0",
    });

    render(<App />);

    expect(
      screen.getByRole("heading", { level: 2, name: "Smallest runnable application" }),
    ).toBeInTheDocument();
    expect(await screen.findByText("Rust core connected")).toBeInTheDocument();
    expect(screen.getByText("macos · aarch64")).toBeInTheDocument();
  });

  it("shows a clear error state when IPC is unavailable", async () => {
    fetchAppInfoMock.mockRejectedValue(new Error("IPC unavailable"));

    render(<App />);

    expect(await screen.findByText("Rust core unavailable")).toBeInTheDocument();
    expect(screen.getByText("IPC unavailable")).toBeInTheDocument();
  });
});
