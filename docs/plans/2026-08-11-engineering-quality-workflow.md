# Engineering quality workflow

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: verified orchestration and per-agent governance plus exact
repository, tool, approval, and change-control decisions
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: stage Coding, QA & Validation, and Security & Risk as bounded
application-owned advisory roles in an orchestrator-controlled engineering
workflow.

## Provisional sequence

```text
Personal Assistant
  -> Coding Agent
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> Personal Assistant synthesis
  -> approval before consequential changes
```

The orchestrator creates and sequences every depth-one specialist task. No
specialist spawns another agent. The provisional total-child cap is exactly
three and active-child concurrency remains one.

## Provisional scope

- Repository inspection, explanation, planning, and patch proposals.
- Separately approved code changes and safe validation tools only after exact
  policy and approval gates exist.
- Independent acceptance, regression, configuration, and output validation.
- Advisory threat, secret, policy, and change-risk assessment.
- Exact attribution, cancellation, bounded inputs/results, and audit evidence.

## Explicit non-goals

- No autonomous commit, push, dependency installation, destructive command,
  privileged shell, secret use, release, or deployment.
- QA & Validation never approves its own privileged action and never becomes
  `ApprovalManager`.
- Security & Risk never becomes `PolicyEngine` and cannot authorize or execute
  remediation.
- No source, tool, IPC, or UI implementation through this draft.

## Required work before Ready

Define exact repository roots, patch/change ownership, safe command catalog,
approval subjects, test isolation, rollback, denial cases, target-platform
evidence, and a fresh architecture/security/readiness review.

## Authority and rollback

This draft authorizes no repository mutation or command execution. Removing it
leaves all three agents disabled and current development workflows unchanged.
