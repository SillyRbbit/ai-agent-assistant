# D-102 non-build proof applicability decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Increment: `d102-non-build-proof-applicability-decision`
Baseline: `04fd0bc5cfbdbb25ebdb9b3f24940dd5d36b2bc3`
Predecessor: completed D-107 operational-scope wording reconciliation

## Goal

Reconsider exactly one existing security constraint: whether D-102's future
build-child-containment policy applies to the frozen, conceptual
`in_process_security_framework_ephemeral_challenge_proof_v1` class when that
class has no build, candidate-launched helper, child, subprocess, external
executable, bundle, artifact, application- or Rust-dependency-authored/
selected/requested filesystem/network/socket/IPC API, `codesign`, product-
signing operation, or application- or Rust-dependency-selected dynamic-loader/
JIT/plugin/external code.

Record an additive, fail-closed applicability decision without weakening
D-102 for any build-bearing or effect-bearing path and without admitting the
candidate.

## User-visible outcome

None. This increment changes repository governance documentation only. It
does not add a feature, run a proof, use a private key, or change application
behavior.

## Scope

1. Add D-108 as the sole durable decision.
2. Define the documentation-only `D102ApplicabilityPolicyV1` for exactly the
   frozen D-107 candidate.
3. Record one of three closed governance dispositions:
   `split_documented_for_frozen_non_build_class`, `split_not_accepted`, or
   `boundary_failed`.
4. Preserve D-107's historical 8 documented / 11 contract-unproven record and
   add a current prospective disposition of 9 documented / 10
   contract-unproven rows only if the split is accepted.
5. Keep every remaining identity, signer, key-export, algorithm, interaction,
   cancellation, late-result, cleanup, and platform-effect contract unproved.
6. Synchronize current project memory and complete the documentation gate.

## Explicit non-goals

- No edit to D-096 through D-107, their plans, increments, reports, digests,
  findings, completion records, or historical Failed, Pending, or Not-run
  evidence.
- No generic D-102 waiver, `not_applicable` state, residual-risk acceptance,
  weighted score, compensating control, alternate candidate, or automatic
  successor.
- No claim that an in-process Security framework call has no Keychain,
  `securityd`, cache, log, IPC, trust, revocation, process-metadata, or
  OS-managed network effect.
- No product or test source, dependency, lockfile, configuration, capability,
  CSP, permission, entitlement, IPC, hook, workflow, script, or toolchain
  change.
- No product build, helper, child, subprocess, executable or artifact
  generation, application-authored filesystem path operation, `codesign`,
  product signing, Apple, Xcode, Keychain, certificate, private-key, target-Mac,
  provider, credential, product, or state-changing external operation.
- Documentation writes, local validation/gate processes, and the approved
  read-only Git remote refresh are the only repository/external effects in
  scope. No authenticated or state-changing external operation is authorized.
- No commit, push, pull request, merge, release, publication, or successor
  start.

## Existing behavior and constraints

- D-102 is the accepted fail-closed policy for future build-bearing paths. It
  requires pre-effect control of the fixed executable graph, filesystem and
  network effects, descendant ownership, shutdown, reaping, quiescence,
  cleanup, quarantine, and minimized evidence.
- D-107's exact conceptual candidate removes its own build, child, artifact,
  filesystem-write, `codesign`, and direct-network-request surface, but D-107
  left the applicability split `contract_unproven` because no accepted decision
  existed.
- D-107 remains an immutable historical classification with eight
  `documented` and eleven `contract_unproven` rows and governance result
  `not_eligible_or_unproven`.
- D-100 evidence outcomes are lowercase ASCII tokens no longer than 32 bytes.
  The approved human-readable governance disposition
  `split_documented_for_frozen_non_build_class` is intentionally not a D-100
  evidence outcome and must never be serialized into a D-100 record.
- D-101, the ten other D-107 unproved contracts, D-097's immutable terminal
  failure, and every operational blocker remain controlling.

## Current-state evidence

- After an approved remote refresh, clean `main`, `HEAD`, and `origin/main`
  all resolved to
  `04fd0bc5cfbdbb25ebdb9b3f24940dd5d36b2bc3`; `HEAD` was an exact ancestor of
  `origin/main`.
- The predecessor ignored gate reported `complete`, `valid: true`, and
  `PASS WITH ADVISORIES` before this increment began.
- The active branch is
  `codex/d102-non-build-proof-applicability-decision`, and the exact gate began
  as `d102-non-build-proof-applicability-decision`.
- The repository contains no D-108 record before this increment.
- This decision uses only accepted repository semantics. It collects no
  target-derived, private, Apple-account, Keychain, certificate, key, signature,
  host, path, provider, or personal evidence.

## Files expected to change

Exactly:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PROJECT_STATUS.md`
8. `ROADMAP.md`
9. `SECURITY.md`
10. `SECURITY_CHECKLIST.md`
11. `TESTING_GUIDE.md`
12. `TROUBLESHOOTING_LOG.md`
13. `docs/increments/d102-non-build-proof-applicability-decision.md`
14. this plan
15. `docs/reviews/2026-09-02-d102-non-build-proof-applicability-decision-post-increment-review.md`

No other path may change.

## Affected components

| Component                    | Effect                                                                         |
| ---------------------------- | ------------------------------------------------------------------------------ |
| D-102 applicability          | Split only for the exact frozen, non-build conceptual class.                   |
| D-107 current interpretation | One additive current row may become documented; historical D-107 is unchanged. |
| Platform effects             | Remain independently `contract_unproven`.                                      |
| Product/runtime              | No implementation or authority change.                                         |
| Roadmap/readiness            | Remains Blocked with no successor Ready.                                       |

## Interfaces and invariants

`D102ApplicabilityPolicyV1` is a documentation policy, not a runtime interface.
It has one fixed candidate and three closed governance dispositions:

| Condition                                                                                             | Disposition                                   |
| ----------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| The exact frozen candidate retains every exclusion and only D-102's build-child subject is classified | `split_documented_for_frozen_non_build_class` |
| The policy question is answered negatively without candidate drift                                    | `split_not_accepted`                          |
| Any required fact is ambiguous, missing, contradictory, or the candidate/policy boundary drifts       | `boundary_failed`                             |

The following definitive features are outside the frozen non-build class and
make D-102 fully mandatory without invoking a split disposition:

1. any product build, package, compiler, linker, lifecycle script, or build
   graph;
2. any helper, child, subprocess, shell, external executable, or detached
   descendant;
3. any executable, bundle, staged file, generated file, signature artifact, or
   other artifact generation;
4. any application- or Rust-dependency-authored, selected, or requested
   filesystem/path API access, read, write, mutation, mapping, or file-loading
   operation; OS-managed access internal to the fixed system-framework
   operations remains separately unproved;
5. any `codesign`, code-signing, product-signing, code-signature or
   product-signature verification, notarization, distribution, or release path;
6. any application- or Rust-dependency-authored, selected, or requested
   network, socket, or IPC API operation; OS-managed effects internal to the
   frozen Security framework/RNG operations remain separately unproved;
7. any application- or Rust-dependency-selected or requested dynamic-loader
   call, module, plug-in, JIT-generated code, or external code path; OS loader
   behavior for the fixed linked system frameworks remains separately unproved;
8. any direct application operation beyond the frozen sequence of one fixed-
   domain fresh 32-byte challenge generation, one data-signature creation, and
   one paired-public-key verification;
9. any caller-, model-, WebView-, environment-, current-directory-, account-,
   home-, path-, runtime-, profile-, task-, run-, workflow-, or agent-selected
   shape or identity.

Any missing, ambiguous, contradictory, or drifted required fact records
`boundary_failed` and also makes D-102 fully mandatory. An unmodeled effect,
scope expansion, implementation drift, fallback, retry, or substitution is
never eligible for the positive split.

D-102 is neither waived, satisfied, replaced, nor weakened. It remains fully
binding for every product-signing, executable-generating, artifact-generating,
build-bearing, helper, child, subprocess, or `codesign` path. The split is
conditional only while the frozen no-build/no-helper/no-child/no-subprocess/
no-bundle/no-artifact/no-application-or-Rust-dependency-filesystem-network-IPC/
dynamic-code shape remains exact.

OS-managed Keychain, `securityd`, directory, cache, log, IPC, trust,
revocation, process-metadata, and network effects are outside this applicability
decision. They remain under the separate unproved `platform_effect_contract`;
the absence of an application child does not disposition them.

## Implementation milestones

- [x] Confirm clean synchronized baseline and valid predecessor gate.
- [x] Create the exact branch and begin the exact approved gate.
- [x] Record D-108 and synchronize the exact documentation scope.
- [x] Independently review architecture, security, code health, debt, and
      readiness.
- [x] Run all required documentation checks and complete the gate.

## Security and privacy considerations

The primary threat is waiver laundering: relabeling an unproved security
boundary as irrelevant because a candidate is described as simpler. The closed
candidate identity, closed reattachment triggers, absence of a generic
`not_applicable` state, and independent platform-effect blocker prevent that
interpretation.

Other threats are candidate drift, caller-controlled classification, hidden
OS effects, D-100 vocabulary misuse, historical evidence rewriting, and an
incorrect readiness promotion. Every one fails closed. No target-derived
evidence or secret enters the diff.

## Test plan

- Table-review every frozen eligibility and reattachment condition exactly
  once.
- Confirm D-096 through D-107 and their plan/increment/review evidence are
  unchanged.
- Confirm all product, test, dependency, configuration, workflow, hook, skill,
  and script paths are unchanged.
- Confirm the exact fifteen-path change inventory contains no secret, private
  value, build output, log, credential, certificate, or key material.
- Run Markdown formatting/link, repository-health, security-scan, whitespace,
  session-inventory, independent-review, and completion-gate checks.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code 04fd0bc5cfbdbb25ebdb9b3f24940dd5d36b2bc3 -- docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md docs/plans/2026-09-01-personal-assistant-v0-build-child-containment-planning.md docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md docs/increments/personal-assistant-v0-key-use-containment-classification.md docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md
git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

`npm run verify`, `npm audit --audit-level=low`, builds, Apple, Xcode,
Keychain, certificate, private-key, signing, target-Mac, provider, product, and
state-changing external-system operational checks are Not run because this
increment changes documentation only and is prohibited from exercising those
boundaries. The approved read-only Git remote refresh is recorded separately.

## Risks

- A positive split could be misread as a D-102 waiver. Mitigation: name the
  exact subject and automatic reattachment triggers, prohibit generic
  `not_applicable`, and retain D-102 verbatim for every effect-bearing path.
- Current totals could overwrite D-107 history. Mitigation: label 8/11 as the
  immutable historical record and 9/10 only as the additive D-108 current
  disposition.
- The long governance outcome could violate D-100's 32-byte outcome bound.
  Mitigation: explicitly keep it outside D-100 serialization; the existing
  D-107 factual `contract_unproven` record remains unchanged.
- “No child” could be misread as “no platform effect.” Mitigation: preserve
  `platform_effect_contract` as independently unproved and list its OS-managed
  effects.

## Rollback or failure strategy

Before publication, remove only this increment's exact fifteen-path
documentation diff. Do not alter predecessor evidence or ignored predecessor
records. Stop and record a truthful failed disposition on an unexpected path,
historical drift, secret finding, failed required check, missing review, or any
attempt to promote readiness or exercise an operational boundary.

## Stop conditions

Stop immediately on dirty/divergent baseline evidence, scope expansion,
candidate drift, an unbounded or caller-selected classification, any source or
operational change, inability to preserve D-107 history, a required-check
failure, or a proposal to make a successor Ready.

## Decisions made

Accepted D-108 selects
`split_documented_for_frozen_non_build_class` for the exact conceptual class
while retaining every reattachment trigger and independent blocker above.

## Discoveries

- The approved positive governance disposition exceeds D-100's 32-byte outcome
  limit. It remains valid as human-readable governance vocabulary but cannot be
  emitted as a D-100 evidence token. No new factual D-100 record is needed for
  this repository-semantic decision.
- A childless application call may still involve OS-managed IPC, Keychain,
  cache, log, trust, revocation, or network effects. Those facts do not defeat
  the build-child subject split, but they independently keep the candidate
  unadmitted.

## Progress

- 2026-09-02: Owner selected and approved reconsideration of exactly
  `d102_applicability_split_contract`.
- 2026-09-02: Refreshed the remote, confirmed clean synchronized main at
  `04fd0bc`, created the exact planning branch, and began the gate.
- 2026-09-02: Independent review found and corrected ambiguous process and
  operational-scope wording, incomplete file/network/dynamic-code triggers,
  conflicting drift behavior, a fresh-challenge omission, a D-100 wording
  conflation, an overbroad OS-filesystem consequence, and one protected-path
  typo.
- 2026-09-02: The exact fifteen-path documentation scope passed all required
  checks and completed with `PASS WITH ADVISORIES`; no successor became Ready.

## Acceptance criteria

- [x] D-108 uses only the three closed governance dispositions and selects the
      exact current result.
- [x] D-102 automatically reattaches on every named trigger and is not waived,
      satisfied, replaced, or weakened.
- [x] OS-managed effects remain independently unproved.
- [x] Historical D-107 remains byte-for-byte unchanged and its 8/11 result is
      not relabeled.
- [x] The additive current disposition is exactly 9 documented / 10 unproved,
      with the candidate and every successor still Blocked.
- [x] The exact fifteen-path documentation diff passes every required check.

## Final results

Accepted D-108 resolves only the exact D-102 build-child subject question. The
exact documentation checks and completion gate passed. Historical D-107 remains
8/11, the additive current interpretation is 9/10, all ten remaining contracts
block the candidate, and no successor is Ready. Quality result: `PASS WITH
ADVISORIES`; next readiness: `Blocked`.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
