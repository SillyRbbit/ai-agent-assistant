# PR #123 ACP child developer-directory comparison post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo fmt --manifest-path src-tauri/Cargo.toml",
    "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike",
    "python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-evidence/validate.py",
    "git diff --check"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_acp_transport_spike.rs",
    "docs/plans/2026-09-23-pr123-acp-developer-dir-comparison.md",
    "docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": true,
      "blocks_next_increment": true,
      "category": "Code health",
      "effort": "Separately diagnose the bounded comparison timeout and authorize a distinct correction only after reviewing evidence",
      "milestone": "Before any diagnostic publication or PR #123 merge decision",
      "risk": "The new comparison produces no pair of closed summaries when its child exceeds the local deadline",
      "severity": "High",
      "summary": "The focused ACP target failed: the new comparison test returned Timeout"
    }
  ],
  "increment_id": "pr123-acp-developer-dir-comparison",
  "manual_verification": [
    {"check": "All predecessor worktrees and prunable registry entries preserved at admission", "required": true, "status": "Passed"},
    {"check": "Exact three-path successor scope and protected bytes after failure", "required": true, "status": "Passed"},
    {"check": "Exact-head Target-Mac comparison after push", "required": false, "status": "Not run"}
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
    {"command": "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike", "required": true, "status": "Failed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check", "required": true, "status": "Failed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml", "required": false, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-evidence/validate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-09-23
Increment: pr123-acp-developer-dir-comparison
Branch: codex/pr123-acp-developer-dir-comparison

## Executive summary

**FAIL.** The new test-only comparison timed out during focused local validation.
Seven pre-existing ACP tests passed; the new comparison failed. No diagnostic
pair was obtained and no branch was committed or pushed.

## Scope and boundaries

The attempted successor touched only the existing ACP fixture test and this
new plan/review pair. It did not change production code, the fixture, runner,
workflow, dependencies, PR #123, or any predecessor report.

## Verification results

The first format check found one formatting difference; `cargo fmt` corrected
it within the declared test-file scope. The focused offline ACP test then ran
eight tests: seven passed, and the new comparison returned `Timeout`. Required
Clippy, full verification, documentation, repository, security, session, and
completion checks were not continued after the required focused test failed.
`git diff --check` subsequently passed as a read-only disposition check. A
read-only external check confirmed the exact three-path scope,
protected-byte identity, and all predecessor worktree/registry fingerprints.
Its corrected state-file path verified four recorded historical raw-state hashes
and current report integrity; the earlier 24-worktree snapshot did not record
usable raw-state hashes for all predecessors.

Required checks stopped after the failure and recorded as **Not run**:

- `CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`: Not run.
- `CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify`: Not run.
- `npm_config_offline=true npm run docs:check`: Not run.
- `npm_config_offline=true npm run repository:check`: Not run.
- `npm_config_offline=true npm run security:scan`: Not run.
- `python3 .codex/hooks/session_end_gate.py`: Not run.

## Architecture findings

The new helper is test-only and uses the existing isolated child setup and
bounded stream readers. No architecture conclusion beyond this narrow diff is
accepted because completion verification failed.

## Security findings

The new helper intends to emit only closed fields, but no successful local
comparison output was obtained. It does not log raw child payloads in source.
An end-to-end safety conclusion is withheld pending valid tests and review.

## Code-health findings

The duplicated probe mechanics and timeout need a separate review. The
timeout's cause is unconfirmed; no automatic correction was made.

## Technical debt

The original Target-Mac fixture startup failure remains unresolved. The new
comparison is blocked before it can inform that diagnosis.

## Roadmap findings

PR #123 remains open and unchanged. Provider/runtime live-success and
Codex-isolation advisories, D-127 and D-128, remain. D-125/M1/M2 stay parked.

## Completion decision

**FAIL.** Freeze a truthful terminal record without a completion marker.
Do not publish this branch or modify PR #123.

## Next-increment readiness

**Blocked.** A separately authorized correction would first need to determine
which child timed out and why, without printing raw child data. No correction
or retry is authorized by this failed record.

## Exact files changed

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `docs/plans/2026-09-23-pr123-acp-developer-dir-comparison.md`
- `docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-post-increment-review.md`

## Exact commands executed

- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: failed on one
  in-scope line-wrap difference.
- `cargo fmt --manifest-path src-tauri/Cargo.toml`: passed and formatted the
  in-scope test file.
- `CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike`: failed; seven passed, new comparison timed out.
- `python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-evidence/validate.py`: passed; 26 predecessor registry entries and three successor paths.
- `git diff --check`: passed during read-only failed-disposition review.
