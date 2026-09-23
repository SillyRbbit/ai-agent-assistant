# PR #123 ACP fixture stream category Clippy retry post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings",
    "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "npm_config_offline=true npm run docs:check",
    "npm_config_offline=true npm run repository:check",
    "npm_config_offline=true npm run security:scan",
    "git diff --check",
    "python3 /private/tmp/cortexa-pr123-acp-fixture-stream-category-clippy-retry-evidence/scope.py",
    "python3 /private/tmp/cortexa-pr123-acp-fixture-stream-category-clippy-retry-evidence/preserve.py",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_acp_transport_spike.rs",
    "docs/plans/2026-09-23-pr123-acp-fixture-stream-category.md",
    "docs/reviews/2026-09-23-pr123-acp-fixture-stream-category-post-increment-review.md",
    "docs/plans/2026-09-23-pr123-acp-fixture-stream-category-clippy-retry.md",
    "docs/reviews/2026-09-23-pr123-acp-fixture-stream-category-clippy-retry-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Review the separate branch's exact-head Target-Mac CI and authorize any evidenced fixture correction separately",
      "milestone": "Before any PR #123 merge decision",
      "risk": "The child-process startup cause is not established by local tests or a closed stderr category",
      "severity": "Advisory",
      "summary": "Target-Mac ACP fixture startup remains unverified pending separate diagnostic CI"
    }
  ],
  "increment_id": "pr123-acp-fixture-stream-category-clippy-retry",
  "manual_verification": [
    {"check": "Exact five-path cumulative scope, three-path successor delta, and inherited-byte preservation", "required": true, "status": "Passed"},
    {"check": "All predecessor worktrees, terminal/completed records, and two prunable entries preserved", "required": true, "status": "Passed"},
    {"check": "Independent architecture, security, code-health, debt, and readiness review", "required": true, "status": "Passed"},
    {"check": "Exact-head CI after separate diagnostic branch push", "required": false, "status": "Manual verification pending"}
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike", "required": true, "status": "Passed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 /private/tmp/cortexa-pr123-acp-fixture-stream-category-clippy-retry-evidence/scope.py", "required": true, "status": "Passed"},
    {"command": "python3 /private/tmp/cortexa-pr123-acp-fixture-stream-category-clippy-retry-evidence/preserve.py", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-09-23
Increment: pr123-acp-fixture-stream-category-clippy-retry
Branch: codex/pr123-acp-fixture-stream-category-clippy-retry

## Executive summary

The separately admitted test-only successor replaces four prohibited `expect()` calls with explicit test error propagation. Focused and full offline verification passed. The original failed increment remains terminal and unchanged. Quality result: **PASS WITH ADVISORIES**, subject to ordinary finalization.

## Scope and boundaries

Only the test assertion correction and this new plan/review pair are successor changes. The five-path cumulative inventory includes the byte-identical failed predecessor plan/review. PR #123, production code, runner/workflow settings, dependencies, governance, and permissions are unchanged.

## Verification results

The focused offline ACP integration target passed seven tests. `cargo fmt --check` and strict all-target/all-feature Clippy passed. Full offline `npm run verify` passed, including 431 frontend tests, 368 Rust library tests, integration tests, frontend builds, and a native no-bundle release build. Offline documentation, repository, security, whitespace, exact-scope, all-worktree preservation, and session inventory checks passed. No native launch or provider/runtime request was performed. The separate branch has not yet been pushed, so its exact-head CI remains pending and is not represented as local verification.

## Architecture findings

The correction is confined to one existing test function. Its surrounding scanner, probe implementation, original assertions, production module boundaries, dependencies, and trust ownership are unchanged. No architecture drift was found in the complete five-path diff.

## Security findings

The inherited diagnostic retains a 512-byte stderr sample cap, bounded rolling signature window, closed output category, independent truncation Boolean, and no raw stderr in the emitted diagnostic. The correction changes only test failure propagation. No credential, IPC, permission, network, policy, or execution boundary changes were found. Synthetic redaction assertions passed.

## Code-health findings

The four `expect()` calls are gone from the new regression test; their failure conditions now return test errors. Every original assertion remains. Strict Clippy and format pass. No new code-health defect was found in the bounded correction.

## Technical debt

No new technical debt was identified. The unresolved Target-Mac startup cause is a pre-existing diagnostic dependency, recorded as an advisory rather than a claim of fixture health.

## Roadmap findings

PR #123 remains open at its unchanged 32-file head. The new separate diagnostic branch still needs exact-head CI before its sanitized category can inform any fixture decision. Provider/runtime live-success, Codex-isolation, D-127, and D-128 advisories remain. D-125/M1/M2 stay parked.

## Completion decision

**PASS WITH ADVISORIES** after ordinary finalization. This local result does not approve PR #123, establish its Target-Mac fixture cause, or authorize a fixture repair.

## Next-increment readiness

**Ready with advisories** for the separately authorized commit/push and exact-head CI inspection after a valid completion marker. PR #123 itself remains blocked pending its Target-Mac outcome and separate review.

## Exact files changed

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `docs/plans/2026-09-23-pr123-acp-fixture-stream-category.md`
- `docs/reviews/2026-09-23-pr123-acp-fixture-stream-category-post-increment-review.md`
- `docs/plans/2026-09-23-pr123-acp-fixture-stream-category-clippy-retry.md`
- `docs/reviews/2026-09-23-pr123-acp-fixture-stream-category-clippy-retry-post-increment-review.md`

## Exact commands executed

- `CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike`: Passed, seven tests.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: Passed.
- `CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`: Passed.
- `CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify`: Passed, including the no-bundle native release build.
- `npm_config_offline=true npm run docs:check`: Passed.
- `npm_config_offline=true npm run repository:check`: Passed.
- `npm_config_offline=true npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 /private/tmp/cortexa-pr123-acp-fixture-stream-category-clippy-retry-evidence/scope.py`: Passed.
- `python3 /private/tmp/cortexa-pr123-acp-fixture-stream-category-clippy-retry-evidence/preserve.py`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed without conflicts.
