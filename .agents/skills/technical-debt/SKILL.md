---
name: technical-debt
description: Identify and classify concrete Cortexa technical debt with risk, effort, milestone, and blocking impact. Use during post-increment review, roadmap planning, or focused maintainability assessment.
---

# Technical debt review

1. Read the active plan, complete diff, `ARCHITECTURE.md`, `CODE_REVIEW.md`, and current roadmap state.
2. Inspect duplication, dead code, misleading abstractions, brittle tests, deferred failures, portability gaps, dependency health, and documentation drift.
3. Separate existing debt from debt introduced by the increment.
4. For each finding record category, severity, summary, concrete risk, effort, recommended milestone, and whether it blocks completion or the next increment.
5. Use only `Critical`, `High`, `Medium`, `Low`, or `Advisory` severity.
6. Report `None` when no evidence-backed debt finding exists.

Do not silently fix findings, broaden the increment, or convert preferences into debt.
