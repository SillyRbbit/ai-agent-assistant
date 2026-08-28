# Native multi-agent final architecture and security review

Date: 2026-08-28
Reviewed head: `181f85162e2b6bfb00c17dfdb7925eab2b92f3b9` (`origin/main`)
Review mode: Read-only; no remediation authority
Result: PASS WITH ADVISORIES

## Scope and method

This review reconciles D-079 and D-082 through D-093, the source-current Rust
agent contracts, sealed Research -> Knowledge lifecycle, Command Center,
Tauri capability/CSP, F-12 repository-health guards, manifests, and completed
increment evidence. The 2026-08-26 native-nine-agent review is historical
evidence only. Its F-01/F-02, F-07, F-08, F-12, and F-15 findings were
revalidated against later bounded implementation evidence and current checks.

No production source, test, dependency, configuration, capability, CSP,
permission, or historical record was changed. This report is not release,
provider, live-model, or target-Mac operational evidence.

## Current architecture and ownership

**Current.** Rust owns the immutable nine-role catalog, trusted task/context
lineage, orchestration, exact run identity, policy/approval boundary,
cancellation, volatile memory, and bounded audit attribution. `NativeAgentRuntime`
remains sole/default. The generic runtime/catalog surface is not caller-
selectable or generically wired to React/Tauri/provider transport.

**Current.** F-01 and F-02 are contained: public agent-orchestration contracts
prove foreign returned identities are rejected and nonterminal rejected runs
remain owned/quarantined until cleanup succeeds. Contradictory and permanent
cleanup-failure paths fail closed.

**Current.** Delegation remains application-owned, depth-one, finite, and
route-specific. D-086, D-087, D-088, D-090, and D-091 preserve exact sealed
fixture selectors, finite caps, cancellation ownership, and bounded journals.
Only `AgentOrchestrator` creates child tasks; specialists cannot spawn agents.

**Current.** D-090/D-093 retain two separate deterministic proofs: a
checkpoint/tool proposal stays non-executable and cannot issue dispatch, while
the A-D manual sealed-dispatch proof consumes an application-owned take-once
token. There is no approval-to-dispatch bridge.

## Data, privacy, and authority

**Current.** Agent memory and attribution remain bounded and volatile for the
sealed workflows. Content, opaque identities, raw errors, and fixture details
are redacted or excluded from public contracts. The repository has pre-existing
startup SQLite bootstrap and approved-document reader boundaries; neither is a
generic agent-memory, agent-UI, provider, or tool authority, and neither was
changed or widened by this review.

**Current.** There is no agent provider/model transport, credential, live
network, general tool execution, durable agent audit, background autonomy, or
device effect. Existing fixture workflows are deterministic proofs, not live
work execution. Hermes remains Deferred/Blocked; its opt-in real executable
probe is intentionally ignored and is not evidence of a selected transport.

**Prohibited.** Generic workflow execution, runtime selection, model-to-device
or WebView-to-device execution, provider connection, approval dispatch, and
unbounded storage/agent I/O remain outside the reviewed authority.

## WebView, IPC, and presentation

**Mocked.** The Command Center catalog, topology, task/progress presentation,
Conversations, and simulated activity are frontend fixtures. They are visibly
labelled `DEMO MODE · SIMULATED AGENT DATA` and do not authenticate or select a
trusted agent, task, run, runtime, fixture, script, stage, or outcome.

**Current.** The only lifecycle exception is one selected-scenario prop-free
panel over a process-local Rust host. The host owns all identities, fixtures,
and a private success/failure schedule. Four fixed commands accept no caller
arguments; the one lifecycle event is notification-only, and validated command
responses alone may commit presentation. Forged, stale, same-revision, or
malformed events fail closed or require explicit recovery.

**Current.** F-07/F-08/F-12 remain effective. Production CSP excludes the
development WebSocket and inline scripts; development adds only the fixed local
Vite WebSocket. The sole capability is `core:default`. Runtime narrowing uses
closed DTO parsers, and repository-health tests pin exact commands/events,
consumer locality, source digests, prohibited browser/Tauri surfaces, and CSP
configuration. F-15 documentation-truth checks pass.

## Threat and contract evidence

| Threat                                   | Result                           | Evidence                                                                 |
| ---------------------------------------- | -------------------------------- | ------------------------------------------------------------------------ |
| Foreign runtime/run identity             | Current containment              | F-01/F-02 orchestration and runtime contract coverage; acceptance passed |
| Rejected-run cleanup failure             | Current fail-closed ownership    | quarantine, retry, cancellation, and late-event tests passed             |
| Forged/stale WebView event               | Current fail-closed UI boundary  | lifecycle-client, panel, Tauri-contract, and F-12 tests passed           |
| CSP/capability broadening                | Current static protection        | paired CSP/capability/F-12 repository tests passed                       |
| Fixture mistaken for authority           | Mocked and disclosed             | Command Center/lifecycle tests and exact disclosure passed               |
| Tool, approval, or dispatch escalation   | Current denial                   | governance, Workflow Automation, and D-093 contract tests passed         |
| Provider/network/credential reachability | Prohibited/absent in agent scope | manifest/source inspection and acceptance contracts passed               |

## Historical-finding disposition

| Historical finding                | Source-current disposition                                    |
| --------------------------------- | ------------------------------------------------------------- |
| F-01 returned runtime identity    | Resolved by current exact identity contracts                  |
| F-02 rejected-run ownership       | Resolved by current quarantine/cleanup contracts              |
| F-07 production/dev CSP           | Resolved by paired static CSP guard and current configuration |
| F-08 app-info runtime narrowing   | Resolved by closed DTO client/parser coverage                 |
| F-12 UI/native static boundary    | Resolved by exact F-12 repository-health guards               |
| F-15 documentation reconciliation | Resolved by documentation-truth/repository-health checks      |

## Findings and advisories

No Critical, High, Medium, or Low current finding was found.

1. **Advisory — target-Mac/rendered evidence not run.** This read-only review
   did not require an app launch or rendered matrix. Existing target-Mac
   evidence remains historical with its stated raw-debug limitations. A future
   change affecting native behavior must acquire fresh target-Mac evidence.
2. **Advisory — Hermes real executable probe intentionally ignored.** Hermes is
   Deferred/Blocked and no transport is selected. The ignored opt-in probe does
   not weaken the native result or authorize activation.

## Conclusion and rollback

The completed native phases remain architecturally consistent with their
bounded, fixture-only, application-owned authority model. No remediation is
authorized or necessary from this review. Before commit, rollback only the
review/report and live-memory documents by inverse patch; after publication,
use a documentation-only revert PR. Never reset, rebase, or rewrite historical
evidence.
