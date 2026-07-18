# Resume a working session

## Goal

Continue safely from repository state without relying on chat history.

Use `.agents/skills/resume-session/SKILL.md` when repository skills are
available. Otherwise use `prompts/workflows/start-session.md` with
`{{SESSION_MODE}}` set to `resume`.

## Procedure

1. Read `HANDOFF.md` first, then follow the full reading order in `AGENTS.md`.
2. Compare the handoff to the actual working tree:

   ```bash
   git status --short --branch
   git diff --stat
   git log -5 --oneline
   ```

3. Verify the exact branch and uncommitted files. Do not assume the branch value written in a prior handoff is still correct.
4. Confirm the current toolchain and the last failed or passed verification command.
5. Re-run the smallest command that proves the handoff baseline still holds.
6. Restate the exact next task and acceptance criteria from `NEXT_STEPS.md`.
7. Continue existing uncommitted work before starting an unrelated increment.
8. If files have changed since the handoff, inspect the diff and update the plan before editing.

## Conflict handling

When documentation and code disagree:

- Treat code and test output as evidence of current behavior.
- Treat product and security guardrails as authority for intended behavior.
- Surface the mismatch.
- Correct the stale project-memory file in the same session.

## Exit condition

The assistant can name what is complete, what remains, why the next edit is safe, and how it will be verified.
