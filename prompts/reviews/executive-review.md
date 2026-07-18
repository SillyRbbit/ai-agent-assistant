# Executive Review

- **Category:** Review
- **Purpose:** Translate verified repository evidence into a concise leadership assessment.
- **Use when:** Executives need current value, controls, risk, readiness, and decisions without implementation detail.
- **Do not use when:** Evidence is unavailable or the request is to create unsupported marketing or compliance claims.
- **Required inputs:** `{{MILESTONE_OR_CHANGE}}` and its verified source or review evidence.
- **Expected outputs:** A plain-language assessment separating implemented, mocked, planned, blocked, and prohibited capability.
- **Related skills:** `$executive-review`.
- **Related prompts:** [Readiness review](readiness-review.md), [Release review](release-review.md), [Architecture review](architecture-review.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $executive-review for {{MILESTONE_OR_CHANGE}}.

Base the assessment only on PRODUCT_REQUIREMENTS.md, ROADMAP.md, PROJECT_STATUS.md, ARCHITECTURE.md, and verified review evidence. Separate implemented, mocked, planned, blocked, and prohibited capability.

Summarize purpose, operational value, controls preserved, verification, residual risk, dependencies, decision required, and the exact next milestone in concise leadership language. Do not suppress material risk, invent evidence, approve work, or change files.
```
