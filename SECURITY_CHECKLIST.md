# Cortexa security checklist

Status: Authoritative change and release security review checklist
Last updated: 2026-09-02

Use this checklist with `SECURITY.md`. Mark an item not applicable only with a
short reason grounded in the actual diff. A plan or test fixture does not prove a
production boundary exists.

## D-104 bootstrap trust-class decision

- [x] `sandbox_activation_adhoc_v1` and `product_signer_binding_v1` are
      distinct conceptual classes with no runtime or operational authority.
- [x] The bootstrap class has no Developer ID, certificate, private key, Team
      ID, Keychain, provisioning, authentication, distribution, or product
      identity claim, and cannot satisfy P4.
- [x] Any eligibility relief is limited to one future static-review
      classification; D-103's historical no-selection result remains intact.
- [x] D-102 effect, bootstrap, graph, terminal, cleanup, evidence, no-root,
      no-global-state, and no-new-dependency requirements remain conjunctive.
- [x] No broad network, filesystem, IPC, automation, Keychain, device, or
      temporary-exception entitlement is permitted or implied.
- [x] P3-3 through P3-5, P4, signing, V0-3, and operational successors remain
      Blocked; no target-Mac or external action is represented as passed.
- [x] D-097 and every historical Failed/Pending/Not-run fact remain unchanged.

## D-103 containment primitive selection

- [x] Candidate identities are frozen; there is no catch-all or caller-selected
      alternative.
- [x] Every D-102 eligibility rule is conjunctive; no score, compensating
      control, empirical inference, or residual-risk acceptance is permitted.
- [x] The App Sandbox helper candidate is rejected because it requires
      entitlement/signing state prohibited independently by P3-2; that signature
      class is not equated with P4's later signer proof. It also lacks complete
      public detached-descendant membership, shutdown, and quiescence contracts.
- [x] `sandbox-exec`, process-group supervision, and output-root/post-hoc scans
      remain fixed negative controls and are not containment proof.
- [x] Privileged system/Endpoint/Network Extension routes are rejected for
      entitlement, signing, privilege, or persistent-state expansion; the VM
      route also requires an entitlement and guest resources. The container
      branch remains `contract_unproven` because no exact qualifying contract
      was identified.
- [x] All ten P3-3 implementation-source checks remain `not_run`; absent or
      ambiguous mandatory contracts remain `contract_unproven`.
- [x] Only the deep-review candidate enters one private candidate/check/attempt-
      bound D-100 review; negative controls retain prior exclusions and scope-
      only classes remain outside the D-100 attempt.
- [x] Unexpected source/provenance state, target-derived data, or malformed
      evidence is `boundary_failed` and stops without retry.
- [x] Public source citations and categorical dispositions contain no target-
      derived sensitive data and grant no operational authority.
- [x] The decision says only “no eligible candidate in the reviewed set”; it
      does not claim universal impossibility or admit an unreviewed candidate.
- [x] D-097 and every historical Failed/Pending/Not-run fact remain unchanged.
- [ ] A fully eligible primitive and separately approved P3-3 controller plan
      exist; both remain Blocked.
- [x] No source, dependency, entitlement, build, process, probe, target-Mac,
      Apple, Keychain, signing, credential, provider, product, or external
      state change occurred; approved read-only public documentation access was
      the sole external contact.

## D-102 build-child-containment planning

- [x] D-102 documents a fixed application-owned no-input future containment
      policy; it does not select or implement a primitive.
- [x] The future contract freezes a reviewed transitive graph, fixed executable
      identity/literal arguments, working directory, closed environment, and
      closed descriptor inheritance; shell/generic-runner/fallback/retry paths
      are prohibited.
- [x] Both host-data reads and filesystem writes require confinement; output
      routing, clean Git status, post-hoc scans, caches, logs, and process groups
      alone cannot prove containment.
- [x] Outside-root writes and undeclared network effects must be denied before
      effect; post-effect observation can only diagnose failure.
- [x] Authoritative descendant ownership survives fork, exec, reparenting,
      `setsid`, and `setpgid`; terminal cancellation/deadline requires shutdown,
      direct-child reaping, pipe closure, and race-free quiescence.
- [x] Descriptor-bound non-following cleanup quarantines failure and blocks
      retry/replacement; it cannot widen deletion authority or abandon ownership.
- [x] The 32 predeclared D-100 check IDs are unique, bounded lowercase ASCII,
      one-predicate records with `boundary_failed` in every allowed outcome set;
      no raw process/build/host/path content is a future evidence field.
- [x] D-097 failed/FAIL/Blocked, its Failed privacy finding, the Pending Open
      Directory boundary, and Not-run signing results remain unchanged.
- [ ] A supported no-new-dependency target-Mac primitive, controller, synthetic
      proof, and disposable no-sign build proof exist; all are Blocked and Not run.
- [x] No build, process, probe, filesystem/network effect, Apple, Keychain,
      certificate, signing, credential, provider, product, or external action
      occurred in this documentation increment.

## D-101 account-directory boundary planning

- [x] The documented application policy is fixed and no-input; its operation-
      specific opaque capability wrapper is process-private, non-serializable,
      attempt-bound, and never reused.
- [x] Caller, WebView, model, runtime, environment, account record, command
      output, or path cannot select the future scope.
- [x] The documented future adapter contract prohibits directory/passwd/account
      lookup, numeric UID/eUID-to-account derivation, login/session/console-user,
      home/standard/current/temporary/configuration-directory, search-list,
      shell/environment, subprocess/path, and generic filesystem resolution.
- [x] Exact scope provenance must establish one fixed application credential
      domain and reject ambient current-user/login/default-Keychain/default-
      search-list/current-directory/environment authority.
- [x] The adapter privately owns any platform-issued reference; its wrapper has
      no generic enumerate/read/write/delete/sign authority. Cleanup is attempted
      on every terminal path; failure retains private ownership until process
      exit and blocks replacement/retry/reuse/exposure.
- [x] Static application-source guarantees are separated from unproven
      OS-internal directory/cache/log/socket/trust/network effects.
- [x] Directory/account-record, cache, log, socket, trust-service, process-
      metadata, and network effects each have a separate one-predicate D-100
      review; none substitutes for another.
- [x] Platform-contract ambiguity blocks; runtime observation cannot prove
      universal absence and owner acceptance is not an automatic fallback.
- [x] Explicit application resolution, application input, and ambient/default
      scope are non-waivable under D-101; later exact acceptance can disposition
      only separately disclosed OS-internal uncertainty.
- [x] Future review categories use D-100, are non-authorizing, and include
      `boundary_failed` for every check.
- [x] The twelve unique check IDs and six outcome tokens meet D-100 lowercase-
      ASCII grammar and 64/32-byte bounds; the longest canonical three-field
      record remains within 256 bytes.
- [x] The consumed query remains prohibited and the historical Open Directory
      finding remains Manual verification pending.
- [ ] A future implementation selects a supported platform API and enforces the
      exact contract with focused tests; this documentation increment does not.
- [x] P3/P4 and every operational successor remain Proposed/Blocked.

## D-100 evidence-privacy protocol planning

- [x] The protocol has exactly a fixed version, a future-plan-owned check ID,
      and a check-owned closed outcome; it has no free-text or extension field.
- [x] The documented future parser contract requires bounded lowercase ASCII,
      exact comparison, one canonical bounded record, unique keys, and no
      normalization/aliases; no parser exists in this increment.
- [x] The documented future consumer contract treats the record as
      non-authorizing, binds it privately to one plan/check/attempt, and rejects
      duplicates, replay, substitution, pre-admission, and late arrival; no
      consumer exists in this increment.
- [x] Each check/outcome represents one bounded predicate and cannot claim
      approval, authorization, safety, readiness, exclusivity, historical
      absence, broad verification, or permission to proceed.
- [x] Screenshots, recordings, attachments, transcripts, raw output, logs,
      target-derived sensitive identifiers, metadata, private paths,
      credentials, and content are prohibited evidence; static protocol
      literals are distinct.
- [x] Source minimization is mandatory; capture-then-redact is not accepted.
- [x] Unknown or malformed evidence, prompts, ambiguity, unexpected output,
      side effects, and boundary failure stop without retry.
- [x] Every operational check predeclares `boundary_failed`; incident handling
      never widens an outcome enum after execution starts.
- [x] A failure records only `boundary_failed`, preserves historical truth, and
      requires separate disposition before related operations resume.
- [x] D-097/D-098/D-099 and the Failed privacy, Pending Open Directory, and
      Not-run signing evidence remain unchanged.
- [ ] A future operational plan implements and reviews its exact local
      minimization boundary, check/outcome table, manual gates, and owner
      approval; this documentation increment does not.
- [x] P2 account-directory, P3 build-child containment, P4 signer binding, and
      every operational successor remain Proposed/Blocked.

## D-099 documentation-only signing-security prerequisite planning

- [x] The exact D-098 schema-v3 lineage admits only the named documentation
      increment and its fifteen-path closeout ceiling.
- [x] D-097 remains `failed` / `FAIL` / `Blocked` without a completion marker;
      the historical screenshot/privacy failure remains Failed and the Open
      Directory boundary remains Manual verification pending.
- [x] The plan distinguishes P1 privacy evidence, P2 account-directory scope,
      P3 executable-build-child containment, and P4 immutable signer binding.
- [ ] No target-Mac, Apple, Keychain, certificate, private-key, signing, build,
      provider, credential, network, or product operation is represented as
      passed by this documentation increment.
- [ ] Any future operational plan has its own owner approval, exact threat
      model, no-caller-input contract, redaction review, and target-Mac gates.

## Scope and threat model

- [ ] The change identifies trusted and untrusted actors, data, and boundaries.
- [ ] Current, mocked, planned, and prohibited behavior are distinguished.
- [ ] New data flow, privilege, persistence, network access, or execution
      authority is explicitly identified.
- [ ] Exact files, dependencies, capabilities, permissions, and rollback are
      bounded before implementation.
- [ ] No model output, WebView state, gateway data, external content, or tool
      result is treated as authorization.

## Native agent governance

- [ ] Agent attribution is derived only after exact live task, lineage,
      runtime, run, request, and profile validation; callers cannot override it.
- [ ] Agent policy input is distinct from legacy gateway `PolicyInput` and has
      no conversion that bypasses profile eligibility.
- [ ] Agent definitions are the sole agent-to-profile mapping; unknown,
      missing, duplicate, stale, or mismatched profile identity fails closed.
- [ ] Runtime tool proposals remain rejected unless a later approved runtime
      coordinator explicitly changes that boundary.
- [ ] Delegation remains outside `ToolRegistry`; only the orchestrator creates
      child tasks through an exact closed matrix and finite limits.
- [ ] Approval retains exact closed origin and one-time subject binding through
      pending, presentation, source result, cancellation, expiry, and resolution.
- [ ] Governance audit reserves bounded evidence before downstream mutation,
      redacts content/identity in Debug, and records exact matrix, policy,
      approval, control, error, and `NotAttempted` execution disposition.
- [ ] Cancellation resolves and audits a pending approval before runtime/task
      mutation, with child-first ordering for root cancellation.
- [ ] No policy/approval outcome reaches an executor, device, provider, IPC,
      memory store, filesystem, network, process, or platform adapter.

## Volatile agent memory and approved documents

- [ ] The immutable built-in definition is the sole agent-to-memory-profile
      mapping, and the exact profile is sealed through task, execution context,
      live attribution, and each memory grant.
- [ ] Agent memory operations require a non-cloneable live grant derived only
      after exact agent, task, root, parent, policy, memory-profile, runtime,
      depth, run, and request validation.
- [ ] Application review, approved-shared deletion, memory disable, document
      registration, and revocation use separate application-control proofs that
      no model, runtime, WebView, file, or caller identity can construct.
- [ ] Approved-shared, agent-private, task-temporary, and proposed-shared memory
      have exact owners, bounds, retention, deletion, cleanup, disable, and
      workflow-drop semantics; no record crosses orchestrator instances.
- [ ] Shared proposals remain inert until an exact expected-version review;
      edit, approve, approve-edited, reject, withdraw, and delete failures leave
      all records, versions, counts, and byte accounting unchanged.
- [ ] Context assembly names exact record IDs, rejects duplicates and
      inaccessible namespaces, applies record/count/byte bounds atomically, and
      never copies history, documents, siblings, private records, or a namespace
      implicitly.
- [ ] Approved-document registration accepts only trusted application-selected
      files, attachments, generated artifacts, or exact approved-root members;
      paths are private and no enumeration API exists.
- [ ] Document reads accept only bounded nonempty lowercase `.txt`/`.md` UTF-8
      content and reject traversal, noncanonical components, symlinks, hard-link
      aliases, non-regular files, replacement, detected mutation, replay,
      revocation, and foreign workflow/task authority.
- [ ] Supported Unix targets compare registered, opened-handle, and final-path
      identity before and after the bounded read; the pure-`std` TOCTOU advisory
      is recorded, and unsupported targets fail unavailable.
- [ ] A document reference follows one linear available/reserved/consumed or
      revoked lifecycle; failed root cancellation aborts the reservation, while
      post-cancellation child-start failure never advertises retry.
- [ ] Document and selected-memory input is labeled untrusted and bounded before
      the runtime request. Paths/content are absent from Debug, errors, events,
      audit, logs, SQLite, and automatic memory.
- [ ] Knowledge eligibility remains non-authorizing. D-085's direct Personal
      Assistant document route and D-086's sealed fixture workflow are separate
      application-service paths; generic Personal-to-Research remains unchanged,
      generic/direct Research-to-Knowledge remains denied, and no specialist can
      delegate or create a child.
- [ ] No IPC, file picker, provider, live model, durable memory, vector search,
      background index, document write, new dependency, permission, executor,
      or device effect is introduced by this boundary.

## Sealed Research and Knowledge workflow

- [ ] Trusted application code selects the exact fixture-only V1 workflow from
      a live Personal Assistant root before output; untrusted content, a runtime,
      an agent, the WebView, or catalog activation cannot select or widen it.
- [ ] The workflow is fixed at three tasks, two non-replenishing sequential
      depth-one sibling children, one active child, four run attempts, 32 runtime
      events, 32 generic events, 16 workflow events, 16 matching audit records,
      and zero automatic retries. Generic/document paths retain their limits.
- [ ] The immutable source catalog contains one to eight deterministic fixtures
      with canonical application-issued IDs. Research references only catalog
      IDs; Knowledge references only IDs present in the exact validated
      predecessor Research task/version; unknown, duplicate, remapped, or
      invented references fail closed.
- [ ] The objective is capped at 2,048 scalar values/8,192 bytes, aggregate
      fixture evidence at 8,192 bytes, serialized catalog at 16,384 bytes,
      specialist results at 8,192 scalars/16,384 bytes, each Research/Knowledge
      input at 26,624 bytes, and synthesis at 36,864 bytes with a 4,096-byte
      disclosure/framing sub-bound. Exact limits reject rather than truncate and
      have N/N+1 plus Unicode coverage.
- [ ] Specialist JSON rejects unknown fields, trailing or outer content,
      malformed values, duplicate references, unsupported confidence or version,
      and reasoning. Missing references or explicit incompleteness produce typed
      partial quality: a Research missing-source partial skips Knowledge, while
      a valid Knowledge partial is retained. Raw invalid output never reaches
      the next stage.
- [ ] Final Personal synthesis is a strict V1 envelope whose answer is capped at
      2,048 scalars/8,192 bytes, source-ID set exactly matches the validated
      Research outcome, fixture disclosure is true, partial status is disclosed
      when applicable, and status matches the validated stage outcomes.
      Invented/duplicate/unknown IDs, false disclosure, wrong status, URLs,
      live-research claims, reasoning, malformed data, and unknown fields fail
      the root without a completed workflow result.
- [ ] Before a terminal runtime event is accepted, the root event cap and all
      transition capacity are checked and parsing, terminal output, next
      task/request/descriptive attribution, and fallback or synthesis input are
      prepared. A preparation error leaves the run, task, budget, events, audit,
      memory, and workflow phase unchanged.
- [ ] A continuation runtime-start failure does not turn an already accepted
      terminal event into rejection, retry it, or replenish budget. The failed
      attempt is terminalized, fallback or root failure is applied, and only the
      closed content-free continuation-failure variant is exposed.
- [ ] Research failure/cancellation/invalid output and valid missing-source
      partials skip Knowledge and permit only typed partial Personal synthesis.
      Knowledge failure/cancellation/invalid output preserves only validated
      Research; a valid incomplete Knowledge result remains partial. Synthesis
      failure fabricates no result, and the 32-event hard cap starts no fallback
      that cannot safely accept an event.
- [ ] Root cancellation resolves pending governance and cancels the active child
      before the root, records one workflow cancellation, starts no next stage,
      rejects late events, and preserves retryable live state when cancellation
      itself fails.
- [ ] Research and Knowledge access only their own agent-private and
      task-temporary memory through exact live grants. Terminal cleanup removes
      task memory, sibling memory is never copied, and reusable Knowledge remains
      pending review with no automatic proposal, approval, selection, persistence,
      or synthesis-as-fact.
- [ ] Workflow events and matching volatile audit records are content-free.
      `ResearchKnowledgeAttribution` is descriptive only, keeps run/request
      identity private and redacted, cannot reconstruct live authority, and
      contains no objective, source content, finding, summary, proposal, path,
      URL, output, or reasoning.
- [ ] D-086 changes no provider/network/process/filesystem/tool/executor,
      persistence, IPC/UI, dependency, capability, permission, external runtime,
      `AgentRuntime`, `NativeAgentRuntime`, or Native sole/default boundary.

## Sealed engineering quality workflow

- [ ] Trusted application code alone selects D-087 from an exact live Personal
      Assistant root; generic, document, D-086, and engineering selectors are
      mutually exclusive and no specialist creates or delegates a task.
- [ ] Coding, QA, and Security are sequential depth-one siblings under exact
      four-task, three-child, five-run, one-active-child, 32-event,
      16-workflow/audit-record, and zero-retry limits.
- [ ] Fixture, criterion, and evidence IDs are immutable application-issued
      references. Evidence is only `ObservedFixture` or `NotRun` and cannot
      claim a live repository observation or executed test.
- [ ] `ChangeProposal`, `ValidationReport`, `RiskAssessment`, and final
      synthesis reject unknown/duplicate/malformed/oversized/trailing output,
      reasoning, URLs, identity injection, authority claims, invented
      references, and inconsistent stage status.
- [ ] QA reconciles every criterion exactly once, cannot approve or fabricate
      execution evidence, and forces incomplete/blocked status for
      not-demonstrated criteria. Security remains evidence-bound or explicitly
      hypothetical and cannot authorize, remediate, replace policy, or expose
      secret values.
- [ ] File write/delete/path escape, dependency/package/test/formatter
      execution, Git operations, destructive shell, credential access, and
      network access are denied proposal data with no dispatch path.
- [ ] Final synthesis discloses fixture-only, proposal-only, and no-execution
      status. `RequiredBeforeMutation` is application-derived for a patch but
      creates no approval request or execution authority.
- [ ] Coding, QA, and Security remain tool-ineligible and memory-disabled;
      runtime tools fail closed, execution is `NotAttempted`, generic routes are
      unchanged, and Native remains sole/default and unchanged.
- [ ] D-087 adds no live repository/filesystem/process/Git/package/network
      access, tool schema, executor, mutation, dependency, Tauri/React behavior,
      IPC, provider, external runtime, permission, or device effect.

## Sealed infrastructure and systems operations workflows

- [x] Trusted application code alone selects exactly one D-088 Cloud or Systems
      workflow; both are mutually exclusive with every existing selector and no
      specialist spawns or delegates.
- [x] Cloud or Systems, QA, and Security are sequential depth-one siblings
      under exact four-task, three-child, five-attempt, one-active-child,
      32-event, 16-workflow/audit-record, and zero-retry limits.
- [x] The Cloud built-in contains only synthetic Terraform configuration, Azure
      architecture, and validation evidence. The Systems built-in contains only
      a synthetic service snapshot, sanitized log, recovery scenario, and
      validation evidence.
- [x] Strict stage outputs preserve application-issued scenario/fixture/
      criterion/evidence/predecessor/result provenance and reject unknown,
      duplicate, malformed, oversized, reasoning-bearing, false-live,
      false-execution, or identity-supplying output.
- [x] QA cannot approve, fabricate execution, or treat `NotRun` as a pass.
      Security remains evidence-bound or hypothetical and cannot authorize,
      remediate, replace policy, or invent credential/platform evidence.
- [x] Terraform/platform commands, live inventory/diagnostics, mutation,
      service/process control, reboot/shutdown, configuration/package/patch,
      privileged shell, VMware/backup mutation, credential access/rotation,
      filesystem/network access, and other consequential capabilities are
      denied inert data with no dispatcher.
- [x] Cloud, Systems, QA, and Security remain tool-ineligible, memory-disabled,
      and `NotAttempted`; no approval request or executable subject exists.
- [x] String and credential-pattern guards are defense in depth only and never
      substitute for trusted authorization, containment, credential handling,
      or a separately approved live/effect plan.
- [x] D-088 adds no tool, command, credential, live access, executor, approval
      dispatch, provider, IPC/UI, dependency, permission, persistence, external
      runtime, `AgentRuntime`/`NativeAgentRuntime` widening, or device effect.

## Typed Workflow Automation proposals and manual dispatch

- [x] Trusted application code alone selects the sealed Personal -> Workflow
      Automation -> Personal proposal lifecycle; generic Personal-to-Workflow-
      Automation delegation remains denied and no specialist creates a task.
- [x] Five immutable templates have exact steps and dependencies. Validation
      rejects unknown or disabled agents, unknown/mismatched tools, malformed
      arguments, duplicate/cyclic dependencies, unsupported/nested/self-
      modifying steps, excessive limits, and false authority or effect claims.
- [x] Known tool and approval steps are recognized only for fail-closed review;
      executable tool count is zero, no policy or approval authority is invoked,
      no approval subject exists, and no token can issue for such a proposal.
- [x] Complete canonical A-D proposals alone may issue one opaque, non-cloneable,
      non-serializable, process-local token. It is taken once, consumed on every
      success or error, and maps in a fresh orchestrator only to the matching
      existing sealed fixture workflow. Template E is proposal-only.
- [x] The original 120-second monotonic deadline propagates into manual A-D
      dispatch and is checked cooperatively at trusted lifecycle ingress.
      Expiry performs child-first cancellation and starts no successor; the
      design does not claim to preempt an in-flight synchronous runtime call.
- [x] Proposal events/audit are content-free and capped at eight each; manual
      dispatch records are capped at four and retain only opaque application-
      derived identity, disposition, and terminal state.
- [x] Workflow Automation is `Initial` only for the sealed proposal selector,
      tool-ineligible, memory-disabled, non-spawning, and non-authorizing. D-090
      adds no general engine, scheduler, persistence, parallelism, tool
      execution, approval dispatch, provider, IPC/UI, external runtime, I/O,
      credential access, permission, or device effect.

## Bounded parallel specialist workflows

- [x] Trusted application code alone selects D-091; selectors are mutually
      exclusive, specialists and Workflow Automation cannot spawn or select a
      nested workflow, and duplicate logical requests fail closed.
- [x] Exact limits are depth one, default active two, hard active and total
      child three, four tasks, five run attempts, zero retries, eight events per
      run, 32 applicable records, a 120-second root lease, and 60-second child
      leases capped by the root. Counters do not replenish.
- [x] Each admitted child has distinct task, run, execution context, policy and
      memory attribution, output state, cancellation handle, deadline, and
      task-memory key. Cross-run, stale, late, terminal, wrong-sequence, and
      over-cap events cannot mutate a sibling.
- [x] `ContinuePartial`, `CancelDependentOnly`, and specialist-lane `FailFast`
      are explicit application-owned policies. Root and policy cancellation
      sweep in ordinal order and preserve closed resumable state on failure;
      rejected run identities remain quarantined until cleanup succeeds.
- [x] Every runtime start validates exact application-created returned identity,
      rejects duplicate live identity, and blocks all new or fallback starts
      until rejected nonterminal-run cleanup succeeds.
- [x] Outcomes are exactly succeeded, failed, cancelled, timed out, or skipped
      and are stored, transferred, cancelled, and synthesized in immutable
      catalog ordinal order rather than completion timing or map iteration.
- [x] Strict Personal synthesis reconciles every source agent, status, finding
      ID, failure, and unresolved issue. Partial results are disclosed;
      malformed or concealing synthesis fails the root.
- [x] Scenario-A memory access preserves exact existing live grants and sibling
      isolation; every task-temporary namespace is cleaned terminally. Other
      specialist profiles remain memory-disabled, and no result copies or
      promotes memory.
- [x] Workflow/audit evidence is bounded, content-free, descriptive, and non-
      authorizing. Planned slots do not fabricate task/run identity and live
      records require exact runtime attribution.
- [x] Same-thread retained-run multiplexing is not represented as provider or
      CPU concurrency, hard preemption, provider-session isolation, or an app-
      global capacity coordinator. The fixture claim filter remains defense in
      depth only.
- [x] D-091 adds no runtime-trait/Native change, provider, thread/async worker,
      scheduler, general graph engine, tool, policy permission, approval
      dispatch, persistence, I/O, dependency, IPC/UI, remote/distributed
      infrastructure, or device effect.

## Deterministic Command Center and read-only projection

- [x] Every displayed entity/event is closed, bounded, fixture-derived,
      redacted, and persistently labeled simulated; fixture IDs have no trusted
      Rust identity or authority.
- [x] Search, filters, selection, graph controls, inspector, structured view,
      and activity mutate only feature-local presentation state and expose no
      consequential action.
- [x] Command Center fixture controls invoke no Tauri command/listener, network,
      clipboard, storage, filesystem, provider, model, tool, approval, policy,
      audit, runtime, permission, or device path. In the selected scenario, a
      separate projection panel makes one explicit argument-free read-only
      query, and a separately labelled prop-free lifecycle panel owns the sole
      fixed listener and no-input lifecycle client.
- [x] Rust owns every projection identity and returns only a closed bounded
      synthetic DTO. The WebView runtime-narrows the response, maps failures to
      fixed unavailable copy, and cannot supply agent, task, run, profile,
      runtime, or workflow identity.
- [x] The projection starts no workflow and adds no provider, model, network,
      credential, tool execution, approval dispatch, persistence, filesystem,
      background autonomy, generic workflow engine, durable audit, or device
      effect.
- [x] `@xyflow/react@12.11.3` and `lucide-react@1.33.0` are the only new
      direct production dependencies; 19 transitives, licenses, peers, lockfile
      effects, production audit, and bundle budgets were reviewed.
- [x] React Flow types/imports stop at one topology adapter. The later
      projection changes only the exact no-argument read-only command/client
      boundary; storage, capability, CSP, permission, dependency, and event
      boundaries remain unchanged and statically pinned.
- [x] Required real-browser/Tauri viewport, input, focus, computed-overflow,
      contrast, reduced-motion, browser-zoom, screenshot, and native-resize
      evidence passed through the approved Browser Control and Computer Use
      runtimes. One scoped light-theme compact-text contrast defect found by M5
      was corrected and revalidated; touch was unavailable where unsupported.

## Volatile Research/Knowledge lifecycle core

- [x] Public production construction and `start`, `advance`, `cancel`, and
      `snapshot` accept no caller-selected identity, fixture, script, event, or
      content; all trusted values remain application-owned.
- [x] Every runtime start remains behind `AgentOrchestrator` F-01 validation.
      Returned-identity cleanup faults retain F-02 ownership, block restart,
      and retry only through no-argument cancellation.
- [x] Persistent Drop-time cleanup failure retains the owner until process
      exit and process-wide denies replacement start/advance/cancel. The
      sentinel is not a concurrency coordinator or future IPC design.
- [x] Lifecycle DTOs, errors, serialization, and Debug output are closed,
      bounded, and content-free; no task/run/request/context, fixture evidence,
      result, path, URL, or raw internal error is exposed.
- [x] No Tauri command/event/state, frontend consumer, capability, CSP,
      permission, provider, model, network, credential, tool, approval dispatch,
      persistence, filesystem, timer, thread, worker, dependency, or device
      effect was added by the core increment.
- [x] The connected presentation retains four no-input commands and one fixed
      notification. Validated command responses alone can commit presentation;
      malformed/older/same-revision events are inert, and every parser-valid
      newer event requires explicit recovery without rendering its state.
- [x] The private completed-run schedule is success -> failure -> repeat;
      cancellation at every active stage consumes no outcome, and no selector
      crosses IPC or the feature boundary.
- [x] F-12 pins the sole client/panel, exact source digests, command/event token
      locality, no-argument operations, and prohibited raw Tauri/browser
      surfaces. It is a static regression guard, not runtime authorization.
- [x] Target-Mac startup and browser fallback evidence pass. Source-current raw
      debug lifecycle interaction, alternate native theme/reduced motion,
      page zoom, and native resize are `Not run` because approved tooling could
      not bind to the raw debug executable.

## Native multi-agent deterministic demonstrations

- [x] Every demonstration discloses deterministic fixture data,
      `MockAgentRuntime` acceptance proofs versus the sealed no-input
      `NativeAgentRuntime` demo host, real governed boundaries, simulated
      external results, and the absence of consequential product or
      external-system effects.
- [x] No provider/application network, live model, credential, production
      document, repository, cloud account, host, service, tool executor, IPC
      command, or new permission participates in the acceptance suite. Cargo is
      locked but not offline, so a cold toolchain cache may fetch locked crates.
- [x] The closed delegation matrix denies every specialist source; only
      `AgentOrchestrator` creates tasks and all identity substitution/replay
      cases fail closed.
- [x] Workflow Automation retains zero executable tool steps. Non-executable
      tool/checkpoint proposals and application-owned take-once manual fixture
      dispatch remain separate and cannot create approval or execution
      authority.
- [x] QA remains advisory and cannot approve; Security remains advisory and
      cannot authorize; unknown tools and destructive cloud/systems capability
      requests remain denied and `NotAttempted`.
- [x] The simulated macOS approval rejection is bound to the exact originating
      Personal task/root, clears pending state, records
      `ApprovalResolved`/`Rejected`/`NotAttempted`, and executes no action.
- [x] Approved-document and memory fixtures exercise explicit grants,
      isolation, cleanup, and versioned application review without silently
      persisting shared knowledge.
- [x] Audit evidence is bounded, redacted, attributed, process-local, and
      non-authorizing; it is not represented as durable product audit.
- [x] The native acceptance suite itself has no connected UI and is not
      end-to-end product evidence. The separately connected sealed demo panel
      has independent presentation evidence and does not connect the acceptance
      workflows to the frontend fixture projection or real execution.

## Tauri IPC review

- [ ] Every command and event is narrow, typed, explicitly registered, and
      required by a user-visible workflow.
- [ ] External payloads enter as `unknown` or equivalent and are validated at
      the boundary.
- [ ] The WebView cannot select raw SQL, shell commands, arbitrary paths,
      provider parameters, tool schemas, risk, permission, approval evidence, or
      execution targets.
- [ ] Errors returned across IPC are closed and redacted.
- [ ] Success, malformed input, denial, cancellation, and unavailable-core paths
      have tests.
- [ ] No generic `execute_action`, shell, filesystem, database, gateway, or tool
      dispatch command exists.

## CSP, capabilities, and plugins

- [ ] `src-tauri/tauri.conf.json` CSP changes are absent or narrowly justified.
- [ ] Capability files name only required windows and least-privilege permissions.
- [ ] New Tauri plugins, commands, events, scopes, and allowlists are reviewed
      together.
- [ ] Development-only origins or unsafe directives do not expand production
      access unintentionally.
- [ ] Capability, CSP, plugin, and generated-schema diffs are included in the
      security review.

## API keys, tokens, and credentials

- [ ] No production provider key is embedded in source, bundle, frontend,
      desktop Rust, SQLite, logs, audit, crash output, tests, or fixtures.
- [ ] Gateway credentials are server-side; any future desktop token is short-
      lived, audience-bound, held in Rust memory, and refreshed through approved
      platform secret storage.
- [ ] OAuth tokens, passwords, authentication codes, private keys, certificates,
      and database keys never enter SQLite or the WebView.
- [ ] Credential errors, debug output, telemetry, and support bundles are
      redacted.
- [ ] Rotation, revocation, expiry, and compromise response are documented for
      every live credential.

## Gateway and provider boundary

- [ ] D-064's four stages are enforced independently: design evidence does not
      authorize provisioning, provisioning does not authorize traffic,
      synthetic traffic does not authorize real content, and no stage starts
      automatically.
- [ ] D-094's synthetic version 1 accepts no caller text or trusted selector;
      the fixed fixture, instruction, OpenAI/Cloudflare profile, model,
      `empty@1`, limits, one request, zero retry, and no fallback are exact.
- [ ] Synthetic start requires the exact current disclosure acknowledgment;
      trusted Rust mints and consumes one volatile private admission before the
      sole transport path may write a socket. The UI does not pre-check it.
- [ ] Before V0-9's earlier zero-body authentication socket, the terminal shows
      the exact `personal-assistant-access-auth-probe@1` disclosure, requires
      exact interactive acknowledgment, and Rust consumes a distinct one-use
      admission. Missing/wrong/non-terminal input opens no socket, and the token
      cannot start model transport.
- [ ] Synthetic-v1 and any future real-content-v2 command, handle, parser,
      disclosure, instruction, data-class, and gateway profiles reject each
      other. D-068's service token never authorizes personal content.
- [ ] The Worker independently verifies the Access service-token application
      JWT: RS256/key constraints, fixed JWKS endpoint, exact issuer/single AUD,
      `type=app`, expected Client ID `common_name`, empty `sub`, required
      integer `iat`/`exp`, optional bounded `nbf`, clock/lifetime bounds, and
      closed refresh/cache limits.
- [ ] Rust uses one reviewed direct HTTPS client only after a separate
      dependency decision; fixed origin, TLS, redirects, proxies, bounded
      incremental reads, active deadlines, original-request abort, cleanup
      quarantine, and late-byte rejection are proved without WebView,
      subprocess, transitive/private API, or TLS bypass.
- [ ] Access authentication-only traffic, provider-secret provisioning, live
      synthetic provider traffic, and real-content activation are separate
      approvals with route/kill/revocation/deletion rollback between them.
- [ ] V0-5 owns exact issuer/AUD/JWKS/origin configuration, V0-8 owns expected
      service-token Client ID, and V0-12 owns the provider secret. Every binding
      matches its owner before traffic; missing/malformed/drifted values deny.
- [ ] The Service Auth policy uses the service-token resource `id` as
      `token_id`; JWT `common_name`, the expected-client binding, and Keychain
      use the distinct `client_id`. Separate sanitized fingerprints tie both to
      one reviewed token record without comparing them for equality.
- [ ] Every credential handoff names the exact authenticated UI, transient
      buffers, pasteboard controls, cleanup owner, rotation/replacement
      semantics, and revocation order; no value crosses shell, environment,
      file, source, chat, screenshot, history, CI, test, or log.
- [ ] The Responses adapter pins current event names/order, closed
      event-specific keys, a private sequence origin plus contiguous
      increments, response/message identity consistency, text/item/content
      indices, intermediary handling, terminal failures, and
      refusal/reasoning/tool rejection. Unknown provider material never becomes
      a normalized frame.
- [ ] `PA_V0_TRAFFIC_ENABLED=false` denies new admission. Route removal and the
      flag are never claimed to abort an active request; active teardown uses
      the owned desktop/Worker abort chain and retains cleanup ownership.
- [ ] Application content logging is absent. Access, Worker, OpenAI, and
      control-plane metadata sinks, retention, access, and deletion ownership
      are recorded honestly; vendor metadata is not described as “no logging.”
- [ ] Access authentication-request and Worker/runtime metadata are content-free
      and retained at most seven days. Cloudflare's mandatory admin-action audit
      trail is separately classified; its current 18-month synthetic-only
      acceptance is reverified for no content/secret fields before Stage C and
      does not authorize real-content processing.
- [ ] The approved threat model and closed configuration specification are
      reviewed for the exact environment before Stage B, Stage C, or Stage D.
- [ ] D-062's Microsoft personal-account boundary is implemented only after the
      exact desktop client, gateway resource, discovery-derived issuer, tenant,
      loopback redirect, delegated scope, and token-validation configuration are
      approved and evidenced.
- [ ] Authorization uses a system browser, PKCE S256, one-time `state`, OIDC
      `nonce`, one callback, and terminal denial, timeout, and cancellation.
- [ ] Work, school, guest, arbitrary Entra tenant, wrong issuer, wrong audience,
      wrong scope, and replayed-code paths fail closed.
- [ ] Initial scopes are only `openid`, `email`, and one exact delegated gateway
      scope. `offline_access` and persistent sessions remain disabled without a
      separate decision.
- [ ] The account key is provider ID plus normalized issuer plus subject; email
      never identifies or automatically links accounts.
- [ ] The registrations are separate public desktop and gateway API
      applications; the Application ID URI format is
      `api://<gateway-api-client-id>`, the only delegated scope is
      `gateway.access`, and the callback is `127.0.0.1` on an ephemeral port at
      `/oauth/callback`.
- [ ] The gateway API requests access-token version 2, and Stage B/C evidence
      proves both the manifest-based IP-literal redirect behavior and an
      enforceable maximum 15-minute gateway-token lifetime. Microsoft defaults
      are not treated as evidence.
- [ ] The gateway validates every trusted issuer, audience, signature,
      expiration, applicable tenant, and authorization context against closed
      server-owned configuration.
- [ ] Cloud hosting approval remains separate from identity and AI provider
      approval. Azure is the single initial planned primary cloud; AWS, Google
      Cloud, active-active multicloud, and cloud failover remain unapproved.
- [ ] Every AI model provider is separately approved for retention, ZDR, data
      use, logging, region, and security. No provider inherits another
      provider's approval, and desktop clients receive no provider credential.
- [ ] D-063 is treated as historical and superseded for synthetic evaluation:
      D-066/D-067 select OpenAI-through-Cloudflare only for the internal
      synthetic demo. Real-content provider/hosting remains separately
      unselected; the Azure items below remain future production-direction
      gates, not current v0 evidence.
- [ ] The gateway uses one dedicated non-shared user-assigned managed identity
      and only `Cognitive Services OpenAI User` at the exact Azure OpenAI
      resource scope; API-key fallback and broader runtime roles are absent.
- [ ] Azure OpenAI public network access is disabled and the gateway uses the
      approved private endpoint and restricted egress before synthetic or real
      provider traffic.
- [ ] The exact Azure resource reports `ContentLogging=false`, and the selected
      stateless Responses configuration has documented no application-state
      retention before real user content.
- [ ] D-061 provider-approved ZDR evidence is verified for the exact provider,
      production organization, project, endpoint, model, and region before real
      user content; pre-verification tests use only synthetic data.
- [ ] External-processing disclosure exists before the first transmission and
      remains visible in Settings.
- [ ] The current disclosure version is explicitly acknowledged before first
      transmission and after a material provider, retention,
      data-classification, or disclosure change.
- [ ] The desktop uses one configured HTTPS gateway origin and cannot override
      model, tools, schemas, credentials, or arbitrary provider parameters.
- [ ] The gateway authenticates and authorizes the principal, selects only an
      approved AI provider under trusted policy, and forces approved limits and
      provider settings.
- [ ] Provider events are normalized upstream and independently validated by
      trusted Rust with closed versions, variants, fields, sequence, and sizes.
- [ ] Cancellation is terminal and idempotent and rejects late events.
- [ ] Errors expose only closed codes, retryability, bounded delay, and opaque
      correlations.
- [ ] Gateway operational logs and local audit have separate ownership,
      retention, minimization, and access controls.

## Tool, policy, and approval boundary

- [ ] Tool identity, version, strict schema, risk, permission, and implementation
      come from the local registry.
- [ ] Unknown fields, duplicate keys, malformed JSON, invalid canonical values,
      unknown tools, and unknown versions fail closed.
- [ ] Policy consumes only trusted typed evidence and cannot be overridden by a
      model explanation or caller metadata.
- [ ] Approval binds exact canonical arguments, run/request/call/tool identity,
      preview, deadline, and one-time manager ownership.
- [ ] Approval rejection, cancellation, expiry, replay, and late outcomes are
      tested.
- [ ] Approval presentations, policy Allow results, resolutions, and audit
      receipts remain non-authorizing.
- [ ] Execution, if introduced, requires one exact registered implementation and
      revalidates target and current permission immediately before effect.

## SQLite and local data

- [ ] Schema changes use ordered versioned migrations with immutable checksums.
- [ ] Foreign keys, transactions, prepared values, busy timeout, and file-mode
      journal policy remain enabled and tested.
- [ ] No generic SQL crosses IPC and no untrusted identifier becomes SQL syntax.
- [ ] Sensitive persistence has approved encryption and key storage.
- [ ] Logs and audit do not duplicate raw prompts, arguments, results, arbitrary
      paths, or personal content.
- [ ] Retention, review, deletion, backup, upgrade, rollback, and corruption
      behavior are documented for new data.
- [ ] Synthetic tests contain no personal or production data.

## Filesystem and operating-system access

- [ ] New access is behind a narrow platform adapter and a user-initiated flow.
- [ ] Paths are scoped through handles or approved roots; traversal, symlink
      escape, unsupported schemes, aliases, and executable content fail closed.
- [ ] Accessibility, screen capture, Apple Events, microphone, broad filesystem,
      and automation permissions are absent unless a separately accepted threat
      model and increment approve them.
- [ ] Permission purpose, timing, denial, revocation, and degraded behavior are
      documented and tested.
- [ ] External communication, deletion, upload, purchase, booking, posting,
      messaging, and account changes remain outside the MVP unless explicitly
      approved.

## Audit and logging

- [ ] Audit records use normalized identities and closed outcomes.
- [ ] Audit receipts do not authorize or prove execution.
- [ ] Raw credentials, prompts, arguments, tool results, provider errors,
      headers, stack traces, and unnecessary personal content are excluded.
- [ ] Local audit and gateway logs have explicit retention, capacity, failure,
      access, and deletion behavior.
- [ ] Audit failure semantics are defined before a caller can receive a success
      value.

## Dependency and supply-chain review

- [ ] The capability cannot reasonably use the standard library or an existing
      dependency.
- [ ] Version is exact and manifests and lockfiles contain only expected changes.
- [ ] Maintenance, ownership, license, advisories, transitive dependencies,
      native code, build scripts, and platform impact are reviewed.
- [ ] JavaScript and Rust advisory results are recorded; exceptions have current
      accepted decisions and bounded exposure.
- [ ] Package install scripts and generated artifacts are reviewed.
- [ ] No dependency is added merely to avoid a small, reviewable implementation.

## GitHub workflow and repository automation

- [ ] Workflow triggers use `pull_request`, scoped `push`, scheduled audit, or
      explicit dispatch as intended; `pull_request_target` is absent.
- [ ] Top-level workflow permissions are read-only and checkout credentials do
      not persist.
- [ ] Workflows receive no secret context and contain no commit, push, merge,
      publish, deploy, signing, or auto-merge step.
- [ ] External actions use immutable commit digests and are reviewed through
      dependency proposals rather than mutable tags.
- [ ] Pull-request code is treated as untrusted even when checks pass.
- [ ] Persistent self-hosted workflows have no `pull_request` or
      `pull_request_target` trigger and accept only the trusted branch-push
      allowlist, schedule, or explicit dispatch defined by D-058.
- [ ] Path classification is deterministic, includes deletions, and fails
      closed by running both application jobs for unknown non-documentation
      paths.
- [ ] Dependency, workflow, Tauri, IPC, storage, migration, permission, and
      security-sensitive changes receive every affected application and audit
      check.
- [ ] Linux runner success is not substituted for required target-Mac native,
      signing, notarization, installer, or release evidence.
- [ ] Dependabot proposals remain human-reviewed and cannot write or merge to
      the default branch automatically.
- [ ] Secret-pattern and advisory scans fail closed on new findings; accepted
      baselines remain exact, documented, and independently reviewable.
- [ ] Accepted advisories remain visible in review evidence and have a named
      remediation owner or future increment; a passing baseline gate is not a
      claim that the dependencies are fixed.
- [ ] CODEOWNERS, labels, milestones, and badges are not represented as remote
      enforcement unless authenticated repository settings prove it.

## Release security

- [ ] The release uses a clean reviewed commit and complete valid increment
      evidence.
- [ ] Full tests, builds, dependency audits, and secret scans pass.
- [ ] Signing identity, private-key handling, Hardened Runtime, entitlements,
      notarization, stapling, Gatekeeper, and artifact hashes are verified.
- [ ] Installer, first launch, upgrade, rollback, and uninstall are tested on
      supported systems.
- [ ] Release notes disclose external processing, permissions, migrations,
      security changes, known limitations, and rollback.
- [ ] No Critical or High finding or required pending manual check remains.

## Review outcome

Record:

- changed trust boundaries and data flows;
- exact checklist items that are not applicable and why;
- findings by severity with path and evidence;
- commands and actual results;
- manual checks and target environment;
- residual risk, owner, and required follow-up;
- final PASS, PASS WITH ADVISORIES, or FAIL decision.
