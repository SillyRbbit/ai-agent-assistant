# Demo cleanup main reconciliation

Date: 2026-09-21. Increment: `demo-cleanup-main-reconciliation`.
Branch: `codex/demo-cleanup-main-reconciliation`.

## Goal and authority

Reconcile the completed PR #116 demo cleanup with the dependency-and-Rust-audit
changes merged through PR #117. Preserve the cleanup implementation exactly,
retain both predecessor histories, publish only after complete validation, and
stop before merging PR #116.

The owner explicitly authorized this bounded successor, its ordinary admission,
one isolated worktree, a normal merge of verified current main, a fast-forward
update to the existing PR branch after passing checks, and the exact 24-path
ceiling below.

## Verified starting evidence

Before admission, `origin/main` resolved to
`ba1336e92734585adcd336ea0a33b9e89a320716`, the squash merge of PR #117, and
PR #116 resolved to `92c2e19eb71b08ad7a2996f83e034afe9babd52a` with exactly
22 paths. The common parent is
`f176c36cc701b5a296162331cfcb2600b2157663`.

The original checkout remains at
`87d52a6d22a589605f485cf7b9765bae39da4f00`, seven commits ahead and one behind
current remote main, with all dirty changes preserved. The cleanup worktree,
successful audit-unblock worktree, and terminal-failed dependency worktree keep
their original heads and gate states.

A read-only three-way analysis identified exactly five conflicts, all leading
current-state sections in `HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`,
`PROJECT_STATUS.md`, and `ROADMAP.md`. `CHANGELOG.md` and `DECISIONS.md` merge
automatically. All 13 UI paths and both original cleanup records are conflict
free.

## Exact scope

The PR-relative ceiling is exactly these 24 paths:

- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `docs/plans/2026-09-20-demo-cleanup-publication.md`
- `docs/reviews/2026-09-20-demo-cleanup-publication-post-increment-review.md`
- `docs/plans/2026-09-21-demo-cleanup-main-reconciliation.md`
- `docs/reviews/2026-09-21-demo-cleanup-main-reconciliation-post-increment-review.md`
- `src/features/command-center/CommandCenterPage.test.tsx`
- `src/features/command-center/CommandCenterPage.tsx`
- `src/features/command-center/command-center.css`
- `src/features/command-center/components/CommandCenterActivityStream.tsx`
- `src/features/command-center/components/CommandCenterHeader.tsx`
- `src/features/command-center/components/CommandCenterOverview.tsx`
- `src/features/command-center/components/ContextualInspector.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.test.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`
- `src/features/command-center/components/OperationalTopologyPanel.test.tsx`
- `src/features/command-center/components/OperationalTopologyPanel.tsx`
- `src/features/command-center/components/TopologyStructuredView.tsx`
- `src/features/command-center/useCommandCenterState.ts`

The 13 UI blobs, including the Structured-view deletion, must match `92c2e19`
exactly. The original cleanup plan and finalized report must remain byte-for-byte
unchanged. Every package, lockfile, Cargo audit, D-127, security, testing, and
troubleshooting change from `ba1336e` is inherited main state and must not appear
as a PR-relative change.

## Conflict resolution

Use a two-parent merge with first parent `92c2e19` and second parent `ba1336e`.
For each of the five expected conflicts, preserve both predecessor sections
verbatim as dated history and prepend one concise superseding section describing
the reconciliation. Do not edit any automatically merged executable or audit
path. Any sixth conflict or different parent stops the increment.

## Non-goals and security

No UI implementation, dependency resolution, audit-baseline change, Cargo
change, native code, IPC, permission, CSP, credential, workflow, hook, skill,
harness, layout engine, runtime, provider, D-125, M1, or M2 work. The merge adds
no product capability or authority. Native GUI smoke remains advisory and
browser evidence does not prove native WebView behavior.

## Validation

After the last relevant edit:

- install from the merged lockfile with `npm ci`;
- inspect exact dependency versions and run full and production npm audits;
- run pinned cargo-audit 0.22.2 and the repository Cargo audit gate;
- run the tracked-secret scan;
- run focused Command Center tests and `npm run verify`;
- run documentation, repository, whitespace, and exact-scope checks;
- verify all 13 UI blobs against `92c2e19` and inherited audit files against
  `ba1336e`;
- verify actual desktop and narrow browser layouts, including Fit View;
- run session, architecture, security, code-health, technical-debt, readiness,
  quality, and post-increment gates.

Required checks must pass. Native GUI smoke remains a non-blocking advisory
unless it is actually performed. Any changed advisory evidence, scope drift,
unresolved conflict, mutated predecessor report, or required failure stops.

## Publication and rollback

After a valid completion marker, commit the reconciliation with a Conventional
Commit and push the resulting two-parent history to
`origin/codex/demo-cleanup-publication` as a fast-forward only. Update PR #116's
description to disclose the 24-path scope and inspect every exact-head job.
Stop before merge.

On failure, preserve the isolated worktree and evidence without reset, clean,
stash, force-push, scope repair outside the ceiling, or automatic rollback.

## Progress

- [x] Required refs, worktrees, PR scope, and predecessor gate states verified.
- [x] Ordinary admission started.
- [x] Two-parent merge produced exactly the five predicted conflicts.
- [x] Conflict strategy preserves both predecessor sections additively.
- [x] Required local validation complete.
- [x] Quality reviews complete; post-increment finalization uses this frozen candidate.
- [ ] Reconciliation committed and pushed without force.
- [ ] New exact-head CI complete.
