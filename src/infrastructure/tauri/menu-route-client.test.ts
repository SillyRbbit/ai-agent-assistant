import { describe, expect, it } from "vitest";

import { parseAssistantMenuRoutePayload } from "./menu-route-client";

describe("parseAssistantMenuRoutePayload", () => {
  it.each(["new_request", "tasks_placeholder"])("accepts the closed route value %s", (route) => {
    expect(parseAssistantMenuRoutePayload({ route })).toBe(route);
  });

  it.each([
    undefined,
    null,
    "new_request",
    ["new_request"],
    {},
    { route: "settings" },
    { route: 42 },
    { route: "new_request", unexpected: true },
  ])("rejects an invalid or extended payload", (payload) => {
    expect(parseAssistantMenuRoutePayload(payload)).toBeNull();
  });
});
