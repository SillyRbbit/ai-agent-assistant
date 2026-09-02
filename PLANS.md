# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Current plan state

The most recently completed owner-approved documentation-only plan is
[`d107-exact-signer-binding-contract-decision`](docs/plans/2026-09-02-d107-exact-signer-binding-contract-decision.md).
D-110 selects `signer_binding_not_accepted`: current repository source has no
immutable expected Developer ID Application signer/certificate/public-key
binding. Correspondence, labels, fingerprints, filters, and ambient/default
authority are not substitutes. No source or runtime boundary is added; all ten
blockers remain unproved and no successor is Ready.

The most recently completed owner-approved documentation-only plan is
[`d107-opaque-prebound-identity-contract-decision`](docs/plans/2026-09-02-d107-opaque-prebound-identity-contract-decision.md).
D-109 selects `reference_issuance_not_accepted` because current repository
source contains no application-owned issuer for a no-input, attempt-bound opaque
signing-identity reference without lookup, enumeration, selection, fallback,
or ambient/default authority. This adds no source or runtime boundary. D-107
remains 8/11, D-108 remains 9/10 prospectively, all ten blockers remain
unproved, and no successor is Ready.

The most recently completed owner-approved documentation-only plan is
[`d102-non-build-proof-applicability-decision`](docs/plans/2026-09-02-d102-non-build-proof-applicability-decision.md).
It reconsiders exactly `d102_applicability_split_contract` for the frozen D-107
concept. D-108 accepts a split only for D-102's build-child subject while the
candidate remains exactly no-build, no-candidate-launched-helper/child/
subprocess/external-executable, no-artifact, no-application-or-Rust-dependency-
authored filesystem/network/socket/IPC API, no-application-or-Rust-dependency-
selected dynamic/JIT/plugin/external-code load, and no-`codesign`. A definitive
trigger makes D-102 fully mandatory; ambiguity or drift records
`boundary_failed` and also makes D-102 mandatory. Ten contracts, including
`platform_effect_contract`, remain unproved; historical D-107 remains 8/11 and
the additive current interpretation is 9/10. The candidate and every successor
remain Blocked. The exact documentation checks and completion gate passed with
`PASS WITH ADVISORIES`.

The preceding completed owner-approved documentation-only plan is
[`d107-operational-scope-wording-reconciliation`](docs/plans/2026-09-02-d107-operational-scope-wording-reconciliation.md).
It preserves all published D-107 evidence while clarifying that documentation
files were written and local validation/gate processes ran. No product/test
source, dependency, configuration, capability, entitlement, IPC, workflow,
hook, or script path changed, and no product-build, signing, Keychain/private-
key, target-Mac, or state-changing external operation ran. D-107 and Blocked
successor readiness remain unchanged. The exact documentation checks and gate
passed with `PASS WITH ADVISORIES`.

The preceding completed owner-approved documentation-only plan is
[`personal-assistant-v0-key-use-containment-classification`](docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md).
It freezes exactly `in_process_security_framework_ephemeral_challenge_proof_v1`
and records D-107's bounded source result `not_eligible_or_unproven`: eight rows
are `documented` and eleven are `contract_unproven`. No identity lookup,
private-key operation, product build or signing, target-Mac operational check,
or state-changing external action ran. Documentation files were written and
local validation/gate processes ran as recorded. The candidate is not admitted,
does not waive D-101 or D-102, and makes no successor Ready. Documentation
validation and the completion workflow passed with `PASS WITH ADVISORIES`;
next-increment readiness is `Blocked`.

The preceding completed owner-approved plan is
[`personal-assistant-v0-codeless-signing-fixture-classification`](docs/plans/2026-09-02-personal-assistant-v0-codeless-signing-fixture-classification.md).
It freezes exactly `repository_owned_codeless_bundle_signing_fixture_v1` for a
documentation-only current-Apple-contract classification. D-106 records
`not_eligible_or_unproven`: five rows are `documented` and eight are
`contract_unproven`. The source record does not admit the candidate or a
successor. It creates no fixture, changes no source, runs no product/build/
signing process, uses no Keychain or private key, performs no signing, and does
not make P3-3, P4, V0-3, or any operational successor Ready.

It completed with `PASS WITH ADVISORIES`; the exact documentation checks and
completion gate passed, and next-increment readiness is `Blocked`.

The preceding completed owner-approved plan is
[`personal-assistant-v0-app-sandbox-containment-rereview`](docs/plans/2026-09-02-personal-assistant-v0-app-sandbox-containment-rereview.md).
It records D-105's bounded documentation result,
`no_eligible_candidate_after_d104_rereview`. D-104 removes only the Developer
ID circularity classification; all 22 D-102 contracts remain
`contract_unproven`, all ten source checks remain `not_run`, and no product or
operational action is authorized. No successor is Ready; P3-3 and every
operational successor remain Blocked.

The preceding completed owner-approved plan is
[`personal-assistant-v0-containment-bootstrap-trust-decision`](docs/plans/2026-09-02-personal-assistant-v0-containment-bootstrap-trust-decision.md).
It completed with `PASS WITH ADVISORIES` as documentation-only evidence. Its
narrow D-104 question is whether a future
identity-free ad-hoc sandbox-activation seal may be reviewed separately from
P4's later Developer ID signer binding. It selects no primitive, changes no
containment requirement, and authorizes no signing, entitlement, build, or
external action. P3-3 and all operational successors remain Blocked.

An earlier completed owner-approved plan is
[`personal-assistant-v0-containment-primitive-selection`](docs/plans/2026-09-02-personal-assistant-v0-containment-primitive-selection.md).
It completed with `PASS WITH ADVISORIES` as a documentation-only authoritative
source review. D-103 selects no eligible candidate in the frozen reviewed set;
it does not claim universal impossibility. The App Sandbox helper candidate is
entitlement/signing-circular and lacks complete public detached-descendant
lifecycle contracts. All other frozen candidates fail D-102 or the approved
scope. P3-3 and every operational successor remain Blocked, and no build,
probe, target-Mac, signing, source, dependency, configuration, or external
action is authorized.

The preceding completed owner-approved plan is
`personal-assistant-v0-build-child-containment-planning`. It is
documentation-only and completed with `PASS WITH ADVISORIES`. It defines
D-102's future fail-closed build-child containment policy without selecting or
implementing a primitive. It preserves
the operational blocker: no supported target-Mac no-new-dependency mechanism
currently proves pre-effect outside-root-write/network denial and full
detached-descendant containment. No build, probe, signing, or external action
is authorized.

An earlier completed owner-approved plan is
[`personal-assistant-v0-account-directory-boundary-planning`](docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md).
It completed with `PASS WITH ADVISORIES` and is documentation-only. Accepted
D-101 defines `ExplicitAccountResolutionPolicyV1::Prohibited`, requires
independent input, exact scope-provenance, and one-predicate effect gates, and
permits only a future application-owned opaque no-input wrapper over an adapter-
private platform reference. It runs no operation, establishes no operational
containment, provides no acceptance fallback, and leaves historical evidence
plus P3/P4/operational work unchanged.

The previously completed owner-approved plan is
[`personal-assistant-v0-evidence-privacy-protocol-planning`](docs/plans/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning.md).
It completed with `PASS WITH ADVISORIES`. It is documentation-only and defines
`evidence_privacy_v1`: a fixed version, one plan-owned check ID, and one closed
outcome, with exact bounds, non-authorizing semantics, private future attempt
binding, source minimization, and stop-without-retry behavior. It collected no
evidence, changed no product or external system, and leaves D-097/D-098/D-099,
P2–P4, and all operational work unchanged.

The predecessor completed owner-approved plan is
[`personal-assistant-v0-signing-security-prerequisite-planning`](docs/plans/2026-08-29-personal-assistant-v0-signing-security-prerequisite-planning.md).
It is documentation-only, completed with `PASS WITH ADVISORIES`, carries valid D-098 schema-v3 lineage, and changed
only the exact fifteen recorded documentation paths. Its closed outcome maps
future evidence privacy, account-directory, build-child-containment, and
immutable signer-contract prerequisites. It cannot alter the D-097 failure,
authorize operational signing, or create a product/external-system edge.

The predecessor exceptional recovery plan is
[`v0-terminal-failed-successor-disposition-recovery`](docs/plans/2026-08-29-v0-terminal-failed-successor-disposition-recovery.md)
from clean synchronized baseline
`a417e5f1c1c602b917ca27c65af71480e3db6a45`. D-098 permits only one bounded,
argument-free `record-failed-disposition` operation that may add schema-v3
lineage from the published D-097 failure to the exact later documentation target
`personal-assistant-v0-signing-security-prerequisite-planning`. The original
status, quality, and readiness remain `failed` / `FAIL` / `Blocked`; no
completion marker may appear. The screenshot/privacy failure and Pending Open
Directory boundary remain carried evidence for that exact target, while
build-script containment, operational signing, V0-3, and all external/product
work remain Blocked. The tracked report freezes only pre-transition evidence;
it necessarily records the disposition command as Not run. Its post-freeze
outcome is established only by the ignored schema-v3 state and redacted
`status`, not by a later tracked edit. No commit, publication, or successor
start is part of this recovery.

The separately owner-approved execution of the
[Xcode Developer ID recovery plan](docs/plans/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md)
is Blocked from baseline `0931df6`. Xcode created one Developer ID Application
certificate record. Sanitized CLI checks found no usable code-signing identity,
while the owner later confirmed that Keychain Access shows the certificate with
a private key beneath it. Local pairing is observed. A later separately
approved exact `keychain_identity_v1` run returned
`passed_one_label_matched_valid_codesigning_identity`, establishing current
scoped identity visibility. Non-exported owner control, signing, and owner-only
prompt/state observations were then assessed: the owner reported no prompt or
visible state change, while the historical non-export criterion cannot Pass as
written and signing remains Not run. The post-increment decision remains
`FAIL`, and no completion marker is valid. The bounded
follow-up selected by the owner was the documentation-only
[sanitized Keychain-scoped identity discrepancy plan](docs/plans/2026-08-29-v0-keychain-scoped-identity-discrepancy.md).
Its exact draft passed documentation-tier and embedded-Python syntax validation,
then passed static architecture/security re-review after disclosure corrections.
The owner subsequently accepted the exact residual boundaries, confirmed target-
Mac control, and authorized the one passing run. That approval is consumed and
authorizes no retry, prompt interaction, Keychain/Apple/Xcode mutation, or
signing. The recovery gate is terminally `failed` / `FAIL` / `Blocked` without
a completion marker, and V0-3 remains Blocked.

The owner next authorized documentation drafting for
[Developer ID present-use and local signing proof](docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md).
The draft records that current Keychain/signing evidence cannot prove historical
absence of export or exclusive custody and leaves the unqueried current
extractability attribute `not_proven`. The owner has now approved that bounded
documentation plan, and D-096 accepts its prospective evidence standard without
rewriting historical results. Its future no-argument sanitizer and one local
timestamp-free signing run remain Proposed/Blocked until every recorded blocker
is resolved, the exact sanitizer passes static review, and the owner separately
accepts the operational residual boundaries and authorizes one attempt. This
documentation decision starts no new gate and grants no Keychain, Apple,
private-key, build, signing, cleanup, or V0-3 authority.
Operational review also requires a no-caller-input expected-team source, exact
fingerprint/leaf binding, a disposable clone for every writing command,
closed host Git-config/filter-isolated preflight, explicit per-surface package-
network/cache policy, implicit-install-audit suppression, exact zero-count
audit parsing, process-group termination, bounded quiescence, and parent/name/
root descriptor-bound cleanup. Those designs are not yet executable.

Post-gate review added three explicit blockers without making operational
execution Ready:
the consumed `getpwuid` lookup's previously undisclosed `opendirectoryd`
account-record/directory-service/cache/socket/log boundary remains Pending; the
then-current hook could not encode a terminal `FAIL` despite the immutable privacy
failure; and configured build/cache roots do not confine executable npm/Cargo/
Tauri child writes, connections, or process escape. These findings authorize no
rerun, build, or signing operation. D-097 later authorized only the exact
same-active-gate terminal-failure recovery. Evidence-standard documentation
reconciliation is now accepted under D-096; every successor and operational
action remains Blocked.

Final gate documentation sync mapped three unsupported report finding
categories to the existing closed allowlist, restored the template's scope
section, and marked the parent plan's old operation boundary historical. The
hook's own read-only validator reached only the expected blocking-evidence
result. D-097 and the later same-active-gate implementation now provide only a
valid terminal failed state; they provide no completion or publication
authority, and the stored readiness remains Blocked.
The future signing plan also remains operationally Blocked until account-
directory resolution is contained or separately disclosed and accepted.

The owner-approved
[`V0-2 — volatile Personal Assistant Rust session host`](docs/plans/2026-08-28-personal-assistant-v0-session-host.md)
is locally verified complete with `PASS WITH ADVISORIES` from synchronized
baseline `8e382e8`.
It was published through [PR #81](https://github.com/SillyRbbit/ai-agent-assistant/pull/81):
reviewed head `7fecf03` squash-merged to `main` at `1513bd8` with all six
required checks passing.
It adds no production transport, provider-frame ingress, Tauri/WebView,
credential, persistence, dependency, or device authority. No successor is
Ready: V0-3 remains Blocked by D-076/TS-017, and every later plan retains its
recorded dependencies and separate approval requirement.

The historical owner-approved documentation-only
[Xcode Developer ID recovery plan](docs/plans/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md)
is verified complete as planning evidence from baseline 9a48b25. It defined a
later owner-operated Xcode-managed Developer ID Application path for V0-3's
private, fake-only proof while explicitly excluding App Store publication,
Apple access, certificate creation, Keychain changes, signing, product source,
configuration, dependencies, and traffic at planning closeout. D-076 and
TS-017 remain unresolved after the blocked execution recorded above.

## Historical V0-1 plan state

The owner-approved
[`Personal Assistant v0 Linux Clippy portability correction`](docs/plans/2026-08-28-personal-assistant-v0-linux-clippy-portability.md)
is locally verified complete with advisories and is the current bounded
publication fix for PR #79. It changes only two Rust
test-module import blocks so macOS-only imports share their consumers' existing
target guards. It adds no lint allowance, test skip, behavior, dependency,
workflow, or capability. The corrected PR head must pass all required checks
before squash merge; no successor implementation is Ready.

The owner-approved
[`V0-1 — sealed empty-tool turn and minimal host`](docs/plans/2026-08-28-personal-assistant-v0-empty-tool-turn.md)
is locally verified complete with advisories from synchronized `main` baseline
`0b22ee7`. It implements only the transport-free Rust contract and no-input
volatile host. It adds no user-text or response ingress, Tauri/WebView path,
provider/network/credential state, persistence, tool, approval, audit,
filesystem, background work, or device effect. Its historical local closeout
preceded commit `a346946` and PR #79.

The dependency-ordered
[`Personal Assistant v0 capability program`](docs/plans/2026-08-28-personal-assistant-v0-program.md)
continues to separate the live synthetic OpenAI-through-Cloudflare proof,
private real-prompt activation, and action-taking/production product.
[`V0-2`](docs/plans/2026-08-28-personal-assistant-v0-session-host.md)
remains Blocked until V0-1 is published or otherwise accepted as the exact
source baseline; V0-3 through V0-14 retain their exact later blockers. No
local-model alternative is authorized and no successor implementation is
Ready.

The owner-approved documentation-only
[`CI-classification engineering-guide reconciliation`](docs/plans/2026-08-28-ci-classification-engineering-guide-reconciliation.md)
increment is verified complete with advisories from synchronized `main`
`7390ea6`. It corrects only the
stale high-level statement that grouped native examples with isolated Rust
tests, and reconciles current-state publication evidence for the completed
risk-based classifier increment. No classifier, workflow, runner, product,
dependency, capability, CSP, permission, credential, IPC, provider,
persistence, filesystem, or tool behavior changes. Documentation-only
validation and the completion marker pass; no next source increment is Ready.

The owner-selected
[`risk-based CI trust-boundary classification`](docs/plans/2026-08-28-risk-based-ci-trust-boundary-classification.md)
increment is verified complete with advisories from synchronized `main`
`3fc14e4`. It closes only the
classifier gap that routes credential, document, memory, menu-bar, sealed-demo
boundary, and native trust-boundary example changes to Rust alone. The plan
makes production Rust fail closed to frontend, Rust, and audit jobs, retains a
narrow exact-path exception mechanism with no current production exception,
and preserves existing narrow path/event classes. It changes no product source,
workflow YAML, runner, dependency, capability, CSP, permission, credential,
persistence, IPC implementation, provider, tool, or external state. It was
published through PR #76 at `main` `7390ea6`; all PR and merged-main CI and
Documentation checks passed. The stale high-level `ENGINEERING_GUIDE.md`
sentence is the active documentation-only reconciliation above; no next source
increment is owner-selected or Ready.

The owner-approved documentation-only
[`native-multi-agent-final-review-planning`](docs/increments/native-multi-agent-final-review-planning.md)
and the separately approved
[`native-multi-agent-final-review`](docs/increments/native-multi-agent-final-review.md)
are verified complete with advisories. The final ExecPlan's source-current
review at `181f851` found no Critical, High, Medium, or Low defect. It confirms
the completed bounded remediation of F-01/F-02, F-07, F-08, F-12, and F-15;
neither the plan nor its review grants remediation or source authority.
The review documentation is published through
[PR #74](https://github.com/SillyRbbit/ai-agent-assistant/pull/74) at `main`
squash commit `d3edc7a`; its PR and merged-main Documentation runs passed.

The owner-approved source
[`2026-08-28-research-knowledge-demo-connected-presentation.md`](docs/plans/2026-08-28-research-knowledge-demo-connected-presentation.md)
is verified complete with advisories and published through
[PR #71](https://github.com/SillyRbbit/ai-agent-assistant/pull/71) at `main`
squash commit `d9c7c13`. It connects only a separately labelled prop-free
lifecycle panel in the selected scenario to the existing four no-input commands. A private
application-owned completed-run schedule alternates deterministic success and
synthetic failure without a caller selector; cancellation consumes no outcome.
Command responses alone commit presentation, while newer valid notifications
only require recovery. F-12 pins the sole consumer and exact reviewed source.
PR-head and merged-main Documentation and CI workflows pass. The current
read-only review plan does not authorize a next source plan.

The earlier owner-approved documentation-only
[`2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter.md`](docs/plans/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter.md)
is **verified complete with advisories** after the owner approved
the narrowly scoped private approval-clock Send prerequisite. One mutex-owned
volatile host now backs four no-caller-input commands and one bounded
notification event; an unconnected client validates and freezes the exact DTO,
rejects concurrent/stale/gapped data, and requires explicit snapshot recovery.
F-12 guards the exact boundary. Command Center connection and capability, CSP,
dependency, provider, persistence, background, and device-effect changes remain
out of scope. Complete automated and target-Mac startup verification pass;
direct observation of the deliberately unconnected lifecycle event is `Not
run`. At that checkpoint no connected presentation plan was Ready; the current
completed source plan above supersedes only that historical readiness claim.

The earlier owner-approved
[`2026-08-27-research-knowledge-demo-volatile-lifecycle-core.md`](docs/plans/2026-08-27-research-knowledge-demo-volatile-lifecycle-core.md)
is **verified complete with advisories** and published on `main` at `68de8a5`.
It implements one manually stepped, process-local, volatile Rust host for the
existing sealed Research -> Knowledge workflow and a public Rust-only contract
needed for strict lint reachability. It keeps Tauri commands/events, React,
timers/workers, and background autonomy out of the core, preserves F-01/F-02,
and leaves F-07, F-08, and F-12 unchanged. Its then-current no-adapter/no-panel
claim is historical; the separately approved source plan above supersedes that
readiness state without changing the core increment's evidence.

At that projection checkpoint, the owner-approved
[`2026-08-27-research-knowledge-demo-projection-contract.md`](docs/plans/2026-08-27-research-knowledge-demo-projection-contract.md)
is verified complete with advisories. It adds
one argument-free, read-only projection of the sealed Research -> Knowledge
fixture vocabulary. Explicit refresh cannot start or control a workflow, and
the Command Center graph remains a separate frontend proof. Today's separately
connected lifecycle panel does not change the projection query's read-only
authority or populate or control the fixture graph.

The owner-approved
[`2026-08-27-app-info-runtime-ipc-narrowing-f08.md`](docs/plans/2026-08-27-app-info-runtime-ipc-narrowing-f08.md)
is verified complete with advisories. The existing WebView app-info
client now narrows `unknown` to one exact bounded DTO and renders only fixed
failure copy. It adds no command, event, Rust, capability, dependency, agent
IPC, provider, execution, persistence, network, filesystem, or device effect.
Focused/full verification and target-Mac native smoke pass; no successor plan
is Ready.

The owner-approved
[`production-development-csp-separation-f07`](docs/plans/2026-08-26-production-development-csp-separation-f07.md)
plan is verified complete with advisories. It removes the fixed development
WebSocket from production, removes inline scripts from both policies, adds an
exact development-only CSP, and updates the F-12 static protection atomically.
Focused/full verification and target-Mac development/release smoke evidence
pass. The retained inline-style allowance and split debug-process/browser
evidence are explicit advisories. No IPC, capability, permission, dependency,
provider, model, credential, tool, persistence, filesystem, background work,
or device effect was added.

The owner-approved
[`runtime-start-containment-f01-f02`](docs/plans/2026-08-26-runtime-start-containment-f01-f02.md)
plan is verified complete with advisories under a complete, valid marker. It
applies universal exact returned-runtime identity validation and rejected
nonterminal-run cleanup ownership across existing sealed Rust workflows. It
adds no IPC, UI, provider, model, dependency, network, tool, approval,
persistence, permission, or device effect. Focused/full verification and
independent review pass.

The owner-approved [`ui-native-static-boundary-f12`](docs/plans/2026-08-26-ui-native-static-boundary-f12.md) is complete with passing static-boundary, repository, documentation, security, and diff checks. It enforces the current UI/native isolation baseline only; it does not change Tauri behavior or authorize agent IPC.

The owner-approved [`documentation-reconciliation-f15`](docs/plans/2026-08-26-documentation-reconciliation-f15.md) is complete with passing focused documentation-truth, repository, documentation, security, and diff checks. It corrects only F-15 documentation drift and adds static repository-health coverage; no Rust/Tauri/CSP behavior, capability, dependency, provider, model, workflow, IPC, storage, credential, or frontend runtime behavior changed.

The owner-approved
[`2026-08-25-pr57-transitive-advisory-remediation.md`](docs/plans/2026-08-25-pr57-transitive-advisory-remediation.md)
is **verified complete with advisories** under the complete, valid gate
`pr57-transitive-advisory-remediation`. It is the
second and final bounded PR #57 merge remediation. Scope is limited to six
development-only transitive lockfile resolutions within existing parent
constraints; `package.json`, production behavior, install-script policy,
governance, and application architecture must remain unchanged. The gate began
on clean published baseline `7b5b7e6` after the first remediation marker and
its exact-head documentation workflow passed.

The resolver changed exactly the six declared lockfile nodes. The manifest,
parents, direct graph, install-script allowlist, lockfile version, and product
source remain unchanged. A scripts-disabled clean install, exact metadata and
installed-graph inspection, full and production-only zero-finding npm audits,
complete `npm run verify`, and independent security/code review pass.
Independent architecture review also passes. Published remediation `c3cc49e`
passes every classifier-selected PR check, including the full npm audit and
unchanged accepted Cargo advisory-baseline gate. The completion result is `PASS
WITH ADVISORIES`; no dependency, security, architecture, code-health, or
technical-debt finding remains. The sole advisory is readiness because no new
implementation plan is owner-selected or Ready.

The owner-approved
[`2026-08-25-pr57-linux-clippy-portability.md`](docs/plans/2026-08-25-pr57-linux-clippy-portability.md)
is **verified complete with advisories** under the complete, valid gate
`pr57-linux-clippy-portability`. It is the first of two
separately gated PR #57 merge remediations. The three-file Rust diff changes
only private conditional-compilation visibility; public APIs, macOS behavior,
approval identity, credential validation, non-macOS `UnsupportedPlatform`, and
all authority boundaries remain unchanged.

Local focused, strict Clippy, all-target Rust, full repository, and independent
review evidence passes. Published correction
`6b2675343db8518587068e7175ce0cec9d2f6107` also passes exact-head Linux Rust,
target-Mac Rust, frontend, and documentation workflows. The result is `PASS
WITH ADVISORIES` only for the separate dependency findings.

Exact marker-bound closeout head `3a0ee66` passed its documentation workflow,
and PR #57 squash-merged to `main` at `3987387`. Merged-main Documentation and
CI runs pass every classified job. The completed plans and original reviews
remain immutable historical evidence; this publication reconciliation changes
only live project memory. No implementation plan is owner-selected or Ready.

The prior owner-approved
[`2026-08-11-multi-agent-end-to-end-demonstrations.md`](docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md)
is **verified complete with advisories** under the complete, valid gate
`native-multi-agent-end-to-end-demonstrations` on branch
`codex/native-multi-agent-end-to-end-demonstrations`, created from the verified
`527f0f4` baseline. The bounded increment added a canonical deterministic
acceptance command, demonstration/fixture/acceptance evidence, one strengthened
approval-resolution unit assertion, and one strengthened approved-document/
shared-knowledge contract assertion. It changes no production behavior,
dependency, IPC, tool, executor, provider, or external-I/O boundary.

All twelve demonstrations pass under the owner-approved acceptance scope. Demo
7 validates checkpoint denial and safe manual take-once A-D dispatch as
separate branches. D-090's exact
zero-executable-tool-step invariant remains unchanged, no approval-to-dispatch
bridge exists, and the absent combined chain is an advisory rather than a
completion blocker. Demo 10 proves a task-bound typed rejection with
`NotAttempted` execution while the root remains `Running`; no runtime text is
injected. Native remains sole/default and unwired, and Hermes remains
Deferred/Blocked.

Fresh final validation on macOS 26.6 build 25G72 arm64 passed: the approval unit
1/1, approved-document/shared-knowledge contract 1/1, canonical acceptance
command 447/447, complete `npm run verify`, independent re-review, and final
documentation/security/session checks. The closeout classification is `PASS
WITH ADVISORIES`; the gate is complete and fingerprint-valid. At that closeout
checkpoint, no next plan was owner-selected or Ready and no commit or push had
occurred; the published PR #57 checkpoint above supersedes that publication
state.

The owner-approved
[`2026-08-12-native-multi-agent-command-center-prototype.md`](docs/plans/2026-08-12-native-multi-agent-command-center-prototype.md)
is **verified complete** under gate
`native-multi-agent-command-center-prototype`. The frontend-only implementation
added one lazy, deterministic, fixture-only Command Center route with one
distinct `AgentOrchestrator`, all nine exact roles,
five presentation groups, seven closed scenarios, a non-editable topology,
synchronized structured alternative, inspector, and bounded activity. It adds
only exact `@xyflow/react@12.11.3` and `lucide-react@1.33.0`, with 19
reviewed transitives and zero production vulnerabilities. No Rust/Tauri/IPC/
storage/capability/CSP path changes.

Source-current focused tests pass 142/142, the full frontend suite passes
211/211, and frontend formatting, lint, typecheck, and production build pass.
Strict Rust checks and 481 all-target tests pass with one intentional Hermes
probe ignored. Initial JS+CSS is 76,183 gzip bytes (+1,119 from baseline);
the separate lazy Command Center JS+CSS is 86,350 gzip bytes, both within their
approved budgets. Final current-tree `npm run verify`, including the Tauri
release no-bundle build, passes. Independent review has no source blocker.

Approved Browser Control and Computer Use runtimes now verify the mandatory
browser/Tauri viewports, themes, reduced motion, scroll ownership and inputs,
focus, accessibility, computed overflow, reachability, screenshots, and native
resize. M5 found and corrected one scoped light-theme compact-text contrast
defect. Owner-operated host zoom produced a rendered 125% state at DPR 1.25
and 832×560 CSS pixels inside the approved 1040×700 frame. Browser Control
verified no overflow, clipping, focus, scrolling, or reachability failure, and
reset restored 1040×700 at DPR 1. The full M5 matrix passes and the consolidated
gate report is `PASS WITH ADVISORIES`. Real multi-agent IPC/provider/runtime/
tool work remains Blocked; the separately approved deterministic evidence
increment above is verified complete under its valid marker.

D-087's
[`2026-08-11-engineering-quality-workflow.md`](docs/plans/2026-08-11-engineering-quality-workflow.md)
is **verified complete with advisories and published at `a5d7ba1`**. Its valid
completion marker preserves the sealed fixture-only, proposal-only Coding -> QA
-> Security workflow with no tool, executor, approval request, repository
access, IPC, provider, or effect.

The owner-selected D-088
[`2026-08-11-infrastructure-systems-operations-workflow.md`](docs/plans/2026-08-11-infrastructure-systems-operations-workflow.md)
is **verified complete with advisories** under its complete, valid gate. It
implements exactly two separate deterministic,
fixture-only, proposal-only Rust selectors: Personal Assistant -> Cloud -> QA
-> Security -> Personal synthesis and Personal Assistant -> Systems -> QA ->
Security -> Personal synthesis. Each remains four-task, five-run, one-active-
child, depth-one, and zero-retry. It adds no infrastructure/operations tool,
Terraform or platform command, live inventory, credential, executor, approval
request, IPC, provider, persistence, parallelism, or effect. Source validation
passes, including 25/25 public D-088 contracts, 8/8 domain units, 11/11
orchestrator units, 362 all-target Rust tests with one intentional ignored
probe, and complete `npm run verify`. The two advisories are to decompose the
large private workflow/orchestrator internals before another workflow or live-
tool increment, and to keep string/credential guards defense-in-depth only.

The owner-selected D-089
[`2026-08-12-agent-workflow-internals-decomposition.md`](docs/plans/2026-08-12-agent-workflow-internals-decomposition.md)
is **verified complete with advisories and published at `140f05b`**. Its valid
marker clears D-088's exact next-increment decomposition finding while
preserving all behavior and authority. It adds no workflow capability,
activation, engine, tool, policy, approval, execution, IPC, or effect.

The owner-selected D-090
[`2026-08-11-workflow-automation.md`](docs/plans/2026-08-11-workflow-automation.md)
is **verified complete with advisories** under a complete, valid
`agent-workflow-automation-proposals` gate.
It activates Workflow Automation only for strict typed proposals, catalogs five
immutable templates, keeps document-to-action proposal-only, and permits only
an explicit one-time process-local manual mapping of complete A-D proposals to
the already implemented sealed fixture-only/no-I/O selectors in a fresh
orchestrator. Tool and approval steps remain recognized but non-executable. It
adds no general engine, scheduler, persistence, tool executor, approval
dispatch, provider, IPC/UI, or effect.
Focused tests pass 12/12, the public contract passes 18/18, strict Clippy
passes, all-target Rust passes 393 tests with one intentional ignored probe,
and `npm run verify` passes with 124 frontend and 208 Rust library tests plus
release builds. Independent review is `PASS WITH ADVISORIES`. Final
documentation, repository, security, diff, session-end, and marker checks
pass.

The owner-selected D-091
[`2026-08-11-bounded-agent-parallelism.md`](docs/plans/2026-08-11-bounded-agent-parallelism.md)
is **verified complete with advisories under a complete, valid
`agent-bounded-parallelism` gate**. It implements only one sealed
fixture-only/no-I/O `BoundedParallel` selector with same-thread event
multiplexing. Its three immutable scenarios use exact default-active two,
hard-active/total-child three, depth-one, four-task, five-run, zero-retry,
per-run eight-event, global/workflow/audit 32, root 120-second, and child
60-second cooperative bounds. Results remain catalog-ordinal; failure policy is
exactly `ContinuePartial`, `CancelDependentOnly`, or specialist-lane `FailFast`.
No runtime trait, thread, provider concurrency, general engine, tool, I/O,
IPC/UI, scheduling, persistence, or distributed infrastructure is authorized.
Focused library and public D-091 contracts pass 41/41 each; strict Clippy,
formatting, 481 all-target Rust tests with one ignored, and complete
`npm run verify` pass. Independent code, architecture, and security review is
`PASS WITH ADVISORIES`. Final post-documentation checks pass and deterministic
finalization reports a complete, valid marker. No successor plan was owner-
selected or Ready at D-091 closeout.

The completed
[`2026-08-11-research-knowledge-workflow.md`](docs/plans/2026-08-11-research-knowledge-workflow.md)
is **verified complete with advisories and published at `3efd2c1`** under D-086.
Its `agent-research-knowledge-workflow` gate validates one deterministic,
fixture-only, Rust Personal Assistant -> Research -> Knowledge -> Personal
sequence above the unchanged runtime boundary. Strict bounded stage and final
synthesis contracts preserve only catalog-issued source IDs, partial outcomes
remain truthful, specialists remain sequential depth-one siblings, and no
provider, retrieval tool, network, filesystem discovery, persistence, IPC,
UI, parallelism, or general workflow engine was added.

The completed
[`2026-08-12-agent-memory-approved-documents.md`](docs/plans/2026-08-12-agent-memory-approved-documents.md)
is **verified complete with advisories and published at `5e53f55`** under
D-085. Its `agent-memory-approved-documents` marker was complete,
fingerprint-valid, and `PASS WITH ADVISORIES` for the published tree. Focused
memory tests passed 6/6, document-reader tests passed 9/9, the public contract
passed 10/10, and complete repository validation passed. The accepted residual
is the narrow pure-`std` Unix document-open TOCTOU race; durable memory and
ARB-005 remain Blocked.

The prior owner-selected
[`2026-08-11-agent-governance.md`](docs/plans/2026-08-11-agent-governance.md)
is **verified complete with advisories and published at `2687294`** under gate
`agent-governance`. D-084's bounded non-executing profile, policy, approval,
delegation-matrix, and volatile audit foundation preserves D-082/D-083
ownership and route boundaries.

The prerequisite AgentDefinition/AgentRegistry increment is published at
`f42a6c7`, and the task/orchestration increment is published at `1d1d9d6` with
a valid completion marker. Native remains sole/default. The governance
implementation added no executor, provider, runtime tool lane, IPC, UI, or
specialist activation. D-085 later added only its verified volatile-memory and
approved-document boundary. D-086 implements only its exact sealed workflow.
D-087 implements only the proposal-only engineering plan above; every
repository tool/effect and other later plan remains Blocked.

The prior owner-selected
[`2026-08-11-hermes-acp-transport-spike.md`](docs/plans/2026-08-11-hermes-acp-transport-spike.md)
is **verified complete with advisories** under gate
`hermes-acp-transport-spike`.
Official pinned source and five passing deterministic fixture tests establish
that ACP is a supported structured stdio wire but fails the required
application-owned governance boundary: the hardcoded `hermes-acp` toolset can
execute terminal, filesystem, browser, memory, skill, code, and delegation
actions inside Hermes before Cortexa can authorize them. The candidate also
lacks complete immutable runtime provenance and the installed ACP SDK. The
verdict is NO GO, the ACP ADR is Rejected under D-081, the adapter remains
Draft/Blocked with the disposition **Deferred — evaluated transport and
containment requirements not met**, Native remains sole/default, and no real
Hermes process was run. No Hermes plan is Ready.

The prior owner-selected
[`2026-08-11-hermes-serve-websocket-spike.md`](docs/plans/2026-08-11-hermes-serve-websocket-spike.md)
increment is **verified complete with advisories** under gate
`hermes-serve-websocket-spike`; its transport verdict remains **FAIL / NO-GO at
Milestone 0**. The supplied Hermes Agent `0.20.0` / `v2026.8.3` candidate passed
source/tag/commit, critical-hash, installed-metadata, and sanitized
module-discovery checks. It did not supply a complete immutable manifest for its
4,077-file virtual environment and external Python runtime. Pinned source has no
supported complete no-update/no-credential/no-plugin/zero-tool startup mode,
and reviewed target-Mac `sandbox-exec` cannot prove the required exact dynamic
listener, package-manager execution denial, blanket Unix-socket denial, or
detached-descendant membership/cleanup. The stop condition fired before any
Hermes process, socket, WebSocket, session, provider, credential, harness, or
adapter work. This evidence-backed negative result completed the spike's
prove-or-disprove objective; every Critical finding still blocks the Draft
adapter and all later Hermes implementation.

The owner-approved
[`2026-08-11-native-agent-runtime-boundary.md`](docs/plans/2026-08-11-native-agent-runtime-boundary.md)
plan is verified complete with advisories under gate
`native-agent-runtime-boundary`. The bounded source adds an application-owned
`AgentRuntime`/`RuntimeRun` foundation, the sole/default `NativeAgentRuntime`
composition over the unchanged `InitialGatewayTurn`, and a private deterministic
`MockAgentRuntime` contract fixture. The full repository verification, Rust
all-target suite, architecture/security/code reviews, documentation checks, and
completion marker pass. It does not wire Tauri or React, add a provider/model/
network/process/dependency, integrate Hermes, change the visible frontend mock,
or add runtime selection or automatic fallback. Its clean published baseline
enabled the now-failed Milestone 0 review; Native remains sole/default.

The owner-approved documentation-only
[`2026-08-11-hermes-runtime-architecture-decisions.md`](docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md)
plan was committed separately at `701c061`. D-079 accepted the small
application-owned runtime target and D-080 conditionally selected managed local
`hermes serve` plus TUI-gateway JSON-RPC/WebSocket for a later contained spike.
That documentation increment added no runtime behavior; its clean published
baseline authorized the current native plan only through the owner's separate
implementation prompt.

The completed documentation-only
[`2026-08-11-hermes-adr-transport-revision.md`](docs/plans/2026-08-11-hermes-adr-transport-revision.md)
increment revised the Proposed multi-runtime ADR after the raw TUI-gateway
stdio NO-GO. At that increment's closeout it kept ACP, Hermes serve, and
native-only unselected; added no runtime, dependency, process, source, or
application behavior; and left runtime implementation Blocked pending an owner
decision and fresh readiness. D-079 and D-080, recorded by the active plan
above, supersede that former decision status without changing its historical
evidence.

The owner-selected isolated
[`2026-08-11-hermes-transport-spike.md`](docs/plans/2026-08-11-hermes-transport-spike.md)
plan is verified complete with advisories under gate `hermes-transport-spike`.
It establishes a **NO-GO** for raw TUI-gateway stdio as a supported production
contract at Hermes Agent `0.20.0` / `v2026.8.3`, while proving only bounded
host mechanics against a deterministic fixture. Hermes was neither installed
nor executed, and no production runtime, dependency, application path, ADR
acceptance, or native behavior changed.

The completed documentation-only
[`2026-08-11-project-direction-runtime-boundaries.md`](docs/plans/2026-08-11-project-direction-runtime-boundaries.md)
plan is verified complete with advisories under D-078 and the valid
`repository-project-direction-runtime-boundaries` marker. It records present
owner-only personal scope, native-architecture preservation, a conceptual
framework-neutral runtime-adapter direction, and refinements to the existing
living ExecPlan convention. It authorizes no production source, dependency,
Hermes/OpenClaw installation, runtime behavior, external action, or future
product capability.

The completed
[`cloudflare-demo-fake-keychain-proof.md`](docs/plans/cloudflare-demo-fake-keychain-proof.md)
adds only a fake-value, status-only macOS Keychain read proof. Its
owner-operated target-Mac evidence observed missing,
cancelled/denied-as-cancelled, available, and post-cleanup missing outcomes.
Repeated authorization prompts did not prove stable unsigned-executable
access, so real credential ingestion remains blocked. No fake item remains, and
no real credential, runtime wiring, IPC, Cloudflare change, provider request,
deployment, or traffic exists.

The completed documentation-only
[`cloudflare-demo-real-credential-readiness-plan.md`](docs/plans/cloudflare-demo-real-credential-readiness-plan.md)
requires stable app-specific Keychain access, secret-memory handling, direct
owner transfer, lifecycle controls, dependency review, and private target-Mac
evidence before real demo-token ingestion can be proposed. It creates no
credential, Keychain action, Cloudflare resource, provider request, traffic, or
runtime behavior.

The completed documentation-only
[`cloudflare-demo-macos-identity-secret-boundary-plan.md`](docs/plans/cloudflare-demo-macos-identity-secret-boundary-plan.md)
defines selection criteria for a future signed identity or narrow Keychain ACL,
bounded secret-memory controls, lifecycle, and private target-Mac evidence. It
chooses neither control and creates no signing, Keychain, credential, Cloudflare,
provider, traffic, or runtime capability.

The completed documentation-only
[`cloudflare-demo-signed-identity-decision.md`](docs/plans/cloudflare-demo-signed-identity-decision.md)
selects stable signed macOS identity as the future credential control model and
defines later signing provenance, secret-memory, lifecycle, and private evidence
requirements. It creates no signing asset, Keychain action, credential,
Cloudflare resource, provider request, traffic, or runtime behavior.

The completed documentation-only
[`cloudflare-demo-signed-identity-secret-memory-implementation-plan.md`](docs/plans/cloudflare-demo-signed-identity-secret-memory-implementation-plan.md)
defines a future three-file fake-only proof for signed identity and bounded
secret memory. It adds no code, dependency, signing, Keychain, credential,
Cloudflare, traffic, or runtime behavior.

The completed documentation-only
[`apple-developer-signing-identity-owner-evidence-plan.md`](docs/plans/apple-developer-signing-identity-owner-evidence-plan.md)
defines a future owner-only, read-only private account review to clarify whether
an existing Apple Developer account could later support a signing-identity
proposal. It authorizes no enrollment, purchase, support request, role change,
signing asset, download, installation, Keychain action, credential, Cloudflare,
provider, traffic, deployment, or runtime behavior.

D-074 defers Apple Developer Program enrollment and conditionally recommends an
individual membership only for a later separately approved owner-only proof
while Cortexa remains personally owned. It requires a fresh organization review
before company ownership, seller identity, or shared certificate control is
needed and authorizes no account, signing, Keychain, credential, Cloudflare,
provider, traffic, deployment, or runtime action.

The completed documentation-only
[`apple-developer-individual-enrollment-execution-plan.md`](docs/plans/apple-developer-individual-enrollment-execution-plan.md)
defines the future owner-approved procedure for D-074's conditional individual
enrollment model. It creates no Apple account action, enrollment, payment,
agreement, signing asset, Keychain item, credential, Cloudflare, provider,
traffic, deployment, or runtime behavior.

The owner separately approved and owner-attested the individual enrollment
described by that plan as active, with no signing asset created. This is a
sanitized operational outcome, not account inspection or signing evidence; it
does not authorize a certificate, key, profile, entitlement, Keychain item,
credential, Cloudflare action, provider request, deployment, traffic, or runtime
behavior.

D-075 and the completed documentation-only
[`macos-developer-id-identity-creation-plan.md`](docs/plans/macos-developer-id-identity-creation-plan.md)
select Developer ID Application as the future certificate class and define its
future owner-controlled creation and private target-Mac evidence boundary. They
create no Apple access, certificate, CSR, key, profile, entitlement, signing,
notarization, Keychain action, credential, Cloudflare action, provider request,
deployment, traffic, code, dependency, or runtime behavior.

The separately approved owner-operated Developer ID Application
certificate-creation attempt stopped safely as `unavailable` before a CSR file
was created. Owner-attested sanitized evidence confirms no certificate and no
new named private key exists. Certificate Assistant's Keychain lookup error has
no established root cause. The operational increment is closed without a
signing asset; any diagnostic or recovery work requires a separate
documentation-only remediation plan before another Apple Developer, CSR,
certificate, key, signing, or Keychain action.

The completed documentation-only
[`macos-certificate-assistant-csr-remediation-plan.md`](docs/plans/macos-certificate-assistant-csr-remediation-plan.md)
defines only the future read-only local diagnostic boundary for TS-017. It
names three closed observations, private sanitized evidence, stop conditions,
and no-state-change rollback. It does not authorize diagnostic execution, Apple
access, CSR retry, Keychain action, signing, credential, Cloudflare, provider,
deployment, traffic, code, dependency, or runtime behavior.

The owner separately approved and completed the plan's three read-only local
observations once. Sanitized evidence records observed user/default Keychain
configuration, zero valid code-signing identities, no authorization prompt,
and no state change. The cause remains `not determined`. This outcome does not
authorize repetition, remediation, CSR retry, signing, credential, Cloudflare,
provider, deployment, traffic, code, dependency, or runtime behavior.

D-076 records the owner's decision to defer the signed macOS identity path
after TS-017. It creates no recovery plan. Apple Support and alternate CSR
workflows remain future options requiring separate owner approval and evidence
that preserves the D-072 target-Mac, owner-controlled, non-exported private-key
boundary. No repeat diagnostic, Apple, Keychain, signing, credential,
Cloudflare, provider, deployment, traffic, code, dependency, or runtime work is
Ready.

The completed documentation-only
[`apple-support-ts-017-assistance-plan.md`](docs/plans/apple-support-ts-017-assistance-plan.md)
defines the future owner-only Apple Support contact boundary for TS-017. It
requires minimum sanitized disclosure, no screen sharing or uploads, closed
outcome categories, stop conditions, and no-state-change rollback. It does not
authorize contact or any Apple, Keychain, signing, credential, Cloudflare,
provider, deployment, traffic, code, dependency, or runtime action.

The separately approved owner contact stopped without Apple Support or Apple
Developer access. One unuploaded CSR file and one unused, unexported filesystem
private-key file now exist; encryption and permissions are undetermined and no
certificate exists. The files do not satisfy D-072 and require a separate
documentation-only containment and disposition plan before any action.

The completed documentation-only
[`filesystem-signing-material-disposition-plan.md`](docs/plans/filesystem-signing-material-disposition-plan.md)
selects future abandonment and paired deletion, with a separate operational
approval, exact-target and ambiguity controls, irreversible rollback boundary,
and sanitized evidence. It authorizes no interaction with either file.

The separately approved owner-operated paired disposition is complete by
sanitized owner evidence. Exactly the intended CSR and filesystem private-key
files were deleted, no additional material or remaining copy was observed,
neither file was uploaded or used, and no certificate exists. Ordinary deletion
does not prove cryptographic erasure from APFS/SSD remnants or snapshots. The
material-disposition plan is exhausted; D-072 remains unsatisfied, D-076 remains
in force, and no signing or credential plan is Ready.

D-077 conditionally reopens consideration of exactly one future owner-operated
Apple Support TS-017 contact under the completed
[`apple-support-ts-017-assistance-plan.md`](docs/plans/apple-support-ts-017-assistance-plan.md).
It retains D-076's signed-identity deferral and does not authorize contact. A
separate operational approval must bind one contact to that plan's minimum
sanitized disclosure, privacy limits, stop conditions, no-execution rule, and
closed outcome evidence. No Apple, signing, credential, Cloudflare, provider,
deployment, traffic, code, dependency, or runtime work is Ready.

The separately approved D-077 owner-contact increment closed without an Apple
Support contact. The owner reported no attempted contact, no guidance, no
observed state change, and no determined cause. Its operational authority is
exhausted. The assistance plan remains documentation-only, and any future
contact requires another exact owner approval before external communication.

The owner-attested Free-plan Cloudflare Zero Trust organization exists with
only Cloudflare's default account-member identity provider. It has no Access
application, policy, service token, Worker, route, DNS change, device
enrollment, secret, provider request, or traffic. The completed
documentation-only
[`cloudflare-demo-local-security-boundary.md`](docs/plans/cloudflare-demo-local-security-boundary.md)
defines fake-credential-first Keychain proof and a future local deny-only Worker
boundary. It authorizes no code, dependency, credential, Keychain write,
Cloudflare change, provider request, or traffic. The
completed documentation-only
[`cloudflare-access-worker-no-traffic-deployment.md`](docs/plans/cloudflare-access-worker-no-traffic-deployment.md)
defines the exact future operational scope for a disabled Cloudflare Access and
Worker boundary. It creates no external resource, token, Keychain item, secret,
deployment, route, DNS record, provider request, or runtime behavior. A
separate project-owner approval remains required before any further operational
action.

The completed
[`cloudflare-access-service-token-demo-exception.md`](docs/plans/cloudflare-access-service-token-demo-exception.md)
records the narrow demo-only authentication exception while leaving all
external resources and credentials blocked. The completed
[`cloudflare-synthetic-demo-gateway-decision.md`](docs/plans/cloudflare-synthetic-demo-gateway-decision.md)
selects Cloudflare Workers Free as the demo-only remote gateway candidate while
leaving client authentication and deployment separately blocked.

The completed
[`openai-synthetic-demo-gateway-readiness-plan.md`](docs/plans/openai-synthetic-demo-gateway-readiness-plan.md)
defines the required evidence and approvals before any future implementation can
be proposed.

The completed
[`openai-synthetic-demo-provider-decision.md`](docs/plans/openai-synthetic-demo-provider-decision.md)
supersedes the unpublished Azure Stage B plan for a future synthetic-only demo.

The former Azure Stage B plan is superseded before publication; no Azure
resource or runtime change was made.

No runtime implementation plan is active. The documentation-only
`docs/plans/repository-governance-codex-instruction-hierarchy.md` is verified
complete with `PASS WITH ADVISORIES`; it created the concise root instruction
entry point and detailed Codex master instruction layer without changing the
product roadmap, source, dependencies, hooks, or runtime behavior.

The exact 19-path documentation-only ARB-002A gateway threat model and closed
configuration is verified complete with advisories and published through PR #41
at `36ce9ab` under
`docs/plans/arb-002a-gateway-threat-model-and-configuration.md`. D-064 records
four independent evidence stages and closed Microsoft registration, Azure
network, managed-identity, RBAC, disclosure, and evidence defaults. This plan
does not authorize Stage B provisioning, Stage C synthetic transport, Stage D
real-content activation, or any runtime implementation. ARB-002 remains High
and unresolved, and no later increment may start automatically.

The documentation-only plan
`docs/plans/o006-phase1-azure-openai-provider.md` is verified complete with
advisories, published through PR #39 from source commit `e432681`, and
squash-merged at `4abd49d`; Documentation run `29706772519` passed, and no
publication action remains. D-063 selects Azure OpenAI in Microsoft Foundry as
the sole Phase 1 synthetic-evaluation candidate while preserving every
deployment, ZDR, disclosure, networking, implementation, and ARB-002 block.

The documentation-only plan
`docs/plans/o006-phase1-microsoft-personal-identity.md` is verified
complete with advisories, published through PR #37 from source commit
`e39523f`, and squash-merged at `c458f27`; branch Documentation run
`29705183818` and post-merge Documentation run `29705209977` passed, and no
publication action remains. D-062 selects Microsoft personal identity as the
sole Phase 1 provider, defers Google and Apple, excludes persistent sessions
and automatic email linking, and preserves all implementation and ARB-002
blocks.

The documentation-only O-006/O-007
provider-boundary amendment is verified complete with advisories under gate
`o006-provider-boundary-amendment` and
`docs/plans/o006-o007-staged-gateway-identity-retention-decisions.md`. It records
current absence; separate identity-provider, cloud-hosting, and AI
model-provider boundaries; a consumer/prosumer Phase 1; a later enterprise Phase
2; Azure-first one-primary-cloud hosting with deferred portability; and D-061's
provider-specific verified-ZDR, data-classification, logging, and disclosure
policy. It changes no application source or behavior and does not implement
ARB-002. Source commit `4b474b4` passed branch Documentation run `29703530854`;
PR #35 squash-merged it at `853da62`, and post-merge Documentation run
`29703588215` passed. No publication action remains. D-062 later selects the
Phase 1 identity provider; D-063 now selects the Phase 1 AI-provider candidate,
while exact Microsoft and Azure operational evidence remains open.

The documentation-only High-severity advisory disposition is verified complete
with advisories under
`docs/plans/remediation-high-severity-advisory-disposition.md`, published
through PR #33, and squash-merged at `7bf1a5c` from source commit `26f68b4`.
Branch and post-merge Documentation passed, and no publication action remains.
It identifies no `REMEDIATE NOW` item and authorizes no product work: ARB-002
remains decision-required, four findings remain blocked on future capabilities,
two remain High with explicit deferred triggers, and ARB-044 remains
superseded. No later product or remediation increment is Ready.

The D-058 dual-self-hosted-runner correction
is verified complete under `docs/plans/meta-risk-based-ci.md`, published through
PR #30, and squash-merged at `1780d7f` from implementation commit `9a2c75d` and
documentation closeout commit `da08573`. Its exact 11-path post-publication
project-memory reconciliation was published through PR #31 and squash-merged at
`74a8d2c`; no D-058 publication action remains. ARB-002 is not Ready because
D-062's identity evidence, O-006's AI-provider configuration, provider-specific
ZDR evidence, deployment, and the required
threat-model and implementation approvals remain unresolved.

Meta Increment 8 Prompt Library Reorganization is verified complete under
`docs/increments/meta-prompt-library-reorganization.md`, published through PR
#25, and squash-merged at `d26b5e1`. It reorganizes only the copy-paste prompt
library, active guidance references, D-055, current project memory, and
mandatory closeout evidence. Product source, behavior, dependencies, Tauri,
storage, permissions, skills, and hooks remain excluded.

Repository Workflow Increment trusted self-hosted runner routing is verified
complete with advisories under `docs/plans/repository-self-hosted-runner.md` and
squash-merged through PR #24 at `eaf6c9f`.

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

Increment 4V terminal approval audit is verified complete and published under
`docs/plans/04v-bind-initial-terminal-approval-audit.md`. Reconstructed source
commit `ec919e9` passed hosted CI, Documentation, and Security before PR #23
was squash-merged at `6e6f91d`. Original reviewed commit `3440ce9` remains
preserved, and the `04v` marker remains complete and valid. No later product or
remediation increment is Ready.

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
docs/plans/o006-o007-staged-gateway-identity-retention-decisions.md
docs/plans/o006-phase1-microsoft-personal-identity.md
docs/plans/remediation-high-severity-advisory-disposition.md
docs/plans/meta-01-branding-foundation.md
docs/plans/meta-02-engineering-operating-system.md
docs/plans/meta-03-codex-automation.md
docs/plans/meta-05-repository-health.md
docs/plans/repository-dependency-baseline-compatibility.md
docs/plans/meta-07-verified-application-icon-rollout.md
docs/plans/meta-risk-based-ci.md
```

Increments 2C and 2D were verified on the Apple Silicon target Mac.

## Plan rules

A plan must contain:

- Goal and user-visible outcome.
- Current-state evidence, existing behavior, and constraints.
- Scope, affected components, and explicit non-goals.
- Interfaces and invariants that must remain stable.
- Files expected to change.
- Ordered implementation milestones and dated progress.
- Security and privacy considerations.
- Tests and verification commands.
- Risks, discoveries, and decisions made while work proceeds.
- Rollback or failure strategy.
- Exit criteria.
- Final results based on observed evidence.
- Documentation updates.

Treat the plan as a living implementation document. Keep its milestones,
decisions, discoveries, progress, verification, and final results synchronized
with actual work. A plan alone grants no authority. When an exact task already
authorizes implementation, update the plan, begin the required gate, and
continue in the same task unless the request is analysis-only or a stop
condition is reached.

## Plan status values

- **Draft** — still being designed.
- **Ready** — enough information exists to implement.
- **Active** — implementation is in progress or verification remains.
- **Blocked** — a prerequisite prevents progress.
- **Complete** — acceptance criteria and verification are complete.
- **Stopped** — explicitly halted before implementation and has no completion evidence.
- **Superseded** — replaced by another plan.

## Plan index

| Plan                                             | Status   | Owner              | Last updated |
| ------------------------------------------------ | -------- | ------------------ | ------------ |
| P3 App Sandbox containment re-review             | Complete | Project owner      | 2026-09-02   |
| P3 containment bootstrap trust decision          | Complete | Project owner      | 2026-09-02   |
| P3-2 containment primitive selection             | Complete | Project owner      | 2026-09-02   |
| Personal Assistant v0 build-child containment    | Complete | Project owner      | 2026-09-01   |
| Personal Assistant v0 account-directory boundary | Complete | Project owner      | 2026-09-01   |
| Personal Assistant v0 evidence-privacy protocol  | Complete | Project owner      | 2026-09-01   |
| Personal Assistant v0 signing-security map       | Complete | Project owner      | 2026-09-01   |
| Personal Assistant v0 capability program         | Complete | Project owner      | 2026-08-28   |
| V0-1 empty-tool turn and minimal host            | Complete | Project owner      | 2026-08-28   |
| V0-1 Linux Clippy portability correction         | Complete | Project owner      | 2026-08-28   |
| V0-2 volatile lifecycle and presentation journal | Complete | Project owner      | 2026-08-28   |
| V0-3 fake signed-client secret owner             | Blocked  | Project owner      | 2026-08-28   |
| V0-4 local deny-only Access verifier             | Blocked  | Project owner      | 2026-08-28   |
| V0-5 Cloudflare no-traffic provisioning          | Blocked  | Project owner      | 2026-08-28   |
| V0-6 direct Rust HTTPS dependency decision       | Blocked  | Project owner      | 2026-08-28   |
| V0-7 fixed-origin Rust transport                 | Blocked  | Project owner      | 2026-08-28   |
| V0-8 real demo credential ingestion              | Blocked  | Project owner      | 2026-08-28   |
| V0-9 Access authentication-only rehearsal        | Blocked  | Project owner      | 2026-08-28   |
| V0-10 OpenAI adapter with fake upstream          | Blocked  | Project owner      | 2026-08-28   |
| V0-11 synthetic-v1 Tauri presentation            | Blocked  | Project owner      | 2026-08-28   |
| V0-12 no-traffic provider provisioning           | Blocked  | Project owner      | 2026-08-28   |
| V0-13 live synthetic Stage C rehearsal           | Blocked  | Project owner      | 2026-08-28   |
| V0-14 private real-prompt admission decision     | Blocked  | Project owner      | 2026-08-28   |
| Native multi-agent end-to-end demonstrations     | Complete | Project owner      | 2026-08-25   |
| Increment 2B-1 SQLite migration skeleton         | Complete | Project maintainer | 2026-07-13   |
| Increment 2C storage startup integration         | Complete | Project maintainer | 2026-07-13   |
| Increment 2D menu-bar/window lifecycle           | Complete | Project maintainer | 2026-07-13   |
| Increment 2E React application shell             | Complete | Project maintainer | 2026-07-13   |
| Increment 2F mocked interaction shell            | Complete | Project maintainer | 2026-07-13   |
| Increment 2G integration hardening               | Complete | Project maintainer | 2026-07-13   |
| Increment 3A in-memory conversation sessions     | Complete | Project maintainer | 2026-07-13   |
| Increment 3B mock context provenance             | Complete | Project maintainer | 2026-07-13   |
| Increment 3C simulated tool result               | Complete | Project maintainer | 2026-07-13   |
| Increment 3D bounded mock-loop completion        | Complete | Project maintainer | 2026-07-14   |
| Increment 4A gateway protocol contract           | Complete | Project maintainer | 2026-07-14   |
| Increment 4B local tool-schema validation        | Complete | Project maintainer | 2026-07-14   |
| Increment 4C trusted policy-input binding        | Complete | Project maintainer | 2026-07-14   |
| Increment 4D exact approval binding              | Complete | Project maintainer | 2026-07-14   |
| Increment 4E trusted approval decision source    | Complete | Project maintainer | 2026-07-14   |
| Increment 4F Cortexa product display rename      | Complete | Project maintainer | 2026-07-14   |
| Workflow Increment 4G post-increment gate        | Complete | Project maintainer | 2026-07-14   |
| Increment 4H typed approval-audit adapter        | Complete | Project maintainer | 2026-07-15   |
| Increment 4I remove generic audit scaffold       | Complete | Project maintainer | 2026-07-15   |
| Workflow Increment 4J deletion fingerprint       | Complete | Project maintainer | 2026-07-15   |
| Increment 4K remove legacy provider scaffold     | Complete | Project maintainer | 2026-07-15   |
| Increment 4L remove legacy memory scaffold       | Complete | Project maintainer | 2026-07-15   |
| Increment 4M remove legacy platform scaffold     | Complete | Project maintainer | 2026-07-15   |
| Increment 4N bounded initial gateway request     | Complete | Project maintainer | 2026-07-15   |
| Increment 4O bound initial gateway turn          | Complete | Project maintainer | 2026-07-15   |
| Increment 4P schema-bound initial events         | Complete | Project maintainer | 2026-07-15   |
| Increment 4Q terminal initial function release   | Complete | Project maintainer | 2026-07-15   |
| Increment 4R terminal initial policy binding     | Complete | Project maintainer | 2026-07-15   |
| Increment 4S terminal approval presentation      | Complete | Project maintainer | 2026-07-15   |
| Increment 4T terminal approval resolution        | Complete | Project maintainer | 2026-07-15   |
| Increment 4U approval run termination            | Complete | Project maintainer | 2026-07-15   |
| Increment 4V terminal approval audit binding     | Complete | Project maintainer | 2026-07-18   |
| Meta Increment 1 branding foundation             | Complete | Project maintainer | 2026-07-15   |
| Meta Increment 2 engineering operating system    | Complete | Project maintainer | 2026-07-15   |
| Meta Increment 3 Codex automation                | Complete | Project maintainer | 2026-07-15   |
| Meta Increment 4 executive documentation         | Stopped  | Project maintainer | 2026-07-16   |
| Meta Increment 5 repository health               | Complete | Project maintainer | 2026-07-16   |
| Meta Increment 6 product readiness audit         | Complete | Project maintainer | 2026-07-16   |
| Repository dependency baseline compatibility     | Complete | Project maintainer | 2026-07-16   |
| Meta Increment 7 application icon rollout        | Complete | Project maintainer | 2026-07-16   |
| Meta Increment 8 Prompt Library Reorganization   | Complete | Project maintainer | 2026-07-17   |
| ARB-022 memory reconciliation                    | Complete | Project maintainer | 2026-07-16   |
| Repository self-hosted runner routing            | Complete | Project maintainer | 2026-07-17   |
| Repository risk-based GitHub Actions validation  | Complete | Project maintainer | 2026-07-18   |

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

## Phase 4 Increment 4V bind initial terminal approval audit - complete

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
passed before commit. No manual check applies. Original commit `3440ce9`
remains preserved on
`codex/feature/bind-terminal-approval-audit-pre-refresh`; reconstructed source
commit `ec919e9` passed hosted CI, Documentation, and Security, and PR #23 was
squash-merged at `6e6f91d`. No later product or remediation increment is Ready.

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
