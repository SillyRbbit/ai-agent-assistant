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
export const CONNECTIONS = [
  "simulation",
  "openai_api",
  "codex",
  "anthropic_api",
  "lm_studio",
  "ollama",
] as const;
export type AgentConnection = (typeof CONNECTIONS)[number];
export const EFFORTS = ["default", "none", "low", "medium", "high", "xhigh", "max"] as const;
export type AgentEffort = (typeof EFFORTS)[number];
export const OPENAI_AGENT_MODELS = ["gpt-5.6-luna", "gpt-5.6-terra", "gpt-5.6-sol"] as const;
export type MemoryMode = "off" | "private_notes";
export interface AgentProfileInput {
  readonly agentId: AgentId;
  readonly connection: AgentConnection;
  readonly model: string;
  readonly effort: AgentEffort;
  readonly endpoint: string;
  readonly localAuth: boolean;
  readonly allowUnknownLocalityNotes: boolean;
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
  "provider_unavailable",
  "out_of_memory",
  "truncated",
  "unsupported",
  "endpoint",
  "locality",
  "catalog",
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
  readonly endpoint: string;
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
function modelId(value: unknown): value is string {
  return (
    bounded(value, 256) &&
    value !== "" &&
    Array.from(value).every((character) => {
      const code = character.charCodeAt(0);
      return code >= 32 && (code < 127 || code > 159);
    })
  );
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
    ((value["connection"] === "openai_api" &&
      value["effort"] !== "max" &&
      member(value["model"], OPENAI_AGENT_MODELS)) ||
      (value["connection"] === "anthropic_api" &&
        bounded(value["model"], 256) &&
        value["model"] !== "" &&
        value["effort"] !== "none") ||
      (isLocal(value["connection"]) &&
        bounded(value["model"], 256) &&
        value["effort"] === "default") ||
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
    "endpoint",
    "localAuth",
    "allowUnknownLocalityNotes",
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
    !bounded(v["endpoint"], 512) ||
    typeof v["localAuth"] !== "boolean" ||
    typeof v["allowUnknownLocalityNotes"] !== "boolean" ||
    (!isLocal(v["connection"]) &&
      (v["endpoint"] !== "" || v["localAuth"] || v["allowUnknownLocalityNotes"])) ||
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
    "endpoint",
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
    !bounded(v["endpoint"], 512) ||
    (!isLocal(v["connection"]) && v["endpoint"] !== "") ||
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
    case "disabled":
      return "This connection requires its native development opt-in before use.";
    case "missing_key":
      return "No valid native credential is available for the selected connection.";
    case "authentication":
      return "The selected service rejected authentication. Check its owner-only credential setup.";
    case "rate_limited":
      return "The selected provider rate or spending limit was reached. No automatic retry was made.";
    case "model_unavailable":
      return "The selected model is unavailable or not loaded. Check the selected server. No fallback was used.";
    case "provider_unavailable":
      return "The selected provider is unavailable. No fallback was used.";
    case "out_of_memory":
      return "The server reported insufficient memory. Choose a smaller already-installed model or context.";
    case "truncated":
      return "The response reached its output or context limit and is incomplete.";
    case "unsupported":
      return "This model or response mode is unsupported. No fallback was used.";
    case "endpoint":
      return "Use a credential-free loopback HTTP(S) endpoint ending in /v1.";
    case "locality":
      return "Locality is unknown. Acknowledge note transmission for this destination before sending.";
    case "catalog":
      return "Refresh the model catalog for this exact connection before sending.";
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
  readonly discover: (request: CatalogRequest) => Promise<ModelCatalog>;
  readonly save: (profile: AgentProfileInput) => Promise<AgentProfile>;
  readonly clearNote: (agentId: AgentId, revision: number) => Promise<AgentProfile>;
  readonly defaults: (agentId: AgentId, revision: number) => Promise<AgentProfile>;
  readonly start: (agentId: AgentId) => Promise<AgentChatSnapshot>;
  readonly send: (
    conversationId: string,
    message: string,
    acknowledgment:
      | "simulation"
      | "openai-agent-text-v1"
      | "anthropic-agent-text-v1"
      | "local-agent-text-v1",
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
  discover: async (request) => {
    const catalog = parseModelCatalog(await invoke<unknown>("discover_agent_models", { request }));
    if (catalog.connection !== request.connection) throw new Error("protocol");
    return catalog;
  },
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

export function isLocal(connection: unknown): boolean {
  return connection === "lm_studio" || connection === "ollama";
}
export interface ModelInfo {
  readonly id: string;
  readonly label: string;
  readonly evidence: "documented" | "discovered";
  readonly availability: "access_unknown" | "available" | "unsupported" | "retired";
  readonly efforts: readonly AgentEffort[];
  readonly thinking: "always_on" | "default" | "unknown" | "unsupported";
  readonly locality: "hosted" | "unknown" | "cloud";
  readonly contextLimit: number | null;
  readonly outputLimit: number | null;
  readonly sizeBytes: number | null;
  readonly quantization: string | null;
  readonly provenance: string | null;
  readonly loaded: boolean | null;
  readonly capabilities: unknown;
}
export interface CatalogRequest {
  readonly connection: AgentConnection;
  readonly endpoint: string;
  readonly localAuth: boolean;
}
export interface ModelCatalog {
  readonly connection: AgentConnection;
  readonly endpoint: string;
  readonly models: readonly ModelInfo[];
}
export const ANTHROPIC_DOCUMENTED_MODELS: readonly ModelInfo[] = [
  ["claude-fable-5-1", "Claude Fable 5.1", "always_on"],
  ["claude-opus-5-5", "Claude Opus 5.5", "always_on"],
  ["claude-sonnet-5", "Claude Sonnet 5", "default"],
  ["claude-haiku-4-5-20251001", "Claude Haiku 4.5", "default"],
  ["claude-opus-5", "Claude Opus 5 · active legacy", "default"],
].map(([id, label, thinking]) => ({
  id: id ?? "",
  label: label ?? "",
  thinking: thinking as ModelInfo["thinking"],
  evidence: "documented",
  availability: "access_unknown",
  locality: "hosted",
  efforts:
    id === "claude-haiku-4-5-20251001"
      ? ["default"]
      : ["default", "low", "medium", "high", "xhigh", "max"],
  contextLimit: null,
  outputLimit: null,
  sizeBytes: null,
  quantization: null,
  provenance: null,
  loaded: null,
  capabilities: null,
}));
export function parseModelCatalog(value: unknown): ModelCatalog {
  const catalog = record(value, ["connection", "endpoint", "models"]);
  if (
    !member(catalog["connection"], ["anthropic_api", "lm_studio", "ollama"]) ||
    !bounded(catalog["endpoint"], 512) ||
    (catalog["connection"] === "anthropic_api" && catalog["endpoint"] !== "") ||
    !Array.isArray(catalog["models"]) ||
    catalog["models"].length > 200
  )
    throw new Error("protocol");
  const models = catalog["models"].map((value: unknown) => {
    const m = record(value, [
      "id",
      "label",
      "evidence",
      "availability",
      "efforts",
      "thinking",
      "locality",
      "contextLimit",
      "outputLimit",
      "sizeBytes",
      "quantization",
      "provenance",
      "loaded",
      "capabilities",
    ]);
    if (
      !modelId(m["id"]) ||
      !bounded(m["label"], 256) ||
      !member(m["evidence"], ["documented", "discovered"]) ||
      !member(m["availability"], ["access_unknown", "available", "unsupported", "retired"]) ||
      !Array.isArray(m["efforts"]) ||
      m["efforts"].length === 0 ||
      new Set(m["efforts"]).size !== m["efforts"].length ||
      m["efforts"].length > EFFORTS.length ||
      !m["efforts"].every((e) => member(e, EFFORTS)) ||
      !member(m["thinking"], ["always_on", "default", "unknown", "unsupported"]) ||
      !member(m["locality"], ["hosted", "unknown", "cloud"]) ||
      !["contextLimit", "outputLimit", "sizeBytes"].every((k) => m[k] === null || integer(m[k])) ||
      !["quantization", "provenance"].every((k) => m[k] === null || bounded(m[k], 1024)) ||
      !(m["loaded"] === null || typeof m["loaded"] === "boolean") ||
      m["capabilities"] === undefined ||
      JSON.stringify(m["capabilities"]).length > 16384
    )
      throw new Error("protocol");
    return Object.freeze(m) as unknown as ModelInfo;
  });
  if (new Set(models.map((m) => m.id)).size !== models.length) throw new Error("protocol");
  return { connection: catalog["connection"], endpoint: catalog["endpoint"], models };
}
