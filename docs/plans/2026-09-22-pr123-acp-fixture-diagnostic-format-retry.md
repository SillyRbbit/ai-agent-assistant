# PR #123 ACP fixture diagnostic formatting retry

Status: Terminal failed; publication stopped
Owner: Cortexa project owner
Last updated: 2026-09-22

## Goal and current evidence

Complete local validation of the already implemented, sanitized Target-Mac ACP fixture diagnostic. PR #123 remains open at head `66090a0909d551c9a373ec142bb46b0f73ef0717` against main `fc6006e892c89cbc83d60f709875e4db3d8f18de`, with exactly 32 PR-relative paths. The original diagnostic worktree has a valid terminal `FAIL / Blocked` record: six focused ACP tests, Rust format, and Clippy passed, while `npm run verify` stopped at Prettier on the earlier plan bytes.

The transferred terminal plan already passes targeted Prettier after its post-failure status update. The approved Rust diagnostic, its redaction regression, the terminal plan, and the terminal report are now inherited byte-for-byte. No application or test implementation is repeated.

## Exact scope and non-goals

The successor delta is exactly this plan and `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-format-retry-post-increment-review.md`. The cumulative PR-head-relative candidate contains five paths: the inherited Rust test, inherited diagnostic plan/report, and this plan/review pair. The original PR branch and all predecessor checkouts, reports, raw gate states, runner settings, and workflows remain unchanged.

No production code, fixture behavior, dependencies, permissions, provider/runtime traffic, PR #123 update, merge, or automatic Target-Mac repair is authorized. The diagnostic prints only exit code or signal, fixed-output-match status, and a closed stderr category when the existing `--version` probe fails. Raw child output, paths, arguments, and environment values must remain absent.

## Work and validation

Use the transferred candidate without further Rust edits. Run the six focused ACP tests offline, Rust format and strict Clippy, full offline `npm run verify`, documentation/repository/security and whitespace checks, exact five-path inventory and inherited-byte checks, session and independent quality reviews, a complete report, ordinary finalization, valid completion status, and the Stop hook. Verify PR head/main and original checkout preservation again before publication.

Only after every required local check passes, commit and push this separate diagnostic branch. Inspect exact-head CI, especially the Target-Mac ACP `--version` line, and report only the sanitized category. A failing Target-Mac application result is diagnostic evidence, not authority to alter the fixture or runner.

## Risks and stop conditions

Stop on ref or evidence drift, failed validation, unsafe output, scope expansion, unexpected inherited-byte change, unavailable offline dependencies, or a need to modify the original PR or runner. Do not reopen the predecessor terminal failure or implement an unapproved repair. Keep provider/runtime live-success and Codex-isolation advisories, D-127/D-128, and parked D-125/M1/M2 intact.

## Progress and final results

- 2026-09-22: Live refs and 32-path PR scope confirmed. The valid terminal candidate was transferred to a separate worktree without gate state. Existing local `node_modules` was cloned copy-on-write, and ordinary admission succeeded. Targeted Prettier found the inherited terminal plan already formatted, reducing the successor delta to this new plan/review pair. Validation remains pending.
- 2026-09-22: The focused offline ACP suite passed six tests; Rust format and strict Clippy passed. Full offline `npm run verify` passed, including repository and lint checks, 74 hook tests, 85 repository tests, 431 frontend tests, 368 Rust library tests, Rust integration tests, and a native release build. Final documentation, preservation, quality, and gate checks remain pending. Target-Mac CI has not run for this branch.
- 2026-09-22: Final documentation, repository, security, whitespace, and session checks passed. The first inline all-worktree preservation comparison stopped on a mistyped expected status hash for a predecessor checkout. A read-only comparison showed that checkout's actual hash still matched the original inventory, but the full validator was not completed. Under the owner's failed-check stop condition, this retry ended without repair, commit, push, or CI.
