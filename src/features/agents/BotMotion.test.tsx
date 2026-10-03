import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { BotAvatar } from "./BotAppearance";
import { BOT_MASCOTS } from "./botMascots";
import { isNewBotSuccess, setBotAnimations } from "./botMotion";
import {
  AGENT_IDS,
  DEFAULT_BOT_IDENTITY,
  type AgentChatSnapshot,
} from "../../infrastructure/tauri/agent-chat-client";
import { MascotFixture } from "../../../scripts/browser/mascot-fixture";
let intersection: IntersectionObserverCallback;
let reduced = false;
let notifyReduced: (() => void) | undefined;
const disconnect = vi.fn();
beforeEach(() => {
  const storage = new Map<string, string>();
  vi.stubGlobal("localStorage", {
    getItem: (key: string) => storage.get(key) ?? null,
    setItem: (key: string, value: string) => storage.set(key, value),
  });
  reduced = false;
  Object.defineProperty(document, "hidden", { configurable: true, value: false });
  vi.stubGlobal("matchMedia", () => ({
    matches: reduced,
    addEventListener: (_: string, cb: () => void) => {
      notifyReduced = cb;
    },
    removeEventListener: vi.fn(),
  }));
  vi.stubGlobal(
    "IntersectionObserver",
    class {
      constructor(cb: IntersectionObserverCallback) {
        intersection = cb;
      }
      observe() {
        /* Explicit visibility controlled by the observer double. */
      }
      disconnect = disconnect;
    },
  );
});
afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});
function visible(value = true) {
  act(() => {
    intersection(
      [{ isIntersecting: value } as IntersectionObserverEntry],
      {} as IntersectionObserver,
    );
  });
}
function end(element: Element, name: string) {
  const event = new Event("webkitAnimationEnd", { bubbles: true });
  Object.defineProperty(event, "animationName", { value: name });
  fireEvent(element, event);
}
function required(container: Element, selector: string): Element {
  const result = container.querySelector(selector);
  if (!result) throw new Error(`Missing ${selector}`);
  return result;
}
function motion(container: HTMLElement) {
  return required(container, "[data-motion]");
}
describe("shared bot motion", () => {
  it("retains the graph avatar grid-cell contract for every mascot", () => {
    const { container } = render(
      <>
        {AGENT_IDS.map((id) => (
          <div key={id} className="operational-card">
            <BotAvatar agentId={id} identity={DEFAULT_BOT_IDENTITY} />
            <strong>{id}</strong>
            <span>Canonical role</span>
          </div>
        ))}
      </>,
    );
    expect(container.querySelectorAll(".operational-card > .bot-appearance")).toHaveLength(9);
    expect(
      container.querySelectorAll(".operational-card > span:not(.bot-appearance)"),
    ).toHaveLength(9);
  });
  it("maps exactly nine stable IDs to distinct local atlases and varied blink timing", () => {
    expect(Object.keys(BOT_MASCOTS)).toEqual([...AGENT_IDS]);
    expect(new Set(Object.values(BOT_MASCOTS).map((v) => v.src)).size).toBe(9);
    expect(new Set(Object.values(BOT_MASCOTS).map((v) => v.blinkSeconds)).size).toBe(9);
  });
  it.each(AGENT_IDS)(
    "%s has finite greeting, jump, spin and idle; duplicate successes do not replay",
    (agentId) => {
      const props = {
        agentId,
        identity: DEFAULT_BOT_IDENTITY,
        expressive: true,
        greeting: true,
        success: 0,
      };
      const view = render(<BotAvatar {...props} />);
      visible();
      expect(motion(view.container)).toHaveAttribute("data-motion", "greeting");
      end(required(motion(view.container), ".bot-mascot-sprite"), "mascot-wave");
      expect(motion(view.container)).toHaveAttribute("data-motion", "idle");
      view.rerender(<BotAvatar {...props} success={1} />);
      expect(motion(view.container)).toHaveAttribute("data-motion", "success");
      end(required(motion(view.container), ".bot-mascot-body"), "mascot-jump");
      view.rerender(<BotAvatar {...props} success={1} />);
      expect(motion(view.container)).toHaveAttribute("data-motion", "idle");
      fireEvent.click(screen.getByRole("button", { name: "Playful spin" }));
      expect(motion(view.container)).toHaveAttribute("data-motion", "playful");
      expect(screen.getByRole("button")).toBeDisabled();
      end(required(motion(view.container), ".bot-mascot-body"), "mascot-spin");
      expect(motion(view.container)).toHaveAttribute("data-motion", "idle");
    },
  );
  it("does not replay historical completion on mount, hides interrupted reactions and releases observers", () => {
    const view = render(
      <BotAvatar agentId="research" identity={DEFAULT_BOT_IDENTITY} expressive success={5} />,
    );
    visible();
    expect(motion(view.container)).toHaveAttribute("data-motion", "idle");
    fireEvent.click(screen.getByRole("button"));
    visible(false);
    expect(motion(view.container)).toHaveAttribute("data-motion", "static");
    visible();
    expect(motion(view.container)).toHaveAttribute("data-motion", "idle");
    fireEvent.click(screen.getByRole("button"));
    Object.defineProperty(document, "hidden", { configurable: true, value: true });
    fireEvent(document, new Event("visibilitychange"));
    expect(motion(view.container)).toHaveAttribute("data-motion", "static");
    view.unmount();
    expect(disconnect).toHaveBeenCalled();
  });
  it("persists disable across remount, honors live reduced motion, never delays a greeting until re-enabled", () => {
    expect(setBotAnimations(false)).toBe(true);
    let view = render(
      <BotAvatar agentId="coding" identity={DEFAULT_BOT_IDENTITY} expressive greeting />,
    );
    visible();
    expect(motion(view.container)).toHaveAttribute("data-motion", "static");
    view.unmount();
    view = render(
      <BotAvatar agentId="coding" identity={DEFAULT_BOT_IDENTITY} expressive greeting />,
    );
    visible();
    expect(motion(view.container)).toHaveAttribute("data-motion", "static");
    act(() => {
      setBotAnimations(true);
    });
    expect(motion(view.container)).toHaveAttribute("data-motion", "idle");
    act(() => {
      reduced = true;
      notifyReduced?.();
    });
    expect(motion(view.container)).toHaveAttribute("data-motion", "static");
  });
  it("leaves dense views static and custom icon choices unchanged", () => {
    const view = render(<BotAvatar agentId="research" identity={DEFAULT_BOT_IDENTITY} />);
    expect(motion(view.container)).toHaveAttribute("data-motion", "static");
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
    view.rerender(
      <BotAvatar
        agentId="research"
        identity={{ ...DEFAULT_BOT_IDENTITY, avatar: "compass" }}
        expressive
      />,
    );
    expect(view.container.querySelector(".lucide-compass")).toBeInTheDocument();
    expect(view.container.querySelector(".bot-mascot-space")).not.toBeInTheDocument();
  });
  it("offline gallery reuses actual component with all nine IDs", () => {
    render(<MascotFixture />);
    expect(
      screen.getByRole("navigation", { name: "Nine mascots" }).querySelectorAll("button"),
    ).toHaveLength(9);
  });
  it("accepts new completions only, never streaming/error/stopped/duplicates/other conversations", () => {
    const previous = {
      conversationId: "c1",
      agentId: "research",
      busy: true,
      status: "streaming",
    } as AgentChatSnapshot;
    const done = { ...previous, status: "completed", busy: false } as AgentChatSnapshot;
    expect(isNewBotSuccess(previous, done, false)).toBe(true);
    expect(isNewBotSuccess(done, done, false)).toBe(false);
    for (const status of ["error", "stopped", "streaming", "idle"] as const)
      expect(isNewBotSuccess(previous, { ...done, status }, true)).toBe(false);
    expect(isNewBotSuccess(previous, { ...done, conversationId: "other" }, true)).toBe(false);
    expect(isNewBotSuccess({ ...previous, busy: false, status: "idle" }, done, true)).toBe(true);
  });
});
