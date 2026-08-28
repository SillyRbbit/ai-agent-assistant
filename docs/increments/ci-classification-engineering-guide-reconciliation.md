# CI-classification engineering-guide reconciliation increment

Status: Verified complete with advisories
Owner: Henry Dang
Date: 2026-08-28
Plan: [2026-08-28-ci-classification-engineering-guide-reconciliation.md](../plans/2026-08-28-ci-classification-engineering-guide-reconciliation.md)

## Goal

Correct one stale CI-policy sentence and record the published classifier
increment's observed workflow evidence without changing executable behavior.

## Scope

- `ENGINEERING_GUIDE.md`
- Required current project-memory, plan, increment, review, and changelog
  evidence only

## Non-goals

No classifier, test, workflow, runner, product source, dependency, lockfile,
capability, CSP, permission, credential, IPC, provider, persistence,
filesystem, tool, network, device, branch, commit, push, merge, release, or
publication change.

## Baseline evidence

- Clean synchronized `main`:
  `7390ea6b704dd8d9c456ac2701d3287fed981442`.
- PR #76 and its exact merged-main CI and Documentation workflows passed.
- The production/example classifier is already verified; only the guide's
  obsolete test/example grouping is inaccurate.
- Gate: `ci-classification-engineering-guide-reconciliation` active.

## Acceptance criteria

- The guide distinguishes isolated Rust tests from production/native examples.
- Current-state records distinguish local closeout history from publication.
- Required docs-only checks, review, session-end, and marker pass.

## Results

`PASS WITH ADVISORIES`. The guide now correctly distinguishes isolated Rust
tests from production Rust source and native examples. Current-state records
now include PR #76, squash merge `7390ea6`, all six passing PR checks, and the
passing merged-main CI and Documentation workflows. Documentation-only checks,
review, session-end, and marker pass; target-Mac UI checks are `Not run`.
