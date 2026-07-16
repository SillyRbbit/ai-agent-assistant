# Product readiness audit

Date: 2026-07-16
Audit: Meta Increment 6 - Product Readiness Audit
Repository baseline: `main` at `6b149fa`
Reviewer: Codex `$readiness-review`
Result: **NOT READY**
Composite score: **57 / 100**

## Executive conclusion

Cortexa is a disciplined pre-production foundation, not a deployable assistant.
The repository has strong local trust-boundary design, strict Rust and
TypeScript verification, a polished mocked shell, deterministic gateway and
approval contracts, and unusually complete engineering documentation for its
stage. The full repository quality gate passes.

The product does not yet provide one production end-to-end assistant workflow.
There is no authenticated gateway transport, credential-owning production
coordinator, frontend-to-core run path, restricted tool executor, durable audit,
product-data persistence, signed installer, or production support boundary.
The current application demonstrates mocked behavior while the Rust security
contracts remain disconnected from the shipping UI. Those gaps make the
project **NOT READY** for a controlled enterprise pilot or production release.

No Critical finding exists in the currently reachable application because the
high-risk capabilities are absent rather than exposed unsafely. Eight High
findings block live traffic, real side effects, enterprise pilot use, or
release. Passing tests do not lower those capability blockers.

## Audit method

The audit used this evidence order:

1. Source, tests, configuration, lockfiles, migrations, and Git state.
2. Successful local and hosted verification evidence.
3. Accepted architecture and security decisions.
4. Current-state documentation only where it matched items 1 through 3.
5. Plans and executive intent as future direction, never as implemented proof.

Scores measure current maturity, not percentage of roadmap work completed and
not certification against an external standard. The composite is the rounded
arithmetic mean of the 16 category scores. A blocking prerequisite overrides
the composite score.

## Baseline evidence

- Git was clean and synchronized before audit edits:
  `## main...origin/main` at `6b149fa`.
- The previous `meta-05` completion marker remained complete and valid with
  `PASS WITH ADVISORIES`.
- The repository contains 339 tracked files, 81 application source files, and
  14,456 TypeScript, CSS, and Rust source lines. Git has 51 commits, no tags,
  and one maintainer identity represented by two email forms.
- The only custom Tauri command is `get_app_info`; the main capability grants
  only `core:default`.
- `npm run verify` passed formatting, repository health, lint, strict Clippy,
  hook tests, health-script tests, frontend tests, Rust tests, TypeScript,
  frontend production builds, and a Tauri release no-bundle build.
- The verified counts were 124 frontend tests, 95 Rust library tests, 21 Rust
  integration tests, 28 hook tests, and 16 repository-health tests.
- `npm audit --audit-level=low` reported zero npm vulnerabilities.
- Live `cargo-audit 0.22.2` reported the accepted but unresolved D-025/D-046
  baseline: two `quick-xml 0.39.4` vulnerabilities and 18 warning-class
  advisories.
- The Vite artifact reported by verification was approximately 229 KiB of
  JavaScript and 21 KiB of CSS before gzip. The no-bundle release executable is
  approximately 12 MiB. These are size observations, not performance results.
- No native application walkthrough, accessibility audit, performance test,
  load test, signed bundle, installer, upgrade, rollback, or disaster-recovery
  exercise was run for this documentation-only audit.

## Score summary

| Category             | Score | Blocking status                                                         |
| -------------------- | ----: | ----------------------------------------------------------------------- |
| Product completeness |    28 | Blocks pilot and production                                             |
| Architecture         |    82 | Blocks live integration until open boundaries are decided               |
| Security             |    72 | Blocks live traffic and release                                         |
| Privacy              |    76 | Blocks live provider traffic until retention and disclosure are decided |
| User experience      |    58 | Blocks representative pilot validation                                  |
| Accessibility        |    48 | Blocks inclusive pilot and release claims                               |
| Reliability          |    62 | Blocks production operation                                             |
| Performance          |    45 | Blocks production service-level claims                                  |
| Scalability          |    38 | Blocks enterprise-scale claims                                          |
| Test coverage        |    74 | Blocks production confidence, not the next bounded contract increment   |
| Maintainability      |    80 | Conditional; advisory debt remains                                      |
| Documentation        |    86 | Does not block the next bounded product increment                       |
| Developer experience |    82 | Does not block the next bounded product increment                       |
| Release readiness    |    18 | Blocks every production release                                         |
| Executive readiness  |    42 | Blocks an evidence-backed investment or pilot claim                     |
| Enterprise readiness |    20 | Blocks enterprise deployment                                            |

## Category assessments

### Product completeness - 28 / 100

**Evidence:** The React shell under `src/` explicitly uses a deterministic mock
run driver. `src-tauri/src/lib.rs` initializes storage and menu/window lifecycle
but does not connect `InitialGatewayTurn` to Tauri IPC. Tasks, Memory,
Integrations, permissions, and Settings remain placeholder or informational
surfaces. Only `get_app_info` crosses IPC.

**Strengths:** The mock loop demonstrates conversation, provenance, simulated
tool output, approval choices, cancellation, retry, and activity states without
misrepresenting them as real actions. Rust contains bounded contracts for the
first gateway turn, local schema validation, policy, approval, and cancellation.

**Gaps:** There is no live model path, gateway client, coordinator, tool
executor, durable product state, real task creation, integration, or complete
user workflow.

**Risks:** Stakeholders could confuse a high-fidelity demonstration with an
operational assistant. Product learning from the mock may not predict live
latency, failures, provider behavior, or approval ergonomics.

**Recommended remediation:** Finish the exact terminal audit binding, decide
the production coordinator and gateway boundaries, then implement one
restricted end-to-end workflow in separately verified increments.

**Blocking status:** High. Blocks controlled pilot and production readiness.

### Architecture - 82 / 100

**Evidence:** `ARCHITECTURE.md`, `src-tauri/src/agent`,
`src-tauri/src/tools`, `src-tauri/src/policy`, `src-tauri/src/approvals`, and
`src-tauri/src/audit` agree on a local trusted core and untrusted model. Closed
types, exact schemas, private ownership, bounded data, and one narrow IPC
command are tested. O-002, O-006, and O-007 remain open.

**Strengths:** Trust boundaries are explicit. The model cannot authorize or
execute. Provider output is normalized, schema-validated, policy-bound, and
approval-bound in portable Rust. Generic execution and broad platform
scaffolds were removed.

**Gaps:** The architecture has no production orchestration composition root,
gateway transport/authentication implementation, executor boundary, durable
audit design, or product persistence model. Current modules prove contracts in
isolation rather than a running system.

**Risks:** Integrating several well-tested contracts at once could create new
ownership, cancellation, error, and audit gaps. Premature crate modularization
could increase coupling before the runtime path is understood.

**Recommended remediation:** Preserve the current single-crate boundary while
adding one composition step at a time. Resolve O-006 and O-007 before live
networking and record durable audit/executor decisions before implementation.

**Blocking status:** High for live integration; not a blocker to Increment 4V.

### Security - 72 / 100

**Evidence:** The Tauri capability grants `core:default`; `get_app_info` is the
only custom command. Rust denies unsafe code and panic-style production paths.
Gateway events, tool arguments, policy facts, approval identities, limits,
cancellation, and redacted errors have negative-path coverage. The lockfile
retains two known `quick-xml` vulnerabilities and 18 warning advisories.

**Strengths:** No production API key, OAuth token, broad filesystem permission,
shell execution, accessibility permission, or autonomous destructive action is
implemented. Model output remains outside every authorization boundary.

**Gaps:** Terminal approval audit is not yet owned by the initial turn. No live
credential storage, gateway authentication, restricted executor, production
audit sink, security telemetry, or incident path exists. CSP still allows
inline scripts and styles. Accepted RustSec findings are unresolved.

**Risks:** A future integration could return an approval resolution without a
record, mishandle gateway identity, or broaden IPC/execution authority. Known
dependency issues may become reachable as native flows expand.

**Recommended remediation:** Implement 4V first, then threat-model and approve
gateway identity, credential storage, transport, executor, audit durability,
and CSP hardening as separate increments. Remediate or replace vulnerable and
unmaintained dependencies before release.

**Blocking status:** High for live traffic, real actions, and production
release.

### Privacy - 76 / 100

**Evidence:** The current product performs no model network request, stores no
conversation/task/memory data, and redacts mock activity and Rust debug/error
surfaces. SQLite stores only migration metadata and `app_initialized`.
Production release mode uses an in-memory database. O-007 is unresolved.

**Strengths:** Data minimization is real today. Secrets and raw tool results are
excluded from SQLite and audit contracts. Provider requests are designed for
`store: false`, bounded content, and no parallel calls.

**Gaps:** There is no approved provider retention mode or user disclosure, data
inventory, durable retention/deletion/export policy, telemetry policy,
enterprise data-processing boundary, or encryption/key-management design for
future product data.

**Risks:** Adding live provider traffic or persistence before these decisions
could create undisclosed retention, excessive logging, or unclear enterprise
data ownership.

**Recommended remediation:** Resolve O-007 before live traffic. Define a
purpose-limited data inventory and lifecycle before adding durable audit,
conversation, task, or memory storage.

**Blocking status:** High for live provider traffic and persistent pilot data;
not a blocker to transport-free 4V.

### User experience - 58 / 100

**Evidence:** The React shell provides navigation, conversation states,
streaming simulation, stop/retry, approval choices, activity, responsive
styles, and dark mode with 124 passing tests. Several routes and settings are
explicit placeholders. The mock approval modal and native approval source are
not connected to a real run.

**Strengths:** The main workflow is coherent, branded, responsive at the Tauri
minimum size, and honest about mock behavior. Error and empty states are
present.

**Gaps:** No real workflow can be completed. Tasks, Memory, Integrations, and
Permission Center do not manage product state. Live latency, offline behavior,
recovery, notification, and long-running action UX are untested.

**Risks:** UX decisions validated against deterministic timing may fail under
real gateway latency and partial failure. Placeholder breadth can overstate
product maturity.

**Recommended remediation:** Validate one narrow live workflow before expanding
navigation. Add end-to-end states for transport, cancellation, approval,
execution result, audit receipt, and recovery.

**Blocking status:** High for a representative pilot; not a blocker to the
next transport-free contract increment.

### Accessibility - 48 / 100

**Evidence:** The frontend uses semantic landmarks, labels, accessible names,
live regions, visible focus styles, and reduced-motion CSS. Tests often query by
role and accessible name. No automated accessibility scanner, contrast report,
VoiceOver run, or keyboard-only end-to-end evidence exists.

**Strengths:** Accessible semantics and motion preferences are considered in
the baseline rather than deferred entirely.

**Gaps:** The React mock approval modal has no verified focus trap, initial
focus, Escape handling, or focus restoration. The native approval dialog has a
documented accepted limitation: Escape does nothing and a window close control
is unavailable. Small text and contrast have not been quantitatively reviewed.

**Risks:** Keyboard and assistive-technology users may become trapped or lose
context at the most security-sensitive interaction.

**Recommended remediation:** Split remediation into frontend dialog focus and
keyboard behavior, native approval replacement/acceptance, and a target-Mac
VoiceOver/contrast/manual matrix. Add automated accessibility checks only after
tool selection is separately reviewed.

**Blocking status:** Medium. Blocks accessibility claims and should block a
broad employee pilot until the approval path is usable.

### Reliability - 62 / 100

**Evidence:** Deterministic contracts test malformed frames, limits, duplicate
keys, identity mismatches, cancellation, late events, expiry, replay, storage
migration rollback, and menu lifecycle. Complete local verification passes.

**Strengths:** Failure behavior is typed, bounded, fail-closed, and extensively
tested within implemented modules. SQLite setup is transactional and verifies
foreign keys, journal mode, timeout, migration order, and checksums.

**Gaps:** There is no live transport, retry orchestration, process recovery,
durable run state, crash recovery, backup/restore, corruption exercise,
installer rollback, telemetry, service-level objective, or fault-injection
suite.

**Risks:** Module-level correctness does not establish recovery across app,
gateway, model, approval dialog, executor, and persistence failures.

**Recommended remediation:** Define the production run state machine and
failure ownership before wiring transport. Add deterministic integration and
recovery tests with each runtime boundary.

**Blocking status:** High for production operation; not a blocker to 4V.

### Performance - 45 / 100

**Evidence:** Request, event, output, argument, count, and timeout limits are
encoded in Rust. The current frontend bundle and native executable are modest
in size. No benchmark, startup measurement, memory profile, interaction timing,
gateway latency, or load result exists.

**Strengths:** Bounded protocol sizes and event counts reduce obvious
unbounded-work risk. The mock UI is small and deterministic.

**Gaps:** No performance budget or target percentile exists. Real model,
network, database, approval, and execution paths are absent, so observed build
sizes do not predict user-perceived performance.

**Risks:** Live response latency, event rendering, memory growth, or database
contention could invalidate the current interaction design.

**Recommended remediation:** Define startup, first-response, stream-render,
approval, action, memory, and CPU budgets when the first live vertical slice is
designed; measure before optimizing.

**Blocking status:** Medium for production claims; no blocker to a bounded
contract increment.

### Scalability - 38 / 100

**Evidence:** The current app is a single local process. Approval and audit
adapters are bounded in-memory collections capped at 1,024 records/subjects.
The gateway protocol bounds a single initial workflow. No gateway deployment,
multi-user tenancy, synchronization, or enterprise fleet design exists.

**Strengths:** Local-first operation and explicit limits provide predictable
single-user behavior and avoid premature distributed-system claims.

**Gaps:** There is no capacity model for gateway concurrency, tenant isolation,
fleet configuration, policy distribution, audit aggregation, large histories,
or integration rate limits.

**Risks:** Enterprise requirements may force redesign if identity, tenancy,
retention, and policy distribution are deferred until after transport wiring.

**Recommended remediation:** Resolve gateway deployment and identity before
live networking. Keep local product increments single-user until a measured
pilot establishes scaling requirements.

**Blocking status:** High for enterprise-scale claims; not a blocker to a
single-user controlled prototype.

### Test coverage - 74 / 100

**Evidence:** Verification passes 124 frontend, 95 Rust library, 21 Rust
integration, 28 hook, and 16 repository-health tests. Tests emphasize closed
contracts and negative paths. No coverage percentage or threshold is produced.

**Strengths:** Security-sensitive Rust contracts have strong adversarial and
state-transition coverage. Repository automation is regression-tested. Tests
run without production model/network credentials.

**Gaps:** There is no browser end-to-end suite, native UI automation,
accessibility scanner, performance/load suite, production transport contract,
installer test, recovery test, or quantitative coverage report.

**Risks:** A high test count can conceal untested composition, UI focus,
packaging, and recovery boundaries.

**Recommended remediation:** Add tests with each real composition boundary.
Establish coverage measurement as an engineering indicator, not a substitute
for trust-boundary and end-to-end evidence.

**Blocking status:** Medium for production confidence; current evidence is
sufficient for the narrow 4V contract scope.

### Maintainability - 80 / 100

**Evidence:** TypeScript strictness, Clippy denial rules, exact dependencies,
typed errors, focused modules, local skills, CI, repository-health scripts, and
current architecture documentation are enforced by `npm run verify`.

**Strengths:** Ownership boundaries are explicit and generic unused scaffolds
were removed. Dependencies are pinned. Documentation records non-goals and
rollback for bounded increments.

**Gaps:** Known vulnerable/unmaintained transitive dependencies remain. Several
security modules are large because their tests live inline. There is one
maintainer identity, no tagged release history, and repository-process
enforcement beyond hosted checks was not independently confirmed in this audit.

**Risks:** Security contract growth and single-maintainer knowledge could slow
safe integration. Accepted advisory baselines can normalize debt if not time
bounded.

**Recommended remediation:** Schedule dependency remediation before release,
keep integration increments narrow, and add ownership/review redundancy before
enterprise support commitments.

**Blocking status:** Medium advisory; does not block 4V.

### Documentation - 86 / 100

**Evidence:** Root architecture, requirements, roadmap, testing, release,
security, engineering, contribution, brand, workflow, plan, increment, decision,
and review documents are extensive. Internal-link and Markdown checks pass.

**Strengths:** Current, mocked, planned, and prohibited states are usually
distinguished. Trust boundaries, verification commands, non-goals, and release
limitations are explicit.

**Gaps:** Project memory still described merged Meta 5 as unpublished before
this audit. The owner's latest Meta 6 audit assignment collides with the older
Meta 6 icon-plan number. The stopped executive-document request produced no
executive package.

**Risks:** Stale publication and queue state can direct a later session to the
wrong increment. Extensive documentation can obscure the few authoritative
current-state files.

**Recommended remediation:** This audit reconciles publication and records the
number collision without rewriting historical plans. Renumber the icon plan in
a separately approved planning change before implementation.

**Blocking status:** Low after this audit; no blocker to 4V.

### Developer experience - 82 / 100

**Evidence:** The repository provides pinned Node/Rust dependencies, one
canonical `npm run verify`, setup and testing guides, local hooks/skills,
least-privilege CI, issue/PR templates, Dependabot proposals, and deterministic
health scripts.

**Strengths:** A contributor can build, test, lint, and inspect policy locally
without production secrets. CI commands match local commands. Error output is
generally actionable.

**Gaps:** `cargo-audit` is not installed by the repository setup itself; the
workflow installs it. No license supports an open contribution program. There
is no one-command signed local package, coverage report, or development gateway
environment.

**Risks:** The governance system is mature relative to the product and can add
process cost. Environment-specific native packaging remains manual.

**Recommended remediation:** Preserve the single verify command, document
future gateway development only when it exists, and avoid adding process that
does not protect a concrete boundary.

**Blocking status:** Low; does not block the next bounded increment.

### Release readiness - 18 / 100

**Evidence:** `RELEASE_CHECKLIST.md` explicitly states that production release
is not enabled. The audit built only a no-bundle executable. There is no root
license, signing identity, notarization evidence, installer validation,
upgrade/uninstall/rollback exercise, update channel, release tag, SBOM, or
support commitment. O-003 remains open.

**Strengths:** Release gaps are documented rather than hidden. Hardened Runtime
and a macOS 14.0 minimum are configured as current targets. CI is read-only and
does not expose signing secrets.

**Gaps:** Every distribution and legal gate remains open. Production icons are
still separately planned. Dependency advisories remain unresolved.

**Risks:** An unsigned or unnotarized artifact would fail normal enterprise
trust and deployment expectations. The current macOS target may not match the
approved support policy.

**Recommended remediation:** Defer release engineering until one real workflow
passes pilot gates, then separately decide licensing, target OS, signing and
notarization ownership, package validation, updates, rollback, and support.

**Blocking status:** High. Blocks every production release.

### Executive readiness - 42 / 100

**Evidence:** Product requirements, roadmap, architecture, brand guidance, and
README communicate the vision and constraints. No `docs/executive/` package
exists because the prior executive-document request was stopped. No measured
pilot, ROI baseline, workflow outcome, or production capability exists.

**Strengths:** The value proposition and human-control model are clear. The
repository avoids unsupported compliance and production claims.

**Gaps:** There is no current boardroom package, verified before/after workflow,
pilot result, cost model, adoption plan, or operational ownership model.

**Risks:** Executive materials created before a real vertical slice may present
vision as delivery and produce unsupported ROI expectations.

**Recommended remediation:** Postpone a full executive package until one
verified workflow and pilot measurement plan exist. A short factual one-pager
may be prepared only from this audit and current-state evidence.

**Blocking status:** Medium. Blocks evidence-backed investment and pilot claims,
not the next product contract increment.

### Enterprise readiness - 20 / 100

**Evidence:** Security principles, local trust boundaries, policy, approvals,
audit contracts, repository governance, and release checklists exist. There is
no live product workflow, SSO/IdP selection, device/fleet deployment, admin
policy, durable audit, data lifecycle, support model, incident process,
integration, compliance evidence, or production distribution.

**Strengths:** The architecture begins with least privilege, local authority,
human approval, and explicit prohibited actions.

**Gaps:** Nearly every operational enterprise control is planned or undecided.
Repository controls are not product administration controls.

**Risks:** Calling the current application enterprise-ready would overstate
both capability and assurance. Identity, retention, fleet policy, and audit
requirements can materially change the runtime architecture.

**Recommended remediation:** Use a gated single-workflow design partner pilot
as the next milestone, not a production rollout. Obtain security and executive
approval for identity, retention, permissions, audit, deployment, and support
before live enterprise data is introduced.

**Blocking status:** High. Blocks enterprise pilot deployment and production.

## Findings

| ID      | Severity | Finding                                                                                                                               | Blocking scope                                         |
| ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| PRA-001 | High     | No production end-to-end assistant workflow connects UI, trusted core, gateway, approval, execution result, and audit.                | Pilot and production                                   |
| PRA-002 | High     | `InitialGatewayTurn` can return a terminal approval resolution without first recording it in its typed audit adapter.                 | Next security-boundary increment and any real action   |
| PRA-003 | High     | No restricted executor exists for either catalog tool; no product action can be completed.                                            | Functional pilot                                       |
| PRA-004 | High     | Gateway identity, deployment, trusted credential storage, and provider retention/disclosure remain undecided or unimplemented.        | Live provider traffic                                  |
| PRA-005 | High     | Signing, notarization, installer, update, rollback, target-OS approval, and license gates are open.                                   | Production release                                     |
| PRA-006 | Medium   | Two Rust vulnerabilities and 18 warning advisories remain in the accepted dependency baseline.                                        | Release; monitor during development                    |
| PRA-007 | High     | Audit, conversation, task, and memory state are not durable and have no approved lifecycle or recovery model.                         | Durable pilot and enterprise evidence                  |
| PRA-008 | Medium   | Approval dialogs lack complete keyboard, focus, VoiceOver, contrast, and close-path evidence.                                         | Inclusive pilot and accessibility claims               |
| PRA-009 | Medium   | No end-to-end, performance, load, fault-injection, crash-recovery, or installer tests exist.                                          | Production confidence                                  |
| PRA-010 | Medium   | The native approval dialog has an accepted Escape/close limitation and may outlive run cancellation visually.                         | Approval UX and future real actions                    |
| PRA-011 | Medium   | CSP permits inline scripts and styles; current exposure is limited but hardening is unverified.                                       | Release hardening                                      |
| PRA-012 | Low      | Production Tauri icons are not yet generated from the approved brand source.                                                          | Brand-complete packaging only                          |
| PRA-013 | Medium   | One maintainer identity owns the 51-commit history; review redundancy and remote enforcement were not independently established here. | Enterprise support resilience                          |
| PRA-014 | Medium   | No executive package, measured pilot baseline, or ROI evidence exists.                                                                | Executive readiness                                    |
| PRA-015 | High     | SSO/IdP, fleet deployment, admin policy, data governance, compliance evidence, and support/incident operations are absent.            | Enterprise deployment                                  |
| PRA-016 | Low      | Stale Meta 5 publication text and the owner-directed Meta 6 naming collision could misdirect future work.                             | Workflow clarity; publication text fixed by this audit |
| PRA-017 | Medium   | No product telemetry, health diagnostics, operational audit export, alerting, or support runbook exists.                              | Production operation                                   |
| PRA-018 | Low      | No release tags or artifact lineage exist.                                                                                            | Release traceability                                   |

## Ordered remediation backlog

| Finding ID | Description                                          | Severity | Business impact                                    | Technical impact                                               | Recommended fix                                                                                     | Effort | Dependencies                             | Proposed milestone            | Blocks next product increment                         |
| ---------- | ---------------------------------------------------- | -------- | -------------------------------------------------- | -------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | ------ | ---------------------------------------- | ----------------------------- | ----------------------------------------------------- |
| PRA-002    | Terminal resolution is not audit-bound               | High     | Approval evidence can be incomplete                | Successful resolution may escape before audit validation       | Implement the exact two-file Increment 4V plan                                                      | S      | Merged 4U; owner approval                | Phase 4 security closure      | **Yes**; make this the next bounded product increment |
| PRA-004    | Gateway identity and retention boundaries unresolved | High     | Live data use cannot be approved                   | No authenticated transport or credential owner                 | Resolve O-006/O-007 and record a threat model                                                       | M      | Security and executive owner decisions   | Live gateway architecture     | No; follows 4V                                        |
| PRA-001    | No integrated production workflow                    | High     | No real pilot value can be delivered               | UI and trusted Rust contracts are disconnected                 | Split one vertical slice into coordinator, IPC/event bridge, gateway client, and result increments  | XL     | PRA-002, PRA-004                         | Controlled workflow prototype | No; it is a sequence, not one increment               |
| PRA-003    | No restricted tool executor                          | High     | No approved action can complete                    | Catalog tools have validation but no side-effect boundary      | Design and implement one exact executor after authority and audit decisions                         | L      | PRA-001, PRA-002, threat model           | Controlled workflow prototype | No                                                    |
| PRA-007    | No durable product or audit lifecycle                | High     | Sessions and evidence cannot survive restart       | Only bootstrap metadata persists                               | Decide data model, retention, encryption/key ownership, migrations, and recovery before persistence | XL     | O-007, security approval                 | Durable pilot                 | No                                                    |
| PRA-008    | Accessibility evidence incomplete                    | Medium   | Excludes users and weakens procurement readiness   | Modal focus/keyboard and native UX gaps                        | Split frontend dialog, native dialog, and target-Mac audit work                                     | M      | Real approval UX selection               | Pilot hardening               | No                                                    |
| PRA-010    | Native approval close/cancellation limitation        | Medium   | Security prompt can feel stuck or stale            | Synchronous native dialog cannot be programmatically closed    | Replace source or explicitly accept with target-Mac evidence before real actions                    | M      | Security/UX decision                     | Pilot hardening               | No                                                    |
| PRA-006    | Rust advisory baseline unresolved                    | Medium   | Security review and procurement friction           | Vulnerable/unmaintained transitive crates remain               | Upgrade/replace in a dedicated dependency increment and rerun full native matrix                    | M-L    | Upstream compatibility                   | Pre-release hardening         | No                                                    |
| PRA-011    | CSP inline allowances                                | Medium   | Weakens defense-in-depth posture                   | Inline script/style execution remains allowed                  | Test narrower CSP in a dedicated config/security increment                                          | M      | Tauri/Vite runtime validation            | Pre-release hardening         | No                                                    |
| PRA-009    | Composition and non-functional tests absent          | Medium   | Reliability and latency are unknown                | No E2E, recovery, performance, or packaging tests              | Add evidence alongside each live boundary, then establish performance/recovery gates                | L      | PRA-001 implementation                   | Pilot and release gates       | No                                                    |
| PRA-017    | Operational observability absent                     | Medium   | Failures cannot be supported at scale              | No health, alert, export, or incident instrumentation          | Define privacy-preserving diagnostics and support runbook                                           | L      | Data policy and live coordinator         | Durable pilot                 | No                                                    |
| PRA-015    | Enterprise control plane absent                      | High     | Enterprise deployment cannot be governed           | No identity, fleet, admin, compliance, or support architecture | Treat each control family as a milestone after a measured single-workflow pilot                     | XL     | PRA-004, PRA-007, executive sponsorship  | Enterprise pilot              | No                                                    |
| PRA-005    | Release/legal system absent                          | High     | Product cannot be distributed or supported         | No signed/notarized/validated artifact or approved license     | Split license, target OS, signing/notarization, installer/update, rollback, and support work        | XL     | One real workflow; owner/legal decisions | Production release            | No                                                    |
| PRA-014    | Executive evidence package absent                    | Medium   | Funding and adoption decisions lack measured proof | No verified use case, ROI input baseline, or pilot results     | Create factual executive materials after vertical-slice evidence exists                             | M      | PRA-001 and pilot measurement plan       | Pilot sponsorship             | No                                                    |
| PRA-013    | Maintainer concentration                             | Medium   | Continuity and review resilience are weak          | Knowledge and approval concentrate in one identity             | Add reviewed ownership and support redundancy before external commitments                           | M      | Team/owner decision                      | Enterprise operations         | No                                                    |
| PRA-012    | Production icons pending                             | Low      | Packaging is not brand complete                    | Existing icon family is not sourced from canonical asset       | Renumber and separately approve the existing 16-file icon plan                                      | S-M    | Packaging environment                    | Release polish                | No                                                    |
| PRA-018    | No release lineage                                   | Low      | Artifact provenance is incomplete                  | No tags or release hashes                                      | Define version/tag/artifact-record procedure with first candidate release                           | S      | Release process                          | Release candidate             | No                                                    |
| PRA-016    | Queue and publication memory drift                   | Low      | Future sessions may select wrong work              | Current docs conflict with repository state                    | Reconciled here; renumber icon plan before implementation                                           | S      | Owner queue direction                    | Meta 6 closeout               | No                                                    |

Effort estimates are relative: S is one bounded increment, M is several bounded
increments, L is a milestone, and XL requires architecture plus multiple
milestones. They are not calendar estimates.

## Roadmap recommendations

### What should happen next

1. Keep Increment 4V Proposed until the project owner explicitly selects it.
   It is the smallest independently verifiable remediation: two source/test
   files, no network, no persistence, no executor, and no new permission.
2. After 4V, resolve O-006 and O-007 and add decisions for durable audit and the
   exact executor authority boundary before any live gateway or side effect.
3. Build one controlled vertical slice in separate increments: trusted
   coordinator, authenticated gateway client, typed IPC/event bridge, one exact
   tool executor, execution result/audit, and recovery.
4. Define pilot measurements only after that slice exists. Use measured results
   to decide persistence, broader integrations, executive materials, and release
   investment.

### What should be postponed

- Full executive and boardroom collateral until one real workflow and a
  measurable pilot plan exist.
- Production icon rollout until packaging preparation resumes; preserve its
  exact 16-file source scope and require a new live increment number.
- Broad integrations, memory, autonomous behavior, elevated OS permissions,
  fleet administration, and multi-tenant scaling until the first controlled
  workflow proves the trust model.
- Signing, notarization, update distribution, and public release work until the
  product is functionally pilot-ready and legal ownership is decided.

### What should be removed

- Remove the stopped Meta Increment 4 executive-document request from any
  active queue interpretation; retain its historical stopped record.
- Remove no application code as part of this audit. Existing mocked surfaces
  remain useful only while they stay explicitly labeled as mocked or planned.

### What should be split

- Split live assistant integration into gateway identity/retention decisions,
  client transport, coordinator, IPC/events, executor, durable audit, and
  recovery increments.
- Split persistence into schema/data classification, encryption/key ownership,
  migrations, retention/deletion/export, and backup/recovery.
- Split accessibility into React modal behavior, native approval behavior, and
  target-Mac assistive-technology verification.
- Split release readiness into licensing, supported OS, dependency remediation,
  signing/notarization, installer/upgrade/uninstall, update/rollback, and support.

### What requires an architecture decision

- O-006 gateway identity provider and deployment platform.
- O-007 provider retention mode and user disclosure.
- Durable audit storage, integrity, retention, export, and failure semantics.
- Exact executor ownership, authority token, idempotency, compensation, and
  execution-result contract.
- Product-data encryption and platform-key ownership before persistence.
- O-002 crate/workspace split only after runtime composition creates measured
  pressure; do not modularize speculatively.

### What requires executive or security approval

- Gateway deployment, identity, provider retention, data disclosure, and pilot
  data classification.
- Any real side-effect tool, OS permission, integration, or executor authority.
- Native approval-dialog acceptance or replacement before real actions.
- Supported macOS versions, enterprise distribution, signing/notarization
  ownership, support model, incident response, and license selection.
- Pilot success criteria, participant scope, workflow owner, rollback authority,
  and acceptable residual security/dependency risk.

## Verification results

### Passed

- Clean synchronized baseline and valid `meta-05` marker.
- `npm run verify` with all formatting, repository-health, lint, Clippy, tests,
  typecheck, frontend builds, and Tauri no-bundle build passing.
- `python3 .codex/hooks/session_end_gate.py` on the pre-audit workspace.
- `npm audit --audit-level=low`: zero vulnerabilities.
- Dependency and source inventories, exact Tauri capability/IPC inspection,
  migration inspection, Git history inspection, and branded asset format checks.
- Final Prettier, rustfmt, documentation-link, repository-health, and Git diff
  checks after the documentation edits.

### Failed

- Plain `cargo audit --version` failed because `cargo-audit` is not globally
  installed. The verified temporary `cargo-audit 0.22.2` binary was used.
- Live Cargo audit exited nonzero because it found the accepted unresolved
  baseline: two vulnerabilities and 18 warnings. This is evidence, not a clean
  security result.

### Not run

- Code coverage measurement; no repository coverage command or threshold exists.
- Browser/native end-to-end automation, performance, load, soak, fault-injection,
  crash-recovery, backup/restore, corruption, and disaster-recovery tests.
- Bundled `.app`/DMG production build, signing, notarization, installer,
  upgrade, uninstall, update, and rollback validation.
- SBOM generation, dependency-license compatibility review, external
  penetration test, and external accessibility assessment.

### Manual verification pending

- Current native app walkthrough on the target Mac.
- Keyboard-only, VoiceOver, contrast, zoom, and reduced-motion review.
- Native approval Escape, close, stale-dialog, and cancellation behavior against
  a real run.
- Finder, Dock, menu, switcher, packaged-icon, signing, notarization, and
  enterprise deployment behavior.

## Commands and actual results

The audit executed these command groups. Read-only `find`, `rg`, `sed`, `nl`,
`wc`, `file`, `du`, and Git inventory commands are included because they formed
the evidence set.

| Command                                                                                                                                                                                                               | Actual result                                                                                                                                                                                                 |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `git status --short --branch`                                                                                                                                                                                         | Passed; clean `main...origin/main` before audit edits.                                                                                                                                                        |
| `git log --oneline --decorate -12` and recent-history inspections                                                                                                                                                     | Passed; baseline head `6b149fa`.                                                                                                                                                                              |
| `python3 .codex/hooks/post_increment_gate.py status`                                                                                                                                                                  | Passed; `meta-05`, complete, valid, `PASS WITH ADVISORIES`.                                                                                                                                                   |
| Root/document inventories using `rg --files`, `find`, `wc`, `sed`, and `nl` across governance, architecture, product, security, testing, release, branding, workflows, plans, increments, reviews, and project memory | Passed; current authorities, historical evidence, and stale queue/publication text identified.                                                                                                                |
| Source/config inventories using `rg --files`, `find`, `rg`, `sed`, and `nl` across `src`, `src-tauri`, tests, workflows, manifests, lockfiles, capabilities, and migrations                                           | Passed; current/mock/planned boundaries reconciled against implementation.                                                                                                                                    |
| `npm run verify`                                                                                                                                                                                                      | Passed; format, health, lint, strict Clippy, 28 hook tests, 16 health tests, 124 frontend tests, 95 Rust library tests, 21 Rust integration tests, typecheck, Vite builds, and Tauri release no-bundle build. |
| `python3 .codex/hooks/session_end_gate.py`                                                                                                                                                                            | Passed before edits; no conflict, staged, unstaged, or untracked path.                                                                                                                                        |
| `npm audit --audit-level=low`                                                                                                                                                                                         | Passed; zero vulnerabilities.                                                                                                                                                                                 |
| `cargo audit --version`                                                                                                                                                                                               | Failed; Cargo reported no installed `audit` subcommand.                                                                                                                                                       |
| `/private/tmp/cortexa-meta05-cargo-audit/bin/cargo-audit --version`                                                                                                                                                   | Passed; `cargo-audit 0.22.2`.                                                                                                                                                                                 |
| `/private/tmp/cortexa-meta05-cargo-audit/bin/cargo-audit audit --file src-tauri/Cargo.lock --json`                                                                                                                    | Exited 1 as expected for findings; two vulnerabilities and 18 warnings, matching D-025/D-046.                                                                                                                 |
| `npm ls --all --json` and `cargo tree --manifest-path src-tauri/Cargo.toml --locked` inventory                                                                                                                        | Completed; full output was too large for concise review and was replaced by bounded depth output.                                                                                                             |
| `npm ls --depth=0 && cargo tree --manifest-path src-tauri/Cargo.toml --locked --depth 1`                                                                                                                              | Passed; exact direct npm and Rust dependencies inventoried.                                                                                                                                                   |
| `git tag --list && git branch --all --verbose --no-abbrev && git rev-list --count HEAD && git shortlog -sne HEAD`                                                                                                     | Passed; zero tags, 51 commits, synchronized main, one maintainer identity with two email forms.                                                                                                               |
| `find assets/branding src-tauri/icons -maxdepth 1 -type f -print \| sort && file assets/branding/* src-tauri/icons/* && du -h src-tauri/target/release/ai-agent-assistant`                                            | Passed; canonical brand assets and 16 Tauri icons exist in expected formats; executable approximately 12 MiB.                                                                                                 |
| `test -d docs/executive` plus plan/increment/review inventory and `rg` for Meta 6/icon/executive references                                                                                                           | Completed; `docs/executive` is absent and the Meta 6 naming collision was confirmed.                                                                                                                          |
| Focused `sed`/`nl` reads of project memory, decisions, readiness template/skill, Meta 5 record, and icon plan                                                                                                         | Passed; publication drift and owner queue conflict confirmed.                                                                                                                                                 |
| Focused `rg` scans for IPC, gateway, audit, process/network/browser storage, dialog/accessibility, mock/placeholder, storage, migrations, release, and licensing boundaries                                           | Passed; no unrecorded production path found.                                                                                                                                                                  |
| `find`/`wc`/Git metrics inventory                                                                                                                                                                                     | Passed; 81 source files, 14,456 source lines, 60 source/test inventory files under the selected test scan, 339 tracked files, zero tags.                                                                      |
| `python3 -c 'scores=[28,82,72,76,58,48,62,45,38,74,80,86,82,18,42,20]; print(sum(scores), sum(scores)/len(scores))'`                                                                                                  | Passed; sum 911, mean 56.9375, rounded composite 57.                                                                                                                                                          |
| `npm run format:check` and `npm run docs:check` immediately after the first draft                                                                                                                                     | Failed only because the new audit needed mechanical Prettier formatting; no source file failed rustfmt.                                                                                                       |
| `npm run repository:check` after the first draft                                                                                                                                                                      | Passed.                                                                                                                                                                                                       |
| `npx prettier --write docs/reviews/2026-07-16-product-readiness-audit.md`                                                                                                                                             | Passed; formatted only the new audit.                                                                                                                                                                         |
| Intermediate `npm run format:check` after project-memory edits                                                                                                                                                        | Failed only because added command-log rows changed Prettier table alignment in the audit.                                                                                                                     |
| Second `npx prettier --write docs/reviews/2026-07-16-product-readiness-audit.md`                                                                                                                                      | Passed; formatted only the new audit.                                                                                                                                                                         |
| Final `npm run format:check`                                                                                                                                                                                          | Passed; Prettier and rustfmt checks clean.                                                                                                                                                                    |
| Final `npm run docs:check`                                                                                                                                                                                            | Passed; Markdown/YAML formatting and internal links clean.                                                                                                                                                    |
| `python3 scripts/repository_health.py links` and final `git diff --check`                                                                                                                                             | Passed.                                                                                                                                                                                                       |
| Focused `rg` stale-state scan                                                                                                                                                                                         | Completed; remaining matches are explicitly historical D-048/Meta 5 closeout text, not current queue authority.                                                                                               |
| Final `npm run verify` after all documentation and handoff edits                                                                                                                                                      | Passed with the same test counts and build results as the baseline quality gate.                                                                                                                              |
| Final `python3 .codex/hooks/session_end_gate.py`                                                                                                                                                                      | Passed; six expected unstaged project-memory files, one expected untracked audit, and no staged or conflicted path.                                                                                           |
| Final `python3 .codex/hooks/post_increment_gate.py status`                                                                                                                                                            | Completed; `meta-05` remains complete with `PASS WITH ADVISORIES` but reports `valid: false` because the later audit documentation changes the workspace fingerprint.                                         |
| Final exact-scope inventory with `git status --porcelain=v1`, `git diff --name-only`, and `git ls-files --others --exclude-standard`                                                                                  | Passed; exactly the declared seven documentation paths are present.                                                                                                                                           |
| Complete tracked diff plus `git diff --no-index -- /dev/null docs/reviews/2026-07-16-product-readiness-audit.md`                                                                                                      | Reviewed; no source, dependency, config, generated output, secret, or unrelated change found.                                                                                                                 |
| `npm run security:scan`                                                                                                                                                                                               | Passed; secret-pattern scan clean.                                                                                                                                                                            |
| `python3 .codex/hooks/post_increment_gate.py begin --increment meta-06`                                                                                                                                               | First sandboxed attempt could not write ignored state; approved rerun passed and activated `meta-06`.                                                                                                         |
| Active-state `npm run verify`                                                                                                                                                                                         | Passed with 124 frontend, 95 Rust library, 21 Rust integration, 28 hook, and 16 repository-health tests plus all builds and static checks.                                                                    |
| Active-state `npm audit --audit-level=low`                                                                                                                                                                            | Passed; zero vulnerabilities.                                                                                                                                                                                 |
| Fresh `cargo-audit 0.22.2 --no-fetch --stale` report plus `python3 scripts/cargo_audit_gate.py /private/tmp/cortexa-meta06-cargo-audit.json --cargo-audit-exit 1`                                                     | Passed; only the exact accepted D-025/D-046 advisory baseline was present.                                                                                                                                    |
| `$post-increment-gate` architecture, security, code-health, technical-debt, roadmap-readiness, scope, and diff review                                                                                                 | Passed for the documentation increment with advisories; product readiness remains Blocked.                                                                                                                    |

## Scope and rollback

This audit changes documentation only. It does not change application source,
tests, dependencies, workflows, configuration, capabilities, CSP, permissions,
SQLite schema, icons, identifiers, or runtime behavior.

Before commit, rollback is to restore the six project-memory documents and
delete this audit plus its consolidated post-increment review. After a future
commit, revert that one documentation-only commit. No data migration,
dependency, build artifact, or remote resource is involved.

## Readiness decision

**Blocked.** The repository is not ready for pilot or production delivery.
Increment 4V is the smallest recommended remediation, but this audit does not
select, approve, begin, or implement it. The project owner must explicitly
choose the next increment.
