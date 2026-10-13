# End a working session

## Goal

Leave the repository in a state that another assistant session can resume without access to the current conversation.

Use `.agents/skills/session-end/SKILL.md` when repository skills are available.
Otherwise use `prompts/workflows/end-session.md`, which incorporates the
post-increment quality and marker closeout.

## Procedure

1. Stop only task-owned development processes when no longer needed; preserve other tasks.
2. Inspect all changes:

   ```bash
   git status --short --branch
   git diff --stat
   git diff
   ```

3. Run relevant final checks under the Risk-Based Validation Policy; reuse valid unchanged-input stages with provenance. The full cross-cutting suite includes:

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
13. Record the QA-build cleanup disposition using the procedure below. Cleanup may
    remain deferred after implementation completion; a completion marker alone
    does not authorize deletion or prove cleanup eligibility.
14. Show the final Git status.

## Task-owned QA-build cleanup

This is a conditional completion/closeout step, not a deletion command or blanket
cleanup authorization. It does not authorize a commit, push, PR, merge or release.
Use existing task authorization only when it explicitly covers the identified
cleanup; otherwise obtain authorization for the concrete inventory before removal.

### Eligibility: every condition must be established

- Required implementation, automated checks and native QA are complete and passed.
- Required owner review is complete, with its approval retained.
- Publication and merge are complete where required and separately authorized;
  applicable post-merge verification passed. Record evidence or a justified
  not-applicable disposition for conditional requirements; absence of publication
  authorization is not a waiver of required publication.
- No active session, process, pending acceptance or rollback requirement depends
  on the artifacts. Check across tasks sharing the paths, not just this session.
  An idle session or lack of a visible process alone does not establish this.

Unknown, failed or pending conditions mean **retain**. Keep the review build while
owner review or mandatory acceptance is pending. Never mark incomplete QA passed,
rewrite historical failure, or waive rollback needs to make artifacts eligible.
This step does not change existing implementation gates or grant successor authority.

### Before removal

1. Record each exact absolute artifact path, resolved location, type, logical and
   allocated size, filesystem, owning task and ownership evidence. Identify shared
   consumers, expected contents and the proposed retained/removed disposition.
   A temporary location or filename prefix alone is not ownership proof. Do not
   follow links into other trees; stop on unexpected links, shared content, changed
   identity, unresolved attribution or evidence drift.
2. Confirm source and necessary build configuration are durably preserved outside
   temporary directories, including uncommitted source where needed. Retain the
   exact revision/patch, dependency/lockfile and toolchain identities, sanitized
   build configuration and rebuild/recovery instructions. Never copy credentials
   or owner data into a cleanup evidence package. Rebuildability does not replace
   required acceptance or rollback artifact custody.
3. Preserve acceptance results, sanitized logs, manifests, hashes, approvals and
   recovery instructions outside temporary directories and outside the deletion
   inventory. Verify the durable copies against their recorded identities before
   removing originals. Keep the evidence-to-source/build binding, original failures
   and any missing-evidence qualifications; do not manufacture missing receipts.
4. Confirm any signals target only task-owned processes using captured process
   identity and current executable/start identity; recheck before signaling and
   verify exit. Never use broad process-name matching, stale PIDs or unrelated
   process termination. If another session still needs the process/artifact,
   retain it. Record no signaling when none is needed.
5. Recheck eligibility, ownership, preservation and cleanup authorization immediately
   before removal. Retain a durable inventory/receipt and baseline disk measurements.
   Stop the affected removal if any prerequisite changes or cannot be established.

### Removal boundaries and reporting

Eligible items may include task-owned disposable QA app bundles, isolated build
output directories and temporary fixtures after all dependencies are resolved.
Remove only the individually identified disposable items. A directory is eligible
only when its complete contents and ownership have been checked; an allowlisted
parent must not authorize removal of unexpected descendants or shared content.
Blanket directory deletion and `git clean` are prohibited. Never run `cargo clean`
against an active or shared target directory.

Never remove source checkouts, linked worktrees, Git metadata, owner application
data, credentials, historical failure records, required evidence or recovery
backups. When protected and disposable content are mixed, retain the containing
path unless an exact safe disposable subset is established. A backup directory
containing a review app remains protected; do not infer it is disposable by name.

After authorized removal, verify the exact removed paths are absent and retained
source/configuration/evidence/recovery identities still match. Record every success,
partial failure and retained item. Measure allocated artifact bytes before/after
and available space on the same filesystem; report actual observed space recovery
separately from logical sizes or estimates. Account for concurrent disk activity,
APFS clones/snapshots or delayed reclamation; if attribution is uncertain, report
that uncertainty instead of claiming an estimated total as recovered space.

The durable cleanup receipt and handoff must identify:

- Eligibility evidence, cleanup authorization, owner/task and exact inventory.
- Removed paths, failures, actual measurements and retained paths with reasons.
- Preserved source/configuration/evidence locations and hash verification results.
- Exact rebuild commands, required nonsecret environment/toolchain/dependency
  setup and recovery instructions; rebuilding does not recreate historical QA.
- If cleanup is deferred: the exact pending condition, artifact dependencies,
  review-build location and next decision/check. Record zero removal and zero
  space recovered when no cleanup ran; no speculative cleanup schedule.

Do not change frozen acceptance reports or completion fingerprints to record
later cleanup. Use an additive durable cleanup receipt/handoff through the existing
authorized documentation route. Required evidence must remain accessible after
cleanup; a receipt referencing a deleted-only file is not preserved evidence.

## Gate or access blockers

Preserve the rejection and evidence, identify the exact restriction, and use an
existing supported authorized route. Do not disable hooks or bypass platform
approval rejection to complete a milestone. Missing access is a blocker; finish
independent work before handing back.

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

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
