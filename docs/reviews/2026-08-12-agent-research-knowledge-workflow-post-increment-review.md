# Fixture-based Research and Knowledge workflow post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment agent-research-knowledge-workflow",
    "python3 .codex/hooks/post_increment_gate.py status",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::research_knowledge::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_research_knowledge_workflow_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
    "npx prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PRODUCT_REQUIREMENTS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md docs/PROJECT_DIRECTION.md docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md docs/increments/agent-research-knowledge-workflow.md docs/plans/2026-08-11-research-knowledge-workflow.md docs/plans/2026-08-12-agent-memory-approved-documents.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "npm run verify",
    "npx prettier --write docs/reviews/2026-08-12-agent-research-knowledge-workflow-post-increment-review.md",
    "npm run docs:check",
    "npx prettier --write docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md",
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
    "docs/increments/agent-research-knowledge-workflow.md",
    "docs/plans/2026-08-11-research-knowledge-workflow.md",
    "docs/plans/2026-08-12-agent-memory-approved-documents.md",
    "docs/reviews/2026-08-12-agent-research-knowledge-workflow-post-increment-review.md",
    "docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "src-tauri/src/agent/definition.rs",
    "src-tauri/src/agent/mod.rs",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/src/agent/research_knowledge.rs",
    "src-tauri/tests/agent_definition_registry_contract.rs",
    "src-tauri/tests/agent_memory_document_contract.rs",
    "src-tauri/tests/agent_research_knowledge_workflow_contract.rs",
    "src-tauri/tests/support/mock_agent_runtime.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Small",
      "milestone": "Future bounded orchestrator maintenance",
      "risk": "The private event-acceptance adapter distinguishes a continuation-start failure by comparing the closed continuation status before and after application; current preflight and mutation ordering are verified, but a typed internal outcome would make future invariant changes easier to review.",
      "severity": "Low",
      "summary": "Represent the private continuation application result directly rather than recognizing it through status change."
    }
  ],
  "increment_id": "agent-research-knowledge-workflow",
  "manual_verification": [
    {
      "check": "The complete diff adds no provider, live retrieval, network, filesystem discovery, persistence, dependency, manifest, lockfile, Tauri/React behavior, IPC, UI, runtime/native-runtime change, tool execution, permission, Hermes integration, general workflow engine, parallelism, or activation of another specialist.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Manual application check (not required because the new Rust boundary is unwired and no Tauri/React behavior changed)",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::research_knowledge::tests::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_research_knowledge_workflow_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked",
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
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
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
Increment: agent-research-knowledge-workflow
Branch: main

## Executive summary

Implemented D-086's exact deterministic fixture-only Personal Assistant ->
Research -> Knowledge & Document -> Personal synthesis workflow. Strict
structured Research, Knowledge, and final synthesis contracts preserve
application-issued fixture source references, truthful stage status, and
fixture disclosure. Sequential sibling tasks, partial failure, cancellation,
memory cleanup, bounded events, and redacted attribution remain application-
owned. All applicable source and complete repository checks pass. The result is
`PASS WITH ADVISORIES` with one low internal representation advisory.

## Scope and boundaries

The exact 27-path inventory matches the living ExecPlan. Production changes are
limited to the new `agent::research_knowledge` domain, Knowledge instruction V2,
module export, and the sealed orchestration integration. Test changes add the
D-086 contract, update exact Knowledge expectations, preserve the D-085 route
wording, and extend only the shared no-I/O mock's start-failure controls.

No provider, live research/retrieval, network, filesystem discovery,
persistence, dependency, manifest, lockfile, runtime/native-runtime change,
Tauri command, React consumer, IPC, UI, parallelism, workflow engine, tool
execution, permission, credential, Hermes integration, external effect, or
other specialist activation was added. Native remains sole/default.

## Verification results

- Passed: Research/Knowledge units 12/12; orchestrator units 10/10; public
  D-086 contract 18/18.
- Passed: memory/document 10/10; generic orchestration 22/22; governance 10/10;
  definition/registry 7/7; runtime 20/20; gateway 10/10.
- Passed: Rust formatting, all-target/all-feature check, and strict Clippy.
- Passed: all-target Rust — 299 passed, 0 failed, with one intentionally
  ignored opt-in real-Hermes probe.
- Passed: `npm run verify`, including repository health, lint, 28 hook tests,
  38 repository tests, 124 frontend tests, 178 library tests, every integration
  contract, TypeScript, Vite, and a Tauri no-bundle release build.
- Passed: final documentation formatting/link validation, repository health,
  security scan, diff hygiene, and conflict-free session-end inventory. The
  first post-report documentation check identified formatting only in the
  amended architecture assessment; the targeted Prettier write completed and
  the authoritative rerun passed.
- Not run and not required: manual application check, because the Rust boundary
  remains unwired and no Tauri/React behavior changed.

## Architecture findings

PASS WITH ADVISORIES. `AgentOrchestrator` remains the only task creator and
owns one fixed state machine rather than a general engine. Research and
Knowledge are sequential depth-one siblings; direct specialist delegation is
still denied. Runtime identity in workflow attribution is private and
non-authorizing. Terminal parse/preflight precedes runtime acceptance, and
continuation-start failure cannot reverse an accepted event or orphan work.
`AgentRuntime`/`NativeAgentRuntime` remain unchanged.

The one low advisory is internal: event acceptance recognizes a handled
continuation-start failure by comparing the closed status before/after applying
the prepared transition. Current preflight, mutation ordering, and combined-
failure tests make this safe; a typed private outcome would reduce future review
coupling.

## Security findings

PASS. Independent settled review found no completion-blocking security,
privacy, or authority defect. Specialist and final outputs are untrusted strict JSON. Source IDs must
come from the immutable application fixture catalog and Knowledge may use only
IDs validated from Research. Final synthesis additionally requires true fixture
disclosure and stage-consistent complete/partial status. Unknown fields,
reasoning, URLs, invented/duplicate/unknown references, false disclosure,
live-research claims, malformed values, and bound violations fail closed.

Journals and errors are bounded/redacted. Task memory is isolated and cleaned;
reusable Knowledge remains `PendingReview`. Cancellation is child-first and a
failed cancel preserves retryable state. No execution, device, provider,
network, filesystem, credential, persistence, IPC, or UI authority was added.

## Code-health findings

PASS WITH ADVISORIES. Closed domain types, exact scalar/byte/count limits,
strict serde envelopes, typed errors, prebuilt terminal transitions, and
deterministic contracts keep the implementation explicit. Review removed a
shadow legacy completion path and added final provenance plus lifecycle tests.
No production panic/unwrap/unsafe path or unrelated refactor was introduced.
The low private continuation-outcome representation advisory is recorded above.

## Technical debt

One Low code-health item: use a typed internal continuation application outcome
instead of recognizing it through a status comparison. Risk is limited to
future maintenance because current ordering and adversarial coverage are
closed. Estimated effort is Small in a future bounded orchestrator maintenance
increment. It blocks neither completion nor the next increment.

## Roadmap findings

D-086 Phase 5A is verified complete with advisories. Generic/direct Research-
to-Knowledge, live retrieval, durable memory/ARB-005, providers, every other
specialist workflow, parallelism, IPC, UI, and device effects remain Blocked.
No later plan is Ready; the roadmap order is unchanged.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked. The next task is separate owner review/publication direction for this
verified increment. Bounded parallelism and every later product plan require
their own accepted decision, Ready ExecPlan, review, and explicit owner
authorization.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/increments/agent-research-knowledge-workflow.md`
- `docs/plans/2026-08-11-research-knowledge-workflow.md`
- `docs/plans/2026-08-12-agent-memory-approved-documents.md`
- `docs/reviews/2026-08-12-agent-research-knowledge-workflow-post-increment-review.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/research_knowledge.rs`
- `src-tauri/tests/agent_definition_registry_contract.rs`
- `src-tauri/tests/agent_memory_document_contract.rs`
- `src-tauri/tests/agent_research_knowledge_workflow_contract.rs`
- `src-tauri/tests/support/mock_agent_runtime.rs`

## Exact commands executed

The machine manifest records each required command and its exact passed status.
It also records the targeted formatting commands used during closeout. The
first post-report documentation check stopped only on the named assessment
formatting warning; the final rerun passed after that file was formatted.
