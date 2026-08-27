# UI/native static boundary F-12

Status: Complete
Owner: Project owner
Last updated: 2026-08-26

## Goal

Enforce the reviewed UI/native isolation baseline statically before any agent
IPC is considered.

## Scope

Add repository-health assertions and adversarial tests for the exact allowed
Tauri frontend imports, Command Center prohibited boundary APIs, app-info-only
invoke registration, single default capability, and the reviewed production CSP
baseline.

## Explicit non-goals

No Rust/Tauri/CSP behavior or configuration change, capability change,
dependency, provider, model, workflow, IPC, storage, credential, filesystem,
approval, or frontend runtime feature.

## Invariants

- The checker fails closed on a prohibited Command Center API, extra Tauri
  import, extra invoke handler, capability broadening, or CSP change.
- F-07's CSP remediation remains separate; the current CSP is enforced as a
  reviewed baseline, not newly approved behavior.
- No static check grants permission for a future demo command.

## Verification

- focused UI/native-boundary positive and adversarial tests
- `npm run test:repository`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`

## Target-Mac checks

Not applicable: no native runtime behavior changes. Stop if a target-Mac check
becomes necessary to support a claim.

## Security review and rollback

The review must confirm that enforcement adds no authority. Revert only this
check, its tests, and documentation if the static baseline is inaccurate.

## Final results

The exact static UI/native baseline and all adversarial cases pass. No
target-Mac check applied because native behavior did not change.
