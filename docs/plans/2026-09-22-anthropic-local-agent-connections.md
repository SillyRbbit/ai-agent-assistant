# Anthropic and local agent connections

Date: 2026-09-22
Increment: anthropic-local-agent-connections

## Authorization and goal

The owner's latest attachment, “CORTEXA — ADD ANTHROPIC AND OPEN-WEIGHT MODEL CONNECTIONS”, expressly authorizes minimal native HTTP transport, typed IPC, connection configuration, capability validation, UI and tests. It specifically authorizes native Anthropic Messages, explicit GET /v1/models and backend-only ANTHROPIC_API_KEY code; and loopback LM Studio/Ollama chat-completions, metadata discovery and separately configured local authentication. Editing this implementation does not read any credential or execute any request. No live-generation allowance is granted. This supersedes the earlier publication task; no commit or push is authorized here.

Work from clean published candidate HEAD 109a6d5f24afb3cad4b13f9a4d4e30fc1c2e24c7, tree 195e69200fa33066bb919c1dbff3f9a8eca1c3ce, with valid optional-api-status-repair predecessor completion. Ordinary begin admitted this task before edits. Preserve other worktrees, original dirty checkout, historical reports and parked D-125/M1/M2.

## Exact scope

- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/agent_models.rs`
- `src-tauri/src/anthropic.rs`
- `src-tauri/src/local_models.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/startup.rs`
- `src-tauri/src/storage/agent_preferences.rs`
- `src-tauri/src/storage/migrations.rs`
- `src-tauri/src/storage/store.rs`
- `src-tauri/tests/startup_storage_smoke.rs`
- `src-tauri/tests/storage_smoke.rs`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`
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
- `docs/plans/2026-09-22-anthropic-local-agent-connections.md`
- `docs/reviews/2026-09-22-anthropic-local-agent-connections-post-increment-review.md`

## Checkpoints and boundaries

1. Add bounded Anthropic model/capability discovery and separate Messages SSE parsing. Exact documented IDs include Fable 5.1, Opus 5.5, Sonnet 5, Haiku 4.5 and active legacy Opus 5. Default effort omitted; use only supported values, no thinking-disable flag. Text-only no-tool followups may omit prior thinking under official guidance; do not store or display signatures.
2. Add one shared local adapter, exact loopback endpoints, no proxy/redirect/retry, fixed routes, isolated optional credentials, native discovery metadata and cloud rejection. Locality remains unknown; enabled private notes require explicit per-destination/model owner decision. No runtime management, model acquisition or family-based capability guessing.
3. Extend existing saved profiles, native-bound context, typed IPC and Agents selector. Append an additive connection-settings table; leave existing preferences table and notes intact and save both atomically. Retain revision invalidation, all-nine-agent isolation, old OpenAI requests, Codex live-disabled boundary and shared ownership/Stop/timeout. New command is narrowly allowlisted; checker rules otherwise unchanged.
4. Offline fixture validation and required completion review; owner smoke instructions only. No builds with credentials, runtime launch, generation, graph change, new dependencies, permissions, CSP, workflows, hooks or governance changes.

## Validation

Run focused frontend tests and typecheck, focused native library tests for agent/providers/storage, then CARGO_NET_OFFLINE=true npm run verify. Run docs:check, repository:check, security:scan and git diff --check, session-end, independent architecture/security/code/readiness review, exact scope/report validation, ordinary finalization and full-payload Stop. Preserve migration checksums 1–4 and historical report bytes. Manual Anthropic/local discovery and live smoke are advisory pending owner setup and new explicit request allowance; fixtures and builds are not live success. No application or model request is part of automated checks.

## Risks, decisions and limits

Untrusted model metadata is bounded; model IDs stay data. Discovered means listed, never live-tested. Separate Anthropic and local parsers require explicit terminal success and nonempty answer; refusal/truncation/timeout/error remain failure and never trigger fallback. Endpoint identity participates in saved revision and snapshots. No secret in IPC, SQLite, logs or response bodies. D-127 audit debt and D-128 transient-memory/remote-abort limitations remain. Previous OpenAI attempts remain historical and are not renewed by this task.

Automatic approval review rejected the first table-rebuild proposal because it would drop persistent preferences. The safer additive table leaves the existing data table unchanged. Routine code/test/format fixes remain within this open increment, at most two additional attempts per distinct failure. Stop on unresolved security/shared-build failure or necessary scope expansion; preserve work rather than reset. No diagnostic successor chain.

## Implementation corrections

All corrections remain in this same increment. Focused frontend initialization
and stale-count fixes passed. A checker false positive on a comment followed by
an ordinary `emit` callback was resolved by naming that callback `on_event`,
without weakening event checks. The repository test's copied command list now
includes only the approved discovery command. Full native verification found a
stale startup migration-count assertion and an incorrectly changed synthetic host
sequence assertion. Restoring the latter removes personal_assistant_v0.rs from
the change set; updating startup.rs's test replaces it in the exact 32-path scope.
No production host or startup behavior changes. The required passing reruns completed; failed runs remain historical task
evidence and do not override the final passing results.

## Progress and handoff

Admission passed. Implementation is finished. Focused frontend/provider/storage checks and the full offline verification passed: 74 hook tests, 85 repository tests, 431 frontend tests, 368 Rust library tests, integration tests, strict checks and frontend/native release builds. Documentation, repository, security, whitespace, exact scope/preservation and session checks passed. Independent review returned PASS WITH ADVISORIES / Ready with advisories with no blocking finding. Ordinary report validation/finalization and Stop are the final recorded gate steps; their current status is authoritative. No live requests were used or newly authorized. Account/runtime/native GUI evidence remains pending; owner-only setup and optional smoke instructions are in TESTING_GUIDE.md. No other independent implementation remains within this amendment.

## Public contract references

Official [Anthropic model overview](https://platform.claude.com/docs/en/models/overview),
[Models API](https://platform.claude.com/docs/en/api/models/list),
[Messages API](https://platform.claude.com/docs/en/api/messages/create),
[effort](https://platform.claude.com/docs/en/build-with-claude/effort),
[stream lifecycle](https://platform.claude.com/docs/en/build-with-claude/streaming),
and [thinking preservation](https://platform.claude.com/docs/en/build-with-claude/thinking#preserving-thinking-blocks)
were inspected publicly without authentication or provider requests. Documented
access is distinct from account-specific discovery and live-tested behavior.

Local contracts use [LM Studio compatibility](https://lmstudio.ai/docs/developer/openai-compat),
[model metadata](https://lmstudio.ai/docs/developer/rest/list),
[LM Link locality caveat](https://lmstudio.ai/docs/developer/core/lmlink),
[Ollama compatibility](https://docs.ollama.com/api/openai-compatibility),
[tags](https://docs.ollama.com/api/tags),
[cloud](https://docs.ollama.com/cloud), and official
[Ollama API types](https://github.com/ollama/ollama/blob/main/api/types.go).
Metadata is not a hardware, license or offline-processing guarantee.
