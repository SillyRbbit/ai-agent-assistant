import {
  USAGE_CATEGORIES,
  USAGE_DISCLOSURE,
  type UsageProjection,
} from "../infrastructure/tauri/usage-efficiency-client";

// Test-only numeric projection. Product code has no frontend fixture/fallback.
export function usageFixture(): UsageProjection {
  return {
    schemaVersion: 1,
    provenance: "synthetic",
    disclosure: USAGE_DISCLOSURE,
    views: USAGE_CATEGORIES.flatMap((category) =>
      ([7, 30] as const).flatMap((days) =>
        (["activity", "empty"] as const).map((example) => {
          const empty = example === "empty",
            input = empty ? 0 : days === 7 ? 1500 : 2550;
          const output = empty ? 0 : days === 7 ? 250 : 530;
          return {
            category,
            days,
            example,
            start: days === 7 ? "2026-09-24" : "2026-09-01",
            end: "2026-10-01" as const,
            entries: empty ? 0 : days === 7 ? 3 : 5,
            completedTasks: empty ? 0 : days === 7 ? 1 : 2,
            tokens: [
              input,
              empty ? 0 : category === "codex" ? "missing" : 0,
              output,
              empty
                ? 0
                : category === "anthropic_api"
                  ? "unavailable"
                  : category === "ollama"
                    ? "unverified"
                    : 0,
            ] as const,
            total: input + output,
            cachePercent: empty
              ? ("unavailable" as const)
              : category === "codex"
                ? ("missing" as const)
                : 0,
            completedTaskMean: empty
              ? ("unavailable" as const)
              : days === 7
                ? ("missing" as const)
                : 965,
            daily: empty
              ? []
              : Array.from({ length: days }, (_, i) => ({
                  date: `2026-09-${String(31 - days + i).padStart(2, "0")}`,
                  tokens: [i === 0 ? input : 0, 0, i === 0 ? output : 0, 0] as const,
                  total: i === 0 ? input + output : 0,
                })),
          };
        }),
      ),
    ),
  };
}
