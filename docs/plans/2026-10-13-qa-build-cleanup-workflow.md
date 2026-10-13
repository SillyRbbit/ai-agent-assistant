# Task-owned QA-build cleanup workflow

Owner authorization: 2026-10-13 request. Increment: qa-build-cleanup-workflow.
Detached task checkout at ef05c48; other worktrees and historical failures retained.

## Objective and scope

Extend END_SESSION.md minimally, link it from ENGINEERING_GUIDE.md and update
HANDOFF, PROJECT_STATUS, NEXT_STEPS, DECISIONS, CHANGELOG, PLANS and these reviews.
No product/hook/permission change, artifact deletion, process signals, runtime,
credential/provider use, branch, commit, push or merge. Publication remains separate.

## Acceptance

- C1: explicit implementation, checks, native QA, owner review, authorized publication,
  applicable post-merge verification and dependency-free eligibility.
- C2: exact artifact ownership/size, durable source/configuration/evidence, safe
  process targeting, narrow removal and verified preservation/recovery reporting.
- C3: protected categories and persistent-memory C7 retention recorded in handoff.
- C4: existing workflow integration, proportionate validation and truthful closeout.

## Risk, dependencies and verification

Premature cleanup can lose acceptance or rollback evidence; require all conditions,
exact inventory and verified durable custody. Existing C7/runtime and historical
failed criteria remain independent of this documentation-only objective. Never
use this checkout to complete, waive or reopen those failures.

Run npm run docs:check, npm run repository:check, npm run security:scan,
git diff --check and python3 -B .codex/hooks/session_end_gate.py. Review complete
diff for architecture, security, code health, debt and readiness. No product build
or live QA needed. Use normal schema-2 admission and existing completion gate.

## Preservation and rollback

Original documents and SHA-256 manifest are copied outside temporary directories:
/Users/hdang/.codex/visualizations/2026/10/13/01a13321-ac73-7a70-8c1c-ef542b1452cd/qa-cleanup-workflow-evidence.
Preserve all external artifacts and other checkouts. A later authorized rollback
must inspect concurrent edits before restoring only this task's documentation.

## Progress

All four documentation criteria are implemented and directly reviewed. The five
required checks passed; final report/documentation checks precede gate finalization.
No cleanup performed; current memory artifacts retained. The first sandbox write
was denied before edits and a scoped platform-approved write succeeded. First
admission rejected noncanonical review labels; corrected labels passed the existing
unchanged validator. Readiness report is now frozen. No failure was relabeled.
Final gate receipt, not this plan, determines completion-marker validity.
