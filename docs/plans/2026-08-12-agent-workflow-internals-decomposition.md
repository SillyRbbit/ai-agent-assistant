# Agent workflow internals decomposition

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-12
Decision: D-089
Gate: `agent-workflow-internals-decomposition`

## Goal

Clear D-088's recorded next-increment blocker by decomposing the private
infrastructure/operations workflow lifecycle and parsing components without
changing product behavior, public contracts, authority, or verified results.
This prerequisite must complete before a fresh Workflow Automation readiness
review.

## User-visible outcome

None. The Rust multi-agent core remains unwired to Tauri and React. Existing
D-086, D-087, and D-088 deterministic workflows behave identically.

## Scope

- Extract D-088-specific lifecycle helpers from `orchestrator.rs` into one
  private child module.
- Split D-088 immutable catalog construction, bounded transfer framing, and
  strict output parsing/validation into private child modules.
- Preserve public type paths and keep `AgentOrchestrator` as the sole owner of
  tasks, runs, runtime events, child creation, cancellation, and selection.
- Run D-086, D-087, D-088, generic orchestration, governance, runtime, and full
  repository verification.
- Record before/after line counts and independent architecture, security, code,
  and technical-debt reviews.

## Explicit non-goals

- No Workflow Automation proposal, template, step, dependency, run, result,
  validation, activation, or execution.
- No workflow trait, general state machine, DAG runner, DSL, scheduler, event
  bus, dynamic registry, recursive workflow, or macro-generated engine.
- No Research/Knowledge or Engineering refactor; their tests are regression
  evidence only.
- No changed tool schema, implementation, policy, approval, governance, audit,
  memory, document, runtime, task, registry, activation, or delegation.
- No dependency, manifest, lockfile, Tauri/React, IPC, storage, permission,
  configuration, provider, external runtime, filesystem/network access,
  persistence, parallelism, background work, or device effect.
- No rename, cleanup, semantic fix, bound adjustment, fixture change, error
  rewrite, or test-expectation update bundled with the move.

## Existing behavior and constraints

The published baseline is clean and synchronized at `3dccb81`. The D-088
marker was `complete` and `valid: true` on that exact baseline. The authorized
D-089 planning diff is expected to make the old marker's live workspace
fingerprint report `valid: false`; that does not rewrite its published evidence.
The D-088 review records `blocks_next_increment: true` because the private
orchestrator is 9,424 lines and `infrastructure_operations.rs` is 4,489 lines.
Another workflow or tool path must not be added until those internals are
decomposed.

`AgentOrchestrator` owns one Personal root, exact live task/run binding, one
selected workflow, child creation, bounded runtime/generic events, governance
reconciliation, terminal transitions, and child-first cancellation. D-088's
Cloud and Systems selectors are distinct, sequential, depth-one,
one-active-child, zero-retry, fixture-only/no-I/O paths.

The domain owns strict `deny_unknown_fields` wire contracts, catalog-issued
provenance, exact byte/scalar/list limits, credential/authority/effect
defense-in-depth guards, and redacted read models. Moving them must not reorder
validation or change an error. Child modules stay private. Parent-called
functions may be `pub(super)`; no new `pub(crate)` or public authority is
allowed.

## Current-state evidence

- Published Git baseline: clean `main...origin/main` at `3dccb81`; the current
  workspace contains only the expected D-089 planning/current-state diff before
  source work begins.
- D-088 gate on the published baseline: `status: complete`, `valid: true`; the
  live fingerprint is expectedly invalid after D-089 planning edits.
- `orchestrator.rs`: 9,424 lines.
- `infrastructure_operations.rs`: 4,489 lines.
- ToolRegistry: exactly two definitions and no executor.
- Workflow Automation: `Deferred`, tool-ineligible, memory disabled.
- Every governed execution disposition: `NotAttempted`.

## Files expected to change

Production:

- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/orchestrator/infrastructure_operations_workflow.rs`
- `src-tauri/src/agent/infrastructure_operations.rs`
- `src-tauri/src/agent/infrastructure_operations/catalog.rs`
- `src-tauri/src/agent/infrastructure_operations/framing.rs`
- `src-tauri/src/agent/infrastructure_operations/validation.rs`

Private unit tests may move with their implementation without changing fixture
values or assertions. No public integration test should change semantically.

Planning/current-state documentation:

- `DECISIONS.md`
- this plan
- `PLANS.md`
- `NEXT_STEPS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`

Closeout documentation, only from observed evidence:

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/increments/agent-workflow-internals-decomposition.md`
- `docs/reviews/2026-08-12-agent-workflow-internals-decomposition-post-increment-review.md`
- the current-state documents above
- `docs/plans/2026-08-11-workflow-automation.md`, only to record the actual
  prerequisite disposition; Workflow Automation stays Blocked pending its own
  accepted decision, complete Ready plan, and review.

`PRODUCT_REQUIREMENTS.md`, `SECURITY.md`, `SECURITY_CHECKLIST.md`, governance
matrix, manifests, lockfiles, Tauri, and frontend files remain unchanged.

## Affected components

- Private native-agent orchestration source ownership.
- Private infrastructure/operations fixture and validation ownership.
- Documentation of the D-088 prerequisite.

No product, policy, approval, audit, runtime, or user-visible component changes.

## Interfaces and invariants

### Orchestrator facade

`orchestrator.rs` retains `AgentOrchestrator<R>` and its fields, all public
methods, `AgentWorkflowSelection`, central runtime-event validation/dispatch,
and shared task/run creation, cleanup, bounds, cancellation entry points, and
runtime invocation.

The private child module owns D-088 start validation/preparation, Cloud/Systems
state transitions, remaining-capacity preflight, parsed terminal application,
QA/Security/synthesis continuation, partial/start failure, workflow-specific
event/audit recording, event-limit terminalization, and workflow-specific
cancellation helpers. The facade calls these only through `pub(super)` methods.

### Infrastructure/operations domain

The parent keeps every public constant, type, accessor, enum, error, and
redacted `Debug` path.

- `catalog.rs`: immutable Cloud/Systems fixture constructors, canonical request
  data, and sealed catalog binding helpers.
- `framing.rs`: bounded stage/synthesis serialization, predecessor transfers,
  stage projection, and partial-code labels.
- `validation.rs`: strict wire structs, typed deserialization,
  reference/provenance reconciliation, bounds, guards, capability consistency,
  and output parsers.

No serialized field, enum spelling, fixture byte, validation order, error
variant, bound, accepted sample, or rejected sample may change.

### Behavior preservation

- Public paths, signatures, variants, errors, and redaction remain compatible.
- Native remains sole/default.
- D-086 remains 3 tasks, 2 children, 4 runs, 1 active child, depth one, zero
  retries, and its exact record limits.
- D-087/D-088 remain 4 tasks, 3 children, 5 runs, 1 active child, depth one,
  zero retries, 32 runtime/generic events, and exact workflow/audit caps.
- Terminal successor input and capacity are prepared before runtime-event
  acceptance; continuation failure never replenishes a budget.
- Cancellation remains child-first with governance-before-runtime ordering and
  retryable runtime-cancel failure.
- Activation, profiles, tool eligibility, delegation, and `NotAttempted`
  execution results do not change.

## Implementation milestones

- [x] Milestone 0: complete review and begin the gate.
- [x] Milestone 1: extract the private D-088 orchestrator lifecycle; compile and
      run focused D-088 tests.
- [x] Milestone 2: split catalog, framing, and validation; run domain and
      adversarial parser tests.
- [x] Milestone 3: run D-086/D-087/D-088 and generic/governance/runtime
      regressions, strict Clippy, and `npm run verify`.
- [x] Milestone 4: independent reviews, evidence sync, and session/quality/
      post-increment gates.

## Security and privacy considerations

The move must not widen visibility, reorder validation, expose content through
`Debug` or errors, weaken guards, alter provenance, or move trusted identity
construction out of the orchestrator. No parser may use an intermediate map
that collapses duplicate keys. No caller gains catalog proof, task/run identity,
policy, approval, audit, or execution authority. Fixture string guards remain
defense in depth only and do not authorize live or consequential behavior.

## Test plan

- Formatting, all-target/all-feature compile, and strict Clippy.
- Infrastructure/operations and orchestrator unit tests.
- D-088 public workflow contract.
- D-086, D-087, generic orchestration, governance, definition/registry,
  runtime, memory/document, approval, and gateway regressions.
- Full all-target Rust, frontend, and Tauri verification through
  `npm run verify`.
- Compare baseline/final counts and file line counts; do not edit an assertion
  or fixture merely to make moved code pass.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::infrastructure_operations::tests::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_infrastructure_operations_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_research_knowledge_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_engineering_quality_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Then run the quality gate and finalize
`agent-workflow-internals-decomposition` with its exact review manifest.

## Risks

- Privacy/import churn could tempt wider internal APIs; stop instead.
- Terminal code movement can change borrow/drop or mutation timing; atomicity,
  continuation, and cancellation tests are mandatory.
- Parser movement can change validation order or errors; preserve bodies and
  move tests with implementation.
- A refactor can grow into a general engine; D-089 forbids that.
- Independent reviewers must distinguish movement from semantic edits.

## Rollback or failure strategy

Before completion, reintegrate only this increment's uncommitted moves through
explicit inverse patches; never reset, clean, stash, or discard unrelated work.
Rollback is source-only. If public behavior must change, stop, keep the
prerequisite Blocked, and record a separate issue/decision.

## Decisions made

- D-089 selects a behavior-preserving D-088 internals extraction.
- The facade retains authority and public APIs.
- Private child visibility is capped at `pub(super)`.
- Workflow Automation remains Blocked throughout this increment.

## Discoveries

- Existing A-D selectors are sealed bespoke state machines, not a reusable
  executor.
- Tool definitions have no implementations and approval never dispatches;
  actual governed tool execution remains separately blocked.
- D-088's debt finding is machine-readable and requires valid evidence before
  Workflow Automation implementation readiness.

## Progress

- [x] 2026-08-12: confirmed the clean synchronized published baseline and its
      valid D-088 marker; recorded the expected live-fingerprint invalidation from
      later D-089 planning changes.
- [x] 2026-08-12: inventoried exact lifecycle/parser seams and recorded D-089.
- [x] 2026-08-12: fresh architecture/security/readiness review returned Ready
      with only the expected published-marker fingerprint advisory; planning checks
      passed and gate `agent-workflow-internals-decomposition` began.
- [x] 2026-08-13: extracted the D-088 lifecycle into the exact private child
      module and split the catalog, framing, and validation/parser bodies into
      the three exact private domain modules. No public contract or behavior
      changed.
- [x] 2026-08-13: final `npm run verify` passed after one formatting-only first
      attempt and targeted Prettier correction of this plan. The independent
      all-target Rust run passed 362 tests with one intentional ignored probe.
- [x] 2026-08-13: independent code, architecture, security, and technical-debt
      review found no actionable defect and explicitly cleared D-088's exact
      next-increment decomposition finding, with one residual non-blocking size
      advisory.
- [x] 2026-08-13: final documentation, repository, security, diff, session-end,
      and quality checks passed after the final documentation edit; the
      deterministic marker is finalized from that exact fingerprint.

## Acceptance criteria

- [x] All new source modules are private and cross-module lifecycle visibility
      is no wider than `pub(super)`.
- [x] Public APIs, serialized contracts, fixtures, errors, limits, transitions,
      attribution, and redaction remain unchanged.
- [x] `orchestrator.rs` no longer contains D-088 lifecycle implementation
      bodies.
- [x] `infrastructure_operations.rs` no longer contains fixture construction,
      transfer framing, or strict wire/parser bodies.
- [x] D-086, D-087, D-088, generic, governance, runtime, full Rust, frontend,
      and Tauri verification preserve published behavior.
- [x] Independent review clears D-088's `blocks_next_increment` debt finding.
- [x] The completion marker is valid for the final workspace fingerprint.
- [x] Workflow Automation remains Deferred/Blocked and no tool execution exists.

## Final results

The behavior-preserving extraction is implemented. `orchestrator.rs` decreased
from 9,424 to 6,817 lines, with the D-088 lifecycle isolated in the exact
2,664-line private child module. `infrastructure_operations.rs` decreased from
4,489 to 2,416 lines; its private catalog, framing, and validation modules are
169, 405, and 1,535 lines. Total code is intentionally almost unchanged because
this increment moves ownership rather than deleting behavior.

Independent semantic comparison found every moved catalog, framing, and
validation function body token-identical to the `3dccb81` baseline. Every
moved lifecycle body also matches; the only path-level differences are the new
private module/import boundary and three equivalent helpers extracted from the
prior inline root-cancellation branch. No new public or `pub(crate)` authority
exists.

Final `npm run verify` passes with 124 frontend tests, 195 Rust library tests,
the unchanged integration-contract counts, and the Tauri no-bundle release
build. A separate all-target Rust run passes 362 tests with one intentional
ignored real-Hermes probe. The first `npm run verify` attempt stopped at
Prettier for this plan; the exact targeted formatting command corrected only
that file before the passing rerun. Final documentation, repository, security,
diff, session-end, and quality checks pass after the last documentation edit.
The deterministic completion marker is finalized from this exact workspace
fingerprint; no file edit follows finalization.

## Documentation updates

- [x] `DECISIONS.md`
- [x] selection plan/current-state documents
- [x] `ARCHITECTURE.md`, after implementation evidence
- [x] `CHANGELOG.md`, after implementation evidence
- [x] increment and post-increment review
