import { useEffect, useRef, useState } from "react";
import {
  VOICE_ERRORS,
  voiceErrorMessage,
  type VoiceOperation,
  type VoiceSnapshot,
} from "../../application/voice";
import { voiceClient, type VoiceClient } from "../../infrastructure/tauri/voice-client";
import "./voice.css";
interface Props {
  readonly conversationId: string;
  readonly disabled: boolean;
  readonly canRead: boolean;
  readonly onUseTranscript: (text: string) => boolean;
  readonly onBusy: (busy: boolean) => void;
  readonly client?: VoiceClient;
}
export function VoiceControls({
  conversationId,
  disabled,
  canRead,
  onUseTranscript,
  onBusy,
  client = voiceClient,
}: Props) {
  const [snapshot, setSnapshot] = useState<VoiceSnapshot | null>(null);
  const [enabled, setEnabled] = useState(false);
  const [pending, setPending] = useState(false);
  const [operation, setOperation] = useState<VoiceOperation | null>(null);
  const [uncertain, setUncertain] = useState(false);
  const [error, setError] = useState("");
  const [review, setReview] = useState("");
  const [seconds, setSeconds] = useState(0);
  const mounted = useRef(false);
  const current = useRef<VoiceSnapshot | null>(null);
  const pendingRef = useRef(false);
  const intent = useRef<"finish" | "cancel" | null>(null);
  const sent = useRef<"finish" | "cancel" | null>(null);
  const epoch = useRef(0);
  const wake = useRef<(() => void) | null>(null);
  const requestedOperation = useRef<VoiceOperation | null>(null);
  const busy = pending || snapshot?.busy === true || uncertain;
  const hasReview = snapshot?.status === "review";
  useEffect(() => {
    onBusy(busy || hasReview);
  }, [busy, hasReview, onBusy]);
  const cancel = () => {
    intent.current = "cancel";
    wake.current?.();
    setReview("");
    if (current.current && !current.current.busy) {
      current.current = null;
      setSnapshot(null);
    }
  };
  const finish = () => {
    intent.current = "finish";
    wake.current?.();
  };
  useEffect(() => {
    mounted.current = true;
    const hidden = () => {
      if (document.hidden) cancel();
    };
    const blur = () => {
      setReview("");
      if (requestedOperation.current !== "authorize") cancel();
    };
    window.addEventListener("blur", blur);
    document.addEventListener("visibilitychange", hidden);
    return () => {
      mounted.current = false;
      intent.current = "cancel";
      const owned = current.current;
      if (owned && (owned.busy || owned.status === "review"))
        void client.control(owned, "cancel").catch(() => undefined);
      window.removeEventListener("blur", blur);
      document.removeEventListener("visibilitychange", hidden);
      onBusy(false);
    };
  }, [client, onBusy]);
  useEffect(() => {
    if (snapshot?.status !== "recording") return;
    const started = performance.now();
    const timer = window.setInterval(() => {
      setSeconds(Math.min(30, Math.floor((performance.now() - started) / 1000)));
    }, 250);
    return () => {
      window.clearInterval(timer);
    };
  }, [snapshot?.status]);
  const isCurrent = (value: number): boolean => mounted.current && value === epoch.current;
  const desired = (): "finish" | "cancel" | null => intent.current;
  const submitted = (): "finish" | "cancel" | null => sent.current;
  const begin = async (operation: VoiceOperation) => {
    if (disabled || busy || pendingRef.current) return;
    pendingRef.current = true;
    setPending(true);
    setOperation(operation);
    requestedOperation.current = operation;
    setSeconds(0);
    setError("");
    setReview("");
    intent.current = null;
    sent.current = null;
    const ownEpoch = ++epoch.current;
    let owned: VoiceSnapshot | null = null;
    try {
      owned = await client.start(conversationId, operation);
      current.current = owned;
      if (!isCurrent(ownEpoch)) {
        if (owned.busy) await client.control(owned, "cancel");
        return;
      }
      setSnapshot(owned);
      // Finite observation; mutations are sent once, never retried after unknown outcomes.
      for (let attempt = 0; owned.busy && owned.status !== "error" && attempt < 600; attempt += 1) {
        if (!isCurrent(ownEpoch)) {
          await client.control(owned, "cancel");
          return;
        }
        const action = desired();
        if (
          action &&
          action !== submitted() &&
          (action === "cancel" || owned.status === "recording")
        ) {
          sent.current = action;
          owned = await client.control(owned, action);
        } else {
          await new Promise<void>((resolve) => {
            const timer = window.setTimeout(() => {
              wake.current = null;
              resolve();
            }, 250);
            wake.current = () => {
              window.clearTimeout(timer);
              wake.current = null;
              resolve();
            };
          });
          if (
            desired() !== submitted() &&
            (desired() === "cancel" || (desired() === "finish" && owned.status === "recording"))
          )
            continue;
          owned = await client.poll(owned);
        }
        current.current = owned;
        if (isCurrent(ownEpoch)) setSnapshot(owned);
      }
      if (owned.busy) throw new Error("cleanup_failed");
      if (owned.status === "error") throw new Error(owned.error ?? "protocol");
      if (owned.status === "authorized") setEnabled(true);
      if (owned.status === "review" && desired() !== "cancel") setReview(owned.text ?? "");
    } catch (failure) {
      if (owned?.busy && submitted() !== "cancel") {
        sent.current = "cancel";
        try {
          owned = await client.control(owned, "cancel");
          for (let i = 0; owned.busy && i < 24; i += 1) {
            await new Promise<void>((resolve) => window.setTimeout(resolve, 250));
            owned = await client.poll(owned);
          }
          current.current = owned;
          if (mounted.current) setSnapshot(owned);
        } catch {
          /* Unknown cleanup remains blocking; never infer success. */
        }
      }
      if (isCurrent(ownEpoch)) {
        const code = owned?.error ?? (failure instanceof Error ? failure.message : failure);
        const unknownStart =
          owned === null &&
          !VOICE_ERRORS.some((known) => known === code && known !== "cleanup_failed");
        setUncertain(owned?.busy === true || code === "cleanup_failed" || unknownStart);
        setError(voiceErrorMessage(owned?.busy ? "cleanup_failed" : code));
        setReview("");
      }
    } finally {
      pendingRef.current = false;
      if (isCurrent(ownEpoch)) setPending(false);
    }
  };
  const discardReview = async () => {
    const owned = current.current;
    if (owned?.status !== "review" || owned.busy || pendingRef.current) return;
    pendingRef.current = true;
    setPending(true);
    try {
      const next = await client.control(owned, "cancel");
      current.current = next;
      if (mounted.current) {
        setSnapshot(next);
        setReview("");
      }
    } catch {
      if (mounted.current) {
        setUncertain(true);
        setError(voiceErrorMessage("cleanup_failed"));
      }
    } finally {
      pendingRef.current = false;
      if (mounted.current) setPending(false);
    }
  };
  if (!client.available())
    return (
      <p className="voice-muted">
        Open Cortexa on a supported Mac to use voice. Typed chat remains available.
      </p>
    );
  return (
    <section className="voice-controls" aria-label="Foreground voice">
      <p>
        On-device English dictation. Audio is not saved by Cortexa. Review the transcript before
        Send.
      </p>
      <div className="voice-actions">
        {!enabled && (
          <button
            type="button"
            disabled={disabled || busy || hasReview}
            onClick={() => void begin("authorize")}
          >
            Enable voice
          </button>
        )}
        {enabled && (
          <>
            <button
              type="button"
              disabled={disabled || uncertain || hasReview || (busy && operation !== "record")}
              aria-label="Hold to talk"
              onPointerDown={(event) => {
                event.currentTarget.setPointerCapture(event.pointerId);
                void begin("record");
              }}
              onPointerUp={finish}
              onPointerCancel={cancel}
              onBlur={cancel}
              onLostPointerCapture={() => {
                if (intent.current !== "finish") cancel();
              }}
              onKeyDown={(event) => {
                if ((event.key === " " || event.key === "Enter") && !event.repeat) {
                  event.preventDefault();
                  void begin("record");
                }
              }}
              onKeyUp={(event) => {
                if (event.key === " " || event.key === "Enter") {
                  event.preventDefault();
                  finish();
                }
              }}
            >
              Hold to talk
            </button>
            <button
              type="button"
              disabled={disabled || busy || hasReview}
              onClick={() => void begin("record")}
            >
              Start dictation
            </button>
          </>
        )}
        <button
          type="button"
          disabled={!busy || operation !== "record" || snapshot?.status === "finishing"}
          onClick={finish}
        >
          Finish dictation
        </button>
        <button type="button" disabled={!busy} onClick={cancel}>
          {snapshot?.status === "speaking" ? "Stop reading" : "Cancel dictation"}
        </button>
        <button
          type="button"
          disabled={disabled || busy || hasReview || !canRead}
          onClick={() => void begin("speak")}
        >
          Read answer
        </button>
      </div>
      <p
        role="status"
        aria-live="polite"
        className={snapshot?.status === "recording" ? "voice-recording" : ""}
      >
        {snapshot?.status === "recording"
          ? `Recording · ${String(seconds)}s / 30s`
          : uncertain
            ? "Voice has not confirmed it stopped"
            : pending && operation === "authorize"
              ? snapshot?.status === "checking_microphone"
                ? "Checking microphone permission…"
                : snapshot?.status === "checking_speech"
                  ? "Microphone allowed. Checking speech permission…"
                  : "Starting permission check…"
              : snapshot?.status === "speaking"
                ? "Reading answer…"
                : busy
                  ? "Finishing voice…"
                  : "Microphone off"}
      </p>
      {hasReview && (
        <div className="voice-review">
          <label>
            Transcript to review
            <textarea
              aria-label="Transcript to review"
              value={review}
              maxLength={4096}
              onChange={(e) => {
                setReview(e.currentTarget.value);
              }}
            />
          </label>
          <button
            type="button"
            disabled={disabled || busy || !review.trim()}
            onClick={() => {
              if (onUseTranscript(review)) {
                void discardReview();
                setError("");
              } else setError(voiceErrorMessage("limit"));
            }}
          >
            Use transcript
          </button>
          <button
            type="button"
            onClick={() => {
              void discardReview();
            }}
          >
            Discard transcript
          </button>
          <p>
            Using this text only changes your draft. Sending still requires the existing
            acknowledgement and Send button.
          </p>
        </div>
      )}
      {error && <p role="alert">{error}</p>}
    </section>
  );
}
