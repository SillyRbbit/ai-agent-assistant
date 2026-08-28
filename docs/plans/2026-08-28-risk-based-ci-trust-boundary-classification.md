# Risk-based CI trust-boundary classification

Status: Verified complete with advisories
Owner: Henry Dang
Last updated: 2026-08-28
Increment: `risk-based-ci-trust-boundary-classification`

## Goal

Close the risk-based CI classification gap so every current or future
production Rust trust-boundary path selects frontend, Rust, and audit jobs while
the repository's deliberately narrower classes remain unchanged.

## User-visible outcome

None. This increment changes repository workflow classification and regression
evidence only; Cortexa runtime and presentation behavior remain unchanged.

## Scope

- Reproduce the owner-supplied under-classified paths and inventory current
  Rust production/trust-boundary examples.
- Make production Rust and trust-boundary examples fail closed to
  `frontend=true`, `rust=true`, and `audit=true`.
- Keep any future production Rust-only exception in an explicit exact-path
  allowlist; accept no production exception now.
- Add table-driven family, current-inventory, future-path, and preserved-
  behavior regression tests.
- Reconcile only the testing standard and required project-memory evidence.

## Explicit non-goals

- Product Rust, TypeScript, React, Tauri, IPC, storage, credential, document,
  memory, provider, tool, policy, approval, audit, or device behavior.
- Dependencies, manifests, lockfiles, capabilities, CSP, permissions, runner
  selectors, workflow triggers or permissions, and pull-request policy.
- Credentials, filesystem/document access, persistence, live networking,
  external resources, publication, or deployment.
- Branch creation, commit, push, PR, merge, release, or publication.

## Existing behavior and constraints

At synchronized `main` `3fc14e4f17bb171957dc09241a868f3c6deb7cb1`,
all eight representative paths classify as
`frontend=false rust=true audit=false`. The complete Rust production/example
inventory finds the same gap for credentials, documents, memory, menu-bar,
Research/Knowledge demo boundaries, and native trust-boundary examples.
Existing agent, policy, approval, audit, storage/startup, tool, app-info, error,
library, and main paths already select all three jobs through exact patterns.

D-057 requires unknown non-documentation paths to run both application jobs,
security-sensitive paths to select audit, and classifier rules, fixtures, and
the testing matrix to remain aligned. Workflow files are explicitly excluded
from this increment.

## Files expected to change

- `scripts/ci_change_scope.py`
- `scripts/tests/test_ci_change_scope.py`
- `TESTING_GUIDE.md`, only for the fail-closed production-Rust rule
- This plan and `docs/increments/risk-based-ci-trust-boundary-classification.md`
- `docs/reviews/2026-08-28-risk-based-ci-trust-boundary-classification-post-increment-review.md`
- Required current project memory: `HANDOFF.md`, `PROJECT_STATUS.md`,
  `NEXT_STEPS.md`, `PLANS.md`, and `CHANGELOG.md`

`DECISIONS.md` and `TROUBLESHOOTING_LOG.md` change only if evidence establishes
a new durable decision or reusable troubleshooting fact.

## Affected components

- Risk-based GitHub job classifier
- Standard-library classifier regression suite
- Risk-based testing documentation and completion evidence

## Interfaces and invariants

- `Scope` and its rendered output remain unchanged.
- Every `src-tauri/src/**/*.rs` path and applicable
  `src-tauri/examples/**/*.rs` path selects all three jobs.
- Synthetic future production Rust paths select all three jobs.
- Production Rust-only exceptions must be exact file paths in a named
  allowlist; wildcard/directory/current production exceptions are absent.
- `src-tauri/tests/**` remains Rust-only.
- Documentation-only, frontend-only, audit-only, schedule, manual dispatch,
  deletion, unknown non-documentation, and unsafe-path behavior remains exact.
- Existing path validation remains before classification.
- No GitHub workflow or runner policy changes.

## Implementation milestones

- [x] Reproduce the representative-path gap and inventory current paths.
- [x] Add fail-closed production Rust classification and the narrow allowlist.
- [x] Add table-driven family, inventory, future-path, allowlist, and preserved-
      behavior regression coverage.
- [x] Run focused and complete verification.
- [x] Complete independent reviews, documentation sync, and marker finalization.

## Security and privacy considerations

The script receives Git-produced path strings. Existing path validation must
remain before matching. The safe failure mode is additional CI, never omission
of a job. The allowlist must not accept globs or directories for production
source. Tests and docs contain paths only, with no credentials or user data.

## Test plan

- Table-drive every affected and already-protected trust-boundary family.
- Assert every current Rust file under `src-tauri/src` and
  `src-tauri/examples` selects all three jobs.
- Assert synthetic future production/example paths select all three jobs.
- Assert the production exception allowlist is empty and only structurally
  permits exact production file paths.
- Preserve exact expectations for documentation, frontend, audit-only, hooks,
  Rust tests, schedules, manual dispatch, dependencies/CI, deletions, unknown
  paths, and unsafe paths.

## Verification commands

```bash
python3 -m unittest scripts.tests.test_ci_change_scope -v
npm run test:repository
npm run repository:check
npm run security:scan
npm run docs:check
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Manual checks:

- Inspect the complete current Rust production/example inventory.
- Inspect the complete diff for prohibited product, workflow, runner,
  permission, dependency, or external-state changes.
- Target-Mac UI/native verification is not required because application and
  platform behavior do not change; record it as `Not run`.

## Risks

- Over-classification increases CI cost but fails safely.
- A broad exception pattern could recreate the gap.
- Matcher ordering could let the generic Rust class bypass audit.
- Documentation could overstate remote enforcement; only local classifier
  behavior is current evidence until publication and exact-run inspection.

## Rollback or failure strategy

Before publication, restore only this increment's classifier, tests, and docs.
Do not reset, clean, stash, or discard unrelated work. If a preserved class
changes or a required gate fails, stop and report the exact failure.

## Decisions made

- Treat all current production Rust source and trust-boundary examples as
  cross-cutting and audited. Accept no production Rust-only exception.
- Preserve `src-tauri/tests/**` as the narrow Rust-only family.
- Implement D-057's existing intent; no new architecture decision is required.

## Discoveries

- Exact patterns protect agent, policy, approval, audit, storage/startup, tool,
  and central Tauri glue paths, but generic `src-tauri/**` makes new production
  modules Rust-only.
- The complete baseline inventory contains 65 Rust files under
  `src-tauri/src` and two under `src-tauri/examples`. Fifty-four already
  selected all three jobs; thirteen current production/example files were
  under-classified.
- The final inventory classifies all 67 paths as frontend, Rust, and audit.
- `ENGINEERING_GUIDE.md` still describes Rust test/example paths together as
  isolated. The new testing guide is exact, but that broader guide is outside
  this increment's owner-authorized file list and remains a documentation
  advisory.

## Progress

- 2026-08-28: Owner selected and authorized the bounded increment.
- 2026-08-28: Clean synchronized baseline, reproduction, inventory, 17-test
  focused baseline, and active gate recorded.
- 2026-08-28: Implemented the production-source/example fail-closed rule, empty
  exact-source allowlist, and table-driven regression coverage.
- 2026-08-28: Focused tests pass 21/21; repository tests pass 80/80; all 67
  current production/example Rust paths select all three jobs.
- 2026-08-28: Complete verification, zero-finding npm audit, repository and
  secret scans, independent reviews, and completion gates pass. One opt-in
  Hermes executable probe remains intentionally ignored.

## Acceptance criteria

- [x] Listed and inventoried trust-boundary paths select all three jobs.
- [x] Tests cover every affected family and future fail-closed behavior.
- [x] Every required preserved behavior remains exact.
- [x] Prohibited files and behaviors remain unchanged.
- [x] Required verification and completion gates pass.

## Final results

`PASS WITH ADVISORIES`. All eight owner-supplied paths moved from
`frontend=false rust=true audit=false` to
`frontend=true rust=true audit=true`. The same final scope applies to 67/67
current production/example Rust files and synthetic future paths. The
production exception allowlist remains empty and structurally accepts only an
exact normalized Rust source file under `src-tauri/src`; examples, tests,
directories, globs, and non-Rust paths are ineligible.

Focused classifier tests pass 21/21, repository tests pass 80/80, and complete
`npm run verify` passes with 28 hook tests, 80 repository tests, 313 frontend
tests, 269 Rust library tests, 244 Rust integration tests, one intentional
ignored opt-in Hermes executable probe, the production frontend build, and the
Tauri release no-bundle build. Repository, secret, documentation, npm audit,
session-end, and diff checks pass. Target-Mac UI checks and remote GitHub
Actions are `Not run` because no product/native behavior changed and nothing
was published.

The initial sandboxed npm audit could not reach the registry; the authorized
network retry passed with zero vulnerabilities. The initial documentation
check identified only Prettier drift in the allowed testing guide; formatting
was corrected and the final documentation check passed.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md` not required unless durable policy changes
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md` not currently required
