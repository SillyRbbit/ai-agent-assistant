# Native nine-agent architecture review post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment native-nine-agent-architecture-review",
    "npm run test:agent-acceptance",
    "npm run verify",
    "cargo test --manifest-path src-tauri/Cargo.toml --locked --test agent_orchestration_contract --test agent_bounded_parallelism_contract",
    "cargo test --manifest-path src-tauri/Cargo.toml --locked --test agent_definition_registry_contract --test agent_governance_contract --test agent_runtime_contract",
    "cargo test --manifest-path src-tauri/Cargo.toml --locked --lib agent::orchestrator::bounded_parallel_workflow::tests",
    "npm run test:frontend",
    "npm run typecheck",
    "npm ls --omit=dev --all --json",
    "npm ls --all --json",
    "cargo tree --manifest-path src-tauri/Cargo.toml --locked -e normal",
    "npm audit --audit-level=low",
    "npx prettier --write docs/reviews/2026-08-26-native-nine-agent-architecture-review-post-increment-review.md",
    "npx prettier --check docs/reviews/NATIVE_NINE_AGENT_ARCHITECTURE_REVIEW.md docs/reviews/2026-08-26-native-nine-agent-architecture-review-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex .agents scripts",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment native-nine-agent-architecture-review --report docs/reviews/2026-08-26-native-nine-agent-architecture-review-post-increment-review.md",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "docs/reviews/2026-08-26-native-nine-agent-architecture-review-post-increment-review.md",
    "docs/reviews/NATIVE_NINE_AGENT_ARCHITECTURE_REVIEW.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "Medium",
      "milestone": "Before any live provider, external runtime, or agent IPC",
      "risk": "A faulty future adapter can return a stale, foreign, or reused run identity that the legacy runtime-start path accepts as trusted context.",
      "severity": "Medium",
      "summary": "F-01: legacy runtime starts trust adapter-returned identity."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "Medium",
      "milestone": "Before any external runtime or provider work",
      "risk": "A rejected nonterminal external run can lose its cleanup owner when cancellation fails or returns a contradictory outcome.",
      "severity": "Medium",
      "summary": "F-02: failed legacy rejection cancellation can drop the live-run handle."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "High",
      "milestone": "Before a configured live model or provider",
      "risk": "Provider invocations do not carry an application-owned role and instruction-version profile separated from untrusted objectives and evidence.",
      "severity": "Medium",
      "summary": "F-03: versioned agent instructions are not bound to runtime invocations."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "High",
      "milestone": "Before live or provider wiring",
      "risk": "Earlier task and workflow families can retain a silent run indefinitely because they have no application-owned liveness deadline.",
      "severity": "Medium",
      "summary": "F-04: most task and workflow families have no liveness deadline."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Medium",
      "milestone": "Before any real action or connected approval experience",
      "risk": "Cancellation terminalizes approval state but cannot dismiss the synchronous native prompt, leaving misleading stale security UI visible.",
      "severity": "Medium",
      "summary": "F-05: native approval prompts cannot be dismissed when the task is cancelled."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "Medium",
      "milestone": "Before backend-connected approval UI or IPC",
      "risk": "An approval rejection is safe and audited but does not deterministically settle or present controlled denial for the owning task.",
      "severity": "Low",
      "summary": "F-06: approval rejection is audited but not task-terminal feedback."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Medium",
      "milestone": "Before live data or connected agent UI",
      "risk": "The production WebView CSP retains a development localhost WebSocket source and inline-script allowance, weakening defense in depth.",
      "severity": "Medium",
      "summary": "F-07: production CSP retains development network and inline-script allowances."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Low",
      "milestone": "Before relying on app-info as a connected UI trust signal",
      "risk": "Malformed or insecure version-skewed IPC data can be presented as a ready local core because the response is not runtime narrowed.",
      "severity": "Low",
      "summary": "F-08: app-info IPC is compile-time typed but not runtime narrowed."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "High",
      "milestone": "Before real actions, retained provider data, or durable user-facing memory and documents",
      "risk": "Separate volatile audit families do not provide durable, unified correlation of agent governance, exact approval interaction, memory, documents, and workflows.",
      "severity": "Low",
      "summary": "F-09: consequential audit is fragmented and drops exact agent-approval correlation."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Medium",
      "milestone": "Before live document exposure or broader document formats",
      "risk": "Unix document validation is strong but does not atomically prevent following a replaced path component during open.",
      "severity": "Low",
      "summary": "F-10: Unix approved-document open is not atomically no-follow."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "High",
      "milestone": "Before claiming provider-backed or app-global parallel execution",
      "risk": "Per-orchestrator cooperative multiplexing does not provide application-global capacity, fairness, session isolation, or hard preemption.",
      "severity": "Low",
      "summary": "F-11: bounded parallelism is not operational concurrency control."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Low",
      "milestone": "Before agent UI or IPC work",
      "risk": "No automated repository rule prevents later Command Center Tauri, network, storage, command, or capability boundary expansion.",
      "severity": "Low",
      "summary": "F-12: UI/native isolation invariants are review-only."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Code health",
      "effort": "Medium-High",
      "milestone": "Before another workflow, concurrency, or Command Center functional expansion",
      "risk": "Large stateful backend and frontend modules make paired lifecycle, audit, cleanup, fixture, and presentation changes difficult to review safely.",
      "severity": "Low",
      "summary": "F-13: backend and frontend hotspots create excessive review coupling."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Architecture",
      "effort": "High",
      "milestone": "Only if the owner separately requires a combined workflow approval bridge",
      "risk": "Joining the accepted checkpoint-denial and manual-dispatch branches without a new exact-binding design could create unintended approval-to-control authority.",
      "severity": "Low",
      "summary": "F-14: Workflow Automation has no combined approval-to-manual-dispatch chain."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Technical debt",
      "effort": "Low-Medium",
      "milestone": "Next documentation-only reconciliation after owner selection",
      "risk": "Duplicate requirement identifiers and stale UI and boundary claims can mislead traceability and readiness decisions.",
      "severity": "Low",
      "summary": "F-15: current documentation has identifiable semantic drift."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Low technical effort plus owner or legal judgment",
      "milestone": "Before public source or binary release",
      "risk": "Publication without a selected project license and documented third-party obligation review would leave reuse rights and notices ambiguous.",
      "severity": "Low",
      "summary": "F-16: public licensing and third-party obligation review are not prepared."
    }
  ],
  "increment_id": "native-nine-agent-architecture-review",
  "manual_verification": [
    {
      "check": "Independent core-lifecycle, governance-boundary, and UI-quality reviews reconciled against direct source and test evidence",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Git scope inspection confirms that only the requested comprehensive review and this closeout report changed",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Fresh rendered Browser Control and native Computer Use matrix (not required because this increment reviewed, but did not change, the previously completed rendered evidence)",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm run test:agent-acceptance",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --locked --test agent_orchestration_contract --test agent_bounded_parallelism_contract",
      "required": false,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --locked --test agent_definition_registry_contract --test agent_governance_contract --test agent_runtime_contract",
      "required": false,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --locked --lib agent::orchestrator::bounded_parallel_workflow::tests",
      "required": false,
      "status": "Passed"
    },
    {
      "command": "npm run test:frontend",
      "required": false,
      "status": "Passed"
    },
    {
      "command": "npm run typecheck",
      "required": false,
      "status": "Passed"
    },
    {
      "command": "npx prettier --check docs/reviews/NATIVE_NINE_AGENT_ARCHITECTURE_REVIEW.md docs/reviews/2026-08-26-native-nine-agent-architecture-review-post-increment-review.md",
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
      "command": "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex .agents scripts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/post_increment_gate.py finalize --increment native-nine-agent-architecture-review --report docs/reviews/2026-08-26-native-nine-agent-architecture-review-post-increment-review.md",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-26

Increment: Native nine-agent architecture comprehensive review

Mode: Review and closeout only

## Executive summary

The requested comprehensive review is complete. The primary artifact records
six P2 Medium and ten P3 Low findings, with no P0 Critical or P1 High finding.
The current deterministic, fixture-only, unwired architecture passes review
with advisories; it is not approved for a live provider, connected agent IPC,
real governed execution, production approval UX, or public release.

No remediation was implemented. No production source, test, fixture,
dependency, lockfile, workflow, capability, or configuration file changed.

## Verification results

- `npm run test:agent-acceptance`: passed 447 tests (249 Rust library and 198
  public agent contract tests).
- `npm run verify`: passed, including 211 frontend tests and 481 Rust tests;
  one intentional opt-in Hermes probe was ignored (482 Rust tests discovered).
- Focused orchestration and bounded-parallel contracts: passed 63 tests.
- Focused registry, governance, and runtime contracts: passed 44 tests.
- Focused bounded-parallel library tests: passed 25 tests.
- `npm audit --audit-level=low`: passed with 0 vulnerabilities.
- Fresh documentation, repository, security, formatting, scope, session-end,
  finalization, and status checks are recorded in the machine manifest.
- `cargo-audit` was not run locally because the binary is absent; the pinned CI
  accepted-baseline gate was inspected and the omission is disclosed in the
  comprehensive review.

## Architecture findings

F-01 through F-04 identify future runtime identity, rejected-run ownership,
instruction binding, and liveness prerequisites. F-06, F-11, and F-14 identify
controlled-denial, operational-concurrency, and intentionally absent workflow
bridge boundaries. None creates a current external effect because the agent
core has no provider, agent IPC, executor, or production platform adapter.

## Security findings

F-05 and F-07 through F-10 identify approval-prompt lifecycle, production CSP,
app-info validation, unified audit, and Unix document-open hardening work. F-12
records that the currently correct UI/native isolation is not enforced by an
automated repository boundary check. Current policy, approval, validation, and
execution authority remain application-owned and fail closed.

## Code-health findings

F-13 records the review coupling created by large stateful backend workflow
modules and frontend Command Center projection and presentation hotspots. A
behavior-preserving decomposition is advisory before another corresponding
functional expansion; a generic workflow engine or event bus is not advised.

## Technical debt

All sixteen findings include severity, evidence, affected symbols, scenario,
impact, remediation, regression expectations, timing, and remediation risk in
the comprehensive review. F-15 separately records concrete documentation drift.
The backlog is advisory and grants no implementation authority.

## Roadmap findings

NativeAgentRuntime remains the sole/default runtime and remains unwired to a
provider or Tauri. Hermes remains Deferred/Blocked. F-16 requires owner license
selection and third-party obligation review only if public distribution is
selected; it does not add SaaS, multi-tenancy, billing, marketplace, or
enterprise-IAM scope.

## Completion decision

`PASS WITH ADVISORIES` for this review-only increment. The review objective and
required artifact are complete, every required closeout check passed, and no
Critical or High finding blocks completion. The unresolved findings remain
explicit blockers at their stated activation or publication milestones.

## Next-increment readiness

`Blocked`. The review does not authorize remediation, and the owner has not
selected and approved a new bounded ExecPlan. The first recommended candidate
is a separate runtime-start containment hardening review/plan for F-01 and F-02;
live-provider, connected-action, and public-release work remain blocked.

## Exact files changed

- `docs/reviews/NATIVE_NINE_AGENT_ARCHITECTURE_REVIEW.md`
- `docs/reviews/2026-08-26-native-nine-agent-architecture-review-post-increment-review.md`

## Exact commands executed

The machine manifest above is the authoritative exact command inventory. It
includes the full and focused test suites, dependency and audit inspection,
fresh documentation-only completion checks, the session-end gate, and the
post-increment finalization and status checks. No commit, push, merge, release,
publication, deployment, or external write command was executed.
