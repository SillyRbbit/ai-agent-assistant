# Direct provider stream diagnostics review

Date: 2026-09-21. Increment: `direct-provider-stream-diagnostics`.
Workspace: `/private/tmp/cortexa-direct-provider-stream-diagnostics`.
Detached baseline: `0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "direct-provider-stream-diagnostics",
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
    "docs/plans/2026-09-21-direct-provider-stream-diagnostics.md",
    "docs/reviews/2026-09-21-direct-provider-diagnostics-test-sequencing-post-increment-review.md",
    "docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md",
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
    "python3 -B /private/tmp/cortexa-direct-stream-diagnostics-evidence/preserve.py"
  ],
  "verification": [
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
      "required": true,
      "status": "Failed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify",
      "required": true,
      "status": "Not run"
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
      "command": "python3 -B /private/tmp/cortexa-direct-stream-diagnostics-evidence/preserve.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Native app and provider rehearsal (no requests authorized)",
      "required": false,
      "status": "Not run"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "High",
      "summary": "Three new UI tests fail at an inherited static-message assertion.",
      "risk": "Required frontend validation failed; later assertions in those cases were not reached. Full verification is not run.",
      "effort": "Separate owner-approved read-only diagnosis and isolated corrective successor.",
      "milestone": "Before accepting these diagnostics.",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Native live success and other existing GUI/custody/abort/dependency advisories remain.",
      "risk": "All three live attempts are exhausted; the prior streamed failure cannot be classified retrospectively. No current production or distributed-use readiness claim.",
      "effort": "Separate owner authorization and bounded verification.",
      "milestone": "Before any additional live request or broader use.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

## Executive summary

FAIL / Blocked. Only the three approved nested ResponseError codes and static
messages were added, with offline regressions. Focused frontend: 15 passed,
3 new UI cases failed. Focused Rust, already started concurrently: 43 passed.
No executable repair or retry occurred. Full offline verification was not run.
No app launch, credential inspection, live request, commit or publication occurred.

## Scope and boundaries

The complete inherited 34-path candidate was transferred byte-for-byte into a
new isolated detached worktree before ordinary begin succeeded. The exact
15-path successor delta is frozen in the [plan](../plans/2026-09-21-direct-provider-stream-diagnostics.md);
cumulative inventory is 36 paths. Existing predecessor records and raw states
are unchanged, including terminal failure and completion records. No stale
copied debug bundle is represented as new native evidence.

The first sandboxed worktree command lacked permission to write Git metadata;
the explicitly authorized elevated command then created the new worktree.
No predecessor was recreated or removed. Copy-on-write caches were used without
package resolution, dependency downloads or credential inspection.

## Verification results

The new UI cases reached the terminal error state and displayed the correct
approved static message, but failed at PersonalAssistantDirectDemo.test.tsx:201.
That inherited assertion requires “No mock response was substituted.” The three
approved messages instead end “No automatic retry was made.” The existing
assertion was inadvertently reused for the new cases. All three failures share
this cause; no production failure or timer scheduling failure is established.
The later no-extra-poll/one-Start assertions in those cases were not reached.
The passing Rust run includes the new nested-code/redaction/fallback cases.
The frontend failure stops acceptance regardless of the Rust result.

The required frontend failure was observed after both focused checks had started.
The already-running Rust check finished before process-stop identification could
confirm a live task-owned process. No further implementation tests were started.
Only required failure-disposition documentation and preservation checks followed.
The exact statuses below are authoritative; Not run is never a passing result.

## Architecture findings

The two production deltas are a closed Rust enum/match and the static client
message map. No new layer, dependency, snapshot field, native command, host
behavior or production component. Existing parser checks precede classification.
Top-level error remains generic. Request/body/transport ownership is preserved.
Architecture review found no separate blocking change beyond failed acceptance.

## Security findings

No raw code string, message, parameter or identifier is exposed by the new errors.
The Rust match accepts exact strings only at response.error.code for response.failed;
all other values use the existing generic enum. Existing TLS, HTTP mappings,
limits, opt-in/credential handling, cancellation and no-retry policies are intact.
Static messages identify a reported category, not account, billing, retryability
or historical root cause. Rust redaction fixtures passed. Overall acceptance
remains failed; this is not a successful native/security verification claim.

## Code-health findings

The three new UI rows reuse a message assertion that is incompatible with their
approved static text. This is a concrete blocking test integration defect. Do
not change the approved production messages or weaken behavior checks to obtain
a pass. A later explicitly authorized isolated correction must preserve the
existing rows and verify the new exact messages plus cleanup/no-retry assertions.
No repair is made in this terminal increment. Client narrowing cases passed.

## Technical debt

High, completion/next-increment blocking: failed required frontend validation.
Risk, effort and milestone are in the manifest. Existing advisory: diagnostic
categories cannot recover discarded historical provider detail, and the generic
top-level error intentionally leaves provider-specific reasons unclassified.
Native success, remaining GUI checks and D-128/D-127 advisories remain inherited.

## Roadmap findings

D-125/M1/M2 stay parked. Zero live rehearsal attempts remain; historical one-attempt
entries are preserved as history, not current authority. No publication or new
live allowance follows. The next action is read-only diagnosis of this failure;
a corrective implementation requires its own owner approval and ordinary admission.

## Completion decision

FAIL. Close with the ordinary terminal failed disposition, no completion marker.
Do not reopen, rewrite or promote predecessor records. The failure is retained
without rollback, repair, retry, commit or publication.

## Next-increment readiness

Blocked. A separately approved read-only review may assess the minimal test
correction and successor admission. It must preserve all current production
bytes, reports, raw states and predecessor worktrees. No automatic successor.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md`
- `docs/plans/2026-09-21-direct-provider-diagnostics-test-sequencing.md`
- `docs/plans/2026-09-21-direct-provider-failure-diagnostics.md`
- `docs/plans/2026-09-21-direct-provider-stream-diagnostics.md`
- `docs/reviews/2026-09-21-direct-provider-diagnostics-test-sequencing-post-increment-review.md`
- `docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md`
- `docs/reviews/2026-09-21-direct-provider-stream-diagnostics-post-increment-review.md`
- `docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/Cargo.lock`
- `src-tauri/Cargo.toml`
- `src-tauri/src/agent/gateway_request.rs`
- `src-tauri/src/agent/native_runtime.rs`
- `src-tauri/src/agent/runtime.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/personal_assistant_direct_tauri.rs`
- `src-tauri/src/personal_assistant_v0.rs`
- `src/App.test.tsx`
- `src/features/conversations/ConversationWorkspace.tsx`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `src/features/conversations/PersonalAssistantDirectDemo.tsx`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`

## Exact commands executed

The gate manifest schema requires every verification command in commands_executed,
including planned commands whose explicit status is Not run. Only Passed/Failed
entries were actually invoked. Required failure-closeout checks do not change FAIL.

- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run test:frontend -- src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/features/conversations/PersonalAssistantDirectDemo.test.tsx` — Failed.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1` — Passed.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run verify` — Not run.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check` — Passed.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check` — Passed.
- `env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan` — Passed.
- `git diff --check` — Passed.
- `python3 -B .codex/hooks/session_end_gate.py` — Passed.
- `python3 -B /private/tmp/cortexa-direct-stream-diagnostics-evidence/preserve.py` — Passed.

Additional bounded operations: instruction/source/record reads; manifest comparison;
new detached worktree creation; candidate copy/byte comparison; ordinary begin;
copy-on-write cache reuse; scoped formatting; exact diff review; failure closeout.
No changes to production/tests followed the focused frontend failure.
