---
name: readiness-review
description: Determine whether a proposed Cortexa increment is Ready, Ready with advisories, or Blocked using repository evidence. Use when selecting the next bounded increment or validating a plan before implementation.
---

# Readiness review

1. Read `NEXT_STEPS.md`, `ROADMAP.md`, `PROJECT_STATUS.md`, `ARCHITECTURE.md`, accepted decisions, and the candidate plan.
2. Confirm prerequisites against Git, source, tests, and valid completion evidence.
3. Require one bounded goal, exact files, risks, non-goals, verification, manual gates, and rollback.
4. Identify unresolved decisions, dependencies, permissions, credentials, target-platform checks, or ownership gaps.
5. Use `docs/templates/READINESS_REVIEW_TEMPLATE.md` when recording a standalone review.
6. Return exactly `Ready`, `Ready with advisories`, or `Blocked`, followed by evidence and the smallest next action.

Do not reorder `NEXT_STEPS.md`, approve the plan, begin a gate, or implement work during the review.
