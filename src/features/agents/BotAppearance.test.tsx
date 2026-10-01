import { describe, it, expect } from "vitest";
import {
  BOT_AVATARS,
  BOT_COLORS,
  AGENT_IDS,
  parseBotIdentity,
  DEFAULT_BOT_IDENTITY,
} from "../../infrastructure/tauri/agent-chat-client";
import { BOT_COLOR_VALUES, ROLE_DESCRIPTIONS } from "./botAppearanceValues";
function luminance(hex: string) {
  const rgb = hex.slice(1).match(/../g) ?? [];
  const c = rgb.map((h) => {
    const v = parseInt(h, 16) / 255;
    return v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
  });
  return (c[0] ?? 0) * 0.2126 + (c[1] ?? 0) * 0.7152 + (c[2] ?? 0) * 0.0722;
}
describe("Bot appearance", () => {
  it("preserves original avatar ids and validates 18 choices and 12 colors", () => {
    expect(BOT_AVATARS).toHaveLength(18);
    expect(BOT_COLORS).toHaveLength(12);
    expect(BOT_AVATARS.slice(0, 6)).toEqual(["bot", "compass", "spark", "leaf", "shield", "star"]);
    for (const avatar of BOT_AVATARS)
      for (const color of BOT_COLORS)
        expect(parseBotIdentity({ ...DEFAULT_BOT_IDENTITY, avatar, color }).color).toBe(color);
  });
  it("uses contrast of at least 4.5:1 on the explicit avatar background in either theme", () => {
    for (const hex of Object.values(BOT_COLOR_VALUES))
      expect((luminance(hex) + 0.05) / (luminance("#101722") + 0.05)).toBeGreaterThanOrEqual(4.5);
  });
  it("keeps legacy empty descriptions and supplies bounded role guidance for all nine", () => {
    const { color, ...legacy } = DEFAULT_BOT_IDENTITY;
    expect(color).toBe("blue");
    expect(parseBotIdentity(legacy).description).toBe("");
    expect(Object.keys(ROLE_DESCRIPTIONS).sort()).toEqual([...AGENT_IDS].sort());
    for (const d of Object.values(ROLE_DESCRIPTIONS)) {
      expect(d.length).toBeLessThanOrEqual(280);
      expect(d).not.toContain("executes");
    }
  });
});
