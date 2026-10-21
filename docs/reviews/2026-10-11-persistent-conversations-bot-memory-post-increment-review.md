# Persistent conversations and per-bot memory post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "persistent-conversations-bot-memory",
  "commands_executed": [
    "npm run verify",
    "./node_modules/.bin/vitest run src/features/agents/AgentsPage.test.tsx src/features/agents/SavedConversations.test.tsx src/infrastructure/tauri/conversation-history-client.test.ts",
    "npm run lint:frontend",
    "npm run typecheck",
    "./node_modules/.bin/prettier --check src/features/agents/AgentsPage.tsx src/features/agents/SavedConversations.tsx src/features/agents/AgentsPage.test.tsx",
    "./node_modules/.bin/vite build --outDir /private/tmp/cortexa-persistent-memory-hk2tezx_/frontend-review-dist-current",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "/usr/bin/python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-11-persistent-conversations-bot-memory.md",
    "docs/reviews/2026-10-11-persistent-conversations-bot-memory-post-increment-review.md",
    "docs/reviews/2026-10-11-persistent-conversations-bot-memory-readiness-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/agent_chat.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/startup.rs",
    "src-tauri/src/storage/agent_preferences.rs",
    "src-tauri/src/storage/conversations.rs",
    "src-tauri/src/storage/error.rs",
    "src-tauri/src/storage/migrations.rs",
    "src-tauri/src/storage/mod.rs",
    "src-tauri/src/storage/store.rs",
    "src-tauri/tests/startup_storage_smoke.rs",
    "src-tauri/tests/storage_smoke.rs",
    "src/features/agents/AgentsPage.test.tsx",
    "src/features/agents/AgentsPage.tsx",
    "src/features/agents/SavedConversations.test.tsx",
    "src/features/agents/SavedConversations.tsx",
    "src/infrastructure/tauri/agent-chat-client.ts",
    "src/infrastructure/tauri/conversation-history-client.test.ts",
    "src/infrastructure/tauri/conversation-history-client.ts"
  ],
  "milestone": {
    "criteria": {
      "conversation-persistence": {
        "status": "automatically_verified",
        "evidence": "Storage/native Simulation CRUD, bot separation, restart and timestamps; native/acceptance-results-002.json."
      },
      "terminal-recovery": {
        "status": "automatically_verified",
        "evidence": "Controlled storage/host failure/race/replay tests plus native Simulation active Stop and quit/restart; incomplete output preserved."
      },
      "bounded-context": {
        "status": "automatically_verified",
        "evidence": "agent_chat persisted_context tests and native included-turn display [2] excluding cancelled/interrupted turns; runtime compatibility enforced."
      },
      "explicit-memory": {
        "status": "automatically_verified",
        "evidence": "Existing notes extended with confirmed facts/provenance; native add/edit/off/clear and restart; immediate deletion refresh tested in frontend-006."
      },
      "privacy-isolation": {
        "status": "automatically_verified",
        "evidence": "Synthetic canaries, closed IPC/native context, fresh-bot UI separation; no credential read or logs; no runtime tools."
      },
      "migration": {
        "status": "automatically_verified",
        "evidence": "Migration10 fixture test preserves v9 note and migration history; transactions/recovery tests; no owner-data migration."
      },
      "runtime-evidence": {
        "status": "automatically_verified",
        "evidence": "API/Codex/Simulation separated; request ledger63/76 and no task live generations."
      },
      "verification-closeout": {
        "status": "automatically_verified",
        "evidence": "Reused verify-004 unchanged backend; frontend-006 and affected lint/typecheck/format/external build; final checks recorded externally."
      },
      "native-qa": {
        "status": "blocked",
        "evidence": "Required live Codex preference/isolation/Memory Off/edit/delete/cancel/recovery unverified; reviewed installed runtime unavailable. Final frontend delta needs native recheck. Simulation outcomes cannot waive this."
      }
    },
    "paths": {
      "ARCHITECTURE.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "CHANGELOG.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "HANDOFF.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "NEXT_STEPS.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "PLANS.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "PROJECT_STATUS.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "TESTING_GUIDE.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "TROUBLESHOOTING_LOG.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/plans/2026-10-11-persistent-conversations-bot-memory.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/reviews/2026-10-11-persistent-conversations-bot-memory-post-increment-review.md": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "scripts/repository_health.py": {
        "criterion": "privacy-isolation",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "scripts/tests/test_repository_health.py": {
        "criterion": "privacy-isolation",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/agent_chat.rs": {
        "criterion": "bounded-context",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/agent_chat_tauri.rs": {
        "criterion": "terminal-recovery",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/lib.rs": {
        "criterion": "privacy-isolation",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/startup.rs": {
        "criterion": "verification-closeout",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/storage/agent_preferences.rs": {
        "criterion": "conversation-persistence",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/storage/conversations.rs": {
        "criterion": "conversation-persistence",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/storage/error.rs": {
        "criterion": "conversation-persistence",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/storage/migrations.rs": {
        "criterion": "migration",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/storage/mod.rs": {
        "criterion": "conversation-persistence",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/storage/store.rs": {
        "criterion": "conversation-persistence",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/tests/startup_storage_smoke.rs": {
        "criterion": "migration",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/tests/storage_smoke.rs": {
        "criterion": "migration",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src/features/agents/AgentsPage.test.tsx": {
        "criterion": "explicit-memory",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src/features/agents/AgentsPage.tsx": {
        "criterion": "explicit-memory",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src/features/agents/SavedConversations.test.tsx": {
        "criterion": "explicit-memory",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src/features/agents/SavedConversations.tsx": {
        "criterion": "explicit-memory",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src/infrastructure/tauri/agent-chat-client.ts": {
        "criterion": "privacy-isolation",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src/infrastructure/tauri/conversation-history-client.test.ts": {
        "criterion": "privacy-isolation",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src/infrastructure/tauri/conversation-history-client.ts": {
        "criterion": "privacy-isolation",
        "rationale": "Implements, tests or documents the admitted persistent-conversation/private-memory objective; retains baseline behavior outside that boundary.",
        "within_objective": true,
        "preserves_existing": true
      }
    }
  },
  "verification": [
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "./node_modules/.bin/vitest run src/features/agents/AgentsPage.test.tsx src/features/agents/SavedConversations.test.tsx src/infrastructure/tauri/conversation-history-client.test.ts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run lint:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run typecheck",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "./node_modules/.bin/prettier --check src/features/agents/AgentsPage.tsx src/features/agents/SavedConversations.tsx src/features/agents/AgentsPage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "./node_modules/.bin/vite build --outDir /private/tmp/cortexa-persistent-memory-hk2tezx_/frontend-review-dist-current",
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
      "command": "/usr/bin/python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Architecture/security/code-health/debt review and preserved trust boundaries",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native Simulation persistence, explicit notes, active cancellation and interrupted restart with cleanup",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Live dedicated Codex preference/isolation/Memory Off/edit/delete/cancellation scenario",
      "required": true,
      "status": "Manual verification pending"
    },
    {
      "check": "Final frontend delta native recheck",
      "required": true,
      "status": "Manual verification pending"
    },
    {
      "check": "Direct API native acceptance with private credential",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Medium",
      "summary": "Reviewed Codex runtime prerequisite unavailable; required live acceptance remains unverified.",
      "risk": "Simulation and prior unrelated provider success cannot establish this milestone live memory behavior.",
      "effort": "Bounded protocol compatibility assessment, approved correction if needed, fresh binding and native scenario.",
      "milestone": "Persistent memory continuation",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Code health",
      "severity": "Low",
      "summary": "Final frontend guidance/provenance/loading delta has automated evidence but awaits native recheck.",
      "risk": "Current source differs in three UI/test files from retained native artifact; do not claim exact current bundle acceptance.",
      "effort": "Fresh incremental bundle and affected native UI recheck after access prerequisite.",
      "milestone": "Persistent memory continuation",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Inherited unsigned local build, platform-only manual evidence, provider retention and bounded synchronous JSON checkpoint costs retained.",
      "risk": "No distribution/signing assurance or universal runtime coverage; bounded whole-record writes may merit later measurement.",
      "effort": "Separate owner-selected follow-up only.",
      "milestone": "Existing advisories; D-125/M1/M2 parked",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked"
}
-->

Date: 2026-10-11
Increment: persistent-conversations-bot-memory
Branch: codex/persistent-bot-memory

## Executive summary

Implemented persistent bot advice and explicit private memory; mandatory live acceptance remains blocked. Quality FAIL, no completion marker. Native Simulation and automated results are useful partial evidence, not live-provider acceptance.

## Scope and boundaries

Owner-only task worktree codex/persistent-bot-memory at ac04c6e53bc3349496eba44be93e933f05e1519c. Ordinary schema-2 admission; no protected gate/skills/workflow/dependency changes. Preserve canonical/other tasks and historical failures. No credentials, personal database migration, commits or publication. Exact path attribution is in the manifest; counts informational.

## Verification results

Full npm run verify PASS is reused from verify-004.log/.exit with verification-inputs-004.json: 610 frontend tests, 467 Rust library tests, integration suites, 145 hook tests, 99 repository tests, strict Clippy, typecheck, formatting, frontend and release native build. After native QA, only three frontend/test files changed; frontend-006 passes 37 affected tests, plus refreshed frontend lint/typecheck/format/build. Evidence-reuse manifest proves Rust/IPC/migration/dependencies unchanged. This is stage reuse, not a newly rerun full verify. Native results/cleanup: native/acceptance-results-002.json. Final documentation/security/schema/preservation/status/Stop receipts are outside the repository. Prior failed verification receipts remain unchanged.

## Architecture findings

Native Rust owns conversation identity, current-profile reload, deterministic context, mutation and lifecycle. Existing SQLite/private-note owner extended, no new service/runtime/executor. Migration10 additive; stored history independent from <=3 completed turns /12288 characters in context. Context selection reserves4096 new-message characters; incomplete turns preserved but excluded. Runtime/model/endpoint/effort pinned; incompatible profile requires new conversation. No provider session identifiers persisted. Bounded whole-record JSON checkpoint writes remain a local demo-scale performance advisory; no speculative queue/coalescing added.

## Security findings

No capability/CSP/permission/network/credential changes. Tauri accepts bounded typed IDs/content, never arbitrary context/provider RPC/tools. SQL parameterized; transaction/revision/turn guards; terminal results immutable; storage failure blocks dispatch/replay. Existing notes untrusted and collaboration excludes private notes. Explicit source excerpt validation and UI confirmation before fact save. Credential stores, raw provider envelopes and internal reasoning not copied. Synthetic canary/privacy tests pass; manually typed secrets cannot be perfectly detected. No arbitrary-version Codex acceptance added to get a pass.

## Code-health findings

Resolved durable poll failure checkpoint, cross-bot component state, duplicate sibling keys, stale note guidance, delayed source-deletion refresh and false empty state while loading. Regression tests cover these meaningful paths. Controlled lifecycle tests and native Simulation preserve partial output and subsequent recovery. Final UI delta natively unverified remains explicit. Required live provider scenario not passed; no claim all functionality is ready for owner sign-off.

## Technical debt

Blocking prerequisites and advisories are enumerated in the manifest. Installed CLI reports0.162.0-alpha.17.2, outside reviewed0.159.0/0.160.1 identity guard. Fixed catalog setup error is observed; discarded handshake details/authentication state are unknown. Do not infer an expired account or weaken guard. Preserve existing provider retention/Stop billing, unsigned build, platform/coverage and native SDK27/Python/Cargo advisories; ECC hooks/MCP disabled; D-125/M1/M2 parked.

## Roadmap findings

Blocked for declaring this milestone complete or starting dependent product work. PR144 merged26447708934ef7f8118329d1b62977d7f14d45b6 contains governance/docs only, not a needed product dependency of the existing admission. No copied governance or private state. Existing shared ledger63/76, thirteen remaining, zero task provider generations; four Simulation generations separate.

## Completion decision

FAIL. Required live Codex acceptance and final UI native recheck remain pending. Never finalize this report as passing. Use ordinary close-failed and verify failed/valid plus full Stop; actual command outcomes are in the external FINAL-HANDOFF. Original failures and completion history remain intact.

## Next-increment readiness

Blocked. First assess supported reviewed runtime availability/protocol compatibility and exact continuation admission. Any change to the reviewed-version boundary needs concrete approval. Then bind current frontend/backend and complete the required dedicated-home scenario within13 remaining requests. API remains conditional on private credential access. No later milestone selected.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-11-persistent-conversations-bot-memory.md`
- `docs/reviews/2026-10-11-persistent-conversations-bot-memory-post-increment-review.md`
- `docs/reviews/2026-10-11-persistent-conversations-bot-memory-readiness-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/startup.rs`
- `src-tauri/src/storage/agent_preferences.rs`
- `src-tauri/src/storage/conversations.rs`
- `src-tauri/src/storage/error.rs`
- `src-tauri/src/storage/migrations.rs`
- `src-tauri/src/storage/mod.rs`
- `src-tauri/src/storage/store.rs`
- `src-tauri/tests/startup_storage_smoke.rs`
- `src-tauri/tests/storage_smoke.rs`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/features/agents/SavedConversations.test.tsx`
- `src/features/agents/SavedConversations.tsx`
- `src/infrastructure/tauri/agent-chat-client.ts`
- `src/infrastructure/tauri/conversation-history-client.test.ts`
- `src/infrastructure/tauri/conversation-history-client.ts`

## Exact commands executed

- `npm run verify` — see manifest and external receipt for result/reuse.
- `./node_modules/.bin/vitest run src/features/agents/AgentsPage.test.tsx src/features/agents/SavedConversations.test.tsx src/infrastructure/tauri/conversation-history-client.test.ts` — see manifest and external receipt for result/reuse.
- `npm run lint:frontend` — see manifest and external receipt for result/reuse.
- `npm run typecheck` — see manifest and external receipt for result/reuse.
- `./node_modules/.bin/prettier --check src/features/agents/AgentsPage.tsx src/features/agents/SavedConversations.tsx src/features/agents/AgentsPage.test.tsx` — see manifest and external receipt for result/reuse.
- `./node_modules/.bin/vite build --outDir /private/tmp/cortexa-persistent-memory-hk2tezx_/frontend-review-dist-current` — see manifest and external receipt for result/reuse.
- `npm run docs:check` — see manifest and external receipt for result/reuse.
- `npm run repository:check` — see manifest and external receipt for result/reuse.
- `npm run security:scan` — see manifest and external receipt for result/reuse.
- `git diff --check` — see manifest and external receipt for result/reuse.
- `/usr/bin/python3 -B .codex/hooks/session_end_gate.py` — see manifest and external receipt for result/reuse.

Installed-tooling process-local Python router, Xcode SDK27, offline external Cargo output, release build-override strip=none. Native debug build/helper checks are retained externally; no app currently running. Full logs never include credential contents.
