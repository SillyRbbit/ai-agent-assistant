# Typed Workflow Automation proposals and manual sealed dispatch

Status: Verified complete with advisories
Date: 2026-08-13
Gate ID: `agent-workflow-automation-proposals`
Plan: `docs/plans/2026-08-11-workflow-automation.md`
Decision: D-090, preserving D-079 and D-082 through D-089
Baseline: clean synchronized `main` at `140f05b`

## Goal

Activate Workflow Automation only for one strict fixture-only proposal route
and allow a complete validated A-D proposal to produce a process-local,
take-once token that a trusted manual application call may consume in a fresh
`AgentOrchestrator`. Keep template E and every tool/approval step
non-executable, and add no general workflow engine, scheduler, persistence,
provider, IPC/UI, or effect.

## Implemented proposal domain

`agent::workflow_automation` owns five immutable V1 templates:

1. Research brief: Research -> Knowledge -> Personal synthesis.
2. Code quality review: Coding -> QA -> Security -> Personal synthesis.
3. Infrastructure assessment: Cloud -> QA -> Security -> Personal synthesis.
4. Systems incident analysis: Systems -> QA -> Security -> Personal synthesis.
5. Document to action plan: Knowledge -> Workflow Automation proposal ->
   Personal synthesis.

The first four derive `ReadyForManualDispatch`; the fifth derives
`ProposalOnly`. Strict proposal validation covers identifiers, version, fields,
template equality, ordering, dependencies, cycles, agents and activation,
registered tool names/versions/arguments, approval-checkpoint binding, fixed
limits, and authority claims. Unknown or duplicate data fails closed. A known
tool plus a valid checkpoint remains `ToolStepsUnavailable`; no policy,
approval, audit subject, executor, or effect is created.

The exact limits are four steps, three agent-task steps, two recognized proposed
tool steps but zero executable tool steps, three dependencies per step, zero
retries, zero nested workflows, a 120-second monotonic lease, an 8,192-byte
catalog, 24,576-byte planner input, 32,768-byte synthesis input, and
16,384-byte proposal and synthesis results. Native encoded-request regressions
preserve the existing 65,536-byte boundary.

## Proposal lifecycle and activation

The proposal selector is exactly Personal -> Workflow Automation -> Personal
synthesis. D-090 narrows the provisional D-082 QA/Security proposal-review
topology for this phase; no QA or Security task or review is claimed. The
selector has two tasks, one depth-one child, three runtime attempts, one active
child, zero retries, 16 runtime/generic events, and eight workflow events and
matching content-free attribution records.

Workflow Automation now uses instruction source V2 and is `Initial` only for
this sealed unwired selector. Its `WorkflowProposalOnlyV1` policy profile,
`MemoryDisabledV1` memory profile, empty tool eligibility, and generic route
denial remain unchanged. Runtime tool proposals fail closed into truthful
partial synthesis; catalog activation alone cannot dispatch.

## Manual dispatch and deadline

A complete truthful A-D synthesis exposes one non-serializable, non-`Clone`,
redacted `WorkflowManualDispatch`. It is bound to proposal/template/catalog,
planner attribution, and the original monotonic deadline. It can be taken once
and is consumed on every destination success or error. A fresh orchestrator
with a live Personal root may map it only to the corresponding existing
D-086/D-087/D-088 fixture-only/no-I/O selector using application-owned fixture
requests. Template E never issues a token.

The deadline propagates into the destination lease. Trusted event, successor,
cancellation, explicit deadline-check, and mutable manual-result ingress check
it cooperatively. The first ingress after expiry performs child-first
cancellation, starts no successor, and records a typed terminal outcome. This
does not preempt a synchronous `runtime.start` already in flight and uses no
timer, thread, scheduler, or background worker.

## Preserved boundaries

No shell, script, arbitrary code, dynamic step kind, generic DAG runner,
scheduler, recurring/startup/background execution, webhook, remote trigger,
parallelism, persistence, tool execution, approval request/dispatch, repository
or platform operation, filesystem/network/credential access, external
communication, provider, dependency, Tauri command, React consumer, IPC,
external runtime, or device effect was added. Native remains sole/default.
Existing sealed A-D workflows keep their own contracts and lifecycle authority.

## Verification evidence

- Focused Workflow Automation tests: 12 passed (seven domain/catalog/validation
  tests and five lifecycle/deadline tests).
- Public Workflow Automation contract: 18 passed.
- Strict all-target/all-feature Clippy: passed.
- All-target Rust: 393 passed, 0 failed, with one intentionally ignored opt-in
  real-Hermes probe.
- `npm run verify`: passed, including 124 frontend tests, 208 Rust library
  tests, all integration contracts, TypeScript/Vite, and the Tauri no-bundle
  release build.
- Final code review: `PASS WITH ADVISORIES`, no completion blocker.
- Independent architecture/security closeout review: `PASS WITH ADVISORIES`,
  no completion blocker.
- Final documentation, repository, security, diff, session-end, and marker
  checks: passed.

Completion result: `PASS WITH ADVISORIES`. Advisories are the
cooperative deadline's inability to preempt an in-flight synchronous start;
future direct 15/16/17 event-cap cases; table-driven second-lease checks across
A-D and terminal variants; additional nested/synthesis duplicate-key and exact
multibyte/escaping boundary cases; and D-089's residual module-size debt. None
authorizes a general engine or broader capability.

Next-increment readiness remains `Blocked`: there is no owner-selected Ready
plan. Executable tools, approval dispatch, template E execution, scheduling,
recurring/background work, persistence, UI/IPC, providers, and effects remain
deferred.

## Rollback

Before publication, remove the Workflow Automation domain/catalog/validation,
proposal and manual-dispatch private modules and public contract; restore the
V1 Deferred definition, shared tool-registry construction, orchestrator/module
integration, exact regression expectations, and D-090 current-state records.
Preserve all D-086 through D-089 source and historical evidence. No external
state requires rollback.
