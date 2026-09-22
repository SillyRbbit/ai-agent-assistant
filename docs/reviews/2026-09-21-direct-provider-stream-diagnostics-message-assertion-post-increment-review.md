# Direct provider stream diagnostics message assertion review

Date: 2026-09-21. Increment:
`direct-provider-stream-diagnostics-message-assertion`.
Workspace:
`/private/tmp/cortexa-direct-provider-stream-diagnostics-message-assertion`.
Detached baseline: `0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "direct-provider-stream-diagnostics-message-assertion",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md",
    "docs/plans/2026-09-21-direct-provider-diagnostics-test-sequencing.md",
    "docs/plans/2026-09-21-direct-provider-failure-diagnostics.md",
    "docs/plans/2026-09-21-direct-provider-stream-diagnostics-message-assertion.md",
    "docs/plans/2026-09-21-direct-provider-stream-diagnostics.md",
    "docs/reviews/2026-09-21-direct-provider-diagnostics-test-sequencing-post-increment-review.md",
    "docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md",
    "docs/reviews/2026-09-21-direct-provider-stream-diagnostics-message-assertion-post-increment-review.md",
    "docs/reviews/2026-09-21-direct-provider-stream-diagnostics-post-increment-review.md",
    "docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/Cargo.lock",
    "src-tauri/Cargo.toml",
    "src-tauri/src/agent/gateway_request.rs",
    "src-tauri/src/agent/native_runtime.rs",
    "src-tauri/src/agent/runtime.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/personal_assistant_direct_tauri.rs",
    "src-tauri/src/personal_assistant_v0.rs",
    "src/App.test.tsx",
    "src/features/conversations/ConversationWorkspace.tsx",
    "src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "src/features/conversations/PersonalAssistantDirectDemo.tsx",
    "src/infrastructure/tauri/personal-assistant-direct-client.test.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.ts"
  ],
  "commands_executed": [
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check",
    "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py",
    "python3 -B /private/tmp/cortexa-direct-stream-message-assertion-evidence/preserve.py",
    "python3 -B .codex/hooks/post_increment_gate.py finalize --increment direct-provider-stream-diagnostics-message-assertion --report docs/reviews/2026-09-21-direct-provider-stream-diagnostics-message-assertion-post-increment-review.md"
  ],
  "verification": [
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
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
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-direct-stream-message-assertion-evidence/preserve.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/post_increment_gate.py finalize --increment direct-provider-stream-diagnostics-message-assertion --report docs/reviews/2026-09-21-direct-provider-stream-diagnostics-message-assertion-post-increment-review.md",
      "required": true,
      "status": "Failed"
    }
  ],
  "manual_verification": [
    {
      "check": "Exact one test-expectation hunk; 11-path successor delta; 38-path cumulative inventory; production and predecessor preservation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Composed architecture, security, code-health, technical-debt and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native app and provider behavior",
      "required": false,
      "status": "Not run"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "High",
      "summary": "Ordinary completion rejected the report's noncanonical scope heading.",
      "risk": "The required completion marker cannot be written, regardless of the passing implementation tests and offline verification.",
      "effort": "Use a separately authorized isolated successor with ordinary admission; do not reopen this terminal record.",
      "milestone": "Before accepting or publishing this candidate.",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Native live success and remaining native GUI evidence are unverified.",
      "risk": "All three approved live attempts are exhausted; offline regressions do not prove provider connectivity, authorization or successful streaming.",
      "effort": "Require a separately approved future evidence plan before any additional live request.",
      "milestone": "Before claiming successful live-demo verification.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Existing D-128 custody/abort limitations and D-127 dependency baseline remain inherited.",
      "risk": "Session credentials are not durable secure storage; local abort cannot prove remote billing stops. Dependency evidence is inherited rather than refreshed by this test-only correction.",
      "effort": "Retain the existing disclosures and separate authorization boundaries.",
      "milestone": "Before broader data, distribution or dependency changes.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

## Executive summary

FAIL / Blocked. The terminal predecessor remains immutable. Its three new
UI cases now pass after the single authorized expectation correction: every row
declares its expected policy suffix and the shared assertion consumes that value.
All 18 focused frontend tests, 43 focused Rust tests and complete offline
verification passed. Ordinary completion rejected the report because its scope
heading was not the exact required phrase. Per the stop condition, no repair or
retry occurred. No production, dependency, Rust, client-test or component byte
changed. No app launch, credential inspection, provider request, commit or
publication occurred. All live attempts remain exhausted.

## Scope and boundaries

The successor was created detached at the required baseline. The terminal
candidate's exact 36 paths were transferred byte-for-byte without its raw gate
state before ordinary admission. The successor delta is exactly the authorized
11 paths; the cumulative inventory is 38 paths. Predecessor checkouts, finalized
reports and recorded raw states remain unchanged. The two already-prunable
registry entries remain recorded and were not inspected, pruned, repaired or
recreated.

The only executable difference from the failed candidate is in
`PersonalAssistantDirectDemo.test.tsx`. Six existing parameterized rows gained
an expected policy-message value, and the one shared suffix assertion consumes
it. Generic `network`, `http_status` and `provider_stream` rows retain “No mock
response was substituted.” The three narrowed stream-code rows retain “No
automatic retry was made.” Every other assertion is unchanged.

## Verification results

- Focused frontend: two files, 18 passed. The three narrowed-code cases now reach
  and pass every inherited streaming, exact-message, partial-output, terminal
  cleanup, control-release, one-Start and no-extra-poll assertion.
- Focused Rust Personal Assistant: 43 passed, covering closed code mapping,
  redaction, malformed/unrecognized fallback, transport and ownership behavior.
- Complete offline verification: passed. Formatting, repository rules, ESLint,
  strict Clippy, 74 hook tests, 83 repository tests, 389 frontend tests, 314 Rust
  unit tests and integration suites, TypeScript, Vite and Tauri no-bundle release
  build succeeded. The build is not a native launch or live-provider result.
- Final documentation, repository, secret, whitespace, session and preservation
  checks passed. No download, credential read, sensitive output or live request
  was used as evidence.
- Ordinary completion: failed. The report used `## Scope and preservation`, while
  the gate requires exactly one `## Scope and boundaries` section. This report
  records the failure; it does not repair and retry the increment.

## Architecture findings

No architecture change occurred. The correction only states the already approved
UI expectations. Request construction, networking, validation, retry policy,
ownership, IPC, dependencies and permissions are byte-identical to the terminal
candidate.

## Security findings

No trust-boundary change occurred. Static payload-free error codes and messages
remain production-owned. The prior generic native provider failure cannot be
retrospectively narrowed, and no live success is inferred from offline fixtures.
All existing advisories remain.

## Code-health findings

The failed expectation represented a test-table mismatch, not a timer,
production, transport or ownership defect. Explicit row values keep the six
closed outcomes readable without weakening or deleting any behavioral assertion.
No new abstraction, timeout, skip, mock fallback or production workaround was
introduced. The completion-report schema failure blocks acceptance.

## Technical debt

No implementation debt was added. The invalid completion-report heading is an
evidence defect and requires a separate isolated successor rather than reopening
this terminal increment.

## Roadmap findings

D-125/M1/M2 remain parked. All live attempts remain exhausted. No provider,
publication or roadmap work starts automatically from this failed successor.

## Completion decision

FAIL. Passing implementation checks do not substitute for the required valid
completion report and marker. This increment is closed terminal failed without
repair or retry.

## Next-increment readiness

Blocked. The corrective successor is not ready for acceptance or
publication. A separately authorized isolated successor may reuse the verified
candidate and correct only the completion-report schema under ordinary admission.
No additional provider rehearsal is ready: all three approved requests are
exhausted. D-125/M1/M2 remain parked. Both failed records remain `FAIL / Blocked`
historical evidence and are not promoted.

## Exact files changed

The cumulative inventory is the exact 38-path `files_changed` manifest above.
Relative to the terminal candidate, the successor delta is exactly the 11 paths
frozen in the plan. No path outside that delta differs from the candidate.

## Exact commands executed

The exact commands and their authoritative statuses are recorded in the
`commands_executed` and `verification` manifest entries above. The ordinary
finalize command failed; no finalize retry occurred.
