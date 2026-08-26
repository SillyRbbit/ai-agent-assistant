# Native multi-agent deterministic demonstrations

Status: Executable acceptance evidence
Last validated: 2026-08-25
Canonical command: `npm run test:agent-acceptance`
Plan:
[`2026-08-11-multi-agent-end-to-end-demonstrations.md`](../plans/2026-08-11-multi-agent-end-to-end-demonstrations.md)
Related artifacts:
[`NATIVE_MULTI_AGENT_FIXTURES.md`](NATIVE_MULTI_AGENT_FIXTURES.md) and
[`NATIVE_MULTI_AGENT_ACCEPTANCE.md`](NATIVE_MULTI_AGENT_ACCEPTANCE.md)

## Read this disclosure first

These are deterministic architecture demonstrations, not live assistant or
external-operation demonstrations.

- Workflow responses are closed fixture strings accepted through the
  application-owned contracts by test-only `MockAgentRuntime`.
- The mock descriptor reports the sole closed runtime identity, `Native`, but
  it is not `NativeAgentRuntime` with a configured provider.
- No provider or live model is configured. No live network, cloud, host,
  service, browser, private repository, credential, or production document is
  used by the demonstrations. Cargo may fetch locked crates when its local
  cache is cold.
- The real application-owned task, orchestrator, validation, policy, approval,
  bounded document, memory, and volatile audit code is exercised where each
  evidence row says so.
- No real governed tool executes. Every governance execution disposition is
  `NotAttempted`.
- The macOS approval result is simulated and fed through the real typed source
  resolution path. No native dialog is displayed in this suite.
- D-091 parallelism is same-thread retained-run event multiplexing, not thread,
  process, provider, or distributed concurrency.
- The frontend Command Center is a separate deterministic fixture projection.
  It is not connected to these Rust workflows and is not backend evidence.

## Evidence classification

| Label                     | Meaning in this document                                                                 |
| ------------------------- | ---------------------------------------------------------------------------------------- |
| Deterministic fixture     | Immutable synthetic input or closed output authored in repository source/tests           |
| `MockAgentRuntime`        | Test-only runtime double; emits no provider, network, process, tool, or device operation |
| `NativeAgentRuntime`      | Sole/default application runtime wrapper; no configured provider or live model           |
| Real governed boundary    | Production Rust validation/policy/approval/audit/document/memory/orchestrator code       |
| Simulated external result | A closed test value that represents an external or operating-system outcome              |
| Product/external effect   | Consequential product/device/service mutation; **none occur in this suite**              |

The harness does create normal local build output, and the document scenario
creates and cleans one bounded temporary fixture file. Those effects are test
infrastructure, not assistant/device operations.

## Summary

| #   | Demonstration                 | Deterministic result | Principal evidence                                                                                              | Qualification                                                                                             |
| --- | ----------------------------- | -------------------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| 1   | Personal direct response      | Pass                 | `personal_assistant_can_complete_directly_without_a_child`                                                      | Fixture text through `MockAgentRuntime`; no delegation                                                    |
| 2   | Research brief                | Pass                 | `deterministic_fixture_workflow_preserves_provenance_and_exact_order` plus partial-failure contracts            | Repository source fixtures; no retrieval                                                                  |
| 3   | Document knowledge            | Pass                 | `selected_markdown_and_approved_shared_memory_flow_only_to_knowledge_and_synthesis`                             | Temporary synthetic approved file; real bounded reader; no silent shared persistence                      |
| 4   | Engineering review            | Pass                 | `orchestrated_success_is_sequential_bounded_attributed_and_proposal_only`                                       | Synthetic repository catalog; no repository access, edit, test run, commit, or push                       |
| 5   | Cloud assessment              | Pass                 | `deterministic_cloud_workflow_is_sequential_attributed_fixture_only_and_inert`                                  | Static synthetic Terraform/Azure text; no plan/apply/API                                                  |
| 6   | Systems incident              | Pass                 | `deterministic_systems_workflow_preserves_evidence_hypotheses_and_no_effects`                                   | Sanitized synthetic snapshot/log; no restart or mutation                                                  |
| 7   | Workflow proposal/manual step | Pass with advisory   | proposal validation, non-executable checkpoint, take-once dispatch, and manual Research bridge contracts        | Owner-approved separate branches; no checkpoint-to-approval-to-manual-run connection exists or is claimed |
| 8   | Bounded parallel workflow     | Pass                 | reverse completion, partial failure, cancellation, ordering, attribution, and synthesis contracts               | Same-thread event multiplexing only                                                                       |
| 9   | Policy denial                 | Pass                 | `specialist_profiles_deny_current_tools_without_execution` and validation-rejection contracts                   | Unwired `NativeAgentRuntime`; real governed boundary; no executor                                         |
| 10  | Approval flow                 | Pass with advisory   | `agent_approval_source_resolution_retains_origin_without_execution`                                             | Simulated macOS result; typed task-bound denial; no automatic runtime-message injection                   |
| 11  | Cancellation                  | Pass                 | generic and bounded-parallel root cancellation plus pending-approval reconciliation contracts                   | Cooperative local cancellation; no remote worker                                                          |
| 12  | Failure recovery              | Pass                 | `child_failure_returns_typed_outcome_and_parent_can_synthesize_fallback` and workflow partial-failure contracts | Failure is simulated by the mock; application remains controlled                                          |

## 1. Personal Assistant direct response

Flow:

```text
Personal Assistant root -> fixture response -> completed root
```

The public orchestration contract starts one root and accepts one bounded
fixture response without requesting delegation. It verifies one task, one run,
no active child, the Personal Assistant identity, and the exact three-event
root lifecycle.

- Input: deterministic objective fixture.
- Runtime: `MockAgentRuntime` in success mode; descriptor identity `Native`.
- Governed tools: none proposed or run.
- External results: none, simulated or live.
- Effect: none.
- Evidence: `agent_orchestration_contract::personal_assistant_can_complete_directly_without_a_child`.

## 2. Research brief

Flow:

```text
Personal Assistant -> Research -> Knowledge & Document -> Personal synthesis
```

The application supplies the two closed sources `approach-a` and `approach-b`.
Research may cite only those catalog-issued IDs. Knowledge receives only the
validated Research result and known source catalog, and final synthesis retains
the exact source IDs and `fixture_based: true` disclosure.

The same suite forces Research failure, Knowledge failure, incomplete
knowledge, missing references, cancellation, invalid output, and synthesis
failure. A failed specialist is represented as a typed partial outcome; it is
never silently converted into complete evidence.

- Input: deterministic `WorkflowSourceCatalog`; no URLs or retrieval.
- Runtime: `MockAgentRuntime`; descriptor identity `Native`.
- Governed tools: none proposed or run.
- External results: simulated provider-unavailable failure in negative cases;
  no real provider call.
- Effect: none.
- Success evidence:
  `agent_research_knowledge_workflow_contract::deterministic_fixture_workflow_preserves_provenance_and_exact_order`.
- Partial evidence:
  `research_failure_skips_knowledge_and_allows_truthful_partial_synthesis`,
  `knowledge_failure_retains_validated_research_for_partial_synthesis`, and
  `missing_references_are_labeled_partial_and_never_invented`.

## 3. Document knowledge workflow

Flow:

```text
explicit synthetic approved .md file -> Knowledge & Document -> Personal synthesis
                                      -> shared-memory proposal -> application review
```

The contract creates a temporary synthetic Markdown file, registers it through
the real approved-document boundary, and requests the Knowledge & Document
operation with an exact grant. The reader opens only selected `.txt` or `.md`
content within the bounded approved root. The test submits a deterministic
reusable-knowledge value through the real Knowledge-context proposal API; it is
not parsed from the fixture runtime result and is not a stored fact.

Shared memory changes require a versioned application review receipt. Rejected,
stale, cross-domain, and unreviewed proposals do not become shared memory.
Task-temporary specialist memory is isolated and cleaned after terminalization.

- Input: temporary deterministic file content explicitly registered as a
  user-selected approved source.
- Runtime: `MockAgentRuntime`; descriptor identity `Native`.
- Real governed boundary: approved-document reader, document grant, memory
  profiles, proposal/review lifecycle.
- Governed tools: no tool-registry call and no executor.
- External results: real local read of only the temporary approved fixture;
  no production document or directory discovery.
- Effect: temporary test file creation/cleanup only. The proposal remains
  process-local `ProposedShared`; it is neither silently promoted to
  `ApprovedShared` nor durably persisted.
- Evidence:
  `agent_memory_document_contract::selected_markdown_and_approved_shared_memory_flow_only_to_knowledge_and_synthesis`,
  `four_memory_domains_require_versioned_application_review_and_redact_content`,
  and `specialist_memory_is_private_and_task_memory_is_cleaned_after_terminalization`.

## 4. Software engineering review

Flow:

```text
Personal Assistant -> Coding -> QA & Validation -> Security & Risk -> Personal synthesis
```

The repository catalog contains only immutable synthetic file records. Coding
returns an inert change proposal. QA maps each criterion to fixture evidence or
`not-run`. Security returns advisory risk data and cannot authorize. Final
synthesis is explicitly fixture-based, proposal-only, changes-not-applied, and
tests-not-executed.

- Input: synthetic `src/main.rs` and `Cargo.lock` records plus closed criteria
  and application evidence.
- Runtime: `MockAgentRuntime`; descriptor identity `Native`.
- Real governed boundary: workflow selection, strict parsing, provenance,
  stage attribution, policy denial, cancellation, audit.
- Governed tools: repository-like names are absent or denied; no executor.
- External results: none. Proposed tests remain `not-run`.
- Effect: none; no file edit, command, dependency operation, commit, or push.
- Evidence:
  `agent_engineering_quality_workflow_contract::orchestrated_success_is_sequential_bounded_attributed_and_proposal_only`,
  `specialist_governance_has_no_repository_executor_and_never_attempts_execution`,
  and `specialist_failures_preserve_only_validated_predecessors_for_partial_synthesis`.

## 5. Cloud infrastructure assessment

Flow:

```text
Personal Assistant -> Cloud Infrastructure -> QA -> Security -> Personal synthesis
```

The closed scenario `terraform-decision-brief-v1` supplies static synthetic
Terraform and Azure architecture text. Evidence explicitly marks Terraform and
provider checks `NotRun`. Capability requests for plan, apply, API, inventory,
credentials, or control-plane change remain denied data and cannot create an
approval or executor path.

- Input: deterministic built-in infrastructure catalog.
- Runtime: `MockAgentRuntime`; descriptor identity `Native`.
- Real governed boundary: catalog binding, structured stage validation,
  capability classification, attribution, cancellation, audit.
- Governed tools: none execute.
- External results: none; no Terraform binary, provider, cloud account, or API.
- Effect: none; no plan, apply, deploy, IAM, firewall, state, or resource
  mutation.
- Evidence:
  `agent_infrastructure_operations_workflow_contract::deterministic_cloud_workflow_is_sequential_attributed_fixture_only_and_inert`
  and `denied_cloud_capability_is_validated_partial_and_never_creates_execution_authority`.

## 6. Systems incident analysis

Flow:

```text
Personal Assistant -> Systems Operations -> QA -> Security -> Personal synthesis
```

The closed scenario `sanitized-service-recovery-v1` contains a synthetic
service snapshot, sanitized log excerpt, and recovery prompt. Findings must
distinguish observed fixture evidence from hypotheses. Proposed diagnostics and
recovery steps remain inert.

- Input: deterministic built-in systems catalog.
- Runtime: `MockAgentRuntime`; descriptor identity `Native`.
- Real governed boundary: catalog binding, structured validation, evidence
  classification, attribution, cancellation, audit.
- Governed tools: none execute.
- External results: none; no host, log service, shell, process, backup, VMware,
  or platform access.
- Effect: none; no restart, stop, reboot, kill, install, patch, configuration,
  account, or permission change.
- Evidence:
  `agent_infrastructure_operations_workflow_contract::deterministic_systems_workflow_preserves_evidence_hypotheses_and_no_effects`
  and `denied_systems_capability_is_partial_inert_and_does_not_create_approval`.

## 7. Workflow automation proposal and manual safe step

Flow A, non-executable checkpoint boundary:

```text
typed proposal with governed-tool/checkpoint values -> application validation -> rejected/no token
```

Flow B, bounded safe manual dispatch:

```text
complete A-D fixture proposal -> application validation -> take-once manual token
-> fresh AgentOrchestrator -> existing sealed fixture selector
```

Workflow Automation cannot create tasks, start a workflow, execute a tool, or
approve anything. Its canonical limit has zero executable tool steps. Known
governed-tool and approval-checkpoint values are recognized as untrusted
proposal data and remain non-executable; they cannot yield a manual token.

A separate complete A-D template can yield one process-local token, taken once
by application code and mapped only to the already implemented Research,
Engineering, Cloud, or Systems fixture selector. The document-to-action
template remains proposal-only. The Research bridge contract demonstrates a
manual dispatch to a fresh sealed selector and reports its complete or partial
fixture result.

This demonstration does not claim Flow A authorizes Flow B. Connecting them
would bypass the implemented boundary and is out of scope.

Under D-093, this demonstration passes with an advisory because the project
owner approved the two branches as separate acceptance evidence. The absent
combined chain remains explicit: proposal/checkpoint denial does not authorize
manual safe dispatch, and no approval-to-dispatch bridge exists or is implied.

- Input: one of five deterministic application-owned template definitions.
- Runtime: `MockAgentRuntime`; descriptor identity `Native`.
- Real governed boundary: strict proposal parser, template binding, limits,
  take-once token, destination selector checks.
- Governed tools: zero executable steps; no workflow-generated approval.
- External results: none.
- Effect: one local manual dispatch into an existing no-I/O fixture selector;
  no tool or device effect.
- Evidence:
  `agent_workflow_automation_contract::canonical_proposals_validate_for_a_through_e_with_application_derived_disposition`,
  `tool_and_checkpoint_steps_are_recognized_validated_and_never_executable`,
  `a_through_d_issue_one_take_once_token_and_map_only_to_existing_sealed_selectors`,
  and `manually_dispatched_research_updates_bridge_for_complete_and_partial_results`.

## 8. Bounded parallel workflow

Flow:

```text
Personal root -> two independent depth-one retained runs
              -> optional bounded dependent run
              -> catalog-ordered Personal synthesis
```

The three closed scenarios use default active limit two, hard active/child cap
three, depth one, at most four tasks and five runtime runs, zero retries,
per-run and aggregate event bounds, cooperative deadlines, and exact failure
policies. Independent work may complete in reverse order, but collection and
synthesis remain catalog-ordinal.

- Input: deterministic built-in parallel catalog.
- Runtime: `MockAgentRuntime`; distinct retained identities with descriptor
  identity `Native`.
- Real governed boundary: workflow admission, identity binding, ordering,
  cancellation, deadlines, memory isolation, attribution, audit.
- Governed tools: tool proposals fail only the proposing child and never enter
  governance.
- External results: simulated runtime failure/cancellation modes only.
- Effect: none.
- Evidence:
  `agent_bounded_parallelism_contract::independent_research_and_knowledge_complete_in_reverse_order_but_collect_by_ordinal`,
  `continue_partial_preserves_a_sibling_failure_in_personal_synthesis`,
  `cancel_dependent_only_keeps_sibling_live_and_skips_qa`,
  `fail_fast_cancels_the_other_specialist_skips_security_and_synthesizes_truthfully`,
  and `root_cancellation_is_child_first_and_leaves_no_live_or_dropped_run`.

## 9. Policy denial

Flow:

```text
specialist tool proposal -> schema/registry/profile/policy evaluation -> denied audit
-> execution NotAttempted
```

Every active specialist profile has no current tool eligibility. Unknown tool
names fail closed during validation. Known-but-ineligible proposals produce a
typed denial and an attributed volatile audit record. Neither path creates a
pending approval or attempts execution.

- Input: deterministic known and unknown tool proposals.
- Runtime: unwired `NativeAgentRuntime` wrapper; no provider/model call.
  Production governance code evaluates the proposal.
- Real governed boundary: local tool registry/schema, policy profile, policy
  engine, task attribution, volatile audit.
- Governed tools: proposal evaluated; none run.
- External results: none.
- Effect: none.
- Evidence:
  `agent_governance_contract::specialist_profiles_deny_current_tools_without_execution`
  and `validation_rejections_are_typed_audited_and_non_executing`.

## 10. Approval flow

Flow:

```text
Personal task -> approval-required proposal -> pending presentation
-> simulated Reject result -> task-bound typed resolution
-> ApprovalResolved / Rejected / NotAttempted audit
```

The macOS unit constructs `AgentOrchestrator::native()` and uses the real
governance, approval manager, presentation, decision-source adapter, resolution,
and audit paths. A deterministic `MessageDialogResult::Custom("Reject")` is
converted through the same typed source adapter without displaying a dialog.

The test verifies the pending subject is cleared, the originating Personal
task remains Running and controlled, the full task/root/parent/runtime/run/
profile/depth attribution and exact action/policy are retained, execution is
`NotAttempted`, and the audit record has no error.

This test is compiled and counted only on macOS. The recorded complete result
is from macOS 26.6 (build 25G72) on Apple silicon (`arm64`); a non-macOS run
omits this unit and does not by itself satisfy the complete twelve-row matrix.

- Input: deterministic `create_local_task` proposal with synthetic title.
- Runtime: `NativeAgentRuntime` wrapper, with no provider/model call.
- Real governed boundary: registry/schema, policy, approval manager,
  presentation, source resolution, task attribution, audit.
- Simulated external result: native dialog rejection result.
- Governed tools: no executor; no action runs.
- Effect: none.
- Advisory: the typed resolution returns to application code but is not
  automatically injected as runtime text and does not terminalize the task.
- Evidence:
  `agent::orchestrator::tests::agent_approval_source_resolution_retains_origin_without_execution`.

## 11. Cancellation

Flow:

```text
root cancellation -> child/pending-approval reconciliation -> retained-run cleanup
-> child terminal event -> root terminal event
```

Generic orchestration and each sealed workflow exercise cooperative
cancellation, retryable cancellation failure, already-terminal contradictions,
late-event rejection, and child-first cleanup. A private unit also seeds a
pending child approval and proves root cancellation resolves the pending
approval before child and root cancellation events.

- Input: deterministic task and runtime states.
- Runtime: `MockAgentRuntime` for workflow cases; `NativeAgentRuntime` wrapper
  for pending-approval reconciliation; no provider.
- Real governed boundary: task lifecycle, orchestrator, approval reconciliation,
  runtime-run ownership, audit.
- External results: simulated runtime cancellation outcomes.
- Effect: local in-memory task/run terminalization only.
- Evidence:
  `agent_orchestration_contract::root_cancellation_propagates_child_first_and_rejects_late_events`,
  `agent_bounded_parallelism_contract::root_cancellation_is_child_first_and_leaves_no_live_or_dropped_run`,
  and
  `agent::orchestrator::tests::root_cancellation_reconciles_child_pending_approval_before_child_and_root`.

## 12. Failure recovery

Flow:

```text
specialist/runtime failure -> typed attributed child outcome -> parent resumes
-> truthful partial/fallback synthesis
```

The generic contract emits a simulated provider timeout for Research, verifies
the child is terminal `Failed`, and lets Personal Assistant synthesize a bounded
fallback. The application does not crash or fabricate a completed specialist
result. Workflow suites repeat the pattern at each stage and retain only
validated predecessors.

- Input: deterministic objective and outputs.
- Runtime: `MockAgentRuntime` failure modes; descriptor identity `Native`.
- Real governed boundary: event validation, typed failure mapping, task state,
  parent resume, partial synthesis.
- External results: simulated provider/runtime failure only.
- Effect: none.
- Evidence:
  `agent_orchestration_contract::child_failure_returns_typed_outcome_and_parent_can_synthesize_fallback`,
  `agent_infrastructure_operations_workflow_contract::first_stage_runtime_failure_skips_specialists_and_returns_truthful_partial_synthesis`,
  and the Research, Engineering, Automation, and Parallel partial-failure
  contracts listed in the acceptance matrix.

## Captured-state policy

The accepted evidence is exact test output, structured task/workflow/audit
state, and the post-increment report. No new screenshot is tracked because the
backend architecture has no connected UI. A Command Center screenshot would
show only frontend fixture presentation and could not prove these Rust paths.
The separately completed real-browser/Tauri M5 evidence remains documented in
the Command Center plan and must not be relabeled as workflow execution.
