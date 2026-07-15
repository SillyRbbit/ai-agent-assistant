# Phase 4 Increment 4S - Bind terminal initial approval presentation

Last updated: 2026-07-15

Status: **Verified complete with uncommitted changes**

## Goal

Make the bound initial gateway turn consume a terminal
`RequireApproval` policy decision through the existing exact approval manager
and return one owned, non-authorizing `ApprovalPresentation` instead of exposing
the decision for a caller-selected approval transition.

## Implementation result

- Planning began from clean synchronized `main` at `5e58edb`; Increment 4R is
  committed, pushed, fast-forward merged, and its `04r` completion marker was
  complete and valid before these planning edits.
- TypeScript typecheck and 72 focused request, protocol, function-validation,
  policy, tool, approval, gateway-contract, approval-binding, and approval-audit
  tests pass.
- `InitialGatewayTurn` now owns terminal schema validation, deterministic
  policy, and one private exact approval manager. Terminal `RequireApproval`
  cannot leave as a standalone policy decision.
- The bound turn consumes that exact decision, creates one manager-owned
  request, and issues one owned presentation before returning to its caller.
- The implementation changes only the bound turn and its public contract:
  terminal `RequireApproval` creates and issues the existing presentation
  atomically, while `Allow` and `Deny` remain non-authorizing policy events.
- The mandatory `04s` gate began before source edits. Focused tests, strict
  Clippy, complete `npm run verify`, npm audit, exact-scope, code, security,
  documentation, and post-increment reviews pass. No manual check is required.
- D-040 records exact terminal approval-manager ownership, presentation
  issuance, non-authority, typed failure, and public event API narrowing.

## Exact source and test scope

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

No approval-manager, approval-type, native-source, audit, policy, lower-level
gateway, schema, registry, tool, module-export, manifest, lockfile, dependency,
Tauri, frontend, storage, capability, entitlement, or permission path changes.

## Exact changed-file scope

Created:

```text
docs/increments/04s-bind-terminal-initial-approval-presentation.md
docs/plans/04s-bind-terminal-initial-approval-presentation.md
docs/reviews/2026-07-15-04s-post-increment-review.md
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
docs/plans/README.md
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

## Risks

- The agent request module owns an approval manager, deepening the existing
  trusted assembly coupling. Any need for a new coordinator or approval-source
  resolution expands scope and requires approval.
- `InitialGatewayEvent` no longer derives equality because
  `ApprovalPresentation` is intentionally owned and non-comparable. The crate is
  unpublished, but a theoretical unsupported external consumer would need to
  migrate.
- A returned presentation has a private pending manager behind it, but 4S
  adds no path to feed a trusted source outcome back into that manager. The path
  is deliberately incomplete and non-executable.
- The approval TTL begins when terminal completion creates the request. Native
  presentation, expiry, run cancellation, and resolution orchestration remain
  future work.
- `Allow` could still be mistaken for dispatch authority even though it remains
  only a retained policy decision.

## Non-goals

Trusted native-dialog invocation, source-outcome resolution, approval
disposition, LocalAuthentication, audit writes, durable approval persistence,
run-liveness, dispatch, executor, tool results, continuation, retries, transport,
gateway deployment, authentication, credentials, Keychain, provider parameters,
runtime coordination, Tauri, WebView, SQLite, dependencies, capabilities,
entitlements, and permissions are excluded.

## Completion gates

- [x] Required repository memory, workflow, product, architecture, security,
      decisions, troubleshooting, 4R, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04r marker, toolchains, caller scan,
      and focused checks recorded.
- [x] Exact two-file source/test scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Mandatory 04s gate state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Documentation and D-040 match actual evidence.
- [x] The post-increment report passes and the 04s marker is complete and valid.

## Rollback

Before commit, restore the two source/test files to `5e58edb` and revert only the
declared 4S documentation. After commit, revert the single 4S commit. No data,
migration, dependency, credential, compatibility identifier, or remote resource
requires rollback.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge Increment
4S. Do not start a later increment; none is Ready.
