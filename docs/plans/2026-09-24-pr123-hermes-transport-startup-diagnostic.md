# PR #123 Hermes transport fixture startup diagnostic

Status: Terminal failure; focused test compilation failed
Owner: Cortexa project owner
Last updated: 2026-09-24

## Goal and evidence

The exact-head Target-Mac job for diagnostic commit `e7c08480d1e998fc1b2e4dbe983fe0aa9f0bd286` passed the ACP fixture (7/7) but failed all seven active `hermes_transport_spike` tests. The unchanged Hermes transport fixture produced a closed `VersionMismatch`, five `UnexpectedExit` results, and an `UnexpectedExit` where a deliberate readiness timeout was expected. Earlier ACP-specific CI-service evidence found that an isolated Python child failed without the verified Xcode developer directory and succeeded with it. That evidence does not establish the cause of this separate fixture failure.

## Exact scope and non-goals

Start from the verified `e7c08480` diagnostic commit in a separate detached worktree with no copied gate state. Change only `src-tauri/tests/hermes_transport_spike.rs`, this plan, and `docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-post-increment-review.md`. The successor delta is exactly three paths. Preserve the inherited eight-path commit, every predecessor worktree, report, raw gate state and external evidence. Do not change PR #123, application source, fixture scripts, runner settings, workflow, dependencies, deadlines, assertions, or transport behavior.

## Diagnostic design

Use the existing isolated command configuration and fixed Python fixture. Observe the finite `--version` probe and the first ready event separately, each under the existing two-second deadline. On macOS, compare the existing environment with an otherwise identical child that receives only the previously verified Xcode `DEVELOPER_DIR`. Preserve child cleanup. Retain no child stderr beyond a bounded fixed-signature scanner and emit no raw stdout, stderr, paths, arguments, environment values or credentials. Failure output may contain only closed stage, exit code or signal, fixed-output-match status, closed stderr category and truncation. Add offline redaction and classifier regressions in the same test file. This is a diagnostic, not a fixture repair or a claim that the Xcode variant succeeds in CI.

## Validation and stop conditions

Run focused offline Hermes transport tests, Rust formatting and strict Clippy, applicable full offline verification, documentation/repository/security/whitespace checks, exact three-path scope and predecessor-preservation checks, session and independent quality review, report-schema validation, ordinary completion gates, and the full-payload Stop hook. Record actual commands and results only. Stop on ref/evidence drift, unsafe diagnostic output, failed validation, scope expansion, or any need to change fixture behavior or deadlines. Exact-head Target-Mac evidence requires separate publication authorization; do not commit, push, rerun CI, update or merge PR #123 automatically. Retain existing provider/runtime, Codex-isolation, D-127 and D-128 advisories; keep D-125/M1/M2 parked.

## Progress

- [x] Verify refs, inherited completion, eight-path scope, all predecessor fingerprints and settled CI.
- [x] Create clean detached successor and complete ordinary admission.
- [x] Add closed startup comparisons and offline redaction regressions within the test file.
- [x] Stop after the first focused offline compile failed on a test-only mutable binding.
- [ ] Required later checks were not run; record them as Not run in the terminal report.
- [ ] Validate and close the truthful FAIL report without repairing code or starting a successor.

The first focused command, `cargo test --manifest-path src-tauri/Cargo.toml --offline --locked --test hermes_transport_spike`, stopped at Rust error E0596 in the new `observe_first_ready_startup` helper because its local command binding was not mutable before `spawn`. No test executed, no CI was triggered, and no Target-Mac comparison result was obtained. The approved stop condition prohibits an automatic correction. The changed test file and this plan remain preserved for separate review; the completion gate must record FAIL / Blocked rather than passing implementation.
