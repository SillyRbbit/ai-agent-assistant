# PR #123 fixture-repair publication preparation

Date: 2026-09-29
Increment: pr123-fixture-repair-publication
Status: Prepared; packaging checks passed; completion transitions follow report validation

## Goal and authority

Prepare one separately admitted publication tree from PR head `66090a0` while
preserving the accepted `e63092c` development candidate. The owner authorizes
worktree/branch creation and this preparation only; no commit, push or PR change.
The exact baseline is `66090a0909d551c9a373ec142bb46b0f73ef0717`, with live main
`fc6006e892c89cbc83d60f709875e4db3d8f18de`. The source completion is
complete/valid/PASS WITH ADVISORIES. All source files and ignored gate state remain
unchanged. The new checkout has its own ordinary gate, not a copied marker.

## Exact scope

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-29-pr123-fixture-repair-publication.md`
- `docs/reviews/2026-09-29-pr123-fixture-repair-publication-post-increment-review.md`
- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `src-tauri/tests/hermes_transport_spike.rs`

These ten paths yield exactly 36 paths relative to main. The original 32-path
Anthropic/local PR retains all product bytes and historical documentation. The
six root documents retain their complete original bodies as exact suffixes.
No later admission hook, hook tests, report template, archive or diagnostic
plan/report is transferred. No dependency installation or application rebuild.

## Existing behavior and transferred evidence

The ACP test retains the isolated macOS Xcode directory, finite startup allowance,
closed timeout-site labels, cleanup and negative-case deadlines. The Hermes test
retains the macOS helper scope, explicit child-only developer directory, meaningful
supported-path comparison assertions, bounded stderr drain and redaction tests.
Its original two-second deadlines remain unchanged; no blanket timeout increase.
Successful macOS stderr must remain bounded, fully drained and free of read
failure; non-macOS success retains the empty-stderr assertion. No production
runtime, real Hermes installation, provider call or host-environment inheritance.

Accepted fixture SHA-256 values:

- `src-tauri/tests/hermes_acp_transport_spike.rs`: `39f9020c5582d486b20a41bb7216b1ec069c9bdb03e5f1fe36d4256477083e1c`
- `src-tauri/tests/hermes_transport_spike.rs`: `601feb04166e38d40fe5b705687c01baf61dfeba222766e4281dd3a9e18aaa38`

The source report SHA-256 is `1cd88f80b68534a9c282c2f6690a4407299d9f2cd3980112f8eb232ee77ceac7`.
The inherited SDK 27.0 full verification log SHA-256 is
`3f03264d949355aad93e3397b47d176b0efe89529d3a9f97277936e255c49fee`.
Historical results: ACP 7 passed; Hermes 13 passed/one existing ignored opt-in
real-Hermes test; full offline verification, Rust formatting/strict Clippy,
frontend tests and native release build passed. The owner expressly permits
reuse. Current executed-command manifests must not claim those application
commands ran again. Product, dependency, compiler-input and fixture identities
are checked against that accepted source; excluded governance is identical to
PR #123 and is validated independently by current hook tests.

## Validation and stop conditions

Fresh checks: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`,
`python3 -B -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v`,
`npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
`git diff --check`, `python3 -B .codex/hooks/session_end_gate.py`, and the external
scope/preservation checker. Read the exact 12-section report with the unchanged
ordinary schema validator before finalization. Require quality/readiness review,
ordinary finalize, complete/valid status and full-payload Stop.

Use existing local Prettier through its absolute executable/PATH; do not copy or
install dependencies. Source node_modules is tooling only and remains unchanged.
No application tests/builds are repeated for identical accepted executable bytes.
Required Linux and Target-Mac exact-head CI remain a later publication condition;
the older failed PR check is not evidence of a pass on this candidate.

Stop on ref/byte/scope drift, admission rejection, failed validation, unsafe
output, missing offline prerequisites or need for a scope expansion. Preserve
work on failure; no automatic rollback, successor, repair or publication.

## Risks, review and rollback

The macOS fixtures require installed Xcode at the verified developer directory;
local passes do not guarantee CI-service behavior. Linux must retain isolation
and strict linting. Relevant bounded diagnostics remain regression tests; unrelated
historical/governance changes remain local. Preserve D-127/D-128, provider/runtime
live-success and Codex-isolation advisories and parked D-125/M1/M2.

Rollback means leave the isolated candidate unpublished for owner review; never
reset, clean or remove existing worktrees. After passing local packaging gates,
request separate publication authority and verify every exact-head job and review
before merge. Never infer publication or request authority from this plan.

## Progress

Ordinary admission and byte-identical transfer completed. Packaging checks passed,
including 74 baseline hook tests, Rust formatting, documentation/repository/security,
whitespace, session, scope and preservation. The final report records the actual
results; inherited application verification remains explicitly historical. Ordinary
finalization and full Stop follow schema validation; the live marker is authoritative.
