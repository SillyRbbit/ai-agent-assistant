# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

Repository Workflow Increment trusted self-hosted runner routing is verified
complete with advisories under
`docs/plans/repository-self-hosted-runner.md`. The exact workflow, repository
policy, regression-test, and operating-guide changes pass complete local
verification. The separately approved two-file portability correction is pushed
at `1621a55` on PR #24. CI, Documentation, and Security pass on runner 21, and
the completion marker is valid. Final documentation publication and PR merge
remain separately approval-gated.

The Repository Dependency Baseline Compatibility Repair is verified, published,
and squash-merged through PR #20 at `b298999` under
`docs/plans/repository-dependency-baseline-compatibility.md`.

Meta Increment 7 is verified complete with advisories on repaired `b298999`.
Its reconstructed scope was squash-merged through PR #19 at `96ba6ae` after
hosted CI, documentation, and security checks passed. The `meta-07` marker was
complete and valid on clean `96ba6ae` before the later advisory report changed
the live workspace fingerprint.

The advisory backlog and first post-Meta-7 project-memory reconciliation were
squash-merged through PR #21 at `cc434d9`. Remediation ARB-022 closes the
remaining live publication drift as documentation-only work under
`docs/increments/remediation-ARB-022-project-memory-reconciliation.md`. It is
verified and squash-merged through PR #22 at `7c79e65`.

Meta Increment 5 repository health and GitHub hygiene is verified complete,
published, and squash-merged at `6b149fa` under
`docs/plans/meta-05-repository-health.md`; its completion marker was valid on
clean `6b149fa` before the audit edits.

Meta Increment 3 Codex automation and post-increment quality gates is verified
complete and squash-merged at `ad9042c` under
`docs/plans/meta-03-codex-automation.md`.

Meta Increment 1 branding and identity foundation is verified complete and
squash-merged at `5edbf4d` under
`docs/plans/meta-01-branding-foundation.md`.

Meta Increment 6 is the documentation-only Product Readiness Audit recorded at
`docs/reviews/2026-07-16-product-readiness-audit.md`. Its result is **NOT READY
(57/100)**, it is squash-merged at `5281fac`, and its completion marker was valid
on that clean baseline before Meta 7 planning edits. It starts no implementation
plan. The repository Stop hook required late
`meta-06` gate initialization and a consolidated closeout report; that workflow
addition changes no product source or audit conclusion.

Meta Increment 7 verified application icon rollout is complete under
`docs/plans/meta-07-verified-application-icon-rollout.md`. Exactly the existing
16 Tauri icon files derive from the canonical source; debug and release app
bundles use Cortexa, while D-051 preserves the approved raw `tauri dev` generic
icon advisory without expanding source or configuration scope.

The stopped Meta Increment 4 executive-document request created no gate state,
plan, repository edit, or completion evidence. D-048 and D-049 record the queue
history.

Increment 4V terminal approval audit is verified on open PR #23 under
`docs/plans/04v-bind-initial-terminal-approval-audit.md`. Its merged 4U
prerequisite and ARB-022 publication are satisfied. Commit `3440ce9` preserves
the verified scope and valid `04v` marker. Publication is blocked until the
self-hosted workflow increment provides usable remote checks; no later product
or remediation increment may start.

## Completed plans

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
docs/plans/02c-storage-startup.md
docs/plans/02d-menu-bar-window-lifecycle.md
docs/plans/02e-react-application-shell.md
docs/plans/02f-mocked-assistant-interaction-shell.md
docs/plans/02g-integration-hardening.md
docs/plans/03a-in-memory-conversation-sessions.md
docs/plans/03b-mock-context-provenance.md
docs/plans/03c-simulated-tool-result.md
docs/plans/03d-bounded-mock-loop-completion.md
docs/plans/04a-gateway-protocol-contract.md
docs/plans/04b-local-tool-schema-validation.md
docs/plans/04c-trusted-policy-input-binding.md
docs/plans/04d-exact-approval-binding.md
docs/plans/04e-trusted-approval-decision-source.md
docs/plans/04i-remove-generic-audit-scaffold.md
docs/plans/04j-post-increment-deletion-fingerprint.md
docs/plans/04k-remove-legacy-provider-scaffold.md
docs/plans/04l-remove-legacy-memory-scaffold.md
docs/plans/04m-remove-legacy-platform-scaffold.md
docs/plans/04n-bounded-initial-gateway-request.md
docs/plans/04o-bound-initial-gateway-turn.md
docs/plans/04p-schema-bound-initial-gateway-events.md
docs/plans/04q-terminally-release-initial-function-call.md
docs/plans/04r-bind-terminal-initial-policy.md
docs/plans/04s-bind-terminal-initial-approval-presentation.md
docs/plans/04t-bind-terminal-initial-approval-resolution.md
docs/plans/04u-bind-initial-approval-run-termination.md
docs/plans/meta-01-branding-foundation.md
docs/plans/meta-02-engineering-operating-system.md
docs/plans/meta-03-codex-automation.md
docs/plans/meta-05-repository-health.md
docs/plans/repository-dependency-baseline-compatibility.md
docs/plans/meta-07-verified-application-icon-rollout.md
```

Increments 2C and 2D were verified on the Apple Silicon target Mac.

## Plan rules

A plan must contain:

- Goal and user-visible outcome.
- Scope and explicit non-goals.
- Existing behavior and constraints.
- Files expected to change.
- Ordered implementation steps.
- Security and privacy considerations.
- Tests and verification commands.
- Rollback or failure strategy.
- Exit criteria.
- Documentation updates.

## Plan status values

- **Draft** — still being designed.
- **Ready** — enough information exists to implement.
- **Active** — implementation is in progress or verification remains.
- **Blocked** — a prerequisite prevents progress.
- **Complete** — acceptance criteria and verification are complete.
- **Stopped** — explicitly halted before implementation and has no completion evidence.
- **Superseded** — replaced by another plan.

## Plan index

| Plan                                           | Status   | Owner              | Last updated |
| ---------------------------------------------- | -------- | ------------------ | ------------ |
| Increment 2B-1 SQLite migration skeleton       | Complete | Project maintainer | 2026-07-13   |
| Increment 2C storage startup integration       | Complete | Project maintainer | 2026-07-13   |
| Increment 2D menu-bar/window lifecycle         | Complete | Project maintainer | 2026-07-13   |
| Increment 2E React application shell           | Complete | Project maintainer | 2026-07-13   |
| Increment 2F mocked interaction shell          | Complete | Project maintainer | 2026-07-13   |
| Increment 2G integration hardening             | Complete | Project maintainer | 2026-07-13   |
| Increment 3A in-memory conversation sessions   | Complete | Project maintainer | 2026-07-13   |
| Increment 3B mock context provenance           | Complete | Project maintainer | 2026-07-13   |
| Increment 3C simulated tool result             | Complete | Project maintainer | 2026-07-13   |
| Increment 3D bounded mock-loop completion      | Complete | Project maintainer | 2026-07-14   |
| Increment 4A gateway protocol contract         | Complete | Project maintainer | 2026-07-14   |
| Increment 4B local tool-schema validation      | Complete | Project maintainer | 2026-07-14   |
| Increment 4C trusted policy-input binding      | Complete | Project maintainer | 2026-07-14   |
| Increment 4D exact approval binding            | Complete | Project maintainer | 2026-07-14   |
| Increment 4E trusted approval decision source  | Complete | Project maintainer | 2026-07-14   |
| Increment 4F Cortexa product display rename    | Complete | Project maintainer | 2026-07-14   |
| Workflow Increment 4G post-increment gate      | Complete | Project maintainer | 2026-07-14   |
| Increment 4H typed approval-audit adapter      | Complete | Project maintainer | 2026-07-15   |
| Increment 4I remove generic audit scaffold     | Complete | Project maintainer | 2026-07-15   |
| Workflow Increment 4J deletion fingerprint     | Complete | Project maintainer | 2026-07-15   |
| Increment 4K remove legacy provider scaffold   | Complete | Project maintainer | 2026-07-15   |
| Increment 4L remove legacy memory scaffold     | Complete | Project maintainer | 2026-07-15   |
| Increment 4M remove legacy platform scaffold   | Complete | Project maintainer | 2026-07-15   |
| Increment 4N bounded initial gateway request   | Complete | Project maintainer | 2026-07-15   |
| Increment 4O bound initial gateway turn        | Complete | Project maintainer | 2026-07-15   |
| Increment 4P schema-bound initial events       | Complete | Project maintainer | 2026-07-15   |
| Increment 4Q terminal initial function release | Complete | Project maintainer | 2026-07-15   |
| Increment 4R terminal initial policy binding   | Complete | Project maintainer | 2026-07-15   |
| Increment 4S terminal approval presentation    | Complete | Project maintainer | 2026-07-15   |
| Increment 4T terminal approval resolution      | Complete | Project maintainer | 2026-07-15   |
| Increment 4U approval run termination          | Complete | Project maintainer | 2026-07-15   |
| Increment 4V terminal approval audit binding   | Active   | Project maintainer | 2026-07-17   |
| Meta Increment 1 branding foundation           | Complete | Project maintainer | 2026-07-15   |
| Meta Increment 2 engineering operating system  | Complete | Project maintainer | 2026-07-15   |
| Meta Increment 3 Codex automation              | Complete | Project maintainer | 2026-07-15   |
| Meta Increment 4 executive documentation       | Stopped  | Project maintainer | 2026-07-16   |
| Meta Increment 5 repository health             | Complete | Project maintainer | 2026-07-16   |
| Meta Increment 6 product readiness audit       | Complete | Project maintainer | 2026-07-16   |
| Repository dependency baseline compatibility   | Complete | Project maintainer | 2026-07-16   |
| Meta Increment 7 application icon rollout      | Complete | Project maintainer | 2026-07-16   |
| ARB-022 memory reconciliation                  | Complete | Project maintainer | 2026-07-16   |
| Repository self-hosted runner routing          | Active   | Project maintainer | 2026-07-17   |

## Meta Increment 1 branding and identity foundation - complete

Goal: establish the owner-supplied Cortexa logo as the single authoritative
identity source and apply it to placeholder brand surfaces without changing
product behavior or compatibility identifiers.

Five canonical assets, six brand guides, the `$branding` skill, README/favicon
references, and the official sidebar mark are implemented. The primary, light,
and dark files preserve source bytes; favicon and future app-icon source use
proportional padding. Focused and complete checks, asset inspection, light/dark
and compact visual review, dependency audit, exact-scope review, and the
mandatory gate pass. Tauri production icons remain unchanged. D-043 records the
durable boundary.

## Meta Increment 2 engineering operating system - complete

Goal: establish one authoritative engineering handbook, current architecture,
normalized product requirements, roadmap, testing standard, security checklist,
and release process without changing product behavior. The exact approved scope,
conflicts, risks, verification, and rollback are frozen in
`docs/plans/meta-02-engineering-operating-system.md`.

The increment reconciles merged Meta Increment 1 at `5edbf4d`, documentation
authority, current versus planned capability, and Meta 2/3 numbering. It changes
no application source, tests, runtime, dependency, config, capability,
permission, SQLite schema, branding asset, icon, or compatibility identifier.

Rendered Markdown links, formatting, complete repository verification,
protected-path review, exact-scope review, documentation sync, code review,
security review, and the mandatory `meta-02` gate pass. No manual application
check applies.

## Meta Increment 3 Codex automation and post-increment quality gates - complete

Goal: extend the existing verified repository-local post-increment system with
shared safe inspection, deterministic session-end inventory, focused review
skills, matching prompts, and reusable templates without changing product
behavior or weakening explicit project-owner control.

The existing supported Stop definition remains unchanged. Shared safe
repository inspection, a read-only session-end inventory, 28 hook regressions,
eight validated review skills, matching prompts, templates, complete repository
verification, exact-scope review, documentation sync, and the mandatory
`meta-03` gate pass. No product or native manual check applies. The increment
was squash-merged at `ad9042c`. It cannot start another increment or publish
future changes without explicit project-owner direction.

## Meta Increment 5 repository health and GitHub hygiene - complete

Goal: establish a production-oriented repository surface with accurate entry
documentation, explicit contribution and licensing boundaries, owner review,
structured GitHub intake, review-only dependency proposals, least-privilege
quality workflows, reusable local repository checks, and release-note guidance
without changing application behavior.

The exact repository-governance scope, workflow constraints, RustSec baseline,
risks, non-goals, verification, and rollback are frozen in
`docs/plans/meta-05-repository-health.md`. Complete local verification, npm and
Rust audits, YAML and link validation, exact-scope review, documentation sync,
and the mandatory `meta-05` gate pass. No product or native manual check applies.

## Meta Increment 6 Product Readiness Audit - complete

Goal: assess current product maturity from repository and verification evidence,
score 16 readiness categories, classify findings, and order remediation without
changing application behavior. The result is **NOT READY (57/100)** and the
complete evidence, category assessments, findings, backlog, roadmap guidance,
verification, command log, and rollback are recorded in
`docs/reviews/2026-07-16-product-readiness-audit.md`.

The audit recommends Increment 4V as the smallest bounded remediation but does
not select or approve it. No source, test, dependency, workflow, configuration,
capability, permission, database, icon, identifier, commit, push, merge, or later
increment starts through this audit. The mandatory `meta-06` report records
`PASS WITH ADVISORIES` and next-increment readiness `Blocked`.

## Meta Increment 7 verified application icon rollout - complete

The exact 16-file plan is
`docs/plans/meta-07-verified-application-icon-rollout.md`. Every output derives
from the canonical 512 x 512 Cortexa source; dimensions, PNG alpha/color,
ICO/ICNS structure, embedded bundle resources, complete repaired-baseline
verification, and target-Mac application-icon inspection pass. D-051 records
the approved non-blocking raw `tauri dev` generic-icon exception. D-052 records
decoded-pixel validation for byte-variable ICNS regeneration while retaining
exact embedded-resource equality. Default DMG automation remains a release
advisory; debug and release `.app` bundles pass.

The verified scope was squash-merged through PR #19 at `96ba6ae` after all
hosted checks passed. Its dated plan, increment record, and post-increment report
remain unchanged as evidence of the verified pre-publication workspace.

## Remediation ARB-022 project-memory reconciliation - complete

Goal: remove stale live instructions to publish the advisory backlog and first
post-Meta-7 memory reconciliation after that exact documentation scope was
already squash-merged through PR #21 at `cc434d9`.

The remediation changes exactly eight live documentation authorities and adds
one increment record and one post-increment review. It records the actual merge,
removes completed publication work from the current queue, and left Increment
4V Ready at that checkpoint. Product source, tests, dependencies, configuration,
security boundaries, 4V plan/source/test/gate state at that checkpoint, and
dated Meta 7 evidence are unchanged.

Focused stale-instruction and protected-path assertions, formatting,
documentation, repository, security, full verification, diff review, and the
mandatory `remediation-arb-022` gate pass. No manual product check applies. PR
#22 squash-merged the resolving scope at `7c79e65`.

## Phase 4 Increment 4U bind initial approval run-termination - complete

Goal: let the bound initial turn terminally deny and consume the exact pending
approval it owns when a trusted future orchestrator reports run termination,
without accepting a caller-selected approval ID, choice, native result, or
interaction evidence.

The exact future source/test plan changes only `agent/gateway_request.rs` and the
public `gateway_request_contract` integration test. The turn privately retains
the manager-assigned ID after presentation and offers one idempotent operation
that delegates to the existing manager's `cancel_for_run_termination`. The
existing non-authorizing resolution, expiry precedence, replay prevention,
exact identity, and rejection of late native outcomes remain authoritative.

No native dialog invocation or closure, proactive expiry, timer, source trait,
runtime coordinator, active-run validation beyond a trusted cancellation call,
audit, persistence, dispatch, execution, continuation, transport,
authentication, credential, Tauri, frontend, SQLite, dependency, capability,
entitlement, or permission path is included. Focused and complete verification,
npm audit, scope and security review, documentation sync, and the mandatory
`04u` gate pass. No manual check applies. The implementation is committed as
`61525bf`, pushed on `codex/phase4-increment-4u`, fast-forward merged into
synchronized `main`, and retains a valid marker. Rollback after publication
reverts the bounded 4U commit.

## Phase 4 Increment 4V bind initial terminal approval audit - verified on PR

Goal: prevent a future initial-turn caller from receiving a successful native
or run-termination approval resolution unless the turn's private typed
in-memory audit adapter has validated and recorded that exact manager-owned
resolution first.

The exact source/test implementation changes only `agent/gateway_request.rs` and
the public `gateway_request_contract` integration test. It adds one private
turn-owned `InMemoryApprovalAuditAdapter`, one closed non-cloneable
resolution-plus-receipt value, and one manager-success-to-audit-success helper
used by both terminal paths. The receipt remains volatile, sequence-only, and
non-authorizing.

No durable audit persistence, SQLite, native invocation or closure, proactive
expiry, timer, runtime coordinator, active-run validation, dispatch, execution,
transport, credential, Tauri, frontend, dependency, capability, entitlement,
or permission path is included. Focused and complete verification, npm audit,
scope and security review, documentation sync, and the mandatory `04v` gate
passed before commit. No manual check applies. Commit `3440ce9` and its valid
marker remain on open PR #23. The hosted jobs failed before runner assignment;
publication waits for this self-hosted workflow increment and fresh remote
checks. No later product or remediation increment may start.

## Phase 4 Increment 4T bind terminal initial approval resolution - complete

Goal: prevent a future initial-turn caller from obtaining the turn-issued
presentation and sealed native source outcome without returning that outcome to
the exact private manager that owns the pending subject.

The exact source/test implementation changes only `agent/gateway_request.rs` and the
existing test-only helper in `approvals/decision_source.rs`. On macOS, the turn
consumes one sealed `TrustedApprovalSourceOutcome`, delegates it directly to
its existing private manager, and returns the exact owned non-authorizing
`ApprovalResolution` or existing typed approval error. Production native-source
behavior remains unchanged, and tests synthesize closed native results without
opening a dialog.

No native invocation, approval-manager/type change, run cancellation, proactive
expiry, audit, transport, gateway service, authentication, credentials,
continuation, runtime coordinator, dispatch, execution, Tauri, frontend,
SQLite, dependency, capability, entitlement, or permission path is included.
Eight request, 17 approval, nine public-contract, two approval-binding, and one
approval-audit tests pass, as do strict Clippy, complete repository verification,
and npm audit. Exact-scope, code, security, documentation, and mandatory gate
reviews pass with `PASS WITH ADVISORIES` and no manual gate. D-041 records the
durable boundary. Commit `244a1d8` is pushed, fast-forward merged into
synchronized `main`, and retained a valid `04t` marker immediately before 4U
planning edits.

## Phase 4 Increment 4S bind terminal initial approval presentation - complete

Goal: prevent a future initial-turn caller from receiving a terminal
`RequireApproval` decision and choosing, replacing, or omitting the verified
approval-manager transition.

The exact source/test implementation changes only `agent/gateway_request.rs` and the public
`gateway_request_contract` integration test. On accepted terminal completion,
the turn routes `RequireApproval` through its private fixed
`InMemoryApprovalManager`, creates one exact request, issues one owned
`ApprovalPresentation`, and returns that non-authorizing presentation event.
`Allow` and `Deny` remain non-authorizing policy events.

No approval-manager source, native-source, audit, transport, gateway service,
authentication, credentials, continuation, retries, deadlines, runtime
coordinator, dispatch, execution, Tauri, frontend, SQLite, dependency,
capability, entitlement, or permission path is included. The 72 focused request,
protocol, function-validation, policy, tool, approval, public-contract,
approval-binding, and approval-audit tests pass, as do strict Clippy, complete
repository verification, and npm audit. Exact-scope, code, security,
documentation, and mandatory gate reviews pass with `PASS WITH ADVISORIES` and
no manual gate. D-040 records the durable boundary. Commit `6d0bed4` is pushed on
`codex/phase4-increment-4s`, fast-forward merged into synchronized `main`, and
retained a valid `04s` marker before 4T planning edits.

## Phase 4 Increment 4R bind terminal initial function call to policy - complete

Goal: prevent the verified bound initial turn from releasing a standalone
schema-validated call before the canonical deterministic policy step.

The exact source/test plan changes only `agent/gateway_request.rs` and the public
`gateway_request_contract` integration test. On accepted terminal completion,
the turn consumes the exact pending call through `PolicyInput` and the fixed
`DeterministicPolicyEngine`, then returns one retained non-authorizing
`PolicyDecision`. No caller on the bound path can receive a standalone call or
select or omit policy evaluation.

No policy-rule, approval, audit, transport, gateway service, authentication,
credentials, continuation, retries, deadlines, runtime coordinator, dispatch,
execution, Tauri, frontend, SQLite, dependency, capability, entitlement, or
permission path is included. The verified implementation preserves all 56
focused request, protocol, function-validation, policy, tool, public-contract,
policy-input, and approval-binding tests. Clippy, complete `npm run verify`, npm
audit, exact-scope, code, security, documentation, and mandatory gate reviews
pass. D-039 records terminal policy ownership, fixed engine selection,
non-authority, and the public event narrowing. Commit `5e58edb` is pushed on
`codex/phase4-increment-4r`, fast-forward merged into synchronized `main`, and
retains a valid `04r` marker before 4S planning edits.

## Phase 4 Increment 4Q terminally release initial function call - complete

Goal: prevent the verified bound turn from releasing a schema-validated function
call before the normalized response reaches required terminal completion.

The exact source/test plan changes only `agent/gateway_request.rs` and the public
`gateway_request_contract` integration test. The turn retains one private bounded
pending call, returns `None` for the accepted non-terminal function frame,
releases the exact typed call only from terminal response completion, and discards
it on gateway failure or cancellation. Protocol errors remain transactional.

No policy, approval, audit, transport, gateway service, authentication,
credentials, continuation, retries, deadlines, runtime coordinator, dispatch,
execution, Tauri, frontend, SQLite, dependency, capability, entitlement, or
permission path is included. Focused request, protocol, function-validation,
tool, public-contract, and policy baselines pass on clean synchronized `main` at
`8c1a2e0`; the `04p` marker was complete and valid before planning edits.

The verified implementation adds one private pending-call slot and narrows frame
acceptance to optional closed events. Both exact local function calls remain
private while status is `Streaming`, transactional protocol errors retain the
pending call for the correct terminal frame, terminal completion releases it
exactly once, and failure or cancellation discards it. Text completion and local
schema-failure behavior remain unchanged. Six request, 18 protocol, six
function-validation, nine tool, nine public contract, and two policy-binding
tests pass, along with Clippy with warnings denied, complete `npm run verify`, npm
audit, exact-scope, code, security, documentation, and mandatory gate reviews.
D-038 records terminal ownership, discard behavior, and the public optional-event
API narrowing. Commit `8598612` is pushed on `codex/phase4-increment-4q`,
fast-forward merged into synchronized `main`, and retained a valid `04q` marker
before 4R planning edits.

## Phase 4 Increment 4P schema-bound initial gateway events - complete

Goal: make the verified bound initial turn own exact local function-call schema
validation so a future trusted caller cannot receive raw normalized argument JSON
or select a separate registry before policy.

The exact source/test plan changes only `agent/gateway_request.rs` and the public
`gateway_request_contract` integration test. The turn constructs one private
registry from the same fixed two-schema catalog, converts normalized events into
a closed `InitialGatewayEvent`, returns only schema-validated calls, and reports
local schema rejection through a typed content-free error plus terminal failed
wrapper state. Lower-level protocol, registry, validator, policy, approval, and
audit APIs remain unchanged.

No transport, gateway service, authentication, credentials, Keychain, provider
parameters, continuation, retries, deadlines, runtime coordinator, policy,
approval, audit writes or persistence, dispatch, execution, Tauri, frontend,
SQLite, dependency, capability, entitlement, or permission path is included.
Focused request, protocol, function-validation, tool, public-contract, and policy
baselines pass on clean synchronized `main` at `87be00e`; the `04o` marker was
complete and valid before planning edits.

The verified implementation keeps one exact private registry inside the bound
turn, exhaustively converts normalized events, returns only locally
schema-validated function calls, and terminally closes after local schema
rejection. Six request, 18 protocol, six function-validation, nine tool, eight
public contract, and two policy-binding tests pass, along with Clippy with
warnings denied, complete `npm run verify`, npm audit, exact-scope, code,
security, documentation, and mandatory gate reviews. D-037 records schema
ownership, terminal failure, and public event/error narrowing. Commit `8c1a2e0`
is pushed on `codex/phase4-increment-4p`, fast-forward merged into synchronized
`main`, and retained a valid `04p` marker before 4Q planning edits.

## Phase 4 Increment 4O bound initial gateway turn - complete

Goal: bind the verified initial request bytes and response validator into one
non-cloneable, transport-free Rust turn so a future trusted caller cannot
independently choose request/response correlation IDs, allowed function names,
or tool-contract version.

The exact source/test plan changes only `agent/gateway_request.rs` and the public
`gateway_request_contract` integration test. `InitialGatewayTurn` derives the
response validator from the same IDs used for request serialization and from the
exact `ToolSchema` catalog. It exposes only borrowed request bytes, stream status,
frame acceptance, and local cancellation. Raw initial-request construction
becomes private; the lower-level public stream validator remains unchanged.

No transport, gateway service, authentication, credentials, Keychain, provider
parameters, continuation, retries, deadlines, runtime coordinator, policy,
approval, audit persistence, dispatch, execution, Tauri, frontend, SQLite,
dependency, capability, entitlement, or permission path is included. Focused
request, protocol, tool, and public-contract baselines pass on clean synchronized
`main` at `d7c4b69`; the `04n` marker was complete and valid before planning
edits.

The verified implementation keeps request serialization private, derives the
exact two local function names and common version from `ToolSchema`, and delegates
status, normalized frame acceptance, and local terminal cancellation to the owned
validator. Six preserved request tests and six public turn-contract tests pass,
along with 18 protocol tests, nine tool tests, Clippy with warnings denied,
complete `npm run verify`, npm audit, exact-scope, code, security, documentation,
and mandatory gate reviews. D-036 records the bound-turn and public API narrowing
decision. Commit `87be00e` is pushed on `codex/phase4-increment-4o`, fast-forward
merged into synchronized `main`, and retained a valid `04o` marker before 4P
planning edits.

## Phase 4 Increment 4N bounded initial gateway request - complete

Goal: create one non-cloneable, transport-free Rust request value for the first
desktop-to-gateway turn. It validates existing opaque identity rules, preserves
one exact user-selected text value only in serialized bytes, fixes the tool-set
identity and all conservative limits, and enforces the 64 KiB bound after JSON
escaping.

The future source/test scope creates `agent/gateway_request.rs` and one public
integration test, changes `gateway_protocol.rs` only to share opaque-ID
validation with its sibling module, and adds one module export. It adds no HTTP,
gateway service, authentication, credentials, Keychain, provider SDK, model or
provider parameters, continuation, context selector, runtime coordinator, IPC,
UI, persistence, policy, approval, audit, dispatch, executor, dependency,
capability, or permission path.

The implementation passes six focused request tests, 18 preserved gateway
protocol tests, nine tool-catalog tests, one public-boundary integration test,
Clippy with warnings denied, complete `npm run verify`, npm audit, exact-scope,
secret, generated-output, code, security, documentation, and mandatory gate
reviews. D-035 records the request boundary. Commit `d7c4b69` is pushed on
`codex/phase4-increment-4n`, fast-forward merged into synchronized `main`, and
retained a valid `04n` marker before 4O planning edits.

## Phase 4 Increment 4M remove legacy platform scaffold - complete

Goal: delete the unused public generic `PlatformAdapter`, arbitrary-string
metadata, caller-authored capability status map, and broad capability/report types
before future permissions or native integration work can mistake them for
authoritative operating-system evidence.

Repository search finds no caller outside the three platform files and their
three embedded tests. The exact source plan deletes the complete
`src-tauri/src/platform/` module and removes only `pub mod platform;` from
`src-tauri/src/lib.rs`. The independent app-info IPC and fixed frontend Permission
Center remain unchanged.

The verified implementation adds no replacement adapter, native framework,
permission query/request, Keychain, LocalAuthentication, frontend state, IPC,
dependency, Tauri capability, entitlement, or operating-system permission. The
app-info unit test, public metadata smoke test, focused Permission Center test,
Clippy, complete `npm run verify`, npm audit, stale-symbol, exact-scope, security,
code-health, documentation, and mandatory gate reviews pass. D-034 preserves the
future capability-specific adapter and authoritative permission-evidence
requirements. Commit `1f03d1e` is pushed on
`codex/phase4-increment-4m`, fast-forward merged into synchronized `main`, and
retains a valid 04m marker.

## Phase 4 Increment 4L remove legacy memory scaffold - complete

Goal: delete the unused public `MemoryStore`, arbitrary-content input/update/record
types, unbounded in-memory map, and six-marker secret-like check before future
memory or persistence work can mistake them for the approved trusted boundary.

Repository search finds no caller outside the three memory files and their three
embedded tests. The exact source plan deletes the complete
`src-tauri/src/memory/` module and removes only `pub mod memory;` from
`src-tauri/src/lib.rs`. The 13-test typed SQLite bootstrap storage boundary
remains unchanged.

The verified implementation adds no replacement memory, repository, migration,
encryption, Keychain, context collection, persistence, IPC, UI, dependency,
capability, or permission. Thirteen storage unit tests, both public storage smoke
tests, Clippy, complete `npm run verify`, npm audit, stale-symbol, exact-scope,
security, code-health, documentation, and mandatory gate reviews pass. D-033
preserves the future bounded, opt-in, provenance-aware, encrypted memory
requirement. Commit `ecd49be` is published and merged into synchronized `main`.

## Phase 4 Increment 4K remove legacy provider scaffold - complete

Goal: delete the unused synchronous `AgentProvider`, arbitrary-string request,
assistant-response, and mock-error scaffold before a future gateway transport can
mistake it for the approved production provider boundary.

Repository search finds no caller outside the two legacy files and their three
embedded tests. The exact source plan deletes `src-tauri/src/agent/provider.rs`
and `src-tauri/src/agent/types.rs` and removes only their exports from
`src-tauri/src/agent/mod.rs`. The 18-test normalized gateway protocol and exact
function-call validator remain unchanged.

The published implementation adds no replacement provider, gateway request contract, HTTPS
transport, credentials, Keychain, runtime coordinator, dispatch, executor,
persistence, IPC, UI, dependency, capability, or permission. Eighteen gateway,
six function-validation, and two public gateway-to-policy tests pass. Clippy,
complete `npm run verify`, npm audit, exact-scope, secret, generated-output,
architecture, code-health, security, documentation, and mandatory gate reviews
pass. D-032 records the future closed provider-transport boundary.

Commit `5415444` is pushed on `codex/phase4-increment-4k`, fast-forward merged
into synchronized `main`, and retains a valid `04k` marker after the tracked
deletions were committed.

## Phase 4 Increment 4I remove generic audit scaffold - complete

Goal: delete the unused public caller-authored `AuditEventInput`, `AuditLogger`, in-memory/no-op logger, arbitrary summary/details records, and token-pattern redactor before a future coordinator can mistake them for the trusted local audit boundary.

The reconstructed source change deletes only `src-tauri/src/audit/logger.rs` and `src-tauri/src/audit/types.rs` and removes their two exports from `src-tauri/src/audit/mod.rs`. The verified typed `audit::approval` module remains unchanged. Reconstructed commit `99f9279` is pushed and fast-forward merged into synchronized `main`; the corrected marker remains valid for that committed content. The original `cf9d701` commit is preserved locally on `codex/phase4-increment-4i-pre-fingerprint-fix` and no remote ref contains it.

The increment adds no replacement audit abstraction, durable repository, storage migration, coordinator, dispatch, executor, provider continuation, IPC, UI, networking, credential, dependency, capability, or permission. Six typed-adapter tests, eleven native-source tests, one public approval-audit integration test, Clippy, `npm run verify`, npm audit, stale-symbol, scope, diff, code, security, documentation, and corrected post-increment reviews pass.

## Repository Workflow Increment 4J post-increment deletion fingerprint - complete

Goal: make one valid post-increment completion marker survive committing reviewed tracked-file deletions while preserving invalidation for files deleted after finalization.

The exact two-file implementation moves path hashing after successful metadata lookup and adds positive and negative deletion regressions. Exact changed-file report inventory, path safety, content and metadata hashing, report hashes, suspicious-path checks, Stop behavior, and state schema remain unchanged. Thirteen declared documentation files record the boundary and evidence. No application source, dependency, hook configuration, skill, Tauri, IPC, storage, provider, gateway, approval, audit, dispatch, executor, capability, or permission changes.

The original 4I commit remains preserved at `cf9d701` under `codex/phase4-increment-4i-pre-fingerprint-fix`. 4J is published and merged, so 4I reconstruction proceeds on the corrected baseline without a legacy-marker migration or compatibility fallback.

Seventeen focused hook tests, complete `npm run verify`, npm audit, exact scope, secret, generated-output, code, security, and complete-diff reviews pass. The consolidated result is `PASS WITH ADVISORIES`; the advisory is the required publication ordering before 4I reconstruction.

## Phase 4 Increment 4H typed approval-audit adapter - complete

Goal: derive one closed, content-free approval-audit record from an exact terminal `ApprovalResolution` and retain it in a bounded deterministic in-memory adapter without creating persistence, orchestration, dispatch, or execution authority.

The exact four-file runtime/test scope creates the dedicated adapter and public-boundary integration test, exports the module, and adds test-only assertions to the existing sealed native-source path. It revalidates exact current tool/policy facts and every terminal disposition/evidence combination, stores no task title or arbitrary string details, rejects invalid evidence, duplicates, capacity overflow, and sequence overflow before mutation, and returns only a non-authorizing sequence receipt.

Six adapter tests, eleven native-source tests, one public-boundary integration test, Clippy, `npm run verify`, npm audit, complete diff review, architecture review, code-health review, security review, documentation synchronization, and the mandatory post-increment gate pass. The generic audit scaffold remains unchanged and disconnected. No dependency, lockfile, Tauri, frontend, SQLite, gateway, provider, approval-manager behavior, native-dialog behavior, executor, IPC, capability, CSP, packaging, or permission changed. D-029 records the non-durable and non-authorizing boundary.

## Repository Workflow Increment 4G post-increment gate - complete

Goal: add a trusted repository-local Stop hook, deterministic Python validator, consolidated review skill/report, and documentation synchronization gate without changing application behavior.

The approved 24-file tracked scope adds no external dependency, network access, transcript parsing, product source, Tauri, Rust, IPC, persistence, capability, permission, approval, audit, or execution path. Fifteen focused hook tests and complete `npm run verify` pass. Exact scope, secret, generated-output, code, and security reviews pass after resolving parent-symlink escapes. The project owner passed normal hook trust and live Stop confirmation; the consolidated result is `PASS WITH ADVISORIES` and the marker is valid.

## Phase 4 Increment 4F product display rename - complete

Goal: rename only the human-facing product name to `Cortexa` while preserving repository, package, crate, executable, bundle-ID, database, storage, event, command, and other compatibility identifiers.

The reviewed implementation updates the Tauri/window/menu/native-dialog metadata, typed Settings app name, sidebar brand, fixed diagnostics, focused tests, repository skills and prompts, and all tracked exact former-name documentation. It adds no dependency, behavior, trust-boundary, persistence, capability, permission, network, credential, or execution change. The full requested automated gate, project-owner target-Mac confirmation, complete diff/code/security review, and synchronized project memory pass. The absent post-increment skill did not run and has no claimed result; D-027 records the project owner's one-time completion exception and defers skill creation to the next clean branch.

## Phase 2 Increment 2E — complete

Verified on 2026-07-13. The React application shell, closed menu-route handling, Settings diagnostics, and Permission Center placeholders passed all required automated and manual checks.

## Phase 2 Increment 2F — complete

Goal: add a deterministic, mocked assistant interaction flow to the verified application shell.

Planned boundaries:

- In-memory conversation messages only.
- Deterministic mock streaming and stop behavior.
- Tool activity card presentation.
- Trusted mock approval dialog.
- No network, API key, real tool execution, new Tauri command, OS permission, or persistence expansion.

Implementation and `npm run verify` pass on the target Mac. Native Tauri launch passes with idempotent storage startup. The project owner confirmed streaming, Stop, approve/reject/edit, small-window, lifecycle, and no-permission-prompt checks passed.

## Phase 2 Increment 2G — complete

Goal: complete bounded cancellation, error-state, audit-view, and release-verification hardening without production model access or privileged automation.

The typed driver, bounded failure and Retry, redacted in-memory Activity feed, and focused tests are implemented. `npm run verify`, `npm audit --audit-level=low`, native launch, and project-owner manual acceptance all pass.

## Phase 3 planning — complete

The product brief and architecture baseline were reconciled with the completed Phase 2 mock loop. The smallest missing capability was volatile conversation identity and history. Increment 3A was approved, implemented, and passed automated verification, native launch, and project-owner manual acceptance.

## Phase 3B planning — complete

The remaining product requirements were reconciled with the verified implementation through Increment 3A. Mock context provenance is the smallest next capability because the product requires visible information-use disclosure and conversation identity now provides the required ownership boundary.

The approved implementation limits the increment to fixed-copy, volatile WebView presentation tied to exact run and conversation IDs. It excludes real context collection, context controls, trusted provenance, persistence, tool results, networking, dependencies, native capability changes, and permissions. Automated verification, native launch, and project-owner native interaction and layout checks pass.

## Phase 3C planning — complete

The remaining tool-result and mocked-loop requirements were reconciled with the verified implementation through Increment 3B. Approve-only simulated tool-result presentation is the smallest next capability because the product requires a distinct result view and the current loop already has exact run, conversation, proposal, and decision boundaries.

The approved implementation limits the increment to fixed-copy volatile WebView presentation with `executed: false`. It excludes real execution, provider continuation, arbitrary payloads, trusted executor or audit claims, persistence, networking, dependencies, native capability changes, and permissions. Automated verification, native launch, and project-owner native interaction and layout checks pass.

## Phase 3 completion planning — complete

The verified implementation through Increment 3C ends at a simulated result, renders no distinct final answer after that result, and does not expose one closed conservative limit contract. Phase 3 therefore needs one final bounded Increment 3D.

The approved plan adds a synchronous fixed frontend mock continuation, exact adjacent result/final pairing, and explicit limits of two model turns, one tool call, one retry, zero network/tool timeout/file/search capacity, and 512 output characters per turn. Production provider continuation, real tools, arbitrary payloads, generic timeline work, persistence, native changes, and permissions remain excluded.

Implementation, automated verification, native development launch, and project-owner manual acceptance pass with 124 frontend tests, 50 Rust library tests, 6 Rust integration tests, production frontend and Tauri builds, zero dependency vulnerabilities, and idempotent startup with two migrations already applied.

## Phase 3D bounded mock-loop completion — complete

The project owner confirmed exact result/final ordering and run identity, request-content exclusion, no final answer after Reject/Edit/Stop, per-conversation restoration, supported layouts, existing context and Activity behavior, native routing, Settings diagnostics, lifecycle behavior, storage startup, and absence of permission prompts all pass. Increment 3D and Phase 3 are verified complete.

The next Ready task is documentation-only Phase 4 gateway and Responses security-boundary planning. No provider or runtime implementation may begin before the exact plan is approved.

## Phase 4 gateway and Responses planning - complete

The product, architecture, security policy, accepted decisions, official OpenAI documentation, and actual Rust/provider boundaries were reconciled. Decision D-021 assigns production OpenAI credentials to server-side gateway secret storage, keeps future gateway tokens in trusted Rust and platform secret storage, and requires a versioned normalized gateway protocol with dual validation, foreground `store: false` streaming, transport-abort cancellation, conservative limits, closed redacted errors, and separate gateway operational and local trusted audit records.

Increment 4A was approved and implemented as the smallest Phase 4 increment: one transport-free portable Rust gateway-protocol module, one exact already-locked parsing dependency, and deterministic inline tests. Focused checks, `npm run verify`, dependency audit, diff checks, code review, and security review pass. It adds no network client, gateway server, credential, IPC, tool execution, persistence, capability, CSP, packaging, or permission path.

## Phase 4 Increment 4A gateway protocol contract - complete

The versioned normalized event contract, transactional stream validator, conservative limits, local cancellation, closed redacted failures, and explicitly non-actionable function-call values are implemented. Seventeen focused tests cover accepted text/function streams and malformed, oversized, mismatched, out-of-order, duplicate, late, mixed, over-limit, duplicate-key, unknown-tool/contract, and error-redaction cases. The full gate passes with 124 frontend tests, 67 Rust library tests, six Rust integration tests, and production frontend/Tauri builds.

The next task was documentation-only Increment 4B planning for exact local per-tool schema validation. That plan was approved and implemented as recorded below.

## Phase 4 Increment 4B local tool-schema validation - complete

The placeholder schema was replaced with exact `get_current_datetime@1` and `create_local_task@1` contracts. Tool definitions now derive identity, version, risk, permission, description, and schema from the closed local catalog. An ownership-consuming validator independently checks normalized gateway calls and returns private typed, redacted, non-authorizing data with no raw JSON.

Focused tests, the full repository gate, dependency audit, diff checks, code review, and security review pass. No dependency, proposal, policy, approval, audit, executor, transport, IPC, persistence, permission, or user-visible path was added. The next task was documentation-only Increment 4C planning for trusted proposal and policy-input binding.

## Phase 4 Increment 4C trusted policy-input binding - complete

The verified 4A/4B boundaries were reconciled with the unused raw proposal/provider-response path, independently constructed policy actions, caller-supplied policy context, generic approval and audit scaffolds, accepted security rules, and actual repository callers. The smallest coherent increment removes the bypasses and lets policy consume one exact locally schema-validated call without caller-supplied state.

The approved plan defined canonical input as the ownership-bound typed `SchemaValidatedFunctionCall`, not serialized bytes or a digest. Because no current type binds intent, permission, scope, and freshness to the exact call, permission-bearing and read-only actions deny and reversible actions require approval. It defined closed policy reasons with derived outcomes and a decision that retains the exact evaluated input while granting no approval or execution authority. Trusted evidence, approval binding, hashes, previews, expiry, one-time consumption, audit, dispatch, provider continuation, networking, credentials, IPC, persistence, and UI remain later work.

The project owner approved the exact plan. Implementation removed the raw proposal/provider-response bypass and caller-supplied policy context, introduced ownership-bound policy input and input-retaining closed decisions, and made unsupported evidence paths fail closed. Four policy unit tests and two public gateway-to-policy integration tests prove the rule table, exact retained metadata and arguments, and debug redaction.

Focused checks, `npm run verify`, dependency audit, diff checks, code review, and security review pass. No dependency, approval, audit, executor, transport, IPC, persistence, permission, or user-visible path was added. Documentation-only Increment 4D planning is now complete as recorded below.

## Phase 4 Increment 4D exact approval binding - complete

The verified policy decision was reconciled with the detached approval and audit scaffolds. The smallest coherent increment first retains validator-owned run and gateway-request IDs through local schema validation and policy, then replaces arbitrary approval strings with an ownership-consuming transport-free manager.

The proposed manager accepts only one exact `RequireApproval` decision, derives a borrowed closed `create_local_task@1` preview from the retained typed arguments, permits one pending request and 1,024 subjects per manager lifetime, uses a relative manager-owned 120-second monotonic deadline, and consumes approve, reject, cancel, or expiry once. Duplicate run/request/call subjects fail closed without tombstone eviction, Edit requires a fresh validated call, and future orchestration must cancel approval when its run terminates.

The plan removes caller-supplied `action_hash` and adds no digest or dependency because direct in-process ownership is the stronger binding. Request, view, and resolution values remain non-cloneable, non-serializable, debug-redacted, and disconnected from audit, dispatch, execution, IPC, persistence, UI, and provider continuation.

Planning baseline checks passed on clean merged `main` at `55626b6`. The project owner approved the exact plan and five-file runtime/test list before implementation.

The implementation retains validator-owned run/request/call identity, removes raw content cloning and debug output, and replaces the detached approval scaffold with one ownership-consuming manager. The manager derives the exact borrowed `create_local_task@1` preview, enforces one pending request, a 1,024-subject lifetime cap, relative 120-second monotonic expiry, explicit cancellation, and non-evicting terminal replay prevention. It adds no digest, dependency, serialization, audit, dispatch, executor, IPC, persistence, network, credential, capability, or permission path.

Focused Rust checks, rustfmt, Clippy with warnings denied, `npm run verify`, dependency audit, diff checks, code review, and security review pass. The full gate contains 124 frontend tests, 82 Rust library tests, and ten Rust integration tests plus production frontend and Tauri no-bundle builds. No native interaction gate applies because the modules remain transport-free and unreferenced by Tauri. D-024 records the durable approval boundary. Documentation-only Increment 4E planning followed and is recorded below.

## Phase 4 Increment 4E trusted approval-decision source - complete

The verified manager, product preview requirements, native/WebView trust boundary, security policy, optional LocalAuthentication requirement, and actual Tauri capabilities were reconciled. The untrusted WebView mock cannot become production approval authority, and LocalAuthentication could authenticate a device owner but would not bind or display the exact action preview. The project owner approved the exact plan and eight-file runtime/test scope.

Implementation now issues one owned presentation from the authoritative manager and lets only a Rust-owned macOS native message-dialog source privately construct a sealed exact-subject outcome. A private `Arc` manager-instance marker moves through that handoff and is pointer-checked before approval/run/request/call identity, preventing cross-manager substitution without a digest or content retention. The manager rechecks one-shot issuance, cancellation, and monotonic expiry before terminal resolution. Edit, native no-decision, source failure, run cancellation, expiry, and replay fail closed. Only recognized Approve/Reject/Edit buttons carry native-button evidence; every source outcome records `NotEvaluated` authentication and grants no execution authority.

The implementation adds exact macOS-target `rfd = "=0.17.2"` with default features disabled. Direct use registers no Tauri dialog plugin, invoke command, JavaScript API, capability, or application `unsafe`; the source remains disconnected from the shipping app, WebView, LocalAuthentication, audit, persistence, dispatch, execution, provider continuation, gateway networking, and credentials. Sixteen approval unit tests, two approval-binding integration tests, rustfmt, Clippy, `npm run verify`, `npm audit --audit-level=low`, dependency review, code review, and security review pass.

The target-Mac owner interaction matrix passes: Approve, Reject, Edit, Return/default, fixed title/content order, terminal redaction, no action or persistence, and no permission prompt were confirmed; Escape had no effect and no window-close control was available. Exact `cargo-audit 0.22.2` exits nonzero on RUSTSEC-2026-0194 and RUSTSEC-2026-0195 in pre-existing `quick-xml 0.39.4`; 4E adds only `rfd`, and source review found the cited vulnerable APIs unused on the existing `plist -> Tauri` path. D-025 records the project owner's scoped reviewed baseline exception without an advisory ignore or dependency change. Increment 4E is verified complete, and no later increment is Ready.
