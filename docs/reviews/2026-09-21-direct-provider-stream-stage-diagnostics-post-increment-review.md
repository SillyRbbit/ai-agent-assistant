# Direct provider stream stage diagnostics post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "direct-provider-stream-stage-diagnostics",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-09-21-direct-provider-stream-stage-diagnostics.md",
    "docs/reviews/2026-09-21-direct-provider-stream-stage-diagnostics-post-increment-review.md",
    "src-tauri/src/personal_assistant_direct.rs",
    "src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "src/infrastructure/tauri/personal-assistant-direct-client.test.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.ts"
  ],
  "commands_executed": [
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked --offline personal_assistant_direct",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/preserve.py --final",
    "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/invariants.py",
    "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/bundle.py",
    "python3 -B .codex/hooks/session_end_gate.py",
    "python3 -B -",
    "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/validate_report.py"
  ],
  "verification": [
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked --offline personal_assistant_direct",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run tauri -- build --debug --bundles app --no-sign -- --locked --offline",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/preserve.py --final",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/invariants.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/bundle.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B -",
      "required": false,
      "status": "Failed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-direct-provider-stream-stage-evidence/validate_report.py",
      "required": true,
      "status": "Failed"
    }
  ],
  "manual_verification": [
    {
      "check": "Independent architecture, security, code-health, technical-debt and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact 15-path scope, protected byte identity, historical suffixes, 16 predecessor worktrees and two prunable registry entries",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native live request, streaming and completion",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Medium",
      "summary": "The passing-completion report failed schema validation after two external validation corrections; work stops with a terminal FAIL report.",
      "risk": "Passing application tests do not supply valid completion evidence or authorize the live request while the required gate is unmet.",
      "effort": "Read-only review and separate owner selection of any bounded evidence closeout; preserve this terminal record.",
      "milestone": "Before any further implementation or native rehearsal.",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Native live success remains unverified; one separately approved request is unused.",
      "risk": "Offline classification tests cannot establish the historical provider cause or successful live completion.",
      "effort": "One owner-private native launch and acknowledged fixed-sample Start, then stop on failure.",
      "milestone": "Before claiming native live success.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Inherited D-128 custody/abort limits, D-127 audit debt and remaining GUI smoke remain.",
      "risk": "This diagnostic-only change does not remove existing security/platform limitations.",
      "effort": "Retain existing evidence and separately scope any remediation.",
      "milestone": "Before broader usage or publication.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "D-125/M1/M2 remain parked.",
      "risk": "No prerequisite or completion claim for those lanes follows from this work.",
      "effort": "No action in this increment.",
      "milestone": "Separate owner selection required.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-09-21
Increment: direct-provider-stream-stage-diagnostics
Branch: detached at 3f99165b4dcb0ef18c52f9242b346eb1d711afaa

## Executive summary

FAIL / Blocked. The three closed static diagnostics and application tests passed, but the passing-completion report failed schema validation. After two external validation corrections, work stopped before the separately approved request. The historical provider cause remains unknown. This terminal failure report preserves successful application evidence and the failed completion attempt; it does not claim completion or live success.

## Scope and boundaries

Exactly 15 changed paths from detached baseline 3f99165b4dcb0ef18c52f9242b346eb1d711afaa. Four existing code/test files, nine additive current-state documents and this new plan/report pair. Every predecessor checkout, report, gate state and marker, all historical document bodies and both prunable entries remain preserved. No governance, dependency, request, host, component, graph, workflow or permission change.

## Verification results

Passed: focused frontend 24 tests; focused Rust 13 tests; full offline npm run verify (74 hook, 83 repository, 395 frontend, 315 Rust library and 247 integration tests; one pre-existing integration test ignored), formatting, lint, typecheck and builds. Passed: locked offline app-only debug build with --no-sign; bundle identity/code/assets checks, documentation/repository/security/whitespace, preservation/invariants and session inventory.

Failure sequence: an extra inline bundle assertion incorrectly searched native bytes for unused Rust Display strings. External correction 1 checked emitted enum codes and frontend messages instead; bundle.py passed. The report-validation wrapper then failed with ModuleNotFoundError: common. External correction 2 added the existing hook directory to its import path. The rerun reached the repository schema and failed with finding has an invalid category because this draft used Readiness instead of one of the five permitted finding categories. No application test failed. The two-correction allowance was conservatively treated as exhausted and no third passing repair was attempted.

For truthful terminal closeout only, this unfrozen report now uses a valid Roadmap category and records the failed required validation with FAIL / Blocked. This is not a passing finalization or predecessor rewrite. Documentation and preservation checks must be rerun on this final failure disposition, followed by read-only validate_failed_report, ordinary close-failed, valid failed status and Stop. Those disposition results are observed separately after this report is frozen; no successful outcome for them is preclaimed.

## Architecture findings

Passed independent review. Existing typed error enum and client map carry closed event-origin/code-shape categories. No new API or generic diagnostic metadata. Request construction, transport, credential handling, decoder validation, host, IPC and production component bytes are unchanged. Rust retires the now-unreachable generic variant; the client retains legacy provider_stream compatibility.

## Security findings

Passed independent review. No raw code, message, body, identifier, header or credential is retained by diagnostics. Exact known ResponseError meanings are preserved and never inferred for top-level errors. Tests verify serialization, Display/Debug, malformed shapes and client/UI redaction. All test/build processes explicitly unset OPENAI_API_KEY and disable the live opt-in. No credential, process environment, Terminal content or provider request was inspected or made. Existing D-128 limitations and D-127 advisories remain.

## Code-health findings

Passed independent review. The production change consists of two existing match branches and three enum/static-map categories; tests preserve validation precedence, frame boundaries, explicit completion, streaming, cleanup, released controls, partial-output labels and no retry. New UI rows reuse the corrected separate 250ms acts without changing behavioral assertions.

## Technical debt

The report-schema failure blocks completion and further work. No new application correctness defect was found in independent review. Existing decoder chunk batching means no displayed text cannot prove no deltas reached the wire; output behavior was preserved. Inherited custody/abort, dependency and GUI advisories remain.

## Roadmap findings

D-125/M1/M2 remain parked. No successor is started automatically. The separately approved single native request remains unused and is not attempted with failed completion evidence. No model change, fallback, account/billing change or publication occurred.

## Completion decision

FAIL. The required passing-report schema check failed after two external validation corrections. Preserve the tested implementation and rebuilt bundle; do not call finalize, launch the app or spend the request. Freeze this truthful terminal report and use only ordinary close-failed, then inspect valid failed status and Stop. No completion marker is claimed, and no historical record is reopened or rewritten.

## Next-increment readiness

Blocked. Any subsequent action first requires read-only review of this terminal failure and separate bounded owner direction. This record grants no successor or request authority. The additional request is unused; live success and remaining native GUI checks are unverified.

## Exact files changed

The machine manifest is the complete 15-path Git inventory; the plan lists each path. All other tracked source/configuration, dependencies, hooks/skills/harnesses and historical reports remain byte-identical. Generated local caches and bundles are ignored and are not tracked.

## Exact commands executed

The manifest records each executed verification command and its actual result. Focused frontend ran once on the unchanged baseline (18 passed) and once after the change (24 passed). Full verification and debug bundling each ran once. The optional failed python3 -B - row is the initial inline native-Display-string assertion. The required validate_report.py row records both its import failure and subsequent invalid-category rejection after the second external correction. No application suite is repeated for final failure-documentation updates. Final read-only failed-report validation and ordinary terminal disposition occur after the report freeze and are reported in the session rather than retroactively asserted here.
