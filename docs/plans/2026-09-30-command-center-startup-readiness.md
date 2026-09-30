# Command Center startup readiness

Increment `command-center-startup-readiness`, owner-authorized 2026-09-30.
Worktree `/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant`;
branch `codex/provider-milestone`; HEAD `fe7e663e175eaf7c515c17397cdd81060137a5b6`.

## Goal and non-goals

Correct first-entry node readiness without timers, remounts, dependencies or graph
redesign. Preserve Conductor/Application coordinator/AgentOrchestrator identity,
nine profiles, node IDs, topology, ownership, authority, routing and cancellation.
No provider request, credential access, commit/publication or native launch.

## Evidence and design

Direct native QA observed domain boxes without bot nodes at first entry. The owner
reports navigation away/back renders correctly; that route result is owner evidence.
Installed React Flow 12.11.3 `adoptUserNodes` replaces measured dimensions when new
controlled node objects arrive, while `NodeWrapper` hides nodes lacking dimensions.
This adapter rebuilt fixed-size cards with CSS size only. Explicit node width/height
now match the existing styles; React Flow can display cards before asynchronous
measurement. Its public `useNodesInitialized` hook runs inside the existing provider
and reports readiness to the adapter. Automatic fit reacts to measurement completion
as well as canvas size/viewport initialization. Manual viewport ownership is retained.
This demonstrates an implementation gap, not a confirmed native fix before QA.

## Exact frozen scope and preservation

Exactly 11 successor paths / 43 cumulative paths:

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-30-command-center-startup-readiness.md`
- `docs/reviews/2026-09-30-command-center-startup-readiness-post-increment-review.md`
- `src/features/command-center/components/OperationalTopologyAdapter.test.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`

Forty inherited changed paths are retained. All bytes outside these eleven paths
are protected. Seven edited root bodies remain exact historical suffixes; preceding
plans/reports remain unchanged. Current Conductor raw state/marker are archived
byte/mode-identically before ordinary admission. Eight valid worktree snapshots,
36 prunable entries and external evidence are frozen. Archive the current bundle
before generating its replacement. Evidence:
`/private/tmp/cortexa-command-center-startup-readiness-evidence`.

## Checklist and verification

- [x] Inspect evidence, freeze scope and preservation, ordinary admission.
- [x] Add delayed startup, route reentry, manual viewport and dimension regressions.
- [x] Confirm regressions fail against old adapter: 3 failed, 22 passed.
- [x] Supply existing node dimensions and reactive measurement-ready fit.
- [x] Affected adapter/page frontend tests, strict lint, typecheck and formatting.
- [x] Offline unsigned debug app-only bundle, preserving previous artifact.
- [x] Documentation, repository, security, whitespace, scope and preservation.
- [x] Session, architecture/security/code-health/readiness reviews, exact schema.
- [ ] Ordinary finalization, complete/valid status and full-payload Stop.

Frontend tier applies; Rust suites/full verify are Not run because Rust, IPC,
storage, permission, dependency and configuration bytes are unchanged. Existing
native results are inherited history. Native QA remains separate and pending.

## Risks, rollback and next action

Measurement and canvas can become ready in either order; regressions must cover
both without arbitrary waits. Do not steal manually moved viewports. Fixed sizes
must remain identical to existing layout styles. No speculative messaging or tenth
bot. Stop on conflicting drift, required scope expansion or unsupported evidence;
repair routine in-scope failures in this task. Rollback requires owner direction
and must preserve existing changes. Keep OpenAI parked 4/5 used, D-125/M1/M2 parked;
retain D-127/D-128, live-success/Codex isolation and process-local Python 3.12,
Xcode/SDK 27.0/Cargo strip override advisories. Next: separately authorized key-free
fresh-entry and route-reentry native QA of the updated artifact.

## Actual results and remaining verification

Affected adapter/page tests: **60/60 passed** (25 adapter, 35 page), including
four new regressions for delayed measurement, subsequent route entry, manual pan
ownership and fixed dimensions across controlled name updates. Red run against
the old adapter: 3 failed / 22 passed. First combined run: 1 failed / 59 passed;
the new manual regression used an unused event. It now uses the actual `onMove`
path; no production behavior or existing assertion was weakened. All failure logs
remain. The first final-documentation/formatting rerun failed on the missing blank
separator before seven historical bodies; only additive spacing was corrected,
and the affected checks were rerun. The external report helper initially selected
the wrong contextual worktree; its explicit local inventory was corrected before
any document write. No predecessor script was changed. Strict ESLint, typecheck, formatting, frontend build and offline unsigned
app-only debug packaging passed. Documentation/repository/security/whitespace,
exact scope, historical suffixes, protected bytes, state archive, registry and
session inventory passed before report freeze; documentation checks rerun after
this additive evidence update. No Rust suites or full verify were run for this
frontend-only delta. Earlier native/test results remain inherited history.

Native first-entry appearance is **Manual verification pending**, not proved by
mocked readiness transitions, DOM checks or packaging. Direct previous QA observed
domain boxes without nodes; route-reentry success is owner-reported evidence.
The installed library/source and red regressions establish the readiness gap;
they do not establish that every native rendering cause is resolved. Remote CI
for these uncommitted changes is pending. Retain D-127 audit debt, D-128 custody/
remote-abort limits, native/provider/runtime live-success and Codex isolation/
internal-retry advisories. Process-local Python 3.12, Xcode/SDK 27.0 and Cargo
build-override strip="none" remain build prerequisites. OpenAI parked **4/5 used**;
D-125/M1/M2 parked. No provider request, native launch, Save, commit or publication.

Executable SHA-256 `03dcce38521eb0d3c9632568c84ee55338dad893f5d42289c2bd50b1fb8dc4c3`. Prior bundle preserved byte/mode-identically.
Final schema/finalization/status/Stop remain external receipts following report
freeze; the last checkbox is completed only by those actual passing receipts.
Next: separately authorized key-free native first-entry/reentry QA.
