# Provider troubleshooting post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "provider-troubleshooting",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh cargo test --manifest-path src-tauri/Cargo.toml --lib --locked diagnostics",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- src/features/settings/DiagnosticsPanel.test.tsx src/features/settings/diagnosticSummary.test.ts src/infrastructure/tauri/diagnostics-client.test.ts src/infrastructure/tauri/agent-chat-client.test.ts",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run verify",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run docs:check",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run repository:check",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-provider-troubleshooting-evidence/preservation.py",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py"
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
    "docs/plans/2026-10-02-local-diagnostics.md",
    "docs/plans/2026-10-02-openai-stream-error-diagnostic.md",
    "docs/plans/2026-10-02-openai-stream-error-shape-diagnostic.md",
    "docs/plans/2026-10-02-provider-troubleshooting.md",
    "docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md",
    "docs/reviews/2026-10-02-provider-troubleshooting-post-increment-review.md",
    "scripts/browser/diagnostics-check.mjs",
    "scripts/browser/diagnostics-fixture.html",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/agent_chat_tauri.rs",
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
    "src/features/agents/AgentsPage.tsx",
    "src/features/settings/DiagnosticsPanel.test.tsx",
    "src/features/settings/DiagnosticsPanel.tsx",
    "src/features/settings/SettingsPage.tsx",
    "src/features/settings/diagnosticSummary.test.ts",
    "src/features/settings/diagnosticSummary.ts",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts",
    "src/infrastructure/tauri/diagnostics-client.test.ts",
    "src/infrastructure/tauri/diagnostics-client.ts",
    "src/styles.css"
  ],
  "findings": [
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Live checkpoint blocked on owner ledger/setup confirmation, private launch and final Send acknowledgement. No live attempt consumed.",
      "risk": "Native Simulation is not live success, historical-cause diagnosis or live cancellation. Overall live QA objective remains incomplete.",
      "effort": "One bounded owner-assisted checkpoint",
      "milestone": "Existing live QA checkpoint",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Retain D-127/D-128, native/provider/runtime, Codex isolation/internal-retry visibility, bounded-log and old-reader limits, owner/760px QA, Vite chunk warning, process-local Python3.12/Xcode SDK27/Cargo stripping route and parked D-125/M1/M2.",
      "risk": "No audit completeness, remote cleanup, production/signing or remote CI claim.",
      "effort": "Separately scoped owner-selected follow-up",
      "milestone": "Existing advisory backlog",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "verification": [
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh cargo test --manifest-path src-tauri/Cargo.toml --lib --locked diagnostics",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- src/features/settings/DiagnosticsPanel.test.tsx src/features/settings/diagnosticSummary.test.ts src/infrastructure/tauri/diagnostics-client.test.ts src/infrastructure/tauri/agent-chat-client.test.ts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run verify",
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
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-provider-troubleshooting-evidence/preservation.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Verified isolated native Simulation streaming/completion, correlated Diagnostics timeline/filter/copy, readable layout and graceful quit",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Live OpenAI checkpoint blocked on owner confirmation/private launch and final paid-Send acknowledgement",
      "required": false,
      "status": "Manual verification pending"
    }
  ]
}
-->

Date: 2026-10-02
Branch: codex/local-diagnostics
HEAD: 662fe1a57a1148a215beb04b215ad35681c4cbe6

## Executive summary

PASS WITH ADVISORIES for implementation and available native/offline acceptance.
The user explicitly permits finishing independent work when private launch or setup
confirmation is missing. Live QA remains blocked on those owner prerequisites; the
overall live-provider objective is incomplete. No live attempt consumed: 3/10 used,
7 remain. No claim that the historical error cause or live cancellation is verified.

## Scope and boundaries

26 successor / 43 cumulative paths. Existing logger/UI/export extended without new
IPC commands/inputs, dependencies, permissions, retry behavior or raw-content fields.
Predecessor files outside scope, historical document bodies, archived raw completion,
receipts and previous bundles are preserved. All current pre-closeout document bytes
also remain as suffixes. No owner-data edits, provider calls, commits or publication.

## Verification results

22 focused Rust and 24 frontend tests passed. Final full offline verify passed after
final source edits: 545 frontend, 435 Rust unit, 255 integration tests; 74 hook and
88 repository tests. One existing opt-in Hermes test remains ignored. Strict format,
lint, Clippy, typecheck, frontend/native builds passed. Separate offline unsigned
isolated debug build passed. Final docs/repository/security/whitespace/preservation
and session checks passed. Exact receipts are in the evidence directory below.

Direct Computer Use, not browser substitution: fresh isolated Simulation profile,
empty instructions/notes, Memory Off; one synthetic Send visibly streamed and completed.
Conversation and Diagnostics request ID matched; dispatch attempt ID was distinct.
First response 1 ms, first text 183 ms, terminal 548 ms, ownership/native terminal-return
771 ms. Provider/request filters and Error empty state worked. Native copy/paste
matched the safe summary; unused draft cleared without Send. Readable 1280x960 UI;
graceful quit verified by executable lookup, artifact bytes unchanged.
No native cancellation, export or restart was repeated; those assertions are automated
or inherited evidence. Live behavior remains unobserved in this increment.

Recoverable failures retained: unchanged exact-IPC checker rejected an extra receipt
call; removed it, observed existing native poll return instead. Corrected explicit
profile fixture, lifecycle/field assertions, unsupported findLast, strict lint and
Clippy queue payload size. Final full verify passed. Computer Use menu interruption
resolved from fresh state, with no product defect inferred. Auto-review rejected
baseline-reconstructed docs; no files changed. Additive-only hash-bound updates passed.

Evidence: `/private/tmp/cortexa-provider-troubleshooting-evidence`:
`full-verify-final.log`, `build.json`, `artifact.json`, `native-observations.json`,
`native-diagnostics.png`, `closeout-checks.json`, `preflight.json`, `preservation.py`.
Report schema/finalization/status/full Stop receipts must also pass before relying on
the marker. Source hashes bind the native artifact; documentation changes do not alter it.

## Architecture findings

Reviewed the full successor diff and inherited scope. Native Rust retains validation,
credential, ownership and transport boundaries. Logical request begins at trusted
conversation preparation; dispatch allocates one separate attempt. Pre-dispatch
rejection has no dispatch ID. One terminal is sealed; workflow/stage correlation stays
in existing hosts. Existing snapshot output exposes the generated request ID. No IPC
input/gate widening. Native terminal poll return is not proof of frontend delivery;
Computer Use establishes actual Simulation display. No architecture drift found.

## Security findings

Allowlisted events/categories and generated/fingerprinted correlations only. No new raw
string channel, stderr capture, key/environment inspection or raw prompt/answer logging.
Existing secure create-new export, symlink/hardlink checks, bounds/rotation and nonfatal
unavailability remain. Synthetic canary, persistence/export and cancellation regressions
passed. The isolated app uses fresh data, not copied owner profiles/rooms. No blocking
security finding; no governance/capability/CSP/network or authority change.

## Code-health findings

Optional request parsing supports old snapshots/logs while rejecting arbitrary values.
Event guards prevent duplicate phases and terminal outcomes; no token/poll flood. Typed
summary and filter regressions passed; native copied content directly matched. Closed
error guidance preserves uncertainty, last phase and last event are separately labeled.
The supported old JavaScript target and queue representation are retained. No blocking
correctness defect found in the final review.

## Technical debt

Advisories in the manifest retain their existing owners/backlog. Bounded retention can
lose earlier events: interruption means no retained terminal, not proof of a crash.
Old readers may reject new enum/schema data. Codex internal retries remain opaque.
Native terminal-return and local transport release do not claim remote success/cleanup.
Existing version binding, latched availability, Vite size and owner-width QA limits remain.
No advisory repair or roadmap expansion performed.

## Roadmap findings

No roadmap reordering; D-125/M1/M2 remain parked. Next is the already-requested live
checkpoint after owner setup/ledger confirmation and private launch, followed by final
acknowledgement. Publication requires separate authorization and remote CI review.

## Completion decision

PASS WITH ADVISORIES. This is implementation/native offline acceptance only. The live
checkpoint remains incomplete for external owner prerequisites; no fixture substitutes
for live success. Ordinary finalization, complete/valid status and full Stop must pass.

## Next-increment readiness

Ready with advisories for the existing owner-assisted live checkpoint, conditional on
its prerequisites. No new allowance, automatic retry, publication or successor authority.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-02-local-diagnostics.md`
- `docs/plans/2026-10-02-openai-stream-error-diagnostic.md`
- `docs/plans/2026-10-02-openai-stream-error-shape-diagnostic.md`
- `docs/plans/2026-10-02-provider-troubleshooting.md`
- `docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md`
- `docs/reviews/2026-10-02-provider-troubleshooting-post-increment-review.md`
- `scripts/browser/diagnostics-check.mjs`
- `scripts/browser/diagnostics-fixture.html`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/agent_chat_tauri.rs`
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
- `src/features/agents/AgentsPage.tsx`
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

## Exact commands executed

- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh cargo test --manifest-path src-tauri/Cargo.toml --lib --locked diagnostics` — Passed.
- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- src/features/settings/DiagnosticsPanel.test.tsx src/features/settings/diagnosticSummary.test.ts src/infrastructure/tauri/diagnostics-client.test.ts src/infrastructure/tauri/agent-chat-client.test.ts` — Passed.
- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run verify` — Passed.
- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run docs:check` — Passed.
- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run repository:check` — Passed.
- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run security:scan` — Passed.
- `git diff --check` — Passed.
- `python3 -B /private/tmp/cortexa-provider-troubleshooting-evidence/preservation.py` — Passed.
- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py` — Passed.

The separate debug bundle argv/exit0 are recorded in build.json. Focused/full logs retain
superseded failures and final passes; no failure receipt was overwritten. Native QA is
recorded separately. Final schema and gate commands are recorded in external receipts.
