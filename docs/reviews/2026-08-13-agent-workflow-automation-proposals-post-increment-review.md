# Typed Workflow Automation proposals post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment agent-workflow-automation-proposals",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked workflow_automation",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_workflow_automation_contract --locked",
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
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "docs/PROJECT_DIRECTION.md",
    "docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md",
    "docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md",
    "docs/increments/agent-workflow-automation-proposals.md",
    "docs/plans/2026-08-11-workflow-automation.md",
    "docs/reviews/2026-08-13-agent-workflow-automation-proposals-post-increment-review.md",
    "docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md",
    "src-tauri/src/agent/definition.rs",
    "src-tauri/src/agent/governance.rs",
    "src-tauri/src/agent/mod.rs",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/src/agent/orchestrator/workflow_automation_dispatch.rs",
    "src-tauri/src/agent/orchestrator/workflow_automation_proposal.rs",
    "src-tauri/src/agent/workflow_automation.rs",
    "src-tauri/src/agent/workflow_automation/catalog.rs",
    "src-tauri/src/agent/workflow_automation/validation.rs",
    "src-tauri/src/tools/registry.rs",
    "src-tauri/tests/agent_definition_registry_contract.rs",
    "src-tauri/tests/agent_governance_contract.rs",
    "src-tauri/tests/agent_orchestration_contract.rs",
    "src-tauri/tests/agent_workflow_automation_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Architecture",
      "effort": "High",
      "milestone": "Before any hard-duration or external-runtime claim",
      "risk": "The cooperative monotonic lease cannot interrupt a synchronous runtime.start already in flight.",
      "severity": "Advisory",
      "summary": "Do not represent the D-090 lease as hard preemption."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low",
      "milestone": "Future focused Workflow Automation hardening",
      "risk": "The event-cap contract is covered indirectly but lacks direct 15/16/17 boundary cases.",
      "severity": "Advisory",
      "summary": "Add direct event-cap boundary tests when this contract is next changed."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Medium",
      "milestone": "Future focused Workflow Automation hardening",
      "risk": "Second lease-check coverage is not table-driven across every A-D selector and terminal variant.",
      "severity": "Advisory",
      "summary": "Table-drive destination lease checks before broadening manual dispatch."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low",
      "milestone": "Future focused parser hardening",
      "risk": "Nested/synthesis duplicate-key and exact multibyte/escaping boundary cases can be more explicit.",
      "severity": "Advisory",
      "summary": "Add the remaining exact parser boundary fixtures when the schema changes."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Before another orchestrated workflow family",
      "risk": "The remaining orchestrator and private lifecycle modules are large; adding a generic engine would increase authority and review coupling.",
      "severity": "Advisory",
      "summary": "Preserve private workflow ownership and do not grow D-090 into a general engine."
    }
  ],
  "increment_id": "agent-workflow-automation-proposals",
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
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked workflow_automation",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_workflow_automation_contract --locked",
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

Date: 2026-08-13
Increment: agent-workflow-automation-proposals
Branch: main

## Executive summary

D-090's narrowed Workflow Automation implementation is source-complete and its
automated implementation verification passes. It adds five immutable typed
templates, strict proposal and synthesis validation, one sealed Personal ->
Workflow Automation -> Personal lifecycle, and an explicit expiring take-once
manual bridge from complete A-D proposals to the already verified no-I/O
selectors in a fresh orchestrator. Template E and every tool/approval step
remain non-executable.

Independent architecture, security, and code review is `PASS WITH ADVISORIES`
and found no blocker. The completion result is `PASS WITH ADVISORIES`; all
required post-documentation checks pass and this report is the deterministic
completion evidence.

## Scope and boundaries

Production changes are confined to a framework-neutral Workflow Automation
domain with private catalog/validation children, two private orchestrator
children for proposal and manual dispatch, exact V2 activation metadata, a
shared read-only built-in tool registry used for validation, and thin
orchestrator/module integration. Tests add one public D-090 contract and update
only catalog, governance, and generic-route expectations.

No tool implementation/execution, policy eligibility, approval request or
dispatch, general/durable audit logger, arbitrary code/shell/script, generic
workflow engine, scheduling, recurring/background work, persistence,
parallelism, provider, external runtime, filesystem/network/credential access,
dependency, manifest/lockfile, Tauri/React IPC, UI, permission, or effect was
added. Native remains sole/default and existing A-D selectors retain their
original semantics.

## Verification results

- Passed: focused Workflow Automation tests, 12/12 (seven domain/catalog/
  validation and five lifecycle/deadline).
- Passed: public Workflow Automation contract, 18/18.
- Passed: strict all-target/all-feature Clippy.
- Passed: all-target Rust, 393 passed with one intentionally ignored opt-in
  real-Hermes probe.
- Passed: `npm run verify`, including 124 frontend tests, 208 Rust library
  tests, all integration contracts, TypeScript/Vite, and the Tauri no-bundle
  release build.
- Passed after documentation closeout: documentation formatting/links,
  repository health, security scan, diff hygiene, and session-end inventory.
- Not required: manual application check because the Rust core remains unwired
  and no UI, IPC, provider, I/O, or device behavior changed.

## Architecture findings

Final independent architecture review is `PASS WITH ADVISORIES` with no
blocker. Application-owned identity, catalog, template, validation, deadline,
token, selection, cancellation, and attribution remain separated from
untrusted runtime output. The cooperative lease is intentionally not hard
preemption, and private workflow ownership remains the required boundary.

## Security findings

Final independent security review is `PASS WITH ADVISORIES` with no blocker.
Tool lookup is read-only; known tools remain non-executable and Workflow
Automation's profile permits neither registered tool. No policy permission,
approval or execution subject, credential source, I/O path, or external effect
exists. Runtime output cannot construct a trusted token or identity.

## Code-health findings

Final code review: `PASS WITH ADVISORIES`, no blocker. Its five advisories are
recorded in the manifest. The focused and public contracts cover the accepted
behavior; direct 15/16/17 event-cap cases and additional exact parser boundary
fixtures remain optional future hardening.

## Technical debt

The remaining private orchestrator and lifecycle modules are large. This is a
non-blocking advisory for D-090, but another workflow family must not extend
them into a general engine without its own bounded decomposition decision.

## Roadmap findings

No next increment is Ready. Tool execution, approval dispatch, template E
execution, scheduling, persistence, UI/IPC, providers, and effects remain
separate blocked roadmap decisions.

## Completion decision

`PASS WITH ADVISORIES` is the final completion result. Every required
verification and manual review records `Passed`; the non-required application
check remains intentionally not run because this boundary is unwired.

## Next-increment readiness

Blocked. No next owner-selected Ready plan exists. Tool execution, approval
dispatch, document-to-action execution, scheduling, recurring/background
execution, persistence, UI/IPC, providers, external runtimes, and every effect
remain deferred.

## Exact files changed

The machine manifest above lists the exact complete 33-path Git change set:
19 documentation paths, 10 production Rust paths, and 4 Rust contract-test
paths. No manifest, lockfile, dependency, Tauri capability, frontend, runtime,
provider, persistence, or external-integration path changed.

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py begin --increment agent-workflow-automation-proposals`
- `cargo test --manifest-path src-tauri/Cargo.toml --lib --locked workflow_automation`
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_workflow_automation_contract --locked`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings`
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked`
- `npm run verify`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 .codex/hooks/session_end_gate.py`
