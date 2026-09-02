# Personal Assistant v0 App Sandbox containment re-review

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Decision: D-105 accepted

## Goal

Re-review exactly
`app_sandbox_build_helper_plus_libsystem_supervision_v2` against all D-102
contracts after D-104 separated an identity-free ad-hoc sandbox-activation seal
from P4's later Developer ID signer binding.

## User-visible outcome

None. This is repository-governance documentation only.

## Scope

- Freeze the exact v2 candidate and authoritative first-party Apple source
  register.
- Change only D-104's signature-class assumption.
- Disposition all 22 D-102 contract rows and retain all ten absent P3-3 source
  checks as `not_run`.
- Record D-105 and synchronize exactly the fifteen declared documentation
  paths.

## Explicit non-goals

No source, test, dependency, lockfile, configuration, capability, CSP,
permission, entitlement, helper, build, process, probe, target-Mac, Apple,
Xcode, Keychain, certificate, private key, signing, credential, provider,
product, or state-changing external action. No P3-3 or later successor, commit,
push, merge, release, or publication.

## Existing behavior and constraints

D-102's requirements are conjunctive and fail closed. D-103's frozen v1
negative result remains immutable. D-104 changes only whether an identity-free
conceptual seal is automatically classified as P4 signer proof; it establishes
no containment or operational authority.

## Current-state evidence

The gate began on `codex/p3-app-sandbox-containment-rereview` from clean
synchronized `main` at `e1b2ff5c06a4c5bc6ad7fc19b63968668fff4923` with a
valid completed D-104 predecessor. No operational evidence was collected.

## Files expected to change

Exactly the fifteen paths in the linked ExecPlan, including this record, the
plan, D-105, project-memory documents, and the post-increment review.

## Affected components

Security architecture and repository governance only. Product behavior and
external state are unchanged.

## Interfaces and invariants

- The candidate identity and source corpus are fixed for one attempt.
- Every D-102 contract is independently necessary and receives one closed
  disposition.
- All ten source checks stay `not_run` because no controller source exists.
- An identity-free seal is neither P4 evidence nor proof of containment.
- No positive inference, residual-risk acceptance, alternative candidate, or
  automatic successor exists.

## Implementation milestones

- [x] Baseline verified and approved gate begun.
- [x] Candidate and public-source corpus frozen.
- [x] All 22 contracts and ten source checks dispositioned.
- [x] D-105 and project memory synchronized.
- [x] Documentation validation and independent reviews completed.
- [x] Passing post-increment report prepared.

## Security and privacy considerations

Reject broad entitlement relief, inherited-channel assumptions, direct-child
APIs as graph ownership, uncontained bootstrap, target-derived evidence, and
archived or silent documentation as current denial guarantees. Preserve D-097
through D-104 and every historical Failed, Pending, and Not-run fact.

## Test plan

Review the frozen v2 identity, exact entitlement ceiling, source register,
22-row contract table, ten `not_run` source checks, negative closed result,
historical preservation, and exact documentation-only diff. Run the
documentation-tier checks; all operational checks remain Not run.

## Verification commands

- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- protected-path `git diff --exit-code`
- `python3 .codex/hooks/session_end_gate.py`
- `python3 .codex/hooks/post_increment_gate.py status`

## Risks

Narrow App Sandbox and supervision mechanics could be overstated as complete
containment. The review must preserve each unproved contract and leave the
operational successor Blocked.

## Rollback or failure strategy

Use `apply_patch` only for uncommitted in-scope documentation rollback. After
publication, require a separately approved additive revert or superseding
decision. Never rewrite historical evidence.

## Decisions made

D-105 accepts `no_eligible_candidate_after_d104_rereview`. All 22 D-102
contracts are `contract_unproven`; all ten source checks are `not_run`. No
candidate or successor is selected.

## Discoveries

Current Apple documentation establishes only narrow platform mechanics. It
does not establish a safe first build, exact complete effect boundary, complete
descendant membership, terminal quiescence, cleanup ownership, or bounded
platform effects for the frozen candidate.

The initial documentation and repository-health runs each found the same three
Apple method URLs parsed as local targets because of URL parentheses. The
links were corrected without changing the source corpus or claims, and the
checks were rerun.

## Progress

- 2026-09-02: Owner approved the exact documentation-only branch, plan, and
  gate begin from the synchronized baseline.
- 2026-09-02: The bounded public-source review produced the negative D-105
  decision without operational action.
- 2026-09-02: Documentation checks and independent reviews passed with one
  advisory that blocks the next increment.

## Acceptance criteria

- [x] Exactly the v2 candidate was reviewed under only D-104's changed
      signature classification.
- [x] All 22 D-102 rows are `contract_unproven`; all ten source checks are
      `not_run`.
- [x] D-105 records a closed negative result and grants no operational
      authority.
- [x] D-097 through D-104 and all historical evidence remain intact.
- [x] Exact documentation scope and required validation passed.
- [x] P3-3 and every operational successor remain Blocked.

## Final results

`PASS WITH ADVISORIES`: the documentation review completed truthfully with
`no_eligible_candidate_after_d104_rereview`. Operational containment remains
unavailable, so no successor is Ready.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
