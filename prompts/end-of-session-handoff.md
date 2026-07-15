# End-of-session handoff

```text
Close this Cortexa working session using docs/workflows/END_SESSION.md.

Inspect all changes and run the appropriate final verification commands. Review the diff against AGENTS.md, SECURITY.md, CODE_REVIEW.md, and the active increment acceptance criteria.

Run $post-increment-gate before ending. Create the required consolidated report, classify every automated and manual check, and finalize the deterministic marker only for PASS or PASS WITH ADVISORIES. Do not automatically fix advisory findings.

Update HANDOFF.md with the phase, increment, branch and working-tree state, completed work, working/partial/broken behavior, exact next task, exact copy-paste resume prompt, files changed, commands run, test results, warnings, and open questions. Update PROJECT_STATUS.md, NEXT_STEPS.md, DECISIONS.md, CHANGELOG.md, TROUBLESHOOTING_LOG.md, PLANS.md, and the relevant increment document when applicable.

Do not claim a command passed unless it actually ran. Clearly separate passed, failed, and not-run checks. Confirm no secret, personal data, build artifact, or local log was added. End with the final Git status and the next recommended prompt.
```
