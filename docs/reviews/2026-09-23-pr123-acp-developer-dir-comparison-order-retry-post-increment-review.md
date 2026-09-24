# PR #123 ACP comparison output-order retry post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike",
    "python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-order-retry-evidence/validate.py",
    "git diff --check"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_acp_transport_spike.rs",
    "docs/plans/2026-09-23-pr123-acp-developer-dir-comparison.md",
    "docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-post-increment-review.md",
    "docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-order-retry.md",
    "docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-order-retry-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": true,
      "blocks_next_increment": true,
      "category": "Code health",
      "effort": "Separately diagnose the variant child timeout with bounded, non-sensitive evidence",
      "milestone": "Before any diagnostic publication or PR #123 merge decision",
      "risk": "The local Xcode-directory comparison child does not produce a terminal status within the existing deadline",
      "severity": "High",
      "summary": "Focused ACP validation failed after the baseline child succeeded and the variant child timed out"
    }
  ],
  "increment_id": "pr123-acp-developer-dir-comparison-order-retry",
  "manual_verification": [
    {"check": "Transferred three-path failed candidate byte-identically without gate state", "required": true, "status": "Passed"},
    {"check": "Exact five-path cumulative scope and all predecessor preservation after failure", "required": true, "status": "Passed"},
    {"check": "Exact-head Target-Mac CI comparison", "required": false, "status": "Not run"}
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
    {"command": "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike", "required": true, "status": "Failed"},
    {"command": "python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-order-retry-evidence/validate.py", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-09-23
Increment: pr123-acp-developer-dir-comparison-order-retry
Branch: codex/pr123-acp-developer-dir-comparison-order-retry

## Executive summary

**FAIL.** The test-only output-order change exposed a bounded baseline success,
then the Xcode-directory variant timed out. Seven existing ACP tests passed;
the new comparison failed. The branch remains uncommitted and unpublished.

## Scope and boundaries

The successor change moves one existing closed baseline print between the
probe calls. The inherited three-path failed candidate is byte-identical
outside that one test-file change. No production code, fixture, runner,
workflow, dependency, PR #123, or predecessor record was changed.

## Verification results

The focused offline ACP target ran eight tests. Seven passed. The new comparison
printed `baseline: exit_code=Some(0) signal=None output_matches=true
stderr_category=empty stderr_truncated=false`, then returned the existing
closed `Timeout` error before any variant terminal summary. This establishes
which child exceeded the local deadline, not the upstream or CI-service cause.

The read-only external preservation check passed for all 27 predecessor
registry entries, exact five-path cumulative scope, protected bytes, and the
single-statement test delta. `git diff --check` passed on the tracked diff.
Other required checks stopped after the focused failure and are **Not run**:

- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: Not run.
- `CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`: Not run.
- `CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify`: Not run.
- `npm_config_offline=true npm run docs:check`: Not run.
- `npm_config_offline=true npm run repository:check`: Not run.
- `npm_config_offline=true npm run security:scan`: Not run.
- `python3 .codex/hooks/session_end_gate.py`: Not run.

## Architecture findings

The change is confined to output order in one integration test. Existing
fixture behavior, ownership, and production boundaries remain unchanged.

## Security findings

The observed summary contains only fixed exit/signal, output-match,
stderr-category, and truncation fields. No raw child stdout, stderr, path,
argument, environment value, or credential appeared in the test output.
Full security validation was not run after the failed focused test.

## Code-health findings

The output ordering now attributes the timeout to the variant child. The
variant's stall remains unexplained; no automatic timeout or environment
change was made.

## Technical debt

The Target-Mac ACP fixture startup failure remains unresolved. This local
result does not prove the same variant behavior in the CI service context.

## Roadmap findings

PR #123 remains open and unchanged. Provider/runtime live-success and
Codex-isolation advisories, D-127 and D-128, remain. D-125/M1/M2 stay parked.

## Completion decision

**FAIL.** Freeze a truthful terminal record without a completion marker. Do
not publish this branch or modify PR #123.

## Next-increment readiness

**Blocked.** Any further variant diagnosis or fixture correction needs
separate evidence and authorization after this terminal record validates.

## Exact files changed

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `docs/plans/2026-09-23-pr123-acp-developer-dir-comparison.md`
- `docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-post-increment-review.md`
- `docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-order-retry.md`
- `docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-order-retry-post-increment-review.md`

## Exact commands executed

- `CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike`: failed; seven tests passed, and the comparison variant timed out after a successful baseline summary.
- `python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-order-retry-evidence/validate.py`: passed; 27 predecessor registry entries, exact five-path cumulative scope, and one-line test delta.
- `git diff --check`: passed on the tracked diff.
