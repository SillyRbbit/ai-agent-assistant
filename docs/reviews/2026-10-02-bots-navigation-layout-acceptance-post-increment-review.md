# Bots navigation layout acceptance post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npm run lint:frontend",
    "npm run typecheck",
    "npm run build:frontend",
    "node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/browser --bots-only",
    "python3 /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/build-native.py",
    "npm run format:frontend",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/preservation.py",
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
    "docs/plans/2026-10-02-bots-navigation-layout-acceptance.md",
    "docs/plans/2026-10-02-bots-navigation-layout.md",
    "docs/plans/2026-10-02-local-diagnostics.md",
    "docs/plans/2026-10-02-openai-stream-error-diagnostic.md",
    "docs/plans/2026-10-02-openai-stream-error-shape-diagnostic.md",
    "docs/plans/2026-10-02-provider-troubleshooting.md",
    "docs/reviews/2026-10-02-animated-bot-identity-post-increment-review.md",
    "docs/reviews/2026-10-02-animated-conductor-post-increment-review.md",
    "docs/reviews/2026-10-02-bots-navigation-layout-acceptance-post-increment-review.md",
    "docs/reviews/2026-10-02-bots-navigation-layout-post-increment-review.md",
    "docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md",
    "docs/reviews/2026-10-02-provider-troubleshooting-post-increment-review.md",
    "scripts/browser/diagnostics-check.mjs",
    "scripts/browser/diagnostics-fixture.html",
    "scripts/browser/knowledge-check.mjs",
    "scripts/browser/knowledge-fixture.tsx",
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
      "summary": "Real preview-label overflow at 961px blocks no-horizontal-overflow acceptance.",
      "risk": "Long bot labels beside the expressive mascot can be clipped in constrained settings.",
      "effort": "Bounded preview CSS correction and affected browser/native acceptance.",
      "milestone": "Separately authorized Bots preview layout correction.",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "All inherited live, native/runtime, Codex-isolation, asset/chunk, initial-graph and owner-QA advisories retained.",
      "risk": "No live-provider or new native layout assurance.",
      "effort": "Separate bounded QA.",
      "milestone": "Owner QA; D-125/M1/M2 parked.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "increment_id": "bots-navigation-layout-acceptance",
  "manual_verification": [
    {
      "check": "Supported Computer Use wide/compact Bots acceptance and test-app cleanup",
      "required": true,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
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
      "command": "node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/browser --bots-only",
      "required": true,
      "status": "Failed"
    },
    {
      "command": "python3 /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/build-native.py",
      "required": true,
      "status": "Passed"
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
      "command": "python3 -B /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/preservation.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-10-02
Branch: codex/bots-navigation-layout-acceptance

## Executive summary

**FAIL — Blocked.** The missing fixture response is fixed without adding execution authority. Required layout acceptance exposed a real preview overflow. Product files remain frozen; no new native QA occurred.

## Scope and boundaries

Exactly eleven successor paths and 96 cumulative paths. 93 source paths transferred byte-identically before ordinary begin. All product bytes, including the existing two-selector CSS repair, remain unchanged. The source raw FAIL, reports, checkouts, profiles, notes, rooms and previous artifacts remain preserved. Only fixture/checker plus additive current-state docs and this new plan/review changed.

## Verification results

Fixed six-connection readiness, denial of nine mutation/discovery/execution/unknown commands and profile preservation assertions passed before the first expanded layout case. Original fixture responses remain; no handler for saves, Sends or execution was added. Browser attempt 1 exposed an exact-label select locator mismatch; accessible combobox locators fixed it. Attempts 2-6 exposed and progressively identified persistent overflow at 961px after inspector close: mainLeft=sideRight=220, contentScroll=743, contentWidth=726, documentWidth=windowWidth=961. Offender: .bot-preview > span text workflow-automation, right=962.953125. Expanded 1600px passed; the complete compact/collapsed/reverse matrix did not. See browser-1 through browser-6 logs, screenshots and command receipts. Static frontend checks passed. An initial tooling symlink appeared as a 97th untracked path; only that task-created link was replaced with an ignored directory reusing installed entries, restoring the exact 96-path inventory without product changes. Final lint passed after the checker-only diagnostic edits; its receipt is external. Offline native build passed in 66.9 seconds, SDK27 verified, artifact.json hashes frozen. Computer Use inventory was available; no app launch followed the scope stop. Prior 67 frontend tests and Rust/animation/workflow evidence are inherited, unchanged and not relabeled new. Closeout checks use actual statuses; schema commands list includes Not run requirements until verified.

## Architecture findings

Fixed read-only browser mock response uses existing public Tauri invocation boundary; no production IPC, runtime or authority changes. No generic fallback. Existing route separation fix works in the observed 961px geometry, but separate preview row sizing fails. No product redesign or workaround attempted.

## Security findings

Synthetic boundary invokes only inside the existing mock browser. Explicit regression confirms save_agent_preferences, clear_agent_note, restore_agent_defaults, discover_agent_models, start_agent_conversation, send_agent_message, start_collaboration, cancel_collaboration and an unknown command reject; profiles remain equal. Existing no-native/provider fixture restriction retained. No credentials read, owner data used, provider requests or profile changes. Native launch not performed.

## Code-health findings

No arbitrary timeout or delayed readiness mock. Roster waits for actual ninth choice. Exact assertions remain: geometry/hit tests, nine profiles, settings and scrolling, overflow and inspector close/Escape. Geometry diagnostics retain only synthetic fixture labels. No name shortening or overflow masking. The fixed-size/nonshrinking large mascot beside a label in nonwrapping .bot-preview requires product-scope assessment.

## Technical debt

Medium blocking preview overflow; small bounded CSS investigation required. No acceptance of the untested remainder. Inherited D-127/D-128, native/provider/runtime/Codex-isolation, process-local Python3.12.1/Xcode SDK27/Cargo strip-none, asset/chunk and graph/owner-aesthetic advisories retained. Live allowance 3/10 used; D-125/M1/M2 parked.

## Roadmap findings

No publication or further implementation. Stop for explicit product-layout scope/admission decision. Source and acceptance terminal histories must remain immutable; no automatic chain or governance change.

## Completion decision

**FAIL.** Record ordinary close-failed after schema and preservation validate. Full Stop validates terminal disposition only; it cannot turn this failed acceptance into completion.

## Next-increment readiness

**Blocked.** Smallest next work is read-only bounded preview sizing/admission review or an explicitly authorized product correction with supported admission. No scope expansion is inferred from this report.

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
- `docs/plans/2026-10-02-bots-navigation-layout-acceptance.md`
- `docs/plans/2026-10-02-bots-navigation-layout.md`
- `docs/plans/2026-10-02-local-diagnostics.md`
- `docs/plans/2026-10-02-openai-stream-error-diagnostic.md`
- `docs/plans/2026-10-02-openai-stream-error-shape-diagnostic.md`
- `docs/plans/2026-10-02-provider-troubleshooting.md`
- `docs/reviews/2026-10-02-animated-bot-identity-post-increment-review.md`
- `docs/reviews/2026-10-02-animated-conductor-post-increment-review.md`
- `docs/reviews/2026-10-02-bots-navigation-layout-acceptance-post-increment-review.md`
- `docs/reviews/2026-10-02-bots-navigation-layout-post-increment-review.md`
- `docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md`
- `docs/reviews/2026-10-02-provider-troubleshooting-post-increment-review.md`
- `scripts/browser/diagnostics-check.mjs`
- `scripts/browser/diagnostics-fixture.html`
- `scripts/browser/knowledge-check.mjs`
- `scripts/browser/knowledge-fixture.tsx`
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

- **Passed** — `npm run lint:frontend`
- **Passed** — `npm run typecheck`
- **Passed** — `npm run build:frontend`
- **Failed** — `node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/browser --bots-only`
- **Passed** — `python3 /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/build-native.py`
- **Passed** — `npm run format:frontend`
- **Passed** — `npm run docs:check`
- **Passed** — `npm run repository:check`
- **Passed** — `npm run security:scan`
- **Passed** — `git diff --check`
- **Passed** — `python3 -B /private/tmp/cortexa-bots-navigation-layout-acceptance-evidence/preservation.py`
- **Passed** — `python3 -B .codex/hooks/session_end_gate.py`

Live main read verified required SHA; branch creation, frozen transfer and ordinary begin succeeded. Task-owned Vite was stopped. No tests, builds or workflows from unchanged evidence repeated; one authorized native bundle was built. Native QA not run.
