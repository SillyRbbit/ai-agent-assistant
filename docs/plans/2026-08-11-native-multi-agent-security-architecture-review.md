# Native multi-agent security and architecture review

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: sufficient verified native multi-agent phase evidence and an exact
owner-selected review scope
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: perform an evidence-based cross-phase review of application
ownership, task isolation, delegation, cancellation, governance, audit, privacy,
portability, failure containment, and rollback.

## Provisional scope

- Reconcile implementation against D-079, D-082, the runtime ADR, and the native
  multi-agent ADR.
- Trace agent/task/parent/runtime/policy/memory identity through every completed
  path.
- Verify specialists remain advisory/non-authorizing and only the orchestrator
  creates tasks.
- Review catalog activation, route and depth limits, concurrency, cancellation,
  tools, approval, audit, memory, IPC, UI, dependencies, and platform adapters.
- Assess threat scenarios, abuse cases, privacy/data lifecycle, portability,
  deterministic tests, operational evidence, and rollback.

## Explicit non-goals

- No automatic remediation, architecture rewrite, implementation, dependency,
  external action, release approval, or specialist activation.
- No Security & Risk Agent substitution for independent security review or
  `PolicyEngine`.
- No QA & Validation substitution for owner approval or `ApprovalManager`.

## Required work before Ready

Select exact completed phases and files, evidence and reviewers, threat and test
matrix, severity/blocking rules, manual environments, disclosure, report
template, rollback, and required owner decisions.

## Authority and rollback

This draft grants only future read-only review authority after separate owner
approval. Removing it leaves completed increments and their historical evidence
unchanged.
