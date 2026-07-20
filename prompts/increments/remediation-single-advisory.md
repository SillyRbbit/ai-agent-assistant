# Single-Advisory Remediation

- **Category:** Increment
- **Purpose:** Resolve one revalidated advisory and only its directly required root-cause work.
- **Use when:** One advisory has an authoritative backlog entry and source report.
- **Do not use when:** Several advisories are intentionally grouped by severity or the selected advisory cannot be bounded independently.
- **Required inputs:** `{{ADVISORY_ID}}`, `{{SHORT_NAME}}`, `{{BACKLOG_PATH}}`, `{{SOURCE_REPORT}}`, and `{{BRANCH_NAME}}`.
- **Expected outputs:** One approved remediation with focused regression evidence and an updated advisory disposition.
- **Related skills:** `$verified-increment`, `$technical-debt`, `$security-review`, `$quality-gate`.
- **Related prompts:** [Remediation by severity](remediation-by-severity.md), [Remediation workflow](../workflows/remediation.md), [Remediation template](../templates/remediation-template.md).
- **Last reviewed:** 2026-07-18

## Prompt

```text
Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md. Apply the current task-specific instructions below without violating the approved increment.

Use $verified-increment.

Resolve only {{ADVISORY_ID}} ({{SHORT_NAME}}) from {{BACKLOG_PATH}} using its original evidence in {{SOURCE_REPORT}}. Use {{BRANCH_NAME}} only after approval.

Before editing, read AGENTS.md, the backlog entry, source report, current project memory, and applicable architecture, security, testing, increment, and decision documents. Confirm a clean Git baseline, revalidate that the advisory still applies, identify dependent behavior, and run the smallest useful baseline checks.

State the exact root cause, remediation approach, files, tests, verification, regression risks, non-goals, and rollback. Identify any prerequisite finding before editing. Wait for project-owner approval before creating the branch, beginning the gate, or changing files.

After approval, implement only {{ADVISORY_ID}} and directly required root-cause work. Preserve verified behavior and trust boundaries, use typed errors where production behavior changes, and add focused regression tests. Stop before unrelated cleanup, features, or advisories.

Mark {{ADVISORY_ID}} resolved only with source and verification evidence. Record its resolving commit as pending until committed, preserve skipped or remaining advisories, and create the required remediation increment record. Follow the Risk-Based Validation Policy in MASTER_PROMPT.md and ENGINEERING_GUIDE.md; task-specific validation may be stricter. Run the post-increment gate and stop. Do not commit, push, merge, or start another remediation automatically.
```
