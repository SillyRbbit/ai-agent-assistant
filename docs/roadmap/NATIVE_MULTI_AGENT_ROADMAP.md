# Native multi-agent roadmap

Status: Subordinate staged roadmap; no implementation authority
Authority: [`ROADMAP.md`](../../ROADMAP.md) remains the authoritative milestone
roadmap
Decision: D-082 and
[`ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`](../adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md)
Last updated: 2026-08-12

## Purpose

This document expands the native multi-agent portion of the root roadmap into a
catalog, activation sequence, workflow map, and separately gated plan set. It
does not change root-roadmap status, authorize implementation, or claim that an
orchestrator, provider, tool, memory store, Tauri consumer, or multi-agent UI
exists. The closed nine-definition catalog and deterministic registry now
exist, remain inert, and grant no operational authority.

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

| Agent                      | Phase 1 catalog state | Earliest activation gate                                                                                    |
| -------------------------- | --------------------- | ----------------------------------------------------------------------------------------------------------- |
| Personal Assistant         | `Initial`             | Definition/registry verification, then the separately verified task/orchestration foundation                |
| Research Agent             | `Initial`             | Initial bounded Personal-to-Research workflow; governed retrieval remains separately gated                  |
| Knowledge & Document Agent | `Deferred`            | Approved document roots, knowledge boundaries, memory lifecycle, and applicable governance                  |
| Coding Agent               | `Deferred`            | Engineering workflow plus approved repository/change/test controls                                          |
| QA & Validation Agent      | `Deferred`            | Engineering-quality validation boundary; it never becomes approval authority                                |
| Security & Risk Agent      | `Deferred`            | Engineering/security review boundary; it remains advisory and never becomes policy or remediation authority |
| Cloud Infrastructure Agent | `Deferred`            | Infrastructure workflow plus separately approved read-only inventory and change-governance controls         |
| Systems Operations Agent   | `Deferred`            | Operations workflow plus separately approved read-only diagnostics and platform controls                    |
| Workflow Automation Agent  | `Deferred`            | Typed workflow schema, validation, governance, approval, audit, and orchestrator coordination               |

Catalog discovery lists all nine definitions and their exact state. Deferred
agents fail closed only at later operational selection/task-creation boundaries;
this roadmap defines no evaluator. No role name, instruction source, catalog
membership, or runtime capability grants a tool, credential, filesystem,
shell, cloud, memory, provider, or device right.

## Root phase mapping

The ten root-roadmap phases remain authoritative. The entries below explain the
native catalog's staged work inside those phases.

| Root phase | Staged native outcome                                                                                                                                                                | Status relationship                                                         |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| 1          | Register all nine privilege-free definitions; mark only Personal Assistant and Research Agent `Initial`                                                                              | Verified and published at `f42a6c7`; catalog remains inert and unwired      |
| 2-3        | Add task lifecycle, trusted context, limits, explicit delegation, result collection, cancellation, and the deterministic Personal-to-Research-to-Personal proof above `AgentRuntime` | Verified complete under D-083; foundation remains Rust-only and unwired     |
| 4          | Bind per-agent governance, then establish approved file/root and document-processing boundaries                                                                                      | Blocked on publication plus reviewed governance and document-boundary plans |
| 5          | Add explicit memory namespaces/lifecycle, then stage the Research/Knowledge workflow                                                                                                 | Blocked on Phase 4, ARB-005, and the memory/workflow plans                  |
| 6          | Increase child concurrency only through explicit deterministic limits, cancellation, and resource bounds                                                                             | Blocked until sequential behavior is verified                               |
| 7          | Stage engineering-quality, infrastructure/operations, and automation workflows with each specialist separately enabled                                                               | Blocked on role-specific governance and tool/platform decisions             |
| 8          | Add typed catalog, task, progress, attribution, cancellation, and approval-explanation UI through narrow Tauri IPC                                                                   | Blocked on stable backend contracts                                         |
| 9          | Demonstrate only bounded synthetic or separately approved workflows without capability overclaiming                                                                                  | Blocked on the selected workflow's complete gates                           |
| 10         | Review architecture, security, privacy, cancellation, audit, portability, and rollback across completed phases                                                                       | Blocked until sufficient implementation evidence exists                     |

## Delegation and sequencing invariants

- Only `AgentOrchestrator` creates child tasks.
- Personal Assistant may request delegation; specialists may not spawn agents.
- Workflow Automation may propose a typed workflow but may not execute or spawn
  it.
- In the initial Personal-to-Research phase, delegation depth, total-child
  budget, and active child concurrency are one. Later sequential workflow plans
  must set their own exact finite total-child caps; the bounded-parallelism
  phase may later increase only the explicit concurrency bound, not depth.
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

The first usable milestone stops after the Research Agent result. Knowledge &
Document joins only after approved file/root, document, memory, and governance
boundaries exist. External research requires separately governed read-only
tools.

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

### Workflow automation

```text
Personal Assistant
  -> Workflow Automation Agent
  -> typed workflow proposal
  -> application validation
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> owner approval where required
  -> AgentOrchestrator coordination
```

Workflow Automation cannot bypass `AgentOrchestrator`, `ToolRegistry`,
`PolicyEngine`, `ApprovalManager`, or `AuditLogger` and cannot create recursively
expanding workflows. Orchestrator coordination means validated task sequencing;
any consequential effect still passes through registered tools, policy, exact
approval, restricted execution, and audit.

## Separately gated plans

The AgentDefinition/AgentRegistry plan is implemented, verified, and published.
The combined orchestration-foundation plan is verified complete under D-083.
No later plan is Ready: Phase 4 remains Blocked until this evidence is
published and the governance and document-boundary plans receive fresh review
and exact owner authorization.

| Area                                   | Plan                                                                                                                                      |
| -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Orchestration foundation               | [`2026-08-11-agent-orchestration-task-lifecycle.md`](../plans/2026-08-11-agent-orchestration-task-lifecycle.md)                           |
| Per-agent governance                   | [`2026-08-11-agent-governance.md`](../plans/2026-08-11-agent-governance.md)                                                               |
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
tools, memory, persistence, processes, networking, Tauri IPC, UI, credentials,
platform permissions, agent activation, workflow execution, commit, or
publication. It does not select Hermes or another external framework. Native
remains sole/default, and Hermes remains Deferred/Blocked.

Distributed workers, remote agents, multi-tenancy, a marketplace, a public
plugin ecosystem, billing, enterprise IAM, a cloud control plane, unlimited
recursion, and autonomous destructive operations remain outside current scope.
