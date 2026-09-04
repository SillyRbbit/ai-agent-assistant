# Deterministic Graph demo and browser prerequisite ExecPlan

Status: Documentation quality PASS WITH ADVISORIES. Implementation Blocked.
Owner: Project owner (approvals); Codex (planning)
Increment: `deterministic-graph-demo-planning`
Last updated: 2026-09-04

## Goal

Plan the smallest connected private personal demo: an explicit fixed synthetic
request, manually stepped Research -> Knowledge -> Personal synthesis through
the existing sealed native host, with a matching Graph, status, journal and
validated final brief. Require repeatable automated real-browser interaction
and responsive evidence before material Graph integration.

The current owner request approves only the nine documentation paths below.
This plan does not approve implementation, dependencies or a presentation
exception. Each future implementation milestone requires separate approval,
a clean baseline, its own gate and a verified predecessor. M0 is read-only
and does not begin an implementation gate.

## User-visible outcome

The selected Research and Knowledge Graph shows the fixed objective before
Start. Three existing controls step or cancel the synthetic workflow. The
executing roles, phase, journal and brief derive from one accepted native
snapshot. Six other catalog agents remain visibly inactive context.
`DEMO MODE · SIMULATED AGENT DATA` stays visible throughout.

The brief appears only after native synthesis validation and successful terminal
cleanup. The next completed run demonstrates the existing synthetic failure;
cancellation remains possible at every active stage. This demonstrates
orchestration and presentation, not a live model, executor or persistent memory.

## Scope

Create this living ExecPlan and increment/review records; add current memory
without removing historical lines. Specify finite contracts, exact inventories,
dependencies, validation, estimates, rollback and unresolved approvals. Reuse
`PLANS.md` and the existing increment/review templates.

## Explicit non-goals

- No harness/demo implementation, source/configuration/tooling changes,
  dependency/lockfile edits, installs, downloads or cleanup in this increment.
- No branch creation, commit, push, merge, deployment or publication. Do not
  repeat PR114 closeout or create publication reconciliation.
- No live model/provider/network, arbitrary native request, new command/event,
  runtime selector, automatic stepping, polling or background work.
- No approval-to-execution bridge, tool/device authority, credential,
  permission, persistence, SQLite change, filesystem access, Hermes or OpenClaw.
- No redesign, broad refactor, restored removed UI sections, altered alternatives,
  Structured behavior change or domain/orchestrator/runtime replacement.
- No reopening D-121 or any blocked V0 operational lane. Preserve private,
  owner-only personal-demo scope and all existing trust boundaries.

## Existing behavior and constraints

The Graph is a frontend fixture. Its `research-knowledge-active` scenario depicts
interleaved work. Native D-086 uses sequential Research and Knowledge children
with the same Personal parent; Research does not spawn Knowledge.
`AgentOrchestrator` owns scheduling/validation; `ResearchKnowledgeDemoHost`
retains the sole/default `NativeAgentRuntime`.

The selected scenario mounts a separate prop-free lifecycle panel. It owns one
strict client and renders content-free v1 snapshots/journal; it does not drive
Graph state or show the native answer. The read-only native projection and
Conversations mock are separate proofs. Mock approval cannot authorize native work.

Preserve `get_app_info`, `get_research_knowledge_demo_projection`, and exactly
these four zero-input lifecycle commands:

```text
get_research_knowledge_demo_lifecycle_snapshot
start_research_knowledge_demo_lifecycle
advance_research_knowledge_demo_lifecycle
cancel_research_knowledge_demo_lifecycle
```

Retain the single mutex-owned host and notification-only event topic
`research-knowledge-demo-lifecycle-v1`. Command responses are state authority;
events are not. A notification failure after native mutation cannot trigger an
automatic mutation retry. Scenario re-entry can explicitly recover a snapshot.

## Current-state evidence

On 2026-09-04, branch `main`, HEAD and locally recorded `origin/main` were
`172d1e961ce8d8ef82dc6d204d1120cef0b758f3`, ahead/behind 0/0, with no staged,
unstaged or untracked paths. Read-only gate status showed
`personal-assistant-v0-pr114-publication-closeout`, `complete`,
`PASS WITH ADVISORIES`, `valid: true`. Other visible same-checkout tasks were
idle; delegated reviews were read-only. No concurrent writer was observed.
Root `AGENTS.md` was reread; no nested repository `AGENTS.md` was found.

Inspected evidence:

- `CommandCenterPage.tsx` derives Graph from the selected frontend scenario and
  separately mounts `ResearchKnowledgeLifecyclePanel.tsx`.
- `commandCenterProjection.ts` validates fixture provenance, seven scenarios,
  nine canonical agents and five groups. Its contract cannot carry native evidence.
- The lifecycle panel owns creation/disposal/busy/recovery state. Its client
  narrows `unknown`, checks exact fields/journal grammar and epoch/revision order.
- `src-tauri/src/research_knowledge_demo_lifecycle.rs` owns the fixed objective,
  fixtures and private success/failure schedule. `advance_synthesis` obtains
  validated synthesis but currently releases the workflow without projecting its answer.
- `src-tauri/src/agent/research_knowledge.rs` already exposes validated
  `FinalSynthesisResult` getters; no agent/core/runtime changes are needed.
- Native contract tests exercise host/adapter behavior, not rendered desktop IPC.
  Existing `ConversationWorkspace.tsx`, `src/App.test.tsx` and Graph adapter tests
  cover Return/composition and viewport contracts through Vitest/jsdom.
- Manifest, lockfile, installed package metadata and `node_modules/.bin` contain
  no Playwright, Puppeteer or Vitest Browser runner. No reproducible browser
  executable/revision is established by this plan.
- `NEXT_STEPS.md` already requires a separately approved real-browser harness.
  The completed [interaction plan](2026-09-02-gui-conversation-enter-graph-wheel-zoom.md),
  [responsive plan](2026-09-02-gui-responsive-alignment-correction.md) and TS-019
  distinguish unit/geometry, browser and actual macOS evidence.

Earlier same-session builds, tests and manual browser/native smokes are
historical audit observations, not fresh acceptance for this documentation
increment or a future integration. A 5120-pixel geometry assertion is not a
rendered 5120-pixel result. CSS transforms, DPR and viewport emulation do not
prove browser chrome zoom or OS display scaling.

## Files expected to change

Exact current documentation allowlist, declared before edits and gate begin:

```text
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
ROADMAP.md
docs/plans/2026-09-04-deterministic-graph-demo-planning.md
docs/increments/deterministic-graph-demo-planning.md
docs/reviews/2026-09-04-deterministic-graph-demo-planning-post-increment-review.md
```

Retain all pre-existing lines in the six memory files. Every other tracked path
remains byte-identical to baseline: accepted decisions/governance, historical
plans/reports, frozen publication/candidate evidence, source/tests, hooks/guards,
Tauri configuration, manifests and lockfiles. Ignored local gate metadata is
workflow evidence, not a tenth repository edit.

## Affected components

Future work affects a test-only browser runner, the sealed host's snapshot,
strict client, one feature-local lifecycle owner and a small Graph render seam.
Reuse existing renderer, controls, canonical roster and responsive geometry.
Do not add a general-purpose runtime/presentation framework.

## Interfaces and invariants

### Proposed additive presentation exception — not accepted

Before M2 executable changes, obtain owner acceptance and record it additively.
Do not rewrite D-092, D-093, D-112, F-12 or the completed
[connected-presentation plan](2026-08-28-research-knowledge-demo-connected-presentation.md).
The exact proposed exception is:

> Only while the existing Research and Knowledge scenario is selected, one
> application-owned lifecycle client may supply accepted command-response state
> to its existing simulated panel and that scenario's Graph, status, inspector
> and bounded journal. The four fixed zero-input commands may return a v2 snapshot
> containing the existing fixed synthetic objective and one exact allowlisted
> validated final brief. This grants no input, runtime-selection, model,
> provider, tool, policy, approval, audit, execution, persistence or device
> authority. Other scenarios, Structured and Conversations remain separate
> fixture presentations. Notifications remain non-authoritative.

F-12 currently pins the sole consumer, prop-free mount, imports, three controls
and source digests. Replace only necessary pins atomically with equally narrow
checks for exactly one feature owner. Preserve negative tests for alternate
consumers, caller arguments/selectors, extra listeners, raw Tauri access,
implicit operations, network/storage and capability/CSP drift. Hash updates
without replacement-boundary tests are unacceptable. Planning adds no accepted ADR.

### Finite native snapshot v2

Keep current epoch/revision bounds, closed errors/states, journal keys, exact
grammar and maximum eight entries. Keep scenario `research-knowledge-demo-v1`,
disclosure and `application-owned-synthetic-fixture` provenance. Atomically
advance the payload schema to `research-knowledge-demo-lifecycle-v2` on the
unchanged event topic; old/mixed payloads fail closed without fallback.

Set `proofBoundary` to exactly:

```text
This selected Graph and lifecycle share one sealed synthetic native workflow. Other scenarios, Structured, Conversations mock, and acceptance fixtures remain separate proofs.
```

Add only `objective` and `result` to the exact top-level key set. `objective`
is the existing 71-byte ASCII literal:

```text
Compare two technical approaches and create a structured decision brief
```

`result` is null except in `succeeded`, where its exact keys and values are:

```json
{
  "kind": "research-knowledge-fixed-brief-v1",
  "answer": "Fixture-based decision brief: A is simpler [approach-a]; B may scale further [approach-b]. Verify before acting.",
  "sourceIds": ["approach-a", "approach-b"],
  "fixtureBased": true,
  "status": "complete"
}
```

The 112-byte ASCII answer and ordered source tuple are literal allowlists, not a
general text channel. Read the validated synthesis getters; check exact answer,
sources, fixture flag and complete status; project only with successful terminal
containment/cleanup. Do not render the fixture constant directly or fabricate a
brief from the success enum. Use a private nested result type; no `lib.rs` edit.

Expose no source bodies, findings, Knowledge output, URLs/paths, runtime/task/run
IDs, memory, audit, raw errors or reasoning. Static labels map `approach-a` to
`Approach A fixture` and `approach-b` to `Approach B fixture`. Redact the nested
result's `Debug`; retain forbidden-content/debug sentinels while narrowly
updating the old content-free serialization assertion.

Start clears the prior result before the new epoch. Idle, active, failure,
cancellation and cleanup-pending contain null. Validation failure follows
existing containment; ownership/quarantine cannot be released early. Preserve
success -> synthesis failure -> success scheduling and cancellation consuming
no outcome. Successful manual revisions remain 1/3/5/6 in one epoch.

Compare all new fields for same-epoch/revision equality. Preserve stale,
malformed, contradictory, reordered and newer-notification rejection/recovery
rules. Reject post-disposal work. Hide the brief while connecting, unavailable,
recovery-required or mutating. Scenario re-entry hydrates via the existing
snapshot call; add no fourth control or automatic mutation retry.

### One owner and closed Graph projection

Extract exactly panel ownership into `useResearchKnowledgeDemoLifecycle.ts`.
Page, panel, Graph, summary/status, inspector, journal and brief share one
accepted snapshot. No second client, global store, polling, automatic stepping
or new Tauri consumer. Preserve read-only hydration and explicit user actions;
leaving the selected scenario disposes the owner.

Keep fixture contract/validator/catalog and Structured unchanged. Add
`operationalGraphProjection.ts` as a closed render-only union of the unchanged
fixture model and the finite `researchKnowledgeGraphProjection.ts` model.
Convert explicitly; never cast native data to `CommandCenterProjection`, weaken
its validator or relabel the existing interleaved fixture.

Native render contract: version `research-knowledge-graph-v1`, provenance
`sealed-native-synthetic`, accepted epoch/revision, exactly nine canonical agent
entries plus AgentOrchestrator and the existing five groups. Only Personal,
Research and Knowledge participate; six others are `inactive-in-this-demo`.
IDs use closed `native-demo:` values, remain presentation-only and never reach Rust.

Exactly five fixed relationships: orchestrator ownership of Personal; Personal
delegation to Research; Personal delegation to Knowledge; Research-to-Knowledge
evidence dependency; Knowledge-to-Personal synthesis return. Result-flow edges
show temporal dependencies, not child creation or authority. No extra work
nodes or inferred live telemetry are needed.

Derive stage status from accepted state/journal: waiting until started, active
at the current stage, complete only with its completion entry, failed/cancelled
only for the interrupted stage. Preserve earlier completed stages. Cleanup-pending
shows blocked/unavailable and no brief/replacement action. Success completes
three roles. Show Research -> Knowledge -> Personal synthesis as an ordered
phase trail; retain native depth-one sibling ownership in labels and edges.

Distinguish Personal's overall root outcome from its synthesis phase. On a
failed/cancelled workflow, Personal's root outcome is failed/cancelled even when
synthesis never started. The phase trail preserves completed phases, marks the
interrupted phase failed/cancelled and labels later unstarted phases skipped.
Personal waits for its children while Research/Knowledge run, synthesizes only
in synthesis, and cannot appear still waiting after a terminal root outcome.

Journal uses native revision order, fixed labels and at most eight entries, no
invented timestamps. All selected-Graph status/summary/activity and inspector
use this native branch, not the parallel fixture timeline. Selection carries
native provenance/epoch and resets on epoch/scenario/view changes. Keep existing
filters meaningful over this closed branch. Structured retains its unchanged
fixture projection with explicit separate-proof labeling, no native selection
or result claim. The static native projection remains separately labeled.

## Implementation milestones

M0 is an approval prerequisite. M1 -> M2 -> M3 -> M4 are sequential separately
approved bounded increments. Do not start a later milestone with an open failure,
unclean baseline, invalid marker or pending required manual evidence.

### M0 — exact browser toolchain decision

Read-only: identify an exact pinned runner and browser engine/revision/executable,
Node compatibility, license/security evidence, acquisition source, lockfile impact
and installation/download commands. `@playwright/test` is proposed; no version
or browser availability is selected or verified today. Prefer an already supplied
approved reproducible toolchain only if positively verified. Do not use implicit
`npx` downloads, unverified globals or a personal browser profile.

Return the concrete acquisition and M1 scope for separate owner approval before
installing or implementing anything. This is a finite prerequisite decision,
not publication or candidate reconciliation. Estimate: 1–2 engineering hours,
plus approval/acquisition waiting. Immediate implementation readiness: **Blocked**.

### M1 — automated real-browser fixture harness first

Only the approved harness against current frontend behavior; no product source
changes. Exact proposed executable/configuration inventory:

```text
package.json
package-lock.json
tsconfig.json
tests/browser/tsconfig.json
tests/browser/playwright.config.ts
tests/browser/helpers.ts
tests/browser/conversations.browser.ts
tests/browser/graph.browser.ts
tests/browser/responsive.browser.ts
scripts/run_browser_tests.mjs
scripts/tests/test_ci_change_scope.py
```

Dedicated strict TypeScript project/root reference; `.browser.ts` and explicit
runner matching avoid Vitest discovery. Proposed `npm run test:browser` invokes
`node scripts/run_browser_tests.mjs`; this command does not exist today. Build
frontend assets, start one owned strict-port loopback preview, use one worker,
bounded timeouts/no automatic retries, and an empty ephemeral browser context.
Never reuse an ambient server or authenticated profile.

Block/fail unexpected remote navigation/requests, popups/downloads; disable
service workers. Allow app resources only from the exact owned loopback origin.
This test request contract is not process-wide no-egress proof or product network
authority. Keep synthetic traces/screenshots temporary/local without uploads.
Failure, timeout and interruption close/reap browser/context/server and verify
port release. Missing runner/runtime or occupied port fails without acquisition.
Preserve fail-closed CI classification; add path tests, no hosted browser jobs
or acquisition workflow. Existing `npm run verify` does not run the browser suite.

Acceptance: full browser matrix below plus required complete verification for
development-dependency/configuration changes. Any exposed product defect blocks
M1/M2 and requires a separately bounded remediation decision, not incidental fixes.
Estimate: 4–7 hours implementation plus 2–4 hours verification/review.

### M2 — connected native Graph integration

Prerequisites: M1 complete/valid, clean baseline and explicit acceptance of the
exception and finite contracts. Exact proposed source/test/guard inventory:

```text
src-tauri/src/research_knowledge_demo_lifecycle.rs
src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs
src-tauri/tests/research_knowledge_demo_lifecycle_contract.rs
src-tauri/tests/research_knowledge_demo_lifecycle_tauri_contract.rs
src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts
src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts
src/features/command-center/useResearchKnowledgeDemoLifecycle.ts
src/features/command-center/useResearchKnowledgeDemoLifecycle.test.tsx
src/features/command-center/operationalGraphProjection.ts
src/features/command-center/operationalGraphProjection.test.ts
src/features/command-center/researchKnowledgeGraphProjection.ts
src/features/command-center/researchKnowledgeGraphProjection.test.ts
src/features/command-center/ResearchKnowledgeLifecyclePanel.tsx
src/features/command-center/ResearchKnowledgeLifecyclePanel.test.tsx
src/features/command-center/CommandCenterPage.tsx
src/features/command-center/CommandCenterPage.test.tsx
src/features/command-center/useCommandCenterState.ts
src/features/command-center/components/OperationalTopologyAdapter.tsx
src/features/command-center/components/OperationalTopologyAdapter.test.tsx
src/features/command-center/components/OperationalTopologyPanel.tsx
src/features/command-center/components/OperationalTopologyPanel.test.tsx
src/features/command-center/command-center.css
scripts/repository_health.py
scripts/tests/test_repository_health.py
```

Protect `lib.rs`, `src-tauri/src/agent/**`, static native projection, Conversations,
Structured, fixture contract/catalog, dependencies/lockfiles and Tauri
capabilities/CSP/command registration. Page tests cover selection state. Test
transport injection cannot become a production input or selectable lifecycle.

Acceptance: matched Graph/status/journal/brief and all negative contracts;
existing browser regressions; full verification; actual Tauri smoke on the target
Mac. Browser fixtures must report absent native capability, never spoof Tauri
internals to claim a native run. Estimate: 10–16 hours integration/focused tests
plus 3–5 hours full verification/native smoke. The integration estimate includes
4–7 hours native contract and 2–4 hours client/policy work; do not add them again.

### M3 — necessary polish only

After M2 passes, select only observed demo-blocking readability, clipping, focus
or status-label defects. If none exist, record a verified no-change disposition.
Do not manufacture polish or redesign work.

Proposed maximum source inventory: `src/features/command-center/command-center.css`,
`src/features/command-center/CommandCenterPage.tsx`, its existing `.test.tsx`,
`src/features/command-center/components/OperationalTopologyAdapter.tsx`, its
existing `.test.tsx`, and `tests/browser/responsive.browser.ts`. Freeze an exact
subset against actual defects for approval before begin. Beyond this ceiling,
including native failure, requires a new bounded remediation decision. Rerun
affected browser/native evidence and the applicable completion tier. Estimate:
0–4 hours fixes plus 1–2 hours verification.

### M4 — final local rehearsal

After M3 passes or is verified unnecessary, rehearse cold launch, fixed request,
three manual advances, exact successful brief, next synthetic failure,
cancellation at each active stage and quit/relaunch reset. Check matched Graph,
status/journal, no stale answer/duplicate run, focus and target window sizes.
Separate actual WebView/IPC, browser fixtures, synthetic composition events,
actual IME and display scaling evidence.

No fixes inside rehearsal: stop and seek bounded repair approval for failures.
Record sanitized evidence and a repeatable operator script in this plan and
new increment/review records. No publication/distribution. Estimate: 2–3 hours,
excluding unavailable target/IME/manual evidence.

### Future documentation inventories

Each milestone uses exactly these common memory/plan paths:
`CHANGELOG.md`, `HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`, `PROJECT_STATUS.md`,
`ROADMAP.md`, `docs/plans/2026-09-04-deterministic-graph-demo-planning.md`.
Proposed new records:

```text
M1: docs/increments/deterministic-graph-browser-harness.md
M1: docs/reviews/2026-09-04-deterministic-graph-browser-harness-post-increment-review.md
M2: docs/increments/deterministic-graph-native-integration.md
M2: docs/reviews/2026-09-04-deterministic-graph-native-integration-post-increment-review.md
M3: docs/increments/deterministic-graph-demo-polish.md
M3: docs/reviews/2026-09-04-deterministic-graph-demo-polish-post-increment-review.md
M4: docs/increments/deterministic-graph-demo-rehearsal.md
M4: docs/reviews/2026-09-04-deterministic-graph-demo-rehearsal-post-increment-review.md
```

M1 additionally changes `TESTING_GUIDE.md` and `DECISIONS.md` for accepted tooling.
M2 additionally changes `AGENTS.md`, `docs/PROJECT_DIRECTION.md`, `ARCHITECTURE.md`,
`PRODUCT_REQUIREMENTS.md`, `SECURITY.md`, `SECURITY_CHECKLIST.md`, `TESTING_GUIDE.md`
and `DECISIONS.md` additively for the accepted presentation exception. M3/M4
need no governing-contract change. Refreeze proposed report dates explicitly to
the actual execution date if approval occurs later. Create no future record now.
Unexpected paths or troubleshooting fixes stop for scope approval.

Total estimate: approximately 23–43 engineering hours, roughly 4–8 focused days
at six productive hours per day,
after prerequisites are available; approval/acquisition, new defects and missing
manual evidence add time. No live-provider or production-readiness claim.

## Security and privacy considerations

The model remains untrusted; Rust retains validation/scheduling/containment.
A fixed result grants no WebView policy, approval, audit, execution or memory
authority. Preserve D-093's separate checkpoint-denial and safe manual-dispatch
branches. Do not fabricate a combined approval-to-execution story.

Preserve zero-input signatures, returned-identity validation, epoch/revision,
rejected-run quarantine, child-first cancellation, Drop sentinel and replacement
blocking. Do not weaken output parsing, typed Rust errors, engine checks, CSP,
capabilities or permissions. Development-tool acquisition is not product egress
or authority for dependency changes in another milestone.

## Test plan

M1 automated real-browser acceptance:

- Plain Return sends exactly one nonempty request; empty/whitespace, disabled,
  busy and awaiting-approval states send none; repeated Return cannot duplicate.
- Shift+Return inserts an actual newline. Alt/Control/Meta+Return never submit;
  retain draft without assuming identical platform modifier editing behavior.
- Separate DOM `isComposing` and key-code-229 guard events never submit. Label
  these synthetic composition tests; actual OS IME confirmation remains manual.
- Wheel up/down inside renderer changes accessible zoom correctly within 50–150%
  without outside scrolling. Outside-wheel on measured overflowing content
  scrolls that region with unchanged Graph zoom. No overflow is missing evidence.
- Wheel/pan enters manual mode; resize preserves manual zoom. Fit/Reset restores
  bounded automatic framing. Do not promise full fit below the 50% floor;
  controls/readable access remain available.
- Click and arrows/Home/End plus Enter/Space select the correct agent/inspector.
  Escape returns viewport-control focus; inspector close restores invoker;
  route change focuses main content. Composer, filters, toolbar, activity and
  sidebar remain keyboard reachable without traps.
- CSS-pixel viewports 760x520, 1040x700, 1280x720, 1440x900, 1678x1038,
  5120x1440; navigation/inspector/activity expanded and collapsed; repeated
  threshold resize. Verify nonzero canvas, no document overflow, reachable
  controls, readable labels, nonoverlapping lanes and orchestrator clearance.
  Unsupported dimensions block or remain explicitly pending, never replaced
  silently by smaller viewports or geometry assertions.
- Fail app console errors/uncaught exceptions, unexpected remote requests and
  leaked owned processes. Record engine/revision, viewport/DPR, source revision
  and clean startup/shutdown evidence.

M2 native/client/projection acceptance:

- One-epoch revisions 1/3/5/6 reach the exact validated brief. Success -> failure
  -> success; cancel each active stage without consuming the next outcome.
  Result null in every nonsuccess state and cleared before a new run.
- Unknown/extra/missing keys, wrong/oversized answer, reordered/duplicate/remapped/
  missing sources, partial status, false fixture flag and state/result
  contradictions fail closed. Preserve identity mismatch, failed cancellation,
  retained owner, quarantine and Drop replacement-blocker tests.
- Exact parsing/equality, old-schema rejection, epoch/revision bounds,
  stale/duplicate/gapped/contradictory responses/events, post-disposal work and
  explicit snapshot recovery. Emission failure after committed success can
  recover the same snapshot without rerunning synthesis.
- Projection uses only accepted state/journal and closed catalog. Stage,
  failure/cancel/cleanup, selection and scenario/view/disposal tests prevent
  fixture/epoch/result mixing. Native journal has no invented timestamps.
- All six registrations/signatures unchanged; F-12 negative tests retained
  under the atomic owner/pin replacement.
- Real target-Mac Tauri: button -> IPC -> native transition -> matching Graph,
  status/journal -> brief. Failure, cancellation, navigation away/back and
  quit/relaunch tested. Native units and browser mocks cannot satisfy this gate.

## Verification commands

Current documentation increment, after final relevant edits:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Also verify exact nine paths, zero historical memory-line removal, equality of
protected tracked paths to baseline, independent architecture/security/code-health/
readiness review and machine report validation. Finalize only this planning gate;
require `complete` / `valid: true`. No application builds, browser runs or
package advisory retrieval are required now.

M1 proposed completion, after approved acquisition (not run now):

```bash
npm run test:browser
npm run verify
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

M2 focused commands, then the same complete M1 command set once:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --locked research_knowledge_demo_lifecycle
cargo test --manifest-path src-tauri/Cargo.toml --locked --test research_knowledge_demo_lifecycle_contract --test research_knowledge_demo_lifecycle_tauri_contract
npm run test:frontend -- src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts src/features/command-center
npm run test:repository
```

M3 freezes the appropriate Risk-Based Validation tier for approved defects, plus
all affected browser assertions and native evidence. M4 uses a matching verified
executable, required native/manual rehearsal, current browser suite and docs-tier
checks. Changed/unverifiable executable evidence requires verification again.
All milestones require independent, session, quality, report and post-increment
gates. Passing checks do not confer owner authority.

## Risks

- Exact browser package/runtime/acquisition unresolved: blocks M1; owner decides.
- Presentation exception unaccepted: blocks M2; owner accepts and reviewers
  prove replacement guards before behavior changes.
- Native/fixture provenance mixing risks a false workflow story: closed models
  and target-native smoke are mandatory M2 evidence.
- Browser tests may expose defects: block rather than expand into product fixes.
- Actual macOS IME/scaling/IPC evidence may be unavailable: mark pending and
  withhold affected completion/readiness, never substitute mocks.
- Planning leaves nine uncommitted documentation paths. Owner Git disposition
  is required for a clean implementation baseline; checks do not make it clean.

## Rollback or failure strategy

Stop on overlap, a writer, unclean baseline, invalid evidence, failed required
gate, scope drift, unapproved acquisition, unsupported finite output or missing
manual prerequisite. Preserve all user changes. Correct only in-scope planning
review defects; never discard work or repeat valid completion.

Harness failure closes/reaps only its owned processes and retains bounded
synthetic diagnostics; never clean shared caches/dependencies. Native failure
hides result and retains containment ownership. Any approved integration revert
must cover its exact host/client/owner/guard/docs change atomically; never add a
fallback parser or substitute old fixture data for native evidence. Required
failure uses the truthful terminal-failed gate, no fabricated PASS or successor.

## Decisions made

The owner selected documentation planning, existing host/orchestrator/runtime,
zero-input commands and harness-first sequencing. Retain nine-agent context
with only three participating roles. The v2 contract and Graph seam are proposals.

Unresolved approvals: M0 exact toolchain/acquisition and M1 inventory; M2 additive
exception/contract/inventory; evidence-driven M3 subset; M4 rehearsal. No ADR,
Git action or successor implementation is approved by documentation acceptance.

## Discoveries

Validated synthesis getters avoid domain/runtime rewrites. Fixture provenance
cannot truthfully carry native state. One narrow Graph seam and one extracted
owner are sufficient; F-12 must protect the new ownership. A real-browser runner
is absent, so acquisition must be frozen explicitly rather than assumed.

## Progress

- [x] Reread governance/current state, applicable plans and source/tests.
- [x] Verify clean main, valid completed closeout and no observed writer.
- [x] Declare nine paths and begin only this new planning gate.
- [x] Prepare contracts, inventories, sequence, estimates and approval stops.
- [x] Pass independent review and documentation/session/preservation checks.
- [x] Prepare the truthful report for mandatory report/fingerprint validation.
- [ ] M0/M1 tooling/scope approved and harness verified.
- [ ] M2 exception approved and integration verified.
- [ ] M3 necessary polish verified complete or unnecessary.
- [ ] M4 rehearsal verified.

## Acceptance criteria

- [x] Distinguish fixtures, native lifecycle and proposed integration.
- [x] Provide exact current scope and proposed implementation inventories.
- [x] Define finite results, authority, cleanup and provenance invariants.
- [x] Specify harness-first outcomes, validation, estimates and rollback.
- [x] State unresolved approvals and premature-implementation stops.
- [x] Required current checks/review pass and exact preservation is proven.
- [x] Prepare a truthful planning report; require final gate/fingerprint validation.

## Final results

Planning content passed documentation checks and independent review with
advisories. Completion evidence belongs in the
[planning review](../reviews/2026-09-04-deterministic-graph-demo-planning-post-increment-review.md).
Finalize only after read-only report validation and final checks; the gate's
`complete` / `valid: true` status is authoritative for report/workspace integrity.
Implementation remains Blocked. No harness, native payload or product behavior
changed. The exact uncommitted nine-file delta requires separate owner Git
disposition before a clean-baseline implementation.

## Documentation updates

Only this plan, increment/review and six allowlisted memory files change.
Accepted decisions, architecture/security requirements and historical records
remain unchanged. Top HANDOFF/NEXT_STEPS entries describe this selected planning
lane; older dated blocked-lane/publication evidence admits no operational successor.
