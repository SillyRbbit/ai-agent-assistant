import { useSyncExternalStore } from "react";
import type { AgentChatSnapshot } from "../../infrastructure/tauri/agent-chat-client";
const KEY = "cortexa.bot-animation.v1";
const EVENT = "cortexa-bot-motion-change";
function subscribe(notify: () => void) {
  window.addEventListener(EVENT, notify);
  window.addEventListener("storage", notify);
  return () => {
    window.removeEventListener(EVENT, notify);
    window.removeEventListener("storage", notify);
  };
}
function enabled() {
  try {
    return localStorage.getItem(KEY) !== "off";
  } catch {
    return false;
  }
}
export function setBotAnimations(enabled: boolean): boolean {
  try {
    localStorage.setItem(KEY, enabled ? "on" : "off");
    window.dispatchEvent(new Event(EVENT));
    return true;
  } catch {
    return false;
  }
}
export function useBotAnimations() {
  return useSyncExternalStore(subscribe, enabled, () => false);
}
function subscribeReduced(notify: () => void) {
  if (typeof window.matchMedia !== "function")
    return () => {
      /* No browser motion API. */
    };
  const query = window.matchMedia("(prefers-reduced-motion: reduce)");
  query.addEventListener("change", notify);
  return () => {
    query.removeEventListener("change", notify);
  };
}
export function useReducedBotMotion() {
  return useSyncExternalStore(
    subscribeReduced,
    () =>
      typeof window.matchMedia !== "function" ||
      window.matchMedia("(prefers-reduced-motion: reduce)").matches,
    () => true,
  );
}
// Called only after the host accepts a bound snapshot. A fresh Send may complete
// synchronously; ordinary polling must have observed an active predecessor.
export function isNewBotSuccess(
  previous: AgentChatSnapshot,
  next: AgentChatSnapshot,
  freshSend: boolean,
): boolean {
  return (
    next.status === "completed" &&
    !next.busy &&
    next.conversationId === previous.conversationId &&
    next.agentId === previous.agentId &&
    (freshSend || (previous.busy && previous.status !== "completed"))
  );
}
