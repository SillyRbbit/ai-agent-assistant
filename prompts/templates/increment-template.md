# Increment Prompt Template

- **Category:** Template
- **Purpose:** Provide the standard skeleton for a new bounded implementation prompt.
- **Use when:** No existing increment prompt covers a genuinely distinct implementation mode.
- **Do not use when:** An existing prompt can be reused with concrete placeholders or a cross-reference.
- **Required inputs:** `{{TITLE}}`, `{{PURPOSE}}`, `{{USE_WHEN}}`, `{{DO_NOT_USE_WHEN}}`, `{{REQUIRED_INPUTS}}`, `{{EXPECTED_OUTPUTS}}`, `{{RELATED_SKILLS}}`, `{{RELATED_PROMPTS}}`, and `{{PROMPT_BODY}}`.
- **Expected outputs:** One metadata-complete increment prompt with explicit approval and publication boundaries.
- **Related skills:** `$verified-increment`.
- **Related prompts:** [Verified increment](../increments/verified-increment.md), [Feature implementation](../increments/feature-implementation.md), [Remediation template](remediation-template.md).
- **Last reviewed:** 2026-07-18

## Template

```markdown
# {{TITLE}}

- **Category:** Increment
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
Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md. Apply the current task-specific instructions below without violating the approved increment.

{{PROMPT_BODY}}

Read the authoritative repository documents, inspect the baseline, state exact scope and verification, and wait for approval before editing. Follow the Risk-Based Validation Policy in MASTER_PROMPT.md and ENGINEERING_GUIDE.md. Do not commit, push, merge, publish, or begin another increment automatically.
\`\`\`
```
