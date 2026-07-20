# Technical-Debt Review

- **Category:** Review
- **Purpose:** Identify and classify concrete maintainability debt without silently fixing it.
- **Use when:** Reviewing an increment, subsystem, or roadmap area for evidence-backed debt.
- **Do not use when:** The request is a broad cleanup based only on preference.
- **Required inputs:** `{{CHANGE_OR_AREA}}`, complete diff or source scope, active plan, and roadmap state.
- **Expected outputs:** Findings with category, severity, risk, effort, milestone, and blocking impact.
- **Related skills:** `$technical-debt`.
- **Related prompts:** [Code review](code-review.md), [Readiness review](readiness-review.md), [Remediation workflow](../workflows/remediation.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md. Apply the current task-specific instructions below without violating the approved increment.

Use $technical-debt.

Review {{CHANGE_OR_AREA}} using the complete diff, active plan, ARCHITECTURE.md, CODE_REVIEW.md, and roadmap state. Identify only concrete duplication, dead code, misleading abstractions, brittle tests, deferred failures, portability gaps, dependency issues, or documentation drift.

For every finding record category, Critical/High/Medium/Low/Advisory severity, concrete risk, effort, milestone, and whether it blocks completion or the next increment. Separate existing debt from newly introduced debt. Do not fix findings or expand scope.
```
