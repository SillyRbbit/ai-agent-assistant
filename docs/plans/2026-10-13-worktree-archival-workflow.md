# Task-owned worktree archival workflow

Owner authorization: 2026-10-13 documentation-only request.
Increment: worktree-archival-workflow. Detached task checkout at
6e663ce2114bd393d4340f540944ff77e3351cb9, the merged QA-cleanup workflow baseline.

## Objective and scope

Add a minimal separate archival procedure to END_SESSION.md, link it from the
Definition of Done in ENGINEERING_GUIDE.md and add current entries to HANDOFF,
PROJECT_STATUS, NEXT_STEPS, DECISIONS, CHANGELOG and PLANS. Add this plan and
readiness/final reviews through the existing planning and completion route.
No actual archival/removal, deletion, process signals, credentials, provider use,
product/hook/configuration change, branch, commit, push, merge or publication.
Keep the original QA-cleanup candidate and publication checkout untouched.

## Acceptance

- C1: completed acceptance, required owner review, applicable authorized publication
  and passing post-merge verification, exact ownership, no dependencies and explicit
  archival authorization are conjunctive prerequisites; unknown means retain.
- C2: prefer managed recoverable archival, verify durable source/evidence and
  necessary ignored files, protected categories and exact inventory before action.
  Define preservation verification, recovery semantics and measured reporting.
- C3: QA cleanup still protects worktrees; retain persistent-memory worktrees,
  review build and required artifacts pending C7 and remaining closeout.
- C4: existing workflow integration, preserved history, complete diff review,
  applicable passing documentation checks and truthful completion-gate closeout.

## Risk, dependencies and verification

Lost source, ignored gate state or rollback artifacts are the main archival risks.
Require verified durable custody outside the checkout and temporary directories;
never infer recoverability from a tool success alone. Managed archival exclusions
remain binding. This procedure does not complete, waive or admit any failed
product criterion, including persistent-memory C7; those dependencies prohibit
archival of the affected work, but do not block this independent documentation.

Run npm run docs:check, npm run repository:check, npm run security:scan,
git diff --check and python3 -B .codex/hooks/session_end_gate.py. Review architecture,
security, code health, debt, readiness, scope and preservation. No product tests,
builds, runtime probes or native QA apply to prose-only changes. Freeze passing
readiness, use schema-2 general admission before implementation, then the normal
report/finalize/status/complete-payload Stop route after final affected checks.

## Preservation and recovery

Eight original document snapshots, SHA-256 inventory, clean/inactive baseline and
unchanged-work records are retained outside temporary directories at
/Users/hdang/.codex/visualizations/2026/10/13/01a13321-ac73-7a70-8c1c-ef542b1452cd/worktree-archival-workflow-evidence.
Preserve protected instructions, hooks, product/dependency/configuration files and
all historical reports. Rollback would need separately authorized, attributed
restoration of only this task's documents; inspect concurrent changes first.
Do not archive this candidate while owner review or publication acceptance is pending.

## Progress

Preparation checks passed and frozen readiness admitted by the unchanged schema-2
ordinary begin. All four documentation criteria are implemented and directly
reviewed. Five required implementation checks and preservation review passed.
Affected final checks precede normal gate/status/complete-payload Stop closeout;
the receipts determine completion validity. No archival eligibility is implied.
No archival/removal or artifact cleanup performed. Existing source and historical
records remain protected; no other session state imported or modified.
