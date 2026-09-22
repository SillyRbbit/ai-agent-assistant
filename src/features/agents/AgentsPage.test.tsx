import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { AgentsPage } from "./AgentsPage";
import {
  AGENT_IDS,
  type AgentChatSnapshot,
  type AgentConnectionReadiness,
  type AgentProfile,
  type AgentProfileInput,
} from "../../infrastructure/tauri/agent-chat-client";

const names = [
  "Personal Assistant",
  "Research Agent",
  "Coding Agent",
  "Cloud Infrastructure Agent",
  "Systems Operations Agent",
  "Knowledge & Document Agent",
  "QA & Validation Agent",
  "Security & Risk Agent",
  "Workflow Automation Agent",
];
const profiles: readonly AgentProfile[] = AGENT_IDS.map((agentId, index) => ({
  agentId,
  displayName: names[index] ?? agentId,
  connection: "simulation",
  model: "simulation",
  effort: "default",
  ownerInstructions: "",
  memoryMode: "off",
  note: "",
  revision: 0,
}));
const connections: readonly AgentConnectionReadiness[] = [
  { connection: "simulation", status: "ready", message: "Simulation ready." },
  {
    connection: "openai_api",
    status: "owner_setup_required",
    message: "Native session key checked on Send.",
  },
  { connection: "codex", status: "blocked", message: "Codex isolation unverified." },
];
const idle: AgentChatSnapshot = {
  version: 1,
  conversationId: "agent-chat-1",
  agentId: "personal-assistant",
  connection: "simulation",
  model: "simulation",
  effort: "default",
  memoryMode: "off",
  settingsRevision: 0,
  status: "idle",
  text: "",
  sequence: 0,
  busy: false,
  error: null,
};
const starting: AgentChatSnapshot = { ...idle, status: "starting", busy: true };
const completed: AgentChatSnapshot = {
  ...idle,
  status: "completed",
  text: "Simulated: <script>escaped answer</script>",
  sequence: 2,
};
const stopped: AgentChatSnapshot = { ...idle, status: "stopped", sequence: 1 };
function harness() {
  return {
    available: () => true,
    list: vi.fn().mockResolvedValue(profiles),
    connections: vi.fn().mockResolvedValue(connections),
    save: vi
      .fn<(profile: AgentProfileInput) => Promise<AgentProfile>>()
      .mockImplementation((profile) =>
        Promise.resolve({
          ...profile,
          displayName:
            profiles.find((entry) => entry.agentId === profile.agentId)?.displayName ?? "Agent",
          revision: profile.revision + 1,
        }),
      ),
    clearNote: vi.fn().mockResolvedValue({ ...profiles[0], note: "", revision: 1 }),
    defaults: vi.fn().mockResolvedValue({ ...profiles[0], revision: 1 }),
    start: vi.fn().mockResolvedValue(idle),
    send: vi.fn().mockResolvedValue(starting),
    poll: vi.fn().mockResolvedValue(completed),
    cancel: vi.fn().mockResolvedValue(stopped),
  };
}
async function ready() {
  await screen.findByRole("heading", { name: "Personal Assistant" });
}
async function openConversation() {
  fireEvent.click(screen.getByRole("button", { name: "Start conversation" }));
  await screen.findByRole("heading", { name: "Conversation · Personal Assistant" });
}
describe("agent configuration and text conversations", () => {
  afterEach(() => vi.useRealTimers());
  it("shows a truthful native prerequisite in browser mode and sends nothing on mount", () => {
    const client = harness();
    render(<AgentsPage client={{ ...client, available: () => false }} />);
    expect(screen.getByText(/Browser mode cannot save/)).toBeInTheDocument();
    expect(client.list).not.toHaveBeenCalled();
    expect(client.send).not.toHaveBeenCalled();
  });
  it("loads nine native agents and saves only the selected agent's settings and note", async () => {
    const client = harness();
    render(<AgentsPage client={client} />);
    await ready();
    for (const name of names)
      expect(
        screen.getByRole("button", { name: new RegExp(name.replaceAll("&", "&")) }),
      ).toBeInTheDocument();
    fireEvent.change(screen.getByLabelText(/Owner instructions/), {
      target: { value: "Keep it concise." },
    });
    fireEvent.change(screen.getByLabelText(/Private note/), {
      target: { value: "Synthetic preference: three bullets." },
    });
    fireEvent.change(screen.getByLabelText("Memory mode"), { target: { value: "private_notes" } });
    expect(screen.getByRole("button", { name: "Start conversation" })).toBeDisabled();
    fireEvent.click(screen.getByRole("button", { name: "Save settings and note" }));
    await screen.findByText("Saved locally · revision 1");
    expect(client.save).toHaveBeenCalledWith(
      expect.objectContaining({
        agentId: "personal-assistant",
        ownerInstructions: "Keep it concise.",
        note: "Synthetic preference: three bullets.",
        memoryMode: "private_notes",
        revision: 0,
      }),
    );
    fireEvent.click(screen.getByRole("button", { name: /Research Agent/ }));
    expect(screen.getByLabelText(/Private note/)).toHaveValue("");
    expect(screen.getByLabelText(/Owner instructions/)).toHaveValue("");
    fireEvent.click(screen.getByRole("button", { name: /Personal Assistant/ }));
    expect(screen.getByLabelText(/Private note/)).toHaveValue(
      "Synthetic preference: three bullets.",
    );
    expect(client.send).not.toHaveBeenCalled();
  });
  it("requires explicit note deletion confirmation and keeps restore behavior native", async () => {
    const client = harness();
    client.list.mockResolvedValue([{ ...profiles[0], note: "Owned note" }, ...profiles.slice(1)]);
    render(<AgentsPage client={client} />);
    await ready();
    fireEvent.click(screen.getByRole("button", { name: "Clear saved note…" }));
    expect(client.clearNote).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Keep note" }));
    expect(client.clearNote).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Clear saved note…" }));
    fireEvent.click(screen.getByRole("button", { name: "Confirm clear note" }));
    await waitFor(() => {
      expect(screen.getByLabelText(/Private note/)).toHaveValue("");
    });
    expect(client.clearNote).toHaveBeenCalledWith("personal-assistant", 0);
    fireEvent.click(screen.getByRole("button", { name: "Restore defaults" }));
    await waitFor(() => {
      expect(client.defaults).toHaveBeenCalledWith("personal-assistant", 1);
    });
  });
  it("offers supported API efforts, keeps Codex disabled, and reports stale revision without retry", async () => {
    const client = harness();
    client.save.mockRejectedValue("stale_context");
    render(<AgentsPage client={client} />);
    await ready();
    fireEvent.change(screen.getByLabelText("Connection"), { target: { value: "openai_api" } });
    expect(screen.getByLabelText("Model")).toHaveValue("gpt-5.6-luna");
    expect(screen.getByRole("option", { name: "gpt-5.6-terra" })).toBeInTheDocument();
    expect(screen.getByRole("option", { name: "gpt-5.6-sol" })).toBeInTheDocument();
    expect(screen.queryByRole("option", { name: /gpt-5\.4/ })).not.toBeInTheDocument();
    expect(screen.getByRole("option", { name: "xhigh" })).toBeInTheDocument();
    fireEvent.change(screen.getByLabelText("Reasoning effort"), { target: { value: "high" } });
    fireEvent.click(screen.getByRole("button", { name: "Save settings and note" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Saved context has changed");
    expect(client.save).toHaveBeenCalledTimes(1);
    fireEvent.change(screen.getByLabelText("Connection"), { target: { value: "codex" } });
    expect(screen.getByText(/Authentication is unverified/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Start conversation" })).toBeDisabled();
    expect(screen.queryByRole("option", { name: "xhigh" })).not.toBeInTheDocument();
  });
  it("saves the selected Sol model and effort for a new native conversation", async () => {
    const client = harness();
    client.start.mockResolvedValue({
      ...idle,
      connection: "openai_api",
      model: "gpt-5.6-sol",
      effort: "low",
      settingsRevision: 1,
    });
    render(<AgentsPage client={client} />);
    await ready();
    fireEvent.change(screen.getByLabelText("Connection"), { target: { value: "openai_api" } });
    fireEvent.change(screen.getByLabelText("Model"), { target: { value: "gpt-5.6-sol" } });
    fireEvent.change(screen.getByLabelText("Reasoning effort"), { target: { value: "low" } });
    fireEvent.click(screen.getByRole("button", { name: "Save settings and note" }));
    await screen.findByText("Saved locally · revision 1");
    expect(client.save).toHaveBeenCalledWith(
      expect.objectContaining({
        agentId: "personal-assistant",
        connection: "openai_api",
        model: "gpt-5.6-sol",
        effort: "low",
      }),
    );
    await openConversation();
    expect(screen.getByText(/gpt-5.6-sol · effort low/)).toBeInTheDocument();
    expect(client.send).not.toHaveBeenCalled();
  });
  it("streams simulated output to explicit completion without automatic sends and permits a bounded followup", async () => {
    const client = harness();
    render(<AgentsPage client={client} />);
    await ready();
    await openConversation();
    expect(client.send).not.toHaveBeenCalled();
    fireEvent.change(screen.getByLabelText("Message"), {
      target: { value: "First synthetic turn" },
    });
    vi.useFakeTimers();
    client.poll
      .mockResolvedValueOnce({ ...starting, status: "streaming", text: "Simulated:", sequence: 1 })
      .mockResolvedValueOnce(completed);
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Send simulated message" }));
      await Promise.resolve();
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    expect(screen.getByText("Partial output · incomplete")).toBeInTheDocument();
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    expect(screen.getByText("Simulated answer · completed")).toBeInTheDocument();
    expect(screen.getByText(/completed · no active native generation/)).toBeInTheDocument();
    expect(document.querySelector("script")).toBeNull();
    expect(client.send).toHaveBeenCalledTimes(1);
    fireEvent.change(screen.getByLabelText("Message"), {
      target: { value: "Second synthetic turn" },
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Send simulated message" }));
      await Promise.resolve();
    });
    expect(screen.getByText("Completed turn 1 · simulated")).toBeInTheDocument();
    expect(client.send).toHaveBeenCalledTimes(2);
  });
  it("requires paid acknowledgement and presents provider failure without simulation fallback", async () => {
    const client = harness();
    client.list.mockResolvedValue([
      { ...profiles[0], connection: "openai_api", model: "gpt-5.6-luna" },
      ...profiles.slice(1),
    ]);
    const api = { ...idle, connection: "openai_api", model: "gpt-5.6-luna" } as const;
    client.start.mockResolvedValue(api);
    client.send.mockResolvedValue({
      ...api,
      status: "error",
      sequence: 1,
      error: "provider_stream_error_event",
    });
    render(<AgentsPage client={client} />);
    await ready();
    await openConversation();
    expect(screen.getByText(/store=false is not Zero Data Retention/)).toBeInTheDocument();
    fireEvent.change(screen.getByLabelText("Message"), {
      target: { value: "Synthetic board update" },
    });
    expect(screen.getByRole("button", { name: "Send to OpenAI" })).toBeDisabled();
    fireEvent.click(screen.getByRole("checkbox"));
    fireEvent.click(screen.getByRole("button", { name: "Send to OpenAI" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("top-level error event");
    expect(client.send).toHaveBeenCalledExactlyOnceWith(
      "agent-chat-1",
      "Synthetic board update",
      "openai-agent-text-v1",
    );
    expect(screen.queryByText("Simulated answer · completed")).not.toBeInTheDocument();
    expect(client.poll).not.toHaveBeenCalled();
  });
  it("cancels an in-flight send on Stop without a duplicate generation", async () => {
    const client = harness();
    let finish!: (value: AgentChatSnapshot) => void;
    client.send.mockReturnValue(
      new Promise<AgentChatSnapshot>((resolve) => {
        finish = resolve;
      }),
    );
    render(<AgentsPage client={client} />);
    await ready();
    await openConversation();
    fireEvent.change(screen.getByLabelText("Message"), {
      target: { value: "Synthetic cancellation" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Send simulated message" }));
    fireEvent.click(screen.getByRole("button", { name: "Stop generation" }));
    await act(async () => {
      finish(starting);
      await Promise.resolve();
    });
    expect(client.cancel).toHaveBeenCalledExactlyOnceWith("agent-chat-1");
    expect(client.send).toHaveBeenCalledTimes(1);
    expect(screen.getByText(/stopped · no active native generation/)).toBeInTheDocument();
  });
  it("rejects another agent's snapshot and requests cancellation", async () => {
    const client = harness();
    render(<AgentsPage client={client} />);
    await ready();
    await openConversation();
    fireEvent.change(screen.getByLabelText("Message"), {
      target: { value: "Synthetic isolation" },
    });
    vi.useFakeTimers();
    client.poll.mockResolvedValue({
      ...completed,
      agentId: "research",
      text: "OTHER AGENT CONTENT",
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Send simulated message" }));
      await Promise.resolve();
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    expect(screen.getByRole("alert")).toHaveTextContent("invalid");
    expect(screen.queryByText("OTHER AGENT CONTENT")).not.toBeInTheDocument();
    expect(client.cancel).toHaveBeenCalledTimes(1);
  });
  it("cancels native ownership when an outstanding send resolves after unmount", async () => {
    const client = harness();
    let finish!: (value: AgentChatSnapshot) => void;
    client.send.mockReturnValue(
      new Promise<AgentChatSnapshot>((resolve) => {
        finish = resolve;
      }),
    );
    const view = render(<AgentsPage client={client} />);
    await ready();
    await openConversation();
    fireEvent.change(screen.getByLabelText("Message"), { target: { value: "Synthetic unmount" } });
    fireEvent.click(screen.getByRole("button", { name: "Send simulated message" }));
    view.unmount();
    await act(async () => {
      finish(starting);
      await Promise.resolve();
    });
    expect(client.cancel).toHaveBeenCalledExactlyOnceWith("agent-chat-1");
    expect(client.send).toHaveBeenCalledTimes(1);
  });
  it("does not claim released ownership after a failed send and unconfirmed cleanup", async () => {
    const client = harness();
    client.send.mockRejectedValue("network");
    client.cancel.mockRejectedValue("internal");
    render(<AgentsPage client={client} />);
    await ready();
    await openConversation();
    fireEvent.change(screen.getByLabelText("Message"), {
      target: { value: "Synthetic failed request" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Send simulated message" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("connection failed");
    await screen.findByText(/native cleanup unconfirmed/);
    expect(screen.queryByText(/no active native generation/)).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Send simulated message" })).toBeDisabled();
    expect(client.send).toHaveBeenCalledTimes(1);
    expect(client.cancel).toHaveBeenCalledTimes(1);
  });
});
