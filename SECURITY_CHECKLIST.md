# Cortexa security checklist

Status: Authoritative change and release security review checklist
Last updated: 2026-08-12

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
