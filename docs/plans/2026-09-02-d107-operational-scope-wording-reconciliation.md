# D-107 operational-scope wording reconciliation

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Increment: `d107-operational-scope-wording-reconciliation`
Baseline: `80dab5bb6b1d9065399bc533c3d6c2bf3c84abfb`
Predecessor: completed D-107 in-process key-use containment classification

## Goal

Reconcile one documentation-accuracy discrepancy discovered after D-107 was
squash-merged: several sentences use unqualified `process` or `filesystem`
absence language even though the documentation increment necessarily wrote
documentation files and ran local validation and gate processes.

The correction must distinguish those repository-governance effects from the
actual D-107 boundary: no product or test source, dependency, configuration,
capability, entitlement, or IPC changed, and no product-build, signing,
Keychain/private-key, target-Mac, or state-changing external operation ran.

## User-visible outcome

None. This is an additive documentation clarification only. It changes no
product behavior, capability, decision outcome, evidence count, or readiness.

## Scope

1. Record one additive correction without altering the published D-107
   decision, plan, increment record, or post-increment review.
2. Correct the two mutable current-state summaries in `PLANS.md` and
   `PROJECT_STATUS.md`.
3. Reconcile current handoff and queue records with the clarification.
4. Record the discrepancy and its preservation strategy in the changelog and
   troubleshooting history.
5. Add this plan, one increment record, and one post-increment review.

## Explicit non-goals

- No edit to D-107, D-097, D-098, any other accepted decision, or any prior
  plan, increment, report, digest, gate result, completion marker, finding, or
  historical Failed, Pending, or Not-run fact.
- No change to D-107's `not_eligible_or_unproven` result, eight `documented`
  rows, eleven `contract_unproven` rows, or Blocked successor readiness.
- No product or test source, dependency, lockfile, configuration, workflow,
  hook, script, capability, CSP, permission, entitlement, IPC, runtime, build,
  credential, provider, signing, Keychain, product filesystem behavior, or
  external-system change.
- No Apple, Xcode, Keychain, certificate, private-key, signing, build,
  target-Mac, provider, network, product, or state-changing external action.
- No commit, push, pull request, merge, release, or publication.

## Existing behavior and constraints

- D-107 remains accepted historical evidence with exact factual outcome
  `contract_unproven` and governance result `not_eligible_or_unproven`.
- The D-107 completion gate remains valid with `PASS WITH ADVISORIES`; its
  report and digest are historical evidence and must not be rewritten.
- `ENGINEERING_GUIDE.md` requires additive corrections or superseding current-
  state records rather than silent rewrites of dated evidence.
- Documentation files were written and local formatting, repository, security,
  Git, session, and completion-gate processes ran during D-107.
- No product/build/signing/Keychain/target-Mac operational process or state-
  changing external action ran during D-107.

## Current-state evidence

- Clean synchronized `main` and `origin/main` both resolved to
  `80dab5bb6b1d9065399bc533c3d6c2bf3c84abfb` before branch creation.
- PR #94 squash-merged the exact fifteen-path D-107 documentation change; its
  pull-request and post-merge Documentation workflows passed.
- The ignored predecessor gate reported `complete`, `valid: true`, and
  `PASS WITH ADVISORIES` before this increment began.
- Independent post-publication review found four originally reported ambiguous
  locations. Required-chain review then found one equivalent current summary in
  `PLANS.md`, bringing the complete ambiguity inventory to five.
- The frozen five-location ambiguity inventory is:
  1. `CHANGELOG.md`;
  2. `PLANS.md`;
  3. `PROJECT_STATUS.md`;
  4. `docs/increments/personal-assistant-v0-key-use-containment-classification.md`;
     and
  5. `docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md`.
     The first receives an additive correction, the second and third are mutable
     current-state summaries, and the fourth and fifth remain byte-for-byte
     unchanged historical evidence.
- `docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md`
  already distinguishes prohibited build/process launch and filesystem-effect
  testing from documentation work and needs no correction.

## Files expected to change

Exactly:

1. `CHANGELOG.md`
2. `HANDOFF.md`
3. `NEXT_STEPS.md`
4. `PLANS.md`
5. `PROJECT_STATUS.md`
6. `TROUBLESHOOTING_LOG.md`
7. `docs/increments/d107-operational-scope-wording-reconciliation.md`
8. this plan
9. `docs/reviews/2026-09-02-d107-operational-scope-wording-reconciliation-post-increment-review.md`

No other path may change.

## Affected components

| Component                   | Effect                                                        |
| --------------------------- | ------------------------------------------------------------- |
| Current project memory      | Uses exact operational-scope wording and records correction.  |
| Published historical record | Preserved byte-for-byte; clarified only by additive evidence. |
| D-107 result and readiness  | Unchanged and Blocked.                                        |
| Product/runtime boundaries  | None.                                                         |

## Interfaces and invariants

The reconciliation must preserve these three separate facts:

1. D-107 wrote documentation files and ran local repository-validation and
   gate processes.
2. D-107 changed no product/test source, dependency, configuration, capability,
   entitlement, IPC, workflow, hook, or script path.
3. D-107 ran no product-build, signing, Keychain/private-key, target-Mac, or
   state-changing external operation.

The additive correction must not be interpreted as a new D-107 evidence row,
an operational test, an eligible candidate, a D-102 waiver, or successor
authority.

## Implementation milestones

- [x] Confirm clean synchronized main and valid predecessor completion.
- [x] Freeze the complete five-location ambiguity inventory.
- [x] Add the correction and update mutable current-state summaries.
- [x] Verify prior D-107 plan, increment, review, and D-107 decision are
      unchanged.
- [x] Run documentation-tier validation and complete the gate.

## Security and privacy considerations

This change must contain no new target-derived or private certificate, identity,
account, key, signature, path, host, credential, provider, or personal value. It
adds no authority and must keep every operational boundary closed. The
historical report and decision remain immutable so the reconciliation cannot
conceal or relabel evidence.

## Test plan

- Review every changed sentence for the exact repository-governance versus
  operational distinction.
- Confirm the published D-107 plan, increment, report, and `DECISIONS.md` have
  no diff.
- Confirm no product, configuration, dependency, workflow, hook, skill, or
  script path changed.
- Run Markdown formatting/link, repository-health, secret-scan, whitespace,
  session-inventory, and post-increment checks.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- DECISIONS.md docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md docs/increments/personal-assistant-v0-key-use-containment-classification.md docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md
git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

`npm run verify`, `npm audit --audit-level=low`, and
`npm run tauri -- build --no-bundle` are Not run because this correction changes
only documentation and no dependency or executable boundary.

## Risks

- Rewriting the published report could invalidate or obscure historical gate
  evidence. Mitigation: preserve it byte-for-byte and use an additive record.
- Partial correction could leave current memory internally inconsistent.
  Mitigation: freeze the complete ambiguity inventory and update both mutable
  summary locations together.
- Broader wording could weaken D-107's operational prohibition. Mitigation:
  name the excluded operational classes explicitly and preserve Blocked
  readiness.

## Rollback or failure strategy

Before publication, remove only this increment's nine-path documentation diff.
Do not alter the predecessor report or gate state. If any historical path,
protected path, secret scan, link check, or required validation fails, stop and
record a truthful failed disposition rather than weakening the check.

## Decisions made

No new durable architecture, security, product, or repository policy decision
is required. Existing documentation-authority rules require an additive
correction.

## Discoveries

- The post-merge advisory named four documents. Required-chain review found a
  fifth equivalent phrase in `PLANS.md`; including it prevents split current
  state without expanding beyond the approved wording discrepancy.
- The published D-107 plan already uses precise operational wording and remains
  unchanged.

## Progress

- 2026-09-02: Owner approved the separately scoped documentation-only wording
  reconciliation.
- 2026-09-02: Created
  `codex/d107-operational-scope-wording-reconciliation` from clean synchronized
  `main` at `80dab5b` and began the exact gate.
- 2026-09-02: Independent reviews found and corrected the missing exact
  location inventory, one overbroad non-goal, one awkward phrase, and one
  owner/privacy-invariant mismatch before final validation.
- 2026-09-02: The exact nine-path documentation scope passed all required
  checks and completed with `PASS WITH ADVISORIES`; no successor became Ready.

## Acceptance criteria

- [x] Current memory explicitly states that documentation writes and local
      validation/gate processes occurred.
- [x] Current memory separately states that no product/build/signing/Keychain/
      target-Mac operational process or state-changing external action ran.
- [x] Published D-107 evidence and every outcome/readiness fact are unchanged.
- [x] The exact nine-path documentation diff passes all required checks.

## Final results

The discrepancy is reconciled additively. The two mutable current-state
summaries use precise operational wording, the changelog and current memory
carry the explicit correction, and the two ambiguous published D-107 evidence
files remain byte-for-byte unchanged. The exact required checks passed. Quality
result: `PASS WITH ADVISORIES`; next readiness: `Blocked`.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md` not required
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
