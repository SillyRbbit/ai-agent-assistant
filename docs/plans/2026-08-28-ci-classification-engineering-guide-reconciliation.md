# CI-classification engineering-guide reconciliation

Status: Verified complete with advisories
Owner: Henry Dang
Last updated: 2026-08-28
Increment: `ci-classification-engineering-guide-reconciliation`

## Goal

Reconcile the one stale high-level CI policy statement with the verified
fail-closed classifier without changing executable behavior.

## User-visible outcome

None. This increment changes documentation and current-state evidence only.

## Scope

- Change `ENGINEERING_GUIDE.md` so isolated Rust tests and production/native
  examples are described accurately.
- Reconcile current project memory with the observed PR #76 and merged-main
  publication evidence for the completed classifier increment.
- Create the required documentation-only increment, plan, review, and gate
  evidence.

## Explicit non-goals

- `scripts/ci_change_scope.py`, its tests, workflows, runner selectors,
  permissions, dependencies, lockfiles, capabilities, CSP, or product source.
- Credentials, filesystem/document access, memory, Tauri IPC, providers,
  persistence, tools, networking, device effects, or publication changes.

## Existing behavior and constraints

At synchronized `main` `7390ea6b704dd8d9c456ac2701d3287fed981442`, the
published classifier selects frontend, Rust, and audit jobs for every
`src-tauri/src/**` and `src-tauri/examples/**` Rust path. `src-tauri/tests/**`
remains Rust-only. `ENGINEERING_GUIDE.md` incorrectly grouped examples with
isolated Rust tests. PR #76 and its merged-main CI and Documentation workflows
have passed.

## Files expected to change

- `ENGINEERING_GUIDE.md`
- `CHANGELOG.md`, `HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`, and
  `PROJECT_STATUS.md`
- This plan, its increment record, and its post-increment review

## Interfaces and invariants

- No executable file, CI classifier rule, workflow, or trust boundary changes.
- The guide distinguishes Rust tests from production source/native examples.
- Current-state records report only observed publication evidence.
- Historical classifier closeout evidence remains unchanged.

## Implementation milestones

- [x] Reconcile clean synchronized baseline and published workflow evidence.
- [x] Apply the minimal wording correction and current-state update.
- [x] Run documentation completion checks and finalize the gate.

## Security and privacy considerations

No credential, provider, user, filesystem, or application data is introduced.
Accurate classification guidance prevents future under-classification of
production native examples.

## Test plan

- Inspect the changed policy sentence against the current classifier and
  `TESTING_GUIDE.md`.
- Run the normal documentation-only validation set.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Risks

- A broad wording change could overstate executable behavior.
- Current-state records could confuse historical closeout evidence with new
  publication evidence.

## Rollback or failure strategy

Before publication, change only this increment's documentation. Do not reset,
clean, stash, or discard unrelated work. Stop if documentation checks fail.

## Decisions made

- The smallest truthful correction names Rust tests as isolated and
  production/native examples as application-job selectors.
- PR and merged-main workflow results are current-state evidence; the prior
  local closeout report remains historical.

## Discoveries

- PR #76 squash-merged `d322317` at `7390ea6`; all six PR checks passed.
- Merged-main CI `33199321088` and Documentation `33199321090` passed.

## Progress

- 2026-08-28: Owner approved the separate documentation reconciliation.
- 2026-08-28: Clean synchronized baseline, published Actions results, and
  active gate confirmed.
- 2026-08-28: Corrected the guide, reconciled current-state publication
  evidence, and passed the complete documentation-only gate.

## Acceptance criteria

- [x] The engineering guide no longer calls production/native examples
      isolated Rust tests.
- [x] Current-state publication records are accurate without rewriting history.
- [x] Documentation-only verification and completion gate pass.

## Final results

`PASS WITH ADVISORIES`. The guide now preserves isolated Rust tests while
describing production Rust source and native examples as selecting affected
application jobs. Current project memory records PR #76's squash merge at
`7390ea6` and the passing PR and merged-main Actions evidence. Documentation,
repository, secret, whitespace, session-end, and completion-marker checks pass.
Target-Mac UI checks are `Not run` because no application behavior changed.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md` not required
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md` not required
