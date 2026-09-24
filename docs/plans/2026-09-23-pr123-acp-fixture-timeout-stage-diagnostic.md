# PR #123 ACP fixture timeout-stage diagnostic

Status: Focused ACP validation failed; terminal disposition is authoritative in gate state
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal and evidence

The valid terminal-failed `pr123-acp-fixture-developer-dir-repair` candidate
preserved the fixed macOS Xcode developer directory in the isolated fixture
child, but all five focused ACP tests reported only `Timeout`. The recorded
output does not identify which two-second probe or response wait expired. A
prior local version probe with the Xcode directory finished after two seconds
and before ten; the settled Target-Mac comparison passed under a ten-second
deadline while the unchanged full fixture suite failed. Neither observation
locates the five focused-test failures.

## Scope and non-goals

Start detached from PR #123 head `66090a0909d551c9a373ec142bb46b0f73ef0717`
and transfer only the failed candidate's test-file bytes, without its gate
state. The original three paths are
`src-tauri/tests/hermes_acp_transport_spike.rs`, this plan, and its matching
review. The owner separately authorized an in-place reporting-schema amendment
to unblock truthful terminal failure. Its exact cumulative six-path ceiling
adds `.codex/hooks/post_increment_gate.py`,
`.codex/hooks/tests/test_post_increment_gate.py`, and
`docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`. Preserve the terminal
predecessor, PR #123, all other worktrees, production code, fixture script,
runner and workflow settings, dependencies, timeouts, and all existing test
assertions. No commit, push, CI rerun, PR update, or fixture repair is authorized.

## Diagnostic design and invariants

Only an actual existing `Timeout` gains a closed site label: a fixed probe
category, or a fixed fixture scenario plus the bounded count of already
received frames. The test harness still kills and reaps timed-out children and
still returns `SpikeErrorCode::Timeout`; deliberate timeout assertions remain
unchanged. Unknown probe and scenario inputs map to fixed `unknown_*` labels.
No raw stdout, stderr, path, argument, environment value, or credential may
enter the emitted error. This is local test evidence, not ACP conformance or
Hermes product capability.

The schema amendment must list only actually executed commands in
`commands_executed`, retain skipped required checks as machine-readable
`Not run` verification entries, and reject a false claim that a skipped
command ran. Executed `Passed` and `Failed` verification entries must still
have an execution record. Required `Not run` or `Failed` checks must still
force `FAIL` and prevent a completion marker. No report or gate state from a
terminal predecessor may be rewritten.

## Milestones and validation

- [x] Verify refs, terminal record, 31 registry entries, and predecessor bytes.
- [x] Transfer the exact test-file candidate and pass ordinary admission.
- [x] Add closed timeout-site labels and a redaction regression.
- [x] Run the focused offline ACP target and record its closed failure sites.
- [x] Obtain separate authorization for the exact three-path schema addition.
- [x] Validate the schema amendment with 68 focused gate tests, 77 full hook
      tests, 85 repository tests, and applicable documentation, repository,
      security, whitespace, scope, and predecessor-preservation checks.
- [x] Freeze the unchanged focused ACP failure for ordinary terminal-failed
      disposition, without a completion marker.
- [ ] Run strict Clippy and full offline verification; stopped after the focused
      target failed.
- [ ] Complete passing ACP validation; blocked by the focused failure.

If a focused test still fails, the closed site may identify the expired wait,
but the increment must stop and be recorded as terminal failed; it is not a
repair. Stop on ref or evidence drift, unsafe output, failed validation,
downloads, scope expansion, or any need to change deadlines or application
behavior. Preserve all work without automatic repair or publication.

## Actual result

The new closed-label redaction test passed. The five existing tests still
failed with `Timeout`: the version probe reached its two-second exit deadline,
and the `unknown_method`, `noisy_stderr`, `cancel`, and `hang_after_update`
scenario children each reached the two-second read deadline before receiving
their first initialize frame. The expected 120-millisecond negative-case timeout
in the bounds test was handled before the later `hang_after_update` failure.
This local result identifies the expired waits but does not prove why the
children are slow, establish CI-service timing, or validate a longer deadline.
No fixture repair or publication followed. The draft report records the
remaining ACP and application checks as Not run. The pre-amendment gate rejected
terminal closeout because it required every verification command in the
executed-command list, including checks truthfully marked Not run. The separately
authorized schema amendment corrects only that status-to-execution relationship;
it does not turn the failed ACP validation into success. Focused and full hook,
repository, documentation, security, whitespace, and preservation checks passed.
Full application verification remains Not run after the focused ACP failure.
The gate state, rather than this living plan, is authoritative for whether
ordinary `close-failed` has completed and validated without a completion marker.
