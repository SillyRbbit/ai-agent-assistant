# Cortexa roadmap

Status: Authoritative milestone roadmap
Last updated: 2026-09-03

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

## V0-6 direct Rust HTTPS dependency decision

The documentation-only V0-6 decision is **Completed** with `PASS WITH
ADVISORIES`. D-118 accepts `no_eligible_client` after all five exact variants
across three frozen families failed at least one mandatory criterion. No
candidate, dependency, implementation, network path, or external authority is
selected.

V0-3 remains **Blocked** and unwaived. V0-7 and all later transport milestones
remain **Blocked** pending their recorded prerequisites, an eligible design,
separate implementation authority, and V0-7's required fake-only versus
hermetic actual-client TLS/socket-test reconciliation. D-107's ten operational
blockers are unchanged; V0-6 is not a signing or D-107 operational successor.

## D-107 late-result rejection decision

The documentation-only `d107-late-result-rejection-contract-decision` is
**Completed** with `PASS WITH ADVISORIES`. Current source supports only
`late_result_rejection_not_accepted`, so D-117 is **Proposed**, negative, and
non-controlling. The work creates no private-key attempt host, result ingress,
runtime interface, cancellation, cleanup, signing, or platform capability.

Historical D-107 remains 8/11 and D-108 remains additively 9/10. All ten
contracts remain blockers, the candidate remains unadmitted, and P3, P4,
signing, V0-3, and every operational successor remain **Blocked**. Completion
of the documentation increment does not accept D-113 through D-117 or select a
successor. Proposed D-117 remains pending and non-controlling. V0-6 is now
**Completed** with accepted D-118 `no_eligible_client`; it changes no D-107
result or successor readiness.

## D-107 post-D-111 decision-lineage reconciliation

The documentation-only reconciliation is **Completed** with `PASS WITH
ADVISORIES`. Proposed D-113 through D-116
provide non-colliding durable numbers for four published negative results:
private-key non-export, fixed algorithm, interaction denial, and hard deadline/
cancellation. They preserve the historical artifacts and unrelated GUI D-112,
do not retroactively validate predecessor gates, and do not change any D-107
outcome.

Historical D-107 remains 8 documented / 11 unproved and D-108's additive
interpretation remains 9/10. All ten contracts remain blockers, no candidate is
admitted, and P3, P4, signing, V0-3, and every operational successor remain
**Blocked**. At that reconciliation checkpoint, late-result rejection had not
begun; it later began only as the separately owner-approved documentation
assessment above. The proposals themselves remain **Proposed** pending owner
acceptance.

## Completed dependency-security remediation

The owner-approved `browserslist-4-28-7-security-remediation` is **Completed**
with `PASS WITH ADVISORIES`. The existing development-only Browserslist lock
resolution is patched at 4.28.7 with only its four required support-floor
movements; exact-head CI is green. It changes no product milestone or
authority. D-112 remains in force, and D-111 continues to block automatic
product and operational successors.

## D-107 account and Keychain scope decision

D-111 is **Completed** with `PASS WITH ADVISORIES`: no application-owned
identity scope exists in the reviewed repository. All operational work remains
**Blocked**.

## D-107 exact signer-binding decision

The documentation-only D-110 decision is **Completed** with
`PASS WITH ADVISORIES`. It selects `signer_binding_not_accepted`: no immutable expected
Developer ID Application signer/certificate/public-key binding exists in the
reviewed repository. No candidate or successor is **Ready**; all operational
work remains **Blocked**.

## D-107 opaque identity-reference decision

The documentation-only D-109 decision is **Completed** with
`PASS WITH ADVISORIES`. Its closed result is
`reference_issuance_not_accepted`: the reviewed repository has no
application-owned issuer for a no-input, attempt-bound opaque signing-identity
reference without lookup, enumeration, selection, fallback, or ambient/default
authority. Object opacity is not provenance.

D-107 remains 8 documented / 11 unproved and D-108's additive prospective
interpretation remains 9/10. No candidate or successor is **Ready**; P3, P4,
signing, V0-3, and all operational work remain **Blocked**.

## D-102 non-build applicability decision

The documentation-only D-108 decision is **Completed** with `PASS WITH
ADVISORIES`. It accepts an applicability split only for D-102's build-child
subject and only for the exact
frozen D-107 no-build/no-candidate-launched-process/no-artifact/no-application-
or-Rust-dependency-filesystem-network-IPC/dynamic-code concept. It does not
implement or admit that candidate. Any build, candidate-launched process,
artifact, application- or Rust-dependency-authored filesystem/network/socket/
IPC API, application- or Rust-dependency-selected dynamic/JIT/plugin/external-
code load, product-signing, or caller-selected authority makes D-102 fully
mandatory. Ambiguity or drift records `boundary_failed` and also makes D-102
mandatory.

Historical D-107 remains 8 documented / 11 unproved; the additive current
interpretation is 9/10. Platform effects and nine other contracts remain
unproved, so the total still contains ten blockers. No successor is **Ready**;
P3, P4, signing, V0-3, and all operational work remain **Blocked**.

## Completed in-process key-use containment classification

The documentation-only source review records D-107's bounded negative result:
`not_eligible_or_unproven`, with eight documented and eleven unproved rows. The
candidate is childless and fileless but is neither an implemented primitive nor
a D-102 waiver. Exact identity provenance, prompt-free bounded key use, cleanup,
platform effects, and the applicability split remain unresolved. No successor
is **Ready**; all operational P3/P4/signing/product work remains **Blocked**.
The documentation increment is **Complete** with `PASS WITH ADVISORIES`.

## Completed codeless signing-fixture classification

The documentation-only classification is **Complete** with `PASS WITH
ADVISORIES` as a bounded negative result. D-106 records
`not_eligible_or_unproven`: five rows are documented and eight remain unproved.
The candidate is not an implemented containment primitive and cannot establish
product signing, hardened runtime, distribution, custody, or V0-3 readiness.
No successor is **Ready**; all operational P3/P4/signing/product work remains
**Blocked**.

## Completed P3 App Sandbox containment re-review

The documentation-only re-review is **Complete** as a bounded negative result.
D-105 selects no eligible candidate after the D-104 re-review: all 22 D-102
contracts remain `contract_unproven`, all ten P3-3 source checks remain
`not_run`, and no operational proof exists. P3-3 through P3-5, P4, signing,
V0-3, and every product or external action remain **Blocked**. No successor is
**Ready**.

## Completed P3 containment bootstrap-trust decision

The owner-approved documentation-only bootstrap-trust decision is **Complete**
with `PASS WITH ADVISORIES`.
It evaluates only whether a future identity-free ad-hoc sandbox-activation seal
is categorically distinct from P4's Developer ID signer binding. It neither
selects a primitive nor relaxes D-102's containment contracts. P3-3 through
P3-5, P4, signing, V0-3, and every operational successor remain **Blocked**.

## Completed P3-2 containment primitive selection

P3-2 is **Complete** with `PASS WITH ADVISORIES` as a documentation-only
bounded negative selection. D-103 finds no eligible candidate in the frozen
reviewed set without claiming universal impossibility. No primitive or
operational proof exists. P3-3, P3-4, P3-5, P4, signing, V0-3, and every
product/external action remain **Blocked**. No successor is **Ready**.

## Completed build-child-containment planning

P3 build-child-containment planning is **Complete** with `PASS WITH
ADVISORIES` as a documentation-only milestone. It defines the future required
policy and blocked evidence path, selects no target-Mac primitive, and runs no
build or process. P3 operational proof, P4 signer binding, V0-3, and every
Apple, Keychain, signing, external, or product action remain **Blocked**.

## Completed account-directory planning

P2 account-directory boundary planning is **Complete** with
`PASS WITH ADVISORIES` as a documentation-only milestone. It defines an
application-resolution policy and proof gates, but selects no platform API,
accepts no OS-internal residual effect, leaves the historical finding Pending,
and advances no operational milestone. P3 build-child containment, P4 signer
binding, V0-3, and every operational/external action remain **Blocked**.

## Completed evidence-privacy planning

P1 evidence-privacy protocol planning is **Complete** with
`PASS WITH ADVISORIES` as a documentation-only milestone. It defines a closed
future evidence vocabulary and source-minimization rule, but it did not collect
evidence, implement a parser/sanitizer, change product capability, or advance an
operational milestone. P2
account-directory, P3 build-child containment, P4 immutable signer binding,
V0-3, and every external operation remain **Blocked**.

## Completed documentation planning

The owner-approved
`personal-assistant-v0-signing-security-prerequisite-planning` increment is
complete with `PASS WITH ADVISORIES` under valid D-098 schema-v3 lineage. It is
documentation-only and cannot change product milestones. It maps separately approvable future privacy,
account-directory, build-child-containment, and immutable signer-contract
prerequisites while preserving D-097 Failed/Pending/Not-run evidence. No
operational signing or product successor is Ready.

## Product milestones

The owner-authorized D-098 terminal-failed successor-disposition recovery has a
tracked pre-disposition evidence freeze from synchronized baseline
`a417e5f1c1c602b917ca27c65af71480e3db6a45`. It is repository governance only:
one proposed argument-free, one-shot disposition may add bounded schema-v3
lineage from the published D-097 failure to
`personal-assistant-v0-signing-security-prerequisite-planning`. It preserves
the original `failed` / `FAIL` / `Blocked` result and no completion marker, and
it carries the screenshot/privacy and Pending Open Directory evidence only to
that exact documentation target. The tracked report necessarily records the
later transition as Not run; only ignored schema-v3 state plus redacted
`status` can evidence post-freeze admission. No successor begins automatically;
the later separately owner-approved exact successor is active as recorded in
the preceding documentation-planning section.
Build containment, operational signing, V0-3, and all Apple, Keychain,
external, or product work remain Blocked.

V0-1 is published at `dca584e`; V0-2 is published through PR #81 at `1513bd8`
with `PASS WITH ADVISORIES` from baseline `8e382e8`. No successor is Ready.

D-095's documentation-only Xcode-managed Developer ID recovery plan is complete
from baseline 9a48b25. A later separately approved execution from baseline
`0931df6` created one Xcode-listed Developer ID Application certificate record,
but sanitized CLI checks found no usable signing identity. The owner later
confirmed that Keychain Access shows the certificate with a private key beneath
it. Local pairing is observed, and one later separately approved exact scoped
check returned `passed_one_label_matched_valid_codesigning_identity`. Current
identity visibility is therefore established, but the historical non-export
criterion cannot Pass as written. D-096 now accepts a truthful prospective
evidence standard, while the signed build is Not run and the historical
evidence-privacy requirement remains Failed. The owner reported no authorization
prompt and no visible state change. The execution remains `FAIL`; D-076/TS-017's
historical cause, V0-3, and every later source increment remain Blocked.
D-097 now represents that result as a valid checkout-local terminal failed
record with no completion marker. Its readiness remains Blocked, so the roadmap
order and product capability are unchanged.

The owner authorized and approved the documentation-only evidence milestone in the
[present-use/local-signing plan](docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md)
after review found that the selected pairing, identity-list, and signature
evidence cannot prove historical absence of export or exclusive custody and
leaves the current extractability attribute unqueried/`not_proven`.
D-096 accepts the evidence-standard documentation reconciliation; its exact
sanitizer, one disposable local signature, and every later product step remain
Blocked and Not run. The documentation decision starts no new gate and changes
no milestone capability.

Post-gate review additionally found an undisclosed historical
`getpwuid`/`opendirectoryd` directory-service boundary, a missing terminal-
failed gate representation, and no selected containment for executable npm/
Cargo/Tauri build-script effects outside configured roots. D-097 resolves only
the representation gap. The Pending directory boundary, current Blocked failed
readiness, and build containment keep every successor and operational signing
step Blocked; they do not change current product capability.

| Milestone                                                             | Status                 | Verified scope                                                                                                                                                                  | Remaining gate                                                                                                                                  |
| --------------------------------------------------------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Phase 1 - runnable foundation                                         | Completed              | Tauri/React shell, toolchain, repository workflow                                                                                                                               | None                                                                                                                                            |
| Phase 2 - local application foundation                                | Completed              | Rust interfaces, SQLite bootstrap, macOS lifecycle, React shell, deterministic mock loop, integration hardening                                                                 | None                                                                                                                                            |
| Phase 3 - bounded mock product loop                                   | Completed              | Conversations, context provenance, simulated results, bounded final answer                                                                                                      | None                                                                                                                                            |
| Phase 4 - trusted proposal and approval boundaries                    | Completed through 4U   | Closed gateway protocol and request, strict schemas, deterministic policy, exact approvals, native source boundary, cancellation, typed in-memory approval audit                | Live transport, execution, and durable audit intentionally absent                                                                               |
| Increment 4V - terminal approval-audit binding                        | Completed and merged   | Exact 19-path scope passed local and hosted verification and was squash-merged through PR #23 at `6e6f91d`                                                                      | Preserve verified boundaries; no later remediation is Ready                                                                                     |
| Personal Assistant text-only v0                                       | Active; V0-2 published | V0-1 seals the fixed synthetic empty-tool request/runtime; V0-2 adds a volatile bounded session journal, deadline, cancellation, and presentation owner without transport or UI | V0-3 is blocked by D-076/TS-017 and all external, Tauri, ZDR, provider, and real-content gates remain blocked                                   |
| Phase 5 - end-to-end policy, approval, audit, and restricted dispatch | Blocked                | Some transport-free primitives were completed in Phase 4                                                                                                                        | Current identity/provider/hosting/ZDR decisions plus separately approved coordinator, durable audit, dispatch, execution, and failure semantics |
| Phase 6 - basic macOS tools                                           | Future                 | Two strict schemas exist without implementations                                                                                                                                | Approve narrow adapters, permissions, tests, and rollback per tool                                                                              |
| Phase 7 - permissions and onboarding                                  | Future                 | Status-only Permission Center exists                                                                                                                                            | Approve request flows, disclosure, revocation, and onboarding                                                                                   |
| Phase 8 - memory and tasks                                            | Blocked                | Volatile mock conversations/tasks only; SQLite bootstrap exists                                                                                                                 | Resolve ARB-005 through approved repositories, encryption, retention, review/delete, and recovery increments                                    |
| Phase 9 - adversarial security validation                             | Future                 | Per-increment security review exists                                                                                                                                            | Complete threat model, abuse tests, red-team cases, and remediation                                                                             |
| Phase 10 - production release                                         | Blocked                | Development and no-bundle builds verified; provisional claims stop at macOS 14+ on Apple Silicon                                                                                | Resolve O-003, O-008, O-009, signing, notarization, installer, update, support, and rollback                                                    |

### Phase 4 acceptance boundary

Phase 4 completion means the transport-free proposal-to-terminal-approval
boundaries through Increment 4U are verified. It does not mean a live provider,
gateway, coordinator, tool implementation, dispatch, execution, product audit,
or user-facing native approval flow exists.

D-060 separates pluggable identity-provider support, Azure-first portable cloud
hosting, and future trusted AI model-provider support. D-062 selects Microsoft
personal identity as the sole Phase 1 provider while leaving exact registration
evidence and all implementation separately controlled. Google and Apple are
deferred. D-066 superseded D-063's Azure synthetic candidate by selecting
OpenAI for the internal synthetic demo, and D-067 selected Cloudflare Workers
only for that same demo. Neither decision authorizes real personal content.
D-061 accepts O-007's product policy; each AI provider still requires
independent ZDR, data-use, logging, region, and security evidence.

D-059 records the current High-severity disposition. It authorizes no missing
capability: ARB-003, ARB-004, ARB-005, and ARB-008 remain blocked as future
capability work. ARB-006 remains High and is non-blocking only while the
repository remains private and all rights reserved; its trigger is public
distribution or external contributions. ARB-007 remains High and is
non-blocking only for unsigned local development; its trigger is release
candidate or public distribution work.

### Product rollout targets

| State             | Target users and account model                                                                                                     | Identity and control boundary                                                                                                                                                                                             |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Current           | Local deterministic engineering proof; no account or external processing                                                           | No gateway, networking, identity provider, credential path, or transmission                                                                                                                                               |
| Private v0 target | One owner using one volatile foreground Personal Assistant text request at a time                                                  | Topology unselected; requires separate synthetic proof, real owner auth and provider/hosting decision, D-061 ZDR/disclosure, and target-Mac gates                                                                         |
| Phase 1 target    | Individual consumers, consultants, IT professionals, small-business owners, and professional power users using personal workspaces | Microsoft personal identity through provider-neutral system-browser OAuth/OIDC with PKCE S256; no workforce tenants, persistent session, automatic email linking, enterprise administration, SCIM, or organization policy |
| Phase 2 target    | Business and enterprise organization accounts and team workspaces                                                                  | Entra workforce SSO, tenant-aware authorization, RBAC, group controls, administration, policy, and audit; SAML, SCIM, and other enterprise providers remain demand-driven                                                 |

The private v0 and both target phases retain D-061's verified-ZDR,
data-minimization, disclosure, content-logging prohibition, and seven-day
operational-metadata boundary. The
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

| Phase                                          | Status              | Bounded outcome                                                                                                                                                                                                                                        | Gate to advance                                                                                           |
| ---------------------------------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| 1. Agent definition and registry               | Completed           | Nine immutable definitions and deterministic registry; only Personal Assistant and Research were `Initial` at that baseline, none operational                                                                                                          | Preserve the verified published catalog                                                                   |
| 2-3. Task orchestration and first bounded flow | Completed           | Closed task lifecycle, trusted context, depth-one typed delegation, one deterministic Research child/result, Personal synthesis, and cancellation above `AgentRuntime`                                                                                 | D-083, verified catalog foundation, deterministic contracts, full validation, and post-increment evidence |
| 4. Agent-specific governance                   | Completed           | D-084's non-executing per-agent identity, profile, policy, approval, delegation-matrix, and volatile audit foundation; published at `2687294`                                                                                                          | Preserve the verified boundary                                                                            |
| 5. Agent-specific memory                       | Completed           | D-085's bounded volatile memory, selected `.txt`/`.md` reader, and direct Personal-to-Knowledge task are published at `5e53f55`; durable memory remains separate                                                                                       | Preserve the verified boundary; ARB-005 still blocks durable storage                                      |
| 5A. Sequential Research/Knowledge workflow     | Completed           | D-086's exact fixture-only Personal-to-Research-to-Knowledge-to-Personal sequence passed its bounded implementation gate and is published at `3efd2c1`                                                                                                 | Preserve the sealed sequential boundary                                                                   |
| 6. Bounded parallelism                         | Completed           | D-091's sealed fixture-only/no-I/O same-thread selector is verified complete with finite limits, cancellation, failure, ordering, and cleanup                                                                                                          | Preserve the verified boundary; active phase 9 may not widen it                                           |
| 7. Staged specialist workflows                 | Completed           | D-087 Engineering, D-088 Cloud/Systems, and D-090 proposal-only Automation verified; D-089 published at `140f05b`                                                                                                                                      | Preserve the sealed proposal-only/no-I/O limits; D-091 phase-6 closeout is separate                       |
| 8. Desktop UI                                  | Prototype validated | Deterministic frontend-only Command Center fixture projection passed the approved M5 browser/Tauri matrix. One separately labelled sealed Research/Knowledge lifecycle panel uses fixed no-input IPC; generic catalog/task/progress UI remains Blocked | Preserve the verified prototype and narrow exception                                                      |
| 9. End-to-end demonstrations                   | Completed           | Focused checks, 447/447 acceptance, full verification, independent review, and a valid marker; Demo 7's two branches remain separate                                                                                                                   | Preserve no bridge and the advisory                                                                       |
| 10. Architecture/security review               | Completed           | Source-current cross-phase review at `181f851` found no Critical, High, Medium, or Low current defect; historical F-01/F-02, F-07, F-08, F-12, and F-15 are reconciled                                                                                 | Preserve the report; future remediation requires a separate approved plan                                 |

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

The owner-approved transport-free
[`V0-2 volatile lifecycle and presentation journal`](docs/plans/2026-08-28-personal-assistant-v0-session-host.md)
is locally verified complete from synchronized baseline `8e382e8`. It adds only
Rust-issued presentation correlation, bounded volatile snapshots/updates,
deterministic monotonic deadlines, resumable cancellation cleanup, restart, and
late-event rejection. Its success/failure/stream results remain fixture-only;
there is no response ingress, UI, IPC, provider, network, credential,
persistence, tool, filesystem, or device effect. Reviewed head `7fecf03`
squash-merged through PR #81 to `main` at `1513bd8` after all six required
checks passed.

The [`Personal Assistant v0 program`](docs/plans/2026-08-28-personal-assistant-v0-program.md)
still records D-094's full sequence. V0-3 remains Blocked by D-076/TS-017 and a
separately accepted signed-identity restart. V0-4 through V0-14 remain Blocked
or unselected across identity, JWT/JWKS, no-traffic
provisioning, HTTPS, credential handoff, authentication/provider traffic,
disclosure-bound Tauri, ZDR, and real-content decisions. No successor
implementation is Ready.

The owner-approved sealed Research -> Knowledge connected presentation is
published through PR #71 at `d9c7c13`. It is a visibly simulated,
process-local exception: one selected-scenario prop-free panel invokes only
four no-input lifecycle commands, and command responses alone may commit its
closed content-free state. The Command Center fixture graph, read-only
projection, Conversations mock, and Rust acceptance workflows remain separate
deterministic proofs. Exact PR-head and merged-main Documentation and CI
workflows pass. This does not make the catalog/task/progress UI authoritative
or permit general agent IPC, provider, tool, approval, persistence, filesystem,
network, or device work. No successor source plan is owner-selected or Ready.
The verified-complete documentation-only
[`native-multi-agent-final-review-planning`](docs/increments/native-multi-agent-final-review-planning.md)
and separately approved read-only
[`native-multi-agent-final-review`](docs/increments/native-multi-agent-final-review.md)
complete the source-current architecture/security review at `181f851`. Its
advisories do not authorize source work or remediation.
The review documentation is published through
[PR #74](https://github.com/SillyRbbit/ai-agent-assistant/pull/74) at `main`
commit `d3edc7a`; both Documentation workflow runs passed.

The completed owner-selected overlay
[`PR #57 transitive advisory remediation`](docs/plans/2026-08-25-pr57-transitive-advisory-remediation.md)
is verified complete with advisories under the complete, valid gate
`pr57-transitive-advisory-remediation`. It changes only six development-only
transitive lockfile entries within existing parent ranges.
The exact resolver diff, scripts-disabled clean install, zero audits, and full
local verification pass without a manifest or product change. Exact remediation
`c3cc49e` also passes every classifier-selected PR check. The result is `PASS
WITH ADVISORIES` solely because no next implementation plan is owner-selected
or Ready. The deterministic marker is complete and valid. Exact closeout head
`3a0ee66` passed its documentation workflow, PR #57 squash-merged at `3987387`,
and merged-main Documentation and CI runs pass. The ordered architecture queue
below is otherwise unchanged.

1. Treat
   [`2026-08-11-multi-agent-end-to-end-demonstrations.md`](docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md)
   without adding production behavior or bridging Demo 7. Final focused checks,
   447/447 acceptance, and full verification pass on macOS 26.6 build 25G72
   arm64. The owner accepts checkpoint denial and safe manual A-D dispatch as
   separate demonstrations; the absent combined chain remains an advisory.
   Closeout passes with advisories and the deterministic marker is valid. No
   further work is authorized by completion.
2. Treat D-086's exact
   [`2026-08-11-research-knowledge-workflow.md`](docs/plans/2026-08-11-research-knowledge-workflow.md)
   as verified complete with advisories and published at `3efd2c1`.
3. Treat D-087's fixture-only, proposal-only
   [`2026-08-11-engineering-quality-workflow.md`](docs/plans/2026-08-11-engineering-quality-workflow.md).
   Its implementation and closeout are complete with `PASS WITH ADVISORIES`
   and published at `a5d7ba1`.
4. D-088's exact
   [`2026-08-11-infrastructure-systems-operations-workflow.md`](docs/plans/2026-08-11-infrastructure-systems-operations-workflow.md)
   is verified complete with advisories. Do not begin another increment.
5. Preserve D-090's exact
   [`2026-08-11-workflow-automation.md`](docs/plans/2026-08-11-workflow-automation.md)
   verified proposal/manual-dispatch boundary. Its complete valid marker and
   `PASS WITH ADVISORIES` review are final.
6. Treat D-091's exact
   [`2026-08-11-bounded-agent-parallelism.md`](docs/plans/2026-08-11-bounded-agent-parallelism.md)
   as verified complete with advisories under its complete, valid gate. Its same-thread fixture
   proof does not permit a runtime/provider/thread/app-global/general-engine
   change; the active phase-9 evidence increment must preserve that boundary.
7. Keep durable memory/ARB-005, live research/retrieval,
   executable automation, provider, IPC, UI, every infrastructure/operations tool or live
   access path, repository effects, and device effects Blocked. Runtime tool
   proposals remain rejected and Native remains sole/default.
8. Do not begin Stage B no-traffic provisioning, Stage C synthetic transport,
   Stage D real-content activation, or another High remediation automatically.
9. Later work must collect D-062's exact Microsoft registration and token
   evidence, D-063's exact Azure deployment evidence, and D-061 provider,
   disclosure, retention, and operational evidence under separate plans.
10. Keep ARB-003, ARB-004, ARB-005, and ARB-008 blocked until separately
    approved capability increments are selected.
11. Revisit ARB-006 only before public distribution or external contributions,
    and ARB-007 only before release-candidate or public-distribution work.
12. Do not add transport, credentials, execution, persistence, enterprise
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
is verified complete with advisories under a complete, valid gate. The
deterministic end-to-end demonstration increment is verified complete with
advisories under a valid marker. Both PR remediations are complete and valid;
no new implementation plan is Ready.

## Rollback and reprioritization

A milestone may be reordered only by explicit project-owner direction and a
corresponding update to `NEXT_STEPS.md`, `PLANS.md`, and any durable decision.
Failed verification returns the active increment to Active or Blocked; it does
not create completion evidence. Rollback reverts only the bounded increment and
must preserve historical reports and decisions.
