# Handoff

Last updated: 2026-07-14

## Current state

Phase 3 and Phase 4 Increments 4A through 4D are verified complete on the target Mac. Increment 4D was implemented on branch `phase4/increment-4d`, based on merged `main` at `55626b6`.

The project owner requested commit, push, and fast-forward merge after verification. Implementation commit `1bf1f10` and this publication-state closeout were pushed on `phase4/increment-4d` and fast-forward merged into `main`. The final `main` working tree is clean.

No runtime approval source, UI, IPC, audit, dispatch, executor, persistence, provider, or network integration was added. Documentation-only Increment 4E planning for a trusted approval-decision source is the next Ready repository task.

## Completed work

- Retained validator-owned run and gateway-request IDs on accepted gateway function calls and carried them through local schema validation, policy, approval request, borrowed preview, and terminal resolution.
- Removed `Clone` and raw derived debug output from content-bearing gateway calls and events; custom debug output omits raw arguments and output-text deltas.
- Replaced detached approval tool names, `action_hash`, preview strings, caller-owned times, and clonable records with one `ApprovalManager` that consumes an exact owned `RequireApproval` policy decision.
- Added one closed borrowed `create_local_task@1` preview derived from the retained typed arguments and local metadata. Information-only and unsupported subjects fail closed.
- Enforced one pending request, a 1,024-subject manager-lifetime cap, a manager-owned relative 120-second monotonic deadline, explicit cancellation, and terminal approve, reject, cancel, and expiry behavior.
- Retained non-evicting run/request/call tombstones so every terminal subject rejects recreation and replay for that manager lifetime.
- Added typed redacted failures for ineligible outcomes, unsupported subjects, pending, replay, capacity, lookup, consumed IDs, ID overflow, and deadline overflow.
- Kept request views, previews, and resolutions non-cloneable and non-serializable, with no consuming conversion to policy input, audit, dispatch, IPC, or execution.
- Added public gateway-to-approval integration coverage and focused lifecycle, exact-expiry-boundary, replay, capacity, overflow, and redaction tests.
- Recorded D-024: direct same-process ownership is the approval binding; no digest or dependency is needed, and `Approved` is not proof of a user gesture, user presence, local authentication, run liveness, or execution eligibility.
- Reviewed the complete implementation for correctness, security, privacy, scope, generated output, secrets, and documentation consistency. The private wire-event debug leak found during review was removed, and replay coverage now includes every terminal disposition.

## Exact files changed

Created:

```text
docs/increments/04d-exact-approval-binding.md
docs/plans/04d-exact-approval-binding.md
src-tauri/tests/approval_binding.rs
```

Changed:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
src-tauri/src/agent/function_call_validation.rs
src-tauri/src/agent/gateway_protocol.rs
src-tauri/src/approvals/manager.rs
src-tauri/src/approvals/types.rs
```

No file outside the approved five-file implementation list and nine-file closeout list changed. Manifests, lockfiles, policy rules, schemas, registry, audit, executor, provider, storage, Tauri, frontend, capabilities, CSP, packaging, permissions, generated files, databases, and build output are absent from the diff.

## Verification classification

Passed before implementation:

```text
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol
  17 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
  6 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::
  4 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
  3 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::
  4 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed; 0 failed
```

Passed after implementation:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol
  18 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
  6 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::
  4 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
  6 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed; 0 failed
npm run verify
  frontend: 10 files, 124 tests passed
  Rust library: 82 tests passed
  Rust integration: 10 tests passed
  TypeScript, Vite production builds, and Tauri release build --no-bundle passed
npm audit --audit-level=low
  0 vulnerabilities
npm run format:check
git diff --check
scope, stale-symbol, secret-pattern, and generated-output checks
code review
security review
documentation synchronization review
  passed with no findings
```

Failed checks:

- No final implementation or repository check remains failed.
- The first planning-stage `npm run format:check` found formatting differences in the new plan. `npx prettier --write docs/plans/04d-exact-approval-binding.md` corrected them, and subsequent format checks passed.
- The first sandboxed `npm audit --audit-level=low` attempt failed with `ENOTFOUND registry.npmjs.org`. The required network-enabled retry completed and reported zero vulnerabilities.

Checks not run:

- Native Tauri launch and interaction checks were not run because the changed modules are transport-free and remain unreferenced by Tauri and the UI. The full Tauri release no-bundle build passed.

Manual verification still pending:

- None for Increment 4D. No user-visible or target-platform interaction behavior changed.

## Remaining boundaries and risks

- `ApprovalManager` is not wired to a production orchestrator. One-time, capacity, and replay guarantees apply to one manager instance; future orchestration must own one authoritative instance.
- The relative approval TTL does not prove the run remains active. Future orchestration must cancel pending approval when the run ends, is cancelled, or reaches its absolute deadline.
- `Approved` records only a closed Rust choice processed while one exact subject was pending and unexpired. No trusted source currently proves who chose it or whether user presence or local authentication occurred.
- No WebView or native approval input exists. A future plan must prevent an untrusted WebView message from becoming authority and must define any native or LocalAuthentication claim precisely.
- Approval resolution cannot be audited, dispatched, executed, persisted, or returned to a provider. Those boundaries require separate approved increments and pre-execution revalidation.
- The generic audit scaffold remains unsuitable for raw approval content or exact personal data and remains disconnected.
- Permission, resource scope, explicit intent, provenance, freshness, and authentication evidence remain undefined; denied policy classes remain denied.
- O-006 still blocks live gateway networking until gateway identity and deployment are selected. O-007 still blocks live provider traffic until retention mode and user disclosure are approved.

## Exact next task

On clean merged `main`, perform documentation-only Increment 4E planning for the trusted approval-decision source; recommend one smallest independently verified runtime increment and wait for approval. Do not start runtime implementation or a later gateway increment during planning.

## Ready-to-paste resume prompt

```text
Use $session-start.

Start documentation-only Phase 4 Increment 4E planning from HANDOFF.md on clean merged main after Increment 4D. Reconcile the transport-free ApprovalManager with the product approval experience, architecture, SECURITY.md, CODE_REVIEW.md, accepted decisions, Tauri/WebView trust boundary, macOS-native interaction options, optional LocalAuthentication, and actual repository capabilities. Define which process and UI surface owns a trusted approval choice; exact approval-ID and run/request/call binding; user-presence and authentication claims; cancellation, expiry, edit invalidation, and replay behavior; closed limits and redacted errors; and the boundary to future structured audit without creating dispatch or execution authority. Recommend one smallest independently verified runtime increment with exact files, risks, non-goals, automated verification, any target-Mac manual gate, and rollback. Update planning documentation only, then wait for project-owner approval. Do not implement, commit, or push unless explicitly asked.
```
