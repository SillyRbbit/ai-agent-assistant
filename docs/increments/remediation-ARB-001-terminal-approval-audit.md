# Remediation ARB-001 - terminal approval audit binding

Date: 2026-07-16
Reconstructed: 2026-07-18
Status: Resolved and published through PR #23 at `6e6f91d`
Branch: `main`
Baseline: `d81b73a1633a35e3432a19434495d7b6b9db2431`
Gate ID: `04v`
Resolving source commit: `ec919e9a17c6b877b21778e8b0323d6a1829d94e`
Squash merge commit: `6e6f91d39ae6b09df3c37972ba600fc69220339d`

## Goal

Resolve ARB-001 by preventing a future initial-turn caller from receiving a
successful native or run-termination approval resolution unless the exact
manager-owned resolution has first been validated and recorded by the turn's
private typed in-memory approval-audit adapter.

## Root cause

Verified Increments 4T and 4U returned terminal `ApprovalResolution` values
directly from `InitialGatewayTurn`. The typed `InMemoryApprovalAuditAdapter`
already existed, but trusted assembly did not own or invoke it. A future caller
could therefore receive manager success without an audit record and receipt.

## Exact scope

Modified source and test:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

Modified closeout documentation:

```text
AGENTS.md
ARCHITECTURE.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PRODUCT_REQUIREMENTS.md
PROJECT_STATUS.md
ROADMAP.md
SECURITY.md
docs/increments/04v-bind-initial-terminal-approval-audit.md
docs/plans/04v-bind-initial-terminal-approval-audit.md
docs/plans/README.md
docs/reviews/2026-07-16-advisory-remediation-backlog.md
```

Created closeout documentation:

```text
docs/increments/remediation-ARB-001-terminal-approval-audit.md
docs/reviews/2026-07-16-04v-post-increment-review.md
```

## Implemented remediation

- Added one private `InMemoryApprovalAuditAdapter` to `InitialGatewayTurn`.
- Added a closed, non-cloneable `AuditedApprovalResolution` with read-only
  access to the exact manager-owned resolution and its sequence-only receipt.
- Routed native-source and run-termination success through one private
  manager-then-audit helper.
- Cleared pending turn ownership after manager terminalization, including when
  audit recording returns a typed error.
- Added `InitialGatewayTurnError::ApprovalAudit` so audit failure remains typed
  and redacted and returns no successful resolution.
- Added focused unit and public-contract regression coverage for exact facts,
  receipt binding, all native outcomes, run termination, idempotence, late
  outcomes, redaction, and the post-manager audit-failure boundary.

## Non-goals and protected boundaries

- No durable audit, SQLite, transaction, retention, export, recovery, or
  cross-run sequence.
- No native dialog invocation or closure, proactive expiry, timer, runtime
  coordination, active-run validation, dispatch, execution, tool result,
  provider continuation, transport, authentication, credential, or Keychain
  work.
- No Tauri IPC, frontend, dependency, manifest, lockfile, capability,
  entitlement, permission, CSP, schema, migration, or lower-level approval,
  audit, policy, tool, or gateway-protocol change.
- No commit, push, merge, or later remediation.

## Security and privacy

The audit receipt remains volatile, sequence-only, redacted, and
non-authorizing. The helper records the exact manager-owned resolution by
reference; it does not reconstruct caller-selected facts. An `Approved`
disposition and receipt still grant no dispatch, execution, continuation, or
durability authority. No new data leaves the process, and no secret, personal
content, credential, permission, network, filesystem, or log boundary changes.

## Verification

Passed:

- Rust formatting, ten gateway-request unit tests, six approval-audit tests, 17
  approval tests, ten public gateway-request contract tests, two
  approval-binding tests, one approval-audit-binding test, and strict Clippy.
- Complete `npm run verify`: 28 hook tests, 19 repository-health tests, 124
  frontend tests, 96 Rust library tests, 21 Rust integration tests, lint,
  typecheck, two frontend builds, and the Tauri release no-bundle build.
- Documentation, security, npm audit, conflict, whitespace, exact-scope,
  session-end, architecture, security, code-health, technical-debt, readiness,
  and complete-diff review. The sandboxed npm audit failed on DNS; the approved
  network retry found zero vulnerabilities.
- Mandatory `04v` post-increment gate with result `PASS WITH ADVISORIES` and a
  complete, valid marker.

Failed required checks: none.

Checks not run: native application launch and UI verification, because no
production caller, native invocation, frontend, Tauri configuration, asset,
dependency, or user-visible behavior changes.

Required manual verification: none. This remediation adds no native UI,
production caller, runtime transport, persistence, dispatch, execution, or
user-visible behavior.

## Risks and rollback

Manager terminalization precedes audit recording and cannot be rolled back. A
typed audit failure must therefore return no resolution and leave no stale turn
ownership. The adapter remains volatile and the request module gains assembly
coupling that should move to a future coordinator rather than be duplicated.

Before commit, restore the exact 19 paths to reconstruction baseline `d81b73a`.
After publication, revert only the bounded Increment 4V commit. Original commit
`3440ce9` remains preserved on the pre-refresh branch. No migration, data,
dependency, credential, identifier, capability, permission, or remote-resource
rollback applies.

## Next task

Review and publish only the documentation-only 4V publication reconciliation
after separate project-owner approval. Do not begin ARB-002 or any other
remediation automatically.
