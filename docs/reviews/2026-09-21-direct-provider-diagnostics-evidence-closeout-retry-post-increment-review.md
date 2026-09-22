# Direct provider diagnostics evidence closeout retry review

Date: 2026-09-21. Increment: direct-provider-diagnostics-evidence-closeout-retry.
Workspace: /private/tmp/cortexa-direct-provider-diagnostics-evidence-closeout-retry.
Detached baseline: 0ed15810e90b6a4bd312a8096c61b0abb1ab7eff.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "direct-provider-diagnostics-evidence-closeout-retry",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "files_changed": [
    "ARCHITECTURE.md", "CHANGELOG.md", "DECISIONS.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md", "PROJECT_STATUS.md", "ROADMAP.md", "SECURITY.md", "TESTING_GUIDE.md", "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md", "docs/plans/2026-09-21-direct-provider-diagnostics-evidence-closeout.md", "docs/plans/2026-09-21-direct-provider-diagnostics-evidence-closeout-retry.md", "docs/plans/2026-09-21-direct-provider-diagnostics-test-sequencing.md", "docs/plans/2026-09-21-direct-provider-failure-diagnostics.md", "docs/plans/2026-09-21-direct-provider-stream-diagnostics-message-assertion.md", "docs/plans/2026-09-21-direct-provider-stream-diagnostics.md",
    "docs/reviews/2026-09-21-direct-provider-diagnostics-evidence-closeout-post-increment-review.md", "docs/reviews/2026-09-21-direct-provider-diagnostics-evidence-closeout-retry-post-increment-review.md", "docs/reviews/2026-09-21-direct-provider-diagnostics-test-sequencing-post-increment-review.md", "docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md", "docs/reviews/2026-09-21-direct-provider-stream-diagnostics-message-assertion-post-increment-review.md", "docs/reviews/2026-09-21-direct-provider-stream-diagnostics-post-increment-review.md", "docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md",
    "scripts/repository_health.py", "scripts/tests/test_repository_health.py", "src-tauri/Cargo.lock", "src-tauri/Cargo.toml", "src-tauri/src/agent/gateway_request.rs", "src-tauri/src/agent/native_runtime.rs", "src-tauri/src/agent/runtime.rs", "src-tauri/src/lib.rs", "src-tauri/src/personal_assistant_direct.rs", "src-tauri/src/personal_assistant_direct_tauri.rs", "src-tauri/src/personal_assistant_v0.rs", "src/App.test.tsx", "src/features/conversations/ConversationWorkspace.tsx", "src/features/conversations/PersonalAssistantDirectDemo.test.tsx", "src/features/conversations/PersonalAssistantDirectDemo.tsx", "src/infrastructure/tauri/personal-assistant-direct-client.test.ts", "src/infrastructure/tauri/personal-assistant-direct-client.ts"
  ],
  "commands_executed": [
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check", "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check", "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan", "git diff --check", "python3 -B /private/tmp/cortexa-direct-diagnostics-evidence-closeout-retry-evidence/preserve.py", "python3 -B .codex/hooks/session_end_gate.py", "python3 -B /private/tmp/cortexa-direct-diagnostics-evidence-closeout-retry-evidence/validate_report.py"
  ],
  "verification": [
    {"command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check", "required": true, "status": "Passed"},
    {"command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check", "required": true, "status": "Passed"},
    {"command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 -B /private/tmp/cortexa-direct-diagnostics-evidence-closeout-retry-evidence/preserve.py", "required": true, "status": "Failed"},
    {"command": "python3 -B .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"},
    {"command": "python3 -B /private/tmp/cortexa-direct-diagnostics-evidence-closeout-retry-evidence/validate_report.py", "required": true, "status": "Not run"}
  ],
  "manual_verification": [
    {"check": "Executable, dependency, configuration, workflow, hook, skill and harness bytes match the predecessor candidate", "required": true, "status": "Not run"},
    {"check": "Architecture, security, code-health, technical-debt and readiness review of the documentation-only delta", "required": true, "status": "Not run"},
    {"check": "Native app and live provider behavior", "required": false, "status": "Not run"}
  ],
  "findings": [
    {"category": "Code health", "severity": "High", "summary": "The new preservation validator compares a string with byte status output.", "risk": "It raises TypeError before proving the retry scope and protected-byte invariants, blocking a completion marker.", "effort": "A separate authorized successor may correct only the external validator and rerun the frozen documentation matrix.", "milestone": "Before ordinary finalization.", "blocks_completion": true, "blocks_next_increment": true},
    {"category": "Security", "severity": "Advisory", "summary": "Native live success and existing GUI evidence remain unverified.", "risk": "All three approved requests are exhausted; historical offline evidence does not prove provider connectivity, authorization or successful streaming.", "effort": "Require a separately approved future evidence plan before another request.", "milestone": "Before claiming successful live-demo verification.", "blocks_completion": false, "blocks_next_increment": false}
  ]
}
-->

## Executive summary

FAIL / Blocked. This retry uses an APFS clone of existing local formatter tooling.
Documentation, repository, security and whitespace checks passed. The new
preservation validator reached earlier evidence then raised TypeError while
comparing a string with byte status output. Per the stop condition, no repair or
retry occurred. Historical application evidence remains 18 focused frontend tests,
43 focused Rust tests and full offline verification passed.

## Scope and boundaries

The retry delta is exactly ten documentation paths and the cumulative inventory is
42 paths. The inherited 40-path candidate was transferred byte-for-byte without
gate state. No executable, test, dependency, configuration, workflow, hook,
skill, harness, credential, provider, native or live-validation byte may change.

## Verification results

Documentation, repository, secret, whitespace and session checks passed.
Preservation failed with TypeError. Report-schema and manual reviews were not
run. Native app and live provider behavior are not run; all live attempts remain
exhausted.

## Architecture findings

Not run after the required preservation failure. No architecture change is claimed.

## Security findings

Not run after the required preservation failure. Existing native live-success,
GUI, D-128 custody/abort and D-127 dependency advisories remain.

## Code-health findings

The validator TypeError blocks scope and protected-byte review. No executable
code-health change is claimed.

## Technical debt

The predecessor formatter absence and unsafe absent-path helper remain historical.
This retry's byte/string comparison defect is recorded without modifying either
earlier failed record.

## Roadmap findings

D-125/M1/M2 remain parked and no new live work starts here.

## Completion decision

FAIL. Required preservation validation failed. This retry must be terminally
recorded and cannot be finalized.

## Next-increment readiness

Blocked. A separately authorized successor is required before another closeout
attempt.

## Exact files changed

The machine manifest records the full 42-path inventory. The retry delta is
exactly the ten paths named in the active plan.

## Exact commands executed

The machine manifest records current results. The preservation command failed and
no application command was rerun.
