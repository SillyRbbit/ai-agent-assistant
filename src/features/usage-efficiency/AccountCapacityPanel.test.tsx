import { fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { ActivityPage } from "../activity/ActivityPage";
import { usageFixture } from "../../test/usage-efficiency-fixture";
import { AccountCapacityPanel } from "./AccountCapacityPanel";

const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke, isTauri: () => true }));

function capacity() {
  return screen.getByRole("region", { name: "Account Capacity preview" });
}
function chooseUnverified() {
  fireEvent.click(within(capacity()).getByText("Capacity preview controls"));
  fireEvent.change(within(capacity()).getByLabelText("Capacity illustration"), {
    target: { value: "unverified" },
  });
}

beforeEach(() => {
  invoke.mockReset();
  invoke.mockResolvedValue(usageFixture());
});
afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("mock Account Capacity preview", () => {
  it("includes the visible disclosure label and category in every accessible name", () => {
    render(<AccountCapacityPanel />);
    for (const value of ["not-connected", "unverified"]) {
      fireEvent.change(within(capacity()).getByLabelText("Capacity illustration"), {
        target: { value },
      });
      for (const title of [
        "Codex subscription allowance",
        "API rate-limit capacity",
        "API spending / balance",
      ]) {
        const card = within(capacity()).getByRole("article", { name: title });
        const summary = within(card).getByText("View fields");
        expect(summary).toBeVisible();
        expect(summary).toHaveAccessibleName(`View fields — ${title}`);
      }
    }
    expect(invoke).not.toHaveBeenCalled();
  });

  it("keeps each account summary visible and all nine fields in independent native disclosures", () => {
    render(<AccountCapacityPanel />);
    const cards = within(capacity()).getAllByRole("article");
    for (const card of cards) {
      const details = card.querySelector("details");
      const field = card.querySelector("dd");
      if (!details || !field) throw new Error("Expected native field disclosure");
      expect(details).not.toHaveAttribute("open");
      expect(field).not.toBeVisible();
      expect(card.querySelector(".account-capacity__summary-status")).toBeVisible();
      fireEvent.click(within(card).getByText("View fields"));
      expect(details).toHaveAttribute("open");
      expect(field).toBeVisible();
      fireEvent.click(within(card).getByText("View fields"));
      expect(field).not.toBeVisible();
    }
    expect(capacity().querySelectorAll("dt")).toHaveLength(9);
    expect(invoke).not.toHaveBeenCalled();
  });

  it("defaults to absent values in three distinct account-scope cards", () => {
    render(<AccountCapacityPanel />);
    const region = within(capacity());
    expect(region.getByText("Mock account-capacity layout—not your balances.")).toBeVisible();
    expect(region.getByText(/does not describe your other Cortexa connections/)).toBeVisible();
    expect(
      region.getAllByRole("article").map((card) => within(card).getByRole("heading").textContent),
    ).toEqual([
      "Codex subscription allowance",
      "API rate-limit capacity",
      "API spending / balance",
    ]);
    expect(region.getByRole("status")).toHaveTextContent("Not connected");
    expect(capacity().querySelectorAll("dd")).toHaveLength(9);
    for (const field of capacity().querySelectorAll("dd"))
      expect(field).toHaveTextContent(/^Not connected$/);
    expect(region.getByText(/Headless is an execution mode, not a separate pool/)).toBeVisible();
    expect(region.getByText(/not interchangeable limits/)).toBeVisible();
    expect(region.getByText("Capacity preview controls").closest("details")).not.toHaveAttribute(
      "open",
    );
  });

  it("labels Unverified as an illustration and can return to Not connected", () => {
    render(<AccountCapacityPanel />);
    chooseUnverified();
    expect(within(capacity()).getByRole("status")).toHaveTextContent(
      "Unverified illustration — no account check has been attempted",
    );
    for (const field of capacity().querySelectorAll("dd"))
      expect(field).toHaveTextContent(/^Unverified$/);
    fireEvent.change(within(capacity()).getByLabelText("Capacity illustration"), {
      target: { value: "not-connected" },
    });
    expect(within(capacity()).getByRole("status")).toHaveTextContent("Not connected");
  });

  it("never represents absent capacity as a number, graphic, currency or sync timestamp", () => {
    render(<AccountCapacityPanel />);
    for (const state of ["not-connected", "unverified"]) {
      fireEvent.change(within(capacity()).getByLabelText("Capacity illustration"), {
        target: { value: state },
      });
      expect(capacity().textContent).not.toMatch(/[0-9%$€£]/);
      expect(capacity().querySelector("svg, progress, meter, time, input, a, button")).toBeNull();
      expect(within(capacity()).queryByRole("progressbar")).not.toBeInTheDocument();
      expect(capacity()).toHaveTextContent("Unknown is not zero");
    }
  });

  it("has no IPC, fetch or storage effects when mounted or toggled", () => {
    const fetch = vi.fn();
    vi.stubGlobal("fetch", fetch);
    const get = vi.spyOn(Storage.prototype, "getItem");
    const set = vi.spyOn(Storage.prototype, "setItem");
    const remove = vi.spyOn(Storage.prototype, "removeItem");
    render(<AccountCapacityPanel />);
    chooseUnverified();
    expect(invoke).not.toHaveBeenCalled();
    expect(fetch).not.toHaveBeenCalled();
    expect(get).not.toHaveBeenCalled();
    expect(set).not.toHaveBeenCalled();
    expect(remove).not.toHaveBeenCalled();
  });

  it("keeps capacity and activity selections and loading independent", async () => {
    render(<ActivityPage events={[]} />);
    const activity = within(screen.getByRole("region", { name: "Usage & Efficiency" }));
    chooseUnverified();
    expect(invoke).not.toHaveBeenCalled();
    fireEvent.change(activity.getByLabelText("Example connection"), { target: { value: "codex" } });
    fireEvent.click(activity.getByRole("radio", { name: "30 days" }));
    expect(within(capacity()).getByRole("status")).toHaveTextContent("Unverified illustration");
    expect(invoke).not.toHaveBeenCalled();
    fireEvent.click(activity.getByRole("button", { name: "Load examples" }));
    await activity.findByRole("img", { name: /Daily sample token activity/ });
    expect(invoke).toHaveBeenCalledTimes(1);
    expect(invoke).toHaveBeenCalledWith("get_usage_efficiency_demo_projection");
    fireEvent.change(within(capacity()).getByLabelText("Capacity illustration"), {
      target: { value: "not-connected" },
    });
    expect(activity.getByRole("img", { name: /Daily sample token activity/ })).toBeVisible();
    expect(activity.getByLabelText("Example connection")).toHaveValue("codex");
    expect(activity.getByRole("radio", { name: "30 days" })).toBeChecked();
    expect(invoke).toHaveBeenCalledTimes(1);
    expect(screen.getByText("No session activity")).toBeVisible();
  });

  it("does not persist illustration state across remounts and uses unique heading IDs", () => {
    const ui = render(<AccountCapacityPanel />);
    chooseUnverified();
    ui.unmount();
    render(
      <>
        <AccountCapacityPanel />
        <AccountCapacityPanel />
      </>,
    );
    const panels = screen.getAllByRole("region", { name: "Account Capacity preview" });
    expect(panels[0]?.getAttribute("aria-labelledby")).not.toEqual(
      panels[1]?.getAttribute("aria-labelledby"),
    );
    for (const panel of panels)
      expect(within(panel).getByRole("status")).toHaveTextContent("Not connected");
  });
  it("uses textual neutral status badges without adding capacity actions or graphics", () => {
    render(<AccountCapacityPanel />);
    for (const value of ["not-connected", "unverified"]) {
      fireEvent.change(within(capacity()).getByLabelText("Capacity illustration"), {
        target: { value },
      });
      const badges = capacity().querySelectorAll(".account-capacity__status-badge");
      expect(badges).toHaveLength(9);
      for (const badge of badges) {
        expect(badge).toHaveTextContent(
          value === "unverified" ? /^Unverified$/ : /^Not connected$/,
        );
        expect(badge.closest("dd")).not.toBeNull();
      }
      expect(capacity().querySelector("button, svg, progress, meter")).toBeNull();
    }
  });
});
