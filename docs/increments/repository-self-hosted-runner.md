# Repository Workflow Increment - trusted self-hosted runner routing

Status: Verification pending
Date: 2026-07-17
Owner: Project maintainer

## Goal

Use the registered repository-scoped Linux x64 runner for Cortexa's existing
read-only GitHub Actions checks without executing pull-request workflow
definitions on the persistent host.

## Root cause

The runner was online but exposed only default self-hosted Linux x64 labels.
The three workflows selected GitHub-hosted macOS or Ubuntu images, so the runner
could not receive them. PR #23's hosted jobs failed before checkout because the
account could not start GitHub-hosted jobs.

## Implemented scope

- Added the custom `cortexa-ci` label to runner 21.
- Routed CI, Documentation, and Security through the exact four-label selector.
- Removed `pull_request` triggers and allowlisted only maintainer-controlled
  repository branch families for push checks.
- Added fail-fast host prerequisite checks without workflow `sudo`.
- Added repository-health enforcement and focused regression tests.
- Added the authoritative host and trust-boundary guide.
- After Linux CI exposed target-conditional dead code, target-gated only private
  approval-source imports, presentation state/parts, conversion methods, and
  evidence constructors whose sole consumer is the macOS decision source.

## Boundaries preserved

No public approval contract, target-Mac behavior, dependency, lockfile, Tauri
command, capability, CSP, permission, database, identifier, signing,
deployment, publication, or secret changed. Linux workflow evidence does not
replace native macOS evidence.

## Verification state

Passed before edits:

- `npm run test:repository` - 16 tests passed.
- `npm run docs:check` - formatting and internal links passed.
- `npm run repository:check` - all repository-health checks passed.

Passed after focused implementation:

- `npm run test:repository` - 19 tests passed.
- Runner API inspection - runner 21 is online and has `self-hosted`, `Linux`,
  `X64`, and `cortexa-ci`.
- Ruby YAML parse - all three changed workflow files passed.
- `npm run docs:check`, `npm run repository:check`, and
  `npm run security:scan` - passed.
- `npm run verify` - formatting, policy, lint, strict Clippy, 28 hook tests, 19
  repository tests, 124 frontend tests, 95 Rust library tests, 21 Rust
  integration tests, typecheck, Vite builds, and the Tauri release no-bundle
  build passed.
- `git diff --check` and complete changed-path review - passed with no product
  behavior change.
- Rust formatting passed after the approved correction.
- Strict all-target Clippy with all features and warnings denied passed after
  the approved correction.
- All six existing focused approval-manager tests passed.
- Final `npm run verify` - formatting, policy, lint, strict Clippy, 28 hook
  tests, 19 repository tests, 124 frontend tests, 95 Rust library tests, 21
  Rust integration tests, typecheck, Vite builds, and Tauri no-bundle build
  passed with the two-file correction.

Intermediate failures:

- One post-edit documentation check and one later `npm run verify` attempt
  stopped only on Prettier wraps in the new plan. Formatting that file and
  rerunning produced the final passing results above.
- The first post-correction documentation check reported only formatting in
  `HANDOFF.md`, this increment record, and the consolidated report. Formatting
  those exact files made the complete rerun pass.

Still required:

- Obtain separate approval to commit and push the exact two-file correction and
  current closeout updates to PR #24.
- Rerun CI successfully on the corrected branch. Documentation and Security
  already pass on runner 21 but must pass again for the pushed commit.
- Update the existing `FAIL` post-increment report with successful remote
  evidence and finalize a valid completion marker.

## Rollback

Restore the previous hosted selectors and remove the trust/preflight blocks,
then remove the `cortexa-ci` custom label from runner 21. Re-run local and hosted
checks before treating rollback as complete.
