# Local diagnostic logging and bounded live QA

Status: Logging implemented and verified; report validated; completion/Stop status retained externally. Live checkpoint blocked by owner-private setup. Owner authorized in the attached 2026-10-02 request.

## Workspace and preservation

Worktree: `/Users/hdang/.codex/worktrees/local-diagnostics/ai-agent-assistant`.
Branch: `codex/local-diagnostics`. Baseline: `662fe1a57a1148a215beb04b215ad35681c4cbe6`.
The Desktop checkout remains older and dirty; Connected Knowledge is preserved.
Ordinary admission succeeded. Initial repository check passed. External evidence:
`/private/tmp/cortexa-local-diagnostics-evidence`. No other visible active local
session targets this new worktree. No gate state or application data was copied.

## Objective and boundaries

Implement bounded content-free Rust diagnostics, native read/export IPC and a
small Settings surface; verify offline and natively before one possible remaining
OpenAI attempt. Existing authentication, approval, routing, cancellation, storage
and provider behavior remain authoritative. No telemetry, raw content, secret,
subprocess stderr, generic filesystem IPC, dependency, governance-hook or branding
change. No commit or publication. OpenAI remains 4/5 used until observed Send.

## Exact planned scope

- `src-tauri/src/diagnostics.rs`
- `src-tauri/src/diagnostics/tests.rs`
- `src-tauri/src/diagnostics_tauri.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/startup.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/personal_assistant_direct_tauri.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/anthropic.rs`
- `src-tauri/src/local_models.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/collaboration_tauri.rs`
- `src/infrastructure/tauri/diagnostics-client.ts`
- `src/infrastructure/tauri/diagnostics-client.test.ts`
- `src/features/settings/DiagnosticsPanel.tsx`
- `src/features/settings/DiagnosticsPanel.test.tsx`
- `src/features/settings/SettingsPage.tsx`
- `src/styles.css`
- `scripts/browser/diagnostics-check.mjs`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- This plan and its post-increment review.

The checker hard-codes all IPC calls. Exactly two allowlist additions and rejection
coverage in `scripts/repository_health.py` and
`scripts/tests/test_repository_health.py` were explicitly approved by Henry in the source session on 2026-10-02.
Only those exact entries and rejection tests are authorized; no gate or policy relaxation.

## Design and risks

Rust owns a bounded nonblocking writer queue and at most three 5 MiB JSONL files.
Closed records include UTC epoch milliseconds, build version, event/severity,
provider, validated model identifier (dynamic identifiers fingerprinted), opaque
application correlations, outcome/error and monotonic timing. First HTTP response
and first accepted text are distinct; protocol-only runtimes identify their first
normalized response. Read/export revalidate persisted records; no raw text fields.
Recent view is bounded. Disk/queue errors make availability false without changing
provider results. Logs are diagnostic, not tamper-proof audit or transcript storage.
File opens reject symlinks, modes restrict access where supported, export uses a
native picker and exclusive creation. No WebView path input.

## Checklist and verification

- [x] Confirm baseline, isolated worktree, preservation and ordinary admission.
- [x] Rust service, rotation/restart/unavailable/export and canary regressions.
- [x] Actual request/stage/runtime lifecycle instrumentation and cancellation tests.
- [x] Closed IPC and Settings availability/filter/copy/export UI regressions.
- [x] Strict formatting/lint/typecheck and full offline `npm run verify`.
- [x] Documentation/repository/security/whitespace and preservation checks.
- [x] Offline isolated unsigned bundle and supported native Computer Use.
- [ ] Reconcile owner-only credential setup and remaining one-attempt live budget.
- [x] Architecture/security/code-health/debt/readiness review and final report.
- [x] Session and report-schema checks; ordinary finalization/status/full Stop receipts are retained externally.

No automated fixture proves live success. Required live work blocked by unavailable
credentials/budget is reported separately after finishing independent logging.
Required native access failure or unapproved checker scope blocks acceptance.
Recover in-scope defects without weakening gates. Preserve all historical records.

## Current checkpoint

Rust service, lifecycle instrumentation, typed IPC and Settings UI implemented; initial focused checks passed. Exact next action: complete the full offline/native verification; the two-command checker exception is approved. Preview from this
worktree: `npm run dev -- --host 127.0.0.1`, http://127.0.0.1:1420. Diagnostics and
providers require the native bundle; ordinary browser preview has no native access.
Retain all advisories, D-127/D-128, native/runtime/Codex-isolation limits, process-local
Python 3.12/Xcode SDK27/Cargo strip=none workaround and parked D-125/M1/M2.

## Native checkpoint and bounded correction

Native Simulation streaming, completion, timings, filtering, copy feedback and
export/refusal passed. At 960px the existing generic sidebar overlay obscured
Settings. Extend the established in-flow inspector/navigation selectors only to
Diagnostics, with actual-App shell browser regression and native recheck.
The previous bundle is retained under external evidence. No owner data changed.

## Final implementation and native evidence

Final source passes full offline verification (538 frontend, 423 native unit,
74 hook, 88 repository tests plus integrations and native build). Actual-shell
Diagnostics regression passes 24 combinations. The corrected isolated unsigned
bundle passed direct native 1280/960/760px, expanded/collapsed navigation,
inspector and scrolling observations; prior records survived restart.
Simulation streaming/completion and correlated timings were directly observed.
Native export refused overwrite after OS confirmation and preserved its hash;
canary prompt/answer are absent from files/export. No live request was made.
Both bundles and original failed intermediate logs are retained externally.
See native-qa.json and artifact-final.json. The task-owned app/server are stopped.

The original task expressly permits finishing independent logging when credentials
are unavailable. Formal logging acceptance does not complete or verify live QA.
The pending owner-private launch/model confirmation is the exact remaining action;
OpenAI remains 4/5 used. No new paid allowance or automatic retry is inferred.
Thirty-one paths include the report and the bounded browser layout regression.

Logging quality is PASS WITH ADVISORIES; live readiness is Blocked, not passed.
The exact31-path report inventory and preservation checks passed.
