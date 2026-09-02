# D-107 fixed algorithm contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Baseline: `238fc5a68e42f88c6b5c2e96c2193240f2fc5e14`
Predecessor: D-112 (`d107-private-key-nonexport-contract-decision`)

## Goal

Perform one repository-only, documentation-only review of
`fixed_algorithm_contract`. Decide only whether current records establish one
immutable permitted Developer ID key type and signature algorithm, with no
caller selection, runtime substitution, algorithm-support fallback, or retry.

## Scope

1. Re-read D-096 through D-112 and cryptography-adjacent repository source.
2. Define only `algorithm_contract_documented`,
   `algorithm_contract_not_accepted`, and `boundary_failed`.
3. Preserve D-097, D-107's 8/11 record, D-108's 9/10 interpretation, D-109
   through D-112, all ten blockers, and Blocked readiness.

## Explicit non-goals

No source/dependency/configuration/test/workflow/hook/capability/permission
change; no Keychain, certificate, private-key, signing, algorithm preflight,
Apple/Xcode, build, target-Mac, provider, product, network, or external action;
no algorithm/key/certificate/identity metadata; no branch, gate, commit, push,
merge, release, or publication.

## Evidence and invariants

D-107 records this contract unproved: the exact Developer ID key type and
permitted algorithm are not frozen, and the safe crate lacks a safe
algorithm-support preflight wrapper. A future
`FixedAlgorithmPolicyV1` is documentation only: trusted Rust would own one
fixed algorithm/key-type policy before an attempt; no input, query, substitute,
fallback, retry, or error-dependent choice may select it. The labels are not
D-100 evidence or runtime values. Missing, ambiguous, contradictory, or drifted
facts select `boundary_failed`; absent complete proof selects
`algorithm_contract_not_accepted`.

## Files, risks, and validation

Any separately approved increment must use an exact documentation-only scope:
this plan, increment record, review, `DECISIONS.md`, and applicable
architecture/security/project-memory documents; any other path stops it. The
threat is algorithm laundering: treating a library default, capability check,
or caller value as fixed trusted policy. Do not infer a permitted algorithm from
identity correspondence or an intended call.

Run `npm run docs:check`, `npm run repository:check`,
`npm run security:scan`, `git diff --check`, session, and gate checks.
Builds, audit, signing, Keychain, Apple/Xcode, target-Mac, provider, product,
and external checks remain unauthorized. A negative/failed result preserves
history and grants no fallback.

## Acceptance criteria

- [ ] One closed disposition with no target-derived data.
- [ ] D-097 through D-112 and all blockers preserved.
- [ ] No operational successor becomes Ready.

## Final results

Completed: `algorithm_contract_not_accepted`; no successor is Ready.
