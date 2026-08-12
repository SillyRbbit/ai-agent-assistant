# Volatile agent memory and approved-document Knowledge boundary

Status: Verified complete with advisories
Date: 2026-08-12
Gate ID: `agent-memory-approved-documents`
Plan: `docs/plans/2026-08-12-agent-memory-approved-documents.md`
Decision: D-085, preserving D-084, D-083, D-082, D-079, and D-033
Baseline: clean synchronized `main` at `2687294`

## Goal

Add a bounded workflow-local volatile memory store, a narrow reader for
explicitly approved UTF-8 text and Markdown, and one direct deterministic
Personal Assistant-to-Knowledge document task without adding persistence,
unrestricted filesystem access, a provider, Tauri/React wiring, or the full
Research-to-Knowledge workflow.

## Implemented boundary

- `MemoryStore` is owned by one one-root `AgentOrchestrator` and has four
  closed namespaces: `ApprovedShared`, `AgentPrivate`, `TaskTemporary`, and
  `ProposedShared`. Nothing crosses an orchestrator workflow or survives store
  drop or process exit.
- Sealed built-in definitions carry exact memory profiles through task,
  execution-context, and attribution identity. Non-cloneable live grants and
  separate application-control proofs prevent caller-, model-, and
  runtime-authored authority.
- Personal Assistant may read approved shared and its own private/task records.
  Research and Knowledge may use only their own private/task records and submit
  inert shared proposals. The other six catalog roles are memory-disabled.
- Shared proposals use expected-version edit, approve, approve-edited, reject,
  withdraw, and delete semantics. Specialists cannot directly publish approved
  shared content. Context assembly names exact records and never scans or
  implicitly copies a namespace, history, document, or sibling result.
- `ApprovedDocumentReader` retains application-selected paths privately behind
  opaque workflow-bound references. It supports user-selected files, task
  attachments, approved-root members, and generated artifacts, but exposes no
  path-based agent API, directory enumeration, write, rename, or delete power.
- Reads accept only nonempty lowercase `.txt` or `.md` UTF-8 files. Supported
  Unix targets reject symlinks, hard-link aliases, non-regular files,
  containment escape, replay, revocation, and detected identity changes while
  comparing registered, opened-handle, and final-path identity. Non-Unix
  targets report the capability unavailable.
- A reference is linearly reserved, aborted, or consumed. The separate direct
  Personal Assistant-to-Knowledge route consumes one approved reference and an
  optional exact approved-shared selection, labels both untrusted, returns one
  attributed `DocumentTaskResult`, and resumes a fresh Personal synthesis run.
  Generic delegation remains exactly Personal Assistant to Research;
  Research-to-Knowledge and Knowledge delegation remain denied.

## Limits and privacy

- Memory is capped at 64 live records, 16 pending proposals, 8,192 bytes per
  record, and 131,072 retained bytes. A selection is capped at eight records
  and 8,192 bytes.
- The document registry is capped at eight roots and sixteen references. Paths
  are capped at 4,096 bytes; relative paths at 64 components and 255 bytes per
  component.
- A document is capped at 16,384 bytes. Selected memory contributes at most
  8,192 bytes and framing at most 2,048 bytes, making the raw request maximum
  26,624 bytes.
- Memory content, paths, document text, selected context, and model output do
  not enter logs, errors, Debug, audit, SQLite, frontend state, or provider
  traffic. There is no provider or network path in this increment.
- Terminal cleanup removes exact task-temporary memory and document state only
  after successful lifecycle mutation. Failed approval/runtime cancellation
  retains retryable live state.

## Preserved boundaries

`NativeAgentRuntime` remains sole/default and unchanged. Runtime tool proposals
remain rejected, governance remains non-executing, and Knowledge activation is
eligibility only for the closed document route. No dependency, manifest,
lockfile, migration, SQLite product table, durable memory, vector database,
embedding, semantic search, file picker, Tauri IPC, React consumer, provider,
live model, executor, permission, credential, Hermes integration, platform
effect, or user-visible behavior was added.

## Evidence

- Memory units: 6 passed.
- Approved-document units: 9 passed.
- Public memory/document contract: 10 passed.
- Definition/registry: 7 passed; governance: 10 passed; orchestration: 22
  passed; runtime: 20 passed; gateway: 10 passed; storage and startup-storage:
  1 passed each.
- Rust formatting, all-target/all-feature check, and strict Clippy passed.
- All-target Rust: 269 passed, 0 failed, with one intentionally ignored opt-in
  real-Hermes probe.
- `npm run verify` passed after formatting four closeout documents. It included
  repository health, lint, 28 hook tests, 38 repository tests, 124 frontend
  tests, 166 library tests, all integration contracts, TypeScript, Vite, and a
  Tauri no-bundle release build.
- Documentation, repository-health, security-scan, diff-hygiene, and session-end
  checks passed. No application check was required because the boundary is
  unwired and no Tauri/React behavior changed.

Quality result: `PASS WITH ADVISORIES`. The sole advisory is the narrow
pure-standard-library document-open TOCTOU residual. Effort is Medium in a
future separately approved file-picker/platform-adapter increment; it blocks
neither completion nor the next increment. Next-increment readiness is
`Blocked` because no later plan is Ready, not because of this advisory.

## Rollback

Before publication, remove the new memory/document modules and public contract;
restore only the declared definition, task, governance, orchestrator, library,
registry-contract, and current-state documentation edits. There is no data,
migration, dependency, IPC, permission, credential, provider, process, network,
or external state to reverse. After publication, use one bounded revert and an
additive superseding decision rather than rewriting D-085 history.
