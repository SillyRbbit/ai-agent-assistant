# Collapsed sidebar reachability

Date: 2026-10-03
Status: FAIL / Blocked; ordinary admission succeeded; required native acceptance blocked.

## Goal and current evidence

Restore independent collapsed sidebar scrolling at short heights while preserving
hover/focus labels outside its clipping list. The source's valid terminal FAIL
remains immutable. Native receipt uva5d33j measured 840x562 points: workspace
scrolling worked; sidebar did not visibly move and Settings was not reached.
Source overrides list overflow to visible and retains a heading row when hidden.
Input delivery for that historical gesture remains unknown.

## Workspace and preservation

Worktree: `/Users/hdang/.codex/worktrees/collapsed-sidebar-reachability/ai-agent-assistant`.
Branch: `codex/collapsed-sidebar-reachability`.
Baseline: `dcea9df088ddb01c26c1a79ab51e0307190f551c`, verified live main,
with tree equal to source HEAD `20961cab5410b749e62e45fa0c765364f5d29ec5`.
46 frozen files transferred byte-identically without gate state/data/build output.
External evidence: `/private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i`.
Original checkout, terminal FAIL, predecessor completion and bundles remain intact.

## Exact scope

13 successor paths; 48 cumulative paths against baseline:

- `src/styles.css`
- `src/components/ApplicationSidebar.tsx`
- `src/App.test.tsx`
- `scripts/browser/knowledge-check.mjs`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-03-collapsed-sidebar-reachability.md`
- `docs/reviews/2026-10-03-collapsed-sidebar-reachability-post-increment-review.md`

## Design and boundaries

Use one constrained collapsed navigation grid row, keeping the list as scroll
owner. Keep tooltip elements outside the list, with bounded viewport positioning
from the existing button and existing hover/focus semantics. Hide on Escape,
expansion and navigation clicks; update or dismiss on scroll/resize. No new dependencies,
timers, execution, routing, persistence, profile, branding or governance changes.
Preserve all historical documentation bodies. No saves, sends or workflow starts.

## Validation and acceptance

Affected App tests; strict lint/typecheck/formatting and frontend build. Extend
actual-App checker with real wheel movement to last/first route, keyboard focus,
visible contained tooltip, independent workspace scroll, compact/wide short-height
bounds (840x562 and 760x520 included), both states and reverse resizing.
Retain existing matrix and fixture rejection assertions. No arbitrary delays.
Documentation/repository/security/whitespace/scope/preservation/session/schema,
quality review and ordinary completion/full Stop are required. Unchanged Rust,
animation and workflow evidence is inherited, not newly executed.
One offline unsigned isolated bundle using installed tooling and process-local
Python/Xcode SDK27/Cargo workaround. Verify identity; native Computer Use with
PID-filtered CoreGraphics bounds and owner resizing; inspect actual short-height
scrolling/tooltips/composer/disclosure, then quit test-owned app and verify absence.

## Risks, rollback and stop conditions

Tooltip clipping or stale coordinates must fail tests; no hidden overflow
workaround for navigation. Stop on drift, admission rejection, scope expansion,
unsupported native access or unresolved security failure. Preserve evidence on
failure; no automatic rollback or modification of original checkout. No commit,
publication, install, owner-data or provider changes. Live QA remains parked 3/10;
retain D-127/D-128, native/provider/runtime/Codex-isolation, icon/artwork/chunk
advisories, process-local workaround and parked D-125/M1/M2.

## Progress

- Verified baseline-tree equivalence and transferred 46 frozen files.
- Ordinary admission succeeded. Implementation and acceptance pending.

## Final results

43 affected App tests, strict frontend checks/build, 28 actual-App browser cases,
formatting and offline unsigned bundle build passed. Required native QA stopped
on supported Computer Use `noWindowsAvailable` at the first scroll after a fresh
760x520 collapsed capture. No native scrolling/expanded/tooltip pass is claimed.
One launch; no retry, input, saves, sends or workflow. SIGTERM cleanup to the exact
test-owned process and absence verified. Preserve every failed diagnostic receipt.
Truthful terminal FAIL required; no passing marker. Final documentation/evidence
checks, schema, terminal status and full Stop follow the frozen report.
