# Increment 4V - bind initial terminal approval audit

Status: Proposed; merged 4U prerequisite satisfied, separate approval required
Last updated: 2026-07-15

## Goal

Prevent a future initial-turn caller from receiving a successful native or
run-termination approval resolution unless that exact manager-owned resolution
has first been validated and recorded by the turn's private typed in-memory
approval-audit adapter.

## Why this is the next smallest follow-on

Increment 4T returns one exact sealed native resolution through the issuing
manager. Merged Increment 4U adds the manager's existing run-termination
resolution path. Both remain explicitly unaudited even though the verified
`InMemoryApprovalAuditAdapter` already accepts every supported terminal
disposition.

After 4U, one bounded turn-local change can route both successful resolution
paths through one private audit helper and return only a closed resolution plus
non-authorizing receipt. It requires no persistence, coordinator, transport,
native invocation, dispatch, or execution.

## Exact future source/test scope

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The implementation will add one private `InMemoryApprovalAuditAdapter` to the
turn and one closed non-cloneable audited-resolution value. Successful native
and run-termination resolutions will be recorded before that value can leave
the turn. The receipt remains sequence evidence only and grants no authority.

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
- No `04v` gate state or implementation change exists.

## Exact next task

Do not start 4V. It remains outside the Ready queue until the project owner
explicitly selects this exact scope and grants separate implementation approval.
