---
name: quality-gate
description: Run Cortexa's evidence-based implementation quality review across verification, architecture, security, code health, technical debt, and readiness. Use after bounded implementation and before documentation closeout or publication.
---

# Quality gate

1. Read the active plan and its required automated and manual checks.
2. Run `python3 .codex/hooks/session_end_gate.py` and refuse acceptance when conflicts exist.
3. Run affected required verification and reuse valid unchanged-input evidence with provenance. Classify each result as `Passed`, `Failed`, `Not run`, or `Manual verification pending`; label inherited evidence explicitly.
4. Apply `$architecture-review`, `$security-review`, `$code-review`, `$technical-debt`, and `$readiness-review` to the complete change set.
5. Resolve findings within the authorized objective and rerun affected checks; record unrelated advisories with owners and follow-up milestones.
6. Return exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL` using the repository blocking rules.
7. Pass the evidence to `$post-increment-gate` for documentation synchronization, the consolidated report, and marker finalization.

This skill does not commit, push, merge, publish, modify product source after verification, or start another increment.

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
