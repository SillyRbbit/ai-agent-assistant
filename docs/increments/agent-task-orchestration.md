# Agent task orchestration and first bounded delegation

Status: Verified complete with advisories
Date: 2026-08-12
Gate ID: `agent-task-orchestration`
Plan: `docs/plans/2026-08-11-agent-orchestration-task-lifecycle.md`
Decision: D-083, preserving D-082 and D-079
Baseline: clean synchronized `main` at `f42a6c7`

## Goal

Implement the smallest application-owned task lifecycle and orchestrator above
the existing one-run `AgentRuntime`, then prove both a direct Personal Assistant
response and one deterministic no-I/O Personal-to-Research-to-Personal flow.

## Implemented boundary

- `AgentTask` owns bounded objective/context/output, exact agent/root/parent/
  depth lineage, the closed lifecycle, and one typed terminal outcome.
- `AgentExecutionContext` is derived from live trusted task/runtime state and
  binds task, lineage, runtime, run, and request identity without adding inert
  policy or memory placeholders.
- `AgentOrchestrator<R: AgentRuntime>` owns one bounded root workflow: at most
  two tasks, three sequential runtime runs, one total and active child, depth
  one, and 32 runtime and orchestration events.
- The only route is Personal Assistant to Research. Delegation is an explicit
  typed application-service call, never a host tool or runtime control event.
- A direct root uses one run. Delegation terminally cancels the initial root
  run, starts one Research child, and starts a fresh Personal synthesis run
  from its bounded attributed success, failure, or cancellation outcome.
- Every event is checked against the exact active task/run/request/sequence
  before capability, content, or event-limit handling. Tool proposals are
  rejected at the text-only boundary.
- Cancellation is child-first and fail-closed. Terminal runtime contradictions
  become typed task failure; nonterminal contradictions or cancellation errors
  preserve coherent live state rather than orphaning work.

## Preserved boundaries

The implementation does not change `AgentRuntime`, `NativeAgentRuntime`,
`InitialGatewayTurn`, the catalog, policy, approval, audit, tools, storage,
credentials, manifests, lockfiles, Tauri IPC, React, permissions, or visible
behavior. Native remains sole/default. The other seven definitions remain
`Deferred`. No provider, model, external runtime, process, network, filesystem,
memory, scheduler, parallelism, Hermes, or device action was added.

## Exact implementation and test files

- `src-tauri/src/agent/task.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/tests/support/mod.rs`
- `src-tauri/tests/support/mock_agent_runtime.rs`
- `src-tauri/tests/agent_runtime_contract.rs`
- `src-tauri/tests/agent_orchestration_contract.rs`

## Evidence

- Task units: 11 passed, 0 failed.
- Orchestrator units: 3 passed, 0 failed.
- Public orchestration contract: 22 passed, 0 failed.
- Unchanged runtime contract: 20 passed, 0 failed.
- Unchanged definition/registry contract: 6 passed, 0 failed.
- Unchanged public gateway contract: 10 passed, 0 failed.
- All-target Rust: 205 passed, 0 failed, 1 explicitly opt-in real-Hermes probe
  ignored as designed.
- Strict all-target/all-feature Clippy, Cargo check, complete repository
  verification, Tauri no-bundle release build, docs/repository health, secret
  scan, diff hygiene, and session-end inventory passed.
- Independent architecture and code reviews: PASS with no remaining actionable
  finding.

Quality result: `PASS WITH ADVISORIES`. No implementation defect or new
technical debt remains. The advisory is procedural: the verified change is
uncommitted, supported cross-target CI is post-publication evidence, and no
later plan becomes Ready automatically.

## Rollback

Remove `task.rs`, `orchestrator.rs`, the new orchestration contract and shared
test-support files; restore the two module exports and the prior private runtime
mock; revert only this increment's decision amendment and current-state docs.
There is no data, dependency, migration, IPC, UI, credential, process, network,
or external-state rollback.
