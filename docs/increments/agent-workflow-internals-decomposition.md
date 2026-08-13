# Private agent workflow internals decomposition

Status: Verified complete with advisories
Date: 2026-08-13
Gate ID: `agent-workflow-internals-decomposition`
Plan: `docs/plans/2026-08-12-agent-workflow-internals-decomposition.md`
Decision: D-089, preserving D-079 and D-082 through D-088
Baseline: clean synchronized `main` at `3dccb81`

## Goal

Clear D-088's exact next-increment maintainability blocker by moving its private
Cloud/Systems lifecycle out of the `AgentOrchestrator` facade and splitting its
fixture catalog, transfer framing, and strict parser/validation implementation
without changing behavior, contracts, authority, or verified results.

## Implemented boundary

`agent/orchestrator/infrastructure_operations_workflow.rs` now owns only the
private D-088 lifecycle. The public Cloud and Systems start methods remain on
`AgentOrchestrator` and delegate to private `pub(super)` implementation methods.
The facade still owns the workflow state, root and child tasks, runs, event
acceptance, workflow selection, shared capacity, governance reconciliation, and
cancellation entry points.

`agent/infrastructure_operations/{catalog,framing,validation}.rs` now own the
immutable built-in fixture constructors, bounded stage-transfer framing, and
strict wire parsing/validation. `agent::infrastructure_operations` retains all
public constants, types, accessors, errors, serialized spellings, and redacted
read models. All four new modules are private; no new public or `pub(crate)`
surface was added.

## Preserved behavior and authority

The extraction changes no fixture byte, validation order, bound, error,
transition, event/audit order, cancellation behavior, activation state, policy
or memory profile, delegation route, runtime, approval, or execution
disposition. D-088 remains two separate sealed fixture-only/no-I/O selectors.
D-086 and D-087 remain separate bespoke workflows. `AgentOrchestrator` remains
the sole task/run/child/cancellation/selector authority.

No workflow trait, state-machine framework, DAG runner, DSL, dynamic registry,
scheduler, event bus, background work, Workflow Automation contract or
activation, tool, policy change, approval dispatch, executor, dependency,
provider, credential, filesystem/network I/O, Tauri/React IPC, persistence,
parallelism, external runtime, or device effect was added. Native remains
sole/default and every execution disposition remains `NotAttempted`.

## Size and review evidence

- `orchestrator.rs`: 9,424 to 6,817 lines.
- Private D-088 lifecycle child: 2,664 lines.
- `infrastructure_operations.rs`: 4,489 to 2,416 lines.
- Private catalog/framing/validation modules: 169, 405, and 1,535 lines.

Independent semantic comparison found every moved catalog, framing, and
validation function body token-identical to the baseline. Every moved lifecycle
body also matches; path changes are limited to the private module/import seam
and three equivalent helpers extracted from the prior inline root-cancellation
branch. The public D-088 contract passes 25/25.

Final `npm run verify` passes with 124 frontend tests, 195 Rust library tests,
the unchanged integration-contract counts, and the Tauri no-bundle release
build. A separate all-target Rust run passes 362 tests with one intentional
ignored opt-in real-Hermes probe. The first full verification attempt stopped
at formatting for the active plan only; targeted Prettier corrected that file,
and the complete rerun passed.

Quality result: `PASS WITH ADVISORIES`. The exact D-088
`blocks_next_increment` decomposition finding is cleared. The remaining size of
the 6,817-line facade and 2,664-line private lifecycle is a non-blocking
maintainability advisory: a future workflow must preserve separate private
ownership and must not create a general engine.

Final documentation, repository, security, diff, session-end, and quality
checks pass after the last documentation edit. The deterministic completion
marker is finalized from this exact workspace fingerprint with no subsequent
file edit.
Workflow Automation remains Deferred/Blocked and requires its own fresh
decision, complete Ready plan, and architecture/security/readiness review.

## Rollback

Before publication, move the D-088 bodies back to their original parent files
and remove only the four private modules and D-089 current-state records.
Preserve all D-088 contracts and historical evidence. This rollback is
source-only; the increment created no external or device state.
