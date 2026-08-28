# V0-11 — synthetic-v1 Tauri contract and accessible presentation

Status: Blocked by V0-2, V0-7, V0-10, and a fresh F-12 review
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-2, V0-7, V0-10, and fresh F-12 review; no live traffic dependency

## Goal and outcome

Expose the fixed synthetic session through three narrow versioned Tauri
commands and a separately labelled volatile Conversations panel. Bind explicit
external-processing acknowledgment inside trusted Rust before transport start;
parse every reply from `unknown`; render chronological bounded text and closed
terminal recovery accessibly. This increment sends no external byte.

## Exact files

- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_v0.rs`
- `src-tauri/src/personal_assistant_v0_transport.rs`
- `src-tauri/src/personal_assistant_v0_tauri.rs` (new)
- `src-tauri/tests/personal_assistant_v0_tauri_contract.rs` (new)
- `src/infrastructure/tauri/personal-assistant-v0-client.ts` (new)
- `src/infrastructure/tauri/personal-assistant-v0-client.test.ts` (new)
- `src/application/personalAssistantV0.ts` (new)
- `src/application/personalAssistantV0.test.ts` (new)
- `src/application/PersonalAssistantV0Provider.tsx` (new)
- `src/application/PersonalAssistantV0Provider.test.tsx` (new)
- `src/application/usePersonalAssistantV0.ts` (new)
- `src/features/conversations/PersonalAssistantV0Panel.tsx` (new)
- `src/features/conversations/PersonalAssistantV0Panel.test.tsx` (new)
- `src/features/conversations/ConversationWorkspace.tsx`
- `src/features/settings/SettingsPage.tsx`
- `src/features/settings/SettingsPage.test.tsx` (new)
- `src/App.tsx`
- `src/App.test.tsx`
- `src/styles.css`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-tauri-presentation.md`
- `docs/increments/personal-assistant-v0-tauri-presentation.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-tauri-presentation-post-increment-review.md`
  (new)

No manifest/lockfile, Tauri configuration, capability, CSP, permission, mock
reducer/driver, Command Center, tool, workflow, memory, storage, gateway, or
provider file may change.

## Exact synthetic-v1 commands

All Rust request/reply/error structs use camel-case serialization and
`deny_unknown_fields`; TypeScript requires exact own keys and rejects inherited,
missing, or additional keys. Every top-level reply/error carries
`contractVersion: 1`.

| Command                                  | Exact input                                                                                                                                         | Success                                                                        |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `start_personal_assistant_synthetic_v1`  | `{ contractVersion: 1, disclosureAcknowledgment: { disclosureVersion: "personal-assistant-synthetic-external-processing@1", acknowledged: true } }` | Rust-issued handle and sequence-0 `starting` snapshot                          |
| `poll_personal_assistant_synthetic_v1`   | `{ contractVersion: 1, presentationHandle, afterSequence: number \| null }`                                                                         | At most 16 contiguous updates and one authoritative snapshot                   |
| `cancel_personal_assistant_synthetic_v1` | `{ contractVersion: 1, presentationHandle }`                                                                                                        | Authoritative `cancelling` or terminal snapshot; repeated cancel is idempotent |

Start accepts no text, fixture, identity, instruction, agent, provider/model,
endpoint, credential, data class, tool, runtime, workflow, outcome, limit,
retry, or fallback. Rust validates the exact disclosure object, mints a private
one-use `SyntheticDisclosureAdmissionV1`, reserves the process-wide session,
and passes that admission to the only production transport-start path. The
transport cannot write a socket without the admission. It is volatile,
nonserializable, non-cloneable, never returned/logged, and consumed once. A
Tauri-adapter Boolean check without this Rust-core gate is forbidden.

Presentation handles/support correlations are nonempty visible ASCII and at
most 128 bytes; they are opaque correlations, not bearer authorization. All
numbers below are JSON safe integers. The exact top-level success shapes are:

```text
StartReplyV1 = {
  contractVersion: 1,
  presentationHandle: string,
  snapshot: StartingSnapshotV1
}

PollReplyV1 = {
  contractVersion: 1,
  presentationHandle: string,
  requestedAfterSequence: number | null,
  throughSequence: number,
  hasMore: boolean,
  updates: UpdateV1[],
  snapshot: SnapshotV1
}

CancelReplyV1 = {
  contractVersion: 1,
  presentationHandle: string,
  snapshot: CancellingSnapshotV1 | CancelledSnapshotV1 |
            CompletedSnapshotV1 | FailedSnapshotV1
}
```

The exact update union, with no keys beyond those shown, is:

```text
StartedUpdateV1   = { kind: "started", sequence: 1 }
TextDeltaUpdateV1 = { kind: "text_delta", sequence: number, text: string }
CompletedUpdateV1 = { kind: "completed", sequence: number, finalAnswer: string }
FailedUpdateV1    = { kind: "failed", sequence: number, failure: FailureV1 }
CancelledUpdateV1 = { kind: "cancelled", sequence: number }
```

Every update sequence is in 1..128. A text delta is nonempty and bounded to
1,024 Unicode scalars/4,096 UTF-8 bytes. Accumulated accepted text is bounded
to 8,192 scalars/32,768 UTF-8 bytes. `completed` is nonempty and
`finalAnswer` equals the concatenation of all accepted deltas exactly.

The exact snapshot union, again with no additional keys, is:

```text
StartingSnapshotV1 = {
  state: "starting", sequence: 0, acceptedText: ""
}
StreamingSnapshotV1 = {
  state: "streaming", sequence: number, acceptedText: string
}
CancellingSnapshotV1 = {
  state: "cancelling", sequence: number, acceptedText: string
}
CompletedSnapshotV1 = {
  state: "completed", sequence: number, acceptedText: string,
  finalAnswer: string
}
FailedSnapshotV1 = {
  state: "failed", sequence: number, acceptedText: string,
  failure: FailureV1
}
CancelledSnapshotV1 = {
  state: "cancelled", sequence: number, acceptedText: string
}
```

`starting.sequence` is exactly 0; `cancelling.sequence` is 0..128;
`streaming`, `failed`, and `cancelled` sequences are 1..128; and `completed`
is 3..128 because it follows sequence-1 `started`, at least one nonempty delta,
and its own terminal update. Null recovery enforces the same state-specific
ranges. Completed text is nonempty and exact. Failure is exactly:

```text
FailureV1 = {
  code: "unauthenticated" | "forbidden" | "rate_limited" |
        "request_rejected" | "provider_unavailable" |
        "provider_timeout" | "protocol_violation" |
        "limit_exceeded" | "deadline_exceeded" |
        "cleanup_failed" | "internal",
  supportCorrelation: string | null
}
```

`supportCorrelation` is required and nonnull only for `cleanup_failed` or
`internal`; it is exactly null for every other failure. Command rejection uses
Tauri's error branch with this exact shape:

```text
CommandErrorV1 = {
  contractVersion: 1,
  code: "disclosure_required" | "busy" | "invalid_request" |
        "invalid_handle" | "invalid_cursor" | "protocol_violation" |
        "limit_exceeded" | "deadline_exceeded" |
        "cleanup_required" | "unavailable",
  supportCorrelation: string | null
}
```

The correlation is required and nonnull only for `unavailable`; it is null for
every other command code. Fixed TypeScript-owned copy maps each closed code;
unknown/native/provider error text is never stringified, rendered, or logged.

For an ordinary poll, `afterSequence` and echoed `requestedAfterSequence` are
the same integer in 0..128. `updates` has 0..16 entries, starts exactly at
`afterSequence + 1`, and is contiguous. `throughSequence` is the last returned
sequence, or equals `afterSequence` for an empty batch. `hasMore` is true if
and only if the authoritative snapshot sequence is greater than
`throughSequence`. When false, snapshot sequence equals `throughSequence` and
reducing the batch from the caller's previously accepted state yields the
snapshot exactly. When true, the snapshot must preserve the already accepted
prefix, but its newer text/state is not committed or rendered until journal
updates catch up. A cursor greater than the authoritative sequence or outside
0..128 is `invalid_cursor`; the 128-entry journal never evicts an update needed
by the sole active consumer.

`afterSequence: null` is recovery only. The reply echoes
`requestedAfterSequence: null`, returns `updates: []`, sets
`throughSequence` to `snapshot.sequence`, and sets `hasMore: false`. The
snapshot belongs to the same handle and may advance the prior client state only
through zero or more legal transitions with prefix-preserving accepted text; it
cannot rewrite a transcript. Cancel returns only `cancelling` or a terminal
snapshot; repeated cancel returns the same terminal snapshot exactly.

The only legal update reductions are `starting --started--> streaming`,
`starting --failed|cancelled--> terminal`, `streaming --text_delta-->
streaming`, and `streaming --completed|failed|cancelled--> terminal`. The only
snapshot-only cancel transitions are `starting -> cancelling` and `streaming ->
cancelling` after local ingress closes, followed only by `cancelling ->
cancelled|failed`. `cancelling` accepts no text. If a terminal event won the
race before cancel, cancel returns that existing terminal snapshot. Terminal
state accepts no update or transition.

## Exact disclosure and presentation

The exact visible paragraph, pinned in Rust and TypeScript tests, is:

```text
This sends the fixed application-owned synthetic prompt to OpenAI through an Access-protected Cloudflare Worker. No personal data, tools, files, memory, persistence, or device action is used. With the current synthetic configuration, OpenAI may retain prompt and response content in abuse-monitoring logs for up to 30 days and encrypted prompt-cache state for up to 24 hours; store=false disables Responses application-state storage but is not Zero Data Retention. On the current Cloudflare Free plan, Access authentication metadata is retained for 24 hours and mandatory admin-action audit records are retained for 18 months. Do not continue if these settings or periods have changed.
```

The unchecked control reads `I understand this fixed synthetic prompt will
leave this Mac.` The UI never checks it automatically. A material provider,
retention, data-class, or copy change requires a new disclosure version. The
same disclosure remains visible in Settings.

The panel is labelled `LIVE SYNTHETIC TEXT PROOF`; it shows the exact fixed
prompt read-only before Start and preserves the separate existing `DEMO MODE ·
SIMULATED AGENT DATA` disclosure where the deterministic mock appears. It
renders one chronological fixed user entry, one assistant entry updated in
place, and one terminal status. Partial output on failure/cancel is retained
and labelled `Incomplete`. Model text is escaped plain text—no Markdown, HTML,
links, or `dangerouslySetInnerHTML`.

The transcript is labelled, is not a live region, and never announces each
token. A separate atomic polite region announces coarse transitions; closed
failure uses one alert. The streaming assistant is `aria-busy`. Focus moves to
Stop after accepted start and back to Start after terminal state. A poll or
parser failure with a retained handle moves focus to `Recover status`; recovery
performs only the null-cursor poll and never resubmits. An unknown start outcome
has no trusted handle and therefore offers no recovery or automatic retry. It
shows fixed closed guidance to wait through the 120-second run deadline, Quit
the app, and relaunch it before trying again. Merely closing the window is not
described as terminating the macOS app.

## TypeScript controller and parser

The controller lives at `ApplicationShell`/provider scope, not in the
route-mounted panel, and retains the handle across route changes. It schedules
one 250 ms `setTimeout` only after the prior poll settles; `setInterval` and
concurrent polls are forbidden. Separate session/poll generations bind each
promise to generation, handle, cursor, and operation token. Route-away,
document invisibility, cancel intent, recovery, terminal state, and teardown
clear the timer and invalidate polling. Route-back performs one snapshot-only
poll, never start. Visibility restoration on the same mounted route also
performs exactly one null-cursor snapshot recovery, never Start, and resumes
post-settlement 250 ms single-flight polling only if the recovered snapshot is
nonterminal. That recovery retains the existing generation/handle guards; on
failure it enters the handle-bound `Recover status` state without a loop.
Cancel invalidates an in-flight poll before invoking cancel. An unknown start
outcome is not retried. Route-away does not end the native run, but window
destruction or application exit invokes trusted cancellation of the owned
request. If a WebView crash is not observable, Rust's active total deadline
still aborts the request; it retains the process lease until teardown is proved
and rejects restart and late data. WebView crash reattachment is not claimed.

Every invoke result begins as `unknown`. The parser builds a complete immutable
candidate and validates exact own keys/literals, safe integers, well-formed
Unicode, scalar/UTF-8 bounds, arrays, contiguous sequences, legal transitions,
terminal exclusivity, prefix-preserving text, and final equality before any
application retention/state commit. Any defect rejects the whole reply.

## F-12, tests, and manual gate

Repository health atomically pins the exact three Personal Assistant commands,
sole Personal Assistant client,
single provider/panel consumer, approved timer/visibility lifecycle,
disclosure/profile constants, no-text start DTO, exact parsers, and absence of
Personal Assistant events, `listen`/`emit`, alternate raw invoke imports,
WebView fetch/XHR, storage, or alternate Personal Assistant consumers. Existing
approved menu/lifecycle event clients remain exact and unchanged.
Production/dev CSP and `core:default` remain unchanged.

Rust/TypeScript tests cover exact serialization, disclosure admission order,
unknown fields/values, poisoned lock, private identities, handles/cursors,
every parser/transition/boundary, chronology, final equality, gap/stale/late
data, route/visibility-away/visibility-restoration/window-loss lifecycle,
exactly one null-cursor visibility recovery, active-deadline containment,
generations, cancel races, focus, busy, unknown-start versus handle-bound
failure/recovery, no mock coupling, and cross-language fixture/disclosure
equality.

The V0-11 preflight must prove `PA_V0_TRAFFIC_ENABLED=false`, no live route is
mapped, and the entire command/controller suite uses the injected fake
transport. Real Keychain/provider credentials must be absent from the test
process and cannot be consulted. A canary network sink must observe zero
external attempts. Any uncertain route, credential lookup, or real transport
construction stops the increment; it cannot be waived by V0-9 handoff state.
Repository health must also preserve the existing global
`__TAURI_INTERNALS__`/`__TAURI__` and `@tauri-apps/` allowlists while adding the
Personal Assistant-specific command restrictions.

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test personal_assistant_v0_tauri_contract
npm run test:frontend
npm run test:repository
npm run typecheck
npm run lint
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Target-Mac manual checks cover non-transmitting layout, disclosure, checkbox,
keyboard/focus, screen-reader semantics where tooling permits, standard/narrow
resize, 200% zoom, light/dark, reduced motion, long-text scroll ownership,
route-away/back and minimize/hide/restore with injected test state, console
redaction, no permission prompt, and no device effect. Live
success/failure/cancellation/network checks are `Not run` until V0-13.

## Rollback, stop conditions, and readiness

Rollback removes only the exact command/state/client/controller/panel/guard and
closeout changes; existing mocks remain unchanged and no external state exists.
Stop on capability/CSP/permission/dependency change, a fourth command/event,
raw bridge, caller configuration, mock rewrite, content persistence/logging,
background poll, stale/gapped commit, same contract for real text,
inaccessible recovery, or failed target-Mac static/accessibility evidence.

**Blocked.** V0-2, V0-7, V0-10, and a fresh F-12 review are unresolved. This
source increment must close before the first synthetic transmission.
