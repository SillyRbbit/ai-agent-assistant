# Infrastructure and systems operations workflow

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: verified orchestration and governance plus exact cloud, platform,
credential, diagnostic, tool, approval, and rollback decisions
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: stage Cloud Infrastructure or Systems Operations with QA &
Validation and Security & Risk for bounded architecture, inventory, diagnostic,
and change-planning workflows.

## Provisional sequence

```text
Personal Assistant
  -> Cloud Infrastructure Agent or Systems Operations Agent
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> Personal Assistant synthesis
  -> approval before consequential actions
```

`AgentOrchestrator` owns all task creation and depth-one sequencing.
Specialists never spawn another agent. The provisional total-child cap is
exactly three and active-child concurrency remains one.

## Provisional scope

- Cloud architecture, Azure/AWS, Terraform, and infrastructure-as-code review.
- Windows, Linux, macOS, VMware, virtualization, service, process, log, patch,
  backup, and operational analysis.
- Separately approved read-only inventory or diagnostics with exact attribution.
- QA/configuration validation and advisory security/risk review.
- Controlled change plans with explicit approval and rollback requirements.

## Explicit non-goals

- No autonomous apply, modification, deletion, IAM/account change, credential
  use, restart, shutdown, configuration change, patching, backup mutation,
  deployment, or privileged shell.
- QA is not approval authority; Security & Risk is not policy or remediation
  authority.
- No cloud/platform tool, credential, permission, IPC, or UI implementation.

## Required work before Ready

Select one bounded read-only use case, exact platform/provider boundary, safe
tool schema, credential exclusion or custody model, approval and rollback
semantics, deterministic fixtures, private target-environment evidence, and a
fresh architecture/security/readiness review.

## Authority and rollback

This draft authorizes no cloud or device access. Removing it leaves all named
specialists disabled and changes no infrastructure, host, credential, or
application state.
