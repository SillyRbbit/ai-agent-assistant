# PR #123 ACP fixture diagnostic preservation retry

Status: Locally validated; exact-head CI pending
Owner: Cortexa project owner
Last updated: 2026-09-22

## Goal and current evidence

Complete the separately authorized, sanitized Target-Mac ACP fixture diagnostic without changing its Rust implementation. PR #123 remains at head `66090a0909d551c9a373ec142bb46b0f73ef0717` and main at `fc6006e892c89cbc83d60f709875e4db3d8f18de`, with 32 PR-relative paths. Both predecessor increments have valid terminal `FAIL / Blocked` records. The second failure was an external comparator's manually mistyped expected hashes, not demonstrated checkout drift. A reviewed external comparator parsed the complete 20-worktree initial inventory, confirmed the two prunable entries, and verified every original worktree before this retry's admission.

## Exact scope and non-goals

The successor repository delta contains only this plan and `docs/reviews/2026-09-22-pr123-acp-fixture-diagnostic-preservation-retry-post-increment-review.md`. The cumulative PR-head-relative candidate contains seven paths: the inherited Rust test and both predecessor plan/review pairs, plus this pair. The inherited five files, raw predecessor gate states, PR #123, runner, workflows, source, dependencies, and all other checkouts remain unchanged. No fixture correction, production change, PR #123 update, merge, or automatic runner repair is authorized.

## Diagnostic boundary

The inherited test-only diagnostic emits only child exit code or signal, fixed-output-match status, and a bounded closed stderr category if the existing `--version` probe fails. It does not emit raw stderr, stdout, paths, arguments, or environment values. The existing ACP test semantics and redaction regression remain byte-identical.

## Validation and publication sequence

Run six focused offline ACP tests, Rust format and strict Clippy, full offline `npm run verify`, documentation/repository/security/whitespace checks, exact seven-path and inherited-byte checks, all-original-worktree and registry preservation checks, session and independent quality reviews, a complete report, ordinary finalization, valid completion status, and a passing Stop hook. Record earlier application checks only as predecessor history; this retry must run its own required checks. Only after valid local completion may this separate diagnostic branch be committed and pushed. Inspect exact-head CI, especially sanitized Target-Mac evidence, and do not implement a repair without separate authorization.

## Risks and stop conditions

Stop on PR/main ref drift, checkout or evidence drift, failed validation, unsafe output, missing offline dependencies, scope expansion, or need to alter PR #123 or runner configuration. Preserve terminal predecessor records and every checkout. Provider/runtime live-success and Codex-isolation advisories, D-127/D-128, and parked D-125/M1/M2 remain.

## Progress and final results

- 2026-09-22: Live PR/main refs, exact 32-file PR scope, terminal predecessor states, 20-entry historical inventory, two prunable entries, and all original worktree fingerprints verified. The five-path candidate was transferred byte-for-byte into an isolated branch without gate state. Existing local `node_modules` was cloned copy-on-write; ordinary admission succeeded. Focused ACP tests, Rust format, strict Clippy, full offline verification, documentation/repository/security/whitespace checks, session review, exact seven-path scope and all-worktree preservation passed. Final report validation and gate finalization remain pending.
