# Native multi-agent roadmap

Status: Subordinate staged roadmap; no implementation authority
Authority: [`ROADMAP.md`](../../ROADMAP.md) remains the authoritative milestone
roadmap
Decision: D-082, D-083, D-084, D-085, D-086, D-087, D-088, D-089, D-090,
D-091, the approved Command Center proposal/ExecPlan, and
[`ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`](../adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md)
Last updated: 2026-08-25

## Purpose

This document expands the native multi-agent portion of the root roadmap into a
catalog, activation sequence, workflow map, and separately gated plan set. It
does not change root-roadmap status or authorize implementation. The closed
nine-definition catalog, deterministic registry, bounded orchestrator, and
non-executing governance foundation now exist, remain unwired, and grant no
device or provider authority. D-085's workflow-local bounded volatile memory
store and approved-document reader are verified complete with advisories and
published at `5e53f55`. D-086's exact fixture-based sequential
Research/Knowledge workflow is verified complete with advisories and published
at `3efd2c1`. D-087 implements one fixture-only, proposal-only engineering-
quality workflow, verified complete with advisories and published at `a5d7ba1`,
without adding a tool or effect. D-088 implements two separate fixture-only
Cloud and Systems workflows and is verified complete with advisories. D-089's
private decomposition is published at `140f05b`. D-090's strict typed proposal
and A-D take-once manual sealed-dispatch boundary is verified and published.
D-091 implements one sealed fixture-only/no-I/O same-thread event-multiplexed
parallel selector and is verified complete with advisories under a complete,
valid gate. The frontend-only deterministic Command Center fixture projection
has passed its approved M5 real-browser/Tauri validation matrix; live typed IPC
remains separately Blocked. The end-to-end demonstration increment has
completed implementation and validation on branch
`codex/native-multi-agent-end-to-end-demonstrations` from baseline `527f0f4`.
Its canonical deterministic evidence covers all twelve demonstrations under
the owner-approved acceptance scope. Demo 7 validates checkpoint denial and
safe manual A-D dispatch as separate branches. D-090's zero-executable-tool-
step boundary and absence of an approval-to-dispatch bridge remain explicit
advisories. No executor,
durable memory, multi-agent Tauri consumer, authoritative/live multi-agent UI
state, or agent control exists.

`AgentOrchestrator`, `AgentRuntime`, `NativeAgentRuntime`, `AgentRegistry`,
`ToolRegistry`, `PolicyEngine`, `ApprovalManager`, `AuditLogger`, `MemoryStore`,
and `PlatformAdapter` remain application-owned. Agents may return bounded
recommendations or typed requests. Application code owns lifecycle,
authorization, approval, execution, audit, and memory decisions.

## Catalog and functional groups

The catalog contains nine application-owned roles. Cross-cutting membership
does not create duplicate definitions or grant authority.

- **Core orchestration:** Personal Assistant.
- **Research and knowledge:** Research Agent; Knowledge & Document Agent.
- **Software engineering:** Coding Agent; QA & Validation Agent; Security &
  Risk Agent.
- **Infrastructure and operations:** Cloud Infrastructure Agent; Systems
  Operations Agent; QA & Validation Agent; Security & Risk Agent.
- **Automation:** Workflow Automation Agent.

QA & Validation and Security & Risk may participate in software,
infrastructure, operations, document, and automation workflows. QA & Validation
is not `ApprovalManager`. Security & Risk is not `PolicyEngine`. Workflow
Automation is not `AgentOrchestrator`.

## Planned staged activation

Phase 1 registers all nine definitions, but catalog presence is
non-authorizing. `Initial` below means eligible only for a future
deterministic orchestration phase; it does not mean operational, wired, or able
to use a live model or tool.

| Agent                      | Phase 1 catalog state | Earliest activation gate                                                                                      |
| -------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------- |
| Personal Assistant         | `Initial`             | Definition/registry verification, then the separately verified task/orchestration foundation                  |
| Research Agent             | `Initial`             | Initial bounded Personal-to-Research workflow; governed retrieval remains separately gated                    |
| Knowledge & Document Agent | `Initial`             | D-085's direct approved-document route; D-086's verified sealed fixture workflow adds no generic delegation   |
| Coding Agent               | `Initial`             | D-087's sealed fixture-only proposal workflow; live repository/change/test controls remain separately gated   |
| QA & Validation Agent      | `Initial`             | D-087's sealed fixture-only validation contract; it never becomes approval authority                          |
| Security & Risk Agent      | `Initial`             | D-087's sealed fixture-only advisory contract; it never becomes policy or remediation authority               |
| Cloud Infrastructure Agent | `Initial`             | D-088 sealed Cloud fixture only; live inventory/change remains gated                                          |
| Systems Operations Agent   | `Initial`             | D-088 sealed Systems fixture only; live diagnostics/platform control remains gated                            |
| Workflow Automation Agent  | `Initial`             | D-090 sealed proposal selector only; A-D take-once manual sealed dispatch; E/tool/approval execution deferred |

Catalog discovery lists all nine definitions and their exact state. Deferred
agents fail closed only at later operational selection/task-creation boundaries;
this roadmap defines no evaluator. No role name, instruction source, catalog
membership, or runtime capability grants a tool, credential, filesystem,
shell, cloud, memory, provider, or device right.

## Root phase mapping

The ten root-roadmap phases remain authoritative. The entries below explain the
native catalog's staged work inside those phases.

| Root phase | Staged native outcome                                                                                                                                                                | Status relationship                                                          |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------- |
| 1          | Register all nine privilege-free definitions; mark only Personal Assistant and Research Agent `Initial`                                                                              | Verified and published at `f42a6c7`; catalog remains inert and unwired       |
| 2-3        | Add task lifecycle, trusted context, limits, explicit delegation, result collection, cancellation, and the deterministic Personal-to-Research-to-Personal proof above `AgentRuntime` | Verified complete under D-083; foundation remains Rust-only and unwired      |
| 4          | Bind D-084's non-executing per-agent profile/policy/approval/audit foundation; establish approved file/root and document-processing boundaries only in a separate later increment    | Governance verified and published at `2687294`                               |
| 5          | Add D-085's selected bounded volatile namespaces and approved `.txt`/`.md` Knowledge boundary; keep durable memory separate                                                          | Verified complete with advisories; published at `5e53f55`                    |
| 5A         | Add D-086's exact fixture-only Personal-to-Research-to-Knowledge-to-Personal sequential workflow without widening generic delegation                                                 | Verified complete with advisories and published at `3efd2c1`                 |
| 6          | Add D-091's exact fixture-only same-thread event-multiplexed selector with deterministic limits, cancellation, failure policy, ordering, and cleanup                                 | Verified complete with advisories; marker complete and valid                 |
| 7          | Stage engineering-quality, infrastructure/operations, and automation workflows with each specialist separately enabled                                                               | Completed: D-087/D-088/D-090 verified; D-089 published                       |
| 8          | Add typed catalog, task, progress, attribution, cancellation, and approval-explanation UI through narrow Tauri IPC                                                                   | Deterministic frontend fixture prototype validated; live IPC remains Blocked |
| 9          | Demonstrate only bounded synthetic or separately approved workflows without capability overclaiming                                                                                  | Verified complete with advisories; marker complete and valid                 |
| 10         | Review architecture, security, privacy, cancellation, audit, portability, and rollback across completed phases                                                                       | Blocked until sufficient implementation evidence exists                      |

## Delegation and sequencing invariants

- Only `AgentOrchestrator` creates child tasks.
- Personal Assistant may request delegation; specialists may not spawn agents.
- Workflow Automation may propose a typed workflow but may not execute or spawn
  it.
- In the initial Personal-to-Research phase, delegation depth, total-child
  budget, and active child concurrency are one. Later sequential workflow plans
  must set their own exact finite total-child caps; the bounded-parallelism
  phase increases only D-091's exact sealed concurrency bound, not depth or
  generic spawning authority.
- Every target is validated against the registry, catalog activation state,
  route policy, limits, and exact execution context before task creation.
- Every task is cancellable and every task/action is attributable.
- Recursive, self-modifying, or free-form agent-to-agent spawning is
  prohibited.

Workflow arrows below describe orchestrator-controlled logical sequencing.
They do not mean that one specialist directly creates the next specialist. To
preserve depth one, the orchestrator owns each accepted child task under the
Personal Assistant root and permits no specialist-originated child.

## Planned workflow families

### Research and knowledge

```text
Personal Assistant
  -> Research Agent
  -> Knowledge & Document Agent
  -> Personal Assistant synthesis
```

The implemented generic-delegation milestone stops after the Research Agent
result. D-085 separately adds a direct Personal Assistant-to-Knowledge task over
one selected approved document. D-086 implements only one sealed,
fixture-based orchestrator sequence in which both specialists remain sibling
children of the Personal root. The deterministic contracts complete that
connection without permitting direct Research-to-Knowledge delegation, and
neither role gains file selection, enumeration, persistence, or tool
authority. External research requires separately governed read-only tools and
remains Blocked.

### Engineering quality

```text
Personal Assistant
  -> Coding Agent
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> Personal Assistant synthesis
  -> approval before consequential changes
```

Coding cannot autonomously commit, push, install dependencies, or run
destructive commands. QA cannot approve its own privileged action. Security &
Risk cannot authorize or execute remediation.

### Infrastructure and systems operations

```text
Personal Assistant
  -> Cloud Infrastructure Agent or Systems Operations Agent
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> Personal Assistant synthesis
  -> approval before consequential actions
```

Initial specialist work is planning or separately approved read-only evidence.
No agent gains credentials, privileged shell, apply, modification, deletion,
IAM, restart, shutdown, account, or configuration authority by role.

D-088 selects two separate application-only fixture selectors, not this `or`
branch as model-selected routing. Both remain sequential depth-one sibling
workflows with one active child and no tool or effect. The Cloud built-in
accepts only synthetic Terraform configuration, Azure architecture, and
validation evidence; the Systems built-in accepts only a synthetic service
snapshot, sanitized log excerpt, recovery scenario, and validation evidence.
Terraform and platform commands, live inventory, diagnostics,
credentials, and approval-to-execution remain outside the Ready plan.

### Workflow automation

```text
Personal Assistant
  -> Workflow Automation Agent
  -> typed workflow proposal
  -> application validation
  -> Personal Assistant synthesis

Complete A-D proposal
  -> expiring take-once application token
  -> fresh AgentOrchestrator
  -> matching existing sealed fixture-only/no-I/O selector
```

Workflow Automation cannot bypass `AgentOrchestrator`, `ToolRegistry`,
`PolicyEngine`, `ApprovalManager`, or `AuditLogger` and cannot create recursively
expanding workflows. D-090 narrows D-082's provisional QA/Security proposal-
review topology: neither role is invoked in this phase. Template E and every
tool or approval step remain non-executable. Any future consequential effect
would require its own separately approved registered-tool, policy, exact-
approval, restricted-execution, and audit boundary.

## Separately gated plans

The AgentDefinition/AgentRegistry, combined orchestration, and D-084 bounded
non-executing governance foundations are implemented, verified, and published.
D-085 separately selects one combined volatile-memory and narrow
approved-document Knowledge increment; it is verified complete with advisories
and published at `5e53f55`. D-086's exact fixture-only Research/Knowledge
workflow is verified complete with advisories and published at `3efd2c1`.
D-087 and the engineering-quality plan implement only the published fixture-
only, proposal-only Engineering workflow. D-088 implements two separate no-I/O
fixture workflows and is verified complete with advisories. D-089 selects a
behavior-preserving private D-088 lifecycle/catalog/framing/validation
decomposition, now published at `140f05b`. D-090 selects only a sealed Personal
-> Workflow Automation -> Personal proposal route, five immutable templates,
and A-D one-time manual dispatch to the existing no-I/O selectors; it is
verified complete and published. D-090 narrows
D-082's provisional QA/Security
proposal-review topology for this phase; those reviews remain deferred.
D-091 implements only one sealed fixture-only/no-I/O `BoundedParallel` selector with
same-thread event multiplexing, exact default active two/hard and total three,
depth one, zero retries, cooperative deadlines, ordinal results, and closed
failure policies. Implementation verification, independent reviews, final
post-documentation checks, and deterministic marker validation pass.

The end-to-end demonstration plan has completed implementation and validation.
Final focused checks, 447/447 canonical acceptance, and full verification pass
on macOS 26.6 build 25G72 arm64. For Demo 7, the owner accepts checkpoint denial
and safe manual A-D dispatch as separate demonstrations. D-090's
checkpoint-containing proposal branch still cannot connect to its separate
manual A-D dispatch while executable tool steps remain exactly zero; no bridge
is added, and the absent combined chain remains an advisory. Demo 10 records
a task-bound typed rejection with execution `NotAttempted`; the root remains
`Running` and receives no runtime text. Native remains sole/default and unwired,
Hermes remains Deferred/Blocked. Fresh closeout passes, and the gate marker is
complete and fingerprint-valid. No next plan is owner-selected or Ready.

Durable memory, live retrieval, every broader parallel/general-engine path, every live
infrastructure/operations or repository tool/effect, executable automation,
scheduling, template E dispatch, and every other later phase remain Blocked.

| Area                                   | Plan                                                                                                                                      |
| -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Orchestration foundation               | [`2026-08-11-agent-orchestration-task-lifecycle.md`](../plans/2026-08-11-agent-orchestration-task-lifecycle.md)                           |
| Per-agent governance                   | [`2026-08-11-agent-governance.md`](../plans/2026-08-11-agent-governance.md)                                                               |
| Volatile memory + approved documents   | [`2026-08-12-agent-memory-approved-documents.md`](../plans/2026-08-12-agent-memory-approved-documents.md)                                 |
| Knowledge and document boundaries      | [`2026-08-11-knowledge-document-boundaries.md`](../plans/2026-08-11-knowledge-document-boundaries.md)                                     |
| Agent-specific memory                  | [`2026-08-11-agent-memory.md`](../plans/2026-08-11-agent-memory.md)                                                                       |
| Research and knowledge workflow        | [`2026-08-11-research-knowledge-workflow.md`](../plans/2026-08-11-research-knowledge-workflow.md)                                         |
| Engineering quality workflow           | [`2026-08-11-engineering-quality-workflow.md`](../plans/2026-08-11-engineering-quality-workflow.md)                                       |
| Infrastructure and operations workflow | [`2026-08-11-infrastructure-systems-operations-workflow.md`](../plans/2026-08-11-infrastructure-systems-operations-workflow.md)           |
| Workflow automation                    | [`2026-08-11-workflow-automation.md`](../plans/2026-08-11-workflow-automation.md)                                                         |
| Bounded parallelism                    | [`2026-08-11-bounded-agent-parallelism.md`](../plans/2026-08-11-bounded-agent-parallelism.md)                                             |
| Desktop UI                             | [`2026-08-11-multi-agent-ui.md`](../plans/2026-08-11-multi-agent-ui.md)                                                                   |
| End-to-end demonstrations              | [`2026-08-11-multi-agent-end-to-end-demonstrations.md`](../plans/2026-08-11-multi-agent-end-to-end-demonstrations.md)                     |
| Final security and architecture review | [`2026-08-11-native-multi-agent-security-architecture-review.md`](../plans/2026-08-11-native-multi-agent-security-architecture-review.md) |

## Non-goals and authority

This roadmap does not authorize production code, dependencies, providers,
tools, additional memory, persistence, processes, networking, Tauri IPC, UI, credentials,
platform permissions, agent activation, workflow execution, commit, or
publication. It does not select Hermes or another external framework. Native
remains sole/default, and Hermes remains Deferred/Blocked.

Distributed workers, remote agents, multi-tenancy, a marketplace, a public
plugin ecosystem, billing, enterprise IAM, a cloud control plane, unlimited
recursion, and autonomous destructive operations remain outside current scope.
