# Knowledge navigation layout

Increment: knowledge-navigation-layout
Status: Implementation and native QA passed; final gate receipts authoritative

## Authority, baseline and preservation

Owner authorizes exactly eleven successor paths in the existing knowledge-documents
worktree, branch codex/knowledge-documents, HEAD e02df763ac2522198c8b9c3169dd51fbf9956cc7.
Live main verified at 02dc35e62ac083a49f0802e8d340c39500f45e78. The clean published
48-path candidate, complete/valid knowledge-inspector-layout report, full Stop,
prior raw state and bundle were verified and frozen externally before ordinary
admission. Evidence: /private/tmp/cortexa-knowledge-navigation-layout-evidence.
Historical records are immutable; current memory updates are additive.

## Goal, evidence and non-goals

At <=960px the generic shell reserves 68px while expanded navigation is 220px;
the native 960x1410 receipt observes persistent occlusion. Existing browser routing
collapsed navigation before assertions. Reserve actual navigation width only on
Knowledge/Collaboration and test the previously omitted expanded resize path.
No automatic collapse, timers, remounts, dependencies, graph changes, Rust/IPC,
storage, execution, provider or permission changes. Preserve inspector placement,
wide docking, focus/Escape and all profiles, rooms and synthetic data.

## Exact successor paths

- `src/styles.css`
- `scripts/browser/knowledge-check.mjs`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-01-knowledge-navigation-layout.md`
- `docs/reviews/2026-10-01-knowledge-navigation-layout-post-increment-review.md`

## Validation and risks

Frontend tier: affected App/Knowledge/Collaboration tests, strict formatting,
ESLint/typecheck and frontend output via one offline isolated unsigned bundle.
Actual-App browser geometry checks resize across 961/960/959px and 760px in both
directions, expanded/collapsed navigation, inspector open/closed, routes, scrolling,
disclosure, focus and controls. Preserve graph docking assertions. No workflow start.
Direct native Computer Use must inspect both screens at wide/compact widths,
verify exact bundle/process identity, preserve data, and quit. Automated checks do
not substitute for native observations. Reuse unchanged native/full/audit evidence
truthfully; do not rerun unchanged suites. Documentation/repository/security,
whitespace, scope/preservation, session/review/schema and full Stop required.
Risk: reduced content width when navigation remains expanded. Check minimum
supported 760x520 and allow ordinary scrolling, not hiding controls or disclosure.
Stop on conflicting drift, admission rejection, scope expansion, unsupported
native access or unresolved security failure. No installs, provider calls or Git
publication. Retain OpenAI parked4/5, D-127/D-128, live/runtime/Codex isolation
advisories, process-local native workaround and parked D-125/M1/M2.

## Checkpoint

Ordinary admission passed after byte-identical predecessor archive and artifact
preservation. Eleven-path scope frozen. Implementation/validation next.

## Implementation and native checkpoint

Scoped CSS correction and actual-shell regression implemented. The old CSS failed
the new overlap assertion; corrected expanded/collapsed resize matrix passed.
Focused tests 65/65, strict lint after one unused-global correction, typecheck and
one offline isolated bundle passed. Direct native 2560x1410 and 960x1410 checks
passed on both screens; exact process quit. Minimum 760x520 remains browser-only;
no new native transmission preview, workflow, save, export or provider request.
Final documentation, preservation, reviews and ordinary completion gates next.

## Final review checkpoint

Architecture/security/code-health/readiness review found no new blocker. All
required frontend and initial governance checks passed. The harness focus race
was corrected by observing existing requestAnimationFrame focus restoration,
without changing application behavior. Historical failures remain in executed.json.
Exact eleven paths, prior raw state, unrelated bytes and old/new artifacts verified.
Final documentation/schema/completion/full Stop receipts determine formal acceptance.
