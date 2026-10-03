# Bounded OpenAI stream-error diagnostic post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "openai-stream-error-diagnostic",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh cargo test --manifest-path src-tauri/Cargo.toml --lib --locked top_level",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- src/infrastructure/tauri/diagnostics-client.test.ts src/features/settings/DiagnosticsPanel.test.tsx",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run verify",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run docs:check",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run repository:check",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-openai-stream-error-diagnostic-evidence/preservation.py",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-openai-stream-error-diagnostic-evidence/report-schema-check.py"
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
    "docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md",
    "scripts/browser/diagnostics-check.mjs",
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
    "src/features/settings/DiagnosticsPanel.test.tsx",
    "src/features/settings/DiagnosticsPanel.tsx",
    "src/features/settings/SettingsPage.tsx",
    "src/infrastructure/tauri/diagnostics-client.test.ts",
    "src/infrastructure/tauri/diagnostics-client.ts",
    "src/styles.css"
  ],
  "findings": [
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "New native visual and live diagnostic behavior remain unobserved; current task forbids launches and requests. Both debug bundles predate this change.",
      "risk": "Do not use offline fixtures or predecessor binaries as new native/live evidence; historical error cause remains unknown.",
      "effort": "Owner-authorized updated bundle and bounded QA",
      "milestone": "Next native diagnostic checkpoint",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "The deliberately narrow code allowlist can report unknown. Older diagnostic readers reject new closed enum labels.",
      "risk": "Additional provider codes need explicit review; do not move new logs into an older build and claim compatibility.",
      "effort": "Bounded future schema/code review only when needed",
      "milestone": "Future diagnostics compatibility",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Retain D-127/D-128, native/provider/runtime and Codex-isolation advisories, process-local Python 3.12/Xcode SDK27/Cargo strip=none workaround, and parked D-125/M1/M2.",
      "risk": "No signing, production, broader isolation, live success/cancellation or remote CI claim.",
      "effort": "Owner-selected bounded follow-up",
      "milestone": "Existing parked roadmap",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Inherited Vite chunk exceeds 500 kB; diagnostic app-version schema, latched logging availability, macOS export and owner/760px QA limitations remain.",
      "risk": "No unrelated performance, platform, storage or layout repair is included.",
      "effort": "Separately scoped follow-up",
      "milestone": "Existing advisory backlog",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "verification": [
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh cargo test --manifest-path src-tauri/Cargo.toml --lib --locked top_level",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- src/infrastructure/tauri/diagnostics-client.test.ts src/features/settings/DiagnosticsPanel.test.tsx",
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
      "command": "python3 -B /private/tmp/cortexa-openai-stream-error-diagnostic-evidence/preservation.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-openai-stream-error-diagnostic-evidence/report-schema-check.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Independent successor architecture/security/code-health review against frozen predecessor; no actionable findings",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Inherited local-diagnostics Computer Use/layout/Simulation receipts and both artifacts preserved; not newly executed",
      "required": false,
      "status": "Passed"
    },
    {
      "check": "New native visual QA and live provider behavior; explicitly excluded by owner no-launch/no-request scope",
      "required": false,
      "status": "Not run"
    }
  ]
}
-->

Date: 2026-10-02
Increment: openai-stream-error-diagnostic
Worktree: `/Users/hdang/.codex/worktrees/local-diagnostics/ai-agent-assistant`.
Branch: `codex/local-diagnostics`; HEAD `662fe1a57a1148a215beb04b215ad35681c4cbe6`.
Evidence: `/private/tmp/cortexa-openai-stream-error-diagnostic-evidence`.

## Executive summary

PASS WITH ADVISORIES for the authorized offline implementation. A validated
top-level SSE error records one closed code classification while preserving the
generic conversation error and terminal stream_error. Native visual and live
verification were not performed because the owner prohibited launches/requests.
This neither diagnoses the historical provider cause nor claims live success.

## Scope and boundaries

Exactly fifteen successor paths and thirty-three cumulative paths. Six source/test
files and nine additive documentation files changed from the preserved candidate.
Ordinary begin succeeded after complete/valid predecessor verification and
byte-identical raw-state/candidate archival. The old report, historical document
bodies, external receipts, source outside scope and both debug bundles are unchanged.
No request/host, retry, cleanup, credential, endpoint, approval, dependency, hook,
configuration, provider, owner-data or publication change.

Top-level code is a string-or-null in the official streaming schema. The three
recognized literals are Cortexa's conservative allowlist, not an upstream enum.
Absent/null is missing; empty/non-string is invalid; other nonempty strings are
unknown. No trimming, nested fallback or raw message/param retention. The event
is explicitly openai_top_level_error, separate from response.failed. Existing
Record fields and old enum values remain; only closed vocabulary is additive.

## Verification results

- Passed: seven new focused Rust tests and seven focused frontend tests.
- Passed: full offline npm run verify (247 seconds), including 540 frontend tests,
  430 native unit tests, all integrations, 74 hook tests, 88 repository tests,
  strict formatting/Clippy/ESLint, typecheck, frontend and release native builds.
  The pre-existing opt-in real_hermes_version_probe_is_opt_in_and_version_only test
  remained ignored (its suite reported 13 passed, 1 ignored); no new verification
  of that external runtime is claimed.
- Passed: documentation, repository, security scan, whitespace, preservation and
  session inventory. Report schema and the final documentation receipts bind the
  frozen report before ordinary finalization.
- Inherited only: unchanged native layout/Simulation and previous live QA receipts.
- Not run: new app launch, Computer Use, live request/cancellation, new debug bundle,
  installation, advisory retrieval or remote CI. These are outside this prompt.
- Installed route verified: Python 3.12.1, Rust 1.90.0, process-local Xcode SDK27.
  Both previously accepted bundles retain their hashes and contain predecessor code.

All raw command logs remain externally. No product test failure occurred.
Automatic approval review rejected an initial documentation script before execution
because it reconstructed files from backup. Current hashes were then rechecked and
additive patches preserved the complete current bodies. An initial multi-operation
patch format was rejected before applying; the corrected additive patch succeeded.
Neither event changed historical data or relaxed a gate.

## Architecture findings

Independent review found no actionable defect. The decoder classifies only after
the unchanged framing/type/sequence/response-created checks. Its diagnostic enum
contains no strings. The existing task-local observer owns correlation and emits
once only for a nonterminal OpenAI attempt. Generic terminal errors, host cleanup
and transport are byte/behavior preserved. No extra execution authority or new IPC.
The existing Settings renderer consumes matching closed TypeScript vocabulary.

## Security findings

PASS for the bounded change. Real decoder tests cover configured/default modes,
split/coalesced streams, malformed types, case/whitespace variants, nested-only
and conflicting shapes, unknown canaries, and rejection before classification.
Snapshot serialization, disk, restart and export exclude canary content and raw
response/conversation identities. Frontend rejects arbitrary fields/categories.
Missing/unavailable logging is nonfatal; duplicate/late and other-provider events
are suppressed. Existing privacy, capacity, file-mode and symlink gates remain.
No credentials were read and no provider request was made.

## Code-health findings

No actionable finding from independent review. Original assertions remain;
new tests complement rather than replace response.failed and protocol tests.
Request construction and all host files compare unchanged with saved predecessor
bytes. New enum classifications have explicit missing/invalid/unknown buckets.
The generic terminal result and one-terminal-record behavior remain unchanged.

## Technical debt

The owner retains the four advisory findings in the manifest. The allowlist is
intentionally narrow; an unknown bucket may still require future owner evidence.
New readers retain old records, but older binaries may reject new vocabulary.
Do not claim data downgrade compatibility. Existing build chunk, portability,
schema-version, logging availability, isolation and QA limitations are unchanged.

## Roadmap findings

Ready with advisories for a separately authorized updated isolated debug bundle
and bounded native diagnostic checkpoint. No automatic roadmap progression,
provider attempt or publication. Old batch remains exhausted at 5/5; replacement
batch remains 1/10 used, 9 remaining. Every future Send still needs final owner
acknowledgement, verified identity/context and no fallback/retry. D-125/M1/M2 parked.

## Completion decision

PASS WITH ADVISORIES. Ordinary finalization is permitted only after all required
verification and frozen-report checks pass. External status and full Stop receipts
establish the actual completed/valid marker. No new native/live acceptance claim.

## Next-increment readiness

Ready with advisories. Obtain owner authorization for one updated isolated unsigned
bundle before native QA. Existing bundles do not contain the classifier. No live
request, key inspection or launch is authorized by this report. Browser preview:
`npm run dev -- --host 127.0.0.1` from this worktree, http://127.0.0.1:1420; native
logging needs the separately prepared artifact. See HANDOFF.md for the exact prompt.

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
- `docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md`
- `scripts/browser/diagnostics-check.mjs`
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
- `src/features/settings/DiagnosticsPanel.test.tsx`
- `src/features/settings/DiagnosticsPanel.tsx`
- `src/features/settings/SettingsPage.tsx`
- `src/infrastructure/tauri/diagnostics-client.test.ts`
- `src/infrastructure/tauri/diagnostics-client.ts`
- `src/styles.css`

## Exact commands executed

- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh cargo test --manifest-path src-tauri/Cargo.toml --lib --locked top_level`.
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- src/infrastructure/tauri/diagnostics-client.test.ts src/features/settings/DiagnosticsPanel.test.tsx`.
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run verify`.
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run docs:check`.
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run repository:check`.
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run security:scan`.
- Passed: `git diff --check`.
- Passed: `python3 -B /private/tmp/cortexa-openai-stream-error-diagnostic-evidence/preservation.py`.
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py`.
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-openai-stream-error-diagnostic-evidence/report-schema-check.py`.

The full suite was run once after product changes stabilized; later documentation
checks were repeated only for final report/handoff edits. No unchanged native QA
or Simulation workflow was repeated. Finalize/status/full Stop are recorded
externally without editing this report after its fingerprint is frozen.
