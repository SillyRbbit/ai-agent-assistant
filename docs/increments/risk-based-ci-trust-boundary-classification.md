# Risk-based CI trust-boundary classification increment

Status: Verified complete with advisories
Owner: Henry Dang
Date: 2026-08-28
Plan: [2026-08-28-risk-based-ci-trust-boundary-classification.md](../plans/2026-08-28-risk-based-ci-trust-boundary-classification.md)

## Goal

Make current and future production Rust trust-boundary changes select frontend,
Rust, and audit CI jobs without changing application or workflow behavior.

## Scope

- `scripts/ci_change_scope.py`
- `scripts/tests/test_ci_change_scope.py`
- Narrow testing clarification and required plan/project-memory/review evidence

## Non-goals

No product source, dependency, lockfile, GitHub workflow, runner, permission,
capability, CSP, credential, filesystem, persistence, IPC implementation,
provider, tool, external resource, or publication change.

## Baseline evidence

- Clean synchronized `main`:
  `3fc14e4f17bb171957dc09241a868f3c6deb7cb1`.
- All eight representative paths reproduce as
  `frontend=false rust=true audit=false`.
- Eleven current production Rust files and both current trust-boundary examples
  are under-classified across credential, document, memory, menu-bar,
  Research/Knowledge demo, and native example families.
- Focused baseline: 17/17 classifier tests passed.
- Gate: `risk-based-ci-trust-boundary-classification` active.

## Acceptance criteria

- Every required trust-boundary family selects all three jobs.
- Current and synthetic future production Rust paths fail closed.
- Production exceptions require an exact documented allowlist; none is accepted
  now.
- Existing narrow classes and event behavior remain exact.
- Required focused, repository, security, documentation, full verification,
  audit, diff, session-end, review, and marker checks pass.

## Results

`PASS WITH ADVISORIES`. All eight representative paths now select frontend,
Rust, and audit. The exact current inventory is 65 Rust source files plus two
native examples: 13 were under-classified at baseline and 67/67 select all
three jobs after the change. Synthetic future source and example paths fail
closed; the production exception allowlist is empty and accepts only an exact
normalized `src-tauri/src/*.rs` file if a later separately reviewed change
deliberately adds one.

Focused classifier tests pass 21/21 and repository tests pass 80/80. Complete
verification passes with 28 hook tests, 80 repository tests, 313 frontend
tests, 269 Rust library tests, 244 Rust integration tests, one intentional
ignored opt-in Hermes executable probe, the production frontend build, and the
Tauri release no-bundle build. Repository, secret, documentation, zero-finding
npm audit, session-end, and diff checks pass. Target-Mac UI checks and remote
GitHub Actions are `Not run` because this changes no product behavior and is
unpublished.

One broader sentence in `ENGINEERING_GUIDE.md` still groups examples with
isolated Rust tests. It is outside the owner-authorized file list and is
recorded as a non-blocking documentation advisory rather than changed.
