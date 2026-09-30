# Bot Identity publication preparation — post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "bot-identity-publication-preparation",
  "commands_executed": [
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run test:frontend -- src/features/command-center/commandCenterBotNames.test.ts",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run test:frontend -- src/features/command-center/commandCenterBotNames.test.ts src/features/command-center/CommandCenterPage.test.tsx",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run lint:frontend",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run typecheck",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run format:frontend",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run build:frontend",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run docs:check",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run repository:check",
    "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-bot-identity-publication-preparation-evidence/check-preservation.py",
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
    "docs/plans/2026-09-30-bot-identity-publication-preparation.md",
    "docs/plans/2026-09-30-command-center-measurement-readiness.md",
    "docs/plans/2026-09-30-command-center-startup-readiness.md",
    "docs/plans/2026-09-30-conductor-display-name.md",
    "docs/reviews/2026-09-30-bot-graph-name-sync-post-increment-review.md",
    "docs/reviews/2026-09-30-bot-identity-personality-post-increment-review.md",
    "docs/reviews/2026-09-30-bot-identity-publication-preparation-post-increment-review.md",
    "docs/reviews/2026-09-30-command-center-measurement-readiness-post-increment-review.md",
    "docs/reviews/2026-09-30-command-center-startup-readiness-post-increment-review.md",
    "docs/reviews/2026-09-30-conductor-display-name-post-increment-review.md",
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
    "src/features/command-center/commandCenterFixtures.ts",
    "src/features/command-center/commandCenterProjection.test.ts",
    "src/features/command-center/components/OperationalTopologyAdapter.test.tsx",
    "src/features/command-center/components/OperationalTopologyAdapter.tsx",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts"
  ],
  "verification": [
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run test:frontend -- src/features/command-center/commandCenterBotNames.test.ts",
      "required": false,
      "status": "Failed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run test:frontend -- src/features/command-center/commandCenterBotNames.test.ts src/features/command-center/CommandCenterPage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run lint:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run typecheck",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run format:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run build:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-bot-identity-publication-preparation-evidence/check-preservation.py",
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
      "check": "Separate read-only architecture/security/code-health/readiness review of the exact delta, inherited passing evidence, native QA attribution and preservation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact-head remote CI/review and live personality/native Unicode observation",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Live personality behavior and exact-head remote CI remain unverified; preserved source native QA does not observe new Unicode names",
      "risk": "Deterministic construction/measurement and local acceptance do not prove model obedience, provider access or another CI host",
      "effort": "Separate owner-authorized publication/CI review and optional later native/live QA",
      "milestone": "Bot Identity publication and owner QA",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories"
}
-->

Date: 2026-09-30
Increment: bot-identity-publication-preparation
Branch: codex/bot-identity-publication

## Executive summary

Prepared the accepted Bot Identity/personality and graph presentation candidate
from actual main ancestry. Transferred 45 paths byte/mode-identically; added one
Unicode counting correction, six regressions and preparation documentation.
Local result: PASS WITH ADVISORIES. No commit, push, PR, merge, native launch or
provider request. No merge readiness or live personality success claim.

## Scope and boundaries

Ten successor paths / 47 cumulative paths against baseline main
`e0d0547f626ce909c3cd74d4441a5adb2cdebbef`. Correct only the nickname counter to
count code points as existing profile validation does. All other 45-path candidate
product/test/report bytes protected; six current-state bodies remain historical
suffixes. Keep canonical IDs/roles, Conductor identity, profiles, six connections,
geometry/topology, authority/routing/cancellation, dependencies, permissions,
workflows and governance unchanged. New worktree is the sole registry addition;
all eight original checkouts and 36 prunable entries preserved. No gate state copied.

## Verification results

Passed: affected name/page tests 53/53 (18 name tests, 35 page cases), strict lint,
typecheck, formatting, production frontend build, documentation/repository/security,
whitespace, exact scope/preservation and session inventory. Intentional pre-fix
red regression Failed 3/18 on valid supplementary/mixed names; the original tests
and over-limit cases passed. That non-required red command remains Failed, not
reclassified. Six added boundary regressions compare accepted/rejected names with
the actual frontend profile validator and preserve graph IDs/roles/fixture data.

Tooling reuse Passed: same APFS via installed-SDK-confirmed statfs, executable
source/target formatter, cp -cR success, Git-ignore and unchanged source snapshot.
DiskManagement was unavailable; no installation, downloads or global setting change.
Final documentation validation reruns after this update. Schema/finalization/status/
full Stop occur after freeze; actual receipts are external, never pre-invented.

Inherited (not newly run): Bot Identity full offline verification/native builds;
subsequent graph/Conductor/readiness tests and unsigned bundle acceptance. Direct
Computer Use receipt observed prior exact bundle first-entry fit 64%, enabled
Fit/Reset/Zoom, coordinator/subtitle, nine names, technical inspector, route return,
manual zoom/pan and Fit restore. No live request occurred there. The preserved
bundle does not contain this Unicode-only correction; no new native bundle/QA run.
Full verify, Rust suites/Clippy and native build: Not run here because their bytes
are unchanged and passing evidence is reused. Live personality and exact-head
remote CI/review: Manual verification pending.

## Architecture findings

Passed by the primary reviewer in a separate read-only review. Code-point counting
aligns the graph overlay with validated profile data and retains canonical routing
and roles. No new store/runtime, IPC, schema, abstraction, timer, remount or topology.
Main and source HEAD trees match, but differ after squash; this main-based worktree
avoids the source branch's prospective 50-path three-dot diff. The new candidate
contains exactly 47 changed paths, without five already-merged provider files.

## Security findings

Passed. Count correction does not relax the 48-character bound, control-character
rejection or canonical fallback for invalid/missing values. No secrets, credential
inspection, provider/runtime contact, permissions, WebView/IPC, SQLite, filesystem
execution, hooks, policy, CSP, dependencies, logs or authority change. Only synthetic
Unicode values in regressions. All original state/report/external/artifact bytes
and other worktree snapshots validate. No security exception or gate weakening.

## Code-health findings

Passed. One production expression matches Rust chars().count() and frontend
Array.from().length. Six regressions cover 25/48 supplementary characters, 48 ASCII
and mixed characters, and 49-character rejection. They prove profile/graph agreement
and preserved role/ID/edge/group/event behavior; all existing assertions remain
byte-identical. Strict checks have no suppressions. Names count code points, not
grapheme clusters, consistent with the existing contract. No remaining blocker.

## Technical debt

Advisory: exact-head Linux/Target-Mac CI/review, live personality behavior and native
Unicode appearance remain unverified. Risk: local/synthetic evidence cannot prove
another host or model behavior; effort: separately authorized publication/CI and
optional later owner QA; milestone: Bot Identity publication/QA; blocks_completion:
false; blocks_next_increment: false. Retain D-127 audit debt, D-128 custody/abort
limits, native/provider/runtime live-success and Codex-isolation/internal-retry
advisories, process-local Python 3.12/Xcode/SDK 27.0/Cargo strip workaround. OpenAI
parked 4/5 used; D-125/M1/M2 parked. No advisory waived or silently repaired.

## Roadmap findings

Ready with advisories for separate publication authorization only. Existing provider
QA remains parked; no bot-to-bot messaging, tools, background work or new provider
starts here. Historical plans/reviews remain immutable; current prefixes attribute
new native QA separately. No roadmap reorder or publication authority inferred.

## Completion decision

PASS WITH ADVISORIES. Required local validation and read-only review Passed. The
optional intentional red command Failed truthfully and is superseded by the required
green run. Report/schema/finalization/status/full Stop must pass externally after
freeze. Original source completion and all terminal histories remain untouched.

## Next-increment readiness

Ready with advisories. Next: explicit owner authorization to commit the 47 paths
with baseline as sole parent, push without force and open one new PR into main;
inspect every exact-head check and review. Do not reopen PR #127 or merge. Exact
standalone publication prompt is in HANDOFF.md and NEXT_STEPS.md.

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
- `docs/plans/2026-09-30-bot-identity-publication-preparation.md`
- `docs/plans/2026-09-30-command-center-measurement-readiness.md`
- `docs/plans/2026-09-30-command-center-startup-readiness.md`
- `docs/plans/2026-09-30-conductor-display-name.md`
- `docs/reviews/2026-09-30-bot-graph-name-sync-post-increment-review.md`
- `docs/reviews/2026-09-30-bot-identity-personality-post-increment-review.md`
- `docs/reviews/2026-09-30-bot-identity-publication-preparation-post-increment-review.md`
- `docs/reviews/2026-09-30-command-center-measurement-readiness-post-increment-review.md`
- `docs/reviews/2026-09-30-command-center-startup-readiness-post-increment-review.md`
- `docs/reviews/2026-09-30-conductor-display-name-post-increment-review.md`
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
- `src/features/command-center/commandCenterFixtures.ts`
- `src/features/command-center/commandCenterProjection.test.ts`
- `src/features/command-center/components/OperationalTopologyAdapter.test.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`

## Exact commands executed

- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run test:frontend -- src/features/command-center/commandCenterBotNames.test.ts` — Failed 3/18 in intentional pre-fix red run; retained as non-required historical execution.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run test:frontend -- src/features/command-center/commandCenterBotNames.test.ts src/features/command-center/CommandCenterPage.test.tsx` — Passed 53/53 in corrected affected run.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run lint:frontend` — Passed.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run typecheck` — Passed.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run format:frontend` — Passed; final applicable documentation receipts recorded externally.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run build:frontend` — Passed.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run docs:check` — Passed; final applicable documentation receipts recorded externally.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run repository:check` — Passed; final applicable documentation receipts recorded externally.
- `sh /private/tmp/cortexa-bot-identity-publication-preparation-evidence/offline.sh npm run security:scan` — Passed; final applicable documentation receipts recorded externally.
- `git diff --check` — Passed; final applicable documentation receipts recorded externally.
- `python3 -B /private/tmp/cortexa-bot-identity-publication-preparation-evidence/check-preservation.py` — Passed; final applicable documentation receipts recorded externally.
- `python3 -B .codex/hooks/session_end_gate.py` — Passed; final applicable documentation receipts recorded externally.

Read-only preflight, exact main-object retrieval, worktree creation, transfer/tooling
clone, admission and targeted formatting are retained in external/tool history.
No unexecuted application command is claimed as executed. Full verify/Rust/native
builds/live/native QA: Not run here. Post-freeze schema/finalization/status/full Stop
receipts are retained externally; no predecessor validator is rewritten.
