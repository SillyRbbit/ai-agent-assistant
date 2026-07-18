# Review Prompt Template

- **Category:** Template
- **Purpose:** Provide the standard skeleton for a non-mutating evidence review prompt.
- **Use when:** A genuinely distinct review cannot use an existing review prompt.
- **Do not use when:** The prompt would silently implement fixes or duplicate architecture, security, code, debt, readiness, executive, quality, or release review.
- **Required inputs:** `{{TITLE}}`, `{{PURPOSE}}`, `{{USE_WHEN}}`, `{{DO_NOT_USE_WHEN}}`, `{{REQUIRED_INPUTS}}`, `{{EXPECTED_OUTPUTS}}`, `{{RELATED_SKILLS}}`, `{{RELATED_PROMPTS}}`, and `{{REVIEW_BODY}}`.
- **Expected outputs:** One metadata-complete review prompt with evidence and no automatic edits.
- **Related skills:** `$code-review`.
- **Related prompts:** [Code review](../reviews/code-review.md), [Quality gate](../reviews/quality-gate.md), [Increment template](increment-template.md).
- **Last reviewed:** 2026-07-17

## Template

```markdown
# {{TITLE}}

- **Category:** Review
- **Purpose:** {{PURPOSE}}
- **Use when:** {{USE_WHEN}}
- **Do not use when:** {{DO_NOT_USE_WHEN}}
- **Required inputs:** {{REQUIRED_INPUTS}}
- **Expected outputs:** {{EXPECTED_OUTPUTS}}
- **Related skills:** {{RELATED_SKILLS}}
- **Related prompts:** {{RELATED_PROMPTS}}
- **Last reviewed:** YYYY-MM-DD

## Prompt

\`\`\`text
{{REVIEW_BODY}}

Report evidence-backed findings first in severity order. Do not modify files, approve work, or invent verification evidence.
\`\`\`
```
