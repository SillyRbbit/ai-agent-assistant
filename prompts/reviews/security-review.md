# Security Review

- **Category:** Review
- **Purpose:** Assess one change or component against Cortexa trust, data, permission, credential, and execution boundaries.
- **Use when:** Work affects hooks, IPC, tools, policy, approvals, storage, network, platform adapters, credentials, or permissions.
- **Do not use when:** The request is a general style review or asks to expand permission scope during assessment.
- **Required inputs:** `{{CHANGE_OR_COMPONENT}}`, active plan, complete diff, affected source, and tests.
- **Expected outputs:** Evidence-backed findings in severity order, missing adversarial tests, and required decisions.
- **Related skills:** `$security-review`.
- **Related prompts:** [Architecture review](architecture-review.md), [Code review](code-review.md), [Release review](release-review.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $security-review for {{CHANGE_OR_COMPONENT}}.

Read AGENTS.md, SECURITY.md, SECURITY_CHECKLIST.md, CODE_REVIEW.md, ARCHITECTURE.md, DECISIONS.md, the active plan, and affected code and tests.

Map data flow and trust boundaries. Check whether the WebView, model output, untrusted content, hook input, or network service gains direct execution or authorization capability. Review schema and path validation, permission scope, risk classification, approval binding, logging and redaction, cancellation, retries, storage, credentials, filesystem access, commands, and supply-chain impact.

Report only evidence-backed findings in severity order with exact locations, impact, evidence, and mitigations. Identify missing adversarial tests and any architecture decision required. Use docs/templates/SECURITY_REVIEW_TEMPLATE.md when a persistent artifact is requested. Do not expand permissions or implement fixes during the review.
```
