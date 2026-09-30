# Provider milestone post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "provider-milestone",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
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
    "docs/plans/2026-09-30-provider-milestone.md",
    "docs/provider-milestone.md",
    "docs/reviews/2026-09-30-provider-milestone-post-increment-review.md",
    "src-tauri/src/agent_chat.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/agent_preferences.rs",
    "src-tauri/src/codex_connection.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/storage/agent_preferences.rs",
    "src/features/agents/AgentsPage.test.tsx",
    "src/features/agents/AgentsPage.tsx",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts"
  ],
  "commands_executed": [
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
    "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
    "python3 -B /private/tmp/cortexa-provider-milestone-evidence/isolation_probe_final.py",
    "python3 -B /private/tmp/cortexa-provider-milestone-evidence/check-preservation.py",
    "python3 -B .codex/hooks/session_end_gate.py",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check"
  ],
  "verification": [
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
      "status": "Passed",
      "required": true
    },
    {
      "command": "sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
      "status": "Passed",
      "required": true
    },
    {
      "command": "python3 -B /private/tmp/cortexa-provider-milestone-evidence/isolation_probe_final.py",
      "status": "Passed",
      "required": true
    },
    {
      "command": "python3 -B /private/tmp/cortexa-provider-milestone-evidence/check-preservation.py",
      "status": "Passed",
      "required": true
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "status": "Passed",
      "required": true
    },
    {
      "command": "npm run docs:check",
      "status": "Passed",
      "required": true
    },
    {
      "command": "npm run repository:check",
      "status": "Passed",
      "required": true
    },
    {
      "command": "npm run security:scan",
      "status": "Passed",
      "required": true
    },
    {
      "command": "git diff --check",
      "status": "Passed",
      "required": true
    }
  ],
  "manual_verification": [
    {
      "check": "Owner native GUI and authenticated OpenAI/Codex live QA",
      "status": "Manual verification pending",
      "required": false
    }
  ],
  "findings": [
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Authenticated live OpenAI/Codex and native GUI checks remain unverified.",
      "risk": "Fixtures cannot establish account entitlement, live output or remote cancellation.",
      "effort": "Owner QA with explicit request allowance.",
      "milestone": "Provider manual QA",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Architecture",
      "severity": "Advisory",
      "summary": "Codex 0.159.0 protocol and dedicated-home prerequisites are intentionally narrow.",
      "risk": "Other versions fail closed; runtime internal retries/token budgets are not Cortexa-controlled.",
      "effort": "Reverify before supporting another runtime version.",
      "milestone": "Future runtime maintenance",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "D-127/D-128, process-local native workaround and non-macOS CI remain advisory.",
      "risk": "No new audit or Linux-host execution was run for unchanged dependencies.",
      "effort": "Retain accepted debt; run exact-head CI only with publication authority.",
      "milestone": "Owner-directed follow-up",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-09-30
Increment: provider-milestone
Branch: codex/provider-milestone

## Executive summary

Implementation and automated functional verification passed. Documentation, security, scope and preservation checks passed. The report is ready for ordinary schema validation/finalization; generated marker and Stop receipts remain authoritative. It does not claim live-service verification.

## Scope and boundaries

Exactly 24 paths, frozen in the plan. Existing native orchestration, six connections and all nine agents are retained. No dependency, migration, governance, workflow, graph, provider catalog expansion, commit or publication changes. Native Codex is an owner-enabled text-only subprocess, not a generic command executor. Original worktrees, terminal states, historical suffixes and prunable entries are preserved.

## Verification results

Final offline verify and unsigned debug bundle passed. Counts: 74 hooks, 85 repository tests, 432 frontend tests, 374 Rust unit tests and all applicable integration suites. One existing real-Hermes test is intentionally ignored, requiring an operator-supplied executable. Key-free actual Codex loopback fixture emitted one synthetic request with no tools, streamed text and completed. No real provider request was made. Earlier PATH, lint, fixture locale and max-effort sidecar failures were corrected in this same increment; logs remain in external evidence. Final app-exit cleanup regression passed. Documentation/repository/security/whitespace, exact-scope, preservation and session checks passed. Schema/finalization/Stop receipts are recorded externally after this report is frozen.

## Architecture findings

Reviewed the full native adapter, IPC, persisted settings and UI boundaries using architecture-review. Models and WebView remain untrusted. Model/effort discovery is revalidated at Send; conversation context is captured natively. Each ephemeral Codex thread receives only the selected context and completed turns. Adapter-owned child registry supports synchronous exit cleanup; no shared runner or runtime settings change. No blocking architecture finding remains.

## Security findings

Reviewed using security-review. Cleared child environment excludes API keys; dedicated runtime auth stays outside WebView and SQLite. Executable/home are owner launch configuration, never IPC. Exact runtime version, strict config, disabled tools/plugins/hooks/background service, empty environments/roots and checked thread permissions fail closed. Raw stderr/account/error payloads are never displayed or logged by the adapter. Frames, queue, event count, prompt/history, output and time are bounded. Drop/cancellation/exit reap owned children before release/exit. D-132 documents the new narrow trust boundary. No live entitlement or universal runtime isolation claim is made.

## Code-health findings

Reviewed using code-review. Typed closed settings/errors, revision checks, explicit acknowledgement, native catalog validation and existing ownership rules remain. Tests cover offline stream completion, tool rejection, error redaction, child reap, exit cleanup, UI discovery/save/send, note exclusion and all-nine-profile restart. Recoverable lint/assertion/storage failures were fixed without weakening gates or creating successors. No blocking finding remains.

## Technical debt

Advisories are listed in the manifest: live/manual proof pending; version-pinned Codex and runtime-controlled internal retries; inherited D-127/D-128 and process-local build workaround. No new framework, dependencies or broad logging platform. D-125/M1/M2 remain parked.

## Roadmap findings

Owner manual QA is next. No graph/Structured work, provider expansion or publication is implied. Existing provider/runtime live-success limitations remain; new offline isolation evidence does not prove a live authenticated turn.

## Completion decision

PASS WITH ADVISORIES. All required implementation and available automated checks passed; ordinary schema/finalization and full Stop must validate the frozen report. Implementation completion is distinct from owner manual QA and authenticated live success.

## Next-increment readiness

Ready with advisories for owner manual QA only; require explicit provider/model/effort, request allowance and acknowledgement before any Send.

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
- `docs/plans/2026-09-30-provider-milestone.md`
- `docs/provider-milestone.md`
- `docs/reviews/2026-09-30-provider-milestone-post-increment-review.md`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/agent_preferences.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/storage/agent_preferences.rs`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`

## Exact commands executed

- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip="none"'` — Passed.
- `sh /private/tmp/cortexa-provider-milestone-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline --config 'profile.release.build-override.strip="none"'` — Passed.
- `python3 -B /private/tmp/cortexa-provider-milestone-evidence/isolation_probe_final.py` — Passed.
- `python3 -B /private/tmp/cortexa-provider-milestone-evidence/check-preservation.py` — Passed.
- `python3 -B .codex/hooks/session_end_gate.py` — Passed.

- `npm run docs:check` — Passed.
- `npm run repository:check` — Passed.
- `npm run security:scan` — Passed.
- `git diff --check` — Passed.

No application, provider, install, commit or publication command was run. Historical unsuccessful attempts are retained in external logs; the final required commands above passed after their corrections. The final report/marker and Stop receipts are external gate results, not invented prior execution.
