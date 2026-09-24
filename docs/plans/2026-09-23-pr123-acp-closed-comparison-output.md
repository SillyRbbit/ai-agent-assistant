# PR #123 closed ACP comparison output

Status: Validation passed; completion pending
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal

Expose the existing two closed ACP fixture startup summaries in the Target-Mac
CI service context. The comparison test passed at diagnostic commit
`39c36ba37be424d0fe389e16300e1c7cb5db1634`, but Rust's default capture hid
its `eprintln!` summaries. The five original fixture tests still failed with
the closed exit-1, output-mismatch, `developer_tool`, truncated result.

## Scope and non-goals

Start in a new detached worktree from that exact commit. Change only
`.github/workflows/ci.yml` and this new plan/review pair. Add one focused
Target-Mac step before the unchanged full Rust tests. Run only
`compares_isolated_version_startup_with_xcode_child_only` with `--exact
--show-output`, so passing output for that one diagnostic is visible. Preserve
all other workflow jobs, tests, product bytes, dependencies, runner settings,
predecessor records, and PR #123. No fixture repair or CI rerun is authorized
by this local implementation.

## Interfaces and safety

The test prints only closed exit/signal, fixed-output-match, stderr-category,
and truncation fields. Its existing bounded stdout/stderr readers prevent raw
payload output. Keep Rust test capture for every other test. The full
`cargo test --all-targets --locked` step and its original failure behavior
remain unchanged. A passing comparison test proves only that both children
finished before the ten-second deadline; it does not assert successful startup.

## Validation and stop conditions

Run a focused offline invocation of the exact command and inspect only its two
closed summaries. Verify workflow YAML and the one-step diff, Rust formatting
and strict Clippy, full offline `npm run verify`, documentation/repository/
security/whitespace checks, exact three-path scope, predecessor/worktree
preservation, session and quality review, report schema, ordinary finalization,
valid completion status, and full-payload Stop hook. Stop on ref or evidence
drift, missing or unsafe output, failed validation, downloads, scope expansion,
or any need to change the fixture, runner, or existing full test step. Do not
commit, push, rerun CI, update PR #123, or merge without separate approval.

## Risk and rollback

The focused step runs the fixture twice more on Target-Mac and may still leave
the full test step failing. A green focused test alone cannot identify either
child's status. If either summary is absent or unsafe, preserve the failed
increment and stop. The change can be removed from the isolated worktree only
under a separately authorized correction; predecessor records remain intact.

## Progress and final results

2026-09-23: Exact predecessor and remote refs, valid completion marker, and
settled exact-head CI confirmed before isolated admission. The one-step diff,
focused offline test with two closed summaries, Rust format, strict Clippy,
full offline `npm run verify`, documentation, repository, security, whitespace,
exact-scope, predecessor-preservation, and session checks passed. Independent
architecture, security, code-health, debt, and readiness review found no
blocking issue. The exact report schema passed; ordinary finalization and
the Stop hook remain pending.
