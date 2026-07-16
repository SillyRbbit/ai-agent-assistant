---
name: security-review
description: Review Cortexa changes that affect tools, hooks, IPC, permissions, approvals, storage, model data, platform adapters, credentials, or other trust boundaries. Use before accepting a security-sensitive increment or release.
---

# Security review

1. Read `SECURITY.md`, `SECURITY_CHECKLIST.md`, `CODE_REVIEW.md`, `ARCHITECTURE.md`, affected decisions, and the active plan.
2. Map inputs, outputs, data stores, actors, and trust boundaries.
3. Verify the model and WebView gain no authorization or unrestricted executor path.
4. Check schema and path validation, risk classification, approval binding, permission scope, cancellation, retries, audit redaction, and error closure.
5. Check credential, personal-data, log, hook, network, filesystem, SQLite, capability, and supply-chain handling.
6. Look for unsafe defaults, hidden privilege expansion, unbounded input, symlink escape, command injection, and fail-open paths.
7. Record evidence in `docs/templates/SECURITY_REVIEW_TEMPLATE.md` when a standalone artifact is required.
8. Report findings in severity order with exact locations, impact, evidence, and the smallest fix.
9. Identify missing regression or adversarial tests and state the final review result.
10. Require a documented decision for any new permission, production dependency, credential, or trust-boundary change.

Do not implement fixes during a review unless explicitly requested.
