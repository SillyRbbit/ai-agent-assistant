# D-107 interaction-denial contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Baseline: `9b1e367`
Predecessor: D-113 (`d107-fixed-algorithm-contract-decision`)

## Goal

Perform one repository-only, documentation-only review of
`interaction_denial_contract`: whether private-key retrieval and signing can
be guaranteed prompt-free for the frozen D-107 candidate.

The review fails closed. Lookup-time skip/context controls must not be treated
as proof that use of an already-held identity cannot display UI.

## Scope and non-goals

Review D-096 through D-113 and current repository source only. Define exactly
`interaction_denial_documented`, `interaction_denial_not_accepted`, and
`boundary_failed`. Preserve D-097, D-107's 8/11 record, D-108's 9/10
interpretation, all later negative decisions, every remaining blocker, and
Blocked readiness.

Do not change source, dependencies, configuration, tests, workflows, hooks,
capabilities, permissions, or product documentation. Do not access Keychain,
certificates, private keys, signing, Apple/Xcode, provider, product, build,
target-Mac, network, or external systems. Do not create a branch, begin a gate,
commit, push, or merge.

## Evidence and invariants

D-107 already records the decisive gap: lookup-time interaction controls do not
establish that private-key retrieval/signing on a held file-based identity
cannot prompt. A future `InteractionDenialPolicyV1` is conceptual only: it
would require trusted Rust and a private adapter to deny all interaction before
identity use, with no caller-selected context, fallback, retry, or UI path.
Neither policy wording nor result rejection proves OS prompt absence.

The closed labels are governance terms, not D-100 evidence or runtime values.
Missing, ambiguous, contradictory, or drifted facts select
`boundary_failed`; no complete current proof selects
`interaction_denial_not_accepted`.

## Files, validation, risks, and rollback

Any separately approved increment must define an exact documentation-only
inventory before its gate. Run `npm run docs:check`,
`npm run repository:check`, `npm run security:scan`, `git diff --check`,
session, and gate checks. Product verification, audit, builds, and all system/
external operations are unauthorized.

The threat is prompt-denial laundering: a lookup control, absent observed
prompt, or callback error is not complete interaction denial. Stop on any need
for target/system evidence. Remove this uncommitted draft only with owner
direction; a negative result grants no fallback or operational action.

## Acceptance criteria

- [ ] One closed disposition contains no target-derived data.
- [ ] Historical evidence and all blockers remain preserved.
- [ ] No operational successor becomes Ready.

## Final results

Completed: `interaction_denial_not_accepted`; no successor is Ready.
