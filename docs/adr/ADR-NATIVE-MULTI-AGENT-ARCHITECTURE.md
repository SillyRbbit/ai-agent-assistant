# ADR: Native multi-agent architecture

Status: Accepted
Date: 2026-08-11
Decision owners: Project owner
Decision authority: D-082
Extends: D-079
Related assessment:
[`NATIVE_MULTI_AGENT_ASSESSMENT.md`](../architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md)

Amended by D-083 on 2026-08-12: the owner combined the former implementation
sequence steps 2 and 3 into one bounded task/orchestration and deterministic
Personal-to-Research-to-Personal increment. D-083 changes sequencing only; all
ownership, trust-boundary, route, limit, and non-authority decisions below
remain accepted.

Amended by D-084 on 2026-08-12: the owner authorized a bounded non-executing
per-agent governance foundation. It adds exact versioned policy profiles,
application-derived live attribution, the closed Personal-to-Research matrix,
and typed policy/approval/audit evidence with execution always
`NotAttempted`. It does not change `AgentRuntime`, enable runtime tool
proposals, add an executor or memory namespace, or activate a specialist.

Amended by D-085 on 2026-08-12: the owner authorized a new bounded workflow-
local volatile `MemoryStore`, exact memory-profile attribution, versioned
application review, opaque approved-document references for selected lowercase
`.txt`/`.md` UTF-8 files, and one separate direct Personal Assistant-to-
Knowledge document task. The implementation and complete verification are
`PASS WITH ADVISORIES`; the accepted advisory is the narrow pure-`std` Unix
document-open TOCTOU residual. Knowledge catalog eligibility applies only
to that application-owned route; generic Personal-to-Research delegation is
unchanged, Research-to-Knowledge remains Blocked, and no persistence, IPC, UI,
provider, live model, executor, unrestricted filesystem tool, new dependency,
or device authority is added. The decision-time text below is preserved as
historical architecture evidence where its future tense or absence statements
describe the earlier checkpoint.

Amended by D-086 on 2026-08-12: the owner authorized and the Rust core now
implements one sealed fixture-only Personal Assistant -> Research -> Knowledge
-> Personal synthesis sequence. Research and Knowledge are sequential
depth-one siblings created only by `AgentOrchestrator`; generic/direct
Research-to-Knowledge remains denied. The exact workflow permits three tasks,
two non-replenishing children, one active child, four run attempts, 32 runtime
and generic events, 16 workflow events and matching descriptive audit records,
and zero retries. Strict versioned specialist and final synthesis results
preserve application-issued fixture source IDs and exact predecessor identity;
final synthesis must preserve exactly the Research source-ID set, disclose
fixture and applicable partial status, and use the stage-consistent status.
Partial results, continuation-start failure, child-first cancellation,
task-memory cleanup, pending-review Knowledge proposals, and content-free
events/audit remain bounded and non-authorizing. It adds no provider, network,
process, filesystem
read, tool, executor, persistence, IPC, UI, dependency, capability, permission,
Hermes/OpenClaw adapter, or `AgentRuntime`/`NativeAgentRuntime` widening. The
decision-time text below remains historical evidence where D-086 now supplies
the exact implemented exception.

## Context

Cortexa now contains an application-owned `AgentRuntime`/`RuntimeRun`
foundation and a sole/default `NativeAgentRuntime` that composes the unchanged
`InitialGatewayTurn`. The boundary is closed, bounded, transport-free, and
verified by deterministic tests. It is not connected to a provider, live
model, Tauri command, React, or a product coordinator.

The repository now has the separately verified nine-definition registry,
D-083's bounded task/orchestrator foundation, and D-084's non-executing
per-agent governance foundation. All are Rust-only and unwired. It still has no
memory namespace, executor, provider, Tauri consumer, frontend flow, durable
audit, or live assistant. Keeping those concerns outside `AgentRuntime`
preserves the one-run execution port and application ownership.

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

### D-086 sealed Research and Knowledge amendment

D-086 implements only the Research sequence above, through the exact
[`Research and Knowledge workflow`](../plans/2026-08-11-research-knowledge-workflow.md).
It is a trusted application-service state machine selected from an exact live
Personal Assistant root, not a `DelegationRequest`, runtime control event,
model-proposed tool, or new route in the generic delegation matrix. The
orchestrator creates Research and then, only after accepting and validating its
strict result, creates Knowledge as a separate sibling with the same root.

The sealed exception is fixed at three tasks, two non-replenishing depth-one
children, one active child, four sequential run attempts, 32 runtime events, 32
generic orchestration events, 16 workflow events, 16 matching audit records,
and zero automatic retries. It accepts one to eight immutable deterministic
fixtures; Research may reference only their application-issued IDs, and
Knowledge may preserve only IDs in the exact validated Research task/version.
Research and Knowledge inputs are each capped at 26,624 bytes, specialist
output at 8,192 scalar values/16,384 bytes, and synthesis at 36,864 bytes with
a 4,096-byte disclosure-and-framing sub-bound. A valid Research result with
missing references remains partial and skips Knowledge; a valid incomplete
Knowledge result remains partial. Unknown, duplicate, remapped, malformed,
oversized, trailing/outer, or reasoning-bearing output fails closed, and raw
invalid output never reaches another stage.

Final synthesis is a separate strict V1 result. Its answer is capped at 2,048
scalar values/8,192 bytes, its source-ID set must equal the validated Research
outcome's set, `fixture_based` must be true, and its `complete`/`partial` status
must match the specialist outcomes. The answer must disclose fixture evidence
and partial status when applicable. Invented, missing, duplicate, or unknown
references, wrong disclosure/status, URLs, live-research claims, reasoning,
malformed data, trailing/outer content, and unknown fields fail the root without
a completed workflow result.

Terminal transitions check remaining capacity and prepare parsing, task output,
the next Knowledge task/request/descriptive attribution, and fallback or
synthesis input before terminal runtime acceptance. Preparation failure causes
zero workflow mutation. Once accepted, a terminal event remains accepted even
if a continuation runtime cannot start; the consumed attempt remains consumed,
the closed continuation category is recorded, and fallback or root failure is
applied without retry. Research failure/cancellation skips Knowledge; Knowledge
failure/cancellation retains only validated Research; synthesis failure
fabricates no result. Root cancellation resolves governance and cancels the
active child before the root, records no later stage, and rejects late events.

Specialist private/task memory remains isolated and terminal cleanup removes
task-temporary records. Structured results, not memory, are the only sibling
transport. Reusable Knowledge remains pending review and is not automatically
created, approved, selected, persisted, or treated as fact in `MemoryStore`.
Workflow events and the volatile `ResearchKnowledgeAuditRecord` are
content-free. Their `ResearchKnowledgeAttribution` is a descriptive snapshot
whose run/request identity stays private and redacted and which cannot recreate
live policy, approval, memory, runtime, or execution authority.

This amendment changes no provider, network, process, filesystem, tool,
executor, persistence, IPC, UI, dependency, capability, permission, external
runtime, `AgentRuntime`, or `NativeAgentRuntime` boundary. Native remains the
sole/default implementation; Hermes remains Deferred/Blocked and OpenClaw
remains evaluation-only.

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
2. Add task lifecycle, execution context, orchestration limits, the explicit
   delegation service, and one deterministic Personal-to-Research-to-Personal
   proof with the initial one-child-total, depth-one, sequential limit and
   without a live provider or UI.
3. Extend per-agent governance and knowledge/document boundaries through
   separate accepted designs before any privilege or persistence is added.
4. Activate the research/knowledge, engineering-quality,
   infrastructure/operations, and automation workflow families only through
   their separate plans and exact route/tool/data limits.
5. Consider bounded parallelism, desktop UI, end-to-end demonstrations, and a
   final architecture/security review only after their prerequisites are
   verified.

The authoritative phases are in [`ROADMAP.md`](../../ROADMAP.md). This ADR does
not authorize implementation by itself.

D-086 completes only the research/knowledge portion of step 4 through its
separately accepted sealed plan. D-087 separately completes only the fixture-
based, proposal-only engineering-quality portion: Coding, QA, and Security are
sequential depth-one siblings created by the orchestrator; strict structured
results preserve application fixture/evidence provenance; every consequential
capability remains denied; and no approval request or execution path exists.
Infrastructure/operations, automation, bounded parallelism, UI/provider wiring,
live repository effects, and every other later phase remain gated. Neither
D-086 nor D-087 makes this ADR general implementation authority.

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
- D-086 supplies the only implemented exception to the original future
  Research/Knowledge family: the fixed fixture workflow described above. It
  leaves D-079's runtime boundary and every generic delegation rule unchanged.
- D-087 supplies the implemented exception for the engineering-quality family:
  one fixed fixture-only, proposal-only sibling sequence. It changes no generic
  route, tool/policy/memory profile, approval/execution boundary,
  `AgentRuntime`, or `NativeAgentRuntime` behavior.

## Rollback

Before implementation, rollback removes this ADR and related planning documents
and adds an explicit superseding decision if the accepted direction changes.
After implementation, each bounded phase must remain independently revertible;
the existing runtime, native turn, governance boundaries, and deterministic
mocks remain the stable fallback.
