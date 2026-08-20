# Command Center Full-Screen Layout and Graph Viewport Audit

**Status:** Read-only diagnostic audit; no implementation authorized by this record
**Date:** 2026-08-20
**Scope:** Command Center route width and Operational Topology graph viewport only

## Audit boundary and evidence

This audit is based on the current React, CSS, fixture, React Flow, test, and Tauri configuration sources. It does not modify application code, dependencies, lockfiles, fixtures, or native configuration.

Rendered browser and Tauri evidence is still pending. The repository has no Playwright, Cypress, WebDriver, or `tauri-driver` setup, and the approved in-app browser runtime required for local visual testing was unavailable during this audit. Numeric viewport examples below are therefore source-derived layout estimates, not claimed screenshots or measured browser results. The test plan identifies the rendered checks still required before accepting a correction.

The Structured topology view is explicitly excluded. This audit does not analyze or recommend changes to its behavior, data, grouping, counts, tables, labels, accessibility model, or alternative-view contract.

## Executive finding

The two visible defects share a width mismatch:

1. The application shell and main content column already fill the window, but the Command Center inherits the shared `.page-stack` maximum width of 1,680px and centered margins. Above an approximately 1,932px window width, the route stops growing even though the region to the right of the 252px navigation continues to grow.
2. The graph's deterministic coordinates place nine agent nodes in one horizontal row. Including the rendered group-lane padding, the visible topology is approximately 1,794px wide. The actual graph canvas can be only about 443px wide at the default 1,040px Tauri window after page padding and the inspector column.
3. React Flow is asked to fit with proportional padding but is constrained to `minZoom={0.35}`. A 1,794px graph at 0.35 zoom is still about 628px wide, so Fit View cannot fit it into the estimated 443px canvas. On wide windows the inverse occurs: the inherited page cap keeps the canvas around 1,212px and the topology is fit to a visually small scale while substantial window width remains unused.
4. React Flow observes canvas size changes, but application-owned fit logic does not rerun on window, shell, or workbench resize. Reset restores a cached initial viewport, so maximize, fullscreen, restore, and responsive layout changes can leave a stale scale or offset.

The smallest maintainable correction is Command-Center-specific: let this route use the available main-column width, adopt a more balanced deterministic two-dimensional graph layout, and centralize measured-container fit/reset/resize behavior. The shared page-width contract for other routes should remain unchanged.

## 1. Root cause: full-screen width

### Source chain

- `.application-shell` already uses a sidebar track plus `minmax(0, 1fr)` and fills the application viewport.
- `.application-main` and `.application-content` already use the available main-column width.
- `CommandCenterPage` renders as `section.page-stack.command-center-page`.
- The shared `.page-stack` rule applies:
  - `width: 100%`
  - `max-width: var(--app-content-max-width)` where the token is `1680px`
  - `margin: 0 auto`
  - horizontal padding up to `56px` per side
- There is no Command Center route-specific width override.

The unused right-side area is therefore not caused by the shell grid, sidebar width, Tauri window width, or React Flow. It is caused by the centered shared page cap inside an otherwise full-width main region.

### Source-derived width examples

The following estimates use the current sidebar, shared padding, workbench gap, and inspector rules. Browser rounding and scrollbar width may change individual pixels.

| Window        | Main region | Command Center page |             Estimated graph canvas | Consequence                                                                  |
| ------------- | ----------: | ------------------: | ---------------------------------: | ---------------------------------------------------------------------------- |
| 760 × 520     |       566px |               566px |               about 502px, stacked | Minimum Tauri width; fit is still below the graph's effective minimum width. |
| 1,040 × 520   |       826px |               826px |               about 739px, stacked | Short-height rule provides more graph width.                                 |
| 1,040 × 700   |       826px |               826px | about 443px beside 280px inspector | Default Tauri window; graph cannot fit at 0.35.                              |
| 1,600 × 1,000 |     1,348px |             1,348px | about 880px beside 340px inspector | Full main width participates, but topology is still compressed.              |
| 2,560 × 1,440 |     2,308px |   capped at 1,680px |                      about 1,212px | About 628px of main-region width is unused in total.                         |
| 3,440 × 1,440 |     3,188px |   capped at 1,680px |                      about 1,212px | About 1,508px is unused in total; graph width no longer grows.               |

The approved Command Center proposal previously said that content at and above 1,600px remains max-width constrained. The newer owner direction for this audit requires the usable region to the right of navigation to participate and forbids leaving most of a large viewport unused. Before implementation, the proposal, active ExecPlan, and D-092 record must explicitly reconcile that changed route-specific width expectation. This audit does not itself authorize that change.

## 2. Root cause: graph initial scale

The initial graph scale is a consequence of content geometry and available canvas geometry, not a font-loading race:

- The nine agent nodes use fixed x coordinates from approximately 60 through 1,660 and share one y row.
- A topology node is 166px wide in the adapter's deterministic geometry.
- Group-lane measurement adds 14px padding around member bounds.
- The resulting visible horizontal bounds are approximately x=46 through x=1,840, or 1,794px wide.
- The default idle topology is roughly seven times wider than it is tall. The longest scenario remains strongly width-dominated.
- React Flow queues fit work until nodes are measured. The route uses system fonts, so an early web-font measurement race is not the primary cause.
- Filtering removes hidden nodes before the adapter computes lanes and graph nodes, so filtered-out nodes are not inflating the live bounds.

Approximate initial fit scales under the current layout are:

| Window        | Estimated canvas | Approximate raw fit zoom | Current effective result                                                          |
| ------------- | ---------------: | -----------------------: | --------------------------------------------------------------------------------- |
| 2,560 × 1,440 |          1,212px |                     0.57 | Fits, but is unnecessarily small because the route cap withholds available width. |
| 1,600 × 1,000 |            880px |                     0.42 | Fits, with small node text.                                                       |
| 1,040 × 700   |            443px |                     0.21 | Clamped to 0.35, so the graph is wider than its viewport.                         |
| 760 × 520     |            502px |               below 0.35 | Clamped to 0.35, so a complete fit remains impossible.                            |

Reducing `minZoom` alone is not an adequate correction. It would technically expose more bounds at standard widths while making labels and status text less readable. The deterministic graph coordinates should instead be compacted into a balanced two-dimensional hierarchy so the supported minimum canvas can fit at a readable scale.

## 3. Root cause: zoom, fit, reset, and resize

### Current behavior

- Initial layout calls `fitView({ duration: 0, padding: 0.18 })` when scenario, layout key, or flow-node count changes.
- Fit View calls the same measured fit routine on demand.
- `minZoom` is explicitly 0.35. `maxZoom` is not explicit and currently inherits the library default of 2.
- Ordinary wheel scrolling passes through the canvas; pan-on-scroll and zoom-on-scroll are disabled. Pinch and supported modifier gestures remain library-owned.
- React Flow observes its element size and updates internal viewport dimensions, but the application fit effect does not depend on measured canvas size.
- Reset restores an initial x/y/zoom snapshot captured after the first fit. It does not recompute the correct default for the current canvas.
- Center Selected uses nominal node dimensions or edge endpoint averages and preserves the current zoom. It does not fit a measured selected group or bounded context.
- No finite translation extent prevents the entire graph from being panned far beyond the useful viewport.

### Failure modes

1. A graph opened at the default 1,040px window may start at the 0.35 floor. Maximizing expands the canvas but preserves that old transform, leaving a small graph in a large canvas.
2. A graph fit while maximized may be restored into a smaller canvas and become clipped or offset.
3. Reset after either transition restores the stale mount-time snapshot rather than the correct current fit.
4. At small supported canvases, Fit View cannot recover the whole graph because the minimum zoom is higher than the required fit zoom.
5. The responsive rules use total viewport width or height rather than the workbench's measured available width. Sharp threshold crossings can reduce graph width instead of increasing it:
   - approximately 630px to 315px across 900→901px;
   - approximately 516px to 457px across 1,120→1,121px;
   - approximately 739px to 443px across a 1,040px-wide window at height 610→611px.

The graph container already clips its canvas appropriately, and its flex/grid children already use the necessary `min-width: 0` and `min-height: 0`. Missing min-size rules are not the root cause here.

## 4. Current layout diagram

```text
Window viewport
┌──────────────┬───────────────────────────────────────────────────────────────┐
│ navigation   │ application main (correctly fills remaining width)           │
│ fixed track  │ ┌───────────────────────────────────────────────────────────┐ │
│              │ │ centered .page-stack, max 1680px                         │ │
│              │ │ ┌──────────────────────────────┬───────────────────────┐ │ │
│              │ │ │ graph canvas                 │ inspector             │ │ │
│              │ │ │                              │                       │ │ │
│              │ │ │  ~1794px one-row topology    │                       │ │ │
│              │ │ │  fit into capped/narrow box  │                       │ │ │
│              │ │ └──────────────────────────────┴───────────────────────┘ │ │
│              │ │ activity                                                  │ │
│              │ └───────────────────────────────────────────────────────────┘ │
│              │      unused margin grows             unused margin grows      │
└──────────────┴───────────────────────────────────────────────────────────────┘
```

At ultrawide sizes, only the outer unused margins grow. At the default Tauri size, the same workbench divides a small inner width between the graph and inspector, forcing the graph below the width needed at the current minimum zoom.

## 5. Proposed layout diagram

```text
Window viewport
┌──────────────┬───────────────────────────────────────────────────────────────┐
│ navigation   │ Command Center route uses the full available main width      │
│ unchanged    │ ┌──────────────────────────────────────┬────────────────────┐ │
│              │ │ measured graph viewport              │ inspector          │ │
│              │ │                                      │                    │ │
│              │ │ balanced deterministic 2D topology   │                    │ │
│              │ │ canonical measured fit + recovery    │                    │ │
│              │ └──────────────────────────────────────┴────────────────────┘ │
│              │ activity uses the same route width                            │
└──────────────┴───────────────────────────────────────────────────────────────┘

When measured workbench width is insufficient:
┌──────────────┐
│ graph        │  graph gets full row width and recomputes canonical fit
├──────────────┤
│ inspector    │  joins document flow without competing for canvas width
├──────────────┤
│ activity     │
└──────────────┘
```

The proposed width rule is scoped to the Command Center. Other routes keep the shared centered readable-width contract, and the Command Center heading/description retain their existing readable text measure.

## 6. Graph viewport contract

### Measured viewport ownership

- The React Flow wrapper owns graph pan and zoom state.
- A single application helper observes the graph container's actual content-box width and height, not only `window.innerWidth`.
- Workbench beside/stack decisions use the available feature container width, or thresholds derived from it, rather than total viewport width alone.
- Inspector relocation, shell resize, maximize, fullscreen, restore, and drag resize all reach the same measurement path.
- Activity expansion or resizing triggers graph work only if it changes the measured graph rectangle.

### Canonical graph bounds

- Automatic fit uses only currently visible semantic graph nodes plus currently visible presentation lanes.
- Hidden filter results do not contribute bounds.
- Edges follow their visible endpoints and do not independently expand fit bounds.
- The first implementation should replace the one-row agent coordinates with deterministic, bounded two-dimensional coordinates. It must preserve the existing graph data and semantics.
- Lane padding, focus ring, node shadow, and edge-marker clearance are included in the effective bounds.

### Initial fit and padding

- Initial fit waits for a nonzero measured canvas and measured visible nodes.
- It executes once for the settled scenario/layout generation and is coalesced if ResizeObserver reports multiple measurements.
- Fit uses a named pixel gutter rather than an unexplained percentage. A 24px starting value matches one existing canvas-grid unit and exceeds the current 14px lane padding; the final value must be confirmed in rendered testing.
- Automatic fit is capped at zoom 1 so a one-node filter does not enlarge a node beyond its designed scale.
- The compact layout should keep automatic full-graph fit at or above a provisional 0.65 readable target at supported native sizes. If rendered evidence disproves that target, change graph geometry or responsive composition before lowering it.

### Zoom range and pan bounds

- Declare both minimum and maximum zoom explicitly; a provisional interactive range of 0.5 through 1.5 limits unreadably small labels and excessive context loss while preserving useful inspection. The exact values require rendered validation.
- Apply a finite translation extent derived from current visible graph bounds plus a viewport-relative exploration gutter. Users may explore around the graph but cannot lose it indefinitely beyond empty space.
- Preserve ordinary page wheel/trackpad scrolling and current explicit graph zoom controls.

### Fit View

- Recompute from the current measured viewport and current visible graph bounds every time.
- Do not reuse a mount-time viewport.
- Fit the farthest left, right, top, and bottom visible content inside the named gutter.
- At supported sizes, Fit View must never be prevented from fitting by the interactive zoom floor; graph geometry is responsible for satisfying the readable-fit contract.

### Reset

- Reset means “return to the canonical fit for the current scenario, filters, container, and inspector composition.”
- Reset recomputes that fit. It must not restore a cached x/y/zoom captured for an earlier window size.

### Center Selected

- Node selection centers measured node bounds and a bounded amount of one-hop context at a capped readable zoom.
- Edge selection fits or centers both visible endpoint bounds.
- Group selection fits its visible member bounds and lane.
- Active-path centering uses the same measured-bounds helper.
- The inspector is outside the graph container, so no manual pixel offset for it is added; resizing the graph container is sufficient.

### Resize and user intent

- Before the user pans or zooms, the viewport is in automatic-fit mode. Canvas resize, maximize, fullscreen, restore, scenario change, filter change, or inspector beside/stack transition recomputes canonical fit.
- A user pan or zoom enters manual mode. Ordinary resizes preserve the user's viewport when it remains recoverable and clamp it only when the visible graph would become wholly inaccessible.
- Fit View or Reset explicitly returns to automatic-fit mode.
- Resize handling is animation-frame coalesced and guarded against ResizeObserver feedback loops.

### Inspector and activity behavior

- The current UI has no inspector open/close control; no new interaction is implied by this audit. Its existing beside/stack transition participates through measured container resize.
- If a future approved control changes inspector visibility, it must use the same resize contract.
- The activity panel remains below the workbench. Its own content scroll does not cause a graph refit; only a real change to the graph container rectangle does.

## 7. Expected files for a separately approved correction

### Primary implementation files

- `src/features/command-center/command-center.css`
  - route-specific full-width contract;
  - measured-width responsive composition;
  - graph viewport sizing without changing the shared shell.
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`
  - canonical measured fit/reset/center behavior;
  - resize and manual-intent state;
  - explicit zoom and translation bounds.
- `src/features/command-center/commandCenterFixtures.ts`
  - compact deterministic graph coordinates, if the correction retains coordinates in fixtures.
- `src/features/command-center/CommandCenterPage.test.tsx`
  - route-width and integration regressions.

### Possible bounded additions

- A focused graph-viewport helper and unit test under `src/features/command-center/` if extracting bounds and fit calculations materially improves determinism.
- `src/features/command-center/CommandCenterPage.tsx` only if a measured workbench ref or narrowly scoped composition state is needed.

### Documentation that must be reconciled before implementation

- `docs/design/NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md`
- `docs/plans/2026-08-12-native-multi-agent-command-center-prototype.md`
- the additive D-092 decision record in `DECISIONS.md`

### Not expected

- No removal of the global `.page-stack` cap from `src/styles.css`.
- No application-shell, navigation, Tauri, Rust, IPC, storage, policy, approval, runtime, package, or lockfile change.
- No Structured topology view file or behavior change.

## 8. Risks

1. Removing the shared cap globally would regress Conversations, Settings, and other readable pages. The width correction must be route-scoped.
2. Full-width chrome can create unreadably long prose. Preserve the existing bounded heading/description measure.
3. Automatic fit on every resize can fight intentional user pan/zoom. Track automatic versus manual viewport ownership.
4. ResizeObserver updates can loop or thrash. Coalesce work and avoid writing layout dimensions from the observer.
5. Lowering zoom without changing geometry can make labels inaccessible. Prefer compact geometry and measured composition.
6. An overly tight translation extent can block legitimate inspection; an infinite extent can let the graph be lost. Derive a bounded exploration gutter.
7. Changing deterministic coordinates can create edge crossings, lane overlap, or a mismatch between visual and keyboard traversal order. Verify all scenarios.
8. An asynchronous fit from an old scenario or filter can overwrite a newer viewport. Key and cancel pending fit work by layout generation.
9. `overflow-x: hidden` on the main content can conceal a width regression. Tests must compare scroll width with client width rather than relying only on appearance.
10. Inspector and activity scroll regions can obscure final content in reduced-height layouts if viewport geometry is not tested.
11. Any Structured topology change would violate this audit boundary.
12. Demo disclosure and all current architecture/security boundaries must remain visible and unchanged.

## 9. Test plan

### Deterministic automated coverage

1. Test graph-bound calculations for the default and deepest fixture scenarios.
2. Assert visible nodes and visible lanes are inside fitted bounds and filtered-out nodes are excluded.
3. Assert automatic fit respects the named pixel gutter, readable minimum target, explicit maximum, and zoom-1 enlargement cap.
4. Replace the no-op ResizeObserver test double with a controllable observer and assert multiple resize reports produce one coalesced fit.
5. Assert maximize-like growth and restore-like shrink recompute fit while in automatic mode.
6. Assert manual pan/zoom is not continuously overwritten by resize and remains recoverable.
7. Assert Reset recomputes from current dimensions rather than replaying an initial snapshot.
8. Assert Center Selected uses measured node, edge-endpoint, group, and active-path bounds.
9. Assert translation bounds prevent losing all visible graph content while retaining bounded exploration.
10. Preserve the existing wheel pass-through and shared application scrolling tests.
11. Assert the Command Center route alone is exempt from the shared content cap; other routes retain it.
12. Assert no horizontal document overflow at every responsive composition.

JSDOM can verify state transitions and calculated geometry but cannot prove actual rendered clipping, font readability, wheel/trackpad behavior, or Tauri window transitions.

### Real browser and Tauri matrix

The mandatory rendered matrix is still not run. Once approved tooling is available, test:

- 3,440 × 1,440 true ultrawide;
- 2,560 × 1,440 large desktop;
- 1,600 × 1,000 desktop;
- 1,440 × 900 and 1,366 × 768 standard desktop;
- 1,040 × 700 default Tauri window;
- 1,040 × 520 short window;
- 760 × 520 minimum Tauri window;
- 640 × 800 browser-only narrow layout.

For the installed Tauri application, exercise normal, maximize, fullscreen, restore, and drag-resize without restarting. Cross the current breakpoint boundaries at 900/901px, 1,120/1,121px, and height 610/611px to ensure available width never collapses unexpectedly.

At every size and transition:

1. Test the default and deepest fixture scenarios, all-node view, and a single-node filter.
2. Record graph and workbench bounding rectangles and confirm the route aligns with the usable main-region edges.
3. Confirm `scrollWidth <= clientWidth` for the document, application content, route, workbench, and graph wrapper.
4. Confirm initial fit and Fit View expose the farthest left, right, top, and bottom visible topology within the gutter.
5. Exercise pan, zoom in/out, Fit View, Reset, Center Selected, and scenario/filter transitions.
6. Exercise mouse wheel, trackpad, scrollbar, keyboard, pinch, and supported modifier-wheel behavior; ordinary vertical gestures must continue to scroll the page.
7. Confirm the graph remains recoverable after aggressive pan/zoom and every resize transition.
8. Confirm inspector and activity final controls remain reachable and no sticky or fixed element obscures them.
9. Capture comparison screenshots when approved visual tooling is available.
10. Re-run all existing route, shell, scroll, format, lint, typecheck, frontend test, and production-build checks required by the active plan.

## Conclusion

The full-screen defect is a Command-Center-specific inheritance problem, not a global shell failure. The graph defect is a deterministic geometry-to-viewport mismatch compounded by a zoom floor, stale reset snapshot, and missing container-resize fit policy. A future correction should change the route width and graph viewport contract together; changing only CSS width or only minimum zoom would leave a complementary failure mode in place.

No implementation, dependency, lockfile, Structured view, native, or configuration change is authorized or made by this audit.

## Subsequent authorized implementation checkpoint — 2026-08-20

After this read-only audit, the owner separately authorized its targeted UI
correction. The implementation keeps the shared route cap unchanged and exempts
only `.page-stack.command-center-page`; the feature workbench now responds to
its own available width. The graph adapter uses a measured `ResizeObserver`
path, compact two-dimensional presentation coordinates below a 1,280px canvas,
the existing wide coordinates above it, a 24px canonical fit gutter, explicit
0.5–1.5 interactive zoom, a zoom-1 automatic-fit cap, bounded translation
extent, current-state Fit/Reset, and bounded-context Center Selected. Ordinary
resizes preserve a manual viewport; scenario, filter, or topology-generation
changes re-enter automatic framing.

Focused viewport and page regression tests pass 18/18; the full frontend suite
passes 211/211 across 13 files, and formatting, strict lint, typecheck, and the
production build pass. The Structured topology implementation and fixture data
were not modified. The required rendered matrix remains Not run: the in-app
Browser runtime is unavailable, and although the local Vite server and existing
Tauri debug executable launched, macOS denied assistive access needed for
deterministic navigation, resize, and screenshots. Both processes were stopped.

## Subsequent authorized node-readability checkpoint — 2026-08-20

The follow-up keeps one explicit 180 x 88 semantic-node geometry contract in
the React Flow adapter and makes each visual card fill that exact box. Agent
titles now wrap within a two-line clamp while status/domain metadata remains in
a separate bottom grid row. Compact coordinates, group bounds, fit bounds, and
translation extents all use the same dimensions. Group heading copy was
compressed into its reserved 24px header band so it does not enter child cards;
edge labels retain their existing semantic background and accessible label.

Automated regression covers every authoritative agent name, full card labels,
node dimensions, all five group-lane child bounds, relationship label surfaces,
and existing Fit, Reset, Center Selected, wheel pass-through, inspector,
Structured, and demo-disclosure behavior. Focused graph/page tests pass 19/19;
the full frontend suite passes 211/211 across 13 files. Frontend formatting,
strict lint, typecheck, and production build pass. The required rendered matrix
and fullscreen screenshot remain Not run because the approved Browser runtime
is unavailable; no screenshot or visual claim is fabricated.
