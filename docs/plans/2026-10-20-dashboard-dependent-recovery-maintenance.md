# Dashboard dependent-recovery maintenance — frozen design

Date: 2026-10-20. Objective: dashboard-dependent-recovery-maintenance.
This owner-approved pre-admission maintenance has no ordinary begin. It must not
execute closure or real recovery admission, transfer dashboard product files, or
publish. Existing failure remains immutable. This design is frozen before code;
later results belong in a separate report, not revisions of this design.

## Task binding and scope

Predecessor: usage-dashboard-publication-preparation.
Successor: usage-dashboard-publication-recovery.
Branch: codex/usage-dashboard-publication. Bound baseline:
faa9f936b78a600c5d5f0dc2d587032fb256d5bf.
Original state SHA-256:
f0b0f7821ec9747f842e44e7bea8d43e19eacf7fbd4f51823258919e8eca0f30.
Original report SHA-256:
a17ba967b0800691004ab7ac4787a42652f3e0db2c2d71be52331f6ea5a72453.
The exact owner-selected checkout is bound by a path digest, not a published
absolute owner path. No caller-selected predecessor or generic recovery profile.

Code scope: new lifecycle_dashboard_publication.py and focused test module;
narrow post_increment_gate.py dispatch/state/report/finalization/Stop integration.
Additive AGENTS, ENGINEERING_GUIDE, DECISIONS and project-memory documentation,
this plan and a new maintenance review. Existing closure, VPS, D-133 modules and
their behavior stay unchanged. No product, dependency, security capability or
hook timeout changes. Governance changes are a separate future publication scope,
not silently part of the accepted dashboard manifest.

## Proposed contract

Add an explicit mutually exclusive --dashboard-publication-request option to
ordinary begin dispatch. It accepts only request schema 1 for the fixed successor.
Request fields: schema_version, increment_id, root, branch, head,
workspace_fingerprint, index_sha256, closure_id, owner_authorization,
milestone, allowed_paths, plan, report, readiness_report, readiness_increment,
obligations. Reject unknown fields and duplicate JSON keys.

Owner authorization has separate admission and execution entries, both bound to
current DECISIONS bytes. Neither maintenance approval nor a closure request grants
them. Require the existing verified closure receipt for the exact original failed
state, original report and schema-2 admission, with no omitted historical lineage.
Closure must identify this separately passing maintenance report and retain all
nine original unresolved obligations. The frozen maintenance report must contain
the existing full-verify and named-review evidence. No acceptance replacements.

Preserve all four original criteria/descriptions and exclusions, and carry all
nine obligations as required criterion mappings. Fresh readiness must pass current
checks and both named reviews. Remaining product transfer/integration acceptance
is carried forward, not falsely required to be complete before admission.
The original failed readiness is protected and cannot serve as new readiness.

Freeze exact allowed paths within the attributed dashboard product/documentation
inventory plus the new recovery plan/final report. All 21 product paths are bound
to accepted-candidate hashes. Preserve existing corrected historical summaries,
frozen reports, dependency/security files and governance implementation. No deletion
authority. The fixed plan/report names are prospective; final reports must exist
and satisfy normal strict report validation before completion.

## State and interruption behavior

Use a distinct schema-6 dashboard_publication_lineage. Never reinterpret schema 5,
general closure lineage or D-133. Seal the request, baseline inventory, raw owner
decision and exact closure reference in a task-specific ignored store. Exclusive
receipt and consumed-pointer creation rejects replay and interrupted attempts.
Check fingerprint, HEAD, index, decision and original state again before effect.
Any orphan receipt or consumed pointer prevents automatic retry. No rollback or
implicit resumption after partial admission.

Revalidate lineage, protected input hashes and scope for finalization, failure,
status and Stop. All inherited required checks plus full verification and explicit
per-obligation evidence checks must remain required. Passing completion additionally
requires all 21 accepted product hashes and all criteria verified. Failed reports
may retain blocked criteria; they never create a completion marker. A new failed
dependent attempt is immutable and requires a new owner recovery decision.
No automatic successor follows. Same-user hash integrity is not authentication
against a malicious operator, an OS sandbox or provider authority.

## Preservation and validation order

A verified fresh snapshot captures 1,123 current source/document files, actual
index, raw lifecycle and original failed evidence before documentation/code edits.
The accepted documentation correction, its terminal snapshot, accepted dashboard
and all other work remain retained. Reversal requires later explicit attribution;
never reset/clean, restore live state or delete evidence.

First run synthetic positive/negative tests: valid closure-bound admission;
complete/failure/Stop; wrong task/root/branch/HEAD/index/fingerprint; missing closure
or approval; stale readiness; omitted obligations/criteria/checks; altered product
hash/scope; malformed/duplicate JSON, unsafe paths; evidence drift; replay and
interruption at receipt/pointer/state writes; old-route compatibility. Synthetic
Git/state operations stay inside temporary fixtures, never the real checkout.

Only after focused viability, inspect existing dependencies and allocation-aware
disk headroom. No installations/downloads. Obtain applicable full verify and
documentation/repository/security/whitespace/session evidence, using supported
unchanged-stage reuse with exact provenance. Stop on missing dependency/access,
unsafe resources, ownership ambiguity or broader authority. Do not hide a required
unverified stage. Freeze a truthful maintenance report and stop for owner review.

Real closure and recovery admission remain separately unapproved. No actual
Stop/finalize/close transition may be used to pretend maintenance completed the
original failed increment. Any automatic Stop rejection must remain visible.
