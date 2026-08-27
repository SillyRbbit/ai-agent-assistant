import { describe, expect, it } from "vitest";

import { parseAppInfo } from "./app-info-client";

const valid = {
  architecture: "aarch64",
  environment: "development",
  name: "Cortexa",
  secureCore: true,
  target: "macos",
  version: "0.1.0",
};

describe("parseAppInfo", () => {
  it("accepts the exact trusted-core DTO", () => {
    expect(parseAppInfo(valid)).toEqual(valid);
    expect(parseAppInfo({ ...valid, name: "🧠".repeat(128) })).toBeDefined();
  });

  it.each([
    null,
    { architecture: "aarch64" },
    { ...valid, secureCore: false },
    { ...valid, environment: "test" },
    { ...valid, target: "" },
    { ...valid, name: "x".repeat(129) },
    { ...valid, extra: "rejected" },
    { ...valid, version: 1 },
  ])("rejects malformed IPC data", (value) => {
    expect(parseAppInfo(value)).toBeUndefined();
  });
});
