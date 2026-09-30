# Saved bot names in the graph post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "bot-graph-name-sync",
  "commands_executed": [
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend -- src/features/command-center/CommandCenterPage.test.tsx src/features/command-center/commandCenterBotNames.test.ts",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend -- src/features/command-center/CommandCenterPage.test.tsx",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run lint:frontend",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run typecheck",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run format:frontend",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run docs:check",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run repository:check",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-bot-graph-name-sync-evidence/check-preservation.py",
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
    "docs/plans/2026-09-30-bot-graph-name-sync.md",
    "docs/plans/2026-09-30-bot-identity-personality.md",
    "docs/reviews/2026-09-30-bot-graph-name-sync-post-increment-review.md",
    "docs/reviews/2026-09-30-bot-identity-personality-post-increment-review.md",
    "src-tauri/src/agent_chat.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/agent_preferences.rs",
    "src-tauri/src/codex_connection.rs",
    "src-tauri/src/startup.rs",
    "src-tauri/src/storage/agent_preferences.rs",
    "src-tauri/src/storage/migrations.rs",
    "src-tauri/src/storage/store.rs",
    "src-tauri/tests/startup_storage_smoke.rs",
    "src-tauri/tests/storage_smoke.rs",
    "src/App.tsx",
    "src/application/navigation.ts",
    "src/features/agents/AgentsPage.css",
    "src/features/agents/AgentsPage.test.tsx",
    "src/features/agents/AgentsPage.tsx",
    "src/features/command-center/CommandCenterPage.test.tsx",
    "src/features/command-center/CommandCenterPage.tsx",
    "src/features/command-center/commandCenterBotNames.test.ts",
    "src/features/command-center/commandCenterBotNames.ts",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts"
  ],
  "verification": [
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend -- src/features/command-center/CommandCenterPage.test.tsx src/features/command-center/commandCenterBotNames.test.ts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend -- src/features/command-center/CommandCenterPage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run lint:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run typecheck",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run format:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-bot-graph-name-sync-evidence/check-preservation.py",
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
      "check": "Owner native graph labels and canonical-role visual inspection",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "New graph native visual QA and remote CI remain unverified; retain all prior provider/runtime, Codex-isolation, D-127/D-128 and process-local native build advisories.",
      "risk": "Automated graph assertions do not prove native visual layout or live provider behavior.",
      "effort": "One owner key-free walkthrough; separate authority for paid requests or publication.",
      "milestone": "Owner graph QA",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories"
}
-->

Date: 2026-09-30
Increment: bot-graph-name-sync
Branch: codex/provider-milestone

## Executive summary

Saved bot nicknames now overlay Command Center agent labels through the existing validated read-only list. Available automated frontend acceptance passed. PASS WITH ADVISORIES; native visual QA remains pending.

## Scope and boundaries

Exactly 13 successor paths and 35 cumulative paths, including 29 inherited Bot Identity paths. Canonical IDs/roles, topology, simulation, ownership, Rust/IPC schemas, providers, instructions, notes, dependencies, workflows and governance stay protected. Seven root documents retain exact historical suffixes; predecessor completed raw state and debug bundle are archived. This owner-requested label presentation alone supersedes the earlier no-graph boundary.

## Verification results

Focused 39/39, frontend 449/449, affected page rerun 28/28 passed. Strict frontend lint, typecheck, formatting and offline app-only debug bundle passed. Docs/repository/security/whitespace, preservation and session checks passed. Initial inspector/search locator and test lint failures were corrected here; logs retained. Rust tests and full npm run verify: Not run for frontend-only scope under ENGINEERING_GUIDE. They are not listed as executed; unchanged prior native results are inherited historical evidence only. Schema, finalization and full Stop run after report freeze and are recorded externally.

## Architecture findings

Reviewed the successor diff and unchanged fixture/client contracts. Name overlay lives solely in presentation; stable canonical IDs route and attribute. All scenarios retain original edges/groups/events and non-agent nodes. No orchestrator contract, authority transfer, messaging infrastructure, graph layout or native proof-panel change. Read once on mount; returning from Bots refreshes names.

## Security findings

Only the existing validated list_agent_preferences client is read. Graph retains ID/name pairs, not notes, instructions or connection data. React renders names as text; bounded/control-character checks and canonical fallback remain. Failure notice is static; no raw error is exposed. Regression asserts no discovery/Send and no private note/instruction text. Credentials, permission, policy, cancellation and live boundaries unchanged.

## Code-health findings

Pure immutable overlay is validated/frozen with the existing projection validator. UI success/failure and native client wiring tests pass; canonical roles remain inspectable/searchable. Unmount guard ignores late results. Browser-only fixtures remain usable with canonical names. No blanket lint allowances, dependencies or dead-code changes.

## Technical debt

Advisory verification debt: native graph visual QA and exact-head remote CI pending. Risk: automated fixtures cannot prove visual behavior or real provider outcomes. Effort: one owner walkthrough; later separately authorized publication. Blocks completion: no. Blocks next increment: no. Retain D-127 audit debt, D-128 custody/abort limits, native/provider/runtime and Codex isolation/internal-retry advisories and process-local native workaround.

## Roadmap findings

Only requested nickname synchronization is implemented. Keep OpenAI parked at 4/5 used, D-125/M1/M2 parked. No provider expansion, Structured view, graph redesign or autonomous bot communication. Next: owner key-free native graph QA; bounded live collaboration requires separate authority.

## Completion decision

PASS WITH ADVISORIES. Automated acceptance requirements passed; optional native visual check pending. Ordinary finalization and full Stop remain separate post-freeze receipts, not inferred from this report.

## Next-increment readiness

Ready with advisories for owner key-free graph QA. No paid request, publication or collaboration authority is granted.

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
- `docs/plans/2026-09-30-bot-graph-name-sync.md`
- `docs/plans/2026-09-30-bot-identity-personality.md`
- `docs/reviews/2026-09-30-bot-graph-name-sync-post-increment-review.md`
- `docs/reviews/2026-09-30-bot-identity-personality-post-increment-review.md`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/startup.rs`
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
- `src/features/command-center/CommandCenterPage.test.tsx`
- `src/features/command-center/CommandCenterPage.tsx`
- `src/features/command-center/commandCenterBotNames.test.ts`
- `src/features/command-center/commandCenterBotNames.ts`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`

## Exact commands executed

- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend -- src/features/command-center/CommandCenterPage.test.tsx src/features/command-center/commandCenterBotNames.test.ts` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run test:frontend -- src/features/command-center/CommandCenterPage.test.tsx` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run lint:frontend` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run typecheck` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run format:frontend` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline --config 'profile.release.build-override.strip="none"'` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run docs:check` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run repository:check` — Passed (final applicable run).
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run security:scan` — Passed (final applicable run).
- `git diff --check` — Passed (final applicable run).
- `python3 -B /private/tmp/cortexa-bot-graph-name-sync-evidence/check-preservation.py` — Passed (final applicable run).
- `python3 -B .codex/hooks/session_end_gate.py` — Passed (final applicable run).

Earlier test harness/locator and test-only lint failures remain in external logs. Strict lint prevented the first build sequence from proceeding; no skipped command is represented as executed. This report lists verification commands only; formatting/admission/read-only inspections and archive creation are in tool/evidence history.
