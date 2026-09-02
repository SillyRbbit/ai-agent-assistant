# Personal Assistant v0 containment bootstrap trust decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Increment: `personal-assistant-v0-containment-bootstrap-trust-decision`
Predecessor: completed D-103 containment primitive selection

## Goal

Decide, from bounded official public documentation only, whether a future static
P3 candidate review may distinguish an identity-free, disposable ad-hoc code
seal used solely to carry an exact App Sandbox entitlement set from P4's later
Developer ID signer-binding proof. The result can only be a closed documentation
policy: eligible for a later static re-review, or still ineligible.

## User-visible outcome

None. This adds no product behavior, signing artifact, target-Mac operation,
entitlement, or external-system state. Approved public documentation reads are
the sole external contact.

## Scope

1. Define two non-substitutable future authority classes:
   `sandbox_activation_adhoc_v1`, a disposable local ad-hoc code seal limited to
   an exact future App Sandbox helper entitlement set; and
   `product_signer_binding_v1`, P4's later Developer ID team, leaf, and
   fingerprint binding with controlled private-key use.
2. Decide whether the first class can remove only the historical independent
   entitlement/signing circularity rejection in a future static re-review. It
   cannot satisfy any other D-102 predicate.
3. Record the bootstrap-provenance question: a helper/controller cannot be
   treated as contained merely because it is later signed or sandboxed.
4. Record D-104 and reconcile only the exact fifteen documentation paths.

## Explicit non-goals

- No candidate selection, P3-3 controller plan, source, test, dependency,
  lockfile, configuration, capability, CSP, permission, or entitlement file.
- No `codesign`, `security`, `xcodebuild`, `xcrun`, compiler, npm install,
  npm build/runtime command, Cargo, Vite, Tauri, helper, child-process, probe,
  target-Mac, filesystem, network, signing, or build operation. The declared
  repository documentation-validation commands are the sole npm exceptions.
- No Apple, Xcode, Keychain, certificate, private key, Team ID, provisioning
  profile, credential, provider, product, authenticated, or state-changing
  external action.
- No weakening of D-102's pre-effect filesystem/network control, complete
  descendant membership, termination, reaping, quiescence, cleanup, evidence,
  no-root, no-global-state, or no-new-dependency requirements.
- No rewrite or downgrade of D-097's `failed` / `FAIL` / `Blocked` state,
  original report/digests, Failed privacy finding, Pending Open Directory
  boundary, Not-run signing evidence, or missing completion marker.
- No branch, commit, push, merge, pull request, release, or publication.

## Existing behavior and constraints

- D-103 selected no eligible candidate in its frozen set. The App Sandbox helper
  composition was rejected both for the then-independent no-entitlement/
  no-prerequisite-signing rule and absent public contracts for complete
  detached-descendant membership, containment-wide termination, and race-free
  quiescence.
- Apple documents an ad-hoc code signature as sealed without a signing identity,
  while App Sandbox relies on entitlement-bearing code signatures. Apple helper
  guidance uses local ad-hoc signing during preparation, then replaces that
  identity during product embedding. These narrow semantics do not establish
  target availability, sandbox enforcement for this future composition,
  bootstrap provenance, effect control, or graph lifecycle ownership.
- P3 precedes P4. P4 Developer ID evidence cannot bootstrap P3, and P3 bootstrap
  evidence cannot establish P4 identity, custody, provenance, or signing success.

## Current-state evidence

- The gate began from clean synchronized `main` at
  `a6601f9920027dfcdc8dd43f99a27554fe0ac786`, matching live `origin/main`.
- Node `26.3.0`, npm `11.16.0`, and Rust/Cargo `1.90.0` match repository pins.
  Baseline `npm run docs:check` passed.
- The predecessor gate is complete, valid, and `PASS WITH ADVISORIES`; its next
  readiness is `Blocked`.

## Files expected to change

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/increments/personal-assistant-v0-containment-bootstrap-trust-decision.md`
- `docs/plans/2026-09-02-personal-assistant-v0-containment-bootstrap-trust-decision.md`
- `docs/reviews/2026-09-02-personal-assistant-v0-containment-bootstrap-trust-decision-post-increment-review.md`

## Affected components

Repository governance and future security architecture only. Product runtime,
IPC, build graph, target Mac, credentials, and external systems are unchanged.

## Interfaces and invariants

- Both authority classes are conceptual documentation labels, not runtime types,
  commands, DTOs, configuration, or operational authorization.
- `sandbox_activation_adhoc_v1` carries no Developer ID, Apple-issued
  certificate, private key, Team ID, Keychain, provisioning profile,
  authentication, distribution, or product-identity claim.
- `product_signer_binding_v1` is P4-only. It cannot bootstrap P3, and the
  bootstrap class is categorically inadmissible as P4 evidence.
- An affirmative D-104 result may admit only a separately owner-approved static
  candidate re-review. It does not select a candidate, make P3-3 Ready, or alter
  D-102's conjunctive contracts.
- Source silence, ambiguity, archived-only availability claims, bootstrap gaps,
  broad entitlements, or inherited authority remain `contract_unproven` and
  Blocked; no residual-risk or compensating-control path exists.
- Future evidence remains D-100-minimized, closed, non-authorizing, and bound
  privately to one approved plan/check/attempt.

## Implementation milestones

- [x] Confirm clean synchronized baseline, valid predecessor gate, and approved
      documentation-only scope.
- [x] Begin the exact post-increment gate.
- [x] Review narrow official public contracts and record their limits.
- [x] Record additive D-104 and synchronize project-memory paths.
- [x] Run documentation-tier validation and independent reviews.
- [x] Prepare a passing post-increment report for deterministic finalization.

## Security and privacy considerations

| Threat                                                     | Required control                                                                                                  |
| ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| P3/P4 signing authority is conflated                       | Keep bootstrap and product signer classes non-substitutable.                                                      |
| A local code seal becomes permission to sign or distribute | Prohibit Developer ID, certificate, Keychain, private key, Team, provisioning, and distribution authority.        |
| App Sandbox is overstated as full containment              | Preserve every D-102 effect, graph, termination, reaping, and quiescence predicate.                               |
| The helper bootstrap is assumed safe                       | Require independently reviewed provenance before any future candidate can pass.                                   |
| Broad entitlements widen authority                         | Permit no unspecified network, filesystem, IPC, automation, Keychain, device, or temporary-exception entitlement. |
| Static source material leaks sensitive data                | Retain only public citations and fixed categorical dispositions.                                                  |
| Historical failure is silently healed                      | Preserve D-097/D-098 and all Failed/Pending/Not-run evidence.                                                     |

## Test plan

- Verify D-104 names only the two authority classes and makes them
  non-substitutable.
- Verify any eligibility relief is limited to the historical independent
  entitlement/signing rejection and does not alter remaining D-102 predicates or
  D-103's historical bounded result.
- Verify public-source material never claims a helper, ad-hoc seal, or process
  observation proves complete containment, target availability, or P4 identity.
- Verify bootstrap provenance, effect denial, graph membership, termination,
  reaping, quiescence, and P3-3 source checks remain Blocked or Not run.
- Verify D-097 through D-103, their historical findings, and the absent D-097
  completion marker remain unchanged.
- Verify exactly the declared fifteen documentation paths change and protected
  source, dependency, configuration, workflow, hook, skill, and script paths
  remain unchanged.

## Verification commands

- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json`
- `python3 .codex/hooks/session_end_gate.py`
- `python3 .codex/hooks/post_increment_gate.py status`

`npm run verify`, builds, process/probe checks, target-Mac checks, and all
Apple/Xcode/Keychain/signing/provider/product/external checks remain Not run by
approved scope.

## Risks

The main risk is treating a documented identity distinction as proof that a
future containment helper works. An affirmative documentation decision leaves
the candidate, bootstrap, effect-control, and lifecycle requirements unproved;
the closed result and continued Blocked readiness prevent that overstatement.

## Rollback or failure strategy

Before publication, reverse only uncommitted in-scope documentation edits with
`apply_patch`. After publication, use a separately approved additive superseding
or revert commit. Never reset, discard, or rewrite historical evidence. If a
required public contract is absent or ambiguous, record the negative decision
and preserve Blocked readiness.

## Decisions made

D-104 accepts the two non-substitutable conceptual classes. It has not selected
a containment primitive or authorized operational work.

## Discoveries

The official public sources support only the narrow distinction between an
ad-hoc seal without a signing identity and later Developer ID identity binding.
They do not prove helper bootstrap provenance or any remaining D-102 predicate.

## Progress

- 2026-09-02: Owner approved the exact documentation-only increment, branch,
  and gate. No operational action has run.
- 2026-09-02: Required documentation, repository, security, whitespace,
  protected-path, session-inventory, and active-gate checks passed. Independent
  architecture, security, code-health, debt, and readiness reviews found no
  completion blocker. One containment advisory blocks successors.

## Acceptance criteria

- [x] D-104 distinguishes narrow bootstrap and P4 signer classes without
      granting operational authority.
- [x] Every remaining D-102 containment predicate and historical D-103 result
      stays intact.
- [x] P3-3, P3-4, P3-5, P4, signing, V0-3, and operational successors remain
      Blocked.
- [x] The exact documentation-only scope and required validation pass.

## Final results

`PASS WITH ADVISORIES`: the fifteen-path documentation-only decision is
complete. The advisory is that no candidate or safe bootstrap proves the
remaining D-102 containment contracts; it blocks P3-3 and every operational
successor, not this closeout.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
