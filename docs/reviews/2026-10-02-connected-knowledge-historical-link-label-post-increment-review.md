# Connected Knowledge historical link label post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "connected-knowledge-historical-link-label",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- --run src/features/knowledge/ConnectedKnowledge.test.tsx src/features/knowledge/KnowledgePage.test.tsx",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run lint:frontend",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run build",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-connected-knowledge-evidence/isolated-native.json -- --locked --offline",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run format:check",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run docs:check",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run repository:check",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run security:scan",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh git diff --check",
    "python3 -B /private/tmp/cortexa-connected-knowledge-historical-link-label-evidence/preservation-check.py",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-connected-knowledge-historical-link-label-evidence/report-schema-check.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-01-connected-knowledge-workspace.md",
    "docs/plans/2026-10-02-connected-knowledge-historical-link-label.md",
    "docs/reviews/2026-10-01-connected-knowledge-workspace-post-increment-review.md",
    "docs/reviews/2026-10-02-connected-knowledge-historical-link-label-post-increment-review.md",
    "package-lock.json",
    "package.json",
    "scripts/browser/connected-knowledge-check.mjs",
    "scripts/browser/knowledge-check.mjs",
    "scripts/browser/knowledge-fixture.tsx",
    "src-tauri/src/knowledge.rs",
    "src-tauri/src/knowledge_links.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/startup.rs",
    "src-tauri/src/storage/knowledge.rs",
    "src-tauri/src/storage/migrations.rs",
    "src-tauri/src/storage/store.rs",
    "src-tauri/tests/startup_storage_smoke.rs",
    "src-tauri/tests/storage_smoke.rs",
    "src/application/ApplicationStateProvider.tsx",
    "src/features/knowledge/ConnectedKnowledge.test.tsx",
    "src/features/knowledge/KnowledgeGraph.tsx",
    "src/features/knowledge/KnowledgeMarkdown.tsx",
    "src/features/knowledge/KnowledgePage.test.tsx",
    "src/features/knowledge/KnowledgePage.tsx",
    "src/features/knowledge/LinkEditor.tsx",
    "src/features/knowledge/PropertiesEditor.tsx",
    "src/features/knowledge/UnsavedKnowledgePrompt.tsx",
    "src/features/knowledge/knowledge.css",
    "src/features/knowledge/knowledgeNeighborhood.ts",
    "src/features/knowledge/knowledgeProperties.ts",
    "src/features/knowledge/useUnsavedKnowledge.ts",
    "src/infrastructure/tauri/knowledge-client.ts"
  ],
  "findings": [
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Owner manual QA, live-provider behavior and exact-head remote CI remain unverified.",
      "risk": "Simulation and local checks do not prove live or remote operation.",
      "effort": "Review in separately approved follow-up",
      "milestone": "Owner QA/publication review",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Bounded Markdown/YAML subset and native application Quit guard limitation.",
      "risk": "No full Obsidian compatibility; save edits before quitting the entire native app.",
      "effort": "Review in separately approved follow-up",
      "milestone": "Owner QA/publication review",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Inherited D-127/D-128, native/provider/runtime custody and Codex-isolation advisories; process-local native workaround; parked D-125/M1/M2.",
      "risk": "No new security or production-readiness claim; OpenAI remains parked 4/5.",
      "effort": "Review in separately approved follow-up",
      "milestone": "Owner QA/publication review",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Vite reports a bundle chunk larger than 500 kB.",
      "risk": "Possible startup cost; bounded current private demo scope, no automatic architecture expansion.",
      "effort": "Review in separately approved follow-up",
      "milestone": "Owner QA/publication review",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "verification": [
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- --run src/features/knowledge/ConnectedKnowledge.test.tsx src/features/knowledge/KnowledgePage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run lint:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run build",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-connected-knowledge-evidence/isolated-native.json -- --locked --offline",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run format:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-connected-knowledge-historical-link-label-evidence/preservation-check.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-connected-knowledge-historical-link-label-evidence/report-schema-check.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Direct supported Computer Use historical/current v1/v2, discard without v3, link/graph navigation, readability and cleanup; native-observations.json",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Live-provider behavior, remote exact-head CI and remaining owner personal QA",
      "required": false,
      "status": "Manual verification pending"
    }
  ]
}
-->

Date: 2026-10-02
Increment: connected-knowledge-historical-link-label
Branch: codex/connected-knowledge-workspace

## Executive summary

Historical Knowledge links remain literal and non-navigating without a false
missing-target label. Required local automated and direct native QA passed.
Quality result: PASS WITH ADVISORIES. Ordinary complete/valid status and full Stop
are separate authoritative closure receipts. No commit or publication.
Worktree: `/Users/hdang/.codex/worktrees/connected-knowledge-workspace/ai-agent-assistant`;
unchanged HEAD `cc8b1b4e21b765b3af5bd5bdf5cf7fc91e021324`.

## Scope and boundaries

Exactly 12 successor and 41 cumulative paths. The only new executable edits are
KnowledgeMarkdown, KnowledgePage and two regressions in ConnectedKnowledge.test.
A historical boolean defaults false; the historical branch returns literal React
text before looking at bindings. KnowledgePage selects it only for old versions
and explains navigation is unavailable. Current link statuses, current graph and
backlinks, aliases, source/history bytes and unsaved-preview behavior stay intact.
No Rust, storage, IPC, dependency, governance, workflow or permission successor edit.
Historical documentation bodies are byte-identical suffixes; original plan/review,
raw completion state, full Stop and bound evidence preserved. Prior accepted bundle
is recoverable from a hash-verified external ZIP. Other checkouts/data remain intact.

## Verification results

Evidence: `/private/tmp/cortexa-connected-knowledge-historical-link-label-evidence`.
Final affected tests passed 24/24. Strict lint/typecheck/frontend build passed.
One unsigned offline isolated bundle built with locked dependencies and verified
installed Python 3.12.1, Rust1.90, Xcode/SDK27.0 and process-local release/build-override
strip=none. No tool installation, downloads or global changes.
Format, docs, repository, security, whitespace, exact-scope/preservation and session
checks passed; final report validation/status/Stop receipts establish closure.
No conflicts, staged changes, suspicious generated or personal-data paths.

Direct Computer Use verified exact process/artifact identity, historical v1 literal
Incident link without missing suffix, visible navigation-unavailable disclosure,
no navigation after historical text click, and v2 restoration of link and QA line.
Both note hashes remained unchanged. Approved unsaved edit was discarded through
visible confirmation; exactly two versions remained, no Save or version3. Current
Incident link opened preserved v3/hash; visible graph node returned to QA v2.
Properties/backlinks, headings, disclosure, scrolling and inspector open/closed
were readable at 1440x1000. App quit; process PID9130 absent. No room/profile access,
existing-note edit, workflow start, source transmission or provider request.

Prior full native verification, audits, storage/fixture and browser width matrix
are explicitly inherited unchanged, not newly executed. Frontend-tier policy
applies to this display-only repair; no unchanged Rust test repeats. Narrow/760px
native behavior, live providers, streaming/cancellation and remote CI are unverified
by this run. No application whole-Quit guard is claimed.

Earlier recoverable new-test failures: accessible-name query omitted the actual
region/name boundary, then TypeScript rejected an unsupported exact query option.
Both corrected without assertion/gate relaxation. Original focused.log and
frontend-build.log remain; final focused-accepted.log and frontend-build-final.log
are passing evidence. No recovery worktree or terminal-history rewrite.

## Architecture findings

Passed. Native storage still enriches current versions only. Historical rendering
cannot substitute current bindings, and graph/backlink labeling remains explicitly
current-library. No new architecture, coupling, runtime or authority surface.
Removing only the declared additions reconstructs all three predecessor source/test
hashes exactly; inherited implementation assertions remain unchanged.

## Security findings

Passed. Historical React text stays inert even when resolved/ambiguous/removed
bindings are supplied. HTML, URLs and images remain inert; no innerHTML, network,
credential access, privilege or new IPC. Private notes, consent, source budgets,
cancellation, storage and export boundaries unchanged. Scanner and preservation
passed. No blocking security finding.

## Code-health findings

Passed. Renderer regression verifies literal aliases, code exclusion, no status
claims and no navigation under supplied bindings. Actual-page regression switches
v2/v1/v2, checks disclosed restrictions and unchanged version hashes/content,
current graph availability, disabled historical editing and working current target
navigation, with no save call. Existing missing/ambiguous/removed/current assertions
remain. Native QA confirms the actual page behavior and approved discard.

## Technical debt

Inherited D-127/D-128, native/provider/runtime live-success, custody, Codex isolation,
process-local native workaround, bounded Markdown/YAML, native Quit guard limitation
and Vite >500kB chunk advisories retained. Risks/effort/milestone are recorded in the
manifest; no new blocker or automatic debt expansion. OpenAI parked4/5;
D-125/M1/M2 parked. Normal authorization rules, no standing delegation.

## Roadmap findings

Ready with advisories for read-only publication review of the 41-path candidate.
Remote CI/publication and live-provider QA require separate authorization. No next
increment, provider expansion, graph work or parked roadmap task is started.

## Completion decision

PASS WITH ADVISORIES. Ordinary finalization only after final required receipts
pass; complete/valid status and full Stop are required and not inferred from prose.
Prior completion history remains preserved, not reopened or rewritten.

## Next-increment readiness

Ready with advisories: read-only publication readiness review, preserving both
accepted increments, external frozen evidence, original artifacts and synthetic data.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-01-connected-knowledge-workspace.md`
- `docs/plans/2026-10-02-connected-knowledge-historical-link-label.md`
- `docs/reviews/2026-10-01-connected-knowledge-workspace-post-increment-review.md`
- `docs/reviews/2026-10-02-connected-knowledge-historical-link-label-post-increment-review.md`
- `package-lock.json`
- `package.json`
- `scripts/browser/connected-knowledge-check.mjs`
- `scripts/browser/knowledge-check.mjs`
- `scripts/browser/knowledge-fixture.tsx`
- `src-tauri/src/knowledge.rs`
- `src-tauri/src/knowledge_links.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/startup.rs`
- `src-tauri/src/storage/knowledge.rs`
- `src-tauri/src/storage/migrations.rs`
- `src-tauri/src/storage/store.rs`
- `src-tauri/tests/startup_storage_smoke.rs`
- `src-tauri/tests/storage_smoke.rs`
- `src/application/ApplicationStateProvider.tsx`
- `src/features/knowledge/ConnectedKnowledge.test.tsx`
- `src/features/knowledge/KnowledgeGraph.tsx`
- `src/features/knowledge/KnowledgeMarkdown.tsx`
- `src/features/knowledge/KnowledgePage.test.tsx`
- `src/features/knowledge/KnowledgePage.tsx`
- `src/features/knowledge/LinkEditor.tsx`
- `src/features/knowledge/PropertiesEditor.tsx`
- `src/features/knowledge/UnsavedKnowledgePrompt.tsx`
- `src/features/knowledge/knowledge.css`
- `src/features/knowledge/knowledgeNeighborhood.ts`
- `src/features/knowledge/knowledgeProperties.ts`
- `src/features/knowledge/useUnsavedKnowledge.ts`
- `src/infrastructure/tauri/knowledge-client.ts`

## Exact commands executed

Each required command and actual passing result is recorded verbatim in the
manifest. Logs bind the final executions; output redirections only select external
evidence destinations. Earlier failed attempts above were superseded in this same
increment and retained. Existing `validate_report` validates schema/evidence before
ordinary finalization; its external receipt and final full Stop/status are closure
checks. No inherited commands are claimed newly executed.
