# Engineering quality workflow

Status: Verified complete with advisories
Owner: Project owner
Decision: D-087
Gate: `agent-engineering-quality-workflow`
Last updated: 2026-08-12
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Implement one sealed, deterministic, fixture-only engineering-review workflow
above `AgentOrchestrator` and the sole/default `NativeAgentRuntime`:

```text
Personal Assistant
  -> Coding Agent
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> Personal Assistant synthesis
  -> approval required before any consequential change
```

Every specialist is a separate depth-one sibling created and sequenced by the
orchestrator. The implementation activates Coding, QA & Validation, and
Security & Risk only for this exact proposal-only workflow. It proves bounded
analysis and review contracts; it does not inspect or mutate the live
repository, execute a command, or create an approval request.

## User-visible outcome

None. This is an unwired Rust application-service contract exercised through
deterministic fixtures and `MockAgentRuntime`. It adds no Tauri command, React
consumer, live provider, filesystem access, shell, Git operation, or device
effect.

## Scope

- Add a versioned, strict `ChangeProposal` for fixture-based code analysis and
  patch proposals.
- Add a versioned, strict `ValidationReport` for acceptance-criteria coverage,
  proposed validation, gaps, and regressions.
- Add a versioned, strict advisory `RiskAssessment` for evidence-bound security
  and change-risk findings.
- Add a strict final Personal Assistant synthesis that preserves validated
  proposal, QA, security, fixture, and partial-failure attribution.
- Add one sealed orchestrator path with exact stage, task, run, event,
  cancellation, audit, and partial-failure behavior.
- Change the three agent definitions from `Deferred` to `Initial` only after
  their exact workflow contracts and tests pass. `Initial` remains
  non-authorizing eligibility, not live or autonomous capability.
- Accurately advertise only analysis of application-supplied fixture content,
  architecture explanation, diff review, implementation planning, patch
  proposal, validation planning, and advisory risk review.
- Preserve the existing empty specialist tool allowlists and
  `AgentExecutionDisposition::NotAttempted` behavior.

## Explicit non-goals

- No `ToolRegistry`, tool-schema, `PolicyEngine`, `ApprovalManager`, executor,
  `AuditLogger`, platform-adapter, or permission expansion.
- No live repository discovery, filesystem read/write, code-search process,
  test execution, formatter execution, shell, package manager, dependency
  installation, credential access, network access, or Git command.
- No approved scoped write in this increment. The requested write case is not
  implemented because no restricted repository executor exists.
- No file deletion, branch deletion, commit, push, destructive command,
  release, publication, or deployment capability.
- No model/runtime-to-device or WebView-to-device path and no generic
  `execute_action` or workflow engine.
- No specialist spawning, recursion, parallelism, automatic retry, background
  work, durable task, durable audit, or new memory access.
- No Codex, Hermes, OpenClaw, external agent framework, provider, or live model
  integration.
- No activation of Cloud Infrastructure, Systems Operations, or Workflow
  Automation.

## Existing behavior and constraints

- The published D-086 baseline is clean `main` at `3efd2c1`.
- `AgentOrchestrator` is Rust-only and unwired. It alone creates tasks and
  invokes the one-run `AgentRuntime` boundary.
- `NativeAgentRuntime` is sole/default. Runtime tool proposals fail closed.
- The generic route remains exactly Personal Assistant to Research. D-085 and
  D-086 use separate sealed application-service paths rather than widening the
  delegation matrix.
- `ToolRegistry` contains only `get_current_datetime` and
  `create_local_task`; neither is an engineering or repository tool.
- Only the Personal Assistant policy profile is eligible for those two tools.
  Coding, QA, and Security have exact existing profiles with empty tool
  allowlists.
- Coding, QA, and Security are presently `Deferred`; their embedded V1
  instructions accurately describe advisory non-authority but also name the
  deferred state.
- All six specialist roles outside Research and Knowledge use
  `MemoryDisabledV1`; this increment does not change that mapping.
- Existing task output is bounded to 8,192 Unicode scalar values and 16,384
  UTF-8 bytes. `RuntimeTurnRequest` accepts selected text up to the 65,536-byte
  gateway-request ceiling, but `NativeAgentRuntime` then JSON-encodes it inside
  `InitialGatewayRequest`; therefore raw selected-text length alone is not a
  sufficient Native start bound.
- Approval is currently exact and trusted, but the agent governance path has no
  executor. Approval outcomes remain `NotAttempted` and cannot be treated as
  dispatch authority.

## Current-state evidence

- Before planning edits, `git status --short --branch` showed clean synchronized
  `main...origin/main`. The current workspace is intentionally planning-dirty
  with only this D-087 decision/plan and current-state reconciliation; source,
  tests, manifests, lockfiles, and gate state remain unchanged.
- `git log`: `3efd2c1 feat(agent): add fixture research knowledge workflow` is
  both `HEAD` and `origin/main`.
- `python3 .codex/hooks/post_increment_gate.py status`: the published D-086
  marker was `complete`, `valid: true`, and `PASS WITH ADVISORIES` before this
  later planning change.
- `src-tauri/src/tools/schema.rs` defines exactly two non-engineering schemas.
- `src-tauri/src/agent/governance.rs` gives every specialist an empty current
  tool allowlist and always records execution as `NotAttempted`.
- `src-tauri/src/agent/definition.rs` keeps Coding, QA, and Security deferred
  behind separate engineering gates.
- The first readiness review returned `Blocked` because the former draft did
  not define exact repository, tool, approval, result, limit, failure, file,
  test, or rollback contracts. D-087 resolves that blocker by selecting the
  smaller fixture-only, proposal-only boundary and explicitly deferring all
  repository tools and effects.

## Files expected to change

Production Rust:

- `src-tauri/src/agent/engineering_quality.rs` (new)
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/mod.rs`

Tests:

- `src-tauri/tests/agent_engineering_quality_workflow_contract.rs` (new)
- `src-tauri/tests/agent_definition_registry_contract.rs`
- `src-tauri/tests/agent_governance_contract.rs`
- `src-tauri/tests/agent_orchestration_contract.rs`
- `src-tauri/tests/support/mock_agent_runtime.rs`, only if another deterministic
  failure fixture is required

Current-state and closeout documentation:

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
- `docs/increments/agent-engineering-quality-workflow.md` (new at closeout)
- `docs/reviews/2026-08-12-agent-engineering-quality-workflow-post-increment-review.md`
  (new at closeout)
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- this plan

No manifest, lockfile, Tauri configuration, capability, frontend, tool,
policy, approval, executor, document, memory, storage, or runtime file is in
scope. An unexpected need to change one stops implementation for owner review.

## Affected components

- Agent catalog activation metadata and three embedded instruction versions.
- One new framework-neutral engineering result-contract module.
- One new sealed `AgentOrchestrator` workflow state.
- Deterministic mock-runtime and public contract tests.
- Architecture, product, security, roadmap, and current-state documentation.

## Interfaces and invariants

### Workflow selection and lifecycle

- A sealed application-only `start_engineering_quality_workflow` selector
  selects exactly `engineering-quality-v1` from a live Personal Assistant root
  before root output starts. It derives workflow, task, root, parent, agent,
  policy profile, memory profile, runtime, run, request, depth, and predecessor
  identity only from the registry and exact live orchestrator state. No identity
  field is accepted from runtime/model JSON, fixtures, the WebView, or a caller.
  Live run/request identity stays private and is redacted on every Debug/error/
  event/audit surface.
- The path is not generic delegation, a runtime control event, a host tool, or
  a model-selected workflow. Selection checks two-way mutual exclusion before
  mutation: generic delegation, the D-085 document path, D-086, or any prior
  engineering selection blocks engineering; after engineering selection, each
  of those selectors and a repeated engineering selection fails closed.
- The workflow owns exactly four tasks: one Personal root and three sequential
  specialist children.
- All children have depth one, the same root, and exact predecessor
  attribution. No specialist creates or delegates a task.
- The finite limits are three non-replenishing children, one active child, five
  sequential runtime-run attempts (initial Personal, Coding, QA, Security,
  final Personal synthesis), at most eight accepted runtime events for each of
  the four event-consuming Coding/QA/Security/synthesis runs, 32 total accepted
  runtime events, 32 generic
  orchestration events, 16 workflow events, 16 matching descriptive workflow
  audit records, and zero automatic retries.
- The deterministic one-delta success fixture consumes exactly 4 tasks, 3
  children, 5 run attempts, 12 accepted runtime events (started, one delta, and
  terminal for each event-consuming run), 16 generic orchestration events (the
  existing two root events; create/start and terminal/result-returned for each
  child; parent resumed; root completed), 8 workflow events (three stage
  start/complete pairs plus synthesis start and completed), and 8 audit records.
  An event-consuming run rejects a seventh delta or any nonterminal event that
  would consume its reserved eighth terminal slot. Thus four per-run caps prove
  the 32 global maximum. Tests enumerate every defined success, partial, start-
  failure, and cancellation transition and prove generic/workflow/audit worst-
  case counts remain within 32/16/16 before mutation.
- Before each terminal event or workflow start mutates state, preflight proves
  all required task/run/event/audit capacity, parses and wraps validated output,
  constructs and bounds the exact successor/fallback selected text, derives
  attribution, and constructs `RuntimeTurnRequest`. Failure before terminal
  runtime acceptance causes zero workflow, task, run, event, audit, or budget
  mutation. `runtime.start` occurs only after the relevant accepted terminal or
  initial-root cancellation and may still return the typed stage-start failure
  defined below; no plan claim treats that post-acceptance failure as zero
  mutation.
- A continuation-start failure cannot reverse an accepted terminal event or
  replenish any task, child, run, event, or retry budget.

### Fixture repository

- Input is one immutable application-owned catalog with one to eight synthetic
  text files. It is not a repository root, file handle, path grant, tool, or
  filesystem authority.
- Each file has one canonical opaque ID of 1-64 ASCII bytes/scalars from
  letters, digits, `-`, or `_`; a display-only canonical relative path label of
  1-256 Unicode scalars
  and 1-1,024 UTF-8 bytes; and nonempty UTF-8 fixture content subject to the
  exact per-value and aggregate limits below.
- Labels reject absolute paths, empty/`.`/`..` components, backslashes,
  schemes, NUL/control characters, duplicate canonical values, and more than
  eight components. Labels never become an OS path.
- Each content value is at most 2,048 scalars/4,096 bytes; aggregate fixture
  content is at most 6,144 bytes; the serialized catalog is at most 8,192 bytes.
- The objective is 1-1,024 Unicode scalars and 1-2,048 bytes. There are one to
  eight acceptance criteria. Each application-issued criterion ID is 1-64 ASCII
  letters, digits, `-`, or `_`; each criterion text is 1-512 scalars and
  1-1,024 bytes, with a 2,048-byte aggregate text limit and a 4,096-byte
  serialized criteria-catalog limit. IDs and labels are unique after exact
  canonical validation; nothing is silently normalized.
- A bounded application-owned `ApplicationValidationEvidenceCatalog` contains
  zero to eight evidence entries and serializes to at most 4,096 bytes. Each application-issued evidence ID uses
  the same 1-64 ASCII-byte/scalar grammar. Its kind is exactly
  `FixtureObservation`, `DependencyObservation`, or `ProposedCheck`; its status is exactly
  `ObservedFixture` or `NotRun`; its description is 1-512 scalars and 1-2,048
  bytes with a 3,072-byte aggregate description limit; and it binds one to
  eight unique known criterion IDs. `FixtureObservation` requires
  `ObservedFixture` and one to eight known fixture-file IDs.
  `DependencyObservation` has the same status/reference rules and means only
  that the synthetic catalog contains dependency-related evidence; it is not a
  live advisory scan. `ProposedCheck`
  requires `NotRun` and zero to eight known fixture-file IDs. Other kind/status
  pairings are invalid. `ObservedFixture` means only a deterministic fact
  present in application-supplied fixtures. `NotRun` means no command or test
  was executed. The catalog cannot express `passed`, `failed`, or live external
  evidence. QA may reference an evidence entry for a criterion only when that
  entry's application-bound criterion set contains the exact criterion ID.
- Every stage receives only the validated fixture IDs/content and bounded
  predecessor transfer projection needed for that stage. The application caps
  `ChangeProposalTransferV1` at 6,144 bytes,
  `ValidationReportTransferV1` at 4,096 bytes, and
  `RiskAssessmentTransferV1` at 4,096 bytes. Coding receives at most the
  objective plus the 8,192-byte fixture catalog, 4,096-byte criteria catalog, the
  4,096-byte evidence catalog, and bounded framing. QA receives only the objective, criteria/evidence catalogs, Coding
  transfer, and framing. Security receives only the evidence catalog, Coding
  transfer, QA transfer or typed unavailable status, and framing. Synthesis
  receives only the three transfers or typed unavailable statuses plus
  framing. Every constructed input is measured and rejected above 24,576
  bytes; aggregate and whole-result caps always override otherwise-valid field
  maxima.
- Every selected-text input is capped at exactly 24,576 raw UTF-8 bytes and
  rejects controls other than newline and tab. JSON encoding therefore expands
  selected text by at most 2x for quote, backslash, newline, and tab, while
  ordinary UTF-8 remains unescaped. The plan reserves 16,384 bytes for maximum
  run/request IDs and fixed gateway wire framing: `2 * 24,576 + 16,384 =
65,536`. The private `InitialGatewayRequest` serializer remains unchanged and
  is not called by generic orchestrator preflight. Native regression tests call
  `NativeAgentRuntime::start` with maximum IDs plus quote, backslash, newline,
  tab, Unicode, and maximum-framing adversaries to prove every accepted
  engineering input fits the existing 65,536-byte encoded request. If
  `runtime.start` nonetheless rejects serialization, the applicable typed
  stage-start path handles it after the preceding accepted transition.

### Parser and application binding

- Each model-emitted result is exactly one JSON object. Leading/trailing
  whitespace, prose/fences, trailing bytes, and a second value are rejected.
  “Canonical JSON” ordering or byte-for-byte reserialization is not required.
- Every raw result and nested raw struct uses `serde(deny_unknown_fields)` and
  is deserialized directly with `serde_json::Deserializer`; no intermediate
  `Value`, `flatten`, or untagged map may collapse keys. Wire enums are scalar
  strings rather than object variants. Serde's direct struct visitor therefore
  rejects repeated declared fields; tests cover duplicate top-level/nested
  keys and duplicate array references.
- Model JSON contains no trusted workflow/task/root/parent/run/request/profile/
  predecessor identity or authoritative version. After parsing content, the
  application wraps it in a validated result carrying private application-
  derived live identity, the closed contract version, and exact predecessor
  binding. Debug and errors redact that identity.

### Coding result

`ChangeProposalV1` follows the exact-object parser and application-binding
rules above. Unknown fields, duplicate keys/references, outer/trailing content,
URLs, reasoning, and unsupported shapes are rejected. Raw output and its
successor transfer are capped at 4,096 Unicode scalars and 6,144 bytes and
contain:

- an objective summary of 1-1,024 scalars and 1-4,096 bytes;
- one to eight findings, each with a finding ID using the 1-64 ASCII opaque-ID
  grammar, a statement of 1-512 scalars/1-1,024 bytes, and one to eight unique
  tagged `EngineeringEvidenceRef` values;
- zero to eight affected fixture file IDs;
- zero to eight proposed patch operations, each referencing a known file ID and
  containing proposal text of 1-512 scalars/1-2,048 bytes rather than a
  filesystem mutation;
- zero to eight risks, each 1-512 scalars/1-1,024 bytes;
- one to eight proposed validation steps, each with a 1-64 ASCII ID and text of
  1-512 scalars/1-2,048 bytes;
- one rollback proposal of 1-512 scalars/1-2,048 bytes;
- one to eight unique `capability_requests` using the closed
  `EngineeringCapability` enum; and
- `fixture_based: true`, `proposal_only: true`, and no execution claim.

The application-owned closed `EngineeringCapability` enum contains exactly
`FixtureInspect`, `FixtureTextSearch`, `ArchitectureExplain`, `DiffReview`,
`ImplementationPlan`, `PatchProposal`, `ValidationPlan`, `FormattingPlan`,
`FileWrite`, `FileDelete`, `OutsideFixturePath`, `DependencyInstall`,
`PackageManagerExecute`, `TestExecute`, `FormatterExecute`, `GitCommit`,
`GitPush`, `BranchDelete`, `DestructiveShell`, `CredentialAccess`, and
`NetworkAccess`. The application alone maps each enum to closed
`ProposalOnly` or `Denied` disposition: the first eight are `ProposalOnly` and
all others are `Denied`. Model text cannot define or override the mapping.
Capability references use the enum, not free-form names. Any `Denied` request
is retained only as the aggregate `ContainsDeniedCapability` disposition on
the Coding result and its one content-free completion audit record,
causes the Coding result to be valid only as `partial`, starts no action or
approval, and forces final synthesis `partial`; an unknown capability or an
execution-success claim invalidates the Coding result and skips QA/Security.
A patch operation requires `PatchProposal`; it never implies `FileWrite`.

`EngineeringEvidenceRef` is a tagged namespace plus opaque ID. Its namespaces
are exactly `FixtureFile`, `ApplicationEvidence`, `ProposalFinding`,
`ProposalValidationStep`, `QaFinding`, and `QaCheck`. Coding may reference only
the first two; QA may additionally reference the proposal namespaces; Security
may reference all six. Each reference resolves against its exact immutable
catalog or predecessor collection, so equal strings in separate namespaces
cannot alias.

### QA result

`ValidationReportV1` follows the exact-object parser and application binding,
and is capped at 3,072 scalars and 4,096 bytes. The application binds it to the
exact validated proposal and it contains:

- coverage for each supplied acceptance-criterion ID exactly once, with a
  closed `demonstrated` or `not_demonstrated` disposition and one to eight
  unique tagged `EngineeringEvidenceRef` values from the namespaces available
  to QA;
- zero to eight correctness or regression findings, each with a unique
  1-64-byte ASCII opaque ID, one to eight exact tagged evidence references,
  and text of 1-512 scalars/1-1,024 bytes;
- zero to eight proposed safe tests/checks, each with a 1-64 ASCII ID, text of
  1-512 scalars/1-1,024 bytes, zero to eight exact tagged evidence references,
  and exact `not_run` status;
- zero to eight validation gaps of 1-512 scalars/1-1,024 bytes;
- a closed `adequate`, `incomplete`, or `blocked` conclusion; and
- `advisory_only: true`, `approval_authority: false`, and
  `evidence_executed: false`.

Missing, duplicate, or unknown criterion coverage invalidates the report. A
`demonstrated` criterion must reference at least one `ObservedFixture` entry;
`NotRun` cannot demonstrate anything. Any `not_demonstrated` criterion forces
the conclusion to `incomplete` or `blocked` and final synthesis to `partial`.
An `adequate` conclusion is valid only when every criterion is demonstrated.
Unknown evidence, fabricated pass/fail results, approval fields, source
modifications, test suppression, reasoning, URLs, and execution claims fail
closed. QA has no tool eligibility and cannot approve.

### Security result

`RiskAssessmentV1` follows the exact-object parser and application binding, and
is capped at 3,072 scalars and 4,096 bytes. The application binds it to the
exact validated proposal and QA outcome and it contains:

- zero to eight findings in the closed categories authorization boundary,
  input validation, secrets/privacy, dependency evidence, audit, rollback, and
  general change risk;
- closed `informational`, `low`, `medium`, or `high` severity and `low`,
  `medium`, or `high` confidence;
- a finding ID using the 1-64 ASCII grammar and a statement of 1-512 scalars/
  1-1,024 bytes;
- a closed finding basis: `EvidenceBound` requires one to eight unique tagged
  `EngineeringEvidenceRef` values that resolve against the exact fixture,
  application-evidence, proposal, or QA predecessor namespace; `Hypothesis`
  requires zero evidence references, cannot use `high` confidence, and cannot
  state certainty;
- zero to eight unresolved risks and zero to eight recommended follow-ups, each
  1-512 scalars/1-1,024 bytes;
- an application-derived closed dependency-evidence status. It is
  `Available { evidence_refs }` only when at least one exact
  `DependencyObservation` with `ObservedFixture` exists; otherwise it is
  `Unavailable`. Model JSON cannot supply or override this status; and
- `advisory_only: true`, `authorization_granted: false`, and
  `remediation_executed: false`.

Security may identify advisory findings but cannot authorize, set trusted
policy/risk/permission metadata, access secrets, remediate, or claim
vulnerability certainty without evidence. Secret-like fixture values and all
free text are absent from Debug, errors, events, and audit.

### Personal synthesis

`EngineeringReviewSynthesisV1` follows the exact-object parser and application
binding and is capped at 2,048 scalars and 4,096 bytes. It contains a summary
of 1-1,024 scalars/1-2,048 bytes, application-validated proposal/QA/Security
dispositions (not model-emitted identity), zero to eight unresolved issues of
1-512 scalars/1-1,024 bytes, the exact fixture file-ID set used by the proposal,
and a closed `complete` or `partial` status. Validation requires:

- `fixture_based: true`;
- `proposal_only: true`;
- `changes_applied: false`;
- `tests_executed: false`;
- a closed approval requirement derived by the application: `NotApplicable`
  when the synthesis contains only analysis and no mutation proposal, or
  `RequiredBeforeMutation` whenever one or more patch operations exist; and
- status consistent with all validated or unavailable stages.

The model does not supply the approval requirement. No approval request is
created because no executable action exists to authorize. The derived value is
a deterministic final-result invariant and does not mean QA, Security, the
model, or the orchestrator approved a change.

All free-text fields remain untrusted descriptive data and never grant
authority. For a narrow contradiction guard, validators map ASCII `A-Z` to
lowercase, retain ASCII letters/digits, replace every maximal run of all other
Unicode scalar values with one ASCII space, trim, and collapse spaces. They
reject these exact normalized token phrases in every free-text field:
`changes applied`, `patch applied`, `files updated`, `test passed`, `tests
passed`, `test failed`, `tests failed`, `test executed`, `tests executed`,
`dependency installed`, `git committed`, `git pushed`, `approval granted`,
`authorized to execute`, `remediation completed`, `confirmed vulnerability`,
`definitely vulnerable`, and `proven exploitable`. Tests cover mixed case,
ASCII punctuation, and non-ASCII separators. This is not semantic proof:
trusted booleans/enums, closed stage outcomes, absence of an executor, and
application-derived synthesis wording remain authoritative.

### Failure, cancellation, and redaction

Stage outcomes and start failures are exact:

| Point                                                                                         | Result and successor                                                                                                             | Workflow event/audit                                                              |
| --------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| Engineering selector preflight fails                                                          | Return typed error; root/run remain live; no workflow selected and no mutation                                                   | None                                                                              |
| Initial root cancellation fails before Coding                                                 | Return runtime-cancellation error; root remains live/retryable; no child, run, or workflow selected                              | Existing cancellation evidence only; no engineering event                         |
| Coding start fails after initial root run is terminally consumed                              | Do not create a live child; consume the one Coding attempt; mark Coding unavailable, skip QA/Security, and try Personal fallback | `PartialFailure(CodingStartFailed)` plus one matching audit record                |
| Coding terminal `ResponseFailed` (including runtime-reported `Cancelled`) or invalid result   | Preserve typed Coding status only; skip QA/Security; try Personal fallback                                                       | `PartialFailure(CodingFailed / CodingCancelled / CodingInvalid)` plus audit       |
| QA start fails after accepted Coding terminal                                                 | Keep validated Coding proposal; consume QA attempt; skip to Security with `QaUnavailable`                                        | `PartialFailure(QaStartFailed)` plus audit, then Security start if possible       |
| QA terminal `ResponseFailed` (including runtime-reported `Cancelled`) or invalid result       | Keep validated Coding proposal; mark QA unavailable; continue to Security                                                        | `PartialFailure(QaFailed / QaCancelled / QaInvalid)` plus audit                   |
| Valid QA is `incomplete`/`blocked`                                                            | Keep report; continue to Security; force final partial                                                                           | `QaCompleted` with closed quality plus audit                                      |
| Security start fails after accepted predecessor                                               | Keep validated Coding/QA; consume Security attempt; try partial Personal synthesis                                               | `PartialFailure(SecurityStartFailed)` plus audit                                  |
| Security terminal `ResponseFailed` (including runtime-reported `Cancelled`) or invalid result | Keep validated Coding/QA; mark Security unavailable; try partial synthesis                                                       | `PartialFailure(SecurityFailed / SecurityCancelled / SecurityInvalid)` plus audit |
| Personal synthesis start fails                                                                | Consume synthesis attempt; terminally fail root; no completed result                                                             | `PartialFailure(SynthesisStartFailed)` plus audit                                 |
| Personal synthesis terminal failure/cancel/invalid result                                     | Terminally fail/cancel root as applicable; no completed result                                                                   | `PartialFailure(SynthesisFailed / SynthesisInvalid)` or `Cancelled`, plus audit   |

Cancellation semantics are also closed:

- An explicit application cancellation of the Coding child cancels its exact
  run and task, records `Cancelled(Coding)`, skips QA/Security, and starts a
  truthful partial Personal fallback. Cancelling QA records
  `Cancelled(QaValidation)` and continues to Security with typed QA-unavailable
  status. Cancelling Security records `Cancelled(SecurityReview)` and starts
  partial synthesis. Each successful child cancellation returns one attributed
  cancelled outcome before its permitted successor starts.
- Explicit root cancellation while a specialist is active cancels that exact
  child run/task first and then the root run/task, records the child-stage and
  root cancellation in that order, and starts no successor or synthesis.
- Cancelling during synthesis cancels only the root synthesis run/root and
  records `Cancelled(Synthesis)`.
- Cancelling an already terminal workflow returns the existing terminal/no-op
  outcome without another event.
- A specialist or synthesis runtime-cancel failure returns the typed runtime
  error, keeps that run/task/workflow live and retryable, emits no false
  `Cancelled` event/audit, and starts no later stage.
- Late, duplicate, foreign, cross-stage, wrong-run, wrong-request, and wrong-
  sequence events fail before state or capacity mutation.
- Root cancellation resolves any pre-existing governance approval before child
  or root mutation and uses existing task-temporary cleanup rules; this workflow
  itself never creates an approval.
- Workflow events are exactly Coding started/completed, QA started/completed,
  Security started/completed, synthesis started, partial failure, cancelled,
  and completed.
- Descriptive audit binds stage, agent, task/root/parent, runtime/run,
  predecessor, capability disposition, and typed outcome from private live
  application state. Public views expose only closed/redacted attribution and
  cannot reconstruct a live context. Audit contains no fixture content, patch text,
  findings, secret-like text, prompt, output, reasoning, or OS path and grants
  no authority.
- Debug and errors expose only closed categories, sizes, and redacted identity.

Every stage-start attempt is represented before the runtime call by its bounded
task/request, consumed child/run budget, content-free created/started evidence,
and closed stage state. A failed runtime start terminalizes that attempted task
(or the root for synthesis), removes any unusable run, records the table's
partial failure, and follows only the stated successor. It never replenishes an
attempt. A failure to start a continuation after an accepted predecessor
terminal event does not turn that accepted event into `Err`; the event API
returns its normal accepted outcome and exposes one closed
`EngineeringContinuationFailure` snapshot. The variants are exactly
`CodingStartFailed`, `CodingAndSynthesisStartFailed`, `QaStartFailed`,
`QaAndSynthesisStartFailed`, `QaAndSecurityStartFailed`, `QaSecurityAndSynthesisStartFailed`,
`SecurityStartFailed`, `SecurityAndSynthesisStartFailed`, and
`SynthesisStartFailed`. Tests assert the exact terminal/nonterminal tasks,
active-run absence or presence, consumed task/run counts, generic events,
workflow events, and audit after every variant.

### Activation and unchanged governance

- Coding, QA, and Security move to `Initial` only with new exact V2 embedded
  instruction sources that name fixture/proposal-only behavior and preserve
  every non-authority rule.
- Their policy-profile and `MemoryDisabledV1` mappings remain unchanged.
- Their current tool allowlists remain empty. `ToolRegistry` remains unchanged.
- Generic delegation remains Personal Assistant to Research only. This workflow
  uses a sealed orchestrator path, creates no generic edges, and is mutually
  exclusive with generic delegation, D-085 document work, and D-086 in both
  selector directions.
- Runtime tool proposals remain rejected. Every governance execution
  disposition remains `NotAttempted`.
- Native remains sole/default and unchanged.

## Implementation milestones

- [x] Milestone 0 — run the readiness, architecture, and security review; begin
      `agent-engineering-quality-workflow` before source edits.
- [x] Milestone 1 — add strict fixture, capability-disposition, Coding, QA,
      Security, synthesis, failure, event, and descriptive-audit contracts.
- [x] Milestone 2 — add the sealed four-task/five-run orchestrator sequence,
      exact transition preparation, cancellation, and partial outcomes.
- [x] Milestone 3 — update the three versioned instructions and activation
      metadata without changing profiles, memory, tools, or generic routes.
- [x] Milestone 4 — add focused unit/public contracts and run affected
      regression tests.
- [ ] Milestone 5 — complete security-sensitive source validation and
      independent architecture/security/code/technical-debt review passed;
      documentation sync is drafted. Final documentation/repository/security/
      diff and session-end validation plus post-increment finalization remain.

## Security and privacy considerations

- Fixture input and all runtime output are untrusted. Application validators
  own bounds, identity, provenance, stage progression, denial classification,
  and final status.
- Fixture paths are display labels and file IDs are references, never path
  capabilities. Path escape is rejected at the contract boundary; this does
  not claim OS sandbox or filesystem containment because no filesystem is used.
- Proposed patches remain data. No policy or approval outcome can reach a
  write, command, Git, network, credential, package, or process boundary.
- QA and Security are intentionally advisory. Strict results reject any claim
  that either approved, authorized, executed, remediated, or proved external
  evidence.
- Synthetic secret-pattern fixtures must be visibly fake. Raw fixture content
  and free-form specialist output remain absent from Debug, errors, events,
  audit, documentation, and logs.
- No new dependency or unsafe code is permitted.

## Test plan

Focused unit and public contract tests must cover:

1. Deterministic fixture-only success with exact Coding -> QA -> Security ->
   Personal ordering, sibling lineage, run/task/event limits, and Native
   runtime attribution.
2. Read-only fixture code analysis, architecture explanation, diff review, and
   structured patch proposal without a live repository or mutation.
3. Strict contract rejection for unknown fields on every nested struct,
   duplicate top-level/nested keys and references, malformed/multiple/outer/
   trailing JSON, reasoning, URLs, caller/model identity, unknown file/evidence/
   criterion IDs, execution claims, and N/N+1 byte/scalar/list bounds including
   Unicode. Max-combination tests prove every valid predecessor projection fits
   the next stage, while aggregate N+1 fails before mutation. Contradictory
   overclaim phrases are tested across every free-text field with mixed case,
   ASCII punctuation, and Unicode-adjacent input.
4. Typed denial of file write/delete, outside-root/path-like target,
   dependency installation, package-manager execution, test/format execution,
   commit, push, branch deletion, destructive shell, credential access, and
   network access. Classifier tests prove these remain inert proposal data.
   Separately, from each exact live Coding, QA, and Security context, submit both
   registered schemas and representative unregistered names (`write_file`,
   `delete_file`, `install_dependency`, `run_package_manager`, `run_tests`,
   `run_formatter`, `git_commit`, `git_push`, `delete_branch`, `run_shell`,
   `read_credential`, and `network_request`) through the real governance entry:
   registered tools return a successful final `Deny` with policy reason
   `ProfileNotEligible` and a `PolicyEvaluated` audit lifecycle; unregistered
   tools return `UnknownTool` with `ValidationRejected`; execution is
   `NotAttempted`, and no pending approval or effect exists in either case.
5. Application evidence accepts only `ObservedFixture` and `NotRun`, preserves
   criterion/evidence provenance, and cannot express a real test pass. QA
   rejects missing/duplicate/unknown criterion coverage; requires every
   criterion exactly once as `demonstrated`/`not_demonstrated`; permits
   demonstrated only with observed-fixture evidence; forces incomplete for any
   not-demonstrated criterion; marks proposed tests `not_run`; cannot approve,
   fabricate evidence, modify source, or suppress a failing test.
6. Security emits an evidence-bound advisory finding, validates every tagged
   namespace against the exact predecessor, derives dependency evidence as
   `Available` or `Unavailable`, keeps hypotheses evidence-free/non-certain,
   cannot authorize or remediate, and cannot expose a fake secret sentinel
   through Debug/error/event/audit surfaces.
7. Coding failure skips QA/Security; QA failure still permits bounded Security
   review of the proposal; Security failure permits partial synthesis; QA
   incomplete forces partial synthesis; final synthesis failure fails the root.
8. Every stage/start-failure table row and closed continuation-failure snapshot;
   Coding/QA/Security independent child cancellation with its exact allowed
   successor; root child-first cancellation with no successor; synthesis
   cancellation; cancellation failure; terminal cancellation; late/duplicate/
   foreign/wrong-stage/wrong-run/wrong-request/wrong-sequence event rejection;
   zero retries; and no budget replenishment.
9. Two-way mutual exclusion among generic delegation, document, D-086, and
   engineering selectors. Preflight capacity/input/start failure causes zero
   unintended workflow mutation; continuation-start failure preserves
   terminal-event atomicity.
10. Content-free event ordering, private application-derived/redacted identity,
    capability disposition, and audit attribution for every success, denial,
    partial, failure, and cancellation path; exact enumerated worst cases remain
    within 4 tasks/3 children/5 runs/32 runtime/32 generic/16 workflow/16 audit.
11. Catalog activation is exact for Personal, Research, Knowledge, Coding, QA,
    and Security; Cloud, Systems Operations, and Workflow Automation remain
    deferred.
12. Specialist policy profiles continue to deny both existing tools; protected
    tool/schema source files remain unchanged; known/unknown governance behavior
    proves no engineering schema was registered; approvals do not dispatch;
    runtime tool proposals fail closed; and execution remains `NotAttempted`.
13. Existing definition, orchestration, governance, memory/document,
    Research/Knowledge, runtime, gateway, policy, approval, audit, storage, and
    frontend regressions remain green.
14. Each accepted stage input is at most 24,576 raw bytes. Direct
    `NativeAgentRuntime::start` regressions use maximum IDs and adversarial
    quotes, backslashes, newline/tab, Unicode, and maximum framing to prove the
    conservative bound serializes within 65,536 bytes. Engineering input at
    24,577 bytes fails construction before workflow mutation. Any unexpected
    Native serialization rejection within the accepted bound follows the exact
    stage-start-failure table after the applicable predecessor transition.
15. Approval requirement is application-derived: analysis-only produces
    `NotApplicable`; any patch proposal produces `RequiredBeforeMutation`; model
    attempts to supply or override it fail.

An approved scoped-write test is not applicable because this increment
deliberately implements no write executor or registered repository tool. Its
absence must be reported, not represented as a pass.

## Verification commands

Run focused tests while implementing, then after the last relevant edit run:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked engineering_quality::tests::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked orchestrator::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_engineering_quality_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_research_knowledge_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

No manual application check is required unless implementation unexpectedly
adds a Tauri/React/native consumer, which is outside scope and requires owner
review. Independent architecture, security, code, technical-debt, and
readiness reviews remain mandatory before closeout. Finalize only after the
post-increment report and workspace fingerprint validate.

## Risks

- **Activation overclaim:** `Initial` could be mistaken for a live coding tool.
  Mitigation: versioned instructions, docs, and tests state fixture-only,
  proposal-only, unwired, and no execution.
- **Proposal-to-authority confusion:** patch text could be treated as a write.
  Mitigation: closed proposal types, denied action classifier, no executor,
  empty specialist tool allowlists, and mandatory false execution flags.
- **QA/security authority drift:** advisory output could be treated as approval
  or policy. Mitigation: strict false authority fields, application-derived
  final approval requirement, and regression tests against existing ownership.
- **Context amplification:** three structured predecessor results could exceed
  runtime limits. Mitigation: per-result, per-list, per-field, stage-input, and
  framing caps checked before state mutation.
- **Secret leakage:** fixture content could reach diagnostics. Mitigation:
  synthetic-only fixtures, redacted Debug, content-free events/audit, typed
  errors, and sentinel tests.
- **Orchestrator coupling:** another large sealed state could make later change
  risky. Mitigation: framework-neutral contracts in a dedicated module, one
  explicit selector, no generic engine, and a post-increment technical-debt
  review.

## Rollback or failure strategy

Before publication, rollback is removal of the new engineering module and
contract test, restoration of the three V1 deferred definitions, removal of
the sealed orchestrator selector/state, and restoration of the current-state
documentation. Existing D-083 through D-086 paths, `AgentRuntime`,
`NativeAgentRuntime`, ToolRegistry, policy, approval, audit, memory, documents,
Tauri, and React remain untouched.

If a required bound, atomic transition, cancellation invariant, redaction
property, or existing regression cannot be proved without adding a repository
tool, executor, permission, dependency, IPC, or wider route, stop the increment
as Blocked. Do not weaken the boundary or activate the three roles partially.

## Decisions made

- D-087 selects the fixture-only proposal mode and defers every live
  repository/tool/effect boundary.
- The workflow is a sealed application-service sequence, not generic
  delegation or a general workflow engine.
- All three specialists remain siblings at depth one with one active child.
- The existing policy profiles remain tool-ineligible and memory-disabled.
- Approval is required before a future consequential change, but no approval
  request is created in a workflow that has no actionable subject.
- Coding, QA, and Security activation is all-or-nothing for the verified sealed
  workflow; it grants no tools or operational capability.

## Discoveries

- The repository has no code-search, repository-read, patch-write, test,
  formatter, package-manager, or Git schema/executor in ToolRegistry.
- The existing non-executing governance foundation is sufficient to preserve
  denial but not to implement approved repository mutation.
- D-086 provides a useful sealed sequential-state precedent, but this workflow
  requires its own exact task/run caps, three strict specialist results, and
  approval-language boundary.

## Progress

- [x] 2026-08-12: inspected published D-086 state, current tools, governance,
      orchestration, agent definitions, tests, sandbox boundaries, and draft.
- [x] 2026-08-12: initial readiness result `Blocked` on absent exact tool,
      approval, contract, limit, failure, file, test, and rollback decisions.
- [x] 2026-08-12: owner selected a safe initial proposal-only workflow and
      explicitly prohibited Codex/external runtime integration and unrestricted
      mutation.
- [x] 2026-08-12: D-087 and this exact plan resolve the planning blocker without
      adding execution authority.
- [x] 2026-08-12: three fresh independent reviews returned `Ready with
advisories`; editorial advisories were reconciled and gate
      `agent-engineering-quality-workflow` began before source edits.
- [x] 2026-08-12: Milestone 3 catalog slice landed V2 Coding, QA, and Security
      instructions and `Initial` eligibility; definition units passed 6/6,
      registry contracts 7/7, generic orchestration contracts 22/22, scoped
      strict Clippy, formatting, and diff checks passed. Profiles, memory, and
      generic routes remain unchanged.
- [x] 2026-08-12: Milestones 1, 2, and 4 landed strict bounded engineering
      contracts, the sealed four-task/five-run orchestrator sequence, and 20
      public workflow contracts. Focused engineering, definition, governance,
      orchestration, and D-086 regressions plus strict Clippy, formatting, and
      diff checks pass.
- [x] 2026-08-12: source freeze validation passes: 8 engineering units, 10
      orchestrator units, 20 D-087 public contracts, registry 7, governance 10,
      generic orchestration 22, memory/document 10, D-086 18, runtime 20, and
      gateway 10. Rust formatting, all-target/all-feature check, strict Clippy,
      326 all-target tests with one intentional ignored probe, and `npm run
verify` including 124 frontend and 186 library tests pass.
- [x] 2026-08-12: independent architecture, security, code-health, technical-
      debt, and readiness review returns `PASS WITH ADVISORIES`. Before another
      multi-specialist workflow, consider decomposing private orchestrator
      internals without introducing a general workflow engine. No later
      owner-approved plan is Ready, so next-increment readiness is `Blocked`.

## Acceptance criteria

- [x] Coding, QA, and Security are `Initial` only for the sealed fixture-only
      workflow and advertise no unavailable tool or live-repository capability.
- [x] The strict Coding, QA, Security, and Personal synthesis contracts enforce
      exact application-derived identity, criterion/evidence provenance,
      per-field/list/input/encoded-request bounds, non-authority, nested
      unknown/duplicate rejection, and redaction.
- [x] The orchestrator alone sequences three depth-one siblings under the exact
      four-task/five-run/one-active-child/zero-retry limits.
- [x] Deterministic success, denial, partial failure, cancellation, atomicity,
      event, audit, activation, redaction, and regression tests pass.
- [x] Writes, path escape, deletion, package/dependency operations, test/format
      execution, Git operations, destructive shell, credentials, and network
      remain denied data with no dispatched effect.
- [x] QA cannot approve or fabricate execution evidence; Security cannot
      authorize or remediate; final synthesis derives `RequiredBeforeMutation`
      when mutation is proposed and `NotApplicable` otherwise.
- [x] ToolRegistry, policy profiles, approval behavior, execution
      `NotAttempted`, memory, Native/default runtime, generic routes, Tauri,
      React, manifests, and lockfiles remain unchanged.
- [x] Complete security-sensitive verification, documentation, repository,
      security, diff, session-end, and deterministic completion gates pass.

## Final results

The bounded proposal-only source and tests are implemented and frozen. Focused
and full source validation passes, including 326 all-target Rust tests with one
intentional ignored probe and complete `npm run verify`. Independent review
returns `PASS WITH ADVISORIES`; the sole advisory is to consider a bounded
private orchestrator decomposition before another multi-specialist workflow,
without introducing a general workflow engine. No tool, executor, filesystem,
process, Git, package, network, IPC, UI, dependency, commit, or push authority
was added. Final documentation, repository, security, diff, session-end, and
deterministic completion gates pass.

## Documentation updates

- [x] Synchronize architecture, product requirements, security model/checklist,
      project direction, ADR/assessment, roadmaps, current project memory,
      changelog, increment record, and post-increment review from observed
      implementation evidence.
- [x] Preserve D-087 and historical D-082 through D-086 evidence additively.
- [x] Do not claim repository inspection, test execution, mutation, approval,
      or security certainty beyond deterministic fixtures.
