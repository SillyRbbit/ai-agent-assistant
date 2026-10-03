# Bots mascot preview layout

Date: 2026-10-03. Status: PASS WITH ADVISORIES; finalized completion is established by external status/Stop receipts.
Worktree: `/Users/hdang/.codex/worktrees/bots-mascot-preview-layout/ai-agent-assistant`.
Branch: `codex/bots-mascot-preview-layout`.
Baseline/source HEAD/live main: `662fe1a57a1148a215beb04b215ad35681c4cbe6`.
Evidence: `/private/tmp/cortexa-bots-mascot-preview-layout-evidence`.

## Goal, evidence and scope

Saved browser-6.log identifies `.bot-preview` label overflow at 961px:
743px content versus 726px available, while sidebar/workspace meet at x=220.
The preview row does not wrap and the large mascot cannot shrink. Permit wrapping
and text reflow only; retain artwork, dimensions, padding, animation and controls.
Transfer 96 frozen paths without gate state; both historical FAILs stay immutable.
Eleven successor paths / 99 cumulative paths:

- `src/features/agents/AgentsPage.css`
- `scripts/browser/knowledge-check.mjs`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-03-bots-mascot-preview-layout.md`
- `docs/reviews/2026-10-03-bots-mascot-preview-layout-post-increment-review.md`

## Invariants and non-goals

Fixture, navigation CSS, other product bytes, names, profiles, rooms, runtime,
provider, authority, graph, inspector overlay and all historical bodies frozen.
No truncation, hidden overflow, arbitrary delay, timers, remount, dependency,
governance, save, Send, workflow, credential, owner-data or publication changes.

## Checklist

- [x] Source/live baseline, 96 files, histories and artifacts verified.
- [x] One isolated worktree, byte transfer and ordinary admission.
- [x] Preview CSS and meaningful geometry/control assertions.
- [x] 1600/961/960/959/761/760px, short height, both navigation states,
      forward/reverse resizing and Knowledge/Collaboration/Bots routes.
- [x] Nine choices, preview text containment, fixed mascot dimensions/padding,
      reachable controls, scrolling, no overflow, inspector close/Escape.
- [x] Affected frontend tests, format, strict lint/typecheck and frontend build.
- [x] One offline unsigned isolated bundle, identity and artifact preservation.
- [x] Computer Use wide/compact Bots, all nine choices, artwork/full labels,
      settings/controls, scrolling and route transitions; no Save; quit QA app.
- [x] Docs/repository/security/whitespace, preservation, review/schema/session,
      quality, complete/valid and full-payload Stop.

## Validation and risk

Rerun affected AgentsPage/BotAppearance/App tests because product CSS changed;
reuse unchanged Rust/animation/workflow evidence truthfully. Existing actual-App
checker retains fixture readiness/rejection assertions. Layout regressions measure
the real preview label, canvas and controls, not implementation strings.
Build with installed offline Python3.12.1/Xcode SDK27/Cargo strip-none route,
new bundle name and existing isolated synthetic data. Preserve old bundle bytes.
Use fresh supported Computer Use; no private credential/argument/environment reads.
Stop on conflicting drift, admission rejection, unsupported native access, necessary
scope expansion or unresolved security failure. Recover in-scope CSS/checker failures.
No rollback or automatic successor. Retain all advisories, live QA 3/10 and D-125/M1/M2.

## Progress / next action

Admission and transfer passed. Browser matrix passed all 22 cases and 198 bot selections.
Focused frontend: 3 files / 67 tests passed; strict lint/typecheck/frontend build passed.
Native bundle passed; exact running executable verified. Direct Computer Use wide/compact
QA passed; app quit. See native-observations.json for precise coverage and limitations.
Closeout checks passed. Finalize, then verify complete/valid status and full-payload Stop.
