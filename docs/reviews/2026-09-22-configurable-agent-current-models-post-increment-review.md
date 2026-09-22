# Configurable agent current-model correction post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "node_modules/.bin/vitest run src/infrastructure/tauri/agent-chat-client.test.ts src/features/agents/AgentsPage.test.tsx",
    "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked storage::migrations::tests",
    "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked --lib",
    "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked --test storage_smoke --test startup_storage_smoke",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
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
    "docs/plans/2026-09-22-configurable-agent-current-models.md",
    "docs/plans/2026-09-22-configurable-agent-demo.md",
    "docs/reviews/2026-09-22-configurable-agent-current-models-post-increment-review.md",
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
      "category": "Security",
      "effort": "Small",
      "milestone": "Any separately authorized future live verification",
      "risk": "The current models are offline-verified, while the consumed owner request ended before live success and no request allowance remains.",
      "severity": "Advisory",
      "summary": "Native live success remains unverified and zero live attempts remain authorized."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Existing dependency and audit maintenance",
      "risk": "The accepted D-127 audit baseline still requires maintenance as advisories and dependencies change.",
      "severity": "Advisory",
      "summary": "Pre-existing D-127 audit debt remains unchanged."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Unknown",
      "milestone": "Owner-selected future work",
      "risk": "Resuming historical harness work would displace the completed demo without current owner direction.",
      "severity": "Advisory",
      "summary": "D-125/M1/M2 remain parked."
    }
  ],
  "increment_id": "configurable-agent-current-models",
  "manual_verification": [
    {
      "check": "Native live success with a current selectable model",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "node_modules/.bin/vitest run src/infrastructure/tauri/agent-chat-client.test.ts src/features/agents/AgentsPage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked storage::migrations::tests",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked --lib",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "CARGO_NET_OFFLINE=true cargo test --manifest-path src-tauri/Cargo.toml --locked --test storage_smoke --test startup_storage_smoke",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
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
Increment: `configurable-agent-current-models`
Branch: `codex/direct-provider-stream-stage-diagnostics`

## Executive summary

The selectable OpenAI catalog now contains only GPT-5.6 Luna, Terra and Sol,
with Luna as default. Stored 5.4 OpenAI profiles migrate safely to Luna. All
required checks passed. Quality result: `PASS WITH ADVISORIES`.

## Scope and boundaries

The exact successor scope is 21 paths and the cumulative candidate is 38 paths.
No dependencies, endpoint, transport, parser, retry, fallback, authority,
permission, workflow, hook, skill or harness changed. No live request was made.

## Verification results

Focused checks passed 16 frontend tests, six migration tests, 337 Rust library
tests and both storage integrations. Final offline `npm run verify` passed 74
hook tests, 85 repository tests, 412 frontend tests, 337 Rust library tests, the
full integration suite, lint, type checking, frontend builds and release Tauri
compilation. Documentation, repository, security, whitespace and session checks
passed. No native launch or live request was performed.

## Architecture findings

PASS. TypeScript and Rust share one closed catalog; request assembly remains
native-owned. Migration 4 is append-only and changes only matching OpenAI rows.
No new coupling, dependency, network behavior or authority boundary was added.

## Security findings

PASS WITH ADVISORY. The migration preserves notes/instructions and fails no
valid maximum-revision row; stale 5.4 captures remain unusable. No credential,
request, environment, log, permission or endpoint behavior changed. Native live
success remains unverified and zero attempts remain authorized.

## Code-health findings

PASS. Closed-catalog tests reject removed 5.4 models, exercise every offered
model/effort request, and cover migration preservation, simulation exclusion,
revision invalidation and signed-maximum saturation. One initial focused test
compile failure was corrected by using its proper boxed test error type. Review
then found and corrected the migration overflow edge before the final full pass.

## Technical debt

D-127 remains an Advisory, Medium-effort audit-maintenance item; it blocks
neither completion nor the next owner-selected increment. No new debt was added.

## Roadmap findings

Ready with advisories. The current-model correction is complete. D-125/M1/M2
remain parked, and no successor starts without owner direction.

## Completion decision

`PASS WITH ADVISORIES`. Every required automated check passed. Optional current-
model live verification was not run because the allowance is exhausted.

## Next-increment readiness

`Ready with advisories`. No automatic next task is required. A future live
verification would need a separately authorized request allowance.

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
- `docs/plans/2026-09-22-configurable-agent-current-models.md`
- `docs/plans/2026-09-22-configurable-agent-demo.md`
- `docs/reviews/2026-09-22-configurable-agent-current-models-post-increment-review.md`
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

Passing commands are recorded verbatim in the machine manifest. The earlier
focused Rust compile failure is retained above as truthful recoverable evidence;
the corrected command and final complete offline verification passed.
