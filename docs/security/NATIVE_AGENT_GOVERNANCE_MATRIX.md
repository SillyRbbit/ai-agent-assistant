# Native agent governance matrices

Status: Accepted D-084 implementation contract
Last updated: 2026-08-12

These matrices are deterministic application configuration, not permissions
granted by a model, role name, group, registry membership, or runtime. No entry
authorizes execution in this increment.

## Agent/profile matrix

| Agent                | Exact profile                   | Current tool eligibility                        | Delegation source | Operational state                          |
| -------------------- | ------------------------------- | ----------------------------------------------- | ----------------- | ------------------------------------------ |
| Personal Assistant   | `PersonalAssistantV1`           | `get_current_datetime@1`, `create_local_task@1` | Research only     | Catalog `Initial`; Rust-only/unwired       |
| Research             | `ResearchReadOnlyV1`            | None                                            | None              | Catalog `Initial`; only bounded child flow |
| Coding               | `CodingGovernedV1`              | None                                            | None              | Deferred                                   |
| Cloud Infrastructure | `CloudInfrastructureGovernedV1` | None                                            | None              | Deferred                                   |
| Systems Operations   | `SystemsOperationsGovernedV1`   | None                                            | None              | Deferred                                   |
| Knowledge & Document | `KnowledgeDocumentsV1`          | None                                            | None              | Deferred                                   |
| QA & Validation      | `QualityValidationAdvisoryV1`   | None                                            | None              | Deferred/advisory only                     |
| Security & Risk      | `SecurityRiskAdvisoryV1`        | None                                            | None              | Deferred/advisory only                     |
| Workflow Automation  | `WorkflowProposalOnlyV1`        | None                                            | None              | Deferred/proposal-only                     |

`Initial` is non-authorizing metadata. Personal Assistant's eligibility only
permits deterministic policy evaluation of existing schemas; it does not
permit dispatch. Research has no current research tool. Deferred roles cannot
enter governance because live task selection already fails closed.

## Tool-policy matrix

| Profile/tool                                       | Schema facts                           | Policy outcome    | Approval                                | Execution      |
| -------------------------------------------------- | -------------------------------------- | ----------------- | --------------------------------------- | -------------- |
| `PersonalAssistantV1` / `get_current_datetime@1`   | Information-only, no permission        | `Allow`           | `NotRequired`                           | `NotAttempted` |
| `PersonalAssistantV1` / `create_local_task@1`      | Reversible local action, no permission | `RequireApproval` | Pending then exact terminal disposition | `NotAttempted` |
| Any other profile / any current tool               | Profile-ineligible                     | `Deny`            | `NotRequired`                           | `NotAttempted` |
| Any profile / unknown tool                         | Validation rejected                    | Not evaluated     | `NotRequired`                           | `NotAttempted` |
| Any profile / wrong version or malformed arguments | Validation rejected                    | Not evaluated     | `NotRequired`                           | `NotAttempted` |

Approval is never execution authority. Approved, rejected, cancelled, and
expired resolutions all retain `NotAttempted`.

## Delegation matrix

| Source                                                   | Target               | Matrix result                                                   | Approval      | Control result                   |
| -------------------------------------------------------- | -------------------- | --------------------------------------------------------------- | ------------- | -------------------------------- |
| Personal Assistant root at depth 0                       | Research             | Allowed after registry/activation/state/depth/budget validation | `NotRequired` | `ChildCreated` or typed `Failed` |
| Personal Assistant                                       | Personal Assistant   | Denied                                                          | `NotRequired` | `NotCreated`                     |
| Personal Assistant                                       | Any other specialist | Audited denial or target Deferred after trusted attribution     | `NotRequired` | `NotCreated`                     |
| Research                                                 | Any target           | Audited denial: unauthorized source/depth                       | `NotRequired` | `NotCreated`                     |
| Any specialist                                           | Any target           | Audited denial                                                  | `NotRequired` | `NotCreated`                     |
| Workflow Automation                                      | Any target           | Denied; proposal-only                                           | `NotRequired` | `NotCreated`                     |
| Unknown, stale, foreign, mismatched, or terminal context | Any target           | Rejected before trusted attribution                             | None          | No mutation                      |

Only the orchestrator may create a child. Delegation is not a tool and does
not traverse tool schema, tool policy, or approval.

## Authority matrix

| Role/component                          | May propose                   | May create child          | May decide policy     | May approve            | May execute               | May write governance audit |
| --------------------------------------- | ----------------------------- | ------------------------- | --------------------- | ---------------------- | ------------------------- | -------------------------- |
| Model/runtime                           | Untrusted text/tool data only | No                        | No                    | No                     | No                        | No                         |
| Personal Assistant role                 | Bounded request data          | No                        | No                    | No                     | No                        | No                         |
| Specialist roles                        | Bounded advisory data         | No                        | No                    | No                     | No                        | No                         |
| Workflow Automation role                | Future typed proposal only    | No                        | No                    | No                     | No                        | No                         |
| `AgentOrchestrator`                     | Coordinates trusted calls     | Yes, exact allowlist only | No                    | No                     | No                        | Via service only           |
| `AgentGovernanceService`                | No                            | No                        | Sequences engine only | Sequences manager only | No                        | Yes, closed records only   |
| `PolicyEngine`                          | No                            | No                        | Yes                   | No                     | No                        | No                         |
| `ApprovalManager` / trusted user source | No                            | No                        | No                    | Yes                    | No                        | No                         |
| Executor / platform adapter             | Absent                        | No                        | No                    | No                     | No current implementation | No                         |

## Audit record matrix

| Record                      | Identity                                              | Policy facts   | Approval facts                      | Result                        | Content retained |
| --------------------------- | ----------------------------------------------------- | -------------- | ----------------------------------- | ----------------------------- | ---------------- |
| Tool validation rejected    | Full trusted attribution + call ID                    | `NotEvaluated` | `NotRequired`                       | `NotAttempted` + closed error | None             |
| Tool policy evaluated       | Full trusted attribution + call ID + known schema     | Outcome/reason | `NotRequired` or `Pending`          | `NotAttempted`                | None             |
| Tool approval resolved      | Same preallocated subject                             | Outcome/reason | Approved/Rejected/Cancelled/Expired | `NotAttempted`                | None             |
| Delegation denied           | Full trusted source attribution + exact closed target | Matrix denied  | `NotRequired`                       | `NotCreated` + closed error   | None             |
| Delegation allowed/finished | Same preallocated subject                             | Matrix allowed | `NotRequired`                       | ChildCreated or Failed        | None             |

Every record has a sequence and infallible logical created/updated tick. The
audit has exactly 32 subject slots, is volatile, and performs no I/O.
