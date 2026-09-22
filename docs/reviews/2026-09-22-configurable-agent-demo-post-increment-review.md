# Configurable agent demo post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "node_modules/.bin/vitest run src/application/state.test.ts",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run tauri -- build --debug --bundles app --no-sign -- --offline --locked",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
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
    "docs/plans/2026-09-22-configurable-agent-demo.md",
    "docs/reviews/2026-09-22-configurable-agent-demo-post-increment-review.md",
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
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Architecture",
      "effort": "Medium",
      "milestone": "A separately approved Codex isolation increment",
      "risk": "Enabling the installed runtime without a supported complete text-only tool and file restriction could cross Cortexa's authority boundary.",
      "severity": "Advisory",
      "summary": "Codex is a persisted but deliberately live-disabled connection until supported isolation is proved."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Small",
      "milestone": "Optional owner-operated live verification",
      "risk": "The configured direct path is offline-tested but has no live success evidence for this batch, and D-128 custody and abort limits still apply.",
      "severity": "Advisory",
      "summary": "OpenAI live verification remains pending private owner setup at 0/1 requests used."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Existing dependency and audit maintenance",
      "risk": "The accepted D-127 audit baseline requires continued maintenance as advisories and dependencies change.",
      "severity": "Advisory",
      "summary": "The pre-existing D-127 audit debt remains and was not expanded by this increment."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Unknown",
      "milestone": "Owner-selected future work",
      "risk": "Starting historical browser-harness work would displace the completed demo without current owner direction.",
      "severity": "Advisory",
      "summary": "D-125/M1/M2 remain parked and do not gate this demo."
    }
  ],
  "increment_id": "configurable-agent-demo",
  "manual_verification": [
    {
      "check": "Native Agents settings and note save, process restart persistence, simulation streaming, explicit completion, and released generation ownership",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "One owner-acknowledged direct OpenAI generation in the new batch",
      "required": false,
      "status": "Manual verification pending"
    },
    {
      "check": "One isolated Codex generation in the new batch",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "node_modules/.bin/vitest run src/application/state.test.ts",
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
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-09-22
Increment: `configurable-agent-demo`
Branch: `codex/direct-provider-stream-stage-diagnostics`

## Executive summary

The usable private demo is implemented. Agents exposes nine independent native
profiles with persisted non-secret settings and manually managed notes, plus a
bounded shared native conversation path. Offline verification and the native
save/restart/simulation walkthrough passed. Direct OpenAI is implemented but
not live-verified in this batch; Codex remains deliberately live-disabled. The
quality-gate result is `PASS WITH ADVISORIES`.

## Scope and boundaries

The exact owner-approved 36-path inventory includes the later approved
navigation-test correction and this report. No dependencies, capabilities,
permissions, workflows, hooks, skills, fixture workflows, graph behavior or
native execution authority changed. The model and WebView still cannot invoke
tools, files, shell, MCP, hooks, plugins, subagents or device actions through
this advice path.

## Verification results

The focused navigation file passed 32/32. The final offline `npm run verify`
passed formatting, repository policy, lint, 74 hook tests, 85 repository tests,
411 frontend tests, 336 Rust library tests, the full integration suite,
TypeScript, frontend production build and release Tauri build. The app-only
debug bundle built offline from locked dependencies without signing. Required
documentation, repository, security, whitespace and session checks passed.

Earlier recoverable evidence remains truthful: one full run first stopped at
the expected exact IPC allowlist, and the next reached 410/411 frontend tests
before finding the stale route expectation. Both were corrected in this same
open task within the owner's repair allowance; the final full run passed.

## Architecture findings

No completion blocker. Canonical agent identities, editable preferences,
conversation binding, persistence and transport remain separately owned. A
single native generation lease spans both text paths through transport cleanup.
Codex remains closed rather than relying on prompt-only or read-only isolation.

## Security findings

No completion blocker. Native validation owns agent/note identity and revision;
Memory Off excludes notes. IPC rejects credentials and authority fields. The
direct path has empty tools, no fallback or retry, bounded history/output,
closed errors and owner-only key custody at explicit send. Notes are untrusted
plaintext local data and the UI accurately discloses hosted inclusion and local
deletion limits.

## Code-health findings

No completion blocker. Strict TypeScript, Clippy and full tests passed. Focused
tests cover all nine profiles, restart persistence, model/effort combinations,
cross-agent note exclusion, stale context, malformed IPC, missing credentials,
stream framing, cancellation, duplicate ownership and cleanup. The repository
checker retains an exact allowlist with negative mutation tests.

## Technical debt

D-127's accepted audit debt remains advisory. Codex isolation and direct API
live verification are explicit capability gaps rather than hidden fallbacks.
No new dependency or generalized layout/runtime framework was introduced.

## Roadmap findings

`Ready with advisories`. The smallest useful offline demo is complete. A future
owner may select one short direct OpenAI verification using private setup, or a
separate Codex isolation proof. D-125/M1/M2 remain parked and no successor starts
automatically.

## Completion decision

`PASS WITH ADVISORIES`. All required automated and native manual checks passed.
Optional live checks remain pending/unavailable and are not presented as live
success.

## Next-increment readiness

`Ready with advisories`. The highest-value unresolved task is one owner-operated
direct OpenAI live verification if desired. It must preserve the 0/1 unused
allowance, disclosure, explicit acknowledgement, no retry and private credential
procedure. No work is authorized automatically.

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
- `docs/plans/2026-09-22-configurable-agent-demo.md`
- `docs/reviews/2026-09-22-configurable-agent-demo-post-increment-review.md`
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
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`

## Exact commands executed

Passing final checks:

```text
node_modules/.bin/vitest run src/application/state.test.ts
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run tauri -- build --debug --bundles app --no-sign -- --offline --locked
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Read-only bundle identity, dependency fingerprints, exact inventory,
preservation, Git status/diff and gate-status commands also passed. Native
Computer Use opened Agents, saved and reloaded settings, and observed simulation
streaming/completion. The test-owned processes were stopped. No live provider,
credential inspection, commit or publication occurred.
