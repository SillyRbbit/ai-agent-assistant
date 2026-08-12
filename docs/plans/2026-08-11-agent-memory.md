# Agent-specific memory

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: verified agent governance, approved knowledge/document boundaries,
resolution of ARB-005, and a fresh privacy, security, architecture, and
readiness review

## Future goal

Define application-owned shared user/project, agent-private, task-temporary,
and proposed-shared memory namespaces with explicit attribution, visibility,
retention, deletion, export, and promotion rules.

## Provisional boundaries

- `MemoryStore` remains application-owned and separate from agents and
  `AgentRuntime`.
- Agent/task/optional-parent/policy identity binds every read, proposal, write,
  promotion, deletion, and audit record.
- Proposed shared memory is inert until application policy and user controls
  promote it.
- Knowledge & Document Agent cannot silently persist output or access outside
  approved files/roots.
- A child result is not durable memory by default.
- No runtime-owned or Hermes memory substitutes for the application store.
- No vector database, embedding service, semantic index, or cross-agent
  retrieval dependency is selected by this draft.

## Non-goals

No implementation, storage schema, migration, persistence dependency, agent
activation, provider, model, tool, Tauri IPC, UI, background indexing, cloud
sync, or production data. This plan grants no authority and remains Blocked.

## Required decisions before Ready

- Exact namespace identifiers, ownership, lifecycle, quotas, and redaction.
- Retention, deletion, export, corruption recovery, and rollback semantics.
- User control and disclosure for each cross-agent read or promotion.
- Exact encrypted storage and key-management boundary, if persistence is later
  selected.
- Deterministic no-I/O tests plus privacy, security, architecture, and
  readiness evidence.

## Rollback

No implementation exists. Removing this draft returns agent memory to the root
roadmap and leaves the runtime, catalog, governance, and Hermes evidence
unchanged.
