# Research/Knowledge demo lifecycle Tauri adapter post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri",
    "cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract",
    "npm run test:frontend",
    "npm run test:repository",
    "npm run test:agent-acceptance",
    "npm run verify",
    "npm run security:scan",
    "git diff --check",
    "npm run tauri -- dev",
    "npm run docs:check",
    "npm run repository:check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/research-knowledge-demo-lifecycle-tauri-adapter-planning.md",
    "docs/increments/research-knowledge-demo-lifecycle-tauri-adapter.md",
    "docs/plans/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter.md",
    "docs/reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-blocked-post-increment-review.md",
    "docs/reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-planning-post-increment-review.md",
    "docs/reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/approvals/manager.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs",
    "src-tauri/tests/research_knowledge_demo_lifecycle_tauri_contract.rs",
    "src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts",
    "src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Small",
      "milestone": "Connected presentation increment",
      "risk": "The notification-only lifecycle event is covered by unit and static tests but was not directly observed in the running unconnected application.",
      "severity": "Advisory",
      "summary": "Direct native lifecycle command/event observation is Not run because no approved UI surface connects the client."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Small to medium",
      "milestone": "Before connected presentation implementation",
      "risk": "Connecting the current success-only production host without a separate design could overstate the fixture-only deterministic failure proof as interactive Rust behavior.",
      "severity": "Medium",
      "summary": "The deterministic failure path remains fixture-only and needs a separately approved application-owned design before a connected interactive demo."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "None unless recurring",
      "milestone": "Monitor",
      "risk": "A transient Cargo incremental-cache race can create a false-negative local run.",
      "severity": "Advisory",
      "summary": "One dep-graph.part.bin failure passed on exact unchanged retry without cache deletion or source workaround."
    }
  ],
  "increment_id": "research-knowledge-demo-lifecycle-tauri-adapter",
  "manual_verification": [
    {
      "check": "Target-Mac native development startup and visual no-permission-prompt inspection",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Direct observation of the deliberately unconnected lifecycle commands and notification event",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract",
      "required": true,
      "status": "Passed"
    },
    {"command": "npm run test:frontend", "required": true, "status": "Passed"},
    {"command": "npm run test:repository", "required": true, "status": "Passed"},
    {"command": "npm run test:agent-acceptance", "required": true, "status": "Passed"},
    {"command": "npm run verify", "required": true, "status": "Passed"},
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "npm run tauri -- dev", "required": true, "status": "Passed"},
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm run repository:check", "required": true, "status": "Passed"},
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-28
Increment: research-knowledge-demo-lifecycle-tauri-adapter
Branch: codex/research-knowledge-demo-lifecycle-tauri-adapter-planning

## Executive summary

PASS WITH ADVISORIES. The owner-approved increment implements and verifies the
smallest unconnected Tauri boundary around one existing application-owned,
volatile Research -> Knowledge host. Four no-caller-input commands return only
the closed content-free snapshot/error contract, and one fixed event publishes
notification-only copies after committed mutations. The runtime-narrowed client
remains unconnected to React. All required automated and target-Mac startup
checks pass; direct observation of the unconnected event is Not run.

## Scope and boundaries

The final 20-path inventory matches the approved adapter, separately approved
private Send prerequisite, F-12 tests, plan/increment/current-memory records,
and review evidence. The prerequisite adds only `ApprovalClock: Send`, a
deterministic Send-safe test clock, and compile-time assertions; it changes no
public approval interface, policy, or production clock behavior.

No caller chooses an agent, task, run, profile, runtime, workflow, fixture,
outcome, or objective. No React connection, provider, model, network,
credential, tool execution, approval dispatch, audit/persistence, filesystem,
background autonomy, capability/CSP/permission change, dependency, or device
effect was added.

## Verification results

- Focused adapter command: Passed, 5 passed, 0 failed, 0 ignored; 262 library
  tests filtered out.
- Public adapter contract: Passed, 1 passed, 0 failed, 0 ignored.
- `npm run test:frontend`: Passed, 17 files and 277 tests.
- `npm run test:repository`: Passed, 61 tests.
- `npm run test:agent-acceptance`: Passed, 267 library plus 207 selected
  integration tests; 474 passed, 0 failed, 0 ignored.
- `npm run verify`: Passed with 28 hook tests, 61 repository tests, 277 frontend
  tests, 267 Rust library tests, 244 Rust integration tests, one intentional
  ignored Hermes probe, production frontend build, and Tauri release no-bundle
  build.
- `npm run security:scan`: Passed with no secret finding.
- `git diff --check`: Passed.
- Target-Mac `npm run tauri -- dev`: Passed. Vite and Rust started, the native
  Cortexa window was visible through approved tooling, and no permission prompt
  appeared.
- Direct observation of the unconnected lifecycle commands/event: Not run;
  there is intentionally no approved connected UI surface.
- Warning: one focused attempt hit the previously documented Cargo incremental
  `dep-graph.part.bin` race before source diagnostics; exact unchanged retry
  passed. No cache was deleted and no workaround was introduced.

## Architecture findings

PASS WITH ADVISORIES. A single Tauri-managed mutex owns exactly one lifecycle
host. Mutation and snapshot derivation occur while locked; the lock is released
before emission. Notification failure cannot roll back the host and returns one
closed error so explicit snapshot recovery remains authoritative. The client is
unconnected and introduces no general event bus, worker, timer, queue, second
host, or autonomous behavior.

The current production host deterministically succeeds; failure remains a
private fixture-only core proof. That is truthful for this increment but blocks
readiness for a connected presentation until a separate plan defines an
application-owned failure selection without caller-selected trusted outcome.

## Security findings

PASS. Tauri injects only `AppHandle` and managed state; the WebView supplies no
command payload. Returned and emitted DTOs contain only schema/scenario,
disclosure/proof boundary, fixture provenance, numeric correlation, closed
state, and bounded journal enums. No internal identities, content, paths, URLs,
reasoning, upstream errors, credentials, or authority are serialized.

F-01 identity validation and F-02 rejected-run cleanup remain inside the host;
F-07 production/development CSP and capabilities are unchanged; F-08-style
runtime narrowing rejects malformed, extra, stale, contradictory, concurrent,
and gapped data; F-12 freezes exact commands/imports/listener/emitter and
prohibited tokens. Secret scanning passes, and no unsafe Rust or new external
surface exists.

## Code-health findings

PASS. Rust operations share one small mutation helper, typed closed errors, and
focused tests for success, cancellation, event failure, lock poisoning, and
threading bounds. TypeScript constructs fresh frozen values, rejects concurrent
requests instead of queueing, ignores stale/duplicate notifications, marks gaps
for explicit recovery, and disposes its one listener idempotently. Strict
formatting, lint, Clippy, typecheck, tests, and build pass without suppression.

## Technical debt

- Category: Roadmap truthfulness. Severity: Medium. Risk: a later UI could
  overstate fixture-only failure as interactive Rust behavior. Effort: Small to
  medium. Milestone: before connected presentation implementation. Blocks
  completion: No. Blocks next increment: Yes.
- Category: Manual verification. Severity: Advisory. Risk: the event is tested
  but not observed through the running unconnected app. Effort: Small once a UI
  is separately approved. Milestone: connected presentation increment. Blocks
  completion: No. Blocks next increment: No.
- Category: Tooling. Severity: Advisory. Risk: transient incremental-cache
  false-negative. Effort: None unless recurring. Milestone: Monitor. Blocks
  completion: No. Blocks next increment: No.

## Roadmap findings

Blocked. The current adapter increment is complete, but no connected
presentation increment is Ready. A separate owner-approved planning increment
must define explicit user-action wiring, preserve all current boundaries, and
reconcile the still fixture-only deterministic failure path before source work.
`NEXT_STEPS.md` is not reordered.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked. The exact next task is a separately approved documentation plan for
the connected presentation slice, including an application-owned deterministic
failure design. No source implementation is authorized.

## Exact files changed

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/increments/research-knowledge-demo-lifecycle-tauri-adapter-planning.md`
- `docs/increments/research-knowledge-demo-lifecycle-tauri-adapter.md`
- `docs/plans/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter.md`
- `docs/reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-blocked-post-increment-review.md`
- `docs/reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-planning-post-increment-review.md`
- `docs/reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs`
- `src-tauri/tests/research_knowledge_demo_lifecycle_tauri_contract.rs`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts`

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py begin --increment
research-knowledge-demo-lifecycle-tauri-adapter`
- `cargo fmt --manifest-path src-tauri/Cargo.toml`
- Focused prerequisite approval-manager and lifecycle-core tests.
- Focused adapter and client tests, typecheck, lint, and F-12 checks.
- `cargo test --manifest-path src-tauri/Cargo.toml
research_knowledge_demo_lifecycle_tauri`
- `cargo test --manifest-path src-tauri/Cargo.toml --test
research_knowledge_demo_lifecycle_tauri_contract`
- `npm run test:frontend`
- `npm run test:repository`
- `npm run test:agent-acceptance`
- `npm run verify`
- `npm run security:scan`
- `git diff --check`
- `npm run tauri -- dev`
- `npm run docs:check`
- `npm run repository:check`
- `python3 .codex/hooks/session_end_gate.py`

No commit, push, merge, release, publication, credential, capability, CSP,
permission, dependency, provider, or device-effect action occurred.
