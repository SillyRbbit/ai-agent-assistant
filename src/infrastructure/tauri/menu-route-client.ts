import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import { isAssistantMenuRoute, type AssistantMenuRoute } from "../../application/navigation";

export const ASSISTANT_MENU_ROUTE_EVENT = "assistant-menu-route";

export type AssistantMenuRouteListener = (route: AssistantMenuRoute) => void;

export interface MenuRouteSource {
  readonly subscribe: (listener: AssistantMenuRouteListener) => Promise<UnlistenFn>;
}

function isObjectRecord(value: unknown): value is object {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

export function parseAssistantMenuRoutePayload(payload: unknown): AssistantMenuRoute | null {
  if (!isObjectRecord(payload)) {
    return null;
  }

  const keys = Object.keys(payload);

  if (keys.length !== 1 || keys[0] !== "route") {
    return null;
  }

  if (!("route" in payload)) {
    return null;
  }

  return isAssistantMenuRoute(payload.route) ? payload.route : null;
}

export const tauriMenuRouteSource: MenuRouteSource = {
  async subscribe(listener) {
    return listen<unknown>(ASSISTANT_MENU_ROUTE_EVENT, (event) => {
      const route = parseAssistantMenuRoutePayload(event.payload);

      if (route !== null) {
        listener(route);
      }
    });
  },
};
