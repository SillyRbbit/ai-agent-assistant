# Command Center measurement readiness

Increment `command-center-measurement-readiness`, owner-authorized 2026-09-30.
Worktree `/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant`;
branch `codex/provider-milestone`; HEAD `fe7e663e175eaf7c515c17397cdd81060137a5b6`.

## Goal, evidence and non-goals

Release the measured graph from its stale readiness flag without changing geometry
or any graph/application behavior. Prior direct QA observed visible Conductor and
Application coordinator plus nine saved names, but 100% zoom, clipped topology
and disabled Fit/Reset/Zoom after canvas measurement. Inspector/reentry/manual QA
were not completed. Preserve that outcome; do not claim prior native QA passed.

Installed React Flow 12.11.3's default hook reads cached `nodesInitialized`.
Controlled-node ResizeObserver updates internal dimensions/handles but does not
refresh that flag without a node-state round trip. The public
`includeHiddenNodes: true` hook path inspects current per-node dimensions and handle
bounds. Cortexa supplies fixed dimensions and generates no hidden flow nodes.
Change only the hook option; retain real measurement, canvas and viewport gates.
No timers, remounts, dependencies, graph redesign, native launch or live request.

## Frozen scope and preservation

Exactly eleven successor paths / 45 cumulative paths:

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-30-command-center-measurement-readiness.md`
- `docs/reviews/2026-09-30-command-center-measurement-readiness-post-increment-review.md`
- `src/features/command-center/components/OperationalTopologyAdapter.test.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`

All 43 inherited paths retained. Seven edited root bodies remain historical
suffixes; previous reports/plans, production/provider/Rust/dependency/governance
bytes protected. Prior raw completion state and debug bundle are archived
byte/mode-identically before ordinary admission. Eight valid worktree snapshots,
36 prunable entries and external evidence frozen in
`/private/tmp/cortexa-command-center-measurement-readiness-evidence`.

## Checklist and validation

- [x] Inspect candidate/source and complete/valid predecessor; freeze preservation.
- [x] Ordinary admission under this increment.
- [x] Add real React Flow measurement regression and demonstrate old-path failure.
- [x] One-line current-internals option; affected adapter/page tests, strict lint,
      typecheck, formatting and frontend build.
- [x] Offline unsigned debug app-only bundle; preserve old artifact.
- [x] Documentation/repository/security/whitespace, scope and preservation.
- [x] Session, architecture/security/code-health/readiness review, exact report.
- [ ] Ordinary finalization, complete/valid and full-payload Stop.

Regression uses actual controlled React Flow and ResizeObserver deliveries, checks
partial/all-node measurement and same-ID/same-size replacements, and verifies
Cortexa selects the public option. No fake readiness flag in that library case.
Existing delayed startup, route reentry, manual viewport and all assertions remain.
Frontend tier applies; Rust suites/full verify are Not run for unchanged native/IPC/
configuration boundaries. Native QA remains separate and pending.

## Risks, stop conditions and next action

This option includes every flow node; no hidden nodes are currently generated.
Future hidden nodes would still need dimensions/handles. Do not weaken the guard
or use a synthetic flag to obtain a pass. Repair recoverable in-scope failures;
stop on conflicting drift, unsupported evidence, admission rejection, security
failure or scope expansion. Rollback requires owner instruction and preservation.
Retain Conductor/Application coordinator/AgentOrchestrator, nine profiles, topology,
authority, routing, cancellation, D-127/D-128 and all live-success/Codex-isolation
and process-local native-workaround advisories. OpenAI parked 4/5 used;
D-125/M1/M2 parked. No credential inspection, provider request, commit/publication.
Next after passing automated acceptance: separately authorized key-free native QA.

## Observed acceptance checkpoint

Actual-library red run Failed 1/26 with the old option; corrected combined run
Passed 61/61. Initial strict lint Failed only on the new inline import type;
namespace type-only import fixed it. Final adapter 26/26, lint/typecheck, formatting,
frontend build and offline unsigned arm64 app bundle Passed. Native QA is pending.
Executable SHA-256: `93bfdefe94a45016928a812bb161a2460f535a9b5708114c9ee4c2bf441ccf27`.
Exact scope/preservation: 11/45 paths, all protected bytes/historical suffixes, seven
other valid worktrees, 36 prunable entries, external evidence and old artifact/state
archive Passed. The report freezes the review and actual command history. Final
documentation/schema/finalization/status/full Stop receipts are external; the last
checklist item remains pending until those commands actually pass. Do not rewrite
this frozen plan/report after completion merely to tick the last checkbox.
