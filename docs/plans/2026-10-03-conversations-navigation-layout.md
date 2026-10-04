# Conversations navigation layout — 2026-10-03

## Authority and current state

Owner explicitly authorizes this eleven-path increment. Worktree:
`/Users/hdang/.codex/worktrees/bots-mascot-preview-layout/ai-agent-assistant`;
branch `codex/bots-mascot-preview-layout`; HEAD `20961cab5410b749e62e45fa0c765364f5d29ec5`.
Prior complete/valid sidebar alignment raw state, 43 candidate paths and two logo
artifacts were byte-verified and archived externally before ordinary begin.
New expected cumulative inventory: 46 paths. No competing writer identified in
recent task inventory; snapshot comparison guards unexplained drift.

## Goal, scope and non-goals

Add `.conversation-page` to both existing compact navigation-track selectors.
At <=960px reserve actual expanded width or 68px collapsed width. Preserve sidebar
state, branding/mascot assets, dimensions, inspector overlays, graph, profiles,
provider routing, permissions, approvals, cancellation and storage. No redesign,
auto-collapse, timers, remounts, dependencies or governance changes.

Exact scope:

- `src/styles.css`
- `scripts/browser/knowledge-check.mjs`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-03-conversations-navigation-layout.md`
- `docs/reviews/2026-10-03-conversations-navigation-layout-post-increment-review.md`

## Checklist and validation

- [x] Inspect instructions/current source, Git, prior valid completion and QA receipt.
- [x] Freeze exact scope; archive predecessor; ordinary begin accepted.
- [x] Extend two CSS selectors and existing checker only; fixture unchanged.
- [x] 42 App tests, lint/typecheck and frontend build; final formatting pending closeout.
- [x] Actual-App Conversations matrix at 1600/961/960/959/760/595px, both states,
      short/tall heights, reverse resizing, route transitions, unobscured controls,
      disclosure/scrolling and no horizontal overflow. Observe animation completion.
- [x] One offline unsigned isolated bundle; preserve previous artifacts and verify identity.
- [ ] Computer Use native wide/compact, expanded/collapsed, no typing/Sends/runs;
      reuse existing isolated synthetic data, quit only test-owned app.
- [ ] Documentation/repository/security/whitespace/scope/preservation/session,
      engineering reviews, report schema, complete/valid and full Stop.

## Risks, evidence and rollback

The generic compact overlay used 68px despite 220px expanded navigation at 760px.
Conversations was missing from the existing route exceptions. Source and saved
screenshots explain the observed obscuring; no resize race is asserted.
Browser evidence uses isolated existing actual-App fixture, denying execution.
Native QA uses separate existing synthetic app identifier, never owner app data.
Old native artifacts do not contain this correction and cannot count as new QA.
No automatic rollback; recoverable in-scope failures are repaired and retained.
Stop on conflicts, rejected admission, unsupported access, security failure or scope expansion.

## Progress and exact next action

Both selectors and all 24 browser cases pass. Initial locator timeout is retained;
the semantic combobox locator fixes only the checker. One unsigned isolated bundle
built; exact process identity and old bundle preservation verified. Direct native
wide expanded/collapsed passed. Native compact remains unobserved because gestures
and the supported window-menu sizing action left size unchanged. No application
defect or native compact pass is inferred. Test-owned app quit; exact executable
absence verified. No typing, Sends, Saves or runs.

Required native compact acceptance blocks completion: quality FAIL, readiness
Blocked. Finish allowed documentation/preservation/security/schema closeout, freeze
the report, use ordinary close-failed and verify valid terminal FAIL/full Stop;
never call finalize. No successor or promotion is authorized. Then require a
separate owner acceptance decision, preserving this record. Evidence is external
in cortexa-conversations-navigation-layout-evidence-h6gauvkv. Live QA remains
parked 3/10; all advisories and D-125/M1/M2 remain. No publication.
