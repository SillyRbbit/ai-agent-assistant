# D-107 private-key non-export contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Baseline: `a126e26156393c4d400e664cbb36ea5e98ca6c58`
Predecessor: D-111 (`d107-account-keychain-scope-contract-decision`)

## Goal

Perform one repository-only, documentation-only review of D-107's
`private_key_nonexport_contract`. Decide only whether accepted repository
records prove a future signing boundary makes private-key bytes and external
representations unreachable from every application-owned path.

The review must fail closed. It cannot infer non-export merely because an
intended call need not export bytes or because a native key reference is opaque.

## User-visible outcome

None. This plan neither accesses nor exports a private key, calls Keychain or
Security framework APIs, signs data, builds the product, or changes behavior.

## Scope

1. Re-read D-096 through D-111 and current cryptography-adjacent repository
   source only.
2. Define a future non-export contract without adding a key wrapper, serializer,
   error/log path, debug implementation, Tauri DTO, persistence, or runtime
   interface.
3. Define only `nonexport_contract_documented`,
   `nonexport_contract_not_accepted`, and `boundary_failed`.
4. Preserve D-097; D-107's 8/11 record; D-108's 9/10 interpretation; and
   D-109 through D-111's negative results. Keep readiness Blocked.

## Explicit non-goals

- No source, dependency, lockfile, configuration, test, workflow, hook,
  capability, CSP, permission, entitlement, or toolchain change.
- No Keychain, certificate, private key, external representation, signing,
  Apple/Xcode, build, target-Mac, provider, product, network, credential, or
  external-system operation.
- No caller/model/WebView/environment/runtime/profile/task/run/workflow/agent
  key selection; no generic signing or key-export interface; no key bytes,
  representations, certificate data, errors, logs, or target-derived values.
- No branch, gate, commit, push, merge, release, or publication.

## Existing behavior and constraints

- D-107 records this contract as unproved: the intended path need not export
  bytes, but Apple and the pinned safe crate expose external representation and
  no implementation inventory proves export unreachable.
- D-109, D-110, and D-111 supply no identity issuer, expected signer binding,
  or owned scope. D-101/D-102 and every remaining D-107 contract remain
  independently controlling.

## Current-state evidence

Clean synchronized `main`, `HEAD`, and `origin/main` resolved to
`a126e26156393c4d400e664cbb36ea5e98ca6c58`. D-111 completed validly with
`PASS WITH ADVISORIES`. No implementation currently proves all key-export
routes unreachable.

## Files expected to change

A separately approved documentation increment must fix an exact 15-file scope:
this plan, one increment record, one review, `DECISIONS.md`, and the
applicable current-state documents `ARCHITECTURE.md`, `CHANGELOG.md`,
`HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`, `PROJECT_STATUS.md`,
`ROADMAP.md`, `SECURITY.md`, `SECURITY_CHECKLIST.md`, `TESTING_GUIDE.md`,
and `TROUBLESHOOTING_LOG.md`. Any other path is a stop condition.

## Interfaces and invariants

`PrivateKeyNonExportPolicyV1` is conceptual governance documentation, not a
Rust/Tauri/WebView interface. A future positive design needs independently
approved proof that a private adapter owns one non-serializable native key
reference; no production path can request/export/debug/format/serialize/store/
log it or derive its external representation; and every error, cancellation,
cleanup, and test boundary preserves that exclusion.

The three governance labels are not D-100 evidence or runtime values. Missing,
ambiguous, contradictory, or drifted facts select `boundary_failed`; absent
complete proof selects `nonexport_contract_not_accepted`. Neither admits a
candidate or reduces other blockers.

## Milestones and verification

- [ ] Obtain separate branch/gate approval.
- [ ] Review repository-only facts and record one closed disposition.
- [ ] Run `npm run docs:check`, `npm run repository:check`,
      `npm run security:scan`, `git diff --check`, session, and gate checks.
- [ ] Stop with readiness Blocked.

Product verification, audit, builds, Keychain/private-key/certificate/signing,
Apple/Xcode, target-Mac, provider, product, and external commands are not
authorized.

## Security, risks, and rollback

The threat is non-export laundering: mistaking opaque references or a
non-exporting happy path for whole-system non-export while hidden safe APIs,
debug output, error paths, tests, or future DTOs remain able to reveal key
material. Keep the review repository-only, preserve all historical evidence,
and stop on scope expansion. Before publication, remove this uncommitted draft
only with owner direction; a future negative or failed decision permits no
fallback or operational action.

## Acceptance criteria

- [ ] One closed disposition is selected with no key-derived data.
- [ ] D-097 through D-111 and all blockers remain preserved.
- [ ] No operational successor becomes Ready.

## Final results

Completed: D-112 selects `nonexport_contract_not_accepted`; no successor is Ready.
