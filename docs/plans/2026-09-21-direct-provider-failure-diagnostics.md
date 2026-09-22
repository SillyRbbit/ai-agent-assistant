# Direct provider failure diagnostics

Date: 2026-09-21. Increment: `direct-provider-failure-diagnostics`.
Status: FAIL; Blocked; stopped at the first required validation failure.

## Goal and evidence

Separate transport, HTTP response and accepted stream failure categories using
closed, payload-free codes. The previous native generic error cannot establish
which stage failed. The successful Python/OpenSSL probe is limited to that
client, address and moment. No live success is inferred.

## Baseline, admission and preservation

The existing detached successor begins at `0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`.
All 30 uncommitted candidate paths were copied and byte-verified from
`/private/tmp/cortexa-personal-assistant-direct-implementation` before ordinary
begin. The external preservation manifest covers ten valid repositories and
separately records two already-prunable entries without inspecting their contents.
Its SHA-256 is `a44b547d4a314300a8d9d6cd761fbbe838ef0a577f41fa9ed3cc31ddc2390d4d`.
Revalidate every predecessor and its exact completion-state file at closeout.
The inherited finalized report and plan stay byte-identical. Do not copy gate
state, prune entries, commit, publish or modify any predecessor.

## Exact successor delta

Exactly these 15 paths are permitted relative to the transferred candidate:

- `src-tauri/src/personal_assistant_direct.rs`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-21-direct-provider-failure-diagnostics.md`
- `docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md`

The cumulative Git inventory is the inherited 30 paths plus the new plan/review:
32 paths. Enforce the 15-path delta and 32-path cumulative inventory separately.
The gate report must inventory the complete cumulative Git change set.

## Implementation and invariants

Keep `network` for non-timeout send/read failures; add `http_status` for HTTP
statuses outside 200 and existing 400/401/403/404/429 mappings. Add
`provider_stream` for accepted `response.failed`/`error` events only after all
existing decoder prerequisites. Use static messages and unit enum variants.
Keep timeout, protocol, refusal, incomplete and limit errors unchanged.
No new IPC fields/commands, logger, raw error strings, body/header/identifier
retention, alternate endpoint, request changes or new abstractions.
No dependency, lockfile, policy, permission, native host, UI layout, workflow,
hook, harness or D-125/M1/M2 changes. No credential inspection, launches,
requests, downloads, automatic retries or use of the final rehearsal attempt.

## Verification

Use existing local dependency/build caches copied without writing to the original.
Run all checks with OPENAI_API_KEY removed, CORTEXA_OPENAI_DEMO=0,
CARGO_NET_OFFLINE=true and npm_config_offline=true. Compilation/test outputs
are authorized; app launches and network probes are not.

- Focused Rust Personal Assistant unit tests, including status mapping, serialized
  closed codes, sanitized SSE failures, malformed/order rejection and existing
  timeout/cancellation/ownership coverage. No test opens a provider socket.
- Focused client and component tests: strict schema, no private-value echo,
  distinct static labels, partial-output labeling, busy cleanup, released controls,
  no automatic retry or polling after terminal ownership release.
- Full `npm run verify` after source/test formatting, plus `npm run docs:check`,
  `npm run repository:check`, `npm run security:scan`, and `git diff --check`.
- Exact delta/cumulative scope, unchanged histories, original checkouts, frozen
  evidence and protected dependency/transport/host bytes.
- Session-end inventory, composed architecture/security/code-health/debt/readiness
  review, ordinary post-increment report/finalize/status/Stop.

No installation, dependency resolution or fresh network audits are required for
this unchanged dependency graph. Native verification remains advisory and pending;
fixtures cannot prove actual provider classification or successful streaming.

## Risks and stopping conditions

Codes identify stages, not DNS/TLS root causes or provider error reasons. A future
native rehearsal needs an updated approved bundle; the old bundle is unchanged.
Stop on admission failure, baseline/scope/evidence drift, failed required checks,
missing offline dependencies, sensitive output or any need to expand scope.
Do not repair after a failed required check in this run. Preserve edits and record
a truthful terminal failed disposition where applicable, without automatic rollback.

## Progress and results

Preservation and candidate transfer passed. Ordinary admission passed. Six
baseline frontend tests passed. Diagnostic implementation is preserved but not accepted. The focused frontend
run returned 9 passed and 3 failed: all new UI cases expected an alert while the
DOM still showed streaming after one 500ms timer advance. No repair or retry was
performed. Timer/React effect scheduling is a hypothesis only. Rust and full
verification were not run after the owner-required stop. Failure closeout records
preservation, scope and documentation checks without claiming implementation
acceptance. The review records actual results and terminal disposition.
