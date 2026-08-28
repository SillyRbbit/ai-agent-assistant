import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { ResearchKnowledgeDemoProjection } from "../../../infrastructure/tauri/research-knowledge-demo-projection-client";
import { ResearchKnowledgeDemoProjectionPanel } from "./ResearchKnowledgeDemoProjectionPanel";

const projection: ResearchKnowledgeDemoProjection = {
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

describe("ResearchKnowledgeDemoProjectionPanel", () => {
  it("does not request native data until the explicit refresh action", async () => {
    const loader = vi.fn(() => Promise.resolve(projection));
    render(<ResearchKnowledgeDemoProjectionPanel loader={loader} />);

    expect(loader).not.toHaveBeenCalled();
    expect(screen.getByText("DEMO MODE · SIMULATED AGENT DATA", { exact: false })).toBeVisible();
    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));

    expect(
      await screen.findByText("Read-only Rust projection received. No workflow was started."),
    ).toBeVisible();
    expect(loader).toHaveBeenCalledOnce();
    expect(screen.getByText("research-knowledge-demo-projection-v1")).toBeVisible();
    expect(screen.getByText("Knowledge & Document Agent")).toBeVisible();
    expect(screen.getByText("cancelled")).toBeVisible();
  });

  it("prevents concurrent requests and shows a fixed loading state", async () => {
    let resolve: ((value: ResearchKnowledgeDemoProjection) => void) | undefined;
    const loader = vi.fn(
      () =>
        new Promise<ResearchKnowledgeDemoProjection>((next) => {
          resolve = next;
        }),
    );
    render(<ResearchKnowledgeDemoProjectionPanel loader={loader} />);

    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));
    const loadingButton = screen.getByRole("button", { name: "Refreshing Rust projection" });
    expect(loadingButton).toBeDisabled();
    expect(screen.getByRole("region", { name: "Read-only Rust demo projection" })).toHaveAttribute(
      "aria-busy",
      "true",
    );
    const status = screen.getByText("Requesting the read-only Rust projection.");
    expect(status).toHaveAttribute("aria-live", "polite");
    expect(status).toHaveAttribute("aria-atomic", "true");
    fireEvent.click(loadingButton);
    expect(loader).toHaveBeenCalledOnce();

    resolve?.(projection);
    await screen.findByText("Read-only Rust projection received. No workflow was started.");
  });

  it("closes errors without exposing upstream detail and allows an explicit retry", async () => {
    const loader = vi
      .fn<() => Promise<ResearchKnowledgeDemoProjection>>()
      .mockRejectedValueOnce(new Error("sensitive native detail"))
      .mockResolvedValueOnce(projection);
    render(<ResearchKnowledgeDemoProjectionPanel loader={loader} />);

    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));
    expect(
      await screen.findByText(
        "Rust demo projection unavailable. The deterministic frontend fixture remains available.",
      ),
    ).toBeVisible();
    expect(screen.queryByText("sensitive native detail")).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));
    expect(
      await screen.findByText("Read-only Rust projection received. No workflow was started."),
    ).toBeVisible();
    expect(loader).toHaveBeenCalledTimes(2);
  });

  it("clears a stale successful projection before a later request settles", async () => {
    let rejectSecond: ((reason?: unknown) => void) | undefined;
    const loader = vi
      .fn<() => Promise<ResearchKnowledgeDemoProjection>>()
      .mockResolvedValueOnce(projection)
      .mockImplementationOnce(
        () =>
          new Promise<ResearchKnowledgeDemoProjection>((_resolve, reject) => {
            rejectSecond = reject;
          }),
      );
    render(<ResearchKnowledgeDemoProjectionPanel loader={loader} />);

    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));
    expect(await screen.findByText("research-knowledge-demo-projection-v1")).toBeVisible();

    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));
    expect(screen.queryByText("research-knowledge-demo-projection-v1")).not.toBeInTheDocument();
    rejectSecond?.(new Error("sensitive native detail"));

    expect(
      await screen.findByText(
        "Rust demo projection unavailable. The deterministic frontend fixture remains available.",
      ),
    ).toBeVisible();
    expect(screen.queryByText("research-knowledge-demo-projection-v1")).not.toBeInTheDocument();
  });

  it("ignores a late reply after the panel unmounts", async () => {
    let resolve: ((value: ResearchKnowledgeDemoProjection) => void) | undefined;
    const loader = () =>
      new Promise<ResearchKnowledgeDemoProjection>((next) => {
        resolve = next;
      });
    const view = render(<ResearchKnowledgeDemoProjectionPanel loader={loader} />);
    fireEvent.click(screen.getByRole("button", { name: "Refresh Rust projection" }));

    view.unmount();
    resolve?.(projection);
    await waitFor(() => {
      expect(
        screen.queryByText("Read-only Rust projection received.", { exact: false }),
      ).not.toBeInTheDocument();
    });
  });
});
