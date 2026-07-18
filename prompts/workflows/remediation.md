# Remediation Workflow

- **Category:** Workflow
- **Purpose:** Select, validate, sequence, and close evidence-backed remediation work without combining unrelated findings.
- **Use when:** An authoritative audit or advisory backlog needs an ordered remediation path.
- **Do not use when:** No current backlog exists or the request is ordinary feature work.
- **Required inputs:** `{{BACKLOG_PATH}}`, `{{SELECTION_MODE}}` as `single advisory` or `severity`, and the selected identifier or severity.
- **Expected outputs:** A validated selection, prerequisite analysis, chosen increment prompt, approval pause, and evidence-backed backlog closeout.
- **Related skills:** `$technical-debt`, `$readiness-review`, `$verified-increment`.
- **Related prompts:** [Single-advisory remediation](../increments/remediation-single-advisory.md), [Remediation by severity](../increments/remediation-by-severity.md), [Remediation template](../templates/remediation-template.md).
- **Last reviewed:** 2026-07-18

## Prompt

```text
Coordinate remediation from {{BACKLOG_PATH}} using {{SELECTION_MODE}}.

Read AGENTS.md, the complete backlog, source reports, current source/tests, project memory, and applicable architecture, security, testing, roadmap, and decision documents. Revalidate candidate findings before treating them as work.

Classify each candidate as still valid, resolved, superseded, duplicate, or no longer relevant. Identify prerequisites and root-cause groupings. Do not combine unrelated subsystems or automatically include another severity.

For a single advisory, prepare prompts/increments/remediation-single-advisory.md. For one severity, prepare prompts/increments/remediation-by-severity.md. Replace every placeholder and present the bounded increment, exact files, risks, non-goals, verification, and rollback. Wait for project-owner approval before implementation.

After an approved increment, require focused regression evidence, backlog dispositions, project-memory synchronization, and the post-increment gate. Follow the Risk-Based Validation Policy in AGENTS.md and ENGINEERING_GUIDE.md: use focused checks during implementation, then run the complete required completion-gate verification for the selected tier once after the final relevant edit. Never reduce security-sensitive, dependency, Tauri-configuration, or release validation. Stop before selecting or starting the next remediation automatically.
```
