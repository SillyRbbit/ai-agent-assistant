# PR #123 ACP comparison output-order retry

Status: Failed; terminal disposition pending
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal

Attribute a future timeout in the existing Target-Mac ACP startup comparison
without changing the fixture, runner, child environment, or probe behavior.

## Scope

From commit `c7fba47a37e595082008428c0eaccb89618db766`, transfer the exact
three-path valid terminal-failed candidate without gate state. The successor
delta is limited to `src-tauri/tests/hermes_acp_transport_spike.rs` and this
new plan/review pair. The cumulative inventory is exactly five paths.

## Current-state evidence

The predecessor `pr123-acp-developer-dir-comparison` is valid terminal
`failed / FAIL / Blocked`, with no completion marker. Its focused local ACP
target passed seven tests and timed out in the new comparison, with no
comparison summary printed. Because both summaries were printed after both
children returned, that output cannot identify which child exceeded the
deadline. At admission, remote main remained
`fc6006e892c89cbc83d60f709875e4db3d8f18de`, PR #123 remained at
`66090a0909d551c9a373ec142bb46b0f73ef0717`, and the diagnostic baseline
remained `c7fba47a37e595082008428c0eaccb89618db766`.

## Implementation

Move only the existing baseline `eprintln!` between the two probe calls. It
continues to emit the same fixed fields—exit code/signal, version-output match,
closed stderr category, and truncation—without raw child data. A subsequent
timeout with a baseline summary implicates the variant child; a timeout
without that summary implicates the baseline child. This is attribution, not
an explanation of the startup cause.

## Non-goals and invariants

Preserve every other test assertion, the original fixture behavior, production
bytes, dependencies, runner/workflow configuration, PR #123, and all predecessor
reports and raw gate states. No fixture repair, provider request, or merge.

## Validation

Run focused offline ACP tests, Rust format and strict Clippy, full offline
`npm run verify`, documentation/repository/security/whitespace checks, exact
scope and all-worktree preservation, session and quality review, and ordinary
post-increment finalization. Stop on drift, unsafe output, failed required
validation, download, or scope expansion. Commit/push is not authorized by
this successor request.

## Progress and final results

2026-09-23: Candidate transferred byte-for-byte into an isolated worktree;
ordinary admission passed. The one-line output-order change is in place.
The focused offline ACP target failed: seven existing tests passed, the
baseline child printed a closed success summary, and the Xcode-directory
variant timed out. Required broader checks stopped. The exact five-path
scope and all predecessor fingerprints passed the external preservation check.
No correction, retry, commit, push, or CI run followed.
