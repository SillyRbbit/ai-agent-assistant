# D-107 opaque prebound identity contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Decision: D-109 proposed

## Goal

Decide, from repository evidence only, whether current accepted contracts can
establish a future application-owned issuer of one no-input, attempt-bound
opaque identity reference for the frozen D-107 conceptual candidate.

## User-visible outcome

None. This documentation-only decision does not create an identity source,
signing operation, Keychain access, or product behavior.

## Scope

- Review only `opaque_prebound_identity_contract`.
- Record the closed negative governance disposition
  `reference_issuance_not_accepted`.
- Preserve D-097, D-107's 8/11 factual record, and D-108's additive 9/10
  interpretation.
- Update only the exact fifteen documentation paths in the approved ExecPlan.

## Explicit non-goals

No source, dependency, configuration, Keychain, Security framework,
certificate, private-key, Apple, Xcode, signing, build, target-Mac, provider,
credential, network, product, or external-system operation; no branch beyond
this approved documentation branch; no commit, push, merge, release, or
publication.

## Existing behavior and constraints

D-107 is a conceptual childless/fileless data-signature candidate, not an
implementation. D-101 prohibits account/home/path/default/search-list identity
resolution. The current Cloudflare credential reader is a separate fixed-label
credential proof, not an identity issuer. D-108 does not establish identity
authority.

## Current-state evidence

The owner approved the increment from clean synchronized `main` at
`51a80f60cf8d803b3c22945d0719a671382ee090`. The dedicated branch was
created and `post_increment_gate.py begin` reported this exact increment as
active. Repository source contains no component that creates or owns an
application-domain, attempt-bound opaque signing-identity reference without
lookup, enumeration, selection, or fallback.

## Files expected to change

Exactly the fifteen paths listed in the approved ExecPlan.

## Interfaces and invariants

`OpaquePreboundIdentityReferencePolicyV1` remains conceptual only. Its three
governance dispositions are `reference_issuance_documented`,
`reference_issuance_not_accepted`, and `boundary_failed`; they are not
D-100 evidence tokens or runtime results. No caller-controlled identity input,
generic selector, generic signing interface, or reference export is permitted.

## Implementation milestones

- [x] Clean synchronized baseline, branch, and gate verified.
- [x] Repository-only review completed.
- [x] Closed negative disposition selected.
- [x] Documentation validation and completion reviews completed.

## Security and privacy considerations

Opacity does not prove provenance. A hidden ambient Keychain/default/search
selection would violate D-101 even if the returned native object were opaque.
Account, certificate, key, label, fingerprint, and native error data remain
out of scope. Platform effects remain independently unproved.

## Test plan

Review D-101 input prohibitions and all remaining D-107 blockers; preserve
historical records; confirm the exact documentation-only diff; and run the
approved documentation validation and completion workflow.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
```

The completion workflow additionally runs its required session, independent
review, and gate checks. Product, build, signing, Keychain, Apple/Xcode,
target-Mac, provider, and external-system checks are Not run by scope.

## Risks

Calling an opaque object “prebound” without issuance proof would launder
ambient authority. Selecting a negative disposition cannot establish
impossibility beyond the reviewed repository state.

## Rollback or failure strategy

Preserve all historical records. If validation fails, record a truthful failed
gate result; do not add an operational fallback or rerun a system operation.

## Decisions made

D-109 selects `reference_issuance_not_accepted` because no present repository
component supplies the required issuer. The D-107 factual outcome remains
`contract_unproven`; no candidate is admitted.

## Discoveries

The existing fixed-label Cloudflare reader proves neither signer provenance nor
an opaque signing-identity issuance path.

## Progress

- 2026-09-02: Began approved documentation-only increment.
- 2026-09-02: Completed repository-only negative source review.

## Acceptance criteria

- [x] Exactly one D-107 blocker is reviewed.
- [x] A closed negative disposition preserves all historical evidence.
- [x] Required validation and gate complete with `PASS WITH ADVISORIES`.
- [x] No operational or external work occurred.

## Final results

Completed with `PASS WITH ADVISORIES`. D-109 selects
`reference_issuance_not_accepted`; no candidate or successor is Ready.

## Documentation updates

See the approved ExecPlan.
