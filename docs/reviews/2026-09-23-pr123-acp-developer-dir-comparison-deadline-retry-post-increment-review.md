# PR #123 ACP comparison deadline retry post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "CARGO_NET_OFFLINE=true cargo test --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings",
    "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "npm_config_offline=true npm run repository:check",
    "npm_config_offline=true npm run security:scan",
    "git diff --check",
    "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry.md docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry-post-increment-review.md",
    "npm_config_offline=true npm run docs:check",
    "python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-deadline-retry-evidence/validate.py",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_acp_transport_spike.rs",
    "docs/plans/2026-09-23-pr123-acp-developer-dir-comparison.md",
    "docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-post-increment-review.md",
    "docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-order-retry.md",
    "docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-order-retry-post-increment-review.md",
    "docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry.md",
    "docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Separately authorize exact-head Target-Mac CI on a diagnostic branch before any fixture or PR #123 decision",
      "milestone": "Before any PR #123 merge decision",
      "risk": "Local success does not establish the CI-service startup cause",
      "severity": "Advisory",
      "summary": "The original Target-Mac fixture startup failure remains unverified by this local-only comparison"
    }
  ],
  "increment_id": "pr123-acp-developer-dir-comparison-deadline-retry",
  "manual_verification": [
    {"check": "Five-path candidate transfer, seven-path cumulative scope, and predecessor preservation", "required": true, "status": "Passed"},
    {"check": "Independent architecture, security, code-health, debt, and readiness review", "required": true, "status": "Passed"},
    {"check": "Exact-head CI after separately authorized diagnostic publication", "required": false, "status": "Manual verification pending"}
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "CARGO_NET_OFFLINE=true cargo test --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike", "required": true, "status": "Passed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry.md docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry-post-increment-review.md", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run docs:check", "required": true, "status": "Passed"},
    {"command": "python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-deadline-retry-evidence/validate.py", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-09-23
Increment: pr123-acp-developer-dir-comparison-deadline-retry
Branch: codex/pr123-acp-developer-dir-comparison-deadline-retry

## Executive summary

The one-line test-only successor extends only the two-child comparison helper's
deadline from two to ten seconds. Focused ACP tests, Rust format and strict
Clippy, full offline verification, documentation and repository checks passed.
Exact-scope and predecessor-preservation checks and independent quality review
also passed. Ordinary finalization and the full-payload Stop hook remain.

## Scope and boundaries

The seven-path cumulative inventory contains five byte-identical inherited
candidate paths plus this new plan/review pair. The successor's only executable
change is the comparison helper deadline. The original `run_probe` deadline,
fixture, assertions, production code, dependencies, runner/workflow settings,
and PR #123 remain unchanged.

## Verification results

The focused offline ACP integration target passed eight tests. Rust formatting
and strict all-target/all-feature Clippy passed. Full offline `npm run verify`
passed, including 431 frontend tests, 368 Rust library tests, integration
tests, frontend builds, and the native no-bundle release build. Repository,
tracked-secret, tracked-diff whitespace, documentation, exact-scope,
predecessor-preservation, and session checks passed. The completion gate and
Stop hook have not yet run.

## Architecture findings

The comparison remains test-only. The shared fixture setup and original probe
are unchanged. No new production dependency or authority path is introduced.

## Security findings

The bounded stdout and stderr readers, closed diagnostic categories, and
no-raw-payload output remain unchanged. No provider/runtime request, credential,
permission, IPC, or runner configuration change was made.

## Code-health findings

The only new behavior is a longer deadline for the diagnostic comparison.
The original fixture tests retain their two-second deadline and assertions.

## Technical debt

The original Target-Mac fixture startup cause is unresolved. Local diagnostic
success cannot discharge that debt or make PR #123 merge-ready.

## Roadmap findings

PR #123 remains open at its unchanged 32-file head. Provider/runtime
live-success and Codex-isolation advisories, D-127 and D-128, remain.
D-125/M1/M2 stay parked.

## Completion decision

Independent quality review: PASS WITH ADVISORIES. The report is ready for final
schema validation and ordinary post-increment finalization. This local-only
successor grants no publication or PR #123 merge authority.

## Next-increment readiness

Ready with advisories for a separately authorized diagnostic publication
decision once the completion gate and Stop hook pass. Exact-head Target-Mac CI
remains necessary before any fixture or PR #123 decision.

## Exact files changed

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `docs/plans/2026-09-23-pr123-acp-developer-dir-comparison.md`
- `docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-post-increment-review.md`
- `docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-order-retry.md`
- `docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-order-retry-post-increment-review.md`
- `docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry.md`
- `docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry-post-increment-review.md`

## Exact commands executed

- `CARGO_NET_OFFLINE=true cargo test --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike`: Passed, eight tests.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: Passed.
- `CARGO_NET_OFFLINE=true cargo clippy --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`: Passed.
- `CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify`: Passed.
- `npm_config_offline=true npm run repository:check`: Passed.
- `npm_config_offline=true npm run security:scan`: Passed.
- `git diff --check`: Passed on the tracked diff.
- `./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry.md docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-deadline-retry-post-increment-review.md`: Passed.
- `npm_config_offline=true npm run docs:check`: Passed.
- `python3 /private/tmp/cortexa-pr123-acp-developer-dir-comparison-deadline-retry-evidence/validate.py`: Passed, including all 28 predecessor entries.
- `python3 .codex/hooks/session_end_gate.py`: Passed.
