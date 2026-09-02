# Personal Assistant v0 containment primitive selection

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-02
Decision: D-103

## Goal

Apply D-102's exact fail-closed requirements to one frozen target-Mac candidate
set using authoritative public documentation, then record either one eligible
candidate or no eligible candidate in that reviewed set.

## User-visible outcome

None. This increment adds no product behavior and performs no target-Mac or
authenticated/state-changing external operation. Approved public documentation
reads are its sole external contact.

## Scope

- Deep-review only the App Sandbox helper plus public libSystem supervision
  composition.
- Retain three fixed negative controls and screen two scope-ineligible classes.
- Record D-103's bounded negative result: no eligible candidate in the reviewed
  set.
- Reconcile only the exact fifteen documentation paths in the ExecPlan.

## Explicit non-goals

No primitive, controller, source, test, dependency, configuration, entitlement,
signing, build, process, probe, filesystem, network, Apple, Keychain, provider,
credential, product, authenticated external-system, or state-changing external-
system action. Approved read-only public documentation access is the sole
external contact. No branch, commit, push, merge, release, or publication.

## Existing behavior and constraints

D-102's pre-effect effect-denial and complete descendant-lifecycle contracts
are conjunctive. A candidate requiring entitlements or signing is ineligible
under this increment's no-circular-prerequisite rule. A missing or ambiguous
public contract is `contract_unproven`, never an inferred pass.

## Current-state evidence

The gate began from clean synchronized `main` at
`355e1efcdd2c5d651dd1609959581e7184b61b60`. No operational or target-derived
evidence was collected.

## Files expected to change

The exact fifteen paths listed in the linked ExecPlan, including this increment
record, D-103, project memory, and the post-increment review.

## Affected components

Repository governance and security planning only. Product runtime, IPC, build
graph, target Mac, and external systems are unchanged.

## Interfaces and invariants

- Candidate identities and eligibility rules are fixed by the ExecPlan.
- Every eligibility gate is mandatory; there is no weighting or compensating
  control.
- Public source silence, ambiguity, deprecation, or conflict fails closed.
- Static citations and categorical outcomes are non-authorizing.
- The result is bounded to the reviewed set and cannot admit an unreviewed
  candidate.
- P3-3 and every later operational milestone remain Blocked.

## Implementation milestones

- [x] Baseline and predecessor verified.
- [x] Documentation gate begun.
- [x] Frozen candidates reviewed against authoritative public contracts.
- [x] D-103 and project-memory reconciliation drafted.
- [x] Documentation checks and independent reviews passed.
- [x] Passing post-increment report prepared for deterministic finalization.

## Security and privacy considerations

Reject entitlement/signing circularity, deprecated/private mechanisms,
privileged/global-state mechanisms, post-hoc observation, incomplete descendant
ownership, and imprecise network or filesystem claims. Retain no target-derived
content or sensitive identifier.

## Test plan

Run documentation formatting/link validation, repository health, secret scan,
whitespace check, protected-path diff, session inventory, independent
architecture/security/code/debt/readiness review, and the deterministic
post-increment gate. Operational and complete product verification remain Not
run by scope.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Risks

The principal risk is overstating a partial sandbox or process API as complete
D-102 containment. The closed negative result prevents that claim but leaves
P3-3 blocked.

## Rollback or failure strategy

Reverse only these uncommitted documentation edits with `apply_patch`, or use a
later separately approved additive revert after publication. Never reset or
rewrite historical evidence.

## Decisions made

D-103 selects no candidate from the frozen reviewed set.

## Discoveries

The publicly documented App Sandbox/helper route requires entitlement and
signing state, while public direct-child/process-event APIs do not fill D-102's
complete detached-descendant membership and quiescence gaps.

## Progress

- 2026-09-02: Owner approved the exact static source review and authoritative
  read-only source access; review completed without operational execution.

## Acceptance criteria

- [x] Candidate inventory and eligibility gates are closed.
- [x] D-103 records only the bounded reviewed-set result.
- [x] Historical evidence and operational blockers are preserved.
- [x] Required checks and reviews pass.
- [x] Report and workspace satisfy completion-marker prerequisites.

## Final results

`PASS WITH ADVISORIES`. Documentation, repository-health, secret, whitespace,
protected-path, session-inventory, and independent review gates passed. No
eligible candidate exists in the frozen reviewed set; P3-3 and every operational
successor remain Blocked. Complete product verification and all operational
checks were Not run by approved scope.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
