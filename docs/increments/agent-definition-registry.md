# Agent definition and registry

Status: Verified complete with advisories
Date: 2026-08-12
Gate ID: `agent-definition-registry`
Plan: `docs/plans/2026-08-11-agent-definition-registry.md`
Baseline: clean synchronized `main` at `48ab264`

## Goal

Implement the smallest framework-neutral application-owned catalog containing
the nine owner-selected roles, closed versioned instructions, descriptive
staged activation, and a deterministic immutable registry without adding task
execution or orchestration.

## Implemented boundary

- `AgentId` provides exactly nine closed canonical identities and exact ASCII
  slug parsing with a content-free unknown-ID error.
- `AgentDefinition` owns bounded display name and purpose, a closed
  `AgentInstructionSource`, and non-authorizing `AgentActivation` metadata.
- Nine exact V1 instruction sources are embedded application-owned Rust assets;
  no file, URL, environment, provider, plugin, or user instruction loading was
  added.
- Only Personal Assistant and Research are `Initial`. Seven specialist roles
  retain exact `Deferred(AgentActivationGate)` values. Every definition remains
  inert and unwired.
- `AgentRegistry` is one concrete immutable `BTreeMap` with deterministic
  listing, typed lookup, duplicate rejection, and crate-private validated test
  assembly.

## Preserved boundaries

The implementation adds no `AgentTask`, `AgentOrchestrator`, delegation,
selection service, gate evaluator, parallelism, tool assignment, policy or
approval authority, audit change, memory, provider/model, process/network,
credential, persistence, Tauri IPC, React/UI, dependency, or Hermes path.
`AgentRuntime`, `NativeAgentRuntime`, `InitialGatewayTurn`, the deterministic
runtime mock, and visible application behavior remain unchanged. Native remains
sole/default.

## Exact implementation files

- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/registry.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/tests/agent_definition_registry_contract.rs`

## Evidence

- Definition unit tests: 5 passed, 0 failed.
- Registry unit tests: 3 passed, 0 failed.
- Public definition/registry contract: 6 passed, 0 failed.
- Unchanged native runtime contract: 20 passed, 0 failed.
- Unchanged public gateway contract: 10 passed, 0 failed.
- Strict all-target/all-feature Clippy and Cargo check: passed.
- Complete all-target, repository, documentation, security, and session-end
  results are recorded in the post-increment review.

Quality result: `PASS WITH ADVISORIES`. No implementation defect or new
technical debt was found. The advisory is procedural: the verified change is
intentionally uncommitted, and Phase 2 remains Blocked pending publication,
fresh architecture/security/readiness review, and exact owner authorization.

## Rollback

Delete `definition.rs`, `registry.rs`, and the new contract test; restore only
the two module exports and this increment's current-state documentation. There
is no data, dependency, migration, IPC, UI, credential, provider, process, or
external-state rollback.
