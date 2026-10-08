---
name: readiness-review
description: Determine whether a proposed Cortexa increment is Ready, Ready with advisories, or Blocked using repository evidence. Use when selecting the next bounded increment or validating a plan before implementation.
---

# Readiness review

1. Read `NEXT_STEPS.md`, `ROADMAP.md`, `PROJECT_STATUS.md`, `ARCHITECTURE.md`, accepted decisions, and the candidate plan.
2. Confirm prerequisites against Git, source, tests, and valid completion evidence.
3. Require an objective, exclusions, acceptance checklist, expected paths, risks and verification. Assess each unresolved predecessor criterion for dependency and actual defect impact.
4. Identify unresolved decisions, dependencies, permissions, credentials, target-platform checks, or ownership gaps.
5. Use `docs/templates/READINESS_REVIEW_TEMPLATE.md` when recording a standalone review.
6. Return exactly `Ready`, `Ready with advisories`, or `Blocked`, followed by evidence and the smallest next action.

Do not reorder `NEXT_STEPS.md`, approve the plan, begin a gate, or implement work during the review.

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
