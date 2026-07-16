# Security review

```text
Perform a focused security review of [CHANGE OR COMPONENT].

Read AGENTS.md, SECURITY.md, SECURITY_CHECKLIST.md, CODE_REVIEW.md, ARCHITECTURE.md, DECISIONS.md, the active plan, and the affected code and tests.

Map data flow and trust boundaries. Check whether the WebView, model output, untrusted content, hook input, or network service gains direct execution or authorization capability. Review schema and path validation, permission scope, risk classification, approval binding, logging and redaction, cancellation, retries, storage, credentials, filesystem access, commands, and supply-chain impact.

Report only evidence-backed findings in severity order with exact locations, impact, evidence, and mitigations. Identify missing adversarial tests and any architecture decision required. Use docs/templates/SECURITY_REVIEW_TEMPLATE.md when a persistent artifact is requested. Do not expand permissions or implement a fix unless I request it after the review.
```
