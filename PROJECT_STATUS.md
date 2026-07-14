# Project status

Last updated: 2026-07-14

## Current milestone

Phase 3 and Phase 4 Increments 4A through 4D - **verified complete**. Documentation-only Increment 4E planning for a trusted approval-decision source is the next Ready task.

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
- Increment 4B: exact local tool-schema validation - **verified complete on target Mac**.
- Increment 4C: trusted policy-input binding - **verified complete on target Mac**.
- Increment 4D: exact approval binding - **verified complete on target Mac**.

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

Plan only Increment 4E's trusted approval-decision source. Reconcile native and WebView trust boundaries, exact choice binding, user-presence and optional-authentication claims, cancellation, expiry, replay, error redaction, and future audit handoff; recommend one exact runtime increment and wait for project-owner approval before implementation.

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

## Increment 4B capability and evidence

- The closed catalog contains only `get_current_datetime@1` with an exact empty object and `create_local_task@1` with one required canonical title capped at 200 Unicode scalar values.
- Schema-backed private definitions derive exact name, description, version, risk, permission, and strict input schema locally.
- `validate_function_call` consumes an Increment 4A `UntrustedFunctionCall`, independently checks local registry identity, contract version, exact shape, and title semantics, then drops the raw JSON.
- Successful output has private typed arguments, locally derived classification, redacted debug output, and explicit non-authorizing semantics.
- Missing/additional/wrong-type fields, malformed values, empty or non-canonical titles, overlength titles, controls, unknown tools, and version mismatches fail closed through typed redacted errors.
- `ToolCallProposal`, provider response, policy, approval, audit, executor, runtime registration, gateway transport, IPC, persistence, and UI remain unchanged.
- Existing direct `serde` and `serde_json` were sufficient; Cargo manifests and lockfiles are unchanged.
- Planning baseline passed TypeScript, three tool-registry tests, and 17 gateway-protocol tests on clean merged `main` at `e1db18b`.
- Focused final tests passed: five schema, four registry, and six gateway-to-local validation tests.
- `npm run verify` passed with 124 frontend tests, 79 Rust library tests, six Rust integration tests, production frontend builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reported zero vulnerabilities; diff checks, code review, and security review passed with no findings.
- No native interaction gate was required because the modules remain transport-free and unreferenced by Tauri.
- No dependency, network, credential, provider, proposal conversion, policy call, approval, audit, executor, IPC, persistence, capability, CSP, packaging, permission, or user-visible path was added.

## Increment 4C planning result

- The unused `ToolCallProposal` and provider tool-call response variant can carry caller-supplied raw JSON and classification around the verified schema boundary, although repository search found no production caller.
- `ProposedAction` independently accepts tool identity, risk, permission, and public context fields, while `PolicyDecision` drops the evaluated action and accepts arbitrary reason text.
- The proposed increment removes those raw construction paths and makes one owned `SchemaValidatedFunctionCall` the sole source of policy identity, contract version, typed arguments, risk, and permission.
- `PolicyContext` is removed rather than relabeled: its booleans cannot prove same-call intent, permission, resource scope, provenance, or freshness.
- Permission-bearing and read-only calls deny, while reversible actions require approval until a later increment defines exact call-bound trusted evidence.
- A closed `PolicyReason` derives one `PolicyOutcome`, and `PolicyDecision` retains the exact consumed input without clone, serialization, raw debug, approval, audit, dispatch, or executor conversion.
- Canonical means ownership-bound structured typed input in Increment 4C. Canonical bytes, hashes, previews, intent/permission/scope evidence, expiry, one-time consumption, and run binding are deferred to later approved increments.
- The exact runtime plan creates one public boundary integration-test file and changes only four existing Rust type/policy files. It adds no dependency and changes no manifest or lockfile.
- Planning baseline passed TypeScript and focused provider, function-call validation, policy, approval, and audit tests on clean merged `main` at `9fa095e`.
- Planning changed documentation only and added no Rust, dependency, lockfile, provider, approval, audit, executor, IPC, persistence, network, credential, capability, CSP, packaging, permission, or user-visible path.
- Project-owner approval was received; the verified implementation evidence follows.

## Increment 4C capability and evidence

- The raw `ToolCallProposal`, unused provider tool-call response variant, caller-supplied `PolicyContext`, and independently constructed `ProposedAction` are removed.
- `PolicyInput` can be constructed only by consuming one `SchemaValidatedFunctionCall`; private policy values retain exact call ID, local name/version, typed arguments, risk, and required permission.
- `PolicyDecision` owns the exact evaluated input, derives its outcome from one closed `PolicyReason`, and redacts argument content from debug output.
- Prohibited and external/high-impact classes deny first; remaining permission-bearing and read-only classes deny; reversible and personal-data classes require approval; only information-only/no-permission calls allow as non-authorizing data.
- Focused tests passed: three mock-provider, six function-call validation, four policy rule-table, and two public policy-input binding tests.
- `npm run verify` passed with 124 frontend tests, 78 Rust library tests, eight Rust integration tests, TypeScript, production frontend builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reported zero vulnerabilities; diff checks, code review, and security review passed with no findings.
- No native interaction gate was required because the modules remain transport-free and unreferenced by Tauri.
- No dependency, lockfile, approval, audit, executor, network, credential, IPC, persistence, Tauri, capability, CSP, packaging, permission, or user-visible path was added.
- D-023 records the durable ownership and conservative-evidence policy boundary.

## Increment 4D planning result

- At planning start, accepted gateway frames verified run and gateway-request IDs, but those identities were dropped before schema validation and policy.
- The previous approval scaffold accepted detached caller-authored tool names, action hashes, and previews; its records were clonable, never expired, lacked cancellation and replay binding, and had no production caller.
- The generic audit scaffold remains detached because arbitrary string details cannot safely represent exact approval evidence or raw personal content.
- The proposed increment carries validator-owned run/request/call identity through `SchemaValidatedFunctionCall` and the existing input-retaining `PolicyDecision`.
- Approval creation consumes only an exact `RequireApproval` decision. A borrowed closed preview is derived from the same retained `create_local_task@1` typed arguments and local metadata.
- The proposed manager allows one pending approval and 1,024 subjects per lifetime, owns a relative 120-second monotonic deadline, and consumes approve, reject, cancel, or expiry exactly once while rejecting duplicate subject identities without eviction. Future orchestration must cancel approval when its run terminates.
- Request, preview, and resolution values remain non-cloneable, non-serializable, and debug-redacted; approved resolution has no audit, dispatch, executor, IPC, persistence, or provider-continuation conversion.
- The plan removes the existing `action_hash` and adds no digest or new dependency. Exact in-process ownership is the canonical binding; approval IDs and any future digest remain non-authorizing correlation data.
- Planning baseline passed TypeScript, 17 gateway-protocol tests, six function-call validation tests, four policy tests, three approval tests, four audit tests, and two public policy-binding tests on clean merged `main` at `55626b6`.
- Planning changed documentation only. The project owner approved the exact plan and five-file runtime/test list; the verified implementation evidence follows.

## Increment 4D capability and evidence

- Accepted function calls retain validator-owned run and gateway-request IDs through local schema validation, policy, approval request, borrowed preview, and terminal resolution.
- Content-bearing gateway calls and events are non-cloneable and use custom debug output that redacts raw arguments and output-text deltas.
- The approval manager consumes only one owned `RequireApproval` decision, derives the exact closed `create_local_task@1` preview from retained typed arguments, and rejects information-only or unsupported subjects.
- Caller-authored tool names, action hashes, preview strings, policy outcomes, creation times, deadlines, and detached records are removed from the approval boundary. No digest or dependency was added.
- One manager permits one pending request and at most 1,024 distinct lifetime subjects, owns a relative 120-second monotonic deadline, and makes approve, reject, cancel, and expiry one-time terminal outcomes with non-evicting replay tombstones.
- Request views, previews, and resolutions are non-cloneable, non-serializable, and debug-redacted. Resolution exposes no consuming path to policy input, audit, dispatch, IPC, or execution.
- `Approved` proves only that the local transport-free manager processed a closed choice while the exact subject was pending and unexpired. It does not prove a user gesture, user presence, local authentication, run liveness, or execution eligibility.
- Focused final checks passed: 18 gateway-protocol tests, six function-call validation tests, four policy tests, six approval tests, two policy-input integration tests, and two approval-binding integration tests.
- `npm run verify` passed with 124 frontend tests, 82 Rust library tests, ten Rust integration tests, TypeScript, production frontend builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reported zero vulnerabilities after a sandboxed DNS failure was retried with network access. rustfmt, Clippy with warnings denied, diff checks, code review, and security review passed.
- No native interaction gate was required because these modules remain transport-free and unreferenced by Tauri or the UI.
- No provider, network, credential, audit, executor, IPC, persistence, Tauri, frontend, manifest, lockfile, capability, CSP, packaging, permission, or user-visible path was added.
- D-024 records the exact ownership, lifecycle, no-digest, and non-authorizing approval boundary.

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
