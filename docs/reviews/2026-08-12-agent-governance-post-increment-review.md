# Per-agent governance foundation post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "python3 .codex/hooks/post_increment_gate.py begin --increment agent-governance",
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment agent-governance --report docs/reviews/2026-08-12-agent-governance-post-increment-review.md",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::governance::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::governance::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
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
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "docs/PROJECT_DIRECTION.md",
    "docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md",
    "docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md",
    "docs/increments/agent-governance.md",
    "docs/plans/2026-08-11-agent-governance.md",
    "docs/reviews/2026-08-12-agent-governance-post-increment-review.md",
    "docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md",
    "docs/security/NATIVE_AGENT_GOVERNANCE_THREAT_MODEL.md",
    "src-tauri/src/agent/definition.rs",
    "src-tauri/src/agent/governance.rs",
    "src-tauri/src/agent/mod.rs",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/src/agent/task.rs",
    "src-tauri/src/approvals/decision_source.rs",
    "src-tauri/src/approvals/manager.rs",
    "src-tauri/src/approvals/types.rs",
    "src-tauri/src/audit/approval.rs",
    "src-tauri/src/audit/governance.rs",
    "src-tauri/src/audit/mod.rs",
    "src-tauri/src/policy/engine.rs",
    "src-tauri/tests/agent_governance_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Small focused contract additions",
      "milestone": "Before a later approval-source or execution-boundary expansion",
      "risk": "Sealed ownership and fail-closed code protect the current unwired boundary, but agent-origin source mismatch, approval-request creation failure, and every delegation error-record mapping are not each asserted directly.",
      "severity": "Advisory",
      "summary": "Three non-authorizing adversarial branches have indirect rather than dedicated agent-specific coverage."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Small publication step followed by a separately approved plan",
      "milestone": "Before Phase 5 memory or any later native-agent implementation",
      "risk": "Starting later work from an uncommitted governance baseline would mix increments and bypass its publication and readiness gates.",
      "severity": "Advisory",
      "summary": "The verified governance increment remains uncommitted and supported cross-target CI is post-publication evidence."
    }
  ],
  "increment_id": "agent-governance",
  "manual_verification": [
    {
      "check": "The complete diff contains no AgentRuntime or NativeAgentRuntime source/behavior change, dependency, manifest, lockfile, Tauri/React behavior, provider, executor, memory, credential, permission, process, network, filesystem, platform effect, Hermes integration, or specialist activation.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "No manual application check is required because the governance foundation is unwired and user-visible behavior is unchanged.",
      "required": false,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "python3 .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::governance::tests::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::governance::tests::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
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

Date: 2026-08-12
Increment: agent-governance
Branch: main

## Executive summary

Implemented D-084's bounded non-executing per-agent governance foundation.
Nine exact policy profiles and orchestrator-validated live attribution now bind
the synthetic agent policy, approval, delegation matrix, cancellation, and
volatile audit chain. Complete applicable validation and independent reviews
pass. The result is `PASS WITH ADVISORIES` because three sealed fail-closed
branches have indirect rather than dedicated coverage, publication remains
separate, and supported cross-target CI follows publication.

## Scope and boundaries

The exact 32-path inventory matches the approved ExecPlan. Production source is
limited to the agent definition/task/orchestrator/governance boundary, the
existing deterministic policy and approval components, and the closed legacy
and new governance audit modules. No runtime implementation or application
consumer changed.

No executor, tool implementation, memory namespace, provider, model, network,
process, filesystem, Tauri command, React state, capability, permission,
credential, dependency, manifest, lockfile, platform effect, Hermes path, or
specialist activation was added. Native remains sole/default. Every governance
execution disposition is `NotAttempted`.

## Verification results

- Passed: governance units 6/6; governance-audit units 14/14; orchestrator units
  8/8; public governance contract 10/10.
- Passed: unchanged orchestration 22/22, runtime 20/20, definition/registry 6/6,
  gateway 10/10, policy 2/2, approval 2/2, and legacy approval audit 1/1.
- Passed: all-target Rust — 241 passed, 0 failed, with one explicitly opt-in
  real-Hermes probe ignored as designed.
- Passed: formatting, all-target/all-feature check, strict Clippy, and complete
  `npm run verify`, including 124 frontend tests and Tauri no-bundle release
  build.
- Passed: documentation formatting/links, repository health, secret scan,
  whitespace/error diff check, and conflict-free session-end inventory.
- Passed as not required: no manual application check because the boundary is
  unwired and user-visible behavior is unchanged.

## Architecture findings

PASS. Trusted attribution construction requires an orchestrator-private proof
after exact live state validation. Component authority remains separated:
orchestration owns lifecycle, the shared policy engine classifies a distinct
sealed agent request, ApprovalManager owns exact one-time approval, and audit
records evidence only. Child pending approval is reconciled before direct or
root child-first cancellation. `AgentRuntime` and `NativeAgentRuntime` were not
broadened.

## Security findings

PASS WITH ADVISORIES. The profile matrix fails closed, agent requests cannot
enter legacy `PolicyInput`, approval origin is exact and redacted, subject
replay is consumed, audit reserves capacity before mutation, and exact matrix
outcome is distinct from control denial. No effect can execute. Direct tests do
not separately inject a forged agent source-origin mismatch or approval-manager
request-creation failure; sealed constructors and fail-closed code make these
coverage advisories rather than current authority gaps.

## Code-health findings

PASS WITH ADVISORIES. Types are closed and bounded, state transitions are
typed, Debug/errors are redacted, test-only fault and child-pending seams do not
exist in production, and complete compile/lint/tests pass. A compact exhaustive
delegation audit assertion table is a non-blocking coverage improvement; the
current contracts and unit tests cover representative matrix-allowed guard
failures, genuine route denial, and control success/failure.

## Technical debt

None introduced. The volatile 32-subject audit, synthetic proposal port, and
lack of execution are explicit boundary constraints rather than hidden partial
shipping behavior.

## Roadmap findings

D-084 Phase 4 is verified locally. Phase 5 memory and every later plan remain
Blocked. The exact next task is owner review and, only on explicit direction,
publication of this increment to a clean synchronized baseline. Later work
requires a separate plan, fresh review, and owner authorization.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked

Do not begin memory, knowledge/document processing, specialist workflows,
execution, provider, IPC, UI, or another roadmap phase automatically.

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
- `SECURITY_CHECKLIST.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/increments/agent-governance.md`
- `docs/plans/2026-08-11-agent-governance.md`
- `docs/reviews/2026-08-12-agent-governance-post-increment-review.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md`
- `docs/security/NATIVE_AGENT_GOVERNANCE_THREAT_MODEL.md`
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/governance.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/task.rs`
- `src-tauri/src/approvals/decision_source.rs`
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/src/approvals/types.rs`
- `src-tauri/src/audit/approval.rs`
- `src-tauri/src/audit/governance.rs`
- `src-tauri/src/audit/mod.rs`
- `src-tauri/src/policy/engine.rs`
- `src-tauri/tests/agent_governance_contract.rs`

## Exact commands executed

Every command in the machine manifest completed successfully. The real-Hermes
probe remained ignored by design and was not required or executed. No command
installed a dependency, launched Hermes, contacted a provider, mutated a remote
system, or created a Git commit.
