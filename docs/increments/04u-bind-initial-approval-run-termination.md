# Increment 4U - bind initial approval run-termination

Status: Verified complete; published and merged at `61525bf`
Last updated: 2026-07-15

## Goal

Let the bound initial turn terminally deny and consume its exact pending approval
when a trusted future orchestrator reports run termination, without accepting a
caller-selected approval ID, choice, native result, or interaction evidence.

## Why this is the next smallest increment

Increment 4T closes same-manager trusted native resolution, but the turn does not
retain the manager-assigned approval ID after presentation. It therefore cannot
use the approval manager's already verified run-termination cancellation path.

This increment is smaller and more independently testable than native dialog
invocation, proactive expiry, audit binding, or dispatch. Native invocation would
require real UI or a new abstraction, proactive expiry would require clock or
timer work, audit binding needs a terminal coordinator decision, and dispatch
remains prohibited before those boundaries close.

## Exact future source/test scope

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The implementation will retain one private manager-assigned approval ID and add
one idempotent no-argument cancellation operation. It will delegate only to the
existing private manager's `cancel_for_run_termination`, return the existing
non-authorizing resolution when consumed, and preserve expiry precedence,
one-time use, replay protection, and late-outcome rejection.

## Declared planning and closeout scope

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04u-bind-initial-approval-run-termination.md
docs/plans/04u-bind-initial-approval-run-termination.md
docs/plans/README.md
docs/reviews/2026-07-15-04u-post-increment-review.md
```

## Risks

- The turn can drift from manager lifecycle state if it clears the retained ID
  before successful terminal resolution.
- Expiry must win at or after the deadline.
- Late native results must remain rejected after cancellation even though a
  stale native prompt may remain visible.
- The returned cancellation resolution remains non-authorizing and unaudited.
- Active-run orchestration and durable audit binding remain open blockers.

## Non-goals

- Native dialog invocation, changes, closure, or stale-window control.
- Proactive expiry, clocks, timers, retries, or background work.
- Source traits, runtime coordination, or caller-provided run-liveness evidence.
- Audit, persistence, dispatch, execution, continuation, transport,
  authentication, credentials, Keychain, Tauri, frontend, SQLite, dependencies,
  capabilities, entitlements, or permissions.
- Any lower-level approval-manager, native-source, policy, schema, or gateway
  protocol change.

## Verification

Focused request, approval, public contract, approval-binding, and approval-audit
tests must pass, followed by Rust formatting, strict Clippy, complete
`npm run verify`, npm audit, diff review, and the mandatory post-increment gate.
No manual verification is planned because no native UI or production caller is
introduced.

## Rollback

Before commit, restore the two source/test files to `244a1d8` and revert only the
declared 4U planning and closeout documents. After commit, revert one bounded 4U
commit. No migration, data, dependency, credential, compatibility identifier,
or remote resource requires rollback.

## Completion evidence

- Clean synchronized `main` resolved to
  `244a1d88bd299c0b3439d89b6984f21d0e201b09` before planning edits.
- A clean archive of `244a1d8` reproduces the stored valid `04t` workspace
  fingerprint exactly. The live marker is stale only because planning files are
  now present.
- The mandatory `04u` gate began before either source/test file changed.
- The exact source/test delta is limited to
  `src-tauri/src/agent/gateway_request.rs` and
  `src-tauri/tests/gateway_request_contract.rs`.
- Focused request (9), approval (17), public contract (10), approval-binding
  (2), and approval-audit-binding (1) tests pass.
- Strict Clippy and complete `npm run verify` pass with 17 hook, 124 frontend,
  95 Rust library, and 21 Rust integration tests plus lint, typecheck, frontend
  builds, and the Tauri release no-bundle build.
- The required network-enabled npm audit reports zero vulnerabilities.
- Complete diff, exact-scope, code, security, formatting, whitespace, secret,
  generated-output, and documentation reviews have no blocking finding.
- D-042 records private ID ownership, run-termination non-authority,
  idempotence, expiry precedence, typed-error retention, and late-outcome
  rejection.
- The consolidated result is `PASS WITH ADVISORIES`; the `04u` completion
  marker is complete and valid. No manual verification applies.

## Exact next task

No 4U work remains. Commit `61525bf208553cb17f3dfe665d3f7b7d32d306c1`
with message `Bind initial approval run termination` is pushed on
`codex/phase4-increment-4u`, fast-forward merged into `main`, and synchronized
with `origin/main`. The `04u` marker remained complete and valid after commit,
merge, and push.

Increment 4V terminal approval audit binding is only a proposed follow-on. Do
not start it automatically; the merged 4U prerequisite is satisfied, but 4V
still requires explicit queue selection and separate project-owner approval.
