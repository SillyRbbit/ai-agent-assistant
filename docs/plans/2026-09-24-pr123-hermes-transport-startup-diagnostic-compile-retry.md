# PR #123 Hermes transport startup diagnostic compile retry

Status: Local implementation and validation passed; gate disposition pending
Owner: Cortexa project owner
Last updated: 2026-09-24

## Goal and current evidence

The predecessor `pr123-hermes-transport-startup-diagnostic` is valid terminal `failed / FAIL / Blocked`, without a completion marker. Its first focused offline Cargo command stopped at E0596 before any test ran: `observe_first_ready_startup` calls `spawn` on a non-mutable `Command` binding. This successor fixes only that compile defect. No Target-Mac comparison result exists yet. PR #123 remains at `66090a0909d551c9a373ec142bb46b0f73ef0717`; main remains at `fc6006e892c89cbc83d60f709875e4db3d8f18de`.

## Exact scope and preservation

Start a separate detached worktree from diagnostic commit `e7c08480d1e998fc1b2e4dbe983fe0aa9f0bd286`. Transfer and byte-verify exactly the failed predecessor's Hermes transport test, plan and review without its raw gate state. Ordinary admission precedes corrective edits. The successor delta is only `src-tauri/tests/hermes_transport_spike.rs`, this plan and the matching review; the cumulative worktree change set is exactly five paths. Preserve all predecessor reports, raw states, completion markers, worktree fingerprints, external evidence and prunable entries.

## Design and non-goals

Make the local command binding mutable at the one E0596 site. Preserve all other inherited diagnostic bytes and every original fixture assertion, timeout, cleanup, environment isolation, output bound and redaction rule. Do not alter production code, Python fixture, dependencies, runner or workflow configuration, PR #123, or the parked D-125/M1/M2 lanes. The two comparison children remain diagnostic only; successful local tests are not CI-service evidence or a fixture repair.

## Validation and stop conditions

Run focused offline Hermes transport tests including the redaction regressions, Rust format and strict all-target/all-feature Clippy, full applicable offline verification, documentation/repository/security/whitespace checks, exact scope and predecessor-preservation checks, session and independent quality review, report-schema validation, ordinary finalization or truthful close-failed, and a full-payload Stop check. Record every executed and skipped command truthfully. Stop on ref drift, admission rejection, unsafe output, failed validation, downloads, scope expansion or any need to change fixture behavior or deadlines. Commit, push and exact-head CI require separate publication authorization; do not update or merge PR #123.

## Progress

- [x] Reverify refs, terminal predecessor, all worktree fingerprints and settled CI.
- [x] Create detached successor, transfer the three-path candidate without gate state, and pass ordinary admission.
- [x] Apply only the E0596 binding correction.
- [x] Pass the focused target (11 passed, one opt-in test ignored), Rust format, strict Clippy and full offline `npm run verify`.
- [x] Validate both local comparison lines against a closed output grammar; the baseline matched, while the Xcode-directory variants did not match within the unchanged two-second deadline. This is local evidence only.
- [ ] Run documentation, security, exact-scope and final preservation checks, then independent review.
- [ ] Validate the final report and complete ordinary gate disposition.
