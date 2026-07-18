# Remediation by Severity

- **Category:** Increment
- **Purpose:** Revalidate and remediate a bounded group of current findings at one selected severity.
- **Use when:** An authoritative backlog contains multiple findings at the same severity that may share a root cause.
- **Do not use when:** One advisory is selected, the backlog is unaudited, or the findings span unrelated systems that cannot be reviewed safely together.
- **Required inputs:** `{{SEVERITY}}`, `{{BACKLOG_PATH}}`, `{{INCREMENT_NAME}}`, `{{BRANCH_NAME}}`, and `{{REPORT_DATE}}`.
- **Expected outputs:** A pre-edit remediation plan, approved bounded fixes and regression tests, and evidence-backed backlog dispositions.
- **Related skills:** `$technical-debt`, `$readiness-review`, `$verified-increment`, `$security-review`.
- **Related prompts:** [Single-advisory remediation](remediation-single-advisory.md), [Remediation workflow](../workflows/remediation.md), [Remediation template](../templates/remediation-template.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $verified-increment.

Plan and implement {{INCREMENT_NAME}} for {{SEVERITY}} findings in {{BACKLOG_PATH}}. Use {{BRANCH_NAME}} only after approval. Record the review date as {{REPORT_DATE}}.

Before editing, read AGENTS.md, the backlog, every source report for the selected findings, and the applicable engineering, architecture, security, testing, roadmap, and project-memory documents. Confirm Git is clean and run the smallest relevant baseline checks.

Revalidate every finding currently labeled {{SEVERITY}} against source and tests. Exclude findings that are resolved, superseded, duplicate, or no longer relevant. Identify prerequisite findings at other severities, group valid findings by root cause and affected subsystem, and assess whether each group is independently reviewable.

Do not automatically remediate findings at other severities. Include a lower-severity prerequisite only when it is required to resolve an in-scope finding and identify it before editing. Stop and propose sub-increments when work crosses unrelated subsystems, requires an architecture decision, or becomes too large for safe review.

Present the exact remediation plan, files, risks, non-goals, tests, verification, backlog updates, and rollback. Wait for project-owner approval before creating the branch, beginning the gate, or editing.

After approval, implement only the accepted group. Add focused regression tests and preserve unrelated behavior and security boundaries. Do not add unrelated feature work. Do not mark a finding resolved without source and verification evidence.

At closeout, update {{BACKLOG_PATH}} with resolved, skipped, deferred, superseded, duplicate, and remaining findings; record exact evidence and keep the resolving commit pending until committed. Run complete required verification and the post-increment gate. Do not start another remediation, commit, push, or merge automatically.
```
