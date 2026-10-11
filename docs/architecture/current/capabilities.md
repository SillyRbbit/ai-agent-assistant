# Capability inventory

Source: `bc12776412c717613de1fdc42c38bf3312493726`; equivalent merged main `bbe7546281fec3c8b4b608d68a52bc9732d9ecd9`. No personal profiles were read.
Canonical names below are roles, not the owner's saved nicknames. Evidence keys link
to immutable source lines. This is a code/retained-evidence assessment, not new native QA.

## Status vocabulary

- **V — Implemented with verification:** source plus relevant retained automated evidence;
  native coverage is named explicitly, never implied for every combination.
- **I — Implemented, native verification not established here:** code exists; this
  walkthrough does not certify live account access or all operating conditions.
- **P — Partial / fixture-only:** a bounded component or simulation exists but the
  broader user capability is not wired or accepted.
- **D — Planned/deferred:** no current product path is claimed.
- **U — Unclear:** available evidence does not establish the capability or guarantee.

## Every bot: role does not grant authority

All nine roles are available to the native profile/chat surface with a saved connection,
model, effort, appearance, owner instructions and optional private notes. Default is
Simulation, Memory Off, empty instructions/notes, neutral tone and balanced verbosity.
All live routes are text advice. Canonical role identity is immutable; nicknames,
descriptions and artwork do not change routing or privileges. [CATALOG](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent/definition.rs#L151) [PROFILE](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_preferences.rs#L19) [CTX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat.rs#L56)

| Canonical bot ID     | Actual advice / fixed workflow role                                | Restrictions and evidence status                                                                                           |
| -------------------- | ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| personal-assistant   | Frames the supplied objective; final synthesis in all four routes  | V: live API/Codex chat and Research route. Cannot independently spawn bots or take actions.                                |
| research             | Analyzes supplied sources; second Research stage                   | V: Research-route native evidence. No autonomous browsing or web retrieval.                                                |
| coding               | Proposes solutions from supplied code; Engineering stage           | I: route and strict handoff implemented; Engineering not tested live in accepted milestone. No repository writes or tests. |
| cloud-infrastructure | Assesses supplied infrastructure; Operations stage                 | I: no actual cloud API, Terraform or deployment execution.                                                                 |
| systems-operations   | Proposes diagnostic/recovery steps from supplied material          | I: no service management, process access or remediation execution.                                                         |
| knowledge-document   | Organizes supplied evidence into attributed summaries/drafts       | V: Research route. Native Knowledge commands are owner UI operations, not this bot's arbitrary file authority.             |
| qa-validation        | Reviews proposal/evidence, names tests and gaps                    | I: advisory Engineering/Proposal stages; does not run a test suite or gate code.                                           |
| security-risk        | Reviews supplied risks and mitigations                             | I: advisory Engineering/Operations stage. It is not PolicyEngine and cannot authorize actions.                             |
| workflow-automation  | Produces a proposed workflow with dependencies and approval points | I: fixed Proposal route only; no scheduler, arbitrary workflow engine or background worker.                                |

The catalog also contains richer versioned instructions for the transport-free native
foundation. The live chat builder uses canonical display role plus communication rules
and saved context; do not assume every catalog instruction template is injected into
live requests. A role description is model context, not a deterministic security check.
[CATALOG](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent/definition.rs#L151) [CTX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat.rs#L56) [ROUTES](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration.rs#L30)

**Conductor is not a tenth provider bot.** The live Rust collaboration executor determines
stage order. Conductor is the interface identity for that coordination; it has no provider,
model or separate chat profile. The graph labels it `AgentOrchestrator`, but the current
live route is `collaboration_tauri::execute_stages`, not proof that the separate generic
foundation is executing live delegation. [COORD](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration_tauri.rs#L172) [CONDUCTOR](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/agents/ConductorIdentity.tsx#L5) [GRAPH](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/command-center/collaborationProjection.ts#L39) [FOUNDATION](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent/native_runtime.rs#L1)

## Runtime and authentication matrix

| Route             | Implementation and data destination                                               | Authentication / selection                                                                                                                                                                                                                              | Verification and limits                                                                                                                                                                           |
| ----------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Simulation        | Native fixed text/events; no hosted inference                                     | No credentials; model `simulation`, effort Default                                                                                                                                                                                                      | V: deterministic tests; explicitly simulated output is not live evidence.                                                                                                                         |
| Direct OpenAI API | Native HTTPS Responses endpoint; input/context/accepted history leave the machine | Debug-only opt-in; native session API key acquired during validated request preparation (including collaboration preflight). Three closed model IDs in profile validation; supported effort validated. API billing is separate from Codex/Work credits. | V: accepted API native matrix uses gpt-5.6-luna/low. Other allowed model/effort combinations are not thereby verified. Tools empty; store=false; background=false; no application retry/fallback. |
| Headless Codex    | Native child app-server over stdio, then its authenticated OpenAI service         | Debug-only opt-in; owner-selected executable and dedicated home; explicit file credential store. Handshake only reviewed 0.159.0/0.160.1. Runtime account/read accepts chatgpt or apiKey; no inherited OpenAI key or automatic account switch.          | V: dedicated authenticated file-store, actual executable, model discovery, minimal completion and restart recheck passed. Internal runtime retry/retention behavior remains external.             |
| Anthropic API     | Native Messages streaming and explicit model discovery                            | Debug-only opt-in, native Anthropic key, endpoint owned by Rust; catalog evidence constrains model/effort                                                                                                                                               | I: adapter and offline tests exist; no accepted live Anthropic matrix in this evidence. Catalog listing is not entitlement.                                                                       |
| LM Studio         | OpenAI-compatible streaming to validated literal loopback endpoint ending /v1     | Existing owner server; optional token bound to exact normalized endpoint; explicit catalog refresh; Default effort only                                                                                                                                 | I: adapter/offline negative tests. Loopback does not prove inference stays on device. No server install/start or model download.                                                                  |
| Ollama            | Loopback model discovery plus compatible text streaming                           | Existing owner server; optional endpoint-bound token; exclude recognized cloud models; Default effort only                                                                                                                                              | I: runtime locality remains unverified even after endpoint checks. No fallback or installation.                                                                                                   |

Sources: [PROFILE](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_preferences.rs#L19) [MODELS](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_models.rs#L10) [API](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/personal_assistant_direct.rs#L31) [CODEX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/codex_connection.rs#L156) [CLAUDE](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/anthropic.rs#L24) [LOCAL](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/local_models.rs#L20) [ADAPTER](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_adapter.rs#L16).
The readiness UI still contains an older Codex version hint; the actual handshake permits
two reviewed versions. Saved effort is shown as **support unverified** before discovery,
not silently replaced. Fresh catalogs are in-memory; discovery does not save a profile.

## Personality, skills and memory

| Kind                                   | Stored / loaded                                                               | Effect and boundary                                                                                                                               | Status                                                        |
| -------------------------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| Canonical role                         | Rust catalog                                                                  | Descriptive ID/role; application-owned routing                                                                                                    | V                                                             |
| Nickname, description, tone, verbosity | SQLite per-agent identity                                                     | Included as untrusted context. Current task precedence is expressed in prompt rules, not a proof against injection.                               | V; UI/owner aesthetic evidence inherited                      |
| Avatar, color                          | Same profile identity                                                         | 19 avatar choices including mascot; 12 colors. Selection controls art; it grants no capability. Identity object also crosses the context builder. | V                                                             |
| Owner instructions                     | SQLite per bot; revision captured on conversation start                       | Bounded at 4,096 characters; saving changes invalidates older conversation revision                                                               | V                                                             |
| Private notes                          | SQLite per bot, max 8,192 characters                                          | Included only for PrivateNotes; Memory Off omits from request but does not erase stored note. Collaboration clears notes and forces Off.          | V                                                             |
| Conversation history                   | Native BoundConversation memory                                               | Completed turns only; up to four turns and 16,384 history characters. One active bound chat session; no durable agent-chat transcript implied.    | V                                                             |
| Shared Knowledge                       | SQLite immutable document versions                                            | Manual keyword search / explicit source selection. No embeddings, automatic learning, vault crawl or hidden retrieval.                            | V for bounded storage; owner workflow comfort not re-run here |
| Procedural skills                      | Development Codex ECC/repository skills; separate native catalog instructions | No evidence of a live Cortexa skills marketplace, dynamic skill loader or executing SKILL.md in bot chat.                                         | D/U for product skill execution                               |
| Workflow-local memory foundation       | Rust MemoryStore, sealed orchestrator grants                                  | Private/task/proposed/shared bounded namespaces, no persistence; separate from live profile notes and room snapshots                              | P: implemented foundation, not automatic live learning        |

Sources: [PROFILE](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_preferences.rs#L19) [PROFILE_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/agent_preferences.rs#L92) [CTX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat.rs#L56) [MEMORY](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/memory.rs#L454) [KNOW](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/knowledge_tauri.rs#L80) [KNOW_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/knowledge.rs#L83).

## Knowledge & Documents and Obsidian-inspired UI

Implemented: owner notes, generated drafts requiring review, UTF-8 Markdown/text native
import, exact new-file Markdown export, bounded AND keyword search, title/version/hash
attribution, immutable versions, explicit re-import, historical version reading, stable
wikilinks/backlinks, contextual one-hop graph, source editor/rendered preview, unsaved
navigation protection, templates and a deliberately small frontmatter property editor.
Wikilinks resolve against stable document identities; rename/removal does not silently
retarget old evidence. Room source snapshots survive library removal. [KNOW](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/knowledge_tauri.rs#L80) [KNOW_VALUES](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/knowledge.rs#L4) [KNOW_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/knowledge.rs#L83) [LINKS](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/knowledge_links.rs#L5) [PROPS](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/knowledge/knowledgeProperties.ts#L29) [KGRAPH](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/knowledge/knowledgeNeighborhood.ts#L3) [MARKDOWN](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/knowledge/KnowledgeMarkdown.tsx#L19)

Bounds: 200 items, eight versions each, 16,384 bytes per version, 4 MiB aggregate;
search query 200 bytes, maximum 50 passage hits; passages prefer newline boundaries and
cap at 1,024 characters. One-hop graph caps at 25 nodes. Property fields are tags,
project, note_type and review_status. They are owner labels, not verified claims.
Markdown HTML, links and images are rendered inert rather than executed/fetched.
Native file dialogs are macOS implementations; unsupported platforms return unavailable.

Absent/deferred: Obsidian plugin compatibility, vault synchronization, folders-as-security,
PDF/OCR, embeddings/vector search, automatic knowledge ingestion, automatic memory updates,
background indexing and silent cross-bot sharing. Export refuses overwrite but an I/O
failure after creating a destination may leave a partial new file; no transactional
filesystem-write/rollback guarantee is inferred. SQL atomicity and file atomicity differ.

## Collaboration, graphs and actual actions

All four routes are implemented and sequential, with 4/5/5/4 provider stages. A single
application generation lease prevents concurrent chat/room execution. Each room has at
most four runs; total rooms cap at ten. Native preflight disallows mixing Simulation
with live stages. Profile revisions and selected document snapshots are rechecked at
Start against a preview serial and run/room IDs. [COORD](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration_tauri.rs#L172) [ROUTES](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration.rs#L30)

Only the four-stage Research route has accepted live native coverage for both API and
Codex in this milestone. Engineering, Operations, Proposal and mixed-live-provider
combinations remain untested live. Fixture-only bounded-parallel foundation code exists,
but current live collaboration does not execute parallel stages. [PARALLEL](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent/bounded_parallelism.rs#L1) [REVIEW](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/docs/reviews/2026-10-06-live-provider-e2e-post-increment-review.md#L261)

Command Center projects current saved profiles, selected live/persisted rooms, historical
runs or explicit deterministic demos. Connections show coordination, stage dependency
and validated handoff relationships. They are not model thoughts. The Knowledge graph
shows document links, not agent execution. Conductor/masked mascot animation follows
accepted UI snapshots, with reduced-motion support; it does not schedule work.
[CENTER](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/command-center/OperationalCommandCenterPage.tsx#L27) [GRAPH](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/command-center/collaborationProjection.ts#L39) [SNAP](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/collaboration/collaborationSnapshots.ts#L14) [CONDUCTOR](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/agents/ConductorIdentity.tsx#L5) [ART](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/agents/botMotion.ts#L20) [KGRAPH](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/knowledge/knowledgeNeighborhood.ts#L3)

**No live arbitrary tool loop or apply-changes workflow is established.** Closed tool
contracts exist for get_current_datetime/create_local_task, with deterministic policy,
approval and audit foundations. They are not registered as an unrestricted executor in
Tauri and not offered in live provider requests. The mock Conversations UI demonstrates
tool cards/approval/retry without I/O. File access in Knowledge is an owner-triggered
native operation; Coding advice does not edit a repository. Tasks/Activity/Permissions
and other demo views must not be counted as live device execution. [TOOLS](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/tools/schema.rs#L14) [POLICY](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/policy/engine.rs#L53) [APPROVAL](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/approvals/manager.rs#L23) [RPC](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/lib.rs#L73) [MOCK](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/conversations/ConversationWorkspace.tsx#L57) [MOCK_STATE](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/application/state.ts#L52)

## Reliability, persistence and accounting

Native chat exposes idle/starting/streaming/completed/stopped/error and a separate busy
ownership bit. Partial accepted text survives Stop/error. Only completed turns enter
history. After stopped/error, use a fresh conversation; the same failed context is not
silently resent. Stop closes ingress, aborts/joins transport and settles the host. Late
or repeated Stop must preserve terminal state; automation covers races, UI QA cannot
prove precise backend arrival ordering. [CHAT](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat_tauri.rs#L247) [HOST](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/personal_assistant_v0.rs#L246) [CTX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat.rs#L56)

Bounded chat timeout is 60 seconds; API output maximum is 2,048 tokens including reasoning.
The host caps 128 journal updates, 8,192 output characters/32,768 bytes. This can stop a
highly fragmented answer before the token limit: the repaired mapping reports resource
limit, rather than mislabeling it protocol invalidity. Direct wire/frame/event caps and
provider-incomplete/timeout/billing categories remain distinct. Codex has its own protocol
caps; no universal provider-token accounting guarantee is claimed. [API](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/personal_assistant_direct.rs#L31) [HOST](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/personal_assistant_v0.rs#L246) [CODEX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/codex_connection.rs#L156)

Profiles, notes, room runs and Knowledge survive restart in the app-owned SQLite database.
Ongoing saved room stages become interrupted; no automatic resume/retry. Bot chat is
volatile. Database uses foreign keys, WAL and bounded busy timeout; migration checksums
protect history. SQLCipher is compiled as a dependency, but this connection path does
not configure an encryption key: do not claim encrypted-at-rest personal storage.
[DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/startup.rs#L10) [SQL](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/connection.rs#L49) [MIGRATIONS](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/migrations.rs#L10) [ROOM_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/collaboration.rs#L9) [PROFILE_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/agent_preferences.rs#L92)

Closed local diagnostics retain event/category/timing/correlation metadata, not prompts,
responses, credentials or arbitrary upstream messages. Up to three 5 MiB files and a
256-entry recent buffer are bounded operational telemetry, not tamper-proof audit or a
complete billing ledger. The owner QA ledger (33 Sends) is external test evidence,
not a product credit balance/token dashboard. Backups in prior QA were external procedures;
a general automatic product backup service is not established. [DIAG](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/diagnostics.rs#L39) [DIAG_IPC](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/diagnostics_tauri.rs#L6) [REVIEW](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/docs/reviews/2026-10-06-live-provider-e2e-post-increment-review.md#L261)
