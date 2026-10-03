# OpenAI stream-error shape diagnostic post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "openai-stream-error-shape-diagnostic",
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
    "python3 -B /private/tmp/cortexa-openai-stream-error-shape-diagnostic-evidence/preservation.py",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-openai-stream-error-shape-diagnostic-evidence/report-schema-check.py"
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
    "docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md",
    "docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md",
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
      "summary": "New native and live diagnostic behavior remain unobserved; current task permits only a synthetic browser fixture. Three prior bundles predate this classifier.",
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
      "command": "python3 -B /private/tmp/cortexa-openai-stream-error-shape-diagnostic-evidence/preservation.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-openai-stream-error-shape-diagnostic-evidence/report-schema-check.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Successor architecture/security/code-health/debt/readiness review against frozen predecessor; no blocking findings",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Supported Computer Use of actual DiagnosticsPanel with isolated synthetic records, labels/correlation/filtering and real copy/paste",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Inherited native/Simulation receipts and three artifacts preserved; not newly executed",
      "required": false,
      "status": "Passed"
    },
    {
      "check": "New native/live diagnostic behavior; excluded by owner no-launch/no-request scope",
      "required": false,
      "status": "Not run"
    }
  ]
}
-->

Date: 2026-10-02
Increment: openai-stream-error-shape-diagnostic
Worktree: `/Users/hdang/.codex/worktrees/local-diagnostics/ai-agent-assistant`.
Branch: `codex/local-diagnostics`; HEAD `662fe1a57a1148a215beb04b215ad35681c4cbe6`.
Evidence: `/private/tmp/cortexa-openai-stream-error-shape-diagnostic-evidence`.

## Executive summary

PASS WITH ADVISORIES for the bounded offline diagnostic. New absent/null code and closed parameter
observations retain no arbitrary provider content. Historical missing-code remains
readable. Synthetic actual-panel Computer Use passed; no new native/live claims.

## Scope and boundaries

Exactly sixteen successor / thirty-six cumulative paths; ordinary admission after
complete/valid predecessor verification and byte-identical raw-state archival.
All 33 predecessor paths outside this scope, historical Markdown bodies, receipts,
protected tracked source and three prior bundles remain unchanged. No credentials,
request construction, response.failed, framing, validation, terminal error, retry,
cleanup, IPC, permission, dependency or storage-field change. No commit/publication.

## Verification results

Nine focused Rust tests and seven focused frontend tests passed. Full offline
verification passed after correcting a relative self-link in the new plan; the first
failed receipt is retained. Strict formatting/lint/Clippy, 74 hook tests, 88 repository
tests, 540 frontend tests, 432 Rust unit tests and integration contracts, frontend and
native no-bundle build ran via installed process-local tooling. Full verify repeats
unit tests through its existing integration command; no gate was weakened.

Direct browser evidence is browser-observations.json and browser-labels.jpg. All
closed labels, legacy missing-code, matching correlation fields, severity filters,
empty state, scrolling and refresh were observed. Copy succeeded and keyboard paste
into a disposable fixture textarea verified fourteen records and fourteen original
fields. The browser tool clipboard read was empty; this was not a product failure.
The label matrix is illustrative, not a single real attempt lifecycle. No native
application, owner data, live provider or real file export was used. Browser tab and
task-owned Vite server were closed. Prior native/Simulation evidence is inherited.

## Architecture findings

No blocking finding. Classification stays at the validated top-level decoder and
crosses into diagnostics only as copyable closed enums. Observer lock plus existing
provider/terminal/duplicate guards bounds one code and one param observation. The
record's fourteen fields, logger ownership, request payloads and cleanup remain.
No framework, new abstraction, provider or authority expansion is introduced.

## Security findings

No blocking finding. Exact literals only; empty/non-string is invalid and all other
strings unknown. No normalization, nested fallback, raw message/frame, excluded
value or hash is retained. Decoder tests include adversarial canaries, conflicting
nested fields, large values and split/coalesced streams; snapshots, restart, disk
and export are checked. Malformed framing and response.failed never emit these
observations. Strict frontend schema rejects extra content channels. Native IPC,
capabilities/CSP, credentials, hooks, filesystem/export policy and networking remain.

## Code-health findings

No blocking finding. Focused lifecycle tests preserve generic terminal errors,
correlation, duplicate/late suppression and unavailable-logger behavior. Legacy
missing-code restart/export stays unchanged. Browser copy semantics use the existing
actual component, with an injected fixed snapshot and synthetic paste target only.
Automatic review rejected a combined broad-format command before execution; scoped
rustfmt then formatted only the three authorized Rust files. No evidence of an
application defect was inferred from tooling retrieval limitations.

## Technical debt

Advisory details, risks, efforts and blocking flags are in the manifest. Preserve
D-127/D-128, native/provider/runtime and Codex-isolation limits, owner/760px QA,
Vite chunk warning, diagnostic app-version and latched availability limitations,
macOS export limits and process-local Python3.12/XcodeSDK27/Cargo strip=none route.
New labels may be rejected by older readers. Bounded logging can lose records;
missing diagnostics do not establish absent upstream fields. D-125/M1/M2 stay parked.

## Roadmap findings

OpenAI paused at 3/10 used, 7 remaining. No cause recovered for earlier HTTP403 or
stream failures, no live success/cancellation verified. No roadmap reordering.
Next proposed work is an explicitly authorized isolated updated diagnostic bundle,
followed only later by owner-private launch and separately acknowledged live QA.

## Completion decision

PASS WITH ADVISORIES. Ordinary finalization is permitted only after all required checks pass;
external status/full-Stop receipts are authoritative. Prior history is not rewritten.

## Next-increment readiness

Ready with advisories for bounded isolated bundle preparation only after owner
approval. Preserve all 36 paths and prior artifacts; no automatic launch or Send.
No publication or provider-use authority follows from a passing local gate.

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
- `docs/reviews/2026-10-02-local-diagnostics-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-diagnostic-post-increment-review.md`
- `docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md`
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
- `src/features/settings/DiagnosticsPanel.test.tsx`
- `src/features/settings/DiagnosticsPanel.tsx`
- `src/features/settings/SettingsPage.tsx`
- `src/infrastructure/tauri/diagnostics-client.test.ts`
- `src/infrastructure/tauri/diagnostics-client.ts`
- `src/styles.css`

## Exact commands executed

- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh cargo test --manifest-path src-tauri/Cargo.toml --lib --locked top_level`
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run test:frontend -- src/infrastructure/tauri/diagnostics-client.test.ts src/features/settings/DiagnosticsPanel.test.tsx`
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run verify`
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run docs:check`
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run repository:check`
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh npm run security:scan`
- Passed: `git diff --check`
- Passed: `python3 -B /private/tmp/cortexa-openai-stream-error-shape-diagnostic-evidence/preservation.py`
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B .codex/hooks/session_end_gate.py`
- Passed: `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh python3 -B /private/tmp/cortexa-openai-stream-error-shape-diagnostic-evidence/report-schema-check.py`

The initial full-verify.log/json records the failed plan-link check; full-verify-2
is the successful complete rerun. An early schema check rejected the unfinished
draft with Not run checks; final schema verification uses the completed evidence.
External run receipts preserve actual commands
and exit statuses. Finalization/status/Stop are recorded after the report freezes.
