# Private agent workflow internals decomposition post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment agent-workflow-internals-decomposition",
    "npx prettier --write docs/plans/2026-08-12-agent-workflow-internals-decomposition.md",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_infrastructure_operations_workflow_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
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
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md",
    "docs/increments/agent-workflow-internals-decomposition.md",
    "docs/plans/2026-08-11-workflow-automation.md",
    "docs/plans/2026-08-12-agent-workflow-internals-decomposition.md",
    "docs/reviews/2026-08-13-agent-workflow-internals-decomposition-post-increment-review.md",
    "docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "src-tauri/src/agent/infrastructure_operations.rs",
    "src-tauri/src/agent/infrastructure_operations/catalog.rs",
    "src-tauri/src/agent/infrastructure_operations/framing.rs",
    "src-tauri/src/agent/infrastructure_operations/validation.rs",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/src/agent/orchestrator/infrastructure_operations_workflow.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "During a separately approved future workflow increment",
      "risk": "The 6,817-line facade and 2,664-line private D-088 lifecycle remain substantial; adding another workflow inline would recreate review coupling.",
      "severity": "Advisory",
      "summary": "Preserve separate private workflow ownership and do not introduce a general workflow engine."
    }
  ],
  "increment_id": "agent-workflow-internals-decomposition",
  "manual_verification": [
    {
      "check": "The complete diff changes only private Rust source ownership and documentation; it adds no public authority, behavior change, workflow engine, Workflow Automation activation, tool, policy, approval, execution, I/O, dependency, provider, IPC/UI, persistence, parallelism, external runtime, or device effect.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Manual application or target-environment validation (not required because the Rust core remains unwired and the increment is behavior-preserving and no-I/O)",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_infrastructure_operations_workflow_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
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
Increment: agent-workflow-internals-decomposition
Branch: main

## Executive summary

D-089's behavior-preserving private source decomposition is implemented and
the independent quality result is `PASS WITH ADVISORIES`. The D-088 lifecycle
now resides in one private orchestrator child module, while fixture catalog,
transfer framing, and strict parser/validation bodies reside in three private
domain modules. No public contract, behavior, or authority changed. Final
repository closeout checks pass and the deterministic completion marker is
ready for finalization.

## Scope and boundaries

The complete 20-path change set matches D-089. It contains the six exact Rust
source paths plus planning/current-state/architecture, increment, and review
documentation. No test expectation, fixture, manifest, lockfile, frontend,
Tauri configuration, permission, dependency, tool, policy, approval, runtime,
memory, document, provider, storage, or platform path changed.

All new modules are private. Parent-called lifecycle and domain helpers are no
wider than `pub(super)`. `AgentOrchestrator` retains task/run/event/child/
cancellation/selector authority. Workflow Automation remains Deferred and no
execution exists.

## Verification results

- Passed: final `npm run verify`, including 124 frontend tests, 195 Rust library
  tests, unchanged integration counts, TypeScript, Vite, strict Clippy, and the
  Tauri no-bundle release build.
- Passed: separate all-target Rust, 362 passed with one intentionally ignored
  opt-in real-Hermes probe.
- Passed: focused public D-088 workflow contracts, 25/25.
- Passed: Rust formatting after source extraction.
- Failed then resolved: the first full verification attempt stopped only
  because the active plan needed Prettier. The exact targeted command formatted
  that file; the complete rerun passed.
- Passed: final documentation, repository, security, diff, and session-end
  checks after the last documentation edit.
- Not run and not required: manual application/target-environment validation,
  because the Rust core remains unwired and this increment adds no behavior or
  I/O.

## Architecture findings

PASS WITH ADVISORIES. The extraction improves private ownership without a
workflow trait, generalized state machine, DAG engine, registry, scheduler, or
new reusable execution authority. Public facades and type paths remain stable.
Native remains sole/default. The remaining module size is a future
maintainability advisory, not a D-089 completion blocker.

## Security findings

PASS. Semantic comparison and complete diff inspection found no validation
reordering, widened visibility, redaction change, mutation-order change,
cancellation change, audit change, capability expansion, secret source,
filesystem/network I/O, unsafe code, dependency, permission, provider, or
execution path. The model/runtime gains no authorization.

## Code-health findings

PASS. Every moved catalog/framing/validation function body is token-identical
to the baseline. Every moved lifecycle body matches after accounting only for
the private module/import path; the three new root-cancellation helpers are
equivalent extractions of the prior inline logic. Public D-088 tests retain all
25 assertions and pass without modification.

## Technical debt

- Advisory, maintainability: `orchestrator.rs` remains 6,817 lines and the
  private D-088 lifecycle is 2,664 lines. Risk: adding another workflow inline
  would recreate cancellation, capacity, and audit review coupling. Effort:
  Medium. Milestone: a separately approved future workflow increment. This
  blocks neither D-089 completion nor a fresh Workflow Automation readiness
  review, provided later work preserves separate private ownership and does not
  create a general engine.

D-088's exact `blocks_next_increment` decomposition finding is cleared:
`orchestrator.rs` is down from 9,424 to 6,817 lines and
`infrastructure_operations.rs` from 4,489 to 2,416 lines, with the exact private
lifecycle/catalog/framing/validation boundaries implemented.

## Roadmap findings

The D-089 source prerequisite is implemented, but Workflow Automation remains
Blocked. D-089 completion grants only a fresh readiness review. Automation
still lacks its own accepted decision, complete Ready plan, exact typed schema,
validated initial family, governance/approval/audit treatment, and deterministic
adversarial evidence. No queue item is promoted by this review.

## Completion decision

PASS WITH ADVISORIES. Every required check passes; finalize and validate the
deterministic completion marker without further file edits.

## Next-increment readiness

Blocked. Complete and validate D-089's deterministic marker, then conduct a
fresh Workflow Automation architecture, security, and readiness review. Do not
begin its implementation from this review.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/increments/agent-workflow-internals-decomposition.md`
- `docs/plans/2026-08-11-workflow-automation.md`
- `docs/plans/2026-08-12-agent-workflow-internals-decomposition.md`
- `docs/reviews/2026-08-13-agent-workflow-internals-decomposition-post-increment-review.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `src-tauri/src/agent/infrastructure_operations.rs`
- `src-tauri/src/agent/infrastructure_operations/catalog.rs`
- `src-tauri/src/agent/infrastructure_operations/framing.rs`
- `src-tauri/src/agent/infrastructure_operations/validation.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/orchestrator/infrastructure_operations_workflow.rs`

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py begin --increment agent-workflow-internals-decomposition`
  — Passed; gate is active.
- `npm run verify` — first attempt failed only at formatting for the active
  plan; after the targeted correction, the final complete rerun passed.
- `npx prettier --write docs/plans/2026-08-12-agent-workflow-internals-decomposition.md`
  — Passed; formatted the one reported plan.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check` — Passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_infrastructure_operations_workflow_contract --locked`
  — Passed, 25/25.
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked` —
  Passed, 362 passed and one intentionally ignored probe.
- `npm run docs:check` — Passed after final documentation edits.
- `npm run repository:check` — Passed after final documentation edits.
- `npm run security:scan` — Passed after final documentation edits.
- `git diff --check` — Passed after final documentation edits.
- `python3 .codex/hooks/session_end_gate.py` — Passed after final documentation
  edits with no conflicts or staged changes.
