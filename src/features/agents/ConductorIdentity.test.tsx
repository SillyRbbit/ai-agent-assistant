/// <reference types="node" />
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { ConductorIdentity } from "./ConductorIdentity";
import { CoordinatorAvatar } from "./BotAppearance";
import { setBotAnimations } from "./botMotion";
import { BOT_MASCOTS } from "./botMascots";
import { AGENT_IDS, DEFAULT_BOT_IDENTITY } from "../../infrastructure/tauri/agent-chat-client";
import { WORKFLOWS, type Run, type Status } from "../../infrastructure/tauri/collaboration-client";
let observe: IntersectionObserverCallback;
let reduced = false;
let notify: (() => void) | undefined;
const disconnect = vi.fn();
beforeEach(() => {
  const values = new Map<string, string>();
  vi.stubGlobal("localStorage", {
    getItem: (k: string) => values.get(k) ?? null,
    setItem: (k: string, v: string) => values.set(k, v),
  });
  reduced = false;
  Object.defineProperty(document, "hidden", { configurable: true, value: false });
  vi.stubGlobal("matchMedia", () => ({
    matches: reduced,
    addEventListener: (_: string, cb: () => void) => {
      notify = cb;
    },
    removeEventListener: vi.fn(),
  }));
  vi.stubGlobal(
    "IntersectionObserver",
    class {
      constructor(cb: IntersectionObserverCallback) {
        observe = cb;
      }
      observe() {
        /* Test controls visibility explicitly. */
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
    observe([{ isIntersecting: value } as IntersectionObserverEntry], {} as IntersectionObserver);
  });
}
function frame(container: HTMLElement) {
  const el = container.querySelector("[data-motion]");
  if (!el) throw Error("No mascot");
  return el;
}
function end(el: Element, name: string) {
  const ev = new Event("webkitAnimationEnd", { bubbles: true });
  Object.defineProperty(ev, "animationName", { value: name });
  const body = el.querySelector(".bot-mascot-body");
  if (!body) throw Error("Missing animated body");
  fireEvent(body, ev);
}
function run(status: Status, sequence = 1): Run {
  return {
    id: "r1",
    status,
    sequence,
    error: null,
    input: { workflow: "research", objective: "Synthetic", sources: [] },
    stages: WORKFLOWS.research.map((agentId, i) => ({
      id: `r1/stage/${String(i)}`,
      status: status === "running" && i > 0 ? "queued" : status,
      timestamp: 0,
      provisional: "",
      input: "",
      participant: {
        agentId,
        role: agentId,
        identity: DEFAULT_BOT_IDENTITY,
        connection: "simulation",
        model: "simulation",
        effort: "default",
        endpoint: "",
        localAuth: false,
        ownerInstructions: "",
        revision: 0,
      },
      handoff:
        status === "completed"
          ? {
              version: 1,
              stage: i,
              agentId,
              status: "complete",
              summary: "Synthetic",
              findings: [],
              evidence: [],
              limitations: [],
            }
          : null,
    })),
  };
}
describe("Conductor bounded presentation", () => {
  it("is separate from nine configurable IDs and static in dense views", () => {
    expect(Object.keys(BOT_MASCOTS)).toEqual([...AGENT_IDS]);
    const v = render(<CoordinatorAvatar />);
    expect(frame(v.container)).toHaveAttribute("data-motion", "static");
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
    expect(v.container.firstChild).toHaveClass("bot-appearance");
  });
  it("waves free hand, returns idle and supports explicit finite spin", () => {
    const v = render(<ConductorIdentity expressive />);
    visible();
    expect(frame(v.container)).toHaveAttribute("data-motion", "greeting");
    end(frame(v.container), "mascot-wave");
    expect(frame(v.container)).toHaveAttribute("data-motion", "idle");
    fireEvent.click(screen.getByRole("button", { name: "Playful spin" }));
    expect(frame(v.container)).toHaveAttribute("data-motion", "playful");
    end(frame(v.container), "mascot-spin");
    expect(frame(v.container)).toHaveAttribute("data-motion", "idle");
  });
  it("conducts only actual running and celebrates overall completion exactly once", () => {
    const v = render(<ConductorIdentity expressive current run={run("running")} />);
    visible();
    expect(frame(v.container)).toHaveAttribute("data-motion", "working");
    expect(screen.getByRole("button")).toBeDisabled();
    const done = run("completed", 2);
    v.rerender(<ConductorIdentity expressive current run={done} />);
    expect(frame(v.container)).toHaveAttribute("data-motion", "success");
    end(frame(v.container), "mascot-jump");
    v.rerender(<ConductorIdentity expressive current run={{ ...done, sequence: 3 }} />);
    expect(frame(v.container)).toHaveAttribute("data-motion", "idle");
    v.rerender(<ConductorIdentity expressive current run={run("running", 4)} />);
    v.rerender(<ConductorIdentity expressive current run={run("completed", 5)} />);
    expect(frame(v.container)).not.toHaveAttribute("data-motion", "success");
  });
  it("one completed stage cannot claim overall success or active execution", () => {
    const v = render(<ConductorIdentity expressive current run={run("running")} />);
    visible();
    const one = run("running", 2);
    v.rerender(
      <ConductorIdentity
        expressive
        current
        run={{
          ...one,
          stages: one.stages.map((s, i) => (i === 0 ? { ...s, status: "completed" } : s)),
        }}
      />,
    );
    expect(frame(v.container)).toHaveAttribute("data-motion", "idle");
  });
  it.each([
    "queued",
    "failed",
    "cancelled",
    "interrupted",
    "partial",
    "blocked",
    "approval_waiting",
  ])("stops conducting for %s without jump", (status) => {
    const v = render(<ConductorIdentity expressive current run={run("running")} />);
    visible();
    v.rerender(<ConductorIdentity expressive current run={run(status as Status, 2)} />);
    expect(frame(v.container)).toHaveAttribute("data-motion", "idle");
  });
  it("rejects incomplete handoffs, errors, stale sequences and changed identity", () => {
    for (const next of [
      { ...run("completed", 2), error: "failure" },
      { ...run("completed", 2), stages: run("queued").stages },
      run("completed", 1),
      { ...run("completed", 2), id: "r2" },
    ]) {
      const v = render(<ConductorIdentity expressive current run={run("running")} />);
      visible();
      v.rerender(<ConductorIdentity expressive current run={next} />);
      expect(frame(v.container)).not.toHaveAttribute("data-motion", "success");
      v.unmount();
    }
  });
  it("does not replay completed history on mount/reentry or stale recovery", () => {
    const done = run("completed", 2);
    const v = render(<ConductorIdentity expressive current run={done} />);
    visible();
    expect(frame(v.container)).not.toHaveAttribute("data-motion", "success");
    v.rerender(<ConductorIdentity expressive current={false} run={run("running", 3)} />);
    expect(frame(v.container)).not.toHaveAttribute("data-motion", "working");
    v.rerender(<ConductorIdentity expressive current run={run("completed", 4)} />);
    expect(frame(v.container)).not.toHaveAttribute("data-motion", "success");
  });
  it("consumes hidden completion and respects disable/reduced-motion/observer cleanup", () => {
    const v = render(<ConductorIdentity expressive current run={run("running")} />);
    visible();
    visible(false);
    v.rerender(<ConductorIdentity expressive current run={run("completed", 2)} />);
    visible();
    expect(frame(v.container)).toHaveAttribute("data-motion", "idle");
    act(() => {
      setBotAnimations(false);
    });
    expect(frame(v.container)).toHaveAttribute("data-motion", "static");
    act(() => {
      setBotAnimations(true);
      reduced = true;
      notify?.();
    });
    expect(frame(v.container)).toHaveAttribute("data-motion", "static");
    v.unmount();
    expect(disconnect).toHaveBeenCalled();
  });
  it("uses actual distinct eyelid/free-hand/baton cells and keeps reduced motion last", () => {
    const css = readFileSync("src/features/agents/bot-mascots.css", "utf8");
    expect(css).toContain("mascot-conduct");
    expect(css).toContain("100% 100%");
    expect(css.lastIndexOf("@media")).toBeGreaterThan(css.indexOf("@keyframes mascot-conduct"));
    const v = render(<CoordinatorAvatar expressive />);
    expect(frame(v.container)).toHaveStyle({
      "--atlas-size": "200% 200%",
      "--blink-frame": "100% 0%",
      "--wave-frame": "0% 100%",
    });
  });
});
