---
name: quality-gate
description: Run Cortexa's evidence-based implementation quality review across verification, architecture, security, code health, technical debt, and readiness. Use after bounded implementation and before documentation closeout or publication.
---

# Quality gate

1. Read the active plan and its required automated and manual checks.
2. Run `python3 .codex/hooks/session_end_gate.py` and refuse acceptance when conflicts exist.
3. Run every increment-required verification command and classify it as `Passed`, `Failed`, `Not run`, or `Manual verification pending`.
4. Apply `$architecture-review`, `$security-review`, `$code-review`, `$technical-debt`, and `$readiness-review` to the complete change set.
5. Do not fix advisory findings automatically. Record them with owners and follow-up milestones.
6. Return exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL` using the repository blocking rules.
7. Pass the evidence to `$post-increment-gate` for documentation synchronization, the consolidated report, and marker finalization.

This skill does not commit, push, merge, publish, modify product source after verification, or start another increment.
