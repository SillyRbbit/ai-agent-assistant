# Readiness Review

- **Category:** Review
- **Purpose:** Determine whether one candidate increment is sufficiently bounded to request implementation approval.
- **Use when:** Selecting or validating a next increment before beginning its gate.
- **Do not use when:** Implementation has already started or the request is to approve work automatically.
- **Required inputs:** `{{CANDIDATE_INCREMENT}}` and its proposed plan.
- **Expected outputs:** Exactly `Ready`, `Ready with advisories`, or `Blocked`, with evidence and the smallest next action.
- **Related skills:** `$readiness-review`.
- **Related prompts:** [Verified increment](../increments/verified-increment.md), [Architecture review](architecture-review.md), [Review template](../templates/review-template.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $readiness-review.

Review {{CANDIDATE_INCREMENT}} against NEXT_STEPS.md, ROADMAP.md, PROJECT_STATUS.md, ARCHITECTURE.md, accepted decisions, Git evidence, prerequisites, and its plan. Require one bounded goal, exact files, risks, non-goals, verification, manual gates, and rollback.

Return exactly Ready, Ready with advisories, or Blocked, followed by the evidence and smallest next action. Use docs/templates/READINESS_REVIEW_TEMPLATE.md if a persistent artifact is requested. Do not approve, reorder the queue, begin a gate, or implement work.
```
