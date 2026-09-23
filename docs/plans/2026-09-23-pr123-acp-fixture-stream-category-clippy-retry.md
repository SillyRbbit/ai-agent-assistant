# PR #123 ACP fixture stream category Clippy retry

Status: Local validation passed; completion finalization pending
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal and admission evidence

The separately authorized successor corrects the four test-only `expect()` calls that caused strict Clippy to fail in the terminal `pr123-acp-fixture-stream-category` increment. That predecessor remains `failed` / `FAIL` / `Blocked` with a valid raw state and unchanged report. The completed diagnostic checkpoint is `a5aa4a8e89e3dca446db9ad4f0752fcee4de73bb`; live PR #123 remains at `66090a0909d551c9a373ec142bb46b0f73ef0717` and remote main at `fc6006e892c89cbc83d60f709875e4db3d8f18de` at admission.

The exact three-path failed candidate was transferred byte-for-byte without gate state. External preservation evidence verifies 23 predecessor worktrees, two existing prunable registry entries, and the four protected completed/terminal states. Ordinary admission for this increment passed before correction.

## Exact scope and non-goals

The successor delta is only `src-tauri/tests/hermes_acp_transport_spike.rs` and this new plan/review pair. The inherited failed plan/report remain byte-identical. The cumulative changed-path inventory against the committed diagnostic checkpoint is five paths. PR #123's 32-file scope is untouched.

Only replace the four prohibited new test `expect()` calls with error propagation and retain every assertion. Do not change the bounded stderr scanner, probe behavior, production code, runner settings, workflows, dependencies, policies, or fixture repair. Do not infer the child-process cause from a closed category.

## Verification and publication

Run focused offline ACP tests, Rust format and strict Clippy, full offline `npm run verify`, documentation/repository/security/whitespace checks, exact five-path cumulative and three-path successor scope checks, inherited-byte and all-worktree preservation, session and independent quality/readiness review, the exact 12-section report schema, ordinary finalization, complete/valid status, and a full-payload Stop hook. Only after all local checks pass may the separate diagnostic branch be committed and pushed. Inspect every exact-head CI job and report sanitized Target-Mac categories or their absence. Do not update or merge PR #123 or repair the fixture automatically.

## Risks, rollback, and stop conditions

Test failures must remain explicit without panicking through `expect`. A passing local suite does not establish Target-Mac fixture health. Stop on ref or preservation drift, admission failure, unsafe output, downloads, failed required validation, or expanded scope. On failure, preserve this worktree and record a truthful terminal FAIL; do not roll back predecessor evidence or create another successor automatically. Retain provider/runtime and Codex-isolation advisories, D-127/D-128, and parked D-125/M1/M2.

## Progress and final results

- 2026-09-23: Live refs and all predecessor records verified; candidate transferred exactly; external preservation passed; ordinary admission passed. Only the four test-only `expect()` calls were replaced with error propagation. Seven focused offline ACP tests, Rust format, strict Clippy, full offline `npm run verify` (including frontend, Rust, and native no-bundle release build), documentation, security, scope, preservation, session inventory, and whitespace checks passed. No provider/runtime request or native launch was performed. PR #123 remains untouched and its Target-Mac cause remains unconfirmed. Report-schema validation, ordinary finalization, and Stop remain pending.
