# End a working session

## Goal

Leave the repository in a state that another assistant session can resume without access to the current conversation.

## Procedure

1. Stop active development processes with `Control-C` where appropriate.
2. Inspect all changes:

   ```bash
   git status --short --branch
   git diff --stat
   git diff
   ```

3. Run the relevant final checks. For a completed implementation increment:

   ```bash
   npm run format:check
   npm run lint
   npm run typecheck
   npm run test
   npm run build
   npm run tauri -- build --no-bundle
   ```

4. Record each command as passed, failed, or not run. Include why any required command was not run.
5. Review changes against `CODE_REVIEW.md` and `SECURITY.md`.
6. Update:
   - `HANDOFF.md`
   - `PROJECT_STATUS.md`
   - `NEXT_STEPS.md`
   - `DECISIONS.md` when a durable decision changed
   - `CHANGELOG.md`
   - `TROUBLESHOOTING_LOG.md` for a resolved or active failure
   - The applicable file under `docs/increments/`
   - Any active plan under `docs/plans/`
7. Put an exact copy-paste resume prompt in `HANDOFF.md`.
8. Confirm no secrets, build output, personal data, or local logs were added.
9. Show the final Git status.

## Required handoff content

- Phase and increment.
- Last completed task.
- Branch and working-tree state.
- What works.
- What is partial.
- What is broken or unknown.
- Next recommended task.
- Exact resume prompt.
- Files changed.
- Commands run and results.
- Warnings and open questions.

## Exit condition

A new session can begin by reading the repository alone and can reproduce the last known result.
