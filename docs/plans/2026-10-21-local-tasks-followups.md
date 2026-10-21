# Local Tasks and Follow-ups

Status: Acceptance verified; final local marker governed by the consolidated report
Owner: project owner; explicit 2026-10-21 implementation authorization

## Goal and outcome

Deliver user-operated local tasks with creation, editing, completion/reopening,
confirmed deletion, due dates, priority, notes, filtering and restart persistence.
Product tasks are separate from internal agent-orchestration tasks.

## Baseline, ownership and dependencies

Dedicated clean detached worktree at integrated PR151 commit
`c3bf5fa6538da2bb4417406895ed3c9f84f17673`. Desktop HP changes and the completed
memory checkout are untouched. No old runtime resources or reservations are reused.
RAG is preparing retrieval-preview scope; it owns Knowledge/retrieval modules.
The owner explicitly approved separate sections in `src-tauri/src/lib.rs`: Tasks
owns only new task module declarations and four task-command registrations; RAG
retains Knowledge/retrieval entries. No RAG module changes are permitted.
Tasks owns new storage/task files and the additive version-11 local-task migration
in the existing migration registry. RAG's proposed temporary collections need no
migration. Root documentation changes are additive in this isolated checkout.
No message was sent to another session and no ownership of its checkout is claimed.

## Contract and invariants

Rust owns validation, SQLite mutations and revision checks. Four fixed IPC commands
list, save, set completion and remove product tasks. The WebView supplies bounded
content/IDs and expected revisions, never paths, SQL, tools or execution commands.
Bound tasks to 500, title to 240 UTF-8 bytes, notes to 8192 bytes and revisions/IDs
to JavaScript safe integers. Unknown request fields reject. Priority is closed
low/normal/high. Due date is an optional valid Gregorian YYYY-MM-DD date, a date
without timezone conversion or scheduled effects. Stale/missing mutations reject.
Use the existing app Storage connection and migration mechanism. Preserve all
existing migration definitions and checksums; add a separate task SQL module and
one migration entry. No Knowledge, conversation or bot-memory table changes.
Task text stays local and is not inserted into prompts, diagnostic logs or model
requests. Delete requires an explicit exact-task confirmation. UI failures retain
unsaved input; transient replies cannot overwrite a newer view or edit.

## Exclusions

No model/provider changes, bot tools, background scheduler, OS notifications,
external calendars, RAG/context changes, credentials, dependencies or installation,
protected cleanup, HP work, commits, pushes, merge or publication.

## Expected paths

`src-tauri/src/local_tasks.rs`, `src-tauri/src/local_tasks_tauri.rs`,
`src-tauri/src/storage/local_tasks.rs`, `src-tauri/src/storage/mod.rs`,
`src-tauri/src/storage/migrations.rs`, the Tasks-only entries in
`src-tauri/src/lib.rs`, `src/features/tasks/*`,
`src/infrastructure/tauri/local-tasks-client*`, related new tests and this plan,
readiness/final review and additive current-state documentation. File inventory is
for attribution; protected unrelated paths remain protected.

## Acceptance

- T1: strict native validation and closed content-free errors.
- T2: transactional task CRUD, revision conflicts, bounded capacity and migration
  preservation verified with disposable databases including close/reopen.
- T3: Tasks UI supports all stated operations, filtering, accessible states,
  exact deletion confirmation and safe asynchronous behavior.
- T4: strict native IPC and frontend response parsing; browser simulation is
  explicitly labeled and never masquerades as persisted native storage.
- T5: relevant automatic checks and required full verification pass; native/UI
  evidence is stated honestly, with no model requests or personal data access.
- T6: preservation, scoped review, additive docs, accounting and actual supported
  gate finalization pass. No historical C7 acceptance is rerun or reclassified.

## Implementation checklist

- [x] Inspect baseline, dependencies and shared-path ownership.
- [x] Save byte-verified before-copies in external task evidence.
- [x] Finish readiness checks and schema-2 admission before product changes.
- [x] Implement typed domain/storage/IPC and deterministic regression coverage.
- [x] Implement client/UI and meaningful interaction/race tests.
- [x] Run affected checks, required full verification and scoped final reviews.
- [x] Update documentation and prepare normal finalization from passing evidence; verify the resulting marker and Stop.

## Verification and resources

Use pinned Node26.3.0/npm11.16.0 and Rust1.90.0, locked/offline Cargo and existing
installed dependencies copied into this task; no installations or acquisition.
Readiness: `npm run docs:check`, `npm run repository:check`,
`npm run security:scan`, `git diff --check`,
`python3 -B .codex/hooks/session_end_gate.py`.
Implementation: focused Rust storage/domain/IPC tests and Tasks/client frontend
cases, TypeScript, strict lint/format, then required `npm run verify` with existing
process-local target-Mac compiler settings if supported. No hidden skipped stage.
Each command has a finite timeout, records its result and keeps failures.
Fresh budget: 16GiB total, 12GiB target, 1GiB tmp, 1GiB existing dependency copy,
64MiB evidence (1MiB reserved for closeout), remaining within total for source.
Keep 8GiB available and 64MiB headroom. Recheck before compilation/native QA; no
mounted images or previous memory allocations are used. Only disposable synthetic
local test databases may be created. No live provider/model activity is authorized.

## Risks, rollback and final results

Main risks are lost updates, malformed dates, asynchronous stale responses and
shared registration conflicts. Use atomic revision predicates, strict date and
content validation and request-generation guards. Verified before-copies restore
only attributed task changes after review; no reset/clean/stash or automatic
rollback. Preserve the complete baseline, failed tests and historical reports.
Final results pending implementation and actual verification.

## 2026-10-21 implementation progress

Supported schema2 begin passed. The first focused Rust task suite passed9 cases;
subsequent storage validation passed47 cases including corrupt-content rejection
and first-ten migration-history preservation. Frontend tests passed16 cases before
the final reload regression was added. Full frontend lint passed before that
last affected refinement. Required final verification will supply final counts.
Storage/startup smoke expectations change only for additive migration11. The
repository checker retains an exact four-command import/call/handler allowlist
with negative path, command, import and registration regressions. No generic
allowlist relaxation or gate-implementation change occurred.

The copied public registry lives outside repository traversal in the task-owned
`local-tasks-dependency-cache-20261021-01` evidence sibling, charged within the
existing source-and-other subcap; original sources are untouched. No global
Python route was changed. All commands use bounded receipts and fresh allocation
checks. Product/profile data remains disposable synthetic fixtures only.

## 2026-10-21 Local Tasks acceptance and closeout

Local Tasks and Follow-ups is implemented and acceptance verified in the dedicated
`local-tasks-followups` checkout at integrated PR151. Required `npm run verify`
passed on the final source: 291 hook checks, 100 repository checks, 629 frontend
tests across 44 files, 537 Rust library tests, integration contracts/smokes,
strict lint/type/format checks and the native release build. Its bounded receipt
records exit0 in620.012598417seconds. Subsequent changes are documentation only.

A separately identified unsigned “Cortexa Tasks QA” build used fresh dedicated
app data and no personal data or provider requests. Native computer control
verified creation, edited notes, high priority, due date2026-11-05, completion,
completed/open filtering, reopening, search, explicit deletion confirmation with
Keep task, and persistence after a normal process quit/restart. Both launch
waiters exited0 with no owned child group remaining. Permanent GUI deletion was
not performed; actual deletion, stale revisions and migration/reopen behavior
passed deterministic Rust and frontend tests. The synthetic task and a verified
final database backup remain retained. No owner database was opened or upgraded.

The original memory/C7 results, ledger89/96, retained mounted image, review builds,
HP histories, raw indexes, admission and RAG-owned code remain unchanged. Tasks
adds migration11; older builds with only migration10 must not share this upgraded
QA database. Coordinate the migration registry and separate command-registration
sections before integrating parallel RAG work. No current RAG acceptance is claimed.

Review found no unresolved completion-blocking defect. Retained advisories are
macOS-only live coverage, a release-tool Xcode-platforms warning despite successful
build under the pinned explicit SDK, and shared-registry integration coordination.
No global toolchain setting, permission or security protection was changed.

All failed preparation/test receipts remain visible. A pre-restart backup attempt
rejected whole-stat equality before copying; the differing field was not retained
and its cause is unknown. After final normal exit, fresh descriptor-bound custody
and explicit non-access-time checks produced a byte-verified backup of the exact
owned QA database/WAL/SHM. No data deletion or irreversible action occurred.

Task evidence, source/configuration bindings, failures and backup are retained in
`/Users/hdang/.codex/visualizations/2026/09/21/01a0c184-189a-7b01-ae8f-07cfb1b83d96/local-tasks-followups-20261021-01`.
The final report is
[Local Tasks post-increment review](../reviews/2026-10-21-local-tasks-followups-post-increment-review.md).
Normal schema2 finalization and an actual task-local complete-payload Stop are
required; consult the current local marker and external final handoff for their
actual outcome. Historical readiness evidence is unchanged.

QA-build cleanup and worktree archival are deferred: retain the accepted Tasks QA
bundle, default-release recovery executable, isolated dependencies, QA data, source
and evidence. No deletion/archival was authorized, none ran and no physical disk
recovery is claimed. No commit, push, merge or publication occurred. The dedicated
worktree remains detached with attributed local changes. Do not begin a successor.
