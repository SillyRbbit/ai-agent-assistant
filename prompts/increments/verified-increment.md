# Verified Increment

- **Category:** Increment
- **Purpose:** Implement one already Ready or explicitly selected bounded increment.
- **Use when:** The repository records a complete goal, scope, risks, verification, and rollback.
- **Do not use when:** Readiness is unresolved, the baseline is failing, or the request combines unrelated work.
- **Required inputs:** `{{INCREMENT_NAME}}`, `{{PLAN_PATH}}`, and `{{BRANCH_NAME}}`.
- **Expected outputs:** One approved implementation, focused tests, verified closeout evidence, synchronized project memory, and no automatic publication.
- **Related skills:** `$verified-increment`, `$quality-gate`, `$post-increment-gate`.
- **Related prompts:** [Feature implementation](feature-implementation.md), [Quality gate](../reviews/quality-gate.md), [End session](../workflows/end-session.md).
- **Last reviewed:** 2026-07-18

## Prompt

```text
Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md. Apply the current task-specific instructions below without violating the approved increment.

Use $verified-increment.

Implement only {{INCREMENT_NAME}} as documented in {{PLAN_PATH}} on {{BRANCH_NAME}}.

Before editing, read AGENTS.md and the authoritative engineering, architecture, security, testing, project-memory, and increment documents. Inspect Git, confirm a clean synchronized baseline, run the smallest relevant baseline check, and revalidate that the increment is Ready.

State the exact goal, non-goals, changed files, risks, verification, manual gates, and rollback. Wait for project-owner approval. After approval, begin the mandatory repository gate before edits.

Preserve behavior outside scope. Add focused success and failure tests, use typed errors at production boundaries, and do not weaken security or quality controls. Stop and request approval before expanding any declared file or capability boundary.

Follow the Risk-Based Validation Policy in MASTER_PROMPT.md and ENGINEERING_GUIDE.md; task-specific validation may be stricter. Review the complete diff for correctness, architecture, security, technical debt, and accidental files. Synchronize project memory only from observed evidence, create the required increment and review records, and finalize a valid post-increment marker.

Treat GitHub Actions as publication evidence only. Confirm which hosted jobs are applicable from TESTING_GUIDE.md and the workflow path policy, but do not substitute a pending, skipped, or successful hosted run for the local final increment gate. When repository paths change, include the relevant workflow filters, scripts/ci_change_scope.py, focused classifier tests, and policy documentation in the approved scope.

Do not commit, push, merge, publish, release, or start another increment unless separately instructed.
```
