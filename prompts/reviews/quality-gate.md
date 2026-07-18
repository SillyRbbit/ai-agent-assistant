# Quality Gate

- **Category:** Review
- **Purpose:** Compose verification, architecture, security, code-health, debt, and readiness evidence for one increment.
- **Use when:** Approved implementation is complete and needs an evidence-based acceptance decision before closeout.
- **Do not use when:** Required implementation or manual verification is still intentionally in progress.
- **Required inputs:** `{{INCREMENT_NAME}}`, its plan, complete diff, and actual command results.
- **Expected outputs:** Exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL` with classified evidence.
- **Related skills:** `$quality-gate`, `$architecture-review`, `$security-review`, `$code-review`, `$technical-debt`, `$readiness-review`.
- **Related prompts:** [End session](../workflows/end-session.md), [Architecture review](architecture-review.md), [Security review](security-review.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $quality-gate for {{INCREMENT_NAME}}.

Run python3 .codex/hooks/session_end_gate.py, then execute every automated and manual check required by the plan. Record Passed, Failed, Not run, and Manual verification pending accurately. Review the complete change set with $architecture-review, $security-review, $code-review, $technical-debt, and $readiness-review.

Return exactly PASS, PASS WITH ADVISORIES, or FAIL using repository blocking rules. Do not silently fix advisories, update project memory before evidence exists, commit, push, merge, publish, or start another increment. Pass accepted evidence to $post-increment-gate for closeout.
```
