# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

No implementation plan is active. Increment 4O is verified complete in the
current uncommitted workspace, and no later increment is Ready.

## Completed plans

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
docs/plans/02c-storage-startup.md
docs/plans/02d-menu-bar-window-lifecycle.md
docs/plans/02e-react-application-shell.md
docs/plans/02f-mocked-assistant-interaction-shell.md
docs/plans/02g-integration-hardening.md
docs/plans/03a-in-memory-conversation-sessions.md
docs/plans/03b-mock-context-provenance.md
docs/plans/03c-simulated-tool-result.md
docs/plans/03d-bounded-mock-loop-completion.md
docs/plans/04a-gateway-protocol-contract.md
docs/plans/04b-local-tool-schema-validation.md
docs/plans/04c-trusted-policy-input-binding.md
docs/plans/04d-exact-approval-binding.md
docs/plans/04e-trusted-approval-decision-source.md
docs/plans/04i-remove-generic-audit-scaffold.md
docs/plans/04j-post-increment-deletion-fingerprint.md
docs/plans/04k-remove-legacy-provider-scaffold.md
docs/plans/04l-remove-legacy-memory-scaffold.md
docs/plans/04m-remove-legacy-platform-scaffold.md
docs/plans/04n-bounded-initial-gateway-request.md
docs/plans/04o-bound-initial-gateway-turn.md
```

Increments 2C and 2D were verified on the Apple Silicon target Mac.

## Plan rules

A plan must contain:

- Goal and user-visible outcome.
- Scope and explicit non-goals.
- Existing behavior and constraints.
- Files expected to change.
- Ordered implementation steps.
- Security and privacy considerations.
- Tests and verification commands.
- Rollback or failure strategy.
- Exit criteria.
- Documentation updates.

## Plan status values

- **Draft** — still being designed.
- **Ready** — enough information exists to implement.
- **Active** — implementation is in progress or verification remains.
- **Blocked** — a prerequisite prevents progress.
- **Complete** — acceptance criteria and verification are complete.
- **Superseded** — replaced by another plan.

## Plan index

| Plan                                          | Status   | Owner              | Last updated |
| --------------------------------------------- | -------- | ------------------ | ------------ |
| Increment 2B-1 SQLite migration skeleton      | Complete | Project maintainer | 2026-07-13   |
| Increment 2C storage startup integration      | Complete | Project maintainer | 2026-07-13   |
| Increment 2D menu-bar/window lifecycle        | Complete | Project maintainer | 2026-07-13   |
| Increment 2E React application shell          | Complete | Project maintainer | 2026-07-13   |
| Increment 2F mocked interaction shell         | Complete | Project maintainer | 2026-07-13   |
| Increment 2G integration hardening            | Complete | Project maintainer | 2026-07-13   |
| Increment 3A in-memory conversation sessions  | Complete | Project maintainer | 2026-07-13   |
| Increment 3B mock context provenance          | Complete | Project maintainer | 2026-07-13   |
| Increment 3C simulated tool result            | Complete | Project maintainer | 2026-07-13   |
| Increment 3D bounded mock-loop completion     | Complete | Project maintainer | 2026-07-14   |
| Increment 4A gateway protocol contract        | Complete | Project maintainer | 2026-07-14   |
| Increment 4B local tool-schema validation     | Complete | Project maintainer | 2026-07-14   |
| Increment 4C trusted policy-input binding     | Complete | Project maintainer | 2026-07-14   |
| Increment 4D exact approval binding           | Complete | Project maintainer | 2026-07-14   |
| Increment 4E trusted approval decision source | Complete | Project maintainer | 2026-07-14   |
| Increment 4F Cortexa product display rename   | Complete | Project maintainer | 2026-07-14   |
| Workflow Increment 4G post-increment gate     | Complete | Project maintainer | 2026-07-14   |
| Increment 4H typed approval-audit adapter     | Complete | Project maintainer | 2026-07-15   |
| Increment 4I remove generic audit scaffold    | Complete | Project maintainer | 2026-07-15   |
| Workflow Increment 4J deletion fingerprint    | Complete | Project maintainer | 2026-07-15   |
| Increment 4K remove legacy provider scaffold  | Complete | Project maintainer | 2026-07-15   |
| Increment 4L remove legacy memory scaffold    | Complete | Project maintainer | 2026-07-15   |
| Increment 4M remove legacy platform scaffold  | Complete | Project maintainer | 2026-07-15   |
| Increment 4N bounded initial gateway request  | Complete | Project maintainer | 2026-07-15   |
| Increment 4O bound initial gateway turn       | Complete | Project maintainer | 2026-07-15   |

## Phase 4 Increment 4O bound initial gateway turn - complete

Goal: bind the verified initial request bytes and response validator into one
non-cloneable, transport-free Rust turn so a future trusted caller cannot
independently choose request/response correlation IDs, allowed function names,
or tool-contract version.

The exact source/test plan changes only `agent/gateway_request.rs` and the public
`gateway_request_contract` integration test. `InitialGatewayTurn` derives the
response validator from the same IDs used for request serialization and from the
exact `ToolSchema` catalog. It exposes only borrowed request bytes, stream status,
frame acceptance, and local cancellation. Raw initial-request construction
becomes private; the lower-level public stream validator remains unchanged.

No transport, gateway service, authentication, credentials, Keychain, provider
parameters, continuation, retries, deadlines, runtime coordinator, policy,
approval, audit persistence, dispatch, execution, Tauri, frontend, SQLite,
dependency, capability, entitlement, or permission path is included. Focused
request, protocol, tool, and public-contract baselines pass on clean synchronized
`main` at `d7c4b69`; the `04n` marker was complete and valid before planning
edits.

The verified implementation keeps request serialization private, derives the
exact two local function names and common version from `ToolSchema`, and delegates
status, normalized frame acceptance, and local terminal cancellation to the owned
validator. Six preserved request tests and six public turn-contract tests pass,
along with 18 protocol tests, nine tool tests, Clippy with warnings denied,
complete `npm run verify`, npm audit, exact-scope, code, security, documentation,
and mandatory gate reviews. D-036 records the bound-turn and public API narrowing
decision. The implementation remains uncommitted, and no later increment is
Ready.

## Phase 4 Increment 4N bounded initial gateway request - complete

Goal: create one non-cloneable, transport-free Rust request value for the first
desktop-to-gateway turn. It validates existing opaque identity rules, preserves
one exact user-selected text value only in serialized bytes, fixes the tool-set
identity and all conservative limits, and enforces the 64 KiB bound after JSON
escaping.

The future source/test scope creates `agent/gateway_request.rs` and one public
integration test, changes `gateway_protocol.rs` only to share opaque-ID
validation with its sibling module, and adds one module export. It adds no HTTP,
gateway service, authentication, credentials, Keychain, provider SDK, model or
provider parameters, continuation, context selector, runtime coordinator, IPC,
UI, persistence, policy, approval, audit, dispatch, executor, dependency,
capability, or permission path.

The implementation passes six focused request tests, 18 preserved gateway
protocol tests, nine tool-catalog tests, one public-boundary integration test,
Clippy with warnings denied, complete `npm run verify`, npm audit, exact-scope,
secret, generated-output, code, security, documentation, and mandatory gate
reviews. D-035 records the request boundary. Commit `d7c4b69` is pushed on
`codex/phase4-increment-4n`, fast-forward merged into synchronized `main`, and
retained a valid `04n` marker before 4O planning edits.

## Phase 4 Increment 4M remove legacy platform scaffold - complete

Goal: delete the unused public generic `PlatformAdapter`, arbitrary-string
metadata, caller-authored capability status map, and broad capability/report types
before future permissions or native integration work can mistake them for
authoritative operating-system evidence.

Repository search finds no caller outside the three platform files and their
three embedded tests. The exact source plan deletes the complete
`src-tauri/src/platform/` module and removes only `pub mod platform;` from
`src-tauri/src/lib.rs`. The independent app-info IPC and fixed frontend Permission
Center remain unchanged.

The verified implementation adds no replacement adapter, native framework,
permission query/request, Keychain, LocalAuthentication, frontend state, IPC,
dependency, Tauri capability, entitlement, or operating-system permission. The
app-info unit test, public metadata smoke test, focused Permission Center test,
Clippy, complete `npm run verify`, npm audit, stale-symbol, exact-scope, security,
code-health, documentation, and mandatory gate reviews pass. D-034 preserves the
future capability-specific adapter and authoritative permission-evidence
requirements. Commit `1f03d1e` is pushed on
`codex/phase4-increment-4m`, fast-forward merged into synchronized `main`, and
retains a valid 04m marker.

## Phase 4 Increment 4L remove legacy memory scaffold - complete

Goal: delete the unused public `MemoryStore`, arbitrary-content input/update/record
types, unbounded in-memory map, and six-marker secret-like check before future
memory or persistence work can mistake them for the approved trusted boundary.

Repository search finds no caller outside the three memory files and their three
embedded tests. The exact source plan deletes the complete
`src-tauri/src/memory/` module and removes only `pub mod memory;` from
`src-tauri/src/lib.rs`. The 13-test typed SQLite bootstrap storage boundary
remains unchanged.

The verified implementation adds no replacement memory, repository, migration,
encryption, Keychain, context collection, persistence, IPC, UI, dependency,
capability, or permission. Thirteen storage unit tests, both public storage smoke
tests, Clippy, complete `npm run verify`, npm audit, stale-symbol, exact-scope,
security, code-health, documentation, and mandatory gate reviews pass. D-033
preserves the future bounded, opt-in, provenance-aware, encrypted memory
requirement. The implementation remains uncommitted and no later increment is
Ready.

## Phase 4 Increment 4K remove legacy provider scaffold - complete

Goal: delete the unused synchronous `AgentProvider`, arbitrary-string request,
assistant-response, and mock-error scaffold before a future gateway transport can
mistake it for the approved production provider boundary.

Repository search finds no caller outside the two legacy files and their three
embedded tests. The exact source plan deletes `src-tauri/src/agent/provider.rs`
and `src-tauri/src/agent/types.rs` and removes only their exports from
`src-tauri/src/agent/mod.rs`. The 18-test normalized gateway protocol and exact
function-call validator remain unchanged.

The published implementation adds no replacement provider, gateway request contract, HTTPS
transport, credentials, Keychain, runtime coordinator, dispatch, executor,
persistence, IPC, UI, dependency, capability, or permission. Eighteen gateway,
six function-validation, and two public gateway-to-policy tests pass. Clippy,
complete `npm run verify`, npm audit, exact-scope, secret, generated-output,
architecture, code-health, security, documentation, and mandatory gate reviews
pass. D-032 records the future closed provider-transport boundary.

Commit `5415444` is pushed on `codex/phase4-increment-4k`, fast-forward merged
into synchronized `main`, and retains a valid `04k` marker after the tracked
deletions were committed.

## Phase 4 Increment 4I remove generic audit scaffold - complete

Goal: delete the unused public caller-authored `AuditEventInput`, `AuditLogger`, in-memory/no-op logger, arbitrary summary/details records, and token-pattern redactor before a future coordinator can mistake them for the trusted local audit boundary.

The reconstructed source change deletes only `src-tauri/src/audit/logger.rs` and `src-tauri/src/audit/types.rs` and removes their two exports from `src-tauri/src/audit/mod.rs`. The verified typed `audit::approval` module remains unchanged. Reconstructed commit `99f9279` is pushed and fast-forward merged into synchronized `main`; the corrected marker remains valid for that committed content. The original `cf9d701` commit is preserved locally on `codex/phase4-increment-4i-pre-fingerprint-fix` and no remote ref contains it.

The increment adds no replacement audit abstraction, durable repository, storage migration, coordinator, dispatch, executor, provider continuation, IPC, UI, networking, credential, dependency, capability, or permission. Six typed-adapter tests, eleven native-source tests, one public approval-audit integration test, Clippy, `npm run verify`, npm audit, stale-symbol, scope, diff, code, security, documentation, and corrected post-increment reviews pass.

## Repository Workflow Increment 4J post-increment deletion fingerprint - complete

Goal: make one valid post-increment completion marker survive committing reviewed tracked-file deletions while preserving invalidation for files deleted after finalization.

The exact two-file implementation moves path hashing after successful metadata lookup and adds positive and negative deletion regressions. Exact changed-file report inventory, path safety, content and metadata hashing, report hashes, suspicious-path checks, Stop behavior, and state schema remain unchanged. Thirteen declared documentation files record the boundary and evidence. No application source, dependency, hook configuration, skill, Tauri, IPC, storage, provider, gateway, approval, audit, dispatch, executor, capability, or permission changes.

The original 4I commit remains preserved at `cf9d701` under `codex/phase4-increment-4i-pre-fingerprint-fix`. 4J is published and merged, so 4I reconstruction proceeds on the corrected baseline without a legacy-marker migration or compatibility fallback.

Seventeen focused hook tests, complete `npm run verify`, npm audit, exact scope, secret, generated-output, code, security, and complete-diff reviews pass. The consolidated result is `PASS WITH ADVISORIES`; the advisory is the required publication ordering before 4I reconstruction.

## Phase 4 Increment 4H typed approval-audit adapter - complete

Goal: derive one closed, content-free approval-audit record from an exact terminal `ApprovalResolution` and retain it in a bounded deterministic in-memory adapter without creating persistence, orchestration, dispatch, or execution authority.

The exact four-file runtime/test scope creates the dedicated adapter and public-boundary integration test, exports the module, and adds test-only assertions to the existing sealed native-source path. It revalidates exact current tool/policy facts and every terminal disposition/evidence combination, stores no task title or arbitrary string details, rejects invalid evidence, duplicates, capacity overflow, and sequence overflow before mutation, and returns only a non-authorizing sequence receipt.

Six adapter tests, eleven native-source tests, one public-boundary integration test, Clippy, `npm run verify`, npm audit, complete diff review, architecture review, code-health review, security review, documentation synchronization, and the mandatory post-increment gate pass. The generic audit scaffold remains unchanged and disconnected. No dependency, lockfile, Tauri, frontend, SQLite, gateway, provider, approval-manager behavior, native-dialog behavior, executor, IPC, capability, CSP, packaging, or permission changed. D-029 records the non-durable and non-authorizing boundary.

## Repository Workflow Increment 4G post-increment gate - complete

Goal: add a trusted repository-local Stop hook, deterministic Python validator, consolidated review skill/report, and documentation synchronization gate without changing application behavior.

The approved 24-file tracked scope adds no external dependency, network access, transcript parsing, product source, Tauri, Rust, IPC, persistence, capability, permission, approval, audit, or execution path. Fifteen focused hook tests and complete `npm run verify` pass. Exact scope, secret, generated-output, code, and security reviews pass after resolving parent-symlink escapes. The project owner passed normal hook trust and live Stop confirmation; the consolidated result is `PASS WITH ADVISORIES` and the marker is valid.

## Phase 4 Increment 4F product display rename - complete

Goal: rename only the human-facing product name to `Cortexa` while preserving repository, package, crate, executable, bundle-ID, database, storage, event, command, and other compatibility identifiers.

The reviewed implementation updates the Tauri/window/menu/native-dialog metadata, typed Settings app name, sidebar brand, fixed diagnostics, focused tests, repository skills and prompts, and all tracked exact former-name documentation. It adds no dependency, behavior, trust-boundary, persistence, capability, permission, network, credential, or execution change. The full requested automated gate, project-owner target-Mac confirmation, complete diff/code/security review, and synchronized project memory pass. The absent post-increment skill did not run and has no claimed result; D-027 records the project owner's one-time completion exception and defers skill creation to the next clean branch.

## Phase 2 Increment 2E — complete

Verified on 2026-07-13. The React application shell, closed menu-route handling, Settings diagnostics, and Permission Center placeholders passed all required automated and manual checks.

## Phase 2 Increment 2F — complete

Goal: add a deterministic, mocked assistant interaction flow to the verified application shell.

Planned boundaries:

- In-memory conversation messages only.
- Deterministic mock streaming and stop behavior.
- Tool activity card presentation.
- Trusted mock approval dialog.
- No network, API key, real tool execution, new Tauri command, OS permission, or persistence expansion.

Implementation and `npm run verify` pass on the target Mac. Native Tauri launch passes with idempotent storage startup. The project owner confirmed streaming, Stop, approve/reject/edit, small-window, lifecycle, and no-permission-prompt checks passed.

## Phase 2 Increment 2G — complete

Goal: complete bounded cancellation, error-state, audit-view, and release-verification hardening without production model access or privileged automation.

The typed driver, bounded failure and Retry, redacted in-memory Activity feed, and focused tests are implemented. `npm run verify`, `npm audit --audit-level=low`, native launch, and project-owner manual acceptance all pass.

## Phase 3 planning — complete

The product brief and architecture baseline were reconciled with the completed Phase 2 mock loop. The smallest missing capability was volatile conversation identity and history. Increment 3A was approved, implemented, and passed automated verification, native launch, and project-owner manual acceptance.

## Phase 3B planning — complete

The remaining product requirements were reconciled with the verified implementation through Increment 3A. Mock context provenance is the smallest next capability because the product requires visible information-use disclosure and conversation identity now provides the required ownership boundary.

The approved implementation limits the increment to fixed-copy, volatile WebView presentation tied to exact run and conversation IDs. It excludes real context collection, context controls, trusted provenance, persistence, tool results, networking, dependencies, native capability changes, and permissions. Automated verification, native launch, and project-owner native interaction and layout checks pass.

## Phase 3C planning — complete

The remaining tool-result and mocked-loop requirements were reconciled with the verified implementation through Increment 3B. Approve-only simulated tool-result presentation is the smallest next capability because the product requires a distinct result view and the current loop already has exact run, conversation, proposal, and decision boundaries.

The approved implementation limits the increment to fixed-copy volatile WebView presentation with `executed: false`. It excludes real execution, provider continuation, arbitrary payloads, trusted executor or audit claims, persistence, networking, dependencies, native capability changes, and permissions. Automated verification, native launch, and project-owner native interaction and layout checks pass.

## Phase 3 completion planning — complete

The verified implementation through Increment 3C ends at a simulated result, renders no distinct final answer after that result, and does not expose one closed conservative limit contract. Phase 3 therefore needs one final bounded Increment 3D.

The approved plan adds a synchronous fixed frontend mock continuation, exact adjacent result/final pairing, and explicit limits of two model turns, one tool call, one retry, zero network/tool timeout/file/search capacity, and 512 output characters per turn. Production provider continuation, real tools, arbitrary payloads, generic timeline work, persistence, native changes, and permissions remain excluded.

Implementation, automated verification, native development launch, and project-owner manual acceptance pass with 124 frontend tests, 50 Rust library tests, 6 Rust integration tests, production frontend and Tauri builds, zero dependency vulnerabilities, and idempotent startup with two migrations already applied.

## Phase 3D bounded mock-loop completion — complete

The project owner confirmed exact result/final ordering and run identity, request-content exclusion, no final answer after Reject/Edit/Stop, per-conversation restoration, supported layouts, existing context and Activity behavior, native routing, Settings diagnostics, lifecycle behavior, storage startup, and absence of permission prompts all pass. Increment 3D and Phase 3 are verified complete.

The next Ready task is documentation-only Phase 4 gateway and Responses security-boundary planning. No provider or runtime implementation may begin before the exact plan is approved.

## Phase 4 gateway and Responses planning - complete

The product, architecture, security policy, accepted decisions, official OpenAI documentation, and actual Rust/provider boundaries were reconciled. Decision D-021 assigns production OpenAI credentials to server-side gateway secret storage, keeps future gateway tokens in trusted Rust and platform secret storage, and requires a versioned normalized gateway protocol with dual validation, foreground `store: false` streaming, transport-abort cancellation, conservative limits, closed redacted errors, and separate gateway operational and local trusted audit records.

Increment 4A was approved and implemented as the smallest Phase 4 increment: one transport-free portable Rust gateway-protocol module, one exact already-locked parsing dependency, and deterministic inline tests. Focused checks, `npm run verify`, dependency audit, diff checks, code review, and security review pass. It adds no network client, gateway server, credential, IPC, tool execution, persistence, capability, CSP, packaging, or permission path.

## Phase 4 Increment 4A gateway protocol contract - complete

The versioned normalized event contract, transactional stream validator, conservative limits, local cancellation, closed redacted failures, and explicitly non-actionable function-call values are implemented. Seventeen focused tests cover accepted text/function streams and malformed, oversized, mismatched, out-of-order, duplicate, late, mixed, over-limit, duplicate-key, unknown-tool/contract, and error-redaction cases. The full gate passes with 124 frontend tests, 67 Rust library tests, six Rust integration tests, and production frontend/Tauri builds.

The next task was documentation-only Increment 4B planning for exact local per-tool schema validation. That plan was approved and implemented as recorded below.

## Phase 4 Increment 4B local tool-schema validation - complete

The placeholder schema was replaced with exact `get_current_datetime@1` and `create_local_task@1` contracts. Tool definitions now derive identity, version, risk, permission, description, and schema from the closed local catalog. An ownership-consuming validator independently checks normalized gateway calls and returns private typed, redacted, non-authorizing data with no raw JSON.

Focused tests, the full repository gate, dependency audit, diff checks, code review, and security review pass. No dependency, proposal, policy, approval, audit, executor, transport, IPC, persistence, permission, or user-visible path was added. The next task was documentation-only Increment 4C planning for trusted proposal and policy-input binding.

## Phase 4 Increment 4C trusted policy-input binding - complete

The verified 4A/4B boundaries were reconciled with the unused raw proposal/provider-response path, independently constructed policy actions, caller-supplied policy context, generic approval and audit scaffolds, accepted security rules, and actual repository callers. The smallest coherent increment removes the bypasses and lets policy consume one exact locally schema-validated call without caller-supplied state.

The approved plan defined canonical input as the ownership-bound typed `SchemaValidatedFunctionCall`, not serialized bytes or a digest. Because no current type binds intent, permission, scope, and freshness to the exact call, permission-bearing and read-only actions deny and reversible actions require approval. It defined closed policy reasons with derived outcomes and a decision that retains the exact evaluated input while granting no approval or execution authority. Trusted evidence, approval binding, hashes, previews, expiry, one-time consumption, audit, dispatch, provider continuation, networking, credentials, IPC, persistence, and UI remain later work.

The project owner approved the exact plan. Implementation removed the raw proposal/provider-response bypass and caller-supplied policy context, introduced ownership-bound policy input and input-retaining closed decisions, and made unsupported evidence paths fail closed. Four policy unit tests and two public gateway-to-policy integration tests prove the rule table, exact retained metadata and arguments, and debug redaction.

Focused checks, `npm run verify`, dependency audit, diff checks, code review, and security review pass. No dependency, approval, audit, executor, transport, IPC, persistence, permission, or user-visible path was added. Documentation-only Increment 4D planning is now complete as recorded below.

## Phase 4 Increment 4D exact approval binding - complete

The verified policy decision was reconciled with the detached approval and audit scaffolds. The smallest coherent increment first retains validator-owned run and gateway-request IDs through local schema validation and policy, then replaces arbitrary approval strings with an ownership-consuming transport-free manager.

The proposed manager accepts only one exact `RequireApproval` decision, derives a borrowed closed `create_local_task@1` preview from the retained typed arguments, permits one pending request and 1,024 subjects per manager lifetime, uses a relative manager-owned 120-second monotonic deadline, and consumes approve, reject, cancel, or expiry once. Duplicate run/request/call subjects fail closed without tombstone eviction, Edit requires a fresh validated call, and future orchestration must cancel approval when its run terminates.

The plan removes caller-supplied `action_hash` and adds no digest or dependency because direct in-process ownership is the stronger binding. Request, view, and resolution values remain non-cloneable, non-serializable, debug-redacted, and disconnected from audit, dispatch, execution, IPC, persistence, UI, and provider continuation.

Planning baseline checks passed on clean merged `main` at `55626b6`. The project owner approved the exact plan and five-file runtime/test list before implementation.

The implementation retains validator-owned run/request/call identity, removes raw content cloning and debug output, and replaces the detached approval scaffold with one ownership-consuming manager. The manager derives the exact borrowed `create_local_task@1` preview, enforces one pending request, a 1,024-subject lifetime cap, relative 120-second monotonic expiry, explicit cancellation, and non-evicting terminal replay prevention. It adds no digest, dependency, serialization, audit, dispatch, executor, IPC, persistence, network, credential, capability, or permission path.

Focused Rust checks, rustfmt, Clippy with warnings denied, `npm run verify`, dependency audit, diff checks, code review, and security review pass. The full gate contains 124 frontend tests, 82 Rust library tests, and ten Rust integration tests plus production frontend and Tauri no-bundle builds. No native interaction gate applies because the modules remain transport-free and unreferenced by Tauri. D-024 records the durable approval boundary. Documentation-only Increment 4E planning followed and is recorded below.

## Phase 4 Increment 4E trusted approval-decision source - complete

The verified manager, product preview requirements, native/WebView trust boundary, security policy, optional LocalAuthentication requirement, and actual Tauri capabilities were reconciled. The untrusted WebView mock cannot become production approval authority, and LocalAuthentication could authenticate a device owner but would not bind or display the exact action preview. The project owner approved the exact plan and eight-file runtime/test scope.

Implementation now issues one owned presentation from the authoritative manager and lets only a Rust-owned macOS native message-dialog source privately construct a sealed exact-subject outcome. A private `Arc` manager-instance marker moves through that handoff and is pointer-checked before approval/run/request/call identity, preventing cross-manager substitution without a digest or content retention. The manager rechecks one-shot issuance, cancellation, and monotonic expiry before terminal resolution. Edit, native no-decision, source failure, run cancellation, expiry, and replay fail closed. Only recognized Approve/Reject/Edit buttons carry native-button evidence; every source outcome records `NotEvaluated` authentication and grants no execution authority.

The implementation adds exact macOS-target `rfd = "=0.17.2"` with default features disabled. Direct use registers no Tauri dialog plugin, invoke command, JavaScript API, capability, or application `unsafe`; the source remains disconnected from the shipping app, WebView, LocalAuthentication, audit, persistence, dispatch, execution, provider continuation, gateway networking, and credentials. Sixteen approval unit tests, two approval-binding integration tests, rustfmt, Clippy, `npm run verify`, `npm audit --audit-level=low`, dependency review, code review, and security review pass.

The target-Mac owner interaction matrix passes: Approve, Reject, Edit, Return/default, fixed title/content order, terminal redaction, no action or persistence, and no permission prompt were confirmed; Escape had no effect and no window-close control was available. Exact `cargo-audit 0.22.2` exits nonzero on RUSTSEC-2026-0194 and RUSTSEC-2026-0195 in pre-existing `quick-xml 0.39.4`; 4E adds only `rfd`, and source review found the cited vulnerable APIs unused on the existing `plist -> Tauri` path. D-025 records the project owner's scoped reviewed baseline exception without an advisory ignore or dependency change. Increment 4E is verified complete, and no later increment is Ready.
