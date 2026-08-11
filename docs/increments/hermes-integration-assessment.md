# Hermes integration architecture assessment

Status: Verified complete with advisories
Date: 2026-08-11
Gate ID: `hermes-integration-assessment`
Plan: `docs/plans/2026-08-11-hermes-integration-assessment.md`
Baseline: `main` at `40f04b6`, with the separately completed project-direction
documentation preserved as pre-existing work

## Goal

Determine whether Nous Research Hermes Agent could become an optional
experimental runtime without weakening or replacing Cortexa's verified native
architecture.

## Authorized boundaries

- Read the complete repository, source, tests, configuration, and governance.
- Inspect official Hermes source and documentation without installing or
  executing it.
- Create an architecture assessment, a Proposed ADR, and a Proposed next-phase
  native-only ExecPlan.
- Create only the plan, increment, and review evidence required to close this
  documentation increment.
- Do not change production source, tests, dependencies, manifests, lockfiles,
  Tauri configuration, permissions, UI, or runtime behavior.
- Do not accept the ADR, execute the future plan, install Hermes, or perform an
  external write.

## Evidence basis

Repository evidence includes the current Rust gateway protocol and initial-turn
pipeline, tool registry, deterministic policy, exact approval binding, typed
approval audit, storage and platform boundaries, Tauri command/capability
surface, deterministic React mock, public contract tests, manifests, and CI.
The assessment records exact paths and symbols.

Official upstream evidence is pinned to GitHub release `v2026.8.3`, Hermes
Agent version `0.20.0`, release commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`, and its tagged documentation.
The assessment separately records that PyPI's current distribution is `0.19.0`
from `v2026.7.20`, so the channels are not interchangeable. No upstream code
was installed or executed.

## Actual result

The assessment recommends **CONDITIONAL GO** for architecture preservation,
not for Hermes integration. A separately approved phase may introduce a small,
closed, application-owned runtime lifecycle/event boundary and a native adapter
that composes the unchanged `InitialGatewayTurn`. Hermes remains blocked until
an exact immutable distribution, narrow compatible protocol, whole-process
containment, lifecycle cleanup, disabled privileged features, secrets model,
packaging, and target-platform evidence are independently approved and proven.

If Hermes is ever approved, the preferred mechanism is a version-pinned,
whole-process-contained managed subprocess using a closed projection of the
documented TUI-gateway JSON-RPC protocol over stdio. Native remains the default,
reference, deterministic-test, and explicit-fallback direction. Runtime output
remains untrusted; Rust retains validation, policy, exact approval, restricted
execution, cancellation, and audit authority.

## Closeout constraints

Fourteen documentation paths from the preceding completed increment were
already dirty when this assessment began. They are outside this increment and
remain untouched. The post-increment report must inventory them because the
gate validates the complete working tree, while distinguishing them from the
six files created here.

Root project-memory files cannot be resynchronized without overwriting or
mixing that pre-existing work. This is a closeout advisory, not evidence that
the Proposed ADR or future plan is Ready. The next implementation remains
Blocked pending owner acceptance of an additive decision, reconciliation of the
working tree, and a fresh readiness review.

## Rollback

Remove only the six assessment artifacts listed in the active plan and restore
the prior gate state through the repository workflow. Do not alter the
pre-existing documentation set or any application path.
