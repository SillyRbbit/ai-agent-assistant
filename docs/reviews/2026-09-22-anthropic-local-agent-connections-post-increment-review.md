# Anthropic and local agent connections post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY -u CORTEXA_LM_STUDIO_TOKEN -u CORTEXA_OLLAMA_TOKEN CARGO_NET_OFFLINE=true npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py",
    "python3 -"
  ],
  "files_changed": [
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
    "docs/plans/2026-09-22-anthropic-local-agent-connections.md",
    "docs/reviews/2026-09-22-anthropic-local-agent-connections-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/agent_chat.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/agent_models.rs",
    "src-tauri/src/agent_preferences.rs",
    "src-tauri/src/anthropic.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/local_models.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/startup.rs",
    "src-tauri/src/storage/agent_preferences.rs",
    "src-tauri/src/storage/migrations.rs",
    "src-tauri/src/storage/store.rs",
    "src-tauri/tests/startup_storage_smoke.rs",
    "src-tauri/tests/storage_smoke.rs",
    "src/features/agents/AgentsPage.test.tsx",
    "src/features/agents/AgentsPage.tsx",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "One separately owner-authorized smoke per configured connection; account/runtime prerequisites vary.",
      "milestone": "Optional owner smoke after offline implementation review.",
      "risk": "Fixtures and release builds do not establish account access, installed-server compatibility, hardware capacity or native GUI/live success. Codex isolation remains unverified; D-127 audit debt and D-128 custody/remote-abort limitations remain.",
      "severity": "Advisory",
      "summary": "Owner retains setup and operational verification responsibility; no live request allowance was granted or consumed."
    }
  ],
  "increment_id": "anthropic-local-agent-connections",
  "manual_verification": [
    {
      "check": "Owner-triggered Anthropic discovery and native smoke with separate paid authorization",
      "required": false,
      "status": "Manual verification pending"
    },
    {
      "check": "Owner-triggered already-installed LM Studio/Ollama discovery and native smoke",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY -u CORTEXA_LM_STUDIO_TOKEN -u CORTEXA_OLLAMA_TOKEN CARGO_NET_OFFLINE=true npm run verify",
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
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-09-22
Increment: anthropic-local-agent-connections
Branch: codex/configurable-agent-demo-publication (no commit/publication)

## Executive summary

Implemented and offline-verified the Anthropic Messages adapter, shared LM Studio/Ollama adapter, explicit model discovery and existing Agents-page configuration. Final quality result: PASS WITH ADVISORIES; readiness: Ready with advisories. No provider/runtime/model request was executed, no live allowance was granted, and native GUI/live operation remains unverified. The ordinary completion marker and Stop result are checked after this report is frozen.

## Scope and boundaries

Exactly 32 paths specified in the active plan. Existing Agents page, shared native ownership and bounded per-agent history reused. One Anthropic Messages adapter and one shared local Chat Completions adapter. No new dependency, tools, runtime management, browser networking, permission, CSP, workflow, hook, gate behavior, graph or publication change. Original dirty checkout and other worktrees were not edited. Both prunable entries remain.

## Verification results

Focused frontend 29, agent/profile 32, Anthropic 9, local adapter 11 and migration 8 checks passed. The separately rerun full native library suite passed 368. Final full offline verify exited 0 and passed formatting, repository health, strict frontend/Rust lint, 74 hook tests, 85 repository tests, 431 frontend tests, 368 Rust library tests, integration tests (one pre-existing ignored), frontend build and native release build. Documentation, repository, secret scanning, whitespace, exact 32-path scope/preservation and session checks passed. Independent architecture/security/code-health/debt/readiness review found no remaining blocker. Final document-only updates are rechecked before finalization; no application source changed after full verification.

Historical failed attempts: omitted profile draft fields and stale connection count (repaired once); three frontend lint brace/control-character errors (repaired once); checker lexical false positive on ordinary callback (renamed callback); stale test IPC fixture (updated exact command); stale startup migration count and incorrect synthetic sequence expectation (corrected without host production changes). Every correction stayed in this task; no gate was weakened and no terminal successor was created.

## Architecture findings

Independent reviews found no remaining source blocker after corrections. Native authority owns selected agent, revision, endpoint, catalog/capability validation, request construction and cancellation. Provider-specific payloads/parsers remain separate; model metadata cannot grant tools. Anthropic text-only followup omits thinking under official guidance. No opaque state crosses providers.

## Security findings

Independent review verified pinned Anthropic HTTPS, native-only opt-in key handling, loopback parsing without DNS/proxy/redirect/retry, separate endpoint-bound local tokens and no raw error echo. Capability validation precedes credential reads. Explicit refresh invalidates stale local authorization; cloud metadata rejects aliases; enabled notes with unknown locality need explicit destination/model consent. Separate reasoning fields are hidden; local content follows runtime framing. No credentials or live process environments were inspected.

## Code-health findings

Independent review identified and resolved catalog loss after Save, known unsupported Start, metadata contract mismatches, stale refresh authorization, terminal whitespace and partial-output framing. Offline fixtures cover these behaviors, memory isolation, cancellation, migration rollback and unchanged OpenAI/Codex boundaries. Full offline frontend and native release builds passed.

## Technical debt

Advisory: account-specific access, local runtime compatibility and native GUI/live success are unverified. Catalogs are bounded to 200 models; slow scans can timeout. Old servers lacking text capability metadata remain unavailable. OOM classification recognizes closed structured codes; free-form messages stay generic. Locality is unknown even at loopback and owner consent does not prove local execution. D-127 accepted audit debt and D-128 transient credential/abort limits remain. Codex live remains blocked on independently verified isolation. These do not authorize operational requests.

## Roadmap findings

Keep D-125/M1/M2 parked. No old live allowance is reused or renewed. Next meaningful work is an owner-directed review or optional, separately acknowledged provider smoke using an existing account/runtime. No downloads, fallback, retry or new architecture phase.

## Completion decision

PASS WITH ADVISORIES. Required automated verification passed; optional native/account/runtime smoke remains pending under owner authority. Ordinary report validation and finalization may proceed after the final document recheck. No commit, publication, live request, runtime installation or model acquisition is authorized.

## Next-increment readiness

Ready with advisories for an owner-directed read-only review or separately authorized optional smoke. No additional implementation or diagnostics successor is required for this amendment. Optional operational smoke still requires private owner setup and explicit request authorization; existing Codex, D-127/D-128 and D-125/M1/M2 boundaries remain.

## Exact files changed

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
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_models.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/anthropic.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/local_models.rs`
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

## Exact commands executed

- `env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY -u CORTEXA_LM_STUDIO_TOKEN -u CORTEXA_OLLAMA_TOKEN CARGO_NET_OFFLINE=true npm run verify`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 -B .codex/hooks/session_end_gate.py`
- `python3 -`

The read-only `python3 -` assertions checked baseline HEAD, planned path inventory, no conflicts, exact historical root-document body suffixes, migration SQL 1–4 identity, protected files and both prunable registry entries. This is an inline check, not a new repository harness. Provider credentials were removed for native full verification; Cargo remained offline. Build logs are local temporary outputs, not repository files.
