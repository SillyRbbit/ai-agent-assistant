# Native multi-agent Command Center deterministic prototype

- Status: Complete — deterministic prototype and M5 matrix verified
- Owner: Project owner
- Prepared: 2026-08-19
- Design proposal: [`NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md`](../design/NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md)

## Authorization gate

This living ExecPlan records the approved deterministic prototype. Gate A and
the M0/M0.5 dependency authorization checkpoints are complete. The frontend
source is implemented, automated source validation passes, and the required
real-browser/Tauri viewport, input, focus, contrast, and screenshot matrix is
complete.

The owner supplied the required first authorization on 2026-08-19:

> The Command Center proposal and ExecPlan are approved. Proceed with the approved deterministic prototype milestone only.

Gate B M0 is complete. That sentence authorizes only current-state inspection,
exact dependency and transitive research, generated baseline evidence, and the
proposed dependency, lockfile, security, license, and bundle ledger. M0 stops
before package installation or source edits. Package installation and source edits may begin only
after the owner reviews the completed ledger and separately states:

> The M0 dependency and bundle ledger is approved. Install only the exact direct production dependencies recorded in that ledger and proceed with the approved deterministic prototype milestone.

The owner supplied that exact second authorization on 2026-08-20. It authorizes
only the approved deterministic frontend prototype, the two exact direct
production dependencies recorded in M0, and the verified file scope below.

On 2026-08-20 the owner separately authorized the targeted Command Center
layout and graph-viewport correction defined by
[`COMMAND_CENTER_LAYOUT_GRAPH_FIX_AUDIT.md`](../design/COMMAND_CENTER_LAYOUT_GRAPH_FIX_AUDIT.md).
That authorization supersedes only the proposal's earlier large-screen
max-width expectation: the Command Center route may use the complete available
main-column width while unrelated routes retain their shared readable-width
constraint. It also authorizes measured graph fit/reset/center/resize behavior
and focused tests inside the existing feature boundary. Structured topology
behavior, fixture semantics, dependencies, native/Tauri source, IPC, and later
milestones remain excluded.

On 2026-08-20 the owner also authorized the targeted Operational Graph node
readability correction. That correction is limited to one shared fixed node-size
contract, controlled two-line labels, deterministic lane containment, and
focused graph/page regressions. Agent names, grouping semantics, fixture data,
Structured topology, dependencies, and every native boundary remain excluded.
Before the first authorization sentence:

- do not edit frontend or Rust production code;
- do not add a route, component, fixture, style, dependency, or lockfile entry;
- do not install packages;
- do not implement or publish the prototype;
- do not commit, push, or open a pull request for this plan.

After the first sentence but before the second, every prohibition above remains
except the read-only M0 inspection and generated ignored baseline evidence
expressly listed in this section.

## Goal

Add one reversible, deterministic, fixture-only Command Center route that makes
the existing nine-role native architecture understandable without connecting
the React WebView to the unwired Rust multi-agent foundation or representing
simulated state as live.

The prototype will preserve the existing application shell and Conversations
experience while showing:

- application-owned `AgentOrchestrator` as a distinct authority node;
- Personal Assistant as a separate user-facing agent;
- all nine exact agent roles in five presentation groups;
- one small selected simulated task/workflow path;
- typed semantic edges, state dimensions, and fixture provenance;
- a contextual inspector and bounded structured activity stream;
- search, filters, selection, fit/reset/center controls;
- an equivalent accessible structured topology view;
- reliable scrolling and responsive behavior at the existing 760×520 minimum;
- a persistent `DEMO MODE · SIMULATED AGENT DATA` disclosure.

## User-visible outcome

The sidebar gains one Command Center destination. Opening it lazy-loads a
theme-adaptive premium operations workbench within the existing shell. Dark
mode uses graphite/midnight surfaces; light mode uses existing neutral tokens
with the same semantic hierarchy rather than an unexplained dark inset. The
user can inspect a fixed topology, switch deterministic scenarios, search,
filter, select entities, use a grouped tree plus relationship table, and review
fixture events. Nothing is executed, persisted, approved, scheduled,
sent across IPC, or connected to a provider.

All existing routes and conversation behavior remain available.

## Scope

The proposed one-increment prototype includes:

- incremental application-shell integration;
- one lazy, reversible Command Center route/view;
- persistent existing left navigation;
- a Command Center search header and scenario selector;
- compact fixture-derived status summary;
- central operational topology;
- visually and semantically distinct `AgentOrchestrator`;
- separate Personal Assistant node;
- all nine agents exactly once;
- five view-only domain groups;
- selected-node/edge state;
- contextual right inspector on wide layouts and stacked inspector on narrow
  layouts;
- bounded bottom activity stream;
- persistent demo-mode indicator;
- deterministic typed frontend fixtures and meaningful fixture edges;
- search, agent/domain/status/entity/workflow filters, a visibly unavailable
  capability filter, and reset;
- zoom in/out, fit-to-view, reset, center selected, and center active simulated
  task;
- selected-work-path drill-down only;
- accessible grouped agent tree plus relationship table alternative;
- keyboard operation and focus management;
- loading, empty/no-results, queued, active, waiting-for-approval, blocked,
  cancelled, failed, and completed/success states;
- reliable shell/page/panel scrolling and resized-window behavior;
- component, interaction, accessibility, projection, scrolling, and protected-
  boundary regression tests;
- a production frontend build and bundle/chunk comparison.

## Explicit non-goals

This increment does not include:

- real model networking or provider configuration;
- live agent execution, live runtime sessions, or autonomous behavior;
- Tauri agent/task/workflow/memory/approval/audit IPC;
- Rust agent, orchestrator, runtime, governance, memory, document, approval,
  audit, tool, or policy changes;
- real cloud, systems, repository, filesystem, document, or connector access;
- real tool execution or repository mutation;
- real workflows, approvals, policy decisions, security decisions, or audit;
- live MCP activity or MCP configuration;
- Hermes, `HermesAgentRuntime`, another external runtime, or provider fallback;
- persistence, SQLite, database migrations, restart recovery, or history;
- public deployment, signing, notarization, installer, or distribution work;
- multi-tenancy, billing, enterprise administration, or usage reporting;
- a dense knowledge graph;
- Sigma, Graphology, ECharts, xterm.js, Motion, Zustand, or cmdk;
- every possible node, edge, chart, filter, animation, or inspector mode;
- command execution from search or a global command palette;
- a new collapsible/icon-rail sidebar state;
- replacing Conversations or the current application shell;
- changing minimum window size, capabilities, permissions, CSP, or IPC;
- feature work outside the exact files and symbols revalidated at Gate B M0.

## Existing behavior and constraints

### Frontend baseline

- `src/application/navigation.ts` defines seven manual reducer-owned routes.
- `src/App.tsx` selects a page from a typed route record; there is no router
  package.
- `ApplicationStateProvider` and `applicationReducer` own volatile global UI
  state.
- `AppServices` injects only app-info, native menu-route, and deterministic mock
  driver boundaries.
- `ApplicationSidebar`, `PageHeader`, `PageState`, `.page-panel`, Activity list
  patterns, theme tokens, visible focus, and reduced-motion CSS are reusable.
- Conversations and Activity are mock-only. Tasks, Memory, and Integrations are
  placeholders. Permissions is static. Settings reads app info/menu status.
- `get_app_info` is the only custom Tauri command. No multi-agent DTO or
  consumer exists.

### Native domain baseline

- `AgentId::ALL` contains exactly Personal Assistant, Research, Coding, Cloud
  Infrastructure, Systems Operations, Knowledge & Document, QA & Validation,
  Security & Risk, and Workflow Automation.
- All definitions are implemented Rust catalog entries but are not live workers.
- `AgentOrchestrator` is separate application code and is not Personal
  Assistant or Workflow Automation.
- QA is advisory and not `ApprovalManager`; Security is advisory and not
  `PolicyEngine`.
- Rust task, governance, volatile memory/document, workflow, event, audit, and
  cancellation foundations are in-memory/unwired.
- Sealed workflows are fixture-only/no-I/O. D-091 parallelism is same-thread
  event multiplexing, not threads or provider concurrency.
- `NativeAgentRuntime` is sole/default and transport-free; it has no live
  provider/model/network/tool executor.
- Hermes remains NO GO/deferred.

### Shell and window baseline

- Default Tauri size is 1040×700; minimum is 760×520; window is resizable.
- Dynamic viewport height uses `100dvh` when supported.
- Body/shell/main lock overflow; `.application-content` is the primary vertical
  page scroll owner.
- `.application-sidebar` independently scrolls.
- The transcript is a justified internal scroll owner so the composer remains
  reachable.
- The approval dialog is viewport-bounded and internally scrollable.
- At 641–900px the shell track is 194px while the sidebar retains a 214px clamp
  minimum. This direct shared-shell mismatch must be corrected before the new
  route depends on that layout.

### Current accessibility constraints

- Semantic landmarks, native controls, labels, `aria-current`, visible focus,
  and reduced motion exist.
- Current route activation does not move focus or announce the new page.
- Conversation history can remain `aria-current="page"` when another workspace
  route is active.
- The approval dialog lacks verified focus trap, initial focus, Escape,
  background inertness, and focus restoration. It is not reused by this
  prototype; remediation requires a separately scoped change unless the owner
  expands this plan.
- Light subtle text is below 4.5:1 on common surfaces and must not be reused for
  compact Command Center labels without a measured token correction.
- JSDOM tests cannot validate geometry, computed overflow, wheel/pointer/touch,
  screenshots, or actual focus visibility.

## Current-state evidence

Gate A inspected:

- `src/App.tsx`, `src/App.test.tsx`, `src/styles.css`;
- `src/application/**`, `src/components/**`, and all `src/features/**` routes;
- typed Tauri clients under `src/infrastructure/tauri/**`;
- `package.json`, `package-lock.json`, Vite/TypeScript/ESLint configuration;
- `src-tauri/Cargo.toml`, `tauri.conf.json`, capabilities, `lib.rs`;
- agent definition, registry, task, orchestrator, runtime, workflow,
  parallelism, governance, memory, document, approval, audit, policy, and tool
  source/tests;
- architecture, security, brand, roadmap, ADR, status, and existing plan docs.

Current ignored `dist` evidence (not rebuilt during Gate A):

- allocated disk space: 388 KiB;
- logical artifact total: 383,117 bytes (374.14 KiB), including assets;
- main JavaScript: 229,541 bytes raw / 70,247 bytes gzip;
- CSS: 23,343 bytes raw / 5,009 bytes gzip.

The current app is eagerly bundled with no analyzer/budget/manual chunks.
Read-only M0 must obtain a fresh pre-change baseline and propose the explicit
initial/lazy budgets before dependency installation. Generated ignored `dist`
output is evidence, not a source or manifest change.

## Dependency decision checkpoint

The plan proposes at most two direct production dependency additions; neither
is installed or approved by Gate A:

1. `@xyflow/react` for the operational topology adapter.
2. `lucide-react` for statically imported functional icons.

Their transitive dependencies are permitted only as reviewed lockfile
consequences. Any optional browser, accessibility, or test dependency requires
a separate ledger and explicit owner approval.

Before any project install, read-only M0 must record:

- exact package and version;
- official package/repository and license;
- React 19/Vite compatibility and peer requirements;
- every proposed transitive and proposed lockfile consequence from an isolated
  temporary npm resolution that does not change repository manifests or locks;
- current maintenance/security posture;
- vulnerability-audit result for the isolated proposed resolution;
- direct and transitive license inventory from official package manifests;
- expected CSS/import requirements;
- initial accessibility/keyboard configuration;
- fresh raw/gzip baseline and the 10 KiB initial-route / 150 KiB lazy-route gzip
  budgets;
- exact proposed install, audit, inventory, and rollback commands/diffs.

M0 then stops and presents this ledger to the owner. It does not install a
package, mutate a manifest/lockfile, or edit application source.

### M0 ledger — 2026-08-19

- Gate opened with the approved post-increment command for
  `native-multi-agent-command-center-prototype`; no plan/repository conflict
  was found.
- Current environment: Node `v26.3.0`, npm `11.16.0`, React/React DOM `19.2.7`,
  Vite `7.3.5`; neither candidate is installed.
- Fresh baseline: `npm run test:frontend` passed 127/127 tests and
  `npm run build:frontend` passed. `dist` logical output is 383,117 bytes
  (374.14 KiB; 388 KiB allocated): main JS 229,541 raw / 70,083 gzip bytes and
  CSS 23,343 raw / 4,981 gzip bytes. The installation budgets remain +10 KiB
  gzip on the initial route and <=150 KiB gzip for the lazy Command Center
  chunk; the command center must not load before route selection.
- Proposed direct production additions only: `@xyflow/react@12.11.3` (MIT,
  `xyflow/xyflow`, React/React DOM peers `>=17`) and `lucide-react@1.33.0`
  (ISC, `lucide-icons/lucide`, React peer `^16.5 || ^17 || ^18 || ^19`). Both
  peer ranges accept the current React 19.2.7.
- Isolated `npm install --package-lock-only --ignore-scripts --save-exact`
  resolution produced 2 direct and 19 reviewed transitive lockfile entries:
  `@xyflow/system@0.0.80` (MIT), `classcat@5.0.5` (MIT),
  `zustand@4.5.7` (MIT), `use-sync-external-store@1.6.0` (MIT),
  `d3-color@3.1.0`, `d3-dispatch@3.0.1`, `d3-drag@3.0.0`,
  `d3-ease@3.0.1` (BSD-3-Clause), `d3-interpolate@3.0.1`,
  `d3-selection@3.0.0`, `d3-timer@3.0.1`, `d3-transition@3.0.1`, and
  `d3-zoom@3.0.0` (all other listed `d3-*` entries ISC), plus
  `@types/d3-color@3.1.3`, `@types/d3-drag@3.0.7`,
  `@types/d3-interpolate@3.0.4`, `@types/d3-selection@3.0.11`,
  `@types/d3-transition@3.0.9`, and `@types/d3-zoom@3.0.8` (all MIT).
  `zustand` is a React Flow internal dependency, not authorization for an
  application-global store.
- `npm audit --package-lock-only --omit=dev --json` against that isolated
  proposed production graph reported 0 vulnerabilities at every severity.
  The temporary directory was removed; no project manifest or lockfile changed.
- No Rust/Tauri/IPC/capability/CSP change is required. React Flow will be
  confined to the approved topology adapter and configured to preserve normal
  page wheel scrolling; Lucide imports will be static and named.

The ledger is ready for the required separate owner approval. No package or
source edit may begin until the exact second authorization sentence is received.

If either package fails this review, stop. Do not substitute another graph,
state, animation, command, icon, or chart library without owner review.

## Files expected to change

This map is proposed and must be revalidated at M0. Unexpected production file
needs stop the increment for review.

### New frontend feature files

- `src/features/command-center/CommandCenterPage.tsx`
- `src/features/command-center/CommandCenterPage.test.tsx`
- `src/features/command-center/commandCenterProjection.ts`
- `src/features/command-center/commandCenterProjection.test.ts`
- `src/features/command-center/commandCenterFixtures.ts`
- `src/features/command-center/useCommandCenterState.ts`
- `src/features/command-center/command-center.css`
- `src/features/command-center/components/CommandCenterHeader.tsx`
- `src/features/command-center/components/SystemStatusSummary.tsx`
- `src/features/command-center/components/OperationalTopologyPanel.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`
- `src/features/command-center/components/TopologyStructuredView.tsx`
- `src/features/command-center/components/ContextualInspector.tsx`
- `src/features/command-center/components/CommandCenterActivityStream.tsx`

The implementation may consolidate component files when doing so improves
cohesion; it must not create a monolith or a generic visualization framework.

### Existing frontend files

- `src/application/navigation.ts` — add one closed route/metadata entry.
- `src/App.tsx` — lazy route integration, Suspense/loading state, and shared
  route-focus behavior only.
- `src/components/ApplicationSidebar.tsx` — route-aware conversation
  `aria-current` correction if needed for the eighth route.
- `src/App.test.tsx` — all-route, focus, shared-shell, scroll-owner, and existing
  behavior regression coverage.
- `src/styles.css` — only shared semantic tokens and the direct 641–900px
  sidebar track correction; Command Center styles remain feature scoped.
- `package.json`, `package-lock.json` — only the two exact, separately approved
  direct production dependencies and their reviewed transitive consequences.

### Optional testing files

One browser/E2E runner and shared spec may be added only under a separate exact
tooling ledger and explicit owner approval; approval of the two production
dependencies does not cover it. Otherwise use existing automated tests plus
documented browser/Tauri manual validation.

A focused `OperationalTopologyAdapter.test.tsx` may be added for deterministic
viewport measurement, fit/reset/center, resize, zoom, and containment behavior;
it adds no runner or dependency.

### Protected unchanged files

- all `src-tauri/src/**` production source;
- `src-tauri/Cargo.toml`, Cargo lockfile, Tauri config/capabilities/CSP;
- storage/migrations;
- Rust agent/runtime/governance/memory/document/approval/audit/policy/tool code;
- Hermes ADRs/spikes/plans;
- existing conversation fixture semantics.

## Affected components

- Manual `AppRoute` and sidebar metadata.
- `ApplicationShell` page selection/loading/focus behavior.
- Shared shell width/overflow tokens.
- One new feature-local projection, reducer, fixture catalog, graph adapter,
  structured view, inspector, and activity view.
- Frontend package/build output and lockfile.
- Existing frontend test matrix.

No native component is an implementation dependency for this prototype.

## Interfaces and invariants

### Authority invariants

- `AgentOrchestrator` is one application-authority node, not an agent.
- Personal Assistant is one separate advisory/user-facing agent node.
- QA & Validation is not approval authority.
- Security & Risk is not policy authority.
- Workflow Automation is not orchestration authority and does not execute or
  create tasks.
- Only presentation selection/filter/viewport state exists in React.
- No UI action crosses IPC or changes trusted state.

The Operational Graph readability follow-up uses explicit 180 x 88 node
dimensions as the graph and CSS contract. Authoritative names wrap to at most
two lines, metadata occupies a separate bottom row, and compact group lanes are
derived from the same dimensions. Edge labels retain their semantic background
and attribution, and Structured view is unchanged.

### Catalog invariants

- Exactly nine agent nodes, one for each `AgentId::ALL` role.
- No duplicate, invented, or decorative agents.
- Exactly five view-only groups: Core, Intelligence, Engineering,
  Infrastructure, Governance.
- Cross-cutting relationships do not duplicate agent identity.

### Demo-truth invariants

- Projection version is `command-center-demo-v1`.
- Every node, edge, event, inspector, and summary is derived from a closed
  deterministic fixture and carries demo origin.
- Persistent visible copy reads `DEMO MODE · SIMULATED AGENT DATA`.
- No fixture state uses ambiguous `LIVE` wording.
- No provider, model, tool, MCP, connector, infrastructure, health, usage, or
  approval fact is presented as real.
- Every event has both a deterministic logical ordinal and a fixed ISO-8601
  fixture timestamp visibly labeled `SIMULATED TIME`; neither implies current
  wall-clock telemetry.

### Data and graph invariants

- App types do not import `@xyflow/react` types.
- The adapter is the only file that maps projection nodes/edges into graph
  library values.
- IDs are opaque closed presentation IDs; React does not construct trusted Rust
  identities.
- Edges derive only from fixture-declared typed endpoints/dependencies.
- Node/edge arrays are finite and immutable; all IDs/endpoints/groups validate.
- Stable explicit coordinates replace runtime layout.
- Completion timing never affects result or display order.

### State invariants

- Operational, availability, health, trust, approval, and demo states are
  distinct closed fields.
- Default health is `not-measured`; simulated health is visibly qualified.
- Selection/search/filter/view mode are feature-local.
- Scenario switching is user-directed and deterministic; no ambient timer
  simulates live work.
- Activity is capped at 48 fixture events.

### Scrolling invariants

- `.application-content` remains primary vertical scroll owner.
- Sidebar remains one independent scroll owner.
- No Command Center ancestor uses `100vh`/`100dvh` or competes with the shell.
- Each flex/grid chain uses `min-width: 0` and `min-height: 0` where required.
- Inspector/activity independently scroll only on bounded wide layouts; they
  rejoin document flow on narrow/reduced-height layouts.
- Unmodified wheel/trackpad vertical movement over the canvas scrolls the page.
- Explicit controls and focused modifier gestures zoom; drag/Space+drag pans.
- All controls and last items remain reachable at 760×520.

### Accessibility invariants

- Graph is never the sole representation.
- Structured view and graph share projection, selection, filters, and inspector.
- Text plus shape/icon/pattern supplements color.
- Visible labels remain for icon controls; names are programmatic.
- Keyboard users can search, filter, select, center, inspect, switch views, and
  reach activity.
- Focus is visible and moved/restored intentionally on route/view/panel changes.
- Reduced motion disables edge movement, pulse, and layout animation.
- Normal text meets 4.5:1; large text/UI boundaries meet applicable AA targets.
- No hidden reasoning, unredacted request, tool argument, secret, path, memory,
  document, approval subject, or runtime identity is shown.

## Proposed projection contract

The feature owns framework-neutral readonly types for:

- `CommandCenterProjection`;
- `TopologyGroup`;
- `TopologyNode` and closed `TopologyNodeKind`;
- `TopologyEdge` and closed `TopologyEdgeKind`;
- `OperationalStatus`, `AvailabilityStatus`, `HealthStatus`, `TrustStatus`,
  `ApprovalStatus`, and `DemoOrigin`;
- `InspectorProjection`;
- `CommandCenterEvent`;
- `CommandCenterSummary`;
- `CommandCenterScenarioId`.

The pure builder validates:

- one orchestrator, nine agents, five groups;
- identity/authority separations;
- unique IDs, valid group references, and valid edge endpoints;
- meaningful allowed source/target combinations per edge kind;
- fixture bounds, ordering, and provenance;
- no graph-library types or open arbitrary fields.

### Required deterministic scenarios

1. `catalog-idle` — all roles visible; no selected work; health not measured.
2. `research-queued` — queued root and Research assignment.
3. `research-knowledge-active` — fixture-only same-thread bounded scenario,
   labeled simulated; active flow and result path.
4. `engineering-waiting-approval` — a simulated application approval
   checkpoint; QA and Security remain advisory.
5. `infrastructure-blocked` — explicit prerequisite/security failure and
   blocked dependent path.
6. `workflow-cancelled` — explicit simulated cancellation propagation and
   terminal cancelled state with no successor work.
7. `workflow-completed` — completed proposal-only path and ordered results.

The implementation may reduce scenario count only if every required visible
state remains covered. It may not invent a live scenario.

## Implementation milestones

### M0 — Read-only readiness, baseline, and dependency ledger

- [x] Receive the exact first owner authorization sentence.
- [x] Re-read this plan, proposal, current Git state/diff, architecture,
      security, and dependency manifests.
- [x] Confirm no conflicting user changes; preserve unrelated work.
- [x] Re-run the protected-path/source inventory and current frontend tests.
- [x] Record a fresh production frontend chunk/raw/gzip baseline.
- [x] Complete the exact `@xyflow/react` and `lucide-react` direct, transitive,
      license, peer, maintenance, security, and accessibility review using an
      isolated temporary resolution without changing repository manifests or
      lockfiles.
- [x] Confirm the exact file map and no Rust/Tauri change need.
- [x] Record the exact proposed install, audit, license/transitive inventory,
      bundle-budget, and rollback commands and expected diffs.
- [x] Present the complete M0 ledger and stop. Do not install packages or edit
      source, `package.json`, or `package-lock.json` pending M0.5 authorization.
- [ ] Stop if a dependency, browser runner, IPC, Rust, Tauri, provider, or
      general graph framework expansion is required.

### M0.5 — Separate owner ledger approval and dependency installation

- [x] Receive the exact second owner ledger-approval sentence.
- [x] Reconfirm Git status and that no unrelated change overlaps manifests or
      expected source files.
- [x] Run only the exact `npm install --save-exact` command approved in the M0
      ledger for the two direct production dependencies.
- [x] Inspect the complete manifest and lockfile diff, direct/transitive tree,
      license inventory, and vulnerability audit before editing source.

### M1 — Typed projection and deterministic fixtures

- [x] Add closed framework-neutral projection/taxonomy types.
- [x] Add pure projection construction/validation.
- [x] Add the exact nine roles, one orchestrator, five groups, bounded scenarios,
      meaningful edges, inspector records, summaries, and <=48 events.
- [x] Add invariant, redaction, bound, and determinism tests.
- [x] Confirm no graph library import outside the adapter.

Checkpoint: production UI behavior remains unchanged; projection tests pass.

### M2 — Reversible route and shell integration

- [x] Add one Command Center route and sidebar item.
- [x] Lazy-load the feature with an accessible `PageState` loading fallback.
- [x] Preserve Conversations as available and unchanged.
- [x] Add persistent demo disclosure and empty/loading shell.
- [x] Correct only the shared 641–900px sidebar track/min-width mismatch.
- [x] Make conversation `aria-current` conditional on Conversations route.
- [x] Add route focus/announcement behavior without creating a focus trap.
- [x] Add all-eight-route and shell regression tests.

Checkpoint: no topology dependency behavior is trusted; every existing route
and current mock interaction still passes.

### M3 — Topology and accessible alternative

- [x] Map projection types to graph-library types in one adapter.
- [x] Render fixed non-editable nodes, groups, and semantic edges.
- [x] Disable connection, deletion, node dragging, ordinary wheel zoom, and
      ordinary wheel panning; verify the approved version's equivalents of
      `zoomOnScroll={false}`, `panOnScroll={false}`, and
      `preventScrolling={false}`.
- [x] Add selection, plain in-page search, agent/domain/status/entity/workflow
      filters, reset, center, fit, and explicit zoom; capability remains visibly
      unavailable until truthful data exists.
- [x] Keep all five domain groups expanded; defer domain collapse. Hover may
      repeat focus-visible labels but never own unique information or actions.
- [x] Add the synchronized grouped agent tree plus relationship table toggle;
      only the active graph or structured view remains in the accessibility
      tree and Tab order.
- [x] Add the composite graph keyboard model: Tab enters once, arrows traverse
      nodes, Home/End jump, Enter/Space select, Escape returns to the toolbar;
      edges remain operable through the relationship table without duplicate
      Tab stops.
- [x] Add graph/structured keyboard and semantic interaction tests.
- [x] Confirm all nine roles and authority boundaries in source/component
      evidence.

Checkpoint: graph can be removed without changing projection or structured view.

### M4 — Inspector and activity stream

- [x] Add closed inspector modes for orchestrator, agent, domain, task,
      workflow/step, checkpoint, and edge selections, with unavailable fields
      stated rather than invented.
- [x] Add bounded structured events with logical ordinals, fixed ISO-8601
      fixture timestamps labeled `SIMULATED TIME`, and attribution.
- [x] Add responsive wide/stacked source rules and focus return behavior.
- [x] Add activity search/filter/detail behavior.
- [x] Add failure, cancellation, blocked, approval-wait, and success tests.
- [x] Confirm no consequential control exists.

### M5 — Scrolling, accessibility, and visual validation

- [x] Validate primary/page/sidebar/canvas/inspector/activity scroll ownership.
- [x] Validate 2560×1440, 1600×1000, 1040×700, 1040×520, 760×520,
      and browser-only 640×800.
- [x] Validate mouse, trackpad, scrollbar, keyboard, browser zoom, resize, and
      touch where available.
- [x] Validate focus into view, grouped tree/relationship table,
      headings/landmarks, announcements, contrast, and reduced motion.
- [x] Verify no horizontal page overflow, clipped label, unreachable control,
      obscured content, or stale resize geometry.
- [x] Capture deterministic screenshots only if approved tooling exists.

Run on 2026-08-20 and completed on 2026-08-25 with the installed in-app Browser Control and
Computer Use Node REPL runtimes. The browser matrix passed at all six approved
sizes in light, dark, and reduced-motion states with zero page/content
horizontal overflow, zero authoritative-label clipping, all five records in
the longest fixture, and no effective motion above 0.001 seconds in reduced
mode. Representative compact text measured at 6.29:1 or better after the
scoped light-theme token correction; the React Flow attribution measured
7.42:1. Real wheel/fine scroll, direct scrollbar drag, keyboard focus scrolling,
independent sidebar/inspector/activity ownership, canvas wheel pass-through,
explicit graph zoom, Structured tree/table exposure, long conversation and
activity reachability, narrow ApprovalDialog, and dynamic native resize passed.
The packaged Tauri app resized 1040×700 -> 760×520 -> 1040×700 without restart,
and screenshots were captured through approved tooling. Touch was unavailable
where the exposed runtimes had no touch input. Owner-operated host zoom changed
the rendered Command Center to DPR 1.25 and 832×560 CSS pixels inside the
approved 1040×700 frame. Browser Control verified no horizontal overflow or
clipped controls, real page/sidebar scrolling, final-control reachability,
visible keyboard focus, and a rendered screenshot. Reset restored 1040×700 at
DPR 1.

### M6 — Verification, independent review, and closeout

- [x] Run focused projection/component tests during implementation.
- [x] Run formatting, lint, typecheck, frontend/full tests, frontend/Tauri build,
      docs, repository, security, and diff checks.
- [x] Record initial/lazy chunk and raw/gzip deltas.
- [x] Run independent code, accessibility, architecture, and security review.
- [x] Confirm protected Rust/Tauri/IPC/storage paths are unchanged.
- [x] Update only required plan/status/design documentation with exact evidence.
- [x] Stop at the deterministic prototype; do not begin real integration.

## Security and privacy considerations

- React data is untrusted presentation and cannot grant identity or authority.
- The prototype sends no IPC, network, provider, model, filesystem, database,
  tool, memory, document, approval, policy, or audit request.
- Fixture IDs are presentation-only and must not be accepted by future Rust
  commands as trusted IDs.
- Exclude objectives, prompts, output bodies, hidden reasoning, raw tool
  arguments/results, memory/document content/paths, approval affected data,
  credentials, environment values, raw JSON, and runtime identity.
- Avoid `dangerouslySetInnerHTML`; render bounded text with React escaping.
- Copyable content is only selectable pre-redacted bounded fixture text; do not
  invoke `navigator.clipboard` or add a Tauri clipboard capability.
- Search is local over fixed strings and performs no external lookup.
- Graph nodes/edges have no executable callback beyond local selection/filter.
- Tauri capabilities, CSP, permissions, and command allowlist remain unchanged.
- No health, usage, provider, connector, or approval status is presented without
  visible simulated/unavailable qualification.
- Hermes remains blocked; no source, dependency, process, or adapter is added.

## Accessibility plan

- Preserve `aside`, `nav`, toolbar/header, `main`, section, form, and list
  landmarks.
- Add a single logical page heading and route-change focus/announcement.
- Use real buttons/inputs; no clickable generic containers.
- Configure graph nodes as one arrow-key composite and relationships through
  the table; do not create one Tab stop for every edge.
- Provide concise custom ARIA labels and graph interaction instructions.
- Keep the synchronized grouped tree plus relationship table in document order;
  only the selected graph/structured mode is exposed to accessibility APIs.
- Expose inspector and activity as named regions when independently scrollable,
  associate their visible headings, preserve predictable focus entry/exit, and
  keep their first/last content keyboard reachable without gratuitous
  `tabIndex="0"`.
- Do not announce decorative edge animation; batch meaningful scenario changes.
- Use `aria-busy` for the lazy route/projection transition as appropriate.
- Restore focus after closing/toggling the inspector/alternative view.
- Apply `prefers-reduced-motion` to every transition and disable directional
  movement/pulses in reduced mode.
- Measure compact-label contrast in both themes; adjust only semantic tokens
  necessary to pass.
- Test keyboard-only navigation, Escape, tab order, focus visibility, browser
  zoom, and focus scrolling.

## Scrolling and responsive plan

Primary owner: `.application-content`.

Independent owners only when justified:

- `.application-sidebar` for sidebar overflow;
- existing conversation transcript for composer reachability;
- graph viewport for pan/zoom, not ordinary vertical scrolling;
- wide-layout inspector/activity only when bounded and exposed as named regions
  associated with visible headings;
- grouped tree/relationship table only when bounded and named; otherwise they
  remain in normal document flow;
- viewport-bounded modal/dialog content (no new modal planned).

Rules:

- no `100vh` inside the feature;
- no body-scroll change;
- `min-width: 0` and `min-height: 0` at every workbench chain;
- no `overflow-x: hidden` as a substitute for fixing child width;
- graph controls remain reachable at minimum height;
- wide inspector becomes normal-flow stacked content at narrow/reduced height;
- activity does not become a fixed footer;
- grouped tree plus relationship table are the default at browser-only <=640px;
- focused controls/nodes scroll/pan into view without obscuring global toolbar;
- resize uses CSS/container observation only; no cached window dimensions or
  restart requirement.

## Performance plan

- Lazy-load the Command Center feature and graph dependency.
- Use fixed node coordinates; no runtime layout package/worker.
- Keep first projection small: 1 orchestrator, 9 agents, 5 groups, and only a
  bounded selected work path.
- Use immutable fixture arrays and memoized projection-to-adapter mapping.
- Keep filter/selection state local with derived selectors.
- Cap fixture events at 48; do not virtualize at this size.
- Avoid ambient requestAnimationFrame/timer loops and continuous animation.
- Use static named Lucide imports only; no dynamic catalog import.
- Record fresh pre/post raw and gzip sizes for initial and lazy chunks using the
  same tool and flags.
- Limit the initial-route JavaScript-plus-CSS increase to 10 KiB gzip and the
  complete Command Center lazy JavaScript-plus-CSS payload to 150 KiB gzip.
- Confirm no graph or Command Center-only icon chunk loads before route
  selection. Crossing any threshold stops implementation for owner review.
- Do not prematurely add virtualization, global stores, layout engines, or
  animation libraries.

## Test plan

### Projection/domain tests

- exact one orchestrator, nine roles, and five groups;
- exact authority separation and labels;
- unique IDs, valid endpoints, allowed edge semantics, bounded event count;
- deterministic ordering and repeated-build equality;
- every value has demo origin; no `LIVE` copy;
- invalid duplicate/missing/foreign endpoints and unbounded fixtures fail;
- graph package types absent from public projection modules, proven by the
  exact import-boundary scan in the verification commands;
- no protected content fields in projection/debug output.

### Component/interaction tests

- loading, idle/empty, queued, active, approval wait, blocked, failed,
  cancelled, and completed states;
- graph/grouped-tree/relationship-table selection synchronization and inspector
  update for every closed selection kind;
- plain search plus agent/domain/status/entity/workflow filters, unavailable
  capability affordance, no-result, and reset;
- zoom/fit/reset/center controls;
- composite-node and relationship-table keyboard selection plus graph help;
- grouped tree plus relationship table contain all graph facts and edge
  semantics, and inactive view controls are absent from the Tab order;
- activity filtering, expansion, attribution, logical ordinal, simulated
  timestamp, and selectable pre-redacted text;
- persistent demo labels on page, canvas, inspector, and activity;
- no action dispatch/approval/tool UI;
- reduced-motion classes/behavior.

### Existing application regressions

- all eight routes render into `.application-content`;
- sidebar conversation/workspace navigation remains reachable;
- current Conversations, mock approval, Stop, Retry, result, Activity,
  Permissions, and Settings tests pass;
- conversation history has one correct current-page indication;
- 641–900px sidebar contract remains consistent;
- no new Tauri invocation/event listener.

### Real browser/Tauri validation

- every route at representative normal, reduced-height, and narrow viewports;
- Command Center at 2560×1440, 1600×1000, 1040×700, 1040×520, 760×520,
  and browser-only 640×800 in light/dark and reduced motion;
- short and longest Command Center fixture scenarios;
- Conversations with a long transcript and reachable composer; Activity with a
  long list; Permissions and Settings through their final controls; Tasks,
  Memory, and Integrations placeholders; long sidebar history; and the existing
  ApprovalDialog at reduced height/narrow browser size;
- mouse wheel, trackpad, keyboard, scrollbar, browser zoom, touch where
  supported;
- ordinary wheel over canvas scrolls the page; explicit canvas zoom works;
- final sidebar item, graph/tree/table item, inspector section, activity event,
  and page control are reachable/usable;
- focus scrolls into view and returns after panel/view changes;
- no horizontal page overflow, clipped popover, stale geometry, or fixed/sticky
  obstruction;
- dynamic resize requires no restart.

## Verification commands

Exact commands must be confirmed after M0. Expected repository commands:

```bash
git status --short --branch
git diff HEAD -- package.json package-lock.json
npm ls --all
npm query '*' --json
npm audit --omit=dev

rg -l '@xyflow/react' src --glob '!*.test.tsx'

npm run format:check
npm run lint:frontend
npm run typecheck
npm run test:frontend
npm run build:frontend

npm run lint:rust
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
npm run verify

npm run docs:check
npm run repository:check
npm run security:scan
git diff --check

git diff --exit-code HEAD -- src-tauri/src src-tauri/Cargo.toml src-tauri/Cargo.lock \
  src-tauri/tauri.conf.json src-tauri/capabilities
git ls-files --others --exclude-standard -- src-tauri/src src-tauri/Cargo.toml \
  src-tauri/Cargo.lock src-tauri/tauri.conf.json src-tauri/capabilities
```

If an approved browser runner is added, include its exact deterministic local
command and screenshots in the plan before closeout.

The `npm query` result is the reproducible direct/transitive license and package
inventory; review the dependency delta, not only top-level names. The audit must
be recorded independently from `npm run security:scan`, which scans repository
content rather than package vulnerabilities. The production-source `rg` output
must name only the topology adapter. A focused adapter test may name the package
only to install a test double; it is excluded from the production import-boundary
scan. The protected-path untracked-file command must print nothing.

## Risks

1. Graph semantics imply live execution or health.
2. New packages materially increase initial bundle/startup.
3. Graph canvas captures ordinary scroll or traps keyboard focus.
4. Nested inspector/activity scrolling makes content unreachable.
5. Visual groups are mistaken for routing/authority groups.
6. Fixture IDs are later reused as trusted backend identities.
7. QA/Security/Workflow Automation presentation overclaims authority.
8. Existing responsive width mismatch or low-contrast token propagates.
9. Feature state leaks into the global reducer and rerenders the shell.
10. Dense future knowledge scope expands the first prototype.
11. JSDOM-only tests create false confidence about geometry/accessibility.
12. Direct dependency changes exceed the two reviewed additions, or the lockfile
    contains an unreviewed transitive consequence.

Each is addressed by the explicit invariants, adapter boundary, lazy route,
structured alternative, finite fixtures, real-browser matrix, protected-path
checks, and stop conditions.

## Stop conditions

Stop and return to the owner if implementation requires:

- any Rust/Tauri/IPC/capability/CSP/storage change;
- a third direct production dependency, an unreviewed transitive lockfile
  consequence, or an unreviewed testing dependency;
- real task/workflow/provider/model/tool/memory/approval/audit data;
- a generic graph engine, runtime layout service, scheduler, worker, or stream;
- Hermes or another external runtime;
- state persistence or global-store replacement;
- removal/replacement of Conversations or the shell;
- broad current-UI redesign beyond direct shared prerequisites;
- accessibility without a structured graph alternative;
- ordinary wheel zoom that prevents page scrolling;
- a package/lockfile/version/license/security condition that cannot be verified;
- unrelated dirty changes that overlap an expected file.

## Rollback or failure strategy

Rollback is file-local and requires no data migration:

1. Remove the Command Center feature directory.
2. Remove the route metadata and lazy page entry.
3. Remove only Command Center semantic tokens and the direct sidebar-track
   correction if that correction is not retained independently.
4. Run `npm uninstall @xyflow/react lucide-react`, then inspect the generated
   manifest and lockfile diff; never hand-edit lockfile entries.
5. Remove Command Center tests and optional approved browser spec/config.
6. Re-run the seven-route shell and current frontend/full verification.

No Rust, IPC, storage, user data, database, credential, provider, or external
resource rollback exists because the prototype creates none.

On a failed milestone, preserve evidence, remove only that milestone’s isolated
changes when necessary, and do not continue into a later milestone.

## Accepted implementation decisions

- Add one dedicated Command Center route; do not replace Conversations.
- Use deterministic frontend projection/fixtures only.
- Use five presentation groups from the owner prompt.
- Use fixed layout rather than runtime graph layout.
- Conditionally add only `@xyflow/react` and `lucide-react`.
- Reuse React context/reducer and CSS motion; defer other evaluated libraries.
- Keep knowledge graph, usage metrics, global command palette, and real IPC out.
- Keep `.application-content` as primary scroll owner.

The owner accepted these decisions through the exact Gate B and M0.5
authorization sentences. They remain limited to this deterministic frontend
prototype and grant no later integration authority.

## Discoveries

- Current routes are reducer state, not a router; one route can be added without
  replacing navigation architecture.
- No current frontend type can truthfully supply multi-agent graph data.
- The bounded-parallel catalog offers strong semantic inspiration but is Rust-
  only, fixture-based, and same-thread; it must not be portrayed as live.
- Current Activity is a presentational mock list, not authoritative audit.
- Current shell scrolling is structurally sound and already tested, with one
  intermediate-width track mismatch and no real-browser validation.
- The approval dialog has independent focus-management debt but is not needed
  by the first prototype.
- A ten-node topology does not need virtualization or an auto-layout engine.
- No supplied visual-reference file was available in the repository; only the
  written high-level qualities informed the proposal.

## Progress

- [x] 2026-08-19: Gate A repository/UI/domain/dependency/scrolling/
      accessibility/performance audit completed read-only.
- [x] 2026-08-19: Design proposal and this proposed ExecPlan drafted.
- [x] Owner first authorization received (M0 only).
- [x] Gate B M0 ledger completed and exact M0.5 authorization received.
- [x] 2026-08-20: Installed only `@xyflow/react@12.11.3` and
      `lucide-react@1.33.0` as exact direct production dependencies.
- [x] 2026-08-20: M1-M4 deterministic frontend source and tests implemented.
- [x] 2026-08-20: Source-current focused tests pass 142/142 and the complete
      frontend suite passes 211/211 across 13 files after the targeted layout
      and viewport correction.
- [x] 2026-08-20: Operational Graph cards use one 180 x 88 layout contract,
      controlled two-line authoritative labels, a separate metadata row, and
      deterministic group-lane bounds; all nine labels and accessible names are
      covered without changing Structured view.
- [x] 2026-08-20: Source-current frontend formatting, lint, typecheck, and
      production build pass; strict Rust formatting/Clippy and the all-target
      Rust suite also pass.
- [x] 2026-08-25: Mandatory real-browser/Tauri M5 validation is complete with
      approved rendered-control tooling and owner-operated host zoom. Viewports,
      themes, reduced motion, scroll ownership/input, focus, accessibility,
      overflow, reachability, screenshots, native resize, zoom, and reset pass;
      touch was unavailable where unsupported.
- [x] Prototype verified.

## Acceptance criteria

- [x] The first owner authorization is recorded before read-only M0.
- [x] The exact version/license/transitive/lockfile/security/bundle ledger and
      second owner authorization are recorded before installation or source
      edits.
- [x] Only the approved direct dependencies and file scope change.
- [x] Command Center lazy-loads from one reversible route.
- [x] Existing seven routes and conversation behavior remain available in
      automated regression evidence.
- [x] One distinct orchestrator, all nine exact agents, and five groups render.
- [x] Authority boundaries are explicit in graph and structured views.
- [x] All displayed data is deterministic, bounded, typed, and visibly
      simulated.
- [x] Search, filters, selection, inspector, activity, fit/reset/center/zoom,
      and selected-path drill-down work.
- [x] All nine authoritative agent labels remain complete in graph cards and
      accessibility metadata; fixed card dimensions match fit and lane geometry.
- [x] Loading, empty, queued, active, approval wait, blocked, cancelled, failed,
      and completed/success states are covered.
- [x] Grouped tree plus relationship table are complete and synchronized.
- [x] Keyboard, focus, contrast, reduced motion, and announcements pass.
- [x] Primary/sidebar/panel/canvas scrolling passes every target viewport and
      input mode.
- [x] No horizontal overflow or unreachable content remains.
- [x] Initial route bundle is code-split and bundle deltas are recorded.
- [x] No Rust/Tauri/IPC/storage/capability/CSP file changes.
- [x] No live provider/model/tool/workflow/approval/MCP/memory/telemetry exists.
- [x] Hermes remains deferred/NO GO.
- [x] Formatting, linting, typecheck, tests, builds, docs, repository, security,
      and diff checks pass.
- [x] Independent code/accessibility/architecture/security review passes or has
      explicit nonblocking advisories.
- [x] Rollback is bounded by isolated ownership and no migration/external
      state.

## Final results

Source implementation is complete within the approved deterministic frontend
scope. The application now has a lazy Command Center route, one distinct
`AgentOrchestrator`, all nine exact agent roles, five view-only groups, seven
closed deterministic scenarios, a non-editable topology, synchronized grouped
structured view and relationship table, contextual inspector, bounded activity
stream, local search/filters, explicit viewport controls, route focus and
announcement behavior, and persistent
`DEMO MODE · SIMULATED AGENT DATA` disclosure. No UI action crosses IPC or
changes trusted state.

The Operational Graph readability follow-up uses explicit 180 x 88 node
dimensions as the graph and CSS contract. Authoritative names wrap to at most
two lines, metadata occupies a separate bottom row, and compact group lanes are
derived from the same dimensions. Edge labels retain their semantic background
and attribution, and Structured view is unchanged.

Only `@xyflow/react@12.11.3` (MIT) and `lucide-react@1.33.0` (ISC) were
added as exact direct production dependencies. The reviewed lock consequence is
19 transitive packages. The production dependency audit reports zero
vulnerabilities; the pre-existing development graph still reports five
unchanged advisories (one moderate and four high). `@xyflow/react` imports are
confined to
`src/features/command-center/components/OperationalTopologyAdapter.tsx`.

Source-current validation evidence is: projection 64/64, Command Center page
14/14, viewport adapter 5/5, `App` 28/28, application state 31/31, focused
total 142/142, and full frontend 211/211 across 13 files. Frontend formatting,
lint, typecheck, tests, and production build pass. Strict Rust formatting and
Clippy pass. Exact
`cargo test --manifest-path src-tauri/Cargo.toml --all-targets --all-features
--locked --quiet` passes 481 tests with zero failures and one intentional
opt-in Hermes probe ignored. Final current-tree `npm run verify` passes after
source and documentation synchronization, including the Tauri release
no-bundle build.

Using the ledger's reproducible `gzip -cn` method, initial JavaScript is
71,189 bytes gzip and initial CSS is 4,994 bytes gzip: 76,183 combined, an
increase of 1,119 bytes from the 75,064-byte baseline and 9,121 bytes below the
+10 KiB cap. The lazy Command Center JavaScript is 80,516 bytes gzip and its
CSS is 5,834 bytes gzip: 86,350 combined, 67,250 bytes below the 150 KiB cap.
The lazy route remains a separate production chunk.

Independent source, architecture, security, accessibility, and dependency
review found no source blocker and reports `PASS WITH ADVISORIES`. Approved
Browser Control and Computer Use evidence now verifies the required geometry,
scroll ownership, real input, focus, accessibility, light/dark contrast,
reduced motion, reachability, screenshots, and native dynamic resize matrix.
That work found and corrected only a scoped light-theme compact-text contrast
defect. Owner-operated host zoom produced a rendered 125% state at DPR 1.25
inside the approved frame; Browser Control verified overflow, clipping, focus,
scrolling, reachability, screenshot, and exact reset behavior. The full M5
matrix passes. Live IPC/provider/runtime/tool integration and every later
milestone remain Blocked pending separate readiness and authorization.

## Documentation updates for an authorized closeout

- [x] Update this plan with exact files, package versions, commands, results,
      screenshot availability, bundle deltas, reviews, risks, and final status;
      approved Browser Control and Computer Use tooling captured rendered
      browser and native screenshots during M5 without adding repository files.
- [x] Update the design proposal only where implementation evidence changes a
      recommendation.
- [x] Update `PLANS.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, and `HANDOFF.md`.
- [x] Update `ARCHITECTURE.md`, `SECURITY.md`, accessibility/security checklists,
      product requirements, roadmap, ADR, and governance matrix only when the
      verified prototype materially changes their current-state statements.
- [x] Add an increment record and post-increment review.
- [x] Keep any follow-on real IPC, knowledge graph, provider, or command palette
      plan separately blocked.

The native multi-agent ADR and governance matrix require no change: the
prototype adds no native catalog, routing, governance, IPC, or authority.

## Current gate status

Gate `native-multi-agent-command-center-prototype` has complete source,
automated, and rendered evidence. The full M5 matrix and fresh post-increment
automated command set pass, and the consolidated result is `PASS WITH
ADVISORIES` because no later increment is currently Ready.
