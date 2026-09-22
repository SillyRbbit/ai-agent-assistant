# Direct provider stream stage diagnostics

Status: FAIL / Blocked; terminal closeout; additional request unused
Owner: Project owner
Last updated: 2026-09-21

## Goal

Distinguish the observed generic stream-failure origin using the existing closed
errors, then permit exactly one separately approved native fixed-sample request
only after passing local validation and owner-private launch/acknowledgment.

## User-visible outcome

A static message identifies top-level error, unrecognized failed-response code,
or unusable failed-response code. Existing response.failed server_error,
rate_limit_exceeded and invalid_prompt classifications remain unchanged.

## Scope

Exactly 15 paths relative to `3f99165b4dcb0ef18c52f9242b346eb1d711afaa` in a new detached worktree.
All predecessor checkouts, terminal reports, markers and raw states are immutable.
The original dirty checkout and two prunable registry entries remain untouched.

## Explicit non-goals

No request/transport repair without evidence; no dependencies, arbitrary prompt,
model change, router, subscription, gateway, general diagnostics API, logging,
request-ID retention, new IPC command, permissions, governance, graph, native-host
or component production changes. No commit, push or publication. D-125/M1/M2 stay
parked. Do not reopen any finalized predecessor.

## Existing behavior and constraints

HTTP must be 200 and content-type text/event-stream before decoding. Existing
UTF-8/JSON, event-name/type, sequence and prior response.created checks run before
failure classification. EOF remains incomplete; invalid data remains protocol;
transport and HTTP failures retain their existing categories. Preserve refusal,
tool rejection, identity checks, limits, cancellation and explicit completion.

## Current-state evidence

The predecessor is clean at `3f99165b4dcb0ef18c52f9242b346eb1d711afaa` and its gate reports complete/valid with
PASS WITH ADVISORIES. Prior native UI showed no text, an error and released
controls. This does not prove the absence of wire deltas: decoder batches a chunk
and a later error can discard earlier events from that chunk. No output-delivery
change is authorized or justified by that observation. The earlier Python TLS
probe is not Rust-client evidence. All earlier live allowances are exhausted;
the owner added exactly one separate request for this task, still unused.

Official [model documentation](https://developers.openai.com/api/docs/models/gpt-5.6-luna)
supports Responses, streaming and reasoning effort none. Official
[streaming schemas](https://developers.openai.com/api/reference/resources/responses/streaming-events)
distinguish response.failed nested ResponseError from the open-ended top-level
error code. Neither source identifies the historical failure's cause.

## Files expected to change

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-21-direct-provider-stream-stage-diagnostics.md`
- `docs/reviews/2026-09-21-direct-provider-stream-stage-diagnostics-post-increment-review.md`
- `src-tauri/src/personal_assistant_direct.rs`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`

## Affected components

Rust adapter inline error enum/branches/tests, TypeScript static error map and
existing client/UI regression matrices. Existing host, IPC, panel and runtime
implementation remain unchanged.

## Interfaces and invariants

- `provider_stream_error_event`: any top-level error reaching existing validation.
- `provider_stream_failed_unknown_code`: nonempty nested string outside the three
  exact accepted codes, including whitespace/case variants.
- `provider_stream_failed_invalid_code`: absent, null, non-string or empty nested
  code. A malformed code is not described as malformed JSON.
- Legacy provider_stream remains recognized at the client boundary. Every new
  message is static and ends with “No automatic retry was made.”
- No value from raw provider fields is returned, copied into error state or logged.
- Existing request bytes, model, HTTPS endpoint, sample, 512 output tokens, empty
  tools, transport configuration, error validation and ownership stay unchanged.

## Implementation milestones

- [x] Freeze preservation manifest; verify clean baseline and 18 frontend tests.
- [x] Ordinary admission without predecessor gate-state transfer.
- [x] Add bounded classification and offline regression matrix.
- [x] Pass focused and full offline verification, rebuild debug bundle.
- [ ] Pass reviews, documentation/preservation/session/schema/completion gates.
- [ ] Attempt one owner-acknowledged native sample, report outcome and stop process.

## Security and privacy considerations

Use existing enum-only diagnostics rather than payload retention. Build/test
processes explicitly unset OPENAI_API_KEY and set CORTEXA_OPENAI_DEMO=0. No
credentials, Terminal contents, arguments, environment or credential stores are
inspected. Local cloned node_modules and Cargo outputs remain ignored. Owner
privately launches only the verified new bundle. No separate API probe is allowed.

## Test plan

Rust: all existing success/refusal/tool/limit/identity/sequence/EOF/status mappings;
three new branches and exact serde, Debug and Display redaction; missing/null/
wrong-type/empty/unknown/case/whitespace/oversized codes; misplaced code fields;
top-level errors never inherit ResponseError semantics; chunk boundary variation.
Frontend: exact static messages; reject object/suffixed/unknown error values and
extra payload fields. Preserve separate 250ms acts, visible streaming, terminal
cleanup, partial-output label, ownership release, one Start and no polling/retry.

## Verification commands

All application commands use `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0
CARGO_NET_OFFLINE=true npm_config_offline=true`.

```bash
npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked --offline personal_assistant_direct
npm run verify
npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/preserve.py --final
python3 -B .codex/hooks/session_end_gate.py
```

For this failed increment, read-only failed-report schema validation, ordinary
close-failed, failed/FAIL/valid status and full-payload Stop follow the existing
hook API. Do not call finalize or edit the report after terminal closure. No
native request follows this failed disposition; preserve the unused allowance.

## Risks

New diagnostics distinguish origin and code shape, not every underlying provider
cause. A failed live attempt ends the rehearsal with no retry or automatic
successor. Native live success, remaining GUI smoke, D-128 limitations and D-127
advisories remain unless directly verified. Local checks do not prove live success.

## Rollback or failure strategy

Preserve the isolated candidate and all predecessor evidence. Stop on admission
failure, scope or baseline drift, sensitive output or required validation failure.
At most two evidence-supported local test repairs are authorized before truthful
terminal closeout. No automatic rollback, gate change or predecessor reclosure.

## Decisions made

No new durable policy. Diagnostic-only closed enum expansion is within the owner's
explicit authorization; D-128 behavior stays unchanged. Copy-on-write reuse of
existing local dependencies/build outputs avoids resolution and downloads.

## Discoveries

Source proves the generic category can originate from two event branches; it
cannot reconstruct the historical provider code. No transport/parser defect has
been established. One additional native request is separately approved, not a
loop until success.

## Progress

2026-09-21: preserved 16 valid checkouts and two prunable entries, cloned local
ignored tooling, passed baseline (18 frontend tests) and ordinary admission.
Implemented three static diagnostic branches, passed focused and full verification,
completed independent review and rebuilt the debug bundle. The historical failure
cause remains unknown; the separately approved live retry remains unused.

## Acceptance criteria

- [x] Exact 15-path scope and all historical/predecessor bytes preserved.
- [x] New static diagnostics pass offline application regression and verification.
- [x] Rebuilt bundle identity verified; failed completion evidence prevents launch.
- [ ] Valid local completion evidence; no live-success claim without observation.

## Final results

Application diagnostics tests and full offline verification passed; see the review
for exact counts and commands. Debug bundle identity, native enum codes and
frontend messages passed after one external assertion correction. The report
wrapper import path required external correction 2. Its next run failed on an
invalid Readiness finding category in the passing-completion draft. No application
test failed. The two-correction allowance was conservatively treated as exhausted.

The result is FAIL / Blocked. This unfrozen report was converted only to a truthful
terminal failure record with the existing valid schema; no third passing repair,
passing finalization or predecessor reclosure was attempted. Final failure docs
must pass documentation, preservation and read-only failed-report validation before
ordinary close-failed, valid failed status and Stop are observed. No completion
marker or live success is claimed. No app was launched, no test-owned process
exists, and the separate additional request remains unused.

## Documentation updates

Nine additive current-state documents plus this plan/report; all historical text
is retained. No policy/decision change is needed.
