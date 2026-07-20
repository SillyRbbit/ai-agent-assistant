# Advisory remediation backlog

- Date: 2026-07-16
- Repository baseline: `main` at `96ba6ae5b8f94559adb0493d718f2b628e9ea8d7`
- Review type: read-only technical-debt and advisory reconciliation
- Product name: Cortexa
- Resolution update: ARB-022 is squash-merged through PR #22 at `7c79e65`.
  ARB-001 is resolved and squash-merged through PR #23 at `6e6f91d` from
  reconstructed source commit `ec919e9`; original reviewed commit `3440ce9`
  remains preserved.
- Disposition update: on 2026-07-19, the project owner approved D-059's
  evidence-based High-severity disposition on clean synchronized `main` at
  `6ce9fce`. The update changes current documentation only and does not rewrite
  this report's dated baseline or original verification evidence.

## Conclusion

The historical evidence contains 64 explicit source findings: 46 structured
post-increment findings and 18 Product Readiness Audit findings. The original
review normalized them to 25 still-valid remediation records plus closed or
superseded historical dispositions. The current backlog contains 23 unresolved
records and two subsequently resolved records, ARB-022 and ARB-001. No Critical
finding exists. Seven unresolved High findings remain: one is
decision-required, four are blocked on future capabilities, and two are
non-blocking only until explicit legal or release triggers. Their severity is
unchanged. No later product remediation is Ready.

Increment 4V resolves ARB-001 by binding both successful terminal approval paths
to the existing typed in-memory approval-audit adapter. ARB-022's
documentation-only prerequisite is published at `7c79e65`. ARB-002 remains the
first security boundary but is `DECISION REQUIRED`, not Ready for
implementation. O-006 and O-007 must close before live model networking.

The original read-only review changed no source or existing documentation and
created only this report. The later ARB-022 resolution changes the exact ten
documentation paths recorded in its increment and post-increment review; it
does not rewrite dated Meta 7 evidence or change product source.

The later High-severity disposition updates only live documentation and this
backlog. It adds no product capability, dependency, license grant, network,
credential, persistence, execution, enterprise control, signing, or release
authority.

## Scope and method

The review inspected:

- all required root governance, architecture, product, security, testing,
  roadmap, decision, troubleshooting, plan, and project-memory documents;
- all 45 files under `docs/increments/`;
- all 39 files under `docs/plans/`;
- all 24 pre-existing files under `docs/reviews/`;
- every structured post-increment manifest and all 18 readiness-audit findings;
- every plan or increment hit for `PASS WITH ADVISORIES`, `Advisory`, `Medium`,
  `Low`, `Deferred`, `Follow-up`, `Technical debt`, or
  `Recommended remediation`;
- current React, Tauri, Rust, SQLite, test, workflow, dependency, icon, and
  release configuration evidence.

Historical wording was not treated as current proof. Each candidate was
classified against the current source tree, current tests, accepted decisions,
Git state, hosted check evidence where available, and the valid Meta 7 marker.

## Status definitions

- **Still valid**: current repository evidence still demonstrates the risk or
  missing boundary.
- **Already resolved**: later verified work closed the original finding.
- **Superseded**: the original finding was replaced by a more precise current
  record or later architecture boundary.
- **Duplicate**: another source describes the same current root cause and maps
  to one canonical backlog record.
- **No longer relevant**: the original premise is not part of the current
  supported product or repository contract.
- **Resolved**: a later bounded remediation closed the canonical backlog record
  with current evidence while retaining its original history.

## Current verification evidence

| Check                                                                      | Actual result                                                                                                                                                                                                                                   |
| -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `git status --short --branch`                                              | Passed before the report was created: clean `main...origin/main`.                                                                                                                                                                               |
| `git log -12 --oneline --decorate`                                         | Confirmed current `HEAD` and `origin/main` at `96ba6ae`; dependency repair is `b298999`.                                                                                                                                                        |
| `git tag --list --sort=-creatordate`                                       | No tags exist.                                                                                                                                                                                                                                  |
| `python3 .codex/hooks/post_increment_gate.py status`                       | `meta-07`, `PASS WITH ADVISORIES`, complete, `valid: true`.                                                                                                                                                                                     |
| `npm run verify`                                                           | Passed: formatting, repository health, lint, strict Clippy, 28 hook tests, 16 repository-health tests, 124 frontend tests, 95 Rust library tests, 21 Rust integration tests, typecheck, two frontend builds, and Tauri release no-bundle build. |
| `npm audit --audit-level=low`                                              | Passed after the sandboxed DNS attempt was retried with network access: 0 vulnerabilities.                                                                                                                                                      |
| `cargo-audit 0.22.2 --no-fetch --stale` plus `scripts/cargo_audit_gate.py` | Audit exited 1 for findings; the gate passed because the report contains exactly 2 accepted vulnerabilities and 18 accepted warnings with no baseline drift.                                                                                    |
| `npm run tauri -- build`                                                   | Failed at `bundle_dmg.sh` after the release executable and `Cortexa.app` built successfully.                                                                                                                                                    |
| PR #19 hosted checks                                                       | CI, Documentation, and Security checks all passed before squash merge at `96ba6ae`.                                                                                                                                                             |
| GitHub main protection and ruleset APIs                                    | Both returned HTTP 403 stating the feature requires GitHub Pro or a public repository. The repository is private and the caller has Admin permission; hosted checks exist, but remote enforcement is unavailable on the current plan.           |
| `git status --short --branch` after builds                                 | Passed: generated outputs remain ignored and the tree stayed clean before this report.                                                                                                                                                          |
| Gate status after creating this report                                     | Expected `valid: false`: this new non-ignored untracked report changes the workspace fingerprint; it does not invalidate the recorded clean-`96ba6ae` Meta 7 evidence.                                                                          |

## ARB-022 resolution evidence

- PR #21 squash-merged the original backlog and first post-Meta-7 memory
  reconciliation at `cc434d9`.
- The pre-edit current-state scan reproduced the remaining stale publication
  instructions in `HANDOFF.md`, `NEXT_STEPS.md`, `PROJECT_STATUS.md`, and
  `ROADMAP.md`.
- The bounded remediation updates exactly eight live documentation authorities
  and creates its increment record and post-increment review. Dated Meta 7
  evidence and the 4V plan/source/test/gate state are unchanged.
- The post-edit stale-instruction scan has no matches. Protected-path checks,
  formatting, documentation, repository, security, full verification, diff
  review, and the mandatory `remediation-arb-022` gate pass.
- PR #22 squash-merged the resolving remediation at `7c79e65`. Clean
  synchronized `main` contained that prerequisite before the `04v` gate began.

## ARB-001 resolution evidence

- The original mandatory `04v` gate began on clean synchronized `main` at
  `7c79e65`. For publication refresh, a new `04v` gate began on clean
  synchronized `main` at `d81b73a` before the preserved change was applied
  without commit.
- `InitialGatewayTurn` now owns one private typed in-memory approval-audit
  adapter. Native and run-termination success use the same private
  manager-then-audit helper and return only a closed
  `AuditedApprovalResolution` containing the exact manager-owned resolution and
  its sequence-only receipt.
- Focused tests prove exact record facts, receipt binding, native and
  run-termination behavior, idempotence, late-outcome rejection, redaction, and
  typed audit failure after manager terminalization without a returned
  resolution or stale pending subject.
- Strict Clippy, complete `npm run verify`, documentation, security, npm audit,
  conflict, whitespace, exact-scope, complete-diff, session-end, and mandatory
  gate checks pass. The sandboxed npm audit failed on DNS; the approved network
  retry found zero vulnerabilities. The consolidated result is `PASS WITH
ADVISORIES`, and the `04v` marker is complete and valid.
- Durable audit, active-run coordination, native dialog lifecycle, dispatch,
  execution, provider continuation, transport, credentials, persistence, IPC,
  UI, dependencies, capabilities, and permissions remain outside Increment 4V.
- Reconstructed source commit `ec919e9` passed hosted CI, Documentation, and
  Security before PR #23 was squash-merged at `6e6f91d`. The exact automated,
  review, and gate evidence is recorded in
  `docs/reviews/2026-07-16-04v-post-increment-review.md`.

## High-severity disposition update

D-059 applies the project owner's 2026-07-19 direction without lowering any
severity or converting deferred risk into resolution.

| ID      | Finding                                     | Current evidence and reachability                                              | Affects implemented behavior                       | Required work                                                | Disposition                   |
| ------- | ------------------------------------------- | ------------------------------------------------------------------------------ | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------- |
| ARB-001 | Terminal approval audit binding             | Published implementation and focused regression evidence close the former gap  | Yes; the implemented terminal paths are now bound  | None                                                         | `RESOLVED`                    |
| ARB-002 | Gateway identity, deployment, and retention | No HTTP client, provider SDK, origin, credential, Keychain, or live traffic    | No current external processing exists              | Architecture, security, privacy, and owner decisions         | `DECISION REQUIRED`           |
| ARB-003 | Restricted executor                         | No dispatcher, executor, platform action path, or execution IPC exists         | No operating-system action path exists             | New product capability plus architecture and security review | `BLOCKED - FUTURE CAPABILITY` |
| ARB-004 | Production end-to-end workflow              | UI is mocked and trusted Rust boundaries are not production-wired              | No production workflow exists                      | New coordinator and integration capabilities                 | `BLOCKED - FUTURE CAPABILITY` |
| ARB-005 | Durable audit and product-data lifecycle    | Audit is process-local; SQLite stores bootstrap metadata only                  | Current volatile behavior matches its stated scope | New durable-data capability and privacy architecture         | `BLOCKED - FUTURE CAPABILITY` |
| ARB-006 | Repository and distribution license         | No license grant exists; repository guidance says none was selected            | No private local behavior is affected              | Legal and project-owner decision                             | `DEFERRED - NON-BLOCKING`     |
| ARB-007 | Release integrity and artifact lineage      | Local builds exist; no signed, notarized, supported release is claimed         | No production distribution path is active          | Release-governance decisions and release engineering         | `DEFERRED - NON-BLOCKING`     |
| ARB-008 | Enterprise controls                         | No SSO, enrollment, fleet, admin policy, governance, or support plane exists   | No enterprise deployment is claimed                | New product and operational capabilities                     | `BLOCKED - FUTURE CAPABILITY` |
| ARB-044 | Combined release and enterprise readiness   | Canonical split findings cover its independent legal, release, and fleet risks | No independent current behavior                    | Follow ARB-006, ARB-007, ARB-008, and ARB-010                | `SUPERSEDED`                  |

No unresolved item is currently exploitable through an implemented live
network, executor, durable-product-data, enterprise, or production-release path
because those paths do not exist. This is not risk acceptance: each High
finding remains tracked and becomes blocking at its recorded trigger.

| ID      | Current and future blocking scope                                                         | Required owner                                        | Revisit trigger                                                        | Acceptance criteria                                                                                                  |
| ------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| ARB-001 | Blocks nothing; regression would block future orchestration                               | Engineering and security                              | Any change to terminal approval or approval-audit ownership            | Existing exact binding, redaction, failure, idempotence, and late-outcome tests continue to pass                     |
| ARB-002 | Does not block local mocks; blocks all live model networking and external processing      | Project, security, privacy, and executive owners      | Before adding a network client, gateway origin, token, or live traffic | Close O-006/O-007 with identity, deployment, token lifecycle, data classes, disclosure, logging, retention, deletion |
| ARB-003 | Does not block transport-free work; blocks local action execution and a functional pilot  | Product, architecture, and security owners            | Before adding dispatch or any operating-system side effect             | One exact allowlisted tool with typed authority, idempotency, cancellation, result, audit, and failure contracts     |
| ARB-004 | Does not block isolated primitives; blocks a controlled end-to-end pilot                  | Product and architecture owners                       | Before wiring UI, transport, policy, approval, audit, and execution    | Separately verified coordinator, IPC/events, transport, executor, result, cancellation, and recovery increments      |
| ARB-005 | Does not block volatile behavior; blocks durable audit, recovery, and a durable pilot     | Product, security, and privacy owners                 | Before persisting approval evidence or user/product content            | Approved inventory, key ownership, transactions, migrations, retention, export/delete, backup, and recovery          |
| ARB-006 | Does not block private development; blocks public distribution and external contributions | Project owner and legal reviewer                      | Before public distribution, open-source publication, or contributions  | Explicit reviewed license and reconciled contribution/distribution terms; until then all rights remain reserved      |
| ARB-007 | Does not block unsigned local development; blocks trusted public macOS distribution       | Project, release, security, and credential owners     | Before release-candidate or public-distribution work                   | Resolve O-003/O-009; verify targets, artifact identity, signing, notarization, hashes, rollback, uninstall, support  |
| ARB-008 | Does not block single-user local work; blocks enterprise onboarding and deployment        | Executive, product, security, privacy, and operations | Before onboarding enterprise users or managed devices                  | Separately approved identity, enrollment, fleet policy, data governance, support, and incident-operation controls    |
| ARB-044 | No independent block                                                                      | Owners of canonical split findings                    | If a source report still treats the combined finding as active         | All references point to canonical findings without duplicate remediation                                             |

## Prioritized unresolved backlog

| Priority | ID      | Severity | Category             | Summary                                                                       | Blocks next product increment    |
| -------: | ------- | -------- | -------------------- | ----------------------------------------------------------------------------- | -------------------------------- |
|        1 | ARB-002 | High     | Security             | Gateway identity, credentials, deployment, and retention remain unresolved    | No; blocks live traffic          |
|        2 | ARB-003 | High     | Security             | No restricted executor or exact platform implementation exists                | No; blocks functional pilot      |
|        3 | ARB-004 | High     | Architecture         | No production end-to-end assistant workflow exists                            | No; blocks pilot                 |
|        4 | ARB-005 | High     | Reliability          | Durable audit and product-data lifecycle are absent                           | No; blocks durable pilot         |
|        5 | ARB-006 | High     | Documentation        | No license is selected                                                        | No; blocks distribution          |
|        6 | ARB-007 | High     | Reliability          | Release integrity and artifact lineage are absent                             | No; blocks release               |
|        7 | ARB-008 | High     | Architecture         | Enterprise control-plane capabilities are absent                              | No; blocks enterprise deployment |
|        8 | ARB-009 | Medium   | Security             | Two Rust vulnerabilities and 18 warnings remain accepted                      | No; blocks release acceptance    |
|        9 | ARB-010 | Medium   | Reliability          | DMG creation fails                                                            | No; blocks installer readiness   |
|       10 | ARB-011 | Medium   | UX                   | Native approval cannot be dismissed after run cancellation                    | No; blocks real-action UX        |
|       11 | ARB-012 | Medium   | Security             | CSP retains inline script and style allowances                                | No; blocks release hardening     |
|       12 | ARB-013 | Medium   | Reliability          | Coupled dependency proposals lack combined merge enforcement                  | No                               |
|       13 | ARB-014 | Medium   | Reliability          | Product operational observability is absent                                   | No; blocks supported operation   |
|       14 | ARB-015 | Medium   | Architecture         | Public lower-level APIs can bypass bound-turn ownership in future wiring      | No; blocks live transport review |
|       15 | ARB-016 | Medium   | Architecture         | `gateway_request` accumulates policy, approval, lifecycle, and macOS coupling | No                               |
|       16 | ARB-017 | Medium   | Testing              | End-to-end, recovery, fault, and installer tests are absent                   | No                               |
|       17 | ARB-018 | Medium   | Performance          | Performance budgets and measurements are absent                               | No                               |
|       18 | ARB-019 | Medium   | Accessibility        | Approval accessibility evidence is incomplete                                 | No                               |
|       19 | ARB-020 | Medium   | Developer experience | Maintainer concentration and remote enforcement remain unresolved             | No                               |
|       20 | ARB-021 | Medium   | Documentation        | Executive and pilot evidence package is absent                                | No                               |
|       21 | ARB-023 | Advisory | UX                   | Raw unbundled `tauri dev` uses the generic macOS executable icon              | No                               |
|       22 | ARB-024 | Advisory | Technical debt       | The authoritative brand source remains an opaque raster                       | No                               |
|       23 | ARB-025 | Advisory | Security             | Repository hooks remain operator-trusted and bypassable                       | No                               |

## Canonical advisory records

### ARB-001 - Bind terminal approval resolutions to audit

- **Original increment or report:** `meta-06-F1`, PRA-002, `04u-F2`, and the
  proposed Increment 4V plan.
- **Current status:** Resolved and published through PR #23 at `6e6f91d`.
- **Severity / category:** High / Security.
- **Why it matters:** A successful native or run-termination resolution can
  leave `InitialGatewayTurn` before the existing typed audit adapter validates
  and records the exact manager-owned result.
- **Current evidence:** `InitialGatewayTurn` owns one private
  `InMemoryApprovalAuditAdapter`. Both terminal paths use one private
  manager-then-audit helper and return only `AuditedApprovalResolution`, which
  owns the exact manager resolution and sequence-only receipt. Focused tests
  prove exact record binding, typed audit failure with no returned resolution or
  stale pending subject, idempotence, late-outcome rejection, and redaction.
- **Impact if ignored:** Future orchestration could expose an incomplete
  approval evidence chain or accidentally create a bypass around audit
  validation.
- **Recommended remediation / effort:** Completed through the exact two-file
  Increment 4V implementation. Effort: Small.
- **Dependencies:** ARB-022 was published at `7c79e65`, project-owner approval
  was recorded, and the reconstructed mandatory `04v` gate began on
  `d81b73a` before the preserved source edits were applied.
- **Regression risks:** Manager state is already terminal before audit. Audit
  failure must return no resolution and must not restore or retain stale pending
  state; receipts must remain non-authorizing and redacted.
- **Blocks next product increment:** No. The audit-binding blocker is closed;
  live traffic and execution remain blocked by separate advisories.
- **Recommended milestone:** Phase 4 security closure.

### ARB-002 - Resolve live gateway identity, credential, deployment, and retention boundaries

- **Original increment or report:** `04k-F1`, `04n-F1`, `04n-F2`,
  `meta-06-F2`, PRA-004, O-006, and O-007.
- **Current status:** `DECISION REQUIRED`; the broad 4K transport deferral is
  superseded by this exact current boundary. D-060 through D-063 select the
  Phase 1 boundaries, and D-064 closes the pre-implementation design, but
  no-traffic provisioning, synthetic transport, provider evidence, and
  real-content activation remain incomplete. O-006 and D-061 block live
  traffic.
- **Severity / category:** High / Security.
- **Why it matters:** Live model traffic cannot be approved without an exact
  desktop principal, gateway deployment, server credential owner, short-lived
  desktop credential boundary, provider retention mode, and user disclosure.
- **Current evidence:** No HTTP client, provider SDK, gateway origin, model,
  authorization header, Keychain adapter, or live transport exists in source or
  manifests. `SECURITY.md` keeps O-006 and O-007 as live blockers.
- **Impact if ignored:** Credentials or personal content could cross the wrong
  boundary, and the fixed local tool-set could disagree with the deployed
  gateway.
- **Recommended remediation / effort:** Complete ARB-002A's documentation-only
  threat model and closed configuration, then use separately approved Stage B
  no-traffic provisioning, Stage C synthetic-only transport, and Stage D
  real-content activation. Effort: Large across all stages.
- **Dependencies:** Security and executive ownership, deployment platform,
  provider retention terms, data classification, and ARB-001.
- **Regression risks:** Premature transport could leak request bytes, log
  content, accept caller-selected origins or tool sets, or weaken cancellation
  and redaction.
- **Blocks next product increment:** No for transport-free 4V; yes for any live
  gateway increment. ARB-002A does not make a runtime increment Ready.
- **Recommended milestone:** Live gateway architecture.

### ARB-003 - Add one restricted executor and exact platform implementation

- **Original increment or report:** PRA-003 and the capability-specific portion
  of `04m-F1`.
- **Current status:** `BLOCKED - FUTURE CAPABILITY`; the old generic platform
  advisory is superseded by this exact executor boundary.
- **Severity / category:** High / Security.
- **Why it matters:** Strict schemas and approvals do not produce user value
  unless one registered implementation can perform an exact action only after
  all trusted transitions succeed.
- **Current evidence:** The tool catalog contains
  `get_current_datetime@1` and `create_local_task@1`, but the source tree has no
  dispatcher, executor, or platform module and no Tauri execution command.
- **Impact if ignored:** No controlled workflow can complete. A later rushed
  implementation could reintroduce a generic action or platform status map.
- **Recommended remediation / effort:** Design one exact executor with bounded
  input, authority, idempotency, result, cancellation, and audit contracts.
  Effort: Large.
- **Dependencies:** ARB-001, coordinator design, threat model, and an approved
  first real tool.
- **Regression risks:** Side effects could occur before audit or after stale
  approval, be duplicated after unknown outcomes, or bypass local policy.
- **Blocks next product increment:** No for 4V; yes for a functional pilot.
- **Recommended milestone:** Controlled workflow prototype.

### ARB-004 - Compose one production end-to-end workflow

- **Original increment or report:** `meta-06-F4`, PRA-001, and trusted-context
  deferrals in the Phase 3 plans.
- **Current status:** `BLOCKED - FUTURE CAPABILITY`.
- **Severity / category:** High / Architecture.
- **Why it matters:** Current UI value is mocked while current Rust security
  primitives are transport-free and disconnected.
- **Current evidence:** `ARCHITECTURE.md` states the React mock loop and Rust
  gateway turn are not wired. `get_app_info` is the only custom invoke command.
  No runtime coordinator, provider client, event bridge, dispatcher, or result
  path exists.
- **Impact if ignored:** The product cannot demonstrate a real controlled
  workflow, measure value, or exercise failure ownership across boundaries.
- **Recommended remediation / effort:** Split one vertical slice into separate
  coordinator, IPC/event, authenticated transport, executor, result/audit, and
  recovery increments. Effort: Large.
- **Dependencies:** ARB-001, ARB-002, ARB-003, and exact pilot workflow
  selection.
- **Regression risks:** A single broad integration change would obscure
  authority, cancellation, privacy, and rollback failures.
- **Blocks next product increment:** No for 4V; yes for pilot readiness.
- **Recommended milestone:** Controlled workflow prototype.

### ARB-005 - Define durable audit and product-data lifecycle

- **Original increment or report:** `04h-F1`, `04i-F1`, `04l-F1`,
  `04u-F2`, and PRA-007.
- **Current status:** `BLOCKED - FUTURE CAPABILITY`; duplicate reports share
  this root cause.
- **Severity / category:** High / Reliability.
- **Why it matters:** Approval evidence, conversations, tasks, memory, and
  recovery cannot survive process restart or satisfy retention and deletion
  requirements.
- **Current evidence:** `audit::approval` is process-local and bounded.
  Release startup uses in-memory SQLite; file-backed startup persists only
  `app_initialized`. No product repository, encryption key boundary, retention,
  export, backup, or recovery implementation exists.
- **Impact if ignored:** State and evidence are lost on restart, and later
  persistence could create data loss, privacy, or migration defects.
- **Recommended remediation / effort:** Decide data classification, durable
  audit ownership, transaction ordering, encryption/key ownership, retention,
  deletion/export, migrations, and recovery in separate increments. Effort:
  Large.
- **Dependencies:** O-007, security approval, coordinator design, and a
  purpose-limited data inventory.
- **Regression risks:** Persisting too early could write plaintext personal
  data, duplicate raw content into audit, or make rollback destructive.
- **Blocks next product increment:** No for volatile 4V; yes for a durable
  pilot.
- **Recommended milestone:** Durable pilot.

### ARB-006 - Select an explicit repository and distribution license

- **Original increment or report:** License portion of PRA-005 and D-047.
- **Current status:** `DEFERRED - NON-BLOCKING`; severity remains High. The
  temporary posture is proprietary and all rights reserved, with no license
  grant.
- **Severity / category:** High / Documentation.
- **Why it matters:** Distribution and an open contribution program require
  explicit legal terms.
- **Current evidence:** No tracked `LICENSE`, `COPYING`, or `NOTICE` file
  exists. `docs/github/LICENSING.md` deliberately records that no license is
  selected.
- **Impact if ignored:** The product cannot be publicly distributed or accept
  contributions under clear rights.
- **Recommended remediation / effort:** Obtain owner/legal direction, add the
  selected authoritative license, and reconcile contribution and release copy.
  Effort: Small.
- **Dependencies:** Project-owner and legal decision.
- **Regression risks:** An inferred or unsuitable license could grant unintended
  rights or conflict with dependency and commercial plans.
- **Blocks next product increment:** No; blocks public release.
- **Recommended milestone:** Release/legal preparation.

### ARB-007 - Establish release integrity and artifact lineage

- **Original increment or report:** Release-technical portion of PRA-005 and
  PRA-018.
- **Current status:** `DEFERRED - NON-BLOCKING`; severity remains High for
  release. The provisional tested baseline is macOS 14+ on Apple Silicon, and
  no Intel or production-distribution claim is made.
- **Severity / category:** High / Reliability.
- **Why it matters:** Enterprise distribution requires a supported target,
  reviewed artifact identity, signing, notarization, update, rollback, and
  support evidence.
- **Current evidence:** No Git tags exist. Tauri sets macOS 14.0 and Hardened
  Runtime, but no signing/notarization evidence, updater, artifact manifest,
  upgrade/uninstall validation, support policy, or release record exists.
- **Impact if ignored:** Users could receive an unsigned, untraceable,
  non-upgradeable artifact with no safe withdrawal path.
- **Recommended remediation / effort:** Split target-OS confirmation, version
  and tag policy, signing/notarization, artifact hashes, update/rollback,
  uninstall, support, and release evidence into bounded work. Effort: Large.
- **Dependencies:** One real workflow, ARB-006, O-003, owner of signing assets,
  and installer remediation.
- **Regression risks:** Release automation can expose credentials, sign the
  wrong content, or create an update path that cannot safely roll back.
- **Blocks next product increment:** No; blocks production release.
- **Recommended milestone:** Release candidate and production release.

### ARB-008 - Define enterprise identity, fleet, policy, data, and support controls

- **Original increment or report:** PRA-015 and the enterprise portion of
  `meta-06-F6`.
- **Current status:** `BLOCKED - FUTURE CAPABILITY`.
- **Severity / category:** High / Architecture.
- **Why it matters:** Enterprise deployment needs governance beyond a
  single-user local prototype.
- **Current evidence:** No SSO/IdP, device enrollment, fleet deployment, admin
  policy distribution, compliance evidence, data-governance controls, support
  ownership, or incident-operation system exists.
- **Impact if ignored:** Enterprise adoption would lack accountable identity,
  revocation, configuration, support, and data controls.
- **Recommended remediation / effort:** Treat each control family as a separate
  post-pilot milestone rather than one broad enterprise increment. Effort:
  Large.
- **Dependencies:** ARB-002, ARB-005, a measured pilot, executive sponsorship,
  and security approval.
- **Regression risks:** Premature multi-user or fleet abstractions could force
  unreviewed tenancy, retention, and authorization assumptions into the local
  core.
- **Blocks next product increment:** No; blocks enterprise deployment.
- **Recommended milestone:** Enterprise pilot.

### ARB-009 - Remediate the accepted RustSec baseline

- **Original increment or report:** D-025, D-046, `meta-05-F1`,
  `meta-06-F3`, `repo-dependency-baseline-compatibility-F1`, and PRA-006.
- **Current status:** Still valid; duplicate reports share one exact baseline.
- **Severity / category:** Medium / Security.
- **Why it matters:** Accepted vulnerabilities, unsoundness, and unmaintained
  transitive packages create security-review and procurement risk even when
  current reachability appears limited.
- **Current evidence:** Current `cargo-audit 0.22.2` reports
  RUSTSEC-2026-0194 and RUSTSEC-2026-0195 plus 16 unmaintained and 2 unsoundness
  warnings. The exact-baseline gate passes, proving no drift rather than proving
  remediation.
- **Impact if ignored:** A currently unreachable path could become reachable,
  upstream support can disappear, and accepted debt can become normalized.
- **Recommended remediation / effort:** Isolate dependency upgrades or
  replacements, review reachability and licenses, and rerun the full native and
  RustSec matrix. Effort: Medium.
- **Dependencies:** Upstream Tauri/rfd/plist/GTK compatibility and Rust 1.90.
- **Regression risks:** Native dependencies or bundled SQLCipher can stop
  compiling, and advisory reduction can be offset by incompatible transitive
  changes.
- **Blocks next product increment:** No for 4V; blocks production release risk
  acceptance.
- **Recommended milestone:** Pre-release dependency hardening.

### ARB-010 - Repair DMG creation

- **Original increment or report:** `meta-07-F2` and installer portion of
  PRA-005/PRA-009.
- **Current status:** Still valid.
- **Severity / category:** Medium / Reliability.
- **Why it matters:** A successful `.app` bundle is not proof that the declared
  default installer can be produced or installed.
- **Current evidence:** Current `npm run tauri -- build` builds the release
  executable and `Cortexa.app`, then fails while running `bundle_dmg.sh` for
  `Cortexa_0.1.0_aarch64.dmg`.
- **Impact if ignored:** Installer readiness and release rollback cannot be
  verified.
- **Recommended remediation / effort:** Diagnose the DMG script on the release
  target, fix only the exact packaging cause, then validate mount, drag-install,
  launch, upgrade, and uninstall. Effort: Medium.
- **Dependencies:** Release target environment and later signing/notarization.
- **Regression risks:** Packaging changes can alter app resources, bundle
  identity, permissions, or the already verified icon.
- **Blocks next product increment:** No; blocks installer readiness.
- **Recommended milestone:** Phase 10 release readiness.

### ARB-011 - Replace or explicitly accept the non-dismissible native approval dialog

- **Original increment or report:** D-025, `04u-F2`, PRA-010, and native part of
  `meta-06-F5`.
- **Current status:** Still valid.
- **Severity / category:** Medium / UX.
- **Why it matters:** Run cancellation terminalizes manager state but cannot
  close the synchronous native dialog, so a stale security prompt may remain
  visible and Escape/window-close are unavailable.
- **Current evidence:** `MacOsNativeApprovalDecisionSource` synchronously calls
  `rfd::MessageDialog::show()`. There is no dialog handle or close operation;
  D-025 records the target-Mac limitation.
- **Impact if ignored:** Users can see a stuck or misleading prompt after the
  underlying run is cancelled, even though late outcomes are safely rejected.
- **Recommended remediation / effort:** Before real actions, select a
  cancellable native source or explicitly accept the residual UX with target-Mac
  security and accessibility evidence. Effort: Medium.
- **Dependencies:** Real approval orchestration design and Security/UX owner
  decision.
- **Regression risks:** A replacement must preserve manager identity sealing,
  default Reject, evidence mapping, redaction, and late-outcome rejection.
- **Blocks next product increment:** No for 4V; blocks production approval UX.
- **Recommended milestone:** Pilot hardening.

### ARB-012 - Harden the Tauri CSP

- **Original increment or report:** PRA-011.
- **Current status:** Still valid.
- **Severity / category:** Medium / Security.
- **Why it matters:** Inline script and style allowances weaken defense in
  depth at the WebView boundary.
- **Current evidence:** `src-tauri/tauri.conf.json` contains
  `script-src 'self' 'unsafe-inline'` and
  `style-src 'self' 'unsafe-inline'`.
- **Impact if ignored:** A future injection defect has a broader execution
  surface than necessary.
- **Recommended remediation / effort:** Test narrower production CSP and
  separately account for development HMR requirements. Effort: Medium.
- **Dependencies:** Tauri/Vite runtime and target-Mac development/build tests.
- **Regression risks:** Incorrect CSP changes can blank the UI, break styles,
  or disable local development while appearing secure in static review.
- **Blocks next product increment:** No; blocks release hardening.
- **Recommended milestone:** Pre-release security hardening.

### ARB-013 - Enforce combined compatibility for coupled dependency proposals

- **Original increment or report:**
  `repo-dependency-baseline-compatibility-F2` and the repair increment record.
- **Current status:** Still valid.
- **Severity / category:** Medium / Reliability.
- **Why it matters:** Individually green dependency branches can produce an
  incompatible combined `main` when merged without refreshed merge-candidate
  checks.
- **Current evidence:** The dependency repair records an invalid JavaScript
  graph and unsupported `rusqlite` combination after eight merges.
  `.github/dependabot.yml` disables rebasing, has no dependency groups, and the
  current private repository plan does not provide branch protection/ruleset
  enforcement.
- **Impact if ignored:** `main` can again become unbuildable even though each
  stale proposal once passed.
- **Recommended remediation / effort:** Require current-base checks immediately
  before merge, group coupled updates, define merge order, and rerun the full
  candidate after every dependency merge. Effort: Small.
- **Dependencies:** Repository policy and available GitHub plan or a documented
  manual merge protocol.
- **Regression risks:** Over-grouping can hide which update caused a failure;
  automatic rebasing can create noisy or unreviewed changes.
- **Blocks next product increment:** No, provided `main` remains verified.
- **Recommended milestone:** Repository dependency governance.

### ARB-014 - Add privacy-preserving operational observability

- **Original increment or report:** PRA-017.
- **Current status:** Still valid.
- **Severity / category:** Medium / Reliability.
- **Why it matters:** A supported product needs bounded health, failure,
  correlation, export, alerting, and incident evidence without logging personal
  content.
- **Current evidence:** Source contains startup/menu `eprintln!` summaries and
  local Settings diagnostics, but no telemetry, health model, audit export,
  alerting, or support runbook.
- **Impact if ignored:** Production failures cannot be diagnosed or supported
  consistently.
- **Recommended remediation / effort:** Define closed privacy-preserving
  diagnostics, retention, user controls, export, and support procedures after
  the live coordinator exists. Effort: Large.
- **Dependencies:** ARB-002, ARB-004, ARB-005, and data policy.
- **Regression risks:** Observability can leak prompts, arguments, identifiers,
  paths, raw errors, or credentials if added without field-level limits.
- **Blocks next product increment:** No; blocks supported operation.
- **Recommended milestone:** Durable pilot.

### ARB-015 - Preserve bound-turn ownership against lower-level public APIs

- **Original increment or report:** `04o-F2`, `04p-F2`, `04q-F2`, and the
  lower-level portion of `04r-F2`.
- **Current status:** Still valid; later reports are duplicates of the same
  boundary.
- **Severity / category:** Medium / Architecture.
- **Why it matters:** Future transport could instantiate public validators,
  registries, or validation functions directly and bypass request, schema,
  terminal-release, policy, or approval ownership in `InitialGatewayTurn`.
- **Current evidence:** `GatewayStreamValidator::new`, normalized protocol
  types, `validate_function_call`, and registry types remain public. The crate
  itself is `publish = false`, so the concern is future internal wiring, not a
  supported external API.
- **Impact if ignored:** A live path could reconstruct a detached partial
  security flow while all individual lower-level tests still pass.
- **Recommended remediation / effort:** Require future transport to own
  `InitialGatewayTurn`; narrow visibility only when fixture coverage has an
  equivalent internal home. Effort: Medium.
- **Dependencies:** Coordinator and transport design.
- **Regression risks:** Premature visibility changes can remove useful contract
  tests without improving runtime ownership.
- **Blocks next product increment:** No for 4V; blocks live transport review.
- **Recommended milestone:** Before authenticated gateway transport.

### ARB-016 - Contain trusted-assembly coupling in `gateway_request`

- **Original increment or report:** `04r-F1`, `04s-F1`, `04t-F1`, `04u-F1`,
  and coupling portion of the 4V plan.
- **Current status:** Still valid; later findings are duplicates of one growing
  assembly boundary.
- **Severity / category:** Medium / Architecture.
- **Why it matters:** One module now constructs requests, validates streams,
  applies policy, owns approval presentation/resolution/cancellation, and
  imports a macOS-specific source outcome.
- **Current evidence:** `src-tauri/src/agent/gateway_request.rs` is 889 lines and
  directly imports agent, tools, policy, approval manager/types, and the macOS
  source outcome. The planned 4V adapter adds audit ownership.
- **Impact if ignored:** Future orchestration changes become harder to review
  and can blur portable versus platform-specific ownership.
- **Recommended remediation / effort:** Complete 4V within the established
  boundary, then introduce a platform-neutral coordinator only when the next
  real composition step requires it. Effort: Medium.
- **Dependencies:** ARB-001 and an approved coordinator contract.
- **Regression risks:** A speculative abstraction can duplicate state machines
  or weaken exact ownership; a late abstraction can make coupling harder to
  untangle.
- **Blocks next product increment:** No.
- **Recommended milestone:** Before expanding initial-turn orchestration.

### ARB-017 - Add composition, recovery, fault, and installer tests

- **Original increment or report:** Testing portion of PRA-009.
- **Current status:** Still valid.
- **Severity / category:** Medium / Testing.
- **Why it matters:** Unit and contract tests cannot prove behavior across live
  process, transport, persistence, and packaging boundaries.
- **Current evidence:** Repository search finds no Playwright/Cypress E2E,
  fault-injection, crash-recovery, installer, or coverage tooling. Current tests
  are deterministic frontend, Rust unit/integration, hook, and repository
  checks.
- **Impact if ignored:** Cross-boundary cancellation, replay, partial failure,
  recovery, and installation defects can escape otherwise strong local tests.
- **Recommended remediation / effort:** Add evidence with each real boundary;
  establish one end-to-end and recovery harness when the controlled vertical
  slice exists. Effort: Large.
- **Dependencies:** ARB-004, ARB-005, and ARB-010.
- **Regression risks:** Premature end-to-end tests against mocks become brittle
  without proving production behavior.
- **Blocks next product increment:** No.
- **Recommended milestone:** Pilot and release gates.

### ARB-018 - Define and measure performance budgets

- **Original increment or report:** Performance portion of PRA-009 and Product
  Readiness Audit performance assessment.
- **Current status:** Still valid.
- **Severity / category:** Medium / Performance.
- **Why it matters:** Startup, streaming, approval, action, persistence, and CPU
  behavior have no acceptance budgets.
- **Current evidence:** No benchmark, load-test, profiling gate, or measured
  performance target exists. Current limits bound protocol sizes and deadlines
  but do not measure user-perceived latency or resource use.
- **Impact if ignored:** A live workflow can be too slow or resource-intensive
  without a clear regression signal.
- **Recommended remediation / effort:** Set budgets when the first live slice is
  designed, then measure before optimizing. Effort: Medium.
- **Dependencies:** ARB-004 and a representative pilot workflow.
- **Regression risks:** Premature synthetic targets can optimize the wrong path
  or produce noisy, non-portable CI gates.
- **Blocks next product increment:** No.
- **Recommended milestone:** Controlled workflow prototype and pilot hardening.

### ARB-019 - Complete approval accessibility evidence

- **Original increment or report:** `meta-06-F5` and PRA-008.
- **Current status:** Still valid.
- **Severity / category:** Medium / Accessibility.
- **Why it matters:** Approval is the most security-sensitive interaction and
  must be keyboard, focus, contrast, and assistive-technology usable.
- **Current evidence:** The React mock dialog has ARIA dialog metadata but no
  focus trap, initial focus, Escape handling, or focus-return behavior, and no
  matching tests. Target-Mac VoiceOver and contrast evidence is absent. The
  native dialog has the separate ARB-011 close limitation.
- **Impact if ignored:** Users can lose context or be unable to operate a
  consequential decision safely.
- **Recommended remediation / effort:** Split React focus/keyboard behavior,
  native dialog acceptance/replacement, and target-Mac VoiceOver/contrast
  validation. Effort: Medium.
- **Dependencies:** Final approval UX and ARB-011.
- **Regression risks:** Focus fixes can trap users, break screen-reader order,
  or conflict with native dialog behavior if combined prematurely.
- **Blocks next product increment:** No; blocks inclusive pilot claims.
- **Recommended milestone:** Pilot hardening.

### ARB-020 - Reduce maintainer concentration and establish remote merge enforcement

- **Original increment or report:** Remote-enforcement portion of
  `meta-05-F2` and PRA-013.
- **Current status:** Still valid. Hosted workflow execution is already
  resolved, but enforcement and human redundancy are not.
- **Severity / category:** Medium / Developer experience.
- **Why it matters:** One human identity owns code, reviews, security decisions,
  and continuity.
- **Current evidence:** `git shortlog` shows one human maintainer identity plus
  Dependabot. CODEOWNERS assigns every path to `@SillyRbbit`. PR #19 proves
  hosted checks run, but GitHub protection and ruleset APIs return HTTP 403
  because the private repository lacks the required plan feature.
- **Impact if ignored:** Review, release, incident response, and support depend
  on one person, while passing checks are procedural rather than enforced.
- **Recommended remediation / effort:** Add a reviewed backup owner, document
  support continuity, and adopt an available protected-merge mechanism or
  explicit manual enforcement protocol. Effort: Medium.
- **Dependencies:** Team and repository-plan decisions.
- **Regression risks:** Nominal reviewers without real knowledge can create
  false assurance; rigid policy can block emergency maintenance without a
  reviewed override.
- **Blocks next product increment:** No.
- **Recommended milestone:** Before external support commitments.

### ARB-021 - Create executive materials only from verified pilot evidence

- **Original increment or report:** PRA-014 and the stopped Meta Increment 4
  request.
- **Current status:** Still valid.
- **Severity / category:** Medium / Documentation.
- **Why it matters:** Sponsorship and ROI decisions need a verified use case,
  measurement inputs, and clear current-versus-planned claims.
- **Current evidence:** No `docs/executive/` directory exists. The stopped Meta
  4 request has no gate or repository implementation. The Product Readiness
  Audit remains `NOT READY (57/100)`.
- **Impact if ignored:** Leadership material may overstate mocked behavior or
  invent ROI and compliance claims.
- **Recommended remediation / effort:** Prepare a factual one-pager only from
  current evidence; defer the full boardroom package until one live workflow
  and pilot measurement plan exist. Effort: Medium.
- **Dependencies:** ARB-004 and approved pilot metrics.
- **Regression risks:** Copy can silently promote planned or mocked capabilities
  to current claims.
- **Blocks next product increment:** No.
- **Recommended milestone:** Pilot sponsorship.

### ARB-022 - Reconcile project memory after Meta 7 publication

- **Original increment or report:** Current recurrence of PRA-016 after Meta 7
  publication; not a rewrite of the dated Meta 7 review.
- **Current status:** Resolved in the current documentation workspace; resolving
  commit pending until committed.
- **Severity / category:** Low / Documentation.
- **Why it matters:** Repository rules select work from project memory, but live
  documents directed publication of already merged work and described 4V as
  blocked by that completed publication.
- **Current evidence:** PR #21 squash-merged the original advisory backlog and
  reconciliation at `cc434d9`, but four live authorities retained their
  pre-publication instructions. This remediation records the merge and removes
  those instructions. The focused stale scan has no post-edit matches, and the
  exact protected product and historical paths are unchanged.
- **Impact if ignored:** A future session can repeat publication or incorrectly
  refuse the next approved work.
- **Recommended remediation / effort:** Completed one documentation-only
  post-publication reconciliation while preserving dated reports as historical
  evidence. Effort: Small.
- **Dependencies:** The merged PR #21 state and the verified repository gate.
- **Regression risks:** Bulk replacement could rewrite historical evidence or
  falsely mark 4V implemented or approved; exact protected-path checks passed.
- **Blocks next product increment:** No after the remediation is present on clean
  synchronized `main`. Separate owner approval for 4V still applies.
- **Recommended milestone:** Resolved before 4V; publication of the resolving
  commit remains pending.

### ARB-023 - Retain or separately remediate the raw development icon exception

- **Original increment or report:** D-051 and `meta-07-F1`.
- **Current status:** Still valid as an explicitly accepted baseline.
- **Severity / category:** Advisory / UX.
- **Why it matters:** The raw unbundled developer process does not present the
  product identity even though packaged apps do.
- **Current evidence:** The valid Meta 7 report records the generic macOS
  `exec` icon for `tauri dev`; debug and release `.app` bundles use Cortexa.
- **Impact if ignored:** Developer screenshots or testing can look inconsistent,
  but production identity is unaffected.
- **Recommended remediation / effort:** Keep the exception documented, or
  investigate only in a separate developer-experience increment. Effort:
  Medium.
- **Dependencies:** A bounded design that does not alter production bundle
  identity or permissions.
- **Regression risks:** A cosmetic fix can expand into runtime/configuration
  changes or destabilize normal development.
- **Blocks next product increment:** No.
- **Recommended milestone:** Optional developer-experience work.

### ARB-024 - Replace the opaque brand source only with owner authority

- **Original increment or report:** `meta-01-F1` and `meta-07-F4`.
- **Current status:** Still valid; the Meta 1 source is a duplicate of the
  current Meta 7 constraint.
- **Severity / category:** Advisory / Technical debt.
- **Why it matters:** The near-white protected field remains visible on dark
  surfaces, and fine circuit detail loses clarity at small sizes.
- **Current evidence:** The canonical source hash remains
  `e31345045817f040c9fc664d4dc090a2002a1f6d0676c870df2c7e14885afaec`, matching
  the reviewed opaque-raster evidence.
- **Impact if ignored:** Brand rendering remains less flexible, but identity
  and current package correctness remain intact.
- **Recommended remediation / effort:** Do not redraw or infer transparency.
  Replace only if the owner supplies an approved vector or transparent master.
  Effort: Medium.
- **Dependencies:** Project-owner brand asset.
- **Regression risks:** Automated recolor, crop, tracing, or transparency can
  materially alter the official identity.
- **Blocks next product increment:** No.
- **Recommended milestone:** Optional future brand-source revision.

### ARB-025 - Preserve the project-hook trust boundary

- **Original increment or report:** `04g-F1` and D-028.
- **Current status:** Still valid as an intentional operating boundary.
- **Severity / category:** Advisory / Security.
- **Why it matters:** Repository-local hooks execute with operator trust and can
  be changed or disabled; their marker cannot be authorization or proof that
  commands ran.
- **Current evidence:** `.codex/hooks.json`, hook skills, `SECURITY.md`, and the
  28 passing hook tests preserve explicit bypass disclosure and loop prevention.
- **Impact if ignored:** Teams could mistake a workflow guardrail for an
  unbypassable security or audit control.
- **Recommended remediation / effort:** No code remediation. Continue trust
  review, bypass recording, and full-gate reruns. Effort: Small.
- **Dependencies:** Normal Codex project trust and operator discipline.
- **Regression risks:** Attempting to make the hook appear authoritative can
  overclaim security or grant it unnecessary permissions.
- **Blocks next product increment:** No.
- **Recommended milestone:** Every Codex session using repository hooks.

## Historical disposition records

### ARB-026 - Increment 4I recovery prerequisite

- **Original:** `04j-F1`.
- **Status / severity / category:** Already resolved / Advisory / Developer
  experience.
- **Why, evidence, and ignored impact:** 4I could not be reconstructed before
  4J. Both were later reconstructed, verified, published, and merged; repeating
  the old block would halt valid work.
- **Remediation / effort:** None remaining / Small historical recovery.
- **Dependencies / regression / next block / milestone:** Corrected gate was the
  dependency; no current regression or next-increment block; completed in 4I
  reconstruction.

### ARB-027 - Deferred production icon rollout

- **Original:** `meta-01-F2` and PRA-012.
- **Status / severity / category:** Already resolved / Low / UX.
- **Why, evidence, and ignored impact:** The old icon family lacked the official
  source. Meta 7 replaced exactly 16 outputs, passed native/package checks, and
  is merged at `96ba6ae`; carrying the finding forward would be false.
- **Remediation / effort:** None; preserve Meta 7 evidence / completed Medium.
- **Dependencies / regression / next block / milestone:** D-051/D-052 remain;
  no block; Meta 7 complete.

### ARB-028 - Meta 7 rebase and reverification

- **Original:** `repo-dependency-baseline-compatibility-F3`.
- **Status / severity / category:** Already resolved / Advisory / Reliability.
- **Why, evidence, and ignored impact:** The old branch targeted a broken
  baseline. Meta 7 was reconstructed from repaired `b298999`, fully reverified,
  force-updated, and merged at `96ba6ae`; repeating recovery risks overwriting
  the preserved branch.
- **Remediation / effort:** None / completed Small.
- **Dependencies / regression / next block / milestone:** Preserved branch
  remains at `a1808e2`; no current block; Meta 7 publication complete.

### ARB-029 - Approval presentation had no resolution path

- **Original:** `04s-F2`.
- **Status / severity / category:** Already resolved / Advisory / Architecture.
- **Why, evidence, and ignored impact:** 4S could issue but not resolve a
  presentation. 4T added same-manager trusted-source resolution and 4U added run
  termination; treating it as open would duplicate later work.
- **Remediation / effort:** None / completed Medium.
- **Dependencies / regression / next block / milestone:** Active residuals map
  to ARB-001, ARB-005, and ARB-011; no separate block; Phase 4T/4U complete.

### ARB-030 - Late Meta 6 gate initialization

- **Original:** `meta-06-F7`.
- **Status / severity / category:** Already resolved / Advisory / Developer
  experience.
- **Why, evidence, and ignored impact:** The one-time audit began without gate
  state, then reran closeout checks and recorded the timing. Later implementation
  work, including Meta 7, began its gate before edits.
- **Remediation / effort:** None beyond existing workflow / Small.
- **Dependencies / regression / next block / milestone:** Mandatory gate rule
  remains; no current block; historical Meta 6 closeout.

### ARB-031 - Byte-variable ICNS regeneration

- **Original:** `meta-07-F3`.
- **Status / severity / category:** Already resolved / Advisory / Testing.
- **Why, evidence, and ignored impact:** Byte-only regeneration is flaky.
  D-052 now requires complete decoded-representation equality and exact
  repository-to-bundle bytes.
- **Remediation / effort:** Preserve D-052 / Small.
- **Dependencies / regression / next block / milestone:** Future icon tooling
  must retain semantic comparison; no current block; resolved in Meta 7.

### ARB-032 - Unsupported external Rust API compatibility

- **Original:** `04i-F2`, `04k-F2`, `04l-F2`, `04m-F2`, `04o-F1`, `04p-F1`,
  and `04q-F1`.
- **Status / severity / category:** No longer relevant / Advisory / Technical
  debt.
- **Why, evidence, and ignored impact:** The findings assumed a theoretical
  unsupported external crate consumer. `src-tauri/Cargo.toml` has
  `publish = false`, no tag exists, and repository search shows no consumer.
- **Remediation / effort:** None; review API stability only if publication is
  separately proposed / Small.
- **Dependencies / regression / next block / milestone:** A future crate
  publication decision would reopen it; no current block; pre-publication only.

### ARB-033 - Historical icon increment numbering

- **Original:** `meta-02-F1`.
- **Status / severity / category:** No longer relevant / Advisory /
  Documentation.
- **Why, evidence, and ignored impact:** Historical records retain numbers that
  were correct at their checkpoints. D-044 through D-050 preserve the
  renumbering trail, and Meta 7 is complete.
- **Remediation / effort:** Do not rewrite historical evidence / Small.
- **Dependencies / regression / next block / milestone:** Current live docs must
  be accurate through ARB-022; no independent block; historical record only.

### ARB-034 - Missing post-increment gate for 4F

- **Original:** deferred 4F plan/increment entries and D-027.
- **Status / severity / category:** Already resolved / Advisory / Developer
  experience.
- **Why, evidence, and ignored impact:** 4F received a one-time exception because
  the skill did not exist. 4G created the gate and every later implementation
  uses it.
- **Remediation / effort:** None / completed Medium.
- **Dependencies / regression / next block / milestone:** D-027 cannot be
  reused; no block; Workflow Increment 4G complete.

### ARB-035 - Early shell and frontend listener deferrals

- **Original:** Increment 1 deferrals and the 2D frontend-listener deferral.
- **Status / severity / category:** Superseded / Low / Architecture.
- **Why, evidence, and ignored impact:** SQLite, menu lifecycle, agent
  contracts, mock interaction, sidebar, approvals, Settings, Permission Center,
  and native menu listeners were implemented in later increments. A global
  shortcut is not a current MVP requirement.
- **Remediation / effort:** None as one finding / completed or future-specific.
- **Dependencies / regression / next block / milestone:** Privileged features
  retain separate security gates; no block; Phases 1-3.

### ARB-036 - Deferred LocalAuthentication

- **Original:** 4E plan/increment deferral.
- **Status / severity / category:** No longer relevant to the current registered
  action / Advisory / Security.
- **Why, evidence, and ignored impact:** `create_local_task@1` is reversible and
  requires no permission; D-025 explicitly records authentication as
  `NotEvaluated`. Adding biometrics now would grant no justified authority.
- **Remediation / effort:** Reopen only for a later policy that explicitly
  requires exact-subject device-owner authentication / Medium.
- **Dependencies / regression / next block / milestone:** Future high-risk tool
  and threat model; no current block; before such execution.

### ARB-037 - Deferred 4C canonicalization and binding details

- **Original:** 4C plan/increment deferred serialization, hashing, intent,
  permission, scope, and run binding.
- **Status / severity / category:** Superseded / Advisory / Architecture.
- **Why, evidence, and ignored impact:** 4D through 4U added exact ownership,
  run/request/call identity, preview, expiry, one-time consumption, trusted
  source, and cancellation. D-024 rejects an unnecessary digest; serialization
  remains unnecessary for same-process authority.
- **Remediation / effort:** Track only concrete residuals in ARB-002 and
  ARB-005 / Medium. ARB-001 is resolved.
- **Dependencies / regression / next block / milestone:** Do not add hashes as
  authority; no separate block; Phase 4.

### ARB-038 - Hosted GitHub behavior remained unverified

- **Original:** `meta-05-F2`.
- **Status / severity / category:** Superseded / Advisory / Developer
  experience.
- **Why, evidence, and ignored impact:** PR #19 proves CI, Documentation, and
  Security jobs execute successfully. Remote enforcement remains unresolved and
  is now ARB-020.
- **Remediation / effort:** No further hosted-execution proof; remediate
  enforcement and redundancy under ARB-020 / Medium.
- **Dependencies / regression / next block / milestone:** GitHub plan/team; no
  4V block; repository governance.

### ARB-039 - Broad production-provider deferral

- **Original:** `04k-F1`.
- **Status / severity / category:** Superseded / Advisory / Architecture.
- **Why, evidence, and ignored impact:** 4N through 4U created more precise
  transport-free request and turn boundaries. The remaining live-transport risk
  is ARB-002.
- **Remediation / effort:** Follow ARB-002 rather than restore the deleted
  stringly provider scaffold / Large.
- **Dependencies / regression / next block / milestone:** O-006/O-007; blocks
  live networking, not 4V; live gateway architecture.

### ARB-040 - Broad capability-specific platform deferral

- **Original:** `04m-F1`.
- **Status / severity / category:** Superseded / Advisory / Architecture.
- **Why, evidence, and ignored impact:** The generic platform scaffold was
  intentionally removed. Current remediation should begin with one exact
  restricted executor under ARB-003, not a new generic adapter.
- **Remediation / effort:** Follow ARB-003 / Large.
- **Dependencies / regression / next block / milestone:** First approved tool;
  blocks functional pilot, not 4V; controlled workflow prototype.

### ARB-041 - Pre-4U incomplete approval lifecycle

- **Original:** `04t-F2` and part of `04u-F2`.
- **Status / severity / category:** Superseded / Advisory / Reliability.
- **Why, evidence, and ignored impact:** 4U added terminal run cancellation.
  Remaining audit, durability, stale-dialog, coordination, and execution gaps
  were split into ARB-001, ARB-005, ARB-011, ARB-003, and ARB-004. Increment
  4V resolves ARB-001; the other records remain current.
- **Remediation / effort:** Use the split current records / Medium to Large.
- **Dependencies / regression / next block / milestone:** 4U and 4V complete in
  the verified workspace; remaining records govern later pilot work.

### ARB-042 - Meta 6 queue/publication drift

- **Original:** PRA-016.
- **Status / severity / category:** Superseded / Low / Documentation.
- **Why, evidence, and ignored impact:** Meta 6 fixed the earlier Meta 5 and
  numbering conflict. A later post-Meta-7 publication drift was recorded as
  ARB-022 and is now resolved in the current workspace.
- **Remediation / effort:** Completed through ARB-022 / Small.
- **Dependencies / regression / next block / milestone:** Preserve historical
  evidence; no current block after the remediation reaches clean synchronized
  `main`; immediate closeout.

### ARB-043 - Combined non-functional test gap

- **Original:** PRA-009.
- **Status / severity / category:** Superseded / Medium / Testing.
- **Why, evidence, and ignored impact:** The combined finding obscures different
  root causes. Current records split composition/recovery tests (ARB-017),
  performance evidence (ARB-018), accessibility evidence (ARB-019), and DMG
  installer validation (ARB-010).
- **Remediation / effort:** Follow the split records / Medium to Large.
- **Dependencies / regression / next block / milestone:** Depends on each live
  boundary; no 4V block; pilot/release gates.

### ARB-044 - Combined release and enterprise readiness finding

- **Original:** `meta-06-F6` and PRA-005.
- **Status / severity / category:** Superseded / High / Architecture.
- **Why, evidence, and ignored impact:** Legal, release engineering, installer,
  and enterprise controls have different owners and failure modes. One broad
  fix would be unreviewable.
- **Remediation / effort:** Follow ARB-006, ARB-007, ARB-008, and ARB-010 /
  Large.
- **Dependencies / regression / next block / milestone:** Pilot and owner
  decisions; no 4V block; release and enterprise milestones.

### ARB-045 - Attachments, voice, global shortcuts, and privileged feature deferrals

- **Original:** Phase 1, 2D, 3A, and 3B plan deferrals.
- **Status / severity / category:** No longer relevant as one remediation item /
  Low / UX.
- **Why, evidence, and ignored impact:** Attachments, voice, broad Accessibility,
  screen capture, Apple Events, microphone, and global shortcuts are current
  non-goals or require independent threat models. Trusted context selection is
  already represented by ARB-004/ARB-005.
- **Remediation / effort:** Do not bundle these features; create an exact future
  increment only after owner selection / Large.
- **Dependencies / regression / next block / milestone:** Product selection and
  security approval; no current block; future roadmap only.

## Source finding traceability

### Post-increment findings

| Source finding                            | Current status     | Canonical record              |
| ----------------------------------------- | ------------------ | ----------------------------- |
| 04g-F1                                    | Still valid        | ARB-025                       |
| 04h-F1                                    | Duplicate          | ARB-005                       |
| 04i-F1                                    | Duplicate          | ARB-005                       |
| 04i-F2                                    | No longer relevant | ARB-032                       |
| 04j-F1                                    | Already resolved   | ARB-026                       |
| 04k-F1                                    | Superseded         | ARB-039, then ARB-002         |
| 04k-F2                                    | No longer relevant | ARB-032                       |
| 04l-F1                                    | Still valid        | ARB-005                       |
| 04l-F2                                    | No longer relevant | ARB-032                       |
| 04m-F1                                    | Superseded         | ARB-040, then ARB-003         |
| 04m-F2                                    | No longer relevant | ARB-032                       |
| 04n-F1                                    | Still valid        | ARB-002                       |
| 04n-F2                                    | Still valid        | ARB-002                       |
| 04o-F1                                    | No longer relevant | ARB-032                       |
| 04o-F2                                    | Still valid        | ARB-015                       |
| 04p-F1                                    | No longer relevant | ARB-032                       |
| 04p-F2                                    | Duplicate          | ARB-015                       |
| 04q-F1                                    | No longer relevant | ARB-032                       |
| 04q-F2                                    | Duplicate          | ARB-015                       |
| 04r-F1                                    | Still valid        | ARB-016                       |
| 04r-F2                                    | Duplicate          | ARB-015 and ARB-016           |
| 04s-F1                                    | Duplicate          | ARB-016                       |
| 04s-F2                                    | Already resolved   | ARB-029                       |
| 04t-F1                                    | Duplicate          | ARB-016                       |
| 04t-F2                                    | Superseded         | ARB-041                       |
| 04u-F1                                    | Duplicate          | ARB-016                       |
| 04u-F2                                    | Duplicate          | ARB-001, ARB-005, and ARB-011 |
| meta-01-F1                                | Duplicate          | ARB-024                       |
| meta-01-F2                                | Already resolved   | ARB-027                       |
| meta-02-F1                                | No longer relevant | ARB-033                       |
| meta-05-F1                                | Duplicate          | ARB-009                       |
| meta-05-F2                                | Superseded         | ARB-038, then ARB-020         |
| meta-06-F1                                | Duplicate          | ARB-001                       |
| meta-06-F2                                | Duplicate          | ARB-002                       |
| meta-06-F3                                | Duplicate          | ARB-009                       |
| meta-06-F4                                | Duplicate          | ARB-004                       |
| meta-06-F5                                | Duplicate          | ARB-011 and ARB-019           |
| meta-06-F6                                | Superseded         | ARB-044                       |
| meta-06-F7                                | Already resolved   | ARB-030                       |
| meta-07-F1                                | Still valid        | ARB-023                       |
| meta-07-F2                                | Still valid        | ARB-010                       |
| meta-07-F3                                | Already resolved   | ARB-031                       |
| meta-07-F4                                | Still valid        | ARB-024                       |
| repo-dependency-baseline-compatibility-F1 | Duplicate          | ARB-009                       |
| repo-dependency-baseline-compatibility-F2 | Still valid        | ARB-013                       |
| repo-dependency-baseline-compatibility-F3 | Already resolved   | ARB-028                       |

### Product Readiness Audit findings

| Source finding | Current status   | Canonical record                       |
| -------------- | ---------------- | -------------------------------------- |
| PRA-001        | Still valid      | ARB-004                                |
| PRA-002        | Already resolved | ARB-001                                |
| PRA-003        | Still valid      | ARB-003                                |
| PRA-004        | Still valid      | ARB-002                                |
| PRA-005        | Superseded       | ARB-006, ARB-007, and ARB-010          |
| PRA-006        | Still valid      | ARB-009                                |
| PRA-007        | Still valid      | ARB-005                                |
| PRA-008        | Still valid      | ARB-019                                |
| PRA-009        | Superseded       | ARB-010, ARB-017, ARB-018, and ARB-019 |
| PRA-010        | Still valid      | ARB-011                                |
| PRA-011        | Still valid      | ARB-012                                |
| PRA-012        | Already resolved | ARB-027                                |
| PRA-013        | Still valid      | ARB-020                                |
| PRA-014        | Still valid      | ARB-021                                |
| PRA-015        | Still valid      | ARB-008                                |
| PRA-016        | Already resolved | ARB-042, then resolved ARB-022         |
| PRA-017        | Still valid      | ARB-014                                |
| PRA-018        | Still valid      | ARB-007                                |

### Plan and increment deferral screening

| Historical marker group                                                                  | Disposition                                                                                     |
| ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| Increment 1 deferred SQLite, menu, shell, UI, approvals, Settings, and Permission Center | Implemented later; ARB-035.                                                                     |
| 2D deferred frontend listener and global shortcut                                        | Listener implemented; global shortcut is not a current requirement; ARB-035/ARB-045.            |
| 3A attachments and voice                                                                 | Current non-goals pending separate threat models; ARB-045.                                      |
| 3B context selector and trusted provenance                                               | Still absent, but duplicate of production composition and data-lifecycle work; ARB-004/ARB-005. |
| 4C serialization, hashing, intent, permission, scope, and run binding                    | Superseded by 4D-4U and accepted no-digest decisions; ARB-037.                                  |
| 4E LocalAuthentication                                                                   | Not justified for the current reversible tool; ARB-036.                                         |
| 4F post-increment gate                                                                   | One-time exception closed by 4G; ARB-034.                                                       |
| 4H durable audit architecture                                                            | Duplicate of ARB-005.                                                                           |
| 4M capability-specific platform design                                                   | Superseded by exact executor/platform record ARB-003 via ARB-040.                               |
| Meta 1 production icons                                                                  | Completed by Meta 7; ARB-027.                                                                   |
| Meta 7 deferred audit-era numbering                                                      | Historical only; later publication drift was resolved by ARB-022.                               |

## Counts

Historical source findings reviewed: **64**.

| Current source disposition |  Count |
| -------------------------- | -----: |
| Still valid                |     23 |
| Already resolved           |      8 |
| Superseded                 |      8 |
| Duplicate                  |     17 |
| No longer relevant         |      8 |
| **Total**                  | **64** |

Superseded or duplicated source findings: **25**. At the original review,
root-cause grouping and splitting over-broad findings produced 25 still-valid
remediation records. After the bounded ARB-022 and ARB-001 resolutions, the
authoritative current backlog contains **23 unresolved records and 2 resolved
records**.

## Top five advisories to remediate

1. **ARB-002:** decide gateway identity, credentials, deployment, provider
   retention, and disclosure before live traffic.
2. **ARB-003:** design one restricted executor and exact platform
   implementation.
3. **ARB-004:** compose one controlled end-to-end workflow in separately
   verified increments.
4. **ARB-005:** define durable audit and product-data lifecycle, encryption,
   retention, and recovery.
5. **ARB-006:** select and record the repository license before distribution.

## First recommended remediation increment

**Planning remediation:** define the exact gateway identity, desktop credential,
gateway deployment, provider retention, and disclosure threat model required by
ARB-002. No ARB-002 implementation increment is Ready. O-006 and O-007 require
security and executive decisions before live transport can be scoped safely.

Increment 4V / ARB-001 is published on clean synchronized `main`. A
documentation-only ARB-002 planning increment may begin only after separate
project-owner, security-owner, and executive-owner approval.

## Ready-to-paste Codex prompt

```text
Use $session-start.

Start from clean synchronized main after the verified Increment 4V / ARB-001 remediation is published. Reconcile the actual repository and valid 04v completion marker. Plan only the smallest documentation-only ARB-002 threat-model increment needed to decide gateway identity, desktop credential ownership, gateway deployment, provider retention, and user disclosure under O-006 and O-007. State exact files, risks, non-goals, verification, decisions required, and rollback. Update planning documentation only, then wait for project-owner, security-owner, and executive-owner approval. Do not begin a product gate, implement transport, add credentials, commit, push, merge, or start another remediation.
```

## 2026-07-19 O-006/O-007 decision-record update

D-060 now records Cortexa AI as the planned gateway operator, Azure Container
Apps in Central US as the planned hosting target, the reserved inactive
`https://api.cortexaai.io` origin, a provider-neutral consumer/prosumer Phase 1,
and a later Entra-capable enterprise Phase 2. Microsoft, Google, and Apple are
candidate Phase 1 providers only. O-006 remains open for the exact first
provider or providers and issuer configuration.

The approved provider-boundary amendment distinguishes three independent
categories. Identity uses a pluggable OAuth/OIDC boundary with consumer
candidates in Phase 1 and compatible enterprise OIDC/SAML providers in Phase 2.
Hosting remains one-primary-cloud Azure-first, with AWS and Google Cloud
portability deferred and no active-active or three-cloud claim. A future trusted
`AgentProvider` abstraction may route only to separately approved AI model
providers; no implementation exists, and every provider requires its own O-007
retention, ZDR, data-use, logging, region, and security evidence. O-006 therefore
remains open for exact identity and AI-provider configurations and any future
cloud expansion.

D-061 accepts O-007's cross-phase product policy: provider-approved ZDR is
required before real user content; pre-verification testing is synthetic only;
initial post-verification data is explicitly submitted, non-sensitive text;
sensitive categories and content logging are prohibited; operational metadata
retention is capped at seven days; and disclosure precedes the first
transmission and remains visible in Settings. Provider ZDR approval and exact
configuration evidence are still pending.

ARB-002 remains High, `DECISION REQUIRED`, and unresolved. It does not affect a
currently reachable external path because no gateway, network client, identity
integration, credential loader, or provider transport exists. It continues to
block authentication and live model networking until O-006 closes and the ZDR,
deployment, disclosure, threat-model, and security-verification gates pass.
This additive update changes no original finding, count, severity, historical
disposition evidence, source code, or product behavior.

## 2026-07-19 Phase 1 identity selection update

D-062 selects Microsoft personal identity as the sole Phase 1 identity
provider. Google is deferred until demonstrated demand after Microsoft
verification; Apple is deferred until Mac App Store planning or demonstrated
demand. The planned personal-account-only flow uses a separate public desktop
client and gateway API resource, system-browser Authorization Code Flow with
PKCE S256, `state`, OIDC `nonce`, and a closed loopback callback. Initial scopes
are `openid`, `email`, and one exact delegated Cortexa gateway scope.
`offline_access`, persistent sessions, Microsoft Graph scopes, workforce
tenants, and automatic email linking remain excluded.

ARB-002 remains High and unresolved. No identity implementation exists, and
the exact client, resource, issuer, tenant, audience, redirect, scope,
account-lifecycle, and threat-model evidence remains pending. O-006 also remains
open for the AI-provider configuration, and D-061 provider-specific evidence
still blocks external transmission. This update changes no original finding,
count, severity, source code, or product behavior.

## 2026-07-19 Phase 1 AI-provider selection update

D-063 selects Azure OpenAI in Microsoft Foundry as the sole Phase 1
synthetic-evaluation candidate. It does not deploy a resource, enable traffic,
or satisfy D-061. The exact Azure resource, deployment, endpoint, model,
version, region, `ContentLogging=false`, stateless Responses behavior, access,
logging, deletion, disclosure, and security evidence remain mandatory before
real user content. Direct OpenAI and other providers remain separately approved
future adapters, and automatic fallback is prohibited.

ARB-002 remains High, unresolved, and not Ready. This additive update changes
no original finding, count, severity, source code, or product behavior.

## 2026-07-19 ARB-002A staged threat-model update

D-064 closes the pre-implementation configuration and threat-model design
without representing operational evidence as present. It separates Stage A
documentation, Stage B no-traffic registration and Azure provisioning, Stage C
synthetic-only authenticated transport, and Stage D real-content activation.
No stage authorizes or automatically starts the next.

The closed design uses separate Microsoft personal desktop and gateway API
registrations, `api://<gateway-api-client-id>`, delegated scope
`gateway.access`, and a `127.0.0.1` ephemeral callback at `/oauth/callback`.
The Azure design uses a dedicated non-shared user-assigned managed identity,
exact-resource inference RBAC, a private Azure OpenAI endpoint, disabled
provider public network access, restricted egress, and no API-key or provider
fallback. Disclosure requires versioned acknowledgement before first external
transmission, including synthetic traffic.

ARB-002 remains High and unresolved. Exact registrations, token claims, Azure
resources, DNS/TLS, provider-approved ZDR, `ContentLogging=false`, stateless
Responses, disclosure copy and storage, security tests, operational evidence,
and separate Stage B through Stage D approvals remain absent. Microsoft's
documented default token lifetime also exceeds the accepted 15-minute maximum,
and the manifest-based IP-literal ephemeral callback remains unproven. Both are
hard Stage C evidence gates. This additive update changes no original finding
count or severity and no product source, dependency, configuration, capability,
permission, data, or runtime behavior.

## Review and resolution boundary and rollback

The original review started no remediation and changed no source, dependency,
configuration, capability, permission, database, icon, or existing
documentation file. It created only this report.

The later ARB-022 resolution changed exactly the ten documentation paths in
`docs/increments/remediation-ARB-022-project-memory-reconciliation.md` and was
squash-merged at `7c79e65`.

The ARB-001 resolution changed exactly the 19 paths recorded in
`docs/increments/remediation-ARB-001-terminal-approval-audit.md` and was
squash-merged through PR #23 at `6e6f91d`. Revert only that bounded squash
commit. Preserve original commit `3440ce9` on the pre-refresh branch. No
migration, data, dependency, credential, configuration, capability, permission,
or remote-resource rollback applies.
