# Direct provider stream diagnostics

Date: 2026-09-21. Increment: `direct-provider-stream-diagnostics`.
Status: Required frontend validation failed; terminal closeout required.

## Goal, authority and baseline

The owner explicitly authorized this isolated successor from the verified
`direct-provider-diagnostics-test-sequencing` candidate. Detached baseline is
`0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`; the exact 34-path candidate was
transferred and byte-verified before ordinary begin succeeded. No predecessor
raw gate state was copied. Eleven predecessor checkouts, finalized reports and
raw gate files remain preserved; two already-prunable entries remain untouched.

All three rehearsal attempts are exhausted. The last native observation was
starting then provider_stream error, with no answer/completion and released
ownership. It cannot be retrospectively classified further. Native live success
and remaining GUI checks remain advisories; D-125/M1/M2 remain parked.

## Exact scope

The successor delta is exactly these 15 paths, separately enforced from the
inherited inventory. The cumulative Git inventory is 36 paths (34 plus this
plan and review). All predecessor plans/reviews remain byte-identical.

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
- `docs/plans/2026-09-21-direct-provider-stream-diagnostics.md`
- `docs/reviews/2026-09-21-direct-provider-stream-diagnostics-post-increment-review.md`

## Frozen mapping and trust boundary

Only response.failed response.error.code is classified: server_error becomes
provider_stream_server_error; rate_limit_exceeded becomes
provider_stream_rate_limit; invalid_prompt becomes provider_stream_invalid_prompt.
The three fixed messages respectively say the provider reported a server error,
a rate limit, or an invalid prompt in the response stream, followed by
“No automatic retry was made.” No provider string is forwarded.

Every top-level error event and every unknown, absent, null, non-string,
oversized, differently cased or misplaced code retains provider_stream and its
existing message. Classification follows existing frame/sequence/creation checks.
Do not change validation precedence, request construction, TLS, HTTP mappings,
limits, cancellation, ownership, retries, dependencies, permissions or IPC fields.
No raw message, parameter, body, header or identifier is retained in diagnostics.
No production UI component or native host change is needed.

Authoritative public evidence: [streaming events](https://developers.openai.com/api/reference/resources/responses/streaming-events)
places response.failed details under response.error, while top-level error.code
is an open nullable string. The [Response schema](https://developers.openai.com/api/reference/resources/responses.md)
enumerates the three selected codes. Their labels do not prove account, billing,
retryability, or the historical cause. Other enumerated codes are outside scope.

## Offline regression matrix

- Rust: three positive nested mappings; generic top-level errors; wrong nesting,
  unknown/missing/null/non-string/oversized/case/whitespace code fallbacks.
- Preserve malformed frames, missing creation, sequence/event mismatch, limits,
  refusal, incomplete and explicit completion checks and existing error mappings.
- Synthetic sentinel values in code/message/parameter/identifier fields must not
  appear in serialized enum or Debug/Display diagnostics, client messages or UI.
- Client: closed codes, exact static messages, reject raw objects/strings and
  additional fields. UI: partial output, busy cleanup, release and one Start/no
  retry for all new codes, retaining separate 250ms fake-timer acts.

## Validation and completion

Remove OPENAI_API_KEY from test processes, set CORTEXA_OPENAI_DEMO=0,
CARGO_NET_OFFLINE=true and npm_config_offline=true. Reuse isolated copy-on-write
local caches only. Run focused frontend/Rust tests, full offline npm run verify,
then docs:check, repository:check, security:scan, git diff --check, preservation
and exact scope checks, session-end, composed quality review and ordinary
post-increment finalization/status/Stop. Never infer live evidence from tests.
No fresh network audits for unchanged dependencies; inherited D-127 advisory stays.

## Stop conditions and rollback

Stop on admission failure, drift, failed required validation, downloads,
sensitive output or scope expansion. Preserve edits without automatic rollback.
A required failure uses a truthful terminal failed report, not a passing marker.
No app launch, credential inspection, provider request, commit or publication.
Do not reopen or mutate predecessors. No new live allowance is granted.

## Progress

Preservation and candidate transfer passed; ordinary admission passed.

Focused frontend validation failed: 15 passed, 3 new UI cases failed at the
inherited no-mock-message assertion. The exact approved messages instead end
with the no-automatic-retry sentence. No executable edit followed the failure.
The concurrently started Rust check completed with 43 passing tests. Full offline
verification was not run. Required failure disposition is FAIL / Blocked, with
no completion marker. Only failure-closeout documentation and preservation checks
continue; no repair or automatic successor is authorized.
