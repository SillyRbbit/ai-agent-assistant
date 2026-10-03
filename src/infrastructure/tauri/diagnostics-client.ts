import { invoke, isTauri } from "@tauri-apps/api/core";

export const diagnosticEvents = [
  "application_started",
  "application_shutdown",
  "request_started",
  "configuration_validated",
  "credentials_available",
  "provider_dispatch",
  "transport_released",
  "ui_terminal_returned",
  "ownership_released",
  "request_interrupted",
  "first_response",
  "first_text",
  "openai_top_level_error",
  "openai_top_level_error_param",
  "request_finished",
  "runtime_started",
  "runtime_exited",
  "runtime_cleanup",
  "stage_started",
  "stage_finished",
  "storage_failure",
  "configuration_failure",
] as const;
export const diagnosticErrors = [
  "authentication",
  "missing_credentials",
  "model_unavailable",
  "runtime_launch",
  "cleanup",
  "interrupted",
  "denied_access",
  "rate_limit",
  "timeout",
  "network",
  "malformed_stream",
  "stream_error",
  "openai_top_level_server_error",
  "openai_top_level_rate_limit_exceeded",
  "openai_top_level_invalid_prompt",
  "openai_top_level_unknown_code",
  "openai_top_level_missing_code",
  "openai_top_level_absent_code",
  "openai_top_level_null_code",
  "openai_top_level_param_model",
  "openai_top_level_param_reasoning",
  "openai_top_level_param_reasoning_effort",
  "openai_top_level_param_max_output_tokens",
  "openai_top_level_param_service_tier",
  "openai_top_level_param_absent",
  "openai_top_level_param_null",
  "openai_top_level_param_invalid",
  "openai_top_level_param_unknown",

  "openai_top_level_invalid_code",
  "incomplete",
  "runtime",
  "configuration",
  "storage",
  "internal",
] as const;
export interface DiagnosticRecord {
  utcMs: number;
  severity: "info" | "warning" | "error";
  appVersion: string;
  event: (typeof diagnosticEvents)[number];
  outcome:
    | "observed"
    | "completed"
    | "failed"
    | "timed_out"
    | "cancelled"
    | "partial"
    | "interrupted";
  error: (typeof diagnosticErrors)[number] | null;
  session: string;
  conversation: string | null;
  attempt: string | null;
  request?: string;
  workflow: string | null;
  stage: string | null;
  provider: "simulation" | "openai_api" | "anthropic_api" | "lm_studio" | "ollama" | "codex" | null;
  model: string | null;
  durationMs: number | null;
}
export interface DiagnosticsSnapshot {
  available: boolean;
  events: DiagnosticRecord[];
}
const fields = [
  "utcMs",
  "severity",
  "appVersion",
  "event",
  "outcome",
  "error",
  "session",
  "conversation",
  "attempt",
  "workflow",
  "stage",
  "provider",
  "model",
  "durationMs",
];
const hash = (v: unknown): v is string => typeof v === "string" && /^sha256:[a-f0-9]{64}$/.test(v);
const integer = (v: unknown): v is number =>
  typeof v === "number" && Number.isSafeInteger(v) && v >= 0;
function object(v: unknown): Record<string, unknown> {
  if (!v || typeof v !== "object" || Array.isArray(v)) throw Error("Invalid diagnostic data.");
  return v as Record<string, unknown>;
}
export function parseDiagnostics(v: unknown): DiagnosticsSnapshot {
  const s = object(v);
  if (
    Object.keys(s).sort().join() !== "available,events" ||
    typeof s["available"] !== "boolean" ||
    !Array.isArray(s["events"]) ||
    s["events"].length > 256
  )
    throw Error("Invalid diagnostic data.");
  for (const raw of s["events"]) {
    const r = object(raw);
    if (
      Object.keys(r).length !== fields.length + ("request" in r ? 1 : 0) ||
      Object.keys(r).some((k) => k !== "request" && !fields.includes(k)) ||
      ("request" in r && !hash(r["request"])) ||
      !integer(r["utcMs"]) ||
      r["utcMs"] > 8640000000000000 ||
      !["info", "warning", "error"].includes(String(r["severity"])) ||
      r["appVersion"] !== "0.1.0" ||
      !diagnosticEvents.includes(r["event"] as never) ||
      ![
        "observed",
        "completed",
        "failed",
        "timed_out",
        "cancelled",
        "partial",
        "interrupted",
      ].includes(String(r["outcome"])) ||
      (r["error"] !== null && !diagnosticErrors.includes(r["error"] as never)) ||
      !hash(r["session"]) ||
      ["conversation", "attempt", "workflow", "stage"].some((k) => r[k] !== null && !hash(r[k])) ||
      (r["provider"] !== null &&
        !["simulation", "openai_api", "anthropic_api", "lm_studio", "ollama", "codex"].includes(
          typeof r["provider"] === "string" ? r["provider"] : "",
        )) ||
      (r["model"] !== null &&
        !hash(r["model"]) &&
        !["simulation", "gpt-5.6-luna", "gpt-5.6-terra", "gpt-5.6-sol"].includes(
          typeof r["model"] === "string" ? r["model"] : "",
        )) ||
      (r["durationMs"] !== null && !integer(r["durationMs"]))
    )
      throw Error("Invalid diagnostic data.");
  }
  return v as DiagnosticsSnapshot;
}
export const diagnosticsClient = {
  async read(): Promise<DiagnosticsSnapshot> {
    if (!isTauri()) throw Error("Diagnostics require the native app.");
    return parseDiagnostics(await invoke<unknown>("read_diagnostics"));
  },
  async export(): Promise<boolean> {
    if (!isTauri()) throw Error("Diagnostics require the native app.");
    const result = await invoke<unknown>("export_diagnostics");
    if (typeof result !== "boolean") throw Error("Invalid diagnostic data.");
    return result;
  },
};
