# Personal Assistant v0 in-process key-use containment classification

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Decision: D-107 accepted

## Goal

Classify exactly
`in_process_security_framework_ephemeral_challenge_proof_v1` from one frozen
current Apple and pinned Rust source register. Determine whether its childless,
fileless, in-memory challenge-signature boundary is eligible only for later
present-use implementation planning or remains ineligible/contract-unproved.

## User-visible outcome

None. This is static repository-governance documentation only.

## Scope

- Freeze one explicit-owner-action, one-attempt, no-input conceptual candidate.
- Separate in-memory private-key liveness from product code signing and every
  build-bearing path.
- Disposition nineteen closed contract rows and record one additive D-107
  result.
- Reconcile only the exact fifteen documentation paths in the ExecPlan.

## Explicit non-goals

No product source, dependency, lockfile, configuration, capability, CSP,
permission, entitlement, hook, helper, fixture, build, process, filesystem,
network, Apple, Xcode, Keychain, certificate, private-key, signing,
verification, target-Mac, provider, product, external-system, commit, push,
merge, release, or publication action.

## Existing behavior and constraints

D-096 through D-106 remain controlling. D-101 prohibits ambient/default
Keychain scope and D-102 remains binding for every build-bearing path. D-097
remains terminally `failed` / `FAIL` / `Blocked` with no completion marker. No
identity-selection, private-key-use, signing, or operational containment
boundary currently exists.

## Current-state evidence

The owner approved this documentation review from clean synchronized `main` at
`2287c1bfe999d733495ee78728f4dc7a653f393f`. The D-106 predecessor gate was
complete and valid with `PASS WITH ADVISORIES`. The branch and shortened valid
gate identifier recorded in the ExecPlan change no candidate or scope. No
operational or target-derived evidence has been collected.

## Files expected to change

Exactly the fifteen documentation paths in the linked ExecPlan.

## Affected components

Repository governance and future signing architecture documentation only.
Product runtime, IPC, UI, target-Mac state, and external systems are unchanged.

## Interfaces and invariants

- Candidate, sources, contracts, and outcomes are frozen and application-owned.
- The primitive accepts no caller input and performs no identity lookup.
- A later proof could claim only one present-attempt challenge signature by an
  already-bound opaque identity reference plus paired-public-key verification.
- D-102 remains unchanged for every product or build-bearing path.
- D-100 closed evidence and D-101 identity-scope requirements remain mandatory.

## Implementation milestones

- [x] Clean synchronized baseline and valid predecessor verified.
- [x] Approved branch created and exact gate begun.
- [x] Static classification and D-107 disposition completed.
- [x] Documentation validation and completion reviews completed.

## Security and privacy considerations

No identity, key, certificate, signature, challenge, account, path, label,
serial, fingerprint, Team ID, raw error, prompt, or target-derived value may
enter the record. Private-key export, debug formatting, persistent references,
fallback, retry, and semantic elevation to product signing are prohibited.

## Test plan

Verify the fixed candidate/source/contract identities, closed result, historical
preservation, claim ceiling, exact documentation diff, and protected-path
non-change. All operational checks remain Not run.

## Verification commands

Use the exact documentation-tier and protected-path commands in the ExecPlan.

## Risks

The result could overstate data signing as code signing, conceal ambient
Keychain authority, or confuse result rejection with cancellation of private-
key use. Each is a fail-closed stop condition.

## Rollback or failure strategy

Use `apply_patch` only for uncommitted in-scope documentation rollback. Never
reset, clean, discard, or rewrite historical evidence.

## Decisions made

D-107 records `not_eligible_or_unproven`. Eight rows are `documented`; eleven
are `contract_unproven`. No candidate or successor is admitted.

## Discoveries

The pinned safe wrappers expose the cryptographic primitives but current public
contracts do not yet establish exact identity provenance, prompt-free signing,
or hard cancellation. The complete frozen record contains eight documented and
eleven unproved rows.

## Progress

- 2026-09-02: Owner approved the exact candidate and source-review plan.
- 2026-09-02: Clean baseline verified; branch created and gate begun.
- 2026-09-02: Static review completed and D-107 closed negatively without any
  operational action.

## Acceptance criteria

- [x] Nineteen rows and one closed D-107 result are complete.
- [x] Historical evidence and operational blockers remain unchanged.
- [x] Exact documentation validation passes.

## Final results

The source classification is `not_eligible_or_unproven`: eight rows are
`documented` and eleven are `contract_unproven`. No candidate or successor is
admitted. Documentation validation and the completion workflow passed with
`PASS WITH ADVISORIES`; next-increment readiness is `Blocked`.

## Documentation updates

See the linked ExecPlan.
