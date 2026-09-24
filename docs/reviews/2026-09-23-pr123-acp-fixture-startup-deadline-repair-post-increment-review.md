# PR #123 ACP fixture startup-deadline repair post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "pr123-acp-fixture-startup-deadline-repair",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "files_changed": [
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_post_increment_gate.py",
    "docs/plans/2026-09-23-pr123-acp-fixture-startup-deadline-repair.md",
    "docs/plans/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic.md",
    "docs/reviews/2026-09-23-pr123-acp-fixture-startup-deadline-repair-post-increment-review.md",
    "docs/reviews/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic-post-increment-review.md",
    "docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md",
    "src-tauri/tests/hermes_acp_transport_spike.rs"
  ],
  "commands_executed": [
    "rustfmt --edition 2021 src-tauri/tests/hermes_acp_transport_spike.rs",
    "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-fixture-startup-deadline-repair.md docs/reviews/2026-09-23-pr123-acp-fixture-startup-deadline-repair-post-increment-review.md",
    "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings",
    "CARGO_NET_OFFLINE=true npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-pr123-acp-fixture-startup-deadline-repair-evidence/preserve.py check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "verification": [
    {
      "command": "rustfmt --edition 2021 src-tauri/tests/hermes_acp_transport_spike.rs",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-fixture-startup-deadline-repair.md docs/reviews/2026-09-23-pr123-acp-fixture-startup-deadline-repair-post-increment-review.md",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "CARGO_NET_OFFLINE=true npm run verify",
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
      "command": "python3 -B /private/tmp/cortexa-pr123-acp-fixture-startup-deadline-repair-evidence/preserve.py check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Required live refs, valid terminal records, Xcode prerequisite, six-path byte transfer, and ordinary admission",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Separate architecture, security, code-health, technical-debt, and readiness review of the successor diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Eight-path inventory, protected inherited bytes, and all 32 predecessor registry entries including both prunable entries",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Linux and full Target-Mac CI for this repair commit, after separate publication authorization",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Local startup repair passes; repair-commit CI remains unverified",
      "risk": "The historical successful comparison child is not proof that the repaired full suite passes in the CI service",
      "effort": "Owner-authorized separate publication and inspection of all resulting checks",
      "milestone": "Before a PR #123 repair or merge decision",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Fixed Xcode location and finite macOS startup allowance remain platform assumptions",
      "risk": "Another macOS host may lack the fixed developer directory or exceed ten seconds; Linux runtime behavior needs its own validation",
      "effort": "Retain the prerequisite and stop on failure rather than widening limits or changing runner settings",
      "milestone": "Before use on another host",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Existing product and audit advisories remain unchanged",
      "risk": "Provider/runtime live-success, Codex isolation, D-127 audit debt and D-128 custody/abort limits remain; no live authority is granted",
      "effort": "Separate owner-scoped work only; keep D-125/M1/M2 parked",
      "milestone": "Before any affected operational claim",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-09-23
Increment: pr123-acp-fixture-startup-deadline-repair
Branch: detached from 66090a0909d551c9a373ec142bb46b0f73ef0717

## Executive summary

**PASS WITH ADVISORIES.** The bounded macOS startup repair passed focused ACP tests and full offline verification locally. It does not establish Linux or Target-Mac CI success for this candidate. The successor remains detached, uncommitted and unpublished; PR #123 is unchanged.

## Scope and boundaries

The successor delta is exactly the ACP integration test and this new plan/review pair. The cumulative inventory is eight paths, including the byte-identical inherited gate implementation/tests, report template, and terminal timeout-diagnostic plan/review. All 32 original registry entries, both prunable entries, changed predecessor bytes, raw states, historical records and external evidence remain frozen and unchanged. No gate state was copied. Existing local node_modules and build outputs were cloned copy-on-write for offline validation; nothing was installed or downloaded. Production, Python fixture, runner, workflow, dependency and permission bytes remain unchanged.

## Verification results

The focused offline ACP target passed all seven tests: the five original behavior tests, inherited closed-label redaction regression, and new deadline-selection regression. Rust formatting and strict all-target/all-feature Clippy passed. Full offline npm run verify passed: 77 hook tests, 85 repository tests, 431 frontend tests, 368 Rust library tests, all integration targets (249 passed, one pre-existing ignored test), type/lint checks, frontend builds and the native release build. The seven-test ACP target also passed within that full run. Documentation, repository, security, whitespace, exact scope, protected bytes, predecessor preservation and session checks passed. Local release compilation did not launch the app. No CI run or provider request was made. Linux and full Target-Mac validation of the repair remain pending separate publication authorization; the ignored real-Hermes version probe does not become passing evidence.

## Architecture findings

A separate review of the successor diff found no production boundary change. A fixed macOS STARTUP_DEADLINE is used only for finite version/check/flood probes, initialize response and early_exit initial exit observation. Linux and other supported Unix targets retain two seconds. The inherited governance amendment is unchanged; no new runtime, abstraction layer or product capability was added. Cooperative fixture results remain host-mechanics evidence, not Hermes/ACP conformance or process-tree containment proof.

## Security findings

The patch does not add environment inheritance or output. env_clear, the fixed macOS developer-directory value, executable validation, bounded readers, closed errors, cleanup and every original assertion remain. Deliberate hanging-probe timing, both 120-millisecond negative tests, later protocol waits and graceful-shutdown deadlines are unchanged. The repository secret scan passed. No credential inspection, raw child data, network request, production authority or permission change was introduced.

## Code-health findings

The startup deadline is ten seconds only on macOS. Probe selection uses the exact finite version/check/flood allowlist; hanging and unknown probes retain two seconds. The same-file regression checks this selection plus unchanged protocol and negative-case budgets. The early_exit observation receives startup time while midstream_exit keeps the normal protocol deadline. Separate review found no unrelated successor hunks and no new blocking finding. Passing local behavior does not identify the root cause of platform latency.

## Technical debt

Advisory: the fixed Xcode installation path and ten-second budget remain portability prerequisites, not universal latency guarantees. Failures must stop without automatic timeout increases. Advisory: repaired CI-service behavior remains unverified until separate publication and successful checks on that exact commit. Existing D-127 audit debt is unchanged.

## Roadmap findings

PR #123 and main retain their required refs. The inherited key-free six-choice Agents observation remains historical evidence, not a new walkthrough. Provider/runtime live-success, Codex-isolation and D-128 native-session custody/remote-abort advisories remain. No new live allowance exists. D-125/M1/M2 stay parked. Root project-memory documents are outside the exact authorized successor delta; this plan/review records current local evidence without rewriting history.

## Completion decision

**PASS WITH ADVISORIES.** All required local checks passed. The gate state and its workspace-bound marker are authoritative for ordinary finalization validity. Historical predecessor FAIL records remain immutable and are not promoted by this result. No commit, push, CI rerun, PR update or merge is authorized.

## Next-increment readiness

**Ready with advisories** for a separately authorized publication-readiness review and separate branch push to obtain Linux and full Target-Mac CI evidence. This is not PR #123 merge readiness. Recheck refs, exact scope, valid completion and preservation first. Stop on drift, unsafe output, failed checks, missing prerequisites or scope expansion; do not automatically repair or increase deadlines.

## Exact files changed

- `.codex/hooks/post_increment_gate.py`
- `.codex/hooks/tests/test_post_increment_gate.py`
- `docs/plans/2026-09-23-pr123-acp-fixture-startup-deadline-repair.md`
- `docs/plans/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic.md`
- `docs/reviews/2026-09-23-pr123-acp-fixture-startup-deadline-repair-post-increment-review.md`
- `docs/reviews/2026-09-23-pr123-acp-fixture-timeout-stage-diagnostic-post-increment-review.md`
- `docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`
- `src-tauri/tests/hermes_acp_transport_spike.rs`

## Exact commands executed

- `rustfmt --edition 2021 src-tauri/tests/hermes_acp_transport_spike.rs`: Passed.
- `./node_modules/.bin/prettier --write docs/plans/2026-09-23-pr123-acp-fixture-startup-deadline-repair.md docs/reviews/2026-09-23-pr123-acp-fixture-startup-deadline-repair-post-increment-review.md`: Passed.
- `CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike`: Passed.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: Passed.
- `CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`: Passed.
- `CARGO_NET_OFFLINE=true npm run verify`: Passed.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 -B /private/tmp/cortexa-pr123-acp-fixture-startup-deadline-repair-evidence/preserve.py check`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed.
