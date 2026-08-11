# Project direction and runtime boundaries

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-11
Gate ID: `repository-project-direction-runtime-boundaries`
Baseline: `main` at `40f04b6`; one non-overlapping orphan-gate review is
preserved unmodified

## Goal

Establish durable present-scope, native-architecture preservation,
framework-adapter, and execution-plan guidance before any Hermes or other
external-runtime work is considered.

## User-visible outcome

Repository contributors and coding agents can identify the project's current
owner-only personal scope, distinguish conceptual runtime direction from current
capability, preserve the verified native path, and use one existing living-plan
convention for later significant work.

## Current-state evidence

- The repository is private, local-first, and currently used by its owner, but
  no single authoritative direction document states the complete present scope.
- `ARCHITECTURE.md` records a transport-free native Rust initial-turn pipeline
  and a separate deterministic frontend mock; neither is a shipping runtime.
- No `AgentRuntime`, `NativeAgentRuntime`, `HermesAgentRuntime`, OpenClaw adapter,
  runtime selector, live transport, coordinator, dispatcher, or executor exists.
- D-032 deleted the unused synchronous arbitrary-string `AgentProvider` API;
  D-060 describes a separate future provider-transport boundary.
- Root `PLANS.md`, `docs/plans/`, and the increment template already form the
  execution-plan convention. The template lacks explicit interfaces,
  invariants, decisions, discoveries, progress, and final-result sections.

## Scope

Create:

- `docs/PROJECT_DIRECTION.md`
- `docs/plans/2026-08-11-project-direction-runtime-boundaries.md`
- `docs/increments/repository-project-direction-runtime-boundaries.md`
- `docs/reviews/2026-08-11-repository-project-direction-runtime-boundaries-post-increment-review.md`

Modify:

- `AGENTS.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/templates/INCREMENT_TEMPLATE.md`

The pre-existing
`docs/reviews/2026-08-11-apple-support-ts-017-owner-contact-d077-contact-1-post-increment-review.md`
belongs to the completed orphan-gate reconciliation. Preserve it byte-for-byte;
the final report must identify it separately because the repository gate checks
the complete working-tree inventory.

## Explicit non-goals

- No production source, tests, dependencies, lockfiles, configuration, Tauri
  permissions, IPC, UI, storage, networking, runtime, tool, or application
  behavior change.
- No Hermes or OpenClaw installation, dependency selection, integration,
  prototype, transport, coordinator, dispatcher, executor, or multi-agent
  implementation.
- No restoration of deleted generic provider, audit, memory, or platform
  scaffolds.
- No cancellation or implementation of accepted future consumer, cloud,
  provider, enterprise, signing, or release targets.
- No branch, commit, publication, deployment, or external action.

## Affected components

- Root repository instructions and reading order.
- Authoritative current/planned architecture documentation.
- Accepted architecture decisions.
- Execution-plan rules and template.
- Current project-memory and completion evidence.

## Interfaces and invariants

- Runtime names remain conceptual; documentation must not claim corresponding
  source types or shipping selection behavior.
- The future runtime seam is framework-neutral and distinct from provider
  transport. D-032's deleted interface stays deleted.
- External-framework types remain adapter-local and translate into closed,
  Cortexa-owned domain values.
- Deterministic Rust retains validation, policy, approval, restricted execution,
  cancellation, and audit ownership.
- The verified native path, mocks, contracts, tests, and decisions remain
  preserved unless a later task explicitly authorizes evidence-backed removal.
- Present personal scope defers rather than silently cancels accepted future
  product possibilities.

## Implementation milestones

- [x] Inspect Git, repository guidance, current architecture, source evidence,
      tests, and the existing plan convention.
- [x] Reconcile and close the orphaned active post-increment gate without
      altering product state.
- [x] Add project-direction and root instruction guidance.
- [x] Add the conceptual runtime boundary and D-078.
- [x] Extend the existing living ExecPlan convention.
- [x] Synchronize current project memory and completion evidence.
- [x] Run the documentation completion gate and review the exact diff.

## Security and privacy considerations

This increment grants no model, framework, gateway, WebView, or runtime device
authority. It preserves fail-closed tool handling, deterministic Rust ownership,
adapter-local external types, credential secrecy, explicit approval, and the
existing no-network/no-execution boundary.

## Validation commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock
python3 .codex/hooks/session_end_gate.py
```

Manual review must confirm current, mocked, planned, and prohibited states are
distinct; no production path changed; and the orphan-gate review stayed
byte-for-byte unchanged.

## Risks

- Runtime wording could imply nonexistent types or shipping behavior. Control:
  label the entire shape conceptual and record current absence.
- “Preserve native” could revive deleted speculative scaffolds. Control:
  preserve verified current assets while retaining explicitly approved removal.
- Personal scope could silently cancel accepted future targets. Control: frame
  it as present scope and deferred possibility under additive D-078.
- Adapter language could transfer trusted authority to Hermes. Control: keep
  deterministic Rust trust gates outside every external adapter.
- Plan guidance could bypass owner approval. Control: distinguish continuing an
  already authorized task from authority granted by a plan.

## Rollback or failure strategy

Before publication, restore only this declared documentation scope while
preserving the pre-existing orphan-gate review. After publication, revert one
bounded documentation commit and supersede D-078 additively if direction
changes. No product, data, credential, deployment, or runtime rollback applies.

## Decisions made

- Reuse and extend the established root `PLANS.md` plus `docs/plans/`
  convention; do not create `.agent/PLANS.md`.
- Add D-078 rather than reinterpret D-032 or cancel D-060's future targets.
- Treat `AgentRuntime`, native, Hermes, and OpenClaw names as conceptual only.

## Discoveries

- The only repository `hermes-parser` and `hermes-estree` references are
  unrelated transitive frontend tooling dependencies.
- The current native initial-turn path is test-invoked and transport-free; the
  deterministic browser mock is separate.
- An orphaned ignored gate was opened after the committed D-077 closeout under
  a different ID. It was closed through an exact no-operation review before
  this increment began.

## Progress

- 2026-08-11: completed repository, guidance, source, test, and plan inventory.
- 2026-08-11: closed and validated the orphaned no-operation gate.
- 2026-08-11: began `repository-project-direction-runtime-boundaries`.
- 2026-08-11: added D-078, project direction, planned runtime-adapter
  boundaries, living ExecPlan fields, and synchronized project memory.
- 2026-08-11: preliminary documentation, repository, security,
  protected-source, whitespace, and session-end checks passed; final review and
  marker remain pending.
- 2026-08-11: independent reviews were reconciled, the required commands passed
  on the final report, and the post-increment marker completed as `PASS WITH
ADVISORIES` and `valid: true`.

## Acceptance criteria

- [x] Present owner-only personal scope and exact guiding principle are durable.
- [x] Native preservation and conceptual multi-runtime direction are explicit.
- [x] Hermes stays optional and adapter-local; OpenClaw remains deferred.
- [x] Execution-plan guidance contains every requested living-plan section.
- [x] Current and future roadmap states remain accurately distinguished.
- [x] No production implementation, dependency, or behavior changes.
- [x] Required checks and post-increment gate pass.

## Final results

The documentation-only increment is verified complete with advisories. D-078,
the root instructions, project direction, and architecture preserve current
owner-only personal scope and the native path while defining only a conceptual
future runtime-adapter seam. The existing plan convention contains every
requested living-plan field. No product source, test, dependency, configuration,
permission, runtime behavior, or current capability changed. No later product
or external-runtime increment is Ready.

## Documentation updates

- [x] `AGENTS.md`
- [x] `docs/PROJECT_DIRECTION.md`
- [x] `ARCHITECTURE.md`
- [x] `DECISIONS.md`
- [x] `PLANS.md` and the increment template
- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `CHANGELOG.md`
- [x] Increment record and post-increment review
