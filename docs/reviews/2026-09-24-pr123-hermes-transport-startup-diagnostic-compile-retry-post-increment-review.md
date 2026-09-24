# PR #123 Hermes transport startup diagnostic compile retry post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --offline --locked --test hermes_transport_spike",
    "cargo fmt --manifest-path src-tauri/Cargo.toml --check",
    "CARGO_NET_OFFLINE=true cargo clippy --manifest-path src-tauri/Cargo.toml --offline --locked --all-targets --all-features -- -D warnings",
    "./node_modules/.bin/prettier --write docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry.md docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry-post-increment-review.md",
    "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_transport_spike.rs",
    "docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic.md",
    "docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-post-increment-review.md",
    "docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry.md",
    "docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Obtain separately authorized exact-head Target-Mac CI evidence before proposing any fixture repair",
      "milestone": "Before PR #123 Target-Mac acceptance or merge",
      "risk": "Local baseline success and Xcode-variant non-matches under two seconds do not establish CI-service behavior",
      "severity": "Advisory",
      "summary": "The fixture startup cause remains unverified in the failing CI service"
    }
  ],
  "increment_id": "pr123-hermes-transport-startup-diagnostic-compile-retry",
  "manual_verification": [
    {"check": "Required refs, valid terminal predecessor, and byte-verified three-path transfer without gate state", "required": true, "status": "Passed"},
    {"check": "Exactly one mutable-binding executable correction; inherited diagnostic and predecessor bytes preserved", "required": true, "status": "Passed"},
    {"check": "Closed comparison output grammar and redaction regressions", "required": true, "status": "Passed"},
    {"check": "Exact five-path cumulative scope, all predecessor fingerprints and no conflicts", "required": true, "status": "Passed"},
    {"check": "Exact-head Target-Mac comparison after separate publication authorization", "required": false, "status": "Not run"}
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --offline --locked --test hermes_transport_spike", "required": true, "status": "Passed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml --check", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true cargo clippy --manifest-path src-tauri/Cargo.toml --offline --locked --all-targets --all-features -- -D warnings", "required": true, "status": "Passed"},
    {"command": "./node_modules/.bin/prettier --write docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry.md docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry-post-increment-review.md", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify", "required": true, "status": "Passed"},
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-09-24
Increment: pr123-hermes-transport-startup-diagnostic-compile-retry
Branch: detached from e7c08480d1e998fc1b2e4dbe983fe0aa9f0bd286

## Executive summary

**PASS WITH ADVISORIES.** The separately admitted successor corrects only the inherited E0596 mutable binding. The focused Hermes transport target, strict Rust checks and full offline verification pass. The diagnostic is locally usable, but its closed comparisons are not Target-Mac CI-service evidence and do not repair PR #123.

## Scope and boundaries

The exact cumulative worktree inventory is five paths: the transferred failed candidate's test/plan/review and this successor's plan/review. The executable difference from the terminal predecessor is one mutable local binding in the test file. The earlier report and raw gate state remain unchanged; no gate state was transferred. No production code, Python fixture, dependency, runner configuration, workflow, permission, provider, or PR #123 file changed.

## Verification results

The focused offline Hermes transport target passed 11 tests; the real-Hermes opt-in probe remained ignored. This includes the closed-signature-after-cap and redaction-format regressions and both comparison tests. A separate captured-output check accepted exactly two closed comparison lines and rejected any line outside a fixed grammar. Locally, the baseline version exited 0 with fixed output and empty stderr; the baseline first-ready event matched. Both Xcode-directory variants did not match within the unchanged two-second diagnostic deadline and had `other_nonempty` stderr without truncation. This is a local observation only; it neither proves an upstream startup cause nor predicts the CI-service result. Rust format and strict all-target/all-feature Clippy passed. Full offline `npm run verify` passed, including hook, repository, frontend and Rust suites and the native release build. No app was launched, provider/runtime contacted, or network download made.

Documentation, repository, security, whitespace, exact-scope, predecessor-preservation and session checks passed. The final report schema and ordinary gate disposition are checked separately after the report is frozen. Exact-head CI requires separate publication authorization and was not run in this successor.

## Architecture findings

Independent review found no new product boundary or module ownership change. The diagnostic remains an isolated test of cooperative fixture startup, not Hermes conformance, production runtime wiring or process-tree containment evidence. The one-line correction does not change fixture behavior or deadlines.

## Security findings

The test-only comparison emits closed stage, exit/signal, fixed-output-match, stderr-category and truncation fields. The scanner retains only a short suffix for fixed signatures while counting truncation independently; the regression detects a developer-tool signature beyond the 512-byte cap and verifies that synthetic sensitive text does not enter the formatted result. The captured-output grammar check accepted only fixed labels and numeric exit/signal fields. No raw stdout, stderr, paths, arguments, environment values or credentials were printed by the diagnostic. Existing local-first and execution boundaries remain unchanged.

## Code-health findings

The source comparison with the terminal predecessor found exactly one altered binding. Focused and full offline tests pass without changing the original seven test assertions or their deadlines. The two new comparisons intentionally pass when their baseline succeeds; a variant non-match remains reported evidence rather than an assertion that the Xcode path is a repair.

## Technical debt

Advisory: the fixed Xcode location and two-second startup comparison are host assumptions. The Target-Mac CI-service result for this exact successor is unverified, and the earlier seven-test `hermes_transport_spike` failure still blocks PR #123. Effort: a separately authorized diagnostic-branch publication and exact-head CI review before any repair. Milestone: before PR #123 acceptance or merge. This advisory does not block local completion.

## Roadmap findings

PR #123 remains open at `66090a0909d551c9a373ec142bb46b0f73ef0717`, with main at `fc6006e892c89cbc83d60f709875e4db3d8f18de`. Provider/runtime live-success, Codex-isolation, D-127 and D-128 advisories remain; D-125/M1/M2 stay parked. No publication or fixture repair is authorized by this local diagnostic.

## Completion decision

**PASS WITH ADVISORIES** is the local quality decision, conditional on final exact-scope, preservation, report-schema, and Stop validation. It is not PR #123 merge readiness or Target-Mac CI acceptance.

## Next-increment readiness

**Ready with advisories** for a separately authorized read-only publication review and, only if approved, a diagnostic branch push for exact-head Target-Mac evidence. Stop on ref drift, scope changes, unsafe output or a failed check. Do not infer Xcode-variant success or automatically repair the fixture.

## Exact files changed

- `src-tauri/tests/hermes_transport_spike.rs`
- `docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic.md`
- `docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-post-increment-review.md`
- `docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry.md`
- `docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry-post-increment-review.md`

## Exact commands executed

- `CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --offline --locked --test hermes_transport_spike`: Passed, 11 tests and one opt-in ignore.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`: Passed.
- `CARGO_NET_OFFLINE=true cargo clippy --manifest-path src-tauri/Cargo.toml --offline --locked --all-targets --all-features -- -D warnings`: Passed.
- `./node_modules/.bin/prettier --write docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry.md docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-compile-retry-post-increment-review.md`: Passed.
- `CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify`: Passed.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed, with no conflicts.
