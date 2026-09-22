# Direct provider stream diagnostics message assertion

Date: 2026-09-21. Increment:
`direct-provider-stream-diagnostics-message-assertion`.
Status: Terminal failed.

## Goal and authorization

The owner explicitly authorized this isolated corrective successor after the
terminal-failed `direct-provider-stream-diagnostics` increment. Correct only the
new UI test table's incompatible policy-message expectation. Preserve all
production behavior, every other test assertion, predecessor reports and raw
gate states. Do not reopen, rewrite or reclose the failed increment.

## Baseline and candidate transfer

The successor is detached at baseline
`0ed15810e90b6a4bd312a8096c61b0abb1ab7eff` in
`/private/tmp/cortexa-direct-provider-stream-diagnostics-message-assertion`.
The exact 36-path terminal candidate was transferred byte-for-byte from
`/private/tmp/cortexa-direct-provider-stream-diagnostics`; its raw gate state
was not copied. Ordinary admission succeeded under this successor's distinct
increment ID.

All predecessor checkouts, dirty changes, finalized reports and raw states must
remain byte-identical. The two already-prunable registry entries remain listed
without inspecting, pruning, repairing or recreating their directories.

## Exact scope

Enforce this exact 11-path successor delta separately from the 36-path inherited
candidate. The cumulative Git inventory is 38 paths after adding this plan and
the successor review.

- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-21-direct-provider-stream-diagnostics-message-assertion.md`
- `docs/reviews/2026-09-21-direct-provider-stream-diagnostics-message-assertion-post-increment-review.md`

All production, dependency, Rust, client-test and component bytes must remain
identical to the terminal candidate. All historical plans and reports, including
the terminal failure report, remain byte-identical.

## Correction

Add one explicit expected policy-message value to every row in the existing
parameterized UI test. The `network`, `http_status` and generic
`provider_stream` rows retain `No mock response was substituted.`. The three
narrowed stream-code rows retain `No automatic retry was made.`. Change only the
shared policy-message assertion to consume that row value.

Preserve the existing exact-message assertion and all separate 250ms timer acts,
streaming transition, terminal cleanup, control release, partial-output label,
one-Start and no-extra-poll assertions. Do not change production messages to
satisfy the test.

## Validation

Use key-free offline processes with `OPENAI_API_KEY` removed,
`CORTEXA_OPENAI_DEMO=0`, `CARGO_NET_OFFLINE=true` and
`npm_config_offline=true`. Reuse only isolated copy-on-write caches.

1. Run the focused client/component frontend tests and focused Rust Personal
   Assistant tests.
2. Run complete offline `npm run verify`.
3. Run `npm run docs:check`, `npm run repository:check`,
   `npm run security:scan` and `git diff --check`.
4. Verify the exact 11-path delta, 38-path cumulative inventory, production
   byte identity, predecessor reports/raw states and worktree registry.
5. Run session-end, architecture, security, code-health, technical-debt,
   readiness, quality and ordinary post-increment completion gates.

Native GUI and provider requests are not validation for this correction. All
live attempts remain exhausted. Existing native-success, GUI, D-128 custody and
abort, and D-127 dependency advisories remain truthful.

## Stop conditions and rollback

Stop on admission rejection, drift, failed validation, downloads, sensitive
output, mutation of a predecessor, production-byte change or scope beyond the
11 paths. Preserve edits without automatic rollback. A required failure receives
a truthful terminal failed record, not a passing marker. Do not launch the app,
inspect credentials, make provider requests, commit or publish. D-125/M1/M2 stay
parked.

## Progress

Predecessor preservation, exact candidate transfer and ordinary admission
passed. The bounded test correction is applied. Focused frontend (18), focused
Rust (43), complete offline verification, final documentation/repository/security/
whitespace, exact scope/preservation and session checks passed. Ordinary finalize
then failed because the report used `## Scope and preservation` instead of the
exact required `## Scope and boundaries` heading. Per the stop condition, the
increment was not repaired or retried and received a terminal failed record. No
app launch, credential inspection, live request, commit or publication occurred.
All live attempts remain exhausted and existing advisories remain.
