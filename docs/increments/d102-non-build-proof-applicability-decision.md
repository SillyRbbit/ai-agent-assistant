# D-102 non-build proof applicability decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02

## Goal

Decide whether D-102's build-child-containment policy can be classified
separately for exactly
`in_process_security_framework_ephemeral_challenge_proof_v1`, without waiving
D-102 or admitting the candidate.

## User-visible outcome

None. This increment changes governance documentation only.

## Scope

- Define one documentation-only applicability policy for the exact frozen
  candidate.
- Add D-108 and synchronize current project memory.
- Preserve D-107 history while recording the additive current contract count.
- Complete the documentation-tier gate.

## Explicit non-goals

No product/test source, dependency, configuration, capability, entitlement,
IPC, hook, workflow, script, build, helper, child, subprocess, artifact,
product-signing, Apple, Xcode, Keychain, certificate, private-key, target-Mac,
provider, credential, product, state-changing external-system, commit, push,
merge, release, or publication work. Documentation writes, local validation/
gate processes, and the approved read-only Git refresh are acknowledged. No
generic waiver, `not_applicable`, residual-risk acceptance, or successor start.

## Existing behavior and constraints

D-102 remains mandatory for build-bearing paths. D-107 historically records
eight `documented` and eleven `contract_unproven` rows and leaves its
applicability split unproved. Ten independent D-107 contracts remain unproved
even if the narrow split is accepted.

## Current-state evidence

- Clean synchronized baseline:
  `04fd0bc5cfbdbb25ebdb9b3f24940dd5d36b2bc3`.
- Valid predecessor gate: `complete`, `valid: true`, `PASS WITH ADVISORIES`.
- Exact branch and gate began only after owner approval.
- No D-108 record existed at baseline.

## Files expected to change

Exactly the fifteen documentation paths listed in the active ExecPlan.

## Affected components

Repository governance and current-state documentation only.

## Interfaces and invariants

The three review-time governance dispositions are
`split_documented_for_frozen_non_build_class`, `split_not_accepted`, and
`boundary_failed`. They are not D-100 evidence tokens. The split can apply only
to the exact frozen candidate while all no-build/no-candidate-launched-helper/
child/subprocess/external-executable/no-artifact/no-application-or-Rust-
dependency-filesystem-network-IPC/dynamic-code exclusions remain exact.

A definitive build, candidate-launched helper/child/subprocess/external
executable, artifact, staged/generated file, application- or Rust-dependency-
authored/selected/requested filesystem/network/socket/IPC API, `codesign`,
product signing, application- or Rust-dependency-selected dynamic-loader/JIT/
plugin/external code, or caller-selected authority makes D-102 fully mandatory
without a split disposition. Ambiguity or drift records `boundary_failed` and
also makes D-102 mandatory. OS-managed Keychain, `securityd`, cache, log, IPC,
trust, revocation, process-metadata, network, and loader effects internal to
the three fixed operations remain under the separate unproved platform-effect
contract.

## Implementation milestones

- [x] Baseline, predecessor gate, branch, and active gate confirmed.
- [x] Record the bounded decision and synchronize current memory.
- [x] Run independent reviews and documentation-tier verification.
- [x] Complete the gate without promoting a successor.

## Security and privacy considerations

The design fails closed against waiver laundering, scope drift, caller-selected
classification, hidden platform effects, D-100 vocabulary misuse, and
historical evidence rewriting. No private or target-derived value enters the
diff.

## Test plan

Review the closed policy table and every reattachment trigger; confirm protected
D-107 evidence and all non-documentation paths have no diff; run the active
plan's exact documentation checks and completion workflow.

## Verification commands

See the active ExecPlan.

## Risks

The bounded split could be overstated as containment or readiness. Exact scope,
automatic D-102 reattachment, independent unproved contracts, historical/current
count separation, and Blocked readiness prevent that claim.

## Rollback or failure strategy

Before publication, remove only this increment's fifteen-path documentation
diff. On drift or failed checks, stop and record a truthful failed disposition.

## Decisions made

Accepted D-108 selects the exact non-build-class split with automatic
fail-closed D-102 reattachment and no readiness promotion.

## Discoveries

The approved long governance disposition is not a D-100 outcome because it
exceeds the protocol's 32-byte outcome bound. D-107's existing factual record
remains unchanged.

## Progress

- 2026-09-02: Owner approved the exact bounded constraint reconsideration;
  branch and gate began from synchronized main.
- 2026-09-02: Independent findings were corrected within scope; the exact
  fifteen-path documentation diff then passed its required checks.

## Acceptance criteria

- [x] Exact split and fail-closed reattachment rules recorded.
- [x] D-107 history unchanged; current interpretation is clearly additive.
- [x] Ten independent contracts and all successors remain Blocked.
- [x] Exact documentation checks and completion gate pass.

## Final results

The exact applicability question is documented without a waiver, runtime
change, or operational authority. Required checks passed. Quality result:
`PASS WITH ADVISORIES`; next readiness: `Blocked`.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
