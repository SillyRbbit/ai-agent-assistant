import { describe, expect, it } from "vitest";
import { withBotNames } from "./commandCenterBotNames";
import { buildCommandCenterProjection } from "./commandCenterFixtures";
import { COMMAND_CENTER_AGENT_IDS, COMMAND_CENTER_SCENARIO_IDS } from "./commandCenterProjection";

const names = new Map(
  COMMAND_CENTER_AGENT_IDS.map((id, i) => [
    id,
    ["Nova", "Mira", "Ada", "Atlas", "Orion", "Clio", "Vera", "Sable", "Tempo"][i] ?? "",
  ]),
);
describe("saved bot graph names", () => {
  it.each(COMMAND_CENTER_SCENARIO_IDS)(
    "keeps %s routing and fixture evidence intact",
    (scenario) => {
      const original = buildCommandCenterProjection(scenario);
      const named = withBotNames(original, names);
      expect(named.edges).toBe(original.edges);
      expect(named.groups).toBe(original.groups);
      expect(named.events).toBe(original.events);
      for (const [i, node] of named.nodes.entries()) {
        const before = original.nodes[i];
        if (node.kind !== "agent") {
          expect(node).toBe(before);
          continue;
        }
        expect(node.label).toBe(names.get(node.agentId));
        expect(node.inspector.title).toBe(node.label);
        expect(node.inspector.facts[0]).toEqual({ label: "Canonical role", value: before?.label });
        expect({ ...node, label: before?.label, inspector: before?.inspector }).toEqual(before);
        expect(before?.label).not.toBe(node.label);
      }
      expect(Object.isFrozen(named)).toBe(true);
    },
  );
  it.each(["", "   ", "a".repeat(49), "bad\nname", "bad\u007fname"])(
    "uses canonical roles for invalid/missing names",
    (value) => {
      const original = buildCommandCenterProjection("catalog-idle");
      expect(withBotNames(original, new Map([["research", value]]))).toEqual(original);
    },
  );
});

describe("Unicode bot-name boundary", () => {
  it.each(["🧭".repeat(25), "🧭".repeat(48), "a".repeat(48), "a🧭".repeat(24)])(
    "keeps a nickname accepted by the profile character bound",
    async (nickname) => {
      const { DEFAULT_BOT_IDENTITY, parseBotIdentity } =
        await import("../../infrastructure/tauri/agent-chat-client");
      expect(parseBotIdentity({ ...DEFAULT_BOT_IDENTITY, nickname }).nickname).toBe(nickname);
      const original = buildCommandCenterProjection("catalog-idle");
      const named = withBotNames(original, new Map([["research", nickname]]));
      const before = original.nodes.find((node) => node.agentId === "research");
      const after = named.nodes.find((node) => node.agentId === "research");
      expect(after?.label).toBe(nickname);
      expect(after?.inspector.title).toBe(nickname);
      expect(after?.inspector.facts[0]).toEqual({
        label: "Canonical role",
        value: before?.label,
      });
      expect(after?.id).toBe(before?.id);
      expect(named.edges).toBe(original.edges);
      expect(named.groups).toBe(original.groups);
      expect(named.events).toBe(original.events);
    },
  );
  it.each(["🧭".repeat(49), `${"a🧭".repeat(24)}a`])(
    "retains canonical roles beyond the Unicode character bound",
    async (nickname) => {
      const { DEFAULT_BOT_IDENTITY, parseBotIdentity } =
        await import("../../infrastructure/tauri/agent-chat-client");
      expect(() => parseBotIdentity({ ...DEFAULT_BOT_IDENTITY, nickname })).toThrow("protocol");
      const original = buildCommandCenterProjection("catalog-idle");
      expect(withBotNames(original, new Map([["research", nickname]]))).toEqual(original);
    },
  );
});
