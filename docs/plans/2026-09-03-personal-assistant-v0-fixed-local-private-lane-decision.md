# Personal Assistant V0 fixed local private-lane decision

Status: Complete (`PASS WITH ADVISORIES`) — documentation-only; no product authority
Readiness: Blocked for every operational successor
Owner: Project owner
Last updated: 2026-09-03
Planning baseline: `51be9ba91ba69c9c96dfb3bbfd4509d3177e902a`
Branch: `codex/personal-assistant-v0-fixed-local-private-lane-decision`
Gate ID: `pa-v0-fixed-local-private-lane-decision`
Decision: D-120; `fixed_local_v2_planning_selected`

## Goal

Make one closed documentation decision about whether D-094's reserved, fixed,
nonselectable `real-content-v2` contract may have an additive local/no-auth
private-use planning lane whose evidence work can proceed before the blocked
remote synthetic-v1 lane completes.

The decision is about program topology and dependency order only. It does not
select, admit, install, load, benchmark, or execute a local engine or model. It
does not create a D-119 profile, change current source, or make any operational
successor Ready.

The narrow proposed direction is additive:

- synthetic-v1 remains the fixed OpenAI-through-Cloudflare proof selected by
  D-094 and remains separate and Blocked;
- `real-content-v2` remains fixed and nonselectable, but a later plan may assess
  one application-owned local/no-auth topology without waiting for V0-13;
- D-119's distinct post-v0 selectable catalog remains unchanged, with all ten
  candidates `candidate_blocked` and no selection handle; and
- every engine, artifact, dependency, filesystem, containment, transport,
  Tauri, UI, and target-Mac boundary remains subject to a later separately
  approved decision and plan.

## User-visible outcome

None. This is a documentation-only architecture and sequencing decision. It
creates no local model, provider choice, connection-profile selector, text
entry, stream, Tauri route, credential path, file access, network path, process,
or device effect.

The owner accepted the positive disposition after repository evidence remained
consistent. The only outcome is that a separately approved documentation plan
may evaluate one fixed local engine, model, artifact, dependency, and
containment boundary for private `real-content-v2`. No such successor exists or
starts in this increment, and the profile remains unavailable until all later
evidence and implementation gates pass.

## Readiness decision

`Ready` applies only to the bounded documentation decision described here.
Operational readiness remains `Blocked`:

- current source has no local inference engine, model artifact, local provider
  adapter, Personal Assistant Tauri route, or production response ingress;
- `src-tauri/Cargo.toml` has no direct local-inference or direct HTTP/TLS client;
- D-118 remains `no_eligible_client` for its exact frozen HTTPS candidates;
- D-119 admits no profile and exposes no selection handle;
- V0-3 and V0-7 remain Blocked for the remote synthetic lane;
- no exact engine/model, tokenizer, prompt template, artifact format, checksum,
  provenance, license, dependency graph, storage, update/removal, no-egress,
  resource, containment, or target-Mac decision exists; and
- existing Personal Assistant fixture tests do not prove local inference,
  engine cancellation, cleanup, or late-result behavior.

The owner-approved decision increment selected
`fixed_local_v2_planning_selected` after repository evidence remained
consistent. D-120 records that closed disposition as documentation planning
authority only; it does not make an operational successor Ready.

## Current-state evidence

- Before this file was added, `HEAD`, local `main`, and the locally recorded
  `origin/main` all resolved to
  `51be9ba91ba69c9c96dfb3bbfd4509d3177e902a`; ahead/behind was `0/0`, and the
  worktree had no tracked or untracked changes. No fetch or external-system
  access was performed for this planning task.
- D-094 fixes synthetic-v1 to the OpenAI-through-Cloudflare synthetic proof and
  leaves the distinct `real-content-v2` topology unselected. The first usable
  v0 is the private real-content capability, not the synthetic proof.
- The V0 program's existing local-alternative impact ledger identifies an
  additive local milestone-2 lane as the narrow option: it would amend only the
  real-content topology/order while preserving the remote synthetic lane and
  all historical evidence.
- D-118 accepts `no_eligible_client` for five exact hostname-based Rust HTTPS
  variants under the current hard cancellation and cleanup contract. It is not
  a universal claim that every transport or a truly offline local topology is
  impossible.
- D-119 reserves only a distinct post-v0
  `personal-assistant-selectable-connection-profile-v3` direction. Its catalog
  has exactly ten entries, all `candidate_blocked`, and no handle.
- `src-tauri/src/personal_assistant_v0.rs` remains a volatile, transport-free
  Rust host. It owns identity, bounds, lifecycle, cancellation, quarantine, and
  presentation state, but its response-event ingestion is fixture-only and does
  not prove a local engine boundary.
- `src-tauri/src/lib.rs` exposes no Personal Assistant Tauri command or event.
- Baseline `npm run docs:check`, `npm run repository:check`,
  `npm run security:scan`, and `git diff --check` passed before this plan was
  added.
- At the clean baseline, the published
  `personal-assistant-v0-pr110-publication-closeout` predecessor was recorded
  complete with `PASS WITH ADVISORIES` and a valid marker. After this one
  untracked plan changed the workspace fingerprint, read-only gate status
  reported the same predecessor `complete` and `valid: false`. The only
  workspace delta is this plan. Do not finalize, rebind, retarget, or otherwise
  mutate that historical predecessor. Before a separately approved `begin`,
  re-read gate status and stop if another increment is active or failed, or if
  predecessor invalidity has any cause other than this exact plan fingerprint.
- The D-120 slot and the proposed plan, increment, and review paths were unused
  when this plan was drafted. A collision or changed controlling decision is a
  stop condition.

## Scope

### This planning task

Create exactly this one file:

1. `docs/plans/2026-09-03-personal-assistant-v0-fixed-local-private-lane-decision.md`

Do not create a branch or begin a gate during this planning task.

### Separately approved decision increment

If the owner approves this exact plan, the documentation-only decision
increment may change exactly these sixteen documentation paths:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PRODUCT_REQUIREMENTS.md`
8. `PROJECT_STATUS.md`
9. `ROADMAP.md`
10. `SECURITY.md`
11. `SECURITY_CHECKLIST.md`
12. `TESTING_GUIDE.md`
13. `docs/PROJECT_DIRECTION.md`
14. `docs/plans/2026-09-03-personal-assistant-v0-fixed-local-private-lane-decision.md`
15. `docs/increments/pa-v0-fixed-local-private-lane-decision.md`
16. `docs/reviews/2026-09-03-pa-v0-fixed-local-private-lane-decision-post-increment-review.md`

`TROUBLESHOOTING_LOG.md` is excluded unless a separately approved amendment is
required for a newly observed troubleshooting fact. Any other changed or new
path is scope drift and stops the increment.

## Explicit non-goals

Neither this plan nor the proposed documentation decision may:

- change Rust, TypeScript, tests, dependencies, manifests, lockfiles,
  toolchains, workflows, hooks, Tauri configuration, capabilities, CSP,
  permissions, entitlements, signing, or product configuration;
- select or evaluate a vendor, engine, model, tokenizer, template,
  quantization, artifact, dependency, runtime, server, helper process, IPC
  protocol, local socket, or filesystem location;
- install, download, copy, load, parse, benchmark, execute, or remove a model,
  engine, executable, library, or artifact;
- access credentials, Keychain, certificates, private keys, Apple/Xcode,
  signing systems, providers, gateways, cloud resources, accounts, billing,
  product systems, or operational external systems;
- open DNS, HTTP, HTTPS, TLS, localhost, Unix-socket, provider, telemetry,
  update, license-check, remote-embedding, or other network traffic;
- admit `local_no_auth` or any other D-119 entry, create a selector or opaque
  profile handle, or change catalog membership or availability;
- make V0-3, V0-7, V0-13, V0-14, a local engine plan, or any product source
  increment Ready;
- remove, replace, implement, test, or transmit through the remote synthetic
  lane;
- weaken hard deadlines, cancellation, cleanup ownership, quarantine, late-
  result rejection, no-fallback, empty-tool, volatility, or no-device-effect
  requirements;
- accept or close any D-107 blocker or promote D-113 through D-117 from their
  Proposed/non-controlling state; or
- commit, push, merge, release, publish, deploy, or begin a later increment
  without the distinct authority required for that action.

## Decision boundary

The decision concerns only whether a future fixed `real-content-v2` may be
planned as local/no-auth before completion of synthetic-v1. It must preserve
these separations:

- **Fixed v2 is not selectable v3.** A local `real-content-v2` topology is one
  application-owned profile selected by an accepted decision. It is not a
  D-119 catalog entry, user-selected provider, profile handle, or catalog proof.
- **Provider no-auth is not user identity.** Local/no-auth means no model-
  provider authentication or external inference connection. It does not
  silently decide Cortexa owner authentication, OS-user trust, or multi-user
  access. D-062 and D-094's approved non-demo Cortexa owner-authentication gate
  remains independently mandatory and Blocked; local/no-auth does not satisfy,
  defer, or waive it.
- **Local processing is not automatically private.** Personal text can still
  leak through logs, caches, crash reports, swap, temporary files, model
  telemetry, update checks, remote embeddings, or hidden network behavior.
- **Local termination is not automatic.** Returning a timeout, dropping a
  future, closing the UI, or rejecting a result does not prove an in-process
  engine, native library, GPU queue, helper, or thread stopped.
- **Planning is not admission.** A positive disposition authorizes only the
  next documentation evidence plan. It does not authorize an engine, artifact,
  filesystem boundary, source change, or target-Mac operation.

## Required decision reconciliation

The decision increment must reconcile these accepted constraints additively;
it may not rewrite dated evidence.

### D-094 — fixed V0 contracts and milestone order

- Preserve synthetic-v1 exactly as the fixed OpenAI-through-Cloudflare proof.
- Preserve `real-content-v2` as fixed, distinct, and nonselectable.
- A positive decision may supersede only the blanket rule that every local-v2
  documentation evidence activity must wait for V0-13. It may permit one new
  local engine/artifact evidence plan to precede V0-13. The existing dated
  V0-14 plan remains historical and Blocked with its recorded V0-13 dependency;
  it is neither started nor reclassified by D-120.
- The decision may not claim that local evidence satisfies the remote proof or
  that remote evidence satisfies local processing.
- The existing no-files, no-process, no-device-effect boundary remains in
  force. A later artifact or containment decision must name and justify any
  narrowly required change before source work.

### D-060 — identity, hosting, provider, and credential separation

- Preserve gateway custody for every cloud/provider credential and every
  credential-bearing OAuth result.
- A local/no-auth planning lane creates no provider credential, direct/native
  custody exception, gateway credential, or provider authorization.
- D-060's remote identity, hosting, and provider directions remain intact. The
  decision may only record that a future proven fully offline local-v2 model
  path does not use those external provider/hosting boundaries. D-062 and
  D-094's separate Cortexa owner-authentication gate remains mandatory and
  Blocked.
- A future cloud profile still requires its own exact D-060 reconciliation and
  cannot inherit authority from local-v2.

### D-061 — external processing and local data handling

- Preserve exact provider-approved ZDR, retention, data-use, region, logging,
  deletion, disclosure, and freshness requirements for every external
  provider.
- Provider ZDR may be classified as inapplicable to local-v2 only after the
  complete future engine, model/artifact, dependency, acquisition/update,
  telemetry, crash, cache/support, and runtime lifecycle proves both no external
  provider processing and no network egress of prompt, output, derived content,
  or identifying metadata. This plan and decision do not supply that proof.
- Local-v2 still requires an exact personal-text data class, visible local-
  processing disclosure, bounded volatile prompt/output handling, content-free
  logs, cache/temp/crash-report policy, deletion/removal behavior, incident
  response, and target-Mac evidence.
- Missing or ambiguous no-egress evidence keeps the local lane Blocked; it never
  degrades into synthetic data, a cloud provider, or a less restrictive policy.

### D-118 — no eligible frozen HTTPS client

- Preserve D-118 and its `no_eligible_client` result byte-for-byte except for
  additive current-state references outside the historical decision text.
- A local lane can be independent of D-118 only when its eventual inference
  path opens no hostname, DNS, HTTP, HTTPS, TLS, proxy, local server, or other
  network edge.
- A localhost service, provider SDK, WebView request, helper transport,
  transitive client, shell command, or hidden updater is not a D-118 bypass. It
  stops this lane and requires a separately approved transport/containment
  decision.
- D-118's accepted negative result continues to block the current synthetic-v1
  HTTPS design and all nine network-backed D-119 candidates under the current
  architecture and hard-cancellation contract. A materially different
  transport or constraint requires its own separately accepted successor; this
  plan makes no universal Rust-HTTPS claim.

### D-119 — post-v0 selectable catalog

Preserve exactly these ten catalog-schema V1 candidates, each
`candidate_blocked`, with no selection handle:

1. `local_no_auth`
2. `google_gemini_oauth`
3. `google_gemini_api_key`
4. `direct_openai_api_key`
5. `direct_openai_workload_identity`
6. `azure_openai_entra`
7. `azure_openai_api_key`
8. `anthropic_api_key`
9. `mistral_api_key`
10. `aws_bedrock_identity`

A fixed local-v2 planning lane may share only the local/no-auth security
property; it may not reuse the D-119 catalog entry as admission, change the
entry to selectable, issue its future handle, or widen v3. Direct OpenAI and
Azure OpenAI remain separate, and ChatGPT login/subscription remains
non-authorizing for OpenAI API use.

### Other controlling decisions and evidence

- D-032's removed synchronous `complete(prompt) -> String` shape remains
  prohibited. Any future local engine must produce bounded incremental events
  through an application-owned adapter and reducer.
- D-065's immutable instruction hierarchy and empty tool set remain mandatory.
- D-078's private personal-project scope remains controlling without implying
  reduced security for personal content.
- D-079 through D-081 preserve `NativeAgentRuntime` as sole/default and keep
  Hermes/OpenClaw dispositions unchanged. A local model engine is not a runtime
  selector, and any new runtime, helper, server, or WebSocket requires a
  separate decision.
- D-096 through D-118 and their historical reports remain intact. D-107 remains
  eight documented of eleven, D-108 remains additively nine of ten, and D-113
  through D-117 remain Proposed and non-controlling.
- Preserve all ten D-107 blockers without claiming they are intrinsic to every
  local model operation:
  `opaque_prebound_identity_contract`, `exact_signer_binding_contract`,
  `account_keychain_scope_contract`, `private_key_nonexport_contract`,
  `fixed_algorithm_contract`, `interaction_denial_contract`,
  `hard_deadline_cancellation_contract`, `late_result_rejection_contract`,
  `cleanup_quarantine_contract`, and `platform_effect_contract`.
- V0-3 and V0-7 remain Blocked and unwaived for the remote lane. A local-v2
  plan may not represent their fake credentials, transport fixtures, signing,
  or D-107 evidence as local proof.

## Future local boundary requirements

These are acceptance requirements for later plans, not current evidence or
implementation authority.

### Closed Rust ownership and interface

The smallest prospective data flow is conceptually:

```text
explicit bounded owner text intent
  -> future versioned narrow Tauri request
  -> Rust-owned fixed local-v2 session/profile snapshot
  -> separately selected local inference adapter
  -> bounded Rust event validator/reducer
  -> future bounded presentation DTO
```

The WebView may eventually provide only the exact versioned bounded text and a
separate current disclosure acknowledgment. It may not provide a model path,
engine, tokenizer, template, runtime, provider, endpoint, profile, artifact,
thread count, device choice, prompt wrapper, limit, retry, or fallback.

Trusted Rust must issue run, attempt, request, presentation, and support
correlation; bind immutable instructions, one fixed local profile, empty tools,
limits, deadlines, lifecycle policy, artifact identity, and kill-switch state;
and validate every untrusted local-engine event before retention or projection.

### No-egress requirements

Before any local engine is eligible, a later plan must prove for its exact
version and complete feature/dependency graph:

- no DNS, network, proxy, HTTP/TLS, localhost, Unix-socket-to-server, provider
  SDK, telemetry, update, license-check, remote-embedding, analytics, crash-
  upload, or cloud-fallback path;
- no environment, user, WebView, model metadata, artifact metadata, plugin,
  dynamic configuration, or ambient service can enable egress;
- an in-process engine and every native dependency run inside and become part
  of the trusted computing base, but gain no policy, configuration, identity,
  or authorization authority. The exact source and feature graph must exclude
  network access and Keychain, environment-secret, account-state, default-
  credential-chain, gateway-token, provider-credential, certificate, private-
  key, signing-material, and unrelated secret-memory reachability during local
  inference. If either network or secret reachability cannot be excluded, the
  candidate remains Blocked pending a separately accepted isolation or
  containment architecture;
- no model or engine is downloaded, updated, replaced, or repaired at runtime;
  and
- target-Mac tests observe no unexpected network, permission, cache, log,
  temporary-file, or device effect, without treating observation alone as a
  universal source-level proof.

An in-process engine is the narrowest architecture worth evaluating first, but
this decision does not select it. A helper process, external executable,
localhost service, WebSocket, plugin, JIT/dynamic-code path, or separate runtime
requires a new containment and architecture decision before it can enter an
engine comparison.

### Model and artifact requirements

A later evidence plan must freeze one exact engine, model, quantization,
tokenizer, chat/prompt template, artifact format, version, cryptographic digest,
source/provenance, license, redistribution/use rights, size, parser, supported
hardware, and compatibility policy. Model files and metadata are untrusted
input, never authority.

It must define application-owned acquisition, verification, storage, file
permissions, corruption/mismatch handling, update, rollback, removal, orphan
cleanup, cache/temp behavior, and a no-arbitrary-path rule before requesting
any filesystem authority. No network acquisition, repair, update, or
replacement is authorized. Unless a later separately approved external and
supply-chain boundary says otherwise, evaluation is limited to one exact
pre-provisioned or bundled artifact. Missing, corrupt, substituted,
incompatible, unlicensed, or drifted artifacts leave the profile unavailable
with a closed redacted error; rollback and removal trigger no download, repair,
network access, or cloud fallback.

### Lifecycle, cancellation, cleanup, and late results

Every later local implementation must preserve:

1. one explicit foreground owner action and at most one process-wide request;
2. one immutable Rust-owned run/profile/artifact snapshot and zero retry or
   fallback;
3. the existing input, event, delta, output, queue, count, and deadline ceilings
   unless a separately accepted decision narrows them further;
4. monotonic `Idle -> Starting -> Streaming -> Completed | Failed` and
   `Starting | Streaming -> Cancelling -> Cancelled | Failed` transitions;
5. cancellation that closes result ingress at one serialized linearization
   point, signals the original work only, and actually stops or boundedly joins
   every engine thread, native callback, GPU/accelerator queue, buffer owner,
   mapping, and support task;
6. no claim that a timer, dropped future, UI transition, or late-result filter
   proves work stopped;
7. bounded cleanup owned by trusted Rust through quiescence; ambiguity retains
   private Rust ownership and denies retry, replacement, fallback, and restart
   until quiescence is positively proved. If proof never arrives, ownership
   remains until process termination. No deadline, UI terminal state, dropped
   future, or late-result filter releases it;
8. rejection of malformed, oversized, duplicate, foreign, wrong-generation,
   wrong-sequence, post-cancel, post-timeout, post-failure, and post-success
   results before state, UI, IPC, logs, evidence, persistence, retry, or follow-
   on work; and
9. closed bounded redacted errors with no prompt, output, model path, artifact
   detail, native error, stack, or device/account identifier crossing the
   boundary.

Late-result rejection remains distinct from cancellation and cleanup. Passing
one cannot substitute for either of the others.

## Threat model

- **Milestone laundering:** local planning is described as completing or
  replacing the synthetic remote proof without an accepted successor decision.
- **Catalog laundering:** fixed local-v2 is treated as admission of D-119's
  `local_no_auth` catalog entry or as selector authority.
- **D-118 evasion:** localhost, a helper, WebView fetch, provider SDK,
  transitive/private client, plugin, shell, or updater becomes undeclared
  transport.
- **Hidden egress:** inference, artifact loading, license checks, telemetry,
  crash handling, update checks, embeddings, or fallback contacts a network.
- **Artifact substitution:** an arbitrary, corrupt, drifted, unlicensed, or
  attacker-controlled model/tokenizer/template is loaded as trusted policy.
- **Caller-selected authority:** the UI, user text, model metadata,
  environment, filesystem, or runtime chooses the model, engine, artifact,
  limits, device, profile, instructions, or fallback.
- **In-process privilege collapse:** unsafe/native inference code shares the
  desktop process's memory, credentials, permissions, and crash boundary without
  an accepted dependency and containment review.
- **Unbounded resource use:** load, prefill, generation, native callback, GPU
  work, allocation, or teardown exceeds memory, time, thermal, queue, or output
  limits.
- **Cancellation laundering:** the UI reaches `Cancelled` while engine, thread,
  native, accelerator, or cleanup work survives.
- **Late-result mutation:** a stale or foreign event changes terminal state,
  exposes personal text, writes a log/cache, or starts follow-on work.
- **Local persistence leakage:** prompts or outputs enter logs, caches, temp
  files, crash reports, swap-backed buffers, screenshots, SQLite, or audit.
- **Fallback drift:** missing/corrupt local state silently starts a cloud
  profile, download, repair, retry, or different model.
- **Fixture transference:** existing deterministic host/runtime/demo tests are
  represented as local-engine or target-Mac evidence.

## Closed decision dispositions

The separately approved documentation increment must select exactly one:

| Disposition                              | Meaning                                                                                                                                                                                                                 |
| ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fixed_local_v2_planning_selected`       | Reserve an additive fixed/nonselectable local/no-auth planning lane for `real-content-v2`; allow a later documentation engine/artifact evidence plan before V0-13; grant no profile admission or operational authority. |
| `current_remote_first_sequence_retained` | Preserve the current V0-13-before-V0-14 sequence; keep local/no-auth only in D-119's blocked post-v0 catalog; select no local-v2 planning successor.                                                                    |
| `evidence_boundary_failed`               | Required repository evidence is missing, contradictory, drifted, colliding, or cannot reconcile the decisions without widening scope; accept no topology or ordering change and authorize no successor.                 |

There is no partial, provisional, implementation-ready, or default-open
outcome. Missing or ambiguous evidence selects `evidence_boundary_failed`.

## Proposed D-120 wording

If and only if the owner accepts `fixed_local_v2_planning_selected`, the
decision increment may add a D-120 record with this substantive result:

> Reserve an additive planning lane in which D-094's fixed, nonselectable
> `real-content-v2` may later use one application-owned local/no-auth topology,
> and allow its separately approved engine/artifact evidence planning to
> precede completion of the blocked remote synthetic-v1 lane. Synthetic-v1,
> D-066 through D-068, V0-3, V0-7, V0-13, and D-118 remain unchanged and
> Blocked for their remote path. The existing V0-14 plan remains historical and
> Blocked with its recorded V0-13 dependency; D-120 only permits a new local-v2
> evidence plan to be proposed earlier. The local lane neither completes nor
> replaces the remote proof. D-119 remains a distinct post-v0 selectable-v3 direction with
> exactly ten `candidate_blocked` entries and no handle; fixed local-v2 is not
> catalog admission. D-060 credential custody and D-061 external-processing
> evidence remain controlling for every cloud provider. D-062 and D-094's
> separate non-demo Cortexa owner-authentication gate remains mandatory and
> Blocked. Provider ZDR becomes inapplicable to local-v2 only after the complete
> later engine, model/artifact, dependency, acquisition/update, telemetry,
> crash, cache/support, and runtime lifecycle proves no external provider
> processing and no network egress of prompt, output, derived content, or
> identifying metadata; local personal-data handling,
> disclosure, logs, cache/temp/crash policy, deletion, lifecycle, cancellation,
> cleanup/quarantine, late-result rejection, target-Mac, empty-tool,
> no-fallback, and no-device-effect requirements remain. This decision selects
> no engine, model, artifact, dependency, filesystem boundary, runtime,
> transport, Tauri/UI contract, credential, provider, or source implementation
> and grants documentation planning authority only.

The quoted substantive result remains exactly owner-approved. Its controlling
security interpretation is fail closed: “no network egress” means no DNS,
socket, or network egress whatsoever, including prompt, output, derived
content, identifying/model/artifact metadata, telemetry, licensing, updates,
crash/support traffic, embeddings, and fallback. An in-process engine becomes
part of the trusted computing base but gains no policy, configuration,
identity, or authorization authority. Cleanup ambiguity never expires merely
because time passed; private Rust ownership remains until quiescence is
positively proved or the owning process terminates.

The final D-120 record must cite the exact accepted plan, baseline, conflicts,
selected disposition, and owner acceptance. A material wording change requires
owner approval rather than an inferred weaker result. If D-120 is occupied
before the decision begins, stop and amend the plan.

### Supersedes or is superseded by

D-120 additively amends D-094 only for the ordering of one separately approved
local-v2 documentation evidence plan. It additively clarifies D-060 and D-061
only for a future path that proves the complete zero-external-processing and
zero-DNS/socket/network-egress condition above. It does not
supersede D-060 or D-061 for any external/cloud path, D-062 or D-094's Cortexa
owner-authentication gate, D-094 synthetic-v1, D-066 through D-068, D-118,
D-119, or the existing V0-14 plan.

## Implementation milestones for the documentation decision

- [x] Confirm the clean synchronized planning baseline, unused paths and D-120
      slot, current source absence, and required documentation validation.
- [x] Draft this exact one-file Ready ExecPlan without a branch or gate.
- [x] Obtain explicit owner approval for this exact plan and one closed
      disposition decision increment.
- [x] Reconfirm clean synchronized `main`, decision/path availability, and all
      controlling decision text before editing. Inspect ignored gate status,
      preserve the completed predecessor, and stop on an active/failed gate or
      unexplained marker invalidity.
- [x] Create the approved `codex/` branch and run the exact `begin` command only
      after separate authorization.
- [x] Select exactly one closed disposition from repository evidence. Add D-120
      only if the owner accepts the positive disposition.
- [x] Reconcile the fifteen pre-report paths within the authorized sixteen-file
      scope while
      preserving all historical artifacts and Blocked boundaries.
- [x] Run documentation, repository, security, exact-scope, preservation,
      independent architecture/security/readiness, session, quality, report,
      and post-increment gates.
- [x] Stop for owner review without committing, pushing, merging, selecting an
      engine/model, or beginning a successor.

## Test plan

### Documentation-decision tests

- Confirm the selected disposition is exactly one of the three closed values.
- Confirm any positive decision changes only local-v2 planning order and does
  not claim local capability, profile admission, or engine eligibility.
- Confirm D-094's synthetic-v1 profile and the complete remote milestone remain
  unchanged and Blocked.
- Confirm D-118 remains `no_eligible_client` for its five frozen variants and
  blocks the current synthetic/network-backed catalog paths unless a separately
  approved architecture or hard-constraint reconsideration is accepted.
- Confirm D-119 still lists exactly the ten unique names above, every entry is
  `candidate_blocked`, and no handle or selector exists.
- Confirm fixed local-v2 is not represented as D-119 catalog admission.
- Confirm D-060 cloud credential custody and D-061 external-provider evidence
  remain controlling, while local ZDR inapplicability remains conditional on
  later proof of no external processing and no content or identifying-metadata
  egress across the complete lifecycle.
- Confirm D-062 and D-094's Cortexa owner-authentication gate remains mandatory
  and Blocked for local-v2.
- Confirm V0-3/V0-7/V0-14, all ten D-107 blockers, D-108's additive
  interpretation, and D-113 through D-117 status remain unchanged.
- Confirm empty tools, explicit foreground action, one request, bounded
  streaming, zero retry/fallback, volatility, cancellation, cleanup/quarantine,
  late-result rejection, and no-device-effect requirements remain explicit.
- Confirm no engine, model, artifact, provider, dependency, source, test,
  manifest, lockfile, Tauri, capability, permission, signing, or operational
  external-system path changed.
- Confirm the decision increment changes exactly the sixteen authorized paths
  and preserves dated V0, D-118, and D-119 artifacts.

### Future source and target-Mac tests — Not run

These requirements belong to later separately approved plans and are not tests
run by this documentation decision:

- exact dependency/feature/build-script/native/dynamic-code inventory and
  license/advisory review;
- model/tokenizer/template digest, corruption, substitution, version,
  incompatibility, arbitrary-path, and missing-artifact rejection;
- no DNS/socket/proxy/telemetry/update/license-check/remote-embedding/provider-
  SDK/default/fallback behavior;
- one explicit action, one process-wide request, immutable Rust-owned identity
  and profile, empty tools, closed errors, and bounded input/event/output/queue;
- incremental streaming and multibyte/chunk boundary handling;
- active initialization/model-open/first-event/idle/generation/total deadlines
  appropriate to the exact local engine, with no connection phase or local-
  server authority;
- cancellation during load, tokenization, prefill, generation, native callback,
  accelerator work, finalization, and teardown, with bounded join;
- cleanup ownership, quarantine, restart denial under ambiguity, and successful
  restart only after proved quiescence;
- malformed, foreign, stale-generation, duplicate, out-of-order, oversized, and
  every post-terminal result rejected before mutation;
- prompt/output/log/cache/temp/crash-report/redaction and volatile deletion
  checks; and
- exact target-Mac OS/CPU/GPU/RAM/disk compatibility, cold/warm load,
  first-token and total latency, peak/terminal memory, thermal behavior,
  cancellation latency, cleanup, no-egress, permission-prompt, resize/theme,
  accessibility, kill-switch, and rollback evidence.

Existing deterministic runtime, Personal Assistant fixture, sealed demo,
gateway, approval, and orchestration tests are design references only and may
not be reported as passing any future local-engine test.

## Verification commands

For this one-file planning task, run before editing and after the final plan
edit:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git status --short --branch
git diff --name-only
git ls-files --others --exclude-standard
```

The final planning scope check must report only this plan path. Do not run
`begin`, a session-end gate, a quality gate, or a post-increment finalizer for
this planning-only task.

For a later separately approved documentation decision, rerun those commands
and also perform:

- complete `npm run verify`, with the actual result recorded;
- exact sixteen-path inventory;
- append-only D-120 and historical-preservation checks;
- exact D-119 ten-entry/blocked/no-handle assertions;
- protected source, dependency, configuration, workflow, hook, and gate-path
  checks;
- independent architecture, security, documentation, and readiness reviews;
- repository-required session-end and quality workflows; and
- the exact post-increment finalizer only after the last authorized edit.

Record every command as Passed, Failed, Pending, or Not run. A valid completion
marker must bind the final report and exact workspace fingerprint.

## Manual gates

For this planning task:

- clean synchronized baseline and one-file scope: Passed;
- documentation-tier baseline validation: Passed;
- complete `npm run verify`: Not run for this one-file planning task; required
  only for the separately approved cross-cutting decision increment;
- owner approval of the decision increment and disposition: Passed; and
- engine/model/artifact, filesystem, no-egress, lifecycle, target-Mac, provider,
  credential, signing, product, and external-system evidence: Not run by scope.

For this documentation decision, owner acceptance of exactly one closed
disposition is mandatory. A positive direction does not authorize the later
engine/artifact evidence plan to begin automatically.

## Risks

- “Local” may be mistaken for proven private or zero-egress behavior.
- Reordering evidence work may be misrepresented as completing synthetic-v1 or
  making private-v2 usable.
- Fixed local-v2 may be confused with the selectable D-119 `local_no_auth`
  candidate.
- Provider ZDR may be declared inapplicable before hidden egress is disproved.
- Model artifacts may introduce filesystem, supply-chain, licensing,
  native-code, dynamic-code, cache, or update authority without a decision.
- In-process inference may share the desktop's privilege and memory-safety
  boundary and may not expose bounded cancellation.
- A helper or localhost server may be introduced as an allegedly harmless
  implementation detail even though it adds process, IPC, transport, and
  containment boundaries.
- UI cancellation or late-result rejection may be overstated as engine
  quiescence.
- Personal prompts may persist through caches, temp files, crash reports,
  logging, swap, or model/runtime behavior.
- Old V0 plan headers contain historical checkpoint statuses; editing them
  would rewrite dated evidence instead of adding current reconciliation.

These risks remain closed by the documentation-only scope, closed dispositions,
exact preservation assertions, and the requirement for separately approved
engine, artifact, dependency, containment, source, and target-Mac plans.

## Rollback or failure strategy

- Before publication, rollback of this planning task is removal of this one new
  untracked plan file. No runtime or external state exists to unwind.
- Before publication of the future decision, restore only its exact sixteen
  documentation paths to the recorded baseline; do not rewrite older evidence.
- After publication, correction requires an additive successor decision or a
  bounded documentation revert through normal Git review.
- `current_remote_first_sequence_retained` changes no V0 ordering and selects no
  successor.
- `evidence_boundary_failed` records the exact conflict without inferring local
  eligibility or changing any existing decision.
- No failure may trigger engine/model research, download, installation,
  dependency work, source edits, external access, fallback, or successor start.

## Stop conditions

Stop immediately and report rather than infer, repair, or widen scope if:

- the baseline is dirty, divergent, ambiguous, or no longer descends from the
  recorded commit;
- ignored gate status is active, failed, identifies another predecessor, or is
  invalid for any reason other than this exact plan's workspace fingerprint;
- work would require finalizing, rebinding, retargeting, or modifying the
  completed `personal-assistant-v0-pr110-publication-closeout` predecessor;
- D-120 or any exact plan/increment/review path is occupied;
- D-094, D-060, D-061, D-118, D-119, the V0 program, or current source differs
  materially from the recorded evidence;
- an accepted result would need to remove or modify synthetic-v1, admit a D-119
  profile, make a selector, or waive a remote prerequisite;
- a positive decision cannot keep local-v2 fixed and nonselectable;
- a conclusion requires external sources, a vendor/engine/model choice,
  artifact inspection, dependency resolution, build, execution, model install,
  benchmark, or target-Mac operation;
- a local proposal uses a localhost server, network, hidden egress, helper,
  process, plugin, dynamic runtime, arbitrary model path, runtime download,
  update, telemetry, remote embedding, provider SDK, or cloud fallback;
- hard cancellation, bounded join, cleanup ownership, quarantine, or late-result
  safeguards would be weakened or conflated;
- any credential, secret, personal prompt, raw content, provider detail, model
  path, account identifier, or sensitive system evidence would enter source,
  docs, logs, screenshots, terminal output, or ordinary CI;
- any file outside the exact current scope changes; or
- branch creation, gate begin/finalization, commit, push, merge, publication, or
  successor work is requested without separate exact authority.

## Acceptance criteria

- [x] The one-file ExecPlan is bounded, repository-evidenced, and Ready only for
      a separately approved documentation decision.
- [x] The plan defines exactly three closed dispositions, preserves the exact
      proposed D-120 wording, and records the owner-accepted positive result.
- [x] The plan preserves D-094 synthetic-v1, D-118, D-119's exact ten blocked
      profiles, V0-3/V0-7, all ten D-107 blockers, and every current security
      boundary.
- [x] No-egress, artifact, lifecycle, cancellation, cleanup/quarantine, late-
      result, bounded-stream, empty-tool, explicit-action, volatility, and
      no-fallback requirements are explicit.
- [x] The planning task changes only this file and runs documentation-tier
      validation.
- [x] The owner separately approves the decision increment and exact branch/gate
      actions.
- [x] The documentation decision selects exactly
      `fixed_local_v2_planning_selected` after consistent repository evidence.
- [x] The completed documentation decision passes its remaining exact gates and
      stops for owner review without starting a successor.

## Decisions made

- Use an additive fixed local-v2 planning decision rather than changing the
  selectable-v3 catalog or replacing the remote synthetic proof.
- Keep the planning task to one new file and the possible future decision to a
  closed sixteen-document ceiling.
- Accept `fixed_local_v2_planning_selected` as documentation planning authority
  only after repository evidence remained consistent.
- Treat zero-egress, engine/model artifact, bounded cancellation/join, cleanup,
  late-result, and target-Mac evidence as later conjunctive gates, not assumed
  properties of local execution.

## Discoveries

- D-118 is specific to its frozen hostname-based HTTPS designs. A truly
  network-free local lane may be independent of it, but a localhost or hidden
  transport cannot be called a bypass.
- D-119 intentionally places selection after v0. A fixed local-v2 lane therefore
  needs an additive D-094 sequencing decision before engine/model evaluation can
  safely advance the usable private milestone.
- The current V0 host provides useful lifecycle design evidence but no local
  engine or production response-ingress proof.
- An in-process candidate is structurally narrowest but may fail the hard
  cancellation/cleanup requirement; a helper may improve termination while
  triggering broader containment and process authority. Neither is selected.

## Progress

- 2026-09-03: confirmed clean synchronized `main` at
  `51be9ba91ba69c9c96dfb3bbfd4509d3177e902a`, inspected current source and
  D-094/D-118/D-119/V0 evidence, confirmed unused decision/artifact paths, and
  passed pre-edit documentation-tier validation.
- 2026-09-03: drafted this one-file Ready ExecPlan; no branch or gate began and
  no source, dependency, model, credential, provider, signing, or external
  system was accessed.
- 2026-09-03: the owner approved this exact plan and conditionally accepted the
  positive disposition if repository evidence remained consistent. Reconfirmed
  the sole plan-file delta, created the approved branch from the recorded
  baseline, preserved the completed predecessor, and began gate
  `pa-v0-fixed-local-private-lane-decision` exactly once.
- 2026-09-03: repository evidence remained consistent; accepted
  `fixed_local_v2_planning_selected`, appended D-120, and reconciled the fifteen
  pre-report paths within the exact sixteen-file ceiling. The completion report,
  remaining gates, and marker remain pending; no successor exists or starts.
- 2026-09-03: completed the exact sixteen-file documentation reconciliation.
  Documentation, repository, security, complete verification, exact-scope,
  preservation, independent-review, session, quality, report-validation, and
  post-increment gates passed. The completion marker is valid; owner review is
  pending, and no successor began.

## Final results

**PASS WITH ADVISORIES.** `fixed_local_v2_planning_selected` is accepted and
D-120 exists as documentation planning authority only. Every required gate
passed and the exact sixteen-file workspace has a valid completion marker. All
product and operational readiness remains `Blocked`; no successor is authorized
or active.

## Documentation updates for this decision

- [x] `ARCHITECTURE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md`
- [x] `HANDOFF.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `PRODUCT_REQUIREMENTS.md`
- [x] `PROJECT_STATUS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `TESTING_GUIDE.md`
- [x] `docs/PROJECT_DIRECTION.md`
- [x] Plan, increment, and post-increment review artifacts
