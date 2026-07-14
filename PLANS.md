# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

None. Increment 4A is complete. The next Ready task is documentation-only Increment 4B planning; no Increment 4B execution plan exists yet.

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

The next Ready task is documentation-only Increment 4B planning for exact local per-tool schema validation. It must not add transport, credentials, IPC, policy authorization, approval, or execution.
