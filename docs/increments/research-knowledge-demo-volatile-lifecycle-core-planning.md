# Research/Knowledge demo volatile lifecycle core planning

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-27

## Goal

Create one exact, implementation-ready ExecPlan for a manually stepped, volatile Rust lifecycle core using the existing sealed D-086 workflow before any lifecycle IPC or presentation work.

## Scope

- Define the private core host, closed lifecycle vocabulary, ownership, cleanup, test, target-Mac, rollback, and stop-condition contract.
- Reconcile only current planning and handoff records.

## Non-goals

No product source, Tauri command/event, frontend/UI, capability, CSP, permission, dependency, provider, model, network, credential, tool, approval, persistence, filesystem, timer, thread, async worker, or device behavior.

## Evidence

- The read-only projection prerequisite is complete, valid, and source-current.
- Readiness is `Ready with advisories` for documentation planning and `Blocked` for source implementation until the owner separately approves the exact plan.
- Existing D-086 and F-01/F-02 contracts already supply the Rust validation, cancellation, late-event, and quarantine foundations.

## Verification

- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- session-end and post-increment review/marker validation

## Progress

- 2026-08-27: Gate began on clean `main` at `8d9df6e`. Drafted the exact Rust-core lifecycle ExecPlan and synchronized planning-state records. No implementation began.
- 2026-08-27: Documentation, repository, secret, diff, session inventory, and
  independent review checks pass. The completion result is `PASS WITH
ADVISORIES`: source approval, implementation, and target-Mac runtime evidence
  remain outside this documentation-only increment.

## Final results

This planning increment is verified complete with advisories. It creates the
exact Rust-core lifecycle ExecPlan and current-state handoff only. The next
action is explicit owner approval of that source plan; no source gate began.
