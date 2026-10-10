# Selective main Linux routing publication

Date: 2026-10-10
Status: Owner-authorized bounded publication preparation; external acceptance pending

## Objective and dependencies

Publish only the completed routing behavior on remote main
`de0d6bcbb63e5377825da4edd6d31bfd98d03dfb`, preserving the independently completed
local remediation and every historical failure. The source is
`/Users/hdang/.codex/worktrees/vps-main-linux-routing/ai-agent-assistant` (R).
The isolated publication checkout is
`/Users/hdang/.codex/worktrees/main-linux-routing-publication/ai-agent-assistant`
on `codex/ci/main-linux-routing-publication` (E). The owner expressly authorizes
publication-only preparation without ordinary gate admission. It does not publish
the mixed 28-path source candidate or adopt D-137 for other tasks.

Authority, budgets, snapshots and publication receipts are preserved in
`/Users/hdang/.codex/backups/cortexa-routing-publication-20261010-01` (Q).
Source final-001 evidence remains in R's ignored
`.codex/state/d137-routing-local-remediation-20261009-01` package and archive.
The original FAIL and D-136 preparation/request remain unchanged and unconsumed.
PR #139 and its failed diagnostic remain isolated.

## Exact editable scope

- `.github/workflows/ci.yml`
- `.github/workflows/documentation.yml`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `docs/github/SELF_HOSTED_RUNNER.md`
- `DECISIONS.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- `docs/plans/2026-10-10-main-linux-routing-publication.md`
- `docs/reviews/2026-10-10-main-linux-routing-publication-review.md`

The first four paths must match the tested source bytes. The other nine use
additive/current documentation based on remote main. Do not copy mixed D-136
documentation, hooks, instruction changes, local Stop configuration, task-local
executables or ignored evidence into the export. No product or validator changes.

## Routing behavior and security

Only the five Linux `runs-on` selectors change: `refs/heads/main` selects
`cortexa-linux`; every other ref selects `cortexa-ci`. Existing events, job
conditions, steps, permissions and concurrency remain unchanged. No
`pull_request` or `pull_request_target` execution is added. Labels are selectors,
not trust boundaries. The Mac selector is unchanged.
VPS24 retains `cortexa-linux` without `cortexa-ci`; offline local Linux23 and
Mac22 retain their current labels. No infrastructure changes are authorized.
The repository policy checker and its workflow-policy regressions enforce this
contract; the module contains 61 total tests across its policy areas.

## Work and finite budgets

Preserve original work start 2026-10-10 04:09:37 CDT, work deadline 08:09:37 CDT,
and preservation-only deadline 08:19:37 CDT. Recovery is limited to one hour,
two corrective retries per stable stage and six overall, with no resets across
sessions. Two worktree-preparation retries are consumed, so that stage has none
remaining. Q's additive ledger is authoritative for exact accumulated usage.
Attended fixed checks have a two-minute command limit and five-second cancellation
bound. This existing local executor is owner-accepted for these checks only;
it does not establish unattended runner guarantees. Do not use the task-bound
D-137 runner for export or Git operations. Stop substantive work at the deadline;
preservation closeout may preserve receipts and report truthfully, not test/repair.

## Validation and evidence reuse

Copy installed dependencies without acquisition; use the preserved shim:
repository checker/tests use framework Python 3.12, hook/session routes use the
system interpreter. Bind tool binaries, package lock, final thirteen paths,
unchanged relevant product inputs and every fresh command receipt.

Fresh requirements: `npm run test:hooks`, `npm run test:repository`,
`npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
`git diff --check`, and `python3 -B .codex/hooks/session_end_gate.py`.
Obtain an independent focused extraction/documentation/evidence review.
Reuse source-bound full verification only for unchanged product, Rust, frontend
and dependency inputs. Do not call this a fresh composite full verify.
The retained run contains 601 frontend cases; 714 unique passing native tests
produced 1,173 executions (459 repeated library executions), with one unique
ignored opt-in case. Strict Rust, frontend checks and release build passed.
Fresh hook/repository totals must come from E, not the mixed source's totals.

## Publication and actual Actions acceptance

After checks and independent review, create one reviewed commit, push the one
branch and create/attach one dedicated PR. A push may enqueue branch workflows;
queued is not passing. Reconcile main identity, exact PR tree, effective rules,
runner identities and queue immediately before the single conditional squash
merge. The owner permits merge before offline local-Linux branch checks finish
only if effective rules permit. Do not bypass rules or modify settings.
Existing main Documentation concurrency may supersede queued run `37948922997`;
no manual dispatch, rerun, cancel or unrelated queue manipulation is authorized.

Observe automatic main CI and Documentation for at most one hour after merge.
At the exact merged SHA, require actual successful classification, frontend,
Linux Rust, dependency-audit and Documentation on VPS24, and target-Mac Rust on
Mac22. Retain run/job IDs, runner assignments, logs and conclusions. Failure,
cancellation, skipping, pending or wrong assignments block activation acceptance.
Keep local macOS and manual VPS evidence separate. A partial publication result
must remain partial; no automatic rollback or infrastructure repair is authorized.

## Preservation, stops and deferred work

Preserve R's source/index/gate/report/archive bindings and all prior results.
Two earlier managed worktree attempts failed with Git mmap errors. The managed
tool attempted internal cleanup with unconfirmed exit; no agent cleanup command
was issued. After authorized Finder materialization the remaining retry succeeded.
Retain strict comparison failure (exit 2), Finder `.DS_Store` change and unavailable
historical `info/packs` byte binding. Materialization is not proof of the mmap cause.

Stop on drift, missing access, conflicting contents, uncertain ownership or
termination, exhausted limits, rule conflict or required implementation/scope
change. Never replay uncertain external operations. No cleanup, force-push,
branch deletion, deployment, local-runner recovery, broader adoption or PR #139
action. Architecture overview and detailed runbook, each in Word/PDF, remain
deferred until deployment and applicable validation finish.

## Progress and results

The remaining worktree retry succeeded; E is based on the reconciled main SHA.
The four implementation files match R. Installed dependencies were copied without
acquisition. All seven fresh extraction checks passed (145 hook and 98 repository tests).
Final documentation checks and independent review bind the frozen export; see the
[publication review](../reviews/2026-10-10-main-linux-routing-publication-review.md)
and Q's final input-bound receipts for their settled results. Publication and
Actions acceptance are separate later stages of this same bounded authority.
