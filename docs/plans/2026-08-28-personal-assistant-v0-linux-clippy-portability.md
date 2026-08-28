# Personal Assistant v0 Linux Clippy portability correction

Status: Verified complete with advisories; corrected-head PR checks pending
Owner: Henry Dang
Last updated: 2026-08-28
Baseline: `a346946d49b4537598197e3e5647f46e9efd3e7a`
PR: [#79](https://github.com/SillyRbbit/ai-agent-assistant/pull/79)

## Goal

Make the V0-1 Rust test modules compile warning-free on Linux by conditionally
importing only the symbols whose test consumers are already macOS-only.

## Evidence and cause

PR #79 CI run `33217662961`, Linux Rust job `99004869413`, passed checkout,
pinned Rust 1.90.0 installation, and formatting, then failed warning-denied
Clippy. The exact errors were unused imports in
`agent/gateway_request.rs` and `agent/native_runtime.rs`. Target-Mac Rust
Clippy and tests passed in job `99004869430`.

The affected imports are consumed only by tests already guarded with
`#[cfg(target_os = "macos")]`. macOS therefore used them while Linux correctly
reported them unused. No production code or runtime behavior failed.

## Exact files

- `src-tauri/src/agent/gateway_request.rs`
- `src-tauri/src/agent/native_runtime.rs`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-linux-clippy-portability.md`
- `docs/increments/personal-assistant-v0-linux-clippy-portability.md`
- `docs/reviews/2026-08-28-personal-assistant-v0-linux-clippy-portability-post-increment-review.md`

No other file may change.

## Implementation

- Keep cross-platform test imports unconditional.
- Move `InitialGatewayEvent`, `InitialGatewayTurnError`, and the relevant
  gateway protocol version import behind the same macOS target guard as their
  consumers.
- Keep `InitialGatewayTurn` cross-platform where a non-macOS test uses it.
- Add no allowance, lint suppression, behavior branch, test skip, dependency,
  or workflow change.

## Invariants and non-goals

- Production request, runtime, cancellation, quarantine, tool, approval,
  provider, and public API behavior is byte-for-byte unchanged.
- Test bodies, assertions, fixtures, limits, and platform coverage are
  unchanged.
- No manifest, lockfile, dependency, workflow, runner, capability, CSP,
  permission, Tauri, UI, provider, network, credential, persistence,
  filesystem, tool, background, or device-effect change.
- V0-2 and every later capability remain out of scope and Blocked.

## Verification

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml agent::gateway_request
cargo test --manifest-path src-tauri/Cargo.toml agent::native_runtime
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run test:agent-acceptance
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Manual review must confirm that only test-import visibility changed. After the
locally verified correction is committed and pushed, PR #79 Linux Rust,
target-Mac Rust, frontend, dependency/secret, classifier, and documentation
checks must all pass before squash merge. Hosted CI is a publication gate, not
a substitute for the local completion marker.

## Rollback

Before merge, revert only the correction commit. After merge, revert the
squash commit if the import correction causes an unexpected platform build
regression. No external resource or credential rollback exists.

## Stop conditions

Stop on any need to change production behavior, a test body, a workflow,
runner, dependency, manifest, lockfile, lint policy, platform coverage, or a
file outside the exact list. Stop publication if any required PR check fails.

## Acceptance criteria

- [x] Linux-only unused-import warnings are structurally impossible without an
      allowance or skipped test.
- [x] Focused and full local verification pass.
- [x] Required documentation and completion marker pass.
- [ ] PR #79's exact corrected head passes every required check before merge.
