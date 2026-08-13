# Cortexa project direction

Status: Owner-approved repository-governance and planned-architecture direction
Last updated: 2026-08-13
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

The current catalog contains nine application-owned roles: Personal Assistant;
Research; Knowledge & Document; Coding; QA & Validation; Security & Risk; Cloud
Infrastructure; Systems Operations; and Workflow Automation. QA and Security
are cross-cutting, but group membership grants no route or authority. The Rust
core also contains the bounded `AgentTask`, trusted `AgentExecutionContext`,
and `AgentOrchestrator` foundation accepted by D-083. These remain unwired and
do not make any definition a shipping or autonomous assistant.

D-084 also implements a non-executing governance foundation above the same
unwired task/runtime core. Nine exact policy profiles bind sealed definition,
task, live runtime, approval, delegation, and volatile audit attribution. The
Personal profile alone is eligible for the two existing local schemas; all
specialists deny them. Approval never dispatches a tool, every execution result
is `NotAttempted`, runtime tool proposals remain rejected, and no provider, IPC,
UI, durable audit, executor, or device action exists.

The generic deterministic orchestration phase keeps delegation depth,
total-child budget per root, and active-child concurrency at one; the root is
Personal Assistant and only the orchestrator may create a child task. Terminal
child work does not replenish that phase's budget. Generic delegation remains
exactly Personal Assistant to Research Agent. D-085 adds a separate direct
Personal Assistant-to-Knowledge approved-document route. Neither path enables
generic/direct Research-to-Knowledge or specialist spawning. Later workflow
families remain orchestrator-sequenced at depth one and require exact finite
task caps and separate plans.

D-085 implements a process-local, one-workflow `MemoryStore` with approved-
shared, agent-private, task-temporary, and proposed-shared domains; exact sealed
memory-profile attribution; explicit bounded context selection; versioned
application review; and terminal cleanup. It also adds an opaque, read-only
approved-document boundary for selected nonempty lowercase `.txt`/`.md` UTF-8
files and one deterministic Knowledge child plus fresh Personal synthesis run.
At the D-085 checkpoint, Knowledge became `Initial` only for that separate
non-authorizing route; D-086 now also uses the same non-authorizing eligibility
inside its sealed fixture workflow. Paths remain application-selected and
private; documents and memory remain untrusted. No persistence, IPC, UI,
provider, live model, vector search, unrestricted file tool, executor, or
device authority exists. Focused contracts, the complete repository suite, and
independent review pass with `PASS WITH ADVISORIES`; the only accepted residual
is the pure-`std` Unix document-open TOCTOU race.

D-086 implements one further sealed, fixture-only application-service sequence:
Personal Assistant -> Research -> Knowledge -> Personal synthesis. Research and
Knowledge are sequential depth-one siblings created only by the orchestrator.
The path is fixed at three tasks, two non-replenishing children, one active
child, four run attempts, 32 runtime and generic events, 16 workflow events and
matching non-authoritative audit records, and zero automatic retries. Strict
versioned results preserve only application-issued fixture source IDs; missing
Research references produce explicit partial status and skip Knowledge, valid
incomplete Knowledge remains partial, and unknown or remapped references fail
closed. Final Personal synthesis is likewise strict: its catalog references,
fixture-only disclosure, and complete/partial status are validated; its source
set must exactly match validated Research and its answer must disclose fixture
and applicable partial status, while invented citations, URLs, live-research
claims, or reasoning fail the root.
Terminal preparation, truthful fallback, child-first cancellation,
task-memory cleanup, pending-review Knowledge proposals, content-free events,
and descriptive redacted attribution remain application-owned. Generic/direct
Research-to-Knowledge remains denied; this exact sequence grants neither
specialist spawn authority nor any tool, provider, network, filesystem,
persistence, IPC, UI, runtime, approval, execution, or device authority.

D-087 implements one separate fixture-only, proposal-only application-service
sequence: Personal Assistant -> Coding -> QA & Validation -> Security & Risk ->
Personal synthesis. The three specialists are sequential depth-one siblings
created only by `AgentOrchestrator` under four-task, five-run,
one-active-child, zero-retry limits. Strict proposal, validation, risk, and
final synthesis results preserve application-issued fixture/criterion/evidence
provenance. QA and Security remain advisory; consequential capabilities are
denied data; a patch derives `RequiredBeforeMutation` without creating an
approval request or execution subject. The path adds no live repository,
filesystem, process, Git, package, network, tool, executor, mutation, memory,
provider, IPC, UI, external runtime, or device authority.

D-088 implements two further separate fixture-only/no-I/O application-service
sequences: Personal Assistant -> Cloud Infrastructure -> QA -> Security ->
Personal synthesis, and the corresponding Systems Operations sequence. The
Cloud built-in contains synthetic Terraform configuration, Azure architecture,
and validation evidence. The Systems built-in contains a synthetic service
snapshot, sanitized log, recovery scenario, and validation evidence. Each uses
four tasks, five attempts, one active depth-one child, and zero retries. Strict
assessments, plans, QA, Security, and synthesis preserve application-issued
provenance; consequential capabilities remain denied; no command, credential,
live access, tool, executor, approval dispatch, provider, IPC/UI, dependency,
or effect exists.

The catalog now marks all nine definitions, including Workflow Automation, as
non-authorizing `Initial`. Coding/QA/Security eligibility exists only for D-087 and the
applicable D-088 cross-cutting stages; Cloud and Systems eligibility exists
only for their separate D-088 sealed unwired selectors. Workflow Automation
eligibility exists only for D-090's sealed unwired proposal selector; its
generic route, tool profile, memory profile, and non-authority posture remain
unchanged. Registration,
activation, task creation, memory profile, or orchestration grants no tool,
policy, approval, provider, execution, or device authority.

D-089 separately decomposes private workflow lifecycle and parser ownership
without changing behavior. D-090 implements one strict Personal -> Workflow
Automation -> Personal proposal lifecycle, five immutable templates, and an
expiring take-once manual bridge from complete A-D proposals to the already
implemented sealed fixture-only/no-I/O selectors. Template E and every tool or
approval step remain non-executable. The cooperative deadline is not hard
preemption. No general engine, scheduling, persistence, executor, approval
dispatch, provider, IPC/UI, external runtime, I/O, or effect is added.

D-091 implements one sealed fixture-only/no-I/O bounded-parallel selector.
`AgentOrchestrator` may retain multiple independent depth-one specialist runs
and accept exact task/run-addressed events on one application thread. Three
immutable scenarios use explicit `ContinuePartial`, `CancelDependentOnly`, or
specialist-lane `FailFast`; catalog ordinals control dependency transfer,
cancellation, outcome order, and truthful Personal synthesis. Bounds remain
default active two, hard active and total child three, four tasks, five run
attempts, zero retries, eight events per run, 32 applicable records, and
cooperative 120-second root/60-second child leases. This is not provider or CPU
concurrency, hard preemption, provider-session isolation, app-global capacity,
a scheduler, or a general engine. Runtime traits and Native remain unchanged.

`AgentOrchestrator`, `AgentRuntime`, `NativeAgentRuntime`, `AgentRegistry`,
`ToolRegistry`, `PolicyEngine`, `ApprovalManager`, `AuditLogger`, `MemoryStore`,
and `PlatformAdapter` remain application-owned authorities. The workflow-local
volatile `MemoryStore` now exists; durable memory, a general audit logger, and
`PlatformAdapter` remain planned. Security & Risk is not the policy engine, QA
& Validation is not the approval manager, and Workflow Automation is not the
orchestrator.

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
- D-082 accepts the application-owned native multi-agent architecture above the
  runtime seam.
- D-086 authorizes only the implemented fixed fixture-based Research and
  Knowledge sibling sequence and leaves generic delegation and runtime authority
  unchanged.
- `ARCHITECTURE.md`, `PROJECT_STATUS.md`, and `NEXT_STEPS.md` distinguish current
  capability, verified state, and authorized queue order.
