# Volatile agent memory and approved-document Knowledge boundary

Status: Verified complete with advisories; publication pending
Owner: Project owner
Last updated: 2026-08-12
Decision: D-085, preserving D-084, D-083, D-082, D-079, and D-033
Gate ID: `agent-memory-approved-documents`
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Add a new bounded, namespace-aware, volatile application `MemoryStore`; add a
narrow approved-document reader; and enable one explicit Personal Assistant to
Knowledge & Document task over selected UTF-8 text or Markdown without adding
durable storage, unrestricted file access, a provider, or the full
Research-to-Knowledge workflow.

## User-visible outcome

None in the shipping application. The Rust core and deterministic mock-runtime
contracts can prove explicit memory isolation, shared-memory review, selected
document containment, and one Knowledge task. There is no Tauri command, file
picker, React surface, provider, live model, background indexing, or persistence.

## Scope

- A new bounded `MemoryStore` whose four closed domains are approved shared,
  agent-private, task-temporary, and proposed shared memory.
- An exact memory-access profile captured definition -> task -> execution
  context -> live attribution.
- Explicit selected-record context assembly, domain review/edit/approve/reject,
  deletion, task cleanup, and a fail-closed disabled mode.
- A new `ApprovedDocumentReader` with opaque task-bound references for trusted
  selected files, task attachments, approved-root members, and generated
  artifacts.
- Exact UTF-8 `.txt`/`.md` support, bounded reads, path containment, symlink and
  replacement checks, revocation, cleanup, and redacted errors.
- A separate document-task route from a live Personal Assistant root to one
  Knowledge child, still depth one, one total child, and one active child.
- Knowledge catalog eligibility and exact truthful document capabilities.
- Focused adversarial tests, existing runtime/orchestration/storage regressions,
  security/privacy docs, current-state sync, increment record, and review.

## Explicit non-goals

- No restoration of D-033's deleted arbitrary-content memory interface.
- No SQLite product-data table, migration, startup/storage behavior, database
  key, durable memory, restart recovery, backup, export, sync, or ARB-005 claim.
- No vector database, embedding, semantic index, full-text search, ranking,
  retrieval dependency, external service, or new Cargo/npm dependency.
- No PDF, DOCX, RTF, HTML, image, archive, OCR, MIME inference, or artifact
  generation.
- No file picker, Tauri IPC, React UI, filesystem plugin, capability, permission,
  directory crawl, generic file tool, write/delete/rename file operation, or
  unrestricted platform adapter.
- No Research-to-Knowledge workflow, specialist spawning, recursion, parallel
  children, additional workflow, or activation of the other six deferred roles.
- No change to `AgentRuntime`, `NativeAgentRuntime`, runtime capabilities,
  provider, tool proposal lane, executor, Hermes, policy/tool schemas, or device
  authority.
- No automatic context copying from shared memory, parent history, documents,
  sibling results, or another agent's records.

## Existing behavior and constraints

- Clean synchronized `main` is published at `2687294`; gate
  `agent-governance` is complete and fingerprint-valid with PASS WITH
  ADVISORIES.
- D-033 deleted the unsafe legacy `memory` scaffold. Production Rust currently
  exports no memory module, repository, retention control, or context selector.
- SQLite stores only migration history and `app_initialized`; debug uses an
  application-local file and release uses in-memory storage. D-010/D-012 and
  ARB-005 forbid product-data persistence without reviewed key management and
  lifecycle decisions.
- React conversations and context provenance are volatile; saved memory is
  always reported not used and the Memory page is a placeholder.
- `AgentExecutionContext` binds agent, policy, task/root/parent, runtime, depth,
  and run/request identity but has no memory profile.
- Knowledge is `Deferred(KnowledgeMemory)`. Generic delegation allows only
  Personal Assistant -> Research, and child input is currently specialized for
  that route.
- No document reader, file tool, attachment store, approved-root registry,
  platform file adapter, or Tauri filesystem permission exists. Current tool
  schemas are only date/time and local-task creation.

## Current-state evidence

- Before this planning diff, `git status --short --branch` showed clean
  synchronized `main` at `2687294` and the `agent-governance` marker was
  complete, fingerprint-valid, and PASS WITH ADVISORIES. The marker now reports
  `valid: false` only because D-085 and this uncommitted successor plan changed
  the workspace; the historical governance report is not rewritten.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked`:
  6 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked`:
  10 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked`:
  22 passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test storage_smoke --locked`:
  1 passed.
- Source inventory confirms no current `MemoryStore` or document boundary.
- The owner explicitly selected and authorized this implementation on
  2026-08-12; D-085 records the narrowed volatile interpretation.

## Files expected to change

Production and contracts:

- `src-tauri/src/memory.rs` (new)
- `src-tauri/src/documents.rs` (new)
- `src-tauri/src/lib.rs`
- `src-tauri/src/agent/definition.rs`
- `src-tauri/src/agent/governance.rs`
- `src-tauri/src/agent/orchestrator.rs`
- `src-tauri/src/agent/task.rs`
- `src-tauri/tests/agent_definition_registry_contract.rs`
- `src-tauri/tests/agent_governance_contract.rs`
- `src-tauri/tests/agent_orchestration_contract.rs`
- `src-tauri/tests/agent_memory_document_contract.rs` (new)

Decision, architecture, privacy, roadmap, plan, and closeout:

- `DECISIONS.md`
- `ARCHITECTURE.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `PRODUCT_REQUIREMENTS.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/security/AGENT_MEMORY_DOCUMENT_PRIVACY.md` (new)
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `docs/plans/2026-08-11-agent-memory.md`
- `docs/plans/2026-08-11-knowledge-document-boundaries.md`
- `docs/plans/2026-08-12-agent-memory-approved-documents.md`
- `PLANS.md`
- `NEXT_STEPS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `CHANGELOG.md`
- `docs/increments/agent-memory-approved-documents.md` (new)
- `docs/reviews/2026-08-12-agent-memory-approved-documents-post-increment-review.md`
  (new)

`TROUBLESHOOTING_LOG.md` and `src-tauri/tests/agent_runtime_contract.rs` are
explicitly excluded: no durable troubleshooting result exists, and the
unchanged runtime regression is run rather than edited. `PRODUCT_REQUIREMENTS.md`
receives only a factual current-baseline distinction between the new unwired,
process-local volatile memory boundary and still-planned durable/user-facing
product memory.
Any storage/migration/startup, manifest/lockfile/dependency, Tauri/React,
capability/permission, runtime/native-runtime, tool/policy/approval, provider,
Hermes, workflow, CI, hook, or skill change is a stop condition.

## Interfaces and invariants

### Memory identity and access

`AgentMemoryProfileId` is closed:

| Agent                | Profile                     | Current access                                                     |
| -------------------- | --------------------------- | ------------------------------------------------------------------ |
| Personal Assistant   | `PersonalAssistantMemoryV1` | approved-shared read; own private/task read-write; propose shared  |
| Research             | `ResearchWorkingMemoryV1`   | own private/task read-write; propose shared; no direct shared read |
| Knowledge & Document | `KnowledgeWorkingMemoryV1`  | own private/task read-write; propose shared; no direct shared read |
| Other six agents     | `MemoryDisabledV1`          | none                                                               |

- `AgentDefinition` is the sole agent-to-memory-profile mapping. Its sealed
  identity captures agent, policy profile, and memory profile together.
- `AgentTask`, `AgentExecutionContext`, and `AgentAttribution` carry the exact
  memory profile. Live-context matching includes it; no public constructor,
  default, or role-name inference exists.
- Store mutation methods are called only after orchestrator live-context
  validation. Test-only construction cannot exist in production.

### Volatile MemoryStore

- `MemoryStore` is a concrete application-owned process-local store, not the
  deleted legacy trait/API. Exactly one store is owned directly by one
  one-root `AgentOrchestrator`; it is shared only among tasks inside that
  bounded workflow and is cleared when the orchestrator/store drops. This
  increment does not expose an injectable/global store, share memory between
  orchestrator instances, or claim cross-session/cross-workflow retention. It
  owns no thread, I/O, database, clock, or runtime.
- Limits: at most 64 live records total, at most 16 pending proposals, at most
  8,192 UTF-8 bytes per record, at most 131,072 retained content bytes, and
  monotonic checked identifiers/versions. Empty, noncanonical, control-bearing,
  or oversized content fails before mutation.
- `MemoryNamespace` is closed: `ApprovedShared`, `AgentPrivate`,
  `TaskTemporary`, and `ProposedShared`. Records bind exact owner/proposer agent,
  task/root identity where applicable, version, and content. Debug/errors expose
  only closed metadata and lengths.
- Reads always name an exact `MemoryRecordId`. Personal Assistant alone may read
  approved shared directly. Private/task reads require exact live agent/task
  ownership. Specialists receive shared content only when Personal Assistant or
  application context assembly explicitly selects an approved record.
- `propose_shared` creates inert proposed content. Application review is closed:
  `Edit`, `Approve`, `ApproveEdited`, or `Reject`. Edit remains pending; approve
  atomically reserves destination capacity, removes the proposal, and creates
  one approved-shared record; reject purges content. Receipts contain no content.
- Delete requires exact allowed namespace/owner or application review authority.
  A failed write/delete/review leaves every namespace unchanged.
- Task cleanup removes only exact task-temporary records after terminal task
  mutation. Failed runtime/approval cancellation leaves memory untouched. Store
  disable clears all volatile content atomically and later operations return
  `MemoryDisabled`; re-enable starts empty.
- Retention is exact and workflow-local: task records until terminal cleanup;
  private/shared/pending records until delete/reject/disable or the owning
  one-root orchestrator/store drops. Nothing crosses orchestrator workflows or
  survives process exit.

The production domain symbols are exactly `MemoryStore`, `MemoryRecordId`,
`MemoryRecordVersion`, `MemoryContent`, `MemoryNamespace`, `MemoryWriteTarget`,
`MemoryRecordView`, `MemoryContextSelection`, `MemoryContextBundle`,
`SharedMemoryProposalId`, `SharedMemoryProposalView`,
`SharedMemoryReviewDecision`, `SharedMemoryReviewReceipt`, `MemoryAccessGrant`,
and `MemoryStoreError`. Mutation stays crate-private to the application
service.
The public orchestrator-facing operations are exactly `write_memory`,
`read_memory`, `select_memory_context`, `propose_shared_memory`,
`shared_memory_proposal`, `review_shared_memory`,
`withdraw_shared_memory_proposal`, `delete_memory`,
`delete_approved_shared_memory`, and `set_memory_enabled`. Application review,
approved-shared deletion, document registration/revocation, and store enable or
disable do not accept model-authored identity or authority.

Authority is typed and non-transferable:

- `LiveMemoryAccessProof(())` and `ApplicationMemoryControlProof(())` have
  fields private to `agent::orchestrator`, are non-`Clone`, and have no public
  constructors. Only successful exact live task/run/context validation creates
  the former; only a trusted application-control orchestrator entry creates the
  latter.
- `MemoryAccessGrant` is non-`Clone` and binds the complete live
  `AgentAttribution`, including agent, policy profile, memory profile, task,
  root, parent, runtime, depth, and current run/request identity. Its production
  constructor is crate-private and requires `LiveMemoryAccessProof`.
- Every agent read/write/select/propose/withdraw/delete method on `MemoryStore`
  is crate-private and requires `&MemoryAccessGrant`. Review, approved-shared
  deletion, and enable/disable require `ApplicationMemoryControlProof`. There
  is no raw-store mutation accepting caller-supplied `AgentId`, task/root ID,
  profile ID, or namespace as authority. Test fixtures use cfg(test)-only
  helpers and cannot enter production.
- Each grant is constructed, used, and dropped inside one orchestrator method;
  it is never retained in the store, task, run, event, or orchestrator fields
  and cannot be replayed after a task/run transition.

The delete/review matrix is closed:

| Operation                                   | Allowed authority                                      | Fail-closed rule                                                       |
| ------------------------------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------- |
| Delete agent-private                        | exact live owner agent                                 | another agent or stale task/run is rejected                            |
| Delete task-temporary                       | exact live owning task                                 | another task/root/agent is rejected                                    |
| Withdraw proposal                           | exact live proposer plus expected proposal version     | stale version, reviewer, or another agent is rejected                  |
| Edit/approve/approve-edited/reject proposal | application control plus exact expected version        | compare-and-swap mismatch leaves proposal and all namespaces unchanged |
| Delete approved shared                      | application control plus exact expected record version | no agent may delete through its live grant                             |
| Terminal task cleanup                       | orchestrator after successful terminal task mutation   | removes only that task's temporary records; no caller authority        |
| Disable                                     | application control                                    | atomically clears all volatile domains and starts disabled             |

All proposal review variants carry an explicit expected
`MemoryRecordVersion`. `Edit` replaces content and increments the pending
version; `Approve`, `ApproveEdited`, and `Reject` compare the reviewed version
before any capacity reservation or mutation. Promotion creates one new
approved-shared record atomically. A stale review/withdraw/delete never changes
content, versions, record counts, proposal counts, or retained-byte accounting.

### Explicit context assembly

- `MemoryContextSelection` contains at most 8 exact record IDs and a maximum
  assembled 8,192 bytes. Duplicates, inaccessible records, or cumulative
  overflow fail without returning partial content.
- Selection returns one redacted provenance list plus bounded context; it never
  scans or copies an entire namespace. No conversation, document, sibling, or
  parent content is automatically included.
- An agent's private or task-temporary record is never transferable to another
  agent through this API. Cross-agent child assembly accepts only exact
  `ApprovedShared` IDs selected by the live Personal Assistant/application;
  Research and Knowledge receive the resulting bounded untrusted text in their
  runtime request but receive no store grant for those records. Private/task
  selections are usable only by their exact owning live agent/task.

### Approved-document boundary

- `ApprovedDocumentReader` is a bounded registry/reader owned by one
  `AgentOrchestrator` workflow, with at most 8 roots and 16 document references
  globally for that one root workflow. It retains exact paths only
  privately; Debug/errors/receipts expose opaque IDs, source kind, format, and
  byte count, never path or content.
- `ApprovedDocumentSource` is exactly `UserSelectedFile`, `TaskAttachment`,
  `ApprovedRootMember`, or `GeneratedArtifact`. Registration is trusted
  application input and binds exact root task plus allowed target
  `KnowledgeDocument`; a runtime/model cannot register or choose paths.
- Every selected-file/root/artifact path must be valid UTF-8, contain no NUL,
  and encode to at most 4,096 bytes. Non-UTF-8 or longer ambient paths fail
  closed rather than entering an error, Debug value, event, or retained record.
- Root-member input is a closed relative path of at most 4,096 UTF-8 bytes, 64
  components, and 255 UTF-8 bytes per component; components are only normal
  nonempty names. Absolute/prefixed/rooted paths, `.`, `..`, empty values,
  schemes, NUL, and noncanonical separators fail.
- Registration/read rejects symlink root, member component, or final target;
  canonical containment uses component-aware `starts_with`. The target must be
  one regular file. Immediately before reading, metadata is captured, the file
  is opened once, opened-handle identity/type is compared with the validated
  target, and `limit + 1` bytes are read from that handle. Changed/replaced files
  fail closed.
- `MAX_DOCUMENT_BYTES` is 16,384. Exact formats are lowercase `.txt` ->
  `Utf8Text` and lowercase `.md` -> `Markdown`. Empty, invalid UTF-8, NUL,
  oversized, unsupported-extension, directory, missing, and non-regular input
  is rejected. No format sniffing expands support.
- References are task/root-bound, one-time readable for task assembly, and
  revocable. Terminal root cleanup revokes all remaining references and roots.
  There is no enumeration or crawl API.
- Reading is transactional with task creation. A reference has exactly
  `Available`, `Reserved`, `Consumed`, or `Revoked` state. `prepare_read`
  revalidates and reads into a non-`Clone` prepared value and changes only the
  reference to `Reserved`; a second prepare fails. `abort_read` is infallible
  and returns an exact reservation to `Available`; `commit_read` is infallible
  and changes that reservation to `Consumed`. Replay after commit/revoke fails.
- The exact mutation order is: validate live Personal context, pending approval,
  Knowledge definition/profile, route, depth, task/child/event/run budgets, and
  reference; reserve/read the document and assemble selected memory without
  mutating tasks; construct the child value and bounded runtime request; then
  cancel the initial root run. Any failure before successful root-run
  cancellation aborts the reservation, drops all raw buffers, and leaves the
  reference retryable. After root-run cancellation succeeds, transition the
  parent to waiting and infallibly commit the document reference before
  starting the child run. A child-start failure therefore consumes the
  one-time reference, terminally fails the parent, drops the request/content,
  and runs terminal cleanup; it does not advertise retry. Successful start
  inserts exactly one child/run and the preflighted content-free events.
- Pure standard-library path validation retains a documented narrow TOCTOU
  advisory because Rust `std` lacks atomic component-by-component no-follow
  open. Metadata/opened-handle identity checks are mandatory; adding unsafe or a
  dependency is outside scope.

The production domain symbols are exactly `ApprovedDocumentReader`,
`ApprovedDocumentId`, `ApprovedRootId`, `ApprovedDocumentSource`,
`ApprovedDocumentFormat`, `ApprovedRelativePath`, `DocumentOperation`,
`DocumentCapabilities`, `DocumentAccessGrant`, `PreparedDocumentRead`,
`DocumentReadCommit`, `DocumentTaskDescriptor`, `DocumentTaskResult`, and
`ApprovedDocumentError`.
The trusted public application operations on `AgentOrchestrator` are exactly
`register_approved_document`, `register_approved_root`,
`register_approved_root_member`, `revoke_approved_document`, and
`request_document_task`; agents and runtime code receive none of those
registration authorities.

`LiveDocumentAccessProof(())` and `ApplicationDocumentControlProof(())` have
fields private to `agent::orchestrator`, are non-`Clone`, and have no public
constructors. A non-`Clone` `DocumentAccessGrant` binds exact live Personal
attribution, the root workflow, the selected opaque reference, target
`KnowledgeDocument`, and operation. Reader registration/revocation methods are
crate-private and require application control; prepare/abort/commit require the
exact access grant or its linear commit token. No reader API treats a raw path,
opaque ID, agent ID, or operation as authority.

On supported Unix targets, private `FileIdentity` uses opened-handle and path
metadata fields `dev`, `ino`, file type/mode, link count, size, `mtime` plus
nanoseconds, and `ctime` plus nanoseconds. Root, each member component, and
target are checked with `symlink_metadata`; a target with link count other than
one is rejected. Identity/type are compared before open, on the opened handle,
after the bounded read, and against the final path. Replacement, hard-link
alias, size/timestamp change, or in-place mutation detected by these fields
fails closed. `cfg(not(unix))` compiles but reports capabilities unavailable and
returns typed `UnsupportedPlatform`; this increment claims support only for the
repository's macOS/Linux executable-test targets and adds no unsafe code or
dependency.

The document/runtime input budget is aggregate, not just a file limit:
`MAX_DOCUMENT_BYTES = 16_384`, selected memory contributes at most 8,192 bytes,
and deterministic metadata/framing contributes at most 2,048 bytes. The builder
uses checked arithmetic and caps the raw UTF-8 `RuntimeTurnRequest` selected
text at 26,624 bytes, below its existing 65,536-byte input ceiling. This raw
limit is not a claim about later JSON serialization: the unchanged native
adapter independently enforces the exact post-serialization gateway-request
limit, and a later live transport increment must preserve that check.
Exact-limit, limit-plus-one, maximum framing, escape-heavy, and multibyte cases
are contract tests.

### Knowledge task activation

- Knowledge & Document changes to `AgentActivation::Initial`; all other
  deferred roles remain unchanged. Initial eligibility is not file, memory,
  tool, provider, or execution authority.
- Generic `request_delegation` and `DelegationMatrix` remain exactly Personal
  Assistant -> Research. A separate `request_document_task` validates a live
  Personal Assistant root, no pending approval, exact approved reference,
  Knowledge activation/profile, depth one, active/total child/event/run limits,
  and document read before mutating task state.
- The direct document route uses one Knowledge child and a fresh Personal
  Assistant synthesis run under the existing three-run maximum. It does not add
  Research, replenish the child budget, or allow Knowledge to delegate.
- `DocumentOperation` is closed to `Read`, `Summarize`, `ExtractFacts`,
  `IdentifySections`, `Classify`, and `ProposeOutline`. Runtime input uses
  application-owned framing that labels document content untrusted and includes
  only operation, content-free source/format metadata, and bounded text.
- Child input construction becomes role-correct and derives the assigned agent
  from the resolved definition; the existing Research path remains byte-for-byte
  compatible where its contract asserts exact input.
- Document content is not inserted into memory automatically. Knowledge may
  write its own task/private memory or propose shared content only through an
  explicit live application call.
- `request_document_task(context, document_id, operation, memory_selection)` is
  the only Knowledge child-creation entry. The optional selection contains at
  most exact `ApprovedShared` IDs selected by the live Personal Assistant; PA
  private/task records and proposed records are rejected rather than copied to
  Knowledge. The assembled bundle is explicitly labeled untrusted in child
  input. No document registration or memory selection is inferred from
  objective/model text.
- `DocumentTaskDescriptor` stores only opaque document ID, format, and operation
  with redacted Debug; it stores no path or content. The raw document exists
  only in `PreparedDocumentRead`, the assembled local `String`, then the
  redacted `RuntimeTurnRequest`/owned runtime run, and is dropped on every
  failure/cancellation/terminal path. It is never copied into `AgentTask`, the
  event journal, governance audit, errors, or task context.
- `DocumentTaskResult` binds exact child task ID, opaque document ID, format,
  operation, and bounded `AgentTaskOutput`; its Debug redacts output. It is the
  structured attributed result returned for parent synthesis. The child
  `AgentTaskOutcome` remains authoritative for lifecycle; the document result
  adds closed provenance rather than a second terminal state.
- `DocumentCapabilities` is owned by `ApprovedDocumentReader` and exposed by
  `AgentOrchestrator::document_capabilities()`. It reports availability for the
  current target, exactly `.txt`/`.md`, the six closed operations, maximum file
  and aggregate request bytes, read-only access, no enumeration, and no artifact
  write. Catalog activation alone grants none of these capabilities.

### Terminal cleanup and failure ordering

| Path                                                       | Required ordering and retained state                                                                                                                                                                                       |
| ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Knowledge child completes/fails/is independently cancelled | terminalize child first; capture its structured success/failure/cancel outcome for parent synthesis; then infallibly remove that child's task-temporary memory and consumed document descriptor/reference; resume the root |
| Root completes/fails/cancels without live child            | after successful root terminal mutation, infallibly remove root task-temporary memory plus all remaining document references/roots                                                                                         |
| Root cancellation while child is live                      | cancel/audit child pending approval first, cancel child runtime/task, clean child temporary memory/document state, then cancel root runtime/task and clean root state; existing child-first event ordering remains         |
| Approval cancellation fails before task mutation           | leave task, run, memory, reservation/reference, and approval unchanged for safe retry                                                                                                                                      |
| Runtime cancellation fails                                 | leave the corresponding live task/run and its task memory/reference unchanged; do not perform terminal cleanup                                                                                                             |
| Parent synthesis start fails after child terminal          | child is already terminal and cleaned; fail the root, then clean root task memory and all remaining document state                                                                                                         |
| Document child start fails after root-run cancellation     | reference is already consumed; fail root, drop request/raw content, and clean root/document state; no live child or retry claim remains                                                                                    |
| Memory store disable                                       | atomically clear memory domains only; it does not cancel tasks/runs or mutate document registry state                                                                                                                      |

All cleanup methods mutate only in-memory maps and are infallible after their
terminal preconditions. Capacity is preflighted before lifecycle mutation, and
no cleanup may run while a cancellation failure leaves a task live.

## Implementation milestones

### Milestone 0 - Plan and trust-boundary gate

- [x] Record D-085 and this exact volatile interpretation.
- [x] Reconcile obsolete publication state and the two prior blocked drafts.
- [x] Complete fresh readiness, architecture, and security review: Ready with
      advisories; retain the pure-`std` Unix TOCTOU advisory and stop if the
      specified identity checks cannot fail closed.
- [x] Begin gate `agent-memory-approved-documents` only after Ready.

### Milestone 1 - Identity and bounded memory domain

- [x] Add memory profile to definitions, tasks, contexts, and attribution.
- [x] Implement the bounded volatile store and typed redacted errors.
- [x] Implement proposal review, deletion, disable, explicit selection, and
      cleanup behavior.
- [x] Add unit and public memory contracts.

### Milestone 2 - Approved-document boundary

- [x] Implement opaque references, roots, relative members, exact formats,
      bounded read, symlink/containment/identity checks, and cleanup.
- [x] Add selected-file/root success plus adversarial path/content tests.

### Milestone 3 - Knowledge activation and deterministic task

- [x] Make Knowledge initially eligible and add the separate direct document
      route without altering generic delegation.
- [x] Assemble one bounded mock-runtime Knowledge task and Personal synthesis.
- [x] Prove exact result attribution, context selection, cleanup, cancellation,
      and unchanged Native runtime behavior.

### Milestone 4 - Validation and closeout

- [x] Run focused and full validation.
- [x] Complete architecture, security, code-health, technical-debt, and roadmap
      reviews with no unresolved Critical/High finding.
- [x] Synchronize architecture/privacy/current-state records and create the
      increment/review artifacts.
- [x] Prepare the immutable `PASS WITH ADVISORIES` report for deterministic
      marker finalization as the final workflow action.

## Security and privacy considerations

- Files and all stored content are untrusted. They grant no instruction,
  policy, approval, execution, path, or tool authority.
- Paths/content never enter logs, errors, Debug, events, governance audit,
  SQLite, frontend state, or provider traffic.
- No model/runtime supplies agent/task/root/profile/namespace/path identity.
- Exact access checks precede content cloning or mutation. Capacity and target
  capacity are preflighted; no silent discard or partial cross-namespace write.
- Approved roots are not permission to enumerate. Each member requires exact
  application selection and containment validation.
- Private/task records never cross agents or tasks. Proposed shared content is
  not readable as approved shared and cannot promote itself.
- Privacy documentation records memory-only storage, ownership, retention,
  deletion, cleanup, provider non-transmission, logging prohibition, and review.

## Test plan

Memory domain and integration:

- approved-shared write through review and Personal read;
- agent-private isolation across Personal/Research/Knowledge;
- task isolation across task/root identities;
- proposed shared edit, approve, edited approve, reject, replay, and capacity;
- expected-version compare-and-swap for every review/withdraw/delete variant,
  including stale version after edit and atomic no-change snapshots;
- forged live-memory proof/grant and application-review authority are
  unconstructible through public production APIs; stale context, agent, task,
  root, policy-profile, memory-profile, runtime, run, and request mismatches are
  rejected before read, content clone, or mutation;
- failed write/review/delete atomicity and redacted errors/Debug;
- task terminal cleanup, failed-cancel preservation, deletion, disable/clear,
  re-enable empty, and no cross-agent leakage;
- exact selection, duplicates, inaccessible record, cumulative bound, and no
  implicit namespace/history/document/sibling copying;
- all six deferred agents remain memory-disabled.

Document boundary:

- selected `.txt` and `.md` read and approved-root member read;
- denied absolute/parent/scheme/noncanonical member;
- symlink root, intermediate, final, and outside-root escape;
- hard-link alias and link-count rejection; root/component/target replacement;
  same-inode size/mtime/ctime mutation before and during read; opened-handle and
  final-path identity mismatch;
- missing, directory, non-regular, unsupported extension, uppercase extension,
  invalid UTF-8, NUL/binary, empty, and oversized document;
- stale/revoked/replayed reference, foreign task/agent, cleanup, capacity, and
  path/content redaction.
- forged registration/document-control and live-document authority are
  unconstructible through public production APIs; wrong root/reference/target/
  operation attribution is rejected before read;
- 4,096-byte/64-component/255-byte-component relative-path bounds and global
  per-workflow root/reference bounds;
- reserve/abort/commit and each pre-/post-root-cancellation failure branch,
  including root-cancel failure retry, child-start failure consumption/cleanup,
  and no partial child/task/event state;
- exact aggregate runtime-input boundary, boundary plus one, maximum framing,
  multibyte document/memory content, and checked arithmetic.

Knowledge integration and regressions:

- exact catalog activation/profile/capability reporting;
- direct Personal -> Knowledge approved-document task and exact attributed
  result/synthesis;
- runtime input contains only selected bounded document and correct untrusted
  framing, never path or unrelated memory;
- raw-content sentinels are absent from Debug/Display/errors/events/audit for
  `MemoryContent`, record/proposal views, context bundle,
  `PreparedDocumentRead`, descriptor/result, `RuntimeTurnRequest`, mock runtime
  starts, orchestration state, and every failure branch; typed test-only content
  access is used only for exact data-flow assertions;
- the test-only mock recorder may retain only synthetic document/context
  sentinels for typed data-flow assertions; its Debug stays redacted and this
  retention is not a production-lifetime claim;
- generic Personal -> Knowledge delegation still denied; Research -> Knowledge
  and Knowledge delegation denied; other specialists unchanged;
- missing/denied/revoked document causes no child/task/run mutation;
- child failure/cancellation/root cancellation revokes refs and cleans exact task
  memory without orphaning or silently dropping a live retryable task;
- terminal cleanup table ordering, including approval/runtime cancellation
  failure preservation and synthesis-start/root-failure cleanup;
- existing definition/governance/orchestration/runtime/gateway/storage/frontend
  regressions remain green; Native remains sole/default.

## Verification commands

```bash
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked memory::tests::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked documents::tests::
cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_definition_registry_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_governance_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test storage_smoke --locked
cargo test --manifest-path src-tauri/Cargo.toml --test startup_storage_smoke --locked
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
python3 .codex/hooks/post_increment_gate.py finalize --increment agent-memory-approved-documents --report docs/reviews/2026-08-12-agent-memory-approved-documents-post-increment-review.md
python3 .codex/hooks/post_increment_gate.py status
```

No manual application check is required while the boundary remains unwired. A
future file picker/IPC/UI increment requires its own target-Mac manual evidence.

## Risks

- Path validation cannot make ambient desktop filesystem authority disappear;
  opaque references, exact registration, no enumeration, no-follow checks, and
  opened-handle identity comparison are load-bearing.
- Standard-library path opening retains a narrow TOCTOU advisory. Stop if tests
  demonstrate that the selected approach cannot fail closed on supported hosts.
- Adding memory identity across existing context/audit types can create broad
  compatibility churn or a missing-field fallback. There must be no optional or
  default identity.
- Cleanup ordered before terminal task mutation can lose retryable state; cleanup
  must follow successful terminalization.
- Calling volatile approved shared memory permanent/durable would be inaccurate.
- Knowledge activation can overstate capability; docs and descriptors must say
  selected UTF-8 text/Markdown, deterministic transport-free task only.

## Rollback or failure strategy

Before publication, remove the new memory/document modules and contract, restore
the exact prior definition/task/context/attribution/orchestrator symbols and
tests, and remove only D-085/current-plan current-state additions. Leave D-033
and every earlier increment/evidence record intact. SQLite has no migration or
product data to reverse; there is no external state, dependency, permission,
provider, process, or user data. After publication, use one bounded revert plus
an additive superseding decision rather than rewriting D-085 history.

## Decisions made

- D-085 accepts only a new volatile namespace-aware store, not restoration of
  the deleted legacy scaffold or resolution of ARB-005.
- Direct Personal Assistant -> Knowledge is a separate document-task route;
  generic delegation and the later Research/Knowledge workflow remain unchanged.
- Initial document support is exact `.txt`/`.md` UTF-8 with no dependency.
- Application preprocessing/context injection is used instead of a filesystem
  runtime tool proposal.

## Discoveries

- The prompt's premise that an existing `MemoryStore` can be extended is stale:
  D-033 deliberately deleted it, and current source has no memory module.
- Existing native storage persists bootstrap metadata only and cannot accept
  memory/document content without resolving ARB-005.
- Current child input is Research-specific and must become role-correct without
  changing the existing route's semantics.
- Tauri capabilities do not constrain trusted Rust `std::fs`; containment must
  be enforced inside the approved-document boundary.

## Progress

- 2026-08-12: Read mandatory authorities, inspected clean Git/current marker,
  inventoried memory/storage/session/document/platform/agent/governance/tests,
  and ran four focused clean-baseline contracts.
- 2026-08-12: Recorded D-085 and wrote this owner-selected merged ExecPlan.
- 2026-08-12: Corrected sealed authority, versioned proposal review,
  workflow-local ownership, document reservation, raw-input bounds, cleanup,
  Unix identity, portability, and adversarial contracts; reconciled current
  memory/roadmap records; fresh readiness, architecture, and security reviews
  returned Ready with advisories.
- 2026-08-12: Began required gate `agent-memory-approved-documents` after the
  Ready-with-advisories verdicts; implementation started with no source scope
  expansion.
- 2026-08-12: Implemented the sealed volatile memory and approved-document
  boundaries plus the separate Personal Assistant-to-Knowledge route. Focused
  contracts pass 6/6, 9/9, and 10/10; registry, governance, orchestration,
  runtime, gateway, storage, and startup regressions pass.
- 2026-08-12: Completed formatting, all-target/all-feature check, strict
  Clippy, all-target Rust (269 passed, one intentional ignored probe), complete
  `npm run verify`, documentation/repository/security checks, diff hygiene,
  session-end inventory, and independent reviews. The quality result is `PASS
WITH ADVISORIES`; only the pure-`std` document-open TOCTOU residual remains.

## Acceptance criteria

- [x] New bounded volatile `MemoryStore` supports all four namespaces without
      restoring the deleted interface or touching SQLite.
- [x] Shared records require a proposal plus application review; specialists
      cannot directly write approved shared memory.
- [x] Exact memory identity is captured and revalidated with no fallback, and
      private/task isolation plus cleanup/deletion/disabled behavior pass.
- [x] Approved-document reads accept only exact task-bound `.txt`/`.md`
      references and reject traversal, symlink escape, replacement, unsupported,
      missing, binary, empty, and oversized inputs.
- [x] Knowledge is truthfully eligible only for the separate bounded document
      route; generic and Research-to-Knowledge workflow scope remains unchanged.
- [x] Explicit context selection prevents unrelated memory, history, documents,
      and sibling output from entering child context.
- [x] No vector database, dependency, unrestricted file access, persistence,
      provider, runtime change, IPC, UI, or unsupported format claim exists.
- [x] All applicable pre-finalization validation passes and the marker-ready
      report exactly matches the complete workspace inventory.

## Final results

Verified complete with advisories. Focused memory, document, and public
contracts pass 6/6, 9/9, and 10/10. Registry, governance, orchestration,
runtime, gateway, storage, startup, formatting, all-target/all-feature check,
strict Clippy, and complete repository verification pass. The all-target Rust
suite reports 269 passed and one intentionally ignored opt-in Hermes probe.
Independent architecture, security, code-health, technical-debt, and readiness
reviews found no blocker. The sole accepted advisory is the narrow pure-`std`
document-open TOCTOU residual; remediation belongs to a future approved
file-picker/platform-adapter increment and blocks neither completion nor the
next increment. No next plan is Ready, so readiness is `Blocked`. Publication
remains a separate project-owner action.

## Documentation updates

- [x] `ARCHITECTURE.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `PRODUCT_REQUIREMENTS.md`
- [x] `docs/security/AGENT_MEMORY_DOCUMENT_PRIVACY.md`
- [x] `ROADMAP.md` and detailed native roadmap
- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `DECISIONS.md` (D-085)
- [x] `CHANGELOG.md`
- [x] increment and consolidated review
- [x] `TROUBLESHOOTING_LOG.md` intentionally unchanged because no durable issue
      was observed
