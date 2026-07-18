# End a working session

## Goal

Leave the repository in a state that another assistant session can resume without access to the current conversation.

Use `.agents/skills/session-end/SKILL.md` when repository skills are available.
Otherwise use `prompts/workflows/end-session.md`, which incorporates the
post-increment quality and marker closeout.

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

4. Record each command as passed, failed, not run, or manual verification pending. Include why any required command was not run.
5. Run the deterministic repository inventory:

   ```bash
   python3 .codex/hooks/session_end_gate.py
   ```

   Resolve every conflict and investigate any unexpected staged, unstaged, or
   untracked path.

6. Run `$quality-gate` to compose architecture, security, code-health,
   technical-debt, and roadmap-readiness review. Do not silently fix advisory
   findings or reorder the roadmap.
7. Review changes against `CODE_REVIEW.md` and `SECURITY.md`.
8. Update:
   - `HANDOFF.md`
   - `PROJECT_STATUS.md`
   - `NEXT_STEPS.md`
   - `DECISIONS.md` when a durable decision changed
   - `CHANGELOG.md`
   - `TROUBLESHOOTING_LOG.md` for a resolved or active failure
   - The applicable file under `docs/increments/`
   - Any active plan under `docs/plans/`
9. Put an exact copy-paste resume prompt in `HANDOFF.md`.
10. Confirm no secrets, build output, personal data, or local logs were added.
11. Run `$post-increment-gate`, create the consolidated report, and finalize the deterministic completion marker only when the result is `PASS` or `PASS WITH ADVISORIES`.
12. Confirm `python3 .codex/hooks/post_increment_gate.py status` reports the active increment as complete and valid.
13. Show the final Git status.

## Emergency hook bypass

Use `/hooks` to review, trust, or disable the repository Stop hook. For an emergency session, start Codex with `--disable hooks`. Record why the hook was bypassed, do not mark the increment complete, and rerun the full post-increment gate before completion. `--dangerously-bypass-hook-trust` is not a routine substitute for normal review.

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
- Consolidated report path and quality-gate result.
- Warnings and open questions.

## Exit condition

A new session can begin by reading the repository alone and can reproduce the last known result. A completed implementation increment also has a valid passing post-increment report and completion marker.
