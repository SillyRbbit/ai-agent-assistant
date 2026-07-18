# Remediation Prompt Template

- **Category:** Template
- **Purpose:** Provide the standard skeleton for a backlog-driven remediation prompt.
- **Use when:** A new remediation selection mode is justified and existing single-advisory or severity prompts cannot express it.
- **Do not use when:** The work is an ordinary bug fix, feature, refactor, or duplicate remediation flow.
- **Required inputs:** `{{TITLE}}`, `{{SELECTION}}`, `{{BACKLOG_PATH}}`, `{{SOURCE_EVIDENCE}}`, `{{SCOPE_CONTROLS}}`, `{{RELATED_SKILLS}}`, and `{{RELATED_PROMPTS}}`.
- **Expected outputs:** One metadata-complete remediation prompt with revalidation, approval, evidence, and backlog-update boundaries.
- **Related skills:** `$technical-debt`, `$verified-increment`.
- **Related prompts:** [Single-advisory remediation](../increments/remediation-single-advisory.md), [Remediation by severity](../increments/remediation-by-severity.md), [Remediation workflow](../workflows/remediation.md).
- **Last reviewed:** 2026-07-18

## Template

```markdown
# {{TITLE}}

- **Category:** Increment
- **Purpose:** Remediate {{SELECTION}} from {{BACKLOG_PATH}}.
- **Use when:** {{SOURCE_EVIDENCE}}
- **Do not use when:** The findings cannot be bounded independently.
- **Required inputs:** {{BACKLOG_PATH}}, source reports, exact selection, branch, and verification plan.
- **Expected outputs:** Revalidated findings, approved fixes, regression evidence, and backlog dispositions.
- **Related skills:** {{RELATED_SKILLS}}
- **Related prompts:** {{RELATED_PROMPTS}}
- **Last reviewed:** YYYY-MM-DD

## Prompt

\`\`\`text
Revalidate the selected findings against current source and tests. {{SCOPE_CONTROLS}}

Present exact files, risks, non-goals, tests, verification, and rollback. Wait for approval before editing. Follow the Risk-Based Validation Policy in AGENTS.md and ENGINEERING_GUIDE.md: use focused checks during implementation, then run the complete required completion-gate verification for the selected tier once after the final relevant edit. Never reduce cross-cutting, security-sensitive, dependency, Tauri-configuration, or release validation. Do not mark findings resolved without source and verification evidence or start another remediation automatically.
\`\`\`
```
