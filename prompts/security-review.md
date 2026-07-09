# Security review

```text
Perform a focused security review of [CHANGE OR COMPONENT].

Read AGENTS.md, SECURITY.md, CODE_REVIEW.md, docs/product/PRODUCT_BRIEF.md, docs/product/ARCHITECTURE_BASELINE.md, DECISIONS.md, and the affected code and tests.

Map data flow and trust boundaries. Check whether the WebView, model output, untrusted content, or network service gains any direct execution or authorization capability. Review schema validation, permission scope, risk classification, approval binding, logging and redaction, cancellation, retries, storage, and secret handling.

Report only evidence-backed findings in severity order with exact locations and mitigations. Identify missing tests and any architecture decision required. Do not expand product permissions or implement a fix unless I request it after the review.
```
