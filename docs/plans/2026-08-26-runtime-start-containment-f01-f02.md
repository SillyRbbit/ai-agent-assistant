# Runtime-start containment F-01/F-02 ExecPlan

Status: Complete
Increment: `runtime-start-containment-f01-f02`
Last updated: 2026-08-26

## Goal

Apply the exact returned-runtime identity validation and rejected-run cleanup
ownership already proven by D-091 to every legacy runtime start, without
changing any sealed workflow result or adding a connected runtime surface.

## Current-state evidence

`RuntimeTurnRequest` owns the expected run/request identity. The legacy
`AgentOrchestrator::start_runtime_run` checks only initial status and can drop a
nonterminal run when rejection cancellation fails. The D-091-only
`start_bounded_parallel_runtime_run`, `reject_unbound_runtime_run`, and
`retry_rejected_runtime_cleanup` path already provides exact identity,
duplicate-live rejection, retained cleanup ownership, and retry behavior.

## Files expected to change

- `src-tauri/src/agent/orchestrator.rs`
- legacy workflow modules only if caller adaptation is required
- `src-tauri/tests/support/mock_agent_runtime.rs`
- focused legacy workflow contract tests
- this plan, its increment record, applicable project memory, and final review

## Interfaces and invariants

- Every returned identity must exactly equal the application-created request
  identity and must not duplicate a live run.
- A rejected nonterminal run is cancelled or retained in the existing
  quarantine; it is never dropped, trusted, or exposed as an active context.
- New and fallback starts fail with `RuntimeCleanupPending` while quarantine is
  non-empty.
- Only explicit cleanup retry may release a retained rejected run after a
  closed terminal cancellation outcome.
- Existing workflow state, error projection, limits, ordering, and fallback
  behavior remain unchanged after successful containment.

## Milestones

- [x] Confirm the source-current baseline and begin the gate.
- [x] Generalize the D-091 containment helper for all runtime starts.
- [x] Add root, child/continuation, synthesis, duplicate, late-event, and
      cleanup-failure regression coverage.
- [x] Run focused and complete source verification and independent reviews.
- [x] Synchronize project memory, write the final report, and finalize the gate.

## Explicit non-goals

No Tauri command/event, UI, CSP, capability, provider, model, network,
credential, tool execution, approval dispatch, persistence, filesystem access,
background autonomy, generic workflow engine, dependency, or external runtime.

## Verification

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract
cargo test --manifest-path src-tauri/Cargo.toml --test agent_research_knowledge_workflow_contract
cargo test --manifest-path src-tauri/Cargo.toml --test agent_engineering_quality_workflow_contract
cargo test --manifest-path src-tauri/Cargo.toml --test agent_infrastructure_operations_workflow_contract
cargo test --manifest-path src-tauri/Cargo.toml --test agent_workflow_automation_contract
npm run verify
npm run security:scan
git diff --check
```

Target-Mac Rust lint and all-target tests are required through `npm run verify`.
No rendered or device-effect manual check applies because the increment changes
no UI, IPC, platform adapter, permission, or effect.

## Risks, rollback, and stop conditions

The primary risk is changing fallback ordering across heterogeneous legacy
workflows. Preserve caller-owned terminal projection and test each workflow
family. Roll back the bounded increment through a normal revert if verified
workflow behavior changes. Stop if the work requires a runtime trait change,
new public API, IPC, dependency, permission, CSP change, provider, persistence,
or broader workflow refactor.

## Progress

- 2026-08-26: owner approved the exact F-01/F-02 native-only containment plan;
  clean baseline `1e5b10e` confirmed and gate activated.
- 2026-08-26: one universal containment helper replaces the weaker legacy
  helper; focused contracts, strict Clippy, 490 executed all-target Rust tests,
  and complete `npm run verify` pass with one intentional ignored Hermes probe.

## Final results

Implementation, focused/full verification, independent review, documentation
sync, and deterministic finalization pass. The result is `PASS WITH
ADVISORIES`; volatile orchestrator-lifetime ownership still blocks any future
external runtime, and no F-07 implementation plan is owner-approved or Ready.
