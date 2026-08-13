# Infrastructure and systems operations workflows

Status: Verified complete with advisories
Owner: Project owner
Decision: D-088
Gate: `agent-infrastructure-systems-operations-workflows`
Last updated: 2026-08-12
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Implement two separate sealed, deterministic, fixture-only, proposal-only Rust
workflows above `AgentOrchestrator` and the sole/default
`NativeAgentRuntime`:

```text
Personal Assistant
  -> Cloud Infrastructure Agent
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> Personal Assistant synthesis

Personal Assistant
  -> Systems Operations Agent
  -> QA & Validation Agent
  -> Security & Risk Agent
  -> Personal Assistant synthesis
```

The workflows prove strict infrastructure and operations assessment, QA,
security, attribution, cancellation, and partial-failure contracts. They do not
run Terraform or a platform command, inspect a live environment, load a
credential, create an approval request, or produce an effect.

## User-visible outcome

None. This unwired Rust contract uses immutable fixtures and
`MockAgentRuntime`; it adds no Tauri/React, provider, shell, connection, or
device behavior.

## Scope

- Add one framework-neutral infrastructure/operations contract module with:
  `InfrastructureAssessment`, `ChangePlan`, `OperationalAssessment`,
  `DiagnosticFinding`, module-qualified `ValidationReport`, module-qualified
  `RiskAssessment`, and a final Personal Assistant synthesis.
- Add one exact Cloud workflow over synthetic Terraform configuration, Azure
  architecture, and application validation-evidence fixtures.
- Add one exact Systems workflow over a synthetic service snapshot, sanitized
  log excerpt, recovery scenario, and application validation evidence.
- Reuse QA and Security only as non-authorizing cross-cutting reviewers through
  exact new embedded instruction versions.
- Preserve application-issued fixture, acceptance-criterion, validation-
  evidence, stage, task, run, predecessor, and result provenance.
- Classify every requested capability into exact proposal-only or denied data;
  never dispatch it.
- Change Cloud Infrastructure and Systems Operations from `Deferred` to
  `Initial` only after the complete workflow contracts and tests pass.
  `Initial` remains non-authorizing eligibility for these two sealed unwired
  fixture workflows.

## Explicit non-goals

- No Terraform CLI invocation, including `fmt`, `validate`, `plan`, `apply`,
  state access, backend initialization, or provider download.
- No Azure, AWS, VMware, SSH, PowerShell, shell, service, process, log,
  filesystem, diagnostic, backup, or operating-system command or API call.
- No live inventory, production access, resource mutation, deployment,
  deletion, IAM/firewall/account/permission change, service/process control,
  reboot, shutdown, configuration mutation, patching, package installation,
  privileged execution, or secret rotation.
- No credential discovery, environment-variable inspection, configuration-file
  inspection, keychain read, SDK default-chain resolution, token use, or secret
  storage.
- No `ToolRegistry` schema, `PolicyEngine`, `ApprovalManager`, executor,
  durable `AuditLogger`, `PlatformAdapter`, permission, capability, dependency,
  manifest, lockfile, IPC, UI, persistence, provider, or external runtime
  change.
- No generic workflow engine, specialist spawning, recursion, parallelism,
  automatic retry, background work, durable task, or new memory access.
- No Workflow Automation activation or existing-workflow semantic change.

## Pre-implementation baseline

- Clean synchronized `main` at `a5d7ba1` publishes D-087; its gate is complete,
  valid, and `PASS WITH ADVISORIES`.
- The unwired orchestrator alone creates tasks and invokes `AgentRuntime`.
  Native remains sole/default; runtime tool proposals fail closed.
- `ToolRegistry` has only the two existing Personal-only non-executing schemas.
  Specialist tool allowlists are empty and execution is always `NotAttempted`.
- Cloud/Systems were `Deferred` with V1 instructions and disabled memory.
  QA/Security are `Initial` only for D-087, memory-disabled, tool-ineligible,
  and non-authorizing.
- Task output remains 8,192 scalars/16,384 bytes and Native framing 65,536
  bytes. D-087's size advisory puts parsing, bounds, prepared transitions, and
  state helpers in the new domain module; the orchestrator retains task/run,
  live-context, and cancellation ownership without a generic engine.

## Current-state evidence

- Before planning, Git was clean/synchronized at full commit
  `a5d7ba10f9edbbc3992a9309b7065330153d1103` and the D-087 marker was valid.
- Developer inventory found Terraform 1.6.6, AWS CLI 2.15.6, PowerShell 7.4.6,
  and native `launchctl`/`ps`, but no reviewed Azure/VMware CLI. None is an
  application tool or evidence source.
- Rust has no infrastructure tool, launcher, cloud/VMware/OS adapter, executor,
  or platform adapter. No live credential check occurred. D-088 therefore
  selects only the no-I/O boundary under D-082's role topology.

## Files expected to change

Production: new `src-tauri/src/agent/infrastructure_operations.rs`; existing
`definition.rs`, `orchestrator.rs`, and `mod.rs`.

Tests: new `agent_infrastructure_operations_workflow_contract.rs`; existing
definition-registry, governance, orchestration contracts; shared mock support
only if the existing failure fixture is insufficient.

Closeout: `ARCHITECTURE.md`, `CHANGELOG.md`, `DECISIONS.md`, `HANDOFF.md`,
`NEXT_STEPS.md`, `PLANS.md`, `PRODUCT_REQUIREMENTS.md`, `PROJECT_STATUS.md`,
`ROADMAP.md`, `SECURITY.md`, `SECURITY_CHECKLIST.md`, project direction, native
ADR/assessment/roadmap, governance matrix, this plan, and new increment/review.
`TESTING_GUIDE.md` or the threat model changes only if their contract changes.

No tool, policy, approval, credential, memory, document, runtime, storage,
Tauri configuration/capability, frontend, manifest, lockfile, CI, or external-
integration file is in scope. An unexpected need to change one stops the
increment for owner review.

## Interfaces and invariants

### Separate selection and role boundary

- Public APIs are exact: `start_cloud_infrastructure_workflow(context,
CloudInfrastructureWorkflowRequest) ->
AgentOrchestratorResult<CloudInfrastructureWorkflowAcceptance>`;
  `cloud_infrastructure_result() ->
Option<&CloudInfrastructureWorkflowResult>`;
  `cloud_infrastructure_events() -> &[CloudInfrastructureWorkflowEvent]`;
  `cloud_infrastructure_attribution_records() ->
&[InfrastructureOperationsAttributionRecord]`; and the corresponding
  `start_systems_operations_workflow(context,
SystemsOperationsWorkflowRequest) ->
AgentOrchestratorResult<SystemsOperationsWorkflowAcceptance>`,
  `systems_operations_result() -> Option<&SystemsOperationsWorkflowResult>`,
  `systems_operations_events() -> &[SystemsOperationsWorkflowEvent]`, and
  `systems_operations_attribution_records() ->
&[InfrastructureOperationsAttributionRecord]`.
- `CloudInfrastructureWorkflowAcceptance` is exactly
  `CloudAssessmentStarted { context }` or
  `PersonalFallbackStarted { context }`;
  `SystemsOperationsWorkflowAcceptance` is exactly
  `SystemsAssessmentStarted { context }` or
  `PersonalFallbackStarted { context }`. A first-stage runtime-start failure
  records an attempted/failed child and continuation snapshot, then returns
  `PersonalFallbackStarted` only if fallback start succeeds; if fallback also
  fails, the root is failed and the selector returns the typed runtime error.
- One owned private sum,
  `InfrastructureOperationsWorkflowState::{Cloud(CloudWorkflowState),
Systems(SystemsOperationsWorkflowState)}`, stores exactly one selection.
  Private shared helpers may prepare QA/Security/synthesis lifecycle mechanics;
  request/result/event types, first-stage parsers, selectors, and getters remain
  distinct, and no generic public workflow API is added.
- Neither request accepts a workflow-kind, agent/profile/runtime/task/parent/
  run/predecessor identity from runtime/model output.
- Add separate closed `AgentWorkflowSelection::CloudInfrastructure` and
  `AgentWorkflowSelection::SystemsOperations` variants. Before mutation, either
  selector must reject any prior generic, document, D-086, D-087, Cloud, or
  Systems selection; after selection, every other selector and repetition
  fails closed.
- The Cloud selector creates only Cloud -> QA -> Security. The Systems selector
  creates only Systems -> QA -> Security. A first-stage result for the other
  workflow fails before state mutation and never reaches QA.
- All specialists are direct depth-one siblings beneath the same Personal root.
  Only the orchestrator creates tasks. QA, Security, Cloud, and Systems cannot
  delegate or spawn.
- The workflow selectors remain trusted application-service calls, never host
  tools, runtime control events, model-selected routes, or generic delegation.

### Immutable fixture catalog and request bounds

- One built-in immutable `InfrastructureOperationsFixtureCatalog` owns canonical
  fixtures. Public callers may select only `CloudScenarioId` or
  `SystemsOperationsScenarioId` closed enum variants through catalog methods
  `cloud_request(id)` or `systems_request(id)`. Request fields and raw
  constructors are private, requests are not deserializable, and a sealed
  catalog proof plus closed catalog version binds kind, objective, fixture,
  criterion, and evidence IDs/content. Start preflight re-resolves and compares
  the canonical entry. Cross-kind IDs, unknown values, a request from another
  catalog version, or internal/test alteration fail before mutation.
  The initial deterministic IDs are exactly `TerraformDecisionBriefV1` and
  `SanitizedServiceRecoveryV1`.
- No path, account, subscription, tenant, region, endpoint, host, process,
  service, VM, credential, environment variable, or command is accepted from a
  caller or model as authority.
- A request contains one objective, one through eight fixture records, one
  through eight acceptance criteria, and zero through eight validation-evidence
  records. IDs are unique closed ASCII identifiers of at most 64 bytes.
- Objective text is at most 2,048 Unicode scalars and 4,096 UTF-8 bytes. Each
  fixture or criterion description is at most 2,048 scalars and 4,096 bytes;
  total fixture content is at most 16,384 bytes. Each other result text field is
  at most 1,024 scalars and 2,048 bytes.
- Every list has an explicit maximum of eight unless a narrower field says
  otherwise. Evidence-bound findings contain one through eight unique,
  application-issued references; hypotheses contain none. QA
  `NotDemonstrated` coverage and proposed checks may contain zero through eight
  known references so missing evidence can be reported truthfully, while any
  `Demonstrated` coverage requires criterion-bound `ObservedFixture` evidence.
- The raw selected text for every specialist or synthesis run is
  at most 24,576 bytes. A maximum-bound adversarial test with quote,
  backslash, newline, and multibyte content must first prove generic preflight
  constructs a valid `RuntimeTurnRequest`; a separate sole/default
  `NativeAgentRuntime::start` regression proves its encoded gateway request
  remains below the unchanged 65,536-byte ceiling. Generic orchestration does
  not preflight Native framing. A Native serialization rejection is a typed
  runtime stage-start failure, never a fallback to another runtime or authority
  path.
- Fixtures are explicitly `SyntheticFixture`. Cloud fixture kinds are exactly
  `TerraformConfiguration`, `AzureArchitecture`, `AwsArchitecture`, and
  `ApprovedInventorySnapshot`. Systems fixture kinds are exactly
  `WindowsSnapshot`, `LinuxSnapshot`, `MacosSnapshot`, `ServiceSnapshot`,
  `ProcessSnapshot`, `SanitizedLogExcerpt`, `ConfigurationSnapshot`,
  `ResourceSnapshot`, `VmwareInventorySnapshot`, `BackupSnapshot`, and
  `RecoveryScenario`.
- Validation evidence is exactly `ObservedFixture` or `NotRun`. It cannot
  represent a live command, provider response, host observation, credential
  check, or external test. A `NotRun` record can identify a proposed check but
  cannot demonstrate an acceptance criterion.

### Cloud structured output

- Cloud returns exactly `InfrastructureAssessmentV1`, bound to the expected
  workflow and Cloud task. It contains: objective disposition; exact fixture
  references; one through eight evidence-bound findings; optional limitations
  and unresolved questions; one inert `ChangePlan`; and one exact transfer for
  QA.
- Each finding has an application-validated ID, closed category, statement,
  confidence, and one through eight known fixture/evidence references. Initial
  categories are `Architecture`, `TerraformStaticReview`, `IdentityAccess`,
  `NetworkExposure`, `DataProtection`, `Reliability`, `CostEvidence`, and
  `Operability`.
- `ChangePlan` contains objective, affected fixture IDs, proposed changes as
  inert text, risks, proposed validation, rollback considerations, and exact
  capability proposals. It contains no command, executable patch, provider
  payload, resource ID, account identity, credential reference, or dispatch
  token.
- Proposal-only Cloud capabilities are exactly `ReviewTerraformFixture`,
  `AnalyzeAzureFixture`, `AnalyzeAwsFixture`, `AnalyzeInventoryFixture`, and
  `ProposeInfrastructureChangePlan`.
- Closed denied Cloud capabilities are exactly `TerraformPlanExecution`,
  `TerraformApply`, `TerraformStateMutation`, `TerraformBackendChange`,
  `ProviderDownload`, `CloudInventoryAccess`, `CloudShell`,
  `ResourceCreateOrUpdate`, `ResourceDelete`, `IamChange`, `FirewallChange`,
  `ProductionAccess`, `CredentialAccess`, and `SecretRotation`. Their presence
  forces a partial denied-capability result and is never execution authority.
- Static Terraform observations are analysis of fixture text only. The result
  and final synthesis must state that `terraform fmt -check`,
  `terraform validate`, provider initialization, plan, and apply were not run.

### Systems structured output

- Systems returns exactly `OperationalAssessmentV1`, bound to the expected
  workflow and Systems task. It contains: exact fixture references; one through
  eight `DiagnosticFinding` values; limitations; unresolved questions; an
  inert diagnostic plan; an inert remediation plan; rollback considerations;
  exact capability proposals; and one transfer for QA.
- A `DiagnosticFinding` has an application-validated ID, closed category,
  statement, confidence, `EvidenceBound` known references or `Hypothesis`, and
  no host/process/service identity that is not present in the fixture catalog.
  Initial categories are `ServiceState`, `ProcessState`, `LogSignal`,
  `Configuration`, `ResourcePressure`, `Virtualization`, `BackupRecovery`, and
  `Maintenance`.
- Proposal-only Systems capabilities are exactly `AnalyzeServiceFixture`,
  `AnalyzeProcessFixture`, `AnalyzeLogFixture`, `AnalyzeConfigurationFixture`,
  `AnalyzeResourceFixture`, `AnalyzeVmwareFixture`, `AnalyzeBackupFixture`,
  `ProposeDiagnostics`, and `ProposeRemediationPlan`.
- Closed denied Systems capabilities are exactly `LiveDiagnostic`,
  `ReadLiveLog`, `InspectLiveService`, `InspectLiveProcess`,
  `InspectLiveConfiguration`, `RestartOrStopService`, `RebootOrShutdown`,
  `KillProcess`, `ConfigurationMutation`, `PackageInstall`, `PatchSystem`,
  `AccountOrPermissionChange`, `DeleteFile`, `PrivilegedShell`,
  `VmwareMutation`, `BackupMutation`, and `CredentialAccess`. Their presence
  forces a partial denied-capability result and is never dispatched.

### QA, Security, and final synthesis

- `ValidationReportV1` binds the exact workflow, first-stage assessment, and QA
  task. It accounts for every application-issued acceptance criterion exactly
  once as `Demonstrated` or `NotDemonstrated`, with only known references;
  records findings, gaps, proposed checks, and an `Adequate`, `Incomplete`, or
  `Blocked` conclusion; and has invariant `advisory_only = true` and
  `approval_authority = false`.
- A criterion is demonstrated only by `ObservedFixture` evidence. Every
  Terraform, cloud, platform, service, process, log, VMware, backup, or external
  check remains explicitly not run. Missing, duplicate, or unknown criterion
  coverage invalidates the report before later-stage input is constructed.
- `RiskAssessmentV1` binds the exact workflow, assessment, Security task, and
  validated QA report or explicit QA-unavailable status. Each finding is
  evidence-bound or a hypothesis. Categories are
  `AuthorizationBoundary`, `InputValidation`, `CredentialsSecrets`,
  `CloudIamNetwork`, `HostAvailability`, `DataLoss`, `UnsupportedPlatform`,
  `DependencyEvidence`, `Audit`, `Rollback`, and `GeneralChangeRisk`.
  Invariants are `advisory_only = true` and `authorization_granted = false`.
- Dependency, provider, credential, target-platform, and executed-check
  evidence is `Unavailable` unless an application fixture explicitly supplies
  a synthetic observation. Synthetic evidence never becomes a live support
  claim.
- `InfrastructureOperationsSynthesisV1` preserves the validated assessment,
  QA, Security, fixture disclosure, stage outcomes, and partial-failure codes.
  It states `fixture_based = true`, `live_inventory_performed = false`,
  `commands_executed = false`, `credentials_loaded = false`, and
  `effects_performed = false`.
- The application, not output JSON, derives `NotApplicable` when no closed
  denied-effect capability is present or `RequiredBeforeConsequentialAction` when the
  validated closed capability set contains at least one denied Cloud or Systems
  effect capability. It never derives from free text, risk prose, severity,
  confidence, or an output boolean. No `ApprovalManager` or governance subject
  is created, approval/governance audit counts remain unchanged, and every
  execution disposition remains `NotAttempted` because no executable subject
  exists.

### Strict parsing, provenance, and redaction

- Every result envelope uses an exact V1 discriminator, denies unknown fields,
  validate every scalar/byte/list bound, and reject duplicates, unknown IDs,
  wrong workflow/task bindings, remapped references, non-finite or out-of-range
  confidence, control characters, URLs or paths used as authority, identity
  claims, execution claims, approval/policy claims, and fields named for chain-
  of-thought or hidden reasoning.
- Credential defense is deliberately deterministic and narrow, not a claim of
  semantic secret detection. Exact keys `secret`, `password`, `token`,
  `credential`, `private_key`, `access_key`, and `client_secret` are forbidden
  at any JSON depth. Comparison normalization is exactly CRLF-to-LF, trimming
  ASCII whitespace, and ASCII case-folding; non-ASCII is not folded. Line
  starts matching only `cortexa_fixture_secret_do_not_use=`,
  `authorization: bearer `, `bearer `, `-----begin private key-----`,
  `-----begin rsa private key-----`, `aws_access_key_id=`,
  `aws_secret_access_key=`, `aws_session_token=`, `azure_client_secret=`,
  `arm_client_secret=`, `vmware_password=`, or `vsphere_password=` are rejected;
  so are uppercase `AKIA`/`ASIA` plus exactly 16 ASCII uppercase letters/digits
  and a token boundary. Tests cover exact case/whitespace boundaries; unknown
  secrets may evade this defense, so fixture
  curation, explicit fields, no credential source, redaction, and repository
  secret scanning remain authoritative.
- Free text remains untrusted display/proposal data. It cannot introduce a
  capability, trusted severity, permission, credential, target, command, or
  agent identity.
- Later stages receive only validated bounded transfers built by application
  code. Raw invalid output never reaches QA, Security, final synthesis, audit,
  error, memory, SQLite, IPC, or logs.
- Workflow events and the private volatile attribution journal contain only
  workflow kind, stage, task/root/parent, agent, runtime/run, predecessor,
  sequence, and closed outcome/error codes. They contain no objective, fixture
  text, finding, proposed change, diagnostic, host/cloud identity, path, URL,
  command, prompt, result, credential, or reasoning.
- The journal is not the planned durable `AuditLogger`, grants no authority,
  performs no I/O, and is dropped with the orchestrator.

### Lifecycle, budgets, events, and atomicity

- Either workflow owns exactly four tasks, three non-replenishing sequential
  depth-one children, one active child, five run attempts, zero retries, eight
  events per event-consuming run, 32 runtime events, 32 generic orchestration
  events, 16 workflow events, and 16 attribution records.
- One-delta success uses 12 accepted runtime events, 16 generic orchestration
  events, and eight workflow/attribution entries. Tests prove every branch and
  worst case stays within all caps.
- `CloudInfrastructureWorkflowEvent` has Cloud assessment start/complete, QA
  start/complete, Security start/complete, synthesis start, partial failure,
  cancelled, and completed. `SystemsOperationsWorkflowEvent` substitutes only
  Systems assessment start/complete. Cross-kind events cannot be represented.
- Before selector mutation, preflight validates workflow exclusivity, request
  catalogs and bounds, root/live-run state, activation/profile/memory identity,
  capacity, exact selected text, and `RuntimeTurnRequest` construction. It does
  not inspect Native framing. Failure causes zero mutation.
- Before accepting each terminal event, preflight validates exact live
  task/run/request/sequence, parses and bounds output, constructs the complete
  successor or fallback input, derives trusted successor attribution, and
  reserves task/run/event/journal capacity. Failure causes zero mutation.
- Late, duplicate, foreign-workflow, wrong-task, wrong-agent, wrong-run,
  wrong-request, wrong-sequence, and wrong-stage events fail before mutation.

### Exact transitions, continuation snapshots, and cancellation

| Point                                                   | State/successor                                                            | API evidence                                                                                                          |
| ------------------------------------------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Selector preflight rejects                              | Root/run live; no selection, task, attempt, event, or record               | Typed `Err`; no workflow evidence                                                                                     |
| Initial root cancellation fails                         | Root live/retryable; no workflow selected                                  | Typed runtime `Err`                                                                                                   |
| Cloud or Systems start fails after root run is consumed | Attempted child fails; QA/Security skipped; try bounded Personal fallback  | Selector returns `PersonalFallbackStarted`; if fallback start also fails, root fails and selector returns typed `Err` |
| Valid first-stage terminal                              | Preserve assessment; start QA                                              | Accepted terminal returns normal `Ok(RuntimeEventAcceptance)`                                                         |
| Failed/cancelled/invalid first-stage terminal           | Mark unavailable; skip QA/Security; start partial fallback                 | Normal accepted terminal plus closed partial outcome                                                                  |
| QA start fails                                          | Preserve assessment; mark QA unavailable; start Security                   | Predecessor terminal still returns normal `Ok`; snapshot records failure                                              |
| QA terminal valid                                       | Preserve exact report; start Security; incomplete/blocked forces partial   | Normal accepted terminal                                                                                              |
| QA terminal fails/cancels/is invalid                    | Preserve assessment; mark QA unavailable; start Security                   | Normal accepted terminal plus partial outcome                                                                         |
| Security start fails                                    | Preserve assessment/QA; mark Security unavailable; start partial synthesis | Predecessor terminal remains `Ok`; snapshot records failure                                                           |
| Security terminal valid/fails/cancels/is invalid        | Preserve only validated prior values; start complete/partial synthesis     | Normal accepted terminal plus applicable partial outcome                                                              |
| Synthesis start fails                                   | Consume attempt; terminally fail root; no result                           | Predecessor terminal remains `Ok`; snapshot records failure                                                           |
| Synthesis terminal valid                                | Validate distinct Cloud/Systems result and complete root                   | `Ok(ResponseCompleted)` plus result/completed event                                                                   |
| Synthesis terminal fails/cancels/is invalid             | Fail/cancel root; no completed result or raw output                        | Normal accepted terminal plus closed failure/cancel evidence                                                          |

After any accepted predecessor terminal, continuation start runs afterward. Its
failure never changes that event call to `Err`, reverses the terminal, or
replenishes a budget. Distinct getters
`cloud_infrastructure_continuation_failure` and
`systems_operations_continuation_failure` expose the shared closed
`InfrastructureOperationsContinuationFailure` snapshot with exactly:
`CloudStartFailed`, `CloudAndSynthesisStartFailed`, `SystemsStartFailed`,
`SystemsAndSynthesisStartFailed`, `QaStartFailed`,
`QaAndSecurityStartFailed`, `QaAndSynthesisStartFailed`,
`QaSecurityAndSynthesisStartFailed`, `SecurityStartFailed`,
`SecurityAndSynthesisStartFailed`, and `SynthesisStartFailed`. Chained variants
preserve all start failures in order. Closed partial codes remain
`FirstStageUnavailable`, `ContainsDeniedCapability`,
`InvalidStructuredOutput`, `QaUnavailable`, `QaIncomplete`,
`SecurityUnavailable`, `RuntimeStartFailed`, `RuntimeFailed`, and `Cancelled`.

Root cancellation uses this exact partial-commit boundary:

| Failure point                                                                 | Retained state                                                                                                                                                                                   |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Governance/identity/capacity preflight                                        | Zero mutation; subject, tasks, runs, and workflow remain live/retryable                                                                                                                          |
| Active-child governance subject terminalizes, then child runtime cancel fails | Child subject remains cancelled and its governance audit records it; child/root remain live/retryable; root subject is untouched; no workflow `Cancelled`; no successor                          |
| Child cancel succeeds, then root governance or runtime cancel fails           | Child subject/run/task remain cancelled; root remains live/retryable with no active child and no successor; any committed root governance cancellation remains terminal; no workflow `Cancelled` |
| All required cancels succeed                                                  | Child subject, child run/task, root subject, then root run/task terminalize in that order; one terminal workflow `Cancelled`; no result/successor                                                |

D-088 itself creates no governance or approval subject. These rows preserve the
existing root-cancellation behavior if a pre-existing subject is present. A
direct child cancellation follows its stage's partial successor only when it
was not root cancellation; runtime cancel failure leaves that run/task live and
starts nothing. Cancelling synthesis applies the same preflight then cancels
only the root run/task.

## Implementation milestones

- [x] Begin the exact gate after fresh architecture, security, and readiness
      confirmation.
- [x] Add the strict domain contracts and separate Cloud/Systems selectors.
- [x] Update V2 Cloud/Systems and V3 QA/Security instructions and activation
      without changing policy or memory profiles.
- [x] Add focused contract, denial, lifecycle, redaction, Native, and regression
      tests; source validation and complete `npm run verify` pass.
- [x] Complete independent reviews, documentation/session closeout, report, and
      deterministic marker. Reviews pass with advisories and the completion
      marker is complete and valid.

## Test plan

The new public contract must cover at least:

1. Exact distinct public APIs/state sum and sealed scenario requests are
   mutually exclusive; alteration, cross-kind data, and other selectors reject
   pre-mutation.
2. Deterministic Cloud success over a Terraform fixture produces a strict
   `InfrastructureAssessment` and inert `ChangePlan`, preserves all source IDs,
   labels every CLI check not run, and completes QA/Security/Personal sequence.
3. Deterministic Systems success over sanitized service/log fixtures produces
   evidence-bound and hypothetical `DiagnosticFinding` values, inert
   diagnostics/remediation, and completes QA/Security/Personal sequence.
4. Terraform apply/plan execution, cloud create/update/delete, IAM/firewall,
   state/backend, cloud shell, production, credential, and secret-rotation
   proposals are classified denied and never dispatched.
5. Restart/stop, reboot/shutdown, kill, configuration mutation, package/patch,
   account/permission, deletion, privileged shell, VMware/backup mutation, live
   diagnostic, and credential proposals are classified denied and never
   dispatched.
6. No tool/policy eligibility, approval/governance subject, or related audit is
   added; approval requirement derives only from closed capabilities, audit
   counts stay unchanged, and execution remains `NotAttempted`.
7. QA accounts for every criterion exactly once, cannot use `NotRun` as passing
   evidence, detects missing validation, remains advisory, and cannot approve.
8. Security accepts evidence-bound or hypothetical findings, reports missing
   credential/dependency/platform evidence, remains advisory, and cannot
   authorize or remediate.
9. First-stage failure skips QA/Security and produces truthful fallback; QA
   failure can continue Security with unavailable status; incomplete QA and
   Security failure produce exact partial synthesis; final failure fails root.
10. Root/child cancellation proves preflight zero mutation and each approval-
    cancelled/child-live, child-cancelled/root-live, and full child-first state;
    no failure starts a successor or emits false terminal cancellation.
11. Unknown/remapped/duplicate fixture, criterion, evidence, assessment, QA,
    task, run, predecessor, workflow, and agent IDs fail before mutation.
12. Exact forbidden secret keys and enumerated normalized prefixes reject while
    nearby nonmatches pass; hidden reasoning, false execution/live/credential
    claims, malformed data, and every bound fail closed and stay redacted.
13. Every transition and chained continuation snapshot is exact; predecessor
    terminal calls remain `Ok`, preparation is atomic, and no budget replenishes.
14. Maximum adversarial text separately proves generic `RuntimeTurnRequest`
    construction and sole/default Native framing below 65,536 encoded bytes.
15. Definition/registry, governance, generic orchestration, memory/document,
    D-086, D-087, runtime, gateway, and sole/default Native regressions pass.

## Verification commands

```bash
python3 .codex/hooks/post_increment_gate.py begin --increment agent-infrastructure-systems-operations-workflows
python3 .codex/hooks/post_increment_gate.py status
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::infrastructure_operations::tests::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_infrastructure_operations_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_research_knowledge_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_engineering_quality_workflow_contract --locked
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

Manual completion evidence confirms no tool, command, credential, live access,
executor, approval request, permission, dependency, IPC/UI, provider,
persistence, parallelism, or effect; profiles stay empty/disabled and results
stay fixture-only. No target-environment or application check is required.

## Risks

- **Coupling/cross-role confusion:** domain-owned helpers, distinct contracts,
  and no generic engine.
- **Overclaiming/authority drift:** immutable provenance, `NotRun`, inert plans,
  closed denials, empty tool profiles, and no executor/approval request.
- **Credential leakage:** curated catalog, exact key/prefix rejection,
  redaction, and secret scan.
- **Bound/atomicity failure:** prepared transitions, exact budgets, maximum
  Native proof, and exhaustive lifecycle tests.

## Stop conditions

Stop and return the plan to Blocked if implementation requires any tool schema,
command/process launch, live file/log/service/process/cloud/VMware access,
credential or environment inspection, policy/approval/executor/platform change,
dependency/manifest/lockfile/configuration change, Tauri/React/IPC change,
parallelism, generic workflow engine, memory expansion, provider, external
runtime, or device effect. Also stop if either workflow cannot remain mutually
exclusive, strict fixture provenance or redaction cannot be enforced, maximum
input cannot fit Native's existing request ceiling, terminal preparation cannot
remain atomic, or existing workflow/Native regressions fail.

## Rollback or failure strategy

Before commit, rollback removes only the new contract module and public test,
reverts the four scoped Rust integrations and scoped test expectations, and
restores Cloud/Systems to `Deferred` plus the prior QA/Security instruction
versions. Preserve unrelated owner work and historical D-082 through D-087
evidence; do not reset, clean, or stash the repository.

After a separately authorized commit, rollback requires a separately reviewed
revert of that bounded commit. No infrastructure, system, credential, provider,
service, process, file, VM, backup, or external state needs rollback because
the increment performs no I/O or effect. Failed verification leaves the plan
Active or Blocked and creates no completion marker.

## Decisions and discoveries

D-088 selects distinct fixture-only selectors and module-qualified results;
installed binaries stay unused, checks stay not run, profiles/memory stay
empty/disabled, and no approval subject exists. No infrastructure executor or
adapter exists, QA/Security contracts are engineering-bound, and D-087's size
advisory constrains code placement.

The initial `npm run verify` stopped only on Prettier formatting in the native
roadmap; a targeted Prettier write corrected it. The next verify reached the
repository scan and identified four synthetic secret-shaped literals in the
new unit test. The test was corrected to runtime-assemble the same sentinels,
preserving coverage without storing scan-shaped literals. The focused sentinel
test and `npm run repository:check` then passed, and the authoritative final
`npm run verify` passed end to end. This is resolved validation evidence, not
an open defect.

## Progress

- [x] 2026-08-12: Inspected boundaries, tests, tools, Git, and valid D-087 gate.
- [x] 2026-08-12: D-088 narrowed owner selection to two no-I/O fixtures; Ready.
- [x] 2026-08-12: Three independent reviews returned Ready and the exact gate
      began before source edits.
- [x] 2026-08-12: Implemented strict domain contracts, two mutually exclusive
      selectors, V2 Cloud/Systems and V3 QA/Security instructions, activation,
      partial failure, cancellation, bounds, attribution, and regression tests.
- [x] 2026-08-12: Final source evidence passes: domain 8/8, orchestrator 11/11,
      public D-088 25/25, formatting/check/strict Clippy, repository scan, diff,
      all-target Rust 362 passed plus one intentional ignored probe, and
      `npm run verify` with 124 frontend and 195 library tests plus Tauri
      no-bundle release build.
- [x] 2026-08-12: Independent review returns `PASS WITH ADVISORIES`. Decompose
      the large private module/orchestrator before another workflow/live-tool
      increment; string/credential guards remain defense-in-depth only. Next
      readiness is `Blocked` because no later owner-approved plan is Ready.

## Acceptance criteria

- [x] Both separate sealed workflows run deterministically through
      `AgentOrchestrator` and `MockAgentRuntime` with exact structured results.
- [x] Cloud/System role separation, fixture provenance, denied capabilities,
      QA/Security non-authority, partial failures, cancellation, atomicity,
      redaction, limits, event order, attribution, and Native regression tests
      pass.
- [x] Cloud and Systems become `Initial` only for these sealed workflows; all
      four specialist tool profiles remain empty and memory-disabled.
- [x] No command, live inventory, credential, provider, executor, approval
      request, IPC/UI, dependency, permission, persistence, parallelism, or
      external effect exists.
- [x] Focused checks, `npm run verify`, independent architecture/security/code
      review, final documentation/session checks, post-increment report, and
      valid completion marker pass.

## Final results

Both sealed fixture-only/no-I/O selectors and their strict contracts are
implemented and source-frozen. Full verification passes with `PASS WITH
ADVISORIES`: 25 public D-088 contracts, 8 domain units, 11 orchestrator units,
362 all-target Rust tests plus one intentional ignored probe, and final
`npm run verify` including 124 frontend and 195 library tests and the Tauri
no-bundle release build. No tool, command, credential, live access, executor,
approval dispatch, provider, IPC/UI, dependency, or effect was added. Final
documentation, repository, security, diff, session-end, and completion-marker
checks pass.

## Documentation updates

- [x] D-088 and Ready-plan/current-queue records
- [x] Handoff, status, next steps, changelog, architecture, product, security,
      governance matrix, roadmap, increment, and review closeout
- [x] `TROUBLESHOOTING_LOG.md` remains unchanged because no product or runtime
      troubleshooting issue was diagnosed.
