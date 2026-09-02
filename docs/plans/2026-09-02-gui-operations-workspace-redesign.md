# GUI operations workspace redesign

Status: Verified complete with advisories
Owner: Codex
Last updated: 2026-09-02

## Goal

Redesign the existing React presentation layer into a responsive desktop AI
operations workspace while preserving Cortexa's local-first authority
boundaries, existing routes, deterministic data provenance, Tauri IPC surface,
and protected Structured view.

## User-visible outcome

The application uses a compact global header, collapsible navigation, a
full-height central workspace, a collapsible inspector, and a bounded activity
dock. The Command Center prioritizes truthful status and active work, Graph mode
uses the available canvas, and selections connect existing surfaces without
presenting mock or synthetic evidence as live runtime state.

## Scope

- Implement the six owner-provided prompts sequentially as one bounded frontend
  increment with an acceptance and validation checkpoint after each phase.
- Extend the existing React reducer/context, Lucide, and `@xyflow/react`
  architecture rather than introducing replacement systems.
- Add focused automated coverage and rendered responsive evidence for the
  redesigned shell, Command Center, Graph, inspector, and activity dock.
- Synchronize repository memory only after implementation evidence is final.

## Explicit non-goals

- No Structured-view internal data, nodes, count, grouping, semantics, layout,
  labels, filters, controls, edge behavior, rendering, or interaction changes.
- No Rust redesign, new Tauri command, model networking, SQLite, persistence,
  execution authority, generic runtime selection, provider integration, or live
  telemetry.
- No new route or unsupported product capability solely to fill navigation.
- No graph, state-management, icon, or design-system dependency replacement.
- No branch, commit, push, merge, release, or publication work.

## Existing behavior and constraints

- Eight closed frontend routes are owned by the application reducer/context.
- `get_app_info` and five sealed synthetic native-demo commands are the only
  current Tauri presentation surface and remain unchanged.
- Command Center is a deterministic frontend projection with one separate
  orchestrator and exactly nine canonical agent roles; its optional native
  Research -> Knowledge proof is sealed, synthetic, and content-free.
- Runtime instances, tasks, workflows, tool executions, approvals, knowledge or
  memory records, and activity events are distinct concepts and may not be
  collapsed into a generic topology entity.

## Current-state evidence

- The root shell already uses a viewport-height boundary and minimum-size
  plumbing, but Command Center is vertically composed inside the route scroller.
- At 1280x720, measured Command Center content is approximately 2558px tall and
  the Graph canvas begins near y=913. At 1920x1080, content remains approximately
  1645px tall and the Graph begins near y=809.
- The Graph adapter already measures with `ResizeObserver`, fits initialized
  content, supports keyboard navigation, centers selected context, and preserves
  manual pan/zoom intent.
- Frontend canonical-agent metadata is repeated across fixture, projection, and
  test surfaces even though Rust owns the immutable canonical definitions.

## Files expected to change

- `src/App.tsx`
- `src/styles.css`
- `src/application/navigation.ts`
- `src/application/state.ts` and focused tests if shell selection is application-owned
- `src/components/ApplicationSidebar.tsx`
- Small new presentation-only shell components under `src/components/`
- Command Center page, presentation components, feature CSS, state/view models,
  and focused tests under `src/features/command-center/`
- Repository plan, increment, review, handoff, status, next-step, decision, and
  changelog documents as evidence requires

Protected from modification: `TopologyStructuredView.tsx`, Rust/Tauri source,
`package.json`, and `package-lock.json`, unless a verified regression makes an
otherwise-forbidden change necessary and the owner prompt permits it.

## Affected components

- Application shell and route viewport
- Navigation and global header
- Command Center overview
- React Flow topology adapter
- Contextual inspector and activity stream
- Frontend state/view-model layer
- Accessibility and responsive behavior

## Interfaces and invariants

- The WebView remains presentation-only and untrusted.
- Selection state carries a discriminated entity kind and provenance.
- Exactly nine unique canonical agent definitions appear on canonical-agent
  surfaces; the orchestrator, tasks, and synthetic proof roles are not agents.
- Fixture and synthetic data retain explicit disclosure at every affected
  status, count, timestamp, and activity surface.
- Manual Graph viewport intent survives shell-panel geometry changes; explicit
  Fit or Reset returns to automatic framing.
- Root/body do not scroll; each region has one intentional scroll owner.
- Existing route focus, native menu routing, conversation behavior, approvals,
  graph keyboard behavior, Structured view, and `get_app_info` continue to work.

## Implementation milestones

- [x] Prompt 1: read-only repository audit and redesign plan
- [x] Prompt 2: shell, responsive layout, navigation, and design tokens
- [x] Prompt 3: Command Center operational hierarchy and canonical roster
- [x] Prompt 4: full-workspace Graph presentation and controls
- [x] Prompt 5: typed selection, inspector, and activity workspace
- [x] Prompt 6: integrated visual, accessibility, and regression QA

## Security and privacy considerations

- Do not render raw prompt, reasoning, tool arguments/results, credentials,
  sensitive paths, memory content, or approval subjects.
- Do not imply that deterministic frontend fixtures or sealed synthetic native
  evidence are current device/runtime telemetry.
- Do not move validation, policy, approval, execution, or audit authority into
  React, topology nodes, selectors, or shell controls.

## Test plan

- Focused unit coverage for shell state, typed selection, canonical roster
  uniqueness, provenance, graph semantic styling, and panel bounds.
- Interaction coverage for route focus, collapse/expand, keyboard resizing,
  Graph selection/controls, inspector Escape/focus return, and activity rows.
- Existing projection, Structured-view, conversation, native-menu, approval,
  and `get_app_info` regression suites after every relevant phase.
- Rendered checks at 1280x720, 1366x768, 1440x900, 1920x1080, and 2560x1440,
  plus native minimum sizes, panel combinations, browser zoom, reduced motion,
  keyboard-only use, and available Tauri smoke checks.

## Verification commands

```bash
npm run format:frontend
npm run lint:frontend
npm run typecheck
npm run test:frontend
npm run build:frontend
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
```

Run `npm run verify` if implementation crosses a dependency, Tauri,
configuration, security-sensitive, or other full-gate boundary.

## Risks

- Broad theme or layout selectors could alter the protected Structured view.
- Shell overflow changes could break transcript/composer reachability or create
  nested scrolling.
- Panel resizing could erase Graph manual viewport intent or collapse the canvas.
- Combining fixture, session-mock, and synthetic-demo activity could overstate
  provenance or invent runtime state.
- A responsive breakpoint based only on window width could ignore the space
  consumed by navigation, inspector, and activity regions.

## Rollback or failure strategy

Keep each prompt as a small, buildable checkpoint. If a phase cannot preserve
its stated invariants, stop that phase, retain prior passing work, record the
failing evidence, and use the repository gate's terminal-failed path rather
than weakening tests, Structured view, security policy, or authority boundaries.

## Decisions made

- Treat the six prompts as one owner-approved frontend increment with six
  sequential internal phases; this avoids beginning an unauthorized successor
  while honoring the requested implementation order.
- Preserve the current closed route set. Graph remains a Command Center mode.
- Use application-owned volatile UI state only for shell mechanics and typed
  presentation selection; do not create domain or execution state in React.
- Preserve the existing Graph adapter's automatic/manual viewport contract.

## Discoveries

- The historical shared max-width Graph defect is already fixed; current wasted
  space is caused by vertical page composition and fixed/clamped panel heights.
- Common desktop widths cross the current 1100px Command Center container rule
  after navigation and page padding, causing the inspector to stack.
- Structured view is a complete accessible non-canvas alternative and must
  remain byte-for-byte unchanged if possible.
- Integrated Prompt 6 testing exposed a zero-height Graph chain at effective
  125% and 150% scaling when the inspector and activity dock were both open.
  The same constrained-height state could place the Graph Filters popover
  beneath the activity dock.
- The former fixed 1280px canvas breakpoint changed from the workspace pack at
  100% zoom to the wider source-coordinate layout at 64% zoom across a
  one-pixel resize. Wide layout therefore has to depend on measured topology
  bounds and readable fit, not window or canvas width alone.
- The Graph's pointer-selectable relationships needed a keyboard-equivalent
  selection surface. That surface also needs an independently bounded scroll
  region because the Graph panel intentionally clips canvas overflow.
- Cold lazy loading could suppress the shell inspector and activity fallbacks
  before Command Center portals mounted, and hoisting the native proof fragment
  outside its exact JSX conditional defeated the repository's static F-12
  lifecycle-boundary proof despite preserving runtime behavior.

## Progress

- 2026-09-02: Completed the read-only audit, baseline frontend lint, 313-test
  frontend suite, rendered current-state measurements, and implementation plan.
- 2026-09-02: Opened repository gate `gui-operations-workspace-redesign` from a
  clean `main` baseline at `ada57ce`.
- 2026-09-02: Completed Prompt 2 with a presentation-only shell reducer,
  compact header, branded collapsible navigation, bounded inspector and
  activity regions, explicit scroll ownership, route-scroll reset, default-dark
  tokens, focused accessibility coverage, and rendered checks from 760x520
  through 2560x1440. Formatting, lint, typecheck, 321-test full frontend suite,
  focused 39-test rerun, frontend build, and diff checks passed.
- 2026-09-02: Completed Prompt 3 with a typed overview derived from the
  validated Command Center projection, a concise simulated system-status row,
  prioritized active work and attention queues, exactly nine canonical agent
  definitions, and deterministic recent activity. Runtime, provider, tool,
  health, stage, cost, and elapsed-time gaps remain explicitly unavailable;
  no replacement data source was introduced. Formatting, lint, typecheck,
  336-test full frontend suite, focused 28-test rerun, frontend build, rendered
  1280x720 and 1920x1080 checks, and diff checks passed.
- 2026-09-02: Completed Prompt 4 with a full-height React Flow workspace,
  measured responsive layouts, automatic fit and explicit manual-viewport
  preservation, accessible viewport controls, relationship focus and legend,
  selected-context emphasis, bounded node text, and truthful graph-state
  recovery. Independent review found no blocking defect. Formatting, lint,
  typecheck, the 347-test full frontend suite, a focused 34-test rerun,
  frontend build, rendered 1280x720 and 1920x1080 panel checks, and diff checks
  passed. `TopologyStructuredView.tsx` remained byte-identical at SHA-256
  `ab939aa56d5b5c66fb39ce201ef40aa53422c4112015b9035b418c43dec06c0a`.
  Direct coverage for raw no-data/measuring states, a one-match search, pointer
  pan, and keyboard-operable relationship selection was deferred to the
  integrated Prompt 6 accessibility pass and is now complete.
- 2026-09-02: Completed Prompt 5 with one feature-local, scenario-bound typed
  selection for topology entities and fixture events; shell-owned inspector and
  activity outlets; truthful type-specific inspector sections; a selectable
  deterministic activity timeline; panel resizing and collapse behavior; and
  modal-aware Escape/focus restoration. A review finding about inferred event
  source and target semantics was corrected through one shared presentation
  mapper: source, target, and status remain explicitly unavailable while
  agent, task, workflow, and related-entity associations retain their exact
  fixture labels. Graph Recent Activity selects the typed event, while
  Structured retains its prior topology-context interaction. Independent
  review found no remaining blocker. Formatting, lint, typecheck, the 361-test
  full frontend suite, a focused 97-test rerun, frontend build, rendered
  1280x720 panel and focus checks, and diff checks passed. The protected
  Structured source hash remains
  `ab939aa56d5b5c66fb39ce201ef40aa53422c4112015b9035b418c43dec06c0a`.
- 2026-09-02: Prompt 6 reproduced and corrected the constrained-height Graph
  collapse, hidden filter content, 1279-to-1280 layout discontinuity, pointer-
  only relationship inspection, composite focus mismatch, cold-loading shell
  outlet gap, lifecycle static-boundary regression, compact-control tooltip
  gap, and inconsistent event terminology. The 367-test frontend suite,
  repository health, security scan, responsive browser evidence, and the
  protected Structured hash pass. `npm run verify` also passed, including
  formatting, repository health, strict frontend and Rust lint, all Python,
  frontend, and Rust tests, frontend builds, and the Tauri no-bundle release
  build. Independent architecture, security, code-health, technical-debt, and
  readiness reviews found no completion blocker.

## Acceptance criteria

- [x] All five implementation/QA prompts complete sequentially and remain buildable
- [x] Main workspace fills remaining viewport without document-level double scroll
- [x] Exactly nine canonical agents appear outside protected Structured view
- [x] Mock and synthetic data remain visibly and accurately disclosed
- [x] Graph fits/resizes/readjusts without label overlap or lost manual intent
- [x] Inspector and activity panels are accessible, bounded, and truthful
- [x] Requested viewport matrix and all existing routes pass rendered review
- [x] Structured view and `get_app_info` remain functionally unchanged
- [x] Required automated and repository completion gates pass

## Final results

All six prompts are complete. The redesign remains presentation-only and uses
the existing application reducer/context, Lucide icons, and React Flow library.
The final manual matrix passed at 1280x720, 1366x768, 1440x900, 1920x1080, and
2560x1440, plus effective 1024x576 and 853x480 scaling proxies. All panel
combinations, rapid resize, route changes with panels open, graph automatic and
manual viewport behavior, keyboard relationship selection, focus/Escape,
popover reachability, empty/loading/error states, and Structured mode passed.
The full quality result is `PASS WITH ADVISORIES`; see
[`2026-09-02-gui-operations-workspace-redesign-post-increment-review.md`](../reviews/2026-09-02-gui-operations-workspace-redesign-post-increment-review.md).

Residual advisories are a missing automated real-browser responsive harness,
large Command Center composition files, and the open legend's mildly stale
“Show relationship legend” wording. They do not block completion. Repository
roadmap readiness remains Blocked independently because D-111 admits no
successor.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] `PLANS.md`
- [x] `docs/increments/gui-operations-workspace-redesign.md`
- [x] `docs/reviews/2026-09-02-gui-operations-workspace-redesign-post-increment-review.md`
- [x] `DECISIONS.md` not required; no durable architecture, security, product,
      or authority decision changed.
