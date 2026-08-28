# V0-10 — synthetic OpenAI adapter with fake upstream

Status: Blocked by V0-9 and current provider-profile evidence
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-9 and accepted current synthetic provider profile

## Goal and outcome

Extend the verified auth-only Worker with one fixed OpenAI Responses adapter
that accepts only the exact V0-1 application-owned synthetic request and is
tested only against a deterministic fake upstream. It adds no provider secret,
resource, request, route activation, user input, or Tauri surface.

## Exact files

- `gateway/cloudflare-worker/src/personal-assistant-v0-worker.mjs`
- `gateway/cloudflare-worker/test/personal-assistant-v0-worker.test.mjs`
- `gateway/cloudflare-worker/wrangler.jsonc` (binding names,
  `PA_V0_TRAFFIC_ENABLED=false`, pinned compatibility date,
  `enable_request_signal`, and explicit `no_request_signal_passthrough` only;
  no secret or identifier)
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-synthetic-gateway.md`
- `docs/increments/personal-assistant-v0-synthetic-gateway.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-synthetic-gateway-post-increment-review.md`
  (new)

No package/lockfile, workflow, Rust, frontend, Tauri, capability, CSP, route,
account, secret, or additional gateway file may change.

## Exact server-owned mapping

Access/JWT admission remains mandatory before the bounded body is retained or
parsed. The Worker accepts one `POST` and exact content type/version. It
validates the complete V0-1 profile, including exact instruction and synthetic
fixture, `openai` / `gpt-5.6-luna`, `application-owned-synthetic@1`,
`empty@1`, empty tools, one turn/request, zero retry/fallback, and every bound.
Run/request IDs are validated opaque correlations, not configuration.

After exact validation, the Worker constructs this provider request from
server-owned constants; it never forwards the desktop envelope or concatenates
instructions into user input:

```json
{
  "model": "gpt-5.6-luna",
  "instructions": "Act as Cortexa's Personal Assistant for one bounded synthetic text request. Answer only from the supplied application-owned synthetic text and return concise plain text. Do not call or propose tools, access files, memory, retrieval, networks, or devices, delegate, schedule, persist, approve, execute, retry, or claim any action or context not supplied.",
  "input": "Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed.",
  "tools": [],
  "tool_choice": "none",
  "parallel_tool_calls": false,
  "reasoning": { "effort": "low" },
  "text": { "format": { "type": "text" }, "verbosity": "low" },
  "truncation": "disabled",
  "max_output_tokens": 2048,
  "top_logprobs": 0,
  "stream": true,
  "stream_options": { "include_obfuscation": false },
  "store": false,
  "background": false
}
```

No previous-response ID, conversation, attachment, hosted tool, retrieval,
reasoning-encrypted-content inclusion, metadata, caller parameter, or alternate
model/provider is allowed. Every optional field not shown is absent rather than
defaulted by caller input. If the exact official API/model contract changes,
stop and revise the decision; do not silently translate.

The sole provider egress is exactly `POST
https://api.openai.com/v1/responses`, with no query or fragment. The outbound
request uses `redirect: "error"`, `cache: "no-store"`, and only these headers:
`Authorization: Bearer <Worker-owned OpenAI secret>`,
`Content-Type: application/json`, and `Accept: text/event-stream`. No inbound,
Access, Cloudflare, forwarding, cookie, referrer, tracing, or caller header is
copied. Any redirect, alternate scheme/host/port/path, DNS/TLS failure, or
unexpected response content type fails closed without forwarding the provider
secret. Tests assert the exact URL, method, redirect/cache mode, header
allowlist, and absence of secret/header forwarding against fake fetch only.

Provider events are untrusted. The adapter parses the SSE `event` name and
`data` JSON independently, requires their type literals to agree, and accepts
at most 128 events/128 KiB total before terminal state. Every JSON event begins
as unknown, must have exactly the event-specific own keys below, and carries a
nonnegative safe-integer `sequence_number`. The adapter records the first
event's number as a private sequence origin, requires it to leave room for all
128 events, and requires every later event to be exactly prior + 1; it does not
assume an undocumented zero base. One provider response ID and one message item
ID are nonempty visible ASCII of at most 128 bytes, remain private, and must be
identical everywhere their respective ID appears. `output_index` and
`content_index` are always 0. `stream_options.include_obfuscation=false` makes
`obfuscation` absent; its presence is a protocol violation.

The sole success grammar is, in this exact order:

1. `response.created`
2. `response.in_progress`
3. `response.output_item.added`
4. `response.content_part.added`
5. one or more `response.output_text.delta`
6. `response.output_text.done`
7. `response.content_part.done`
8. `response.output_item.done`
9. `response.completed`

The exact event envelopes are:

```text
ResponseEventV1 = {
  type: "response.created" | "response.in_progress" |
        "response.completed" | "response.failed" |
        "response.incomplete",
  response: ProviderResponseV1,
  sequence_number: number
}
OutputItemEventV1 = {
  type: "response.output_item.added" | "response.output_item.done",
  output_index: 0,
  item: ProviderMessageItemV1,
  sequence_number: number
}
ContentPartEventV1 = {
  type: "response.content_part.added" | "response.content_part.done",
  item_id: string,
  output_index: 0,
  content_index: 0,
  part: ProviderOutputTextV1,
  sequence_number: number
}
TextDeltaEventV1 = {
  type: "response.output_text.delta",
  item_id: string,
  output_index: 0,
  content_index: 0,
  delta: string,
  logprobs: [],
  sequence_number: number
}
TextDoneEventV1 = {
  type: "response.output_text.done",
  item_id: string,
  output_index: 0,
  content_index: 0,
  text: string,
  logprobs: [],
  sequence_number: number
}
ErrorEventV1 = {
  type: "error",
  code: string | null,
  message: string,
  param: string | null,
  sequence_number: number
}
ProviderMessageItemV1 = {
  id: string,
  type: "message",
  status: "in_progress" | "completed",
  role: "assistant",
  content: ProviderOutputTextV1[]
}
ProviderOutputTextV1 = {
  type: "output_text",
  text: string,
  annotations: [],
  logprobs: []
}
```

Where the current API omits `logprobs` from an empty text part/event despite
the current formal type, the parser may accept exactly one documented
compatibility shape with that single key absent; it normalizes immediately to
an empty array. No other optional/unknown event-envelope key is accepted. The
implementation increment must pin both shapes as named fixtures and recheck
the official schema; any additional compatibility need is a stop and plan
revision, not permissive parsing.

`ProviderResponseV1` is a closed internal projection constructed only after the
raw response object passes the 64 KiB/64-own-key bound. It contains only
`id`, `object`, `status`, `model`, `instructions`, `max_output_tokens`,
`output`, `parallel_tool_calls`, `previous_response_id`, `reasoning`, `store`,
`text`, `tool_choice`, `tools`, `top_logprobs`, `truncation`, `background`,
`error`, and `incomplete_details`. Other provider response metadata is neither
retained nor allowed to influence state, output, errors, logs, or authority.
Those projected fields must echo the Worker-owned profile exactly:
`object="response"`, model/instructions/max-output values equal the request,
empty tools, `tool_choice="none"`, parallel/store/background false,
previous-response null, low reasoning, plain-text/low-verbosity text, zero top
logprobs, and disabled truncation. The created/in-progress output is empty; the
completed output is exactly one completed assistant message whose single
output-text part has no annotation/logprob and equals all accepted deltas. A
different returned model or configuration is `protocol_violation`, satisfying
exact returned-runtime identity validation rather than silently accepting an
alias or snapshot.

Only `response.in_progress` emits the normalized V0-1 `Started`; each nonempty
text delta emits one normalized bounded `TextDelta`; intermediary item/part and
done events validate order/content but emit nothing; `response.completed`
emits `Completed` only after exact final-text equality. Empty deltas, malformed
Unicode, text above per-event/aggregate bounds, annotations, logprobs,
refusals, reasoning items, tool/function/hosted-tool items, a second output,
or any other event type are protocol violations. A refusal-shaped
`response.content_part.added` fails while validating its `part`, before any
refusal text is retained. A `response.refusal.delta` or
`response.refusal.done` fails on its unaccepted type before event-specific
field parsing. No refusal envelope is part of the accepted grammar, and every
such path maps only to the closed `protocol_violation` failure.

`response.failed` is accepted only as the terminal event with status `failed`,
the same response ID/profile, empty output, and a bounded provider error; its
message/parameter are discarded and its code maps through an exact allowlist
to a closed V0-1 failure. `response.incomplete` is terminal and maps only
documented `max_output_tokens`/`max_tokens` reasons to `limit_exceeded`; any
other reason maps to `request_rejected`. An `error` event closes the stream and
maps its allowlisted code to a closed failure while discarding
message/parameter. An unsolicited cancellation or any unknown failure code
maps to `provider_unavailable`. No failure can later become success, and no raw
provider string or ID crosses the gateway.

The Worker enables incoming `Request.signal` and deliberately disables automatic
signal passthrough. It creates one owned `AbortController`, attaches the
incoming abort signal to it, and passes only that controller's signal to the
single provider fetch. Downstream disconnect, owned deadline, response limit,
or protocol failure triggers the controller exactly once and closes downstream
output before discarding late upstream frames. No `waitUntil`, detached work,
retry, fallback, or late-frame forward is permitted. Abort ambiguity stays
terminal failure and blocks a success claim. Changing
`PA_V0_TRAFFIC_ENABLED` to `false` or removing a route denies **new** admission;
it does not claim to abort an already admitted request. Active teardown requires
the owned controller through client cancellation/deadline/limit/protocol paths.

## Logging and operational boundary

Application/Worker content logging and observability are disabled. Code may
emit only an enumerated outcome, coarse timing bucket, bounded byte/event
counts, and opaque Worker correlation to an injected test sink; production has
no custom sink in this increment. Prompt, output, instruction, token, JWT,
claim, header, provider body/ID/error, and stack are prohibited. Cloudflare
Access/runtime metadata must be content-free and retained at most seven days.
The mandatory admin-action audit trail is the separate D-094 control-plane
class: its current 18-month synthetic-only acceptance still requires fresh
field/retention inspection before traffic and never becomes a “no logging”
claim or real-content authority.

## Tests

Tests inject generated Access/JWKS material and a fake OpenAI stream. Cover
exact request-to-provider mapping, unknown/alternate fields/configuration,
size/scalar/count/time boundaries, partial/multibyte provider frames,
malformed/unknown/duplicate/out-of-order events, every tool/function shape,
zero- and nonzero sequence origins with contiguous/gap/overflow cases, first
refusal-part rejection, refusal delta/done rejection, reasoning-item rejection,
closed provider failures, incoming-signal support, owned-controller
disconnect/cancel/deadline aborts, false traffic flag denying new admission but
not masquerading as active abort, late frames, zero retry/fallback, fixed
provider URL/method/redirect/cache/headers, redirect and alternate-egress
denial, fixed correlations, and canary absence from responses/errors/log
captures. No test contacts Cloudflare or OpenAI.

## Verification

```bash
npm run test:gateway-admission
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Target-Mac, Cloudflare, OpenAI, real credential, provider-secret, route, and UI
checks are `Not run`.

## Non-goals, rollback, and stop conditions

No real/personal content, external provider call, secret, route, storage,
durable audit, tool, file, memory, background response, retry/fallback, second
provider/model, desktop/Tauri/UI change, or production claim. Rollback reverts
only exact source/test/docs files; no external cleanup exists.

Stop on stale provider docs, need for an SDK/dependency/stateful resource,
caller-controlled mapping, missing `enable_request_signal`, automatic signal
passthrough, redirect following, alternate provider egress, forwarded inbound
headers, provider content in errors/logs, an accepted tool event, unbounded
stream, detached continuation, abort ambiguity, or any external action.

Before V0-10 may move from Blocked to Ready, fresh primary provider evidence
must confirm that `gpt-5.6-luna` with this exact low-effort, text-only, empty-
tool profile supports the accepted message/text event grammar without a
required reasoning output item. If official evidence does not establish that
trace, keep V0-10 Blocked and revise this plan under separate approval; do not
ignore or forward a reasoning item and do not use provider traffic as an
unapproved discovery probe.

## Readiness

**Blocked.** V0-9 and exact current model/API/retention evidence are absent.
Source-only success would not authorize a provider secret or request.
