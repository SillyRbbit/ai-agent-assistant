# Repository project direction and runtime boundaries

Status: Verified complete with advisories
Date: 2026-08-11
Gate ID: `repository-project-direction-runtime-boundaries`
Plan: `docs/plans/2026-08-11-project-direction-runtime-boundaries.md`
Baseline: `main` at `40f04b6`; the completed orphan-gate review is preserved as
non-overlapping pre-existing work

## Goal

Record the owner's present personal-project scope, preserve verified native
architecture, define a conceptual framework-neutral runtime-adapter direction,
and strengthen the existing living execution-plan convention without changing
product behavior.

## Approved boundaries

- Documentation and repository instruction changes only.
- No implementation, dependency, Hermes/OpenClaw installation, provider,
  transport, execution, permission, credential, deployment, or external action.
- Current native modules and deterministic mocks remain unchanged.
- Existing future product targets remain deferred possibilities rather than
  present requirements.
- The separate orphan-gate review remains untouched.

## Expected evidence

- Root instructions link the authoritative direction document.
- D-078 and architecture documentation distinguish conceptual runtime names
  from current source.
- Root plan rules and the increment template contain objective, current-state
  evidence, scope, non-goals, affected components, interfaces and invariants,
  milestones, validation, risks, rollback, decisions, discoveries, progress,
  and final results.
- Documentation-tier verification and the exact protected-source diff pass.

## Actual result

The requested governance is verified complete with advisories. Root instructions
link the authoritative project direction; D-078 records the present scope and
conceptual runtime-adapter direction; architecture documentation states the
current absence and retained Rust authority; and the existing plan convention
contains all requested living-plan sections. The completion marker is `PASS
WITH ADVISORIES` and valid. No product source, test, dependency, configuration,
permission, runtime behavior, current product capability, or readiness changed.
Previously accepted consumer, cloud, provider, enterprise, signing, and release
targets are neither canceled nor implemented.

## Rollback

Restore only the bounded documentation paths. Do not alter the completed
orphan-gate review, application source, historical decisions, or prior reports.
