import type { DiagnosticRecord } from "../../infrastructure/tauri/diagnostics-client";

export function diagnosticAdvice(error: DiagnosticRecord["error"]): string {
  switch (error) {
    case "missing_credentials":
      return "No usable native credential was available. Check owner-only private launch. Retrying unchanged will not help.";
    case "openai_invalid_api_key":
    case "authentication":
      return "Authentication was rejected. Check the owner-only credential/account setup. Do not retry unchanged.";
    case "denied_access":
      return "Access was denied; the reason is not known. Check project/model permission privately. Do not assume billing or retry unchanged.";
    case "openai_model_not_found":
    case "model_unavailable":
      return "The runtime reported the selected model unavailable. Check its catalog or loaded model; no fallback was used.";
    case "openai_credit_balance_exhausted":
      return "OpenAI reports no prepaid API credits remaining for this request's organization. Check the key's API organization billing privately; Work/Codex credits are separate. Do not retry unchanged or change billing automatically.";
    case "openai_insufficient_quota":
      return "The provider reported insufficient_quota. Check the API project quota/billing privately. Do not retry unchanged or change billing automatically.";
    case "openai_unsupported_value":
    case "openai_invalid_value":
      return "The provider rejected a request value. Use its separately recorded parameter category to check compatibility; do not substitute a model automatically.";
    case "openai_envelope_object":
      return "An error object was present inside the stream error event. Nested categories are separate observations; missing top-level fields do not establish a cause.";
    case "rate_limit":
    case "openai_top_level_rate_limit_exceeded":
      return "A rate or spending limit was reported. Check limits privately; a later explicitly authorized retry may help.";
    case "network":
      return "Connection or service availability failed. Check connectivity/service status. Authentication and billing are not established; an authorized later retry may help.";
    case "timeout":
      return "The local deadline expired. Check the last observed phase and runtime availability; a later authorized retry may help. Remote cancellation is unconfirmed.";
    case "malformed_stream":
      return "The response violated the expected protocol. Check runtime compatibility; retain this sanitized trace. Retry usefulness is unknown.";
    case "stream_error":
      return "The provider signaled a stream failure. Consult the closed code/parameter observations; the root cause may remain unknown. No automatic retry.";
    case "runtime_launch":
      return "Runtime setup or process launch failed. Check the installed executable and dedicated owner-authenticated home; do not capture raw stderr.";
    case "runtime":
      return "Runtime execution failed. Check its separately observed exit and cleanup events. Retry usefulness is unknown.";
    case "cleanup":
      return "Local runtime cleanup did not confirm success. Verify process ownership before another run; do not retry automatically.";
    case "configuration":
      return "Configuration, context or approval validation failed. Check saved profile, model, revision and acknowledgement before retrying.";
    case "storage":
      return "Local storage failed. Preserve data and check writable storage/space. Do not retry automatically.";
    case "interrupted":
      return "No terminal outcome was retained before restart. The outcome and cleanup are unknown; logs may be incomplete. Do not replay automatically.";
    case "resource_limit":
      return "Cortexa reached a bounded local response or event limit. Partial output is incomplete. Request a shorter response in a new conversation; no automatic retry was made.";
    case "incomplete":
      return "The response was incomplete, refused or truncated. Inspect the terminal UI and bounded output limit; do not assume success.";
    case "openai_top_level_server_error":
      return "The provider reported server_error. A later authorized retry may help; no automatic retry was made.";
    case "openai_top_level_invalid_prompt":
      return "The provider reported invalid_prompt. Check the request's supported settings privately; no prompt content was retained.";
    case "openai_top_level_param_model":
    case "openai_top_level_param_reasoning":
    case "openai_top_level_param_reasoning_effort":
    case "openai_top_level_param_max_output_tokens":
    case "openai_top_level_param_service_tier":
      return "The provider named this request field. Check its supported value and project permission; the parameter alone does not establish the cause.";
    case null:
      return "No classified error on this observation. Absence of an error is not proof of success or remote cleanup.";
    default:
      return "The safe category does not establish a specific cause. Inspect the last observed phase and owner-only prerequisites; retry usefulness is unknown.";
  }
}

export function troubleshootingSummary(events: readonly DiagnosticRecord[]): string {
  const groups = new Map<string, DiagnosticRecord[]>();
  for (const e of events) {
    const key = e.request ?? e.attempt ?? e.session;
    const list = groups.get(key) ?? [];
    list.push(e);
    groups.set(key, list);
  }
  return [...groups]
    .map(([id, records]) => {
      const last = records.at(-1);
      if (!last) return "";
      const terminal = records
        .slice()
        .reverse()
        .find((e) => e.event === "request_finished" || e.event === "request_interrupted");
      const phase = records
        .slice()
        .reverse()
        .find(
          (e) =>
            ![
              "request_finished",
              "stage_finished",
              "transport_released",
              "ui_terminal_returned",
              "ownership_released",
              "request_interrupted",
            ].includes(e.event),
        );
      const errors = records.filter((e) => e.error);
      return [
        `Request/correlation: ${id}`,
        `Version: ${last.appVersion}; runtime: ${last.provider ?? "application"}; model: ${last.model ?? "not recorded"}`,
        `Attempt: ${
          records
            .slice()
            .reverse()
            .find((e) => e.attempt)?.attempt ?? "no dispatch observed"
        }`,
        `Workflow: ${last.workflow ?? "none"}; stage: ${last.stage ?? "none"}`,
        `Last observed phase: ${phase?.event ?? "unknown"}; last event: ${last.event}`,
        `UTC: ${new Date(last.utcMs).toISOString()}; elapsed: ${String(terminal?.durationMs ?? last.durationMs ?? "unknown")} ms`,
        `Terminal outcome: ${terminal?.outcome ?? "not retained / not yet observed"}`,
        ...errors.map((e) => `${e.event}: ${e.error ?? "none"}. ${diagnosticAdvice(e.error)}`),
        terminal?.outcome === "cancelled"
          ? "Cancelled locally; this does not prove remote cancellation or zero billing."
          : "",
        "Bounded snapshot; missing events are not evidence of success. No automatic retry.",
      ]
        .filter(Boolean)
        .join("\n");
    })
    .join("\n\n");
}
