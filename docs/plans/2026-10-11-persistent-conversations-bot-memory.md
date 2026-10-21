# Persistent conversations and per-bot memory

Owner authorization: the 2026-10-11 Main Session request; exact request retained in
`/private/tmp/cortexa-persistent-memory-hk2tezx_/owner-request.txt`.

## Objective and boundaries

Make owner-only bot advice conversations durable, with deterministic bounded history,
explicit private memory and recoverable interruption. Preserve typed native ownership,
empty tools, no fallback, collaboration behavior and existing Knowledge storage.
No RAG, automatic extraction, summarization calls, new providers, dependencies,
governance implementation, owner-data migration, publication or unrelated changes.

## Workspace and concurrent work

Task branch: `codex/persistent-bot-memory`.
Worktree: `/Users/hdang/.codex/worktrees/persistent-bot-memory/ai-agent-assistant`.
Assessment baseline: remote main `ac04c6e53bc3349496eba44be93e933f05e1519c`.
Canonical checkout retains its unique commits and untracked branding source.
The other session owns dependent-admission maintenance/publication (PR #144).
That prospective-dependent-report repair is not needed for this new independent
schema-2 milestone: D-134 already supports fresh admission with a present readiness
report. Do not import private gate state or use a new checkout to evade failures.
Recheck its merge before implementation/final verification and incorporate relevant
merged guidance only through supported Git operations. PR #139's isolated-action
product changes remain outside main and outside this milestone; do not copy them.
Cross-chat messaging was rejected by platform approval review. Observe status read-only;
serialize UI and provider use and stop the affected step if exclusive use is uncertain.

## Existing implementation and demonstrated gaps

- SQLite migrations 1-9 store preferences, private notes, identities, collaboration
  rooms, Knowledge items and links. Reuse these patterns and immutable migrations.
- `agent_chat.rs` holds volatile history of at most four completed turns. Failed and
  stopped partial answers do not become persisted conversation records.
- `agent_chat_tauri.rs` owns one native conversation and generation; IDs are process-local.
- Bots settings already edit/clear a private note and Memory Off. Notes lack creation,
  update and source-conversation metadata. Reuse this store, not a parallel memory engine.
- `AgentsPage.tsx` shows volatile earlier answers and refuses another Send after stop/error.
- Collaboration independently stores rooms and already interrupts unfinished runs on restart;
  preserve this behavior and continue excluding bot private notes from collaboration.
- Knowledge is explicit owner-managed content; do not import it automatically into memory.

## Implementation design

Add a transactional migration for bounded bot-conversation records and private-note
provenance. Store conversation/bot IDs, title, time, per-turn runtime/model/effort,
user text and assistant output with explicit complete/failed/cancelled/interrupted states.
Do not persist transport payloads, secrets, runtime handles or provider session IDs.
Persist before dispatch, checkpoint accepted text, and mark pending records interrupted
on startup without replay. Native ownership guards mutation and selection during requests.
Storage failure blocks further sends and produces a fixed error; retry must not duplicate turns.

Use deterministic recent completed-turn context within existing request-size bounds;
show stored versus sent turn counts and exclusions. Never send unfinished output as a
complete answer. Bot ownership is immutable per conversation. Explicit saved runtime/model
changes take effect on a subsequent request after disclosure, without provider-session reuse.

Extend existing per-bot notes with bounded explicit entries/provenance using the existing
preferences storage ownership. Preserve legacy note bytes. Add/edit/delete and selected-fact
save require explicit owner UI action and preview. Memory Off excludes durable memory on
subsequent requests independently of ordinary history. Conversation deletion keeps separately
saved memory, marks its provenance source deleted, and explains backups/provider retention.

## Acceptance checklist

- [ ] C1: conversation create/select/rename/delete, per-bot separation, timestamps and restart.
- [ ] C2: complete/failed/cancelled/partial records; interruption without replay; storage failures.
- [ ] C3: bounded deterministic history and visible omissions; runtime/model change semantics.
- [ ] C4: explicit memory CRUD and preview/confirmation; legacy notes and provenance preserved.
- [ ] C5: Memory Off, deletion and cross-bot canaries; no authority, credentials or private logging.
- [ ] C6: migration success/failure/rollback on disposable data; owner data untouched.
- [ ] C7: native isolated restart/preference/isolation/edit/delete/cancellation/recovery scenario.
- [ ] C8: API and Codex evidence separated; shared budget and private-auth constraints honored.
- [ ] C9: affected/full verification, reviews, documentation, preservation and truthful final gates.

## Verification and QA

Focused Rust storage/migration/context/ownership/race tests and strict typed-client/UI tests.
Required cross-cutting verify coverage, docs/repository/security/whitespace/session checks,
architecture/security/code-health/debt/readiness reviews. Reuse unchanged evidence only with
input/environment provenance. Synthetic canaries test exact constructed context, not model claims.

Use task-owned native identity/data/ports and an isolated database. Dedicated Codex home:
`/Users/hdang/Library/Application Support/Cortexa-Codex-QA`; never inspect credentials.
Historical generation ledger 63/66 pending fresh reconciliation; no new request dispatched.
Three unconsumed requests do not cover every required live scenario. Complete offline work;
reserve before each Send and pause only live coverage needing more authorization.
No direct API check without privately available credentials and applicable allowance.
QA artifact, process/window binding, startup deadline, additional-window cap, cleanup and
request ledger will be recorded externally; no simultaneous UI control with another session.

## Risks, recovery and current status

Largest risks: writes during streaming, stale snapshots, mixed identities, migration data loss
and confusing saved history with provider context. Use transactions, revision/turn identities,
strict deserialization and native ownership. Back up only disposable fixture databases in QA;
manual owner upgrade instructions must require a stopped app and recoverable database copy.
Old versions cannot safely open a newer schema; restore a compatible backup rather than downgrade.

Assessment/preparation only. No implementation or native acceptance claimed. No gate state copied.

## Implementation checkpoint (2026-10-11)

Ordinary schema-2 admission passed. Implementation is now in progress; native acceptance
and final completion remain pending. Historical generation count63, owner-authorized ceiling76,
13 remaining; zero generations dispatched by this task.

The implementation extends the existing single private-note document per bot (explicit add/edit/
clear), with created/updated time and last confirmed fact provenance, rather than a second memory
store. Runtime/model/effort changes require a new conversation; old histories remain readable.
Current notes, instructions and Memory Off apply to each subsequent request without reusing
provider session identifiers. Native context selection exposes actual included turn IDs and omitted
counts. At most three completed turns and12,288 history characters are included, reserving4,096
characters for the new message. Incomplete turns are preserved but excluded.

PR144 merged remotely at26447708934ef7f8118329d1b62977d7f14d45b6. Read-only comparison found only
governance/documentation changes, no product deltas. This task already has valid independent
schema-2 admission under D-134 and does not require dependent-remediation authority. Preserve
the admitted protected gate; do not copy another session's private state or edit frozen readiness.

## Automated verification checkpoint

`npm run verify` passed with the process-local installed Python/Xcode SDK27/Cargo workaround,
with inputs frozen in `verification-inputs-004.json` and full output/exit in `verify-004.log`
and `verify-004.exit` under the external evidence root. This includes 610 frontend tests,
467 Rust library tests, integration suites, strict Clippy, type checking, formatting,
repository checks and a release-mode native build. Prior failed receipts are preserved.
The isolated debug bundle reuses the verified frontend dist; native acceptance remains pending.
This checkpoint supersedes the earlier assessment-only status and provisional 63/66 budget.
The current ledger is 63/76, with 13 remaining and no task generations dispatched.

## Final evidence reconciliation (2026-10-11)

This additive record supersedes provisional assessment/implementation status above, preserving
historical design wording. Current memory is one private-note document per bot with last-source
provenance; runtime/model/effort remain pinned per conversation and latest notes/instructions
apply to subsequent sends. No model calls or credentials are needed for the automated evidence.

| Criterion | Actual evidence and remaining gap                                                                                                                                  |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| C1        | Automated plus native Simulation create/select/rename/delete, bot separation and restart passed.                                                                   |
| C2        | Automated failure/storage/race cases and native Simulation cancel/partial/send-recovery/interrupted restart passed; live Codex remains pending.                    |
| C3        | Deterministic context tests pass; native shows exact included completed turn IDs and omissions. Runtime/model/effort mismatch rejects.                             |
| C4        | Automated plus native explicit preview/save/edit/clear and provenance pass; immediate deletion-label refresh fixed and regression-tested after native observation. |
| C5        | Automated injection/isolation/Memory Off canaries pass. Native UI settings isolation passes; live provider use/exclusion remains pending.                          |
| C6        | Disposable v9 migration and recovery tests pass; original migrations/configuration/notes preserved. Owner database untouched.                                      |
| C7        | Required live Codex scenario blocked; native Simulation is separate evidence and cannot satisfy it. Final UI delta awaits native recheck.                          |
| C8        | Ledger63/76, zero new provider requests; four Simulation dispatches. API pending private access, no fallback.                                                      |
| C9        | Full verify plus affected frontend delta, reviews and preservation recorded; ordinary truthful FAIL disposition/Stop required, never completion.                   |

All task-owned apps closed with successful retained child/independent absence for launches002,
003 and005. Historical launch001 unknown status and launch004 no-launch failure remain. The
reviewed-runtime prerequisite and final native UI recheck are the exact next actions; no later
milestone starts. Reuse unchanged product evidence and preserve all advisories/parked work.
