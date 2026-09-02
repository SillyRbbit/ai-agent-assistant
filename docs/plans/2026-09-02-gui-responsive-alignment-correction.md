# GUI responsive alignment correction

Status: Verified complete with advisories
Owner: Codex
Last updated: 2026-09-02

## Goal

Correct the owner-reported sidebar spacing and Graph alignment defects visible
on a MacBook Pro window and a 5120x1440 ultrawide display without reopening the
completed GUI redesign or changing protected Structured behavior.

## User-visible outcome

Workspace navigation follows the conversation history at a stable compact
distance while only the local-first footer remains bottom-pinned. Graph domain
containers have consistent gutters, their labels remain above relationship
lines, and a clear routing lane separates AgentOrchestrator from the domain
headers in compact, workspace, dense, and ultrawide layouts.

## Scope

- Correct expanded-sidebar flex allocation and footer anchoring.
- Correct Graph vertical rank spacing, domain-container gutters, and node/edge
  paint order.
- Preserve container-CSS-pixel measurement and recompute responsive positions
  from the existing `ResizeObserver` path.
- Add focused layout-contract tests and manually verify the owner-reported
  screen classes.

## Explicit non-goals

- No Structured-view source, behavior, data, labels, layout, or controls.
- No canonical-agent, scenario, task, event, selection, or provenance changes.
- No visual-system rewrite, route change, graph-library replacement, or new
  state framework.
- No dependency, Rust, Tauri, IPC, persistence, model, provider, tool,
  permission, approval, audit, or device-authority change.
- No branch, commit, push, merge, release, or publication work.

## Existing behavior and constraints

- The completed predecessor increment is valid with `PASS WITH ADVISORIES` and
  its 39-path uncommitted workspace must be preserved.
- The expanded conversation region currently consumes all spare sidebar height,
  pushing Workspace navigation down as the viewport grows.
- Domain lanes extend 14 CSS pixels beyond their member nodes while current
  group gaps are only 24 pixels, causing adjacent dashed containers to overlap.
- In workspace mode, the orchestrator bottom and domain-lane top share the same
  vertical coordinate, so smooth-step routing crosses the header-label band.
- React Flow geometry is already measured from the actual canvas in CSS pixels;
  screenshot DPI must not be converted into layout coordinates.

## Current-state evidence

- Owner screenshots reproduce an expanding blank sidebar band at approximately
  1678x1038 and 5120x1440 display sizes.
- The Mac screenshot shows the orchestrator relationship line through the
  Engineering header and overlapping/touching domain borders.
- Focused baseline tests pass: 2 files and 58 tests.
- The predecessor completion marker was valid before this separately
  owner-requested correction began.

## Files expected to change

- `src/styles.css`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.test.tsx`
- Current plan, increment, troubleshooting, status, handoff, changelog, next
  steps, review, and completion-gate records as evidence requires

Protected from modification:
`src/features/command-center/components/TopologyStructuredView.tsx`,
`src-tauri/`, `package.json`, and `package-lock.json`.

## Affected components

- Expanded application sidebar
- React Flow responsive position builders
- Graph domain-lane presentation and fit calculations

## Interfaces and invariants

- The sidebar and Graph remain presentation-only.
- The Graph uses actual canvas CSS-pixel bounds, not screen pixels or
  `devicePixelRatio`.
- Exactly nine configuration-derived canonical agents remain outside
  Structured; AgentOrchestrator remains a separate entity.
- Automatic fit, manual viewport preservation, keyboard navigation, selection,
  and truthful fixture disclosure remain unchanged.
- Root/body retain one bounded viewport and existing scroll ownership.

## Implementation milestones

- [x] Reproduce the screenshot symptoms and pass the focused baseline
- [x] Bound conversation-history growth and bottom-pin only the footer
- [x] Add Graph routing clearance, non-overlapping domain gutters, and label-safe layering
- [x] Pass focused and full automated checks plus Mac/ultrawide rendered QA
- [x] Synchronize documentation and complete the post-increment gate

## Security and privacy considerations

No new trust boundary is introduced. Do not add screenshot files, personal
paths, raw prompts, model content, credentials, logs, or external data to the
repository. Existing mock and synthetic provenance remains explicit.

## Test plan

- Extend topology tests to require positive domain-lane gutters and vertical
  clearance below AgentOrchestrator in workspace, dense, and wide modes.
- Preserve existing fit, resize, manual viewport, keyboard, and graph-state
  tests.
- Manually verify expanded sidebar order and Graph label/edge clearance at
  1280x720, a Mac-class 1678x1038 window, and 5120x1440 ultrawide size.
- Resize repeatedly across layout thresholds and verify no stale positions,
  clipped labels, React warnings, or document-level scrolling.

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
shasum -a 256 src/features/command-center/components/TopologyStructuredView.tsx
git diff --exit-code -- src/features/command-center/components/TopologyStructuredView.tsx
git diff --exit-code -- src-tauri package.json package-lock.json
```

## Risks

- Added vertical clearance can reduce fit zoom in short activity-expanded
  canvases.
- Increased horizontal gutters can push the ultrawide mode below its readable
  fit threshold if spacing is excessive.
- Sidebar caps must retain independently reachable long conversation and
  primary-navigation lists at short heights.
- React Flow z-index changes must keep relationships visible while protecting
  label text.

## Rollback or failure strategy

Revert only the bounded CSS constants, position calculations, and focused tests.
If the owner-reported sizes cannot be corrected without harming short-height
fit or Structured, stop and record a failed gate rather than weaken either
contract.

## Decisions made

- Implementation-start readiness was **Ready with advisories** because the
  owner explicitly approved this bounded presentation correction, the
  predecessor gate was valid, the focused baseline passed, and no blocked
  product/operational boundary was in scope. Final successor readiness remains
  `Blocked` under D-111.
- Treat dimensions as container CSS pixels and let the existing observer drive
  recomputation across displays; do not scale geometry from image DPI.
- Move spare sidebar height below Workspace navigation, not inside a one-item
  conversation list.

## Discoveries

- The screenshot defects are deterministic spacing and stacking defects, not a
  missing dependency, DPR conversion problem, or stale canonical data.

## Progress

- 2026-09-02: Inspected the three owner screenshots at original resolution,
  identified the flex-growth, vertical-rank, group-gutter, and stacking causes,
  passed the focused 58-test baseline, and began gate
  `gui-responsive-alignment-correction`.
- 2026-09-02: Added failing geometry regressions, corrected sidebar flow,
  balanced wide geometry, domain-lane clearances, paint order, and fit-derived
  dense layout selection. Review then exposed and closed cross-row lane overlap
  and the former 280-to-281-pixel fit discontinuity.
- 2026-09-02: Passed 369 frontend tests, formatting, lint, type-check, frontend
  build, documentation, repository, secret, diff, and protected-path checks.
  Rendered browser QA passed at 1280x720, 1678x1038, 760x520, and an
  ultrawide-class 4096x1440 controller limit; an exact 5120-pixel canvas
  geometry regression separately passed.
- 2026-09-02: The consolidated `npm run verify` additionally passed hook and
  repository tests, frontend and Rust formatting/lint/tests, frontend builds,
  and the Tauri no-bundle release build.

## Acceptance criteria

- [x] Workspace navigation follows conversation history by a compact tokenized gap at tall sizes
- [x] Only the local-first footer consumes remaining sidebar space
- [x] Domain-lane borders have at least 8 CSS pixels of horizontal clearance
- [x] At least 32 CSS pixels separate AgentOrchestrator from the nearest domain lane
- [x] No relationship line covers a domain label
- [x] Long agent labels remain fully readable and node baselines remain aligned
- [x] Automatic/manual fit behavior remains stable across repeated resize
- [x] Requested Mac and ultrawide screen classes pass rendered verification
- [x] Structured remains byte-identical and protected paths remain unchanged

## Final results

Completed with `PASS WITH ADVISORIES`. The conversation region no longer grows
into the sidebar's spare height; the local-first footer remains bottom-pinned.
Compact, workspace, dense, and wide Graph arrangements now keep at least eight
world-space CSS pixels between domain lanes and at least 32 between
AgentOrchestrator and the first lane. Semantic nodes paint above group lanes,
wide displays use a balanced two-row domain arrangement at up to 150% automatic
fit, and short canvases select the dense arrangement from measured fit instead
of a fixed height cliff.

Rendered CSS-pixel viewport checks measured a 12-pixel conversation-to-
Workspace gap, 12-pixel footer inset, zero document overflow, fully fitting
agent labels, 42-pixel orchestrator clearance, and eight-pixel lane clearance at
1678x1038. At the browser controller's 4096x1440 ultrawide limit, automatic fit
was 150%, the balanced topology aspect was 2.63, labels fit, and scaled
clearances were 63 and 12 screen pixels. The exact 5120-pixel canvas path passed
the automated geometry contract. This is display-sized browser evidence, not a
claim of a post-fix run on the owner's physical displays or native Tauri/DPR
hardware.

The full `npm run verify` completion gate passed, including the 22-file,
369-test frontend suite and the Tauri no-bundle release build. Structured
remains byte-identical at SHA-256
`ab939aa56d5b5c66fb39ce201ef40aa53422c4112015b9035b418c43dec06c0a`;
Rust/Tauri and dependency paths have no diff. Nonblocking advisories remain for
an automated real-browser responsive harness and the size of the Graph
composition. D-111 keeps next-increment readiness `Blocked`.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md` reviewed; no durable decision changed
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] `PLANS.md`
- [x] `docs/increments/gui-responsive-alignment-correction.md`
- [x] Consolidated post-increment review
