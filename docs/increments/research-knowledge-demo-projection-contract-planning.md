# Research/Knowledge demo projection-contract planning

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-27

## Goal

Produce one implementation-ready ExecPlan for the smallest non-authorizing
Rust-to-WebView demo boundary after F-08.

## Scope

- Define the exact argument-free read-only projection command and v1 closed DTO.
- Record the dependency-ordered lifecycle and presentation follow-ons.
- Reconcile only current planning and handoff records.

## Non-goals

No product source, command, event, capability, permission, dependency, provider,
model, network, credential, tool, approval, persistence, filesystem, background
work, or device behavior changes.

## Evidence

- F-08 is complete and valid on published main before this planning increment.
- D-086 remains a sealed Rust proof and Command Center remains a separate
  frontend fixture proof.

## Verification

- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- Session-end and post-increment gate review

## Progress

- 2026-08-27: Gate began. Drafted the exact projection-contract ExecPlan and
  synchronized planning-state records. No implementation began.
- 2026-08-27: Documentation, repository, secret, diff, and session-end checks
  passed. The planning-only closeout remains advisory because implementation
  approval and all target-Mac/runtime evidence belong to the later increment.
