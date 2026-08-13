# Native multi-agent architecture assessment

Status: Owner-accepted architecture assessment; catalog, governance, memory/document, and sealed fixture workflow foundations implemented
Assessment date: 2026-08-11
Last reconciled: 2026-08-13
Decision authority:
[`ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`](../adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md)
and D-082 through D-090

This assessment records the smallest native multi-agent direction supported by
the repository's current code and trust boundaries. The catalog increment
implements agent definitions and a registry. D-083's combined increment now
also implements a bounded task domain and deterministic Personal-to-Research-
to-Personal application-service contract. D-084 adds exact profiles, live
attribution, non-executing policy/approval composition, a closed delegation
matrix, and bounded volatile governance evidence. None is connected to Tauri,
React, a provider, live model, executor, memory store, or external runtime at
that D-084 published checkpoint.

> **Additive current-state note (D-085, 2026-08-12):** The focused D-085
> implementation now adds a new workflow-local volatile `MemoryStore`, sealed
> memory-profile attribution, explicit bounded context selection, versioned
> application review, opaque approved-document references for selected
> lowercase `.txt`/`.md` UTF-8 files, and one separate direct Personal
> Assistant-to-Knowledge document task. Focused memory, document, integration,
> registry, governance, orchestration, and runtime contracts, complete
> repository validation, and independent review pass. The result is
> `PASS WITH ADVISORIES`; the only accepted residual is the pure-`std` Unix
> document-open TOCTOU race. The boundary has no persistence, IPC, UI, provider,
> live model, executor, unrestricted file tool, vector index, new dependency, or
> device effect. Generic Personal-to-Research remains unchanged and
> Research-to-Knowledge remained Blocked at that checkpoint. Earlier absence,
> catalog-state, and future-target statements below remain the dated assessment
> checkpoint rather than current D-085 capability evidence.

> **Additive current-state note (D-086, 2026-08-12):** D-086 now implements one
> separately selected, Rust-only, fixture-only Personal Assistant -> Research ->
> Knowledge -> Personal synthesis application-service sequence. Research and
> Knowledge are sequential depth-one siblings created only by
> `AgentOrchestrator`; generic/direct Research-to-Knowledge remains denied and
> no specialist can spawn. The sealed path permits three tasks, two
> non-replenishing children, one active child, four runtime-run attempts, 32
> runtime and generic events, 16 workflow events and matching content-free
> attribution records, and zero automatic retries. One to eight immutable
> application fixtures supply the only source IDs. Strict Research V1 and
> Knowledge V1 results preserve exact predecessor/source provenance, label
> missing references as partial, skip Knowledge when Research lacks complete
> source attribution, retain valid incomplete Knowledge as partial, and reject
> unknown, duplicate, remapped, malformed, oversized, or reasoning-bearing
> output. The strict final synthesis V1 envelope also requires the exact
> Research source-ID set, fixture disclosure, applicable partial disclosure, and
> stage-derived status; invented citations, URLs, live-research claims,
> reasoning, and unknown fields fail the root. Terminal parsing, remaining
> capacity, task output, next-task/request attribution, and fallback
> or synthesis input are prepared before terminal event acceptance; preparation
> failure has zero workflow mutation, and a later continuation-start failure
> does not reverse the accepted terminal event. Research or Knowledge failure
> yields only typed partial synthesis input; root cancellation is child-first
> and starts no later stage. Specialist private/task memory stays isolated,
> task memory is cleaned at terminal state, and reusable Knowledge remains
> pending review. The workflow journal and audit expose no source content,
> objective, finding, summary, proposal, path, URL, output, or reasoning, and
> their descriptive attribution cannot grant authority. Focused 12-unit and
> 18-contract evidence passes. No provider, network, process, filesystem read,
> tool, executor, persistence, IPC, UI, dependency, permission, external runtime,
> or `AgentRuntime`/`NativeAgentRuntime` widening was added. Earlier future and
> absence statements below remain decision-time evidence where this note now
> records the superseding current state.

> **Additive current-state note (D-087, 2026-08-12):** D-087 now implements one
> separately selected, Rust-only, fixture-only and proposal-only Personal
> Assistant -> Coding -> QA & Validation -> Security & Risk -> Personal
> synthesis application-service sequence. The specialists are sequential
> depth-one siblings created only by `AgentOrchestrator` under four-task,
> three-child, five-run, one-active-child, 32-event, 16-workflow/audit-record,
> and zero-retry limits. Strict V1 `ChangeProposal`, `ValidationReport`, and
> `RiskAssessment` results preserve application-issued fixture, criterion,
> proposal, QA, and evidence provenance. QA cannot approve or fabricate
> execution; Security is evidence-bound or explicitly hypothetical and cannot
> authorize, remediate, or become policy. Consequential capabilities remain
> denied data. Final synthesis discloses fixture/proposal-only input and no
> execution, and derives `RequiredBeforeMutation` for a patch without creating
> an approval request. Coding, QA, and Security are `Initial` only for this
> sealed unwired workflow; generic routes and their tool-ineligible, memory-
> disabled profiles are unchanged. No live repository/filesystem/process/Git/
> package/network access, tool, executor, mutation, dependency, IPC/UI,
> provider, external runtime, or `AgentRuntime`/`NativeAgentRuntime` widening
> was added. Earlier catalog/future statements below remain decision-time
> evidence where this note records the superseding current state.

> **Additive current-state note (D-088, 2026-08-12):** D-088 implements two
> separately selected, Rust-only, fixture-only/no-I/O sequences: Personal
> Assistant -> Cloud Infrastructure -> QA -> Security -> Personal synthesis,
> and the corresponding Systems Operations sequence. The Cloud built-in
> contains synthetic Terraform configuration, Azure architecture, and
> validation evidence; the Systems built-in contains a synthetic service
> snapshot, sanitized log, recovery scenario, and validation evidence. Each
> workflow owns four tasks, three sequential depth-one children, five attempts,
> one active child, 32 events, 16 workflow/audit records, and zero retries.
> Strict assessment, plan, QA, Security, and synthesis results preserve exact
> application-issued provenance. Consequential capabilities remain denied, QA
> cannot approve, Security cannot authorize/remediate, and no approval request
> or execution subject exists. Cloud and Systems are `Initial` only for their
> separate sealed unwired selectors; QA/Security remain advisory and all four
> roles remain tool-ineligible and memory-disabled. No command, credential,
> live access, tool, executor, approval dispatch, provider, IPC/UI, dependency,
> persistence, external runtime, or effect was added. String/credential guards
> are defense-in-depth only. Earlier future statements below remain historical
> checkpoint evidence where this note records current state.

> **Additive current-state note (D-090, 2026-08-13):** D-090 implements one
> separately selected, Rust-only Personal Assistant -> Workflow Automation ->
> Personal proposal lifecycle. Five immutable templates are validated under
> strict shape, dependency, agent, tool-inspection, cycle, limit, framing, and
> output contracts. Complete A-D proposals alone may issue one expiring,
> process-local, take-once token that a fresh orchestrator consumes to select
> the matching existing sealed fixture-only/no-I/O workflow. Template E and all
> tool/approval steps remain non-executable. Workflow Automation is `Initial`
> only for this selector; its generic route is denied, tool profile empty, and
> memory disabled. D-090 narrows the provisional D-082 QA/Security automation
> review route: neither role participates in this proposal lifecycle. Deadline
> enforcement is cooperative and cannot preempt an in-flight synchronous
> runtime call. Focused tests pass 12/12, the public contract passes 18/18,
> strict Clippy passes, all-target Rust passes 393 tests with one intentionally
> ignored probe, and complete verification passes. No general engine,
> scheduling, persistence, executor, approval dispatch, provider, IPC/UI,
> external runtime, I/O, credential, permission, or effect was added.

## 1. Executive summary

The recommended architecture is an application-owned orchestration layer above
the existing single-run runtime boundary:

```text
User
  |
  v
Personal Assistant
  |
  v
AgentOrchestrator
  |
  +-- Research and knowledge: Research; Knowledge & Document
  +-- Software engineering: Coding; QA & Validation; Security & Risk
  +-- Infrastructure and operations: Cloud Infrastructure; Systems Operations;
  |   QA & Validation; Security & Risk
  `-- Automation: Workflow Automation
          |
          v
AgentRuntime
  `-- NativeAgentRuntime (sole/default)
          |
          v
application-owned providers, tools, policy, approvals, audit, and memory
```

The application-owned `AgentRuntime` and sole/default `NativeAgentRuntime`
already exist. They are deliberately small, transport-free, and not connected
to Tauri, React, a provider, or a live model. The closed nine-role definition
catalog and non-authorizing activation metadata now exist. The bounded
`AgentTask`, derived `AgentExecutionContext`, and
`AgentOrchestrator<R: AgentRuntime>` foundations also exist. D-084's exact
policy-profile/attribution, agent-origin approval, delegation-matrix, and
volatile governance-audit foundation now exists; memory, IPC, durable audit,
and live execution do not.

Preserve the existing runtime contract, `InitialGatewayTurn`, gateway
validation, local tool schemas, deterministic policy, exact approval binding,
typed approval audit, deterministic runtime mock, and visible React mock. Add
orchestration above those boundaries instead of rewriting them.

Hermes integration is **Deferred — evaluated transport and containment
requirements not met**. Raw TUI-gateway stdio, managed `hermes serve`
WebSocket, and ACP were each rejected for the exact evaluated Hermes Agent
`0.20.0` / tag `v2026.8.3` / commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb` conditions. Those results do not
claim that every future Hermes release or integration is impossible.

Qualitative implementation complexity is **medium for the nine-definition
catalog and task foundations, high for governed staged workflows, and very high
for live providers, tools, memory, or external runtimes**. The recommended first
usable milestone remains a deterministic, no-I/O Personal Assistant to Research
Agent delegation that returns one bounded attributed result for Personal
Assistant synthesis. That milestone follows the definition/registry and
task/orchestrator foundations; it does not require a live model, provider, tool,
memory store, or UI. The other seven catalog entries remain gated until their
named workflow and governance phases are verified.

## 2. Actual current runtime foundation

### Runtime contract

`src-tauri/src/agent/runtime.rs` defines:

- `AgentRuntime`, with `describe` and `start`;
- `RuntimeRun`, with exact run identity, status, closed event acceptance, and
  idempotent cancellation;
- `RuntimeDescriptor`, `RuntimeCapabilities`, and the closed `RuntimeId`, whose
  only current variant is `Native`;
- `RuntimeTurnRequest`, which carries a run ID, request ID, and bounded selected
  text, but no agent, task, parent, policy, or memory identity;
- closed lifecycle, text, tool-proposal, terminal, and failure event values;
- bounded and redacted IDs, content, errors, and failure metadata; and
- the `AwaitingStart`, `Streaming`, `Completed`, `Failed`, and `Cancelled`
  runtime-run states.

These types describe one bounded runtime turn. They are not a session store,
task scheduler, provider abstraction, orchestration service, policy engine, or
execution authority.

### Native runtime

`src-tauri/src/agent/native_runtime.rs` defines the stateless,
`Default`-constructible `NativeAgentRuntime` and per-run `NativeAgentRun`.
Starting a native run constructs exactly one unchanged `InitialGatewayTurn`.
The native descriptor reports local-boundary availability and health, not live
provider or application readiness. It reports closed text-streaming support and
does not report shared untrusted tool-proposal support.

The concrete native frame lane preserves the existing post-validation policy,
approval, and audit results. The shared runtime event lane accepts only closed
application-owned events. The two lanes cannot be mixed. Cancellation is local,
terminal, and idempotent; it can close turn-owned pending approval state through
the existing typed audited termination path. There is no child process,
provider request, or live stream to interrupt.

### Native turn and governance path

`InitialGatewayTurn` in `src-tauri/src/agent/gateway_request.rs` remains the
verified native composition root. It owns one bounded request, a
`GatewayStreamValidator`, the fixed `InMemoryToolRegistry`, deterministic
policy, the `InMemoryApprovalManager`, and the typed
`InMemoryApprovalAuditAdapter`.

The request and event validators bind protocol version, run/request identity,
sequence, limits, terminal state, and the two fixed local tool schemas:
`get_current_datetime@1` and `create_local_task@1`. Function calls are
revalidated against the local catalog before policy. Policy and approval do not
grant execution authority, and no tool executor exists.

### Mock runtime and visible mock

`MockAgentRuntime` is private test infrastructure in
`src-tauri/tests/agent_runtime_contract.rs`. It deterministically covers
success, unavailability, start failure, event failure, capability
contradiction, invalid state, cancellation, and late-event rejection without a
network, provider, process, clock, filesystem, or Hermes installation.

The shipping React experience is separate. `src/App.tsx` injects
`browserMockRunDriver`; the application state and mock loop under
`src/application/` provide deterministic streaming, stop, retry, approval, and
simulated tool-result behavior. The simulated result records `executed: false`.
It is not backed by `NativeAgentRuntime` and should remain unchanged until a
separately approved UI/runtime increment.

### Provider, lifecycle, and application integration

There is no provider implementation or current `AgentProvider`. D-032 deleted
the obsolete synchronous arbitrary-string scaffold. There is no live model,
runtime selector, coordinator, dispatcher, executor, continuation loop, or
credential consumer.

`src-tauri/src/lib.rs` registers only `get_app_info`. No runtime, agent, task,
delegation, tool, approval, audit, or memory command crosses Tauri IPC. The
current lifecycle is therefore one transport-free runtime turn exercised by
tests, not a live assistant session.

### Current test evidence

- `src-tauri/tests/agent_runtime_contract.rs`: 20 native/runtime/mock contract
  tests.
- `src-tauri/tests/gateway_request_contract.rs`: 10 direct native-turn
  contract tests.
- `src-tauri/tests/policy_input_binding.rs`: trusted policy-input binding.
- `src-tauri/tests/approval_binding.rs`: exact approval ownership.
- `src-tauri/tests/approval_audit_binding.rs`: typed terminal approval audit.
- Frontend application and mock-driver tests cover the deterministic visible
  demonstration.

`src-tauri/tests/agent_definition_registry_contract.rs` exercises the exact
nine-definition public catalog, closed IDs, embedded instruction sources,
activation metadata, deterministic ordering/lookup, and redaction. Private
definition and registry units cover bounds, control characters, mapping
mismatches, duplicate registration, and partial lookup. No current test
exercises a task, orchestrator, delegation, agent-specific policy, or agent
memory because those do not exist.

## 3. Existing agent concepts and disposition

| Existing concept or evidence                                                 | Classification                      | Treatment                                                                             |
| ---------------------------------------------------------------------------- | ----------------------------------- | ------------------------------------------------------------------------------------- |
| D-079 `AgentRuntime` / `NativeAgentRuntime` seam                             | **KEEP AND EXTEND**                 | Keep it as the single-run execution boundary; add orchestration above it.             |
| `RuntimeRun`, closed events, capabilities, errors, and cancellation          | **KEEP**                            | Preserve bounded non-authorizing lifecycle mechanics.                                 |
| `InitialGatewayTurn` and `GatewayStreamValidator`                            | **KEEP**                            | Compose unchanged; do not duplicate or move governance into the orchestrator.         |
| Tool schemas and `ToolRegistry`                                              | **KEEP AND EXTEND**                 | Remain application-owned; later agent assignments require a separate governed design. |
| `PolicyEngine` and `ApprovalManager`                                         | **KEEP**                            | Remain the authorization and exact human-approval boundaries.                         |
| Typed approval audit                                                         | **KEEP AND EXTEND**                 | Add agent/task attribution later through closed records, not arbitrary strings.       |
| Private `MockAgentRuntime` and visible React mock                            | **KEEP**                            | Preserve deterministic no-I/O contract and UI evidence.                               |
| Final-vision specialist/supervisor concepts                                  | **ADAPT**                           | Replace generic prose with the accepted bounded native topology.                      |
| One-turn/single-agent execution assumptions                                  | **ADAPT**                           | Keep one-run execution and place task coordination above it.                          |
| Frontend `user`/`assistant` message roles                                    | **KEEP**                            | Presentation roles are not trusted agent identities.                                  |
| Tasks and Memory placeholder surfaces                                        | **KEEP**                            | Do not reinterpret placeholders as implementation evidence.                           |
| Product memory requirements                                                  | **KEEP AND EXTEND**                 | Future namespaces must sit on a separately approved governed memory lifecycle.        |
| Product memory implementation and vector search                              | **DEFER**                           | No `MemoryStore` exists and no vector database is justified.                          |
| Repository prompts and `.agents/skills`                                      | **KEEP**                            | They are development workflow assets, not product-agent instruction sources.          |
| Deleted provider, audit, memory, and platform scaffolds                      | **REPLACE ONLY WITH JUSTIFICATION** | Do not revive D-030/D-032/D-033/D-034 convenience abstractions.                       |
| Hermes adapter and Hermes tools/memory/subagents                             | **DEFER**                           | Preserve the negative evidence; select nothing automatically.                         |
| Distributed workers, recursive spawning, marketplace, or cloud control plane | **DEFER**                           | Outside current personal-project scope.                                               |

The repository now has a closed product-agent definition, deterministic role
registry, and versioned embedded instruction source. It has no route, task
model, orchestration model, memory namespace, or agent privilege declaration.

## 4. Domain model and remaining target

### Implemented catalog foundation

`AgentId` is a stable, bounded, canonical application-owned identifier. The
catalog contains all nine owner-selected roles: Personal Assistant,
Research Agent, Coding Agent, Cloud Infrastructure Agent, Systems Operations
Agent, Knowledge & Document Agent, QA & Validation Agent, Security & Risk Agent,
and Workflow Automation Agent.

`AgentDefinition` is immutable and privilege-free. It contains only:

- `AgentId`;
- a bounded display name;
- a bounded purpose/description; and
- a closed, versioned application-owned instruction source;
- a closed catalog activation disposition.

Definitions, documented group membership, and activation disposition must not
grant tools, policy outcomes, memory, provider access, or runtime capabilities.
The current instructions are compiled application assets represented by a
closed enum/version. They are not loaded from repository skills, arbitrary
files, URLs, user content, Hermes profiles, or provider payloads.

The current catalog marks only Personal Assistant and Research Agent as
`Initial` for the later deterministic first flow. `Initial` means
catalog-eligible, not
operational: no current consumer can execute either definition. Every other
role is `Deferred` with an explicit closed activation gate naming the later
boundary or workflow required before it can become eligible. All nine may be
looked up/listed non-authoritatively; unknown IDs fail at lookup, and deferred
or unavailable roles fail closed at later operational selection/task creation.
A caller cannot override the disposition.

`AgentRegistry` is a concrete immutable collection containing
exactly those nine definitions, with validated construction, deterministic
ordering, exact lookup, duplicate rejection, a typed unknown-agent result, and
the closed activation disposition on every returned definition. It has no
start/selection method or gate evaluator. A registry trait, mutable plugins, or
dynamic discovery is premature with one built-in source.

### Add for orchestration

`AgentTask`, `AgentTaskStatus`, and `AgentTaskResult` are justified for the
next phase because runtime-run state alone cannot represent queued work,
delegation lineage, synthesis, or a result waiting to be consumed. Task status
should be a separate closed state machine whose relation to `RuntimeRunStatus`
is explicit.

`DelegationRequest` should carry the trusted parent task, target agent, bounded
input, and limits needed for one child. It is a request to the orchestrator,
not permission to execute a tool or spawn a process.

`AgentExecutionContext` should bind agent ID, task ID, optional parent task ID,
runtime ID, policy profile, and memory namespace before a runtime begins.

`AgentOrchestrator` should own assignment, validated delegation, lifecycle,
result collection, cancellation propagation, attribution, and bounded limits.
It should call `AgentRuntime`; it must not become a runtime implementation.

`AgentLimits` should initially allow depth one, one child task total per root,
one active child, bounded task count/content/time, and one terminal result.
Completion or cancellation does not replenish the child budget. Limits are
application-owned and fail closed.

### Defer or omit

`AgentInstance` is not justified until active state exists beyond an
`AgentTask` plus `RuntimeRun`. Avoid a duplicate session object now.

A separate `AgentEvent` should be deferred until an orchestration-level journal
or UI consumer demonstrates a stable need. Reuse closed runtime events inside
the orchestrator without exposing them as task authority.

Runtime capability requirements may later express only execution mechanics
needed by a definition or task, such as bounded text streaming. They may not
grant a tool, policy, memory, provider, or device permission.

Agent policy profiles and memory namespaces are required target concepts but
belong to their separately reviewed phases. Do not place inert privilege fields
on the first definitions before enforcement exists.

## 5. Responsibility boundaries

| Boundary                               | Responsibility                                                                                     | Explicit exclusions                                                                                                                 |
| -------------------------------------- | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `AgentDefinition`                      | Immutable application-owned identity, purpose, versioned instructions, and catalog gate.           | Runtime state, group-based routing, tool grants, policy decisions, memory access, operational claims, or execution.                 |
| `AgentRegistry`                        | Validated deterministic lookup and discovery of known definitions plus their closed catalog state. | Tool registry, gate evaluation, operational selection, task state, route authorization, caller overrides, or mutable configuration. |
| `AgentInstance`                        | Only if later evidence requires active state beyond task/run context.                              | Not part of the first two phases.                                                                                                   |
| `AgentTask`                            | One bounded unit of work, lineage, lifecycle, input, and terminal result.                          | Runtime implementation, approval authority, or persistence by default.                                                              |
| `AgentOrchestrator`                    | Assignment, delegation, limits, task lifecycle, result collection, and cancellation.               | Model execution, provider transport, policy decisions, tools, approvals, or memory storage.                                         |
| `AgentRuntime`                         | Framework-neutral execution contract for one bounded run.                                          | Catalog activation, task routing, authorization, provider selection, or child creation.                                             |
| `NativeAgentRuntime`                   | Execute one bounded task/turn through the application-owned runtime boundary.                      | Agent routing, child creation, policy profiles, memory selection, or tool authority.                                                |
| `PolicyEngine`                         | Deterministic authorization decisions from trusted typed input.                                    | Agent planning or execution.                                                                                                        |
| `ToolRegistry`                         | Application-controlled tool identity, schema, risk, and permission metadata.                       | Agent discovery or dynamic external tools.                                                                                          |
| `ApprovalManager`                      | Exact one-time human approval bound to the canonical subject.                                      | Delegation, runtime selection, or audit authority.                                                                                  |
| Future application-owned `AuditLogger` | Closed redacted action/task history and attribution when separately implemented.                   | Authorization or revival of D-030's deleted arbitrary-string logger.                                                                |
| Future `MemoryStore`                   | User-controlled application memory with explicit namespace and lifecycle.                          | Runtime-owned/Hermes memory or implicit cross-agent sharing.                                                                        |
| Future `PlatformAdapter`               | Narrow OS-specific operations after trusted validation, policy, approval, and execution selection. | Agent authority, generic shell access, or direct model/WebView device access.                                                       |

## 6. Initial delegation architecture

The first usable topology is:

This is a logical task/delegation sequence, not a component-call or authority
graph. The application/orchestrator invokes `AgentRuntime` for each root or
child run; agent definitions never call a runtime directly.

```text
User request
  -> Personal Assistant root task
  -> AgentOrchestrator validates one DelegationRequest
  -> Research Agent child task
  -> one bounded attributed child result
  -> Personal Assistant synthesis
  -> terminal root result
```

Implemented combined Phase 2-3 invariants:

- the root agent is exactly Personal Assistant;
- the only initial delegation edge is Personal Assistant to Research Agent;
- Research Agent cannot delegate, and self, reverse, unknown, or other routes
  fail before child-task creation;
- maximum delegation depth is one;
- each root may create at most one child task total and at most one child may be
  active; completion or cancellation does not replenish the budget;
- only `AgentOrchestrator` creates a child task;
- no runtime, model, agent definition, tool, or WebView creates one directly;
- no free-form or recursive spawning;
- parent and child are independently identifiable and cancellable;
- parent cancellation propagates to the active child exactly once;
- every accepted event, proposal, result, cancellation, and future action is
  attributable to its task and agent;
- child results are bounded data, not instructions or authority; and
- ordinary tests use deterministic runtimes and no external I/O.

The deterministic delegation contract uses the shared test-only mock and fixed
application-owned events. It proves ordering, lineage, attribution,
cancellation, limits, failure fallback, and synthesis inputs without I/O. A
direct Personal response uses one run; the delegated path uses an initial
Personal run, one Research child run, and one fresh Personal synthesis run.
This evidence does not authorize a provider or live model.

### Staged workflow families

The following arrows describe bounded artifact/task sequencing, not direct
agent-to-agent spawning. Every specialist task remains a child created and
validated by `AgentOrchestrator`; specialist agents never spawn another agent.
D-086 implements only the Research and Knowledge family through its exact
fixture-only plan and two-child cap. Each remaining family requires its own plan
to expand the generic one-child-total budget to an explicit finite count while
keeping depth one and concurrency one until a later bounded-parallelism
decision. D-091 now supplies only its exact same-thread bounded-parallel graphs;
it does not grant generic concurrency or specialist spawning.

- **Research and knowledge:** Personal Assistant -> Research Agent -> Knowledge
  & Document Agent -> Personal Assistant synthesis. Current only as D-086's
  sealed deterministic fixture workflow; it is not a generic delegation route
  or live research capability.
- **Engineering quality:** Personal Assistant -> Coding Agent -> QA & Validation
  Agent -> Security & Risk Agent -> Personal Assistant synthesis -> approval
  before any separately authorized consequential change.
- **Infrastructure and operations:** Personal Assistant -> Cloud Infrastructure
  Agent or Systems Operations Agent -> QA & Validation Agent -> Security & Risk
  Agent -> Personal Assistant synthesis -> approval before any separately
  authorized consequential action.
- **Automation:** Personal Assistant -> Workflow Automation Agent -> typed
  workflow proposal -> application validation -> QA & Validation Agent ->
  Security & Risk Agent -> owner approval where required -> orchestrator-managed
  execution through separately approved application boundaries.

QA & Validation and Security & Risk are cross-cutting advisory participants.
QA cannot approve its own privileged action or become `ApprovalManager`;
Security cannot become `PolicyEngine` or authorize/execute remediation.
Workflow Automation may propose a typed workflow but cannot execute, create
tasks, bypass the orchestrator or governance chain, self-modify, or expand
recursively.

## 7. Internal delegation mechanism

### A. Typed runtime control event

Not recommended initially. It would make the one-run `AgentRuntime` aware of
orchestration and require every future adapter to understand child-task
creation. Runtime output is untrusted and cannot create a child directly.

### B. Application-internal `agent.delegate` tool

Not recommended initially. The current native shared runtime does not support
untrusted tool proposals, and the concrete native tool lane already proceeds
through validation, policy, and approval semantics. Treating orchestration as a
tool would conflate internal task routing with device-effect governance and
risk implying model authority to create work.

### C. Explicit application service call

Recommended. `AgentOrchestrator::delegate` should accept a typed
`DelegationRequest`, validate trusted lineage and `AgentLimits`, resolve the
target through `AgentRegistry`, create the child task, and call the selected
runtime. A later model-originated delegation proposal may be translated into a
closed untrusted value, but only this application service can create the task.

The generic route policy is application/orchestrator-owned rather than a field
on `AgentDefinition`: only Personal Assistant may be the root, generic
delegation remains Personal Assistant to Research Agent, and Research Agent may
not delegate. Registry membership or catalog activation never authorizes a
delegation edge. D-086 is a separate sealed application-service sequence, not a
new generic route. Later workflow plans may add exact closed routes and a finite
total-stage cap, but may not relax specialist non-spawning, depth-one lineage,
or application validation by implication.

Delegation is neither shell execution nor an external host tool.

## 8. Governance architecture

Every governed task action is now bound to a trusted `AgentExecutionContext`
containing:

- `AgentId`;
- `AgentTaskId`;
- optional parent `AgentTaskId`;
- `RuntimeId`;
- closed policy profile identity; and
- no memory namespace yet; that identity remains mandatory before any future
  memory/data-bearing privileged action.

The application derives this context from the validated registry and
orchestrator. A model/runtime/WebView may not supply or override it. Tool
proposal validation, policy input, approval subject, audit record, memory
access, cancellation, and terminal result must carry or resolve the same exact
context before they can affect state.

Missing, unknown, duplicate, mismatched, stale, or out-of-lineage identity
fails closed before runtime start or action handling. The system must never
silently default an unknown agent to Personal Assistant, inherit a parent's
privilege, or treat a runtime capability as policy authority.

Agent-specific governance is implemented under D-084 as a separate sealed agent
policy input, closed origin-aware approval lifecycle, exact delegation matrix,
and 32-subject volatile audit. It does not add optional identity to legacy
`PolicyInput`; agent attribution is derived only after the orchestrator checks
the exact live task/run binding. Every execution disposition is
`NotAttempted`. Memory namespace remains mandatory for a later privileged or
data-bearing phase.

The authoritative application boundaries are `AgentOrchestrator`,
`AgentRuntime`, `NativeAgentRuntime`, `AgentRegistry`, `ToolRegistry`,
`PolicyEngine`, `ApprovalManager`, `AuditLogger`, `MemoryStore`, and
`PlatformAdapter`. The runtime/native runtime, orchestrator, agent registry,
tool registry, policy engine, approval manager, narrow legacy approval audit,
and closed per-agent governance audit now exist. A generic audit logger, memory
store, and platform adapter remain absent; D-084 does not revive the deleted
arbitrary-string logger.
No agent substitutes for one: Security & Risk is not `PolicyEngine`, QA &
Validation is not `ApprovalManager`, and Workflow Automation is not
`AgentOrchestrator`.

## 9. Memory architecture

No product `MemoryStore` or memory namespace exists. The initial definition and
orchestration phases require no memory implementation.

A future minimal namespace model should distinguish:

- shared user/project memory, explicitly visible to approved agents;
- agent-private memory, scoped to one `AgentId`;
- task/session temporary memory, scoped to one `AgentTaskId` and deleted or
  retained through a defined lifecycle; and
- proposed shared memory, which is an inert candidate until reviewed and
  promoted through application policy and user controls.

Cross-agent sharing must be explicit, attributable, editable, exportable, and
deletable. A child result is not automatically durable memory. Runtime-owned or
Hermes memory cannot substitute for the application store. No vector database,
embedding service, semantic index, or retrieval dependency is justified by the
current requirements.

## 10. Current catalog, grouping, and privilege posture

The first registry phase defined all nine roles with only Personal Assistant
and Research `Initial`. D-085 later made Knowledge `Initial` for its exact
routes, and D-087 makes Coding, QA, and Security `Initial` for its sealed
fixture-only proposal workflow. D-088 now makes Cloud and Systems `Initial`
only for their separate fixture/no-I/O selectors and reuses QA/Security as
advisory stages. D-090 makes Workflow Automation `Initial` only for its sealed
proposal selector. Every definition is application-owned,
immutable, privilege-free, and non-operational until a separate consumer
exists. Catalog listing may show all nine and their exact state without
presenting eligibility as a tool or live capability.

| Functional group              | Catalog members                                                                                    |
| ----------------------------- | -------------------------------------------------------------------------------------------------- |
| Core orchestration            | Personal Assistant                                                                                 |
| Research and knowledge        | Research Agent; Knowledge & Document Agent                                                         |
| Software engineering          | Coding Agent; QA & Validation Agent; Security & Risk Agent                                         |
| Infrastructure and operations | Cloud Infrastructure Agent; Systems Operations Agent; QA & Validation Agent; Security & Risk Agent |
| Automation                    | Workflow Automation Agent                                                                          |

QA & Validation and Security & Risk deliberately appear in multiple groups.
Catalog listing remains deterministic; functional grouping is
documentation-only and never authorizes a route, tool, permission, or action.

| Agent                      | Catalog disposition / activation prerequisite            |
| -------------------------- | -------------------------------------------------------- |
| Personal Assistant         | `Initial` for verified bounded workflows                 |
| Research Agent             | `Initial`; live retrieval remains gated                  |
| Knowledge & Document Agent | `Initial` only for D-085/D-086 sealed routes             |
| Coding Agent               | `Initial` only for D-087 fixture/proposal workflow       |
| QA & Validation Agent      | `Initial` only for D-087 fixture/proposal workflow       |
| Security & Risk Agent      | `Initial` only for D-087 fixture/proposal workflow       |
| Cloud Infrastructure Agent | `Initial` only for D-088 sealed Cloud fixture workflow   |
| Systems Operations Agent   | `Initial` only for D-088 sealed Systems fixture workflow |
| Workflow Automation Agent  | `Initial` only for D-090's sealed proposal selector      |

`Initial` is catalog eligibility for exact verified unwired workflows, not a
claim that a role is operational today. Every `Deferred` state is closed and
application-controlled; no caller, model, runtime, group, or definition may
promote it.

### Core orchestration

**Personal Assistant — `Initial`:** primary user-facing role for task
classification, controlled delegation requests, progress communication, result
synthesis, and approval explanation. It must not receive unrestricted
privileged tools and has no direct device, shell, filesystem, network, provider,
memory, policy, approval, execution, or audit authority. It may explain an
approval presentation but cannot approve it, change its canonical subject, or
interpret user-facing text as approval.

### Research and knowledge

**Research Agent — `Initial`:** produces one bounded attributed result
from supplied content in the first usable milestone. Later governed read-only
tools may support external/internal source gathering, factual comparison, and
evidence-backed reports. Its name grants no browser, search, network, file,
memory, or tool access. Any later source content remains untrusted input.

**Knowledge & Document Agent — gated on knowledge/document and memory
boundaries:** may read only explicitly approved files or roots for
summarization, comparison, knowledge extraction, organization, and document
preparation. It cannot crawl the filesystem, cross approved roots, or silently
write permanent shared memory.

### Software engineering

**Coding Agent — gated on the engineering workflow:** may inspect a repository,
explain code, plan implementation, propose patches, and perform separately
approved code changes or test execution. It cannot autonomously commit, push,
install dependencies, run destructive commands, or acquire shell/Git authority
from its role.

**QA & Validation Agent — gated on engineering-quality governance:** may plan
tests, validate outputs/configuration, assess acceptance criteria and
regressions, and use separately approved safe validation tools. It cannot
approve its own privileged action and is not `ApprovalManager`.

**Security & Risk Agent — gated on engineering/security governance:** provides
advisory threat modeling, security/secrets/policy review, risk analysis, and
change-risk assessment. It is not `PolicyEngine` and cannot authorize or
execute remediation or supply trusted risk/permission metadata. Secrets review
uses only redacted or sanitized evidence and never exposes credential values.

### Infrastructure and operations

**Cloud Infrastructure Agent — `Initial` only for D-088's sealed fixture
workflow:** provides bounded assessment and inert change planning over the
application's synthetic Terraform/Azure fixture. It cannot inspect live
inventory, run Terraform/cloud commands, apply, modify, delete, change IAM, or
use credentials.

**Systems Operations Agent — `Initial` only for D-088's sealed fixture
workflow:** provides bounded assessment and inert recovery planning over the
application's synthetic service snapshot, sanitized log, and recovery fixture.
It cannot inspect a live host, run diagnostics or platform commands, restart,
shut down, delete, change configuration/accounts, or execute a privileged
shell.

QA & Validation and Security & Risk are cross-cutting roles in software,
infrastructure, operations, document, and automation workflows. Cross-cutting
participation does not change their advisory, non-authorizing posture.

D-089 decomposes only the private D-088 implementation: the orchestrator facade
retains task/run/event/cancellation authority, a private child owns the exact
Cloud/Systems lifecycle, and private catalog/framing/validation children own
the unchanged fixture and parser bodies. Public contracts, bounds, ordering,
profiles, and `NotAttempted` execution semantics are unchanged. This clears the
specific D-088 next-increment review-coupling blocker without selecting a
general workflow engine.

### Automation

**Workflow Automation Agent — gated on typed workflows and governance:** may
propose structured workflows and analyze dependencies/sequencing. It cannot
execute arbitrary commands, create tasks, bypass `AgentOrchestrator`,
`ToolRegistry`, `PolicyEngine`, `ApprovalManager`, or `AuditLogger`, or create a
self-modifying or recursively expanding workflow.

D-090 implements the narrowed first phase: one Personal -> Workflow Automation
-> Personal proposal selector, five
immutable templates, and an expiring single-use manual token for A-D that maps
only to existing fixture-only/no-I/O selectors in a fresh orchestrator.
Template E and every tool/approval step remain non-executable. D-090 supersedes
the provisional D-082 QA/Security proposal-review sequence for this phase;
those roles are not invoked and no review claim is made. The original monotonic
deadline propagates through any manual dispatch and is cooperatively checked at
central destination lifecycle ingress; it is not a hard-preemption claim.

D-091 implements one separate fixture-only/no-I/O `BoundedParallel` selector.
Research and Knowledge may run independently under `ContinuePartial`; Coding
and Security may run independently before dependent QA under
`CancelDependentOnly`; and Cloud and Systems may run independently before
dependent Security under specialist-lane `FailFast`. Personal synthesis uses
stable catalog ordinals and must disclose exact source agents, statuses,
validated finding IDs, failures, and unresolved issues.

The selector retains several `RuntimeRun` values and multiplexes exact task/run
events on one application thread. Limits are depth one, default active two,
hard active/total child three, four tasks, five attempts, zero retries, eight
events per run, 32 applicable records, and cooperative 120-second root/
60-second child leases. This proves neither provider/CPU concurrency nor hard
preemption, provider-session isolation, app-global coordination, scheduling,
or distributed execution.

## 11. Personal-project scope

The architecture is for one owner and one local desktop application. Defer:

- distributed agents or remote workers;
- multi-tenancy;
- a marketplace or public plugin ecosystem;
- billing;
- enterprise IAM;
- a hosted cloud control plane;
- unlimited or free-form recursion;
- autonomous destructive operations; and
- production-scale queues, event buses, worker fleets, or service discovery.

These omissions are deliberate scope controls, not deficiencies in the first
milestone.

## 12. Future publication boundaries worth preserving

Preserve framework-neutral IDs and closed domain types, explicit application
ownership, deterministic registry ordering, the runtime port, adapter-local
external types, strict Tauri IPC, typed errors, redaction, bounded task lineage,
and replaceable persistence interfaces only when a real store is introduced.

Also preserve the ability to add another runtime behind `AgentRuntime` after a
separate accepted decision and evidence. Do not build SaaS tenancy, public
plugin loading, commercial infrastructure, remote execution, or distributed
coordination merely to keep that option open.

The near-term sequence is recorded in the authoritative root
[`ROADMAP.md`](../../ROADMAP.md) and expanded in the subordinate
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md).
The
[`Agent definition and registry`](../plans/2026-08-11-agent-definition-registry.md)
plan is implemented and verified locally. That sentence and the original
future-phase language record the assessment checkpoint. D-083 through D-088 now
add the verified task/orchestrator, non-executing governance, volatile
memory/approved-document, sealed fixture-only Research/Knowledge, sealed
fixture-only proposal Engineering, and two sealed fixture-only/no-I/O Cloud/
Systems foundations described in the additive notes above. Automation, live
repository/infrastructure effects, persistence, provider/runtime wiring,
IPC/UI, and every other later phase remain separately gated.
