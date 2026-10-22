export const VOICE_ERRORS = [
  "unsupported",
  "unavailable",
  "permission_denied",
  "busy",
  "stale_session",
  "protocol",
  "limit",
  "timeout",
  "capture_failed",
  "recognition_failed",
  "recognition_no_speech",
  "recognition_assets_missing",
  "recognition_disabled",
  "recognition_service",
  "recognition_busy",
  "helper_diagnostics",
  "cleanup_failed",
] as const;
export type VoiceError = (typeof VOICE_ERRORS)[number];
export type VoiceOperation = "authorize" | "record" | "speak";
export const VOICE_STATUSES = [
  "idle",
  "starting",
  "checking_microphone",
  "checking_speech",
  "authorized",
  "recording",
  "finishing",
  "speaking",
  "review",
  "cancelled",
  "completed",
  "error",
] as const;
export interface VoiceSnapshot {
  readonly version: 1;
  readonly conversationId: string;
  readonly sessionId: string;
  readonly sequence: number;
  readonly status: (typeof VOICE_STATUSES)[number];
  readonly busy: boolean;
  readonly text: string | null;
  readonly error: VoiceError | null;
}
function forbiddenControl(text: string): boolean {
  return Array.from(text).some((c) => {
    const n = c.charCodeAt(0);
    return (n < 32 && c !== "\n" && c !== "\t") || n === 127;
  });
}
export function parseVoiceSnapshot(
  value: unknown,
  conversationId: string,
  previous?: VoiceSnapshot,
): VoiceSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value))
    throw new Error("protocol");
  const v = value as Record<string, unknown>;
  const fields = [
    "version",
    "conversationId",
    "sessionId",
    "sequence",
    "status",
    "busy",
    "text",
    "error",
  ];
  if (
    Object.keys(v).length !== fields.length ||
    !fields.every((k) => k in v) ||
    v["version"] !== 1 ||
    v["conversationId"] !== conversationId ||
    typeof v["sessionId"] !== "string" ||
    !/^[a-f0-9-]{1,64}$/.test(v["sessionId"]) ||
    typeof v["sequence"] !== "number" ||
    !Number.isSafeInteger(v["sequence"]) ||
    v["sequence"] < 0 ||
    v["sequence"] > 64 ||
    !VOICE_STATUSES.some((s) => s === v["status"]) ||
    typeof v["busy"] !== "boolean" ||
    (v["text"] !== null &&
      (typeof v["text"] !== "string" ||
        v["text"].length > 4096 ||
        v["text"].trim() === "" ||
        forbiddenControl(v["text"]))) ||
    (v["error"] !== null && !VOICE_ERRORS.some((e) => e === v["error"])) ||
    (v["text"] !== null && (v["status"] !== "review" || v["busy"])) ||
    (v["status"] === "review" && !v["busy"] && v["text"] === null) ||
    (v["status"] === "error") !== (v["error"] !== null)
  )
    throw new Error("protocol");
  const next = v as unknown as VoiceSnapshot;
  if (
    previous &&
    (next.sessionId !== previous.sessionId ||
      next.sequence < previous.sequence ||
      (!previous.busy &&
        ["status", "busy", "text", "error", "sequence"].some(
          (k) => v[k] !== (previous as unknown as Record<string, unknown>)[k],
        )))
  )
    throw new Error("protocol");
  return Object.freeze(next);
}
export function appendTranscript(draft: string, transcript: string): string | null {
  if (!transcript.trim() || forbiddenControl(transcript)) return null;
  const joined = draft ? `${draft}${/\s$/.test(draft) ? "" : " "}${transcript}` : transcript;
  return joined.length <= 4096 ? joined : null;
}
export function voiceErrorMessage(error: unknown): string {
  const code = error instanceof Error ? error.message : error;
  switch (code) {
    case "permission_denied":
      return "Microphone or speech permission is not allowed. Typed chat remains available.";
    case "unsupported":
      return "Voice is available only on a supported Mac.";
    case "unavailable":
      return "On-device English recognition or an installed voice is unavailable. No cloud fallback or download was used.";
    case "limit":
      return "The voice limit was reached. Your existing draft is unchanged.";
    case "busy":
      return "Another native operation still owns this conversation.";
    case "timeout":
      return "Voice timed out. Pending audio and text were discarded.";
    case "helper_diagnostics":
      return "The native speech helper reported diagnostic output, so voice was stopped safely. Your existing draft is unchanged.";
    case "capture_failed":
      return "The microphone could not start safely. Your existing draft is unchanged.";
    case "recognition_no_speech":
      return "No speech was recognized. Your existing draft is unchanged.";
    case "recognition_assets_missing":
      return "The required on-device speech assets are not installed. No download or cloud fallback was used.";
    case "recognition_disabled":
      return "macOS reports that Siri or Dictation is disabled. Your existing draft is unchanged.";
    case "recognition_service":
      return "The local speech service could not initialize or stay connected. Your existing draft is unchanged.";
    case "recognition_busy":
      return "macOS reports another recognition instance is active. Your existing draft is unchanged.";
    case "recognition_failed":
      return "On-device speech recognition could not complete. Your existing draft is unchanged.";
    case "protocol":
      return "The voice response did not pass safety validation. Your existing draft is unchanged.";
    case "cleanup_failed":
      return "Voice cleanup is unconfirmed. New recording and Send remain blocked.";
    default:
      return "Voice could not complete safely. Your existing draft is unchanged.";
  }
}
