# Per-agent governance foundation

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-12
Decision: D-084, preserving D-083, D-082, and D-079
Gate ID: `agent-governance`
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Bind an application-derived policy profile and exact live agent/task/runtime
attribution through one closed, non-executing tool-governance path, strengthen
the existing Personal Assistant -> Research delegation matrix, and record
bounded redacted policy/approval evidence without enabling any tool, memory,
provider, platform, or specialist workflow.

## User-visible outcome

None. The Rust core gains deterministic governance contracts only. No Tauri
command, React surface, provider, model, runtime tool lane, executor, or device
effect is connected. `NativeAgentRuntime` remains sole/default and the visible
application remains unchanged.

## Scope

- Nine exact versioned `AgentPolicyProfileId` values and one immutable
  application-owned profile registry.
- Exact `AgentId` -> policy-profile association in every built-in definition,
  task, execution context, delegation request, governed tool request, approval
  presentation/resolution, and governance audit record.
- One closed `AgentAttribution` derived only from the exact live
  `AgentExecutionContext`; callers cannot construct or override its trusted
  fields.
- The existing application-owned `ToolRegistry`, deterministic `PolicyEngine`,
  and `ApprovalManager` reused for a synthetic, non-executing agent governance
  contract.
- A bounded volatile application-owned governance audit family with injected
  clock evidence and execution fixed to `NotAttempted`.
- The exact initial delegation matrix: Personal Assistant may request Research;
  every specialist, including Research, may not delegate.
- Focused adversarial tests, regression tests, architecture/security docs,
  current-memory sync, increment record, and post-increment review.

## Explicit non-goals

- No tool execution, executor, platform adapter, dispatch, device effect,
  memory access, persistence, credential, provider, process, network, shell,
  filesystem, Tauri IPC, React, or UI.
- No runtime capability change and no acceptance of runtime `ToolProposal`.
  `AgentOrchestrator::accept_runtime_event` continues to reject it.
- No change to `AgentRuntime` or `NativeAgentRuntime`; no selector, fallback,
  Hermes, or external framework.
- No activation of the seven deferred agents and no new specialist workflow.
- No `agent.delegate` tool. Delegation remains an explicit orchestrator service
  and never traverses `ToolRegistry`.
- No generic `AuditLogger` resurrection. Only the closed bounded governance
  record family in this plan may be added.
- No memory-namespace placeholder. A real namespace remains mandatory before
  any future memory- or data-bearing privileged action.
- No new generic policy engine and no role-derived booleans such as
  `can_execute`, `can_approve`, or `can_delegate`.
- No typed QA, security, or workflow report DTO without a current consumer.

## Pre-implementation baseline and constraints

- Published baseline is clean synchronized `main` at `1d1d9d6`.
- Gate `agent-task-orchestration` is complete and fingerprint-valid with PASS
  WITH ADVISORIES; task/orchestration contracts pass 22/22.
- `AgentDefinition` has no policy profile; `AgentTask` and
  `AgentExecutionContext` have no policy-profile identity.
- `AgentOrchestrator` owns exact live task/run binding and rejects every runtime
  tool proposal. It is not connected to Tauri, React, or a provider.
- The current native concrete gateway lane privately composes tool registry,
  deterministic policy, approval, and approval-resolution audit. It has no
  production caller and does not carry agent/task attribution.
- `PolicyInput` currently owns only a locally schema-validated function call.
  `ApprovalManager` binds run/request/call identity and has one pending request.
- No executor exists. An approval or `Allow` decision cannot dispatch anything.
- D-030 deleted the earlier generic arbitrary-string audit scaffold. This
  increment may not revive it.
- D-082 requires policy-profile and memory-namespace identity before privileged
  behavior. D-084 adds an enforced profile now but permits no privileged or
  memory-bearing behavior, so memory namespace remains separately gated.

## Current-state evidence

- `git status --short --branch`: clean synchronized `main` before plan edits.
- `git rev-list --left-right --count HEAD...@{upstream}`: `0 0`.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked`:
  6 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked`:
  22 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked`:
  2 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked`:
  2 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked`:
  1 passed.
- The owner explicitly approved D-084 and this bounded non-executing direction
  on 2026-08-12.

## Files expected to change

Production Rust and focused contracts:

- `src-tauri/src/agent/governance.rs` (new)
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/function_call_validation.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/task.rs`
- `src-tauri/src/policy/engine.rs`
- `src-tauri/src/policy/types.rs`
- `src-tauri/src/approvals/decision_source.rs` (target-mac presentation binding only)
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/src/approvals/types.rs`
- `src-tauri/src/audit/governance.rs` (new)
- `src-tauri/src/audit/mod.rs`
- `src-tauri/tests/agent_definition_registry_contract.rs`
- `src-tauri/tests/agent_orchestration_contract.rs`
- `src-tauri/tests/agent_governance_contract.rs` (new)
- Existing policy, approval, audit, runtime, and gateway tests only where an
  additive origin enum or accessor requires compatibility updates.

Decision, architecture, security, roadmap, plan, and closeout:

- `DECISIONS.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/PROJECT_DIRECTION.md`
- `ARCHITECTURE.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `docs/security/NATIVE_AGENT_GOVERNANCE_THREAT_MODEL.md` (new)
- `docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md` (new)
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `docs/plans/2026-08-11-agent-governance.md`
- `PLANS.md`
- `NEXT_STEPS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `CHANGELOG.md`
- `docs/increments/agent-governance.md` (new)
- `docs/reviews/2026-08-12-agent-governance-post-increment-review.md` (new)

`TROUBLESHOOTING_LOG.md` changes only if a new durable troubleshooting outcome
occurs. Any other production, dependency, manifest, lockfile, capability,
configuration, workflow, hook, skill, Tauri, React, provider, or Hermes path is
a stop condition pending owner review.

## Interfaces and invariants

### Profiles and definition binding

`AgentPolicyProfileId` is exactly:

| Agent                | Profile                         |
| -------------------- | ------------------------------- |
| Personal Assistant   | `PersonalAssistantV1`           |
| Research             | `ResearchReadOnlyV1`            |
| Coding               | `CodingGovernedV1`              |
| Cloud Infrastructure | `CloudInfrastructureGovernedV1` |
| Systems Operations   | `SystemsOperationsGovernedV1`   |
| Knowledge & Document | `KnowledgeDocumentsV1`          |
| QA & Validation      | `QualityValidationAdvisoryV1`   |
| Security & Risk      | `SecurityRiskAdvisoryV1`        |
| Workflow Automation  | `WorkflowProposalOnlyV1`        |

- `AgentDefinition` is the sole `AgentId` -> `AgentPolicyProfileId` mapping.
  The profile registry is keyed by profile ID, contains all nine exact profile
  definitions once, and has no agent-remapping API. Missing, duplicate,
  unknown, or mismatched association fails closed; there is no fallback.
- `AgentTask::new_root` and `AgentTask::new_child` consume one sealed resolved
  definition identity containing agent and profile together; they never accept
  those two fields independently. `AgentExecutionContext` carries the captured
  profile and exact live-context matching includes it.
- Personal Assistant's profile alone recognizes the two already registered
  schemas as governance-eligible: `get_current_datetime@1` and
  `create_local_task@1`. Eligibility is not permission or execution authority.
- The other eight profiles have an empty current tool allowlist. Research is
  read-only by posture but has no eligible research tool in this increment.
- Workflow Automation may only participate in a future typed workflow proposal
  boundary; it cannot create tasks, delegate, approve, or execute here.

### Composition and lifecycle owner

- `AgentGovernanceService` is the concrete non-executing coordinator. It owns
  one `AgentPolicyProfileRegistry`, one existing `InMemoryToolRegistry`, one
  existing `DeterministicPolicyEngine`, one existing
  `InMemoryApprovalManager`, and one new
  `InMemoryAgentGovernanceAudit`. Component authority remains separate: the
  service sequences them but cannot override their decisions or execute.
- `AgentOrchestrator<R>` composes exactly one `AgentGovernanceService` so the
  live task/run state and pending approval share a bounded lifecycle. The
  orchestrator remains the task authority; the service remains the governance
  coordinator; neither becomes the other component authorities.
- Public orchestrator entry points are exactly:
  `govern_tool_proposal(context, proposal)`,
  `pending_governance_approval(task_id)`,
  `issue_governance_presentation(context)`, target-mac
  `resolve_governance_source_outcome(context, outcome)`,
  `expire_governance_approval(context)`, and existing `cancel_task(task_id)`.
  The last method calls the service's internal
  `cancel_pending_for_task(attribution)` before runtime/task terminalization.
- The service entry points are exactly `evaluate_tool`, `pending`,
  `issue_presentation`, `resolve_source_outcome`, `expire_due`,
  `cancel_pending_for_task`, `begin_delegation`, and the infallible
  `finish_delegation`. Only the orchestrator calls mutation entry points, after
  reconstructing and revalidating exact live attribution.
- `govern_tool_proposal` returns exactly
  `AgentToolGovernanceOutcome::{Final { policy_outcome, policy_reason,
approval: NotRequired, execution: NotAttempted }, ApprovalPending {
approval_id, policy_reason }}`. Deny and Allow are both final non-executing
  outcomes; `ApprovalPending` is neither approval nor execution.
- Source resolution, expiry, and protective cancellation return
  `AgentApprovalGovernanceOutcome { approval_id, disposition,
execution: NotAttempted }`. The public presentation/request views expose a
  closed `ApprovalOrigin::{LegacyGateway, Agent(AgentAttribution)}`; existing
  legacy accessors remain compatible.
- All failures use closed `AgentGovernanceError` variants for context, profile,
  validation, replay/capacity, policy, approval, audit, and lifecycle stages.
  They never contain proposal names, arguments, task content, or arbitrary
  downstream strings. `AgentOrchestratorError::Governance` retains that typed
  error without flattening it.
- While an approval is pending, the orchestrator rejects runtime events and
  delegation for that task with typed `GovernanceApprovalPending`; this
  prevents task completion from racing the approval. Task cancellation first
  consumes and audits the pending approval, then cancels runtime/task state.
  An approval-manager cancellation failure leaves both task and approval
  coherently pending. If approval cancellation succeeds but later runtime
  cancellation fails, the task/run remains live with the approval terminally
  cancelled and may be cancelled again; no approval or execution authority
  remains.
- Root cancellation preserves the existing child-first order. If the active
  child has a pending approval, the orchestrator reconstructs the child's live
  attribution, calls `cancel_pending_for_task(child)` and commits its
  `Cancelled` audit state before `cancel_child`; only after the child is
  terminal does it reconcile a root approval and cancel the root. A failure at
  either approval-manager step stops before the corresponding task/runtime
  mutation.
- There is at most one pending approval and one root workflow. No background
  worker exists. Expiry is evaluated only on a governance entry point, as in
  the existing manager; the one bounded pending value cannot grow without
  limit.

### Attribution and request binding

- `AgentAttribution` binds exact agent, task, root task, optional parent task,
  runtime, policy profile, depth, and current runtime run/request identity.
- Only `AgentOrchestrator` may derive attribution, after exact comparison with
  its live task and active run. Stale, foreign, terminal, missing, or mismatched
  context fails before tool lookup, policy, approval, or audit mutation.
- A new `AgentToolProposal` supplies only bounded call ID, name, contract
  version, and argument JSON. It supplies no trusted agent/task/runtime/profile
  fields. Its `Debug` redacts call ID, name, and arguments; errors never echo
  them. `UntrustedRuntimeToolProposal` remains unchanged and remains rejected.
- Agent schema validation produces a distinct non-clonable
  `AgentSchemaValidatedToolRequest` containing the exact `AgentAttribution` and
  locally derived known schema facts. Its constructor is private to the
  service. The run/request identity comes only from attribution; the proposal
  cannot duplicate or override it.
- Existing `SchemaValidatedFunctionCall`, `PolicyInput::from_validated_call`,
  and `PolicyEngine::evaluate` remain legacy-gateway-only and unchanged. There
  is no conversion from an agent request into legacy `PolicyInput`.
- The same `DeterministicPolicyEngine` adds a distinct
  `evaluate_agent(request, profile_registry)` method. It consumes the sealed
  agent request, verifies the exact profile and eligible schema, then returns a
  sealed `AgentPolicyDecision`. Existing policy classification is reused.
- `ApprovalManager::create_agent_request` accepts only the sealed agent policy
  decision. Its internal pending subject uses a closed
  `LegacyGateway|Agent` decision enum; no optional identity or `None` fallback
  exists. Agent origin is immutable through presentation, resolution, and
  governance audit.

### Deterministic policy and approval

- One existing `DeterministicPolicyEngine` remains authoritative. The agent
  profile is checked before its existing risk/permission classifier is reused.
- Profile denial is a closed deterministic deny reason. Unknown tool, version
  mismatch, malformed arguments, missing profile, and attribution mismatch are
  typed failures; none defaults or reaches approval.
- `get_current_datetime@1` under `PersonalAssistantV1` produces `Allow`.
  `create_local_task@1` produces `RequireApproval`. Every other current
  agent/tool association produces `Deny`.
- `Allow` is non-authorizing and returns execution `NotAttempted`.
- A `RequireApproval` decision uses the existing one-pending
  `ApprovalManager`. Its presentation retains the full closed agent
  attribution plus action, target, affected parameters, risk, policy reason,
  remaining TTL, scope/reversibility facts, and tool contract. Debug remains
  redacted.
- Approval source outcomes remain manager-, presentation-, and exact-subject
  bound. Agent/task/profile mismatch, replay, duplicate presentation,
  expiration, and forged response fail closed.
- Approved, rejected, cancelled, or expired resolution never executes a tool
  in this increment. Approval is evidence, not dispatch authority.
- Pending agent approval is cancelled before its task becomes terminal. A
  resolution for a no-longer-live task cannot authorize anything.

### Delegation matrix

- Delegation remains an explicit `AgentOrchestrator::request_delegation`
  application-service operation, outside `ToolRegistry` and runtime events.
- `DelegationMatrix` contains exactly Personal Assistant -> Research and lives
  in `AgentGovernanceService`; it is not a tool definition or profile grant.
- Research and all other specialists cannot delegate. Self, reverse, deferred,
  unknown, stale, mismatched, and over-depth requests fail before child
  creation or runtime cancellation.
- `DelegationRequest` retains the exact source attribution snapshot and target;
  the orchestrator revalidates the live context immediately before mutation.
- After exact live source attribution and a closed `AgentId` target are
  established, the orchestrator calls `begin_delegation` before source-role,
  registry, activation, matrix, depth, budget, runtime cancellation, or child
  allocation checks. The audit subject records that exact typed target, not
  only Research. Every self, specialist-source, deferred, route, depth, active-
  child, or total-budget denial after attribution commits a closed `Denied`
  control result. Stale, foreign, missing, or otherwise pre-attribution errors
  remain unaudited rather than falsely attributed.
- An allowed matrix result returns a one-shot `DelegationAuditReservation`;
  runtime cancellation/child-start failure commits `Failed`, and successful
  child allocation commits `ChildCreated`. The finishing update cannot fail.
- Registry membership, profile, role name, QA output, Security output, or
  Workflow output never grants route, approval, or execution authority.

### Governance audit

- The new audit family is closed and bounded; it is not a generic string logger
  and stores no prompt, objective, context, parameters, tool output, child
  result, approval message, credential, secret, or reasoning.
- `MAX_AGENT_GOVERNANCE_RECORDS` is exactly 32. One subject consumes one slot
  for its complete lifecycle; state updates do not consume more capacity.
- `AgentGovernanceSubjectKey` is closed and replay-safe:
  `Tool { attribution, call_id }` or
  `Delegation { attribution, target_agent_id }`. Call ID plus exact full
  attribution is consumed for Allow, Deny, validation rejection, and approval
  alike; changing a name, version, or arguments cannot reuse the call ID.
- `AgentGovernanceRecord` is exactly `Tool(AgentToolGovernanceRecord)` or
  `Delegation(AgentDelegationGovernanceRecord)`. Tool lifecycle state is
  `ValidationRejected`, `PolicyEvaluated`, `ApprovalPending`, or
  `ApprovalResolved`. Delegation lifecycle state is `Denied`, `Allowed`,
  `ChildCreated`, or `Failed`.
- Applicable facts are closed rather than fake optional results:
  `AgentPolicyAuditOutcome::{NotEvaluated, Evaluated { outcome, reason }}`,
  `AgentApprovalAuditDisposition::{NotRequired, Pending, Approved, Rejected,
Cancelled, Expired}`, `AgentExecutionDisposition::NotAttempted`, and
  `AgentControlResult::{NotCreated, ChildCreated, Failed}`. Closed validation
  and control error codes carry no arbitrary strings.
- A tool record stores sequence, created/updated logical tick, full typed
  attribution, known action/target or `UnknownTool`, policy audit outcome,
  approval disposition, execution `NotAttempted`, and closed error. A
  delegation record stores the same attribution/ticks plus the exact typed
  target `AgentId`, matrix result, approval `NotRequired`, control result, and
  closed error.
- Untrusted arbitrary tool names are recorded only as `UnknownTool`, never
  copied into audit. Arguments and approval affected data are never recorded.
- `AgentGovernanceTick(u64)` is a process-local logical timestamp, not wall
  time. `AgentGovernanceClock::next_tick()` is infallible; the default
  saturating sequence clock and injected deterministic test clock perform no
  I/O. Tick saturation cannot block safety cancellation and is observable in
  tests without exposing content.
- `InMemoryAgentGovernanceAudit::reserve(subject)` checks capacity and replay,
  allocates the one lifecycle record and tick before policy, approval, runtime
  cancellation, or child allocation, and returns a non-clonable reservation.
  `commit_initial` and `update_terminal` modify that existing slot infallibly.
  A reservation may be `abort`ed only before any downstream component mutation;
  otherwise downstream failure is committed as a closed failure state.
- Approval request creation occurs only after its tool record exists. Manager
  failure updates that record without partial approval state. Every resolution,
  expiry, and protective task cancellation updates the preallocated record
  after the manager returns; capacity/timestamp failure is therefore impossible
  at terminal safety handling. If the manager returns an error without state
  change, the record remains pending and the task remains live.
- `issue_presentation` has one exceptional existing manager behavior: if its
  deadline has elapsed, it consumes the pending request and returns
  `PresentationUnavailableOrExpired` rather than an `ApprovalResolution`. The
  service recognizes only that exact error while the pre-call request is due,
  terminalizes the preallocated record as `Expired`, and clears its pending
  task binding. Other presentation errors leave both manager and record in
  their actual unchanged state.
- Pre-attribution rejection is returned as a typed error and is not falsely
  attributed in audit. Matrix decisions and downstream delegation results are
  now audited; the existing content-free orchestration journal remains
  complementary lifecycle evidence.
- Records are volatile and application-owned. No persistence or retention
  policy is implied.

## Implementation milestones

- [x] Milestone 0 — record D-084, make this plan exact, synchronize roadmap
      sequencing, complete architecture/security/readiness review, and confirm
      `Ready` or `Ready with advisories` before source edits.
- [x] Milestone 1 — add the closed profile registry, definition/task/context
      binding, attribution, and delegation matrix.
- [x] Milestone 2 — add agent-origin schema validation and reuse the existing
      deterministic policy and approval boundaries without changing runtime
      capability or enabling execution.
- [x] Milestone 3 — add bounded volatile governance audit, pending-approval
      lifecycle integration, and adversarial contracts.
- [x] Milestone 4 — run focused and full validation plus independent
      architecture, security, code, and technical-debt review.
- [x] Milestone 5 — synchronize documentation, create the increment/review
      evidence, and finalize a valid completion marker.

## Security and privacy considerations

- The model, runtime, WebView, tool proposal, arguments, files, and tool results
  remain untrusted. None can construct trusted attribution or a policy profile.
- The application checks exact live task/run binding before all downstream
  lookup or mutation. Closed origin variants prevent ambiguous optional agent
  identity.
- Policy and approval remain non-executing. No current outcome reaches a tool,
  platform adapter, shell, filesystem, network, or provider.
- QA and Security remain advisory and cannot decide policy or approval.
  Workflow remains proposal-only and cannot create tasks or dispatch effects.
- Audit and Debug surfaces redact IDs and content where required and never
  retain arguments, output, secrets, or reasoning.
- Memory namespace is deliberately absent because no memory/data-bearing
  privileged action is introduced; it remains a mandatory later gate rather
  than an inert placeholder.

## Test plan

- Profile units: exact nine associations in `AgentId::ALL` order,
  duplicate/missing/mismatch,
  empty tool eligibility for eight profiles, and redacted Debug.
- Definition/task/context contracts: exact captured profile, forged/mismatched
  profile rejection, live-run comparison, and no caller constructor.
- Delegation contracts: only Personal -> Research; Research/self/reverse/
  deferred/unknown/stale/forged-profile denial with no task/run/event mutation;
  accepted request retains exact source attribution.
- Tool validation: unknown name, wrong version, malformed/oversized arguments,
  unknown task, terminal task, stale run/request, agent/task/root/parent/runtime/
  profile/depth mismatch, and forged call identity all fail closed.
- Policy matrix: Personal datetime -> Allow/NotAttempted; Personal local task ->
  RequireApproval; every other profile/tool pair -> Deny/NotAttempted; unknown
  profile and unsupported capability never default.
- Approval: full attributed presentation, one pending request, duplicate and
  replay denial, mismatch denial, expiration/cancellation, task-terminal
  cancellation, and target-mac approved/rejected source outcomes where
  applicable. Every resolution remains NotAttempted.
- Audit: policy allow/deny/approval pending/resolved records, exact attribution,
  deterministic logical ticks, sequence/order, replay of Allow and Deny,
  reservation/abort, manager-failure update, capacity exhaustion, protective
  cancellation using preallocated capacity, and Debug/content/secret/reasoning
  redaction.
- Delegation audit: allow, route deny, runtime-cancellation/child-start failure,
  child creation, audit-capacity failure before mutation, exact profile/runtime
  attribution, approval `NotRequired`, and content-free records. Explicit
  denial records cover self target, deferred target, specialist source, depth,
  active-child, and total-budget denial.
- Lifecycle: pending approval blocks runtime events/delegation; root or child
  task cancellation consumes and audits pending approval first; cancellation
  tests distinguish approval-manager failure (task/run/approval remain pending)
  from later runtime-cancel failure (approval is terminal `Cancelled` while
  task/run remain live and safely retryable). Root-with-child-pending and direct
  child cancellation prove child-first approval reconciliation.
- Expiry: ordinary expiry and presentation-after-deadline both terminalize the
  same preallocated record as `Expired`; the latter tests the manager's
  consume-and-error behavior explicitly.
- Redaction: a valid model-supplied tool-name sentinel is absent from proposal,
  error, policy, approval, audit, and service Debug output.
- Compatibility: an agent request cannot enter legacy `PolicyInput`; a legacy
  decision is rejected by the agent service while the gateway path continues
  unchanged.
- Replay tables reuse the same call ID with changed name/version/arguments
  after validation rejection, Allow, Deny, ApprovalPending, and terminal
  approval resolution; every case is rejected without new policy, approval, or
  audit state.
- Regression: runtime tool proposals still terminate at the text-only
  orchestration boundary; direct/delegated orchestration remains 22/22;
  definition, policy, approval, audit, gateway, Native runtime, and frontend
  suites remain unchanged in behavior.

## Verification commands

```bash
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::governance::tests::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::governance::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked
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

No manual UI or live-model check is required because no shipping consumer,
provider, model, IPC, permission, platform API, or user-visible behavior may
change. Target-mac approval source tests remain applicable where compiled.

## Risks

- A shared validated-call input could bypass profile checks; distinct sealed
  agent input and legacy-only `PolicyInput` prevent conversion in either
  direction.
- A cloned stale context could authorize later work; the orchestrator must
  compare it to the exact live task and run immediately before every request.
- A profile name could be mistaken for authority; profiles are immutable inputs
  to deterministic policy only, never grants or role booleans.
- Approval could be mistaken for execution; all outcomes carry explicit
  `ExecutionDisposition::NotAttempted` and no executor exists.
- Audit failure after approval mutation could lose evidence; one lifecycle slot
  is reserved before mutation and every later update is infallible.
- Generalizing legacy policy/approval could break the verified gateway lane;
  compatibility tests and the explicit origin enum are required, and a needed
  runtime/native rewrite stops the increment.

## Rollback or failure strategy

Before publication, restore the declared modified files and remove only the new
governance/audit/test/security/closeout files. After publication, use a bounded
revert plus an additive decision superseding D-084; never erase decision
history. No data, migration, dependency, IPC, permission, credential, process,
network, provider, or external-state rollback exists because none may be added.

## Decisions made

- D-084 authorizes only the bounded non-executing governance foundation.
- Delegation remains outside `ToolRegistry` and is enforced by a closed matrix.
- Nine versioned profiles provide exact attribution; one deterministic policy
  engine remains shared.
- Legacy gateway and agent policy inputs remain distinct; approval uses an
  explicit closed origin enum, never an optional identity field.
- Execution is always `NotAttempted`; memory namespace remains a later
  mandatory gate.
- The audit family is volatile, bounded, typed, redacted, and clock-injected;
  it is not the deleted generic logger.

## Discoveries

- The shared `AgentRuntime` lane deliberately exposes no native tool capability
  and the orchestrator rejects tool proposals. Governance must therefore be a
  synthetic application contract, not claimed shipping execution.
- The verified concrete native gateway turn already composes policy and
  approval but has no agent/task attribution and no production caller. It must
  remain compatible without being misrepresented as multi-agent governance.
- Existing approval source binding is target-mac and exact-subject aware; an
  agent origin can be carried through that boundary without adding an
  untrusted approval constructor.
- The current approval-resolution audit is too narrow for allow/deny decisions;
  a separate closed agent-governance event family is required instead of
  reviving a generic logger.
- The first fresh review found that pending approval/task lifecycle needs one
  named composition owner and audit needs preallocated lifecycle records. The
  revised plan selects `AgentGovernanceService` inside the orchestrator and one
  record per subject before repeating readiness.
- Implementation required a three-stage delegation-audit service surface:
  reserve, record matrix allow/deny, then finish child-created/failed. The audit
  stores the deterministic matrix result separately from later control denial.
- Exact pending approval for a Research child is unreachable through the public
  policy matrix because all specialist profiles deny current tools. Private
  unit-only seeding tests the generic child-first cancellation invariant without
  weakening Research eligibility or adding a production bypass.
- Trusted attribution construction now requires an unconstructible
  orchestrator-owned proof after exact live task/run validation. Audit tests use
  a cfg(test) orchestrator helper that performs the same validation rather than
  exposing a general constructor.
- Independent review found and corrected an agent-origin Debug leak before
  closeout. Agent-origin request-view, presentation, and resolution tests now
  prove task/root/run/request/profile/content sentinels remain absent.

## Progress

- 2026-08-12 — Published orchestration baseline at `1d1d9d6` verified clean;
  focused definition, orchestration, policy, approval, and audit tests pass.
- 2026-08-12 — Initial readiness review returned Blocked on delegation/tool
  conflation, unsupported execution, missing profile/memory decision, and
  unspecified audit semantics.
- 2026-08-12 — Owner approved the recommended bounded non-executing D-084
  direction; gate `agent-governance` became active and Milestone 0 began.
- 2026-08-12 — First fresh review remained Blocked on coordinator ownership,
  agent-policy bypass, delegation audit, replay identity, and audit
  transactionality. The plan was tightened without source edits.
- 2026-08-12 — Final architecture, security, and readiness re-review returned
  Ready with advisories after exact cancellation, expiry, delegation ordering,
  and target-attribution corrections. Documentation, repository, security, and
  diff checks pass; source implementation may begin.
- 2026-08-12 — Implemented exact profile binding, sealed live attribution,
  profile-aware policy, closed agent approval origin, non-executing governance
  service, delegation matrix, bounded volatile audit, and cancellation binding.
- 2026-08-12 — Independent code/security/architecture reviews found and closed
  agent-origin Debug redaction, matrix-audit accuracy, attribution-construction,
  and child pending-approval cancellation evidence gaps. Final review is PASS
  WITH ADVISORIES with no remaining completion blocker.
- 2026-08-12 — Focused contracts, all-target Rust, strict Clippy, complete
  repository verification, Tauri no-bundle build, docs/repository/security
  checks, and session-end inventory passed; closeout evidence was synchronized.

## Acceptance criteria

- [x] D-084 and synchronized architecture/roadmaps describe the exact
      non-executing boundary and memory-namespace deferral.
- [x] Every built-in definition, task, context, delegation request, governed
      policy input, approval, and audit record carries the exact profile and
      attribution required by this plan.
- [x] Only Personal Assistant -> Research delegation is possible; no tool or
      runtime event can create a child.
- [x] The profile/tool matrix is deterministic, least-privilege, and fail
      closed; no role, unknown input, or missing profile defaults.
- [x] Policy and approval reuse existing authoritative components and every
      outcome remains non-executing with `NotAttempted` evidence.
- [x] Audit is closed, bounded, transactional, timestamped, redacted, and
      volatile with no generic string or content surface.
- [x] Runtime, Native, gateway, existing orchestration behavior/limits, Tauri,
      React, provider, dependency, permission, and visible behavior remain
      unchanged; additive orchestrator governance composition is the only
      orchestration API expansion.
- [x] Focused/adversarial tests, complete applicable validation, independent
      reviews, documentation sync, and the completion marker pass.

## Final results

Implemented the exact D-084 non-executing governance boundary. Nine immutable
profiles are captured from sealed definitions into tasks and live contexts.
Only the orchestrator derives full attribution. The existing registry, policy,
and approval components are composed through one synthetic agent-governance
service; runtime tool proposals remain rejected. Personal date/time allows,
Personal local-task creation requires approval, every specialist denies current
tools, and execution is always `NotAttempted`.

The new volatile audit reserves one of 32 slots before downstream mutation,
updates it through terminal policy/approval/delegation state, prevents exact
subject replay, uses deterministic logical ticks, and retains no arguments or
content. Pending approvals block events/delegation and terminalize before task
cancellation, including child-first root cancellation.

Quality result: `PASS WITH ADVISORIES`. There is no remaining implementation or
security blocker and no new technical debt. Non-blocking coverage advisories
remain for agent-origin forged approval-source mismatch, approval-manager
request-creation failure, and exhaustive delegation error-record assertions.
The increment remains uncommitted and later phases remain Blocked.

## Documentation updates

- [x] D-084 and native architecture ADR
- [x] Architecture, security, threat model, and governance matrices
- [x] Root and detailed roadmaps
- [x] `PLANS.md`, `NEXT_STEPS.md`, `HANDOFF.md`, `PROJECT_STATUS.md`
- [x] `CHANGELOG.md`, increment record, and post-increment review
