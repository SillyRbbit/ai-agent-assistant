# PR #123 ACP fixture diagnostic formatting retry post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike --locked --offline",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings",
    "env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 - <<'PY' (inline five-path and all-worktree preservation comparator)"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_acp_transport_spike.rs",
    "docs/plans/2026-09-22-pr123-acp-fixture-diagnostic.md",
    "docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-post-increment-review.md",
    "docs/plans/2026-09-22-pr123-acp-fixture-diagnostic-format-retry.md",
    "docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-format-retry-post-increment-review.md"
  ],
  "findings": [],
  "increment_id": "pr123-acp-fixture-diagnostic-format-retry",
  "manual_verification": [
    {
      "check": "Complete exact-scope and all-worktree preservation comparison",
      "required": true,
      "status": "Failed"
    },
    {
      "check": "Exact-head Target-Mac CI diagnostic after branch publication",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
    {
      "command": "env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike --locked --offline",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-09-22
Increment: pr123-acp-fixture-diagnostic-format-retry
Branch: codex/pr123-acp-fixture-diagnostic-format-retry

## Executive summary

The previously implemented, sanitized ACP `--version` diagnostic passed focused and full offline verification in a separate worktree. No Rust code was changed in this successor. An inline preservation comparison failed because its expected hash was mistyped. The owner's failed-check stop condition ended this retry before completion or publication; no exact-head Target-Mac CI result is claimed.

## Scope and boundaries

The successor delta contains only this plan/review pair. The cumulative candidate has five PR-head-relative paths: the inherited Rust test, inherited failed plan/report, and this pair. PR #123's original 32-file branch, all predecessor worktrees and terminal evidence, runner configuration, workflows, production code, dependencies, and permissions are outside scope.

## Verification results

The focused offline ACP suite passed six tests, including closed-category redaction. Rust format and strict Clippy passed. Full offline `npm run verify` passed: 74 hook tests, 85 repository tests, 431 frontend tests, 368 Rust library tests, integration tests, frontend builds, and the native release build. Report-inclusive documentation, repository, security, whitespace, and session checks passed. The inline all-worktree preservation comparison stopped at one mistyped expected hash. A subsequent read-only check showed that checkout's actual status hash still matched the initial inventory, but the remaining comparisons were not completed. No Target-Mac diagnostic from this branch exists yet.

## Architecture findings

The transferred test-only diagnostic adds no production interface, runtime ownership, dependency, or provider authority. The successor adds only documentation. No architecture blocker was found in the bounded diff.

## Security findings

The inherited diagnostic exposes only numeric child exit code or signal, a Boolean fixed-output-match status, and a closed stderr category on `--version` failure. Its regression proves private stderr text does not enter the diagnostic line. No raw stderr, stdout, paths, arguments, or environment values are intentionally emitted. This successor does not alter the Rust bytes or request behavior.

## Code-health findings

Existing ACP test semantics and assertions remain intact, and the inherited redaction regression passed. No new implementation code was added. The predecessor plan's terminal bytes are already Prettier-compliant; the earlier failed run occurred before its terminal status update.

## Technical debt

The Target-Mac fixture startup cause remains unknown until this separate branch runs in the failing CI service context. That diagnostic evidence cannot itself prove a runner or application repair. Provider/runtime and Codex-isolation advisories, D-127/D-128, and parked D-125/M1/M2 remain.

## Roadmap findings

This branch was limited to collecting sanitized Target-Mac evidence. PR #123 remains unchanged and its Target-Mac blocker is not declared resolved. The failed preservation check prevents local completion and publication under the owner's stop condition.

## Completion decision

FAIL. The all-worktree preservation comparator used a mistyped expected hash and exited nonzero. No checkout drift was established for that path, but the complete required comparison did not pass. Stop without repair or publication under the explicit owner condition.

## Next-increment readiness

Blocked. The terminal failed increment cannot be promoted to completion. A separate owner-authorized retry would need to establish preservation with a correct, reviewed comparator before any publication.

## Exact files changed

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `docs/plans/2026-09-22-pr123-acp-fixture-diagnostic.md`
- `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-post-increment-review.md`
- `docs/plans/2026-09-22-pr123-acp-fixture-diagnostic-format-retry.md`
- `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-format-retry-post-increment-review.md`

## Exact commands executed

- Focused offline ACP test command above — Passed, six tests.
- Rust format and strict Clippy commands above — Passed.
- Full offline `npm run verify` command above — Passed.
- `npm run docs:check`, `npm run repository:check`, `npm run security:scan`, and `git diff --check` — Passed with this report present before the final failure disposition edits.
- Inline `python3 - <<'PY'` five-path/all-worktree comparator — Failed on a mistyped expected hash; full comparison was not completed.
