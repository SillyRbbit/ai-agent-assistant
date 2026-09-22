import { invoke, isTauri } from "@tauri-apps/api/core";

export const DIRECT_SAMPLE =
  "Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed.";
export const DIRECT_DISCLOSURE =
  "Send only this fixed synthetic sample to OpenAI using gpt-5.6-luna. This is a paid external request. No tools or personal content. store=false is not Zero Data Retention; provider abuse-monitoring and cache retention may still apply. Stop closes local output and aborts the request, but cannot guarantee immediate remote termination or zero billing.";
const ERRORS = {
  disabled:
    "Native live mode is disabled. Launch a development session with CORTEXA_OPENAI_DEMO=1.",
  missing_key: "No valid native session OPENAI_API_KEY is available.",
  authentication: "OpenAI rejected authentication. Check the owner-only API key setup.",
  model_unavailable:
    "The fixed model is unavailable or OpenAI rejected the request. No fallback was used.",
  http_bad_request: "OpenAI rejected the request (HTTP 400). No automatic retry was made.",
  http_forbidden: "OpenAI denied access (HTTP 403). No automatic retry was made.",
  http_not_found:
    "OpenAI could not find the requested resource (HTTP 404). No automatic retry was made.",
  rate_limited: "OpenAI rate or spending limit reached. No automatic retry was made.",
  timeout: "The request timed out. Partial output is incomplete.",
  network:
    "The provider connection failed while sending or receiving data. No mock response was substituted.",
  http_status:
    "The provider returned an unsuccessful HTTP response. No mock response was substituted.",
  provider_stream:
    "The provider reported a failure in the response stream. No mock response was substituted.",
  provider_stream_server_error:
    "The provider reported a server error in the response stream. No automatic retry was made.",
  provider_stream_rate_limit:
    "The provider reported a rate limit in the response stream. No automatic retry was made.",
  provider_stream_invalid_prompt:
    "The provider reported an invalid prompt in the response stream. No automatic retry was made.",
  provider_stream_error_event:
    "The provider emitted a top-level error event in the response stream. No automatic retry was made.",
  provider_stream_failed_unknown_code:
    "The provider reported response.failed with an unrecognized error code. No automatic retry was made.",
  provider_stream_failed_invalid_code:
    "The provider reported response.failed without a usable error code. No automatic retry was made.",
  refused: "The provider refused this request.",
  incomplete: "The stream ended without a complete answer.",
  limit: "The response exceeded the demo limit.",
  protocol: "The provider or native response was invalid.",
  busy: "A previous native request is still active or stopping.",
  invalid_request: "The native request or session handle is invalid.",
  internal: "The native session is unavailable.",
} as const;
export type DirectErrorCode = keyof typeof ERRORS;
export type DirectStatus = "idle" | "starting" | "streaming" | "completed" | "stopped" | "error";
export interface DirectSnapshot {
  readonly version: 1;
  readonly handle: string | null;
  readonly status: DirectStatus;
  readonly text: string;
  readonly sequence: number;
  readonly error: DirectErrorCode | null;
  readonly busy: boolean;
}
export const IDLE_DIRECT: DirectSnapshot = {
  version: 1,
  handle: null,
  status: "idle",
  text: "",
  sequence: 0,
  error: null,
  busy: false,
};
const fields = ["version", "handle", "status", "text", "sequence", "error", "busy"];
export function parseDirectSnapshot(value: unknown): DirectSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value))
    throw new Error("protocol");
  const v = value as Record<string, unknown>;
  if (
    Object.keys(v).length !== fields.length ||
    Object.keys(v).some((key) => !fields.includes(key)) ||
    v["version"] !== 1 ||
    !(
      v["handle"] === null ||
      (typeof v["handle"] === "string" && /^pa-v0-present-[0-9a-f]{16}$/.test(v["handle"]))
    ) ||
    typeof v["status"] !== "string" ||
    !["idle", "starting", "streaming", "completed", "stopped", "error"].includes(v["status"]) ||
    typeof v["text"] !== "string" ||
    Array.from(v["text"]).length > 8192 ||
    new TextEncoder().encode(v["text"]).length > 32768 ||
    typeof v["sequence"] !== "number" ||
    !Number.isSafeInteger(v["sequence"]) ||
    v["sequence"] < 0 ||
    v["sequence"] > 128 ||
    typeof v["busy"] !== "boolean" ||
    !(v["error"] === null || (typeof v["error"] === "string" && Object.hasOwn(ERRORS, v["error"])))
  )
    throw new Error("protocol");
  if (
    (v["status"] === "idle") !== (v["handle"] === null) ||
    (v["status"] === "idle" && (v["busy"] || v["text"] !== "" || v["sequence"] !== 0)) ||
    (v["status"] === "starting" && (!v["busy"] || v["sequence"] !== 0 || v["text"] !== "")) ||
    (v["status"] === "streaming" && !v["busy"]) ||
    (v["status"] === "completed" && (v["text"] === "" || v["sequence"] === 0)) ||
    (v["status"] === "error") !== (v["error"] !== null)
  )
    throw new Error("protocol");
  return Object.freeze({
    version: 1,
    handle: v["handle"],
    status: v["status"] as DirectStatus,
    text: v["text"],
    sequence: v["sequence"],
    error: v["error"] as DirectErrorCode | null,
    busy: v["busy"],
  });
}
export function directErrorMessage(error: unknown): string {
  const key = error instanceof Error ? error.message : error;
  return typeof key === "string" && Object.hasOwn(ERRORS, key)
    ? ERRORS[key as DirectErrorCode]
    : ERRORS.internal;
}
export interface DirectClient {
  readonly available: () => boolean;
  readonly start: () => Promise<DirectSnapshot>;
  readonly poll: (handle: string | null, cursor: number | null) => Promise<DirectSnapshot>;
  readonly cancel: (handle: string) => Promise<DirectSnapshot>;
}
export const directClient: DirectClient = {
  available: () => isTauri(),
  start: async () =>
    parseDirectSnapshot(
      await invoke<unknown>("start_personal_assistant_direct", {
        request: { version: 1, acknowledgment: "openai-synthetic-direct-v1" },
      }),
    ),
  poll: async (handle, cursor) =>
    parseDirectSnapshot(
      await invoke<unknown>("poll_personal_assistant_direct", { request: { handle, cursor } }),
    ),
  cancel: async (handle) =>
    parseDirectSnapshot(
      await invoke<unknown>("cancel_personal_assistant_direct", { request: { handle } }),
    ),
};
