import type { Metric, UsageDay } from "../../infrastructure/tauri/usage-efficiency-client";

const ABSENCE = {
  missing: "Missing",
  unavailable: "Unavailable",
  unverified: "Unverified",
} as const;

export function displayMetric(value: Metric, suffix = ""): string {
  return typeof value === "number"
    ? value.toLocaleString("en-US", { maximumFractionDigits: 2 }) + suffix
    : ABSENCE[value];
}

// Display geometry only. Rust supplies every token count and aggregate.
// A nonnumeric day ends a segment; it is neither zero nor a connecting line.
export function trendGeometry(daily: readonly UsageDay[]) {
  const ceiling = Math.max(
    1,
    ...daily.map((day) => (typeof day.total === "number" ? day.total : 0)),
  );
  const segments: { x: number; y: number; date: string; total: number }[][] = [];
  let segment: (typeof segments)[number] = [];
  for (const [index, day] of daily.entries()) {
    if (typeof day.total !== "number") {
      if (segment.length) segments.push(segment);
      segment = [];
      continue;
    }
    segment.push({
      x: 40 + (index * 620) / Math.max(1, daily.length - 1),
      y: 210 - (day.total / ceiling) * 170,
      date: day.date,
      total: day.total,
    });
  }
  if (segment.length) segments.push(segment);
  return { ceiling, segments };
}
