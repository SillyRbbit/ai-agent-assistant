# D-107 exact signer binding contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Decision: D-110 proposed

## Goal

Decide only whether repository evidence establishes an immutable expected
Developer ID Application signer/certificate/public-key binding for D-107's
frozen conceptual candidate.

## Scope and non-goals

This documentation-only increment reviews only
`exact_signer_binding_contract` and records
`signer_binding_not_accepted`. It preserves D-097, D-107's 8/11 factual
record, D-108's 9/10 interpretation, and D-109's negative issuance result.
It changes only the approved fifteen documentation paths.

It performs no source, dependency, configuration, Keychain, certificate,
private-key, signing, Apple/Xcode, build, target-Mac, provider, product, or
external-system operation.

## Evidence and invariants

Current source establishes certificate/private-key correspondence only. It
contains no immutable expected Developer ID Application signer class or
certificate/public-key binding. Labels, fingerprints, default Keychain state,
search lists, filters, identity pairing, caller input, and ambient authority
are not substitutes for that binding.

`ExactSignerBindingPolicyV1` is conceptual only. Its closed governance
dispositions are `signer_binding_documented`,
`signer_binding_not_accepted`, and `boundary_failed`; they are not D-100
evidence or runtime values. No candidate or successor is admitted.

## Milestones

- [x] Clean baseline, approved branch, and active gate verified.
- [x] Repository-only review completed.
- [x] Closed negative disposition selected.
- [x] Documentation validation and completion workflow completed.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
```

Product verification, audit, builds, Apple/Xcode/Keychain/certificate/private-
key/signing/target-Mac/provider/product/external checks remain Not run by
scope.

## Final results

Completed with `PASS WITH ADVISORIES`. D-110 selects
`signer_binding_not_accepted`; no candidate or successor is Ready.
