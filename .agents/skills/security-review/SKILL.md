---
name: security-review
description: Review AI Agent Assistant changes that affect tools, IPC, permissions, approvals, storage, model data, platform adapters, or other trust boundaries.
---

# Security review

1. Read `SECURITY.md`, `CODE_REVIEW.md`, product architecture, and affected decisions.
2. Map inputs, outputs, data stores, actors, and trust boundaries.
3. Verify the model and WebView gain no authorization or unrestricted executor path.
4. Check schema validation, risk classification, approval binding, permission scope, cancellation, retries, and audit redaction.
5. Check credential and personal-data handling.
6. Look for unsafe default behavior, hidden privilege expansion, and fail-open paths.
7. Report evidence-backed findings in severity order with exact locations and fixes.
8. Identify missing regression or adversarial tests.
9. Require a documented decision for any new permission, production dependency, or trust-boundary change.

Do not implement fixes during a review unless explicitly requested.
