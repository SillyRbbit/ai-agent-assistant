# ADR: Native multi-agent architecture

Status: Accepted
Date: 2026-08-11
Decision owners: Project owner
Decision authority: D-082
Extends: D-079
Related assessment:
[`NATIVE_MULTI_AGENT_ASSESSMENT.md`](../architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md)

## Context

Cortexa now contains an application-owned `AgentRuntime`/`RuntimeRun`
foundation and a sole/default `NativeAgentRuntime` that composes the unchanged
`InitialGatewayTurn`. The boundary is closed, bounded, transport-free, and
verified by deterministic tests. It is not connected to a provider, live
model, Tauri command, React, or a product coordinator.

The repository has no product-agent definitions, registry, task model,
orchestrator, delegation service, agent-aware governance, or agent memory.
Adding those concerns inside `AgentRuntime` would turn a one-run execution port
into a broad framework interface and weaken application ownership.

The evaluated Hermes Agent `0.20.0` / tag `v2026.8.3` raw stdio, managed
`hermes serve` WebSocket, and ACP paths did not satisfy their approved
transport, provenance, containment, or governance conditions. Their evidence
is preserved. No replacement Hermes transport is selected.

The project owner has selected a native multi-agent direction appropriate for a
private, owner-only, local-first personal project while preserving clean seams
for possible future publication.

## Decision

Adopt application-owned native multi-agent orchestration as the primary agent
architecture.

The diagram is a logical task/delegation topology, not a direct call or
authority graph. `AgentOrchestrator` invokes `AgentRuntime` for each root or
child run; definitions do not invoke runtimes.

```text
User -> Personal Assistant -> AgentOrchestrator
                              |
                              +-- Research and knowledge
                              |   +-- Research Agent
                              |   `-- Knowledge & Document Agent
                              +-- Software engineering
                              |   +-- Coding Agent
                              |   +-- QA & Validation Agent
                              |   `-- Security & Risk Agent
                              +-- Infrastructure and operations
                              |   +-- Cloud Infrastructure Agent
                              |   +-- Systems Operations Agent
                              |   +-- QA & Validation Agent
                              |   `-- Security & Risk Agent
                              `-- Automation
                                  `-- Workflow Automation Agent

AgentOrchestrator -> AgentRuntime -> NativeAgentRuntime (sole/default)
```

### Application-owned agent catalog

The first definition/registry increment introduces all nine closed,
application-owned roles. Functional grouping is exact:

| Functional group              | Roles                                                                                              |
| ----------------------------- | -------------------------------------------------------------------------------------------------- |
| Core orchestration            | Personal Assistant                                                                                 |
| Research and knowledge        | Research Agent; Knowledge & Document Agent                                                         |
| Software engineering          | Coding Agent; QA & Validation Agent; Security & Risk Agent                                         |
| Infrastructure and operations | Cloud Infrastructure Agent; Systems Operations Agent; QA & Validation Agent; Security & Risk Agent |
| Automation                    | Workflow Automation Agent                                                                          |

QA & Validation and Security & Risk are cross-cutting roles. A separately
approved workflow may use them for software, infrastructure, operations,
document, or automation review without granting either role authority over the
reviewed work.

Role responsibility and activation posture are bounded as follows:

| Role                       | Responsibility and non-authority boundary                                                                                                                                                                                            | Staged activation                                                  |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------ |
| Personal Assistant         | User-facing classification, controlled delegation requests, progress communication, synthesis, and approval explanation; no unrestricted privileged tools.                                                                           | Initially catalog-eligible for the later deterministic first flow. |
| Research Agent             | Evidence-backed internal or external research through separately governed tools; read-only by default and no implicit browser, network, file, or memory access.                                                                      | Initially catalog-eligible for the later deterministic first flow. |
| Knowledge & Document Agent | Approved-file reading, comparison, extraction, organization, and preparation; no unrestricted crawling, access outside approved roots, or silent durable shared-memory writes.                                                       | Gated on the knowledge/document boundary and memory phase.         |
| Coding Agent               | Repository inspection, explanation, planning, patch proposals, approved changes, and approved tests; no autonomous commit, push, dependency installation, or destructive commands.                                                   | Gated on the engineering phase.                                    |
| QA & Validation Agent      | Test planning, acceptance and regression assessment, output/configuration validation, and approved safe validation tools; no self-approval and not `ApprovalManager`.                                                                | Gated on the engineering-quality phase.                            |
| Security & Risk Agent      | Advisory threat modeling, secrets/policy review, security and change-risk analysis; not `PolicyEngine` and cannot authorize or execute remediation.                                                                                  | Gated on the engineering-security phase.                           |
| Cloud Infrastructure Agent | Cloud architecture, Azure/AWS analysis, Terraform review, approved read-only inventory, and change planning; no autonomous apply, mutation, deletion, IAM change, or credential use.                                                 | Gated on the infrastructure phase.                                 |
| Systems Operations Agent   | Approved read-only diagnostics and operational planning for supported systems, services, logs, patching, backup, and virtualization; no autonomous restart, shutdown, deletion, configuration/account change, or privileged shell.   | Gated on the infrastructure/operations phase.                      |
| Workflow Automation Agent  | Typed workflow proposals and dependency/sequence composition; no arbitrary execution, bypass of `AgentOrchestrator`, `ToolRegistry`, `PolicyEngine`, `ApprovalManager`, or `AuditLogger`, recursive expansion, or self-modification. | Gated until typed workflows and their governance exist.            |

Catalog eligibility is closed application state, not operational evidence or
authorization. The first increment wires no definition to a runtime, provider,
tool, Tauri command, or frontend. Personal Assistant and Research Agent are
only eligible for a later deterministic no-I/O flow; the other seven fail
closed as gated until their named phase is separately approved and verified.
Registry membership, grouping, purpose, or activation posture grants no route,
tool, policy outcome, memory, provider, credential, or device permission.

### Ownership

- Agents are immutable or configuration-driven application-owned
  `AgentDefinition` values resolved through an application-owned
  `AgentRegistry`; definitions include closed activation posture but no
  privileges. Functional groups are a normative catalog/documentation mapping,
  not a routing or authorization field in the first definition model.
- `AgentOrchestrator` is a separate application service above
  `AgentRuntime`. It owns task assignment, bounded delegation, task lifecycle,
  workflow sequencing, result collection, attribution, and cancellation
  propagation. It is the only component that may create or schedule an agent
  task.
- `AgentRuntime` continues to execute one bounded run. It does not own agent
  discovery, task routing, child creation, policy profiles, memory namespaces,
  or orchestration state.
- `NativeAgentRuntime` remains the sole/default, reference, deterministic test,
  explicit-fallback, and potentially publishable runtime.
- Tool identity and schemas, deterministic policy, exact approval, restricted
  execution, audit, credentials, and memory remain application-owned and
  outside the generic `AgentRuntime` contract and orchestrator authority.
  `NativeAgentRuntime` continues to compose the existing application-owned
  `InitialGatewayTurn` without transferring that authority into the contract.

`AgentOrchestrator`, `AgentRuntime`, `NativeAgentRuntime`, `AgentRegistry`,
`ToolRegistry`, `PolicyEngine`, `ApprovalManager`, the future application-owned
`AuditLogger`, the future application-owned `MemoryStore`, and a future bounded
application-owned `PlatformAdapter` remain authoritative application
components within their separate boundaries. Naming a planned component here
is not implementation evidence and does not revive the deleted generic
scaffolds. Agents may return recommendations or typed requests; application
code alone validates lifecycle and routes, authorizes through `PolicyEngine`,
binds exact approval through `ApprovalManager`, records through audit, and
dispatches any separately approved restricted effect through registered tools
and a bounded platform adapter. Security & Risk is not `PolicyEngine`, QA &
Validation is not `ApprovalManager`, and Workflow Automation is not
`AgentOrchestrator`. The orchestrator coordinates validated agent work; it does
not become an unrestricted device-effect executor.

### Initial Personal-to-Research bounds

- All nine definitions exist in the first planned catalog, but only Personal
  Assistant and Research Agent are initially catalog-eligible; none is yet
  operational.
- The root is exactly Personal Assistant. The only initial child edge is
  Personal Assistant to Research Agent; Research Agent cannot delegate, and
  self, reverse, unknown, or other routes fail before task creation.
- Initial delegation depth is one.
- Each root may create at most one child task total and at most one child may be
  active; completion or cancellation does not replenish the budget.
- Only `AgentOrchestrator` may create a child task.
- No agent, runtime, model output, tool proposal, or WebView event may directly
  spawn another agent.
- Every task and action must be attributed to a validated agent, task, optional
  parent task, runtime, policy profile, and memory namespace before privileged
  behavior is introduced.
- Missing, unknown, or mismatched identity fails closed.
- Tasks and cancellation are closed, bounded, and deterministic in ordinary
  tests.

The one-total-child budget is the bound for the first
Personal-to-Research milestone, not a hidden claim that later sequential
workflow families already fit that budget. Each later workflow plan must
separately set an exact finite total-child budget for its allowlisted sequence.
Maximum depth remains one: every specialist task is a direct child of the
Personal Assistant root. Active-child concurrency remains one until the later
bounded-parallelism phase changes it through a separate decision and plan.

### Future staged workflow families

The arrows below express orchestrator-owned sequencing and bounded result flow,
not direct agent-to-agent spawning. A completed specialist result may become
bounded untrusted input to the next direct child task only after application
validation.

- **Research:** Personal Assistant root -> Research Agent child -> Knowledge &
  Document Agent child -> Personal Assistant synthesis. This later route needs
  an explicit total-child budget of two.
- **Engineering:** Personal Assistant root -> Coding Agent child -> QA &
  Validation Agent child -> Security & Risk Agent child -> Personal Assistant
  synthesis -> approval before any consequential application action. This
  later route needs an explicit total-child budget of three.
- **Infrastructure and operations:** Personal Assistant root -> exactly one of
  Cloud Infrastructure Agent or Systems Operations Agent -> QA & Validation
  Agent -> Security & Risk Agent -> Personal Assistant synthesis -> approval
  before any consequential application action. This later route needs an
  explicit total-child budget of three.
- **Automation:** Personal Assistant root -> Workflow Automation Agent child ->
  application validation of the typed proposal -> QA & Validation Agent child
  -> Security & Risk Agent child -> owner approval where required ->
  `AgentOrchestrator` coordinates only the validated, approved workflow. This
  later route needs an explicit total-child budget of three agent tasks; any
  device effect still traverses the separately governed application execution
  boundary.

Specialists never create children. Workflow Automation proposes a typed
workflow but cannot schedule or execute it. Self, reverse, recursive,
unallowlisted, unknown, unavailable, or over-budget routes fail before task
creation. Every root and child task remains independently attributable and
cancellable, and root cancellation propagates exactly once to the active
child.

### Delegation mechanism

Use an explicit application-service call with a typed `DelegationRequest`.
Do not make child creation a runtime control event or an `agent.delegate` host
tool. A future untrusted model proposal may be translated into a bounded request,
but only the orchestrator validates it and creates the child task.

The allowed-route policy is application/orchestrator-owned, not definition
metadata. Registry membership identifies a known agent but never authorizes a
delegation edge.

### Framework and runtime direction

No external agent framework is required for the initial implementation. Native
application-owned definitions, tasks, and orchestration are the primary path.

Hermes integration is **Deferred — evaluated transport and containment
requirements not met**. `HermesAgentRuntime` remains Draft/Blocked, no Hermes
dependency is added, and no alternate transport is selected through this
decision. Deferral is not permanent abandonment; a future release or mechanism
may be considered only through a separate owner-approved decision and evidence.

Future external runtimes remain possible behind the existing `AgentRuntime`
port. External runtime types and behavior must stay inside a narrow adapter and
cannot acquire tool, policy, approval, execution, audit, credential, memory, or
device authority.

## Alternatives considered

### Put orchestration inside `AgentRuntime`

Rejected. It would broaden every runtime implementation with application task,
delegation, and governance semantics and make a later external adapter an
authority surface.

### Represent delegation as `agent.delegate`

Rejected for the initial design. It would conflate internal application task
routing with model-proposed host tools. The current shared native runtime also
does not support untrusted tool proposals.

### Adopt an external agent framework first

Rejected for the initial design. No current framework has demonstrated value
that justifies dependency, packaging, lifecycle, compatibility, or authority
cost. The application-owned domain model is small enough to implement directly.

### Stay permanently single-agent

Not selected. It is the smallest implementation but does not support the
owner-selected bounded delegation use case. Native single-run execution remains
a reusable boundary inside the multi-agent design.

## Consequences

Positive consequences:

- verified native runtime and governance work is preserved;
- orchestration remains independently testable and framework-neutral;
- specialist roles do not imply privileges;
- depth and concurrency are explicit rather than emergent;
- deterministic no-I/O development can precede providers and tools; and
- future external runtimes remain replaceable adapters rather than system
  owners.

Costs and constraints:

- the application needs new definition, registry, task, orchestration, context,
  and later governance types;
- current runtime requests do not carry agent/task lineage;
- a completed runtime turn is not by itself a complete agent task;
- agent-aware policy, approval, audit, and memory require separate end-to-end
  increments; and
- no live assistant behavior exists until separately approved provider and
  application-integration work is complete.

## Implementation sequence

1. Add only immutable framework-neutral definitions and a deterministic
   registry for all nine roles, with only Personal Assistant and Research Agent
   catalog-eligible for the later first flow and every definition still inert.
2. Add task lifecycle, execution context, orchestration limits, and the explicit
   delegation service with the initial one-child-total, depth-one, sequential
   limit and without a live provider or UI.
3. Prove one deterministic Personal-to-Research-to-Personal flow.
4. Extend per-agent governance and knowledge/document boundaries through
   separate accepted designs before any privilege or persistence is added.
5. Activate the research/knowledge, engineering-quality,
   infrastructure/operations, and automation workflow families only through
   their separate plans and exact route/tool/data limits.
6. Consider bounded parallelism, desktop UI, end-to-end demonstrations, and a
   final architecture/security review only after their prerequisites are
   verified.

The authoritative phases are in [`ROADMAP.md`](../../ROADMAP.md). This ADR does
not authorize implementation by itself.

## Preservation guarantees

- Do not delete, duplicate, or weaken `InitialGatewayTurn`,
  `GatewayStreamValidator`, local function validation, tool schemas,
  deterministic policy, approval binding, typed approval audit,
  `AgentRuntime`, `NativeAgentRuntime`, or deterministic mocks.
- Do not revive D-030/D-032/D-033/D-034 generic scaffolds.
- Do not wire an agent, runtime, task, tool, memory, or Hermes concept directly
  to React or raw Tauri IPC.
- Do not infer network, browser, shell, filesystem, memory, cloud, or provider
  access from an agent name.
- Do not create recursive delegation, background autonomy, automatic failover,
  or autonomous destructive operations.
- Preserve every Hermes assessment, ADR, spike, fixture, and NO-GO record.

## Scope relationship to earlier decisions

- D-079 remains accepted and unchanged; this decision places an application
  service layer above its runtime seam.
- D-080 remains the historical accepted evaluation decision whose selected
  WebSocket mechanism later failed.
- D-081 remains the accepted ACP rejection for the pinned release.
- D-065 remains in force for no-implicit-authority rules, implementation
  prohibitions, and trust boundaries; D-082 supersedes only its then-current
  documentation-only limitation by accepting this bounded architecture.
- D-078's personal-project scope and possible future publication direction
  remain unchanged.

## Rollback

Before implementation, rollback removes this ADR and related planning documents
and adds an explicit superseding decision if the accepted direction changes.
After implementation, each bounded phase must remain independently revertible;
the existing runtime, native turn, governance boundaries, and deterministic
mocks remain the stable fallback.
