# Native multi-agent fixture catalog

Status: Deterministic test-fixture documentation
Last validated: 2026-08-25
Related artifacts:
[`NATIVE_MULTI_AGENT_DEMONSTRATIONS.md`](NATIVE_MULTI_AGENT_DEMONSTRATIONS.md)
and [`NATIVE_MULTI_AGENT_ACCEPTANCE.md`](NATIVE_MULTI_AGENT_ACCEPTANCE.md)

## Fixture rules

Every fixture in the native multi-agent acceptance suite is synthetic,
repository-owned, bounded, and non-authorizing.

- Fixture content is not live provider, web, repository, cloud, host, service,
  credential, or production-document evidence.
- Fixture IDs are application-issued provenance identifiers. Model/runtime text
  may reference only IDs already present in the selected catalog.
- Fixture output is parsed by strict typed contracts. Unknown fields,
  references, identities, authority claims, and over-limit values fail closed.
- Debug representations redact objectives, content, opaque task/run identities,
  arguments, and other untrusted text where the contract requires it.
- No fixture value can grant task creation, tool, policy, approval, execution,
  memory, audit, provider, or device authority.

## Runtime fixtures

Public workflow contracts use the shared test-only
`src-tauri/tests/support/mock_agent_runtime.rs` implementation.

`MockAgentRuntime` can deterministically represent success, unavailable or
unhealthy runtime state, start/event/cancel failure, bounded ordinal failures,
returned identity mismatch, duplicate live identity, already-terminal cancel,
and capability contradiction. It records only redacted start/cancellation and
run-lifecycle evidence.

The mock descriptor intentionally reports `RuntimeId::Native`, because `Native`
is the only closed runtime identity. This does not make the mock a configured
provider. `NativeAgentRuntime` remains the sole/default application wrapper and
has no configured provider or live model.

## Direct and generic orchestration fixtures

The generic orchestration contract uses bounded text constants for a root
objective, one Research objective, one Research result, and Personal synthesis.
Runtime events are constructed with the exact live task/run identity and closed
sequence. Negative cases substitute foreign, stale, out-of-sequence, oversized,
tool-proposal, cancellation, and typed failure events.

These values prove task state and orchestration contracts only. They do not
represent a live user exchange or provider response.

## Research and Knowledge fixtures

The Research/Knowledge catalog contains exactly two deterministic sources in
the primary demonstration:

| Source ID    | Label              | Synthetic observation                                           |
| ------------ | ------------------ | --------------------------------------------------------------- |
| `approach-a` | Approach A fixture | Lower operational complexity and cost in the bounded comparison |
| `approach-b` | Approach B fixture | Greater scale potential with greater operational complexity     |

Strict fixture strings represent Research findings, Knowledge sections/facts,
and final Personal synthesis. Each stage must preserve only known source IDs.
Negative fixtures cover missing known references, unknown references,
incomplete output, outer whitespace, authority claims, cancellation, and
runtime failure.

No URL, browser, search, retrieval tool, filesystem discovery, or network
operation exists in this catalog.

## Approved-document and memory fixtures

Document contracts create temporary synthetic `.txt` or `.md` files and have
the test harness explicitly register the file/root as user-selected through the
real bounded approved-document reader. The tests exercise allowed content,
unsupported format/path escape, revocation, cross-workflow identity, capacity,
cancellation, and cleanup.

The document is a real local temporary file but its content is deterministic
fixture data. The test does not read any user or production document. Temporary
directories clean up after the test.

Memory fixtures use bounded synthetic text in four application-owned domains:
Personal, specialist-private, task-temporary, and proposal-only shared memory.
The live Knowledge context submits a deterministic application test value
through the real proposal API; it is not parsed from `KNOWLEDGE_RESULT`. A
shared-memory proposal requires an exact versioned application review. The
fixture never silently promotes a proposal to approved shared memory and never
durably persists it.

## Engineering review fixtures

The principal engineering workflow request contains:

| Kind                 | ID                | Synthetic content or meaning                              |
| -------------------- | ----------------- | --------------------------------------------------------- |
| Repository file      | `src-main`        | `src/main.rs` record whose `answer()` fixture returns 41  |
| Dependency file      | `dependency-lock` | Synthetic `Cargo.lock` record for one fake package        |
| Acceptance criterion | `returns-42`      | Propose returning 42 without applying repository mutation |
| Observed evidence    | `obs-bug`         | The supplied source fixture contains 41                   |
| Observed evidence    | `obs-dependency`  | The lock fixture identifies one synthetic dependency      |
| Proposed check       | `proposed-check`  | A unit check is proposed and explicitly marked `NotRun`   |

Closed JSON fixture strings represent the Coding change proposal, QA report,
Security assessment, complete synthesis, and partial syntheses. The fixture
catalog includes a non-secret sentinel used only to verify content redaction;
the acceptance documentation does not reproduce it.

No working-tree read, filesystem edit, test/formatter/shell/package-manager
run, dependency installation, Git operation, network request, credential read,
commit, or push occurs.

## Cloud infrastructure fixtures

Closed scenario: `terraform-decision-brief-v1`.

| Kind      | ID                            | Synthetic content or meaning                                   |
| --------- | ----------------------------- | -------------------------------------------------------------- |
| Fixture   | `terraform-configuration`     | Static Terraform-like configuration with a public-ingress flag |
| Fixture   | `azure-architecture`          | Static architecture note with an internet-facing endpoint      |
| Criterion | `criterion-static-review`     | Identify fixture-supported risk and limitations                |
| Criterion | `criterion-no-execution`      | State Terraform/provider/external checks were not run          |
| Evidence  | `evidence-static-observation` | Application supplied the two synthetic fixtures                |
| Not-run   | `evidence-terraform-not-run`  | Terraform and provider checks explicitly did not run           |

The catalog never invokes Terraform, a cloud CLI/API/provider, state backend,
inventory, credentials, or resource mutation.

## Systems operations fixtures

Closed scenario: `sanitized-service-recovery-v1`.

| Kind      | ID                                 | Synthetic content or meaning                                 |
| --------- | ---------------------------------- | ------------------------------------------------------------ |
| Fixture   | `service-snapshot`                 | Inactive synthetic service after fixture-only start attempts |
| Fixture   | `sanitized-log`                    | Dependency-timeout log with no host/account identity         |
| Fixture   | `recovery-scenario`                | Analyze recovery without restart or configuration change     |
| Criterion | `criterion-diagnostic`             | Separate observed fixture findings from hypotheses           |
| Criterion | `criterion-safe-remediation`       | Propose inert diagnostics/remediation                        |
| Evidence  | `evidence-service-log-observation` | Application supplied the service and log fixtures            |

No live host, service manager, process table, log provider, shell,
PowerShell, VMware, backup system, credential, network, or system mutation is
used.

## Workflow Automation templates

The application-owned catalog has five immutable template IDs:

| Template ID                    | Closed proposal route                                  | Manual dispatch availability                   |
| ------------------------------ | ------------------------------------------------------ | ---------------------------------------------- |
| `research-brief-v1`            | Research -> Knowledge -> Personal synthesis            | Take-once mapping to sealed Research/Knowledge |
| `code-quality-review-v1`       | Coding -> QA -> Security -> Personal synthesis         | Take-once mapping to sealed Engineering        |
| `infrastructure-assessment-v1` | Cloud -> QA -> Security -> Personal synthesis          | Take-once mapping to sealed Cloud              |
| `systems-incident-analysis-v1` | Systems -> QA -> Security -> Personal synthesis        | Take-once mapping to sealed Systems            |
| `document-to-action-plan-v1`   | Knowledge -> Workflow Automation -> Personal synthesis | Unavailable; remains proposal-only             |

Canonical limits are four steps, three agent-task steps, zero executable tool
steps, 120 seconds, zero retries, and zero nested workflows. Fixtures containing
known governed-tool or approval-checkpoint step shapes are validation probes;
they remain non-executable and cannot yield dispatch authority.

Manual dispatch is an explicit application take-once operation into a fresh
existing fixture selector. It is not tool execution, workflow self-dispatch,
approval, scheduling, persistence, or external effect.

## Bounded-parallel fixtures

The built-in catalog has three scenario IDs:

| Scenario ID                         | Initial work           | Dependent work        | Failure policy        |
| ----------------------------------- | ---------------------- | --------------------- | --------------------- |
| `research-knowledge-independent-v1` | Research and Knowledge | None before synthesis | `ContinuePartial`     |
| `code-security-qa-v1`               | Coding and Security    | QA                    | `CancelDependentOnly` |
| `cloud-systems-security-v1`         | Cloud and Systems      | Security              | `FailFast`            |

The fixture IDs cover approach comparisons, decision context/format, an inert
code proposal, a threat boundary, a QA criterion, a cloud facet, a sanitized
systems facet, and a Security criterion. Each work item owns exact fixture IDs
and may not cite another item's undisclosed content.

Default active children are two; hard active and total child caps are three;
depth is one; root tasks cap at four; runtime runs cap at five; retries are
zero. Reverse completion, failure, cancellation, deadlines, dependent skips,
identity substitution, and canonical collection are all deterministic test
modes.

## Governance and approval fixtures

Governance tests construct bounded known-tool and unknown-tool proposals.
Production `ToolRegistry`, schema validation, per-agent policy profiles,
approval manager, task attribution, and volatile audit code evaluate them.
There is no tool executor.

The approval demonstration uses a synthetic `create_local_task` title and a
simulated macOS dialog result. `Approve` and `Reject` are test outcomes passed
through the real typed decision-source adapter. Both resolve with execution
`NotAttempted`; no proposed local task or external action is created. The
orchestrator's existing root task remains controlled.

## Data hygiene and reproducibility

- No fixture contains a real credential, token, personal production datum,
  hostname, account identity, cloud subscription, or private repository data.
- Catalogs are constructed from source constants and closed enums. They require
  no download, network, service, database, or environment variable.
- Repeated fresh construction is tested for stable ordering and equality.
- Test-generated temporary files are scoped to temporary directories and are
  cleaned by their owners.
- The acceptance command does not install dependencies or update lockfiles.
