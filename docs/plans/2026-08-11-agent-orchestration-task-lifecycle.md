# Agent task orchestration and first bounded delegation

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-12
Decision: D-083, preserving D-082 and D-079
Gate ID: `agent-task-orchestration`

## Goal

Implement the smallest application-owned task lifecycle and orchestrator above
the existing one-run `AgentRuntime`, then prove one deterministic no-I/O
Personal Assistant -> Research Agent -> Personal Assistant synthesis flow and
the direct-response path.

## User-visible outcome

No shipping UI or live assistant behavior changes. The Rust core gains a
deterministic, framework-neutral application-service contract that can create a
Personal Assistant root task, either finish it directly or run one Research
child, return the attributed child outcome, resume Personal Assistant synthesis,
and cancel without orphaned work.

## Scope

- Closed task IDs, lineage, bounded task content, status, result, failure, and
  cancellation types.
- Trusted execution context generated from application state.
- A generic application-owned `AgentOrchestrator<R: AgentRuntime>` with a
  default Native constructor.
- Exact Personal Assistant root and Personal Assistant -> Research route.
- One total child per root, one active child, depth one, no replenishment.
- Orchestrator-owned runtime-event routing and result accumulation.
- Direct response and one deterministic delegated synthesis flow.
- Test-only extraction and reuse of the existing deterministic
  `MockAgentRuntime`.
- Architecture, roadmap, current-memory, increment, and review closeout.

## Explicit non-goals

- No provider, model, gateway, process, network, credential, Hermes, external
  framework, runtime selector, or automatic fallback.
- No `AgentRuntime` or `NativeAgentRuntime` contract change.
- No `agent.delegate` tool, runtime control event, host tool, tool execution,
  policy/approval change, audit persistence, memory store, policy-profile ID,
  memory-namespace ID, storage migration, Tauri IPC, React, or UI.
- No activation of Knowledge & Document, Coding, QA & Validation, Security &
  Risk, Cloud Infrastructure, Systems Operations, or Workflow Automation.
- No parallelism, recursion, scheduler, retries, background autonomy, durable
  tasks, timestamps, deadlines, or wall-clock dependency.
- No Phase 4 or later work.

## Pre-implementation baseline and constraints

- Phase 1 is published at `f42a6c7`; gate `agent-definition-registry` is
  complete and fingerprint-valid, and its focused contract passes 6/6.
- `AgentRegistry` contains nine inert definitions. Only Personal Assistant and
  Research have the descriptive `Initial` marker.
- `AgentRuntime` constructs one run and accepts closed runtime events;
  `NativeAgentRuntime` is sole/default and remains unwired.
- A runtime run has no continuation input. The delegated flow therefore uses
  at most three sequential runs: initial Personal, Research child, and fresh
  Personal synthesis.
- Before this increment, `MockAgentRuntime` was private to one integration test
  and was an event sink, not an autonomous executor.
- Before this increment, no Rust task, work-item, operation, product session,
  orchestrator, or delegation type existed. The TypeScript conversation
  session and visible frontend mock remain separate and unchanged.

## Current-state evidence

- `git status --short --branch`: clean synchronized `main` before plan edits.
- `git rev-list --left-right --count HEAD...@{upstream}`: `0 0`.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked`:
  6 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked`:
  20 passed.
- D-083 explicitly combines the former roadmap Phases 2 and 3 while preserving
  D-082's boundaries.

## Files expected to change

Implementation and focused tests:

- `src-tauri/src/agent/task.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/tests/support/mod.rs`
- `src-tauri/tests/support/mock_agent_runtime.rs`
- `src-tauri/tests/agent_runtime_contract.rs`
- `src-tauri/tests/agent_orchestration_contract.rs`

Decision, architecture, roadmap, plan, and closeout:

- `DECISIONS.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/PROJECT_DIRECTION.md`
- `ARCHITECTURE.md`
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `docs/plans/2026-08-11-agent-orchestration-task-lifecycle.md`
- `PLANS.md`
- `NEXT_STEPS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `CHANGELOG.md`
- `docs/increments/agent-task-orchestration.md`
- `docs/reviews/2026-08-12-agent-task-orchestration-post-increment-review.md`

`TROUBLESHOOTING_LOG.md` changes only if a new durable troubleshooting outcome
occurs. Any path outside this list is a stop condition pending owner review.

## Affected components

- Application-owned agent task domain.
- Application-owned orchestration service above `AgentRuntime`.
- Test-only deterministic runtime fixture and contract tests.
- Current architecture and roadmap documentation.

## Interfaces and invariants

### Task domain

- `AgentTaskId`, `RootTaskId`, and `ParentTaskId` are application-generated
  closed identities; callers cannot choose lineage.
- `AgentTaskStatus` is exactly `Pending`, `Running`, `WaitingForChild`,
  `Completed`, `Failed`, or `Cancelled`.
- Allowed transitions are `Pending -> Running|Failed|Cancelled`,
  `Running -> WaitingForChild|Completed|Failed|Cancelled`, and
  `WaitingForChild -> Running|Failed|Cancelled`. Terminal states reject every
  transition; Research tasks never enter `WaitingForChild`.
- Each task has one bounded objective, optional bounded delegated context,
  exact agent/root/parent/depth attribution, and at most one terminal outcome.
- Objectives are at most 4,096 characters and 16,384 bytes; delegated context
  is at most 2,048 characters and 8,192 bytes; expected deliverables are at
  most 512 characters and 2,048 bytes; accumulated task output is at most 8,192
  characters and 16,384 bytes. Empty required content and prohibited controls
  fail closed.
- Content is accessible only through explicit domain getters; `Debug`, errors,
  and orchestration events redact it.

### Execution and delegation

- `AgentExecutionContext` binds exact agent, task, root, optional parent,
  runtime, depth, and current runtime-run identity. The orchestrator creates and
  validates it; untrusted callers cannot construct or override trusted fields.
- Policy-profile and memory-namespace identities are omitted until their
  enforcing phases.
- A public bounded `DelegationProposal` carries only target, objective,
  optional context, and expected deliverable. The orchestrator creates the
  trusted `DelegationRequest` from the active execution context.
- Delegation is rejected unless the exact source is the active Personal
  Assistant root at depth zero, the target is registered and `Initial`, the
  route is Personal Assistant -> Research, no output has been accepted from the
  current root run, and both child budgets remain available.
- Unknown, deferred, self, reverse, Research-originated, stale, mismatched,
  over-depth, over-total, or over-active requests fail before child allocation.
- Accepting delegation cancels only the initial root runtime run, moves the
  root task to `WaitingForChild`, and starts exactly one Research child.
- Terminal child success, typed failure, or cancellation becomes a bounded
  attributed `ChildTaskOutcome`. Unless root cancellation initiated it, the
  parent resumes and starts one fresh synthesis run with a canonical bounded
  input derived from that outcome.

### Runtime and event ownership

- `AgentOrchestrator<R: AgentRuntime>` owns active `R::Run` values and routes a
  closed `RuntimeEventEnvelope` only to the exact bound task/run.
- Direct response uses one root run. Delegated success or failure uses no more
  than three total runs. At most two tasks and 32 runtime events exist per root.
- Runtime output is accumulated transactionally within the task-output bounds.
- A `ToolProposal` is rejected and the affected task fails closed. It is never
  forwarded to a registry, policy, approval, or executor.
- Application events contain only typed identities, agents, statuses, failure
  codes, and outcome kinds—never objectives, context, output, or reasoning.
- Required success ordering is `RootTaskCreated`, `TaskStarted`,
  `DelegationRequested`, `DelegationAccepted`, `ChildCreated`, `ChildStarted`,
  `ChildCompleted`, `ResultReturned`, `ParentResumed`, `RootCompleted`.

### Cancellation

- Cancellation is terminal and idempotent at task and runtime boundaries.
- Root cancellation while waiting cancels the active child first, emits the
  child cancellation, then cancels the root; no observable orphan remains.
- Independent child cancellation returns a cancelled child outcome and resumes
  the parent. Root-initiated cancellation never resumes it.
- Late runtime events, child outcomes, and duplicate terminal transitions fail
  closed without mutating task state or event history.

## Implementation milestones

- [x] Milestone 0 — record D-083, synchronize phase ordering, complete fresh
      readiness/architecture/security review, and begin the gate.
- [x] Milestone 1 — implement bounded task identities, content, lifecycle,
      results, failures, limits, and trusted execution context.
- [x] Milestone 2 — implement generic orchestration, Native default,
      delegation, runtime routing, result return, synthesis resume, and exact
      cancellation.
- [x] Milestone 3 — extract the existing mock into test-only shared support and
      add unit/public contracts for direct, delegated, denied, failed, bounded,
      redacted, and cancelled behavior.
- [x] Milestone 4 — run applicable verification and independent reviews.
- [x] Milestone 5 — synchronize current memory, create closeout evidence, and
      finalize a valid post-increment marker.

## Security and privacy considerations

- Model/runtime text is untrusted data and never supplies trusted identity,
  route, status, limit, policy, memory, or cancellation fields.
- No content enters orchestration events, error variants, Debug output, logs,
  audit, persistence, IPC, or the WebView.
- Registry activation remains descriptive; the orchestrator owns the exact
  route allowlist and rejects every deferred role.
- No tool, shell, filesystem, network, process, provider, credential, memory,
  policy, approval, or execution authority is introduced.
- All normal tests remain deterministic, no-I/O, and independent of clocks,
  threads, external models, Hermes, or platform behavior.

## Test plan

- Task units cover every accepted and rejected transition, terminal replay,
  cancellation, single terminal outcome, bounds, controls, and redaction.
- Orchestrator units use controlled registries/state to cover missing target,
  deferred target, self/reverse/Research source, stale context, depth, active
  and total child limits, no replenishment, output-before-delegation, run/task
  identity mismatch, cumulative output, tool proposal, and state atomicity.
- Public contract proves direct response, the exact deterministic delegated
  event order, attributed Research result in the canonical synthesis request,
  deterministic repetition, child failure and fallback synthesis,
  cancellation before child, cancellation while child runs/parent waits,
  propagation order, late-event rejection, no orphan, and Native construction.
- Existing registry, runtime, native gateway, and visible frontend contracts
  remain regression evidence.

## Verification commands

```bash
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::task::tests::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

No manual UI or target-platform check is required because the increment adds
only target-neutral Rust application/domain behavior and documentation. Linux
and target-Mac CI remain post-publication evidence, not a local completion
substitute.

## Risks

- A task may accidentally be treated as one runtime run; the explicit three-run
  lifecycle and stage tests prevent this.
- Trusted lineage could leak into caller-supplied proposals; constructors stay
  private and stale/mismatch tests fail closed.
- Cancellation ordering could orphan a child; child-first propagation and
  exact event-order tests are mandatory.
- Accumulated output or synthesis input could exceed runtime limits; both
  domain and final runtime-request construction remain bounded and tested.
- Generic orchestration could broaden `AgentRuntime`; any required runtime or
  native-runtime source change stops this increment for plan review.

## Rollback or failure strategy

Before publication, restore only the declared documentation and existing mock
test, remove the new task/orchestrator/test-support/contract/closeout files, and
remove their `mod.rs` exports. After publication, use a bounded revert plus an
additive decision superseding D-083; do not erase decision history. There is no
data, migration, dependency, IPC, configuration, credential, process, network,
or external-state rollback.

## Decisions made

- D-083 combines former roadmap Phases 2 and 3 while preserving D-082.
- One delegated workflow uses three sequential runtime runs and two tasks.
- Delegation is an application-service call, never a host tool.
- Policy and memory identities are deferred rather than represented falsely.
- The deterministic mock remains test-only and shared between contract suites.

## Discoveries

- `AgentRuntime` is a one-run event sink; it does not source events or resume a
  completed run.
- The TypeScript conversation session and browser mock are separate visible
  demo state and are not a Rust task/session foundation.
- No reusable task, work-item, operation, or Rust product-session type exists.
- A deterministic ID reused by every orchestrator instance would permit stale
  cross-workflow event binding. The final design assigns a checked process-
  unique workflow namespace and derives every task/run/request identity from
  it while tests compare normalized topology rather than raw IDs.
- Runtime-event attribution and sequence validation must precede capability and
  event-limit handling; otherwise a foreign event can mutate or terminate live
  work. The implementation and adversarial cap/tool tests pin this order.
- Runtime cancellation can report a contradictory terminal or nonterminal
  status. Terminal reports are reconciled to typed task failure without a live
  run; nonterminal contradictions and cancellation failures preserve the
  coherent live workflow for explicit recovery instead of orphaning work.

## Progress

- 2026-08-12 — Phase 1 published at `f42a6c7`; focused registry and runtime
  baselines pass.
- 2026-08-12 — Owner explicitly combined former Phases 2 and 3; D-083 and this
  exact bounded plan record the amendment.
- 2026-08-12 — Fresh review returned Ready with advisories; documentation checks
  passed and gate `agent-task-orchestration` became active.
- 2026-08-12 — Implemented `AgentTask`, trusted execution context,
  `AgentOrchestrator<R>`, the Native constructor, and shared test-only mock
  without changing `AgentRuntime`, `NativeAgentRuntime`, IPC, UI, or dependencies.
- 2026-08-12 — Added 11 task units, 3 orchestrator units, and 22 public
  orchestration contracts covering direct/delegated output, attribution,
  denials, limits, identity isolation, failure, redaction, and cancellation.
- 2026-08-12 — Independent architecture and code reviews returned PASS after
  identity, event-order, and cancellation-state corrections. The complete
  applicable local validation passed and closeout records were synchronized.

## Acceptance criteria

- [x] D-083 and both roadmaps consistently record the combined increment.
- [x] Task/context/result/event/error types are closed, bounded, typed, and
      redacted with the exact state machine.
- [x] `AgentOrchestrator` alone creates tasks and enforces the exact route,
      depth, total-child, active-child, task, run, and event limits.
- [x] Direct Personal response and one deterministic Personal -> Research ->
      Personal synthesis pass through `AgentRuntime` without changing it.
- [x] Research success/failure/cancellation returns a bounded attributed outcome
      and no child remains orphaned.
- [x] All denial, invalid-transition, limit, late-event, cancellation, and
      redaction contracts pass.
- [x] Native remains sole/default; the seven deferred roles remain inactive;
      Hermes and every external/framework/provider/tool path remain absent.
- [x] Applicable validation, independent reviews, documentation sync, and the
      completion marker pass.

## Final results

Implemented the closed application-owned task and orchestration foundation
without changing the runtime contract or any shipping application path. A root
may finish directly or use exactly one Research child followed by a fresh
Personal synthesis run. Exact trusted context, route, task/run/event bounds,
attribution, failure, cancellation, and redaction invariants are enforced.

Focused evidence passes: 11/11 task units, 3/3 orchestrator units, 22/22 public
orchestration contracts, 20/20 runtime contracts, 6/6 catalog contracts, and
10/10 gateway contracts. The all-target Rust suite passes 205 tests with the
one explicitly opt-in real-Hermes probe ignored. Formatting, all-target check,
strict Clippy, full repository verification, documentation/repository health,
security scan, diff hygiene, and session-end inventory pass. Independent
architecture and code reviews report PASS. The completion decision is `PASS
WITH ADVISORIES` only because the verified increment remains uncommitted and
cross-target CI is post-publication evidence.

No manual UI check was required: no Tauri, React, provider, model, process,
permission, or user-visible behavior changed. No later plan is Ready.

## Documentation updates

- [x] `ARCHITECTURE.md`
- [x] `docs/PROJECT_DIRECTION.md`
- [x] `ROADMAP.md` and `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- [x] `PLANS.md`
- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] Increment and review records
