import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { VoiceSnapshot } from "../../application/voice";
import { VoiceControls } from "./VoiceControls";
const base: VoiceSnapshot = {
  version: 1,
  conversationId: "conversation-1",
  sessionId: "abc-123",
  sequence: 0,
  status: "starting",
  busy: true,
  text: null,
  error: null,
};
function harness() {
  const client = {
    available: () => true,
    start: vi
      .fn()
      .mockImplementation((_id: string, operation: string) =>
        Promise.resolve(
          operation === "authorize"
            ? { ...base, status: "authorized", busy: false, sequence: 1 }
            : { ...base, status: "recording", sequence: 1 },
        ),
      ),
    poll: vi.fn().mockResolvedValue({
      ...base,
      sequence: 2,
      status: "review",
      busy: false,
      text: "Synthetic phrase.",
    }),
    control: vi.fn().mockImplementation((previous: VoiceSnapshot) =>
      Promise.resolve({
        ...previous,
        sequence: previous.sequence + 1,
        status: "cancelled",
        busy: false,
        text: null,
        error: null,
      }),
    ),
  };
  return { client, onBusy: vi.fn(), onUseTranscript: vi.fn().mockReturnValue(true) };
}
async function enable() {
  await act(async () => {
    fireEvent.click(screen.getByRole("button", { name: "Enable voice" }));
    await Promise.resolve();
  });
}
async function advance() {
  await act(async () => {
    await Promise.resolve();
    await vi.advanceTimersByTimeAsync(300);
  });
}
describe("foreground voice controls", () => {
  it("keeps controls blocked after an explicit cleanup error even after process exit", async () => {
    const h = harness();
    h.client.start.mockResolvedValueOnce({
      ...base,
      status: "error",
      error: "cleanup_failed",
      busy: false,
      sequence: 1,
    });
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={true} />,
    );
    await enable();
    expect(screen.getByRole("alert").textContent).toContain("cleanup is unconfirmed");
    expect(screen.getByRole("button", { name: "Enable voice" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Read answer" })).toBeDisabled();
    expect(h.onBusy).toHaveBeenLastCalledWith(true);
    expect(h.client.control).not.toHaveBeenCalled();
  });

  it("shows the validated terminal diagnostic after bounded cleanup without inventing capture success", async () => {
    const h = harness();
    h.client.start.mockResolvedValueOnce({
      ...base,
      status: "error",
      error: "protocol",
      busy: true,
      sequence: 1,
    });
    h.client.control.mockResolvedValueOnce({
      ...base,
      status: "error",
      error: "helper_diagnostics",
      busy: false,
      sequence: 2,
    });
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    await enable();
    expect(screen.getByRole("alert")).toHaveTextContent("diagnostic output");
    expect(screen.getByRole("button", { name: "Enable voice" })).toBeEnabled();
    expect(h.onBusy).toHaveBeenLastCalledWith(false);
    expect(h.onUseTranscript).not.toHaveBeenCalled();
    expect(h.client.control).toHaveBeenCalledTimes(1);
  });

  it("shows only fixed permission stages without recording or auto Send", async () => {
    const h = harness();
    h.client.start.mockResolvedValueOnce({ ...base, status: "checking_microphone", sequence: 1 });
    h.client.poll.mockResolvedValueOnce({ ...base, status: "checking_speech", sequence: 2 });
    h.client.poll.mockResolvedValueOnce({
      ...base,
      status: "authorized",
      sequence: 3,
      busy: false,
    });
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    await enable();
    expect(screen.getByRole("status").textContent).toBe("Checking microphone permission…");
    await advance();
    expect(screen.getByRole("status").textContent).toBe(
      "Microphone allowed. Checking speech permission…",
    );
    await advance();
    expect(screen.getByRole("button", { name: "Start dictation" })).toBeEnabled();
    expect(h.client.start).toHaveBeenCalledTimes(1);
    expect(h.onUseTranscript).not.toHaveBeenCalled();
  });

  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());
  it("never captures on mount, and unsupported browser mode leaves typed chat available", () => {
    const h = harness();
    const view = render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    expect(h.client.start).not.toHaveBeenCalled();
    expect(screen.queryByLabelText("Transcript to review")).not.toBeInTheDocument();
    view.rerender(
      <VoiceControls
        {...h}
        client={{ ...h.client, available: () => false }}
        conversationId="conversation-1"
        disabled={false}
        canRead={false}
      />,
    );
    expect(screen.getByText(/Open Cortexa on a supported Mac/)).toBeInTheDocument();
  });
  it("requires explicit enable, review and Use transcript; editing never sends automatically", async () => {
    const h = harness();
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    expect(screen.queryByRole("button", { name: "Start dictation" })).not.toBeInTheDocument();
    await enable();
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start dictation" }));
      await Promise.resolve();
    });
    await advance();
    expect(screen.getByLabelText("Transcript to review")).toHaveValue("Synthetic phrase.");
    expect(h.onUseTranscript).not.toHaveBeenCalled();
    fireEvent.change(screen.getByLabelText("Transcript to review"), {
      target: { value: "Edited draft" },
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Use transcript" }));
      await Promise.resolve();
    });
    expect(h.onUseTranscript).toHaveBeenCalledWith("Edited draft");
    expect(screen.queryByLabelText("Transcript to review")).not.toBeInTheDocument();
  });
  it("retains review when append would overflow, and discard leaves existing draft untouched", async () => {
    const h = harness();
    h.onUseTranscript.mockReturnValue(false);
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    await enable();
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start dictation" }));
      await Promise.resolve();
    });
    await advance();
    fireEvent.click(screen.getByRole("button", { name: "Use transcript" }));
    expect(screen.getByRole("alert")).toHaveTextContent("limit");
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Discard transcript" }));
      await Promise.resolve();
    });
    expect(screen.queryByLabelText("Transcript to review")).not.toBeInTheDocument();
    expect(h.onUseTranscript).toHaveBeenCalledTimes(1);
  });
  it("queues keyboard release during start and submits Finish only once", async () => {
    const h = harness();
    h.client.control.mockResolvedValue({ ...base, status: "finishing", sequence: 1 });
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    await enable();
    const hold = screen.getByRole("button", { name: "Hold to talk" });
    await act(async () => {
      await Promise.resolve();
      fireEvent.keyDown(hold, { key: " " });
      fireEvent.keyUp(hold, { key: " " });
    });
    await advance();
    expect(h.client.control).toHaveBeenCalledTimes(1);
    expect(h.client.control.mock.calls[0]?.[1]).toBe("finish");
  });
  it("cancels on unmount and rejects late insertion into another conversation", async () => {
    const h = harness();
    const view = render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    await enable();
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start dictation" }));
      await Promise.resolve();
    });
    view.unmount();
    await advance();
    expect(h.client.control).toHaveBeenCalledWith(
      expect.objectContaining({ conversationId: "conversation-1" }),
      "cancel",
    );
    expect(h.onUseTranscript).not.toHaveBeenCalled();
  });
  it("stops reading explicitly and does not overlap generation", async () => {
    const h = harness();
    h.client.start.mockResolvedValue({ ...base, status: "speaking", sequence: 1 });
    const view = render(<VoiceControls {...h} conversationId="conversation-1" disabled canRead />);
    expect(screen.getByRole("button", { name: "Read answer" })).toBeDisabled();
    view.rerender(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead />,
    );
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Read answer" }));
      await Promise.resolve();
    });
    fireEvent.click(screen.getByRole("button", { name: "Stop reading" }));
    await advance();
    expect(h.client.start).toHaveBeenCalledWith("conversation-1", "speak");
    expect(h.client.control).toHaveBeenCalledTimes(1);
  });
  it("shows permission denial without raw errors or starting recording", async () => {
    const h = harness();
    h.client.start.mockResolvedValue({
      ...base,
      status: "error",
      error: "permission_denied",
      busy: false,
      sequence: 1,
    });
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    await enable();
    expect(screen.getByRole("alert")).toHaveTextContent("permission");
    expect(h.client.poll).not.toHaveBeenCalled();
  });
  it("keeps an empty edited review owned until explicit Discard", async () => {
    const h = harness();
    render(
      <VoiceControls {...h} conversationId="conversation-1" disabled={false} canRead={false} />,
    );
    await enable();
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start dictation" }));
      await Promise.resolve();
    });
    await advance();
    fireEvent.change(screen.getByLabelText("Transcript to review"), { target: { value: "" } });
    expect(screen.getByLabelText("Transcript to review")).toHaveValue("");
    expect(screen.getByRole("button", { name: "Use transcript" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Start dictation" })).toBeDisabled();
    expect(h.onBusy).toHaveBeenLastCalledWith(true);
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Discard transcript" }));
      await Promise.resolve();
    });
    expect(screen.queryByLabelText("Transcript to review")).not.toBeInTheDocument();
    expect(h.onBusy).toHaveBeenLastCalledWith(false);
    expect(screen.getByRole("button", { name: "Start dictation" })).toBeEnabled();
    expect(h.onUseTranscript).not.toHaveBeenCalled();
  });
});
