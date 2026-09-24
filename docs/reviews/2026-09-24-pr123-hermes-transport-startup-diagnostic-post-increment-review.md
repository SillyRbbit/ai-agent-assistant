# PR #123 Hermes transport startup diagnostic post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "rustfmt --edition 2021 src-tauri/tests/hermes_transport_spike.rs",
    "cargo test --manifest-path src-tauri/Cargo.toml --offline --locked --test hermes_transport_spike",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "src-tauri/tests/hermes_transport_spike.rs",
    "docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic.md",
    "docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": true,
      "blocks_next_increment": true,
      "category": "Code health",
      "effort": "One separately authorized test-only binding correction, followed by the full required validation",
      "milestone": "Before any diagnostic-branch publication or PR #123 acceptance",
      "risk": "The new diagnostic target does not compile, so no startup comparison or redaction regression ran",
      "severity": "High",
      "summary": "Focused offline compilation failed with E0596 in the new first-ready helper"
    }
  ],
  "increment_id": "pr123-hermes-transport-startup-diagnostic",
  "manual_verification": [
    {"check": "Required refs, eight-path inherited commit, valid predecessor completion and preservation preflight", "required": true, "status": "Passed"},
    {"check": "Exact three-path successor scope and no merge conflicts", "required": true, "status": "Passed"},
    {"check": "Compiled diagnostic output contains only closed fields and redacts sensitive values", "required": true, "status": "Not run"},
    {"check": "Exact-head Target-Mac comparison after separate publication authorization", "required": false, "status": "Not run"}
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
    {"command": "rustfmt --edition 2021 src-tauri/tests/hermes_transport_spike.rs", "required": true, "status": "Passed"},
    {"command": "cargo test --manifest-path src-tauri/Cargo.toml --offline --locked --test hermes_transport_spike", "required": true, "status": "Failed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml --check", "required": true, "status": "Not run"},
    {"command": "cargo clippy --manifest-path src-tauri/Cargo.toml --offline --locked --all-targets --all-features -- -D warnings", "required": true, "status": "Not run"},
    {"command": "CARGO_NET_OFFLINE=true npm run verify", "required": true, "status": "Not run"},
    {"command": "npm run docs:check", "required": true, "status": "Not run"},
    {"command": "npm run repository:check", "required": true, "status": "Not run"},
    {"command": "npm run security:scan", "required": true, "status": "Not run"}
  ]
}
-->

Date: 2026-09-24
Increment: pr123-hermes-transport-startup-diagnostic
Branch: detached from e7c08480d1e998fc1b2e4dbe983fe0aa9f0bd286

## Executive summary

**FAIL.** The first focused offline test command failed to compile the new test-only diagnostic. It reported E0596 in `observe_first_ready_startup`: the local command binding was not mutable before `spawn`. The owner-directed stop condition applies. No diagnostic test ran, no comparison result was obtained, and no CI or publication followed.

## Scope and boundaries

The intended successor delta is exactly the Hermes transport integration test and this new plan/review pair. It is isolated from the verified eight-path ACP diagnostic commit. No application, fixture script, dependency, runner, workflow, PR #123, or predecessor file was edited. The diagnostic's uncompiled source remains preserved for separately authorized review; this failure is not a repaired fixture or a passing increment.

## Verification results

The initial refs, valid predecessor completion, settled exact-head CI and recorded 32-worktree/external-evidence preservation preflight matched. `rustfmt` and `git diff --check` passed. The focused offline Cargo command failed at compilation before running any test. Rust format check, strict Clippy, full offline verification, documentation, repository, security, redaction behavior and exact-head Target-Mac checks were **Not run** after that failure. `session_end_gate.py` found no conflicts. No provider or runtime request occurred.

## Architecture findings

The draft diagnostic is confined to a test-only host-mechanics fixture and introduces no product or runtime authority. It has not compiled or passed independent architecture review. Existing ACP and PR #123 boundaries remain unchanged.

## Security findings

The draft uses closed summary types and a bounded stderr signature scanner, but its redaction regression did not run. No claim of safe diagnostic output can be made. No credential, child output, process environment, or raw CI log was printed during this increment.

## Code-health findings

The first focused compile identified a test-only mutable-binding defect. Existing fixture assertions and deadlines were not intentionally changed. Because the target did not compile, no behavior or redaction result is accepted.

## Technical debt

High, completion-blocking: the test-only diagnostic cannot compile. A separate owner-approved correction and full validation are required before any diagnostic-branch publication.

## Roadmap findings

PR #123 remains open and unchanged. Provider/runtime live-success, Codex-isolation, D-127 and D-128 advisories remain; D-125/M1/M2 stay parked. This failed record grants no automatic successor or publication authority.

## Completion decision

**FAIL.** The focused test failed and required checks remain Not run. Close only through the ordinary terminal-failed gate. No completion marker may be issued.

## Next-increment readiness

**Blocked.** The smallest possible correction is a separately authorized test-only mutable binding change, then focused compilation, redaction tests, all applicable offline checks, independent review and valid completion before any exact-head Target-Mac evidence.

## Exact files changed

- `src-tauri/tests/hermes_transport_spike.rs`
- `docs/plans/2026-09-24-pr123-hermes-transport-startup-diagnostic.md`
- `docs/reviews/2026-09-24-pr123-hermes-transport-startup-diagnostic-post-increment-review.md`

## Exact commands executed

- `rustfmt --edition 2021 src-tauri/tests/hermes_transport_spike.rs`: Passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --offline --locked --test hermes_transport_spike`: Failed at compile error E0596; no test executed.
- `git diff --check`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed; no conflicts.
