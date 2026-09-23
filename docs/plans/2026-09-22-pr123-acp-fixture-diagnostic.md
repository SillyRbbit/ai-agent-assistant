# PR #123 Target-Mac ACP fixture diagnostic

Status: Terminal failed; publication stopped
Owner: Cortexa project owner
Last updated: 2026-09-22

## Goal

Capture the isolated Python fixture's exit status and one closed stderr category when the existing `--version` probe fails in Target-Mac CI. The information must distinguish child startup failures without disclosing stderr, stdout, paths, arguments, environment values, or sensitive content.

## Current-state evidence

- PR #123 is open at `66090a0909d551c9a373ec142bb46b0f73ef0717` against main `fc6006e892c89cbc83d60f709875e4db3d8f18de`, with exactly 32 changed paths.
- Main's Target-Mac CI run `35749752577` passed all five `hermes_acp_transport_spike` tests. PR run `35799924211`, attempt 2, job `106999871237` passed Clippy and failed all five tests; its `--version` case returned only `VersionMismatch`.
- The test and fixture blobs are identical on PR head and main. A local cleared-environment `--version` probe passed both with and without Xcode `DEVELOPER_DIR`, so the runner-service cause is unconfirmed.
- The existing test drains stderr and preserves only byte count/truncation; it discards child exit status from the reported error.

## Scope and non-goals

This separately admitted diagnostic branch begins at the exact PR head. Its only executable change is `src-tauri/tests/hermes_acp_transport_spike.rs`. The only other changed paths are this plan and `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-post-increment-review.md`. The original PR branch, its 32-file scope, completion marker, all other worktrees, the runner service, and workflows remain unchanged.

Do not change the Python fixture, production code, dependencies, Rust error mappings, fixture validation, timeouts, process behavior, models, providers, or network behavior. Do not update or merge PR #123 or implement a repair from the diagnostic result without separate authorization.

## Implementation

Continue draining stderr, retaining at most the first 512 bytes in memory for closed classification; never print or persist the bytes. On failure of `run_probe(--version)` only, emit one static diagnostic line with the numeric exit code or signal, a Boolean indicating whether stdout matched the fixed expected version, and one of `empty`, `developer_tool`, `loader_architecture`, `python_startup`, `other_nonempty`, `truncated`, or `read_failure`. Existing `VersionMismatch` and all other results remain unchanged.

Add a regression in the same test file that exercises the closed categories and proves a private sentinel cannot appear in the formatted line. The diagnostic does not establish Hermes conformance or any application/runtime capability.

## Validation and publication boundary

Run focused offline ACP tests, Rust format and Clippy, full offline `npm run verify`, documentation/repository/security checks, whitespace and exact three-path checks, session/quality/post-increment gates, and a passing Stop hook. Treat the local completion report as local evidence; Target-Mac CI remains pending until the diagnostic branch is pushed. After valid local completion, commit and push only the diagnostic branch. Inspect its exact-head CI, especially the Target-Mac line, without rerunning or changing PR #123. A failed Target-Mac application test is not diagnostic-branch readiness evidence; the purpose is to observe its sanitized child category.

## Risks and stop conditions

The added line must never contain raw stderr, stdout, paths, arguments, environment values, or sensitive content. Stop on ref or worktree drift, admission failure, unsafe output, a changed application result, failed required local validation, scope expansion, or a CI result that requires an unapproved repair. Preserve any truthful failure; do not automatically create a successor.

## Progress and final results

- 2026-09-22: Exact refs, unchanged fixture/test blobs, 32-path PR scope, and valid predecessor completion marker confirmed. Separate worktree created and ordinary admission accepted. The six focused ACP tests, Rust format, and Clippy passed. Full offline `npm run verify` stopped at Prettier because this plan did not match formatting. The approved stop condition ended validation before publication; the diagnostic branch was not committed or pushed, and CI did not run for it.
