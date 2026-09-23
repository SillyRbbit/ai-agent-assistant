# PR #123 ACP fixture streamed stderr category

Status: Terminal failed; publication stopped
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal and evidence

The separately published ACP fixture diagnostic at `a5aa4a8e89e3dca446db9ad4f0752fcee4de73bb` is locally complete and valid. Its exact-head Target-Mac test job reported exit code 1, no matching version output, and `stderr_category=truncated`; the existing 512-byte capture limit and truncation-first category prevented a known startup signature from being distinguished. The same five ACP fixture tests failed in PR #123's earlier Target-Mac job before this diagnostic was added. The underlying child-process cause remains unknown.

PR #123 stays open at head `66090a0909d551c9a373ec142bb46b0f73ef0717`; main stays at `fc6006e892c89cbc83d60f709875e4db3d8f18de`. Before ordinary admission, the 22 valid predecessor worktrees, two prunable registry entries, three completion/terminal state hashes, and prior report/test bytes were frozen and checked. No predecessor worktree or gate state is copied.

## Exact scope and non-goals

The successor delta is only `src-tauri/tests/hermes_acp_transport_spike.rs` and this plan/review pair. It inherits seven committed diagnostic paths and adds two new documents, for nine PR-head-relative paths on this separate branch. PR #123's 32-file scope remains unchanged. No production code, fixture protocol, runner, workflow, dependency, credential, permission, policy, or original PR edit is authorized.

## Diagnostic behavior

Keep the existing 512-byte stderr sample cap, byte count cap, read-failure precedence, and probe failure behavior. A bounded rolling suffix recognizes only the existing developer-tool, loader-architecture, and Python-startup signatures across stderr chunks, including after the retention cap. The diagnostic still emits only numeric exit or signal, Boolean output match, one closed category, and now an independent Boolean truncation field. Unknown long stderr remains `truncated`; no raw child output, paths, arguments, or environment values enter the diagnostic line.

## Verification and publication

Run the focused offline ACP suite, Rust format and strict Clippy, full offline `npm run verify`, documentation/repository/security/whitespace, exact three-path scope and predecessor preservation checks, session and independent quality/readiness review, report-schema validation, ordinary finalization, complete/valid status, and the full-payload Stop hook. Only after valid local completion, commit and push this separate branch and inspect every exact-head CI job. Report only sanitized Target-Mac fields and stop without automatic fixture repair or PR #123 update.

## Risks and stop conditions

A recognized signature is a category, not proof of a complete upstream cause; if no known token appears, the result may remain `truncated`. Stop on drift, admission failure, unsafe output, failed required validation, downloads, or scope expansion. Preserve all historical failures, provider/runtime and Codex-isolation advisories, D-127/D-128, and parked D-125/M1/M2.

## Progress and final results

- 2026-09-23: Live refs, completion and terminal records, all-worktree preservation, and prior Target-Mac evidence verified. A new isolated worktree was created from the exact diagnostic commit. Existing local formatter tooling was cloned copy-on-write; ordinary admission passed. The bounded test-file change and its seven focused offline ACP tests passed. Rust formatting passed. Strict Clippy failed on four new test-only `expect()` calls (`clippy::expect_used` under `-D warnings`). The owner-directed stop condition prevents repair in this increment; full offline verification and later publication checks were not run. No commit, push, PR update, runner change, or fixture repair occurred. The failed report records a terminal FAIL without a completion marker.
