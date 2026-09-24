# PR #123 ACP comparison deadline retry

Status: Validation passed; completion pending
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal

Allow the test-only two-child startup comparison to finish locally without
altering the original ACP fixture version probe or masking its Target-Mac
failure. The preceding isolated local probe completed after two seconds and
before ten seconds with fixed output matched and a closed, non-truncated
`other_nonempty` stderr category. That observation does not establish the
CI-service cause.

## Scope

Start from `c7fba47a37e595082008428c0eaccb89618db766` and transfer the
exact five-path terminal-failed candidate without gate state. The successor
delta is limited to `src-tauri/tests/hermes_acp_transport_spike.rs` and this
new plan/review pair. The cumulative inventory is exactly seven paths.

## Current-state evidence

The predecessor `pr123-acp-developer-dir-comparison-order-retry` is valid
terminal `failed / FAIL / Blocked`, with no completion marker. Its focused ACP
run passed seven existing tests, then printed a successful closed baseline
summary before the Xcode-directory variant exceeded the existing two-second
deadline. At admission, remote main remained
`fc6006e892c89cbc83d60f709875e4db3d8f18de` and PR #123 remained at
`66090a0909d551c9a373ec142bb46b0f73ef0717`.

## Implementation and invariants

Change only the `compare_version_child` helper deadline to ten seconds. Leave
the original `run_probe`, its two-second `TEST_DEADLINE`, all fixture behavior
and assertions, bounded closed stderr classification, and no-raw-output
policy unchanged. No production, dependency, runner, workflow, or PR #123
change is authorized.

## Validation

Run the focused offline ACP target, Rust format and strict Clippy, full offline
`npm run verify`, documentation/repository/security/whitespace checks, exact
scope and all-worktree preservation, independent quality review, and ordinary
post-increment completion gates. Stop on drift, failed validation, download,
unsafe output, or scope expansion. Do not commit, push, rerun CI, or repair the
fixture.

## Risk and limits

One local completion inside ten seconds does not prove reliable timing or
equivalence to the CI service. Even a passing diagnostic comparison does not
change the original CI probe's observed exit-1 `developer_tool` failure.

## Progress and final results

2026-09-23: Five-path candidate transferred byte-for-byte, offline formatter
tooling cloned copy-on-write, ordinary admission passed, and the helper-only
deadline edit is in place. The focused eight-test ACP target, Rust format and
strict Clippy, full offline `npm run verify`, documentation, repository,
security, whitespace, exact-scope and predecessor-preservation checks passed.
The independent quality review found no blocking issue; ordinary finalization
and the full-payload Stop hook remain pending.
