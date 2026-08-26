# Native multi-agent acceptance matrix and test evidence

Status: Verified complete with advisories; marker complete and valid
Evidence date: 2026-08-25
Branch: `codex/native-multi-agent-end-to-end-demonstrations`
Baseline commit: `527f0f4bafc4e263221e1ca246bc3e49600e0a26`
Target evidence: macOS 26.6 (build 25G72), Apple silicon (`arm64`)
Related artifacts:
[`NATIVE_MULTI_AGENT_DEMONSTRATIONS.md`](NATIVE_MULTI_AGENT_DEMONSTRATIONS.md)
and [`NATIVE_MULTI_AGENT_FIXTURES.md`](NATIVE_MULTI_AGENT_FIXTURES.md)

## Canonical command

```bash
npm run test:agent-acceptance
```

The command executes the complete Rust library unit suite and exactly these ten
public contract binaries:

1. `agent_definition_registry_contract`
2. `agent_runtime_contract`
3. `agent_orchestration_contract`
4. `agent_governance_contract`
5. `agent_memory_document_contract`
6. `agent_research_knowledge_workflow_contract`
7. `agent_engineering_quality_workflow_contract`
8. `agent_infrastructure_operations_workflow_contract`
9. `agent_workflow_automation_contract`
10. `agent_bounded_parallelism_contract`

The tests perform no provider call, governed-tool execution, product mutation,
or consequential external operation. Cargo is locked but not offline, so a
cold local cache may fetch locked crates. Normal local build output and bounded
temporary-fixture file I/O occur.

## Exact focused result

The first post-edit run on 2026-08-25 exited zero:

| Test binary                                         |  Passed | Failed | Ignored |
| --------------------------------------------------- | ------: | -----: | ------: |
| Rust library units                                  |     249 |      0 |       0 |
| `agent_definition_registry_contract`                |       7 |      0 |       0 |
| `agent_runtime_contract`                            |      26 |      0 |       0 |
| `agent_orchestration_contract`                      |      22 |      0 |       0 |
| `agent_governance_contract`                         |      11 |      0 |       0 |
| `agent_memory_document_contract`                    |      10 |      0 |       0 |
| `agent_research_knowledge_workflow_contract`        |      18 |      0 |       0 |
| `agent_engineering_quality_workflow_contract`       |      20 |      0 |       0 |
| `agent_infrastructure_operations_workflow_contract` |      25 |      0 |       0 |
| `agent_workflow_automation_contract`                |      18 |      0 |       0 |
| `agent_bounded_parallelism_contract`                |      41 |      0 |       0 |
| **Total**                                           | **447** |  **0** |   **0** |

The pre-edit inventory ran the ten public contracts alone and passed 198/198.
The source edits added no new test case; they strengthened the existing macOS
approval unit within the 249 library units and the existing approved-document
contract within its ten-test binary. The 249/447 counts are target-macOS
evidence: the approval unit is `#[cfg(target_os = "macos")]`, so a non-macOS
run omits it and does not satisfy this complete matrix by itself.

## Required demonstration evidence

| #   | Requirement                         | Result             | Executable evidence                                                                                                       |
| --- | ----------------------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| 1   | Personal direct response            | Pass               | `personal_assistant_can_complete_directly_without_a_child`                                                                |
| 2   | Research brief/provenance/partial   | Pass               | `deterministic_fixture_workflow_preserves_provenance_and_exact_order`; Research/Knowledge partial-failure tests           |
| 3   | Approved document/shared proposal   | Pass               | `selected_markdown_and_approved_shared_memory_flow_only_to_knowledge_and_synthesis`; versioned shared-memory review tests |
| 4   | Engineering/QA/Security synthesis   | Pass               | `orchestrated_success_is_sequential_bounded_attributed_and_proposal_only`; specialist failure/governance tests            |
| 5   | Cloud/QA/Security synthesis         | Pass               | `deterministic_cloud_workflow_is_sequential_attributed_fixture_only_and_inert`; denied cloud capability test              |
| 6   | Systems/QA/Security synthesis       | Pass               | `deterministic_systems_workflow_preserves_evidence_hypotheses_and_no_effects`; denied systems capability test             |
| 7   | Automation proposal/manual safe run | Pass with advisory | strict A-E validation; non-executable checkpoint; A-D take-once mapping; manually dispatched Research bridge              |
| 8   | Bounded parallel behavior           | Pass               | reverse ordering, attribution, partial failure, all failure policies, cancellation, and synthesis contracts               |
| 9   | Policy denial                       | Pass               | `specialist_profiles_deny_current_tools_without_execution`; `validation_rejections_are_typed_audited_and_non_executing`   |
| 10  | Approval denial/no action/audit     | Pass with advisory | strengthened `agent_approval_source_resolution_retains_origin_without_execution`                                          |
| 11  | Root cancellation/child cleanup     | Pass               | generic and D-091 root cancellation plus private pending-approval reconciliation                                          |
| 12  | Specialist failure recovery         | Pass               | `child_failure_returns_typed_outcome_and_parent_can_synthesize_fallback` plus sealed-workflow partial synthesis contracts |

## Architecture and governance acceptance matrix

| Acceptance criterion                       | Result | Evidence                                                                                                                                                                                        |
| ------------------------------------------ | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| All nine definitions registered            | Pass   | `built_in_registry_contains_exact_nine_definition_contract`; `AgentId::ALL`; repeated construction equality                                                                                     |
| Availability/activation metadata accurate  | Pass   | `catalog_activation_is_exact_descriptive_metadata`; all nine are `Initial` metadata only; runtime availability/health/capability contradiction contracts                                        |
| No specialist can spawn another directly   | Pass   | Library `delegation_matrix_allows_only_personal_to_research` exhausts all 81 source/target pairs; generic and selector-specific public denial tests prove no mutation                           |
| `AgentOrchestrator` owns task creation     | Pass   | Task constructors/transition methods remain crate-owned; public orchestration and workflow tests create every root/child through `AgentOrchestrator`; no new task-creation interface in diff    |
| Workflow Automation cannot execute         | Pass   | `MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS == 0`; known/unknown tool and checkpoint tests; take-once tokens map only through application-owned manual dispatch                                         |
| QA cannot approve                          | Pass   | QA definition denies approval authority; specialist governance tests deny current tools and create no pending approval/execution                                                                |
| Security cannot authorize                  | Pass   | Security definition and strict risk schema require `authorization_granted: false`; governance tests provide no policy/approval/execution authority                                              |
| Unknown tools fail closed                  | Pass   | Governance validation rejection and workflow unknown-tool tests produce typed rejection/audit or proposal failure, never pending approval/execution                                             |
| Task and agent identity cannot be forged   | Pass   | stale/foreign/cross-workflow context tests; returned identity mismatch; cross-run envelope; approval cross-manager/identity substitution; task/root/run/request binding                         |
| Memory boundaries hold                     | Pass   | four domains, profile mapping, task isolation/cleanup, versioned shared review, cross-workflow opaque-ID rejection                                                                              |
| Document boundaries hold                   | Pass   | explicit registration/grant, `.txt`/`.md` only, root/path validation, revocation, non-enumeration, cross-workflow identity, cancellation cleanup                                                |
| Destructive cloud actions remain blocked   | Pass   | closed capability classifier, `NotRun` Terraform/provider evidence, no executor/tool/provider/API, denial tests                                                                                 |
| Destructive systems actions remain blocked | Pass   | closed capability classifier, inert diagnostics, no shell/service/platform executor, denial tests                                                                                               |
| Audit attribution complete                 | Pass   | task/agent/root/parent/runtime/run identity in governance/workflow records; strict bounded counts and debug redaction; approval rejection now asserts exact originating task/root and lifecycle |
| Native remains default                     | Pass   | runtime descriptor and wrapper contracts; every workflow context reports `RuntimeId::Native`; no runtime selector/fallback/provider added                                                       |
| Hermes remains deferred                    | Pass   | `RuntimeId` contains only `Native`; no `HermesAgentRuntime`, dependency, process, adapter, selector, or transport change; existing NO-GO/Deferred records unchanged                             |

## Governance evidence by authority

| Authority                     | Application owner/evidence                                          | Agent/runtime authority                                    |
| ----------------------------- | ------------------------------------------------------------------- | ---------------------------------------------------------- |
| Task creation and transitions | `AgentOrchestrator`                                                 | None                                                       |
| Agent registry/definition     | Closed `AgentRegistry`/`AgentDefinition`                            | Descriptive data only                                      |
| Tool schema and registration  | Local `ToolRegistry` and schema validators                          | Proposal data only                                         |
| Policy                        | Application policy engine plus exact agent policy profile           | No agent may change policy                                 |
| Approval                      | Application approval manager and trusted source-resolution adapter  | Personal may explain a presentation; no agent approves     |
| Execution                     | No native-agent executor exists                                     | None; all dispositions `NotAttempted`                      |
| Audit                         | Bounded application-owned process-local records                     | No agent may write or authorize audit                      |
| Memory/document               | Exact profile, grant, proposal/review, and approved-root boundaries | Only the application-supplied task/document/memory context |
| Provider/device/network       | Not configured or implemented for this architecture                 | None                                                       |

## Failure and cancellation evidence

- Generic child failure is typed and attributed; the parent resumes and can
  synthesize fallback content.
- Each sequential workflow preserves only validated predecessor output and
  marks missing stages partial or unavailable.
- D-091 exercises `ContinuePartial`, `CancelDependentOnly`, and `FailFast`,
  including retained cancellation failures and eventual cleanup.
- Root cancellation is child-first and rejects late events.
- Pending approvals are reconciled before task/run cancellation. A failed
  approval cancellation is atomic and retryable.
- Start failures and identity contradictions do not leave falsely completed
  tasks or unowned live runs.

## Advisory gaps that remain intentionally open

1. **No configured native provider or live model.** The suite proves the
   application-owned architecture with `MockAgentRuntime` fixture responses and
   the unwired `NativeAgentRuntime` wrapper.
2. **No real governed tool execution.** Policy and approval are real boundaries;
   every execution disposition is `NotAttempted` because no executor exists.
3. **Workflow Automation has no combined approval-to-dispatch chain.** Under
   D-093, checkpoint denial and safe A-D manual dispatch pass as separate
   owner-approved branches. The canonical executable tool-step limit remains
   zero; an approval does not authorize dispatch and no bridge is claimed.
4. **Approval rejection is not automatically injected into runtime text.** The
   application receives a typed task-bound rejection, clears pending state,
   audits it, and leaves the root task Running. Feedback/terminalization would
   be a new separately planned behavior.
5. **Audit is volatile and workflow-specific.** The records are complete within
   their bounded process-local contract, not durable operational audit.
6. **Parallelism is same-thread multiplexing.** No provider, OS-thread, worker,
   or distributed concurrency is claimed.
7. **No connected backend UI.** The Command Center remains a separate frontend
   fixture projection; screenshots cannot prove these Rust paths.
8. **Hermes remains deferred/blocked.** Nothing in this suite activates or
   integrates it.

The executable suite proves all twelve demonstrations under the
owner-approved acceptance scope. Demo 7 carries the explicit advisory that its
two proved branches are separate and do not form a combined chain. The gaps
block claims of live external operation, production readiness, executable
automation, durable audit, or an end-user backend-connected command center.

## Completion review

- Final-source `npm run test:agent-acceptance`: 447 passed, 0 failed,
  0 ignored.
- Final-source `npm run verify`: passed formatting, repository health, strict
  frontend/Rust lint, 28 hook tests, 38 repository-workflow tests, 211 frontend
  tests, 249 Rust library tests, 232 Rust integration tests with one intentional
  Hermes probe ignored, typecheck, frontend production build, and Tauri release
  no-bundle build.
- Independent workflow/governance review: the implemented trust boundaries
  pass. D-093 supersedes the initial completion-blocker classification by
  accepting Demo 7's two branches separately and retaining the absent chain as
  an advisory.

The consolidated
[`post-increment review`](../reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md)
records the fresh final documentation/session-end evidence and `PASS WITH
ADVISORIES` result. The gate marker is complete and fingerprint-valid.
