# Security policy and development guardrails

Status: Authoritative security policy
Last updated: 2026-09-03

Use `SECURITY_CHECKLIST.md` for change and release review. `ARCHITECTURE.md`
identifies which security boundaries are current, mocked, planned, or
prohibited.

## D-120 fixed local private-lane direction

D-120 authorizes documentation evidence planning only. A separately approved
plan may assess one fixed, nonselectable local/no-auth `real-content-v2`
topology before V0-13. No engine, model, artifact, dependency, filesystem,
transport, Tauri/UI, credential, provider, source, or operational authority is
created. Fixed local-v2 is not admission of D-119's local candidate; all ten
catalog entries remain `candidate_blocked`, and no handle or selector exists.

Local/no-auth means only no model-provider authentication or external inference
connection. It does not satisfy, defer, replace, or waive D-062/D-094 Cortexa
owner authentication, which remains independently mandatory and Blocked. It
creates no provider credential or direct/native custody exception; D-060 keeps
every cloud/provider credential and credential-bearing OAuth result gateway-
owned. D-061 remains controlling for every external provider.

“Local” is not privacy evidence. Provider ZDR can become inapplicable only
after the exact complete engine, model/artifact, dependency,
acquisition/update, telemetry, crash, cache/support, and runtime lifecycle
proves no external processing and no DNS, socket, or network egress whatsoever,
including prompt, output, derived content, identifying/model/artifact metadata,
telemetry, licensing, updates, crash/support traffic, embeddings, and fallback.
Target-Mac non-observation is required but cannot substitute for source,
feature, and dependency proof. Ambiguity keeps the lane Blocked.

An in-process engine and every native dependency become part of Cortexa's
trusted computing base but gain no policy, configuration, identity, or
authorization authority. The exact source and feature graph must exclude
network access and Keychain, environment-secret, account-state, default-
credential-chain, gateway-token, provider-credential, certificate, private-key,
signing-material, and unrelated secret-memory reachability. Failure to exclude
either network or secret access leaves the candidate Blocked pending a
separately accepted containment or isolation architecture. A helper process,
localhost service, WebView request, provider SDK, plugin, dynamic runtime, or
hidden updater is not an implicit remedy or D-118 bypass.

Model artifacts remain untrusted. Before any filesystem authority, a later plan
must freeze the exact engine, model, quantization, tokenizer, prompt template,
format, version, cryptographic digest, provenance, license/use rights, size,
parser, target hardware, permissions, corruption/mismatch handling, removal,
and no-arbitrary-path policy. No network acquisition, runtime download, repair,
update, replacement, or fallback is authorized. Absent a separately accepted
acquisition boundary, evaluation is limited to one exact pre-provisioned or
bundled artifact.

Every future lane remains one explicit foreground action and one process-wide
request with an immutable Rust-owned identity/configuration snapshot, empty
tools, bounded input/events/output/queues/resources, zero retry/fallback,
terminal idempotent cancellation, and closed redacted errors. Cancellation must
close ingress at one serialized point, signal only the original work, and stop
or boundedly join every engine, native callback, accelerator, buffer, mapping,
and support owner. Cleanup ambiguity retains private Rust ownership and denies
retry, replacement, fallback, and restart until quiescence is positively
proved. If proof never arrives, ownership remains until process termination; no
deadline, UI terminal state, dropped future, or late-result filter releases it.
Every malformed, foreign, stale-generation, duplicate, out-of-order,
oversized, or post-terminal result is rejected before any state, UI, IPC, log,
evidence, persistence, retry, or follow-on dispatch.

Synthetic-v1, historical V0-14, D-118 `no_eligible_client`, V0-3/V0-7, all ten
D-107 blockers, empty tools, no fallback, volatility, and no-device-effect
boundaries remain unchanged and Blocked. Local personal-text classification,
visible local-processing disclosure, bounded volatile handling, content-free
logs, cache/temp/crash policy, deletion/removal, incident response, and target-
Mac evidence remain mandatory under separate approvals.

## D-119 closed post-v0 connection-profile direction

D-119 accepts a documentation direction, not an operational profile. Exactly
ten application-owned provider/authentication candidates exist in the closed
catalog record and every one is `candidate_blocked`. The blocked catalog has no
selection handle. Only a separately approved future catalog may issue an
opaque Rust-owned handle for a fully admitted entry and atomically bind its
provider, auth, model, endpoint, credential owner, disclosure, limits,
cancellation, cleanup, late-result, and lifecycle policy. Raw authority fields
and secrets must never cross IPC.

Direct OpenAI and Azure OpenAI are distinct security boundaries. ChatGPT and
other consumer logins, subscriptions, cookies, CLI sessions, environment
variables, shared credential files, metadata services, and ambient SDK chains
are not provider API authority. Any future OAuth flow requires the system
browser, authorization code with PKCE S256, unpredictable state, OIDC nonce
when applicable, one exact redirect, minimal scope, one bounded attempt, and
closed replay, cancellation, cleanup, and late-result handling. Provider auth
does not replace Cortexa identity. Authentication admission and model-
processing disclosure/admission remain separate explicit user actions.

D-060 catalog-wide gateway credential custody, D-021's current OpenAI/gateway
contract, and D-061 exact provider-approved external-processing/ZDR evidence
remain controlling. Any direct/native credential design requires separately
accepted reconciliation of every applicable decision. No fallback is allowed.
Cancellation or local result disposal does not prove the browser, provider,
network, credential source, OS, or local engine stopped. D-118 remains
`no_eligible_client`; no credential, auth, provider, network, local-model,
transport, persistence, signing, or device authority is granted.

## D-118 V0-6 direct Rust HTTPS dependency decision

D-118 accepts `no_eligible_client` for the five exact variants in the frozen
V0-6 comparison. No dependency or transport is selected. Every hostname-based
variant reaches blocking operating-system DNS work that cannot be aborted or
boundedly joined after start. Returning a timeout, dropping a future, or
rejecting a late result cannot establish cleanup or quiescence while that work
may survive. Reqwest 0.13.4 also cannot impose exact response-header and
read-buffer caps before retention.

The hypothetical lock graphs and their transitive packages are supply-chain
inventory only; they grant no dependency, feature, source, runtime, or network
authority. The frozen evidence supports this scoped negative decision, not a
universal claim that Rust HTTPS is impossible. V0-3 and V0-7 remain `Blocked`.
A later V0-7 proposal must also replace its contradictory fake-only proof with
hermetic actual-client TLS, routing, bound, cancellation, and socket-cleanup
tests while keeping production trust and routing non-injectable. Any changed
architecture or reconsideration of the hard cancellation requirement needs a
separate owner-approved plan.

## Proposed D-117 late-result rejection boundary

D-117 proposes `late_result_rejection_not_accepted`. Current source contains no
private-key attempt host or ownership-bound result ingress, so it cannot prove
that trusted Rust rejects post-terminal results before further
application-owned mutation. Missing, ambiguous, contradictory, unbounded, or
drifted evidence fails closed as `boundary_failed`.

Any future positive proof would require one Rust-issued non-reusable opaque
attempt handle and monotonic epoch, one serialized result-admission and
terminalization point, exact ownership-bound correlation, monotonic terminal
state, bounded retained state, closed redacted errors, and rejection before
any additional application-owned state, evidence, UI, IPC, readiness,
follow-on operation, or external dispatch. These are future evidence
requirements, not current capability or a selected mechanism.

Agent-runtime, fixture, demo, gateway, approval, and orchestration event
rejection remains non-transferable. Result rejection or disposal does not stop
or undo synchronous private-key use, prevent interaction, release uncertain
operation ownership, prove cleanup/quiescence, or establish absence of
OS-managed effects. D-107 remains 8/11, D-108 remains additively 9/10, all ten
contracts remain unproved, and readiness remains `Blocked`.

## Proposed D-116 hard-deadline and cancellation boundary

D-116 proposes `deadline_contract_not_accepted`. Synchronous lookup, sign, and
verify APIs expose no hard deadline or cancellation parameter; a timer, dropped
future, worker, or late-result filter does not stop an in-flight private-key
operation or prove cleanup and quiescence. The D-107 row remains unproved.

## Proposed D-115 interaction-denial boundary

D-115 proposes `interaction_denial_not_accepted`. Lookup-time interaction
controls and absence of an observed prompt do not prove that retrieval or use
of an already-held identity cannot display UI. Result rejection is not prompt
prevention. The D-107 row remains unproved.

## Proposed D-114 fixed-algorithm boundary

D-114 proposes `algorithm_contract_not_accepted`. The current record contains
no immutable application-owned Developer ID key-type/algorithm pair and no
safe preflight boundary. Library defaults, capability results, inputs,
fallbacks, and errors cannot select the algorithm. The D-107 row remains
unproved.

## Proposed D-113 private-key non-export boundary

D-113 proposes `nonexport_contract_not_accepted`. A non-exporting happy path or
opaque native reference does not prove private-key bytes and external
representations are unreachable across production, helper, test, error, log,
debug, serialization, persistence, and DTO surfaces. The D-107 row remains
unproved.

These four additive proposals reconcile historical numbering only if accepted.
They do not retroactively validate predecessor reports or markers, change
D-107 8/11 or D-108 9/10, or reduce the exact ten unproved contracts. Readiness
remains `Blocked`; no Keychain, certificate, private-key, signing, provider,
product, target-Mac, or external-system authority is created.

## Browserslist 4.28.7 supply-chain remediation

PR #102's required audit reported GHSA-c83g-rgw3-j3cx / CVE-2026-73089 and
GHSA-73wf-gq98-2v4g / CVE-2026-73088 against the existing development-only
`browserslist@4.28.2` node. Both advisories affect releases through 4.28.6 and
identify 4.28.7 as patched. The path remains
`@vitejs/plugin-react@4.7.0` -> `@babel/core@7.29.7` ->
`@babel/helper-compilation-targets@7.29.7` -> `browserslist@4.28.7`.

The bounded npm resolution changes only Browserslist 4.28.7 and its raised
support floors: `baseline-browser-mapping@2.10.44`,
`caniuse-lite@1.0.30001806`, `electron-to-chromium@1.5.393`, and
`node-releases@2.0.51`. Their licenses are MIT, Apache-2.0, CC-BY-4.0, ISC, and
MIT respectively. Every moved node remains development-only, registry-
resolved, SHA-512 integrity-bound, engine-compatible, and free of an install
lifecycle hook.

`update-browserslist-db@1.2.3`, `package.json`, the parent graph, lockfile
topology, durable overrides, install-script allowlist, audit threshold,
application/native source, workflows, and trust boundaries are unchanged. Full
and production-only npm audits report zero vulnerabilities, complete local
verification passes, and exact remediation CI run `33694943603` passes secret
scanning, JavaScript audit, and the unchanged accepted Rust advisory-baseline
gate. This is dependency evidence, not new product, provider, network,
permission, execution, or device authority.

## D-111 account and Keychain scope boundary

D-111 rejects default/search-list, account/home/path, environment, access-group
labels, disabled features, and fallback as application-owned identity-scope
proof. It creates no Keychain authority; all other D-107 contracts remain
unproved.

## D-110 exact signer-binding boundary

D-110 selects `signer_binding_not_accepted`. A certificate/private-key pair,
label, fingerprint, filter, default Keychain, search list, caller value, or
ambient state is not an immutable application-owned expected Developer ID
Application signer binding. No future implementation may substitute any such
selector for trusted-Rust policy and adapter-private comparison over fixed
application-owned material. This decision creates no signing/Keychain authority
and leaves every other D-107 contract independently unproved.

## D-109 opaque identity-reference issuance boundary

D-109 selects `reference_issuance_not_accepted` for exactly
`opaque_prebound_identity_contract`. Repository source does not contain an
application-owned issuer of one no-input, attempt-bound opaque signing-identity
reference that avoids lookup, enumeration, selection, fallback,
default/search-list, account, home, and path authority. An opaque native
reference proves neither selection provenance nor permitted scope.

No later source may turn fixed labels, a certificate filter, a fingerprint, a
default Keychain, a search list, environment state, or a caller-provided value
into a substitute issuer. Any future positive design needs separately approved
contracts for trusted-Rust ownership, private adapter issuance, non-export,
attempt binding, cleanup/quarantine, and all remaining D-107 rows. This
decision creates no Keychain, certificate, private-key, signing, Tauri, or
runtime authority. Platform effects remain independently unproved.

## D-108 D-102 non-build applicability boundary

D-108 accepts exactly the governance disposition
`split_documented_for_frozen_non_build_class` for
`in_process_security_framework_ephemeral_challenge_proof_v1`. This means only
that D-102's build-child subject is not invoked by the frozen conceptual class
while that class contains no build, helper, child, subprocess, external
executable, bundle, artifact, staged or generated file, application- or Rust-
dependency-authored/selected/requested filesystem/network/socket/IPC API,
`codesign`, product-signing operation, or application- or Rust-dependency-
selected dynamic-loader/JIT/plugin/external-code path. Its only direct
application operations are the frozen fresh 32-byte challenge generation, one
data-signature creation, and one paired-public-key verification. OS-managed
access, IPC, network, or loader effects internal to those fixed operations are
not accepted by this split and remain separately unproved.

This is not a waiver, satisfaction, replacement, residual-risk acceptance, or
generic `not_applicable` result. Any named trigger, caller/model/WebView/
environment-selected authority, unmodeled effect, fallback, retry, or
substitution definitively leaves the frozen class and makes D-102 fully
mandatory. Ambiguity, missing facts, contradiction, scope expansion, or
implementation drift records `boundary_failed` and also makes D-102 mandatory.
D-102 remains fully binding for every product-signing, executable-generating,
artifact-generating, build-bearing, helper, child, subprocess, or `codesign`
path.

Absence of an application child or direct application network request does not
prove absence of Keychain database, `securityd`, directory, cache, log, IPC,
trust, revocation, process-metadata, or OS-managed network effects. Those
remain independently `contract_unproven` under `platform_effect_contract`.
Ten D-107 contracts, including that platform-effect contract, remain unproved;
the existing factual D-100 record remains `contract_unproven`, and the long
D-108 governance disposition is not a D-100 evidence token. Historical D-107
remains 8 documented / 11 unproved; the additive current interpretation is
9/10. The candidate and every successor remain Blocked.

## D-107 in-process key-use containment classification boundary

D-107 records `not_eligible_or_unproven` for exactly
`in_process_security_framework_ephemeral_challenge_proof_v1`. The review
documents candidate distinctness, pinned dependency provenance, a narrow safe
primitive surface, fresh-challenge and single-sign capabilities, paired-key
verification, D-100 evidence minimization, and the claim/history ceiling.
Eleven complete-boundary contracts remain `contract_unproven`.

No future operation may search an ambient/default Keychain, accept an identity,
label, certificate, path, account, algorithm, challenge, or result from a
caller, or expose a generic signing oracle. A permissible design would require
one separately proven opaque application-domain identity reference, one fixed
algorithm, one fresh application-owned challenge, one sign, one verify, closed
errors, no prompt, hard cancellation, late-use rejection, terminal ownership,
and D-100-minimized evidence.

The pinned safe crate's ability to return an external key representation and to
debug identity/key objects is outside the candidate and must remain unreachable
from any later source. The current review does not prove that prohibition in an
implementation. It also does not prove absence of Keychain database,
`securityd`, trust, cache, log, IPC, process-metadata, or OS-managed network
effects.

Eliminating a child and filesystem artifact is scope reduction, not a D-102
containment proof or waiver. Data-signature verification is not code signing.
D-096 through D-106 remain unchanged, no operational check ran, and no
successor is Ready.

## D-106 codeless signing-fixture classification boundary

D-106 records `not_eligible_or_unproven` for exactly
`repository_owned_codeless_bundle_signing_fixture_v1`. The frozen public-source
review documents only candidate distinctness, codeless signature storage and
resource sealing, the hardened-runtime nonclaim, preservation of operational
boundaries, and the claim ceiling. Eight exact shape, repository-provenance,
Developer ID-on-codeless, present-private-key-use, verification, identifier/
designated-requirement, total no-build, and D-102 applicability-split contracts
remain `contract_unproven`.

The factual D-100 record is exact compact JSON with outcome
`contract_unproven`; eligibility language is not evidence. Removing a proposed
build graph is scope reduction, not containment, implementation evidence, or a
D-102 waiver. D-102 remains fully applicable to every executable-generating or
product-build path. Any later staging, hashing, verifier, sanitizer,
`/usr/bin/codesign`, Keychain/Security framework, filesystem, process,
deadline, cancellation, reaping, quiescence, late-result, effect, cleanup, and
quarantine boundary remains unresolved and separately reviewable.

The only external contacts were approved read-only Git remote synchronization/
checks and reads of the three frozen first-party Apple public-documentation
pages. No fixture, source, dependency, configuration, entitlement, or product/
signing/target-Mac operational process or state change exists. D-096 present-session use remains
`not_run`; the codeless
class cannot prove a signed Cortexa application, hardened runtime, product
identity, Gatekeeper, notarization, distribution, release, historical
non-export, technical nonextractability, exclusive custody, or V0-3 readiness.
D-097/D-098 and D-100 through D-105 remain unchanged. No successor is Ready.

## D-105 App Sandbox containment re-review boundary

D-105 records `no_eligible_candidate_after_d104_rereview` for exactly
`app_sandbox_build_helper_plus_libsystem_supervision_v2`. D-104 removes only
the prior Developer ID circularity classification. Current Apple documentation
does not jointly establish the frozen candidate's bootstrap provenance, fixed
graph and executable identity, closed environment/descriptors, exact
filesystem/network denial, detached-descendant ownership, complete shutdown,
reaping, race-free quiescence, pipe closure, descriptor-bound cleanup,
quarantine, or bounded platform effects. All 22 D-102 contracts therefore
remain `contract_unproven`; all ten absent P3-3 source checks remain `not_run`.

The negative decision is not a residual-risk acceptance, operational failure,
or universal impossibility claim. It adds no authority, source, dependency,
configuration, entitlement, helper, build, process, target-Mac, Apple, signing,
credential, provider, product, or external action. P3-3 and every operational
successor remain Blocked. D-097 through D-104 and every historical Failed,
Pending, and Not-run fact remain unchanged.

## D-104 bootstrap trust-class boundary

D-104 defines only a future documentation distinction between an identity-free,
disposable ad-hoc sandbox-activation seal and P4's later Developer ID signer
binding. The first class carries no Apple-issued identity, certificate, private
key, Team ID, Keychain, provisioning, authentication, distribution, or product
authority; it cannot prove any P4 predicate. It may remove only the historical
signature-class circularity from a separately approved future static candidate
review. It neither selects nor proves a containment primitive.

Every D-102 predicate remains mandatory: exact pre-effect filesystem/network
denial, bootstrap provenance, complete descendant membership across detachment,
containment-wide termination, reaping, race-free quiescence, cleanup, and
D-100-minimized evidence. Broad entitlements and any inferred, unavailable, or
archived-only guarantee remain `contract_unproven`. No source, entitlement,
build, process, target-Mac, Apple, signing, credential, provider, product, or
state-changing external action is authorized. P3-3 and all operational
successors remain Blocked.

## D-103 containment primitive selection boundary

D-103 records a bounded negative static decision: no eligible candidate in the
frozen P3-2 reviewed set. The publicly documented App Sandbox/helper composition
requires entitlements and entitlement-bearing app/helper code signatures.
Developer ID distribution separately uses Developer ID signing; this review
does not equate development/ad hoc signing with P4's later signer proof. Either
a new entitlement or any prerequisite signing state independently fails the
approved P3-2 eligibility gate. The reviewed `waitpid`, `kqueue` `EVFILT_PROC`,
`setpgid`, and process-group contracts do not establish complete application-
owned membership, containment-wide termination, direct-child reaping plus
graph-wide quiescence across detachment/reparenting, or D-102's exact pre-effect
filesystem/network boundary.

Deprecated/private `sandbox-exec`, mutable process groups, and post-hoc scans
remain negative controls. Endpoint Security, Network Extension, system
extensions, and virtual machines cannot be substituted silently: the reviewed
routes require privilege, entitlements, signing, user/global state, or guest
resources. No exact supported OS-shipped macOS 14+ container candidate contract
was identified. Missing or ambiguous public contracts remain
`contract_unproven`; static citations and categorical dispositions are not
runtime evidence or authority.

Only the App Sandbox composition entered one privately candidate/check/attempt-
bound D-100 contract review. Negative controls retain D-102's existing
exclusions and the scope-only classes failed the independent eligibility
screen. Unexpected sources, incomplete provenance, target-derived data, or
malformed evidence is `boundary_failed` and stops without retry.

No source, entitlement, dependency, process, probe, target-Mac, Apple, signing,
or state-changing external operation is part of this decision. Approved read-
only public documentation access was the sole external contact. P3-3 and every
operational successor remain Blocked. D-097's Failed privacy finding, Pending
Open Directory boundary, Not-run signing state, original report/digests, and
missing completion marker remain unchanged.

## Security model

Cortexa treats the model as an untrusted planner. The architecture requires the
trusted Rust core to validate requests, apply deterministic policy, obtain exact
approval where required, execute only registered tools, and record redacted
audit events.

The current repository implements transport-free validation, policy, approval,
cancellation, and a turn-bound in-memory approval-audit adapter. It also has an
unwired non-executing per-agent governance foundation: nine closed profiles,
orchestrator-derived live attribution, profile-aware deterministic policy,
agent-origin approval, an exact delegation matrix, and a bounded volatile audit
family. D-085 adds a separately bounded workflow-local volatile memory store,
selected-record context assembly, an approved-document reader, and one direct
Personal Assistant-to-Knowledge document task. D-086 adds one separately
selected, fixture-only Personal-to-Research-to-Knowledge-to-Personal sequence
with strict structured results, source-ID provenance, truthful partial outcomes,
child-first cancellation, and a content-free volatile workflow journal and
attribution audit. D-087 adds one separately selected fixture-only,
proposal-only Personal-to-Coding-to-QA-to-Security-to-Personal sequence. Its
strict results preserve only application-issued fixture, criterion, proposal,
QA, and evidence references; consequential capabilities are denied data, QA
and Security remain advisory, and final approval requirement is derived without
creating an approval request or execution subject. D-088 adds two further
separately selected fixture-only/no-I/O sequences: Cloud
Infrastructure or Systems Operations followed by QA, Security, and Personal
synthesis. Their immutable built-ins contain only synthetic Terraform/Azure or
sanitized service/log/recovery evidence. Consequential capabilities remain
denied data and no command, credential, live access, approval dispatch, or
effect exists. D-090 adds one strict fixture-only Personal-to-Workflow-
Automation-to-Personal proposal lifecycle and a take-once manual bridge from
complete A-D proposals to those existing sealed selectors. Template E and all
tool/approval steps remain non-executable. Workflow Automation cannot construct
trusted identity, create tasks, issue a token, approve, execute, or select a
destination. D-091 adds one further application-only, fixture-only/no-I/O
bounded-parallel selector. It binds every live event to exact task/run/profile
identity, orders outcome and cancellation handling by sealed catalog ordinal,
isolates task memory and cancellation handles, validates strict bounded
results/synthesis, and quarantines rejected runtime identities until cleanup
succeeds. Its same-thread retained-run model is neither provider/CPU
concurrency nor an app-global budget or session boundary. A successful terminal native or run-termination resolution
cannot leave the initial turn without one typed legacy audit receipt. Agent
governance reserves one audit slot before downstream mutation and records
execution only as `NotAttempted`. There is no live provider transport,
dispatcher, executor, durable or user-facing product memory, platform adapter,
or durable audit. Current resolutions, memory, records, references, and receipts
are volatile and non-authorizing and must not be mistaken for an end-to-end
security path.

The Active Command Center prototype is a separate untrusted WebView
presentation boundary. Its `command-center-demo-v1` data is frontend-owned,
closed, bounded, fixture-derived, redacted, and persistently labeled simulated.
Search, filters, selection, graph viewport controls, inspector, and activity
change only feature-local presentation state. Those fixture controls invoke no
Tauri command or event listener and reach no network, clipboard, storage,
filesystem, provider, tool, approval, policy, audit, runtime, or device action.
Fixture IDs are not trusted Rust identities.

The separately rendered Research/Knowledge panel performs one explicit,
argument-free read-only projection query. Rust owns the fixed synthetic
projection identity and returns a closed bounded DTO; the WebView
runtime-narrows every field and maps every rejection to fixed unavailable copy.
The query starts no workflow, accepts no caller-selected identity, registers no
event, and exposes no provider, model, network, credential, tool, approval,
persistence, filesystem, background, audit, or device effect. Its volatile data
is persistently labeled `DEMO MODE · SIMULATED AGENT DATA` and remains separate
from both the frontend Command Center fixtures and Rust acceptance workflows.

The separately implemented `ResearchKnowledgeDemoHost` is exposed only through
one narrow Tauri adapter and one prop-free visibly simulated panel in the
selected scenario. Its public production
constructor and no-argument lifecycle methods
cannot accept an agent, task, root, run, request, profile, runtime, workflow,
objective, source, fixture, script, outcome, or runtime event. Every run still
passes through the orchestrator's exact returned-identity and duplicate-live
identity validation. Cleanup-pending state retains the orchestrator and blocks
replacement; persistent Drop-time cleanup failure retains the owner until
process exit and sets a private process-wide sentinel that denies all later
mutating lifecycle operations. That sentinel is not synchronization or
authorization for future IPC. Snapshots and errors are finite, bounded, and
content-free. One mutex-owned Tauri state registers only no-argument
snapshot/start/advance/cancel commands and emits one fixed notification-only
snapshot event. Its WebView client narrows `unknown`, enforces the exact DTO and
journal grammar, and treats only validated command responses as presentation
authority. Malformed, older, or same-revision events cannot mutate presentation;
every parser-valid newer notification requires explicit snapshot recovery. Even
though `core:default` includes WebView event emission, a forged
grammar-valid event can at most fail the panel closed before its next explicit
operation; it cannot render a lifecycle transition or outcome. No caller can
select identity, fixture, script, or outcome, and no provider, model, network,
credential, tool, approval dispatch, persistence, filesystem, thread, timer,
background work, or device effect was added.

The owner approved exact `@xyflow/react@12.11.3` and
`lucide-react@1.33.0` after direct/transitive, license, peer, bundle, and
security review. Their lockfile consequence is 19 reviewed transitives. At that
frontend-only Command Center checkpoint, the production audit reported zero vulnerabilities
while five pre-existing development-only advisories remained unchanged; the
superseding PR #57 remediation below resolves them. React Flow types/imports
stop at one feature adapter. That checkpoint changed no Rust/Tauri capability,
CSP, IPC, permission, or native dependency. The later read-only projection adds
only the exact query described above; capabilities, CSP, permissions, and
dependencies remain unchanged.

PR #57's baseline full development audit reports five vulnerable indirect
package-level findings—four High and one Moderate—across six lockfile nodes,
while its production-only audit and repository secret scan pass. The completed
`pr57-transitive-advisory-remediation` gate advanced only those six
development-only nodes within existing parent ranges. Exact graph, license,
integrity, engine, install-hook, clean-install, full/production zero-audit, and
complete repository verification pass with no manifest, override,
install-script allowlist, audit-policy, application, or governance change.
Independent architecture/security/code review and every classifier-selected
check on exact remediation `c3cc49e` also pass, including the unchanged
accepted Cargo advisory baseline. The completion report is `PASS WITH
ADVISORIES`; no dependency or security finding remains, and the sole advisory
is that no next implementation plan is owner-selected or Ready. The
deterministic marker is complete and valid.

## Non-negotiable invariants

- The WebView cannot execute a generic local action.
- Unknown tools and invalid arguments fail closed.
- Tool schemas use strict validation and reject additional properties.
- Class 3 personal-data modifications always require an exact trusted preview and explicit approval.
- Class 4 actions are not registered in the MVP.
- Class 5 behavior is prohibited.
- Approval is bound to canonical arguments, tool identity, expiry, and one-time consumption.
- Agent-origin approval is additionally bound to exact live agent/task/root/
  parent/runtime/profile/depth/run/request attribution.
- Runtime tool proposals remain rejected; synthetic agent governance has no
  executor and every execution disposition is `NotAttempted`.
- Delegation stays outside `ToolRegistry`; only the orchestrator may create a
  child after the exact Personal Assistant-to-Research matrix and finite limits.
- Memory access requires sealed definition/task/live-runtime attribution,
  including the exact memory profile; no caller or untrusted content supplies a
  grant, namespace, owner, or application-review authority.
- Proposed shared memory is unreadable as approved shared until an exact
  version-checked application review promotes it. Private and task memory never
  transfer across agents or workflows.
- Approved-document access uses opaque workflow-bound references selected by
  trusted application code. Paths remain private, roots cannot be enumerated,
  and one-time read authority grants no general filesystem permission.
- The direct Personal Assistant-to-Knowledge document route and D-086's sealed
  fixture workflow do not alter generic Personal-to-Research delegation.
  Generic or direct Research-to-Knowledge remains denied, specialists cannot
  spawn agents, and only the orchestrator may create the Knowledge sibling after
  validating the exact Research result.
- The sealed workflow permits only three tasks, two non-replenishing sequential
  depth-one children, one active child, four run attempts, 32 runtime events,
  32 generic events, 16 workflow events, 16 matching workflow audit records,
  and zero automatic retries. Overflow fails closed and never replenishes a
  task, child, run, event, or retry budget.
- Research output may reference only IDs from the immutable one-to-eight-source
  application fixture catalog. Knowledge output may preserve only references
  from the validated predecessor Research task and version. Unknown, duplicate,
  remapped, malformed, oversized, or reasoning-bearing output fails closed;
  a valid Research result with missing references remains partial and skips
  Knowledge, a valid incomplete Knowledge result remains partial, and citations
  are never invented.
- D-086 terminal parsing, terminal task output, remaining capacity, next task,
  request, descriptive attribution, and fallback/synthesis input are prepared
  before terminal runtime acceptance. Preparation failure causes zero workflow
  mutation. A continuation start failure cannot reverse an accepted terminal
  event and is exposed only through a closed content-free failure category.
- Research or Knowledge failure/cancellation never forwards raw invalid output;
  only validated results plus typed stage status may reach Personal synthesis.
  Personal synthesis itself must pass the strict bounded V1 result contract,
  preserve exactly the Research outcome's source-ID set, affirm fixture-only
  evidence, disclose partial status when applicable, and match the derived
  complete/partial status; invented references, URLs, live-research claims,
  reasoning, false disclosure, and unknown fields fail the root.
  Research missing-source partials skip Knowledge; Knowledge partials preserve
  validated Research. Root cancellation resolves pending governance, cancels
  the active child before the root, starts no later stage, preserves retryable
  live state on cancellation failure, and rejects late events.
- `ResearchKnowledgeAttribution` is a non-authoritative descriptive snapshot.
  Its run/request identity remains private and redacted, it cannot reconstruct a
  live execution context or grant policy/approval/memory/runtime/execution
  authority, and its journal records contain no objective, fixture content,
  findings, summary, proposal, path, URL, output, or reasoning.
- D-087's sealed engineering workflow permits only four tasks, three non-
  replenishing sequential depth-one children, one active child, five run
  attempts, 32 runtime/generic events, 16 workflow events/audit records, and
  zero retries. Generic, document, D-086, and engineering selectors are
  mutually exclusive, and no specialist can spawn.
- Engineering fixtures, criteria, and evidence use only immutable application-
  issued IDs. Evidence is `ObservedFixture` or `NotRun`; it cannot represent a
  live test pass. Unknown, duplicate, malformed, oversized, reasoning-bearing,
  URL-bearing, identity-supplying, or authority-claiming output fails closed.
- Coding patch text is inert proposal data. File mutation/deletion, path escape,
  dependency/package/test/formatter execution, Git operations, destructive
  shell, credential access, and network access are denied and never dispatched.
  QA cannot approve or fabricate evidence; Security cannot authorize,
  remediate, replace policy, or access/expose secret values.
- Final engineering synthesis must disclose fixture-only, proposal-only, and
  no-execution status. `RequiredBeforeMutation` is application-derived only
  when a patch is proposed; no approval request or execution authority exists.
  Coding, QA, and Security remain tool-ineligible, memory-disabled, and
  `NotAttempted` for execution.
- Each D-088 selector is application-only and mutually exclusive with the other
  selector and every prior workflow. Cloud or Systems, QA, and Security are
  sequential depth-one siblings under four-task, three-child, five-attempt,
  one-active-child, 32-event, 16-workflow/audit-record, and zero-retry limits.
- Cloud accepts only the sealed synthetic Terraform-configuration/Azure-
  architecture scenario. Systems accepts only the sealed synthetic service-
  snapshot/sanitized-log/recovery scenario. Findings and stage transfers may
  reference only application-issued scenario, fixture, criterion, evidence,
  predecessor, and result identities.
- Terraform/platform commands, live inventory/diagnostics, cloud or system
  mutation, IAM/firewall/account changes, service/process control, reboot/
  shutdown, configuration/package/patch operations, privileged shell, VMware/
  backup mutation, credential access/rotation, filesystem/network access, and
  every other consequential operation are denied inert proposal data.
- QA cannot approve or treat `NotRun` as passing evidence. Security cannot
  authorize, remediate, become policy, or claim unavailable credential/
  platform evidence. Final synthesis creates no approval request or executable
  subject. Cloud, Systems, QA, and Security remain tool-ineligible, memory-
  disabled, and `NotAttempted` for execution.
- D-088 string and credential-pattern guards are defense in depth only. They do
  not prove secret absence and cannot authorize any later live, credential,
  tool, command, or effect path; those require separate trusted containment and
  governance.
- D-090's proposal selector is application-only and mutually exclusive with
  every other selector. It permits two sequential depth-one tasks, three run
  attempts, one child, zero retries, and bounded content-free event/audit
  evidence. Only exact immutable A-D templates can become manually dispatchable;
  E remains proposal-only.
- Proposal validation fails closed on unknown/disabled agents, unknown or
  mismatched tools, malformed arguments, duplicate or cyclic dependencies,
  unsupported steps, excessive limits, nested execution, self-modification,
  false authority/effect claims, and noncanonical template content. Known tool
  and approval steps remain non-executable, create no approval subject, and
  cannot issue a token.
- A dispatch token is opaque, non-cloneable, non-serializable, process-local,
  taken once, and consumed on success or every error. A fresh destination maps
  it only to an existing sealed A-D selector. The propagated 120-second
  monotonic deadline is cooperatively checked at trusted destination ingress;
  expiry cancels child-first and starts no successor, but cannot preempt a
  synchronous runtime call already in flight.
- Workflow Automation remains tool-ineligible and memory-disabled; its generic
  delegation route is denied. D-090 invokes neither QA nor Security in the
  proposal lifecycle and adds no policy decision, approval request, executor,
  scheduler, persistence, IPC/UI, provider, external runtime, I/O, credential,
  or device effect.
- D-091 is application-selected and mutually exclusive with every prior
  selector. Specialists and Workflow Automation cannot select it, create a
  child, delegate, or nest a workflow. Its exact limits are depth one, default
  active two, hard active and total child three, four tasks, five run attempts,
  zero retries, eight events per run, 32 applicable records, a 120-second root
  lease, and 60-second child leases capped by the root.
- Every admitted D-091 child has exact distinct task/run/context/profile/memory
  attribution and an application cancellation handle. Cross-run, stale, late,
  terminal, wrong-sequence, or over-cap events fail closed. Task memory remains
  isolated and terminally cleaned; successful result transfer contains only
  strict bounded fixture data, never memory or authority.
- D-091 outcomes are exactly succeeded, failed, cancelled, timed out, or
  skipped in catalog ordinal order. Explicit `ContinuePartial`,
  `CancelDependentOnly`, and specialist-lane `FailFast` policy controls sibling
  and dependent handling. Final synthesis must expose exact source agents,
  statuses, finding IDs, failures, and unresolved issues.
- Root cancellation/expiry sweeps active children deterministically before the
  root/synthesis run. Cancellation failure preserves closed resumable live
  state, and a rejected returned run remains quarantined until cleanup succeeds;
  no root terminal may conceal a live rejected run.
- Every legacy and bounded-parallel runtime start validates the exact
  application-created run/request identity and rejects duplicate live identity.
  A rejected nonterminal run is retained for explicit cancellation cleanup, and
  no new or fallback start proceeds while cleanup is pending.
- Cooperative deadlines cannot preempt a synchronous runtime call already in
  flight. Per-orchestrator limits do not prove provider or app-global capacity,
  and distinct runtime-run identity does not prove provider-session isolation.
  The lexical fixture authority-claim filter is defense in depth only and must
  never authorize a future live/provider/tool/effect path.
- Governance audit is closed, redacted, volatile, capped at 32 subjects, and
  never authorizes an action.
- Untrusted content cannot grant permission or change policy.
- Credentials, authentication codes, private keys, and production API keys are never logged or stored in SQLite.
- Privileged macOS permissions are requested only from a user-initiated feature flow.

## Phase 4 gateway boundary

- Every production AI model-provider credential belongs only to the authenticated product gateway and must be loaded there from server-side managed secret storage. It must never enter the desktop application, WebView, Tauri IPC, SQLite, local logs, crash reports, or audit records.
- No gateway is deployed, no identity provider is integrated, no cloud deployment exists, and no gateway or AI model-provider networking or external transmission is currently permitted.
- D-060 separates identity-provider, cloud-hosting, and AI model-provider approval. Approval in one category grants no approval in another.
- D-062 selects Microsoft personal identity as the sole Phase 1 provider without authorizing implementation. The planned flow uses the system browser, OAuth 2.0 Authorization Code Flow, PKCE S256, `state`, and OIDC `nonce`. The personal-account authority, exact discovery-derived issuer, tenant, separate public desktop client and gateway API resource, gateway audience, loopback redirect, and delegated scope are closed configuration. Work, school, guest, and arbitrary Entra tenants fail closed. Google is deferred until demonstrated demand after Microsoft verification; Apple is deferred until Mac App Store planning or demonstrated demand.
- The canonical external identity key is provider ID plus normalized issuer plus subject. Email is never an identity key and cannot automatically link accounts. Any future explicit cross-provider linking requires a separate approved threat model and reauthentication design.
- Phase 2 may add Entra workforce SSO, tenant-aware validation, group authorization, and enterprise administration. Those controls and other compatible enterprise OIDC or SAML identity providers remain separately approved future capabilities. AWS and Google Cloud accounts are not treated as consumer identity providers.
- Azure Container Apps in Central US is the initial planned hosting target, and `https://api.cortexaai.io` is the reserved origin. The origin remains inactive until DNS, TLS, deployment, authentication, authorization, logging, and security verification pass. Initial production uses one primary cloud. Container portability does not authorize AWS, Google Cloud, active-active multicloud, cloud failover, or a three-cloud release.
- A future trusted `AgentProvider` abstraction may select only separately approved AI model providers under trusted gateway or core policy. No implementation currently exists. Each AI provider requires independent retention, ZDR, data-use, logging, region, and security approval.
- D-066 selects OpenAI as the sole synthetic-demo candidate. Any future credential is gateway-owned and never reaches the desktop or WebView; exact data-control evidence, disclosure, limits, and a separate implementation plan remain mandatory. The owner configured a non-secret project boundary, but no API credential, endpoint integration, or network path exists.
- D-067 selects Cloudflare Workers Free only as the future internal-demo gateway
  candidate. The OpenAI project exists with owner-confirmed synthetic-only
  restrictions, bounded spend/rate settings, one allowed model, and disabled
  API-call logging; its identifier and any future key remain outside the
  repository. A future key may exist only as a Worker secret. No Worker, route,
  secret, client authentication, deployment, or provider traffic exists.
- D-068 permits one future 30-day-maximum Cloudflare Access service token only
  for the owner-only fake-data demo. Its secret is macOS-Keychain-only and
  trusted-Rust-only; the Access policy is restricted to one application and the
  Worker must validate JWT signature, issuer, and exact audience. Revocation and
  route disablement fail closed. No token or Keychain item currently exists,
  and D-064's production 15-minute access-token maximum is unchanged.
- D-069 accepts only the fake-value macOS Keychain read proof. The trusted Rust
  adapter uses fixed service/account labels, exposes no raw value, returns
  closed redacted outcomes, and has no write, delete, enumeration, IPC, WebView,
  SQLite, startup, network, or runtime-consumer path. Owner-operated evidence
  removed both fake items after successful proof. Repeated authorization
  prompts did not prove stable unsigned-executable access; real credential
  ingestion remains blocked.
- D-070 requires a separately approved stable signed application identity or
  narrowly reviewed app-specific Keychain ACL, production secret-memory
  lifecycle, direct owner-only transfer, rotation, revocation, rollback,
  dependency reassessment, and target-Mac evidence before real demo-token
  ingestion can be proposed. This is a planning gate, not credential or
  Cloudflare-action authority.
- D-071 requires a separate owner-approved selection between signed identity
  and narrow app-specific ACL; it also requires a distinct secret-memory model.
  Unsigned prompt behavior is not an approved fallback, and documentation
  creates no signing, Keychain, credential, or Cloudflare authority.
- D-072 selects stable signed macOS identity as the future model. It does not
  authorize signing assets or real credential handling; a later exact plan must
  define least-privilege scope, secret-memory ownership, lifecycle, and private
  target-Mac evidence.
- D-073 constrains a later fake-only signed-identity and secret-memory proof to
  three existing Rust paths. Any dependency, configuration, entitlement, IPC,
  networking, or runtime need requires new owner approval.
- D-064 closes the pre-implementation configuration and separates design,
  no-traffic provisioning, synthetic-only transport, and real-content
  activation. Its authoritative configuration is
  [`docs/security/phase4-gateway-configuration-spec.md`](docs/security/phase4-gateway-configuration-spec.md),
  and its threat actors, trust boundaries, abuse cases, controls, and required
  tests are in
  [`docs/security/phase4-gateway-threat-model.md`](docs/security/phase4-gateway-threat-model.md).
  No stage authorizes or starts the next stage.
- The closed registration uses separate Microsoft personal desktop and gateway
  API applications, `api://<gateway-api-client-id>`, delegated scope
  `gateway.access`, and a `127.0.0.1` ephemeral callback at
  `/oauth/callback`. Actual identifiers and observed claims remain restricted
  operational evidence; mismatch fails closed.
- The closed Azure boundary below is retained as historical D-064 design
  evidence only; D-066 supersedes it as the synthetic-demo provider direction.
  It uses one dedicated non-shared user-assigned managed
  identity, exact-resource `Cognitive Services OpenAI User` RBAC, a private
  Azure OpenAI endpoint, and disabled provider public network access before any
  provider traffic. API-key fallback, broad runtime roles, unrestricted egress,
  alternate origins, and automatic fallback are prohibited.
- Other providers remain separately approved future adapters. Automatic or
  silent provider fallback is prohibited because it can cross retention, region,
  and contractual boundaries.
- A future desktop gateway access token must be short-lived with a maximum 15-minute lifetime, audience-bound, read only by trusted Rust through the platform secret-store abstraction, and stored in process memory. Initial scopes are `openid`, `email`, and one exact delegated gateway scope. `profile`, Microsoft Graph, directory, group, mail, calendar, file, contact, and `offline_access` scopes are excluded. Persistent sessions and `offline_access` require a separate decision; any later approved refresh or session credential must use platform-secure credential storage. The WebView must never receive any credential.
- Microsoft's documented default access-token lifetime does not satisfy the
  accepted 15-minute maximum. Stage B must prove an enforceable compatible
  personal-account configuration or an additive decision must define another
  short-lived gateway-session boundary. The manifest-based `127.0.0.1`
  ephemeral callback also requires Stage B and target-Mac Stage C evidence. No
  silent lifetime or redirect fallback is permitted.
- The Rust core may send only a closed, size-bounded request contract to one configured HTTPS gateway origin. The WebView must not choose the gateway URL, identity provider, AI model provider, model, provider parameters, tool schemas, or authorization headers.
- The gateway authenticates and authorizes the desktop principal, applies request/rate/model/tool-set limits, selects an approved AI model provider under trusted policy, injects only that server-held provider credential, and normalizes upstream events. It cannot approve or execute local tools.
- The gateway selects exact server-side tool-set versions. It must not forward arbitrary caller-supplied OpenAI tools, hosted tools, MCP servers, shell tools, or provider parameters.
- OpenAI function definitions must use strict mode, require every property, reject additional properties, and disable parallel tool calls for the first production loop. Strict provider generation is defense in depth, not local authorization.
- The gateway must validate recognized upstream event types and required fields, enforce sequence and size limits, and reject unknown event types. Additive fields on a recognized OpenAI event may be ignored because the upstream API documents them as backward-compatible additions; no unrecognized field may cross the normalized protocol.
- The gateway-to-Rust protocol must be versioned, closed, size-bounded, sequence-checked, and reject unknown fields. Function calls remain untrusted until trusted Rust independently validates the call ID, registered tool name, tool-contract version, argument JSON, exact per-tool schema, risk, permission, policy, and approval state.
- Use foreground Responses streaming with `store: false` and `background: false`. Cancellation transitions the local validator to a terminal cancelled state, aborts the desktop-to-gateway request and gateway-to-provider stream, discards late events, and is idempotent. Do not expect a cancellation event over the aborted stream or claim provider-side cancellation completion.
- `store: false` minimizes Responses application-state storage but does not eliminate provider abuse-monitoring retention. D-061 requires verified provider-approved ZDR independently for each AI provider's exact production organization, project, endpoint, model, and region configuration before real user content. Until then, only synthetic data may be considered under a separately approved future transport test.
- Azure real-content approval additionally requires exact resource evidence for `ContentLogging=false` and documented confirmation that the selected stateless Responses configuration retains no application state. Hosted tools, files, retrieval, Agents, Assistants, Batch, stored completions, web search, MCP, code execution, and response retrieval remain excluded.
- After ZDR verification, the initial real-user data class is explicitly submitted, non-sensitive text only. Credentials, attachments, regulated data, financial or healthcare data, and sensitive personal data remain prohibited.
- Provider and gateway failures cross into Rust only as closed redacted error codes, retryability, bounded retry delay, and opaque correlation IDs. Raw response bodies, headers, stack traces, prompts, model output, function arguments, and credentials must not cross this boundary.
- The gateway operational log and the local trusted audit log are separate. Gateway logs contain only authentication outcome, opaque principal/correlation IDs, contract/model versions, timing, status, rate-limit metadata, and aggregate usage, with a maximum seven-day retention. Content logging is prohibited. The local audit owns model-proposal, policy, approval, execution, cancellation, and outcome evidence without duplicating prompts, raw arguments, raw results, or raw errors.
- External-processing disclosure must be presented before the first transmission and remain visible in Settings.
- Disclosure requires explicit versioned acknowledgement before the first
  external transmission, including a synthetic-only Stage C transmission, and
  after material provider, retention, data-classification, or disclosure
  changes. Exact copy and acknowledgement storage require separate Stage C
  approval and evidence.

### D-094 Personal Assistant v0 boundary

V0-1 is published at `dca584e`. V0-2's local volatile lifecycle boundary is
verified complete from baseline `8e382e8`; all external and persistent
boundaries remain blocked.

D-094 narrows the first usable capability to one volatile, foreground,
explicitly user-initiated Personal Assistant text request with bounded streaming
and one final answer. Its tool set is empty. It has no file, persistence,
memory, scheduling, background, delegation, retry/fallback, approval dispatch,
durable audit, or device-effect path.

The dependency-ordered security boundary is:

- an application-owned synthetic empty-tool request that pins complete
  instructions, fixture, OpenAI/`gpt-5.6-luna`, features, limits, and a
  data-class profile;
- one Rust-owned process-local session with private identities, an opaque
  presentation handle, deadlines, terminal cancellation, cleanup ownership,
  and late-event rejection;
- a signed-client/Keychain proof that remains blocked by D-076 and TS-017;
- a Cloudflare Worker that independently validates Access JWT signature,
  issuer, time claims, and exact audience before any body handling;
- a synthetic-only OpenAI stream with fixed admission, no tools, no retry or
  fallback, content-free application logs, a default-deny new-admission flag,
  and bounded owned abort at both hops;
- a three-command no-text synthetic-v1 start/poll/cancel Tauri boundary parsed
  from `unknown`, with Rust-bound disclosure admission and no authoritative
  WebView-forgeable event; and
- a distinct real-content-v2 admission only after D-061, explicit
  provider/hosting authority, and an approved non-demo authentication boundary
  pass.

The first external operation is V0-9's zero-body authentication probe, so it
has a separate exact terminal disclosure and one-use Rust admission before any
credential projection or socket. It cannot mint or consume the later model-
transport admission. Live configuration is staged: V0-5 owns
issuer/AUD/JWKS/origin, V0-8 owns expected Client ID, and V0-12 owns the
provider secret. Only exact `PA_V0_TRAFFIC_ENABLED=true` admits a new request;
false/missing denies but does not claim to abort an active request.

The Cloudflare service-token resource `id` used by a Service Auth `token_id`
selector is not the token `client_id` carried in JWT `common_name`, the Worker
binding, and Keychain. V0-8 tracks separate sanitized fingerprints tied to one
reviewed record and owns exact Dashboard/Keychain/pasteboard transfer and
rotation cleanup. The provider adapter accepts only a current, bounded,
contiguous Responses event grammar, keeps provider IDs private, rejects
refusals/reasoning/tools outside that grammar, and never assumes an
undocumented sequence-number origin.

For the synthetic lane, Access authentication-request logs and Worker/runtime
metadata are gateway logging and remain content-free with retention at most
seven days. D-094 separately accepts Cloudflare's current 18-month mandatory
admin-action audit retention only as control-plane metadata and only after a
pre-traffic inspection proves no content or secret fields. Any drift blocks
Stage C, and real prompts require a fresh V0-14 outcome.

The exact synthetic disclosure names current documented retention rather than
saying only “operational retention”: OpenAI abuse-monitoring content up to 30
days, encrypted prompt-cache state up to 24 hours, Cloudflare Free-plan Access
authentication metadata for 24 hours, and mandatory admin-action audit records
for 18 months. It states `store=false` is not ZDR. V0-12/V0-13 must reverify
these facts and version the disclosure on any change.

Current source implements only V0-1 and V0-2's transport-free prerequisites: a
sealed application-owned synthetic `empty@1` request, a distinct branch behind
the sole Native runtime start boundary, exact returned identity/initial-status
validation, and a no-input volatile session host. A rejected or ambiguously
cleaned run remains owned with its process lease in private fail-closed
quarantine. Predictable host-issued handles and failure correlations are local
correlation, not authentication. The host exposes closed bounded snapshots and
updates but no request/frame ingress. Deterministic success/failure/stream
fixtures cross the real runtime event acceptance boundary and the same
production-private reducer; they are not live-provider evidence.

The current initial gateway request still separately advertises two tools. No
direct Rust HTTPS client, Tauri/WebView Personal Assistant boundary, signed
identity, credential, Worker/provider path, network request, persistence,
execution, durable audit, or exact ZDR evidence exists. V0-2's success,
failure, and stream outcomes remain fixture-only; production can reach only
local start/poll/deadline/cancellation/restart projections until later approved
transport work.
The complete blocker and rollback record is
[`2026-08-28-personal-assistant-v0-program.md`](docs/plans/2026-08-28-personal-assistant-v0-program.md).

The transport-free V0-2 session-host increment is published through
[PR #81](https://github.com/SillyRbbit/ai-agent-assistant/pull/81): reviewed
head `7fecf03` squash-merged to `main` at `1513bd8` after all six required PR
checks passed. V0-3 remains Blocked by D-076/TS-017 and a separately accepted
restart of the signed-identity lane.
D-095's Xcode-managed Developer ID recovery decision remains documentation-only
and did not itself authorize external work. A later separately approved
execution created one Xcode-listed Developer ID Application certificate record,
but sanitized CLI checks found no usable signing identity. The owner later
confirmed categorically that Keychain Access shows the certificate with a
private key beneath it. After separate exact residual-risk acceptance and
approval, one bounded sanitized default-user-Keychain query returned exactly one
valid code-signing identity with the fixed Developer ID Application label
prefix. Local pairing and current scoped identity visibility are observed, but
the historical non-export criterion cannot Pass as written, its evidence
standard is now truthfully governed by D-096, and a signed build is Not run. The owner
reported no authorization prompt and no visible state change. No
credential or fake generic-password Keychain item was created or read, and no
entitlement, App Store publication, notarization, distribution, source,
configuration, dependency, product-network, or runtime capability changed. The
one-run approval is consumed; recovery remains stopped closed and V0-3 remains
Blocked.
The separately drafted
[Developer ID present-use and local signing proof](docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md)
records the security-evidence correction accepted in D-096: pairing, identity-list, and signature
evidence cannot retrospectively prove that the key was never exported or held
exclusively, and the separate unqueried current extractability attribute remains
`not_proven`. Prospective evidence may use bounded owner attestation of no known
private-key export/copy/backup/share action, record that the reviewed workflow
performs no private-key or standalone certificate-file export, and leave
historical absence of export, non-extractability, and exclusive custody
`not_proven`. Any present-use signing proof remains Blocked pending an exact
closed-output no-argument sanitizer, static review, and separate one-attempt
owner approval plus every other recorded gate. This documentation decision
grants no private-key use or operational
authority.

Post-execution review found that the consumed identity wrapper resolved its
account home through `pwd.getpwuid()` and therefore potentially
`opendirectoryd`. A full account record, configured local or remote directory
service, and OS-owned cache/socket/log state may have been involved even though
the wrapper used only the home field and emitted no account value. Remote
traffic is not proven. This boundary was not separately accepted before the run
and remains Pending; the wrapper must not be rerun. Future wrappers must disclose
or contain account-directory resolution. Separately, environment/cache routing
does not confine executable npm lifecycle, Cargo build, or Tauri/frontend child
effects; future signing remains Blocked until an exact reviewed no-new-
dependency control detects or contains outside-root writes, undeclared network
effects, and escaped children.

The owner-authorized D-098 repository-governance recovery uses synchronized
baseline
`a417e5f1c1c602b917ca27c65af71480e3db6a45`. Its proposed one-shot,
argument-free `record-failed-disposition` path may record only exact schema-v3
lineage from the published D-097 failure to the sole documentation successor
`personal-assistant-v0-signing-security-prerequisite-planning`. It must preserve
the original `failed` / `FAIL` / `Blocked` values and absence of a completion
marker. It carries, rather than clears, the historical screenshot/privacy
failure and the undisclosed `getpwuid`/`opendirectoryd` boundary. It cannot make
build containment, private-key use, operational signing, V0-3, or any broader
successor admissible. Its tracked report freezes before the transition and
cannot attest its later result; only the ignored schema-v3 state and redacted
`status` output may do so. No Apple, Xcode, Keychain, signing, credential,
provider, network, or product action is authorized.

The completed exact D-098 successor is documentation-only. It preserves the
historical screenshot/privacy failure as Failed and the
`getpwuid`/`opendirectoryd` boundary as Manual verification pending. It may
describe, but not operate, the separately approvable P1 evidence-privacy, P2
account-directory, P3 build-child-containment, and P4 immutable-signer
prerequisites. A passing documentation closeout is not evidence of Keychain
custody, signing, containment, Apple-system safety, or product authority.

D-100 defines the documentation-only P1 protocol as a closed
`evidence_privacy_v1` record: one fixed version, one future-plan-owned check ID,
and one check-owned categorical outcome. It has no optional or free-text data
channel. Sensitive evidence must be minimized at its approved local source;
capturing raw data and redacting it later is prohibited. Screenshots,
recordings, transcripts, raw output, logs, target-derived identifiers,
account/certificate metadata, fingerprints, target-derived labels, private
paths, credentials, and sensitive content cannot be evidence. Static
repository-owned protocol literals remain permitted. Tokens are exact bounded
lowercase ASCII, and any future serialization is one canonical bounded record.
The record has no standalone authority; future trusted state must bind it once
to one approved plan/check/attempt and reject duplicate, replayed, cross-plan,
pre-admission, or late data. Each category represents one bounded predicate and
cannot claim approval, safety, readiness, exclusivity, historical absence, or
broader verification. A boundary failure emits only the applicable
closed category, which every operational check must predeclare, stops without
retry, and preserves historical truth. D-100 is policy documentation, not an
implemented sanitizer, runtime, IPC, filesystem, logging, target-Mac, or
external-system boundary.

The D-100 payload prohibition applies to inspection-target or operation-derived
security/signing evidence. Necessary non-sensitive repository validation and
governance metadata that are not derived from the inspected security target are
separate from the three-field protocol record.

D-102 defines documentation-only future build-child containment as a required
pre-effect boundary, not a disposable output location or a post-execution
observation. A future application-owned opaque, non-serializable,
attempt-bound `ContainedBuildAttemptV1` may accept no caller/model/WebView/
environment-selected executable, arguments, working directory, graph, retry,
network policy, or cleanup target. It must freeze one reviewed transitive graph,
use shell-free fixed executable identity and literal arguments, close inherited
environment and descriptors, and constrain ambient host-data reads as well as
writes. Before an effect, it must deny outside-root writes and undeclared
network effects. Authoritative membership must survive fork, exec,
reparenting, `setsid`, and `setpgid`; process groups alone cannot prove it.
Terminal deadline/cancellation must own graph shutdown, direct-child reaping,
pipe closure, and race-free quiescence. Descriptor-bound non-following cleanup
must quarantine unresolved ownership and block retry/replacement. Each future
check uses D-100's fixed categorical record; raw output, command lines, paths,
host/process/account identifiers, traces, packets, and free text are
prohibited. No target-Mac primitive is selected, no operational evidence exists,
and P3/P4/signing remain Blocked.

D-101 defines the accepted documentation-only application-resolution policy.
Future trusted application code must not resolve an account, username, numeric
UID/eUID, login/session/console user, home or standard directory, current/
temporary/configuration directory, search list, or Keychain filesystem path;
read account/home environment values; use a caller-selected scope; invoke a
directory-service client; or fall back to a subprocess or path. The application
may own only an opaque no-input, process-private, non-serializable, attempt-
bound, operation-specific wrapper over an adapter-private platform-issued
reference. The wrapper grants no generic enumerate/read/write/delete/sign
authority. Cleanup is attempted on every terminal path; success destroys the
reference, while failure retains the wrapper/reference in private adapter-owned
quarantine until process exit and blocks replacement, retry, reuse, exposure,
or early ownership loss.

Independent authoritative static input and exact scope-provenance contracts
are mandatory and non-waivable under D-101. Provenance must bind one fixed
application credential domain and reject ambient current-user/login/default-
Keychain/default-search-list/current-directory/environment selection. Separate
one-predicate reviews must cover account-record/directory resolution, every
cache read/mutation, log emission, socket/IPC effect, trust evaluation/service,
process-metadata path, and network effect caused by the selected operation. The
application policy alone does not establish operational P2 containment. Source
review can prove only the absence of prohibited application paths, not the
absence of OS-internal effects. Ambiguous or unsupported contracts stay
unavailable. A separately approved future disposition may address only exact
OS-internal uncertainty; it cannot waive application resolution, input,
provenance, or fallback rules.

Any need for an unapproved dependency,
WebView or subprocess networking,
caller-selected trusted configuration, raw-content logging, persistent
acknowledgement, missing JWT/ZDR evidence, accepted late event, incomplete
cleanup, or unapproved external action fails closed and stops the program.

## Prohibited changes before an approved live-gateway increment

Do not add:

- Accessibility APIs.
- ScreenCaptureKit.
- Apple Events.
- Microphone permissions.
- Unrestricted shell execution.
- Generic filesystem access.
- Production model API credentials.
- Automatic external communication or destructive actions.

Any request to introduce one of these requires a new architecture decision and a later-phase threat-model review. Live AI provider networking, gateway authentication, identity federation, OAuth/OIDC, SAML, PKCE, token validation, platform-secure credentials, cloud deployment, `AgentProvider`, and a deployed gateway also remain prohibited until their own exact implementation plan is approved and all O-006 and D-061 evidence gates pass.

## Dependency review

Before adding a production dependency:

1. Explain the exact capability it provides.
2. Confirm the standard library or an existing dependency cannot satisfy the need.
3. Review maintenance, licensing, native build impact, and transitive dependencies.
4. Record the decision in `DECISIONS.md`.
5. Pin the version and update lockfiles.
6. Run the full relevant verification suite.

Apply the dependency and supply-chain sections of `SECURITY_CHECKLIST.md` and
`RELEASE_CHECKLIST.md` when those scopes apply.

## Data handling

- Keep local data local unless the user explicitly sends it.
- Minimize data before passing it across IPC or network boundaries.
- Redact content from logs and audit details unless the exact field is required for an approved action.
- Use opaque references rather than arbitrary paths where possible.
- Reject symlink escapes and unsupported executable content when file tooling is implemented.
- D-085 memory is process-local to one bounded orchestrator. Approved shared,
  private, task-temporary, and proposed-shared content has exact ownership,
  quotas, versioned review, deletion, terminal cleanup, disable, and drop
  behavior; it never enters SQLite or another workflow.
- The approved-document reader accepts only exact registered lowercase `.txt`
  or `.md` nonempty UTF-8 files within the documented bounds. It rejects
  traversal, noncanonical members, symlinks, hard-link aliases, unsupported or
  changed targets, replay, revocation, and cross-workflow references.
- On supported Unix targets, registered, opened-handle, and final-path identity
  are compared before and after the bounded read. The pure-standard-library open
  sequence retains a narrow TOCTOU residual advisory. Non-Unix targets report
  this boundary unavailable rather than weakening it.
- Raw document text and explicitly selected approved-shared context are labeled
  untrusted, bounded before runtime request construction, and excluded from
  Debug, errors, orchestration events, governance audit, and automatic memory.
- D-085 adds no IPC, file picker, provider transmission, unrestricted file tool,
  persistence, vector search, background indexing, document writing, or device
  effect. Any later consumer requires a separate approved plan and privacy
  evidence.
- D-086 transports specialist data only through validated structured results;
  it never reads sibling memory. Research and Knowledge retain access only to
  their own live agent-private and task-temporary namespaces, and terminal
  cleanup removes task-temporary records.
- A reusable Knowledge value remains `PendingReview`. It is not inserted into
  `MemoryStore`, assigned a shared-proposal identity, approved, selected,
  persisted, or treated as fact by synthesis.
- D-086 fixture labels/evidence, objective text, findings, summaries, proposals,
  runtime output, arbitrary references, paths, URLs, and reasoning remain absent
  from Debug, errors, workflow events, workflow audit, logs, SQLite, IPC, and
  automatic memory. Its fixture-only disclosure does not prove live retrieval
  or factual correctness.
- D-086 adds no provider, model, credential, network, process, filesystem read,
  tool, executor, persistence, IPC, UI, dependency, capability, permission,
  Hermes/OpenClaw adapter, or `AgentRuntime`/`NativeAgentRuntime` widening.
- D-087 fixture content, free-form output, patch descriptions, criterion text,
  evidence descriptions, and fake-secret sentinels remain absent from Debug,
  errors, workflow events/audit, logs, SQLite, IPC, and automatic memory.
- D-087 adds no live repository/filesystem/process/Git/package/network access,
  registered tool, executor, approval request, mutation, persistence, IPC/UI,
  dependency, capability, permission, provider, external runtime, or
  `AgentRuntime`/`NativeAgentRuntime` widening. Native remains sole/default.
- D-088 fixture content, sanitized log text, assessment/change/diagnostic plan,
  criterion/evidence descriptions, and synthetic sentinels remain absent from
  Debug, errors, events/audit, logs, SQLite, IPC, and automatic memory.
- D-088 adds no Terraform/platform/OS command, live inventory or diagnostic,
  credential lookup/use, tool, executor, approval request/dispatch, provider,
  IPC/UI, dependency, permission, persistence, external runtime, or effect and
  does not widen `AgentRuntime` or `NativeAgentRuntime`.

## GitHub automation boundary

- Pull-request code and dependency updates are untrusted. GitHub workflows run
  with `contents: read`, receive no repository secret context, do not use
  `pull_request_target`, and disable persisted checkout credentials.
- Active persistent runners receive only reviewed branch pushes, scheduled
  audit, and explicit dispatch. They never subscribe to `pull_request` or
  `pull_request_target`; fork and dependency-bot changes must be reproduced on
  a maintainer-controlled allowlisted branch before execution.
- Linux or target-Mac Rust verification does not prove native macOS menus,
  windows, dialogs, icons, permissions, signing, notarization, or installer
  behavior. Those checks remain target-Mac evidence.
- Every external action reference is pinned to an immutable commit digest.
  Dependabot proposes action updates for review; mutable action tags are not a
  trust decision.
- Workflows may inspect, compile, test, and retrieve public advisory data. They
  must not commit, push, merge, publish, deploy, sign, notarize, or begin another
  increment.
- The CI dependency-audit job runs the pinned JavaScript audit and Cargo audit
  for dependency, security-sensitive, workflow, scheduled, and manually
  dispatched validation. The
  Cargo result fails on any finding outside D-025's exact two-vulnerability
  `quick-xml 0.39.4` baseline and D-046's exact 18-warning lockfile baseline.
  Accepted findings remain unresolved and visible; the gate does not declare
  them fixed or generally safe.
- The tracked secret-pattern scan is defense in depth, not proof that a
  repository or artifact contains no secret. Release review still requires a
  complete secret and artifact assessment.
- CODEOWNERS, issue labels, milestones, badges, and workflow success are
  repository coordination evidence, not authorization, branch-protection proof,
  security approval, or release evidence.
- Security design-review issues must contain sanitized material only.
  Vulnerabilities and incidents follow the private reporting policy.

The authoritative runner labels, host prerequisites, trust policy, maintenance,
incident response, and rollback are in
`docs/github/SELF_HOSTED_RUNNER.md`.

## Repository hook boundary

- Repository-local hooks execute code with the developer's Codex session permissions and require review and trust through `/hooks` before use.
- `.codex/hooks/common.py` provides bounded repository, path, JSON, conflict,
  and suspicious-path validation to the two hook scripts.
- `.codex/hooks/post_increment_gate.py` may read repository metadata, changed
  files, its bounded structured report, and ignored gate state.
- `.codex/hooks/session_end_gate.py` may read only fixed Git status evidence and
  emit a structured staged, unstaged, untracked, and conflicted-path inventory.
  It is read-only and exits nonzero when conflicts exist.
- The hook scripts must not parse the unstable transcript, inspect model or
  personal content, access the network, execute arbitrary report content, or
  modify product source.
- Hook input is untrusted JSON. Validate event type, booleans, repository root, bounded size, report schema, and every path before use. Reject absolute paths, traversal, symlink escapes, merge conflicts, stale fingerprints, and suspicious changed paths. The workspace fingerprint covers only paths that exist in the current snapshot; reviewed deletions remain mandatory report-inventory entries but contribute no content before or after commit. Removing a path that existed at finalization must invalidate the marker.
- The script may invoke only fixed Git inspection commands. It must not use report content to construct shell commands.
- `.codex/state/post_increment_gate.json` contains no secrets and is ignored.
  Passing state may contain a completion marker. Terminal failed state contains
  exact `FAIL`, readiness, report/hash, HEAD, and workspace-fingerprint evidence
  but no completion marker and cannot be promoted to complete. A valid failed
  state lets the Stop hook end; in the checkout retaining the ignored state,
  Blocked readiness denies a successor, and a non-Blocked successor still
  requires a valid state, a clean workspace, and separate owner approval.
- The ignored state is checkout-local governance evidence, not authentication
  or a durable audit boundary. Its hashes detect ordinary unreclosed report or
  workspace drift; a same-user process able to rewrite the state and repository
  can forge those values. A fresh clone does not inherit the ignored state.
  Neither passing nor failed state proves commands ran or grants approval,
  authorization, signing, publication, or product authority; the report and
  actual command output remain the evidence.
- D-098's in-progress recovery may extend only the exact published D-097 failed
  record with bounded one-shot lineage to the named documentation successor.
  Until the argument-free record command and every required check actually
  pass, the existing schema-v2 record remains authoritative. Even after a valid
  disposition, the original `FAIL`/Blocked result, historical blockers, and
  same-user/fresh-clone limitations remain; the lineage is not authentication,
  approval, completion, operational-signing admission, or durable audit.
- `stop_hook_active` must suppress a repeated continuation request. This loop guard does not waive the mandatory completion criteria.
- In an emergency, disable the hook through `/hooks` or start a session with `codex --disable hooks`. Record the bypass and rerun the complete gate before marking an increment complete. Do not routinely bypass hook trust.

## Reporting a security concern

Do not place secrets, personal files, tokens, or exploitable details in a public issue. Record a sanitized summary in the project handoff and notify the repository owner through a private channel.

A security fix is not complete until regression tests and the relevant threat-model documentation are updated.

Production release additionally requires every applicable item in
`RELEASE_CHECKLIST.md`, including signing, notarization, installer, secret,
artifact, and rollback evidence.
