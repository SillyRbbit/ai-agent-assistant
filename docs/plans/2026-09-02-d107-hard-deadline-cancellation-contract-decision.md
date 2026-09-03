# D-107 hard-deadline cancellation contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Baseline: `38af59a`
Predecessor: D-114 (`d107-interaction-denial-contract-decision`)

## Goal

Perform one repository-only, documentation-only review of
`hard_deadline_cancellation_contract`: whether a future identity lookup,
private-key sign, and verification attempt can have an enforced deadline,
cancellation, cleanup ownership, and no fallback despite synchronous APIs.

## Scope and non-goals

Review D-096 through D-114 and current repository source only. Define exactly
`deadline_contract_documented`, `deadline_contract_not_accepted`, and
`boundary_failed`. Preserve D-097, D-107's 8/11 record, D-108's 9/10
interpretation, later negative decisions, all blockers, and Blocked readiness.

Do not change source, dependency, configuration, test, workflow, hook,
capability, permission, or product documentation. Do not access Keychain,
certificate, private key, signing, Apple/Xcode, build, target-Mac, provider,
product, network, or external systems. Do not create a branch, gate, commit,
push, or merge.

## Evidence and invariants

D-107 records the gap: Keychain lookup is documented as blocking, while
synchronous identity/sign/verify APIs expose no timeout or cancellation
parameter. A future `HardDeadlineCancellationPolicyV1` is conceptual only:
trusted Rust would own one attempt deadline, cancellation state, terminal
cleanup owner, and retry denial before the first effect. Rejecting a later
result does not prove cancellation stopped a private-key operation.

Labels are governance terms, not D-100 evidence or runtime values. Missing,
ambiguous, contradictory, or drifted facts select `boundary_failed`; absence
of complete pre-effect deadline/cancellation proof selects
`deadline_contract_not_accepted`.

## Files, validation, risk, rollback

Any separately approved increment must set an exact documentation-only inventory
before its gate. Run documentation, repository, security-scan, whitespace,
session, and gate checks only. Builds, audit, Keychain, signing, Apple/Xcode,
target-Mac, provider, product, and external work remain unauthorized.

The threat is cancellation laundering: a timer, UI cancellation, dropped
future, or late-result rejection must not be portrayed as stopping synchronous
private-key use or guaranteeing cleanup. Stop if target/system evidence is
needed. A negative result grants no fallback, retry, or operational action.

## Acceptance criteria

- [ ] One closed disposition with no target-derived data.
- [ ] Historical evidence and blockers remain preserved.
- [ ] No operational successor becomes Ready.

## Final results

Completed: `deadline_contract_not_accepted`; no successor is Ready.
