# Research/Knowledge demo volatile lifecycle core ExecPlan

Status: Verified complete with advisories
Increment: `research-knowledge-demo-volatile-lifecycle-core`
Last updated: 2026-08-27

## Goal

Add the smallest truthful Rust-backed interactive-demo prerequisite: one process-local, manually stepped, volatile host that drives the existing sealed D-086 Research -> Knowledge workflow through the production `AgentOrchestrator<NativeAgentRuntime>` boundary. It establishes lifecycle and cleanup ownership before any lifecycle Tauri IPC or Command Center connection.

## User-visible outcome

None in this increment. The host is deliberately unwired to Tauri and React. It prepares the later UI to truthfully show a fixed application-owned synthetic workflow without claiming that the Command Center, Conversations mock, and Rust acceptance workflows are already one connected proof.

## Scope

- Add one private Rust module, `research_knowledge_demo_lifecycle.rs`, the
  minimum `lib.rs` declaration, and a public Rust-only re-export/contract so
  strict `-D warnings` does not require fake application wiring or a broad
  dead-code allowance.
- Own exactly one volatile D-086 workflow at a time through a manually stepped `ResearchKnowledgeDemoHost`.
- Use only application-owned fixed objective, sources, synthetic runtime envelopes, and private success/failure scripts.
- Drive every run through the existing `NativeAgentRuntime`, `AgentOrchestrator::start_root`, `request_research_knowledge_workflow`, `accept_runtime_event`, and `cancel_task` boundaries.
- Project a small, content-free, bounded in-memory lifecycle journal and snapshot for later adaptation.
- Add focused Rust unit/contract coverage plus the documentation and review evidence required by the completion gate.

## Explicit non-goals

- No Tauri command, event, managed state, capability, CSP, permission, plugin, frontend client, React component, UI control, or rendered behavior.
- No timer, async task, thread, polling, background worker, automatic continuation, scheduler, queue, or generic workflow engine.
- No caller-selected agent, task, root, run, request, profile, runtime, workflow, objective, source, fixture, script, stage, outcome, or event.
- No provider, model, network, credential, tool execution, approval dispatch, persistence, durable audit, filesystem/document access, dependency, or device effect.
- No change to `AgentRuntime`, `NativeAgentRuntime`, `AgentOrchestrator`, the sealed D-086 workflow, current projection command, or Command Center fixture graph.

## Current-state evidence

- `research-knowledge-demo-projection-contract` is complete and valid at source-current `main`; it is read-only and has no lifecycle capability.
- D-086 already proves the selected fixture workflow, exact runtime event identity and sequence validation, child-first cancellation, late-event rejection, partial/failure containment, and strict final synthesis.
- `NativeAgentRuntime` is sole/default and can accept the existing closed application-owned runtime event envelope without a provider, model, network, process, or external runtime.
- F-01/F-02 exact returned-runtime identity validation and rejected-run quarantine apply to every orchestrator start. F-07, F-08, F-12, and F-15 are complete but do not authorize lifecycle IPC.
- The latest native nine-agent architecture review identifies private lifecycle ownership and module-size containment as prerequisites before a connected demo. This module keeps new ownership outside the orchestrator and frontend.

## Files expected to change

- `src-tauri/src/research_knowledge_demo_lifecycle.rs`
- `src-tauri/src/lib.rs`
- focused Rust unit tests in the new module and, only if needed for a stable public library contract, one dedicated integration contract test
- this plan, its increment record, applicable project memory, and final review

No Tauri configuration, capability, frontend, dependency, lockfile, or `src-tauri/src/agent/**` path is expected to change.

## Interfaces and invariants

The host is a private Rust owner with no deserializable caller input:

```text
ResearchKnowledgeDemoHost
  start() -> Result<DemoSnapshot, DemoLifecycleError>
  advance() -> Result<DemoTransition, DemoLifecycleError>
  cancel() -> Result<DemoTransition, DemoLifecycleError>
  snapshot() -> DemoSnapshot
```

`start` creates the only accepted root and D-086 request from private constants. `advance` submits only the next fixed, application-created runtime envelope; it never accepts an envelope, identity, or outcome from a caller. `cancel` targets only the internally owned active root. Production construction fixes the success script. Test-only construction may select a fixed failure script to prove terminal failure; script selection is never a future IPC input.

`DemoSnapshot` and journal entries are closed, versioned, and bounded. The
snapshot schema version is exactly `research-knowledge-demo-lifecycle-v1`; its
proof-boundary text is exactly `Command Center, Conversations mock, and Rust
acceptance workflows are separate deterministic proofs.` Its only visible
lifecycle values are:

```text
idle | research | knowledge | synthesis | succeeded | failed | cancelled | cleanup-pending
```

They retain the exact `DEMO MODE · SIMULATED AGENT DATA` disclosure, a
Rust-issued `u32` presentation epoch, and a `u8` monotonic revision that cannot
exceed the eight-entry journal cap. A start when the next epoch would overflow
fails closed as `unavailable`. The exact projected-entry kinds are
`research-started`, `research-completed`, `knowledge-started`,
`knowledge-completed`, `synthesis-started`, `completed`, `failed`, `cancelled`,
and `cleanup-pending`. Epoch and revision are display correlation only: they are never
accepted back by Rust and cannot represent a task, root, run, request, profile,
runtime, workflow, or authority token.

Snapshots, errors, and Debug output exclude task/run/request/root IDs, execution contexts, objective, sources, fixture evidence, output, memory, audit records, paths, URLs, reasoning, and raw internal errors. Errors close to `already-active`, `not-active`, `invalid-transition`, `cleanup-pending`, or `unavailable`.

The host owns an orchestrator until every run is terminal and rejected-run
cleanup succeeds. A `RuntimeCleanupPending` or cancellation failure retains the
orchestrator, enters `cleanup-pending`, and blocks start and replacement. A
later no-argument `cancel()` first retries cancellation of any still-live root,
then retries `retry_rejected_runtime_cleanup` when quarantine remains. It must
never discard a nonterminal/rejected run. Terminal cancellation is child-first,
starts no successor, and a late application-generated event is rejected before
journal or snapshot mutation.

If destruction itself encounters persistent cleanup failure, the owner is
retained until process exit and a private process-wide atomic sentinel blocks
every replacement `start`, `advance`, and `cancel`. There is deliberately no
production reset. A replacement object's `snapshot()` remains a host-local
snapshot; the closed `cleanup-pending` operation error is the authoritative
process-availability signal. A later Tauri adapter must retain one managed,
synchronized host and must not treat this destructor sentinel as a concurrency
coordinator.

## Implementation milestones

- [x] Confirm a clean baseline, begin the source increment gate, and retain this plan as the active scope.
- [x] Add the private volatile host and its fixed application-owned D-086 fixture/script constructors without modifying agent internals.
- [x] Add closed snapshot, transition, journal, error mapping, cancellation, late-event, and cleanup ownership tests.
- [x] Run focused, complete, architecture, security, code-health, debt, and readiness checks; synchronize documentation and finalize the gate.

## Security and privacy considerations

- The WebView does not exist in this increment. Later explicit user action is a UI behavior invariant, not proof of human presence; it may be acceptable only because this host is bounded, volatile, and has no consequential effect.
- The host must use `NativeAgentRuntime`, never the test-only `MockAgentRuntime`, in production code.
- F-01/F-02 are preserved by using, not copying around, `AgentOrchestrator` start and cleanup ownership. A new host-level error must not hide cleanup pending or permit a replacement run.
- F-12/F-07/F-08 remain frozen in this core increment. Any future IPC adapter must separately and atomically define its commands, event, client narrowing, event-loss/reorder behavior, static guard changes, and target-Mac evidence.

## Test plan

- Success: exact Research -> Knowledge -> synthesis ordering and terminal succeeded snapshot.
- Failure: private deterministic synthesis failure produces terminal failed state and no fabricated result/retry.
- Cancellation: root cancellation during Research, Knowledge, and synthesis is child-first, idempotent, and starts no successor.
- Late events/steps: post-terminal or post-cancellation ingress cannot change snapshot, revision, or journal count.
- Cleanup: returned-identity mismatch, cancellation failure, retained
  quarantine, blocked replacement, and successful retry are covered at this
  host without nonterminal drops. D-086 is sequential and cannot manufacture a
  second simultaneously live identity; exact duplicate-live rejection remains
  inherited from `AgentOrchestrator::start_runtime_run` and is independently
  covered by `agent_runtime_contract` and
  `agent_bounded_parallelism_contract`.
- Bounds/redaction: exact state/journal cap, epoch/revision exhaustion, closed error mapping, and Debug/serialization absence of internal identity/content.
- Regression: existing D-086 contracts remain unchanged.

## Verification commands

```bash
cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm run security:scan
git diff --check
```

On the target Mac, run the focused Rust lifecycle contract, strict Rust checks, and `npm run verify`. No rendered, IPC, viewport, theme, motion, focus, zoom, or resize check applies until the separate Tauri adapter/presentation increment; record each as `Not run`, not passed.

## Risks, rollback, and stop conditions

- Risk: a parallel lifecycle machine could drift from D-086. Mitigate by using the existing orchestrator as the sole task/run/event authority and projecting only its closed outcome.
- Risk: cleanup could be lost at host replacement. Mitigate with retained host ownership and an explicit `cleanup-pending` state.
- Roll back only the bounded lifecycle module, its tests, and associated documentation through a normal revert. There is no persistence or user data to migrate.
- Stop before implementation if any requirement needs a Tauri command/event, frontend, timer/thread/async worker, caller input, agent-internal change, new dependency/capability/CSP/permission, provider/model/network/credential, tool/approval, persistence/filesystem, or device effect. Such work requires a different separately approved plan.

## Decisions made

No new durable decision is accepted by this planning document. It applies the existing native-first and no-background-autonomy direction to a proposed bounded core; the owner must separately approve the exact source increment.

## Discoveries

- 2026-08-27: Readiness, architecture, and security review found the existing `NativeAgentRuntime` sufficient for application-owned synthetic envelopes; no demo runtime adapter or runtime trait change is required.
- 2026-08-27: Separating the manual Rust core from a later IPC adapter avoids combining lifecycle cleanup, event delivery/reordering, and rendered UI concerns in one security-sensitive change.

## Progress

- 2026-08-27: Owner approved this documentation-only planning increment after a read-only readiness review. The gate began from clean `main` at `8d9df6e`. No source, IPC, UI, dependency, or configuration work has begun.
- 2026-08-27: Documentation, repository, secret, diff, and session-inventory
  checks pass. Independent architecture, security, code-health, debt, and
  readiness review retain the plan's source-approval advisory; no source work
  began.
- 2026-08-27: The owner approved the exact source plan. The source gate began
  at `8d9df6e`, and the Rust-only lifecycle host, public library contract,
  fixed success/failure scripts, cancellation, bounded projection, and
  cleanup-quarantine fault coverage were implemented without changing
  `src-tauri/src/agent/**`, Tauri IPC, frontend, configuration, or dependencies.
- 2026-08-27: Focused evidence passes with nine module tests, one public
  contract test, and strict Clippy. Review found and closed one Drop-time F-02
  replacement gap with a process-wide fail-closed sentinel; final complete
  verification and gate closeout remain.
- 2026-08-27: Complete target-Mac `npm run verify` passes with 28 hook, 57
  repository, 247 frontend, 261 Rust library, and 243 Rust integration tests;
  one explicit opt-in Hermes probe remains intentionally ignored. Frontend and
  Tauri release no-bundle builds pass. Independent architecture, security, and
  code review report no completion blocker.

## Acceptance criteria

- [x] A source implementation plan has one bounded Rust-only goal, exact ownership, no-input interface, non-goals, files, tests, target-Mac boundaries, rollback, and stop conditions.
- [x] The plan uses the existing D-086 and Native boundaries without claiming a connected UI or live agent.
- [x] The plan explicitly preserves F-01/F-02 and defers F-07/F-08/F-12 IPC changes to a later increment.
- [x] Owner source-implementation approval is obtained before a source gate.

## Final results

The bounded Rust-only lifecycle prerequisite matches the approved scope and all
required checks pass. The result is `PASS WITH ADVISORIES`: the Drop sentinel
is deliberately not a future concurrency coordinator, a replacement snapshot
remains host-local while operations surface process quarantine, and no later
adapter/presentation plan is Ready. No Tauri or user-visible behavior changed.
