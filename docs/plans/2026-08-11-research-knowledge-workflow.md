# Research and Knowledge workflow

Status: Verified complete with advisories; publication pending
Owner: Project owner
Last updated: 2026-08-12
Decision: D-086, preserving D-079 and D-082 through D-085
Gate ID: `agent-research-knowledge-workflow`
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Implement one deterministic, Rust-only Personal Assistant -> Research Agent ->
Knowledge & Document Agent -> Personal Assistant synthesis workflow above
`AgentRuntime`. Both specialists are sequential depth-one children of the same
Personal Assistant root. The workflow consumes only bounded
application-supplied deterministic fixtures, validates strict structured
specialist output, preserves known source references, and exposes truthful
partial results without adding a provider, retrieval tool, general workflow
engine, persistence, IPC, or UI.

## User-visible outcome

None in the shipping application. Deterministic Rust contracts can demonstrate
the workflow through `MockAgentRuntime` and can construct the unchanged
`NativeAgentRuntime` path. The result must state that its sources are fixtures;
it does not claim that live external research, retrieval, or factual
verification occurred.

## Scope

- A closed `ResearchKnowledgeWorkflowV1` request selected by trusted
  application code from a live Personal Assistant root.
- Closed framework-neutral Research and Knowledge result contracts parsed from
  strict JSON with unknown fields rejected.
- An application-owned source catalog containing one to eight deterministic
  fixtures. Runtime output may reference only catalog-issued opaque source IDs.
- Exactly one Research child followed, only after a structurally valid Research
  completion, by exactly one Knowledge child. The orchestrator, not Research,
  creates the Knowledge task.
- A final Personal Assistant synthesis input containing only validated
  structured results, exact stage status, and the trusted fixture disclosure.
- Content-free, bounded workflow events and volatile typed attribution records.
- Truthful partial synthesis when a stage fails, is cancelled, or returns
  incomplete source attribution.
- Existing task-local memory access while each specialist context is live. A
  Knowledge reusable-memory value remains a typed pending-review value in the
  workflow result and is never written to `MemoryStore` or promoted to approved
  shared memory automatically.
- Focused adversarial tests, full validation, current-state synchronization,
  an increment record, and a consolidated review.

## Explicit non-goals

- No general workflow engine, workflow DSL, scheduler, recursion, specialist
  spawning, parallel child, automatic retry, or dynamic route construction.
- No live search, browser, network, provider, model credential, filesystem
  discovery, new document read, tool proposal, tool execution, platform action,
  or Hermes integration.
- No activation of Coding, QA & Validation, Security & Risk, Cloud
  Infrastructure, Systems Operations, or Workflow Automation.
- No change to `AgentRuntime`, `NativeAgentRuntime`, runtime capabilities,
  Tauri commands, React, manifests, lockfiles, dependencies, capabilities,
  storage, SQLite, or ARB-005.
- No automatic access to documents, conversation history, another task's
  private or temporary memory, unselected shared memory, or sibling memory.
- No automatic permanent shared-memory write or approval. Model text cannot
  approve a proposal.
- No chain-of-thought, hidden-reasoning field, free-form citation, arbitrary
  URL, raw path, or claim that fixture data is live research.

## Published prerequisite and current behavior

- Before the D-086 planning diff, `main` was clean and synchronized at
  `5e53f55`; D-085's marker `agent-memory-approved-documents` is complete and
  valid for that published tree with PASS WITH ADVISORIES. The historical
  D-085 review remains unchanged. The current workspace is intentionally dirty
  under active gate `agent-research-knowledge-workflow`.
- D-083 currently permits one child, two tasks, and three runs. Every terminal
  child immediately resumes Personal synthesis.
- Generic delegation is exactly Personal Assistant -> Research. D-085 provides
  a separate Personal Assistant -> Knowledge approved-document task and leaves
  Research -> Knowledge denied.
- Research and Knowledge are catalog-eligible but have no retrieval or spawn
  authority. Runtime tool proposals remain rejected.
- D-085 provides workflow-local volatile memory and an approved `.txt`/`.md`
  reader. This increment reuses memory contracts but does not broaden document
  access or read new files.
- `MockAgentRuntime` is a deterministic no-I/O event sink. The test harness
  supplies closed runtime events; it does not simulate a live provider.

## Closed contracts and limits

### Source catalog

`ResearchKnowledgeWorkflowRequest` owns an immutable
`WorkflowSourceCatalog`. V1 supports only `DeterministicFixture` sources.
Each source has an application-issued canonical opaque `WorkflowSourceId`, a
bounded label, and bounded untrusted evidence content.

- sources: 1 through 8;
- ID: 1 through 64 ASCII characters from `[a-z0-9._-]`, starting with an
  alphanumeric character;
- label: 1 through 256 Unicode scalar values and 1,024 UTF-8 bytes;
- evidence: 1 through 4,096 Unicode scalar values and 8,192 UTF-8 bytes;
- aggregate source evidence: at most 8,192 bytes;
- aggregate application-serialized catalog, including IDs, labels, evidence,
  separators, and escaping: at most 16,384 bytes;
- duplicate IDs, control characters other than LF/TAB in evidence, invalid
  canonical form, empty values, and overflow fail before runtime or task
  mutation.

IDs are correlation keys, not proof of truth. Unknown or duplicate IDs in
runtime output fail closed. A finding with no source reference is accepted only
as explicitly incomplete; the application never invents a reference.

### Research result V1

Strict JSON carries zero to 16 findings, zero to four unresolved questions, zero
to four limitations, and one optional recommended follow-up. Those four
top-level properties are intentionally optional and default to their empty or
absent values; `{}` is therefore a valid, explicitly partial no-findings result
rather than a complete result. A finding contains
a statement of 1 through 1,024 Unicode scalar values and at most 4,096 UTF-8
bytes, zero to four known source IDs, and one closed confidence value: `high`,
`medium`, `low`, or `unknown`. Each unresolved question, limitation, and
follow-up contains 1 through 512 scalar values and at most 2,048 bytes. The
entire result JSON is bounded by the existing 8,192-character/16,384-byte
task-output limit. An empty findings array or a finding without a source
produces `PartialMissingSources`, not a complete result. Unknown fields,
unknown IDs, duplicate IDs, invalid confidence, or any field/count/text bound
failure produces a typed invalid-result stage failure; raw partial JSON is not
forwarded.

The application binds the parsed value to exact Research task/root/run
attribution. `ResearchResultVersion` is the closed enum value `V1`; result
identity is the exact Research `AgentTaskId` plus that version, so no arbitrary
result-ID string exists. Parsing uses `serde_json` with `deny_unknown_fields`,
requires one complete JSON value with no trailing content, preserves source-ID
order only for presentation, rejects duplicate references within a finding,
and accepts a result exactly once for the currently active Research run. No
reasoning field is accepted.

### Knowledge transformation V1

The fixed operation is `OrganizeResearchEvidence`. Strict JSON carries one to
eight thematic sections, zero to sixteen extracted facts, zero to four
contradictions, one summary, one optional reusable-knowledge proposal, and one
optional artifact outline. A section has a 1-to-128-scalar/512-byte heading, a
1-to-1,024-scalar/4,096-byte body, and zero to four source IDs. A fact or
contradiction has a 1-to-1,024-scalar/4,096-byte statement and zero to four
source IDs. Summary, reusable proposal, and outline are each 1 through 2,048
scalar values and at most 8,192 bytes when present. The entire JSON remains
within the existing 8,192-character/16,384-byte task-output limit. Every source
reference must be present in the validated Research result's source set.
Missing references or `incomplete: true` makes the result partial; an unknown
reference or invalid field/count/text bound fails the stage. The application
binds Knowledge to the exact predecessor Research result ID/version while both
tasks remain sibling children of the Personal root.

`KnowledgeResultVersion` is the closed enum value `V1`. Knowledge predecessor
identity is the exact Research `AgentTaskId` plus `ResearchResultVersion::V1`;
it is constructed by the orchestrator and is not parsed from model JSON.
Knowledge parsing also uses `deny_unknown_fields`, requires one complete JSON
value with no trailing content, rejects duplicate references within each item,
and accepts a result exactly once for the currently active Knowledge run.

Research-to-Knowledge input is rejected, never truncated, above 26,624 bytes:
at most 16,384 bytes of validated Research JSON, 8,192 bytes of selected
fixture provenance, and 2,048 bytes of application framing. Specialist task
output remains within the existing 8,192-character/16,384-byte bound.

The workflow objective is 1 through 2,048 scalar values and at most 8,192
bytes. Research runtime input is rejected above 26,624 bytes: at most 8,192
bytes of objective, 16,384 bytes of the complete serialized source catalog, and
2,048 bytes of framing.
Final synthesis input is rejected above 36,864 bytes: at most 16,384 bytes for
each validated specialist result or typed stage status plus 4,096 bytes of
fixture disclosure and application framing. No builder truncates to satisfy a
bound.

Final Personal output is a strict `FinalSynthesisResult` V1 JSON envelope with
exactly `version`, a 1-to-8,192-scalar/16,384-byte `answer`, `source_ids`,
`fixture_based: true`, and the stage-derived status `complete` or `partial`.
One or more catalog-issued source IDs are required when validated specialist
evidence exists; an empty list is allowed only when no validated stage source
exists. Missing, duplicate, unknown, or invented IDs, false fixture
disclosure, wrong status, URL-bearing or live-research-claiming answer text,
reasoning, malformed content, trailing content, and unknown fields fail the
root without creating a completed workflow result.

### Topology, runs, events, and retries

- root tasks per orchestrator: 1;
- total tasks: 3;
- total child creation budget: 2, never replenished;
- active children: 1;
- specialist depth: 1;
- successful runtime runs: 4 (initial Personal, Research, Knowledge, final
  Personal synthesis);
- runtime-event limit: existing 32;
- generic orchestration-event limit: existing 32;
- workflow events and workflow attribution records: at most 16 each;
- automatic retries: 0. Runtime retry metadata never creates a run or
  replenishes a budget.

Generic delegation retains its one-child limit. The expanded limits are usable
only after the sealed workflow has been selected before the initial Personal
run emits output.

## Lifecycle and invariants

1. Trusted application code starts a Personal root and selects
   `ResearchKnowledgeWorkflowV1` with a bounded fixture catalog.
2. The orchestrator revalidates the exact live root context, governance-pending
   state, registry identities/profiles, output-free initial run, task/run/event
   capacity, and closed Personal -> Research governance route before mutation.
3. It cancels only the initial Personal run, moves the root to
   `WaitingForChild`, creates a depth-one Research child, and emits
   `ResearchStarted`.
4. A valid Research completion is bound to its exact task/run and emits
   `ResearchCompleted`. Only then does the orchestrator create a new depth-one
   Knowledge sibling whose predecessor is the validated Research result. The
   root stays `WaitingForChild`; Research never creates or delegates to
   Knowledge.
5. A terminal Knowledge outcome emits `KnowledgeOrganizationCompleted` or a
   typed partial failure. The orchestrator returns both attributed stage
   statuses to the root, resumes it once, emits `SynthesisStarted`, and starts
   a fresh Personal synthesis run.
6. Final Personal completion emits `Completed` and exposes a bounded workflow
   result. No specialist content is treated as trusted instruction.

The workflow journal exposes exactly the closed variants
`ResearchStarted`, `ResearchCompleted`, `KnowledgeOrganizationStarted`,
`KnowledgeOrganizationCompleted`, `SynthesisStarted`, `PartialFailure`,
`Cancelled`, and `Completed`. It contains identity, stage, result quality, and
typed failure only; no objective, source label/content, finding, summary,
proposal, path, URL, output, or reasoning.

A separate bounded volatile `ResearchKnowledgeAuditRecord` binds sequence,
root/task/agent/runtime/run/request/profile/memory identity, predecessor task
where applicable, stage, and closed outcome. It is not a durable or generic
`AuditLogger` and grants no authority.

### Transition preflight and accounting

Every transition validates exact live task/run/request/sequence identity and
preflights all task, run, generic-event, workflow-event, workflow-audit, and
input/output capacity needed for that transition before cancelling a run or
mutating a task. A reserved audit record is committed infallibly after the
corresponding mutation. The workflow-event cardinality is:

| Branch                             | Ordered workflow events                                                                                                       |     Count |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------: |
| Success                            | ResearchStarted, ResearchCompleted, KnowledgeOrganizationStarted, KnowledgeOrganizationCompleted, SynthesisStarted, Completed |         6 |
| Research partial                   | ResearchStarted, PartialFailure(Research), SynthesisStarted, Completed                                                        |         4 |
| Knowledge partial                  | ResearchStarted, ResearchCompleted, KnowledgeOrganizationStarted, PartialFailure(Knowledge), SynthesisStarted, Completed      |         6 |
| Root cancellation before a child   | Cancelled                                                                                                                     |         1 |
| Root cancellation during Research  | ResearchStarted, Cancelled                                                                                                    |         2 |
| Root cancellation during Knowledge | ResearchStarted, ResearchCompleted, KnowledgeOrganizationStarted, Cancelled                                                   |         4 |
| Root cancellation during synthesis | Successful stage prefix, SynthesisStarted, Cancelled                                                                          | At most 6 |

The workflow reserves at most one matching attribution record per event; its
16-record bound covers every branch without reuse.

| Transition failure                                               | Required state after failure                                                                                                                                                                                              |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial preflight or Research request construction               | Root/run remain Running; no child, workflow event, audit record, or budget mutation                                                                                                                                       |
| Initial Personal run cancellation failure                        | Root/run remain Running; no child or workflow mutation; any already-terminal governance record remains truthfully terminal under D-084                                                                                    |
| Research runtime start failure after root enters WaitingForChild | Research becomes Failed, no active child/run remains, Research child/task/run budget stays consumed, one Research partial record/event is retained, then fallback synthesis starts; fallback-start failure fails the root |
| Research invalid/runtime failure/cancel                          | Research becomes terminal and task-temporary memory is cleaned; Knowledge is skipped; invalid raw output is absent; fallback synthesis starts                                                                             |
| Knowledge request/input/preflight failure before task mutation   | Validated Research is retained; no Knowledge task/budget is created; root starts partial synthesis or fails if that run cannot start                                                                                      |
| Knowledge runtime start failure after task creation              | Knowledge becomes Failed, no active child/run remains, both child/task/run budgets stay consumed, one Knowledge partial record/event is retained, then fallback synthesis starts; fallback-start failure fails the root   |
| Knowledge invalid/runtime failure/cancel                         | Knowledge becomes terminal and task-temporary memory is cleaned; validated Research remains; invalid raw output is absent; fallback synthesis starts                                                                      |
| Existing 32-event root cap is exhausted                          | Exact active child and root terminalize Failed with a typed partial record; no fallback run starts because the root cannot safely accept another runtime event                                                            |
| Root cancellation failure                                        | Pending governance and completed cancellation steps remain truthfully terminal; the still-live task/run remains retryable and no later stage starts                                                                       |
| Final synthesis start/runtime failure                            | Root becomes Failed; validated specialist records remain inspectable; no completed workflow result is fabricated                                                                                                          |

## Partial failure and cancellation

- Research runtime/start/structured-output failure or independent cancellation:
  a reserved Research child is terminalized with the exact failure, Knowledge
  is `SkippedResearchUnavailable`, and the Personal root resumes with a typed
  partial input. Invalid raw Research output is never forwarded. If fallback
  synthesis cannot start, the root fails.
- Knowledge runtime/start/structured-output failure or independent
  cancellation: a reserved Knowledge child is terminalized with the exact
  failure and Personal resumes with the validated Research result plus a typed
  Knowledge-unavailable status. Invalid raw Knowledge output is never
  forwarded. If fallback synthesis cannot start, the root fails.
- Missing source references or `incomplete: true`: the stage completes as
  partial and final synthesis is explicitly instructed not to claim a complete
  workflow.
- Final synthesis start/runtime failure: the root fails with a typed task
  failure; no result is fabricated.
- Exhausting the existing 32-event per-root hard cap terminalizes the active
  specialist and root instead of starting a fallback run that could not accept
  another event.
- Root cancellation: pending governance is cancelled first, then the active
  child run/task, then the root. No later specialist or fallback synthesis is
  started. The workflow emits one `Cancelled` event.
- Cancellation failure preserves the existing live state for retry; no child,
  result, event, or memory budget is silently replenished.
- Late, duplicate, foreign, cross-stage, wrong-run, wrong-request, or
  wrong-sequence events fail before state mutation.

## Memory and provenance rules

- Research and Knowledge may use their existing agent-private and
  task-temporary namespaces only through their exact live contexts.
- Task-temporary memory is removed at terminal cleanup. Private memory remains
  isolated inside the one volatile orchestrator and is never copied to the
  sibling.
- Workflow result transport is an explicit validated payload, never a memory
  read or promotion.
- An optional Knowledge `reusable_knowledge_proposal` remains a typed bounded
  `PendingReview` value in `KnowledgeResult`. V1 never writes it to
  `MemoryStore`, assigns a `SharedMemoryProposalId`, approves it, selects it,
  persists it, or includes it in synthesis as approved fact. A future explicit
  application review may separately copy it into D-085 `ProposedShared`; that
  action is outside this increment. The workflow contract proves both the
  pending value and the absence of automatic memory mutation.
- Final synthesis receives application-owned fixture disclosure and only known
  source IDs carried through validated Research and Knowledge results. It must
  label the evidence fixture-based and must not fabricate citations.

## Files expected to change

Production and contracts:

- `src-tauri/src/agent/research_knowledge.rs` (new)
- `src-tauri/src/agent/mod.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/definition.rs` for `KnowledgeDocumentV2`
- `src-tauri/tests/agent_research_knowledge_workflow_contract.rs` (new)
- `src-tauri/tests/agent_definition_registry_contract.rs` for the intentional
  Knowledge V2 source expectation
- `src-tauri/tests/agent_memory_document_contract.rs` for a wording-only
  correction that preserves the generic-delegation denial while recognizing
  the separately governed D-086 workflow route
- `src-tauri/tests/support/mock_agent_runtime.rs` for one additional
  deterministic dual-start-failure mode used only by the D-086 adversarial
  contract

Documentation and closeout:

- `DECISIONS.md`
- `ARCHITECTURE.md`
- `PRODUCT_REQUIREMENTS.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- this plan
- `docs/plans/2026-08-12-agent-memory-approved-documents.md` for one additive
  successor-state correction that preserves D-085's historical evidence
- `PLANS.md`
- `NEXT_STEPS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `CHANGELOG.md`
- `docs/increments/agent-research-knowledge-workflow.md` (new)
- `docs/reviews/2026-08-12-agent-research-knowledge-workflow-post-increment-review.md`
  (new)

`TROUBLESHOOTING_LOG.md` changes only for a new durable failure/resolution.
No other file is in scope without an explicit plan amendment.

Existing `agent_orchestration_contract`, `agent_governance_contract`,
`agent_runtime_contract`, and
`gateway_request_contract` are verification-only regression targets and are
not expected to change.

## Implementation milestones

- [x] Record D-086, reconcile the published D-085 prerequisite, pass a fresh
      readiness/architecture/security review, and begin the gate.
- [x] Implement closed bounded fixture-source, Research-result,
      Knowledge-result, quality, event, audit, and error contracts with unit
      boundary/redaction tests.
- [x] Add the sealed four-run orchestrator state machine while preserving
      generic delegation and the direct approved-document path.
- [x] Add deterministic happy-path, partial-failure, cancellation, memory,
      provenance, attribution, ordering, bound, and Native regression tests.
- [x] Run focused and complete validation, perform architecture/security/code
      and debt reviews, synchronize documentation, and finalize a valid marker.

## Test plan

- Exact successful four-run order and the six successful workflow events;
  `PartialFailure` and `Cancelled` occur only on their respective non-success
  paths.
- Both specialist tasks have the Personal root as parent, depth one, distinct
  task/run identities, one active child, and exact Research predecessor link.
- The deterministic “compare two technical approaches and create a structured
  decision brief” demo discloses fixture-only evidence.
- Research start/runtime/invalid-output/cancel paths skip Knowledge and permit
  partial Personal synthesis; Knowledge equivalents preserve only validated
  Research data; synthesis failure fails root.
- Research- or Knowledge-run start failure consumes that planned task/run/stage
  attempt, records one typed failed stage, removes any unusable run, and either
  starts fallback synthesis or terminalizes the root if synthesis cannot start.
  Task/run/event/audit capacity is preflighted before the initial Personal run
  is cancelled.
- Empty/missing source references produce partial status; unknown, duplicate,
  stale, replayed, cross-workflow, and wrong-version sources/results fail.
- Exact N/N+1 and Unicode byte/scalar checks for source count/IDs/text,
  findings/references/sections/facts/contradictions, stage output, aggregate
  Knowledge input, tasks, children, runs, events, and audit records.
- Generic Personal -> Knowledge, Research -> Knowledge, Research -> Research,
  Knowledge -> any, self, reverse, deferred, unknown, and third-child routes
  remain denied before allocation.
- Foreign/stale task/root/parent/agent/runtime/run/request/policy/memory and
  predecessor attribution fails without mutation.
- Root cancellation during initial, Research, Knowledge, and synthesis is
  child-first where applicable and starts no next stage; late events reject.
- Runtime retryable failure causes zero retries and no replenished budget.
- Research and Knowledge task-local memory works while live and is cleaned at
  terminal state; private/unreviewed/unselected memory is excluded; reusable
  knowledge remains a typed pending-review value with no automatic
  `MemoryStore` proposal or approval.
- Debug, errors, workflow events, audit records, and docs evidence exclude
  sentinel source content, labels, findings, summaries, proposals, paths,
  URLs, objective text, output, and reasoning.
- Existing agent definition/registry, governance, orchestration,
  memory/document, runtime, gateway, and `NativeAgentRuntime` contracts pass
  unchanged except the intentional Knowledge V2 expectation.

## Verification commands

```bash
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::research_knowledge::tests::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_research_knowledge_workflow_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py finalize --increment agent-research-knowledge-workflow --report docs/reviews/2026-08-12-agent-research-knowledge-workflow-post-increment-review.md
python3 .codex/hooks/post_increment_gate.py status
```

No manual app check is required because the change remains unwired Rust with no
Tauri/React/user-visible path. Local arm64 evidence does not replace later
target-runner portability evidence.

## Risks and stop conditions

- Stop if safe implementation requires a general workflow engine, generic
  delegation widening, `AgentRuntime`/`NativeAgentRuntime` changes, a live
  provider/retrieval/tool path, new dependency, unsafe code, filesystem or
  network access, persistence, IPC/UI, capability/permission, or another agent.
- Stop if a specialist can construct the next task, source IDs can be invented
  or remapped, raw invalid output reaches synthesis, content enters events or
  audit, approved memory is automatic, or cancellation can orphan a run/task.
- Stop if the fixed bounds cannot fit the existing runtime input/output limits
  without truncation.

## Rollback

Before publication, remove the new workflow module/contract and restore only
the touched workflow integration, Knowledge V1 expectation, D-086/current-state
documentation, and gate evidence. Preserve D-085 memory/documents, D-084
governance, D-083 Personal-to-Research, D-079 runtime, and every historical
report. After publication, use one bounded revert plus an additive superseding
decision; do not erase D-086. There is no data, migration, dependency,
credential, process, network, IPC, or external-state rollback.

## Decisions and discoveries

- D-086 authorizes this fixed sequence and supersedes only the one-child/
  three-run sequencing limit for this workflow.
- The current generic child completion path resumes Personal immediately, so a
  closed workflow state is required; widening generic delegation is forbidden.
- Fixture provenance is application-owned correlation evidence, not proof that
  the model performed live research or that the claims are true.
- Zero automatic retries is the exact bounded V1 policy.
- Terminal structured output is parsed and every fallible continuation input is
  prepared before the runtime accepts the terminal event. A later Knowledge or
  synthesis start failure therefore cannot reverse an accepted runtime event;
  the closed continuation-failure status records the exact outcome.
- The deterministic final synthesis fixture names both known source IDs. The
  application validates its strict V1 envelope against the same source catalog
  and retains it in the workflow result; arbitrary Personal prose cannot bypass
  that provenance/disclosure/status contract.

## Progress

- 2026-08-12: Published D-085 prerequisite, source inventory, baseline focused
  contracts, and gate-marker validity confirmed.
- 2026-08-12: Owner explicitly selected and authorized this implementation.
- 2026-08-12: D-086 and this exact Ready plan drafted before source edits.
- 2026-08-12: Readiness is Ready with advisories; architecture and security
  review found no unresolved authority blocker. Documentation/repository/
  security/diff checks passed and gate `agent-research-knowledge-workflow`
  began before production edits.
- 2026-08-12: Implemented the closed fixture/result contracts and sealed
  four-run orchestrator path. Focused contracts, strict Clippy, the all-target
  Rust suite, and complete repository verification passed after terminal
  preparation and combined continuation-start-failure regressions were added.
- 2026-08-12: Independent architecture, security, code-health, and debt review
  identified final-output provenance, lifecycle-coverage, and duplicate-path
  gaps. The implementation added a strict final synthesis contract, the missing
  adversarial lifecycle cases, and removed the shadow completion path before
  final validation. The remaining advisory is the private internal
  continuation-outcome representation.

## Acceptance criteria

- [x] Strict bounded Research and Knowledge contracts preserve only known
      application source IDs and expose no reasoning.
- [x] The deterministic fixture-only four-run workflow completes with exact
      sibling lineage, one active child, two non-replenishing children, and
      correct event/audit ordering.
- [x] Partial failures and cancellations are truthful, bounded, and orphan-free.
- [x] Task-local memory remains isolated and any reusable knowledge remains an
      unapproved proposal.
- [x] Existing Native/runtime, governance, direct document, and generic
      delegation behavior remains valid; no other agent is activated.
- [x] All declared checks and reviews pass and the completion marker is valid.

## Final results

Implemented and verified with `PASS WITH ADVISORIES`. The deterministic
fixture workflow performs the exact Personal Assistant -> Research ->
Knowledge & Document -> Personal synthesis sequence as sequential depth-one
sibling tasks above the unchanged `AgentRuntime`. Research and Knowledge
outputs are strict bounded structured values; only catalog-issued source IDs
survive into the Knowledge input, synthesis input, and retained workflow
result. Research/Knowledge failures, incomplete attribution, cancellation, and
continuation-start failures produce closed partial or terminal outcomes
without retries or orphaned work.

Focused D-086 units pass 12/12, orchestrator units pass 10/10, and the public
D-086 contract passes 18/18. All declared regression targets, strict Clippy,
all-target Rust, complete repository verification, documentation, repository,
security, diff, and session-end checks pass. Native remains sole/default and
the implementation remains unwired Rust with no provider, retrieval, network,
filesystem discovery, persistence, IPC, UI, dependency, or other-agent
activation. The accepted advisory is that the internal continuation-failure
suppression can later be represented by a more explicit typed outcome without
changing the closed public contract.

## Documentation updates

- [x] Architecture, requirements, security, roadmaps, direction, ADR, and
      assessment match observed implementation.
- [x] `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `PLANS.md`, and
      `CHANGELOG.md` match observed evidence.
- [x] Increment and consolidated post-increment review are complete.
- [x] `TROUBLESHOOTING_LOG.md` remains unchanged because no new durable issue
      was observed.
