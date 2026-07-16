# Cortexa roadmap

Status: Authoritative milestone roadmap
Last updated: 2026-07-15

## Status model

- **Completed**: acceptance gates and recorded verification passed.
- **Active**: approved work is in progress and not yet complete.
- **Ready**: bounded plan is complete and awaits implementation approval.
- **Proposed**: candidate plan exists but is not selected for the queue.
- **Blocked**: a prerequisite, decision, or verification gate is unresolved.
- **Future**: milestone direction only; no bounded Ready increment exists.

`PROJECT_STATUS.md` contains detailed capability evidence. `NEXT_STEPS.md`
contains the ordered execution queue. Completed plans and reviews are the
verification record; this roadmap does not create new completion evidence.

## Product milestones

| Milestone                                                             | Status               | Verified scope                                                                                                                                                   | Remaining gate                                                                               |
| --------------------------------------------------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| Phase 1 - runnable foundation                                         | Completed            | Tauri/React shell, toolchain, repository workflow                                                                                                                | None                                                                                         |
| Phase 2 - local application foundation                                | Completed            | Rust interfaces, SQLite bootstrap, macOS lifecycle, React shell, deterministic mock loop, integration hardening                                                  | None                                                                                         |
| Phase 3 - bounded mock product loop                                   | Completed            | Conversations, context provenance, simulated results, bounded final answer                                                                                       | None                                                                                         |
| Phase 4 - trusted proposal and approval boundaries                    | Completed through 4U | Closed gateway protocol and request, strict schemas, deterministic policy, exact approvals, native source boundary, cancellation, typed in-memory approval audit | Live transport, execution, and durable audit intentionally absent                            |
| Increment 4V - terminal approval-audit binding                        | Proposed             | Exact two-file plan exists; 4U prerequisite is merged                                                                                                            | Queue selection and separate owner approval                                                  |
| Phase 5 - end-to-end policy, approval, audit, and restricted dispatch | Future               | Some transport-free primitives were completed in Phase 4                                                                                                         | Approve coordinator, durable audit, dispatch, execution, and failure semantics incrementally |
| Phase 6 - basic macOS tools                                           | Future               | Two strict schemas exist without implementations                                                                                                                 | Approve narrow adapters, permissions, tests, and rollback per tool                           |
| Phase 7 - permissions and onboarding                                  | Future               | Status-only Permission Center exists                                                                                                                             | Approve request flows, disclosure, revocation, and onboarding                                |
| Phase 8 - memory and tasks                                            | Future               | Volatile mock conversations/tasks only; SQLite bootstrap exists                                                                                                  | Approve repositories, encryption, retention, review/delete, and user controls                |
| Phase 9 - adversarial security validation                             | Future               | Per-increment security review exists                                                                                                                             | Complete threat model, abuse tests, red-team cases, and remediation                          |
| Phase 10 - production release                                         | Future               | Development and no-bundle builds verified                                                                                                                        | Resolve O-003, signing, notarization, installer, update, support, and rollback               |

### Phase 4 acceptance boundary

Phase 4 completion means the transport-free proposal-to-terminal-approval
boundaries through Increment 4U are verified. It does not mean a live provider,
gateway, coordinator, tool implementation, dispatch, execution, product audit,
or user-facing native approval flow exists.

O-006 and O-007 remain blockers for live gateway traffic. Increment 4V remains
separately controlled and cannot be inferred from the completion of 4U.

## Meta and repository milestones

| Meta milestone                                        | Status                            | Goal                                                                                                                | Acceptance gate                                                                                                   |
| ----------------------------------------------------- | --------------------------------- | ------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Meta Increment 1 - branding foundation                | Completed and merged at `5edbf4d` | Canonical Cortexa assets, guidance, skill, README/favicon/sidebar use                                               | Asset, visual, build, scope, and `meta-01` gate evidence passed                                                   |
| Meta Increment 2 - engineering operating system       | Completed                         | Consolidate authoritative engineering, architecture, requirements, roadmap, testing, security, and release guidance | Documentation accuracy, links, formatting, complete repository verification, diff review, and `meta-02` gate pass |
| Meta Increment 3 - Codex automation and quality gates | Verified complete; unpublished    | Modular safe repository inspection and evidence-based review workflows                                              | Hook regressions, skill validation, complete verification, scope review, and valid `meta-03` marker passed        |
| Meta Increment 4 - verified application icon rollout  | Ready                             | Replace only the 16 existing Tauri icon files from the approved source                                              | Reconciled Meta 3 publication, separate owner approval, generation review, package build, and target-Mac matrix   |

Meta Increments 2 and 3 change documentation and repository governance only.
They do not satisfy any product capability or release gate. Meta Increment 4 is
the first Ready item and must not start automatically.

## Milestone acceptance gates

Every product or meta milestone requires:

1. A bounded approved plan with exact files, risks, non-goals, verification,
   manual gates, and rollback.
2. Focused tests or document checks appropriate to the change.
3. Complete relevant repository verification.
4. Security, privacy, architecture, code-health, and scope review.
5. Synchronized current-state memory and an increment record.
6. A PASS or PASS WITH ADVISORIES post-increment report and valid marker.
7. Separate project-owner direction for commit, publication, merge, or release.

Release milestones additionally require `RELEASE_CHECKLIST.md` and
`SECURITY_CHECKLIST.md` to pass with target-platform evidence.

## Current queue

1. Obtain explicit project-owner direction before committing and publishing
   verified Meta Increment 3.
2. Reconcile clean synchronized `main`, then request separate approval for Meta
   Increment 4 under `docs/plans/meta-04-verified-application-icon-rollout.md`.
3. Do not infer selection of Increment 4V or any product milestone from Meta 3
   or Meta 4 readiness.

No later product increment is Ready in this roadmap.

## Rollback and reprioritization

A milestone may be reordered only by explicit project-owner direction and a
corresponding update to `NEXT_STEPS.md`, `PLANS.md`, and any durable decision.
Failed verification returns the active increment to Active or Blocked; it does
not create completion evidence. Rollback reverts only the bounded increment and
must preserve historical reports and decisions.
