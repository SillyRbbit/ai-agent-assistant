# Project status

Last updated: 2026-07-14

## Current milestone

Phase 3 and Phase 4 Increment 4A - **verified complete**. The next Ready task is documentation-only Increment 4B planning for exact local tool-schema validation.

## Increment status

- Increment 1: smallest runnable Tauri application — **complete**.
- Increment 1.1: Node.js 26/npm 11 compatibility — **complete**.
- Increment 1.2: repository workflow and handoff system — **complete**.
- Increment 2A: platform-neutral Rust interfaces and deterministic mocks — **verified complete on target Mac**.
- Increment 2B-0: SQLite storage dependency and design decision — **complete**.
- Increment 2B-1: SQLite dependency and migration skeleton — **verified complete on target Mac**.
- Increment 2B-1A: Rust 1.90 SQLite compatibility repair — **verified complete on target Mac**.
- Increment 2C: storage startup integration — **verified complete on target Mac**.
- Increment 2D: macOS menu-bar and window lifecycle — **verified complete on target Mac**.
- Increment 2E: React application shell — **verified complete on target Mac**.
- Increment 2F: mocked assistant interaction shell — **verified complete on target Mac**.
- Increment 2G: integration hardening — **verified complete on target Mac**.
- Increment 3A: in-memory conversation sessions — **verified complete on target Mac**.
- Increment 3B: mock context provenance — **verified complete on target Mac**.
- Increment 3C: simulated tool result — **verified complete on target Mac**.
- Increment 3D: bounded mock-loop completion — **verified complete on target Mac**.
- Increment 4A: deterministic gateway protocol contract - **verified complete on target Mac**.

## Verified baseline through Increment 2E

- Tauri launches on the Apple Silicon target Mac.
- React renders in the native main window and invokes typed `get_app_info` IPC.
- SQLite startup is idempotent and persists only the bootstrap marker in development.
- The macOS status-item menu, close-to-hide, menu reopen, Dock reopen, and Quit work.
- The seven-route React shell, Settings diagnostics, Permissions placeholders, and closed menu routing work.
- No operating-system permission prompt appears.

## Increment 2F capability

- In-memory user and assistant messages.
- Fixed deterministic text chunks and progressive streaming.
- Stop with timer cancellation and late-event rejection.
- Mock `create_local_task` activity card.
- Exact mock preview for target, affected data, reversibility, permission, and risk.
- Deterministic approve, reject, and edit outcomes with no execution.
- Edit returns a deterministic draft to the composer.

## Verification evidence

Passed on the target Mac:

```text
npm run lint:frontend
npm run typecheck
targeted Vitest — 3 files, 37 tests
npm run verify
full Vitest — 4 files, 47 tests
Rust library tests — 50 passed
Rust integration tests — 6 passed
Vite production build
Tauri release build --no-bundle
git diff --check
native Tauri development launch
storage startup — idempotent, 2 migrations already applied
```

The project owner confirmed manual progressive streaming, Stop, approve/reject/edit outcomes, Edit draft restoration, minimum-window layout, close/reopen/Dock/quit behavior, Settings diagnostics, idempotent storage startup, and absence of permission prompts all passed.

## Security posture

- The model remains outside the authorization boundary.
- `get_app_info` remains the only custom Tauri command.
- Increment 2F changes frontend source, tests, styles, and project documentation only.
- Capabilities, CSP, Tauri configuration, Rust source, storage, dependencies, and lockfiles are unchanged.
- No model network, API key, OAuth, OS permission, shell, platform automation, or user-data persistence was added.
- WebView approval decisions are explicitly mock-only and cannot authorize or invoke an action.
- Run identifiers and valid-state checks reject stale asynchronous events.

## Increment 2G capability and evidence

- Typed mock-run driver with explicit cancellation.
- Bounded failure copy and deterministic Retry.
- Redacted in-memory Activity feed with no request, argument, result, or error-detail content.
- Stale chunk, completion, and failure events fail closed.

Passed:

```text
npm run verify
Frontend — 6 files, 63 tests passed
Rust library — 50 tests passed
Rust integration — 6 tests passed
Vite production build
Tauri release build --no-bundle
npm audit --audit-level=low — 0 vulnerabilities
git diff --check
```

Native launch passed with idempotent storage startup. The project owner confirmed streaming, Stop, approval decisions, Activity empty and populated states, newest-first lifecycle events, Activity redaction, close/reopen/Dock/quit behavior, Settings diagnostics, and absence of permission prompts all passed.

## Next action

On clean merged `main`, plan only the smallest Increment 4B exact local tool-schema validation contract. Reconcile the placeholder `ToolSchema`, tool registry, `ToolCallProposal`, policy boundary, and verified `UntrustedFunctionCall`; update planning documentation only and wait for project-owner approval before runtime or dependency changes.

## Phase 4 planning result

- The current synchronous Rust provider has no stream, gateway authentication, protocol version, event sequence, cancellation, deadline, correlation, redacted-error, or provider audit boundary.
- Production OpenAI credentials belong only to an authenticated gateway's server-side secret storage. A future gateway access token belongs to trusted Rust and platform secret storage, never the WebView or SQLite.
- The gateway selects exact server-owned tool contracts, forces foreground `stream: true`, `store: false`, `background: false`, and no parallel tool calls, and normalizes recognized OpenAI Responses events without gaining local tool authority.
- The upstream adapter may ignore additive fields on recognized events for documented API compatibility; unknown event types and malformed required fields fail. The normalized product protocol rejects unknown fields and variants.
- Function calls remain untrusted through strict provider generation and independent gateway/Rust validation. No call becomes actionable until exact local per-tool schema, policy, approval, and executor gates exist.
- Foreground cancellation propagates transport abort and rejects late events; it does not claim confirmed provider-side cancellation.
- Initial limits are two model turns, one non-parallel function call, one retry, three gateway requests, bounded request/event/argument/output/event-count sizes, and explicit connection/idle/turn/run deadlines.
- Gateway operational telemetry and local trusted audit are separate and exclude credentials and raw content by default.
- D-021 records the durable boundary. O-006 defers identity-provider and deployment selection until before live networking.
- O-007 defers provider retention-mode selection and user disclosure until before live provider traffic; `store: false` alone is not treated as zero retention.
- Increment 4A adds only a transport-free Rust normalized-protocol module, an exact direct `serde_json 1.0.150` dependency already present transitively, its module export/lock update, and inline fixture tests.
- Planning baseline passed `npm run typecheck`, 124 frontend tests, and 50 Rust library tests on clean merged main at `f56cab2`.
- Planning changed no runtime, dependency, lockfile, Rust, IPC, Tauri, capability, CSP, persistence, credential, network, packaging, or permission file.

## Increment 4A capability and evidence

- Protocol version `1` and conservative model-turn, function-call, retry, gateway-request, byte, output, event-count, and deadline constants are frozen in portable Rust.
- Normalized frames are bounded before decoding and validated against a closed envelope/event union, exact expected IDs, contiguous sequence, one start, text-or-one-call output, one terminal event, and no late frames.
- Local cancellation is idempotent and terminal without claiming confirmed provider cancellation.
- Completed calls validate opaque identity, exact allowed name and tool-contract version, and bounded duplicate-free JSON-object arguments, but remain private-field `UntrustedFunctionCall` data with no proposal, policy, approval, IPC, or executor conversion.
- Closed typed failures and errors structurally exclude provider messages, frames, output, arguments, headers, URLs, and credentials.
- Focused protocol tests: 17 passed.
- `npm run verify`: 124 frontend tests, 67 Rust library tests, six Rust integration tests, TypeScript, Vite production builds, and Tauri release no-bundle build passed.
- `npm audit --audit-level=low`: zero vulnerabilities. `git diff --check`, code review, and security review passed with no findings.
- No native manual interaction gate was required because the module is not wired to Tauri.
- No network, gateway, credential, IPC, WebView, provider, tool execution, persistence, capability, CSP, packaging, or permission path was added.

## Phase 3D planning result

- Phase 3 cannot close at Increment 3C because the verified loop has no distinct final answer after its simulated result.
- The current generic approve outcome renders before the result and does not represent a post-result continuation.
- The current one-proposal flow is bounded structurally, but the product's conservative loop limits are not represented as one closed contract.
- Repeated injected failures can continue offering Retry without an explicit retry-attempt cap.
- Increment 3D is limited to one fixed deterministic post-result answer, exact result/final identity and ordering, and explicit mock limits.
- Planned limits are two consecutive model turns, one tool call, one retry, zero network requests, zero tool timeout, zero file bytes, zero search results, and 512 assistant-output characters per turn.
- Production provider continuation, real execution, arbitrary payloads, generic timeline work, persistence, Rust, IPC, Tauri, dependencies, capabilities, CSP, and permissions remain excluded.
- Planning baseline passed TypeScript type checking, 104 frontend tests, and 50 Rust library tests on clean `phase3/increment-3d` at `5c3f934`.

## Increment 3D capability and evidence

- A frozen mock-loop contract limits each run to two model turns, one tool call, one retry, 512 output code points per turn, and zero network, tool-timeout, file, and search capacity.
- Approve appends one fixed final answer bound to the exact run, conversation, and simulated result; it renders immediately after that result.
- Reject and Edit retain fixed outcomes; Stop, failure, stale events, invalid decisions, and duplicate decisions create no final answer.
- Failure of retry attempt `1` creates no further Retry, and output over the fixed ceiling fails closed.
- Focused tests pass: 4 files, 81 tests.
- `npm run verify` passes with 124 frontend tests, 50 Rust library tests, 6 Rust integration tests, Vite builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reports zero vulnerabilities.
- Code review and security review pass with no findings.
- Native Tauri development launch passes with idempotent storage startup and two migrations already applied.
- No dependency, lockfile, Rust, IPC, Tauri, SQLite, capability, CSP, credential, network, packaging, or operating-system permission file changed.
- The project owner confirmed result/final ordering and run identity, privacy, no final answer after Reject/Edit/Stop, per-conversation restoration, normal and minimum-window layout, existing context and Activity behavior, native routing, Settings diagnostics, lifecycle behavior, storage startup, and absence of permission prompts all pass.
- Increment 3D and Phase 3 are verified complete on the target Mac.

## Phase 3C planning result

- The product brief requires tool results in the conversation center pane.
- The verified loop has a tool proposal, action preview, decision states, and fixed assistant outcomes but no distinct result model or view.
- The proposed increment adds one approve-only fixed result tied to exact run and conversation IDs and the derived proposal ID.
- The result encodes `executed: false`, `simulated`, and fixed no-change copy.
- Reject, Edit, Stop, stale events, and invalid decisions produce no result.
- Request text, arguments, preview content, errors, paths, and personal content remain excluded from result state and Activity.
- Real execution, provider continuation, arbitrary result schemas, trusted executor output, persistence, networking, dependencies, native capability changes, and permissions remain out of scope.
- Planning baseline passed with TypeScript type checking, 92 frontend tests, and 50 Rust library tests.

## Increment 3C capability and evidence

- Approve-only fixed results bind exact run, conversation, and derived proposal IDs.
- Result fields are fixed to `create_local_task`, `simulated`, `executed: false`, and no-change summary copy.
- Missing or mismatched proposals fail closed; Reject, Edit, Stop, stale events, invalid decisions, and duplicate decisions create no result.
- Result constructors accept identifiers only, and the UI identifies the card as frontend mock output rather than verified executor output.
- Focused tests pass: 4 files, 70 tests.
- `npm run verify` passes with 104 frontend tests, 50 Rust library tests, 6 Rust integration tests, Vite builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reports zero vulnerabilities.
- Native Tauri development launch passes with idempotent storage startup and two migrations already applied.
- Project-owner approve/reject/edit/Stop behavior, result-content exclusion, per-conversation restoration, normal and minimum-window layout, and existing native regression checks passed.

## Phase 3B planning result

- The verified application has no run-bound disclosure of what information the deterministic mock used.
- The product brief requires users to see what information the agent used, and D-017 identifies conversation identity as the prerequisite.
- The proposed increment adds one fixed-copy provenance record per run, tied to exact run and conversation IDs and stored only in volatile session state.
- The current request is the only source marked used; prior messages, saved memory, device data, and external services are explicitly not used.
- Request text and personal content remain excluded from provenance and Activity.
- Real context selection or collection, trusted provenance, persistence, tool results, networking, dependencies, native capability changes, and permissions remain out of scope.
- Planning baseline passed with TypeScript type checking, 83 frontend tests, and 50 Rust library tests.

## Increment 3B capability and evidence

- Fixed-copy `MockContextProvenance` records bind each run to its volatile conversation.
- Current request is marked used; prior messages, saved memory, device data, and external services are marked not used.
- Provenance constructors accept identifiers only, and the UI states that the disclosure is frontend mock data rather than trusted audit evidence.
- Submit and Retry append one fresh record; conversation selection restores only the owning records.
- Focused tests pass: 4 files, 66 tests.
- `npm run verify` passes with 92 frontend tests, 50 Rust library tests, 6 Rust integration tests, Vite builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reports zero vulnerabilities.
- Native Tauri development launch passes with idempotent storage startup and two migrations already applied.
- Project-owner native interaction, per-conversation restoration, minimum-window layout, and existing native regression checks passed.

## Phase 3 planning result

- The current mock loop already covers messages, streaming, Stop, mock tool activity, mock approval decisions, bounded failure, Retry, stale-event rejection, and redacted Activity presentation.
- The current transcript has no conversation identity or history, and New Request clears only the draft.
- Increment 3A adds volatile conversation sessions, bounded titles, newest-first history, New conversation, and idle selection without persistence or trust-boundary expansion.
- Active and retryable runs are bound to conversation IDs; session changes fail closed while streaming or awaiting approval.
- `npm run verify` passed with 83 frontend tests, 50 Rust library tests, 6 Rust integration tests, Vite production build, and Tauri release no-bundle build.
- The dependency audit reports zero vulnerabilities, and native launch passed with idempotent storage startup.
- No dependency, lockfile, Rust, Tauri, IPC, SQLite, capability, CSP, credential, network, packaging, or permission file changed.
- The project owner confirmed conversation layout, creation, restoration, empty-session reuse, busy-state guards, native New Request behavior, existing mock interactions, Activity redaction, lifecycle, diagnostics, storage, and no-permission-prompt behavior all passed.
