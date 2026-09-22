# Direct provider failure diagnostics review

Date: 2026-09-21. Increment: `direct-provider-failure-diagnostics`.
Workspace: `/private/tmp/cortexa-direct-provider-failure-diagnostics`.
Detached HEAD: `0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "direct-provider-failure-diagnostics",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md",
    "docs/plans/2026-09-21-direct-provider-failure-diagnostics.md",
    "docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md",
    "docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/Cargo.lock",
    "src-tauri/Cargo.toml",
    "src-tauri/src/agent/gateway_request.rs",
    "src-tauri/src/agent/native_runtime.rs",
    "src-tauri/src/agent/runtime.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/personal_assistant_direct_tauri.rs",
    "src-tauri/src/personal_assistant_v0.rs",
    "src/App.test.tsx",
    "src/features/conversations/ConversationWorkspace.tsx",
    "src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "src/features/conversations/PersonalAssistantDirectDemo.tsx",
    "src/infrastructure/tauri/personal-assistant-direct-client.test.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.ts"
  ],
  "commands_executed": [
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "verification": [
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
      "required": true,
      "status": "Failed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1",
      "required": true,
      "status": "Not run"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
      "required": true,
      "status": "Not run"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Exact 15-path successor delta and 32-path cumulative inventory; nine predecessor repositories and state files unchanged; historical text and finalized report preserved",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Composed architecture, security, code-health, technical-debt and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Updated native bundle and live provider behavior",
      "required": false,
      "status": "Not run"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "High",
      "summary": "All three new terminal error UI regression cases failed; acceptance is blocked.",
      "risk": "The required error presentation and control-release regression evidence is incomplete. Rust and full verification were not run after the stop condition.",
      "effort": "Separately authorized read-only diagnosis and bounded correction planning.",
      "milestone": "Before diagnostic implementation acceptance or native rehearsal.",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Successful native provider response and remaining native GUI smoke remain unverified.",
      "risk": "Python/OpenSSL TLS success and offline fixtures cannot establish Rust/rustls, provider access or live completion. One approved rehearsal attempt remains.",
      "effort": "Separate owner-authorized native rehearsal only after prerequisites pass.",
      "milestone": "Before claiming live-demo verification.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

## Executive summary

FAIL. The diagnostic code and offline regressions are preserved, but acceptance
stopped at the first required validation failure: 9 frontend tests passed and
3 newly added UI cases failed. No repair or retry was performed. No native
launch, credential inspection, live request, install, download, commit or
publication occurred. One rehearsal attempt remains. This is truthful terminal
failure evidence, not a completion marker or authority for a successor.

## Scope and boundaries

The verified 30-path candidate was transferred byte-for-byte after revalidating
all ten repository snapshots and the registry. Its gate state was not copied.
Ordinary begin admitted this new successor before diagnostic edits. Local
node_modules and Cargo target outputs were copied using APFS copy-on-write for
isolated offline reuse; predecessor caches were not used as writable targets.

The exact delta from the transferred candidate is 15 paths, listed in the
[plan](../plans/2026-09-21-direct-provider-failure-diagnostics.md). The gate's
cumulative Git inventory is 32 paths including the inherited implementation.
The original candidate and every other existing checkout/state are unchanged;
two already-prunable entries remain recorded, not pruned or treated as valid
repositories. Their directory contents and other ignored files were not assessed.
All historical document suffixes and the finalized predecessor plan/report remain
byte-identical. No dependency, policy, host, component production logic, transport
configuration, hook, workflow, harness or D-125/M1/M2 change was made.

## Verification results

- Baseline frontend run: Passed, 6 tests across 2 files before diagnostic edits.
- Required post-edit frontend run: Failed, 9 passed / 3 failed across 2 files.
- All 5 client tests and 4 existing component tests passed. The 3 new parameterized
  cases (`network`, `http_status`, `provider_stream`) failed at the alert assertion.
  The rendered status was `streaming` with partial text after advancing fake time
  500ms in one act. No alert existed at that assertion. Later control-release
  assertions were not reached. This does not prove a production runtime failure.
- A test scheduling issue is a hypothesis: existing passing streaming tests advance
  250ms in separate acts, while new cases batch two intervals. No reproduction,
  test change or confirming rerun followed the failure.
- Focused Rust tests and full offline verify: Not run after the mandated stop.
  Added Rust tests are uncompiled and unverified; no passing claim is made.
- Failure-closeout documentation, repository, secret scanning, whitespace and
  session inventory: Passed. No conflicts or staged files.
- Preservation and exact scope: Passed against the frozen external manifest.
- Native GUI/live behavior: Not run; remaining advisory retained. Historical
  Python/OpenSSL DNS/TCP/TLS success is not Rust-client or provider evidence.

## Architecture findings

Static review: existing error enum and derived frontend allowlist suffice. No new
IPC command/field, abstraction, authority, persistence or coupling was added.
Only two payload-free variants separate generic HTTP and accepted stream errors;
network still covers non-timeout send/read failures. Existing ownership and
cancellation paths are unchanged. Runtime correctness remains unaccepted because
required tests and full verification have not passed.

## Security findings

Static review: no credential, raw error, header, body, request identifier, URL
or status payload enters a new error field/log. HTTP error bodies remain unread;
SSE failures still pass the existing framing/sequence/creation prerequisites.
Other special mappings and timeout/protocol/refusal/incomplete/limit paths remain.
No retry, fallback, proxy, redirect, endpoint, TLS, request or permission changes.
Secret scan passed. Rust regressions use dummy fixtures and request construction
only, but were not executed. D-128 limitations and D-127 dependency advisories
remain inherited and unchanged; no fresh audit or native success is claimed.

## Code-health findings

High: required UI regression assertions failed; completion is blocked. The
client tests accept only fixed codes and reject extra data/unrecognized strings.
New Rust fixtures cover status/serialization and sanitized stream failures, but
are not validated. No source repair was made after the failed check. Production
component and native session files remain byte-identical to the candidate.

## Technical debt

The unresolved validation blocker above requires separate diagnosis and explicit
authorization before correction. Diagnostic codes intentionally identify only a
failure stage; they cannot recover the cause of the previous native failure.
No general logging system or provider-data retention was introduced to compensate.
Native/provider and inherited dependency advisories remain, with their existing
private-demo boundaries; no unrelated debt was repaired.

## Roadmap findings

D-125/M1/M2 remain parked. This failure does not advance implementation readiness,
authorize another request, or reopen any predecessor record. The exact next task
is read-only diagnosis of the failed new UI test scheduling and a bounded
correction/admission proposal. Do not start a successor automatically.

## Completion decision

FAIL. Freeze this report and use ordinary `close-failed`; never finalize this
increment. Preserve the implementation edits and original completion evidence.
No completion marker is warranted. Gate status and Stop must validate the terminal
failed disposition. Passing closeout hygiene does not override the failed tests.

## Next-increment readiness

Blocked. A separately approved read-only diagnosis may assess the test failure
and lawful successor prerequisites; it cannot reopen or promote this failure.
Do not launch the old bundle as evidence of new diagnostic behavior. No live
request is authorized by this report. One approved rehearsal attempt remains.

## Exact files changed

The complete 32-path Git inventory (including 30 inherited paths) is:

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md`
- `docs/plans/2026-09-21-direct-provider-failure-diagnostics.md`
- `docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md`
- `docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/Cargo.lock`
- `src-tauri/Cargo.toml`
- `src-tauri/src/agent/gateway_request.rs`
- `src-tauri/src/agent/native_runtime.rs`
- `src-tauri/src/agent/runtime.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/personal_assistant_direct_tauri.rs`
- `src-tauri/src/personal_assistant_v0.rs`
- `src/App.test.tsx`
- `src/features/conversations/ConversationWorkspace.tsx`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `src/features/conversations/PersonalAssistantDirectDemo.tsx`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`

## Exact commands executed

The machine schema requires every verification command in its command catalog,
including commands explicitly marked Not run. The statuses below distinguish
actual execution from planned-but-unrun checks; no Not run entry is execution
or passing evidence. The frontend command ran once before edits (6 passed) and
once after edits (9 passed / 3 failed), with no retry after failure.

- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx` — Failed.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1` — Not run.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify` — Not run.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check` — Passed.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check` — Passed.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan` — Passed.
- `git diff --check` — Passed.
- `python3 -B .codex/hooks/session_end_gate.py` — Passed.

Additional actions: ordinary begin passed; exact candidate transfer and two-snapshot
preservation scripts passed; scoped Prettier and rustfmt formatting completed;
static composed review produced FAIL / Blocked. Repository checks after the stop
were limited to truthful failure closeout, not continued implementation acceptance.
