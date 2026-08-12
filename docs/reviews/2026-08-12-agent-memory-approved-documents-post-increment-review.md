# Volatile agent memory and approved-document Knowledge boundary post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "python3 .codex/hooks/post_increment_gate.py begin --increment agent-memory-approved-documents",
    "python3 .codex/hooks/post_increment_gate.py status",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked memory::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked documents::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test storage_smoke --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test startup_storage_smoke --locked",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
    "npm run verify",
    "npx prettier --write ARCHITECTURE.md ROADMAP.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md docs/security/AGENT_MEMORY_DOCUMENT_PRIVACY.md",
    "npx prettier --write docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md docs/plans/2026-08-12-agent-memory-approved-documents.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md ROADMAP.md",
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
    "docs/increments/agent-memory-approved-documents.md",
    "docs/plans/2026-08-11-agent-memory.md",
    "docs/plans/2026-08-11-knowledge-document-boundaries.md",
    "docs/plans/2026-08-11-research-knowledge-workflow.md",
    "docs/plans/2026-08-12-agent-memory-approved-documents.md",
    "docs/reviews/2026-08-12-agent-memory-approved-documents-post-increment-review.md",
    "docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "docs/security/AGENT_MEMORY_DOCUMENT_PRIVACY.md",
    "src-tauri/src/agent/definition.rs",
    "src-tauri/src/agent/governance.rs",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/src/agent/task.rs",
    "src-tauri/src/documents.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/memory.rs",
    "src-tauri/tests/agent_definition_registry_contract.rs",
    "src-tauri/tests/agent_memory_document_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Future separately approved file-picker or platform-adapter increment",
      "risk": "A path can change between metadata validation and File::open; opened-handle and final-path identity checks make detected replacement fail closed, but pure standard-library opening cannot atomically no-follow every component.",
      "severity": "Advisory",
      "summary": "Pure-standard-library approved-document opening retains a narrow Unix TOCTOU residual."
    }
  ],
  "increment_id": "agent-memory-approved-documents",
  "manual_verification": [
    {
      "check": "The complete diff contains no dependency, manifest, lockfile, migration, SQLite product-data, Tauri/React behavior, provider, live model, executor, permission, credential, unrestricted filesystem tool, Hermes integration, Native runtime behavior change, external effect, or unrelated increment.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Manual application check (not required because the Rust boundary is unwired and no Tauri/React behavior changed)",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked memory::tests::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked documents::tests::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked",
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
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test storage_smoke --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test startup_storage_smoke --locked",
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
Increment: agent-memory-approved-documents
Branch: main

## Executive summary

Implemented D-085's bounded workflow-local volatile memory namespaces,
versioned application review, explicit context selection, selected UTF-8
document boundary, and separate direct Personal Assistant-to-Knowledge
mock-runtime route. Complete applicable validation and independent reviews
pass. The result is `PASS WITH ADVISORIES`; the only accepted advisory is the
narrow pure-standard-library Unix document-open TOCTOU residual.

## Scope and boundaries

The exact 31-path inventory matches the approved ExecPlan. Production changes
are limited to new memory/document modules and the necessary definition,
governance, task, orchestrator, and library wiring. The implementation remains
Rust-only, local, deterministic, unwired, and read-only at the document
boundary.

No dependency, manifest, lockfile, migration, SQLite product data, durable
memory, vector index, unrestricted file API, file picker, Tauri command, React
consumer, provider, live model, executor, permission, credential, process,
network, Hermes integration, external effect, or user-visible behavior was
added. `AgentRuntime` and `NativeAgentRuntime` are unchanged; Native remains
sole/default. Generic delegation remains Personal Assistant to Research and
the full Research-to-Knowledge workflow remains Blocked.

## Verification results

- Passed: memory units 6/6; approved-document units 9/9; public memory/document
  contract 10/10.
- Passed: definition/registry 7/7; governance 10/10; orchestration 22/22;
  runtime 20/20; gateway 10/10; storage 1/1; startup storage 1/1.
- Passed: Rust formatting, all-target/all-feature check, and strict Clippy.
- Passed: all-target Rust — 269 passed, 0 failed, with one intentionally
  ignored opt-in real-Hermes probe.
- The first `npm run verify` invocation exited 1 only at `prettier --check .`,
  which named four closeout documents; no later step ran. The exact targeted
  Prettier write completed, and the authoritative full `npm run verify` rerun
  passed repository health, lint, 28 hook tests, 38 repository tests, 124
  frontend tests, 166 library tests, every integration contract, TypeScript,
  Vite, and the Tauri no-bundle release build.
- Passed: final documentation formatting/link validation, repository health,
  security scan, diff hygiene, and conflict-free session-end inventory.
- The first post-closeout `npm run docs:check` identified formatting only in
  four newly synchronized documents. The exact targeted Prettier write
  completed before the authoritative final documentation check.
- Not run and not required: manual application check, because the Rust boundary
  is unwired and no Tauri/React behavior changed.

## Architecture findings

PASS WITH ADVISORIES. Memory and document authority remains application-owned
inside one bounded orchestrator workflow. Exact definition-derived profiles,
live grants, application-control proofs, opaque workflow-qualified IDs, linear
document tokens, and role-specific task creation keep authority out of model,
runtime, file, and WebView data. No new coupling to runtime, transport, storage,
provider, Tauri, or React exists. The pure-`std` open race is the only accepted
portability/security advisory.

## Security findings

PASS WITH ADVISORIES. Memory namespaces are isolated, proposal publication is
version-checked application review, and content never silently transfers across
agents. Paths remain private; traversal, symlink, hard-link, containment,
identity-change, replay, and revocation cases fail closed. Raw content and paths
stay out of Debug, errors, audit, logs, SQLite, frontend state, and provider
traffic. The opened-handle and final-path identity checks substantially narrow
but cannot atomically eliminate the standard-library document-open TOCTOU race.

## Code-health findings

PASS. The new domains use closed enums, bounded validated values, typed errors,
redacted Debug implementations, exact compare-and-swap versions, checked
capacity, linear reservation/commit, and exhaustive profile/route matching.
Focused adversarial contracts and all existing regressions pass under strict
Clippy. No unrelated refactor or dependency was introduced.

## Technical debt

One Advisory item: pure-standard-library approved-document opening retains a
narrow Unix TOCTOU residual between metadata validation and `File::open`.
Detected replacement still fails closed through opened-handle and final-path
identity comparison. Estimated effort is Medium in a future separately approved
file-picker/platform-adapter increment. It blocks neither completion nor the
next increment.

## Roadmap findings

D-085 Phase 5's selected volatile boundary is verified complete with
advisories. Durable memory remains blocked on ARB-005 and a new persistence
plan. Research-to-Knowledge, every other specialist workflow, parallelism, IPC,
UI, provider, and device effects remain separately Blocked. No later plan is
Ready and the roadmap order is unchanged.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked

The exact next task is project-owner review and, only on explicit direction,
publication of this complete increment. Do not begin another implementation
increment automatically.

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
- `docs/increments/agent-memory-approved-documents.md`
- `docs/plans/2026-08-11-agent-memory.md`
- `docs/plans/2026-08-11-knowledge-document-boundaries.md`
- `docs/plans/2026-08-11-research-knowledge-workflow.md`
- `docs/plans/2026-08-12-agent-memory-approved-documents.md`
- `docs/reviews/2026-08-12-agent-memory-approved-documents-post-increment-review.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `docs/security/AGENT_MEMORY_DOCUMENT_PRIVACY.md`
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/governance.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/task.rs`
- `src-tauri/src/documents.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/memory.rs`
- `src-tauri/tests/agent_definition_registry_contract.rs`
- `src-tauri/tests/agent_memory_document_contract.rs`

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py begin --increment agent-memory-approved-documents`
  established the active gate; `python3 .codex/hooks/post_increment_gate.py status`
  reported that exact increment active before closeout finalization.
- Every focused Cargo test command in the machine manifest passed with the
  counts recorded above. `cargo fmt`, all-target/all-feature `cargo check`, and
  strict `cargo clippy` passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked`
  passed 269 tests with one intentionally ignored opt-in Hermes probe.
- The first `npm run verify` exited 1 only on the four-file Prettier check.
  `npx prettier --write ARCHITECTURE.md ROADMAP.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md docs/security/AGENT_MEMORY_DOCUMENT_PRIVACY.md`
  formatted exactly those files. The final `npm run verify` passed completely.
- The first closeout-document `npm run docs:check` invocation named four
  formatting-only findings.
  `npx prettier --write docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md docs/plans/2026-08-12-agent-memory-approved-documents.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md ROADMAP.md`
  formatted exactly those files before the final passing documentation check.
- `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, and `python3 .codex/hooks/session_end_gate.py` all passed.
- `git status --short --branch` confirmed `main` at the synchronized `2687294`
  baseline with only the declared D-085 workspace. No command committed,
  pushed, installed a dependency, launched Hermes, contacted a provider, or
  mutated external state.
