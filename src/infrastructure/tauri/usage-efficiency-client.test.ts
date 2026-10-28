import { beforeEach, describe, expect, it, vi } from "vitest";
import { usageFixture } from "../../test/usage-efficiency-fixture";
import { fetchUsageProjection, parseUsageProjection } from "./usage-efficiency-client";

const { invokeMock, nativeMock } = vi.hoisted(() => ({ invokeMock: vi.fn(), nativeMock: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock, isTauri: nativeMock }));

beforeEach(() => {
  invokeMock.mockReset();
  nativeMock.mockReturnValue(true);
});

describe("closed usage projection", () => {
  it("accepts bounded complete source selections and retains zero versus absence", () => {
    const parsed = parseUsageProjection(usageFixture());
    expect(parsed?.views).toHaveLength(24);
    expect(
      parsed?.views.find((v) => v.category === "codex" && v.example === "activity")?.tokens[1],
    ).toBe("missing");
    expect(
      parsed?.views.find((v) => v.category === "openai_api" && v.example === "activity")?.tokens[1],
    ).toBe(0);
    expect(Object.isFrozen(parsed?.views[0]?.tokens)).toBe(true);
  });
  it.each([
    null,
    {},
    [],
    { ...usageFixture(), schemaVersion: 2 },
    { ...usageFixture(), accountId: "excluded" },
    { ...usageFixture(), views: usageFixture().views.slice(1) },
    { ...usageFixture(), views: Array(25).fill(usageFixture().views[0]) },
    { ...usageFixture(), provenance: "live" },
  ])("rejects malformed envelope %#", (value) => {
    expect(parseUsageProjection(value)).toBeUndefined();
  });
  it.each([
    { category: "account" },
    { days: 31 },
    { start: "2026-09-23" },
    { end: "2026-10-02" },
    { entries: 129 },
    { entries: -1 },
    { completedTasks: 128 },
    { total: null },
    { total: Number.MAX_SAFE_INTEGER + 1 },
    { cachePercent: 101 },
    { completedTaskMean: Infinity },
    { tokens: [1, 2, 1, 0] },
    { tokens: [1, 0, 1, 2] },
    { tokens: [true, 0, 1, 0] },
    { tokens: [1.5, 0, 1, 0] },
    { tokens: [-1, 0, 1, 0] },
    { extra: 0 },
    { daily: [] },
    { example: "live" },
  ])("rejects malformed view %#", (patch) => {
    const source = usageFixture();
    expect(
      parseUsageProjection({
        ...source,
        views: [{ ...source.views[0], ...patch }, ...source.views.slice(1)],
      }),
    ).toBeUndefined();
  });
  it("rejects duplicate combinations and private/unknown daily fields", () => {
    const source = usageFixture();
    expect(
      parseUsageProjection({ ...source, views: [source.views[1], ...source.views.slice(1)] }),
    ).toBeUndefined();
    const first = source.views[0];
    expect(
      parseUsageProjection({
        ...source,
        views: [
          { ...first, daily: first?.daily.map((d) => ({ ...d, taskId: "excluded" })) },
          ...source.views.slice(1),
        ],
      }),
    ).toBeUndefined();
  });
  it("requires a truly empty zero illustration", () => {
    const source = usageFixture();
    expect(
      parseUsageProjection({
        ...source,
        views: source.views.map((v) => (v.example === "empty" ? { ...v, total: "missing" } : v)),
      }),
    ).toBeUndefined();
  });
  it("invokes only the no-input command and rejects unsafe upstream detail", async () => {
    invokeMock.mockResolvedValueOnce(usageFixture());
    await expect(fetchUsageProjection()).resolves.toHaveProperty("schemaVersion", 1);
    expect(invokeMock).toHaveBeenCalledWith("get_usage_efficiency_demo_projection");
    invokeMock.mockRejectedValueOnce(new Error("private native detail"));
    await expect(fetchUsageProjection()).rejects.toThrow(
      "Usage & Efficiency examples unavailable.",
    );
  });
  it("rejects malformed native results and browser-only execution without invoking", async () => {
    invokeMock.mockResolvedValueOnce({});
    await expect(fetchUsageProjection()).rejects.toThrow("examples unavailable");
    nativeMock.mockReturnValue(false);
    invokeMock.mockClear();
    await expect(fetchUsageProjection()).rejects.toThrow("Native preview unavailable.");
    expect(invokeMock).not.toHaveBeenCalled();
  });
});
