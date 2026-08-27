# Runtime-start containment F-01/F-02 post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/session_end_gate.py",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --test agent_research_knowledge_workflow_contract --test agent_engineering_quality_workflow_contract --test agent_infrastructure_operations_workflow_contract --test agent_workflow_automation_contract --test agent_bounded_parallelism_contract",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/runtime-start-containment-f01-f02.md",
    "docs/plans/2026-08-26-runtime-start-containment-f01-f02.md",
    "docs/reviews/2026-08-26-runtime-start-containment-f01-f02-post-increment-review.md",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/src/agent/orchestrator/bounded_parallel_workflow.rs",
    "src-tauri/tests/agent_engineering_quality_workflow_contract.rs",
    "src-tauri/tests/agent_infrastructure_operations_workflow_contract.rs",
    "src-tauri/tests/agent_orchestration_contract.rs",
    "src-tauri/tests/agent_research_knowledge_workflow_contract.rs",
    "src-tauri/tests/agent_workflow_automation_contract.rs",
    "src-tauri/tests/support/mock_agent_runtime.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "before any external runtime or provider integration",
      "risk": "A future external session cannot treat process-local orchestrator quarantine as application-global or durable cleanup ownership.",
      "severity": "Advisory",
      "summary": "Rejected-run quarantine is volatile and owned only for the lifetime of the unwired orchestrator; a future external runtime still requires separately approved application lifecycle ownership."
    }
  ],
  "increment_id": "runtime-start-containment-f01-f02",
  "manual_verification": [
    {
      "check": "Rendered UI, IPC, or device-effect validation",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --test agent_research_knowledge_workflow_contract --test agent_engineering_quality_workflow_contract --test agent_infrastructure_operations_workflow_contract --test agent_workflow_automation_contract --test agent_bounded_parallelism_contract",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings",
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
    }
  ]
}
-->

Date: 2026-08-26
Increment: runtime-start-containment-f01-f02
Branch: `main`

## Executive summary

F-01/F-02 runtime-start containment is complete. Every existing runtime start
now validates exact application-owned returned identity, rejects duplicate live
identity, retains rejected nonterminal runs, and blocks new or fallback starts
until explicit cleanup succeeds. Acceptance criteria are met. The result is
`PASS WITH ADVISORIES` because the retained owner is intentionally volatile and
no separate F-07 plan is owner-approved or Ready.

## Scope and boundaries

The complete 20-file inventory stays inside the approved Rust containment,
adversarial-test, evidence, and project-memory scope. No runtime trait, Tauri
command/event, WebView path, CSP, capability, provider, model, dependency,
network, credential, tool, approval, persistence, filesystem, platform, or
device-effect surface changed.

## Verification results

- Focused orchestration/workflow contracts: `Passed`, 153/153.
- Strict focused Clippy: `Passed` with warnings denied.
- All-target Rust: `Passed`, 490 executed tests passed and one explicitly
  opt-in Hermes version probe was ignored.
- Complete `npm run verify`: `Passed`, including 211/211 frontend tests and the
  target-Mac release no-bundle build.
- Documentation, repository policy, security scan, session-end inventory, and
  diff checks: `Passed`.
- Rendered UI, IPC, and device-effect checks: `Not run`; not required because
  those boundaries did not change.

## Architecture findings

`PASS`. The change removes the weaker legacy start path and reuses the existing
application-owned D-091 containment abstraction across every sealed workflow.
Module ownership, runtime traits, deterministic workflow projections,
portability, dependencies, and no-I/O boundaries remain unchanged.

## Security findings

`PASS`. The request is the sole trusted identity source. Foreign, stale, and
duplicate returned identities never become active contexts. Nonterminal
rejections are cancelled or retained; failed or contradictory cleanup remains
closed and blocks all starts. Errors and mock lifecycle evidence remain typed
and content-redacted. No authorization, execution, secret, log, storage,
network, filesystem, IPC, permission, or credential surface was added.

## Code-health findings

`PASS`. One shared helper replaces duplicated weaker/stronger paths. New
contracts cover root, child, synthesis, Research, Engineering, Cloud, Systems,
Workflow Automation, one-shot and permanent cancellation failures,
contradictory nonterminal disposition, fallback blocking, and cleanup retry.
Existing bounded-parallel and complete repository tests remain green.

## Technical debt

- Category: Runtime lifecycle. Severity: `Advisory`. The quarantine is volatile
  and lasts only for the lifetime of the unwired orchestrator. Risk: a future
  external session cannot rely on this as application-global durable cleanup.
  Effort: Medium. Milestone: before any external runtime/provider. It does not
  block completion or F-07; it blocks live runtime/provider integration.

No debt was introduced inside the approved current native-only boundary.

## Roadmap findings

`Blocked`. F-15, F-12, and F-01/F-02 prerequisites are complete, but the next
dependency-ordered F-07 production/development CSP increment has no exact
owner-approved ExecPlan. The smallest next action is a separate F-07
documentation/readiness run.

## Completion decision

`PASS WITH ADVISORIES`.

## Next-increment readiness

`Blocked`. Prepare and approve a bounded F-07 production/development CSP plan;
do not begin F-07, F-08, agent IPC, UI integration, or provider work from this
increment.

## Exact files changed

The machine manifest lists the complete 20-file current inventory.

## Exact commands executed

The machine manifest records every required verification command and its actual
result.
