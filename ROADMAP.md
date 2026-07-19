# Cortexa roadmap

Status: Authoritative milestone roadmap
Last updated: 2026-07-19

## Status model

- **Completed**: acceptance gates and recorded verification passed.
- **Active**: approved work is in progress and not yet complete.
- **Ready**: bounded plan is complete and awaits implementation approval.
- **Proposed**: candidate plan exists but is not selected for the queue.
- **Blocked**: a prerequisite, decision, or verification gate is unresolved.
- **Stopped**: the owner halted the request before implementation; no completion
  evidence exists.
- **Future**: milestone direction only; no bounded Ready increment exists.

`PROJECT_STATUS.md` contains detailed capability evidence. `NEXT_STEPS.md`
contains the ordered execution queue. Completed plans and reviews are the
verification record; this roadmap does not create new completion evidence.

## Product milestones

| Milestone                                                             | Status               | Verified scope                                                                                                                                                   | Remaining gate                                                                                                    |
| --------------------------------------------------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Phase 1 - runnable foundation                                         | Completed            | Tauri/React shell, toolchain, repository workflow                                                                                                                | None                                                                                                              |
| Phase 2 - local application foundation                                | Completed            | Rust interfaces, SQLite bootstrap, macOS lifecycle, React shell, deterministic mock loop, integration hardening                                                  | None                                                                                                              |
| Phase 3 - bounded mock product loop                                   | Completed            | Conversations, context provenance, simulated results, bounded final answer                                                                                       | None                                                                                                              |
| Phase 4 - trusted proposal and approval boundaries                    | Completed through 4U | Closed gateway protocol and request, strict schemas, deterministic policy, exact approvals, native source boundary, cancellation, typed in-memory approval audit | Live transport, execution, and durable audit intentionally absent                                                 |
| Increment 4V - terminal approval-audit binding                        | Completed and merged | Exact 19-path scope passed local and hosted verification and was squash-merged through PR #23 at `6e6f91d`                                                       | Preserve verified boundaries; no later remediation is Ready                                                       |
| Phase 5 - end-to-end policy, approval, audit, and restricted dispatch | Blocked              | Some transport-free primitives were completed in Phase 4                                                                                                         | ARB-002 decisions plus separately approved coordinator, durable audit, dispatch, execution, and failure semantics |
| Phase 6 - basic macOS tools                                           | Future               | Two strict schemas exist without implementations                                                                                                                 | Approve narrow adapters, permissions, tests, and rollback per tool                                                |
| Phase 7 - permissions and onboarding                                  | Future               | Status-only Permission Center exists                                                                                                                             | Approve request flows, disclosure, revocation, and onboarding                                                     |
| Phase 8 - memory and tasks                                            | Blocked              | Volatile mock conversations/tasks only; SQLite bootstrap exists                                                                                                  | Resolve ARB-005 through approved repositories, encryption, retention, review/delete, and recovery increments      |
| Phase 9 - adversarial security validation                             | Future               | Per-increment security review exists                                                                                                                             | Complete threat model, abuse tests, red-team cases, and remediation                                               |
| Phase 10 - production release                                         | Blocked              | Development and no-bundle builds verified; provisional claims stop at macOS 14+ on Apple Silicon                                                                 | Resolve O-003, O-008, O-009, signing, notarization, installer, update, support, and rollback                      |

### Phase 4 acceptance boundary

Phase 4 completion means the transport-free proposal-to-terminal-approval
boundaries through Increment 4U are verified. It does not mean a live provider,
gateway, coordinator, tool implementation, dispatch, execution, product audit,
or user-facing native approval flow exists.

O-006 and O-007 remain blockers for live gateway traffic. Increment 4V is
verified complete and published at `6e6f91d`; its volatile audit boundary does
not resolve those live-transport blockers.

D-059 records the current High-severity disposition. It authorizes no missing
capability: ARB-003, ARB-004, ARB-005, and ARB-008 remain blocked as future
capability work. ARB-006 remains High and is non-blocking only while the
repository remains private and all rights reserved; its trigger is public
distribution or external contributions. ARB-007 remains High and is
non-blocking only for unsigned local development; its trigger is release
candidate or public distribution work.

## Meta and repository milestones

| Meta milestone                                          | Status                                          | Goal                                                                                                                | Acceptance gate                                                                                                       |
| ------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Meta Increment 1 - branding foundation                  | Completed and merged at `5edbf4d`               | Canonical Cortexa assets, guidance, skill, README/favicon/sidebar use                                               | Asset, visual, build, scope, and `meta-01` gate evidence passed                                                       |
| Meta Increment 2 - engineering operating system         | Completed                                       | Consolidate authoritative engineering, architecture, requirements, roadmap, testing, security, and release guidance | Documentation accuracy, links, formatting, complete repository verification, diff review, and `meta-02` gate pass     |
| Meta Increment 3 - Codex automation and quality gates   | Completed and merged at `ad9042c`               | Modular safe repository inspection and evidence-based review workflows                                              | Hook regressions, skill validation, complete verification, scope review, and valid `meta-03` marker passed            |
| Meta Increment 4 - executive documentation request      | Stopped                                         | No gate, plan, or repository edit exists                                                                            | Requires a newly selected and separately approved future increment                                                    |
| Meta Increment 5 - repository health and GitHub hygiene | Completed and merged at `6b149fa`               | Honest repository entry points, review-only automation, GitHub intake, health checks, and licensing status          | Local and hosted verification plus `meta-05` gate evidence passed                                                     |
| Meta Increment 6 - product readiness audit              | Completed and merged at `5281fac`               | Evidence-based readiness assessment and ordered remediation backlog                                                 | Documentation audit and valid `meta-06` gate evidence passed; result `NOT READY (57/100)`                             |
| Meta Increment 7 - verified application icon rollout    | Completed and merged at `96ba6ae`               | Exactly 16 existing Tauri icon files generated from the approved source; debug/release bundles verified             | Raw dev icon and default DMG remain documented advisories; no product capability gate is satisfied                    |
| Remediation ARB-022 - project-memory reconciliation     | Completed and merged at `7c79e65`               | PR #21 publication state and the live queue were reconciled without changing product source                         | Documentation checks and the `remediation-arb-022` gate passed; no remaining ARB-022 publication gate                 |
| Repository risk-based GitHub Actions validation         | Completed at `1780d7f`; reconciled at `74a8d2c` | Two read-only risk-based workflows, deterministic path classification, consolidated audits, and dual-runner routing | Branch and post-merge runs passed on exact Linux runner 21 and macOS runner 22 selectors; D-058 publication is closed |
| High-severity advisory disposition                      | Documentation complete in current workspace     | Evidence-based disposition of ARB-001 through ARB-008 and ARB-044 without source or feature work                    | O-006 through O-009 remain open where owner, legal, privacy, security, or release decisions are required              |

Meta Increments 2, 3, 5, and 6 and repository risk-based CI change documentation
or repository governance only. They do not satisfy any product capability or
release gate. Meta Increment 4 was stopped before implementation. Meta Increment
7 changes identity assets only and does not satisfy a product capability or
release gate.

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

1. Do not begin ARB-002 or another High remediation automatically.
2. Resolve O-006 and O-007 through explicit project, security, privacy, and
   executive ownership before any live model-networking plan can become Ready.
3. Keep ARB-003, ARB-004, ARB-005, and ARB-008 blocked until separately
   approved capability increments are selected.
4. Revisit ARB-006 only before public distribution or external contributions,
   and ARB-007 only before release-candidate or public-distribution work.
5. Do not add transport, credentials, execution, persistence, enterprise
   controls, a license grant, signing, or notarization from this roadmap entry.

Increment 4V is verified complete and published. D-058 and its project-memory
reconciliation are closed. The High-severity disposition identifies no
immediate code remediation, and no later product or remediation increment is
Ready.

## Rollback and reprioritization

A milestone may be reordered only by explicit project-owner direction and a
corresponding update to `NEXT_STEPS.md`, `PLANS.md`, and any durable decision.
Failed verification returns the active increment to Active or Blocked; it does
not create completion evidence. Rollback reverts only the bounded increment and
must preserve historical reports and decisions.
