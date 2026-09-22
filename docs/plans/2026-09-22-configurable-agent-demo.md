# Configurable agent demo

Date: 2026-09-22. Increment: `configurable-agent-demo`.

## Goal and authority

The owner explicitly authorizes one coherent private demo: nine independent
agent profiles, manually managed private notes, a bounded text conversation,
direct OpenAI and an optional safely restricted Codex connection. Historical
provider failures do not block independent implementation. D-125/M1/M2 remain
parked. No commit, publication, workflow redesign or agent tool execution.

## Baseline and preservation

Work in the existing clean published candidate at
`/private/tmp/cortexa-direct-provider-stream-stage-live-result-evidence-closeout`,
HEAD `d9b09255f4c95a80e275a15714fbb6c509673f2c`. The single authorized fetch
confirmed main `afeff59ed8fd7da8bc7edbb6d56f2d8e89e9eee6`; both trees are
`3b7a8a2f371152fa1103105f10adab4f3a2f63d3`. External preservation evidence
freezes all 19 valid worktrees, two prunable entries and the prior completed raw
state. The previous completion record remains archived; ordinary admission
succeeded without governance changes. The 24 existing direct-demo frontend
tests passed before edits.

## Expected file scope

- Native: `src-tauri/src/agent_preferences.rs`,
  `src-tauri/src/storage/agent_preferences.rs`, storage `mod.rs`, `store.rs`,
  `migrations.rs` and `error.rs` only if needed; `startup.rs`, `lib.rs`,
  `agent_chat.rs`, `agent_chat_tauri.rs`, `personal_assistant_direct.rs`,
  `personal_assistant_direct_tauri.rs` for shared transport ownership;
  storage/startup integration tests for the additive migration.
- Frontend: `src/application/navigation.ts`,
  `src/components/ApplicationSidebar.tsx`, `src/App.tsx`, new Agents page/CSS/test
  and typed agent-chat client/test; existing navigation tests only as needed.
- IPC boundary registration: `scripts/repository_health.py` and
  `scripts/tests/test_repository_health.py`, adding only the nine approved
  commands/client imports to the existing exact allowlists and rejection tests.
- Current evidence: this plan, one post-increment review, `CHANGELOG.md`,
  `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `PLANS.md`, `DECISIONS.md`,
  `ARCHITECTURE.md`, `SECURITY.md`, `TESTING_GUIDE.md`.

Dependencies, historical plans/reviews, workflows, permissions, hooks, skills,
graph and fixture workflow implementations remain unchanged.

## Design and invariants

Append one SQLite migration; use the native registry for all nine identities.
Settings and notes are non-secret owner data outside the repository. Bound
conversations capture saved settings and only their own selected note. Revisions
reject stale follow-ups after edits/clear; changes never silently retarget an
active request. Stateless provider requests carry bounded local text history and
never reuse cross-agent/provider IDs. Existing native run ownership gates one
generation. Output is inert text; tools, retries and fallback are absent.

OpenAI offers only `gpt-5.4-mini` and `gpt-5.4-nano` with documented Default/none/low/medium/high/xhigh
efforts and bounded reasoning-item framing; reasoning is never displayed. The
fixed diagnostic sample keeps its original request and decoder behavior.
Codex live remains disabled if the installed runtime cannot prove full text-only
tool/file isolation; do not infer subscription login or inherit credentials.

Sources: [model](https://developers.openai.com/api/docs/models/gpt-5.4-mini),
[nano model](https://developers.openai.com/api/docs/models/gpt-5.4-nano),
[streaming](https://developers.openai.com/api/docs/guides/streaming-responses),
[reasoning](https://developers.openai.com/api/docs/guides/reasoning),
[Codex protocol](https://developers.openai.com/codex/app-server/),
[Codex authentication](https://developers.openai.com/codex/auth/).
[ECC](https://github.com/affaan-m/ECC) is a conceptual reference only; no code,
configuration or hooks are imported.

## Validation and checkpoints

1. Offline native profile persistence, revision and nine-agent isolation tests;
   request capture verifies chosen settings and notes, no cross-agent content.
2. Strict stream tests, error/no-fallback/duplicate/cancel/cleanup tests and UI
   settings, disclosure, save/clear and conversation regressions.
3. Focused tests while working, then full offline `npm run verify`, documentation,
   repository, secret, whitespace, preservation and required completion checks.
4. Native walkthrough: settings/note save, restart persistence, simulation,
   then at most one explicitly acknowledged owner-triggered generation per
   available live connection. Private owner credential/login setup only.
   No live success is inferred from tests; unavailable connections stay pending.
5. Independent security/architecture/quality/readiness review and one exact
   twelve-section report; ordinary finalization and full-payload Stop.

## Risks, repair and rollback

Notes are manually managed, not automatic learning. Hosted inclusion needs an
explicit disclosure; local deletion cannot erase provider copies. Preserve
D-128 custody/abort limitations and D-127 audit debt. No encryption-at-rest or
remote cancellation guarantee is added. Routine failures may receive at most
two evidence-based repairs in this open task. Preserve changes on a blocker;
do not reset, create recovery successors or manufacture a passing marker.

## Progress and results

Admission, baseline and implementation passed. The final exact inventory is 36
paths after the owner approved the existing navigation test needed to add Agents
to both deterministic route expectations. The focused file passed 32/32.
Complete offline `npm run verify` passed: formatting, repository policy, frontend
and Rust lint, 74 hook tests, 85 repository tests, 411 frontend tests, 336 Rust
library tests, the full integration suite, type checking, frontend production
build and release Tauri build. The debug app bundle built offline with locked
dependencies, app-only target and no signing; its executable SHA-256 is
`5370f902ff03a819513646768d5dc8e707821b92caea81d6f4bfc6ba752a46e4`.

Direct native observation opened Agents, saved simulation settings, owner
instructions and a harmless private note at revision 1, stopped and relaunched
the same bundle, and observed the values after restart. A simulated message
visibly entered streaming with native ownership and then reached explicit
completion with no active generation. The answer was visibly labeled simulation
and stated that no hosted request was made. Both test-owned processes stopped.

Independent native review identified and resolved transport-lifetime ownership
gaps with a guard shared by both native text paths, weak task references and
cleanup regressions. A strict Clippy closure warning was corrected. The first
full verification stopped before application checks because the exact IPC
allowlists did not yet include the approved new client/commands; their bounded
registration and negative fixtures passed. A later full run found only the stale
navigation expectation; its approved same-task correction passed and the final
full run succeeded.
New batch live requests used: OpenAI 0/1, Codex 0/1. Historical batches remain
exhausted. OpenAI live verification remains pending private owner setup; Codex
live remains disabled for the documented isolation limitation. Neither is a
claim of live success. Completion result: PASS WITH ADVISORIES.
