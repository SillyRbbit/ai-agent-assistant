# PR #123 ACP fixture diagnostic preservation retry post-increment review

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
    "python3 /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/preserve.py > /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/post-verify.json",
    "python3 /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/scope.py",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_acp_transport_spike.rs",
    "docs/plans/2026-09-22-pr123-acp-fixture-diagnostic.md",
    "docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-post-increment-review.md",
    "docs/plans/2026-09-22-pr123-acp-fixture-diagnostic-format-retry.md",
    "docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-format-retry-post-increment-review.md",
    "docs/plans/2026-09-22-pr123-acp-fixture-diagnostic-preservation-retry.md",
    "docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-preservation-retry-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "One separately authorized correction after sanitized Target-Mac evidence is collected.",
      "milestone": "PR #123 Target-Mac fixture follow-up.",
      "risk": "The macOS CI service-context fixture startup cause is still unknown; the diagnostic branch collects a closed category and does not repair it. Provider/runtime live-success, Codex isolation, D-127 and D-128 remain advisory.",
      "severity": "Advisory",
      "summary": "Target-Mac fixture cause and related operational advisories remain unverified."
    }
  ],
  "increment_id": "pr123-acp-fixture-diagnostic-preservation-retry",
  "manual_verification": [
    {
      "check": "Exact seven-path scope, five inherited byte hashes, 20 historical worktree fingerprints, two prunable entries and both predecessor raw gate states",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health, technical-debt and readiness review of the complete candidate",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact-head Target-Mac diagnostic CI after branch publication",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
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
    },
    {
      "command": "python3 /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/preserve.py > /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/post-verify.json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/scope.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-09-22
Increment: pr123-acp-fixture-diagnostic-preservation-retry
Branch: codex/pr123-acp-fixture-diagnostic-preservation-retry

## Executive summary

The inherited ACP fixture diagnostic passed this separately admitted preservation retry. The only successor repository edits are this plan/review pair; seven cumulative paths are changed. No Rust implementation was repeated. Local quality is PASS WITH ADVISORIES; exact-head CI remains pending until publication.

## Scope and boundaries

All five inherited candidate files are byte-identical to the valid terminal formatting retry. The predecessor reports and raw gate states, PR #123, runner configuration, workflows, production code, dependencies and all other worktrees remain unchanged. The Rust diagnostic emits only an exit code or signal, fixed-output-match status and a closed stderr category when the existing `--version` probe fails.

## Verification results

This retry ran six focused ACP tests (all passed), Rust formatting, strict Clippy and full offline `npm run verify` (passed, including hook/repository/frontend/Rust tests and native release build). Documentation, repository, security, whitespace, session, seven-path scope and complete historical-worktree preservation checks passed. The external comparator derived the full 20-entry inventory from the recorded session instead of manually typed expected hashes, and confirmed both prunable entries. Earlier predecessor test results are history, not this retry’s validation.

## Architecture findings

No new runtime, provider, IPC, permission, dependency or production boundary is introduced. The inherited test-only diagnostic stays within the ACP fixture; the successor adds only documentation. No architecture blocker found.

## Security findings

The diagnostic line contains only fixed field names, numeric exit or signal, a Boolean fixed-output match and one closed stderr category. The redaction regression passed; raw stderr, stdout, paths, arguments and environment values are not printed by the diagnostic. No credentials or provider requests were used. No security blocker found.

## Code-health findings

The six ACP tests passed, including the inherited redaction regression. The two new documents pass Prettier and repository link checks. No new implementation code or code-health blocker was found.

## Technical debt

Advisory: the Target-Mac CI service-context fixture failure remains undiagnosed until exact-head diagnostic CI runs. A later fixture or runner correction requires separate authorization. Existing provider/runtime live-success, Codex-isolation, D-127 and D-128 advisories remain; they do not block this diagnostic branch.

## Roadmap findings

PR #123 stays at its original 32-file scope and is not declared merge-ready. The immediate bounded next action is publication of this separate diagnostic branch and read-only inspection of exact-head CI. D-125/M1/M2 stay parked.

## Completion decision

PASS WITH ADVISORIES. All required local commands and manual scope/preservation checks passed. Exact-head CI is an optional post-publication observation, not a claimed local result.

## Next-increment readiness

Ready with advisories for publishing only this diagnostic branch and inspecting its exact-head CI. The Target-Mac cause remains uncertain; no fixture repair is authorized.

## Exact files changed

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `docs/plans/2026-09-22-pr123-acp-fixture-diagnostic.md`
- `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-post-increment-review.md`
- `docs/plans/2026-09-22-pr123-acp-fixture-diagnostic-format-retry.md`
- `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-format-retry-post-increment-review.md`
- `docs/plans/2026-09-22-pr123-acp-fixture-diagnostic-preservation-retry.md`
- `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-preservation-retry-post-increment-review.md`

## Exact commands executed

- `env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike --locked --offline` — Passed.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check` — Passed.
- `env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings` — Passed.
- `env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify` — Passed.
- `npm run docs:check` — Passed.
- `npm run repository:check` — Passed.
- `npm run security:scan` — Passed.
- `git diff --check` — Passed.
- `python3 /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/preserve.py > /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/post-verify.json` — Passed.
- `python3 /private/tmp/cortexa-pr123-acp-fixture-diagnostic-preservation-retry-evidence/scope.py` — Passed.
- `python3 .codex/hooks/session_end_gate.py` — Passed.
