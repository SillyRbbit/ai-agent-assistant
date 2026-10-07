# Live provider end-to-end post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npm run verify",
    "npm run test:frontend -- src/features/agents/AgentsPage.test.tsx",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked codex_connection::tests",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "/usr/bin/python3 -B .codex/hooks/session_end_gate.py",
    "/usr/local/bin/python3 -I -B /private/tmp/cortexa-live-provider-final-s9667wje/preserve.py"
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
    "docs/plans/2026-10-06-conversation-adapter-ownership.md",
    "docs/plans/2026-10-06-live-provider-e2e.md",
    "docs/reviews/2026-10-06-conversation-adapter-ownership-post-increment-review.md",
    "docs/reviews/2026-10-06-live-provider-e2e-post-increment-review.md",
    "src-tauri/src/agent_adapter.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/codex_connection.rs",
    "src-tauri/src/collaboration_tauri.rs",
    "src-tauri/src/diagnostics.rs",
    "src-tauri/src/diagnostics/tests.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/personal_assistant_v0.rs",
    "src/features/agents/AgentsPage.test.tsx",
    "src/features/agents/AgentsPage.tsx",
    "src/features/settings/diagnosticSummary.test.ts",
    "src/features/settings/diagnosticSummary.ts",
    "src/infrastructure/tauri/agent-chat-client.test.ts",
    "src/infrastructure/tauri/agent-chat-client.ts",
    "src/infrastructure/tauri/diagnostics-client.test.ts",
    "src/infrastructure/tauri/diagnostics-client.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.test.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.ts"
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Medium",
      "summary": "The existing 128-update journal can end modest but highly fragmented responses at a local resource bound.",
      "risk": "Usable partial output can end early; truthful resource_limit guidance and deliberate recovery now work. Caps were not raised.",
      "effort": "Medium; separately approved coalescing design and boundary/race tests.",
      "milestone": "Owner-selected journal usability increment; proposal C not approved.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Code health",
      "severity": "Low",
      "summary": "Codex setup hint still mentions only 0.159.0; discovery guidance and diagnostic panel mapping are generic/fixed.",
      "risk": "Users may misunderstand supported 0.160.1 setup or failure stage; no runtime fallback or authority is implied.",
      "effort": "Small bounded wording/mapping review, separately authorized.",
      "milestone": "Later owner-approved UX maintenance.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Retain D-127 audit debt and D-128 native custody and remote-abort limitations.",
      "risk": "Native Stop cannot guarantee remote termination or zero usage; current local diagnostics are not a production audit guarantee.",
      "effort": "Separate decisions and scoped work.",
      "milestone": "Existing D-127/D-128 follow-up; unchanged.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Dedicated Codex home/executable remain owner-controlled; internal Codex transport retries and account retention remain external behavior.",
      "risk": "Same-user modification is outside integrity guarantees; Cortexa does not resubmit or switch auth/runtime automatically.",
      "effort": "Keep documented setup and source-bound QA; broader hardening needs approval.",
      "milestone": "Existing runtime/isolation advisories.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Owner-private one-shot diagnostic is debug/macOS only and opt-in; Terminal/OS retention cannot be excluded.",
      "risk": "An unknown provider code could contain sensitive text; owner-only bounded escaped display is not zero-retention. Never copy private output into evidence.",
      "effort": "Retain explicit opt-in and inherited privacy tests; no expansion.",
      "milestone": "Existing accepted diagnostic exception; release path disabled.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Only minimum research collaboration was tested live per runtime; other variants remain not tested live.",
      "risk": "Do not infer all-route acceptance or sign-off for other providers/accounts/models.",
      "effort": "Separate bounded QA plan if owner selects broader coverage.",
      "milestone": "Minimum-route milestone scope retained; no waiver of a required row.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Near-completion native UI stability does not prove backend precompletion Stop delivery.",
      "risk": "Native observations cannot identify race ordering; deterministic controlled boundary tests supply distinct evidence.",
      "effort": "Reuse existing controlled race results; do not bypass disabled Stop.",
      "milestone": "Current acceptance evidence limitation, explicitly retained.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Inherited dependency/runtime, Node experimental localStorage, frontend chunk and native toolchain workaround advisories remain.",
      "risk": "No general production readiness, signing/notarization, toolchain compatibility or dependency remediation claim.",
      "effort": "Track through existing owner-selected maintenance.",
      "milestone": "Native process-local Python/Xcode SDK27/Cargo workaround; D-125/M1/M2 parked.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "increment_id": "live-provider-e2e",
  "manual_verification": [
    {
      "check": "Runtime separation \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Model and reasoning \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Normal completion \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Incremental streaming / terminal \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Follow-up context \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Active Stop / partial / recovery \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Near/repeated Stop \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Errors and recovery \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Restart persistence \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Per-bot isolation \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Collaboration \u2014 scoped API/Codex evidence reconciled; see table",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "A/B repaired-artifact discovery/completion/restart and verified test-owned cleanup",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Engineering/operations/workflow-proposal collaboration live variants outside minimum-route scope",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Backend ordering inferred from completion-edge native click timing",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:frontend -- src/features/agents/AgentsPage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked codex_connection::tests",
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
      "command": "/usr/bin/python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/usr/local/bin/python3 -I -B /private/tmp/cortexa-live-provider-final-s9667wje/preserve.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-10-07
Increment: live-provider-e2e
Branch: codex/live-provider-qa

## Executive summary

PASS WITH ADVISORIES for the approved bounded API/Codex minimum-route milestone.
Fresh required documentation checks passed; ordinary finalization, complete/valid
status and full Stop must confirm the frozen report before completion is asserted.

The owner-approved private minimum-route milestone repairs closed stream-error
classification and local-limit reporting, adds bounded private diagnostic tooling,
restores reviewed Codex protocol/catalog/file-store compatibility, and displays
saved effort truthfully before discovery. The inherited adapter-ownership candidate
is preserved. Native API and Codex core acceptance is supported by the evidence
below; there is no all-provider/all-workflow or production-readiness claim.

## Scope and boundaries

The cumulative inventory is exactly 31 paths: 30 preserved candidate paths and
this reserved final review. This stage modifies only nine existing documentation
paths additively and creates this report. Product/test/artifact bytes, previous
completion, historical reports, failure receipts, and original document bodies
are preserved. No credential, account, personal-profile, dependency, permission,
CSP/capability, gate implementation or product change occurs in closeout.

Earlier approved changes retain Rust ownership of validation/execution/cleanup;
WebView input and model/provider output remain untrusted. No new tools, device
execution, fallback, automatic Cortexa resubmission or larger resource limits.
External helper receipts are local evidence, not malicious same-user authentication.

## Verification results

Existing exact-source verification is reused under the owner's explicit closeout
authorization, which supersedes routine workflow instructions to repeat tests.
`npm run verify` exited zero after the final product changes: 105 hook, 94 repository,
601 frontend, 459 native library and 255 integration tests. One historical opt-in
Hermes probe remains ignored. Strict lint/Clippy/format/typecheck and frontend/native
builds passed. Focused AgentsPage 27 and Codex adapter 10 passed. The earlier stream
repair's initial TypeScript failure and later bounded correction remain historical;
this later A/B full verification passed and includes those unchanged corrections.

Evidence roots (outside Git; preserve originals):

- A/B verification: `/private/tmp/cortexa-codex-effort-repair-o41b458n`.
- API matrix: `/private/tmp/cortexa-native-acceptance-afvjqy21`.
- Earlier Codex matrix: `/private/tmp/cortexa-codex-file-store-wq7bovw_`.
- A/B native recheck: `/private/tmp/cortexa-codex-effort-recheck-3nvc11hq`.
- Final closeout: `/private/tmp/cortexa-live-provider-final-s9667wje`.

| Required criterion               | API evidence (inherited)                                                                                                                                                                                                                              | Codex evidence and affected recheck                                                                                                                                                                                                           |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Runtime separation               | Passed: native explicit OpenAI API route, no fallback                                                                                                                                                                                                 | Passed native: dedicated ChatGPT file-store Codex; explicit route, no API fallback.                                                                                                                                                           |
| Model and reasoning              | Passed: gpt-5.6-luna / low observed saved                                                                                                                                                                                                             | Passed native discovery: gpt-5.6-luna and low advertised, saved revision2; unloaded catalog display defect separately recorded.                                                                                                               |
| Normal completion                | Passed: multiple completed requests, including post-cancel OK                                                                                                                                                                                         | Passed native request22: OK; additional READY/CX73 and recovery OK.                                                                                                                                                                           |
| Incremental streaming / terminal | Passed: partial text observed while active; terminal correlated                                                                                                                                                                                       | Passed native request26: partial text while streaming, growing partial retained after Stop; sanitized lifecycle correlated.                                                                                                                   |
| Follow-up context                | Passed: QA42 recalled in next turn                                                                                                                                                                                                                    | Passed native request23/24: synthetic CX73 remembered across completed turns.                                                                                                                                                                 |
| Active Stop / partial / recovery | Passed: active Stop preserved partial; cancelled diagnostics; fresh OK afterward                                                                                                                                                                      | Passed request26 cancelled with partial preserved and runtime exit; request27 fresh OK completed.                                                                                                                                             |
| Near/repeated Stop               | Passed observed UI completion-edge stability: Stop attempted after final text while UI streaming, completed result preserved; two late disabled clicks unchanged. Backend precompletion Stop delivery not established; deterministic races inherited. | Passed observed UI stability request28: final 1..20 END while streaming then Stop attempt settled completed; late disabled Stop unchanged. Backend precompletion cancellation delivery not established; deterministic races inherited.        |
| Errors and recovery              | Passed classification/recovery: resource_limit fixed actionable message; fresh OK afterward. Modest longer output still reaches journal bound                                                                                                         | Earlier catalog failure recovered under explicit file-store wrapper; no unchanged retry. Runtime negative cases remain inherited automated evidence, not new native failure tests. Generic discovery guidance remains a usability limitation. |
| Restart persistence              | Passed native restart persistence of Personal Assistant; Research/Knowledge API roster retained                                                                                                                                                       | Passed launch002 native: PA/Research/Knowledge retained Codex/luna/low revision2 and MemoryOff; no Save/Send. PA unloaded catalog display issue reproduced; metadata restored low. Both research rooms retained.                              |
| Per-bot isolation                | Passed: Research/Knowledge changed separately; PA revision1 unchanged; six other profiles remain Simulation                                                                                                                                           | Passed direct: PA/Research/Knowledge changed independently to Codex/luna/low revision2, six other bots remain Simulation. Prior API profiles revision1 evidence retained.                                                                     |
| Collaboration                    | Passed minimum research flow: four actual API stages completed, validated handoffs and final synthesis                                                                                                                                                | Passed minimum research workflow room-2/run-1: four Codex stages completed, three validated handoffs and final synthesis. Other routes not tested live.                                                                                       |

The table quotes the earlier matrix's scope and limitations; A/B later supersedes
only its repaired gaps: actual installed Codex executable (no old store wrapper),
one successful catalog refresh preserving low without Save, one fresh completed OK
request, low support-unverified display before discovery, restart revision2 and
both cleanups passed. Detailed receipts bind PID/window identity and artifact;
owner foreground reports are separate from direct native AX/screenshot observations.
Both Cmd-Q exits were zero with independent BSD absence; no force kill. CUA remains
app-scoped, not CG-ID-directed. The corrected-interpreter preparation failure is
retained unchanged and was followed only by separately authorized validation.

Ledger33 = 9 historical + 8 API chat + 4 API workflow stages + 8 Codex chat +
4 Codex workflow stages. The A/B recheck added one Send and one separate catalog
refresh. No generation occurred during restart or closeout. The original historical
budget/receipts are not rewritten; later owner authorization expanded the QA run.

Near-completion and repeated Stop combine native terminal stability with controlled
automated race tests; native backend ordering is not claimed. Safe negative-case
coverage uses offline fixtures plus actual native failure/recovery; no deliberate
billing exhaustion, credential revocation or machine-network disruption. Other
collaboration routes are not required by the declared minimum research-route scope
and remain untested live, not passed. The ignored Hermes probe is not this scope.

## Architecture findings

Passed for the bounded candidate: source/diff review and unchanged prior review
evidence preserve adapter ownership and callback/lease boundaries. The inherited
agent_adapter module separates execution selection from chat/collaboration hosts;
no external runtime gains governance authority. LimitExceeded remains distinguishable
from protocol violations through direct host, callback, polling and diagnostics.
Fixed version allowlisting and bounded four-page/128-row catalog parsing retain
closed RPC/thread/tool validation and cleanup; no arbitrary runtime acceptance.
The UI fallback option displays an existing unverified effort without authorizing
it or triggering discovery. No unrelated architecture refactor was introduced.

## Security findings

Passed with retained advisories. Native credentials stay outside WebView/profile/
ordinary logs. Explicit dedicated file store uses cleared environment, fresh HOME,
exact owner-selected Codex home and fixed disabled tools/features; no personal-home
or API-key fallback. Fixed diagnostic categories, strict parser validation and
unknown-error behavior remain; no raw payload or private code is copied here.
The exceptional private sink is macOS debug-only, explicitly armed once, bounded
and escaped, fixed controlling TTY only, with no write retry/fallback and release
cfg exclusion. Existing synthetic tests cover cancellation/drop/short-write/privacy.
No security-policy, capabilities, dependency or production exposure changes.

## Code-health findings

Passed with advisories. Focused native boundary tests distinguish local limits,
malformed protocol and cancellation/completion, preserve partial text, prove repeated
terminal Stop stability and subsequent send in controlled fixtures. Cross-layer
closed error enums/static advice and diagnostic persistence/reload are exercised.
Five added UI cases cover low/high/default display, remount/bot isolation and
successful/failed/unsupported discovery without unintended Save. Native recheck
confirms the changed path, not every broader fixture. Existing comments/hints and
fixed diagnostic mapping limitations remain tracked rather than silently repaired.
Reviews were performed by the primary agent using current diff and hash-bound prior
reviews; no independent reviewer or new test execution is claimed.

## Technical debt

The manifest records each retained finding's category, severity, risk, effort,
owner-selected follow-up and blocking impact. No new Critical/High completion or
next-review blocker was identified. The 128-update journal can terminate fragmented
outputs early; classification/recovery is now truthful, but coalescing proposal C
remains unapproved. Other advisories include stale version guidance, diagnostic
mapping, D-127/D-128, Codex internal retries/retention/isolation, debug Terminal
custody, dependency/runtime/toolchain and Node/frontend-size warnings. No waiver,
automatic fix, new permission or resumed parked milestone follows from acceptance.

## Roadmap findings

Ready with advisories for a separately authorized read-only publication-readiness
assessment of the frozen candidate, completion records and live refs. No commit,
push, merge or publication is authorized. Roadmap priorities are not reordered;
D-125/M1/M2 remain parked and ECC hooks/MCP disabled. Other runtime/collaboration
coverage and journal coalescing require separate owner-selected bounded work.

## Completion decision

PASS WITH ADVISORIES. Every required verification/manual criterion in this bounded
report is Passed. Nonrequired broader live variants and native backend race-order
observability remain explicitly Not run, not waived required checks.

Historical failures and causes that remain unknown are preserved; acceptance
uses later criterion-specific evidence rather than rewriting failures as passes.
Only ordinary finalize after every required final check passes may create the
completion marker. Require complete/valid status and actual full Stop acceptance;
exit zero alone is insufficient. No exceptional admission or governance override.

## Next-increment readiness

Ready with advisories: read-only publication readiness, not publication itself.
No next implementation or broader QA starts automatically. Stop on ref/scope drift,
invalid evidence, a required uncovered criterion or new blocking review finding.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-06-conversation-adapter-ownership.md`
- `docs/plans/2026-10-06-live-provider-e2e.md`
- `docs/reviews/2026-10-06-conversation-adapter-ownership-post-increment-review.md`
- `docs/reviews/2026-10-06-live-provider-e2e-post-increment-review.md`
- `src-tauri/src/agent_adapter.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/collaboration_tauri.rs`
- `src-tauri/src/diagnostics.rs`
- `src-tauri/src/diagnostics/tests.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/personal_assistant_v0.rs`
- `src/features/agents/AgentsPage.test.tsx`
- `src/features/agents/AgentsPage.tsx`
- `src/features/settings/diagnosticSummary.test.ts`
- `src/features/settings/diagnosticSummary.ts`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`
- `src/infrastructure/tauri/diagnostics-client.test.ts`
- `src/infrastructure/tauri/diagnostics-client.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`

## Exact commands executed

Existing bound command receipts are reused, not rerun:

- `npm run verify` — Passed; retained A/B receipt.
- `npm run test:frontend -- src/features/agents/AgentsPage.test.tsx` — Passed; retained A/B receipt.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib --locked codex_connection::tests` — Passed; retained A/B receipt.

Fresh documentation-only closeout commands and outcomes are in the machine manifest
and external receipts. No product builds/tests or native QA ran here. Finalization
uses the system interpreter and the complete Stop payload. No bypass is permitted.
