# D-107 post-D-111 decision-lineage reconciliation

Status: Complete (`PASS WITH ADVISORIES`) — proposals pending owner acceptance
Owner: Project owner
Last updated: 2026-09-02
Baseline: `5770c90a3601eee41883a2934863f9c2a2c3e2f3`
Branch: `codex/d107-post-d111-decision-lineage-reconciliation`
Increment: `d107-post-d111-decision-lineage-reconciliation`

## Goal

Reconcile four already published, documentation-only negative D-107 contract
outcomes with the durable decision ledger and current project memory. Add
non-colliding proposed decisions D-113 through D-116, identify the historical
numbering and closeout discrepancies without rewriting them, correct stale
PR #102 queue wording, and preserve all ten contracts remaining unproved under
D-108's additive current interpretation and `Blocked` readiness.

This is an additive governance repair. It is not a rerun of any predecessor,
does not retroactively validate an unavailable completion marker or command
output, and does not change any product or security capability.

## User-visible outcome

None. No application behavior, interface, data, dependency, configuration,
permission, credential, signing state, or external system changes.

## Scope

1. Preserve the four published plan/increment/review triplets byte-for-byte.
2. Preserve GUI D-112 byte-for-byte as the accepted Graph-local wheel decision.
3. Append four closed negative decision proposals:
   - D-113: `nonexport_contract_not_accepted`;
   - D-114: `algorithm_contract_not_accepted`;
   - D-115: `interaction_denial_not_accepted`; and
   - D-116: `deadline_contract_not_accepted`.
4. State that the historical private-key D-112 result label, fixed-algorithm
   D-112 predecessor, interaction-denial D-113 predecessor, and hard-deadline
   D-114 predecessor are noncanonical lineage references. Map those references
   through the proposed entries; do not supersede or control accepted evidence
   while the entries remain Proposed.
5. Reconcile the twelve authorized current-memory, architecture, security,
   testing, roadmap, and troubleshooting documents.
6. Correct current wording that still treats already merged PR #102 as awaiting
   squash merge.
7. Preserve D-097, D-107's immutable 8/11 factual record, D-108's additive
   9/10 interpretation, all ten unproved contracts, and `Blocked` readiness.

## Explicit non-goals

- No edit to any of the twelve historical predecessor artifacts or to D-112.
- No source, test source, dependency, lockfile, configuration, workflow, hook,
  capability, CSP, permission, entitlement, runner, or toolchain change.
- No Keychain, certificate, private-key, signing, Apple/Xcode, provider,
  product-system, target-derived, network, credential, or external-system work.
- No claim that report hashes prove commands ran, that overwritten predecessor
  markers are recoverable, or that the current dirty original checkout is clean.
- No reduction of the ten blockers, admission of the D-107 candidate, or
  readiness for operational work.
- No late-result-rejection assessment or plan in this increment.
- No correction of independently stale V0-2 direction wording or inconsistent
  historical/current acceptance counts; those require separate authorization
  and source-current evidence.
- No commit, push, merge, release, publication, or successor start.

## Existing behavior and constraints

- D-107 records `private_key_nonexport_contract`, `fixed_algorithm_contract`,
  `interaction_denial_contract`, and `hard_deadline_cancellation_contract` as
  `contract_unproven`.
- D-108 changes only the D-102 applicability interpretation, producing the
  current additive 9 documented / 10 unproved view.
- D-109 through D-111 add three durable negative decisions without reducing
  the blocker count.
- D-112 is already the unrelated Graph-local wheel decision.
- Commits `238fc5a`, `ada57ce`, `38af59a`, and `5770c90` each published only a
  plan, increment, and review for their respective negative result. None added
  a durable decision or current-memory reconciliation.
- The existing checkout remains intentionally untouched with 30 untracked
  ` 2.*` paths. This increment runs in a separate clean linked worktree.

## Current-state evidence

- Clean linked worktree and branch created from local `main` and local
  `origin/main` at
  `5770c90a3601eee41883a2934863f9c2a2c3e2f3`; ahead/behind was `0/0`.
- No network fetch ran because external-system work is prohibited. “Synchronized”
  therefore means synchronized with the existing local tracking reference.
- `git fsck --no-progress` exited zero with only recoverable dangling objects.
- Repository-pinned Node 26.3.0, npm 11.16.0, Cargo 1.90.0, and Rust 1.90.0 are
  active on macOS 26.6.
- `npm ci --ignore-scripts --offline` installed the locked dependencies with
  zero vulnerabilities and no network access.
- Baseline `npm run docs:check` passed.

Frozen SHA-256 preservation values:

| Historical artifact              | SHA-256                                                            |
| -------------------------------- | ------------------------------------------------------------------ |
| Private-key non-export plan      | `780d2058970e703cc87cf2b9d699ad2101b479e8e9fcb66ed6fe5b64899f21d6` |
| Private-key non-export increment | `d05bd5d31e4c56e5973550e1da83fb447817de56a0e8ca497713093444721725` |
| Private-key non-export review    | `b2c08b14e3acbf0818d009db15c23109815d0d5bbebf1611b732fefe0f7ad6e4` |
| Fixed-algorithm plan             | `f17897ae9e89cb892ec1fd96fe87e81d0d575b950f9a29cddbe96f0cf46f1f17` |
| Fixed-algorithm increment        | `290eb2c2761a14ab2aaf8cbe5d7470c3d58cf7a1c7413aa826b073c809ee5b89` |
| Fixed-algorithm review           | `0431c70a25bbe79a91ecae30d7413f80af2d24512a91a577fa0952cdeadb249a` |
| Interaction-denial plan          | `129071aabbdb6708744dd9c700ba34af2e2bb74c9d443c19bde83423a580cd07` |
| Interaction-denial increment     | `3c5fea8b4692191138fba79540ee94826293ba27300a1929d489390e55fd9569` |
| Interaction-denial review        | `13ccd9c9d759aa4c98914729e2b8bf75525f822a0fecfe27d5246fcc8e10675a` |
| Hard-deadline plan               | `b7c6b0cbc1b1af7da52eba67be9f965e42eed50981674ae24580694836a19558` |
| Hard-deadline increment          | `df0774db9bceb05db7d5a9e30eabe6fe77d5fbb50c5fc41110d983ef234ae483` |
| Hard-deadline review             | `03c634ee93e3d852b0c7a3cfc09733891050c9123c8307d01415abe846afbbea` |
| GUI D-112 decision block         | `904fbe07e8e843462f32303a0e4072676d1a61b69c53f7d95d09ebe9e3c8315c` |

## Files expected to change

Exactly these fifteen documentation paths may change:

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
13. `docs/increments/d107-post-d111-decision-lineage-reconciliation.md`
14. `docs/plans/2026-09-02-d107-post-d111-decision-lineage-reconciliation.md`
15. `docs/reviews/2026-09-02-d107-post-d111-decision-lineage-reconciliation-post-increment-review.md`

Any other tracked or gate-visible untracked path is a stop condition.

## Affected components

- Durable governance decision history.
- Current architecture, security, testing, project status, roadmap, handoff,
  plan index, changelog, next-step, and troubleshooting memory.
- Documentation-only completion evidence for this reconciliation.

No runtime, Tauri, WebView, provider, storage, filesystem, tool, approval,
execution, or device component is affected.

## Interfaces and invariants

The four disposition policies remain conceptual governance vocabulary only:

- `PrivateKeyNonExportPolicyV1` proposes
  `nonexport_contract_not_accepted`;
- `FixedAlgorithmPolicyV1` proposes `algorithm_contract_not_accepted`;
- `InteractionDenialPolicyV1` proposes
  `interaction_denial_not_accepted`; and
- `HardDeadlineCancellationPolicyV1` proposes
  `deadline_contract_not_accepted`.

They are not Rust, Tauri, WebView, D-100 evidence, configuration, or runtime
interfaces. Each proposed result means the current reviewed repository record
does not establish the full positive contract. Missing, ambiguous,
contradictory, or drifted facts remain fail-closed. None supplies fallback,
retry, operational authority, or proof about target state.

D-113 is the first available non-colliding decision number after GUI D-112.
D-114 depends on D-113, D-115 depends on D-114, and D-116 depends on D-115. The
published historical documents remain immutable evidence of the earlier
incorrect labels and incomplete closeouts.

## Implementation milestones

- [x] Record this exact owner-approved plan and begin the gate from the clean
      linked worktree.
- [x] Append proposed D-113 through D-116 without changing D-112 or predecessor
      artifacts.
- [x] Synchronize all twelve current-memory documents and correct stale PR #102
      queue wording.
- [x] Confirm the exact fifteen-path change set and historical hashes.
- [x] Run focused documentation checks and complete repository verification.
- [x] Complete independent architecture, security, code-health, debt, and
      readiness reviews.
- [x] Record an exact post-increment report and valid completion marker.
- [x] Stop with next-increment readiness `Blocked` for owner review.

## Security and privacy considerations

The primary threats are governance laundering and accidental authority drift:
renumbering history, treating a negative result as a proved positive contract,
claiming old commands or markers are recoverable, reducing the blocker count,
or using reconciliation as permission for operational work. Additive records,
closed negative vocabulary, exact historical hashes, an exact path allowlist,
and explicit Not-run classifications contain those risks.

No secret, credential, account identifier, certificate material, private-key
material, provider data, target-derived data, or personal path belongs in the
tracked change set or report.

## Test plan

- Validate Markdown formatting, links, repository policy, and secret scanning.
- Run complete `npm run verify` because this is cross-cutting security
  governance, even though no product source changes.
- Compare the complete changed-path set with the exact fifteen-path allowlist.
- Recompute and compare all twelve historical artifact hashes with the frozen
  baseline values.
- Verify D-112 remains byte-identical to baseline and proposed D-113 through
  D-116 are unique, ordered, closed negative entries.
- Search all twelve current-memory documents for the four proposed decision
  entries, ten-blocker preservation, and `Blocked` readiness.
- Confirm source, dependencies, configuration, workflows, hooks, and protected
  paths have no diff.
- Inspect the session inventory and complete independent reviews.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
npm run verify
git diff --check
git status --short --branch
git diff --name-only 5770c90a3601eee41883a2934863f9c2a2c3e2f3 --
python3 -c 'from pathlib import Path; import sys; sys.path.insert(0,".codex/hooks"); from common import changed_paths; expected=frozenset(("ARCHITECTURE.md","CHANGELOG.md","DECISIONS.md","HANDOFF.md","NEXT_STEPS.md","PLANS.md","PROJECT_STATUS.md","ROADMAP.md","SECURITY.md","SECURITY_CHECKLIST.md","TESTING_GUIDE.md","TROUBLESHOOTING_LOG.md","docs/increments/d107-post-d111-decision-lineage-reconciliation.md","docs/plans/2026-09-02-d107-post-d111-decision-lineage-reconciliation.md","docs/reviews/2026-09-02-d107-post-d111-decision-lineage-reconciliation-post-increment-review.md")); actual=frozenset(changed_paths(Path.cwd())); print("\n".join(sorted(actual))); raise SystemExit(0 if actual == expected else 1)'
git diff --exit-code HEAD -- docs/increments/d107-private-key-nonexport-contract-decision.md docs/plans/2026-09-02-d107-private-key-nonexport-contract-decision.md docs/reviews/2026-09-02-d107-private-key-nonexport-contract-decision-post-increment-review.md docs/increments/d107-fixed-algorithm-contract-decision.md docs/plans/2026-09-02-d107-fixed-algorithm-contract-decision.md docs/reviews/2026-09-02-d107-fixed-algorithm-contract-decision-post-increment-review.md docs/increments/d107-interaction-denial-contract-decision.md docs/plans/2026-09-02-d107-interaction-denial-contract-decision.md docs/reviews/2026-09-02-d107-interaction-denial-contract-decision-post-increment-review.md docs/increments/d107-hard-deadline-cancellation-contract-decision.md docs/plans/2026-09-02-d107-hard-deadline-cancellation-contract-decision.md docs/reviews/2026-09-02-d107-hard-deadline-cancellation-contract-decision-post-increment-review.md
python3 -c 'from pathlib import Path; import subprocess; baseline=subprocess.check_output(["git","show","HEAD:DECISIONS.md"]); current=Path("DECISIONS.md").read_bytes(); raise SystemExit(0 if current.startswith(baseline) else 1)'
git diff --exit-code 5770c90a3601eee41883a2934863f9c2a2c3e2f3 -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

`npm audit --audit-level=low`, target-Mac identity/signing/launched-product/
device-effect checks, Keychain, certificate, private-key, Apple/Xcode,
provider, product-system, and external-system checks are not required and must
be recorded `Not run`.

## Manual gates

- Confirm all twelve predecessor artifacts match their frozen SHA-256 values.
- Confirm D-112 and its GUI meaning are unchanged.
- Confirm the complete change set is exactly the fifteen authorized paths.
- Confirm every new decision is negative, non-authorizing, and preserves all
  ten blockers.
- Confirm no sensitive, target-derived, operational, or external evidence is
  present.

## Risks

- Decision-number collision if the baseline changes before publication.
- Historical-evidence mutation masked as cleanup.
- Treating syntactically valid predecessor reports as complete command proof.
- Stale queue wording causing a later session to repeat an already merged PR.
- Unrelated documentation drift being pulled into this bounded reconciliation.

## Rollback or failure strategy

Before publication, retain the isolated worktree and report any failure without
discarding evidence. Do not reset, rebase, clean, stash, or edit the original
checkout. If a required check fails or scope drifts, stop and either correct
only within the exact owner-approved scope or record a truthful terminal
failure. Any later rollback requires an explicit owner-approved documentation
revert; historical artifacts remain immutable.

## Decisions made

- Use four new durable decision entries rather than rewriting or reusing D-112.
- Propose the four historical negative outcomes as non-colliding durable entries
  without treating them as accepted policy or their predecessor gates as
  retroactively complete.
- Keep every D-107 contract count and readiness result unchanged.
- Keep independently stale, unrelated documentation outside this increment.

## Discoveries

- The original checkout is locally ref-synchronized but not clean; the linked
  worktree is clean and isolates all 30 existing untracked paths.
- The predecessor report manifests match their three-file commits, but ignored
  single-slot marker history cannot independently validate the first three.
- The current hard-deadline marker is invalid only in the original dirty
  checkout; this reconciliation neither repairs nor replaces it.

## Progress

- 2026-09-02: Owner authorized the exact documentation-only reconciliation,
  clean linked worktree, branch, gate, validation, independent reviews, and
  completion workflow. No publication or operational work is authorized.
- 2026-09-02: Clean branch established from local ref-synchronized baseline;
  offline scripts-disabled dependency installation and baseline documentation
  validation passed.
- 2026-09-02: Began the active gate, appended proposed D-113 through D-116,
  and reconciled the authorized current-memory set without touching historical
  triplets or GUI D-112.
- 2026-09-02: A later owner acceptance review identified incomplete explicit
  D-096/D-099 lineage, overly broad fixed-algorithm capability wording, and
  ambiguous D-107/D-108 count wording. The owner authorized only those bounded
  same-increment corrections and complete revalidation; no proposal was
  accepted and no successor was authorized.

## Acceptance criteria

- [x] Exactly fifteen authorized documentation paths change.
- [x] The twelve historical predecessor artifacts and GUI D-112 remain
      byte-identical to baseline.
- [x] Proposed D-113 through D-116 record the four historical closed negative
      outcomes and identify the historical lineage discrepancies.
- [x] All twelve current-memory documents consistently preserve the ten
      blockers and `Blocked` readiness.
- [x] Stale PR #102 pending-merge instructions are removed from current memory.
- [x] All required checks pass and all prohibited checks are explicitly
      `Not run`.
- [x] The post-increment report and completion marker validate.
- [x] No successor begins automatically.

## Final results

`PASS WITH ADVISORIES`. The exact fifteen-path reconciliation passes
documentation, repository, security, complete verification, immutable-history,
exact-path, protected-path, independent-review, session, and gate checks.
Proposed D-113 through D-116 remain non-controlling pending owner acceptance;
all ten contracts remaining unproved under D-108's additive current
interpretation and `Blocked` readiness remain unchanged.

## Documentation updates

- [x] `ARCHITECTURE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md`
- [x] `HANDOFF.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `PROJECT_STATUS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `TESTING_GUIDE.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] Increment record and consolidated review
