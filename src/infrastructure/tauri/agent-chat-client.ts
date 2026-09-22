import { invoke, isTauri } from "@tauri-apps/api/core";
import { directErrorMessage } from "./personal-assistant-direct-client";

export const AGENT_IDS = [
  "personal-assistant",
  "research",
  "coding",
  "cloud-infrastructure",
  "systems-operations",
  "knowledge-document",
  "qa-validation",
  "security-risk",
  "workflow-automation",
] as const;
export type AgentId = (typeof AGENT_IDS)[number];
export const CONNECTIONS = ["simulation", "openai_api", "codex"] as const;
export type AgentConnection = (typeof CONNECTIONS)[number];
export const EFFORTS = ["default", "none", "low", "medium", "high", "xhigh"] as const;
export type AgentEffort = (typeof EFFORTS)[number];
export const OPENAI_AGENT_MODELS = ["gpt-5.6-luna", "gpt-5.6-terra", "gpt-5.6-sol"] as const;
export type MemoryMode = "off" | "private_notes";
export interface AgentProfileInput {
  readonly agentId: AgentId;
  readonly connection: AgentConnection;
  readonly model: string;
  readonly effort: AgentEffort;
  readonly ownerInstructions: string;
  readonly memoryMode: MemoryMode;
  readonly note: string;
  readonly revision: number;
}
export interface AgentProfile extends AgentProfileInput {
  readonly displayName: string;
}
export interface AgentConnectionReadiness {
  readonly connection: AgentConnection;
  readonly status: "ready" | "owner_setup_required" | "blocked";
  readonly message: string;
}
export const AGENT_ERROR_CODES = [
  "invalid_request",
  "unsupported_settings",
  "stale_context",
  "storage",
  "busy",
  "unavailable",
  "codex_isolation",
  "disabled",
  "missing_key",
  "authentication",
  "model_unavailable",
  "rate_limited",
  "timeout",
  "network",
  "http_status",
  "http_bad_request",
  "http_forbidden",
  "http_not_found",

  "provider_stream_error_event",
  "provider_stream_failed_unknown_code",
  "provider_stream_failed_invalid_code",
  "provider_stream_server_error",
  "provider_stream_rate_limit",
  "provider_stream_invalid_prompt",
  "refused",
  "incomplete",
  "limit",
  "protocol",
  "internal",
] as const;
export type AgentChatError = (typeof AGENT_ERROR_CODES)[number];
export type AgentChatStatus = "idle" | "starting" | "streaming" | "completed" | "stopped" | "error";
export interface AgentChatSnapshot {
  readonly version: 1;
  readonly conversationId: string;
  readonly agentId: AgentId;
  readonly connection: AgentConnection;
  readonly model: string;
  readonly effort: AgentEffort;
  readonly memoryMode: MemoryMode;
  readonly settingsRevision: number;
  readonly status: AgentChatStatus;
  readonly text: string;
  readonly sequence: number;
  readonly busy: boolean;
  readonly error: AgentChatError | null;
}

function record(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value))
    throw new Error("protocol");
  const result = value as Record<string, unknown>;
  if (
    Object.keys(result).length !== keys.length ||
    Object.keys(result).some((key) => !keys.includes(key))
  )
    throw new Error("protocol");
  return result;
}
function member<T extends string>(value: unknown, values: readonly T[]): value is T {
  return typeof value === "string" && values.some((item) => item === value);
}
function bounded(value: unknown, limit: number): value is string {
  return (
    typeof value === "string" &&
    Array.from(value).length <= limit &&
    new TextEncoder().encode(value).length <= limit * 4
  );
}
function integer(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}
function effective(value: Record<string, unknown>): boolean {
  return (
    member(value["connection"], CONNECTIONS) &&
    member(value["effort"], EFFORTS) &&
    ((value["connection"] === "openai_api" && member(value["model"], OPENAI_AGENT_MODELS)) ||
      (value["connection"] === "simulation" &&
        value["model"] === "simulation" &&
        value["effort"] === "default") ||
      (value["connection"] === "codex" &&
        value["model"] === "unavailable" &&
        value["effort"] === "default"))
  );
}
export function parseAgentProfile(value: unknown): AgentProfile {
  const v = record(value, [
    "agentId",
    "displayName",
    "connection",
    "model",
    "effort",
    "ownerInstructions",
    "memoryMode",
    "note",
    "revision",
  ]);
  if (
    !member(v["agentId"], AGENT_IDS) ||
    !bounded(v["displayName"], 64) ||
    v["displayName"] === "" ||
    !effective(v) ||
    !bounded(v["ownerInstructions"], 4096) ||
    !bounded(v["note"], 8192) ||
    !member(v["memoryMode"], ["off", "private_notes"]) ||
    !integer(v["revision"])
  )
    throw new Error("protocol");
  return Object.freeze(v) as unknown as AgentProfile;
}
export function parseAgentProfiles(value: unknown): readonly AgentProfile[] {
  if (!Array.isArray(value) || value.length !== AGENT_IDS.length) throw new Error("protocol");
  const result = value.map(parseAgentProfile);
  if (new Set(result.map((profile) => profile.agentId)).size !== AGENT_IDS.length)
    throw new Error("protocol");
  return Object.freeze(result);
}
export function parseAgentConnections(value: unknown): readonly AgentConnectionReadiness[] {
  if (!Array.isArray(value) || value.length !== CONNECTIONS.length) throw new Error("protocol");
  const result = value.map((entry) => {
    const v = record(entry, ["connection", "status", "message"]);
    if (
      !member(v["connection"], CONNECTIONS) ||
      !member(v["status"], ["ready", "owner_setup_required", "blocked"]) ||
      !bounded(v["message"], 1024) ||
      (v["connection"] === "codex" && v["status"] !== "blocked")
    )
      throw new Error("protocol");
    return Object.freeze(v) as unknown as AgentConnectionReadiness;
  });
  if (new Set(result.map((entry) => entry.connection)).size !== CONNECTIONS.length)
    throw new Error("protocol");
  return Object.freeze(result);
}
export function parseAgentChatSnapshot(value: unknown): AgentChatSnapshot {
  const v = record(value, [
    "version",
    "conversationId",
    "agentId",
    "connection",
    "model",
    "effort",
    "memoryMode",
    "settingsRevision",
    "status",
    "text",
    "sequence",
    "busy",
    "error",
  ]);
  if (
    v["version"] !== 1 ||
    !bounded(v["conversationId"], 128) ||
    !/^[a-zA-Z0-9-]+$/.test(v["conversationId"]) ||
    !member(v["agentId"], AGENT_IDS) ||
    !effective(v) ||
    !member(v["memoryMode"], ["off", "private_notes"]) ||
    !integer(v["settingsRevision"]) ||
    !member(v["status"], ["idle", "starting", "streaming", "completed", "stopped", "error"]) ||
    !bounded(v["text"], 8192) ||
    !integer(v["sequence"]) ||
    v["sequence"] > 512 ||
    typeof v["busy"] !== "boolean" ||
    !(v["error"] === null || member(v["error"], AGENT_ERROR_CODES)) ||
    (v["status"] === "error") !== (v["error"] !== null) ||
    ((v["status"] === "starting" || v["status"] === "streaming") && !v["busy"]) ||
    (v["status"] === "idle" && (v["busy"] || v["text"] !== "" || v["sequence"] !== 0)) ||
    (v["status"] === "completed" && v["text"] === "")
  )
    throw new Error("protocol");
  return Object.freeze(v) as unknown as AgentChatSnapshot;
}
export function agentChatErrorMessage(error: unknown): string {
  const code = error instanceof Error ? error.message : error;
  switch (code) {
    case "model_unavailable":
      return "The selected model is unavailable or OpenAI rejected the request. No fallback was used.";
    case "limit":
      return "The conversation or response reached the bounded demo limit. Start a new conversation.";
    case "unsupported_settings":
      return "This connection does not support the selected model or reasoning effort.";
    case "stale_context":
      return "Saved context has changed. Reload settings and start a new conversation.";
    case "storage":
      return "Agent settings could not be read or saved. No automatic retry was made.";
    case "unavailable":
      return "Native agent conversations are unavailable in this session.";
    case "codex_isolation":
      return "Codex live is blocked: the installed runtime does not provide verified tool and file isolation. Authentication is unverified.";
    default:
      return member(code, AGENT_ERROR_CODES)
        ? directErrorMessage(code)
        : "The native session is unavailable.";
  }
}
export interface AgentChatClient {
  readonly available: () => boolean;
  readonly list: () => Promise<readonly AgentProfile[]>;
  readonly connections: () => Promise<readonly AgentConnectionReadiness[]>;
  readonly save: (profile: AgentProfileInput) => Promise<AgentProfile>;
  readonly clearNote: (agentId: AgentId, revision: number) => Promise<AgentProfile>;
  readonly defaults: (agentId: AgentId, revision: number) => Promise<AgentProfile>;
  readonly start: (agentId: AgentId) => Promise<AgentChatSnapshot>;
  readonly send: (
    conversationId: string,
    message: string,
    acknowledgment: "simulation" | "openai-agent-text-v1",
  ) => Promise<AgentChatSnapshot>;
  readonly poll: (conversationId: string) => Promise<AgentChatSnapshot>;
  readonly cancel: (conversationId: string) => Promise<AgentChatSnapshot>;
}
function profileFor(value: unknown, agentId: AgentId): AgentProfile {
  const profile = parseAgentProfile(value);
  if (profile.agentId !== agentId) throw new Error("protocol");
  return profile;
}
function conversationFor(value: unknown, conversationId: string): AgentChatSnapshot {
  const snapshot = parseAgentChatSnapshot(value);
  if (snapshot.conversationId !== conversationId) throw new Error("protocol");
  return snapshot;
}
export const agentChatClient: AgentChatClient = {
  available: () => isTauri(),
  list: async () => parseAgentProfiles(await invoke<unknown>("list_agent_preferences")),
  connections: async () => parseAgentConnections(await invoke<unknown>("list_agent_connections")),
  save: async (request) =>
    profileFor(await invoke<unknown>("save_agent_preferences", { request }), request.agentId),
  clearNote: async (agentId, revision) =>
    profileFor(
      await invoke<unknown>("clear_agent_note", { request: { agentId, revision } }),
      agentId,
    ),
  defaults: async (agentId, revision) =>
    profileFor(
      await invoke<unknown>("restore_agent_defaults", { request: { agentId, revision } }),
      agentId,
    ),
  start: async (agentId) => {
    const next = parseAgentChatSnapshot(
      await invoke<unknown>("start_agent_conversation", { request: { agentId } }),
    );
    if (next.agentId !== agentId || next.status !== "idle") throw new Error("protocol");
    return next;
  },
  send: async (conversationId, message, acknowledgment) =>
    conversationFor(
      await invoke<unknown>("send_agent_message", {
        request: { conversationId, message, acknowledgment },
      }),
      conversationId,
    ),
  poll: async (conversationId) =>
    conversationFor(
      await invoke<unknown>("poll_agent_conversation", { request: { conversationId } }),
      conversationId,
    ),
  cancel: async (conversationId) =>
    conversationFor(
      await invoke<unknown>("cancel_agent_conversation", { request: { conversationId } }),
      conversationId,
    ),
};
