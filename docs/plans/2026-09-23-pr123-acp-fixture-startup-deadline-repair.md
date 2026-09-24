# PR #123 ACP fixture startup-deadline repair

Status: Local implementation and validation passed; gate state controls completion
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal and current evidence

Test a macOS-only ten-second startup allowance for the isolated ACP fixture.
PR #123 remains at `66090a0909d551c9a373ec142bb46b0f73ef0717`; main remains
at `fc6006e892c89cbc83d60f709875e4db3d8f18de`. The settled Target-Mac comparison
at `5a1019c2613163224112c38cafe36c5cc97c8810` reported baseline exit 1,
output mismatch, developer_tool and truncation; the Xcode-directory child
exited 0 with matching fixed output and empty stderr. That ten-second probe
does not establish full-suite success. The valid terminal timeout diagnostic
identified the version-probe exit wait and four first-initialize response waits
at two seconds locally. Preserve those failures and their limits.

## Exact scope and preservation

Create this detached successor from the verified PR head and transfer the exact
six-path timeout-diagnostic candidate without its gate state. The successor
delta is only `src-tauri/tests/hermes_acp_transport_spike.rs`, this plan, and
the matching post-increment review: exactly eight cumulative changed paths.
The inherited gate implementation/tests, review template, and predecessor
plan/review remain byte-identical. External machine-generated evidence freezes
all 32 preceding worktree registry entries, both prunable entries, raw gate
states, reports, existing changes, and prior external evidence. Source checkout
and all predecessor records remain unchanged. Existing local dependency/build
outputs were cloned copy-on-write for offline validation; no install or package
resolution is authorized.

## Design and invariants

On macOS only, finite version/check/flood probes, the first initialize response,
and early_exit's initial exit observation receive a ten-second allowance.
Linux retains two seconds. Preserve the deliberate hanging probe's two seconds,
both 120-millisecond negative waits, all later protocol response and shutdown
deadlines, env_clear, fixed child environment, cleanup, closed errors and every
existing assertion. Add same-file deadline-selection regressions. No production,
fixture script, runner, workflow, dependency, permission, or PR #123 changes.
No further timeout increase or automatic repair is permitted.

## Validation and stop conditions

Run focused offline ACP tests, Rust formatting, strict all-target/all-feature
Clippy, full offline npm run verify, documentation, repository, security,
whitespace, exact scope/preservation, session and quality review, report schema,
ordinary finalization or truthful close-failed, and full-payload Stop.
Record all skipped required checks as Not run. Local results cannot replace
future exact-commit Linux and full Target-Mac validation, which requires separate
publication authorization. Stop on drift, missing Xcode/tooling prerequisites,
timeouts, failed validation, unsafe output, downloads, or scope expansion.

## Risks and non-goals

The fixed Xcode path limits macOS portability; ten seconds is a bounded
evidence-supported hypothesis, not a latency guarantee. Slow startup failures
can take longer to report. This fixture is not Hermes/ACP conformance or process
containment proof. Retain provider/runtime live-success, Codex-isolation,
D-127 and D-128 advisories. Keep D-125/M1/M2 parked. No live request, native
launch, commit, push, CI rerun, PR update, merge, or automatic successor.

## Progress and actual results

- [x] Verify refs, predecessor terminal status, settled CI, and prerequisites.
- [x] Freeze preservation evidence and byte-verify the six-path transfer.
- [x] Admit this separate increment and implement the bounded test-only change.
- [x] Run the required local validation and record actual results.
- [x] Prepare the final report for read-only schema validation and ordinary disposition.

The seven focused ACP tests passed, as did Rust format, strict Clippy and full
offline verification (77 hook, 85 repository, 431 frontend, 368 Rust library
tests and all integration targets; native release build passed). Documentation,
repository, security, whitespace, eight-path scope and 32-entry predecessor
preservation checks passed. No downloads or live requests occurred. Linux and
full Target-Mac CI for this repair remain pending separate authorization.
The local quality/readiness decision is PASS WITH ADVISORIES / Ready with
advisories. The gate state is authoritative for terminal disposition.
