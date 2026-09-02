# Personal Assistant v0 containment bootstrap trust decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Decision: D-104 accepted

## Goal

Record a narrow documentation decision about whether an identity-free ad-hoc
code seal used only to activate a future sandbox helper can be considered
separately from P4's later Developer ID signer-binding proof.

## User-visible outcome

None. This is repository-governance documentation only.

## Scope

- Distinguish `sandbox_activation_adhoc_v1` from
  `product_signer_binding_v1`.
- Preserve D-102's complete containment requirements and D-103's historical
  negative reviewed-set result.
- Record only D-104 and the fifteen declared documentation paths.

## Explicit non-goals

No source, test, dependency, configuration, entitlement, build, process,
target-Mac, Apple, Xcode, Keychain, certificate, private key, signing,
credential, provider, product, or state-changing external action. No branch,
commit, push, merge, or publication.

## Existing behavior and constraints

D-103 selects no eligible candidate. A narrow authority-class decision cannot
stand in for D-102 pre-effect filesystem/network denial, complete descendant
membership, termination, reaping, quiescence, cleanup, bootstrap provenance, or
P4 identity binding.

## Current-state evidence

The gate began from clean synchronized `main` at
`a6601f9920027dfcdc8dd43f99a27554fe0ac786`. The exact branch is
`codex/p3-containment-bootstrap-trust-decision`. No operational evidence has
been collected.

## Files expected to change

The exact fifteen paths in the linked ExecPlan, including this record, the
plan, D-104, project-memory documents, and the post-increment review.

## Affected components

Security architecture and repository governance only. Product behavior is
unchanged.

## Interfaces and invariants

- The bootstrap class has no product identity or Apple-account authority.
- The P4 signer class cannot bootstrap P3, and the bootstrap class cannot prove
  P4.
- A later static candidate re-review requires separate owner approval.
- P3-3 and every operational successor remain Blocked.

## Implementation milestones

- [x] Baseline verified and approved gate begun.
- [x] Official public-contract review completed.
- [x] D-104 and project-memory synchronization completed.
- [x] Documentation validation and independent reviews completed.
- [x] Passing post-increment report prepared.

## Security and privacy considerations

Reject broad signing/entitlement relief, target-derived evidence, a helper
bootstrap assumption, incomplete containment contracts, and any attempt to
reclassify documentation as operational proof. Preserve all D-097 historical
evidence and D-102 requirements.

## Test plan

Review the two-class distinction, D-102/D-103 preservation, official-source
limits, exact documentation scope, and historical evidence preservation. Run
the documentation-tier commands in the plan; operational checks remain Not run.

## Verification commands

- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 .codex/hooks/session_end_gate.py`
- `python3 .codex/hooks/post_increment_gate.py status`

## Risks

The documentation distinction could be mistaken for containment or Developer
ID proof. The decision must leave candidate sufficiency and operational
readiness Blocked.

## Rollback or failure strategy

Use `apply_patch` only for uncommitted in-scope documentation rollback. Use a
separately approved additive revert or superseding decision after publication.
Never rewrite historical evidence.

## Decisions made

D-104 accepts the bounded two-class distinction for a future separately
approved static candidate review only. It selects no primitive and authorizes
no operational action.

## Discoveries

Official public documentation supports the narrow identity distinction but
does not establish helper bootstrap provenance, D-102 containment, or P4
identity evidence.

## Progress

- 2026-09-02: Owner approved this exact documentation-only increment and its
  gate began on the declared branch.
- 2026-09-02: Documentation validation and independent review passed. One
  advisory preserves Blocked successor readiness because D-102 remains unproved.

## Acceptance criteria

- [x] A closed, non-substitutable bootstrap/P4 authority distinction is recorded.
- [x] D-102/D-103 and all historical evidence remain intact.
- [x] No operational authority, product behavior, or external state changes.
- [x] Required documentation validation and review pass.

## Final results

`PASS WITH ADVISORIES`: the documentation decision completed without product or
external action. No candidate or safe bootstrap proves D-102; successors remain
Blocked.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
