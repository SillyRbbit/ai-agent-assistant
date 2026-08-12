# Per-agent governance foundation

Status: Verified complete with advisories
Date: 2026-08-12
Gate ID: `agent-governance`
Plan: `docs/plans/2026-08-11-agent-governance.md`
Decision: D-084, preserving D-083, D-082, and D-079
Baseline: clean synchronized `main` at `1d1d9d6`

## Goal

Bind exact application-derived agent/task/runtime/profile attribution through a
closed non-executing policy, approval, delegation, and volatile audit boundary
without enabling a tool, provider, memory store, IPC surface, or device effect.

## Implemented boundary

- Nine exact versioned policy profiles are captured from sealed built-in
  definitions into tasks and live execution contexts. There is no fallback or
  independent caller-supplied profile mapping.
- Only `AgentOrchestrator` may derive `AgentAttribution`, after exact task,
  lineage, runtime, run, request, status, and profile validation.
- `AgentGovernanceService` composes the existing fixed `ToolRegistry`,
  `DeterministicPolicyEngine`, and `ApprovalManager` with a new bounded volatile
  audit. It does not own execution.
- Personal Assistant may govern the two registered schemas. Date/time is
  deterministic `Allow`; local-task creation is `RequireApproval`. Every other
  profile deterministically denies current tools.
- Agent approval has a closed origin and retains exact attribution through
  pending view, presentation, trusted source result, cancellation, expiry, and
  resolution. Debug surfaces redact origin identity and content.
- Delegation remains an explicit orchestrator operation outside `ToolRegistry`.
  The audit records the exact Personal-to-Research matrix result separately
  from later guard denial or child-creation outcome.
- The governance audit is typed, redacted, volatile, replay-safe, and capped at
  32 subjects. One slot is reserved before downstream mutation and updated
  through terminal disposition without additional capacity.
- Pending approval blocks runtime events and delegation. Cancellation resolves
  and audits approval before task/run mutation, with child-first ordering for a
  root waiting on a child.

## Preserved boundaries

Every execution disposition is `NotAttempted`. Runtime tool proposals remain
rejected. `AgentRuntime` and `NativeAgentRuntime` are unchanged; Native remains
sole/default. The implementation adds no executor, tool implementation, memory
namespace, persistence, provider, model, process, network, filesystem, Tauri
IPC, React state, capability, permission, dependency, credential, Hermes path,
specialist activation, or visible behavior.

## Exact implementation and test files

- `src-tauri/src/agent/governance.rs`
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/task.rs`
- `src-tauri/src/policy/engine.rs`
- `src-tauri/src/approvals/types.rs`
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/src/approvals/decision_source.rs`
- `src-tauri/src/audit/governance.rs`
- `src-tauri/src/audit/approval.rs`
- `src-tauri/src/audit/mod.rs`
- `src-tauri/tests/agent_governance_contract.rs`

## Evidence

- Agent-governance units: 6 passed.
- Governance-audit units: 14 passed.
- Orchestrator units: 8 passed.
- Public governance contract: 10 passed.
- Existing orchestration 22/22, runtime 20/20, definition/registry 6/6,
  gateway 10/10, policy 2/2, approval 2/2, and legacy approval audit 1/1 pass.
- All-target Rust: 241 passed, 0 failed, with one explicitly opt-in real-Hermes
  probe ignored as designed.
- `npm run verify`, including 124 frontend tests, complete Rust tests,
  TypeScript, frontend production build, and Tauri no-bundle release build,
  passed.
- Formatting, all-target/all-feature Cargo check, strict Clippy,
  docs/repository/security checks, diff hygiene, session-end inventory, and
  independent architecture/security/code review passed.

Quality result: `PASS WITH ADVISORIES`. No implementation blocker or new
technical debt remains. Coverage advisories are limited to agent-specific
forged approval-source origin, approval-manager request-creation failure, and
exhaustive delegation error-record assertions. Publication and supported
cross-target CI remain later evidence.

## Rollback

Remove the two new governance modules and governance contract; restore only the
declared agent, policy, approval, audit, decision, architecture, security,
roadmap, plan, and current-memory edits. There is no data, migration,
dependency, IPC, permission, credential, process, network, provider, or
external-state rollback.
