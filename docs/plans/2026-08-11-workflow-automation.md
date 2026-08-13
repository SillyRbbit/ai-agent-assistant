# Workflow automation proposals and manual sealed dispatch

Status: Verified complete with advisories
Owner: Project owner
Decision: D-090
Gate: `agent-workflow-automation-proposals`
Last updated: 2026-08-13
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Activate Workflow Automation only as an application-owned, proposal-only
planner. Add one strict typed proposal contract, deterministic validation, and
an explicit single-use manual dispatch token for the four already implemented
sealed fixture-only/no-I/O workflow families:

1. Research brief: Research -> Knowledge -> Personal synthesis.
2. Code quality review: Coding -> QA -> Security -> Personal synthesis.
3. Infrastructure assessment: Cloud -> QA -> Security -> Personal synthesis.
4. Systems incident analysis: Systems -> QA -> Security -> Personal synthesis.

Catalog a fifth document-to-action-plan template, Knowledge -> Workflow
Automation proposal -> Personal synthesis, as proposal-only with no dispatch
token. The Workflow Automation Agent never creates a live task, invokes a
tool, approves a step, or starts another workflow. `AgentOrchestrator` remains
the sole task and runtime authority.

## User-visible outcome

None. This increment is an unwired deterministic Rust application-service
contract. It adds no Tauri command, React consumer, provider, model network,
filesystem access, scheduler, persistence, or device effect.

## Scope

- Narrow and supersede D-082's provisional Automation QA/Security sequence for
  this phase: the exact proposal route is Personal -> Workflow Automation ->
  Personal synthesis. QA and Security proposal review remains deferred and is
  never implied by the result.
- Add a framework-neutral `workflow_automation` domain with closed template,
  proposal, step, dependency, limit, validation, result, failure, event,
  attribution, and audit types.
- Add an immutable application-owned catalog for the five exact templates.
- Add one sealed Personal -> Workflow Automation -> Personal proposal and
  synthesis lifecycle in a private orchestrator child module.
- Parse runtime output with strict versioned JSON and deny unknown fields,
  identities, agents, tools, step kinds, dependencies, and authority claims.
- Validate graph shape, exact template equality, enabled agents, registered
  tool names, limits, no cycle, no nesting, and bounded inputs before a proposal
  is eligible for dispatch.
- Recognize `WorkflowStep::{AgentTask, Synthesis, GovernedTool,
ApprovalCheckpoint}` as closed typed data. The last two are non-executable in
  D-090 and always prevent token issuance after their schema/registry/binding
  checks. No approval request is created.
- Issue a private, non-clone, process-local, deadline-bound dispatch token only
  for a complete validated A-D proposal. The planner orchestrator releases it
  at most once; a fresh orchestrator consumes it by value through a manual
  application-service call and delegates to the corresponding existing sealed
  selector. No proposal automatically dispatches.
- Activate Workflow Automation as `Initial` only for the exact unwired
  proposal selector. Source contracts now pass; publication acceptance still
  requires the completion gate. Preserve its empty tool allowlist and
  `MemoryDisabledV1` profile.

## Explicit non-goals

- No general workflow engine, generic DAG executor, DSL, dynamic step registry,
  arbitrary code, shell text, inline script, executable callback, macro engine,
  or self-modifying definition.
- No scheduler, cron, recurring/startup/background execution, webhook, remote
  trigger, queue, worker, parallelism, nested workflow execution, automatic
  dispatch, automatic retry, compensation engine, or persistence.
- No tool execution, tool schema, repository tool, credential access,
  filesystem/network/platform operation, mutation, external communication, or
  device effect. The existing two tool definitions remain non-executing.
- No `ApprovalManager` request, presentation, decision, or dispatch. Workflow
  creation or proposal acceptance authorizes no current or future execution.
- No new `PolicyEngine` permission, agent allowlist, executor, durable
  `AuditLogger`, `PlatformAdapter`, memory access, document access, provider,
  external runtime, dependency, manifest, lockfile, configuration, IPC, UI, or
  Tauri capability/permission.
- No dispatch for document-to-action-plan. No generic composition of the
  existing workflows and no change to their stage semantics.
- No Codex, Hermes, OpenClaw, SaaS, multi-user, deployment, or distribution
  work.

## Pre-implementation baseline and prerequisite evidence

- Before this planning-only diff, Git was clean and synchronized at full commit
  `140f05bb55b9a6b39993c4f4764d198b465f90a6` on `main` and `origin/main`.
- At that baseline, `python3 .codex/hooks/post_increment_gate.py status`
  reported D-089 `status: complete`, `quality_gate: PASS WITH ADVISORIES`, and
  `valid: true`. This later D-090 planning diff expectedly makes the old live
  workspace fingerprint invalid without changing its recorded published-tree
  evidence.
- D-089 moved D-088 lifecycle/catalog/framing/validation internals into private
  modules without changing public behavior and cleared the prior
  `blocks_next_increment` finding. Its remaining module-size advisory requires
  this lifecycle to live in a separate private child and forbids a general
  engine.
- D-086, D-087, and D-088 already implement the four A-D target families as
  mutually exclusive, single-root, deterministic fixture-only/no-I/O
  selectors. They remain the manual-dispatch lifecycle targets; D-090 does not
  copy or reinterpret their state machines or imply a device/tool effect.
- `ToolRegistry` contains only `get_current_datetime` and `create_local_task`.
  Only Personal is eligible; approval never dispatches; every execution result
  is `NotAttempted`; no tool executor or general/durable audit logger exists.
- Workflow Automation is now `Initial` only for the D-090 sealed proposal
  selector, remains memory-disabled and tool-ineligible, and is absent from
  every generic delegation route.
- `AgentOrchestrator` owns one selected root workflow. A proposal run and its
  later manual execution therefore use separate orchestrator instances. The
  second instance starts only from an explicit trusted application call after
  the first is terminal; this is not nested execution or agent spawning.

## Exact files expected to change

Production Rust:

- `src-tauri/src/agent/workflow_automation.rs` (new public domain parent)
- `src-tauri/src/agent/workflow_automation/catalog.rs` (new private catalog)
- `src-tauri/src/agent/workflow_automation/validation.rs` (new private parser
  and validator)
- `src-tauri/src/agent/orchestrator/workflow_automation_proposal.rs` (new
  private proposal lifecycle)
- `src-tauri/src/agent/orchestrator/workflow_automation_dispatch.rs` (new
  private take-once dispatch/deadline boundary)
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/governance.rs` (reuse the shared built-in read-only tool
  catalog; no policy, approval, or audit transition)
- `src-tauri/src/agent/mod.rs`
- `src-tauri/src/tools/registry.rs` (crate-private built-in registry
  constructor reused by governance and proposal validation)

Tests:

- `src-tauri/tests/agent_workflow_automation_contract.rs` (new)
- `src-tauri/tests/agent_definition_registry_contract.rs`
- `src-tauri/tests/agent_governance_contract.rs`
- `src-tauri/tests/agent_orchestration_contract.rs`
- `src-tauri/tests/support/mock_agent_runtime.rs` only if the existing
  deterministic failure fixture cannot express a required deadline/start case

Planning and closeout documentation:

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md`
- `docs/increments/agent-workflow-automation-proposals.md` (new at closeout)
- `docs/reviews/2026-08-13-agent-workflow-automation-proposals-post-increment-review.md`
  (new at closeout)
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- this plan

No tool schema/implementation, policy, approval, executor, memory, document,
runtime, storage,
frontend, Tauri configuration/capability, dependency, manifest, lockfile, CI,
or external-integration file is in scope. An unexpected need to change one
stops the increment for owner review.

## Interfaces and invariants

### Proposal domain and immutable templates

- Closed identifiers are `WorkflowProposalId`, `WorkflowStepId`, and
  `WorkflowTemplateId`. Template IDs are exactly `ResearchBriefV1`,
  `CodeQualityReviewV1`, `InfrastructureAssessmentV1`,
  `SystemsIncidentAnalysisV1`, and `DocumentToActionPlanV1`.
- `WorkflowDefinition` contains a version, template ID, bounded objective,
  ordered closed steps, explicit dependencies, expected outputs, failure
  behavior, and `WorkflowLimit`. It contains no executable string, callback,
  dynamic type name, trusted identity, policy result, approval result, or
  dispatch authority.
- `WorkflowStep` is exactly `AgentTask`, `GovernedTool`,
  `ApprovalCheckpoint`, or `Synthesis`. Every wire enum uses strict known
  spellings and `deny_unknown_fields`. Agent identity is selected from
  `AgentId`; tool references contain exact name/version and bounded strict JSON
  arguments but never a shell/script field.
- `WorkflowTemplateCatalog::built_in()` owns canonical definitions. A model
  proposal must match the selected canonical step kind, agent, dependency,
  expected-output, failure-policy, and limits exactly. The runtime may provide
  bounded objective wording but cannot add/remove/reorder/remap a step.
- A-D contain only agent-task and synthesis steps. E contains only Knowledge,
  Workflow Automation proposal, and Personal synthesis and is marked
  `ProposalOnly`. No built-in contains a tool or approval step.
- Global validation bounds are four steps, three agent-task steps (synthesis
  excluded), three dependencies per step, two proposed tool steps, 8,192
  objective bytes/2,048 objective scalars, and 16,384 proposal bytes/8,192
  proposal scalars. `MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS` is exactly zero;
  every canonical template declares zero tool steps, zero retries, and zero
  nested workflows. The separate proposed-tool bound permits precise
  adversarial validation but grants no execution.
- The exact canonical ordered table is:

  | Template                  | Step 1    | Step 2                       | Step 3             | Step 4             | Dependencies                                                       | Disposition               |
  | ------------------------- | --------- | ---------------------------- | ------------------ | ------------------ | ------------------------------------------------------------------ | ------------------------- |
  | Research brief            | Research  | Knowledge                    | Personal synthesis | —                  | Knowledge <- Research; synthesis <- Knowledge                      | Ready for manual dispatch |
  | Code quality review       | Coding    | QA                           | Security           | Personal synthesis | QA <- Coding; Security <- QA; synthesis <- Security                | Ready for manual dispatch |
  | Infrastructure assessment | Cloud     | QA                           | Security           | Personal synthesis | QA <- Cloud; Security <- QA; synthesis <- Security                 | Ready for manual dispatch |
  | Systems incident analysis | Systems   | QA                           | Security           | Personal synthesis | QA <- Systems; Security <- QA; synthesis <- Security               | Ready for manual dispatch |
  | Document to action plan   | Knowledge | Workflow Automation proposal | Personal synthesis | —                  | Workflow Automation <- Knowledge; synthesis <- Workflow Automation | Proposal only             |

  Every edge is exact and no other topological ordering, transitive edge,
  optional step, or target is accepted.

### Framing and final synthesis bounds

- The serialized canonical catalog is at most 8,192 bytes. Planner selected
  text (fixed framing + 8,192-byte objective + catalog) is at most 24,576 bytes.
  The validated proposal transfer is at most 16,384 bytes; final Personal
  synthesis selected text (fixed framing + objective + transfer + disclosure)
  is at most 32,768 bytes. Final raw output is at most 8,192 scalars and 16,384
  bytes. Fixed framing is checked, not assumed.
- Maximum-bound quote/backslash/newline/multibyte tests construct each exact
  `RuntimeTurnRequest`; separate `NativeAgentRuntime::start` regressions prove
  the JSON-encoded initial gateway request for planner and synthesis remains at
  or below the unchanged 65,536-byte ceiling. Failure is typed before runtime
  mutation; there is no alternate runtime/fallback authority.
- `WorkflowAutomationSynthesisV1` strictly denies unknown fields and contains
  only version, bounded summary, template ID, proposal ID, application-derived
  expected validation disposition, expected complete/partial status,
  unresolved issues, and invariants `fixture_based = true`, `unwired = true`,
  `tools_executed = false`, `approvals_requested = false`, and
  `effects_performed = false`. Output cannot supply token eligibility,
  dispatchability, policy/approval/execution authority, trusted identity,
  runtime/task attribution, or change any disposition/status. Any contrary
  claim or mismatch invalidates synthesis and issues no token. The final result
  stores the disposition/status derived by application code, not trusted wire
  booleans.
- Token eligibility is derived only by application code from canonical A-D
  template identity, exact validated proposal, complete lifecycle and final
  disclosure, unexpired deadline, and unused take state. Runtime/model text
  cannot request, construct, preserve, or extend a token.

### Validation order and fail-closed behavior

Application validation occurs before token creation and is atomic:

1. strict envelope/version/size/text/identifier parsing;
2. unique step IDs, known dependency IDs, no self-edge, deterministic cycle
   detection, and stable topological order;
3. fixed count, byte, duration, retry, nesting, proposed-tool, and dependency
   bounds without applying the executable-tool zero prematurely;
4. exact registry lookup and `Initial` activation for every agent;
5. exact `ToolRegistry` name/version/argument validation for any tool step
   through a shared crate-private read-only built-in registry;
6. mandatory approval-checkpoint binding to one known prior tool step;
7. exact equality with the application-owned selected template; and
8. dispatch-mode derivation by application code, never by runtime output.

`InMemoryToolRegistry::built_in()` becomes the single application-owned source
for the existing two definitions and is reused by governance and proposal
validation. Read-only lookup plus schema argument validation creates no
attribution, policy decision, governance reservation, audit record, pending
approval, presentation, or execution disposition and returns no validated
executable subject.

Unknown agent, unavailable/deferred agent, unknown tool, version mismatch,
invalid arguments, missing/duplicate dependency, cycle, excessive bound,
unsupported step, arbitrary shell/script field, nested workflow, definition
mutation, approval mismatch, or template deviation returns a closed redacted
error before a token exists. A recognized tool step without its required
checkpoint returns `MissingApprovalCheckpoint`; with a valid checkpoint it
still returns `ToolStepsUnavailable`. Any standalone
checkpoint returns `ApprovalDispatchUnavailable`. Validation creates no
approval request and performs no tool call.

### Proposal lifecycle

- The exact selector is Personal root -> one Workflow Automation depth-one
  child -> Personal synthesis on the same root. Only the orchestrator creates
  the child or invokes the runtime. The Workflow Automation definition never
  calls another agent.
- Public contracts are `WorkflowAutomationProposalRequest`,
  `WorkflowAutomationWorkflowAcceptance`, `WorkflowAutomationWorkflowEvent`,
  `WorkflowAutomationAuditRecord`, `WorkflowAutomationWorkflowResult`, and
  `WorkflowAutomationContinuationFailure`. A validated proposal has
  `WorkflowValidationDisposition::ReadyForManualDispatch` only for A-D or
  `ProposalOnly` for E.
- The selector is mutually exclusive with generic delegation, approved-
  document, D-086, D-087, D-088, and repeated proposal selection.
- Limits are exactly two tasks, one non-replenishing child, three runtime-run
  attempts (initial Personal, Workflow Automation, Personal synthesis), one
  active child, depth one, zero retries, at most eight accepted runtime events
  per event-consuming run, 16 total runtime events, 16 generic orchestration
  events, eight workflow events, and eight matching content-free attribution/
  audit records.
- Exact successful proposal event/audit order is
  `ProposalStarted`, `ProposalCompleted`, `SynthesisStarted`, `Completed`, with
  one matching content-free audit record per event. Partial branches insert
  exactly one `PartialFailure` event/record before `SynthesisStarted`; expiry
  emits exactly one `Expired`; cancellation exactly one `Cancelled`; terminal
  failure exactly one `Failed`. No branch exceeds eight workflow events or
  eight matching records. Every append preflights remaining capacity before
  task/run/event mutation.
- Planner failure, invalid output, continuation-start failure, and incomplete
  proposal produce a typed truthful Personal fallback when the remaining
  run/event capacity permits. Deadline expiry instead performs child-first
  cancellation, records terminal `Expired`, and starts no synthesis successor.
  No dispatch token is issued for a partial, failed, cancelled, expired,
  tool-bearing, approval-bearing, E, or otherwise non-dispatchable result.
- Root cancellation remains child-first, idempotent, terminal, and starts no
  successor. Late events fail closed. Task IDs, root/parent identity, runtime
  identity, policy and memory profiles, predecessor attribution, status, and
  audit identity are application-derived and redacted.

### Cooperative monotonic deadline

- `WorkflowLimit` declares an application-fixed maximum duration of 120
  seconds; retries and nested workflow depth are fixed at zero. The application
  derives one process-local `std::time::Instant` deadline with checked
  arithmetic when the proposal selector starts. Runtime/model output cannot
  supply or extend the trusted instant.
- The orchestrator checks the deadline before child creation, runtime start,
  runtime-event acceptance, terminal preparation, synthesis start, token take,
  and manual dispatch. Expiry invokes the ordinary child-first cancellation
  path and records a typed content-free `Expired` outcome without starting a
  successor.
- This is cooperative, not preemptive: it cannot interrupt a synchronous
  `runtime.start` call already executing. Completion documentation must state
  that limitation. Unit tests use a private injected monotonic clock or exact
  already-expired fixture; production APIs expose no clock control.
- The dispatch token carries the original deadline and cannot extend it. A
  token expired before manual consumption fails without selecting or mutating
  the fresh execution orchestrator.
- Successful A-D dispatch transfers that same deadline into a private manual
  workflow lease in the destination orchestrator. Before each destination
  `accept_runtime_event`, terminal preparation/successor start, and explicit
  cancellation or `manual_workflow_result(&mut self)` call, the orchestrator
  checks the lease. On the first
  trusted mutating call after expiry it attempts ordinary child-first
  cancellation, starts no successor, and returns a typed `Expired` or
  `ExpiryCancellationFailed` state. Consumers of a manually dispatched
  workflow use the central mutable result entrypoint; legacy immutable
  per-family snapshot getters do not advance time or claim asynchronous expiry.
  There is no timer or background worker.

### Single-use manual dispatch

- A complete validated A-D result permits `take_workflow_manual_dispatch()`
  exactly once after the proposal root is terminal. The process-local token is
  a `WorkflowManualDispatch`: non-serializable, non-`Clone`, redacted on
  `Debug`, and bound to proposal ID, template ID, planner root/task attribution,
  catalog version, and deadline.
- A fresh `AgentOrchestrator` with a live Personal root accepts the token by
  value through `start_manual_workflow(...)` and returns the closed
  `WorkflowManualDispatchAcceptance`. The destination derives only the
  application-owned deterministic fixture request for the token's template;
  no caller/model objective, fixture catalog, source, workflow input, agent, or
  selector is accepted. Template mismatch, E, replay, expiry, different
  catalog version, prior selection, or non-live context fails before mutation.
- `start_manual_workflow` consumes the token on every success or error. It never
  returns the token for correction or retry. Every validation, deadline,
  selection, runtime-start, or downstream selector failure therefore drops it;
  a new proposal lifecycle is required.
- Dispatch maps directly to the existing D-086/D-087/D-088 public selector.
  Those selectors retain their exact tasks, attempts, events, cancellation,
  provenance, partial-failure, fixture, and result contracts. D-090 adds no
  cross-family state machine and no generic step runner.
- The trusted application must explicitly create the fresh orchestrator, start
  its Personal root, and call dispatch. Proposal completion never calls it.
  Workflow-local content-free attribution binds planner proposal/template to
  the execution root and records manual dispatch accepted/expired/cancelled
  state; it is not a durable or general `AuditLogger`. The private dispatch
  journal has an exact cap of four records: `Requested`, `Accepted`, then one
  terminal `Completed` (whose status is complete or partial), `Cancelled`,
  `Expired`, or `Failed`; an `ExpiryCancellationFailed` may appear before the
  eventual `Expired`. It preflights capacity before destination
  selection/cancellation mutation and redacts proposal content, objectives,
  fixture text, runtime identity, arguments, and raw errors.

### Activation and authority

- Workflow Automation moves to `Initial` only for this sealed proposal
  selector. Its purpose/instructions advertise typed application validation,
  five templates, A-D manual eligibility, E proposal-only, and no execution.
- `WorkflowProposalOnlyV1`, `MemoryDisabledV1`, empty tool eligibility, the
  delegation matrix, runtime tool rejection, and `NotAttempted` execution
  remain unchanged. Generic Personal -> Workflow Automation delegation remains
  denied.
- A proposal, validation success, token, planner/synthesis text, approval
  checkpoint, policy result, or audit record is not device authority.
  `AgentOrchestrator` remains the only task/runtime coordinator; no specialist
  spawns.

## Implementation milestones

- [x] Record D-090, fresh architecture/security/readiness evidence, and begin
      gate `agent-workflow-automation-proposals` only while the plan is Ready.
- [x] Implement the strict domain, immutable five-template catalog, parser,
      validator, limits, deadline, redacted errors, and focused unit tests.
- [x] Implement the private proposal lifecycle, partial/cancel/expiry behavior,
      one-time token, and fresh-orchestrator A-D dispatch mapping.
- [x] Update activation instructions and regression contracts without changing
      governance, memory, tool, approval, or existing selector authority.
- [x] Run focused and complete verification, quality/security review, current-
      state sync, and deterministic post-increment finalization.

## Test plan

Focused domain and public contracts cover:

- all five canonical proposals and exact deterministic ordering;
- exact Personal -> Workflow Automation -> Personal routing with no QA or
  Security task or review claim;
- A-D token issuance and one-time manual mapping to the existing selectors;
- E proposal-only result with no token;
- unknown and deferred/unavailable agent;
- unknown tool, tool version/argument mismatch, zero tool-step execution limit,
  missing approval checkpoint, valid-but-non-executable checkpoint, and proof
  that no approval request or execution attempt occurs;
- duplicate/missing/self dependency, cycle, excessive steps/tasks/dependencies/
  inputs/duration/retries, unsupported or unknown step, arbitrary shell/script,
  nested/self-modifying workflow, extra fields, malformed/duplicate-key JSON,
  and output/input byte limits;
- incomplete proposal, planner/runtime/start/synthesis failure, truthful partial
  fallback, cancellation, deadline expiry at every proposal and destination
  event/successor/cancel/manual-result checkpoint, injected monotonic clock
  behavior without sleeps, late-event rejection, and token expiry before
  dispatch;
- mismatched input/template, token replay/take replay, prior selector conflict,
  and atomic no-mutation failures;
- exact event ordering, task/run/depth/child/retry limits, content-free audit
  attribution across planner and dispatch roots, and Debug/error redaction;
- Workflow Automation cannot delegate, create a task directly, approve, alter
  policy, execute a tool, or invoke dispatch;
- activation/catalog/governance regressions and unchanged Native runtime,
  D-086, D-087, D-088, generic orchestration, memory/document, and task
  contracts.

## Verification commands

Focused while implementing:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked workflow_automation::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_workflow_automation_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
```

Completion after the final source edit:

```bash
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Then run the repository quality-gate and post-increment-gate skills. No native
application manual check is required because the Rust boundary remains unwired
and no I/O or device behavior is added. Manual diff review must confirm no
tool, policy, approval, executor, dependency, configuration, IPC/UI, provider,
filesystem/network, persistence, scheduler, or external-effect path changed.

## Risks and security impact

- A proposal/token may be mistaken for authorization. Mitigation: closed types,
  non-clone one-time ownership, explicit manual call, separate orchestrators,
  no serialization, no tool/approval execution, and `NotAttempted` invariants.
- A generic engine may emerge accidentally. Mitigation: five immutable exact
  templates, strict equality, direct dispatch to existing selectors, private
  lifecycle ownership, and stop on any reusable executor/DAG abstraction.
- Deadline claims may overstate preemption. Mitigation: document cooperative
  checkpoints and the synchronous-runtime limitation; never claim hard kill.
- Untrusted strings may smuggle shell or self-modification intent. Mitigation:
  strict enums/fields, no executable field, unknown-field denial, registry
  checks, template equality, bounded text, and redacted errors.
- Cross-root attribution may be replayed or confused. Mitigation: process-local
  non-clone token, exact planner/template/catalog/deadline binding, take-once
  storage, consume-by-value dispatch, live-context and selection preflight.

## Rollback and stop conditions

Before publication, remove the two new private/domain modules and new contract,
restore definition activation/instructions and orchestrator/mod exports, and
revert only D-090 current-state documentation. Existing A-D workflow source and
evidence remain unchanged. No external rollback exists because D-090 creates no
persistent or device state.

Stop for owner review if implementation requires a tool/policy/approval/
executor/audit-authority change, a new dependency or configuration, persistent
token/workflow state, a generic runner/DSL, thread/background work, hard
preemption, new Tauri/React/IPC, template E dispatch, any I/O/effect, widened
specialist route, parallelism, or changes to D-086/D-087/D-088 semantics.

## Decisions and discoveries

- D-090 selects the narrowed proposal/manual-dispatch boundary and explicitly
  defers governed tool execution and template E execution.
- D-089 is published at `140f05b`; its valid marker clears the prior private-
  decomposition prerequisite but grants no execution authority.
- The missing executor, general audit logger, and approval-to-dispatch path make
  executable tool steps unsafe in this increment. Typed recognition plus
  fail-closed validation is the truthful boundary.
- One orchestrator owns one root selection, so a later explicit application
  dispatch must consume a process-local token in a fresh orchestrator rather
  than recursively starting a workflow from the planner run.

## Progress

- [x] 2026-08-13: owner selected the narrowed D-090 scope.
- [x] 2026-08-13: clean synchronized `140f05b` and D-089 complete/valid marker
      verified.
- [x] 2026-08-13: exact scope, contracts, files, limits, deadline semantics,
      rollback, tests, and non-goals recorded; readiness is Ready.
- [x] Began gate `agent-workflow-automation-proposals` and completed the bounded
      source/test implementation.
- [x] Source verification passed: focused Workflow Automation tests 12/12,
      public contract 18/18, strict Clippy, 393 all-target tests with one
      intentional ignored probe, and complete `npm run verify` with 124
      frontend and 208 Rust library tests plus the Tauri no-bundle release
      build.
- [x] Independent architecture, security, and code review returned `PASS WITH
ADVISORIES` and no blockers.
- [x] Post-documentation checks passed and deterministic finalization completed.

## Acceptance criteria

- [x] Strict application-owned proposals validate deterministically; all
      adversarial/unknown/unbounded/nested/self-modifying values fail closed.
- [x] A-D alone produce one expiring take-once token and map manually to only
      the corresponding existing sealed selector in a fresh orchestrator.
- [x] E remains proposal-only and every tool/approval step is non-executable.
- [x] Workflow Automation is active only for the sealed proposal selector and
      remains memory-disabled, tool-ineligible, non-spawning, and non-authorizing.
- [x] Cooperative cancellation/deadline, partial failure, event order,
      attribution, redaction, and all finite limits pass deterministic tests.
- [x] Existing workflow/runtime/governance contracts and complete source
      verification pass.
- [x] Final post-documentation evidence and completion marker are complete.

## Final results

Implemented in the unwired Rust core with exact five-template proposal
validation, the Personal -> Workflow Automation -> Personal lifecycle, A-D
take-once manual dispatch, proposal-only E, cooperative monotonic deadlines,
strict non-executable tool/approval handling, and content-free bounded
attribution. Source verification passes: 12/12 focused Workflow Automation
tests, 18/18 public contracts, strict Clippy, 393 all-target tests with one
intentional ignored probe, and complete `npm run verify` including 124
frontend tests, 208 Rust
library tests, and the Tauri no-bundle release build.

Independent architecture, security, and code review is `PASS WITH ADVISORIES`
with no blocker. Final documentation, repository, security, diff, and
session-end checks pass, and the deterministic completion marker is complete.
No next increment is Ready.
