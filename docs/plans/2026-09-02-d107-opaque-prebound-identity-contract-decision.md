# D-107 opaque prebound identity contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Baseline: `51a80f60cf8d803b3c22945d0719a671382ee090`
Predecessor: D-108 (`d102-non-build-proof-applicability-decision`)

## Goal

Perform one repository-only, documentation-only source review of the first
remaining D-107 blocker, `opaque_prebound_identity_contract`, for the frozen
conceptual `in_process_security_framework_ephemeral_challenge_proof_v1`
candidate. Decide only whether current accepted repository contracts can define
a future no-input, application-owned, attempt-bound opaque identity-reference
issuance boundary without lookup, enumeration, selection, fallback,
persistence, or IPC exposure.

The review fails closed. It cannot claim that an identity exists, can be
obtained, is safe to use, or is ready to implement.

## User-visible outcome

None. This is governance documentation planning only. It does not enable
signing, inspect Keychain state, use a certificate or private key, run a
cryptographic proof, or change application behavior.

## Scope

1. Re-read D-096 through D-108 and current Rust credential/signing-adjacent
   source inventory using repository files only.
2. Draft one bounded decision increment for future opaque identity-reference
   ownership and provenance, not identity use.
3. Define only three closed governance dispositions:
   `reference_issuance_documented`, `reference_issuance_not_accepted`, and
   `boundary_failed`.
4. Preserve D-107's immutable factual record and its exact
   `contract_unproven` outcome. A later positive governance decision can add
   only a prospective current-state row; it cannot admit the candidate or make
   a successor Ready.
5. Synchronize only project-memory documents that an approved and completed
   decision actually changes, with readiness still Blocked.

## Explicit non-goals

- No product, Rust, TypeScript, test, dependency, lockfile, workflow, hook,
  script, capability, CSP, permission, entitlement, or configuration change.
- No Keychain, Security framework, certificate, private key, account, home,
  path, Apple, Xcode, signing, build, provider, network, credential, device,
  target-Mac, or external-system operation.
- No external source research. A need for Apple or other evidence stops the
  increment rather than expanding it.
- No caller-selected identity, certificate, key, label, fingerprint, account,
  path, runtime, profile, agent, task, run, workflow, algorithm, challenge, or
  result, and no generic signing or identity-selection interface.
- No claim that `SecIdentity` opacity proves provenance, scope, interaction
  denial, key non-export, cancellation, cleanup, or absent platform effects.
- No readiness promotion, implementation authorization, branch creation, gate
  start, commit, push, merge, release, or publication.

## Existing behavior and constraints

- D-107 records `opaque_prebound_identity_contract` as
  `contract_unproven`: no current component issues the required no-input,
  application-domain, attempt-bound reference without lookup, enumeration, or
  fallback.
- D-108 changes only D-102's build-child applicability for the exact frozen
  non-build class. It leaves this identity contract and all other D-107
  blockers controlling.
- D-101 prohibits explicit account, home, path, default/search-list, and
  ambient-current-user resolution. Any future identity boundary must remain in
  an application-owned, unexported, non-serializable, operation-specific,
  attempt-bound wrapper; a private adapter owns destruction and quarantine.
- `src-tauri/src/credentials/cloudflare_access.rs` reads two fixed generic
  password labels for a distinct Cloudflare credential proof. It is not an
  identity-issuance, certificate-binding, or signing authority.
- The ten current blockers remain conjunctive: opaque reference issuance,
  exact signer binding, account/Keychain scope, private-key non-export, fixed
  algorithm, interaction denial, deadline/cancellation, late-result rejection,
  cleanup/quarantine, and platform effects.

## Current-state evidence

- Clean synchronized `main`, `HEAD`, and `origin/main` resolved to
  `51a80f60cf8d803b3c22945d0719a671382ee090` before this plan was drafted.
- The D-108 post-increment gate is valid and complete with `PASS WITH
ADVISORIES`; it did not make a successor Ready.
- D-107's immutable record is `documented=8`,
  `contract_unproven=11`, `not_run=0`, and `boundary_failed=0`, with
  result `not_eligible_or_unproven`.
- D-108's additive prospective classification is `documented=9` and
  `contract_unproven=10`; opaque prebound identity remains unproved.

## Files expected to change

The exact documentation inventory is:

1. `DECISIONS.md` — one additive durable decision, if supported.
2. `docs/increments/d107-opaque-prebound-identity-contract-decision.md`.
3. `docs/reviews/<date>-d107-opaque-prebound-identity-contract-decision-post-increment-review.md`.
4. `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `PLANS.md`,
   `ROADMAP.md`, `SECURITY.md`, `SECURITY_CHECKLIST.md`,
   `TESTING_GUIDE.md`, `CHANGELOG.md`, and `TROUBLESHOOTING_LOG.md` only
   where observed result requires an additive current-state update.
5. `ARCHITECTURE.md`
6. `CHANGELOG.md`
7. `HANDOFF.md`
8. `NEXT_STEPS.md`
9. `PLANS.md`
10. `PROJECT_STATUS.md`
11. `ROADMAP.md`
12. `SECURITY.md`
13. `SECURITY_CHECKLIST.md`
14. `TESTING_GUIDE.md`
15. `TROUBLESHOOTING_LOG.md`

No other path may change.

No product or configuration path is eligible. An unexpected path is a stop
condition.

## Affected components

| Component                  | Effect                                                                          |
| -------------------------- | ------------------------------------------------------------------------------- |
| D-107 conceptual candidate | Clarifies only its still-unproved opaque-reference ownership boundary.          |
| D-101 policy               | Remains controlling; no account/home/path/default/search-list route is allowed. |
| D-108 split                | Remains build-child-only and supplies no identity authority.                    |
| Product/runtime            | No behavior or authority change.                                                |
| Roadmap/readiness          | Remains Blocked; no operational successor becomes Ready.                        |

## Interfaces and invariants

`OpaquePreboundIdentityReferencePolicyV1` is a future documentation contract,
not an implemented Rust, Tauri, or WebView interface. A future implementation
could be considered only after separate decisions and approvals establish:

1. Trusted Rust creates one fresh, attempt-private context after explicit
   foreground user action; no caller owns trusted selection.
2. A private adapter obtains or is handed exactly one application-domain,
   operation-specific opaque native reference without selector input or lookup,
   enumeration, search-list, default-Keychain, account, home, path,
   environment, profile, runtime, or fallback selection.
3. The wrapper is unexported, non-cloneable, non-serializable, cannot cross
   Tauri commands/events, logs, errors, evidence, persistence, or tests, and
   cannot be reused across attempts.
4. The adapter destroys a successful reference and retains failed/cancelled
   reference state only in adapter-owned quarantine until process exit, with no
   replacement, retry, reuse, early ownership loss, or caller visibility.
5. Every other D-107 contract is independently proved under separately
   approved work. No inference across rows is permitted.

The three disposition labels are human-readable governance labels, not D-100
`evidence_privacy_v1` outcome tokens and not operational results. Missing,
ambiguous, contradictory, or drifted facts select `boundary_failed`. A
negative source finding selects `reference_issuance_not_accepted`. Neither
non-failure disposition grants implementation authority.

## Implementation milestones

- [x] Obtain approval, create the branch, and run the exact gate.
- [x] Reconcile repository-only source facts against D-096 through D-108.
- [x] Record `reference_issuance_not_accepted` and preserve D-107 historical
      evidence.
- [x] Run documentation validation and required completion workflow.
- [x] Stop with readiness Blocked and no operational successor.

## Security and privacy considerations

The primary threat is provenance laundering: calling an opaque platform object
“prebound” without proving who selected it, its application domain, or whether
hidden lookup/default authority occurred. Other threats are ambient Keychain
authority, selector input, reference escape through DTOs/logs, reuse after
cancellation, documentation presented as runtime proof, and exposure of
account/certificate/key metadata.

Repository-only evidence, closed no-input constraints, independent contracts,
immutable historical records, and `boundary_failed` prevent those failures.
OS-managed Keychain, `securityd`, cache, log, IPC, trust, revocation,
process-metadata, and possible network effects remain unproved and out of
scope.

## Test plan

- Compare proposed constraints with every D-101 prohibited input and D-107
  remaining blocker.
- Confirm no implementation claim and no classification of the Cloudflare
  credential reader as identity proof.
- Confirm D-097, D-107, D-108, existing plans, increments, and reviews remain
  unchanged.
- Confirm any approved future increment changes only its approved
  documentation inventory and contains no sensitive values or operation log.
- Confirm missing source facts yield `boundary_failed`, not narrative or
  partial success.

## Verification commands

The future approved documentation increment must run:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

This planning draft may run only the first four checks. It must not call
`post_increment_gate.py begin` until separately authorized.

## Risks

- Repository evidence may be insufficient. Record a closed negative or failed
  disposition; do not add external research silently.
- The wrapper may be mistaken for implementation. It is explicitly conceptual
  and no source/interface changes.
- Governance labels may be confused with D-100 evidence. They are expressly
  non-serializable governance terms.
- Broader signing, key, provider, filesystem, IPC, or runtime work could be
  bundled in. Stop and require a separate increment.

## Rollback or failure strategy

Before publication, remove this uncommitted draft only with owner direction if
it proves unsuitable. After an approved increment begins, preserve all evidence
and record the closed negative or failed disposition; never rewrite D-107/D-108
records. A negative or failed result grants no retry or operational fallback.

## Decisions made

- 2026-09-02: Selected `reference_issuance_not_accepted`: current repository
  source contains no application-owned issuer of the required reference.
- 2026-09-02: Sources are repository-only. Any need to inspect Apple,
  Keychain, account, signing, or target-Mac state stops the future increment.

## Discoveries

- D-108 removed only the build-child applicability blocker for the frozen
  non-build candidate; it supplies no identity, signer, or platform-effect
  proof.
- The existing Cloudflare credential reader has fixed labels but is a separate
  credential read; it does not issue or bind a signing identity.

## Progress

- 2026-09-02: Drafted from clean synchronized `main` at
  `51a80f60cf8d803b3c22945d0719a671382ee090`.
- 2026-09-02: Owner approved the exact increment; branch
  `codex/d107-opaque-prebound-identity-contract-decision` was created and
  its gate began. Repository-only review selected the closed negative
  disposition. No source, external operation, or readiness promotion occurred.

## Acceptance criteria

- [x] The owner approved exact increment scope before branch and gate.
- [x] The decision selected exactly one closed disposition.
- [x] D-107's original record, D-108's split, D-097's failure, and all
      remaining blockers are preserved.
- [x] No source/system/credential/certificate/private-key/account/external
      operation occurs.
- [x] Completion leaves no operational successor Ready.

## Final results

Completed as a documentation-only negative decision. D-109 selects
`reference_issuance_not_accepted`; D-107 remains unproved and no successor is
Ready. Required checks passed and the completion result is
`PASS WITH ADVISORIES`.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
