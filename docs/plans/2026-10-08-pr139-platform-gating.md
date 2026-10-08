# PR139 platform gating and diagnostic publication — 2026-10-08

## Objective and authority

Owner authorizes consistent macOS gating in approvals/manager.rs and types.rs,
local verification and one Conventional Commit/non-force push updating PR139.
Required HEAD 3296cc41387ea1ef3e40c0383679fbeefa1b9e1c; live main
fc239b6161d9379915891e04f7c56fcaa9c4971c. Use ordinary D-134 schema-2 admission.

## Scope and invariants

Gate only the private isolated-change constructor, decision/subject variants and
dependent match arms. Keep public preview/provenance metadata, native approval
binding, non-macOS rejection, strict warnings and all runtime validation intact.
Split the resolution OR pattern for separate conditional arms. No suppressions,
fixture behavior or HOME change, raw Git output, gate changes or dependencies.
The inherited Git helper diagnostic stays byte-identical. Seven current-state
root documents may be appended; create this plan and two independent reports.

## Acceptance and validation

- Verify exact required local/remote refs, snapshot and completed predecessor.
- Admit normally before implementation; preserve old raw state and reports.
- Attribute only the two Rust files and additive current documentation.
- Run affected Mac Rust format, strict Clippy and all-target tests; obtain native
  compilation coverage and reuse only unchanged frontend/governance stages.
- Record composite verification provenance; do not call reuse a new full run.
- Verify fixed diagnostic preservation, public metadata and non-macOS denial.
- Run docs/repository/security/whitespace/session/schema/preservation and reviews.
- Finalize local successor truthfully, full Stop, freeze exact bytes/modes.
- Commit exact attributed delta with required HEAD as sole parent, push without
  force, inspect newly triggered exact-head CI without rerunning historical jobs.

## Evidence and known limitations

Prior diagnostics and one representative Mac fixture test passed. The old Mac
CI Git cause remains unknown; its final expression already returns TestResult.
Linux reported unused create_isolated_change/ApprovalDecision::IsolatedChange;
production and test construction are macOS-specific. New Linux CI will determine
actual target acceptance. Preserve all failed runs and do not infer Mac causes.
No app launch, provider request, install, merge or further repair authorized.
Ledger63/66, ECC hooks/MCP disabled, advisories/native workaround/parked work stay.

## Preservation and recovery

Verified snapshot, raw completed state/index/link/admissions and external receipts:
/private/tmp/cortexa-pr139-platform-repair-tl9x12y6. Restore only after separately
scoped owner authorization; never delete or overwrite existing work/history.

## Progress

Required refs match; verified 1125-path recoverable snapshot. Readiness preparation
only; no implementation or successor admission yet.

## Executed local results

Ordinary admission passed. Exact two-file gating implemented; owner authorized
only the one-line rustfmt adjustment after the initial formatting failure.
That failed receipt remains unchanged. Corrected format, strict Clippy,
all-target Rust tests and native no-bundle build passed. Existing diagnostic and
all unrelated bytes are preserved. Unchanged frontend/repository/hooks evidence
is reused explicitly; no full verify or native QA repetition is claimed.

Final reviews/documentation/gates and exact one-commit publication are pending
freeze. New Linux/target-Mac CI must be observed at the resulting exact head.
