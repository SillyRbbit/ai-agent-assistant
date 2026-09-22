import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { PersonalAssistantDirectDemo } from "./PersonalAssistantDirectDemo";
import {
  IDLE_DIRECT,
  type DirectSnapshot,
} from "../../infrastructure/tauri/personal-assistant-direct-client";

const starting: DirectSnapshot = {
  ...IDLE_DIRECT,
  handle: "pa-v0-present-0000000000000001",
  status: "starting",
  busy: true,
};
const completed: DirectSnapshot = {
  ...starting,
  status: "completed",
  busy: false,
  sequence: 3,
  text: "<script>escaped synthetic answer</script>",
};
const stopped: DirectSnapshot = { ...starting, status: "stopped", busy: false, sequence: 1 };
function harness() {
  return {
    available: () => true,
    start: vi.fn<() => Promise<DirectSnapshot>>().mockResolvedValue(starting),
    poll: vi.fn<() => Promise<DirectSnapshot>>().mockResolvedValue(IDLE_DIRECT),
    cancel: vi.fn<() => Promise<DirectSnapshot>>().mockResolvedValue(stopped),
  };
}
async function acknowledge() {
  await waitFor(() => {
    expect(screen.getByRole("checkbox")).toBeEnabled();
  });
  fireEvent.click(screen.getByRole("checkbox"));
}
describe("native sample presentation", () => {
  afterEach(() => {
    vi.useRealTimers();
  });
  it("does not dispatch in browser mode or start merely by mounting", async () => {
    const client = harness();
    const view = render(
      <PersonalAssistantDirectDemo client={{ ...client, available: () => false }} />,
    );
    expect(client.poll).not.toHaveBeenCalled();
    expect(client.start).not.toHaveBeenCalled();
    expect(screen.queryByRole("textbox")).not.toBeInTheDocument();
    view.unmount();
    render(<PersonalAssistantDirectDemo client={client} />);
    await waitFor(() => {
      expect(screen.getByRole("checkbox")).toBeEnabled();
    });
    expect(screen.getByRole("button", { name: "Start native sample" })).toBeDisabled();
    expect(client.start).not.toHaveBeenCalled();
  });
  it("streams escaped text to explicit completion and permits a fresh explicit run", async () => {
    const client = harness();
    render(<PersonalAssistantDirectDemo client={client} />);
    await acknowledge();
    vi.useFakeTimers();
    client.poll
      .mockResolvedValueOnce({ ...starting, status: "streaming", sequence: 1 })
      .mockResolvedValueOnce(completed);
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start native sample" }));
      await Promise.resolve();
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    expect(screen.getByText(completed.text)).toBeInTheDocument();
    expect(document.querySelector("script")).toBeNull();
    expect(screen.getByText("Answer")).toBeInTheDocument();
    expect(client.start).toHaveBeenCalledTimes(1);
    client.start.mockResolvedValueOnce({ ...starting, handle: "pa-v0-present-0000000000000002" });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start native sample" }));
      await Promise.resolve();
    });
    expect(screen.queryByText(completed.text)).not.toBeInTheDocument();
    expect(client.start).toHaveBeenCalledTimes(2);
  });
  it("cancels a start that resolves after Stop or unmount", async () => {
    for (const unmount of [false, true]) {
      const client = harness();
      let resolveStart!: (value: DirectSnapshot) => void;
      client.start.mockReturnValue(
        new Promise((resolve) => {
          resolveStart = resolve;
        }),
      );
      const view = render(<PersonalAssistantDirectDemo client={client} />);
      await acknowledge();
      fireEvent.click(screen.getByRole("button", { name: "Start native sample" }));
      if (unmount) view.unmount();
      else fireEvent.click(screen.getByRole("button", { name: "Stop native request" }));
      await act(async () => {
        resolveStart(starting);
        await Promise.resolve();
      });
      expect(client.cancel).toHaveBeenCalledWith(starting.handle);
      if (!unmount) expect(screen.getByRole("status")).toHaveTextContent("stopped");
      view.unmount();
    }
  });
  it("rejects a late poll after cancellation and shows no mock fallback on errors", async () => {
    const client = harness();
    render(<PersonalAssistantDirectDemo client={client} />);
    await acknowledge();
    vi.useFakeTimers();
    let resolvePoll!: (value: DirectSnapshot) => void;
    client.poll.mockReturnValueOnce(
      new Promise((resolve) => {
        resolvePoll = resolve;
      }),
    );
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start native sample" }));
      await Promise.resolve();
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Stop native request" }));
      await Promise.resolve();
    });
    await act(async () => {
      resolvePoll(completed);
      await Promise.resolve();
    });
    expect(screen.getByRole("status")).toHaveTextContent("stopped");
    expect(screen.queryByText(completed.text)).not.toBeInTheDocument();
    client.start.mockRejectedValueOnce("missing_key");
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start native sample" }));
      await Promise.resolve();
    });
    expect(screen.getByRole("alert")).toHaveTextContent("No valid native session");
    expect(screen.queryByText("Answer")).not.toBeInTheDocument();
  });
  it.each([
    [
      "network",
      "connection failed while sending or receiving data",
      "No mock response was substituted.",
    ],
    ["http_status", "returned an unsuccessful HTTP response", "No mock response was substituted."],
    [
      "provider_stream",
      "reported a failure in the response stream",
      "No mock response was substituted.",
    ],
    [
      "provider_stream_server_error",
      "The provider reported a server error in the response stream. No automatic retry was made.",
      "No automatic retry was made.",
    ],
    [
      "provider_stream_rate_limit",
      "The provider reported a rate limit in the response stream. No automatic retry was made.",
      "No automatic retry was made.",
    ],
    [
      "provider_stream_invalid_prompt",
      "The provider reported an invalid prompt in the response stream. No automatic retry was made.",
      "No automatic retry was made.",
    ],
  ] as const)(
    "shows terminal %s without retry and releases controls",
    async (error, message, policyMessage) => {
      const client = harness();
      render(<PersonalAssistantDirectDemo client={client} />);
      await acknowledge();
      vi.useFakeTimers();
      const partial: DirectSnapshot = {
        ...starting,
        status: "streaming",
        sequence: 1,
        text: "Synthetic partial text",
      };
      client.poll
        .mockResolvedValueOnce(partial)
        .mockResolvedValueOnce({ ...partial, status: "error", error, busy: true, sequence: 2 })
        .mockResolvedValueOnce({ ...partial, status: "error", error, busy: false, sequence: 2 });
      await act(async () => {
        fireEvent.click(screen.getByRole("button", { name: "Start native sample" }));
        await Promise.resolve();
      });
      await act(async () => {
        await vi.advanceTimersByTimeAsync(250);
      });
      expect(screen.getByRole("status")).toHaveTextContent(/^streaming$/);
      await act(async () => {
        await vi.advanceTimersByTimeAsync(250);
      });
      expect(screen.getByRole("alert")).toHaveTextContent(message);
      expect(screen.getByRole("status")).toHaveTextContent("finishing cleanup");
      expect(screen.getByRole("button", { name: "Start native sample" })).toBeDisabled();
      await act(async () => {
        await vi.advanceTimersByTimeAsync(250);
      });
      expect(screen.getByRole("status")).toHaveTextContent(/^error$/);
      expect(screen.getByRole("button", { name: "Start native sample" })).toBeEnabled();
      expect(screen.getByRole("button", { name: "Stop native request" })).toBeDisabled();
      expect(screen.getByRole("checkbox")).toBeEnabled();
      expect(screen.getByText("Partial output · incomplete")).toBeInTheDocument();
      expect(screen.getByText(partial.text)).toBeInTheDocument();
      expect(screen.queryByText("Answer")).not.toBeInTheDocument();
      expect(screen.getByRole("alert")).toHaveTextContent(policyMessage);
      const polls = client.poll.mock.calls.length;
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });
      expect(client.start).toHaveBeenCalledTimes(1);
      expect(client.poll).toHaveBeenCalledTimes(polls);
    },
  );
});
