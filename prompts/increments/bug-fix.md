# Bug Fix

- **Category:** Increment
- **Purpose:** Reproduce and correct one concrete defect with the smallest verified change.
- **Use when:** A command, test, build, or user-visible behavior has a specific reproducible failure.
- **Do not use when:** The issue is an unverified advisory, broad feature request, dependency refresh, or behavior-preserving refactor.
- **Required inputs:** `{{BUG_DESCRIPTION}}`, `{{REPRODUCTION_COMMAND}}`, and `{{BRANCH_NAME}}`.
- **Expected outputs:** Reproduction evidence, one root cause, focused regression coverage, and verified resolution.
- **Related skills:** `$troubleshoot`, `$verified-increment`, `$code-review`.
- **Related prompts:** [Refactor](refactor.md), [Technical-debt review](../reviews/technical-debt.md), [End session](../workflows/end-session.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $troubleshoot, then $verified-increment for this defect:

{{BUG_DESCRIPTION}}

Reproduction command or steps:
{{REPRODUCTION_COMMAND}}

Use {{BRANCH_NAME}} only after approval. Read AGENTS.md, HANDOFF.md, TROUBLESHOOTING_LOG.md, docs/workflows/TROUBLESHOOTING.md, and the applicable source, tests, architecture, security, and testing documents. Capture the sanitized environment and Git state.

Reproduce the narrowest failure and test one evidence-based hypothesis at a time. Do not delete lockfiles, upgrade unrelated dependencies, weaken security or quality controls, or make broad source changes without evidence.

Once the root cause is proven, state the exact fix, files, regression tests, risks, non-goals, verification, and rollback. Wait for project-owner approval before creating the branch, beginning the gate, or editing.

Implement only the approved correction. Re-run the original failure, focused regression test, complete required verification, and the post-increment gate. Record a reusable diagnosis in TROUBLESHOOTING_LOG.md only when the issue can recur. Do not commit, push, merge, or begin another increment automatically.
```
