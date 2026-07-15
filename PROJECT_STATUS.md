# Project status

Last updated: 2026-07-15

## Current milestone

Phase 3 and Phase 4 Increments 4A through 4F are **verified complete on the target Mac**. Repository Workflow Increments 4G and 4J and Phase 4 Increments 4H through 4Q are **verified complete, published, and merged into `main`**. Increment 4R bind terminal initial function call to policy is **verified complete in the current uncommitted workspace** with no production caller or user-visible behavior change. No later implementation increment is Ready.

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
- Increment 4E: trusted approval-decision source - **verified complete on target Mac**.
- Increment 4F: Cortexa product display rename - **verified complete by project-owner direction**.
- Repository Workflow Increment 4G: automated post-increment gate - **verified complete**.
- Increment 4H: typed approval-audit adapter - **verified complete**.
- Increment 4I: remove generic audit scaffold - **verified complete after reconstruction**.
- Repository Workflow Increment 4J: deletion-stable post-increment fingerprint - **verified complete**.
- Increment 4K: remove legacy provider scaffold - **verified complete**.
- Increment 4L: remove legacy memory scaffold - **verified complete; published and merged**.
- Increment 4M: remove legacy platform scaffold - **verified complete; published and merged**.
- Increment 4N: bounded initial gateway request - **verified complete; published and merged**.
- Increment 4O: bound initial gateway turn - **verified complete; published and merged**.
- Increment 4P: schema-bound initial gateway events - **verified complete; published and merged**.
- Increment 4Q: terminally release initial function call - **verified complete; published and merged**.
- Increment 4R: bind terminal initial function call to policy - **verified complete; uncommitted**.

## Increment 4R capability and evidence

- The bound turn privately retains one `SchemaValidatedFunctionCall` after its
  non-terminal function frame returns `None`.
- The exact source/test scope changes only
  `src-tauri/src/agent/gateway_request.rs` and
  `src-tauri/tests/gateway_request_contract.rs`.
- Accepted terminal completion consumes the exact pending call through a
  locally selected `DeterministicPolicyEngine` and returns one closed
  `PolicyEvaluated` event. No caller can receive a standalone call or select the
  policy engine on that path.
- `get_current_datetime@1` remains `Allow` / `InformationOnly`, and
  `create_local_task@1` remains `RequireApproval` /
  `ReversibleRequiresApproval`. `Allow` remains non-authorizing.
- Function-frame withholding, failure/cancellation discard, transactional
  protocol errors, text behavior, local schema failure, status, limits, and
  redaction remain unchanged.
- Six request, 18 protocol, six function-validation, four policy, nine tool,
  nine public request-contract, two policy-input, and two approval-binding tests
  pass. Clippy, complete `npm run verify`, and npm audit also pass.
- Exact-scope, conflict, secret, generated-output, architecture, code-health,
  security, preserved-boundary, documentation, and mandatory gate reviews have
  no blocking finding. No manual verification applies because no production
  caller or user-visible behavior exists.
- D-039 records terminal initial-turn policy ownership, fixed deterministic
  engine selection, non-authorizing decisions, and public event API narrowing.
- The consolidated result is `PASS WITH ADVISORIES`; the `04r` completion marker
  is complete and valid for the current uncommitted workspace.
- No policy-rule, approval, native interaction, audit, dispatch, execution,
  tool-result, continuation, transport, gateway deployment, authentication,
  credential, runtime, Tauri, frontend, SQLite, dependency, capability,
  entitlement, or permission work is included.
- Exact risks, rollback, verification, and closeout evidence are documented in
  `docs/plans/04r-bind-terminal-initial-policy.md` and
  `docs/reviews/2026-07-15-04r-post-increment-review.md`.

## Increment 4Q capability and evidence

- `InitialGatewayTurn` now owns one private optional
  `SchemaValidatedFunctionCall` pending slot that is absent from debug output and
  inaccessible to callers.
- The exact source/test scope changes only
  `src-tauri/src/agent/gateway_request.rs` and
  `src-tauri/tests/gateway_request_contract.rs`.
- A valid non-terminal function frame returns `None` while status remains
  `Streaming`; accepted terminal completion takes and releases the exact typed
  call once.
- Gateway failure and successful local cancellation discard the pending call and
  make late release impossible. Transactional malformed, identity-mismatched,
  and out-of-sequence frames retain it privately for the correct contiguous
  terminal frame.
- Text events remain caller-visible and text terminal completion retains the
  closed `ResponseCompleted` event. Local schema rejection remains typed,
  redacted, and terminal without populating the pending slot.
- Lower-level protocol, registry, function-validation, policy, approval, and
  audit APIs remain unchanged and independently testable.
- Six request tests, 18 protocol tests, six function-validation tests, nine tool
  tests, nine public request-contract tests, and two policy-binding tests pass.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library,
  and 20 Rust integration tests plus lint, typecheck, builds, and Tauri release
  no-bundle. Clippy passes with warnings denied, and the network-enabled npm audit
  reports zero vulnerabilities.
- Exact-scope, conflict, secret, generated-output, complete-diff, architecture,
  code-health, security, preserved-boundary, and documentation reviews have no
  blocking finding. No manual verification applies because no production caller
  or user-visible behavior exists.
- D-038 records terminal pending-call ownership, failure/cancellation discard,
  protocol-error retention, and the intentional optional-event API narrowing.
- The consolidated result is `PASS WITH ADVISORIES`; the `04q` completion marker
  is complete and valid for the current uncommitted workspace.
- No networking, gateway deployment, authentication, credentials, provider
  parameters, continuation, retries, deadlines, coordinator, policy, approval,
  audit writes or persistence, dispatch, execution, Tauri, frontend, SQLite,
  dependency, capability, entitlement, or permission work is included.

## Increment 4P capability and evidence

- `InitialGatewayTurn` now owns one private exact `InMemoryToolRegistry` built
  from the same fixed two-schema catalog used to configure response validation.
- Its closed `InitialGatewayEvent` exhaustively mirrors normalized initial events
  but exposes function calls only as `SchemaValidatedFunctionCall`; no raw
  `UntrustedFunctionCall` leaves the bound turn.
- `InitialGatewayTurnError` distinguishes typed protocol rejection from typed
  local function-call validation rejection without retaining untrusted content.
- Local schema rejection sets private terminal state, reports `Failed`, rejects
  every late frame as already terminal, and makes later cancellation a no-op
  without claiming gateway/provider failure or transport abort.
- The exact source/test scope changes only
  `src-tauri/src/agent/gateway_request.rs` and
  `src-tauri/tests/gateway_request_contract.rs`.
- Lower-level protocol, registry, function-validation, policy, approval, and
  audit APIs remain unchanged and independently testable.
- Six request tests, 18 protocol tests, six function-validation tests, nine tool
  tests, eight public request-contract tests, and two policy-binding tests pass.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library,
  and 19 Rust integration tests plus lint, typecheck, builds, and Tauri release
  no-bundle. Clippy passes with warnings denied, and the network-enabled npm audit
  reports zero vulnerabilities.
- Exact-scope, conflict, secret, generated-output, complete-diff, architecture,
  code-health, security, preserved-boundary, and documentation reviews have no
  blocking finding. No manual verification applies because no production caller
  or user-visible behavior exists.
- D-037 records exact registry ownership, terminal local-schema failure, and the
  intentional public event/error API narrowing. The result is `PASS WITH
ADVISORIES` for that theoretical unsupported external consumer and the
  intentionally public lower-level raw protocol boundary.
- No HTTP, gateway deployment, authentication, credentials, provider parameters,
  continuation, retries, deadlines, coordinator, policy, approval, audit writes
  or persistence, dispatch, execution, Tauri, frontend, SQLite, dependency,
  capability, entitlement, or permission work is included.

## Increment 4O capability and evidence

- The verified initial request and gateway stream validator currently accept the
  same correlation and tool-contract concerns through separate public
  constructors.
- No production caller exists, but a future caller could pair valid request bytes
  with a validator configured for different run/request IDs, function names, or
  tool-contract version.
- Repository search found no existing bound-turn abstraction. The implementation
  adds the smallest local wrapper inside `agent/gateway_request.rs`.
- The exact source/test scope changes only
  `src-tauri/src/agent/gateway_request.rs` and
  `src-tauri/tests/gateway_request_contract.rs`.
- `InitialGatewayTurn` derives request bytes and validator state from the same IDs
  and exact `ToolSchema` catalog, exposing only borrowed request bytes, status,
  frame acceptance, and local cancellation.
- Raw initial-request construction is private. The existing lower-level
  public `GatewayStreamValidator::new` remains available for protocol fixtures
  and existing downstream tests.
- Six preserved request tests, 18 protocol tests, nine tool tests, and six public
  bound-turn contract tests pass. Clippy passes with warnings denied.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library,
  and 17 Rust integration tests plus lint, typecheck, Vite builds, and Tauri
  release no-bundle. The network-enabled npm audit reports zero vulnerabilities.
- Exact source, closeout, conflict, secret, generated-output, architecture,
  code-health, security, and complete-diff reviews have no blocking finding. No
  manual interaction gate applies because no production caller exists.
- No HTTP, gateway deployment, authentication, credentials, provider parameters,
  continuation, retries, deadlines, coordinator, policy, approval, audit
  persistence, dispatch, execution, Tauri, frontend, SQLite, dependency,
  capability, entitlement, or permission work is included.
- D-036 records the bound initial-turn and public API narrowing decision. The
  lower-level validator remains public for protocol fixtures, and future initial
  transport code must use the bound turn.
- The consolidated result is `PASS WITH ADVISORIES`; advisories are the
  theoretical unsupported external consumer of the narrowed request API and the
  intentionally public lower-level validator boundary. They block neither 4O
  completion nor separately approved later planning.

## Increment 4N capability and evidence

- The verified gateway response protocol defines normalized inbound events and
  conservative constants but no outbound desktop-to-gateway request envelope.
- D-021 requires a closed request with protocol version, opaque run/request
  identity, bounded user-selected content, a fixed server-recognized tool-set,
  and fixed limits before authenticated transport is considered.
- The implementation is an initial-turn-only transport-free Rust request value,
  not HTTP, authentication, credentials, continuation, or orchestration.
- Exact source/test scope is `agent/gateway_request.rs`, sibling-only opaque-ID
  validator visibility in `gateway_protocol.rs`, one `agent/mod.rs` export, and
  `tests/gateway_request_contract.rs`.
- The request is non-cloneable, debug-redacted, serialized through private closed
  wire types, and checked against the 64 KiB limit after JSON escaping.
- The fixed tool-set identity is gateway correlation/authorization input only;
  it grants no local policy, approval, audit, dispatch, or execution authority.
- O-006 and O-007 still block live traffic. Networking, gateway deployment,
  credentials, Keychain, provider SDKs/parameters, model selection, continuation,
  retry/cancellation orchestration, context selection, runtime coordination,
  Tauri, frontend, SQLite, dependencies, capabilities, and permissions remain
  excluded.
- Six focused request tests, 18 preserved gateway tests, nine tool tests, and one
  public-boundary integration test pass. Clippy passes with warnings denied.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library,
  and 12 Rust integration tests plus lint, typecheck, frontend builds, and Tauri
  release no-bundle. The network-enabled npm audit reports zero vulnerabilities.
- Exact source, closeout, conflict, secret, generated-output, architecture,
  code-health, security, and complete-diff reviews have no blocking finding. No
  manual interaction gate applies because no production caller exists.
- D-035 records the fixed closed request boundary. O-006 and O-007 still block
  authenticated gateway transport and live provider traffic.

## Increment 4M capability and evidence

- The legacy Rust platform module exposes arbitrary-string metadata and a public
  mock builder that can assign `Available`, `Disabled`, or `Unavailable` to broad
  capabilities without authoritative operating-system evidence.
- Its status has no resource scope, provenance, observation time, freshness,
  requestability, user-initiation evidence, dependent feature, last-use time, or
  capability-specific failure semantics.
- Repository search finds no caller outside the platform module and its three
  embedded tests. The only external reference is `pub mod platform;` in
  `src-tauri/src/lib.rs`.
- The typed app-info API, public metadata smoke test, and fixed frontend Permission
  Center are independent and pass focused baseline checks.
- The implementation deletes `src-tauri/src/platform/adapter.rs`,
  `src-tauri/src/platform/mod.rs`, and `src-tauri/src/platform/types.rs`, then
  removes only `pub mod platform;` from `src-tauri/src/lib.rs`.
- No replacement adapter, OS query, native framework, permission request,
  Keychain, LocalAuthentication, resource scope, IPC, UI, dependency, Tauri
  capability, entitlement, or operating-system permission is included.
- Planning baseline on clean synchronized `main` at `ecd49be` passed typecheck,
  three legacy platform tests, the app-info unit test, public metadata smoke test,
  and focused Permission Center test. The `04l` marker was complete and valid
  before documentation edits.
- Focused implementation verification passes with rustfmt, the app-info unit test,
  public metadata smoke test, focused Permission Center test, Clippy with warnings
  denied, and the required no-match legacy-symbol scan.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 86 Rust library,
  and 11 Rust integration tests plus lint, typecheck, Vite builds, and Tauri
  release no-bundle. The network-enabled npm audit reports zero vulnerabilities.
- Exact source, closeout, conflict, secret, generated-output, architecture,
  code-health, security, and complete-diff reviews have no blocking finding. No
  manual interaction gate applies because no production or user-visible path
  changed.
- D-034 preserves future capability-specific adapters and authoritative, scoped,
  fresh permission evidence while prohibiting restoration of caller-authored
  generic status as an authority boundary.
- The consolidated result is `PASS WITH ADVISORIES`; advisories are the
  intentionally deferred future platform design and theoretical unsupported
  external consumer of the removed public scaffold. They block neither completion
  nor later bounded planning.

## Increment 4L capability and evidence

- The legacy Rust memory module exposes public clonable records containing
  arbitrary title, content, and source strings and retains them in an unbounded
  in-memory map.
- Its six-marker substring check is not a complete sensitive-data policy. The
  types omit required opt-in, creation time, optional expiration, visibility,
  export, encryption, retention, and authoritative provenance semantics.
- Repository search finds no caller outside the memory module and its three
  embedded tests. Historical Increment 2A records remain unchanged.
- The verified SQLite bootstrap storage module is independent, has 13 passing
  focused tests, and intentionally persists no user memory before reviewed
  encryption and repository contracts exist.
- The implementation deletes `src-tauri/src/memory/mod.rs`,
  `src-tauri/src/memory/store.rs`, and `src-tauri/src/memory/types.rs`, then
  removes only `pub mod memory;` from `src-tauri/src/lib.rs`.
- No replacement memory types, repository, migration, encryption, Keychain,
  context selection, persistence, IPC, UI, dependency, capability, or permission
  is included.
- Planning baseline on clean synchronized `main` at `5415444` passed typecheck,
  three legacy memory tests, 13 storage tests, and both public storage smoke
  tests. The `04k` marker was complete and valid before documentation edits.
- Focused implementation verification passes with rustfmt, 13 storage unit tests,
  both public storage smoke tests, Clippy with warnings denied, and the required
  no-match legacy-symbol scan.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 89 Rust library,
  and 11 Rust integration tests plus lint, typecheck, Vite builds, and Tauri
  release no-bundle. The network-enabled npm audit reports zero vulnerabilities.
- Exact source, closeout, conflict, secret, generated-output, architecture,
  code-health, security, and complete-diff reviews have no blocking finding. No
  manual interaction gate applies because no production or user-visible path
  changed.
- D-033 preserves future bounded, opt-in, provenance-aware, encrypted,
  user-controlled memory and prohibits restoration of the deleted arbitrary-content
  API as a convenience boundary.
- The consolidated result is `PASS WITH ADVISORIES`; the advisory is the
  theoretical unsupported external consumer of the removed public scaffold. It
  blocks neither completion nor later bounded planning.

## Increment 4K capability and evidence

- The legacy `agent::provider` module exposes a synchronous `complete` trait,
  arbitrary-string request fields, arbitrary assistant text, and arbitrary mock
  failure strings. `agent::types` exists only for that scaffold.
- Repository search finds no caller outside those two files and their three
  embedded unit tests. Historical backup documentation is not executable code.
- The verified `agent::gateway_protocol` and
  `agent::function_call_validation` modules are independent and remain the only
  approved normalized provider-event and exact local call-validation boundaries.
- The implementation deletes `src-tauri/src/agent/provider.rs` and
  `src-tauri/src/agent/types.rs` and removes only their exports from
  `src-tauri/src/agent/mod.rs`.
- No replacement provider, request contract, transport, networking, credential,
  Keychain, coordinator, dispatch, executor, persistence, IPC, UI, dependency,
  capability, or permission is included.
- Planning baseline on clean synchronized `main` at `99f9279` passed typecheck,
  three legacy provider tests, 18 normalized gateway-protocol tests, six exact
  function-call validation tests, and two public gateway-to-policy tests. The
  corrected `04i` marker was complete and valid before documentation edits.
- Focused implementation checks pass with 18 gateway-protocol tests, six exact
  function-call validation tests, two public gateway-to-policy tests, rustfmt,
  Clippy with warnings denied, and the required no-match stale-symbol scan.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library,
  and 11 Rust integration tests plus lint, typecheck, Vite builds, and Tauri
  release no-bundle. The network-enabled npm audit retry reports zero
  vulnerabilities.
- Exact source, closeout, conflict, secret, generated-output, architecture,
  code-health, security, and complete-diff reviews have no blocking finding. No
  manual interaction gate applies because no production or user-visible path
  changed.
- D-032 records that future provider transport requires separately approved
  closed bounded request and normalized event contracts rather than restoration
  of the deleted synchronous arbitrary-string API.
- The consolidated result is `PASS WITH ADVISORIES`; the advisory is the
  theoretical unsupported external consumer of the removed public scaffold. It
  blocks neither completion nor later bounded planning.

## Increment 4I capability and evidence

- `src-tauri/src/audit/logger.rs` and `src-tauri/src/audit/types.rs` are deleted, and `audit::mod` exports only the verified typed `approval` module.
- Repository search found no production or integration caller before deletion, and the final stale-symbol scan returns no matches in current Rust source or tests.
- The typed approval adapter is unchanged. Six typed-adapter tests, eleven native-source tests, and one public approval-audit integration test pass.
- The reconstructed implementation is committed as `99f9279`, pushed on `codex/phase4-increment-4i`, fast-forward merged into `main`, and synchronized with `origin/main`.
- The original implementation commit remains preserved at `cf9d701` on local `codex/phase4-increment-4i-pre-fingerprint-fix`; no remote ref contains that pre-fingerprint commit.
- D-030 records that future trusted audit event families require separately approved closed typed contracts rather than restoration of an arbitrary-string API.
- No replacement audit abstraction, dependency, lockfile, migration, persistence, coordinator, dispatch, executor, IPC, UI, networking, credential, capability, or permission changed.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 95 Rust library, and 11 Rust integration tests plus lint, typecheck, builds, and Tauri release no-bundle. Npm audit reports zero vulnerabilities.
- Complete scope, diff, architecture, code-health, security, secret, generated-output, and conflict reviews have no blocking finding. The corrected 04i marker is complete and valid.

## Repository Workflow Increment 4J capability and evidence

- The clean synchronized baseline is `main` at `e3af5a4`; the implementation branch is `codex/repository-workflow-increment-4j`.
- The pre-fix 4I commit remains preserved exactly at `cf9d701` on `codex/phase4-increment-4i-pre-fingerprint-fix`. Its marker became invalid only after its reviewed tracked deletions were committed, motivating 4J.
- The corrected fingerprint omits paths absent from the current workspace while preserving path, executable-bit, type, regular-file content, symlink-target, path-safety, and fail-closed I/O handling for existing paths.
- Exact changed-file and report-inventory validation remains unchanged, so reviewed deletions must still be recorded before finalization.
- The positive regression proves a reviewed tracked deletion remains valid after commit. The negative regression proves deleting a tracked file after finalization invalidates the marker.
- Focused Python and npm hook checks pass with 17 tests; the pre-edit baseline passed with 15 tests.
- D-031 records existing-content snapshot semantics and rejects legacy-marker fallback or silent migration.
- No application source, dependency, hook configuration, skill, Tauri, IPC, storage, provider, gateway, approval, audit, dispatch, executor, capability, or permission changed.
- Complete repository verification passes with 17 hook, 124 frontend, 99 Rust library, and 11 Rust integration tests plus Clippy, builds, and Tauri release no-bundle. The dependency audit reports zero vulnerabilities.
- Exact scope, secret, generated-output, complete-diff, code, and security reviews pass. The consolidated result is `PASS WITH ADVISORIES`; 4J is now published and merged, satisfying the prerequisite for fresh 4I reconstruction.

## Increment 4H capability and evidence

- The clean planning baseline is `codex/phase4-increment-4h` at merged `main` commit `74692c1`.
- The generic audit scaffold accepts arbitrary event, summary, and details strings, applies only token-pattern redaction, is unbounded, and has no production caller.
- `ApprovalResolution` already exposes the exact opaque identity, locally derived tool and policy facts, terminal disposition, and closed optional interaction evidence required for a content-free record. The planned adapter must never call its title-bearing `preview()` accessor.
- The adapter is bounded to 1,024 in-memory records, one record per exact approval/run/request/call key, checked deterministic sequencing, no eviction, no arbitrary event strings, and no serialization or authority conversion.
- Exact subject metadata and the complete Approved/Rejected/Edit/native-no-decision/source-failure/run-termination/expiry evidence matrix are revalidated before mutation. Missing, extra, contradictory, duplicate, over-capacity, and sequence-overflow states fail closed.
- `ApprovalAuditRecord` never calls or stores the title-bearing preview. Its custom debug output redacts identity, and typed errors expose only fixed variants and the fixed capacity.
- The exact runtime/test scope creates `src-tauri/src/audit/approval.rs` and `src-tauri/tests/approval_audit_binding.rs`, exports the module from `src-tauri/src/audit/mod.rs`, and changes only test coverage in `src-tauri/src/approvals/decision_source.rs`.
- Focused checks pass with six adapter tests, eleven native-source tests, and one new public-boundary integration test.
- `npm run verify` passes with 15 hook tests, 124 frontend tests, 99 Rust library tests, 11 Rust integration tests, lint, typecheck, Vite builds, and the Tauri release no-bundle build. The network-enabled npm audit retry reports zero vulnerabilities.
- Complete scope, secret, generated-output, architecture, code-health, and security reviews have no blocking finding. No manual interaction gate applies because no production native-dialog or shipping-app behavior changed.
- D-029 records the typed, redacted, bounded, non-durable, and non-authorizing adapter boundary. The generic audit scaffold remains disconnected and non-production.
- Persistence, runtime orchestration, dispatch, execution, IPC, UI, gateway networking, credentials, capabilities, permissions, manifests, and lockfiles remain unchanged.

## Workflow Increment 4G current evidence

- Clean, synchronized `main` at `a4ab51f` was the implementation baseline; the implementation branch was `codex/post-increment-gate`.
- One repository-local Stop hook, Python standard-library validator, consolidated review skill, report schema/template, ignored state marker, and focused test suite are implemented.
- The validator uses fixed Git argument arrays, bounded JSON/report input, safe repository-relative path checks, merge-conflict and suspicious-path rejection, exact changed-file evidence, report hashing, and a deterministic workspace-content fingerprint.
- Fifteen focused hook tests pass, including missing/failed/pending/passing evidence, marker validity after commit, stale workspace rejection, report re-finalization, parent-symlink escapes, suspicious paths, merge conflicts, malformed input, and `stop_hook_active` loop prevention.
- Direct active-state Stop evaluation emits the exact required continuation prompt; the loop-guard case emits no continuation output.
- No application source, dependency, lockfile, Tauri, Rust, frontend, database, capability, permission, provider, gateway, approval, audit, or execution path changed.
- Full post-documentation `npm run verify` passes with 15 hook, 124 frontend, 92 Rust library, and ten Rust integration tests plus formatting, ESLint, Clippy, typecheck, frontend build, and Tauri release no-bundle build.
- Exact 24-file scope, no-application-source, secret, generated-output, complete-diff, code, and security reviews pass after the parent-symlink escape correction.
- The project owner passed normal `/hooks` trust and live active-state Stop confirmation.
- The consolidated report result is `PASS WITH ADVISORIES`; the advisory is the documented operator-controlled hook trust/bypass boundary. The deterministic completion marker is complete and valid.

## Increment 4F current evidence

- The pre-edit working tree was clean on `main` at `8e174a7`, ahead of `origin/main` by one repository-workflow commit.
- `npm run build` passed before rename edits.
- All 37 tracked exact former product-name occurrences were reviewed and replaced with `Cortexa`.
- Tauri product/window metadata, typed Rust app information, native dialog title, menu labels/tooltip, fixed startup diagnostics, sidebar branding, tests, prompts, skills, and documentation now use `Cortexa`.
- Settings requires no direct component change because it renders the typed Rust `AppInfo.name`; focused React coverage now expects `Cortexa` there.
- Repository/package/crate/executable/bundle-ID/database/storage/event/command identifiers remain intentionally unchanged under D-026.
- Every requested automated command passed after one targeted Prettier correction. Focused checks passed with 25 React app, one app-info, nine menu-bar, 16 approval, and one smoke test; complete checks passed with 124 frontend, 92 Rust library, and ten Rust integration tests.
- The target-Mac app launch and project-owner window, application-menu, status-item menu, sidebar, Settings, regression, and no-permission-prompt checks passed.
- Exact 43-file scope, former-name absence, compatibility, Cargo target metadata, secret, generated-output, complete-diff, code-review, and security-review checks passed.
- The referenced `$post-increment-gate` skill and report workflow are not present in `.agents/skills`, so that gate did not run and no result is claimed. The project owner explicitly confirmed 4F complete and deferred skill creation to the next clean branch. D-027 records the one-time sequencing exception.
- Implementation commit `972a874` was pushed on `phase4/increment-4f`, fast-forward merged into `main`, and pushed to `origin/main`.

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

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4M. Do not start later planning or implementation.

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

## Increment 4E planning result

- The Increment 4D manager has no production caller and still accepts a raw public `ApprovalChoice`; this cannot prove a trusted interaction source.
- The React approval dialog remains mock-only untrusted WebView state. `get_app_info` is still the only custom Tauri command, and `core:default` is still the only capability permission.
- The smallest coherent source is one Rust-owned macOS native message dialog consuming a manager-issued, non-cloneable, owned presentation. No WebView input or Tauri dialog plugin is added.
- The presentation carries a private in-memory manager-instance marker, exact approval/run/request/call identity, closed local preview facts, and one bounded transient application-owned title clone. The marker moves into the sealed outcome and is pointer-checked before public IDs, preventing cross-manager substitution without a digest, randomness, dependency, or content. The source message and native renderer may briefly hold additional bounded transient title copies; no title survives in the source outcome or enters logs, errors, debug output, audit, or persistence.
- The native message is capped at 1,024 Unicode scalar values, places all trusted fixed facts before the final untrusted title row, and rejects the plan's exact closed zero-width/default-ignorable/line/bidirectional presentation set while allowing ordinary non-ASCII text.
- Reject is the proposed first/default native button; Return, Escape, close, no-decision, and failure paths must not approve.
- The sealed native source outcome carries exact identity, a closed Approve/Reject/Edit/NativeNoDecision/SourceFailed result, source kind, and `NotEvaluated` authentication evidence. The manager rechecks identity, pending state, one-shot issuance, cancellation, and monotonic expiry before resolution.
- Approve maps to `Approved`; Reject maps to `Rejected`; Edit, native no-decision, and source failure map to typed terminal cancellation. The dependency's `Cancel` result makes no user-intent claim. Edit requires a fresh gateway call, schema validation, policy decision, approval ID, and presentation.
- Run cancellation and expiry consume the subject while a prompt is outstanding, and every late or replayed source outcome fails closed. The proposed dialog library cannot programmatically close a stale visible prompt, so live orchestration remains out of scope and that UX risk must be revisited before integration.
- LocalAuthentication is deferred. The current `create_local_task@1` subject is reversible and requires no permission; native interaction does not claim actor identity, biometric, device-owner authentication, run liveness, or execution eligibility.
- A future typed audit adapter may receive opaque identity, source, authentication evidence, an optional recognized button, source-failure code, cancellation reason, and disposition fields, but not the raw title. Increment 4E makes no audit call.
- The proposed exact macOS-target dependency is `rfd = "=0.17.2"` with default features disabled. Direct use avoids registering Tauri dialog invoke commands or WebView permissions; file-dialog APIs remain unused and unexposed.
- `cargo-audit` is not installed or configured. The implementation gate uses exact version 0.22.2 from temporary storage with network approval and cannot complete without a clean reviewed RustSec lockfile result.
- The exact runtime list creates two files and changes six files. It does not include `lib.rs`, frontend, Tauri configuration, capability, CSP, policy, schema, registry, audit, executor, provider, gateway, or storage files.
- Planning baseline passed TypeScript, six approval library tests, two approval-binding integration tests, three platform tests, nine menu-bar tests, and three menu-bar-routing integration tests on clean merged `main` at `1cf190f`.
- Planning changed documentation only and added no runtime, dependency, lockfile, Tauri, frontend, permission, audit, execution, persistence, provider, network, or credential path.

## Increment 4E capability and evidence

- `ApprovalChoice` and the public raw-choice manager path are removed. One non-cloneable manager-issued `ApprovalPresentation` is now the only input to the macOS source, and one sealed non-cloneable `TrustedApprovalSourceOutcome` is the only source-backed manager-resolution input.
- A private pointer-identical manager marker plus exact approval/run/gateway-request/function-call identity and manager-retained issuance state prevent cross-manager or cross-subject substitution. Issuance does not extend the 120-second TTL, and one-pending, 1,024-subject, cancellation, expiry, and non-evicting replay behavior remain intact.
- The macOS-only source uses fixed `Cortexa approval` title and Reject-first Approve/Reject/Edit buttons. It displays trusted facts before the final affected-title row, rejects the exact planned presentation-format set, allows ordinary non-ASCII text, and caps the complete message at 1,024 Unicode scalar values.
- Approve and Reject map directly. Edit, native no-decision, source failure, run termination, and expiry are closed terminal outcomes. Only recognized buttons carry button evidence; all source outcomes record `NotEvaluated` authentication and grant no actor-identity, run-liveness, dispatch, or execution authority.
- Exact macOS-target `rfd = "=0.17.2"` is added with default features disabled. Source, MIT license, resolved target feature and duplicate trees, lockfile, AppKit/native `unsafe` boundary, and compatibility were reviewed. The lockfile diff adds only `rfd`; the application exposes no raw handle, file-dialog API, dependency object, Tauri plugin, WebView route, or application `unsafe`.
- The standalone main-thread example exercises the public gateway -> schema -> policy -> approval -> native source -> resolution path, prints only bounded approval identity and a fixed terminal label, and performs no action or persistence.
- Focused approval checks pass: 16 library tests and two approval-binding integration tests. rustfmt, Clippy with warnings denied, and `npm run verify` pass with 124 frontend tests, 92 Rust library tests, ten Rust integration tests, TypeScript, Vite production builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reports zero vulnerabilities after the sandboxed DNS failure was retried with network access. Dependency/scope review, stale-symbol search, code review, and security review found no 4E runtime boundary defect.
- Exact temporary `cargo-audit 0.22.2` ran and exited nonzero on RUSTSEC-2026-0194 and RUSTSEC-2026-0195 in pre-existing `quick-xml 0.39.4`. The path is existing `plist 1.9.0 -> Tauri`; source review found plain `quick_xml::Reader`, not `NsReader`, and no attribute iteration on that path. D-025 records the project owner's scoped reviewed baseline exception. No advisory ignore or dependency upgrade was added.
- The target-Mac manual gate passes. Approve -> `approved`, Reject -> `rejected`, Edit -> `cancelled: edit requested`, and Return/default -> `rejected`; Escape had no effect and no window-close control was available. The project owner confirmed fixed title/content order, post-resolution terminal redaction, no action or persistence, and no permission prompt.
- No shipping Tauri wiring, frontend, capability, CSP, LocalAuthentication, audit, storage, dispatch, executor, tool implementation, provider continuation, network, credential, identity, or permission path was added.
- D-025 records the native-source boundary, exact dependency, no-authentication claim, stale-visible-dialog limitation, and scoped RustSec baseline exception. Increment 4E is verified complete.

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
