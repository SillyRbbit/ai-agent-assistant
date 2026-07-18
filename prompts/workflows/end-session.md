# End a Session

- **Category:** Workflow
- **Purpose:** Close one Cortexa session with verified evidence, synchronized memory, and a valid marker when completion is allowed.
- **Use when:** Approved implementation or documentation work is ready for final review and handoff.
- **Do not use when:** Required implementation, tests, or mandatory manual checks are intentionally incomplete without a recorded blocker.
- **Required inputs:** `{{INCREMENT_NAME}}`, active plan, complete diff, and actual automated/manual results.
- **Expected outputs:** Classified checks, consolidated review, synchronized project memory, exact next prompt, and valid marker or explicit failure state.
- **Related skills:** `$session-end`, `$quality-gate`, `$post-increment-gate`, `$documentation-sync`.
- **Related prompts:** [Quality gate](../reviews/quality-gate.md), [Documentation workflow](documentation.md), [Start session](start-session.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $session-end for {{INCREMENT_NAME}} and follow docs/workflows/END_SESSION.md.

Stop active development processes where appropriate. Inspect branch, staged, unstaged, untracked, conflicted, and complete diff state. Run every required automated and manual check and classify each as Passed, Failed, Not run, or Manual verification pending. Do not infer a result.

Run python3 .codex/hooks/session_end_gate.py. Apply $quality-gate across architecture, security, code health, technical debt, and readiness. Do not silently fix advisory findings or modify product source after verification.

Synchronize HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, CHANGELOG.md, PLANS.md, applicable decisions and troubleshooting history, and the active plan/increment only from collected evidence. Preserve dated history and include an exact copy-paste next prompt.

Use $post-increment-gate. Create the dated consolidated report from docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md with the exact live changed-file inventory and commands. Return exactly PASS, PASS WITH ADVISORIES, or FAIL. Finalize only a non-blocking result and require the expected increment to report complete with valid true.

Confirm no secret, personal data, generated output, database, build artifact, or unrelated file was added. Do not commit, push, merge, publish, release, or start another increment without separate approval.
```
