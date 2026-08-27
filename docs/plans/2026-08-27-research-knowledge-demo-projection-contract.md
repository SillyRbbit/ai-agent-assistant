# Research/Knowledge demo projection contract

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-27

## Goal

Define the smallest first Rust-to-WebView step toward a truthful interactive
demo: a read-only, application-owned projection of the sealed D-086 Research
-> Knowledge fixture workflow. This plan does not connect a model, provider,
runtime, tool, approval, audit store, or live workflow.

## User-visible outcome

Command Center can refresh one
Rust-issued closed projection labelled exactly `DEMO MODE · SIMULATED AGENT
DATA`. It shows only the fixed scenario, role catalog, fixture provenance, and
permitted simulated state vocabulary. It cannot start or control a workflow.

## Scope

- One narrow argument-free Tauri query command:
  `get_research_knowledge_demo_projection()`.
- A private Rust constructor owns the sole scenario, roles, disclosure, status
  vocabulary, and bounded redacted display text.
- The exact finite v1 DTO defined below is the entire response surface.
- A typed WebView client invokes as `unknown`, accepts only the exact DTO, and
  maps all rejection to fixed unavailable copy.
- The selected Command Center scenario may refresh that projection only after
  explicit user action; all other Command Center data remains fixture-only.
- F-12 static protection expands atomically to cover the exact command,
  capability, and frontend client boundary.

## Explicit non-goals

- No start, cancellation, restart, transition, subscription, event, polling,
  timer, background work, session state, or mutable native workflow.
- No caller-selected agent, task, run, profile, runtime, workflow, objective,
  source, fixture, or outcome identity.
- No provider, model, network, credential, tool execution, approval dispatch,
  persistence, durable audit, filesystem/document access, generic workflow
  engine, dependency, permission/capability expansion, or device effect.
- No claim that Command Center, mock Conversations, and Rust acceptance
  workflows are one connected proof.

## Current-state evidence

- At the planning baseline, `src-tauri/src/lib.rs` registered only
  `get_app_info`; agent Rust was unwired to Tauri and React.
- `src-tauri/src/agent/research_knowledge.rs` and
  `src-tauri/src/agent/orchestrator.rs` contain D-086's sealed fixture workflow,
  strict provenance, cancellation, and late-event rejection. Their internal
  task/run/context types must not cross IPC.
- `src-tauri/tests/agent_research_knowledge_workflow_contract.rs` is the
  separate deterministic Rust proof. `src/features/command-center/` is the
  separate deterministic frontend proof.
- F-12, F-07, F-01/F-02, F-15, and F-08 are complete. F-03/F-04 are future
  provider prerequisites, not authorization for this fixture-only query.

## Files expected during a later implementation

- `src-tauri/src/lib.rs` and one new private Rust demo-projection module
- focused Rust DTO/command tests and one public contract test
- typed client/tests under `src/infrastructure/tauri/`
- selected Command Center projection/state/component tests and implementation
- `scripts/repository_health.py` and its tests
- affected architecture/current-state records, increment record, and review

## Interfaces and invariants

The query accepts no argument and constructs a fresh DTO solely from private
application-owned constants. It is descriptive only: no native identifier,
authority token, mutable handle, raw event, audit record, objective, fixture
evidence, memory/document content, URL, reasoning, user data, or raw error may
be serialized. Any internal error maps to one closed command error.

The Rust schema is finite and versioned. The TypeScript client rejects null,
arrays, unknown/missing fields, invalid enums/version/disclosure, wrong types,
and oversized strings before rendering. The untrusted WebView cannot select or
forge workflow identity and the query cannot bypass orchestration, policy,
approval, execution, or audit ownership.

The exact v1 response is:

```text
ResearchKnowledgeDemoProjection {
  schemaVersion: "research-knowledge-demo-projection-v1"
  scenarioId: "research-knowledge-demo-v1"
  disclosure: "DEMO MODE · SIMULATED AGENT DATA"
  proofBoundary: "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs."
  fixtureProvenance: "application-owned-synthetic-fixture"
  roles: [
    { id: "personal-assistant", label: "Personal Assistant", state: "ready" },
    { id: "research", label: "Research Agent", state: "ready" },
    { id: "knowledge-document", label: "Knowledge & Document Agent", state: "ready" }
  ]
  simulatedOutcomes: ["succeeded", "failed", "cancelled"]
}
```

There are exactly three `roles`, exactly three `simulatedOutcomes`, and no
other enum values. Every string has a 128-Unicode-code-point maximum except
`proofBoundary`, which has a 160-code-point maximum. The client must construct
fresh local values after validation rather than forwarding an untrusted object.
The listed outcomes are presentation vocabulary only; the command creates no
run and does not assert that any outcome occurred.

Visible and spoken UI disclosure remains exactly `DEMO MODE · SIMULATED AGENT
DATA`, including an explicit statement that Command Center, mock
Conversations, and Rust acceptance workflows are separate deterministic proofs.

## Dependency-ordered follow-ons

1. This read-only projection contract.
2. A separately approved volatile lifecycle plan: one explicit no-argument
   start action, one application-owned active demo, closed events, deterministic
   success/failure/cancellation scripts, late-event rejection, and cleanup.
3. A separately approved connected Command Center presentation plan consuming
   only the lifecycle DTO/events and completing the target-Mac rendered matrix.

No follow-on is authorized here. The later lifecycle plan must prove late-event
rejection and cancellation cleanup; this query makes neither claim.

## Security and privacy review requirements

Review forged/malformed replies, stale frontend state, enum/version confusion,
raw-error disclosure, identity/content exposure, capability broadening, and a
read-only UI control being mistaken for workflow authority. Require exact Rust
ownership, `unknown` narrowing, finite bounds, fixed errors, no fixture-content
logging, no new permissions, and no event/subscription surface.

## Test and verification plan

- Rust: exact DTO, allowed fields/enums only, no sensitive/native identity, and
  closed internal-error mapping.
- Client: accepted DTO plus null, array, missing, extra, wrong-type, oversized,
  invalid-version/state/disclosure, and error-sentinel rejection.
- UI: explicit refresh, loading/ready/unavailable, persistent disclosure, and
  no start/cancel/control affordance.
- Static boundary: reject additional commands/events/capabilities, direct
  frontend Tauri use, caller-selected workflow input, and forbidden
  provider/network/storage/tool imports.
- Run focused tests, `npm run docs:check`, `npm run repository:check`,
  `npm run security:scan`, `npm run verify`, and `git diff --check`.

On the target Mac, confirm development and release views retain disclosure,
refresh only after explicit action, and preserve theme, reduced motion, focus,
scroll, zoom, and native resize. Mark unavailable approved-tooling checks
`Not run`; browser-only evidence is not native evidence.

## Risks, rollback, and stop conditions

- A DTO could become a covert control/identity-export surface; mitigate with no
  inputs/handles, exact validation, and static allowlists.
- The UI could overstate a query as a live agent connection; mitigate with the
  persistent exact disclosure and separate-proof copy.
- Roll back only the bounded command/module/client/projection paths; retain
  planning and review history.
- Stop before implementation if lifecycle control, events, capability/permission
  change, dependency, unbounded DTO, durable state, or provider/tool/filesystem/
  network path becomes necessary. That requires a separate approved plan.

## Progress

- 2026-08-27: Drafted after F-08 completion as planning-only work. Read-only
  projection is intentionally sequenced before lifecycle control so the first
  Tauri boundary cannot start work or confer authority.
- 2026-08-27: The exact v1 query/DTO, finite enum values, field bounds, and
  non-authorizing follow-on sequence were reviewed for architecture and
  security consistency. The plan is ready for a separate owner implementation
  approval; the separate documentation-planning gate is complete, while no
  implementation gate has begun.
- 2026-08-27: The owner approved this exact implementation scope. The active
  `research-knowledge-demo-projection-contract` gate added the argument-free
  Rust query, exact cross-language DTO, fail-closed client, explicit-refresh
  panel, F-12 enforcement, and focused tests without widening the approved
  boundary.
- 2026-08-27: Focused evidence passes: 68 frontend tests, three Rust module
  tests, one exact public Rust contract test, 27 repository-health tests,
  TypeScript, Rust formatting, and the exact UI/native boundary check.
- 2026-08-27: Target-Mac source-current release evidence passes explicit
  refresh, exact ready content, light/dark, reduced-motion on/off, native resize
  from 1040x700 to 803x563 and back, and disclosure retention. The debug process
  launched, but approved accessibility tooling cannot bind directly to its raw
  executable; the source-current browser view and local release `.app` provide
  separate rendered evidence. Final full verification and closeout remain.
- 2026-08-27: Review hardening expanded F-12 to recursively reject every extra
  capability file, top-level plugin configuration, extra security keys, all six
  pinned Tauri emitter methods, `std::net`, and every other prohibited
  projection boundary token. Regression coverage also locks stale-result
  clearing and accessible loading/status attributes.
- 2026-08-27: Final evidence passes: 69 focused frontend tests, three Rust
  module tests, one Rust public contract, 32 repository-health tests, complete
  `npm run verify` with 247 frontend and 494 executed Rust tests plus one
  intentional ignored Hermes probe, and independent architecture, security,
  and code review. The required target-Mac matrix passes with the documented
  raw-debug and host-zoom tooling advisories.

## Acceptance criteria

- [x] Exactly one no-argument query exposes one finite Rust-owned DTO.
- [x] The WebView cannot select or forge trusted workflow identity.
- [x] The UI preserves exact simulation disclosure and has no control effect.
- [x] No provider, model, network, credential, tool, approval, persistence,
      filesystem, background autonomy, generic engine, dependency, permission,
      or device effect is added.
- [x] Focused/full/static/security/target-Mac evidence passes.

## Final results

Implementation matches the approved first prerequisite and remains a
descriptive query only. Complete verification and independent reviews pass.
The result is `PASS WITH ADVISORIES`: direct raw-debug accessibility binding
and host/browser page zoom were unavailable, and non-required DMG packaging
failed after producing the source-current `.app`. No required check failed, no
artifact was published, and no lifecycle successor is Ready or authorized.
