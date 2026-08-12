# Cortexa project direction

Status: Owner-approved repository-governance and planned-architecture direction
Last updated: 2026-08-11
Decision authority: D-078

This document defines present project scope and future-facing architecture
constraints. It is not implementation, readiness, release, or security evidence.
`ARCHITECTURE.md` and `PROJECT_STATUS.md` remain the current-capability
authorities, and accepted decisions govern changes to architectural targets.

## Current project scope

The AI-Agent-Assistant repository contains Cortexa, currently a private,
personally owned side project whose only technical user is the repository owner.
It is local-first and is used for personal productivity, experimentation,
learning, development, and demonstrations.

The project does not currently need:

- SaaS infrastructure;
- multi-tenancy;
- billing;
- enterprise identity and access management;
- public deployment infrastructure; or
- production-scale distributed systems.

These are present-scope statements, not a cancellation of accepted future
product targets. Cortexa may eventually be published or become a publicly
distributed product. Future-facing capabilities remain deferred until an
owner-selected bounded increment has evidence, a plan, required decisions, and
explicit authorization.

## Guiding principle

> Build it with clean architecture. Scope it like a personal project. Preserve
> the path to a future product.

Apply that principle by retaining clean application-owned boundaries and
portable domain contracts without building speculative commercial
infrastructure. Future publication should not require an avoidable rewrite, but
publication requirements do not belong in the present implementation before
their triggers occur.

## Native architecture preservation

The verified native Rust boundaries, deterministic frontend mocks, closed
contracts, tests, documentation, and accepted decisions are durable project
assets. Read their implementation and evidence before proposing replacement.
Prefer adapting or wrapping current behavior over rewriting it around an
external framework.

Preservation applies to current, meaningful implementation and evidence. It
does not restore the unused generic scaffolds deleted in Increments 4I through
4M, including D-032's synchronous arbitrary-string `AgentProvider`, and it does
not prevent a later explicitly approved increment from removing a verified
component for evidence-backed reasons.

## Current runtime and native multi-agent direction

The application-owned runtime foundation now exists in Rust but is not wired to
Tauri, React, a provider, or a live model:

```text
AgentRuntime
├── NativeAgentRuntime
└── HermesAgentRuntime (deferred; not implemented)
```

- `AgentRuntime` is a framework-neutral one-run contract owned by Cortexa.
- `NativeAgentRuntime` composes the existing typed native path. It is the
  sole/default runtime, explicit future fallback, reference implementation,
  deterministic contract-test path, and possible standalone runtime. “Fallback”
  does not mean automatic runtime or model-provider failover.
- `HermesAgentRuntime` remains **Deferred — evaluated transport and containment
  requirements not met**. Raw stdio, managed `hermes serve` WebSocket, and ACP
  were rejected for the exact evaluated Hermes `0.20.0` / `v2026.8.3`
  conditions. No replacement transport is selected.
- OpenClaw may be evaluated later as another possible adapter. It is not a
  current adapter, selected dependency, implementation task, or planned
  integration.

This runtime seam is distinct from the future model-provider transport boundary
described as `AgentProvider`. It must not revive D-032's removed interface or
spread framework-specific payloads through general application APIs.

Every runtime and provider result remains untrusted. Deterministic Rust outside
an external adapter retains validation, policy, exact approval, restricted
execution, cancellation, and audit ownership. No external runtime, framework,
model, gateway, WebView, or hook gains direct device authority.

D-082 accepts a native multi-agent application-service layer above this runtime:

The diagram is a logical task/delegation topology. Actual control remains with
the application/orchestrator, which invokes `AgentRuntime` for each run; agent
definitions do not call runtimes or gain authority.

```text
Personal Assistant
       |
       v
AgentOrchestrator
       |
       +-- Research & knowledge roles
       +-- Software engineering roles
       +-- Infrastructure & operations roles
       `-- Workflow Automation Agent
               |
               v
       NativeAgentRuntime
```

The planned catalog contains nine application-owned roles: Personal Assistant;
Research; Knowledge & Document; Coding; QA & Validation; Security & Risk; Cloud
Infrastructure; Systems Operations; and Workflow Automation. QA and Security
are cross-cutting, but group membership grants no route or authority.
`AgentOrchestrator`, task lifecycle, delegation, policy profiles, and memory
namespaces stay outside `AgentRuntime`.

The initial deterministic phase keeps delegation depth, total-child budget per
root, and active-child concurrency at one; the root is Personal Assistant, only
Personal Assistant to Research Agent is enabled, and only the orchestrator may
create a child task. Terminal child work does not replenish that phase's budget.
Future staged workflows remain orchestrator-sequenced at depth one and require
exact finite task caps and separate plans. This is accepted architecture and
planning direction, not current behavior; only the separately approved
AgentDefinition/AgentRegistry plan is Ready. It defines all nine roles but
initially enables only Personal Assistant and Research Agent, and registration
or activation never grants tools, policy, approval, memory, provider, or device
authority.

`AgentOrchestrator`, `AgentRuntime`, `NativeAgentRuntime`, `AgentRegistry`,
`ToolRegistry`, `PolicyEngine`, `ApprovalManager`, `AuditLogger`, `MemoryStore`,
and `PlatformAdapter` remain application-owned authorities (some are planned,
not current code). Security & Risk is not the policy engine, QA & Validation is
not the approval manager, and Workflow Automation is not the orchestrator.

## Architectural boundaries to preserve

Keep explicit ownership between:

- React and TypeScript presentation;
- the narrow Tauri command and event boundary;
- Rust application services;
- framework-neutral domain logic;
- future agent-runtime adapters;
- deterministic policy;
- exact approvals;
- registered tools and any separately approved restricted executor;
- auditing;
- memory and persistence; and
- platform-specific adapters.

General application and domain APIs own their types. External-framework types
must not cross their adapter. Use strongly typed Rust domain models and closed
typed errors. Production paths must not use `unwrap()`, `expect()`, `panic!()`,
`todo!()`, or `unimplemented!()` except under an existing narrowly documented
repository convention.

Preserve deterministic, visible, no-network mocks. Unknown tools, privileged
actions, framework events, identities, and states fail closed. Never place
secrets, credentials, tokens, sensitive memory, or raw provider payloads in
source, the WebView, SQLite, logs, tests, documentation, or ordinary CI.

## Integration and work guardrails

- Read current source, tests, plans, and decisions before proposing replacement
  or integration work.
- Add an external framework through one narrow adapter; do not restructure the
  application around it or expose its payloads as application contracts.
- Preserve cancellation, limits, translation, redaction, negative-path, and
  deterministic contract tests across every future adapter.
- After an exact task or increment is authorized, safe in-scope local work may
  proceed without repeated confirmation. External writes, destructive actions,
  publishing, deployment, credential changes, and material scope expansion
  still require explicit authorization.
- Run applicable formatting, linting, type checking, tests, security review, and
  repository gates. Do not suppress or delete failing tests to claim success.
- A production dependency requires an accepted need, evidence that current
  dependencies are insufficient, maintenance and license review, minimal
  features, exact lockfile review, and applicable verification.
- Do not change unrelated files or use an integration as permission for a
  speculative refactor.

## Execution plans

Use the existing `PLANS.md`, `docs/plans/`, and
`docs/templates/INCREMENT_TEMPLATE.md` convention. Do not create a parallel
`.agent/PLANS.md` hierarchy.

An ExecPlan is a living implementation document for significant architectural
changes, external integrations, cross-cutting refactors, multi-step features,
and security-sensitive work. It records objective, current-state evidence,
scope and non-goals, affected components, interfaces and invariants, milestones,
validation, risks, rollback, decisions, discoveries, progress, and final
results.

A plan does not grant authority. When a prompt already authorizes implementation,
update the relevant plan, begin the required repository gate, and continue in
the same task unless the prompt requests analysis only or a stop condition is
reached.

## Related authority

- D-026 governs Cortexa display naming and preserved `ai-agent-assistant`
  compatibility identifiers.
- D-032 records deletion of the legacy provider scaffold.
- D-060 separates identity-provider, cloud-hosting, and AI model-provider
  boundaries for possible future product phases.
- D-065 governs the repository instruction hierarchy and documentation-only
  treatment of future multi-agent concepts.
- D-078 records this present-scope, preservation, and runtime-adapter direction.
- `ARCHITECTURE.md`, `PROJECT_STATUS.md`, and `NEXT_STEPS.md` distinguish current
  capability, verified state, and authorized queue order.
