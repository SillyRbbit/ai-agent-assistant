# Architecture Review

- **Category:** Review
- **Purpose:** Assess a change or plan for architecture accuracy, ownership, trust boundaries, and drift.
- **Use when:** A proposed or implemented change may affect modules, data flow, portability, dependencies, or system boundaries.
- **Do not use when:** The request is only a code-style review or asks to implement architecture changes immediately.
- **Required inputs:** `{{CHANGE_OR_PLAN}}` and the applicable plan or diff.
- **Expected outputs:** Evidence-backed findings in severity order and a boundary-consistency decision.
- **Related skills:** `$architecture-review`, `$readiness-review`.
- **Related prompts:** [Code review](code-review.md), [Security review](security-review.md), [Readiness review](readiness-review.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $architecture-review.

Review {{CHANGE_OR_PLAN}} against ARCHITECTURE.md, ENGINEERING_GUIDE.md, accepted decisions, and its exact increment boundary. Inspect the complete diff and affected source and tests. Distinguish current, mocked, planned, and prohibited behavior.

Report evidence-backed findings in severity order with exact paths, concrete impact, and the smallest correction. Cover trust boundaries, ownership, coupling, cohesion, portability, dependency direction, failure containment, performance, and architecture drift. State whether the work remains within its goal and non-goals.

Do not modify files, approve work, or promote planned behavior to current capability.
```
