---
name: verified-increment
description: Implement one small, bounded Cortexa increment with explicit non-goals, typed errors, focused tests, security review, and complete handoff updates.
---

# Verified increment

1. Select one ready item from `NEXT_STEPS.md` or one explicitly requested bounded task.
2. Read the relevant product, security, decision, and increment documents.
3. State goal, non-goals, expected files, risks, and verification commands.
4. After approval and before editing, run `python3 .codex/hooks/post_increment_gate.py begin --increment <increment>` when the repository gate exists.
5. Preserve current behavior outside scope.
6. Implement the smallest coherent change.
7. Add or update focused tests for success and failure behavior.
8. Use strict TypeScript and typed Rust errors; no panic-style production shortcuts.
9. Run targeted checks, then the full relevant verification.
10. Review the diff using `CODE_REVIEW.md` and `SECURITY.md`.
11. Update handoff, status, next steps, changelog, decisions, troubleshooting, and increment documentation as applicable.
12. Run `$post-increment-gate`; do not mark the increment complete without a valid passing report and completion marker.

Stop broad implementation if the baseline fails or the required architecture decision is unresolved.
