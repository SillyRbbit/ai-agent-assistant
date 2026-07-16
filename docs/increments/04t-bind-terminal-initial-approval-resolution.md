# Phase 4 Increment 4T - Bind terminal initial approval resolution

Last updated: 2026-07-15

Status: **Verified complete with uncommitted changes**

## Goal

Make the bound initial gateway turn consume one sealed trusted approval source
outcome through the same private manager that issued its presentation and return
one exact non-authorizing terminal resolution.

## Implementation result

- Work began from clean synchronized `main` at `6d0bed4`; Increment 4S is
  committed, pushed, fast-forward merged, and its `04s` completion marker was
  complete and valid before these planning edits.
- The mandatory `04t` gate began before source edits.
- `InitialGatewayTurn::resolve_approval_source_outcome` now consumes one sealed
  outcome and delegates directly to its existing private manager on macOS.
- Every closed native result maps to the exact existing disposition and
  interaction evidence; a foreign-manager outcome fails before mutating the
  recipient pending subject, which can still consume its own outcome.
- The existing synthetic dialog-result mapper is crate-visible only under
  `cfg(test)`; production native-source behavior is unchanged.
- Eight focused request, 17 approval, nine gateway-contract, two
  approval-binding, and one approval-audit test pass. Clippy, complete
  `npm run verify`, and npm audit also pass.
- Before 4T, the turn owned exact validation, policy, request creation, and
  presentation issuance, but its private manager could not consume the sealed
  source outcome produced from that presentation.
- The native source and manager already implement exact sealed-outcome
  construction, pointer-identical manager binding, identity checks, expiry,
  closed result/evidence mapping, replay prevention, and redaction.
- The smallest enforceable closure added one macOS-gated delegation method to
  the turn and widened only an existing synthetic native-result mapper under
  `cfg(test)` so unit tests avoid native UI.
- No native invocation, production caller, run cancellation, proactive expiry,
  audit, persistence, dispatch, execution, transport, IPC, or user-visible
  behavior is included.

## Exact source and test scope

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/src/approvals/decision_source.rs
```

No approval-manager/type, native-source production behavior, public integration
test, audit, policy, lower-level gateway, schema, registry, tool, module-export,
manifest, lockfile, dependency, Tauri, frontend, storage, capability,
entitlement, or permission path changes.

## Exact closeout scope

Create:

```text
docs/increments/04t-bind-terminal-initial-approval-resolution.md
docs/plans/04t-bind-terminal-initial-approval-resolution.md
```

Change:

```text
AGENTS.md
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/plans/README.md
```

## Risks

- The request module gains a macOS-gated dependency on the sealed native source
  outcome and resolution, deepening trusted assembly coupling.
- `Approved` resolution data could be mistaken for dispatch authority even
  though it remains non-authorizing and unaudited.
- The test-only mapper gains crate-wide test-build visibility and must remain
  absent from production builds and public APIs.
- Run-termination cancellation, proactive expiry, stale-dialog handling, audit,
  and active-run validation remain unresolved production blockers.

## Non-goals

Native dialog invocation or changes, source traits, coordinator/runtime work,
run cancellation, proactive expiry, audit writes, persistence, run-liveness,
dispatch, executor, tool results, continuation, retries, transport, gateway
deployment, authentication, credentials, Keychain, provider parameters, Tauri,
WebView, SQLite, dependencies, capabilities, entitlements, and permissions are
excluded.

## Completion gates

- [x] Required repository memory, workflow, product, architecture, security,
      decisions, troubleshooting, 4S, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04s marker, toolchains, caller scan,
      and focused checks recorded.
- [x] Exact two-file source/test scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Mandatory 04t gate state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Documentation and D-041 match actual evidence.
- [x] The post-increment report passes and the 04t marker is complete and valid.

## Rollback

Before commit, restore the two source/test files to `6d0bed4` and revert only the
declared 4T documentation. After commit, revert the single 4T commit. No data,
migration, dependency, credential, compatibility identifier, or remote resource
requires rollback.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge Increment
4T. Do not start a later increment; none is Ready.
