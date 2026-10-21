import { invoke } from "@tauri-apps/api/core";
import {
  AGENT_IDS,
  CONNECTIONS,
  EFFORTS,
  parseAgentChatSnapshot,
  parseAgentProfile,
  type AgentId,
  type AgentConnection,
  type AgentEffort,
  type AgentChatSnapshot,
  type AgentProfile,
} from "./agent-chat-client";

export interface SavedTurn {
  readonly id: number;
  readonly message: string;
  readonly answer: string;
  readonly status: "running" | "completed" | "cancelled" | "failed" | "interrupted";
  readonly failure: "timeout" | "limit" | "provider" | "storage" | null;
  readonly createdAtMs: number;
  readonly updatedAtMs: number;
}
export interface SavedConversation {
  readonly id: string;
  readonly agentId: AgentId;
  readonly title: string;
  readonly createdAtMs: number;
  readonly updatedAtMs: number;
  readonly connection: AgentConnection;
  readonly model: string;
  readonly endpoint: string;
  readonly effort: AgentEffort;
  readonly turns: readonly SavedTurn[];
  readonly contextTurnIds: readonly number[];
}
export interface SavedSummary {
  readonly id: string;
  readonly agentId: AgentId;
  readonly title: string;
  readonly updatedAtMs: number;
  readonly turns: number;
}
export interface NoteProvenance {
  readonly createdAtMs: number;
  readonly updatedAtMs: number;
  readonly sourceConversation: string | null;
  readonly sourceDeleted: boolean;
}
function object(v: unknown, keys: readonly string[]): Record<string, unknown> {
  if (
    !v ||
    typeof v !== "object" ||
    Array.isArray(v) ||
    Object.keys(v).length !== keys.length ||
    Object.keys(v).some((k) => !keys.includes(k))
  )
    throw new Error("protocol");
  return v as Record<string, unknown>;
}
function text(v: unknown, max: number): v is string {
  return typeof v === "string" && Array.from(v).length <= max;
}
function number(v: unknown): v is number {
  return typeof v === "number" && Number.isSafeInteger(v) && v >= 0;
}
function id(v: unknown): v is string {
  return typeof v === "string" && /^saved-chat-[1-9][0-9]{0,18}$/.test(v);
}
function includes<T extends string>(v: unknown, options: readonly T[]): v is T {
  return typeof v === "string" && options.some((o) => o === v);
}
export function parseSavedConversation(value: unknown): SavedConversation {
  const v = object(value, [
    "id",
    "agentId",
    "title",
    "createdAtMs",
    "updatedAtMs",
    "connection",
    "model",
    "endpoint",
    "effort",
    "turns",
    "contextTurnIds",
  ]);
  if (
    !id(v["id"]) ||
    !includes(v["agentId"], AGENT_IDS) ||
    !text(v["title"], 80) ||
    !v["title"].trim() ||
    !number(v["createdAtMs"]) ||
    !number(v["updatedAtMs"]) ||
    v["updatedAtMs"] < v["createdAtMs"] ||
    !includes(v["connection"], CONNECTIONS) ||
    !text(v["model"], 256) ||
    !text(v["endpoint"], 2048) ||
    !includes(v["effort"], EFFORTS) ||
    !Array.isArray(v["turns"]) ||
    v["turns"].length > 200
  )
    throw new Error("protocol");
  v["turns"].forEach((turn: unknown, index: number) => {
    const t = object(turn, [
      "id",
      "message",
      "answer",
      "status",
      "failure",
      "createdAtMs",
      "updatedAtMs",
    ]);
    if (
      t["id"] !== index + 1 ||
      !text(t["message"], 4096) ||
      !text(t["answer"], 8192) ||
      !includes(t["status"], ["running", "completed", "cancelled", "failed", "interrupted"]) ||
      !(
        t["failure"] === null || includes(t["failure"], ["timeout", "limit", "provider", "storage"])
      ) ||
      !number(t["createdAtMs"]) ||
      !number(t["updatedAtMs"]) ||
      t["updatedAtMs"] < t["createdAtMs"]
    )
      throw new Error("protocol");
  });
  const turns = v["turns"] as SavedTurn[];
  if (
    !Array.isArray(v["contextTurnIds"]) ||
    v["contextTurnIds"].length > 3 ||
    new Set(v["contextTurnIds"]).size !== v["contextTurnIds"].length ||
    v["contextTurnIds"].some(
      (id: unknown) => !number(id) || !turns.some((t) => t.id === id && t.status === "completed"),
    )
  )
    throw new Error("protocol");
  return value as SavedConversation;
}
function summaries(value: unknown, agent: AgentId): readonly SavedSummary[] {
  if (!Array.isArray(value) || value.length > 200) throw new Error("protocol");
  const ids = new Set<string>();
  for (const item of value) {
    const v = object(item, ["id", "agentId", "title", "updatedAtMs", "turns"]);
    if (
      !id(v["id"]) ||
      ids.has(v["id"]) ||
      v["agentId"] !== agent ||
      !text(v["title"], 80) ||
      !number(v["updatedAtMs"]) ||
      !number(v["turns"]) ||
      v["turns"] > 200
    )
      throw new Error("protocol");
    ids.add(v["id"]);
  }
  return value as readonly SavedSummary[];
}
export interface ConversationHistoryClient {
  list(agentId: AgentId): Promise<readonly SavedSummary[]>;
  read(conversationId: string): Promise<SavedConversation>;
  open(conversationId: string): Promise<AgentChatSnapshot>;
  rename(conversationId: string, title: string): Promise<void>;
  delete(conversationId: string): Promise<void>;
  fact(
    conversationId: string,
    turnId: number,
    text: string,
    revision: number,
  ): Promise<AgentProfile>;
  provenance(agentId: AgentId): Promise<NoteProvenance | null>;
}
export const conversationHistoryClient: ConversationHistoryClient = {
  list: async (agentId) =>
    summaries(
      await invoke<unknown>("list_saved_agent_conversations", { request: { agentId } }),
      agentId,
    ),
  read: async (conversationId) => {
    const c = parseSavedConversation(
      await invoke<unknown>("read_saved_agent_conversation", { request: { conversationId } }),
    );
    if (c.id !== conversationId) throw new Error("protocol");
    return c;
  },
  open: async (conversationId) => {
    const c = parseAgentChatSnapshot(
      await invoke<unknown>("open_saved_agent_conversation", { request: { conversationId } }),
    );
    if (c.conversationId !== conversationId) throw new Error("protocol");
    return c;
  },
  rename: async (conversationId, title) => {
    if (
      (await invoke<unknown>("rename_saved_agent_conversation", {
        request: { conversationId, title },
      })) !== null
    )
      throw new Error("protocol");
  },
  delete: async (conversationId) => {
    if (
      (await invoke<unknown>("delete_saved_agent_conversation", {
        request: { conversationId },
      })) !== null
    )
      throw new Error("protocol");
  },
  fact: async (conversationId, turnId, text, revision) =>
    parseAgentProfile(
      await invoke<unknown>("save_conversation_fact", {
        request: { conversationId, turnId, text, revision },
      }),
    ),
  provenance: async (agentId) => {
    const value = await invoke<unknown>("read_agent_note_provenance", { request: { agentId } });
    if (value === null) return null;
    const v = object(value, ["createdAtMs", "updatedAtMs", "sourceConversation", "sourceDeleted"]);
    if (
      !number(v["createdAtMs"]) ||
      !number(v["updatedAtMs"]) ||
      v["updatedAtMs"] < v["createdAtMs"] ||
      !(v["sourceConversation"] === null || id(v["sourceConversation"])) ||
      typeof v["sourceDeleted"] !== "boolean"
    )
      throw new Error("protocol");
    return value as NoteProvenance;
  },
};
