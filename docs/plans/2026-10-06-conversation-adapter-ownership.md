# Conversation adapter ownership

Status: Active; completion requires the final review and ordinary gate receipts.
Owner: Cortexa project owner. Updated: 2026-10-06.

## Goal and evidence

Give the existing shared native provider dispatcher an explicit private owner,
separate from the conversation IPC module. Collaboration currently imports
`AdapterRequest`, `run_adapter` and `run_adapter_traced` from `agent_chat_tauri`.
The same module also owns preferences, catalog validation, conversation state,
request preparation and Tauri commands. The dispatcher already serves both
chat and collaboration; moving that closed boundary makes its consumers and
contract independently reviewable without merging their distinct lifecycles.

The completed ECC 2.2.3 security trial found no confirmed defect. Targeted source
review supports this ownership cleanup, not a provider-error fix. A frontend
hook extraction was considered but would touch callback/effect identities without
removing demonstrated duplication. Generic Session/cancellation consolidation
was rejected because the three hosts have different intentional ordering.
ECC Rust ownership/minimal-visibility guidance supplements existing repository
reviews; hooks and optional MCP remain disabled.

## Baseline and preservation

Canonical Desktop remains `/Users/hdang/Desktop/Projects/ai-agent-assistant`,
with its seven local commits, 66 changed paths and historical closure untouched.
This isolated worktree is
`/Users/hdang/.codex/worktrees/conversation-adapter-ownership/ai-agent-assistant`,
on `codex/conversation-adapter-ownership` at verified remote main
`6bebfa1ba43bb6ec8da4f19084ce8154faba96ef`. Its tree equals clean UI/UX HEAD
`0b827502a03b4d2ec27aa870415cd06943680663`; PR136 is already merged. ECC has no
unmerged repository prerequisite. Ordinary begin succeeded from an inactive,
clean checkout. No historical failed state was moved or bypassed.

External evidence: `/private/tmp/cortexa-conversation-adapter-evidence-fqr8ao92`.
It holds the owner prompt, baseline hashes, immutable copies of existing raw
state, all-worktree inventories, exact scope and new command receipts. All old
checkouts, artifacts, private profiles and historical reports remain untouched.
Installed node_modules and native target caches were copied into this worktree;
no installation or old-artifact replacement. No inherited cached bundle is
represented as a refactor artifact.

## Exact scope

Exactly fifteen candidate paths; no dependency, governance or configuration edits:

- `src-tauri/src/agent_adapter.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/collaboration_tauri.rs`
- `src-tauri/src/diagnostics/tests.rs`
- `src-tauri/src/lib.rs`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-06-conversation-adapter-ownership.md`
- `docs/reviews/2026-10-06-conversation-adapter-ownership-post-increment-review.md`

## Contracts and non-goals

Move the enum and two dispatcher functions intact; use a private crate module
and retain their current crate visibility. No generic transport trait, new
provider or runtime, dynamic dispatch, automatic retry or fallback. Preserve
separate Codex conversation/collaboration dispatch arms, exact Simulation text
and timing, error propagation, callback acceptance order, task-local diagnostic
scope and transport Drop behavior. Preparation still acquires credentials and
constructs requests in its current owner. No Debug implementation for requests.

Host validation, single-flight lease, event grammar, late-event rejection,
cancellation/abort/join ordering, completed-turn history and persistent room
transitions remain unchanged. No IPC signature/registration, schema, migration,
permissions, CSP, auth, redaction, UI, branding, mascot, animation, navigation,
profile or owner-data change. No confirmed defect correction is claimed.

## Milestones and acceptance checklist

- [x] Inspect instructions, ECC findings, current refs and competing tasks.
- [x] Freeze exact scope and preserve prior checkouts/state; ordinary admission.
- [x] Pass relevant existing native baseline before restructuring.
- [x] Add focused characterization for accepted/rejected event ordering,
      interleaved trace isolation and fresh-attempt recovery after failure.
- [x] Move only the shared boundary and update every existing consumer.
- [x] Prove moved bodies unchanged and all non-scope source bytes preserved.
- [x] Pass affected tests and required full offline verification.
- [x] Pass independent architecture/security/code-health review and bounded readiness review.
- [x] Pass pre-finalization documentation/repository/security/whitespace, session
      and preservation checks; prepare the frozen report for schema/finalization.
      Complete/valid status and full Stop are mandatory external receipts after freeze.

## Verification

Use installed tooling with an external process-local environment: Python 3.12.1,
Xcode SDK27, offline Cargo, release/build-override strip=none. No global changes.
The external `run.py` captures exact commands, exit status and separate logs.
Baseline filters: `agent_chat_tauri::`, `collaboration_tauri::`, `diagnostics::`.
Run added characterization before and after relocation where practical. Preserve
existing cancellation/join, timeout, duplicate/late event, new-bot isolation,
request/privacy, all workflow route and persistence/reload tests. Do not weaken
or replace existing assertions.

Required final commands: `npm run verify`, `npm run docs:check`,
`npm run repository:check`, `npm run security:scan`, `git diff --check`,
`/usr/bin/python3 -B .codex/hooks/session_end_gate.py`, external exact-scope/
preservation/body-equivalence and report-schema checks, ordinary finalization,
status and the full Stop payload. Record actual results, not planned success.

No visible behavior changes: inherited source-bound aesthetic/layout evidence is
reused, not rerun or attributed to this binary. No app or browser launch is
required for a private module relocation. Live provider success/cancellation,
Windows operation, screen-reader speech and owner ordinary workflows remain
unverified. New tests are synthetic native contracts, not live integration proof.

## Risks and stop conditions

Imports, test visibility, dispatch-arm changes or diagnostic ordering could cause
regression. Exact body comparison and observer-level tests constrain these risks.
Stop on conflicting changes, failed baseline, admission rejection, missing tooling,
necessary scope expansion or unresolved security/verification failure. Preserve
evidence and truthful status; no automatic rollback or governance exception.

## Rollback

Before publication, compare the frozen candidate with baseline `6bebfa1` and
reverse only these five Rust changes plus this milestone's additive documentation.
Keep new tests/evidence archived for reference. Do not blanket restore a checkout,
overwrite later edits, reset branches or remove old artifacts. If subsequent work
shares these paths, prepare a reviewed reverse patch rather than restoring files.
No data migration or application-profile rollback is involved. No commit, push,
merge or publication is authorized.

## Progress and final results

Baseline passed: seven chat tests, twelve collaboration tests and the diagnostics
suite. Three added characterization tests passed against the original dispatcher.
The shared boundary is relocated; post-move characterization and full offline
verification passed. Final documentation validation caught two relative-link
errors in the new review; both were corrected, with the failed receipt retained.
Independent review narrowed callback-order wording to FirstResponse and FirstText.
See the final review and external receipts for corrected documentation and final gates.
Live QA stays parked at 4/10 used, six remaining; the fourth owner-operated Send
used a different prompt and ended in a top-level stream error. Its cause remains
unknown. Preserve all earlier receipts and exhausted prior allowance separately.
Retain D-127/D-128, unsigned/platform/provider/runtime/Codex-isolation, build-weight
and process-local workaround advisories, plus parked D-125/M1/M2.

## Final verification and handoff

Passed: baseline chat 7, collaboration 12 and diagnostics 18 tests; three new
characterization tests before and after extraction. Full offline `npm run verify`
passed: 105 hook, 94 repository, 587 frontend, 439 Rust unit and 255 Rust integration
tests, with one inherited opt-in Hermes test ignored. The integration script also
reruns the 439 library tests; this is not an additional unique-test total.
Formatting, strict lint/Clippy, typecheck, frontend and release no-bundle native
builds passed. The process-local Python 3.12.1/Xcode SDK27/Cargo strip-none route
used installed tooling; no installs, global changes, app launches or live requests.

Independent source review found no blocking issue. Exact-scope/preservation checks
retain all sixteen other checkouts, their files/status, historical document bodies,
prior raw gate states and the bound UI/UX QA/personal bundle bytes. Final report
schema, ordinary completion, complete/valid status and full Stop are recorded
externally after the report freezes; those receipts control completion.

Native visual/layout/animation and owner six-screen approval are inherited,
not repeated or attributed to a new refactor bundle. No isolated refactor bundle
was created; cached bundle copies are old evidence, not new QA artifacts. Windows,
screen-reader speech, ordinary owner workflows, live success/cancellation and new
remote CI remain unverified. Preserve D-127/D-128, unsigned/platform/provider/runtime/
Codex-isolation, build-weight/Node localStorage warnings, process-local workaround
and all other historical advisories. Live QA remains parked at 4/10 used, six left;
D-125/M1/M2 remain parked. No commit, push, merge or publication occurred.

Manual QA, if later authorized against an isolated source-bound artifact:

1. Use only synthetic Simulation data; observe ordered text and one terminal state.
2. Stop once during a run, wait for ownership release, then start a separate fresh conversation.
3. Confirm diagnostics correlate that run without exposing synthetic text; preserve existing records.

These optional checks are not new live-provider evidence or a reason to repeat the
completed aesthetic/layout review. Do not use a personal profile as a QA substitute.

Rollback: compare against baseline `6bebfa1` and reverse only this milestone's
five Rust paths and additive documentation after checking for later edits. Preserve
new evidence/tests in the external archive. No database migration or profile rollback
is needed. Never blanket restore/reset, remove other worktrees or erase history.
