# Quality gate

```text
Use $quality-gate for the active increment.

Run python3 .codex/hooks/session_end_gate.py, then execute every automated and manual check required by the plan. Record Passed, Failed, Not run, and Manual verification pending accurately. Review the complete change set with $architecture-review, $security-review, $code-review, $technical-debt, and $readiness-review.

Return exactly PASS, PASS WITH ADVISORIES, or FAIL using repository blocking rules. Do not silently fix advisories, update project memory before evidence exists, commit, push, merge, publish, or start another increment. Pass accepted evidence to $post-increment-gate for closeout.
```
