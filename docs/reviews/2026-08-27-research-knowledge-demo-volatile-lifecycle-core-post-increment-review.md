# Research/Knowledge demo volatile lifecycle core post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment research-knowledge-demo-volatile-lifecycle-core",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked research_knowledge_demo_lifecycle",
    "cargo test --manifest-path src-tauri/Cargo.toml --locked --test research_knowledge_demo_lifecycle_contract",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "npm run test:agent-acceptance",
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "docs/increments/research-knowledge-demo-volatile-lifecycle-core-planning.md",
    "docs/increments/research-knowledge-demo-volatile-lifecycle-core.md",
    "docs/plans/2026-08-27-research-knowledge-demo-volatile-lifecycle-core.md",
    "docs/reviews/2026-08-27-research-knowledge-demo-volatile-lifecycle-core-planning-post-increment-review.md",
    "docs/reviews/2026-08-27-research-knowledge-demo-volatile-lifecycle-core-post-increment-review.md",
    "src-tauri/src/lib.rs",
    "src-tauri/src/research_knowledge_demo_lifecycle.rs",
    "src-tauri/tests/research_knowledge_demo_lifecycle_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "Separate security-sensitive plan",
      "milestone": "Before lifecycle Tauri IPC",
      "risk": "Treating the Drop sentinel as synchronization could allow concurrent hosts or ambiguous command ordering.",
      "severity": "Advisory",
      "summary": "The process-wide Drop sentinel is fail-closed replacement protection, not a concurrency coordinator."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low",
      "milestone": "When a lifecycle adapter is separately planned",
      "risk": "A fresh host snapshot remains locally idle during process quarantine even though every mutating operation returns cleanup-pending.",
      "severity": "Advisory",
      "summary": "Process availability is authoritative through closed operation errors, not a replacement host snapshot."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Small",
      "milestone": "Only if the fixed D-086 demo fixtures change",
      "risk": "The application-owned lifecycle script and existing acceptance fixtures could drift if one changes without the other.",
      "severity": "Low",
      "summary": "The fixed D-086 JSON outputs are duplicated across the lifecycle script and pre-existing acceptance tests."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Small",
      "milestone": "Before expanding lifecycle states, events, or journal capacity",
      "risk": "The containment path relies on the frozen transition count remaining below the eight-entry cap when it suppresses a journal-commit error.",
      "severity": "Low",
      "summary": "Fixed-cap containment assumptions must be revisited before lifecycle vocabulary expansion."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Owner decision and documentation planning",
      "milestone": "Before any lifecycle IPC or connected presentation",
      "risk": "Starting an adapter without a new plan would widen the trusted Tauri boundary and introduce synchronization/event-ordering requirements not reviewed here.",
      "severity": "Advisory",
      "summary": "No Tauri lifecycle adapter or connected presentation increment is Ready or authorized."
    }
  ],
  "increment_id": "research-knowledge-demo-volatile-lifecycle-core",
  "manual_verification": [
    {
      "check": "Pinned Node/npm/Rust toolchains on target macOS arm64 host",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Rendered UI, lifecycle IPC, viewport, theme, reduced motion, focus, scroll, zoom, and native resize",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "GitHub Actions for the uncommitted workspace",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked research_knowledge_demo_lifecycle",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --locked --test research_knowledge_demo_lifecycle_contract",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:agent-acceptance",
      "required": true,
      "status": "Passed"
    },
    {"command": "npm run verify", "required": true, "status": "Passed"},
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-27
Increment: research-knowledge-demo-volatile-lifecycle-core
Branch: `main`

## Executive summary

`PASS WITH ADVISORIES`. The owner-approved Rust-only prerequisite is complete:
one no-input, manually stepped host drives the existing sealed D-086 Research
-> Knowledge fixture through `NativeAgentRuntime` and projects only closed,
bounded, content-free lifecycle state. All required checks pass. No Tauri,
frontend, provider, tool, persistence, background, filesystem, dependency, or
device-effect boundary changed.

## Scope and boundaries

The workspace inventory includes the prior planning records, one private
crate-root lifecycle module, a minimal Rust-only re-export, one public contract
test, architecture/security/current-state reconciliation, and this report. The
host alone owns the objective, sources, script, run/request identities, runtime
envelopes, root, orchestrator, and cleanup. Test-only seams can select fixed
faults; production callers cannot.

The read-only Command Center projection, frontend fixtures, Conversations mock,
and sealed Rust workflow remain separate deterministic proofs. No command,
event, managed Tauri state, capability, CSP, permission, WebView client, React
consumer, timer, thread, poller, worker, provider, model, network, credential,
tool, approval dispatch, persistence, durable audit, filesystem, generic
workflow engine, dependency, or device effect was added.

## Verification results

- Lifecycle module: `Passed`, 9/9; 0 failed, 0 ignored.
- Public Rust-only lifecycle contract: `Passed`, 1/1; 0 failed, 0 ignored.
- Sealed-agent acceptance: `Passed`, 468/468; 0 failed, 0 ignored.
- Complete `npm run verify`: `Passed`; hook tests 28/28, repository tests 57/57,
  frontend tests 247/247 across 16 files, Rust library tests 261/261, Rust
  integration tests 243/243, and one explicit opt-in Hermes version probe
  ignored. Frontend production and Tauri release no-bundle builds passed.
- Rustfmt, strict all-target/all-feature Clippy, documentation, repository
  policy, secret scan, diff, and session-end checks: `Passed`.
- Target Mac: Node 26.3.0, npm 11.16.0, Rust/Cargo 1.90.0 on macOS 26.6 build
  25G72 arm64: `Passed`.
- Rendered UI, lifecycle IPC, viewport, theme, reduced motion, focus, scroll,
  zoom, and native resize: `Not run`; no such boundary changed.
- GitHub Actions for this exact workspace: `Not run`; the source is uncommitted
  and nothing was pushed or published.

## Architecture findings

`PASS WITH ADVISORIES`. The cohesive application service composes but does not
modify D-086 or agent internals. Production is sealed to `NativeAgentRuntime`;
the public Rust-only no-input contract avoids fake startup/Tauri wiring and a
broad dead-code suppression. A process-wide Drop sentinel permanently blocks
replacement operations after destructor cleanup failure, but is deliberately
not a mutex, scheduler, or future concurrent Tauri-state design.

## Security findings

`PASS WITH ADVISORIES`. Exact returned-runtime identity validation and
duplicate-live rejection remain centralized in `AgentOrchestrator`; the
sequential host cannot create an independent duplicate-live seam. Returned
identity and cancellation fault tests prove retained quarantine, blocked
restart, successful retry, no nonterminal destructor drop, and process-wide
replacement denial. DTOs/errors are finite and redacted. A future adapter must
retain exactly one synchronized managed host and separately review command and
event ordering.

## Code-health findings

`PASS WITH ADVISORIES`. Success, private fixture failure, cancellation at each
stage, idempotence, terminal late-step rejection, restart epoch, journal and
revision caps, closed serialization/errors, F-01 mismatch, F-02 retry, Drop
quarantine, and replacement denial are directly covered. A replacement host's
snapshot remains host-local and may be `idle` while all mutating operations
authoritatively return `cleanup-pending`; this is documented rather than
expanded into a process coordinator.

## Technical debt

No completion-blocking debt was introduced. The Drop sentinel's intentionally
limited role and the host-local replacement snapshot are recorded advisories
for a future separately planned adapter. Two Low items are deferred without a
speculative refactor: the fixed JSON script duplicates existing acceptance
fixtures, and the containment path relies on the frozen journal vocabulary
remaining below its cap. Neither grants authority to expand the current scope.

## Roadmap findings

`Blocked`. The lifecycle core is complete, but no Tauri lifecycle adapter or
connected Command Center presentation plan is owner-approved or Ready. The
smallest possible next action is a documentation/readiness review only.

## Completion decision

`PASS WITH ADVISORIES`.

## Next-increment readiness

`Blocked`. Do not begin lifecycle IPC, synchronization, events, client/UI
connection, or presentation without a separate Ready plan and owner approval.

## Exact files changed

The machine manifest lists the complete 17-file tracked and untracked workspace
inventory, including the prior planning records. No generated build output is
included.

## Exact commands executed

The machine manifest records every required focused, complete, documentation,
repository, security, diff, and session-end verification command. No commit,
push, merge, reset, rebase, release, deployment, credential, or publication
command ran.
