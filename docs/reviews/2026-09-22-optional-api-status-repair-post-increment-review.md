# Optional API status repair post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/infrastructure/tauri/agent-chat-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked --lib personal_assistant_direct",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run tauri -- build --debug --bundles app --no-sign -- --offline --locked",
    "env npm_config_offline=true npm run docs:check",
    "env npm_config_offline=true npm run repository:check",
    "env npm_config_offline=true npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-09-22-configurable-agent-current-models.md",
    "docs/plans/2026-09-22-configurable-agent-demo.md",
    "docs/plans/2026-09-22-optional-api-status-repair.md",
    "docs/reviews/2026-09-22-configurable-agent-current-models-post-increment-review.md",
    "docs/reviews/2026-09-22-configurable-agent-demo-post-increment-review.md",
    "docs/reviews/2026-09-22-optional-api-status-repair-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/agent_chat.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/agent_preferences.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/personal_assistant_direct_tauri.rs",
    "src-tauri/src/startup.rs",
    "src-tauri/src/storage/agent_preferences.rs",
    "src-tauri/src/storage/migrations.rs",
    "src-tauri/src/storage/mod.rs",
    "src-tauri/src/storage/store.rs",
    "src-tauri/tests/startup_storage_smoke.rs",
    "src-tauri/tests/storage_smoke.rs",
    "src/App.test.tsx",
    "src/App.tsx",
    "src/application/navigation.ts",
    "src/application/state.test.ts",
    "src/components/ApplicationSidebar.tsx",
    "src/features/agents/AgentsPage.css",
    "src/features/agents/AgentsPage.test.tsx",
    "src/features/agents/AgentsPage.tsx",
    "src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.test.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.ts"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Small",
      "milestone": "Optional owner-acknowledged native observation",
      "risk": "An HTTP status alone does not identify the model, account, billing or request defect; the rebuilt candidate has no live-success evidence.",
      "severity": "Advisory",
      "summary": "Optional live verification awaits private owner setup and acknowledgment; this new batch remains 0/1 used."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Existing follow-up",
      "milestone": "Existing D-127/D-128 maintenance",
      "risk": "Accepted audit debt and synthetic custody/abort/retention limitations remain unchanged.",
      "severity": "Advisory",
      "summary": "D-127 and D-128 advisories remain; D-125/M1/M2 stay parked and Codex live stays disabled."
    }
  ],
  "increment_id": "optional-api-status-repair",
  "manual_verification": [
    {
      "check": "Exact bundle identity and executable equality after the offline app-only debug build",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Independent architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Worktree/registry, archived gate, protected-byte and scope preservation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "One acknowledged fixed-sample native request and live result observation",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/infrastructure/tauri/agent-chat-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked --lib personal_assistant_direct",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run tauri -- build --debug --bundles app --no-sign -- --offline --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env npm_config_offline=true npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env npm_config_offline=true npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env npm_config_offline=true npm run security:scan",
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
  ]
}
-->

Date: 2026-09-22
Increment: optional-api-status-repair
Branch: codex/direct-provider-stream-stage-diagnostics

## Executive summary

The optional bounded repair distinguishes HTTP 400, 403 and 404 using payload-free static errors. Focused and full offline verification passed. Quality result: PASS WITH ADVISORIES. No new live request has been made; the optional batch is 0/1 used pending private owner setup and acknowledgement. OpenAI API remains unavailable/unverified for live success.

## Scope and boundaries

Exactly 14 task paths, with a 44-path cumulative dirty candidate. Six source/test paths implement only the status distinction; six current-state documents plus this plan/report record it. Preserve the 38 inherited paths, historical failures and previous completed report/raw-state archive. No model, endpoint, request, transport, dependency, ownership, permission, hook or governance changes. No provider response bodies, credentials, headers or raw errors are inspected. No successor task or evidence framework was created.

## Verification results

Passed: 38 focused frontend tests; 17 focused Rust tests; full offline verify with 74 hook, 85 repository, 421 frontend and 337 Rust library tests plus the integration suite, strict formatting/lint/type checks and release build. The existing opt-in real-Hermes version test remains ignored; it is unrelated and was not activated. Offline debug app-only locked build passed with --no-sign. Bundle identifier com.aiagentassistant.desktop, version 0.1.0; bundle executable equals the newly built debug executable, SHA-256 e3f329d126153251d618e52d5ccfb815430b300af8280672c6d08a2ea158555e.

Documentation formatting initially failed on extra blank lines in the six added entries. Only those new prefixes were formatted in this same open task; the rerun of documentation, repository, security and whitespace checks passed. Historical text is preserved. Final report formatting/schema and final preservation checks are performed before ordinary finalization. Native observation is optional and pending, never inferred from key activity.
The draft report-schema check rejected the unsupported finding category
`Readiness`; it was corrected to the existing allowed `Roadmap` category
within this same open task before finalization.

## Architecture findings

Independent read-only review found no concrete issue. Rust retains status classification before any unsuccessful response-body read. Both frontend clients use closed code lists, and legacy model_unavailable remains readable for historical evidence. Request constructors, transport policy, SSE validation, generation ownership and cancellation match their pre-task behavior. No new abstractions or coupling.

## Security findings

No concrete security finding. Static error enums/messages expose only HTTP categories. Frontend tests reject suffixed codes, raw objects and private dummy text. Existing no-proxy, no-redirect, TLS, timeout and no-retry rules remain. Models and owner settings are unchanged. No credential, process environment or Terminal content inspection; no provider request. Existing D-128 synthetic-only custody/abort limits and retention disclosures remain.

## Code-health findings

No concrete review finding. The existing parameterized UI lifecycle tests now cover all three new codes through streaming, terminal busy cleanup and released controls, preserving partial output, absent completed answer and one-Start/no-retry assertions. Agent and fixed client tests cover closed parsing and exact static messages. Rust tests prove status mapping and code serialization. Production UI components remain byte-identical.

## Technical debt

Existing advisories only: API upstream cause/live success unknown (owner, small observation effort; optional API lane, blocks neither this repair nor verified Agents/settings/notes); D-127 audit debt (owner, existing maintenance effort); D-128 custody/abort/retention limitations (owner, unchanged synthetic-only boundary). All are Advisory and block neither completion nor unrelated next work. No new evidence pipeline or general logging debt.

## Roadmap findings

Keep D-125/M1/M2 parked, historical provider_stream_error_event evidence and prior model_unavailable failures intact. Codex live remains disabled under its existing isolation advisory. This optional API lane is independent of previously verified Agents, settings and private notes. If a later authorized one-shot observation fails or remains unclear, park this connection; no automatic further investigation, retry or closeout chain.

## Completion decision

PASS WITH ADVISORIES. The local status repair meets its bounded automated acceptance criteria. API connection success is explicitly not an acceptance prerequisite for this optional diagnostic repair. Ordinary finalization is permitted only after final report/scope/preservation validation. Marker status and the full-payload Stop result are checked afterward, without mutating finalized records.

## Next-increment readiness

Ready with advisories. No new implementation increment is proposed. The one optional fixed-sample native observation requires the pending owner-only private launch and explicit acknowledgement. The assistant has performed zero new Starts. No automatic retry, model change, account action, commit or publication is authorized. If the owner declines or provides no acknowledgement, keep live verification pending and the API connection unverified.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-22-configurable-agent-current-models.md`
- `docs/plans/2026-09-22-configurable-agent-demo.md`
- `docs/plans/2026-09-22-optional-api-status-repair.md`
- `docs/reviews/2026-09-22-configurable-agent-current-models-post-increment-review.md`
- `docs/reviews/2026-09-22-configurable-agent-demo-post-increment-review.md`
- `docs/reviews/2026-09-22-optional-api-status-repair-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/personal_assistant_direct_tauri.rs`
- `src-tauri/src/startup.rs`
- `src-tauri/src/storage/agent_preferences.rs`
- `src-tauri/src/storage/migrations.rs`
- `src-tauri/src/storage/mod.rs`
- `src-tauri/src/storage/store.rs`
- `src-tauri/tests/startup_storage_smoke.rs`
- `src-tauri/tests/storage_smoke.rs`
- `src/App.test.tsx`
- `src/App.tsx`
- `src/application/navigation.ts`
- `src/application/state.test.ts`
- `src/components/ApplicationSidebar.tsx`
- `src/features/agents/AgentsPage.css`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`

Task-only 14-path scope:

- `src-tauri/src/personal_assistant_direct.rs`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-22-optional-api-status-repair.md`
- `docs/reviews/2026-09-22-optional-api-status-repair-post-increment-review.md`

## Exact commands executed

- Passed: `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/infrastructure/tauri/agent-chat-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- Passed: `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked --lib personal_assistant_direct`
- Passed: `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify`
- Passed: `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run tauri -- build --debug --bundles app --no-sign -- --offline --locked`
- Passed: `env npm_config_offline=true npm run docs:check`
- Passed: `env npm_config_offline=true npm run repository:check`
- Passed: `env npm_config_offline=true npm run security:scan`
- Passed: `git diff --check`
- Passed: `python3 -B .codex/hooks/session_end_gate.py`

The first documentation-check run failed formatting, then passed after the bounded prefix-only repair. The prior completed gate was copied byte-for-byte to /private/tmp/cortexa-configurable-agent-current-models-evidence/completed-current-models-before-api-repair.json (SHA-256 39e9ba834715bf37c662c5394e19a5bd7587ef9e8654b03c6d8841bd334a082b), then ordinary begin admitted optional-api-status-repair. Read-only inline Python/SHA-256 and in-memory comparisons verified identity, preservation and scope; no external validator framework was added. Public [OpenAI error guidance](https://developers.openai.com/api/docs/guides/error-codes#python-library-error-types) was inspected; it does not establish this account's underlying cause.
