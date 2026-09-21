# Demo cleanup publication

Date: 2026-09-20. Increment: `demo-cleanup-publication`.
Branch: `codex/demo-cleanup-publication`.

## Goal and authority

The owner explicitly approved an isolated cleanup-only PR into main, with
commit and push after validation and a stop before merge. Publish the existing
completed UI delta without repeating implementation. This bounded approval
supersedes the historical requirement to retain Structured view for this
cleanup only; it does not accept or advance the historical M1/M2 plan.

## Current-state evidence

Remote main was verified by `git ls-remote` at
`f176c36cc701b5a296162331cfcb2600b2157663`. The original checkout remains on
local main at `87d52a6d22a589605f485cf7b9765bae39da4f00`, seven commits ahead
with 43 changed paths. None of the seven commits changes product source.
Their harness and admission work is excluded. Unfinished local D-125 and
historical failures remain preserved and parked, without acceptance claims.

An isolated worktree began from the verified remote commit. Its existing gate
admitted this increment normally. The unchanged lockfile installed 296
packages from the local cache. The baseline adapter/panel tests passed 21
tests. The 13-file UI patch applied without conflicts.

## Scope and exact files

- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `docs/plans/2026-09-20-demo-cleanup-publication.md`
- `docs/reviews/2026-09-20-demo-cleanup-publication-post-increment-review.md`
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

The 13 UI changes are transferred byte-for-byte from the original checkout,
including the Structured component deletion. Seven current-state documents
receive additive publication entries based on remote main, plus this plan
and a new candidate-specific report. No original local report is rewritten
or treated as verification of this candidate.

## Components and invariants

Keep React Flow, existing widths, orchestrator sizing, deterministic fixtures,
connectors, selection, focus, inspector, activity and graph controls.
Remove Structured composition, selector, narrow fallback and exclusive code.
Use plain readable group headers inside the existing dashed outlines.
All nine agent cards use 120px layout/render height; standard row tops are
180px and 312px, with a 12px gap. The four two-card containers share bottoms;
Governance remains one card. Existing compact behavior remains.

## Non-goals and security

No governance, hooks, skills, harnesses, dependencies, native code, IPC, CSP,
permissions, credentials, live providers, layout engine, redesign or unrelated
polish. No historical milestone is completed or waived. This remains a
private deterministic demo with unchanged Rust authorization boundaries.

## Validation and acceptance

Run after the final relevant edits:

```sh
npm run format:frontend
npm run lint:frontend
npm run typecheck
npm run test:frontend -- src/features/command-center src/App.test.tsx
npm run build:frontend
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 -B .codex/hooks/session_end_gate.py
```

Verify actual desktop and narrow browser layouts, equal card heights, row
edges, two-line titles, status spacing, header clearance, container bounds,
connectors, pan/zoom/Fit View, keyboard selection, inspector and activity.
Check exact 22-path scope and original-checkout byte/Git preservation.
Apply architecture, security, code-health, debt and readiness review, then
the unchanged post-increment gate and Stop hook. Native GUI smoke remains a
truthful advisory; browser evidence does not prove native WebView behavior.
Rust, full verify and historical harness acceptance are outside this
frontend/documentation tier. Applicable remote checks run after publication.

## Risks and stopping conditions

Main risks are accidental import of unpublished governance history, stale
evidence and responsive regressions. Stop on admission failure, unexpected
scope or failed required validation. Preserve the original checkout and
candidate; do not reset, stash, clean, rewrite history or automatically roll
back. No merge is authorized.

## Progress and final results

Admission, dependency setup, 21 baseline tests and exact UI transfer passed.
Candidate lint, typecheck, 191 affected tests, production build, actual browser
checks at 1400x900, 820x800 and 600x800, documentation, repository, security,
whitespace and session checks passed. The exact 22-path scope and all 879
recorded original-checkout paths plus original HEAD/status were preserved.
Quality: PASS WITH ADVISORIES for pending native GUI smoke. No source repair
was needed. Gate completion and publication follow the frozen
[candidate report](../reviews/2026-09-20-demo-cleanup-publication-post-increment-review.md)
for actual results. Gate completion requires a valid frozen report.

## Next step

After passing verification, commit only this scope, push the isolated branch
and open a PR into main. Stop before merging or starting product work.
