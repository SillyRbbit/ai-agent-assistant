# Bots navigation layout post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "bots-navigation-layout",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "commands_executed": [
    "npm run test:frontend -- src/features/agents/AgentsPage.test.tsx src/features/agents/BotAppearance.test.tsx src/App.test.tsx",
    "npm run lint:frontend",
    "npm run typecheck",
    "npm run build:frontend",
    "node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-bots-navigation-layout-evidence/browser --bots-only",
    "npm run format:frontend",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-bots-navigation-layout-evidence/preservation.py",
    "python3 -B .codex/hooks/session_end_gate.py"
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
    "assets/mascots/README.md",
    "assets/mascots/animation-source.json",
    "assets/mascots/cloud-infrastructure.png",
    "assets/mascots/cloud-infrastructure.webp",
    "assets/mascots/coding.png",
    "assets/mascots/coding.webp",
    "assets/mascots/concept.png",
    "assets/mascots/conductor/README.md",
    "assets/mascots/conductor/animation.json",
    "assets/mascots/conductor/atlas.png",
    "assets/mascots/conductor/atlas.webp",
    "assets/mascots/conductor/original.png",
    "assets/mascots/knowledge-document.png",
    "assets/mascots/knowledge-document.webp",
    "assets/mascots/personal-assistant.png",
    "assets/mascots/personal-assistant.webp",
    "assets/mascots/qa-validation.png",
    "assets/mascots/qa-validation.webp",
    "assets/mascots/research.png",
    "assets/mascots/research.webp",
    "assets/mascots/security-risk.png",
    "assets/mascots/security-risk.webp",
    "assets/mascots/systems-operations.png",
    "assets/mascots/systems-operations.webp",
    "assets/mascots/workflow-automation.png",
    "assets/mascots/workflow-automation.webp",
    "docs/plans/2026-10-02-animated-bot-identity.md",
    "docs/plans/2026-10-02-animated-conductor.md",
    "docs/plans/2026-10-02-bots-navigation-layout.md",
    "docs/plans/2026-10-02-local-diagnostics.md",
    "docs/plans/2026-10-02-openai-stream-error-diagnostic.md",
    "docs/plans/2026-10-02-openai-stream-error-shape-diagnostic.md",
    "docs/plans/2026-10-02-provider-troubleshooting.md",
    "docs/reviews/2026-10-02-animated-bot-identity-post-increment-review.md",
    "docs/reviews/2026-10-02-animated-conductor-post-increment-review.md",
    "docs/reviews/2026-10-02-bots-navigation-layout-post-increment-review.md",
    "docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md",
    "docs/reviews/2026-10-02-provider-troubleshooting-post-increment-review.md",
    "scripts/browser/diagnostics-check.mjs",
    "scripts/browser/diagnostics-fixture.html",
    "scripts/browser/knowledge-check.mjs",
    "scripts/browser/mascot-fixture.html",
    "scripts/browser/mascot-fixture.tsx",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/agent_preferences.rs",
    "src-tauri/src/anthropic.rs",
    "src-tauri/src/codex_connection.rs",
    "src-tauri/src/collaboration_tauri.rs",
    "src-tauri/src/diagnostics.rs",
    "src-tauri/src/diagnostics/tests.rs",
    "src-tauri/src/diagnostics_tauri.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/local_models.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/personal_assistant_direct_tauri.rs",
    "src-tauri/src/startup.rs",
    "src/features/agents/AgentsPage.test.tsx",
    "src/features/agents/AgentsPage.tsx",
    "src/features/agents/BotAppearance.test.tsx",
    "src/features/agents/BotAppearance.tsx",
    "src/features/agents/BotMotion.test.tsx",
    "src/features/agents/ConductorIdentity.test.tsx",
    "src/features/agents/ConductorIdentity.tsx",
    "src/features/agents/bot-mascots.css",
    "src/features/agents/botMascots.ts",
    "src/features/agents/botMotion.ts",
    "src/features/collaboration/CollaborationPage.test.tsx",
    "src/features/collaboration/CollaborationPage.tsx",
    "src/features/command-center/CollaborationTopology.tsx",
    "src/features/command-center/OperationalCommandCenterPage.tsx",
    "src/features/settings/DiagnosticsPanel.test.tsx",
    "src/features/settings/DiagnosticsPanel.tsx",
    "src/features/settings/SettingsPage.tsx",
    "src/features/settings/diagnosticSummary.test.ts",
    "src/features/settings/diagnosticSummary.ts",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts",
    "src/infrastructure/tauri/diagnostics-client.test.ts",
    "src/infrastructure/tauri/diagnostics-client.ts",
    "src/styles.css",
    "tsconfig.app.json"
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Medium",
      "summary": "Required Bots browser acceptance is blocked by the unchanged fixture rejecting connection readiness.",
      "risk": "Nine-bot controls and compact geometry have not been accepted.",
      "effort": "Small read-only fixture extension plus full affected browser/native acceptance.",
      "milestone": "Bots navigation layout acceptance after explicit scope/admission decision.",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "All inherited advisories and parked work remain.",
      "risk": "No new live-provider, initial graph, release or owner-aesthetic assurance.",
      "effort": "Separately authorized bounded QA.",
      "milestone": "Owner QA; D-125/M1/M2 remain parked.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "verification": [
    {
      "command": "npm run test:frontend -- src/features/agents/AgentsPage.test.tsx src/features/agents/BotAppearance.test.tsx src/App.test.tsx",
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
      "command": "npm run build:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-bots-navigation-layout-evidence/browser --bots-only",
      "required": true,
      "status": "Failed"
    },
    {
      "command": "npm run format:frontend",
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
      "command": "python3 -B /private/tmp/cortexa-bots-navigation-layout-evidence/preservation.py",
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
      "check": "New isolated offline unsigned native bundle and identity/preservation verification",
      "required": true,
      "status": "Not run"
    },
    {
      "check": "Computer Use wide/compact Bots, navigation states, artwork/roles, settings, scrolling and route transitions",
      "required": true,
      "status": "Manual verification pending"
    }
  ]
}
-->

Date: 2026-10-02
Increment: bots-navigation-layout
Branch: codex/local-diagnostics

## Executive summary

**FAIL — Blocked.** The two-selector CSS repair and browser matrix are implemented. Acceptance is incomplete; no completion marker is justified. Stop condition: necessary fixture scope expansion.

## Scope and boundaries

Exactly eleven successor paths and 93 cumulative paths. CSS only adds Bots to both existing route-specific compact navigation-width selectors (and updates its comment). All previous fixture assertions remain; --bots-only isolates the new matrix. No fixture, avatar, profile, animation, graph, inspector design, provider, Rust, permission, dependency or governance edit. No owner data or workflows touched.

## Verification results

67 tests across AgentsPage, BotAppearance and App passed. Lint, typecheck and frontend build passed. The new actual-App browser check failed at its first roster count: 0 versus 9. No complete viewport matrix, bundle or native QA is claimed. AgentsPage.tsx:62 waits for list plus connections; agent-chat-client.ts:441 invokes list_agent_connections; knowledge-fixture.tsx:251-285 has no response for it and rejects unsupported IPC. This conclusively prevents roster assignment even if count timing were adjusted. The fixture must gain a narrow read-only response before this acceptance can work; no generic fallback is proposed. Historical automated/native evidence remains inherited and unchanged. Validation statuses in the manifest distinguish executed commands from Not run requirements; the schema commands list also contains those declared pending requirements.

## Architecture findings

Source review: the product correction remains a route-scoped CSS variable override; no runtime or ownership change. Existing inspector overlay is preserved. The browser matrix exercises the real App, but its fixture lacks a required read-only endpoint. No architectural workaround or product substitution was attempted.

## Security findings

No new IPC, network, credentials, profile persistence, execution or authority. Existing fixture deny-by-default behavior is preserved. Any future extension must provide fixed synthetic readiness only, keep non-Simulation connections blocked and continue rejecting save/discovery/send/execution. No live calls were made.

## Code-health findings

The failing test is retained truthfully. Do not weaken roster assertions or bypass production profile loading. No animation or workflow suite was repeated. New geometry, scrolling and inspector assertions remain unaccepted until the fixture is supported.

## Technical debt

Medium blocking acceptance gap: fixture connection readiness missing. Small explicit fixture extension and browser/native validation required. Inherited asset weight/chunk warnings, initial roster graph, owner-aesthetic, D-127/D-128, native/provider/runtime, Codex-isolation and process-local Python/Xcode SDK27/Cargo strip-none advisories remain. Live QA parked 3/10 used; D-125/M1/M2 parked.

## Roadmap findings

Stop at scope boundary. No new milestone, publication or automatic recovery chain. Obtain an explicit bounded fixture/admission decision preserving terminal history before further implementation. Other inherited roadmap items are unchanged.

## Completion decision

**FAIL.** Use ordinary close-failed only after this truthful report and preservation validate. Do not finalize or promote the record. A passing full Stop on a terminal failure means valid disposition, not acceptance.

## Next-increment readiness

**Blocked.** Smallest next decision: allow scripts/browser/knowledge-fixture.tsx to return only a fixed valid six-connection synthetic readiness list; preserve execution restrictions. An explicit supported admission route must preserve this FAIL. Native build/QA and remaining browser acceptance are still required.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `assets/mascots/README.md`
- `assets/mascots/animation-source.json`
- `assets/mascots/cloud-infrastructure.png`
- `assets/mascots/cloud-infrastructure.webp`
- `assets/mascots/coding.png`
- `assets/mascots/coding.webp`
- `assets/mascots/concept.png`
- `assets/mascots/conductor/README.md`
- `assets/mascots/conductor/animation.json`
- `assets/mascots/conductor/atlas.png`
- `assets/mascots/conductor/atlas.webp`
- `assets/mascots/conductor/original.png`
- `assets/mascots/knowledge-document.png`
- `assets/mascots/knowledge-document.webp`
- `assets/mascots/personal-assistant.png`
- `assets/mascots/personal-assistant.webp`
- `assets/mascots/qa-validation.png`
- `assets/mascots/qa-validation.webp`
- `assets/mascots/research.png`
- `assets/mascots/research.webp`
- `assets/mascots/security-risk.png`
- `assets/mascots/security-risk.webp`
- `assets/mascots/systems-operations.png`
- `assets/mascots/systems-operations.webp`
- `assets/mascots/workflow-automation.png`
- `assets/mascots/workflow-automation.webp`
- `docs/plans/2026-10-02-animated-bot-identity.md`
- `docs/plans/2026-10-02-animated-conductor.md`
- `docs/plans/2026-10-02-bots-navigation-layout.md`
- `docs/plans/2026-10-02-local-diagnostics.md`
- `docs/plans/2026-10-02-openai-stream-error-diagnostic.md`
- `docs/plans/2026-10-02-openai-stream-error-shape-diagnostic.md`
- `docs/plans/2026-10-02-provider-troubleshooting.md`
- `docs/reviews/2026-10-02-animated-bot-identity-post-increment-review.md`
- `docs/reviews/2026-10-02-animated-conductor-post-increment-review.md`
- `docs/reviews/2026-10-02-bots-navigation-layout-post-increment-review.md`
- `docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md`
- `docs/reviews/2026-10-02-provider-troubleshooting-post-increment-review.md`
- `scripts/browser/diagnostics-check.mjs`
- `scripts/browser/diagnostics-fixture.html`
- `scripts/browser/knowledge-check.mjs`
- `scripts/browser/mascot-fixture.html`
- `scripts/browser/mascot-fixture.tsx`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/anthropic.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/collaboration_tauri.rs`
- `src-tauri/src/diagnostics.rs`
- `src-tauri/src/diagnostics/tests.rs`
- `src-tauri/src/diagnostics_tauri.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/local_models.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/personal_assistant_direct_tauri.rs`
- `src-tauri/src/startup.rs`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/features/agents/BotAppearance.test.tsx`
- `src/features/agents/BotAppearance.tsx`
- `src/features/agents/BotMotion.test.tsx`
- `src/features/agents/ConductorIdentity.test.tsx`
- `src/features/agents/ConductorIdentity.tsx`
- `src/features/agents/bot-mascots.css`
- `src/features/agents/botMascots.ts`
- `src/features/agents/botMotion.ts`
- `src/features/collaboration/CollaborationPage.test.tsx`
- `src/features/collaboration/CollaborationPage.tsx`
- `src/features/command-center/CollaborationTopology.tsx`
- `src/features/command-center/OperationalCommandCenterPage.tsx`
- `src/features/settings/DiagnosticsPanel.test.tsx`
- `src/features/settings/DiagnosticsPanel.tsx`
- `src/features/settings/SettingsPage.tsx`
- `src/features/settings/diagnosticSummary.test.ts`
- `src/features/settings/diagnosticSummary.ts`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`
- `src/infrastructure/tauri/diagnostics-client.test.ts`
- `src/infrastructure/tauri/diagnostics-client.ts`
- `src/styles.css`
- `tsconfig.app.json`

## Exact commands executed

- **Passed** — `npm run test:frontend -- src/features/agents/AgentsPage.test.tsx src/features/agents/BotAppearance.test.tsx src/App.test.tsx`
- **Passed** — `npm run lint:frontend`
- **Passed** — `npm run typecheck`
- **Passed** — `npm run build:frontend`
- **Failed** — `node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-bots-navigation-layout-evidence/browser --bots-only`
- **Passed** — `npm run format:frontend`
- **Passed** — `npm run docs:check`
- **Passed** — `npm run repository:check`
- **Passed** — `npm run security:scan`
- **Passed** — `git diff --check`
- **Passed** — `python3 -B /private/tmp/cortexa-bots-navigation-layout-evidence/preservation.py`
- **Passed** — `python3 -B .codex/hooks/session_end_gate.py`

Ordinary begin succeeded. Local Vite ran on 127.0.0.1:4175 and was stopped. New native build and native Computer Use: Not run. No installs, live requests, commits or publication.
