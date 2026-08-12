# Per-agent governance

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: verified definition/registry and task/orchestration foundations plus
a fresh end-to-end trust-boundary decision and readiness review
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: bind exact agent, task, optional parent, runtime, policy-profile,
and memory-namespace identity through every governed proposal, tool, approval,
cancellation, audit, and result path without transferring authority to an
agent or runtime.

## Provisional scope

- Closed `AgentExecutionContext` identity and lineage validation.
- Per-agent policy-profile selection owned by application code.
- Tool availability derived from application configuration, never a role name.
- Exact context binding through policy input, approval subject, audit record,
  cancellation, and terminal result.
- Deterministic failure for missing, unknown, stale, duplicate, or mismatched
  identity.
- Advisory-role boundaries: Security & Risk is not `PolicyEngine`; QA &
  Validation is not `ApprovalManager`; Workflow Automation is not
  `AgentOrchestrator`.

## Explicit non-goals

- No agent authorizes itself or another agent.
- No new tool implementation, executor, provider, memory store, credential,
  Tauri IPC, UI, or platform permission.
- No specialist activation merely because a definition exists.
- No Hermes, external framework, recursive spawning, or autonomous remediation.

## Required work before Ready

- Derive exact files and public types from the verified orchestration baseline.
- Decide the closed policy-profile and memory-namespace identities without
  adding unenforced privilege metadata.
- Complete architecture, security, privacy, and approval/audit binding reviews.
- Specify focused denial, attribution, redaction, cancellation, and regression
  tests plus full validation and rollback.

## Authority and rollback

This draft grants no editing or implementation authority. Removing it returns
governance to roadmap-only status without changing definitions, runtime,
policy, approvals, audit, memory, or application behavior.
