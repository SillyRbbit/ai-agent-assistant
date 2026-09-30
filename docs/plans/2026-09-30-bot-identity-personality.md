# Bot Identity and Personality milestone

Owner-authorized 2026-09-30; increment `bot-identity-personality`.

## Context and preservation

Worktree: `/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant`.
Branch: `codex/provider-milestone`; starting commit `fe7e663e175eaf7c515c17397cdd81060137a5b6`.
Remote main `e0d0547f626ce909c3cd74d4441a5adb2cdebbef` has the identical accepted tree.
Initial worktree clean. Provider milestone completed raw state archived byte-for-byte
in `/private/tmp/cortexa-bot-identity-personality-evidence/provider-state` before ordinary
admission. All other checkouts/records/prunable registrations are preserved.
OpenAI QA is parked, 4/5 attempts used; no live request is authorized here.

## Goal and scope

Extend existing profiles and revision ownership, additive SQLite identity sidecar,
Bots presentation, native provider context and focused tests. Keep stable canonical
agent IDs/roles; personality is untrusted presentation/communication data, not
capability or task ownership. Keep six connections and all runtime controls.

Expected executable paths: agent_preferences.rs, storage/agent_preferences.rs,
storage/migrations.rs, agent_chat.rs, agent_chat_tauri.rs (test constructors),
codex_connection.rs (shared precedence only), tests/storage_smoke.rs, tests/startup_storage_smoke.rs and storage/store.rs
(migration-version assertions only); frontend
AgentsPage.tsx, AgentsPage.css, AgentsPage.test.tsx, agent-chat-client.ts and its
test, application/navigation.ts and App.tsx. Documentation: this plan/review,
ARCHITECTURE, CHANGELOG, DECISIONS, HANDOFF, NEXT_STEPS, PLANS, PROJECT_STATUS,
SECURITY, TESTING_GUIDE and TROUBLESHOOTING_LOG. No dependency, graph, workflow,
governance or orchestrator changes. Exact final inventory must match the report.

## Completion checklist

- [x] Validate bounded nickname, static avatar, description, tone and verbosity.
- [x] Add compatible migration/defaults; preserve settings and notes on restart.
- [x] Present Bots with nickname and canonical role; retain six connections.
- [x] Personality-only reset retains provider settings, instructions and notes.
- [x] Native context captures identity/role/personality for all existing adapters.
- [x] Application rules > current task > custom owner preferences > presets;
      no personality text grants tools, permissions, delegation or note access.
- [x] Revision changes reject stale conversations and isolate all nine bots.
- [x] Focused validation/migration/persistence/context/UI/redaction tests pass.
- [x] Full offline verification, docs/security/preservation and completion pass.
- [x] Manual QA instructions and live-unverified limitations recorded.

## Validation and risks

Run focused Rust/profile/context and frontend/client tests, then full offline verify
and debug app-only bundle with existing Python 3.12, Xcode/SDK 27.0 and Cargo
build-override stripping workaround. Validate migration checksums, all prior schema
entries, unchanged orchestration fixtures, dependency bytes, historical suffixes,
report schema, session/quality/finalization and full Stop. No live generation.
Risks: profile injection, stale context and note leakage; tests must enforce native
validation and context separation. Recover routine in-scope failures here. No new
runtime authority, speculative messaging, live bot-to-bot, background work, graph,
voice or framework changes. Rollback requires owner approval; never discard work.

## Historical assessment checkpoint

Assessment complete; ordinary admission passed. Existing profile revisions and
sidecar persistence will be extended. Next: implement bounded identity fields,
additive migration and shared non-authorizing provider context, then UI and tests.
All verification remains pending until observed. No secrets in checkpoint data.

### Implementation checkpoint

Identity validation, additive migration 6, shared non-authorizing context and Bots UI
are implemented. Focused frontend/client 32/32 and native identity/migration 27/27
passed; typecheck passed. Existing schema-version assertions in storage/store.rs
and startup_storage_smoke.rs also need version 6, within migration regression scope.
No orchestrator or fixture implementation changed. Next: focused storage/context
regressions, strict lint, then full offline verification and key-free debug bundle.

## Exact milestone inventory

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
- `docs/plans/2026-09-30-bot-identity-personality.md`
- `docs/reviews/2026-09-30-bot-identity-personality-post-increment-review.md`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/storage/agent_preferences.rs`
- `src-tauri/src/storage/migrations.rs`
- `src-tauri/src/storage/store.rs`
- `src-tauri/tests/startup_storage_smoke.rs`
- `src-tauri/tests/storage_smoke.rs`
- `src/App.tsx`
- `src/application/navigation.ts`
- `src/features/agents/AgentsPage.css`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`

### Native regression checkpoint

The full native unit run passed 377 tests and found one stale startup-summary
assertion expecting five migrations. Only that test expectation in startup.rs now
expects six; the exact inventory includes this necessary migration regression path
(29 total). An earlier IPC test fixture lacked the now-required identity object and
was corrected without weakening unknown-field rejection. No startup production
behavior or deadlines changed. Full verification is next.

- `src-tauri/src/startup.rs` (test assertion only)

### Validation recovery

Full verification caught six shorthand callbacks rejected by the existing strict
frontend lint rule. The callbacks were changed to explicit blocks in the same
component; lint policy is unchanged. Shared instructions explicitly apply nickname
and style as presentation only. Earlier failed logs are retained; full verification
will rerun after these final implementation edits.

## Final implementation and validation checkpoint

All implementation checklist items are complete; all available required automated
checks passed. Final report validation, ordinary finalization and full Stop are
performed after the report is frozen; inspect gate status/receipt for their result.
This is implementation acceptance with advisories, not verified live personality.
Full offline verification passed: 434 frontend, 378 native unit, 255 integration,
74 hook and 85 repository tests. One opt-in real-Hermes test remains ignored by its
existing policy. Strict Clippy/format/typecheck, release build, debug app bundle,
docs/repository/security/whitespace and preservation checks passed. No live requests.
The required file inventory is 29 paths including startup.rs's test-only assertion.

### Manual QA and launch

Open the exact prepared unsigned debug app from Finder:
`/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant/src-tauri/target/acceptance/debug/bundle/macos/Cortexa.app`.
Bundle ID `com.aiagentassistant.desktop`, version `0.1.0`, arm64. Executable SHA-256:
`7e1ce670611952647a85e1fb6586a07342415a6a843e6deca316373179e64029`.
Do not rebuild or assume an older running Cortexa window is this artifact.

1. On Bots, inspect all nine canonical roles and all six connection choices.
2. Edit one nickname/avatar/description/tone/verbosity, Save, switch bots and return;
   confirm canonical role, provider/model/effort/instructions/notes are retained.
3. Quit/reopen the exact bundle and verify persistence. Other bots remain isolated.
4. Reset identity and personality, then Save: only the five identity fields reset.
5. Start a fresh Simulation conversation; inspect identity attribution and controls.
   Simulation is not evidence of a provider following the personality.
6. Any live check needs separate owner-only authentication, selected runtime/model,
   approved context, allowance and explicit acknowledgement. OpenAI remains parked;
   no automatic retries are authorized. Codex may retry inside its own runtime.

Native visual QA and live behavior remain pending for the owner. Automated stale
revision, restart persistence and context-boundary checks already passed and must
not be represented as direct Computer Use observations. Remote CI was not triggered.

### Resume prompt

```text
Assist owner manual QA of the completed Bot Identity and Personality milestone in
/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant. Inspect AGENTS.md,
the current plan/review, valid completion marker and prepared debug bundle first.
Preserve all 29 changed paths, historical records and other checkouts; do not rebuild
or repeat passing checks. Verify nine bots, nickname plus canonical role, static
avatars, tone/verbosity, six connections, save/restart persistence, personality-only
reset and per-bot note isolation. Record only observed native results. Keep OpenAI
parked at 4/5 used; no live Send or discovery without separate provider/model/context,
request allowance and final acknowledgement, including Codex internal-retry terms.
Do not edit, publish or start bot-to-bot messaging. Stop on drift, unsupported access
or unexpected live failure. Keep D-125/M1/M2 parked.
```
