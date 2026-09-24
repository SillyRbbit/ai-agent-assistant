# PR #123 ACP fixture timeout-stage diagnostic post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "rustfmt --edition 2021 src-tauri/tests/hermes_acp_transport_spike.rs",
    "git diff --check",
    "python3 -B preserve.py check",
    "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike",
    "python3 -B -m unittest discover -s .codex/hooks/tests -p test_post_increment_gate.py",
    "python3 -B -m unittest discover -s .codex/hooks/tests -p 'test_*.py'",
    "python3 -B -m unittest discover -s scripts/tests -p 'test_*.py'",
    "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic.md docs/reviews/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic-post-increment-review.md docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_post_increment_gate.py",
    "docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md",
    "src-tauri/tests/hermes_acp_transport_spike.rs",
    "docs/plans/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic.md",
    "docs/reviews/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": true,
      "blocks_next_increment": true,
      "category": "Code health",
      "effort": "Separately assess a startup-only timing repair using the identified local waits and exact-head Target-Mac evidence",
      "milestone": "Before PR #123 Target-Mac acceptance or merge",
      "risk": "The version probe and first initialize reads still exceed their two-second deadlines locally",
      "severity": "High",
      "summary": "The closed diagnostic identified five timeout sites; it did not repair the failing fixture"
    }
  ],
  "increment_id": "pr123-acp-fixture-timeout-stage-diagnostic",
  "manual_verification": [
    {"check": "Required refs, terminal predecessor, and exact transferred candidate", "required": true, "status": "Passed"},
    {"check": "Exact six-path scope and preserved original worktrees and prunable entries", "required": true, "status": "Passed"},
    {"check": "Exact-head Target-Mac validation of a future repair", "required": false, "status": "Not run"}
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
    {"command": "rustfmt --edition 2021 src-tauri/tests/hermes_acp_transport_spike.rs", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 -B preserve.py check", "required": true, "status": "Passed"},
    {"command": "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike", "required": true, "status": "Failed"},
    {"command": "python3 -B -m unittest discover -s .codex/hooks/tests -p test_post_increment_gate.py", "required": true, "status": "Passed"},
    {"command": "python3 -B -m unittest discover -s .codex/hooks/tests -p 'test_*.py'", "required": true, "status": "Passed"},
    {"command": "python3 -B -m unittest discover -s scripts/tests -p 'test_*.py'", "required": true, "status": "Passed"},
    {"command": "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic.md docs/reviews/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic-post-increment-review.md docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md", "required": true, "status": "Passed"},
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "cargo fmt --manifest-path src-tauri/Cargo.toml --check", "required": true, "status": "Not run"},
    {"command": "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings", "required": true, "status": "Not run"},
    {"command": "CARGO_NET_OFFLINE=true npm run verify", "required": true, "status": "Not run"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-09-23
Increment: pr123-acp-fixture-timeout-stage-diagnostic
Branch: detached from 66090a0909d551c9a373ec142bb46b0f73ef0717

## Executive summary

**FAIL.** The test-only diagnostic identified the expired local waits without
changing deadlines or fixture behavior. Its redaction regression passed, but
the five original focused ACP tests still failed with closed `Timeout` errors.
The candidate is uncommitted, unpublished, and not a PR #123 repair.

## Scope and boundaries

The owner-authorized cumulative scope is exactly six paths: the ACP integration
test, this plan/review pair, the report validator and its tests, and the report
template. The fixed macOS developer-directory addition was transferred byte-for-byte from the
terminal-failed repair candidate before admission. The predecessor plan,
report, raw state, and every other checkout remain unchanged. Production,
fixture script, dependencies, runner, workflow, timeout and existing assertion
bytes were not intentionally changed.

## Verification results

The focused offline target compiled and ran six tests. The new closed-label
redaction test passed. The five pre-existing tests failed: `--version` reached
the two-second probe-exit deadline; `unknown_method`, `noisy_stderr`, `cancel`,
and `hang_after_update` each reached the two-second response-read deadline with
zero frames received. In `hang_after_update`, the deliberate 120-millisecond
negative case was already handled before the later initialize read failed.
This identifies local wait sites, not the cause of child latency or CI-service
timing. The focused gate module passed 68 tests, the full hook suite passed 77,
and repository tests passed 85. The schema keeps skipped checks in
machine-readable verification while excluding them from the list of commands
actually executed. A false execution claim for a Not run command is rejected;
executed Passed and Failed checks still require an execution record. Rust format
and strict Clippy checks and full offline application verification were **Not
run** after the required focused ACP test failed. Targeted Markdown formatting,
documentation, repository, security, whitespace, exact-scope, predecessor
preservation, and session inventory checks passed. No CI workflow was rerun.

The skipped required commands remain explicitly **Not run**:

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`: Not run.
- `CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`: Not run.
- `CARGO_NET_OFFLINE=true npm run verify`: Not run.

## Architecture findings

The diagnostic is confined to a test host-mechanics spike. The schema amendment
changes only report validation for truthful Not run entries; required skipped
checks still force FAIL, and passing finalization remains blocked. It does not
add ACP conformance, Hermes runtime, product, or production boundary evidence. The
closed stage labels use existing test-owned scenario names and received-frame
counts; no architecture change followed.

## Security findings

Probe and scenario labels come from fixed allowlists. Unknown inputs map to
fixed unknown labels; the new regression checks that sentinel input is absent
from both display and debug forms. Child stdout, stderr, paths, arguments,
environment values and credentials are not emitted by the diagnostic. The
repository security scan passed for the amended scope. No production
authorization, secret handling, or model boundary changed.

## Code-health findings

The original `Timeout` code, cleanup, assertions, two-second and
120-millisecond deadlines remain in place. The diagnostic adds only a closed
site field to the existing test error. Gate regressions cover truthful Not run
terminal failure, false-execution rejection, missing-execution rejection for
Passed and Failed, and absence of a completion marker. The timeout-site result
is local and cannot be treated as a validated fixture fix.

## Technical debt

High, completion-blocking: the version probe and first initialize response
waits exceed two seconds locally with the Xcode developer directory. The
cause, a safe timing allowance, and Target-Mac full-suite behavior remain
unverified. A separate bounded test-only repair needs owner authorization.

## Roadmap findings

PR #123 remains open and unchanged. Provider/runtime live-success,
Codex-isolation, D-127 and D-128 advisories remain, and D-125/M1/M2 stay
parked. This diagnostic does not authorize another run or publication.

## Completion decision

**FAIL.** The failed ACP check remains a required failure. The schema amendment
allows skipped required checks to remain machine-readable without claiming they
ran. Ordinary `close-failed` is the only permitted disposition for this report;
it must produce a valid failed / FAIL / Blocked state without a completion
marker. No passing completion or PR #123 update is authorized.

## Next-increment readiness

**Blocked.** A startup-timing repair requires separate exact scope and owner
authorization, followed by full local and exact-head Target-Mac validation.

## Exact files changed

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `.codex/hooks/post_increment_gate.py`
- `.codex/hooks/tests/test_post_increment_gate.py`
- `docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`
- `docs/plans/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic.md`
- `docs/reviews/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic-post-increment-review.md`

## Exact commands executed

- `rustfmt --edition 2021 src-tauri/tests/hermes_acp_transport_spike.rs`: Passed.
- `git diff --check`: Passed.
- `python3 -B preserve.py check`: Passed for the 31 original registry entries.
- `CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike`: Failed; one new regression passed and five existing tests timed out at the closed sites above.
- `python3 -B -m unittest discover -s .codex/hooks/tests -p test_post_increment_gate.py`: Passed; 68 gate tests.
- `python3 -B -m unittest discover -s .codex/hooks/tests -p 'test_*.py'`: Passed; 77 hook tests.
- `python3 -B -m unittest discover -s scripts/tests -p 'test_*.py'`: Passed; 85 repository tests.
- `./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic.md docs/reviews/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic-post-increment-review.md docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`: Passed.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed.
