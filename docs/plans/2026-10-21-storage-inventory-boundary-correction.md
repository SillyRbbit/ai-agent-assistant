# Storage inventory boundary correction

Status: Correction implementation and scoped review complete; final report-input checks and gate/Stop receipts control completion.

## Objective and attribution

Correct only the three October21 read-only review findings in the existing
storage-inventory-checker detached checkout at4e3e94d. The seventeen original
candidate paths match their sealed source hashes. Original evidence has78 passing
identity records, a complete/valid gate and passing Stop; these records remain
historical, never rewritten to imply they tested these newly found defects.
Current owner approval explicitly covers these related corrections, tests,
review, documentation and supported gates. No failed history is imported or
claimed independent. The earlier Python3.9 repository incompatibility remains
unchanged; required repository validation uses3.12, checker compatibility uses both.

A recoverable complete source snapshot and raw gate-state snapshot precede edits.
The task-owned source/test and utility guide may be extended; no user attribution
is ambiguous. Keep the first milestone plan, readiness/final reports and all sealed
external evidence byte-identical. Preserve primary and other active checkouts.

## Acceptance

- aliases: Physical ancestor device/inode identities and conservative case/Unicode
  comparisons prevent candidate/output overlap through aliases. Retain no-follow
  traversal and dataless checks. Unknown/inaccessible scope refuses report creation.
- holds: All held and required sealed paths, plus context candidate/receipt inputs,
  are report exclusions. Preserve recognized exclusions even when another context
  field is malformed. Missing paths cannot clear a hold; aliases remain protected.
- schema: Evidence role must be a string before membership checks. Lists/objects
  yield fixed invalid-context reasons and safe deferred reports outside exclusions;
  no traceback, ignored hold or removal eligibility.
- tests: Meaningful synthetic regressions for actual Mac case aliases, physical
  identities, missing/Unicode scopes, held/sealed output and malformed roles; run
  affected checker tests on Python3.9/3.12 and full repository tests on3.12.
- preservation: Additive current-state/correction records, scoped review and normal
  finalization/Stop. Preserve original snapshots and all protected executable inputs.

## Components and boundaries

Adapt `scripts/storage_inventory.py` and its existing test module only. Add this
plan/new readiness/final review; update the utility guide and append observed
current-state notes to existing project-memory files. No package, hook, CI,
application, source snapshot, original plan/report, skill or prompt changes.
No branches, commits or publication. File counts are attribution, not ceilings.

Use bounded descriptor-relative scope observations, not realpath or link-following.
Compare directory/file identities across paths, including alternate existing
prefixes. Conservatively fold case/Unicode spellings for missing names without
assuming a case-sensitive volume makes similar scopes safe. Bind final scope
checks and exclusive creation to the same output-parent descriptor; recheck its
identity before creation. These are point-in-time guards, not an OS sandbox or
continuous protection against malicious same-user renames.

Use one conservative context-exclusion collector for valid and parsed-but-invalid
contexts. It extracts only validated absolute metadata paths and never grants
operation authority. Bad parsed context defers candidates; unreadable/ambiguous scope stops output; output must
pass its own independent scope/owner-only checks. Bound scope depth/path counts.
No output content, secret access, network, subprocess, process signal, permission
change, artifact transfer/restore/removal, schedule or automation is introduced.
Test fixture setup/teardown is synthetic only; never use actual QA originals.

## Validation and reuse

Preparation and final: docs:check, repository:check, security:scan, whitespace and
session checks. Freeze new passing readiness before normal schema2 begin. Run
focused tests during correction and final affected tests once stable. Reuse
unchanged hook291 and frontend/Rust/build stages from the sealed original evidence
only after input hashes and command/result/environment provenance match. Reuse
all293 product inputs and original full verification lineage; this is composite
coverage, not a freshly executed npm run verify. No native QA or build replay.
Assess architecture, security, code-health, debt and readiness. Every final delta
needs attribution. Final report/actual gate/status/Stop records determine completion.

## Risks, recovery and stop conditions

Filesystem metadata can become unavailable or change concurrently: stop affected
scope, never treat it as distinct/safe. False-positive retention is acceptable;
unsafe report creation is not. Unknown dependencies and receipt assertions still
cannot grant removal eligibility. Original failure results remain unchanged.
Recover from the verified source/state snapshot only with attributable in-scope
edits; no reset, stash, cleanup, automatic rollback or broad restoration.

External evidence: storage-inventory-boundary-correction-20261021 under this chat's
visualization directory. Original storage-inventory-checker-20261021 stays sealed.
No worktree archival, QA artifact cleanup, new runtime or successor milestone.

## Progress

- [x] Instructions, source/tests, current Git and original seal inspected.
- [x] Recoverable source/state and other-checkout identities recorded.
- [x] New readiness and supported admission.
- [x] Corrections and53 synthetic regressions passed on both checker interpreters.
- [x] Affected validation, review and additive documentation; actual gate/Stop receipts remain external.

## Review discovery — ambiguous context scope

Parsed malformed role/schema fields preserve recognized exclusions and allow safe
deferred reports outside them. Duplicate-key/unparseable or unreadable context
cannot establish its held scopes; it now stops report creation with the fixed
context_scope_unavailable category. This closes the related possibility of hiding
a hold behind a duplicate key. It is an affected unsafe-scope stop, not discarded
context or renewed operation authority. A synthetic regression covers both cases.

Noncanonical or mistyped context scope paths likewise cannot be dropped from
output exclusions. They stop report creation. Invalid explicit candidate syntax
also remains in the output guard rather than being treated as a safe omission.
A synthetic trailing-separator regression checks both paths without writing.
