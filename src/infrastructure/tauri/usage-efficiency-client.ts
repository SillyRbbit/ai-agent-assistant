import { invoke, isTauri } from "@tauri-apps/api/core";

export const USAGE_DISCLOSURE = "Mock only — Cortexa-shaped examples, not your activity.";
export const USAGE_CATEGORIES = [
  "simulation",
  "openai_api",
  "anthropic_api",
  "lm_studio",
  "ollama",
  "codex",
] as const;
export type UsageCategory = (typeof USAGE_CATEGORIES)[number];
export type Metric = number | "missing" | "unavailable" | "unverified";
export type TokenCounts = readonly [Metric, Metric, Metric, Metric];
export interface UsageDay {
  readonly date: string;
  readonly tokens: TokenCounts;
  readonly total: Metric;
}
export interface UsageView {
  readonly category: UsageCategory;
  readonly days: 7 | 30;
  readonly example: "activity" | "empty";
  readonly start: string;
  readonly end: "2026-10-01";
  readonly entries: number;
  readonly completedTasks: number;
  readonly tokens: TokenCounts;
  readonly total: Metric;
  readonly cachePercent: Metric;
  readonly completedTaskMean: Metric;
  readonly daily: readonly UsageDay[];
}
export interface UsageProjection {
  readonly schemaVersion: 1;
  readonly provenance: "synthetic";
  readonly disclosure: typeof USAGE_DISCLOSURE;
  readonly views: readonly UsageView[];
}
export type UsageLoader = () => Promise<UsageProjection>;

function record(value: unknown, keys: readonly string[]): value is Record<string, unknown> {
  return (
    value !== null &&
    typeof value === "object" &&
    !Array.isArray(value) &&
    Object.keys(value).length === keys.length &&
    Object.keys(value).every((key) => keys.includes(key))
  );
}
function array(value: unknown): value is readonly unknown[] {
  return Array.isArray(value);
}
function integer(value: unknown, maximum = Number.MAX_SAFE_INTEGER): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0 && value <= maximum;
}
function metric(value: unknown, fraction = false): value is Metric {
  return (
    value === "missing" ||
    value === "unavailable" ||
    value === "unverified" ||
    (typeof value === "number" &&
      Number.isFinite(value) &&
      value >= 0 &&
      value <= Number.MAX_SAFE_INTEGER &&
      (fraction || Number.isSafeInteger(value)))
  );
}
function counts(value: unknown): TokenCounts | undefined {
  if (!array(value) || value.length !== 4) return undefined;
  const [input, cached, output, reasoning] = value;
  if (!metric(input) || !metric(cached) || !metric(output) || !metric(reasoning)) return undefined;
  if (
    (typeof cached === "number" && typeof input === "number" && cached > input) ||
    (typeof reasoning === "number" && typeof output === "number" && reasoning > output)
  )
    return undefined;
  return Object.freeze([input, cached, output, reasoning]);
}
function category(value: unknown): value is UsageCategory {
  return USAGE_CATEGORIES.some((candidate) => candidate === value);
}
function day(value: unknown, expected: string): UsageDay | undefined {
  if (
    !record(value, ["date", "tokens", "total"]) ||
    value["date"] !== expected ||
    !metric(value["total"])
  )
    return undefined;
  const tokens = counts(value["tokens"]);
  return tokens ? Object.freeze({ date: expected, tokens, total: value["total"] }) : undefined;
}
function view(value: unknown): UsageView | undefined {
  if (
    !record(value, [
      "category",
      "days",
      "example",
      "start",
      "end",
      "entries",
      "completedTasks",
      "tokens",
      "total",
      "cachePercent",
      "completedTaskMean",
      "daily",
    ])
  )
    return undefined;
  const c = value["category"],
    days = value["days"],
    example = value["example"];
  const entries = value["entries"],
    completedTasks = value["completedTasks"];
  const total = value["total"],
    cachePercent = value["cachePercent"],
    completedTaskMean = value["completedTaskMean"];
  const daily = value["daily"],
    tokens = counts(value["tokens"]);
  if (
    !category(c) ||
    (days !== 7 && days !== 30) ||
    (example !== "activity" && example !== "empty") ||
    !integer(entries, 128) ||
    !integer(completedTasks, entries) ||
    !tokens ||
    !metric(total) ||
    !metric(cachePercent, true) ||
    (typeof cachePercent === "number" && cachePercent > 100) ||
    !metric(completedTaskMean, true) ||
    !array(daily) ||
    daily.length !== (example === "empty" ? 0 : days)
  )
    return undefined;
  const start = `2026-09-${String(31 - days).padStart(2, "0")}`;
  if (value["start"] !== start || value["end"] !== "2026-10-01") return undefined;
  if (
    example === "empty" &&
    (entries !== 0 ||
      completedTasks !== 0 ||
      total !== 0 ||
      tokens.some((n) => n !== 0) ||
      cachePercent !== "unavailable" ||
      completedTaskMean !== "unavailable")
  )
    return undefined;
  const parsed: UsageDay[] = [];
  for (const [index, item] of daily.entries()) {
    const next = day(item, `2026-09-${String(31 - days + index).padStart(2, "0")}`);
    if (!next) return undefined;
    parsed.push(next);
  }
  return Object.freeze({
    category: c,
    days,
    example,
    start,
    end: "2026-10-01",
    entries,
    completedTasks,
    tokens,
    total,
    cachePercent,
    completedTaskMean,
    daily: Object.freeze(parsed),
  });
}
export function parseUsageProjection(value: unknown): UsageProjection | undefined {
  try {
    if (
      !record(value, ["schemaVersion", "provenance", "disclosure", "views"]) ||
      value["schemaVersion"] !== 1 ||
      value["provenance"] !== "synthetic" ||
      value["disclosure"] !== USAGE_DISCLOSURE ||
      !array(value["views"]) ||
      value["views"].length !== 24
    )
      return undefined;
    const views: UsageView[] = [],
      keys = new Set<string>();
    for (const item of value["views"]) {
      const next = view(item);
      if (!next) return undefined;
      const key = `${next.category}/${String(next.days)}/${next.example}`;
      if (keys.has(key)) return undefined;
      keys.add(key);
      views.push(next);
    }
    const projection: UsageProjection = Object.freeze({
      schemaVersion: 1,
      provenance: "synthetic",
      disclosure: USAGE_DISCLOSURE,
      views: Object.freeze(views),
    });
    if (new TextEncoder().encode(JSON.stringify(projection)).length > 64 * 1024) return undefined;
    return projection;
  } catch {
    return undefined;
  }
}
export async function fetchUsageProjection(): Promise<UsageProjection> {
  if (!isTauri()) throw new Error("Native preview unavailable.");
  try {
    const parsed = parseUsageProjection(
      await invoke<unknown>("get_usage_efficiency_demo_projection"),
    );
    if (parsed) return parsed;
  } catch {
    /* Never expose native details or fall back to a live source. */
  }
  throw new Error("Usage & Efficiency examples unavailable.");
}
