---
name: architecture-review
description: Review Cortexa changes for current-state accuracy, trust-boundary integrity, module ownership, coupling, portability, and architecture drift. Use for implementation diffs, plans, or readiness decisions that may affect system structure.
---

# Architecture review

1. Read `ARCHITECTURE.md`, `ENGINEERING_GUIDE.md`, accepted decisions, and the active plan.
2. Inspect the complete diff and the source and tests on both sides of each changed boundary.
3. Distinguish current, mocked, planned, and prohibited behavior.
4. Review trust boundaries, ownership, coupling, cohesion, portability, dependencies, performance, and failure containment.
5. Identify over-engineering, under-engineering, duplicated authority, and misleading abstractions.
6. Report evidence-backed findings by severity with exact paths, impact, and the smallest correction.
7. State whether the architecture remains consistent with the increment goal and non-goals.

Do not change files or promote planned behavior to current capability during the review.
