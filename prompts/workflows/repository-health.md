# Repository Health Workflow

- **Category:** Workflow
- **Purpose:** Assess and improve repository governance, CI, documentation, dependency hygiene, and contribution ergonomics without changing product behavior.
- **Use when:** Repository policy, GitHub configuration, documentation links, or maintenance checks need bounded review.
- **Do not use when:** The requested outcome is a product feature, runtime fix, deployment, or automatic dependency merge.
- **Required inputs:** `{{HEALTH_SCOPE}}`, current Git state, repository checks, and applicable governance documents.
- **Expected outputs:** Evidence-backed findings, one bounded approved correction if requested, and verified repository-only closeout.
- **Related skills:** `$readiness-review`, `$security-review`, `$verified-increment`.
- **Related prompts:** [Documentation workflow](documentation.md), [Code review](../reviews/code-review.md), [Verified increment](../increments/verified-increment.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Review repository health for {{HEALTH_SCOPE}}.

Read AGENTS.md, ENGINEERING_GUIDE.md, CONTRIBUTING.md, CODE_REVIEW.md, SECURITY.md, TESTING_GUIDE.md, and applicable GitHub and workflow documentation. Inspect Git status, CI definitions, action pins, permissions, templates, ownership, dependency automation, documentation links, ignored/generated files, licensing evidence, and available repository checks.

Report evidence-backed findings by severity. Distinguish local repository evidence from remote GitHub settings that were not authenticated. Do not expose secrets, enable automatic commits or merges, add deployment, or change product source.

If correction is requested, propose one bounded repository-governance increment with exact files, risks, non-goals, verification, and rollback. Wait for approval before editing. Run repository policy, documentation, security, and complete required verification before closeout.
```
