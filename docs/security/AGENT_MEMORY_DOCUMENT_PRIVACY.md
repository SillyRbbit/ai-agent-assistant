# Agent memory and approved-document privacy boundary

Status: Verified D-085 privacy contract; PASS WITH ADVISORIES
Last updated: 2026-08-12
Scope: workflow-local volatile memory and explicitly approved UTF-8 documents

## Privacy objective

Keep agent context opt-in, bounded, attributable, and local. The application,
not a model or runtime, owns memory selection, shared-memory review, document
registration, task creation, and terminal cleanup. This increment adds no
persistence, provider transport, WebView or Tauri IPC, logging, vector store,
search index, OCR service, document generator, or unrestricted filesystem tool.

## Storage and retention

| Data                           | Storage location                                                                     | Retention                                                                                       | Terminal or control cleanup                                                                               |
| ------------------------------ | ------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Approved shared memory         | One `MemoryStore` owned by one bounded `AgentOrchestrator`                           | Until application deletion, memory disable, or store/orchestrator drop                          | Disable clears all volatile memory; process exit also clears it                                           |
| Agent-private memory           | The same process-local store, bound to one exact agent                               | Until exact-owner deletion, memory disable, or store/orchestrator drop                          | A task ending does not silently promote or delete private memory                                          |
| Task-temporary memory          | The same store, bound to one exact agent and task                                    | Only while that task is live                                                                    | Removed after successful terminal task mutation; cancellation failure leaves it intact for retry          |
| Proposed shared memory         | A separate pending-proposal map in the same store                                    | Until edit, approval, rejection, exact-proposer withdrawal, disable, or store/orchestrator drop | Reject and withdrawal purge content; approval atomically moves reviewed content to approved shared        |
| Approved document target paths | Private maps inside one `ApprovedDocumentReader`                                     | Until one-time consumption, revocation, terminal root cleanup, or orchestrator drop             | Consumption and revocation immediately purge the target path while retaining a path-free replay tombstone |
| Approved root paths            | The same private reader                                                              | Until terminal root cleanup or orchestrator drop                                                | Root cleanup removes the root and every remaining member reference; there is no enumeration API           |
| Raw document content           | A bounded local read buffer, then one runtime request owned by the local runtime run | Only for request construction and the bounded task/run lifetime                                 | Dropped on every failure or terminal path; never copied to memory automatically                           |
| Structured document output     | Bounded `AgentTaskOutput` plus opaque document provenance                            | Through the bounded child result and Personal Assistant synthesis in the current orchestrator   | Not persisted; dropped with the orchestrator                                                              |

None of these domains uses SQLite, the filesystem for memory storage, Keychain,
browser storage, cloud storage, or another orchestrator instance. Existing
bootstrap SQLite storage is unchanged and does not receive agent memory or
document content.

## Namespace ownership and transfer

- Personal Assistant may read approved shared memory, read/write its own
  private and current-task memory, and propose shared content.
- Research and Knowledge & Document may read/write only their own private and
  current-task memory and may create inert shared-memory proposals. They cannot
  directly read or publish approved shared memory.
- The other six catalog roles use `MemoryDisabledV1` and receive no current
  memory operation.
- A cross-agent child request may receive only exact approved-shared record IDs
  selected under a live Personal Assistant root. Private, task-temporary, and
  proposed records fail closed instead of being copied.
- No namespace is scanned automatically. Selection names at most eight opaque
  record IDs and assembles at most 8,192 UTF-8 bytes.

## Shared-memory review

A specialist proposal is non-authoritative. It remains in `ProposedShared`
until trusted application control performs one version-checked action:

- `Edit` replaces the pending content and increments its version;
- `Approve` promotes the reviewed content to one approved-shared record;
- `ApproveEdited` promotes explicitly edited content;
- `Reject` removes the proposal and its content.

Every edit, approval, rejection, withdrawal, and deletion compares an expected
`MemoryRecordVersion` before mutation. A stale version, wrong owner, capacity
failure, or disabled store leaves other records, proposals, versions, and byte
accounting unchanged. There is no silent specialist write to approved shared
memory.

## Approved-document boundary

Only trusted application operations can register one user-selected file, task
attachment, application-generated artifact, or member of an approved root.
Registration produces opaque workflow-bound IDs; agents and runtimes cannot
supply a path, register a root, enumerate a directory, or discover references.

The current reader supports lowercase `.txt` and `.md` containing nonempty
UTF-8 text only. One document is limited to 16,384 bytes. On supported Unix
targets it rejects symlinks, hard-linked targets, non-regular files, invalid
relative paths, traversal, containment escape, and detected identity changes.
It reads at most limit-plus-one bytes from one opened handle and compares path
and opened-handle identity before and after the read. Pure standard-library
path opening retains a narrow documented TOCTOU residual risk; no unsafe code
or new dependency is added. Non-Unix targets report the boundary unavailable.

A reference is one-time and transitions through `Available`, `Reserved`, then
`Consumed`, or can become `Revoked`. Any failure before root-run cancellation
aborts the reservation and drops the buffer. Once root-run cancellation
succeeds, the reference is consumed before the Knowledge child starts; a later
start failure does not advertise retry.

## Knowledge & Document capability

Knowledge & Document is catalog-eligible only for the separate direct
Personal-Assistant-to-Knowledge document route. This does not change the
generic delegation matrix, which remains Personal Assistant to Research only.
Knowledge cannot delegate, choose files, crawl roots, use a tool, execute an
action, write an artifact, or directly publish shared memory.

The route accepts one approved reference, one closed read-only operation, and
an optional explicit approved-shared selection. Document text and selected
memory are labeled untrusted in a raw UTF-8 runtime request capped at 26,624
bytes. The existing runtime request boundary independently applies its own
limits. Output is returned as one attributed `DocumentTaskResult`, then a fresh
Personal Assistant run produces the final bounded synthesis.

## Provider transmission and logging

There is no provider or network transport in the current product path. The
Rust `AgentRuntime`/`NativeAgentRuntime` foundation remains transport-free and
unwired, so this increment performs no external transmission. A later live
provider increment must make selected document and memory transmission
explicit, apply the approved provider/retention gates, and preserve all local
bounds; D-085 grants no such authority.

Memory content, document paths, raw document text, selected context, and model
output are not written to logs, audit records, errors, orchestration events, or
Debug output. Typed read models intentionally expose content only to trusted
Rust callers that requested it. Issued typed snapshots are caller-owned and can
outlive later store cleanup; store retention guarantees do not revoke those
already-issued values. Debug surfaces report closed metadata and lengths, with
opaque IDs and content redacted.

## Limits

- 64 live memory records, 16 pending proposals, and 131,072 retained memory
  bytes per one-root orchestrator.
- 8,192 bytes per memory record; eight records and 8,192 assembled bytes per
  explicit selection.
- Eight approved roots and sixteen document references per one-root
  orchestrator.
- 4,096-byte ambient or relative path, 64 relative components, and 255 bytes
  per relative component.
- 16,384 document bytes plus at most 8,192 selected-memory bytes and 2,048
  framing bytes, for a maximum 26,624-byte raw document task input.
- Existing orchestration limits remain one root, one child, depth one, one
  active child, two tasks, three runtime runs, and bounded event journals.

## Explicit exclusions

No full-home or project crawl, Keychain or browser-profile access, cloud-drive
scan, vector database, embeddings, semantic retrieval, external OCR, new file
format, document comparison across multiple files, document write, Research to
Knowledge workflow, durable memory, automatic conversation ingestion, or
cross-workflow retention is implemented or authorized.
