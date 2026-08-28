# Native multi-agent final architecture and security review ExecPlan

Status: Ready with advisories; separate owner approval required before review
Increment: `native-multi-agent-final-review`
Owner: Project owner
Last updated: 2026-08-28
Roadmap: [`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Perform one read-only, source-current review of completed native multi-agent
phases and the separately labelled sealed Research -> Knowledge presentation.
Reconcile current implementation against D-079 and D-082 through D-093,
distinguish completed remediation from historical findings, and produce one
evidence-backed architecture/security report without changing behavior or
granting new authority.

## Current-state evidence

- Native phase evidence through registry, orchestration, governance, volatile
  memory, sealed workflows, bounded parallelism, fixture demonstrations, and
  the frontend Command Center is complete. The generic catalog/task/progress UI
  remains frontend-fixture-only.
- NativeAgentRuntime remains sole/default, provider-free, and unwired. The only
  Tauri-connected exception is one process-local, no-input, content-free
  Research -> Knowledge lifecycle panel. It is visibly simulated and separate
  from the fixture graph, Conversations mock, and acceptance workflows.
- F-01/F-02, F-07, F-08, F-12, and F-15 have later bounded completion evidence.
  The final review must validate their current disposition rather than restate
  historical pre-remediation findings as current defects.

## Exact review scope

Review only these evidence families at the source-current reviewed head:

1. Decisions D-079 through D-093; `ARCHITECTURE.md`, `SECURITY.md`,
   `SECURITY_CHECKLIST.md`, `CODE_REVIEW.md`, `TESTING_GUIDE.md`, the root and
   native-multi-agent roadmaps, and current project-memory records.
2. `src-tauri/src/agent/**`, its public contracts under `src-tauri/tests/`, and
   sealed workflow/runtime/approval/memory/audit boundaries they exercise.
3. Command Center and lifecycle evidence limited to
   `src/features/command-center/**`, `src/infrastructure/tauri/**`,
   `src-tauri/src/research_knowledge_demo_lifecycle*.rs`, current Tauri
   capabilities/CSP, and F-12 repository-health tests.
4. Completed phase plans, increments, security reviews, post-increment reports,
   and the latest native-nine-agent review as immutable historical evidence.

The review may add only its comprehensive review artifact, post-increment
report, and strictly necessary live project-memory reconciliation. It must not
modify source, tests, dependencies, lockfiles, configuration, capabilities,
CSP, decisions, or dated historical evidence.

## Review matrix and invariants

| Area                        | Required question                                                                                                      | Required evidence                                               |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| Ownership and identity      | Does Rust retain agent/task/run/runtime authority and reject foreign or rejected starts?                               | D-079/D-082, F-01/F-02 evidence, runtime/orchestrator contracts |
| Delegation and cancellation | Are depth, routing, parallelism, cleanup, and terminal states deterministic and fail closed?                           | D-083/D-086/D-091 workflow and public contracts                 |
| Governance and approvals    | Do specialist roles remain advisory and does approval remain application-owned?                                        | D-084/D-090/D-093 policy, approval, and audit tests             |
| Data and privacy            | Are memory, documents, audit, errors, and fixture content bounded and volatile?                                        | D-085 and memory/document/security evidence                     |
| UI and IPC                  | Does the WebView remain untrusted and the lifecycle exception stay narrow?                                             | D-092, F-07/F-08/F-12, client/panel/contract tests              |
| External authority          | Are provider, model, network, credentials, tools, filesystem, persistence, background work, and device effects absent? | source/config/dependency inspection                             |
| Portability and rollback    | Are platform claims bounded and findings reversible without history rewrite?                                           | pinned checks, reports, Git scope                               |

Every conclusion must label its subject **Current**, **Mocked**, **Planned**, or
**Prohibited**. A historical finding is current only when source-current code or
fresh required evidence proves it remains open.

## Threat, test, and manual evidence

- Trace hostile WebView input, forged/stale event data, foreign runtime identity,
  rejected-run cleanup failure, cancellation race, capability/CSP widening,
  fixture-to-authority confusion, document/memory disclosure, and accidental
  provider/tool/device reachability.
- Run source-current acceptance, complete verification, focused native contract
  checks selected by findings, frontend/type checks, dependency inventory,
  security scan, audit, documentation/repository checks, and diff checks.
- Inspect exact Git state, Actions for the reviewed head, manifests/lockfiles,
  capabilities/CSP, and the declared source boundary.
- Target-Mac or rendered checks are `Not run` unless separately owner-required;
  historical evidence may be cited only with its original limitations.

## Severity, stop conditions, and decisions

Use Critical, High, Medium, Low, or Advisory. Any Critical or High current
finding blocks completion. A needed source change, new permission, dependency,
provider, credential, external action, or revised decision is a stop condition:
report it and do not remediate it. Stop if evidence cannot distinguish a sealed
simulation from a live capability, if the reviewed head changes, or if Git state
is dirty before review starts.

No new decision is authorized. Preserve D-093's separate checkpoint-denial and
manual-dispatch branches; do not infer an approval-to-dispatch bridge.

## Verification and completion

Required commands are `npm run test:agent-acceptance`, `npm run verify`,
`npm run test:frontend`, `npm run typecheck`, `npm audit --audit-level=low`,
`npm run docs:check`, `npm run repository:check`, `npm run security:scan`, and
`git diff --check`. Add focused `cargo test --locked` commands only when a
current finding requires their exact contract. Record every command and manual
check as Passed, Failed, Not run, or Manual verification pending. Run
architecture, security, code-health, technical-debt, and readiness reviews, then
the post-increment gate. Completion grants no remediation authority.

## Rollback and approval boundary

Before commit, rollback only review/report and live-reconciliation documents by
an inverse patch. After publication, use a documentation-only revert PR; never
reset, rebase, or rewrite historical evidence. The final review requires
separate explicit owner approval after this plan is published. Any later
remediation requires its own bounded plan and approval.
