# Workflow automation

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-13
Blocked on: its own accepted decision, complete Ready plan, exact typed workflow
contracts and initial family, approval/audit treatment, and a fresh review
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: let Workflow Automation Agent propose one closed bounded workflow
that application code validates and `AgentOrchestrator` alone may coordinate.
Any consequential effect remains a separately governed application execution
through registered tools, policy, exact approval, restricted execution, and
audit.

## Provisional sequence

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

The arrows describe orchestrator-controlled logical ordering. Workflow
Automation, QA, and Security never spawn the next task. The provisional
total-child cap is exactly three agent tasks and active-child concurrency
remains one. Orchestrator coordination is task sequencing only; any
consequential effect still traverses registered tools, policy, exact approval,
restricted execution, and audit.

## Provisional scope

- Closed workflow, step, dependency, input/output, limit, and failure types.
- Deterministic dependency and sequencing validation.
- Exact agent/task lineage, cancellation, approval subjects, and audit records.
- An allowlisted workflow family whose steps resolve through application-owned
  registries and policy.
- Fail-closed rejection of cycles, unknown targets, unavailable agents,
  unsupported tools, or excessive bounds.

## Explicit non-goals

- No arbitrary commands, free-form scripts, generic executor, recursive or
  self-modifying workflows, scheduler, background autonomy, or direct spawning.
- No bypass of `AgentOrchestrator`, `ToolRegistry`, `PolicyEngine`,
  `ApprovalManager`, or `AuditLogger`.
- No Workflow Automation activation, production execution, persistence, IPC,
  or UI through this draft.

## Required work before Ready

D-089's private D-088 lifecycle/catalog/framing/validation decomposition is
implemented and independent review clears the exact prior next-increment
finding. Its final repository checks and deterministic completion marker remain
the immediate closeout gate. Completion does not activate Workflow Automation
or authorize execution.

Define one exact schema and workflow family, hard limits, cycle/dependency
semantics, cancellation and compensation, approval granularity, audit and
redaction, deterministic adversarial tests, full validation, and rollback.

## Authority and rollback

This draft grants no workflow or execution authority. Removing it leaves
Workflow Automation disabled and preserves current application behavior.
