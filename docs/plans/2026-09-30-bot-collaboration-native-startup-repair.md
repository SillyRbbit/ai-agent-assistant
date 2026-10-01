# Bot Collaboration native startup repair

## Baseline and authorization

Owner-authorized native dispatch repair, 2026-09-30. Worktree:
`/Users/hdang/.codex/worktrees/bot-identity-publication/ai-agent-assistant`,
branch `codex/bot-identity-publication`, HEAD `6a164d8d8cd360dd1e4ecc67c5c530cc973a9518`.
Preserve all 45 existing candidate paths and complete/valid Bot Collaboration
history. Prior raw state and full bundle were archived and byte/mode verified in
`/private/tmp/cortexa-bot-collaboration-native-startup-repair-evidence` before ordinary admission.
No profile, room, workflow execution, provider call or publication is authorized.

## Evidence and bounded correction

Native QA persisted `room-1/run-1`, then the process aborted in tokio::spawn called
by synchronous start_collaboration. Restart marked the run and all four stages
interrupted without replay. The restored Nova/Orion/Mira settings were verified
after restart. No completed handoff or final output was observed.
Installed Tokio spawn requires current thread runtime context; its rejection
panics. Installed Tauri provides a supported global runtime handle with an inner
Tokio handle. Use that explicit handle at the existing synchronous command task
dispatch boundary, retaining the same Tokio abort/join type, lease, guards and
executor. No routes, profile/context, acknowledgement, limits, retries, data
schema, provider, graph or cancellation algorithm changes.

## Exact scope

- `src-tauri/src/collaboration_tauri.rs` (production dispatch helper and regressions)
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `TESTING_GUIDE.md`
- `docs/plans/2026-09-30-bot-collaboration-native-startup-repair.md`
- `docs/reviews/2026-09-30-bot-collaboration-native-startup-repair-post-increment-review.md`

Ten successor paths, 47 cumulative paths. Preserve historical document suffixes,
prior plan/review, raw completion archive, all other product bytes, other worktrees
and prunable entries. No dependency, workflow, governance or toolchain changes.

## Checklist and validation

- [x] Inspect completion, frozen candidate, sanitized crash and supported runtime source.
- [x] Archive raw predecessor state and previous complete debug bundle byte-identically.
- [x] Ordinary admission and exact ten-path scope frozen.
- [x] Red regression on existing spawn from plain synchronous thread.
- [x] Explicit Tauri runtime dispatch; regressions for execution and abort/join/lease release.
- [x] Focused native tests, format/strict Clippy, required full offline verification.
- [x] Updated offline unsigned debug bundle; old artifact unchanged.
- [x] Documentation/security/whitespace/preservation and session checks.
- [ ] Final report/schema, ordinary completion and full Stop receipts (external terminal evidence).
- [x] Handoff: native workflow QA remains separately authorized and unverified.

Reuse installed Python 3.12, Xcode/SDK 27.0 and documented Cargo build-override
strip=none process-locally; offline/locked dependencies and existing target.
Fixtures and tests must not inspect owner profile/history or send provider calls.
The focused dispatch regressions use the production helper from a thread without
Tokio context and verify cleanup, rather than hiding the boundary inside a test
runtime. Full native UI success still requires subsequent explicitly authorized QA.
Stop on conflicting drift, unsupported evidence, scope expansion or missing tools.
Repair recoverable in-scope failures within this increment, without a recovery chain.

## Progress and exact next action

Scope admitted. Implement the regression against old dispatch, observe the bounded
failure, then change only explicit runtime selection and rerun focused checks.
OpenAI remains parked at 4/5 used; retain D-127/D-128, native/provider/runtime and
Codex-isolation advisories, process-local native workaround and parked D-125/M1/M2.

Checkpoint: old dispatch failed both new plain-thread tests with the exact
missing-reactor panic. Corrected dispatch passed both tests. One test-only
struct-update compile error was repaired using explicit Session fields, preserving
Drop and every assertion. Next: strict native/full offline verification, package,
preservation and report gates. No owner profile/history or app process was used.

## Local acceptance checkpoint

Full offline verification passed: 481 frontend tests, 395 native library tests,
all applicable integrations and strict frontend/Rust/format checks. The existing
real-Hermes opt-in test remains ignored. Release build and unsigned offline debug
bundle passed using installed process-local tooling. No Cortexa launch or owner
workflow, database, profile, request or history operation occurred during repair.

The red test establishes the missing Tokio context cause; the same production
synchronous dispatch helper passes with the explicit Tauri handle. A second test
proves abort/join and generation-lease cleanup. This does not constitute a native
WebView/IPC walkthrough; that remains separately authorized next work. Original
executor, cancellation, routing, approval and pre-existing tests are byte-identical.

The previous full bundle is retained as `previous-Cortexa.app` in the evidence
directory; new artifact identity is in artifact.json. Ten successor paths and 47
cumulative paths are frozen. Final documentation/preservation/schema/session and
ordinary finalize/status/full Stop receipts must pass; consult their external
receipts rather than inferring completion from this checkpoint.

Recommended next bounded QA: one explicit new key-free Research Simulation run
in the updated app, with approved temporary Nova settings and restoration. Preserve
interrupted room-1/run-1; no automatic replay, provider call or deletion. All four
workflow native outcomes, live behavior and remote CI are still pending.
