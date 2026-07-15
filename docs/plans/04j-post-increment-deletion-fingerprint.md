# Repository Workflow Increment 4J - Post-increment deletion fingerprint

Status: Complete
Owner: Project maintainer
Last updated: 2026-07-15

## Goal

Make the deterministic post-increment workspace fingerprint represent existing repository content so a valid completion marker survives a commit that deletes reviewed tracked files.

## User-visible outcome

- A reviewed tracked deletion has the same workspace fingerprint immediately before and after its commit.
- Deleting a tracked file after finalization still invalidates the completion marker.
- Increment 4I remains preserved, unpushed, and unmerged until it can be reconstructed and revalidated on the corrected workflow baseline.

## Scope

- Change only `workspace_fingerprint` handling for paths that are absent from the working tree.
- Add one positive committed-deletion regression and one negative post-finalization-deletion regression.
- Document the exact existing-content snapshot semantics, diagnosed failure, recovery boundary, and verification evidence.

## Explicit non-goals

- No legacy marker migration, fallback fingerprint, schema-version change, or report-inventory relaxation.
- No change to `changed_paths`, report validation, suspicious-path checks, report hashing, state storage, hook configuration, or skills.
- No application, Rust, TypeScript, Tauri, IPC, database, dependency, permission, gateway, approval, audit, dispatch, or execution behavior change.
- No commit, push, merge, or Increment 4I reconstruction within this increment.

## Baseline

```text
branch: codex/repository-workflow-increment-4j
commit: e3af5a4
main relative to origin/main: synchronized
working tree before edits: clean
preserved 4I branch: codex/phase4-increment-4i at cf9d701
preserved 4I publication state: unpushed and unmerged
npm run test:hooks: passed, 15 tests
```

The Increment 4I marker became invalid after commit because a staged tracked deletion contributed its path and a `missing` token before commit, while the committed deletion disappeared from `git ls-files --cached` and contributed nothing afterward. Supported re-finalization then failed because the committed clean tree no longer matched the report's pre-commit changed-file inventory.

## Exact files expected to change

```text
.codex/hooks/post_increment_gate.py
.codex/hooks/tests/test_post_increment_gate.py
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
SECURITY.md
TROUBLESHOOTING_LOG.md
docs/increments/04j-post-increment-deletion-fingerprint.md
docs/plans/04j-post-increment-deletion-fingerprint.md
docs/plans/README.md
docs/reviews/2026-07-15-04j-post-increment-review.md
```

## Implementation steps

- [x] Preserve `codex/phase4-increment-4i` at `cf9d701` and create a new branch from synchronized `main`.
- [x] Begin mandatory ignored gate state for Increment `04j` before edits.
- [x] Move path hashing after successful metadata lookup and omit currently absent paths.
- [x] Add positive and negative tracked-deletion regressions.
- [x] Run focused and complete verification.
- [x] Review code, security, scope, secrets, generated output, and the complete diff.
- [x] Synchronize project memory and create the consolidated report.
- [x] Finalize and validate the completion marker.

## Security and privacy considerations

- `changed_paths` continues to include reviewed staged or unstaged deletions, and the report must still inventory them exactly.
- A file present at finalization contributes its path, executable bits, type, and content or symlink target; deleting it afterward changes the fingerprint and invalidates the marker.
- A file already absent at finalization contributes no current workspace content, matching its state after commit.
- Metadata and content read failures still fail closed. No network, transcript, personal-content, report-execution, or arbitrary-command path is added.
- The hook remains a trusted workflow guardrail rather than a security or authorization boundary.

## Risks

- Omitting all missing cached paths could conceal an unreviewed deletion if the final snapshot semantics were wrong. The negative post-finalization deletion regression proves that a previously hashed file still invalidates the marker when removed.
- Concurrent filesystem mutation remains a workflow race. Existing metadata and file-read errors remain fail closed; no broader concurrency redesign is included.
- Pre-fix completion markers for deletion commits remain invalid. Increment 4I must be reconstructed and revalidated rather than silently migrated.
- Mixing this correction into 4I would expand its approved source scope, so the preserved 4I branch must remain untouched.

## Verification commands

```bash
PYTHONPYCACHEPREFIX=/private/tmp/cortexa-4j-pycache python3 -m py_compile .codex/hooks/post_increment_gate.py .codex/hooks/tests/test_post_increment_gate.py
PYTHONDONTWRITEBYTECODE=1 python3 .codex/hooks/tests/test_post_increment_gate.py -v
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
npm run test:hooks
npm run format:check
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Manual verification:

- Review the exact trusted hook diff and confirm it retains fixed Git commands, bounded inputs, and no product-source modification.
- Confirm the 4I branch still resolves to `cf9d701` and has not been pushed or merged.

## Rollback or failure strategy

If verification fails, leave `main` and the preserved 4I branch unchanged and do not publish 4J. Reverting the isolated hook/test change restores the 4G fingerprint implementation and its known deletion-commit defect; no application, dependency, or stored-data rollback is required.

After 4J is separately approved and merged, preserve the old 4I branch under a backup name, recreate 4I from corrected `main`, apply `cf9d701` without committing, rerun the complete 4I gate, and publish only after its post-commit marker remains valid. That recovery is outside 4J.

## Exit criteria

- Both deletion regressions and all existing hook tests pass.
- Complete repository verification and dependency audit pass.
- The exact 15-file scope contains no application or generated output.
- Code and security review find no blocking issue.
- The project-memory documents and consolidated report match actual evidence.
- The 4J marker reports complete and valid with `PASS` or `PASS WITH ADVISORIES`.

## Verification result

All 17 focused hook tests pass, including deletion of the last tracked file and its containing directory before finalization and the corresponding unreviewed post-finalization deletion. `npm run verify` passes with 124 frontend, 99 Rust library, and 11 Rust integration tests plus formatting, ESLint, Clippy with warnings denied, builds, and Tauri release no-bundle. The network-enabled dependency audit reports zero vulnerabilities.

Exact scope, conflict, secret, generated-output, complete-diff, code, and security reviews pass. The consolidated result is `PASS WITH ADVISORIES`; the advisory requires 4J publication before separately approved 4I reconstruction.
