# Code Review

- **Category:** Review
- **Purpose:** Review a complete change set for concrete correctness, security, privacy, tests, portability, and documentation defects.
- **Use when:** A diff or commit needs findings before acceptance.
- **Do not use when:** The user has asked to implement fixes immediately without a separate review result.
- **Required inputs:** `{{CHANGE_SET}}`, active plan, affected source, and tests.
- **Expected outputs:** Findings first in severity order, verification gaps, and an acceptance-criteria assessment.
- **Related skills:** `$code-review`, `$security-review`, `$architecture-review`.
- **Related prompts:** [Architecture review](architecture-review.md), [Security review](security-review.md), [Quality gate](quality-gate.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $code-review to review {{CHANGE_SET}} without rewriting it.

Read AGENTS.md, SECURITY.md, CODE_REVIEW.md, the active increment or plan, and the relevant source and tests. Inspect Git status and the complete diff.

Report findings in severity order. For each real finding, provide the file and line, concrete impact, evidence or reproduction path, and the smallest recommended fix. Focus on correctness, authorization boundaries, permissions, personal-data handling, error and cancellation behavior, test gaps, portability, and documentation drift. Avoid speculative style comments.

Then list verification commands that ran, commands still required, and whether acceptance criteria are satisfied. Do not modify files unless separately requested.
```
