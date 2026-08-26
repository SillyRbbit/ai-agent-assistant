# Native multi-agent end-to-end demonstrations

Status: Verified complete with advisories; marker complete and valid
Owner: Project owner
Last updated: 2026-08-25
Gate: `native-multi-agent-end-to-end-demonstrations`
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Create one deterministic, non-destructive demonstration and acceptance layer
for the implemented native nine-agent architecture. Reuse the verified
application-owned workflows and governance boundaries, expose one exact local
acceptance command, and record reproducible evidence that distinguishes test
doubles, fixture data, simulated operating-system results, and unavailable live
capabilities.

This increment proves the implemented architecture. It does not fill capability
gaps by adding a provider, tool executor, live data source, backend UI, or new
workflow engine.

## User-visible outcome

Repository users can run one named command and consult one linked evidence set
to understand:

- which of the twelve required demonstrations execute against deterministic
  fixtures and `MockAgentRuntime`;
- which governance behavior uses the real application-owned policy, approval,
  audit, task, and orchestrator code;
- which operating-system approval result is simulated in a macOS unit test;
- which capabilities remain unavailable, including a configured native
  provider, real external tools, live multi-agent IPC/UI, durable audit, and
  Hermes; and
- how every requested architecture acceptance criterion maps to executable
  evidence.

## Scope

- Add `npm run test:agent-acceptance` as the canonical composition of the Rust
  library tests and ten public native-agent contract suites.
- Strengthen the existing macOS approval-resolution unit test so rejection
  proves the approval is cleared, execution is `NotAttempted`, the originating
  task remains controlled, and the attributed rejection audit record exists.
- Strengthen the existing approved-document contract so the live Knowledge
  child creates a reusable shared-memory proposal that remains explicitly
  pending and unapproved through synthesis.
- Document all twelve demonstrations and their expected success, denial,
  cancellation, or partial-failure outcomes.
- Document the immutable synthetic fixtures already embedded in the public
  workflow catalogs and contract tests; do not copy them into a second fixture
  implementation.
- Record an acceptance matrix and exact test evidence, including honest
  advisory outcomes for boundaries the present architecture intentionally does
  not implement.
- Reconcile authoritative project-memory and roadmap records with the completed
  Command Center validation baseline and this active increment.
- Run focused acceptance validation, complete `npm run verify`, required
  documentation/security/diff checks, independent review, and the consolidated
  post-increment gate.

## Explicit non-goals

- No new agent, workflow selector, runtime trait, provider, model, networking,
  process, dependency, tool, executor, approval manager, policy engine, audit
  store, memory store, IPC command, React route, or Tauri capability.
- No live repository, document, browser, cloud, infrastructure, system,
  service, provider, credential, or operating-system mutation.
- No commit, push, merge, release, deployment, or publication.
- No successful approval-to-execution claim. Approval resolution remains
  application-owned and every execution disposition remains `NotAttempted`.
- No claim that same-thread retained-run event multiplexing is provider, thread,
  or operating-system concurrency.
- No claim that `MockAgentRuntime` is a configured `NativeAgentRuntime` provider
  merely because its closed runtime descriptor has `RuntimeId::Native`.
- No hidden bridge from a workflow approval checkpoint to D-090 manual dispatch.
  `MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS` remains zero.
- No automatic injection of a rejected approval into runtime output or automatic
  task terminalization. The current typed, task-bound resolution is documented
  exactly; changing feedback semantics requires a separate architecture plan.

## Existing behavior and constraints

- `AgentRegistry::built_in()` deterministically registers exactly the nine
  `AgentId::ALL` definitions. All nine catalog entries are currently marked
  `Initial`, but catalog activation is descriptive and grants no authority.
- `AgentOrchestrator` is the application-owned task creator and owns all
  workflow transitions. Specialists operate at depth one and cannot call the
  child-creation path as an authorized source.
- The sole/default runtime identity is `Native`. `NativeAgentRuntime` remains
  unwired to a provider or live model. Public workflow contracts use the
  deterministic test-only `MockAgentRuntime` whose descriptor intentionally
  reports the closed `Native` runtime identity.
- D-086 through D-091 are sealed, finite, fixture-only/no-I/O selectors. Their
  structured fixtures and outcomes are already exercised by public contract
  tests.
- Workflow Automation emits typed proposals. Tool and approval-checkpoint
  steps are recognized but non-executable; only complete A-D proposals can
  yield one process-local take-once token for manual dispatch into an existing
  sealed selector.
- The governance service evaluates real typed proposals through the real
  registry, policy, approval, and volatile audit boundaries, but no tool
  executor exists. Unknown tools and unauthorized specialist proposals fail
  closed before execution.
- The macOS approval test converts deterministic simulated dialog results into
  the same typed source-resolution path used by application code. It does not
  display or click a real native dialog.
- The deterministic Command Center is a separate frontend fixture projection;
  it has no backend multi-agent IPC and is not execution evidence for these
  demonstrations.
- Hermes remains Deferred/Blocked and absent from the runtime catalog.

## Current-state evidence

Before implementation, the exact ten public contract suites passed **198/198**
tests:

- `agent_definition_registry_contract`: 7
- `agent_runtime_contract`: 26
- `agent_orchestration_contract`: 22
- `agent_governance_contract`: 11
- `agent_memory_document_contract`: 10
- `agent_research_knowledge_workflow_contract`: 18
- `agent_engineering_quality_workflow_contract`: 20
- `agent_infrastructure_operations_workflow_contract`: 25
- `agent_workflow_automation_contract`: 18
- `agent_bounded_parallelism_contract`: 41

The baseline command exited zero on 2026-08-25. No repository file changed
during that inventory run.

The existing macOS unit
`agent_approval_source_resolution_retains_origin_without_execution` already
proved approved and rejected outcomes produce `NotAttempted`. Its rejected
branch did not yet assert pending-state cleanup, root-task control, or the exact
audit record; this increment adds only those assertions.

## Files expected to change

Implementation and test entry point:

- `package.json`
- `src-tauri/src/agent/orchestrator.rs` (`#[cfg(test)]` assertions only)
- `src-tauri/tests/agent_memory_document_contract.rs` (test assertions only)

Living plan and evidence artifacts:

- `docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md`
- `docs/demos/NATIVE_MULTI_AGENT_DEMONSTRATIONS.md`
- `docs/demos/NATIVE_MULTI_AGENT_FIXTURES.md`
- `docs/demos/NATIVE_MULTI_AGENT_ACCEPTANCE.md`
- `docs/increments/native-multi-agent-end-to-end-demonstrations.md`
- `docs/reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md`

Repository guidance and current-state memory:

- `TESTING_GUIDE.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `docs/design/NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md`
- `ARCHITECTURE.md`
- `SECURITY_CHECKLIST.md`
- `CHANGELOG.md`
- `DECISIONS.md` (additive D-092 evidence plus D-093 acceptance clarification)

No other file is authorized. `DECISIONS.md` receives additive evidence for the
already accepted D-092 decision and one new owner-approved D-093 acceptance
clarification; no prior decision text or status is rewritten.
`PRODUCT_REQUIREMENTS.md` and `TROUBLESHOOTING_LOG.md` remain unchanged unless
observed evidence creates a new durable requirement or troubleshooting fact;
that would require an explicit plan update before editing.

## Affected components

- Local npm command catalog.
- Rust unit and integration acceptance tests.
- Native multi-agent demonstration and fixture documentation.
- Architecture, roadmap, security-checklist, and repository-memory truth.
- Consolidated increment verification and review evidence.

## Interfaces and invariants

- `AgentRuntime`, `RuntimeId`, `NativeAgentRuntime`, and all production runtime
  behavior remain byte-for-byte unchanged.
- All nine `AgentDefinition` values, activation metadata, prompts, policy and
  memory profile bindings remain unchanged.
- `AgentOrchestrator` remains the only production task creator. No public
  child-construction or specialist-to-specialist delegation interface is added.
- Workflow Automation remains proposal-only. QA cannot approve; Security cannot
  authorize; neither gets policy, approval, execution, or audit authority.
- Unknown tools, forged task/agent/run identity, and document/memory boundary
  violations continue to fail closed.
- Cloud and systems fixtures remain static analysis only; Terraform apply,
  cloud mutation, service restart, and system modification remain absent.
- All governance records remain bounded and process-local. Documentation must
  not call them durable, unified, or production audit.
- Approval is not execution. Both approval dispositions retain
  `AgentExecutionDisposition::NotAttempted`.
- Fixture output must always be labeled fixture or simulated. No screenshot or
  UI state may be used to represent backend execution.

## Demonstration map

1. Personal Assistant direct response: one root, one run, no delegation.
2. Research brief: Personal -> Research -> Knowledge -> Personal, with exact
   source provenance and partial-failure cases.
3. Document knowledge: explicit approved `.txt`/`.md` content, bounded compare
   or summary, proposal-only shared knowledge, no silent persistence.
4. Software engineering review: Personal -> Coding -> QA -> Security ->
   synthesis over an immutable synthetic repository fixture; no commit/push.
5. Cloud assessment: Personal -> Cloud -> QA -> Security -> synthesis over
   static synthetic IaC; no apply.
6. Systems incident: Personal -> Systems -> QA -> Security -> synthesis over
   sanitized synthetic service/log fixtures; no system mutation.
7. Workflow automation: typed proposal and application validation; non-
   executable checkpoint values are rejected; a separate complete safe A-D
   proposal can be manually dispatched once into an existing sealed fixture
   selector. The two branches are not represented as one approval-authorized
   execution chain.
8. Bounded parallel workflow: two or three retained specialist runs within
   finite limits, canonical attribution/order, cancellation, partial failure,
   and synthesis. This is same-thread event multiplexing.
9. Policy denial: unauthorized/unknown specialist tool proposal rejected and
   audited with execution `NotAttempted`.
10. Approval flow: task-bound approval requested and simulated rejection
    resolved through application code, pending state cleared, no execution,
    attributed audit complete. Automatic runtime feedback remains absent.
11. Cancellation: root cancellation cleans up child tasks/runs and reconciles
    pending approval state before terminal task events.
12. Failure recovery: specialist/runtime failure returns a typed outcome and
    the application can continue or synthesize a bounded fallback without an
    application crash.

## Implementation milestones

- [x] M0 — Read repository, security, workflow, test, UI, approval, audit, and
      fixture state; verify clean published baseline and 198/198 focused tests.
- [x] M1 — Receive project-owner selection, create the dedicated branch, and
      begin gate `native-multi-agent-end-to-end-demonstrations`.
- [x] M2 — Add the canonical acceptance command and narrow rejected-approval
      and approved-document reusable-proposal assertions without production
      behavior changes.
- [x] M3 — Create demonstration, fixture, acceptance-matrix, and exact test
      evidence artifacts.
- [x] M4 — Run focused and complete validation, correct only defects found in
      scope, and update exact results.
- [x] M5 — Reconcile the owner-approved Demo 7 acceptance clarification, rerun
      architecture/security/code/quality/debt review and complete validation,
      then finalize and validate the deterministic gate marker.

## Security and privacy considerations

- Every workflow fixture is synthetic, sanitized, and repository-owned. No
  production data, credential, secret, token, private repository, cloud
  account, host, or service is accessed.
- The tests themselves perform deterministic local execution only. Cargo runs
  with `--locked`, not `--offline`; a cold toolchain cache may fetch locked
  crates. The command does not install packages, call a provider, or perform a
  product/external operation. Bounded build output and temporary-fixture I/O
  are disclosed separately.
- Test debug output must remain redacted. The acceptance documents list labels
  and source IDs, not secret-like values or personal data.
- The approval test uses a deterministic simulated native-dialog result; it
  does not claim real user consent or create an external action.
- Audit evidence is attributed to closed task/agent/run identities and remains
  process-local. It is sufficient contract evidence, not operational retention.
- No screenshot is required for backend acceptance. The repository contains no
  tracked Command Center validation screenshots, and the frontend projection
  is not authoritative backend evidence. Existing rendered M5 results may be
  linked only as separate UI-fixture evidence.

## Test plan

Focused implementation checks:

1. Run the exact macOS approval unit after assertion changes.
2. Run `npm run test:agent-acceptance` and record per-suite counts.
3. Run formatter and strict Clippy for the test-only Rust edit.

Completion checks:

1. Run complete `npm run verify` once after the final relevant source edit.
2. Run documentation, repository, security, and diff checks after final
   documentation synchronization.
3. Run the session-end, quality, and post-increment gate workflow.
4. Validate the completion marker and workspace fingerprint before reporting
   completion.

## Verification commands

```bash
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked \
  agent::orchestrator::tests::agent_approval_source_resolution_retains_origin_without_execution
npm run test:agent-acceptance
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
npm run lint:rust
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

The exact review and finalization commands are added to the progress log when
the applicable skills are run.

## Risks

- **Capability overclaim:** a fixture may be mistaken for a provider or live
  external operation. Mitigation: every demonstration contains a runtime,
  input, tool, external-result, and effect disclosure.
- **Approval conflation:** an `Approved` audit disposition may be mistaken for
  executed work. Mitigation: assert and display `NotAttempted` in both approval
  branches and state that no executor exists.
- **Automation bypass:** manual D-090 dispatch may be represented as the result
  of an approval checkpoint. Mitigation: demonstrate the rejected checkpoint
  proposal and take-once safe dispatch as separate application-controlled
  branches.
- **Parallelism overclaim:** retained simultaneous runs may be called provider
  concurrency. Mitigation: label it same-thread event multiplexing everywhere.
- **Audit overclaim:** bounded in-memory records may be called durable audit.
  Mitigation: document process-local scope and remaining durability gap.
- **Documentation drift:** the roadmap and architecture still contain pre-M5
  Command Center wording. Mitigation: reconcile those exact current-state
  passages in this evidence increment without changing historical records.

## Rollback or failure strategy

- Remove the npm acceptance alias and revert only the new test assertions and
  additive evidence documents.
- Existing native-agent implementation and its ten individual contract suites
  remain independently runnable and unchanged.
- If a test exposes a production defect, return the plan to Active, record the
  failing evidence, and correct only that defect if it is directly within this
  increment. Stop and seek owner direction if the correction requires a new
  runtime, executor, provider, IPC, persistence, or workflow architecture.
- If any required demonstration cannot be proved without an unsafe shortcut,
  report it as an explicit remaining gap; do not relabel a fixture or weaken a
  boundary to obtain a pass.

## Decisions made

- 2026-08-25: Reuse the existing draft plan rather than create a parallel plan
  system.
- 2026-08-25: Treat the existing ten public contract suites plus Rust library
  tests as the canonical acceptance corpus and expose them through one npm
  command; do not duplicate sealed workflow fixtures.
- 2026-08-25: Strengthen only missing rejected-approval assertions. Do not add
  production task-feedback semantics.
- 2026-08-25: Close the independently found Demo 3 evidence gap in the existing
  approved-document contract by having the Knowledge child create one pending
  proposal; do not add production memory behavior.
- 2026-08-25: Demonstrate D-090 checkpoint denial and safe manual fixture
  dispatch as separate branches because connecting them would bypass the
  implemented zero-executable-step boundary.
- 2026-08-25: Use deterministic test output and post-increment evidence for the
  backend demonstrations. Do not create a screenshot that could imply the
  frontend fixture projection is connected to the Rust workflows.

## Discoveries

- The exact ten public native-agent suites already cover all twelve requested
  scenario families and passed 198/198 before this increment.
- The shared integration-test `MockAgentRuntime` reports `RuntimeId::Native` by
  design, but it remains a test double and has no configured provider.
- The approval rejection path already returns a typed task-bound `Rejected`
  resolution and `NotAttempted`; it does not automatically add a denial message
  to runtime text or terminalize the task.
- Workflow proposals cap executable tool steps at zero. Complete A-D templates
  can be taken once for manual dispatch only into an already sealed fixture
  workflow; template E remains proposal-only.
- The Command Center's completed M5 rendered evidence is separate from backend
  architecture acceptance and no screenshots are tracked in the repository.
- Independent review found that the approved-document contract previously
  proved document flow and shared review separately but did not have the live
  Knowledge child create the claimed reusable proposal. The expanded test-only
  assertion now creates a deterministic application value through the real
  Knowledge-context proposal API, proves it remains `ProposedShared`, and proves
  it is not injected into Personal synthesis. It does not parse the proposal
  from runtime output or promote/durably persist it.
- Independent review confirmed the literal Demo 7
  checkpoint-to-approval-to-manual-execution chain does not exist because the
  executable tool-step limit is zero. Demo 7 is therefore recorded as partial;
  the separately validated manual A-D dispatch branch is not relabeled as an
  approval-authorized continuation.
- On 2026-08-25 the project owner explicitly revised this increment's Demo 7
  acceptance: checkpoint denial and manual safe A-D dispatch are validated as
  separate branches, the absent combined chain is an advisory, and no
  approval-to-dispatch bridge may be added. This supersedes the completion-
  blocker classification without changing D-090 or production architecture.

## Progress

- 2026-08-25: Verified feature branch, local `main`, and `origin/main` all began
  this increment at `527f0f4bafc4e263221e1ca246bc3e49600e0a26` with clean
  worktrees after the owner-authorized prior-increment merge.
- 2026-08-25: Completed mandatory repository and security reading and three
  independent read-only inventories of workflows/tests, governance, and
  documentation/UI evidence.
- 2026-08-25: Ran the ten public contract suites: 198 passed, 0 failed, 0
  ignored; command exited zero.
- 2026-08-25: Created branch
  `codex/native-multi-agent-end-to-end-demonstrations` and began the required
  gate.
- 2026-08-25: Added the canonical command and strengthened only test evidence:
  the macOS rejection audit now binds the full live root attribution, action,
  and policy, and the approved-document contract proves a Knowledge-authored
  pending proposal remains unapproved and absent from synthesis. Focused
  approved-document validation passed 1/1 without changing the suite count.
- 2026-08-25: Final-source focused approval and approved-document checks each
  passed 1/1. `npm run test:agent-acceptance` passed 447/447: 249 Rust library
  units plus 198 tests across the ten exact public contract binaries.
- 2026-08-25: Final-source `npm run verify` passed formatting, repository
  health, strict frontend/Rust lint, 28 hook tests, 38 repository-workflow
  tests, 211 frontend tests across 13 files, 249 Rust library tests, 232 Rust
  integration tests with one intentional Hermes probe ignored, TypeScript
  typecheck, Vite production build, and Tauri release no-bundle build.
- 2026-08-25: Independent governance/workflow review classified the literal
  Demo 7 checkpoint-to-approval-to-manual-execution chain as a completion
  blocker. D-090 deliberately prevents a checkpoint proposal from issuing a
  token, while the separately validated manual A-D dispatch branch has no
  approval-authorized connection. No production bridge was added.
- 2026-08-25: Architecture, security, code-health, technical-debt, and readiness
  reviews are complete. The consolidated report is
  [`2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md`](../reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md)
  and initially recorded `FAIL` under the original literal-chain requirement.
- 2026-08-25: The project owner approved separate Demo 7 branch acceptance,
  prohibited an approval-to-dispatch bridge, and directed a fresh closeout.
  Documentation/report reconciliation, focused tests, 447/447 acceptance,
  complete `npm run verify`, independent re-review, and final
  documentation/security/session checks pass. Gate finalization then produced a
  complete, fingerprint-valid `PASS WITH ADVISORIES` marker.

## Acceptance criteria

Demonstrations:

- [x] All twelve required scenario families have a reproducible evidence row
      and an exact runtime/input/tool/external-result/effect disclosure.
- [x] Direct, success, partial-failure, denial, approval-rejection,
      cancellation, and recovery outcomes execute deterministically.
- [x] Fixture and simulated results are never presented as live external
      operations.

Architecture and governance:

- [x] All nine definitions are registered and availability metadata is exact.
- [x] No specialist can directly spawn or delegate to another specialist.
- [x] `AgentOrchestrator` remains the sole task-creation owner.
- [x] Workflow Automation cannot execute tools or dispatch itself.
- [x] QA cannot approve and Security cannot authorize.
- [x] Unknown tools fail closed without execution.
- [x] Task, agent, root, request, and runtime-run identity forgery fails closed.
- [x] Memory and approved-document boundaries hold; shared knowledge is never
      silently persisted.
- [x] Destructive cloud and systems actions remain blocked and absent.
- [x] Audit attribution is complete for every governed demonstration, within
      its documented process-local limit.
- [x] Native remains the sole/default runtime identity, unwired to a provider.
- [x] Hermes remains deferred/blocked and absent from the implementation.

Validation and closeout:

- [x] `npm run test:agent-acceptance` passes with exact counts recorded.
- [x] Complete `npm run verify` passes after the final relevant source edit.
- [x] Required documentation, repository, security, formatting, lint, and diff
      checks pass.
- [x] Independent architecture, security, code, quality, and technical-debt
      reviews have no blocker under the owner-approved acceptance clarification.
- [x] The post-increment marker is complete, fingerprint-valid, and linked to a
      truthful passing report.
- [x] No commit or push is performed without separate project-owner authority.

## Final results

Implementation and automated validation are complete, and every listed
implemented-architecture criterion passes. Under the owner-approved Demo 7
clarification, checkpoint denial and safe manual A-D dispatch are two separate
accepted branches. The absent combined chain remains an explicit advisory and
grants no execution authority. The consolidated result is `PASS WITH
ADVISORIES`; the marker is complete and fingerprint-valid.

Consolidated evidence:
[`2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md`](../reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md).

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `ROADMAP.md` and the subordinate native multi-agent roadmap
- [x] `docs/design/NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md`
- [x] `ARCHITECTURE.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `TESTING_GUIDE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md` additive D-092 evidence and owner-approved D-093 acceptance
      clarification; no production architecture/security/product capability
      changes.
- [x] `TROUBLESHOOTING_LOG.md` not required at plan activation; update only if
      validation produces a new troubleshooting fact.
