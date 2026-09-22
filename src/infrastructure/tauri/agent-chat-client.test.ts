import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  AGENT_IDS,
  ANTHROPIC_DOCUMENTED_MODELS,
  parseModelCatalog,
  AGENT_ERROR_CODES,
  agentChatClient,
  agentChatErrorMessage,
  parseAgentProfile,
  parseAgentProfiles,
  parseAgentConnections,
  parseAgentChatSnapshot,
} from "./agent-chat-client";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(), isTauri: vi.fn() }));
const profile = {
  agentId: "personal-assistant",
  displayName: "Personal Assistant",
  connection: "simulation",
  model: "simulation",
  effort: "default",
  endpoint: "",
  localAuth: false,
  allowUnknownLocalityNotes: false,
  ownerInstructions: "",
  memoryMode: "off",
  note: "",
  revision: 0,
} as const;
const profiles = AGENT_IDS.map((agentId) => ({ ...profile, agentId }));
const connections = [
  { connection: "simulation", status: "ready", message: "Simulation ready." },
  {
    connection: "openai_api",
    status: "owner_setup_required",
    message: "Native session key checked on Send.",
  },
  { connection: "anthropic_api", status: "owner_setup_required", message: "Native key setup." },
  { connection: "lm_studio", status: "owner_setup_required", message: "Connect server." },
  { connection: "ollama", status: "owner_setup_required", message: "Connect server." },
  { connection: "codex", status: "blocked", message: "Isolation unverified." },
];
const idle = {
  version: 1,
  conversationId: "agent-chat-1",
  agentId: "personal-assistant",
  connection: "simulation",
  model: "simulation",
  effort: "default",
  memoryMode: "off",
  endpoint: "",
  settingsRevision: 0,
  status: "idle",
  text: "",
  sequence: 0,
  busy: false,
  error: null,
} as const;

describe("agent chat native boundary", () => {
  beforeEach(() => vi.resetAllMocks());
  it("uses only narrow preference and conversation commands; captured context stays native", async () => {
    vi.mocked(isTauri).mockReturnValue(false);
    expect(agentChatClient.available()).toBe(false);
    vi.mocked(invoke)
      .mockResolvedValueOnce(profiles)
      .mockResolvedValueOnce(connections)
      .mockResolvedValueOnce(profile)
      .mockResolvedValueOnce(profile)
      .mockResolvedValueOnce(profile)
      .mockResolvedValue(idle);
    await agentChatClient.list();
    await agentChatClient.connections();
    const request = {
      agentId: profile.agentId,
      connection: profile.connection,
      model: profile.model,
      effort: profile.effort,
      endpoint: "",
      localAuth: false,
      allowUnknownLocalityNotes: false,
      ownerInstructions: "tone",
      memoryMode: profile.memoryMode,
      note: "local note",
      revision: 0,
    };
    await agentChatClient.save(request);
    await agentChatClient.clearNote(profile.agentId, 0);
    await agentChatClient.defaults(profile.agentId, 0);
    await agentChatClient.start(profile.agentId);
    await agentChatClient.send(idle.conversationId, "Synthetic request", "simulation");
    await agentChatClient.poll(idle.conversationId);
    await agentChatClient.cancel(idle.conversationId);
    expect(vi.mocked(invoke).mock.calls).toEqual([
      ["list_agent_preferences"],
      ["list_agent_connections"],
      ["save_agent_preferences", { request }],
      ["clear_agent_note", { request: { agentId: profile.agentId, revision: 0 } }],
      ["restore_agent_defaults", { request: { agentId: profile.agentId, revision: 0 } }],
      ["start_agent_conversation", { request: { agentId: profile.agentId } }],
      [
        "send_agent_message",
        {
          request: {
            conversationId: idle.conversationId,
            message: "Synthetic request",
            acknowledgment: "simulation",
          },
        },
      ],
      ["poll_agent_conversation", { request: { conversationId: idle.conversationId } }],
      ["cancel_agent_conversation", { request: { conversationId: idle.conversationId } }],
    ]);
  });
  it("requires all nine distinct canonical native identities and bounded closed profile fields", () => {
    expect(parseAgentProfiles(profiles)).toHaveLength(9);
    for (const value of [[], profiles.slice(1), [...profiles.slice(1), profiles[1]], { profiles }])
      expect(() => parseAgentProfiles(value)).toThrow("protocol");
    for (const value of [
      null,
      {},
      { ...profile, secret: "DUMMY" },
      { ...profile, agentId: "unknown" },
      { ...profile, note: "x".repeat(8193) },
      { ...profile, ownerInstructions: "x".repeat(4097) },
      { ...profile, revision: -1 },
      { ...profile, effort: "high" },
      { ...profile, model: "arbitrary" },
      { ...profile, memoryMode: "automatic" },
    ])
      expect(() => parseAgentProfile(value)).toThrow("protocol");
    expect(
      parseAgentProfile({
        ...profile,
        connection: "openai_api",
        model: "gpt-5.6-luna",
        effort: "xhigh",
      }).effort,
    ).toBe("xhigh");
    expect(
      parseAgentProfile({
        ...profile,
        connection: "openai_api",
        model: "gpt-5.6-sol",
        effort: "low",
      }).model,
    ).toBe("gpt-5.6-sol");
    for (const model of ["gpt-5.4-mini", "gpt-5.4-nano", "gpt-6-astra"])
      expect(() =>
        parseAgentProfile({ ...profile, connection: "openai_api", model, effort: "low" }),
      ).toThrow("protocol");
    expect(
      parseAgentProfile({ ...profile, connection: "codex", model: "unavailable" }).connection,
    ).toBe("codex");
  });
  it("rejects responses bound to another agent or conversation", async () => {
    vi.mocked(invoke).mockResolvedValue({ ...profile, agentId: "research" });
    await expect(agentChatClient.clearNote("personal-assistant", 0)).rejects.toThrow("protocol");
    vi.mocked(invoke).mockResolvedValue({ ...idle, agentId: "research" });
    await expect(agentChatClient.start("personal-assistant")).rejects.toThrow("protocol");
    vi.mocked(invoke).mockResolvedValue({ ...idle, conversationId: "agent-chat-2" });
    await expect(agentChatClient.poll("agent-chat-1")).rejects.toThrow("protocol");
  });
  it("fails closed on unsupported Codex readiness and malformed connection data", () => {
    expect(parseAgentConnections(connections)).toHaveLength(6);
    for (const value of [
      [],
      [...connections.slice(0, 2), { connection: "codex", status: "ready", message: "DUMMY" }],
      [...connections.slice(0, 2), connections[0]],
      connections.map((entry) => ({ ...entry, key: "DUMMY" })),
    ])
      expect(() => parseAgentConnections(value)).toThrow("protocol");
  });
  it.each([
    ["http_bad_request", "OpenAI rejected the request (HTTP 400). No automatic retry was made."],
    ["http_forbidden", "OpenAI denied access (HTTP 403). No automatic retry was made."],
    [
      "http_not_found",
      "OpenAI could not find the requested resource (HTTP 404). No automatic retry was made.",
    ],
  ])("preserves the closed %s status without raw provider details", (code, message) => {
    expect(parseAgentChatSnapshot({ ...idle, status: "error", error: code }).error).toBe(code);
    expect(agentChatErrorMessage(code)).toBe(message);
    for (const error of [`${code}: DUMMY-PRIVATE-CONTENT`, { code, message: "DUMMY" }]) {
      expect(() => parseAgentChatSnapshot({ ...idle, status: "error", error })).toThrow("protocol");
      expect(agentChatErrorMessage(error)).toBe("The native session is unavailable.");
    }
  });
  it("validates bounded snapshots and lifecycle consistency without exposing raw errors", () => {
    expect(parseAgentChatSnapshot(idle)).toEqual(idle);
    const completed = { ...idle, status: "completed", text: "Answer", sequence: 2 };
    expect(parseAgentChatSnapshot(completed).text).toBe("Answer");
    for (const value of [
      null,
      {},
      { ...idle, extra: "DUMMY" },
      { ...idle, text: "x" },
      { ...idle, status: "streaming" },
      { ...idle, status: "completed" },
      { ...idle, sequence: 513 },
      { ...completed, text: "x".repeat(8193) },
      { ...idle, conversationId: "../DUMMY" },
      { ...idle, error: { code: "DUMMY" } },
      { ...idle, status: "error", error: "DUMMY" },
    ])
      expect(() => parseAgentChatSnapshot(value)).toThrow("protocol");
    for (const code of AGENT_ERROR_CODES) {
      expect(parseAgentChatSnapshot({ ...idle, status: "error", error: code }).error).toBe(code);
      expect(agentChatErrorMessage(code)).not.toContain("DUMMY");
    }
    for (const failure of ["DUMMY secret", new Error("DUMMY secret"), { message: "DUMMY secret" }])
      expect(agentChatErrorMessage(failure)).toBe("The native session is unavailable.");
  });
});

describe("provider catalog boundary", () => {
  const model = ANTHROPIC_DOCUMENTED_MODELS[0];
  it("retains exact discovered IDs and bounded capabilities but rejects malformed metadata", () => {
    const catalog = {
      connection: "anthropic_api",
      endpoint: "",
      models: [
        {
          ...model,
          id: "claude-exact-fixture",
          evidence: "discovered",
          capabilities: { effort: { supported: true } },
        },
      ],
    };
    expect(parseModelCatalog(catalog).models[0]?.id).toBe("claude-exact-fixture");
    for (const bad of [
      { ...catalog, models: [...catalog.models, ...catalog.models] },
      { ...catalog, apiKey: "fixture" },
      { ...catalog, models: [{ ...model, efforts: ["invented"] }] },
      { ...catalog, models: [{ ...model, locality: "local-guaranteed" }] },
    ])
      expect(() => parseModelCatalog(bad)).toThrow("protocol");
  });
  it("issues discovery only explicitly without note, prompt or credential payload", async () => {
    const request = {
      connection: "ollama" as const,
      endpoint: "http://127.0.0.1:11434/v1",
      localAuth: false,
    };
    vi.mocked(invoke).mockResolvedValue({
      connection: request.connection,
      endpoint: request.endpoint,
      models: [],
    });
    await agentChatClient.discover(request);
    expect(invoke).toHaveBeenLastCalledWith("discover_agent_models", { request });
  });
  it("accepts bound local settings and rejects hosted endpoint or OpenAI max effort", () => {
    expect(
      parseAgentProfile({
        ...profile,
        connection: "lm_studio",
        model: "owner/model:q4",
        endpoint: "http://127.0.0.1:1234/v1",
      }).model,
    ).toBe("owner/model:q4");
    expect(() => parseAgentProfile({ ...profile, endpoint: "http://127.0.0.1/v1" })).toThrow(
      "protocol",
    );
    expect(() =>
      parseAgentProfile({
        ...profile,
        connection: "openai_api",
        model: "gpt-5.6-luna",
        effort: "max",
      }),
    ).toThrow("protocol");
  });
});
