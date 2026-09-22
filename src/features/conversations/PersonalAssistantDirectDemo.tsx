import { useCallback, useEffect, useRef, useState } from "react";
import {
  directClient,
  directErrorMessage,
  DIRECT_DISCLOSURE,
  DIRECT_SAMPLE,
  IDLE_DIRECT,
  type DirectClient,
  type DirectSnapshot,
} from "../../infrastructure/tauri/personal-assistant-direct-client";

export function PersonalAssistantDirectDemo({
  client = directClient,
}: {
  readonly client?: DirectClient;
}) {
  const [snapshot, setSnapshot] = useState(IDLE_DIRECT);
  const [acknowledged, setAcknowledged] = useState(false);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);
  const mounted = useRef(false);
  const current = useRef(IDLE_DIRECT);
  const operation = useRef(0);
  const startPending = useRef(false);
  const stopRequested = useRef(false);
  const available = client.available();
  const accept = useCallback((next: DirectSnapshot, fresh = false) => {
    const previous = current.current;
    if (
      !fresh &&
      previous.handle !== null &&
      (next.handle !== previous.handle ||
        next.sequence < previous.sequence ||
        !next.text.startsWith(previous.text) ||
        (["completed", "stopped", "error"].includes(previous.status) &&
          (next.status !== previous.status || next.text !== previous.text)))
    )
      throw new Error("protocol");
    current.current = next;
    if (mounted.current) setSnapshot(next);
  }, []);
  const stop = useCallback(async () => {
    stopRequested.current = true;
    const generation = ++operation.current;
    if (startPending.current) return;
    const handle = current.current.handle;
    if (handle === null) return;
    try {
      const next = await client.cancel(handle);
      if (mounted.current && generation === operation.current) accept(next);
    } catch (failure) {
      if (mounted.current) setError(directErrorMessage(failure));
    }
  }, [accept, client]);

  useEffect(() => {
    mounted.current = true;
    let disposed = false;
    if (available) {
      void client
        .poll(null, null)
        .then((next) => {
          if (!disposed) {
            accept(next, true);
            setLoading(false);
          }
        })
        .catch((failure: unknown) => {
          if (!disposed) {
            setError(directErrorMessage(failure));
            setLoading(false);
          }
        });
    }
    const onVisibility = () => {
      if (document.hidden) void stop();
    };
    document.addEventListener("visibilitychange", onVisibility);
    return () => {
      disposed = true;
      mounted.current = false;
      document.removeEventListener("visibilitychange", onVisibility);
      void stop();
    };
  }, [accept, available, client, stop]);

  useEffect(() => {
    if (!snapshot.busy || snapshot.handle === null || startPending.current) return;
    let disposed = false;
    const generation = operation.current;
    const timer = window.setTimeout(() => {
      void client
        .poll(snapshot.handle, snapshot.sequence)
        .then((next) => {
          if (!disposed && mounted.current && generation === operation.current) accept(next);
        })
        .catch((failure: unknown) => {
          if (!disposed && mounted.current) {
            setError(directErrorMessage(failure));
            void stop();
          }
        });
    }, 250);
    return () => {
      disposed = true;
      window.clearTimeout(timer);
    };
  }, [accept, client, snapshot, stop]);

  const shouldStop = () => !mounted.current || stopRequested.current;
  const start = async () => {
    if (!acknowledged || !available || loading || current.current.busy || startPending.current)
      return;
    startPending.current = true;
    stopRequested.current = false;
    ++operation.current;
    setError("");
    setLoading(true);
    try {
      let next = await client.start();
      if (shouldStop()) {
        if (next.handle !== null) next = await client.cancel(next.handle);
      }
      if (mounted.current) accept(next, true);
    } catch (failure) {
      if (mounted.current) setError(directErrorMessage(failure));
    } finally {
      startPending.current = false;
      if (mounted.current) setLoading(false);
    }
  };

  return (
    <section className="settings-card page-panel" aria-label="Native OpenAI synthetic demo">
      <h2>Native live model · fixed synthetic sample</h2>
      <p>{DIRECT_SAMPLE}</p>
      <p>{DIRECT_DISCLOSURE}</p>
      {!available ? (
        <p role="status">
          Browser-only mode: native OpenAI requests are unavailable. Use the Cortexa development
          app. The mock demonstration remains separate.
        </p>
      ) : (
        <>
          <label>
            <input
              type="checkbox"
              checked={acknowledged}
              disabled={loading || snapshot.busy}
              onChange={(event) => {
                setAcknowledged(event.currentTarget.checked);
              }}
            />
            I acknowledge the external synthetic request and possible API charges.
          </label>
          <p>
            Development opt-in and a native session key are required. Selecting this mode sends
            nothing.
          </p>
          <button
            type="button"
            disabled={!acknowledged || loading || snapshot.busy}
            onClick={() => void start()}
          >
            Start native sample
          </button>
          <button type="button" disabled={!snapshot.busy && !loading} onClick={() => void stop()}>
            Stop native request
          </button>
          <p role="status">
            {loading ? "starting / checking native session" : snapshot.status}
            {snapshot.busy && ["completed", "stopped", "error"].includes(snapshot.status)
              ? " · finishing cleanup"
              : ""}
          </p>
          {snapshot.text !== "" ? (
            <div className="conversation-message conversation-message--assistant">
              <p>{snapshot.status === "completed" ? "Answer" : "Partial output · incomplete"}</p>
              <p style={{ whiteSpace: "pre-wrap" }}>{snapshot.text}</p>
            </div>
          ) : null}
          {error !== "" || snapshot.error !== null ? (
            <p role="alert">{error || directErrorMessage(snapshot.error)}</p>
          ) : null}
        </>
      )}
    </section>
  );
}
