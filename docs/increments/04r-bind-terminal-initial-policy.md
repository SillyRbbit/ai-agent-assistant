# Phase 4 Increment 4R - Bind terminal initial function call to policy

Last updated: 2026-07-15

Status: **Verified complete; uncommitted**

## Goal

Make the bound initial gateway turn consume its terminal schema-validated
function call through the fixed deterministic policy engine and return only the
owned `PolicyDecision`.

## Completion result

- Planning began from clean synchronized `main` at `8598612`; Increment 4Q is
  committed, pushed, fast-forward merged, and its completion marker was valid
  before these planning edits.
- Focused baseline verification passes with TypeScript typecheck, six request,
  18 protocol, six function-validation, four policy, nine tool, nine public
  request-contract, two policy-input, and two approval-binding tests.
- The bound turn now retains the exact schema-valid call until accepted terminal
  completion consumes it through the fixed deterministic engine.
- The closed `PolicyEvaluated` event owns one non-authorizing `PolicyDecision`;
  no standalone owned call leaves the bound turn.
- Both exact tool outcomes, retained typed facts, terminal ordering,
  protocol-error transactionality, failure/cancellation discard, and redaction
  pass focused contract coverage.
- Clippy, complete `npm run verify`, npm audit, complete-diff, scope, code,
  security, documentation, and mandatory gate reviews pass. No manual gate is
  required.

## Exact source and test scope

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

No policy source, lower-level gateway protocol, schema validation, tools,
approval, audit, transport, runtime, module export, manifest, lockfile,
dependency, Tauri, frontend, storage, capability, entitlement, or permission
path changes.

## Exact closeout file scope

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04r-bind-terminal-initial-policy.md
docs/plans/04r-bind-terminal-initial-policy.md
docs/plans/README.md
docs/reviews/2026-07-15-04r-post-increment-review.md
```

## Risks

- The bounded agent-to-policy dependency is bidirectional at the module level,
  although the ownership graph is non-recursive. Any need for a coordinator
  module expands scope and requires approval.
- The public initial event variant narrows for a theoretical unsupported external
  consumer.
- `Allow` could be misread as execution authority despite the existing type and
  decision contract.
- Policy must not run before terminal completion or after failure/cancellation.
- Lower-level validators and policy constructors remain public for independent
  tests; future initial transport must use the bound turn.

## Non-goals

Policy-rule changes, approval, native interaction, audit, dispatch, execution,
tool results, continuation, retries, transport, gateway deployment,
authentication, credentials, Keychain, provider parameters, runtime
coordination, Tauri, WebView, SQLite, dependencies, capabilities, entitlements,
and permissions are excluded.

## Completion gates

- [x] Required repository memory, workflow, product, architecture, security,
      decisions, troubleshooting, 4Q, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04q marker, toolchains, caller scan,
      and focused checks recorded.
- [x] Exact two-file source/test scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Mandatory 04r gate state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Documentation and durable decision match actual evidence.
- [x] The post-increment report passes and the 04r marker is complete and valid.

## Rollback

Before commit, restore the two source/test files to `8598612` and revert only the
declared 4R documentation. After commit, revert the single 4R commit. No data,
migration, dependency, credential, compatibility identifier, or remote resource
requires rollback.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4R. Do not start later planning or implementation.
