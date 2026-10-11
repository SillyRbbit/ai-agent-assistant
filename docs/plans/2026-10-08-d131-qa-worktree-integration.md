# Selective D-131 governance integration

Status: Governance integration verified with advisories; action milestone remains active.

## Objective and authority

Port the completed Desktop D-130/D-131 general milestone route and D-132 correction
without replacing the QA gate wholesale or changing product/runtime boundaries.
Owner explicitly authorized this integration, snapshots, focused tests, policy,
skills, templates and workflow guidance. The active legacy milestone stays intact.

## Baseline and attribution

QA HEAD `bc12776412c717613de1fdc42c38bf3312493726`; Desktop source
`a9f2e5e2a5d94dc09b75a28a19bc1114597e65b3`. Reuse Desktop D-131 review and retained
complete/valid/Stop evidence and D-132's complete/valid record. The source commit
contains unrelated historical work, so no wholesale copy or cherry-pick is used.
Snapshot `/private/tmp/cortexa-d131-integration-rin1mbe4` preserves 1114 paths, raw
active state, worktree pointer and actual owning index. Existing 63-path candidate
and artifacts are historical inputs, not erased or rebound to changed governance.
Ordinary same-task begin passed and left state byte-identical before editing.

## Implementation and exclusions

Add `.codex/hooks/lifecycle_closure.py`; selectively integrate general admission,
report criteria and lineage into the existing `post_increment_gate.py`; extend
its existing tests without removing old tests. Retain lifecycle_acceptance.py,
its tests, D-098 and D-133 identities, original fingerprint encoding and hooks.
Use Git-resolved index ownership for closure backups with strict byte matching.
Refuse mixed routes/lineages and conversion of an active legacy state.

Add authoritative D-131 guidance to AGENTS, MASTER_PROMPT, ENGINEERING_GUIDE,
CODE_REVIEW, six affected skills, three templates and three workflow documents.
Update existing eight milestone memory documents additively. This plan and its
review attribute only governance changes. No product, dependency, authentication,
permission, live QA, paid request, install, Git integration or publication.

## Acceptance and validation

- [x] Existing legacy/D-133 tests preserved and passing.
- [x] General closure/admission and schema-2 objective/path attribution pass.
- [x] Protected paths, unrelated/missing attribution, tampering and false completion reject.
- [x] D-132 absence handling and actual linked-worktree index/closure tests pass.
- [x] Active state unchanged; no conversion or hidden completion; Stop remains blocked.
- [x] Product, artifact, historical evidence and unrelated Desktop bytes preserved.
- [x] Governance documentation/repository/security/whitespace/session checks pass.

Reuse unchanged product results; run focused then full hook and repository suites,
applicable documentation/security/whitespace checks. No product build/live QA is
needed for this governance-only integration. Record all actual outcomes. Review
architecture, security, code health and preservation before declaring integration
verified. Overall action milestone remains incomplete; no finalization command.

## Risks and recovery

Gate routes must remain distinct; overlapping history must not be silently adopted.
Local evidence is not same-user authentication. Preserve exact original test methods
and historical report bytes. Recover only this attributed delta from the snapshot
under explicit owner direction; no automatic rollback or cleanup. Stop on unresolved
attribution, gate rejection or required expansion beyond governance integration.

## Recorded outcome

145 hook and95 repository tests pass. Retained system-Python repository failure
was resolved by the documented Python3.12 route without code changes. Final
checks and preservation receipts are in the external evidence directory. The
standalone review assesses only governance integration, not action completion.
Full Stop remains blocked as required; no live state or historical seal changed.
