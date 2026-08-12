# Native multi-agent roadmap

Status: Subordinate staged roadmap; no implementation authority
Authority: [`ROADMAP.md`](../../ROADMAP.md) remains the authoritative milestone
roadmap
Decision: D-082, D-083, D-084, D-085, and
[`ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`](../adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md)
Last updated: 2026-08-12

## Purpose

This document expands the native multi-agent portion of the root roadmap into a
catalog, activation sequence, workflow map, and separately gated plan set. It
does not change root-roadmap status or authorize implementation. The closed
nine-definition catalog, deterministic registry, bounded orchestrator, and
non-executing governance foundation now exist, remain unwired, and grant no
device or provider authority. D-085's workflow-local bounded volatile memory
store and approved-document reader are verified complete with advisories; no
executor, durable memory, Tauri consumer, or multi-agent UI exists.

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
| Knowledge & Document Agent | `Initial`             | Only D-085's separate bounded Personal-to-Knowledge approved-document route; no generic delegation          |
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

| Root phase | Staged native outcome                                                                                                                                                                | Status relationship                                                     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------- |
| 1          | Register all nine privilege-free definitions; mark only Personal Assistant and Research Agent `Initial`                                                                              | Verified and published at `f42a6c7`; catalog remains inert and unwired  |
| 2-3        | Add task lifecycle, trusted context, limits, explicit delegation, result collection, cancellation, and the deterministic Personal-to-Research-to-Personal proof above `AgentRuntime` | Verified complete under D-083; foundation remains Rust-only and unwired |
| 4          | Bind D-084's non-executing per-agent profile/policy/approval/audit foundation; establish approved file/root and document-processing boundaries only in a separate later increment    | Governance verified and published at `2687294`                          |
| 5          | Add D-085's selected bounded volatile namespaces and approved `.txt`/`.md` Knowledge boundary; keep durable memory and Research/Knowledge separate                                   | Verified complete with advisories; publication pending                  |
| 6          | Increase child concurrency only through explicit deterministic limits, cancellation, and resource bounds                                                                             | Blocked until sequential behavior is verified                           |
| 7          | Stage engineering-quality, infrastructure/operations, and automation workflows with each specialist separately enabled                                                               | Blocked on role-specific governance and tool/platform decisions         |
| 8          | Add typed catalog, task, progress, attribution, cancellation, and approval-explanation UI through narrow Tauri IPC                                                                   | Blocked on stable backend contracts                                     |
| 9          | Demonstrate only bounded synthetic or separately approved workflows without capability overclaiming                                                                                  | Blocked on the selected workflow's complete gates                       |
| 10         | Review architecture, security, privacy, cancellation, audit, portability, and rollback across completed phases                                                                       | Blocked until sufficient implementation evidence exists                 |

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

The first generic-delegation milestone stops after the Research Agent result.
D-085 separately adds a direct Personal Assistant-to-Knowledge task over one
selected approved document. It does not connect Research to Knowledge or grant
either role file selection, enumeration, persistence, or tool authority.
External research requires separately governed read-only tools.

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

The AgentDefinition/AgentRegistry, combined orchestration, and D-084 bounded
non-executing governance foundations are implemented, verified, and published.
D-085 separately selects one combined volatile-memory and narrow
approved-document Knowledge increment. It is verified complete with advisories;
publication requires separate owner direction. Durable memory,
Research-to-Knowledge, and every other later phase remain Blocked, and no next
plan is Ready.

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
