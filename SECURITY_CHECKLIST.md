# Cortexa security checklist

Status: Authoritative change and release security review checklist
Last updated: 2026-07-16

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

- [ ] O-006 selects the exact approved Phase 1 identity provider or providers,
      issuer configuration, redirect handling, audience, and token-validation
      boundary before authentication or live gateway traffic.
- [ ] The gateway validates every trusted issuer, audience, signature,
      expiration, applicable tenant, and authorization context against closed
      server-owned configuration.
- [ ] Cloud hosting approval remains separate from identity and AI provider
      approval. Azure is the single initial planned primary cloud; AWS, Google
      Cloud, active-active multicloud, and cloud failover remain unapproved.
- [ ] Every AI model provider is separately approved for retention, ZDR, data
      use, logging, region, and security. No provider inherits another
      provider's approval, and desktop clients receive no provider credential.
- [ ] D-061 provider-approved ZDR evidence is verified for the exact provider,
      production organization, project, endpoint, model, and region before real
      user content; pre-verification tests use only synthetic data.
- [ ] External-processing disclosure exists before the first transmission and
      remains visible in Settings.
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
