# Native agent governance threat model

Status: Accepted implementation constraints under D-084
Last updated: 2026-08-12
Scope: non-executing per-agent governance foundation only

## Security objective

Prove that one application-owned service can bind a synthetic untrusted tool
proposal or explicit delegation request to the exact live agent task, runtime
run, policy profile, deterministic policy result, exact approval lifecycle,
and bounded audit evidence without executing anything or granting authority to
an agent, runtime, WebView, role name, advisory output, or approval response.

This is not a shipping tool lane. `AgentRuntime` and `NativeAgentRuntime` do not
gain tool capability; runtime `ToolProposal` remains rejected by
`AgentOrchestrator`; Tauri and React remain disconnected; no executor exists.

## Assets and authorities

| Asset or authority                   | Owner                          | D-084 invariant                                                      |
| ------------------------------------ | ------------------------------ | -------------------------------------------------------------------- |
| Agent definition/profile association | `AgentDefinition`              | Exact immutable one-to-one built-in mapping; no fallback             |
| Live task/run attribution            | `AgentOrchestrator`            | Reconstructed from current task and active run before every mutation |
| Profile rules                        | `AgentPolicyProfileRegistry`   | Nine closed profiles; Personal alone recognizes two current schemas  |
| Tool schemas                         | `ToolRegistry`                 | Existing two closed local schemas; registration does not execute     |
| Policy result                        | `DeterministicPolicyEngine`    | One engine; distinct sealed agent input; fail-closed profile check   |
| Approval                             | `ApprovalManager`              | One exact pending subject; source/manager/presentation/TTL binding   |
| Governance sequencing                | `AgentGovernanceService`       | Composes authorities but cannot override them or execute             |
| Task/delegation lifecycle            | `AgentOrchestrator`            | Sole child creator; exact Personal-to-Research matrix                |
| Governance evidence                  | `InMemoryAgentGovernanceAudit` | Thirty-two preallocated lifecycle records; no content or persistence |
| Execution                            | None                           | Every disposition is `NotAttempted`                                  |

## Trust boundaries and dataflow

```text
Untrusted AgentToolProposal
  -> AgentOrchestrator live-context equality check
  -> AgentGovernanceService audit reservation
  -> ToolRegistry schema lookup and local argument validation
  -> AgentPolicyProfileRegistry exact eligibility check
  -> DeterministicPolicyEngine
  -> Allow / Deny / RequireApproval
  -> ApprovalManager only when required
  -> InMemoryAgentGovernanceAudit lifecycle update
  -> typed result with ExecutionDisposition::NotAttempted

Untrusted DelegationProposal
  -> AgentOrchestrator exact live attribution + closed target derivation
  -> AgentGovernanceService delegation reservation
  -> source/registry/activation/matrix/depth/budget checks
  -> AgentOrchestrator runtime cancellation and child creation
  -> infallible delegation record update
```

The proposal contains no trusted agent, task, root, parent, runtime, profile,
depth, run, or request identity. The orchestrator reconstructs
`AgentAttribution` from live state. The tool call ID is untrusted but bounded;
once paired with exact attribution, it becomes a consumed replay subject for
all validation and policy outcomes.

## Threats and required controls

### Forged or stale attribution

Threat: a model, test caller, old task, other workflow, terminal task, or stale
runtime run reuses a context to act under another agent or profile.

Controls:

- trusted constructors remain agent-module private;
- tasks consume one resolved definition identity containing agent and profile;
- every service entry compares task, root, parent, runtime, profile, depth,
  run, and request to the currently owned task/run;
- mismatch fails before audit attribution, schema lookup, policy, approval, or
  task mutation;
- no unknown identity defaults to Personal Assistant.

### Policy-profile bypass

Threat: an agent-origin request enters the legacy gateway `PolicyInput` and is
classified only by schema risk, bypassing an empty specialist allowlist.

Controls:

- legacy `SchemaValidatedFunctionCall`, `PolicyInput`, and `evaluate` remain
  legacy-only;
- agent validation emits a distinct sealed non-clonable request;
- there is no conversion to legacy `PolicyInput`;
- the same engine exposes a separate profile-aware agent method and checks the
  profile before shared risk classification;
- approval accepts agent requests only through the sealed agent decision.

### Tool, parameter, or name injection

Threat: an untrusted name, version, arguments JSON, or affected parameter is
unknown, malformed, oversized, logged, or mistaken for authority.

Controls:

- proposal fields are bounded and Debug-redacted, including name;
- `ToolRegistry` lookup and exact local schema validation precede policy;
- unknown tools are represented only by a closed error/audit category;
- arguments and affected data never enter audit, errors, Debug, or logs;
- no outcome reaches execution.

### Replay and duplicate decisions

Threat: reusing a call, denial, approval, or delegation produces another
policy/approval/control effect.

Controls:

- `Tool { attribution, call_id }` is consumed for validation reject, Allow,
  Deny, and approval alike;
- changing tool name/version/arguments cannot reuse the call ID;
- `Delegation { attribution, target }` is consumed before downstream mutation;
- every post-attribution self/specialist/deferred/route/depth/budget denial is
  committed; stale/foreign pre-attribution failures are not falsely audited;
- approval retains its existing consumed-subject and presentation/source
  replay protections;
- all duplicate errors are typed and content-free.

### Approval/task lifecycle race

Threat: a task finishes, delegates, changes runtime run, or is cancelled while
its approval remains capable of resolving.

Controls:

- one `AgentGovernanceService` is composed inside the one-root orchestrator;
- a pending approval blocks runtime events and delegation for that task;
- presentation, source resolution, expiry, and cancellation revalidate the
  exact live attribution;
- task cancellation first consumes and audits the pending approval, then
  terminalizes runtime/task state;
- root cancellation performs the same reconciliation for an active child's
  pending approval before child-first cancellation and only then handles root;
- if approval-manager cancellation fails without mutation, task and approval
  remain pending; if approval cancellation succeeds but runtime cancellation
  later fails, the task/run remains live with approval terminally cancelled and
  no executable authority;
- no background worker or independent resolver exists.

### Audit loss or partial success

Threat: policy, approval, or child creation mutates before audit capacity or a
clock failure, leaving an unaudited effect.

Controls:

- capacity is fixed at 32 subjects;
- one record slot is reserved before policy, approval, runtime cancellation, or
  child creation;
- the injected logical clock is infallible and performs no I/O;
- later state changes update the same preallocated slot infallibly;
- a reservation may abort only before downstream mutation;
- terminal safety cancellation never needs new audit capacity.

### Delegation/tool conflation

Threat: an `agent.delegate` tool or runtime event bypasses orchestrator route,
depth, or budget checks.

Controls:

- delegation remains outside `ToolRegistry`;
- only `AgentOrchestrator::request_delegation` creates a child;
- `DelegationMatrix` allows only Personal Assistant to Research;
- Research and all specialists cannot delegate;
- runtime tool proposals remain rejected;
- definition/profile/group membership grants no route.

### Advisory-role authority escalation

Threat: QA output becomes approval, Security output becomes policy, or Workflow
output becomes execution/orchestration.

Controls:

- all eight non-Personal profiles have empty current tool allowlists;
- QA and Security remain advisory and cannot approve or authorize;
- Workflow remains proposal-only and cannot create tasks, coordinate the
  orchestrator, or execute;
- no advisory DTO is introduced without a current consumer.

### Content, secret, or reasoning disclosure

Threat: prompts, objectives, arguments, affected data, results, credentials, or
hidden reasoning enter audit, Debug, errors, tests, or the WebView.

Controls:

- governance records contain only typed attribution, closed action/target,
  outcomes, logical ticks, and closed errors;
- content-bearing types redact Debug;
- errors contain no caller strings;
- no persistence, IPC, UI, provider, credential, or logger is added;
- sentinel tests cover proposal, policy, approval, audit, and service Debug.

## Residual risk and later gates

This increment does not prove a live model, provider, tool executor, device
effect, durable audit, memory store, or UI. A future action remains blocked on
an accepted restricted executor and platform boundary, durable audit and
failure semantics, exact memory namespace where data is involved, relevant
permission evidence, Tauri exposure review, and separate owner approval.

Hermes remains Deferred/Blocked. Its rejected transports do not participate in
this contract.
