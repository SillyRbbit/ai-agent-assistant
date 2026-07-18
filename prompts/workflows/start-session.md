# Start or Resume a Session

- **Category:** Workflow
- **Purpose:** Reconcile the actual repository and define one approved session goal before edits.
- **Use when:** Beginning a new task or resuming work from `HANDOFF.md`.
- **Do not use when:** The current task is already active and its verified scope has not changed.
- **Required inputs:** `{{SESSION_MODE}}` as `start` or `resume`, plus `{{REQUESTED_TASK}}` or `HANDOFF.md`.
- **Expected outputs:** Reconciled Git/toolchain state, one bounded goal, non-goals, exact files, risks, verification, and an approval pause.
- **Related skills:** `$session-start`, `$resume-session`, `$readiness-review`.
- **Related prompts:** [Verified increment](../increments/verified-increment.md), [Readiness review](../reviews/readiness-review.md), [End session](end-session.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $session-start when {{SESSION_MODE}} is start, or $resume-session when {{SESSION_MODE}} is resume.

Requested task or handoff source: {{REQUESTED_TASK}}

Read AGENTS.md and its complete required reading order. For resume mode, begin with HANDOFF.md and continue only its exact recorded task after reconciling it with Git. Inspect branch, working tree, recent commits, active gate state, toolchain, and relevant source and tests.

Run the narrowest baseline check that proves the repository state needed for this task. Identify stale documentation, uncommitted work, or conflicts between repository evidence and the handoff.

State the current phase, one session goal, explicit non-goals, exact expected files, risks, verification commands, manual gates, rollback, and blockers. Do not create a branch, begin a gate, edit, commit, push, merge, or start unrelated work until the baseline and scope are clear and project-owner approval is given.
```
