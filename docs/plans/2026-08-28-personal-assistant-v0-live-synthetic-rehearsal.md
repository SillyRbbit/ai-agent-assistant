# V0-13 — live synthetic-text Stage C rehearsal

## Direct implementation successor — 2026-09-21

This entry supersedes the historical gateway-only execution instructions below
for the owner's explicitly approved private synthetic development task.
Baseline/scope evidence remains immutable in
`/private/tmp/cortexa-personal-assistant-direct-scope`; its fixture-only staging
recommendation is superseded by the owner's instruction to implement both
checkpoints now. See [D-128](../../DECISIONS.md) for the narrow credential,
topology and cancellation contract. Ordinary begin admitted
`personal-assistant-direct-implementation` from clean remote main
`0ed15810e90b6a4bd312a8096c61b0abb1ab7eff` in a new detached worktree.

### Goal, non-goals and invariants

Connect existing Conversations to a fixed native OpenAI Responses request,
using existing Personal Assistant ownership, identities, runtime validation,
snapshots, journal, cancellation and terminal semantics. No provider traffic on
startup, mode selection, settings or polling. No arbitrary prompts, content,
URLs, credentials or provider events from IPC. No gateway/proxy/server,
execution, tools, persistence, voice, RAG, graph redesign, other-agent activation,
permission changes, hook/harness changes, publication or D-125/M1/M2 work.

Only explicit Start after disclosure can read the native session environment.
Debug build plus `CORTEXA_OPENAI_DEMO=1` is required. Errors are closed and
provider text is escaped. Browser-only mode has no native Start; mock UI is
separate and its editable composer is hidden in native mode. Stop closes ingress,
aborts and awaits the owned request task; new runs cannot inherit old ownership.
Task abort does not prove DNS/remote-computation/billing cessation or immediate
library-buffer erasure. Key bytes may remain in process memory/environment.

### Checkpoints and dependency decision

A: fixed Responses adapter and crate-private direct profile/ingress now reuse
`PersonalAssistantV0Host`/`NativeAgentRuntime`. Focused Rust tests passed.
B: application-owned async task, three versioned/closed control commands,
strict TypeScript snapshot client and Conversations fixed-sample panel are
implemented; focused frontend and exact-boundary checker tests passed.

Pin reqwest 0.13.5, rustls TLS with verification, and tokio 1.52.3 runtime/time
features. Lock resolution changes no unrelated existing version; Rust/Cargo 1.90
is the tested toolchain (repository minimum 1.88, reqwest minimum 1.85).
No fallback, redirect, proxy or retry. One POST:
`https://api.openai.com/v1/responses`, `gpt-5.6-luna`, foreground stream,
`store=false`, `background=false`, `tools=[]`, `tool_choice=none`,
512 output tokens, reasoning effort none, plain text, truncation disabled.
Only application instructions and this exact existing sample are transmitted:

> Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed.

No provider conversation, previous response ID, personal history or attachments.
Contract sources checked for this path: [model](https://developers.openai.com/api/docs/models/gpt-5.6-luna),
[request](https://developers.openai.com/api/reference/resources/responses/methods/create),
[stream events](https://developers.openai.com/api/reference/resources/responses/streaming-events),
[data controls](https://developers.openai.com/api/docs/guides/your-data), and
[reqwest client](https://docs.rs/reqwest/0.13.5/reqwest/struct.ClientBuilder.html).
Model access and billing remain unverified; closed errors never trigger fallback.
Limits: connect 10s/read-idle 20s/total 60s, 64KiB chunk/frame, 1MiB wire input,
512 wire events, and existing host 128 updates / 8192 Unicode scalars / 32768 bytes.
Reject non-text output, unknown/tool events, malformed IDs/sequences, missing
completion and mismatched final text. Library buffers can allocate before
application checks. `store=false` is not Zero Data Retention.

### Exact authorized implementation inventory

- Cargo.toml/lock; runtime.rs, native_runtime.rs, gateway_request.rs;
  personal_assistant_v0.rs, lib.rs; new personal_assistant_direct.rs and
  personal_assistant_direct_tauri.rs, all under src-tauri.
- Conversations workspace; new PersonalAssistantDirectDemo component/test;
  new personal-assistant-direct-client and test; App.test.tsx.
- scripts/repository_health.py and its existing test module: extend only the
  exact three IPC declarations and negative fixtures.
- ARCHITECTURE, CHANGELOG, DECISIONS, HANDOFF, NEXT_STEPS, PLANS, PROJECT_STATUS,
  ROADMAP, SECURITY, TESTING_GUIDE, TROUBLESHOOTING_LOG; this existing plan;
  one new implementation post-increment review. Exact paths are in the
  [review](../reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md).

Preserve every historical section and finalized/failed report. Stop on an
unexpected changed path, unpreservable overlapping work, new audit findings,
material scope expansion or failure after at most two evidence-based repairs
per distinct recoverable failure. No branch/commit/push. Rollback is an owner
decision; leave isolated changes intact rather than resetting original work.

### Validation and limits of evidence

Required: focused Rust host/adapter tests, frontend/client/UI tests, checker
regressions; clean npm install; exact dependency/lock inspection; full/production
npm audit; pinned cargo-audit 0.22.2 and unchanged exact Cargo gate;
`npm run verify`; documentation/repository/secret/whitespace checks; preservation
inventory; session-end, composed quality review and ordinary post-increment
finalize/status/Stop. Do not substitute old baseline evidence for new checks.
The [review](../reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md)
records actual outcomes.

Native window/IPC/interaction, Research/Knowledge native GUI, credential ingestion
and paid request completion remain pending. The owner explicitly allows native
visual verification to remain advisory. No fixture test is live proof.

### Owner-only native setup for a separately approved rehearsal

No key is required for compilation or local tests. Node/npm satisfying
package.json, Rust/Cargo 1.90 as tested, Xcode Command Line Tools and an available
localhost port 1420 are required. Release builds intentionally reject live Start.

In a key-free terminal, use the isolated candidate:

```bash
cd /private/tmp/cortexa-personal-assistant-direct-implementation
cargo build --manifest-path src-tauri/Cargo.toml --locked
npm run dev
```

In a separate owner-controlled zsh terminal, after authorizing one paid sample,
read the key privately without writing its literal value to shell history:

```zsh
cd /private/tmp/cortexa-personal-assistant-direct-implementation
read -r -s 'demo_key?OpenAI API key (private input): '
printf '\n'
OPENAI_API_KEY="$demo_key" CORTEXA_OPENAI_DEMO=1 ./src-tauri/target/debug/ai-agent-assistant
unset demo_key
```

The native debug executable uses the existing Vite development URL. Do not use
a VITE variable, repository .env, chat, source, logs, artifacts or a frontend
field. Do not launch npm/tooling with the key. The process environment and local
shell variable remain accessible under local inspection; this is not secure
credential storage. Quit the native process and unset the temporary variable
afterward, including after interruption.

Owner must have API billing/model access, accept external synthetic transmission
and possible charges, select native mode, read the displayed sample/disclosure,
check acknowledgment and press Start once. No automatic fallback/retry. Record
only safe statuses, not credentials or provider bodies. Stop remains local
cancellation, not a guarantee about remote cost. Do not begin this rehearsal
automatically during implementation.

Status: Blocked; no traffic authority or complete prerequisite chain exists
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-9 through V0-12, fresh V0-8 credential window, and exact Stage C approval

## Goal and outcome

On the signed target Mac, perform the first and only live synthetic-text proof:
an explicit acknowledged Start sends the fixed application-owned fixture to
OpenAI through the Access-protected Cloudflare Worker, streams bounded text,
and proves closed terminal behavior and full rollback. This is not a usable
personal assistant and authorizes no real/personal content.

## Exact repository files

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
- `docs/security/phase4-gateway-threat-model.md`
- `docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md`
- `docs/increments/personal-assistant-v0-live-synthetic-rehearsal.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal-post-increment-review.md`
  (new)

No production source/test, dependency/lockfile, Tauri/frontend, capability/CSP,
permission, gateway code/config shape, workflow, signing state, or new cloud
resource may change. Private credentials, identifiers, URLs, raw content/logs,
screenshots containing private state, and evidence locations stay outside Git.

## Preflight gates

- Reconfirm exact commit/toolchains, signed app identity, V0-8 token/policy and
  Keychain ACL/lifetime, V0-12 provider project/key/model/limits, deployed
  source hash, route/origin/TLS/JWT bindings,
  `PA_V0_TRAFFIC_ENABLED=false`, and
  rollback owners.
- Reconfirm current OpenAI synthetic retention/data-use evidence and actual
  Cloudflare Access/Worker/control-plane log retention. Confirm no custom
  content log/observability/storage exists. Access/Worker runtime metadata must
  be content-free and at most seven days; the separately classified mandatory
  admin-action audit trail must contain no app content/secret and match D-094's
  current synthetic-only 18-month acceptance.
- Display the exact V0-11 disclosure and fixed fixture. The checkbox begins
  unchecked. Rust must reject Start without the exact disclosure version and
  may write no socket before consuming its private admission.
- Prohibit personal/sensitive data. No composer or alternate input is present.
  One explicit Start equals one model request; there is no automatic preflight,
  retry, fallback, continuation, or second cancel request.

## Rehearsal matrix

Record each as Passed, Failed, or Not run with sanitized evidence:

1. `PA_V0_TRAFFIC_ENABLED=false` and disabled-route denial before activation;
   map only the fixed route, re-prove false-flag denial, then set the flag to
   exact `true` for the bounded rehearsal window.
2. Explicit disclosure acknowledgment followed by one fixed-fixture success;
   verify ordered stream, exact final equality, 128-event/output caps, no tool
   event, and one terminal state.
3. Closed failure through a safe pre-provider control
   (`PA_V0_TRAFFIC_ENABLED=false` before a fresh start) and, only where
   practical without changing scope, one upstream closed error. The false flag
   proves new-admission denial only.
4. Explicit cancellation during connect and streaming, each in a fresh run;
   UI closes ingress, Rust aborts the original request/socket, Worker aborts
   upstream on disconnect, no `waitUntil` continues work, cleanup terminalizes,
   and all late frames are rejected.
5. Connect/idle/provider/total deadline paths where practical, network loss,
   offline start, route disable, Access revocation, provider-key revocation,
   rate/budget denial, busy denial, and restart only after cleanup.
6. Route-away/back and minimize/hide/restore snapshot recovery,
   keyboard/focus, narrow/native resize, 200% zoom, light/dark, reduced motion,
   long output scroll, accessibility semantics, native/WebView console
   redaction, no permission prompt, no file, durable state, tool, device effect,
   or residual active request.
7. Vendor consoles/log surfaces show only accepted operational metadata; no
   fixture, instruction, output, token, JWT, claim, header, provider body/ID,
   raw error, or stack is present.

Unavailable controls are `Not run`, never inferred. A required row that cannot
be observed blocks completion rather than being replaced with fixture evidence.

## Verification

```bash
npm ci
npm audit --audit-level=low
npm run verify
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Use only repository-pinned Node/npm/Rust on the target Mac. Start the native app
through the prescribed Tauri command, capture sanitized results, and stop every
development process after the matrix.

## Rollback and stop conditions

Rollback first sets `PA_V0_TRAFFIC_ENABLED=false` and disables/removes the route
to deny new admission immediately. It then closes local ingress and triggers
the owned cancellation path for any active run; Access-token and OpenAI-key
revocation follow immediately after abort initiation without waiting for
teardown. The owner waits only for the bounded active cleanup, retaining cleanup
ownership on ambiguity, then deletes Keychain items, proves denial/missing
state, removes policy/secret/bindings, and deletes newly approved resources
where required. The traffic flag, route removal, and credential revocation are
never claimed to abort an already admitted request; the owned controller is the
active-request kill path.

Stop immediately on any real/personal input, unacknowledged transmission,
caller-selected configuration, provider/tool event, retry/fallback, content
log/reflection, unknown terminal state, abort/cleanup ambiguity, late accepted
event, unavailable new-admission flag, failed revocation/removal, permission prompt,
device effect, runtime metadata above seven days, content/secret in the
control-plane audit class, or source/config change need. Do not remediate within
rehearsal.

## Readiness

**Blocked.** This is the only Stage C live synthetic increment. It requires a
fresh explicit traffic approval after every prerequisite closes. Success does
not authorize milestone 2 or real prompts.
