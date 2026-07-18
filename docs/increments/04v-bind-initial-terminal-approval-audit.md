# Increment 4V - bind initial terminal approval audit

Status: Verified complete; published through PR #23 at `6e6f91d`
Last updated: 2026-07-18

## Goal

Prevent a future initial-turn caller from receiving a successful native or
run-termination approval resolution unless that exact manager-owned resolution
has first been validated and recorded by the turn's private typed in-memory
approval-audit adapter.

## Implementation result

Increment 4T returned one exact sealed native resolution through the issuing
manager. Merged Increment 4U added the manager's existing run-termination
resolution path. Increment 4V now binds both paths to one private turn-owned
`InMemoryApprovalAuditAdapter`.

Both successful paths route the exact manager-owned resolution through one
private helper and return only a closed non-cloneable resolution plus
non-authorizing receipt. Manager success clears pending turn ownership before
recording; a typed audit failure therefore returns no result and cannot leave a
stale pending subject or roll back manager state.

## Exact source/test scope

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

No other source, test, dependency, manifest, lockfile, Tauri, SQLite,
capability, entitlement, permission, or configuration path changes.

## Risks

- The approval manager terminalizes before the in-memory audit adapter records;
  an unexpected typed audit failure cannot roll the manager state back.
- Pending approval ownership must be cleared after manager terminalization even
  if audit recording fails, because the manager subject has already been
  consumed.
- A caller could misread the receipt or an `Approved` disposition as dispatch or
  execution authority.
- The adapter is volatile and per-turn; this increment does not satisfy durable
  audit, crash recovery, or cross-run sequencing requirements.

## Non-goals

- SQLite or other durable audit persistence, transactions, recovery, export, or
  retention policy.
- Native dialog invocation, closure, stale-window control, proactive expiry,
  timers, runtime coordination, or active-run validation.
- Dispatch, execution, tool results, provider continuation, transport,
  authentication, credentials, Keychain, Tauri, frontend, dependencies,
  capabilities, entitlements, or permissions.
- Any lower-level approval-manager, native-source, audit-adapter, policy,
  schema, or gateway-protocol behavior change.

## Verification

Focused gateway-request, approval, approval-binding, and approval-audit tests
must pass, followed by Rust formatting, strict Clippy, complete
`npm run verify`, npm audit, diff review, and the mandatory `04v` gate. No
manual verification is planned because no native UI or production caller is
introduced.

## Rollback

Before commit, restore the two source/test files to the verified merged 4U
commit and revert only the declared 4V planning and closeout documents. After
commit, revert one bounded 4V commit. No migration, data, dependency,
credential, compatibility identifier, or remote resource requires rollback.

## Current evidence

- Increment 4T is merged at `244a1d8` and its clean-commit fingerprint matches
  the stored valid `04t` marker.
- The typed in-memory approval-audit adapter already accepts native outcomes,
  run termination, and expiry while rejecting inconsistent evidence.
- Increment 4U is published and merged at `61525bf`; its technical prerequisite
  is satisfied and this plan is reconciled against that exact API.
- Meta Increment 7 is squash-merged through PR #19 at `96ba6ae`, so its former
  publication prerequisite no longer blocks this product increment.
- ARB-022 is squash-merged through PR #22 at `7c79e65`. The original reviewed
  4V commit is preserved at `3440ce9`.
- The active branch was recreated from clean synchronized `main` at `d81b73a`,
  and a fresh mandatory `04v` gate began before the original change was applied
  without commit.
- The implementation changes exactly the two approved source/test paths.
- Ten gateway-request unit tests, ten public gateway-request contract tests, six
  audit tests, 17 approval tests, two approval-binding tests, one
  approval-audit-binding test, Rust formatting, and strict Clippy pass.
- Complete `npm run verify` passes 28 hook tests, 19 repository-health tests,
  124 frontend tests, 96 Rust library tests, 21 Rust integration tests, lint,
  typecheck, frontend builds, and the Tauri release no-bundle build.
- Documentation, security, conflict, whitespace, exact-scope, session-end,
  architecture, code-health, technical-debt, readiness, and complete-diff
  reviews pass. The sandboxed npm audit failed on DNS; the approved network
  retry found zero vulnerabilities.
- No manual gate applies. The consolidated result is `PASS WITH ADVISORIES`,
  and the `04v` completion marker is complete and valid.
- Reconstructed source commit `ec919e9` passed hosted CI, Documentation, and
  Security before PR #23 was squash-merged at `6e6f91d`.

## Exact next task

Review and publish only the documentation-only 4V publication reconciliation
after separate project-owner approval. Preserve dated evidence and do not begin
ARB-002 or any other remediation automatically.
