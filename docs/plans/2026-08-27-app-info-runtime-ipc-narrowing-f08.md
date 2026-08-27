# F-08 app-info runtime IPC narrowing

Status: Draft — not implementation-approved
Owner: Project owner
Last updated: 2026-08-27

## Goal

Accept the existing `get_app_info` WebView result only as an exact closed DTO
with `secureCore: true`, and map every rejection to fixed safe UI copy.

## Scope and invariants

- Invoke as `unknown`, then validate exactly six known camel-case keys.
- Require `environment` to be `development` or `production` and
  `secureCore === true`.
- Limit each string field to 128 Unicode code points; reject missing, unknown,
  wrong-type, oversized, or otherwise invalid values.
- Never place rejected data or upstream `Error.message` into UI state.
- Preserve the sole `get_app_info` command and its Rust response unchanged.

## Non-goals

No command/event, Rust, CSP, capability, permission, dependency, agent IPC,
provider, model, tool, approval, execution, network, credential, persistence,
filesystem, background work, audit, or device-effect change.

## Expected paths

- `src/infrastructure/tauri/app-info-client.ts`
- `src/application/useCoreConnection.ts`
- focused app-info/App tests
- current-state closeout records only when implementation is complete

## Required tests and manual gates

Test accepted data plus missing, extra, wrong-type, oversized,
invalid-environment, false-secure-core, and sensitive-error-sentinel cases.
Run focused frontend tests, typecheck, repository/security checks, complete
verification, and `git diff --check`. On the target Mac, confirm a valid
response retains ready state and rejected test-seam data shows fixed safe copy.

## Risks, rollback, and stop conditions

Over-strict validation can falsely mark the core unavailable; under-validation
preserves UI-integrity and error-disclosure risk. Revert only bounded paths.
Stop if test seams require a broader abstraction, or any new command, event,
capability, or policy decision is needed.

## Approval boundary

This draft is documentation only. It does not authorize a gate, source edit,
commit, push, merge, or later increment. Separate owner approval of a Ready
plan is required before implementation.
