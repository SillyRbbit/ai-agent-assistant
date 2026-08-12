# Native multi-agent architecture documentation

Status: Complete with advisories; uncommitted for owner review
Owner: Project owner
Last updated: 2026-08-12

## Goal

Assess and accept the smallest native multi-agent architecture above the
implemented `AgentRuntime`/`NativeAgentRuntime` foundation, record a bounded
roadmap, and make only the AgentDefinition/AgentRegistry phase Ready without
changing production behavior.

## User-visible outcome

None. Cortexa keeps the existing deterministic React mock and unwired native
runtime foundation. No agent definition, task, delegation, provider, tool,
memory, process, dependency, IPC, or UI behavior is added.

## Scope

- Inventory the current runtime, native turn, mock, tools, policy, approvals,
  audit, memory, Tauri, frontend, and tests.
- Add an evidence-backed native multi-agent assessment.
- Accept one ADR that places `AgentOrchestrator` above `AgentRuntime` and keeps
  Native sole/default.
- Append D-082 without rewriting D-078 through D-081.
- Add the staged native multi-agent sequence to the authoritative root roadmap
  and a subordinate catalog-specific roadmap.
- Create one Ready, owner-approved but inactive AgentDefinition/AgentRegistry
  ExecPlan covering all nine roles with closed staged activation.
- Create concise Blocked plans for the owner-specified orchestration,
  governance, knowledge/document, workflow, parallelism, UI, demonstration, and
  final-review boundaries. No follow-on becomes Ready through this increment.
- Correct current documentation that still calls the implemented native
  runtime foundation conceptual or says no multi-agent design is accepted.
- Synchronize current project memory and completion evidence.

## Explicit non-goals

- No production or test source change.
- No `AgentDefinition`, `AgentRegistry`, `AgentTask`, `AgentOrchestrator`,
  delegation, agent policy, agent memory, concurrency, specialist-agent
  behavior, or UI implementation.
- No provider, model, network, process, tool executor, storage migration,
  credential, platform permission, Tauri command/event, or dependency.
- No Hermes installation, execution, dependency, transport evaluation,
  adapter, evidence deletion, or candidate modification.
- No new external agent framework.
- No branch, commit, push, PR, merge, release, or publication.

## Existing behavior and constraints

- Baseline: clean synchronized `main` at `641ccac`.
- `AgentRuntime`/`RuntimeRun` and sole/default `NativeAgentRuntime` are
  implemented, published, and verified, but no production caller exists.
- `NativeAgentRuntime` composes one unchanged `InitialGatewayTurn`; Tauri
  exposes only `get_app_info`, and React uses `browserMockRunDriver`.
- `MockAgentRuntime` is private deterministic contract-test infrastructure.
- There is no provider, live model, task, definition, registry, orchestrator,
  delegation, agent context, product memory, or tool executor.
- Raw TUI-gateway stdio, managed `hermes serve` WebSocket, and ACP were rejected
  for the exact evaluated Hermes Agent `0.20.0` / `v2026.8.3` / commit
  `3c27eb6234bf91b8ceee9e9071591b31e9b148cb` conditions.
- The owner explicitly accepts native multi-agent architecture but authorizes
  documentation and planning only in this increment.
- A Ready plan is queue/readiness evidence; it never grants implementation
  authority by itself.

## Current-state and symbol inventory

- `src-tauri/src/agent/runtime.rs`
  - `AgentRuntime::{describe,start}`
  - `RuntimeRun::{run_id,identity,status,accept_event,cancel}`
  - closed runtime descriptor, capability, request, identity, event, status,
    cancellation, and error types
- `src-tauri/src/agent/native_runtime.rs`
  - `NativeAgentRuntime`
  - `NativeAgentRun::{request_bytes,accept_frame,cancel_pending_approval_for_run_termination}`
  - concrete/shared input-lane isolation and unchanged native governance output
- `src-tauri/src/agent/gateway_request.rs`
  - `InitialGatewayTurn` as the transport-free request, validation, policy,
    approval, and typed-audit composition
- `src-tauri/tests/agent_runtime_contract.rs`
  - 20 runtime/native/private-mock tests
- `src-tauri/src/tools/`, `policy/`, `approvals/`, and `audit/`
  - independent application-owned trust boundaries
- `src-tauri/src/lib.rs`
  - only `get_app_info` IPC
- `src/App.tsx` and `src/application/`
  - separate deterministic browser mock and volatile state

## Files expected to change

- `AGENTS.md`
- `docs/governance/MASTER_PROMPT.md`
- `docs/PROJECT_DIRECTION.md`
- `ENGINEERING_GUIDE.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md` (new)
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md` (new)
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md` (new)
- `docs/architecture/final-vision/ai-agent-assistant-architecture-summary.md`
- `docs/plans/2026-08-11-hermes-agent-runtime-adapter.md`
- `docs/plans/2026-08-11-native-multi-agent-architecture.md` (new)
- `docs/plans/2026-08-11-agent-definition-registry.md` (new)
- `docs/plans/2026-08-11-agent-orchestration-task-lifecycle.md` (new)
- `docs/plans/2026-08-11-agent-governance.md` (new)
- `docs/plans/2026-08-11-knowledge-document-boundaries.md` (new)
- `docs/plans/2026-08-11-agent-memory.md` (new)
- `docs/plans/2026-08-11-research-knowledge-workflow.md` (new)
- `docs/plans/2026-08-11-engineering-quality-workflow.md` (new)
- `docs/plans/2026-08-11-infrastructure-systems-operations-workflow.md` (new)
- `docs/plans/2026-08-11-workflow-automation.md` (new)
- `docs/plans/2026-08-11-bounded-agent-parallelism.md` (new)
- `docs/plans/2026-08-11-multi-agent-ui.md` (new)
- `docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md` (new)
- `docs/plans/2026-08-11-native-multi-agent-security-architecture-review.md`
  (new)
- `docs/increments/native-multi-agent-architecture.md` (new)
- `docs/reviews/2026-08-11-native-multi-agent-architecture-post-increment-review.md`
  (new)

Any production, test, dependency, manifest, lockfile, configuration, workflow,
hook, skill, Hermes evidence, or additional documentation path requires owner
approval before editing.

## Interfaces and invariants

- Target layering is `AgentOrchestrator -> AgentTask -> AgentRuntime ->
NativeAgentRuntime`.
- `AgentRuntime` remains a one-run execution boundary and does not own routing,
  delegation, agent identity, policy, memory, tools, approvals, audit, provider,
  or UI.
- The catalog contains nine application-owned, privilege-free definitions with
  closed, non-authorizing staged activation. Only Personal Assistant and
  Research are initially selected for a later flow; none is operational.
- Only the orchestrator may create a child task.
- In the initial Personal-to-Research flow, depth, total child budget per root,
  and active-child concurrency are one without replenishment. Later workflows
  remain depth one and require exact finite total-stage caps.
- The initial root is Personal Assistant and only its Research edge is enabled;
  later routes remain Blocked and are always orchestrator-sequenced, never
  specialist-spawned.
- Native remains sole/default; no runtime selector or automatic fallback is
  added.
- Every later action must carry trusted agent/task/parent/runtime/policy/memory
  identity; missing or mismatched identity fails closed.
- Existing native typed boundaries, mocks, contracts, and tests are preserved.
- Hermes is Deferred — evaluated transport and containment requirements not
  met. No current transport is selected.
- Historical decision and spike evidence remains unchanged.

## Documentation milestones

- [x] Milestone 0 - inspect the clean baseline, prior gate, required governance,
      runtime source, tests, and current project memory.
- [x] Milestone 1 - add the assessment, Accepted ADR, and D-082.
- [x] Milestone 2 - update the authoritative roadmap and create the Ready first
      implementation plan plus the separately Blocked follow-on plans.
- [x] Milestone 3 - reconcile current direction, architecture, status, queue,
      handoff, changelog, and related planning documents.
- [x] Milestone 4 - run documentation validation, independent reviews,
      session-end, quality, and post-increment completion gates.

## Security and privacy considerations

This increment handles no runtime input, model output, personal content,
credential, process, network, or device action. Its design keeps agents and
runtimes non-authorizing; retains application ownership of tools, policy,
approval, execution, audit, and memory; requires fail-closed identity binding;
and prohibits recursive spawning or privilege derived from an agent name.

Hermes evidence is preserved because the evaluated mechanisms exposed
provenance, containment, lifecycle, and internal-tool-authority failures. No
new transport is inferred from the native pivot.

## Validation plan

Required documentation-tier commands after the final edit:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Also verify:

- the changed inventory is documentation only;
- no `src/`, `src-tauri/`, dependency, manifest, lockfile, config, workflow,
  hook, or skill path changed;
- D-082 and the Accepted ADR agree;
- exactly one implementation plan is Ready;
- orchestration and later phases remain Blocked/Future;
- Hermes assessment, ADR, spike, fixture, increment, and review evidence is
  preserved;
- Native remains sole/default and is not claimed live or wired; and
- no manual application check is required for a documentation-only change.

Then run the repository quality-gate and post-increment-gate workflows and
validate the final report/fingerprint marker.

## Risks and mitigations

- **Stale current-state prose:** distinguish implemented-but-unwired Native from
  live application behavior everywhere.
- **Runtime overreach:** keep orchestration and context outside `AgentRuntime`.
- **Privilege by role name:** keep first definitions privilege-free and defer
  tool/policy/memory fields until enforced.
- **Speculative domain bloat:** omit `AgentInstance` and defer `AgentEvent` until
  a consumer proves need.
- **Plan sprawl:** create only the exact owner-specified follow-on stubs, keep
  them concise and Blocked, and keep authoritative sequencing in `ROADMAP.md`.
- **Historical rewriting:** append D-082 and current notices; do not rewrite
  dated Hermes or earlier decision evidence.
- **Premature execution:** leave the completed documentation uncommitted and
  block the first code run until separate owner publication/authorization.

## Rollback approach

Before publication, revert or remove only this plan's declared documentation
paths. After publication, use one bounded revert plus an additive superseding
decision if the owner changes direction. Preserve D-078 through D-081 and all
Hermes evidence. No source, dependency, data, credential, process, or external
state rollback is required.

## Acceptance criteria

- [x] The assessment covers all twelve owner-requested sections with concrete
      file/symbol evidence.
- [x] The native multi-agent ADR is Accepted and recorded as D-082.
- [x] Native multi-agent orchestration is above, not inside, `AgentRuntime`.
- [x] Native remains sole/default and existing behavior is not overstated.
- [x] Hermes is accurately Deferred and all evidence is preserved.
- [x] Root `ROADMAP.md` contains the ten requested phases.
- [x] The subordinate roadmap records all nine roles, staged activation, four
      future workflow families, and the separately Blocked plan set.
- [x] Only AgentDefinition/AgentRegistry is Ready.
- [x] The immediate orchestration plan and every later phase remain gated.
- [x] Documentation validation and all required gates pass after the steering
      update.
- [x] No production, test, dependency, config, workflow, hook, or skill path
      changes.
- [x] The increment remains uncommitted for owner review.

## Progress

- 2026-08-11: owner selected the documentation-only native multi-agent pivot.
- 2026-08-11: baseline, source, test, governance, architecture, and planning
  inventories completed; readiness is Blocked for code and Ready for this
  bounded documentation increment.
- 2026-08-11: gate `native-multi-agent-architecture` started from clean
  synchronized commit `641ccac`.
- 2026-08-11: assessment, Accepted ADR/D-082, root roadmap, Ready first plan,
  Blocked immediate follow-on, and current-state reconciliation completed.
- 2026-08-11: documentation, repository, security, diff, protected-path, and
  session-end checks passed; independent architecture and security reviews
  found no remaining actionable issue.
- 2026-08-11: owner steering expanded the planned catalog from four to nine
  roles; completion evidence was reopened before gate finalization for catalog,
  activation, workflow, roadmap, and Ready-plan reconciliation.

## Discoveries

- The current runtime request lacks agent/task/policy/memory identity, so a
  later execution context must bind those fields above the runtime before
  governed actions.
- A completed runtime turn may still own a pending approval; task status cannot
  simply alias `RuntimeRunStatus`.
- The Native shared event path does not support untrusted tool proposals, making
  an `agent.delegate` tool a poor initial orchestration mechanism.
- `AgentInstance` and a separate orchestration `AgentEvent` are not yet
  justified.
- Catalog activation needs a closed descriptive state distinct from registry
  membership, runtime availability, route policy, and authorization.
- Future workflow arrows preserve depth one only when every specialist is an
  orchestrator-created sibling under the Personal root; each workflow requires
  an exact finite total-child cap.

## Final results

The documentation-only increment is complete with `PASS WITH ADVISORIES`.
D-082 accepts an application-owned `AgentOrchestrator` above the implemented
one-run runtime and a nine-role catalog with closed staged activation. The sole
Ready plan defines all nine roles but marks only Personal Assistant and Research
`Initial`; none is operational. The root roadmap retains ten phases, the
subordinate roadmap expands their staged work, and all 12 later plans remain
Blocked. Hermes is Deferred/Blocked and all historical evidence remains intact.

Documentation, repository, security, diff, protected-path, and session-end
checks pass. Architecture, security, code-health, technical-debt, and readiness
reviews have no actionable findings. No manual application check is required.

Advisory: this completed documentation remains intentionally uncommitted. The
Ready plan may start only after owner-reviewed publication to a clean baseline
and a separate exact owner implementation task.

## Documentation updates

Synchronized root instructions, master/engineering guidance, project direction,
architecture, D-082, both roadmaps, plan queue, handoff, project status, next
steps, changelog, related runtime/final-vision/Hermes disposition records, this
plan, the increment record, 12 Blocked follow-ons, and the consolidated review.
Historical Hermes evidence remains unchanged.
