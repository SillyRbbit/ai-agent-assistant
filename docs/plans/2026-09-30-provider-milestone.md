# Provider milestone execution plan

Owner authorization: 2026-09-30. Increment: `provider-milestone`.

Complete the existing direct OpenAI and headless Codex per-agent text conversation
connections while preserving Anthropic, LM Studio, Ollama and simulation. Reuse
SQLite settings, native conversation ownership and typed IPC. The living checklist,
checkpoint, scope, blockers and QA status are in [provider milestone](../provider-milestone.md).

Keep credential values outside the WebView, database, diagnostics and reports.
Codex authentication belongs to its supported runtime; it must not receive the
OpenAI API environment credential or inherit arbitrary user tools/configuration.
No model-to-device authority or provider fallback follows from profile selection.
Use bounded protocol input/output, explicit terminal completion and child cleanup.

Run focused native/client/UI regressions first, then the applicable full verify,
documentation/repository/security checks, independent quality/session/report review
and ordinary completion. Offline fixtures prove implementation, not live service
access. Manual QA and blocked live checks must be identified separately. Preserve
historical records. Rollback means discarding only this future reviewed delta with
owner permission, never resetting existing work. Do not commit or publish.

## Frozen implementation inventory

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
- `docs/plans/2026-09-30-provider-milestone.md`
- `docs/provider-milestone.md`
- `docs/reviews/2026-09-30-provider-milestone-post-increment-review.md`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/storage/agent_preferences.rs`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`

## Final implementation result

All functional checklist items are implemented and available automated suites passed.
The existing SQLite sidecar stores max effort without changing migrations or notes.
Six connection choices remain. Final full verification and debug bundle passed;
read the consolidated review and ordinary marker status for completed gate evidence.
Live account access, owner GUI QA and other-platform CI remain separate limitations.
No paid generation, credential inspection, app launch or publication was performed.
