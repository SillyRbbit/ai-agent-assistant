# D-107 exact signer binding contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Baseline: `cc4f8b41638434977d4a038b8aa5d027f4964fb0`
Predecessor: D-109 (`d107-opaque-prebound-identity-contract-decision`)

## Goal

Perform one repository-only, documentation-only source review of the second
remaining D-107 blocker, `exact_signer_binding_contract`. Decide only whether
accepted repository records establish a future immutable expected Developer ID
Application signer class and certificate/public-key binding for the frozen
`in_process_security_framework_ephemeral_challenge_proof_v1` candidate.

The decision must fail closed. It cannot infer a signer from an opaque identity,
fixed label, certificate subject, fingerprint, default Keychain, search list,
environment, account, path, or caller-provided value.

## User-visible outcome

None. This is documentation planning only. It does not inspect a certificate,
access Keychain, use a private key, sign data, build the product, or change
application behavior.

## Scope

1. Re-read D-096 through D-109 and signer-adjacent current source using
   repository files only.
2. Define one future immutable expected-signer contract without adding a signer
   selector, configuration source, persistence record, or runtime interface.
3. Define only these closed governance dispositions:
   `signer_binding_documented`, `signer_binding_not_accepted`, and
   `boundary_failed`.
4. Preserve D-097, D-107's immutable factual 8/11 record, D-108's additive
   prospective 9/10 interpretation, and D-109's negative identity-issuance
   result.
5. Leave readiness Blocked regardless of the selected disposition.

## Explicit non-goals

- No product, Rust, TypeScript, test, dependency, lockfile, workflow, hook,
  script, capability, CSP, permission, entitlement, or configuration change.
- No Apple documentation research, Apple account/Xcode/Keychain/certificate/
  private-key/signing/build/target-Mac operation, provider, network, credential,
  product, or external-system action.
- No certificate, public key, serial, issuer, subject, fingerprint, team,
  account, label, path, host, raw error, or target-derived value in repository
  documentation, evidence, tests, logs, or ordinary CI.
- No caller-, model-, WebView-, environment-, runtime-, profile-, task-, run-,
  workflow-, agent-, account-, home-, path-, default-, search-list-, or
  fallback-selected signer.
- No claim that certificate/private-key pairing establishes immutable expected
  signer identity, Developer ID class, private-key custody, prompt denial,
  cancellation, cleanup, or platform-effect bounds.
- No branch creation, gate start, commit, push, merge, release, or publication.

## Existing behavior and constraints

- D-107 records `exact_signer_binding_contract` as `contract_unproven`:
  identity pairing proves certificate/key correspondence only; no immutable
  expected Developer ID Application class and certificate/public-key binding
  exists.
- D-109 records `reference_issuance_not_accepted`: there is no current
  application-owned issuer for the prerequisite opaque identity reference.
  Signer binding cannot repair that absence or use an alternate identity route.
- D-101 prohibits explicit account/home/path resolution and ambient/default
  identity authority. D-108 decides only D-102's build-child subject and grants
  no signer, identity, or platform-effect authority.
- Existing Cloudflare credential code is a separate fixed-label credential
  reader. It is neither a Developer ID signer source nor a binding record.

## Current-state evidence

- Clean synchronized `main`, `HEAD`, and `origin/main` resolved to
  `cc4f8b41638434977d4a038b8aa5d027f4964fb0` when this plan was drafted.
- D-109's gate is valid and complete with `PASS WITH ADVISORIES`.
- Current D-107 source evidence establishes an identity's certificate/private-
  key correspondence, but no expected Developer ID Application class or
  immutable certificate/public-key binding.
- No signer-derived or target-derived evidence has entered the repository.

## Files expected to change

The exact documentation inventory is fifteen paths: this plan; one increment
record; one completion review; `DECISIONS.md`; and
`ARCHITECTURE.md`, `CHANGELOG.md`, `HANDOFF.md`, `NEXT_STEPS.md`,
`PLANS.md`, `PROJECT_STATUS.md`, `ROADMAP.md`, `SECURITY.md`,
`SECURITY_CHECKLIST.md`, `TESTING_GUIDE.md`, and
`TROUBLESHOOTING_LOG.md`. No other path may change.

## Affected components

| Component               | Effect                                                           |
| ----------------------- | ---------------------------------------------------------------- |
| D-107 candidate         | Clarifies only immutable expected-signer binding.                |
| D-109 identity issuance | Remains independently negative and controlling.                  |
| D-101 / D-108           | Retain scope and build-child constraints; no authority transfer. |
| Product/runtime         | No behavior, interface, or authority change.                     |
| Readiness               | Remains Blocked.                                                 |

## Interfaces and invariants

`ExactSignerBindingPolicyV1` is a future documentation contract, not a Rust,
Tauri, WebView, or persistence interface. A future positive design would need
separate approved evidence that trusted Rust owns one immutable, application-
owned expected signer policy before an attempt begins; a private adapter can
compare only fixed application-owned signer material without exposing it; and
no input or ambient lookup selects, replaces, refreshes, or falls back to a
signer.

The governance labels are not D-100 evidence outcomes, runtime values, or
authorization. Missing, ambiguous, contradictory, non-immutable, or drifted
facts select `boundary_failed`. A repository record that lacks the complete
binding selects `signer_binding_not_accepted`. Neither outcome admits a
candidate, supplies an identity issuer, or relaxes any other D-107 contract.

## Implementation milestones

- [x] Obtain approval, create the branch, and begin the exact documentation
      gate.
- [x] Reconcile repository-only facts against D-096 through D-109.
- [x] Record `signer_binding_not_accepted` without private or target-derived
      signer data.
- [x] Run documentation validation and the completion workflow.
- [x] Stop with readiness Blocked and no operational successor.

## Security and privacy considerations

The primary threat is signer laundering: treating correspondence between a
certificate and a private key as proof that it is the expected Developer ID
Application signer. Additional threats are user/caller selection, certificate
metadata disclosure, persistent identifier storage, label/fingerprint filters,
ambient Keychain/default-search behavior, fallback/retry, and a later
substitution attack.

The proposed review avoids these effects by using repository-only records,
closed dispositions, no signer values, and independent D-109/D-101 constraints.
It does not claim that a positive static contract could prove private-key
non-export, interaction denial, cancellation, cleanup, or OS-managed effects.

## Test plan

- Compare the proposed contract with D-107's exact signer-binding reason and
  D-109's negative identity-issuance result.
- Confirm fixed labels, certificate metadata, fingerprints, and identity pairing
  are not treated as immutable expected-signer proof.
- Confirm D-097, D-107, D-108, D-109, and all historical reports stay
  unchanged.
- Confirm the final approved increment contains only documentation and no
  signer-derived, account, credential, certificate, key, or target data.
- Confirm missing source facts fail closed.

## Verification commands

The future approved increment must run:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

No product verification, audit, build, certificate, Keychain, signing, or
target-Mac command is authorized by this planning draft.

## Risks

- Existing repository evidence is insufficient. Record a closed negative or
  failed disposition; do not add external research silently.
- A future policy abstraction could be misread as a live signer registry.
  Keep it conceptual and non-serializable.
- Scope could expand into certificate inspection or signing. Stop and require
  a distinct approval.

## Rollback or failure strategy

Before publication, remove this uncommitted plan only with owner direction.
After a future approved gate begins, preserve all evidence and record a closed
negative or failed result; never rewrite D-107 through D-109. No fallback,
retry, or operational action follows a negative result.

## Decisions made

- 2026-09-02: Selected `signer_binding_not_accepted`; the repository has no
  immutable expected Developer ID signer/certificate/public-key binding.
- 2026-09-02: Sources are restricted to the repository. Any need for
  certificate, Keychain, Apple, account, or target-Mac evidence stops the
  proposed increment.

## Discoveries

- Current identity pairing establishes only correspondence; it does not bind a
  future proof to an immutable expected Developer ID Application signer.
- D-109's negative issuance result prevents using ambient lookup or a distinct
  credential reader as a substitute signer source.

## Progress

- 2026-09-02: Drafted from clean synchronized `main` at
  `cc4f8b41638434977d4a038b8aa5d027f4964fb0`.
- 2026-09-02: Owner approved the exact increment; branch
  `codex/d107-exact-signer-binding-contract-decision` was created and the
  gate began. The repository-only review selected the closed negative result.

## Acceptance criteria

- [x] Owner approved the exact documentation increment before branch and
      gate creation.
- [x] One closed signer-binding disposition is selected.
- [x] D-097 through D-109 and all remaining blockers are preserved.
- [x] No signer-derived, operational, or external data/action occurs.
- [x] No operational successor becomes Ready.

## Final results

Completed as a documentation-only negative decision. D-110 selects
`signer_binding_not_accepted`; no candidate or successor is Ready. Required
checks passed with `PASS WITH ADVISORIES`.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
