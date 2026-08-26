# Native nine-agent architecture comprehensive review

Date: 2026-08-26

Mode: Comprehensive review only

Baseline commit: 74343c8533ea22304abd506b03daa3df12925840

Baseline tree: 6c984175736465fda9c4acd64fbaca04e1f9e03e

Reviewer: Codex with independent core-lifecycle, governance-boundary, and UI-quality reviews

## Result

PASS WITH ADVISORIES for the deterministic, fixture-only, unwired native
architecture that the repository actually implements.

Blocked for a live provider, connected agent IPC, real tool execution,
provider-backed concurrency, production approval UX, or public release until
the applicable prerequisites in this review are resolved through separately
approved increments. Public release also requires the audit, documentation,
licensing, notice, portability, and release gates identified below.

No P0 Critical or P1 High finding was identified. Six P2 Medium and ten P3
Low findings remain. No finding permits a model, specialist, runtime event, or
WebView to perform a current unauthorized external operation: there is no agent
executor, live provider, agent Tauri command, agent network path, or
PlatformAdapter implementation.

## Executive assessment

The current implementation is a strong deterministic security and lifecycle
proof, not a complete user-ready nine-agent product. Its most important
strength is that authority is application-owned and structurally separate from
model output:

- AgentRegistry contains exactly nine closed definitions.
- AgentOrchestrator alone creates tasks and derives live execution contexts.
- ToolRegistry, PolicyEngine, ApprovalManager, memory grants, document grants,
  and audit records use closed application-owned types.
- Specialist policies are empty; only Personal Assistant recognizes the two
  current local schemas, and every agent governance result remains
  NotAttempted.
- Workflow Automation can propose only five exact templates, has zero
  executable tool steps, zero retries, and zero nested workflow depth.
- Coding, Cloud, and Systems workflows inspect sealed fixture data and have no
  repository, cloud, host, shell, Git, dependency, network, or credential
  capability.
- The Command Center is persistently disclosed as simulated fixture data and
  has no agent backend connection.
- NativeAgentRuntime is sole/default. Hermes is absent from production code and
  remains Deferred/Blocked with preserved NO GO evidence.

The highest-risk gaps are all at future connection boundaries. Legacy runtime
starts do not verify the adapter-returned identity and can lose a rejected live
run on cancellation failure. The cataloged role instructions are not bound to
runtime invocations. Earlier task/workflow families have no liveness deadline.
These do not create a current effect because NativeAgentRuntime is local and
unwired, but they are mandatory blockers for any live or external runtime.

The architecture is intentionally broader than a private single-user project
needs today. That breadth is defensible where it protects authority, identity,
approval, memory, and document boundaries. The large workflow state machines
and frontend projection, however, now create material review coupling. Further
workflow-family expansion should stop until private lifecycle ownership is
decomposed.

## Scope and current-state truth

### Reviewed

- Rust agent definitions, registry, tasks, runtime, native adapter,
  orchestrator, governance, workflow, memory, document, approval, audit, tool,
  policy, credential, storage, and Tauri boundaries.
- Public and private Rust contracts, deterministic MockAgentRuntime behavior,
  frontend state and Command Center contracts, dependency manifests and
  lockfiles, fixtures, audit records, and acceptance documentation.
- AGENTS.md, MASTER_PROMPT.md, current project memory, SECURITY.md,
  SECURITY_CHECKLIST.md, CODE_REVIEW.md, TESTING_GUIDE.md, architecture
  documents, ADRs, D-079 through D-093, relevant ExecPlans, completed increment
  records, post-increment reviews, and Git history from the native runtime
  through deterministic end-to-end acceptance.

### Not present and therefore not credited

- No configured model provider or live model call.
- No agent/runtime/provider Tauri command or event.
- No real tool dispatcher or executor.
- No agent-owned shell, filesystem, Git, package-manager, cloud, systems,
  network, or credential route.
- No production PlatformAdapter.
- No durable multi-agent task, memory, document, workflow, or unified audit
  store.
- No real operating-system concurrency, provider-session concurrency, hard
  preemption, or app-global capacity control.
- No HermesAgentRuntime or other runtime selector.

The Desktop UI to Application Services to AgentOrchestrator flow in the target
architecture is therefore not connected. The Rust core and frontend projection
are separate deterministic proofs.

## Architecture and trust-boundary map

| Boundary                      | Current owner                                            | Observed behavior                                                                             | Review conclusion                                                       |
| ----------------------------- | -------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Desktop UI                    | React application shell                                  | Mock conversation flow and simulated Command Center; no agent IPC                             | Safe and honestly disclosed, with CSP and app-info validation gaps      |
| Application services          | React service injection plus Rust composition roots      | App-info/menu routing on the UI; agent services remain Rust-only                              | Agent service layer is not product-connected                            |
| AgentOrchestrator             | Rust AgentOrchestrator                                   | Creates root/children, routes runs/events, validates live context, sequences sealed workflows | Correct authority owner; legacy start and liveness defects remain       |
| AgentRegistry and definitions | Rust closed catalog                                      | Exactly nine immutable identities with derived profiles and versioned instruction sources     | Strong identity catalog; instructions are not invocation-bound          |
| Task/workflow lifecycle       | Rust closed state machines                               | Bounded deterministic transitions, cancellation, partial failure, stable ordering             | Strong fixture proof; deadlines are inconsistent and complexity is high |
| NativeAgentRuntime            | Rust wrapper over InitialGatewayTurn                     | Sole/default, streaming-text only, no provider/network/process                                | Safe current default; not a configured provider                         |
| ToolRegistry                  | Rust closed catalog                                      | get_current_datetime v1 and create_local_task v1 only                                         | Unknown tools fail closed                                               |
| PolicyEngine                  | Rust deterministic engine                                | Re-derives outcome from sealed schema/profile metadata                                        | Agent text cannot authorize                                             |
| ApprovalManager               | Rust exact-subject manager and trusted macOS source      | Exact binding, TTL, manager/source/presentation checks, single use                            | Authorization integrity strong; UX/lifecycle completion gaps remain     |
| Audit                         | Separate bounded in-memory adapters and workflow vectors | Redacted typed evidence; no durable unified logger                                            | Safe but incomplete for production accountability                       |
| MemoryStore                   | Workflow-local volatile Rust store                       | Sealed live grants, private/task/shared namespaces, proposal/review, cleanup                  | Strong isolation; mutations are not unified-audited                     |
| Documents                     | ApprovedDocumentReader                                   | Selected txt/md only, root/symlink/identity/size/content checks                               | Strong boundary with accepted Unix open race                            |
| PlatformAdapter               | Not implemented                                          | No agent effect route                                                                         | Safe absence; target architecture is not operationally complete         |

## Role separation

| Boundary pair                            | Evidence and conclusion                                                                                                                                                                                                                                                                                                                       |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Personal Assistant vs AgentOrchestrator  | Personal may request delegation and synthesize results, but cannot create a task. AgentOrchestrator owns task IDs, root lineage, child creation, run assignment, event routing, cancellation, workflow state, and live attribution. No authority overlap found.                                                                               |
| QA vs ApprovalManager                    | QA receives validated fixture evidence and returns advisory validation. Its instruction explicitly forbids approval and policy authority; its policy profile is tool-ineligible. ApprovalManager owns exact subjects, presentations, resolution, TTL, source validation, and consumption. No overlap found.                                   |
| Security vs PolicyEngine                 | Security returns bounded advisory risk output and cannot authorize or alter policy. PolicyEngine derives decisions from trusted local schema, risk, permission, and profile metadata. Severity text never becomes permission. No overlap found.                                                                                               |
| Workflow Automation vs AgentOrchestrator | Workflow Automation returns an exact proposal from a five-template catalog. It cannot create tasks, dispatch itself, execute tools, approve, recurse, schedule, persist, or modify limits. AgentOrchestrator issues the take-once application-owned manual dispatch and sequences only existing sealed workflows. No authority overlap found. |
| Cloud vs Systems                         | Cloud is limited to synthetic Terraform/Azure fixture assessment. Systems is limited to sanitized service/log/recovery fixtures. Each has separate scenario types and structured validators; neither can access live control planes, hosts, credentials, or commands. Responsibility is distinct.                                             |
| Research vs Knowledge & Document         | Research analyzes supplied source evidence and preserves provenance. Knowledge consumes approved documents or validated research evidence, organizes it, and may propose shared knowledge without persistence. Retrieval and knowledge organization are separate.                                                                             |

### Per-agent authority result

| Agent                | Current effective capability                                                                  | Prohibited authority confirmed                                                                      |
| -------------------- | --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Personal Assistant   | Root response, controlled orchestration request, synthesis, two governed local tool proposals | Child creation, policy, approval decision, execution, device access                                 |
| Research             | Supplied-evidence analysis in sealed workflows                                                | Browser, network, filesystem, tool, delegation                                                      |
| Coding               | Synthetic fixture review and inert proposal                                                   | Live repository, file writes, shell, tests, Git, dependencies, network, secrets                     |
| Cloud Infrastructure | Synthetic Terraform/Azure assessment and inert plan                                           | Credentials, APIs/CLI, init/plan/apply, state, IAM, firewall, delete/deploy                         |
| Systems Operations   | Sanitized fixture diagnosis and inert recovery plan                                           | Privileged shell, services/processes, packages, accounts, config, VMware, backup mutation           |
| Knowledge & Document | Approved txt/md content and validated context processing                                      | Ambient filesystem, provider/network, silent shared persistence, delegation                         |
| QA & Validation      | Evidence/criterion reconciliation and advisory gaps                                           | Approval, policy mutation, evidence fabrication, execution                                          |
| Security & Risk      | Evidence-bound advisory assessment                                                            | Authorization, PolicyEngine mutation, remediation execution, unnecessary secrets                    |
| Workflow Automation  | Exact typed fixture proposal; A-D may yield a separate take-once manual dispatch              | Arbitrary steps/tools/shell, nesting, retry, recursion, schedule, self-modification, approval reuse |

The role definitions are clear and authority profiles are separate. Finding
F-03 reduces confidence in live semantic specialization because the embedded
role instructions are not part of runtime invocation composition.

## Trusted identity and forgery analysis

| Candidate forgery            | Can model/UI supply the trusted value?                                                        | Evidence and residual                                                                                                               |
| ---------------------------- | --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| AgentId                      | No                                                                                            | Closed enum; task definition is resolved by application selectors. Workflow JSON agent IDs must match the exact canonical template. |
| Task ID                      | No                                                                                            | AgentOrchestrator generates root and child IDs; constructors and task fields are private.                                           |
| Root task ID                 | No                                                                                            | Derived from the task lineage and checked through live context.                                                                     |
| Policy profile               | No                                                                                            | Derived from the built-in AgentDefinition mapping; public model data cannot construct a different profile binding.                  |
| Runtime ID                   | No                                                                                            | Derived from runtime descriptor at orchestrator construction.                                                                       |
| Runtime run/request identity | Not from model output, but a future adapter can violate the request contract on legacy starts | F-01: legacy start trusts the returned identity instead of checking the application request.                                        |
| Memory namespace/profile     | No                                                                                            | Profile derives from AgentDefinition; writes accept only closed targets and sealed live grants.                                     |
| Workflow identity            | No                                                                                            | Application-owned scenario/template selectors and exact proposal equality checks own identity.                                      |
| Approval state               | No                                                                                            | ApprovalManager-issued subject, manager instance, presentation, trusted source, TTL, and consumed-state checks are required.        |

AgentAttribution fields are private and are derived from a live
AgentExecutionContext with a crate-sealed proof. Replayed or foreign contexts,
unknown tools, mismatched versions, malformed arguments, disabled profiles,
stale events, and duplicate subjects fail closed. F-01 is the exception at the
runtime-adapter boundary and must be resolved before that boundary becomes
external.

## Delegation, lifecycle, and workflow review

### Delegation

- The generic delegation matrix permits only Personal Assistant to Research.
- Generic delegation depth is one, one child may ever be created, only one
  child may be active, and completion does not replenish the child budget.
- Specialist-to-specialist spawning is denied.
- Later multi-stage workflows do not let specialists delegate. They are
  application-selected sibling children of the Personal root under
  workflow-specific sealed catalogs.
- Bounded parallelism admits at most three specialist work items and five total
  runs under one root, with stable ordinal synthesis.
- Unknown, self, reverse, disabled, deferred, over-depth, over-count, recursive,
  and post-output delegation paths are rejected before task creation.

### Lifecycle

The implementation has closed task states, event sequence checks, output
bounds, child-first cancellation, retryable cancellation failures in the newer
paths, partial-failure projection, strict successor preparation, and stable
audit ordering. Workflow Automation and bounded parallelism use monotonic
cooperative deadlines. No workflow performs an automatic retry.

Residual lifecycle risks are:

- legacy runtime identity and rejected-run cleanup, F-01 and F-02;
- no deadline for the generic and D-083 through D-088 paths, F-04;
- an approval rejection does not itself resolve the root task, F-06;
- operational concurrency and global backpressure are not implemented, F-11.

### Workflow safety

- Research/Knowledge preserves source IDs, rejects unknown provenance, supports
  typed partial results, and supplies only approved shared context.
- Engineering sequences Coding to QA to Security to Personal synthesis. Coding
  cannot apply a change; QA cannot approve; Security cannot authorize.
- Infrastructure and Systems workflows have separate closed fixture catalogs,
  reject credential-like and live-operation claims, and never execute
  Terraform, cloud, service, process, VMware, or backup operations.
- Workflow Automation validates duplicate keys, closed agents, exact template
  equality, dependency existence, self/duplicate dependencies, cycles, limits,
  tool schemas, checkpoint binding, and authority claims. Canonical limits are
  four steps, three agent tasks, zero executable tools, zero retries, zero
  nested workflows, and 120 seconds.
- Manual A-D dispatch is application-owned, take-once, expiring, and maps only
  to four already-implemented sealed workflows. Document-to-Action remains
  proposal-only.
- D-093 deliberately proves approval-checkpoint denial and manual safe dispatch
  as separate branches. There is no approval-to-dispatch bridge; F-14 records
  the accepted gap without recommending a shortcut.

## Governance-bypass trace

| Capability              | Current path                                                                                                                  | Can it bypass ToolRegistry, PolicyEngine, ApprovalManager, or audit?        |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Shell execution         | No agent shell tool, executor, IPC, or process adapter exists                                                                 | No current path                                                             |
| Filesystem access       | SQLite bootstrap is outside agents; documents use ApprovedDocumentReader                                                      | Agent document access cannot bypass the reader; no agent write path         |
| Document access         | Application registers a selected file/root; reader validates path, type, identity, size, UTF-8, and control characters        | No ambient path access; F-10 is a low Unix race residual                    |
| Code modification       | Coding sees immutable synthetic fixture labels/content only                                                                   | No current path                                                             |
| Git operations          | No Git tool or process path                                                                                                   | No current path                                                             |
| Dependency installation | No package-manager tool or process path                                                                                       | No current path                                                             |
| Network use             | No agent/provider client; no agent IPC; production CSP still has a local dev socket allowance                                 | No agent path; F-07 is WebView defense-in-depth                             |
| Cloud operations        | Cloud outputs validated inert fixture assessments                                                                             | No credentials, API, CLI, state, apply, delete, IAM, or firewall path       |
| Systems operations      | Systems outputs validated inert fixture assessments                                                                           | No host, service, process, package, account, config, VMware, or backup path |
| Credentials             | Agent input validators reject credential claims; credential probe is disconnected from Tauri/agents                           | No agent credential route                                                   |
| Memory writes           | Orchestrator derives a live grant; namespace and owner checks apply                                                           | No model-direct write; F-09 notes missing unified audit                     |
| Workflow dispatch       | Application issues a sealed take-once dispatch for A-D only                                                                   | Workflow Agent cannot dispatch; no tool execution follows                   |
| Tool proposal           | Registry lookup, version/schema validation, profile check, deterministic policy, exact approval when required, redacted audit | No execution follows; disposition is always NotAttempted                    |

The absence of an executor is an important safety fact, not evidence that a
future executor would automatically be safe. Any effect path must preserve the
entire ToolRegistry to PolicyEngine to ApprovalManager to AuditLogger to
restricted implementation chain.

## Memory, documents, and provider transmission

### Memory

- Memory is bounded, workflow-local, process-volatile, and root-bound.
- AgentPrivate is readable only by the owning agent.
- TaskTemporary is readable only by the owning agent and task and is removed on
  task cleanup.
- ProposedShared is not readable as shared memory.
- ApprovedShared exists only after application review with optimistic version
  checks; agents cannot silently promote it.
- Personal Assistant is the only profile that directly reads ApprovedShared;
  cross-agent context is explicitly selected through the orchestrator.
- Coding, Cloud, Systems, QA, Security, and Workflow profiles have memory
  disabled.
- Disabling memory clears records and proposals; approved shared records have an
  application-owned delete path.

No provider exists, so no memory is currently transmitted externally. A future
provider plan must separately define exactly which selected context crosses the
provider boundary, with minimization and audit; current safety does not grant
that authority.

### Documents

- Only txt and md are accepted.
- Direct paths and approved roots reject symlinks and non-regular files,
  canonicalize, enforce root containment, compare Unix file identity, bound
  size, require UTF-8, and reject NUL/control content.
- Content is selected by application-owned document descriptors and is not
  generally enumerable.
- No document is persisted to shared memory without a separate proposal and
  application review.
- The accepted pure-standard-library open race is F-10.

No document route is connected to the WebView or a provider.

## Audit assessment

Current audit records are typed, bounded, redacted, content-free, and
application-attributed. Tool governance captures agent, task, root, parent,
runtime, policy/memory profiles, run identity, action, policy, approval
disposition, execution disposition, and closed error. Workflow records preserve
workflow stage, child, predecessor, ordering, outcome, and partial-failure
attribution. Exact approval subject and trusted interaction evidence is retained
by the LegacyGateway approval-audit adapter, not by the agent-governance audit
path: AgentGovernanceService consumes the resolution and retains only its closed
disposition and execution result.

F-09 prevents an Excellent rating: the evidence is split across adapters and
workflow vectors, agent governance drops exact approval ID/source/interaction
correlation, memory/document lifecycle mutations have no unified audit record,
and all multi-agent audit is process-volatile. There is also no implemented
user/principal identity; the root task is the highest current correlation
boundary. This is sufficient for the deterministic proof but not for production
forensics or durable user accountability.

## UI, Tauri, dependency, and publication assessment

### UI and Tauri

- src-tauri/src/lib.rs registers only get_app_info. No agent, task, workflow,
  tool, approval, memory, document, credential, or executor command is exposed.
- The main window capability contains only core:default.
- App.tsx injects the existing app-info, closed menu event, and browser mock
  services. Command Center imports no Tauri, network, storage, clipboard, or
  device API.
- The Command Center disclosure is DEMO MODE · SIMULATED AGENT DATA and its
  footer says deterministic fixture · no backend connection. Availability and
  health are explicitly fixture-only/not measured.
- Projection validation enforces closed IDs, references, bounds, cycles,
  authority wording, provenance, protected fields, and immutability.
- The rendered browser/Tauri validation is recorded as complete, including
  resize, input, focus, overflow, themes, reduced motion, scroll, screenshots,
  and 125 percent manual zoom.

The UI cannot currently forge native identities, call tools, bypass approvals,
alter native task state, or access secrets through an agent route. F-07, F-08,
and F-12 cover the remaining UI-boundary risks.

### Dependencies

- JavaScript and Rust direct versions are exact; lockfiles are present.
- Production JavaScript dependencies are limited to Tauri API, React,
  ReactDOM, React Flow, and Lucide. React Flow is isolated behind one adapter.
- Rust direct dependencies are Tauri, rusqlite with bundled SQLCipher/OpenSSL,
  serde, serde_json, thiserror, and the macOS dialog/security-framework
  dependencies.
- Only esbuild and fsevents install scripts are allowlisted.
- npm audit reported zero vulnerabilities for the current lockfile.
- cargo-audit 0.22.2 is pinned in CI with an accepted-baseline gate, but the
  cargo-audit binary was not installed locally and was not independently rerun
  in this review. Cargo dependency resolution and the complete locked build
  passed.
- No Hermes, external agent framework, provider SDK, cloud SDK, shell wrapper,
  or generic execution dependency exists.

### Future publication

The application-owned ports and closed domain contracts are publishable in
principle without fundamental rearchitecture. Publication does not require
billing, multi-tenancy, enterprise IAM, a marketplace, or SaaS infrastructure.
It does require the live-boundary prerequisites in this review, a clearer
internal module structure, durable privacy-preserving audit decisions, target
portability evidence, and licensing/notice work in F-16.

## Ranked findings

### F-01 — Legacy runtime starts trust adapter-returned identity

- Severity: P2 Medium.
- Evidence: RuntimeTurnRequest owns the expected identity at
  src-tauri/src/agent/runtime.rs:246-286. AgentOrchestrator::start_runtime_run
  checks only status at src-tauri/src/agent/orchestrator.rs:5923-5933.
  ActiveRun::context then trusts run.identity at orchestrator.rs:781-790, and
  event ingress validates against that returned identity at 2093-2115. The
  D-091-only helper performs the missing exact and duplicate-live comparison at
  5935-5967. MockAgentRuntime can return foreign or duplicate identities at
  src-tauri/tests/support/mock_agent_runtime.rs:224-244.
- Affected files and symbols: src-tauri/src/agent/orchestrator.rs,
  start_runtime_run, ActiveRun::context, every legacy root/child/successor/
  synthesis caller; src-tauri/src/agent/runtime.rs, RuntimeTurnRequest;
  src-tauri/tests/support/mock_agent_runtime.rs.
- Failure or abuse scenario: a faulty or hostile future adapter returns an
  AwaitingStart run with a stale, foreign, or reused identity. The orchestrator
  binds it into trusted context, so the stored run and trusted context disagree
  with the application-requested identity. A late or cross-workflow event with
  the reused identity can be accepted if routed to the affected task.
- Impact: violates application-owned identity provenance, can stall workflows,
  confuses audit/approval correlation, and makes an external runtime boundary
  unsound. It does not currently change agent/profile authority.
- Recommended remediation: replace all runtime-start paths with one mutable
  helper that compares the exact request identity, rejects any duplicate live
  identity, and uses the existing rejection quarantine.
- Required regression test: inject foreign, stale, and duplicate identities at
  root and every child/successor/synthesis start; assert no trusted context,
  task, workflow, or audit mutation and no accepted late event.
- Fix now or defer: next bounded hardening increment; mandatory before any live
  provider, external runtime, or agent IPC.
- Remediation risk: Medium-High because all workflow families use the legacy
  helper and differ in prepared-state/fallback behavior.

### F-02 — Failed legacy rejection cancellation can drop the live-run handle

- Severity: P2 Medium.
- Evidence: start_runtime_run calls cancel and propagates an error, dropping the
  run, at src-tauri/src/agent/orchestrator.rs:5923-5933. It also does not inspect
  contradictory AlreadyTerminal(nonterminal) outcomes. The D-091 path retains
  rejected runs at 5970-5989 and exposes cleanup retry near 4017-4054.
  MockAgentRuntime records nonterminal drops, and LEGACY-TD-01 is preserved in
  docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md.
- Affected files and symbols: src-tauri/src/agent/orchestrator.rs,
  start_runtime_run, reject_unbound_runtime_run,
  retry_rejected_runtime_cleanup; all legacy start callers.
- Failure or abuse scenario: a future network/process runtime begins work,
  reports an invalid nonterminal initial status, and cancellation errors or
  contradictorily reports a nonterminal terminal result. The orchestrator loses
  the only handle and may start fallback work.
- Impact: orphan external session, duplicate work, resource/data-processing
  leak, and no retryable cleanup owner.
- Recommended remediation: quarantine every rejected nonterminal run, block all
  new/fallback starts while cleanup is pending, and reconcile exact cancellation
  outcomes.
- Required regression test: unexpected start status plus one-shot/permanent
  cancellation failure and contradictory disposition at root and continuation;
  assert RuntimeCleanupPending, one retained run, zero fallback start, zero
  nonterminal drop, and successful explicit retry.
- Fix now or defer: fix with F-01 before any external runtime/provider work.
- Remediation risk: Medium-High because error and terminal projection ordering
  changes across fallback paths.

### F-03 — Versioned agent instructions are not bound to runtime invocations

- Severity: P2 Medium.
- Evidence: AgentDefinition stores and validates nine versioned instruction
  sources at src-tauri/src/agent/definition.rs:283-379, but production consumers
  do not call AgentDefinition::instructions. RuntimeTurnRequest carries only
  identity and selected_text at src-tauri/src/agent/runtime.rs:246-286.
  runtime_request forwards only generated IDs and user/workflow text at
  orchestrator.rs:5909-5921; NativeAgentRuntime forwards that text unchanged at
  native_runtime.rs:47-61. Stage builders include closed contracts and some role
  labels, but not the cataloged instruction source/version.
- Affected files and symbols: src-tauri/src/agent/definition.rs,
  AgentDefinition and AgentInstructionSource; src-tauri/src/agent/runtime.rs,
  RuntimeTurnRequest; src-tauri/src/agent/orchestrator.rs, runtime_request;
  workflow input builders; NativeAgentRuntime.
- Failure or abuse scenario: a future provider receives a Research, QA,
  Security, or Personal run without a trusted role/instruction profile or
  separated trusted/untrusted lanes. The only semantic instruction is the
  selected workflow/user text.
- Impact: host governance remains sealed, but live role specialization,
  instruction-version provenance, and prompt-injection resistance are not
  proved.
- Recommended remediation: add an application-owned invocation profile with
  exact AgentId and instruction source/version and preserve strict trusted
  instruction versus untrusted objective/evidence separation.
- Required regression test: capture every root, specialist, and synthesis
  request; assert exact role/version binding for all nine agents; inject
  adversarial objective/predecessor text; reject mismatched invocation
  profiles.
- Fix now or defer: defer for the fixture-only unwired proof; mandatory before a
  configured live model/provider.
- Remediation risk: High because AgentRuntime, native gateway serialization, all
  mocks, and all workflow fixtures would change.

### F-04 — Most task and workflow families have no liveness deadline

- Severity: P2 Medium.
- Evidence: D-083 explicitly excluded deadlines in
  docs/plans/2026-08-11-agent-orchestration-task-lifecycle.md:39-50. Generic,
  Research/Knowledge, Engineering, Cloud, and Systems workflow states have no
  clock/deadline. Event ingress enforces deadlines only for Workflow Automation,
  manual dispatch, and bounded parallelism at orchestrator.rs:2093-2123.
  Later deadlines are cooperative and cannot preempt synchronous runtime start.
- Affected files and symbols: src-tauri/src/agent/orchestrator.rs generic,
  ResearchKnowledgeWorkflowState, EngineeringQualityWorkflowState, and
  InfrastructureOperationsWorkflowState; workflow deadline helpers.
- Failure or abuse scenario: a runtime starts and emits nothing. A root stays
  Running or WaitingForChild indefinitely, retaining run capacity and
  task-temporary state until a trusted caller manually cancels.
- Impact: availability loss, stuck UI/workflow, resource retention, and no
  deterministic cleanup of a future provider session.
- Recommended remediation: add application-owned monotonic root/run leases to
  the earlier lifecycle families with child-first cancellation, ingress/poll
  enforcement, and rejected-run quarantine. Keep automatic retries disabled.
- Required regression test: injected-clock N-minus-1/N tests for direct root and
  every sequential stage, cancellation-failure resumption, exactly one
  DeadlineExceeded terminal, late-event rejection, no successor/retry, and
  memory/governance cleanup.
- Fix now or defer: defer under current deterministic scope; mandatory before
  live/provider wiring.
- Remediation risk: High because terminal ordering, partial results, audit, and
  cancellation are cross-cutting.

### F-05 — Native approval prompts cannot be dismissed when the task is cancelled

- Severity: P2 Medium.
- Evidence: MacOsNativeApprovalDecisionSource synchronously uses the rfd native
  message dialog and exposes no dialog handle/close operation. The repository
  preserves this as ARB-011 in
  docs/reviews/2026-07-16-advisory-remediation-backlog.md:464-486. Manager
  cancellation safely terminalizes state and rejects late outcomes, but cannot
  remove the visible prompt.
- Affected files and symbols: src-tauri/src/approvals/decision_source.rs,
  MacOsNativeApprovalDecisionSource; ApprovalManager cancellation/resolution
  integration; native approval UX.
- Failure or abuse scenario: a task is cancelled while the synchronous native
  dialog is visible. The underlying approval is no longer usable, but the stale
  security prompt remains and the user may interact with a misleading request.
- Impact: user-confusing approval state and accessibility/UX failure. Late
  interaction does not authorize execution.
- Recommended remediation: select a cancellable trusted native approval source
  or explicitly accept the residual after target-Mac security and accessibility
  testing; preserve default Reject and exact source evidence.
- Required regression test: cancel with a visible prompt, assert prompt closure
  or explicit stale-state UX, one Cancelled audit outcome, late-source
  rejection, and no execution.
- Fix now or defer: defer while approvals are not product-connected; mandatory
  before any real action/approval UX.
- Remediation risk: Medium because replacement UI must preserve trusted-source,
  focus, accessibility, redaction, and exact-binding properties.

### F-06 — Approval rejection is audited but not task-terminal feedback

- Severity: P3 Low.
- Evidence: the target-macOS regression at
  src-tauri/src/agent/orchestrator.rs:7010-7076 proves Rejected and NotAttempted
  while the root remains Running. resolve_governance_source_outcome returns the
  approval outcome but does not inject controlled denial text or settle the
  task. E2E-TD-02 is preserved in
  docs/reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md:78-87.
- Affected files and symbols: AgentOrchestrator::resolve_governance_source_outcome,
  AgentGovernanceService resolution, future application service/UI consumer.
- Failure or abuse scenario: a connected caller treats rejection as complete
  but leaves the task spinner Running, or resumes the model without an explicit
  denial contract.
- Impact: ambiguous lifecycle and misleading UI; no action occurs and replay is
  still denied.
- Recommended remediation: before connected approval UI, define an
  application-owned controlled-denial transition and presentation contract
  without turning rejection into model authority.
- Required regression test: reject an exact subject; assert no action, complete
  correlated audit, deterministic task/result state, explicit denial
  presentation, and replay/late-event rejection.
- Fix now or defer: defer until backend approval UI/IPC is separately approved.
- Remediation risk: Medium because task terminality versus continued
  conversation is a product/lifecycle decision.

### F-07 — Production CSP retains development network and inline-script allowances

- Severity: P2 Medium.
- Evidence: src-tauri/tauri.conf.json:26-34 places
  ws://localhost:1420 in production connect-src and unsafe-inline in script-src.
  The installed Tauri schema supports separate csp and devCsp. ARCHITECTURE.md:
  103-105 correctly says there is no network plugin permission, but it does not
  clearly disclose that the production CSP retains this development WebSocket
  source. ARB-012 already records the inline allowance as Medium at
  docs/reviews/2026-07-16-advisory-remediation-backlog.md:488-506.
- Affected files and symbols: src-tauri/tauri.conf.json app.security.csp;
  development/release WebView configuration; ARCHITECTURE.md.
- Failure or abuse scenario: an injected WebView script gains a broader inline
  execution surface and can connect to a local service on port 1420 in a
  production build.
- Impact: XSS/local-network defense-in-depth gap, not a current native
  agent-authority bypass.
- Recommended remediation: move development WebSocket policy to devCsp, narrow
  production connect-src to required bundled/IPC sources, and remove inline
  script if rendered target testing proves it unnecessary. Retain inline style
  only if a verified dependency requires it.
- Required regression test: static production/dev CSP assertion, Tauri release
  launch, development HMR smoke, IPC/app-info smoke, and Command Center rendered
  regression.
- Fix now or defer: next bounded security/configuration hardening increment,
  before live data or connected agent UI.
- Remediation risk: Medium because an incorrect CSP can blank the UI, break HMR,
  IPC, assets, or React Flow styling.

### F-08 — App-info IPC is compile-time typed but not runtime narrowed

- Severity: P3 Low.
- Evidence: src/infrastructure/tauri/app-info-client.ts:12-14 invokes directly as
  AppInfo. useCoreConnection at src/application/useCoreConnection.ts:20-47
  accepts any resolved value as ready, never requires secureCore true, and
  exposes arbitrary Error.message. Tests cover success/loading/rejection but
  not malformed or insecure responses. ARCHITECTURE.md:82 claims the response is
  narrowed. The current production command is trusted Rust and returns a closed
  AppInfo with secure_core true at src-tauri/src/app_info.rs:12 onward.
- Affected files and symbols: src/infrastructure/tauri/app-info-client.ts,
  fetchAppInfo and AppInfo; src/application/useCoreConnection.ts,
  useCoreConnection/getErrorMessage; App tests; ARCHITECTURE.md.
- Failure or abuse scenario: version-skewed or malformed IPC data, including
  secureCore false, is presented as Local core ready; a native error containing
  internal detail reaches diagnostics.
- Impact: version-skew robustness/UI integrity failure and possible low-grade
  error-detail disclosure; the current infallible trusted Rust command returns
  a closed AppInfo with secureCore true, and no agent execution authority
  follows.
- Recommended remediation: invoke unknown, validate an exact closed object with
  bounded strings/enums/no extra fields and secureCore true, and map failures to
  closed UI copy.
- Required regression test: missing, extra, wrong-type, oversized, invalid
  environment, secureCore false, and sensitive-error sentinel cases.
- Fix now or defer: next small UI-boundary hardening increment.
- Remediation risk: Low.

### F-09 — Consequential audit is fragmented and drops exact agent-approval correlation

- Severity: P3 Low.
- Evidence: src-tauri/src/audit/approval.rs:61-120 can record exact approval
  identity and interaction evidence, but its adapter accepts only
  LegacyGateway origin near approval.rs:259 and is not the agent-governance
  audit. AgentGovernanceService consumes ApprovalResolution near
  src-tauri/src/agent/governance.rs:731 and 847, while AgentToolGovernanceRecord
  at src-tauri/src/audit/governance.rs:69-153 has no approval ID or interaction
  evidence. Memory write/review/delete calls at orchestrator.rs:934-1035 derive
  live attribution but append no unified audit record. Workflow audit is stored
  in separate bounded vectors. The increment documentation explicitly calls the
  evidence volatile, process-local, and non-unified.
- Affected files and symbols: audit/approval.rs,
  audit/governance.rs/InMemoryAgentGovernanceAudit,
  AgentGovernanceService, AgentOrchestrator memory/document methods, workflow
  audit record families.
- Failure or abuse scenario: after an agent approval or volatile
  memory/document mutation, a reviewer cannot independently correlate the exact
  approval source/button interaction with governance/task/workflow records;
  process exit removes the evidence. There is no user/principal identity above
  the root-task correlation boundary.
- Impact: incomplete forensics, retention, and production accountability, not a
  current approval bypass.
- Recommended remediation: under a separate audit/privacy plan, unify
  metadata-only records around exact attribution, subject/call/approval
  identity, trusted interaction evidence, principal identity when separately
  implemented, and memory/document/workflow lifecycle. Keep content, paths,
  arguments, secrets, and reasoning absent.
- Required regression test: approve/reject/cancel/expire exact subjects and
  assert one correlated lifecycle; audit memory/document create/review/delete/
  cleanup; later test durable write failure atomicity and restart recovery.
- Fix now or defer: defer under the fixture-only scope; release blocker before
  real actions, a live provider with retained data, or user-facing durable
  memory/documents.
- Remediation risk: High because privacy, persistence, transaction, retention,
  export, and failure semantics are cross-cutting.

### F-10 — Unix approved-document open is not atomically no-follow

- Severity: P3 Low.
- Evidence: src-tauri/src/documents.rs:875-942 rejects symlinks, canonicalizes,
  and checks containment. read_validated_file_with_hook at 952-1005 compares
  before metadata, opened-handle identity, final handle/path identity, and size,
  but File::open at 963 is not an atomic per-component no-follow traversal. The
  accepted residual is documented in
  docs/plans/2026-08-12-agent-memory-approved-documents.md.
- Affected files and symbols: src-tauri/src/documents.rs,
  validate_direct_target, resolve_root_member, read_validated_file_with_hook.
- Failure or abuse scenario: an adversary races a path component or terminal
  entry between validation and open. Post-open identity checks prevent returned
  content on mismatch, but the process can open the wrong object briefly or
  block on an unexpected file type.
- Impact: bounded local denial/blocking or transient wrong-object open; no
  mismatched content is returned under current checks.
- Recommended remediation: when the route is activated or expanded, introduce
  a target-specific descriptor-relative openat/no-follow traversal behind a
  narrow adapter while preserving portability.
- Required regression test: adversarial component/terminal replacement,
  symlink, FIFO/device, and rename races; assert no content return, no hang, and
  exact closed error.
- Fix now or defer: defer while documents are unwired; mandatory before live
  document exposure or broader formats.
- Remediation risk: Medium due target-specific APIs and portability.

### F-11 — Bounded parallelism is per-orchestrator cooperative multiplexing, not operational concurrency control

- Severity: P3 Low.
- Evidence: D-091 creates no thread, async executor, provider scheduler, or
  session registry. ARCHITECTURE.md:330-352 states limits are
  per-AgentOrchestrator, deadlines are cooperative, and there is no app-global
  coordinator or hard preemption. The accepted advisory is recorded in
  docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md.
- Affected files and symbols:
  src-tauri/src/agent/orchestrator/bounded_parallel_workflow.rs,
  BoundedParallelWorkflowState and deadline/cancellation helpers; future
  application/provider service.
- Failure or abuse scenario: multiple orchestrators independently admit their
  maximum against one future provider; one blocking synchronous start delays all
  cooperative deadline/cancellation progress on the application thread.
- Impact: no global backpressure, fairness, provider-session isolation, or hard
  cancellation guarantee. Current same-thread fixture ordering remains safe.
- Recommended remediation: only in a separately approved provider/concurrency
  plan, add an application-owned capacity/session coordinator and nonblocking
  provider boundary while retaining exact task identity and stable synthesis
  order.
- Required regression test: multiple orchestrators sharing one fake
  coordinator, global cap/fairness, blocked-run isolation, cancellation/deadline
  progress, and no session/cross-workflow identity reuse.
- Fix now or defer: defer; mandatory before claiming provider-backed or
  app-global parallel execution.
- Remediation risk: High.

### F-12 — UI/native isolation invariants are review-only

- Severity: P3 Low.
- Evidence: only app-info-client imports Tauri invoke, only menu-route-client
  imports Tauri event listening, lib.rs registers only get_app_info, and Command
  Center has no Tauri/network/storage API. No repository-health or lint rule
  asserts those exact boundaries. test:agent-acceptance is Rust-only, and the
  acceptance documentation explicitly has no connected backend UI.
- Affected files and symbols: package.json validation scripts,
  scripts/repository_health.py, ESLint configuration, src/features/
  command-center, src-tauri/src/lib.rs, src-tauri/capabilities/default.json,
  src-tauri/tauri.conf.json.
- Failure or abuse scenario: a later change adds a Command Center invoke/fetch/
  storage call, new invoke handler, or expanded capability while the existing
  behavioral suites remain green.
- Impact: future trust-boundary regression can escape ordinary tests.
- Recommended remediation: add a narrow restricted-import and exact Tauri
  command/capability/config repository check before backend UI work.
- Required regression test: current-tree positive fixture and negative fixtures
  for prohibited Tauri import, network/storage API, extra invoke handler,
  expanded capability, and broadened production CSP.
- Fix now or defer: next test/governance-only increment, before agent UI/IPC.
- Remediation risk: Low.

### F-13 — Stateful backend and frontend hotspots create excessive review coupling

- Severity: P3 Low.
- Evidence: the Rust agent directory is 40,906 lines. orchestrator.rs is 7,243
  lines and bounded_parallel_workflow.rs is 5,188; several workflow modules are
  1,800-2,900 lines. The Command Center contains 5,938 production lines,
  including a 1,597-line fixture catalog, 929-line projection, 824-line graph
  adapter, 604-line page, and about 960 lines of CSS. D091-TD-01 already blocks
  another parallel family.
- Affected files and symbols: src-tauri/src/agent/orchestrator.rs,
  orchestrator/bounded_parallel_workflow.rs and other workflow state modules;
  src/features/command-center/commandCenterFixtures.ts,
  commandCenterProjection.ts, components/OperationalTopologyAdapter.tsx,
  CommandCenterPage.tsx, command-center.css.
- Failure or abuse scenario: a small lifecycle, cancellation, fixture, or layout
  change requires coordinated edits across large modules and misses a paired
  audit/cleanup/structured-view transition.
- Impact: regression risk, slow review, duplicated derivation, and
  disproportionate complexity for a private single-user prototype.
- Recommended remediation: before another workflow/lifecycle family, perform a
  behavior-preserving private decomposition by admission, start/quarantine,
  deadline/cancellation, terminal projection, and audit. Decompose Command
  Center only when its next functional change is authorized, separating
  declarative fixtures, selectors, viewport, and accessibility behavior. Do not
  introduce a generic workflow engine or event bus.
- Required regression test: preserve all Rust workflow contracts, exact event/
  audit order, 211 frontend tests, graph/structured parity, bundle budgets, and
  the rendered viewport/accessibility matrix.
- Fix now or defer: defer until immediately before the next workflow/concurrency
  or Command Center functional expansion.
- Remediation risk: Medium-High because state ordering and rendered behavior are
  sensitive.

### F-14 — Workflow Automation has no combined approval-to-manual-dispatch chain

- Severity: P3 Low.
- Evidence: D-093 at DECISIONS.md:3896 onward accepts separate
  checkpoint-denial and application-owned manual safe-dispatch demonstrations.
  Workflow validation rejects tool/checkpoint steps, while manual dispatch A-D
  directly selects existing sealed workflows. E2E-TD-01 records the missing
  combined chain.
- Affected files and symbols:
  src-tauri/src/agent/workflow_automation/validation.rs,
  orchestrator/workflow_automation_proposal.rs,
  orchestrator/workflow_automation_dispatch.rs; D-093 and demo evidence.
- Failure or abuse scenario: documentation or UI later implies that approving a
  checkpoint authorizes dispatch, although no such bridge exists; alternatively
  an implementer joins the branches without exact subject binding.
- Impact: claim/expectation mismatch now; a future unsafe bridge could reuse or
  over-broaden approval.
- Recommended remediation: keep the two branches separate and disclosed. Only
  if a real combined behavior becomes necessary, require a new owner decision
  and ExecPlan with exact step/tool/arguments/workflow binding and single use.
- Required regression test: current denial and manual-dispatch branches remain
  independent; any future bridge must prove reject/cancel/expire/replay/no-
  dispatch, exact subject equality, and no approval reuse.
- Fix now or defer: defer indefinitely unless the owner explicitly requires the
  combined behavior. Do not add a bridge to make a demo pass.
- Remediation risk: High because it would create new approval-to-control
  authority.

### F-15 — Current documentation has identifiable semantic drift

- Severity: P3 Low.
- Evidence: PRODUCT_REQUIREMENTS.md:139 and 169 both define FR-020, making the
  summary at 187 ambiguous. PRODUCT_REQUIREMENTS.md:478-481 still says Command
  Center rendered validation is pending, while DECISIONS.md:3885-3894 and
  PROJECT_STATUS.md:98-106 record completion. ARCHITECTURE.md:82 says app-info
  is narrowed despite F-08, while its Tauri section does not clearly disclose
  the production CSP's retained development WebSocket allowance from F-07.
- Affected files and symbols: PRODUCT_REQUIREMENTS.md FR-020 and current
  baseline; ARCHITECTURE.md React/Tauri sections; related requirement
  references.
- Failure or abuse scenario: a reviewer or traceability tool maps evidence to
  the wrong requirement, treats completed rendered validation as pending, or
  assumes a boundary validation/configuration that source does not provide.
- Impact: governance, readiness, and remediation confusion; no runtime behavior
  change.
- Recommended remediation: assign the gateway-demo requirement a unique stable
  ID and update every reference; mark the rendered matrix complete; make
  app-info/CSP wording match source until the production fixes land.
- Required regression test: documentation check for unique FR identifiers plus
  targeted semantic current-status assertions for the rendered validation and
  Tauri boundary.
- Fix now or defer: next documentation-only reconciliation; do not mix with
  production remediation.
- Remediation risk: Low-Medium because cross-references must be exhaustively
  searched.

### F-16 — Public licensing and third-party obligation review are not prepared

- Severity: P3 Low.
- Evidence: the repository has no root LICENSE file or documented third-party
  license/notice-obligation review. package.json private and
  src-tauri/Cargo.toml publish = false accurately describe the current
  non-registry-publication posture.
- Affected files and symbols: repository root publication artifacts,
  package.json, src-tauri/Cargo.toml, future release checklist.
- Failure or abuse scenario: source is published without clear reuse rights or
  a determination of which dependency/source/binary notice obligations apply.
- Impact: publication/legal ambiguity, not architectural or runtime failure.
- Recommended remediation: before public release, have the owner select a
  project license and document a third-party license/notice-obligation review.
  Add a NOTICE artifact only where the selected distribution model or
  dependency obligations require it.
- Required regression test: publication/release check requiring an approved
  license and documented obligation review, plus NOTICE evidence when
  applicable, against the dependency inventory.
- Fix now or defer: defer until publication is selected; mandatory before public
  source or binary release.
- Remediation risk: Low technically; owner/legal judgment is required.

## Ratings

| Dimension                   | Rating            | Rationale                                                                                                                             |
| --------------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Architecture                | Good              | Strong application-owned layers and closed ports; UI/provider/effect path remains unwired and PlatformAdapter absent                  |
| Role separation             | Good              | Authority separation is excellent; runtime invocation does not bind the stored instruction version                                    |
| Agent isolation             | Good              | Sealed definitions, profiles, contexts, and grants; legacy returned-run identity trust lowers the score                               |
| Task lifecycle              | Needs Improvement | Strong states/cancellation/tests, but legacy cleanup and broad deadline gaps are provider blockers                                    |
| Workflow safety             | Excellent         | Exact catalogs, validation, no recursion/retry/executable tool step, stable ordering, controlled partial failure                      |
| Governance                  | Good              | Closed non-executing registry/schema/profile/policy/approval chain; agent approval and lifecycle audit integration remains incomplete |
| Approval integrity          | Excellent         | Exact subject, manager/source/presentation, TTL, single use, replay denial, and NotAttempted execution                                |
| Memory isolation            | Excellent         | Root/task/agent ownership, sealed grants, proposal-only sharing, cleanup, disable/delete, hard bounds                                 |
| Document isolation          | Good              | Strong root/symlink/identity/content checks; accepted non-atomic Unix open residual                                                   |
| Auditability                | Needs Improvement | Typed/redacted attribution is strong but fragmented, volatile, and incomplete for memory/document lifecycle                           |
| Runtime reliability         | Needs Improvement | Returned identity, rejected-run handle, instruction binding, and liveness gaps block live provider use                                |
| Concurrency safety          | Good              | Excellent for the exact same-thread bounded fixture claim; not operational/app-global concurrency                                     |
| Maintainability             | Needs Improvement | Very large stateful modules and projection hotspots create review coupling                                                            |
| Testability                 | Excellent         | Deterministic mocks, public contracts, adversarial cases, acceptance suite, and 211 frontend tests                                    |
| UI integrity                | Good              | Honest fixture disclosure and no agent IPC; CSP/app-info/static-boundary gaps remain                                                  |
| Personal-project simplicity | Needs Improvement | Critical boundaries are justified, but workflow/UI breadth is disproportionate to an unwired owner-only prototype                     |
| Future publishability       | Good              | Ports and closed contracts are reusable; live hardening, audit, portability, documentation, and licensing remain                      |

## Testing and evidence assessment

### Coverage strengths

- Deterministic MockAgentRuntime modes cover success, unavailable/unhealthy,
  start/event/cancel failures, unexpected status, foreign/duplicate identity,
  capability contradiction, and lifecycle accounting.
- Registry tests cover exact nine identities, activation/profile/instruction
  mapping, ordering, duplicates, unknowns, and redaction.
- Lifecycle tests cover direct response, delegation, state transitions,
  sequencing, result identity, event sequence, cancellation, cancellation
  failure, partial failure, capacity, output bounds, and late events.
- Governance tests cover unknown tools, versions, schemas, profile denial,
  exact approval, replay, expiry, cancellation, audit capacity, and
  NotAttempted execution.
- Memory/document tests cover namespace/owner isolation, shared proposal review,
  version conflicts, cleanup, roots, traversal, symlinks, identity changes,
  format/size/UTF-8/control content, and selected-context transfer.
- Workflow tests cover provenance, strict JSON, duplicate keys, evidence
  references, authority claims, cycles, limits, cancellation, partial failure,
  stable ordering, cooperative deadlines, cleanup retry, and synthesis truth.
- UI tests cover projection validation, provenance, immutability, authority
  labels, graph/structured parity, state, interactions, focus, and accessibility
  contracts. The completed rendered matrix adds real browser and native Tauri
  evidence.
- Deterministic end-to-end acceptance covers the twelve required demonstration
  families and all nine definitions without presenting fixture output as live.

### Coverage gaps tied to findings

- No legacy workflow regression returns a foreign/duplicate run identity,
  F-01.
- No legacy start regression combines unexpected status and cancellation
  failure while asserting retained cleanup, F-02.
- No test proves instruction source/version reaches an invocation, F-03.
- Earlier workflows have no deadline contracts, F-04.
- Native cancellation cannot close the synchronous prompt, F-05.
- No connected controlled-denial contract exists, F-06.
- Production/dev CSP and app-info runtime parsing lack negative tests, F-07 and
  F-08.
- No unified audit correlation/durability test exists, F-09.
- Atomic no-follow document opening is not implemented, F-10.
- No app-global/provider concurrency test exists, F-11.
- UI/native trust-boundary invariants are not statically enforced, F-12.

### Commands and exact results

| Command or analysis                                                                                                                                               | Result                                                                                                                                                                                                                                                                              |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| npm run test:agent-acceptance                                                                                                                                     | Passed: 447 tests, comprising 249 Rust library and 198 public agent contract tests                                                                                                                                                                                                  |
| npm run verify                                                                                                                                                    | Passed: formatting, repository health, strict lint/Clippy, 28 hook tests, 38 repository tests, 211 frontend tests across 13 files, 481 Rust tests passed and 1 intentional opt-in Hermes real probe was ignored (482 discovered), frontend build, and Tauri release no-bundle build |
| cargo test --manifest-path src-tauri/Cargo.toml --locked --test agent_orchestration_contract --test agent_bounded_parallelism_contract                            | Passed: 63 tests, 0 failed                                                                                                                                                                                                                                                          |
| cargo test --manifest-path src-tauri/Cargo.toml --locked --test agent_definition_registry_contract --test agent_governance_contract --test agent_runtime_contract | Passed: 44 tests, 0 failed                                                                                                                                                                                                                                                          |
| cargo test --manifest-path src-tauri/Cargo.toml --locked --lib agent::orchestrator::bounded_parallel_workflow::tests                                              | Passed: 25 tests, 0 failed                                                                                                                                                                                                                                                          |
| npm run test:frontend                                                                                                                                             | Passed: 211 tests across 13 files                                                                                                                                                                                                                                                   |
| npm run typecheck                                                                                                                                                 | Passed                                                                                                                                                                                                                                                                              |
| npm run docs:check                                                                                                                                                | Passed                                                                                                                                                                                                                                                                              |
| npm run repository:check                                                                                                                                          | Passed                                                                                                                                                                                                                                                                              |
| npm run security:scan                                                                                                                                             | Passed                                                                                                                                                                                                                                                                              |
| npm ls --omit=dev --all --json                                                                                                                                    | Passed: production dependency tree resolved                                                                                                                                                                                                                                         |
| npm ls --all --json                                                                                                                                               | Passed: complete dependency tree resolved                                                                                                                                                                                                                                           |
| cargo tree --manifest-path src-tauri/Cargo.toml --locked -e normal                                                                                                | Passed: locked normal dependency tree resolved                                                                                                                                                                                                                                      |
| git diff --check                                                                                                                                                  | Passed                                                                                                                                                                                                                                                                              |
| npm audit --audit-level=low                                                                                                                                       | Passed: found 0 vulnerabilities                                                                                                                                                                                                                                                     |
| cargo-audit                                                                                                                                                       | Not run locally: binary absent; pinned CI accepted-baseline gate inspected                                                                                                                                                                                                          |
| Git history/tree analysis                                                                                                                                         | Passed: reviewed native runtime through PR 57 acceptance; HEAD and origin/main have the same tree despite squash-divergent commit IDs                                                                                                                                               |

## Git-history assessment

The reviewed sequence is coherent and incremental:

| Commit  | Increment                              |
| ------- | -------------------------------------- |
| 293aa04 | Native runtime boundary                |
| f42a6c7 | Nine-definition registry               |
| 1d1d9d6 | Task orchestration                     |
| 2687294 | Agent governance                       |
| 5e53f55 | Memory and approved documents          |
| 3efd2c1 | Research/Knowledge workflow            |
| a5d7ba1 | Engineering workflow                   |
| 3dccb81 | Cloud/Systems workflows                |
| 140f05b | Workflow-internal decomposition        |
| 7e2f29e | Workflow Automation proposals          |
| f11a04b | Bounded parallelism                    |
| fa66ce2 | Deterministic Command Center           |
| 527f0f4 | Rendered Command Center validation fix |
| 3987387 | Deterministic end-to-end acceptance    |

The multi-agent increments did not add a provider, executor, agent IPC,
privileged capability, credential path, network client, or Hermes dependency.
Several findings in this review predate the nine-agent increments: the native
approval dialog and CSP are existing repository advisories. The review keeps
them because the requested scope includes approval and Tauri integrity.

## Hermes status

- Hermes remains Deferred/Blocked.
- Raw TUI-gateway stdio, Hermes serve WebSocket, and ACP spike NO GO evidence is
  preserved in the ADRs, ExecPlans, and spike reports.
- No production source defines HermesAgentRuntime.
- No production dependency, process, network path, selector, fallback, or
  runtime ID depends on Hermes.
- The ignored real Hermes probe remains opt-in evidence only.
- src-tauri/src/agent/native_runtime.rs declares NativeAgentRuntime the only and
  default implementation; AgentOrchestrator::native constructs it directly at
  orchestrator.rs:6096-6099.

## Sequenced remediation backlog

This backlog is advisory only. It does not authorize implementation or change
NEXT_STEPS.md.

1. Runtime-start containment hardening: resolve F-01 and F-02 together with one
   universal exact-identity/quarantine helper. Gate: before any live/external
   runtime, provider, or agent IPC.
2. Small current-boundary hardening: resolve F-07 and F-08 in separately scoped
   Tauri/UI increments with rendered target validation. Gate: before live data
   or connected UI.
3. Trusted provider invocation contract: resolve F-03 and F-04 with a new
   decision/ExecPlan for instruction lanes and uniform monotonic leases. Gate:
   before a configured provider; do not add automatic retries.
4. Production approval lifecycle: resolve F-05 and make the owner decision
   required by F-06. Gate: before real actions or connected approval UI.
5. Audit/privacy and document hardening: design F-09 and F-10 as separate
   bounded increments. Gate: before durable/live memory, documents, providers
   with retained content, or real effects.
6. Regression-boundary automation: implement F-12 as a small test/governance
   increment before any agent UI/IPC work.
7. Maintainability checkpoint: address the applicable backend or frontend part
   of F-13 immediately before adding another workflow/concurrency family or
   changing Command Center behavior. Preserve all external contracts.
8. Operational concurrency: address F-11 only after a separately approved
   provider/session/capacity decision. Do not reinterpret D-091 as real
   concurrency.
9. Workflow approval bridge: keep F-14 deferred unless explicitly required.
   Never add a bridge merely to satisfy a demonstration.
10. Documentation reconciliation: resolve F-15 in a documentation-only run
    after the owner selects the next work item.
11. Publication preparation: resolve F-16 plus release/security/portability
    gates only when public distribution is selected. Do not add SaaS,
    multi-tenancy, billing, marketplace, or enterprise IAM scope.

## Documentation drift

Material semantic drift exists and is captured in F-15:

- duplicate FR-020 identifiers;
- stale pending status for completed Command Center rendered validation;
- an app-info narrowing claim not implemented at runtime;
- no clear architecture disclosure that the production CSP retains the
  development local-WebSocket source.

Otherwise, current project memory correctly discloses that the agent core is
unwired, fixture-only, no-I/O/no-execution; Command Center is simulated;
NativeAgentRuntime has no provider; audit/memory are volatile; bounded
parallelism is same-thread/cooperative; the Workflow Automation branches are
separate; and Hermes remains deferred.

## Review closeout

- Highest current risk: F-01 and F-02 at the runtime start/cleanup boundary,
  followed by F-03/F-04 for any live provider and F-07 at the UI boundary.
- Overall verdict: Good deterministic architecture with strong governance and
  excellent workflow safety, but not live-provider, connected-action, or
  public-release ready.
- Role boundaries: no current authority-confusion defect found.
- Governance: no current shell, Git, dependency, network, cloud, systems,
  credential, memory, workflow, or tool-execution bypass found.
- Workflow: exact sealed flows are safe; operational deadlines/concurrency and
  the intentionally absent approval bridge remain bounded gaps.
- Files created: docs/reviews/NATIVE_NINE_AGENT_ARCHITECTURE_REVIEW.md only.
- Remediation implemented: none.
- Production implementation changed: no.
