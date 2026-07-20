# Behavior-Preserving Refactor

- **Category:** Increment
- **Purpose:** Improve internal structure while preserving every supported observable contract.
- **Use when:** Concrete coupling, duplication, ownership, portability, or maintainability evidence justifies a bounded structural change.
- **Do not use when:** User-visible behavior, permissions, schema, dependencies, or architecture authority must change.
- **Required inputs:** `{{REFACTOR_GOAL}}`, `{{PRESERVED_CONTRACTS}}`, `{{FILE_SCOPE}}`, and `{{BRANCH_NAME}}`.
- **Expected outputs:** One bounded structural change with contract-focused regression evidence and no behavior drift.
- **Related skills:** `$architecture-review`, `$technical-debt`, `$verified-increment`.
- **Related prompts:** [Feature implementation](feature-implementation.md), [Code review](../reviews/code-review.md), [Quality gate](../reviews/quality-gate.md).
- **Last reviewed:** 2026-07-18

## Prompt

```text
Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md. Apply the current task-specific instructions below without violating the approved increment.

Use $verified-increment for this behavior-preserving refactor.

Goal: {{REFACTOR_GOAL}}
Contracts that must remain unchanged: {{PRESERVED_CONTRACTS}}
Exact file scope: {{FILE_SCOPE}}
Branch after approval: {{BRANCH_NAME}}

Read AGENTS.md and the applicable engineering, architecture, testing, security, source, test, and technical-debt evidence. Confirm the current contracts from code and tests rather than documentation claims alone.

State the structural root cause, exact edits, protected behavior, risks, regression tests, verification, and rollback. Wait for project-owner approval before creating the branch, beginning the gate, or editing.

Do not add features, dependencies, permissions, schema changes, public API changes, or speculative abstractions. Keep the refactor within {{FILE_SCOPE}} and stop before scope expansion.

Run focused contract tests and architecture and code review. Follow the Risk-Based Validation Policy in MASTER_PROMPT.md and ENGINEERING_GUIDE.md; task-specific validation may be stricter. Run the post-increment gate. Report any behavior change as a blocker rather than accepting it as refactor fallout. Do not commit, push, merge, or start another increment automatically.
```
