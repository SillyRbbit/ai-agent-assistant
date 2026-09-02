# Personal Assistant v0 account-directory boundary planning

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-01
Increment: `personal-assistant-v0-account-directory-boundary-planning`
Predecessor: completed D-100 evidence-privacy protocol planning

## Goal

Define the smallest application-resolution policy and the separate proof gates
required before any future Personal Assistant v0 signing-security checker can
claim account-directory containment. Future trusted code must not explicitly
resolve an account, username, home directory, or Keychain filesystem path. This
increment documents policy and blockers only; it does not establish operating-
system containment, choose or call an Apple API, access a Keychain, rerun the
consumed query, or accept residual operating-system effects.

## User-visible outcome

A later operational plan can distinguish two facts that were previously mixed:
application code can prohibit explicit account-directory resolution, while the
absence of account-record acquisition or account/passwd/Open Directory/
directory-service resolution requires separate authoritative platform-contract
evidence and remains unproven today. Exact scope provenance is a third distinct
gate, while cache, log, socket, trust-service, process-metadata, and possible-
network effects each retain a one-predicate disposition. Unless every
application and effect predicate is satisfied, the checker stays unavailable.

## Scope

1. Define the conceptual
   `ExplicitAccountResolutionPolicyV1::Prohibited` policy for a future trusted
   platform adapter.
2. Prohibit explicit account and home resolution, account-derived paths,
   caller/environment-selected scope, directory-service clients, and fallback.
3. Require any future credential-store authority to be an in-process,
   operation-specific application capability over an adapter-private platform
   reference whose input, scope-provenance, and platform-effect contracts are
   separately reviewed before implementation.
4. Separate application-source guarantees from unproven OS-internal behavior;
   the application policy alone does not satisfy operational P2 containment.
5. Define P1-compatible categorical review evidence without collecting it.
6. Preserve the historical consumed-query finding as Manual verification
   pending and prohibit rerun.
7. Reconcile only the exact fifteen documentation paths listed below.

## Explicit non-goals

- No Apple, Xcode, Keychain, Security.framework, certificate, private-key,
  signing, build, notarization, credential, provider, model, network, product,
  target-Mac security/signing/product, or external-system operational work.
- No platform API selection, FFI, Rust, Python, shell, subprocess, Tauri, IPC,
  filesystem, environment, capability, permission, or source implementation.
- No account lookup, directory-service query, default-Keychain query, identity
  enumeration, path inspection, trust evaluation, tracing, traffic capture,
  logging inspection, or experimental probe.
- No acceptance, waiver, remediation, downgrade, deletion, or reclassification
  of the historical `getpwuid`/`opendirectoryd` finding.
- No P3 build-child containment, P4 immutable signer binding, operational
  signing, V0-3, or later product work.
- No branch, commit, push, merge, pull request, release, or publication.

## Existing behavior and constraints

- The consumed `keychain_identity_v1` wrapper called `pwd.getpwuid()` once to
  derive an account home. It must never run again under that approval.
- That lookup may have invoked `opendirectoryd`, materialized a full account
  record, consulted configured local or remote directory systems, and used
  OS-owned cache/socket/log state. No evidence proves remote traffic occurred.
- The wrapper emitted no account field, but the historical boundary was not
  separately disclosed or accepted and remains Manual verification pending.
- D-097 remains `failed` / `FAIL` / `Blocked` with no completion marker; its
  original report, digests, and findings remain unchanged.
- D-098/D-099/D-100 remain historical accepted governance decisions.
- P1 policy documentation is complete; no P1 parser, sanitizer, trusted local
  minimization boundary, consumer, or operational evidence path exists.
- P3/P4 and every operational successor remain Proposed/Blocked.

## Current-state evidence

- Baseline `HEAD` and `origin/main` are both
  `5bf37681a2554094575f17025f47b7cf2b6b5d36`, with ahead/behind `0/0` and a clean
  working tree.
- `git fsck --full --no-dangling` completed successfully.
- The ignored prior gate reports P1 complete, valid, and
  `PASS WITH ADVISORIES`.
- Repository-pinned toolchains are Node `26.3.0`, npm `11.16.0`, Cargo/Rust
  `1.90.0`; the read-only platform prerequisite completed, with no host detail
  retained as protocol evidence.
- Baseline `npm run docs:check` passed.
- No account-directory, Apple, Keychain, signing, network, or external query ran
  during this planning baseline.

## Files expected to change

Exactly:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PROJECT_STATUS.md`
8. `ROADMAP.md`
9. `SECURITY.md`
10. `SECURITY_CHECKLIST.md`
11. `TESTING_GUIDE.md`
12. `TROUBLESHOOTING_LOG.md`
13. `docs/increments/personal-assistant-v0-account-directory-boundary-planning.md`
14. this plan
15. `docs/reviews/2026-09-01-personal-assistant-v0-account-directory-boundary-planning-post-increment-review.md`

## Affected components

| Component                   | Effect                                                          |
| --------------------------- | --------------------------------------------------------------- |
| Repository governance       | Adds one application policy and input/provenance/effect gates.  |
| Security/privacy planning   | Prohibits explicit account resolution and path materialization. |
| Historical D-097 evidence   | None; Pending/Failed/Not-run states remain immutable.           |
| Product runtime and IPC     | None.                                                           |
| Target Mac/external systems | None.                                                           |

## Interfaces and invariants

### Conceptual future policy

```text
ExplicitAccountResolutionPolicyV1::Prohibited
```

This is a documentation invariant, not a serialized DTO or implemented type.

- Trusted application code owns the policy; no WebView, model, caller,
  environment value, command output, account record, runtime, or future API
  response can select or weaken it.
- The future platform adapter takes no account, user, home, path, Keychain,
  profile, runtime, task, run, workflow, or other caller-selected identity.
- It must not call or wrap account-database, passwd, Open Directory, directory-
  service, numeric UID/eUID-to-account, login/session/console-user, home or
  standard-directory, current/temporary/configuration-directory, search-list,
  or shell-expansion resolution or derivation.
- It must not read `HOME`, user/account environment variables, shell startup
  state, a caller-supplied path, or a constructed `/Users/<name>` path.
- It must not materialize or expose an account-home or Keychain filesystem path
  in argv, environment, logs, errors, events, evidence, or ordinary CI.
- A future credential-store authority must be an unexported, non-serializable,
  process-bound, operation-specific application capability wrapping only an
  adapter-private platform-issued reference. Trusted Rust owns the fixed policy
  and wrapper; the target-gated platform adapter privately owns the native
  reference and its lifecycle. It may not cross IPC, persistence, logs, or
  evidence and grants no generic enumerate, read, write, delete, sign, search-
  list, or filesystem authority.
- The capability is created in-process by one no-input trusted adapter only
  after the separately approved explicit action and is bound to one attempt.
  Cleanup is attempted on every terminal path. Success destroys the native
  reference; failure moves the application wrapper and adapter-private reference
  into private quarantine until process exit, blocks replacement, retry, and
  reuse, and never exposes or discards the reference early.
- Before implementation, authoritative static evidence must document three
  independent contracts: the selected API requires no application-supplied
  account identity or home/path resolution; the exact scope provenance is one
  fixed application credential domain rather than an ambient current-user,
  login-session, default-Keychain, default-search-list, current-directory, or
  environment selector; and the selected operation excludes account-record
  acquisition plus account/passwd/Open Directory/directory-service resolution.
  An absent, ambiguous, unsupported, or deprecated-without-a-supported-
  replacement contract produces only `contract_unproven`.
- The explicit application-resolution prohibition and the input and exact-
  scope contracts are non-waivable under D-101 and cannot substitute for one
  another. Operational P2 remains Blocked unless they are satisfied and every
  directory, cache, log, socket, trust-service, process-metadata, and network
  predicate has an exact disposition.
- A later separate exact decision and owner approval may disposition only one
  or more specifically disclosed OS-internal effect uncertainties. It cannot
  waive the application prohibition, permit caller/environment/account/home/
  path input, accept ambient/default scope, or authorize fallback, retry, or
  reuse.
- Static source review can establish only that application code contains no
  prohibited resolution path. It cannot prove the absence of undocumented
  OS-internal directory, cache, log, socket, trust-service, or network effects.
- Runtime observation, packet capture, process tracing, or an absence of visible
  effects cannot upgrade an undocumented platform contract to “contained.”
- There is no fallback to explicit owner acceptance. Any later residual-risk
  acceptance is a separate documentation decision and separate owner approval;
  it may not alter the historical finding or authorize a rerun.
- Unknown, ambiguous, prompt-producing, path-returning, account-returning, or
  directory-dependent behavior fails closed without retry.

### P1-compatible future review evidence

If a later separately approved documentation or implementation review emits
evidence, it uses `evidence_privacy_v1` with these predeclared check tables:

| `check_id`                            | Allowed outcomes                                                | Direct predicate                                                                                                              |
| ------------------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `prohibited_resolution_source_review` | `observed`, `not_observed`, `not_run`, `boundary_failed`        | Commit-bound complete adapter/reachable-helper/FFI/dependency source contains a prohibited explicit resolver.                 |
| `opaque_scope_input_contract`         | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract documents no application-supplied account, home, or path input.                                 |
| `opaque_scope_provenance_contract`    | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract binds one fixed application credential domain without an ambient/default selector.              |
| `platform_directory_effect_contract`  | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract excludes account-record acquisition and account/passwd/Open Directory/directory-service lookup. |
| `platform_cache_effect_contract`      | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract excludes any OS cache read or mutation caused by the selected operation.                        |
| `platform_log_effect_contract`        | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract excludes any OS log emission caused by the selected operation.                                  |
| `platform_socket_effect_contract`     | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract excludes any socket or IPC activity caused by the selected operation.                           |
| `platform_trust_effect_contract`      | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract excludes any trust evaluation or trust-service activity caused by the selected operation.       |
| `process_metadata_source_review`      | `observed`, `not_observed`, `not_run`, `boundary_failed`        | Commit-bound complete source/launch contract exposes an account/home/Keychain path through local process metadata.            |
| `platform_network_effect_contract`    | `documented`, `contract_unproven`, `not_run`, `boundary_failed` | Authoritative static contract excludes network traffic caused by the selected operation.                                      |
| `caller_selected_scope_source_review` | `observed`, `not_observed`, `not_run`, `boundary_failed`        | Commit-bound complete source accepts caller/environment/ambient/default-selected account scope.                               |
| `account_path_materialization_review` | `observed`, `not_observed`, `not_run`, `boundary_failed`        | Commit-bound complete source materializes an account-home/Keychain path.                                                      |

Each record remains non-authorizing and privately bound to one approved
plan/check/attempt as D-100 requires. Source review requires a fixed commit and
a complete predeclared adapter, reachable-helper, wrapper, FFI, and dependency
source inventory; an incomplete inventory returns `boundary_failed`, not
`not_observed`. `contract_unproven` is a non-authorizing predicate result;
private policy state, rather than the evidence token, keeps the successor
Blocked. No check may emit an account value, path, API output, trace, log,
packet, identifier, or explanatory free text. These tables are design evidence
only; every outcome is Not run in this increment.

## Implementation milestones

- [x] Confirm clean synchronized baseline, toolchains, prior gate, and
      documentation baseline.
- [x] Add D-101, the explicit application-resolution policy, and independent
      input, scope-provenance, and complete platform-effect proof gates.
- [x] Reconcile project-memory, security, testing, and troubleshooting records.
- [x] Run documentation-tier validation and independent reviews.
- [x] Record the post-increment review and finalize the gate if all evidence
      passes.

## Security and privacy considerations

| Threat                                            | Required control                                                      |
| ------------------------------------------------- | --------------------------------------------------------------------- |
| Future code repeats `getpwuid`/Open Directory     | Explicit resolution APIs and wrappers are prohibited.                 |
| Environment or caller chooses an account/path     | No-input application-owned policy and opaque capability only.         |
| No-input API hides ambient current-user authority | Require exact fixed application-domain provenance; otherwise block.   |
| Opaque capability becomes broad/ambient authority | Operation-specific, non-serializable, attempt-bound, never reused.    |
| Native cleanup fails                              | Retain private ownership until process exit; block replacement/reuse. |
| Account/Keychain path leaks through process state | No path construction, argv, environment, log, event, or evidence.     |
| One contract is mistaken for complete containment | Require all input, provenance, and one-predicate effect checks.       |
| OS internals are falsely described as contained   | Treat undocumented internal effects as unproven and block.            |
| Source review omits an indirect resolver          | Bind it to one commit and complete source/FFI/dependency inventory.   |
| Observation is mistaken for universal absence     | Static contract/source evidence only; observation cannot upgrade.     |
| Owner acceptance silently replaces containment    | Separate plan and approval; no fallback or inferred acceptance.       |
| Historical Pending result is rewritten            | Preserve D-097 report/digests/findings and no completion marker.      |
| Closed evidence smuggles account detail           | D-100 exact categories, source minimization, no raw/free-text data.   |

## Test plan

- Inspect the complete diff and confirm it matches the exact fifteen paths.
- Confirm D-097/D-098/D-099/D-100 and every historical
  Failed/Pending/Not-run fact remain unchanged.
- Table-review accepted policy cases:
  - no-input application-owned `Prohibited` policy;
  - opaque non-path input contract documented by authoritative static evidence;
  - exact fixed application-domain scope provenance separately documented;
  - platform contract separately excludes account-record acquisition and
    account/passwd/Open Directory/directory-service lookup;
  - cache, log, socket, trust-service, process-metadata, and possible-network
    predicates each separately satisfy their exact closed review;
  - prohibited resolution source reference `not_observed`;
  - caller/environment-selected scope `not_observed`;
  - account-path materialization `not_observed`;
  - `boundary_failed` available for every check.
  - all unique check IDs and all outcome tokens match `[a-z][a-z0-9_]*`; check
    IDs are at most 64 bytes, outcomes at most 32 bytes, and the longest
    canonical three-field record remains at most 256 bytes.
- Table-review rejected policy cases:
  - `getpwuid`, `getpwnam`, passwd/account database, Open Directory, directory-
    service, numeric UID/eUID-to-account, login/session/console-user, home/
    standard/current/temporary/configuration-directory, search-list, shell
    expansion, environment home, or constructed account path resolution;
  - caller-, WebView-, model-, environment-, runtime-, or output-selected scope;
  - path string, subprocess, argv/environment path, search-list enumeration,
    generic filesystem access, or fallback resolver;
  - any input, scope-provenance, or directory-effect contract undocumented,
    ambiguous, unsupported, or deprecated without a supported replacement;
  - any cache, log, socket, trust-service, process-metadata, or network predicate
    omitted, combined into a broad claim, or left unproven;
  - ambient current-user/login-session/default-Keychain/default-search-list/
    current-directory authority or a scope broader than one fixed application
    credential domain;
  - serialized, reusable, cross-attempt, logged, persisted, IPC-visible, or
    generic enumerate/read/write/delete/sign-capable wrapper;
  - cleanup failure followed by replacement, retry, reuse, exposure, or early
    ownership loss rather than private quarantine until process exit;
  - source review without a fixed commit and complete reachable source/FFI/
    dependency inventory;
  - runtime observation, trace, log, packet capture, or “nothing visible” used
    as proof of no OS-internal effect;
  - check table missing `boundary_failed`, unknown category, raw/free-text
    evidence, retry, or inferred owner acceptance.
  - duplicate, non-ASCII, uppercase, malformed, over-64-byte check ID, over-32-
    byte outcome, or over-256-byte canonical record.
- Run documentation formatting/link, repository-health, secret-scan,
  protected-path, whitespace, session-end, and post-increment checks.
- Account-directory, Apple, Xcode, Keychain, certificate, signing, build,
  credential, provider, network, and product-runtime checks are Not run.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Manual gates

- Confirm the future policy takes no caller/environment/account/path input and
  has no fallback.
- Confirm the input, exact scope-provenance, directory, cache, log, socket,
  trust-service, process-metadata, and network predicates are independent
  mandatory gates and none is represented as proof of another.
- Confirm application-source guarantees are not represented as proof of
  undocumented OS-internal behavior.
- Confirm all P1 review categories are closed, non-authorizing, and contain
  `boundary_failed`.
- Confirm all static check/outcome tokens and the longest canonical record meet
  D-100 grammar, uniqueness, and 64/32/256-byte limits.
- Confirm the consumed wrapper was not rerun and the historical finding remains
  Manual verification pending.
- Confirm no account-directory, Apple, Keychain, signing, network, or external
  operation occurred.
- Confirm P3/P4 and every operational successor remain Proposed/Blocked.

## Dependencies

- Accepted D-100 `evidence_privacy_v1` documentation policy.
- Existing repository documentation/gate workflows only.
- A future operational implementation remains blocked on the authoritative
  input and exact-provenance contracts, every one-predicate historical effect
  disposition, and a separately approved exact plan.
- No new dependency, external account, service, credential, or tool.

## Risks

- “No explicit resolver” cannot prove the OS implementation never consults a
  directory service.
- A future opaque platform handle may lack an authoritative supported contract
  and therefore remain unavailable.
- A later plan could reintroduce path authority or fallback unless its source
  and tests enforce this policy.
- Documentation cannot retroactively complete historical risk acceptance.

## Rollback or failure strategy

If validation fails, correct only these fifteen documentation paths. If a
truthful passing closeout is impossible, record a normal terminal failure; do
not change predecessor evidence or broaden scope. A future rollback reverts
only this documentation increment and leaves D-097/D-098/D-099/D-100 intact.

## Stop conditions

Stop on any query, probe, API call, trace, packet capture, Apple/Keychain/signing
operation, raw evidence, file outside the declared scope, new dependency,
executable implementation, external research/action, residual-risk acceptance,
or claim that application-source review proves undocumented OS-internal
absence. Also stop on dirty/divergent Git state not caused by this increment or
any failed security/gate check that cannot be resolved in the exact
documentation scope.

## Decisions made

- D-101 defines `ExplicitAccountResolutionPolicyV1::Prohibited` as an accepted
  application-level documentation policy, requires independent input, scope-
  provenance, and complete effect-suite proof gates, and rejects inferred owner-
  acceptance fallback.

## Discoveries

- The application can prohibit explicit account resolution without claiming
  control over undocumented operating-system internals.
- An opaque no-input capability is only an application-input prerequisite, not
  evidence of exact authority, an acceptable API, or contained platform
  effects.

## Progress

- 2026-09-01: Owner approved the P2 documentation-only planning increment.
- 2026-09-01: Began from clean synchronized `main` at `5bf3768` after the
  documentation baseline passed.
- 2026-09-01: Independent inventory, architecture, security, code-health,
  technical-debt, and readiness reviews completed. The policy was narrowed to
  application resolution, exact scope provenance was separated, the complete
  historical effect suite gained one-predicate checks, and the current fake-
  only Keychain reader inventory was reconciled.
- 2026-09-01: Documentation, repository, secret, protected-path, whitespace,
  session-end, and post-increment checks passed. Operational checks remained
  Not run by design.

## Acceptance criteria

- [x] Explicit account/directory/home/path resolution and fallback are
      prohibited.
- [x] The future application capability is operation-specific, opaque,
      no-input, in-process, non-path-based, non-serializable, and attempt-bound;
      terminal cleanup is always attempted, and failure retains private
      ownership until process exit while blocking replacement/retry/reuse.
- [x] Independent input, exact scope-provenance, directory, cache, log, socket,
      trust-service, process-metadata, and network predicates are required; none
      substitutes for another.
- [x] Static application guarantees are separated from unproven OS-internal
      effects.
- [x] Platform-contract ambiguity blocks rather than falling back or inferring
      acceptance.
- [x] P1-compatible review tables are closed, non-authorizing, and include
      `boundary_failed`.
- [x] Historical D-097 and Pending Open Directory evidence remain unchanged;
      the consumed query is not rerun.
- [x] P3/P4 and every operational successor remain Proposed/Blocked.
- [x] Exact documentation checks and the post-increment gate pass.

## Final results

`PASS WITH ADVISORIES`. D-101 and the closed P1-compatible review tables are
complete documentation. No supported platform API, authoritative contract
evidence, parser/sanitizer, adapter, native capability, target-Mac behavior, or
runtime test exists. Every operational P2 predicate is Not run; operational P2,
P3/P4, signing, product, and external work remain Blocked.

## Documentation updates

- [x] `ARCHITECTURE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md`
- [x] `HANDOFF.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `PROJECT_STATUS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `TESTING_GUIDE.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] Increment record and post-increment review
