# Fixture-only engineering quality workflow

Status: Verified complete with advisories
Date: 2026-08-12
Gate ID: `agent-engineering-quality-workflow`
Plan: `docs/plans/2026-08-11-engineering-quality-workflow.md`
Decision: D-087, preserving D-079 and D-082 through D-086
Baseline: clean synchronized `main` at `3efd2c1`

## Goal

Implement exactly one deterministic, Rust-only Personal Assistant -> Coding
Agent -> QA & Validation Agent -> Security & Risk Agent -> Personal Assistant
synthesis workflow above the unchanged `AgentRuntime`, using only immutable
application-supplied repository fixtures and proposal-only results. The
workflow introduces no live repository, filesystem, process, Git, package,
network, provider, or device access and creates no approval request.

## Implemented boundary

- `EngineeringQualityWorkflowRequest` owns a bounded objective, one to eight
  synthetic fixture files, one to eight application-issued acceptance criteria,
  and up to eight application validation-evidence records.
- Evidence is closed to `ObservedFixture` facts bound to fixture IDs or
  `NotRun` proposed checks. It cannot represent a live test pass, repository
  observation, or external result.
- `ChangeProposal` is a strict bounded V1 proposal. It contains fixture-bound
  findings, affected file IDs, inert patch descriptions, risks, validation
  steps, rollback, and closed capability requests. Analysis, architecture
  explanation, diff review, implementation planning, patch proposal, test
  planning, formatting planning, and security review are proposal-only. File
  mutation/deletion, path escape, dependency or package-manager execution,
  test/formatter execution, Git commit/push/branch deletion, destructive shell,
  credential access, and network access are denied.
- `ValidationReport` is bound to the exact proposal and reconciles every
  application-issued criterion exactly once. Demonstrated criteria require
  exact observed-fixture evidence; any not-demonstrated criterion forces an
  incomplete or blocked report. Proposed checks remain `NotRun`. QA cannot
  approve, modify source, suppress tests, or fabricate execution evidence.
- `RiskAssessment` is bound to the exact proposal and either the exact QA
  report or a closed QA-unavailable status. Findings carry evidence references
  or are explicitly hypotheses. Dependency evidence is available only when the
  application catalog contains it. Security remains advisory and cannot
  authorize, remediate, become policy, or expose secret values.
- Final Personal synthesis preserves the validated Coding, QA, and Security
  outcomes and derives `NotApplicable` for analysis-only or
  `RequiredBeforeMutation` for a patch proposal. It explicitly states that the
  input was fixture-based, the result is proposal-only, and no changes or tests
  were executed. The workflow creates no approval request because it has no
  actionable execution subject.

## Sequence and lifecycle

`AgentOrchestrator` alone selects `engineering-quality-v1`, terminally cancels
the initial Personal run, and creates Coding, QA, and Security as three
sequential depth-one siblings beneath the same Personal root. Specialists never
spawn or invoke one another. A fresh Personal run performs final synthesis.

The exact limits are four tasks, three non-replenishing child attempts, one
active child, five sequential runtime-run attempts, 32 runtime events, 32
generic orchestration events, 16 engineering workflow events, 16 matching
descriptive audit records, and zero automatic retries. Each accepted stage
input is capped at 24,576 bytes and is tested through the unchanged Native
65,536-byte encoded request boundary. Terminal parsing, capacity, task output,
successor input, and successor state are prepared before accepting the terminal
event. An accepted terminal event is never reversed or retried when a later
runtime start fails.

The sealed selector is mutually exclusive in both directions with generic
delegation, D-085 document work, and D-086. Generic routes remain unchanged.
Coding, QA, and Security are `Initial` only for this exact unwired workflow;
their policy profiles remain tool-ineligible and their memory profile remains
`MemoryDisabledV1`.

## Partial failure and cancellation

Coding failure skips QA and Security and permits only truthful Personal
fallback synthesis. QA failure preserves the proposal and permits bounded
Security review with QA explicitly unavailable. QA incomplete or blocked
forces partial synthesis. Security failure preserves validated Coding and QA
results for partial synthesis. Final synthesis failure fails the root without
fabricating a result. Raw invalid output never reaches a successor.

Independent child cancellation records the exact stage and follows only its
allowed successor. Root cancellation resolves pending governance, cancels the
active child before the root, starts no later stage, and rejects late events.
Runtime-reported cancellation is mapped to the same typed terminal outcome.
Cancellation failure preserves live retryable state; no task, run, event, or
retry budget is replenished.

## Preserved authority boundaries

There is no live repository inspection, filesystem operation, code-search
process, test or formatter run, shell or package-manager command, dependency
installation, Git operation, credential access, unrestricted network access,
tool registration, executor, approval request, Tauri command, React consumer,
IPC, persistence, provider, external runtime, or device effect. Existing tool
schemas, policy profiles, approval behavior, memory, documents,
`AgentRuntime`, and `NativeAgentRuntime` are unchanged. Every engineering
governance execution disposition remains `NotAttempted`. Native remains
sole/default; Codex, Hermes, and OpenClaw are not integrated.

## Evidence

- Engineering contract/parser units: 8 passed.
- Orchestrator units: 10 passed.
- Public D-087 workflow contract: 20 passed.
- Definition registry 7, governance 10, generic orchestration 22,
  memory/document 10, D-086 18, runtime 20, and gateway 10 contracts passed.
- Rust formatting, all-target/all-feature check, and strict Clippy passed.
- All-target Rust passed 326 tests with one intentionally ignored opt-in
  real-Hermes probe.
- `npm run verify` passed, including 124 frontend tests, 186 library tests, all
  integration contracts, TypeScript, Vite, and Tauri no-bundle build.
- Final documentation, repository, security, diff, and session-end checks
  passed. The deterministic completion marker validates this recorded evidence.
- No manual application check is required because the Rust boundary remains
  unwired and no Tauri/React behavior changed.

Quality result: `PASS WITH ADVISORIES`. No completion-blocking correctness,
security, privacy, or authority finding remains. Before another
multi-specialist workflow, consider decomposing private orchestrator internals
into smaller typed helpers or state components without creating a general
workflow engine. No later owner-approved plan is Ready; next-increment
readiness is `Blocked`.

## Rollback

Before publication, remove the engineering-quality module and public contract,
restore the three V1 deferred definitions, remove only the sealed engineering
selector/state and related fixture support, and revert D-087 current-state
documentation. Preserve D-079 and D-082 through D-086 and all historical
increment/review evidence. There is no data, migration, dependency, credential,
provider, process, network, IPC, approval, execution, or external state to
reverse. After publication, use one bounded revert plus an additive superseding
decision rather than rewriting D-087 history.
