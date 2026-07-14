import { describe, expect, it } from "vitest";

import { APP_ROUTES, type AppRoute } from "./navigation";
import { applicationReducer, INITIAL_APPLICATION_STATE } from "./state";

describe("applicationReducer", () => {
  it("exposes every route in deterministic navigation order", () => {
    expect(APP_ROUTES).toEqual([
      "conversations",
      "tasks",
      "memory",
      "activity",
      "integrations",
      "permissions",
      "settings",
    ]);
  });

  it.each<AppRoute>([
    "conversations",
    "tasks",
    "memory",
    "activity",
    "integrations",
    "permissions",
    "settings",
  ])("navigates deterministically to %s", (route) => {
    expect(
      applicationReducer(INITIAL_APPLICATION_STATE, { route, type: "navigate" }).activeRoute,
    ).toBe(route);
  });

  it("returns the same state when navigation is unchanged", () => {
    expect(
      applicationReducer(INITIAL_APPLICATION_STATE, {
        route: "conversations",
        type: "navigate",
      }),
    ).toBe(INITIAL_APPLICATION_STATE);
  });

  it("routes a new request to conversations and clears the in-memory draft", () => {
    const state = {
      activeRoute: "settings" as const,
      composerDraft: "Unsent text",
    };

    expect(
      applicationReducer(state, {
        route: "new_request",
        type: "menu-route-received",
      }),
    ).toEqual({
      activeRoute: "conversations",
      composerDraft: "",
    });
  });

  it("routes the tasks placeholder to the tasks page", () => {
    expect(
      applicationReducer(INITIAL_APPLICATION_STATE, {
        route: "tasks_placeholder",
        type: "menu-route-received",
      }).activeRoute,
    ).toBe("tasks");
  });
});
