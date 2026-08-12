# Knowledge and document boundaries

Status: Superseded by D-085's selected narrow `.txt`/`.md` boundary
Owner: Project owner
Last updated: 2026-08-11
Disposition: The owner-selected
[`2026-08-12-agent-memory-approved-documents.md`](2026-08-12-agent-memory-approved-documents.md)
replaces this provisional draft for a volatile, no-persistence, read-only
selected-file boundary. Any richer format, parser, picker, IPC, artifact writer,
or durable storage work requires a new plan; this historical draft grants no
authority.
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: define the approved document-reading, knowledge-extraction, and
memory boundaries required before Knowledge & Document Agent can be enabled.

## Provisional scope

- Explicit user-approved files or roots with no unrestricted filesystem crawl.
- Read-only document intake, bounded parsing, summarization, comparison,
  organization, and preparation.
- Typed provenance and exact agent/task attribution for every source.
- Shared user/project, agent-private, task-temporary, and proposed-shared
  namespace rules owned by a future application `MemoryStore`.
- Proposed-shared content remains inert until explicit policy and user control
  promote it.
- Review, correction, export, retention, deletion, and cancellation behavior.

## Explicit non-goals

- No filesystem discovery outside approved roots, silent permanent memory
  write, vector database, embedding service, semantic index, or external
  retrieval dependency.
- No runtime-owned or Hermes memory.
- No provider, tool implementation, persistence schema, OS permission, IPC, or
  UI until separately planned.
- No activation of Knowledge & Document Agent through this draft.

## Required work before Ready

- Resolve exact storage, encryption, retention, recovery, and privacy decisions.
- Select supported document types and bounded parser/dependency policy.
- Define exact path authorization, symlink, package, archive, and error-redaction
  behavior.
- Add a complete deterministic fixture, adversarial, portability, rollback, and
  user-control validation plan.

## Authority and rollback

This draft grants no file, memory, persistence, permission, or implementation
authority. Removing it leaves Knowledge & Document Agent disabled and preserves
all current storage and runtime behavior.
