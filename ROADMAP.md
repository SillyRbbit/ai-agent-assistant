# Cortexa roadmap

Status: Authoritative milestone roadmap
Last updated: 2026-08-20

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

| Milestone                                                             | Status               | Verified scope                                                                                                                                                   | Remaining gate                                                                                                                                                                |
| --------------------------------------------------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Phase 1 - runnable foundation                                         | Completed            | Tauri/React shell, toolchain, repository workflow                                                                                                                | None                                                                                                                                                                          |
| Phase 2 - local application foundation                                | Completed            | Rust interfaces, SQLite bootstrap, macOS lifecycle, React shell, deterministic mock loop, integration hardening                                                  | None                                                                                                                                                                          |
| Phase 3 - bounded mock product loop                                   | Completed            | Conversations, context provenance, simulated results, bounded final answer                                                                                       | None                                                                                                                                                                          |
| Phase 4 - trusted proposal and approval boundaries                    | Completed through 4U | Closed gateway protocol and request, strict schemas, deterministic policy, exact approvals, native source boundary, cancellation, typed in-memory approval audit | Live transport, execution, and durable audit intentionally absent                                                                                                             |
| Increment 4V - terminal approval-audit binding                        | Completed and merged | Exact 19-path scope passed local and hosted verification and was squash-merged through PR #23 at `6e6f91d`                                                       | Preserve verified boundaries; no later remediation is Ready                                                                                                                   |
| Phase 5 - end-to-end policy, approval, audit, and restricted dispatch | Blocked              | Some transport-free primitives were completed in Phase 4                                                                                                         | D-062 identity evidence, D-063 Azure deployment and per-provider ZDR evidence, and separately approved coordinator, durable audit, dispatch, execution, and failure semantics |
| Phase 6 - basic macOS tools                                           | Future               | Two strict schemas exist without implementations                                                                                                                 | Approve narrow adapters, permissions, tests, and rollback per tool                                                                                                            |
| Phase 7 - permissions and onboarding                                  | Future               | Status-only Permission Center exists                                                                                                                             | Approve request flows, disclosure, revocation, and onboarding                                                                                                                 |
| Phase 8 - memory and tasks                                            | Blocked              | Volatile mock conversations/tasks only; SQLite bootstrap exists                                                                                                  | Resolve ARB-005 through approved repositories, encryption, retention, review/delete, and recovery increments                                                                  |
| Phase 9 - adversarial security validation                             | Future               | Per-increment security review exists                                                                                                                             | Complete threat model, abuse tests, red-team cases, and remediation                                                                                                           |
| Phase 10 - production release                                         | Blocked              | Development and no-bundle builds verified; provisional claims stop at macOS 14+ on Apple Silicon                                                                 | Resolve O-003, O-008, O-009, signing, notarization, installer, update, support, and rollback                                                                                  |

### Phase 4 acceptance boundary

Phase 4 completion means the transport-free proposal-to-terminal-approval
boundaries through Increment 4U are verified. It does not mean a live provider,
gateway, coordinator, tool implementation, dispatch, execution, product audit,
or user-facing native approval flow exists.

D-060 separates pluggable identity-provider support, Azure-first portable cloud
hosting, and future trusted AI model-provider support. D-062 selects Microsoft
personal identity as the sole Phase 1 provider while leaving exact registration
evidence and all implementation separately controlled. Google and Apple are
deferred. D-063 selects Azure OpenAI as the Phase 1 synthetic-evaluation
candidate while exact deployment and identity evidence remain open. D-061
accepts O-007's product policy; each AI provider still
requires independent ZDR, data-use, logging, region, and security evidence.

D-059 records the current High-severity disposition. It authorizes no missing
capability: ARB-003, ARB-004, ARB-005, and ARB-008 remain blocked as future
capability work. ARB-006 remains High and is non-blocking only while the
repository remains private and all rights reserved; its trigger is public
distribution or external contributions. ARB-007 remains High and is
non-blocking only for unsigned local development; its trigger is release
candidate or public distribution work.

### Product rollout targets

| State          | Target users and account model                                                                                                     | Identity and control boundary                                                                                                                                                                                             |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Current        | Local deterministic engineering proof; no account or external processing                                                           | No gateway, networking, identity provider, credential path, or transmission                                                                                                                                               |
| Phase 1 target | Individual consumers, consultants, IT professionals, small-business owners, and professional power users using personal workspaces | Microsoft personal identity through provider-neutral system-browser OAuth/OIDC with PKCE S256; no workforce tenants, persistent session, automatic email linking, enterprise administration, SCIM, or organization policy |
| Phase 2 target | Business and enterprise organization accounts and team workspaces                                                                  | Entra workforce SSO, tenant-aware authorization, RBAC, group controls, administration, policy, and audit; SAML, SCIM, and other enterprise providers remain demand-driven                                                 |

Both target phases retain D-061's verified-ZDR, data-minimization, disclosure,
content-logging prohibition, and seven-day operational-metadata boundary. The
table is roadmap direction, not implementation or release evidence.

Cloud hosting is a separate boundary. Initial production targets one primary
Azure Container Apps deployment in Central US at the reserved inactive
`https://api.cortexaai.io` origin. Container portability preserves a future AWS
or Google Cloud option but does not define current cloud support, active-active
multicloud, failover, or a three-cloud release. AI model-provider support is
also separate: a future trusted `AgentProvider` boundary may route only to
individually approved providers, and no such implementation currently exists.

## Native multi-agent roadmap

D-082 accepts application-owned native multi-agent architecture above the
implemented single-run `AgentRuntime`/sole-default `NativeAgentRuntime`
foundation. This sequence is subordinate to the same acceptance gates as every
other product milestone. It does not imply a live provider, model, tool,
durable memory, Tauri consumer, or frontend integration.

The detailed activation and workflow sequence lives in the subordinate
[`NATIVE_MULTI_AGENT_ROADMAP.md`](docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md);
this root roadmap remains authoritative.

| Phase                                          | Status                       | Bounded outcome                                                                                                                                                        | Gate to advance                                                                                           |
| ---------------------------------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| 1. Agent definition and registry               | Completed                    | Nine immutable definitions and deterministic registry; only Personal Assistant and Research were `Initial` at that baseline, none operational                          | Preserve the verified published catalog                                                                   |
| 2-3. Task orchestration and first bounded flow | Completed                    | Closed task lifecycle, trusted context, depth-one typed delegation, one deterministic Research child/result, Personal synthesis, and cancellation above `AgentRuntime` | D-083, verified catalog foundation, deterministic contracts, full validation, and post-increment evidence |
| 4. Agent-specific governance                   | Completed                    | D-084's non-executing per-agent identity, profile, policy, approval, delegation-matrix, and volatile audit foundation; published at `2687294`                          | Preserve the verified boundary                                                                            |
| 5. Agent-specific memory                       | Completed                    | D-085's bounded volatile memory, selected `.txt`/`.md` reader, and direct Personal-to-Knowledge task are published at `5e53f55`; durable memory remains separate       | Preserve the verified boundary; ARB-005 still blocks durable storage                                      |
| 5A. Sequential Research/Knowledge workflow     | Completed                    | D-086's exact fixture-only Personal-to-Research-to-Knowledge-to-Personal sequence passed its bounded implementation gate and is published at `3efd2c1`                 | Preserve the sealed sequential boundary                                                                   |
| 6. Bounded parallelism                         | Completed                    | D-091's sealed fixture-only/no-I/O same-thread selector is verified complete with finite limits, cancellation, failure, ordering, and cleanup                          | Preserve the verified boundary; no successor is Ready                                                     |
| 7. Staged specialist workflows                 | Completed                    | D-087 Engineering, D-088 Cloud/Systems, and D-090 proposal-only Automation verified; D-089 published at `140f05b`                                                      | Preserve the sealed proposal-only/no-I/O limits; D-091 phase-6 closeout is separate                       |
| 8. Desktop UI                                  | Prototype validation pending | Deterministic frontend-only Command Center fixture projection implemented; no authoritative data or control                                                            | Complete the active real-browser/Tauri M5 matrix; live typed IPC remains separately Blocked               |
| 9. End-to-end demonstrations                   | Blocked                      | Bounded synthetic or separately approved demonstrations with exact attribution and no overclaiming                                                                     | Selected workflow gates plus the Blocked demonstration plan                                               |
| 10. Architecture/security review               | Blocked                      | Cross-phase ownership, isolation, cancellation, audit, privacy, portability, and rollback review                                                                       | Complete selected evidence plus the Blocked final-review plan                                             |

The first usable engineering milestone is the completed deterministic
Personal-to-Research contract. It is not a shipping/live assistant milestone.
Delegation remains an explicit application-service operation: only
`AgentOrchestrator` creates child tasks, with initial depth, total-child budget
per root, and active-child concurrency all fixed at one. Completion or
cancellation does not replenish that phase's child budget. Later workflow
arrows are orchestrator-controlled sequential stages at depth one, never direct
specialist spawning. Every expansion requires an exact finite task cap, and
later phases may not begin automatically.

Hermes integration remains **Deferred — evaluated transport and containment
requirements not met**. The rejected raw TUI-gateway stdio, managed
`hermes serve` WebSocket, and ACP evidence for Hermes Agent `0.20.0` /
`v2026.8.3` is preserved and does not block the native sequence. It also does
not select a replacement external transport.

## Meta and repository milestones

| Meta milestone                                           | Status                                          | Goal                                                                                                                | Acceptance gate                                                                                                                                                                         |
| -------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Meta Increment 1 - branding foundation                   | Completed and merged at `5edbf4d`               | Canonical Cortexa assets, guidance, skill, README/favicon/sidebar use                                               | Asset, visual, build, scope, and `meta-01` gate evidence passed                                                                                                                         |
| Meta Increment 2 - engineering operating system          | Completed                                       | Consolidate authoritative engineering, architecture, requirements, roadmap, testing, security, and release guidance | Documentation accuracy, links, formatting, complete repository verification, diff review, and `meta-02` gate pass                                                                       |
| Meta Increment 3 - Codex automation and quality gates    | Completed and merged at `ad9042c`               | Modular safe repository inspection and evidence-based review workflows                                              | Hook regressions, skill validation, complete verification, scope review, and valid `meta-03` marker passed                                                                              |
| Meta Increment 4 - executive documentation request       | Stopped                                         | No gate, plan, or repository edit exists                                                                            | Requires a newly selected and separately approved future increment                                                                                                                      |
| Meta Increment 5 - repository health and GitHub hygiene  | Completed and merged at `6b149fa`               | Honest repository entry points, review-only automation, GitHub intake, health checks, and licensing status          | Local and hosted verification plus `meta-05` gate evidence passed                                                                                                                       |
| Meta Increment 6 - product readiness audit               | Completed and merged at `5281fac`               | Evidence-based readiness assessment and ordered remediation backlog                                                 | Documentation audit and valid `meta-06` gate evidence passed; result `NOT READY (57/100)`                                                                                               |
| Meta Increment 7 - verified application icon rollout     | Completed and merged at `96ba6ae`               | Exactly 16 existing Tauri icon files generated from the approved source; debug/release bundles verified             | Raw dev icon and default DMG remain documented advisories; no product capability gate is satisfied                                                                                      |
| Remediation ARB-022 - project-memory reconciliation      | Completed and merged at `7c79e65`               | PR #21 publication state and the live queue were reconciled without changing product source                         | Documentation checks and the `remediation-arb-022` gate passed; no remaining ARB-022 publication gate                                                                                   |
| Repository risk-based GitHub Actions validation          | Completed at `1780d7f`; reconciled at `74a8d2c` | Two read-only risk-based workflows, deterministic path classification, consolidated audits, and dual-runner routing | Branch and post-merge runs passed on exact Linux runner 21 and macOS runner 22 selectors; D-058 publication is closed                                                                   |
| High-severity advisory disposition                       | Completed and merged at `7bf1a5c`               | Evidence-based disposition of ARB-001 through ARB-008 and ARB-044 without source or feature work                    | D-062 selects the Phase 1 provider; exact identity evidence and O-006 AI-provider configuration remain open; O-008/O-009 retain legal and release gates; D-061 evidence remains pending |
| ARB-002A - gateway threat model and closed configuration | Published at `36ce9ab`                          | D-064 separates design, no-traffic provisioning, synthetic transport, and real-content activation                   | Exact 19-path documentation checks and gate pass with advisories; later stages remain separately blocked                                                                                |

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

1. Treat D-086's exact
   [`2026-08-11-research-knowledge-workflow.md`](docs/plans/2026-08-11-research-knowledge-workflow.md)
   as verified complete with advisories and published at `3efd2c1`.
2. Treat D-087's fixture-only, proposal-only
   [`2026-08-11-engineering-quality-workflow.md`](docs/plans/2026-08-11-engineering-quality-workflow.md).
   Its implementation and closeout are complete with `PASS WITH ADVISORIES`
   and published at `a5d7ba1`.
3. D-088's exact
   [`2026-08-11-infrastructure-systems-operations-workflow.md`](docs/plans/2026-08-11-infrastructure-systems-operations-workflow.md)
   is verified complete with advisories. Do not begin another increment.
4. Preserve D-090's exact
   [`2026-08-11-workflow-automation.md`](docs/plans/2026-08-11-workflow-automation.md)
   verified proposal/manual-dispatch boundary. Its complete valid marker and
   `PASS WITH ADVISORIES` review are final.
5. Treat D-091's exact
   [`2026-08-11-bounded-agent-parallelism.md`](docs/plans/2026-08-11-bounded-agent-parallelism.md)
   as verified complete with advisories under its complete, valid gate. Its same-thread fixture
   proof does not permit a runtime/provider/thread/app-global/general-engine
   change, and no successor plan is Ready.
6. Keep durable memory/ARB-005, live research/retrieval,
   executable automation, provider, IPC, UI, every infrastructure/operations tool or live
   access path, repository effects, and device effects Blocked. Runtime tool
   proposals remain rejected and Native remains sole/default.
7. Do not begin Stage B no-traffic provisioning, Stage C synthetic transport,
   Stage D real-content activation, or another High remediation automatically.
8. Later work must collect D-062's exact Microsoft registration and token
   evidence, D-063's exact Azure deployment evidence, and D-061 provider,
   disclosure, retention, and operational evidence under separate plans.
9. Keep ARB-003, ARB-004, ARB-005, and ARB-008 blocked until separately
   approved capability increments are selected.
10. Revisit ARB-006 only before public distribution or external contributions,
    and ARB-007 only before release-candidate or public-distribution work.
11. Do not add transport, credentials, execution, persistence, enterprise
    controls, a license grant, signing, or notarization from this roadmap entry.

Increment 4V is verified complete and published. D-058 and its project-memory
reconciliation are closed. The High-severity disposition identifies no
immediate code remediation. ARB-002A is documentation-only. The agent
definition/registry increment is published at `f42a6c7`; combined Phase 2-3 is
published at `1d1d9d6`; Phase 4 governance is published at `2687294`; and D-085
is verified complete with advisories and published at `5e53f55` with a valid
published-tree marker. D-086's fixture-based sequential workflow is verified
complete with advisories and published at `3efd2c1`. D-087's fixture-only,
proposal-only engineering-quality workflow is verified complete with advisories
and published at `a5d7ba1`. D-088's two separate fixture-only infrastructure
and systems operations workflows are verified complete with advisories. D-091
is verified complete with advisories under a complete, valid gate. No later
owner-approved plan is Ready.

## Rollback and reprioritization

A milestone may be reordered only by explicit project-owner direction and a
corresponding update to `NEXT_STEPS.md`, `PLANS.md`, and any durable decision.
Failed verification returns the active increment to Active or Blocked; it does
not create completion evidence. Rollback reverts only the bounded increment and
must preserve historical reports and decisions.
