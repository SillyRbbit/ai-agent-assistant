# PR #123 closed ACP comparison output post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike compares_isolated_version_startup_with_xcode_child_only -- --exact --show-output",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings",
    "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-closed-comparison-output.md docs/reviews/2026-09-23-pr123-acp-closed-comparison-output-post-increment-review.md",
    "npm_config_offline=true npm run docs:check",
    "npm_config_offline=true npm run repository:check",
    "npm_config_offline=true npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-pr123-acp-closed-comparison-output-evidence/validate.py",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    ".github/workflows/ci.yml",
    "docs/plans/2026-09-23-pr123-acp-closed-comparison-output.md",
    "docs/reviews/2026-09-23-pr123-acp-closed-comparison-output-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Separately authorize diagnostic publication and inspect exact-head Target-Mac CI",
      "milestone": "Before any PR #123 fixture or merge decision",
      "risk": "Local output does not reveal the two CI-service child results; the existing five fixture tests still fail in CI",
      "severity": "Advisory",
      "summary": "The focused workflow step remains unverified in the Target-Mac CI service"
    }
  ],
  "increment_id": "pr123-acp-closed-comparison-output",
  "manual_verification": [
    {"check": "Exact three-path scope, source-byte preservation, and all-worktree registry/fingerprints", "required": true, "status": "Passed"},
    {"check": "Independent architecture, security, code-health, debt, and readiness review", "required": true, "status": "Passed"},
    {"check": "Two closed summaries on a separately authorized Target-Mac CI run", "required": false, "status": "Manual verification pending"}
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike compares_isolated_version_startup_with_xcode_child_only -- --exact --show-output", "required": true, "status": "Passed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify", "required": true, "status": "Passed"},
    {"command": "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-closed-comparison-output.md docs/reviews/2026-09-23-pr123-acp-closed-comparison-output-post-increment-review.md", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm_config_offline=true npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 -B /private/tmp/cortexa-pr123-acp-closed-comparison-output-evidence/validate.py", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-09-23
Increment: pr123-acp-closed-comparison-output
Branch: detached from 39c36ba37be424d0fe389e16300e1c7cb5db1634

## Executive summary

The isolated workflow adds one focused Target-Mac step before the unchanged
full Rust tests. Its `--show-output` flag applies only to the existing closed
comparison test. Focused local and full offline verification passed; the
documentation, repository, security, whitespace, scope, preservation, session,
and independent review checks passed. Completion gates remain pending.

## Scope and boundaries

The successor changes only `.github/workflows/ci.yml` and this plan/review
pair. Test, production, dependency, runner, and PR #123 bytes remain unchanged.
No CI rerun or publication is authorized by this local increment.

## Verification results

The focused offline test passed and displayed exactly two closed summaries:
baseline exit 0, output match, empty stderr; Xcode-directory exit 0, output
match, `other_nonempty` stderr without truncation. These are local results,
not CI-service results. Rust format, strict Clippy, and full offline
`npm run verify` passed. Documentation, repository, security, whitespace,
exact-scope, predecessor-preservation, and session checks passed. Target-Mac
CI was not rerun.

## Architecture findings

The test and fixture remain unchanged. The extra CI step uses the existing
test target and does not change runtime or product architecture.

## Security findings

Only the one existing closed-output test receives `--show-output`; the full
test suite retains captured output. The test's bounded readers and category
format remain unchanged. No raw stdout, stderr, path, argument, environment,
credential, or provider data is emitted by the test.

## Code-health findings

The workflow step is narrowly filtered by integration target and exact test
name. The existing full target-Mac test step is unchanged.

## Technical debt

The five original CI-service fixture failures remain unresolved. Passing the
comparison test does not assert either child's successful startup.

## Roadmap findings

PR #123 remains open and unchanged. Provider/runtime live-success and
Codex-isolation advisories, D-127, and D-128 remain; D-125/M1/M2 stay parked.

## Completion decision

PASS WITH ADVISORIES after independent architecture, security, code-health,
debt, and readiness review. The exact report schema passed; ordinary
finalization and Stop hook remain. No publication or PR #123 action follows
automatically.

## Next-increment readiness

Ready with advisories for a separately authorized diagnostic publication
decision once the completion gates pass. Exact-head Target-Mac inspection is
required before any fixture or PR #123 decision.

## Exact files changed

- `.github/workflows/ci.yml`
- `docs/plans/2026-09-23-pr123-acp-closed-comparison-output.md`
- `docs/reviews/2026-09-23-pr123-acp-closed-comparison-output-post-increment-review.md`

## Exact commands executed

- `CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike compares_isolated_version_startup_with_xcode_child_only -- --exact --show-output`: Passed with two closed summaries.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: Passed.
- `CARGO_NET_OFFLINE=true cargo clippy --offline --locked --quiet --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`: Passed.
- `CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify`: Passed.
- `./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-closed-comparison-output.md docs/reviews/2026-09-23-pr123-acp-closed-comparison-output-post-increment-review.md`: Passed.
- `npm_config_offline=true npm run docs:check`: Passed.
- `npm_config_offline=true npm run repository:check`: Passed.
- `npm_config_offline=true npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 -B /private/tmp/cortexa-pr123-acp-closed-comparison-output-evidence/validate.py`: Passed, including all 29 predecessor entries.
- `python3 .codex/hooks/session_end_gate.py`: Passed.
