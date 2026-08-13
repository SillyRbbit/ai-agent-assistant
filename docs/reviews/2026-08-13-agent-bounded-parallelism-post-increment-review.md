# Bounded agent parallelism post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment agent-bounded-parallelism",
    "cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked bounded_parallel",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_bounded_parallelism_contract --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
    "npm run verify",
    "npx prettier --write ROADMAP.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "npx prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PRODUCT_REQUIREMENTS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md docs/PROJECT_DIRECTION.md docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md docs/increments/agent-bounded-parallelism.md docs/plans/2026-08-11-bounded-agent-parallelism.md docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md",
    "npx prettier --write docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "docs/PROJECT_DIRECTION.md",
    "docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md",
    "docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md",
    "docs/increments/agent-bounded-parallelism.md",
    "docs/plans/2026-08-11-bounded-agent-parallelism.md",
    "docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md",
    "docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md",
    "src-tauri/src/agent/bounded_parallelism.rs",
    "src-tauri/src/agent/bounded_parallelism/catalog.rs",
    "src-tauri/src/agent/bounded_parallelism/validation.rs",
    "src-tauri/src/agent/infrastructure_operations/framing.rs",
    "src-tauri/src/agent/mod.rs",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/src/agent/orchestrator/bounded_parallel_workflow.rs",
    "src-tauri/src/agent/task.rs",
    "src-tauri/tests/agent_bounded_parallelism_contract.rs",
    "src-tauri/tests/agent_runtime_contract.rs",
    "src-tauri/tests/support/mock_agent_runtime.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "High",
      "milestone": "Before any app-global or provider-backed concurrency increment",
      "risk": "Several live RuntimeRun values are multiplexed on one application thread, deadlines are cooperative, and limits are per AgentOrchestrator; this proves no provider/CPU concurrency, hard preemption, provider-session separation, or app-global capacity.",
      "severity": "Advisory",
      "summary": "D091-TD-04: require a separate plan for provider-backed or app-global concurrency."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Before another parallel workflow family",
      "risk": "The private bounded-parallel lifecycle module is 5,188 lines and would increase review coupling if extended.",
      "severity": "Low",
      "summary": "D091-TD-01: decompose private lifecycle ownership before concurrency or lifecycle expansion."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low",
      "milestone": "When the public error contract is next revised",
      "risk": "Six typed BoundedParallelError variants are not naturally reachable through the sealed production catalogs.",
      "severity": "Low",
      "summary": "D091-TD-02: resolve the six unused public variants before IPC or public-contract expansion."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "High",
      "milestone": "Before any live tool, provider, or effect path",
      "risk": "Natural-language authority-claim filtering is defense in depth and cannot authorize future effects.",
      "severity": "Advisory",
      "summary": "D091-TD-03: require deterministic trusted authorization before live/provider/effect reuse."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Before a live provider reuses the generic runtime-start path",
      "risk": "A pre-existing legacy start_runtime_run cancellation-error path may orphan a rejected runtime run; D-091's new paths quarantine their rejected identities but do not rewrite the legacy path.",
      "severity": "Medium",
      "summary": "LEGACY-TD-01: resolve rejected-run cleanup before live/external-runtime or legacy-provider work."
    }
  ],
  "increment_id": "agent-bounded-parallelism",
  "manual_verification": [
    {
      "check": "Final independent code review of the complete source diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Final independent architecture and security review of the complete diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Manual application check (not required because the Rust boundary remains unwired and adds no I/O or Tauri/React behavior)",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked bounded_parallel",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_bounded_parallelism_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
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
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-13
Increment: agent-bounded-parallelism
Branch: main

## Executive summary

D-091's bounded parallelism source is complete and its implementation checks
pass. One sealed fixture-only/no-I/O selector retains multiple independent
specialist runs and multiplexes events on one application thread. Stable
catalog ordinals determine dependency transfer, cancellation, result order,
and truthful Personal synthesis. Three explicit failure policies, cooperative
deadlines, resumable cancellation, rejected-run quarantine, and terminal
cleanup prevent recursive or uncontrolled work.

Final code, architecture, security, and technical-debt reviews are `PASS WITH
ADVISORIES` with no completion blocker. The final quality result is `PASS WITH
ADVISORIES`. Post-documentation repository checks pass, and deterministic
finalization reports a complete, valid marker.

## Scope and boundaries

Production changes add one closed public domain with private catalog/validation
children, one private orchestrator lifecycle, exact facade/module wiring, the
typed `DeadlineExceeded` task failure code, and one exhaustive D-088 framing
arm. Tests add the public D-091 contract and bounded mock/runtime regression
instrumentation. The exact 30-path inventory is in the manifest.

No runtime trait or Native implementation, provider, thread, async executor,
general workflow engine, tool, policy permission, approval dispatch, durable
audit or memory, storage, I/O, dependency, configuration, Tauri/React IPC, UI,
remote worker, distributed infrastructure, or device effect changed.

## Verification results

- Passed: focused bounded-parallel library tests, 41/41.
- Passed: public D-091 contract, 41/41.
- Passed: Rust formatting and strict all-target/all-feature Clippy.
- Passed: all-target Rust, 481 passed, 0 failed, one intentionally ignored
  opt-in real-Hermes probe.
- Passed: `npm run verify`, including 124 frontend tests and 249 passed Rust
  library tests with one intentionally ignored probe, all integration
  contracts, frontend builds, and Tauri no-bundle release build.
- Passed: preliminary session-end inventory with no staged files or conflicts.
- Passed after documentation closeout: documentation formatting/links,
  repository health, security scan, diff hygiene, and final session-end
  inventory. Prettier changed only `ROADMAP.md` and the native roadmap.
- Not required: manual application validation because the core remains unwired
  and adds no UI, IPC, provider, I/O, or device behavior.

## Architecture findings

Independent architecture review is `PASS WITH ADVISORIES` with no completion
blocker. Application-owned identity, catalog, ordering, admission, limits,
deadlines, cancellation, quarantine, result validation, attribution, and
synthesis remain outside untrusted runtime output.

The material constraints are explicit: same-thread multiplexing is not
simultaneous provider or CPU work; cooperative deadlines cannot hard-preempt a
synchronous call; and per-orchestrator bounds are not an app-global provider
capacity coordinator.

## Security findings

Independent security review is `PASS WITH ADVISORIES` with no completion
blocker. The reviewer confirmed that no runtime, provider, tool, approval,
document, memory-sharing, I/O, or effect authority was added. Strict structured
claims and absent effect paths remain the security boundary; natural-language
claim filtering is defense in depth only.

## Code-health findings

Final code review is `PASS WITH ADVISORIES`. The private parallel lifecycle is
large and should not be extended without a separate decomposition plan. Six
public error variants are not naturally reached through the sealed built-ins
and may be narrowed when the contract is next revised.

## Technical debt

Technical-debt review is `PASS WITH ADVISORIES`. D091-TD-01 through TD-04 and
LEGACY-TD-01 are recorded in the manifest. D-091's rejected-run paths are
quarantined, but the pre-existing legacy `start_runtime_run`
cancellation-error orphan risk remains out of scope and must be resolved before
a live provider relies on that path.

## Roadmap findings

No successor is owner-selected or Ready. Provider-backed or app-global
concurrency, hard preemption, another workflow family, IPC/UI, tools/effects,
and scheduling remain separately gated.

## Completion decision

`PASS WITH ADVISORIES`. Source verification, independent reviews, and all
post-documentation checks pass. Deterministic finalization completed and the
marker reports `complete`, `valid: true`, and `PASS WITH ADVISORIES`.

## Next-increment readiness

Blocked. No owner-selected Ready plan follows D-091. Live/provider/app-global
concurrency, hard preemption, another workflow family, IPC/UI, tools/effects,
and scheduling require separate decisions and Ready plans.

## Exact files changed

The machine manifest above lists the exact 30-path Git change set: 19
documentation paths, 8 production Rust paths, and 3 Rust contract-test paths.
No manifest, lockfile, dependency, Tauri capability, frontend, runtime-trait,
provider, persistence, or external-integration path changed.

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py begin --increment agent-bounded-parallelism`
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`
- `cargo test --manifest-path src-tauri/Cargo.toml --lib --locked bounded_parallel`
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_bounded_parallelism_contract --locked`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings`
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked`
- `npm run verify`
- `npx prettier --write ROADMAP.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `npx prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PRODUCT_REQUIREMENTS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md docs/PROJECT_DIRECTION.md docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md docs/increments/agent-bounded-parallelism.md docs/plans/2026-08-11-bounded-agent-parallelism.md docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md`
- `npx prettier --write docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 .codex/hooks/session_end_gate.py`

The deterministic completion marker was finalized and validated after these
checks.
