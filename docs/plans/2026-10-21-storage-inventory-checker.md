# On-demand storage inventory and eligibility checker

Status: Implementation and scoped review finished; final documentation checks and actual gate/Stop receipts control completion.

## Objective and current evidence

Implement only a standalone, on-demand, standard-library Python repository utility.
The owner explicitly selected this milestone on October 21. Explicit candidate
paths are required; no discovery of other artifacts or task completion trigger.
Published main f08f4ac06c044fe5c17d10c09d4f65d3cf0fb520 was inspected through
GitHub. This detached checkout uses local 4e3e94d with the identical published
7c35e1169d9b6ef178d7e5a86c5eae904e1def8e tree, avoiding artifact acquisition
and any branch/ref change. The older dirty primary, original documentation
candidate, publication checkout and active product tasks are preserved.

Read the canonical END_SESSION inventory/holds/recovery procedure and retained
October21 allowlist, archive index, source manifest and VERIFIED receipt formats.
They are historical evidence only, never execution inputs or renewed releases.
Prior completed cleanup is not replayed. Known HP, M1/M2, provider and product
failures remain in their owning records; this independent local maintenance
utility resolves none of their acceptance and needs none of their missing live
observations. No failed gate is imported, reopened or bypassed.

## Acceptance contract

- C1: Explicit absolute candidate inputs only; bounded descriptor-relative no-follow
  inspection, stable identities, exact membership digest, file hashes/modes,
  device/inode/link counts, symlink targets and nonadditive allocation observations.
- C2: Exact supplied ownership assertions remain unverified assertions. Protected
  checkouts/Git, dependencies, application data, authentication, evidence, source,
  cloud sync roots and explicit holds override optional metadata. Unknown facts,
  membership drift, inaccessible paths and unsupported types retain or defer.
- C3: Optional exact hash-bound local evidence is inspected as data only. Recognize
  retained payload/recovery receipt identity and comparison fields; report missing,
  invalid, mismatched or recorded-pass evidence without claiming current cloud
  availability, source coverage, actual restore or operation authorization.
- C4: Private new local JSON report only, exclusive creation and restricted initial
  mode, safe output ancestors and no candidate/output overlap. No content, raw
  external error or evidence body in stdout/report; metadata stays outside Git.
- C5: Meaningful synthetic filesystem-boundary, race, evidence, budget and privacy
  tests; full required verification coverage with unchanged-input stage provenance.
- C6: Scoped review, additive documentation, supported admission/finalization and
  actual Stop. Preserve original inputs/evidence and truthful partial outcomes.

## Design and boundary

`scripts/storage_inventory.py` and `scripts/tests/test_storage_inventory.py` are
new. Documentation: this plan/readiness/final reports, utility guide, END_SESSION
reference and additive HANDOFF/STATUS/NEXT/PLANS/DECISIONS/CHANGELOG/testing/security/
troubleshooting records as applicable. No package, hook, workflow, product or
configuration edit is expected. File counts are informational; every delta needs
criterion attribution. Baseline archive/hashes are external to the checkout.

CLI: repeat `--candidate` absolute paths, optional `--context` exact local JSON,
required `--report` new absolute local path. Context supplies per-path owner/task
assertions, exact holds and typed local evidence references with expected SHA256;
it cannot clear a hold or grant authority. Do not echo arbitrary assertion text.
Report candidacy as `retain` or `defer`, with fixed reason codes. This bounded
checker deliberately never emits removal eligibility: current cloud availability,
process/runner/task consumers, ownership/release and full source/archive coverage
require human/agent review outside this offline observation. Directory/symlink/
hardlink observations are dependencies, not absence-of-use proof.

Walk each absolute ancestor without symlinks; inspect only explicit candidate
subtrees, never follow symlinks or cross devices. Stop content hashing on detected
protected members; no scanning outside candidates, shell, subprocess, network,
credential API, app execution or deserialization with side effects. Bound input,
entries, total hashed bytes and depth; partial work remains incomplete. Check
entry identity before/after reads, directory identity/membership before/after,
and candidate ancestry again. Descriptor traversal reduces races, but no snapshot
or ongoing quiescence is promised. A checker must be rerun before a separately
approved future action. It cannot guarantee access-time neutrality of reads.

Private output requires an existing owner-only non-synced parent outside inputs,
candidates, repository and protected stores. Create 0600 exclusively; do not chmod
existing resources. Failed writes may leave a private incomplete file; never delete
or overwrite it automatically. Do not store reports in repository or shared logs.

## Non-goals and authorization

No archive operation, cloud query/transfer, restore, deletion, process signal,
permission/credential change, scheduling, automation, product action/tool, generic
executor, hook modification, Git branch/ref operation, commit or publication.
Implementation completion grants no archive/removal authority. No release of an
existing local hold. Tests use only disposable synthetic fixtures, not real QA
artifacts; the checker itself has no removal API. Test fixture lifecycle is normal
test-owned setup/teardown, never original-artifact cleanup.

## Validation and resources

Preparation: docs:check, repository:check, security:scan, whitespace, session check.
Before implementation freeze readiness and use existing schema-2 begin. Focused
unittest tests cover no-follow ancestors/leaves, external links/hardlinks, mounted
boundary simulation, FIFO/socket, protected entries, drift, bounded resource use,
malformed/forged/missing receipts, output safety and independent partial results.
Run affected checker tests freshly on Python3.9 and3.12; full repository tests on Python3.12, documentation,
security, whitespace and session checks. Obtain full npm run verify coverage:
reuse only hash-matching product/dependency/native/frontend stages from retained
Vitest security repair, with exact command/result/environment receipts; fresh
repository/hook/format checks cover changed tooling. Do not call this composite a
fresh exit-zero full run. No builds or native acceptance solely for new Python.
Any missing relevant stage evidence blocks that conclusion; do not silently waive.

No installs or large caches/builds; use retained read-only formatter binaries.
Inventory defaults: at most64 candidates,10,000 total entries/run, depth64,
64MiB total file hashing/run and1MiB per context/evidence input; serialized report capped at16MiB. Files beyond
the budget remain unhashed/incomplete. Reports remain bounded by the same limits.
No APFS physical reclaim estimate; record same-volume available-space observation
only, with zero artifact operations and no reclaimed-space claim.

## Recovery, risks and progress

Recoverable baseline and preservation identities are in the external
`storage-inventory-checker-20261021` evidence directory. No automatic rollback.
Stop affected candidates on drift, missing evidence, unsafe/inaccessible scope;
complete independent candidate reports. Whole-run invalid context defers all
candidates rather than ignoring holds. Platform denial never means absent.
Same-user evidence is not authentication; hash matching is byte identity only.

- [x] Instructions, Git, published workflow and retained formats inspected.
- [x] Frozen readiness and normal admission.
- [x] Implementation and37 adversarial tests; both interpreters and supported full repository tests pass.
- [x] Verification, scoped review, documentation and gate preflight; actual final gate/Stop receipts are external.

## Observed validation adjustment

The new checker passes on Python3.9 and3.12. An additional whole-repository
Python3.9 diagnostic failed in unchanged repository_health.py:1094, which uses
zip(strict=True), unsupported before3.10. Its170 subcase errors and original
exit1 are retained. This is an existing environment incompatibility outside the
checker objective, not a failed checker criterion or a reason to edit protected
validator code. Full repository completion uses Python3.12, which passes. The
initial broad two-interpreter repository wording is corrected by this additive
observation; no failed result is relabeled or silently waived.

## Current verified results

37 focused filesystem tests pass on Python3.9 and3.12;137 repository tests pass
on3.12;291 unchanged hook tests and formatting pass. Retained payload and separate
recovery-record receipt formats were inspected by the new parser without opening
archives or replaying recovery. A private synthetic CLI report correctly defers an
ordinary candidate, retains an exact hold and defers a missing path; fixture hashes,
identities and modes remain unchanged. Same-volume available space is measured
before/after with no attribution or APFS reclaim estimate.

All293 product inputs match sealed final security-repair verification evidence;
the published baseline delta from that source is Markdown only. Reused frontend,
Rust and no-bundle stages preserve their command/result/input/environment provenance.
Affected tool/repository/hook/format stages run freshly. This is composite full
coverage, not a fresh full invocation. Whole-repository3.9 failure remains an
explicit advisory, separate from passing checker3.9 acceptance. Final gate/status/
Stop and seals are recorded externally after the final input checks. No successor.

Final security review found a macOS placeholder hydration edge: SF_DATALESS files
and ancestors must be rejected before opening. Added focused regression tests and
restricted report roots to supported local temporary/Codex visualization locations.
No placeholder was read in actual acceptance; only synthetic flag injection is used.
Fresh affected tests/checks supersede earlier source bindings; hook/product stages
remain unchanged. Older passing receipts stay preserved with their original inputs.
