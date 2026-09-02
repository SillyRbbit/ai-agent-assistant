import { describe, expect, it } from "vitest";

import {
  ACTIVITY_PANEL_DEFAULT_HEIGHT,
  ACTIVITY_PANEL_MAX_HEIGHT,
  ACTIVITY_PANEL_MIN_HEIGHT,
  applicationShellReducer,
  createApplicationShellState,
  getActivityPanelMaxHeight,
} from "./applicationShellState";

describe("applicationShellState", () => {
  it("starts with an expanded navigation rail and collapsed secondary panels", () => {
    expect(createApplicationShellState(900)).toEqual({
      activityExpanded: false,
      activityHeight: ACTIVITY_PANEL_DEFAULT_HEIGHT,
      activityMaxHeight: 360,
      inspectorExpanded: false,
      navigationExpanded: true,
    });
  });

  it("toggles each shell region without changing the others", () => {
    const initial = createApplicationShellState(900);
    const navigationCollapsed = applicationShellReducer(initial, {
      type: "toggle-navigation",
    });
    const inspectorExpanded = applicationShellReducer(navigationCollapsed, {
      type: "toggle-inspector",
    });
    const activityExpanded = applicationShellReducer(inspectorExpanded, {
      type: "toggle-activity",
    });

    expect(navigationCollapsed).toMatchObject({
      activityExpanded: false,
      inspectorExpanded: false,
      navigationExpanded: false,
    });
    expect(inspectorExpanded).toMatchObject({
      activityExpanded: false,
      inspectorExpanded: true,
      navigationExpanded: false,
    });
    expect(activityExpanded).toMatchObject({
      activityExpanded: true,
      activityHeight: ACTIVITY_PANEL_DEFAULT_HEIGHT,
      inspectorExpanded: true,
      navigationExpanded: false,
    });
  });

  it("opens and closes secondary regions idempotently", () => {
    const initial = createApplicationShellState(900);
    const activityOpen = applicationShellReducer(initial, { type: "activity-opened" });
    const activityStillOpen = applicationShellReducer(activityOpen, { type: "activity-opened" });
    const inspectorOpen = applicationShellReducer(activityStillOpen, { type: "inspector-opened" });
    const inspectorStillOpen = applicationShellReducer(inspectorOpen, {
      type: "inspector-opened",
    });
    const inspectorClosed = applicationShellReducer(inspectorStillOpen, {
      type: "inspector-closed",
    });
    const activityClosed = applicationShellReducer(inspectorClosed, { type: "activity-closed" });

    expect(activityOpen.activityExpanded).toBe(true);
    expect(activityStillOpen).toBe(activityOpen);
    expect(inspectorOpen.inspectorExpanded).toBe(true);
    expect(inspectorStillOpen).toBe(inspectorOpen);
    expect(inspectorClosed.inspectorExpanded).toBe(false);
    expect(activityClosed.activityExpanded).toBe(false);
  });

  it("clamps activity resizing to the current viewport bounds", () => {
    const initial = createApplicationShellState(720);
    const tooTall = applicationShellReducer(initial, {
      height: ACTIVITY_PANEL_MAX_HEIGHT + 100,
      type: "activity-resized",
    });
    const tooShort = applicationShellReducer(tooTall, {
      height: 12,
      type: "activity-resized",
    });

    expect(initial.activityMaxHeight).toBe(302);
    expect(tooTall.activityHeight).toBe(302);
    expect(tooShort.activityHeight).toBe(ACTIVITY_PANEL_MIN_HEIGHT);
  });

  it("preserves a valid size across collapse and clamps it after viewport resize", () => {
    const initial = createApplicationShellState(1200);
    const resized = applicationShellReducer(initial, {
      height: 320,
      type: "activity-resized",
    });
    const expanded = applicationShellReducer(resized, { type: "toggle-activity" });
    const collapsed = applicationShellReducer(expanded, { type: "toggle-activity" });
    const constrained = applicationShellReducer(collapsed, {
      height: 520,
      type: "viewport-resized",
    });
    const reopened = applicationShellReducer(constrained, { type: "toggle-activity" });

    expect(collapsed.activityHeight).toBe(320);
    expect(constrained.activityMaxHeight).toBe(218);
    expect(constrained.activityHeight).toBe(218);
    expect(reopened).toMatchObject({ activityExpanded: true, activityHeight: 218 });
  });

  it("uses stable fallback bounds for invalid viewport dimensions", () => {
    expect(getActivityPanelMaxHeight(Number.NaN)).toBe(ACTIVITY_PANEL_MAX_HEIGHT);
    expect(getActivityPanelMaxHeight(0)).toBe(ACTIVITY_PANEL_MAX_HEIGHT);
  });
});
