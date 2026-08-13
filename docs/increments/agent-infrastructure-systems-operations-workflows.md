# Fixture-only infrastructure and systems operations workflows

Status: Verified complete with advisories
Date: 2026-08-12
Gate ID: `agent-infrastructure-systems-operations-workflows`
Plan: `docs/plans/2026-08-11-infrastructure-systems-operations-workflow.md`
Decision: D-088, preserving D-079 and D-082 through D-087
Baseline: clean synchronized `main` at `a5d7ba1`

## Goal

Implement exactly two separate deterministic Rust-only, fixture-only/no-I/O
workflows above the unchanged runtime boundary:

- Personal Assistant -> Cloud Infrastructure -> QA & Validation -> Security &
  Risk -> Personal synthesis.
- Personal Assistant -> Systems Operations -> QA & Validation -> Security &
  Risk -> Personal synthesis.

## Implemented boundary

The immutable Cloud built-in contains synthetic Terraform configuration, Azure
architecture, and application validation evidence. The immutable Systems
built-in contains a synthetic service snapshot, sanitized log excerpt,
recovery scenario, and application validation evidence. Callers select only
closed scenario IDs; they cannot supply paths, accounts, tenants, endpoints,
hosts, services, credentials, environment variables, commands, or trusted
identity.

Strict bounded contracts include `InfrastructureAssessment`, inert
`ChangePlan`, `OperationalAssessment`, evidence-bound or hypothetical
`DiagnosticFinding`, module-qualified `ValidationReport` and `RiskAssessment`,
and final Personal synthesis. Every validated stage preserves exact scenario,
fixture, criterion, evidence, task, run, predecessor, and result provenance.
Evidence is only `ObservedFixture` or `NotRun`; it cannot represent a live
command, provider response, host observation, credential check, or executed
external test.

QA reconciles every acceptance criterion and cannot approve, fabricate
execution evidence, or treat `NotRun` as a pass. Security remains evidence-
bound or explicitly hypothetical; it cannot authorize, remediate, replace
policy, or invent unavailable credential/platform evidence. Final synthesis
derives a closed approval requirement but creates no approval request or
execution subject.

## Lifecycle and limits

The two trusted application-service selectors are distinct and mutually
exclusive with one another and every prior generic/document/workflow selector.
For either selector, `AgentOrchestrator` alone creates the named first
specialist, QA, and Security as three sequential depth-one siblings beneath the
same Personal root, then starts fresh Personal synthesis. No specialist spawns,
delegates, invokes another specialist, or selects a workflow.

Each workflow is limited to four tasks, three non-replenishing children, five
runtime attempts, one active child, depth one, 32 runtime/generic events, 16
workflow events/audit records, and zero retries. Each selected-text input is at
most 24,576 bytes and is proven to fit Native's unchanged 65,536-byte encoded
gateway boundary. Terminal parsing, task output, remaining capacity, successor
input, and successor state are prepared before terminal event acceptance.
Continuation-start failure never reverses an accepted event or replenishes a
budget. Cancellation is child-first and late events fail closed.

First-stage failure skips QA/Security and permits only truthful fallback. QA
failure may continue to Security with QA explicitly unavailable. Incomplete QA
or Security failure forces partial synthesis. Synthesis failure fails the root.
Raw invalid output never reaches a successor.

## Preserved authority boundaries

Terraform and platform commands, live inventory or diagnostics, cloud/system
mutation, IAM/firewall/account changes, service/process control, reboot/
shutdown, configuration/package/patch changes, privileged shell, VMware/
backup mutation, credential access/rotation, filesystem/network access, and
every consequential capability remain denied inert proposal data. No tool,
command, credential, live access, executor, approval dispatch, provider, IPC/
UI, dependency, permission, persistence, external runtime, or effect was
added.

Cloud and Systems are `Initial` only for their separate sealed unwired
selectors. QA and Security remain advisory. All four policy profiles remain
tool-ineligible, all four memory profiles remain disabled, runtime tool
proposals fail closed, and execution remains `NotAttempted`.
`AgentRuntime`/`NativeAgentRuntime` are unchanged and Native remains sole/
default. String and credential-pattern guards are defense-in-depth only and
cannot authorize any future live/effect path.

## Evidence

- Infrastructure/operations domain units: 8 passed.
- Orchestrator units: 11 passed.
- Public D-088 contract: 25 passed.
- Rust formatting, all-target/all-feature check, strict Clippy, repository scan,
  and diff hygiene passed.
- Independent all-target Rust: 362 passed with one intentionally ignored
  opt-in real-Hermes probe.
- Final `npm run verify`: passed, including 124 frontend tests, 195 library
  tests, all integration contracts, TypeScript, Vite, and Tauri no-bundle
  release build.
- Final documentation, repository, security, diff, session-end, and
  completion-marker checks pass.
- No manual application or target-environment check is required because the
  Rust workflows remain unwired and perform no I/O.

The initial full verification stopped only on native-roadmap formatting, which
targeted Prettier corrected. A second run reached the repository scan and found
four synthetic secret-shaped test literals; the test was corrected to runtime-
assemble equivalent sentinels. The focused sentinel test and repository check
passed, then final `npm run verify` passed end to end. These are resolved gate
discoveries, not open defects.

Quality result: `PASS WITH ADVISORIES`. Before another workflow or live/tool
increment, decompose the large private infrastructure module/orchestrator
integration into smaller typed private components without creating a general
engine. String/credential guards remain defense in depth and cannot authorize a
later live/effect path. No later owner-approved plan is Ready; next-increment
readiness is `Blocked`.

## Rollback

Before publication, remove the infrastructure/operations module and public
contract, restore the prior instruction/activation definitions, remove only the
two sealed selector integrations, and revert D-088 current-state documentation.
Preserve D-079 and D-082 through D-087 and all historical evidence. There is no
infrastructure, host, service, process, VM, credential, provider, file,
network, approval, execution, or other external state to reverse.
