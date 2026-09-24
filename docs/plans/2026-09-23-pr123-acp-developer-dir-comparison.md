# PR #123 ACP child developer-directory comparison

Status: Active
Owner: Cortexa project owner
Last updated: 2026-09-23

## Goal

Compare the existing isolated ACP fixture version child with an otherwise
identical child that receives only the previously verified Xcode developer
directory. This is a diagnostic of the Target-Mac CI service context, not a
fixture repair.

## User-visible outcome

None. The test emits two bounded, closed startup summaries in CI.

## Scope

One existing Rust integration-test file and this plan/review pair, based on
diagnostic commit `c7fba47a37e595082008428c0eaccb89618db766`.

## Explicit non-goals

No production code, fixture semantics, runner or workflow configuration,
dependency changes, PR #123 update, automatic repair, or merge.

## Existing behavior and constraints

The original isolated fixture probe clears the child environment and retains
the existing version check and assertions. Its prior exact-head Target-Mac
result was a failed test step with a closed `developer_tool` category and
independent truncation flag; that category does not establish a root cause.

## Current-state evidence

At admission, PR #123 remained at `66090a0909d551c9a373ec142bb46b0f73ef0717`,
remote main at `fc6006e892c89cbc83d60f709875e4db3d8f18de`, and the committed
diagnostic baseline at `c7fba47a37e595082008428c0eaccb89618db766`.
The baseline completion marker was valid, and all 24 valid predecessor worktrees
and two prunable registry entries matched the external preservation inventory.

## Files expected to change

- `src-tauri/tests/hermes_acp_transport_spike.rs`
- `docs/plans/2026-09-23-pr123-acp-developer-dir-comparison.md`
- `docs/reviews/2026-09-23-pr123-acp-developer-dir-comparison-post-increment-review.md`

## Affected components

Test-only ACP fixture startup diagnostics.

## Interfaces and invariants

Both children use the same pinned Python, fixture, arguments, isolated setup,
bounded readers, deadline, and fixed expected version. The variant adds only
the known Xcode developer directory to that child. Output is limited to
exit code/signal, fixed-output-match, closed stderr category, and truncation.
Raw stderr, stdout, paths, arguments, environment values, and credentials are
never emitted.

## Implementation milestones

- [x] Verify refs, all worktrees, baseline gate status, and exact-head CI.
- [x] Admit an isolated successor and add the bounded comparison test.
- [ ] Complete required offline validation and gate review.
- [ ] Commit, push, and inspect exact-head CI only after valid local completion.

## Security and privacy considerations

The existing bounded readers retain at most one stdout frame and a 512-byte
stderr sample for classification. The new test returns only closed fields; it
does not make a provider request or print child payloads.

## Test plan

Run the focused ACP integration target, Rust format and strict Clippy, full
offline verification, documentation/repository/security/whitespace checks,
scope and preservation checks, then session/quality/post-increment gates.

## Verification commands

```bash
CARGO_NET_OFFLINE=true cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
CARGO_NET_OFFLINE=true cargo clippy --offline --locked --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify
npm_config_offline=true npm run docs:check
npm_config_offline=true npm run repository:check
npm_config_offline=true npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

## Risks

The two sequential children may see transient service changes. A closed
`developer_tool` category alone cannot establish the underlying startup cause.

## Rollback or failure strategy

Stop on drift, unsafe output, failed validation, or scope expansion. Freeze a
truthful terminal failure through the ordinary gate; do not alter predecessors
or automatically repair the fixture.

## Decisions made

Use a separate test-only comparison and leave the original version probe and
its assertions unchanged.

## Discoveries

Pending exact-head Target-Mac comparison.

## Progress

2026-09-23: Baseline, CI and preservation verified; comparison added under
ordinary admission. Required validation is pending.

## Acceptance criteria

- [ ] All required local checks and completion gates pass.
- [ ] Exactly the three declared paths differ from the diagnostic baseline.
- [ ] No raw child output or sensitive data is emitted.
- [ ] Separate branch exact-head CI is inspected after a valid local marker.

## Final results

Pending required validation and exact-head CI.

## Documentation updates

This exact owner-approved successor limits documentation to this new plan and
review. Existing historical project-memory documents remain unchanged.
