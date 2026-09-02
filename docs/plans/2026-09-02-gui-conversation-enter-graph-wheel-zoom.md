# GUI conversation Enter and Graph wheel zoom

Status: Verified complete with advisories
Owner: Codex
Last updated: 2026-09-02

## Goal

Make the conversation composer submit from the keyboard and make ordinary
mouse-wheel gestures zoom the Command Center Graph in the direction requested
by the owner.

## User-visible outcome

With focus in the conversation composer, Return sends a non-empty request and
Shift+Return inserts a line break. Composition-confirmation Return does not
send. With the pointer over the Graph canvas, scrolling up zooms in and
scrolling down zooms out; scrolling outside the Graph continues to move the
page.

## Scope

- Add guarded keyboard submission to the existing conversation form.
- Give React Flow ordinary wheel ownership only over its renderer and keep the
  existing manual-viewport transition.
- Update Graph help, focused tests, and the current product requirement.
- Record the owner-directed wheel-policy change additively and synchronize
  current project memory.

## Explicit non-goals

- No conversation, message, task, event, fixture, or canonical-agent data
  change.
- No composer redesign, global keyboard shortcut, modifier-to-send preference,
  graph panning change, Structured change, or graph-library replacement.
- No dependency, Rust, Tauri, IPC, persistence, model, provider, tool,
  permission, approval, audit, network, execution, or device-authority change.
- No branch, commit, push, merge, release, or publication work.

## Existing behavior and constraints

- The conversation composer is already a form with a submit button, but its
  textarea has no keyboard submit handler, so Return only inserts a line break.
- React Flow currently has `zoomOnScroll` and `preventScrolling` disabled;
  FR-039K and visible help intentionally describe the former page-wheel
  pass-through behavior.
- The existing Graph `onMove` callback distinguishes automatic fit from manual
  interaction and must continue to own the zoom readout and resize behavior.
- The validated 42-path GUI workspace and the owner-provided `AGENTS.md` change
  must be preserved without reset, stash, or unrelated rewriting.

## Current-state evidence

- The prerequisite `gui-responsive-alignment-correction` completion marker is
  complete and valid.
- Focused frontend baselines pass before this increment.
- Readiness is **Ready with advisories**: the owner's exact request is a
  separately approved bounded presentation increment despite D-111 blocking
  automatic product successors. FR-039K must be explicitly superseded for
  wheel behavior, and real-browser interaction needs direct verification.

## Files expected to change

- `src/features/conversations/ConversationWorkspace.tsx`
- `src/App.test.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.test.tsx`
- `src/features/command-center/CommandCenterPage.test.tsx`
- `PRODUCT_REQUIREMENTS.md`
- `DECISIONS.md`
- Current plan, increment, status, handoff, next steps, changelog,
  troubleshooting, review, and gate records as evidence requires

Protected from modification:
`src/features/command-center/components/TopologyStructuredView.tsx`,
`src-tauri/`, `package.json`, and `package-lock.json`.

## Affected components

- Conversation request composer
- Command Center Graph viewport interaction and help

## Interfaces and invariants

- Plain Return submits exactly once only when the textarea is enabled and the
  trimmed draft is non-empty.
- Shift+Return remains multiline input; IME composition Return, including the
  WebKit 229 fallback, never submits.
- The form submit path remains the single request action, with existing reducer
  guards authoritative for run state.
- Graph wheel zoom uses React Flow's bounded `minZoom`/`maxZoom`, retains
  `panOnScroll={false}`, and consumes wheel events only over the renderer.
- Wheel zoom enters existing manual viewport mode; Fit/Reset and dataset-change
  automatic framing remain unchanged.
- Structured, canonical data, fixture disclosure, and all native/trust
  boundaries remain unchanged.

## Implementation milestones

- [x] Inspect current composer, Graph, tests, and authoritative requirements
- [x] Complete bounded readiness review and focused baseline
- [x] Add failing keyboard and wheel interaction regressions
- [x] Implement guarded Return submit and Graph-local wheel zoom
- [x] Pass focused/full checks and rendered interaction verification
- [x] Synchronize documentation and complete the post-increment gate

## Security and privacy considerations

The change is presentation/input handling only. Keyboard content remains in the
existing volatile frontend mock path. No raw prompt, screenshot, credential,
personal path, external data, or new execution authority may be persisted or
logged. Composition events must fail safe by retaining text instead of sending.

## Test plan

- Verify plain Return sends one non-empty request.
- Verify Shift+Return, empty Return, busy state, `isComposing`, and key code 229
  do not accidentally submit.
- Verify React Flow owns ordinary wheel zoom while panning-on-scroll stays off.
- Verify a wheel-originated move updates the zoom output, enters manual mode,
  and suppresses automatic resize fitting until reset or content change.
- In a real browser, verify both wheel directions, Graph-local scroll capture,
  outside-Graph page scrolling, Return send, and Shift+Return multiline input.

## Verification commands

```bash
npm run test:frontend -- src/App.test.tsx src/features/command-center/components/OperationalTopologyAdapter.test.tsx src/features/command-center/CommandCenterPage.test.tsx
npm run format:frontend
npm run lint:frontend
npm run typecheck
npm run test:frontend
npm run build:frontend
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
npm run verify
shasum -a 256 src/features/command-center/components/TopologyStructuredView.tsx
git diff --exit-code -- src/features/command-center/components/TopologyStructuredView.tsx
git diff --exit-code -- src-tauri package.json package-lock.json
```

## Risks

- A naive Return handler could send during IME confirmation, send twice through
  the form, or remove multiline composition.
- Capturing wheel events outside the React Flow renderer could make the page
  feel trapped; leaving `preventScrolling` false disables ordinary React Flow
  wheel zoom in the installed version.
- Wheel interaction must not accidentally restore automatic fit during a later
  resize or bypass bounded zoom.

## Rollback or failure strategy

Revert only the bounded textarea handler, React Flow wheel props, help copy,
focused tests, and current requirement/decision record. If wheel ownership
cannot remain canvas-local or keyboard submission cannot be made IME-safe, stop
and record a failed gate rather than ship the interaction.

## Decisions made

- Use Return to send and Shift+Return for a newline, while ignoring composition
  Return and the WebKit 229 fallback.
- Use React Flow's native wheel handler with `zoomOnScroll` and
  `preventScrolling` enabled, not a custom inverted wheel implementation.
- Record the owner-directed wheel behavior as an additive decision and update
  current FR-039K; preserve dated historical evidence of the former policy.

## Discoveries

- In the installed React Flow version, ordinary wheel zoom requires both
  `zoomOnScroll` and `preventScrolling`; default wheel math already maps scroll
  up to zoom in and scroll down to zoom out.
- A first keyboard implementation treated modifier chords as sends. Final
  review tightened the contract to exact unmodified Return, so Shift, Alt,
  Control, Meta, IME composition, and WebKit key code 229 all retain the draft.
- The current jsdom suite can prove handlers, cancellation, React Flow props,
  viewport state, and exactly-once submission, but actual text insertion,
  wheel direction, and scroll chaining still require rendered interaction QA.

## Progress

- 2026-09-02: Inspected the two reported interactions, confirmed the valid
  predecessor gate, passed focused baselines, and classified the bounded work
  as Ready with advisories.
- 2026-09-02: Added failing regressions, implemented exact plain-Return submit
  and Graph-local wheel zoom, resolved the modifier-chord review finding, and
  passed the focused 87-test set plus the full 370-test frontend suite.
- 2026-09-02: Verified Shift+Return multiline input, Return submission, both
  wheel directions, canvas-local wheel capture, and outside-canvas page scroll
  in a rendered browser. Verified plain Return and both wheel directions again
  in a freshly bundled native Tauri application.
- 2026-09-02: Full repository verification, protected-path checks, reviews,
  documentation synchronization, and the deterministic completion workflow
  passed with nonblocking advisories.

## Acceptance criteria

- [x] Plain Return sends one enabled, non-empty conversation request
- [x] Shift+Return preserves multiline input
- [x] Composition Return does not submit
- [x] Scroll up over Graph zooms in and scroll down zooms out
- [x] Graph wheel use does not also scroll the page; outside-Graph scrolling remains available
- [x] Graph zoom bounds, readout, manual/automatic framing, and toolbar controls remain correct
- [x] Structured and protected native/dependency paths remain unchanged
- [x] Required automated, rendered, quality, and post-increment gates pass

## Final results

Verified complete with `PASS WITH ADVISORIES`. Exact unmodified Return now
submits one enabled, non-empty volatile mock request through the existing form
action. Shift/Alt/Control/Meta Return, IME composition, key code 229, empty
drafts, and busy state do not submit. React Flow now owns ordinary wheel input
only over its renderer, maps scroll up to zoom in and scroll down to zoom out,
updates the existing readout/manual viewport state, and leaves page scrolling
available outside the Graph.

Focused tests pass 87/87, the full frontend suite passes 370 tests, and
`npm run verify` passes. Browser and freshly bundled native Tauri interaction
checks confirm the requested directions and Return behavior. Structured remains
byte-identical at SHA-256
`ab939aa56d5b5c66fb39ce201ef40aa53422c4112015b9035b418c43dec06c0a`;
native and dependency paths have no diff. No trust boundary changed.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `PRODUCT_REQUIREMENTS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] Consolidated post-increment review
