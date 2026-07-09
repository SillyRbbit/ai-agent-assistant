---
name: verified-increment
description: Implement one small, bounded AI Agent Assistant increment with explicit non-goals, typed errors, focused tests, security review, and complete handoff updates.
---

# Verified increment

1. Select one ready item from `NEXT_STEPS.md` or one explicitly requested bounded task.
2. Read the relevant product, security, decision, and increment documents.
3. State goal, non-goals, expected files, risks, and verification commands.
4. Preserve current behavior outside scope.
5. Implement the smallest coherent change.
6. Add or update focused tests for success and failure behavior.
7. Use strict TypeScript and typed Rust errors; no panic-style production shortcuts.
8. Run targeted checks, then the full relevant verification.
9. Review the diff using `CODE_REVIEW.md` and `SECURITY.md`.
10. Update handoff, status, next steps, changelog, decisions, troubleshooting, and increment documentation as applicable.

Stop broad implementation if the baseline fails or the required architecture decision is unresolved.
