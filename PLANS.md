# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

None. Increment 4D is complete. Documentation-only Increment 4E planning is next; no execution plan exists yet.

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

| Plan                                         | Status   | Owner              | Last updated |
| -------------------------------------------- | -------- | ------------------ | ------------ |
| Increment 2B-1 SQLite migration skeleton     | Complete | Project maintainer | 2026-07-13   |
| Increment 2C storage startup integration     | Complete | Project maintainer | 2026-07-13   |
| Increment 2D menu-bar/window lifecycle       | Complete | Project maintainer | 2026-07-13   |
| Increment 2E React application shell         | Complete | Project maintainer | 2026-07-13   |
| Increment 2F mocked interaction shell        | Complete | Project maintainer | 2026-07-13   |
| Increment 2G integration hardening           | Complete | Project maintainer | 2026-07-13   |
| Increment 3A in-memory conversation sessions | Complete | Project maintainer | 2026-07-13   |
| Increment 3B mock context provenance         | Complete | Project maintainer | 2026-07-13   |
| Increment 3C simulated tool result           | Complete | Project maintainer | 2026-07-13   |
| Increment 3D bounded mock-loop completion    | Complete | Project maintainer | 2026-07-14   |
| Increment 4A gateway protocol contract       | Complete | Project maintainer | 2026-07-14   |
| Increment 4B local tool-schema validation    | Complete | Project maintainer | 2026-07-14   |
| Increment 4C trusted policy-input binding    | Complete | Project maintainer | 2026-07-14   |
| Increment 4D exact approval binding          | Complete | Project maintainer | 2026-07-14   |

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

Focused Rust checks, rustfmt, Clippy with warnings denied, `npm run verify`, dependency audit, diff checks, code review, and security review pass. The full gate contains 124 frontend tests, 82 Rust library tests, and ten Rust integration tests plus production frontend and Tauri no-bundle builds. No native interaction gate applies because the modules remain transport-free and unreferenced by Tauri. D-024 records the durable approval boundary. Documentation-only Increment 4E planning for a trusted approval-decision source is next.
