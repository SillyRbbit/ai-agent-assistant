# Bounded agent parallelism

Status: Verified complete with advisories
Date: 2026-08-13
Gate ID: `agent-bounded-parallelism`
Plan: `docs/plans/2026-08-11-bounded-agent-parallelism.md`
Decision: D-091, preserving D-079 and D-082 through D-090
Baseline: clean synchronized `main` at `1f85264`

## Goal

Add one application-owned, fixture-only/no-I/O `BoundedParallel` selector to
`AgentOrchestrator`. The selector may retain several independent depth-one
specialist runs and accept their events through one same-thread multiplexed
boundary, while preserving deterministic result order, explicit failure
policy, bounded cancellation, attribution, and cleanup.

## Implemented concurrency model

The implementation retains multiple `RuntimeRun` values inside one
orchestrator and addresses every event by exact task and run identity. It does
not create a thread, async executor, provider-concurrency implementation,
worker, scheduler, durable queue, distributed service, or general workflow
engine. `AgentRuntime` and the sole/default `NativeAgentRuntime` are unchanged.

Only `AgentOrchestrator` creates child tasks. Every admitted child has a unique
task ID, distinct execution context and runtime-run identity, exact agent,
policy, and memory-profile attribution, isolated output and task-memory state,
an application-owned cancellation handle, and cooperative deadline. A runtime
run identity is the only implemented session-like separation; no provider
session registry exists.

The exact limits are depth one, two active children by default, three active at
the hard cap, three total specialist children, four tasks including Personal,
five runtime attempts including synthesis, zero automatic retries, eight
accepted events per run, 32 runtime/generic/workflow/audit records per
applicable family, a 120-second root lease, and a 60-second admitted-child lease
capped by the root deadline. Excess built-in work remains queued in the bounded
root-local set; invalid limits, duplicate requests, nested selection, excess
tasks/runs/events, and unsupported routes fail with typed behavior.

## Implemented scenarios and result handling

1. Research and Knowledge run independently under `ContinuePartial`, followed
   by truthful Personal synthesis.
2. Coding and Security run independently; QA starts only after both succeed
   under `CancelDependentOnly`, followed by Personal synthesis.
3. Cloud Infrastructure and Systems Operations run independently; Security is
   the dependent stage under specialist-lane `FailFast`, followed by truthful
   complete or partial Personal synthesis while the root remains live.

Every workflow slot projects exactly `Succeeded`, `Failed`, `Cancelled`,
`TimedOut`, or `Skipped`. Storage, dependency transfer, cancellation sweep,
public projection, and synthesis use immutable catalog ordinal rather than
completion order or map iteration. Strict final synthesis identifies the
source agent, status, exact application-derived finding IDs, failures, and
unresolved issues; invalid or incomplete disclosure fails the root.

Root cancellation and expiry sweep live children in ordinal order before the
root or synthesis run. A failed cancellation retains the affected and later
runs in a closed-cancelling state and resumes from the exact cursor on trusted
ingress. Cancelling one independent child does not cancel unrelated siblings
under `ContinuePartial` or `CancelDependentOnly`; `FailFast` cancels the
remaining specialist lane. Rejected runtime identities are quarantined until
their cleanup succeeds, so a root cannot terminalize while a rejected run is
still live.

## Preserved security and authority boundaries

All inputs and results are bounded, strict, versioned fixture data. Unknown
fields, invalid identity, authority claims, unsupported references, malformed
or oversized output, stale or cross-run events, and late events fail closed.
Workflow/audit records are content-free and non-authorizing. Memory remains
workflow-local: Research and Knowledge retain only their existing isolated
profiles, other specialists remain memory-disabled, and terminal task memory
is cleaned.

No provider, live model, provider session, tool request or execution, policy
permission, approval request or dispatch, general audit logger, persistence,
filesystem/network/platform I/O, credential path, repository or infrastructure
effect, Tauri command, React consumer, IPC, dependency, manifest, lockfile,
process, remote worker, or distributed infrastructure was added. The natural-
language claim filter is defense in depth for these fixtures, not a future
authorization boundary.

## Verification evidence

- Focused bounded-parallel library tests: 41 passed.
- Public D-091 contract: 41 passed.
- Strict all-target/all-feature Clippy: passed.
- Rust formatting: passed.
- All-target Rust: 481 passed, 0 failed, with one intentionally ignored opt-in
  real-Hermes probe.
- `npm run verify`: passed, including 124 frontend tests and 249 passed Rust
  library tests with one intentionally ignored probe, all integration
  contracts, frontend builds, and the Tauri no-bundle release build.
- Final code review: `PASS WITH ADVISORIES`, no completion blocker.
- Independent architecture and security review: `PASS WITH ADVISORIES`, no
  completion blocker.
- Preliminary session-end inventory: passed with no staged files or conflicts.
- Final post-documentation documentation, repository, security, diff, and
  session-end checks: passed. Prettier corrected only `ROADMAP.md` and
  `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md` before the passing rerun.
- Deterministic finalization: completed; marker status is `complete`, `valid:
true`, and `PASS WITH ADVISORIES`.

Completion result: `PASS WITH ADVISORIES`. `D091-TD-01` is the Low
5,188-line private-lifecycle debt to decompose before concurrency/lifecycle
expansion. `D091-TD-02` is the Low six-unused-public-error-variant debt to
resolve before IPC/public expansion. `D091-TD-03` records that the lexical
fixture-claim filter is defense in depth, not authorization, and blocks live/
provider/effect reuse. `LEGACY-TD-01` is the Medium pre-existing legacy
`start_runtime_run` cancellation-error orphan risk that blocks live/external-
runtime or legacy-provider work. `D091-TD-04` records the same-thread,
cooperative, per-orchestrator model's lack of app-global provider-budget/session
proof. None blocks D-091 completion or expands its authority.

Next-increment readiness is `Blocked`: no owner-selected plan follows D-091.
Any live/provider/app-global concurrency, hard preemption, additional workflow,
tool/effect, IPC/UI, or scheduling work requires a separate decision and Ready
plan.

## Rollback

Before publication, remove the bounded-parallel domain, private catalog and
validation modules, private orchestrator lifecycle, public contract, exact
deadline failure code, and the single D-088 framing match arm. Restore the
orchestrator/module integration and D-091 current-state records. Preserve all
D-083 through D-090 source and historical evidence. No external state or data
requires rollback.
