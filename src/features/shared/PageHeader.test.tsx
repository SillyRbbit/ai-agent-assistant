import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { PageHeader } from "./PageHeader";

describe("PageHeader context", () => {
  it("does not imply all activity stays on-device", () => {
    render(
      <PageHeader headingId="test-title" title="Workspace" description="Choose a workflow." />,
    );
    expect(screen.getByRole("heading", { level: 1, name: "Workspace" })).toHaveAttribute(
      "id",
      "test-title",
    );
    expect(screen.queryByText("Local only")).not.toBeInTheDocument();
  });
  it("shows only the explicit context supplied by the screen", () => {
    const { rerender } = render(
      <PageHeader
        headingId="test-title"
        title="Conversations"
        description="Your request."
        badge="Mock · no provider calls"
      />,
    );
    expect(screen.getByText("Mock · no provider calls")).toBeInTheDocument();
    rerender(
      <PageHeader
        headingId="test-title"
        title="Conversations"
        description="Your request."
        badge="Review provider disclosure"
      />,
    );
    expect(screen.queryByText("Mock · no provider calls")).not.toBeInTheDocument();
    expect(screen.getByText("Review provider disclosure")).toBeInTheDocument();
  });
});
