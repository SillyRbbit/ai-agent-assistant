# Documentation reconciliation F-15

Status: Complete
Owner: Project owner
Last updated: 2026-08-26

## Goal

Make current requirement, architecture, and status claims match verified source
evidence before any later connected agent UI work is considered.

## Scope

Assign the synthetic-gateway requirement a unique identifier; correct the
Command Center rendered-matrix status; disclose the missing app-info runtime
narrowing and production CSP development-WebSocket allowance; and add narrow
repository-health regression coverage.

## Explicit non-goals

No Rust/Tauri/CSP behavior, capability, dependency, provider, model, workflow,
IPC, storage, credential, filesystem, approval, or UI-runtime change.

## Current-state evidence

The native nine-agent review records F-15 in
`docs/reviews/NATIVE_NINE_AGENT_ARCHITECTURE_REVIEW.md`. It identifies duplicate
`FR-020`, stale rendered-matrix wording, and architecture claims that exceed
the current app-info and CSP source boundaries.

## Files expected to change

- `PRODUCT_REQUIREMENTS.md`
- `ARCHITECTURE.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- current project-memory and completion records

## Interfaces and invariants

- Requirement identifiers are unique in `PRODUCT_REQUIREMENTS.md`.
- Current-state markers describe source, not planned remediation.
- Repository health rejects duplicate identifiers and stale/missing F-15
  markers without reading runtime state.
- No application behavior changes.

## Verification

- focused documentation-truth unit tests
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`

## Target-Mac checks

Not applicable: this increment changes no native behavior. Stop if target-Mac
validation becomes necessary to support a documentation claim.

## Security review

No trust boundary is added. Review must confirm that the corrected documents do
not overstate runtime IPC narrowing or production CSP restrictions.

## Rollback and stop conditions

Revert only this documentation/health-check increment. Stop if source
contradicts a correction, if static checks require runtime behavior, or if any
Tauri, capability, CSP, dependency, or product-source change becomes necessary.

## Final results

The duplicate requirement identifier, stale rendered-matrix claim, and
overstated app-info/CSP claims were corrected. The focused documentation-truth
tests, complete repository tests, documentation, repository, security, and
diff checks passed. No target-Mac check applied because native behavior did not
change.
