# Cortexa security checklist

Status: Authoritative change and release security review checklist
Last updated: 2026-08-13

Use this checklist with `SECURITY.md`. Mark an item not applicable only with a
short reason grounded in the actual diff. A plan or test fixture does not prove a
production boundary exists.

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
- [ ] D-063's Azure OpenAI candidate is implemented only with an exact approved
      Standard/Regional Central US deployment, managed identity,
      least-privilege RBAC, foreground Responses, disabled storage and
      background mode, strict custom functions, and no automatic provider
      fallback.
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
